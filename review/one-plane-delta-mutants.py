import subprocess, os, sys
F='/home/user/cad/crates/topo/src/injective.rs'
orig=open(F).read()
AX="""        && [(m.u(), n.u()), (m.v(), n.v()), (m.w(), n.w())]"""
M={
 'M6_u': [(AX,"""        && [(m.v(), n.v()), (m.w(), n.w())]""")],
 'M7_v': [(AX,"""        && [(m.u(), n.u()), (m.w(), n.w())]""")],
 'M10_w': [(AX,"""        && [(m.u(), n.u()), (m.v(), n.v())]""")],
 'M2_main_origin_w': [(AX,"""        && [(m.w(), n.w())]""")],
 'M5_origin': [("""    f.origin().equals_exactly(g.origin())
        && [""","""    true
        && [""")],
 'Mr_radius': [("""one_stated_plane(&a.frame, &b.frame) && a.radius() == b.radius()""","""one_stated_plane(&a.frame, &b.frame)""")],
 'Ms_stored_alone': [("""        && one_stated_conic(se, so)
    {
        return true;""","""    {
        return one_stated_conic(se, so);""")],
 'M3_no_stored': [("""        && one_stated_conic(se, so)
    {""","""        && one_stated_conic(se, so) && false
    {""")],
 'M4_signs': [("""            .all(|(a, b)| a.get().equals_exactly(b.get()) || a.get().equals_exactly(-b.get()))""","""            .all(|(a, b)| a.get().equals_exactly(b.get()))""")],
}
env=dict(os.environ, CARGO_TARGET_DIR='/home/user/review-target')
try:
  for name in (sys.argv[1:] or list(M)):
    s=orig
    for a,b in M[name]:
        assert s.count(a)==1,(name,a); s=s.replace(a,b)
    open(F,'w').write(s)
    with open(f'/home/user/review-target/mut/d_{name}.log','w') as log:
        r=subprocess.run(['cargo','nextest','run','--no-fail-fast','-p','topo','--lib','-E','test(/^injective::/)'],cwd='/home/user/cad',env=env,stdout=log,stderr=subprocess.STDOUT)
    fails=[l.strip() for l in open(f'/home/user/review-target/mut/d_{name}.log') if l.strip().startswith('FAIL')]
    print(name, r.returncode, sorted(set(f.split(') ')[-1] for f in fails)), flush=True)
finally:
  open(F,'w').write(orig)
print('DONE',flush=True)
