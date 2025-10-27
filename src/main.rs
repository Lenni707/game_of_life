use macroquad::prelude::*;

const GRID_WIDTH: usize = 80;
const GRID_HEIGHT: usize = 60;
const CELL_SIZE: f32 = 10.0;
const UPDATE_INTERVAL: f32 = 0.1; // Zeit alle x. sekunde

#[derive(Clone, Copy, PartialEq)]
enum Cell {
    Empty,
    Filled
}

struct World {
    grid: Vec<Vec<Cell>>,
    running: bool,
    timer: f32
}

impl World {
    fn new() -> Self {
        Self {
            grid: vec![vec![Cell::Empty; GRID_WIDTH]; GRID_HEIGHT],
            running: false,
            timer: 0.0
        }
    }
    fn get(&self, x: usize, y: usize) -> Option<Cell> {
        self.grid.get(y)?.get(x).copied() // ka
    }

    fn set(&mut self, x: usize, y: usize, cell: Cell) {
        if y < GRID_HEIGHT && x < GRID_WIDTH {
            self.grid[y][x] = cell;
        }
    }

    fn is_empty(&self, x: usize, y: usize) -> bool {
        let cell = self.get(x, y);
        if cell == Some(Cell::Empty) {
            return true
        }
        return false
    }
    fn draw(&self) {
        for y in 0..GRID_HEIGHT {
            for x in 0..GRID_WIDTH {
                let cell = self.grid[y][x];
                let color =  match cell {
                    Cell::Empty => continue,
                    Cell::Filled => GREEN
                };

                draw_rectangle(
                    x as f32 * CELL_SIZE, 
                    y as f32 * CELL_SIZE, 
                    CELL_SIZE,
                    CELL_SIZE,
                    color
                );
            }
        }
    }
    fn draw_grid_lines(&self) { // macht so geile lines aber von chatty gemacht, weil ich zu faul war
        let width = GRID_WIDTH as f32 * CELL_SIZE;
        let height = GRID_HEIGHT as f32 * CELL_SIZE;
        // Vertikal
        for x in 0..=GRID_WIDTH {
            let x_pos = x as f32 * CELL_SIZE;
            draw_line(x_pos, 0.0, x_pos, height, 1.0, Color::new(0.4, 0.4, 0.4, 1.0) // bisschen heller als der hintergrund
);
        }
        // Horizontal
        for y in 0..=GRID_HEIGHT {
            let y_pos = y as f32 * CELL_SIZE;
            draw_line(0.0, y_pos, width, y_pos, 1.0, Color::new(0.4, 0.4, 0.4, 1.0)
);
        }
    }
    fn check_neighbors(grid: &Vec<Vec<Cell>>, x: usize, y: usize) -> u32 { // es muss aus dem alten grid ausgelsen werden, nicht dem neuen
        let x = x as i32;
        let y = y as i32;

        let neighbors = [ // alle möglichen felder um die cell
            (x + 1, y),
            (x - 1, y),
            (x, y + 1),
            (x, y - 1),
            (x + 1, y + 1),
            (x + 1, y - 1),
            (x - 1, y + 1),
            (x - 1, y - 1),
        ];

        let mut count = 0;
        for (nx, ny) in neighbors {
            if nx < 0 || ny < 0 || nx >= GRID_WIDTH as i32 || ny >= GRID_HEIGHT as i32 {
                continue;
            }
            if grid[ny as usize][nx as usize] == Cell::Filled {
                count += 1;
            }
        }
        count // return die anzahl der gefühlten cellen
    }
    fn update(&mut self) {
        // man muss das grid clonen weil man nicht in dem aktuellen grid gleichzeitig, durch iterieren 
        // und bearbeiten soll, damit die neuen Cells die alten nicht beeinflussen
        let old_grid = self.grid.clone();

        for y in 0..GRID_HEIGHT {
            for x in 0..GRID_WIDTH {
                let cell = old_grid[y][x];
                let neighbors = Self::check_neighbors(&old_grid, x, y);

                self.grid[y][x] = match cell {
                    Cell::Empty => {
                        if neighbors == 3 {
                            Cell::Filled
                        } else {
                            Cell::Empty
                        }
                    }
                    Cell::Filled => {
                        if neighbors < 2 || neighbors > 3 {
                            Cell::Empty
                        } else {
                            Cell::Filled
                        }
                    }
                };
            }
        }
    }

    fn left_click(&mut self) {
        let (mouse_x, mouse_y) = mouse_position();
        let grid_x = (mouse_x / CELL_SIZE) as usize;
        let grid_y = (mouse_y / CELL_SIZE) as usize;

        if grid_x >= GRID_WIDTH || grid_y >= GRID_HEIGHT {
            return;
        }

        // setzt draw mode also entwerde erease oder place
        static mut DRAW_MODE: Option<Cell> = None;

        // draw mode wird entschieden
        if is_mouse_button_pressed(MouseButton::Left) {
            unsafe {
                DRAW_MODE = if self.is_empty(grid_x, grid_y) {
                    Some(Cell::Filled)
                } else {
                    Some(Cell::Empty)
                };
            }
        }

        if is_mouse_button_down(MouseButton::Left) {
            unsafe {
                if let Some(mode) = DRAW_MODE {
                    self.set(grid_x, grid_y, mode);
                }
            }
        }

        if is_mouse_button_released(MouseButton::Left) {
            unsafe {
                DRAW_MODE = None;
            }
    }
    }

    fn space_pressed(&mut self) {
        if is_key_pressed(KeyCode::Space) {
            self.running = !self.running;
            println!("game stopped")     
        }
    }

    fn handle_keys(&mut self) { // r um das grid zurückzusetzen aber von chatty gemacht weil ich zu faul war
        if is_key_pressed(KeyCode::R) {
            for y in 0..GRID_HEIGHT {
                for x in 0..GRID_WIDTH {
                    self.grid[y][x] = Cell::Empty;
                }
            }
            self.running = false;
            self.timer = 0.0;
            println!("Grid reseted")
        }
        if is_key_pressed(KeyCode::Right) {
            self.update(); // jumps one frame each click
        }
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Game of Life".to_string(),
        high_dpi: true, // this line enables high-DPI scaling
        window_width: (GRID_WIDTH as f32 * CELL_SIZE) as i32,
        window_height: (GRID_HEIGHT as f32 * CELL_SIZE) as i32,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut game = World::new();

    loop {
        clear_background(DARKGRAY);
        
        game.space_pressed();
        game.left_click();
        game.handle_keys();
        
        if game.running {
            game.timer += get_frame_time();
            if game.timer >= UPDATE_INTERVAL { // je nach Interval x mal pro sekunde aufrufen
                game.update();
                game.timer = 0.0;
            }
        }
        
        game.draw();
        game.draw_grid_lines();

        next_frame().await
    }
}