use std::time::Instant;
use crate::algorithms::*;
use crate::player::SortPlayer;
use egui::{ComboBox, Window};
pub use list::*;
use nannou::prelude::*;
use nannou::winit::event::VirtualKeyCode;
use nannou_egui::egui::Slider;
use nannou_egui::{self, egui, Egui};
use strum::IntoEnumIterator;
use crate::methods::{RenderMethod, SortMethod};

mod algorithms;
mod list;
mod player;
mod renderers;
mod methods;

fn main() {
    nannou::app(model).fullscreen().update(update).run();
}

struct Model {
    _window: window::Id,
    player: SortPlayer,
    egui: Egui,
    sorter: SortMethod,
    length_log2: usize,
    renderer: RenderMethod,
    last_play: Instant
}

fn model(app: &App) -> Model {
    let window_id = app
        .new_window()
        .view(view)
        .event(event)
        .raw_event(raw_window_event)
        .build()
        .unwrap();
    let window = app.window(window_id).unwrap();

    let egui = Egui::from_window(&window);

    Model {
        _window: window_id,
        player: SortPlayer::new(2_usize.pow(8), quicksort::sort, 50),
        egui,
        sorter: SortMethod::Quick,
        length_log2: 8,
        renderer: RenderMethod::Classic,
        last_play: Instant::now(),
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
    draw.background().color(BLACK);
    model.renderer.draw(&model.player, &draw, aspect);
    draw.to_frame(app, &frame).unwrap();
    model.egui.draw_to_frame(&frame).unwrap();
}

fn raw_window_event(_app: &App, model: &mut Model, event: &nannou::winit::event::WindowEvent) {
    // Let egui handle things like keyboard and mouse input.
    model.egui.handle_raw_event(event);
}

