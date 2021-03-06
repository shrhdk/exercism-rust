pub struct SimpleLinkedList<T> {
    head: Option<Box<Node<T>>>,
}

struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        Self { head: None }
    }

    pub fn len(&self) -> usize {
        let mut size = 0;
        let mut opt_node = &self.head;
        while let Some(ref node) = opt_node {
            opt_node = &node.next;
            size += 1;
        }
        size
    }

    pub fn push(&mut self, _element: T) {
        let mut opt_node = &mut self.head;
        while let Some(ref mut node) = opt_node {
            opt_node = &mut node.next;
        }
        *opt_node = Some(Box::new(Node {
            data: _element,
            next: None,
        }));
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.as_ref()?;

        let mut opt_node = &mut self.head;
        loop {
            if opt_node.as_ref().unwrap().next.is_none() {
                let tail = opt_node.take();
                return Some(tail.unwrap().data);
            } else {
                opt_node = &mut opt_node.as_mut().unwrap().next;
            }
        }
    }

    pub fn peek(&self) -> Option<&T> {
        let mut node = match self.head.as_ref() {
            Some(head) => head,
            None => return None,
        };
        while let Some(ref next) = node.next {
            node = next;
        }
        Some(&node.data)
    }

    pub fn push_front(&mut self, data: T) {
        let new_node = Node {
            data,
            next: self.head.take(),
        };
        self.head = Some(Box::new(new_node));
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.head.as_ref()?;

        let mut old_head = self.head.take().unwrap();
        let new_head = old_head.next.take();
        self.head = new_head;
        Some(old_head.data)
    }
}

impl<T: Clone> SimpleLinkedList<T> {
    pub fn rev(&self) -> Self {
        let mut reversed = Self::new();
        let mut opt_node = &self.head;
        while let Some(ref node) = opt_node {
            reversed.push_front(node.data.clone());
            opt_node = &node.next;
        }
        reversed
    }
}

impl<'a, T: Clone> From<&'a [T]> for SimpleLinkedList<T> {
    fn from(_item: &[T]) -> Self {
        let mut list = Self::new();
        for data in _item {
            list.push(data.clone());
        }
        list
    }
}

impl<T> Into<Vec<T>> for SimpleLinkedList<T> {
    fn into(self) -> Vec<T> {
        let mut vec = Vec::new();
        let mut opt_node = self.head;
        while let Some(node) = opt_node {
            vec.push(node.data);
            opt_node = node.next;
        }
        vec
    }
}
