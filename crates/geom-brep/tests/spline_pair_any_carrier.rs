//! **The spline-pair edge certificates take a carrier of any kind**
//! (C1 rung 3, C2): the plane × NURBS and NURBS × NURBS lanes read a
//! line, circle or ellipse through the carrier-enclosure door
//! (`geom_brep::carrier_net`) exactly as they read a spline, and a
//! closed form refuses where, and by what, its spline twin does.
//!
//! The verdicts are the lanes' own: a line's degree-1 chart image
//! certifies on a flat wall, and a circle's or an ellipse's refuses at
//! the hull wherever its degree-1 image's chord error passes the band,
//! with that error measured. Each row offers the same locus as a
//! spline carrier and asks the same verdict of it.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::sync::Arc;

use crate::shared::fixture::quarter_cylinder_wall;
use crate::shared::tol::{band, eps};
use geom::{Curve3, NurbsCurve3, NurbsSurface, Surface};
use geom_brep::keys::SurfaceKey;
use geom_brep::{
    CarrierEnclosureRefusal, CertifyError, EdgeCurve, EdgeCurveSpec, EdgeDescriptionSpec,
    NurbsLane, PlaneNurbsLimbs, PlaneNurbsRefusal, SsiLimb, SurfacePair, nurbs_pair_limbs,
    plane_nurbs_limbs,
};
use geom_core::spline::KnotVector;
use geom_core::test_support::upper;
use geom_core::{Point3, Vec3};
use slotmap::SlotMap;

type Limbs = Result<PlaneNurbsLimbs<f64>, PlaneNurbsRefusal>;

type IntervalLimbs = Result<PlaneNurbsLimbs<geom_core::interval::Interval>, PlaneNurbsRefusal>;

/// A bilinear wall through four corners, `control[iu * 2 + iv]`.
fn bilinear(corners: [Point3<f64>; 4]) -> NurbsSurface<f64> {
    let k = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
    NurbsSurface::new(k.clone(), k, corners.to_vec(), vec![1.0; 4]).unwrap()
}

/// The flat wall `z = 0` over `[0, 1]²`, its chart the identity.
fn flat_wall() -> NurbsSurface<f64> {
    let p = |x, y| Point3::new(x, y, 0.0);
    bilinear([p(0.0, 0.0), p(0.0, 1.0), p(1.0, 0.0), p(1.0, 1.0)])
}

/// The off-iso diagonal on the flat wall, `t ∈ [0, 1]`.
fn diagonal() -> Curve3<f64> {
    Curve3::Line {
        origin: Point3::new(0.1, 0.1, 0.0),
        dir: Vec3::new(0.6, 0.8, 0.0),
    }
}

/// The vertical plane through the diagonal.
fn diagonal_plane() -> Surface<f64> {
    Surface::Plane {
        origin: Point3::new(0.1, 0.1, 0.0),
        normal: Vec3::new(0.8, -0.6, 0.0),
        u_ref: Vec3::new(0.6, 0.8, 0.0),
    }
}

/// The vertical bilinear wall through the diagonal, reaching past its
/// ends.
fn diagonal_wall() -> NurbsSurface<f64> {
    let at = |t: f64, z| Point3::new(0.1 + 0.6 * t, 0.1 + 0.8 * t, z);
    bilinear([at(-0.1, -0.5), at(-0.1, 0.5), at(1.1, -0.5), at(1.1, 0.5)])
}

/// A spline twin of a closed form over `[t0, t1]`: a degree-1 segment
/// for a line, the exact rational quadratic arc for a circle or an
/// ellipse of half-angle under π/2, on the closed form's own interval.
fn twin(carrier: &Curve3<f64>, (t0, t1): (f64, f64)) -> Curve3<f64> {
    let net = match *carrier {
        Curve3::Line { .. } => NurbsCurve3::new(
            KnotVector::clamped(vec![t0, t0, t1, t1], 1).unwrap(),
            vec![carrier.eval(t0), carrier.eval(t1)],
            vec![1.0, 1.0],
        ),
        Curve3::Circle {
            center,
            axis,
            radius,
            u_ref,
        } => arc(center, u_ref * radius, axis.cross(u_ref) * radius, (t0, t1)),
        Curve3::Ellipse {
            center,
            axis,
            semi_u,
            semi_v,
            u_ref,
        } => arc(center, u_ref * semi_u, axis.cross(u_ref) * semi_v, (t0, t1)),
        _ => unreachable!("the rows twin a line, a circle or an ellipse"),
    }
    .unwrap();
    Curve3::Nurbs(Arc::new(net))
}

/// `c + u·cos θ + v·sin θ` over `[a, b]` as a rational quadratic.
fn arc(
    c: Point3<f64>,
    u: Vec3<f64>,
    v: Vec3<f64>,
    (a, b): (f64, f64),
) -> Result<NurbsCurve3<f64>, geom_core::spline::SplineError> {
    let (h, m) = (0.5 * (b - a), 0.5 * (a + b));
    let at = |t: f64| c + u * t.cos() + v * t.sin();
    let mid = c + (u * m.cos() + v * m.sin()) / h.cos();
    NurbsCurve3::new(
        KnotVector::clamped(vec![a, a, a, b, b, b], 2).unwrap(),
        vec![at(a), mid, at(b)],
        vec![1.0, h.cos(), 1.0],
    )
}

