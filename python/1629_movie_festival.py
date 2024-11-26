n = int(input())

movies = []
for _ in range(n):
    start, end = map(int, input().split())
    movies.append((start, end))

movies.sort(key=lambda x: x[1])

total = 0
end = 0
for m in movies:
    if end <= m[0]:
        total += 1
        end = m[1]


print(total)
