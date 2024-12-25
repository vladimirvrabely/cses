n, x = map(int, input().split())
prices = list(map(int, input().split()))
pages = list(map(int, input().split()))

dp = [0 for _ in range(x+1)]

for i in range(n):
    page = pages[i]
    price = prices[i]
    for j in range(x, price - 1, -1):
        dp[j] = max(page + dp[j - price], dp[j])

print(dp[-1])
