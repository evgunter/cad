import sys
m=sys.argv[1]
R='crates/topo/src/boolean/rest.rs'; J='crates/topo/src/boolean/join.rs'
def rep(f,old,new):
    s=open(f).read(); assert old in s,(m,old[:60]); open(f,'w').write(s.replace(old,new,1))
if m=='M1':
    rep(R,"if frames.is_none() && germs.iter().any(|g| !g.used) {","if false && frames.is_none() && germs.iter().any(|g| !g.used) {")
elif m=='M2':
    rep(R,"super::join::germ_separation(frame, &gi.germ, gi.point, gj.point)\n","{ let _ = super::join::germ_separation(frame, &gi.germ, gi.point, gj.point); dist }\n")
elif m=='M3a':
    rep(R,"    if found.len() > 1 {\n","    if found.len() > 1 {\n        return Err(unsupported(RestZipFrontier::ParallelSeamEdges));\n")
elif m=='M3b':
    rep(R,"    if found.len() > 1 {\n","    if found.len() > 1 {\n        return Ok(found.first().copied());\n")
elif m=='M3c':
    rep(R,"    if found.len() > 1 {\n","    if found.len() > 1 {\n        return Ok(found.last().copied());\n")
elif m=='M4':
    rep(J,"let swept = T::pi() + (-sin).atan2(-cos);","let swept = T::pi() + (-(sin + T::zero())).atan2(-cos);")
