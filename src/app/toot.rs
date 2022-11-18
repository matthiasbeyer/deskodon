use std::sync::Arc;

use iced::widget::{Text, Row};
use iced::{Point, Rectangle, Color, Background};
use iced_native::renderer;
use megalodon::entities::status::Status;

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

impl<M, R> iced_native::widget::Widget<M, R> for Toot
where
    R: iced_native::Renderer + iced_native::text::Renderer<Font = iced_native::Font>,
    R::Theme: iced_style::text::StyleSheet,
{
    fn width(&self) -> iced_native::Length {
        iced_native::Length::Fill
    }

    fn height(&self) -> iced_native::Length {
        iced_native::Length::Fill
    }

    fn layout(
        &self,
        renderer: &R,
        limits: &iced_native::layout::Limits,
    ) -> iced_native::layout::Node {
        let header = Row::<'_, M, R>::new()
            .spacing(20)
            .push(Text::new(self.status.account.username.to_string()))
            .push(Text::new(
                self.status
                    .reblog
                    .is_some()
                    .then(|| "Reblog")
                    .unwrap_or("Original"),
            ))
            .push(Text::new(self.status.created_at.to_string()));

        let content = iced_native::widget::Row::new()
            .spacing(20)
            .push(Text::new(self.status.content.to_string()));

        let replies_count = Text::new(self.status.replies_count.to_string());
        let reblogs_count = Text::new(self.status.reblogs_count.to_string());

        let footer = Row::new()
            .spacing(20)
            .push(replies_count)
            .push(reblogs_count);

        iced_native::widget::Column::new()
            .spacing(20)
            .push(header)
            .push(content)
            .push(footer)
            .layout(renderer, limits)
    }

    fn draw(
        &self,
        _state: &iced_native::widget::Tree,
        renderer: &mut R,
        _theme: &R::Theme,
        _style: &renderer::Style,
        layout: iced_native::Layout,
        _cursor_position: Point,
        _viewport: &Rectangle,
    ) {
        let bounds = layout.bounds();
        // let children = layout.children();
        // let is_mouse_over = bounds.contains(cursor_position);

        renderer.fill_quad(
            renderer::Quad {
                bounds,
                border_radius: 0.0,
                border_width: 1.0,
                border_color: Color::TRANSPARENT,
            },
            Background::Color(Color::TRANSPARENT),
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
