from collections import deque

n = int(input())
bosses = list(map(int, input().split()))
bosses = [-1] + [x-1 for x in bosses]
# print(bosses)

# or dfs?
nobosses = set(range(n)).difference(bosses)

suboordinates = [0 for _ in range(n)]
visited = [False for _ in range(n)]

for x in nobosses:
    subs = 1
    while bosses[x] != -1: 
        if not visited[x]:
            subs = suboordinates[x] + 1
        suboordinates[bosses[x]] += subs
        visited[x] = True
        x = bosses[x]
        


print(" ".join(map(str, suboordinates)))
