#!/usr/bin/env python3
"""Join a probe run's per-pass times onto the candidate table.

usage: probe.py <probe run id> <attempt> <cand.tsv> <out.tsv>
The probe job runs each shard's candidates in three nextest passes: -j1,
-j1 again, then at default parallelism on the same runner.

Kept for the drift check PR 7397 proposes, which reuses this reader.
"""

import json
import re
import subprocess
import sys

run, attempt, cand, out = sys.argv[1:]
ANSI = re.compile(r"\x1b\[[0-9;]*m")
PASS = re.compile(
    r"^\S+Z\s+PASS\s+\[\s*([0-9.]+)s\]\s+(?:\(\s*\d+/\d+\)\s+)?(\S+)\s+(\S+)\s*$"
)
jobs = json.loads(
    subprocess.run(
        ["gh", "api", f"repos/evgunter/cad/actions/runs/{run}/attempts/{attempt}/jobs"],
        check=True,
        capture_output=True,
        text=True,
    ).stdout
)["jobs"]
times: dict[str, list[float]] = {}
cpu = {}
for j in jobs:
    log = subprocess.run(
        [
            "gh",
            "api",
            f"repos/evgunter/cad/actions/jobs/{j['id']}/logs",
            "--allow-escape-sequences",
        ],
        capture_output=True,
        text=True,
    ).stdout
    step = -1
    for raw in log.splitlines():
        line = ANSI.sub("", raw)
        if "##[group]Run cargo nextest run --profile ci" in line:
            step += 1
        if "Model name" in line:
            cpu[j["name"]] = line.split("Model name:")[-1].strip()
        m = PASS.match(line)
        if m and step >= 0:
            times.setdefault(f"{m.group(2)} {m.group(3)}", [None, None, None])[step] = (
                float(m.group(1))
            )
print(cpu)
with open(cand) as f, open(out, "w") as o:
    head = f.readline().rstrip("\n")
    o.write(head + f"\tprobe_j1_a\tprobe_j1_b\tprobe_parallel\t# {cpu}\n")
    for line in f:
        t = line.rstrip("\n").split("\t")[-1]
        p = times.get(t)
        if p is None:
            continue
        o.write(
            line.rstrip("\n")
            + "\t"
            + "\t".join("" if v is None else f"{v:.3f}" for v in p)
            + "\n"
        )
