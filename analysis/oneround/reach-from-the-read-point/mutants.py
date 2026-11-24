import subprocess, sys, os, json
os.chdir('/home/user/cad')
env=dict(os.environ, CARGO_TARGET_DIR='/home/user/tgt-review')
TOPO="test(graze_reach) | test(spline_wall_rows) | test(reach_review_probes) | test(a_trace_reads) | test(crossing_reach_rows)"
GB="test(line_cone_rows) | test(lever_rows)"
P='crates/geom-brep/src/curve_face/pose.rs'
O='crates/topo/src/offset_derive.rs'
S='crates/topo/src/ring_path/spline_wall.rs'
TR='crates/topo/src/boolean/trace.rs'
E='crates/geom-brep/src/extent.rs'
SRB="geom_brep::span_reach_bounds(edge, span, at)"
M={
 'M1a': (P,"Margin::levered(a2 / dd, run)","Margin::levered(a2 / dd, crate::edge_extent_inf(&geom::Curve3::Line(line), lo, hi))"),
 'M1b': (P,"crate::span_reach_bounds(&geom::Curve3::Line(line), (lo, hi), line.at(foot))","crate::span_reach_bounds(&geom::Curve3::Line(line), (lo, hi), line.at(lo))"),
 'M1c': (P,"Margin::levered(a2 / dd, run)","Margin::levered(a2 / dd, geom_core::Lever::unsourced(crate::edge_extent_inf(&geom::Curve3::Line(line), lo, hi).length() / T::from_f64(2.0)))"),
 'M2a': (O,SRB,"geom_brep::LeverBounds::Exact(geom_brep::edge_extent_inf(edge, span.0, span.1))"),
 'M2b': (O,SRB,"geom_brep::span_reach_bounds(edge, span, edge.eval(span.0))"),
 'M2c': (O,SRB,"geom_brep::LeverBounds::Exact(geom_brep::span_reach_bounds(edge, span, at).above())"),
 'M2d': (O,SRB,"geom_brep::LeverBounds::Exact(geom_brep::span_reach_bounds(edge, span, at).below())"),
 'M2f_sup': (O,SRB,"geom_brep::LeverBounds::Exact(geom_brep::edge_extent_sup(edge, span.0, span.1))"),
 'M2g_seedpt': (O,"if !matches!(graze_at(sine.abs(), p), Ok(Sign::Positive))","if !matches!(graze_at(sine.abs(), carrier.eval(seed)), Ok(Sign::Positive))"),
 'M2h_farend': (O,SRB,"{ let (a, b) = (edge.eval(span.0), edge.eval(span.1)); geom_brep::span_reach_bounds(edge, span, if (a - at).norm() > (b - at).norm() { a } else { b }) }"),
 'M3a': (S,"geom_brep::within_carrier_bend(reach, &carrier, stop)","geom_brep::within_carrier_bend({ let _ = reach; self.travel().0 }, &carrier, stop)"),
 'M3d': (S,"crate::cover::piece_reach_bounds(&carrier, self.span, ends, carrier.eval(stop)).below();","crate::cover::piece_reach_bounds(&carrier, self.span, ends, carrier.eval(stop)).above();"),
 'M4b': (TR,"let arm = |point| crate::cover::piece_reach_bounds(&carrier, params, ends, point).below();","let arm = |_point: Point3<T>| crate::cover::piece_reach_bounds(&carrier, params, ends, ends.0).below();"),
 'M4d_circle_r': (TR,"crate::cover::piece_reach_bounds(&carrier, (T::zero(), tau), (start, start), point)\n                    .below();","geom_core::Lever::unsourced(radius);"),
 'M5a_swap': (E,"    edge_reach_bounds(carrier, (t0, t1), pivot, above)\n}","    match edge_reach_bounds(carrier, (t0, t1), pivot, above) { LeverBounds::Between { below, above } => LeverBounds::Between { below: above, above: below }, x => x }\n}"),
 'M5b_belowonly': (E,"    edge_reach_bounds(carrier, (t0, t1), pivot, above)\n}","    LeverBounds::Exact(edge_reach_bounds(carrier, (t0, t1), pivot, above).below())\n}"),
 'M5c_aboveonly': (E,"    edge_reach_bounds(carrier, (t0, t1), pivot, above)\n}","    LeverBounds::Exact(edge_reach_bounds(carrier, (t0, t1), pivot, above).above())\n}"),
}
M.update({
 'M2i_final_above': (O,"    if graze(sine.abs(), p)? {\n        return Ok(Err(CornerVerdict::Graze { sine }));\n    }\n    Ok(Ok(t))","    if matches!(crate::validate::decide(\"offset_corner_graze\", Margin::levered(sine.abs(), geom_brep::span_reach_bounds(edge, span, p).above()), band)?, Sign::Zero) {\n        return Ok(Err(CornerVerdict::Graze { sine }));\n    }\n    Ok(Ok(t))"),
 'M2j_final_below': (O,"    if graze(sine.abs(), p)? {\n        return Ok(Err(CornerVerdict::Graze { sine }));\n    }\n    Ok(Ok(t))","    if matches!(crate::validate::decide(\"offset_corner_graze\", Margin::levered(sine.abs(), geom_brep::span_reach_bounds(edge, span, p).below()), band)?, Sign::Zero) {\n        return Ok(Err(CornerVerdict::Graze { sine }));\n    }\n    Ok(Ok(t))"),
 'M2k_line_seedpt': (O,"        let at = carrier.eval(if geom_core::is_finite_length(t) {","        let at = carrier.eval(if false && geom_core::is_finite_length(t) {"),
 'M3c_from_start': (S,"crate::cover::piece_reach_bounds(&carrier, self.span, ends, carrier.eval(stop)).below();","crate::cover::piece_reach_bounds(&carrier, self.span, ends, carrier.eval(if self.forward { self.span.0 } else { self.span.1 })).below();"),
 'M4c_arc_old': (TR,"crate::cover::piece_reach_bounds(&carrier, (T::zero(), run), ends, point)\n                        .below();","geom_brep::ExtentBall::of_covered(geom::cover::Cover::arc(&frame, (radius, radius), geom::cover::ParamWindow::new(T::zero(), run)).as_ref()).lever();"),
})
names=sys.argv[1:] or list(M)
for n in names:
    f,old,new=M[n]
    s=open(f).read()
    assert s.count(old)==1,(n,s.count(old))
    open(f,'w').write(s.replace(old,new))
    try:
        filt = GB if f in (P,E) else TOPO
        pk = ['-p','geom-brep'] if f==P else (['-p','geom-brep','-p','topo'] if f==E else ['-p','topo'])
        if f==E: filt = GB+" | "+TOPO
        r=subprocess.run(['cargo','nextest','run',*pk,'--no-fail-fast','-E',filt],env=env,capture_output=True,text=True,timeout=1500)
        out=r.stdout+r.stderr
        fails=sorted(set(l.split('] ')[-1].strip() for l in out.splitlines() if l.strip().startswith('FAIL [')))
        summ=[l.strip() for l in out.splitlines() if 'tests run' in l]
        if 'error[' in out: summ=['COMPILE ERROR: '+next(l for l in out.splitlines() if 'error[' in l)]
        print(n, summ, fails, flush=True)
        open('/home/user/muts/'+n+'.log','w').write(out)
    finally:
        open(f,'w').write(s)
