use crate::point::Point;
use std::collections::HashSet;

pub struct WorldState {
    pub field: HashSet<Point>,
    to_remove: HashSet<Point>,
    to_add: HashSet<Point>,
    to_check: HashSet<Point>,
}

impl WorldState {
    pub fn new() -> WorldState {
        WorldState {
            field: HashSet::new(),
            to_remove: HashSet::new(),
            to_add: HashSet::new(),
            to_check: HashSet::new(),
        }
    }

    pub fn toggle_point(&mut self, point: Point) {
        if self.field.contains(&point) {
            self.field.remove(&point)
        } else {
            self.field.insert(point)
        };
    }

    fn count_neighbours(&self, point: &Point) -> (i8, HashSet<Point>) {
        let mut count: i8 = 0;
        let mut to_check: HashSet<Point> = HashSet::new();

        for x in -1..=1 {
            for y in -1..=1 {
                let check_x = point.x + x;
                let check_y = point.y + y;
                let check_point = Point {
                    x: check_x,
                    y: check_y,
                };
                if self.field.contains(&check_point) {
                    if *point != check_point {
                        count += 1
                    }
                } else {
                    to_check.insert(Point {
                        x: check_x,
                        y: check_y,
                    });
                }
            }
        }
        (count, to_check)
    }

    fn check_alive(&mut self) {
        for point in self.field.iter() {
            let (count, to_check) = self.count_neighbours(point);
            if count < 2 || count > 3 {
                self.to_remove.insert(Point { ..*point });
            }
            if !to_check.is_empty() {
                self.to_check
                    .extend(to_check.iter().map(|p| Point { ..*p }))
            }
        }
    }
    fn check_new(&mut self) {
        for point in self.to_check.iter() {
            let (count, _) = self.count_neighbours(point);
            if count == 3 {
                self.to_add.insert(Point { ..*point });
            }
        }
    }

    pub fn next_step(&mut self) {
        self.check_alive();
        self.check_new();

        for point in self.to_remove.iter() {
            // TODO how to optimize?
            self.field.remove(point);
        }

        self.field
            .extend(self.to_add.iter().map(|p| Point { ..*p }));

        self.to_remove.clear();
        self.to_check.clear();
        self.to_add.clear();
    }
}
