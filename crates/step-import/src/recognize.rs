//! D7 stage-1 NURBS surface recognition (ruling #256): every imported
//! NURBS surface is tested for promotion to an analytic kind, and
//! promotion fires iff the per-kind residual CERTIFIES at ε_in —
//! verified, never trusted (a file's `.PLANE_SURF.` annotation is not
//! consulted; the certificate is the only authority). A patch that
//! certifies nowhere stays NURBS silently — the M7-3-legal state:
//! recognition failing must never refuse a face that imports today.
//!
//! Promotion runs on the WHOLE patch, at the face, before
//! `normalize_shell`'s C⁰-knot split: a promoted carrier has no knots,
//! so the split never cuts it, and a foreign rational conic arrives as
//! one analytic face rather than one face per span.
//!
//! # Kind scope
//!
//! Plane, cylinder, cone, sphere and torus — every analytic kind with an
//! implicit form. The spindle sheet (a torus with `R ≤ r`) is not
//! proposed: such a patch stays NURBS.
//!
//! # Certification (D4 ¶2 shapes, ε_in budget) — whole-patch, always
//!
//! NO promotion without a whole-patch certificate (a bare sampled grid is
//! not one), and every kind reads the same one:
//! [`geom_brep::patch_distance_sup`], an upper bound on the distance of
//! every point of the patch from the candidate's locus. The candidate's
//! canonical implicit form is composed with the patch as a
//! tensor-product rational Bernstein form in interval arithmetic, per
//! knot-span cell; the coefficient hull bounds the form on the cell, and
//! the chart's `|f|`-to-metres conversion turns it into a distance using
//! the cell's control box. With strictly positive weights the rational
//! basis is a nonnegative partition of unity, so the hull holds every
//! point of the cell, and the bound is the upper end of an enclosure. On
//! exact geometry the form vanishes identically, so its coefficients are
//! rounding-width and the bound is too: an exact plane or conic certifies
//! at fine ε, and a bulge between any samples needs bulged coefficients,
//! which the hull reads.
//!
//! A cheap refutation runs before each curved certificate: the
//! candidate's `f64` implicit residual — the linearized distance — at a
//! fixed 3×3 grid per knot-span cell ([`refuted_at_samples`]). It can only
//! refuse, so it never makes promotion incorrect, only incomplete where a
//! sample's linearized residual reads past the band and the true distance
//! does not; it keeps a freeform patch from paying for four algebraic
//! certificates.
//!
//! # Estimators (D9-clean: closed form, fixed evaluation order)
//!
//! - **Plane**: translate-to-origin Newell over the control net's
//!   boundary ring in index order (the `newell_plane` method, run on
//!   the net rather than a loop so the fit is certification-free
//!   here — certification is the residual bound above), oriented to
//!   the NURBS chart normal at the domain midpoint, `u_ref` the chart's
//!   `∂u` there made perpendicular to the normal. Every
//!   step is a fixed-order sum or a closed-form map of one — no
//!   iteration, no data-dependent branching beyond the orientation
//!   sign.
//! - **Cylinder**: axis as the fixed-order sum of the net's v-column
//!   translations (an extruded patch's columns are translates along
//!   its axis); three fixed-parameter samples of the `v = v0` boundary
//!   projected ⊥ axis; the exact circumcenter solve for center and
//!   radius; `u_ref` toward the first sample (the patch's u-start
//!   azimuth — the seam, so a full-period patch's seam generator
//!   certifies as the seam image).
//! - **Cone, sphere, torus** read the patch's curvature at fixed domain
//!   fractions ([`curvature_at`]: the first and second fundamental forms
//!   and the closed-form principal curvatures). A point plus its normal
//!   over a principal curvature is that curvature's centre, and each
//!   kind's centres lie on a locus it names: the **sphere**'s at its
//!   centre (one sample, the domain midpoint); the **torus**'s
//!   larger-magnitude centre on its core circle (three diagonal samples,
//!   whose circumcircle is the core: centre, axis and `R`; `r` the mean
//!   of the three radii of curvature); the **cone**'s larger-magnitude
//!   centre on its axis (two diagonal samples give the axis; the samples'
//!   heights and axial distances give the slope `tan α` and the apex,
//!   the axis oriented so the patch is on the opening nappe). The seam
//!   `u_ref` sits at the patch's start azimuth ([`seam_reference`]); a
//!   sphere's axis runs through the patch's own pole where its net has a
//!   collapsed side, so the promoted chart is singular where the NURBS
//!   chart was.
//!
//! Each estimator only PROPOSES; the certificate above decides. A
//! wrong estimate fails certification and the patch stays NURBS — an
//! estimator can therefore never make promotion incorrect, only
//! incomplete. (A torus with `R < 2r` whose diagonal samples reach the
//! inner equator, where the parallel curvature outgrows the meridian's,
//! proposes a wrong core and stays NURBS.)
//!
//! Every DERIVED frame here (origin/normal/axis/u_ref) goes through
//! [`crate::signed_zero`], for the reason that module states — the same
//! posture [`crate::recognize_curve`] takes for curves — and the
//! certificate reads the surface as stored, after that pass.
//!
//! # Conditioning (D7's typed ambiguity, its only stage-1 site)
//!
//! The cylinder estimator carries its own margin trilean: a patch
//! whose azimuth samples are collinear within ε_in determines no
//! finite radius (the near-flat patch), and one whose v-columns sum
//! below ε_in determines no axis. Either is
//! [`Recognition::IllConditioned`] — the estimator cannot answer at
//! the interpretation budget — unless a later kind certifies. The caller escalates that to the typed
//! `RecognitionAmbiguous` refusal ONLY where promotion was needed to
//! import at all (the multi-bound curved-face gate); an importable
//! face with an ill-conditioned estimator stays NURBS, exactly like a
//! refuted one. The cone, sphere and torus estimators carry no trilean:
//! a degenerate curvature reading proposes nothing.

use geom::{NurbsSurface, Surface};
use geom_core::{Band, LengthOutOfFormat, NoDirection, Point3, RigidMotion, Vec3};

use crate::interpret::{FileBand, Read, STEP_RECOGNIZED_CONDITIONING, STEP_RECOGNIZED_RESIDUAL};
use crate::signed_zero::{plus_zero, plus_zero_point};

pub(crate) use crate::PromotedKind;

/// The outcome of testing one NURBS patch for promotion.
#[derive(Debug)]
pub(crate) enum Recognition {
    /// A kind certified at ε_in.
    Promoted {
        /// The analytic surface. A plane's normal is aligned to the
        /// NURBS chart normal; every curved kind's chart normal points
        /// out of its locus by construction, whatever the patch's
        /// orientation — the caller composes [`chart_flipped`] into the
        /// face's `same_sense` so orientation survives the promotion.
        surface: Surface<f64>,
        /// The certified residual sup (meters).
        residual: f64,
        /// Which kind certified.
        kind: PromotedKind,
    },
    /// No implemented kind certifies — the patch stays NURBS (the
    /// M7-3-legal state, silent).
    StaysNurbs,
    /// An estimator could not answer at ε_in (its own margin trilean).
    /// Note (R2 NOTE-1, contract-consistent): a NEAR-plane sitting
    /// just past the plane budget lands here rather than in
    /// [`Recognition::StaysNurbs`] — its azimuth samples are collinear
    /// within ε_in, which IS the documented near-flat ambiguity class.
    /// Both outcomes are non-promoting; only the refusing-class
    /// escalation differs (the honest D4 answer there).
    IllConditioned {
        /// The kind whose estimator declined.
        kind: PromotedKind,
        /// The margin that fell inside ε_in (meters).
        margin: f64,
    },
}

/// Tests `patch` for promotion at the file's band (module docs: kinds,
/// estimators, certificates). The kind is a lane choice: it promotes
/// where its certified residual reads Zero at the band, and stays NURBS
/// in band and apart; an estimator's conditioning that reads Zero is its
/// refusal.
///
/// Selection is the fixed kind-preference order **Plane > Cylinder >
/// Cone > Sphere > Torus**. A patch that certifies as two kinds is
/// canonicalization, not ambiguity: both analytic surfaces agree with the
/// patch — hence with each other — within 2·ε_in everywhere on it, so
/// either answer is a correct reading of the file and the fixed order
/// merely picks the canonical one (D7's typed ambiguity is reserved for
/// the estimator's own conditioning, where no answer exists at the
/// budget). A cylinder estimator that declines does not stop the later
/// kinds: its refusal is the answer only where none of them certifies.
pub(crate) fn recognize(patch: &NurbsSurface<f64>, file: FileBand) -> Recognition {
    if let Some((surface, residual)) = try_plane(patch, file) {
        return Recognition::Promoted {
            surface,
            residual,
            kind: PromotedKind::Plane,
        };
    }
    // The cylinder estimator reads its axis off the net's `v` columns, so
    // it runs on the patch and then on its transpose: an extrusion along
    // `u` is the same cylinder, and the kind must not depend on which
    // parameter the file ran along the axis.
    let first = try_cylinder(patch, file);
    let cylinder = if matches!(first, Ok(Some(_))) {
        first
    } else {
        match try_cylinder(&patch.transposed(), file) {
            found @ Ok(Some(_)) => found,
            _ => first,
        }
    };
    if let Ok(Some((surface, residual))) = cylinder {
        return Recognition::Promoted {
            surface,
            residual,
            kind: PromotedKind::Cylinder,
        };
    }
    let curved: [(PromotedKind, Estimator); 3] = [
        (PromotedKind::Cone, estimate_cone),
        (PromotedKind::Sphere, estimate_sphere),
        (PromotedKind::Torus, estimate_torus),
    ];
    for (kind, estimate) in curved {
        if let Some((surface, residual)) =
            estimate(patch, file).and_then(|candidate| certify_curved(patch, candidate, file))
        {
            return Recognition::Promoted {
                surface,
                residual,
                kind,
            };
        }
    }
    match cylinder {
        Err(margin) => Recognition::IllConditioned {
            kind: PromotedKind::Cylinder,
            margin,
        },
        Ok(_) => Recognition::StaysNurbs,
    }
}

