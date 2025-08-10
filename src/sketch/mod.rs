use std::time::Instant;
use crate::sketch::algorithms::*;
use crate::sketch::player::SortPlayer;
use egui::{ComboBox, Window};
pub use list::*;
use nannou::prelude::*;
use nannou::winit::event::VirtualKeyCode;
use nannou_egui::egui::Slider;
use nannou_egui::{self, egui, Egui};
use strum::IntoEnumIterator;
use crate::sketch::methods::{RenderMethod, SortMethod};
use nannou::wgpu::{Backends, DeviceDescriptor, Limits};
use std::cell::RefCell;
use crate::resort;

mod algorithms;
mod list;
mod player;
mod renderers;
mod methods;

pub async fn run_app(width: u32, height: u32) {
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    // Since ModelFn is not a closure we need this workaround to pass the calculated model
    thread_local!(static MODEL: RefCell<Option<Model>> = Default::default());

    app::Builder::new_async(move |app| {
        Box::new(async move {
            create_window(app, width, height).await;
            let model = Model::new(app);
            MODEL.with(|m| m.borrow_mut().replace(model));
            MODEL.with(|m| m.borrow_mut().take().unwrap())
        })
    })
        .backends(Backends::PRIMARY | Backends::GL)
        .update(update)
        .run_async()
        .await;
}

async fn create_window(app: &App, width: u32, height: u32) {
    let device_desc = DeviceDescriptor {
        limits: Limits {
            max_texture_dimension_2d: 8192,
            ..Limits::downlevel_webgl2_defaults()
        },
        ..Default::default()
    };

    let app = if (width * height) == 0 {
        app.new_window().fullscreen()
    } else {
        app.new_window()
            .size(width, height)
    };

    app.device_descriptor(device_desc)
        .title("sorting")
        .view(view)
        .event(event)
        .raw_event(raw_window_event)
        .build_async()
        .await
        .unwrap();
}




struct Model {
    player: SortPlayer,
    egui: Egui,
    sorter: SortMethod,
    length_log2: usize,
    renderer: RenderMethod,
    last_play: Instant
}

impl Model {
    fn new(app: &App) -> Model {
        let egui = Egui::from_window(&app.main_window());

        Model {
            player: SortPlayer::new(2_usize.pow(8), quicksort::sort, 50),
            egui,
            sorter: SortMethod::Quick,
            length_log2: 8,
            renderer: RenderMethod::Classic,
            last_play: Instant::now(),
        }
    }
}

fn event(_app: &App, model: &mut Model, event: WindowEvent) {
    if let KeyPressed(key) = event {
        match key {
            VirtualKeyCode::Up => {
                model.length_log2 += 1;
                resort!(model);
            }
            VirtualKeyCode::Down => {
                model.length_log2 -= 1;
                resort!(model);
            }
            VirtualKeyCode::Right => {model.player.playback_rate *= 2}
            VirtualKeyCode::Left => {model.player.playback_rate /= 2}
            VirtualKeyCode::Space => {model.player.reset_play()}
            _ => {}
        }

        let key = key as u32;
        if key < 10 {
            RenderMethod::iter()
                .nth(key as usize)
                .map(|x| model.renderer = x);
        }
        else if key > 36 && key < 61 {
            let f_key = key - 37;
            SortMethod::iter()
                .nth(f_key as usize)
                .map(|x| {
                    if x != model.sorter {
                        model.sorter = x;
                        resort!(model);
                    }
                });
        }
    }
}

fn update(_app: &App, model: &mut Model, update: Update) {
    gui(_app, model, update);

    let since_last = model.last_play.elapsed();
    if since_last.as_secs_f64() < 1.0 / model.player.playback_rate as f64 {
        // println!("Since last play = {:?}", since_last);
        return
    } else {
        // println!("Played");
    }

    let raw_updates = model.player.playback_rate as f64 * update.since_last.as_secs_f64();
    let moves = (raw_updates).ceil() as usize;

    model.player.play(moves);
    model.last_play = Instant::now();
}

fn gui(_app: &App, model: &mut Model, update: Update) {
    let egui = &mut model.egui;
    egui.set_elapsed_time(update.since_start);
    egui.begin_frame();

    Window::new("Settings").show(egui.ctx(), |ui| {
        ComboBox::from_label("Renderer")
            .selected_text(format!("{}", model.renderer))
            .show_ui(ui, |ui| {
                for option in RenderMethod::iter() {
                    ui.selectable_value(&mut model.renderer, option, format!("{option}"));
                }
            });
        ComboBox::from_label("Algorithm")
            .selected_text(format!("{}", model.sorter))
            .show_ui(ui, |ui| {
                for option in SortMethod::iter() {
                    let response =
                        ui.selectable_value(&mut model.sorter, option, format!("{}", option));
                    if response.changed() {
                        resort!(model);
                    }
                }
            });

        ui.add(Slider::new(&mut model.player.playback_rate, 1..=10000).text("Playback rate (ops/secs)"));
        let res = ui.add(Slider::new(&mut model.length_log2, 1..=16).text("Length (log2)"));
        if res.changed() {
            resort!(model);
        }
        if ui.button("Replay").clicked() {
            model.player.reset_play();
        }
    });
}

fn view(app: &App, model: &Model, frame: Frame) {
    let aspect = app.window_rect().x.len() / app.window_rect().y.len();
    let draw = app
        .draw()
        .scale_x(app.window_rect().x.len())
        .scale_y(app.window_rect().y.len())
        .xy(Vec2::splat(-0.5));

    (0.0, 1.0, 0.0, 1.0);
    draw.background().color(Srgb::new(20_u8, 20, 20));
    model.renderer.draw(&model.player, &draw, aspect);
    draw.to_frame(app, &frame).unwrap();
    model.egui.draw_to_frame(&frame).unwrap();
}

fn raw_window_event(_app: &App, model: &mut Model, event: &nannou::winit::event::WindowEvent) {
    // Let egui handle things like keyboard and mouse input.
    model.egui.handle_raw_event(event);
}

