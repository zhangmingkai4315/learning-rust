use std::collections::HashMap;
use std::rc::Rc;
use std::cell::{RefCell, Ref};
use std::borrow::Borrow;

type OptionEntry = Option<Rc<RefCell<Entry<K,V>>>>;

#[derive(Debug, Clone)]
struct Entry<K, V>{
    val: V,
    key: K,
    left: OptionEntry,
    right: OptionEntry,
}


impl<K,V> Entry<K,V>{
    fn new(k: K, v: V) -> Rc<RefCell<Entry<K,V>>>{
        Rc::new(RefCell::new(Entry{
            val:v,
            key: k,
            left: None,
            right: None,
        }))
    }
}


struct LRUCache<K,V>{
    start: OptionEntry,
    end: OptionEntry,
    hashmap: HashMap<K, Rc<RefCell<Entry<K,V>>>>,
    size: usize,
}

impl<K,V> LRUCache<K,V>{
    fn new(size: usize)->LRUCache<K,V>{
        LRUCache{
            size,
            start: None,
            end: None,
            hashmap: HashMap::new(),
        }
    }

    fn get<K,V>(&mut self, k:K)-> OptionEntry
    where V: Clone
    {
        if let Some(entry) = self.hashmap.get(k){
            self.remove_entry(entry);
            self.push_front(v);
            return Some(entry.clone())
        }
        return None
    }


    fn put<K,V>(&mut self, k: K, v: V) where K: Clone{
        match self.hashmap.get(k){
            Some(entry) => {
                let mut new_entry = (*entry).clone();
                new_entry.borrow_mut().val = v;
                self.remove_entry(entry);
                self.push_front(new_entry);
            }
            _ => {
                let entry = Entry::new(k.clone(), v);
                if self.hashmap.len() > self.size {
                    self.hashmap.remove(self.end.key);
                    self.remove_entry(self.end.unwrap().borrow());
                }
                self.push_front(entry);
                self.hashmap.insert(k.clone(), entry.clone());
            }
        }
    }

    fn push_front<K,V>(&mut self, node: Rc<RefCell<Entry<K,V>>>){
        node.borrow_mut().left = None;
        match self.start.take(){
            Some(mut old_start) =>{
                old_start.borrow_mut().left = Some(node.clone());
                self.start = Some(node.clone());
            }
            _ => {
                self.start = Some(node.clone());
            }
        }
    }

    fn remove_entry<K,V>(&mut self, node: &Entry<K,V>){
        if node.left.is_some(){
            node.left.unwrap().borrow_mut().right = node.right.clone();
        }else{
            self.start = node.right.clone();
        }
        if node.right.is_some(){
            node.right.unwrap().borrow_mut().right = node.left.clone();
        }else{
            self.end = node.left.clone();
        }
    }
}


#[cfg(test)]
mod test{
    #[test]
    fn lru_test(){
        unimplemented!()
    }
}