/// A height-`z` unit circle about the z axis, `t` its angle: on the
/// quarter-cylinder wall.
fn circle(z: f64) -> Curve3<f64> {
    Curve3::Circle {
        center: Point3::new(0.0, 0.0, z),
        axis: Vec3::new(0.0, 0.0, 1.0),
        radius: 1.0,
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    }
}

/// The plane `z = h`.
fn horizontal(h: f64) -> Surface<f64> {
    Surface::Plane {
        origin: Point3::new(0.0, 0.0, h),
        normal: Vec3::new(0.0, 0.0, 1.0),
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    }
}

/// The quarter-cylinder wall's section by the plane `z = 0.4 + 0.3·x`:
/// `(cos t, sin t, 0.4 + 0.3·cos t)`.
fn ellipse() -> (Curve3<f64>, Surface<f64>) {
    let k = 0.3_f64;
    let s = (1.0 + k * k).sqrt();
    let axis = Vec3::new(-k / s, 0.0, 1.0 / s);
    (
        Curve3::Ellipse {
            center: Point3::new(0.0, 0.0, 0.4),
            axis,
            semi_u: s,
            semi_v: 1.0,
            u_ref: Vec3::new(1.0 / s, 0.0, k / s),
        },
        Surface::Plane {
            origin: Point3::new(0.0, 0.0, 0.4),
            normal: axis,
            u_ref: Vec3::new(1.0 / s, 0.0, k / s),
        },
    )
}

/// A short arc of the circle from `0.7` rad whose degree-1 chart image
/// stays inside the band at every ε: `scale·√(ε/10⁻⁹)` rad, so the
/// chord error, quadratic in the arc, scales with ε.
fn short(scale: f64) -> (f64, f64) {
    (0.7, 0.7 + scale * (eps() / 1e-9).sqrt().min(20.0))
}

/// How far a refusing carrier itself sits off the wall: its largest
/// foot distance over `params`, at 257 samples. The refusal's limb-2
/// break is one of the span's points, so a carrier this close to the
/// wall everywhere is on it at the break, and the miss limb 2 reads is
/// the certificate's own image's (`SsiLimb::ImageMiss`).
fn carrier_foot_sup<T: geom_core::CertifiedBounds + geom_core::SpanLocate>(
    carrier: &Curve3<T>,
    (t0, t1): (f64, f64),
    wall: &NurbsSurface<T>,
) -> f64 {
    (0..=256)
        .map(|i| {
            let t = t0 + (t1 - t0) * f64::from(i) / 256.0;
            let foot = wall
                .project(carrier.eval(T::from_f64(t)))
                .unwrap_or_else(|e| panic!("the carrier's foot at {t}: {e:?}"));
            foot.distance.hi()
        })
        .fold(0.0, f64::max)
}

/// The largest foot distance a refusing row's carrier may read over
/// its span. These carriers are exact on their wall: the 1.3 rad circle
/// and ellipse read 8.1e-14 m at most (f64; the circle 8.2e-14 m at
/// `Interval`), and the circle 1.1e-16 m at the refusing break. A
/// carrier moved a picometre off the wall breaks the rows that read it.
const ON_THE_WALL: f64 = 1e-12;

/// The verdict a row compares between a closed form and its twin:
/// certified, or refused at a limb.
#[derive(Debug, PartialEq)]
enum Verdict {
    Certified,
    Refused(SsiLimb),
}

/// The verdict, with the bound it was reached on: a limb's definite
/// refusal or its undecided one alike.
fn verdict(name: &str, got: &Limbs) -> (Verdict, f64) {
    match got {
        Ok(l) => (Verdict::Certified, l.hull_sup),
        Err(PlaneNurbsRefusal::Limb { limb, margin }) => (Verdict::Refused(*limb), upper(*margin)),
        // A circle no boundary row holds reads its fitted image, and its
        // limb's refusal carries that no exact image held.
        Err(PlaneNurbsRefusal::ArcRim(geom_brep::ArcRimRefusal::Refused {
            iso: None,
            limb,
            bound,
        })) => (Verdict::Refused(*limb), upper(*bound)),
        // A miss inside the band refuses undecided, on the bound it read.
        Err(PlaneNurbsRefusal::Escalated { limb, cause }) => {
            (Verdict::Refused(*limb), upper(cause.margin))
        }
        Err(other) => panic!("{name}: certifies or refuses at a limb with its bound: {other:?}"),
    }
}

/// A plane × spline row: a carrier, its interval, the plane and the
/// wall, and the verdict the carrier and its spline twin both reach.
type PlaneRow = (
    &'static str,
    Curve3<f64>,
    (f64, f64),
    Surface<f64>,
    NurbsSurface<f64>,
    Verdict,
);

