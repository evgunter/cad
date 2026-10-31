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
        assert!(r.abs() <= pad, "{what}: {p:?} is {r:e} off {s:?}, past {pad:e}");
    }
}

/// **A pair with no bounded side needs the region**: asked without one,
/// the value refuses typed rather than reading the pair over nothing.
#[test]
fn two_cylinders_without_a_region_refuse_typed() {
    let got = geom_brep::pair_section(&drum(), &placed(Vec3::unit_x(), 0.7, (0.1, 0.35, 0.2)), None, band());
    assert!(
        matches!(got, Err(SectionError::NoRegion { .. })),
        "{got:?}"
    );
}

/// **Parallel axes** (the survey's p0): two rulings, open with both ends
/// cut by the region, each a closed-form line on both walls, its span
/// the region's chord; no tangency.
#[test]
fn parallel_drums_cut_in_two_rulings_cut_by_the_region() {
    let (a, b) = (drum(), placed(Vec3::unit_x(), 0.0, (0.45, 0.1, 0.05)));
    for (label, x, y) in [("a, b", &a, &b), ("b, a", &b, &a)] {
        let section = built(x, y, region());
        assert!(section.decided().is_empty(), "{label}: {:?}", section.decided());
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
                panic!("{label}: a ruling is a closed form over a span: {:?}", c.carrier);
            };
            for t in [span.0, span.1, 0.5 * (span.0 + span.1)] {
                let p = curve.eval(t);
                on_both(p, x, y, label);
                let out = (p - region().center()).norm() - region().radius();
                assert!(out < 1e-9, "{label}: the span stays in the region, {out:e} out");
            }
            assert!(span.1 - span.0 > 3.0, "{label}: the chord of a 2 m ball: {span:?}");
        }
        let carried = section.carry(None, band()).expect("closed forms carry with no lane");
        assert!(carried.components().iter().all(|c| !c.closed()), "{label}: open");
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
        ("p1 at π/2", placed(Vec3::unit_x(), core::f64::consts::FRAC_PI_2, (0.3, 0.2, 0.1))),
        ("p2", placed(Vec3::unit_x(), 0.7, (0.1, 0.35, 0.2))),
        ("p3", placed(Vec3::new(0.3, 1.0, 0.2), 1.1, (0.42, -0.15, 0.3))),
        ("p5", placed(Vec3::new(1.0, 1.0, 0.0), 0.9, (0.05, 0.1, 0.02))),
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
        for (label, x, y, winds) in [("a, b", &a, &b, winds), ("b, a", &b, &a, [winds[1], winds[0]])] {
            let section = built(x, y, region());
            let listed = section
                .counted()
                .unwrap_or_else(|| panic!("e = {e} ({label}): counted"));
            assert_eq!(listed.len(), loops, "e = {e} ({label}): loops");
            for c in listed {
                assert_eq!(c.shape, ComponentShape::Loop, "e = {e} ({label})");
                assert_eq!(c.winds, winds, "e = {e} ({label}): winding");
                assert!(matches!(c.carrier, ClassCarrier::General), "e = {e} ({label})");
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
    assert!(matches!(section.components(), Components::Marched), "cut: marched");
    let carried = section.carry(Some(SsiLane::marched()), band()).unwrap();
    assert!(
        carried.components().iter().any(|c| c.shape
            == ComponentShape::Open {
                ends: ComponentEnds::cut()
            }),
        "a component cut by the region: {:?}",
        carried.components().iter().map(|c| c.shape).collect::<Vec<_>>()
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
    assert!(matches!(far.components(), Components::Marched), "cut: marched");
    let got = far.carry(Some(SsiLane::marched()), band());
    assert!(
        matches!(
            got,
            Err(CarryError::March(geom_brep::SsiError::TubeNotOneArc { .. }))
        ),
        "{got:?}"
    );
}
