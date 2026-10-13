//!

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::Surface;
use geom_brep::PcurveCertifyError;
use geom_core::{Band, Point2, Tol};
use profile::test_support::bulge_loop;
use sweep::test_support::stacked_at;
use topo::pcurves::PcurveMintError;
use topo::{Body, FaceKey, FaceSurface, HalfEdgeKey, Pcurve};

fn tol() -> Tol {
    Tol::witness()
}

fn band() -> Band {
    Band::linear(tol()).unwrap()
}

/// A prism whose profile's second edge is a semicircle (bulge 1), so
/// its wall's rims are half-period arcs, `breaks` mirror-symmetric and
/// the wall reversible in `u` bitwise.
fn semicircle_prism() -> Body<f64> {
    let v = |x: f64, y: f64, bulge: f64| (Point2::new(x, y), bulge);
    let profile = || {
        vec![bulge_loop(vec![
            v(0.0, 0.0, 0.0),
            v(2.0, 0.0, 1.0),
            v(2.0, 2.0, 0.0),
            v(0.0, 2.0, 0.0),
        ])]
    };
    sweep::loft_body::<f64>(&[profile(), profile()], &stacked_at(&[0.0, 1.0]), 1, tol())
        .expect("the prism builds")
        .body
}

/// The arc wall and its two rim half-edges, `v = 0` first.
fn arc_wall(body: &Body<f64>) -> (FaceKey, [HalfEdgeKey; 2]) {
    let mut rims: Vec<(f64, HalfEdgeKey)> = body
        .pcurves()
        .filter_map(|(h, c)| match c.pcurve() {
            Pcurve::IsoArc { p0, .. } => Some((p0.y, h)),
            _ => None,
        })
        .collect();
    rims.sort_by(|a, b| a.0.total_cmp(&b.0));
    let [(_, bottom), (_, top)] = rims.as_slice() else {
        panic!("the arc wall's two rims are its IsoArc rows: {rims:?}")
    };
    let face = body.face_of_half_edge(*bottom).unwrap();
    assert_eq!(body.face_of_half_edge(*top), Some(face), "one wall");
    (face, [*bottom, *top])
}

/// What `nurbs_iso_derive` answers for `h`, the stored row detached:
/// `pcurve_of` answers a cached image first.
fn derived(body: &Body<f64>, h: HalfEdgeKey) -> Result<Pcurve<f64>, PcurveMintError> {
    let mut fresh = body.clone();
    fresh.detach_pcurve(h);
    topo::pcurve_of(&fresh, h, band())
}

/// `(p0.x, pd.x, p0.y)` of an IsoArc image.
fn arc(image: &Pcurve<f64>) -> (f64, f64, f64) {
    match image {
        Pcurve::IsoArc { p0, pd, .. } => (p0.x, pd.x, p0.y),
        other => panic!("an arc rim's image is an IsoArc: {other:?}"),
    }
}

fn recharted(
    body: &Body<f64>,
    wall: FaceKey,
    f: impl Fn(&geom::NurbsSurface<f64>) -> geom::NurbsSurface<f64>,
    flip: bool,
) -> Body<f64> {
    let mut b = body.clone();
    let face = b.get_face(wall).unwrap();
    let (key, sense) = (face.surface, face.sense);
    let Some(Surface::Nurbs(net)) = b.get_surface(key).cloned() else {
        panic!()
    };
    b.set_face_surface_unvouched_for_tests(
        wall,
        FaceSurface::New {
            surface: Surface::Nurbs(std::sync::Arc::new(f(&net))),
            sense: if flip { !sense } else { sense },
        },
    )
    .unwrap();
    b
}

/// P1: a wall re-charted with `v` reversed: each rim now lies on the
/// OTHER row and runs with `u`. The row pick must follow the row.
#[test]
fn r4988_p1_reversed_v_rim_takes_the_other_row() {
    let body = semicircle_prism();
    let (wall, rims) = arc_wall(&body);
    let mut b = recharted(&body, wall, |n| n.reversed_v().unwrap(), true);
    for (h, row) in rims.into_iter().zip([1.0, 0.0]) {
        let got = derived(&b, h);
        eprintln!("P1 {h:?}: {got:?}");
        assert_eq!(arc(&got.unwrap()), (0.0, 1.0, row));
    }
    assert_eq!(topo::mint_pcurves(&mut b, tol()), Ok(()));
}

