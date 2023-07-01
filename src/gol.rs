use regex::Regex;
use std::convert::TryFrom;
use std::fmt;
use std::fs;
use std::str;
use rand::Rng;

pub const WORLD_SIZE: usize = 17;

pub struct World {
    pub map: [[u8; WORLD_SIZE]; WORLD_SIZE],
}

impl World {
    pub fn new_empty() -> World {
        World {
            map: [[0; WORLD_SIZE]; WORLD_SIZE],
        }
    }

    pub fn new_random() -> World {
        let mut rng = rand::thread_rng();
        let mut world_map: [[u8; WORLD_SIZE]; WORLD_SIZE] = [[0; WORLD_SIZE]; WORLD_SIZE];

        for x in 0..WORLD_SIZE {
            for y in 0..WORLD_SIZE {
                world_map[x][y] += rng.gen_range(0..2);
            }
        }

        World {
            map: world_map,
        }
    }

    pub fn new(seed: [[u8; WORLD_SIZE]; WORLD_SIZE]) -> World {
        World { map: seed }
    }

    pub fn new_from_rle(path: &str) -> World {
        let file_string: String = fs::read_to_string(path).expect("Unable to read file.");
        let mut file_string_split: Vec<&str> = file_string.split(":").collect();
        let rle_str: &str = file_string_split.pop().unwrap();

        let mut decoded_flat_vec: Vec<u8> = Vec::new();

        for caps in Regex::new(r"(?P<num>\d+)(?P<state>a|d)")
            .unwrap()
            .captures_iter(rle_str)
        {
            let num: u8 = caps["num"].to_string().parse::<u8>().unwrap();

            for _ in 0..num {
                if &caps["state"] == "a" {
                    decoded_flat_vec.push(1);
                } else if &caps["state"] == "d" {
                    decoded_flat_vec.push(0);
                } else {
                    panic!();
                }
            }
        }

        let mut twod_vec: Vec<&[u8]> = Vec::new();

        for i in 0..WORLD_SIZE {
            twod_vec.push(&decoded_flat_vec[i * WORLD_SIZE..(i + 1) * WORLD_SIZE]);
        }

        let world_map: [[u8; WORLD_SIZE]; WORLD_SIZE] = twod_vec
            .into_iter()
            .map(|slice| slice.try_into().unwrap())
            .collect::<Vec<[u8; WORLD_SIZE]>>()
            .try_into()
            .unwrap();

        World { map: world_map }
    }

    pub fn to_rle(&self) -> String {
        let map: [[u8; WORLD_SIZE]; WORLD_SIZE] = self.map;
        let mut scan_line: String = String::new();

        for row in map {
            for i in row {
                match i {
                    0 => scan_line.push('0'),
                    1 => scan_line.push('1'),
                    _ => {}
                }
            }
        }

        let mut rle: String = (WORLD_SIZE as u32).to_string();
        rle.push_str("::");
        rle.push_str(&run_length_encoding(scan_line));

        return rle;
    }

    pub fn save(&self, path: &str) {
        let rle: String = self.to_rle();

        fs::write(path, &rle).expect("Unable to write file.");
    }

    pub fn step_forward(&mut self) {
        let mut world_with_buffer: [[u8; WORLD_SIZE + 2]; WORLD_SIZE + 2] =
            [[0; WORLD_SIZE + 2]; WORLD_SIZE + 2];

        for x in 0..WORLD_SIZE {
            for y in 0..WORLD_SIZE {
                world_with_buffer[x + 1][y + 1] = self.map[x][y];
            }
        }

        for x in 0..WORLD_SIZE {
            for y in 0..WORLD_SIZE {
                let neighborhood: [[u8; 3]; 3] = [
                    [
                        world_with_buffer[x][y],
                        world_with_buffer[x][y + 1],
                        world_with_buffer[x][y + 2],
                    ],
                    [
                        world_with_buffer[x + 1][y],
                        world_with_buffer[x + 1][y + 1],
                        world_with_buffer[x + 1][y + 2],
                    ],
                    [
                        world_with_buffer[x + 2][y],
                        world_with_buffer[x + 2][y + 1],
                        world_with_buffer[x + 2][y + 2],
                    ],
                ];
                let cell_neighborhood_sum: u8 = convolution(neighborhood);
                let cell_state: u8 = self.map[x][y];

                self.map[x][y] = rule(cell_neighborhood_sum, cell_state);
            }
        }
    }

