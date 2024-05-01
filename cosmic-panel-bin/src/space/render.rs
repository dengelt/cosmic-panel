use std::{num::NonZeroU32, time::Duration};

use crate::iced::elements::CosmicMappedInternal;

use super::{
    corner_element::{RoundedRectangleShader, RoundedRectangleShaderElement},
    PanelSpace,
};
use cctk::wayland_client::{Proxy, QueueHandle};

use sctk::shell::WaylandSurface;
use smithay::{
    backend::renderer::{
        damage::OutputDamageTracker,
        element::{
            memory::MemoryRenderBufferRenderElement,
            surface::{render_elements_from_surface_tree, WaylandSurfaceRenderElement},
            Element, RenderElement, UnderlyingStorage,
        },
        gles::{GlesError, GlesFrame, GlesRenderer},
        Bind, Frame, Renderer, Unbind,
    },
    utils::{Buffer, Physical, Rectangle},
};
use xdg_shell_wrapper::{shared_state::GlobalState, space::WrapperSpace};
pub(crate) enum PanelRenderElement {
    Wayland(
        WaylandSurfaceRenderElement<GlesRenderer>,
        Rectangle<f64, Buffer>,
        Rectangle<i32, Physical>,
    ),
    RoundedRectangle(RoundedRectangleShaderElement),
    OverflowMenu(MemoryRenderBufferRenderElement<GlesRenderer>),
}

impl smithay::backend::renderer::element::Element for PanelRenderElement {
    fn id(&self) -> &smithay::backend::renderer::element::Id {
        match self {
            Self::Wayland(e, ..) => e.id(),
            Self::RoundedRectangle(e) => e.id(),
            Self::OverflowMenu(e) => e.id(),
        }
    }

    fn current_commit(&self) -> smithay::backend::renderer::utils::CommitCounter {
        match self {
            Self::Wayland(e, ..) => e.current_commit(),
            Self::RoundedRectangle(e) => e.current_commit(),
            Self::OverflowMenu(e) => e.current_commit(),
        }
    }

    fn src(&self) -> Rectangle<f64, Buffer> {
        match self {
            Self::Wayland(e, src, ..) => *src,
            Self::RoundedRectangle(e) => e.src(),
            Self::OverflowMenu(e) => e.src(),
        }
    }

    fn geometry(&self, scale: smithay::utils::Scale<f64>) -> Rectangle<i32, Physical> {
        match self {
            Self::Wayland(e, _, geo) => *geo,
            Self::RoundedRectangle(e) => e.geometry(scale),
            Self::OverflowMenu(e) => e.geometry(scale),
        }
    }
}

impl RenderElement<GlesRenderer> for PanelRenderElement {
    fn draw(
        &self,
        frame: &mut GlesFrame<'_>,
        src: Rectangle<f64, Buffer>,
        dst: Rectangle<i32, Physical>,
        damage: &[Rectangle<i32, Physical>],
    ) -> Result<(), GlesError> {
        match self {
            Self::Wayland(e, ..) => e.draw(frame, src, dst, damage),
            Self::RoundedRectangle(e) => e.draw(frame, src, dst, damage),
            Self::OverflowMenu(e) => e.draw(frame, src, dst, damage),
        }
    }

    fn underlying_storage(&self, renderer: &mut GlesRenderer) -> Option<UnderlyingStorage> {
        match self {
            PanelRenderElement::Wayland(e, ..) => e.underlying_storage(renderer),
            PanelRenderElement::RoundedRectangle(e) => e.underlying_storage(renderer),
            PanelRenderElement::OverflowMenu(e) => e.underlying_storage(renderer),
        }
    }
}

