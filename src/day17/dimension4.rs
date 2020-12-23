use std::cmp::Eq;
use std::hash::{Hash, Hasher};
use std::collections::HashSet;

#[derive(Debug, Eq, PartialEq)]
struct Position4 {
    x: isize,
    y: isize,
    z: isize,
    w: isize,
}

impl Hash for Position4 {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
        self.z.hash(state);
        self.w.hash(state);
    }
}

impl Position4 {
    fn new(x: isize, y: isize, z: isize, w: isize) -> Position4 {
        Position4 {
            x: x,
            y: y,
            z: z,
            w: w,
        }
    }

    fn from_tuple(tuple: (isize, isize, isize, isize)) -> Position4 {
        Position4 {
            x: tuple.0,
            y: tuple.1,
            z: tuple.2,
            w: tuple.3,
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
pub struct PocketDimension4 {
    active: HashSet<Position4>,
    x_limit: Limit,
    y_limit: Limit,
    z_limit: Limit,
    w_limit: Limit,
}

impl PocketDimension4 {
    pub fn from_string(s: &String) -> PocketDimension4 {
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
                    let p = Position4::from_tuple((x, y, 0, 0));
                    active.insert(p);
                }
            }
        }

        PocketDimension4 {
            active: active,
            x_limit: x_limit,
            y_limit: y_limit,
            z_limit: Limit::new(),
            w_limit: Limit::new(),
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
        let (w_min, w_max) = (self.w_limit.min - 1, self.w_limit.max + 2);

        for x in x_min..x_max {
            for y in y_min..y_max {
                for z in z_min..z_max {
                    for w in w_min..w_max {
                        let p = Position4::new(x, y, z, w);

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
        }

        self.active = new;
        self.update_limits();
    }

    fn update_limits(&mut self) {
        let n = self.active.iter().next().unwrap();
        let mut x_limit = Limit::from(n.x, n.x);
        let mut y_limit = Limit::from(n.y, n.y);
        let mut z_limit = Limit::from(n.z, n.z);
        let mut w_limit = Limit::from(n.w, n.w);

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

            if a.w < w_limit.min {
                w_limit.min = a.w;
            }

            if a.w > w_limit.max {
                w_limit.max = a.w;
            }
        }

        self.x_limit = x_limit;
        self.y_limit = y_limit;
        self.z_limit = z_limit;
        self.w_limit = w_limit;
    }

    fn active_neighbours(&self, position: &Position4) -> usize {
        let mut active_neighbour_count = 0;

        let (x_min, x_max) = (position.x - 1, position.x + 2);
        let (y_min, y_max) = (position.y - 1, position.y + 2);
        let (z_min, z_max) = (position.z - 1, position.z + 2);
        let (w_min, w_max) = (position.w - 1, position.w + 2);

        for x in x_min..x_max {
            for y in y_min..y_max {
                for z in z_min..z_max {
                    for w in w_min..w_max {
                        let p = Position4::new(x, y, z, w);
                        if *position == p {
                            continue;
                        }

                        if self.active.contains(&p) {
                            active_neighbour_count += 1;
                        }
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
        let (w_min, w_max) = (self.w_limit.min, self.w_limit.max + 1);

        for w in w_min..w_max {
            for z in z_min..z_max {
                println!("z={}, w={}", z, w);
                for y in y_min..y_max {
                    for x in x_min..x_max {
                        let p = Position4::new(x, y, z, w);

                        if self.active.contains(&p) {
                            print!("#");
                        } else {
                            print!(".");
                        }
                    }
                    print!("\n");
                }

                print!("\n");
            }

            // print!("\n\n");
        }
    }
}