/// A curved kind's estimator: the candidate it proposes at the file's
/// band, if any.
type Estimator = fn(&NurbsSurface<f64>, FileBand) -> Option<Surface<f64>>;

/// Whether `p` lies inside the session box (D4 ¶4): a carrier whose
/// defining point lies outside it is the degenerate limit of a simpler
/// kind, not a reading of an in-box model.
fn in_session_box(p: Point3<f64>) -> bool {
    let e = geom_core::tolerance::SESSION_BOX_EXTENT;
    p.x.abs() <= e && p.y.abs() <= e && p.z.abs() <= e
}

/// The curved-kind certificate (module docs): `candidate` and its
/// certified residual where [`geom_brep::patch_distance_sup`] reads Zero at the
/// band, after the sampled refutation has not refused it.
fn certify_curved(
    patch: &NurbsSurface<f64>,
    candidate: Surface<f64>,
    file: FileBand,
) -> Option<(Surface<f64>, f64)> {
    if refuted_at_samples(patch, &candidate, file) {
        return None;
    }
    let residual = geom_brep::patch_distance_sup(patch, &candidate);
    (file.read(STEP_RECOGNIZED_RESIDUAL, residual) == Read::Zero).then_some((candidate, residual))
}

/// Whether some point of a fixed 3×3 grid per knot-span cell (the cell's
/// corners, edge midpoints and centre) has an `f64` implicit residual
/// from `candidate` — the linearized distance, not a bound on it — that
/// does not read Zero at the band. A refutation, never a certificate: it
/// only declines to run the certificate. The schedule depends on the
/// knot vectors alone (D9). Poison refutes.
fn refuted_at_samples(patch: &NurbsSurface<f64>, candidate: &Surface<f64>, file: FileBand) -> bool {
    let bu: Vec<f64> = patch.knot_values_u().breakpoints().collect();
    let bv: Vec<f64> = patch.knot_values_v().breakpoints().collect();
    for cu in bu.windows(2) {
        let (ua, ub) = (cu[0], cu[1]);
        for cv in bv.windows(2) {
            let (va, vb) = (cv[0], cv[1]);
            let Some(win) = patch.window_at(geom::mid_param(ua, ub), geom::mid_param(va, vb))
            else {
                return true;
            };
            for i in 0..3 {
                let u = ua + (ub - ua) * f64::from(i) / 2.0;
                for j in 0..3 {
                    let v = va + (vb - va) * f64::from(j) / 2.0;
                    let r = geom_brep::implicit_residual(candidate, win.eval_in_span(u, v));
                    if file.read(STEP_RECOGNIZED_RESIDUAL, r) != Read::Zero {
                        return true;
                    }
                }
            }
        }
    }
    false
}

/// The NURBS chart's first derivatives at the domain midpoint: their
/// cross product is the orientation reference every promoted chart is
/// aligned to, so the face's `same_sense` keeps its meaning across the
/// promotion.
fn patch_jet_mid(patch: &NurbsSurface<f64>) -> geom::SurfaceJet<f64> {
    let (u0, u1) = patch.knots_u_f64_interim().domain();
    let (v0, v1) = patch.knots_v_f64_interim().domain();
    patch.ders(geom::mid_param(u0, u1), geom::mid_param(v0, v1))
}

/// `v` scaled by an exact power of two so its largest component lies in
/// `[1, 2)`: the same direction, every component kept bit for bit except
/// one more than 2⁻¹⁰²² below the largest, so a sign or a length read of
/// it cannot leave the format. `v` itself where no component is a finite
/// nonzero number.
fn in_range(v: Vec3<f64>) -> Vec3<f64> {
    let m = v.norm_inf();
    if !m.is_finite() || m == 0.0 {
        return v;
    }
    // `m`'s binary exponent, `⌊log₂ m⌋`, read off its bits (a subnormal's
    // from its leading one).
    let bits = m.to_bits();
    let biased = i32::try_from(bits >> 52).unwrap_or(0);
    let e = if biased == 0 {
        -1011 - i32::try_from(bits.leading_zeros()).unwrap_or(0)
    } else {
        biased - 1023
    };
    // Two exact factors, since `2^−e` itself can leave the format.
    let half = -e / 2;
    v * geom_core::pow2(half) * geom_core::pow2(-e - half)
}

/// The plane candidate: Newell over the control net's boundary ring,
/// certified by the whole-patch certificate (module docs).
fn try_plane(patch: &NurbsSurface<f64>, file: FileBand) -> Option<(Surface<f64>, f64)> {
    let (nu, nv) = patch.control_counts();
    let control = patch.control();
    // Centroid over the whole net (fixed left-to-right order).
    let n = control.len() as f64;
    let mut sum = Vec3::zero();
    for p in control {
        sum = sum + (*p - Point3::origin());
    }
    let origin = Point3::origin() + sum / n;
    // Newell cross-sum over the net's boundary ring: row v=0 forward,
    // column u=nu−1 up, row v=nv−1 backward, column u=0 down — one
    // fixed cycle in index order. (For a 2×2 net this is the quad's
    // own cycle.)
    let at = |iu: usize, iv: usize| control[iu * nv + iv];
    let mut ring: Vec<Point3<f64>> = Vec::with_capacity(2 * (nu + nv));
    for iu in 0..nu {
        ring.push(at(iu, 0));
    }
    for iv in 1..nv {
        ring.push(at(nu - 1, iv));
    }
    for iu in (0..nu - 1).rev() {
        ring.push(at(iu, nv - 1));
    }
    for iv in (1..nv - 1).rev() {
        ring.push(at(0, iv));
    }
    let mut normal_sum = Vec3::zero();
    for (i, p) in ring.iter().enumerate() {
        let next = ring[(i + 1) % ring.len()];
        normal_sum = normal_sum + (*p - origin).cross(next - origin);
    }
    // Orient to the chart normal (module docs). Every vector read here
    // is first brought into range exactly ([`in_range`]): the Newell sum
    // carries the square of the patch's scale and the chart normal the
    // product of its two derivatives, either of which can leave the
    // format where the patch itself does not. A net normal orthogonal to
    // the chart's, or no number, determines no oriented plane: refuted
    // here, stays NURBS.
    let chart = patch_jet_mid(patch);
    let du = in_range(chart.du);
    let chart_normal = in_range(du.cross(in_range(chart.dv)));
    let normal_sum = in_range(normal_sum);
    let align = normal_sum.dot(chart_normal);
    if !(align.is_finite() && align != 0.0) {
        return None;
    }
    let normal_sum = if align < 0.0 { -normal_sum } else { normal_sum };
    // The plane's frame, its normal the oriented sum and its roll the
    // input's: a promoted plane's `u_ref` is kept, so it is the patch's
    // own `∂u` at the domain midpoint, the tangent the chart normal was
    // crossed from, and the plane's chart runs the patch's way round.
    // Both are an estimator's choices, which the certificate below
    // decides, so their lengths are read at the least band; in range,
    // neither can be out of the format, so only a zero sum (or a `∂u`
    // along the normal) determines no frame.
    let frame = RigidMotion::from_axis_and_reference(
        origin,
        normal_sum,
        du,
        crate::interpret::STEP_RECOGNIZED_PLANE_FRAME,
        Band::least(),
    )
    .ok()?;
    let normal = frame.w().get();
    // The flush is posture no test guards (`crate::signed_zero`).
    let plane = Surface::Plane {
        origin: plus_zero_point(origin),
        normal: plus_zero(normal),
        u_ref: plus_zero(frame.u().get()),
    };
    // The whole-patch certificate every kind reads
    // ([`geom_brep::patch_distance_sup`]): on a plane the composite is
    // the signed height itself, metres already.
    let residual = geom_brep::patch_distance_sup(patch, &plane);
    (file.read(STEP_RECOGNIZED_RESIDUAL, residual) == Read::Zero).then_some((plane, residual))
}

