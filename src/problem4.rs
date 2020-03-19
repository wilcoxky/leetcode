/// This answer is wrong but since the time complexity required was O(log(n))
/// Didn't know that when I started, but it passed since Rust is so fast haha
/// to fix you do a recursive solution finding the median of both arrays until
/// they equal each other, until you get to a size of 2 for both of them.
/// After that you do the max(of the first position elements) + min(2nd pos)
/// divided by 2
pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let merged_vec = merge_vecs(nums1, nums2);
    let total = merged_vec.len();
    let halfway_index = total / 2;
    if total % 2 == 0 {
        let index_second = halfway_index - 1;
        ((merged_vec[halfway_index] as f64) + (merged_vec[index_second] as f64)) / 2.0
    } else {
        merged_vec[halfway_index] as f64
    }
}

/// merges in m + n time advancing pointers when needed
fn merge_vecs(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    let mut combined = vec![];
    let mut i = 0;
    let mut j = 0;
    while i < nums1.len() || j < nums2.len() {
        if i >= nums1.len() {
            combined.push(nums2[j]);
            j += 1;
        } else if j >= nums2.len() {
            combined.push(nums1[i]);
            i += 1;
        } else if nums1[i] < nums2[j] {
            combined.push(nums1[i]);
            i += 1;
        } else {
            combined.push(nums2[j]);
            j += 1;
        }
    }
    combined
}

#[test]
fn it_combines_vecs() {
    let n1 = vec![1, 3];
    let n2 = vec![2];
    let n3 = merge_vecs(n1, n2);
    assert_eq!(n3, vec![1, 2, 3]);
}

#[test]
fn it_works_simple() {
    let n1 = vec![1, 3];
    let n2 = vec![2];
    assert_eq!(find_median_sorted_arrays(n1, n2), 2.0);
}

#[test]
fn it_works_simple2() {
    let n1 = vec![1, 2];
    let n2 = vec![3, 4];
    assert_eq!(find_median_sorted_arrays(n1, n2), 2.5);
}
