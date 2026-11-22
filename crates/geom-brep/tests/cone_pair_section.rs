//! **A cone against an oblique cylinder or another cone on the section
//! value** (`geom_brep::pair_section`): the unit cone (apex at the origin,
//! axis `z`, `α = π/4`, `ρ = |z|`) against the poses of `topo`'s
//! `section_cert_cone_pair_rows`, read over a region, each row both ways
//! round and pinned against a traced section.
//!
//! The oracle ([`traced`]) shares no code with the value: it solves the
//! partner's implicit equation along each of a dense fan of the cone's
//! rulings in closed form, keeps the roots inside the region, and joins
//! roots on neighbouring rulings, and the two roots of one ruling, that
//! stand within a link of each other. Each connected cluster is one
//! component inside the region, or one node-joined set; a cluster of
//! roots at the apex alone is the apex touch, which a sign change does
//! not see. Every row asserts its shapes, its count against the trace,
//! each witness on both carriers and on its own cluster, and each cut end
//! on the region's boundary.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::{FRAC_1_SQRT_2, FRAC_PI_4, TAU};
use geom::Surface;
use geom_brep::{
    CarryError, CarryRefusal, ClassCarrier, Coincident, ComponentShape, ExtentBall, NoCarrier,
    PairSection, SectionDecision, SectionError, SectionRead, SsiLane,
};
use geom_core::{Band, Point3, Tol, Vec3};

fn band() -> Band {
    Band::linear(Tol::witness()).unwrap()
}

fn p(x: f64, y: f64, z: f64) -> Point3<f64> {
    Point3::new(x, y, z)
}

fn cone_about(apex: Point3<f64>, axis: Vec3<f64>, alpha: f64) -> Surface<f64> {
    let axis = axis.normalize();
    Surface::Cone {
        frame: geom::Frame::new(apex, axis, axis.orthonormal_basis().0),
        half_angle: alpha,
    }
}

fn unit_cone() -> Surface<f64> {
    cone_about(Point3::origin(), Vec3::unit_z(), FRAC_PI_4)
}

fn cylinder(origin: Point3<f64>, axis: Vec3<f64>, r: f64) -> Surface<f64> {
    let axis = axis.normalize();
    Surface::Cylinder {
        frame: geom::Frame::new(origin, axis, axis.orthonormal_basis().0),
        radius: r,
    }
}

/// The region every row reads over unless it names its own.
fn region() -> ExtentBall<f64> {
    ExtentBall::fixture(Point3::origin(), 5.0)
}

fn read(
    a: &Surface<f64>,
    b: &Surface<f64>,
    region: ExtentBall<f64>,
) -> Result<SectionRead<f64>, SectionError> {
    geom_brep::pair_section(a, b, Some(region), None::<geom_core::NoLane>, band())
}

fn built(a: &Surface<f64>, b: &Surface<f64>, region: ExtentBall<f64>) -> PairSection<f64> {
    match read(a, b, region) {
        Ok(SectionRead::Built(section)) => section,
        other => panic!("expected the section, got {other:?}"),
    }
}

// -------------------------------------------------------------------
// The traced oracle
// -------------------------------------------------------------------

/// The roots `t` of the partner's equation along `x(t) = b + t·v`, `v`
/// unit, in closed form: against a cylinder `|(x − o) × d|² = r²`,
/// against a cone `((x − k)·d)² = cos²α |x − k|²`.
fn roots_along(partner: &Surface<f64>, b: Point3<f64>, v: Vec3<f64>) -> Vec<f64> {
    let (a2, a1, a0) = match *partner {
        Surface::Cylinder { frame, radius } => {
            let d = frame.axis().normalize();
            let (w, vv) = ((b - frame.origin()).cross(d), v.cross(d));
            (vv.dot(vv), 2.0 * w.dot(vv), w.dot(w) - radius * radius)
        }
        Surface::Cone { frame, half_angle } => {
            let d = frame.axis().normalize();
            let c2 = half_angle.cos().powi(2);
            let w = b - frame.origin();
            let (dw, dv) = (d.dot(w), d.dot(v));
            (
                dv * dv - c2,
                2.0 * (dw * dv - c2 * w.dot(v)),
                dw * dw - c2 * w.dot(w),
            )
        }
        _ => panic!("the oracle reads a cylinder or a cone partner"),
    };
    // A ruling through an apex on the partner, in its tangent plane,
    // has a double root there, which rounding reads a hair below zero:
    // the rows are of unit scale, so `1e-14` is far past that rounding
    // and far inside every discriminant the rows read off zero.
    let disc = a1 * a1 - 4.0 * a2 * a0;
    if disc < -1e-14 {
        return Vec::new();
    }
    let disc = disc.max(0.0);
    let q = -0.5 * (a1 + a1.signum() * disc.sqrt());
    let mut out = Vec::new();
    if a2 != 0.0 {
        out.push(q / a2);
    }
    // `q` at rounding's scale is that double root at the apex, which
    // `q/a2` already places: `a0/q` would read rounding over rounding.
    if q.abs() > 1e-12 {
        out.push(a0 / q);
    }
    out
}