/// The cylinder candidate (module docs). `Err(margin)` is the
/// estimator's own conditioning refusal; `Ok(None)` a refuted
/// certificate.
#[allow(clippy::type_complexity)]
fn try_cylinder(
    patch: &NurbsSurface<f64>,
    file: FileBand,
) -> Result<Option<(Surface<f64>, f64)>, f64> {
    let (nu, nv) = patch.control_counts();
    let control = patch.control();
    let at = |iu: usize, iv: usize| control[iu * nv + iv];
    // Axis: the fixed-order sum of each u-row's v-span translation.
    // For an extruded patch every row's `last − first` is the same
    // vector (length × axis); summing is the D9-clean mean.
    let mut axis_sum = Vec3::zero();
    for iu in 0..nu {
        axis_sum = axis_sum + (at(iu, nv - 1) - at(iu, 0));
    }
    // An underflowed or zero axis is the conditioning refusal below,
    // at its mean span read through the prescale.
    let (axis, axis_norm) = match axis_sum.checked_direction() {
        Ok(pair) => pair,
        Err(NoDirection::OutOfFormat(LengthOutOfFormat::NonFinite)) => return Ok(None),
        Err(_) => return Err(axis_sum.direction_and_length().1 / (nu as f64)),
    };
    // Divided by nu it is the mean v-span — a patch length. Where it
    // reads Zero at the file's band, no axis is determined.
    if file.read(STEP_RECOGNIZED_CONDITIONING, axis_norm / (nu as f64)) == Read::Zero {
        return Err(axis_norm / (nu as f64));
    }
    // Three fixed samples of the v-start boundary at u-fractions
    // 0, 3/8, 6/8 of the domain — distinct azimuths even on a
    // full-period patch (where u-start and u-end coincide).
    let (u0, u1) = patch.knots_u_f64_interim().domain();
    let (v0, _) = patch.knots_v_f64_interim().domain();
    let sample = |f: f64| patch.eval(u0 + (u1 - u0) * f, v0);
    let p0 = sample(0.0);
    let pa = sample(3.0 / 8.0);
    let pb = sample(6.0 / 8.0);
    // Project ⊥ axis into the plane through p0.
    let plane = geom::Line {
        origin: p0,
        dir: axis,
    };
    let proj = |p: Point3<f64>| plane.square_foot(p);
    let (q0, qa, qb) = (proj(p0), proj(pa), proj(pb));
    // Conditioning margin: the sagitta of the middle sample over the
    // chord — collinear-at-ε_in samples determine no finite radius
    // (the near-flat patch; D7's ambiguity class).
    let chord = qb - q0;
    let Some((chord_dir, _)) = chord.checked_direction().ok() else {
        return Ok(None);
    };
    let off = geom::Line {
        origin: q0,
        dir: chord_dir,
    }
    .offset(qa);
    let sagitta = off.norm();
    if !sagitta.is_finite() {
        return Ok(None);
    }
    if file.read(STEP_RECOGNIZED_CONDITIONING, sagitta) == Read::Zero {
        return Err(sagitta);
    }
    // Exact circumcenter of (q0, qa, qb) in their common plane ⊥
    // axis: the standard closed form
    // `c = q0 + (|b|²·(a×b)×a + |a|²·b×(a×b)) / (2·|a×b|²)` with
    // `a = qa − q0`, `b = qb − q0`.
    let a = qa - q0;
    let b = qb - q0;
    let axb = a.cross(b);
    let denom = 2.0 * axb.norm_squared();
    let center = q0 + (axb.cross(a) * b.norm_squared() + b.cross(axb) * a.norm_squared()) / denom;
    let radial = q0 - center;
    let Some((radial_hat, radius)) = radial.checked_direction().ok() else {
        return Ok(None);
    };
    let cylinder = Surface::Cylinder {
        origin: plus_zero_point(center),
        axis: plus_zero(axis),
        radius,
        u_ref: plus_zero(radial_hat),
    };
    // Orientation: the cylinder chart's normal is radially outward by
    // construction, whatever the patch's chart orientation — the
    // caller compares chart normals and composes `same_sense`.
    Ok(certify_curved(patch, cylinder, file))
}

/// The parameters at the domain fractions `(fu, fv)`.
fn at_fraction(patch: &NurbsSurface<f64>, fu: f64, fv: f64) -> (f64, f64) {
    let (u0, u1) = patch.knot_values_u().domain();
    let (v0, v1) = patch.knot_values_v().domain();
    (u0 + (u1 - u0) * fu, v0 + (v1 - v0) * fv)
}

/// One curvature sample: the point, the unit chart normal `n`, and the
/// two principal curvatures with the larger magnitude first, signed so
/// that `point + n/κ` is that curvature's centre.
#[derive(Clone, Copy, Debug)]
struct CurvatureSample {
    point: Point3<f64>,
    normal: Vec3<f64>,
    larger: f64,
    smaller: f64,
}

impl CurvatureSample {
    /// The centre of the larger-magnitude principal curvature.
    fn centre(self) -> Point3<f64> {
        self.point + self.normal / self.larger
    }
}

/// The curvature sample at domain fractions `(fu, fv)`: with `E, F, G`
/// and `L, M, N` the fundamental forms, the principal curvatures are
/// `H ± √(H² − K)`, `H = (EN + GL − 2FM)/(2(EG − F²))`,
/// `K = (LN − M²)/(EG − F²)` — closed form, no iteration. `None` where
/// the chart is singular there or a reading is not finite.
fn curvature_at(patch: &NurbsSurface<f64>, fu: f64, fv: f64) -> Option<CurvatureSample> {
    let (u, v) = at_fraction(patch, fu, fv);
    let jet = patch.ders(u, v);
    let (normal, _) = jet.du.cross(jet.dv).checked_direction().ok()?;
    let (e, f, g) = (jet.du.dot(jet.du), jet.du.dot(jet.dv), jet.dv.dot(jet.dv));
    let (l, m, n) = (
        jet.duu.dot(normal),
        jet.duv.dot(normal),
        jet.dvv.dot(normal),
    );
    let det = e * g - f.powi(2);
    if det.is_nan() || det <= 0.0 {
        return None;
    }
    let h = (e * n + g * l - 2.0 * f * m) / (2.0 * det);
    let k = (l * n - m.powi(2)) / det;
    let root = (h.powi(2) - k).max(0.0).sqrt();
    let (a, b) = (h + root, h - root);
    let (larger, smaller) = if a.abs() >= b.abs() { (a, b) } else { (b, a) };
    (larger.is_finite() && smaller.is_finite() && larger != 0.0).then_some(CurvatureSample {
        point: jet.point,
        normal,
        larger,
        smaller,
    })
}

/// The diagonal fractions the cone and torus estimators sample: distinct
/// in both parameters, so the samples sit at distinct azimuths and
/// heights whichever parameter is the azimuth.
const DIAGONAL: [f64; 3] = [1.0 / 8.0, 1.0 / 2.0, 7.0 / 8.0];

/// The unit direction of `p − centre` with its component along `axis`
/// removed.
fn radial_of(p: Point3<f64>, centre: Point3<f64>, axis: Vec3<f64>) -> Option<Vec3<f64>> {
    let d = p - centre;
    (d - axis * d.dot(axis))
        .checked_direction()
        .ok()
        .map(|(r, _)| r)
}

/// The seam reference for a carrier of revolution about `axis` through
/// `centre`: the radial direction of the patch's start in its azimuthal
/// parameter — `S(u0, v_mid)` where `u` runs more nearly around the axis
/// at the domain midpoint, `S(u_mid, v0)` otherwise — so a full-period
/// patch's seam is its own start boundary, as the cylinder's is.
fn seam_reference(
    patch: &NurbsSurface<f64>,
    centre: Point3<f64>,
    axis: Vec3<f64>,
) -> Option<Vec3<f64>> {
    let (um, vm) = at_fraction(patch, 0.5, 0.5);
    let jet = patch.ders(um, vm);
    let tangential = axis.cross(radial_of(jet.point, centre, axis)?);
    let along = |d: Vec3<f64>| {
        d.checked_direction()
            .ok()
            .map_or(0.0, |(t, _)| t.dot(tangential).abs())
    };
    let (u0, v0) = at_fraction(patch, 0.0, 0.0);
    let start = if along(jet.du) >= along(jet.dv) {
        patch.eval(u0, vm)
    } else {
        patch.eval(um, v0)
    };
    radial_of(start, centre, axis)
}

/// The cone candidate (module docs): the larger principal curvature's
/// centre lies on the axis, so two diagonal samples give it; their
/// heights `h` and axial distances `ρ` along it give `tan α` as the slope
/// `Δρ/Δh` and the apex where `ρ` reaches zero, the axis oriented so the
/// slope is positive (the patch on the opening nappe).
fn estimate_cone(patch: &NurbsSurface<f64>, file: FileBand) -> Option<Surface<f64>> {
    let s0 = curvature_at(patch, DIAGONAL[0], DIAGONAL[0])?;
    let s2 = curvature_at(patch, DIAGONAL[2], DIAGONAL[2])?;
    let base = s0.centre();
    let (mut axis, _) = (s2.centre() - base).checked_direction().ok()?;
    let read = |p: Point3<f64>, axis: Vec3<f64>| {
        let d = p - base;
        let h = d.dot(axis);
        (h, (d - axis * h).norm())
    };
    let (h0, rho0) = read(s0.point, axis);
    let (h2, rho2) = read(s2.point, axis);
    let (mut h0, mut slope) = (h0, (rho2 - rho0) / (h2 - h0));
    if slope < 0.0 {
        axis = -axis;
        (h0, slope) = (-h0, -slope);
    }
    // A radius that does not change across the samples at the band is a
    // cylinder's, whose apex runs off to infinity: no cone.
    if !(slope.is_finite() && slope > 0.0)
        || file.read(STEP_RECOGNIZED_CONDITIONING, rho2 - rho0) == Read::Zero
    {
        return None;
    }
    let apex = base + axis * (h0 - rho0 / slope);
    if !in_session_box(apex) {
        return None;
    }
    let axis = plus_zero(axis);
    let apex = plus_zero_point(apex);
    let u_ref = plus_zero(seam_reference(patch, apex, axis)?);
    Some(Surface::Cone {
        apex,
        axis,
        half_angle: slope.atan(),
        u_ref,
    })
}

/// The side of the net that collapses to one point — every control point
/// of a boundary row or column bitwise equal — as that point and whether
/// it is a `u` side (a row of fixed `iu`), in the fixed order `u` start,
/// `u` end, `v` start, `v` end.
fn collapsed_side(patch: &NurbsSurface<f64>) -> Option<(Point3<f64>, bool)> {
    let (nu, nv) = patch.control_counts();
    let control = patch.control();
    let at = |iu: usize, iv: usize| control[iu * nv + iv];
    let bits = |p: Point3<f64>| p.to_array().map(f64::to_bits);
    let side = |count: usize, point: &dyn Fn(usize) -> Point3<f64>| {
        let first = point(0);
        (1..count)
            .all(|k| bits(point(k)) == bits(first))
            .then_some(first)
    };
    [
        (side(nv, &|iv| at(0, iv)), true),
        (side(nv, &|iv| at(nu - 1, iv)), true),
        (side(nu, &|iu| at(iu, 0)), false),
        (side(nu, &|iu| at(iu, nv - 1)), false),
    ]
    .into_iter()
    .find_map(|(p, u_side)| p.map(|p| (p, u_side)))
}

