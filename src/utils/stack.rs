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

    pub fn pop(&mut self) -> Result<i32, &str> {
        if self.is_empty() {
            return Err("Can't pop from stack because it is empty")
        }
        self.pointer -= 1;
        Ok(self.buffer[self.pointer])
    }

    pub fn push(&mut self, value: i32) -> Result<&mut Stack, &str> {
        if self.is_full() {
            return Err("Can't push on stack because it is full")
        }
        self.buffer[self.pointer] = value;
        self.pointer += 1;
        Ok(self)
    }

    fn is_full(&self,) -> bool {
        self.pointer == SIZE
    }

    fn is_empty(&self) -> bool {
        self.pointer == 0
    }

    pub fn size(&self) -> usize {
        self.pointer
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
        let _ = stack.push(value);
        assert_eq!(stack.buffer[0], value);
    }

    #[test]
    fn pushing_on_stack_should_increment_pointer() {
        //given: a new stack
        let mut stack = Stack::new();
        let _ = stack.push(15);
        assert_eq!(stack.pointer, 1);
    }

    #[test]
    fn popping_from_stack_should_return_correct_value() {
        //given: a new stack
        let mut stack = Stack::new();
        stack.pointer = 3;
        stack.buffer = [11,12,13,0];
        assert_eq!(stack.pop(), Ok(13));
        assert_eq!(stack.pop(), Ok(12));
        assert_eq!(stack.pop(), Ok(11));
    }

    #[test]
    fn popping_from_stack_should_decrement_pointer() {
        //given: a new stack
        let mut stack = Stack::new();
        stack.pointer = 1;
        let _ = stack.pop();
        assert_eq!(stack.pointer, 0);
        stack.pointer = 2;
        let _ = stack.pop();
        assert_eq!(stack.pointer, 1);
    }

    #[test]
    fn pushing_once_in_the_stack_results_in_expected_size() {
        //given: a new stack
        let mut stack = Stack::new();
        let _ = stack.push(1);
        let _ = stack.push(2);
        assert_eq!(stack.size(), 2);
    }

    #[test]
    fn stack_is_empty_should_return_empty_state() {
        //given: a new stack
        let mut stack = Stack::new();
        assert_eq!(stack.is_empty(), true);
        let _ = stack.push(5);
        assert_eq!(stack.is_empty(), false);
        let _ = stack.pop();
        assert_eq!(stack.is_empty(), true);
    }

    #[test]
    fn stack_is_full_should_return_full_state() {
        //given: a new stack
        let mut stack = Stack::new();
        assert_eq!(stack.is_full(), false);
        let _ = stack.push(5);
        let _ = stack.push(5);
        let _ = stack.push(5);
        let _ = stack.push(5);
        assert_eq!(stack.is_full(), true);
        let _ = stack.pop();
        assert_eq!(stack.is_full(), false);
    }

}