use macroquad::prelude::*;

// Center text helper
fn draw_centered_text(text: &str, y:f32, size:f32, color: Color) {
    let dimensions = measure_text(text, None, size as u16, 1.0);

    draw_text(
        text, 
        screen_width() / 2.0 - dimensions.width / 2.0, 
        y, 
        size, 
        color
    );
}

pub fn draw_start_menu() {
    draw_centered_text("Pong", 250.0, 48.0, WHITE);
    draw_centered_text("Press Enter to Start", 280.0, 28.0, WHITE);
}

pub fn draw_pause_menu() {
    draw_rectangle(
        0.0,
        0.0,
        screen_width(),
        screen_height(),
        Color::new(0.0, 0.0, 0.0, 0.6),
    );

    draw_centered_text("Paused", 210.0, 48.0, YELLOW);
    draw_centered_text("Esc - Resume", 270.0, 28.0, WHITE);
    draw_centered_text("Q - Main Menu", 310.0, 28.0, WHITE);
}

pub fn draw_reset_menu(last_to_score: &str) {
    draw_rectangle(
        0.0,
        0.0,
        screen_width(),
        screen_height(),
        Color::new(0.0, 0.0, 0.0, 0.6),
    );

    draw_centered_text("Ready?", 270.0, 48.0, WHITE);
    if last_to_score == "player" {
        draw_centered_text("Serve >", 300.0, 28.0, WHITE);
    }
    else if last_to_score == "opponent" {
        draw_centered_text("< Serve", 300.0, 28.0, WHITE);
    }
}

pub fn draw_score_overlay(player_score: i32, opponent_score: i32) {
    let text = format!("{}     {}", player_score, opponent_score);
    let size = 42.0;
    let dimensions = measure_text(&text, None, size as u16, 1.0);

    // Transparent background panel
    draw_rectangle(
        screen_width() / 2.0 - dimensions.width / 2.0 - 24.0,
        20.0,
        dimensions.width + 48.0,
        56.0,
        Color::new(0.0, 0.0, 0.0, 0.0),
    );

    draw_text(
        &text,
        screen_width() / 2.0 - dimensions.width / 2.0,
        62.0,
        size,
        Color::new(1.0, 1.0, 1.0, 0.55),
    );
}

pub fn draw_end_menu(winner: &str) {
    draw_rectangle(
        0.0,
        0.0,
        screen_width(),
        screen_height(),
        Color::new(0.0, 0.0, 0.0, 0.6),
    );

    if winner == "player" {
        draw_centered_text("Player Won", 240.0, 48.0, YELLOW);
        draw_centered_text("Enter - Restart", 270.0, 28.0, WHITE);
        draw_centered_text("Q - Main Menu", 300.0, 28.0, WHITE);
    }
    else {
        draw_centered_text("Opponent Won", 240.0, 48.0, YELLOW);
        draw_centered_text("Enter - Restart", 270.0, 28.0, WHITE);
        draw_centered_text("Q - Main Menu", 300.0, 28.0, WHITE);
    }
}