/// The sphere candidate (module docs): the midpoint's centre of mean
/// curvature, the radius its distance from the midpoint. The axis runs
/// through the patch's collapsed side where it has one (the NURBS chart's
/// own pole), else along `∂S/∂u` at the midpoint, which puts the
/// midpoint on the equator and the poles a quarter turn off it; the seam
/// at the patch's start in the parameter that runs around that axis.
fn estimate_sphere(patch: &NurbsSurface<f64>, _file: FileBand) -> Option<Surface<f64>> {
    let s = curvature_at(patch, 0.5, 0.5)?;
    let mean = 0.5 * (s.larger + s.smaller);
    if !(mean.is_finite() && mean != 0.0) {
        return None;
    }
    let center = plus_zero_point(s.point + s.normal / mean);
    // A vanishing mean curvature puts the centre past the session box:
    // the flat limit is a plane's, not a sphere's.
    if !in_session_box(center) {
        return None;
    }
    let radius = (s.point - center).norm();
    let (um, vm) = at_fraction(patch, 0.5, 0.5);
    let (u0, v0) = at_fraction(patch, 0.0, 0.0);
    let (axis, seam) = match collapsed_side(patch) {
        Some((pole, u_side)) => {
            let (axis, _) = (pole - center).checked_direction().ok()?;
            (
                axis,
                if u_side {
                    patch.eval(um, v0)
                } else {
                    patch.eval(u0, vm)
                },
            )
        }
        None => {
            let (axis, _) = patch.ders(um, vm).du.checked_direction().ok()?;
            (axis, patch.eval(um, v0))
        }
    };
    let axis = plus_zero(axis);
    let u_ref = plus_zero(radial_of(seam, center, axis)?);
    (radius.is_finite() && radius > 0.0).then_some(Surface::Sphere {
        center,
        radius,
        axis,
        u_ref,
    })
}

/// The torus candidate (module docs): the larger principal curvature's
/// centres at the three diagonal samples lie on the core circle, whose
/// exact circumcircle solve gives the centre, the axis (the samples'
/// winding) and `R`; `r` is the mean radius of curvature. A ring torus
/// only: `R > r > 0`.
fn estimate_torus(patch: &NurbsSurface<f64>, file: FileBand) -> Option<Surface<f64>> {
    let s = [
        curvature_at(patch, DIAGONAL[0], DIAGONAL[0])?,
        curvature_at(patch, DIAGONAL[1], DIAGONAL[1])?,
        curvature_at(patch, DIAGONAL[2], DIAGONAL[2])?,
    ];
    let c = s.map(CurvatureSample::centre);
    // A core whose three points are collinear at the band is a straight
    // spine — a cylinder's limit, `R` running off to infinity: no torus.
    let sagitta = geom::Line {
        origin: c[0],
        dir: c[2] - c[0],
    }
    .distance(c[1]);
    if !sagitta.is_finite() || file.read(STEP_RECOGNIZED_CONDITIONING, sagitta) == Read::Zero {
        return None;
    }
    let a = c[1] - c[0];
    let b = c[2] - c[0];
    let axb = a.cross(b);
    let center = c[0]
        + (axb.cross(a) * b.norm_squared() + b.cross(axb) * a.norm_squared())
            / (2.0 * axb.norm_squared());
    let (axis, _) = axb.checked_direction().ok()?;
    let center = plus_zero_point(center);
    let axis = plus_zero(axis);
    let major_radius = (c[0] - center).norm();
    let minor_radius =
        (1.0 / s[0].larger.abs() + 1.0 / s[1].larger.abs() + 1.0 / s[2].larger.abs()) / 3.0;
    if !(major_radius.is_finite()
        && minor_radius > 0.0
        && major_radius > minor_radius
        && in_session_box(center))
    {
        return None;
    }
    let u_ref = plus_zero(seam_reference(patch, center, axis)?);
    Some(Surface::Torus {
        center,
        axis,
        major_radius,
        minor_radius,
        u_ref,
    })
}

/// Whether the promoted chart's normal OPPOSES the NURBS chart's at
/// the domain midpoint — the `same_sense` composition bit. Computed
/// against the promoted surface's own chart normal via the implicit
/// gradient (plane: the stored normal; the curved kinds: out of their
/// locus, on either cone nappe), which points along the chart normal for
/// every promoted kind.
pub(crate) fn chart_flipped(patch: &NurbsSurface<f64>, promoted: &Surface<f64>) -> bool {
    let jet = patch_jet_mid(patch);
    let nurbs_normal = jet.du.cross(jet.dv);
    let grad = geom_brep::implicit_gradient(promoted, jet.point);
    nurbs_normal.dot(grad) < 0.0
}

/// **Adversarial probes**: hostile patches fed to the recognizer to
/// falsify the certification claims by execution — a patch exact at
/// every sample and bulging between them, near-boundary nets, exact
/// conics, and the selection order.
#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic)]
mod review_probes {
    use super::*;
    use geom_core::spline::KnotVector;

    const EPS_IN: f64 = 1e-9;

    /// A piecewise-quadratic C0 spline in u (double interior knots →
    /// per-span Bézier, endpoints interpolated), degree 1 in v: base
    /// plane z = 0 on [0,8]×[0,1], with per-span mid control points
    /// lifted to `bulge` (alternating sign so the net centroid stays
    /// on z = 0). The surface passes EXACTLY through z = 0 at every
    /// integer u — which is exactly the 9-point certification grid on
    /// this domain — and reaches z = bulge/2 at span midpoints.
    fn bulged_plane(bulge: f64, weights_value: f64) -> NurbsSurface<f64> {
        let mut knots = vec![0.0, 0.0, 0.0];
        for k in 1..8 {
            knots.push(k as f64);
            knots.push(k as f64);
        }
        knots.extend([8.0, 8.0, 8.0]);
        let ku = KnotVector::clamped(knots, 2).unwrap();
        let kv = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
        let mut control = Vec::new();
        for i in 0..17 {
            let u = i as f64 / 2.0;
            let z = if i % 2 == 1 {
                if (i / 2) % 2 == 0 { bulge } else { -bulge }
            } else {
                0.0
            };
            control.push(Point3::new(u, 0.0, z));
            control.push(Point3::new(u, 1.0, z));
        }
        NurbsSurface::new(ku, kv, control, vec![weights_value; 34]).unwrap()
    }

    /// **P1 — the missing between-samples envelope, plane/rational
    /// track.** Uniform weights 2.0 route the polynomial geometry
    /// onto the sampled track; the patch lies on z = 0 at every
    /// certification sample but bulges to ±0.25 between them.
    #[test]
    fn p1_rational_plane_bulge_between_samples() {
        let patch = bulged_plane(0.5, 2.0);
        let z_mid = patch.eval(0.5, 0.5).z.abs();
        assert!(z_mid > 0.2, "the bulge is real: {z_mid}");
        match recognize(&patch, crate::interpret::FileBand::test(EPS_IN)) {
            Recognition::Promoted { residual, kind, .. } => panic!(
                "PROMOTED a patch that deviates {z_mid} from the plane \
                 (kind {kind:?}, certified residual {residual:e}) — the fixed \
                 9x9 grid missed every bulge; no between-samples envelope"
            ),
            other => println!("stayed unpromoted: {other:?}"),
        }
    }

    /// **P1b — same construction, non-rational.** The hull sup-bound
    /// track must catch it (control points deviate by `bulge`).
    #[test]
    fn p1b_nonrational_plane_bulge_is_caught_by_the_hull() {
        let patch = bulged_plane(0.5, 1.0);
        match recognize(&patch, crate::interpret::FileBand::test(EPS_IN)) {
            Recognition::Promoted { kind, residual, .. } => {
                panic!("hull track promoted the bulged net: {kind:?} at {residual:e}")
            }
            other => println!("hull track refused, as designed: {other:?}"),
        }
    }

    /// **P2 — a bulge between samples, cylinder track (non-rational).**
    /// Integer-u samples lie EXACTLY on the unit cylinder (a 9×9 grid
    /// and the estimator's three azimuth samples); span-mid control
    /// points pushed radially bulge ~0.15 off the cylinder between
    /// samples, so a certificate that reads samples alone promotes a
    /// non-cylinder silently.
    #[test]
    fn p2_cylinder_bulge_between_samples() {
        let mut knots = vec![0.0, 0.0, 0.0];
        for k in 1..8 {
            knots.push(k as f64);
            knots.push(k as f64);
        }
        knots.extend([8.0, 8.0, 8.0]);
        let ku = KnotVector::clamped(knots, 2).unwrap();
        let kv = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
        let step = core::f64::consts::FRAC_PI_2 / 8.0;
        let mut control = Vec::new();
        for i in 0..17 {
            let theta = (i as f64 / 2.0) * step;
            let r = if i % 2 == 1 { 1.3 } else { 1.0 };
            let (x, y) = (r * theta.cos(), r * theta.sin());
            control.push(Point3::new(x, y, 0.0));
            control.push(Point3::new(x, y, 1.0));
        }
        let patch = NurbsSurface::new(ku, kv, control, vec![1.0; 34]).unwrap();
        let p = patch.eval(0.5, 0.5);
        let dev = ((p.x * p.x + p.y * p.y).sqrt() - 1.0).abs();
        assert!(dev > 0.05, "the bulge is real: {dev}");
        match recognize(&patch, crate::interpret::FileBand::test(EPS_IN)) {
            Recognition::Promoted { kind, residual, .. } => panic!(
                "PROMOTED a bulged non-cylinder (true radial deviation {dev}) as \
                 {kind:?} with certified residual {residual:e} — the fixed 9x9 \
                 grid sampled only the exact-cylinder points"
            ),
            other => println!("stayed unpromoted: {other:?}"),
        }
    }

