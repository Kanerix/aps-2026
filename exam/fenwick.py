input = [6, 2, 5, 7, 1, 4, 2, 9]


# largest power of two that divides k
def p(k):
    # k & (~k - 1)
    return k & (-k)


def build(array):
    tree = [0] + [0] * len(array)

    # O(n log n)
    for k in range(1, len(tree)):
        start = k - p(k) + 1
        tree[k] = sum(array[i - 1] for i in range(start, k + 1))

    return tree[1:]


def prefix_sum(tree, k):
    s = 0
    # O(log k)
    while k > 0:
        s += tree[k]
        k -= p(k)
    return s


def range_sum(tree, start, end):
    return prefix_sum(tree, end) - prefix_sum(tree, start - 1)


def add(tree, k, delta):
    while k < len(tree):
        tree[k] += delta
        k += p(k)


tree = build(input)
print(tree)
