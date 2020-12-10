mod heap {
    pub struct Job {
        pub priority_level: usize,
        pub description: String,
    }

    impl Job {
        pub fn new(level: usize, desc: &str) -> Job {
            Job {
                priority_level: level,
                description: desc.to_string(),
            }
        }
    }

    pub struct Executor {
        pub length: usize,
        heap: Vec<Box<Job>>,
    }

    impl Executor {
        pub fn new() -> Executor {
            Executor {
                heap: vec![],
                length: 0,
            }
        }

        pub fn add(&mut self, job: Job) {
            // 追加到序列尾部，然后执行bubbles up 向上检查是否大于其父元素
            self.heap.push(Box::new(job));
            self.length = self.heap.len();
            if self.length > 1 {
                let mut i = self.length;
                // 如果存在父元素，且父元素大于子元素
                while i / 2 > 0 && self.compare_priority(i, i / 2) {
                    self.heap.swap(i - 1, i / 2 - 1);
                    i /= 2;
                }
            }
        }

        fn compare_priority(&self, pos1: usize, pos2: usize) -> bool {
            // 如果位置1的优先级大于位置2的优先级(注意这里为了计算方便，使用的是按照从1开始的index方式)
            // 否则返回false
            &self.heap[pos1 - 1].priority_level > &self.heap[pos2 - 1].priority_level
        }

        pub fn pop(&mut self) -> Option<Job> {
            if self.length <= 0 {
                return None;
            }
            // 删除顶部元素，并替换为尾部的元素，返回原顶部元素
            let elem = self.heap.swap_remove(0);
            self.length = self.heap.len();
            self.length = self.heap.len();

            let mut i = 1;

            while i * 2 < self.length {
                let children = (i * 2, i * 2 + 1);
                i = if self.compare_priority(children.0, children.1) {
                    if self.compare_priority(children.0, i) {
                        self.heap.swap(i - 1, children.0 - 1);
                        children.0
                    } else {
                        break;
                    }
                } else {
                    if self.compare_priority(children.1, i) {
                        self.heap.swap(i - 1, children.1 - 1);
                        children.1
                    } else {
                        break;
                    }
                }
            }

            Some(*elem)
        }
    }
}

#[cfg(test)]
mod test {

    use crate::heap::heap::{Executor, Job};

    #[test]
    fn test_heap() {
        let mut excutor = Executor::new();
        let j1 = Job::new(1, "buy milk");
        let j2 = Job::new(5, "run");
        let j3 = Job::new(2, "buy breakfast");

        excutor.add(j1);
        excutor.add(j2);
        excutor.add(j3);

        assert_eq!(excutor.pop().unwrap().priority_level, 5);
        assert_eq!(excutor.pop().unwrap().priority_level, 2);
        assert_eq!(excutor.pop().unwrap().priority_level, 1);
    }
}
