//! **A link's reach skips the faces its own window ends in, not its
//! chain's.** A face at a link's own end is a plane end face lying on the
//! cap that ends the window, so it never meets the reach's inside; a
//! face at another link's end of the same chain lies on no bound of this
//! reach and is metered like any other.
//!
//! Every row reads the meter over the chains production builds: the
//! battery breaks a chain at each turn, so a multi-link chain spans
//! valence-2 joints only. The witness is such a chain — a station on the
//! rim of a prism whose end face leans back over the edge: the end face
//! ends the far link and cuts the near one's band. Read at the meter,
//! since the door's joint-foot screen refuses the body first; a station
//! far from the end face, and none, are the controls.
//!
//! The rest pin behaviour the per-link skip shares with the chain-wide
//! one it replaced, or that sits beside it: a thin trapezoid's back face
//! cutting the band of the front link between two turns (its own chain
//! once broken at the turns, so metered either way), at `f64` and at
//! `Interval`; an L-shaped end plate whose arm overhangs the band; a
//! curved end face, which the meter fails loud on and the battery
//! refuses first; the bands of chains that meet at a turn, metered
//! against each other; a far end face just clear of the band, and a
//! chain into a corner patch, both clear and built.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::Surface;
use geom_core::{Band, Bounds, Interval, Point2, Point3, Tol};
use sweep::blend::{BlendError, BlendRequest, fillet_edges};
use sweep::test_support::band_reach;
use topo::{Body, EdgeKey, EntityId, FaceKey, validate_geometric};

use crate::common::cavity::{brick, cut, edges_with_corners, prism};
use crate::common::interval::{iv, p2};
use crate::common::stations::cut_stations;

fn tol() -> Tol {
    Tol::witness()
}

fn band() -> Band {
    Band::linear(tol()).expect("the witness band")
}

/// The plane of `face` as its unit normal and its offset along it.
fn plane_of(body: &Body<f64>, face: FaceKey) -> ([f64; 3], f64) {
    let s = body
        .get_face(face)
        .and_then(|f| body.get_surface(f.surface))
        .expect("the face's surface");
    let Surface::Plane { origin, normal, .. } = s else {
        panic!("a plane face, got {s:?}");
    };
    let n = normal.normalize();
    ([n.x, n.y, n.z], (*origin - Point3::origin()).dot(n))
}

/// Whether `face` lies on the plane `y = y0`.
fn on_y(body: &Body<f64>, face: FaceKey, y0: f64) -> bool {
    let ([nx, ny, nz], d) = plane_of(body, face);
    nx.abs() < 1e-12 && nz.abs() < 1e-12 && (d * ny - y0).abs() < 1e-12
}

/// The prism on the trapezoid `(0,0) (4,0) (4,w) (−1,w)` from `z = 0`
/// to `2`, and its three top edges but the back one: `L0` over the
/// oblique left face, `L1` over the front, `L2` over the right, turning
/// into each other at the front corners, so three chains of one link
/// each. Every vertex of the top face is a vertex of a chain, so the
/// strip screen inside the meter passes it at any width.
fn trapezoid(w: f64) -> (Body<f64>, Vec<topo::EdgeKey>) {
    let body = prism(
        &[
            Point2::new(0.0, 0.0),
            Point2::new(4.0, 0.0),
            Point2::new(4.0, w),
            Point2::new(-1.0, w),
        ],
        0.0,
        2.0,
    );
    let top = |p: Point3<f64>| (p.z - 2.0).abs() < 1e-12;
    let back = edges_with_corners(&body, |p| top(p) && (p.y - w).abs() < 1e-12);
    let edges: Vec<_> = edges_with_corners(&body, top)
        .into_iter()
        .filter(|e| !back.contains(e))
        .collect();
    assert_eq!(edges.len(), 3, "w = {w}: the chain's three top edges");
    (body, edges)
}

