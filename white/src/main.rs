// TODO:
// redo input system    (DONE)
// add menu, end and options screens
// improve ui and graphics in general
//        (new font, better colors, etc.) (DONE)

use std::{io::{Read, Write}, path::Path, fs};

use quicksilver::{
    geom::{Vector, Rectangle},
    graphics::{Color, Graphics, VectorFont},
    Input, Window, Result, Settings, Timer, run,
};

use rand::Rng;
use chrono::Datelike;

const WINDOW_SIZE_PX: f32 = 500.0;
const GRID_SIZE: usize = 5;
const GRID_CELL_SIZE_PX: f32 = WINDOW_SIZE_PX / GRID_SIZE as f32;
const FPS: f64 = 30.0;
#[derive(Clone)]
struct save_data_unit {
    score: i32,
    date: String,
}
enum GameState {
    Menu,
    Game,
    End,
    Options,
}

enum MenuOptions {
    TargetTime,
    TargetScore,
    Reset,
    Back,
}

fn main() {
    run(
        Settings {
            title: "2D aim",
            size: Vector::new(WINDOW_SIZE_PX + GRID_SIZE as f32 - 4.0, WINDOW_SIZE_PX + GRID_SIZE as f32 - 4.0),
            fullscreen: false,
            vsync: true,
            resizable: false,
            use_static_dir: true,
            ..Settings::default()
        },
        app,
    );
}

fn add_rand_to_grid(grid: &mut [[bool; GRID_SIZE]; GRID_SIZE]) {
    let mut rng = rand::thread_rng();
    let mut x = rng.gen_range(0..GRID_SIZE);
    let mut y = rng.gen_range(0..GRID_SIZE);

    while grid[x][y] {
        x = rng.gen_range(0..GRID_SIZE);
        y = rng.gen_range(0..GRID_SIZE);
    }
        
    grid[x][y] = true;
}
fn reset_grid(grid: &mut [[bool; GRID_SIZE]; GRID_SIZE]) {
    for row in grid.iter_mut() {
        for cell in row.iter_mut() {
            *cell = false;
        }
    }
    for _ in 0..3 {
        add_rand_to_grid(grid);
    }
}

// load five best scores (by score) from file "scores.best" into vector
fn load_saved_data() -> Vec<save_data_unit> {
    match std::fs::File::open("scores.best") {
        Ok(mut file) => {
            let mut contents = String::new();
            if let Ok(_) = file.read_to_string(&mut contents) {
                let mut scores: Vec<save_data_unit> = contents
                    .lines()
                    .filter_map(|line| {
                        let mut parts = line.split_whitespace();
                        match (parts.next(), parts.next()) {
                            (Some(score_str), Some(date)) => {
                                score_str.parse::<i32>().ok().map(|score| save_data_unit { score, date: date.to_owned() })
                            }
                            _ => None,
                        }
                    })
                    .collect();
                scores.sort_by_key(|unit| std::cmp::Reverse(unit.score));
                scores.truncate(5);
                scores
            } else {
                Vec::new()
            }
        }
        Err(_) => Vec::new(),
    }
}
// save five best scores (by score) from vector into file "scores.best"
fn save_new_data(mut data: Vec<save_data_unit>) {
    data.sort_by(|a, b| b.score.cmp(&a.score));
    if let Ok(mut file) = std::fs::File::create("scores.best") {
        for unit in data.iter().take(5) {
            writeln!(file, "{} {}", unit.score, unit.date).ok();
        }
    }
}
fn place_score_to_save(score: i32, date: String, mut data: Vec<save_data_unit>) -> Vec<save_data_unit> {
    data.push(save_data_unit { score, date });
    data.sort_by(|a, b| b.score.cmp(&a.score));
    data.truncate(5);
    data
}
fn get_date() -> String {
    let now = chrono::Local::now();
    format!("{}.{}.{}", now.day(), now.month(), now.year())
}
fn new_game(grid: &mut [[bool; GRID_SIZE]; GRID_SIZE], time: &mut f64, target_time: &mut f64, score: &mut i32, target_score: &mut i32, has_started: &mut bool, mouse_holded: &mut bool) {
    reset_grid(grid);
    *time = 0.0;
    *target_time = 10.0;
    *score = 0;
    *target_score = 35;
    *has_started = false;
    *mouse_holded = false;
}


