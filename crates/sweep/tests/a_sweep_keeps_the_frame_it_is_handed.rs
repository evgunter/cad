//! **A sweep keeps the frame it is handed.** A door is handed its
//! sketch plane as a rigid frame, and every direction it stores is that
//! frame turning a direction decided once in the sketch: a cap is its
//! station's frame, a line wall's normal is the frame turning its
//! segment's right normal, an arc wall's and a rim circle's `u_ref` the
//! frame turning the arc's radial, a strut the sweep's direction, and a
//! revolve's latitude circle the revolve's seam direction. Nothing is
//! fitted to the vertices the door placed, so over a parameter box a
//! datum the geometry holds fixed is stored fixed.
//!
//! Every row builds at `Sym<Interval>`, the scalar a drive replays a
//! leaf in: at a bare `Interval` a boxed input does not build at all,
//! since a vertex and the carrier through it are enclosed apart. A box
//! on a depth or on a frame's angle is macroscopic; a box on a profile
//! vertex is as narrow as a drive's leaves, which is where the profile's
//! own validation decides.
//!
//! Two kinds of row. The first holds the guarantee where the truth is
//! fixed across the box (a depth, a section's size, a profile's width,
//! a vertex's radius): the stored datum is exact to rounding, and a
//! cap's origin carries the depth's width and no other. The second
//! holds that a frame-derived datum is never narrower than the truth
//! where the truth varies (a sketch plane turned by a boxed angle, a
//! profile edge whose direction a boxed vertex sets, an arc whose
//! radial a boxed bulge sets): every member's `f64` build stores a
//! datum the box's enclosure contains.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::FRAC_PI_2;
use core::ops::Deref;

use geom::Surface;
use geom_core::sym::with_session;
use geom_core::{
    Bounds, Decide, Interval, ParamSymbol, Point2, Point3, Real, RigidMotion, Sym, SymBudget, Tol,
    UnitVec3, Vec2, Vec3,
};
use profile::test_support::bulge_loop;
use profile::{Profile, ProfileLoop, RawLoop, SketchPlane};
use sweep::{
    ExtrudeSide, Extruded, Extrusion, Revolution, RevolveAxis, RevolvedKind, extrude, loft_body,
    revolve,
};
use topo::{Body, FaceKey};

fn tol() -> Tol {
    Tol::witness()
}

/// The box scalar.
type S = Sym<Interval>;

/// A parameter boxed to `[lo, hi]`.
fn boxed(id: u64, lo: f64, hi: f64) -> S {
    Sym::param(ParamSymbol::new(id), Interval::from_bounds(lo, hi))
}

/// A literal at the box scalar.
fn lit(x: f64) -> S {
    S::from_f64(x)
}

/// `f` inside a symbolic session.
fn in_session(f: impl FnOnce()) {
    let budget = SymBudget {
        max_terms: 4096,
        max_degree: 128,
    };
    with_session(budget, f);
}

/// Rounding, and nothing a box could add: a few ulps at unit scale.
const ROUNDING: f64 = 1e-14;

/// The half-width of a box on a profile vertex: a box the profile's
/// validation decides over (a drive bisects to such boxes), and three
/// orders past rounding, so a datum re-derived from boxed vertices reads
/// its width.
fn vertex_box() -> f64 {
    tol().eps() / 64.0
}

/// A parameter boxed to `centre ± vertex_box()`.
fn about(id: u64, centre: f64) -> S {
    boxed(id, centre - vertex_box(), centre + vertex_box())
}

fn width<T: Bounds>(x: T) -> f64 {
    x.hi() - x.lo()
}

/// Every component of `v` is exact to rounding.
fn assert_exact<T: Bounds>(what: &str, v: Vec3<T>) {
    for (axis, c) in [("x", v.x), ("y", v.y), ("z", v.z)] {
        assert!(
            width(c) <= ROUNDING,
            "{what}.{axis} is exact to rounding: [{}, {}]",
            c.lo(),
            c.hi()
        );
    }
}

/// `v` is a unit vector to rounding: exact, and of length 1.
fn assert_exact_unit<T: Bounds>(what: &str, v: Vec3<T>) {
    assert_exact(what, v);
    let n = v.norm();
    assert!(
        n.lo() >= 1.0 - ROUNDING && n.hi() <= 1.0 + ROUNDING,
        "{what} has length 1: [{}, {}]",
        n.lo(),
        n.hi()
    );
}

