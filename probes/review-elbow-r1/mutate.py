import sys
p = sys.argv[1]
s = open(p).read()
m = lambda name: f'std::env::var("R1_MUT").as_deref() == Ok("{name}")'
old1 = """                crate::rim_wedge::Departure::Opposite => Ok(()),
                crate::rim_wedge::Departure::Same"""
new1 = f"""                crate::rim_wedge::Departure::Opposite if {m('SEAM_UNROUTED')} => Err(Unrouted),
                crate::rim_wedge::Departure::Opposite => Ok(()),
                crate::rim_wedge::Departure::Same if {m('SAME_GLUE')} => Ok(()),
                crate::rim_wedge::Departure::Same"""
old2 = """        crate::rim_wedge::RimRouting::Transverse => Err(TangencyFailure::NotTangent),"""
new2 = f"""        crate::rim_wedge::RimRouting::Transverse if {m('T_GLUE')} => Ok(()),
        crate::rim_wedge::RimRouting::Transverse => Err(TangencyFailure::NotTangent),"""
old3 = """    match routing {
        crate::rim_wedge::RimRouting::Seam => {"""
new3 = f"""    if {m('NO_ROUTE')} {{ return Ok(()); }}
    match routing {{
        crate::rim_wedge::RimRouting::Seam => {{"""
for o, n in [(old1, new1), (old2, new2), (old3, new3)]:
    assert s.count(o) == 1, o
    s = s.replace(o, n)
open(p, 'w').write(s)
