use crate::{
    a_star::{cell::Cell, config::CELL_SIZE},
    app::App,
    shared::{Point, get_taxicab_neighbours},
};
use macroquad::{input, prelude::*, rand::rand};
use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
};

mod cell;
mod config;

pub fn get_title() -> String {
    return "AStar".to_owned();
}

pub struct AStar {
    row_count: usize,
    column_count: usize,
    grid: Vec<Vec<Cell>>,
    start: Point,
    end: Point,
    simulation_state: SimulationState,
}

struct SimulationState {
    point_to_move_cost: HashMap<Point, MoveCost>,
    // path: HashSet<Point>,
    open_set: BinaryHeap<(Reverse<i32>, Point)>,
    closed_set: HashSet<Point>,
    point_to_came_from: Vec<(Point, Point)>,
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
            point_to_move_cost: HashMap::new(),
            // path: HashSet::new(),
            open_set: BinaryHeap::new(),
            closed_set: HashSet::new(),
            point_to_came_from: Vec::new(),
            path: HashSet::new(),
        };
        Self {
            row_count: 0,
            column_count: 0,
            grid: Vec::new(),
            start: Point::new(0, 0),
            end: Point::new(0, 0),
            simulation_state,
        }
    }

    fn find_path(&mut self) {
        self.simulation_state.point_to_move_cost.clear();
        // self.simulation_state.path.clear();
        self.simulation_state.open_set.clear();
        self.simulation_state.closed_set.clear();
        self.simulation_state.point_to_came_from.clear();

        let start = self.start;
        let end = self.end;
        let start_to_diff = (start.x - end.x).abs() + (start.y - end.y).abs();
        self.simulation_state
            .open_set
            .push((Reverse(start_to_diff), start));
        self.simulation_state.point_to_move_cost.insert(
            start,
            MoveCost {
                estimated: start_to_diff,
                real: 0,
            },
        );

        return;

        // self.simulation_state.path.insert(start);

        while let Some((_total, current)) = self.simulation_state.open_set.pop() {
            if current.x == end.x && current.y == end.y {
                break;
            }

            if self.simulation_state.closed_set.contains(&current) {
                continue;
            }

            self.simulation_state.closed_set.insert(current);

            let current_cost = self.simulation_state.point_to_move_cost[&current];

            let neighbours = get_taxicab_neighbours(current.x, current.y, 1);
            for neighbour in neighbours {
                if neighbour.x < 0
                    || neighbour.y < 0
                    || neighbour.x >= self.column_count as i32
                    || neighbour.y >= self.row_count as i32
                {
                    continue;
                }
                if self.grid[neighbour.y as usize][neighbour.x as usize].cell_type
                    == CellType::Blocked
                {
                    continue;
                }

                // self.simulation_state.path.insert(current);
                let estimated = (neighbour.x - end.x).abs() + (neighbour.y - end.y).abs();
                let move_cost = MoveCost {
                    estimated,
                    real: current_cost.real + 1,
                };
                if !self
                    .simulation_state
                    .point_to_move_cost
                    .contains_key(&neighbour)
                {
                    self.simulation_state
                        .open_set
                        .push((Reverse(move_cost.total()), neighbour));
                    self.simulation_state
                        .point_to_move_cost
                        .insert(neighbour, move_cost);
                }
            }
        }
    }

    fn step(&mut self) {
        if let Some((_total, current)) = self.simulation_state.open_set.pop() {
            if current.x == self.end.x && current.y == self.end.y {
                return;
            }

            if self.simulation_state.closed_set.contains(&current) {
                return;
            }

            self.simulation_state.closed_set.insert(current);

            let current_cost = self.simulation_state.point_to_move_cost[&current];

            let neighbours = get_taxicab_neighbours(current.x, current.y, 1);
            for neighbour in neighbours {
                if neighbour.x < 0
                    || neighbour.y < 0
                    || neighbour.x >= self.column_count as i32
                    || neighbour.y >= self.row_count as i32
                {
                    continue;
                }
                if self.grid[neighbour.y as usize][neighbour.x as usize].cell_type
                    == CellType::Blocked
                {
                    continue;
                }

                // self.simulation_state.path.insert(current);
                self.simulation_state
                    .point_to_came_from
                    .push((neighbour, current));
                let estimated = (neighbour.x - self.end.x).abs() + (neighbour.y - self.end.y).abs();
                let move_cost = MoveCost {
                    estimated,
                    real: current_cost.real + 1,
                };
                if !self
                    .simulation_state
                    .point_to_move_cost
                    .contains_key(&neighbour)
                {
                    self.simulation_state
                        .open_set
                        .push((Reverse(move_cost.total()), neighbour));
                    self.simulation_state
                        .point_to_move_cost
                        .insert(neighbour, move_cost);
                }
            }
        }

        self.simulation_state.path.clear();
        if let Some((child, parent)) = self.simulation_state.point_to_came_from.last() {
            let mut current_end = *child;
            for (new_child, new_parent) in self.simulation_state.point_to_came_from.iter().rev() {
                if current_end == *new_child {
                    self.simulation_state.path.insert(*new_child);
                    current_end = *new_parent;
                }
            }
        }

        // Found shortest path
        if let Some(_) = self.simulation_state.point_to_move_cost.get(&self.end) {
            self.simulation_state.path.clear();
            let mut current_parent = self.end;
            self.simulation_state.path.insert(self.end);
            while current_parent != self.start {
                let neighbours = get_taxicab_neighbours(current_parent.x, current_parent.y, 1);
                let mut distance = i32::MAX;
                for neighbour in neighbours {
                    if let Some(move_cost) =
                        self.simulation_state.point_to_move_cost.get(&neighbour)
                        && move_cost.real < distance
                    {
                        distance = move_cost.real;
                        current_parent = neighbour;
                    }
                }
                self.simulation_state.path.insert(current_parent);
            }
        }
    }
}

