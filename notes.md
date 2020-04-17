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
