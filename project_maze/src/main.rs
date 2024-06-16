use rand::Rng;

const MAP_SIZE: usize = 25;

//ch1, sec.9 => fn
//ch2, sec.2 => rand
//ch2, sec.3 => Vector
fn make_maze() -> [[usize; MAP_SIZE]; MAP_SIZE] {
    let mut rng = rand::thread_rng();

    let mut maze = [[0; MAP_SIZE]; MAP_SIZE];

    for n in 0..MAP_SIZE {
        maze[n][0] = 1;
        maze[0][n] = 1;
        maze[n][MAP_SIZE-1] = 1;
        maze[MAP_SIZE-1][n] = 1;
    }

    for y in 2..MAP_SIZE-2 {
        for x in 2..MAP_SIZE-2 {
            if x%2 == 1 || y%2 == 1 {
                continue;
            }
            maze[y][x] = 1;

            let r = rng.gen_range(0..=3);
            match r {
                0 => maze[y-1][x] = 1,
                1 => maze[y+1][x] = 1,
                2 => maze[y][x-1] = 1,
                3 => maze[y][x+1] = 1,
                _ => {},
            }
        }
    }

    maze[1][0] = 0;
    maze[MAP_SIZE-2][MAP_SIZE-1] = 0;
    maze
}

fn maze_escapabuility(maze: &[[usize; MAP_SIZE]; MAP_SIZE]) -> bool{
    let mut queue: Vec<[usize; 2]> = vec![[1, 0]];
    let directions:[[usize; 2]; 2] = [[0, 1], [1, 0]];
    let mut result: bool = false;
    while !queue.is_empty() {
        let [y, x]: [usize; 2]  = queue.pop().expect("REASON");
        if y == MAP_SIZE-2 && y == MAP_SIZE-1 {
            result = true;
            break
        }
        if maze[y][x] == 0 {
            for idx in 0..2 {
                let (dy, dx): (usize, usize) = (y+directions[idx][0], x+directions[idx][1]);
                
                if y+dy > 0 && y+dy < MAP_SIZE && x+dx > 0 && x+dx < MAP_SIZE && maze[y+dy][x+dx] == 0 {
                    queue.push([y+dy, x+dx]);
                }
            }
        }
    }
    result
}

fn print_maze(maze: [[usize; MAP_SIZE]; MAP_SIZE]) {
    let tiles = ["□", "■"];
    for y in 0..MAP_SIZE {
        for x in 0..MAP_SIZE {
            print!("{}", tiles[maze[y][x]]);
        }
        println!("");
    }
}

fn main() {
    println!("Hello, world!");
    println!("Wait, now making a maze...");
    let mut tmp_maze: [[usize; MAP_SIZE]; MAP_SIZE] = make_maze();
    let validity = maze_escapabuility(&tmp_maze);
    while validity {
        tmp_maze = make_maze();
    }
    print_maze(tmp_maze);
    println!("Done!");
}