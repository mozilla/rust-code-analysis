import os.path

# these ids mustn't be treated as macros
specs = [
    "int{}_t", "int_fast{}_t", "int_least{}_t",
    "uint{}_t", "uint_fast{}_t", "uint_least{}_t",
    "char{}_t",
    "bool", "char", "int", "long", "short", "float", "double",
    "size_t", "ssize_t", "intmax_t", "intptr_t", "uintptr_t",
    "uintmax_t", "charptr_t", "ptrdiff_t", "max_align_t", "wchar_t",
    "signed", "unsigned", "false", "true", "nullptr", "NULL",
    "static", "const", "inline", "restrict", "constexpr", "mutable", "explicit", "namespace",
]

specials = set()

for x in specs:
    for i in [8, 16, 32, 64]:
        specials.add(x.format(i))

old = set()

if os.path.isfile("./c_specials.txt"):
    with open("./c_specials.txt", "r") as In:
        for l in In.readlines():
            l = l.strip()
            old.add(l)

diff = specials - old
if diff:
    for d in diff:
        old.add(d)
    with open("./c_specials.txt", "w") as Out:
        for x in sorted(old):
            Out.write(f"{x}\n")
