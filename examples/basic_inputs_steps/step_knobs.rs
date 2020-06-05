use iced::{Column, Element, Length, Row, Text};
//use iced_native::image;

use iced_audio::{
    knob, FloatParam, IntParam, Knob, LogDBParam, Normal, OctaveParam,
    TickMark, TickMarkGroup, TickMarkTier,
};

use crate::{style, Step};

/// Unique identifier for each parameter. Note you may also use u32, i32, or
/// Strings if you wish.
#[derive(Debug, Copy, Clone)]
pub enum KnobsID {
    Float,
    Int,
    DB,
    Octave,
    Circle,
    Line,
    Texture,
}

#[derive(Debug, Clone)]
pub enum Message {
    KnobsChanged((KnobsID, Normal)),
}

pub struct KnobsStep {
    knob_float_param: FloatParam<KnobsID>,
    knob_float_state: knob::State,
    knob_float_label: String,

    knob_int_param: IntParam<KnobsID>,
    knob_int_state: knob::State,
    knob_int_label: String,

    knob_db_param: LogDBParam<KnobsID>,
    knob_db_state: knob::State,
    knob_db_label: String,

    knob_oct_param: OctaveParam<KnobsID>,
    knob_oct_state: knob::State,
    knob_oct_label: String,

    knob_circle_param: FloatParam<KnobsID>,
    knob_circle_state: knob::State,
    knob_circle_label: String,

    knob_line_param: FloatParam<KnobsID>,
    knob_line_state: knob::State,
    knob_line_label: String,

    /*
    knob_texture_param: FloatParam<KnobsID>,
    knob_texture_state: knob::State,
    knob_texture_label: String,

    knob_texture_handle: image::Handle,
    */
    float_tick_marks: TickMarkGroup,
    int_tick_marks: TickMarkGroup,
    db_tick_marks: TickMarkGroup,
    octave_tick_marks: TickMarkGroup,

    output_text: String,
}

