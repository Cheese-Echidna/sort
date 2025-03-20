use std::sync::Arc;
use std::cell::UnsafeCell;

use alea;

mod algorithms;

struct Array {
    inner: UnsafeCell<Vec<usize>>
}

unsafe impl Sync for Array {}

pub const WIDTH: usize = 1000;
pub const HEIGHT: usize = 1000;

use error_iter::ErrorIter as _;
use log::error;
use pixels::{Error, Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

fn main() -> Result<(), Error> {
    let mut their_ref = Arc::new(array_rand());
    let our_ref = Arc::clone(&their_ref);


    env_logger::init();
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let window = {
        let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
        WindowBuilder::new()
            .with_title("Sorting...")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(WIDTH as usize, HEIGHT as usize, surface_texture)?
    };

    let sorting_thread = std::thread::spawn(move || {
       sort(&mut their_ref)
    });

    event_loop.run(move |event, _, control_flow| {
        // Draw the current frame
        if let Event::RedrawRequested(_) = event {
            draw(&our_ref, pixels.frame_mut());
            if let Err(err) = pixels.render() {
                log_error("pixels.render", err);
                *control_flow = ControlFlow::Exit;
                return;
            }
        }

        // Handle input events
        if input.update(&event) {
            // Close events
            if input.key_pressed(VirtualKeyCode::Escape) || input.close_requested() {
                *control_flow = ControlFlow::Exit;
                return;
            }

            // Resize the window
            if let Some(size) = input.window_resized() {
                if let Err(err) = pixels.resize_surface(size.width, size.height) {
                    log_error("pixels.resize_surface", err);
                    *control_flow = ControlFlow::Exit;
                    return;
                }
            }

            window.request_redraw();
        }
    });
}

fn log_error<E: std::error::Error + 'static>(method_name: &str, err: E) {
    error!("{method_name}() failed: {err}");
    for source in err.sources().skip(1) {
        error!("  Caused by: {source}");
    }
}

fn draw(our_ref: &ArcArray, array: &mut [u8]) {
    // wipe screen
    array.iter_mut().for_each(|x| *x = 0);

    assert_eq!(our_ref.len(), WIDTH);
    assert_eq!(WIDTH, HEIGHT);
    for (x, column) in our_ref.iter().enumerate() {
        for i in 0..=*column {
            let y = HEIGHT + 1 - i as usize;

            let index = (y * WIDTH + x) * 4;
            array[index] = 255;
            array[index + 1] = 255;
            array[index + 2] = 255;
            array[index + 3] = 255;
        }
    }
}

fn sort(vec: &mut Array) {
    algorithms::swap_sort::sort(vec)
}

fn array_rand() -> Array {
    alea::set_seed(10);
    let mut vec = Vec::new();
    for i in 0..WIDTH {
        vec.push
    };

}

fn array_sorted(ray: &Array) -> bool {
    for i in 0..MAX_LENGTH - 1 {
        if ray[i] > ray[i + 1] {
            return false;
        }
    }
    true
}