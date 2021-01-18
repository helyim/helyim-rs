use std::ptr::NonNull;
use std::marker::PhantomData;
use std::mem;
use rand::Rng;

const SKIP_LIST_MAX_LEVEL: usize = 32;
const SKIP_LIST_P: f32 = 0.25;

#[derive(Debug)]
struct SkipListLevel<T>
    where
        T: PartialOrd
{
    forward: Option<NonNull<SkipListNode<T>>>,
    span: usize,
}

#[derive(Debug)]
struct SkipListNode<T>
    where
        T: PartialOrd
{
    element: Option<T>,
    backward: Option<NonNull<SkipListNode<T>>>,
    level: Vec<SkipListLevel<T>>,
}

#[derive(Debug)]
pub struct SkipList<T>
    where
        T: PartialOrd
{
    header: Option<NonNull<SkipListNode<T>>>,
    tail: Option<NonNull<SkipListNode<T>>>,
    length: usize,
    level: usize,
    marker: PhantomData<Box<SkipListNode<T>>>,
}

impl<T: PartialOrd> Default for SkipList<T> {
    fn default() -> Self {
        let header = SkipListNode::new(SKIP_LIST_MAX_LEVEL, None);
        SkipList {
            header: Some(Box::leak(header).into()),
            tail: None,
            length: 0,
            level: 1,
            marker: PhantomData,
        }
    }
}

// public function
impl<T: PartialOrd> SkipList<T> {
    pub fn new() -> Self {
        SkipList::default()
    }

    pub fn push(&mut self, element: T) {
        let mut update = Vec::<Box<SkipListNode<T>>>::with_capacity(SKIP_LIST_MAX_LEVEL);
        let mut rank = [0usize; SKIP_LIST_MAX_LEVEL];
        let mut header = self.header.unwrap();
        for i in (0..=self.level - 1).rev().step_by(1) {
            if self.level - 1 == i {
                rank[i] = 0;
            } else {
                rank[i] = rank[i + 1];
            }
            unsafe {
                let mut header = Box::from_raw(header.as_ptr());
                if let Some(level) = header.level.get(i) {
                    while level.forward.is_some() {
                        let forward = Box::from_raw(level.forward.unwrap().as_ptr());
                        if forward.element.unwrap() < element {
                            rank[i] += level.span;
                            header = forward;
                        }
                    }
                    update[i] = header;
                }
            }
        }
        let level = random_level();
        if level > self.level {
            for i in self.level..level {
                rank[i] = 0;
                update[i] = unsafe { Box::from_raw(self.header.unwrap().as_ptr()) };
                update[i].level[i].span = self.length;
            }
            self.level = level;
        }
        let header = SkipListNode::new(level, Some(element));
        for i in 0..level {
            header.level[i].forward = update[i].level[i].forward;
        }
    }
}


// private function
impl<T: PartialOrd> SkipList<T> {
    fn push_node(&mut self, node: Box<SkipListNode<T>>) {}
}

impl<T: PartialOrd> SkipListNode<T> {
    fn new(level: usize, element: Option<T>) -> Box<Self> {
        let mut skip_list_level = vec![];
        for _ in 0..level {
            let level = SkipListLevel {
                forward: None,
                span: 0,
            };
            skip_list_level.push(level);
        }
        Box::new(SkipListNode {
            element,
            backward: None,
            level: skip_list_level,
        })
    }
}

fn random_level() -> usize {
    let mut level = 1;
    let mut rng = rand::thread_rng();
    while ((rng.gen::<usize>() & 0xFFFF) as f32) < (SKIP_LIST_P * 0xFFFF as f32) {
        level += 1;
    }
    if level < SKIP_LIST_MAX_LEVEL {
        level
    } else {
        SKIP_LIST_MAX_LEVEL
    }
}

#[cfg(test)]
mod test {
    use crate::collections::list::skip_list::random_level;

    #[test]
    fn test_rand_level() {
        for _ in 0..=100 {
            println!("{}", random_level());
        }
    }
}