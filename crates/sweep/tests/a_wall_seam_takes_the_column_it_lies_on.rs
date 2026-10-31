//! **A wall–wall seam on a spline chart takes the column it lies on**,
//! whichever way the chart's `v` runs against its neighbour's.
//!
//! The fixture is a loft of a profile with one semicircular edge: four
//! spline walls whose vertical seams are each described as an `IsoLine`
//! image on the NEIGHBOURING wall's chart. Re-charting the arc wall onto
//! the same surface reversed in `v` (and, for the second row, in `u`
//! too) leaves every seam on it running against the neighbour's `v`
//! map: the column that carries the stated map verbatim does not hold
//! the carrier, and the one that runs the other way does.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::{NurbsSurface, Surface};
use geom_brep::EdgeDescriptionSpec;
use geom_core::{Point2, Point3, Tol, Vec3};
use profile::test_support::bulge_loop;
use std::sync::Arc;
use sweep::test_support::stacked_at;
use topo::construction::Construct;
use topo::construction::FaceSurface;
use topo::{Body, FaceKey, HalfEdgeKey, Pcurve};

fn tol() -> Tol {
    Tol::witness()
}

/// A prism whose profile's second edge is a semicircle (bulge 1): its
/// wall's knots mirror bitwise, so `reversed_u` and `reversed_v` both
/// accept it.
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

/// The arc wall: the face whose rows include the `IsoArc` rims.
fn arc_wall(body: &Body<f64>) -> FaceKey {
    let rim = body
        .pcurves()
        .find(|(_, c)| matches!(c.pcurve(), Pcurve::IsoArc { .. }))
        .map(|(h, _)| h)
        .expect("the prism has an arc rim");
    body.face_of_half_edge(rim).unwrap()
}

/// The wall's half-edges.
fn wall_half_edges(body: &Body<f64>, wall: FaceKey) -> Vec<HalfEdgeKey> {
    body.half_edges()
        .map(|(k, _)| k)
        .filter(|&k| body.face_of_half_edge(k) == Some(wall))
        .collect()
}

/// `body` with `wall` re-charted in place onto `remap(its surface)`,
/// stating `sense`.
fn recharted(
    body: &Body<f64>,
    wall: FaceKey,
    remap: impl Fn(&NurbsSurface<f64>) -> NurbsSurface<f64>,
    flip_sense: bool,
) -> Body<f64> {
    let mut out = body.clone();
    let face = out.get_face(wall).unwrap();
    let (key, sense) = (face.surface, face.sense);
    let Some(Surface::Nurbs(net)) = out.get_surface(key).cloned() else {
        panic!("the arc wall is a spline chart")
    };
    // Lifts RechartUnvouched and RechartStrandsDescriptions: the wall
    // re-charted in place onto the same surface reparameterized, its
    // edges' descriptions left as they were, is the row's premise.
    out.set_face_surface_unvouched_for_tests(
        wall,
        FaceSurface::New {
            surface: Surface::Nurbs(std::sync::Arc::new(remap(&net))),
            sense: sense != flip_sense,
        },
    )
    .unwrap();
    out
}

