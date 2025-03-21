use crate::algorithms::*;
pub use list::*;
use egui::{Window, ComboBox};
use nannou::prelude::*;
use nannou_egui::{self, egui, Egui};
use nannou_egui::egui::Slider;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use crate::player::SortPlayer;

mod algorithms;
mod list;
mod renderers;
mod player;

fn main() {
    nannou::app(model).fullscreen().update(update).run();
}

struct Model {
    _window: window::Id,
    player: SortPlayer,
    egui: Egui,
    selected: SortMethod,
    length_log2: usize,
    playback_rate: usize,
    renderer: RenderMethod
}

fn model(app: &App) -> Model {
    let window_id = app
        .new_window()
        .view(view)
        .raw_event(raw_window_event)
        .build()
        .unwrap();
    let window = app.window(window_id).unwrap();

    let egui = Egui::from_window(&window);

    Model {
        _window: window_id,
        player: SortPlayer::new(2_usize.pow(8), quicksort::sort),
        egui,
        selected: SortMethod::Quick,
        length_log2: 8,
        playback_rate: 200,
        renderer: RenderMethod::Classic
    }
}

fn update(_app: &App, model: &mut Model, update: Update) {
    let moves = (model.playback_rate as f64 * update.since_last.as_secs_f64()).ceil() as usize;
    model.player.play(moves);
    gui(_app, model, update);
}

fn gui(_app: &App, model: &mut Model, update: Update) {
    let egui = &mut model.egui;
    egui.set_elapsed_time(update.since_start);
    egui.begin_frame();

    Window::new("Settings").show(egui.ctx(), |ui| {
        ComboBox::from_label("Renderer")
            .selected_text(format!("{:?}", model.renderer))
            .show_ui(ui, |ui| {
                for option in RenderMethod::iter() {
                    ui.selectable_value(&mut model.renderer, option, format!("{:?}", option));
                }
            });
        ComboBox::from_label("Algorithm")
            .selected_text(format!("{:?}", model.selected))
            .show_ui(ui, |ui| {
                for option in SortMethod::iter() {
                    let response =
                        ui.selectable_value(&mut model.selected, option, format!("{:?}", option));
                    if response.changed() {
                        model.player = SortPlayer::new(
                            2_usize.pow(model.length_log2 as u32),
                            model.selected.func(),
                        );
                    }
                }
            });

        ui.add(
            Slider::new(&mut model.playback_rate, 100..=10000)
                .text("Playback rate (ops/secs)"),
        );
        let res = ui.add(Slider::new(&mut model.length_log2, 1..=16).text("Length (log2)"));
        if res.changed() {
            model.player =
                SortPlayer::new(2_usize.pow(model.length_log2 as u32), model.selected.func());

        }
        if ui.button("Replay").clicked() {
            model.player.reset_play();
        }
    });
}

#[derive(Debug, PartialEq, Copy, Clone, EnumIter)]
enum SortMethod {
    Quick,
    Merge,
    Bubble,
    Selection,
    Radix,
    Bucket
}

impl SortMethod {
    fn func(&self) -> fn(&mut List) {
        match self {
            SortMethod::Quick => quicksort::sort,
            SortMethod::Merge => mergesort::sort,
            SortMethod::Bubble => bubble::sort,
            SortMethod::Selection => selection::sort,
            SortMethod::Radix => radix::sort,
            SortMethod::Bucket => bucket::sort,
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone, EnumIter)]
enum RenderMethod {
    Classic,
    DisparityDots,
}

impl RenderMethod {
    fn func(&self) -> fn(player: &SortPlayer, draw: &Draw) {
        match self {
            RenderMethod::Classic => {renderers::classic::draw_state}
            RenderMethod::DisparityDots => {todo!()}
        }
    }
    fn draw(&self, player: &SortPlayer, draw: &Draw) {
        self.func()(player, draw);
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app
        .draw()
        .scale_x(app.window_rect().x.len())
        .scale_y(app.window_rect().y.len())
        .xy(Vec2::splat(-0.5));

    (0.0, 1.0, 0.0, 1.0);
    draw.background().color(BLACK);
    model.renderer.draw(&model.player, &draw);
    draw.to_frame(app, &frame).unwrap();
    model.egui.draw_to_frame(&frame).unwrap();
}

fn raw_window_event(_app: &App, model: &mut Model, event: &nannou::winit::event::WindowEvent) {
    // Let egui handle things like keyboard and mouse input.
    model.egui.handle_raw_event(event);
}
