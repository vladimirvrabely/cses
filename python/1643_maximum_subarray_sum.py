n = int(input())
a = list(map(int, input().split()))

# Kadane's algorithm
best = float("-inf")
curr = 0

for x in a:
    curr = max(x, curr + x)
    best = max(best, curr)
print(best)
