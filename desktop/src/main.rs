use std::fs;
use std::sync::Arc;
use std::time::{Duration, Instant};

use clap::Parser;
use emulator::Chip8;
use pixels::{Pixels, SurfaceTexture};
use winit::application::ApplicationHandler;
use winit::dpi::LogicalSize;
use winit::event::{KeyEvent, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::keyboard::{KeyCode, PhysicalKey};
use winit::window::{Window, WindowAttributes};

use crate::args::Args;

const WINDOW_WIDTH: u32 = emulator::SCREEN_WIDTH as u32;
const WINDOW_HEIGHT: u32 = emulator::SCREEN_HEIGHT as u32;
const WINDOW_SCALE: u32 = 20;

const CPU_FREQUENCY: u32 = 600; // Hz
const TIMER_FREQUENCY: u32 = 60; // Hz

mod args;

struct App {
    pub args: Args,
    pub window: Option<Arc<Window>>,
    pub pixels: Option<Pixels<'static>>,
    pub chip8: Option<Chip8>,
    pub last_cpu_tick_time: Instant,
    pub last_timer_tick_time: Instant,
    audio_stream: Option<rodio::OutputStream>,
    audio_handle: Option<rodio::OutputStreamHandle>,
    beep_sink: Option<rodio::Sink>,
}

impl App {
    fn new(args: Args) -> Self {
        let (audio_stream, audio_handle) = rodio::OutputStream::try_default().unwrap();
        App {
            args,
            window: None,
            pixels: None,
            chip8: None,
            last_cpu_tick_time: Instant::now(),
            last_timer_tick_time: Instant::now(),
            audio_stream: Some(audio_stream),
            audio_handle: Some(audio_handle),
            beep_sink: None,
        }
    }

    fn draw_screen(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // Unwrap components, returning if they are not initialized.
        let pixels = self.pixels.as_mut().ok_or("Pixels not initialized")?;
        let chip8 = self.chip8.as_ref().ok_or("Chip8 not initialized")?;

        let frame = pixels.frame_mut();
        let emu_screen = chip8.get_display();

        // Iterate over each pixel of the Chip-8's screen.
        for y in 0..WINDOW_HEIGHT as usize {
            for x in 0..WINDOW_WIDTH as usize {
                // Calculate starting index for current pixel in the frame buffer.
                let pixel_idx = (y * WINDOW_WIDTH as usize + x) * 4;
                let is_pixel_on = emu_screen[y][x];

                // Set pixel color (black or white) based on the emulator's display state.
                if is_pixel_on {
                    // White pixel (R, G, B, A)
                    frame[pixel_idx] = 0xFF;
                    frame[pixel_idx + 1] = 0xFF;
                    frame[pixel_idx + 2] = 0xFF;
                    frame[pixel_idx + 3] = 0xFF; // Full opacity
                } else {
                    // Black pixel (R, G, B, A)
                    frame[pixel_idx] = 0x00;
                    frame[pixel_idx + 1] = 0x00;
                    frame[pixel_idx + 2] = 0x00;
                    frame[pixel_idx + 3] = 0xFF; // Full opacity
                }
            }
        }
        Ok(())
    }
}

impl ApplicationHandler for App {
    // Function
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        // Calculate initial window size, adjusted for scale.
        let window_pixel_width = WINDOW_WIDTH * WINDOW_SCALE;
        let window_pixel_height = WINDOW_HEIGHT * WINDOW_SCALE;

        // Set window attributes.
        let window_attributes = WindowAttributes::default()
            .with_title("CHIP-8 Emulator")
            .with_inner_size(LogicalSize::new(window_pixel_width, window_pixel_height))
            .with_min_inner_size(LogicalSize::new(window_pixel_width, window_pixel_height));

        // Create window.
        let window = event_loop
            .create_window(window_attributes)
            .expect("Failed to create Winit window");

        // Wrap the window in an Arc to allow shared ownership.
        let window_arc = Arc::new(window);

        // Initialize pixels renderer.
        let window_size = window_arc.inner_size();
        let surface_texture =
            SurfaceTexture::new(window_size.width, window_size.height, window_arc.clone());

        // Pixels expects the internal resolution (64x32 for CHIP-8)
        let pixels = Pixels::new(
            emulator::SCREEN_WIDTH as u32,
            emulator::SCREEN_HEIGHT as u32,
            surface_texture,
        )
        .expect("Could not create pixels.rs instance."); // TODO: Better error handling.

        // Initialize Chip8 emulator core.
        let mut chip8 = Chip8::new();

        // Load ROM from file specified in arguments.
        let rom_data = fs::read(&self.args.rom_path)
            .expect(&format!("Failed to read ROM file '{}'", self.args.rom_path)); // TODO: Better error handling.

        chip8.load(&rom_data); // TODO: Implement error handling.

        // Store all initialized components in App struct.
        self.pixels = Some(pixels);
        self.chip8 = Some(chip8);
        self.window = Some(window_arc); // Store the Arc'd window

        // Reset last CPU and timer tick times.
        self.last_cpu_tick_time = Instant::now();
        self.last_timer_tick_time = Instant::now();
    }

    // Function that is called when the window recieves an event.
    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        let Some(pixels) = self.pixels.as_mut() else {
            return;
        };
        let Some(chip8) = self.chip8.as_mut() else {
            return;
        };

        match event {
            // Handle window being closed.
            WindowEvent::CloseRequested => event_loop.exit(),

            // Handle window being resized.
            WindowEvent::Resized(size) => {
                if let Err(err) = pixels.resize_surface(size.width, size.height) {
                    eprintln!("Failed to resize pixels surface: {:?}", err); // TODO: Better error handling
                    event_loop.exit();
                }
            }

            // Handle keyboard input on the window.
            #[rustfmt::skip] // Skip formatting for this block due to weird rustfmt behavior.
            WindowEvent::KeyboardInput { event: KeyEvent { physical_key, state, .. }, .. } => {
                // Get the key's state.
                let is_pressed = state.is_pressed();

                // If the key is 'esc', close the application.
                if physical_key == PhysicalKey::Code(KeyCode::Escape) {
                    event_loop.exit();
                }

                // Map all other keys to Chip-8 keys (or discard within function).
                if let Some(chip8_key_idx) = map_keyboard_to_chip8_key(physical_key) {
                    chip8.keypress(chip8_key_idx, is_pressed);
                }
            }

            // Ignore other window events.
            _ => {}
        }
    }

    // Function that is called when the event loop is about to wait for new events.
    fn about_to_wait(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        // Unwrap components, returning if they are not initialized.
        let chip8 = match self.chip8.as_mut() {
            Some(chip8) => chip8,
            None => return, // If chip8 is None, return early
        };

        let now = Instant::now();

        // Calculate the time it takes to execute one CPU instruction (in microseconds).
        let cpu_tick_duration = Duration::from_micros(1_000_000 / CPU_FREQUENCY as u64);
        // Loop to execute CPU ticks until we've caught up with real time.
        while now.duration_since(self.last_cpu_tick_time) >= cpu_tick_duration {
            // Tick the Chip-8 CPU once.
            chip8.tick();

            // Advance the last tick time by the speed of a single CPU instruction.
            self.last_cpu_tick_time += cpu_tick_duration;
        }

        // Calculate the time it takes to execute one timer tick (in microseconds).
        let timer_tick_duration = Duration::from_micros(1_000_000 / TIMER_FREQUENCY as u64);
        // Loop to execute timer ticks until we've caught up with real time.
        while now.duration_since(self.last_timer_tick_time) >= timer_tick_duration {
            // Tick the Chip-8 timers once.
            chip8.tick_timers();

            // Get the sound timer.
            let st = chip8.get_sound_timer();

            if st > 0 {
                if self.beep_sink.is_none() || self.beep_sink.as_ref().unwrap().empty() {
                    if let Some(audio_handle) = &self.audio_handle {
                        let sink = rodio::Sink::try_new(audio_handle).unwrap();
                        sink.append(rodio::source::SineWave::new(440.0));
                        sink.play();
                        self.beep_sink = Some(sink);
                    }
                }
            } else {
                if let Some(sink) = &self.beep_sink {
                    sink.stop();
                }
                self.beep_sink = None;
            }

            // Advance the last tick time by the speed of a single timer tick.
            self.last_timer_tick_time += timer_tick_duration;
        }

        // Draw the emulator's current screen state to the pixels buffer.
        self.draw_screen().expect("Failed to draw emulator screen");

        let (window, pixels) = match (self.window.as_ref(), self.pixels.as_mut()) {
            (Some(window), Some(pixels)) => (window, pixels),
            _ => return, // If any are None, return early
        };

        if let Err(err) = pixels.render() {
            eprintln!("Failed to render pixels: {:?}", err);
            event_loop.exit();
        }

        // `window` is used here, so its immutable borrow of `self.window` is still active.
        window.request_redraw();

        event_loop.set_control_flow(ControlFlow::Poll);
    }
}

