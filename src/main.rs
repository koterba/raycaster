use raylib::prelude::*;

const WINDOW_HEIGHT: i32 = 500;
const WINDOW_WIDTH: i32 = WINDOW_HEIGHT * 2;

// const WINDOW_HEIGHT: i32 = 1080;
// const WINDOW_WIDTH: i32 = 1920;

const FOV: f64 = std::f64::consts::PI / 3.0;
const HALF_FOV: f64 = FOV / 2.0; 

const TILES: i32 = 10;
const TILE_SIZE: f64 = (WINDOW_HEIGHT / TILES) as f64;

const CASTED_RAYS: i32 = 250;
const RAY_DISTANCE: f64 = 10.0 * TILE_SIZE;
const ANGLE_STEP: f64 = FOV / (CASTED_RAYS as f64);
const SCALE: f64 =  (WINDOW_HEIGHT as f64) / (CASTED_RAYS as f64);

const WALL: i32 = 1;

const MAP: [[i32; 10]; 10] = [
    [1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 1, 1, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 1, 0, 1],
    [1, 0, 0, 0, 0, 0, 1, 1, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 1, 0, 1],
    [1, 0, 0, 0, 1, 1, 1, 1, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
];

struct Window {
    rl: RaylibHandle,
    thread: RaylibThread,
    prev_mouse_pos: i32
}

struct Player {
    x: f64,
    y: f64,
    angle: f64
}

impl Window {
    fn new() -> Self {
        let (mut rl, thread) = init()
            .size(WINDOW_WIDTH, WINDOW_HEIGHT)
            .title("ANOTHER RAYLIB RAY CASTER")
            .build();

        rl.set_target_fps(60);
        rl.disable_cursor();

        let prev_mouse_pos = rl.get_mouse_x();

        Window { rl, thread, prev_mouse_pos }
    }

    fn draw(&mut self, player: &Player) {
        let mut d = self.rl.begin_drawing(&self.thread);
        d.clear_background(Color::BLACK);

        // draw the board
        for (ind_row, row) in MAP.iter().enumerate() {
            for (ind_elm, element) in row.iter().enumerate() {
                let y = ((ind_row as i32) * (TILE_SIZE as i32));
                let x = ((ind_elm as i32) * (TILE_SIZE as i32));
                let color = if element == &1 { Color::new(50, 50, 50,  255) } else { Color::new(40, 20, 20,  255) };
                d.draw_rectangle(
                    x,
                    y,
                    (TILE_SIZE as i32),
                    (TILE_SIZE as i32),
                    color)
                ;
            }
        }

        //draw "sky"
        d.draw_rectangle(
            WINDOW_WIDTH / 2,
            0,
            WINDOW_WIDTH / 2,
            WINDOW_HEIGHT / 2,
            Color::new(80, 130, 200,  255)
        );

        //draw "floor"
        d.draw_rectangle(
            WINDOW_WIDTH / 2,
            WINDOW_HEIGHT / 2,
            WINDOW_WIDTH / 2,
            WINDOW_HEIGHT / 2,
            Color::new(40, 20, 20,  255)
        );

        // draw rays
        let mut start_angle = player.angle - HALF_FOV;

        for ray in 0..CASTED_RAYS {
            for (mut depth) in 0..(RAY_DISTANCE as i32) {
                let target_x = player.x - (start_angle.sin() * (depth as f64));
                let target_y = player.y + (start_angle.cos() * (depth as f64));
                let mut col = (target_x / TILE_SIZE) as usize;
                let mut row = (target_y / TILE_SIZE) as usize;

                if row > (TILES - 1) as usize { row = (TILES - 1) as usize }
                if col > (TILES - 1) as usize { col = (TILES - 1) as usize }

                if MAP[row][col] == WALL {
                    let y = ((row as i32) * (TILE_SIZE as i32)) + 10;
                    let x = ((col as i32) * (TILE_SIZE as i32)) + 10;
                    //d.draw_rectangle_gradient_h(x, y, (TILE_SIZE as i32) - 15, (TILE_SIZE as i32) - 15, Color::GREEN, Color::PURPLE);
                    d.draw_rectangle(x, y, (TILE_SIZE as i32) - 20, (TILE_SIZE as i32) - 20, Color::WHITE);
                    d.draw_line(player.x as i32, player.y as i32, target_x as i32, target_y as i32, Color::WHITE);

                    let color = (150.0 / (1.0 + (depth as f64) * (depth as f64) * 0.0001)) as u8;

                    let depth = (depth as f64) * (player.angle - start_angle).cos();
                    let mut wall_height = 21000.0 / (depth + 0.0001);
                    // if wall is higher than screen height then make it screen height
                    if wall_height > WINDOW_HEIGHT as f64 { wall_height = WINDOW_HEIGHT as f64 }

                    d.draw_rectangle(((WINDOW_WIDTH / 2) as f64 + (ray as f64) * SCALE) as i32, (WINDOW_HEIGHT) / 2 - (wall_height / 2.0 ) as i32, SCALE as i32, wall_height as i32, Color::new(color, color, color,  255));

                    break;
                }
            }

            start_angle += ANGLE_STEP;
        }

        // draw the player
        let player_x = player.x as i32;
        let player_y = player.y as i32;
        d.draw_circle(player_x+3, player_y+3, 10.0, Color::BLACK);
        d.draw_circle(player_x, player_y, 10.0, Color::new(80, 130, 200,  255));

    }
}


impl Player {
    fn new() -> Self {
        Player {
            x: 200.0,
            y: 200.0,
            angle: 0.0
        }
    }

    fn update(&mut self, window: &mut Window) {
        // up && down / y/x
        if window.rl.is_key_down(raylib::consts::KeyboardKey::KEY_S) {
            self.x -= -(self.angle.sin() * 3.0);
            self.y -= self.angle.cos() * 3.0;
        } else if window.rl.is_key_down(raylib::consts::KeyboardKey::KEY_W) {
            self.x += -(self.angle.sin() * 3.0);
            self.y += self.angle.cos() * 3.0;
        }

        // left && right / change angle
        if window.rl.is_key_down(raylib::consts::KeyboardKey::KEY_A) {
            self.x -= -(self.angle - 4.8).sin() * 3.0;
            self.y -= (self.angle - 4.8).cos() * 3.0;
        } else if window.rl.is_key_down(raylib::consts::KeyboardKey::KEY_D) {
            self.x += -(self.angle - 4.8).sin() * 3.0;
            self.y += (self.angle - 4.8).cos() * 3.0;
        }

        let mouse_x = window.rl.get_mouse_x();

        self.angle = self.angle + ((mouse_x - window.prev_mouse_pos) as f64) / 200.0;
        window.prev_mouse_pos = mouse_x;
    }
}

fn main() {
    let mut window = Window::new();
    let mut player = Player::new();
    
    while !window.rl.window_should_close() {
        player.update(&mut window);
        window.draw(&player);
    }
}
