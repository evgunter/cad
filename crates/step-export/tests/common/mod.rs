//! Shared acceptance-body builders for the STEP export suites, via the
//! public profile/sweep/boolean APIs only (the same shapes as the M3
//! STL review suites: bricks, the pocketed die, the corner-kiss
//! assembly, the voided subtract).
#![allow(dead_code)] // each consumer uses a subset
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Point2, Point3, Tolerance, Vec3};
use profile::{Profile, ProfileLoop, SketchPlane, ValidatedProfile};
use sweep::{Extrusion, extrude};
use topo::{Body, BooleanResult, BooleanResultKind, subtract, union};

fn validated(plane: SketchPlane<f64>, lp: ProfileLoop<f64>) -> ValidatedProfile<f64> {
    Profile::new(plane, vec![lp])
        .validate(Tolerance::get())
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
    let BooleanResult::Body(b) = subtract(&cube, &cutter).unwrap() else {
        panic!("die subtract is a body");
    };
    b.body
}

/// Two pocketed dies kissing at the corner `(1,1,1)` — the M3 R6
/// assembly: one solid, TWO shells (both outward), exact volume 1.75.
pub fn kiss_assembly() -> Body<f64> {
    let d1 = die(0.0, 0.0, 0.0);
    let d2 = die(1.0, 1.0, 1.0);
    let BooleanResult::Body(assembly) = union(&d1, &d2).unwrap() else {
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
    let BooleanResult::Body(result) = subtract(&a, &b).unwrap() else {
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
        ProfileVertex {
            pos: Point2::new(0.0, -1.0),
            bulge: 1.0,
        },
        ProfileVertex {
            pos: Point2::new(0.0, 1.0),
            bulge: 0.0,
        },
    ]);
    let profile = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(Tolerance::get())
        .unwrap();
    revolve(&profile, revolve_y(), Revolution::Full)
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
        .validate(Tolerance::get())
        .unwrap();
    revolve(&profile, revolve_y(), Revolution::Full)
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
        ProfileVertex {
            pos: Point2::new(2.0, -0.5),
            bulge: 1.0,
        },
        ProfileVertex {
            pos: Point2::new(2.0, 0.5),
            bulge: 1.0,
        },
    ]);
    let profile = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(Tolerance::get())
        .unwrap();
    revolve(&profile, revolve_y(), Revolution::Full)
        .unwrap()
        .body
}

/// A revolved washer: the rectangle [1,2]×[0,1] swept fully — genus 1,
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
        .validate(Tolerance::get())
        .unwrap();
    revolve(&profile, revolve_y(), Revolution::Full)
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
        ProfileVertex {
            pos: Point2::new(-1.0, 0.0),
            bulge: 1.0,
        },
        ProfileVertex {
            pos: Point2::new(1.0, 0.0),
            bulge: 1.0,
        },
    ]);
    let profile = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(Tolerance::get())
        .unwrap();
    let cylinder = extrude(&profile, Extrusion::Distance(2.5)).unwrap().body;
    let phi: f64 = 0.3;
    let plane = SplitPlane {
        origin: Point3::new(0.0, 0.0, 1.25),
        normal: Vec3::new(phi.sin(), 0.0, phi.cos()),
    };
    let result = split(&cylinder, &plane).unwrap();
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
    )
    .unwrap()
    .body;
    let b120 = (core::f64::consts::PI / 6.0).tan();
    let at = |deg: f64| {
        let th: f64 = deg.to_radians();
        Point2::new(2.0 + 0.5 * th.cos(), 2.0 + 0.5 * th.sin())
    };
    let boss_loop = ProfileLoop::new(vec![
        ProfileVertex {
            pos: at(0.0),
            bulge: b120,
        },
        ProfileVertex {
            pos: at(120.0),
            bulge: b120,
        },
        ProfileVertex {
            pos: at(240.0),
            bulge: b120,
        },
    ]);
    let sketch = SketchPlane::new(Affine3::translation(Vec3::new(0.0, 0.0, 0.4)));
    let boss_profile = Profile::new(sketch, vec![boss_loop])
        .validate(Tolerance::get())
        .unwrap();
    let boss = extrude(&boss_profile, Extrusion::Distance(1.2))
        .unwrap()
        .body;
    let BooleanResult::Body(bb) = union(&plate, &boss).unwrap() else {
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
    let lp = ProfileLoop::builder(Point2::new(0.0, 0.0))
        .arc_to(Point2::new(2.0, 0.0), b)
        .line_to(Point2::new(2.0, 1.5))
        .arc_to(Point2::new(0.0, 1.5), -b)
        .close();
    extrude(&validated(SketchPlane::xy(), lp), Extrusion::Distance(1.0))
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
    )
    .unwrap()
    .body;
    let r = 0.35;
    let bulge = (PI / 6.0).tan();
    let at = |i: usize| {
        let th = 2.0 * PI / 3.0 * i as f64;
        Point2::new(1.2 + r * th.cos(), 1.7 + r * th.sin())
    };
    let boss_loop = ProfileLoop::new(
        (0..3)
            .map(|i| ProfileVertex { pos: at(i), bulge })
            .collect(),
    );
    let sketch = SketchPlane::new(Affine3::translation(Vec3::new(0.0, 0.0, -0.2)));
    let boss = extrude(
        &Profile::new(sketch, vec![boss_loop])
            .validate(Tolerance::get())
            .unwrap(),
        Extrusion::Distance(1.2),
    )
    .unwrap()
    .body;
    let BooleanResult::Body(stubs) = subtract(&boss, &plate).unwrap() else {
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
pub fn certified_carrier(body: &Body<f64>, edge: topo::EdgeKey) -> &geom_curves::Curve3<f64> {
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
/// so a row added here needs one written by hand).
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
        // The M5 PR 12 fillet set: the die blank carries all five
        // elementary surface kinds' hardest pairing for a writer —
        // plane, cylinder AND sphere faces meeting along TANGENT
        // trimlines, straight ones and circular ones.
        ("filleted_die", filleted_die()),
        // The M6 composed die: blank + 21 pips + 21 rim TORUS bands
        // in one body (the composition surgery). Adds the writer's
        // first fillet-minted TOROIDAL_SURFACEs (slit-seamed annuli)
        // alongside every kind the blank already carries.
        ("composed_die", composed_die()),
    ]
}

