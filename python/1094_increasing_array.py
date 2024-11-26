n = int(input())
a = [int(x) for x in input().split()]

p = a[0]
m = 0
for i in range(1, n):
    x = max(p - a[i], 0)
    m += x
    a[i] += x
    p = a[i]
print(m)
