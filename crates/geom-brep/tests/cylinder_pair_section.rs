//! **Two cylinders on the section value** (`geom_brep::pair_section`):
//! the pose decided once over the operation's region, in every pose the
//! survey of `work/sectarms/skew-cylinder-germ-pair-has-no-section-frame.md`
//! measured, and the components carried over the pair's ball cut by the
//! region.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::Surface;
use geom_brep::{
    CarryError, ClassCarrier, Coincident, ComponentEnds, ComponentShape, Components, ExtentBall,
    PairSection, SectionError, SectionRead, SsiLane, ZeroKind,
};
use geom_core::{Band, Point3, Tol, Vec3};

fn band() -> Band {
    Band::linear(Tol::witness()).unwrap()
}

/// The survey's drum, radius `r` about `axis` through `origin`.
fn wall(origin: Point3<f64>, axis: Vec3<f64>, r: f64) -> Surface<f64> {
    let axis = axis.normalize();
    Surface::Cylinder {
        origin,
        axis,
        radius: r,
        u_ref: axis.orthonormal_basis().0,
    }
}

/// `z` turned by `angle` about `about`.
fn turned(about: Vec3<f64>, angle: f64) -> Vec3<f64> {
    let k = about.normalize();
    let z = Vec3::unit_z();
    let (s, c) = angle.sin_cos();
    z * c + k.cross(z) * s + k * (k.dot(z) * (1.0 - c))
}

/// The drum at the origin, radius 0.5 about `z`.
fn drum() -> Surface<f64> {
    wall(Point3::origin(), Vec3::unit_z(), 0.5)
}

/// The survey's placements of the second drum: a turn, then a move.
fn placed(about: Vec3<f64>, angle: f64, to: (f64, f64, f64)) -> Surface<f64> {
    wall(Point3::new(to.0, to.1, to.2), turned(about, angle), 0.5)
}

/// The operation's region for two unit-long drums about the origin: the
/// ball about their boxes' overlap, padded.
fn region() -> ExtentBall<f64> {
    ExtentBall::of_sphere(Point3::origin(), 2.0)
}

fn built(a: &Surface<f64>, b: &Surface<f64>, region: ExtentBall<f64>) -> PairSection<f64> {
    match geom_brep::pair_section(a, b, Some(region), None, band()) {
        Ok(SectionRead::Built(section)) => section,
        other => panic!("expected the section, got {other:?}"),
    }
}

/// `p` lies on both walls, a closed form's point to rounding.
fn on_both(p: Point3<f64>, a: &Surface<f64>, b: &Surface<f64>, what: &str) {
    near_both(p, 1e-9, a, b, what);
}

/// `p` lies within `pad` of both walls.
fn near_both(p: Point3<f64>, pad: f64, a: &Surface<f64>, b: &Surface<f64>, what: &str) {
    for s in [a, b] {
        let r = geom_brep::implicit_residual(s, p);
        assert!(
            r.abs() <= pad,
            "{what}: {p:?} is {r:e} off {s:?}, past {pad:e}"
        );
    }
}

/// **A pair with no bounded side needs the region**: asked without one,
/// the value refuses typed rather than reading the pair over nothing.
#[test]
fn two_cylinders_without_a_region_refuse_typed() {
    let got = geom_brep::pair_section(
        &drum(),
        &placed(Vec3::unit_x(), 0.7, (0.1, 0.35, 0.2)),
        None,
        None,
        band(),
    );
    assert!(matches!(got, Err(SectionError::NoRegion { .. })), "{got:?}");
}

/// **Parallel axes** (the survey's p0): two rulings, open with both ends
/// cut by the region, each a closed-form line on both walls, its span
/// the region's chord; no tangency.
#[test]
fn parallel_drums_cut_in_two_rulings_cut_by_the_region() {
    let (a, b) = (drum(), placed(Vec3::unit_x(), 0.0, (0.45, 0.1, 0.05)));
    for (label, x, y) in [("a, b", &a, &b), ("b, a", &b, &a)] {
        let section = built(x, y, region());
        assert!(
            section.decided().is_empty(),
            "{label}: {:?}",
            section.decided()
        );
        let listed = section.counted().expect("rulings are counted");
        assert_eq!(listed.len(), 2, "{label}: two rulings");
        for c in listed {
            assert_eq!(
                c.shape,
                ComponentShape::Open {
                    ends: ComponentEnds::cut()
                },
                "{label}: a ruling is cut by the region at both ends"
            );
            let ClassCarrier::Span { curve, span } = &c.carrier else {
                panic!(
                    "{label}: a ruling is a closed form over a span: {:?}",
                    c.carrier
                );
            };
            for t in [span.0, span.1, 0.5 * (span.0 + span.1)] {
                let p = curve.eval(t);
                on_both(p, x, y, label);
                let out = (p - region().center()).norm() - region().radius();
                assert!(
                    out < 1e-9,
                    "{label}: the span stays in the region, {out:e} out"
                );
            }
            assert!(
                span.1 - span.0 > 3.0,
                "{label}: the chord of a 2 m ball: {span:?}"
            );
        }
        let carried = section
            .carry(None, band())
            .expect("closed forms carry with no lane");
        assert!(
            carried.components().iter().all(|c| !c.closed()),
            "{label}: open"
        );
    }
}

