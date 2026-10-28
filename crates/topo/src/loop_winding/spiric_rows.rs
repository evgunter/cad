//! **A spiric arc's term in the winding sum** ([`super::spiric_segment_term`]):
//! its bulge over its chord against the area a dense polygon of the arc
//! cuts off, and the winding of loops on whole ovals.

use super::*;
use core::f64::consts::{PI, TAU};

/// The oval of the torus `(R, r)` about `axis` through `c`, cut by the
/// plane `offset` along `u_ref`.
fn oval(
    c: Point3<f64>,
    axis: Vec3<f64>,
    u_ref: Vec3<f64>,
    (big_r, r): (f64, f64),
    offset: f64,
) -> Curve3<f64> {
    Curve3::Spiric {
        center: c,
        axis,
        u_ref,
        major_radius: big_r,
        minor_radius: r,
        offset,
    }
}

/// Twice the vector area between the arc of `carrier` from `ta` to `tb`
/// and its chord, by the polygon of `n` points along it: short of the
/// arc by `O(1/n²)`.
fn polygon(carrier: &Curve3<f64>, (ta, tb): (f64, f64), n: u32) -> Vec3<f64> {
    let p0 = carrier.eval(ta);
    (0..n)
        .map(|k| {
            let t = |k: u32| ta + (tb - ta) * f64::from(k) / f64::from(n);
            (carrier.eval(t(k)) - p0).cross(carrier.eval(t(k + 1)) - p0)
        })
        .fold(Vec3::zero(), |a, b| a + b)
}

/// [`polygon`] at 2000 and 4000 points, extrapolated (Richardson): the
/// `1/n²` terms cancel.
fn polygon_bulge(carrier: &Curve3<f64>, span: (f64, f64)) -> Vec3<f64> {
    (polygon(carrier, span, 4000) * 4.0 - polygon(carrier, span, 2000)) / 3.0
}

/// The carriers every row reads: the donut `(2, 0.5)` and a fat torus
/// `(1, 0.6)`, each cut on both sides of its axis's trace, in two frames.
fn carriers() -> Vec<(String, Curve3<f64>)> {
    let frames = [
        (Point3::origin(), Vec3::unit_y(), Vec3::unit_x()),
        (
            Point3::new(0.3, -1.2, 2.5),
            Vec3::new(2.0, 2.0, 1.0) / 3.0,
            Vec3::new(1.0, -2.0, 2.0) / 3.0,
        ),
    ];
    let mut out = Vec::new();
    for (k, &(c, axis, u)) in frames.iter().enumerate() {
        for (radii, offsets) in [((2.0, 0.5), [0.3, -1.2]), ((1.0, 0.6), [0.1, -0.2])] {
            for d in offsets {
                for (u_ref, offset) in [(u, d), (-u, -d)] {
                    out.push((
                        format!("frame {k}, {radii:?}, offset {offset}"),
                        oval(c, axis, u_ref, radii, offset),
                    ));
                }
            }
        }
    }
    out
}

/// `carrier`'s arc `(t0, t1)` lifted to the interval scalar.
fn lifted(carrier: &Curve3<f64>, (t0, t1): (f64, f64)) -> (Curve3<Interval>, (Interval, Interval)) {
    (
        carrier.map_scalar(Interval::from_certified),
        (Interval::from_certified(t0), Interval::from_certified(t1)),
    )
}

