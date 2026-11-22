#!/usr/bin/env python3
"""Collect per-test default-eps times from PR gate job logs.

usage: collect.py <out.tsv> <max_runs>
Appends rows: run_id job_id shard head_sha conclusion test seconds

Reads the PASS lines of the default-eps step of `test (kernel)` and
`test (app)`: the step check-fast-set-times.py holds to its bars, at the
gate's own build (libraries at opt 1, integration binaries at opt 0) and
parallelism. Resumable: jobs already in <out.tsv>.jobs are skipped. The
junit report is not uploaded by the gate, so the job log is the record.

Kept for the drift check PR 7397 proposes, which reuses this reader.
"""

import json
import os
import re
import subprocess
import sys

out, max_runs = sys.argv[1], int(sys.argv[2])
REPO = "repos/evgunter/cad"
ANSI = re.compile(r"\x1b\[[0-9;]*m")
PASS = re.compile(
    r"^\S+Z\s+(PASS|FLAKY \d+/\d+)\s+\[\s*([0-9.]+)s\]\s+(?:\(\s*\d+/\d+\)\s+)?(\S+)\s+(\S+)\s*$"
)
done_jobs = set()
if os.path.exists(out):
    for line in open(out):
        done_jobs.add(line.split("\t")[1])
seen_jobs = (
    set(s.strip() for s in open(out + ".jobs"))
    if os.path.exists(out + ".jobs")
    else set()
)


def gh(*a):
    return subprocess.run(
        ["gh", "api", *a], check=True, capture_output=True, text=True
    ).stdout


runs = json.loads(
    subprocess.run(
        [
            "gh",
            "run",
            "list",
            "-R",
            "evgunter/cad",
            "--workflow",
            "ci.yml",
            "--event",
            "pull_request",
            "--limit",
            "300",
            "--json",
            "databaseId,status,headSha,createdAt",
        ],
        check=True,
        capture_output=True,
        text=True,
    ).stdout
)
n = 0
for r in runs:
    if r["status"] != "completed":
        continue
    jobs = json.loads(gh(f"{REPO}/actions/runs/{r['databaseId']}/jobs?per_page=100"))[
        "jobs"
    ]
    took = False
    for j in jobs:
        if j["name"] not in ("test (kernel)", "test (app)"):
            continue
        step = next(
            (
                s
                for s in j.get("steps", [])
                if s["name"].startswith("test (default eps")
            ),
            None,
        )
        if not step or step["conclusion"] not in ("success", "failure"):
            continue
        took = True
        jid = str(j["id"])
        if jid in done_jobs or jid in seen_jobs:
            continue
        log = subprocess.run(
            [
                "gh",
                "api",
                f"{REPO}/actions/jobs/{jid}/logs",
                "--allow-escape-sequences",
            ],
            capture_output=True,
            text=True,
        ).stdout
        rows, inseg = [], False
        for raw in log.splitlines():
            line = ANSI.sub("", raw)
            if "##[group]Run " in line:
                inseg = line.rstrip().endswith("Run rc=0")
                continue
            if not inseg:
                continue
            m = PASS.match(line)
            if m:
                rows.append((m.group(3), m.group(4), m.group(2)))
        shard = j["name"][6:-1]
        with open(out, "a") as f:
            for b, t, s in rows:
                f.write(
                    f"{r['databaseId']}\t{jid}\t{shard}\t{r['headSha'][:10]}\t{step['conclusion']}\t{b} {t}\t{s}\n"
                )
        with open(out + ".jobs", "a") as f:
            f.write(jid + "\n")
        print(r["databaseId"], j["name"], len(rows), r["createdAt"], flush=True)
    if took:
        n += 1
        if n >= max_runs:
            break
