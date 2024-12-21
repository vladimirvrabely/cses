n, money = map(int, input().split())
coins = list(map(int, input().split()))

MOD = 1_000_000_007

values = [0 for _ in range(money + 1)]
values[0] = 1

for c in coins:
    for m in range(money + 1):
        if m >= c:
            values[m] += values[m - c] % MOD

print(values[money] % MOD)
