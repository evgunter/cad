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
    ExtentBall::new(Point3::origin(), 2.0)
}

fn built(a: &Surface<f64>, b: &Surface<f64>, region: ExtentBall<f64>) -> PairSection<f64> {
    match geom_brep::pair_section(a, b, Some(region), band()) {
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
/// carried through the march, every carried point on both walls. Where
/// the region holds the pair's ball the count is the closed form's;
/// where it cuts the ball the components are the march's, closed or cut.
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
                (Components::Counted(_), ComponentShape::Loop) => {}
                (Components::Marched, ComponentShape::Loop | ComponentShape::Open { .. }) => {}
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

/// **A region that cuts the section lists the march's components**:
/// the survey's p2 over a region of radius 0.6 about the origin, which
/// cuts the pair's one loop, carries components that end on the region.
#[test]
fn a_region_that_cuts_the_loop_carries_cut_components() {
    let (a, b) = (drum(), placed(Vec3::unit_x(), 0.7, (0.1, 0.35, 0.2)));
    let section = built(&a, &b, ExtentBall::new(Point3::origin(), 0.6));
    assert!(
        matches!(section.components(), Components::Marched),
        "cut: marched"
    );
    let carried = section.carry(Some(SsiLane::marched()), band()).unwrap();
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
        let nodes = section.nodes().to_vec();
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
            let [Some(from), Some(to)] = ends.nodes else {
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
/// the thinner wall touching the fatter from inside. One node in closed
/// form, `r₁` from the first axis and `r₂` from the second, recorded as a
/// `Zero` tangency; its lobes have no carrier, so carrying refuses as an
/// unbuilt arm, never as a desync.
#[test]
fn the_pinch_lists_its_node_and_refuses_to_carry() {
    let a = drum();
    let b = wall(Point3::new(0.0, 0.3, 0.0), Vec3::unit_x(), 0.2);
    for (label, x, y) in [("a, b", &a, &b), ("b, a", &b, &a)] {
        let section = built(x, y, region());
        let [node] = section.nodes() else {
            panic!("{label}: one node: {:?}", section.nodes());
        };
        on_both(*node, x, y, label);
        assert!(
            (*node - Point3::new(0.0, 0.5, 0.0)).norm() < 1e-9,
            "{label}: the pinch on the common perpendicular: {node:?}"
        );
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
/// past the region) cut by the region, so marched.
///
/// The march then refuses typed, at every tolerance row: limb 3 cannot
/// prove one arc in the tube of either nearly parallel arc
/// (`ssi_tube_one_arc` undecided). This row pins that refusal; it moves
/// when the accounting
/// design (`work/sectarms/a-near-parallel-pairs-accounting-grows-with-the-operation-region.md`)
/// lands.
#[test]
fn thin_rods_read_crossing_at_the_region_and_refuse_typed() {
    let r = 1e-3;
    let tilt = 0.3 * band().zero() / r;
    let a = wall(Point3::origin(), Vec3::unit_z(), r);
    let b = wall(Point3::new(5e-4, 0.0, 0.0), turned(Vec3::unit_x(), tilt), r);
    let near = built(&a, &b, ExtentBall::new(Point3::origin(), r));
    assert!(
        matches!(
            near.pose(),
            geom_brep::PairPose::CylinderCylinder(p)
                if matches!(p.stance(), geom_brep::CcStance::Parallel { .. })
        ),
        "levered at the rods' size the tilt reads parallel"
    );
    let far = built(&a, &b, ExtentBall::new(Point3::origin(), 1.0));
    let geom_brep::PairPose::CylinderCylinder(pose) = far.pose() else {
        panic!("a cylinder pair's pose");
    };
    assert!(
        matches!(pose.stance(), geom_brep::CcStance::Crossing { .. }),
        "levered at the region the tilt reads crossing"
    );
    assert!(
        matches!(far.components(), Components::Marched),
        "cut: marched"
    );
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
        let got = geom_brep::pair_section(x, y, Some(region()), band());
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
    let far = ExtentBall::new(Point3::new(0.0, 0.0, 30.0), 0.1);
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
    let meet = whole.nodes()[0] + (whole.nodes()[1] - whole.nodes()[0]) * 0.5;
    let section = built(&a, &b, ExtentBall::new(meet, 0.6));
    let got = listed(&section);
    assert_eq!(got.len(), 4, "four arcs");
    let cut: Vec<bool> = got
        .iter()
        .map(|c| match c.shape {
            ComponentShape::Open { ends } => {
                assert!(
                    ends.nodes.iter().all(Option::is_some),
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
    let section = built(&a, &b, ExtentBall::new(p, 0.05));
    let got = listed(&section);
    assert_eq!(got.len(), 1, "one ruling reaches the region");
    let w = got[0].witness.expect("a witness");
    assert!(
        (w - p).norm() < 0.05 + 1e-9,
        "the ruling through the region"
    );
}

/// **A region off the pair that cuts its loop does not count it**: p2's
/// one loop against a ball of radius 3.2 about `(0, 0, 3)`, which holds
/// the pair's ball's size but not its place.
#[test]
fn a_region_off_the_pair_that_cuts_the_loop_marches_it() {
    let (a, b) = (drum(), placed(Vec3::unit_x(), 0.7, (0.1, 0.35, 0.2)));
    let section = built(&a, &b, ExtentBall::new(Point3::new(0.0, 0.0, 3.0), 3.2));
    assert!(
        matches!(section.components(), Components::Marched),
        "cut by the region: {:?}",
        section.components()
    );
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
    let ball = pose.ball();
    let carried = section.carry(Some(SsiLane::marched()), band()).unwrap();
    for c in carried.components() {
        for i in 0..=64 {
            let t = c.span.0 + (c.span.1 - c.span.0) * f64::from(i) / 64.0;
            let out = (c.carrier.eval(t) - ball.center()).norm() - ball.radius();
            assert!(out <= c.pad, "{out:e} past the pair's ball");
        }
    }
}
