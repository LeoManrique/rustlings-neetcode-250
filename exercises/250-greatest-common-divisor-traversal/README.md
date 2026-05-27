# Greatest Common Divisor Traversal

**LeetCode #2827** | **Difficulty:** Hard

**Tags:** `Array` `Math` `Union-Find` `Number Theory`

---

## Problem Description

You are given a **0-indexed** integer array `nums`, and you are allowed to **traverse** between its indices. You can traverse between index `i` and index `j`, `i != j`, if and only if `gcd(nums[i], nums[j]) > 1`, where `gcd` is the **greatest common divisor**.

Your task is to determine if for **every pair** of indices `i` and `j` in nums, where `i 2 -> 1, where we move from index 0 to index 2 because gcd(nums[0], nums[2]) = gcd(2, 6) = 2 > 1, and then move from index 2 to index 1 because gcd(nums[2], nums[1]) = gcd(6, 3) = 3 > 1.
To go from index 0 to index 2, we can just go directly because gcd(nums[0], nums[2]) = gcd(2, 6) = 2 > 1. Likewise, to go from index 1 to index 2, we can just go directly because gcd(nums[1], nums[2]) = gcd(3, 6) = 3 > 1.

```

**Example 2:**

```

**Input:** nums = [3,9,5]
**Output:** false
**Explanation:** No sequence of traversals can take us from index 0 to index 2 in this example. So, we return false.

```

**Example 3:**

```

**Input:** nums = [4,3,12,8]
**Output:** true
**Explanation:** There are 6 possible pairs of indices to traverse between: (0, 1), (0, 2), (0, 3), (1, 2), (1, 3), and (2, 3). A valid sequence of traversals exists for each pair, so we return true.

```

**Constraints:**

- `1 <= nums.length <= 10^5`

- `1 <= nums[i] <= 10^5`

---

## Hints

<details>
<summary>Hint 1</summary>

Create a (prime) factor-numbers list for all the indices.

</details>

<details>
<summary>Hint 2</summary>

Add an edge between the neighbors of the (prime) factor-numbers list. The order of the numbers doesn’t matter. We only need edges between 2 neighbors instead of edges for all pairs.

</details>

<details>
<summary>Hint 3</summary>

The problem is now similar to checking if all the numbers (nodes of the graph) are in the same connected component.

</details>

<details>
<summary>Hint 4</summary>

Any algorithm (i.e., BFS, DFS, or Union-Find Set) should work to find or check connected components

</details>

