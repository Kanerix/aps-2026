n = 3
restrictions = [(1, 2), (2, 3)]

subset = []
pizzas = []


def search(k):
    if k == n:
        for r in restrictions:
            if all(p in subset for p in r):
                return
        pizzas.append(subset.copy())
    else:
        search(k + 1)
        subset.append(k + 1)
        search(k + 1)
        subset.pop()


search(0)
print(pizzas)
print(len(pizzas))
