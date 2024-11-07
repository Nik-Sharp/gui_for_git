use std::any::Any;

use iced::advanced::{widget, Widget};
use iced::widget::Component;
use iced::{
    advanced::{layout, renderer},
    widget::{canvas::Program, Canvas},
    Border, Color, Length, Element, Renderer, Size,
};


#[derive(Clone, Copy)]
pub struct Rectangle<'a, Theme>
where Theme: Catalog{
    width: Length,
    height: Length,
    curvature: f32,

    class: Theme::Class<'a>
}

impl<'a, Message, Theme, Renderer> Widget<Message, Theme, Renderer> for Rectangle<'a, Theme>
where
    Renderer: renderer::Renderer,
    Theme: Catalog,
{
    fn size(&self) -> Size<Length> {
        Size {
            width: self.width,
            height: self.height,
        }
    }

    fn layout(
        &self,
        _tree: &mut widget::Tree,
        _renderer: &Renderer,
        limits: &iced::advanced::layout::Limits,
    ) -> iced::advanced::layout::Node {

        layout::atomic(
            limits,
            self.width,
            self.height,
        )
    }

    fn draw(
        &self,
        _tree: &widget::Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        _style: &iced::advanced::renderer::Style,
        layout: iced::advanced::Layout<'_>,
        _cursor: iced::advanced::mouse::Cursor,
        _viewport: &iced::Rectangle,
    ) {

        let style = theme.style(&self.class);

        renderer.fill_quad(
            renderer::Quad {
                bounds: layout.bounds(),
                border: Border::rounded(self.curvature),
                ..renderer::Quad::default()
            },
            style.color.unwrap_or(Color::TRANSPARENT),
        );
    }
}

impl<'a, Message, Theme, Renderer> From<Rectangle<'a, Theme>> for Element<'a, Message, Theme, Renderer>
where
    Renderer: renderer::Renderer,
    Theme: Catalog + 'a,
{
    fn from(rect: Rectangle<'a, Theme>) -> Self {
        Self::new(rect)
    }
}

impl<'a, Theme> Rectangle<'a, Theme>
where Theme: Catalog {
    pub fn new(width: impl Into<Length>, height: impl Into<Length>, curvature: f32) -> Self {
        Rectangle {
            width: width.into(),
            height: height.into(),
            curvature: curvature,
            class: Theme::default()
        }
    }

    pub fn width(&mut self, width: impl Into<Length>) {
        self.width = width.into();
    }

    pub fn height(&mut self, height: impl Into<Length>) {
        self.height = height.into();
    }

    pub fn curvature(&mut self, curvature: f32) {
        self.curvature = curvature;
    }

    #[must_use]
    pub fn style(mut self, style: impl Fn(&Theme) -> Style + 'a) -> Self
    where
        Theme::Class<'a>: From<StyleFn<'a, Theme>>,
    {
        self.class = (Box::new(style) as StyleFn<'a, Theme>).into();
        self
    }

    #[must_use]
    pub fn class(mut self, class: impl Into<Theme::Class<'a>>) -> Self {
        self.class = class.into();
        self
    }
}



pub type StyleFn<'a, Theme> = Box<dyn Fn(&Theme) -> Style + 'a>;
pub struct Style {
    pub color: Option<Color>,
}

pub trait Catalog {
    type Class<'a>;

    fn default<'a>() -> Self::Class<'a>;

    fn style(&self, class: &Self::Class<'_>) -> Style;
}