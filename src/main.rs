fn count_neighbors(board: &Vec<Vec<bool>>, x: i32, y: i32) -> u8 {
    let neighbor_diffs = [(-1,-1), (-1,0), (-1,1), (0,-1), (0,1), (1,-1), (1,0), (1,1)];
    let mut neighbors = 0;
    for &(a,b) in &neighbor_diffs {
        let new_x: i32 = (x + a + 16) % 16;
        let new_y: i32 = (y + b + 16) % 16;
        if board[new_x as usize][new_y as usize] {
            neighbors += 1;
        }
    }
    neighbors
}

fn is_alive(board: &Vec<Vec<bool>>, x: usize,y: usize) -> bool {
    let neighbors = count_neighbors(board, x as i32,y as i32);
    let mut status = board[x][y];
    match neighbors {
        2 => status = status,
        3 => status = true,
        _ => status = false,
    }
    status
}

fn next_gen(current_gen: Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    let mut next_gen: Vec<Vec<bool>> = current_gen.clone();
    for x in 0..16 {
        for y in 0..16{
            next_gen[x as usize][y as usize] = is_alive(&current_gen, x,y);
        }
    }
    next_gen
}

fn print_game(board: Vec<Vec<bool>>) {
    for x in 0..16 {
        for y in 0..16 {
            match board[x][y] {
                true => print!(" LIVE {} ", count_neighbors(&board,x as i32,y as i32)),
                false => print!(" dead {} ", count_neighbors(&board,x as i32,y as i32)),
            }
        }
        println!();
    }
}

fn test_status_dead_to_live() {
    let mut temp_board: Vec<Vec<bool>> = vec![vec![false; 16 as usize]; 16 as usize];
    temp_board[1][1] = false;
    let temp_live_cells = [(0,0),(0,1),(0,2)];
    for (x,y) in temp_live_cells {
        temp_board[x][y] = true
    }
    assert_eq!(is_alive(&temp_board, 1,1), true, "Dead did not update to Live");
}

fn test_status_live_to_live() {
    let mut temp_board: Vec<Vec<bool>> = vec![vec![false; 16 as usize]; 16 as usize];
    temp_board[1][1] = true;
    let temp_live_cells = [(0,0),(0,1)];
    for (x,y) in temp_live_cells {
        temp_board[x][y] = true
    }
    assert_eq!(is_alive(&temp_board, 1,1), true, "Live did not remain Live");
    temp_board[0][2] = true;
    assert_eq!(is_alive(&temp_board, 1,1), true, "Live did not remain Live");
}


fn test_status_live_to_dead() {
    let mut temp_board: Vec<Vec<bool>> = vec![vec![false; 16 as usize]; 16 as usize];
    temp_board[1][1] = true;
    assert_eq!(is_alive(&temp_board, 1,1), false, "Live did not update to Dead");
    temp_board[0][0] = true;
    assert_eq!(is_alive(&temp_board, 1,1), false, "Live did not update to Dead");
    let temp_live_cells = [(0,1),(0,2), (1,0), (1,2)];
    for (x,y) in temp_live_cells {
        temp_board[x][y] = true
    }
    assert_eq!(is_alive(&temp_board, 1,1), false, "Live did not update to Dead");
}

fn test_status_dead_to_dead() {
    let mut temp_board: Vec<Vec<bool>> = vec![vec![false; 16 as usize]; 16 as usize];
    assert_eq!(is_alive(&temp_board, 1,1), false, "Dead did not remain Dead");
    let temp_live_cells = [(0,0),(0,1), (0,2), (1,0)];
    for (x,y) in temp_live_cells {
        temp_board[x][y] = true
    }
    assert_eq!(is_alive(&temp_board, 1,1), false, "Dead did not remain Dead");
}


fn main() {
    test_status_dead_to_live();
    test_status_live_to_live();
    test_status_live_to_dead();
    test_status_dead_to_dead();
    
    let mut board:Vec<Vec<bool>> = vec![vec![false ;16 as usize]; 16 as usize];
    let starting_cells = [(0,1), (1,2), (2,0), (2,1), (2,2)];
    for &(x,y) in &starting_cells {
        board[x][y] = true;
    }

    for i in 1..11 {
        println!("Generation: {}", i);
        print_game(board.clone());
        board = next_gen(board.clone());

    }

}
