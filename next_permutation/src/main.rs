fn main() {
    println!("Hello, world!");
    let mut nums = vec![5,4,7,5,3,2];
    next_permutation(&mut nums);
    for x in &nums {
        print!("{x} ");
    }
}

pub fn next_permutation(nums: &mut Vec<i32>) {
    let mut start_swap = usize::MAX;
    for index in (1..nums.len()).rev() {
        if nums[index] > nums[index-1] {
            start_swap = index-1;
            break;
        }
    }
    if start_swap != usize::MAX {
        let mut swap_with = start_swap;
        for index in start_swap+1 .. nums.len() {
            if nums[index] > nums[start_swap] {
                swap_with = index;
            } else {
                break;
            }
        }
        swap(nums, start_swap, swap_with);
        reverse(nums, start_swap+1);
    } else {
        reverse(nums, 0);
    }
    
}

fn swap(nums: &mut Vec<i32>, source: usize, target: usize){
    let temp = nums[source];
    nums[source] = nums[target];
    nums[target] = temp;
}

fn reverse(nums: &mut Vec<i32>, start: usize) {
    let mut counter = 0;
    for index in (start..nums.len()).rev() {
        if index <= counter+counter {
            break;
        }
        swap(nums, index, start+counter);
        counter = counter+1;
    }
}

