use std::{
    borrow::Cow,
    hash::Hash,
    sync::{
        atomic::{self, AtomicBool},
        Arc,
    },
};

use calloop::LoopHandle;
// element for rendering a button that toggles the overflow popup when clicked
use cosmic::{iced::Padding, iced_core::id, theme, widget::Id, Element};
use smithay::{
    desktop::space::SpaceElement,
    utils::{IsAlive, Logical, Point, Rectangle, Size},
};
use xdg_shell_wrapper::shared_state::GlobalState;

use crate::iced::{IcedElement, Program};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]

pub struct OverflowButtonElement(pub IcedElement<OverflowButton>);

impl OverflowButtonElement {
    pub fn new(
        name: impl Into<Cow<'static, str>>,
        pos: Point<i32, Logical>,
        icon_size: u16,
        button_padding: Padding,
        selected: Arc<AtomicBool>,
        icon: Cow<'static, str>,
        handle: LoopHandle<'static, GlobalState<crate::space_container::SpaceContainer>>,
        theme: cosmic::Theme,
    ) -> Self {
        let size = (
            (icon_size as f32 + button_padding.horizontal()).round() as i32,
            (icon_size as f32 + button_padding.vertical()).round() as i32,
        );
        Self(IcedElement::new(
            OverflowButton::new(name, pos, icon_size, button_padding, selected, icon),
            Size::from(size),
            handle,
            theme,
        ))
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    TogglePopup,
}

#[derive(Debug, Clone)]
pub struct OverflowButton {
    id: id::Id,
    pos: Point<i32, Logical>,
    icon_size: u16,
    button_padding: Padding,
    /// Selected if the popup is open
    selected: Arc<AtomicBool>,
    icon: Cow<'static, str>,
}

impl OverflowButton {
    pub fn new(
        name: impl Into<std::borrow::Cow<'static, str>>,
        pos: Point<i32, Logical>,
        icon_size: u16,
        button_padding: Padding,
        selected: Arc<AtomicBool>,
        icon: Cow<'static, str>,
    ) -> Self {
        Self { id: Id::new(name), pos, icon_size, button_padding, selected, icon }
    }
}

impl PartialEq for OverflowButton {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for OverflowButton {}

impl Hash for OverflowButton {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl Program for OverflowButton {
    type Message = Message;

    fn update(
        &mut self,
        message: Self::Message,
        loop_handle: &calloop::LoopHandle<
            'static,
            xdg_shell_wrapper::shared_state::GlobalState<crate::space_container::SpaceContainer>,
        >,
    ) -> cosmic::Command<Self::Message> {
        match message {
            Message::TogglePopup => {
                let id = self.id.clone();
                loop_handle.insert_idle(move |state| {
                    state.space.toggle_overflow_popup(id);
                });
            },
        }
        cosmic::Command::none()
    }

    fn view(&self) -> crate::iced::Element<'_, Self::Message> {
        Element::from(
            cosmic::widget::button::icon(
                cosmic::widget::icon::from_name(self.icon.clone())
                    .symbolic(true)
                    .size(self.icon_size),
            )
            .style(theme::Button::AppletIcon)
            .padding(self.button_padding)
            .on_press(Message::TogglePopup)
            .selected(self.selected.load(atomic::Ordering::SeqCst)),
        )
    }
}

impl IsAlive for OverflowButtonElement {
    fn alive(&self) -> bool {
        true
    }
}

impl SpaceElement for OverflowButtonElement {
    fn bbox(&self) -> smithay::utils::Rectangle<i32, smithay::utils::Logical> {
        self.0.with_program(|p| Rectangle {
            loc: p.pos,
            size: Size::from((p.icon_size as i32, p.icon_size as i32)),
        })
    }

    fn is_in_input_region(
        &self,
        point: &smithay::utils::Point<f64, smithay::utils::Logical>,
    ) -> bool {
        self.bbox().to_f64().contains(*point)
    }

    fn set_activate(&self, _activated: bool) {}

    fn output_enter(
        &self,
        _output: &smithay::output::Output,
        _overlap: smithay::utils::Rectangle<i32, smithay::utils::Logical>,
    ) {
    }

    fn output_leave(&self, _output: &smithay::output::Output) {}
}
