//! **The rows an Euler operator mints at the site are the minting
//! pass's rows**, on every analytic chart a revolve builds — cone,
//! cylinder, sphere and torus, over a quarter turn and over a full turn
//! whose loops wrap the chart's seam.
//!
//! `mev`, `mef` and `mekr` re-mint a complete face they add half-edges
//! to before they mutate (`topo::pcurves`' `site_rows`), walking each
//! loop they rewire from its `first` exactly as `mint_pcurves` does,
//! and keeping the rows of the loops they do not touch. Each row here
//! runs one operator on a minted face, then runs the pass over the
//! result and asserts it rewrites the same bits: image, interval and
//! certificate. The cylinder-sheet rows are `topo`'s
//! `euler_site_pcurve_rows`.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::{Curve3, Surface};
use geom_brep::EdgeCurveSpec;
use geom_core::{Band, Point2, Point3, Tol, Vec3};
use sweep::Revolution;
use sweep::test_support::{dome_profile, revolved_about_y};
use topo::pcurves::validate_pcurves;
use topo::{Body, EdgeKey, FaceKey, HalfEdgeKey, MefSite, MekrSite, MevSite, VertexKey};

fn tol() -> Tol {
    Tol::witness()
}

fn band() -> Band {
    Band::linear(tol()).unwrap()
}

/// Every stored row of the body with its interval, image and
/// certificate, sorted.
fn rows_deep(body: &Body<f64>) -> Vec<String> {
    let mut out: Vec<String> = body
        .pcurves()
        .map(|(he, c)| {
            format!(
                "{he:?} {:?} {:?} {:?}",
                c.params(),
                c.pcurve(),
                c.certificate()
            )
        })
        .collect();
    out.sort();
    out
}

/// Every half-edge of every loop of `face`.
fn halves_of(body: &Body<f64>, face: FaceKey) -> Vec<HalfEdgeKey> {
    let f = body.get_face(face).unwrap();
    core::iter::once(f.outer)
        .chain(f.rings.iter().copied())
        .filter_map(|lk| match body.get_loop(lk).unwrap().boundary {
            topo::LoopBoundary::Cycle { first } => Some(body.loop_cycle(first).unwrap()),
            topo::LoopBoundary::Empty { .. } => None,
        })
        .flatten()
        .collect()
}

/// `(rows stored, half-edges with no row)` over every loop of `face`.
fn rows_of(body: &Body<f64>, face: FaceKey) -> (usize, usize) {
    let halves = halves_of(body, face);
    let stored = halves
        .iter()
        .filter(|&&he| body.pcurve(he).is_some())
        .count();
    (stored, halves.len() - stored)
}

/// The one half-edge of `face` leaving `v`.
fn leaving(body: &Body<f64>, face: FaceKey, v: VertexKey) -> HalfEdgeKey {
    let hit: Vec<_> = halves_of(body, face)
        .into_iter()
        .filter(|&he| body.get_half_edge(he).unwrap().start == v)
        .collect();
    assert_eq!(hit.len(), 1, "one half-edge of the face leaves {v:?}");
    hit[0]
}

fn point_of(body: &Body<f64>, v: VertexKey) -> Point3<f64> {
    *body.get_point(body.get_vertex(v).unwrap().point).unwrap()
}

fn curve_of(body: &Body<f64>, e: EdgeKey) -> geom_brep::EdgeCurve<f64> {
    body.get_curve_geom(body.get_edge(e).unwrap().curve)
        .and_then(topo::CurveGeom::certified)
        .unwrap()
        .clone()
}

fn surface_of(body: &Body<f64>, face: FaceKey) -> Surface<f64> {
    body.get_surface(body.get_face(face).unwrap().surface)
        .unwrap()
        .clone()
}

/// The rims of `face`: its circle edges about the revolve axis, world
/// `Y`, lower first.
fn rims(body: &Body<f64>, face: FaceKey) -> Vec<EdgeKey> {
    let mut out: Vec<EdgeKey> = Vec::new();
    for he in halves_of(body, face) {
        let e = body.get_half_edge(he).unwrap().edge;
        if out.contains(&e) {
            continue;
        }
        if let Curve3::Circle { axis, .. } = curve_of(body, e).carrier()
            && axis.y.abs() > 0.999
        {
            out.push(e);
        }
    }
    let height = |e: &EdgeKey| match curve_of(body, *e).carrier() {
        Curve3::Circle { center, .. } => center.y,
        _ => unreachable!("a rim is a circle"),
    };
    out.sort_by(|a, b| height(a).total_cmp(&height(b)));
    out
}

