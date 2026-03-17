import sys
from collections import deque


def main():
    input_data = sys.stdin.buffer.read().split()
    idx = 0
    n = int(input_data[idx])
    idx += 1
    m = int(input_data[idx])
    idx += 1
    s = int(input_data[idx])
    idx += 1
    t = int(input_data[idx])
    idx += 1

    graph = [[] for _ in range(n)]

    edge_info = []

    def add_edge(u, v, cap):
        idx_uv = len(graph[u])
        idx_vu = len(graph[v])
        graph[u].append([v, idx_vu, cap, 0])
        graph[v].append([u, idx_uv, 0, 0])
        edge_info.append((u, v, cap, idx_uv))

    for _ in range(m):
        u = int(input_data[idx])
        idx += 1
        v = int(input_data[idx])
        idx += 1
        c = int(input_data[idx])
        idx += 1
        add_edge(u, v, c)

    # Dinic's algorithm
    level = [-1] * n
    it = [0] * n

    def bfs():
        for i in range(n):
            level[i] = -1
        level[s] = 0
        queue = deque()
        queue.append(s)
        while queue:
            u = queue.popleft()
            for e in graph[u]:
                if e[2] - e[3] > 0 and level[e[0]] < 0:
                    level[e[0]] = level[u] + 1
                    queue.append(e[0])
        return level[t] >= 0

    def dfs(u, pushed):
        if u == t:
            return pushed
        while it[u] < len(graph[u]):
            e = graph[u][it[u]]
            v = e[0]
            rem = e[2] - e[3]
            if rem > 0 and level[v] == level[u] + 1:
                d = dfs(v, min(pushed, rem))
                if d > 0:
                    e[3] += d
                    graph[v][e[1]][3] -= d
                    return d
            it[u] += 1
        return 0

    total_flow = 0
    while bfs():
        for i in range(n):
            it[i] = 0
        while True:
            f = dfs(s, float("inf"))
            if f == 0:
                break
            total_flow += f

    result_edges = []
    for i in range(len(edge_info)):
        u, v, cap, ei = edge_info[i]
        flow = graph[u][ei][3]
        if flow > 0:
            result_edges.append((u, v, flow))

    out = []
    out.append(f"{n} {total_flow} {len(result_edges)}")
    for u, v, f in result_edges:
        out.append(f"{u} {v} {f}")
    print("\n".join(out))


if __name__ == "__main__":
    main()
