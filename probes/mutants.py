import sys
m=sys.argv[1]
def sub(p,a,b,count=1):
    s=open(p).read(); assert s.count(a)>=count, (p,a,s.count(a)); s=s.replace(a,b); open(p,"w").write(s)
B="crates/topo/src/boolean/join.rs"; SJ="crates/topo/src/splitting/join.rs"; C="crates/topo/src/chord_join.rs"
if m=="M1": sub(B,"    let escalate = |diag| BooleanError::coincidence(Coincide::Join, DeclarationRead::Moot, diag);\n    for end in [p_c, p_e] {","    if true { return Ok(false); }\n    let escalate = |diag| BooleanError::coincidence(Coincide::Join, DeclarationRead::Moot, diag);\n    for end in [p_c, p_e] {")
if m=="M2": sub(B,".map(|g| g.dir)",".map(|g| -g.dir)")
if m=="M3":
    sub(SJ,"h1: split_leave(&red.body, red.plane.normal, &st.above_set, end)?,","h1: -split_leave(&red.body, red.plane.normal, &st.above_set, end)?,")
    sub(SJ,"h2: split_leave(&red.body, red.plane.normal, &st.above_set, half)?,","h2: -split_leave(&red.body, red.plane.normal, &st.above_set, half)?,")
if m=="M4": sub(C,"let ccw = arc_leaving(face, band, &conic, p1, leave)?;","let ccw = { let _ = arc_leaving(face, band, &conic, p1, leave)?; true };",2)
