use crate::{
    a_star::config::CELL_SIZE,
    app::App,
    shared::{Point, get_taxicab_neighbours},
};
use macroquad::{
    input,
    prelude::*,
    rand::rand,
    ui::{
        hash, root_ui,
        widgets::{self},
    },
};
use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
};

mod config;

pub fn get_title() -> String {
    return "AStar".to_owned();
}

pub struct AStar {
    row_count: usize,
    column_count: usize,
    grid: Vec<Vec<CellType>>,
    start: Point,
    end: Point,
    can_play: bool,
    is_dragging_start: bool,
    is_dragging_end: bool,
    simulation_state: SimulationState,
}

struct SimulationState {
    cell_to_move_cost: HashMap<Point, MoveCost>,
    open_set: BinaryHeap<(Reverse<i32>, Point)>,
    closed_set: HashSet<Point>,
    active_connections: Vec<(Point, Point)>,
    cell_to_came_from: HashMap<Point, Point>,
    path: HashSet<Point>,
}

#[derive(Clone, Copy)]
struct MoveCost {
    pub estimated: i32,
    pub real: i32,
}

impl MoveCost {
    pub fn total(&self) -> i32 {
        self.real + self.estimated
    }
}

impl AStar {
    pub fn new() -> Self {
        let simulation_state = SimulationState {
            cell_to_move_cost: HashMap::new(),
            open_set: BinaryHeap::new(),
            closed_set: HashSet::new(),
            active_connections: Vec::new(),
            cell_to_came_from: HashMap::new(),
            path: HashSet::new(),
        };
        Self {
            row_count: 0,
            column_count: 0,
            grid: Vec::new(),
            start: Point::new(0, 0),
            end: Point::new(0, 0),
            can_play: false,
            is_dragging_start: false,
            is_dragging_end: false,
            simulation_state,
        }
    }

    fn initialize(&mut self) {
        self.simulation_state.cell_to_move_cost.clear();
        self.simulation_state.open_set.clear();
        self.simulation_state.closed_set.clear();
        self.simulation_state.active_connections.clear();
        self.simulation_state.cell_to_came_from.clear();
        self.simulation_state.path.clear();
        self.can_play = false;

        let start_end_diff = (self.start.x - self.end.x).abs() + (self.start.y - self.end.y).abs();
        self.simulation_state
            .open_set
            .push((Reverse(start_end_diff), self.start));
        self.simulation_state.cell_to_move_cost.insert(
            self.start,
            MoveCost {
                estimated: start_end_diff,
                real: 0,
            },
        );
    }

    fn find(&mut self) {
        while !self.simulation_state.open_set.is_empty()
            && !self.simulation_state.closed_set.contains(&self.end)
        {
            self.step();
        }
    }

    fn step(&mut self) {
        if let Some((_total, current)) = self.simulation_state.open_set.pop() {
            if current.x == self.end.x && current.y == self.end.y {
                self.simulation_state.closed_set.insert(current);
                return;
            }

            if self.simulation_state.closed_set.contains(&current) {
                return;
            }

            self.simulation_state.closed_set.insert(current);

            let current_cost = self.simulation_state.cell_to_move_cost[&current];

            let neighbours = get_taxicab_neighbours(current.x, current.y, 1);
            for neighbour in neighbours {
                if neighbour.x < 0
                    || neighbour.y < 0
                    || neighbour.x >= self.column_count as i32
                    || neighbour.y >= self.row_count as i32
                {
                    continue;
                }
                if self.grid[neighbour.y as usize][neighbour.x as usize] == CellType::Blocked {
                    continue;
                }

                let estimated = (neighbour.x - self.end.x).abs() + (neighbour.y - self.end.y).abs();
                let move_cost = MoveCost {
                    estimated,
                    real: current_cost.real + 1,
                };

                self.simulation_state
                    .active_connections
                    .push((neighbour, current));
                // final path drawing algorithm
                if let Some(parent) = self.simulation_state.cell_to_came_from.get(&neighbour) {
                    if self.simulation_state.cell_to_move_cost[&current].real
                        < self.simulation_state.cell_to_move_cost[parent].real
                    {
                        self.simulation_state
                            .cell_to_came_from
                            .insert(neighbour, current);
                    }
                } else {
                    self.simulation_state
                        .cell_to_came_from
                        .insert(neighbour, current);
                }
                // ========
                if !self
                    .simulation_state
                    .cell_to_move_cost
                    .contains_key(&neighbour)
                {
                    self.simulation_state
                        .open_set
                        .push((Reverse(move_cost.total()), neighbour));
                    self.simulation_state
                        .cell_to_move_cost
                        .insert(neighbour, move_cost);
                }
            }
        }

        self.simulation_state.path.clear();
        if let Some((_child, parent)) = self.simulation_state.active_connections.last() {
            self.simulation_state.path.insert(*parent);
            let mut current_child = *parent;
            for (new_child, new_parent) in self.simulation_state.active_connections.iter().rev() {
                if current_child == *new_child {
                    self.simulation_state.path.insert(*new_child);
                    self.simulation_state.path.insert(*parent);
                    current_child = *new_parent;
                }
            }
        }

        if let Some(tile) = self.simulation_state.cell_to_came_from.get(&self.end) {
            self.simulation_state.path.clear();
            self.simulation_state.path.insert(self.end);
            self.simulation_state.path.insert(*tile);
            let mut current_parent = *tile;
            while current_parent != self.start {
                let new_parent = self.simulation_state.cell_to_came_from[&current_parent];
                self.simulation_state.path.insert(new_parent);
                current_parent = new_parent;
            }
        }
    }
}

