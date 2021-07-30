



#[derive(PartialEq)]
pub enum Life {
    Death, Living(char)
}

impl Clone for Life {
    fn clone(&self) -> Self {
        return match self {
            Life::Death => Life::Death,
            Life::Living(n) => Life::Living(n.clone()),
        };
    }
}

impl Copy for Life {
}

impl Life {
    pub fn birth() -> Self {
        Life::Living('&')
    }

    pub fn get_next_state(&self, count_of_neighbor: u32) -> Self {
        return match self {
            Life::Death => {
                if count_of_neighbor == 3 {
                    Life::birth()
                } else {
                    Life::Death
                }
            },
            Life::Living(n) => {
                if count_of_neighbor == 2 || count_of_neighbor == 3 {
                    self.clone()
                } else {
                    // count <= 1 or 4 <= count
                    Life::Death
                }
            }
        };
    }
}

