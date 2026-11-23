#!/usr/bin/env python3
"""Reviewer r1's mutants re-planted on PR 7225's delta head 1e81f303.
Each is one textual edit, run over the session-box rows, reverted."""
import subprocess, sys, os
ROOT = sys.argv[1]
SB = "crates/topo/src/session_box.rs"
CV = "crates/topo/src/cover.rs"
GR = "crates/geom/src/cover/readings.rs"
MUTANTS = {
 "stage2_never_admits": (SB,
   "            None => return Ok(()),",
   "            None => vec![ValidationError::SessionBoxUnshown { margin: geom_core::MarginDiag::value(-1.0) }],"),
 "uncovered_only_first_named": (SB,
   "            Err(why) => uncovered.push((face, why)),",
   "            Err(why) => { if uncovered.is_empty() { uncovered.push((face, why)) } }"),
 "uncovered_face_skipped": (SB,
   "            Err(why) => uncovered.push((face, why)),",
   "            Err(_) => {}"),
 "cone_one_winding_reads_off": (CV,
   "    !matches!(winding_once, 0 | 2)",
   "    !matches!(winding_once, 0 | 1 | 2)"),
 "placement_keeps_all": (CV,
   "                    _ => Err(()),\n                }\n            })\n            .is_some()\n        })\n        .collect()",
   "                    _ => Ok::<(), ()>(()),\n                }\n            })\n            .is_some()\n        })\n        .collect()"),
 "cylinder_axis_guard_dropped": (GR,
   "Surface::Cylinder { frame, .. } if !is_direction(frame.axis())",
   "Surface::Cylinder { frame, .. } if false && !is_direction(frame.axis())"),
 "cone_three_windings_read_off": (CV,
   "    !matches!(winding_once, 0 | 2)",
   "    !matches!(winding_once, 0 | 2 | 3 | 4)"),
}
FILTER = ("test(session_box) | test(/seamed_slender/) | test(/seamless_frustum/) | test(/tcost_k3_certificate::the_exhausting_prism_whose_net/) | "
          "test(a_copy_whose_bulge_alone_is_past_the_box_refuses_at_its_placement) | test(/^geometric_cube::/) | "
          "test(/^review_m3_pr4::/) | test(/^review_m2_pr3::/) | test(/^interval_body::/) | test(/nurbs_faces_refuse_typed_at_both_doors/) | "
          "test(/section_cert_cone_rows/) | test(/^tier3_tests::/) | test(/cover::tests/)")
env = dict(os.environ, CARGO_TARGET_DIR="/home/user/tgt-r1")
names = sys.argv[2:] or list(MUTANTS)
for name in names:
    path, old, new = MUTANTS[name]
    full = os.path.join(ROOT, path)
    src = open(full).read()
    if src.count(old) != 1:
        print(f"== {name}: PATTERN COUNT {src.count(old)}"); sys.stdout.flush(); continue
    open(full, "w").write(src.replace(old, new))
    try:
        r = subprocess.run(["cargo", "nextest", "run", "--no-fail-fast", "-p", "topo", "-p", "sweep",
                            "-p", "editor-core", "-p", "step-import", "-p", "geom", "-E", FILTER],
                           cwd=ROOT, env=env, capture_output=True, text=True)
        out = r.stdout + r.stderr
        fails = sorted({l.split(") ")[-1].strip() for l in out.splitlines() if l.strip().startswith("FAIL [")})
        summ = [l.strip() for l in out.splitlines() if l.strip().startswith("Summary")]
        print(f"== {name}: {summ[-1] if summ else 'NO SUMMARY'}")
        if not summ:
            print("\n".join(out.splitlines()[-25:]))
        for f in fails:
            print("   RED", f)
        sys.stdout.flush()
    finally:
        open(full, "w").write(src)