impl App for AStar {
    fn update(&mut self) {
        if input::is_key_pressed(KeyCode::F) {
            self.initialize();
        }
        if input::is_key_pressed(KeyCode::D) {
            self.find();
        }
        if input::is_key_down(KeyCode::A) {
            self.step();
        }
        if input::is_key_pressed(KeyCode::S) {
            self.step();
        }
        if input::is_mouse_button_pressed(MouseButton::Left) {
            let pos = input::mouse_position();
            let column = pos.0 as usize / CELL_SIZE as usize;
            let row = pos.1 as usize / CELL_SIZE as usize;
            let point = Point::new(column as i32, row as i32);
            if (0..self.grid.len()).contains(&row) && (0..self.grid[0].len()).contains(&column) {
                if point == self.start {
                    self.is_dragging_start = true;
                } else if point == self.end {
                    self.is_dragging_end = true;
                } else if self.grid[row][column] == CellType::Blocked {
                    self.grid[row][column] = CellType::Open;
                } else if self.grid[row][column] == CellType::Open {
                    self.grid[row][column] = CellType::Blocked;
                }
            }
        }
        if input::is_mouse_button_released(MouseButton::Left) {
            let pos = input::mouse_position();
            let column = pos.0 as usize / CELL_SIZE as usize;
            let row = pos.1 as usize / CELL_SIZE as usize;
            let point = Point::new(column as i32, row as i32);
            if self.is_dragging_start {
                self.is_dragging_start = false;
                self.start = point;
            } else if self.is_dragging_end {
                self.is_dragging_end = false;
                self.end = point;
            }
        }

        if self.can_play {
            self.step();
            if let Some(_) = self.simulation_state.cell_to_came_from.get(&self.end) {
                self.can_play = false;
            }
        }

        widgets::Window::new(
            hash!(),
            vec2(screen_width() / 2.0, screen_height() / 2.0),
            vec2(200.0, 140.0),
        )
        .label("Controls")
        .titlebar(true)
        .ui(&mut *root_ui(), |ui| {
            if ui.button(None, "Reset") {
                self.initialize();
            }
            ui.same_line(0.0);
            if ui.button(None, ">|") {
                self.step();
            }
            ui.same_line(0.0);
            if ui.button(None, ">>|") {
                self.initialize();
                self.find();
            }
            ui.same_line(0.0);
            if ui.button(None, ">") {
                self.initialize();
                self.can_play= true;
            }
            let mut data = "Drag and drop start and end\ncells. Click any cell to\ntoggle blocked.\nDrag this window.".to_string();
            ui.editbox(hash!(), vec2(194.0, 80.0), &mut data);
        });
    }

    fn draw(&self) {
        for row in 0..self.row_count {
            for column in 0..self.column_count {
                let x = column as f32 * CELL_SIZE;
                let y = row as f32 * CELL_SIZE;
                let point = Point::new(column as i32, row as i32);
                if point == self.start {
                    draw_rectangle(x, y, CELL_SIZE, CELL_SIZE, GREEN);
                } else if point == self.end {
                    draw_rectangle(x, y, CELL_SIZE, CELL_SIZE, RED);
                }
                if self.grid[row][column] == CellType::Blocked {
                    draw_rectangle(x, y, CELL_SIZE, CELL_SIZE, GRAY);
                }

                if self.simulation_state.cell_to_move_cost.contains_key(&point) {
                    let move_cost = &self.simulation_state.cell_to_move_cost[&point];
                    draw_rectangle_lines(x, y, CELL_SIZE, CELL_SIZE, 8.0, PINK);
                    draw_text(
                        &format!("{}", move_cost.real),
                        x + 3.0,
                        y + 13.0,
                        14.0,
                        BLACK,
                    );
                    // draw_multiline_text(
                    //     &format!(
                    //         "{},{}\n{}|{}|{}",
                    //         column,
                    //         row,
                    //         move_cost.estimated,
                    //         move_cost.real,
                    //         move_cost.total()
                    //     ),
                    //     x + 3.0,
                    //     y + 13.0,
                    //     14.0,
                    //     None,
                    //     BLACK,
                    // );
                }

                draw_rectangle_lines(x, y, CELL_SIZE, CELL_SIZE, 2.0, BLACK);
            }
        }
        for point in &self.simulation_state.path {
            let x = point.x as f32 * CELL_SIZE;
            let y = point.y as f32 * CELL_SIZE;
            draw_rectangle_lines(x, y, CELL_SIZE, CELL_SIZE, 8.0, BLUE);
        }

        if self.is_dragging_start {
            let (x, y) = input::mouse_position();
            draw_circle(x, y, 10.0, GREEN);
        } else if self.is_dragging_end {
            let (x, y) = input::mouse_position();
            draw_circle(x, y, 10.0, RED);
        }
    }

    fn resize(&mut self, width: f32, height: f32) {
        let row_count = height as i32 / CELL_SIZE as i32;
        let column_count = width as i32 / CELL_SIZE as i32;
        let mut grid: Vec<Vec<CellType>> = Vec::new();
        let start = Point::new(column_count - 1, row_count - 1);
        let end = Point::new(0, 0);
        for row in 0..row_count {
            grid.push(Vec::new());
            for column in 0..column_count {
                let cell_type = if row == start.y && column == start.x {
                    CellType::Start
                } else if row == end.y && column == end.x {
                    CellType::End
                } else {
                    let rnd = rand() % 5;
                    if rnd == 0 {
                        CellType::Blocked
                    } else {
                        CellType::Open
                    }
                };
                grid[row as usize].push(cell_type);
            }
        }
        self.row_count = row_count as usize;
        self.column_count = column_count as usize;
        self.grid = grid;
        self.start = start;
        self.end = end;
    }
}

#[derive(PartialEq)]
pub enum CellType {
    Open,
    Blocked,
    Start,
    End,
}
