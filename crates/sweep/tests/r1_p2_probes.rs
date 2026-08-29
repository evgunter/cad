//! **PCURVE P-2, R1 consumer probes — the interior-column deriver, the
//! rim-arm widening, and the certifier's teeth.**
//!
//! Independent adversarial rows, not a re-run of the unit's own. The
//! fixture helpers are copied VERBATIM from `m8_4_intersection_iso.rs`
//! (the fixture is shared ground; the assertions are not), plus one
//! generalization: a chart widened by `k` columns each way, so that
//! SEVERAL interior columns exist and "the right column" is
//! distinguishable from "a column that merely certifies".
//!
//! What each row attacks:
//!
//! - `r1_general_image_is_the_measured_column_among_many`: on a chart
//!   with four interior knot columns, the derived `General` image must
//!   sit on the column the CARRIER actually occupies — verified in
//!   METRES against the carrier, not against the deriver's own pick.
//! - `r1_certify_general_refuses_a_plausible_wrong_column`: the same
//!   certificate door must RED on the neighbouring knot column — if it
//!   did not, every claim resting on "certified" would be vacuous.
//! - `r1_cap_rim_widening_measures_the_face_not_the_chart`: the kept
//!   half of the rim widening — the measured `u` map must land on the
//!   face's own patch and match the carrier pointwise in metres.
//! - `r1_wall_seam_arm_still_refuses_the_interior_column`: the
//!   reverted half — a `Chart`-described wall-wall seam on the widened
//!   chart must refuse typed, NOT mint the measured column.
//! - `r1_general_image_is_operand_order_blind`: the deriver must not
//!   see the description's operand order.
//! - `r1_dual_scalar_still_reaches_the_mint`: the lane bound is
//!   signature churn, not capability loss — `Dual64` still names the
//!   mint entry points.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::Curve3;
use geom::{NurbsSurface, Surface};
use geom_brep::{EdgeCurveSpec, EdgeDescriptionSpec};
use geom_core::Tol;
use geom_core::spline::KnotVector;
use geom_core::{Affine3, Band, Point2, Point3, Vec3};
use profile::RawLoop;
use std::sync::Arc;
use topo::{Body, FaceSurface, Pcurve, PcurveMintError};

// ---------------------------------------------------------------- //
// Fixture helpers, verbatim from m8_4_intersection_iso.rs           //
// ---------------------------------------------------------------- //

fn prism(scale: f64) -> Body<f64> {
    let square = move || -> sweep::Section {
        let v = |x: f64, y: f64| profile::ProfileVertex::new(Point2::new(x, y), 0.0);
        vec![profile::ProfileLoop::new(vec![
            v(-scale, -scale),
            v(scale, -scale),
            v(scale, scale),
            v(-scale, scale),
        ])]
    };
    let sections = vec![square(), square(), square()];
    let places = vec![
        Affine3::identity(),
        Affine3::translation(Vec3::new(0.5 * scale, 0.0, 1.0 * scale)),
        Affine3::translation(Vec3::new(0.0, 0.0, 2.0 * scale)),
    ];
    sweep::loft_body::<f64>(&sections, &places, 2, Tol::witness())
        .expect("the offset square prism builds")
        .body
}

fn band() -> Band {
    Band::linear(Tol::witness()).unwrap()
}

fn is_flat_wall(body: &Body<f64>, key: topo::SurfaceKey, scale: f64) -> bool {
    matches!(body.get_surface(key), Some(Surface::Nurbs(n))
        if !n.is_placeholder() && n.control().iter().all(|p| p.y == -scale))
}

fn is_bowed_wall(body: &Body<f64>, key: topo::SurfaceKey, scale: f64) -> bool {
    matches!(body.get_surface(key), Some(Surface::Nurbs(n))
        if !n.is_placeholder() && n.control().iter().any(|p| p.y != -scale)
            && n.control().iter().any(|p| p.x.abs() == scale))
}

fn he_surface(body: &Body<f64>, he: topo::HalfEdgeKey) -> topo::SurfaceKey {
    let hed = body.get_half_edge(he).unwrap();
    let lp = body.get_loop(hed.parent_loop).unwrap();
    body.get_face(lp.face).unwrap().surface
}

