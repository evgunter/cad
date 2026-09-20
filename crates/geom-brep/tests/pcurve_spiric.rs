//! **The spiric pcurve lane's crate-local rows** — the door, not a
//! body.
//!
//! `sweep`'s `spiric_rim` suite measures what the kernel's one
//! spiric-bearing body does; these rows measure what
//! `PcurveCache::certify` does with images a body cannot hand it. That
//! distinction is the reason this file exists: every premise the
//! wall's `EnvelopeStatement::SpiricIdentity` rests on is bit-true on
//! every minted image, so a body-level row can only ever see the
//! premise satisfied, and the arms that carry the premise when it is
//! NOT satisfied were shipped once with nothing looking at them.
//!
//! The fixture is the sectioned vessel's cavity rim, written out as
//! numbers (`R = 0.09375`, `r = 0.0703125`, `d = 0.0078125`, the
//! quarter-revolve span) so the rows stand on their own.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::{Curve3, Surface};
use geom_brep::{
    ChartWindow, EnvelopeStatement, Pcurve, PcurveCache, PcurveCertifyError, SpiricImage,
};
use geom_core::{Band, Point2, Point3, Tol, Vec2, Vec3};

const R: f64 = 0.09375;
const RR: f64 = 0.0703125;
const D: f64 = 0.0078125;
const T0: f64 = -0.8911225078866529;
const T1: f64 = 0.8911225078866529;
/// The chart's azimuth lever `R + r` — the arm both the `sense` gate
/// and the tilt gate meter at.
const ARM: f64 = R + RR;

fn tol() -> Tol {
    Tol::witness()
}

fn band() -> Band {
    Band::linear(tol()).expect("band")
}

/// The carrier: a spiric on the `+y`-axis torus at the origin, cut by
/// the plane whose normal is `+x` standing `D` off the centre.
fn carrier() -> Curve3<f64> {
    Curve3::Spiric {
        center: Point3::origin(),
        axis: Vec3::unit_y(),
        u_ref: Vec3::unit_x(),
        major_radius: R,
        minor_radius: RR,
        offset: D,
    }
}

/// The carrier's own torus, as a chart.
fn torus() -> Surface<f64> {
    Surface::Torus {
        center: Point3::origin(),
        axis: Vec3::unit_y(),
        major_radius: R,
        minor_radius: RR,
        u_ref: Vec3::unit_x(),
    }
}

/// The cutting plane, as a chart: origin on the plane, normal `+x`,
/// `u_ref` the direction `m = axis × n` the cap's `f` channel runs
/// along.
fn cap_plane() -> Surface<f64> {
    Surface::Plane {
        origin: Point3::new(D, 0.0, 0.0),
        normal: Vec3::unit_x(),
        u_ref: Vec3::unit_y().cross(Vec3::unit_x()),
    }
}

/// The wall image the mint produces for [`carrier`] on [`torus`],
/// with `sense` and `v0` open so a row can move exactly one number.
fn wall(sense: f64, v0: f64) -> Pcurve<f64> {
    Pcurve::Spiric {
        major: R,
        minor: RR,
        offset: D,
        image: SpiricImage::Wall {
            // `u0` is the chart azimuth of the cutting plane's normal:
            // `n = +x` is the chart's own `u_ref`, so it is zero here.
            u0: 0.0,
            v0,
            sense,
        },
    }
}

/// The cap image the mint produces for [`carrier`] on [`cap_plane`],
/// with the `f` coefficient open. At `scale = 1.0` it is the minted
/// one; `k₁` is then exactly `m`, which is what check 4's
/// `(k₁ − m)·f_max` term measures.
fn cap(scale: f64) -> Pcurve<f64> {
    Pcurve::Spiric {
        major: R,
        minor: RR,
        offset: D,
        image: SpiricImage::Cap {
            p0: Point2::new(0.0, 0.0),
            pm: Vec2::new(scale, 0.0),
            pa: Vec2::new(0.0, RR),
        },
    }
}