/// **Crossing axes in the survey's skew and one-loop poses** (p1 at
/// exactly a right angle, p2, p3, p5): no tangency, and the components
/// the ruling sweep counts carried through the march, every carried
/// point on both walls, each a loop or an arc the region cuts.
#[test]
fn the_surveys_crossing_poses_are_on_the_value_and_carry() {
    let poses = [
        (
            "p1 at π/2",
            placed(
                Vec3::unit_x(),
                core::f64::consts::FRAC_PI_2,
                (0.3, 0.2, 0.1),
            ),
        ),
        ("p2", placed(Vec3::unit_x(), 0.7, (0.1, 0.35, 0.2))),
        (
            "p3",
            placed(Vec3::new(0.3, 1.0, 0.2), 1.1, (0.42, -0.15, 0.3)),
        ),
        (
            "p5",
            placed(Vec3::new(1.0, 1.0, 0.0), 0.9, (0.05, 0.1, 0.02)),
        ),
    ];
    let a = drum();
    for (label, b) in &poses {
        let section = built(&a, b, region());
        assert!(section.tangency().is_none(), "{label}: transverse");
        assert!(section.nodes().is_empty(), "{label}: no node");
        let carried = section
            .carry(Some(SsiLane::marched()), band())
            .unwrap_or_else(|e| panic!("{label}: carries: {e:?}"));
        assert!(!carried.components().is_empty(), "{label}: the walls cross");
        for c in carried.components() {
            assert!(c.certificate.is_some(), "{label}: a fitted component");
            near_both(c.witness, c.pad + band().zero(), &a, b, label);
            match (&section.components(), c.shape) {
                (Components::Counted(_), ComponentShape::Loop | ComponentShape::Open { .. }) => {}
                (listed, shape) => panic!("{label}: {shape:?} under {listed:?}"),
            }
        }
    }
}

/// **The count is the closed form's where the region holds the ball**:
/// perpendicular axes a gap `e` apart, `r₁ = 0.5`, `r₂ = 0.2`. At
/// `e = 0.1` the thinner wall passes wholly through the fatter one, two
/// loops each winding about it; at `e = 0.4` one loop that winds about
/// neither. The march finds exactly those.
#[test]
fn crossing_walls_count_one_or_two_loops_by_the_nest() {
    let a = drum();
    for (e, loops, winds) in [(0.1, 2, [false, true]), (0.4, 1, [false, false])] {
        let b = wall(Point3::new(0.0, e, 0.0), Vec3::unit_x(), 0.2);
        for (label, x, y, winds) in [
            ("a, b", &a, &b, winds),
            ("b, a", &b, &a, [winds[1], winds[0]]),
        ] {
            let section = built(x, y, region());
            let listed = section
                .counted()
                .unwrap_or_else(|| panic!("e = {e} ({label}): counted"));
            assert_eq!(listed.len(), loops, "e = {e} ({label}): loops");
            for c in listed {
                assert_eq!(c.shape, ComponentShape::Loop, "e = {e} ({label})");
                assert_eq!(c.winds, winds, "e = {e} ({label}): winding");
                assert!(
                    matches!(c.carrier, ClassCarrier::General),
                    "e = {e} ({label})"
                );
            }
        }
        let section = built(&a, &b, region());
        let carried = section.carry(Some(SsiLane::marched()), band()).unwrap();
        assert_eq!(carried.components().len(), loops, "e = {e}: carried");
        assert!(carried.components().iter().all(|c| c.closed()), "e = {e}");
    }
}

/// **A region that cuts the section counts its cut arcs**: the survey's
/// p2 over a region of radius 0.6 about the origin, which cuts the pair's
/// one loop. The ruling sweep lists arcs that end on the region, each
/// with its two cut ends on the region's sphere, and the march carries
/// branches that end there, at the sphere and not the cube about it.
#[test]
fn a_region_that_cuts_the_loop_carries_cut_components() {
    let (a, b) = (drum(), placed(Vec3::unit_x(), 0.7, (0.1, 0.35, 0.2)));
    let region = ExtentBall::of_sphere(Point3::origin(), 0.6);
    let section = built(&a, &b, region);
    let arcs = listed(&section);
    assert!(!arcs.is_empty(), "the region cuts the loop");
    for c in arcs {
        let ends = c
            .cut_ends
            .expect("an arc the region cuts names its cut ends");
        for p in ends {
            on_both(p, &a, &b, "a cut end");
            let off = (p - region.center()).norm() - region.radius();
            // Placed to where its side of the sphere stops deciding.
            assert!(
                off.abs() <= 10.0 * band().zero(),
                "a cut end on the region's sphere: {off:e}"
            );
        }
    }
    let carried = section.carry(Some(SsiLane::marched()), band()).unwrap();
    assert_eq!(carried.components().len(), arcs.len(), "one branch per arc");
    // Each branch ends where its arc leaves the region, not past it on
    // the march domain's cube.
    for c in carried.components() {
        for t in [c.span.0, c.span.1] {
            let off = (c.carrier.eval(t) - region.center()).norm() - region.radius();
            assert!(
                off.abs() <= c.pad + 10.0 * band().zero(),
                "a branch end on the region's sphere: {off:e} (pad {:e})",
                c.pad
            );
        }
    }
    assert!(
        carried.components().iter().any(|c| c.shape
            == ComponentShape::Open {
                ends: ComponentEnds::cut()
            }),
        "a component cut by the region: {:?}",
        carried
            .components()
            .iter()
            .map(|c| c.shape)
            .collect::<Vec<_>>()
    );
}

/// **The survey's p4, equal radii on meeting axes**: two nodes in closed
/// form, recorded with the two coincidences and the nest's `Zero`
/// tangency; four arcs, each from one node to the other on a bisector
/// ellipse, carried as closed forms with no march.
#[test]
fn the_bisector_pose_lists_two_nodes_and_four_arcs() {
    let (a, b) = (drum(), placed(Vec3::unit_y(), 0.4, (0.6, 0.0, 0.0)));
    for (label, x, y) in [("a, b", &a, &b), ("b, a", &b, &a)] {
        let section = built(x, y, region());
        let nodes: Vec<Point3<f64>> = section.nodes().iter().map(|n| n.point).collect();
        assert_eq!(nodes.len(), 2, "{label}: two nodes");
        for p in &nodes {
            on_both(*p, x, y, label);
        }
        let records: Vec<_> = section.coincidences().map(|(c, _)| c).collect();
        assert_eq!(
            records,
            [Coincident::CylinderRadii, Coincident::CylinderAxesMeet],
            "{label}: the two coincidences"
        );
        let zero = section.tangency().expect("the node is a Zero tangency");
        assert_eq!(zero.decision.kind(), ZeroKind::Tangency, "{label}");
        assert_eq!(zero.decision.name(), "cc_crossing_nest", "{label}");
        let listed = section.counted().expect("counted");
        assert_eq!(listed.len(), 4, "{label}: four arcs");
        for c in listed {
            let ComponentShape::Open { ends } = c.shape else {
                panic!("{label}: an arc has ends: {:?}", c.shape);
            };
            let [Some(from), Some(to)] = ends.nodes.both().map(|e| e.map(|e| e.node)) else {
                panic!("{label}: an arc ends at nodes: {ends:?}");
            };
            assert_ne!(from, to, "{label}: from one node to the other");
            let ClassCarrier::Span { curve, span } = &c.carrier else {
                panic!("{label}: {:?}", c.carrier);
            };
            for (t, node) in [(span.0, from), (span.1, to)] {
                let gap = (curve.eval(t) - nodes[node]).norm();
                assert!(gap < 1e-9, "{label}: the arc ends at its node, {gap:e} off");
            }
            on_both(curve.eval(0.5 * (span.0 + span.1)), x, y, label);
        }
        let carried = section.carry(None, band()).expect("closed forms carry");
        assert_eq!(carried.components().len(), 4, "{label}");
    }
}

