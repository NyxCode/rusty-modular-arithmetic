use std::time::{Instant, Duration};

#[derive(Debug, Clone)]
pub enum Animation {
    Factor(f32),
    Divisions(f32),
}

#[derive(Debug)]
pub struct State {
    pub scale: f32,
    pub divisions: u16,
    pub factor: f32,
    pub current_animation: Option<Animation>,
    pub last_frame: Instant,
}

impl Default for State {
    fn default() -> Self {
        State {
            scale: 0.9,
            divisions: 100,
            factor: 2.,
            current_animation: None,
            last_frame: Instant::now(),
        }
    }
}

use glutin::{ElementState::Pressed, MouseScrollDelta::*, *};
use std::sync::mpsc::Sender;

impl State {
    pub fn handle_event(&mut self, event: Event, animation_timer: &Sender<()>) -> (bool, bool) {
        use glutin::WindowEvent::{CloseRequested, KeyboardInput, MouseWheel, Refresh};

        match event {
            Event::WindowEvent { event, .. } => match event {
                // exit
                CloseRequested { .. } => (false, false),

                // redraw
                Refresh => (true, true),

                // mutate & redraw
                MouseWheel {
                    delta, modifiers, ..
                } => {
                    (true, self.on_mouse_wheel(delta, modifiers))
                }

                // mutate & redraw
                KeyboardInput { input, .. } => {
                    self.on_key_pressed(&input, animation_timer);
                    (true, true)
                }

                // don't redraw
                _ => (true, false),
            },
            // mutate & redraw
            Event::Awakened => {
                self.handle_animation();
                (true, true)
            }

            // don't redraw
            _ => (true, false),
        }
    }

    fn handle_animation(&mut self) {
        use Animation::*;

        match self.current_animation {
            Some(Factor(speed)) => self.factor += speed,
            Some(Divisions(speed)) => self.divisions = (self.divisions + speed as u16).max(0),
            _ => (),
        }
    }

    fn manipulate(&mut self, factor: f32, modifiers: ModifiersState) -> bool {
        if Instant::now().duration_since(self.last_frame) < ::FRAME_TIME {
            return false;
        }

        if modifiers.ctrl {
            let diff = if modifiers.shift { 10. } else { 1. } * factor;

            self.divisions = (f32::from(self.divisions) + diff).max(0.) as u16;
        } else {
            let diff = if modifiers.shift { 0.2 } else { 0.01 } * factor;

            self.factor += diff;
        }

        true
    }

    fn on_mouse_wheel(&mut self, delta: MouseScrollDelta, mods: ModifiersState) -> bool {
        use glutin::dpi::LogicalPosition;

        let amount = match delta {
            LineDelta(_, y) => y,
            PixelDelta(LogicalPosition { y, .. }) => y as f32,
        };

        self.manipulate(amount, mods)
    }

    fn on_key_pressed(&mut self, input: &KeyboardInput, animation_timer: &Sender<()>) -> bool {
        use glutin::VirtualKeyCode::{Add, Down, Escape, Left, Right, Space, Subtract, Up, A};

        if input.state != Pressed {
            false
        } else if let Some(key) = input.virtual_keycode {
            match key {
                Up | Right | Add => self.manipulate(1., input.modifiers),
                Down | Left | Subtract => self.manipulate(-1., input.modifiers),
                Space | A => {
                    animation_timer.send(()).unwrap();
                    match self.current_animation {
                        Some(_) => self.current_animation = None,
                        None => {
                            let mut speed = if input.modifiers.shift { 10. } else { 1. };
                            if input.modifiers.alt {
                                speed *= -1.;
                            }
                            let animation = if input.modifiers.ctrl {
                                Animation::Divisions(speed)
                            } else {
                                Animation::Factor(speed * 0.002)
                            };
                            self.current_animation = Some(animation);
                        }
                    };
                    true
                }
                Escape => {
                    let current_animation = self.current_animation.clone();
                    *self = State {
                        current_animation,
                        ..State::default()
                    };
                    true
                }
                _ => false,
            }
        } else {
            false
        }
    }
}
