//! **Iso-curve extraction** for tensor-product NURBS surfaces (M6-3,
//! the loft/sweep assembly's seam substrate).
//!
//! A *clamped* surface's boundary iso-curves are **rows of its own
//! control net, verbatim**: `S(0, v)` is the spline over `knots_v`
//! whose control points are the first u-row (`P[0·nv + j]`) with the
//! matching weights, and likewise for `u = 1` (last u-row) and
//! `v = 0` / `v = 1` (first/last v-columns) over `knots_u`. No
//! arithmetic is performed — extraction is a **copy**, so the
//! extracted curve is exact structure (C6), not an approximation, and
//! two walls that share a control row share the curve bit for bit.
//!
//! An INTERIOR iso-curve `u = u*` is the **de Boor collapse** of the
//! net at `u*` ([`interior_iso_u`]): the same curve in the same `v`
//! space, whose control polygon is a convex combination of the net's
//! u-rows rather than one of them. It is exact in ℝ and computed at
//! the caller's scalar, so at the interval scalar each control point
//! ENCLOSES the exact one — the collapse's rounding lives inside the
//! enclosure the caller certifies, never in a separate term. The
//! collapsed row's WEIGHTS must be structure a carrier can share, so
//! the collapse is offered only for a weight net constant along `u`
//! or along `v`; a net varying along both refuses typed.
//!
//! # Why this lives in `geom-brep` and not beside the payloads
//!
//! **Iso-curve extraction belongs to the EdgeDescription layer, not to
//! the evaluator layer**, and that is a placement rule rather than an
//! accident of which crate the types used to sit in. Extraction is the
//! step that turns one entity's data into *another entity's carrier*:
//! its whole purpose is to hand a curve to an edge, which is what this
//! layer is for. `geom` answers "what is this locus and what does it
//! evaluate to"; it does not know that a surface row is about to
//! become an edge's geometry, and giving it a door that produces
//! carriers would make the evaluator layer aware of the B-rep above
//! it.

use geom::NurbsCurve3;
use geom::NurbsSurface;
use geom_core::k_stats::decide;
use geom_core::spline::basis::basis_funs;
use geom_core::spline::{KnotVector, SpanLocate, SplineError};
use geom_core::{Band, Decide, Indeterminate, Margin, NO_DECLARATION_RECOURSE, Point3, Real, Sign};

/// A surface's net as the iso doors read it: both knot vectors and
/// the row-major control and weight slices. The doors index the net by
/// counts taken from the knots, so [`NetView::counts`] checks the
/// slices against those counts before any index is formed.
struct NetView<'a, T: Real> {
    knots_u: &'a KnotVector,
    knots_v: &'a KnotVector,
    control: &'a [Point3<T>],
    weights: &'a [f64],
}

impl<'a, T: Real> NetView<'a, T> {
    fn of(s: &'a NurbsSurface<T>) -> Self {
        Self {
            knots_u: s.knots_u(),
            knots_v: s.knots_v(),
            control: s.control(),
            weights: s.weights(),
        }
    }

    /// `(nu, nv)`, once the control and weight slices both hold
    /// `nu · nv` entries — the counts every index below is formed from.
    fn counts(&self) -> Result<(usize, usize), SplineError> {
        let (nu, nv) = (self.knots_u.control_count(), self.knots_v.control_count());
        let expected = nu * nv;
        if self.control.len() != expected {
            return Err(SplineError::ControlCountMismatch {
                control: self.control.len(),
                expected,
            });
        }
        if self.weights.len() != expected {
            return Err(SplineError::WeightCountMismatch {
                weights: self.weights.len(),
                control: expected,
            });
        }
        Ok((nu, nv))
    }
}

