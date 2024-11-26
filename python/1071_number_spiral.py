def get_value(row: int, col: int) -> int:
    k = max(row, col)
    # Diagonal value
    d = k * (k - 1) + 1
    # Add or subtract along the axis
    if k % 2 == 0:
        val = d - (col - row)
    else:
        val = d + (col - row)
    return val


n = int(input())

for i in range(n):
    row, col = [int(x) for x in input().split()]
    print(get_value(row, col))
