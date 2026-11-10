import subprocess, sys, os, re
W='/home/user/rev6805-mut'
R=W+'/crates/geom-brep/src/props/quad/rotational.rs'
RR=W+'/crates/geom-brep/src/props/rational_rim.rs'
M={
 'M0_none': [],
 'M1_old_per_span': [(R,"(pieces / EDGE_INIT_PIECES).max(1) * EDGE_INIT_PIECES.div_ceil(spans.max(1))","{ let _ = spans; pieces }")],
 'M2_under_read': [(R,"(pieces / EDGE_INIT_PIECES).max(1) * EDGE_INIT_PIECES.div_ceil(spans.max(1))","{ let _ = spans; (pieces / EDGE_INIT_PIECES).max(1) }")],
 'M3_cost_old_count': [(R,"let spread = |weight: usize, spans: usize| weight * spans * span_pieces(pieces, spans);","let spread = |weight: usize, spans: usize| weight * spans * pieces;")],
 'M4_cost_no_pieces': [(R,"let spread = |weight: usize, spans: usize| weight * spans * span_pieces(pieces, spans);","let spread = |weight: usize, spans: usize| weight * spans;")],
 'M5_drop_last_span_many': [(RR,"        e.spans.iter().map(|s| (s.a, s.b, s)),\n        pieces,\n        |s, p, q| piece_form(s, p, q),\n        |f, t| {\n            let (b, db, w) = piece_jets(f, t);\n            g(b, db, w)","        e.spans.iter().take(if e.spans.len() >= 16 { e.spans.len() - 1 } else { e.spans.len() }).map(|s| (s.a, s.b, s)),\n        pieces,\n        |s, p, q| piece_form(s, p, q),\n        |f, t| {\n            let (b, db, w) = piece_jets(f, t);\n            g(b, db, w)")],
 'M6_chart_term_bias_many': [(R,"let flux = (0..3).fold(acc, |a, k| a + term.lever[k] * va[k]);","let bias = if e.span_count() >= 16 { pt(1.001) } else { Interval::one() };\n            let flux = (0..3).fold(acc, |a, k| a + term.lever[k] * va[k] * bias);")],
 'M7_framed_spans_one': [(R,"FramedQ::Net(net) => net.span_count(),\n        FramedQ::Circle","FramedQ::Net(_) => 1,\n        FramedQ::Circle")],
 'M8_edge_terms_skip_spread_cyl': [(R,"let per_span = span_pieces(pieces, framed_spans(&p.carrier));\n            let (terms, [rf, _])","let per_span = (pieces / 16).max(1);\n            let (terms, [rf, _])")],
}
names = sys.argv[1:] or list(M)
for name in names:
    saved={}
    if not M[name]: pass
    for f,old,new in M[name]:
        s=saved.get(f) or open(f).read(); saved.setdefault(f,s)
        cur=open(f).read()
        assert cur.count(old)==1,(name,old[:60])
        open(f,'w').write(cur.replace(old,new))
    env=dict(os.environ,CARGO_TARGET_DIR='/home/user/rev6805-mut-target')
    r=subprocess.run(['cargo','nextest','run','-p','geom-brep','--lib','--no-fail-fast','--ignore-default-filter','-E','test(/rotational/) - test(/rev6805_many_span_sphere/)'],cwd=W,env=env,capture_output=True,text=True)
    out=r.stdout+r.stderr
    fails=sorted(set(re.findall(r'FAIL \[.*?\] \(\s*\d+/\d+\) geom-brep (\S+)',out)))
    summ=[l for l in out.splitlines() if 'Summary' in l or 'error[' in l][:3]
    with open('/home/user/rev6805-mutants/results.txt','a') as fo:
        fo.write(f'== {name}: {summ}\n'+''.join(f'   {x}\n' for x in fails))
    for f,s in saved.items(): open(f,'w').write(s)
    print(name, summ, len(fails), flush=True)
