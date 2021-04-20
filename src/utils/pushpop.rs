pub trait PushPop {
    fn push(&mut self, item: i32) -> Result<&mut dyn PushPop, &str>;
    fn pop(&mut self) -> Result<i32, &str>;
    fn size(&self) -> usize;
    fn is_empty(&self) -> bool;
    fn is_full(&self) -> bool;
}