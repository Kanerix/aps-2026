import sys
from collections import deque

input_data = sys.stdin.read().split()
idx = 0

while True:
    L = int(input_data[idx])
    R = int(input_data[idx + 1])
    C = int(input_data[idx + 2])

    idx += 3

    if L == 0 and R == 0 and C == 0:
        break

    dungeon = []
    start = None
    end = None

    for level in range(L):
        grid = []
        for row in range(R):
            line = input_data[idx]
            idx += 1
            grid.append(line)
            for col in range(C):
                if line[col] == "S":
                    start = (level, row, col)
                elif line[col] == "E":
                    end = (level, row, col)
        dungeon.append(grid)

    visited = [[[False] * C for _ in range(R)] for _ in range(L)]
    sl, sr, sc = start
    visited[sl][sr][sc] = True
    queue = deque()
    queue.append((sl, sr, sc, 0))

    directions = [
        (1, 0, 0),
        (-1, 0, 0),
        (0, 1, 0),
        (0, -1, 0),
        (0, 0, 1),
        (0, 0, -1),
    ]

    found = False
    while queue:
        cl, cr, cc, dist = queue.popleft()

        if (cl, cr, cc) == end:
            print(f"Escaped in {dist} minute(s).")
            found = True
            break

        for dl, dr, dc in directions:
            nl, nr, nc = cl + dl, cr + dr, cc + dc
            if 0 <= nl < L and 0 <= nr < R and 0 <= nc < C:
                if not visited[nl][nr][nc] and dungeon[nl][nr][nc] != "#":
                    visited[nl][nr][nc] = True
                    queue.append((nl, nr, nc, dist + 1))

    if not found:
        print("Trapped!")
