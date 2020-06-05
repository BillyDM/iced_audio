//! wgpu renderer for the [`VSlider`] widget
//!
//! [`VSlider`]: ../native/v_slider/struct.VSlider.html

use crate::core::{Normal, TickMarkGroup, TickMarkTier};
use crate::native::v_slider;
use iced_native::{Background, Color, MouseCursor, Point, Rectangle};
use iced_wgpu::{Primitive, Renderer};

pub use crate::native::v_slider::State;
pub use crate::style::v_slider::{
    ClassicHandle, ClassicStyle, RectBipolarStyle, RectStyle, Style,
    StyleSheet, TextureStyle, TickMarkStyle,
};

/// This is an alias of a `crate::native` [`VSlider`] with an
/// `iced_wgpu::Renderer`.
///
/// [`VSlider`]: ../../native/v_slider/struct.VSlider.html
pub type VSlider<'a, Message, ID> =
    v_slider::VSlider<'a, Message, Renderer, ID>;

impl v_slider::Renderer for Renderer {
    type Style = Box<dyn StyleSheet>;

    fn draw(
        &mut self,
        bounds: Rectangle,
        cursor_position: Point,
        normal: Normal,
        is_dragging: bool,
        tick_marks: Option<&TickMarkGroup>,
        style_sheet: &Self::Style,
    ) -> Self::Output {
        let is_mouse_over = bounds.contains(cursor_position);

        let style = if is_dragging {
            style_sheet.dragging()
        } else if is_mouse_over {
            style_sheet.hovered()
        } else {
            style_sheet.active()
        };

        let bounds_x = bounds.x.floor();
        let bounds_y = bounds.y.floor();

        let bounds_width = bounds.width.floor();
        let bounds_height = bounds.height.floor();

        let rail_x = (bounds_x + (bounds_width / 2.0)).round();

        let tick_marks: Primitive = {
            if let Some(tick_marks) = tick_marks {
                if let Some(style) = style_sheet.tick_mark_style() {
                    let center_offset = style.center_offset as f32;
                    let handle_offset = style.handle_offset as f32;
                    let notch_span = bounds_height - (handle_offset * 2.0);

                    let mut primitives: Vec<Primitive> = Vec::new();
                    for tick_mark in tick_marks.group.iter() {
                        let y_offset = ((notch_span
                            * tick_mark.position.value())
                            + handle_offset)
                            .floor();

                        let (scale, height, color) = match tick_mark.tier {
                            TickMarkTier::One => (
                                style.scale_tier_1,
                                style.height_tier_1,
                                style.color_tier_1,
                            ),
                            TickMarkTier::Two => (
                                style.scale_tier_2,
                                style.height_tier_2,
                                style.color_tier_2,
                            ),
                            TickMarkTier::Three => (
                                style.scale_tier_3,
                                style.height_tier_3,
                                style.color_tier_3,
                            ),
                        };

                        let notch_width = (scale * bounds_width).round();
                        let half_notch_width = (notch_width / 2.0).round();
                        let half_height = (height as f32 / 2.0).round();

                        if style.center_offset == 0 {
                            let mark = Primitive::Quad {
                                bounds: Rectangle {
                                    x: rail_x - half_notch_width,
                                    y: bounds_y + bounds_height
                                        - y_offset
                                        - half_height,
                                    width: notch_width,
                                    height: height as f32,
                                },
                                background: Background::Color(color),
                                border_radius: 0,
                                border_width: 0,
                                border_color: Color::TRANSPARENT,
                            };

                            primitives.push(mark);
                        } else {
                            let top = Primitive::Quad {
                                bounds: Rectangle {
                                    x: rail_x
                                        - half_notch_width
                                        - center_offset,
                                    y: bounds_y + bounds_height
                                        - y_offset
                                        - half_height,
                                    width: half_notch_width,
                                    height: height as f32,
                                },
                                background: Background::Color(color),
                                border_radius: 0,
                                border_width: 0,
                                border_color: Color::TRANSPARENT,
                            };

                            let bottom = Primitive::Quad {
                                bounds: Rectangle {
                                    x: rail_x + center_offset,
                                    y: bounds_y + bounds_height
                                        - y_offset
                                        - half_height,
                                    width: half_notch_width,
                                    height: height as f32,
                                },
                                background: Background::Color(color),
                                border_radius: 0,
                                border_width: 0,
                                border_color: Color::TRANSPARENT,
                            };

                            primitives.push(top);
                            primitives.push(bottom);
                        }
                    }

                    Primitive::Group { primitives }
                } else {
                    Primitive::None
                }
            } else {
                Primitive::None
            }
        };

        match style {
            Style::Texture(style) => {
                let (left_rail_width, right_rail_width) = style.rail_widths;
                let left_rail_width = left_rail_width as f32;
                let right_rail_width = right_rail_width as f32;
                let full_rail_width = left_rail_width + right_rail_width;
                let half_full_rail_width = (full_rail_width / 2.0).floor();

                let (rail_left, rail_right) = (
                    Primitive::Quad {
                        bounds: Rectangle {
                            x: rail_x - half_full_rail_width,
                            y: bounds_y,
                            width: left_rail_width,
                            height: bounds_height,
                        },
                        background: Background::Color(style.rail_colors.0),
                        border_radius: 0,
                        border_width: 0,
                        border_color: Color::TRANSPARENT,
                    },
                    Primitive::Quad {
                        bounds: Rectangle {
                            x: rail_x - half_full_rail_width + left_rail_width,
                            y: bounds_y,
                            width: right_rail_width,
                            height: bounds_height,
                        },
                        background: Background::Color(style.rail_colors.1),
                        border_radius: 0,
                        border_width: 0,
                        border_color: Color::TRANSPARENT,
                    },
                );

                let handle_height = style.handle_height as f32;

                let handle_offset = ((bounds_height - handle_height)
                    * (1.0 - normal.value()))
                .round();

                let handle = {
                    if let Some(pad) = style.texture_padding {
                        Primitive::Image {
                            handle: style.texture,
                            bounds: Rectangle {
                                x: (rail_x - (bounds_width / 2.0)).round()
                                    - pad.bottom as f32,
                                y: bounds.y + handle_offset - pad.top as f32,
                                width: bounds_width
                                    + (pad.bottom + pad.top) as f32,
                                height: handle_height
                                    + (pad.top + pad.bottom) as f32,
                            },
                        }
                    } else {
                        Primitive::Image {
                            handle: style.texture,
                            bounds: Rectangle {
                                x: (rail_x - (bounds_width / 2.0) + 1.0)
                                    .round(),
                                y: bounds.y + handle_offset,
                                width: bounds_width,
                                height: handle_height,
                            },
                        }
                    }
                };

                (
                    Primitive::Group {
                        primitives: vec![
                            tick_marks, rail_left, rail_right, handle,
                        ],
                    },
                    MouseCursor::default(),
                )
            }

            Style::Classic(style) => {
                let (left_rail_width, right_rail_width) = style.rail_widths;
                let left_rail_width = left_rail_width as f32;
                let right_rail_width = right_rail_width as f32;
                let full_rail_width = left_rail_width + right_rail_width;
                let half_full_rail_width = (full_rail_width / 2.0).floor();

                let (rail_left, rail_right) = (
                    Primitive::Quad {
                        bounds: Rectangle {
                            x: rail_x - half_full_rail_width,
                            y: bounds_y,
                            width: left_rail_width,
                            height: bounds_height,
                        },
                        background: Background::Color(style.rail_colors.0),
                        border_radius: 0,
                        border_width: 0,
                        border_color: Color::TRANSPARENT,
                    },
                    Primitive::Quad {
                        bounds: Rectangle {
                            x: rail_x - half_full_rail_width + left_rail_width,
                            y: bounds_y,
                            width: right_rail_width,
                            height: bounds_height,
                        },
                        background: Background::Color(style.rail_colors.1),
                        border_radius: 0,
                        border_width: 0,
                        border_color: Color::TRANSPARENT,
                    },
                );

                let (handle_height, handle_border_radius) = (
                    f32::from(style.handle.height),
                    style.handle.border_radius,
                );

                let handle_offset = ((bounds_height - handle_height)
                    * (1.0 - normal.value()))
                .round();

                let notch_height = style.handle.notch_height as f32;

                let handle = Primitive::Quad {
                    bounds: Rectangle {
                        x: (rail_x - (bounds_width / 2.0)).round(),
                        y: bounds_y + handle_offset,
                        width: bounds_width,
                        height: handle_height,
                    },
                    background: Background::Color(style.handle.color),
                    border_radius: handle_border_radius,
                    border_width: style.handle.border_width,
                    border_color: style.handle.border_color,
                };

                let handle_notch = Primitive::Quad {
                    bounds: Rectangle {
                        x: (rail_x - (bounds_width / 2.0)).round(),
                        y: (bounds_y + handle_offset + (handle_height / 2.0)
                            - (notch_height / 2.0))
                            .round(),
                        width: bounds_width,
                        height: notch_height,
                    },
                    background: Background::Color(style.handle.notch_color),
                    border_radius: 0,
                    border_width: 0,
                    border_color: Color::TRANSPARENT,
                };

                (
                    Primitive::Group {
                        primitives: vec![
                            tick_marks,
                            rail_left,
                            rail_right,
                            handle,
                            handle_notch,
                        ],
                    },
                    MouseCursor::default(),
                )
            }

            Style::Rect(style) => {
                let rect_x = rail_x - (bounds_width / 2.0).round();

                let empty_rect = Primitive::Quad {
                    bounds: Rectangle {
                        x: rect_x,
                        y: bounds_y,
                        width: bounds_width,
                        height: bounds_height,
                    },
                    background: Background::Color(style.back_empty_color),
                    border_radius: style.border_radius,
                    border_width: style.border_width,
                    border_color: style.border_color,
                };

                let handle_height = style.handle_height as f32;
                let border_width = style.border_width as f32;

                let handle_offset = (((bounds_height - (border_width * 2.0))
                    - handle_height)
                    * (1.0 - normal.value()))
                .round();

                let filled_rect_offset = handle_offset
                    + handle_height
                    + style.handle_filled_gap as f32;

                let filled_rect = Primitive::Quad {
                    bounds: Rectangle {
                        x: rect_x,
                        y: bounds_y + filled_rect_offset,
                        width: bounds_width,
                        height: bounds_height - filled_rect_offset
                            + border_width,
                    },
                    background: Background::Color(style.back_filled_color),
                    border_radius: style.border_radius,
                    border_width: style.border_width,
                    border_color: Color::TRANSPARENT,
                };

                let handle = Primitive::Quad {
                    bounds: Rectangle {
                        x: rect_x,
                        y: bounds_y + handle_offset,
                        width: bounds_width,
                        height: handle_height + (border_width * 2.0),
                    },
                    background: Background::Color(style.handle_color),
                    border_radius: style.border_radius,
                    border_width: style.border_width,
                    border_color: Color::TRANSPARENT,
                };

                (
                    Primitive::Group {
                        primitives: vec![
                            empty_rect,
                            tick_marks,
                            filled_rect,
                            handle,
                        ],
                    },
                    MouseCursor::default(),
                )
            }

            Style::RectBipolar(style) => {
                let rect_x = rail_x - (bounds_width / 2.0).round();

                let handle_height = style.handle_height as f32;
                let border_width = style.border_width as f32;

                let bottom_empty_rect = Primitive::Quad {
                    bounds: Rectangle {
                        x: rect_x,
                        y: bounds_y,
                        width: bounds_width,
                        height: bounds_height,
                    },
                    background: Background::Color(
                        style.back_bottom_empty_color,
                    ),
                    border_radius: style.border_radius,
                    border_width: style.border_width,
                    border_color: style.border_color,
                };

                let half_bounds_height = (bounds_height / 2.0).round();

                let top_empty_rect = Primitive::Quad {
                    bounds: Rectangle {
                        x: rect_x,
                        y: bounds_y,
                        width: bounds_width,
                        height: half_bounds_height,
                    },
                    background: Background::Color(style.back_top_empty_color),
                    border_radius: style.border_radius,
                    border_width: style.border_width,
                    border_color: Color::TRANSPARENT,
                };

                let handle_offset = ((((bounds_height
                    - (border_width * 2.0))
                    - handle_height)
                    * (1.0 - normal.value()))
                    + border_width)
                    .round();

                if normal.value() > 0.499 && normal.value() < 0.501 {
                    let handle = Primitive::Quad {
                        bounds: Rectangle {
                            x: rect_x,
                            y: bounds_y + handle_offset - border_width,
                            width: bounds_width,
                            height: handle_height + (border_width * 2.0),
                        },
                        background: Background::Color(
                            style.handle_center_color,
                        ),
                        border_radius: style.border_radius,
                        border_width: style.border_width,
                        border_color: Color::TRANSPARENT,
                    };

                    (
                        Primitive::Group {
                            primitives: vec![
                                bottom_empty_rect,
                                top_empty_rect,
                                tick_marks,
                                handle,
                            ],
                        },
                        MouseCursor::default(),
                    )
                } else if normal.value() > 0.5 {
                    let filled_rect_offset = handle_offset
                        + handle_height
                        + style.handle_filled_gap as f32
                        - border_width;

                    let filled_rect = Primitive::Quad {
                        bounds: Rectangle {
                            x: rect_x,
                            y: bounds_y + filled_rect_offset,
                            width: bounds_width,
                            height: half_bounds_height - filled_rect_offset,
                        },
                        background: Background::Color(
                            style.back_top_filled_color,
                        ),
                        border_radius: style.border_radius,
                        border_width: style.border_width,
                        border_color: Color::TRANSPARENT,
                    };

                    let handle = Primitive::Quad {
                        bounds: Rectangle {
                            x: rect_x,
                            y: bounds_y + handle_offset - border_width,
                            width: bounds_width,
                            height: handle_height + (border_width * 2.0),
                        },
                        background: Background::Color(style.handle_top_color),
                        border_radius: style.border_radius,
                        border_width: style.border_width,
                        border_color: Color::TRANSPARENT,
                    };

                    (
                        Primitive::Group {
                            primitives: vec![
                                bottom_empty_rect,
                                top_empty_rect,
                                tick_marks,
                                filled_rect,
                                handle,
                            ],
                        },
                        MouseCursor::default(),
                    )
                } else {
                    let filled_rect_offset = half_bounds_height;
                    let filled_rect = Primitive::Quad {
                        bounds: Rectangle {
                            x: rect_x,
                            y: bounds_y + filled_rect_offset
                                - (border_width * 2.0),
                            width: bounds_width,
                            height: handle_offset - filled_rect_offset
                                + (border_width * 3.0)
                                - style.handle_filled_gap as f32,
                        },
                        background: Background::Color(
                            style.back_bottom_filled_color,
                        ),
                        border_radius: style.border_radius,
                        border_width: style.border_width,
                        border_color: Color::TRANSPARENT,
                    };

                    let handle = Primitive::Quad {
                        bounds: Rectangle {
                            x: rect_x,
                            y: bounds_y + handle_offset - border_width,
                            width: bounds_width,
                            height: handle_height + (border_width * 2.0),
                        },
                        background: Background::Color(
                            style.handle_bottom_color,
                        ),
                        border_radius: style.border_radius,
                        border_width: style.border_width,
                        border_color: Color::TRANSPARENT,
                    };

                    (
                        Primitive::Group {
                            primitives: vec![
                                bottom_empty_rect,
                                top_empty_rect,
                                tick_marks,
                                filled_rect,
                                handle,
                            ],
                        },
                        MouseCursor::default(),
                    )
                }
            }
        }
    }
}
