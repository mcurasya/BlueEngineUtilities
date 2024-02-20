/*
 * Blue Engine by Elham Aryanpur
 *
 * Basic GUI example
 *
 * Licensed under Apache-2.0
*/

// Basic imports
use blue_engine::{
    header::{Engine, ObjectSettings},
    primitive_shapes::triangle,
};

use iced_wgpu::Renderer;
use iced_widget::{slider, text_input, Column, Row, Text};
use iced_winit::core::{Alignment, Color, Element, Length};
use iced_winit::runtime::{Command, Program};
use iced_winit::style::Theme;

pub struct Controls {
    background_color: Color,
    text: String,
}

#[derive(Debug, Clone)]
pub enum Message {
    BackgroundColorChanged(Color),
    TextChanged(String),
}

impl Controls {
    pub fn new() -> Controls {
        Controls {
            background_color: Color::BLACK,
            text: String::default(),
        }
    }

    pub fn background_color(&self) -> Color {
        self.background_color
    }
}

impl Program for Controls {
    type Renderer = Renderer;
    type Message = Message;
    type Theme = Theme;

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::BackgroundColorChanged(color) => {
                self.background_color = color;
            }
            Message::TextChanged(text) => {
                self.text = text;
            }
        }

        Command::none()
    }

    fn view(&self) -> Element<Message, Theme, Self::Renderer> {
        let background_color = self.background_color;
        let text = &self.text;

        let sliders = Row::new()
            .width(500)
            .spacing(20)
            .push(
                slider(0.0..=1.0, background_color.r, move |r| {
                    Message::BackgroundColorChanged(Color {
                        r,
                        ..background_color
                    })
                })
                .step(0.01),
            )
            .push(
                slider(0.0..=1.0, background_color.g, move |g| {
                    Message::BackgroundColorChanged(Color {
                        g,
                        ..background_color
                    })
                })
                .step(0.01),
            )
            .push(
                slider(0.0..=1.0, background_color.b, move |b| {
                    Message::BackgroundColorChanged(Color {
                        b,
                        ..background_color
                    })
                })
                .step(0.01),
            );

        Row::new()
            .width(Length::Fill)
            .height(Length::Fill)
            .align_items(Alignment::End)
            .push(
                Column::new()
                    .width(Length::Fill)
                    .align_items(Alignment::End)
                    .push(
                        Column::new()
                            .padding(10)
                            .spacing(10)
                            .push(Text::new("Background color").style(Color::WHITE))
                            .push(sliders)
                            .push(
                                Text::new(format!("{background_color:?}"))
                                    .size(14)
                                    .style(Color::WHITE),
                            )
                            .push(text_input("Placeholder", text).on_input(Message::TextChanged)),
                    ),
            )
            .into()
    }
}

fn main() {
    // Initialize the engine with default settings
    let mut engine = Engine::new().expect("win");

    // Add a triangle to the screen
    triangle(
        "triangle",
        ObjectSettings::default(),
        &mut engine.renderer,
        &mut engine.objects,
    )
    .unwrap();

    let controls = Controls::new();

    // Start the iced context
    let gui_context = blue_engine_utilities::iced::Iced::new(
        &engine.event_loop,
        &mut engine.window,
        &mut engine.renderer,
    );

    // We add the gui as plugin, which runs once before everything else to fetch events, and once during render times for rendering and other stuff
    engine.plugins.push(Box::new(gui_context));

    let mut color = [1f32, 1f32, 1f32, 1f32];

    // Update loop
    engine
        .update_loop(move |_, window, objects, _, _, plugins| {})
        .expect("Error during update loop");
}