impl Default for KnobsStep {
    fn default() -> Self {
        // initalize parameters

        let knob_float_param =
            FloatParam::<KnobsID>::new(KnobsID::Float, -1.0, 1.0, 0.0, 0.0);

        let knob_int_param = IntParam::<KnobsID>::new(KnobsID::Int, 0, 5, 0, 2);

        let knob_db_param = LogDBParam::<KnobsID>::new(
            KnobsID::DB,
            -12.0,
            12.0,
            0.0,
            0.0,
            0.5.into(),
        );

        let knob_oct_param = OctaveParam::<KnobsID>::new(
            KnobsID::Octave,
            20.0,
            20_480.0,
            1000.0,
            1000.0,
        );

        let knob_circle_param =
            FloatParam::<KnobsID>::new(KnobsID::Circle, -1.0, 1.0, 0.0, 0.0);

        let knob_line_param =
            FloatParam::<KnobsID>::new(KnobsID::Line, -1.0, 1.0, 0.0, 0.0);

        /*
        let knob_texture_param = FloatParam::<KnobsID>::new(
            KnobsID::Texture, -1.0, 1.0, 0.0, 0.0);
        */

        // create application

        Self {
            // add the parameter
            knob_float_param,
            // initialize the state of the Knob widget
            knob_float_state: knob::State::new(&knob_float_param),
            // initialize the label above the Knob widget
            knob_float_label: String::from("Float Range"),

            knob_int_param,
            knob_int_state: knob::State::new(&knob_int_param),
            knob_int_label: String::from("Int Range"),

            knob_db_param,
            knob_db_state: knob::State::new(&knob_db_param),
            knob_db_label: String::from("Log dB Range"),

            knob_oct_param,
            knob_oct_state: knob::State::new(&knob_oct_param),
            knob_oct_label: String::from("Octave Freq Range"),

            knob_circle_param,
            knob_circle_state: knob::State::new(&knob_circle_param),
            knob_circle_label: String::from("Custom Vector Circle Style"),

            knob_line_param,
            knob_line_state: knob::State::new(&knob_line_param),
            knob_line_label: String::from("Custom Vector Line Style"),

            /*
            knob_texture_param,
            knob_texture_state: knob::State::new(
                &knob_texture_param
            ),
            knob_texture_label: String::from("Custom Texture Style"),


            knob_texture_handle: format!(
                "{}/examples/images/iced_knob.png",
                env!("CARGO_MANIFEST_DIR")
            ).into(),
            */
            float_tick_marks: TickMarkGroup::subdivided(
                1,
                1,
                1,
                Some(TickMarkTier::Two),
            ),

            int_tick_marks: TickMarkGroup::subdivided(
                0,
                4,
                0,
                Some(TickMarkTier::Two),
            ),

            db_tick_marks: vec![
                TickMark {
                    position: knob_db_param.value_to_normal(0.0),
                    tier: TickMarkTier::One,
                },
                TickMark {
                    position: knob_db_param.value_to_normal(1.0),
                    tier: TickMarkTier::Two,
                },
                TickMark {
                    position: knob_db_param.value_to_normal(3.0),
                    tier: TickMarkTier::Two,
                },
                TickMark {
                    position: knob_db_param.value_to_normal(6.0),
                    tier: TickMarkTier::Two,
                },
                TickMark {
                    position: knob_db_param.value_to_normal(12.0),
                    tier: TickMarkTier::Two,
                },
                TickMark {
                    position: knob_db_param.value_to_normal(-1.0),
                    tier: TickMarkTier::Two,
                },
                TickMark {
                    position: knob_db_param.value_to_normal(-3.0),
                    tier: TickMarkTier::Two,
                },
                TickMark {
                    position: knob_db_param.value_to_normal(-6.0),
                    tier: TickMarkTier::Two,
                },
                TickMark {
                    position: knob_db_param.value_to_normal(-12.0),
                    tier: TickMarkTier::Two,
                },
            ]
            .into(),

            octave_tick_marks: vec![
                TickMark {
                    position: knob_oct_param.value_to_normal(20.0),
                    tier: TickMarkTier::Two,
                },
                TickMark {
                    position: knob_oct_param.value_to_normal(50.0),
                    tier: TickMarkTier::Two,
                },
                TickMark {
                    position: knob_oct_param.value_to_normal(100.0),
                    tier: TickMarkTier::One,
                },
                TickMark {
                    position: knob_oct_param.value_to_normal(200.0),
                    tier: TickMarkTier::Two,
                },
                TickMark {
                    position: knob_oct_param.value_to_normal(400.0),
                    tier: TickMarkTier::Two,
                },
                TickMark {
                    position: knob_oct_param.value_to_normal(1000.0),
                    tier: TickMarkTier::One,
                },
                TickMark {
                    position: knob_oct_param.value_to_normal(2000.0),
                    tier: TickMarkTier::Two,
                },
                TickMark {
                    position: knob_oct_param.value_to_normal(5000.0),
                    tier: TickMarkTier::Two,
                },
                TickMark {
                    position: knob_oct_param.value_to_normal(10000.0),
                    tier: TickMarkTier::One,
                },
                TickMark {
                    position: knob_oct_param.value_to_normal(20000.0),
                    tier: TickMarkTier::Two,
                },
            ]
            .into(),

            output_text: String::from("Move a widget"),
        }
    }
}