/// The `u = 0` (`end = false`) or `u = 1` (`end = true`) boundary
/// iso-curve of a clamped surface: the first/last u-row of the control
/// net over `knots_v`, weights matching.
///
/// # Errors
///
/// [`SplineError`], each arm unreachable for a surface
/// `NurbsSurface::new` built and surfaced rather than swallowed
/// (D4 ¶2): [`SplineError::ControlCountMismatch`] or
/// [`SplineError::WeightCountMismatch`] for a net whose length
/// disagrees with its knot vectors, refused before any row is indexed
/// (the counts are the whole net's); [`SplineError::NonPositiveWeight`]
/// or [`SplineError::NonFiniteWeight`] for a bad weight on the
/// extracted row (its `index` counts along the row).
pub fn boundary_iso_u<T: Real>(
    s: &NurbsSurface<T>,
    end: bool,
) -> Result<NurbsCurve3<T>, SplineError> {
    net_iso_u(&NetView::of(s), end)
}

fn net_iso_u<T: Real>(net: &NetView<'_, T>, end: bool) -> Result<NurbsCurve3<T>, SplineError> {
    let (nu, nv) = net.counts()?;
    let base = if end { (nu - 1) * nv } else { 0 };
    let control = net.control[base..base + nv].to_vec();
    let weights = net.weights[base..base + nv].to_vec();
    NurbsCurve3::new(net.knots_v.clone(), control, weights)
}

/// The `v = 0` (`end = false`) or `v = 1` (`end = true`) boundary
/// iso-curve of a clamped surface: the first/last v-column of the
/// control net over `knots_u`, weights matching.
///
/// # Errors
///
/// [`SplineError`], as [`boundary_iso_u`]'s.
pub fn boundary_iso_v<T: Real>(
    s: &NurbsSurface<T>,
    end: bool,
) -> Result<NurbsCurve3<T>, SplineError> {
    net_iso_v(&NetView::of(s), end)
}

fn net_iso_v<T: Real>(net: &NetView<'_, T>, end: bool) -> Result<NurbsCurve3<T>, SplineError> {
    let (nu, nv) = net.counts()?;
    let offset = if end { nv - 1 } else { 0 };
    let control = (0..nu).map(|iu| net.control[iu * nv + offset]).collect();
    let weights = (0..nu).map(|iu| net.weights[iu * nv + offset]).collect();
    NurbsCurve3::new(net.knots_u.clone(), control, weights)
}

/// **The interior iso-curve `u = u*` of a clamped surface, by de Boor
/// collapse** — the curve `S(u*, ·)` in the surface's own `v` spline
/// space (`knots_v`, degree `q`), with control polygon
///
/// ```text
/// Q_j = Σᵢ λᵢ(j)·P_ij,   λᵢ(j) = Nᵢ(u*)·wᵢⱼ / Σₖ Nₖ(u*)·wₖⱼ
/// ```
///
/// (a convex combination of the net's u-rows at each `j`) and the
/// weights `W_j = Σᵢ Nᵢ(u*)·wᵢⱼ`. Exact in ℝ: it is the row that
/// inserting `u*` to multiplicity `p` would expose as a control-net
/// copy, which is why [`boundary_iso_u`] (multiplicity already `p+1`)
/// is a copy with no arithmetic.
///
/// **Weights are structure (C6), so the row's weight vector must be
/// structure too.** A carrier compared against this row in the same
/// rational space needs `W` as stored `f64`s, which holds exactly when
/// the net's weights factor in a way an exact `f64` test can see:
///
/// - **constant along `u`** (`wᵢⱼ == w₀ⱼ` for every `i`, bitwise):
///   `W_j = w₀ⱼ` in ℝ and `λᵢ = Nᵢ(u*) / Σₖ Nₖ(u*)`; the row carries
///   row 0's weights;
/// - **constant along `v`** (`wᵢⱼ == wᵢ₀` for every `j`, bitwise):
///   `W_j` is the same number at every `j` and cancels, so the curve
///   is the polynomial `Σⱼ Nⱼ(v)·Q_j` with
///   `λᵢ = Nᵢ(u*)·wᵢ₀ / Σₖ Nₖ(u*)·wₖ₀`; the row carries weights `1.0`
///   (every arc-profile loft/sweep wall and every imported cylinder
///   wall is this case).
///
/// A polynomial net is both at once; any other net refuses
/// [`IsoRowError::WeightsNotSeparable`]. The tests are bitwise on
/// stored structure, never a banded compare.
///
/// **At the interval scalar** `u*` may straddle a knot: every span it
/// overlaps is collapsed on its own polynomial and the rows are hulled
/// per coordinate ([`SpanLocate::enclosure_hull`], the evaluator's own
/// idiom), so each `Q_j` encloses the exact one. Total on the
/// arithmetic — a poisoned `u*` poisons the row (D4) — and
/// comparison-free on `T`: the only comparisons are the weight tests
/// on `f64` structure and the sealed locator.
///
/// # Errors
///
/// [`IsoRowError::WeightsNotSeparable`] for a net that factors neither
/// way; [`IsoRowError::Structure`] for a net whose length disagrees
/// with its knot vectors, refused before any row is indexed, or a row
/// that is not valid spline structure — unreachable for a surface
/// `NurbsSurface::new` built, surfaced rather than swallowed (D4 ¶2).
pub fn interior_iso_u<T: SpanLocate>(
    s: &NurbsSurface<T>,
    u: T,
) -> Result<NurbsCurve3<T>, IsoRowError<T>> {
    net_interior_iso_u(&NetView::of(s), u)
}