/// At `r = 0.5` `L1`'s band removes material up to `y = r` under the top
/// face, so a back face at `y = 0.2` or `0.45` lies in it, at `0.6`
/// clear of it. The meter names the back face — the end face of `L0`
/// and `L2`, at no vertex of `L1` — uncertified; the door refuses the
/// thin top at its support screen first; the wide body builds. A pin,
/// not a witness: the turns break the three links into chains of their
/// own, so the back face was metered against `L1` under the chain-wide
/// skip too.
#[test]
fn a_cut_off_face_at_another_links_end_is_metered_against_the_link() {
    let r = 0.5;
    for w in [0.2, 0.45] {
        let (body, edges) = trapezoid(w);
        let req = BlendRequest {
            body: &body,
            edges: edges.clone(),
            size: r,
        };
        let e = band_reach(&req, band()).expect_err("the back face cuts L1's band");
        let BlendError::FaceClearance {
            at: EntityId::Face(f),
            bounded: true,
            ..
        } = e
        else {
            panic!("w = {w}: the meter refuses a face uncertified: {e:?}");
        };
        assert!(
            on_y(&body, f, w),
            "w = {w}: the face named is the back face"
        );
        let door = fillet_edges(
            &sweep::test_support::at_rest(&body, tol()),
            &edges,
            r,
            tol(),
        )
        .map(|_| ())
        .map_err(|e| e.error);
        assert!(
            matches!(door, Err(BlendError::FaceClearanceUncertified { .. })),
            "w = {w}: the door refuses the thin top at its support screen: {door:?}"
        );
    }
    let (body, edges) = trapezoid(0.6);
    let req = BlendRequest {
        body: &body,
        edges: edges.clone(),
        size: r,
    };
    if let Err(e) = band_reach(&req, band()) {
        panic!("w = 0.6: clear at the meter: {e:?}");
    }
    let f = fillet_edges(
        &sweep::test_support::at_rest(&body, tol()),
        &edges,
        r,
        tol(),
    )
    .unwrap_or_else(|e| panic!("w = 0.6: the wide body builds: {:?}", e.error));
    assert_eq!(
        validate_geometric(&f.body, tol()),
        Ok(()),
        "w = 0.6: tier 3"
    );
}