#[allow(clippy::type_complexity)]
fn flat_bowed_seam(
    body: &Body<f64>,
    scale: f64,
) -> (
    topo::EdgeKey,
    topo::SurfaceKey,
    topo::SurfaceKey,
    topo::HalfEdgeKey,
) {
    for (ek, edge) in body.edges() {
        let (sp, sm) = (
            he_surface(body, edge.he_plus),
            he_surface(body, edge.he_minus),
        );
        let carrier_is_spline = matches!(
            body.get_curve_geom(edge.curve),
            Some(topo::CurveGeom::Certified(c)) if matches!(c.carrier(), Curve3::Nurbs(_))
        );
        if !carrier_is_spline {
            continue;
        }
        if is_flat_wall(body, sp, scale) && is_bowed_wall(body, sm, scale) {
            return (ek, sp, sm, edge.he_minus);
        }
        if is_flat_wall(body, sm, scale) && is_bowed_wall(body, sp, scale) {
            return (ek, sm, sp, edge.he_plus);
        }
    }
    panic!("the offset square prism has a flat-wall/bowed-wall seam");
}

#[allow(clippy::type_complexity)]
fn intrinsic_seam_at(
    swap: bool,
    scale: f64,
) -> Result<(Body<f64>, topo::HalfEdgeKey, topo::SurfaceKey), topo::EulerOpError> {
    let mut body = prism(scale);
    let (edge, flat, bowed, he_bowed) = flat_bowed_seam(&body, scale);
    let flat_face = {
        let (fk, _) = body
            .faces()
            .find(|(_, f)| f.surface == flat)
            .expect("the flat wall has a face");
        fk
    };
    let (carrier, t0, t1) = {
        let Some(topo::CurveGeom::Certified(c)) =
            body.get_curve_geom(body.get_edge(edge).expect("the seam resolves").curve)
        else {
            panic!("the seam's carrier is certified");
        };
        let (a, b) = c.params();
        (c.carrier().clone(), a, b)
    };
    let plane = body
        .set_face_surface(
            flat_face,
            FaceSurface::New(Surface::Plane {
                origin: Point3::new(0.0, -scale, 0.0),
                normal: Vec3::new(0.0, -1.0, 0.0),
                u_ref: Vec3::new(1.0, 0.0, 0.0),
            }),
        )
        .expect("the exactly-planar wall restates as a plane");
    let (s1, s2) = if swap { (bowed, plane) } else { (plane, bowed) };
    body.set_edge_curve_nurbs_lane(
        edge,
        EdgeCurveSpec {
            description: EdgeDescriptionSpec::Intersection {
                s1,
                s2,
                witness: carrier.eval((t0 + t1) * 0.5),
            },
            carrier,
            param_start: t0,
            param_end: t1,
        },
        Tol::witness(),
    )?;
    body.detach_pcurve(he_bowed);
    Ok((body, he_bowed, bowed))
}

fn chart_of(body: &Body<f64>, key: topo::SurfaceKey) -> NurbsSurface<f64> {
    match body.get_surface(key) {
        Some(Surface::Nurbs(n)) => (**n).clone(),
        other => panic!("the bowed wall is a described NURBS chart: {other:?}"),
    }
}

fn rechart(body: &mut Body<f64>, old: topo::SurfaceKey, new: Surface<f64>) -> topo::SurfaceKey {
    let (fk, _) = body
        .faces()
        .find(|(_, f)| f.surface == old)
        .expect("the bowed wall has a face");
    body.set_face_surface(fk, FaceSurface::New(new))
        .expect("the wall takes its restated chart")
}

/// The unit's scale lever, reused at its own value.
const SCALE: f64 = 1.0 / 1024.0;

// ---------------------------------------------------------------- //
// This suite's own machinery                                        //
// ---------------------------------------------------------------- //

