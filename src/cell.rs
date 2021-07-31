

use std::time::SystemTime;


#[derive(PartialEq)]
pub enum Cell {
    Death, Life(char)
}

impl Clone for Cell {
    fn clone(&self) -> Self {
        return match self {
            Cell::Death => Cell::Death,
            Cell::Life(n) => Cell::Life(n.clone()),
        };
    }
}

impl Copy for Cell {
}

impl Cell {
    pub fn update(&mut self, count_of_neighbor: u32, c: char) -> bool {
        if let Some(n) = self.next_cell(count_of_neighbor, c) {
            *self = n;
            return true;
        }
        return false;
    }

    pub fn next_cell(&self, count_of_neighbor: u32, character: char) -> Option<Self> {
        return match self {
            Cell::Death => {
                if count_of_neighbor == 3 {
                    Some(Cell::Life(character))
                } else {
                    None
                }
            },
            Cell::Life(_) => {
                if count_of_neighbor == 2 || count_of_neighbor == 3 {
                    None
                } else {
                    // count <= 1 or 4 <= count
                    Some(Cell::Death)
                }
            }
        };
    }
}