fn net_interior_iso_u<T: SpanLocate>(
    net: &NetView<'_, T>,
    u: T,
) -> Result<NurbsCurve3<T>, IsoRowError<T>> {
    let (nu, nv) = net
        .counts()
        .map_err(|source| IsoRowError::Structure { source })?;
    let w = net.weights;
    let along_u_constant = (0..nu).all(|i| (0..nv).all(|j| w[i * nv + j] == w[j]));
    let along_v_constant = (0..nu).all(|i| (0..nv).all(|j| w[i * nv + j] == w[i * nv]));
    // The per-row factor `ωᵢ` of `λᵢ ∝ Nᵢ(u*)·ωᵢ`, and the weights the
    // row is wrapped with (module docs: the two separable cases).
    let (omega, weights): (Vec<f64>, Vec<f64>) = if along_u_constant {
        (vec![1.0; nu], w[..nv].to_vec())
    } else if along_v_constant {
        ((0..nu).map(|i| w[i * nv]).collect(), vec![1.0; nv])
    } else {
        return Err(IsoRowError::WeightsNotSeparable {
            control_counts: (nu, nv),
        });
    };
    let ku = net.knots_u;
    let spans = u.locate_spans(ku);
    let mut hulled: Option<Vec<Point3<T>>> = None;
    for index in spans.first.index()..=spans.last.index() {
        // The locator's range may cross an EMPTY span (interior knot
        // multiplicity); the evaluators skip those the same way.
        let Some(span) = ku.span(index) else {
            continue;
        };
        let n = basis_funs(span, u);
        let base = span.first_control();
        // `λᵢ = Nᵢ·ωᵢ / Σₖ Nₖ·ωₖ`, the denominator once per span (it
        // does not depend on `j`). In case (a) `ωᵢ = 1` and the
        // denominator is the partition of unity, identically 1 in ℝ:
        // dividing by its enclosure would only widen the row, so the
        // row is the bare sum there — which still encloses the exact
        // point, because the exact `Σ Nᵢ` IS 1.
        let lam: Vec<T> = n
            .iter()
            .enumerate()
            .map(|(r, nr)| *nr * T::from_f64(omega[base + r]))
            .collect();
        let den = lam.iter().fold(T::zero(), |acc, l| acc + *l);
        let row: Vec<Point3<T>> = (0..nv)
            .map(|j| {
                let (mut x, mut y, mut z) = (T::zero(), T::zero(), T::zero());
                for (r, l) in lam.iter().enumerate() {
                    let p = net.control[(base + r) * nv + j];
                    x = x + *l * p.x;
                    y = y + *l * p.y;
                    z = z + *l * p.z;
                }
                if along_u_constant {
                    Point3::new(x, y, z)
                } else {
                    Point3::new(x / den, y / den, z / den)
                }
            })
            .collect();
        hulled = Some(match hulled {
            None => row,
            Some(acc) => acc
                .iter()
                .zip(&row)
                .map(|(a, b)| {
                    Point3::new(
                        a.x.enclosure_hull(b.x),
                        a.y.enclosure_hull(b.y),
                        a.z.enclosure_hull(b.z),
                    )
                })
                .collect(),
        });
    }
    // The locator's range is inclusive and its ends are validated
    // spans, so at least one iteration produced a row.
    let Some(control) = hulled else {
        unreachable!("locate_spans returned no nonempty span")
    };
    NurbsCurve3::new(net.knots_v.clone(), control, weights)
        .map_err(|source| IsoRowError::Structure { source })
}

