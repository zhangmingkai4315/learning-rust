pub mod binary_search_tree{
    use std::rc::Rc;
    use std::cell::RefCell;


    pub type NodeOption = Option<Rc<RefCell<Node>>>;

    #[derive(PartialEq, Debug)]
    pub struct Node{
        pub val: u64,
        left: NodeOption,
        right: NodeOption,
    }

    impl Node{
        pub fn new(val: u64)->Node{
            Node{
                val,
                left: None,
                right: None,
            }
        }
    }

    pub struct BinarySearchTree{
        pub root: NodeOption,
        pub count: u64
    }

    impl BinarySearchTree {
        pub fn new() -> BinarySearchTree {
            BinarySearchTree {
                root: None,
                count: 0
            }
        }

        pub fn is_empty(&self) -> bool {
            self.count == 0
        }

        fn new_node_option(&self, key: u64) -> NodeOption {
            Some(Rc::new(RefCell::new(Node::new(key))))
        }

        pub fn insert(&mut self, val: u64) {
            if self.is_empty() {
                self.root = self.new_node_option(val);
                self.count = 1;
                return;
            }

            let insert_right;
            let mut curr = Rc::clone(self.root.as_ref().unwrap());

            loop {
                let next_curr;

                if val < curr.borrow().val {
                    match &curr.borrow().left {
                        Some(left_node) => next_curr = Rc::clone(left_node),
                        _ => {
                            insert_right = false;
                            break;
                        }
                    }
                } else {
                    match &curr.borrow().right {
                        Some(right_node) => next_curr = Rc::clone(right_node),
                        _ => {
                            insert_right = true;
                            break;
                        }
                    }
                }
                curr = next_curr;
            }

            if insert_right {
                curr.borrow_mut().right = self.new_node_option(val);
            } else {
                curr.borrow_mut().left = self.new_node_option(val);
            }

            self.count += 1;
        }


        // pub fn delete(&mut self, val:u64)->bool{
        //     if self.is_empty(){
        //         return false;
        //     }
        //     if self.count == 1 {
        //         if self.root.unwrap().borrow().val == val{
        //             self.root == None;
        //             self.count = 0;
        //             true;
        //         }
        //         false;
        //     }
        //
        //     let mut curr = Rc::clone(self.root.as_ref().unwrap());
        //     let mut parent : NodeOption = None ;
        //     let is_left;
        //     loop {
        //         let next_curr;
        //         if val == curr.borrow().val{
        //             // 找到节点后进行处理， 如果左右子树均不存在，直接父节点删除
        //             if curr.borrow().right == None && curr.borrow().left == None{
        //                 if parent != None {
        //                     if is_left == true{
        //                         parent.unwrap().borrow_mut().left == None
        //                     }else{
        //                         parent.unwrap().borrow_mut().right == None
        //                     }
        //                 }else{
        //                     self.root = None
        //                 }
        //             }else if curr.borrow().right != None || curr.borrow().left != None{
        //                 // 左右子树均存在 inorder 右子树的最小值作为当前值
        //                 let right = curr.borrow_mut().right;
        //
        //             } else{
        //               // 左子树可能存在， 右子树可能存在 ， 获取存在的节点
        //                 let exist_node = {
        //                     if curr.borrow().right == None{
        //                         Rc::clone(curr.as_ref().borrow().left.as_ref().unwrap())
        //                     }else{
        //                         Rc::clone(curr.as_ref().borrow().right.as_ref().unwrap())
        //                     }
        //                 };
        //                 if parent != None {
        //                     if is_left == true{
        //                         parent.unwrap().borrow_mut().left == exist_node
        //                     }else{
        //                         parent.unwrap().borrow_mut().right == exist_node
        //                     }
        //                 }else{
        //                     // 无parent只能是根节点
        //                     self.root = Some(exist_node);
        //                 }
        //             }
        //             self.count-=1;
        //         }else if val < curr.borrow().val{
        //             match &curr.borrow().left {
        //                 Some(left_node) => next_curr = Rc::clone(left_node),
        //                 _ => break,
        //             }
        //         }else {
        //             match &curr.borrow().right {
        //                 Some(right_node) => next_curr = Rc::clone(right_node),
        //                 _ => break,
        //             }
        //         }
        //         curr = next_curr;
        //     }
        //     return false
        // }

        pub fn find(&self, val: u64) -> bool {
            if self.is_empty() {
                return false
            }

            let mut curr = Rc::clone(self.root.as_ref().unwrap());
            loop {
                let next_curr;
                if val == curr.borrow().val {
                    return true
                } else if val < curr.borrow().val {
                    match &curr.borrow().left {
                        Some(left_node) => next_curr = Rc::clone(left_node),
                        _ => break,
                    }
                } else {
                    match &curr.borrow().right {
                        Some(right_node) => next_curr = Rc::clone(right_node),
                        _ => break,
                    }
                }
                curr = next_curr;
            }
            false
        }

        pub fn sort(&self, arr: &mut Vec<u64>) {
            self.inorder(&self.root, arr)
        }

        fn inorder(&self, root: &NodeOption, arr: &mut Vec<u64>) {
            if let Some(node) = root {
                self.inorder(&node.borrow().left, arr);
                arr.push(node.borrow().val);
                self.inorder(&node.borrow().right, arr);
            }
        }

        pub fn minimum_key(subtree: &NodeOption) -> NodeOption {
            if subtree.is_none() {
                return None;
            }
            let mut current  = subtree.as_ref().unwrap().clone();
            loop{
                if current.borrow().left.is_none() {
                    break
                }
                current = current.clone().borrow().left.as_ref().unwrap().clone();
            }
            return Some(current)
        }
    }


}

#[cfg(test)]
mod tests{
    use crate::binary_search_tree::binary_search_tree::{BinarySearchTree, Node};

    #[test]
    fn test_new_node(){
        let node = Node::new(5);
        assert_eq!(node.val, 5);
    }


    #[test]
    fn test_bst(){
        let mut bs_tree = BinarySearchTree::new();
        assert_eq!(bs_tree.count, 0);
        assert_eq!(bs_tree.is_empty(), true);

        bs_tree.insert(10);
        bs_tree.insert(20);
        bs_tree.insert(0);
        assert_eq!(bs_tree.count, 3);

        bs_tree.insert(8);
        bs_tree.insert(3);
        bs_tree.insert(25);
        bs_tree.insert(7);

        assert_eq!(bs_tree.find(8), true);
        assert_eq!(bs_tree.find(25), true);
        assert_eq!(bs_tree.find(24), false);
        assert_eq!(bs_tree.find(99), false);
        assert_eq!(bs_tree.find(3), true);

        assert_eq!(bs_tree.count, 7);

        let mut arr = vec![];
        bs_tree.sort(&mut arr);
        assert_eq!(arr, vec![0, 3, 7, 8, 10, 20, 25]);
    }

    #[test]
    fn test_min(){
        let mut bs_tree = BinarySearchTree::new();
        assert_eq!(bs_tree.count, 0);
        assert_eq!(bs_tree.is_empty(), true);

        bs_tree.insert(10);
        bs_tree.insert(20);
        bs_tree.insert(0);
        assert_eq!(bs_tree.count, 3);

        bs_tree.insert(8);
        bs_tree.insert(3);
        bs_tree.insert(25);
        bs_tree.insert(7);

        let item = BinarySearchTree::minimum_key(&bs_tree.root);
        assert_eq!(item.is_some(), true);
        assert_eq!(item.unwrap().borrow().val, 0)
    }

}