impl PanelSpace {
    pub(crate) fn render<W: WrapperSpace>(
        &mut self,
        renderer: &mut GlesRenderer,
        time: u32,
        qh: &QueueHandle<GlobalState<W>>,
    ) -> anyhow::Result<()> {
        if self.space_event.get() != None && (self.actual_size.w <= 20 || self.actual_size.h <= 20)
        {
            return Ok(());
        }
        let mut bg_color = self.bg_color();
        for c in 0..3 {
            bg_color[c] *= bg_color[3];
        }

        if self.is_dirty && self.has_frame {
            let my_renderer = match self.damage_tracked_renderer.as_mut() {
                Some(r) => r,
                None => return Ok(()),
            };
            renderer.unbind()?;
            renderer.bind(self.egl_surface.as_ref().unwrap().clone())?;
            let clear_color = bg_color;
            // if not visible, just clear and exit early
            let not_visible = self.config.autohide.is_some()
                && matches!(self.visibility, xdg_shell_wrapper::space::Visibility::Hidden);
            let dim = self.dimensions.to_f64().to_physical(self.scale).to_i32_round();
            // TODO check to make sure this is not going to cause damage issues
            if not_visible {
                if let Ok(mut frame) = renderer.render(dim, smithay::utils::Transform::Normal) {
                    _ = frame
                        .clear([0.0, 0.0, 0.0, 0.0], &[Rectangle::from_loc_and_size((0, 0), dim)]);
                    if let Ok(sync_point) = frame.finish() {
                        if let Err(err) = sync_point.wait() {
                            tracing::error!("Error waiting for sync point: {:?}", err);
                        }
                        self.egl_surface.as_ref().unwrap().swap_buffers(None)?;
                    }
                    let wl_surface = self.layer.as_ref().unwrap().wl_surface();
                    wl_surface.frame(qh, wl_surface.clone());
                    wl_surface.commit();
                    // reset the damage tracker
                    *my_renderer =
                        OutputDamageTracker::new(dim, 1.0, smithay::utils::Transform::Flipped180);
                }

                renderer.unbind()?;
                self.is_dirty = false;
                self.has_frame = false;
                return Ok(());
            }

            if let Some((o, _info)) = &self.output.as_ref().map(|(_, o, info)| (o, info)) {
                let elements: Vec<PanelRenderElement> = (self.panel_changed
                    && (self.config.anchor_gap || self.config.border_radius > 0))
                    .then(|| {
                        PanelRenderElement::RoundedRectangle(RoundedRectangleShader::element(
                            renderer,
                            Rectangle::from_loc_and_size((0, 0), dim.to_logical(1)),
                            self.panel_rect_settings,
                        ))
                    })
                    .into_iter()
                    .chain(
                        self.space
                            .elements()
                            .filter_map(|w| {
                                let loc = self
                                    .space
                                    .element_location(w)
                                    .unwrap_or_default()
                                    .to_f64()
                                    .to_physical(self.scale)
                                    .to_i32_round();
                                w.toplevel().map(|t| {
                                    render_elements_from_surface_tree(
                                        renderer,
                                        t.wl_surface(),
                                        loc,
                                        self.scale,
                                        1.0,
                                        smithay::backend::renderer::element::Kind::Unspecified,
                                    )
                                    .into_iter()
                                    .map(
                                        |r: WaylandSurfaceRenderElement<GlesRenderer>| {
                                            let mut src = r.src();
                                            let mut geo = r.geometry(self.scale.into());
                                            if let Some(size) = t.current_state().size {
                                                let scaled_size = size
                                                    .to_f64()
                                                    .to_buffer(
                                                        self.scale,
                                                        smithay::utils::Transform::Normal,
                                                    )
                                                    .to_i32_ceil();
                                                if size.w > 0 && scaled_size.w < src.size.w {
                                                    src.size.w = scaled_size.w;
                                                    geo.size.w = scaled_size.w.ceil() as i32;
                                                }
                                                if size.h > 0 && scaled_size.h < src.size.h {
                                                    src.size.h = scaled_size.h;
                                                    geo.size.h = scaled_size.h.ceil() as i32;
                                                }
                                            }
                                            PanelRenderElement::Wayland(r, src, geo)
                                        },
                                    )
                                })
                            })
                            .flatten(),
                    )
                    .collect();

                _ = my_renderer.render_output(
                    renderer,
                    self.egl_surface.as_ref().unwrap().buffer_age().unwrap_or_default() as usize,
                    &elements,
                    clear_color,
                );

                self.egl_surface.as_ref().unwrap().swap_buffers(None)?;

                for window in self.space.elements().filter_map(|w| {
                    if let CosmicMappedInternal::Window(w) = w {
                        Some(w)
                    } else {
                        None
                    }
                }) {
                    let output = *o;
                    window.send_frame(o, Duration::from_millis(time as u64), None, move |_, _| {
                        Some(output.clone())
                    });
                }
                let wl_surface = self.layer.as_ref().unwrap().wl_surface().clone();
                wl_surface.frame(qh, wl_surface.clone());
                wl_surface.commit();

                self.is_dirty = false;
                self.has_frame = false;
            }
        }
        let clear_color = [0.0, 0.0, 0.0, 0.0];
        // TODO Popup rendering optimization
        for p in self.popups.iter_mut().filter(|p| {
            p.dirty
                && p.egl_surface.is_some()
                && p.state.is_none()
                && p.s_surface.alive()
                && p.c_popup.wl_surface().is_alive()
                && p.has_frame
        }) {
            renderer.unbind()?;
            renderer.bind(p.egl_surface.as_ref().unwrap().clone())?;

            let elements: Vec<WaylandSurfaceRenderElement<_>> = render_elements_from_surface_tree(
                renderer,
                p.s_surface.wl_surface(),
                (0, 0),
                self.scale,
                1.0,
                smithay::backend::renderer::element::Kind::Unspecified,
            );
            p.damage_tracked_renderer.render_output(
                renderer,
                p.egl_surface.as_ref().unwrap().buffer_age().unwrap_or_default() as usize,
                &elements,
                clear_color,
            )?;

            p.egl_surface.as_ref().unwrap().swap_buffers(None)?;

            let wl_surface = p.c_popup.wl_surface().clone();
            wl_surface.frame(qh, wl_surface.clone());
            wl_surface.commit();
            p.dirty = false;
            p.has_frame = false;
        }
        renderer.unbind()?;

        Ok(())
    }
}