/// A pocket's floor–wall edge `x ∈ [0, 4]` between two end plates, and
/// an arm `x ∈ [2.5, 4.5]`, `y ∈ [y0, 1.5]`, `z ∈ [h, 1.5]` standing out
/// of the far plate over the band. At `r = 1` the band adds material up
/// to `y ≈ 0.286` at `z = 0.3`: the arm at `(0.1, 0.3)` lies in it, at
/// `(0.5, 0.6)` clear. The arm's faces are at no vertex of the chain, so
/// the meter refuses one of them as a measurement; the door refuses the
/// arm's ring on the end plate first, at the end face's own meter. Clear
/// of the band, and with no arm, the body builds. A pin of behaviour the
/// chain-wide skip shared: the arm's faces were never skipped.
#[test]
fn an_l_shaped_end_plates_arm_over_the_band_is_metered_and_refused() {
    let p = Point3::new;
    let r = 1.0;
    let pocket = cut(
        "pocket",
        &brick(p(-1.0, -1.0, -1.0), p(5.0, 3.0, 3.0)),
        &brick(p(0.0, 0.0, 0.0), p(4.0, 3.5, 3.5)),
    );
    let with_arm = |y0: f64, h: f64| {
        let t = tol();
        let a = sweep::test_support::finished("pocket", pocket.clone(), t);
        let b = sweep::test_support::finished("arm", brick(p(2.5, y0, h), p(4.5, 1.5, 1.5)), t);
        topo::union(&a, &b, t)
            .expect("the union succeeds")
            .body()
            .expect("the union leaves material")
            .body
            .clone()
            .into_body()
    };
    let edge = |body: &Body<f64>| {
        let e = edges_with_corners(body, |q| {
            q.y.abs() < 1e-12 && q.z.abs() < 1e-12 && (-0.5..4.5).contains(&q.x)
        });
        assert_eq!(e.len(), 1, "the floor–wall edge");
        e
    };

    let body = with_arm(0.1, 0.3);
    let edges = edge(&body);
    let req = BlendRequest {
        body: &body,
        edges: edges.clone(),
        size: r,
    };
    let e = band_reach(&req, band()).expect_err("the arm stands in the band");
    let BlendError::FaceClearance {
        at: EntityId::Face(f),
        bounded: false,
        ..
    } = e
    else {
        panic!("in: the meter measures an arm face in the band: {e:?}");
    };
    let ([nx, _, _], d) = plane_of(&body, f);
    assert!(
        !(nx.abs() > 1.0 - 1e-12 && (d * nx - 4.0).abs() < 1e-12),
        "in: the face named is the arm's, not the end plate's"
    );
    let door = fillet_edges(
        &sweep::test_support::at_rest(&body, tol()),
        &edges,
        r,
        tol(),
    )
    .map(|_| ())
    .map_err(|e| e.error);
    assert!(
        matches!(door, Err(BlendError::RingClearance { bounded: false, .. })),
        "in: the end face's ring meter refuses the arm's ring first: {door:?}"
    );

    for (what, body) in [("clear", with_arm(0.5, 0.6)), ("no arm", pocket.clone())] {
        let edges = edge(&body);
        let f = fillet_edges(
            &sweep::test_support::at_rest(&body, tol()),
            &edges,
            r,
            tol(),
        )
        .unwrap_or_else(|e| panic!("{what}: builds: {:?}", e.error));
        assert_eq!(validate_geometric(&f.body, tol()), Ok(()), "{what}: tier 3");
    }
}

/// The trapezoid's top edges at `T`, by their end points: every top edge
/// but the back one.
fn trapezoid_edges<T: Bounds>(body: &Body<T>, w: f64) -> Vec<EdgeKey> {
    let mid = |v| {
        let q = body
            .get_vertex(v)
            .and_then(|x| body.get_point(x.point))
            .expect("a vertex's point");
        [q.x, q.y, q.z].map(|c| 0.5 * (c.lo() + c.hi()))
    };
    let top = |q: [f64; 3]| (q[2] - 2.0).abs() < 1e-9;
    let back = |q: [f64; 3]| top(q) && (q[1] - w).abs() < 1e-9;
    let mut out: Vec<EdgeKey> = body
        .edges()
        .filter_map(|(k, e)| {
            let a = mid(body.get_half_edge(e.he_plus)?.start);
            let b = mid(body.half_edge_end(e.he_plus)?);
            (top(a) && top(b) && !(back(a) && back(b))).then_some(k)
        })
        .collect();
    out.sort_unstable();
    out
}

/// The trapezoid at `Interval` gives `f64`'s verdict, its face read by
/// the same per-link skip: the back face refused uncertified where it
/// cuts `L1`'s band, clear where it does not.
#[test]
fn the_trapezoids_back_face_reads_alike_at_interval() {
    let verdict = |e: Result<(), BlendError>| match e {
        Ok(()) => "clear".to_string(),
        Err(BlendError::FaceClearance {
            at: EntityId::Face(_),
            bounded,
            ..
        }) => format!("a face, bounded {bounded}"),
        Err(e) => panic!("the meter refuses a face or passes: {e:?}"),
    };
    for (w, want) in [(0.45, "a face, bounded true"), (0.6, "clear")] {
        let (fb, fe) = trapezoid(w);
        let ib: Body<Interval> = sweep::test_support::prism_at(
            [(0.0, 0.0), (4.0, 0.0), (4.0, w), (-1.0, w)]
                .into_iter()
                .map(|(x, y)| (p2(x, y), iv(0.0)))
                .collect(),
            iv(0.0),
            iv(2.0),
            tol(),
        );
        let ie = trapezoid_edges(&ib, w);
        assert_eq!(ie.len(), 3, "w = {w}: the chain's three top edges");
        let at_f64 = verdict(band_reach(
            &BlendRequest {
                body: &fb,
                edges: fe,
                size: 0.5,
            },
            band(),
        ));
        let at_interval = verdict(band_reach(
            &BlendRequest {
                body: &ib,
                edges: ie,
                size: iv(0.5),
            },
            band(),
        ));
        assert_eq!(at_f64, want, "w = {w}: f64");
        assert_eq!(at_interval, want, "w = {w}: Interval");
    }
}