/// Why [`iso_boundary_row`] or [`interior_iso_u`] could not hand back
/// a row.
#[derive(Clone, Debug)]
pub enum IsoRowError<T: Real> {
    /// `u` is not either end of the surface's own `u` domain, so there
    /// is no domain-end float to re-state the description at
    /// ([`iso_boundary_row`]'s contract; the collapse itself is
    /// [`interior_iso_u`]).
    Interior {
        /// The `u` asked for, echoed as data.
        u: T,
        /// The surface's `u` domain.
        domain: (f64, f64),
    },
    /// The net's length disagrees with its knot vectors, or the row is
    /// not valid spline structure — unreachable for a surface
    /// `NurbsSurface::new` built, surfaced rather than swallowed
    /// (D4 ¶2).
    Structure {
        /// The spline layer's typed refusal.
        source: SplineError,
    },
    /// The domain-endpoint coincidence test escalated.
    Escalated {
        /// The predicate-layer escalation.
        source: Indeterminate,
    },
    /// The weight net varies along BOTH parameters, so the collapsed
    /// row's weights `W_j = Σᵢ Nᵢ(u*)·wᵢⱼ` are computed, not structure:
    /// no stored carrier can share the row's rational space, which is
    /// the seam class's whole hypothesis ([`interior_iso_u`]).
    WeightsNotSeparable {
        /// The net's `(nu, nv)` control counts.
        control_counts: (usize, usize),
    },
}

impl<T: Real> core::fmt::Display for IsoRowError<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Interior { u, domain } => write!(
                f,
                "iso_boundary_row: u = {u:?} is interior to the chart's u domain {domain:?} — \
                 only a boundary row has a domain-end float to re-state the description at"
            ),
            Self::Structure { source } => {
                write!(
                    f,
                    "iso_boundary_row: the extracted row is not valid spline structure: {source}"
                )
            }
            // `iso_boundary_row` takes no declaration, so the recourse
            // is the two levers left rather than the payload's own tail.
            Self::Escalated { source } => write!(
                f,
                "whether an edge lies on the boundary of its face's fitted surface is too \
                 close to call: {} — {NO_DECLARATION_RECOURSE}",
                source.payload()
            ),
            Self::WeightsNotSeparable { control_counts } => write!(
                f,
                "interior_iso_u: the {control_counts:?} weight net varies along both u and v, \
                 so the collapsed row's weights are computed rather than structure and no \
                 carrier can share its rational space"
            ),
        }
    }
}

impl<T: Real> std::error::Error for IsoRowError<T> {}

