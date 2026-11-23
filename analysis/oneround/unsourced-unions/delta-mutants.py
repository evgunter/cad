P='crates/geom-brep/src/curve_pair/pose.rs'
R='crates/topo/src/rim_wedge.rs'; C='crates/topo/src/chart_region.rs'; O='crates/topo/src/offset_axial.rs'
V='crates/topo/src/boolean/overlap.rs'; G='crates/topo/src/ring_path.rs'; CV='crates/topo/src/cover.rs'
GB='-p geom-brep --lib'; TL='-p topo --lib'; LR='test(curve_pair::)'
SR='    let (ra, rb) = (read(a), read(b));\n    (ra.below().max(rb.below()), ra.above().min(rb.above()))'
OLDREAD='''        let cover = s
            .carrier()
            .cover_over(geom::cover::ParamWindow::new(t0, t1))
            .ok();
        let above = crate::ExtentBall::of_cover_from(cover.as_ref(), at).lever();
        crate::edge_reach_bounds(s.carrier(), (t0, t1), at, above)'''
DIAM='''        let _ = at;
        crate::LeverBounds::Between { below: crate::edge_extent_inf(s.carrier(), t0, t1), above: crate::edge_extent_sup(s.carrier(), t0, t1) }'''
OWNMID='''        let at = s.carrier().eval(geom::mid_param(t0, t1));
        let cover = s
            .carrier()
            .cover_over(geom::cover::ParamWindow::new(t0, t1))
            .ok();
        let above = crate::ExtentBall::of_cover_from(cover.as_ref(), at).lever();
        crate::edge_reach_bounds(s.carrier(), (t0, t1), at, above)'''
CP='''        match crate::validate::decide_levered(
            ("offset_axial_cap_pair", "offset_axial_cap_pair_straddle"),
            sine.abs(),
            arm,
            band,
        ) {'''
SP='''            match crate::validate::decide_levered(
                ("offset_axial_corner", "offset_axial_corner_straddle"),
                det.abs(),
                arm,
                band,
            ) {'''
OT='test(offset_axial::) | test(offset)'
RG='test(ring_path)'
M=[
 ('D1_diameters',P,OLDREAD,DIAM,GB,LR),
 ('D2_minmax_swap',P,SR,'    let (ra, rb) = (read(a), read(b));\n    (ra.below().min(rb.below()), ra.above().max(rb.above()))',GB,LR),
 ('D3_bounds_swap',P,SR,'    let (ra, rb) = (read(a), read(b));\n    (ra.above().max(rb.above()), ra.below().min(rb.below()))',GB,LR),
 ('D4_over_below',P,SR,'    let (ra, rb) = (read(a), read(b));\n    (ra.below().max(rb.below()), ra.below().min(rb.below()))',GB,LR),
 ('D5_cross_above',P,SR,'    let (ra, rb) = (read(a), read(b));\n    (ra.above().max(rb.above()), ra.above().min(rb.above()))',GB,LR),
 ('D6_own_middle',P,OLDREAD,OWNMID,GB,LR),
 ('D7_route_L2e',P,'    if decide(row::TILT, Margin::levered(tilt, over), band) != Ok(Sign::Zero) {','    if decide(row::TILT, Margin::levered(tilt, geom_core::Lever::unsourced(length + e + e)), band) != Ok(Sign::Zero) {',GB,LR),
 ('D8_lc_semiaxis',P,'    let (arm, over) = span_reaches(a, b, mid);\n    match route(\n        den,','    let (arm, over) = { let (_, o_) = span_reaches(a, b, mid); let (t0, t1) = a.params(); (geom_core::Lever::unsourced(crate::edge_extent_inf(a.carrier(), t0, t1).length().max(k.speed_hi())), o_) };\n    match route(\n        den,',GB,LR),
 ('O1_overlap_unit',V,'Margin::levered(na.dot(nb), arm),','Margin::levered(na.dot(nb), geom_core::Lever::unsourced(T::one())),',TL,'test(overlap) | test(holdings_rows)'),
 ('O2_overlap_world',V,'crate::cover::boundary_reach_below(a.0, a.1, p)','crate::cover::boundary_reach_below(a.0, a.1, Point3::origin())',TL,'test(overlap) | test(holdings_rows)'),
 ('R1_rim_sum',R,'ra.reach_from(ra.center).max(rb.reach_from(ra.center))','geom_core::Lever::unsourced(ra.radius + rb.radius)',TL,'test(rim_wedge::)'),
 ('R2_rim_world',R,'ra.reach_from(ra.center).max(rb.reach_from(ra.center))','ra.reach_from(Point3::origin()).max(rb.reach_from(Point3::origin()))',TL,'test(rim_wedge::)'),
 ('C1_chart_world',C,'geom_brep::ExtentBall::of_cover_from(Some(&pair_cover(body, face, band)?), pivot)','geom_brep::ExtentBall::of_cover_from(Some(&pair_cover(body, face, band)?), geom_core::Point3::origin())','-p topo','test(chart_region) | test(mate5_cyl_eps_rung)'),
 ('A1_corner_arclen',O,'        out.push(reach);\n','        out.push(match edge.certified().map(|c| (c.carrier().clone(), c.params())) { Some((Curve3::Circle(ci), (t0, t1))) => LeverBounds::Exact(Lever::unsourced((t1 - t0).abs() * ci.radius())), _ => reach });\n',TL,'test(offset_axial::)'),
 ('A2_corner_mid',O,'crate::cover::edge_reach_bounds(body, edge, corner)','crate::cover::edge_reach_bounds(body, edge, { let c = edge.certified().unwrap(); let (t0, t1) = c.params(); c.carrier().eval(geom::mid_param(t0, t1)) })',TL,'test(offset_axial::)'),
 ('A3_pairs_below',O,SP,'            match decide("offset_axial_corner", Margin::levered(det.abs(), arm.below()), band) {',TL,OT),
 ('A4_pairs_above',O,SP,'            match decide("offset_axial_corner", Margin::levered(det.abs(), arm.above()), band) {',TL,OT),
 ('A5_cap_below',O,CP,'        match decide("offset_axial_cap_pair", Margin::levered(sine.abs(), arm.below()), band) {',TL,OT),
 ('A6_cap_above',O,CP,'        match decide("offset_axial_cap_pair", Margin::levered(sine.abs(), arm.above()), band) {',TL,OT),
 ('A7_planar_above',O,'arms.iter().map(|arm| arm.below()).collect()','arms.iter().map(|arm| arm.above()).collect()',TL,OT),
 ('G1_ring_arclen',G,'''        let length = length.unwrap_or_else(|| {''','''        let length = length.unwrap_or_else(|| { if true { let (t0, t1) = params; return match &curve { geom::Curve3::Circle(c) => Lever::unsourced(c.radius() * (t1 - t0)), _ => unreachable!() }; }''',TL,RG),
 ('G2_ring_diam',G,'''        let length = length.unwrap_or_else(|| {''','''        let length = length.unwrap_or_else(|| { if true { let (t0, t1) = params; return geom_brep::edge_extent_inf(&curve, t0, t1); }''',TL,RG),
 ('G3_ring_above',G,'''            .below()
        });''','''            .above()
        });''',TL,RG),
 ('G4_seg_half',G,'''            let length = geom_brep::ExtentBall::point(from).lever_from(to);
            (''','''            let length = geom_brep::ExtentBall::point(from).lever_from(from + (to - from) * T::from_f64(0.5));
            (''',TL,RG),
]
