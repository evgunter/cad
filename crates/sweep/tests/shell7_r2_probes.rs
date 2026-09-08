//! **SHELL-7 review probes (R2).** Rows that try to FALSIFY the unit's
//! claims, plus the consumer's-seat end-to-end exercise. Nothing here
//! is a gate for the unit; every row states what it measured.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::PI;

use geom::Curve3;
use geom_core::{Point2, Point3, Tol, Vec2, Vec3};
use profile::{Profile, ProfileLoop, ProfileVertex, RawLoop, SketchPlane};
use sweep::{Revolution, RevolveAxis, TubeWindow, revolve, tube_along_arc, tube_along_arc_hollow};
use topo::{Body, EdgeKey, ReplaceFaceError, ShellError, VertexKey};

const R: f64 = 2.0;
const SMALL_R: f64 = 0.5;

fn p2(x: f64, y: f64) -> Point2<f64> {
    Point2::new(x, y)
}

fn tol() -> Tol {
    Tol::witness()
}

fn axial(p: Point3<f64>) -> (f64, f64) {
    ((p.x * p.x + p.z * p.z).sqrt(), p.y)
}

fn point(body: &Body<f64>, v: VertexKey) -> Point3<f64> {
    *body
        .get_point(body.get_vertex(v).expect("vertex").point)
        .expect("point")
}

fn carrier(body: &Body<f64>, e: EdgeKey) -> (Curve3<f64>, (f64, f64)) {
    let c = body
        .get_curve_geom(body.get_edge(e).expect("edge").curve)
        .and_then(|g| g.certified())
        .expect("a certified curve");
    (c.carrier().clone(), c.params())
}

fn try_revolved(lp: ProfileLoop<f64>, turn: Revolution<f64>) -> Option<Body<f64>> {
    let profile = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(tol())
        .ok()?;
    Some(
        revolve(
            &profile,
            RevolveAxis {
                origin: p2(0.0, 0.0),
                dir: Vec2::new(0.0, 1.0),
            },
            turn,
            tol(),
        )
        .ok()?
        .body,
    )
}

fn revolved(lp: ProfileLoop<f64>, turn: Revolution<f64>) -> Body<f64> {
    let profile = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(tol())
        .expect("the meridian validates");
    revolve(
        &profile,
        RevolveAxis {
            origin: p2(0.0, 0.0),
            dir: Vec2::new(0.0, 1.0),
        },
        turn,
        tol(),
    )
    .expect("the meridian revolves")
    .body
}

fn wedge(r: f64, h: f64, turn: f64) -> Body<f64> {
    revolved(
        ProfileLoop::new(vec![
            ProfileVertex::new(p2(0.0, 0.0), 0.0),
            ProfileVertex::new(p2(r, 0.0), 0.0),
            ProfileVertex::new(p2(r, h), 0.0),
            ProfileVertex::new(p2(0.0, h), 0.0),
        ]),
        Revolution::Partial(turn),
    )
}

fn tube_torus(major: f64, minor: f64) -> Body<f64> {
    tube_along_arc::<f64>(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::unit_y(),
        Vec3::unit_x(),
        major,
        TubeWindow::Full,
        minor,
        tol(),
    )
    .expect("the solid torus builds")
    .body
}

/// Split `edge` at its parameter midpoint; the new vertex.
fn split_mid(body: &mut Body<f64>, edge: EdgeKey) -> VertexKey {
    let (_, (t0, t1)) = carrier(body, edge);
    body.split_edge(edge, (t0 + t1) * 0.5, tol())
        .expect("the edge splits")
        .vertex
}

fn corner_refusal(e: &ShellError<f64>) -> (VertexKey, usize, &'static str) {
    let ShellError::Face { error, .. } = e else {
        panic!("expected the axial door's refusal, got {e}");
    };
    let ReplaceFaceError::TogetherAxialCorner {
        vertex,
        surfaces,
        what,
    } = **error
    else {
        panic!("expected TogetherAxialCorner, got {error}");
    };
    (vertex, surfaces, what)
}

