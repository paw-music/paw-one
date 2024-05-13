use embedded_graphics::{
    geometry::{AnchorPoint, Point, Size},
    mono_font::{MonoFont, MonoTextStyle, MonoTextStyleBuilder},
    pixelcolor::{BinaryColor, GrayColor, PixelColor},
    primitives::{CornerRadii, PrimitiveStyleBuilder, Rectangle, RoundedRectangle, StyledDrawable},
    transform::Transform,
    Drawable,
};

// macro_rules! component_builder_stub_methods {
//     ($($method: ident ($($param: ident: $ty: ty),* $(,)?)),* $(,)?) => {
//         $(
//             fn $method(self, $($param: $ty),*) -> Self {
//                 defmt::warn!("UI: Component does not have property {}", stringify!($method));
//                 self
//             }
//         )*
//     };
// }

// pub trait ComponentBuilder: Sized {
//     type Comp: Component;

//     fn new() -> Self;
//     fn build(self) -> Self::Comp;

//     component_builder_stub_methods! {
//         // Bounds
//         bounds(bounds: embedded_graphics::primitives::Rectangle),

//         width(width: u32),
//         height(height: u32),
//         top(top: i32),
//         left(left: i32),

//         // Content
//         text(text: &str),
//     }
// }

pub trait DefaultColor: PixelColor {
    fn default_background() -> Self;
    fn default_foreground() -> Self;
}

impl DefaultColor for BinaryColor {
    fn default_background() -> Self {
        Self::Off
    }

    fn default_foreground() -> Self {
        Self::On
    }
}

pub struct BlockComponent<C: PixelColor> {
    pub bounds: Rectangle,
    pub border_radius: CornerRadii,
    pub border_width: u32,
    pub border_color: C,
    pub background_color: C,
}

impl<C: PixelColor> Drawable for BlockComponent<C> {
    type Color = C;
    type Output = ();

    fn draw<D>(&self, target: &mut D) -> Result<Self::Output, D::Error>
    where
        D: embedded_graphics::prelude::DrawTarget<Color = Self::Color>,
    {
        RoundedRectangle::new(self.bounds, self.border_radius).draw_styled(
            &PrimitiveStyleBuilder::new()
                .stroke_color(self.border_color)
                .stroke_width(self.border_width)
                .fill_color(self.background_color)
                .build(),
            target,
        )
    }
}

// pub struct TextComponent<C: PixelColor> {
//     pub text: alloc::string::String,
//     pub bounds: Rectangle,
//     pub character_style: MonoTextStyle<'static, C>,
//     pub style: TextBoxStyle,
// }

// impl<C: PixelColor + Default> Drawable for TextComponent<C> {
//     type Color = C;
//     type Output = ();

//     fn draw<D>(&self, target: &mut D) -> Result<Self::Output, D::Error>
//     where
//         D: embedded_graphics::prelude::DrawTarget<Color = Self::Color>,
//     {
//         TextBox::with_textbox_style(&self.text, self.bounds, self.character_style, self.style)
//             .draw(target)?;

//         Ok(())
//     }
// }

// TODO: Compile-time required fields check
pub struct ComponentProps<'a, C: PixelColor> {
    pub block: BlockComponent<C>,
    // pub text: TextComponent<C>,
    pub text: TextBox<'a, MonoTextStyle<'a, C>>,
}

// impl<C: PixelColor> ComponentProps<C> {
//     fn refined(mut self) -> Self {
//         // Layout calculation //
//         let text_bounds = self.text.

//     }
// }

// TODO:
// - Calculate layout
// - Add padding and margin (?)
// - Depend on character_size (kerning)
impl<'a, C: PixelColor> ComponentProps<'a, C> {
    pub fn refined(mut self) -> Self {
        // let max_size = self.block.bounds.size;

        // let text_height = self.text.style.measure_text_height(
        //     &self.text.character_style,
        //     self.text.text,
        //     max_size.width,
        // );

        // self.text.bounds = Size::new(max_size.width, text_height);
        self.text.bounds = self
            .block
            .bounds
            .resized(
                self.block.bounds.size.saturating_sub(Size::new_equal(2)),
                AnchorPoint::Center,
            )
            .translate(Point::new_equal(1));

        self
    }
}

impl<'a, C: DefaultColor> Default for ComponentProps<'a, C> {
    fn default() -> Self {
        Self {
            // text: TextComponent {
            //     text: alloc::string::String::new(),
            //     bounds: Rectangle::zero(),
            //     character_style: MonoTextStyleBuilder::new()
            //         .font(FONT_SMALL)
            //         .background_color(C::default_background())
            //         .text_color(C::default_foreground())
            //         .build(),
            //     style: TextBoxStyleBuilder::new().build(),
            // },
            text: TextBox::with_textbox_style(
                "",
                Rectangle::zero(),
                MonoTextStyleBuilder::new()
                    .font(FONT_SMALL)
                    .background_color(C::default_background())
                    .text_color(C::default_foreground())
                    .build(),
                TextBoxStyleBuilder::new()
                    .alignment(embedded_text::alignment::HorizontalAlignment::Center)
                    // TODO .height_mode(height_mode)
                    .vertical_alignment(embedded_text::alignment::VerticalAlignment::Middle)
                    .build(),
            ),
            block: BlockComponent {
                bounds: Rectangle::zero(),
                border_radius: CornerRadii::default(),
                border_width: 1,
                border_color: C::default_foreground(),
                background_color: C::default_background(),
            },
        }
    }
}

