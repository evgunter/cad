import subprocess, sys, json, os, re
W='/home/user/probe'
S=W+'/crates/topo/src/boolean/sectors.rs'
V=W+'/crates/topo/src/boolean/vtxfac.rs'
env=dict(os.environ, CARGO_TARGET_DIR='/home/user/probe-target')
def sub(path, old, new, count=1):
    return (path, old, new, count)
M = {
 'BASE': [],
 'M1_apart_never': [sub(S, "fn apart<T: Decide>(s: &BoolSector<T>, arc: GreatArc<T>, band: Band) -> bool {\n", "fn apart<T: Decide>(s: &BoolSector<T>, arc: GreatArc<T>, band: Band) -> bool {\n    if true { return false; }\n")],
 'M2_no_apart_p_in_band': [sub(S, "                Err(_) if apart(s, arc, band) => continue,\n", "")],
 'M3_no_apart_crossing': [sub(S, "                    Some(Ok(false)) => continue,\n                    _ if apart(s, arc, band) => continue,\n", "                    Some(Ok(false)) => continue,\n")],
 'M4_no_apart_arc_side': [sub(S, "                    _ if apart(s, arc, band) => {\n                        crosses = false;\n                        break;\n                    }\n", "")],
 'M5_no_past_u': [sub(S, "let past_u = ud == Some(-1) && up == Some(-1);", "let past_u = false && ud == Some(-1) && up == Some(-1);")],
 'M6_no_past_v': [sub(S, "let past_v = vd == Some(1) && vp == Some(1);", "let past_v = false && vd == Some(1) && vp == Some(1);")],
 'M7_no_past_ends': [sub(S, "    past_u || past_v || past_ends()\n", "    let _ = past_ends; past_u || past_v\n")],
 'M8_no_past_d': [sub(S, "        past_d || past_p\n", "        let _ = past_d; past_p\n")],
 'M9_no_past_p': [sub(S, "        past_d || past_p\n", "        let _ = past_p; past_d\n")],
 'M10_unlevered': [sub(S, "            Margin::levered(a.cross(b).dot(n), lever),\n", "            Margin::of(a.cross(b).dot(n) + lever * T::from_f64(0.0)),\n")],
 'M11_refusal_reverted': [sub(V, "    if let Some(unread) = pairs.iter().position(|p| p.read.is_none()) {\n        return Err(refuse(unread, usize::from(unread == 0)));\n    }\n", "")],
 'M12_inside_none_falls_back': [
    sub(V, "    let inside = |i: usize, j: usize| -> Result<bool, BooleanError> {\n        let read = super::sectors::wedge_classes(&pairs[i].sectors, &pairs[j].sectors, band)?\n            .ok_or_else(|| refuse(j, i))?;\n        Ok(!read.rows.is_empty() && read.rows.iter().all(|&(_, c)| held(c, read.met)))\n    };",
          "    let _ = &refuse;\n    let inside = |i: usize, j: usize| -> Result<Option<bool>, BooleanError> {\n        let read = super::sectors::wedge_classes(&pairs[i].sectors, &pairs[j].sectors, band)?;\n        Ok(read.map(|read| !read.rows.is_empty() && read.rows.iter().all(|&(_, c)| held(c, read.met))))\n    };"),
    sub(V, "            if inside(i, j)? {\n                outermost = false;\n            }", "            match inside(i, j)? {\n                None => return Ok(None),\n                Some(true) => outermost = false,\n                Some(false) => {}\n            }")],
 'M13_span_longer_arm': [sub(S, "let gate = Margin::levered(span, arm.min(arc.p_arm));", "let gate = Margin::levered(span, arm.max(arc.p_arm));")],
 'X1_lever_min_reach': [sub(S, "        let lever = least_lever([(la, one), (lb, one)]).unwrap_or(T::from_f64(0.0));\n", "        let lever = la.min(lb) + one * T::from_f64(0.0);\n")],
 'X2_lever_max_reach': [sub(S, "        let lever = least_lever([(la, one), (lb, one)]).unwrap_or(T::from_f64(0.0));\n", "        let lever = la.max(lb) + one * T::from_f64(0.0);\n")],
 'X3_strict_sign_no_band': [sub(S, "            Margin::levered(a.cross(b).dot(n), lever),\n", "            Margin::of(a.cross(b).dot(n) * T::from_f64(1e300) + lever * T::from_f64(0.0)),\n")],
 'X4_one_sided_past_u': [sub(S, "let past_u = ud == Some(-1) && up == Some(-1);", "let past_u = ud == Some(-1);")],
 'X5_one_sided_past_d': [sub(S, "let past_d = ud.is_some() && ud == vd && pd.is_some_and(|k| Some(-k) == ud);", "let past_d = ud.is_some() && ud == vd && pd.is_some();")],
 'X6_apart_skips_bound_keeps_crosses': [sub(S, "                    _ if apart(s, arc, band) => {\n                        crosses = false;\n                        break;\n                    }\n", "                    _ if apart(s, arc, band) => continue,\n")],
 'X7_past_d_wrong_side': [sub(S, "pd.is_some_and(|k| Some(-k) == ud)", "pd.is_some_and(|k| Some(k) == ud)")],
 'X8_inband_counts_as_side': [sub(S, "            Ok(Sign::Zero) | Err(_) => None,\n        }\n    };\n    let (ud, up)", "            Ok(Sign::Zero) => None,\n            Err(_) => match decide(\"bool_cone_within\", Margin::of(a.cross(b).dot(n) * T::from_f64(1e300)), band) { Ok(Sign::Positive) => Some(1), Ok(Sign::Negative) => Some(-1), _ => None },\n        }\n    };\n    let (ud, up)")],
}
def run(cmd, extra):
    e=dict(env, **extra)
    p=subprocess.run(cmd, cwd=W, env=e, capture_output=True, text=True)
    out=p.stdout+p.stderr
    fails=sorted(set(re.findall(r'^\s+FAIL \[.*?\] \(.*?\) (.*)$', out, re.M)))
    return p.returncode, fails, out