fn certify(
    p: Pcurve<f64>,
    surface: &Surface<f64>,
    b: Band,
) -> Result<PcurveCache<f64>, PcurveCertifyError> {
    let window = wide();
    PcurveCache::certify(p, T0, T1, &carrier(), surface, window, b)
}

/// A window no chart box can escape — check 5 is not what these rows
/// are about.
fn wide() -> ChartWindow<f64> {
    ChartWindow {
        u_min: -100.0,
        u_max: 100.0,
        v_min: -100.0,
        v_max: 100.0,
    }
}

/// **The minted pair is the baseline**: the wall certifies
/// `SpiricIdentity` with an exactly zero envelope and the cap
/// certifies `MapResidualClosedForm`, both at rounding-level
/// residuals. Everything below moves one number away from here.
#[test]
fn the_minted_wall_and_cap_images_certify_with_a_zero_envelope() {
    let w = certify(wall(1.0, 0.0), &torus(), band()).expect("the minted wall certifies");
    assert_eq!(w.certificate().statement, EnvelopeStatement::SpiricIdentity);
    assert_eq!(w.certificate().envelope, 0.0);
    assert!(w.certificate().max_residual <= 1e-15);
    let c = certify(cap(1.0), &cap_plane(), band()).expect("the minted cap certifies");
    assert_eq!(
        c.certificate().statement,
        EnvelopeStatement::MapResidualClosedForm
    );
    assert!(c.certificate().envelope <= 1e-15);
    assert!(c.certificate().max_residual <= 1e-15);
}

/// **The `sense` gate's band is where its DIMENSION puts it.**
/// `|sense| − 1` is dimensionless, so it is metered by MULTIPLYING the
/// chart's azimuth arm: `Sign::Zero` holds while `|η|·(R + r) ≤ ε`,
/// i.e. `|η| ≤ ε/(R + r) ≈ 6.1e-9`. The row walks the window's two
/// sides and reads WHICH check answers, because that is what
/// distinguishes the door from its predecessor:
///
/// - `η = 0.8·ε/(R + r)` is INSIDE check 1's window, so check 1 admits it and
///   the refusal comes from check 4 — the envelope, where the drift
///   this gate admitted is priced. Under the retired `over_lever`
///   door the window was `|η| ≤ ε·(R + r) ≈ 1.6e-10` and this same
///   input refused at check 1 as `UnsupportedCarrier`, 37× early.
/// - `η = 50·ε/(R + r)` is past the escalation threshold either way
///   and refuses `UnsupportedCarrier`.
///
/// And the price itself is the second half of the fix: the envelope's
/// `sense` term carries BOTH channels. `v` moves by `η·reach` at
/// `|∂S/∂v| = r`; `u` moves by `η·|atan2(f, d)| ≤ η·π` at the chart's
/// outer arm — the `u` half the over-strict gate used to mask, and the
/// reason the admitted `η` costs `≈ 2.5·ε` rather than `≈ 0.1·ε`.
///
/// Every magnitude is a multiple of the run's ε — CI gates three of
/// them and a hard-coded drift is a row that holds at one.
#[test]
fn the_sense_gates_band_is_the_levered_one_and_both_channels_are_priced() {
    // Every magnitude below is a multiple of the RUN's ε, because CI
    // gates three of them (1e-9, 1e-6, 1e-12) and a hard-coded drift
    // is a row that only holds at one.
    let eps = tol().eps();
    let window = eps / ARM;
    assert!(
        (window / eps - 6.09).abs() < 0.01,
        "the levered window is eps/(R+r): {window:e}"
    );

    // Just inside: admitted by check 1, and refused DOWNSTREAM of it
    // — by the schedule or the envelope, both of which price the
    // displacement `η` buys. What the row pins is which check did NOT
    // answer: `UnsupportedCarrier` is check 1's refusal, and under the
    // retired `over_lever` door that is exactly what this input got,
    // 37× early.
    let e = certify(wall(1.0 + 0.8 * window, 0.0), &torus(), band())
        .expect_err("a drift this size is a displacement over eps");
    assert!(
        !matches!(e, PcurveCertifyError::UnsupportedCarrier),
        "check 1 must ADMIT a residue inside its own levered window; got {e:?}"
    );
    assert!(
        matches!(
            e,
            PcurveCertifyError::ResidualExceeded { .. } | PcurveCertifyError::Escalated { .. }
        ),
        "and the price is what refuses; got {e:?}"
    );

    // A drift small enough that the price fits under eps certifies,
    // and its envelope is NOT zero — the term is live, not decorative.
    // Small enough that the azimuth price `arm·η·π` fits under ε.
    let afford = 0.1 * eps / ARM;
    let ok = certify(wall(1.0 + afford, 0.0), &torus(), band())
        .expect("a drift the envelope can afford certifies");
    let env = ok.certificate().envelope;
    assert!(
        env > 0.0 && env <= eps,
        "the admitted drift is priced and affordable: {env:e}"
    );
    // The u channel DOMINATES the price: `r·η·reach` alone is an order
    // under `arm·η·π`, so a term that carried only the minor angle
    // would not reach this floor.
    assert!(
        env >= ARM * afford * core::f64::consts::PI,
        "the azimuth half of the sense price is in the envelope: {env:e}"
    );

    // Outside both windows.
    let e = certify(wall(1.0 + 50.0 * window, 0.0), &torus(), band())
        .expect_err("a definite sense residue is not a unit sign");
    assert!(matches!(e, PcurveCertifyError::UnsupportedCarrier), "{e:?}");
}

