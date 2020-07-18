use std::fmt::{Display, Formatter};

fn main() {

    let mut game = Game::new();
    let sleep_duration = std::time::Duration::from_millis(33);
    loop {
        println!("{}", game);
        game.step();
        std::thread::sleep(sleep_duration);
    }
}

struct Game {
    frame: Frame,
    ball: Ball,
}

struct Frame {
    width: u32,
    height: u32,
}

struct Ball {
    x: u32,
    y: u32,
    vert_dir: VertDir,
    horiz_dir: HorizDir,
}

enum VertDir {
    Up,
    Down,
}

enum HorizDir {
    Left,
    Right,
}

impl Game {
    fn new() -> Game {
        Game {
            frame: Frame { width: 50, height: 26 },
            ball: Ball {
                x: 2,
                y: 4,
                vert_dir: VertDir::Up,
                horiz_dir: HorizDir::Left,
            }
        }
    }

    fn step(&mut self) {
        self.ball.bounce(&self.frame);
        self.ball.mv();
    }
}

impl Display for Game {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        let mut top_bottom = |f: &mut Formatter| {
            write!(f, "+")?;
            for _ in 0..self.frame.width {
                write!(f, "-")?;
            }
            write!(f, "+\n")
        };
        top_bottom(f)?;
        for row in 0..self.frame.height {
            write!(f, "|")?;
            for column in 0..self.frame.width {
                let c = if row == self.ball.y && column == self.ball.x {
                    'o'
                } else {
                    ' '
                };
                write!(f, "{}", c)?;
            }
            write!(f, "|\n")?;
        }
        top_bottom(f)
    }
}

impl Ball {
    fn bounce(&mut self, frame: &Frame) {
        if self.x == 0 {
            self.horiz_dir = HorizDir::Right;
        } else if self.x == frame.width - 1 {
            self.horiz_dir = HorizDir::Left;
        }

        if self.y == 0 {
            self.vert_dir = VertDir::Down;
        } else if self.y == frame.height - 1 {
            self.vert_dir = VertDir::Up;
        }
    }

    fn mv(&mut self) {
        match self.horiz_dir {
            HorizDir::Left => self.x -= 1,
            HorizDir::Right => self.x += 1,
        };

        match self.vert_dir {
            VertDir::Up => self.y -= 1,
            VertDir::Down => self.y += 1,
        }
    }
}