/// One stored wall row: its kind, its `p0`, and its moving channel.
type Row = (&'static str, (f64, f64), (f64, f64));

/// The minted wall's rows, sorted: the two seams are columns and the
/// two rims are rows.
fn stored_rows(body: &Body<f64>, wall: FaceKey) -> Vec<Row> {
    let mut rows: Vec<_> = wall_half_edges(body, wall)
        .into_iter()
        .map(|h| {
            match body
                .pcurve(h)
                .unwrap_or_else(|| panic!("{h:?}: the mint stores the wall's row"))
                .pcurve()
            {
                Pcurve::IsoLine { p0, pl } => ("column", (p0.x, p0.y), (pl.x, pl.y)),
                Pcurve::IsoArc { p0, pd, .. } => ("rim", (p0.x, p0.y), (pd.x, pd.y)),
                other => panic!("{h:?}: the wall's rows are exact iso images: {other:?}"),
            }
        })
        .collect();
    rows.sort_by(|a, b| a.partial_cmp(b).unwrap());
    rows
}

/// **A wall reversed in `v`: each seam takes the column that runs
/// against the neighbour's `v`.** The mint stores every wall row: the
/// two seams on columns `u = 0` and `u = 1` running from `v = 1` down
/// to `v = 0`, and the two rims each on the other row, running with
/// `u`. The native wall is the control, with every row forward.
#[test]
fn a_wall_reversed_in_v_takes_its_seams_reversed() {
    let body = semicircle_prism();
    let wall = arc_wall(&body);
    let mut native = body.clone();
    assert_eq!(
        topo::mint_pcurves(&mut native, tol()),
        Ok(()),
        "the control mints"
    );
    let forward = stored_rows(&native, wall);
    assert_eq!(
        forward
            .iter()
            .filter(|r| r.0 == "column")
            .collect::<Vec<_>>(),
        [
            &("column", (0.0, 0.0), (0.0, 1.0)),
            &("column", (1.0, 0.0), (0.0, 1.0))
        ],
        "the control: each seam lies on its boundary column, running with v"
    );

    let mut reversed = recharted(&body, wall, |n| n.reversed_v().unwrap(), true);
    assert_eq!(
        topo::mint_pcurves(&mut reversed, tol()),
        Ok(()),
        "the mint stores the reversed wall's rows"
    );
    let got = stored_rows(&reversed, wall);
    let columns: Vec<_> = got.iter().filter(|r| r.0 == "column").collect();
    assert_eq!(
        columns,
        [
            &("column", (0.0, 1.0), (0.0, -1.0)),
            &("column", (1.0, 1.0), (0.0, -1.0))
        ],
        "each seam runs down its column, against the neighbour's v"
    );
    let rims: Vec<_> = got
        .iter()
        .filter(|r| r.0 == "rim")
        .map(|r| (r.1, r.2))
        .collect();
    let mut rims_forward: Vec<_> = forward
        .iter()
        .filter(|r| r.0 == "rim")
        .map(|r| ((r.1.0, 1.0 - r.1.1), r.2))
        .collect();
    rims_forward.sort_by(|a, b| a.partial_cmp(b).unwrap());
    assert_eq!(
        rims, rims_forward,
        "each rim moves to the other row, still with u"
    );
}

/// **A wall reversed in `u` and `v`: the seams swap columns and run
/// reversed.** Every row of the wall is the native one mirrored in both
/// channels, and the mint stores them all.
#[test]
fn a_wall_reversed_in_u_and_v_takes_its_seams_reversed() {
    let body = semicircle_prism();
    let wall = arc_wall(&body);
    let mut reversed = recharted(
        &body,
        wall,
        |n| n.reversed_u().unwrap().reversed_v().unwrap(),
        false,
    );
    assert_eq!(
        topo::mint_pcurves(&mut reversed, tol()),
        Ok(()),
        "the mint stores the twice-reversed wall's rows"
    );
    let got = stored_rows(&reversed, wall);
    assert_eq!(
        got,
        [
            ("column", (0.0, 1.0), (0.0, -1.0)),
            ("column", (1.0, 1.0), (0.0, -1.0)),
            ("rim", (1.0, 0.0), (-1.0, 0.0)),
            ("rim", (1.0, 1.0), (-1.0, 0.0)),
        ],
        "every row is the native one mirrored in u and v"
    );
}

/// The band's interior: a distance the probe can neither confirm nor
/// rule out (between ε and K·ε).
fn in_band() -> f64 {
    tol().eps() * tol().k().sqrt()
}

fn band() -> geom_core::Band {
    geom_core::Band::linear(tol()).unwrap()
}

/// What `nurbs_iso_derive` answers for `h`, the stored row detached:
/// `pcurve_of` answers a cached image first.
fn derived(body: &Body<f64>, h: HalfEdgeKey) -> Result<Pcurve<f64>, topo::PcurveMintError> {
    let mut fresh = body.clone();
    fresh.detach_pcurve(h);
    topo::pcurve_of(&fresh, h, band())
}

/// The wall's two seams: the half-edges the native mint stores as a
/// column (`u` held).
fn seams(body: &Body<f64>, wall: FaceKey) -> Vec<HalfEdgeKey> {
    let mut native = body.clone();
    topo::mint_pcurves(&mut native, tol()).expect("the native prism mints");
    wall_half_edges(&native, wall)
        .into_iter()
        .filter(|&h| {
            matches!(
                native.pcurve(h).map(|c| c.pcurve().clone()),
                Some(Pcurve::IsoLine { pl, .. }) if pl.x == 0.0
            )
        })
        .collect()
}

/// **A seam whose every candidate escalates answers the escalation.**
/// The arc wall re-charted onto its surface moved `in_band()` along
/// `y`, which is the wall's normal at both seams: each seam's stated
/// column, each closed-form column and the column the foot names all
/// sit inside the band at the probe. The derivation answers
/// `Escalated`, and does not go on to the seam's `General` image, which
/// would chart a carrier the probe could not place.
#[test]
fn a_seam_in_the_band_answers_the_escalation() {
    let body = semicircle_prism();
    let wall = arc_wall(&body);
    let lift = Vec3::new(0.0, in_band(), 0.0);
    let moved = recharted(&body, wall, |n| n.map_points(|p| p + lift), false);
    let seams = seams(&body, wall);
    assert_eq!(seams.len(), 2, "the arc wall has two seams");
    for h in seams {
        let got = derived(&moved, h);
        assert!(
            matches!(got, Err(topo::PcurveMintError::Escalated { .. })),
            "{h:?}: a seam in the band escalates: {got:?}"
        );
    }
}

/// **An escalated candidate does not block a later one that holds.** A
/// flat wall re-charted onto its plane stretched `in_band()` past each
/// seam: the stated column at each chart boundary sits inside the band
/// at the probe, and the column the start's foot names holds exactly.
/// Each seam takes that interior column. (Its certification, a band's
/// width from the chart boundary, is the seam class's own question and
/// escalates there; this row is the pick's.)
#[test]
fn a_seam_inside_a_chart_a_hair_wider_takes_its_interior_column() {
    let body = semicircle_prism();
    let arc = arc_wall(&body);
    let (flat, chart) = body
        .faces()
        .filter(|(f, _)| *f != arc)
        .find_map(|(f, face)| match body.get_surface(face.surface) {
            Some(Surface::Nurbs(n)) if n.control().iter().all(|p| p.y == 0.0) => {
                Some((f, Arc::clone(n)))
            }
            _ => None,
        })
        .expect("the prism has a flat wall on y = 0");
    let xs: Vec<f64> = chart.control().iter().map(|p| p.x).collect();
    let (x0, x1) = xs
        .iter()
        .fold((f64::INFINITY, f64::NEG_INFINITY), |(a, b), &x| {
            (a.min(x), b.max(x))
        });
    let (centre, half) = (0.5 * (x0 + x1), 0.5 * (x1 - x0));
    let stretch = (half + in_band()) / half;
    let wider = recharted(
        &body,
        flat,
        |n| n.map_points(|p| Point3::new(centre + (p.x - centre) * stretch, p.y, p.z)),
        false,
    );
    let seams = seams(&body, flat);
    assert_eq!(seams.len(), 2, "the flat wall has two seams");
    let inset = in_band() / (2.0 * (half + in_band()));
    for h in seams {
        let image = derived(&wider, h).unwrap();
        let Pcurve::IsoLine { p0, pl } = image else {
            panic!("{h:?}: a straight seam takes an iso line: {image:?}")
        };
        assert_eq!(pl.x, 0.0, "{h:?}: a column holds u: {pl:?}");
        let off = p0.x.min(1.0 - p0.x);
        assert!(
            (off - inset).abs() < 1e-12,
            "{h:?}: the column the foot names, {inset:e} inside the boundary: {}",
            p0.x
        );
    }
}

/// **A circle stated as an iso LINE on a spline chart is refused at
/// the door.** The wall-seam arm hands any carrier its stated image
/// does not place to the boundary-iso schedule, whose only exact class
/// for a circle is the arc rim; no row reaches that path, because no
/// door certifies a circle as an `IsoLine` image of a spline chart (the
/// chart's own rim is an `IsoArc`, on its rational parameter). If a
/// door ever does, this goes red and the path is owed a row.
#[test]
fn a_circle_stated_as_an_iso_line_on_a_spline_chart_is_refused_at_the_door() {
    let mut body = semicircle_prism();
    let wall = arc_wall(&body);
    let wall_key = body.get_face(wall).unwrap().surface;
    let (edge, geom) = body
        .edges()
        .find_map(|(e, edge)| {
            let curve = body
                .get_curve_geom(edge.curve)
                .and_then(topo::CurveGeom::certified)?;
            matches!(curve.carrier(), geom::Curve3::Circle { .. }).then(|| (e, curve.clone()))
        })
        .expect("the prism has an arc rim");
    let (t0, t1) = geom.params();
    let slope = 1.0 / (t1 - t0);
    let mut spec = geom.restated_spec();
    spec.description = EdgeDescriptionSpec::Chart {
        surface: wall_key,
        image: Some(Pcurve::IsoLine {
            p0: Point2::new(-slope * t0, 0.0),
            pl: geom_core::Vec2::new(slope, 0.0),
        }),
        wrap: false,
        declared: None,
    };
    let before = format!("{body:?}");
    let refused = body.set_edge_curve(edge, spec, tol());
    assert!(
        matches!(refused, Err(topo::EulerOpError::Certification { .. })),
        "a circle is no iso line of a rational chart: {refused:?}"
    );
    assert_eq!(format!("{body:?}"), before, "the body is untouched");
}

/// **Without a foot, a sub-seam takes its stated column.** At `Dual64`
/// no lane measures a chart foot, so the only column a seam's own
/// candidates offer is the stated one at each chart boundary. Splitting
/// a seam leaves two sub-edges whose stated `v` map is the parent's
/// restricted, which the whole-domain closed forms do not match. Each
/// child, on each of its two walls, derives its boundary column with
/// that map; without the stated candidates it would fall to `General`,
/// which a dual refuses.
#[test]
fn at_a_dual_a_split_seam_takes_its_stated_column() {
    use geom_core::{Bounds, Dual64, Real};
    let v = |x: f64, y: f64, bulge: f64| (Point2::new(x, y), bulge);
    let profile = || {
        vec![bulge_loop(vec![
            v(0.0, 0.0, 0.0),
            v(2.0, 0.0, 1.0),
            v(2.0, 2.0, 0.0),
            v(0.0, 2.0, 0.0),
        ])]
    };
    let mut body = sweep::loft_body::<Dual64>(
        &sweep::test_support::lift_sections::<Dual64>(&[profile(), profile()]),
        &sweep::test_support::lift_places::<Dual64>(
            &stacked_at(&[0.0, 1.0]),
            geom_core::Tol::witness(),
        ),
        1,
        tol(),
    )
    .expect("the dual prism builds")
    .body;
    let seam = body
        .pcurves()
        .find(|(_, c)| matches!(c.pcurve(), Pcurve::IsoLine { pl, .. } if pl.x.lo() == 0.0))
        .map(|(h, _)| body.get_half_edge(h).unwrap().edge)
        .expect("the prism has a seam on a column");
    let (t0, t1) = body
        .get_curve_geom(body.get_edge(seam).unwrap().curve)
        .and_then(topo::CurveGeom::certified)
        .unwrap()
        .params();
    let made = body
        .split_edge(seam, (t0 + t1) * Dual64::from_f64(0.5), tol())
        .expect("the seam splits");
    let band = geom_core::Band::linear(tol()).unwrap();
    let halves = [seam, made.new_edge].into_iter().flat_map(|e| {
        let edge = body.get_edge(e).unwrap();
        [edge.he_plus, edge.he_minus]
    });
    for h in halves {
        let mut fresh = body.clone();
        fresh.detach_pcurve(h);
        let got = topo::pcurve_of(&fresh, h, band);
        let Ok(Pcurve::IsoLine { p0, pl }) = got else {
            panic!("{h:?}: a sub-seam takes its column: {got:?}")
        };
        assert!(
            pl.x.lo() == 0.0 && pl.y.lo() != 0.0 && [0.0, 1.0].contains(&p0.x.lo()),
            "{h:?}: on a boundary column, v moving: {p0:?} {pl:?}"
        );
    }
}

/// **A stated column is offered before the closed forms.** Split a
/// seam at `t = 1/6` and `t = 1/2`: the middle child's stated map is
/// the parent's, `v = t`, and the whole-domain closed form for it is
/// `v = 3(t − 1/6)`. The two are different lines through the same
/// point at the probe (`t = 1/4`, `v = 1/4`), so both hold there, and
/// the order decides. Each of the child's halves derives the stated
/// map, which certifies; the closed form does not.
#[test]
fn a_stated_column_is_offered_before_the_closed_form_it_crosses() {
    let mut body = semicircle_prism();
    let seam = body
        .pcurves()
        .find(|(_, c)| matches!(c.pcurve(), Pcurve::IsoLine { pl, .. } if pl.x == 0.0))
        .map(|(h, _)| body.get_half_edge(h).unwrap().edge)
        .expect("the prism has a seam on a column");
    let params = |b: &Body<f64>, e| {
        b.get_curve_geom(b.get_edge(e).unwrap().curve)
            .and_then(topo::CurveGeom::certified)
            .unwrap()
            .params()
    };
    assert_eq!(params(&body, seam), (0.0, 1.0), "the seam runs t = 0..1");
    let upper = body.split_edge(seam, 1.0 / 6.0, tol()).unwrap().new_edge;
    let middle = [upper, body.split_edge(upper, 0.5, tol()).unwrap().new_edge]
        .into_iter()
        .find(|&e| params(&body, e) == (1.0 / 6.0, 0.5))
        .expect("the middle child spans t = 1/6 .. 1/2");
    let (t0, t1) = params(&body, middle);
    let edge = body.get_edge(middle).unwrap();
    for h in [edge.he_plus, edge.he_minus] {
        let face = body.face_of_half_edge(h).unwrap();
        let chart = body
            .get_surface(body.get_face(face).unwrap().surface)
            .unwrap()
            .clone();
        let image = derived(&body, h).unwrap();
        let Pcurve::IsoLine { p0, pl } = image else {
            panic!("{h:?}: a sub-seam takes an iso line: {image:?}")
        };
        assert_eq!(
            (pl.x, pl.y, p0.y),
            (0.0, 1.0, 0.0),
            "{h:?}: the stated map v = t, not the closed form through the same probe point"
        );
        let curve = body
            .get_curve_geom(edge.curve)
            .and_then(topo::CurveGeom::certified)
            .unwrap();
        geom_brep::PcurveCache::certify(image, t0, t1, curve.carrier(), &chart, band())
            .unwrap_or_else(|e| panic!("{h:?}: the stated column certifies: {e:?}"));
    }
}

/// The prism's flat wall on `y = 0`, re-charted onto the same plane
/// with its `v` knots moved `in_band()`: the chart now answers at
/// `v + δ` what it answered at `v`, so every seam's stated `v` map (the
/// old chart's) misses its carrier by `δ` metres, inside the band,
/// while the chart's own whole-domain columns are exact.
fn flat_wall_shifted_in_v(body: &Body<f64>) -> (Body<f64>, FaceKey) {
    let arc = arc_wall(body);
    let flat = body
        .faces()
        .filter(|(f, _)| *f != arc)
        .find_map(|(f, face)| match body.get_surface(face.surface) {
            Some(Surface::Nurbs(n)) if n.control().iter().all(|p| p.y == 0.0) => Some(f),
            _ => None,
        })
        .expect("the prism has a flat wall on y = 0");
    let shifted = recharted(
        body,
        flat,
        |n| {
            let kv = n.knots_v_f64_interim();
            let moved: Vec<f64> = kv.knots().iter().map(|k| k + in_band()).collect();
            NurbsSurface::new(
                n.knots_u_f64_interim().clone(),
                geom_core::spline::KnotVector::clamped(moved, kv.degree()).unwrap(),
                n.control().to_vec(),
                n.weights().to_vec(),
            )
            .unwrap()
        },
        false,
    );
    (shifted, flat)
}

/// **An escalated stated column does not outrank a closed form that
/// holds.** On the shifted flat wall each seam's stated columns (at the
/// boundary and at the foot) sit inside the band at the probe, and the
/// chart's own boundary column holds exactly. Each seam takes that
/// column, `v` running over the shifted domain.
#[test]
fn an_escalated_stated_column_gives_way_to_a_closed_form_that_holds() {
    let body = semicircle_prism();
    let (shifted, flat) = flat_wall_shifted_in_v(&body);
    for h in seams(&body, flat) {
        let got = derived(&shifted, h);
        let Ok(Pcurve::IsoLine { p0, pl }) = got else {
            panic!("{h:?}: the closed-form column holds: {got:?}")
        };
        assert!(
            pl.x == 0.0 && (pl.y - 1.0).abs() < 1e-12 && p0.y == in_band(),
            "{h:?}: the chart's own column, v from δ: {p0:?} {pl:?}"
        );
    }
}

/// **An escalated stated column is answered when nothing holds.** Split
/// a seam of the shifted flat wall at its midpoint: each child's stated
/// columns still sit inside the band, and the whole-domain closed forms
/// are now definitely off (they would map half the seam onto all of
/// `v`). A spline seam is offered no measured column, so nothing holds:
/// the derivation answers the escalation, not the child's `General`
/// image.
#[test]
fn an_escalated_stated_column_is_answered_when_nothing_holds() {
    let body = semicircle_prism();
    let (mut shifted, flat) = flat_wall_shifted_in_v(&body);
    let seam = seams(&body, flat)[0];
    let edge = shifted.get_half_edge(seam).unwrap().edge;
    let made = shifted
        .split_edge(edge, 0.5, tol())
        .expect("the seam splits");
    for e in [edge, made.new_edge] {
        let child = shifted.get_edge(e).unwrap();
        let on_flat = [child.he_plus, child.he_minus]
            .into_iter()
            .find(|&h| shifted.face_of_half_edge(h) == Some(flat))
            .expect("each child has a half on the flat wall");
        let got = derived(&shifted, on_flat);
        assert!(
            matches!(got, Err(topo::PcurveMintError::Escalated { .. })),
            "{on_flat:?}: the in-band stated column is answered: {got:?}"
        );
    }
}
