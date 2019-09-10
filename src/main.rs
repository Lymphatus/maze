use rand::Rng;
use std::env;
use std::process;

const N: u8 = 0b0001;
const E: u8 = 0b0010;
const S: u8 = 0b0100;
const W: u8 = 0b1000;

fn print_maze(height: usize, grid: &Vec<u8>) {
    for (i, item) in grid.iter().enumerate() {
        if i % height == 0 {
            println!();
        }
        print!("{:04b} ", item);
    }
    println!();
}

fn initialize_maze(width: usize, height: usize, value: u8) -> Vec<u8> {
    vec![value; width * height]
}

fn select_next_index(width: usize, height: usize, index: usize, maze: &Vec<u8>) -> (usize, u8, u8) {
    let mut possibilities: Vec<(usize, u8, u8)> = Vec::new();
    //N
    if index >= width && maze[index - width] == 0b1111 {
        possibilities.push((index - width, N, S));
    }

    //E
    if index % width < width - 1 && maze[index + 1] == 0b1111 {
        possibilities.push((index + 1, E, W));
    }

    //S
    if index < width * (height - 1) && maze[index + width] == 0b1111 {
        possibilities.push((index + width, S, N));
    }

    //W
    if index % width != 0 && maze[index - 1] == 0b1111 {
        possibilities.push((index - 1, W, E));
    }

    if possibilities.len() > 0 {
        let next_index = rand::thread_rng().gen_range(0, possibilities.len());
        return possibilities[next_index];
    }

    (0 as usize, 0, 0)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Invalid arguments number.");
        process::exit(1);
    }

    let width: usize = args[1].trim().parse().expect("Width must be an unsigned integer value");
    let height: usize = args[2].trim().parse().expect("Height must be an unsigned integer value");
    let mut maze: Vec<u8> = initialize_maze(width, height, 0b1111);
    let mut stack: Vec<usize> = vec![0; 1];

    while stack.len() > 0 {
        let stack_index = *stack.last().unwrap();
        let next_stack_index = select_next_index(width, height, stack_index, &maze);
        if next_stack_index.0 > 0 {
            let next_index = next_stack_index.0;
            maze[stack_index] ^= next_stack_index.1;
            maze[next_index] ^= next_stack_index.2;
            stack.push(next_index);
        } else {
            //End of path
            stack.pop();
        }
//        print_maze(height, &maze);
//        println!();
    }

    print_maze(height, &maze);
}