/// The traced section of the cone `cone` against `partner` inside
/// `region`: clusters of points, each a component (or a node-joined set)
/// inside the region, and whether each is the apex alone.
struct Trace {
    points: Vec<Point3<f64>>,
    cluster: Vec<usize>,
    clusters: usize,
    apex_only: Vec<bool>,
}

fn find(parent: &mut [usize], i: usize) -> usize {
    let mut r = i;
    while parent[r] != r {
        r = parent[r];
    }
    let mut j = i;
    while parent[j] != r {
        let next = parent[j];
        parent[j] = r;
        j = next;
    }
    r
}

fn traced(
    cone: &Surface<f64>,
    partner: &Surface<f64>,
    region: ExtentBall<f64>,
    link: f64,
) -> Trace {
    const RULINGS: usize = 40_000;
    let Surface::Cone { frame, half_angle } = *cone else {
        panic!("the oracle rules a cone")
    };
    let (apex, axis) = (frame.origin(), frame.axis().normalize());
    let (n, m) = axis.orthonormal_basis();
    let (s, c) = half_angle.sin_cos();
    let (centre, radius) = (region.center(), region.radius());
    let mut points = Vec::new();
    let mut ruling_of = Vec::new();
    for k in 0..RULINGS {
        #[allow(clippy::cast_precision_loss)]
        let phi = TAU * k as f64 / RULINGS as f64;
        let v = axis * c + (n * phi.cos() + m * phi.sin()) * s;
        for t in roots_along(partner, apex, v) {
            let x = apex + v * t;
            if (x - centre).norm() < radius {
                points.push(x);
                ruling_of.push(k);
            }
        }
    }
    let mut parent: Vec<usize> = (0..points.len()).collect();
    // Points are in ruling order: join each to the next ruling's and its
    // own ruling's, the last ruling to the first.
    let mut by_ruling: Vec<Vec<usize>> = vec![Vec::new(); RULINGS];
    for (i, &k) in ruling_of.iter().enumerate() {
        by_ruling[k].push(i);
    }
    for k in 0..RULINGS {
        let next = (k + 1) % RULINGS;
        for &i in &by_ruling[k] {
            for &j in by_ruling[k].iter().chain(&by_ruling[next]) {
                if i != j && (points[i] - points[j]).norm() <= link {
                    let (ri, rj) = (find(&mut parent, i), find(&mut parent, j));
                    parent[ri] = rj;
                }
            }
        }
    }
    let mut label = vec![usize::MAX; points.len()];
    let mut cluster = vec![0; points.len()];
    let mut clusters = 0;
    for i in 0..points.len() {
        let r = find(&mut parent, i);
        if label[r] == usize::MAX {
            label[r] = clusters;
            clusters += 1;
        }
        cluster[i] = label[r];
    }
    let mut apex_only = vec![true; clusters];
    for (i, x) in points.iter().enumerate() {
        if (*x - apex).norm() > link {
            apex_only[cluster[i]] = false;
        }
    }
    Trace {
        points,
        cluster,
        clusters,
        apex_only,
    }
}

impl Trace {
    /// The cluster a point stands within `link` of, if exactly one.
    fn cluster_at(&self, x: Point3<f64>, link: f64) -> Vec<usize> {
        let mut near: Vec<usize> = self
            .points
            .iter()
            .zip(&self.cluster)
            .filter(|(q, _)| (**q - x).norm() <= link)
            .map(|(_, &c)| c)
            .collect();
        near.sort_unstable();
        near.dedup();
        near
    }