    /// **P3 — the ε_in boundary on the hull track.** One control
    /// point of an otherwise exact plane lifted by δ; the fitted
    /// plane's residual is 5δ/6 on this 6-point net.
    #[test]
    fn p3_near_plane_boundary_behavior() {
        let patch_with = |delta: f64| {
            let ku = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap();
            let kv = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
            let control = vec![
                Point3::new(0.0, 0.0, 0.0),
                Point3::new(0.0, 1.0, 0.0),
                Point3::new(0.5, 0.0, delta),
                Point3::new(0.5, 1.0, 0.0),
                Point3::new(1.0, 0.0, 0.0),
                Point3::new(1.0, 1.0, 0.0),
            ];
            NurbsSurface::new(ku, kv, control, vec![1.0; 6]).unwrap()
        };
        match recognize(
            &patch_with(0.5 * EPS_IN),
            crate::interpret::FileBand::test(EPS_IN),
        ) {
            Recognition::Promoted { kind, residual, .. } => {
                assert_eq!(kind, PromotedKind::Plane);
                assert!(residual <= EPS_IN, "truthful residual: {residual:e}");
            }
            other => panic!("0.5×ε_in must promote: {other:?}"),
        }
        match recognize(
            &patch_with(2.0 * EPS_IN),
            crate::interpret::FileBand::test(EPS_IN),
        ) {
            Recognition::Promoted { kind, residual, .. } => {
                panic!("2×ε_in promoted as {kind:?} at {residual:e}")
            }
            other => println!("2×ε_in stayed unpromoted: {other:?}"),
        }
        match recognize(
            &patch_with(1.2 * EPS_IN),
            crate::interpret::FileBand::test(EPS_IN),
        ) {
            Recognition::Promoted { residual, .. } => {
                println!("boundary case promoted at residual {residual:e}")
            }
            other => println!("boundary case stayed unpromoted: {other:?}"),
        }
    }

    /// **P4 — a rational EXACT plane still promotes.** (Originally
    /// "via the sampled track"; the M-1 fix collapsed the plane
    /// certificate onto the hull sup-bound for BOTH tracks — the
    /// positive-weight convexity argument — so this now pins that the
    /// rational plane rides the whole-patch hull certificate.)
    #[test]
    fn p4_rational_exact_plane_promotes_via_the_hull_track() {
        let patch = bulged_plane(0.0, 3.0);
        match recognize(&patch, crate::interpret::FileBand::test(EPS_IN)) {
            Recognition::Promoted { kind, residual, .. } => {
                assert_eq!(kind, PromotedKind::Plane);
                assert!(residual <= 1e-15, "exact plane: {residual:e}");
            }
            other => panic!("the rational exact plane must promote: {other:?}"),
        }
    }

