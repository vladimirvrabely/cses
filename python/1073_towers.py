from bisect import bisect

n = int(input())
blocks = list(map(int, input().split()))

# Keep track of the top block on the tower
towers: list[int] = []

for x in blocks:
    i = bisect(towers, x)
    if i == len(towers):
        towers.append(x)
    else:
        towers[i] = x

print(len(towers))
