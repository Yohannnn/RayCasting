use sdl2::{
    event::Event,
    image::LoadSurface,
    keyboard::Keycode,
    pixels::Color,
    rect::{Point, Rect},
    surface::Surface,
};
use std::{ops, time::Instant};

// Screen size
const PLANE_WIDTH: u32 = 320;
const PLANE_HEIGHT: u32 = 200;
const TEX_SIZE: u32 = 64;

// Point struct
#[derive(Debug, Copy, Clone)]
struct V<T: num::Num> {
    x: T,
    y: T,
}

impl ops::Neg for V<f64> {
    type Output = V<f64>;

    fn neg(self) -> V<f64> {
        V {
            x: -self.x,
            y: -self.y,
        }
    }
}

// State struct
#[derive(Debug, Copy, Clone)]
struct State {
    pos: V<f64>,
    dir: V<f64>,
    plane: V<f64>,
}

//TODO: Switch to using config file for texture and map data
// Map of the walls
const MAP: [[u8; 24]; 24] = [
    [
        4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 7, 7, 7, 7, 7, 7, 7, 7,
    ],
    [
        4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 7, 0, 0, 0, 0, 0, 0, 7,
    ],
    [
        4, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 7,
    ],
    [
        4, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 7,
    ],
    [
        4, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 7, 0, 0, 0, 0, 0, 0, 7,
    ],
    [
        4, 0, 4, 0, 0, 0, 0, 5, 5, 5, 5, 5, 5, 5, 5, 5, 7, 7, 0, 7, 7, 7, 7, 7,
    ],
    [
        4, 0, 5, 0, 0, 0, 0, 5, 0, 5, 0, 5, 0, 5, 0, 5, 7, 0, 0, 0, 7, 7, 7, 1,
    ],
    [
        4, 0, 6, 0, 0, 0, 0, 5, 0, 0, 0, 0, 0, 0, 0, 5, 7, 0, 0, 0, 0, 0, 0, 8,
    ],
    [
        4, 0, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 7, 7, 7, 1,
    ],
    [
        4, 0, 8, 0, 0, 0, 0, 5, 0, 0, 0, 0, 0, 0, 0, 5, 7, 0, 0, 0, 0, 0, 0, 8,
    ],
    [
        4, 0, 0, 0, 0, 0, 0, 5, 0, 0, 0, 0, 0, 0, 0, 5, 7, 0, 0, 0, 7, 7, 7, 1,
    ],
    [
        4, 0, 0, 0, 0, 0, 0, 5, 5, 5, 5, 0, 5, 5, 5, 5, 7, 7, 7, 7, 7, 7, 7, 1,
    ],
    [
        6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 0, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6,
    ],
    [
        8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4,
    ],
    [
        6, 6, 6, 6, 6, 6, 0, 6, 6, 6, 6, 0, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6,
    ],
    [
        4, 4, 4, 4, 4, 4, 0, 4, 4, 4, 6, 0, 6, 2, 2, 2, 2, 2, 2, 2, 3, 3, 3, 3,
    ],
    [
        4, 0, 0, 0, 0, 0, 0, 0, 0, 4, 6, 0, 6, 2, 0, 0, 0, 0, 0, 2, 0, 0, 0, 2,
    ],
    [
        4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 6, 2, 0, 0, 5, 0, 0, 2, 0, 0, 0, 2,
    ],
    [
        4, 0, 0, 0, 0, 0, 0, 0, 0, 4, 6, 0, 6, 2, 0, 0, 0, 0, 0, 2, 2, 0, 2, 2,
    ],
    [
        4, 0, 6, 0, 6, 0, 0, 0, 0, 4, 6, 0, 0, 0, 0, 0, 5, 0, 0, 0, 0, 0, 0, 2,
    ],
    [
        4, 0, 0, 5, 0, 0, 0, 0, 0, 4, 6, 0, 6, 2, 0, 0, 0, 0, 0, 2, 2, 0, 2, 2,
    ],
    [
        4, 0, 6, 0, 6, 0, 0, 0, 0, 4, 6, 0, 6, 2, 0, 0, 5, 0, 0, 2, 0, 0, 0, 2,
    ],
    [
        4, 0, 0, 0, 0, 0, 0, 0, 0, 4, 6, 0, 6, 2, 0, 0, 0, 0, 0, 2, 0, 0, 0, 2,
    ],
    [
        4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 1, 1, 1, 2, 2, 2, 2, 2, 2, 3, 3, 3, 3, 3,
    ],
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
fn move_player(s: &mut State, speed: f64) {
    let move_x = s.pos.x + s.dir.x * speed;
    let move_y = s.pos.y + s.dir.y * speed;

    if MAP[s.pos.y as usize][move_x as usize] == 0 {
        s.pos.x = move_x;
    }
    if MAP[move_y as usize][s.pos.x as usize] == 0 {
        s.pos.y = move_y;
    }
}

fn main() -> Result<(), String> {
    // Initialize SDL2
    let sdl_context = sdl2::init()?;

    // Hide the cursor
    sdl_context.mouse().show_cursor(false);

    // Initialize the video subsystem
    let video_subsystem = sdl_context.video()?;

    // Create a window
    let window = video_subsystem
        .window("raycaster", PLANE_WIDTH, PLANE_HEIGHT)
        .fullscreen()
        .build()
        .map_err(|e| e.to_string())?;

    // Create a canvas
    let mut canvas = window
        .into_canvas()
        .present_vsync()
        .build()
        .map_err(|e| e.to_string())?;

    // Set the size of the canvas
    canvas.set_logical_size(PLANE_WIDTH, PLANE_HEIGHT).unwrap();

    let mut event_pump = sdl_context.event_pump()?;

    // Load in the surfaces from files
    let textures: [Surface; 8] = [
        Surface::from_file("assets/eagle.png")?,
        Surface::from_file("assets/redbrick.png")?,
        Surface::from_file("assets/purplestone.png")?,
        Surface::from_file("assets/greystone.png")?,
        Surface::from_file("assets/bluestone.png")?,
        Surface::from_file("assets/mossy.png")?,
        Surface::from_file("assets/wood.png")?,
        Surface::from_file("assets/colorstone.png")?,
    ];

    let mut state = State {
        pos: V { x: 4.5, y: 5.0 },
        dir: V { x: -1.0, y: 0.0 },
        plane: V { x: 0.0, y: 0.66 },
    };

    let mut movement_keys = [false; 4];

    // Main loop
    'running: loop {
        // Get the current time
        let start_time = Instant::now();

        // Render
        // TODO: Add a texture for the floor and ceiling
        // Draw floor and ceiling
        canvas.set_draw_color(Color::RGB(0, 0, 255));
        canvas.fill_rect(Rect::new(0, 0, PLANE_WIDTH, PLANE_HEIGHT / 2))?;
        canvas.set_draw_color(Color::RGB(0, 255, 0));
        canvas.fill_rect(Rect::new(
            0,
            (PLANE_HEIGHT / 2) as i32,
            PLANE_WIDTH,
            PLANE_HEIGHT / 2,
        ))?;

        // Render the walls
        for x in 0..PLANE_WIDTH {
            // X cord on the camera plane
            let camera_x = 2.0 * x as f64 / PLANE_WIDTH as f64 - 1.0;

            // Ray direction
            let ray = V {
                x: state.dir.x + state.plane.x * camera_x,
                y: state.dir.y + state.plane.y * camera_x,
            };

            // Position of the player on the map as an int
            let mut map_pos = V {
                x: state.pos.x as i32,
                y: state.pos.y as i32,
            };

            // Length of the ray from one x or y-side to the next x or y-side
            let delta_dist = V {
                x: (1.0 / ray.x).abs(),
                y: (1.0 / ray.y).abs(),
            };

            // Value to increment the map pos by
            let mut step = V { x: 0, y: 0 };

            // Distance to the next x or y-side
            let mut side_dist = V { x: 0.0, y: 0.0 };

            // Length of the ray from the wall hit to the camera plane
            let perp_wal_dist: f64;

            // Was a NS wall hit?
            let mut ns_side: bool;

            // Calculate the step and initial side_dist
            if ray.x < 0.0 {
                step.x = -1;
                side_dist.x = (state.pos.x - map_pos.x as f64) * delta_dist.x;
            } else {
                step.x = 1;
                side_dist.x = (map_pos.x as f64 + 1.0 - state.pos.x) * delta_dist.x;
            }

            if ray.y < 0.0 {
                step.y = -1;
                side_dist.y = (state.pos.y - map_pos.y as f64) * delta_dist.y;
            } else {
                step.y = 1;
                side_dist.y = (map_pos.y as f64 + 1.0 - state.pos.y) * delta_dist.y;
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
                if MAP[map_pos.y as usize][map_pos.x as usize] > 0 {
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
            let line_height = (PLANE_HEIGHT as f64 / perp_wal_dist) as i32;

            // Calculate the lowest and highest pixel to fill in the current stripe
            let draw_start = std::cmp::max(-line_height / 2 + (PLANE_HEIGHT / 2) as i32, 0);
            let draw_end = std::cmp::min(
                line_height / 2 + (PLANE_HEIGHT / 2) as i32,
                PLANE_HEIGHT as i32,
            );

            // Get the correct texture for the wall
            let tex_num = (MAP[map_pos.y as usize][map_pos.x as usize] - 1) as usize;

            // Calculate the value of the wall x coordinate
            let wall_x = (if ns_side {
                state.pos.x + perp_wal_dist * ray.x
            } else {
                state.pos.y + perp_wal_dist * ray.y
            })
            .fract();

            // Get the x coordinate on the texture
            let tex_x = if (ns_side && ray.y < 0.0) || (!ns_side && ray.x > 0.0) {
                TEX_SIZE - (wall_x * TEX_SIZE as f64) as u32 - 1
            } else {
                (wall_x * TEX_SIZE as f64) as u32
            };

            // Value to increment the texture coordinate by
            let tex_step = TEX_SIZE as f64 / line_height as f64;

            // Starting texture coordinate
            let mut tex_pos =
                (draw_start - (PLANE_HEIGHT / 2) as i32 + line_height / 2) as f64 * tex_step;

            // Draw the texture
            for y in draw_start..draw_end {
                // Get the correct texture pixel
                let tex_y = tex_pos as u32 & (TEX_SIZE - 1);
                tex_pos += tex_step;
                let index = ((tex_y * TEX_SIZE) as usize + (tex_x as usize)) * 3;
                let buffer = textures[tex_num as usize]
                    .without_lock()
                    .expect("Could not get surface pixels");
                let mut color = Color::RGB(buffer[index], buffer[index + 1], buffer[index + 2]);
                // Makes the y sides of the walls darker
                if ns_side {
                    color.r /= 2;
                    color.g /= 2;
                    color.b /= 2;
                }

                // Draw the pixel
                canvas.set_draw_color(color);
                canvas.draw_point(Point::new(x as i32, y))?;
            }
        }

        // Update the screen
        canvas.present();

        // TODO: Draw the fps on the screen
        let elapsed_time = start_time.elapsed().as_secs_f64();
        let mov_speed = elapsed_time * 5.0;
        let rot_speed = elapsed_time * 3.0;

        // Handle events
        for event in event_pump.poll_iter() {
            match event {
                // Detect window close
                Event::Quit { .. } => break 'running,

                // Detect key presses
                Event::KeyDown {
                    keycode: Some(keycode),
                    repeat: false,
                    ..
                } => match keycode {
                    Keycode::W => movement_keys[0] = true,
                    Keycode::S => movement_keys[1] = true,
                    Keycode::A => movement_keys[2] = true,
                    Keycode::D => movement_keys[3] = true,
                    _ => {}
                },
                Event::KeyUp {
                    keycode: Some(keycode),
                    repeat: false,
                    ..
                } => match keycode {
                    Keycode::W => movement_keys[0] = false,
                    Keycode::S => movement_keys[1] = false,
                    Keycode::A => movement_keys[2] = false,
                    Keycode::D => movement_keys[3] = false,
                    _ => {}
                },
                _ => {}
            }
        }

        // Update
        if movement_keys[0] {
            move_player(&mut state, mov_speed);
        }

        if movement_keys[1] {
            move_player(&mut state, -mov_speed);
        }

        if movement_keys[2] {
            rotate(&mut state, rot_speed)
        }

        if movement_keys[3] {
            rotate(&mut state, -rot_speed)
        }
    }
    Ok(())
}
