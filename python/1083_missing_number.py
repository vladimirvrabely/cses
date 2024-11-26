n = int(input())
spots = [False for _ in range(n)]
for i in input().split():
    spots[int(i) - 1] = True

print(spots.index(False) + 1)
