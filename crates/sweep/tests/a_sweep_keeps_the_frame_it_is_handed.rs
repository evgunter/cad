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
        Surface::Plane { frame } => {
            let (origin, normal, u_ref) = (frame.origin(), frame.axis(), frame.u_ref());
            (origin, normal, u_ref)
        }
        other => panic!("a plane, got {other:?}"),
    }
}

fn p<T: Real>(x: f64, y: f64) -> Point2<T> {
    Point2::new(T::from_f64(x), T::from_f64(y))
}

/// The unit square, with corner `(1, 1)` at `(1, h)` and the edge
/// leaving it bulged by `bulge` (zero: a line).
fn quad<T: geom_core::Decide>(h: T, bulge: T) -> ProfileLoop<T> {
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
/// caps' normals and `u_ref`s and the walls' normals and `u_ref`s are
/// exact to rounding. The top cap's origin carries the depth's width on
/// its normal coordinate and nothing on the others; a wall's origin, the
/// middle of the wall, carries half of it on the same coordinate (half
/// the sweep) and nothing on the others.
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
        assert!(
            width(origin.x) <= ROUNDING && width(origin.y) <= ROUNDING,
            "{lane} wall {:?} origin is exact across the sweep: {origin:?}",
            wall.face
        );
        assert!(
            (width(origin.z) - 0.5 * width(depth)).abs() <= ROUNDING,
            "{lane} wall {:?} origin, the wall's middle, carries half the depth's width: [{}, {}]",
            wall.face,
            origin.z.lo(),
            origin.z.hi()
        );
    }
}

#[test]
fn a_slab_over_a_depth_box_stores_exact_caps_and_walls() {
    in_session(|| the_slab_is_exact("Sym<Interval>", boxed(0x5f1a, 0.95, 1.05)));
}

/// A square of half-width `a` about the sketch origin.
fn square<T: geom_core::Decide>(a: T) -> ProfileLoop<T> {
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
fn band<T: geom_core::Decide>(a: T) -> ProfileLoop<T> {
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
        let carrier = body
            .edge(*rim)
            .and_then(topo::EdgeView::certified)
            .expect("a carried rim")
            .carrier();
        let circle = carrier.as_circle().expect("a latitude is a circle");
        assert_exact_unit(
            &format!("{what} latitude {rim:?} u_ref"),
            circle.frame().u_ref(),
        );
        seen += 1;
    }
    assert!(seen > 0, "{what} has latitude circles");
}

/// Samples across `[lo, hi]`: both ends and three points between.
fn members(lo: f64, hi: f64) -> [f64; 5] {
    [0.0, 0.25, 0.5, 0.75, 1.0].map(|s| lo + (hi - lo) * s)
}

/// The stored datum of a face, at either scalar: a plane's normal and
/// `u_ref`, or a cylinder's axis and `u_ref`.
fn datum<T: Real>(body: &Body<T>, face: FaceKey) -> [Vec3<T>; 2] {
    match body.face_surface(face).expect("a live face") {
        Surface::Plane { frame, .. } => {
            let (normal, u_ref) = (frame.axis(), frame.u_ref());
            [normal, u_ref]
        }
        Surface::Cylinder { frame, .. } => {
            let (axis, u_ref) = (frame.axis(), frame.u_ref());
            [axis, u_ref]
        }
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
/// member's datum to a few ulps ([`assert_holds`]). Where the truth
/// varies (`tight` is set), somewhere it varies by more than the
/// narrowest box here, and every enclosure is no wider than the truth
/// needs plus today's named width ([`assert_tight`]), so the row goes red
/// when an enclosure degrades as well as when it stops containing.
fn assert_holds_its_members(
    what: &str,
    boxed: &Extruded<S>,
    members: impl IntoIterator<Item = Extruded<f64>>,
    tight: Option<(f64, f64)>,
) {
    let varies = tight.is_some();
    let held: Vec<[Vec3<S>; 2]> = faces(boxed)
        .into_iter()
        .map(|f| datum(&boxed.body, f))
        .collect();
    let mut spread = 0.0_f64;
    let mut first: Option<Vec<[Vec3<f64>; 2]>> = None;
    let mut all: Vec<Vec<[Vec3<f64>; 2]>> = Vec::new();
    for member in members {
        let got: Vec<[Vec3<f64>; 2]> = faces(&member)
            .into_iter()
            .map(|f| datum(&member.body, f))
            .collect();
        assert_eq!(got.len(), held.len(), "{what}: one face per face");
        for (i, (h, m)) in held.iter().zip(&got).enumerate() {
            assert_holds(&format!("{what} face {i} normal"), h[0], m[0]);
            assert_holds(&format!("{what} face {i} u_ref"), h[1], m[1]);
        }
        all.push(got.clone());
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
    if let Some(width_over_truth) = tight {
        for (i, h) in held.iter().enumerate() {
            for (k, name) in [(0, "normal"), (1, "u_ref")] {
                let ms: Vec<Vec3<f64>> = all.iter().map(|m| m[i][k]).collect();
                assert_tight(
                    &format!("{what} face {i} {name}"),
                    h[k],
                    &ms,
                    width_over_truth,
                );
            }
        }
    }
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
        assert_holds_its_members("a turned plane", &boxed_slab, members, Some((0.01, 1.25)));
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
        assert_holds_its_members(
            "a turned edge",
            &boxed_slab,
            members,
            Some((vertex_box(), 4.25)),
        );
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
        assert_holds_its_members(
            "a turned arc",
            &boxed_slab,
            members,
            Some((vertex_box(), 11.0)),
        );
    });
}

/// The rectangle `[0, w] × [0, 1]`.
fn rect<T: geom_core::Decide>(w: T) -> ProfileLoop<T> {
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
/// enclosure holds every member's normal. The profile records each
/// side's director, minted as the axis here, and a door that read it
/// would store these walls exact; the doors do not yet
/// (`work/boxfinish/a-door-reading-the-minted-axis-loses-identities-with-quotient-minted-partners.md`).
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
        assert_holds_its_members("a width box", &boxed_slab, members, None);
    });
}

