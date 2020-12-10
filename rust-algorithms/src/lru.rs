use std::cell::{Ref, RefCell};
use std::collections::HashMap;
use std::rc::Rc;

type OptionEntry<K, V> = Option<Rc<RefCell<Entry<K, V>>>>;

#[derive(Debug, Clone, PartialEq)]
pub struct Entry<K, V> {
    val: V,
    key: K,
    left: OptionEntry<K, V>,
    right: OptionEntry<K, V>,
}

impl<K, V> Entry<K, V> {
    pub fn new(k: K, v: V) -> Rc<RefCell<Entry<K, V>>> {
        Rc::new(RefCell::new(Entry {
            val: v,
            key: k,
            left: None,
            right: None,
        }))
    }
}

pub struct LRUCache<K, V> {
    head: OptionEntry<K, V>,
    tail: OptionEntry<K, V>,
    hashmap: HashMap<K, Rc<RefCell<Entry<K, V>>>>,
    size: usize,
    current: usize,
}

impl<K, V> LRUCache<K, V> {
    pub fn new(size: usize) -> LRUCache<K, V> {
        LRUCache {
            size,
            current: 0,
            head: None,
            tail: None,
            hashmap: HashMap::new(),
        }
    }

    pub fn get(&mut self, k: K) -> OptionEntry<K, V>
    where
        K: std::cmp::Eq + std::hash::Hash + Clone,
    {
        if let Some(entry) = self.hashmap.get(&k.clone()) {
            let entry = entry.clone();
            self.remove_entry(entry.clone());
            self.push_front(entry.clone());
            return Some(entry.clone());
        }
        return None;
    }

    pub fn put(&mut self, k: K, v: V)
    where
        K: std::cmp::Eq + std::hash::Hash + Clone,
    {
        match self.hashmap.get(&k) {
            Some(entry) => {
                let entry = entry.clone();
                self.remove_entry(entry.clone());
                self.push_front(entry.clone());
            }
            _ => {
                let entry = Entry::new(k.clone(), v);
                if self.current >= self.size {
                    // 删除原来的尾部元素
                    self.remove_entry(self.tail.as_ref().unwrap().clone());
                }
                self.push_front(entry.clone());
                self.hashmap.insert(k.clone(), entry.clone());
            }
        }
    }

    fn push_front(&mut self, node: Rc<RefCell<Entry<K, V>>>)
    where
        K: std::cmp::Eq + std::hash::Hash + Clone,
    {
        match self.head.take() {
            Some(mut old_start) => {
                old_start.borrow_mut().left = Some(node.clone());
                node.borrow_mut().right = Some(old_start.clone());
                node.borrow_mut().left = None;
            }
            _ => {
                self.tail = Some(node.clone());
            }
        }
        self.current += 1;
        self.head = Some(node.clone());
        self.hashmap
            .insert(node.borrow_mut().key.clone(), node.clone());
    }

    fn remove_entry(&mut self, node: Rc<RefCell<Entry<K, V>>>)
    where
        K: std::cmp::Eq + std::hash::Hash + Clone,
    {
        let mut temp = node.borrow_mut();
        if let Some(left) = temp.left.take() {
            if temp.right.is_none() {
                self.tail = Some(left.clone());
            }
            left.borrow_mut().right = temp.right.take();
        }
        //
        if let Some(right) = temp.right.take() {
            if temp.left.is_none() {
                self.head = Some(right.clone());
            }
            right.borrow_mut().left = temp.left.take();
        }
        self.current -= 1;
        self.hashmap.remove(&temp.key.clone());
    }
}

pub struct LRUCacheIterator<K, V> {
    current: Option<Rc<RefCell<Entry<K, V>>>>,
}

impl<K, V> LRUCacheIterator<K, V> {
    fn new(start: Option<Rc<RefCell<Entry<K, V>>>>) -> LRUCacheIterator<K, V> {
        LRUCacheIterator { current: start }
    }
}
impl<K, V> Iterator for LRUCacheIterator<K, V>
where
    K: Clone,
    V: Clone,
{
    type Item = (K, V);
    fn next(&mut self) -> Option<(K, V)> {
        let current = &self.current;
        let mut result = None;
        self.current = match current {
            Some(ref current) => {
                let current = current.borrow();
                result = Some((current.key.clone(), current.val.clone()));
                current.right.clone()
            }
            None => None,
        };
        result
    }
}

#[cfg(test)]
mod test {
    use crate::lru::{Entry, LRUCache, LRUCacheIterator};
    use std::cell::{Ref, RefCell};
    use std::rc::Rc;

    #[test]
    fn lru_fn_test() {
        let mut cache = LRUCache::new(10);
        cache.push_front(Entry::new("hello", 1));
        if let Some(v) = cache.hashmap.get("hello") {
            assert_eq!(v.borrow().val, 1);
        }
        if let Some(v) = &cache.head {
            assert_eq!(v.borrow().val, 1);
        }
        // assert_eq!((*cache.end.unwrap().borrow_mut()), (*Entry::new("hello", 1).borrow_mut()));
        cache.push_front(Entry::new("hello2", 2));
        cache.push_front(Entry::new("hello3", 3));
        cache.push_front(Entry::new("hello4", 4));

        let iterator = LRUCacheIterator::new(cache.head.clone());
        assert_eq!(
            iterator.collect::<Vec<(&str, i32)>>(),
            vec![("hello4", 4), ("hello3", 3), ("hello2", 2), ("hello", 1)]
        );

        match cache.hashmap.get("hello2") {
            Some(v) => {
                cache.remove_entry(v.clone());
                let iterator = LRUCacheIterator::new(cache.head.clone());
                assert_eq!(
                    iterator.collect::<Vec<(&str, i32)>>(),
                    vec![("hello4", 4), ("hello3", 3), ("hello", 1)]
                )
            }
            _ => {
                assert!(false);
            }
        }

        match cache.get("hello3") {
            Some(v) => {
                assert_eq!(v.borrow().val, 3);
                let iterator = LRUCacheIterator::new(cache.head.clone());
                // 位置发生变化
                assert_eq!(
                    iterator.collect::<Vec<(&str, i32)>>(),
                    vec![("hello4", 3), ("hello3", 4), ("hello", 1)]
                )
            }
            _ => {
                assert!(false);
            }
        }
    }

    #[test]
    fn lru_size_test() {
        let mut cache = LRUCache::new(4);
        for i in 0..=100 {
            cache.put(format!("hello {}", i), i);
        }

        let iterator = LRUCacheIterator::new(cache.head.clone());
        assert_eq!(
            iterator.collect::<Vec<(String, i32)>>(),
            vec![
                ("hello 100".to_owned(), 100),
                ("hello 99".to_owned(), 99),
                ("hello 98".to_owned(), 98),
                ("hello 97".to_owned(), 97)
            ]
        );
    }
}
