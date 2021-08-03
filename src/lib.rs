use std::rc::Rc;

pub struct CLList<T> {
    items:  Vec<Rc<T>>,
    index:  usize
}

impl<T> CLList<T> {
    pub fn new() -> Self {
        CLList { items: Vec::new(), index: 0 }
    }

    pub fn with_capacity(capacity:  usize) -> Self {
        CLList { items: Vec::with_capacity(capacity), index: 0 }
    }

    pub fn capacity(&self) -> usize {
        self.items.capacity()
    }

    pub fn push(&mut self, value: T, replecas: usize) {
        let sourc = Rc::new(value);
        for _ in 0..replecas {
            self.items.push(Rc::clone(&sourc));
        }
    }

    pub fn next(&mut self) -> Option<&Rc<T>> {
        let original_index = self.index;
        self.index = (self.index + 1) % self.items.len();
        self.items.get(original_index)
    }
}