// ---------------------------------------------------------------------
// P1 — claim 2: is `(Line profile, one meridian)` reachable, and is the
// new `what` true of what reaches it?
//
// The unit calls this arm "unreached by any fixture ... no door builds
// either". A quarter-turn wedge's WALL–CAP generator, split at its
// midpoint, is a vertex whose surfaces are exactly the cylinder (a Line
// profile) and one meridian cap. It reaches the arm.
// ---------------------------------------------------------------------

#[test]
fn p1_a_line_profile_beside_one_meridian_cap_is_reachable_by_a_hand_split() {
    let (r, h) = (1.0, 2.0);
    let mut body = wedge(r, h, PI / 2.0);
    // The generator where the cylinder wall meets a meridian cap: a
    // LINE along the axis direction standing at radius r, whose two
    // sides are DIFFERENT surfaces (unlike the drum's seam).
    let hits: Vec<EdgeKey> = body
        .edges()
        .filter(|(e, data)| {
            let f = |he| {
                let face = body
                    .get_loop(body.get_half_edge(he).unwrap().parent_loop)
                    .unwrap()
                    .face;
                body.get_face(face).unwrap().surface
            };
            if f(data.he_plus) == f(data.he_minus) {
                return false;
            }
            match carrier(&body, *e).0 {
                Curve3::Line { origin, dir } => {
                    (axial(origin).0 - r).abs() <= 1e-12
                        && dir.dot(Vec3::unit_y()).abs() >= 1.0 - 1e-12
                }
                _ => false,
            }
        })
        .map(|(e, _)| e)
        .collect();
    assert_eq!(hits.len(), 2, "two wall/cap generators, got {hits:?}");
    let split = split_mid(&mut body, hits[0]);
    topo::mint_pcurves(&mut body, tol()).expect("pcurves mint");
    assert_eq!(topo::validate_geometric(&body, tol()), Ok(()), "operand");
    let e = topo::shell(&body, 0.05, tol()).expect_err("the split wedge refuses");
    let (vertex, surfaces, what) = corner_refusal(&e);
    assert_eq!(vertex, split, "the refusal names the split vertex");
    eprintln!("[p1] surfaces = {surfaces}, what = {what:?}");
    assert!(
        what.starts_with("one profile constraint and a plane containing the axis"),
        "expected the line-beside-a-meridian refusal, got {what:?}"
    );
}

// ---------------------------------------------------------------------
// P2 — claim 6: the closed form at a thickness approaching the tube's
// own minor radius. `r − t → 0`; what refuses, and does it name the
// right thing?
// ---------------------------------------------------------------------

#[test]
fn p2_the_torus_at_a_thickness_approaching_its_minor_radius() {
    for t in [0.4, 0.49, 0.499_999, SMALL_R, 0.6] {
        let body = tube_torus(R, SMALL_R);
        match topo::shell(&body, t, tol()) {
            Ok(out) => {
                let props = topo::mass_properties(&out.body, tol()).expect("props");
                let want = 2.0 * PI * PI * R * (SMALL_R * SMALL_R - (SMALL_R - t) * (SMALL_R - t));
                eprintln!(
                    "[p2] t = {t}: shells, volume {} (pad {}), closed form {want}, err {:e}",
                    props.volume,
                    props.volume_pad,
                    (props.volume - want).abs()
                );
                assert!(
                    (props.volume - want).abs() <= 1e-9 + props.volume_pad,
                    "t = {t}: volume off the closed form"
                );
            }
            Err(e) => eprintln!("[p2] t = {t}: refuses — {e}"),
        }
    }
}

// ---------------------------------------------------------------------
// P3 — claim 3: is the meridian seam / latitude seam decide confusable?
// `offset_axial_seam_latitude` decides `|radial(centre)|` against the
// band with NO lever. A meridian seam's centre stands at radial R; a
// latitude seam's at 0. Shrink R toward the band and see what happens.
// ---------------------------------------------------------------------

