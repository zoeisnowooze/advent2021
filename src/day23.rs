use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::fmt;
use std::hash::{Hash, Hasher};

#[derive(Copy, Clone, Eq)]
enum Space {
    Empty = 1,
    Amber,
    Bronze,
    Copper,
    Desert,
}

impl Space {
    fn cost(&self) -> usize {
        match self {
            Self::Amber => 1,
            Self::Bronze => 10,
            Self::Copper => 100,
            Self::Desert => 1000,
            _ => panic!("empty spaces don't move"),
        }
    }

    fn glyph(&self) -> char {
        match self {
            Self::Empty => '.',
            Self::Amber => 'A',
            Self::Bronze => 'B',
            Self::Copper => 'C',
            Self::Desert => 'D',
        }
    }
}

impl Hash for Space {
    fn hash<H: Hasher>(&self, state: &mut H) {
        (*self as u32).hash(state);
    }
}

impl PartialEq for Space {
    fn eq(&self, other: &Self) -> bool {
        *self as u32 == *other as u32
    }
}

impl Ord for Space {
    fn cmp(&self, other: &Self) -> Ordering {
        (*other as u32).cmp(&(*self as u32))
    }
}

impl PartialOrd for Space {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl fmt::Debug for Space {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.glyph())
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct Step {
    cost: usize,
    position: [Space; 19],
}

impl Ord for Step {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for Step {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl fmt::Debug for Step {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "#############")?;
        writeln!(
            f,
            "#{}#",
            self.position[0..11]
                .iter()
                .map(|space| space.glyph())
                .collect::<String>()
        )?;
        writeln!(
            f,
            "###{}#{}#{}#{}###",
            self.position[11].glyph(),
            self.position[13].glyph(),
            self.position[15].glyph(),
            self.position[17].glyph()
        )?;
        writeln!(
            f,
            "  #{}#{}#{}#{}#",
            self.position[12].glyph(),
            self.position[14].glyph(),
            self.position[16].glyph(),
            self.position[18].glyph()
        )?;
        writeln!(f, "  #########")
    }
}

fn swap(position: [Space; 19], a: usize, b: usize) -> [Space; 19] {
    let mut new_position = position;
    new_position[a] = position[b];
    new_position[b] = position[a];
    new_position
}

struct Burrow {
    goal: [Space; 19],
}

impl Burrow {
    fn valid_moves(&self, position: [Space; 19]) -> Vec<([Space; 19], usize)> {
        let mut moves = Vec::new();

        for door in [2, 4, 6, 8] {
            if position[door] != Space::Empty {
                if position[door - 1] == Space::Empty {
                    moves.push((swap(position, door - 1, door), position[door].cost()));
                }
                if position[door + 1] == Space::Empty {
                    moves.push((swap(position, door, door + 1), position[door].cost()));
                }
                if !moves.is_empty() {
                    return moves;
                }
            }
        }
        for room in [11, 13, 15, 17] {
            if position[room] != Space::Empty && position[room + 1] == Space::Empty {
                moves.push((swap(position, room, room + 1), position[room].cost()));
            }
            if position[room + 1] != Space::Empty && position[room] == Space::Empty {
                moves.push((swap(position, room, room + 1), position[room + 1].cost()));
            }
            if position[room] != Space::Empty && position[room - 9] == Space::Empty {
                moves.push((swap(position, room, room - 9), position[room].cost()));
            }
            if (position[room - 9] != Space::Empty && position[room] == Space::Empty)
                && ((position[room - 9] == self.goal[room + 1]
                    && position[room + 1] == Space::Empty)
                    || (position[room - 9] == self.goal[room]
                        && position[room + 1] == self.goal[room + 1]))
            {
                moves.push((swap(position, room - 9, room), position[room - 9].cost()));
            }
        }
        for hallway in [1, 3, 5, 7, 9] {
            if position[hallway] != Space::Empty {
                if position[hallway - 1] == Space::Empty {
                    moves.push((
                        swap(position, hallway, hallway - 1),
                        position[hallway].cost(),
                    ));
                }
                if position[hallway + 1] == Space::Empty {
                    moves.push((
                        swap(position, hallway, hallway + 1),
                        position[hallway].cost(),
                    ));
                }
            }
        }
        if position[0] != Space::Empty && position[1] == Space::Empty {
            moves.push((swap(position, 0, 1), position[0].cost()));
        }
        if position[10] != Space::Empty && position[9] == Space::Empty {
            moves.push((swap(position, 9, 10), position[10].cost()));
        }

        moves
    }
}

fn least_energy(start: &[Space; 19], goal: &[Space; 19]) -> Option<usize> {
    let burrow = Burrow { goal: *goal };

    let mut dist: HashMap<[Space; 19], usize> = HashMap::new();
    let mut heap = BinaryHeap::new();

    dist.insert(*start, 0);
    heap.push(Step {
        cost: 0,
        position: *start,
    });

    while let Some(Step { cost, position }) = heap.pop() {
        if &position == goal {
            return Some(cost);
        }

        let d = dist.entry(position).or_insert(usize::MAX);
        if &cost > d {
            continue;
        }

        println!("{:?}", Step { cost, position });

        let moves = burrow.valid_moves(position);
        for (next_position, next_cost) in moves {
            let next_cost = next_cost + cost;
            let next = Step {
                cost: next_cost,
                position: next_position,
            };
            if &next_cost < dist.get(&next_position).unwrap_or(&usize::MAX) {
                heap.push(next);
                dist.insert(next_position, next_cost);
            }
        }
    }

    None
}

fn main() {
    use Space::*;
    let start = [
        Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Amber, Amber,
        Bronze, Bronze, Copper, Copper, Desert, Desert,
    ];
    let goal = [
        Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Desert,
        Bronze, Desert, Amber, Copper, Amber, Bronze, Copper,
    ];
    println!("solution {}", least_energy(&start, &goal).unwrap());
}
