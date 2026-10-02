#!/usr/bin/env python3
"""PR 3814 delta review: apply one source mutant, run a nextest filter, revert.

usage: delta3814_mutants.py <mutant> [<nextest -E expr>] [--partition k/n]
"""
import pathlib, subprocess, sys, os

R = pathlib.Path(__file__).resolve().parent.parent
CC = "crates/topo/src/boolean/carrier_cross.rs"
RD = "crates/topo/src/boolean/reduce.rs"
RS = "crates/topo/src/boolean/rest.rs"
VF = "crates/topo/src/boolean/vtxfac.rs"

M = {
    # boundary_crossing answers Clear / Unread before reading anything.
    "clear": (CC, "    let mid = geom::mid_param(t0, t1);\n",
              "    if true { return Ok(BoundaryCrossing::Clear); }\n    let mid = geom::mid_param(t0, t1);\n"),
    "unread": (CC, "    let mid = geom::mid_param(t0, t1);\n",
               "    if true { return Ok(BoundaryCrossing::Unread); }\n    let mid = geom::mid_param(t0, t1);\n"),
    # The circle arm stops asking, but keeps its interior certificate.
    "no_circle_call": (RD, "                    if on_carrier\n                        && let Some(event) =\n",
                       "                    if on_carrier && false\n                        && let Some(event) =\n"),
    # The line arm stops asking, but keeps its interior certificate.
    "no_line_call": (RD, "            if on_carrier\n                && let Some(event) = on_carrier_crossing",
                     "            if on_carrier && false\n                && let Some(event) = on_carrier_crossing"),
    # A lone ring vertex's loop is skipped instead of answering Unread.
    "lone_skip": (CC, "Some(LoopBoundary::Empty { .. }) => return Ok(BoundaryCrossing::Unread),",
                  "Some(LoopBoundary::Empty { .. }) => continue,"),
    # The line arm's certificate regardless of the carrier-identity gate.
    "line_gate_true": (RD, "Placement::declared([Some(hu), Some(hv)], on_carrier)",
                       "Placement::declared([Some(hu), Some(hv)], true)"),
    # vtxfac's declared-Rest gate always open.
    "vtxfac_gate": (VF, "Some(carrier) if declared_rest => carrier,", "Some(carrier) => carrier,"),
    # declared(): an Undecided end records.
    "undecided_records": (RD, "        if on.clone().any(|p| *p == Self::Undecided) {\n            None\n",
                          "        if on.clone().any(|p| *p == Self::Undecided) {\n            Some(CurvedEvent::Recorded)\n"),
    # Twin never found: straight chords everywhere (the pre-fix-pass mint).
    "no_twin": (RS, "        let Some(edge) = fan_edge_between(other, ou, ov)? else {",
                "        let Some(edge) = None::<EdgeKey>.or(fan_edge_between(other, ou, ov)?).filter(|_| false) else {"),
    # Twin's direction read backwards.
    "twin_flip": (RS, "start: if first == ou { u } else { v },", "start: if first == ou { v } else { u },"),
}

name = sys.argv[1]
expr = sys.argv[2] if len(sys.argv) > 2 and not sys.argv[2].startswith("--") else None
part = None
if "--partition" in sys.argv:
    part = sys.argv[sys.argv.index("--partition") + 1]
f, old, new = M[name]
p = R / f
src = p.read_text()
assert src.count(old) == 1, f"{name}: anchor found {src.count(old)} times"
p.write_text(src.replace(old, new))
try:
    cmd = ["cargo", "nextest", "run", "-p", "topo", "-p", "sweep", "-p", "editor-core",
           "--no-fail-fast", "--status-level", "fail", "--final-status-level", "fail"]
    if expr:
        cmd += ["-E", expr]
    if part:
        cmd += ["--partition", f"count:{part}"]
    env = dict(os.environ, CARGO_INCREMENTAL="0")
    r = subprocess.run(cmd, cwd=R, env=env, capture_output=True, text=True, timeout=590)
    out = r.stdout + r.stderr
    for line in out.splitlines():
        s = line.strip()
        if s.startswith(("FAIL", "TIMEOUT", "SIGABRT", "SIGSEGV", "Summary", "error[", "error:")) \
                or "panicked at" in s or "the mate does not union" in s:
            print(f"[{name}] {s[:260]}")
    for line in out.splitlines():
        if "assertion" in line or "Err(" in line[:200]:
            pass
finally:
    p.write_text(src)
