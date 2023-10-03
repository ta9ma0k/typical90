use proconio::input;

fn main() {
    input! {
        (n, l): (usize, u32),
        k: u32,
        a: [u32; n],
    }
    let yokan = Youkan::new(l, a);
    println!("{}", resolve(&yokan, k));
}

struct BinaryTree {
    left: i32,
    right: i32,
}

impl BinaryTree {
    fn init(l: u32) -> Self {
        Self {
            left: -1,
            right: (l + 1) as i32,
        }
    }

    fn set_left(&mut self, x: u32) {
        self.left = x as i32;
    }
    fn set_right(&mut self, x: u32) {
        self.right = x as i32;
    }
    fn is_valid(&self) -> bool {
        self.right - self.left > 1
    }
    fn total(&self) -> u32 {
        (self.right + self.left) as u32
    }
}

fn resolve(yokan: &Youkan, k: u32) -> u32 {
    let mut binary_tree = BinaryTree::init(yokan.length);
    while binary_tree.is_valid() {
        let mid = binary_tree.total() / 2;
        if yokan.count_slice_piece(mid) >= k + 1 {
            binary_tree.set_left(mid);
        } else {
            binary_tree.set_right(mid);
        }
    }
    binary_tree.left as u32
}

#[test]
fn test_resolve() {
    let yokan = Youkan::new(34, vec![8, 13, 26]);
    assert_eq!(resolve(&yokan, 1), 13);
}

struct Youkan {
    length: u32,
    break_points: Vec<u32>,
}

impl Youkan {
    fn new(length: u32, break_points: Vec<u32>) -> Self {
        Self {
            length,
            break_points,
        }
    }

    fn count_slice_piece(&self, size: u32) -> u32 {
        let mut count = 0;
        let mut prev = 0;
        for &p in &self.break_points {
            if p - prev >= size {
                count += 1;
                prev = p;
            }
        }
        if self.length - prev >= size {
            count += 1;
        }
        count
    }
}

#[test]
fn test_count_slice_piece() {
    let sut = super::Youkan::new(10, vec![2, 4, 7]);
    assert_eq!(sut.count_slice_piece(3), 3);
}
