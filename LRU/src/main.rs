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


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
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



/*
  Your LRUCache object will be instantiated and called as such:
  let obj = LRUCache::new(capacity);
  let ret_1: i32 = obj.get(key);
  obj.put(key, value);
 */



/*

class LRUCache {
    
    public Map<Integer,Node> cache = new HashMap<>();
    public Node head = null;
    public Node tail = null;
    int capacity = 0;
    
    static private class Node {
        public Node next = null;
        public Node previous = null;
        public int val = 0;
        public int key = 0;
    }

    public LRUCache(int capacity) {
        this.capacity = capacity;
    }
    
    public int get(int key) {
        Node element = cache.get(key);
        if (element!=null){
            if (element!=head){
                removeLRU(element);
                makeMRU(element);
            }
            return element.val;
        }
        return -1;
    }
    
    public void put(int key, int value) {
        Node element = cache.get(key);
        if (element!=null){
            if (element!=head){
                removeLRU(element);
            }
            element.val = value;
        }else {
            if (cache.size()==capacity){
                cache.remove(tail.key);
                removeLRU(tail);
            }
            element = new Node();
            element.key = key;
            element.val = value;
            cache.put(key,element);
        }
        if (element!=head) makeMRU(element);
     }
    
    public void  removeLRU( Node element) {
        if (element!=null) {
            if(element.previous!=null)  element.previous.next = element.next;
            if(element.next!=null)  element.next.previous = element.previous;
        }
        if (element==head){
            head = element.next;
        }
        if (element==tail){
            tail = element.previous;
        }
   }
    
    public void  makeMRU( Node element) {
        element.next = head;
        if (head!=null) head.previous = element;
        head = element;
        if (tail==null){
            tail = head;
        }
    }
}


*/