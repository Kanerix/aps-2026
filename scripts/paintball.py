import sys
from collections import deque


def main():
    input_data = sys.stdin.buffer.read().split()
    idx = 0
    n = int(input_data[idx])
    idx += 1
    m = int(input_data[idx])
    idx += 1

    adj = [[] for _ in range(n)]

    for _ in range(m):
        u = int(input_data[idx]) - 1
        idx += 1
        v = int(input_data[idx]) - 1
        idx += 1
        adj[u].append(v)
        adj[v].append(u)

    # Hopcroft-Karp algorithm
    match_left = [-1] * n
    match_right = [-1] * n
    INF = n + 1
    dist = [0] * n

    def bfs():
        queue = deque()
        for u in range(n):
            if match_left[u] == -1:
                dist[u] = 0
                queue.append(u)
            else:
                dist[u] = INF
        found = False
        while queue:
            u = queue.popleft()
            for v in adj[u]:
                w = match_right[v]
                if w == -1:
                    found = True
                elif dist[w] == INF:
                    dist[w] = dist[u] + 1
                    queue.append(w)
        return found

    def dfs(u):
        for v in adj[u]:
            w = match_right[v]
            if w == -1 or (dist[w] == dist[u] + 1 and dfs(w)):
                match_left[u] = v
                match_right[v] = u
                return True
        dist[u] = INF
        return False

    matching = 0
    while bfs():
        for u in range(n):
            if match_left[u] == -1:
                if dfs(u):
                    matching += 1

    if matching < n:
        print("Impossible")
    else:
        out = []
        for u in range(n):
            out.append(str(match_left[u] + 1))
        print("\n".join(out))


if __name__ == "__main__":
    main()
