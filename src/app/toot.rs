use std::alloc::Layout;
use std::sync::Arc;

use iced::{Point, Rectangle};
use iced::widget::{text, Column};
use iced::{
    alignment::{Horizontal, Vertical},
    widget::Container,
    widget::Row,
    Length,
};
use iced_native::{renderer, renderer::Renderer};
use megalodon::entities::status::Status;

use super::message::Message;

#[derive(Clone, Debug)]
pub struct Toot {
    status: Arc<Status>,
}

impl From<Status> for Toot {
    fn from(status: Status) -> Self {
        Self {
            status: Arc::new(status),
        }
    }
}

impl Toot {
    pub fn update(&mut self, _message: Message) -> iced::Command<Message> {
        iced::Command::none()
    }

    fn view(&self) -> iced::Element<Message> {
        let account_name = text(self.status.account.username.to_string());
        let created_at = text(self.status.created_at.to_string());
        let status_text = text(self.status.content.to_string());
        let replies_count = text(self.status.replies_count.to_string());
        let reblogs_count = text(self.status.reblogs_count.to_string());

        let header = Row::new().spacing(20).push(account_name).push(created_at);

        let content = Row::new().spacing(20).push(status_text);

        let footer = Row::new()
            .spacing(20)
            .push(replies_count)
            .push(reblogs_count);

        let column = Column::new()
            .spacing(20)
            .push(header)
            .push(content)
            .push(footer);

        Container::new(column)
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(Horizontal::Center)
            .align_y(Vertical::Center)
            .into()
    }
}

impl<M, R> iced_native::widget::Widget<M, R> for Toot
    where R: Renderer,
{
    fn width(&self) -> iced_native::Length {
        iced_native::Length::Shrink
    }

    fn height(&self) -> iced_native::Length {
        iced_native::Length::Shrink
    }

    fn layout(
        &self,
        _renderer: &R,
        _limits: &iced_native::layout::Limits,
    ) -> iced_native::layout::Node {
        iced_native::layout::Node::new({
            let width = self.status.account.username.len()
                + self.status.account.created_at.to_string().len();
            let width = f32::from(width as u16);
            let height = 5.0; // TODO
                              //
            iced_native::Size::new(width, height)
        })
    }

    fn draw(
        &self,
        _state: &iced_native::widget::Tree,
        renderer: &mut R,
        _theme: &R::Theme,
        _style: &renderer::Style,
        layout: iced_native::Layout,
        _cursor_position: Point,
        _viewport: &Rectangle
    ) {
        renderer.fill_quad(
            renderer::Quad {
                bounds: layout.bounds(),
                border_radius: 0.5,
                border_width: 0.5,
                border_color: iced_native::Color::TRANSPARENT,
            },
            iced_native::Color::TRANSPARENT
        );
    }

    fn tag(&self) -> iced_native::widget::tree::Tag {
        iced_native::widget::tree::Tag::stateless()
    }

    fn state(&self) -> iced_native::widget::tree::State {
        iced_native::widget::tree::State::None
    }

    fn children(&self) -> Vec<iced_native::widget::Tree> {
        Vec::new()
    }

    fn diff(&self, _tree: &mut iced_native::widget::Tree) {}

    fn operate(
        &self,
        _state: &mut iced_native::widget::Tree,
        _layout: iced_native::Layout<'_>,
        _operation: &mut dyn iced_native::widget::Operation<M>,
    ) {
    }

    fn on_event(
        &mut self,
        _state: &mut iced_native::widget::Tree,
        _event: iced::Event,
        _layout: iced_native::Layout<'_>,
        _cursor_position: Point,
        _renderer: &R,
        _clipboard: &mut dyn iced_native::Clipboard,
        _shell: &mut iced_native::Shell<'_, M>,
    ) -> iced::event::Status {
        iced::event::Status::Ignored
    }

    fn mouse_interaction(
        &self,
        _state: &iced_native::widget::Tree,
        _layout: iced_native::Layout<'_>,
        _cursor_position: Point,
        _viewport: &Rectangle,
        _renderer: &R,
    ) -> iced_native::mouse::Interaction {
        iced_native::mouse::Interaction::Idle
    }

    fn overlay<'a>(
        &'a self,
        _state: &'a mut iced_native::widget::Tree,
        _layout: iced_native::Layout<'_>,
        _renderer: &R,
    ) -> Option<iced_native::overlay::Element<'a, M, R>> {
        None
    }
}
