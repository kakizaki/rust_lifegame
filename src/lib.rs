
mod life;

pub use crate::life::*;


const WORLD_COLUMN_SIZE:usize = 8;
const WORLD_ROW_SIZE:usize = 10;

pub struct LifeGame {
    pub lifes: [[Life; WORLD_ROW_SIZE]; WORLD_COLUMN_SIZE],
    //pub lifes_: Vec<Vec<u8>>,
    pub x_size: usize,
    pub y_size: usize,
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
            lifes: [[Life::Death; WORLD_ROW_SIZE]; WORLD_COLUMN_SIZE],
            x_size: WORLD_COLUMN_SIZE,
            y_size: WORLD_ROW_SIZE,
        }
    }


    pub fn update(&mut self) {
        let mut new_generation = [[Life::Death; WORLD_ROW_SIZE]; WORLD_COLUMN_SIZE];

        for x in 0..self.x_size {
            for y in 0..self.y_size {
                let count = self.count_neighbor(x, y);
                new_generation[x][y] = self.lifes[x][y].get_next_state(count);
            }
        }

        self.lifes = new_generation;
    }


    fn exists_life_as_numeric(&self, lifes: &[Life], i: usize) -> u32 {
        if lifes.len() <= i {
            return 0;
        }

        return match lifes[i] {
            Life::Living => 1,
            Life::Death => 0
        };
    }


    fn count_neighbor(&self, x: usize, y: usize) -> u32 {
        let mut c = 0u32;

        if self.x_size <= x {
            return 0;
        }

        if 0 < x {
            let l = &self.lifes[x - 1];
            if 0 < y { c = c + self.exists_life_as_numeric(l, y - 1); }
            c = c + self.exists_life_as_numeric(l, y);
            c = c + self.exists_life_as_numeric(l, y + 1);
        }

        if x < self.x_size {
            let l = &self.lifes[x];
            if 0 < y { c = c + self.exists_life_as_numeric(l, y - 1); }
            c = c + self.exists_life_as_numeric(l, y + 1);
        }

        if x + 1 < self.x_size {
            let l = &self.lifes[x + 1];
            if 0 < y { c = c + self.exists_life_as_numeric(l, y - 1); }
            c = c + self.exists_life_as_numeric(l, y);
            c = c + self.exists_life_as_numeric(l, y + 1);
        } 

        c
    }

}