    /// The clusters that are not the apex alone.
    fn components(&self) -> usize {
        self.apex_only.iter().filter(|a| !**a).count()
    }
}

/// **What a row reads**, its shapes counted: loops, open components cut
/// by the region, open components ending at a node, points, and the
/// node-joined sets.
#[derive(Debug, PartialEq, Eq, Default)]
struct Shapes {
    loops: usize,
    cut: usize,
    noded: usize,
    points: usize,
    sets: usize,
}

/// **The row's section, checked against the trace** (module docs): the
/// cone `cone` first and then second, the same shapes both ways round.
fn checked(
    cone: &Surface<f64>,
    partner: &Surface<f64>,
    region: ExtentBall<f64>,
    link: f64,
) -> (Shapes, PairSection<f64>) {
    let trace = traced(cone, partner, region, link);
    let mut seen = None;
    for (a, b) in [(cone, partner), (partner, cone)] {
        let section = built(a, b, region);
        let listed = section.counted().expect("a cone pair's section is counted");
        let mut shapes = Shapes {
            sets: section.node_sets().len(),
            ..Shapes::default()
        };
        let mut clusters = Vec::new();
        let in_set = |i: usize| section.node_sets().iter().any(|s| s.arcs.contains(&i));
        for (i, c) in listed.iter().enumerate() {
            let w = c.witness.expect("every listed component is witnessed");
            for s in [a, b] {
                let r = geom_brep::implicit_residual(s, w);
                assert!(
                    r.abs() <= 1e-9,
                    "witness {w:?} of component {i} is {r:e} off {s:?}"
                );
            }
            assert!(
                (w - region.center()).norm() <= region.radius(),
                "witness {w:?} outside the region"
            );
            match c.shape {
                ComponentShape::Loop => shapes.loops += 1,
                ComponentShape::Point => {
                    shapes.points += 1;
                    continue;
                }
                ComponentShape::Open { ends } => {
                    if ends.nodes.nodes().iter().any(Option::is_some) {
                        shapes.noded += 1;
                    } else {
                        assert!(ends.cut, "an open component with no node is cut");
                        shapes.cut += 1;
                    }
                }
            }
            for x in c.cut_ends.into_iter().flatten() {
                let off = (x - region.center()).norm() - region.radius();
                assert!(
                    off.abs() <= 1e-6,
                    "cut end {x:?} stands {off:e} off the region's boundary"
                );
                for s in [a, b] {
                    let r = geom_brep::implicit_residual(s, x);
                    assert!(r.abs() <= 1e-6, "cut end {x:?} is {r:e} off {s:?}");
                }
            }
            let near = trace.cluster_at(w, link);
            assert_eq!(
                near.len(),
                1,
                "witness {w:?} of component {i} on one traced cluster: {near:?}"
            );
            if !in_set(i) {
                clusters.push(near[0]);
            }
        }
        for set in section.node_sets() {
            let first = listed[set.arcs[0]].witness.unwrap();
            clusters.push(trace.cluster_at(first, link)[0]);
        }
        let distinct = {
            let mut c = clusters.clone();
            c.sort_unstable();
            c.dedup();
            c.len()
        };
        assert_eq!(
            distinct,
            clusters.len(),
            "each component on its own traced cluster: {clusters:?}"
        );
        assert_eq!(
            clusters.len(),
            trace.components(),
            "the components (each node-joined set one) against the traced clusters"
        );
        match &seen {
            None => seen = Some((shapes, section)),
            Some((first, _)) => assert_eq!(&shapes, first, "the same shapes both ways round"),
        }
    }
    seen.unwrap()
}

/// Every non-point component's carrier is the general rung's.
fn general(section: &PairSection<f64>) -> bool {
    section
        .counted()
        .unwrap()
        .iter()
        .filter(|c| c.shape != ComponentShape::Point)
        .all(|c| matches!(c.carrier, ClassCarrier::General))
}

// -------------------------------------------------------------------
// Cone × oblique cylinder
// -------------------------------------------------------------------

