fn main() {
    proconio::input! {
        n: usize,
        q: usize,
        elevations: [i64; n],
        queries: [(usize, usize, i64); q],
    }
    let mut floor_difference = FloorDifference::from_elevations(elevations);
    for (start, end, value) in queries {
        floor_difference.crustal_deformation(start, end, value);
        println!("{}", floor_difference.inconvenience);
    }
}

struct FloorDifference {
    n: usize,
    inconvenience: i64,
    floor_difference: Vec<i64>,
}

impl FloorDifference {
    fn new(n: usize, inconvenience: i64, floor_difference: Vec<i64>) -> Self {
        Self {
            n,
            inconvenience,
            floor_difference,
        }
    }

    fn from_elevations(elevations: Vec<i64>) -> Self {
        let floor_difference = (1..elevations.len()).map(|x| elevations[x] - elevations[x - 1]);
        Self::new(
            elevations.len(),
            floor_difference.clone().map(|x| x.abs()).sum(),
            floor_difference.collect(),
        )
    }

    fn crustal_deformation(&mut self, start: usize, end: usize, value: i64) {
        if start - 1 > 0 {
            let before = self.floor_difference[start - 2].abs();
            self.floor_difference[start - 2] += value;
            self.inconvenience += self.floor_difference[start - 2].abs() - before;
        }
        if end < self.n {
            let before = self.floor_difference[end - 1].abs();
            self.floor_difference[end - 1] -= value;
            self.inconvenience += self.floor_difference[end - 1].abs() - before;
        }
    }
}

#[test]
fn test_case1() {
    let mut sut = FloorDifference::from_elevations(vec![1, 2, 3]);
    assert_eq!(sut.floor_difference, vec![1, 1]);
    sut.crustal_deformation(2, 3, 1);
    assert_eq!(sut.floor_difference, vec![2, 1]);
    assert_eq!(sut.inconvenience, 3);
}

#[test]
fn test_get_floor_difference() {
    let sut1 = FloorDifference::from_elevations(vec![1, 2, 3]);
    assert_eq!(sut1.floor_difference, vec![1, 1]);
    assert_eq!(sut1.n, 3);
    assert_eq!(sut1.inconvenience, 2);
    let sut2 = FloorDifference::from_elevations(vec![3, 1, 4]);
    assert_eq!(sut2.floor_difference, vec![-2, 3]);
    assert_eq!(sut2.n, 3);
    assert_eq!(sut2.inconvenience, 5);
}

#[test]
fn test_crustal_deformation() {
    let mut sut1 = FloorDifference::new(5, 12, vec![-2, 3, -3, 4]);
    sut1.crustal_deformation(3, 4, 5);
    assert_eq!(sut1.floor_difference, vec![-2, 8, -3, -1]);
}
