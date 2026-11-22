import subprocess, shutil, os, sys
F='/home/user/cad/crates/topo/src/injective.rs'
orig=open(F).read()
SP_OLD="""            .all(|(a, b)| a.get().equals_exactly(b.get()) || a.get().equals_exactly(-b.get()))"""
M={
 'M1_lifts_alone': [("""        || stated(e)
            .zip(stated(o))
            .is_some_and(|(f, g)| one_stated_plane(&f, &g))""","""        || (false && stated(e)
            .zip(stated(o))
            .is_some_and(|(f, g)| one_stated_plane(&f, &g)))""")],
 'M2_w_alone': [("""        && [(m.u(), n.u()), (m.v(), n.v()), (m.w(), n.w())]""","""        && [(m.w(), n.w())]""")],
 'M3_no_stored_conic': [("""        && one_stated_conic(se, so)
    {""","""        && one_stated_conic(se, so) && false
    {""")],
 'M4_signs_not_admitted': [(SP_OLD,"""            .all(|(a, b)| a.get().equals_exactly(b.get()))""")],
 'M5_origin_not_compared': [("""    f.origin().equals_exactly(g.origin())
        && [""","""    true
        && [""")],
 'M6_u_not_compared': [("""        && [(m.u(), n.u()), (m.v(), n.v()), (m.w(), n.w())]""","""        && [(m.v(), n.v()), (m.w(), n.w())]""")],
 'M7_v_not_compared': [("""        && [(m.u(), n.u()), (m.v(), n.v()), (m.w(), n.w())]""","""        && [(m.u(), n.u()), (m.w(), n.w())]""")],
 'M8_coplanar_always': [("""    lifted(e)
        .zip(lifted(o))
        .is_some_and(|(f, g)| one_plane(f, g))""","""    true || lifted(e)
        .zip(lifted(o))
        .is_some_and(|(f, g)| one_plane(f, g))""")],
 'M9_stated_plane_always': [("""    f.origin().equals_exactly(g.origin())
        && [""","""    true || f.origin().equals_exactly(g.origin())
        && [""")],
 'M10_w_not_compared': [("""        && [(m.u(), n.u()), (m.v(), n.v()), (m.w(), n.w())]""","""        && [(m.u(), n.u()), (m.v(), n.v())]""")],
}
env=dict(os.environ, CARGO_TARGET_DIR='/home/user/review-target')
which=sys.argv[1:] or list(M)
try:
  for name in which:
    s=orig
    for a,b in M[name]:
        assert s.count(a)==1,(name,a)
        s=s.replace(a,b)
    open(F,'w').write(s)
    log=open(f'/home/user/review-target/mut/{name}.log','w')
    r1=subprocess.run(['cargo','nextest','run','--no-fail-fast','-p','topo','--lib','-E','test(/^injective::/)'],cwd='/home/user/cad',env=env,stdout=log,stderr=subprocess.STDOUT)
    r2=subprocess.run(['cargo','nextest','run','--no-fail-fast','-p','sweep','--test','all','-E','test(/self_intersection_proves_injectivity::(a_ball_in_a_turned_frame|two_turned_balls|a_cylinder_cylinder_tee|a_two_ball_union)/)'],cwd='/home/user/cad',env=env,stdout=log,stderr=subprocess.STDOUT)
    log.close()
    print(name, r1.returncode, r2.returncode, flush=True)
finally:
  open(F,'w').write(orig)
print('DONE',flush=True)
