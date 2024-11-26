n = int(input())

# There are at least as many 2 as 5 in the factorial product
zeros = 0
m = 5
while m <= n:
    zeros += n // m
    m *= 5
print(zeros)
