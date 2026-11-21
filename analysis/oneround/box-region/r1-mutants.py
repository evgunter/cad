#!/usr/bin/env python3
"""Reviewer r1's planted mutants for PR 7225: each is one textual edit
of the frozen head, applied in a scratch worktree, run over the
session-box rows, and reverted."""
import subprocess, sys, os
ROOT = sys.argv[1]
SB = "crates/topo/src/session_box.rs"
CV = "crates/topo/src/cover.rs"
MUTANTS = {
 # Stage 2 skipped: past the carrier-and-boundary bound, go straight to the points.
 "stage2_skipped": (SB,
   "    for (face, _) in body.faces() {\n        cover = match (cover, crate::cover::face_cover(body, face, band)) {",
   "    for (face, _) in body.faces().take(0) {\n        cover = match (cover, crate::cover::face_cover(body, face, band)) {"),
 # Stage 2 read as stage 1's verdict: any body past stage 1 refuses as a limit.
 "stage2_never_admits": (SB,
   "            None => return Ok(()),\n            Some(margin) => ValidationError::SessionBoxUnshown { margin },",
   "            None => ValidationError::SessionBoxUnshown { margin: geom_core::MarginDiag::value(-1.0) },\n            Some(margin) => ValidationError::SessionBoxUnshown { margin },"),
 # A Zero (in band) read as past, at every box decision.
 "zero_read_as_past": (SB,
   "            sign: Sign::Negative,\n            margin,\n        }) => Some(margin),",
   "            sign: Sign::Negative | Sign::Zero,\n            margin,\n        }) => Some(margin),"),
 # The uncovered face dropped at the gate and tier 3'.
 "uncovered_dropped_at_gate": (SB,
   "        Err(uncovered @ ValidationError::SessionBoxUncovered { .. }) => Ok(Some(uncovered)),",
   "        Err(ValidationError::SessionBoxUncovered { .. }) => Ok(None),"),
 # The cone arm of stage 1 without its apex.
 "cone_apex_dropped": (CV,
   "            Some(at_apex) => and_trim(carrier, boundary().map(|b| b.hull(at_apex).convex())),",
   "            Some(_at_apex) => and_trim(carrier, boundary().map(|b| b.convex())),"),
 # The torus's along-axis compass points dropped.
 "torus_compass_dropped": (CV,
   "                    for (cos, sin) in compass {",
   "                    for (cos, sin) in compass.into_iter().take(0) {"),
 # The torus extreme with the minor term turned the wrong way.
 "torus_extreme_wrong": (CV,
   "                    let reach = off * (big / off.norm()) + e * small;",
   "                    let reach = off * (big / off.norm()) - e * small;"),
 # Stage 3 reads no edge points (vertices and face points only).
 "stage3_no_edge_points": (SB,
   "        .fold(points, T::max);",
   "        .fold(points_only(body, |p| p), T::max);"),
 # Author's: the placed face read reads its carrier unmapped.
 "placed_carrier_unmapped": (SB,
   "        |surface| crate::transform::surface_at(motion, surface).map(Some),",
   "        |_surface| Ok(None),"),
 # Placement filter accepts every candidate (a candidate not on the face kept).
 "placement_keeps_all": (CV,
   "                    _ => Err(()),\n                }\n            })\n            .is_some()",
   "                    _ => Ok(()),\n                }\n            })\n            .is_some()"),
}
FILTER = ("test(session_box) | test(/review_box_region_r1_probes::probe_(c2|spindle)/) | "
          "test(a_copy_whose_bulge_alone_is_past_the_box_refuses_at_its_placement) | "
          "test(/tcost_k3_certificate::a_(multi_solid_certificate_split|refusing_arm)/)")
env = dict(os.environ, CARGO_TARGET_DIR="/home/user/tgt-mut")
names = sys.argv[2:] or list(MUTANTS)
for name in names:
    path, old, new = MUTANTS[name]
    full = os.path.join(ROOT, path)
    src = open(full).read()
    assert src.count(old) == 1, (name, src.count(old))
    open(full, "w").write(src.replace(old, new))
    try:
        r = subprocess.run(["cargo", "nextest", "run", "--no-fail-fast", "-p", "topo", "-p", "sweep",
                            "-p", "editor-core", "-E", FILTER, "--run-ignored", "all"],
                           cwd=ROOT, env=env, capture_output=True, text=True)
        out = r.stdout + r.stderr
        fails = sorted({l.split(") ")[-1].strip() for l in out.splitlines() if l.strip().startswith("FAIL [")})
        summ = [l.strip() for l in out.splitlines() if l.strip().startswith("Summary")]
        print(f"== {name}: {summ[-1] if summ else 'NO SUMMARY (build failed?)'}")
        if not summ:
            print("\n".join(out.splitlines()[-30:]))
        for f in fails:
            print("   RED", f)
        sys.stdout.flush()
    finally:
        open(full, "w").write(src)
