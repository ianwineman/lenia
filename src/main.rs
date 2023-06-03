use std::fmt;

const WORLD_SIZE: usize = 4;

struct World {
    map: [[u8; WORLD_SIZE]; WORLD_SIZE],
}

impl World {
    fn new_world() -> World {
        World {
            map: [[0; WORLD_SIZE]; WORLD_SIZE]
        }
    }

    fn step_forward(&mut self) {
        for x in 0..WORLD_SIZE {
            for y in 0..WORLD_SIZE {
                if x == 0 {
                    if y == 0 {
                        let neighborhood: [[u8; 3]; 3] = [
                            [
                                0, 0, 0
                            ], 
                            [
                                0, self.map[x][y], self.map[x][y+1]
                            ], 
                            [
                                0, self.map[x+1][y], self.map[x+1][y+1]
                            ]
                        ]
                    }
                    else if (y in 1..(WORLD_SIZE-1)) {
                        let neighborhood: [[u8; 3]; 3] = [
                            [
                                0, 0, 0
                            ], 
                            [
                                self.map[x][y-1], self.map[x][y], self.map[x][y+1]
                            ], 
                            [
                                self.map[x+1][y-1], self.map[x+1][y], self.map[x+1][y+1]
                            ]
                        ]
                    }
                    else {
                        let neighborhood: [[u8; 3]; 3] = [
                            [
                                0, 0, 0
                            ], 
                            [
                                self.map[x][y-1], self.map[x][y], 0
                            ], 
                            [
                                self.map[x+1][y-1], self.map[x+1][y], 0
                            ]
                        ]
                    }
                }
                else if (x in 1..(WORLD_SIZE-1)) {
                    if y == 0 {
                        let neighborhood: [[u8; 3]; 3] = [
                            [
                                0, self.map[x-1][y], self.map[x-1][y+1]
                            ], 
                            [
                                0, self.map[x][y], self.map[x][y+1]
                            ], 
                            [
                                0, self.map[x+1][y], self.map[x+1][y+1]
                            ]
                        ]
                    }
                    else if (y in 1..(WORLD_SIZE-1)) {
                        let neighborhood: [[u8; 3]; 3] = [
                            [
                                self.map[x-1][y-1], self.map[x-1][y], self.map[x-1][y+1]
                            ], 
                            [
                                self.map[x][y-1], self.map[x][y], self.map[x][y+1]
                            ], 
                            [
                                self.map[x+1][y-1], self.map[x+1][y], self.map[x+1][y+1]
                            ]
                        ]
                    }
                    else {
                        let neighborhood: [[u8; 3]; 3] = [
                            [
                                self.map[x-1][y-1], self.map[x-1][y], 0
                            ], 
                            [
                                self.map[x][y-1], self.map[x][y], 0
                            ], 
                            [
                                self.map[x+1][y-1], self.map[x+1][y], 0
                            ]
                        ]
                    }
                }
                else {
                    if y == 0 {
                        let neighborhood: [[u8; 3]; 3] = [
                            [
                                0, self.map[x-1][y], self.map[x-1][y+1]
                            ], 
                            [
                                0, self.map[x][y], self.map[x][y+1]
                            ], 
                            [
                                0, 0, 0
                            ]
                        ]
                    }
                    else if (y in 1..(WORLD_SIZE-1)) {
                        let neighborhood: [[u8; 3]; 3] = [
                            [
                                self.map[x-1][y-1], self.map[x-1][y], self.map[x-1][y+1]
                            ], 
                            [
                                self.map[x][y-1], self.map[x][y], self.map[x][y+1]
                            ], 
                            [
                                0, 0, 0
                            ]
                        ]
                    }
                    else {
                        let neighborhood: [[u8; 3]; 3] = [
                            [
                                self.map[x-1][y-1], self.map[x-1][y], 0
                            ], 
                            [
                                self.map[x][y-1], self.map[x][y], 0
                            ], 
                            [
                                0, 0, 0
                            ]
                        ]
                    }
                }

                let cell_neighborhood_sum: u8 = convolution(neighborhood);
                let cell_state: u8 = self.map[x][y];

                self.map[x][y] = rule(cell_neighborhood_sum, cell_state);
            }
        }
    }

    fn step_backward() {
        todo!(); // probably have to do cacheing for this
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

fn convolution(neighborhood: [[u8; 3]; 3]) -> u8 {
    let mut neighborhood_sum: u8 =  neighborhood.map(|row| row.into_iter().sum::<u8>()).into_iter().sum::<u8>();
    neighborhood_sum -= neighborhood[1][1];

    return neighborhood_sum
}

fn rule(neighborhood_sum: u8, cell_state: u8) -> u8 {
    match cell_state {
        0 => match neighborhood_sum {
            0..=2 => return 0,
            3     => return 1,
            4..=8 => return 0,
            _     => panic!(),
        },
        1 => match neighborhood_sum {
            0..=1  => return 0,
            2..=3 => return 1,
            3..=8 => return 0,
            _     => panic!(), 
        },
        _ => panic!(),
    }
}

fn main() {
    let world: World = World::new_world();

    println!("{}", world);
}
