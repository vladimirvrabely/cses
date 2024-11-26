import sys
from bisect import bisect

sys.setrecursionlimit(200_000)

n, m = map(int, input().split())
prices = [int(x) for x in input().split()]
prices.sort()
customers = [int(x) for x in input().split()]

available = list(range(n))


def get_available(i: int) -> int:
    if i == -1 or i == available[i]:
        return i
    else:
        # Update pointers on the fly
        available[i] = get_available(available[i])
        return available[i]


for c in customers:
    i = get_available(bisect(prices, c) - 1)
    if i >= 0:
        print(prices[i])
        # Ticket is sold; point to the cheaper one
        available[i] = i - 1
    else:
        print(-1)
