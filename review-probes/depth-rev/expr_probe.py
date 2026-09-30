import sys
from pncad import Doc
n = int(sys.argv[1])
kind = sys.argv[2]
d = Doc("expr-probe")
if kind == "paren":
    src = "(" * n + "1" + ")" * n
elif kind == "neg":
    src = "-" * n + "1"
else:
    src = "+".join(["1"] * n)
e = d.parse_expr(src)
print("parsed", kind, n)
