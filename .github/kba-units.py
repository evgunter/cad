import json, re, sys
h = open(sys.argv[1]).read()
m = re.search(r'const UNIT_DATA = (\[.*?\]);\n', h, re.S)
u = json.loads(m.group(1))
end = max(x["start"] + x["duration"] for x in u)
tot = sum(x["duration"] for x in u)
print(f"units={len(u)} end={end:.1f} unit_s={tot:.1f}")
pat = sys.argv[2] if len(sys.argv) > 2 else None
for x in sorted(u, key=lambda x: -x["duration"])[: int(sys.argv[3]) if len(sys.argv) > 3 else 15]:
    if pat and not re.search(pat, x["name"]): continue
    print(f'{x["start"]:7.1f} {x["duration"]:7.1f} {x.get("rmeta_time") or 0:7.1f} {x["name"]}{x["target"]} {x.get("mode","")}')