/// **A rod biting the cone's side cuts one null loop**: the rod about
/// `y = 1.3, z = 1` along `x`, radius `0.4`, its axis outside the cone.
/// The bite's pose (`topo`'s `a_rod_biting_the_side_cuts_one_null_loop`).
/// A sweep that reads the discriminant's two branch points as anything
/// but one arc: red.
#[test]
fn a_rod_biting_the_side_cuts_one_null_loop() {
    let bite = cylinder(p(0.0, 1.3, 1.0), Vec3::unit_x(), 0.4);
    let (shapes, section) = checked(&unit_cone(), &bite, region(), 0.05);
    assert_eq!(
        shapes,
        Shapes {
            loops: 1,
            ..Shapes::default()
        }
    );
    assert!(general(&section), "the loop is the march's");
    assert!(section.tangency().is_none() && section.nodes().is_empty());
    let loop_ = &section.counted().unwrap()[0];
    assert_eq!(loop_.winds, [false, false], "a null loop claims no winding");
}

/// **A rod through the cone cuts two loops**, the discriminant's four
/// branch points two arcs: the rod about `y = 0, z = 1` along `x`, radius
/// `0.2`.
#[test]
fn a_rod_through_the_cone_cuts_two_loops() {
    let rod = cylinder(p(0.0, 0.0, 1.0), Vec3::unit_x(), 0.2);
    let (shapes, _) = checked(&unit_cone(), &rod, region(), 0.05);
    assert_eq!(
        shapes,
        Shapes {
            loops: 2,
            ..Shapes::default()
        }
    );
}

/// **A fat steep wall about the apex cuts one loop round each nappe**: the
/// axis leans `0.3` rad off `z` through `(0.1, 0, 0)`, radius `1`. The
/// discriminant is positive round the whole turn, so each sheet is one
/// loop, on the nappe its `t` keeps, and each winds about the cone. A
/// sweep reading one sheet, or both on one nappe: red.
#[test]
fn a_fat_steep_wall_cuts_one_loop_round_each_nappe() {
    let lean = 0.3f64;
    let wall = cylinder(
        p(0.1, 0.0, 0.0),
        Vec3::new(lean.sin(), 0.0, lean.cos()),
        1.0,
    );
    let (shapes, section) = checked(&unit_cone(), &wall, region(), 0.05);
    assert_eq!(
        shapes,
        Shapes {
            loops: 2,
            ..Shapes::default()
        }
    );
    let mut nappes: Vec<bool> = section
        .counted()
        .unwrap()
        .iter()
        .map(|c| {
            assert_eq!(c.winds, [true, false], "each loop winds about the cone");
            c.witness.unwrap().z > 0.0
        })
        .collect();
    nappes.sort_unstable();
    assert_eq!(nappes, [false, true], "one loop on each nappe");
}

/// **A rod clear of the cone has an empty section**: about `y = 5, z = 1`
/// along `x`, radius `0.4`. A sweep that reads a component where the
/// discriminant is negative round the turn: red.
#[test]
fn a_rod_clear_of_the_cone_has_no_section() {
    let clear = cylinder(p(0.0, 5.0, 1.0), Vec3::unit_x(), 0.4);
    let (shapes, section) = checked(&unit_cone(), &clear, region(), 0.05);
    assert_eq!(shapes, Shapes::default());
    assert!(section.decided().is_empty(), "nothing decided Zero");
}

/// **A region across the bite cuts its loop into an arc with two cut
/// ends**: the region about `(0, 1.3, 1)` of radius `0.6`, which the
/// loop runs out of. A sweep that lists the loop whole, or loses a cut
/// end: red.
#[test]
fn a_region_across_the_bite_cuts_its_loop_open() {
    let bite = cylinder(p(0.0, 1.3, 1.0), Vec3::unit_x(), 0.4);
    let region = ExtentBall::fixture(p(0.0, 1.3, 1.0), 0.6);
    let (shapes, section) = checked(&unit_cone(), &bite, region, 0.02);
    assert!(
        shapes.cut >= 1 && shapes.loops == 0,
        "the loop cut open: {shapes:?}"
    );
    assert!(
        section
            .counted()
            .unwrap()
            .iter()
            .all(|c| c.cut_ends.is_some()),
        "each arc carries its two cut ends"
    );
}

