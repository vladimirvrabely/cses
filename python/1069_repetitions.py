s = input()
longest = 0
c = s[0]
n = 0

for x in s:
    if x == c:
        n += 1
    else:
        longest = max(longest, n)
        n = 1
        c = x
longest = max(longest, n)
print(longest)