/// A full torus built through the REVOLVE door from two semicircular
/// arcs — the fixture `shell7_seam_corner` uses — at a major radius
/// small enough that `offset_axial_seam_latitude`'s unlevered
/// `Margin::of(R)` could read Zero and route a MERIDIAN seam to
/// `latitude_carrier`.
#[test]
fn p3b_a_band_scale_major_radius_through_the_revolve_door() {
    for (major, minor) in [(2.0e-8, 5.0e-9), (2.0e-9, 5.0e-10), (1.0e-10, 2.0e-11)] {
        let a = p2(major + minor, 0.0);
        let b = p2(major - minor, 0.0);
        let lp = RawLoop::new(vec![ProfileVertex::new(a, 1.0), ProfileVertex::new(b, 1.0)]);
        let profile = match Profile::new(SketchPlane::xy(), vec![lp]).validate(tol()) {
            Ok(p) => p,
            Err(e) => {
                eprintln!("[p3b] R = {major:e}: the profile refuses — {e:?}");
                continue;
            }
        };
        let built = revolve(
            &profile,
            RevolveAxis {
                origin: p2(0.0, 0.0),
                dir: Vec2::new(0.0, 1.0),
            },
            Revolution::Full,
            tol(),
        );
        let Ok(rev) = built else {
            eprintln!("[p3b] R = {major:e}: the revolve door refuses");
            continue;
        };
        match topo::shell(&rev.body, minor * 0.1, tol()) {
            Ok(out) => {
                let props = topo::mass_properties(&out.body, tol());
                eprintln!("[p3b] R = {major:e}: SHELLS — volume {props:?}");
            }
            Err(e) => eprintln!("[p3b] R = {major:e}: refuses — {e}"),
        }
    }
}

#[test]
fn p3_the_seam_decide_at_small_major_radii() {
    for (major, minor) in [
        (2.0, 0.5),
        (0.02, 0.005),
        (2.0e-4, 5.0e-5),
        (2.0e-6, 5.0e-7),
        (2.0e-8, 5.0e-9),
    ] {
        let built = tube_along_arc::<f64>(
            Point3::new(0.0, 0.0, 0.0),
            Vec3::unit_y(),
            Vec3::unit_x(),
            major,
            TubeWindow::Full,
            minor,
            tol(),
        );
        let Ok(rev) = built else {
            eprintln!("[p3] R = {major:e}, r = {minor:e}: the tube door refuses");
            continue;
        };
        let t = minor * 0.1;
        match topo::shell(&rev.body, t, tol()) {
            Ok(out) => {
                let props = topo::mass_properties(&out.body, tol()).expect("props");
                let want = 2.0 * PI * PI * major * (minor * minor - (minor - t) * (minor - t));
                eprintln!(
                    "[p3] R = {major:e}, r = {minor:e}: shells, volume {:e} want {want:e} \
                     relerr {:e}",
                    props.volume, props.volume_pad,
                );
                assert!(
                    (props.volume - want).abs() <= 1e-9 + props.volume_pad,
                    "R = {major:e}: volume {} off closed form {want}",
                    props.volume
                );
            }
            Err(e) => eprintln!("[p3] R = {major:e}, r = {minor:e}: refuses — {e}"),
        }
    }
}

// ---------------------------------------------------------------------
// P4 — claim 7: `Body::split_edge` leaves its children without pcurve
// rows, so a hand-split operand is tier-3 invalid until `mint_pcurves`.
// ---------------------------------------------------------------------

