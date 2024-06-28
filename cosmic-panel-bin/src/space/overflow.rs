use std::rc::Rc;

use cctk::wayland_client::QueueHandle;
use cosmic::iced::id;
use sctk::{
    compositor::Region,
    shell::xdg::{popup::Popup, XdgPositioner},
};
use smithay::{
    backend::{
        egl::EGLSurface,
        renderer::{damage::OutputDamageTracker, element},
    },
    utils::{Logical, Rectangle},
};
use wayland_protocols::wp::{
    fractional_scale::v1::client::wp_fractional_scale_v1::WpFractionalScaleV1,
    viewporter::client::wp_viewport::WpViewport,
};

use crate::xdg_shell_wrapper::{
    shared_state::GlobalState, space::WrapperPopupState,
    wp_fractional_scaling::FractionalScalingManager, wp_viewporter::ViewporterState,
};

use super::PanelSpace;

impl PanelSpace {
    pub fn toggle_overflow_popup(
        &mut self,
        element_id: id::Id,
        compositor_state: &sctk::compositor::CompositorState,
        fractional_scale_manager: Option<&FractionalScalingManager>,
        viewport: Option<&ViewporterState>,
        qh: &QueueHandle<GlobalState>,
        xdg_shell_state: &mut sctk::shell::xdg::XdgShell,
    ) {
        if self.overflow_popup.is_some() {
            self.overflow_popup = None;
            return;
        }
        // get popup location and anchor based on element_id and panel
        // let positioner = XdgPositioner::new(wm_base)
        // anchor create popup using sctk
        let c_wl_surface = compositor_state.create_surface(qh);

        // let parent = self.popups.iter().find_map(|p| {
        //     if s_surface.get_parent_surface().is_some_and(|s| &s ==
        // p.s_surface.wl_surface()) {         Some(p.c_popup.
        // xdg_surface())     } else {
        //         None
        //     }
        // });

        // let c_popup = popup::Popup::from_surface(
        //     parent,
        //     &positioner,
        //     qh,
        //     c_wl_surface.clone(),
        //     xdg_shell_state,
        // )?;

        // let input_region = Region::new(compositor_state)?;

        // if let (Some(s_window_geometry), Some(input_regions)) =
        //     with_states(s_surface.wl_surface(), |states| {
        //         (
        //
        // states.cached_state.current::<SurfaceCachedState>().geometry,
        //             states
        //                 .cached_state
        //                 .current::<SurfaceAttributes>()
        //                 .input_region
        //                 .as_ref()
        //                 .cloned(),
        //         )
        //     })
        // {
        //     c_popup.xdg_surface().set_window_geometry(
        //         s_window_geometry.loc.x,
        //         s_window_geometry.loc.y,
        //         s_window_geometry.size.w.max(1),
        //         s_window_geometry.size.h.max(1),
        //     );
        //     for r in input_regions.rects {
        //         input_region.add(0, 0, r.1.size.w, r.1.size.h);
        //     }
        //     c_wl_surface.set_input_region(Some(input_region.wl_region()));
        // }

        // if parent.is_none() {
        //     self.layer.as_ref().unwrap().get_popup(c_popup.xdg_popup());
        // }
        // let fractional_scale =
        //     fractional_scale_manager.map(|f|
        // f.fractional_scaling(&c_wl_surface, &qh));

        // let viewport = viewport.map(|v| {
        //     with_states(&s_surface.wl_surface(), |states| {
        //         with_fractional_scale(states, |fractional_scale| {
        //             fractional_scale.set_preferred_scale(self.scale);
        //         });
        //     });
        //     let viewport = v.get_viewport(&c_wl_surface, &qh);
        //     viewport.set_destination(
        //         positioner_state.rect_size.w.max(1),
        //         positioner_state.rect_size.h.max(1),
        //     );
        //     viewport
        // });
        // if fractional_scale.is_none() {
        //     c_wl_surface.set_buffer_scale(self.scale as i32);
        // }

        // // must be done after role is assigned as popup
        // c_wl_surface.commit();

        // let cur_popup_state = Some(WrapperPopupState::WaitConfigure);

        // self.popups.push(WrapperPopup {
        //     damage_tracked_renderer: OutputDamageTracker::new(
        //         positioner_state.rect_size.to_f64().to_physical(self.scale).
        // to_i32_round(),         1.0,
        //         smithay::utils::Transform::Flipped180,
        //     ),
        //     c_popup,
        //     s_surface,
        //     egl_surface: None,
        //     dirty: false,
        //     rectangle: Rectangle::from_loc_and_size((0, 0),
        // positioner_state.rect_size),     state: cur_popup_state,
        //     input_region,
        //     wrapper_rectangle: Rectangle::from_loc_and_size((0, 0),
        // positioner_state.rect_size),     positioner,
        //     has_frame: true,
        //     fractional_scale,
        //     viewport,
        //     scale: self.scale,
        // });

        // Ok(())
    }
}