/// The top-front edge `x ∈ [0, 4]` of a prism whose end face runs from
/// `(4, 0)` back to `(3.4, 3)`, cut by a station at `x = s` into two
/// collinear links, one chain through a valence-2 joint. At `r = 0.5`
/// the end face lies within the band over `x ∈ [3.9, 4]`, the near
/// link's own sliver of it.
fn leaning_cut_off(station: Option<f64>) -> (Body<f64>, Vec<EdgeKey>) {
    let body = prism(
        &[
            Point2::new(0.0, 0.0),
            Point2::new(4.0, 0.0),
            Point2::new(3.4, 3.0),
            Point2::new(0.0, 3.0),
        ],
        0.0,
        2.0,
    );
    let rim =
        |b: &Body<f64>| edges_with_corners(b, |q| q.y.abs() < 1e-12 && (q.z - 2.0).abs() < 1e-12);
    let body = match station {
        None => body,
        Some(s) => {
            let edge = rim(&body);
            cut_stations(body, edge[0], &[Point3::new(s, 0.0, 2.0)], tol())
        }
    };
    let edges = rim(&body);
    assert_eq!(
        edges.len(),
        1 + usize::from(station.is_some()),
        "{station:?}: the rim's links"
    );
    (body, edges)
}

/// **The witness**: a station at `x = 3.95`, inside the end face's span
/// in the band, leaves the near link `[0, 3.95]` a band the end face —
/// at the far link's end, no vertex of the near one — cuts across the
/// joint. The meter refuses the end face uncertified; under the
/// chain-wide skip it passed. The door's joint-foot screen refuses the
/// body first. A station at `x = 2`, and none, are clear at the meter
/// and build.
#[test]
fn a_jointed_chains_far_end_face_is_metered_against_the_near_link() {
    let r = 0.5;
    let (body, edges) = leaning_cut_off(Some(3.95));
    let req = BlendRequest {
        body: &body,
        edges: edges.clone(),
        size: r,
    };
    let e = band_reach(&req, band()).expect_err("the end face cuts the near link's band");
    let BlendError::FaceClearance {
        at: EntityId::Face(f),
        bounded: true,
        ..
    } = e
    else {
        panic!("the meter refuses a face uncertified: {e:?}");
    };
    let ([nx, ny, _], _) = plane_of(&body, f);
    assert!(
        (nx.abs() - 3.0 / 9.36f64.sqrt()).abs() < 1e-9
            && (ny.abs() - 0.6 / 9.36f64.sqrt()).abs() < 1e-9,
        "the face named is the leaning end face, got normal ({nx}, {ny})"
    );
    let door = fillet_edges(
        &sweep::test_support::at_rest(&body, tol()),
        &edges,
        r,
        tol(),
    )
    .map(|_| ())
    .map_err(|e| e.error);
    assert!(
        matches!(door, Err(BlendError::FaceClearanceUncertified { .. })),
        "the door's joint-foot screen refuses first: {door:?}"
    );
    for station in [Some(2.0), None] {
        let (body, edges) = leaning_cut_off(station);
        let req = BlendRequest {
            body: &body,
            edges: edges.clone(),
            size: r,
        };
        if let Err(e) = band_reach(&req, band()) {
            panic!("{station:?}: clear at the meter: {e:?}");
        }
        let f = fillet_edges(
            &sweep::test_support::at_rest(&body, tol()),
            &edges,
            r,
            tol(),
        )
        .unwrap_or_else(|e| panic!("{station:?}: builds: {:?}", e.error));
        assert_eq!(
            validate_geometric(&f.body, tol()),
            Ok(()),
            "{station:?}: tier 3"
        );
    }
}