async fn app(window: Window, mut gfx: Graphics, mut input: Input) -> Result<()> {
    let ttf = VectorFont::load("./assets/monogram.ttf").await?;
    let mut font = ttf.to_renderer(&gfx, 40.0)?;
    let mut small_font = ttf.to_renderer(&gfx, 20.0)?;

    let font_color = Color::from_hex("C69749"); // Orange-ish
    let time_color = Color::from_hex("735F32"); // Dark orange-ish
    let line_color = Color::from_hex("282A3A"); // Dark blue-ish
    let cell_color = Color::from_hex("FFF4E0"); // orange-white-ish

    let mut timer = Timer::time_per_second(FPS as f32);
    let mut time:f64 = 0.0;
    let mut target_time:f64 = 10.0;

    let mut has_started = false;
    let mut mouse_holded = false;
    let mut button_holded = false;

    let mut score = 0;
    let mut target_score = 40;

    let mut selected_option = MenuOptions::TargetTime;
    let mut game_state = GameState::Menu; //FIXME: Change to menu
    let mut saved_data = load_saved_data();

    // if empty, add five empty scores, with date "00.00.0000", and save data
    if saved_data.len() == 0 {
        for _ in 0..5 {
            saved_data.push(save_data_unit { score: 0, date: "00.00.0000".to_owned() });
        }
        save_new_data(saved_data.clone());
    }

    // 5x5 gird of bools
    let mut grid = [[false; GRID_SIZE]; GRID_SIZE];
    reset_grid(&mut grid);

    loop {
        while let Some(_) = input.next_event().await {}

        if timer.exhaust().is_some() {
        match game_state {
            GameState::Game=>{
                // UPDATE
                if has_started {
                    time += 1.0 / FPS;
                }

                // check if time is up (target time), if so, check if target score is reached, if not, end game, else add 20 to target score
                if time >= target_time {
                    if score < target_score {
                        saved_data = place_score_to_save(score, get_date(), saved_data.clone());
                        save_new_data(saved_data.clone());
                        game_state = GameState::End;
                    } else {
                        target_score += 35;
                        target_time += 10.0;
                    }
                }
        
                let mouse = gfx.screen_to_camera(&window, input.mouse().location());
                if input.mouse().left() && !mouse_holded {
                    let x = (mouse.x / GRID_CELL_SIZE_PX) as usize;
                    let y = (mouse.y / GRID_CELL_SIZE_PX) as usize;

                    mouse_holded = true;
        
                    if x < GRID_SIZE && y < GRID_SIZE {
                        if grid[x][y] {
                            if !has_started {
                                has_started = true;
                            }
                            score += 1;
                            grid[x][y] = false;
                            add_rand_to_grid(&mut grid);
                        } else {
                            score -= 4;
                            if score < 0 {
                                score = 0;
                            }
                        }
                    }
                }
                if !input.mouse().left() {
                    mouse_holded = false;
                }
                if input.key_down(quicksilver::input::Key::R) {new_game(&mut grid, &mut time, &mut target_time, &mut score, &mut target_score, &mut has_started, &mut mouse_holded)}
                if input.key_down(quicksilver::input::Key::L) {game_state = GameState::End}
                // DRAWING
                gfx.clear(Color::BLACK);
                // Draw grid
                for (i, row) in grid.iter().enumerate() {
                    for (j, cell) in row.iter().enumerate() {
                        let x = i as f32 * GRID_CELL_SIZE_PX;
                        let y = j as f32 * GRID_CELL_SIZE_PX;
                        let color = if *cell { cell_color } else { Color::BLACK };
                        gfx.fill_rect(&Rectangle::new(Vector::new(x, y), Vector::new(GRID_CELL_SIZE_PX, GRID_CELL_SIZE_PX)), color);
                    }
                }

                // Draw grid lines
                for i in 0..GRID_SIZE + 1 {
                    let x = i as f32 * GRID_CELL_SIZE_PX;
                    let y = i as f32 * GRID_CELL_SIZE_PX;
                    gfx.fill_rect(&Rectangle::new(Vector::new(x, 0.0), Vector::new(2.0, WINDOW_SIZE_PX)), line_color);
                    gfx.fill_rect(&Rectangle::new(Vector::new(0.0, y), Vector::new(WINDOW_SIZE_PX, 2.0)), line_color);
                }

                // Print score, target score and time
                let mut score_str = format!("{}", score);
                while score_str.len() < 3 {
                    score_str = format!("0{}", score_str);
                }
                let mut target_score_str = format!("{}", target_score);
                while target_score_str.len() < 3 {
                    target_score_str = format!("0{}", target_score_str);
                }
                // Score
                small_font.draw(&mut gfx, "Score:", font_color, Vector::new(WINDOW_SIZE_PX / 2.0 - 73.0, 18.0))?;
                font.draw(&mut gfx, &score_str, font_color, Vector::new(WINDOW_SIZE_PX / 2.0 - 75.0, 45.0))?;
                // Target score
                small_font.draw(&mut gfx, "Target:", font_color, Vector::new(WINDOW_SIZE_PX / 2.0 + 23.0, 18.0))?;
                font.draw(&mut gfx, &target_score_str, font_color, Vector::new(WINDOW_SIZE_PX / 2.0 + 25.0, 45.0))?;
                // Time
                small_font.draw(&mut gfx, "Time:", time_color, Vector::new(WINDOW_SIZE_PX - 80.0, 18.0))?;
                font.draw(&mut gfx, &format!("{:.2}", time), time_color, Vector::new(WINDOW_SIZE_PX - 100.0, 45.0))?;
            },
            GameState::Menu=>{
                // INPUT
                /*if input.mouse().left() {
                    game_state = GameState::Game;
                    reset_grid(&mut grid);
                    score = 0;
                    time = 0.0;
                    has_started = false;
                    target_score = 20;
                }*/
                if input.mouse().left() {
                    new_game(&mut grid, &mut time, &mut target_time, &mut score, &mut target_score, &mut has_started, &mut mouse_holded);
                    game_state = GameState::Game;
                }
                if input.mouse().right() {
                    game_state = GameState::Options;
                }
                if input.key_down(quicksilver::input::Key::Escape) {
                    return Ok(());
                }

                // DRAWING
                gfx.clear(Color::WHITE);
                font.draw(
                    &mut gfx,
                    "2D aim",
                    font_color,
                    Vector::new(WINDOW_SIZE_PX / 2.0 - 50.0, WINDOW_SIZE_PX / 3.0),
                )?;
                small_font.draw(
                    &mut gfx,
                    "Best Scores:",
                    font_color,
                    Vector::new(WINDOW_SIZE_PX / 2.0 - 65.0, WINDOW_SIZE_PX * 0.5 - 25.0),
                )?;

                for (i, unit) in saved_data.iter().enumerate() {
                    small_font.draw(
                        &mut gfx,
                        &format!("{}. {} - {}", i + 1, unit.score, unit.date),
                        font_color,
                        Vector::new(WINDOW_SIZE_PX / 2.0 - 65.0, WINDOW_SIZE_PX * 0.6 + (i as f32) * 25.0 - 45.0),
                    )?;
                }

                small_font.draw(
                    &mut gfx,
                    "Left click to start, right click for options",
                    font_color,
                    Vector::new(WINDOW_SIZE_PX / 2.0 - 197.0, WINDOW_SIZE_PX * 0.9),
                )?;
                small_font.draw(
                    &mut gfx,
                    "Press Esc to quit",
                    font_color,
                    Vector::new(WINDOW_SIZE_PX / 2.0 - 70.0, WINDOW_SIZE_PX * 0.95),
                )?;

            },
            GameState::End => {
                // INPUT
                if input.key_down(quicksilver::input::Key::Space) || input.key_down(quicksilver::input::Key::Return) {
                    game_state = GameState::Menu;
                }
                if input.key_down(quicksilver::input::Key::Escape) {
                    return Ok(());
                }

                // DRAWING
                gfx.clear(Color::BLACK);
                font.draw(
                    &mut gfx,
                    "Game Over",
                    font_color,
                    Vector::new(WINDOW_SIZE_PX / 2.0 - 70.0, WINDOW_SIZE_PX / 3.0),
                )?;
                font.draw(
                    &mut gfx,
                    &format!("Score: {}", score),
                    font_color,
                    Vector::new(WINDOW_SIZE_PX / 2.0 - 70.0, WINDOW_SIZE_PX / 2.0),
                )?;
                small_font.draw(
                    &mut gfx,
                    "Press space to return to the menu",
                    font_color,
                    Vector::new(WINDOW_SIZE_PX / 2.0 - 150.0, WINDOW_SIZE_PX * 0.9),
                )?;
                small_font.draw(
                    &mut gfx,
                    "Press escape to quit",
                    font_color,
                    Vector::new(WINDOW_SIZE_PX / 2.0 - 100.0, WINDOW_SIZE_PX * 0.95),
                )?;
                
            },
            GameState::Options=>{
                // options for: target time and target score, reset best scores
                //FIXME:

                // INPUT
                if input.key_down(quicksilver::input::Key::Escape) && !button_holded {
                    game_state = GameState::Menu;
                    button_holded = true;
                } else if input.key_down(quicksilver::input::Key::Up) && !button_holded {
                    match selected_option {
                        MenuOptions::TargetTime => {
                            selected_option = MenuOptions::Back;
                        },
                        MenuOptions::TargetScore => {
                            selected_option = MenuOptions::TargetTime;
                        },
                        MenuOptions::Reset => {
                            selected_option = MenuOptions::TargetScore;
                        },
                        MenuOptions::Back => {
                            selected_option = MenuOptions::Reset;
                        },
                    }
                    button_holded = true;
                } else if input.key_down(quicksilver::input::Key::Down) && !button_holded {
                    match selected_option {
                        MenuOptions::TargetTime => {
                            selected_option = MenuOptions::TargetScore;
                        },
                        MenuOptions::TargetScore => {
                            selected_option = MenuOptions::Reset;
                        },
                        MenuOptions::Reset => {
                            selected_option = MenuOptions::Back;
                        },
                        MenuOptions::Back => {
                            selected_option = MenuOptions::TargetTime;
                        },
                    }
                    button_holded = true;
                } else if input.key_down(quicksilver::input::Key::Left) && !button_holded {
                    match selected_option {
                        MenuOptions::TargetTime => {
                            if target_time > 0.0 {
                                target_time -= 1.0;
                            }
                        },
                        MenuOptions::TargetScore => {
                            if target_score > 0 {
                                target_score -= 1;
                            }
                        },
                        MenuOptions::Reset => {
                            // do nothing
                        },
                        MenuOptions::Back => {
                            // do nothing
                        },
                    }
                    button_holded = true;
                } else if input.key_down(quicksilver::input::Key::Right) && !button_holded {
                    match selected_option {
                        MenuOptions::TargetTime => {
                            target_time += 1.0;
                        },
                        MenuOptions::TargetScore => {
                            target_score += 1;
                        },
                        MenuOptions::Reset => {
                            // do nothing
                        },
                        MenuOptions::Back => {
                            // do nothing
                        },
                    }
                    button_holded = true;
                } else if input.key_down(quicksilver::input::Key::Return) && !button_holded {
                    match selected_option {
                        MenuOptions::TargetTime => {
                            // do nothing
                        },
                        MenuOptions::TargetScore => {
                            // do nothing
                        },
                        MenuOptions::Reset => {
                            // reset best scores
                            //saved_data = Vec::new();
                            //save_new_data(saved_data);
                            // delete file
                            let path = Path::new("scores.best");
                            if path.exists() {
                                fs::remove_file(path)?;
                            }
                        },
                        MenuOptions::Back => {
                            game_state = GameState::Menu;
                        },
                    }
                    button_holded = true;
                } else {
                    button_holded = false;
                }
                // DRAWING

                gfx.clear(Color::WHITE);
                font.draw(
                    &mut gfx,
                    "Options",
                    font_color,
                    Vector::new(WINDOW_SIZE_PX / 2.0 - 50.0, WINDOW_SIZE_PX / 3.0),
                )?;
                small_font.draw(
                    &mut gfx,
                    "Target time:",
                    font_color,
                    Vector::new(WINDOW_SIZE_PX / 2.0 - 100.0, WINDOW_SIZE_PX / 2.0 - 50.0),
                )?;
                small_font.draw(
                    &mut gfx,
                    &format!("{}", target_time),
                    font_color,
                    Vector::new(WINDOW_SIZE_PX / 2.0 + 50.0, WINDOW_SIZE_PX / 2.0 - 50.0),
                )?;
                small_font.draw(
                    &mut gfx,
                    "Target score:",
                    font_color,
                    Vector::new(WINDOW_SIZE_PX / 2.0 - 100.0, WINDOW_SIZE_PX / 2.0),
                )?;
                small_font.draw(
                    &mut gfx,
                    &format!("{}", target_score),
                    font_color,
                    Vector::new(WINDOW_SIZE_PX / 2.0 + 50.0, WINDOW_SIZE_PX / 2.0),
                )?;
                small_font.draw(
                    &mut gfx,
                    "Reset best scores",
                    font_color,
                    Vector::new(WINDOW_SIZE_PX / 2.0 - 100.0, WINDOW_SIZE_PX / 2.0 + 50.0),
                )?;
                small_font.draw(
                    &mut gfx,
                    "Back",
                    font_color,
                    Vector::new(WINDOW_SIZE_PX / 2.0 - 100.0, WINDOW_SIZE_PX / 2.0 + 100.0),
                )?;
                match selected_option {
                    MenuOptions::TargetTime => {
                        small_font.draw(
                            &mut gfx,
                            ">",
                            font_color,
                            Vector::new(WINDOW_SIZE_PX / 2.0 - 120.0, WINDOW_SIZE_PX / 2.0 - 50.0),
                        )?;
                    },
                    MenuOptions::TargetScore => {
                        small_font.draw(
                            &mut gfx,
                            ">",
                            font_color,
                            Vector::new(WINDOW_SIZE_PX / 2.0 - 120.0, WINDOW_SIZE_PX / 2.0),
                        )?;
                    },
                    MenuOptions::Reset => {
                        small_font.draw(
                            &mut gfx,
                            ">",
                            font_color,
                            Vector::new(WINDOW_SIZE_PX / 2.0 - 120.0, WINDOW_SIZE_PX / 2.0 + 50.0),
                        )?;
                    },
                    MenuOptions::Back => {
                        small_font.draw(
                            &mut gfx,
                            ">",
                            font_color,
                            Vector::new(WINDOW_SIZE_PX / 2.0 - 120.0, WINDOW_SIZE_PX / 2.0 + 100.0),
                        )?;
                    },
                }

                //game_state = GameState::Menu;
            },
            _=>{}
        };
        
        // Present the frame
        gfx.present(&window)?;
        }
    }
}