/// **A rod grazing the cone is decided tangent**: the rod about
/// `y = 0, z = 1` along `x` of radius `1/√2` touches the cone at
/// `(0, ±1/2, 1/2)`, two double roots of the discriminant, which is
/// positive elsewhere: two sheets crossing at each. The tangency is
/// decided `Zero` on the rulings' clearance ceiling, and every component
/// through it mints no carrier. A sweep that escalates the double root,
/// or lists the tangent loops as the march's: red.
#[test]
fn a_rod_grazing_the_cone_is_decided_tangent() {
    let graze = cylinder(p(0.0, 0.0, 1.0), Vec3::unit_x(), FRAC_1_SQRT_2);
    for (a, b) in [(unit_cone(), graze.clone()), (graze.clone(), unit_cone())] {
        let section = built(&a, &b, region());
        let tangency = section.tangency().expect("the graze is decided a tangency");
        assert_eq!(
            tangency.decision,
            SectionDecision::Tangency("ruling_tangency")
        );
        let listed = section.counted().unwrap();
        assert!(!listed.is_empty(), "the tangent section is listed");
        assert!(
            listed
                .iter()
                .all(|c| matches!(c.carrier, ClassCarrier::Unminted(NoCarrier::Tangent))),
            "every component runs through a tangency: {listed:?}"
        );
    }
}

/// **A rod touching the cone from outside at one point is a point
/// component**: the rod along `x` whose wall touches the cone at
/// `(0, 1, 1)`, its axis `0.3` out along the cone's normal there. The
/// discriminant's double root is a maximum, so the touch is an isolated
/// point of the section, and nothing else is. A sweep reading the
/// maximum as an arc: red.
#[test]
fn a_rod_touching_the_cone_from_outside_is_a_point() {
    let at = p(0.0, 1.0, 1.0);
    let normal = Vec3::new(0.0, 1.0, -1.0).normalize();
    let rod = cylinder(at + normal * 0.3, Vec3::unit_x(), 0.3);
    for (a, b) in [(unit_cone(), rod.clone()), (rod.clone(), unit_cone())] {
        let section = built(&a, &b, region());
        assert!(
            section.tangency().is_some(),
            "the touch is decided a tangency"
        );
        let listed = section.counted().unwrap();
        assert_eq!(listed.len(), 1, "one component: {listed:?}");
        assert_eq!(listed[0].shape, ComponentShape::Point);
        let w = listed[0].witness.unwrap();
        assert!((w - at).norm() < 1e-4, "the touch at {w:?}");
    }
}

/// **A wall through the apex, the cone's form definite on its tangent
/// plane there, is an apex touch and one loop**: the rod about
/// `y = 0, z = 0.2` along `x`, radius `0.2`, its wall through the apex
/// with tangent plane `z = 0`, which meets the cone at the apex alone.
/// The apex is decided on the wall (a coincidence, not a tangency), the
/// apex root divided out, and the one sheet left is a loop round the
/// upper nappe. A pose reading the apex `Zero` as a tangency, or the
/// sweep leaving the apex root in: red.
#[test]
fn a_wall_through_the_apex_touching_it_is_an_apex_touch() {
    let rod = cylinder(p(0.0, 0.0, 0.2), Vec3::unit_x(), 0.2);
    let (shapes, section) = checked(&unit_cone(), &rod, region(), 0.02);
    assert_eq!(
        shapes,
        Shapes {
            loops: 1,
            points: 1,
            ..Shapes::default()
        }
    );
    assert!(
        section.tangency().is_none(),
        "the apex on the wall is no tangency"
    );
    assert_eq!(
        section.coincidence().map(|(c, _)| c),
        Some(Coincident::ConeApex),
        "the apex on the wall is recorded"
    );
    let point = section
        .counted()
        .unwrap()
        .iter()
        .find(|c| c.shape == ComponentShape::Point)
        .unwrap();
    assert!(matches!(
        point.carrier,
        ClassCarrier::Unminted(NoCarrier::ApexTouch)
    ));
    assert_eq!(
        (point.witness.unwrap() - Point3::origin()).norm(),
        0.0,
        "at the apex"
    );
    assert!(section.nodes().is_empty(), "a touch is no node");
}

