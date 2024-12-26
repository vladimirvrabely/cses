from itertools import accumulate

n = int(input())

tasks = []
for _ in range(n):
    duration, deadline = map(int, input().split())
    tasks.append((duration, deadline))

tasks.sort(key=lambda x: x[0])
ends = list(accumulate(duration for duration, _ in tasks))

res = sum(deadline - end for (_, deadline), end in zip(tasks, ends))
print(res)
