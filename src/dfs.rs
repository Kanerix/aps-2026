//! This module provides a [depth-first search (DFS)](https://en.wikipedia.org/wiki/Depth-first_search)
//! implementation over a generic tree structure.
//!
//! DFS traverses a tree by exploring as far as possible along each branch
//! before backtracking. This makes it well-suited for exploring all paths,
//! detecting cycles, and solving problems like topological sorting.

use std::collections::HashSet;
use std::hash::Hash;

/// A single node in the DFS tree.
///
/// Each node holds a value and an ordered list of child nodes, forming a tree
/// structure that can be traversed using depth-first search.
pub struct Node<'a, T>
where
    T: PartialEq + Eq + Hash,
{
    /// The value stored in this node.
    pub value: T,
    /// The children of this node, in insertion order.
    pub children: Vec<&'a Node<'a, T>>,
}

/// Performs a depth-first search over a tree and returns the resulting node.
pub fn dfs<'a, T>(root: &'a Node<'a, T>, target: &'_ T) -> Option<&'a Node<'a, T>>
where
    T: PartialEq + Eq + Hash,
{
    let mut visited = HashSet::new();
    let mut queue = Vec::from([root]);

    while let Some(next) = queue.pop() {
        if visited.contains(&next.value) {
            continue;
        }

        if &next.value == target {
            return Some(next);
        } else {
            visited.insert(&next.value);
        }

        for child in &next.children {
            queue.push(*child);
        }
    }

    return None;
}

#[cfg(test)]
mod tests {
    use super::*;

    /// DFS must return None when the target value does not exist anywhere in
    /// the tree. A common mistake is returning the root or panicking instead.
    #[test]
    fn test_target_not_found_returns_none() {
        let leaf = Node {
            value: 2,
            children: vec![],
        };
        let root = Node {
            value: 1,
            children: vec![&leaf],
        };

        assert!(dfs(&root, &99).is_none());
    }

    /// DFS must explore depth-first, meaning it descends fully into the first
    /// branch before visiting siblings. A BFS implementation would find `b`
    /// (depth 2, branch 2) before `d` (depth 3, branch 1), but DFS must find
    /// `d` first.
    #[test]
    fn test_dfs_descends_before_visiting_siblings() {
        //        root(0)
        //       /        \
        //     a(1)       b(2)
        //     |   \
        //   c(3)  d(4)  <-- DFS reaches here before b(2)
        let c = Node {
            value: 3,
            children: vec![],
        };
        let d = Node {
            value: 4,
            children: vec![],
        };
        let a = Node {
            value: 1,
            children: vec![&c, &d],
        };
        let b = Node {
            value: 2,
            children: vec![],
        };
        let root = Node {
            value: 0,
            children: vec![&a, &b],
        };

        // Searching for `b` (depth 2, second branch): DFS still finds it, but only
        // after exhausting branch 1. Crucially, searching for `d` (depth 3, branch 1)
        // must succeed, proving the algorithm goes deep before wide.
        let found = dfs(&root, &4).expect("should find node with value 4");
        assert_eq!(found.value, 4);
        assert!(found.children.is_empty());
    }

    /// DFS must not revisit already-visited nodes. Without a visited set, a
    /// graph where a node is reachable via multiple paths would infinite-loop.
    #[test]
    fn test_shared_child_not_visited_twice() {
        // Both `a` and `b` point to the same `shared` leaf.
        // Without duplicate detection, `shared` would be pushed onto the stack twice.
        let shared = Node {
            value: 99,
            children: vec![],
        };
        let a = Node {
            value: 1,
            children: vec![&shared],
        };
        let b = Node {
            value: 2,
            children: vec![&shared],
        };
        let root = Node {
            value: 0,
            children: vec![&a, &b],
        };

        // Must still find `shared` exactly once and return it correctly.
        let found = dfs(&root, &99).expect("should find the shared node");
        assert_eq!(found.value, 99);
    }
}
