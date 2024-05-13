use super::text::FONT_SMALL;
use embedded_graphics::{
    geometry::{AnchorPoint, Point, Size},
    mono_font::{MonoFont, MonoTextStyle, MonoTextStyleBuilder},
    pixelcolor::{BinaryColor, GrayColor, PixelColor},
    primitives::{CornerRadii, PrimitiveStyleBuilder, Rectangle, RoundedRectangle, StyledDrawable},
    text::renderer::TextRenderer,
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

pub trait Component: Sized {}

impl<'a, S: TextRenderer> Component for TextBox<'a, S> {}

#[derive(Clone, Copy)]
pub enum HorizontalAlign {
    Left,
    Middle,
    Right,
}

#[derive(Clone, Copy)]
pub enum VerticalAlign {
    Top,
    Middle,
    Bottom,
}

pub enum LinearDirection {
    Horizontal,
    Vertical,
}

pub trait Layout {
    fn horizontal_align(&self) -> HorizontalAlign;
    fn vertical_align(&self) -> VerticalAlign;
    fn linear_direction(&self) -> LinearDirection;
}

pub struct LayoutComponent {
    pub horizontal_align: HorizontalAlign,
    pub vertical_align: VerticalAlign,
    pub linear_direction: LinearDirection,
}

pub trait DefaultColor: PixelColor + Default {
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

impl<'a, C: PixelColor> From<&ComponentProps<'a, C>> for TextBox<'a, MonoTextStyle<'a, C>> {
    fn from(props: &ComponentProps<'a, C>) -> Self {
        props.text.clone()
    }
}

#[derive(Clone, Copy)]
pub struct BlockComponent<C: PixelColor> {
    pub bounds: Rectangle,
    pub border_radius: CornerRadii,
    pub border_width: u32,
    pub border_color: C,
    pub background_color: C,
}

impl<C: PixelColor> Component for BlockComponent<C> {}

impl<'a, C: PixelColor> From<&ComponentProps<'a, C>> for BlockComponent<C> {
    fn from(props: &ComponentProps<'a, C>) -> Self {
        props.block
    }
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

pub trait ComponentChildren: Default {}

pub trait ParentComponent {
    type Children: ComponentChildren;
}

pub struct ComponentInit<'a, C: PixelColor, Ch: ComponentChildren> {
    pub props: ComponentProps<'a, C>,
    pub children: Ch,
}

// TODO: Compile-time required fields check
pub struct ComponentProps<'a, C: PixelColor> {
    pub block: BlockComponent<C>,
    pub text: TextBox<'a, MonoTextStyle<'a, C>>,
    pub layout: LayoutComponent,
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
            layout: LayoutComponent {
                horizontal_align: HorizontalAlign::Left,
                vertical_align: VerticalAlign::Top,
                linear_direction: LinearDirection::Horizontal,
            },
        }
    }
}

