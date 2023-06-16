use std::fmt;

pub const WORLD_SIZE: usize = 16;

pub struct World {
    pub map: [[u8; WORLD_SIZE]; WORLD_SIZE],
}

impl World {
    pub fn new_empty_world() -> World {
        World {
            map: [[0; WORLD_SIZE]; WORLD_SIZE]
        }
    }

    pub fn new_world(seed: [[u8; WORLD_SIZE]; WORLD_SIZE]) -> World {
        World {
            map: seed
        }
    }

    pub fn step_forward(&mut self) {
        let mut world_with_buffer: [[u8; WORLD_SIZE + 2]; WORLD_SIZE + 2] = [[0; WORLD_SIZE + 2]; WORLD_SIZE + 2];

        for x in 0..WORLD_SIZE {
            for y in 0..WORLD_SIZE {
                world_with_buffer[x+1][y+1] = self.map[x][y];
            }
        }

        for x in 0..WORLD_SIZE {
            for y in 0..WORLD_SIZE {
                let neighborhood: [[u8; 3]; 3] = [
                    [world_with_buffer[x  ][y], world_with_buffer[x  ][y+1], world_with_buffer[x  ][y+2]],
                    [world_with_buffer[x+1][y], world_with_buffer[x+1][y+1], world_with_buffer[x+1][y+2]],
                    [world_with_buffer[x+2][y], world_with_buffer[x+2][y+1], world_with_buffer[x+2][y+2]],
                ];
                let cell_neighborhood_sum: u8 = convolution(neighborhood);
                let cell_state: u8 = self.map[x][y];

                self.map[x][y] = rule(cell_neighborhood_sum, cell_state);
            }
        }
    }
}

impl fmt::Display for World {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in self.map {
            write!(f, "{:?}\n", row);
        }
        Ok(())
    }
}

pub fn convolution(neighborhood: [[u8; 3]; 3]) -> u8 {
    return neighborhood[0].into_iter().sum::<u8>() + neighborhood[1][0] + neighborhood[1][2] + neighborhood[2].into_iter().sum::<u8>()
}

pub fn rule(neighborhood_sum: u8, cell_state: u8) -> u8 {
    match cell_state {
        0 => match neighborhood_sum {
            0 | 1 | 2 => return 0,
            3         => return 1,
            4..       => return 0,
        },
        1 => match neighborhood_sum {
            0 | 1 => return 0,
            2 | 3 => return 1,
            4..   => return 0,
        },
        _ => panic!("cell_state != 0 | 1"),
    }
}