/// **The pinch refuses typed**: perpendicular axes `e = r₁ − r₂` apart,
/// the thinner wall touching the fatter from inside, recorded as a
/// `Zero` tangency. Its lobes have no carrier, so carrying refuses as an
/// unbuilt arm, never as a desync, and the value lists no node while
/// they mint none: which lobe leaves along which ray is not assigned.
#[test]
fn the_pinch_lists_no_node_and_refuses_to_carry() {
    let a = drum();
    let b = wall(Point3::new(0.0, 0.3, 0.0), Vec3::unit_x(), 0.2);
    for (label, x, y) in [("a, b", &a, &b), ("b, a", &b, &a)] {
        let section = built(x, y, region());
        assert!(
            section.nodes().is_empty(),
            "{label}: no node listed: {:?}",
            section.nodes()
        );
        assert_eq!(listed(&section).len(), 2, "{label}: two lobes");
        assert_eq!(
            section.tangency().map(|z| z.decision.name()),
            Some("cc_crossing_nest"),
            "{label}"
        );
        let got = section.carry(Some(SsiLane::marched()), band());
        assert!(
            matches!(got, Err(CarryError::Unminted(why)) if why == geom_brep::pair_section::cylinders::PINCH),
            "{label}: {got:?}"
        );
    }
}

/// **The outer touch is a point**: perpendicular axes `r₁ + r₂` apart,
/// one point component on both walls, a `Zero` tangency, no carrier.
#[test]
fn walls_touching_outside_are_one_point() {
    let a = drum();
    let b = wall(Point3::new(0.0, 0.7, 0.0), Vec3::unit_x(), 0.2);
    let section = built(&a, &b, region());
    let [c] = section.counted().expect("counted") else {
        panic!("one component");
    };
    assert_eq!(c.shape, ComponentShape::Point);
    on_both(c.witness.expect("its point"), &a, &b, "the touch");
    assert_eq!(
        section.tangency().map(|z| z.decision.name()),
        Some("cc_crossing_gap")
    );
}

/// **Parallelism is levered at the region** (C5): two 1 mm rods whose
/// axes `e = 5e-4` apart are tilted so their section moves by `0.3 ε`
/// across the rods' own size (3e-7 rad at ε 1e-9). Over a region that
/// size they read parallel; over a 1 m region the tilt moves the section
/// by `300 ε` and they read crossing, the pair's own ball (`2r/θ`, far
/// past the region) cut by the region.
///
/// The ruling sweep counts that section in two arcs across the region,
/// each with its two cut ends on the region's sphere, with no 3-D sweep
/// (at ε 1e-12 the tilt's inverse levers the meetings' rounding past the
/// band, and the sweep escalates on `ruling_cut` instead).
/// The march then refuses typed: limb 3 cannot prove one arc in the tube
/// of either 2 m, nearly parallel arc (`ssi_tube_one_arc` undecided;
/// `work/ssiarith/limb-three-reads-no-margin-on-a-long-near-parallel-cylinder-arc.md`).
#[test]
fn thin_rods_read_crossing_at_the_region_and_refuse_typed() {
    let r = 1e-3;
    let tilt = 0.3 * band().zero() / r;
    let a = wall(Point3::origin(), Vec3::unit_z(), r);
    let b = wall(Point3::new(5e-4, 0.0, 0.0), turned(Vec3::unit_x(), tilt), r);
    let near = built(&a, &b, ExtentBall::of_sphere(Point3::origin(), r));
    assert!(
        matches!(
            near.pose(),
            geom_brep::PairPose::CylinderCylinder(p)
                if matches!(p.stance(), geom_brep::CcStance::Parallel { .. })
        ),
        "levered at the rods' size the tilt reads parallel"
    );
    let far_region = ExtentBall::of_sphere(Point3::origin(), 1.0);
    // At ε 1e-12 the tilt is 3e-10 rad, and the meetings' rounding,
    // levered by its inverse, outgrows the band about the region's sphere:
    // the sweep cannot place a cut end and escalates on the cut's row.
    if band().zero() < 1e-10 {
        match geom_brep::pair_section(&a, &b, Some(far_region), None, band()) {
            Err(SectionError::Escalated(diag)) => assert!(
                diag.predicate.is_some_and(|p| p.starts_with("ruling_cut")),
                "on the cut: {diag:?}"
            ),
            other => panic!("at ε {:e} the sweep escalates: {other:?}", band().zero()),
        }
        return;
    }
    let far = built(&a, &b, far_region);
    let geom_brep::PairPose::CylinderCylinder(pose) = far.pose() else {
        panic!("a cylinder pair's pose");
    };
    assert!(
        matches!(pose.stance(), geom_brep::CcStance::Crossing { .. }),
        "levered at the region the tilt reads crossing"
    );
    let arcs = listed(&far);
    assert_eq!(arcs.len(), 2, "two arcs across the region");
    for c in arcs {
        let ends = c
            .cut_ends
            .expect("an arc the region cuts names its cut ends");
        for p in ends {
            let off = (p - Point3::origin()).norm() - 1.0;
            assert!(
                off.abs() <= 10.0 * band().zero(),
                "on the region's sphere: {off:e}"
            );
            near_both(p, 10.0 * band().zero(), &a, &b, "a cut end");
        }
    }
    let got = far.carry(Some(SsiLane::marched()), band());
    assert!(
        matches!(
            got,
            Err(CarryError::March(geom_brep::SsiError::TubeNotOneArc { .. }))
        ),
        "{got:?}"
    );
}