/// **A spiric arc's bulge is the area it cuts off its chord**: arcs
/// short and long, across the seam at `0` and the inner point at `π`, the
/// whole oval, forward and back, on every carrier; the certified
/// enclosure at 1024 pieces holds the area a dense polygon of the arc
/// cuts off, to the polygon's own error, and is under `1e-11` m² wide;
/// the length bounds the polygon's.
#[test]
fn a_spiric_arcs_bulge_is_the_area_it_cuts_off_its_chord() {
    use geom_core::Bounds;
    for (name, carrier) in carriers() {
        for (t0, t1) in [(0.2, 0.9), (-1.0, 2.5), (2.0, 4.0), (-PI, PI), (0.0, TAU)] {
            for forward in [true, false] {
                let (c, span) = lifted(&carrier, (t0, t1));
                let (bulge, length) =
                    spiric_segment_term((&c, span), forward, 1024).expect("a spiric's term");
                let span = if forward { (t0, t1) } else { (t1, t0) };
                let want = polygon_bulge(&carrier, span);
                let scale = want.norm().max(1e-3);
                for (k, (got, want)) in [(bulge.x, want.x), (bulge.y, want.y), (bulge.z, want.z)]
                    .into_iter()
                    .enumerate()
                {
                    assert!(
                        got.lo() - 1e-8 * scale <= want && want <= got.hi() + 1e-8 * scale,
                        "{name}, {t0}..{t1}, forward {forward}: component {k} [{}, {}] \
                         against {want}",
                        got.lo(),
                        got.hi()
                    );
                    assert!(
                        got.hi() - got.lo() <= 1e-11,
                        "{name}, {t0}..{t1}: component {k} {} wide",
                        got.hi() - got.lo()
                    );
                }
                let walked: f64 = (0..1000)
                    .map(|k| {
                        let t = |k: i32| t0 + (t1 - t0) * f64::from(k) / 1000.0;
                        carrier.eval(t(k)).distance(carrier.eval(t(k + 1)))
                    })
                    .sum();
                assert!(
                    length.lo() >= walked,
                    "{name}, {t0}..{t1}: length {} under {walked}",
                    length.lo()
                );
            }
        }
    }
}

/// A planar face bounded by the whole oval `x = d` cuts on the `+z`
/// side of the donut `(2, 0.5)` about `y`, in two edges, and the face
/// the closing edge mints beside it: the two loops of one curve, run
/// opposite ways.
fn oval_faces(d: f64) -> (Body<f64>, [LoopKey; 2]) {
    use crate::euler::{FaceSurface, MefSite};
    let tol = geom_core::Tol::witness();
    let carrier = oval(
        Point3::origin(),
        Vec3::unit_y(),
        Vec3::unit_x(),
        (2.0, 0.5),
        d,
    );
    let mut body = Body::<f64>::new();
    let seed = body.mvfs(carrier.eval(0.0), true).unwrap();
    let plane = body
        .set_face_surface(
            seed.face,
            FaceSurface::New {
                surface: geom::Surface::Plane {
                    origin: Point3::new(d, 0.0, 0.0),
                    normal: Vec3::unit_x(),
                    u_ref: Vec3::unit_y(),
                },
                sense: true,
            },
        )
        .unwrap();
    let torus = body.add_surface(geom::Surface::Torus {
        center: Point3::origin(),
        axis: Vec3::unit_y(),
        major_radius: 2.0,
        minor_radius: 0.5,
        u_ref: Vec3::unit_x(),
    });
    let spec = |(t0, t1): (f64, f64)| geom_brep::EdgeCurveSpec {
        description: geom_brep::EdgeDescriptionSpec::Intersection {
            pair: geom_brep::SurfacePair::new(plane, torus),
            selector: geom_brep::CurveSelector::Witness(carrier.mid_point(t0, t1)),
        },
        carrier: carrier.clone(),
        param_start: t0,
        param_end: t1,
    };
    let first = body
        .mev(
            crate::construction::MevSite::Lone {
                r#loop: seed.r#loop,
            },
            carrier.eval(PI),
            spec((0.0, PI)),
            tol,
        )
        .unwrap();
    let made = body
        .mef(
            MefSite::Chords {
                he1: first.he_minus,
                he2: first.he_plus,
            },
            spec((PI, TAU)),
            FaceSurface::Shared {
                key: plane,
                sense: true,
            },
            tol,
        )
        .unwrap();
    let outer = |f| body.get_face(f).unwrap().outer;
    let loops = [outer(seed.face), outer(made.face)];
    (body, loops)
}

