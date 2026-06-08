n = 8

count = 0
column = [False] * n
diag1 = [False] * (2 * n - 1)
diag2 = [False] * (2 * n - 1)


def search(y):
    global count
    if y == n:
        count += 1
        return
    for x in range(n):
        if column[x] or diag1[x + y] or diag2[x - y + n - 1]:
            continue
        column[x] = diag1[x + y] = diag2[x - y + n - 1] = True
        search(y + 1)
        column[x] = diag1[x + y] = diag2[x - y + n - 1] = False


search(0)
print(count)
