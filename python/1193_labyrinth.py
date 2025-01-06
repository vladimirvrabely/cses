from collections import deque
n, m = map(int, input().split())

labyrinth = []
for _ in range(n):
    labyrinth.append(input())

for row in range(n):
    for col in range(m):
        if labyrinth[row][col] == "A":
            a = (row, col)
        if labyrinth[row][col] == "B":
            b = (row, col)

DIRECTION_ROW = [-1, 0, 1, 0]
DIRECTION_COL = [0, -1, 0, 1]
DIRECTION = "ULDR"

seen = [[False for _ in range(m)] for _ in range(n)]
parent = [[None for _ in range(m)] for _ in range(n)]
connected = False

to_see = deque()
to_see.append(a)

while to_see:
    row, col = to_see.popleft()

    if (row, col) == b:
        connected = True
        break

    if seen[row][col]:
        continue

    seen[row][col] = True
    # Check neighbours
    for d in range(4):
        r = row + DIRECTION_ROW[d]
        c = col + DIRECTION_COL[d]
        if 0 <= r < n and 0 <= c < m and not seen[r][c] and labyrinth[r][c] != "#":
            parent[r][c] = d
            to_see.append((r, c))

if connected:
    path = []
    row, col = b
    while (row, col) != a:
        d = parent[row][col]
        path.append(DIRECTION[d])
        row -= DIRECTION_ROW[d]
        col -= DIRECTION_COL[d]
    path = "".join(reversed(path))

    print("YES")
    print(len(path))
    print(path)
else:
    print("NO")