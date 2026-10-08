import subprocess, sys, os, shutil
S=os.environ['S']; P=S+'/probe/crates/topo/src/boolean/circle_torus.rs'
orig=open(P).read()
gate_old="""    if matches!(
        geom::ring_torus(major_radius, minor_radius, band).map(|d| d.sign),
        Ok(Sign::Positive)
    ) {"""
M={
 'none': [],
 'M1_eta_dropped': [("let eta = r.powi(2) * sigma_sq / (T::from_f64(2.0) * (major_radius - r));","let eta = T::zero();")],
 'M2_tilt_first_order_half_r_sigma': [("off_centre + (radius - r).abs() + tilt","off_centre + (radius - r).abs() + r * sigma_sq.sqrt() * T::from_f64(0.5) + T::zero() * tilt")],
 'M3_gate_dropped': [(gate_old,"    if true {")],
 'M4_centre_to_axis': [("let off_centre = (h0.powi(2) + (offset - major_radius).powi(2)).sqrt();","let off_centre = (h0.powi(2) + offset.powi(2)).sqrt();")],
 'M5_h0_dropped': [("let off_centre = (h0.powi(2) + (offset - major_radius).powi(2)).sqrt();","let off_centre = (offset - major_radius).abs() + T::zero() * h0;")],
 'M6_eta_R_plus_r': [("(T::from_f64(2.0) * (major_radius - r))","(T::from_f64(2.0) * (major_radius + r))")],
 'M7_tilt_halved': [("off_centre + (radius - r).abs() + tilt","off_centre + (radius - r).abs() + tilt * T::from_f64(0.5)")],
}
which=sys.argv[1:] or list(M)
for name in which:
    s=orig
    for a,b in M[name]:
        assert s.count(a)==1,(name,a); s=s.replace(a,b)
    open(P,'w').write(s)
    env=dict(os.environ, CARGO_TARGET_DIR=S+'/probe-target', PROBE_IN=S+'/py/fx.txt', PROBE_OUT=S+f'/py/out_{name}.txt')
    r=subprocess.run(['cargo','nextest','run','-p','topo','--no-fail-fast','--run-ignored','all','-E','test(/circle_torus::/)'],cwd=S+'/probe',env=env,capture_output=True,text=True)
    fails=[l.strip() for l in (r.stdout+r.stderr).splitlines() if l.strip().startswith('FAIL') or 'error[' in l]
    summ=[l for l in (r.stdout+r.stderr).splitlines() if 'Summary' in l or 'tests run' in l]
    print(name, summ[-1:] , *sorted(set(fails))[:20], sep='\n  ', flush=True)
open(P,'w').write(orig)