/// Each row's closed form and its spline twin reach `want`: a
/// certified bound within ε, or a limb-2 refusal on a measured miss
/// past ε. The twin is the same LOCUS, not the same parameterised
/// curve (a rational arc is not the angle's), so its image and bound
/// differ and only the verdict is compared.
fn plane_rows_meet_their_twins(rows: Vec<PlaneRow>) {
    for (name, carrier, params, plane, wall, want) in rows {
        let got = plane_nurbs_limbs(&carrier, params, &plane, &wall, 1.0, band());
        let (closed, bound) = verdict(name, &got);
        let spline = twin(&carrier, params);
        let (twinned, twin_bound) = verdict(
            name,
            &plane_nurbs_limbs(&spline, params, &plane, &wall, 1.0, band()),
        );
        println!("{name}: {closed:?} on {bound:e} m; its twin {twinned:?} on {twin_bound:e} m");
        for (who, v, b) in [
            ("closed form", &closed, bound),
            ("twin", &twinned, twin_bound),
        ] {
            assert_eq!(
                std::mem::discriminant(v),
                std::mem::discriminant(&want),
                "{name}: the {who}'s verdict: {v:?}"
            );
            match v {
                Verdict::Certified => assert!(b <= eps(), "{name}: the {who} within ε: {b:e}"),
                Verdict::Refused(limb) => assert!(
                    matches!(limb, SsiLimb::HullSupChart | SsiLimb::ImageMiss)
                        && (eps()..1e-3).contains(&b),
                    "{name}: the {who} refused at limb 2 on a miss past ε: {limb:?}, {b:e}"
                ),
            }
        }
    }
}

/// **Plane × spline, a certifying line.** The off-iso diagonal on the
/// flat wall certifies, hull bound within ε, as its degree-1 twin does.
#[test]
fn a_line_on_a_plane_by_spline_pair_certifies_with_its_twin() {
    plane_rows_meet_their_twins(vec![(
        "line",
        diagonal(),
        (0.0, 1.0),
        diagonal_plane(),
        flat_wall(),
        Verdict::Certified,
    )]);
}

/// **Plane × spline, a certifying circle.** A circle over a short arc
/// ([`short`]) of the quarter-cylinder wall certifies, its enclosing
/// net and its degree-1 image within ε, as its rational twin does.
#[test]
fn a_short_circle_on_a_plane_by_spline_pair_certifies_with_its_twin() {
    plane_rows_meet_their_twins(vec![(
        "circle, short",
        circle(0.5),
        short(0.002),
        horizontal(0.5),
        quarter_cylinder_wall(),
        Verdict::Certified,
    )]);
}

/// **Plane × spline, the refusing closed forms.** A circle and an
/// ellipse over 1.3 rad of the quarter-cylinder wall refuse at limb 2
/// on the degree-1 image's chord error, measured, as their twins do.
#[test]
fn a_closed_form_carrier_on_a_plane_by_spline_pair_refuses_with_its_twin() {
    let (ellipse, tilted) = ellipse();
    for (name, carrier) in [("circle", circle(0.5)), ("ellipse", ellipse.clone())] {
        let foot = carrier_foot_sup(&carrier, (0.1, 1.4), &quarter_cylinder_wall());
        println!("{name}, 1.3 rad: the carrier's own foot reads {foot:e} m at most");
        assert!(
            foot <= ON_THE_WALL,
            "{name}: the carrier is on the wall: {foot:e} m"
        );
    }
    plane_rows_meet_their_twins(vec![
        (
            "circle, 1.3 rad",
            circle(0.5),
            (0.1, 1.4),
            horizontal(0.5),
            quarter_cylinder_wall(),
            Verdict::Refused(SsiLimb::HullSup),
        ),
        (
            "ellipse, 1.3 rad",
            ellipse,
            (0.1, 1.4),
            tilted,
            quarter_cylinder_wall(),
            Verdict::Refused(SsiLimb::HullSup),
        ),
    ]);
}

/// A wall on `[0, 1]²` whose height along `u` is `z(u)`, flat between
/// `u = 0` and `u = knee` and rising linearly by `rise` to `u = 1`;
/// linear in `v`.
fn kneed_wall(knee: f64, rise: f64) -> NurbsSurface<f64> {
    let ku = KnotVector::clamped(vec![0.0, 0.0, knee, 1.0, 1.0], 1).unwrap();
    let kv = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
    let control = [(0.0, 0.0), (knee, 0.0), (1.0, rise)]
        .into_iter()
        .flat_map(|(x, z)| [Point3::new(x, 0.0, z), Point3::new(x, 1.0, z)])
        .collect();
    NurbsSurface::new(ku, kv, control, vec![1.0; 6]).unwrap()
}

/// **The certificate covers the edge's whole interval.** The diagonal
/// lies on a wall that is flat up to `u = 1/2` and rises past it, so
/// the edge leaves the wall over the last third of its interval
/// (`x = 0.1 + 0.6·t` passes `1/2` at `t = 2/3`) and the certificate
/// refuses it, at the edge certificate alone: no mint, no fitted lane.
/// A certificate whose span covered only the first half of the
/// interval would certify it. The first half alone, stated as the
/// edge, does certify, so the refusal is the second half's.
#[test]
fn the_certificate_reads_the_edges_whole_interval() {
    let wall = kneed_wall(0.5, 0.2);
    let got = plane_nurbs_limbs(
        &diagonal(),
        (0.0, 1.0),
        &diagonal_plane(),
        &wall,
        1.0,
        band(),
    );
    assert!(
        matches!(got, Err(PlaneNurbsRefusal::Limb { .. })),
        "the edge leaves the wall over its last third: {got:?}"
    );
    let half = plane_nurbs_limbs(
        &diagonal(),
        (0.0, 0.5),
        &diagonal_plane(),
        &wall,
        1.0,
        band(),
    );
    assert!(half.is_ok(), "its first half is on the wall: {half:?}");
}

