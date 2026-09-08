//! SHELL-7 review lane R1: execution probes against PR #2200's claims.
//! Every row here is a measurement first; the assertions pin what was
//! measured on the frozen head and say so in their messages.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::{FRAC_PI_2, PI};

use geom::{Curve3, Surface};
use geom_core::{Point2, Point3, Tol, Vec2, Vec3};
use profile::{Profile, ProfileLoop, ProfileVertex, RawLoop, SketchPlane};
use sweep::{Revolution, RevolveAxis, TubeWindow, revolve, tube_along_arc, tube_along_arc_hollow};
use topo::{Body, EdgeKey, FaceKey, ReplaceFaceError, ShellError, ShellRole, VertexKey};

fn p2(x: f64, y: f64) -> Point2<f64> {
    Point2::new(x, y)
}
fn tol() -> Tol {
    Tol::witness()
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

fn tube_torus(big_r: f64, r: f64) -> Result<Body<f64>, String> {
    tube_along_arc::<f64>(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::unit_y(),
        Vec3::unit_x(),
        big_r,
        TubeWindow::Full,
        r,
        tol(),
    )
    .map(|b| b.body)
    .map_err(|e| format!("{e:?}"))
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

fn face_of_he(body: &Body<f64>, he: topo::HalfEdgeKey) -> FaceKey {
    let lp = body.get_half_edge(he).unwrap().parent_loop;
    body.get_loop(lp).unwrap().face
}

fn same_surface(body: &Body<f64>, e: EdgeKey) -> bool {
    let d = body.get_edge(e).unwrap();
    let k = |he| body.get_face(face_of_he(body, he)).unwrap().surface;
    k(d.he_plus) == k(d.he_minus)
}

fn distinct_surfaces_at(body: &Body<f64>, v: VertexKey) -> usize {
    let em = body.get_vertex(v).unwrap().emanating.unwrap();
    let mut s: Vec<_> = body
        .vertex_orbit(em)
        .unwrap()
        .into_iter()
        .map(|he| body.get_face(face_of_he(body, he)).unwrap().surface)
        .collect();
    s.sort();
    s.dedup();
    s.len()
}

fn split_mid(body: &mut Body<f64>, edge: EdgeKey) -> VertexKey {
    let (_, (t0, t1)) = carrier(body, edge);
    body.split_edge(edge, (t0 + t1) * 0.5, tol())
        .expect("the edge splits")
        .vertex
}

fn corner_refusal(e: &ShellError<f64>) -> Option<(VertexKey, usize, &'static str)> {
    let ShellError::Face { error, .. } = e else {
        return None;
    };
    match **error {
        ReplaceFaceError::TogetherAxialCorner {
            vertex,
            surfaces,
            what,
        } => Some((vertex, surfaces, what)),
        _ => None,
    }
}

fn edge_refusal(e: &ShellError<f64>) -> Option<(EdgeKey, &'static str)> {
    let ShellError::Face { error, .. } = e else {
        return None;
    };
    match **error {
        ReplaceFaceError::TogetherAxialEdge { edge, what } => Some((edge, what)),
        _ => None,
    }
}

/// Claim 2: a LINE profile beside ONE meridian cap — the wedge's
/// wall/cap generator edge split by hand at mid-height — refuses with
/// the new `what`, while the CIRCLE twin (klein elbow's rim corner) is
/// carried. The vertex's position along the line is exactly as
/// conventional as the circle corner's angle about its centre.
#[test]
fn r1_line_beside_one_meridian_refuses_where_its_circle_twin_is_carried() {
    let (r, h) = (1.0, 2.0);
    let mut body = revolved(
        ProfileLoop::new(vec![
            ProfileVertex::new(p2(0.0, 0.0), 0.0),
            ProfileVertex::new(p2(r, 0.0), 0.0),
            ProfileVertex::new(p2(r, h), 0.0),
            ProfileVertex::new(p2(0.0, h), 0.0),
        ]),
        Revolution::Partial(FRAC_PI_2),
    );
    // The wall/cap generator at azimuth 0: a line at radius r along y,
    // between two DISTINCT surfaces (cylinder wall, meridian cap).
    let hits: Vec<EdgeKey> = body
        .edges()
        .filter(|(e, _)| match carrier(&body, *e).0 {
            Curve3::Line { origin, dir } => {
                origin.x > 0.5
                    && origin.z.abs() < 1e-12
                    && dir.dot(Vec3::unit_y()).abs() >= 1.0 - 1e-15
                    && !same_surface(&body, *e)
            }
            _ => false,
        })
        .map(|(e, _)| e)
        .collect();
    assert_eq!(
        hits.len(),
        1,
        "one wall/cap generator at azimuth 0: {hits:?}"
    );
    let split = split_mid(&mut body, hits[0]);
    topo::mint_pcurves(&mut body, tol()).expect("pcurves");
    assert_eq!(distinct_surfaces_at(&body, split), 2, "wall + cap");
    let e = topo::shell(&body, 0.05, tol()).expect_err("measured: refuses");
    let (v, n, what) = corner_refusal(&e).unwrap_or_else(|| panic!("not a corner refusal: {e}"));
    println!("[r1] line-beside-meridian: {v:?} surfaces={n} what={what}");
    assert_eq!(v, split);
    assert_eq!(n, 2);
    assert!(what.starts_with("one profile constraint and a plane containing the axis"));
}

/// Claim 6 / row 7: the tube torus at `t` across `r`. What refuses when
/// the cavity's minor radius would reach zero or go negative?
#[test]
fn r1_torus_thickness_ladder_names_what_refuses() {
    let (big_r, r) = (2.0, 0.5);
    let body = tube_torus(big_r, r).unwrap();
    for t in [0.45, 0.5 - 1e-6, 0.5, 0.55, 1.4, 1.5] {
        match topo::shell(&body, t, tol()) {
            Ok(s) => {
                let props = topo::mass_properties(&s.body, tol()).expect("props");
                let want = 2.0 * PI * PI * big_r * (r * r - (r - t) * (r - t));
                println!(
                    "[r1] t={t}: OK vol={} want={want} tier3={:?}",
                    props.volume,
                    topo::validate_geometric(&s.body, tol())
                );
                assert!((props.volume - want).abs() <= 1e-9 + props.volume_pad);
            }
            Err(e) => println!("[r1] t={t}: Err {e}"),
        }
    }
}

/// Claim 3: a torus whose tube-centre circle is BELOW the band — the
/// meridian seam's centre decides `Zero` against the axis and is
/// routed to the latitude rule; what happens?
#[test]
fn r1_tiny_major_radius_torus() {
    for (big_r, r) in [(4e-9, 2e-9), (4e-8, 2e-8), (4e-7, 2e-7), (4e-6, 2e-6)] {
        match tube_torus(big_r, r) {
            Err(e) => println!("[r1] R={big_r} r={r}: tube door refuses {e}"),
            Ok(body) => match topo::shell(&body, r / 10.0, tol()) {
                Ok(_) => println!("[r1] R={big_r} r={r}: shells"),
                Err(e) => println!("[r1] R={big_r} r={r}: shell Err {e}"),
            },
        }
    }
}

/// Claim 6/7 reachability: the DOOR builds a one-surface cylinder
/// vertex — a rectangle with a collinear vertex on its wall is a legal
/// profile (same-carrier continuation), and its full revolve has a
/// vertex at `(r, h/2)` on the seam whose faces are all one cylinder.
#[test]
fn r1_a_collinear_wall_vertex_is_a_door_built_one_surface_vertex() {
    let (r, h) = (1.0, 2.0);
    let body = revolved(
        ProfileLoop::new(vec![
            ProfileVertex::new(p2(0.0, 0.0), 0.0),
            ProfileVertex::new(p2(r, 0.0), 0.0),
            ProfileVertex::new(p2(r, h / 2.0), 0.0),
            ProfileVertex::new(p2(r, h), 0.0),
            ProfileVertex::new(p2(0.0, h), 0.0),
        ]),
        Revolution::Full,
    );
    assert_eq!(
        topo::validate_geometric(&body, tol()),
        Ok(()),
        "operand tier 3"
    );
    let mids: Vec<VertexKey> = body
        .vertices()
        .filter(|(v, _)| (point(&body, *v).y - h / 2.0).abs() < 1e-12)
        .map(|(v, _)| v)
        .collect();
    for &v in &mids {
        println!(
            "[r1] collinear drum: mid vertex {v:?} at {:?} distinct_surfaces={}",
            point(&body, v),
            distinct_surfaces_at(&body, v)
        );
    }
    assert!(!mids.is_empty());
    let one_surface = mids.iter().all(|&v| distinct_surfaces_at(&body, v) == 1);
    match topo::shell(&body, 0.05, tol()) {
        Ok(s) => {
            let props = topo::mass_properties(&s.body, tol()).expect("props");
            println!("[r1] collinear drum: shells, vol={}", props.volume);
        }
        Err(e) => println!(
            "[r1] collinear drum: shell Err {e} / corner={:?} edge={:?}",
            corner_refusal(&e),
            edge_refusal(&e)
        ),
    }
    assert!(
        one_surface,
        "the door built a vertex whose only surface is a cylinder"
    );
}

/// The PR's sweep says the sphere's seam is always a great circle. A
/// semicircle authored as TWO cocircular arcs (legal undeclared) revolves
/// to two faces on one sphere with a LATITUDE seam between them.
#[test]
fn r1_a_cocircular_profile_vertex_makes_a_sphere_latitude_seam() {
    let r = 1.0;
    let v = PI / 4.0;
    let (s, c) = v.sin_cos();
    // (0,-r) --arc (π/2+v)--> (r c, r s) --arc (π/2−v)--> (0, r) --line--> back
    let body = revolved(
        ProfileLoop::new(vec![
            ProfileVertex::new(p2(0.0, -r), ((FRAC_PI_2 + v) / 4.0).tan()),
            ProfileVertex::new(p2(r * c, r * s), ((FRAC_PI_2 - v) / 4.0).tan()),
            ProfileVertex::new(p2(0.0, r), 0.0),
        ]),
        Revolution::Full,
    );
    println!(
        "[r1] two-arc sphere: tier3={:?} faces={} edges={} verts={}",
        topo::validate_geometric(&body, tol()),
        body.faces().count(),
        body.edges().count(),
        body.vertices().count()
    );
    let mut spheres: Vec<_> = body.faces().map(|(_, f)| f.surface).collect();
    spheres.sort();
    spheres.dedup();
    println!(
        "[r1] two-arc sphere: distinct surface keys={}",
        spheres.len()
    );
    for (e, _) in body.edges() {
        let (cv, _) = carrier(&body, e);
        println!(
            "[r1] two-arc sphere: edge {e:?} same_surface={} carrier={cv:?}",
            same_surface(&body, e)
        );
    }
    match topo::shell(&body, 0.05, tol()) {
        Ok(s) => {
            let props = topo::mass_properties(&s.body, tol()).expect("props");
            let want = 4.0 / 3.0 * PI * (r.powi(3) - (r - 0.05f64).powi(3));
            println!(
                "[r1] two-arc sphere: shells vol={} want={want}",
                props.volume
            );
        }
        Err(e) => println!(
            "[r1] two-arc sphere: shell Err {e} / corner={:?} edge={:?}",
            corner_refusal(&e),
            edge_refusal(&e)
        ),
    }
}

/// The same two-arc profile on a torus but PARTIAL: the latitude seam's
/// start corner is a [torus, meridian cap] corner whose azimuth comes
/// from the MOVED cap — off the sketch plane — feeding `reauthor`'s
/// `RevolvedPoint` arm with `q.z ≠ 0` if the edge reaches it.
#[test]
fn r1_partial_two_arc_torus_off_plane_corner() {
    let (big_r, r) = (2.0, 0.5);
    let a = p2(big_r + r, 0.0);
    let b = p2(big_r - r, 0.0);
    let body = revolved(
        RawLoop::new(vec![ProfileVertex::new(a, 1.0), ProfileVertex::new(b, 1.0)]),
        Revolution::Partial(FRAC_PI_2),
    );
    println!(
        "[r1] partial two-arc torus: tier3={:?}",
        topo::validate_geometric(&body, tol())
    );
    match topo::shell(&body, 0.05, tol()) {
        Ok(_) => println!("[r1] partial two-arc torus: shells"),
        Err(e) => println!(
            "[r1] partial two-arc torus: shell Err {e} / corner={:?} edge={:?}",
            corner_refusal(&e),
            edge_refusal(&e)
        ),
    }
}

/// Claim 7: `Body::split_edge` leaves its children without pcurve rows.
#[test]
fn r1_split_edge_children_lack_pcurves() {
    let (r, h) = (1.0, 2.0);
    let mut body = revolved(
        ProfileLoop::new(vec![
            ProfileVertex::new(p2(0.0, 0.0), 0.0),
            ProfileVertex::new(p2(r, 0.0), 0.0),
            ProfileVertex::new(p2(r, h), 0.0),
            ProfileVertex::new(p2(0.0, h), 0.0),
        ]),
        Revolution::Full,
    );
    let seam = body
        .edges()
        .find(|(e, _)| match carrier(&body, *e).0 {
            Curve3::Line { origin, dir } => {
                origin.x > 0.0
                    && dir.dot(Vec3::unit_y()).abs() >= 1.0 - 1e-15
                    && same_surface(&body, *e)
            }
            _ => false,
        })
        .map(|(e, _)| e)
        .expect("a seam");
    assert_eq!(topo::validate_geometric(&body, tol()), Ok(()));
    split_mid(&mut body, seam);
    let verdict = topo::validate_geometric(&body, tol());
    println!("[r1] split drum before mint: {verdict:?}");
    assert!(
        verdict.is_err(),
        "measured: the split operand is tier-3 invalid"
    );
    let e = topo::shell(&body, 0.05, tol()).expect_err("shell refuses the unminted operand");
    println!("[r1] split drum shell: {e}");
    topo::mint_pcurves(&mut body, tol()).expect("mint");
    assert_eq!(topo::validate_geometric(&body, tol()), Ok(()));
}

/// The consumer's seat: solid + hollow tori through the tube doors,
/// shell each, shell the hollow one AGAIN, classify, volumes, tessellate,
/// and `shell_open` on a torus face.
#[test]
fn r1_end_to_end_consumer_seat() {
    let (big_r, r, w, t) = (2.0, 0.5, 0.125, 0.05);
    let (c, a, u) = (Point3::new(0.0, 0.0, 0.0), Vec3::unit_y(), Vec3::unit_x());
    let solid = tube_along_arc::<f64>(c, a, u, big_r, TubeWindow::Full, r, tol())
        .expect("solid")
        .body;
    let hollow = tube_along_arc_hollow::<f64>(c, a, u, big_r, TubeWindow::Full, r, w, tol())
        .expect("hollow")
        .body;
    let pi2r = 2.0 * PI * PI * big_r;
    let ring = |a: f64, b: f64| a * a - b * b;

    let s1 = topo::shell(&solid, t, tol()).expect("solid shells");
    let roles: Vec<_> = topo::classify_shells(&s1.body, tol())
        .expect("classify")
        .into_iter()
        .map(|c| (c.solid, c.role))
        .collect();
    println!(
        "[e2e] solid shelled: solids={} shells={} roles={roles:?}",
        s1.body.solids().count(),
        s1.body.shells().count()
    );
    let p1 = topo::mass_properties(&s1.body, tol()).expect("props");
    assert!((p1.volume - pi2r * ring(r, r - t)).abs() <= 1e-9 + p1.volume_pad);

    let s2 = topo::shell(&hollow, t, tol()).expect("hollow shells");
    let p2v = topo::mass_properties(&s2.body, tol()).expect("props");
    let want2 = pi2r * (ring(r, r - t) + ring(r - w + t, r - w));
    println!(
        "[e2e] hollow shelled: solids={} vol={} want={want2}",
        s2.body.solids().count(),
        p2v.volume
    );
    assert!((p2v.volume - want2).abs() <= 1e-9 + p2v.volume_pad);

    // SHELL-5's semantics again: shell the twice-hollow result (2 solids).
    let t2 = 0.01;
    match topo::shell(&s2.body, t2, tol()) {
        Ok(s3) => {
            let p3 = topo::mass_properties(&s3.body, tol()).expect("props");
            let want3 = pi2r
                * (ring(r, r - t2)
                    + ring(r - t + t2, r - t)
                    + ring(r - w + t, r - w + t - t2)
                    + ring(r - w + t2, r - w));
            println!(
                "[e2e] hollow shelled again: solids={} shells={} vol={} want={want3}",
                s3.body.solids().count(),
                s3.body.shells().count(),
                p3.volume
            );
        }
        Err(e) => println!("[e2e] hollow shelled again: Err {e}"),
    }
    // Tessellate the two thin tori.
    match mesh::tessellate(&s2.body, 0.01, tol()) {
        Ok(m) => println!(
            "[e2e] tessellate: positions={} patches={} check={:?}",
            m.positions.len(),
            m.patches.len(),
            mesh::validate::check_mesh(&m)
        ),
        Err(e) => println!("[e2e] tessellate: Err {e:?}"),
    }
    // shell_open on a torus: designate one of the two torus faces.
    let face = solid.faces().next().map(|(k, _)| k).expect("a face");
    match topo::shell_open(&solid, t, &[face], tol()) {
        Ok(o) => println!(
            "[e2e] shell_open on a torus face: Ok solids={} shells={} rims={}",
            o.body.solids().count(),
            o.body.shells().count(),
            o.naming.rims.len()
        ),
        Err(e) => println!("[e2e] shell_open on a torus face: Err {e}"),
    }
    let _ = ShellRole::Outer;
    let _: Option<&Surface<f64>> = None;
}
