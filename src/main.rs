#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[macro_use]
extern crate glium;

mod graphics;
mod state;
mod vertex;

use glium::glutin;
use graphics::*;
use state::*;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use std::time::Instant;
use vertex::Vertex;

static FRAME_TIME: Duration = Duration::from_millis((1000. / 60.) as u64);

fn main() {
    implement_vertex!(Vertex, position, color);

    let (mut events_loop, display, shader, parameters) = init_graphics();
    let uniforms = uniform!(
        alpha: 0.3_f32,
    );
    let mut state = State::default();

    let (animation_timer_tx, animation_timer_rx) = mpsc::channel::<()>();
    let loop_proxy = events_loop.create_proxy();
    thread::spawn(move || loop {
        animation_timer_rx.recv().unwrap();
        while animation_timer_rx.try_recv().is_err() {
            loop_proxy.wakeup().unwrap();
            thread::sleep(FRAME_TIME);
        }
    });

    events_loop.run_forever(|event| {
        let (keep_running, redraw) = state.handle_event(event, &animation_timer_tx);

        if keep_running {
            if redraw {
                draw_frame(&display, &shader, &uniforms, &parameters, &state);
                state.last_frame = Instant::now();
            }
            glutin::ControlFlow::Continue
        } else {
            glutin::ControlFlow::Break
        }
    });
}
