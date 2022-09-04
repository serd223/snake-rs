use macroquad::prelude::*;

#[derive(Copy, Clone)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    Stopped
}

pub struct Snake {
    pub pos: Vec<Vec2>,
    pub dir: Direction,
    pub speed: f32,
    pub alive: bool,
    head_raw: Vec2,
    head_last_frame: Vec2,
}

impl Snake {
    pub fn append(&mut self) {
        let tail = self.pos[self.pos.len()-1];
        self.pos.push(Vec2::new(tail.x, tail.y));
    }

    pub fn pop(&mut self) {
        if self.pos.len() > 1 {
            self.pos.pop();
        }
    }

    /// Called every frame to update the snake's data.
    pub fn update(&mut self, game_map_size: (i32, i32)) {
        match self.dir{
            Direction::Up => self.head_raw.y -= self.speed,
            Direction::Down => self.head_raw.y += self.speed,
            Direction::Left => self.head_raw.x -= self.speed,
            Direction::Right => self.head_raw.x += self.speed,
            _ => ()
        }
        let snake_rounded_pos: Vec2 = Vec2::new(self.head_raw.x.round(), self.head_raw.y.round());
        let mut moving = true;
        if self.head_last_frame.x == snake_rounded_pos.x &&  self.head_last_frame.y == snake_rounded_pos.y {
            moving = false;
        }
        self.head_last_frame = snake_rounded_pos;

        let mut i = self.pos.len() - 1;
        while i > 0 {
            if moving {
                self.pos[i].x = self.pos[i-1].x;
                self.pos[i].y = self.pos[i-1].y;
            }
            if self.pos[i].x == snake_rounded_pos.x && self.pos[i].y == snake_rounded_pos.y && self.pos.len() > 2 {
                self.alive = false;
            }
            i -= 1;
        }
        if snake_rounded_pos.x >= (game_map_size.0 as f32) || snake_rounded_pos.x < 0. ||
        snake_rounded_pos.y >= (game_map_size.1 as f32) || snake_rounded_pos.y < 0. {
            self.alive = false;
        }
        self.pos[0] = snake_rounded_pos;
    }

    pub fn draw(&self, head_color: Color, tail_color: Color, game_map_size: (i32, i32), screen_size: (f32, f32)) {
        let grid_cell_size: (f32, f32) = (screen_size.0 / (game_map_size.0 as f32), screen_size.1 / (game_map_size.1 as f32));
        let mut current_color = head_color;
        for pos in &self.pos {
            draw_rectangle(
                pos.x * grid_cell_size.0,
                pos.y * grid_cell_size.1, 
                grid_cell_size.0, grid_cell_size.1,
                current_color
            );
            current_color = tail_color;
        }
    }

    pub fn reset(&mut self) {
        self.alive = true;
        self.head_raw = Vec2::new(0.,0.);
        self.head_last_frame = Vec2::new(0.,0.);
        self.pos = vec![Vec2::new(0., 0.)];
        self.dir = Direction::Stopped;
        self.speed = 0.13;
    }

    pub fn new() -> Self {
        Self {
            pos: vec![Vec2::new(0., 0.)],
            dir: Direction::Stopped,
            speed: 0.13,
            alive: true,
            head_raw: Vec2::new(0.,0.),
            head_last_frame: Vec2::new(0.,0.)
        }
    }
}