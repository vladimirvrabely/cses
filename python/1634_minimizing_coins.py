n, money = map(int, input().split())
coins = list(map(int, input().split()))

values = [money + 1 for _ in range(money + 1)]
values[0] = 0

for c in coins:
    for m in range(1, money + 1):
        if m - c >= 0 and values[m - c] + 1 < values[m]:
            values[m] = values[m - c] + 1

if values[-1] == money + 1:
    print(-1)
else:
    print(values[-1])
