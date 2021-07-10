pub struct MinHeap {
    values:Vec<isize>,
    size: usize
    
}
impl MinHeap{
    pub fn empty(capacity: usize) -> Self {
        Self {
            values: Vec::with_capacity(capacity),
            size: 0,
        }
    }
    pub fn new(data: Vec<isize>) -> Self {
        let mut h = MinHeap::empty(data.len()*2);
        data.into_iter().for_each(|elm| h.push(elm));
        return h;
    }
    fn get_left_child_index(&self, parent_index: usize) -> usize {
        return 2*parent_index+1;
    }
    fn get_right_child_index(&self, parent_index: usize) -> usize {
        return 2*parent_index+2;
    }
    fn get_parent_index(&self, child_index: usize) -> Option<usize> {
        if child_index / 2 < 1 { return None;}
        return Some(child_index/2 - 1);
    }
    fn has_left_child(&self, parent_index:usize)->bool {
        return &self.get_left_child_index(parent_index) < &self.size;
    }
    fn has_right_child(&self, parent_index:usize)->bool {
        return &self.get_right_child_index(parent_index) < &self.size;
    }
    fn has_parent(&self, child_index:usize)->bool {
        match &self.get_parent_index(child_index) {
            None => false,
            Some(_) => true
        }
    }
    fn get_left_child(&self, parent_index: usize) -> isize {
        return self.values[self.get_left_child_index(parent_index)];
    }
    fn get_right_child(&self, parent_index: usize) -> isize {
        return self.values[self.get_right_child_index(parent_index)];
    }
    fn get_parent(&self, child_index: usize) -> Option<isize> {
        match self.get_parent_index(child_index) {
            None => None,
            Some(u) =>Some(self.values[u])
        }
    }
    fn swap_index(&mut self,a:usize,b:usize){
        let temp = self.values[a];
        self.values[a]=self.values[b];
        self.values[b]=temp;
    }
    pub fn peek(&self) -> Option<isize> {
        if self.values.len() == 0 { return None;}
        return Some(self.values[0]);
    }
    pub fn poll(&mut self) -> Option<isize> {
        match self.size {
            0 => None,
            1 => self.values.pop(),
            _ => {
                let item = self.values[0];
                self.values[0] = self.values.remove(self.size-1);
                self.size-=1;
                self.heapify_down();
                Some(item)
            }
        }
        
    }
    pub fn push(&mut self, val:isize){
        self.values.push(val);
        self.size+=1;
        self.heapify_up();
    }
    fn heapify_down(&mut self){
        let mut i = 0;
        while self.has_left_child(i) {
            let mut smaller_idx = self.get_left_child_index(i);
            if self.has_right_child(i) && self.get_right_child(i) < self.get_left_child(i) {
                smaller_idx = self.get_right_child_index(i);
            }
            if self.values[i] < self.values[smaller_idx] {break;}
            else {
                self.swap_index(smaller_idx, i);
                i = smaller_idx;
            }
        }
    }
    fn heapify_up(&mut self){
        let mut i = self.size-1;
        while self.has_parent(i) && self.get_parent(i).unwrap_or(self.values[i]-1) > self.values[i] {
            self.swap_index(self.get_parent_index(i).unwrap(),i);
            i = self.get_parent_index(i).unwrap();
        }
    }
}