    pub fn update_cell(&mut self, row: usize, col: usize) {
        match self.map[row][col] {
            1 => self.map[row][col] = 0,
            0 => self.map[row][col] = 1,
            _ => (),
        };
    }
}

#[allow(unused_must_use)]
impl fmt::Display for World {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in self.map {
            write!(f, "{:?}\n", row);
        }
        Ok(())
    }
}

fn run_length_encoding(scan_line: String) -> String {
    let mut char_pairs: Vec<(u32, char)> = Vec::new();
    let scan_str: &str = scan_line.as_str();

    let mut counting_char: char = scan_str.chars().next().unwrap();
    let mut counter: u32 = 0;

    for c in scan_str.chars() {
        if c == counting_char {
            counter += 1;
        } else {
            char_pairs.push((counter, counting_char));
            counter = 1;
            counting_char = c;
        }
    }
    char_pairs.push((counter, counting_char));

    let mut encoded: String = String::new();

    for i in char_pairs {
        let num: String = i.0.to_string();
        for c in num.chars() {
            encoded.push(c);
        }

        if i.1 == '0' {
            encoded.push('d')
        } else if i.1 == '1' {
            encoded.push('a')
        } else {
            panic!();
        }
    }

    return encoded;
}

fn convolution(neighborhood: [[u8; 3]; 3]) -> u8 {
    return neighborhood[0].into_iter().sum::<u8>()
        + neighborhood[1][0]
        + neighborhood[1][2]
        + neighborhood[2].into_iter().sum::<u8>();
}

fn rule(neighborhood_sum: u8, cell_state: u8) -> u8 {
    match cell_state {
        0 => match neighborhood_sum {
            0 | 1 | 2 => return 0,
            3 => return 1,
            4.. => return 0,
        },
        1 => match neighborhood_sum {
            0 | 1 => return 0,
            2 | 3 => return 1,
            4.. => return 0,
        },
        _ => panic!("cell_state != 0 | 1"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn convolution_test_1() {
        let neighborhood: [[u8; 3]; 3] = [[1, 0, 1], [0, 1, 0], [1, 0, 1]];

        let convolution_output: u8 = convolution(neighborhood);

        assert_eq!(
            convolution_output, 4_u8,
            "Expected 4, got {}.",
            convolution_output,
        );
    }

    #[test]
    fn rule_test_1() {
        let neighborhood_sum: u8 = 3;
        let cell_state: u8 = 0;

        let rule_output: u8 = rule(neighborhood_sum, cell_state);

        assert_eq!(rule_output, 1_u8, "Expected 1, got {}.", rule_output,);
    }

    #[test]
    fn rule_test_2() {
        let neighborhood_sum: u8 = 1;
        let cell_state: u8 = 0;

        let rule_output: u8 = rule(neighborhood_sum, cell_state);

        assert_eq!(rule_output, 0_u8, "Expected 0, got {}.", rule_output,);
    }

    #[test]
    fn rule_test_3() {
        let neighborhood_sum: u8 = 100;
        let cell_state: u8 = 0;

        let rule_output: u8 = rule(neighborhood_sum, cell_state);

        assert_eq!(rule_output, 0_u8, "Expected 0, got {}.", rule_output,);
    }

    #[test]
    fn rule_test_4() {
        let neighborhood_sum: u8 = 0;
        let cell_state: u8 = 1;

        let rule_output: u8 = rule(neighborhood_sum, cell_state);

        assert_eq!(rule_output, 0_u8, "Expected 0, got {}.", rule_output,);
    }

    #[test]
    fn rule_test_5() {
        let neighborhood_sum: u8 = 3;
        let cell_state: u8 = 1;

        let rule_output: u8 = rule(neighborhood_sum, cell_state);

        assert_eq!(rule_output, 1_u8, "Expected 1, got {}.", rule_output,);
    }

    #[test]
    fn rule_test_6() {
        let neighborhood_sum: u8 = 7;
        let cell_state: u8 = 1;

        let rule_output: u8 = rule(neighborhood_sum, cell_state);

        assert_eq!(rule_output, 0_u8, "Expected 0, got {}.", rule_output,);
    }
}