#[test]
fn p4_split_edge_leaves_its_children_without_pcurves() {
    // (a) The wedge's AXIS edge — the operand of the PR's row 5, which
    // does NOT call `mint_pcurves`.
    let mut w = wedge(1.0, 2.0, PI / 2.0);
    assert_eq!(topo::validate_geometric(&w, tol()), Ok(()), "unsplit wedge");
    let axis_edge: EdgeKey = w
        .edges()
        .find(|(e, _)| match carrier(&w, *e).0 {
            Curve3::Line { origin, dir } => {
                axial(origin).0 <= 1e-15 && dir.dot(Vec3::unit_y()).abs() >= 1.0 - 1e-15
            }
            _ => false,
        })
        .map(|(e, _)| e)
        .expect("the axis edge");
    split_mid(&mut w, axis_edge);
    let after_axis = topo::validate_geometric(&w, tol());
    eprintln!("[p4a] wedge axis edge, tier 3 after split_edge: {after_axis:?}");

    // (b) The drum's cylinder seam — the operand of the PR's row 6.
    let (r, h) = (1.0, 2.0);
    let mut d = revolved(
        ProfileLoop::new(vec![
            ProfileVertex::new(p2(0.0, 0.0), 0.0),
            ProfileVertex::new(p2(r, 0.0), 0.0),
            ProfileVertex::new(p2(r, h), 0.0),
            ProfileVertex::new(p2(0.0, h), 0.0),
        ]),
        Revolution::Full,
    );
    assert_eq!(topo::validate_geometric(&d, tol()), Ok(()), "unsplit drum");
    let seam: EdgeKey = d
        .edges()
        .find(|(e, data)| {
            let f = |he| {
                let face = d
                    .get_loop(d.get_half_edge(he).unwrap().parent_loop)
                    .unwrap()
                    .face;
                d.get_face(face).unwrap().surface
            };
            if f(data.he_plus) != f(data.he_minus) {
                return false;
            }
            match carrier(&d, *e).0 {
                Curve3::Line { origin, dir } => {
                    axial(origin).0 > 0.5 && dir.dot(Vec3::unit_y()).abs() >= 1.0 - 1e-15
                }
                _ => false,
            }
        })
        .map(|(e, _)| e)
        .expect("the drum seam");
    split_mid(&mut d, seam);
    let after_seam = topo::validate_geometric(&d, tol());
    eprintln!("[p4b] drum cylinder seam, tier 3 after split_edge: {after_seam:?}");
    assert!(
        after_seam.is_err(),
        "the PR's TOPO finding: a split CURVED chart leaves no pcurve rows"
    );
    topo::mint_pcurves(&mut d, tol()).expect("pcurves mint");
    assert_eq!(topo::validate_geometric(&d, tol()), Ok(()));
}

// ---------------------------------------------------------------------
// P5 — claim 1, the arithmetic: what does `image_of`'s foot do to a
// vertex `δ` off its own surface, and can the end-of-solve residual see
// it? Replicated from `Profile::image_of` (a private fn) at the same
// evaluation order.
// ---------------------------------------------------------------------

#[test]
fn p5_the_foot_snaps_a_delta_off_vertex_and_the_concurrence_meter_cannot_see_it() {
    // The moved wall of a drum of radius 1 shelled by t: `ρ = 1 − t`,
    // as `n̂·(ρ, h) = c` with n̂ = (1, 0).
    let (n, c) = ((1.0f64, 0.0f64), 1.0 - 0.05);
    for delta in [0.0, 1e-12, 1e-9, 1e-7] {
        // A vertex δ OFF the old wall (ρ = 1), normal to it.
        let (rho, h) = (1.0 + delta, 1.0);
        let gap = n.0 * rho + n.1 * h - c;
        let foot = (rho - gap * n.0, h - gap * n.1);
        // The true normal move of the surface point it is nearest.
        let normal_move = (1.0 - 0.05, 1.0);
        let err = (foot.0 - normal_move.0).hypot(foot.1 - normal_move.1);
        // `Profile::residual` of the answer against the moved line.
        let residual = n.0 * foot.0 + n.1 * foot.1 - c;
        eprintln!(
            "[p5] delta = {delta:e}: foot = {foot:?}, err vs normal move = {err:e}, \
             concurrence residual = {residual:e}"
        );
        assert!(err <= delta + 1e-15, "the error is not O(delta)");
        assert_eq!(
            residual, 0.0,
            "the foot lands EXACTLY on the moved line, so the \
             `offset_axial_concurrence` meter reads zero whatever delta was"
        );
    }
}

// ---------------------------------------------------------------------
// E2E — the consumer's seat (required by the brief).
// ---------------------------------------------------------------------

