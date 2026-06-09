# Adjacency list is chosen over edge list because topological sort needs to
# iterate over the neighbours of each node repeatedly.

# DAG:
#   A ---> C ---> E
#          ^      ^
#          |      |
#   B -----+      |
#   |             |
#   +----> D -----+
graph = {
    "A": ["C"],
    "B": ["C", "D"],
    "C": ["E"],
    "D": ["E"],
    "E": [],
}


def dfs(graph, start, state, result):
    stack = [start]

    while stack:
        node = stack[-1]

        if state.get(node) == 2:
            stack.pop()
            continue

        if state.get(node) == 1:
            state[node] = 2
            result.append(node)
            stack.pop()
            continue

        state[node] = 1
        for n in graph[node]:
            if state.get(n) == 1:
                raise ValueError(f"Cycle detected: {node} -> {n}")
            if state.get(n) != 2:
                stack.append(n)


def topological_sort(graph):
    result = []
    state = {}

    # O(V + E)
    for node in graph:
        if node not in state:
            dfs(graph, node, state, result)

    return list(reversed(result))


print(topological_sort(graph))
