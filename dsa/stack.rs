#[derive(Debug)]
struct Stack<T> {
    items: Vec<T>,
}

impl Stack {
    fn new(&self, items: Vec<T>){
        Stack { items }

    }

    fn push(&self, item: T){
        self.items.push(item);
    }
    fn pop(&self){
        self.items.pop()

    }
    fn peek(&self){
        self.items.last()
    }
    fn is_empty(&self){
        self.items.is_empty()
    }

}