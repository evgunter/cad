//! **`Body::split_edge`'s pcurve-row carry on a DESCRIBED-NURBS chart**,
//! and the frontier that sits beside it.
//!
//! `loft_body`'s prism walls are spline charts, and their rows are the
//! two kinds a cylinder-wall fixture never presents: `Pcurve::IsoLine`
//! (a square profile's flat walls) and `Pcurve::IsoArc` (a bulged
//! profile's arc rims). The carry restricts both exactly, like any
//! other kind, and the split body is tier-3 valid with no caller mint.
//!
//! What a spline chart does not have is the RECOVERY step. `mint_pcurves`
//! — the pass every "this face is left rowless" caveat names — mints
//! these bodies happily UNSPLIT and refuses on the body a split
//! produces, because the iso derivation's rim arms map an edge's WHOLE
//! carrier interval onto the chart's whole `u` domain, which a sub-edge
//! no longer spans. The refusal is the arms' and not the carry's: that
//! pass CLEARS the map before it derives, so no row this op wrote is
//! an input to it, and the same call refuses the same way on a tree
//! where the split leaves the children rowless. Filed on TRIM's slate
//! as `iso-derivation-arms-assume-an-edge-spans-the-charts-whole-domain`
//! and pinned below, so the frontier is stated rather than silent.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_brep::PcurveCertifyError;
use geom_core::{Point2, Tol};
use profile::test_support::bulge_loop;
use sweep::test_support::stacked_at;
use topo::pcurves::PcurveMintError;
use topo::{Body, EdgeKey, Pcurve};

fn tol() -> Tol {
    Tol::witness()
}

/// A square-profile prism: flat walls, so every wall row is `IsoLine`.
fn line_prism() -> Body<f64> {
    let v = |x: f64, y: f64| (Point2::new(x, y), 0.0);
    let sq = || {
        vec![bulge_loop(vec![
            v(0.0, 0.0),
            v(2.0, 0.0),
            v(2.0, 2.0),
            v(0.0, 2.0),
        ])]
    };
    sweep::loft_body::<f64>(&[sq(), sq()], &stacked_at(&[0.0, 1.0]), 1, tol())
        .expect("the prism builds")
        .body
}

/// A bulged-profile prism: one profile edge is an arc, so that wall's
/// rims carry `IsoArc` rows — the chart's own rational-quadratic
/// parameter rather than the arc angle. The bulge is chosen so the arc
/// is a THREE-span rational quadratic (`breaks` has interior knots at
/// 1/3 and 2/3): a restriction that re-parameterized the map rather
/// than restricting it would move the image at a break first, and a
/// single-span arc could not tell the two apart.
fn bulged_prism() -> Body<f64> {
    let v = |x: f64, y: f64, bulge: f64| (Point2::new(x, y), bulge);
    let bulged = || {
        vec![bulge_loop(vec![
            v(0.0, 0.0, 0.0),
            v(2.0, 0.0, 1.6),
            v(2.0, 2.0, 0.0),
            v(0.0, 2.0, 0.0),
        ])]
    };
    sweep::loft_body::<f64>(&[bulged(), bulged()], &stacked_at(&[0.0, 1.0]), 1, tol())
        .expect("the bulged prism builds")
        .body
}

fn kind(p: &Pcurve<f64>) -> &'static str {
    match p {
        Pcurve::Harmonic { .. } => "Harmonic",
        Pcurve::IsoLine { .. } => "IsoLine",
        Pcurve::IsoArc { .. } => "IsoArc",
        Pcurve::Fitted(_) => "Fitted",
        Pcurve::General(_) => "General",
    }
}

fn body_of(which: &str) -> Body<f64> {
    match which {
        "IsoLine" => line_prism(),
        "IsoArc" => bulged_prism(),
        other => panic!("no fixture for {other}"),
    }
}

/// An edge one of whose half-edges stores a row of `want`, with the
/// edge's own certified parameter interval.
fn edge_with_kind(body: &Body<f64>, want: &str) -> (EdgeKey, (f64, f64)) {
    for (he, c) in body.pcurves() {
        if kind(c.pcurve()) != want {
            continue;
        }
        let h = body.get_half_edge(he).unwrap();
        let params = body
            .get_curve_geom(body.get_edge(h.edge).unwrap().curve)
            .and_then(topo::CurveGeom::certified)
            .unwrap()
            .params();
        return (h.edge, params);
    }
    panic!("this fixture stores no {want} row");
}

