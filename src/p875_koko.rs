use std::collections::BinaryHeap;

pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
    let ranked_el = h - piles.len() as i32;
    let required_rank = if ranked_el > piles.len() as i32 {
        piles.len() as i32
    } else {
        ranked_el
    };

    let mu

    let mut max_heap = BinaryHeap::from(piles);

    let mut min_speed = i32::max_value();

    // let (quotient, remainder) = div_mod_floor();

    min_speed
}

fn div_mod_floor(n: i32, rhs: i32) -> (i32, i32) {
    let quotient = n / rhs;
    let remainder = n - (quotient * rhs);
    (quotient, remainder)
}

#[test]
fn test_div_mod() {
    assert_eq!((17, 3), div_mod_floor(88, 5));
}

#[test]
fn simple_test() {
    let piles = vec![3, 6, 7, 11];
    let h = 8;
    assert_eq!(min_eating_speed(piles, h), 4);
}

#[test]
fn piles_length_hour_check() {
    let piles = vec![30, 11, 23, 4, 20];
    let h = 5;
    assert_eq!(min_eating_speed(piles, h), 30);
}

#[test]
fn piles_more_complicated() {
    let piles = vec![30, 11, 23, 4, 20];
    let h = 6;
    assert_eq!(min_eating_speed(piles, h), 23);
}
