/*
  Copyright (c) 2021 José Puga. Under MIT License.
  My Conway Game of Life implementation usign a 1d vector.
  A simple exercise to learn Rust.
*/

use rand::prelude::*; // To fill the world randomly
use std::fmt; // Imports fmt to use println! in world
use std::{thread, time}; // To pause between cycles

const WORLD_WIDTH: usize = 80;
const WORLD_HEIGHT: usize = 25;
const CYCLE_DELAY: u64 = 250; //Delay between cycles in milliseconds
const START_POPULATION: f32 = 0.3; //(1.0 = 100%) % (percent) of random population at start.

fn main() {
    let mut world = World::new(WORLD_WIDTH, WORLD_HEIGHT);

    //Start random population
    let mut rnd = rand::thread_rng();
    let world_size = world.width * world.height;
    let init_pop = (world_size as f32 * START_POPULATION) as usize;
    for _i in 0..(init_pop) {
        world.set(rnd.gen_range(0..world_size), true); //TODO: Check if already used.
    }

    loop {
        print!("\x1B[2J\x1B[1;1H"); //Clears screens
        println!("{}", world);
        world.cycle();
        thread::sleep(time::Duration::from_millis(CYCLE_DELAY));
    }
}

struct World {
    width: usize,
    height: usize,
    _wrap: bool, //TODO:
    rules: Vec<(bool, bool)>,
    map: Vec<bool>,
    map_next_gen: Vec<bool>,
}

impl fmt::Display for World {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut col = 0;
        let mut buffer = String::with_capacity(self.width * self.height + self.height); // + '\n'
        for idx in 0..(self.width * self.height) {
            if self.get(idx) {
                buffer.push('*');
            } else {
                buffer.push(' ');
            }
            col += 1;
            if col == self.width {
                buffer.push('\n');
                col = 0;
            }
        }
        writeln!(f, "{}", buffer)
    }
}

impl World {
    fn new(width: usize, height: usize) -> Self {
        World {
            width,
            height,
            _wrap: false,
            //  Index = Neighbours Count => (cell state, new cell state)
            rules: vec![
                (true, false), // 0 and 1 dies for underpopulation
                (true, false),
                (true, true),  // 2 cell survive
                (false, true), // 3 neighbours born new cell
                (true, false), // >3 dies for overpopulation
                (true, false),
                (true, false),
                (true, false),
                (true, false),
            ],
            map: vec![false; width * height],
            map_next_gen: vec![false; width * height],
        }
    }

    fn set(&mut self, idx: usize, cell: bool) {
        self.map[idx] = cell;
    }

    fn get(&self, idx: usize) -> bool {
        self.map[idx]
    }

    fn neighbors(&self, idx: usize) -> u32 {
        const SURROUND: &'static [(i32, i32)] = &[
            (-1, -1),
            (0, -1),
            (1, -1),
            (-1, 0),
            (1, 0),
            (-1, 1),
            (0, 1),
            (1, 1),
        ];
        let col = idx % self.width;
        let row = idx / self.width;
        let mut result: u32 = 0;

        for rel_pos in SURROUND.iter() {
            if (rel_pos.0 == -1 && col == 0) || (rel_pos.0 == 1 && col == self.width - 1) {
                continue;
            }

            if (rel_pos.1 == -1 && row == 0) || (rel_pos.1 == 1 && row == self.height - 1) {
                continue;
            }
            let n_col;
            let n_row;
            // Is not simple in rust to use usize and other integers together...
            if rel_pos.0.is_negative() {
                n_col = col - rel_pos.0.wrapping_abs() as u32 as usize;
            } else {
                n_col = col + rel_pos.0 as usize;
            }

            if rel_pos.1.is_negative() {
                n_row = row - rel_pos.1.wrapping_abs() as u32 as usize;
            } else {
                n_row = row + rel_pos.1 as usize;
            }

            if self.get(n_col + n_row * self.width) {
                result += 1;
            }
        }
        result
    }

    fn cycle(&mut self) {
        for idx in 0..(self.width * self.height) {
            // usize because is an index of self.rules[]
            let neighbors_count = self.neighbors(idx) as usize;
            // Apply rules
            if self.map[idx] == self.rules[neighbors_count].0 {
                self.map_next_gen[idx] = self.rules[neighbors_count].1;
            } else {
                self.map_next_gen[idx] = self.map[idx];
            }
        }
        std::mem::swap(&mut self.map, &mut self.map_next_gen)
    }
}
