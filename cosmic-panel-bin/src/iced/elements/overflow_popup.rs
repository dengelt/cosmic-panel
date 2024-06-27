// popup for rendering overflow items in their own space

use calloop::LoopHandle;
use cosmic::{
    iced::{id, Length},
    widget::horizontal_space,
    Theme,
};

use crate::{
    iced::{Element, IcedElement, Program},
    xdg_shell_wrapper::shared_state::GlobalState,
};

use super::overflow_button::OverflowButton;

pub type OverflowPopupElement = IcedElement<OverflowPopup>;

pub fn overflow_popup_element(
    id: id::Id,
    logical_width: f32,
    logical_height: f32,
    loop_handle: LoopHandle<'static, GlobalState>,
    theme: Theme,
    panel_id: usize,
) -> OverflowPopupElement {
    IcedElement::new(
        OverflowPopup { id, logical_width, logical_height },
        (logical_width.round() as i32, logical_height.round() as i32),
        loop_handle,
        theme,
        panel_id,
        false,
    )
}

pub struct OverflowPopup {
    pub id: id::Id,
    pub logical_width: f32,
    pub logical_height: f32,
}

impl Program for OverflowPopup {
    type Message = ();

    fn view(&self) -> Element<'_, ()> {
        Element::from(
            cosmic::widget::container(horizontal_space(Length::Fixed(self.logical_width)))
                .width(Length::Fixed(self.logical_width))
                .height(Length::Fixed(self.logical_height))
                .style(cosmic::theme::Container::WindowBackground),
        )
    }
}
