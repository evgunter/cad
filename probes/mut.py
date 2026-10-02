import sys,subprocess,os
P='crates/topo/src/splitting/classify.rs'
orig=open(P).read()
M={
 'M1_gate_always_pass': ("                if !face_clears(face_key) {\n                    return Err(SplitReduceError::CurvedBooleanUnsupported {","                if false && !face_clears(face_key) {\n                    return Err(SplitReduceError::CurvedBooleanUnsupported {"),
 'M2_box_whole_ball': ("    let ball = crate::census::face_reach(body, face, band)?;\n","    let ball = crate::census::face_reach(body, face, band)?;\n    if true { return Some(ball); }\n"),
 'M3_pad_zero': ("    let pad = T::from_f64(crate::boolean::boxes::sweep_pad(band));","    let pad = T::from_f64(0.0 * crate::boolean::boxes::sweep_pad(band));"),
 'M4_edge_face_box_dropped': ("                        || bounding.into_iter().flatten().any(face_clears);","                        || (false && bounding.into_iter().flatten().any(face_clears));"),
 'M5_edge_always_pass': ("                    if !clears {\n                        return Err(SplitReduceError::CurvedEdgeUnsupported","                    if false && !clears {\n                        return Err(SplitReduceError::CurvedEdgeUnsupported"),
 'M6_zone_h_swapped': ("        lo: trim.lat_hi.map_or(T::zero() - *radius, |(h, _)| h),\n        hi: trim.lat_lo.map_or(*radius, |(h, _)| h),","        lo: trim.lat_hi.map_or(T::zero() - *radius, |(h, _)| h),\n        hi: trim.lat_hi.map_or(*radius, |(h, _)| h),"),
 'M7_zone_hi_drops_pole': ("        hi: trim.lat_lo.map_or(*radius, |(h, _)| h),","        hi: trim.lat_lo.map_or(T::zero(), |(h, _)| h),"),
 'M8_insert_crossings_same_side_refuses': ("                    (PlaneSide::Above, PlaneSide::Above) | (PlaneSide::Below, PlaneSide::Below)\n                ) {\n                    continue;","                    (PlaneSide::Above, PlaneSide::Above) | (PlaneSide::Below, PlaneSide::Below)\n                ) && false {\n                    continue;"),
 'M9_one_side_any': ("    sides[0] != sides[1]\n","    sides[0] || sides[1]\n"),
}
filt="test(/split_gate_per_face|reach_split_gate_per_face|r1_reach3843|m5_pr7_split_meter|split/)"
for name,(a,b) in M.items():
    if sys.argv[1:] and name not in sys.argv[1:]: continue
    assert orig.count(a)==1,(name,orig.count(a))
    open(P,'w').write(orig.replace(a,b))
    try:
        r=subprocess.run(['cargo','nextest','run','-p','topo','-p','sweep','--profile','ci','--no-fail-fast','-E',filt],capture_output=True,text=True,env=dict(os.environ,CARGO_INCREMENTAL='0'))
        out=r.stdout+r.stderr
        fails=sorted(set(l.split(']')[-1].strip() for l in out.splitlines() if l.strip().startswith('FAIL [')))
        summ=[l for l in out.splitlines() if 'Summary' in l or 'error[' in l]
        print(name, summ[-1:] , fails, flush=True)
    finally:
        open(P,'w').write(orig)
