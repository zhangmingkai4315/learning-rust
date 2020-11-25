
pub mod double_linklist{
    use std::rc::Rc;
    use std::cell::RefCell;

    // 使用RefCell实现内部可变性
    #[derive(Clone)]
    struct Node<T>{
        val: T,
        next: Option<Rc<RefCell<Node<T>>>>,
        prev: Option<Rc<RefCell<Node<T>>>>,
    }

    impl<T> Node<T>{
        fn new(value: T) -> Rc<RefCell<Node<T>>> {
            Rc::new(RefCell::new(Node {
                val: value,
                next: None,
                prev: None,
            }))
        }
    }

    pub struct DoubleLinkList<T>{
        head: Option<Rc<RefCell<Node<T>>>>,
        tail: Option<Rc<RefCell<Node<T>>>>,
        length: u64,
    }

    impl<T> DoubleLinkList<T> where T: Clone {
        pub fn new_empty()->DoubleLinkList<T>{
            DoubleLinkList{
                head: None,
                tail: None,
                length: 0,
            }
        }

        pub fn len(&self) -> u64{
            self.length
        }

        pub fn append(&mut self, value: T){
            let new = Node::new(value.clone());
            match self.tail.take() {
                Some(old) => {
                    old.borrow_mut().next = Some(new.clone());
                    new.borrow_mut().prev = None;
                },
                None => self.head = Some(new.clone()),
            }
            self.length += 1;
            self.tail = Some(new)
        }

        pub fn pop(&mut self) -> Option<T>{
            self.head.take().map(|head|{
                if let Some(next) = head.borrow_mut().next.take(){
                    next.borrow_mut().prev = None;
                    self.head = Some(next);
                }else{
                    self.tail.take();
                }
                self.length -=1;
                Rc::try_unwrap(head).ok().expect("something wrong").into_inner().val
            })
        }

    }

    pub struct DoubleLinkListIterator<T>
    {
        current: Option<Rc<RefCell<Node<T>>>>
    }

    impl<T> DoubleLinkListIterator<T>{
        fn new(start: Option<Rc<RefCell<Node<T>>>>) -> DoubleLinkListIterator<T>{
            DoubleLinkListIterator{
                current: start,
            }
        }
    }

    impl<T> Iterator for DoubleLinkListIterator<T> where T: Clone{
        type Item = T;
        fn next(&mut self)->Option<T>{
            let current = &self.current;
            let mut result = None;
            self.current = match current {
                Some(ref current) => {
                    let current = current.borrow();
                    result = Some(current.val.clone());
                    current.next.clone()
                },
                None => None
            };
            result
        }
    }

    impl<T> DoubleEndedIterator for DoubleLinkListIterator<T> where T: Clone{
        fn next_back(&mut self) -> Option<T>{
            let current = &self.current;
            let mut result = None;

            self.current = match current {
                Some(ref current) => {
                    let current = current.borrow();
                    result = Some(current.val.clone());
                    current.prev.clone()
                },
                None => None
            };
            result
        }
    }

}

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn test_double_linklist_string(){
        let mut ll = double_linklist::DoubleLinkList::new_empty();
        ll.append("hello1".to_string());
        ll.append("hello2".to_string());
        ll.append("hello3".to_string());
        assert_eq!(ll.len(), 3);
        let expect = ll.pop();
        assert_eq!(expect.unwrap(), "hello1".to_string());
        let expect = ll.pop();
        assert_eq!(expect.unwrap(), "hello2".to_string());
        let expect = ll.pop();
        assert_eq!(expect.unwrap(), "hello3".to_string());
        assert_eq!(ll.len(), 0);
        assert!(ll.pop() == None);
    }

    #[test]
    fn test_double_linklist_i32(){
        let mut ll = double_linklist::DoubleLinkList::new_empty();
        ll.append(1);
        ll.append(2);
        ll.append(3);
        assert_eq!(ll.len(), 3);
        let expect = ll.pop();
        assert_eq!(expect.unwrap(), 1);
        let expect = ll.pop();
        assert_eq!(expect.unwrap(), 2);
        let expect = ll.pop();
        assert_eq!(expect.unwrap(), 3);
        assert_eq!(ll.len(), 0);
        assert!(ll.pop() == None);
    }
}