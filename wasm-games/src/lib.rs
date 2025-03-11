use glow::HasContext;
use js_sys::{Date, Math};
use std::collections::HashSet;
use std::panic;
use wasm_bindgen::prelude::*;
use web_sys::Element;
use winit::event::{ElementState, Event, VirtualKeyCode, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::platform::web::WindowExtWebSys;
use winit::window::WindowBuilder;

// ### Constants
const CANNON_WIDTH: f32 = 0.05;
const CANNON_LENGTH: f32 = 0.1;
const CANNON_Y: f32 = -0.9;
const BALL_SIZE: f32 = 0.02;
const BALL_RADIUS: f32 = BALL_SIZE / 2.0;
const BALL_SPEED: f32 = 1.0;
const BRICK_WIDTH: f32 = 0.1;
const BRICK_HEIGHT: f32 = 0.1;
const BRICK_SPACING: f32 = 0.01;
const ROTATION_SPEED: f32 = 2.0 * std::f32::consts::PI;
const FIRE_COOLDOWN: f32 = 0.01;
const FRICTION: f32 = 0.9;
const MAX_OFFSET: f32 = 0.02; // Maximum position offset for bricks
const HIT_EFFECT_DURATION: f32 = 0.1; // Duration of hit flash in seconds
const PARTICLE_LIFETIME: f32 = 0.5; // Particle lifespan in seconds
const PARTICLE_SPEED: f32 = 0.2; // Base speed for particles
const GROUND_TOP: f32 = -0.95; // Top y-coordinate of ground
const GROUND_COLOR: [f32; 4] = [0.3, 0.2, 0.1, 1.0]; // Brown ground color
const BASE_WIDTH: f32 = 0.1; // Cannon base width
const BASE_HEIGHT: f32 = 0.05; // Cannon base height
const BASE_COLOR: [f32; 4] = [0.4, 0.4, 0.4, 1.0]; // Dark gray base
const BARREL_COLOR: [f32; 4] = [0.6, 0.6, 0.6, 1.0]; // Light gray barrel
const BALL_COLOR: [f32; 4] = [0.0, 1.0, 0.0, 1.0]; // Green bullets

// ### Structures

// Brick structure with fields for position, health, color, and hit effects
struct Brick {
    left: f32,
    bottom: f32,
    right: f32,
    top: f32,
    health: u32,
    base_color: [f32; 4], // Base color based on position
    hit_timer: f32,       // Timer for hit effect
}

// Bullet structure
struct Bullet {
    x: f32,
    y: f32,
    dx: f32,
    dy: f32,
    radius: f32,
    health: u32,
}

// Particle structure for visual effects
struct Particle {
    x: f32,
    y: f32,
    dx: f32,
    dy: f32,
    lifetime: f32,
    color: [f32; 4],
}

// HashGrid structure for efficient collision detection
struct HashGrid {
    cell_width: f32,
    cell_height: f32,
    grid: Vec<Vec<usize>>,
    grid_width: usize,
    grid_height: usize,
}

impl HashGrid {
    fn new(cell_width: f32, cell_height: f32, grid_width: usize, grid_height: usize) -> Self {
        HashGrid {
            cell_width,
            cell_height,
            grid: vec![vec![]; grid_width * grid_height],
            grid_width,
            grid_height,
        }
    }

    fn get_cell_index(&self, x: f32, y: f32) -> (i32, i32) {
        let i_x = ((x + 1.0) / self.cell_width).floor() as i32;
        let i_y = ((y + 1.0) / self.cell_height).floor() as i32;
        (i_x, i_y)
    }

    fn add_brick(&mut self, brick_index: usize, left: f32, bottom: f32, right: f32, top: f32) {
        let (i_x_min, i_y_min) = self.get_cell_index(left, bottom);
        let (i_x_max, i_y_max) = self.get_cell_index(right, top);
        for i_x in i_x_min..=i_x_max {
            for i_y in i_y_min..=i_y_max {
                if i_x >= 0
                    && i_x < self.grid_width as i32
                    && i_y >= 0
                    && i_y < self.grid_height as i32
                {
                    let index = (i_x as usize) * self.grid_height + (i_y as usize);
                    self.grid[index].push(brick_index);
                }
            }
        }
    }

    fn get_bricks_in_cell(&self, i_x: i32, i_y: i32) -> &[usize] {
        if i_x >= 0 && i_x < self.grid_width as i32 && i_y >= 0 && i_y < self.grid_height as i32 {
            let index = (i_x as usize) * self.grid_height + (i_y as usize);
            &self.grid[index]
        } else {
            &[]
        }
    }
}

// ### Weapon Trait and Implementation

trait Weapon {
    fn update(&mut self, delta_time: f32);
    fn try_fire(&mut self, cannon_position: (f32, f32), cannon_angle: f32) -> Option<Bullet>;
}

struct SimpleCannon {
    cooldown_timer: f32,
    cooldown_min: f32,
    cooldown_max: f32,
    angle_spread: f32,
    speed_min: f32,
    speed_max: f32,
    bullet_radius: f32,
}

impl SimpleCannon {
    fn new() -> Self {
        SimpleCannon {
            cooldown_timer: 0.0,
            cooldown_min: 0.005,
            cooldown_max: 0.015,
            angle_spread: 0.1,
            speed_min: 0.8 * BALL_SPEED,
            speed_max: 1.2 * BALL_SPEED,
            bullet_radius: BALL_RADIUS,
        }
    }
}

impl Weapon for SimpleCannon {
    fn update(&mut self, delta_time: f32) {
        self.cooldown_timer = (self.cooldown_timer - delta_time).max(0.0);
    }

    fn try_fire(&mut self, cannon_position: (f32, f32), cannon_angle: f32) -> Option<Bullet> {
        if self.cooldown_timer > 0.0 {
            return None;
        }

        let (cannon_x, cannon_y) = cannon_position;
        let random_angle_offset = (Math::random() as f32 * 2.0 - 1.0) * self.angle_spread;
        let firing_angle = cannon_angle + random_angle_offset;
        let random_speed =
            self.speed_min + Math::random() as f32 * (self.speed_max - self.speed_min);
        let dx = -random_speed * firing_angle.sin();
        let dy = random_speed * firing_angle.cos();

        let tip_x = cannon_x + CANNON_LENGTH * firing_angle.sin();
        let tip_y = cannon_y + CANNON_LENGTH * firing_angle.cos();
        let bullet_x = -tip_x;
        let bullet_y = tip_y;

        self.cooldown_timer =
            self.cooldown_min + Math::random() as f32 * (self.cooldown_max - self.cooldown_min);

        Some(Bullet {
            x: bullet_x,
            y: bullet_y,
            dx,
            dy,
            radius: self.bullet_radius,
            health: 1,
        })
    }
}

// ### Game State

struct GameState {
    cannon_x: f32,
    cannon_y: f32,
    theta: f32,
    bullets: Vec<Bullet>,
    bricks: Vec<Brick>,
    particles: Vec<Particle>,
    left_pressed: bool,
    right_pressed: bool,
    fire_pressed: bool,
    weapon: SimpleCannon,
    hash_grid: HashGrid,
}

impl GameState {
    fn new() -> Self {
        let mut bricks = Vec::new();
        let num_rows = 10;
        let num_cols = 10;
        let span_x = (num_cols - 1) as f32 * (BRICK_WIDTH + BRICK_SPACING) + BRICK_WIDTH;
        let starting_x = -span_x / 2.0;
        let starting_y =
            1.0 - BRICK_HEIGHT - (num_rows - 1) as f32 * (BRICK_HEIGHT + BRICK_SPACING);

        for i in 0..num_rows {
            for j in 0..num_cols {
                let offset_x = (Math::random() as f32 * 2.0 - 1.0) * MAX_OFFSET;
                let offset_y = (Math::random() as f32 * 2.0 - 1.0) * MAX_OFFSET;
                let x = starting_x + (j as f32) * (BRICK_WIDTH + BRICK_SPACING) + offset_x;
                let y = starting_y + (i as f32) * (BRICK_HEIGHT + BRICK_SPACING) + offset_y;
                let hue = ((x + 1.0) / 2.0) * 360.0; // Map x from [-1,1] to [0,360]
                let color = hsv_to_rgb(hue, 0.7, 1.0);
                bricks.push(Brick {
                    left: x,
                    bottom: y,
                    right: x + BRICK_WIDTH,
                    top: y + BRICK_HEIGHT,
                    health: 5,
                    base_color: [color[0], color[1], color[2], 1.0],
                    hit_timer: 0.0,
                });
            }
        }

        let grid_width = (2.0 / BRICK_WIDTH).ceil() as usize;
        let grid_height = (2.0 / BRICK_HEIGHT).ceil() as usize;
        let mut hash_grid = HashGrid::new(BRICK_WIDTH, BRICK_HEIGHT, grid_width, grid_height);

        for (index, brick) in bricks.iter().enumerate() {
            hash_grid.add_brick(index, brick.left, brick.bottom, brick.right, brick.top);
        }

        GameState {
            cannon_x: 0.0,
            cannon_y: CANNON_Y,
            theta: 0.0,
            bullets: Vec::new(),
            bricks,
            particles: Vec::new(),
            left_pressed: false,
            right_pressed: false,
            fire_pressed: false,
            weapon: SimpleCannon::new(),
            hash_grid,
        }
    }

    fn update(&mut self, delta_time: f32) {
        // Update cannon rotation
        if self.left_pressed {
            self.theta += ROTATION_SPEED * delta_time;
        }
        if self.right_pressed {
            self.theta -= ROTATION_SPEED * delta_time;
        }
        self.theta = self
            .theta
            .clamp(-std::f32::consts::PI / 2.0, std::f32::consts::PI / 2.0);

        // Update weapon and fire bullets
        self.weapon.update(delta_time);
        if self.fire_pressed {
            if let Some(bullet) = self
                .weapon
                .try_fire((self.cannon_x, self.cannon_y), self.theta)
            {
                self.bullets.push(bullet);
                let tip_x = self.cannon_x + CANNON_LENGTH * self.theta.sin();
                let tip_y = self.cannon_y + CANNON_LENGTH * self.theta.cos();
                self.spawn_particles(tip_x, tip_y, 5, [1.0, 0.5, 0.0, 1.0]); // Orange muzzle flash
            }
        }

        let mut to_spawn = Vec::new();

        // Update bullets and check collisions
        for bullet in &mut self.bullets {
            let friction_factor = FRICTION.powf(delta_time);
            bullet.dx *= friction_factor;
            bullet.dy *= friction_factor;

            bullet.x += bullet.dx * delta_time;
            bullet.y += bullet.dy * delta_time;

            if bullet.x - bullet.radius < -1.0 {
                bullet.x = -1.0 + bullet.radius;
                bullet.dx = -bullet.dx;
            }
            if bullet.x + bullet.radius > 1.0 {
                bullet.x = 1.0 - bullet.radius;
                bullet.dx = -bullet.dx;
            }
            if bullet.y + bullet.radius > 1.0 {
                bullet.y = 1.0 - bullet.radius;
                bullet.dy = -bullet.dy;
            }

            let bullet_left = bullet.x - bullet.radius;
            let bullet_right = bullet.x + bullet.radius;
            let bullet_bottom = bullet.y - bullet.radius;
            let bullet_top = bullet.y + bullet.radius;

            let (i_x_min, i_y_min) = self.hash_grid.get_cell_index(bullet_left, bullet_bottom);
            let (i_x_max, i_y_max) = self.hash_grid.get_cell_index(bullet_right, bullet_top);

            let mut bricks_to_check = HashSet::new();
            for i_x in i_x_min..=i_x_max {
                for i_y in i_y_min..=i_y_max {
                    let bricks_in_cell = self.hash_grid.get_bricks_in_cell(i_x, i_y);
                    for &brick_index in bricks_in_cell {
                        bricks_to_check.insert(brick_index);
                    }
                }
            }

            for brick_index in bricks_to_check {
                let brick = &mut self.bricks[brick_index];
                if brick.health > 0
                    && bullet_right > brick.left
                    && bullet_left < brick.right
                    && bullet_top > brick.bottom
                    && bullet_bottom < brick.top
                {
                    brick.health -= 1;
                    bullet.health -= 1;
                    bullet.dy = -bullet.dy;
                    brick.hit_timer = HIT_EFFECT_DURATION;
                    to_spawn.push((bullet.x, bullet.y, 3, brick.base_color));
                    if brick.health == 0 {
                        let brick_center_x = (brick.left + brick.right) / 2.0;
                        let brick_center_y = (brick.bottom + brick.top) / 2.0;
                        to_spawn.push((brick_center_x, brick_center_y, 10, brick.base_color));
                    }
                }
            }
        }

        for spawn in to_spawn {
            self.spawn_particles(spawn.0, spawn.1, spawn.2, spawn.3);
        }

        self.bullets
            .retain(|bullet| bullet.health > 0 && bullet.y - bullet.radius >= -1.0);

        // Update brick hit timers
        for brick in &mut self.bricks {
            if brick.hit_timer > 0.0 {
                brick.hit_timer -= delta_time;
                if brick.hit_timer < 0.0 {
                    brick.hit_timer = 0.0;
                }
            }
        }

        // Update particles
        for particle in &mut self.particles {
            particle.x += particle.dx * delta_time;
            particle.y += particle.dy * delta_time;
            particle.lifetime -= delta_time;
        }
        self.particles.retain(|p| p.lifetime > 0.0);
    }

    fn spawn_particles(&mut self, x: f32, y: f32, count: usize, color: [f32; 4]) {
        for _ in 0..count {
            let angle = Math::random() as f32 * 2.0 * std::f32::consts::PI;
            let speed = Math::random() as f32 * PARTICLE_SPEED;
            let dx = speed * angle.cos();
            let dy = speed * angle.sin();
            let lifetime = PARTICLE_LIFETIME;
            self.particles.push(Particle {
                x,
                y,
                dx,
                dy,
                lifetime,
                color,
            });
        }
    }
}

// ### Utility Functions

// Convert HSV to RGB for brick colors
fn hsv_to_rgb(h: f32, s: f32, v: f32) -> [f32; 3] {
    let c = v * s;
    let h_prime = (h % 360.0) / 60.0;
    let x = c * (1.0 - (h_prime % 2.0 - 1.0).abs());
    let (r1, g1, b1) = if h_prime < 1.0 {
        (c, x, 0.0)
    } else if h_prime < 2.0 {
        (x, c, 0.0)
    } else if h_prime < 3.0 {
        (0.0, c, x)
    } else if h_prime < 4.0 {
        (0.0, x, c)
    } else if h_prime < 5.0 {
        (x, 0.0, c)
    } else {
        (c, 0.0, x)
    };
    let m = v - c;
    [r1 + m, g1 + m, b1 + m]
}

// ### Main Rendering Function

pub fn main_with_container(container: Element) {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Brick Breaker with Cannon")
        .build(&event_loop)
        .unwrap();
    let canvas = window.canvas();
    container
        .append_child(&canvas)
        .expect("Append canvas to container");

    let webgl2_context = canvas
        .get_context("webgl2")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::WebGl2RenderingContext>()
        .unwrap();
    let gl = glow::Context::from_webgl2_context(webgl2_context);

    // Enable blending for particle transparency
    unsafe {
        gl.enable(glow::BLEND);
        gl.blend_func(glow::SRC_ALPHA, glow::ONE_MINUS_SRC_ALPHA);
    }

    let program = unsafe {
        let vertex_array = gl
            .create_vertex_array()
            .expect("Cannot create vertex array");
        gl.bind_vertex_array(Some(vertex_array));

        let program = gl.create_program().expect("Cannot create program");
        let vertex_shader_source = "#version 300 es\n
        in vec2 position;\n
        uniform mat4 model;\n
        void main() {\n
            gl_Position = model * vec4(position, 0.0, 1.0);\n
        }\n";
        let fragment_shader_source = "#version 300 es\n
        precision mediump float;\n
        uniform vec4 color;\n
        out vec4 fragColor;\n
        void main() {\n
            fragColor = color;\n
        }\n";

        let shader_sources = [
            (glow::VERTEX_SHADER, vertex_shader_source),
            (glow::FRAGMENT_SHADER, fragment_shader_source),
        ];

        let mut shaders = Vec::new();
        for (shader_type, source) in shader_sources.iter() {
            let shader = gl
                .create_shader(*shader_type)
                .expect("Cannot create shader");
            gl.shader_source(shader, source);
            gl.compile_shader(shader);
            if !gl.get_shader_compile_status(shader) {
                panic!("{}", gl.get_shader_info_log(shader));
            }
            gl.attach_shader(program, shader);
            shaders.push(shader);
        }

        gl.link_program(program);
        if !gl.get_program_link_status(program) {
            panic!("{}", gl.get_program_info_log(program));
        }

        for shader in shaders {
            gl.detach_shader(program, shader);
            gl.delete_shader(shader);
        }

        program
    };

    let pos_attrib = unsafe { gl.get_attrib_location(program, "position").unwrap() };
    let color_location = unsafe { gl.get_uniform_location(program, "color").unwrap() };
    let model_location = unsafe { gl.get_uniform_location(program, "model").unwrap() };
    let mut game_state = GameState::new();
    let mut last_time = Date::now();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                WindowEvent::KeyboardInput { input, .. } => {
                    if let Some(keycode) = input.virtual_keycode {
                        match keycode {
                            VirtualKeyCode::Left => {
                                game_state.left_pressed = input.state == ElementState::Pressed;
                            }
                            VirtualKeyCode::Right => {
                                game_state.right_pressed = input.state == ElementState::Pressed;
                            }
                            VirtualKeyCode::Space => {
                                game_state.fire_pressed = input.state == ElementState::Pressed;
                            }
                            _ => {}
                        }
                    }
                }
                _ => {}
            },
            Event::MainEventsCleared => {
                let current_time = Date::now();
                let delta_time = (current_time - last_time) / 1000.0;
                last_time = current_time;
                game_state.update(delta_time as f32);
                window.request_redraw();
            }
            Event::RedrawRequested(_) => unsafe {
                gl.clear_color(0.1, 0.2, 0.3, 1.0);
                gl.clear(glow::COLOR_BUFFER_BIT);
                gl.use_program(Some(program));

                // Draw background
                let model_matrix = [
                    1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
                ];
                gl.uniform_matrix_4_f32_slice(Some(&model_location), false, &model_matrix);
                draw_rectangle(
                    &gl,
                    pos_attrib,
                    &color_location,
                    -1.0,
                    -1.0,
                    1.0,
                    1.0,
                    [0.2, 0.3, 0.5, 1.0], // Light blue-ish background
                );

                // Draw ground
                draw_ground(&gl, pos_attrib, &color_location);

                // Draw bricks
                for brick in &game_state.bricks {
                    if brick.health > 0 {
                        let health_factor = brick.health as f32 / 5.0;
                        let color = if brick.hit_timer > 0.0 {
                            [1.0, 1.0, 1.0, 1.0] // White flash
                        } else {
                            [
                                brick.base_color[0] * health_factor,
                                brick.base_color[1] * health_factor,
                                brick.base_color[2] * health_factor,
                                1.0,
                            ]
                        };
                        let model_matrix = create_translation_matrix(brick.left, brick.bottom);
                        gl.uniform_matrix_4_f32_slice(Some(&model_location), false, &model_matrix);
                        draw_rectangle(
                            &gl,
                            pos_attrib,
                            &color_location,
                            0.0,
                            0.0,
                            BRICK_WIDTH,
                            BRICK_HEIGHT,
                            color,
                        );
                    }
                }

                // Draw cannon base
                let model_matrix =
                    create_translation_matrix(game_state.cannon_x, game_state.cannon_y);
                gl.uniform_matrix_4_f32_slice(Some(&model_location), false, &model_matrix);
                draw_rectangle(
                    &gl,
                    pos_attrib,
                    &color_location,
                    -BASE_WIDTH / 2.0,
                    -BASE_HEIGHT / 2.0,
                    BASE_WIDTH / 2.0,
                    BASE_HEIGHT / 2.0,
                    BASE_COLOR,
                );

                // Draw cannon barrel
                let model_matrix =
                    create_model_matrix(game_state.cannon_x, game_state.cannon_y, game_state.theta);
                gl.uniform_matrix_4_f32_slice(Some(&model_location), false, &model_matrix);
                draw_rectangle(
                    &gl,
                    pos_attrib,
                    &color_location,
                    -CANNON_WIDTH / 2.0,
                    0.0,
                    CANNON_WIDTH / 2.0,
                    CANNON_LENGTH,
                    BARREL_COLOR,
                );

                // Draw bullets
                for bullet in &game_state.bullets {
                    let model_matrix = create_translation_matrix(bullet.x, bullet.y);
                    gl.uniform_matrix_4_f32_slice(Some(&model_location), false, &model_matrix);
                    draw_rectangle(
                        &gl,
                        pos_attrib,
                        &color_location,
                        -bullet.radius,
                        -bullet.radius,
                        bullet.radius,
                        bullet.radius,
                        BALL_COLOR,
                    );
                }

                // Draw particles
                for particle in &game_state.particles {
                    let alpha = particle.lifetime / PARTICLE_LIFETIME;
                    let color = [
                        particle.color[0],
                        particle.color[1],
                        particle.color[2],
                        alpha,
                    ];
                    let model_matrix = create_translation_matrix(particle.x, particle.y);
                    gl.uniform_matrix_4_f32_slice(Some(&model_location), false, &model_matrix);
                    draw_rectangle(
                        &gl,
                        pos_attrib,
                        &color_location,
                        -0.005,
                        -0.005,
                        0.005,
                        0.005,
                        color,
                    );
                }
            },
            _ => {}
        }
    });
}

