use std::io::Write;
use std::time::{Duration, Instant};

use sdl2::event::{Event, WindowEvent};
use sdl2::gfx::primitives::DrawRenderer;
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

#[derive(Debug)]
struct Meta {
    width: i16,
    height: i16,
}

impl Meta {
    pub fn new(width: i16, height: i16) -> Meta {
        Meta { width, height }
    }
}

pub struct Engine {
    sdl_context: sdl2::Sdl,
    canvas: sdl2::render::Canvas<sdl2::video::Window>,
    meta: Meta,
}

impl Engine {
    pub fn new() -> anyhow::Result<Engine> {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem
            .window("STG Engine", 800, 600)
            .vulkan()
            .fullscreen_desktop()
            .position_centered()
            .build()?;

        let (width, height) = window.size();
        let meta = Meta::new(width.try_into().unwrap(), height.try_into().unwrap());
        let canvas = window.into_canvas().build()?;

        println!("Meta: {:?}", meta);

        Ok(Engine {
            sdl_context,
            canvas,
            meta,
        })
    }

    pub fn run(&mut self) {
        let mut event_pump = self.sdl_context.event_pump().unwrap();
        let mut state = State::default();

        // init Game window
        // display Box
        const CHARACTER_RADIUS: i16 = 25;
        const NORMAL_BOX_SPEED: i32 = 10;
        const SLOW_BOX_SPEED: i32 = 5;

        let (window_width, window_height) = self.canvas.output_size().unwrap();

        let position_x = (window_width / 2) as i32;
        let position_y = (window_height / 2) as i32;
        state.position = (position_x, position_y);

        self.canvas.set_draw_color(Color::RGB(255, 255, 255));
        self.canvas.clear();

        self.canvas
            .filled_circle(
                position_x.try_into().unwrap(),
                position_y.try_into().unwrap(),
                CHARACTER_RADIUS,
                Color::BLACK,
            )
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

            let box_color = if keyboad_state.is_scancode_pressed(Scancode::Z) {
                Color::RED
            } else {
                Color::BLACK
            };

            if state.x() < 0 {
                state.position.0 = 0;
            }
            if state.y() < 0 {
                state.position.1 = 0;
            }
            if state.x() > (self.meta.width - CHARACTER_RADIUS) as i32 {
                state.position.0 = self.meta.width.into();
            }
            if state.y() > (self.meta.height - CHARACTER_RADIUS) as i32 {
                state.position.1 = self.meta.height.into();
            }

            self.canvas.set_draw_color(Color::RGB(255, 255, 255));
            self.canvas.clear();

            self.canvas
                .filled_circle(
                    state.x().try_into().unwrap(),
                    state.y().try_into().unwrap(),
                    CHARACTER_RADIUS,
                    box_color,
                )
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
