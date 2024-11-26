n = int(input())
playlist = list(map(int, input().split()))

m = 0
start = 0
# Last occurrence of the given song
last: dict[int, int] = {}
for end, song in enumerate(playlist):
    if song in last and start <= last[song]:
        start = last[song] + 1
    last[song] = end
    m = max(m, end - start + 1)
print(m)