/// **A wall through the apex, the cone's form indefinite on its tangent
/// plane, is a node at the apex**: the wall of radius `0.5` about the
/// line through `(0.5, 0, 0)` along `y`, tangent plane `x = 0` at the
/// apex, which holds the rulings `y = ±z`. The divided sheet passes
/// through the apex twice, so the section is two arcs, one per nappe,
/// each from the node to the node, and one node-joined set; the node's
/// lines are those two rulings. A pose reading the form definite, or a
/// sweep that does not end its arcs at the node: red.
#[test]
fn a_wall_through_the_apex_crossing_it_is_a_node() {
    let wall = cylinder(p(0.5, 0.0, 0.0), Vec3::unit_y(), 0.5);
    let (shapes, section) = checked(&unit_cone(), &wall, region(), 0.02);
    assert_eq!(
        shapes,
        Shapes {
            noded: 2,
            sets: 1,
            ..Shapes::default()
        }
    );
    let [node] = section.nodes() else {
        panic!("one node: {:?}", section.nodes())
    };
    assert_eq!(
        (node.point - Point3::origin()).norm(),
        0.0,
        "the node is the apex"
    );
    assert!(
        (node.normal.x.abs() - 1.0).abs() < 1e-12,
        "the wall's normal there, ±x"
    );
    for l in node.lines {
        assert!(
            l.x.abs() < 1e-9 && (l.y.abs() - l.z.abs()).abs() < 1e-9,
            "a ruling y = ±z: {l:?}"
        );
    }
    assert!(
        node.lines[0].cross(node.lines[1]).dot(node.normal) > 0.0,
        "the second line turned counterclockwise from the first"
    );
    for c in section.counted().unwrap() {
        let ComponentShape::Open { ends } = c.shape else {
            panic!("an arc: {c:?}")
        };
        assert_eq!(
            ends.nodes.nodes(),
            [Some(0), Some(0)],
            "each arc runs from the node to the node"
        );
        assert!(!ends.cut, "the region holds each arc whole");
        // Each end's ray points along its arc's nappe.
        let up = c.witness.unwrap().z > 0.0;
        for e in ends.nodes.both().into_iter().flatten() {
            assert_eq!(
                node.ray(e.line, e.sense).z > 0.0,
                up,
                "the arc leaves along its nappe"
            );
        }
    }
}

// -------------------------------------------------------------------
// Cone × cone
// -------------------------------------------------------------------

/// **A narrow cone through the side cuts two loops**: apex `(−5, 0, 2)`,
/// axis `x`, `α = 0.2`, its directions all off the unit cone's, so
/// nothing runs past every bound.
#[test]
fn a_narrow_cone_through_the_side_cuts_two_loops() {
    let narrow = cone_about(p(-5.0, 0.0, 2.0), Vec3::unit_x(), 0.2);
    let region = ExtentBall::fixture(Point3::origin(), 8.0);
    let (shapes, section) = checked(&unit_cone(), &narrow, region, 0.05);
    assert_eq!(
        shapes,
        Shapes {
            loops: 2,
            ..Shapes::default()
        }
    );
    assert!(general(&section));
}

/// **A tilted cone sharing asymptotic directions cuts unbounded branches
/// open at the region**: apex `(3, 0, 0)`, axis `z` turned `1` rad about
/// `y`, `α = π/4`. Where a ruling runs along the partner's asymptotic
/// directions its sheet leaves every bound, so that sheet is two open
/// components, each cut twice by the region; the other sheet is a loop
/// round the unit cone. A sweep that loses the asymptote (one sheet read
/// as a loop), or fails to place a cut end past it: red.
#[test]
fn a_tilted_cone_sharing_asymptotes_cuts_its_unbounded_branches_open() {
    let tilted = cone_about(
        p(3.0, 0.0, 0.0),
        Vec3::new(1f64.sin(), 0.0, 1f64.cos()),
        FRAC_PI_4,
    );
    let region = ExtentBall::fixture(Point3::origin(), 12.0);
    let (shapes, section) = checked(&unit_cone(), &tilted, region, 0.1);
    assert_eq!(
        shapes,
        Shapes {
            loops: 1,
            cut: 2,
            ..Shapes::default()
        }
    );
    for c in section.counted().unwrap() {
        if c.shape == ComponentShape::Loop {
            assert_eq!(c.winds[0], true, "the loop winds about the unit cone");
        } else {
            assert!(c.cut_ends.is_some(), "an unbounded branch has two cut ends");
        }
    }
}

