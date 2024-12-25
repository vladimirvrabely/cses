n, x = map(int, input().split())

prices = list(map(int, input().split()))
pages = list(map(int, input().split()))

values = [0 for _ in range(x + 1)]

for i in range(n):
    for j in range(x, prices[i] - 1, -1):
        values[j] = max(values[j], pages[i] + values[j - prices[i]])

print(values[-1])