/// **The identity's other premise: the chart IS the carrier's torus.**
/// Check 1 gates the chart's centre, axis, `R` and `r` against the
/// carrier's, and the residues ride into the envelope. Before those
/// four gates a wall image certified against a DRIFTED torus and
/// stored `SpiricIdentity` with `envelope = 0` while every sample was
/// displaced by ~1e-9 m — a certificate that says "zero, by an
/// algebraic identity" over a real error, read downstream as slack.
///
/// The author's choice between the adjudication's two options is
/// taken here and stated: a drift the band ADMITS certifies with a
/// NONZERO envelope (the number is the honest one), and a drift the
/// band calls definite refuses `UnsupportedCarrier`.
#[test]
fn a_drifted_chart_is_priced_or_refused_but_never_certified_as_zero() {
    let drifted = |f: &dyn Fn(&mut Surface<f64>)| {
        let mut s = torus();
        f(&mut s);
        s
    };

    // Every drift below is a multiple of the RUN's ε: `inside` is
    // within check 1's Zero window and `definite` is past the
    // escalation threshold, at all three ε cells CI gates.
    let eps = tol().eps();
    let inside = 0.5 * eps;
    let definite = 50.0 * eps;

    // Each of the four premises, drifted inside the band: certifies,
    // envelope strictly positive and at least the drift itself.
    let cases: [(&str, f64, Surface<f64>); 4] = [
        (
            "major",
            inside,
            drifted(&move |s| {
                if let Surface::Torus { major_radius, .. } = s {
                    *major_radius += inside;
                }
            }),
        ),
        (
            "minor",
            inside,
            drifted(&move |s| {
                if let Surface::Torus { minor_radius, .. } = s {
                    *minor_radius += inside;
                }
            }),
        ),
        (
            "center",
            inside,
            drifted(&move |s| {
                if let Surface::Torus { center, .. } = s {
                    *center = Point3::new(center.x, center.y + inside, center.z);
                }
            }),
        ),
        (
            "tilt",
            0.0,
            drifted(&move |s| {
                if let Surface::Torus { axis, .. } = s {
                    *axis = Vec3::new(0.5 * eps / ARM, 1.0, 0.0).normalize();
                }
            }),
        ),
    ];
    for (what, floor, surface) in cases {
        let c = certify(wall(1.0, 0.0), &surface, band())
            .unwrap_or_else(|e| panic!("{what}: a drift inside the band certifies, got {e:?}"));
        let env = c.certificate().envelope;
        assert_eq!(
            c.certificate().statement,
            EnvelopeStatement::SpiricIdentity,
            "{what}"
        );
        // `0.99·floor` because the price is arithmetic on the drift,
        // not the drift itself: a `5e-10` shift of the chart's centre
        // reads back as `4.999999997368221e-10` through a norm.
        assert!(
            env > 0.0 && env >= floor * 0.99,
            "{what}: the chart's drift is priced, got {env:e}"
        );
        assert!(
            env <= eps,
            "{what}: and the price is affordable, got {env:e}"
        );
    }

    // Definite drift: refused at check 1 rather than priced.
    for (what, surface) in [
        (
            "major",
            drifted(&move |s| {
                if let Surface::Torus { major_radius, .. } = s {
                    *major_radius += definite;
                }
            }),
        ),
        (
            "center",
            drifted(&move |s| {
                if let Surface::Torus { center, .. } = s {
                    *center = Point3::new(center.x, center.y + definite, center.z);
                }
            }),
        ),
    ] {
        let e = certify(wall(1.0, 0.0), &surface, band()).unwrap_err_or_else_msg(what);
        assert!(
            matches!(e, PcurveCertifyError::UnsupportedCarrier),
            "{what}: {e:?}"
        );
    }
}

