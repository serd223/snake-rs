use macroquad::prelude::*;
mod snake;
use snake::*;

const SCORE_PANEL_HEIGHT: f32 = 80.;
const SCORE_TEXT_MIN_REQUIRED_WIDTH: f32 = 475.;
const GAME_OVER_TIMER_DURATION: f64 = 1.5;
const DEFAULT_MAP_SIZE: (i32,i32) = (16, 9);
const SNAKE_ACCELERATION_MULTIPLIER: f32 = 0.00001;
const SNAKE_DEFAULT_SPEED: f32 = 0.13;

fn available_screen_size() -> (f32, f32) {
    let actual_width = screen_width();
    let actual_height = screen_height();
    if actual_height >= 500. && actual_width >= SCORE_TEXT_MIN_REQUIRED_WIDTH {
        return (actual_width, actual_height - SCORE_PANEL_HEIGHT);
    }

    (actual_width, actual_height)
}


fn print_ingame(text: Vec<String>) {
    for (i, string) in text.iter().enumerate() {
        draw_text(
            string,
            5., screen_height() - 5. - 20. * (i as f32), 20., WHITE
        );
    }
}

/// Draws a 2D grid that has x cells horizontally and y cells vertically.
fn draw_grid2d(x: i32, y: i32, color: Color) {
    let (scrw, scrh) = available_screen_size();
    let grid_cell_size: (f32, f32) = (scrw / (x as f32),scrh / (y as f32));
    for i in 0..x {
        for j in 0..y {
            draw_rectangle_lines(
                (i as f32) * grid_cell_size.0,
                (j as f32) * grid_cell_size.1, 
                grid_cell_size.0, grid_cell_size.1, 
                1., color
            )
        }
    }
}

fn draw_tile(x: f32, y: f32, game_map_size: (i32, i32), color: Color) {
    let (scrw, scrh) = available_screen_size();
    let grid_cell_size: (f32, f32) = (scrw / (game_map_size.0 as f32), scrh / (game_map_size.1 as f32));
    draw_rectangle(x * grid_cell_size.0, y * grid_cell_size.1, grid_cell_size.0, grid_cell_size.1, color);
}

fn spawn_food(snake_pos: &Vec<Vec2>, game_map_size: (i32, i32)) -> Vec2 {
    let mut found_pos = false;
    let mut res = Vec2::new(0., 0.);
    while !found_pos {
        res.x = macroquad::rand::gen_range(0, game_map_size.0) as f32;
        res.y = macroquad::rand::gen_range(0, game_map_size.1) as f32;
        found_pos = true;
        for spos in snake_pos {
            if spos.x == res.x && spos.y == res.y {
                found_pos = false;
                break;
            }
        }
    }
    res
}

