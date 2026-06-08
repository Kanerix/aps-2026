points = [(1, 3), (1, 1), (5, 6), (3, 1), (4, 3), (4, 2), (7, 2)]


def cross(origin, a, b):
    ax = a[0] - origin[0]
    ay = a[1] - origin[1]
    bx = b[0] - origin[0]
    by = b[1] - origin[1]
    return ax * by - ay * bx


def convex_hull(points):
    # O(n log n)
    points.sort()

    # O(n)
    lower = []
    for p in points:
        while len(lower) >= 2 and cross(lower[-2], lower[-1], p) <= 0:
            lower.pop()
        lower.append(p)

    upper = []
    for p in reversed(points):
        while len(upper) >= 2 and cross(upper[-2], upper[-1], p) <= 0:
            upper.pop()
        upper.append(p)
    return lower[:-1] + upper[:-1]


print(convex_hull(points))
