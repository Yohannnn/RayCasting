use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Point;
use sdl2::render::WindowCanvas;
use std::time::{Duration, Instant};
use std::ops;


// Screen size
const SCREEN_WIDTH: u32 = 640;
const SCREEN_HEIGHT: u32 = 480;

// Speed and rotation speed
const MOV_SPEED: f64 = 0.03;
const ROT_SPEED: f64 = 0.03;

// Point struct
#[derive(Debug, Copy, Clone)]
struct V<T> {
    x: T,
    y: T,
}

impl ops::Neg for V<f64> {
    type Output = V<f64>;

    fn neg(self) -> V<f64> {
        V{x: -self.x, y: -self.y}
    }
} 


// State struct
#[derive(Debug, Copy, Clone)]
struct State {
    pos: V<f64>,
    dir: V<f64>,
    plane: V<f64>,
}

// Map of the walls
const MAP: [[u8; 24]; 24] =[
  [4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,7,7,7,7,7,7,7,7],
  [4,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,7,0,0,0,0,0,0,7],
  [4,0,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,7],
  [4,0,2,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,7],
  [4,0,3,0,0,0,0,0,0,0,0,0,0,0,0,0,7,0,0,0,0,0,0,7],
  [4,0,4,0,0,0,0,5,5,5,5,5,5,5,5,5,7,7,0,7,7,7,7,7],
  [4,0,5,0,0,0,0,5,0,5,0,5,0,5,0,5,7,0,0,0,7,7,7,1],
  [4,0,6,0,0,0,0,5,0,0,0,0,0,0,0,5,7,0,0,0,0,0,0,8],
  [4,0,7,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,7,7,7,1],
  [4,0,8,0,0,0,0,5,0,0,0,0,0,0,0,5,7,0,0,0,0,0,0,8],
  [4,0,0,0,0,0,0,5,0,0,0,0,0,0,0,5,7,0,0,0,7,7,7,1],
  [4,0,0,0,0,0,0,5,5,5,5,0,5,5,5,5,7,7,7,7,7,7,7,1],
  [6,6,6,6,6,6,6,6,6,6,6,0,6,6,6,6,6,6,6,6,6,6,6,6],
  [8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,4],
  [6,6,6,6,6,6,0,6,6,6,6,0,6,6,6,6,6,6,6,6,6,6,6,6],
  [4,4,4,4,4,4,0,4,4,4,6,0,6,2,2,2,2,2,2,2,3,3,3,3],
  [4,0,0,0,0,0,0,0,0,4,6,0,6,2,0,0,0,0,0,2,0,0,0,2],
  [4,0,0,0,0,0,0,0,0,0,0,0,6,2,0,0,5,0,0,2,0,0,0,2],
  [4,0,0,0,0,0,0,0,0,4,6,0,6,2,0,0,0,0,0,2,2,0,2,2],
  [4,0,6,0,6,0,0,0,0,4,6,0,0,0,0,0,5,0,0,0,0,0,0,2],
  [4,0,0,5,0,0,0,0,0,4,6,0,6,2,0,0,0,0,0,2,2,0,2,2],
  [4,0,6,0,6,0,0,0,0,4,6,0,6,2,0,0,5,0,0,2,0,0,0,2],
  [4,0,0,0,0,0,0,0,0,4,6,0,6,2,0,0,0,0,0,2,0,0,0,2],
  [4,4,4,4,4,4,4,4,4,4,1,1,1,2,2,2,2,2,2,3,3,3,3,3]
];


// Rotate the player and the camera plane
fn rotate(state: &mut State, angle: f64) {
    let (s, c) = angle.sin_cos();
    let old_dir_x = state.dir.x;
    state.dir.x = state.dir.x * c - state.dir.y * s;
    state.dir.y = old_dir_x * s + state.dir.y * c;
    let old_plane_x = state.plane.x;
    state.plane.x = state.plane.x * c - state.plane.y * s;
    state.plane.y = old_plane_x * s + state.plane.y * c;
}

// Move the player
fn move_player(s: &mut State, forward: bool) {
    let dir = if forward {s.dir} else {-s.dir};
    let move_x = s.pos.x + dir.x * MOV_SPEED;
    let move_y = s.pos.y + dir.y * MOV_SPEED;

    if MAP[s.pos.y as usize][move_x as usize] == 0 {
        s.pos.x = move_x;
    }
    if MAP[move_y as usize][s.pos.x as usize] == 0 {
        s.pos.y = move_y;
    }
}