/// **The `u = const` boundary row of a chart, selected by the stored
/// parameter** — [`boundary_iso_u`] with the end decided rather than
/// passed, which is what a consumer holding an
/// an iso chart image's `u` actually has.
///
/// Returns the row together with the DOMAIN endpoint it sits at, so
/// the caller re-states the description against the chart's own float
/// rather than the one it came in with.
///
/// The coincidence `u = u₀` / `u = u₁` is a named margined decide
/// (`iso_row_at_domain_end`), not an equality: a stored parameter and a
/// stored knot are two floats, and asking whether they name the same
/// chart line is a question with a band.
///
/// # Errors
///
/// [`IsoRowError`] — an interior `u`, an escalated coincidence, or a
/// row the spline layer refuses.
pub fn iso_boundary_row<T: Decide>(
    fit: &NurbsSurface<T>,
    u: T,
    band: Band,
) -> Result<(NurbsCurve3<T>, T), IsoRowError<T>> {
    let (u0, u1) = fit.knots_u().domain();
    let at = |end: f64| -> Result<bool, IsoRowError<T>> {
        let margin = Margin::of(u - T::from_f64(end));
        let sign = decide("iso_row_at_domain_end", margin, band)
            .map_err(|source| IsoRowError::Escalated { source })?;
        Ok(sign == Sign::Zero)
    };
    let end = if at(u0)? {
        (false, u0)
    } else if at(u1)? {
        (true, u1)
    } else {
        return Err(IsoRowError::Interior {
            u,
            domain: (u0, u1),
        });
    };
    let row = boundary_iso_u(fit, end.0).map_err(|source| IsoRowError::Structure { source })?;
    Ok((row, T::from_f64(end.1)))
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use geom_core::Point3;
    use geom_core::spline::KnotVector;

    use super::*;

    /// [`iso_boundary_row`] selects the row the stored parameter names,
    /// hands back the chart's OWN domain float for it, and refuses an
    /// interior parameter rather than approximating one.
    ///
    /// The row is asserted against the surface, not against a constant:
    /// it carries `knots_v` verbatim, which is the whole reason a
    /// consumer extracts instead of elevating and refining its own
    /// carrier into that space.
    /// The shared 3×2 (u×v) bilinear-ish fixture.
    fn surface() -> NurbsSurface<f64> {
        let ku = KnotVector::clamped(vec![0.0, 0.0, 0.5, 1.0, 1.0], 1).unwrap();
        let kv = KnotVector::unit_segment(core::num::NonZeroUsize::MIN);
        // Row-major iu·nv + iv, nv = 2.
        let control = vec![
            Point3::new(0.0, 0.0, 0.0),
            Point3::new(0.0, 1.0, 0.5),
            Point3::new(1.0, 0.0, 0.2),
            Point3::new(1.0, 1.0, 0.9),
            Point3::new(2.0, 0.0, -0.3),
            Point3::new(2.0, 1.0, 0.1),
        ];
        NurbsSurface::<f64>::new(ku, kv, control, vec![1.0; 6]).unwrap()
    }

    #[test]
    fn iso_boundary_row_selects_by_parameter_and_refuses_the_interior() {
        let s = surface();
        let band = geom_core::Band::linear(geom_core::Tol::witness()).unwrap();
        let (u0, u1) = s.knots_u().domain();

        let (first, at) = iso_boundary_row(&s, u0, band).expect("the u0 row extracts");
        assert_eq!(
            at, u0,
            "the description is re-stated at the chart's own float"
        );
        assert_eq!(first.knots().knots(), s.knots_v().knots());
        let want = boundary_iso_u(&s, false).unwrap();
        assert!(
            first
                .control()
                .iter()
                .zip(want.control())
                .all(|(a, b)| a.distance(*b) == 0.0),
            "the selected row IS the u0 row, point for point"
        );

        let (last, at) = iso_boundary_row(&s, u1, band).expect("the u1 row extracts");
        assert_eq!(at, u1);
        let want = boundary_iso_u(&s, true).unwrap();
        assert!(
            last.control()
                .iter()
                .zip(want.control())
                .all(|(a, b)| a.distance(*b) == 0.0),
            "and the u1 row for the other end"
        );

        let mid = (u0 + u1) * 0.5;
        let e = iso_boundary_row(&s, mid, band).expect_err("an interior u has no row");
        assert!(
            matches!(e, IsoRowError::Interior { u, domain } if u == mid && domain == (u0, u1)),
            "expected the interior refusal echoing the ask, got {e}"
        );
    }

    /// The collapse IS `S(u*, ·)`: on the 3×2 fixture, at a mid-span
    /// `u*` and at the interior knot, the extracted curve matches dense
    /// evaluation; at a domain end it is the boundary row bit for bit.
    #[test]
    fn interior_iso_matches_surface_evaluation() {
        let s = surface();
        for u in [0.25, 0.5, 0.8] {
            let c = interior_iso_u(&s, u).unwrap();
            assert_eq!(c.knots().knots(), s.knots_v().knots());
            for i in 0..=8 {
                let t = f64::from(i) / 8.0;
                assert!(
                    c.eval(t).distance(s.eval(u, t)) < 1e-15,
                    "u* = {u}, v = {t}"
                );
            }
        }
        let end = interior_iso_u(&s, 1.0).unwrap();
        let copy = boundary_iso_u(&s, true).unwrap();
        assert!(
            end.control()
                .iter()
                .zip(copy.control())
                .all(|(a, b)| a.distance(*b) == 0.0),
            "at a domain end the collapse is the boundary row's copy"
        );
    }

    /// A rational net whose weights vary along `u` only (a quarter
    /// circle swept in `v`, case (b)): the collapse carries the weights
    /// through `λ` and the row is wrapped polynomial. The transposed
    /// net (the quarter circle as the `v` profile, case (a)) wraps row
    /// 0's weights. A net varying both ways refuses typed.
    #[test]
    fn interior_iso_rational_cases() {
        let ku = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap();
        let kv = KnotVector::unit_segment(core::num::NonZeroUsize::MIN);
        let h = core::f64::consts::FRAC_1_SQRT_2;
        let mut control = Vec::new();
        let mut weights = Vec::new();
        for (x, y, w) in [(1.0, 0.0, 1.0), (1.0, 1.0, h), (0.0, 1.0, 1.0)] {
            for z in [0.0, 2.0] {
                control.push(Point3::new(x, y, z));
                weights.push(w);
            }
        }
        let s = NurbsSurface::<f64>::new(ku.clone(), kv.clone(), control.clone(), weights.clone())
            .unwrap();
        let c = interior_iso_u(&s, 0.3).unwrap();
        assert!(
            c.weights().iter().all(|w| *w == 1.0),
            "case (b) wraps polynomial"
        );
        for i in 0..=4 {
            let t = f64::from(i) / 4.0;
            let p = c.eval(t);
            assert!(p.distance(s.eval(0.3, t)) < 1e-14);
            assert!(
                (p.x.hypot(p.y) - 1.0).abs() < 1e-14,
                "on the cylinder: {p:?}"
            );
        }
        // Case (a): the same quarter circle as the PROFILE in `v`, swept
        // along `u` — weights vary along `v` only, so the collapsed row
        // carries row 0's weights and is the rational arc itself.
        let sa = {
            let ku = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
            let kv = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap();
            let mut control = Vec::new();
            let mut weights = Vec::new();
            for z in [0.0, 2.0] {
                for (x, y, w) in [(1.0, 0.0, 1.0), (1.0, 1.0, h), (0.0, 1.0, 1.0)] {
                    control.push(Point3::new(x, y, z));
                    weights.push(w);
                }
            }
            NurbsSurface::<f64>::new(ku, kv, control, weights).unwrap()
        };
        let c = interior_iso_u(&sa, 0.3).unwrap();
        assert_eq!(
            c.weights(),
            &[1.0, h, 1.0],
            "case (a) wraps row 0's weights"
        );
        for i in 0..=4 {
            let t = f64::from(i) / 4.0;
            let p = c.eval(t);
            assert!(p.distance(sa.eval(0.3, t)) < 1e-14);
            assert!(
                (p.x.hypot(p.y) - 1.0).abs() < 1e-14,
                "on the cylinder: {p:?}"
            );
        }
        weights[3] = 0.9;
        let s = NurbsSurface::<f64>::new(ku, kv, control, weights).unwrap();
        let e = interior_iso_u(&s, 0.3).expect_err("a net varying both ways has no shared space");
        assert!(
            matches!(
                e,
                IsoRowError::WeightsNotSeparable {
                    control_counts: (3, 2)
                }
            ),
            "{e}"
        );
    }

    /// A net whose length disagrees with its knot vectors refuses
    /// typed at every door, before any row is indexed — on both ends
    /// and both directions, short a control point or short a weight.
    /// `NurbsSurface::new` cannot build such a net, so the rows read
    /// the doors through [`NetView`] over the fixture's own parts.
    /// Without the count check each `end = true` row panics in its
    /// slice and each `end = false` row extracts `Ok`.
    #[test]
    fn a_net_whose_length_disagrees_with_its_knots_refuses_typed() {
        let s = surface();
        let (control, weights) = (s.control(), s.weights());
        let short_control = NetView {
            control: &control[..5],
            ..NetView::of(&s)
        };
        let short_weights = NetView {
            weights: &weights[..5],
            ..NetView::of(&s)
        };
        let control_arm = |e: &SplineError| {
            matches!(
                e,
                SplineError::ControlCountMismatch {
                    control: 5,
                    expected: 6
                }
            )
        };
        let weight_arm = |e: &SplineError| {
            matches!(
                e,
                SplineError::WeightCountMismatch {
                    weights: 5,
                    control: 6
                }
            )
        };
        for (what, net, arm) in [
            (
                "one control point short",
                &short_control,
                &control_arm as &dyn Fn(&SplineError) -> bool,
            ),
            ("one weight short", &short_weights, &weight_arm),
        ] {
            for end in [false, true] {
                for (door, got) in [("u", net_iso_u(net, end)), ("v", net_iso_v(net, end))] {
                    let e = got.expect_err("a net that disagrees with its knots has no row");
                    assert!(arm(&e), "{what}, boundary_iso_{door}(end = {end}): {e}");
                }
            }
            let e = net_interior_iso_u(net, 0.25).expect_err("nor an interior row");
            assert!(
                matches!(&e, IsoRowError::Structure { source } if arm(source)),
                "{what}, interior_iso_u: {e}"
            );
        }
    }

    /// The boundary-row coincidence escalating, raised for real: its
    /// sentence says what was being decided and routes the two levers a
    /// door without a declaration has, once.
    #[test]
    fn an_escalated_row_states_its_decision_and_no_declaration() {
        let band = geom_core::Band::new(1e-6, 1e-3).unwrap();
        let e = iso_boundary_row(&surface(), 1e-4, band).expect_err("1e-4 is in the band");
        assert!(matches!(e, IsoRowError::Escalated { .. }), "{e}");
        let text = e.to_string();
        assert!(
            test_utils::refusal::subjectless_escalations(&text).is_empty(),
            "the escalation names its decision: {text}"
        );
        assert_eq!(
            test_utils::refusal::recourse_markers(&text),
            1,
            "one recourse: {text}"
        );
        assert!(
            text.ends_with(NO_DECLARATION_RECOURSE) && !text.contains("declare"),
            "no declaration is offered where none is taken: {text}"
        );
    }

    /// A 3×2 (u×v) bilinear-ish surface: boundary extraction matches
    /// dense evaluation along each boundary.
    #[test]
    fn boundary_isos_match_surface_evaluation() {
        let s = surface();
        let u0 = boundary_iso_u(&s, false).unwrap();
        let u1 = boundary_iso_u(&s, true).unwrap();
        let v0 = boundary_iso_v(&s, false).unwrap();
        let v1 = boundary_iso_v(&s, true).unwrap();
        for i in 0..=8 {
            let t = f64::from(i) / 8.0;
            assert!(u0.eval(t).distance(s.eval(0.0, t)) < 1e-15);
            assert!(u1.eval(t).distance(s.eval(1.0, t)) < 1e-15);
            assert!(v0.eval(t).distance(s.eval(t, 0.0)) < 1e-15);
            assert!(v1.eval(t).distance(s.eval(t, 1.0)) < 1e-15);
        }
    }
}
