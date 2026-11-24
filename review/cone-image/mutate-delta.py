import sys
name=sys.argv[1]
I='crates/topo/src/injective.rs'; C='crates/topo/src/clearance.rs'
ROW="""        return Some(Row::Bulging(curve_of(pcurve, lift)?.at(t)));"""
def rowmut(body): return [(C,ROW,"""        let c = curve_of(pcurve, lift)?;
        if matches!(c, injective::ChartCurve::Focal(_)) {
"""+body+"""
        }
        return Some(Row::Bulging(c.at(t)));""")]
M={
 'Mx1row':rowmut("            return Some(Row::Ends([c.at(t0), c.at(t1)]));"),
 'Mtenth':rowmut("""            let [u, v] = c.at(t); let w = v.hi() - v.lo();
            return Some(Row::Bulging([u, Interval::from_bounds(v.lo() + 0.1 * w, v.hi() - 0.1 * w)]));"""),
 'M9':rowmut("            return None;"),
 'RnearLine':[(I,"""    let from_start = o.ends[0] == c.vertex;
    let speed = speeds(surface, x, y);""","""    let from_start = o.ends[0] == c.vertex;
    let nv = if y.0.abs() < y.1.abs() { y.0 } else { y.1 };
    let speed = speeds(surface, x, if let Surface::Cone { .. } = surface { (nv, nv) } else { y });""")],
 'RnearLeave':[(I,"""    let speed = speeds(surface, r.0, r.1);
    let (near, far) = (shifted""","""    let nv = if r.1.0.abs() < r.1.1.abs() { r.1.0 } else { r.1.1 };
    let speed = speeds(surface, r.0, if let Surface::Cone { .. } = surface { (nv, nv) } else { r.1 });
    let (near, far) = (shifted""")],
 'Rregular':[(I,"""        Surface::Cone { .. } => (y.0 > 0.0 || y.1 < 0.0) && x.1 - x.0 < TURN,""","""        Surface::Cone { .. } => true,""")],
 'Mside':[(I,"""            Err(e) => return Err(e),
        };
        // A run through the vertex: the part of it on the shared cell is""","""            Err(e) => return Err(e),
        };
        let above = if let Surface::Cone { .. } = piece.surface { !above } else { above };
        // A run through the vertex: the part of it on the shared cell is""")],
 'M8':[(I,"""            (ChartCurve::Focal(f), Surface::Cone { .. }) => Some(Self::Focal(f)),""","""            (ChartCurve::Focal(_), Surface::Cone { .. }) => None,""")],
 'Mdelete':[(C,"""    if let Pcurve::FocalSection(f) = *p {""","""    if let Pcurve::FocalSection(f) = *p && false {""")],
 'Mhullin':[(I,"""        if low.is_certified() && high.is_certified() {
            Interval::from_bounds(low.lo(), high.hi())""","""        if low.is_certified() && high.is_certified() {
            if axis == Axis::U { return Interval::from_bounds(low.lo().next_up(), high.hi().next_down()); }
            Interval::from_bounds(low.lo(), high.hi())""")],
}
for f,a,b in M[name]:
    s=open(f).read(); assert s.count(a)==1,(name,f,s.count(a)); s=s.replace(a,b); open(f,'w').write(s)
print('applied',name)
