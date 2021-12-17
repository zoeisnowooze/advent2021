use std::ops::RangeInclusive;

#[derive(Clone, Debug)]
struct Area {
    x: RangeInclusive<i32>,
    y: RangeInclusive<i32>,
}

impl Area {
    fn contains(&self, x: i32, y: i32) -> bool {
        self.x.contains(&x) && self.y.contains(&y)
    }
}

fn simulate(area: &Area, dx: i32, dy: i32) -> Option<i32> {
    let mut hit = false;
    let mut max_y = 0;
    for step in 0..1000 {
        let x = if step < dx {
            // General solution for $ x_{t+1} - x_t = -t - dx + 1 $.
            ((2 * dx + 1) * step - step * step) / 2
        } else {
            // Special case when $ x_{t+1} - x_t = 0 $ is reached.
            (dx * dx + dx) / 2
        };
        let y = ((2 * dy + 1) * step - step * step) / 2;

        if y > max_y {
            max_y = y;
        }

        if area.contains(x, y) {
            hit = true;
        }

        if y < *area.y.start() {
            break;
        }
    }

    hit.then(|| max_y)
}

fn find_max_y(area: Area) -> i32 {
    let mut max_y = 0;
    for dx in 1..=*area.x.end() {
        for dy in 1..1000 {
            if let Some(y) = simulate(&area, dx, dy) {
                if y > max_y {
                    max_y = y;
                }
            }
        }
    }
    max_y
}

fn count_hits(area: Area) -> i32 {
    let mut counter = 0;
    for dx in 1..=*area.x.end() {
        for dy in *area.y.start()..1000 {
            if simulate(&area, dx, dy).is_some() {
                counter += 1;
            }
        }
    }
    counter
}

fn main() {
    let area = Area {
        x: (179..=201),
        y: (-109..=-63),
    };
    println!("solution {}", find_max_y(area.clone()));
    println!("solution {}", count_hits(area));
}

#[test]
fn test_find_max_y() {
    assert_eq!(
        find_max_y(Area {
            x: (20..=30),
            y: (-10..=-5)
        }),
        45
    );
}

#[test]
fn test_simulate() {
    let area = Area {
        x: (20..=30),
        y: (-10..=-5),
    };
    assert_eq!(simulate(&area, 7, 2), Some(3));
    assert_eq!(simulate(&area, 6, 3), Some(6));
    assert_eq!(simulate(&area, 17, -4), None);
    assert_eq!(simulate(&area, 6, 9), Some(45));
}

#[test]
fn test_count_hits() {
    assert_eq!(
        count_hits(Area {
            x: (20..=30),
            y: (-10..=-5)
        }),
        112
    );
}
