// pub trait Iterator {
//     type Item;

//     fn next(&mut self) -> Option<Self::Item>;

//     // methods with default implementations elided
// }

struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes
        .into_iter()
        .filter(|s: &Shoe| s.size == shoe_size)
        .collect()
}

fn main() {
    // let v1: Vec<i32> = vec![1, 2, 3];

    // // Iterators in Rust are lazy, so nothing happens until we actually use the iterator.
    // let v1_iter = v1.iter();

    // for value in v1_iter {
    //     println!("Got: {}", value);
    // }

    // Maps - takes a closure as an argument and then calls that closure over each element in the sequence.
    let v1: Vec<i32> = vec![1, 2, 3];
    let v2: Vec<_> = v1.iter().map(|x: &i32| x + 1).collect();
    assert_eq!(v2, vec![2, 3, 4]);
}

#[test]
fn iterator_demonstration() {
    let v1 = vec![1, 2, 3];
    let mut v1_iter = v1.iter();

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
}

#[test]
fn iterator_sum() {
    let v1: Vec<i32> = vec![1, 2, 3];
    let v1_iter = v1.iter();

    let total: i32 = v1_iter.sum();

    assert_eq!(total, 6);
}

#[test]
fn map_test() {
    let v1: Vec<i32> = vec![1, 2, 3];
    let v2: Vec<_> = v1.iter().map(|x: &i32| x + 1).collect();
    assert_eq!(v2, vec![2, 3, 4]);
}

#[test]
fn filters_by_size() {
    let shoes: Vec<Shoe> = vec![
        Shoe {
            size: 10,
            style: String::from("sneaker"),
        },
        Shoe {
            size: 13,
            style: String::from("sandal"),
        },
        Shoe {
            size: 10,
            style: String::from("boot"),
        },
    ];

    let actual = shoes_in_my_size(shoes, 10);

    let expected: Vec<Shoe> = vec![
        Shoe {
            size: 10,
            style: String::from("sneaker"),
        },
        Shoe {
            size: 10,
            style: String::from("boot"),
        },
    ];

    assert_eq!(actual, expected);
}

#[test]
fn calling_next_direct() {
    let mut counter: Counter = Counter::new();

    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next(), Some(3));
    assert_eq!(counter.next(), Some(4));
    assert_eq!(counter.next(), Some(5));
    assert_eq!(counter.next(), None);
}

#[test]
fn using_other_iterator_trait_methods() {
    let sum: u32 = Counter::new()
        .zip(Counter::new().skip(1))
        .map(|(a, b)| a * b)
        .filter(|x: &u32| x % 3 == 0)
        .sum();
    assert_eq!(18, sum);
}
