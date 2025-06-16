// Quantum Maze Solver - Rust + SDL2
use rand::{thread_rng, seq::SliceRandom};
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
const START: (usize, usize) = (1, 1);
const END: (usize, usize) = (47, 47);

#[derive(Clone, Copy, PartialEq)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
}

#[derive(Clone)]
struct Agent {
    position: (usize, usize),
    path: Vec<(usize, usize)>,
    id: usize,
}

impl Agent {
    fn new(position: (usize, usize), path: Vec<(usize, usize)>, id: usize) -> Self {
        Agent { position, path, id }
    }
}

fn create_maze() -> Vec<Vec<u8>> {
    let mut maze = vec![vec![1; GRID_SIZE]; GRID_SIZE]; // 1 = wall, 0 = path

    fn carve(x: usize, y: usize, maze: &mut Vec<Vec<u8>>) {
        let mut dirs = vec![(0, -2), (2, 0), (0, 2), (-2, 0)];
        let mut rng = thread_rng();
        dirs.shuffle(&mut rng);

        for (dx, dy) in dirs {
            let nx = x as isize + dx;
            let ny = y as isize + dy;
            // if nx == 47 && ny == 47 {
            //     return;
            // }

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

fn draw_maze(canvas: &mut Canvas<Window>, maze: &Vec<Vec<u8>>, agents: &Vec<Agent>) -> Result<(), String> {
    for y in 0..GRID_SIZE {
        for x in 0..GRID_SIZE {
            let color = if (x, y) == END {
                Color::RED
            } else {
                match maze[y][x] {
                    1 => Color::RGB(150, 150, 150),
                    2 => Color::RGB(0, 100, 255),
                    3 => Color::RGB(255, 100, 100),
                    _ => Color::BLACK,
                }
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

    for agent in agents {
        canvas.set_draw_color(Color::GREEN);
        let rect = Rect::new(
            (agent.position.0 as u32 * CELL_SIZE) as i32,
            (agent.position.1 as u32 * CELL_SIZE) as i32,
            CELL_SIZE,
            CELL_SIZE,
        );
        canvas.fill_rect(rect)?;
    }

    Ok(())
}

fn check_available_moves(maze: &Vec<Vec<u8>>, agent: &Agent) -> Vec<Direction> {
    let (px, py) = agent.position;
    let mut dirs = Vec::new();

    let checks = [
        (Direction::Up, px, py.wrapping_sub(1)),
        (Direction::Down, px, py + 1),
        (Direction::Right, px + 1, py),
        (Direction::Left, px.wrapping_sub(1), py),
    ];

    for (dir, x, y) in checks {
        if x < GRID_SIZE && y < GRID_SIZE && maze[y][x] == 0 && !agent.path.contains(&(x, y)) {
            dirs.push(dir);
        }
    }

    dirs
}

fn move_agent(agent: &mut Agent, dir: Direction) {
    match dir {
        Direction::Up => agent.position.1 -= 1,
        Direction::Down => agent.position.1 += 1,
        Direction::Right => agent.position.0 += 1,
        Direction::Left => agent.position.0 -= 1,
    }
    agent.path.push(agent.position);
}

fn draw_solution_path(canvas: &mut Canvas<Window>, maze: &Vec<Vec<u8>>, path: &Vec<(usize, usize)>) -> Result<(), String> {
    for y in 0..GRID_SIZE {
        for x in 0..GRID_SIZE {
            let color = if (x, y) == START {
                Color::GREEN
            } else if (x, y) == END {
                Color::RED
            } else if path.contains(&(x, y)) {
                Color::RGB(0, 200, 255)
            } else if maze[y][x] == 1 {
                Color::RGB(150, 150, 150)
            } else {
                Color::BLACK
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
        .window("Quantum Maze Search", WINDOW_SIZE, WINDOW_SIZE)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;
    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    let mut event_pump = sdl_context.event_pump()?;

    let mut maze = create_maze();
    let mut agents = vec![Agent::new(START, vec![START], 0)];
    let mut winner_path = None;

    'running: loop {
        for event in event_pump.poll_iter() {
            if let Event::KeyDown { keycode: Some(Keycode::Escape), .. } = event {
                break 'running;
            }
        }

        if let Some(ref path) = winner_path {
            canvas.set_draw_color(Color::BLACK);
            canvas.clear();
            draw_solution_path(&mut canvas, &maze, path)?;
            canvas.present();
            continue;
        }

        let mut new_agents = vec![];
        let mut i = 0;
        while i < agents.len() {
            let mut agent = agents[i].clone();
            let dirs = check_available_moves(&maze, &agent);

            if agent.position == END {
                println!("Agent {} reached the end!", agent.id);
                winner_path = Some(agent.path.clone());
                break;
            }

            if dirs.is_empty() {
                maze[agent.position.1][agent.position.0] = 3;
                agents.remove(i);
                continue;
            }

            if dirs.len() > 1 {
                for (j, dir) in dirs.iter().enumerate() {
                    if j == dirs.len()-1 {
                        move_agent(&mut agent, *dir);
                        maze[agent.position.1][agent.position.0] = 2;
                        agents[i] = agent.clone();
                    } else {
                        let mut new = agent.clone();
                        //new.path.pop();
                        move_agent(&mut new, *dir);
                        maze[new.position.1][new.position.0] = 2;
                        new.id = agents.len() + new_agents.len();
                        new_agents.push(new);
                    }
                }
            } else {
                move_agent(&mut agent, dirs[0]);
                maze[agent.position.1][agent.position.0] = 2;
                agents[i] = agent;
            }

            i += 1;
        }

        agents.extend(new_agents);

        canvas.set_draw_color(Color::BLACK);
        canvas.clear();
        draw_maze(&mut canvas, &maze, &agents)?;
        canvas.present();
        std::thread::sleep(Duration::from_millis(50));
    }

    Ok(())
}

