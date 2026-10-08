import subprocess, sys, os
P = sys.argv[1]
F = P + '/crates/geom-brep/src/extent.rs'
orig = open(F).read()
M = {
 'bulge_from_quarter_start': ('let bulge = (at(s0 + quarter * T::from_f64(0.5)) - c) * shift;', 'let bulge = (at(s0) - c) * shift;'),
 'crest_off_by_pi': ('let away = (-(r * u.dot(w)), -(r * v.dot(w)));', 'let away = (r * u.dot(w), r * v.dot(w));'),
 'cap_dropped': ('        .min(w.norm() + r.abs())\n}', '\n}'),
 'ends_only': ('ends.max(crest_in_span(away, (t0, t1)).select_le_zero(crest, T::zero()))', 'ends.max(T::zero() * crest_in_span(away, (t0, t1)).select_le_zero(crest, T::zero()))'),
 'abs_r_dropped_in_crest': ('let crest = (h.powi(2) + (r.abs() + rho).powi(2)).sqrt();', 'let crest = (h.powi(2) + (r + rho).powi(2)).sqrt();'),
 'r_dropped_in_away': ('let away = (-(r * u.dot(w)), -(r * v.dot(w)));', 'let away = (-(u.dot(w)), -(v.dot(w)));'),
 'halfturn_cap_dropped': ('            .min(T::from_f64(core::f64::consts::PI))\n            .cos();\n    edge - (pa', '            .cos();\n    edge - (pa'),
 'rho_is_w_norm': ('let rho = (w - k * h).norm();', 'let rho = w.norm();'),
 'ellipse_turn_clamp_dropped': ('let quarter = ((t1 - t0).max(-turn).min(turn)) * T::from_f64(0.25);', 'let quarter = (t1 - t0 + T::zero() * turn) * T::from_f64(0.25);'),
 'crest_in_span_uses_t0_not_mid': ('let (sm, cm) = ((t0 + t1) * half).sin_cos();', 'let (sm, cm) = (t0 + T::zero() * half).sin_cos();'),
}
which = sys.argv[2:] or list(M)
filt = "test(a_conic_arcs_reach_is_its_farthest_point_over_the_span) | test(span_reach_hunt) | test(a_faces_turn_lever) | test(range_along) | test(conic_arc)"
env = dict(os.environ, CARGO_TARGET_DIR='/home/user/r2p-target')
for name in which:
    a, b = M[name]
    assert orig.count(a) == 1, name
    open(F, 'w').write(orig.replace(a, b))
    try:
        r = subprocess.run(['cargo', 'nextest', 'run', '-p', 'geom-brep', '--no-fail-fast', '-E', filt], cwd=P, env=env, capture_output=True, text=True)
        out = r.stdout + r.stderr
        fails = [l.strip() for l in out.splitlines() if l.strip().startswith('FAIL [')]
        summ = [l.strip() for l in out.splitlines() if 'Summary' in l or 'error[' in l][:2]
        print(name, 'KILLED' if r.returncode else 'SURVIVED', summ, sorted(set(f.split(')')[-1].strip() for f in fails)), flush=True)
    finally:
        open(F, 'w').write(orig)