/// A face's plane: origin, normal, `u_ref`.
fn plane<T: Real>(body: &Body<T>, face: FaceKey) -> (Point3<T>, Vec3<T>, Vec3<T>) {
    match body.face_surface(face).expect("a live face") {
        Surface::Plane {
            origin,
            normal,
            u_ref,
        } => (*origin, *normal, *u_ref),
        other => panic!("a plane, got {other:?}"),
    }
}

fn p<T: Real>(x: f64, y: f64) -> Point2<T> {
    Point2::new(T::from_f64(x), T::from_f64(y))
}

/// The unit square, with corner `(1, 1)` at `(1, h)` and the edge
/// leaving it bulged by `bulge` (zero: a line).
fn quad<T: Real>(h: T, bulge: T) -> ProfileLoop<T> {
    let z = T::zero;
    bulge_loop(vec![
        (p(0.0, 0.0), z()),
        (p(1.0, 0.0), z()),
        (Point2::new(T::one(), h), bulge),
        (p(0.0, 1.0), z()),
    ])
}

fn slab<T: Decide + topo::AtRestPolicy>(
    plane: SketchPlane<T>,
    lp: ProfileLoop<T>,
    depth: T,
) -> Extruded<T> {
    let profile = Profile::new(plane, vec![lp]).validate(tol()).unwrap();
    let side = ExtrudeSide::Along;
    extrude(&profile, Extrusion::Distance { depth, side }, tol()).expect("the slab extrudes")
}

/// **A slab whose depth carries a box stores exact caps and walls**: the
/// caps' normals and `u_ref`s, the walls' normals, `u_ref`s and origins
/// are exact to rounding, and the top cap's origin carries the depth's
/// width on its normal coordinate and nothing on the others.
fn the_slab_is_exact<T: Decide + topo::AtRestPolicy + Bounds>(lane: &str, depth: T)
where
    T::Leaf: Deref<Target = Body<T>>,
{
    let built = slab(SketchPlane::xy(), quad(T::one(), T::zero()), depth);
    let body: &Body<T> = &built.body;
    for (name, face) in [("top", built.top), ("bottom", built.bottom)] {
        let (origin, normal, u_ref) = plane(body, face);
        assert_exact_unit(&format!("{lane} {name} cap normal"), normal);
        assert_exact_unit(&format!("{lane} {name} cap u_ref"), u_ref);
        assert!(
            width(origin.x) <= ROUNDING && width(origin.y) <= ROUNDING,
            "{lane} {name} cap origin is exact in the plane: {origin:?}"
        );
        let expect = if name == "top" { width(depth) } else { 0.0 };
        assert!(
            (width(origin.z) - expect).abs() <= ROUNDING,
            "{lane} {name} cap origin carries the depth's width ({expect}) and no other: [{}, {}]",
            origin.z.lo(),
            origin.z.hi()
        );
    }
    for wall in &built.walls[0] {
        let (origin, normal, u_ref) = plane(body, wall.face);
        assert_exact_unit(&format!("{lane} wall {:?} normal", wall.face), normal);
        assert_exact_unit(&format!("{lane} wall {:?} u_ref", wall.face), u_ref);
        assert_exact(
            &format!("{lane} wall {:?} origin", wall.face),
            origin - Point3::origin(),
        );
    }
}

#[test]
fn a_slab_over_a_depth_box_stores_exact_caps_and_walls() {
    in_session(|| the_slab_is_exact("Sym<Interval>", boxed(0x5f1a, 0.95, 1.05)));
}

/// A square of half-width `a` about the sketch origin.
fn square<T: Real>(a: T) -> ProfileLoop<T> {
    let z = T::zero;
    let m = T::zero() - a;
    bulge_loop(vec![
        (Point2::new(m, m), z()),
        (Point2::new(a, m), z()),
        (Point2::new(a, a), z()),
        (Point2::new(m, a), z()),
    ])
}

