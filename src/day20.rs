use std::fmt;
use std::str::FromStr;

#[derive(Clone)]
struct Image {
    image: Vec<Vec<char>>,
    algorithm: Vec<char>,
    background: bool,
}

impl Image {
    fn is_light(&self, x: i32, y: i32) -> bool {
        if x < 1 || y < 1 || x as usize > self.image[0].len() || y as usize > self.image.len() {
            return self.background;
        }
        match self.image[(y - 1) as usize][(x - 1) as usize] {
            '#' => true,
            '.' => false,
            p => panic!("invalid pixel '{}'", p),
        }
    }

    fn enhance(self) -> Self {
        let image = (0..self.image.len() + 2)
            .map(|y| {
                (0..self.image[0].len() + 2)
                    .map(|x| {
                        let mut idx = 0;
                        if self.is_light(x as i32 - 1, y as i32 - 1) {
                            idx += 0x100;
                        }
                        if self.is_light(x as i32, y as i32 - 1) {
                            idx += 0x080;
                        }
                        if self.is_light(x as i32 + 1, y as i32 - 1) {
                            idx += 0x040;
                        }
                        if self.is_light(x as i32 - 1, y as i32) {
                            idx += 0x020;
                        }
                        if self.is_light(x as i32, y as i32) {
                            idx += 0x010;
                        }
                        if self.is_light(x as i32 + 1, y as i32) {
                            idx += 0x008;
                        }
                        if self.is_light(x as i32 - 1, y as i32 + 1) {
                            idx += 0x004;
                        }
                        if self.is_light(x as i32, y as i32 + 1) {
                            idx += 0x002;
                        }
                        if self.is_light(x as i32 + 1, y as i32 + 1) {
                            idx += 0x001;
                        }
                        self.algorithm[idx]
                    })
                    .collect()
            })
            .collect();
        let background = if self.algorithm[0] == '#' {
            !self.background
        } else {
            false
        };
        Image {
            image,
            algorithm: self.algorithm,
            background,
        }
    }

    fn light_pixels(self) -> usize {
        let result = self.enhance().enhance();
        result
            .image
            .iter()
            .map(|row| row.iter().filter(|p| **p == '#').count())
            .sum()
    }

    fn super_light_pixels(self) -> usize {
        let mut image = self;
        for _ in 0..50 {
            image = image.enhance();
        }
        image
            .image
            .iter()
            .map(|row| row.iter().filter(|p| **p == '#').count())
            .sum()
    }
}

impl FromStr for Image {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (algorithm, image) = s.split_once("\n\n").unwrap();
        let algorithm = algorithm.chars().collect();
        let image = image
            .lines()
            .map(|line| line.trim_end().chars().collect())
            .collect();
        Ok(Image {
            image,
            algorithm,
            background: false,
        })
    }
}

impl fmt::Debug for Image {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "algorithm: {}\n",
            self.algorithm.iter().collect::<String>()
        )?;
        for row in self.image.iter() {
            writeln!(f, "{}", row.iter().collect::<String>())?;
        }
        Ok(())
    }
}

fn main() {
    const INPUT: &str = include_str!("../inputs/day20.txt");
    let image: Image = INPUT.parse().unwrap();
    println!("solution {}", image.clone().light_pixels());
    println!("solution {}", image.super_light_pixels());
}

#[test]
fn test_light_pixels() {
    const INPUT: &str = "..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

#..#.
#....
##..#
..#..
..###";
    let image: Image = INPUT.parse().unwrap();
    assert_eq!(image.light_pixels(), 35);
}

#[test]
fn test_super_light_pixels() {
    const INPUT: &str = "..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

#..#.
#....
##..#
..#..
..###";
    let image: Image = INPUT.parse().unwrap();
    assert_eq!(image.super_light_pixels(), 3351);
}