/// The horizontal bilinear chart `z = h` over `[-0.1, 1.1]²`.
fn horizontal_wall(h: f64) -> NurbsSurface<f64> {
    let p = |x, y| Point3::new(x, y, h);
    bilinear([p(-0.1, -0.1), p(-0.1, 1.1), p(1.1, -0.1), p(1.1, 1.1)])
}

/// **NURBS × NURBS, a closed-form carrier.** The diagonal between the
/// flat wall and the vertical wall through it certifies; a circle
/// between the quarter-cylinder wall and a horizontal spline chart
/// certifies over a short arc ([`short`]; the horizontal chart's own
/// image of it is curved, so shorter than on the plane) and refuses at
/// the hull over 1.3 rad. Each
/// spline twin reaches the same verdict.
#[test]
fn a_closed_form_carrier_between_two_spline_charts_meets_its_twins_verdict() {
    let rows = [
        (
            "line",
            diagonal(),
            (0.0, 1.0),
            flat_wall(),
            diagonal_wall(),
            Verdict::Certified,
        ),
        (
            "circle, short",
            circle(0.5),
            short(0.002),
            quarter_cylinder_wall(),
            horizontal_wall(0.5),
            Verdict::Certified,
        ),
        (
            "circle, 1.3 rad",
            circle(0.5),
            (0.1, 1.4),
            quarter_cylinder_wall(),
            horizontal_wall(0.5),
            Verdict::Refused(SsiLimb::HullSup),
        ),
    ];
    for (name, carrier, params, a, b, want) in rows {
        let got = nurbs_pair_limbs(&carrier, params, &a, &b, 1.0, band());
        let (closed, bound) = verdict(name, &got);
        let spline = twin(&carrier, params);
        let (twinned, twin_bound) = verdict(
            name,
            &nurbs_pair_limbs(&spline, params, &a, &b, 1.0, band()),
        );
        println!("{name}: {closed:?} on {bound:e} m; its twin {twinned:?} on {twin_bound:e} m");
        assert_eq!(
            std::mem::discriminant(&closed),
            std::mem::discriminant(&want),
            "{name}: {closed:?}"
        );
        assert_eq!(
            std::mem::discriminant(&closed),
            std::mem::discriminant(&twinned),
            "{name}: the twin's verdict: {closed:?}, {twinned:?}"
        );
        if closed == Verdict::Certified {
            assert!(
                bound <= eps() && twin_bound <= eps(),
                "{name}: both bounds within ε: {bound:e}, {twin_bound:e}"
            );
        } else {
            assert!(
                [bound, twin_bound]
                    .iter()
                    .all(|b| (eps()..1e-3).contains(b)),
                "{name}: a miss past ε, measured: {bound:e}, {twin_bound:e}"
            );
        }
    }
}

/// A wall on `[0, 1]²` that leaves `z = 0` by `peak` between two
/// adjacent stations of the image schedule and is flat elsewhere: in
/// `u`, a degree-4 Bézier over that station span with controls
/// `(0, 0, peak/0.375, 0, 0)` (its middle Bernstein is `6/16` at the
/// span's middle), meeting the flat spans either side level; in `v`,
/// linear.
fn bumped_wall(peak: f64) -> NurbsSurface<f64> {
    let spans = f64::from(geom_brep::edge_nurbs::PXN_FIT_SAMPLES - 1);
    let (a, b) = (16.0 / spans, 17.0 / spans);
    let mut knots = vec![0.0; 5];
    knots.extend([a; 4]);
    knots.extend([b; 4]);
    knots.extend([1.0; 5]);
    let ku = KnotVector::clamped(knots, 4).unwrap();
    let kv = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
    let xs = |lo: f64, hi: f64| (0..4).map(move |k| lo + (hi - lo) * f64::from(k) / 4.0);
    let mut row: Vec<(f64, f64)> = xs(0.0, a).map(|x| (x, 0.0)).collect();
    row.extend(
        xs(a, b)
            .enumerate()
            .map(|(k, x)| (x, if k == 2 { peak / 0.375 } else { 0.0 })),
    );
    row.extend(xs(b, 1.0).map(|x| (x, 0.0)));
    row.push((1.0, 0.0));
    let n = row.len();
    let control = row
        .into_iter()
        .flat_map(|(x, z)| [Point3::new(x, 0.0, z), Point3::new(x, 1.0, z)])
        .collect();
    NurbsSurface::new(ku, kv, control, vec![1.0; 2 * n]).unwrap()
}

