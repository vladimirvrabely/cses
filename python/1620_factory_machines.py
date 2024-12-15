n, goal = map(int, input().split())
machines = list(map(int, input().split()))

low, high = 0, min(machines) * goal
time = 0

while low <= high:
    mid = (low + high) // 2

    # Produced until time mid
    produced = sum(mid // m for m in machines)
    if produced >= goal:
        time = mid
        high = mid - 1
    else:
        low = mid + 1

print(time)