/// The section's listed components, counted.
fn listed(section: &PairSection<f64>) -> &[geom_brep::SectionComponent<f64>] {
    section.counted().expect("a counted section")
}

/// **Coaxial walls of two radii nest and have no section**: a coincidence
/// is the axes' distance and the radii both `Zero`, never the distance
/// alone. Concentric at `d = 0` and at `d` in the band, a pipe's two
/// walls a little off one axis, and a boss in its bore: each empty, in
/// both orders, at the value, at the closed-form door and at the C5 pose
/// question, which serves it.
#[test]
fn coaxial_walls_of_two_radii_nest_and_have_no_section() {
    let z = Vec3::unit_z();
    let eps = band().zero();
    let poses = [
        (
            "concentric, 0.5 and 0.2",
            drum(),
            wall(Point3::origin(), z, 0.2),
        ),
        (
            "concentric, 0.5 and 0.3",
            drum(),
            wall(Point3::origin(), z, 0.3),
        ),
        (
            "concentric, d in the band",
            drum(),
            wall(Point3::new(3.0 * eps, 0.0, 0.0), z, 0.3),
        ),
        (
            "a pipe's walls",
            drum(),
            wall(Point3::new(0.05, 0.0, 0.0), z, 0.4),
        ),
        (
            "a boss in its bore",
            drum(),
            wall(Point3::new(0.1, 0.05, 0.0), z, 0.3),
        ),
    ];
    let edge = geom_brep::Reach::Ball(region());
    for (label, a, b) in &poses {
        for (order, x, y) in [("a, b", a, b), ("b, a", b, a)] {
            let section = built(x, y, region());
            assert!(
                listed(&section).is_empty(),
                "{label} ({order}): nested walls, no section"
            );
            assert!(
                section.decided().is_empty(),
                "{label} ({order}): {:?}",
                section.decided()
            );
            let closed = geom_brep::cylinder_cylinder_section(x, y, &region(), band());
            assert!(
                matches!(closed, Ok(geom_brep::EqualCylinderSection::Empty)),
                "{label} ({order}): the closed-form door: {closed:?}"
            );
            let posed = geom_brep::route_pose(x, y, &edge, band());
            assert!(
                matches!(posed, Ok(r) if r.implemented),
                "{label} ({order}): the pose question serves it: {posed:?}"
            );
        }
    }
}

/// **Coaxial walls of one radius are one surface**: the distance and the
/// radii both `Zero` refuse as a coincidence, in both orders and at the
/// closed-form door.
#[test]
fn coaxial_walls_of_one_radius_are_one_surface() {
    let (a, b) = (
        drum(),
        wall(Point3::new(0.0, 0.0, 0.4), Vec3::unit_z(), 0.5),
    );
    for (order, x, y) in [("a, b", &a, &b), ("b, a", &b, &a)] {
        let got = geom_brep::pair_section(x, y, Some(region()), None, band());
        assert!(
            matches!(got, Err(SectionError::CoincidentSurfaces)),
            "{order}: {got:?}"
        );
        let closed = geom_brep::cylinder_cylinder_section(x, y, &region(), band());
        assert!(
            matches!(closed, Err(SectionError::CoincidentSurfaces)),
            "{order}: {closed:?}"
        );
    }
}

/// **Parallel walls of two radii cut in the two circles' chord**,
/// `x = (d² + r₁² − r₂²)/(2d)` from the first axis: two rulings while
/// `|r₁ − r₂| < d < r₁ + r₂`, one tangent ruling at either level (its
/// point on both walls), nothing nested or apart; both orders.
#[test]
fn parallel_walls_of_two_radii_cut_in_the_circles_chord() {
    let z = Vec3::unit_z();
    for (d, rulings, tangent) in [
        (0.1, 0, false),
        (0.2, 0, true),
        (0.3, 2, false),
        (0.6, 2, false),
        (0.8, 0, true),
        (1.0, 0, false),
    ] {
        let b = wall(Point3::new(d * 0.6, d * 0.8, 0.0), z, 0.3);
        let a = drum();
        for (order, x, y) in [("a, b", &a, &b), ("b, a", &b, &a)] {
            let label = format!("d = {d} ({order})");
            let section = built(x, y, region());
            let got = listed(&section);
            let want = if tangent { 1 } else { rulings };
            assert_eq!(got.len(), want, "{label}: components");
            assert_eq!(section.tangency().is_some(), tangent, "{label}: tangency");
            for c in got {
                on_both(c.witness.expect("a witness"), x, y, &label);
                if let ClassCarrier::Span { curve, span } = &c.carrier {
                    on_both(curve.eval(span.0), x, y, &label);
                }
            }
        }
    }
}

/// **Coplanar axes with two radii** (`e = 0`): the thinner wall passes
/// through the fatter one in two loops, each winding about it, counted
/// and carried; both orders, `r₂` 0.2 and 0.45.
#[test]
fn meeting_axes_of_two_radii_cut_in_two_loops_about_the_thinner_wall() {
    let a = drum();
    for r2 in [0.2, 0.45] {
        let b = wall(Point3::origin(), Vec3::unit_x(), r2);
        for (order, x, y, winds) in [
            ("a, b", &a, &b, [false, true]),
            ("b, a", &b, &a, [true, false]),
        ] {
            let label = format!("r₂ = {r2} ({order})");
            let section = built(x, y, region());
            let got = listed(&section);
            assert_eq!(got.len(), 2, "{label}: two loops");
            assert!(
                got.iter().all(|c| c.winds == winds),
                "{label}: about the thinner wall"
            );
            let carried = section.carry(Some(SsiLane::marched()), band()).unwrap();
            assert_eq!(carried.components().len(), 2, "{label}: carried");
            assert!(carried.components().iter().all(|c| c.closed()), "{label}");
        }
    }
}

