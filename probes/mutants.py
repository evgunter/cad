#!/usr/bin/env python3
"""Reviewer mutant runner (PR 3973 dual): apply one textual mutant, run the
PR's touched rows, record which go red, restore the file. Run from the repo
root at the frozen head."""
import subprocess, sys, os
B = "crates/topo/src/boolean/"
G = "crates/geom-brep/src/implicit.rs"
MUTANTS = {
 "M1 slack meter dropped": (B+"ellipse_roots.rs", "Some(&meter),", "{ let _ = &meter; None },"),
 "M2 M4 read at degree 2": (B+"circle_roots.rs", "let fourth_hi = (1..=n_deg)", "let fourth_hi = (1..=n_deg.min(2))"),
 "M3 Bernstein d1 at 2 not N": (B+"circle_roots.rs", "(d1 - deg * noise, d1 + deg * noise)", "(d1 - two * noise, d1 + two * noise)"),
 "M4 slack ceiling -> floor": (B+"ellipse_roots.rs", "f_per_metre_hi: h.f_per_metre_hi,", "f_per_metre_hi: h.f_per_metre_lo,"),
 "M5 slack drops running error": (B+"circle_roots.rs", ".magnitude();", ".value.abs();"),
 "M6 sin/cos ulp uncharged": (G, "error: ulp * x.abs(),", "error: T::zero() * x.abs(),"),
 "M7 torus noise zero": (B+"ellipse_roots.rs", "noise: rounding_charge(h.terms),", "noise: rounding_charge(h.terms) * T::zero(),"),
 "M8 count cap 2N -> 8N": (B+"circle_roots.rs", "roots.len() > 2 * n_deg", "roots.len() > 8 * n_deg"),
 "M9 reduce: no torus arm": (B+"reduce.rs", "            | geom::Surface::Cylinder { .. }\n            | geom::Surface::Torus { .. } => {\n                super::ellipse_roots", "            | geom::Surface::Cylinder { .. } => {\n                super::ellipse_roots"),
 "M10 circle-torus dropped harmonics uncharged": (B+"circle_torus.rs", "noise: rounding_charge(h.terms) + dropped,", "noise: rounding_charge(h.terms) + dropped * T::zero(),"),
 "M11 ceiling reach w/o speed": (G, "let reach = w.norm() + conic.speed_hi() + major_radius;", "let reach = w.norm() + major_radius;"),
 "M12 slack 1000x lenient": (B+"circle_roots.rs", "match decide(meter.row, Margin::of(arc), band)", "match decide(meter.row, Margin::of(arc * T::from_f64(1e-3)), band)"),
 "M13 trig_product sin diff sign": (G, "core::cmp::Ordering::Less => sin[diff] = sin[diff] + sc - cs,", "core::cmp::Ordering::Less => sin[diff] = sin[diff] + cs - sc,"),
 "M14 F floor 1000x": (G, "f_per_metre_lo: two * minor_radius * k,", "f_per_metre_lo: T::from_f64(1e3) * two * minor_radius * k,"),
 "M15 Bernstein d2/d3 at 2": (B+"circle_roots.rs", "let d2_hi = d2 + deg.powi(2) * noise;\n        let d3_hi = d3 + deg.powi(3) * noise;", "let d2_hi = d2 + T::from_f64(4.0) * noise;\n        let d3_hi = d3 + T::from_f64(8.0) * noise;"),
}
RUNS = [
 ["cargo", "nextest", "run", "-p", "topo", "--lib", "--no-fail-fast", "ellipse_roots", "circle_roots", "circle_torus", "circle_sphere", "-E", "not test(probe_)"],
 ["cargo", "nextest", "run", "-p", "geom-brep", "--lib", "--no-fail-fast", "conic_tests"],
 ["cargo", "nextest", "run", "-p", "sweep", "--test", "all", "--no-fail-fast", "ellipse_torus::"],
]
env = dict(os.environ, CARGO_INCREMENTAL="0")
only = sys.argv[1:]
def main():
  pass
for name, (f, old, new) in (MUTANTS.items() if __name__ == "__main__" else []):
    if only and not any(name.startswith(o) for o in only):
        continue
    src = open(f).read()
    assert src.count(old) == 1, (name, src.count(old))
    open(f, "w").write(src.replace(old, new))
    try:
        red = []
        for cmd in RUNS:
            r = subprocess.run(cmd, capture_output=True, text=True, env=env)
            txt = r.stdout + r.stderr
            if "error[E" in txt or "could not compile" in txt:
                red.append("COMPILE-ERROR " + cmd[3])
            red += [l.split()[-1] for l in txt.splitlines() if l.strip().startswith("FAIL [")]
        print(f"{name}: {'RED ' + ', '.join(sorted(set(red))) if red else 'all green (SURVIVED)'}", flush=True)
    finally:
        open(f, "w").write(src)
