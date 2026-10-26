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
    EnvelopeStatement, Pcurve, PcurveCache, PcurveCertifyError, SpiricImage, WallTorus,
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
    // `u0` is the chart azimuth of the cutting plane's normal: `n = +x`
    // is the chart's own `u_ref`, so it is zero here.
    wall_image(0.0, sense, WallTorus::Own, v0)
}

fn wall_image(u0: f64, sense: f64, torus: WallTorus, v0: f64) -> Pcurve<f64> {
    Pcurve::Spiric {
        major: R,
        minor: RR,
        offset: D,
        image: SpiricImage::Wall {
            u0,
            v0,
            sense,
            torus,
        },
    }
}

/// The carrier's torus reflected through the cutting plane: centre
/// `2D` along `n = +x`, the same axis and radii.
fn mirror_torus() -> Surface<f64> {
    Surface::Torus {
        center: Point3::new(2.0 * D, 0.0, 0.0),
        axis: Vec3::unit_y(),
        major_radius: R,
        minor_radius: RR,
        u_ref: Vec3::unit_x(),
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
    PcurveCache::certify(p, T0, T1, &carrier(), surface, b)
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
///   this gate admitted is priced. A window of `ε·(R + r)` (the
///   `over_lever` door) would refuse this input at check 1, 37× early.
/// - `η = 50·ε/(R + r)` is past the escalation threshold either way
///   and refuses at check 1: a sense that is not a unit sign is not
///   the carrier's image (`ImageMismatch`).
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
    // answer: `ImageMismatch` is check 1's refusal, which an
    // `over_lever` window would have given this input, 37× early.
    let e = certify(wall(1.0 + 0.8 * window, 0.0), &torus(), band())
        .expect_err("a drift this size is a displacement over eps");
    assert!(
        !matches!(e, PcurveCertifyError::ImageMismatch { .. }),
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
    assert!(
        matches!(
            e,
            PcurveCertifyError::ImageMismatch {
                image: geom_brep::PcurveKind::Spiric,
                ..
            }
        ),
        "{e:?}"
    );
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
/// band calls definite refuses `CarrierOffChart` — a non-circular
/// spiric oval lies on no torus but its own and its mirror's
/// ([`the_mirror_torus_holds_the_oval_and_is_uncovered`]).
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
    // A quarter of ε, not a half: the price a drift pays is not the
    // drift itself — a wrong `r` moves two channels and pays twice, and
    // a tilt pays `(R + r)·θ·π/2` — and a half-ε `r` drift lands its
    // price exactly on the band edge, where it escalates.
    let inside = 0.25 * eps;
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
                    *axis = Vec3::new(0.25 * eps / ARM, 1.0, 0.0).normalize();
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
            matches!(e, PcurveCertifyError::CarrierOffChart { .. }),
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

/// **The wall's `v0` lifted a whole period is the value `topo`'s
/// second-channel shift produces, and it is no row.** The shift of the
/// wall image by `k` periods along `v` is `v0 + k·period`, pinned here
/// at the door: no body on this tree walks a spiric loop across the `v`
/// cut (a spiric rim's parameter span is the revolved PROFILE arc's,
/// and the one spiric-bearing body's rims span 1.78 rad, so the walk's
/// `k` is 0 everywhere). That lift is the row with a deck element,
/// never a row of its own (C4: a row is its placed image), so
/// certification refuses it at the second channel's fidelity term; the
/// wall's identity alone meets it exactly.
#[test]
fn a_wall_image_lifted_a_whole_period_is_no_row_and_a_half_one_moves() {
    let period = core::f64::consts::TAU;
    for k in [-2, -1, 1, 2] {
        let lifted = wall(1.0, 0.0).translate(0.0, f64::from(k) * period);
        assert_eq!(
            format!("{lifted:?}"),
            format!("{:?}", wall(1.0, f64::from(k) * period)),
            "k = {k}: the lift is v0 + k·period"
        );
        let e = certify(lifted, &torus(), band()).expect_err("a lifted copy is no row");
        assert!(
            matches!(
                e,
                PcurveCertifyError::ResidualExceeded {
                    check: geom_brep::PcurveCheck::EnvelopeTerm(geom_brep::EnvelopeTerm::FidelityV),
                    ..
                }
            ),
            "k = {k}: refused at the second channel's term, got {e:?}"
        );
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
/// plane and the two tori that hold it (its own and that torus's mirror
/// through the plane) and on no other analytic chart, and the mint says
/// so per kind rather than falling through one refusal.
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
            matches!(e, PcurveCertifyError::CarrierOffChart { .. }),
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
    assert!(
        matches!(e, PcurveCertifyError::CarrierOffChart { .. }),
        "{e:?}"
    );
}

/// **The one foreign torus that holds the oval, imaged.** Reflecting
/// the carrier's torus through the cutting plane (centre moved `2·D`
/// along the plane normal) leaves the section unchanged, so the oval
/// lies on that MIRROR torus — measured here at every sample. About the
/// mirror the oval's radial part is `−n·d + m·f`, so its azimuth runs
/// from `−n` (chart azimuth `π`) the other way round:
/// `u = π − atan2(f, d)`, `v = t`. The mint images it so, and the
/// image certifies the same identity the own-torus wall does.
#[test]
fn the_mirror_torus_holds_the_oval_and_mints_its_wall_image() {
    let mirror = mirror_torus();
    let c = carrier();
    for k in 0..16 {
        let p = c.eval(T0 + (T1 - T0) * f64::from(k) / 15.0);
        // The mirror torus's implicit form, about (2D, 0, 0) and +y.
        let (x, y, z) = (p.x - 2.0 * D, p.y, p.z);
        let rho = x.hypot(z);
        let implicit = (rho - R).powi(2) + y * y - RR * RR;
        assert!(implicit.abs() < 1e-14, "on the mirror torus: {implicit:e}");
    }
    let minted = geom_brep::chart_pcurve(&c, &mirror, band()).expect("the mirror wall mints");
    let Pcurve::Spiric {
        image:
            SpiricImage::Wall {
                u0,
                v0,
                sense,
                torus,
            },
        ..
    } = minted
    else {
        panic!("a wall image: {minted:?}")
    };
    assert_eq!(
        (u0.abs(), v0, sense, torus),
        (core::f64::consts::PI, 0.0, 1.0, WallTorus::Mirror),
        "the azimuth of −n, the chart axis the carrier's, and the mirror named"
    );
    let row = certify(minted, &mirror, band()).expect("the mirror wall certifies");
    assert_eq!(
        row.certificate().statement,
        EnvelopeStatement::SpiricIdentity
    );
    assert_eq!(row.certificate().envelope, 0.0);
    assert!(
        row.certificate().max_residual <= 1e-15,
        "{:e}",
        row.certificate().max_residual
    );
}

/// **A wall image names one of the two tori, and is no image on the
/// other.** The image's tag names the torus the identity maps through,
/// so the own-torus image on the mirror and the mirror image
/// on the own torus are each the wrong image of a carrier the chart
/// holds (`ImageMismatch`, not off the chart); a torus that is neither
/// holds no oval of it (`CarrierOffChart`), whichever image it is
/// handed.
#[test]
fn a_wall_image_names_one_torus_and_is_refused_on_the_other() {
    let own = wall(1.0, 0.0);
    let mirrored = wall_image(core::f64::consts::PI, 1.0, WallTorus::Mirror, 0.0);
    for (what, image, surface) in [
        ("the own image on the mirror", own.clone(), mirror_torus()),
        (
            "the mirror image on the own torus",
            mirrored.clone(),
            torus(),
        ),
    ] {
        let e = certify(image, &surface, band()).unwrap_err_or_else_msg(what);
        assert!(
            matches!(
                e,
                PcurveCertifyError::ImageMismatch {
                    image: geom_brep::PcurveKind::Spiric,
                    ..
                }
            ),
            "{what}: {e:?}"
        );
    }
    let neither = Surface::Torus {
        center: Point3::new(4.0 * D, 0.0, 0.0),
        axis: Vec3::unit_y(),
        major_radius: R,
        minor_radius: RR,
        u_ref: Vec3::unit_x(),
    };
    for (what, image) in [("own", own), ("mirror", mirrored)] {
        let e = certify(image, &neither, band()).unwrap_err_or_else_msg(what);
        assert!(
            matches!(e, PcurveCertifyError::CarrierOffChart { .. }),
            "the {what} image on a third torus: {e:?}"
        );
    }
}

/// **A chart gate decided apart answers past an earlier in-band one**
/// (`geom_brep::ladder`'s rule on the wall's chart ladders). Each pose
/// puts one gate `2ε` off, inside the band, ahead of another `3·K·ε`
/// off, past it; the chart is then neither torus that holds the oval,
/// and refuses as off it, where a ladder returning on its first in-band
/// gate escalates:
///
/// - the own torus, its centre in band and its major radius apart;
/// - the mirror torus (centre apart from the own), its minor radius in
///   band and its axis tilted apart — the other torus's ladder;
/// - the mirror torus moved apart from the mirror, its major radius in
///   band — the other torus's ladder, answered by its last datum.
///
/// The control is the mirror torus with its major radius in band and
/// nothing else apart: no datum decides, so the in-band one refuses.
///
/// The poses are multiples of `Tol::witness`'s own ε and K, so the row
/// reads the same at every ε row by design. Every pose is read before
/// any assertion, so a regression names all of them.
#[test]
fn a_chart_gate_decided_apart_answers_past_an_in_band_one() {
    let (eps, k) = (tol().eps(), tol().k());
    let (inside, past) = (2.0 * eps, 3.0 * k * eps);
    let chart = |center: Point3<f64>, axis: Vec3<f64>, major: f64, minor: f64| Surface::Torus {
        center,
        axis,
        major_radius: major,
        minor_radius: minor,
        u_ref: Vec3::unit_x(),
    };
    let mirror = Point3::new(2.0 * D, 0.0, 0.0);
    let tilted = (Vec3::unit_y() + Vec3::unit_z() * (past / ARM)).normalize();
    let mut wrong = Vec::new();
    for (what, surface) in [
        (
            "own: centre in band, major apart",
            chart(Point3::new(0.0, 0.0, inside), Vec3::unit_y(), R + past, RR),
        ),
        (
            "mirror: minor in band, tilt apart",
            chart(mirror, tilted, R, RR + inside),
        ),
        (
            "mirror moved apart: major in band",
            chart(
                mirror + Vec3::unit_z() * past,
                Vec3::unit_y(),
                R + inside,
                RR,
            ),
        ),
    ] {
        let got = certify(wall(1.0, 0.0), &surface, band());
        let neither = matches!(
            &got,
            Err(PcurveCertifyError::CarrierOffChart { why, .. })
                if why.starts_with("the chart torus is neither the carrier's own nor its mirror")
        );
        if !neither {
            wrong.push(format!(
                "{what}: a definite gate answers neither torus: {got:?}"
            ));
        }
    }
    let control = chart(mirror, Vec3::unit_y(), R + inside, RR);
    let got = certify(wall(1.0, 0.0), &control, band());
    if !matches!(
        got,
        Err(PcurveCertifyError::Escalated {
            check: geom_brep::PcurveCheck::ImageIdentity,
            cause: geom_core::Indeterminate {
                predicate: Some("pcurve_spiric_chart_major"),
                ..
            },
            ..
        })
    ) {
        wrong.push(format!(
            "control: the mirror with only its major radius in band refuses on it: {got:?}"
        ));
    }
    assert!(wrong.is_empty(), "{wrong:#?}");
}
