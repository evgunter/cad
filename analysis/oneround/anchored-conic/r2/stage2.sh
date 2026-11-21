#!/bin/bash
# wait for stage 1
while ! grep -q "R19" /home/user/logs/mutants.log; do sleep 15; done
cd /home/user/cad-mut
git checkout -q . 
export CARGO_TARGET_DIR=/home/user/tgt-mut CARGO_INCREMENTAL=0
# the near-gap counts, printed, on the pristine head
python3 - <<'PY'
p='crates/topo/src/boolean/conic_quadric/in_band_sweep.rs'
s=open(p).read()
old="            let got = certified_near_gap(&near_gap_family(ratio, born));\n"
assert s.count(old)==1
open(p,'w').write(s.replace(old, old+"            eprintln!(\"NEARGAP {ratio:?} {born:?} {got}\");\n"))
PY
cargo nextest run --profile ci --no-fail-fast --no-capture -p topo -E 'test(the_near_gap_families_certify_by_their_stored_form)' > /home/user/logs/neargap.log 2>&1
git checkout -q .
python3 - <<'PY' > /home/user/logs/mutants2.log 2>&1
import subprocess, os, sys
sys.path.insert(0, '/home/user/cad/analysis/oneround/anchored-conic/r2')
src = open('/home/user/cad/analysis/oneround/anchored-conic/r2/mutants.py').read().split("env = dict(")[0]
exec(src)
env = dict(os.environ)
WIDE = [["-p", "topo", "-E", "test(cone) | test(seam) | test(chord) | test(section) | test(ellipse)"],
        ["-p", "sweep", "--ignore-default-filter", "-E", "test(a_ring_on_a_cone_face) | test(cone_join_lane)"]]
for name in ["R9_azimuth_ignores_phase_sine", "R6_map_scalar_drops_phase", "R10_azimuth_ignores_phase_cosine", "R7_placed_drops_phase", "R5_same_content_ignores_phase", "R8_data_omits_phase", "R3_same_content_ignores_form_ellipse"]:
    path, old, new, _ = M[name]
    s = open(path).read(); open(path, "w").write(s.replace(old, new))
    res = []
    try:
        for sc in WIDE:
            r = subprocess.run(["cargo", "nextest", "run", "--profile", "ci", "--no-fail-fast", *sc], env=env, capture_output=True, text=True)
            out = r.stdout + r.stderr
            if "could not compile" in out: res.append((sc[1], "COMPILE-ERROR")); continue
            fails = sorted({l.split("]")[-1].strip() for l in out.splitlines() if l.lstrip().startswith(("FAIL", "SIGABRT", "TIMEOUT"))})
            summ = [l.strip() for l in out.splitlines() if "tests run:" in l]
            res.append((sc[1], "RED" if fails else "GREEN(survives)", len(fails), fails[:8], summ[-1:]))
    finally:
        open(path, "w").write(s)
    print(f"MUTANT2 {name}: {res}", flush=True)
PY
echo done > /home/user/logs/stage2.done
