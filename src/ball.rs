use crate::player::*;
use crate::opponent::*;
use macroquad::prelude::*;
use ::rand::Rng;

pub struct Ball {
    radius: f32,

    position: Vec<f32>,

    velocity_x: f32,
    velocity_y: f32
}

// Circular ball
impl Ball {
    pub fn new() -> Self {
        Ball {
            radius: 10.0,

            position: vec![],

            velocity_x: 400.0,
            velocity_y: 300.00
        }
    }

    // Accessors
    pub fn get_position(&self) -> &Vec<f32> {
        &self.position
    }

    pub fn get_velocity_x(&self) -> f32 {
        return self.velocity_x;
    }

    pub fn get_velocity_y(&self) -> f32 {
        return self.velocity_y;
    }

    // Mutators
    pub fn set_velocity_x(&mut self, velocity: f32) {
        self.velocity_x = velocity;
    }

    pub fn set_velocity_y(&mut self, velocity:f32) {
        self.velocity_y = velocity;
    }

    // Starting position in center of screen
    pub fn set_start_position(&mut self) {
        self.position = vec![screen_width() / 2.0, screen_height() / 2.0];
        self.velocity_x = 0.0;
        self.velocity_y = 0.0;
    }

    pub fn serve_opponent(&mut self) {
        let mut rng = ::rand::thread_rng();
        self.velocity_y = rng.gen_range(-200.0..201.0);
        self.velocity_x = 400.0;
    }

    pub fn serve_player(&mut self) {
        let mut rng = ::rand::thread_rng();
        self.velocity_y = rng.gen_range(-200.0..201.0);
        self.velocity_x = -400.0;
    }

    /*
     * Bounces the ball off the top and bottom walls of the window
     * Multiplies Y velocity by -1 when met with either wall coordinate
     */
    fn check_wall_collision(&mut self) {
        // Top wall
        if self.position[1] <= self.radius {
            self.position[1] = self.radius;
            self.velocity_y *= -1.0;
        }

        // Bottom wall
        if self.position[1] >= screen_height() - self.radius {
            self.position[1] = screen_height() - self.radius;
            self.velocity_y *= -1.0;
        }
    }

    /*
     * Handling collision with player on a 100 point gradient.
     * Hitting the player in the center of the paddle will result in
     * diminishing y value velocity, while the outside of the paddle
     * increases y velocity
     */
    pub fn check_player_collision(&mut self, player: &Player) {
        let player_x = player.get_position()[0];
        let player_y = player.get_position()[1];
        let player_width = player.get_width();
        let player_height = player.get_height();

        let ball_x = self.position[0];
        let ball_y = self.position[1];

        // Checking collision and returning if collision is not true
        let is_colliding =
            ball_x <= player_x + player_width + self.radius &&
            ball_x >= player_x + player_width &&
            ball_y >= player_y &&
            ball_y <= player_y + player_height;

        if !is_colliding {
            return;
        }

        // Determining segment height when the player is split 100 times
        let segments = 100;
        let segment_height = player_height / segments as f32;

        // Determining relative ball y with player y (top of player)
        let relative_y = ball_y - player_y;

        // Grabbing the index of the player's gradient segment
        let mut index = (relative_y / segment_height) as usize;

        if index >= segments {
            index = segments - 1;
        }

        // Grabbing the multiplicative value assigned in player's gradient index
        let value = player.get_gradient()[index].1;

        // Reversing velociies based on "value"
        self.position[0] = player_x + player_width + self.radius;
        self.velocity_x *= -1.0;
        self.velocity_y = -300.0 * value;

    }

    /*
     * Handling collision with opponent on a 100 point gradient
     */
    pub fn check_opponent_collision(&mut self, opponent: &Opponent) {
        let opponent_x = opponent.get_position()[0];
        let opponent_y = opponent.get_position()[1];
        let opponent_height = opponent.get_height();

        let ball_x = self.position[0];
        let ball_y = self.position[1];

        // Checking collision and returning if collision is not true
        let is_colliding =
            ball_x >= opponent_x - self.radius &&
            ball_x <= opponent_x &&
            ball_y >= opponent_y &&
            ball_y <= opponent_y + opponent_height;

        if !is_colliding {
            return;
        }

        // Determining segment height when the player is split 100 times
        let segments = 100;
        let segment_height = opponent_height / segments as f32;

        // Determining relative ball y with player y (top of player)
        let relative_y = ball_y - opponent_y;

        // Grabbing the index of the player's gradient segment
        let mut index = (relative_y / segment_height) as usize;

        if index >= segments {
            index = segments - 1;
        }

        // Grabbing the multiplicative value assigned in player's gradient index
        let value = opponent.get_gradient()[index].1;

        // Reversing velociies based on "value"
        self.position[0] = opponent_x - self.radius;
        self.velocity_x *= -1.0;
        self.velocity_y = -300.0 * value;
    }

    // Draw function 
    pub fn draw_and_update(&mut self) {
        self.check_wall_collision();

        self.position[0] += self.velocity_x * get_frame_time();
        self.position[1] += self.velocity_y * get_frame_time();

        draw_circle(
            self.position[0],
            self.position[1],
            self.radius,
            RED
        )
    }
}