mod card;
mod hands;
mod banker;
mod pokerbot;
mod game_manager;
mod basicpokerbot;
// mod tui_pokerbot;

use std::f64;

use eframe;
use egui::{self, vec2, Color32, Vec2b};
use egui_plot::*;

use card::{Card, Suit, Value};
use game_manager::Game;
use pokerbot::PokerBot;

use basicpokerbot::BasicPokerBot;
// use tui_pokerbot::TUIPokerBot;

// fn main() -> eframe::Result {
    // let options = eframe::NativeOptions {
    //     viewport: egui::ViewportBuilder::default().with_inner_size([1100.0, 600.0]).with_position([60.0, 60.0]),
    //     ..Default::default()
    // };

    // eframe::run_native(
    //     "Plot",
    //     options,
    //     Box::new(|_cc| Ok(Box::<PokerPlot>::default())),
    // )
// }

fn main() {
    let mut bots: Vec<Box<dyn PokerBot>> = Vec::new();
    bots.push(Box::new(BasicPokerBot));
    bots.push(Box::new(BasicPokerBot));
    bots.push(Box::new(BasicPokerBot));
    bots.push(Box::new(BasicPokerBot));
    // bots.push(Box::new(TUIPokerBot));
    // bots.push(Box::new(TUIPokerBot));
    // bots.push(Box::new(TUIPokerBot));
    // bots.push(Box::new(TUIPokerBot));
    
    let numplayers = bots.len();

    let mut game = Game::new(bots, 4000, "out.txt");

    let mut players : Vec<Vec<_>> = (0..numplayers).map(|_| vec![]).collect();
    
    (0..10).for_each(|i| {
        println!("Round {}", i);
        game.play_round();
        game.print_values();

        for x in 0..numplayers {
            players[x].push([i as f64, game.get_player_money(x) as f64]);
        }
    });
}

struct PokerPlot {
    players: Vec<Vec<[f64; 2]>>
}


impl Default for PokerPlot {
    fn default() -> Self {
        let mut bots: Vec<Box<dyn PokerBot>> = Vec::new();
        bots.push(Box::new(BasicPokerBot));
        bots.push(Box::new(BasicPokerBot));
        bots.push(Box::new(BasicPokerBot));
        bots.push(Box::new(BasicPokerBot));
        // bots.push(Box::new(TUIPokerBot));
        // bots.push(Box::new(TUIPokerBot));
        // bots.push(Box::new(TUIPokerBot));
        // bots.push(Box::new(TUIPokerBot));
        
        let numplayers = bots.len();

        let mut game = Game::new(bots, 4000, "out.txt");

        let mut players : Vec<Vec<_>> = (0..numplayers).map(|_| vec![]).collect();
        
        (0..2).for_each(|i| {
            println!("Round {}", i);
            game.play_round();
            game.print_values();

            for x in 0..numplayers {
                players[x].push([i as f64, game.get_player_money(x) as f64]);
            }
        });


        Self { 
            players
        }
    }
}

impl eframe::App for PokerPlot {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            egui_plot::Plot::new("plot")
                .allow_drag(false)
                .allow_zoom(false)
                .allow_scroll(false)
                .set_margin_fraction(vec2(0.0, 0.4))
                .x_axis_label("Rounds")
                .y_axis_label("Money")
                .legend(Legend::default())
                .show(ui, |plot_ui| {

                    plot_ui.set_plot_bounds(PlotBounds::from_min_max([0.0, 2500.0], [50.0, 5500.0]));
                    plot_ui.set_auto_bounds(Vec2b::new(true, false));
                    // plot_ui.points(
                    //     Points::new(self.player_one.clone())
                    //     .radius(4.0)
                    // );


                    let colors = [
                        Color32::RED,
                        Color32::BLUE,
                        Color32::GREEN,
                        Color32::ORANGE,
                        Color32::LIGHT_BLUE
                    ];

                    for (i, player) in self.players.iter().enumerate() {
                        let p = i+1;      
                        plot_ui.line(
                            Line::new(player.clone()).color(colors[i]).name(format!("Player {p}"))
                        );
                    }
                });
        });
    }
}


