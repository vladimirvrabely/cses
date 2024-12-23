n = int(input())
coins = list(map(int, input().split()))

coins.sort()
x = 1

for c in coins:
    if c > x:
        break
    else:
        x += c
print(x)
