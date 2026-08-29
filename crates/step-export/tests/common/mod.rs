//! Shared acceptance-body builders for the STEP export suites, via the
//! public profile/sweep/boolean APIs only (the same shapes as the M3
//! STL review suites: bricks, the pocketed die, the corner-kiss
//! assembly, the voided subtract).
#![allow(dead_code)] // loaded once per consumer; each uses a subset
#![allow(unreachable_pub)] // why: root Cargo.toml, the `unreachable_pub` stanza
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::Tol;
use geom_core::{Point2, Point3, Vec3};
use profile::RawLoop;
use profile::{Profile, ProfileLoop, ProfileVertex, SketchPlane, ValidatedProfile};
use sweep::{Extrusion, extrude};
use topo::{Body, BooleanResult, BooleanResultKind, subtract, union};

fn validated(plane: SketchPlane<f64>, lp: ProfileLoop<f64>) -> ValidatedProfile<f64> {
    Profile::new(plane, vec![lp])
        .validate(Tol::witness())
        .unwrap()
}

/// An axis-aligned brick `[x0,x1]×[y0,y1]×[z0,z1]`.
pub fn brick(x: (f64, f64), y: (f64, f64), z: (f64, f64)) -> Body<f64> {
    let lp = ProfileLoop::polygon([
        Point2::new(x.0, y.0),
        Point2::new(x.1, y.0),
        Point2::new(x.1, y.1),
        Point2::new(x.0, y.1),
    ]);
    extrude(
        &validated(
            SketchPlane::from_frame(
                Point3::new(0.0, 0.0, z.0),
                Vec3::new(1.0, 0.0, 0.0),
                Vec3::new(0.0, 1.0, 0.0),
            ),
            lp,
        ),
        Extrusion::Distance(z.1 - z.0),
        Tol::witness(),
    )
    .unwrap()
    .body
}

/// The unit cube `[0,1]³` — the spike's 6/12/8 reference shape.
pub fn cube() -> Body<f64> {
    brick((0.0, 1.0), (0.0, 1.0), (0.0, 1.0))
}

/// A pocketed die at `[x0,x0+1]³`: unit cube minus a centered
/// 0.5×0.5×0.5 pocket opening through the TOP face (the M3 die shape;
/// exact volume 0.875). A genuine boolean result: the top face carries
/// a ring (the pocket mouth).
pub fn die(x0: f64, y0: f64, z0: f64) -> Body<f64> {
    let cube = brick((x0, x0 + 1.0), (y0, y0 + 1.0), (z0, z0 + 1.0));
    let cutter = brick(
        (x0 + 0.25, x0 + 0.75),
        (y0 + 0.25, y0 + 0.75),
        (z0 + 0.5, z0 + 1.5),
    );
    let BooleanResult::Body(b) = subtract(&cube, &cutter, Tol::witness()).unwrap() else {
        panic!("die subtract is a body");
    };
    b.body
}

/// Two pocketed dies kissing at the corner `(1,1,1)` — the M3 R6
/// assembly: one solid, TWO shells (both outward), exact volume 1.75.
pub fn kiss_assembly() -> Body<f64> {
    let d1 = die(0.0, 0.0, 0.0);
    let d2 = die(1.0, 1.0, 1.0);
    let BooleanResult::Body(assembly) = union(&d1, &d2, Tol::witness()).unwrap() else {
        panic!("kiss union is a body");
    };
    assert_eq!(assembly.body.shells().count(), 2, "two kissing shells");
    assembly.body
}

/// A∖B with B strictly inside A: `[0,3]³` minus `[1,2]³` — the Voided
/// boolean result (outer shell + reverted void shell; cavity volume 1).
pub fn voided() -> Body<f64> {
    let a = brick((0.0, 3.0), (0.0, 3.0), (0.0, 3.0));
    let b = brick((1.0, 2.0), (1.0, 2.0), (1.0, 2.0));
    let BooleanResult::Body(result) = subtract(&a, &b, Tol::witness()).unwrap() else {
        panic!("voided subtract is a body");
    };
    assert_eq!(result.kind, BooleanResultKind::Voided, "B inside A voids");
    result.body
}

// ---- the curved corpus (M5 PR 13) ------------------------------------
//
// The R5 shapes constructible at rest, chosen so that every new writer
// arm has a body behind it: CIRCLE and CYLINDRICAL_SURFACE (several),
// ELLIPSE (`cut_cylinder`), SPHERICAL_SURFACE (`ball`),
// CONICAL_SURFACE (`cone`), TOROIDAL_SURFACE (`donut`), and the
// `same_sense = .F.` arm (`notched`, `washer`). Recipes are the same
// ones the mesh/stl/sweep suites use — deliberately not new geometry.

/// The revolve axis of the corpus: the profile plane's +y through the
/// origin (so revolved bodies stand on the world Y axis).
fn revolve_y() -> sweep::RevolveAxis<f64> {
    sweep::RevolveAxis {
        origin: Point2::new(0.0, 0.0),
        dir: geom_core::Vec2::new(0.0, 1.0),
    }
}

