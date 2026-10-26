import sys
p='/home/user/rvm/crates/topo/src/props.rs'
M={
 'J_circles': ("geom::Curve3::Line { .. } | geom::Curve3::Circle { .. } | geom::Curve3::Ellipse { .. } => {\n            false","geom::Curve3::Circle { .. } => true,\n        geom::Curve3::Line { .. } | geom::Curve3::Ellipse { .. } => {\n            false"),
 'J_lines': ("geom::Curve3::Line { .. } | geom::Curve3::Circle { .. } | geom::Curve3::Ellipse { .. } => {\n            false","geom::Curve3::Line { .. } => true,\n        geom::Curve3::Circle { .. } | geom::Curve3::Ellipse { .. } => {\n            false"),
 'P_last': ("        .copied()\n        .find(|&f| !matches!(resolve_face(body, f).1, Surface::Plane { .. }));","        .copied()\n        .rev()\n        .find(|&f| !matches!(resolve_face(body, f).1, Surface::Plane { .. }));"),
 'E_nosquare': ("(p - x).dot(n).powi(2))","(p - x).dot(n).abs())"),
 'G_8': ("const CARRY_REACH: f64 = 16.0;","const CARRY_REACH: f64 = 8.0;"),
 'G_32': ("const CARRY_REACH: f64 = 16.0;","const CARRY_REACH: f64 = 32.0;"),
 'Near_all': ("            if plane.is_none() || !curved {","            if true {"),
 'F': ("    for faces in quadrature.into_iter().skip(1) {","    for faces in quadrature.into_iter().skip(0) {"),
}
s=open(p).read()
if sys.argv[1]=='list': print(' '.join(M)); sys.exit()
o,n=M[sys.argv[1]]; assert s.count(o)==1,(sys.argv[1]); open(p,'w').write(s.replace(o,n))
