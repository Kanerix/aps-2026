# Weighted directed graph represented as an adjacency list
# graph[u] = list of (v, capacity, flow) edges from u to v
#
# Example graph:
#
#        10        10
#   s ------> a ------> t
#   |         ^         ^
#   |    5    |    15   |
#   +-------> b --------+
#             ^
#        8    |
#   c --------+
graph = {
    "s": [("a", 10, 0), ("b", 5, 0)],
    "a": [("t", 10, 0)],
    "b": [("a", 5, 0), ("t", 15, 0)],
    "c": [("b", 8, 0)],
}

source = "s"
sink = "t"


def dfs(u, flow):
    return


def max_flow():
    return