/// [`m8_4_intersection_iso`]'s `widened_u_chart`, generalized: the
/// degree-1 `u` net continued linearly `k` columns EACH way, so the
/// face's patch occupies `u ∈ [k, k+1]` of a `[0, 2k+1]` chart and the
/// chart carries `2k` interior knot columns — several plausible
/// answers, one correct one.
fn widened_u_chart_by(n: &NurbsSurface<f64>, k: usize) -> Surface<f64> {
    let (nu, nv) = n.control_counts();
    assert_eq!((nu, n.knots_u().degree()), (2, 1), "the loft wall's u span");
    let cols = 2 * k + 2;
    let knots: Vec<f64> = std::iter::once(0.0)
        .chain((0..=(cols - 1)).map(|i| i as f64))
        .chain(std::iter::once((cols - 1) as f64))
        .collect();
    let ku = KnotVector::clamped(knots, 1).unwrap();
    let (mut control, mut weights) = (Vec::new(), Vec::new());
    for i in 0..cols {
        for j in 0..nv {
            let (a, b) = (n.control()[j], n.control()[nv + j]);
            // Column k is `a`, column k+1 is `b`; the rest continue
            // the line through them.
            let f = i as f64 - k as f64;
            control.push(a + (b - a) * f);
            weights.push(n.weights()[if i <= k { j } else { nv + j }]);
        }
    }
    Surface::Nurbs(Arc::new(
        NurbsSurface::new(ku, n.knots_v().clone(), control, weights).unwrap(),
    ))
}

/// The widened fixture: seam re-described intrinsically, chart widened
/// by `k` columns each way, stale caches gone. Returns the body, the
/// seam's bowed-side half-edge, and the NEW chart key.
fn widened_fixture(swap: bool, k: usize) -> (Body<f64>, topo::HalfEdgeKey, topo::SurfaceKey) {
    let (mut body, he, bowed) = intrinsic_seam_at(swap, SCALE)
        .expect("the seam attaches at every ε this matrix draws (the scale lever)");
    let widened = widened_u_chart_by(&chart_of(&body, bowed), k);
    let key = rechart(&mut body, bowed, widened);
    (body, he, key)
}

/// The carrier and params of a half-edge's edge.
fn carrier_of(body: &Body<f64>, he: topo::HalfEdgeKey) -> (Curve3<f64>, f64, f64) {
    let edge = body.get_edge(body.get_half_edge(he).unwrap().edge).unwrap();
    let Some(topo::CurveGeom::Certified(c)) = body.get_curve_geom(edge.curve) else {
        panic!("the carrier is certified")
    };
    let (t0, t1) = c.params();
    (c.carrier().clone(), t0, t1)
}

/// The plane operand of the seam's intersection description (the mate
/// the fitted-grade certificate needs) — same read as the unit's
/// `seam_operands`, restated here independently.
fn seam_plane(body: &Body<f64>, he: topo::HalfEdgeKey) -> Surface<f64> {
    let edge = body.get_edge(body.get_half_edge(he).unwrap().edge).unwrap();
    let Some(topo::CurveGeom::Certified(c)) = body.get_curve_geom(edge.curve) else {
        panic!("the seam's carrier is certified")
    };
    let geom_brep::EdgeDescription::Intersection { s1, s2, .. } = *c.description() else {
        panic!("the seam is described as an intersection")
    };
    [s1, s2]
        .into_iter()
        .find_map(|k| match body.get_surface(k) {
            Some(p @ Surface::Plane { .. }) => Some(p.clone()),
            _ => None,
        })
        .expect("one operand is the restated plane")
}

// ---------------------------------------------------------------- //
// The rows                                                          //
// ---------------------------------------------------------------- //

