pub mod overflow_button;
pub mod overflow_popup;

use overflow_button::OverflowButton;
use smithay::{
    desktop::Window,
    space_elements,
    wayland::{seat::WaylandFocus, shell::xdg::ToplevelSurface},
};

space_elements! {
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub CosmicMappedInternal;
    OverflowButton=OverflowButton,
    Window=Window
}

impl CosmicMappedInternal {
    pub fn toplevel(&self) -> Option<&ToplevelSurface> {
        match self {
            CosmicMappedInternal::Window(w) => w.toplevel(),
            _ => None,
        }
    }
}

impl WaylandFocus for CosmicMappedInternal {
    fn wl_surface(
        &self,
    ) -> Option<smithay::reexports::wayland_server::protocol::wl_surface::WlSurface> {
        match self {
            CosmicMappedInternal::Window(w) => w.wl_surface(),
            _ => None,
        }
    }
}
