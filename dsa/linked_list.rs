#[derive(Debug)]
struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

#[derive(Debug)]
struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
}


impl<T> LinkedList<T>
where
    T: std::fmt::Debug + PartialEq,
{
    fn new() -> Self {
        LinkedList { head: None }
    }

    fn display(&self) {
        let mut current = &self.head;
        let mut elements = vec![];
        while let Some(node) = current {
            elements.push(format!("{:?}", node.data));
            current = &node.next;
        }
        println!("{}", if elements.is_empty() {
            "Empty List".to_string()
        } else {
            elements.join(" -> ")
        });
    }

    fn insert(&mut self, data: T) {
        let new_node = Box::new(Node { data, next: None });
        if self.head.is_none() {
            self.head = Some(new_node);
            return;
        }
        
        let mut current = self.head.as_mut().unwrap();
        while current.next.is_some() {
            current = current.next.as_mut().unwrap();
        }
        current.next = Some(new_node);
    }
    
    fn search(&self, key: &T) -> bool {
        let mut current = &self.head;
        while let Some(node) = current {
            if &node.data == key {
                return true;
            }
            current = &node.next;
        }
        false
    }
    
    fn update(&mut self, old_data: &T, new_data: T) -> bool {
        let mut current = self.head.as_mut();
        while let Some(node) = current {
            if &node.data == old_data {
                node.data = new_data;
                return true;
            }
            current = node.next.as_mut();
        }
        false
    }
    
    fn delete(&mut self, key: &T) -> bool {
        let mut current = &mut self.head;
        
        loop {
            match current {
                None => return false,
                Some(node) if &node.data == key => {
                    *current = node.next.take();
                    return true;
                }
                Some(node) => {
                    current = &mut node.next;
                }
            }
        }
    }
    fn reverse(&mut self) {
        let mut prev: Option<Box<Node<T>>> = None;
        let mut current = self.head.take();
    
        while let Some(mut node) = current {
            let next = node.next.take();  // Save next
            node.next = prev;             // Point backward
            prev = Some(node);            // Move prev forward
            current = next;               // Move current forward
        }
    
        self.head = prev;
    }        
}

/**
 * Reverse Linked List
 * Time Complexity: O(n)
 * Space Complexity: O(1)
 * 
 * Example:
 * let mut list = LinkedList::new();
 * list.insert(1);
 * list.insert(2);
 * list.insert(3);
 * list.reverse();
 * list.display();
 */