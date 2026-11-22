# id, file, old, new, package, filter
P='crates/geom-brep/src/curve_pair/pose.rs'
R='crates/topo/src/rim_wedge.rs'
C='crates/topo/src/chart_region.rs'
O='crates/topo/src/offset_axial.rs'
V='crates/topo/src/boolean/overlap.rs'
G='crates/topo/src/ring_path.rs'
GB='-p geom-brep --lib'
TL='-p topo --lib'
LR="test(curve_pair::)"
M=[
 ('GM1_route_L2e',P,'        Margin::levered(tilt, over),\n','        Margin::levered(tilt, geom_core::Lever::unsourced(length + e + e)),\n',GB,LR),
 ('GM2_lc_semiaxis',P,'    let (arm, over) = span_extents(a, b);\n    let ends_a = line_ends(a)?;','    let (arm, over) = { let (_, o) = span_extents(a, b); let (t0, t1) = a.params(); (geom_core::Lever::unsourced(crate::edge_extent_inf(a.carrier(), t0, t1).length().max(k.speed_hi())), o) };\n    let ends_a = line_ends(a)?;',GB,LR),
 ('GM3_over_below',P,'(inf_a.max(inf_b), sup_a.min(sup_b))','(inf_a.max(inf_b), inf_a.min(inf_b))',GB,LR),
 ('GM4_cross_above',P,'(inf_a.max(inf_b), sup_a.min(sup_b))','(sup_a.max(sup_b), sup_a.min(sup_b))',GB,LR),
 ('GM5_minmax_swap',P,'(inf_a.max(inf_b), sup_a.min(sup_b))','(inf_a.min(inf_b), sup_a.max(sup_b))',GB,LR),
 ('GM6_over_longer',P,'(inf_a.max(inf_b), sup_a.min(sup_b))','(inf_a.max(inf_b), sup_a.max(sup_b))',GB,LR),
 ('GM7_ll_old_metered',P,'    let (arm, over) = span_extents(a, b);\n    let crossing = || {','    let (arm, over) = { let (_, o) = span_extents(a, b); let m = |s: &Span<\'_, T>| { let (t0, t1) = s.params(); Margin::metered_sup((t1 - t0).abs(), s.speeds().1).value() }; (geom_core::Lever::unsourced(m(a).max(m(b))), o) };\n    let crossing = || {',GB,LR),
 ('TM1_rim_sum',R,'ra.reach_from(ra.center).max(rb.reach_from(ra.center))','geom_core::Lever::unsourced(ra.radius + rb.radius)',TL,'test(rim_wedge::)'),
 ('TM2_rim_world',R,'ra.reach_from(ra.center).max(rb.reach_from(ra.center))','ra.reach_from(Point3::origin()).max(rb.reach_from(Point3::origin()))',TL,'test(rim_wedge::)'),
 ('TM3_rim_own_centres',R,'ra.reach_from(ra.center).max(rb.reach_from(ra.center))','ra.reach_from(ra.center).max(rb.reach_from(rb.center))',TL,'test(rim_wedge::)'),
 ('TM3b_rim_pivot_rb',R,'ra.reach_from(ra.center).max(rb.reach_from(ra.center))','ra.reach_from(rb.center).max(rb.reach_from(rb.center))',TL,'test(rim_wedge::)'),
 ('TM4_chart_tilt_world',C,'ExtentBall::of_cover_from(Some(&cover), pivot)','ExtentBall::of_cover_from(Some(&cover), geom_core::Point3::origin())','-p topo','test(chart_region) | test(mate5_cyl_eps_rung)'),
 ('TM5_chart_sense_above',C,'Margin::levered(a.axis().dot(b.axis()), below)','Margin::levered(a.axis().dot(b.axis()), above)','-p topo','test(chart_region) | test(mate5_cyl_eps_rung)'),
 ('TM6_chart_tilt_below',C,'Margin::levered(a.axis().cross(b.axis()).norm(), above)','Margin::levered(a.axis().cross(b.axis()).norm(), below)','-p topo','test(chart_region) | test(mate5_cyl_eps_rung)'),
 ('TM6b_chart_old_hyp',C,'Margin::levered(a.axis().cross(b.axis()).norm(), above)','Margin::levered(a.axis().cross(b.axis()).norm(), { let mut reach = T::zero(); for (body, face) in [(body_a, face_a), (body_b, face_b)] { let cover = pair_cover(body, face, band)?; for frame in [a, b] { let (lo, hi) = cover.range_along(frame.axis()); let base = (frame.origin() - geom_core::Point3::origin()).dot(frame.axis()); reach = reach.max((lo - base).abs()).max((hi - base).abs()); } } Lever::unsourced((reach.powi(2) + a_radius.max(b_radius).powi(2)).sqrt()) })','-p topo','test(chart_region) | test(mate5_cyl_eps_rung)'),
 ('TM7_corner_arclen',O,'        out.push(reach);\n','        out.push(match body.edge_curve_linked(key, edge).certified().map(|c| (c.carrier().clone(), c.params())) { Some((Curve3::Circle(ci), (t0, t1))) => LeverBounds::Exact(Lever::unsourced((t1 - t0).abs() * ci.radius())), _ => reach });\n',TL,'test(offset_axial::)'),
 ('TM8_corner_mid',O,'crate::cover::edge_reach_bounds(body, key, edge, corner)','crate::cover::edge_reach_bounds(body, key, edge, { let c = body.edge_curve_linked(key, edge).certified().unwrap(); let (t0, t1) = c.params(); c.carrier().eval(geom::mid_param(t0, t1)) })',TL,'test(offset_axial::)'),
 ('TM9_corner_below_only',O,'''        match crate::validate::decide_levered(
                ("offset_axial_corner", "offset_axial_corner_straddle"),
                det.abs(),
                arm,
                band,
            ) {''','''        match decide("offset_axial_corner", Margin::levered(det.abs(), arm.below()), band) {''',TL,'test(offset_axial::) | test(offset)'),
 ('TM10_corner_above_only',O,'''        match crate::validate::decide_levered(
                ("offset_axial_corner", "offset_axial_corner_straddle"),
                det.abs(),
                arm,
                band,
            ) {''','''        match decide("offset_axial_corner", Margin::levered(det.abs(), arm.above()), band) {''',TL,'test(offset_axial::) | test(offset)'),
 ('TM11_cappair_below_only',O,'''        match crate::validate::decide_levered(
            ("offset_axial_cap_pair", "offset_axial_cap_pair_straddle"),
            sine.abs(),
            arm,
            band,
        ) {''','''        match decide("offset_axial_cap_pair", Margin::levered(sine.abs(), arm.below()), band) {''',TL,'test(offset_axial::) | test(offset)'),
 ('TM12_planar_above',O,'arms.iter().map(|arm| arm.below()).collect()','arms.iter().map(|arm| arm.above()).collect()',TL,'test(offset_axial::) | test(offset)'),
 ('TM13_overlap_above',V,'.map(geom_brep::LeverBounds::below)','.map(geom_brep::LeverBounds::above)',TL,'test(overlap)'),
 ('TM14_overlap_unit',V,'Margin::levered(na.dot(nb), arm),','Margin::levered(na.dot(nb), Lever::unsourced(T::one())),',TL,'test(overlap)'),
 ('TM15_ring_swap',G,'            let length = geom_brep::edge_extent_inf(&arc, t0, t1);\n            let extent = geom_brep::edge_extent_sup(&arc, t0, t1);','            let length = geom_brep::edge_extent_sup(&arc, t0, t1);\n            let extent = geom_brep::edge_extent_inf(&arc, t0, t1);',TL,'test(ring_path)'),
 ('TM16_ring_arclen',G,'            let length = geom_brep::edge_extent_inf(&arc, t0, t1);','            let length = Lever::unsourced(p.circle.radius() * (t1 - t0));',TL,'test(ring_path)'),
]
