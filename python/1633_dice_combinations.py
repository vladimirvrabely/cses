n = int(input())

counts = [1]
for x in range(n):
    counts.append(sum(counts[-6:]) % 1_000_000_007)
print(counts[-1])
