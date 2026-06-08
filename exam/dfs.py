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


def dfs(graph, start):
    visited = set()
    stack = [start]

    # O(V + E)
    while stack:
        node = stack.pop()
        if node not in visited:
            visited.add(node)
            stack.extend(graph[node])


dfs(graph, "A")
