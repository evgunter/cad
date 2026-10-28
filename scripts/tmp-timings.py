#!/usr/bin/env python3
"""THROWAWAY (green/latency-shared-kernel-build): summarise cargo's --timings
report for the test shards. Removed before the branch is proposed."""
import json, os, re, subprocess, sys
from collections import defaultdict

html = open(sys.argv[1]).read()
m = re.search(r"const UNIT_DATA = (\[.*?\]);\n", html, re.S)
units = json.loads(m.group(1))
meta = json.loads(subprocess.run(["cargo", "metadata", "--no-deps", "--format-version", "1"],
                                 capture_output=True, text=True, check=True).stdout)
members = {p["name"] for p in meta["packages"]}
wall = max(u["start"] + u["duration"] for u in units)
groups = defaultdict(lambda: [0, 0.0])
ws_lib = []
for u in units:
    ws = u["name"] in members
    kind = "lib" if u["target"].strip() in ("", "(lib)") or "lib" == u["target"].strip(" ()") else (
        "test" if "test" in u["target"] else u["target"].strip() or "lib")
    if u.get("mode") == "run-custom-build" or "build script" in u["target"]:
        kind = "build-script"
    key = ("ws " if ws else "dep ") + kind
    groups[key][0] += 1
    groups[key][1] += u["duration"]
    if ws:
        ws_lib.append((u["start"], u["duration"], u["name"], u["target"], u.get("mode")))
print(f"TIMINGS wall {wall:.0f}s units {len(units)}")
for k, (n, d) in sorted(groups.items()):
    print(f"TIMINGS {k:24s} n={n:3d} unit-s {d:7.0f}")
for s, d, n, t, mo in sorted(ws_lib):
    print(f"TIMINGS unit {n:14s} {t:28s} {mo or '':10s} start {s:6.0f} end {s+d:6.0f} dur {d:5.0f}")
