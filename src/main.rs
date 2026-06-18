mod player;
mod opponent;
mod ball;
mod ui;

use macroquad::prelude::*;
use crate::player::*;
use crate::opponent::*;
use crate::ball::*;
use crate::ui::*;
use ::rand::Rng;

fn window_conf() -> Conf {
    Conf {
        window_title: "Rusty Pong".to_owned(),
        window_width: 640,
        window_height: 480,
        window_resizable: false, 
        ..Default::default()
    }
}

enum GameState {
    StartMenu,
    Playing,
    Reset,
    Paused,
    End,
}

fn draw_divider() {
    draw_rectangle(
        screen_width() / 2.0,
        0.0,
        1.0,
        screen_height(),
        GRAY
    )
}
#[macroquad::main(window_conf)]
async fn main() {
    let mut game_state = GameState::StartMenu;

    let mut rng;

    let mut _player = Player::new();
    let mut _opponent = Opponent::new();
    let mut _ball = Ball::new();

    let mut player_score: i32 = 0;
    let mut opponent_score: i32 = 0;
    let mut last_to_score: &str = "";

    const ROUND_DELAY: f64 = 1.0;
    const WINNING_SCORE: i32 = 11;
    let mut playing_starts_at = 0.0;

    let mut ball_paused_velocity_y = _ball.get_velocity_y();
    let mut ball_paused_velocity_x = _ball.get_velocity_x();

    loop {
        clear_background(BLACK);

        match game_state {
            GameState::StartMenu => {
                ui::draw_start_menu();

                // When Enter is pressed set positions, coin toss who the first serve goes to, and set a brief delay 
                // for the first time that GameState::Reset is called.
                if is_key_pressed(KeyCode::Enter) {
                    player_score = 0;
                    opponent_score = 0;

                    rng = ::rand::thread_rng();

                    if rng.gen_range(0..2) == 0 {
                        last_to_score = "player";
                    }
                    else {
                        last_to_score = "opponent";
                    }

                    playing_starts_at = get_time() + ROUND_DELAY;
                    game_state = GameState::Reset;
                }
            }
            // Read keyboard inputs and check for collisions on frame udpates. Call GameState::Reset if the ball 
            // overcomes player or opponent paddles, add 1 to respective score, and make sure to reset the brief timer.
            GameState::Playing => {
                draw_divider();
                _player.keyboard_movement();
                _player.update_position();

                _opponent.automatic_movement(&_ball);
                _opponent.update_position();

                _ball.check_player_collision(&_player);
                _ball.check_opponent_collision(&_opponent);
                _ball.draw_and_update();

                ui::draw_score_overlay(player_score, opponent_score);

                if is_key_pressed(KeyCode::Escape) {
                    ball_paused_velocity_x = _ball.get_velocity_x();
                    ball_paused_velocity_y = _ball.get_velocity_y();
                    game_state = GameState::Paused;
                }

                if _ball.get_position()[0] < 0.0 {
                    opponent_score += 1;
                    if opponent_score >= WINNING_SCORE {
                        game_state = GameState::End;
                    } else {
                        last_to_score = "opponent";
                        playing_starts_at = get_time() + ROUND_DELAY;
                        game_state = GameState::Reset;
                    }
                }
                else if _ball.get_position()[0] > screen_width() {
                    player_score += 1;
                    if player_score >= WINNING_SCORE {
                        game_state = GameState::End;
                    } else {
                        last_to_score = "player";
                        playing_starts_at = get_time() + ROUND_DELAY;
                        game_state = GameState::Reset;
                    }
                }
            }
            // Maintain positions during 1 - 2 second delay. Serve to the player who did NOT score in the last round.
            GameState::Reset => {
                draw_divider();
                _player.set_start_position();
                _player.create_gradient();
                _player.update_position();

                _opponent.set_start_position();
                _opponent.create_gradient();
                _opponent.update_position();

                _ball.set_start_position();
                _ball.draw_and_update();

                ui::draw_reset_menu(last_to_score);
                ui::draw_score_overlay(player_score, opponent_score);

                if get_time() >= playing_starts_at {
                    if last_to_score == "player" {
                        _ball.serve_opponent();
                    }
                    else if last_to_score == "opponent" {
                        _ball.serve_player();
                    }

                    game_state = GameState::Playing;
                }
            }
            // Set player and opponent directions to none and continually update to simulate pausing.
            GameState::Paused => {
                draw_divider();
                _player.set_direction("None");
                _player.update_position();
                _opponent.set_direction("None");
                _opponent.update_position();

                _ball.set_velocity_y(0.0);
                _ball.set_velocity_x(0.0);

                _ball.draw_and_update();

                ui::draw_score_overlay(player_score, opponent_score);
                ui::draw_pause_menu();

                if is_key_pressed(KeyCode::Escape) {
                    _ball.set_velocity_y(ball_paused_velocity_y);
                    _ball.set_velocity_x(ball_paused_velocity_x);
                    game_state = GameState::Playing;
                }
                if is_key_pressed(KeyCode::Q) {
                    game_state = GameState::StartMenu;
                }
            }
            GameState::End => {
                if player_score >= WINNING_SCORE {
                    ui::draw_end_menu("player");
                }
                else {
                    ui::draw_end_menu("opponent");
                }

                // Restart game 
                if is_key_pressed(KeyCode::Enter) {
                    player_score = 0;
                    opponent_score = 0;

                    rng = ::rand::thread_rng();

                    if rng.gen_range(0..2) == 0 {
                        last_to_score = "player";
                    }
                    else {
                        last_to_score = "opponent";
                    }

                    playing_starts_at = get_time() + ROUND_DELAY;
                    game_state = GameState::Reset;
                }

                // Return to main menu
                if is_key_pressed(KeyCode::Q) {
                    player_score = 0;
                    opponent_score = 0;

                    game_state = GameState::StartMenu;
                }
            }
        }

        next_frame().await;
    }
}
