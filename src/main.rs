mod card;
mod hands;
mod banker;
mod pokerbot;
mod game_manager;
mod basicpokerbot;
mod tui_pokerbot;

use eframe;
use egui::{self, vec2};
use egui_plot::*;

use card::{Card, Suit, Value};
use game_manager::Game;
use pokerbot::PokerBot;

use basicpokerbot::BasicPokerBot;
use tui_pokerbot::TUIPokerBot;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Plot",
        options,
        Box::new(|_cc| Ok(Box::<PokerPlot>::default())),
    )
}

struct PokerPlot {

}


impl Default for PokerPlot {
    fn default() -> Self {
        Self {  }
    }
}

impl eframe::App for PokerPlot {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            egui_plot::Plot::new("plot")
                .legend(Legend::default())  
                .auto_bounds(egui::Vec2b { x: true, y: true })
                .show(ui, |plot_ui| {
                    let pts = vec![
                        [0.0, 110.0],
                        [1.0, 452.0],
                        [2.0, 364.0],
                        [3.0, 575.0],
                    ];

                    plot_ui.points(
                        Points::new(pts.clone())
                        .radius(4.0)
                    );

                    plot_ui.line(
                        Line::new(pts.clone())
                    );
                });
        });
    }
}



fn run() {

    let mut bots: Vec<Box<dyn PokerBot>> = Vec::new();

    bots.push(Box::new(BasicPokerBot));
    bots.push(Box::new(BasicPokerBot));
    bots.push(Box::new(BasicPokerBot));
    bots.push(Box::new(BasicPokerBot));

    let mut game = Game::new(bots);

    (0..50).for_each(|i| {
        println!("Round {}", i);
        game.play_round();
        game.print_values();
    });

    // println!("COOKIE");
}



