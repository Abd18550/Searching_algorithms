use rand::{thread_rng, Rng};
use rand::seq::SliceRandom;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use std::time::Duration;

const GRID_SIZE: usize = 49;
const CELL_SIZE: u32 = 20;
const WINDOW_SIZE: u32 = (GRID_SIZE as u32) * CELL_SIZE;

fn create_maze() -> Vec<Vec<u8>> {
    let mut maze = vec![vec![1; GRID_SIZE]; GRID_SIZE]; // 1 = wall, 0 = path

    fn carve(x: usize, y: usize, maze: &mut Vec<Vec<u8>>) {
        let mut dirs = vec![(0, -2), (2, 0), (0, 2), (-2, 0)];
        let mut rng = thread_rng();
        dirs.shuffle(&mut rng);

        for (dx, dy) in dirs {
            let nx = x as isize + dx;
            let ny = y as isize + dy;

            if nx > 0 && ny > 0 && nx < (GRID_SIZE - 1) as isize && ny < (GRID_SIZE - 1) as isize {
                let nx = nx as usize;
                let ny = ny as usize;

                if maze[ny][nx] == 1 {
                    maze[ny][nx] = 0;
                    maze[(y as isize + dy / 2) as usize][(x as isize + dx / 2) as usize] = 0;
                    carve(nx, ny, maze);
                }
            }
        }
    }

    // Start carving from inside
    maze[1][1] = 0;
    carve(1, 1, &mut maze);

    // Ensure start and end points are path
    maze[1][1] = 0;
    maze[47][47] = 0;

    // Add solid border
    for i in 0..GRID_SIZE {
        maze[0][i] = 1;
        maze[GRID_SIZE - 1][i] = 1;
        maze[i][0] = 1;
        maze[i][GRID_SIZE - 1] = 1;
    }

    maze
}

fn draw_maze(canvas: &mut Canvas<Window>, maze: &Vec<Vec<u8>>) -> Result<(), String> {
    let start = (1, 1);
    let end = (47, 47);

    for y in 0..GRID_SIZE {
        for x in 0..GRID_SIZE {
            let color = if (x, y) == start {
                Color::GREEN
            } else if (x, y) == end {
                Color::RED
            } else if maze[y][x] == 1 {
                Color::RGB(150, 150, 150) // Wall
            } else {
                Color::BLACK // Path
            };

            canvas.set_draw_color(color);
            let rect = Rect::new(
                (x as u32 * CELL_SIZE) as i32,
                (y as u32 * CELL_SIZE) as i32,
                CELL_SIZE,
                CELL_SIZE,
            );
            canvas.fill_rect(rect)?;
        }
    }

    Ok(())
}

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("Maze Simulation", WINDOW_SIZE, WINDOW_SIZE)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window
        .into_canvas()
        .present_vsync()
        .build()
        .map_err(|e| e.to_string())?;

    let mut event_pump = sdl_context.event_pump()?;
    let maze = create_maze();

    'running: loop {
        for event in event_pump.poll_iter() {
            if let Event::Quit { .. }
            | Event::KeyDown {
                keycode: Some(Keycode::Escape),
                ..
            } = event
            {
                break 'running;
            }
        }

        canvas.set_draw_color(Color::BLACK);
        canvas.clear();

        draw_maze(&mut canvas, &maze)?;
        canvas.present();

        std::thread::sleep(Duration::from_millis(16));
    }

    Ok(())
}