// ### Helper Functions

fn draw_rectangle(
    gl: &glow::Context,
    pos_attrib: u32,
    color_location: &glow::UniformLocation,
    left: f32,
    bottom: f32,
    right: f32,
    top: f32,
    color: [f32; 4],
) {
    let vertices = [left, bottom, right, bottom, left, top, right, top];
    let indices = [0u32, 1, 2, 2, 1, 3];

    unsafe {
        let vbo = gl.create_buffer().expect("Cannot create vertex buffer");
        gl.bind_buffer(glow::ARRAY_BUFFER, Some(vbo));
        gl.buffer_data_u8_slice(
            glow::ARRAY_BUFFER,
            &vertices
                .iter()
                .flat_map(|f| f.to_ne_bytes())
                .collect::<Vec<u8>>(),
            glow::STATIC_DRAW,
        );

        let ibo = gl.create_buffer().expect("Cannot create index buffer");
        gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, Some(ibo));
        gl.buffer_data_u8_slice(
            glow::ELEMENT_ARRAY_BUFFER,
            &indices
                .iter()
                .flat_map(|i| i.to_ne_bytes())
                .collect::<Vec<u8>>(),
            glow::STATIC_DRAW,
        );

        gl.vertex_attrib_pointer_f32(pos_attrib, 2, glow::FLOAT, false, 0, 0);
        gl.enable_vertex_attrib_array(pos_attrib);
        gl.uniform_4_f32(Some(color_location), color[0], color[1], color[2], color[3]);
        gl.draw_elements(glow::TRIANGLES, 6, glow::UNSIGNED_INT, 0);

        gl.delete_buffer(vbo);
        gl.delete_buffer(ibo);
    }
}

