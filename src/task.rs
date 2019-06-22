use crate::prelude::*;

#[derive(Hash, Copy, Clone, PartialEq, Eq, Debug, PartialOrd, Ord)]
pub struct Pos {
    pub x: i32,
    pub y: i32,
}

impl Pos {
    pub fn new(x: i32, y: i32) -> Pos {
        Pos { x, y }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct PosDiff {
    pub dx: i32,
    pub dy: i32,
}

impl std::ops::Add<PosDiff> for Pos {
    type Output = Self;
    fn add(self, rhs: PosDiff) -> Self {
        Pos::new(self.x + rhs.dx, self.y + rhs.dy)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum Angle {
    A0,
    A90,
    A180,
    A270,
}

impl Angle {
    pub fn turn_clock_wise(self) -> Angle {
        use Angle::*;
        match self {
            A0 => A90,
            A90 => A180,
            A180 => A270,
            A270 => A0,
        }
    }
    pub fn turn_counter_clock_wise(self) -> Angle {
        use Angle::*;
        match self {
            A0 => A270,
            A90 => A0,
            A180 => A90,
            A270 => A180,
        }
    }
}

impl PosDiff {
    pub fn new(dx: i32, dy: i32) -> PosDiff {
        PosDiff { dx, dy }
    }

    pub fn gen_all_diff() -> &'static [PosDiff] {
        lazy_static! {
            static ref DIFFS: Vec<PosDiff> = vec![
                PosDiff::new(1, 0),
                PosDiff::new(0, 1),
                PosDiff::new(-1, 0),
                PosDiff::new(0, -1)
            ];
        }
        &DIFFS[..]
    }

    pub fn turn(self, angle: Angle) -> PosDiff {
        use Angle::*;
        match angle {
            A0 => self,
            A90 => PosDiff::new(self.dy, -self.dx),
            A180 => PosDiff::new(-self.dx, -self.dy),
            A270 => PosDiff::new(-self.dy, self.dx),
        }
    }
}

pub type TaskId = u64;
pub type Tour = Vec<Pos>;

#[derive(Hash, Copy, Clone, PartialEq, Eq, Debug, PartialOrd, Ord)]
pub struct Booster {
    pub pos: Pos,
    pub kind: BoosterKind,
}

#[derive(Hash, Copy, Clone, PartialEq, Eq, Debug, PartialOrd, Ord)]
pub enum BoosterKind {
    ExtendManipulator,
    FastWheels,
    Drill,
    Mysterious,
    Teleport,
    Cloning,
}

impl BoosterKind {
    fn from_char(c: char) -> BoosterKind {
        use BoosterKind::*;
        if c == 'B' {
            ExtendManipulator
        } else if c == 'F' {
            FastWheels
        } else if c == 'L' {
            Drill
        } else if c == 'X' {
            Mysterious
        } else if c == 'R' {
            Teleport
        } else if c == 'C' {
            Cloning
        } else {
            unreachable!()
        }
    }
}

impl std::fmt::Display for BoosterKind {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use BoosterKind::*;
        writeln!(
            f,
            "{}",
            match self {
                ExtendManipulator => "B",
                FastWheels => "F",
                Drill => "L",
                Mysterious => "X",
                Teleport => "R",
                Cloning => "C",
            }
        )
    }
}

pub struct Task {
    pub id: TaskId,
    pub map: Tour,
    pub bot: Pos,
    pub obstacles: Vec<Tour>,
    pub boosters: Vec<Booster>,
}

impl Task {
    pub fn read_with_id(id: TaskId) -> Result<Task> {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push(format!("contest/problem/prob-{:03}.desc", id));
        Task::read_from(id, path)
    }

    pub fn parse_tour(s: &str) -> Tour {
        if s.is_empty() {
            vec![]
        } else {
            s[1..s.len() - 1]
                .split("),(")
                .map(|p| {
                    let mut x_y = p.split(',');
                    Pos::new(
                        x_y.next().unwrap().parse().unwrap(),
                        x_y.next().unwrap().parse().unwrap(),
                    )
                })
                .collect()
        }
    }

    fn parse_pos(s: &str) -> Pos {
        assert!(!s.is_empty());
        let mut x_y = s[1..s.len() - 1].split(',');
        Pos::new(
            x_y.next().unwrap().parse().unwrap(),
            x_y.next().unwrap().parse().unwrap(),
        )
    }

    fn parse_obstacles(s: &str) -> Vec<Tour> {
        if s.is_empty() {
            vec![]
        } else {
            s.split(';').map(Task::parse_tour).collect()
        }
    }

    fn parse_boosters(s: &str) -> Vec<Booster> {
        if s.is_empty() {
            vec![]
        } else {
            s.split(';').map(Task::parse_booster).collect()
        }
    }

    fn parse_booster(s: &str) -> Booster {
        assert!(!s.is_empty());
        // F(5,6)
        let pos = Task::parse_pos(&s[1..]);
        Booster {
            pos,
            kind: BoosterKind::from_char(s.chars().next().unwrap()),
        }
    }

    pub fn read_from(id: TaskId, path: impl AsRef<Path>) -> Result<Task> {
        let path = path.as_ref();
        debug!("read: {}", path.display());
        let s = std::fs::read_to_string(path)?;
        let s = s.split('#').collect::<Vec<_>>();
        assert_eq!(s.len(), 4);

        let map = Task::parse_tour(&s[0]);
        let bot = Task::parse_pos(&s[1]);
        let obstacles = Task::parse_obstacles(&s[2]);
        let boosters = Task::parse_boosters(&s[3]);

        Ok(Task {
            id,
            map,
            bot,
            obstacles,
            boosters,
        })
    }

    fn max_x(&self) -> i32 {
        assert!(!self.map.is_empty());
        self.map.iter().map(|pos| pos.x).max().unwrap()
    }

    fn max_y(&self) -> i32 {
        assert!(!self.map.is_empty());
        self.map.iter().map(|pos| pos.y).max().unwrap()
    }
}

#[derive(Hash, Copy, Clone, PartialEq, Eq, Debug, PartialOrd, Ord)]
pub enum Cell {
    Wall,
    Empty,
    Marked,
}

pub struct Map {
    pub id: u64,
    pub max_x: i32,
    pub max_y: i32,
    pub cells: Vec<Vec<Cell>>,
    pub empty_cell_count: usize,
    pub bot_start_pos: Pos,
    pub boosters: Vec<Booster>,
}

impl Map {
    #[cfg(test)]
    pub fn dump_map(&self) -> String {
        let mut rectangle = (0..self.max_x)
            .map(|x| {
                (0..self.max_y)
                    .map(|y| match self.cells[x as usize][y as usize] {
                        Cell::Wall => '#',
                        Cell::Empty => '.',
                        Cell::Marked => '-',
                    })
                    .collect::<Vec<char>>()
            })
            .collect::<Vec<_>>();

        rectangle[self.bot_start_pos.x as usize][self.bot_start_pos.y as usize] = 'O';

        for b in &self.boosters {
            rectangle[b.pos.x as usize][b.pos.y as usize] =
                b.kind.to_string().chars().next().unwrap();
        }

        (0..self.max_y as usize)
            .rev()
            .map(|y| {
                (0..self.max_x as usize)
                    .map(|x| rectangle[x][y])
                    .collect::<String>()
            })
            .collect::<Vec<_>>()
            .join("\n")
    }

    fn fill_tour(tour: &[Pos]) -> HashSet<Pos> {
        // BFS to fill interior of tour
        let mut visited = HashSet::new();
        let mut filled = HashSet::new();
        let mut q = VecDeque::new();

        for i in 0..tour.len() {
            let start = tour[i];
            let end = tour[(i + 1) % tour.len()];
            // Vec<(Interior, Wall)>)
            let borders: Vec<(Pos, Pos)> = {
                if start.x == end.x {
                    let x = start.x;
                    if start.y < end.y {
                        // Up
                        (start.y..end.y)
                            .map(|y| (Pos::new(x - 1, y), Pos::new(x, y)))
                            .collect()
                    } else if start.y > end.y {
                        // Down
                        (end.y..start.y)
                            .map(|y| (Pos::new(x, y), Pos::new(x - 1, y)))
                            .collect()
                    } else {
                        unreachable!();
                    }
                } else if start.y == end.y {
                    let y = start.y;
                    if start.x < end.x {
                        // => Right
                        (start.x..end.x)
                            .map(|x| (Pos::new(x, y), Pos::new(x, y - 1)))
                            .collect()
                    } else if start.x > end.x {
                        // <= Left
                        (end.x..start.x)
                            .map(|x| (Pos::new(x, y - 1), Pos::new(x, y)))
                            .collect()
                    } else {
                        unreachable!();
                    }
                } else {
                    unreachable!();
                }
            };
            for (interior, wall) in borders {
                visited.insert(wall);
                visited.insert(interior);
                filled.insert(interior);
                q.push_back(interior);
            }
        }

        while let Some(current) = q.pop_front() {
            for diff in PosDiff::gen_all_diff() {
                let next = current + *diff;
                if visited.contains(&next) {
                    continue;
                }
                visited.insert(next);
                filled.insert(next);
                q.push_back(next);
            }
        }
        filled
    }

    pub fn new(task: Task) -> Map {
        let max_x = task.max_x();
        let max_y = task.max_y();
        let mut cells = vec![vec![Cell::Wall; max_y as usize]; max_x as usize];

        for cell in Map::fill_tour(&task.map) {
            assert!(0 <= cell.x);
            assert!(cell.x < max_x);
            assert!(0 <= cell.y);
            assert!(cell.y < max_y);
            cells[cell.x as usize][cell.y as usize] = Cell::Empty;
        }

        for obstacle in &task.obstacles {
            for cell in Map::fill_tour(obstacle) {
                assert!(0 <= cell.x);
                assert!(cell.x < max_x);
                assert!(0 <= cell.y);
                assert!(cell.y < max_y);
                cells[cell.x as usize][cell.y as usize] = Cell::Wall;
            }
        }

        let empty_cell_count = cells
            .iter()
            .map(|r| r.iter().filter(|cell| **cell == Cell::Empty).count())
            .sum();

        Map {
            id: task.id,
            max_x,
            max_y,
            cells,
            empty_cell_count,
            bot_start_pos: task.bot,
            boosters: task.boosters,
        }
    }

    pub fn is_in_range(&self, pos: Pos) -> bool {
        0 <= pos.x && pos.x < self.max_x && 0 <= pos.y && pos.y < self.max_y
    }

    pub fn is_empty(&self, pos: Pos) -> bool {
        self.is_in_range(pos) && self.cells[pos.x as usize][pos.y as usize] == Cell::Empty
    }

    pub fn is_free(&self, pos: Pos) -> bool {
        self.is_in_range(pos) && self.cells[pos.x as usize][pos.y as usize] != Cell::Wall
    }

    pub fn is_wall(&self, pos: Pos) -> bool {
        self.is_in_range(pos) && self.cells[pos.x as usize][pos.y as usize] == Cell::Wall
    }

    pub fn do_drill(&mut self, pos: Pos) {
        assert!(self.is_in_range(pos));
        assert!(self.is_wall(pos));
        self.cells[pos.x as usize][pos.y as usize] = Cell::Marked;
    }

    pub fn mark_pos(&mut self, pos: Pos) {
        if !self.is_in_range(pos) {
            return;
        }
        let cell: &mut Cell = &mut self.cells[pos.x as usize][pos.y as usize];
        match cell {
            Cell::Empty => {
                self.empty_cell_count -= 1;
                *cell = Cell::Marked;
            }
            Cell::Wall => {}
            Cell::Marked => {}
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn pase_tour_test() {
        assert_eq!(
            Task::parse_tour("(0,0),(10,0),(10,10),(0,10)"),
            vec![
                Pos::new(0, 0),
                Pos::new(10, 0),
                Pos::new(10, 10),
                Pos::new(0, 10)
            ]
        );
    }

    #[test]
    fn pase_pos_test() {
        assert_eq!(Task::parse_pos("(0,1)"), Pos::new(0, 1),);
    }

    #[test]
    fn pase_obstables_test() {
        assert_eq!(
            Task::parse_obstacles("(4,2),(6,2);(0,1)"),
            vec![vec![Pos::new(4, 2), Pos::new(6, 2)], vec![Pos::new(0, 1)]]
        );
    }

    #[test]
    fn pase_booster_test() {
        assert_eq!(
            Task::parse_booster("F(4,2)"),
            Booster {
                pos: Pos::new(4, 2),
                kind: BoosterKind::FastWheels,
            }
        );
    }

    #[test]
    fn pase_boosters_test() {
        assert_eq!(
            Task::parse_boosters("F(4,2);B(0,1)"),
            vec![
                Booster {
                    pos: Pos::new(4, 2),
                    kind: BoosterKind::FastWheels,
                },
                Booster {
                    pos: Pos::new(0, 1),
                    kind: BoosterKind::ExtendManipulator,
                }
            ]
        );

        assert_eq!(Task::parse_boosters(""), vec![]);
    }

    #[test]
    fn read_all_tasks_test() -> Result<()> {
        for i in 1..=150 {
            Task::read_with_id(i)?;
        }
        Ok(())
    }

    #[test]
    fn read_task_1_test() -> Result<()> {
        let task = Task::read_with_id(1)?;
        assert_eq!(task.max_x(), 8);
        assert_eq!(task.max_y(), 3);
        Ok(())
    }

    #[test]
    fn map_test() -> Result<()> {
        let task = Task::read_with_id(2)?;
        let map = Map::new(task);
        // println!("{}", map.dump_map());
        assert_eq!(
            map.dump_map(),
            "##############............################
##############............################
##############............################
##############.........#..################
##############.........#..################
##############......#..#..################
##############..#####..#..################
##############......#..#..################
##############.....###.#..################
##############.....######.#######...######
##############.....###....#######...######
##############....######..#######...######
......########....######..########....####
......########...########.########....####
......########....######..########....####
......########.#########..####........####
......########..#.######..####........####
......########..#..##.....####........#...
......########..#..##.....####....##......
......########............######...#......
......##########....#......#####...#...###
O.....##########....#......#...#...#######
##..................#......#...##...######
##.......X...F......#......#...##...######
##..............B...#......#...###..######
##..................#......##.......######
##.......#..........####............######
##....######........####..........########
##..#.#########.....####.....#############
##.#########.....L..######################
##....######........######################
##.#########........######################
##....######....##########################
##......###.....##########################
##...B..###.....##########################
##......###.....##########################
##...F..###.....##########################
##......###.....##########################
##......###...F.##########################
##......###.....##########################
##......###.....##########################
##......###.....##########################
##..............##########################"
        );
        Ok(())
    }

}
