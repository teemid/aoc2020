use std::cmp::Eq;
use std::hash::{Hash, Hasher};
use std::collections::HashSet;

#[derive(Debug, Eq, PartialEq)]
struct Position3 {
    x: isize,
    y: isize,
    z: isize,
}

impl Hash for Position3 {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
        self.z.hash(state);
    }
}

impl Position3 {
    fn new(x: isize, y: isize, z: isize) -> Position3 {
        Position3 {
            x: x,
            y: y,
            z: z,
        }
    }

    fn from_tuple(tuple: (isize, isize, isize)) -> Position3 {
        Position3 {
            x: tuple.0,
            y: tuple.1,
            z: tuple.2,
        }
    }
}

#[derive(Debug, PartialEq)]
struct Limit {
    min: isize,
    max: isize,
}

impl Limit {
    fn new() -> Limit {
        Limit {
            min: 0,
            max: 0,
        }
    }

    fn from(min: isize, max: isize) -> Limit {
        Limit {
            min: min,
            max: max,
        }
    }
}

#[derive(Debug)]
pub struct PocketDimension3 {
    active: HashSet<Position3>,
    x_limit: Limit,
    y_limit: Limit,
    z_limit: Limit,
}

impl PocketDimension3 {
    pub fn from_string(s: &String) -> PocketDimension3 {
        let mut active = HashSet::new();

        let mut x_limit = Limit::new();
        let mut y_limit = Limit::new();
        for (y, line) in s.lines().enumerate() {
            let y = y as isize;
            if y > y_limit.max {
                y_limit.max = y;
            }

            for (x, c) in line.chars().enumerate() {
                let x = x as isize;
                if x > x_limit.max {
                    x_limit.max = x;
                }

                if c == '#' {
                    let p = Position3::from_tuple((x, y, 0));
                    active.insert(p);
                }
            }
        }

        PocketDimension3 {
            active: active,
            x_limit: x_limit,
            y_limit: y_limit,
            z_limit: Limit::new(),
        }
    }

    pub fn active_cubes(&self) -> usize {
        self.active.len()
    }

    pub fn cycle(&mut self) {
        let mut new = HashSet::new();

        let (x_min, x_max) = (self.x_limit.min - 1, self.x_limit.max + 2);
        let (y_min, y_max) = (self.y_limit.min - 1, self.y_limit.max + 2);
        let (z_min, z_max) = (self.z_limit.min - 1, self.z_limit.max + 2);

        for x in x_min..x_max {
            for y in y_min..y_max {
                for z in z_min..z_max {
                    let p = Position3::new(x, y, z);

                    let active_neighbour_count = self.active_neighbours(&p);
                    if self.active.contains(&p) {
                        if active_neighbour_count == 2 || active_neighbour_count == 3 {
                            new.insert(p);
                        }
                    } else if active_neighbour_count == 3 {
                        new.insert(p);
                    }
                }
            }
        }

        self.active = new;
        self.update_limits();
    }

    fn update_limits(&mut self) {
        let n = self.active.iter().next().unwrap();
        let mut x_limit = Limit::from(n.x, n.x);
        let mut y_limit = Limit::from(n.y, n.y);
        let mut z_limit = Limit::from(n.z, n.z);

        for a in self.active.iter() {
            if a.x < x_limit.min {
                x_limit.min = a.x;
            }

            if a.x > x_limit.max {
                x_limit.max = a.x;
            }

            if a.y < y_limit.min {
                y_limit.min = a.y;
            }

            if a.y > y_limit.max {
                y_limit.max = a.y;
            }

            if a.z < z_limit.min {
                z_limit.min = a.z;
            }

            if a.z > z_limit.max {
                z_limit.max = a.z;
            }
        }

        self.x_limit = x_limit;
        self.y_limit = y_limit;
        self.z_limit = z_limit;
    }

    fn active_neighbours(&self, position: &Position3) -> usize {
        let mut active_neighbour_count = 0;

        for x in position.x-1..position.x+2 {
            for y in position.y-1..position.y+2 {
                for z in position.z-1..position.z+2 {
                    let p = Position3::new(x, y, z);
                    if *position == p {
                        continue;
                    }

                    if self.active.contains(&p) {
                        active_neighbour_count += 1;
                    }
                }
            }
        }

        active_neighbour_count
    }

    pub fn draw_dimension(&self) {
        let (x_min, x_max) = (self.x_limit.min, self.x_limit.max + 1);
        let (y_min, y_max) = (self.y_limit.min, self.y_limit.max + 1);
        let (z_min, z_max) = (self.z_limit.min, self.z_limit.max + 1);

        for z in z_min..z_max {
            println!("z={}", z);
            for y in y_min..y_max {
                for x in x_min..x_max {
                    let p = Position3::new(x, y, z);

                    if self.active.contains(&p) {
                        print!("#");
                    } else {
                        print!(".");
                    }
                }

                print!("\n");
            }

            print!("\n\n");
        }
    }
}