trait UnwrapErrMsg<E> {
    fn unwrap_err_or_else_msg(self, what: &str) -> E;
}

impl<T, E> UnwrapErrMsg<E> for Result<T, E> {
    fn unwrap_err_or_else_msg(self, what: &str) -> E {
        match self {
            Ok(_) => panic!("{what}: a definite chart drift must refuse"),
            Err(e) => e,
        }
    }
}

/// **The cap's exactness term is load-bearing.** Check 4's cap form is
/// `|k₀| + |k₁ − m|·f_max + |k₁|·f_drift + |k₂|`, and every image a
/// mint produces has `k₁ = m` bit-exactly — so the middle term is
/// zero on every reachable input and a mutant that drops it stays
/// green on every body-level row. This row hands the door a cap image
/// whose `f` coefficient is scaled, which is the only thing that term
/// can see.
#[test]
fn a_cap_image_with_the_wrong_f_coefficient_reds_on_the_exactness_term() {
    // The term is `|Δk₁|·f_max` with `f_max` near `R + r`, so a scale
    // error of `50·ε/(R + r)` buys a displacement of `50·ε` — over the
    // band at every ε cell CI gates.
    let eps = tol().eps();
    let e = certify(cap(1.0 + 50.0 * eps / ARM), &cap_plane(), band())
        .expect_err("a wrong f coefficient is a real displacement");
    assert!(
        matches!(
            e,
            PcurveCertifyError::ResidualExceeded { .. } | PcurveCertifyError::Escalated { .. }
        ),
        "the exactness term refuses, got {e:?}"
    );
    // And a scale small enough to afford certifies with the term's own
    // number in the envelope rather than a zero.
    let small = 0.1 * eps / ARM;
    let c = certify(cap(1.0 + small), &cap_plane(), band())
        .expect("a tiny scale error is priced, not refused");
    let env = c.certificate().envelope;
    assert!(
        env >= small * (R - RR),
        "the |k1 - m|·f_max term is in the envelope: {env:e}"
    );
}

/// **The banded gate's design, at a band 100× wider.** Deviation 1
/// replaced the spec's bit-equal C6 compare with a banded one because
/// the C6 read needs `T: Bounds` at a `T: Decide` door. What makes
/// that an improvement rather than a loosening is that check 4 prices
/// whatever check 1 admits — so widening check 1 does NOT widen what
/// certifies. This row widens it by two orders and measures that the
/// door still refuses.
#[test]
fn a_hundredfold_structural_band_still_refuses_at_the_envelope() {
    let eps = tol().eps();
    let wide_band = Band::linear_at(tol(), 100.0 * eps).expect("a 100x band");
    // `major` off by `50·ε`: definite at the shipped band, Zero at
    // this one — so check 1 admits it here and only check 4 can refuse.
    let mut image = wall(1.0, 0.0);
    if let Pcurve::Spiric { ref mut major, .. } = image {
        *major += 50.0 * eps;
    }
    let e = certify(image, &torus(), wide_band)
        .expect_err("check 4 prices what the wider check 1 admitted");
    assert!(
        matches!(
            e,
            PcurveCertifyError::ResidualExceeded { .. } | PcurveCertifyError::Escalated { .. }
        ),
        "the envelope is what refuses at a wide band, got {e:?}"
    );
}

