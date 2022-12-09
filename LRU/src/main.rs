fn main() {
    println!("Hello, world!");
    let mut obj = LRUCache::new(5);
    let ret_1: i32 = obj.get(5);
    obj.put(5, 10);
}

struct LRUCache {
    cach: Vec<Node>,
    capacity: i32,
}

impl LRUCache {

    fn new(capacity: i32) -> Self {
        LRUCache { cach: vec![], capacity: capacity }
    }
    
    fn get(&mut self, key: i32) -> i32 {
        let mut found_index = usize::MAX;
        for index in 0..self.cach.len() {
            let found_node = &self.cach[index];
            if found_node.key == key {
                found_index = index;
                break;
            }
        }
        if found_index != usize::MAX {
            let node = self.cach.remove(found_index);
            let val = node.val;
            self.cach.insert(0, node);
            return val;
        }
        return -1;
    }
    
    fn put(&mut self, key: i32, value: i32) {
        let mut found_index = usize::MAX;
        for index in 0..self.cach.len() {
            let found_node = &self.cach[index];
            if found_node.key == key {
                found_index = index;
                break;
            }
        }
        if found_index != usize::MAX {
            let mut node = self.cach.remove(found_index);
            node.val = value;
            self.cach.insert(0, node);
        }else {
            // capacity check becuae we will insert 
            let capacity_usize = self.capacity as usize;
            if capacity_usize == self.cach.len() {
                self.cach.pop();
            }
            let node = Node {key:key, val:value};
            self.cach.insert(0, node);
        }
    }
}

struct Node {
    val : i32,
    key :i32,
}
