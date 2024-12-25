n, x = map(int, input().split())

prices = list(map(int, input().split()))
pages = list(map(int, input().split()))

values = [[0 for _ in range(x+1)] for _ in range(n+1)]

for i in range(1, n+1):
    for j in range(x+1):
        if prices[i-1] > j:
            values[i][j] = values[i-1][j]
        else:
            values[i][j] = max(values[i-1][j], values[i-1][j-prices[i-1]] + pages[i-1])

print(values[-1][-1])