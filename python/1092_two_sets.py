n = int(input())

s = n * (n + 1) / 2

if s % 2 == 0:
    print("YES")
    t = s // 2
    a = []
    b = []
    for i in range(n, 0, -1):
        print(f"{i = }")
        print(f"\t{t = }")
        if t >= i:
            a.append(i)
            t -= i
        else:
            b.append(i)
        print(f"\t{t = }")
        print(f"\t{a = }")
        print(f"\t{b = }")

    print(len(a))
    print(" ".join(map(str, a)))
    print(len(b))
    print(" ".join(map(str, b)))
else:
    print("NO")

7 * 8 / 2
t = 14
7