/// A line `AB` running into an arc wall at `B`: the window ends in a
/// curved face, which no cap bounds, so the meter refuses to skip it and
/// says the battery's promise broke; the door refuses the curved end face
/// first.
#[test]
fn a_curved_end_face_fails_loud_at_the_meter() {
    let lp = profile::test_support::bulge_loop(vec![
        (Point2::new(0.0, 0.0), 0.0),
        (Point2::new(4.0, 0.0), 2.5),
        (Point2::new(3.0, 0.45), 0.0),
    ]);
    let body =
        sweep::test_support::extruded(crate::common::cavity::sketch_at(0.0), vec![lp], 2.0, tol());
    let edges = edges_with_corners(&body, |q| q.y.abs() < 1e-12 && (q.z - 2.0).abs() < 1e-12);
    assert_eq!(edges.len(), 1, "the line AB");
    let req = BlendRequest {
        body: &body,
        edges: edges.clone(),
        size: 0.5,
    };
    let e = band_reach(&req, band()).expect_err("a curved end face bounds no window");
    assert!(
        matches!(
            e,
            BlendError::SurgeryInvariant {
                at: EntityId::Face(_),
                ..
            }
        ),
        "the meter fails loud on the curved face: {e:?}"
    );
    let door = fillet_edges(
        &sweep::test_support::at_rest(&body, tol()),
        &edges,
        0.5,
        tol(),
    )
    .map(|_| ())
    .map_err(|e| e.error);
    assert!(
        matches!(
            door,
            Err(BlendError::UnsupportedRunOut { detail, .. })
                if detail == sweep::blend::battery::END_FACE_CURVED
        ),
        "the battery refuses the curved end face: {door:?}"
    );
}

/// The top-front edge turning down the front-right edge into the
/// bottom-right one, on a box of height `h`, `r = 0.5`: three chains
/// meeting at two turns, the first and the last sharing no support. Below
/// `h = 1` their bands overlap along the middle edge, and the meter and
/// the door refuse the other chain's band; past it the body builds.
#[test]
fn the_bands_of_chains_that_meet_at_turns_are_metered_against_each_other() {
    let p = Point3::new;
    let near = |a: f64, b: f64| (a - b).abs() < 1e-12;
    for h in [0.9, 0.98, 1.005, 1.02] {
        let body = brick(p(0.0, 0.0, 0.0), p(4.0, 3.0, h));
        let edges: Vec<_> = [
            edges_with_corners(&body, |q| near(q.y, 0.0) && near(q.z, h)),
            edges_with_corners(&body, |q| near(q.x, 4.0) && near(q.y, 0.0)),
            edges_with_corners(&body, |q| near(q.x, 4.0) && near(q.z, 0.0)),
        ]
        .concat();
        assert_eq!(edges.len(), 3, "h = {h}: the three edges");
        let req = BlendRequest {
            body: &body,
            edges: edges.clone(),
            size: 0.5,
        };
        let meter = band_reach(&req, band());
        let door = fillet_edges(
            &sweep::test_support::at_rest(&body, tol()),
            &edges,
            0.5,
            tol(),
        )
        .map(|_| ())
        .map_err(|e| e.error);
        for (what, out) in [("meter", meter), ("door", door)] {
            match out {
                Err(BlendError::FaceClearance {
                    at: EntityId::Edge(_),
                    bounded: true,
                    ..
                }) if h < 1.0 => {}
                Ok(()) if h > 1.0 => {}
                out => panic!("h = {h}: {what}: {out:?}"),
            }
        }
    }
}