/// The M5 PR 12 die blank: a unit cube with every edge blended at
/// r = 0.12 — 6 shrunk planes, 12 quarter-cylinders, 8 sphere octants.
pub fn filleted_die() -> Body<f64> {
    let tol = geom_core::Tolerance::get();
    let band = geom_core::Band::new(tol.eps, tol.k * tol.eps).expect("band");
    let lp = profile::ProfileLoop::polygon([
        Point2::new(0.0, 0.0),
        Point2::new(1.0, 0.0),
        Point2::new(1.0, 1.0),
        Point2::new(0.0, 1.0),
    ]);
    let prof = profile::Profile::new(profile::SketchPlane::xy(), vec![lp])
        .validate(tol)
        .expect("the die's square");
    let body = sweep::extrude(&prof, sweep::Extrusion::Distance(1.0))
        .expect("the cube")
        .body;
    let edges: Vec<_> = body.edges().map(|(k, _)| k).collect();
    sweep::fillet::build::fillet_edges(&body, &edges, 0.12, band)
        .expect("the die blank")
        .body
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

/// The M6 composed die: the pipped cube (21 balls, one group cut)
/// filleted IN PLACE — twelve box-edge blends with the rims carried
/// through as rings, then all 21 rims replaced by torus bands. The
/// geometry is `sweep/tests/m6_surgery.rs`'s, constant for constant.
pub fn composed_die() -> Body<f64> {
    use profile::ProfileVertex;
    use sweep::fillet::build::fillet_edges;
    use sweep::{Revolution, RevolveAxis, revolve};
    use topo::BooleanDeclarations;
    use topo::boolean::{BooleanOp, SweepStrategy, boolean_op_with};

    let tol = Tolerance::get();
    let band = geom_core::Band::new(tol.eps, tol.k * tol.eps).expect("band");
    let (l, pip_r, pip_h, pip_d, die_r, rim_r) = (1.0, 0.09, 0.05, 0.22, 0.12, 0.02);

    let ball_at = |c: geom_core::Vec3<f64>| -> Body<f64> {
        let lp = ProfileLoop::new(vec![
            ProfileVertex {
                pos: Point2::new(0.0, -pip_r),
                bulge: 1.0,
            },
            ProfileVertex {
                pos: Point2::new(0.0, pip_r),
                bulge: 0.0,
            },
        ]);
        let vp = Profile::new(SketchPlane::xy(), vec![lp])
            .validate(Tolerance::get())
            .unwrap();
        let axis = RevolveAxis {
            origin: Point2::new(0.0, 0.0),
            dir: geom_core::Vec2::new(0.0, 1.0),
        };
        let b = revolve(&vp, axis, Revolution::Full).unwrap().body;
        topo::transform_rigid(&b, &geom_core::Affine3::translation(c)).unwrap()
    };
    let poled = |c: geom_core::Vec3<f64>, pole: geom_core::Vec3<f64>| -> Body<f64> {
        use core::f64::consts::PI;
        let b = ball_at(geom_core::Vec3::new(0.0, 0.0, 0.0));
        let y = geom_core::Vec3::new(0.0, 1.0, 0.0);
        let rot = y.cross(pole);
        let origin = geom_core::Point3::new(0.0, 0.0, 0.0);
        let placed = if rot.norm() < 1e-12 {
            if y.dot(pole) > 0.0 {
                b
            } else {
                topo::transform_rigid(
                    &b,
                    &geom_core::Affine3::rotation_about_axis(
                        origin,
                        geom_core::Vec3::new(1.0, 0.0, 0.0),
                        PI,
                    ),
                )
                .unwrap()
            }
        } else {
            topo::transform_rigid(
                &b,
                &geom_core::Affine3::rotation_about_axis(
                    origin,
                    rot.normalize(),
                    y.dot(pole).clamp(-1.0, 1.0).acos(),
                ),
            )
            .unwrap()
        };
        topo::transform_rigid(&placed, &geom_core::Affine3::translation(c)).unwrap()
    };

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
    let h = l / 2.0;
    let v = geom_core::Vec3::new;
    let faces = [
        (1u32, v(0.0, 0.0, 1.0), v(1.0, 0.0, 0.0), v(0.0, 1.0, 0.0)),
        (6, v(0.0, 0.0, -1.0), v(1.0, 0.0, 0.0), v(0.0, 1.0, 0.0)),
        (2, v(1.0, 0.0, 0.0), v(0.0, 1.0, 0.0), v(0.0, 0.0, 1.0)),
        (5, v(-1.0, 0.0, 0.0), v(0.0, 1.0, 0.0), v(0.0, 0.0, 1.0)),
        (3, v(0.0, 1.0, 0.0), v(0.0, 0.0, 1.0), v(1.0, 0.0, 0.0)),
        (4, v(0.0, -1.0, 0.0), v(0.0, 0.0, 1.0), v(1.0, 0.0, 0.0)),
    ];
    let mut places = Vec::new();
    for (n, normal, ex, ey) in faces {
        let base = v(h, h, h) + normal * (h + (pip_r - pip_h));
        for (u, w) in layout(n) {
            places.push((base + ex * (u * pip_d) + ey * (w * pip_d), normal));
        }
    }
    let mut tool = poled(places[0].0, places[0].1);
    for (c, n) in &places[1..] {
        tool = boolean_op_with(
            BooleanOp::Union,
            &tool,
            &poled(*c, *n),
            &BooleanDeclarations::none(),
            SweepStrategy::Realized,
        )
        .expect("the pip tool assembles")
        .body()
        .expect("a body")
        .body
        .clone();
    }
    let pipped = boolean_op_with(
        BooleanOp::Subtract,
        &brick((0.0, l), (0.0, l), (0.0, l)),
        &tool,
        &BooleanDeclarations::none(),
        SweepStrategy::Realized,
    )
    .expect("the pips cut")
    .body()
    .expect("a body")
    .body
    .clone();

    let box_edges: Vec<topo::EdgeKey> = pipped
        .edges()
        .filter(|(_, e)| {
            pipped
                .get_curve_geom(e.curve)
                .and_then(|g| g.certified())
                .is_some_and(|c| matches!(c.carrier(), geom_curves::Curve3::Line { .. }))
        })
        .map(|(k, _)| k)
        .collect();
    let blanked = fillet_edges(&pipped, &box_edges, die_r, band)
        .expect("the box edges blend in place")
        .body;
    let is_kind = |b: &Body<f64>, f: topo::FaceKey, want_plane: bool| -> bool {
        b.get_face(f)
            .and_then(|fd| b.get_surface(fd.surface))
            .is_some_and(|s| {
                if want_plane {
                    matches!(s, geom_surfaces::Surface::Plane { .. })
                } else {
                    matches!(s, geom_surfaces::Surface::Sphere { .. })
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
    fillet_edges(&blanked, &rims, rim_r, band)
        .expect("the rims blend to torus bands")
        .body
}