/// **A region clear of the pair lists nothing, in every stance**: the
/// parallel rulings, a tangent ruling, the outer touch, one loop, two
/// loops, the bisector arcs and the pinch, each against a ball of radius
/// 0.1 at `(0, 0, 30)`.
#[test]
fn a_region_clear_of_the_pair_lists_nothing() {
    let far = ExtentBall::of_sphere(Point3::new(0.0, 0.0, 30.0), 0.1);
    let a = drum();
    let poses = [
        ("rulings", placed(Vec3::unit_x(), 0.0, (0.45, 0.1, 0.05))),
        (
            "a tangent ruling",
            wall(Point3::new(0.8, 0.0, 0.0), Vec3::unit_z(), 0.3),
        ),
        (
            "the outer touch",
            wall(Point3::new(0.0, 0.7, 0.0), Vec3::unit_x(), 0.2),
        ),
        (
            "one loop",
            wall(Point3::new(0.0, 0.4, 0.0), Vec3::unit_x(), 0.2),
        ),
        (
            "two loops",
            wall(Point3::new(0.0, 0.1, 0.0), Vec3::unit_x(), 0.2),
        ),
        ("the bisector", placed(Vec3::unit_y(), 0.4, (0.6, 0.0, 0.0))),
        (
            "the pinch",
            wall(Point3::new(0.0, 0.3, 0.0), Vec3::unit_x(), 0.2),
        ),
    ];
    for (label, b) in &poses {
        let section = built(&a, b, far);
        assert!(
            section.counted().is_some_and(<[_]>::is_empty),
            "{label}: no component, counted: {:?}",
            section.components()
        );
    }
}

/// **The bisector arcs against a region that cuts them**: a ball of
/// radius 0.6 about the axes' meeting point holds the short ellipse's
/// two arcs and cuts the long one's, which are carried whole and flagged
/// cut; every arc keeps its nodes.
#[test]
fn a_region_that_cuts_the_bisector_flags_the_long_arcs_cut() {
    let (a, b) = (drum(), placed(Vec3::unit_y(), 0.4, (0.6, 0.0, 0.0)));
    let whole = built(&a, &b, region());
    let meet = whole.nodes()[0].point + (whole.nodes()[1].point - whole.nodes()[0].point) * 0.5;
    let section = built(&a, &b, ExtentBall::of_sphere(meet, 0.6));
    let got = listed(&section);
    assert_eq!(got.len(), 4, "four arcs");
    let cut: Vec<bool> = got
        .iter()
        .map(|c| match c.shape {
            ComponentShape::Open { ends } => {
                assert!(
                    ends.nodes.both().iter().all(Option::is_some),
                    "an arc keeps its nodes"
                );
                ends.cut
            }
            other => panic!("an arc: {other:?}"),
        })
        .collect();
    assert_eq!(
        cut.iter().filter(|&&c| c).count(),
        2,
        "the long ellipse's arcs: {cut:?}"
    );
}

/// **A ruling the region misses is not listed**: a small region about one
/// of p0's rulings lists that ruling alone.
#[test]
fn a_ruling_the_region_misses_is_not_listed() {
    let (a, b) = (drum(), placed(Vec3::unit_x(), 0.0, (0.45, 0.1, 0.05)));
    let whole = built(&a, &b, region());
    let p = listed(&whole)[0].witness.expect("a witness");
    let section = built(&a, &b, ExtentBall::of_sphere(p, 0.05));
    let got = listed(&section);
    assert_eq!(got.len(), 1, "one ruling reaches the region");
    let w = got[0].witness.expect("a witness");
    assert!(
        (w - p).norm() < 0.05 + 1e-9,
        "the ruling through the region"
    );
}

/// **A region off the pair counts the arc it cuts**: p2's one loop
/// against a ball of radius 3.2 about `(0, 0, 3)`, which holds the
/// pair's ball's size but not its place and cuts the loop once across:
/// one arc, its two cut ends named, carried by one branch.
#[test]
fn a_region_off_the_pair_counts_the_arc_it_cuts() {
    let (a, b) = (drum(), placed(Vec3::unit_x(), 0.7, (0.1, 0.35, 0.2)));
    let section = built(
        &a,
        &b,
        ExtentBall::of_sphere(Point3::new(0.0, 0.0, 3.0), 3.2),
    );
    let got = listed(&section);
    assert_eq!(got.len(), 1, "one arc: {got:?}");
    assert_eq!(
        got[0].shape,
        ComponentShape::Open {
            ends: ComponentEnds::cut()
        },
        "cut by the region"
    );
    assert!(got[0].cut_ends.is_some(), "its cut ends");
    let carried = section.carry(Some(SsiLane::marched()), band()).unwrap();
    assert_eq!(carried.components().len(), 1, "one branch");
}

/// **Every carried point lies within the domain's ball**: the
/// perpendicular pair `e = 0.1`, `r₁ = 0.5`, `r₂ = 0.2`, whose two loops
/// the region holds, sampled along their fitted carriers.
#[test]
fn the_carried_loop_lies_within_the_pairs_ball() {
    let a = drum();
    let b = wall(Point3::new(0.0, 0.1, 0.0), Vec3::unit_x(), 0.2);
    let section = built(&a, &b, region());
    let geom_brep::PairPose::CylinderCylinder(pose) = section.pose() else {
        panic!("a cylinder pair's pose");
    };
    assert_eq!(
        pose.placed(),
        geom_brep::RegionPlacement::Inside,
        "the region holds it"
    );
    let ball = pose.region();
    let carried = section.carry(Some(SsiLane::marched()), band()).unwrap();
    for c in carried.components() {
        for i in 0..=64 {
            let t = c.span.0 + (c.span.1 - c.span.0) * f64::from(i) / 64.0;
            let out = (c.carrier.eval(t) - ball.center()).norm() - ball.radius();
            assert!(out <= c.pad, "{out:e} past the pair's ball");
        }
    }
}

