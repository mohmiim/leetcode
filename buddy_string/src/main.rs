use std::collections::HashSet;
fn main() {
    let result = buddy_strings(String::from("ab"), String::from("ab"));
    println!("{result}");
}

pub fn buddy_strings(s: String, goal: String) -> bool {
    if goal.len() != s.len() {
        return false;
    }
    if goal == s {
        return handle_same(&s);
    }
    return hanlde_different(&s, &goal);
}

fn handle_same(s: &String) -> bool {
    let mut contents = HashSet::new();
    for c in s.chars() {
        if contents.contains(&c) {
            return true;
        }
        contents.insert(c);
    }
    return true;
}

fn hanlde_different(s: &String, goal: &String) -> bool {
    let s_cont = s.as_bytes();
    let goal_cont = goal.as_bytes();
    let mut start = usize::MAX;
    let mut end = usize::MIN;
    for index in 0..s_cont.len() {
       if s_cont[index] != goal_cont[index] {
            if start == usize::MAX {
                start = index;
            } else if end == usize::MIN {
                end = index;
            } else {
                return false;
            }
       }
    }
    return  start != usize::MAX && 
            end!= usize::MIN &&
            s_cont[start] == goal_cont[end] &&
            s_cont[end] == goal_cont[start] ;
}


