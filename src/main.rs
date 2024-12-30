use std::io::Write;
use std::time::{Duration, Instant};

use sdl2::event::{Event, WindowEvent};
use sdl2::keyboard::{Keycode, Scancode};
use sdl2::pixels::Color;
use sdl2::rect::Rect;

#[derive(Default)]
struct State {
    position: (i32, i32),
}

impl State {
    fn x(&self) -> i32 {
        self.position.0
    }

    fn y(&self) -> i32 {
        self.position.1
    }
}

struct Application {
    sdl_context: sdl2::Sdl,
    canvas: sdl2::render::Canvas<sdl2::video::Window>,
}

impl Application {
    fn new() -> anyhow::Result<Application> {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem
            .window("STG Engine", 800, 600)
            .vulkan()
            .fullscreen_desktop()
            .position_centered()
            .build()?;

        let mut canvas = window.into_canvas().build()?;

        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.clear();
        canvas.present();

        Ok(Application {
            sdl_context,
            canvas,
        })
    }

    fn run(&mut self) {
        let mut event_pump = self.sdl_context.event_pump().unwrap();
        let mut state = State::default();

        // init Game window
        // display Box
        const BOX_WIDTH: u32 = 50;
        const BOX_HEIGHT: u32 = 50;
        const NORMAL_BOX_SPEED: i32 = 10;
        const SLOW_BOX_SPEED: i32 = 5;

        let (window_width, window_height) = self.canvas.output_size().unwrap();

        let position_x = ((window_width as i32) / 2) - (BOX_WIDTH as i32 / 2);
        let position_y = ((window_height as i32) / 2) - (BOX_HEIGHT as i32 / 2);
        state.position = (position_x, position_y);

        self.canvas.set_draw_color(Color::RGB(255, 255, 255));
        self.canvas.clear();

        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
        self.canvas
            .fill_rect(Rect::new(state.x(), state.y(), BOX_WIDTH, BOX_HEIGHT))
            .unwrap();
        self.canvas.present();

        let target_frame_duration = Duration::from_secs_f64(1.0 / 60.0);
        let mut fps_timer = std::time::Instant::now();
        let mut frame_count = 0;

        'running: loop {
            let frame_start = std::time::Instant::now();
            for event in event_pump.poll_iter() {
                match event {
                    Event::Window { win_event, .. } => match win_event {
                        WindowEvent::Resized(..) => {
                            self.canvas.clear();
                            self.canvas.present();
                        }
                        _ => {}
                    },
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => break 'running,
                    _ => {}
                }
            }
            // The rest of the game loop goes here...
            let keyboad_state = event_pump.keyboard_state();
            if keyboad_state.is_scancode_pressed(Scancode::Up) {
                if keyboad_state.is_scancode_pressed(Scancode::LShift)
                    || keyboad_state.is_scancode_pressed(Scancode::RShift)
                {
                    state.position.1 -= SLOW_BOX_SPEED;
                } else {
                    state.position.1 -= NORMAL_BOX_SPEED;
                }
            }
            if keyboad_state.is_scancode_pressed(Scancode::Down) {
                if keyboad_state.is_scancode_pressed(Scancode::LShift)
                    || keyboad_state.is_scancode_pressed(Scancode::RShift)
                {
                    state.position.1 += SLOW_BOX_SPEED;
                } else {
                    state.position.1 += NORMAL_BOX_SPEED;
                }
            }
            if keyboad_state.is_scancode_pressed(Scancode::Left) {
                if keyboad_state.is_scancode_pressed(Scancode::LShift)
                    || keyboad_state.is_scancode_pressed(Scancode::RShift)
                {
                    state.position.0 -= SLOW_BOX_SPEED;
                } else {
                    state.position.0 -= NORMAL_BOX_SPEED;
                }
            }
            if keyboad_state.is_scancode_pressed(Scancode::Right) {
                if keyboad_state.is_scancode_pressed(Scancode::LShift)
                    || keyboad_state.is_scancode_pressed(Scancode::RShift)
                {
                    state.position.0 += SLOW_BOX_SPEED;
                } else {
                    state.position.0 += NORMAL_BOX_SPEED;
                }
            }

            if state.x() < 0 {
                state.position.0 = 0;
            }
            if state.y() < 0 {
                state.position.1 = 0;
            }
            if state.x() > (window_width as i32) - (BOX_WIDTH as i32) {
                state.position.0 = (window_width as i32) - (BOX_WIDTH as i32);
            }
            if state.y() > (window_height as i32) - (BOX_HEIGHT as i32) {
                state.position.1 = (window_height as i32) - (BOX_HEIGHT as i32);
            }

            self.canvas.set_draw_color(Color::RGB(255, 255, 255));
            self.canvas.clear();

            self.canvas.set_draw_color(Color::RGB(0, 0, 0));
            self.canvas
                .fill_rect(Rect::new(state.x(), state.y(), BOX_WIDTH, BOX_HEIGHT))
                .unwrap();
            self.canvas.present();

            frame_count += 1;
            if fps_timer.elapsed() >= Duration::from_secs(1) {
                let current_fps = frame_count as f64 / fps_timer.elapsed().as_secs_f64();
                frame_count = 0;
                fps_timer = Instant::now();

                print!(
                    "\rFPS: {:.2} Position: ({}, {})",
                    current_fps,
                    state.x(),
                    state.y()
                );
                std::io::stdout().flush().unwrap();
            }

            let frame_duration = frame_start.elapsed();
            if frame_duration < target_frame_duration {
                std::thread::sleep(target_frame_duration - frame_duration);
            }
        }
    }
}

pub fn main() -> anyhow::Result<()> {
    let mut app = Application::new()?;
    app.run();
    Ok(())
}
