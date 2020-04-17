// Added for testing proposes
// mod substring;
// mod problem1;
// mod problem2;
// mod problem4;
// mod p780_reachingpts;
// mod toptal;
// mod p938_bst_sum;
// mod p106_bnt_inorder;
// mod p875_koko;
// mod p1389_target_order;
mod p583_delete_operation;

fn main() {
    println!("Hello LeetCode");
    let word1 = "sea".to_owned();
    let word2 = "eat".to_owned();
    use p583_delete_operation::min_distance;
    min_distance(word1, word2);
}