/// **A cone whose apex lies on the unit cone is ruled from that apex**:
/// apex `(1, 0, 1)`, on the unit cone's ruling there, axis back toward
/// the origin, `α = 0.3`. The unit cone's apex stands on the partner's
/// axis, off its wall, so the partner is the side ruled, its apex root
/// divided out; the unit cone's tangent plane there holds the partner's
/// axis, so its form is indefinite on it and the apex is a node. A pose
/// that rules the unit cone (whose sweep meets a double root at the
/// partner's apex): red.
#[test]
fn a_cone_with_its_apex_on_the_other_is_ruled_from_that_apex() {
    let facing = cone_about(p(1.0, 0.0, 1.0), Vec3::new(-1.0, 0.0, -1.0), 0.3);
    let (shapes, section) = checked(&facing, &unit_cone(), region(), 0.02);
    assert_eq!(shapes.sets, 1, "one node-joined set: {shapes:?}");
    assert!(shapes.noded >= 2, "arcs end at the node: {shapes:?}");
    let [node] = section.nodes() else {
        panic!("one node: {:?}", section.nodes())
    };
    assert!(
        (node.point - p(1.0, 0.0, 1.0)).norm() < 1e-12,
        "the node is the partner's apex"
    );
}

/// **Two cones through one apex are not built here**: their section is
/// their shared rulings, read where it was read before.
#[test]
fn two_cones_through_one_apex_are_read_elsewhere() {
    let other = cone_about(Point3::origin(), Vec3::new(0.2, 1.0, 0.3), 0.4);
    assert!(matches!(
        read(&unit_cone(), &other, region()),
        Ok(SectionRead::Elsewhere(_))
    ));
    let twin = cone_about(p(1.0, 0.0, 0.0), Vec3::unit_z(), FRAC_PI_4);
    assert!(
        matches!(
            read(&unit_cone(), &twin, region()),
            Ok(SectionRead::Elsewhere(_))
        ),
        "parallel axes of one aperture: the plane conic"
    );
}

// -------------------------------------------------------------------
// Refusals and the carry
// -------------------------------------------------------------------

/// **The degenerate poses refuse typed**: no region, and a generator
/// along the cylinder's axis.
#[test]
fn the_degenerate_poses_refuse_typed() {
    let rod = cylinder(p(0.0, 1.3, 1.0), Vec3::unit_x(), 0.4);
    assert!(matches!(
        geom_brep::pair_section(&unit_cone(), &rod, None, None::<geom_core::NoLane>, band()),
        Err(SectionError::NoRegion { .. })
    ));
    let along = cylinder(p(0.5, 0.0, 0.0), Vec3::new(1.0, 0.0, 1.0), 0.3);
    assert!(matches!(
        read(&unit_cone(), &along, region()),
        Err(SectionError::Pinched {
            predicate: "kc_generator_along_axis"
        })
    ));
}

/// **Carrying a cone pair's section refuses typed**: its components are
/// the march's, which needs the cone's ℝ³ enclosures; at `f64` with a
/// march door the refusal names them, without one the lane.
#[test]
fn carrying_refuses_on_the_cones_enclosures() {
    let bite = cylinder(p(0.0, 1.3, 1.0), Vec3::unit_x(), 0.4);
    let section = built(&unit_cone(), &bite, region());
    assert!(matches!(
        section.carry(Some(SsiLane::marched()), band()),
        Err(CarryError::Refused(CarryRefusal::Unminted(
            NoCarrier::ConeUnenclosed
        )))
    ));
    assert!(matches!(
        section.carry(None::<geom_core::NoLane>, band()),
        Err(CarryError::Refused(CarryRefusal::NoLane { .. }))
    ));
}

/// **The value decides at `Interval` as it does at `f64`**: the bite's
/// one loop, read on certified enclosures.
#[test]
fn the_bite_reads_one_loop_at_interval() {
    use geom_core::{Interval, Real as _};
    let lift = |s: &Surface<f64>| s.map_scalar(Interval::from_computed);
    let bite = cylinder(p(0.0, 1.3, 1.0), Vec3::unit_x(), 0.4);
    let region = ExtentBall::fixture(
        Point3::origin().map(Interval::from_computed),
        Interval::from_computed(5.0),
    );
    let got = geom_brep::pair_section(
        &lift(&unit_cone()),
        &lift(&bite),
        Some(region),
        None::<geom_core::NoLane>,
        band(),
    );
    let Ok(SectionRead::Built(section)) = got else {
        panic!("the bite at Interval: {got:?}")
    };
    let listed = section.counted().unwrap();
    assert_eq!(listed.len(), 1);
    assert_eq!(listed[0].shape, ComponentShape::Loop);
}
