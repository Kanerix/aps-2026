a, b, h = map(int, input().split())
height = 0
climbs = 0
while True:
    height += a
    climbs += 1
    if height >= h:
        break
    height -= b
print(climbs)