use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Point;
use std::time::{Duration, Instant};
use std::ops;


// Screen size
const SCREEN_WIDTH: u32 = 640;
const SCREEN_HEIGHT: u32 = 480;
const TEX_SIZE: u32 = 64;

// Speed and rotation speed
const MOV_SPEED: f64 = 0.1;
const ROT_SPEED: f64 = 0.08;

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
  [1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1],
  [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
  [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
  [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
  [1,0,0,0,0,0,2,2,2,2,2,0,0,0,0,3,0,3,0,3,0,0,0,1],
  [1,0,0,0,0,0,2,0,0,0,2,0,0,0,0,0,0,0,0,0,0,0,0,1],
  [1,0,0,0,0,0,2,0,0,0,2,0,0,0,0,3,0,0,0,3,0,0,0,1],
  [1,0,0,0,0,0,2,0,0,0,2,0,0,0,0,0,0,0,0,0,0,0,0,1],
  [1,0,0,0,0,0,2,2,0,2,2,0,0,0,0,3,0,3,0,3,0,0,0,1],
  [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
  [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
  [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
  [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
  [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
  [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
  [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
  [1,4,4,4,4,4,4,4,4,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
  [1,4,0,4,0,0,0,0,4,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
  [1,4,0,0,0,0,5,0,4,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
  [1,4,0,4,0,0,0,0,4,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
  [1,4,0,4,4,4,4,4,4,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
  [1,4,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
  [1,4,4,4,4,4,4,4,4,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
  [1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1]
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

fn convert_color(color_value: i32) -> Color {
    let r = (color_value >> 16) & 0xFF;
    let g = (color_value >> 8) & 0xFF;
    let b = color_value & 0xFF;

    Color::RGB(r as u8, g as u8, b as u8)
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
    // Generate the textures
    let mut texture = [[0; (TEX_SIZE * TEX_SIZE) as usize]; 8];
    for x in 0..TEX_SIZE {
        for y in 0..TEX_SIZE {
            let xorcolor = ((x * 256 / TEX_SIZE) ^ (y * 256 / TEX_SIZE)) as i32;
            let ycolor = (y * 256 / TEX_SIZE ) as i32;
            let xycolor = (y * 128 / TEX_SIZE + x * 128 / TEX_SIZE) as i32;
            let p = (TEX_SIZE * y + x) as usize;
            texture[0][p] = 65536 * 254 * (x != y && x != TEX_SIZE - y) as i32; //flat red texture with black cross
            texture[1][p] = xycolor + 256 * xycolor + 65536 * xycolor; //sloped greyscale
            texture[2][p] = 256 * xycolor + 65536 * xycolor; //sloped yellow gradient
            texture[3][p] = xorcolor + 256 * xorcolor + 65536 * xorcolor; //xor greyscale
            texture[4][p] = 256 * xorcolor; //xor green
            texture[5][p] = 65536 * 192 * ((x % 16 != 0) && (y % 16 != 0)) as i32; //red bricks
            texture[6][p] = 65536 * ycolor; //red gradient
            texture[7][p] = 128 + 256 * 128 + 65536 * 128; //flat grey texture
        }
    }

    // Swap the x and y coordinates of the texture to optimize the rendering
    for i in 0..8{
        for x in 0..TEX_SIZE {
            for y in 0..x {
                texture[i].swap((TEX_SIZE * y + x) as usize, (TEX_SIZE * x + y) as usize);
            }
        }
    }

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
        // Clear the screen to black
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        // Render the walls
        for x in 0..SCREEN_WIDTH {
            // X cord on the camera plane
            let camera_x = 2.0 * x as f64 / SCREEN_WIDTH as f64 - 1.0;

            // Ray direction
            let ray = V{x:state.dir.x + state.plane.x * camera_x, y: state.dir.y + state.plane.y * camera_x};

            // Position of the player on the map as an int
            let mut map_pos = V{x:state.pos.x as i32, y: state.pos.y as i32};

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

            // Calculate the step and initial side_dist
            if ray.x < 0.0 {
                step.x = -1;
                side_dist.x = (state.pos.x - map_pos.x as f64) * delta_dist.x;
            } else {
                step.x = 1;
                side_dist.x = (map_pos.x as f64 + 1.0 -state.pos.x) * delta_dist.x;
            }

            if ray.y < 0.0 {
                step.y = -1;
                side_dist.y = (state.pos.y - map_pos.y as f64) * delta_dist.y;
            } else {
                step.y = 1;
                side_dist.y = (map_pos.y as f64 + 1.0 -state.pos.y) * delta_dist.y;
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
                if MAP[map_pos.y as usize][map_pos.x as usize] > 0 {break;}
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

            // Get the correct texture for the wall
            let tex_num = MAP[map_pos.y as usize][map_pos.x as usize] - 1;

            // Calculate the value of the wall x coordinate
            let mut wall_x: f64;
            if ns_side {
                wall_x =state.pos.x + perp_wal_dist * ray.x;
            } else {
                wall_x =state.pos.y + perp_wal_dist * ray.y;
            }
            wall_x -= wall_x.floor();

            // Get the x coordinate on the texture
            let mut tex_x = (wall_x * TEX_SIZE as f64) as u32;
            if ns_side && ray.y < 0.0{
                tex_x = TEX_SIZE - tex_x - 1;
            }
            if !ns_side && ray.x > 0.0{
                tex_x = TEX_SIZE - tex_x - 1;
            }

            let tex_step = TEX_SIZE as f64 / line_height as f64;
            
            // Starting texture coordinate
            let mut tex_pos = (draw_start.y - (SCREEN_HEIGHT / 2) as i32 + line_height / 2) as f64 * tex_step;

            for y in draw_start.y..draw_end.y {
                // Get the correct texture pixel
                let tex_y = tex_pos as u32 & (TEX_SIZE - 1);
                tex_pos += tex_step;
                let mut color = texture[tex_num as usize][(TEX_SIZE * tex_x + tex_y) as usize];
                if ns_side {color = (color >> 1) & 8355711} // Make y sides darker

                // Draw the pixel
                canvas.set_draw_color(convert_color(color));
                canvas.draw_point(Point::new(x as i32, y))?;
            }
        }

        // Update the screen
        canvas.present();

        //TODO: Fix timing
        // Sleep to maintain framerate of 60fps
        let elapsed_time = start_time.elapsed();
        if elapsed_time < Duration::from_millis(16) {
            std::thread::sleep(Duration::from_millis(16) - elapsed_time);
        }

        // Print the FPS
        let fps = 1.0 / start_time.elapsed().as_secs_f64();
        println!("FPS: {}", fps);
    }

    Ok(())
}