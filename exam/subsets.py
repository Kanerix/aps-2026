n = 3

subset = []
all_subsets = []


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