names = sys.argv[1:] or list(M)
for name in names:
    orig={}
    try:
        for (path, old, new, c) in M[name]:
            if path not in orig: orig[path]=open(path).read()
            s=open(path).read()
            assert old in s, (name, old[:60])
            open(path,'w').write(s.replace(old,new,c))
        base=['cargo','nextest','run','-p','topo','--lib','--test','all','--no-fail-fast']
        rc1,f1,o1=run(base+['-E','test(/boolean::sectors|boolean::vtxfac|every_edge_a_vertex_read_again_reads_is_classed_against_the_germ/) - test(review_probe)'],{'CAD_FUZZ_SEED':'1','CAD_FUZZ_EFFORT':'3'})
        if 'error[' in o1 or 'could not compile' in o1 or 'error: ' in o1 and not f1:
            res={'name':name,'compile_error':o1[-3000:]}
        else:
            rc2,f2,o2=run(base+['-E','test(the_polygon_cone_reader_never_contradicts_the_exact_oracle)'],{'CAD_FUZZ_SEED':'4','CAD_FUZZ_EFFORT':'3'})
            m=re.findall(r'Summary .*', o1+o2); res={'summary':m,'name':name,'killed':bool(f1 or f2),'fails_seed1':f1,'fails_seed4':f2}
            if name.startswith(('M1_','M5','M6','M7','M8','M9','M10','X')) and not name.startswith(('X6',)):
                rc3,f3,o3=run(['cargo','nextest','run','-p','topo','--lib','-E','test(review_probe_apart)'],{'CAD_FUZZ_SEED':'0x86a44e36ccd3a3f6'})
                res['review_probe']=('killed' if (f3 or rc3) else 'survived')
    finally:
        for path,s in orig.items(): open(path,'w').write(s)
    print(json.dumps(res), flush=True)
    with open('/home/user/rev-logs/mut/results.jsonl','a') as fh: fh.write(json.dumps(res)+'\n')
