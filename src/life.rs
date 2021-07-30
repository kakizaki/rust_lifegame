



#[derive(PartialEq)]
pub enum Life {
    Death, Living
}

impl Clone for Life {
    fn clone(&self) -> Self {
        return match self {
            Life::Death => Life::Death,
            Life::Living => Life::Living,
        };
    }
}

impl Copy for Life {
}

impl Life {
    pub fn get_next_state(&self, count_of_neighbor: u32) -> Self {
        return match self {
            Life::Death => {
                if count_of_neighbor == 3 {
                    Life::Living
                } else {
                    Life::Death
                }
            },
            Life::Living => {
                if count_of_neighbor == 2 || count_of_neighbor == 3 {
                    Life::Living
                } else {
                    // count <= 1 or 4 <= count
                    Life::Death
                }
            }
        };
    }
}

