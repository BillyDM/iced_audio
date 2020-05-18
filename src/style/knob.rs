use iced::Color;
//use iced_native::image;

use crate::{KnobAngleRange};

/// The appearance of a [`Knob`],
///
/// Note, `Texture` style is not implemented yet, and is just a placeholder
/// for now.
///
/// [`Knob`]: struct.Knob.html
#[derive(Debug, Clone)]
pub enum Style {
    Texture,
    //Vector(VectorStyle),
    VectorCircle(VectorCircleStyle),
}

/// A vector [`Style`] of a [`Knob`] (not working yet)
///
/// * `knob_color` - the color of the knob
/// * `knob_border_width` - the width of the border around the knob
/// * `knob_border_color` - the color of the border around the knob
/// * `notch` - the color of the notch line
/// * `notch_width` - the width of the notch line
/// * `notch_height` - the height of the notch line
/// * `notch_offset` - the offset of the notch line from the edge of the knob
/// * `inner_circle` - an optional [`InnerCircle`] to draw
///
/// [`Style`]: enum.Style.html
/// [`Knob`]: struct.Knob.html
/// [`InnerCircle`]: struct.InnerCircle.html
#[derive(Debug, Clone)]
pub struct VectorStyle {
    pub knob_color: Color,
    pub knob_border_width: u16,
    pub knob_border_color: Color,
    pub notch_color: Color,
    pub notch_width: u16,
    pub notch_height: u16,
    pub notch_offset: u16,
    pub inner_circle: Option<InnerCircle>,
}

/// A vector [`Style`] of a [`Knob`] witch a circle as the notch
///
/// * `knob_color` - the color of the knob
/// * `knob_border_width` - the width of the border around the knob
/// * `knob_border_color` - the color of the border around the knob
/// * `notch` - the color of the notch line
/// * `notch_diameter - the diameter of the notch
/// * `notch_offset` - the offset of the notch from the edge of the knob
///
/// [`Style`]: enum.Style.html
/// [`Knob`]: struct.Knob.html
/// [`InnerCircle`]: struct.InnerCircle.html
#[derive(Debug, Clone)]
pub struct VectorCircleStyle {
    pub knob_color: Color,
    pub knob_border_width: u16,
    pub knob_border_color: Color,
    pub notch_color: Color,
    pub notch_diameter: u16,
    pub notch_offset: u16,
}

/// An additional circle drawn inside of the main circle in [`VectorStyle`],
///
/// * `scale` - the scale of the circle relative to the size of the knob. For
/// example, a `scale` of `0.5` will draw the inner circle with half the radius
/// of the knob.
/// * `color` - the color of the inner circle
/// * `border_width` - the width of the border around the inner circle
/// * `border_color` - the color of the border around the inner circle
///
/// [`VectorStyle`]: enum.Style.html
#[derive(Debug, Clone)]
pub struct InnerCircle {
    pub scale: f32,
    pub color: Color,
    pub border_width: u16,
    pub border_color: Color,
}

/// A set of rules that dictate the style of a [`Knob`].
///
/// [`Knob`]: struct.Knob.html
pub trait StyleSheet {
    /// Produces the style of an active [`Knob`].
    ///
    /// [`Knob`]: struct.Knob.html
    fn active(&self) -> Style;

    /// Produces the style of a hovered [`Knob`].
    ///
    /// [`Knob`]: struct.Knob.html
    fn hovered(&self) -> Style;

    /// Produces the style of a [`Knob`] that is being dragged.
    ///
    /// [`Knob`]: struct.Knob.html
    fn dragging(&self) -> Style;

    /// The diameter of the knob
    fn diameter(&self) -> u16;

    /// a [`KnobAngleRange`] that defines the minimum and maximum angle that the
    /// knob rotates
    ///
    /// [`KnobAngleRange`]: struct.KnobAngleRange.html
    fn angle_range(&self) -> KnobAngleRange { KnobAngleRange::default() }
}

struct Default;

impl StyleSheet for Default {
    fn active(&self) -> Style {
        Style::VectorCircle(
        VectorCircleStyle {
            knob_color: Color::from_rgb(0.36, 0.36, 0.36),
            knob_border_width: 2,
            knob_border_color: Color::from_rgb(0.41, 0.41, 0.41),
            notch_color: Color::WHITE,
            notch_diameter: 5,
            notch_offset: 4,
        })
    }

    fn hovered(&self) -> Style {
        let active = self.active();
        if let Style::VectorCircle(active) = self.active() {

        Style::VectorCircle(
        VectorCircleStyle {
            notch_color: Color::from_rgb(0.9, 0.9, 0.9),
            ..active
        })

        } else { active }
    }

    fn dragging(&self) -> Style {
        self.hovered()
    }

    fn diameter(&self) -> u16 {
        31
    }
}

/*
impl StyleSheet for Default {
    fn active(&self) -> Style {
        Style::Vector(
        VectorStyle {
            knob_color: Color::from_rgb(0.4, 0.4, 0.4),
            knob_border_width: 2,
            knob_border_color: Color::from_rgb(0.42, 0.42, 0.42),
            notch_color: Color::WHITE,
            notch_width: 3,
            notch_height: 6,
            notch_offset: 2,
            inner_circle: None,
        })
    }

    fn hovered(&self) -> Style {
        let active = self.active();
        if let Style::Vector(active) = self.active() {

        Style::Vector(
        VectorStyle {
            notch_color: Color::from_rgb(0.9, 0.9, 0.9),
            ..active
        })

        } else { active }
    }

    fn dragging(&self) -> Style {
        self.hovered()
    }

    fn diameter(&self) -> u16 {
        31
    }
}
*/


impl std::default::Default for Box<dyn StyleSheet> {
    fn default() -> Self {
        Box::new(Default)
    }
}

impl<T> From<T> for Box<dyn StyleSheet>
where
    T: 'static + StyleSheet,
{
    fn from(style: T) -> Self {
        Box::new(style)
    }
}