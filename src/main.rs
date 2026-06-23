pub mod ball;
pub mod opponent;
pub mod player;
pub mod ui;

use crate::ball::*;
use crate::opponent::*;
use crate::player::*;
use ::rand::Rng;
use macroquad::audio::{Sound, load_sound, play_sound_once};
use macroquad::prelude::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "Rusty Pong".to_owned(),
        window_width: 640,
        window_height: 480,
        window_resizable: false,
        ..Default::default()
    }
}

// Game states
enum GameState {
    StartMenu,
    Playing,
    Paused,
    End,
}

// Reseting playing positions on game start
fn start_playing(player: &mut Player, opponent: &mut Opponent, ball: &mut Ball) {
    let mut rng = ::rand::thread_rng();

    player.set_start_position();
    opponent.set_start_position();

    if rng.gen_range(0..2) == 0 {
        ball.serve_player();
    } else {
        ball.serve_opponent();
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut game_state = GameState::StartMenu;

    // Sounds used for various components
    let player_paddle_hit_sound: Sound = load_sound("resources/paddle_hit_2.wav").await.unwrap();
    let opponent_paddle_hit_sound: Sound = load_sound("resources/paddle_hit.wav").await.unwrap();

    let mut _player = Player::new();
    let mut _opponent = Opponent::new();
    let mut _ball = Ball::new();

    let mut player_score: i32 = 0;
    let mut opponent_score: i32 = 0;

    const WINNING_SCORE: i32 = 11;
    const POINT_FLASH_SECONDS: f64 = 1.0;

    //Used for storing the current x, y velocities when the game is in pause
    let mut ball_paused_velocity_y = _ball.get_velocity_y();
    let mut ball_paused_velocity_x = _ball.get_velocity_x();

    // Used to stall serves afer either side scores
    let mut point_flash_ends_at = 0.0;
    let mut point_scored_by = "player";
    let mut serve_to_player = false;
    let mut waiting_to_serve = false;

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
                    waiting_to_serve = false;
                    start_playing(&mut _player, &mut _opponent, &mut _ball);

                    game_state = GameState::Playing;
                }
            }
            // Read keyboard inputs and check for collisions on frame udpates. Call GameState::Reset if the ball
            // overcomes player or opponent paddles, add 1 to respective score, and make sure to reset the brief timer.
            GameState::Playing => {
                ui::draw_divider();
                //_player.automatic_movement(&_ball);
                _player.keyboard_movement();
                _player.update_position();

                _opponent.automatic_movement(&_ball);
                _opponent.update_position();

                // Serve timer
                if waiting_to_serve && get_time() >= point_flash_ends_at {
                    if serve_to_player {
                        _ball.serve_player();
                    } else {
                        _ball.serve_opponent();
                    }

                    waiting_to_serve = false;
                }

                if !waiting_to_serve {
                    _ball.check_player_collision(&_player, &player_paddle_hit_sound);
                    _ball.check_opponent_collision(&_opponent, &opponent_paddle_hit_sound);
                }

                _ball.draw_and_update();

                ui::draw_score_overlay(player_score, opponent_score);

                // Flashing point if waiting for another serve
                if waiting_to_serve {
                    ui::flash_point_screen(point_scored_by);
                }

                // Pause the game is the player presses escape
                if is_key_pressed(KeyCode::Escape) {
                    ball_paused_velocity_x = _ball.get_velocity_x();
                    ball_paused_velocity_y = _ball.get_velocity_y();
                    game_state = GameState::Paused;
                }

                // Small wait time to flash +1 over the user who scored before the ball is served again
                if !waiting_to_serve && _ball.get_position()[0] < 0.0 {
                    opponent_score += 1;

                    if opponent_score >= WINNING_SCORE {
                        game_state = GameState::End;
                    } else {
                        point_scored_by = "opponent";
                        serve_to_player = true;
                        point_flash_ends_at = get_time() + POINT_FLASH_SECONDS;
                        waiting_to_serve = true;
                    }
                } else if !waiting_to_serve && _ball.get_position()[0] > screen_width() {
                    player_score += 1;

                    if player_score >= WINNING_SCORE {
                        game_state = GameState::End;
                    } else {
                        point_scored_by = "player";
                        serve_to_player = false;
                        point_flash_ends_at = get_time() + POINT_FLASH_SECONDS;
                        waiting_to_serve = true;
                    }
                }
            }
            // Set player and opponent directions to none and continually update to simulate pausing.
            GameState::Paused => {
                ui::draw_divider();
                _player.set_direction("None");
                _player.update_position();
                _opponent.set_direction("None");
                _opponent.update_position();

                _ball.set_velocity_y(0.0);
                _ball.set_velocity_x(0.0);

                _ball.draw_and_update();

                ui::draw_score_overlay(player_score, opponent_score);
                ui::draw_pause_menu();

                // Resume game with frozen ball velocities
                if is_key_pressed(KeyCode::Escape) {
                    _ball.set_velocity_y(ball_paused_velocity_y);
                    _ball.set_velocity_x(ball_paused_velocity_x);
                    game_state = GameState::Playing;
                }
                // End game
                if is_key_pressed(KeyCode::Q) {
                    game_state = GameState::StartMenu;
                }
            }
            GameState::End => {
                // Display winner
                if player_score >= WINNING_SCORE {
                    ui::draw_end_menu("player");
                } else {
                    ui::draw_end_menu("opponent");
                }

                // Restart game
                if is_key_pressed(KeyCode::Enter) {
                    player_score = 0;
                    opponent_score = 0;
                    waiting_to_serve = false;
                    start_playing(&mut _player, &mut _opponent, &mut _ball);
                    game_state = GameState::Playing;
                }

                // Return to main menu
                if is_key_pressed(KeyCode::Q) {
                    player_score = 0;
                    opponent_score = 0;

                    game_state = GameState::StartMenu;
                }
            }
            _ => {}
        }

        next_frame().await;
    }
}