/// A revolved unit ball — ONE `Surface::Sphere` carrying two half-bands
/// (V2 E2 F2), the seam meridian and its π copy as `Circle` carriers,
/// the poles as ordinary vertices. Exact volume 4π/3.
pub fn ball() -> Body<f64> {
    use profile::ProfileVertex;
    use sweep::{Revolution, revolve};
    let lp = ProfileLoop::new(vec![
        ProfileVertex::new(Point2::new(0.0, -1.0), 1.0),
        ProfileVertex::new(Point2::new(0.0, 1.0), 0.0),
    ]);
    let profile = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(Tol::witness())
        .unwrap();
    revolve(&profile, revolve_y(), Revolution::Full, Tol::witness())
        .unwrap()
        .body
}

/// A revolved cone: the right triangle (base radius 1, height 1)
/// swept fully — `Surface::Cone` (apex fan) plus the base `Plane`
/// disc. Exact volume π/3. The one corpus body that exercises the
/// apex-placement `CONICAL_SURFACE` encoding.
pub fn cone() -> Body<f64> {
    use sweep::{Revolution, revolve};
    let lp = ProfileLoop::polygon([
        Point2::new(0.0, 0.0),
        Point2::new(1.0, 0.0),
        Point2::new(0.0, 1.0),
    ]);
    let profile = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(Tol::witness())
        .unwrap();
    revolve(&profile, revolve_y(), Revolution::Full, Tol::witness())
        .unwrap()
        .body
}

/// A revolved donut: the radius-½ circle centred at (2, 0) swept
/// fully — a single `Surface::Torus`, both meridians `Seam`. Exact
/// volume 2π²Rr² = π².
pub fn donut() -> Body<f64> {
    use profile::ProfileVertex;
    use sweep::{Revolution, revolve};
    let lp = ProfileLoop::new(vec![
        ProfileVertex::new(Point2::new(2.0, -0.5), 1.0),
        ProfileVertex::new(Point2::new(2.0, 0.5), 1.0),
    ]);
    let profile = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(Tol::witness())
        .unwrap();
    revolve(&profile, revolve_y(), Revolution::Full, Tol::witness())
        .unwrap()
        .body
}

/// The globe lily's **lantern** (the M6 globe-lily demo unit's flower,
/// re-authored here in the corpus frame): a sphere ZONE of radius 0.44
/// truncated at BOTH poles — 0.40 above the centre (the attachment
/// disc the pedicel enters through) and 0.36 below it — closed by a
/// conical pucker dropping 0.16 to a disc of radius 0.09.
///
/// What it adds to the corpus that nothing else has: a spherical face
/// with NEITHER pole on it. `ball` is a whole sphere (both poles),
/// `die_pips`' dimples are caps (one pole each) — and OCC normalises
/// each surviving pole into a DEGENERATE edge on import (+4 and +2
/// respectively, per those fixtures' sidecars). A doubly-truncated
/// zone has none, so this is the corpus's one curved fixture whose
/// FreeCAD edge count equals the kernel's exactly. It is also the only
/// body pairing a sphere zone with a cone across a shared circle.
///
/// Exact volume: the zone integral plus the frustum,
/// `π[r²(a+b) − (a³+b³)/3] + π·h(R₂² + R₂ρ + ρ²)/3` with r = 0.44,
/// a = 0.40, b = 0.36, R₂ = √(r²−b²), ρ = 0.09, h = 0.16 —
/// **0.36225803729804673 m³**.
///
/// **This is no longer the same solid as the tour's `lily_lantern`,
/// and the difference is deliberate.** That body was re-authored to
/// weld to its stem on a shared circle (#1059): above the truncation
/// circle it opens through a NECK cone cut at the arch tube's radius,
/// which adds a third frustum term and brings it to
/// 0.36455193285177373 m³. What this fixture is FOR is the corpus
/// property below — a spherical face with neither pole on it, the one
/// curved body whose FreeCAD edge count equals the kernel's — and a
/// neck would add faces without adding that. The two bodies share a
/// name and a globe, not a volume.
pub fn lily_lantern() -> Body<f64> {
    use profile::{ArcSweep, Center, Open, Start};
    use sweep::{Revolution, revolve};
    let (globe, top, mouth, lip_r, lip_drop): (f64, f64, f64, f64, f64) =
        (0.44, 0.40, 0.36, 0.09, 0.16);
    let r_top = (globe.powi(2) - top.powi(2)).sqrt();
    let r_mouth = (globe.powi(2) - mouth.powi(2)).sqrt();
    let lp = Open
        .at(Point2::new(0.0, top))
        .line_to(Point2::new(r_top, top), Tol::witness())
        .unwrap()
        // The belly, on the globe's own carrier: past the equator, so
        // the sweep is the CLOCKWISE (descending-angle) one.
        .arc_to(
            Center {
                c: Point2::new(0.0, 0.0),
                winding: ArcSweep::Cw,
                p: Point2::new(r_mouth, -mouth),
            },
            Tol::witness(),
        )
        .unwrap()
        .line_to(Point2::new(lip_r, -mouth - lip_drop), Tol::witness())
        .unwrap()
        .line_to(Point2::new(0.0, -mouth - lip_drop), Tol::witness())
        .unwrap()
        .line_to(Start, Tol::witness())
        .unwrap()
        .loop_;
    let profile = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(Tol::witness())
        .unwrap();
    revolve(&profile, revolve_y(), Revolution::Full, Tol::witness())
        .unwrap()
        .body
}

