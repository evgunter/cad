import subprocess, sys, os, re
F='/home/user/r2-dbg/crates/topo/src/boolean/sectors.rs'
orig=open('/home/user/r2-dbg-orig.rs').read()
env=dict(os.environ, CARGO_TARGET_DIR='/home/user/r2-dbg-target')
FZ='/tmp/claude-0/-home-user-cad/17be02d4-9d3d-5c93-8bb8-ef3f1ac00587/scratchpad/fz'
M={
 'R1_facing_inverted': [("Ok(facing.map_err(escalate)? == Sign::Positive)","Ok(facing.map_err(escalate)? != Sign::Positive)")],
 'R2_crossing_read_at_p': [("Some(d * pn.abs() + p * toward)","Some(p)")],
 'R3_in_sector_at_sector_arm': [("in_sector(s, d, reach.length(), band)","in_sector(s, d, s.arm, band)"),("in_sector(s, x, reach.length().min(s0.arm), band)","in_sector(s, x, s.arm, band)")],
 'R4_arc_min_to_max': [("lever = lever.min(length / sine);","lever = lever.max(length / sine);")],
 'R5_facing_dropped': [("Ok(facing.map_err(escalate)? == Sign::Positive)","{ let _ = facing; Ok(true) }")],
 'R6_p_on_plane_exempt_undecided': [("(SideCode::On, Some(_)) => Some(p),","(SideCode::On, Some(_)) => continue,")],
 'R7_span_min_to_max': [("Margin::levered(span, arm.min(arc.p_arm))","Margin::levered(span, arm.max(arc.p_arm))")],
 'R8_crossing_sign_flipped': [("let toward = if side == SideCode::Out { -dn } else { dn };","let toward = if side == SideCode::Out { dn } else { -dn };")],
 'R9_both_beside_pass_S': [("(SideCode::On, None) => continue 'reference,","(SideCode::On, None) => continue,")],
 'R10_zero_past_bound_outside': [("if past.map_err(escalate)? == Sign::Negative {","if past.map_err(escalate)? != Sign::Positive {")],
 'R11_crossing_lever_reach_only': [("in_sector(s, x, reach.length().min(s0.arm), band)","in_sector(s, x, reach.length(), band)")],
 'R12_arc_lever_dir_only': [("    for (length, sine) in [\n        (bound_reach.length(), d.cross(arc.p).norm()),\n        (arc.p_arm, d.cross(b).norm()),\n    ] {","    for (length, sine) in [(arc.p_arm, T::from_f64(0.0)); 0] {")],
}
sel=sys.argv[1:] or list(M)
for name in sel:
    s=orig
    for a,b in M[name]:
        assert a in s, (name,a); s=s.replace(a,b)
    open(F,'w').write(s)
    p=subprocess.run(['cargo','nextest','run','-p','topo','--lib','--no-fail-fast','-E','test(/boolean::sectors::tests/) | test(/boolean::vtxfac::tests/)'],cwd='/home/user/r2-dbg',env=env,capture_output=True,text=True)
    fails=sorted(set(re.findall(r'FAIL \[.*?\] \(\d+/\d+\) topo (\S+)',p.stdout+p.stderr)))
    summ=[l for l in (p.stdout+p.stderr).splitlines() if 'Summary' in l or 'error[' in l]
    fz=''
    if len(sys.argv)>1 or not fails:
        e=dict(env, RV_CASES=f'{FZ}/cases_2.txt', RV_OUT=f'{FZ}/mut_{name}.txt', RV_EPS='1e-9')
        subprocess.run(['cargo','nextest','run','-p','topo','--lib','rv_probe_cases'],cwd='/home/user/r2-dbg',env=e,capture_output=True)
        q=subprocess.run(['python3','cmp.py','2',f'mut_{name}.txt','0'],cwd=FZ,capture_output=True,text=True,env=dict(os.environ,RV_EPS='1e-9'))
        fz=q.stdout.splitlines()[0] if q.stdout else q.stderr[-200:]
    print(name, '| unit fails:', [f.split('::')[-1] for f in fails], summ[-1:] , '| fuzz s2:', fz, flush=True)
open(F,'w').write(orig)
