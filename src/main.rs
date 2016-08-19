// Maze generator in Rust that require user input
// 2016-08-11

extern crate rand;

use std::io;
use std::vec::Vec;

enum Element {
    Wall,
    Passage,
}

struct Maze {
    width:      usize,
    height:     usize,
    data:       Vec<Element>,
}

impl Maze {

    // Create an empty maze structure.
    fn init(width: usize, height: usize) -> Maze {
        let mut maze = Maze {
            width:  width,
            height: height,
            data:   Vec::new(),
        };
        for y in 0 .. height {
            for x in 0 .. width {
                let value =
                    if y == 0 || y == height - 1 {
                        Element::Passage
                    } else if x == 0 || x == width - 1 {
                        Element::Passage
                    } else {
                        Element::Wall
                    };
                maze.data.push(value);
            }
        }
        maze
    }

    // Show the maze.
    fn show(&self) {
        for y in 0 .. self.height {
            for x in 0 .. self.width {
                match self.data[y * self.width + x] {
                   Element::Wall => print!("[]"),
                   Element::Passage => print!("  "),
                }
            }
            println!("")
        }
    }

    fn is_wall(&self, x: isize, y: isize) -> bool {
        let (ux, uy) = (x as usize, y as usize);
        match self.data[(uy * self.width + ux) as usize] {
            Element::Wall => true,
            Element::Passage => false,
        }
    }

    // Carve the maze starting at x, y.
    fn carve<R: rand::Rng>(&mut self, rng: &mut R, x: usize, y: usize) {
        let xdirs = [ 1, -1, 0, 0 ];
        let ydirs = [ 0, 0, 1, -1 ];
        self.data[y * self.width + x] = Element::Passage;
        let d = rng.gen::<usize>() % 4;
        for i in 0 .. 4 {
            let dx: isize = xdirs[(d + i) % 4];
            let dy: isize = ydirs[(d + i) % 4];
            let x2 = (x as isize) + dx;
            let y2 = (y as isize) + dy;
            if self.is_wall(x2, y2) {
                let nx = x2 + dx;
                let ny = y2 + dy;
                if self.is_wall(nx, ny) {
                    let index = (y2 as usize) * self.width + (x2 as usize);
                    self.data[index] = Element::Passage;
                    self.carve(rng, nx as usize, ny as usize);
                }
            }
        }

    }

    // Generate a maze.
    fn generate(width: usize, height: usize) -> Maze {
        let mut maze = Maze::init(width, height);
        let mut rng = rand::thread_rng();
        maze.carve(&mut rng, 2, 2);
        maze.data[1 * width + 2] = Element::Passage;
        maze.data[(height - 2) * width + (width - 3)] = Element::Passage;
        maze
    }

}

fn main() {
	println!("Let's start create a maze!\n");
    println!("- Please input the width (from 3 to 99): ");
    
    let mut width = String::new();

    io::stdin().read_line(&mut width)
        .expect("Failed to read line");

    let mut width: usize = width.trim().parse()
        .expect("Please type a number!");

    if width < 3 || width > 99 {
    	panic!("You must stay in range: from 3 to 99\n");
    }

    println!("\n");
    println!("- Please input the height (from 3 to 99): ");
    
    let mut height = String::new();

    io::stdin().read_line(&mut height)
        .expect("Failed to read line");

    let mut height: usize = height.trim().parse()
        .expect("Please type a number!");

    if height < 3 || height > 99 { panic!("You must stay in range: from 3 to 99\n"); }
    
    println!("\n");

    if width % 2 == 0 { width = width + 1; }
    width = width + 2;

    if height % 2 == 0 { height = height + 1; }
    height = height + 2;

    //let width = 51;     // Maze width; must be odd.
    //let height = 33;    // Maze height; must be odd.
    let maze = Maze::generate(width, height);
    maze.show();
}