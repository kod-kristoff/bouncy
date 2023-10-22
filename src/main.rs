use std::fmt::{Display, Formatter};
mod parse_args;

fn main() -> Result<(), parse_args::ParseError> {
    let window = pancurses::initscr();
    let (max_y, max_x) = window.get_max_yx();
    println!("max = ({}, {})", max_x, max_y);
    let frame = Frame {
        width: (max_x - 4) as u32,
        height: (max_y - 4) as u32
    };
    let mut game = Game::new(frame);
    let sleep_duration = std::time::Duration::from_millis(33);
    window.draw_box('|', '-');
    loop {
        window.mvaddch(
            game.ball.y as i32,
            game.ball.x as i32,
            ' ');
        game.step();
        window.mvaddch(
            game.ball.y as i32,
            game.ball.x as i32,
            'o'
        );
        window.refresh();
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
    fn new(frame: Frame) -> Game {
        let ball = Ball {
            x: 2,
            y: 4,
            vert_dir: VertDir::Up,
            horiz_dir: HorizDir::Left,
        };
        Game { frame, ball }
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
            write!(f, "+\r\n")
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
            write!(f, "|\r\n")?;
        }
        top_bottom(f)
    }
}

impl Ball {
    fn bounce(&mut self, frame: &Frame) {
        if self.x == 1 {
            self.horiz_dir = HorizDir::Right;
        } else if self.x == frame.width - 1 {
            self.horiz_dir = HorizDir::Left;
        }

        if self.y == 1 {
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
