use rand::{thread_rng, Rng};
use rand::seq::SliceRandom;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use std::time::Duration;

// Grid and cell size definitions
const GRID_SIZE: usize = 49;
const CELL_SIZE: u32 = 20;
const WINDOW_SIZE: u32 = (GRID_SIZE as u32) * CELL_SIZE;

// Directions for movement (not currently used in logic)
#[derive(Clone, Copy, PartialEq)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
}

// Player struct with a position field
struct Player {
    position: (usize, usize),
}

impl Player {
    // Constructor for Player, starts at (1,1)
    fn new() -> Self {
        Player {
            position: (1, 1),
        }
    }
}

// Maze generation using recursive backtracking
fn create_maze() -> Vec<Vec<u8>> {
    let mut maze = vec![vec![1; GRID_SIZE]; GRID_SIZE]; // 1 = wall

    // Recursive carving function
    fn carve(x: usize, y: usize, maze: &mut Vec<Vec<u8>>) {
        let mut dirs = vec![(0, -2), (2, 0), (0, 2), (-2, 0)];
        let mut rng = thread_rng();
        dirs.shuffle(&mut rng); // Randomize directions

        for (dx, dy) in dirs {
            let nx = x as isize + dx;
            let ny = y as isize + dy;

            // Check bounds and avoid carving out of maze
            if nx > 0 && ny > 0 && nx < (GRID_SIZE - 1) as isize && ny < (GRID_SIZE - 1) as isize {
                let nx = nx as usize;
                let ny = ny as usize;

                // If the target cell is a wall, carve a path to it
                if maze[ny][nx] == 1 {
                    maze[ny][nx] = 0;
                    maze[(y as isize + dy / 2) as usize][(x as isize + dx / 2) as usize] = 0;
                    carve(nx, ny, maze);
                }
            }
        }
    }

    // Start carving from (1,1)
    maze[1][1] = 0;
    carve(1, 1, &mut maze);

    // Set start and end points
    maze[1][1] = 0;
    maze[47][47] = 0;

    // Create borders (outer frame as walls)
    for i in 0..GRID_SIZE {
        maze[0][i] = 1;
        maze[GRID_SIZE - 1][i] = 1;
        maze[i][0] = 1;
        maze[i][GRID_SIZE - 1] = 1;
    }

    maze
}

// Drawing the maze on the SDL2 canvas
fn draw_maze(canvas: &mut Canvas<Window>, maze: &Vec<Vec<u8>>, player: &Player) -> Result<(), String> {
    let start = player.position;
    let end = (47, 47);

    for y in 0..GRID_SIZE {
        for x in 0..GRID_SIZE {
            let color = if (x, y) == start {
                Color::GREEN // Start cell
            } else if (x, y) == end {
                Color::RED // End cell
            } else if maze[y][x] == 1 {
                Color::RGB(150, 150, 150) // Wall
            } else if maze[y][x] == 2 {
                Color::RGB(0, 100, 255) // Path taken
            } else if maze[y][x] == 3 {
                Color::RGB(255, 100, 100) // Backtracked path
            } else {
                Color::BLACK // Empty space
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

// Recursive backtracking maze solver with animation
fn solve_maze_step_by_step(
    maze: &mut Vec<Vec<u8>>,
    x: usize,
    y: usize,
    canvas: &mut Canvas<Window>,
    player: &Player,
) -> bool {
    // Bounds and wall check
    if x >= GRID_SIZE || y >= GRID_SIZE || maze[y][x] != 0 {
        return false;
    }

    // Reached the goal
    if (x, y) == (47, 47) {
        maze[y][x] = 2;
        draw_maze(canvas, maze, player).unwrap();
        canvas.present();
        std::thread::sleep(Duration::from_millis(10));
        return true;
    }

    // Mark current cell as visited path
    maze[y][x] = 2;
    draw_maze(canvas, maze, player).unwrap();
    canvas.present();
    std::thread::sleep(Duration::from_millis(50));

    // Explore 4 directions: up, right, down, left
    let dirs = [(0isize, -1), (1, 0), (0, 1), (-1, 0)];
    for (dx, dy) in dirs {
        let nx = x as isize + dx;
        let ny = y as isize + dy;

        if nx >= 0 && ny >= 0 {
            if solve_maze_step_by_step(maze, nx as usize, ny as usize, canvas, player) {
                return true;
            }
        }
    }

    // Backtrack and mark the cell as tried but failed
    maze[y][x] = 3;
    draw_maze(canvas, maze, player).unwrap();
    canvas.present();
    std::thread::sleep(Duration::from_millis(50));
    false
}

// Entry point
fn main() -> Result<(), String> {
    // Initialize SDL2 context and create window and canvas
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
    let mut maze = create_maze();
    let mut solving = true;

    // Print maze to console for debugging
    for line in &maze {
        println!("{:?}", line)
    }

    let mut player = Player::new();

    // Main event/render loop
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::KeyDown { keycode: Some(k), repeat: false, .. } => {
                    match k {
                        Keycode::Escape => break 'running, // Exit on ESC
                        Keycode::Return => {
                            if solving {
                                solve_maze_step_by_step(&mut maze, 1, 1, &mut canvas, &player);
                                solving = false;
                            }
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }

        // If player reaches the end
        if player.position == (47, 47) {
            println!("you win");
            break 'running;
        }

        // Clear screen
        canvas.set_draw_color(Color::BLACK);
        canvas.clear();

        // Draw current maze state
        draw_maze(&mut canvas, &maze, &player)?;
        canvas.present();

        // Wait a short time to control FPS
        std::thread::sleep(Duration::from_millis(16));
    }

    Ok(())
}