/// A revolved washer: the rectangle `[1,2]×[0,1]` swept fully — genus 1,
/// two annuli and two full-2π cylinder walls. Its BORE wall and its
/// under-side annulus both carry `sense: false` (S11), so this is the
/// corpus's two-`.F.` body. Exact volume 3π.
pub fn washer() -> Body<f64> {
    use sweep::{Revolution, revolve};
    let lp = ProfileLoop::polygon([
        Point2::new(1.0, 0.0),
        Point2::new(2.0, 0.0),
        Point2::new(2.0, 1.0),
        Point2::new(1.0, 1.0),
    ]);
    let profile = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(Tol::witness())
        .unwrap();
    revolve(&profile, revolve_y(), Revolution::Full, Tol::witness())
        .unwrap()
        .body
}

/// M5 shape (i), the above half: a unit cylinder of height 2.5 split
/// by a plane tilted 0.3 rad through the mid-height axis point. The
/// section rim is an exact `Ellipse` (semi-axes 1 and 1/cos 0.3) —
/// the corpus's only ELLIPSE carrier. Exact volume π·1.25 (the tilted
/// plane passes through the axis midpoint, so it halves the cylinder).
pub fn cut_cylinder() -> Body<f64> {
    use profile::ProfileVertex;
    use topo::splitting::{SplitPart, SplitPlane, split};
    let lp = ProfileLoop::new(vec![
        ProfileVertex::new(Point2::new(-1.0, 0.0), 1.0),
        ProfileVertex::new(Point2::new(1.0, 0.0), 1.0),
    ]);
    let profile = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(Tol::witness())
        .unwrap();
    let cylinder = extrude(&profile, Extrusion::Distance(2.5), Tol::witness())
        .unwrap()
        .body;
    let phi: f64 = 0.3;
    let plane = SplitPlane {
        origin: Point3::new(0.0, 0.0, 1.25),
        normal: Vec3::new(phi.sin(), 0.0, phi.cos()),
    };
    let result = split(&cylinder, &plane, Tol::witness()).unwrap();
    let SplitPart::Body(above) = &result.above else {
        panic!("the above half carries material");
    };
    above.clone()
}

/// M5 shape (ii): a 4×4×1 plate ∪ a radius-½ cylindrical boss (three
/// 120° arc segments) rising from z = 0.4 to z = 1.6 — the first
/// transverse curved boolean. The seam is three exact circle arcs
/// shared between the protruding walls and the plate's ringed top
/// face. Exact volume 16 + π·0.25·0.6.
pub fn boss_union() -> Body<f64> {
    use geom_core::Affine3;
    use profile::ProfileVertex;
    let plate_loop = ProfileLoop::polygon([
        Point2::new(0.0, 0.0),
        Point2::new(4.0, 0.0),
        Point2::new(4.0, 4.0),
        Point2::new(0.0, 4.0),
    ]);
    let plate = extrude(
        &validated(SketchPlane::xy(), plate_loop),
        Extrusion::Distance(1.0),
        Tol::witness(),
    )
    .unwrap()
    .body;
    let b120 = (core::f64::consts::PI / 6.0).tan();
    let at = |deg: f64| {
        let th: f64 = deg.to_radians();
        Point2::new(2.0 + 0.5 * th.cos(), 2.0 + 0.5 * th.sin())
    };
    let boss_loop = ProfileLoop::new(vec![
        ProfileVertex::new(at(0.0), b120),
        ProfileVertex::new(at(120.0), b120),
        ProfileVertex::new(at(240.0), b120),
    ]);
    let sketch = SketchPlane::new(Affine3::translation(Vec3::new(0.0, 0.0, 0.4)));
    let boss_profile = Profile::new(sketch, vec![boss_loop])
        .validate(Tol::witness())
        .unwrap();
    let boss = extrude(&boss_profile, Extrusion::Distance(1.2), Tol::witness())
        .unwrap()
        .body;
    let BooleanResult::Body(bb) = union(&plate, &boss, Tol::witness()).unwrap() else {
        panic!("the boss union yields a body");
    };
    bb.body
}

/// The S11 notched prism: a 2×1.5 rectangle with a CONVEX 45° bulge on
/// the bottom edge and an equal CONCAVE bite on the top, extruded 1.
/// The two bulges cancel, so the exact volume is 3. The concave wall
/// is the kernel's canonical `sense: false` face — this body is why the
/// `same_sense = .F.` arm exists.
pub fn notched() -> Body<f64> {
    let b = core::f64::consts::FRAC_PI_8.tan();
    // Leaving bulges: the bottom arc bows out (+b), the top one bows
    // into the region (-b); the two sides are straight.
    let lp = <ProfileLoop<f64> as RawLoop<f64>>::new(vec![
        ProfileVertex::new(Point2::new(0.0, 0.0), b),
        ProfileVertex::new(Point2::new(2.0, 0.0), 0.0),
        ProfileVertex::new(Point2::new(2.0, 1.5), -b),
        ProfileVertex::new(Point2::new(0.0, 1.5), 0.0),
    ]);
    extrude(
        &validated(SketchPlane::xy(), lp),
        Extrusion::Distance(1.0),
        Tol::witness(),
    )
    .unwrap()
    .body
}

