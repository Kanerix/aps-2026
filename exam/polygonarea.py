points = [(1, 1), (3, 1), (4, 2), (7, 2), (5, 6), (4, 3), (1, 3)]


def cross(origin, a, b):
    ax = a[0] - origin[0]
    ay = a[1] - origin[1]
    bx = b[0] - origin[0]
    by = b[1] - origin[1]
    return ax * by - ay * bx


def polygon_area(points):
    n = len(points)
    area = 0

    # O(n)
    for i in range(n):
        a = points[i]
        b = points[(i + 1) % n]
        # Origin is (0, 0) always
        area += cross((0, 0), a, b)
    return abs(area) / 2


print(polygon_area(points))
