use rand::Rng;

fn print_maze(h: usize, grid: Vec<u8>) {
    for (i, item) in grid.iter().enumerate() {
        if i % h == 0 {
            println!();
        }
        print!("{}  ", item);
    }
}

fn initialize_maze(w: usize, h: usize, value: u8) -> Vec<u8> {
    vec![value; w * h]
}

fn enumerate_unvisited(w: usize, h: usize, index: usize, maze: &Vec<u8>) -> Vec<usize> {
    let mut possibilities: Vec<usize> = Vec::new();
    //N
    if index >= w && maze[index - w] == 0 {
        possibilities.push(index - w);
    }

    //E
    if index % w < w - 1 && maze[index + 1] == 0 {
        possibilities.push(index + 1);
    }

    //S
    if index < w * (h - 1) && maze[index + w] == 0 {
        possibilities.push(index + w);
    }

    //W
    if index % w != 0 && maze[index - 1] == 0 {
        possibilities.push(index - 1);
    }

    possibilities
}

fn main() {
    let w = 5;
    let h = 5;
    let mut maze: Vec<u8> = initialize_maze(w, h, 0);
    let mut stack: Vec<usize> = vec![0];

    while stack.len() > 0 {
        let stack_index = stack.last().unwrap();
        let unvisited_indexes = enumerate_unvisited(w, h, *stack_index, &maze);
        if unvisited_indexes.len() > 0 {
            let next_index = rand::thread_rng().gen_range(0, unvisited_indexes.len() + 1) - 1;
            maze[next_index] = 0b0001;
            stack.push(next_index);
        } else {
            //End of path
            stack.pop();
        }
    }

    print_maze(h, maze);
}
