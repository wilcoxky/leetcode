# Problem 875 [Koko](https://leetcode.com/problems/koko-eating-bananas/solution/)
___
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