/// Splits `e` at the parameter in its interval where its carrier's
/// angle is `angle`, or at its middle.
fn split_at(body: &mut Body<f64>, e: EdgeKey, angle: Option<f64>) -> VertexKey {
    let (t0, t1) = curve_of(body, e).params();
    let t = match angle {
        None => 0.5 * (t0 + t1),
        Some(a) => {
            let (lo, hi) = (t0.min(t1), t0.max(t1));
            let t = a + core::f64::consts::TAU * ((lo - a) / core::f64::consts::TAU).ceil();
            assert!(t > lo && t < hi, "{a} is not interior to {t0}..{t1}");
            t
        }
    };
    body.split_edge(e, t, tol()).unwrap().vertex
}

/// The angle of `p` on a circle carrier.
fn angle_on(c: &Curve3<f64>, p: Point3<f64>) -> f64 {
    let Curve3::Circle {
        center,
        axis,
        u_ref,
        ..
    } = c
    else {
        panic!("not a circle")
    };
    let d = p - *center;
    d.dot(axis.cross(*u_ref)).atan2(d.dot(*u_ref))
}

/// Asserts the body is tier-3 clean and that the minting pass over it
/// rewrites every row with the same bits.
fn assert_the_passs_rows(label: &str, body: &Body<f64>) {
    assert_eq!(validate_pcurves(body, band()), vec![], "{label}");
    let rows = rows_deep(body);
    let mut pass = body.clone();
    topo::mint_pcurves(&mut pass, tol()).unwrap();
    assert_eq!(rows_deep(&pass), rows, "{label}: the pass rewrote a row");
}

/// A revolve profile: points with their bulges.
type Profile = Vec<(Point2<f64>, f64)>;

/// Whether a surface is the chart kind a row is about.
type OnChart = fn(&Surface<f64>) -> bool;

fn v(x: f64, y: f64) -> (Point2<f64>, f64) {
    (Point2::new(x, y), 0.0)
}

/// A full revolve of the unit-wide square annulus: its outer wall is an
/// `r = 2` cylinder whose two rims each close a full turn of the chart,
/// joined by one seam line.
fn tube() -> (Body<f64>, FaceKey) {
    let body = revolved_about_y(
        vec![v(1.0, 0.0), v(2.0, 0.0), v(2.0, 1.0), v(1.0, 1.0)],
        Revolution::Full,
        tol(),
    );
    let wall = body
        .faces()
        .map(|(k, _)| k)
        .find(|&f| {
            matches!(surface_of(&body, f), Surface::Cylinder { radius, .. } if (radius - 2.0).abs() < 1e-9)
        })
        .unwrap();
    (body, wall)
}

/// The tube wall with its seam killed by `kemr`: the lower rim is the
/// outer loop and the upper rim a ring, each a one-edge loop that wraps
/// the chart.
fn ringed_tube() -> (Body<f64>, FaceKey, topo::LoopKey) {
    let (mut body, wall) = tube();
    let seam: Vec<EdgeKey> = halves_of(&body, wall)
        .into_iter()
        .map(|he| body.get_half_edge(he).unwrap().edge)
        .filter(|&e| matches!(curve_of(&body, e).carrier(), Curve3::Line { .. }))
        .collect();
    assert_eq!(seam.len(), 2, "the seam's two halves bound the wall");
    let e = body.get_edge(seam[0]).unwrap();
    let (a, b) = (e.he_plus, e.he_minus);
    let ring = body.kemr(a, b).unwrap().ring;
    (body, wall, ring)
}