#[macro_export]
macro_rules! component {
    // Helpers
    (@help size $equal: expr) => {
        embedded_graphics::geometry::Size::new_equal($equal)
    };

    (@help color $color: expr) => {
        From::from($color)
    };

    // Layout
    (@prop $props: ident vertical_align: $vertical_align: expr) => {
        $props.parent.vertical_align = $vertical_align;
    };

    (@prop $props: ident horizontal_align: $horizontal_align: expr) => {
        $props.parent.horizontal_align = $horizontal_align;
    };

    (@prop $props: ident linear_direction: $linear_direction: expr) => {
        $props.parent.linear_direction = $linear_direction;
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
        $props.block.bounds.top_left.y = $top;
    };

    (@prop $props: ident left: $left: expr) => {
        $props.block.bounds.top_left.x = $left;
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

    ($comp: ident {
        $($prop: ident: $($prop_val: expr),+;)*
    }) => {{
        let mut props = $crate::ui::builder::ComponentProps::default();
        $(component!(@prop props $prop: $($prop_val),+);)*
        <$comp<_> as From<&$crate::ui::builder::ComponentProps::<_>>>::from(&props.refined())
    }};
}

pub(crate) use component;

// #[derive(Clone, Copy)]
// pub enum LinearContainerDirection {
//     Horizontal,
//     Vertical,
// }

// #[macro_export]
// macro_rules! linear_container {
//     (@prop_ty direction: $direction: ident) => {
//         $crate::ui::builder::LinearContainerDirection
//     };

//     (@prop_ty children: {
//         $($child: ident: $child_ty: ident $($child_comp_props: tt)?)*
//     }) => {
//         $($child: $child_ty),*
//     };

//     (@prop_val direction: horizontal) => {
//         $crate::ui::builder::LinearContainerDirection::Horizontal
//     };

//     (@prop_val direction: vertical) => {
//         $crate::ui::builder::LinearContainerDirection::Vertical
//     };

//     (@prop_val children: {
//         $($child: ident: $child_ty: ident $($child_comp_props: tt)?)*
//     }) => {
//         $crate::ui::builder::component!($child_ty $($child_comp_props)?)
//     };

//     (@list_children $prop: ident: $($prop_val: tt),+) => {};

//     (@list_children children: {
//         $($child: ident: $child_ty: ident $($child_comp_props: tt)?)*
//     }) => {
//         $($child),*
//     };

//     (@list_children $($prop: ident: $($prop_val: expr),+)*) => {
//         $(linear_container!(@list_children $prop)),*
//     };

//     ({
//         $($prop: ident: $($prop_val: tt),+)*
//     }) => {
//         mod sealed {
//             $crate::ui::focus::declare_focus!(linear_container!(@list_children $($prop: $($prop_val),+)*));

//             struct LinearContainer<'a, C: $crate::ui::builder::DefaultColor> {
//                 marker: core::marker::PhantomData<&'a C>,
//                 block: $crate::ui::builder::BlockComponent<C>,
//                 $($prop: linear_container!(@prop_ty $prop: $($prop_val),+)),*
//             }

//             impl<'a, C: $crate::ui::builder::DefaultColor> LinearContainer<'a, C> {
//                 fn new() -> Self {
//                     Self {
//                         $($prop: linear_container!(@prop_val $prop: $($prop_val),+)),*
//                     }
//                 }
//             }

//             impl<'a, C: $crate::ui::builder::DefaultColor> embedded_graphics::Drawable for LinearContainer<'a, C> {
//                 type Color = C;
//                 type Output = ();

//                 fn draw<D>(&self, target: &mut D) -> Result<Self::Output, D::Error>
//                 where
//                     D: embedded_graphics::prelude::DrawTarget<Color = Self::Color>,
//                 {
//                     todo!()
//                 }
//             }
//         }
//     };
// }

use embedded_text::{
    style::{TextBoxStyle, TextBoxStyleBuilder},
    TextBox,
};

#[macro_export]
macro_rules! declare_component {
    (@extends text) => {
        embedded_text::TextBox<'a, MonoTextStyle<'a, C>>
    };

    (@extends block) => {
        $crate::ui::builder::BlockComponent<C>
    };

    (@extends $another: ident) => {
        $another<'a, C>
    };

    ($vis: vis $name: ident $(extends {$($extends: ident: $extends_ty: ident),+})? {
        $($child: ident: $child_comp: ty),* $(,)?
    }) => {
        mod sealed {
            #[derive(Default, Clone)]
            pub struct Children {
                $($child: $child_comp),*
            }

            impl $crate::ui::builder::ComponentChildren for Children {

            }
        }

        $vis struct $name<'a, C: $crate::ui::builder::DefaultColor> {
            marker: core::marker::PhantomData<&'a C>,
            children: <Self as $crate::ui::builder::ParentComponent>::Children,
            $($($extends: declare_component!(@extends $extends_ty)),+)?
        }

        impl<'a, C: $crate::ui::builder::DefaultColor> $crate::ui::builder::ParentComponent for $name<'a, C> {
            type Children = sealed::Children;
        }

        impl<'a, C: $crate::ui::builder::DefaultColor> From<&$crate::ui::builder::ComponentProps<'a, C>> for $name<'a, C> {
            fn from(props: &$crate::ui::builder::ComponentProps<'a, C>) -> Self {
                Self {
                    marker: Default::default(),
                    children: props.children.clone(),
                    $($($extends: From::from(props)),*)?
                }
            }
        }

        impl<'a, C: $crate::ui::builder::DefaultColor> Drawable for $name<'a, C> {
            type Color = C;
            type Output = ();

            fn draw<D>(&self, target: &mut D) -> Result<Self::Output, D::Error>
            where
                D: embedded_graphics::prelude::DrawTarget<Color = Self::Color>,
            {
                $(self.$child.draw(target)?;)*

                Ok(())
            }
        }
    };
}

pub use declare_component;