#[macroquad::main("Not Snake")]
async fn main() {
    {
        use macroquad::rand::*;
        use std::time::*;
        srand(match UNIX_EPOCH.elapsed() {
            Ok(a) => a.as_secs(),
            Err(_) => 0
        });
    }

    let mut game_map_size = DEFAULT_MAP_SIZE;
    let mut game_over = false;
    let mut game_over_timer_start = 0.;

    let mut snake = Snake::new();
    snake.speed = SNAKE_DEFAULT_SPEED;
    let mut snake_dir: Direction = Direction::Stopped;
    let mut snake_dir_before_pause = Direction::Stopped;
    let mut god_mode = false;

    let mut food_position: Vec2 = spawn_food(&snake.pos, game_map_size);

    let mut help_text = true;
    loop {
        let game_time = get_time();
        let (screen_w, screen_h) = available_screen_size();

        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        if is_key_pressed(KeyCode::Up) { snake_dir = Direction::Up; }
        if is_key_pressed(KeyCode::Down) { snake_dir = Direction::Down }
        if is_key_pressed(KeyCode::Left) { snake_dir = Direction::Left }
        if is_key_pressed(KeyCode::Right) { snake_dir = Direction::Right }

        if is_key_pressed(KeyCode::R) { snake.reset() }
        if is_key_pressed(KeyCode::KpAdd) { snake.append() }
        if is_key_pressed(KeyCode::KpSubtract) { snake.pop() }
        if is_key_pressed(KeyCode::W) { game_map_size.1 += 1; }
        if is_key_pressed(KeyCode::S) { if game_map_size.1 > 1 { game_map_size.1 -= 1; } }
        if is_key_pressed(KeyCode::A) { if game_map_size.0 > 1 { game_map_size.0 -= 1; } }
        if is_key_pressed(KeyCode::D) { game_map_size.0 += 1; }
        if is_key_down(KeyCode::Q) { snake.speed -= 0.001; }
        if is_key_down(KeyCode::E) { snake.speed += 0.001; }
        if is_key_pressed(KeyCode::G) { god_mode = !god_mode; }
        if is_key_pressed(KeyCode::Tab) { help_text = !help_text }
        if is_key_pressed(KeyCode::P) {
            match snake_dir {
                Direction::Stopped => snake_dir = snake_dir_before_pause,
                _ => { snake_dir_before_pause = snake_dir; snake_dir = Direction::Stopped }
            }
        }
        
        if game_over && game_over_timer_start > 0. {
            if game_time - game_over_timer_start >= GAME_OVER_TIMER_DURATION {
                game_over = false;
                game_over_timer_start = 0.;
                game_map_size = DEFAULT_MAP_SIZE;
                snake.reset();
                snake.speed = SNAKE_DEFAULT_SPEED;
                food_position = spawn_food(&snake.pos, game_map_size);
            }
        } else if game_over {
            game_over_timer_start = get_time();
        }

        if game_over {
            snake_dir = Direction::Stopped;
        }
        snake.dir = snake_dir;
        snake.update(game_map_size);

        if !snake.alive {
            game_over = true;
        }
        
        if god_mode {
            game_over = false;
            snake.alive = true;
        }

        if snake.pos[0].x == food_position.x && snake.pos[0].y == food_position.y && !game_over{
            food_position = spawn_food(&snake.pos, game_map_size);
            snake.append();
            let snake_len = snake.pos.len();
            let snake_len_f32 = snake_len as f32;
            if snake_len > 1 {
                snake.speed += snake_len_f32 * (snake_len_f32 - 1.) * SNAKE_ACCELERATION_MULTIPLIER;
                if (snake_len-1) % 10 == 0 {
                    game_map_size.0 += 10;
                    game_map_size.1 += 6;
                }
            }
        }

        clear_background(DARKGRAY);
        draw_grid2d(game_map_size.0, game_map_size.1, DARKGREEN);
        draw_tile(food_position.x, food_position.y, game_map_size, RED);
        
        snake.draw(GREEN, Color::new(0.4, 0.7, 0.1, 1.), game_map_size, (screen_w, screen_h));

        if screen_h != screen_height() {
            draw_rectangle(0., screen_height() - SCORE_PANEL_HEIGHT, screen_w, SCORE_PANEL_HEIGHT, BLACK);
            if screen_w >= 475.{
                draw_text(
                    format!("Score: {}", snake.pos.len() - 1).as_str(), 
                    screen_w / 3., screen_height() - 20., SCORE_PANEL_HEIGHT, DARKGREEN
                );
            }
        }

        if help_text {
            print_ingame(vec![
                format!("Press [+/-] to make the snake longer/shorter. Current length is {}.", snake.pos.len()),
                format!("Press [Q/E] to make the snake faster/slower. Current speed is {}.", snake.speed),
                format!("Press [G] to toggle god mode. Currently it is {}.", (|| {if god_mode {"on"} else {"off"}})()),
                "Press [P] to make the snake stop".to_string(),
                "Press [R] to reset the snake.".to_string(),
                format!("Press [WASD] to change the map size. Currently it is {}x{}.", game_map_size.0, game_map_size.1),
                "Press [TAB] to toggle these help texts on/off.".to_string()
            ]);
        }
        next_frame().await;
    }
}