/// **A carrier 2ε off the wall between samples is refused at limb 2,
/// with its measured bound.** The line `y = 1/2, z = 0` lies on the
/// plane `y = 1/2` and touches the bumped wall at every station of the
/// schedule, so limb 1 and the image's feet read zero; between them
/// the wall stands `2ε` off it, and only limb 2's hull sees that. `2ε`
/// is inside the band, so the hull's verdict is undecided and says so
/// with the bound it read. The line and its spline twin alike.
#[test]
fn a_carrier_two_epsilon_off_the_wall_between_samples_escalates_at_limb_two() {
    let line = Curve3::Line {
        origin: Point3::new(0.0, 0.5, 0.0),
        dir: Vec3::new(1.0, 0.0, 0.0),
    };
    let plane = Surface::Plane {
        origin: Point3::new(0.0, 0.5, 0.0),
        normal: Vec3::new(0.0, 1.0, 0.0),
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    };
    let wall = bumped_wall(2.0 * eps());
    for (name, carrier) in [("line", line.clone()), ("spline", twin(&line, (0.0, 1.0)))] {
        let (got, frame) = geom_core::k_stats::detached(|| {
            plane_nurbs_limbs(&carrier, (0.0, 1.0), &plane, &wall, 1.0, band())
        });
        match got {
            Err(PlaneNurbsRefusal::Escalated { limb, cause }) => {
                assert!(
                    matches!(limb, SsiLimb::HullSupChart | SsiLimb::HullValueChart),
                    "{name}: limb 2 is what cannot decide: {limb:?}"
                );
                let read = upper(cause.margin);
                assert!(
                    (1.9 * eps()..=4.0 * eps()).contains(&read),
                    "{name}: the bound it read is the 2ε the wall stands off: {read:e}"
                );
                // The image-miss tell reads the carrier's foot here too,
                // 2ε off and in band, but it names a refusal and states
                // nothing: the frame's one escalation is limb 2's own.
                let escalated: Vec<_> = frame
                    .recorded()
                    .escalations
                    .iter()
                    .map(|e| e.predicate())
                    .collect();
                assert_eq!(
                    escalated,
                    [cause
                        .predicate
                        .expect("limb 2's escalation names its predicate")],
                    "{name}: the escalation log holds limb 2's decision alone"
                );
            }
            other => panic!("{name}: a 2ε miss between samples escalates at limb 2: {other:?}"),
        }
    }
}

/// **A carrier definitely off the wall at a break is not the image's
/// miss.** The same line and wall as the 2ε row with the bump at 100ε:
/// limb 1 and the image's feet still read zero at every station, and at
/// the break where limb 2's composite reads past the band the carrier's
/// own foot is 100ε off too, so the refusal is the bound on the
/// carrier's miss (`HullValueChart`), never [`SsiLimb::ImageMiss`].
#[test]
fn a_carrier_definitely_off_the_wall_at_a_break_refuses_on_its_own_miss() {
    let line = Curve3::Line {
        origin: Point3::new(0.0, 0.5, 0.0),
        dir: Vec3::new(1.0, 0.0, 0.0),
    };
    let plane = Surface::Plane {
        origin: Point3::new(0.0, 0.5, 0.0),
        normal: Vec3::new(0.0, 1.0, 0.0),
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    };
    let wall = bumped_wall(100.0 * eps());
    for (name, carrier) in [("line", line.clone()), ("spline", twin(&line, (0.0, 1.0)))] {
        match plane_nurbs_limbs(&carrier, (0.0, 1.0), &plane, &wall, 1.0, band()) {
            Err(PlaneNurbsRefusal::Limb { limb, margin }) => {
                assert_eq!(
                    limb,
                    SsiLimb::HullValueChart,
                    "{name}: the carrier is off at the break, so the miss is its own"
                );
                let read = upper(margin);
                assert!(
                    (50.0 * eps()..=200.0 * eps()).contains(&read),
                    "{name}: the reading is the 100ε the wall stands off: {read:e}"
                );
            }
            other => panic!("{name}: a 100ε miss between samples refuses at limb 2: {other:?}"),
        }
    }
}

/// `z = k·x` over `[0, 1]²`, bilinear: it meets the plane `z = 0` along
/// `x = 0`.
fn slanted_wall(k: f64) -> NurbsSurface<f64> {
    let p = |x, y, z| Point3::new(x, y, z);
    bilinear([
        p(0.0, 0.0, 0.0),
        p(0.0, 1.0, 0.0),
        p(1.0, 0.0, k),
        p(1.0, 1.0, k),
    ])
}

/// Degree 2 along `x = 0`, `y ∈ [0.1, 0.9]`, every interior knot doubled:
/// on the plane `z = 0` at each of its `n + 1` knots and bulging `h` off
/// it, toward `+z`, between them.
fn scallops(n: usize, h: f64) -> Curve3<f64> {
    let mut kv = vec![0.0; 3];
    for i in 1..n {
        #[allow(clippy::cast_precision_loss)]
        let t = i as f64 / n as f64;
        kv.extend([t, t]);
    }
    kv.extend([1.0; 3]);
    let control: Vec<Point3<f64>> = (0..=2 * n)
        .map(|j| {
            #[allow(clippy::cast_precision_loss)]
            let t = j as f64 / (2 * n) as f64;
            let z = if j % 2 == 1 { 2.0 * h } else { 0.0 };
            Point3::new(0.0, 0.1 + 0.8 * t, z)
        })
        .collect();
    Curve3::Nurbs(Arc::new(
        NurbsCurve3::new(
            KnotVector::clamped(kv, 2).unwrap(),
            control,
            vec![1.0; 2 * n + 1],
        )
        .unwrap(),
    ))
}

