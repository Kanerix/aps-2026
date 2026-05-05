//! This module provides a [breadth-first search (BFS)](https://en.wikipedia.org/wiki/Breadth-first_search)
//! implementation over a generic tree structure.
//!
//! BFS traverses a tree level by level, visiting all nodes at the current depth
//! before moving on to nodes at the next depth. This makes it well-suited for
//! finding the shortest path in an unweighted graph or tree.

use std::collections::{HashSet, VecDeque};
use std::hash::Hash;

/// A single node in the BFS tree.
///
/// Each node holds a value and an ordered list of child nodes, forming a tree
/// structure that can be traversed using breadth-first search.
pub struct Node<'a, T>
where
    T: PartialEq + Eq + Hash,
{
    /// The value stored in this node.
    pub value: T,
    /// The children of this node, in insertion order.
    pub children: Vec<&'a Node<'a, T>>,
}

/// Performs a breadth-first search over a tree and returns the resulting node.
pub fn bfs<'a, T>(root: &'a Node<'a, T>, target: &'_ T) -> Option<&'a Node<'a, T>>
where
    T: PartialEq + Eq + Hash,
{
    let mut visited = HashSet::new();
    let mut queue = VecDeque::from([root]);

    while let Some(next) = queue.pop_front() {
        if visited.contains(&next.value) {
            continue;
        }

        if &next.value == target {
            return Some(next);
        } else {
            visited.insert(&next.value);
        }

        for child in &next.children {
            queue.push_back(*child);
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    /// BFS must return None when the target value does not exist anywhere in
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

        assert!(bfs(&root, &99).is_none());
    }

    /// BFS must visit nodes level-by-level, so it should find a node at depth 2
    /// on a later branch *before* a node at depth 3 on the first branch. A DFS
    /// implementation would wrongly descend into branch 1's subtree first.
    #[test]
    fn test_bfs_finds_shallower_node_before_deeper_one() {
        //        root(0)
        //       /        \
        //     a(1)       b(2)  <-- target is here (depth 2, branch 2)
        //     |   \
        //   c(3)  d(4)         <-- depth 3, only reachable via branch 1
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

        // BFS must find `b` (depth 2) and confirm it has no children,
        // proving it didn't accidentally return `a` or one of its children.
        let found = bfs(&root, &2).expect("should find node with value 2");
        assert_eq!(found.value, 2);
        assert!(
            found.children.is_empty(),
            "found the wrong node — expected leaf b"
        );
    }

    /// BFS must not revisit already-visited nodes. Without a visited set, a
    /// graph where a node is reachable via multiple paths would loop or produce
    /// duplicate results.
    #[test]
    fn test_shared_child_not_visited_twice() {
        // Both `a` and `b` point to the same `shared` leaf.
        // Without cycle/duplicate detection, `shared` would be enqueued twice.
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
        let found = bfs(&root, &99).expect("should find the shared node");
        assert_eq!(found.value, 99);
    }
}