    /// **P5 — a genuinely freeform patch stays NURBS, and D9: two
    /// runs are bit-identical.**
    #[test]
    fn p5_freeform_stays_nurbs_and_recognition_is_deterministic() {
        let ku = KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.5, 1.0, 1.0, 1.0], 2).unwrap();
        let kv = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap();
        let mut control = Vec::new();
        for i in 0..4 {
            for j in 0..3 {
                let (x, y) = (i as f64, j as f64);
                control.push(Point3::new(x, y, (1.3 * x + 0.7 * y).sin()));
            }
        }
        let patch = NurbsSurface::new(ku, kv, control, vec![1.0; 12]).unwrap();
        match recognize(&patch, crate::interpret::FileBand::test(EPS_IN)) {
            Recognition::StaysNurbs | Recognition::IllConditioned { .. } => {}
            Recognition::Promoted { kind, residual, .. } => {
                panic!("freeform patch promoted as {kind:?} at {residual:e}")
            }
        }
        let promoting = bulged_plane(0.0, 2.0);
        let (a, b) = (
            recognize(&promoting, crate::interpret::FileBand::test(EPS_IN)),
            recognize(&promoting, crate::interpret::FileBand::test(EPS_IN)),
        );
        match (a, b) {
            (
                Recognition::Promoted {
                    surface: sa,
                    residual: ra,
                    ..
                },
                Recognition::Promoted {
                    surface: sb,
                    residual: rb,
                    ..
                },
            ) => {
                assert_eq!(ra.to_bits(), rb.to_bits(), "residual bits");
                assert_eq!(
                    format!("{sa:?}"),
                    format!("{sb:?}"),
                    "surface bits (debug repr)"
                );
            }
            other => panic!("both runs must promote identically: {other:?}"),
        }
    }

    /// **P6 — Plane wins the selection order on an exactly planar
    /// patch** (the D-c2 preference's plane-first arm where only the
    /// plane certificate holds; P10 is the same order exercised where
    /// BOTH hold).
    #[test]
    fn p6_plane_wins_the_selection_order() {
        let patch = bulged_plane(0.0, 1.0);
        match recognize(&patch, crate::interpret::FileBand::test(EPS_IN)) {
            Recognition::Promoted { kind, .. } => assert_eq!(kind, PromotedKind::Plane),
            other => panic!("the exact plane promotes: {other:?}"),
        }
    }

    /// A unit-radius rational quarter cylinder: `u` runs the arc CCW
    /// from `(1,0)` toward `(0,1)`, `v` runs `+z` — so the chart
    /// normal `∂u × ∂v = tangential × ẑ` points radially OUTWARD,
    /// agreeing with [`Surface::Cylinder`]'s chart.
    fn unit_quarter_cylinder() -> NurbsSurface<f64> {
        let w = core::f64::consts::FRAC_1_SQRT_2;
        let ku = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap();
        let kv = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
        let control = vec![
            Point3::new(1.0, 0.0, 0.0),
            Point3::new(1.0, 0.0, 1.0),
            Point3::new(1.0, 1.0, 0.0),
            Point3::new(1.0, 1.0, 1.0),
            Point3::new(0.0, 1.0, 0.0),
            Point3::new(0.0, 1.0, 1.0),
        ];
        NurbsSurface::new(ku, kv, control, vec![1.0, 1.0, w, w, 1.0, 1.0]).unwrap()
    }

    /// The [`unit_quarter_cylinder`]'s locus as the kernel's analytic
    /// cylinder (chart normal radially outward by construction).
    fn unit_cylinder() -> Surface<f64> {
        Surface::Cylinder {
            origin: Point3::new(0.0, 0.0, 0.0),
            axis: Vec3::new(0.0, 0.0, 1.0),
            radius: 1.0,
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        }
    }

    /// **P7 — an EXACT cylinder certifies at a fine budget.** The unit
    /// quarter cylinder's implicit form vanishes identically, so the
    /// composite's coefficients are rounding-width and so is the
    /// certificate: it promotes at ε_in = 1e-9 with a residual many
    /// orders below it, and the certificate it reads is the one
    /// [`geom_brep::patch_distance_sup`] gives the stored carrier.
    #[test]
    fn p7_an_exact_cylinder_certifies_at_a_fine_budget() {
        let patch = unit_quarter_cylinder();
        let direct = geom_brep::patch_distance_sup(&patch, &unit_cylinder());
        assert!(
            direct < 1e-14,
            "the exact cylinder's certificate: {direct:e}"
        );
        match recognize(&patch, crate::interpret::FileBand::test(EPS_IN)) {
            Recognition::Promoted {
                kind,
                residual,
                surface,
            } => {
                assert_eq!(kind, PromotedKind::Cylinder);
                assert!(residual < 1e-14, "the certified residual: {residual:e}");
                assert_eq!(
                    residual.to_bits(),
                    geom_brep::patch_distance_sup(&patch, &surface).to_bits(),
                    "the residual is the stored carrier's certificate"
                );
            }
            other => panic!("an exact cylinder promotes at 1e-9: {other:?}"),
        }
    }

    /// **P8 (R2 MIN-1) — [`chart_flipped`] pinned DIRECTLY, both
    /// outcomes.** The sense-compose arm is reached only by a
    /// promoted CYLINDER whose chart runs inward — promoted planes
    /// are pre-aligned by `try_plane` and can never flip — so at the
    /// import budgets a metre-scale part is read at it is not reached
    /// at all, and a stubbed-`false` mutation survives the whole
    /// suite unless this row pins it directly. Constructed inputs, no
    /// import, so the pin does not depend on that budget ratio (P9 is
    /// the promoting path that reaches this arm through `recognize`):
    ///
    /// * the quarter cylinder as authored (u along the arc, v along
    ///   `+z`) has an OUTWARD chart normal → not flipped;
    /// * the same patch [`NurbsSurface::transposed`] swaps `∂u` and
    ///   `∂v`, negating the chart normal (inward) → flipped;
    /// * a promoted plane is never flipped, in either patch
    ///   orientation, BECAUSE `try_plane` aligns the stored normal to
    ///   the patch's own chart — pinned by promoting a plane patch
    ///   and its transpose and asserting the two stored normals are
    ///   opposite while `chart_flipped` answers false for both.
    #[test]
    fn p8_chart_flipped_pinned_directly() {
        let patch = unit_quarter_cylinder();
        let cylinder = unit_cylinder();
        assert!(
            !chart_flipped(&patch, &cylinder),
            "outward-chart patch agrees with the cylinder's outward normal"
        );
        assert!(
            chart_flipped(&patch.transposed(), &cylinder),
            "the transposed patch's chart normal points inward — the flip arm"
        );

        let plane_patch = bulged_plane(0.0, 1.0);
        let promoted_normal =
            |p: &NurbsSurface<f64>| match recognize(p, crate::interpret::FileBand::test(EPS_IN)) {
                Recognition::Promoted { surface, .. } => {
                    assert!(
                        !chart_flipped(p, &surface),
                        "a promoted plane is pre-aligned: never flipped"
                    );
                    match surface {
                        Surface::Plane { normal, .. } => normal,
                        other => panic!("a plane patch promotes to a plane: {other:?}"),
                    }
                }
                other => panic!("the exact plane promotes: {other:?}"),
            };
        let n_fwd = promoted_normal(&plane_patch);
        let n_rev = promoted_normal(&plane_patch.transposed());
        assert!(
            n_fwd.dot(n_rev) < -0.99,
            "the alignment is real: opposite patch orientations store opposite \
             normals ({n_fwd:?} vs {n_rev:?})"
        );
    }
    /// A unit-radius, `h`-tall FULL-PERIOD rational cylinder: four
    /// 90-degree quadratic Bézier spans (double interior knots), `u`
    /// CCW from `(1, 0)`, `v` along `+z`. Exact — every surface point
    /// is at radius 1.
    fn unit_full_cylinder(h: f64) -> NurbsSurface<f64> {
        let ku = KnotVector::clamped(
            vec![0.0, 0.0, 0.0, 1.0, 1.0, 2.0, 2.0, 3.0, 3.0, 4.0, 4.0, 4.0],
            2,
        )
        .unwrap();
        let kv = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
        let mut control = Vec::new();
        let mut weights = Vec::new();
        for i in 0..9 {
            let theta = core::f64::consts::FRAC_PI_4 * i as f64;
            let (radius, w) = if i % 2 == 1 {
                (core::f64::consts::SQRT_2, core::f64::consts::FRAC_1_SQRT_2)
            } else {
                (1.0, 1.0)
            };
            let (x, y) = (radius * theta.cos(), radius * theta.sin());
            control.push(Point3::new(x, y, 0.0));
            control.push(Point3::new(x, y, h));
            weights.push(w);
            weights.push(w);
        }
        NurbsSurface::new(ku, kv, control, weights).unwrap()
    }

    /// **P9 — the cylinder arm executes at every budget an exact
    /// cylinder meets**, on [`unit_full_cylinder`]: a coarse one, where the
    /// plane certificate still refuses so the selection reaches the
    /// cylinder, and a fine one, where only the cylinder could certify.
    #[test]
    fn p9_the_cylinder_promoting_arm_executes() {
        let patch = unit_full_cylinder(1.0);
        // The fixture is an EXACT cylinder.
        for (i, j) in [(0, 0), (1, 3), (2, 5), (3, 7)] {
            let p = patch.eval(f64::from(i) + 0.375, f64::from(j) / 8.0);
            let rho = (p.x * p.x + p.y * p.y).sqrt();
            assert!((rho - 1.0).abs() < 1e-14, "exact locus: rho = {rho}");
        }
        assert!(
            try_plane(&patch, crate::interpret::FileBand::test(0.9)).is_none(),
            "the plane certificate must refuse, so the selection reaches try_cylinder"
        );
        match recognize(&patch, crate::interpret::FileBand::test(0.9)) {
            Recognition::Promoted {
                surface,
                residual,
                kind,
            } => {
                assert_eq!(kind, PromotedKind::Cylinder);
                assert!(residual <= 0.9, "truthful residual: {residual:e}");
                match surface {
                    Surface::Cylinder { radius, axis, .. } => {
                        assert!((radius - 1.0).abs() < 1e-12, "radius {radius}");
                        assert!(axis.dot(Vec3::new(0.0, 0.0, 1.0)).abs() > 1.0 - 1e-12);
                    }
                    other => panic!("the cylinder arm promotes to a cylinder: {other:?}"),
                }
            }
            other => panic!("the cylinder arm must fire at eps_in = 0.9: {other:?}"),
        }
        match recognize(&patch, crate::interpret::FileBand::test(EPS_IN)) {
            Recognition::Promoted { kind, residual, .. } => {
                assert_eq!(kind, PromotedKind::Cylinder);
                assert!(residual < 1e-14, "the fine budget's residual: {residual:e}");
            }
            other => panic!("the exact cylinder promotes at 1e-9 too: {other:?}"),
        }
    }

    /// **P10 — the `Plane > Cylinder` order decides a patch that
    /// certifies as BOTH.** At ε_in = 0.99 m the same
    /// [`unit_full_cylinder`] certifies as a plane (0.969) and as a
    /// cylinder (rounding-scale); `recognize` returns the plane. P9 reads the
    /// same patch one budget finer, where only the cylinder holds —
    /// so the pair brackets the plane certificate's own boundary. This is the
    /// canonicalization `recognize`'s docs claim, executed rather
    /// than argued: both residuals are within ε_in, so the two
    /// analytic surfaces agree with the patch — hence with each other
    /// — within 2·ε_in on it, and the fixed order picks one.
    #[test]
    fn p10_a_double_certifying_patch_selects_plane() {
        let patch = unit_full_cylinder(1.0);
        let eps_in = 0.99;
        let Some((_, plane_residual)) = try_plane(&patch, crate::interpret::FileBand::test(eps_in))
        else {
            panic!("the plane certificate must hold at this budget")
        };
        let Ok(Some((_, cylinder_residual))) =
            try_cylinder(&patch, crate::interpret::FileBand::test(eps_in))
        else {
            panic!("the cylinder certificate must hold at this budget")
        };
        assert!(
            plane_residual <= eps_in && cylinder_residual <= eps_in,
            "both readings are correct at the budget: {plane_residual} / {cylinder_residual}"
        );
        match recognize(&patch, crate::interpret::FileBand::test(eps_in)) {
            Recognition::Promoted { kind, residual, .. } => {
                assert_eq!(kind, PromotedKind::Plane);
                assert_eq!(residual.to_bits(), plane_residual.to_bits());
            }
            other => panic!("a double-certifying patch promotes: {other:?}"),
        }
    }

    /// A quarter turn about `+y` of a meridian profile `(ρ, y)` — control
    /// points `profile` with weights `profile_w` at `degree` over one
    /// span — as Piegl–Tiller's rational surface of revolution: `u` the
    /// profile, `v` the quarter turn from `+x` toward `−z`. The profile
    /// point `index` is lifted by `lift` along `+y` (zero: exact).
    fn revolved(
        profile: &[(f64, f64)],
        profile_w: &[f64],
        degree: usize,
        (index, lift): (usize, f64),
    ) -> NurbsSurface<f64> {
        let mut knots = vec![0.0; degree + 1];
        knots.extend(vec![1.0; degree + 1]);
        let ku = KnotVector::clamped(knots, degree).unwrap();
        let kv = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap();
        let w = core::f64::consts::FRAC_1_SQRT_2;
        let turn = [((1.0, 0.0), 1.0), ((1.0, 1.0), w), ((0.0, 1.0), 1.0)];
        let (mut control, mut weights) = (Vec::new(), Vec::new());
        for (i, (&(rho, y), &wp)) in profile.iter().zip(profile_w).enumerate() {
            let y = if i == index { y + lift } else { y };
            for &((c, s), wt) in &turn {
                control.push(Point3::new(rho * c, y, -rho * s));
                weights.push(wp * wt);
            }
        }
        NurbsSurface::new(ku, kv, control, weights).unwrap()
    }

    /// The quarter of a minor circle of radius `r` about `(major, 0)`
    /// from the outer equator up, as a rational quadratic profile with
    /// semi-axes `(r, r·aspect)` — a circle at `aspect = 1`.
    fn tube_profile(major: f64, r: f64, aspect: f64) -> [(f64, f64); 3] {
        [
            (major + r, 0.0),
            (major + r, r * aspect),
            (major, r * aspect),
        ]
    }

    const QUARTER_W: [f64; 3] = [1.0, core::f64::consts::FRAC_1_SQRT_2, 1.0];

    /// The kind and residual `patch` promotes to at `eps_in`, or a panic
    /// naming `what`.
    fn promoted(patch: &NurbsSurface<f64>, eps_in: f64, what: &str) -> (Surface<f64>, f64) {
        match recognize(patch, crate::interpret::FileBand::test(eps_in)) {
            Recognition::Promoted {
                surface, residual, ..
            } => (surface, residual),
            other => panic!("{what} promotes at ε_in = {eps_in:e}: {other:?}"),
        }
    }

    /// **P11 — exact torus, sphere and cone patches promote to their
    /// kinds at 1e-9 and at 1e-12**, with the carrier the geometry is,
    /// and a residual that is the stored carrier's own certificate.
    #[test]
    fn p11_exact_conics_promote_to_their_kinds() {
        let torus = revolved(&tube_profile(3.0, 1.0, 1.0), &QUARTER_W, 2, (0, 0.0));
        let sphere = revolved(&tube_profile(0.0, 2.0, 1.0), &QUARTER_W, 2, (0, 0.0));
        let cone = revolved(&[(1.0, 0.0), (2.0, 1.0)], &[1.0, 1.0], 1, (0, 0.0));
        for eps_in in [1e-9, 1e-12] {
            let (surface, residual) = promoted(&torus, eps_in, "the exact torus");
            match surface {
                Surface::Torus {
                    center,
                    axis,
                    major_radius,
                    minor_radius,
                    ..
                } => {
                    assert!((major_radius - 3.0).abs() < 1e-12, "R = {major_radius}");
                    assert!((minor_radius - 1.0).abs() < 1e-12, "r = {minor_radius}");
                    assert!(axis.y.abs() > 1.0 - 1e-12, "axis {axis:?}");
                    assert!(
                        (center - Point3::origin()).norm() < 1e-12,
                        "centre {center:?}"
                    );
                }
                ref other => panic!("the torus promotes to a torus: {other:?}"),
            }
            assert_eq!(
                residual.to_bits(),
                geom_brep::patch_distance_sup(&torus, &surface).to_bits()
            );
            let (surface, _) = promoted(&sphere, eps_in, "the exact sphere octant");
            match surface {
                Surface::Sphere {
                    center,
                    radius,
                    axis,
                    ..
                } => {
                    assert!((radius - 2.0).abs() < 1e-12, "radius {radius}");
                    assert!(
                        (center - Point3::origin()).norm() < 1e-12,
                        "centre {center:?}"
                    );
                    // The octant's net collapses at its pole `(0, 2, 0)`.
                    assert!(
                        axis.y > 1.0 - 1e-12,
                        "the axis runs through the pole: {axis:?}"
                    );
                }
                ref other => panic!("the sphere promotes to a sphere: {other:?}"),
            }
            let (surface, _) = promoted(&cone, eps_in, "the exact cone frustum");
            match surface {
                Surface::Cone {
                    apex,
                    axis,
                    half_angle,
                    ..
                } => {
                    assert!(
                        (half_angle - core::f64::consts::FRAC_PI_4).abs() < 1e-12,
                        "half-angle {half_angle}"
                    );
                    assert!(
                        (apex - Point3::new(0.0, -1.0, 0.0)).norm() < 1e-12,
                        "apex {apex:?}"
                    );
                    assert!(
                        axis.y > 1.0 - 1e-12,
                        "the opening nappe holds the patch: {axis:?}"
                    );
                }
                ref other => panic!("the cone promotes to a cone: {other:?}"),
            }
        }
    }

    /// **P12 — a conic patch that is no analytic kind stays NURBS, and
    /// so does an exact one with one control point lifted between the
    /// samples' reach.** The elliptic tube (semi-axes 1 and 1.2) is a
    /// rational conic of revolution, a torus nowhere; each exact kind's
    /// net with its profile's middle control lifted 1e-6 m (the cone's a
    /// straight generator raised to degree 2) bulges off its locus by a
    /// fraction of that, past ε_in = 1e-9.
    #[test]
    fn p12_off_locus_conics_stay_nurbs() {
        let band = crate::interpret::FileBand::test(EPS_IN);
        let cases = [
            (
                "the elliptic tube",
                revolved(&tube_profile(3.0, 1.0, 1.2), &QUARTER_W, 2, (0, 0.0)),
            ),
            (
                "the lifted torus",
                revolved(&tube_profile(3.0, 1.0, 1.0), &QUARTER_W, 2, (1, 1e-6)),
            ),
            (
                "the lifted sphere",
                revolved(&tube_profile(0.0, 2.0, 1.0), &QUARTER_W, 2, (1, 1e-6)),
            ),
            (
                "the lifted cone",
                revolved(
                    &[(1.0, 0.0), (1.5, 0.5), (2.0, 1.0)],
                    &[1.0; 3],
                    2,
                    (1, 1e-6),
                ),
            ),
        ];
        for (what, patch) in cases {
            if let Recognition::Promoted { kind, residual, .. } = recognize(&patch, band) {
                panic!("{what} promoted as {kind:?} at {residual:e}");
            }
        }
    }

    /// **P14 — a cone patch that reaches its apex** promotes where the
    /// apex cell's root reading clears the band, and only there: the
    /// certificate reads the double cone's lesser bound, the generator
    /// lever's away from the apex and `√|f|` on the cell holding it, and on
    /// exact geometry `|f|` there is rounding-width, so the root is about
    /// `1e-8` at unit scale — under 1e-6, over 1e-9.
    #[test]
    fn p14_a_cone_reaching_its_apex_promotes_where_its_root_reads() {
        let cap = revolved(
            &[(0.0, 0.0), (0.5, 0.5), (1.0, 1.0)],
            &[1.0; 3],
            2,
            (0, 0.0),
        );
        match recognize(&cap, crate::interpret::FileBand::test(1e-6)) {
            Recognition::Promoted { kind, residual, .. } => {
                assert_eq!(kind, PromotedKind::Cone);
                assert!(residual > 1e-10, "the apex root reads: {residual:e}");
            }
            other => panic!("the apex cone promotes at 1e-6: {other:?}"),
        }
        assert!(
            !matches!(
                recognize(&cap, crate::interpret::FileBand::test(EPS_IN)),
                Recognition::Promoted { .. }
            ),
            "at 1e-9 the apex root does not clear the band"
        );
    }

    /// **P13 — the kind is a function of the geometry, not of the frame
    /// or the parameterization**: each exact kind's patch promotes to the
    /// same kind after a rigid motion (a turn of 0.7 rad about a skew axis
    /// and a translation) and with its parameters swapped. The cylinder
    /// row is the one whose estimator reads a parameter direction (the
    /// net's columns), so its transpose is the case that used to read a
    /// cylinder as a degenerate cone.
    #[test]
    fn p13_the_kind_survives_a_rigid_motion_and_a_transposition() {
        let (k, angle) = (Vec3::new(1.0, 2.0, 3.0) / 14.0_f64.sqrt(), 0.7_f64);
        let (s, c) = angle.sin_cos();
        let shift = Vec3::new(0.3, -1.2, 2.5);
        let moved = |p: Point3<f64>| {
            let v = p - Point3::origin();
            let turned = v * c + k.cross(v) * s + k * (k.dot(v) * (1.0 - c));
            Point3::origin() + turned + shift
        };
        let cases = [
            (PromotedKind::Cylinder, unit_full_cylinder(1.0)),
            (
                PromotedKind::Cone,
                revolved(&[(1.0, 0.0), (2.0, 1.0)], &[1.0, 1.0], 1, (0, 0.0)),
            ),
            (
                PromotedKind::Sphere,
                revolved(&tube_profile(0.0, 2.0, 1.0), &QUARTER_W, 2, (0, 0.0)),
            ),
            (
                PromotedKind::Torus,
                revolved(&tube_profile(3.0, 1.0, 1.0), &QUARTER_W, 2, (0, 0.0)),
            ),
        ];
        let band = crate::interpret::FileBand::test(EPS_IN);
        for (want, patch) in cases {
            for (how, p) in [
                ("as stated", patch.clone()),
                ("moved", patch.map_points(moved)),
                ("transposed", patch.transposed()),
                ("moved and transposed", patch.map_points(moved).transposed()),
            ] {
                match recognize(&p, band) {
                    Recognition::Promoted { kind, .. } => {
                        assert_eq!(kind, want, "{want:?} {how}");
                    }
                    other => panic!("{want:?} {how} promotes: {other:?}"),
                }
            }
        }
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic)]
mod plane_frame {
    use geom_core::spline::KnotVector;