// pub trait BoundsBuilder {
//     fn bounds_mut(&mut self) -> &mut Rectangle;
// }

// pub trait TextStyleBuilder {
//     fn mono_text_style_mut(&mut self) -> &mut MonoTextStyle<'static, BinaryColor>;
// }

// pub trait Component {
//     type Builder: ComponentBuilder;
// }

// macro_rules! component {
//     // Bounds
//     (@prop $builder: ident width: $width: expr) => {{
//         $builder.bounds_mut().size.width = $width;
//         $builder
//     }};

//     (@prop $builder: ident height: $height: expr) => {{
//         $builder.bounds_mut().size.height = $height;
//         $builder
//     }};

//     (@prop $builder: ident top: $top: expr) => {{
//         $builder.bounds_mut().top_left.y = $top;
//         $builder
//     }};

//     (@prop $builder: ident left: $left: expr) => {{
//         $builder.bounds_mut().top_left.x = $left;
//         $builder
//     }};

//     // Text style / Char style
//     (@prop $builder: ident font: $font: expr) => {{
//         $builder.mono_text_style_mut().font = $font;
//     }};

//     // Non-special synth
//     (@prop $builder: ident $prop: ident: $($prop_val: expr)+) => {
//         crate::ui::ComponentBuilder::$prop($builder, $($prop_val),+)
//     };

//     ($comp: ty {
//         $($prop: ident: $($prop_val: expr)+),+ $(,)?
//     }) => {{
//         let mut builder = <$comp as crate::ui::builder::Component>::Builder::new();
//         $(let mut builder = component!(@prop builder $prop: $($prop_val)+);)+
//         builder.build()
//     }};
// }

// pub(crate) use component;

#[macro_export]
macro_rules! component {
    // Helpers
    (@help size $equal: expr) => {
        embedded_graphics::geometry::Size::new_equal($equal)
    };

    (@help color $color: expr) => {
        From::from($color)
    };

    // Block //

    (@prop $props: ident background_color: $background_color: expr) => {
        $props.block.background_color = $background_color;
    };

    // Bounds
    (@prop $props: ident width: $width: expr) => {
        $props.block.bounds.size.width = $width;
    };

    (@prop $props: ident height: $height: expr) => {
        $props.block.bounds.size.height = $height;
    };

    (@prop $props: ident top: $top: expr) => {
        $props.bounds.top_left.y = $top;
    };

    (@prop $props: ident left: $left: expr) => {
        $props.bounds.top_left.x = $left;
    };

    // Border
    (@prop $props: ident border_radius: $top_left: expr, $top_right: expr, $bottom_right: expr, $bottom_left: expr) => {
        $props.block.border_radius.top_left = component!(@help size $top_left);
        $props.block.border_radius.top_right = component!(@help size $top_right);
        $props.block.border_radius.bottom_right = component!(@help size $bottom_right);
        $props.block.border_radius.bottom_left = component!(@help size $bottom_left);
    };

    (@prop $props: ident border_radius: $equal: expr) => {
        component!(@prop $props border_radius: $equal, $equal, $equal, $equal);
    };

    (@prop $props: ident border_width: $border_width: expr) => {
        $props.block.border_width = $border_width;
    };

    (@prop $props: ident border_color: $border_color: expr) => {
        $props.block.border_color = component!(@help color $border_color);
    };

    // Text //
    (@prop $props: ident text: $text: literal) => {
        $props.text.text = $text;
    };

    (@prop $props: ident font: $font: expr) => {
        $props.text.character_style.font = $font;
    };

    (@prop $props: ident text_color: $text_color: expr) => {
        $props.text.character_style.text_style.text_color = component!(@help color $text_color);
    };

    (@prop $props: ident text_background_color: $text_color: expr) => {
        $props.text.character_style.text_style = component!(@help color $text_color);
    };

    ($comp: ty {
        $($prop: ident: $($prop_val: expr),+;)*
    } @schema {
        color: $color_ty: ty
    }) => {{
        let mut props = $crate::ui::builder::ComponentProps::<$color_ty>::default();
        $(component!(@prop props $prop: $($prop_val),+);)*
        <$comp as From<$crate::ui::builder::ComponentProps<$color_ty>>>::from(props.refined())
    }};
}

pub(crate) use component;
use embedded_text::{
    style::{TextBoxStyle, TextBoxStyleBuilder},
    TextBox,
};

use super::text::FONT_SMALL;
