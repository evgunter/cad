import sys,subprocess,os
S="/tmp/claude-0/-home-user-cad/d11ba72d-ed5b-55f9-8ddf-9748fbbf61c4/scratchpad"
f=S+"/mut/crates/topo/src/boolean/join.rs"
orig=open(f).read()
M={
"M1_main_formula":("""        normal.dot(p - from.site) * from.sense,
        normal.dot(from.tangent).abs(),""","""        normal.dot(p - from.center) * from.sense + T::zero() * from.tangent.x,
        radial.norm(),"""),
"M2_order_flipped":("""            Sign::Negative => return Ok(true),
            Sign::Positive => return Ok(false),
            Sign::Zero => {}""","""            Sign::Negative => return Ok(false),
            Sign::Positive => return Ok(true),
            Sign::Zero => {}"""),
"M3_keep_incumbent":("""match decide("bool_join_arc_travel", turned_past(b, c.site), band)""","""match decide("bool_join_arc_travel", Margin::of(T::one() + T::zero() * turned_past(b, c.site).value()), band)"""),
"M4_travel_tie_to_chord":("""match decide("bool_join_arc_travel", turned_past(b, c.site), band)""","""match decide("bool_join_arc_travel", Margin::of(T::zero() * turned_past(b, c.site).value()), band)"""),
}
which=sys.argv[1:]
for name in which:
    a,b=M[name]
    assert orig.count(a)==1,name
    open(f,"w").write(orig.replace(a,b))
    env=dict(os.environ,CARGO_TARGET_DIR=S+"/tgt-mut")
    r=subprocess.run("cargo test -p sweep --test all a_steep_ellipse -- --nocapture 2>&1",shell=True,cwd=S+"/mut",env=env,capture_output=True,text=True)
    lines=[l[:200] for l in r.stdout.splitlines() if "error[" in l or "PROBE" in l or "test result" in l or "error" in l[:8] or "panicked" in l or "refused where" in l or "OK SOUND" in l]
    print("=== ",name); print("\n".join(lines[:40]))
open(f,"w").write(orig)