/// P2: both reversed: each rim on the other row AND against `u`.
#[test]
fn r4988_p2_reversed_uv_rim_takes_the_other_row_reversed() {
    let body = semicircle_prism();
    let (wall, rims) = arc_wall(&body);
    let mut b = recharted(
        &body,
        wall,
        |n| n.reversed_u().unwrap().reversed_v().unwrap(),
        false,
    );
    for (h, row) in rims.into_iter().zip([1.0, 0.0]) {
        let got = derived(&b, h);
        eprintln!("P2 {h:?}: {got:?}");
        assert_eq!(arc(&got.unwrap()), (1.0, -1.0, row));
    }
    assert_eq!(topo::mint_pcurves(&mut b, tol()), Ok(()));
}

/// P3: the reversed-u wall's half-rims (split after the re-chart):
/// both halves still refuse typed, for the map, at the mint too.
#[test]
fn r4988_p3_half_rims_of_a_reversed_wall_refuse_typed() {
    let body = semicircle_prism();
    let (wall, [bottom, _]) = arc_wall(&body);
    let mut b = recharted(&body, wall, |n| n.reversed_u().unwrap(), true);
    let edge = b.get_half_edge(bottom).unwrap().edge;
    let (t0, t1) = b
        .get_curve_geom(b.get_edge(edge).unwrap().curve)
        .and_then(topo::CurveGeom::certified)
        .unwrap()
        .params();
    let made = b.split_edge(edge, 0.5 * (t0 + t1), tol()).unwrap();
    for h in [made.he_plus, made.he_minus, bottom] {
        if b.face_of_half_edge(h) != Some(wall) {
            continue;
        }
        let got = derived(&b, h);
        eprintln!("P3 {h:?}: {got:?}");
        assert!(matches!(
            got,
            Err(PcurveMintError::Certify {
                error: PcurveCertifyError::IsoUnsupported { .. },
                ..
            })
        ));
    }
    let minted = topo::mint_pcurves(&mut b, tol());
    eprintln!("P3 mint: {minted:?}");
    assert!(minted.is_err());
}

/// P4: transposed wall (rims become columns): the arm offers rows only;
/// record what it answers (typed refusal expected, never a wrong row).
#[test]
fn r4988_p4_transposed_wall_rims_answer_typed() {
    let body = semicircle_prism();
    let (wall, rims) = arc_wall(&body);
    let mut b = recharted(&body, wall, |n| n.transposed(), true);
    for h in rims {
        let got = derived(&b, h);
        eprintln!("P4 {h:?}: {got:?}");
        assert!(got.is_err(), "{got:?}");
    }
    let minted = topo::mint_pcurves(&mut b, tol());
    eprintln!("P4 mint: {minted:?}");
    assert!(minted.is_err());
}

/// P5: of the four candidates the arm offers, `PcurveCache::certify`
/// alone accepts exactly the right one, per rim, native and reversed-u:
/// a wrong row or direction the probe let through would not reach rest.
#[test]
fn r4988_p5_certify_alone_accepts_only_the_true_candidate() {
    let body = semicircle_prism();
    let (wall, rims) = arc_wall(&body);
    let rev = recharted(&body, wall, |n| n.reversed_u().unwrap(), true);
    for (b, label) in [(&body, "native"), (&rev, "reversed_u")] {
        let surface = b
            .get_surface(b.get_face(wall).unwrap().surface)
            .cloned()
            .unwrap();
        for h in rims {
            let truth = derived(b, h).unwrap();
            let Pcurve::IsoArc {
                t0, angle, breaks, ..
            } = truth.clone()
            else {
                panic!()
            };
            let ec = b
                .get_curve_geom(b.get_edge(b.get_half_edge(h).unwrap().edge).unwrap().curve)
                .and_then(topo::CurveGeom::certified)
                .unwrap()
                .clone();
            let (e0, e1) = ec.params();
            let mut ok = vec![];
            for v in [0.0, 1.0] {
                for (p0x, s) in [(0.0, 1.0), (1.0, -1.0)] {
                    let cand = Pcurve::IsoArc {
                        p0: Point2::new(p0x, v),
                        pd: geom_core::Vec2::new(s, 0.0),
                        t0,
                        angle,
                        breaks: breaks.clone(),
                    };
                    let r = geom_brep::PcurveCache::certify(
                        cand,
                        e0,
                        e1,
                        ec.carrier(),
                        &surface,
                        band(),
                    );
                    eprintln!(
                        "P5 {label} {h:?} v={v} p0x={p0x}: {:?}",
                        r.as_ref().map(|_| ())
                    );
                    if r.is_ok() {
                        ok.push((p0x, s, v));
                    }
                }
            }
            assert_eq!(ok, vec![arc(&truth)], "{label} {h:?}");
        }
    }
}
