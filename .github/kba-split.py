"""Split crates/<crate>/tests/all.rs into N binaries balanced by line count,
keeping suites that name each other (`crate::<suite>`) in one binary (measurement only)."""
import re, sys, pathlib
root = pathlib.Path(sys.argv[1]); n = int(sys.argv[2])
t = root / "tests"; src = (t / "all.rs").read_text().splitlines()
helpers, suites, i = [], {}, 0
while i < len(src):
    l = src[i]
    m = re.match(r'#\[path = "([^"]+)"\]$', l)
    if m:
        name = re.match(r"mod ([a-z0-9_]+);", src[i + 1]).group(1)
        text = (t / m.group(1)).read_text()
        suites[name] = (l, src[i + 1], text.count("\n"), text); i += 2; continue
    if re.match(r"mod [a-z0-9_]+;$", l): helpers.append(l)
    i += 1
parent = {s: s for s in suites}
def find(x):
    while parent[x] != x: parent[x] = parent[parent[x]]; x = parent[x]
    return x
for s, v in suites.items():
    for ref in set(re.findall(r"(?:crate|super)::([a-z0-9_]+)", v[3])):
        if ref in suites: parent[find(ref)] = find(s)
comps = {}
for s in suites: comps.setdefault(find(s), []).append(s)
comps = sorted(comps.values(), key=lambda c: -sum(suites[s][2] for s in c))
groups = [[] for _ in range(n)]; load = [0] * n
for c in comps:
    k = load.index(min(load)); groups[k] += c; load[k] += sum(suites[s][2] for s in c)
for k, g in enumerate(groups):
    name = "all" if k == 0 else f"all{k+1}"
    body = helpers + [x for s in sorted(g) for x in suites[s][:2]]
    if k == 0: body.append("test_utils::every_suite_file_is_aggregated!();")
    (t / f"{name}.rs").write_text("\n".join(body) + "\n")
    print(name, len(g), "suites", load[k], "lines")
man = root / "Cargo.toml"
man.write_text(man.read_text() + "".join(f'\n[[test]]\nname = "all{k+1}"\npath = "tests/all{k+1}.rs"\n' for k in range(1, n)))
