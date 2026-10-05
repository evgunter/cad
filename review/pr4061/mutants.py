import sys
p='/tmp/claude-0/wt-mu5/crates/topo/src/boolean/insert.rs'
orig=open(p+'.orig').read()
M={
 'heldorigin0':[("walks_before(secs, lo.0, p, q, band)","walks_before(secs, 0, p, q, band)")],
 'precedesorigin':[("walks_before(sectors, first, p, q, band)","walks_before(sectors, p.0, p, q, band)")],
 'arun':[("let a_run = walk_run(n, a_pos[i0], a_pos[i1]);","let a_run = walk_run(n, a_pos[i0], a_pos[i1]).or(Some(false));")],

 'none':[],
 'tieflip':[("        return walks_after(&sectors[p.0], p.1, q.1, band);\n    }\n    let n = sectors.len();\n    let rel = |e: usize| (e + n - origin) % n;",
             "        return Ok(walks_after(&sectors[p.0], p.1, q.1, band)?.map(|b| !b));\n    }\n    let n = sectors.len();\n    let rel = |e: usize| (e + n - origin) % n;")],
 'noorigin':[("    let rel = |e: usize| (e + n - origin) % n;\n    Ok(Some(rel(p.0) < rel(q.0)))",
              "    let rel = |e: usize| (e + n - 0 * origin) % n;\n    Ok(Some(rel(p.0) < rel(q.0)))")],
 'runflip':[("run_order(n, p0, p1).unwrap_or(Some(p1 < p0))","run_order(n, p0, p1).unwrap_or(Some(p0 < p1))")],
 'runfwd':[("run_order(n, p0, p1).unwrap_or(Some(p1 < p0))","run_order(n, p0, p1).unwrap_or(Some(false))")],
 'two_fwd':[("run_order(n, p0, p1).unwrap_or(Some(p1 < p0))","run_order(n, p0, p1).map(|o| o.or(Some(false))).unwrap_or(Some(p1 < p0))")],
}
s=orig
for a,b in M[sys.argv[1]]:
    assert s.count(a)==1,(sys.argv[1],a); s=s.replace(a,b)
open(p,'w').write(s)