/// **A loop on a whole oval winds, one way round and the other, at
/// every stand-off from the inner equator**: the oval of `x = d` on the
/// donut bounds two faces, whose loops wind opposite ways about the
/// plane's normal, for `d = 0.3` and for planes `1e-3` to `1e-9` short of
/// the inner equator, where the oval pinches toward a point at `t = π`.
#[test]
fn a_loop_on_an_oval_winds_one_way_round_and_the_other() {
    let band = geom_core::Band::linear(geom_core::Tol::witness()).unwrap();
    for d in [0.3, 1.5 - 1e-3, 1.5 - 1e-6, 1.5 - 1e-9] {
        let (body, loops) = oval_faces(d);
        let wound = loops.map(|l| body.planar_loop_winding(l, Vec3::unit_x(), band));
        assert!(
            matches!(
                wound,
                [Some(Ok(Sign::Positive)), Some(Ok(Sign::Negative))]
                    | [Some(Ok(Sign::Negative)), Some(Ok(Sign::Positive))]
            ),
            "x = {d}: the two loops of one oval: {wound:?}"
        );
    }
}

/// **A spiric arc the enclosure does not read is never wound as its
/// chord**: the open run along the donut's oval from `0.2` to `0.9` and
/// back by its chord winds, but the same arc stored with its span
/// descending (no certified enclosure, [`super::spiric_segment_term`]'s
/// `None`) has no winding ([`Unwound::Uncertified`]) rather than the
/// chord's.
#[test]
fn a_spiric_arc_with_no_enclosure_has_no_winding() {
    let band = geom_core::Band::linear(geom_core::Tol::witness()).unwrap();
    let carrier = oval(
        Point3::origin(),
        Vec3::unit_y(),
        Vec3::unit_x(),
        (2.0, 0.5),
        0.3,
    );
    let (a, b) = (carrier.eval(0.2), carrier.eval(0.9));
    let wound = |span: (f64, f64), forward: bool| {
        Body::<f64>::spiric_winding(
            &[(a, Some(((&carrier, span), forward)))],
            b,
            None,
            true,
            Vec3::unit_x(),
            band,
        )
    };
    assert!(
        matches!(wound((0.2, 0.9), true), Ok(Ok(_))),
        "the ascending arc winds: {:?}",
        wound((0.2, 0.9), true)
    );
    assert!(
        matches!(wound((0.9, 0.2), false), Err(Unwound::Uncertified)),
        "the descending arc: {:?}",
        wound((0.9, 0.2), false)
    );
}