// ---- The doors' other boxes, and rows that go red when an enclosure
// degrades. ----

/// How far a member may sit outside the stored enclosure: a few ulps
/// of a unit-scale datum. It is below every spread a row reads (the
/// narrowest, a profile-vertex box at ε = 1e-12, is 1.6e-14), so an
/// enclosure that kept only its middle misses the box's ends and goes
/// red.
const CONTAIN: f64 = 4.0 * f64::EPSILON;

/// `member` lies in `held`, componentwise, to [`CONTAIN`].
fn assert_holds(what: &str, held: Vec3<S>, member: Vec3<f64>) {
    for (axis, h, m) in [
        ("x", held.x, member.x),
        ("y", held.y, member.y),
        ("z", held.z, member.z),
    ] {
        assert!(
            h.lo() - CONTAIN <= m && m <= h.hi() + CONTAIN,
            "{what}.{axis}: the member's {m} lies in the stored [{}, {}]",
            h.lo(),
            h.hi()
        );
    }
}

/// How much wider than the truth's spread an enclosure may be before a
/// row calls it degraded, before any width the row names.
const SLACK: f64 = 2.0;

/// One component of a held vector: its name, its enclosure, and how a
/// member's `f64` value reads it.
type Component = (&'static str, S, fn(&Vec3<f64>) -> f64);

/// **The enclosure is no wider than the truth needs, or than today's
/// named width**: componentwise, the stored width is at most [`SLACK`]
/// times the members' spread, plus `width_over_truth.1` half-widths of
/// the box (`width_over_truth.0`), plus rounding. So a row goes red when
/// an enclosure widens, not only when it stops containing.
///
/// The second term is what a row measures today beyond its truth, as a
/// multiple of its box's half-width, which is the same at every ε row.
/// Two mechanisms put it there, neither this unit's:
/// - a sketch plane turned by a boxed angle stores its first axis
///   `[0.99, 1.01]`, though a rotation about `x` fixes `x`
///   (`work/scalar/a-rotation-about-an-exact-axis-widens-the-axis-it-turns-about.md`):
///   1.22 half-widths on the turned plane, 0.44 on the tilted loft;
/// - a profile edge or arc whose direction a boxed vertex sets divides
///   that vertex's difference by its own length in the sketch, once
///   (`swept::sketch_vector` and `sketch_chord`, which read the
///   segment's points, not the direction its kind records): 4.0
///   half-widths on the edge, 10.7 on the arc.
///
/// Each row's factor is that measurement rounded up by a few percent.
fn assert_tight(what: &str, held: Vec3<S>, members: &[Vec3<f64>], width_over_truth: (f64, f64)) {
    let comps: [Component; 3] = [
        ("x", held.x, |v| v.x),
        ("y", held.y, |v| v.y),
        ("z", held.z, |v| v.z),
    ];
    for (axis, h, pick) in comps {
        let (lo, hi) = members
            .iter()
            .map(pick)
            .fold((f64::INFINITY, f64::NEG_INFINITY), |(lo, hi), v| {
                (lo.min(v), hi.max(v))
            });
        let truth = hi - lo;
        let ceiling = SLACK * truth + width_over_truth.0 * width_over_truth.1 + ROUNDING;
        assert!(
            width(h) <= ceiling,
            "{what}.{axis}: the stored width {:e} is within {SLACK}× the truth's spread \
             {truth:e} plus {} box half-widths ({:e}): the enclosure degraded",
            width(h),
            width_over_truth.1,
            width_over_truth.0
        );
    }
}

/// [`assert_holds`] and [`assert_tight`] over one stored direction and
/// its members.
fn assert_encloses(what: &str, held: Vec3<S>, members: &[Vec3<f64>], width_over_truth: (f64, f64)) {
    for m in members {
        assert_holds(what, held, *m);
    }
    assert_tight(what, held, members, width_over_truth);
}

/// A quarter turn about the sketch `y` axis by `angle`, partial.
fn quarter<T: Decide + topo::AtRestPolicy>(
    plane: SketchPlane<T>,
    lp: ProfileLoop<T>,
    angle: T,
) -> Result<sweep::Revolved<T>, sweep::RevolveError> {
    let profile = Profile::new(plane, vec![lp]).validate(tol()).unwrap();
    revolve(&profile, y_axis(), Revolution::Partial(angle), tol())
}

/// A partial revolve's end cap and its latitudes' `u_ref`s, in order.
fn far_station<T: Real + topo::AtRestPolicy>(r: &sweep::Revolved<T>) -> Vec<Vec3<T>>
where
    T::Leaf: Deref<Target = Body<T>>,
{
    let RevolvedKind::Partial { end_cap, .. } = r.kind else {
        panic!("a partial revolve has caps")
    };
    let body: &Body<T> = &r.body;
    let (_, normal, u_ref) = plane(body, end_cap);
    let mut out = vec![normal, u_ref];
    for rim in r.rims.iter().flatten().flatten() {
        let carrier = body
            .edge(*rim)
            .and_then(topo::EdgeView::certified)
            .expect("a carried rim")
            .carrier();
        out.push(
            carrier
                .as_circle()
                .expect("a latitude is a circle")
                .frame()
                .u_ref(),
        );
    }
    out
}

/// **A revolve's angle carries a box: the far station turns with it.**
/// Over a leaf-narrow box on the angle the end cap's normal and `u_ref`
/// and every latitude's `u_ref` hold every member's and are no wider
/// than the truth's spread allows. Over a macroscopic box the revolve
/// refuses at the far rims' endpoint pin (check 3): a rim's carrier end
/// and its far vertex reach the turned point by two routes through the
/// box's angle, so their distance reads the angle's width, today's
/// truth (`work/onehome/an-arc-on-a-turned-plane-misses-its-own-vertex-at-check-3-over-the-box.md`
/// is the same class on a turned plane).
#[test]
fn a_revolves_far_station_turns_with_a_boxed_angle() {
    let (lo, hi) = (1.0 - vertex_box(), 1.0 + vertex_box());
    let members: Vec<Vec<Vec3<f64>>> = members(lo, hi)
        .iter()
        .map(|&a| far_station(&quarter(SketchPlane::xy(), band(0.5), a).unwrap()))
        .collect();
    in_session(|| {
        let r = quarter(SketchPlane::<S>::xy(), band(lit(0.5)), boxed(0x9a1, lo, hi))
            .expect("a leaf-narrow angle box revolves");
        let held = far_station(&r);
        assert_eq!(held.len(), members[0].len(), "one datum per datum");
        for (i, h) in held.iter().enumerate() {
            let ms: Vec<Vec3<f64>> = members.iter().map(|m| m[i]).collect();
            assert_encloses(
                &format!("far station datum {i}"),
                *h,
                &ms,
                (vertex_box(), 0.0),
            );
        }
        let wide = quarter(
            SketchPlane::<S>::xy(),
            band(lit(0.5)),
            boxed(0x9a2, 1.0, 1.02),
        );
        let err = format!("{:?}", wide.expect_err("a macroscopic angle box refuses"));
        assert!(
            err.contains("EndpointEnd") || err.contains("EndpointStart"),
            "it refuses at check 3's endpoint pin: {err}"
        );
    });
}

/// A loft from a square on the floor to a smaller one on a plane tilted
/// by `tilt` about `x` and raised 1.5.
fn tilted_loft<T: Decide + topo::AtRestPolicy>(tilt: T) -> sweep::Lofted<T> {
    let lit = T::from_f64;
    let top = SketchPlane::from_frame(
        RigidMotion::rotation(UnitVec3::exact_x(), tilt)
            .unwrap()
            .translated(Vec3::new(T::zero(), T::zero(), lit(1.5)))
            .unwrap(),
    );
    loft_body(
        &[vec![square(T::one())], vec![square(lit(0.6))]],
        &[SketchPlane::xy(), top],
        1,
        tol(),
    )
    .expect("the tilted loft builds")
}

/// **A loft's placement carries a box: its top cap is that frame.** Over
/// a box on the top section's tilt, the bottom cap stays exact and the
/// top cap's normal and `u_ref` hold every member's and are no wider
/// than the truth's spread allows.
#[test]
fn a_lofts_top_cap_turns_with_a_boxed_placement() {
    let (lo, hi) = (0.10, 0.12);
    let members: Vec<[Vec3<f64>; 2]> = members(lo, hi)
        .iter()
        .map(|&t| {
            let l = tilted_loft(t);
            let (_, n, u) = plane(&l.body, l.top);
            [n, u]
        })
        .collect();
    in_session(|| {
        let l = tilted_loft(boxed(0x9b1, lo, hi));
        let (_, n, u) = plane(&l.body, l.bottom);
        assert_exact_unit("boxed loft bottom cap normal", n);
        assert_exact_unit("boxed loft bottom cap u_ref", u);
        let (_, n, u) = plane(&l.body, l.top);
        let ns: Vec<Vec3<f64>> = members.iter().map(|m| m[0]).collect();
        let us: Vec<Vec3<f64>> = members.iter().map(|m| m[1]).collect();
        assert_encloses("boxed loft top cap normal", n, &ns, (0.01, 0.5));
        assert_encloses("boxed loft top cap u_ref", u, &us, (0.01, 0.5));
        assert!(
            width(n.y).max(width(n.z)) > 1e-3,
            "the tilt's box turns the top cap"
        );
    });
}

/// **Two parameters at once**: a slab on a plane turned by a boxed angle,
/// extruded a boxed depth. Every face's normal and `u_ref` hold every
/// member's on a 3 × 3 grid of the box and are no wider than the truth
/// needs, and the top cap's origin holds every member's.
#[test]
fn a_slab_over_a_two_parameter_box_holds_every_member() {
    let ((alo, ahi), (dlo, dhi)) = ((0.3, 0.32), (0.95, 1.05));
    let grid: Vec<(f64, f64)> = [alo, 0.5 * (alo + ahi), ahi]
        .into_iter()
        .flat_map(|a| [dlo, 1.0, dhi].map(|d| (a, d)))
        .collect();
    let members: Vec<Extruded<f64>> = grid
        .iter()
        .map(|&(a, d)| slab(turned(a), quad(1.0, 0.0), d))
        .collect();
    in_session(|| {
        let b = slab(
            turned(boxed(0x9c1, alo, ahi)),
            quad(lit(1.0), lit(0.0)),
            boxed(0x9c2, dlo, dhi),
        );
        let held: Vec<[Vec3<S>; 2]> = faces(&b).into_iter().map(|f| datum(&b.body, f)).collect();
        for (i, h) in held.iter().enumerate() {
            for (k, hk) in h.iter().enumerate() {
                let ms: Vec<Vec3<f64>> = members
                    .iter()
                    .map(|m| datum(&m.body, faces(m)[i])[k])
                    .collect();
                assert_encloses(
                    &format!("two-parameter face {i} datum {k}"),
                    *hk,
                    &ms,
                    (0.01, 1.25),
                );
            }
        }
        let (origin, _, _) = plane(&b.body, b.top);
        for m in &members {
            let (o, _, _) = plane(&m.body, m.top);
            assert_holds(
                "two-parameter top origin",
                origin - Point3::origin(),
                o - Point3::origin(),
            );
        }
    });
}

/// **The vector arm keeps the width this unit removed elsewhere**
/// (`work/boxfinish/a-boxed-vector-extrusions-walls-carry-the-vectors-width.md`).
/// A slab extruded along a boxed vector `(0, 0, [0.95, 1.05])` stores
/// exact caps (the sketch frame), but its line walls' normals are
/// `place(d) × v/‖v‖` normalized, a division by a length the box moves,
/// so every wall's normal stores the vector's width though the truth
/// is fixed. The row states today's truth on both sides: the caps exact
/// to rounding, the walls holding every member and at least
/// [`VECTOR_WALL_WIDTH`] wide. It goes red when the arm is fixed, and
/// that is the row's purpose: re-cut it then.
const VECTOR_WALL_WIDTH: f64 = 0.3;

#[test]
fn a_slab_along_a_boxed_vector_stores_exact_caps_and_wide_walls() {
    let (lo, hi) = (0.95, 1.05);
    let along = |z: f64| {
        let profile = Profile::new(SketchPlane::xy(), vec![quad(1.0, 0.0)])
            .validate(tol())
            .unwrap();
        extrude(&profile, Extrusion::Vector(Vec3::new(0.0, 0.0, z)), tol()).unwrap()
    };
    let members: Vec<Extruded<f64>> = members(lo, hi).iter().map(|&z| along(z)).collect();
    in_session(|| {
        let profile = Profile::new(SketchPlane::<S>::xy(), vec![quad(lit(1.0), lit(0.0))])
            .validate(tol())
            .unwrap();
        let z = boxed(0x9d1, lo, hi);
        let b = extrude(
            &profile,
            Extrusion::Vector(Vec3::new(lit(0.0), lit(0.0), z)),
            tol(),
        )
        .expect("a boxed vector extrudes");
        for (name, face) in [("top", b.top), ("bottom", b.bottom)] {
            let (_, n, u) = plane(&b.body, face);
            assert_exact_unit(&format!("vector {name} cap normal"), n);
            assert_exact_unit(&format!("vector {name} cap u_ref"), u);
        }
        for (i, wall) in b.walls[0].iter().enumerate() {
            let (_, n, _) = plane(&b.body, wall.face);
            for m in &members {
                let (_, mn, _) = plane(&m.body, m.walls[0][i].face);
                assert_holds(&format!("vector wall {i} normal"), n, mn);
            }
            let w = width(n.x).max(width(n.y)).max(width(n.z));
            assert!(
                w >= VECTOR_WALL_WIDTH,
                "vector wall {i}'s normal is {w:e} wide, today's truth on this arm"
            );
        }
    });
}

/// The largest distance of a planar face's loop vertices from its
/// stored plane, over every planar face.
fn worst_vertex_off_plane(body: &Body<f64>) -> f64 {
    let mut worst = 0.0_f64;
    for (fk, _) in body.faces() {
        let Ok(Surface::Plane {
            frame: carrier_frame,
            ..
        }) = body.face_surface(fk)
        else {
            continue;
        };
        let (origin, normal) = (carrier_frame.origin(), carrier_frame.axis());
        for (_, lp, _) in body.face(fk).expect("a live face").loops() {
            if let topo::entity::LoopBoundary::Cycle { first } = lp.boundary {
                for he in body.loop_cycle(first).expect("a cycle") {
                    let q = body.half_edge_start_point(he).expect("a placed vertex");
                    worst = worst.max(((q - origin).dot(normal)).abs());
                }
            }
        }
    }
    worst
}

/// **A sketch origin far from its profile costs rounding, and the ε
/// budget holds it inside the session box.** The sketch plane is turned
/// and set 1e3 from the world origin (the session box's edge), and the
/// profile is drawn about the sketch point that lands at the world
/// origin, so the body is small and near the origin while its sketch
/// coordinates are 1e3. Every cap and wall carrier is the frame and
/// every vertex is the frame placing a sketch point, so a vertex misses
/// its plane by the placement's rounding of a 1e3 coordinate: at most a
/// few ulps of 1e3, measured 3.4e-13, where a profile at its sketch
/// origin reads 3e-15. The body builds at every ε row; at ε = 1e-12 that
/// residual spends a third of ε, and a sketch origin past the box
/// (4.2e3) refuses there
/// (`work/onehome/a-sketch-origin-far-from-its-profile-spends-the-placements-rounding.md`).
#[test]
fn a_slab_far_from_its_sketch_origin_holds_its_vertices_to_the_placements_rounding() {
    let far = 1.0e3;
    let frame = RigidMotion::rotation(UnitVec3::exact_x(), 0.3)
        .unwrap()
        .translated(Vec3::new(-far, 0.0, 0.0))
        .unwrap();
    let plane = SketchPlane::from_frame(frame);
    // The sketch point the frame places at the world origin.
    let centre = Point2::new(far, 0.0);
    let lp = bulge_loop(vec![
        (Point2::new(centre.x - 0.5, centre.y - 0.5), 0.0),
        (Point2::new(centre.x + 0.5, centre.y - 0.5), 0.0),
        (Point2::new(centre.x + 0.5, centre.y + 0.5), 0.0),
        (Point2::new(centre.x - 0.5, centre.y + 0.5), 0.0),
    ]);
    let built = slab(plane, lp, 1.0);
    let off = worst_vertex_off_plane(&built.body);
    let bound = 8.0 * f64::EPSILON * far;
    assert!(
        off <= bound,
        "every vertex lies on its plane to the placement's rounding of a {far:e} coordinate: \
         {off:e} against {bound:e}"
    );
    assert!(
        off < tol().eps() / 2.0,
        "and inside the session box that rounding fits the ε budget: {off:e} at ε {:e}",
        tol().eps()
    );
}

/// The washer `[r0, 2] × [0, 1]` about the sketch `y` axis, `r0` a
/// leaf-narrow box about 1, revolved whole or by one radian, in a
/// session under the shipped rules: whether it builds, and the
/// session's decision counts.
fn washer_counts(full: bool) -> (bool, u64, u64) {
    use geom_core::SymRules;
    use geom_core::sym::with_session_rules;
    let budget = SymBudget {
        max_terms: 4096,
        max_degree: 128,
    };
    let (ok, counts) = with_session_rules(budget, SymRules::shipped(), || {
        let r0 = about(0x9e1, 1.0);
        let profile = Profile::new(
            SketchPlane::<S>::xy(),
            vec![bulge_loop(vec![
                (Point2::new(r0, lit(0.0)), lit(0.0)),
                (p(2.0, 0.0), lit(0.0)),
                (p(2.0, 1.0), lit(0.0)),
                (Point2::new(r0, lit(1.0)), lit(0.0)),
            ])],
        )
        .validate(tol())
        .expect("the washer validates over its box");
        let r = if full {
            Revolution::Full
        } else {
            Revolution::Partial(lit(1.0))
        };
        revolve(&profile, y_axis(), r, tol()).is_ok()
    });
    (ok, counts.symbolic, counts.numeric)
}

/// **A latitude's endpoint identity is decided symbolically, with
/// nothing registered.** The washer's latitudes take their `u_ref` from
/// the revolve's frame, so a latitude through its point is
/// `centre + r·u₃ = q`, which the plain form proves; the registration
/// that used to state it (`register_rim_identity`, deleted) bought
/// nothing the tier does not now reach. Pinned as the session's
/// counts: a build that decided those endpoints numerically would move
/// decisions from the first count to the second. The full revolve reads
/// 501 / 462 and the partial 493 / 612, each numeric count holding the
/// revolve's one reading of the side of its axis the profile lies on.
/// Validation then stopped reading the four corners' shared vertices
/// (`profile::seg`'s `line_line`), six decisions a corner, 20 symbolic
/// and 4 numeric: 481 / 458 and 473 / 608. The profile's direction mint
/// then asks the chord's expression whether it is zero
/// (`geom_core::Expression::proves_zero`, no decision) where it read a
/// proved Zero: four symbolic fewer, 477 / 458 and 469 / 608. Of the
/// full washer's decisions, nine that read a carrier's distance from
/// the wall of radius `r0` and nine that rest on a ruling's phase there
/// are then theorems (`geom_core::radius_gap`; the phase read off the
/// offset over the radius), where a measured norm less the radius and
/// the angle of the offset at its length left them to the numeric
/// count: 495 / 440. The partial washer's eight are all the distance's:
/// 477 / 600. A full revolve's curved wall is one band closed on its
/// wrap edge, which is certified once more, restated from what it stores
/// so its certificate is a fresh run's (`revolve/upgrade.rs`,
/// `upgrade_meridian_wrap`): the full washer reads 503 / 442.
#[test]
fn a_revolves_latitude_identity_is_symbolic_without_a_registration() {
    assert_eq!(washer_counts(true), (true, 503, 442), "the full washer");
    assert_eq!(washer_counts(false), (true, 477, 600), "the partial washer");
}

/// A plane turned 0.4 about `x` and lifted 2: no axis of it is a world
/// axis.
fn tilted<T: Real>() -> SketchPlane<T> {
    SketchPlane::from_frame(
        RigidMotion::rotation(UnitVec3::exact_x(), T::from_f64(0.4))
            .unwrap()
            .translated(Vec3::new(T::zero(), T::zero(), T::from_f64(2.0)))
            .unwrap(),
    )
}

/// **Off `SketchPlane::xy()`**: a partial revolve on a tilted plane,
/// over a box on the profile's width. Its caps' normals and `u_ref`s and
/// its latitudes' `u_ref`s are its frames' turned axes, exact to
/// rounding though no component is 0 or 1, and equal to the `f64`
/// build's.
#[test]
fn a_partial_revolve_on_a_tilted_plane_stores_its_frames_exactly() {
    let exact = far_station(&quarter(tilted(), band(0.5), FRAC_PI_2).unwrap());
    in_session(|| {
        let r = quarter(tilted::<S>(), band(about(0x9f1, 0.5)), lit(FRAC_PI_2))
            .expect("the tilted band revolves a quarter turn");
        let held = far_station(&r);
        assert_eq!(held.len(), exact.len());
        for (i, (h, m)) in held.iter().zip(&exact).enumerate() {
            assert_exact(&format!("tilted far station datum {i}"), *h);
            assert_holds(&format!("tilted far station datum {i}"), *h, *m);
        }
        let RevolvedKind::Partial { start_cap, .. } = r.kind else {
            panic!("a partial revolve has caps")
        };
        let (_, n, u) = plane(&r.body, start_cap);
        assert_exact_unit("tilted start cap normal", n);
        assert_exact_unit("tilted start cap u_ref", u);
    });
}

/// A disc of radius 5e-3 on the sketch plane turned about `y` by
/// `theta`, extruded its radius.
fn turned_disc<T: Decide + topo::AtRestPolicy>(
    theta: T,
) -> Result<Extruded<T>, sweep::ExtrudeError> {
    let c = T::from_f64;
    let radius = 5e-3;
    let turn = RigidMotion::rotation(UnitVec3::exact_y(), theta).unwrap();
    let rim = RawLoop::new([(
        Point2::new(c(radius), c(0.0)),
        profile::Segment::Arc(geom_core::Arc2 {
            centre: Point2::new(c(0.0), c(0.0)),
            radius: c(radius),
            sweep: c(core::f64::consts::TAU),
        }),
    )]);
    let profile = Profile::new(SketchPlane::from_frame(turn), vec![rim])
        .validate(tol())
        .expect("the disc validates on its turned plane");
    let side = ExtrudeSide::Along;
    extrude(
        &profile,
        Extrusion::Distance {
            depth: c(radius),
            side,
        },
        tol(),
    )
}

/// **A disc on a plane turned by a boxed angle refuses at check 3**
/// (`work/onehome/an-arc-on-a-turned-plane-misses-its-own-vertex-at-check-3-over-the-box.md`):
/// the circle's carrier and its one vertex reach the same point through
/// the turned frame by two routes, so their distance reads the frame's
/// width, first order in the box. Today's truth on both box scalars,
/// with the `f64` member building; it goes red when either builds.
#[test]
fn a_disc_on_a_turned_plane_refuses_at_check_3_over_the_box() {
    let (lo, hi) = (29f64.to_radians(), 31f64.to_radians());
    turned_disc(30f64.to_radians()).expect("the f64 disc builds");
    let plain = format!(
        "{:?}",
        turned_disc(Interval::from_bounds(lo, hi)).expect_err("the Interval disc refuses")
    );
    assert!(
        plain.contains("carrier_endpoint_start"),
        "at Interval it refuses at check 3's start pin: {plain}"
    );
    in_session(|| {
        let sym = format!(
            "{:?}",
            turned_disc(boxed(0x9f2, lo, hi)).expect_err("the Sym<Interval> disc refuses")
        );
        assert!(
            sym.contains("carrier_endpoint_end"),
            "at Sym<Interval> it refuses at check 3's end pin: {sym}"
        );
    });
}

/// How far one arc verb's slab gets over a box of `half` either way on
/// the quantity that fixes its arc's radius: `"finished"`, or the stage
/// that refused (`validation`, `door`, `finish`) and its refusal's text.
/// A slab the door built is handed to `built` before it is finished.
fn verb_slab_stage(verb: &str, c: f64, half: f64, built: impl FnOnce(&Extruded<S>)) -> String {
    let q = boxed(0x9b0, c - half, c + half);
    let lp = profile::test_support::arc_verb_loop::<S>(verb, q, tol());
    let profile = match Profile::new(SketchPlane::xy(), vec![lp]).validate(tol()) {
        Ok(profile) => profile,
        Err(e) => return format!("validation: {e:?}"),
    };
    let side = ExtrudeSide::Along;
    let depth = lit(1.0);
    match extrude(&profile, Extrusion::Distance { depth, side }, tol()) {
        Err(e) => format!("door: {e:?}"),
        Ok(slab) => {
            built(&slab);
            match topo::take::<S>(&std::sync::Arc::new(slab.body), tol()) {
                Err(e) => format!("finish: {e:?}"),
                Ok(_) => "finished".to_owned(),
            }
        }
    }
}

/// A box three orders past the band or wider: `1e-6`, and `1e3·ε` where
/// that is wider.
fn wide_box() -> f64 {
    1e-6_f64.max(1e3 * tol().eps())
}

/// Each arc verb, what its slab does over a box validation decides
/// ([`vertex_box`]) and what it does over a wide one ([`wide_box`]): a
/// stage and a word its refusal carries.
const VERB_SLABS: [(&str, [&str; 2], [&str; 2]); 15] = [
    ("circle", ["finished", ""], ["finish", "VolumeUncomputable"]),
    ("split circle", ["finished", ""], ["door", "MappedSource"]),
    ("bulge", ["finished", ""], ["validation", "gate: ArcSpan"]),
    (
        "bulge cw",
        ["door", "MappedSource"],
        ["validation", "gate: ArcSpan"],
    ),
    ("via", ["finished", ""], ["validation", "gate: ArcSpan"]),
    ("via cw", ["finished", ""], ["validation", "gate: ArcSpan"]),
    (
        "tangent",
        ["finished", ""],
        ["validation", "gate: LineSpan"],
    ),
    (
        "tangent cw",
        ["finished", ""],
        ["validation", "gate: LineSpan"],
    ),
    (
        "onto",
        ["finished", ""],
        ["validation", "gate: SegmentDirection"],
    ),
    ("sweep", ["finished", ""], ["validation", "gate: LineSpan"]),
    (
        "sweep cw",
        ["finished", ""],
        ["validation", "gate: LineSpan"],
    ),
    (
        "fillet",
        ["validation", "gate: CanonicalOrderX"],
        ["validation", "gate: LineSpan"],
    ),
    (
        "arc len",
        ["finished", ""],
        ["validation", "gate: LineSpan"],
    ),
    (
        "arc len cw",
        ["finished", ""],
        ["validation", "gate: LineSpan"],
    ),
    (
        "radius",
        ["validation", "gate: ArcSpan"],
        ["validation", "gate: ArcStartOnCarrier"],
    ),
];

/// **Each arc verb's slab over a box on the quantity that fixes its
/// arc's radius.** Over a box validation decides, the door builds the
/// slab and its consumer finishes it: every cylinder wall's `u_ref` and
/// every rim circle's is a unit through the tier (its squared length
/// less one is the zero form, a theorem and not an enclosure inside the
/// band), and every member's `f64` build lies in the stored enclosures.
/// Three fixtures stop short there: the clockwise bulge's mapped-source
/// residual reads a hair over the band at the door, the fillet's
/// four-fold loop ties its canonical order over any box, and the
/// `Radius` arrival's fillet leaves a tangent point validation's span
/// reader cannot place
/// (`work/boxfinish/the-fillet-reads-two-signs-off-numbers-into-what-it-stores.md`).
///
/// Over a wide box the circle alone passes the door, and its
/// finish stops at the volume reading's rim premise
/// (`work/boxfinish/the-volume-readings-interval-re-derivation-re-asks-a-closed-forms-premises-on-the-value-channel.md`).
/// The split circle's door refuses its mapped-source residual (its
/// recorded radials are rounded sines and cosines, a unit to rounding
/// and not through the tier), and every verb drawn beside a line stops
/// in validation's span readers
/// (`work/boxfinish/a-bulge-arc-beside-a-line-stops-in-validations-span-readers-over-a-box.md`),
/// the onto arc in its first leg's direction.
///
/// Red under the circle verb storing `radius.abs()`
/// (`profile::path::circle_loop`): the circle's door refuses over the
/// wide box, at its mapped-source residual. Red under the door's arc
/// arm turned onto the start vertex's normalized offset in place of the
/// recorded radial (`sweep::swept::sketch_direction`): both circles'
/// doors refuse over the wide box at the carrier's start, and over the
/// narrow one the clockwise via arc's door refuses. Red under
/// `Arc2::register_radials` without its turned pair (the start radial
/// turned by the sweep lands on the far vertex): the split circle's door
/// refuses over the wide box at its far end.
#[test]
fn each_arc_verbs_slab_over_a_box_reaches_its_pinned_stage() {
    let band = geom_core::Band::linear(tol()).unwrap();
    // A unit as a theorem: the squared length less one is the zero
    // form. An enclosure inside the band reads Zero too, and is not one.
    let unit = |what: &str, u: Vec3<S>| {
        let miss = u.norm_squared() - lit(1.0);
        let read = miss.sign_within(band).ok();
        assert_eq!(
            read.map(|d| (d.sign, d.margin.proven())),
            Some((geom_core::Sign::Zero, true)),
            "{what} is a unit through the tier, as a theorem"
        );
    };
    let mut moved = Vec::new();
    for ((verb, c, _), (name, narrow, wide)) in
        profile::test_support::ARC_VERBS.into_iter().zip(VERB_SLABS)
    {
        assert_eq!(verb, name, "the two tables list the verbs in one order");
        for (half, [stage, word]) in [(vertex_box(), narrow), (wide_box(), wide)] {
            let mut got = String::new();
            in_session(|| {
                got = verb_slab_stage(verb, c, half, |slab| {
                    if stage != "finished" {
                        return;
                    }
                    let body: &Body<S> = &slab.body;
                    let mut curved = 0;
                    for wall in &slab.walls[0] {
                        if let Surface::Cylinder { frame, .. } =
                            body.face_surface(wall.face).expect("a live face")
                        {
                            unit(&format!("{verb} wall {:?} u_ref", wall.face), frame.u_ref());
                            curved += 1;
                        }
                    }
                    assert!(curved > 0, "{verb} sweeps a cylinder wall");
                    let mut rims = 0;
                    for (edge, _) in body.edges() {
                        let circle = body
                            .edge(edge)
                            .and_then(topo::EdgeView::certified)
                            .and_then(|c| c.carrier().as_circle());
                        if let Some(circle) = circle {
                            unit(
                                &format!("{verb} rim {edge:?} u_ref"),
                                circle.frame().u_ref(),
                            );
                            rims += 1;
                        }
                    }
                    assert!(rims > 0, "{verb} has rim circles");
                    let members = members(c - half, c + half).map(|q| {
                        slab_of(profile::test_support::arc_verb_loop::<f64>(verb, q, tol()))
                    });
                    assert_holds_its_members(verb, slab, members, None);
                });
            });
            let short: String = got.chars().take(200).collect();
            println!("{verb} ±{half:e}: {short}");
            if !(got.starts_with(stage) && got.contains(word)) {
                moved.push(format!(
                    "{verb} ±{half:e}: pinned {stage} {word:?}, got {short}"
                ));
            }
        }
    }
    assert!(
        moved.is_empty(),
        "an arc verb's slab reaches a different stage over its box:\n{}",
        moved.join("\n")
    );
}

/// A unit slab of `lp` on the sketch plane `xy`.
fn slab_of<T: Decide + topo::AtRestPolicy>(lp: ProfileLoop<T>) -> Extruded<T> {
    slab(SketchPlane::xy(), lp, T::one())
}

/// **A table's written radial does not reach a wall's frame as written**
/// (a fixture row: the table door, `RawLoop::new_directed`, is compiled
/// for tests only, so no shipped build holds a written radial). A
/// two-arc unit circle whose start radial is written too long by a part
/// of ε: validation accepts it (`segment_direction` reads
/// `‖c + r·d − p‖`, which bounds the stretch times the radius), and the
/// extrude turns the wall's and the rims' frames onto it through
/// `geom_core::RigidMotion::spun_onto`, which measures the frame it
/// mints and squares one past its rounding. So the body builds, at a
/// stretch of `0.1·ε` and of `0.4·ε` alike, and every `u_ref` it stores
/// is a unit to rounding.
///
/// Red when a written stretch reaches a stored frame (a frame door that
/// takes a direction for a unit without measuring it).
#[test]
fn a_tables_stretched_radial_is_squared_at_the_frame_door() {
    let half = geom_core::Arc2 {
        centre: Point2::new(0.0, 0.0),
        radius: 1.0,
        sweep: core::f64::consts::PI,
    };
    for part in [0.1, 0.4] {
        let stretch = part * tol().eps();
        let lp: ProfileLoop<f64> = RawLoop::new_directed([
            (
                Point2::new(1.0, 0.0),
                profile::Segment::Arc(half),
                Some(profile::SegmentDirections::Arc {
                    start: Vec2::new(1.0 + stretch, 0.0),
                    end: Vec2::new(-1.0, 0.0),
                }),
            ),
            (Point2::new(-1.0, 0.0), profile::Segment::Arc(half), None),
        ]);
        let built = slab(SketchPlane::xy(), lp, 1.0);
        let body: &Body<f64> = &built.body;
        let mut longest = 0.0_f64;
        for (face, _) in body.faces() {
            if let Surface::Cylinder { frame, .. } = body.face_surface(face).expect("a live face") {
                longest = longest.max((frame.u_ref().norm() - 1.0).abs());
            }
        }
        for (edge, _) in body.edges() {
            let circle = body
                .edge(edge)
                .and_then(topo::EdgeView::certified)
                .and_then(|c| c.carrier().as_circle());
            if let Some(circle) = circle {
                longest = longest.max((circle.frame().u_ref().norm() - 1.0).abs());
            }
        }
        println!("stretch {stretch:e}, longest u_ref defect {longest:e}");
        assert!(
            longest <= 4.0 * f64::EPSILON,
            "a stretch of {stretch:e} is squared out of every frame: {longest:e}"
        );
    }
}

/// **At an enclosure scalar a table's stretched radial is refused,
/// typed.** The same table at `Interval` and at `Sym<Interval>`: the
/// frame mint keeps an enclosure's bits, so a radial `0.1·ε` or `0.4·ε`
/// too long gives a frame that is no rotation, poisoned, and the door
/// refuses at the carrier's start (`EndpointStart`, an invalid margin).
/// Unstretched it builds.
///
/// Red where a stretched radial builds a body at an enclosure scalar.
#[test]
fn a_tables_stretched_radial_is_refused_typed_at_the_enclosure_scalars() {
    fn door<T: Decide + topo::AtRestPolicy>(stretch: f64) -> Result<(), String> {
        let l = T::from_f64;
        let half = geom_core::Arc2 {
            centre: Point2::new(l(0.0), l(0.0)),
            radius: l(1.0),
            sweep: l(core::f64::consts::PI),
        };
        let lp: ProfileLoop<T> = RawLoop::new_directed([
            (
                Point2::new(l(1.0), l(0.0)),
                profile::Segment::Arc(half),
                Some(profile::SegmentDirections::Arc {
                    start: Vec2::new(l(1.0 + stretch), l(0.0)),
                    end: Vec2::new(l(-1.0), l(0.0)),
                }),
            ),
            (
                Point2::new(l(-1.0), l(0.0)),
                profile::Segment::Arc(half),
                None,
            ),
        ]);
        let profile = Profile::new(SketchPlane::xy(), vec![lp])
            .validate(tol())
            .map_err(|e| format!("validation: {e:?}"))?;
        let side = ExtrudeSide::Along;
        extrude(
            &profile,
            Extrusion::Distance {
                depth: l(1.0),
                side,
            },
            tol(),
        )
        .map(|_| ())
        .map_err(|e| format!("door: {e:?}"))
    }
    for part in [0.0, 0.1, 0.4] {
        let stretch = part * tol().eps();
        let mut at_sym = Ok(());
        in_session(|| at_sym = door::<S>(stretch));
        for (lane, got) in [
            ("Interval", door::<Interval>(stretch)),
            ("Sym<Interval>", at_sym),
        ] {
            if part == 0.0 {
                assert_eq!(got, Ok(()), "{lane}: the unstretched table builds");
            } else {
                let refusal = got.expect_err("a stretched radial builds nowhere");
                assert!(
                    refusal.starts_with("door:") && refusal.contains("EndpointStart"),
                    "{lane}, stretch {stretch:e}: {refusal}"
                );
            }
        }
    }
}

/// A three-arc circle of radius `r` about the origin, its first vertex
/// a quarter of a radian round: radials that are rounded sines and
/// cosines, a unit to rounding.
fn split_circle<T: profile::ArcCarrierScalar>(r: f64) -> ProfileLoop<T> {
    let l = T::from_f64;
    profile::circle_split(Point2::new(l(0.0), l(0.0)), l(r), 3, l(0.25), tol())
        .expect("the split circle draws")
        .loop_
        .into_loop()
}

/// **An `f64` loft of arc sections held at `Interval` turns its rims
/// onto radials that enclose a unit.** Two three-arc circles lofted at
/// `f64` and held at `Interval` as points
/// (`sweep::test_support::f64_body_held_as_points_construction`, a test
/// door, the one caller of `ValidatedProfile::lift_onto`): the caps'
/// rims are built from the `f64` canonical form lifted onto the held
/// placements. A lifted radial's two numbers are a unit to rounding, and
/// an arc's rim frame is turned onto its radial with no normalization,
/// so the lift normalizes them at `Interval`: every rim circle's `u_ref`
/// has a width (it is an enclosure, not the `f64` point) and its length
/// holds one.
///
/// Red under the lift carrying an arc's radials through `from_f64`
/// alone (`ValidatedSegment::lift` without its `normalize`): each rim's
/// `u_ref` is a point.
#[test]
fn a_loft_of_arc_sections_at_interval_turns_its_rims_onto_enclosed_units() {
    let top = SketchPlane::<f64>::xy_at(Point3::new(0.0, 0.0, 1.5));
    let lofted = sweep::test_support::f64_body_held_as_points_construction::<Interval>(
        &[
            vec![split_circle::<f64>(1.0)],
            vec![split_circle::<f64>(0.6)],
        ],
        &[SketchPlane::xy(), top],
        1,
        tol(),
    )
    .expect("the f64 loft of arc sections is held at Interval");
    let body: &Body<Interval> = &lofted.body;
    let mut rims = 0;
    for (edge, _) in body.edges() {
        let circle = body
            .edge(edge)
            .and_then(topo::EdgeView::certified)
            .and_then(|c| c.carrier().as_circle());
        let Some(circle) = circle else {
            continue;
        };
        rims += 1;
        let u = circle.frame().u_ref();
        let n = u.norm();
        assert!(
            n.lo() <= 1.0 && 1.0 <= n.hi(),
            "rim {edge:?}: the u_ref's length holds one: [{}, {}]",
            n.lo(),
            n.hi()
        );
        assert!(
            width(u.x) > 0.0 || width(u.y) > 0.0,
            "rim {edge:?}: the u_ref is an enclosure, not the f64 radial's point: {u:?}"
        );
        // And a tight one: a normalization's rounding, not a width the
        // lift made up.
        assert!(
            n.hi() - n.lo() <= ROUNDING,
            "rim {edge:?}: the u_ref's length is one to rounding: [{}, {}]",
            n.lo(),
            n.hi()
        );
    }
    assert!(rims > 0, "the loft's caps have circle rims");
}