    use super::*;

    /// A flat bilinear patch on `z = 0` through `corners` (in `u`-major
    /// order: `(u0,v0), (u0,v1), (u1,v0), (u1,v1)`), each corner mapped
    /// through `place`, over the knot domains `u` and `v`.
    fn flat_over(
        corners: [(f64, f64); 4],
        place: impl Fn(Point3<f64>) -> Point3<f64>,
        (u, v): (f64, f64),
    ) -> NurbsSurface<f64> {
        let k = |t: f64| KnotVector::clamped(vec![0.0, 0.0, t, t], 1).unwrap();
        let control = corners
            .iter()
            .map(|&(x, y)| place(Point3::new(x, y, 0.0)))
            .collect();
        NurbsSurface::new(k(u), k(v), control, vec![1.0; 4]).unwrap()
    }

    fn flat(
        corners: [(f64, f64); 4],
        place: impl Fn(Point3<f64>) -> Point3<f64>,
    ) -> NurbsSurface<f64> {
        flat_over(corners, place, (1.0, 1.0))
    }

    const SQUARE: [(f64, f64); 4] = [(0.0, 0.0), (0.0, 1.0), (1.0, 0.0), (1.0, 1.0)];

    fn promoted_plane(patch: &NurbsSurface<f64>) -> (Vec3<f64>, Vec3<f64>) {
        match try_plane(patch, crate::interpret::FileBand::test(1e-9)) {
            Some((Surface::Plane { normal, u_ref, .. }, _)) => (normal, u_ref),
            other => panic!("a flat patch promotes to a plane: {other:?}"),
        }
    }

    /// The kept frame is the input's (D10 `[placement.frame]`): the patch
    /// turned about its normal and then tilted off it carries its plane's
    /// normal and roll with it.
    #[test]
    fn a_promoted_planes_frame_turns_with_its_patch() {
        let (s, c) = 0.5f64.sin_cos();
        let (ts, tc) = 0.3f64.sin_cos();
        let turn = |v: Vec3<f64>| {
            let v = Vec3::new(c * v.x - s * v.y, s * v.x + c * v.y, v.z);
            Vec3::new(v.x, tc * v.y - ts * v.z, ts * v.y + tc * v.z)
        };
        let moved = |p: Point3<f64>| Point3::origin() + turn(p - Point3::origin());
        let (normal, u_ref) = promoted_plane(&flat(SQUARE, |p| p));
        let (turned_normal, turned_u_ref) = promoted_plane(&flat(SQUARE, moved));
        for (what, got, want) in [
            ("normal", turned_normal, turn(normal)),
            ("u_ref", turned_u_ref, turn(u_ref)),
        ] {
            assert!(
                (got - want).norm() < 1e-15,
                "the {what} is carried by the motion: {got:?} vs {want:?}"
            );
        }
    }