fn create_translation_matrix(tx: f32, ty: f32) -> [f32; 16] {
    [
        1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, tx, ty, 0.0, 1.0,
    ]
}

fn create_model_matrix(tx: f32, ty: f32, theta: f32) -> [f32; 16] {
    let cos_theta = theta.cos();
    let sin_theta = theta.sin();
    [
        cos_theta, sin_theta, 0.0, 0.0, -sin_theta, cos_theta, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, tx,
        ty, 0.0, 1.0,
    ]
}

fn draw_ground(gl: &glow::Context, pos_attrib: u32, color_location: &glow::UniformLocation) {
    let vertices = [
        -1.0,
        -1.0, // Bottom-left
        1.0,
        -1.0, // Bottom-right
        1.0,
        GROUND_TOP + 0.02, // Top-right with slight variation
        -1.0,
        GROUND_TOP - 0.02, // Top-left with slight variation
    ];
    let indices = [0u32, 1, 2, 2, 3, 0];

    unsafe {
        let vbo = gl.create_buffer().expect("Cannot create vertex buffer");
        gl.bind_buffer(glow::ARRAY_BUFFER, Some(vbo));
        gl.buffer_data_u8_slice(
            glow::ARRAY_BUFFER,
            &vertices
                .iter()
                .flat_map(|f| f.to_ne_bytes())
                .collect::<Vec<u8>>(),
            glow::STATIC_DRAW,
        );

        let ibo = gl.create_buffer().expect("Cannot create index buffer");
        gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, Some(ibo));
        gl.buffer_data_u8_slice(
            glow::ELEMENT_ARRAY_BUFFER,
            &indices
                .iter()
                .flat_map(|i| i.to_ne_bytes())
                .collect::<Vec<u8>>(),
            glow::STATIC_DRAW,
        );

        gl.vertex_attrib_pointer_f32(pos_attrib, 2, glow::FLOAT, false, 0, 0);
        gl.enable_vertex_attrib_array(pos_attrib);
        gl.uniform_4_f32(
            Some(color_location),
            GROUND_COLOR[0],
            GROUND_COLOR[1],
            GROUND_COLOR[2],
            GROUND_COLOR[3],
        );
        gl.draw_elements(glow::TRIANGLES, 6, glow::UNSIGNED_INT, 0);

        gl.delete_buffer(vbo);
        gl.delete_buffer(ibo);
    }
}

// ### WebAssembly Bindings

#[wasm_bindgen]
pub fn initialize(container: Element) {
    main_with_container(container);
}

#[wasm_bindgen(start)]
pub fn run() {
    console_log::init_with_level(log::Level::Debug).unwrap();
    panic::set_hook(Box::new(|info| {
        log::error!("Panicked: {}", info);
    }));
}