/// **A region through a branch point escalates**: perpendicular walls
/// `r₁ = 0.5` about `z` and `r₂ = 0.2` about `x`, `e = 0.4` apart, cross
/// in one loop whose two branch points (where a ruling of the thinner
/// wall touches the fatter one) stand at `(0, 0.5, ±0.1·√3)`. A region
/// about the origin whose sphere runs through both cannot say on which
/// side of it the loop leaves there, and escalates on the branch cell's
/// reading; a region a millimetre smaller or larger counts the arcs.
#[test]
fn a_region_through_a_branch_point_escalates() {
    let a = drum();
    let b = wall(Point3::new(0.0, 0.4, 0.0), Vec3::unit_x(), 0.2);
    let through = (0.25f64 + 0.03).sqrt();
    match geom_brep::pair_section(
        &a,
        &b,
        Some(ExtentBall::of_sphere(Point3::origin(), through)),
        None,
        band(),
    ) {
        Err(SectionError::Escalated(diag)) => assert!(
            diag.predicate
                .is_some_and(|p| p.starts_with("ruling_branch_cut")),
            "on the branch cell: {diag:?}"
        ),
        other => panic!("a sphere through the branch points escalates: {other:?}"),
    }
    // Either side of it the sphere cuts the saddle-shaped loop in arcs
    // the march carries one to one.
    for (what, radius) in [("smaller", through - 1e-3), ("larger", through + 1e-3)] {
        let section = built(&a, &b, ExtentBall::of_sphere(Point3::origin(), radius));
        let got = listed(&section);
        assert!(
            !got.is_empty() && got.iter().all(|c| c.cut_ends.is_some()),
            "{what}: arcs the region cuts, {got:?}"
        );
        let carried = section
            .carry(Some(SsiLane::marched()), band())
            .unwrap_or_else(|e| panic!("{what}: carries: {e:?}"));
        assert_eq!(
            carried.components().len(),
            got.len(),
            "{what}: one branch per arc"
        );
    }
}

/// **A branch point on one of the sweep's first cuts is counted**:
/// perpendicular walls `r = 0.5` about `z` and `ρ = 0.2` about `x`, the
/// gap `e = 0.5 − 0.2·cos(π/4)` placing a branch point of the thinner
/// wall's sweep at `φ = π/4` or `3π/4` (as the frame falls), both cuts of
/// the turn's first 32. There `Δ` reads zero, so a cell ending there
/// could not decide its end; the sweep cuts off the zero and counts the
/// one loop the region holds, in both orders.
#[test]
fn a_branch_point_on_a_first_cut_is_counted() {
    let a = drum();
    let e = 0.5 - 0.2 * core::f64::consts::FRAC_PI_4.cos();
    let b = wall(Point3::new(0.0, e, 0.0), Vec3::unit_x(), 0.2);
    for (label, x, y) in [("a, b", &a, &b), ("b, a", &b, &a)] {
        let section = built(x, y, region());
        let got = listed(&section);
        assert!(
            got.len() == 1 && got[0].shape == ComponentShape::Loop,
            "{label}: one loop, {got:?}"
        );
    }
}

/// The listed rulings' lines, each a point and its unit direction.
fn ruling_lines(section: &PairSection<f64>) -> Vec<(Point3<f64>, Vec3<f64>)> {
    listed(section)
        .iter()
        .map(|c| match &c.carrier {
            ClassCarrier::Span {
                curve: geom::Curve3::Line(geom::Line { origin, dir }),
                ..
            } => (*origin, dir.normalize()),
            other => panic!("a parallel pair's component is a ruling, not {other:?}"),
        })
        .collect()
}

/// How far `q` is from the line through `o` along the unit `d`.
fn off_line(q: Point3<f64>, (o, d): (Point3<f64>, Vec3<f64>)) -> f64 {
    (q - o).cross(d).norm()
}

/// **The rulings are the walls' meeting lines, in any frame** (moved
/// from the join's rulings-plane rows when the parallel pair came to
/// ride its carried rulings): each listed ruling runs along the axes
/// through one of the cross-section circles' two meeting points, found
/// apart from the value (`x = (d² + r₁² − r₂²)/(2d)` along the axes'
/// offset, `±h` across it). Equal radii, either one larger, an axis
/// inside the other wall, both operand orders, axes off every
/// coordinate direction, origins slid along the axis.
#[test]
fn the_parallel_rulings_run_through_the_circles_meeting_points() {
    let axis = Vec3::new(1.0, 2.0, 0.5).normalize();
    let (e1, e2) = axis.orthonormal_basis();
    for (r1, r2, d, slide) in [
        (0.5, 0.2, 0.5, 0.0),
        (0.5, 0.5, 0.7, 0.3),
        (0.5, 0.8, 0.9, -1.0),
        (0.5, 0.2, 0.4, 2.0),
        (0.3, 0.3, 0.1, 0.0),
    ] {
        let tilt = 0.6_f64;
        let toward = e1 * tilt.cos() + e2 * tilt.sin();
        let o1 = Point3::new(0.2, -0.1, 0.3);
        let o2 = o1 + toward * d + axis * slide;
        let (c1, c2) = (wall(o1, axis, r1), wall(o2, -axis, r2));
        let x = (d * d + r1 * r1 - r2 * r2) / (2.0 * d);
        let h = (r1 * r1 - x * x).sqrt();
        let across = axis.cross(toward);
        let meets = [o1 + toward * x + across * h, o1 + toward * x - across * h];
        for (label, own, partner) in [("ab", &c1, &c2), ("ba", &c2, &c1)] {
            let tag = format!("r {r1}/{r2} d {d} {label}");
            let lines = ruling_lines(&built(own, partner, ExtentBall::of_sphere(o1, 2.0)));
            assert_eq!(lines.len(), 2, "{tag}: two rulings");
            for q in meets {
                let on: Vec<_> = lines.iter().filter(|&&l| off_line(q, l) < 1e-12).collect();
                assert_eq!(on.len(), 1, "{tag}: one ruling through {q:?}");
                assert!(on[0].1.cross(axis).norm() < 1e-12, "{tag}: along the axes");
            }
        }
    }
}