/// S12's two-stub complement: a radius-0.35 cylindrical boss spanning
/// z ∈ [−0.2, 1.0] MINUS a 3×3×0.8 plate — the boss's two ends stick
/// out above and below, so the result is one solid with TWO disjoint
/// shells, both carrying cylinder walls. The only curved MULTI-shell
/// body constructible at rest, and therefore the only body that
/// reaches (and refuses at) the export's outward/void classifier.
/// Deliberately NOT a committed fixture: it does not export.
pub fn two_stub_complement() -> Body<f64> {
    use core::f64::consts::PI;

    use geom_core::Affine3;
    use profile::ProfileVertex;
    let plate = extrude(
        &validated(
            SketchPlane::xy(),
            ProfileLoop::polygon([
                Point2::new(0.0, 0.0),
                Point2::new(3.0, 0.0),
                Point2::new(3.0, 3.0),
                Point2::new(0.0, 3.0),
            ]),
        ),
        Extrusion::Distance(0.8),
        Tol::witness(),
    )
    .unwrap()
    .body;
    let r = 0.35;
    let bulge = (PI / 6.0).tan();
    let at = |i: usize| {
        let th = 2.0 * PI / 3.0 * i as f64;
        Point2::new(1.2 + r * th.cos(), 1.7 + r * th.sin())
    };
    let boss_loop = ProfileLoop::new((0..3).map(|i| ProfileVertex::new(at(i), bulge)).collect());
    let sketch = SketchPlane::new(Affine3::translation(Vec3::new(0.0, 0.0, -0.2)));
    let boss = extrude(
        &Profile::new(sketch, vec![boss_loop])
            .validate(Tol::witness())
            .unwrap(),
        Extrusion::Distance(1.2),
        Tol::witness(),
    )
    .unwrap()
    .body;
    let BooleanResult::Body(stubs) = subtract(&boss, &plate, Tol::witness()).unwrap() else {
        panic!("boss minus plate is a body");
    };
    stubs.body
}

/// **The writer's emission order, mirrored** (faces, then edges by
/// first encounter): solids in arena order → shells in `Solid::shells`
/// order → faces in `Shell::faces` order → the outer loop then rings in
/// stored order → half-edges in loop-cycle order.
///
/// The suites that compare emitted records against the body's own
/// geometry need to know WHICH kernel entity a record belongs to, and
/// entity ids are allocated along exactly this walk (`writer.rs`'s
/// module docs, D9). Note this is deliberately NOT `body.faces()` /
/// `body.edges()`: those are arena order, which coincides with the walk
/// on simple extrusions and diverges on boolean results — a helper that
/// used them would silently compare the wrong pairs on precisely the
/// most interesting bodies.
pub fn walk_order(body: &Body<f64>) -> (Vec<topo::FaceKey>, Vec<topo::EdgeKey>) {
    let mut faces = Vec::new();
    let mut edges = Vec::new();
    for (_, solid) in body.solids() {
        for &shell_key in &solid.shells {
            let shell = body.get_shell(shell_key).expect("shell resolves");
            for &face_key in &shell.faces {
                faces.push(face_key);
                let face = body.get_face(face_key).expect("face resolves");
                for &loop_key in std::iter::once(&face.outer).chain(face.rings.iter()) {
                    let loop_ = body.get_loop(loop_key).expect("loop resolves");
                    let topo::LoopBoundary::Cycle { first } = loop_.boundary else {
                        panic!("a finished body has no empty loop");
                    };
                    for he_key in body.loop_cycle(first).expect("cycle closes") {
                        let he = body.get_half_edge(he_key).expect("half-edge resolves");
                        if !edges.contains(&he.edge) {
                            edges.push(he.edge);
                        }
                    }
                }
            }
        }
    }
    (faces, edges)
}

/// The certified carrier of an edge, for the suites that compare
/// emitted geometry with the body's own (`topo`'s public accessors,
/// spelled once).
pub fn certified_carrier(body: &Body<f64>, edge: topo::EdgeKey) -> &geom::Curve3<f64> {
    let edge = body.get_edge(edge).expect("edge key resolves");
    match body.get_curve_geom(edge.curve).expect("curve key resolves") {
        topo::CurveGeom::Certified(curve) => curve.carrier(),
        topo::CurveGeom::NullScaffold(_) => panic!("a finished body has no null scaffolding"),
    }
}

