# Kth Smallest Element in a BST

**LeetCode #230** | **Difficulty:** Medium

**Tags:** `Tree` `Depth-First Search` `Binary Search Tree` `Binary Tree`

---

## Problem Description

Given the `root` of a binary search tree, and an integer `k`, return *the* `k^th` *smallest value (**1-indexed**) of all the values of the nodes in the tree*.

**Example 1:**

```

**Input:** root = [3,1,4,null,2], k = 1
**Output:** 1

```

**Example 2:**

```

**Input:** root = [5,3,6,2,4,null,null,1], k = 3
**Output:** 3

```

**Constraints:**

- The number of nodes in the tree is `n`.

- `1 <= k <= n <= 10^4`

- `0 <= Node.val <= 10^4`

**Follow up:** If the BST is modified often (i.e., we can do insert and delete operations) and you need to find the kth smallest frequently, how would you optimize?

---

## Hints

<details>
<summary>Hint 1</summary>

Try to utilize the property of a BST.

</details>

<details>
<summary>Hint 2</summary>

Try in-order traversal. (Credits to @chan13)

</details>

<details>
<summary>Hint 3</summary>

What if you could modify the BST node's structure?

</details>

<details>
<summary>Hint 4</summary>

The optimal runtime complexity is O(height of BST).

</details>

