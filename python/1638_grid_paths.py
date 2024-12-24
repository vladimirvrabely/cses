n = int(input())

MOD = 1_000_000_007
board = []
for _ in range(n):
    board.append(list(input()))

dp = [[0 for _ in range(n)] for _ in range(n)]
dp[0][0] = int(board[0][0] == ".")

for row in range(n):
    for col in range(n):
        if board[row][col] != "*":
            if row - 1 >= 0:
                dp[row][col] += dp[row - 1][col] % MOD
            if col - 1 >= 0:
                dp[row][col] += dp[row][col - 1] % MOD

print(dp[-1][-1] % MOD)