/// **The rulings are read at the region, wherever an origin is
/// stored** (moved from the join's rulings-plane rows). A radius-0.5
/// wall about `z` and a radius-0.3 one whose axis passes `(d, 0, 0)`
/// tilted half the zero band toward `y`, read over a region at the
/// origin; the partner's origin is stored up to 1000 m along its axis.
/// Offset (`d = 0.6`), the rulings pass the walls' two meeting points in
/// `z = 0` within `zero` at every stored origin; coaxial (`d = 0`), the
/// walls list no ruling at every stored origin.
#[test]
fn the_parallel_rulings_are_read_at_the_region_wherever_an_origin_is_stored() {
    let zero = band().zero();
    let theta = 0.5 * zero;
    let tilted = Vec3::new(0.0, theta.sin(), theta.cos());
    let z = Vec3::unit_z();
    let (r1, r2, d): (f64, f64, f64) = (0.5, 0.3, 0.6);
    let x = (d * d + r1 * r1 - r2 * r2) / (2.0 * d);
    let h = (r1 * r1 - x * x).sqrt();
    let meets = [Point3::new(x, h, 0.0), Point3::new(x, -h, 0.0)];
    let at = ExtentBall::of_sphere(Point3::origin(), 2.0);
    for along in [0.0, 10.0, -10.0, 1000.0, -1000.0] {
        let own = wall(Point3::origin(), z, r1);
        let partner = wall(Point3::new(d, 0.0, 0.0) + tilted * along, tilted, r2);
        for (label, a, b) in [("ab", &own, &partner), ("ba", &partner, &own)] {
            let lines = ruling_lines(&built(a, b, at));
            assert_eq!(
                lines.len(),
                2,
                "stored {along} m along ({label}): two rulings"
            );
            for q in meets {
                let off = lines
                    .iter()
                    .map(|&l| off_line(q, l))
                    .fold(f64::INFINITY, f64::min);
                assert!(
                    off < zero,
                    "stored {along} m along ({label}): a ruling passes the walls' meeting \
                     point {q:?} at the region, {off:e} off"
                );
            }
        }
        let partner = wall(Point3::origin() + tilted * along, tilted, r2);
        for (label, a, b) in [("ab", &own, &partner), ("ba", &partner, &own)] {
            let got = listed(&built(a, b, at)).len();
            assert_eq!(
                got, 0,
                "stored {along} m along ({label}): coaxial at the region, no ruling"
            );
        }
    }
}

/// The cylinder `s`'s axis, unit, and radius.
fn axis_of(s: &Surface<f64>) -> (Point3<f64>, Vec3<f64>, f64) {
    let Surface::Cylinder {
        origin,
        axis,
        radius,
        ..
    } = *s
    else {
        panic!("a cylinder: {s:?}")
    };
    (origin, axis.normalize(), radius)
}

/// The cylinder `s`'s second fundamental form at a point, along the unit
/// tangent `v`, against its outward radial: `(|v|² − (v·â)²)/r`.
fn second_form(s: &Surface<f64>, v: Vec3<f64>) -> f64 {
    let (_, a, r) = axis_of(s);
    (v.norm_squared() - v.dot(a).powi(2)) / r
}

/// **The value lists each bisector node with its frame, read off the
/// walls' relative second fundamental form** (C5): at three angles
/// between the axes, in both orders, each node's normal is both walls'
/// outward radial there; its lines are unit, square to it, turned
/// counterclockwise about it, and null for `Q = II₁ − II₂`; the side it
/// lists on each sector is `Q`'s sign at the sector's middle (the first
/// wall stands on the normal's side where it curves away less, `Q < 0`).
/// Each node has one arc end along each of its four rays, and each end's
/// ray is the way its carrier leaves the node: `+C′` at the span's first
/// end, `−C′` at its last (the derivative as the test's oracle).
#[test]
fn the_bisector_nodes_list_their_frame_off_the_relative_form() {
    use geom_brep::{PairSide, SectionNode};
    for angle in [0.4, 1.1, 2.0] {
        let (a, b) = (drum(), placed(Vec3::unit_y(), angle, (0.6, 0.0, 0.0)));
        for (label, x, y) in [("a, b", &a, &b), ("b, a", &b, &a)] {
            let label = format!("{angle} rad, {label}");
            let section = built(x, y, region());
            assert_eq!(section.nodes().len(), 2, "{label}: two nodes");
            for node in section.nodes() {
                for s in [x, y] {
                    let (o, ax, r) = axis_of(s);
                    let w = node.point - o;
                    let radial = (w - ax * w.dot(ax)) / r;
                    assert!(
                        (radial - node.normal).norm() < 1e-9,
                        "{label}: the normal is each wall's outward radial"
                    );
                }
                let [l0, l1] = node.lines;
                for l in [l0, l1] {
                    assert!((l.norm() - 1.0).abs() < 1e-12, "{label}: a unit line");
                    assert!(l.dot(node.normal).abs() < 1e-12, "{label}: tangent");
                    let q = second_form(x, l) - second_form(y, l);
                    assert!(q.abs() < 1e-9, "{label}: a null line of Q, Q = {q:e}");
                }
                assert!(
                    l0.cross(l1).dot(node.normal) > 0.1,
                    "{label}: the lines turn counterclockwise about the normal"
                );
                let rays = [l0, l1, -l0, -l1];
                for i in 0..4 {
                    let m = (rays[i] + rays[(i + 1) % 4]).normalize();
                    let q = second_form(x, m) - second_form(y, m);
                    let want = if q < 0.0 {
                        PairSide::First
                    } else {
                        PairSide::Second
                    };
                    assert_eq!(node.above_in(i), want, "{label}: sector {i}, Q = {q:e}");
                }
            }
            let mut ends = [[0usize; 4]; 2];
            for c in listed(&section) {
                let ComponentShape::Open { ends: at } = c.shape else {
                    panic!("{label}: an arc: {:?}", c.shape);
                };
                let ClassCarrier::Span { curve, span } = &c.carrier else {
                    panic!("{label}: {:?}", c.carrier);
                };
                for (end, t, sign) in [(at.nodes.start, span.0, 1.0), (at.nodes.end, span.1, -1.0)]
                {
                    let end = end.expect("each arc end stands at a node");
                    let node = &section.nodes()[end.node];
                    let d = curve.deriv(t);
                    let leaves = d * (sign / d.norm());
                    assert!(
                        (node.ray(end.line, end.sense) - leaves).norm() < 1e-9,
                        "{label}: the end's ray is the way its carrier leaves the node"
                    );
                    ends[end.node][SectionNode::<f64>::ray_index(end.line, end.sense)] += 1;
                }
            }
            assert_eq!(ends, [[1; 4]; 2], "{label}: one arc end along each ray");
        }
    }
}

