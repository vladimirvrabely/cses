n = int(input())

arrivals = []
departures = []
for _ in range(n):
    a, d = map(int, input().split())
    arrivals.append(a)
    departures.append(d)

arrivals.sort()
departures.sort()

i, j = 0, 0

m = 0
s = 0
while i < n and j < n:
    if arrivals[i] < departures[j]:
        i += 1
        s += 1
    elif arrivals[i] > departures[j]:
        j += 1
        s -= 1
    else:
        i += 1
        j += 1

    m = max(s, m)

print(m)
