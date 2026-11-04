import sys,subprocess,os
R='crates/geom-brep/src/props/quad/rotational.rs'
P='crates/geom-brep/src/props/quad/projected_edge.rs'
M={
 'M1_junction_gap_no_m':(R,"    delta - pt(m) * crate::props::bracket::tau_bracket()\n","    let _ = m; delta\n"),
 'M2_connector_pad_zero':(R,"let pad = chart.residual(pt(2.0) * gap * width);","let pad = chart.residual(pt(0.0) * gap * width);"),
 'M3_sphere_speed_half':(R,"radius * cross / (p[0].sqr() + p[1].sqr() + p[2].sqr())","pt(0.5) * radius * cross / (p[0].sqr() + p[1].sqr() + p[2].sqr())"),
 'M4_chart_term_pad_zero':(R,"let term = term + Interval::from_bounds(-term_pad, term_pad);","let term = term + Interval::from_bounds(-term_pad * 0.0, term_pad * 0.0);"),
 'M5_latitude_tol_loose':(P,"off.hi() <= 1e-12 * (1.0 + k.abs())","off.hi() <= 1e-2 * (1.0 + k.abs())"),
 'M6_sphere_bracket_pad_zero':(R,"let pad = (f * e.u.cl).mag() * wt.hi();","let pad = 0.0 * (f * e.u.cl).mag() * wt.hi();"),
 'M7_sphere_lever_no_cos':(R,"Self::Sphere { radius, .. } => radius * trig_of(level).0.abs_enclosure(),","Self::Sphere { radius, .. } => { let _ = level; radius }"),
 'M8_face_area_side_off':(R,"        (face, whole),\n","        (sphere, whole),\n"),
 'M9_entry_turn_half':(R,"let eighth = core::f64::consts::TAU / 8.0;","let eighth = core::f64::consts::TAU / 2.0;"),
 'M10_connector_level_v1':(R,"let level = Interval::hull(v1, v0);\n                let forms","let level = { let _ = v0; v1 };\n                let forms"),
 'M11_residual_density_zero':(R,"        [(pt(area) * self.flux_density()).mag(), area]","        [0.0, 0.0 * area]"),
}
env=dict(os.environ,CARGO_TARGET_DIR='/tmp/claude-0/tgt')
for name,(f,a,b) in M.items():
    s=open(f).read()
    if s.count(a)!=1: print(name,'PATTERN COUNT',s.count(a),flush=True); continue
    open(f,'w').write(s.replace(a,b))
    r=subprocess.run(['cargo','nextest','run','-p','geom-brep','--lib','--no-fail-fast','-E','test(rotational)'],capture_output=True,text=True,env=env)
    out=r.stdout+r.stderr
    fails=sorted(set(l.split('geom-brep ')[-1].strip() for l in out.splitlines() if l.strip().startswith('FAIL [')))
    summ=[l for l in out.splitlines() if 'Summary' in l or 'error[' in l][:3]
    print(name,'|',summ,'|',fails,flush=True)
    subprocess.run(['git','checkout','--',f])
print('DONE',flush=True)
