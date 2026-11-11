import sys, subprocess, os
p='crates/geom-brep/src/pcurve_cache/projected.rs'
orig=open(p).read()
E="inside.then(|| (rho_hi - level(near)).max(level(far) - rho_lo))"
assert orig.count(E)==1
M={
 "MC": "None::<S>.filter(|_| inside)",
 "MU": "inside.then(|| rho_hi - rho_hi)",
 "MW16": "inside.then(|| (rho_hi - level(near)).max(level(far) - rho_lo) * S::from_f64(16.0))",
 "MW0": "inside.then(|| rho_hi.max(level(far) - rho_lo))",
 "MWadd": "inside.then(|| (rho_hi - level(near)).max(level(far) - rho_lo) + S::from_f64(1e-12))",
}
name=sys.argv[1]
open(p,'w').write(orig.replace(E,M[name]))
env=dict(os.environ, CARGO_TARGET_DIR="/home/user/r2-target")
try:
    r=subprocess.run(["cargo","nextest","run","-p","geom-brep","--no-fail-fast","-E",
      "test(/limb_2_against_a_horn_reads_the_sheet_up_to_its_cusp/) | test(/a_spindle_bound_is_this_sheets_distance/) | test(=pcurve_cache::envelope_lemma_fuzz::projected::a_part_beside_a_horns_cusp_is_read_at_its_own_height) | test(=pcurve_cache::envelope_lemma_fuzz::projected::a_sheets_reading_is_the_least_that_reads) | test(=closed_form_limb2::limb_2_against_a_sheet_reads_that_sheet_up_to_its_apexes) | test(=closed_form_limb2::past_a_thin_lemons_apex_limb_2_reads_the_sheet_not_its_circle)"],
      env=env, capture_output=True, text=True)
    out=r.stdout+r.stderr
    print(name, "exit", r.returncode)
    for l in out.splitlines():
        if any(k in l for k in ("PASS [","FAIL [","error[","Summary")) : print("  ",l.strip()[:200])
finally:
    open(p,'w').write(orig)