    /// A flat patch promotes at every scale the format holds it at, and
    /// over knot domains of any extent: the orientation and the frame are
    /// read of vectors brought into range exactly, so neither the Newell
    /// sum's square of the scale nor the chart's product of the two
    /// domains' reaches out of the format.
    #[test]
    fn a_flat_patch_promotes_at_the_formats_extremes() {
        for (scale, domains) in [
            (1e-82, (1.0, 1.0)),
            (1e78, (1.0, 1.0)),
            (1.0, (1e-160, 1e160)),
            (1e-6, (1e150, 1e150)),
        ] {
            let patch = flat_over(
                SQUARE,
                |p| Point3::new(p.x * scale, p.y * scale, p.z),
                domains,
            );
            let Some((Surface::Plane { normal, u_ref, .. }, _)) =
                try_plane(&patch, crate::interpret::FileBand::test(1e-9))
            else {
                panic!("a flat square at scale {scale:e} over {domains:?} promotes");
            };
            assert_eq!(
                [normal.x, normal.y, normal.z, u_ref.x, u_ref.y, u_ref.z],
                [0.0, 0.0, 1.0, 1.0, 0.0, 0.0],
                "scale {scale:e} over {domains:?}"
            );
        }
    }
}

/// Review probes (PR 7172 fix pass), not shipped rows.
#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic)]
mod zz_review_conditioning {
    use super::*;
    use geom_core::spline::KnotVector;

    /// A frustum of half-angle `alpha` about `+z`, its apex at the origin,
    /// over heights `[z0, z1]`, as a half turn (two quarter spans) in `v`
    /// and a line in `u`, then moved by `shift`.
    fn frustum(alpha: f64, z0: f64, z1: f64, shift: Vec3<f64>) -> NurbsSurface<f64> {
        let ku = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
        let kv = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 2.0, 2.0, 2.0], 2).unwrap();
        let w = core::f64::consts::FRAC_1_SQRT_2;
        let turn = [
            ((1.0, 0.0), 1.0),
            ((1.0, 1.0), w),
            ((0.0, 1.0), 1.0),
            ((-1.0, 1.0), w),
            ((-1.0, 0.0), 1.0),
        ];
        let t = alpha.tan();
        let (mut control, mut weights) = (Vec::new(), Vec::new());
        for z in [z0, z1] {
            let rho = z * t;
            for &((c, s), wt) in &turn {
                control.push(Point3::new(rho * c, rho * s, z) + shift);
                weights.push(wt);
            }
        }
        NurbsSurface::new(ku, kv, control, weights).unwrap()
    }

    fn outcome(p: &NurbsSurface<f64>, eps: f64) -> String {
        match recognize(p, crate::interpret::FileBand::test(eps)) {
            Recognition::Promoted { kind, residual, .. } => {
                format!("Promoted {kind:?} {residual:e}")
            }
            Recognition::StaysNurbs => "StaysNurbs".into(),
            Recognition::IllConditioned { kind, margin } => {
                format!("IllConditioned {kind:?} {margin:e}")
            }
        }
    }

    /// A draft-angle cone whose apex lies past the session box, every
    /// point of the patch well inside it; and the same in-box cone moved
    /// rigidly so its apex leaves the box while the patch stays in.
    #[test]
    fn review_shallow_cones_and_the_box() {
        // 0.05° draft, radius ~1 m at z ≈ 1146 m from the apex: the patch
        // spans z ∈ [1146, 1147] from the apex, so put the apex at
        // z = −1146 and the patch around z ∈ [0, 1].
        let alpha = 0.05_f64.to_radians();
        let z0 = 1.0 / alpha.tan();
        let p = frustum(alpha, z0, z0 + 1.0, Vec3::new(0.0, 0.0, -z0));
        for eps in [1e-6, 1e-9] {
            eprintln!(
                "shallow draft cone (apex at z=-{z0:.0}) @ {eps:e}: {}",
                outcome(&p, eps)
            );
        }
        // 0.1 rad cone, apex in box: 600 m from patch.
        let alpha = 0.1_f64;
        let z0 = 1.0 / alpha.tan();
        let base = frustum(alpha, z0, z0 + 1.0, Vec3::new(0.0, 0.0, -z0));
        for shift in [0.0, -500.0, -995.0, -999.0] {
            let p = base.map_points(|q| q + Vec3::new(0.0, 0.0, shift));
            eprintln!(
                "cone α=0.1, apex at z={:.1}, patch z∈[{:.1},{:.1}]: {}",
                -z0 + shift,
                shift,
                shift + 1.0,
                outcome(&p, 1e-9)
            );
        }
        let alpha = 0.002_f64;
        let z0 = 1.0 / alpha.tan();
        let base = frustum(alpha, z0, z0 + 1.0, Vec3::new(0.0, 0.0, -z0));
        for shift in [400.0, 0.0, -100.0] {
            let p = base.map_points(|q| q + Vec3::new(0.0, 0.0, shift));
            eprintln!(
                "cone α=0.002, apex at z={:.1}, patch z∈[{:.1},{:.1}]: {} / transposed {}",
                -z0 + shift,
                shift,
                shift + 1.0,
                outcome(&p, 1e-9),
                outcome(&p.transposed(), 1e-9)
            );
        }
    }

    /// The headline's conic torus wall: 9 conic spans each way, R = 24,
    /// r = 6, about +y; its certificate and recognition at 1e-9 / 1e-12.
    #[test]
    fn review_headline_torus_wall_at_fine_eps() {
        let arc = |a0: f64, a1: f64, n: usize| -> (Vec<((f64, f64), f64)>, Vec<f64>) {
            let mut pts = Vec::new();
            let mut knots = vec![0.0; 3];
            for k in 0..n {
                let (b0, b1) = (
                    a0 + (a1 - a0) * (k as f64) / (n as f64),
                    a0 + (a1 - a0) * ((k + 1) as f64) / (n as f64),
                );
                let half = 0.5 * (b1 - b0);
                let (bm, w) = (0.5 * (b0 + b1), half.cos());
                if k == 0 {
                    pts.push(((b0.cos(), b0.sin()), 1.0));
                }
                pts.push(((bm.cos() / w, bm.sin() / w), w));
                pts.push(((b1.cos(), b1.sin()), 1.0));
                if k + 1 < n {
                    knots.push((k + 1) as f64);
                    knots.push((k + 1) as f64);
                }
            }
            knots.extend([n as f64; 3]);
            (pts, knots)
        };
        let (big_r, r) = (24.0, 6.0);
        let (minor, ku) = arc(0.0, core::f64::consts::PI, 9);
        let (major, kv) = arc(0.0, core::f64::consts::FRAC_PI_2, 9);
        let (mut control, mut weights) = (Vec::new(), Vec::new());
        for &((cx, sx), wm) in &minor {
            for &((ca, sa), wa) in &major {
                let d = big_r + r * cx;
                control.push(Point3::new(d * ca, r * sx, -d * sa));
                weights.push(wm * wa);
            }
        }
        let patch = NurbsSurface::new(
            KnotVector::clamped(ku, 2).unwrap(),
            KnotVector::clamped(kv, 2).unwrap(),
            control,
            weights,
        )
        .unwrap();
        let exact = Surface::Torus {
            center: Point3::origin(),
            axis: Vec3::new(0.0, 1.0, 0.0),
            major_radius: big_r,
            minor_radius: r,
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        };
        eprintln!(
            "headline wall: certificate vs the exact torus {:e}",
            geom_brep::patch_distance_sup(&patch, &exact)
        );
        for eps in [1e-6, 1e-9, 1e-12] {
            let band = crate::interpret::FileBand::test(eps);
            let est = estimate_torus(&patch, band);
            let cert = est
                .as_ref()
                .map(|s| geom_brep::patch_distance_sup(&patch, s));
            let refuted = est.as_ref().map(|s| refuted_at_samples(&patch, s, band));
            eprintln!(
                "headline wall @ {eps:e}: {} | estimate cert {cert:?} refuted {refuted:?}",
                outcome(&patch, eps)
            );
        }
    }

    /// A closed freeform revolution (a non-conic profile, `v` the full
    /// turn) and its transpose: the refusal class each orientation reports.
    #[test]
    fn review_closed_freeform_refusal_under_transposition() {
        let ku = KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0], 3).unwrap();
        let mut kv = vec![0.0; 3];
        for t in 1..4 {
            kv.push(f64::from(t));
            kv.push(f64::from(t));
        }
        kv.extend([4.0; 3]);
        let kv = KnotVector::clamped(kv, 2).unwrap();
        let w = core::f64::consts::FRAC_1_SQRT_2;
        let dirs = [
            (1.0, 0.0, 1.0),
            (1.0, 1.0, w),
            (0.0, 1.0, 1.0),
            (-1.0, 1.0, w),
            (-1.0, 0.0, 1.0),
            (-1.0, -1.0, w),
            (0.0, -1.0, 1.0),
            (1.0, -1.0, w),
            (1.0, 0.0, 1.0),
        ];
        let profile = [(1.0, 0.0), (1.6, 0.4), (0.7, 0.8), (1.2, 1.3)];
        let (mut control, mut weights) = (Vec::new(), Vec::new());
        for &(rho, z) in &profile {
            for &(c, s, wt) in &dirs {
                control.push(Point3::new(rho * c, rho * s, z));
                weights.push(wt);
            }
        }
        let p = NurbsSurface::new(ku, kv, control, weights).unwrap();
        eprintln!(
            "closed vase: as stated {} / transposed {}",
            outcome(&p, 1e-6),
            outcome(&p.transposed(), 1e-6)
        );
    }
}