/// **The committed-fixture corpus, in file order** — the single list
/// behind `examples/export_fixtures.rs` (which writes the `.step`
/// files), `tests/export.rs::committed_fixtures_are_byte_golden`
/// (which pins them), and `scripts/check_step.sh` (which imports every
/// `.step` in the directory into FreeCAD/OCC against its hand-authored
/// `.expect` sidecar — a `.step` WITHOUT a sidecar is a hard failure,
/// so a row added here needs one: the `EXPECT_*` lines written by hand
/// from the FreeCAD run, the `KERNEL_*` lines from the live kernel —
/// `tests/kernel_sidecars.rs`'s failure output prints the exact block).
///
/// Order is planar-first then curved, and it is the order the files
/// were minted in; it has no semantic weight beyond keeping diffs
/// readable.
pub fn fixture_corpus() -> Vec<(&'static str, Body<f64>)> {
    vec![
        // The M4 planar set.
        ("cube", cube()),
        ("die", die(0.0, 0.0, 0.0)),
        ("kiss_assembly", kiss_assembly()),
        // The M5 PR 13 curved set.
        ("cut_cylinder", cut_cylinder()),
        ("boss_union", boss_union()),
        ("notched", notched()),
        ("washer", washer()),
        ("ball", ball()),
        ("cone", cone()),
        ("donut", donut()),
        // The M6 globe-lily unit: a doubly-truncated sphere zone meeting
        // a conical pucker (see `lily_lantern`) — the corpus's first
        // spherical face with no pole on it.
        ("lily_lantern", lily_lantern()),
        // The M5 PR 12 fillet set: the die blank carries all five
        // elementary surface kinds' hardest pairing for a writer —
        // plane, cylinder AND sphere faces meeting along TANGENT
        // trimlines, straight ones and circular ones.
        ("filleted_die", filleted_die()),
        // The OTHER half of PR 12's die, taken by the M6 curation unit
        // (M5 exit-walk row 12): the pipped cube. Its STEP export was
        // verified BY HAND only, through the tour's `diepips` render;
        // here it joins the CI-gated corpus like every other shipped
        // body. Twenty-one spherical dimples cut in ONE group
        // operation, so every pip mouth is a ring in a planar face
        // whose carrier is an exact circle and whose floor is a
        // sphere patch — the writer's plane-with-many-rings arm and
        // its curved-ring pairing, on one solid.
        ("die_pips", die_pips()),
        // The M6 composed die (unit 1): blank + 21 pips + 21 rim TORUS
        // bands in ONE body (the composition surgery). Adds the
        // writer's first fillet-minted TOROIDAL_SURFACEs (slit-seamed
        // annuli) alongside every kind the blank already carries.
        ("composed_die", composed_die()),
        // The M6-3 loft assembly: the corpus's first NURBS-walled body
        // (B_SPLINE_SURFACE_WITH_KNOTS walls, B_SPLINE_CURVE_WITH_KNOTS
        // seam carriers, planar caps with exact line rims).
        ("loft_prism", loft_prism()),
        // The #210 corpus-widening fold: the exportable class the
        // #207 skin-fit fix opened. `nonuniform_loft` is
        // `loft_prism`'s minimal pair (same sections, spacing 1 : 2);
        // `swept_elbow` is the tree's first curved-path `sweep_body`
        // and the corpus's first body whose walls approximate rather
        // than reproduce a closed-form surface.
        ("nonuniform_loft", nonuniform_loft()),
        ("swept_elbow", swept_elbow()),
    ]
}

/// The M5 PR 12 die blank: a unit cube with every edge blended at
/// r = 0.12 — 6 shrunk planes, 12 quarter-cylinders, 8 sphere octants.
pub fn filleted_die() -> Body<f64> {
    let tol = geom_core::Tol::witness();
    let band = geom_core::Band::new(tol.eps(), tol.k() * tol.eps()).expect("band");
    let lp = profile::ProfileLoop::polygon([
        Point2::new(0.0, 0.0),
        Point2::new(1.0, 0.0),
        Point2::new(1.0, 1.0),
        Point2::new(0.0, 1.0),
    ]);
    let prof = profile::Profile::new(profile::SketchPlane::xy(), vec![lp])
        .validate(tol)
        .expect("the die's square");
    let body = sweep::extrude(&prof, sweep::Extrusion::Distance(1.0), Tol::witness())
        .expect("the cube")
        .body;
    let edges: Vec<_> = body.edges().map(|(k, _)| k).collect();
    sweep::fillet::build::fillet_edges(&body, &edges, 0.12, band, Tol::witness())
        .expect("the die blank")
        .body
}

