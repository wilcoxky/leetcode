pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
    let mut copy = piles.clone();
    copy.sort();

    let mut max = *copy.last().unwrap();
    let mut low = 1;

    while low <= max {
        let min = (max + low) / 2;
        if can_eat_all(&piles, h, min) {
            max = min - 1;
        } else {
            low = min + 1
        }
    }
    low
}

fn can_eat_all(piles: &[i32], h: i32, k: i32) -> bool {
    let mut count = 0;
    for p in piles {
        count += *p / k;
        if  *p % k != 0 {
            count += 1
        }
    }
    count <= h
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