/// **Does the deriver find the RIGHT column, or one that merely
/// certifies?** Chart widened two columns each way: interior knots at
/// `u ∈ {1, 2, 3, 4}`, the face's patch on `[2, 3]`. The `General`
/// image must hold `u` constant at ONE of those columns, and the
/// arbiter of WHICH is the carrier itself, in metres: `S(P(t))` must
/// coincide with `C(t)` along the whole span, at a tolerance far below
/// the column spacing (one column over is ~`SCALE·2` metres away in
/// model space, 13 decades above the check).
#[test]
fn r1_general_image_is_the_measured_column_among_many() {
    let (body, he, key) = widened_fixture(false, 2);
    let chart = chart_of(&body, key);
    assert_eq!(chart.knots_u().domain(), (0.0, 5.0), "2k+1 = 5");
    let out = topo::pcurve_of(&body, he, band());
    let Ok(Pcurve::General(ref image)) = out else {
        panic!("an interior column's home is the General arm: {out:?}")
    };
    // Constant-u (within projection noise), strictly interior, and ON
    // a knot column.
    let u0 = image.control()[0].x;
    for p in image.control() {
        assert!(
            (p.x - u0).abs() < 1e-9,
            "the image is a COLUMN: {p:?} vs u = {u0}"
        );
    }
    assert!(
        u0 > 0.0 && u0 < 5.0,
        "and an interior one: u = {u0} of [0, 5]"
    );
    assert!(
        (u0 - u0.round()).abs() < 1e-9 && (1.0..=4.0).contains(&u0.round()),
        "and an interior KNOT column: u = {u0}"
    );
    // The arbiter: the carrier, in metres, along the whole span.
    let (carrier, t0, t1) = carrier_of(&body, he);
    for i in 0..=16 {
        let t = t0 + (t1 - t0) * (f64::from(i) / 16.0);
        let uv = image.eval(t);
        let gap = carrier.eval(t).distance(chart.eval(uv.x, uv.y));
        assert!(
            gap < 1e-10 * SCALE.max(1.0),
            "S(P({t})) is {gap:e} m from C({t}) — the image is not the carrier's \
             own column (u = {u0})"
        );
    }
    println!("R1: among interior knots {{1, 2, 3, 4}}, the deriver chose u = {u0} and it measures");
}

/// **The certificate door must RED on a plausible WRONG column.** Take
/// the correct derived image, translate it one knot column over (`u`
/// −1) — still a genuine interior knot column of the same chart, still
/// traversing the same `v` range — and offer it to the same
/// `certify_general` door with the same operands. If this certifies,
/// "certified" carries no information and every acceptance built on it
/// is vacuous.
#[test]
fn r1_certify_general_refuses_a_plausible_wrong_column() {
    let (body, he, key) = widened_fixture(false, 2);
    let chart = chart_of(&body, key);
    let out = topo::pcurve_of(&body, he, band());
    let Ok(Pcurve::General(ref image)) = out else {
        panic!("the deriver speaks first: {out:?}")
    };
    let shifted = geom::NurbsCurve2::new(
        image.knots().clone(),
        image
            .control()
            .iter()
            .map(|p| Point2::new(p.x - 1.0, p.y))
            .collect(),
        image.weights().to_vec(),
    )
    .expect("the shifted image is a well-formed curve");
    let (carrier, t0, t1) = carrier_of(&body, he);
    let mate = seam_plane(&body, he);
    let shifted = Arc::new(shifted);
    let window = Pcurve::General(Arc::clone(&shifted)).chart_box(t0, t1);
    let verdict = geom_brep::PcurveCache::certify_general(
        shifted,
        t0,
        t1,
        &carrier,
        &Surface::Nurbs(Arc::new(chart)),
        Some(&mate),
        window,
        band(),
    );
    assert!(
        verdict.is_err(),
        "a column one knot over CERTIFIED — the certificate has no teeth: {:?}",
        verdict.map(|c| *c.certificate())
    );
    println!(
        "R1: the wrong column refuses as {:?}",
        verdict
            .err()
            .map(|e| format!("{e:?}").chars().take(120).collect::<String>())
    );
}

/// **Operand order, on the INTERIOR column.** The unit's own
/// order-blindness row runs at the boundary; this one asks the same
/// question of the new path: `Intersection{plane, wall}` and
/// `Intersection{wall, plane}` must derive the same `General` image,
/// bit for bit.
#[test]
fn r1_general_image_is_operand_order_blind() {
    let (a_body, a_he, _) = widened_fixture(false, 1);
    let (b_body, b_he, _) = widened_fixture(true, 1);
    let a = topo::pcurve_of(&a_body, a_he, band());
    let b = topo::pcurve_of(&b_body, b_he, band());
    match (&a, &b) {
        (Ok(x @ Pcurve::General(_)), Ok(y @ Pcurve::General(_))) => assert_eq!(
            format!("{x:?}"),
            format!("{y:?}"),
            "the same locus, the same image, either operand order"
        ),
        other => panic!("both orders derive the General image: {other:?}"),
    }
}