/// The M5 PR 12 pipped die: a SHARP unit cube with 21 spherical
/// dimples — the classical layout, face `n` carrying `n` pips and
/// opposite faces summing to seven — cut in ONE group subtraction.
///
/// Each pip ball has radius 0.09 and is centred 0.09 − 0.05 OUTSIDE
/// its face plane, so the removed volume is exactly a spherical cap of
/// height 0.05, and each ball is charted with its POLE along the
/// cutting face's normal (the split-join's azimuth-anchored arc-side
/// rule needs a polar section — a tilted chart refuses typed). The
/// same recipe the tour's `diepips` stop and
/// `sweep/tests/m5_pr12_die.rs` build, spelled once more here because
/// the fixture corpus builds through the public API only.
pub fn die_pips() -> Body<f64> {
    use core::f64::consts::PI;

    use geom_core::Affine3;
    use profile::ProfileVertex;
    use sweep::{Revolution, RevolveAxis, revolve};
    use topo::boolean::{BooleanOp, SweepStrategy, boolean_op_with};

    const L: f64 = 1.0;
    const PIP_R: f64 = 0.09;
    const PIP_H: f64 = 0.05;
    const PIP_D: f64 = 0.22;

    // A radius-PIP_R ball at the origin, poles on the sketch axis.
    let unit_ball = || -> Body<f64> {
        let lp = ProfileLoop::new(vec![
            ProfileVertex::new(Point2::new(0.0, -PIP_R), 1.0),
            ProfileVertex::new(Point2::new(0.0, PIP_R), 0.0),
        ]);
        let vp = Profile::new(SketchPlane::xy(), vec![lp])
            .validate(Tol::witness())
            .unwrap();
        revolve(
            &vp,
            RevolveAxis {
                origin: Point2::new(0.0, 0.0),
                dir: geom_core::Vec2::new(0.0, 1.0),
            },
            Revolution::Full,
            Tol::witness(),
        )
        .unwrap()
        .body
    };
    // The same ball, rotated so its pole lies along `pole`, then moved
    // to `c`.
    let poled = |c: Vec3<f64>, pole: Vec3<f64>| -> Body<f64> {
        let b = unit_ball();
        let y = Vec3::new(0.0, 1.0, 0.0);
        let axis = y.cross(pole);
        let origin = Point3::new(0.0, 0.0, 0.0);
        let placed = if axis.norm() < 1e-12 {
            if y.dot(pole) > 0.0 {
                b
            } else {
                topo::transform_rigid(
                    &b,
                    &Affine3::rotation_about_axis(origin, Vec3::new(1.0, 0.0, 0.0), PI),
                    Tol::witness(),
                )
                .unwrap()
            }
        } else {
            topo::transform_rigid(
                &b,
                &Affine3::rotation_about_axis(
                    origin,
                    axis.normalize(),
                    y.dot(pole).clamp(-1.0, 1.0).acos(),
                ),
                Tol::witness(),
            )
            .unwrap()
        };
        topo::transform_rigid(&placed, &Affine3::translation(c), Tol::witness()).unwrap()
    };
    // The classical 2-D pip layout of face value `n`, in units of
    // PIP_D about the face centre.
    let layout = |n: u32| -> Vec<(f64, f64)> {
        let c = vec![(0.0, 0.0)];
        let diag = vec![(-1.0, -1.0), (1.0, 1.0)];
        let anti = vec![(-1.0, 1.0), (1.0, -1.0)];
        let sides = vec![(-1.0, 0.0), (1.0, 0.0)];
        match n {
            1 => c,
            2 => diag,
            3 => [diag.clone(), c].concat(),
            4 => [diag.clone(), anti.clone()].concat(),
            5 => [diag.clone(), anti.clone(), c].concat(),
            _ => [diag, anti, sides].concat(),
        }
    };
    let v = Vec3::new;
    let h = L / 2.0;
    // (face value, outward normal, the two in-face axes).
    type Face = (u32, Vec3<f64>, Vec3<f64>, Vec3<f64>);
    let faces: [Face; 6] = [
        (1, v(0.0, 0.0, 1.0), v(1.0, 0.0, 0.0), v(0.0, 1.0, 0.0)),
        (6, v(0.0, 0.0, -1.0), v(1.0, 0.0, 0.0), v(0.0, 1.0, 0.0)),
        (2, v(1.0, 0.0, 0.0), v(0.0, 1.0, 0.0), v(0.0, 0.0, 1.0)),
        (5, v(-1.0, 0.0, 0.0), v(0.0, 1.0, 0.0), v(0.0, 0.0, 1.0)),
        (3, v(0.0, 1.0, 0.0), v(0.0, 0.0, 1.0), v(1.0, 0.0, 0.0)),
        (4, v(0.0, -1.0, 0.0), v(0.0, 0.0, 1.0), v(1.0, 0.0, 0.0)),
    ];
    let mut places = Vec::new();
    for (n, normal, ex, ey) in faces {
        let base = v(h, h, h) + normal * (h + (PIP_R - PIP_H));
        for (u, w) in layout(n) {
            places.push((base + ex * (u * PIP_D) + ey * (w * PIP_D), normal));
        }
    }
    assert_eq!(places.len(), 21, "21 pips, opposite faces summing to 7");

    // One tool of 21 disjoint sphere shells, then ONE subtraction.
    let mut tool = poled(places[0].0, places[0].1);
    for (c, n) in &places[1..] {
        tool = boolean_op_with(
            BooleanOp::Union,
            &tool,
            &poled(*c, *n),
            &topo::BooleanDeclarations::none(),
            SweepStrategy::Realized,
            Tol::witness(),
        )
        .expect("the pip tool assembles")
        .body()
        .expect("a body")
        .body
        .clone();
    }
    assert_eq!(tool.shells().count(), 21, "21 disjoint sphere shells");
    boolean_op_with(
        BooleanOp::Subtract,
        &brick((0.0, L), (0.0, L), (0.0, L)),
        &tool,
        &topo::BooleanDeclarations::none(),
        SweepStrategy::Realized,
        Tol::witness(),
    )
    .expect("the pips cut")
    .body()
    .expect("a body")
    .body
    .clone()
}

