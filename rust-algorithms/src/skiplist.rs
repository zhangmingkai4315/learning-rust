pub mod skiplist {
    use rand::*;
    use std::cell::RefCell;
    use std::rc::Rc;
    type RealNode = Rc<RefCell<Node>>;
    type Link = Option<RealNode>;
    struct Node {
        data: String,
        next: Vec<Link>,
        // 可以通过offset比较具体的偏移量
        offset: u64,
    }

    impl Node {
        fn new(next: Vec<Link>, offset: u64, data: String) -> RealNode {
            Rc::new(RefCell::new(Node { next, offset, data }))
        }
    }

    pub struct SkipList {
        head: Link,
        tails: Vec<Link>,
        max_level: usize,
        length: u64,
    }

    impl SkipList {
        pub fn new(level: usize) -> SkipList {
            SkipList {
                head: None,
                tails: vec![None; level],
                max_level: level - 1,
                length: 0,
            }
        }

        fn random_level(&self) -> usize {
            let mut n = 0;
            while random::<bool>() && n < self.max_level {
                n += 1;
            }
            n
        }

        pub fn append(&mut self, offset: u64, data: String) {
            // 初始化时级别设置为最大， 否则使用random_level
            let level = 1 + if self.head.is_none() {
                self.max_level
            } else {
                self.random_level()
            };

            let node = Node::new(vec![None; level], offset, data);

            for i in 0..level {
                // 当插入新的元素的时候，更新当前的next指针，指向新的元素
                if let Some(old) = self.tails[i].take() {
                    let next = &mut old.borrow_mut().next;
                    next[i] = Some(node.clone());
                }
                self.tails[i] = Some(node.clone());
            }

            if self.head.is_none() {
                self.head = Some(node.clone());
            }
            self.length += 1;
        }

        pub fn find(&self, offset: u64) -> Option<String> {
            match self.head {
                Some(ref head) => {
                    let mut start_level = self.max_level - 1;
                    // Rc类型 clone只更新其引用计数
                    let node = head.clone();
                    let mut result = None;
                    // 定位到存在start_level的位置
                    loop {
                        if node.borrow().next[start_level].is_some() {
                            break;
                        }
                        start_level -= 1;
                    }
                    // 从该级别起逐步查找符合的节点
                    let mut n = node;
                    for level in (0..=start_level).rev() {
                        loop {
                            let next = n.clone();
                            match next.borrow().next[level] {
                                Some(ref tmp) => {
                                    if tmp.borrow().offset <= offset {
                                        n = tmp.clone();
                                    } else {
                                        break;
                                    }
                                }
                                _ => break,
                            };
                        }
                        if n.borrow().offset == offset {
                            let tmp = n.borrow();
                            result = Some(tmp.data.clone());
                            break;
                        }
                    }
                    result
                }
                None => None,
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_skip_list() {
        let mut skl = skiplist::SkipList::new(5);
        for i in 1..1000 {
            skl.append(i, format!("data-{}", i));
        }
        for i in 1..1000 {
            assert_eq!(skl.find(i), Some(format!("data-{}", i)));
        }
    }
}