/// Just past the width at which a far end face touches the band, nothing
/// over-refuses: the trapezoid's back face at `w = 0.51` and `0.55`, and
/// an oblique back face ending the left edge `d = 0.51` and `0.55` past
/// a turn from the front edge, are clear at the meter and build.
#[test]
fn a_far_end_face_just_clear_of_the_band_builds() {
    let near = |a: f64, b: f64| (a - b).abs() < 1e-12;
    let mut cases: Vec<(String, Body<f64>, Vec<EdgeKey>)> = Vec::new();
    for w in [0.51, 0.55] {
        let (body, edges) = trapezoid(w);
        cases.push((format!("trapezoid w = {w}"), body, edges));
    }
    for d in [0.51, 0.55] {
        let body = prism(
            &[
                Point2::new(0.0, 0.0),
                Point2::new(4.0, 0.0),
                Point2::new(4.0, 3.0),
                Point2::new(0.0, d),
            ],
            0.0,
            2.0,
        );
        let edges = [
            edges_with_corners(&body, |q| near(q.y, 0.0) && near(q.z, 2.0)),
            edges_with_corners(&body, |q| near(q.x, 0.0) && near(q.z, 2.0)),
        ]
        .concat();
        assert_eq!(edges.len(), 2, "d = {d}: the front and left edges");
        cases.push((format!("oblique back d = {d}"), body, edges));
    }
    for (what, body, edges) in cases {
        let req = BlendRequest {
            body: &body,
            edges: edges.clone(),
            size: 0.5,
        };
        if let Err(e) = band_reach(&req, band()) {
            panic!("{what}: clear at the meter: {e:?}");
        }
        let f = fillet_edges(
            &sweep::test_support::at_rest(&body, tol()),
            &edges,
            0.5,
            tol(),
        )
        .unwrap_or_else(|e| panic!("{what}: builds: {:?}", e.error));
        assert_eq!(validate_geometric(&f.body, tol()), Ok(()), "{what}: tier 3");
    }
}

/// The left edge turning into the front edge, which ends at the
/// front-right corner patch with the front-right and top-right edges:
/// the right face, at the front link's far end and a support of the
/// patch's other chains, stays clear of the left link's reach on a box
/// as narrow as `x = 1.05`, and the body builds.
#[test]
fn a_chain_into_a_corner_patch_builds() {
    let p = Point3::new;
    let near = |a: f64, b: f64| (a - b).abs() < 1e-12;
    for wx in [4.0, 1.05] {
        let body = brick(p(0.0, 0.0, 0.0), p(wx, 4.0, 2.0));
        let edges = [
            edges_with_corners(&body, |q| near(q.y, 0.0) && near(q.z, 2.0)),
            edges_with_corners(&body, |q| near(q.x, 0.0) && near(q.z, 2.0)),
            edges_with_corners(&body, |q| near(q.x, wx) && near(q.y, 0.0)),
            edges_with_corners(&body, |q| near(q.x, wx) && near(q.z, 2.0)),
        ]
        .concat();
        assert_eq!(edges.len(), 4, "wx = {wx}: the four edges");
        let req = BlendRequest {
            body: &body,
            edges: edges.clone(),
            size: 0.5,
        };
        if let Err(e) = band_reach(&req, band()) {
            panic!("wx = {wx}: clear at the meter: {e:?}");
        }
        let f = fillet_edges(
            &sweep::test_support::at_rest(&body, tol()),
            &edges,
            0.5,
            tol(),
        )
        .unwrap_or_else(|e| panic!("wx = {wx}: builds: {:?}", e.error));
        assert_eq!(
            validate_geometric(&f.body, tol()),
            Ok(()),
            "wx = {wx}: tier 3"
        );
    }
}
