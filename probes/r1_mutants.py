#!/usr/bin/env python3
"""Reviewer mutants (reach-dual4135-r1). Each mutant is one textual edit
to the frozen head; the script applies it, runs the named rows, records
pass/fail, and restores the file. Run from the repo root."""
import os, subprocess, sys, shutil
R = 'crates/topo/src/boolean/reduce.rs'
M = 'crates/topo/src/boolean/conic_quadric/mod.rs'
I = 'crates/geom-brep/src/implicit.rs'
MUTANTS = {
 'M1-convexity-restored-for-cone': (R, "geom::Surface::Torus { .. } | geom::Surface::Cone { .. }\n", "geom::Surface::Torus { .. }\n"),
 'M2-no-floor-reread': (M, "Ok(Sign::Positive | Sign::Negative) | Err(_) => CircleRoots::Uncertain,\n            });", "Ok(Sign::Positive | Sign::Negative) | Err(_) => CircleRoots::OnSurface,\n            });"),
 'M3-no-nappe-telloff': (R, "if let Some((apex, axis, half_angle, nappe)) = nappe {", "if let Some((apex, axis, half_angle, nappe)) = nappe.filter(|_| false) {"),
 'M4-no-conic-apex-rung': (M, "Ok(Sign::Zero | Sign::Negative) | Err(_) => return Ok(CircleRoots::AtApex),", "Ok(Sign::Zero | Sign::Negative) | Err(_) => {}"),
 'M5-no-line-apex-rung': (R, 'Sign::Zero | Sign::Negative => return Ok(CircleRoots::AtApex),', 'Sign::Zero | Sign::Negative => {}'),
 'M6-no-conic-root-slack': (M, "            Some(&meter),\n", "            None.or(Some(&meter)).filter(|_| false),\n"),
 'M7-floor-one': (I, "floor: sin_a.min(cos_a) * near / per,", "floor: T::one() + T::zero() * (sin_a.min(cos_a) * near / per),"),
 'M8-height-c2-sign': (I, "(hu.powi(2) - hv.powi(2)) * half,", "(hv.powi(2) - hu.powi(2)) * half,"),
}
def run(cmd):
    p = subprocess.run(cmd, shell=True, capture_output=True, text=True)
    return p.returncode, p.stdout + p.stderr
which = sys.argv[1:] or list(MUTANTS)
env = 'CARGO_INCREMENTAL=0 '
for name in which:
    path, old, new = MUTANTS[name]
    src = open(path).read()
    assert src.count(old) == 1, (name, src.count(old))
    shutil.copy(path, path + '.orig')
    open(path, 'w').write(src.replace(old, new))
    try:
        rc1, out1 = run(env + "cargo nextest run -p topo -p geom-brep --no-fail-fast -E '(test(cone_rows) | test(line_cone_rows) | test(conic_quadric)) - test(r1_)' 2>&1 | grep -E '^\\s+(FAIL|PASS) |Summary|error\\[' ")
        rc2, out2 = run(env + "cargo nextest run -p sweep --no-fail-fast -E 'test(reach_cone_root_lane)' 2>&1 | grep -E '^\\s+(FAIL|PASS) |Summary|error\\[' ")
        fails = [l for l in (out1 + out2).splitlines() if 'FAIL' in l or 'error[' in l]
        print(f'== {name}: topo/geom rc={rc1} sweep rc={rc2}')
        for l in sorted(set(fails)):
            print('   ', l.strip())
        for l in (out1 + out2).splitlines():
            if 'Summary' in l:
                print('   ', l.strip())
        sys.stdout.flush()
    finally:
        shutil.move(path + '.orig', path)
