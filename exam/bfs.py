from collections import deque

# Adjacency list representation
adjacency_list = {
    "A": ["B", "C"],
    "B": ["A", "D", "E"],
    "C": ["A", "F"],
    "D": ["B"],
    "E": ["B", "F"],
    "F": ["C", "E"],
}

# Edge list representation
edge_list = [
    ("A", "B"),
    ("A", "C"),
    ("B", "D"),
    ("B", "E"),
    ("C", "F"),
    ("E", "F"),
]


graph = adjacency_list


def bfs(graph, start):
    visited = set()
    queue = deque([start])

    # O(V + E)
    while queue:
        node = queue.popleft()
        if node not in visited:
            visited.add(node)
            queue.extend(graph[node])


bfs(graph, "A")