/// **`mekr` joining the tube's two seam-wrapped rims.** With the seam
/// killed, the wall's outer loop and its ring are each one rim, and
/// each closes by a whole period of the chart. Both rims are split on
/// one ruling and the face re-minted; `mekr` joins them along that
/// ruling. The merged loop walks up the ruling, round the ring's rim,
/// down and round the outer rim, so it pins the ring's half-edges one
/// period from where the ring's own walk from its `first` put them —
/// and the rows it writes are the pass's over the merged loop. Two
/// rulings, on either side of the rims' shared start.
#[test]
fn a_mekr_joining_the_tubes_seam_wrapped_rims_mints_the_passs_rows() {
    for angle in [2.0, -2.5] {
        let (mut body, wall, ring) = ringed_tube();
        let rims = rims(&body, wall);
        let m1 = split_at(&mut body, rims[0], Some(angle));
        let p1 = point_of(&body, m1);
        let upper = curve_of(&body, rims[1]).carrier().clone();
        let m2 = split_at(
            &mut body,
            rims[1],
            Some(angle_on(&upper, p1 + Vec3::unit_y())),
        );
        let p2 = point_of(&body, m2);
        topo::mint_pcurves(&mut body, tol()).unwrap();
        assert_eq!(rows_of(&body, wall), (4, 0), "{angle}");
        let (h1, h2) = (leaving(&body, wall, m1), leaving(&body, wall, m2));
        let in_ring = |he: HalfEdgeKey| body.get_half_edge(he).unwrap().parent_loop == ring;
        let ((target, pt), (ring_he, pr)) = if in_ring(h2) {
            ((h1, p1), (h2, p2))
        } else {
            ((h2, p2), (h1, p1))
        };
        body.mekr(
            MekrSite::Cycles {
                target,
                ring: ring_he,
            },
            EdgeCurveSpec::line_between(pt, pr),
            tol(),
        )
        .unwrap();
        assert_eq!(rows_of(&body, wall), (6, 0), "{angle}");
        assert_the_passs_rows(&format!("ruling at {angle}"), &body);
    }
}

/// **The ring a strut does not touch keeps its rows, on a wrapping
/// chart.** On the ringed tube wall a strut up a ruling from the outer
/// rim re-mints the outer loop and leaves the ring's row — a rim that
/// closes by a whole period — exactly as it was; the face's rows are
/// the pass's.
#[test]
fn a_strut_on_the_ringed_tube_wall_keeps_the_rings_row() {
    let (mut body, wall, ring) = ringed_tube();
    let lower = rims(&body, wall)[0];
    let m = split_at(&mut body, lower, Some(2.0));
    topo::mint_pcurves(&mut body, tol()).unwrap();
    let topo::LoopBoundary::Cycle { first } = body.get_loop(ring).unwrap().boundary else {
        panic!("the ring is the upper rim")
    };
    let ring_row = |body: &Body<f64>| {
        let row = body.pcurve(first).unwrap();
        format!(
            "{:?} {:?} {:?}",
            row.params(),
            row.pcurve(),
            row.certificate()
        )
    };
    let before = ring_row(&body);
    let he = leaving(&body, wall, m);
    body.mev_line(
        MevSite::Fan { he1: he, he2: he },
        point_of(&body, m) + Vec3::unit_y() * 0.4,
        tol(),
    )
    .unwrap();
    assert_eq!(ring_row(&body), before);
    assert_eq!(rows_of(&body, wall), (5, 0));
    assert_the_passs_rows("ringed tube strut", &body);
}

/// The meridian of `s` from `p0` toward `p1`, `frac` of the way: a line
/// on a cylinder or cone, a great-circle arc on a sphere, a tube-circle
/// arc on a torus. Returns the spec and its far end.
fn meridian(
    s: &Surface<f64>,
    p0: Point3<f64>,
    p1: Point3<f64>,
    frac: f64,
) -> (EdgeCurveSpec<f64>, Point3<f64>) {
    let arc_about = |c: Point3<f64>| {
        let (a, b) = (p0 - c, p1 - c);
        let r = a.norm();
        // Two points on one line through the centre (a sphere zone's
        // rims straight above one another) span no plane: the meridian
        // plane is then the one through the revolve axis.
        let n = a.cross(b);
        let axis = if n.norm() < 1e-9 {
            a.cross(Vec3::unit_y()).normalize()
        } else {
            n.normalize()
        };
        let theta = (a.dot(b) / (r * b.norm())).clamp(-1.0, 1.0).acos() * frac;
        let circle = Curve3::Circle {
            center: c,
            axis,
            radius: r,
            u_ref: a / r,
        };
        let end = circle.eval(theta);
        (
            EdgeCurveSpec::arc_of_circle(circle, 0.0, theta).unwrap(),
            end,
        )
    };
    match s {
        Surface::Cylinder { .. } | Surface::Cone { .. } => {
            let end = p0 + (p1 - p0) * frac;
            (EdgeCurveSpec::line_between(p0, end), end)
        }
        Surface::Sphere { center, .. } => arc_about(*center),
        Surface::Torus {
            center,
            axis,
            major_radius,
            ..
        } => {
            let d = p0 - *center;
            let h = (d - *axis * d.dot(*axis)).normalize();
            arc_about(*center + h * *major_radius)
        }
        other => panic!("no meridian on {other:?}"),
    }
}

