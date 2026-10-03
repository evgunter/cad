#!/usr/bin/env python3
"""Reviewer mutant runner (PR #3982, lane reach-dual3982-r1).
Each mutant is a literal (old, new) edit of one file in the worktree
/home/user/mut; it is built, the selected rows run, and the file restored."""
import os, subprocess, sys
W = "/home/user/mut"
C = "crates/topo/src/splitting/classify.rs"
B = "crates/topo/src/boolean/boxes.rs"
OLD_CLEARS = """    let n = plane.normal.get();
    let at = n.x * plane.origin.x + n.y * plane.origin.y + n.z * plane.origin.z;
    let gap = (lo.x - at).max(at - hi.x) - pad;"""
WORLD_CLEARS = """    let n = plane.normal.get();
    let half = T::from_f64(0.5);
    let centre = Point3::new((lo.x + hi.x) * half, (lo.y + hi.y) * half, (lo.z + hi.z) * half);
    let support = n.x.abs() * ((hi.x - lo.x) * half + pad)
        + n.y.abs() * ((hi.y - lo.y) * half + pad)
        + n.z.abs() * ((hi.z - lo.z) * half + pad);
    let gap = crate::sector_shape::plane_offset(plane.origin, n, centre).abs() - support;"""
MUTANTS = {
    "M1-world-axis-box": [
        (C, "    let frame = BoxFrame::aimed(plane.normal);", "    let frame = BoxFrame::World;"),
        (C, "    let frame = &BoxFrame::aimed(plane.normal);", "    let frame = &BoxFrame::World;"),
        (C, OLD_CLEARS, WORLD_CLEARS),
    ],
    "M2-loosened-pad-x1e5": [
        (C, "    let gap = (lo.x - at).max(at - hi.x) - pad;", "    let gap = (lo.x - at).max(at - hi.x) - pad * T::from_f64(1e5);"),
    ],
    "M3-crest-offset-from-hi": [
        (C, "    let into = (crest - w.lo).reduce_periodic(T::tau());", "    let into = (crest - w.hi).reduce_periodic(T::tau());"),
    ],
    "M4-torus-v-crest-read-on-u-window": [
        (C, "most_cos(a.atan2(m), v)", "most_cos(a.atan2(m), u)"),
    ],
    "M5-pad-dropped": [
        (C, "    let gap = (lo.x - at).max(at - hi.x) - pad;", "    let gap = (lo.x - at).max(at - hi.x) - pad * T::zero();"),
    ],
    "M6-zone-polar-term-sign": [
        (C, "        at * a + s * (r.powi(2) - at.powi(2)).max(T::zero()).sqrt()", "        at * a.abs() + s * (r.powi(2) - at.powi(2)).max(T::zero()).sqrt()"),
    ],
    "M7-aimed-point-unrotated": [
        (B, """            Self::Aimed(_) => {
                let v = self.vector(Vec3::new(p.x, p.y, p.z));
                Point3::new(v.x, v.y, v.z)
            }""", """            Self::Aimed(_) => p,"""),
    ],
}
ROWS = os.environ.get("ROWS", "reach_split_gate or dual_probe or zone_extent or torus_rect_extent")
def run(name, edits):
    saved = {}
    for f, old, new in edits:
        p = os.path.join(W, f)
        s = saved.setdefault(p, open(p).read())
        cur = open(p).read()
        assert old in cur, (name, old[:60])
        open(p, "w").write(cur.replace(old, new, 1))
    try:
        env = dict(os.environ, CARGO_INCREMENTAL="0", CARGO_TARGET_DIR="/home/user/tgt-mut",
                   PROBE_ONLY=os.environ.get("PROBE_ONLY", "rounded cylinder, partial 2.0"))
        r = subprocess.run(["cargo", "nextest", "run", "-p", "topo", "-p", "sweep", "--run-ignored", "all",
                            "--no-fail-fast", "--no-capture", "-E",
                            "(" + " or ".join(f"test(/{t.strip()}/)" for t in ROWS.split(" or ")) + ") and not test(=reach_split_gate_pose::probe)"],
                           cwd=W, env=env, capture_output=True, text=True)
        out = r.stdout + r.stderr
        open(f"/tmp/claude-0/mut-{name}.log", "w").write(out)
        fails = [l.strip() for l in out.splitlines() if l.strip().startswith("FAIL [")]
        rows = [l for l in out.splitlines() if l.startswith("ROW |")]
        print(f"== {name}: exit {r.returncode}; {len(fails)} failed")
        for l in sorted(set(fails)): print("   ", l)
        bad = [l for l in rows if "meets_split: 0" not in l or "clear_gate: 0" not in l or "wrong_volume: 0" not in l]
        for l in bad[:6]: print("    probe:", l[:260])
    finally:
        for p, s in saved.items(): open(p, "w").write(s)
for n in (sys.argv[1:] or MUTANTS):
    run(n, MUTANTS[n])