/// **A loft's caps are its end sections' frames**: over a box on the
/// sections' size, both caps' normals and `u_ref`s are exact and their
/// origins are the planes' origins.
#[test]
fn a_lofts_caps_over_a_section_box_are_its_end_frames() {
    in_session(|| {
        let a = about(0x10f7, 1.0);
        let places = [
            SketchPlane::<S>::xy(),
            SketchPlane::xy_at(Point3::new(lit(0.0), lit(0.0), lit(1.0))),
        ];
        let lofted = loft_body(&[vec![square(a)], vec![square(a)]], &places, 1, tol())
            .expect("the boxed prism lofts");
        for (name, face, z) in [("bottom", lofted.bottom, 0.0), ("top", lofted.top, 1.0)] {
            let (origin, normal, u_ref) = plane(&lofted.body, face);
            assert_exact_unit(&format!("loft {name} cap normal"), normal);
            assert_exact_unit(&format!("loft {name} cap u_ref"), u_ref);
            assert_exact(
                &format!("loft {name} cap origin"),
                origin - Point3::origin(),
            );
            assert!(
                origin.z.lo() <= z && z <= origin.z.hi(),
                "loft {name} cap origin is its plane's"
            );
        }
    });
}

/// The rectangle `[1, 1 + a] × [0, 1]`.
fn band<T: Real>(a: T) -> ProfileLoop<T> {
    let z = T::zero;
    let far = T::one() + a;
    bulge_loop(vec![
        (p(1.0, 0.0), z()),
        (Point2::new(far, T::zero()), z()),
        (Point2::new(far, T::one()), z()),
        (p(1.0, 1.0), z()),
    ])
}

/// The sketch's `y` axis.
fn y_axis<T: Real>() -> RevolveAxis<T> {
    RevolveAxis {
        origin: p(0.0, 0.0),
        dir: Vec2::new(T::zero(), T::one()),
    }
}

/// **A partial revolve's caps are its stations' frames, and its
/// latitude circles aim along its seam**: over a box on the profile's
/// width, both caps' normals and `u_ref`s are exact to rounding (the end
/// station's turned by a quarter turn), and every latitude circle's
/// `u_ref` is a unit vector exact to rounding.
#[test]
fn a_partial_revolves_caps_and_latitudes_over_a_width_box_are_exact() {
    in_session(|| {
        let profile = Profile::new(SketchPlane::<S>::xy(), vec![band(about(0x2e1, 0.5))])
            .validate(tol())
            .unwrap();
        let revolved = revolve(
            &profile,
            y_axis(),
            Revolution::Partial(lit(FRAC_PI_2)),
            tol(),
        )
        .expect("the boxed band revolves a quarter turn");
        let RevolvedKind::Partial {
            start_cap, end_cap, ..
        } = revolved.kind
        else {
            panic!("a partial revolve has caps")
        };
        for (name, face) in [("start", start_cap), ("end", end_cap)] {
            let (_, normal, u_ref) = plane(&revolved.body, face);
            assert_exact_unit(&format!("revolve {name} cap normal"), normal);
            assert_exact_unit(&format!("revolve {name} cap u_ref"), u_ref);
        }
        assert_latitudes_exact("partial revolve", &revolved.body, &revolved.rims);
    });
}

/// **A full revolve's latitude circles aim along its seam**, over the
/// same box.
#[test]
fn a_full_revolves_latitudes_over_a_width_box_are_exact() {
    in_session(|| {
        let profile = Profile::new(SketchPlane::<S>::xy(), vec![band(about(0x2e2, 0.5))])
            .validate(tol())
            .unwrap();
        let revolved = revolve(&profile, y_axis(), Revolution::Full, tol())
            .expect("the boxed band revolves whole");
        assert_latitudes_exact("full revolve", &revolved.body, &revolved.rims);
    });
}

/// **A circle revolved part way has a latitude at its one vertex**,
/// whose `u_ref` is the seam turned to the far station: exact to
/// rounding over a box on the circle's centre.
#[test]
fn a_revolved_circles_latitude_over_a_centre_box_is_exact() {
    in_session(|| {
        let c = about(0x2e3, 2.0);
        let circle: ProfileLoop<S> = RawLoop::new([(
            Point2::new(c + lit(0.5), lit(0.0)),
            profile::Segment::Arc(geom_core::Arc2 {
                centre: Point2::new(c, lit(0.0)),
                radius: lit(0.5),
                sweep: lit(core::f64::consts::TAU),
            }),
        )]);
        let profile = Profile::new(SketchPlane::<S>::xy(), vec![circle])
            .validate(tol())
            .unwrap();
        let revolved = revolve(
            &profile,
            y_axis(),
            Revolution::Partial(lit(FRAC_PI_2)),
            tol(),
        )
        .expect("the boxed circle revolves a quarter turn");
        assert_latitudes_exact("revolved circle", &revolved.body, &revolved.rims);
    });
}

