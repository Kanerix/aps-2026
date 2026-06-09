n = 3

subset = []
all_subsets = []


def bit_repr(n):
    for b in range(1 << n):
        subset = []
        for i in range(n):
            if b & (1 << i):
                subset.append(i)
        # process subset


def search(k):
    if k == n:
        all_subsets.append(subset.copy())
    else:
        search(k + 1)
        subset.append(k)
        search(k + 1)
        subset.pop()


search(0)
print(all_subsets)
