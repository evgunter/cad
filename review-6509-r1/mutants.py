import sys, subprocess, os
F="crates/geom-brep/src/props/quad/rotational.rs"
muts = {
 "M_chart_term_scale": ("let flux = (0..3).fold(acc, |a, k| a + term.lever[k] * va[k]);", "let flux = (0..3).fold(acc, |a, k| a + term.lever[k] * va[k] * pt(1.00001));"),
 "A_lever_no_abs": ("radius * trig_of(level).0.abs_enclosure()", "radius * trig_of(level).0"),
 "B_gap_floor": ("let m = (mid(delta) / core::f64::consts::TAU).round();", "let m = (mid(delta) / core::f64::consts::TAU).floor();"),
 "D_speed_no_norm": ("radius * cross / (p[0].sqr() + p[1].sqr() + p[2].sqr())", "radius * cross * pt(0.25)"),
 "E_corner_quarter": ("let pad = chart.residual(pt(2.0) * gap * width);", "let pad = chart.residual(pt(0.5) * gap * width);"),
 "I_sph_bracket_pad": ("let pad = (f * e.u.cl).mag() * wt.hi();", "let pad = 0.0 * (f * e.u.cl).mag() * wt.hi();"),
 "J_n_floor_only": ("let n = (mid(x) / mid(sphere)).floor() + 1.0;", "let n = (mid(x) / mid(sphere)).round() + 1.0;"),
 "K_residual_sphere_zero": ("let pad = chart.residual(pt(entry.length) * env);", "let pad = if matches!(chart, RotationalChartQ::Sphere{..}) { [0.0;2] } else { chart.residual(pt(entry.length) * env) };"),
 "L_level_lever_one": ("            Self::Sphere { radius, .. } => radius,\n        }\n    }\n\n    /// The flux density", "            Self::Sphere { .. } => Interval::one(),\n        }\n    }\n\n    /// The flux density"),
}
names = sys.argv[1:] or list(muts)
orig = open(F).read()
env = dict(os.environ, CARGO_TARGET_DIR="/home/user/r1-target")
for n in names:
    a,b = muts[n]
    assert orig.count(a)==1, n
    open(F,"w").write(orig.replace(a,b))
    try:
        r = subprocess.run(["cargo","nextest","run","-p","geom-brep","--lib","--no-fail-fast","-E","test(/rotational/) - test(/r1_probes/)"],capture_output=True,text=True,env=env,timeout=900)
        out = r.stdout+r.stderr
        fails=[l.split()[-1] for l in out.splitlines() if l.strip().startswith("FAIL [")]
        summ=[l for l in out.splitlines() if "Summary" in l or "error[" in l]
        print(n, "RED" if fails else ("BUILD?" if not summ or "error[" in out else "SURVIVES"), sorted(set(fails)), summ[:2], flush=True)
    finally:
        open(F,"w").write(orig)