fn main() {
    // Parse command line arguments
    let args = args::Args::parse();

    // Initialize application.
    let mut app = App::new(args);

    // Initialize event loop & run the application.
    let event_loop = EventLoop::new().unwrap();
    event_loop.run_app(&mut app).unwrap();
}

fn map_keyboard_to_chip8_key(physical_key: PhysicalKey) -> Option<usize> {
    // Chip8 Keypad:
    // 1 2 3 C
    // 4 5 6 D
    // 7 8 9 E
    // A 0 B F

    match physical_key {
        PhysicalKey::Code(KeyCode::Digit1) => Some(0x1),
        PhysicalKey::Code(KeyCode::Digit2) => Some(0x2),
        PhysicalKey::Code(KeyCode::Digit3) => Some(0x3),
        PhysicalKey::Code(KeyCode::Digit4) => Some(0xC),
        PhysicalKey::Code(KeyCode::KeyQ) => Some(0x4),
        PhysicalKey::Code(KeyCode::KeyW) => Some(0x5),
        PhysicalKey::Code(KeyCode::KeyE) => Some(0x6),
        PhysicalKey::Code(KeyCode::KeyR) => Some(0xD),
        PhysicalKey::Code(KeyCode::KeyA) => Some(0x7),
        PhysicalKey::Code(KeyCode::KeyS) => Some(0x8),
        PhysicalKey::Code(KeyCode::KeyD) => Some(0x9),
        PhysicalKey::Code(KeyCode::KeyF) => Some(0xE),
        PhysicalKey::Code(KeyCode::KeyZ) => Some(0xA),
        PhysicalKey::Code(KeyCode::KeyX) => Some(0x0),
        PhysicalKey::Code(KeyCode::KeyC) => Some(0xB),
        PhysicalKey::Code(KeyCode::KeyV) => Some(0xF),
        _ => None,
    }
}
