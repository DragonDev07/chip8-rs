use std::fs;
use std::time::Duration;
use std::{sync::Arc, time::Instant};

use emulator::Emulator;
use miette::Report;
use pixels::{Pixels, SurfaceTexture};
use winit::application::ApplicationHandler;
use winit::dpi::LogicalSize;
use winit::event::{KeyEvent, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::keyboard::{KeyCode, PhysicalKey};
use winit::window::WindowAttributes;
use winit::{event_loop::ActiveEventLoop, window::Window};

use anyhow::{Context, Result};

use crate::args::Args;
use crate::keyboard::map_keyboard;
use crate::sound::Sound;

const WINDOW_WIDTH: u32 = emulator::constants::DISPLAY_WIDTH as u32;
const WINDOW_HEIGHT: u32 = emulator::constants::DISPLAY_HEIGHT as u32;
const WINDOW_SCALE: u32 = 20;
const TIMER_FREQUENCY: u32 = 60;

pub struct App {
    pub args: Args,
    pub window: Option<Arc<Window>>,
    pub pixels: Option<Pixels<'static>>,
    pub emu: Option<Emulator>,
    pub last_cpu_tick_time: Instant,
    pub last_timer_tick_time: Instant,
    pub sound: Sound,
}

impl App {
    pub fn new(args: Args) -> Self {
        Self {
            args,
            window: None,
            pixels: None,
            emu: None,
            last_cpu_tick_time: Instant::now(),
            last_timer_tick_time: Instant::now(),
            sound: Sound::new(),
        }
    }

    pub fn run(&mut self) -> Result<()> {
        let event_loop = EventLoop::new().context("Failed to create event loop.")?;
        event_loop.run_app(self).context("Failed to run app")?;
        Ok(())
    }

    fn draw_screen(&mut self) -> Result<()> {
        let pixels = self.pixels.as_mut().context("Pixels not initialized")?;
        let emu: &mut Emulator = self.emu.as_mut().context("Emulator not initialized")?;

        let frame = pixels.frame_mut();
        let emu_screen = emu.get_display_buffer();

        for y in 0..WINDOW_HEIGHT as usize {
            for x in 0..WINDOW_WIDTH as usize {
                let pixel_idx = (y * WINDOW_WIDTH as usize + x) * 4;
                let is_pixel_on = emu_screen[y][x];

                if is_pixel_on {
                    frame[pixel_idx] = 0xFF;
                    frame[pixel_idx + 1] = 0xFF;
                    frame[pixel_idx + 2] = 0xFF;
                    frame[pixel_idx + 3] = 0xFF;
                } else {
                    frame[pixel_idx] = 0x00;
                    frame[pixel_idx + 1] = 0x00;
                    frame[pixel_idx + 2] = 0x00;
                    frame[pixel_idx + 3] = 0xFF;
                }
            }
        }
        Ok(())
    }

    fn step_cpu(&mut self, now: Instant) {
        let emu = match self.emu.as_mut() {
            Some(emu) => emu,
            None => return,
        };

        if let Some(steps) = self.args.steps_per_frame {
            for _ in 0..steps {
                if let Err(e) = emu.cycle() {
                    eprintln!("{:?}", Report::new(e));
                }
            }
        } else {
            let cpu_tick_duration =
                Duration::from_micros(1_000_000 / self.args.cpu_frequency as u64);
            while now.duration_since(self.last_cpu_tick_time) >= cpu_tick_duration {
                if let Err(e) = emu.cycle() {
                    eprintln!("{:?}", Report::new(e));
                }
                self.last_cpu_tick_time += cpu_tick_duration;
            }
        }
    }

    fn tick_timers(&mut self, now: Instant) {
        let emu = match self.emu.as_mut() {
            Some(emu) => emu,
            None => return,
        };

        let timer_tick_duration = Duration::from_micros(1_000_000 / TIMER_FREQUENCY as u64);
        while now.duration_since(self.last_timer_tick_time) >= timer_tick_duration {
            emu.tick_timers();
            let st = emu.get_st();
            if st > 0 {
                self.sound.start_beep();
            } else {
                self.sound.stop_beep();
            }
            self.last_timer_tick_time += timer_tick_duration;
        }
    }

    fn draw_and_render(&mut self, event_loop: &ActiveEventLoop) {
        if let Err(e) = self.draw_screen() {
            eprintln!("Error: {e}");
            for cause in e.chain().skip(1) {
                eprintln!("Caused by: {cause}");
            }
        }

        let (window, pixels) = match (self.window.as_ref(), self.pixels.as_mut()) {
            (Some(window), Some(pixels)) => (window, pixels),
            _ => return,
        };

        if let Err(err) = pixels.render() {
            eprintln!("Failed to render pixels: {:?}", err);
            event_loop.exit();
        }

        window.request_redraw();
        event_loop.set_control_flow(ControlFlow::Poll);
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window_pixel_width = WINDOW_WIDTH * WINDOW_SCALE;
        let window_pixel_height = WINDOW_HEIGHT * WINDOW_SCALE;

        let window_attributes = WindowAttributes::default()
            .with_title("CHIP-8 Emulator")
            .with_inner_size(LogicalSize::new(window_pixel_width, window_pixel_height))
            .with_min_inner_size(LogicalSize::new(window_pixel_width, window_pixel_height));

        let window = match event_loop.create_window(window_attributes) {
            Ok(w) => w,
            Err(e) => {
                eprintln!("Failed to create window: {:?}", e);
                event_loop.exit();
                return;
            }
        };

        let window_arc = Arc::new(window);

        let window_size = window_arc.inner_size();
        let surface_texture =
            SurfaceTexture::new(window_size.width, window_size.height, window_arc.clone());

        let pixels = match Pixels::new(
            emulator::constants::DISPLAY_WIDTH as u32,
            emulator::constants::DISPLAY_HEIGHT as u32,
            surface_texture,
        ) {
            Ok(p) => p,
            Err(e) => {
                eprintln!("Could not create pixels.rs instance: {:?}", e);
                event_loop.exit();
                return;
            }
        };

        let mut emu = Emulator::new();

        let rom_data = match fs::read(&self.args.rom_path)
            .with_context(|| format!("Failed to read ROM file '{}'", self.args.rom_path))
        {
            Ok(data) => data,
            Err(e) => {
                eprintln!("Error: {e}");
                for cause in e.chain().skip(1) {
                    eprintln!("Caused by: {cause}");
                }
                event_loop.exit();
                return;
            }
        };

        if let Err(e) = emu.load_rom(&rom_data) {
            eprintln!("{:?}", Report::new(e));
        }

        self.pixels = Some(pixels);
        self.emu = Some(emu);
        self.window = Some(window_arc);

        self.last_cpu_tick_time = Instant::now();
        self.last_timer_tick_time = Instant::now();
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        let Some(pixels) = self.pixels.as_mut() else {
            return;
        };
        let Some(emu) = self.emu.as_mut() else {
            return;
        };

        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::Resized(size) => {
                if let Err(err) = pixels.resize_surface(size.width, size.height) {
                    eprintln!("Failed to resize pixels surface: {:?}", err);
                    event_loop.exit();
                }
            }
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        physical_key,
                        state,
                        ..
                    },
                ..
            } => {
                let is_pressed = state.is_pressed();
                if physical_key == PhysicalKey::Code(KeyCode::Escape) {
                    event_loop.exit();
                }
                if let Some(chip8_key_idx) = map_keyboard(physical_key) {
                    if is_pressed {
                        if let Err(err) = emu.press_key(chip8_key_idx) {
                            eprintln!("Failed to press key: {:?}", err);
                        }
                    } else {
                        if let Err(err) = emu.release_key(chip8_key_idx) {
                            eprintln!("Failed to release key: {:?}", err);
                        }
                    }
                }
            }
            _ => {}
        }
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        let now = Instant::now();
        self.step_cpu(now);
        self.tick_timers(now);
        self.draw_and_render(event_loop);
    }
}
