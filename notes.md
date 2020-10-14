### Problem 875 [Koko](https://leetcode.com/problems/koko-eating-bananas/solution/)

My thoughts on this problem, were that you had to find the max. And instead of
first thinking of binary search. I thought a heap would be an easy way of
testing whether or not I need to keep it. After looking at the related topics,
the binary search method makes much more sense. With binary search the key take
aways I got with it are the following.

- high value starts at max of piles
- low value will start at 0
- High value will always be able to eat the naners
- if it can't then we need to up the min value and try between that
- we try and value in between low and high (binary search) and adjust accordingly
- when low => high we have the problem solved

This makes sense. Notes for next time is to not get caught up in a data
structure and to look more towards different methods instead.

### Problem 583 [Delete Operation](https://leetcode.com/problems/delete-operation-for-two-strings)

First thought was to slide the word 1 and word 2 pointers together
to get a count. While this works for the most part the next problem
which we are currently trying to solve is how to deal with multiple
entries of the same letter. I think to do this I will need to make a
map, and then at the end maybe count the values left in the map and
return. Basically it is failing because it is too stupid and is just
looking at the next time the character occurs. We need to do a better
job of deciding which index of the character we take.

New thought, is to basically keep a longest sequence seen along with
the index of the last match. If the lengths of the next match are the
same then we take the one with the shorter index. This will give us
the length and from there we can decide what to do with the string.

[Link Explanation](https://www.youtube.com/watch?v=NnD96abizww)
The solution to this program was to basically implement the
algorithm described above. It took advantage of dynamic programming
which allowed us to use our previous calculations to easily compare
the strings. Watch the video for the refresher. Another thing that
was helpful was I learned how to debug Rust code in VSCode, it was
super simple and just followed this [guide](https://jason-williams.co.uk/debugging-rust-in-vscode).


### Problem 350 [Delete Operation](https://leetcode.com/problems/intersection-of-two-arrays-ii/)

Naive solution, is to go through the larger array and when a corrisponding element is found. You remove it from array *num2* and then redo the search. This requires you to pass through the second array twice results in O(max(n,m)^2).

Trying to think of a better solution now...
You could sort the arrays (nlogn) and then search for 

Proposed Solutions: Create a map over the first array holding a count of the occurances, then loop through subtracking the entries in the map and add if the count is greater than 0.

Related questions:
- What if the given array is already sorted? How would you optimize your algorithm
If sorted, you can compare the 
- What if nums1's size is small compared to nums2's size? Which algorithm is better
If num1 is smaller you can use it as the array in the naive solution that you would loop through.

- What if elements of nums2 are stored on disk, and the memory is limited such that you cannot load all elements into the memory at once?
In disk, the looping over the num2 array would not be ideal. Thus one should loop through that array only once.

