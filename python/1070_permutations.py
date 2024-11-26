import itertools

n = int(input())

if n == 1:
    print(1)
elif n < 4:
    print("NO SOLUTION")
else:
    print(
        " ".join(
            str(x) for x in itertools.chain(range(2, n + 1, 2), range(1, n + 1, 2))
        )
    )
