use minifb::{Key, Scale, ScaleMode, Window, WindowOptions};
use rand::prelude::*;
use rand::rng;

const WIDTH: usize = 256;
const HEIGHT: usize = 256;
const MAX: usize = WIDTH * HEIGHT;

#[derive(Debug)]
struct DoubleBuffer {
    a: Vec<u8>,
    b: Vec<u8>,
    current: u32,
    rng: ThreadRng,
}

impl DoubleBuffer {
    fn get_swapped(&mut self) -> (&mut Vec<u8>, &mut Vec<u8>) {
        let (current_buf, next_buf) = if self.current == 0 {
            (&mut self.a, &mut self.b)
        } else {
            (&mut self.b, &mut self.a)
        };

        self.current ^= 1; // Swap buffers
        (current_buf, next_buf)
    }

    fn random_fill(&mut self) {
        let buf = if self.current == 0 {
            &mut self.a
        } else {
            &mut self.b
        };

        for i in 0..MAX {
            buf[i] = self.rng.random_range(0..2);
        }
    }
}

fn main() {
    let image_data = include_bytes!("../assets/conway.data");
    let conway: Vec<u8> = image_data
        .iter()
        .map(|b| if *b == 1 { 0x01 } else { 0x00 })
        .collect();

    let mut frame_buffer: Vec<u32> = image_data
        .iter()
        .map(|b| if *b == 1 { 0x00ff_ffff } else { 0x0000_0000 })
        .collect();

    let mut double_buf = DoubleBuffer {
        a: conway,
        b: vec![0; MAX],
        current: 0,
        rng: rng(),
    };

    let mut window = Window::new(
        "CONWAY",
        WIDTH,
        HEIGHT,
        WindowOptions {
            resize: true,
            scale: Scale::X2,
            scale_mode: ScaleMode::AspectRatioStretch,
            ..WindowOptions::default()
        },
    )
    .expect("Unable to Open Window");

    window.set_target_fps(60);
    window.set_background_color(0, 0, 20);

    // Display portrait for ~2 seconds.
    let mut i: u8 = 0;
    while window.is_open() && !window.is_key_down(Key::Escape) && i < 120 {
        window.update_with_buffer(&frame_buffer, WIDTH, HEIGHT).unwrap();
        i +=1;
    }
    
    while window.is_open() && !window.is_key_down(Key::Escape) {
        if window.is_key_down(Key::Space) {
            double_buf.random_fill();
        }

        let (current_buf, next_buf) = double_buf.get_swapped();

        for i in 0..MAX {
            let is_alive = current_buf[i] == 1;

            let mut sum = 0;
            sum += current_buf[i.wrapping_sub(1) % MAX]; // LEFT
            sum += current_buf[i.wrapping_add(1) % MAX]; // RIGHT
            sum += current_buf[i.wrapping_sub(WIDTH - 1) % MAX]; // TOP LEFT CORNER
            sum += current_buf[i.wrapping_sub(WIDTH) % MAX]; // TOP
            sum += current_buf[i.wrapping_sub(WIDTH + 1) % MAX]; // TOP RIGHT CORNER
            sum += current_buf[i.wrapping_add(WIDTH - 1) % MAX]; // BOTTOM LEFT CORNER
            sum += current_buf[i.wrapping_add(WIDTH) % MAX]; // BOTTOM
            sum += current_buf[i.wrapping_add(WIDTH + 1) % MAX]; // BOTTOM RIGHT CORNER

            if sum == 3 || is_alive && sum == 2 {
                next_buf[i] = 1;
                frame_buffer[i] = 0x00ff_ffff;
            } else {
                next_buf[i] = 0;
                frame_buffer[i] = 0;
            }
        }
        // let t = time::Duration::from_millis(250);
        // thread::sleep(t);

        window.update_with_buffer(&frame_buffer, WIDTH, HEIGHT).unwrap();
    }
}