/// **A small loop near the pinch winds decided**: an arc of an oval
/// cut `1e-14` to `1.3e-9` short of the inner equator, closed by its
/// chord, decides with its sign wherever its area over its perimeter
/// clears the band's escalation. That ratio and its sign are a 40-digit
/// oracle's (the arc's area by Green's form and by parts, which agree
/// to 25 digits, and its length by quadrature of `|P′|`). The arcs run
/// along the oval's neck, where `sup|sin t|` over `inf f` read apart
/// is up to 10⁴ times the arc's speed ([`spiric::spiric_length_bound`]).
/// On the two loops cut `1e-14` short, that reading puts the margin near
/// `1.5e-12`, under `ε = 1e-12`'s escalation, against the oracle's
/// `2.2e-8` and `4.8e-8`: a reading apart fails at every ε.
#[test]
fn a_small_loop_near_the_pinch_winds_decided() {
    /// `((R, r), d, (t₀, t₁), forward, area/perimeter)`.
    type PinchLoop = ((f64, f64), f64, (f64, f64), bool, f64);
    let band = geom_core::Band::linear(geom_core::Tol::witness()).unwrap();
    let loops: [PinchLoop; 7] = [
        ((3.0, 1.0), 2.0 - 1.3e-9, (2.68, 3.1414), false, -1.850e-3),
        (
            (2.0, 0.5),
            -(1.5 - 1e-9),
            (PI + 1e-3, PI + 0.4),
            true,
            5.698e-4,
        ),
        ((2.0, 0.5), 1.5 - 1e-12, (PI - 1e-3, PI), true, 2.138e-7),
        ((3.0, 1.0), -(2.0 - 1e-12), (PI, PI + 2e-3), true, 2.873e-7),
        (
            (1.0, 0.6),
            0.4 - 1e-12,
            (PI + 1e-6, PI + 4e-3),
            false,
            -1.030e-7,
        ),
        (
            (2.0, 0.5),
            1.5 - 1e-14,
            (PI - 1e-2, PI - 1e-7),
            true,
            2.239e-8,
        ),
        (
            (3.0, 1.0),
            -(2.0 - 1e-14),
            (PI, PI + 1e-2),
            false,
            -4.799e-8,
        ),
    ];
    let mut read = 0;
    for ((big_r, r), d, (t0, t1), forward, ratio) in loops {
        if ratio.abs() <= 2.0 * band.escalate() {
            continue;
        }
        read += 1;
        let carrier = oval(
            Point3::origin(),
            Vec3::unit_y(),
            Vec3::unit_x(),
            (big_r, r),
            d,
        );
        let (ta, tb) = if forward { (t0, t1) } else { (t1, t0) };
        let wound = Body::<f64>::spiric_winding(
            &[(carrier.eval(ta), Some(((&carrier, (t0, t1)), forward)))],
            carrier.eval(tb),
            None,
            true,
            Vec3::unit_x(),
            band,
        );
        let want = if ratio > 0.0 {
            Sign::Positive
        } else {
            Sign::Negative
        };
        assert!(
            matches!(wound, Ok(Ok(Decided { sign, .. })) if sign == want),
            "({big_r}, {r}) cut at {d}, {t0}..{t1} forward {forward} (area/perimeter \
             {ratio}): {wound:?}"
        );
    }
    assert!(read > 0, "no loop clears the band at this ε");
}

/// **A count with no term reads nothing; a finer one decides**: an arc
/// of the torus `(1, 0.4)` cut through its hole at `0.8`, over its own
/// arc `|t| < acos(−1/2)` to within `10⁻³` and `10⁻²` of that arc's ends,
/// closed by its chord. At 16 pieces the end piece's `f` is not certified
/// positive, so the arc has no term there; at 128 it has one, and the
/// loop winds with the sign of its area over its perimeter, `+0.229` by a
/// 40-digit oracle.
#[test]
fn a_spiric_with_no_term_at_the_first_count_winds_at_a_finer_one() {
    let band = geom_core::Band::linear(geom_core::Tol::witness()).unwrap();
    let arc = (-0.5_f64).acos();
    let (t0, t1) = (-arc * (1.0 - 1e-3), arc * (1.0 - 1e-2));
    let carrier = oval(
        Point3::origin(),
        Vec3::unit_y(),
        Vec3::unit_x(),
        (1.0, 0.4),
        0.8,
    );
    let (lifted_carrier, span) = lifted(&carrier, (t0, t1));
    assert!(
        spiric_segment_term((&lifted_carrier, span), true, SPIRIC_PIECES[0]).is_none()
            && spiric_segment_term((&lifted_carrier, span), true, SPIRIC_PIECES[1]).is_some(),
        "the fixture's premise: no term at the first count, one at the next"
    );
    let wound = Body::<f64>::spiric_winding(
        &[(carrier.eval(t0), Some(((&carrier, (t0, t1)), true)))],
        carrier.eval(t1),
        None,
        true,
        Vec3::unit_x(),
        band,
    );
    assert!(
        matches!(
            wound,
            Ok(Ok(Decided {
                sign: Sign::Positive,
                ..
            }))
        ),
        "{wound:?}"
    );
}
