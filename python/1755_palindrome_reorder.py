from collections import Counter

s = input()

counter = Counter(s)

even = []
odd = []
for c, n in counter.items():
    if n % 2 == 0:
        even.append(c * (n // 2))
    else:
        odd.append(c * n)

if len(odd) > 1:
    print("NO SOLUTION")
else:
    print("".join(even + odd + even[::-1]))
