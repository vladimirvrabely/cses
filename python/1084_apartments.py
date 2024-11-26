n, m, k = map(int, input().split())
applicants = sorted(list(map(int, input().split())))
apartments = sorted(list(map(int, input().split())))

i = 0
j = 0
cnt = 0
while i < len(applicants) and j < len(apartments):
    applicant = applicants[i]
    apartment = apartments[j]

    if applicant > apartment + k:
        j += 1
    elif applicant < apartment - k:
        i += 1
    else:
        i += 1
        j += 1
        cnt += 1

print(cnt)
