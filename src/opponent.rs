use crate::ball::*;
use macroquad::prelude::*;
use ::rand::Rng;

pub enum Direction {
    None,
    Up,
    Down,
}

pub struct Opponent {
    height: f32,
    width: f32,
    speed: f32,
    direction: Direction,
    position: Vec<f32>,
    gradient: Vec<(f32, f32)>,
    target_y: f32,
    reaction_delay: f32,
    difficulty: f32,
}

impl Opponent {
    pub fn new() -> Self {
        let mut opponent = Opponent {
            height: 100.0,
            width: 8.0,
            speed: 300.0,
            direction: Direction::None,
            position: vec![],
            gradient: vec![],
            target_y: 0.0,
            reaction_delay: 0.08,
            difficulty: 0.85,
        };
        opponent.create_gradient();  // build once
        opponent
    }

    // Accessors
    pub fn get_position(&self) -> &Vec<f32> {
        &self.position
    }

    pub fn get_height(&self) -> f32 {
        return self.height;
    }

    pub fn get_gradient(&self) -> &Vec<(f32, f32)> {
        &self.gradient
    }

    pub fn get_width(&self) -> f32 {
        return self.width;
    }

    // Mutators
    pub fn set_direction(&mut self, direction: &str) {
        match direction {
            "None" => {
                self.direction = Direction::None;
            }
            "Up" => {
                self.direction = Direction::Up;
            }
            "Down" => {
                self.direction = Direction::Down;
            }
            &_ => {}
        }
    }

    /*
     * Velocity Gradient function - Creates a spread from 1.0 - 0.1, then -.01 - -1.0
     * Pasted to opponent paddle for dynamic velocity changes based on where the ball hits the paddle
     */
    pub fn create_gradient(&mut self) {
        self.gradient.clear();  // <-- clear stale data first

        let segments = 100;
        let segment_height = self.height / segments as f32;

        for i in 0..segments {
            // Relative offset from paddle top, not absolute Y
            let y_offset = i as f32 * segment_height;

            let value = if i < 50 {
                let t = i as f32 / 49.0;
                1.0 + (0.01 - 1.0) * t
            } else {
                let t = (i - 50) as f32 / 49.0;
                -0.1 + (-1.0 - -0.1) * t
            };

            self.gradient.push((y_offset, value));
        }
    }

    // Starting position at right side, middle of screen
    pub fn set_start_position(&mut self) {
        self.position = vec![
            screen_width() - (self.width + 10.0),
            (screen_height() / 2.0) - (self.height / 2.0),
        ];
        self.target_y = self.position[1];
    }

    // Keyboard listener to assign enum Direction to player direction
    pub fn keyboard_movement(&mut self) {
        if is_key_down(KeyCode::Up) && self.position[1] > 0.0 {
            self.direction = Direction::Up;
        } else if is_key_down(KeyCode::Down) && (self.position[1] + self.height) < screen_height() {
            self.direction = Direction::Down;
        } else {
            self.direction = Direction::None;
        }
    }

    pub fn automatic_movement(&mut self, ball: &Ball) {
        let ball_pos = ball.get_position();
        let ball_vel_x = ball.get_velocity_x();

        // Only track ball when it's heading toward us and in our half
        if ball_vel_x > 0.0 && ball_pos[0] >= screen_width() / 2.0 {
            // Target the ball center, offset slightly by difficulty
            // (harder = aims closer to true center; easier = aims at edge)
            let paddle_center = self.position[1] + self.height / 2.0;
            let ball_center_y = ball_pos[1];

            // Dead zone: don't react if already close enough (kills jitter)
            let dead_zone = self.height * (1.0 - self.difficulty) * 0.3 + 4.0;
            if (paddle_center - ball_center_y).abs() > dead_zone {
                // Lerp target_y toward ball — smooth, not instant
                let lerp_speed = self.difficulty * 8.0; // higher = snappier
                self.target_y = self.target_y
                    + (ball_center_y - self.height / 2.0 - self.target_y)
                    * (lerp_speed * get_frame_time()).min(1.0);
            }
            // else: hold current target — no micro-corrections
        } else {
            // Ball moving away: drift back toward center lazily
            let center = screen_height() / 2.0 - self.height / 2.0;
            self.target_y = self.target_y
                + (center - self.target_y)
                * (2.0 * get_frame_time()).min(1.0);
        }

        // Convert target into direction enum for update_position()
        let paddle_top = self.position[1];
        let diff = self.target_y - paddle_top;

        if diff > 2.0 && paddle_top + self.height < screen_height() {
            self.direction = Direction::Down;
        } else if diff < -2.0 && paddle_top > 0.0 {
            self.direction = Direction::Up;
        } else {
            self.direction = Direction::None;
        }
    }

    // Draw function
    fn draw(&self) {
        draw_rectangle(
            self.position[0],
            self.position[1],
            self.width,
            self.height,
            RED,
        )
    }

    pub fn update_position(&mut self) {
        match self.direction {
            Direction::Up => {
                self.position[1] -= self.speed * get_frame_time();
            }
            Direction::Down => {
                self.position[1] += self.speed * get_frame_time();
            }
            Direction::None => {
                self.position = self.position.clone();
            }
            _ => {}
        }
        self.position[1] = self.position[1].clamp(0.0, screen_height() - self.height);
        // Implementing draw() in update_position() to keep Driver file slim
        self.draw();
    }
}