/// On a minted face with two rims, split at their middles: a strut
/// `mev` part-way up the meridian, and — on a fresh copy — a `mef`
/// closing the meridian. Each result's rows are the pass's. Returns how
/// many ops ran, so a fixture that stopped reaching this is loud.
fn strut_and_chord(label: &str, body0: &Body<f64>, face: FaceKey) -> usize {
    let s = surface_of(body0, face);
    let rim = rims(body0, face);
    assert_eq!(rim.len(), 2, "{label}: two rims");
    let site = |body: &mut Body<f64>| {
        let m1 = split_at(body, rim[0], None);
        let m2 = split_at(body, rim[1], None);
        (m1, m2)
    };
    let mut body = body0.clone();
    let (m1, m2) = site(&mut body);
    let (spec, end) = meridian(&s, point_of(&body, m1), point_of(&body, m2), 0.4);
    let he = leaving(&body, face, m1);
    let made = body
        .mev(MevSite::Fan { he1: he, he2: he }, end, spec, tol())
        .unwrap_or_else(|e| panic!("{label} mev: {e:?}"));
    assert!(body.pcurve(made.he_plus).is_some(), "{label} mev");
    assert_eq!(rows_of(&body, face), (8, 0), "{label} mev");
    assert_the_passs_rows(&format!("{label} mev"), &body);

    let mut body = body0.clone();
    let (m1, m2) = site(&mut body);
    let (spec, _) = meridian(&s, point_of(&body, m1), point_of(&body, m2), 1.0);
    let (he1, he2) = (leaving(&body, face, m1), leaving(&body, face, m2));
    let made = body
        .mef(
            MefSite::Chords { he1, he2 },
            spec,
            topo::FaceSurface::Inherit,
            tol(),
        )
        .unwrap_or_else(|e| panic!("{label} mef: {e:?}"));
    assert_eq!(rows_of(&body, face), (4, 0), "{label} mef");
    assert_eq!(rows_of(&body, made.face), (4, 0), "{label} mef");
    assert_the_passs_rows(&format!("{label} mef"), &body);
    2
}

/// **The site rows are the pass's on cone, cylinder, sphere and torus**,
/// each revolved a quarter turn and a full turn — the full turn's faces
/// are bounded by a seam, so their loops wrap the chart and the walk
/// parks a one-period offset where the pass parks it.
#[test]
fn the_site_rows_are_the_passs_on_every_revolved_chart() {
    let trapezoid = vec![v(1.0, 0.0), v(2.0, 0.0), v(1.5, 1.0), v(1.0, 1.0)];
    let circle = vec![(Point2::new(1.5, 0.0), 1.0), (Point2::new(2.5, 0.0), 1.0)];
    let kinds: [(&str, Profile, OnChart); 4] = [
        ("cone", trapezoid.clone(), |s| {
            matches!(s, Surface::Cone { .. })
        }),
        ("cylinder", trapezoid, |s| {
            matches!(s, Surface::Cylinder { .. })
        }),
        ("sphere", dome_profile(1.0), |s| {
            matches!(s, Surface::Sphere { .. })
        }),
        ("torus", circle, |s| matches!(s, Surface::Torus { .. })),
    ];
    let mut ran = 0;
    for (kind, profile, pick) in kinds {
        for (turn, revolution) in [
            ("quarter", Revolution::Partial(core::f64::consts::FRAC_PI_2)),
            ("full", Revolution::Full),
        ] {
            let body = revolved_about_y(profile.clone(), revolution, tol());
            let faces: Vec<FaceKey> = body
                .faces()
                .map(|(k, _)| k)
                .filter(|&f| pick(&surface_of(&body, f)))
                .collect();
            assert!(!faces.is_empty(), "{kind} {turn}: a face on the chart");
            for face in faces {
                let (stored, missing) = rows_of(&body, face);
                assert!(stored > 0 && missing == 0, "{kind} {turn}: minted");
                ran += strut_and_chord(&format!("{kind} {turn} {face:?}"), &body, face);
            }
        }
    }
    assert!(ran >= 16, "{ran} ops ran");
}
