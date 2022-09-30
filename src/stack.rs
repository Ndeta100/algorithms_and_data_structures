pub struct Stack<T> {
    max_size: usize,
    items: Vec<T>,
}

impl<T> Stack<T> {
    pub fn with_capacity(max_size: usize) -> Self {
        Self {
            max_size,
            items: Vec::with_capacity(max_size),
        }
    }
    pub fn pop(&mut self) -> Option<T> {
        self.items.pop()
    }
    pub fn push(&mut self, item: T) -> bool {
        if self.items.len() == self.max_size {
            return false;
        }
        self.items.push(item);
        true
    }
    pub fn size(&self) -> usize {
        self.items.len()
    }
    pub fn peek(&self) -> Option<&T> {
        self.items.last()
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_new_capacity() {
        let stack = Stack::<i32>::with_capacity(10);
        assert_eq!(10, stack.items.capacity());
    }
    #[test]
    fn test_pop() {
        let mut stack = Stack::<u32>::with_capacity(1);
        stack.push(1u32);
        assert_eq!(Some(1u32), stack.pop());
        assert_eq!(None, stack.pop());
    }
}
