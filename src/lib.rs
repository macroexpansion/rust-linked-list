#[derive(Debug)]
pub struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    pub fn new(value: T) -> Node<T> {
        Node { value, next: None }
    }

    pub fn with_next(mut self, next: Node<T>) -> Node<T> {
        self.next = Some(Box::new(next));

        self
    }
}

impl<T> Drop for Node<T> {
    fn drop(&mut self) {
        while let Some(next) = self.next.take() {
            *self = *next;
        }
        // println!("dropped");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stack_overflow() {
        let mut head = Node::new(0);
        for i in 0..10 {
            head = Node::new(i).with_next(head);
        }
        drop(head); // boom

        /* println!("{:?}", head.value);

        if let Some(node) = &head.next {
            println!("{:?}", node.value);
        } */
    }
}