/// **A bound on the plane is about the carrier alone.** Scallops on the
/// plane `z = 0` at every knot, bulging off it between them, refuse at
/// limb 2 against the PLANE, the first operand the lane certifies: the
/// limb is [`SsiLimb::HullSup`], the analytic one, and its words name
/// an analytic surface and no image.
#[test]
fn a_plane_side_scallop_refuses_on_the_carriers_own_bound() {
    let plane = Surface::Plane {
        origin: Point3::new(0.0, 0.0, 0.0),
        normal: Vec3::new(0.0, 0.0, 1.0),
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    };
    let carrier = scallops(8192, 50.0 * eps());
    let got = plane_nurbs_limbs(
        &carrier,
        (0.0, 1.0),
        &plane,
        &slanted_wall(10.0),
        1.0,
        band(),
    );
    let Err(refusal @ PlaneNurbsRefusal::Limb { limb, .. }) = &got else {
        panic!("the scallops refuse at a limb against the plane: {got:?}")
    };
    assert_eq!(
        *limb,
        SsiLimb::HullSup,
        "the plane's bound, the carrier read alone"
    );
    let words = refusal.to_string();
    assert!(
        words.contains("from an analytic surface") && !words.contains("image"),
        "the words name the analytic surface and no image: {words}"
    );
}

/// **The door's typed refusal reaches the edge certificate.** A spiric
/// whose square root's argument reaches zero at `v = π` (offset equal
/// to the inner radius), on its plane `x = 3/2` and over the flat wall
/// `z = 0` it crosses, is refused at limb 2 by the carrier-enclosure
/// door, naming the kind and the span that holds `π`, not as malformed
/// enclosure data.
#[test]
fn a_carrier_the_door_cannot_enclose_is_refused_by_kind_and_span() {
    let spiric = Curve3::Spiric {
        center: Point3::new(0.0, 0.0, 0.0),
        axis: Vec3::new(0.0, 0.0, 1.0),
        u_ref: Vec3::new(1.0, 0.0, 0.0),
        major_radius: 2.0,
        minor_radius: 0.5,
        offset: 1.5,
    };
    let plane = Surface::Plane {
        origin: Point3::new(1.5, 0.0, 0.0),
        normal: Vec3::new(1.0, 0.0, 0.0),
        u_ref: Vec3::new(0.0, 1.0, 0.0),
    };
    let p = |x, y| Point3::new(x, y, 0.0);
    let wall = bilinear([p(1.0, -1.0), p(1.0, 1.0), p(2.0, -1.0), p(2.0, 1.0)]);
    let got = plane_nurbs_limbs(&spiric, (3.0, 3.3), &plane, &wall, 1.0, band());
    let Err(PlaneNurbsRefusal::CarrierEnclosure(CarrierEnclosureRefusal { kind, span })) = got
    else {
        panic!("the door's refusal, typed: {got:?}")
    };
    assert_eq!(kind, geom::CurveKind::Spiric, "named by kind");
    let pi = core::f64::consts::PI;
    assert!(
        span.0 <= pi && pi <= span.1 && span.1 - span.0 < 0.3 / 16.0,
        "named by the span holding π, one span of the grid: {span:?}"
    );
}

/// The arena and spec of an `Intersection` of `a` and `b` over
/// `carrier` on `params`, its witness the carrier's midpoint.
fn door_spec(
    a: Surface<f64>,
    b: Surface<f64>,
    carrier: Curve3<f64>,
    (t0, t1): (f64, f64),
) -> (
    impl Fn(SurfaceKey) -> Option<Surface<f64>>,
    EdgeCurveSpec<f64>,
) {
    let mut arena: SlotMap<SurfaceKey, Surface<f64>> = SlotMap::with_key();
    let (s1, s2) = (arena.insert(a), arena.insert(b));
    let witness = carrier.eval(0.5 * (t0 + t1));
    (
        move |k| arena.get(k).cloned(),
        EdgeCurveSpec {
            description: EdgeDescriptionSpec::Intersection {
                pair: SurfacePair::new(s1, s2),
                selector: geom_brep::CurveSelector::Witness(witness),
            },
            carrier,
            param_start: t0,
            param_end: t1,
        },
    )
}

