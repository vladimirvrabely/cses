n, s = map(int, input().split())

arr = list(map(int, input().split()))

index: dict[int, int] = {}
pair = None
if s > 1:
    for i in range(n):
        complement = s - arr[i]
        if complement in index:
            pair = i + 1, index[complement] + 1
            break
        else:
            index[arr[i]] = i
if pair:
    print(pair[0], pair[1])
else:
    print("IMPOSSIBLE")