/// **A node the region does not hold is not listed** (C5: a node is
/// listed where the domain holds it). A region about one node holds it
/// and cuts all four arcs: one node, each arc ending there at one end
/// and cut at the other, one node-joined set. A region about the axes'
/// meeting point too small to reach either node cuts all four arcs and
/// lists no node: each arc is cut at both ends, and no set joins them.
#[test]
fn a_node_the_region_does_not_hold_is_not_listed() {
    let (a, b) = (drum(), placed(Vec3::unit_y(), 0.4, (0.6, 0.0, 0.0)));
    let whole = built(&a, &b, region());
    let node0 = whole.nodes()[0].point;
    let one = built(&a, &b, ExtentBall::of_sphere(node0, 0.3));
    let [node] = one.nodes() else {
        panic!("one node: {:?}", one.nodes());
    };
    assert!(
        (node.point - node0).norm() < 1e-12,
        "the node the region holds"
    );
    let arcs = listed(&one);
    assert_eq!(arcs.len(), 4, "every arc reaches the region");
    for c in arcs {
        let ComponentShape::Open { ends } = c.shape else {
            panic!("an arc: {:?}", c.shape);
        };
        let at: Vec<usize> = ends.nodes.both().iter().flatten().map(|e| e.node).collect();
        assert_eq!(at, [0], "one end at the listed node: {ends:?}");
        assert!(ends.cut, "the other end runs past the region");
    }
    assert_eq!(one.node_sets().len(), 1, "one set");
    assert_eq!(one.node_sets()[0].arcs, [0, 1, 2, 3], "joined at the node");
    one.carry(None, band()).expect("closed forms carry");

    let meet = node0 + (whole.nodes()[1].point - node0) * 0.5;
    let none = built(&a, &b, ExtentBall::of_sphere(meet, 0.45));
    assert!(none.nodes().is_empty(), "no node: {:?}", none.nodes());
    let arcs = listed(&none);
    assert_eq!(arcs.len(), 4, "every arc reaches the region");
    for c in arcs {
        let ComponentShape::Open { ends } = c.shape else {
            panic!("an arc: {:?}", c.shape);
        };
        assert_eq!(ends.nodes.both(), [None, None], "no end at a node");
        assert!(ends.cut, "cut");
    }
    assert!(none.node_sets().is_empty(), "no set");
}

/// **A site reads as a node or as one arc** (C5), node first: on the
/// carried bisector section, a site inside an arc's span reads that arc
/// and no other (its ellipse's other arc lies on the same carrier), and
/// each node reads as the node. Under a band whose escalation edge
/// stands just past its zero (`1e-4`, `1.1e-4`), a site `1.3e-4` from a
/// node between two of its rays lies within the band of both arcs while
/// decided off the node: it escalates, at its foot's place on the arc
/// or on its distance from the node, never naming one; so does a site
/// in the band of the node itself.
#[test]
fn a_site_reads_as_a_node_or_as_one_arc() {
    use geom_brep::{SiteReading, SiteRefusal};
    let (a, b) = (drum(), placed(Vec3::unit_y(), 0.4, (0.6, 0.0, 0.0)));
    let section = built(&a, &b, region());
    let carried = section.carry(None, band()).expect("closed forms carry");
    for (k, c) in carried.components().iter().enumerate() {
        let (lo, hi) = c.span;
        for f in [0.1, 0.5, 0.9] {
            let t = lo + f * (hi - lo);
            match carried.site(c.carrier.eval(t), band()) {
                Ok(SiteReading::OnArc(foot)) => {
                    assert_eq!(foot.component, k, "arc {k} at {f}: its own arc");
                    assert!(
                        (foot.t - t).abs() < 1e-9,
                        "arc {k} at {f}: {} for {t}",
                        foot.t
                    );
                }
                other => panic!("arc {k} at {f}: {other:?}"),
            }
        }
    }
    for (n, node) in carried.nodes().iter().enumerate() {
        assert!(
            matches!(carried.site(node.point, band()), Ok(SiteReading::AtNode(m)) if m == n),
            "node {n}"
        );
    }
    let tight = Band::new(1e-4, 1.1e-4).unwrap();
    let node = section.nodes()[0];
    let tight_carried = section.carry(None, tight).expect("closed forms carry");
    let near_node = |p: Point3<f64>| match tight_carried.site(p, tight) {
        Err(SiteRefusal::Escalated(d)) => {
            matches!(
                d.predicate,
                Some("section_site_at_node" | "section_site_in_span")
            )
        }
        _ => false,
    };
    let between = node.point + (node.lines[0] + node.lines[1]).normalize() * 1.3e-4;
    assert!(
        near_node(between),
        "near a node, on two of its arcs: {:?}",
        tight_carried.site(between, tight)
    );
    let near = node.point + node.lines[0] * 1.05e-4;
    assert!(
        near_node(near),
        "in band of the node: {:?}",
        tight_carried.site(near, tight)
    );
}
