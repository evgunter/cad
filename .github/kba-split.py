"""Split crates/<crate>/tests/all.rs into N binaries balanced by suite line count (measurement only)."""
import re, sys, pathlib
root = pathlib.Path(sys.argv[1]); n = int(sys.argv[2])
t = root / "tests"; src = (t / "all.rs").read_text().splitlines()
helpers, suites, i = [], [], 0
while i < len(src):
    l = src[i]
    m = re.match(r'#\[path = "([^"]+)"\]$', l)
    if m:
        suites.append((l, src[i + 1], (t / m.group(1)).read_text().count("\n"))); i += 2; continue
    if re.match(r"mod [a-z0-9_]+;$", l): helpers.append(l)
    i += 1
tot = sum(s[2] for s in suites); groups = [[] for _ in range(n)]; acc = 0
for s in suites:
    groups[min(n - 1, acc * n // tot)].append(s); acc += s[2]
for k, g in enumerate(groups):
    name = "all" if k == 0 else f"all{k+1}"
    body = helpers + [x for s in g for x in s[:2]]
    if k == 0: body.append("test_utils::every_suite_file_is_aggregated!();")
    (t / f"{name}.rs").write_text("\n".join(body) + "\n")
    print(name, len(g), "suites", sum(s[2] for s in g), "lines")
man = root / "Cargo.toml"; txt = man.read_text()
txt += "".join(f'\n[[test]]\nname = "all{k+1}"\npath = "tests/all{k+1}.rs"\n' for k in range(1, n))
man.write_text(txt)