/// **The kept half of the rim widening, for the reason claimed.** On
/// the widened chart the face's patch is `u ∈ [1, 2]` of `[0, 3]`; the
/// cap rims (Line carriers, `Chart`/`Scaffold` descriptions) no longer
/// span the chart, so the closed form's `v`-side pick finds nothing
/// and the arm must MEASURE. The claim under test: `u` is the MOVING
/// channel and only the map was wrong. So the derived `IsoLine` must
/// (a) move in `u` (`pl.x != 0`), (b) hold `v` on a genuine chart
/// boundary, and (c) match the carrier pointwise in metres — including
/// MID-SPAN, where a map measured only at endpoints could lie.
#[test]
fn r1_cap_rim_widening_measures_the_face_not_the_chart() {
    let (mut body, seam_he, key) = widened_fixture(false, 1);
    // `pcurve_of` short-circuits on a STORED cache, and the loft
    // minted every rim's cache against the ORIGINAL chart — the trap
    // the PR body documents. Detach them so the row reads the ARM.
    let rims: Vec<_> = body
        .edges()
        .flat_map(|(_, e)| [e.he_plus, e.he_minus])
        .filter(|he| he_surface(&body, *he) == key)
        .collect();
    for he in rims {
        body.detach_pcurve(he);
    }
    let chart = chart_of(&body, key);
    let (dv0, dv1) = chart.knots_v().domain();
    let mut rims = 0;
    for (_, edge) in body.edges() {
        for he in [edge.he_plus, edge.he_minus] {
            if he_surface(&body, he) != key || he == seam_he {
                continue;
            }
            let (carrier, t0, t1) = carrier_of(&body, he);
            if !matches!(carrier, Curve3::Line { .. }) {
                continue;
            }
            rims += 1;
            let out = topo::pcurve_of(&body, he, band());
            let Ok(Pcurve::IsoLine { p0, pl }) = out else {
                panic!("a cap rim on the widened chart derives via the measured map: {out:?}")
            };
            assert_ne!(pl.x, 0.0, "u is the MOVING channel on a rim: {pl:?}");
            assert_eq!(pl.y, 0.0, "and v is fixed: {pl:?}");
            assert!(
                p0.y == dv0 || p0.y == dv1,
                "on a genuine chart v-boundary: v = {} of [{dv0}, {dv1}]",
                p0.y
            );
            for i in 0..=8 {
                let t = t0 + (t1 - t0) * (f64::from(i) / 8.0);
                let (u, v) = (p0.x + pl.x * t, p0.y + pl.y * t);
                assert!(
                    (1.0..=2.0).contains(&u),
                    "the measured map lands on the FACE's patch [1, 2]: u({t}) = {u}"
                );
                let gap = carrier.eval(t).distance(chart.eval(u, v));
                assert!(
                    gap < 1e-10,
                    "the rim image must match the carrier mid-span too: {gap:e} m at t = {t}"
                );
            }
        }
    }
    assert!(
        rims >= 2,
        "the wall face has its two cap rims: found {rims}"
    );
    println!("R1: {rims} rim half-edges derive measured IsoLine maps on the face's own patch");
}