/// Census tuple (faces, edges, vertices) of a body — the kernel-side
/// oracle the parse-back reconstruction must match.
pub fn census(body: &Body<f64>) -> (usize, usize, usize) {
    (
        body.faces().count(),
        body.edges().count(),
        body.vertices().count(),
    )
}

/// The M6 composed die (unit 1): [`die_pips`]'s pipped cube filleted
/// IN PLACE — the twelve box edges blended with every pip rim carried
/// through as a ring, then all 21 rims replaced by slit-seamed torus
/// bands. The geometry is `sweep/tests/m6_surgery.rs`'s, constant for
/// constant (blend r = 0.12, rim r = 0.02).
pub fn composed_die() -> Body<f64> {
    use sweep::fillet::build::fillet_edges;

    let tol = Tol::witness();
    let band = geom_core::Band::new(tol.eps(), tol.k() * tol.eps()).expect("band");
    let (die_r, rim_r) = (0.12, 0.02);
    let pipped = die_pips();
    let box_edges: Vec<topo::EdgeKey> = pipped
        .edges()
        .filter(|(_, e)| {
            pipped
                .get_curve_geom(e.curve)
                .and_then(|g| g.certified())
                .is_some_and(|c| matches!(c.carrier(), geom::Curve3::Line { .. }))
        })
        .map(|(k, _)| k)
        .collect();
    let blanked = fillet_edges(&pipped, &box_edges, die_r, band, Tol::witness())
        .expect("the box edges blend in place")
        .body;
    let is_kind = |b: &Body<f64>, f: topo::FaceKey, want_plane: bool| -> bool {
        b.get_face(f)
            .and_then(|fd| b.get_surface(fd.surface))
            .is_some_and(|s| {
                if want_plane {
                    matches!(s, geom::Surface::Plane { .. })
                } else {
                    matches!(s, geom::Surface::Sphere { .. })
                }
            })
    };
    let rims: Vec<topo::EdgeKey> = blanked
        .edges()
        .filter(|(_, e)| {
            let face_of = |he| {
                let h = blanked.get_half_edge(he)?;
                Some(blanked.get_loop(h.parent_loop)?.face)
            };
            match (face_of(e.he_plus), face_of(e.he_minus)) {
                (Some(fa), Some(fb)) => {
                    (is_kind(&blanked, fa, true) && is_kind(&blanked, fb, false))
                        || (is_kind(&blanked, fa, false) && is_kind(&blanked, fb, true))
                }
                _ => false,
            }
        })
        .map(|(k, _)| k)
        .collect();
    fillet_edges(&blanked, &rims, rim_r, band, Tol::witness())
        .expect("the rims blend to torus bands")
        .body
}

/// The M6-3 loft: R5 shape (iii)'s three-section polyline loft —
/// squares at z = 0 and z = 2, a non-affine trapezoid at z = 1,
/// skinned at v-degree 2 (`sweep::loft_body`). The corpus's first
/// NURBS-walled body: 4 described non-rational NURBS walls, 2 planar
/// caps, NURBS seam carriers on the 4 wall–wall edges. Same sections
/// as `sweep/tests/m6_loft_body.rs` (where V = 9 m³ is derived) and
/// the editor-core corpus document `loft_prism`.
pub fn loft_prism() -> Body<f64> {
    let sections = vec![
        quad(PRISM_SQUARE),
        quad(PRISM_TRAPEZOID),
        quad(PRISM_SQUARE),
    ];
    sweep::loft_body::<f64>(&sections, &lofted_at_z(&[0.0, 1.0, 2.0]), 2, Tol::witness())
        .expect("shape (iii) loft builds")
        .body
}

/// A closed four-line quad section (one loop) in the LIB-U3 profile
/// vocabulary — the plainest INTEGRAL profile: unit weights, no arc
/// anywhere.
fn quad(pts: [(f64, f64); 4]) -> sweep::Section {
    vec![ProfileLoop::polygon(
        pts.iter().map(|&(x, y)| Point2::new(x, y)),
    )]
}

/// The prism loft's end section: the square `[-1, 1]²`.
const PRISM_SQUARE: [(f64, f64); 4] = [(-1.0, -1.0), (1.0, -1.0), (1.0, 1.0), (-1.0, 1.0)];
/// Its middle section: the NON-AFFINE trapezoid whose two bottom
/// corners flare by ±d, d = 0.375 — what makes the walls genuinely
/// curved in v rather than ruled.
const PRISM_TRAPEZOID: [(f64, f64); 4] = [(-1.375, -1.0), (1.375, -1.0), (1.0, 1.0), (-1.0, 1.0)];

/// Section placements: pure translations up the world z-axis.
fn lofted_at_z(zs: &[f64]) -> Vec<geom_core::Affine3<f64>> {
    zs.iter()
        .map(|z| geom_core::Affine3::translation(Vec3::new(0.0, 0.0, *z)))
        .collect()
}

