n = int(input())
a = [int(x) for x in input().split()]

total = sum(a)
diff = total
# Iterate over powerset encoded to as bits
for mask in range(1 << n):
    # Sum over the elements in a subset (other set is complementary)
    partial = sum(a[i] for i in range(n) if mask & (1 << i))
    diff = min(diff, abs(total - 2 * partial))
print(diff)