/// **The reverted half stays reverted.** The wall-wall seam on the
/// widened chart (`Chart`-described, spline carrier — the OTHER seam
/// of the same wall) is an interior column with no operand pair: the
/// exact class does not apply and no fitted statement exists, so the
/// arm must refuse TYPED — not mint the measured column, which is
/// exactly what `an_interior_column_still_refuses` forbids the
/// certification lane downstream.
#[test]
fn r1_wall_seam_arm_still_refuses_the_interior_column() {
    let (mut body, seam_he, key) = widened_fixture(false, 1);
    // Same stored-cache trap as the rim row: detach, then derive.
    let hes: Vec<_> = body
        .edges()
        .flat_map(|(_, e)| [e.he_plus, e.he_minus])
        .filter(|he| he_surface(&body, *he) == key)
        .collect();
    for he in hes {
        body.detach_pcurve(he);
    }
    let mut seams = 0;
    for (_, edge) in body.edges() {
        for he in [edge.he_plus, edge.he_minus] {
            if he_surface(&body, he) != key || he == seam_he {
                continue;
            }
            let (carrier, _, _) = carrier_of(&body, he);
            if !matches!(carrier, Curve3::Nurbs(_)) {
                continue;
            }
            seams += 1;
            let out = topo::pcurve_of(&body, he, band());
            let Err(PcurveMintError::Certify {
                error: geom_brep::PcurveCertifyError::IsoUnsupported { what },
                ..
            }) = out
            else {
                panic!("a Chart-described interior column refuses typed, never mints: {out:?}")
            };
            assert!(
                what.contains("neither chart boundary"),
                "and names the boundary assumption: {what}"
            );
        }
    }
    assert!(
        seams >= 1,
        "the wall face has its other seam: found {seams}"
    );
}

/// **The lane bound is signature churn, not capability loss.** The
/// statically-refusing `Dual` impl must leave every mint entry point
/// nameable at `Dual64` — this row is a COMPILE fact, pinned as code
/// so a future where-clause change reds it.
#[test]
fn r1_dual_scalar_still_reaches_the_mint() {
    let _mint: fn(&mut Body<geom_core::Dual64>, Tol) -> Result<(), PcurveMintError> =
        topo::mint_pcurves;
    type PcurveOf = fn(
        &Body<geom_core::Dual64>,
        topo::HalfEdgeKey,
        Band,
    ) -> Result<Pcurve<geom_core::Dual64>, PcurveMintError>;
    let _of: PcurveOf = topo::pcurve_of;
}

