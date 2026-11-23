import sys
name=sys.argv[1]
I='crates/topo/src/injective.rs'; C='crates/topo/src/clearance.rs'
M={
 'Rswap':[(I,"""                    Sense::With => Interval::from_bounds(first.lo(), last.hi()),
                    Sense::Against => Interval::from_bounds(last.lo(), first.hi()),""","""                    Sense::Against => Interval::from_bounds(first.lo(), last.hi()),
                    Sense::With => Interval::from_bounds(last.lo(), first.hi()),""")],
 'Rnosense':[(I,"""                    Sense::Against => Interval::from_bounds(last.lo(), first.hi()),""","""                    Sense::Against => Interval::from_bounds(first.lo(), last.hi()),""")],
 'Rregular':[(I,"""        Surface::Cone { .. } => (y.0 > 0.0 || y.1 < 0.0) && x.1 - x.0 < TURN,""","""        Surface::Cone { .. } => true,""")],
 'Rturnhalf':[(I,"""        Surface::Cone { .. } => (y.0 > 0.0 || y.1 < 0.0) && x.1 - x.0 < TURN,""","""        Surface::Cone { .. } => (y.0 > 0.0 || y.1 < 0.0),""")],
 'Rnear':[(I,"""    let (ju, jv, _) = rect_jet(surface, u, v);
    [ju.norm().hi(), jv.norm().hi()]""","""    let v = if let Surface::Cone { .. } = surface { let n = if v.0.abs() < v.1.abs() { v.0 } else { v.1 }; (n, n) } else { v };
    let (ju, jv, _) = rect_jet(surface, u, v);
    [ju.norm().hi(), jv.norm().hi()]""")],
 'Rlower':[(I,"""    let (ju, jv, _) = rect_jet(surface, u, v);
    [ju.norm().hi(), jv.norm().hi()]""","""    let (ju, jv, _) = rect_jet(surface, u, v);
    [ju.norm().lo(), jv.norm().lo()]""")],
 'Mx1row':[(C,"""        return Some(Row::Bulging(curve_of(pcurve, lift)?.at(t)));""","""        let c = curve_of(pcurve, lift)?;
        if matches!(c, injective::ChartCurve::Focal(_)) {
            return Some(Row::Ends([c.at(t0), c.at(t1)]));
        }
        return Some(Row::Bulging(c.at(t)));""")],
 'M7':[(I,"""            | (Surface::Cone { .. }, Some(c @ ChartCurve::Focal(_))) => Some(Reading::Line(c)),""",""" => Some(Reading::Line(c)),""")],
 'M8':[(I,"""            (ChartCurve::Focal(f), Surface::Cone { .. }) => Some(Self::Focal(f)),""","""            (ChartCurve::Focal(_), Surface::Cone { .. }) => None,""")],
 'M9':[(C,"""        return Some(Row::Bulging(curve_of(pcurve, lift)?.at(t)));""","""        let c = curve_of(pcurve, lift)?;
        if matches!(c, injective::ChartCurve::Focal(_)) {
            return None;
        }
        return Some(Row::Bulging(c.at(t)));""")],
 'Rnearsite':[(I,"""    let from_start = o.ends[0] == c.vertex;
    let speed = speeds(surface, x, y);""","""    let from_start = o.ends[0] == c.vertex;
    let near = if y.0.abs() < y.1.abs() { y.0 } else { y.1 };
    let speed = speeds(surface, x, if let Surface::Cone { .. } = surface { (near, near) } else { y });"""),(I,"""    let speed = speeds(surface, r.0, r.1);
    let (near, far) = (shifted""","""    let nv = if r.1.0.abs() < r.1.1.abs() { r.1.0 } else { r.1.1 };
    let speed = speeds(surface, r.0, if let Surface::Cone { .. } = surface { (nv, nv) } else { r.1 });
    let (near, far) = (shifted""")],
 'Mdelete':[(C,"""    if let Pcurve::FocalSection(f) = *p {""","""    if let Pcurve::FocalSection(f) = *p && false {""")],
}
for f,a,b in M[name]:
    s=open(f).read(); assert s.count(a)==1,(name,f,s.count(a)); s=s.replace(a,b); open(f,'w').write(s)
print('applied',name)
