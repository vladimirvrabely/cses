n = int(input())
arr = list(map(int, input().split()))
arr.sort()

m = arr[n // 2]
s = sum(abs(x - m) for x in arr)
print(s)