/// **Item 7's "third excluded case" — and the spec's ruling on it was
/// WRONG.** A chart whose `v` domain extends past the face: the wall's
/// quadratic restated as one Bezier on `v ∈ [0, 2]` (an exact
/// polynomial extension, weights all 1, so the surface is bit-for-bit
/// unchanged on the face's own `v ∈ [0, 1]`). The seam is still a
/// genuine BOUNDARY column in `u`, but its `v` map is a PARTIAL affine
/// restatement — the fixed four-candidate schedule, which assumes full
/// traversal, cannot find it.
///
/// The spec said such a locus "takes the exact class", a phrase lifted
/// from the refusal payload without checking whether the exact class
/// can certify it. **It cannot**: the seam class's hull limb compares
/// the image against the chart's own boundary ROW, which needs ONE
/// spline space, and a partial column is not a control-net copy of that
/// row and cannot be made into one. Minting the exact class here would
/// hand the certifier an image it must structurally refuse — the same
/// defect the wall–wall seam arm was reverted for. So the corrected
/// contract, and what this row pins: a partial restatement takes
/// `General`, and `General` CERTIFIES it.
///
/// This row was contributed RED against the pre-fix arm, which did
/// re-offer the exact class. It keeps its teeth in both directions: it
/// fails if the arm hands back an exact `IsoLine` again, and it fails
/// if `General` stops certifying the locus.
#[test]
fn r1_a_partial_column_restatement_takes_general_and_certifies() {
    let (mut body, he, bowed) =
        intrinsic_seam_at(false, SCALE).expect("the seam attaches at this scale");
    let n = chart_of(&body, bowed);
    assert!(
        n.weights().iter().all(|w| *w == 1.0),
        "the polynomial extension needs the integral wall"
    );
    assert_eq!(n.knots_v().degree(), 2, "the loft's v degree");
    let (nu, nv) = n.control_counts();
    assert_eq!((nu, nv), (2, 3), "the loft wall's net");
    // The same quadratic on [0, 2]: Q0 = P0, Q1 = 2 P1 - P0,
    // Q2 = P0 - 4 P1 + 4 P2 (endpoint/derivative matching at v = 0).
    let kv = KnotVector::clamped(vec![0.0, 0.0, 0.0, 2.0, 2.0, 2.0], 2).unwrap();
    let mut control = Vec::new();
    for iu in 0..2 {
        let p = |iv: usize| n.control()[iu * 3 + iv];
        let (p0, p1, p2) = (p(0), p(1), p(2));
        control.push(p0);
        control.push(p1 + (p1 - p0));
        control.push(p2 + (p2 - p1) * 3.0 + (p0 - p1));
    }
    let extended = Surface::Nurbs(Arc::new(
        NurbsSurface::new(n.knots_u().clone(), kv, control, vec![1.0; 6]).unwrap(),
    ));
    // The plane operand, read before the rechart mints a new key.
    let plane_key = {
        let edge = body.get_edge(body.get_half_edge(he).unwrap().edge).unwrap();
        let Some(topo::CurveGeom::Certified(c)) = body.get_curve_geom(edge.curve) else {
            panic!("certified")
        };
        let geom_brep::EdgeDescription::Intersection { s1, s2, .. } = *c.description() else {
            panic!("intersection")
        };
        [s1, s2]
            .into_iter()
            .find(|k| matches!(body.get_surface(*k), Some(Surface::Plane { .. })))
            .expect("one operand is the plane")
    };
    let key = rechart(&mut body, bowed, extended);
    let chart = chart_of(&body, key);
    assert_eq!(chart.knots_v().domain(), (0.0, 2.0));
    let out = topo::pcurve_of(&body, he, band());
    let Ok(Pcurve::General(ref image)) = out else {
        panic!(
            "a PARTIAL restatement of a boundary column has no exact class that can \
             certify it, so it takes General: {out:?}"
        )
    };
    let (carrier, t0, t1) = carrier_of(&body, he);
    // It is still a column: u constant, on a genuine u boundary.
    let (du0, du1) = chart.knots_u().domain();
    let us: Vec<f64> = image.control().iter().map(|p| p.x).collect();
    for u in &us {
        assert!(
            (u - us[0]).abs() < 1e-9,
            "a column holds u constant: {us:?}"
        );
    }
    assert!(
        (us[0] - du0).abs() < 1e-9 || (us[0] - du1).abs() < 1e-9,
        "on a genuine u boundary: {} of [{du0}, {du1}]",
        us[0]
    );
    // And the v map is the MEASURED partial one — the carrier's own
    // [0, 1] of the chart's [0, 2].
    let (va, vb) = (image.eval(t0).y, image.eval(t1).y);
    for v in [va, vb] {
        assert!(
            (-1e-9..=1.0 + 1e-9).contains(&v),
            "the measured map stays on the face's own v range [0, 1]: {v}"
        );
    }
    assert!(
        ((vb - va).abs() - 1.0).abs() < 1e-9,
        "and traverses exactly that range: |{vb} - {va}|"
    );
    // In metres, along the span.
    for i in 0..=8 {
        let t = t0 + (t1 - t0) * (f64::from(i) / 8.0);
        let uv = image.eval(t);
        let gap = carrier.eval(t).distance(chart.eval(uv.x, uv.y));
        assert!(
            gap < 1e-10,
            "the measured image tracks the carrier mid-span: {gap:e} m at t = {t}"
        );
    }
    // ---- And General CERTIFIES it, which is the corrected claim. ----
    // The description is re-stated against the chart the face now
    // carries, so the mate is the one `mate_surface` would find rather
    // than a hand-picked one (`rechart` mints a new surface key).
    body.set_edge_curve_nurbs_lane(
        body.get_half_edge(he).unwrap().edge,
        EdgeCurveSpec {
            description: EdgeDescriptionSpec::Intersection {
                s1: plane_key,
                s2: key,
                witness: carrier.eval((t0 + t1) * 0.5),
            },
            carrier: carrier.clone(),
            param_start: t0,
            param_end: t1,
        },
        Tol::witness(),
    )
    .expect("the seam re-attaches against the extended chart");
    let mate = {
        let edge = body.get_edge(body.get_half_edge(he).unwrap().edge).unwrap();
        let Some(topo::CurveGeom::Certified(c)) = body.get_curve_geom(edge.curve) else {
            panic!("certified")
        };
        let geom_brep::EdgeDescription::Intersection { s1, s2, .. } = *c.description() else {
            panic!("intersection")
        };
        let other = if key == s1 {
            s2
        } else if key == s2 {
            s1
        } else {
            panic!("mate_surface's precondition: the face's own surface is in the pair")
        };
        body.get_surface(other).cloned().expect("the mate resolves")
    };
    let eps = Tol::witness().get().eps;
    let window = out.as_ref().unwrap().chart_box(t0, t1);
    let cache = geom_brep::PcurveCache::certify_general(
        std::sync::Arc::clone(image),
        t0,
        t1,
        &carrier,
        &Surface::Nurbs(Arc::new(chart.clone())),
        Some(&mate),
        window,
        band(),
    )
    .expect("General certifies a partial column against its operand pair");
    let cert = cache.certificate();
    assert!(
        cert.envelope <= eps,
        "its between-samples bound is inside eps: {:e} vs {eps:e}",
        cert.envelope
    );
    assert!(cert.ssi.is_some(), "the FULL C2 certificate: {cert:?}");
    println!(
        "R1: a partial column takes General and certifies, envelope {:e} m",
        cert.envelope
    );
}

