// Given a non-empty array of integers nums, every element appears twice except for one. Find that single one.

// Follow up: Could you implement a solution with a linear runtime complexity and without using extra memory?

// Example 1:

// Input: nums = [2,2,1]
// Output: 1

// Example 2:

// Input: nums = [4,1,2,1,2]
// Output: 4

// Example 3:

// Input: nums = [1]
// Output: 1

const singleNumber = function (nums) {
  const counts = new Map();
  for (let n of nums) {
    const cur = counts.get(n);
    if (cur !== undefined) {
      counts.set(n, cur + 1);
    } else {
      counts.set(n, 1);
    }
  }

  for (let key in counts) {
    const num = counts.get(key);
    if (num === 1) {
      return num;
    }
  }
};
