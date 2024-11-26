n, x = map(int, input().split())
weights = list(map(int, input().split()))
weights.sort()

count = n
i, j = 0, n - 1
while i < j:
    if weights[i] + weights[j] <= x:
        i += 1
        j -= 1
        count -= 1
    else:
        j -= 1

print(count)
