from itertools import permutations

s = input()
s = "".join(sorted(list(s)))
lines = set("".join(p) for p in permutations(s))

print(len(lines))
for line in lines:
    print(line)
