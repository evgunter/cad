import sys
p='/home/user/rv-mut/crates/geom-brep/src/intersect.rs'
s=open(p).read()
m=sys.argv[1]
def rep(a,b):
    global s
    assert a in s, (m,a)
    s=s.replace(a,b,1)
coax="    let q = apex - o;\n    let d = (q - b * q.dot(b)).norm();"
lev="        | Surface::Torus { center: anchor, .. } => lever.max(reach.lever_from(anchor)),\n        Surface::Plane { .. }\n        | Surface::Cylinder { .. }"
if m=='M1': rep(coax,"    let q = o - apex;\n    let d = (q - a * q.dot(a)).norm();")
elif m=='MA': rep(coax,"    let f = o + b * (apex - o).dot(b);\n    let q = f - apex;\n    let d = (q - a * q.dot(a)).norm();")
elif m=='M4': rep(lev,"        | Surface::Torus { center: anchor, .. }\n        | Surface::Cylinder { origin: anchor, .. } => lever.max(reach.lever_from(anchor)),\n        Surface::Plane { .. }")
elif m.startswith('L'):
    f=m[1:]
    rep("        (Cone, Cylinder) => cone_cylinder_section(a, b, extent, band).map(drop),\n        (Cylinder, Cone) => cone_cylinder_section(b, a, extent, band).map(drop),",
        f"        (Cone, Cylinder) => cone_cylinder_section(a, b, extent * T::from_f64({f}), band).map(drop),\n        (Cylinder, Cone) => cone_cylinder_section(b, a, extent * T::from_f64({f}), band).map(drop),")
elif m=='MP':  # parallel row levered from the stored origin instead of the apex
    rep('Margin::levered(a.cross(b).norm(), extent),','Margin::levered(a.cross(b).norm(), extent + (o - apex).norm()),')
open(p,'w').write(s)