#[test]
fn e2e_a_consumer_shells_classifies_measures_and_tessellates_tori() {
    let t = 0.05;
    let w = 0.125;

    // 1. The SOLID torus.
    let solid = tube_torus(R, SMALL_R);
    let shelled = topo::shell(&solid, t, tol()).expect("the solid torus shells");
    assert_eq!(topo::validate_geometric(&shelled.body, tol()), Ok(()));
    let cls = topo::classify_shells(&shelled.body, tol()).expect("shells classify");
    let roles: Vec<String> = cls.iter().map(|c| format!("{:?}", c.role)).collect();
    let props = topo::mass_properties(&shelled.body, tol()).expect("props");
    let want = 2.0 * PI * PI * R * (SMALL_R * SMALL_R - (SMALL_R - t) * (SMALL_R - t));
    eprintln!(
        "[e2e] solid torus shelled: {} shells {roles:?}, volume {} want {want} err {:e}",
        shelled.body.shells().count(),
        props.volume,
        (props.volume - want).abs()
    );
    assert!((props.volume - want).abs() <= 1e-9 + props.volume_pad);
    let mesh = mesh::tessellate(&shelled.body, 0.01, tol()).expect("the shelled torus meshes");
    eprintln!(
        "[e2e] mesh: {} vertices, {} triangles",
        mesh.positions.len(),
        mesh.patches
            .iter()
            .map(|p| p.triangles.len())
            .sum::<usize>()
    );

    // 2. The HOLLOW torus, shelled.
    let hollow = tube_along_arc_hollow::<f64>(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::unit_y(),
        Vec3::unit_x(),
        R,
        TubeWindow::Full,
        SMALL_R,
        w,
        tol(),
    )
    .expect("the hollow torus builds")
    .body;
    assert_eq!(hollow.solids().count(), 1, "one solid, two shells");
    let hs = topo::shell(&hollow, t, tol()).expect("the hollow torus shells");
    assert_eq!(topo::validate_geometric(&hs.body, tol()), Ok(()));
    let hp = topo::mass_properties(&hs.body, tol()).expect("props");
    let hwant = 2.0
        * PI
        * PI
        * R
        * ((SMALL_R * SMALL_R - (SMALL_R - t) * (SMALL_R - t))
            + ((SMALL_R - w + t) * (SMALL_R - w + t) - (SMALL_R - w) * (SMALL_R - w)));
    eprintln!(
        "[e2e] hollow torus shelled: {} solids, {} shells, volume {} want {hwant} err {:e}",
        hs.body.solids().count(),
        hs.body.shells().count(),
        hp.volume,
        (hp.volume - hwant).abs()
    );
    assert!((hp.volume - hwant).abs() <= 1e-9 + hp.volume_pad);

    // 3. Hollow the hollow one AGAIN — SHELL-5's semantics from a
    // consumer's seat.
    match topo::shell(&hs.body, 0.01, tol()) {
        Ok(_) => eprintln!("[e2e] the twice-shelled hollow torus shells again"),
        Err(e) => eprintln!("[e2e] shelling the shelled hollow torus refuses — {e}"),
    }

    // 4. `shell_open` on a torus: no planar face to designate. The
    //    only faces are the two torus half-walls.
    let faces: Vec<topo::FaceKey> = solid.faces().map(|(k, _)| k).collect();
    eprintln!(
        "[e2e] the solid torus offers {} faces to designate",
        faces.len()
    );
    let any = *faces.first().expect("a face");
    match topo::shell_open(&solid, t, &[any], tol()) {
        Ok(out) => eprintln!(
            "[e2e] shell_open on a torus face: {} shells, {} rims",
            out.body.shells().count(),
            out.naming.rims.len()
        ),
        Err(e) => eprintln!("[e2e] shell_open on a torus face refuses — {e}"),
    }
}

// ---------------------------------------------------------------------
// P6 — claim 5 shape: an independent differential over a corpus the
// lane's dump did not choose, printed as `[r2dump]` rows for a
// merge-base comparison.
// ---------------------------------------------------------------------

fn dump(what: &str, body: &Body<f64>) {
    let mut vs: Vec<String> = body
        .vertices()
        .map(|(k, _)| {
            let p = point(body, k);
            format!("v {:.17e} {:.17e} {:.17e}", p.x, p.y, p.z)
        })
        .collect();
    vs.sort();
    let mut es: Vec<String> = body
        .edges()
        .map(|(e, _)| {
            let (c, (t0, t1)) = carrier(body, e);
            format!("e {c:?} {t0:.17e} {t1:.17e}")
        })
        .collect();
    es.sort();
    let props = topo::mass_properties(body, tol());
    println!(
        "[r2dump] {what} | V {} E {} F {} S {} | vol {:?}",
        body.vertices().count(),
        body.edges().count(),
        body.faces().count(),
        body.shells().count(),
        props.map(|p| format!("{:.17e}", p.volume))
    );
    for row in vs.into_iter().chain(es) {
        println!("[r2dump] {what} | {row}");
    }
}

