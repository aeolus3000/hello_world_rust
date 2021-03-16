const SIZE: usize = 4;

pub struct Stack {
    buffer: [i32; SIZE],
    pointer: usize
}

impl Stack {
    pub fn new() -> Stack {
        let buffer: [i32; SIZE] = [0; SIZE];
        let pointer: usize = 0;
        let stack = Stack {
            buffer,
            pointer,
        };
        stack
    }

    pub fn pop(&mut self) -> i32 {
        self.pointer -= 1;
        self.buffer[self.pointer]
    }

    pub fn push(&mut self, value: i32) {
        self.buffer[self.pointer] = value;
        self.pointer += 1;
    }

    fn is_full(&self,) {

    }

    fn is_empty(&self) {

    }

    pub fn size(&self) -> i32 {
        0
    }
}

#[cfg(test)]
mod stack_test {
    use super::*;

    #[test]
    fn a_new_stack_should_be_empty() {
        //given: a new stack
        let stack = Stack::new();
        assert_eq!(stack.size(), 0);
    }

    #[test]
    fn pushing_on_stack_should_be_visible_in_buffer() {
        let value = 15;
        //given: a new stack
        let mut stack = Stack::new();
        assert_eq!(stack.buffer[0], 0);
        stack.push(value);
        assert_eq!(stack.buffer[0], value);
    }

    #[test]
    fn pushing_on_stack_should_increment_pointer() {
        //given: a new stack
        let mut stack = Stack::new();
        stack.push(15);
        assert_eq!(stack.pointer, 1);
    }

    #[test]
    fn popping_from_stack_should_return_correct_value() {
        //given: a new stack
        let mut stack = Stack::new();
        stack.pointer = 3;
        stack.buffer = [11,12,13,0];
        assert_eq!(stack.pop(), 13);
        assert_eq!(stack.pop(), 12);
        assert_eq!(stack.pop(), 11);
    }

    #[test]
    fn popping_from_stack_should_decrement_pointer() {
        //given: a new stack
        let mut stack = Stack::new();
        stack.pointer = 1;
        stack.pop();
        assert_eq!(stack.pointer, 0);
        stack.pointer = 2;
        stack.pop();
        assert_eq!(stack.pointer, 1);
    }

}