impl KnobsStep {
    pub fn title(&self) -> &str {
        "Knobs"
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::KnobsChanged((id, normal)) => {
                // Update the parameter with the output of the corresponding
                // Knobs widget (Note this must be done or the widget will
                // not work).

                // Then update the output text with the new value of the
                // parameter.
                match id {
                    KnobsID::Float => {
                        self.knob_float_param.set_from_normal(normal);
                        self.output_text = crate::info_text_f32(
                            id,
                            self.knob_float_param.value(),
                        );
                    }
                    KnobsID::Int => {
                        self.knob_int_param.set_from_normal(normal);
                        self.output_text = crate::info_text_i32(
                            id,
                            self.knob_int_param.value(),
                        );
                    }
                    KnobsID::DB => {
                        self.knob_db_param.set_from_normal(normal);
                        self.output_text =
                            crate::info_text_db(id, self.knob_db_param.value());
                    }
                    KnobsID::Octave => {
                        self.knob_oct_param.set_from_normal(normal);
                        self.output_text = crate::info_text_octave(
                            id,
                            self.knob_oct_param.value(),
                        );
                    }
                    KnobsID::Circle => {
                        self.knob_circle_param.set_from_normal(normal);
                        self.output_text = crate::info_text_f32(
                            id,
                            self.knob_circle_param.value(),
                        );
                    }
                    KnobsID::Line => {
                        self.knob_line_param.set_from_normal(normal);
                        self.output_text = crate::info_text_f32(
                            id,
                            self.knob_line_param.value(),
                        );
                    }
                    KnobsID::Texture => {
                        /*
                        self.knob_texture_param.set_from_normal(normal);
                        self.output_text = crate::info_text_f32(id,
                            self.knob_texture_param.value());
                        */
                    }
                }
            }
        }
    }

    pub fn view(&mut self, _debug: bool) -> Element<Message> {
        // create each of the Knobs widgets, passing in the value of
        // the corresponding parameter

        let knob_float = Knob::new(
            &mut self.knob_float_state,
            &self.knob_float_param,
            Message::KnobsChanged,
        )
        .tick_marks(&self.float_tick_marks);

        let knob_int = Knob::new(
            &mut self.knob_int_state,
            &self.knob_int_param,
            Message::KnobsChanged,
        )
        .tick_marks(&self.int_tick_marks);

        let knob_db = Knob::new(
            &mut self.knob_db_state,
            &self.knob_db_param,
            Message::KnobsChanged,
        )
        .tick_marks(&self.db_tick_marks);

        let knob_oct = Knob::new(
            &mut self.knob_oct_state,
            &self.knob_oct_param,
            Message::KnobsChanged,
        )
        .tick_marks(&self.octave_tick_marks);

        let knob_circle = Knob::new(
            &mut self.knob_circle_state,
            &self.knob_circle_param,
            Message::KnobsChanged,
        )
        .tick_marks(&self.float_tick_marks)
        .style(style::KnobCustomStyleCircle);

        let knob_line = Knob::new(
            &mut self.knob_line_state,
            &self.knob_line_param,
            Message::KnobsChanged,
        )
        .tick_marks(&self.float_tick_marks)
        .style(style::KnobCustomStyleLine);

        /*
        let knob_texture = Knob::new(
            &mut self.knob_texture_state,
            &self.knob_texture_param,
            Message::KnobsChanged,
        )
        // clone the handle to the loaded texture
        .style(style::KnobTextureStyle(
            self.knob_texture_handle.clone()
        ));
        */

        // push the knobs into columns

        let knob_row = Row::new()
            .spacing(20)
            .push(
                Column::new()
                    .max_height(400)
                    .width(Length::Fill)
                    .spacing(10)
                    .push(Text::new(&self.knob_float_label))
                    .push(knob_float)
                    .push(Text::new(&self.knob_int_label))
                    .push(knob_int),
            )
            .push(
                Column::new()
                    .max_height(400)
                    .width(Length::Fill)
                    .spacing(10)
                    .push(Text::new(&self.knob_db_label))
                    .push(knob_db)
                    .push(Text::new(&self.knob_oct_label))
                    .push(knob_oct),
            )
            .push(
                Column::new()
                    .max_height(400)
                    .width(Length::Fill)
                    .spacing(10)
                    .push(Text::new(&self.knob_circle_label))
                    .push(knob_circle)
                    .push(Text::new(&self.knob_line_label))
                    .push(knob_line), //.push(Text::new(&self.knob_texture_label))
                                      //.push(knob_texture)
            );

        let content = Column::new()
            .spacing(20)
            .padding(20)
            .push(knob_row)
            .push(Text::new(&self.output_text).size(16));

        Step::container("Knobs").push(content).into()
    }
}
