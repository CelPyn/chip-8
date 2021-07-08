use std::collections::VecDeque;

const MAX_STACK_SIZE: usize = 16;


pub struct Stack {
    content: VecDeque<usize>
}

impl Stack {

    pub fn new() -> Self {
        Stack {
            content: VecDeque::with_capacity(MAX_STACK_SIZE)
        }
    }

    pub fn push(&mut self, v:usize) {
        if self.content.len() == MAX_STACK_SIZE {
            panic!("Attempted to push value to stack but stack was full!")
        }
        self.content.push_front(v);
    }

    pub fn pop(&mut self) -> usize {
        match self.content.pop_front() {
            None => panic!("Attempted to get value from stack but stack was empty!"),
            Some(value) => value
        }
    }

}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[should_panic(expected = "Attempted to get value from stack but stack was empty!")]
    fn test_pop_on_empty() {
        let mut stack = Stack::new();
        stack.pop();
    }

    #[test]
    #[should_panic(expected = "Attempted to push value to stack but stack was full!")]
    fn test_push_on_full() {
        let mut stack = Stack::new();

        for i in 0..16 {
            stack.push(i);
        }

        assert_eq!(stack.content[0], 15);
        stack.push(0);
    }

    #[test]
    fn test_pop_on_one_element() {
        let mut stack = Stack::new();
        stack.push(0xFF);
        let result = stack.pop();
        assert_eq!(result, 0xFF);
    }

    #[test]
    fn test_pop_on_multiple_elements() {
        let mut stack = Stack::new();
        stack.push(0xFF);
        stack.push(0xFA);
        let result = stack.pop();
        assert_eq!(result, 0xFA);
        assert_eq!(stack.content.len(), 1);
        assert_eq!(stack.content[0], 0xFF);
    }
}