fn assert_latitudes_exact(what: &str, body: &Body<S>, rims: &[Vec<Option<topo::EdgeKey>>]) {
    let mut seen = 0;
    for rim in rims.iter().flatten().flatten() {
        let carrier = topo::readback::edge_carrier_ref(body, *rim).expect("a carried rim");
        let circle = carrier.as_circle().expect("a latitude is a circle");
        assert_exact_unit(
            &format!("{what} latitude {rim:?} u_ref"),
            circle.frame.u_ref,
        );
        seen += 1;
    }
    assert!(seen > 0, "{what} has latitude circles");
}

/// Samples across `[lo, hi]`: both ends and three points between.
fn members(lo: f64, hi: f64) -> [f64; 5] {
    [0.0, 0.25, 0.5, 0.75, 1.0].map(|s| lo + (hi - lo) * s)
}

/// `member` lies in `held`, componentwise, to rounding.
fn assert_contains(what: &str, held: Vec3<S>, member: Vec3<f64>) {
    for (axis, h, m) in [
        ("x", held.x, member.x),
        ("y", held.y, member.y),
        ("z", held.z, member.z),
    ] {
        assert!(
            h.lo() - ROUNDING <= m && m <= h.hi() + ROUNDING,
            "{what}.{axis}: the member's {m} lies in the stored [{}, {}]",
            h.lo(),
            h.hi()
        );
    }
}

/// The stored datum of a face, at either scalar: a plane's normal and
/// `u_ref`, or a cylinder's axis and `u_ref`.
fn datum<T: Real>(body: &Body<T>, face: FaceKey) -> [Vec3<T>; 2] {
    match body.face_surface(face).expect("a live face") {
        Surface::Plane { normal, u_ref, .. } => [*normal, *u_ref],
        Surface::Cylinder { axis, u_ref, .. } => [*axis, *u_ref],
        other => panic!("a plane or a cylinder, got {other:?}"),
    }
}

/// The faces of an extrusion: top, bottom, then the walls in order.
fn faces<T: topo::AtRestPolicy>(e: &Extruded<T>) -> Vec<FaceKey> {
    [e.top, e.bottom]
        .into_iter()
        .chain(e.walls[0].iter().map(|w| w.face))
        .collect()
}

/// Every face of `boxed` (top, bottom, walls in order) against the same
/// face of each member's `f64` build: the box's enclosure contains every
/// member's datum, and, where `varies`, somewhere the truth varies by
/// more than the narrowest box here, which the enclosure is at least as
/// wide as.
fn assert_holds_its_members(
    what: &str,
    boxed: &Extruded<S>,
    members: impl IntoIterator<Item = Extruded<f64>>,
    varies: bool,
) {
    let held: Vec<[Vec3<S>; 2]> = faces(boxed)
        .into_iter()
        .map(|f| datum(&boxed.body, f))
        .collect();
    let mut spread = 0.0_f64;
    let mut first: Option<Vec<[Vec3<f64>; 2]>> = None;
    for member in members {
        let got: Vec<[Vec3<f64>; 2]> = faces(&member)
            .into_iter()
            .map(|f| datum(&member.body, f))
            .collect();
        assert_eq!(got.len(), held.len(), "{what}: one face per face");
        for (i, (h, m)) in held.iter().zip(&got).enumerate() {
            assert_contains(&format!("{what} face {i} normal"), h[0], m[0]);
            assert_contains(&format!("{what} face {i} u_ref"), h[1], m[1]);
        }
        if let Some(f) = &first {
            for (a, b) in f.iter().zip(&got) {
                spread = spread.max((a[0] - b[0]).norm()).max((a[1] - b[1]).norm());
            }
        } else {
            first = Some(got);
        }
    }
    assert!(
        !varies || spread > vertex_box(),
        "{what}: the truth varies across the box ({spread:e}), so the row reads a width"
    );
}

fn turned<T: Real>(angle: T) -> SketchPlane<T> {
    SketchPlane::from_frame(RigidMotion::rotation(UnitVec3::exact_x(), angle).unwrap())
}