/// **The non-uniform loft (#210 / #207).** [`loft_prism`]'s own three
/// sections, re-placed at z = 0, 1, **3** — spacing 1 : 2 instead of
/// 1 : 1. That single change is the whole fixture: until #207 the skin
/// fit synthesized a weight channel whose LU round-trip landed an ulp
/// off 1.0 on exactly this parameterization, making the walls bitwise
/// RATIONAL and the body refuse at assembly. This is the corpus's
/// minimal pair with `loft_prism` — same sections, same degree, same
/// builder, non-uniform spacing.
///
/// **The v-parameterization is NOT `[0, ⅓, 1]`.** `skin_parameters`
/// averages cumulative **chord** lengths, not z-spacings, and the
/// trapezoid's ±0.375 flare lengthens the first chord: both rows of the
/// first strip travel `√(0.375² + 1²) = √73/8` then
/// `√(0.375² + 2²) = √265/8`, so the average is exact and the middle
/// parameter is
///
/// ```text
/// t = √73 / (√73 + √265) = 0.34419950074181277
/// ```
///
/// The fixture ASKS `sweep::loft_parameters` for that value and pins
/// this derivation against it (LIB-U5), so the algebra here is a
/// cross-check rather than the only record of what the skin chose.
///
/// The naive `⅓` would put the volume at 13.6875 m³ — out by 1.9e-3
/// relative, 1.6e8 times the certified pad. Carrying the real `t`
/// through the quadratic Lagrange fit (one Bézier span; slices are
/// planar trapezoids of area `4 + 2d·L1(v)`, d = 0.375) gives
///
/// ```text
/// V = 12 + 0.375 / (t(1 − t)) = 12.75 + 126.75/√19345
///   = 13.661304680798798 m³
/// ```
///
/// which is the derived volume. The `.expect` sidecar carries the
/// integration step by step and pins it against the kernel.
pub fn nonuniform_loft() -> Body<f64> {
    let sections = vec![
        quad(PRISM_SQUARE),
        quad(PRISM_TRAPEZOID),
        quad(PRISM_SQUARE),
    ];
    let places = lofted_at_z(&[0.0, 1.0, 3.0]);
    // The `t` the doc comment derives above, ASKED rather than
    // re-derived (LIB-U5 deliverable 1).
    assert_eq!(
        sweep::loft_parameters(&sections, &places, 2, Tol::witness()).expect("the sections skin"),
        vec![0.0, NONUNIFORM_T, 1.0],
        "the derived v-parameterization is no longer what the skin chose"
    );
    sweep::loft_body::<f64>(&sections, &places, 2, Tol::witness())
        .expect("the non-uniform loft builds")
        .body
}

/// The middle section's v-parameter, `√73 / (√73 + √265)` — the pin
/// the derivation above rests on, checked against `loft_parameters`
/// every time the fixture builds.
const NONUNIFORM_T: f64 = 0.34419950074181277;

/// **The swept elbow (#210 / #207): the corpus's first CURVED-PATH
/// sweep.** A square profile of half-width 0.25 swept along a 90° arc
/// of radius 3 in the world YZ plane, at 9 stations, skinned at
/// v-degree 3 (`sweep::sweep_body`). `sweep_body` had zero successful
/// callers anywhere in the tree before #207: every curved path drove
/// the same synthesized-weight drift the non-uniform loft did.
///
/// Construction is `sweep/tests/m7_skin_integral.rs`'s elbow, constant
/// for constant (that suite derives the Pappus bracket; the
/// `step-export` twin in `tests/m7_swept_elbow.rs` pins the wire's
/// non-rationality). Duplicated across the crate boundary exactly as
/// [`loft_prism`] duplicates `sweep/tests/m6_loft_body.rs`.
pub fn swept_elbow() -> Body<f64> {
    /// Path radius.
    const R: f64 = 3.0;
    /// Profile half-width.
    const H: f64 = 0.25;
    // The sketch arc runs (0,0) → (R,R) with bulge = tan(θ/4) =
    // tan(π/8), a 90° turn; the placement rotates the sketch plane by
    // −π/2 about the world y-axis, sending sketch (x, y) to world
    // (0, y, x). So the path leaves the origin with tangent +z — the
    // identity-placed profile (world XY plane) is already normal to it.
    let path = sweep::skin::segment_curve(
        0,
        sweep::SketchSegment::Arc {
            a: Point2::new(0.0, 0.0),
            b: Point2::new(R, R),
            bulge: (core::f64::consts::PI / 8.0).tan(),
        },
        geom_core::Affine3::rotation_about_axis(
            Point3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
            -core::f64::consts::FRAC_PI_2,
        ),
    )
    .expect("the elbow path is a well-formed quarter arc");
    sweep::sweep_body::<f64>(
        &quad([(-H, -H), (H, -H), (H, H), (-H, H)]),
        geom_core::Affine3::identity(),
        &path,
        9,
        3,
        Tol::witness(),
    )
    .expect("the curved-path sweep body builds")
    .body
}
