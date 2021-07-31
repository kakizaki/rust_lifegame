
mod cell;

use std::usize;

pub use crate::cell::*;


const WORLD_COLUMN_SIZE:usize = 16;
const WORLD_ROW_SIZE:usize = 64;

pub struct LifeGame {
    pub cells: [[Cell; WORLD_ROW_SIZE]; WORLD_COLUMN_SIZE],
    //pub lifes_: Vec<Vec<u8>>,
    pub x_size: usize,
    pub y_size: usize,
    generation_count: u32,
}


impl LifeGame {
    pub fn generate_vec(w: usize, h: usize) -> Vec<Vec<u8>> {
        let mut v = Vec::<Vec::<u8>>::with_capacity(w);

        for _ in 0..w {
            let mut v2 = Vec::<u8>::with_capacity(h);
            for _ in 0..h {
                v2.push(0);
            }
            v.push(v2);
        }

        return v;
    }
    
    pub fn new() -> LifeGame {
        LifeGame {
            cells: [[Cell::Death; WORLD_ROW_SIZE]; WORLD_COLUMN_SIZE],
            x_size: WORLD_COLUMN_SIZE,
            y_size: WORLD_ROW_SIZE,
            generation_count: 0,
        }
    }


    pub fn update(&mut self) -> bool {
        let mut changed = false;
        let mut new_generation = self.cells.clone();

        self.generation_count = self.generation_count + 1;
        let c = self.generate_character();

        for x in 0..self.x_size {
            for y in 0..self.y_size {
                let count = self.count_neighbor(x, y);

                if let Some(n) = Cell::next_cell(&self.cells[x][y], count, c) {
                    changed = true;
                    new_generation[x][y] = n;
                }
            }
        }

        self.cells = new_generation;
        return changed;
    }


    pub fn generate_character(&self) -> char {
        char::from_u32(
            self.generation_count % 24 + 0x30
        ).unwrap()
    }


    fn life_as_numeric(&self, x: usize, y: usize) -> u32 {
        if self.x_size <= x || self.y_size <= y {
            return 0;
        }

        return match self.cells[x][y] {
            Cell::Death => 0,
            Cell::Life(_) => 1,
        };
    }


    fn neighbor() -> [(i8, i8); 8] {
        [
            (-1, -1),
            (0, -1),
            (1, -1),
            (-1, 0),
            (1, 0),
            (-1, 1),
            (0, 1),
            (1, 1),
        ] 
    }


    fn usize_offset(i: usize, d: i8) -> Option<usize>{
        let ud = d.abs() as usize;
       
        if 0 <= d {
            return if i < usize::MAX - ud {
                Some(i + ud)
            } else {
                None
            };
        }

        return if ud <= i {
            Some(i - ud)
        } else {
            None
        };
    }


    fn count_neighbor(&self, x: usize, y: usize) -> u32 {
        if self.x_size <= x || self.y_size <= y {
            return 0;
        }
        
        let mut c = 0u32;
        
        for (dx, dy) in LifeGame::neighbor() {
            if let Some(_x) = LifeGame::usize_offset(x, dx) {
                if let Some(_y) = LifeGame::usize_offset(y, dy) {
                    c = c + self.life_as_numeric(_x, _y);
                }
            }
        }

        c
    }

}

