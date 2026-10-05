#!/usr/bin/env python3
"""Instrument the conic x quadric dispatch in reduce.rs so every door
call appends `carrier | surface | t0 | t1 | answer` (Debug, exact f64
round-trip) to $REVIEW_DOOR_LOG. `door_log.py head|main <tree>`."""
import re, sys
which, tree = sys.argv[1], sys.argv[2]
p = f"{tree}/crates/topo/src/boolean/reduce.rs"
s = open(p).read()
helper = '''
/// Review instrumentation (reach-full4042): one line per conic door call.
fn review_door_log<C: core::fmt::Debug, S: core::fmt::Debug, T: core::fmt::Debug, R: core::fmt::Debug>(
    c: &C, s: &S, t0: &T, t1: &T, r: &R,
) {
    use std::io::Write as _;
    if let Ok(path) = std::env::var("REVIEW_DOOR_LOG") {
        let line = format!("{c:?} | {s:?} | {t0:?} | {t1:?} | {r:?}\\n");
        if let Ok(mut f) = std::fs::OpenOptions::new().create(true).append(true).open(path) {
            let _ = f.write_all(line.as_bytes());
        }
    }
}
'''
s = s.replace("fn wall_crossing<T: Decide>(", helper + "\nfn wall_crossing<T: Decide>(", 1)
def wrap(call):
    return ("{ let review_r = " + call + "; review_door_log(carrier, surface, &t0, &t1, &review_r); review_r? }")
if which == "head":
    old = "super::conic_quadric::conic_quadric_roots(carrier, t0, t1, surface, band)?"
    assert s.count(old) == 1
    s = s.replace(old, wrap(old[:-1]))
else:
    for old in ["super::circle_sphere::circle_sphere_roots(carrier, t0, t1, surface, band)?",
                "super::circle_cylinder::circle_cylinder_roots(carrier, t0, t1, surface, band)?"]:
        assert s.count(old) == 1, old
        s = s.replace(old, wrap(old[:-1]))
    # The ellipse door also answers tori on main; log sphere/wall only.
    old = "super::ellipse_roots::ellipse_roots(carrier, t0, t1, surface, band)?"
    assert s.count(old) == 1
    s = s.replace(old, "{ let review_r = super::ellipse_roots::ellipse_roots(carrier, t0, t1, surface, band); "
                  "if !matches!(surface, geom::Surface::Torus { .. }) { review_door_log(carrier, surface, &t0, &t1, &review_r); } review_r? }")
open(p, "w").write(s)