/// **The cap-rim widening's CERTIFICATION, which nothing else covers.**
///
/// `r1_cap_rim_widening_measures_the_face_not_the_chart` proves the
/// measured map is geometrically right; it stops at the derivation. The
/// whole-body mint cannot carry the check the rest of the way, because
/// the loop walk refuses at the OTHER seam (`16v1`, an interior column
/// through a `Chart` description — issue #1195) before any face-level
/// certification runs. So the widened rim images ship with a measured
/// derivation and, without this row, ZERO certification coverage.
///
/// Forced here at the door the mint would use: the closed-form
/// `PcurveCache::certify`, each rim against its own chart box. A rim is
/// an exact `v`-constant boundary row, so the envelope is exactly `0`
/// — asserted as `0.0`, not as a tolerance, because anything else means
/// the image is no longer the control-net copy the class rests on.
#[test]
fn r1_cap_rim_measured_map_certifies() {
    let (mut body, seam_he, key) = widened_fixture(false, 1);
    let all: Vec<_> = body
        .edges()
        .flat_map(|(_, e)| [e.he_plus, e.he_minus])
        .filter(|he| he_surface(&body, *he) == key)
        .collect();
    for he in &all {
        body.detach_pcurve(*he);
    }
    let chart = chart_of(&body, key);
    let surface = Surface::Nurbs(Arc::new(chart.clone()));
    let mut certified = 0;
    for he in all {
        if he == seam_he {
            continue;
        }
        let (carrier, t0, t1) = carrier_of(&body, he);
        if !matches!(carrier, Curve3::Line { .. }) {
            continue;
        }
        let Ok(pcurve) = topo::pcurve_of(&body, he, band()) else {
            panic!("the rim derives")
        };
        let window = pcurve.chart_box(t0, t1);
        let cache = geom_brep::PcurveCache::certify(
            pcurve.clone(),
            t0,
            t1,
            &carrier,
            &surface,
            window,
            band(),
        )
        .unwrap_or_else(|e| {
            panic!("the measured rim map certifies at the closed-form door: {e:?}")
        });
        let cert = cache.certificate();
        assert_eq!(
            cert.envelope, 0.0,
            "an exact boundary row's envelope is EXACTLY zero, not merely small: \
             {:e} (image {pcurve:?})",
            cert.envelope
        );
        assert_eq!(
            cert.max_residual, 0.0,
            "and so is its sampled max: {:e}",
            cert.max_residual
        );
        certified += 1;
    }
    assert_eq!(
        certified, 2,
        "both cap rims of the widened face were certified, not zero of them"
    );
    println!("R1: both widened cap rims certify at envelope 0e0");
}