#[test]
fn p6_an_independent_corpus_differential() {
    // sf2b's frustums and drums, shelled.
    for (r0, r1, h) in [(1.0, 0.5, 2.0), (1.0, 0.25, 1.0), (0.75, 0.75, 1.5)] {
        let body = revolved(
            ProfileLoop::new(vec![
                ProfileVertex::new(p2(0.0, 0.0), 0.0),
                ProfileVertex::new(p2(r0, 0.0), 0.0),
                ProfileVertex::new(p2(r1, h), 0.0),
                ProfileVertex::new(p2(0.0, h), 0.0),
            ]),
            Revolution::Full,
        );
        dump(&format!("frustum {r0}/{r1}/{h}, operand"), &body);
        match topo::shell(&body, 0.05, tol()) {
            Ok(out) => dump(&format!("frustum {r0}/{r1}/{h}, shelled"), &out.body),
            Err(e) => println!("[r2dump] frustum {r0}/{r1}/{h}, shelled | refuses {e}"),
        }
    }
    // Partial wedges at several turns.
    for turn in [PI / 2.0, PI, 1.75 * PI] {
        let body = wedge(1.0, 2.0, turn);
        dump(&format!("wedge {turn:.3}, operand"), &body);
        match topo::shell(&body, 0.05, tol()) {
            Ok(out) => dump(&format!("wedge {turn:.3}, shelled"), &out.body),
            Err(e) => println!("[r2dump] wedge {turn:.3}, shelled | refuses {e}"),
        }
    }
    // Sphere-zone vases: the wall is an arc whose circle is centred ON
    // the axis, so the wall surface is a sphere.
    for bulge in [0.5f64, -0.5] {
        let Some(vase) = try_revolved(
            RawLoop::new(vec![
                ProfileVertex::new(p2(0.0, -0.8), 0.0),
                ProfileVertex::new(p2(0.6, -0.8), bulge),
                ProfileVertex::new(p2(0.6, 0.8), 0.0),
                ProfileVertex::new(p2(0.0, 0.8), 0.0),
            ]),
            Revolution::Full,
        ) else {
            println!("[r2dump] vase {bulge} | the revolve door refuses");
            continue;
        };
        dump(&format!("vase {bulge}, operand"), &vase);
        match topo::shell(&vase, 0.05, tol()) {
            Ok(out) => dump(&format!("vase {bulge}, shelled"), &out.body),
            Err(e) => println!("[r2dump] vase {bulge}, shelled | refuses {e}"),
        }
    }
    // The klein elbow: a quarter tube.
    let elbow = tube_along_arc::<f64>(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::unit_y(),
        Vec3::unit_x(),
        R,
        TubeWindow::Arc {
            t0: 0.0,
            t1: PI / 2.0,
        },
        SMALL_R,
        tol(),
    )
    .expect("the elbow builds")
    .body;
    dump("elbow, operand", &elbow);
    match topo::shell(&elbow, 0.05, tol()) {
        Ok(out) => dump("elbow, shelled", &out.body),
        Err(e) => println!("[r2dump] elbow, shelled | refuses {e}"),
    }
    // The hollow torus of SHELL-5, and the solid one.
    let hollow = tube_along_arc_hollow::<f64>(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::unit_y(),
        Vec3::unit_x(),
        R,
        TubeWindow::Full,
        SMALL_R,
        0.125,
        tol(),
    )
    .expect("hollow torus")
    .body;
    dump("hollow torus, operand", &hollow);
    match topo::shell(&hollow, 0.05, tol()) {
        Ok(out) => dump("hollow torus, shelled", &out.body),
        Err(e) => println!("[r2dump] hollow torus, shelled | refuses {e}"),
    }
}