/// **A sketch plane turned by a boxed angle**: every face's direction
/// turns with it, and the stored enclosure holds every member's.
#[test]
fn a_frame_turned_by_a_boxed_angle_stores_every_members_direction() {
    let (lo, hi) = (0.3, 0.32);
    let members = members(lo, hi).map(|angle| slab(turned(angle), quad(1.0, 0.0), 1.0));
    in_session(|| {
        let boxed_slab = slab(
            turned(boxed(0x7a1, lo, hi)),
            quad(lit(1.0), lit(0.0)),
            lit(1.0),
        );
        assert_holds_its_members("a turned plane", &boxed_slab, members, true);
    });
}

/// **A profile edge whose direction a boxed vertex sets**: the wall's
/// normal and chart direction vary with it, and the stored enclosure
/// holds every member's.
#[test]
fn a_wall_whose_edge_turns_with_a_boxed_vertex_stores_every_members_direction() {
    let (lo, hi) = (1.0 - vertex_box(), 1.0 + vertex_box());
    let members = members(lo, hi).map(|h| slab(SketchPlane::xy(), quad(h, 0.0), 1.0));
    in_session(|| {
        let boxed_slab = slab(
            SketchPlane::xy(),
            quad(boxed(0x7a2, lo, hi), lit(0.0)),
            lit(1.0),
        );
        assert_holds_its_members("a turned edge", &boxed_slab, members, true);
    });
}

/// **An arc whose radial a boxed bulge sets**: the cylinder's `u_ref`
/// varies with it, and the stored enclosure holds every member's.
#[test]
fn an_arc_wall_whose_radial_turns_with_a_boxed_bulge_stores_every_members_direction() {
    let (lo, hi) = (0.25 - vertex_box(), 0.25 + vertex_box());
    let members = members(lo, hi).map(|b| slab(SketchPlane::xy(), quad(1.0, b), 1.0));
    in_session(|| {
        let boxed_slab = slab(
            SketchPlane::xy(),
            quad(lit(1.0), boxed(0x7a3, lo, hi)),
            lit(1.0),
        );
        assert_holds_its_members("a turned arc", &boxed_slab, members, true);
    });
}

/// The rectangle `[0, w] × [0, 1]`.
fn rect<T: Real>(w: T) -> ProfileLoop<T> {
    let z = T::zero;
    bulge_loop(vec![
        (p(0.0, 0.0), z()),
        (Point2::new(w, T::zero()), z()),
        (Point2::new(w, T::one()), z()),
        (p(0.0, 1.0), z()),
    ])
}

/// **A slab whose width carries a box**: the caps and the wall at
/// `x = 0`, whose ends the box does not move, store exact unit normals,
/// from the frame. The other three walls each have an end that carries
/// the box, and take their sketch directions from their ends, so they
/// store the box's width in the sketch plane and none across it; the
/// enclosure holds every member's normal. The profile recording its segments' directions is what would
/// make it exact (`the-profile-drops-the-unit-directions-its-verbs-made`).
/// The box is a leaf's, as for every profile-vertex box here.
#[test]
fn a_slab_over_a_width_box_stores_its_fixed_faces_exact_and_holds_the_moving_walls() {
    let (lo, hi) = (1.0 - vertex_box(), 1.0 + vertex_box());
    let members = members(lo, hi).map(|w| slab(SketchPlane::xy(), rect(w), 1.0));
    in_session(|| {
        let boxed_slab = slab(SketchPlane::xy(), rect(boxed(0x3a1, lo, hi)), lit(1.0));
        let body: &Body<S> = &boxed_slab.body;
        for (name, face) in [("top", boxed_slab.top), ("bottom", boxed_slab.bottom)] {
            let (_, normal, u_ref) = plane(body, face);
            assert_exact_unit(&format!("width box {name} cap normal"), normal);
            assert_exact_unit(&format!("width box {name} cap u_ref"), u_ref);
        }
        let walls = &boxed_slab.walls[0];
        let (_, fixed, _) = plane(body, walls[3].face);
        assert_exact_unit("width box wall x = 0 normal", fixed);
        for wall in &walls[..3] {
            let (_, moving, _) = plane(body, wall.face);
            assert!(
                width(moving.z) <= ROUNDING,
                "a wall the box moves takes no width across the sketch plane: {moving:?}"
            );
        }
        assert_holds_its_members("a width box", &boxed_slab, members, false);
    });
}