// Render function
fn render(canvas: &mut WindowCanvas, s: &State) -> Result<(), String> {
    // Clear the screen to black
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();

    // Render the walls
    let mut x = 0;
    while x < SCREEN_WIDTH{
        // X cord on the camera plane
        let camera_x = 2.0 * x as f64 / SCREEN_WIDTH as f64 - 1.0;

        // Ray direction
        let ray = V{x: s.dir.x + s.plane.x * camera_x, y: s.dir.y + s.plane.y * camera_x};

        // Position of the player on the map as an int
        let mut map_pos = V{x: s.pos.x as i32, y: s.pos.y as i32};

        // Length of the ray from one x or y-side to the next x or y-side
        let delta_dist = V{x: (1.0 / ray.x).abs(), y: (1.0 / ray.y).abs()};

        // Value to increment the map pos by
        let mut step = V{x: 0, y: 0};

        // Distance to the next x or y-side
        let mut side_dist = V{x: 0.0, y: 0.0};

        // Length of the ray from the wall hit to the camera plane
        let perp_wal_dist: f64;

        // Was a NS wall hit?
        let mut ns_side: bool;

        // Color of the wall
        let mut wall_color: Color;

        // Calculate the step and initial side_dist
        if ray.x < 0.0 {
            step.x = -1;
            side_dist.x = (s.pos.x - map_pos.x as f64) * delta_dist.x;
        } else {
            step.x = 1;
            side_dist.x = (map_pos.x as f64 + 1.0 - s.pos.x) * delta_dist.x;
        }

        if ray.y < 0.0 {
            step.y = -1;
            side_dist.y = (s.pos.y - map_pos.y as f64) * delta_dist.y;
        } else {
            step.y = 1;
            side_dist.y = (map_pos.y as f64 + 1.0 - s.pos.y) * delta_dist.y;
        }

        // Perform DDA
        loop {
            // Jump to the next map square, OR in x-direction, OR in y-direction
            if side_dist.x < side_dist.y {
                side_dist.x += delta_dist.x;
                map_pos.x += step.x;
                ns_side = false;
            } else {
                side_dist.y += delta_dist.y;
                map_pos.y += step.y;
                ns_side = true;
            }

            // Check if the ray has hit a wall
            let wall = MAP[map_pos.y as usize][map_pos.x as usize];
            if wall > 0 {
                wall_color = match wall {
                    1 => Color::RED,
                    2 => Color::GREEN,
                    3 => Color::BLUE,
                    4 => Color::WHITE,
                    _ => Color::YELLOW
                }; 
                break;
            }
        }

        // Calculate the distance projected on the camera direction
        if ns_side {
            perp_wal_dist = side_dist.y - delta_dist.y;
        } else {
            perp_wal_dist = side_dist.x - delta_dist.x;
        }

        // Calculate the height of the line to draw on the screen
        let line_height = (SCREEN_HEIGHT as f64 / perp_wal_dist) as i32;

        // Calculate the lowest and highest pixel to fill in the current stripe
        let mut draw_start: Point = Point::new(x as i32, -line_height / 2 + (SCREEN_HEIGHT / 2) as i32);
        if draw_start.y < 0 {
            draw_start.y = 0;
        }

        let mut draw_end: Point = Point::new(x as i32, line_height / 2 + (SCREEN_HEIGHT / 2) as i32);
        if draw_end.y >= SCREEN_HEIGHT as i32 {
            draw_end.y = SCREEN_HEIGHT as i32 - 1;
        }

        // Give x and y sides different brightness
        if ns_side {
            wall_color.r /= 2;
            wall_color.g /= 2;
            wall_color.b /= 2;
        }

        // Draw the wall
        canvas.set_draw_color(wall_color);
        canvas.draw_line(draw_start, draw_end)?;
        x += 1;
    }

    // Update the screen
    canvas.present();
    Ok(())
}

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem.window("raycaster", SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .build()
        .expect("could not initialize video subsystem");

    let mut canvas = window.into_canvas().build()
        .expect("could not make a canvas");

    let mut event_pump = sdl_context.event_pump()?;

    let mut state = State {
        pos: V { x: 4.5, y: 5.0 },
        dir: V { x:-1.0, y: 0.0 },
        plane: V { x: 0.0, y: 0.66 },
     };
    
    let mut movement_keys = [false; 4];

    'running: loop {
        // Get the current time
        let start_time = Instant::now();

        // Handle events
        for event in event_pump.poll_iter() {
            match event {
                // Detect window close or escape key
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running;
                },

                // Movement input
                Event::KeyDown { keycode: Some(Keycode::W), repeat: false, .. } => {
                    movement_keys[0] = true;
                },
                Event::KeyDown { keycode: Some(Keycode::S), repeat: false, .. } => {
                    movement_keys[1] = true;
                },
                Event::KeyDown { keycode: Some(Keycode::A), repeat: false, .. } => {
                    movement_keys[2] = true;
                },
                Event::KeyDown { keycode: Some(Keycode::D), repeat: false, .. } => {
                    movement_keys[3] = true;
                },

                Event::KeyUp { keycode: Some(Keycode::W), repeat: false, .. } => {
                    movement_keys[0] = false;
                },
                Event::KeyUp { keycode: Some(Keycode::S), repeat: false, .. } => {
                    movement_keys[1] = false;
                },
                Event::KeyUp { keycode: Some(Keycode::A), repeat: false, .. } => {
                    movement_keys[2] = false;
                },
                Event::KeyUp { keycode: Some(Keycode::D), repeat: false, .. } => {
                    movement_keys[3] = false;
                },
                _ => {}
            }
        }

        // Update
        if movement_keys[0] {
            move_player(&mut state, true);
        }

        if movement_keys[1] {
            move_player(&mut state, false);
        }

        if movement_keys[2] {
            rotate(&mut state, ROT_SPEED)
        }
        
        if movement_keys[3] {
            rotate(&mut state, -ROT_SPEED)
        }

        // Render
        render(&mut canvas, &state)?;

        // Sleep to maintain framerate of 60fps
        let elapsed_time = start_time.elapsed();
        if elapsed_time < Duration::from_millis(16) {
            std::thread::sleep(Duration::from_millis(16) - elapsed_time);
        }
    }

    Ok(())
}