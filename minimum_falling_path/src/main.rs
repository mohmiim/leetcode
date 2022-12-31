fn main() {
    let matrix = vec![vec![-19,57],vec![-40,-5]];
    println!("{}",min_falling_path_sum(matrix));
}

pub fn min_falling_path_sum(matrix: Vec<Vec<i32>>) -> i32 {
    let columns = matrix[0].len();
    let mut memory: Vec<Vec<i32>> = Vec::new();
    for row in 0..matrix.len() {
        memory.insert(row as usize, vec![i32::MIN; columns]);
    }
    let mut min = i32::MAX;
    for column in 0..columns {
        let current =  calculate_sum(0, column, &matrix, &mut memory);
        if current < min {
            min = current;
        }
    }
    min
}

fn calculate_sum(row: usize, column: usize, matrix: &Vec<Vec<i32>>, memory: &mut Vec<Vec<i32>>) -> i32 {
    if  row >= matrix.len() || 
        column >= matrix[0].len()  {
            return i32::MAX;
    }
    let val = matrix[row][column];
    // last line
    if row+1 == matrix.len() {
        return val;
    }
    let calcuated = memory[row][column];
    if calcuated != i32::MIN {
        return calcuated;
    }
    let mut to_add;
    let mut min = i32::MAX;
    if column > 0 {
        to_add = calculate_sum(row+1, column-1, matrix, memory);
        if to_add != -1 {
            min = to_add;
        }
    }

    to_add = calculate_sum(row+1, column, matrix, memory);
    if to_add < min {
        min = to_add;
    }

    to_add = calculate_sum(row+1, column+1, matrix, memory);
    if to_add<min {
        min = to_add;
    }

    memory[row][column] = val + min;
    return val + min;
}


