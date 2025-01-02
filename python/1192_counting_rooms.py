n, m = map(int, input().split())

building = []
for _ in range(n):
    building.append(list(input()))


components = 0


def dfs(row, col):
    to_visit = [(row, col)]
    while to_visit:
        row, col = to_visit.pop()
        if building[row][col] == ".":
            building[row][col] = "x"
            if row > 0:
                to_visit.append((row - 1, col))
            if col > 0:
                to_visit.append((row, col - 1))
            if row + 1 < n:
                to_visit.append((row + 1, col))
            if col + 1 < m:
                to_visit.append((row, col + 1))


for row in range(n):
    for col in range(m):
        if building[row][col] == ".":
            dfs(row, col)
            components += 1
print(components)
