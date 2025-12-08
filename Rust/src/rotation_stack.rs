use num::BigInt;
use num::ToPrimitive;
use num::traits::Euclid;

use rand::Rng;       // trait for random number generation
use rand::thread_rng;
use rand::rngs::ThreadRng;

/// Treap node with implicit index keys
#[derive(Clone, Debug)]
struct Node<T> {
    val: T,
    prio: u32,
    size: usize,
    left: Link<T>,
    right: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

fn size<T>(t: &Link<T>) -> usize {
    t.as_ref().map(|n| n.size).unwrap_or(0)
}

fn update<T>(t: &mut Box<Node<T>>) {
    t.size = 1 + size(&t.left) + size(&t.right);
}

/// Split treap at position `k`
/// left subtree contains first k elements
fn split<T>(t: Link<T>, k: usize) -> (Link<T>, Link<T>) {
    if let Some(mut node) = t {
        let left_size = size(&node.left);
        if k <= left_size {
            let (l, r) = split(node.left.take(), k);
            node.left = r;
            update(&mut node);
            (l, Some(node))
        } else {
            let (l, r) = split(node.right.take(), k - left_size - 1);
            node.right = l;
            update(&mut node);
            (Some(node), r)
        }
    } else {
        (None, None)
    }
}

/// Merge two treaps
fn merge<T>(a: Link<T>, b: Link<T>) -> Link<T> {
    match (a, b) {
        (None, t) | (t, None) => t,
        (Some(mut a), Some(mut b)) => {
            if a.prio > b.prio {
                a.right = merge(a.right.take(), Some(b));
                update(&mut a);
                Some(a)
            } else {
                b.left = merge(Some(a), b.left.take());
                update(&mut b);
                Some(b)
            }
        }
    }
}

fn get_at<'a, T>(t: &'a Link<T>, idx: usize) -> Option<&'a T> {
    let node = t.as_ref()?;
    let left_size = size(&node.left);

    if idx < left_size {
        get_at(&node.left, idx)
    } else if idx == left_size {
        Some(&node.val)
    } else {
        get_at(&node.right, idx - left_size - 1)
    }
}

fn get_at_mut<'a, T>(t: &'a mut Link<T>, idx: usize) -> Option<&'a mut T> {
    let node = t.as_mut()?;
    let left_size = size(&node.left);

    if idx < left_size {
        get_at_mut(&mut node.left, idx)
    } else if idx == left_size {
        Some(&mut node.val)
    } else {
        get_at_mut(&mut node.right, idx - left_size - 1)
    }
}

/// Stack wrapper around implicit treap
#[derive(Clone, Debug)]
pub struct RotStack<T> {
    root: Link<T>,
    rng: ThreadRng,
}

impl<T: Ord> Ord for RotStack<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.to_vec().cmp(&other.to_vec())
    }
}

impl<T: Ord> PartialOrd for RotStack<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}


impl<T: Eq> PartialEq for RotStack<T> {
    fn eq(&self, other: &Self) -> bool {
        self.to_vec() == other.to_vec()
    }
}

impl<T: Eq> Eq for RotStack<T> {}

impl<T> RotStack<T> {
    pub fn new() -> Self {
        Self { root: None, rng: thread_rng() }
    }

    pub fn len(&self) -> usize {
        size(&self.root)
    }

    pub fn push(&mut self, val: T) {
        let node = Box::new(Node {
            val,
            prio: self.rng.gen(),
            size: 1,
            left: None,
            right: None,
        });

        // Insert at end
        self.root = merge(self.root.take(), Some(node));
    }

    pub fn pop(&mut self) -> Option<T> {
        let n = self.len();
        if n == 0 { return None; }
        let (a, b) = split(self.root.take(), n - 1);
        if let Some(mut last) = b {
            self.root = a;
            return Some(last.val);
        }
        None
    }

    /// rotate top k elements right by 1
    pub fn rotate_right(&mut self, k: usize) {
        let n = self.len();
        if k <= 1 || k > n { return; }

        // Split: [bottom, top_k]
        let (a, b) = split(self.root.take(), n - k);

        // b = [head, tail_element]
        let (b1, b2) = split(b, k - 1);

        // new top-k = [tail_element, head]
        let new_b = merge(b2, b1);

        // final tree = [bottom, new_top_k]
        self.root = merge(a, new_b);
    }

    /// rotate top k elements left by 1
    pub fn rotate_left(&mut self, k: usize) {
        let n = self.len();
        if k <= 1 || k > n { return; }

        // Split: [bottom, top_k]
        let (a, b) = split(self.root.take(), n - k);

        // b = [first_element, rest]
        let (b1, b2) = split(b, 1);

        // new top-k = [rest, first_element]
        let new_b = merge(b2, b1);

        // final tree = [bottom, new_top_k]
        self.root = merge(a, new_b);
    }

    pub fn rotate_right_amount(&mut self, k: usize, r: usize) {
        let n = self.len();
        if k <= 1 || k > n { return; }

        let r = r % k;
        if r == 0 { return; }

        // A = bottom, B = top k
        let (a, b) = split(self.root.take(), n - k);

        // B1 = first (k-r) elements
        // B2 = last r elements
        let (b1, b2) = split(b, k - r);

        // rotate: B2 + B1
        let new_b = merge(b2, b1);

        self.root = merge(a, new_b);
    }

    pub fn rotate_left_amount(&mut self, k: usize, r: usize) {
        let r = r % k;
        // left rotate by r = right rotate by (k - r)
        self.rotate_right_amount(k, k - r);
    }

    /// convert to Vec from bottom to top
    pub fn to_vec(&self) -> Vec<&T> {
        fn dfs<'a, T>(t: &'a Link<T>, out: &mut Vec<&'a T>) {
            if let Some(n) = t.as_ref() {
                dfs(&n.left, out);
                out.push(&n.val);
                dfs(&n.right, out);
            }
        }
        let mut v = Vec::new();
        dfs(&self.root, &mut v);
        v
    }

    pub fn get(&self, idx: usize) -> Option<&T> {
        if idx >= self.len() { return None; }
        get_at(&self.root, idx)
    }

    pub fn get_mut(&mut self, idx: usize) -> Option<&mut T> {
        if idx >= self.len() { return None; }
        get_at_mut(&mut self.root, idx)
    }
}