/// **The carry is exact on a spline chart too.** Splitting a wall edge
/// whose row is `IsoLine` or `IsoArc` leaves each child half-edge the
/// PARENT's chart image — the same `Debug` bytes — with only the
/// interval moved, and the body tier-3 valid with no caller mint.
///
/// Split near `t₀`, at mid-parameter, near `t₁`, and at fractions whose
/// sub-intervals straddle the arc chart's interior `breaks` knots
/// (1/3 and 2/3 — see the fixture).
#[test]
fn a_split_on_a_spline_chart_carries_the_parents_image() {
    for which in ["IsoLine", "IsoArc"] {
        for frac in [0.02_f64, 0.3, 0.5, 0.7, 0.98] {
            let mut body = body_of(which);
            let (edge, (t0, t1)) = edge_with_kind(&body, which);
            let parent: Vec<Option<String>> = {
                let e = body.get_edge(edge).unwrap();
                [e.he_plus, e.he_minus]
                    .into_iter()
                    .map(|he| body.pcurve(he).map(|c| format!("{:?}", c.pcurve())))
                    .collect()
            };
            let t = t0 + (t1 - t0) * frac;
            let created = body.split_edge(edge, t, tol()).unwrap();
            assert_eq!(
                topo::validate_geometric(&body, tol()),
                Ok(()),
                "{which} @{frac}: tier 3 after the split"
            );
            let e = body.get_edge(edge).unwrap();
            for (i, (parent_half, new_half)) in
                [(e.he_plus, created.he_plus), (e.he_minus, created.he_minus)]
                    .into_iter()
                    .enumerate()
            {
                for (he, want) in [(parent_half, (t0, t)), (new_half, (t, t1))] {
                    let stored = body.pcurve(he).map(|c| format!("{:?}", c.pcurve()));
                    assert_eq!(
                        stored, parent[i],
                        "{which} @{frac}: {he:?}'s image is not the parent's"
                    );
                    if parent[i].is_some() {
                        assert_eq!(
                            body.pcurve(he).unwrap().params(),
                            want,
                            "{which} @{frac}: {he:?}'s interval"
                        );
                    }
                }
            }
        }
    }
}

/// **The frontier, pinned as the current behaviour.** `mint_pcurves`
/// mints either loft body UNSPLIT — so the pass has nothing against
/// these charts — and refuses on the body a split of one of their wall
/// edges produces, naming the sub-edge that no longer spans the
/// chart's `u` domain. The refusals are the iso derivation's, filed on
/// TRIM's slate; this row exists so that a fix there reds here and the
/// state is never read off a silent suite.
#[test]
fn the_mint_pass_accepts_these_charts_unsplit_and_refuses_after_a_split() {
    for which in ["IsoLine", "IsoArc"] {
        let mut control = body_of(which);
        assert_eq!(
            topo::mint_pcurves(&mut control, tol()),
            Ok(()),
            "{which}: the control — the mint pass on the UNSPLIT loft body"
        );

        let mut body = body_of(which);
        let (edge, (t0, t1)) = edge_with_kind(&body, which);
        body.split_edge(edge, (t0 + t1) * 0.5, tol()).unwrap();
        let refusal = topo::mint_pcurves(&mut body, tol());
        match (which, &refusal) {
            // The arc rim's own arm refuses typed: its two candidate
            // images are the chart's whole `u` domain in each
            // direction, and a half-rim matches neither.
            (
                "IsoArc",
                Err(PcurveMintError::Certify {
                    error: PcurveCertifyError::IsoUnsupported { .. },
                    ..
                }),
            ) => {}
            // The line arm mints an image for the sub-edge that spans
            // the whole column, so the refusal arrives one step later,
            // as the loop walk's discontinuity.
            ("IsoLine", Err(PcurveMintError::LoopDiscontinuity { .. })) => {}
            _ => panic!("{which}: the mint pass after a split read {refusal:?}"),
        }
    }
}