/// **Through the certification door**: the diagonal as a `Line` on the
/// plane × wall pair and on the wall × wall pair certifies, recording
/// the lane's bound, and a circle over 1.3 rad on the plane × wall pair
/// refuses at limb 2 with its measured miss — where the door
/// refused every non-spline carrier before it read a limb.
#[test]
fn the_door_certifies_a_closed_form_carrier_on_a_spline_pair() {
    let lane = NurbsLane::certified();
    let line = diagonal();
    let ends = (line.eval(0.0), line.eval(1.0));
    for (name, a, b) in [
        (
            "plane × wall",
            diagonal_plane(),
            Surface::Nurbs(Arc::new(flat_wall())),
        ),
        (
            "wall × wall",
            Surface::Nurbs(Arc::new(diagonal_wall())),
            Surface::Nurbs(Arc::new(flat_wall())),
        ),
    ] {
        let (arena, spec) = door_spec(a, b, line.clone(), (0.0, 1.0));
        let edge = EdgeCurve::certify(spec, ends.0, ends.1, arena, 1.0, band(), lane)
            .unwrap_or_else(|e| panic!("{name}: the line certifies through the door: {e}"));
        assert!(
            edge.certificate().max_residual <= eps(),
            "{name}: within ε: {:e}",
            edge.certificate().max_residual
        );
    }
    let (t0, t1) = (0.1, 1.4);
    let arc = circle(0.5);
    let (arena, spec) = door_spec(
        horizontal(0.5),
        Surface::Nurbs(Arc::new(quarter_cylinder_wall())),
        arc.clone(),
        (t0, t1),
    );
    let foot = carrier_foot_sup(&arc, (t0, t1), &quarter_cylinder_wall());
    println!("the circle's own foot reads {foot:e} m at most");
    assert!(foot <= ON_THE_WALL, "the circle is on the wall: {foot:e} m");
    let got = EdgeCurve::certify(spec, arc.eval(t0), arc.eval(t1), arena, 1.0, band(), lane);
    let Err(CertifyError::PlaneNurbs(PlaneNurbsRefusal::Limb { limb, margin })) = got else {
        panic!("the circle refuses at a limb through the door: {got:?}")
    };
    assert!(
        matches!(limb, SsiLimb::HullSupChart | SsiLimb::ImageMiss),
        "limb 2 refuses: {limb:?}"
    );
    assert!(
        (eps()..1e-3).contains(&upper(margin)),
        "a miss past ε, measured: {:e}",
        upper(margin)
    );
}

/// **D7's promoted line certifies wherever its spline did.** An off-iso
/// straight edge between a plane and a flat bilinear wall, stated as a
/// degree-1 spline and as the `Line` D7 recognition promotes it to (its
/// first control point the origin, the unit chord the direction, on the
/// same interval because the chord has unit length): both certify
/// through the door on bounds within roundoff of each other, and the
/// chart image the certificate derives (the mint's own producer,
/// `FittedLane::general_image`) is the same net for both.
#[test]
fn a_promoted_line_certifies_where_its_spline_did_on_the_same_image() {
    let lane = NurbsLane::certified();
    let line = diagonal();
    let spline = twin(&line, (0.0, 1.0));
    let ends = (line.eval(0.0), line.eval(1.0));
    let mut bounds = Vec::new();
    for carrier in [&spline, &line] {
        let (arena, spec) = door_spec(
            diagonal_plane(),
            Surface::Nurbs(Arc::new(flat_wall())),
            carrier.clone(),
            (0.0, 1.0),
        );
        let edge = EdgeCurve::certify(spec, ends.0, ends.1, arena, 1.0, band(), lane)
            .unwrap_or_else(|e| panic!("{:?} certifies: {e}", carrier.kind()));
        bounds.push(edge.certificate().max_residual);
    }
    println!("spline, then line: bounds {bounds:?}");
    assert!(
        bounds.iter().all(|b| *b <= eps()) && (bounds[0] - bounds[1]).abs() <= 1e-14,
        "the same bound, within ε and within roundoff of each other: {bounds:?}"
    );
    let fitted = geom_brep::FittedLane::<f64>::certified();
    let images: Vec<Vec<(f64, f64)>> = [&spline, &line]
        .into_iter()
        .map(|c| {
            let image = fitted.general_image(c, (0.0, 1.0), &flat_wall()).unwrap();
            assert_eq!(
                image.knots_f64_interim().domain(),
                (0.0, 1.0),
                "on the edge's interval"
            );
            image.control().iter().map(|p| (p.x, p.y)).collect()
        })
        .collect();
    let apart = images[0]
        .iter()
        .zip(&images[1])
        .map(|(a, b)| (a.0 - b.0).abs().max((a.1 - b.1).abs()))
        .fold(0.0, f64::max);
    assert!(
        images[0].len() == images[1].len() && apart <= 1e-15,
        "one image, its feet within roundoff: {apart:e}"
    );
}

/// `x` as an `Interval` point.
fn itv(x: f64) -> geom_core::interval::Interval {
    use geom_core::Real as _;
    geom_core::interval::Interval::from_f64(x)
}