impl App for AStar {
    fn update(&mut self) {
        if input::is_key_pressed(KeyCode::F) {
            self.find_path();
        }
        if input::is_key_pressed(KeyCode::S) {
            self.step();
        }
    }

    fn draw(&self) {
        for row in 0..self.row_count {
            for column in 0..self.column_count {
                self.grid[row][column].draw();

                let x = column as f32 * CELL_SIZE;
                let y = row as f32 * CELL_SIZE;

                let point = Point::new(column as i32, row as i32);
                if self
                    .simulation_state
                    .point_to_move_cost
                    .contains_key(&point)
                {
                    let move_cost = &self.simulation_state.point_to_move_cost[&point];
                    draw_multiline_text(
                        &format!(
                            "{},{}\n{}|{}|{}",
                            column,
                            row,
                            move_cost.estimated,
                            move_cost.real,
                            move_cost.total()
                        ),
                        x + 3.0,
                        y + 13.0,
                        14.0,
                        None,
                        BLACK,
                    );
                }
            }
        }
        // for point in &self.simulation_state.closed_set {
        //     let x = point.x as f32 * CELL_SIZE + CELL_SIZE / 2.0;
        //     let y = point.y as f32 * CELL_SIZE + CELL_SIZE / 2.0;
        //     draw_text("C", x, y, 50.0, RED);
        // }
        for point in &self.simulation_state.path {
            let x = point.x as f32 * CELL_SIZE + CELL_SIZE / 2.0;
            let y = point.y as f32 * CELL_SIZE + CELL_SIZE / 2.0;
            draw_circle_lines(x, y, 10.0, 3.0, BLUE);
        }
    }

    fn resize(&mut self, width: f32, height: f32) {
        let row_count = height as i32 / CELL_SIZE as i32;
        let column_count = width as i32 / CELL_SIZE as i32;
        let mut grid: Vec<Vec<Cell>> = Vec::new();
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
                    let rnd = rand() % 4;
                    if rnd == 0 {
                        CellType::Blocked
                    } else {
                        CellType::Open
                    }
                };
                grid[row as usize].push(Cell::new(column, row, cell_type));
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
