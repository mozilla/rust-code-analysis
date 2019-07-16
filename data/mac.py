import os.path

macs = [
    "PRId{}", "PRIi{}", "PRIu{}", "PRIo{}", "PRIx{}", "PRIX{}",
    "PRIdLEAST{}", "PRIiLEAST{}", "PRIuLEAST{}", "PRIoLEAST{}", "PRIxLEAST{}", "PRIXLEAST{}",
    "PRIdFAST{}", "PRIiFAST{}", "PRIuFAST{}", "PRIoFAST{}", "PRIxFAST{}", "PRIXFAST{}",
    "PRIdMAX", "PRIiMAX", "PRIuMAX", "PRIoMAX", "PRIxMAX", "PRIXMAX",
    "PRIdPTR", "PRIiPTR", "PRIuPTR", "PRIoPTR", "PRIxPTR", "PRIXPTR",
    "SCNd{}", "SCNi{}", "SCNu{}", "SCNo{}", "SCNx{}",
    "SCNdLEAST{}", "SCNiLEAST{}", "SCNuLEAST{}", "SCNoLEAST{}", "SCNxLEAST{}",
    "SCNdFAST{}", "SCNiFAST{}", "SCNuFAST{}", "SCNoFAST{}", "SCNxFAST{}",
    "SCNdMAX", "SCNiMAX", "SCNuMAX", "SCNoMAX", "SCNxMAX",
    "SCNdPTR", "SCNiPTR", "SCNuPTR", "SCNoPTR", "SCNxPTR",
    "INT{}_MIN", "INT_FAST{}_MIN", "INT_LEAST{}_MIN", "INT{}_C",
    "INTPTR_MIN", "INTMAX_MIN",
    "INT{}_MAX", "INT_FAST{}_MAX", "INT_LEAST{}_MAX",
    "INTPTR_MAX", "INTMAX_MAX",
    "UINT{}_MIN", "UINT_FAST{}_MIN", "UINT_LEAST{}_MIN", "UINT{}_C",
    "UINTPTR_MIN", "UINTMAX_MIN",
    "UINT{}_MAX", "UINT_FAST{}_MAX", "UINT_LEAST{}_MAX",
    "UINTPTR_MAX", "UINTMAX_MAX",
]

macros = set()

for x in macs:
    for i in [8, 16, 32, 64]:
        macros.add(x.format(i))

old = set()

if os.path.isfile("./c_macros.txt"):
    with open("./c_macros.txt", "r") as In:
        for l in In.readlines():
            l = l.strip()
            old.add(l)

diff = macros - old
if diff:
    for d in diff:
        old.add(d)
    with open("./c_macros.txt", "w") as Out:
        for x in sorted(old):
            Out.write(f"{x}\n")