/// **The wall's `v0` is a whole-period constant, and this is the row
/// that pins the value `topo`'s polar branch shift must produce.**
/// `shift_polar_branch`'s wall arm answers `v0 + k·period`; a mutant
/// that drops the `k·period` is silent on every body-level row,
/// because **no body on this tree walks a spiric loop across the `v`
/// cut** — a spiric rim's parameter span is the revolved PROFILE
/// arc's, and the one spiric-bearing body's rims span 1.78 rad, so the
/// walk's `k` is 0 everywhere. Said rather than worked around: the
/// value the arm owes is pinned here, at the door.
#[test]
fn a_wall_image_certifies_at_a_whole_period_shift_and_not_at_a_half_one() {
    let period = core::f64::consts::TAU;
    for k in [-2.0, -1.0, 1.0, 2.0] {
        let c = certify(wall(1.0, k * period), &torus(), band())
            .unwrap_or_else(|e| panic!("v0 = {k}·tau must certify, got {e:?}"));
        assert_eq!(c.certificate().statement, EnvelopeStatement::SpiricIdentity);
        assert_eq!(c.certificate().envelope, 0.0, "k = {k}");
        assert!(c.certificate().max_residual <= 1e-14, "k = {k}");
    }
    // Any shift that is not a whole period is a different locus.
    let e = certify(wall(1.0, core::f64::consts::PI), &torus(), band())
        .expect_err("a half-period shift is a moved image");
    assert!(
        matches!(e, PcurveCertifyError::ResidualExceeded { .. }),
        "the schedule sees a half-turn at every sample, got {e:?}"
    );
}

/// **The chart roster, at the door.** A spiric lies on its own cutting
/// plane and its own torus and on no other analytic chart, and the
/// mint says so per kind rather than falling through one refusal.
#[test]
fn the_mint_refuses_every_chart_a_spiric_does_not_lie_on() {
    let c = carrier();
    for (what, surface) in [
        (
            "cylinder",
            Surface::Cylinder {
                origin: Point3::origin(),
                axis: Vec3::unit_y(),
                radius: R,
                u_ref: Vec3::unit_x(),
            },
        ),
        (
            "cone",
            Surface::Cone {
                apex: Point3::origin(),
                axis: Vec3::unit_y(),
                half_angle: 0.5,
                u_ref: Vec3::unit_x(),
            },
        ),
        (
            "sphere",
            Surface::Sphere {
                center: Point3::origin(),
                radius: R,
                axis: Vec3::unit_y(),
                u_ref: Vec3::unit_x(),
            },
        ),
    ] {
        let e = geom_brep::chart_pcurve(&c, &surface, band()).unwrap_err_or_else_msg(what);
        assert!(
            matches!(e, PcurveCertifyError::UnsupportedCarrier),
            "{what}: {e:?}"
        );
    }
    // And a chart whose axis is perpendicular to the carrier's is not
    // the torus this spiric sections: the sign the wall rides has no
    // answer there, so the mint refuses rather than picking one.
    let perpendicular = Surface::Torus {
        center: Point3::origin(),
        axis: Vec3::unit_z(),
        major_radius: R,
        minor_radius: RR,
        u_ref: Vec3::unit_x(),
    };
    let e = geom_brep::chart_pcurve(&c, &perpendicular, band())
        .unwrap_err_or_else_msg("perpendicular torus");
    assert!(matches!(e, PcurveCertifyError::UnsupportedCarrier), "{e:?}");
}
