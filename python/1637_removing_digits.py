n = int(input())

dp = [n for _ in range(n + 1)]
dp[0] = 0

for i in range(1, n + 1):
    x = i
    while x:
        d = x % 10
        dp[i] = min(dp[i], dp[i - d] + 1)
        x //= 10
print(dp[-1])
