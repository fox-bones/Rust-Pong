use macroquad::prelude::*;
use crate::ball::*;

pub enum Direction {
    None, 
    Up, 
    Down,
}

pub struct Player {
    height: f32,
    width: f32,
    speed: f32,

    direction: Direction,

    position: Vec<f32>,
    
    gradient: Vec<(f32, f32)>
}

impl Player {
    pub fn new() -> Self {
        Player {
            height: 100.0,
            width: 8.0,
            speed: 300.0,

            direction: Direction::None,

            position: vec![],

            gradient: vec![]
        }
    }

    // Accessors
    pub fn get_position(&self) -> &Vec<f32> {
        &self.position
    }

    pub fn get_width(&self) -> f32 {
        return self.width;
    }

    pub fn get_height(&self) -> f32 {
        return self.height;
    }

    pub fn get_gradient(&self) -> &Vec<(f32, f32)> {
        &self.gradient
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
     * Pasted to player paddle for dynamic velocity changes based on where the ball hits the paddle
     */
    pub fn create_gradient(&mut self) {
        let segments = 100;
        let segment_height = self.height / segments as f32;

        for i in 0..segments {
            let y = self.position[1] + i as f32 * segment_height;

            let value = if i < 50 {
                // 1.0 down to 0.01 across the first 50 segments
                let t = i as f32 / 49.0;
                1.0 + (0.01 - 1.0) * t
            } else {
                // -0.1 down to -1.0 across the last 50 segments
                let t = (i - 50) as f32 / 49.0;
                -0.1 + (-1.0 - -0.1) * t
            };

            self.gradient.push((y, value));
        }
    }

    // Setting start position 
    pub fn set_start_position(&mut self) {
        self.position = vec![10.0, (screen_height() / 2.0) - (self.height / 2.0)]
    }

    // Draw player function 
    fn draw(&self) {
        draw_rectangle(
            self.position[0],
            self.position[1],
            self.width,
            self.height,
            RED
        )
    }

    // Keyboard listener to assign enum Direction to player direction
    pub fn keyboard_movement(&mut self) {
        if is_key_down(KeyCode::W) && self.position[1] > 0.0 {
            self.direction = Direction::Up;
        }
        else if is_key_down(KeyCode::S) && (self.position[1] + self.height) < screen_height() {
            self.direction = Direction::Down;
        }
        else {
            self.direction = Direction::None;
        }
    }

    pub fn automatic_movement(&mut self, ball: &Ball) {
        if ball.get_velocity_x() < 0.0 && ball.get_position()[0] <= screen_width() / 2.0 {
            if (self.position[1] + (self.height / 2.0)) < ball.get_position()[1] &&
                self.position[1] + self.height < screen_height() {

                self.direction = Direction::Down;
            }
            else if (self.position[1] + (self.height / 2.0)) > ball.get_position()[1] &&
                self.position[1] > 0.0 {

                self.direction = Direction::Up;
            }
            else {
                self.direction = Direction::None;
            }
        }
        else {
            self.direction = Direction::None;
        }
    }

    // Updating player position based on direction assigned in listener
    pub fn update_position(&mut self) {
        match self.direction {
            Direction::Up => {
                self.position[1] -= self.speed * get_frame_time();
            }
            Direction::Down => {
                self.position[1] += self.speed * get_frame_time();
            }
            Direction::None => {
                self.position= self.position.clone();
            }
            _ => {}
        }
        // Implementing draw() in update_position() to keep Driver file slim
        self.draw();
        self.create_gradient();
    }
}