/// **The closed forms certify alike at `Interval`.** The diagonal on
/// the plane × wall pair and on the wall × wall pair, and a short
/// circle on the quarter-cylinder wall, certify at `Interval` as at
/// `f64`, each hull bound within ε; the 1.3 rad circle refuses at limb
/// 2's break value at both scalars. The two bounds are not nested: each
/// scalar derives its own chart image from its own foot schedule, and
/// each bound is certified for the image it read (the short circle's
/// `Interval` bound comes out a few parts in 10⁶ BELOW the `f64` one at
/// the default ε, and above it at 1e-6 and 1e-12).
#[test]
fn the_closed_forms_reach_the_same_verdict_at_interval() {
    use geom_core::interval::Interval;
    use geom_core::{Bounds as _, Real as _};
    let one = Interval::from_f64(1.0);
    let lifted = |p: (f64, f64)| (itv(p.0), itv(p.1));
    let line = diagonal();
    let short_arc = short(0.002);
    let certified: [(&str, Limbs, IntervalLimbs); 3] = [
        (
            "line, plane × wall",
            plane_nurbs_limbs(
                &line,
                (0.0, 1.0),
                &diagonal_plane(),
                &flat_wall(),
                1.0,
                band(),
            ),
            plane_nurbs_limbs(
                &line.map_scalar(itv),
                lifted((0.0, 1.0)),
                &diagonal_plane().map_scalar(itv),
                &flat_wall().map_scalar(itv),
                one,
                band(),
            ),
        ),
        (
            "line, wall × wall",
            nurbs_pair_limbs(
                &line,
                (0.0, 1.0),
                &flat_wall(),
                &diagonal_wall(),
                1.0,
                band(),
            ),
            nurbs_pair_limbs(
                &line.map_scalar(itv),
                lifted((0.0, 1.0)),
                &flat_wall().map_scalar(itv),
                &diagonal_wall().map_scalar(itv),
                one,
                band(),
            ),
        ),
        (
            "circle, short",
            plane_nurbs_limbs(
                &circle(0.5),
                short_arc,
                &horizontal(0.5),
                &quarter_cylinder_wall(),
                1.0,
                band(),
            ),
            plane_nurbs_limbs(
                &circle(0.5).map_scalar(itv),
                lifted(short_arc),
                &horizontal(0.5).map_scalar(itv),
                &quarter_cylinder_wall().map_scalar(itv),
                one,
                band(),
            ),
        ),
    ];
    for (name, at_f64, at_itv) in certified {
        let (Ok(f), Ok(i)) = (&at_f64, &at_itv) else {
            panic!("{name}: certifies at both scalars: {at_f64:?}, {at_itv:?}")
        };
        println!(
            "{name}: f64 hull {:e}, Interval hull {:?}",
            f.hull_sup, i.hull_sup
        );
        assert!(
            f.hull_sup <= eps() && i.hull_sup.hi() <= eps(),
            "{name}: both hull bounds within ε: {:e} and {:?}",
            f.hull_sup,
            i.hull_sup
        );
    }
    let at_f64 = plane_nurbs_limbs(
        &circle(0.5),
        (0.1, 1.4),
        &horizontal(0.5),
        &quarter_cylinder_wall(),
        1.0,
        band(),
    );
    let at_itv = plane_nurbs_limbs(
        &circle(0.5).map_scalar(itv),
        lifted((0.1, 1.4)),
        &horizontal(0.5).map_scalar(itv),
        &quarter_cylinder_wall().map_scalar(itv),
        one,
        band(),
    );
    // The tell's label, not only a refusal, is the scalar's to agree on:
    // the circle is on the wall at the break at both, so both name the
    // image's own miss.
    let feet = (
        carrier_foot_sup(&circle(0.5), (0.1, 1.4), &quarter_cylinder_wall()),
        carrier_foot_sup(
            &circle(0.5).map_scalar(itv),
            (0.1, 1.4),
            &quarter_cylinder_wall().map_scalar(itv),
        ),
    );
    println!(
        "the circle's own foot, f64 then Interval: {:e}, {:e} m at most",
        feet.0, feet.1
    );
    assert!(
        feet.0 <= ON_THE_WALL && feet.1 <= ON_THE_WALL,
        "the circle is on the wall at both scalars: {:e}, {:e} m",
        feet.0,
        feet.1
    );
    let label = |scalar: &str, got: Option<&PlaneNurbsRefusal>| match got {
        Some(PlaneNurbsRefusal::Limb { limb, .. }) => ("limb", *limb),
        Some(PlaneNurbsRefusal::Escalated { limb, .. }) => ("escalated", *limb),
        other => panic!("the 1.3 rad circle refuses at limb 2 at {scalar}: {other:?}"),
    };
    let (f, i) = (
        label("f64", at_f64.as_ref().err()),
        label("Interval", at_itv.as_ref().err()),
    );
    assert_eq!(f, ("limb", SsiLimb::ImageMiss), "at f64: {at_f64:?}");
    assert_eq!(i, f, "Interval reaches f64's verdict and label: {at_itv:?}");
}

/// **A circle's parameter is periodic.** The short arc read at `t + 2π`
/// is the same locus, and certifies on the same bound.
#[test]
fn a_short_arc_past_two_pi_certifies_on_the_same_bound() {
    let tau = core::f64::consts::TAU;
    let (a, b) = short(0.002);
    let bound = |params| {
        plane_nurbs_limbs(
            &circle(0.5),
            params,
            &horizontal(0.5),
            &quarter_cylinder_wall(),
            1.0,
            band(),
        )
        .unwrap_or_else(|e| panic!("{params:?}: the short arc certifies: {e}"))
        .hull_sup
    };
    let (base, shifted) = (bound((a, b)), bound((a + tau, b + tau)));
    println!("short arc: {base:e}; past 2π: {shifted:e}");
    assert!(
        (shifted - base).abs() <= 1e-3 * base.max(f64::MIN_POSITIVE) + 1e-15,
        "the same bound past 2π: {base:e} against {shifted:e}"
    );
}
