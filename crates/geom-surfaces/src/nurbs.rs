//! The tensor-product NURBS surface — the [`crate::Surface::Nurbs`]
//! payload (M5 PR 3).
//!
//! Data model, evaluation contract, and fixed-association rules are
//! the curve module's, lifted to the tensor product (see
//! `geom_curves::nurbs` and `docs/M5-PR3-SPEC.md`; the conventions are
//! stated once there and once here — clamped-v1 per direction, f64
//! structure, positive weights, span contract with documented
//! polynomial-extension garbage-out, poison on invalid span indices).
//!
//! # Grid layout (binding)
//!
//! Control points and weights are **row-major over `u` then `v`**:
//! `index = iu · nv + iv` with `nu = knots_u.control_count()`,
//! `nv = knots_v.control_count()`. Combination is a double ascending
//! pass — outer `iu`, inner `iv`, exactly as written in the evaluator
//! bodies.
//!
//! # Direction-mapped knot algebra
//!
//! The u-direction operations apply the shared curve plans
//! (`geom_core::spline::algebra`) **per v-column** (each column has
//! its own weights, hence its own projective λs; the knot schedule is
//! shared). The v-direction operations are the u-direction ones
//! conjugated by [`NurbsSurface::transposed`] — one implementation,
//! both directions, deterministic.

use geom_core::spline::{self, KnotAlgebraError, KnotVector, SpanLocate, SplineError};
use geom_core::{Point3, Real, Vec3};

/// The point and first/second partials of one evaluation — everything
/// the [`crate::Surface`] evaluator arms need from a single
/// span-restricted pass.
#[derive(Clone, Copy, Debug)]
pub struct SurfaceJet<T: Real> {
    /// The surface point `S(u, v)`.
    pub point: Point3<T>,
    /// `∂S/∂u`.
    pub du: Vec3<T>,
    /// `∂S/∂v`.
    pub dv: Vec3<T>,
    /// `∂²S/∂u²`.
    pub duu: Vec3<T>,
    /// `∂²S/∂u∂v`.
    pub duv: Vec3<T>,
    /// `∂²S/∂v²`.
    pub dvv: Vec3<T>,
}

/// A validated tensor-product NURBS surface (module docs; immutable
/// after construction — every knot-algebra operation returns a new
/// surface).
#[derive(Clone, Debug)]
pub struct NurbsSurface<T: Real> {
    knots_u: KnotVector,
    knots_v: KnotVector,
    control: Vec<Point3<T>>,
    weights: Vec<f64>,
}

impl<T: Real> NurbsSurface<T> {
    /// Validated construction: `control.len()` must equal
    /// `knots_u.control_count() · knots_v.control_count()` (row-major
    /// layout, module docs), weights match and are strictly positive
    /// and finite.
    ///
    /// # Errors
    ///
    /// [`SplineError`] naming the exact violation.
    pub fn new(
        knots_u: KnotVector,
        knots_v: KnotVector,
        control: Vec<Point3<T>>,
        weights: Vec<f64>,
    ) -> Result<Self, SplineError> {
        let expected = knots_u.control_count() * knots_v.control_count();
        if control.len() != expected {
            return Err(SplineError::ControlCountMismatch {
                control: control.len(),
                expected,
            });
        }
        if weights.len() != control.len() {
            return Err(SplineError::WeightCountMismatch {
                weights: weights.len(),
                control: control.len(),
            });
        }
        for (index, w) in weights.iter().enumerate() {
            if !(*w > 0.0) {
                return Err(SplineError::NonPositiveWeight { index, weight: *w });
            }
            if !w.is_finite() {
                return Err(SplineError::NonFiniteWeight { index, weight: *w });
            }
        }
        Ok(Self {
            knots_u,
            knots_v,
            control,
            weights,
        })
    }

    /// The "no description yet" placeholder payload for
    /// [`crate::Surface::Nurbs`]: structurally valid (bilinear on
    /// `[0,1]²`, unit weights) with all-poison control points, so
    /// every evaluation is all-poison — bit-for-bit the former unit
    /// placeholder's totality behavior (D4 ¶2: fails every downstream
    /// certification loudly).
    pub fn placeholder() -> Self {
        let nan = T::from_f64(f64::NAN);
        let p = Point3::new(nan, nan, nan);
        Self {
            knots_u: KnotVector::unit_segment(1),
            knots_v: KnotVector::unit_segment(1),
            control: vec![p; 4],
            weights: vec![1.0; 4],
        }
    }

    /// The u-direction knot vector.
    pub fn knots_u(&self) -> &KnotVector {
        &self.knots_u
    }

    /// The v-direction knot vector.
    pub fn knots_v(&self) -> &KnotVector {
        &self.knots_v
    }

    /// The control net, row-major (`iu · nv + iv` — module docs).
    pub fn control(&self) -> &[Point3<T>] {
        &self.control
    }

    /// The weights, same layout as [`NurbsSurface::control`].
    pub fn weights(&self) -> &[f64] {
        &self.weights
    }

    /// `(nu, nv)` — control counts per direction.
    pub fn control_counts(&self) -> (usize, usize) {
        (self.knots_u.control_count(), self.knots_v.control_count())
    }

    /// The all-poison point.
    fn poison_point() -> Point3<T> {
        let nan = T::from_f64(f64::NAN);
        Point3::new(nan, nan, nan)
    }

    /// The all-poison vector.
    fn poison_vec() -> Vec3<T> {
        let nan = T::from_f64(f64::NAN);
        Vec3::new(nan, nan, nan)
    }

    /// The all-poison jet.
    fn poison_jet() -> SurfaceJet<T> {
        SurfaceJet {
            point: Self::poison_point(),
            du: Self::poison_vec(),
            dv: Self::poison_vec(),
            duu: Self::poison_vec(),
            duv: Self::poison_vec(),
            dvv: Self::poison_vec(),
        }
    }

    /// The point at `(u, v)` in the given span pair — the generic core
    /// (span contract and garbage-out as the curve core; invalid span
    /// indices poison). Double ascending pass (outer `iu`, inner `iv`),
    /// then one division per coordinate.
    pub fn eval_in_span(&self, span_u: usize, span_v: usize, u: T, v: T) -> Point3<T> {
        if !self.spans_valid(span_u, span_v) {
            return Self::poison_point();
        }
        let (pu, pv) = (self.knots_u.degree(), self.knots_v.degree());
        let nv = self.knots_v.control_count();
        let bu = spline::basis::basis_funs(&self.knots_u, span_u, u);
        let bv = spline::basis::basis_funs(&self.knots_v, span_v, v);
        let (mut x, mut y, mut z, mut w) = (T::zero(), T::zero(), T::zero(), T::zero());
        for (i, bui) in bu.iter().enumerate() {
            let iu = span_u - pu + i;
            for (j, bvj) in bv.iter().enumerate() {
                // Indexing justified: valid spans ⇒ iu < nu, iv < nv
                // (the curve-core argument per direction).
                let iv = span_v - pv + j;
                let idx = iu * nv + iv;
                let cw = (*bui * *bvj) * T::from_f64(self.weights[idx]);
                let pt = self.control[idx];
                x = x + cw * pt.x;
                y = y + cw * pt.y;
                z = z + cw * pt.z;
                w = w + cw;
            }
        }
        Point3::new(x / w, y / w, z / w)
    }

    /// Point plus first and second partials at `(u, v)` in the given
    /// span pair — one homogeneous tensor pass (basis orders 0..=2 in
    /// each direction), then the rational corrections exactly as
    /// written: `S = A₀₀/w₀₀`, `S_u = (A₁₀ − S·w₁₀)/w₀₀` (v symmetric),
    /// `S_uu = (A₂₀ − S·w₂₀ − S_u·w₁₀·2)/w₀₀` (v symmetric),
    /// `S_uv = (A₁₁ − S·w₁₁ − S_u·w₀₁ − S_v·w₁₀)/w₀₀`.
    pub fn ders_in_span(&self, span_u: usize, span_v: usize, u: T, v: T) -> SurfaceJet<T> {
        if !self.spans_valid(span_u, span_v) {
            return Self::poison_jet();
        }
        let (pu, pv) = (self.knots_u.degree(), self.knots_v.degree());
        let nv = self.knots_v.control_count();
        let du = spline::basis::ders_basis_funs(&self.knots_u, span_u, u, 2);
        let dv = spline::basis::ders_basis_funs(&self.knots_v, span_v, v, 2);
        // Homogeneous partials A_kl for the six (k, l) with k + l ≤ 2,
        // indexed [k][l]; each lane accumulated in the double
        // ascending pass.
        let mut ax = [[T::zero(); 3]; 3];
        let mut ay = [[T::zero(); 3]; 3];
        let mut az = [[T::zero(); 3]; 3];
        let mut aw = [[T::zero(); 3]; 3];
        for (i, _) in du[0].iter().enumerate() {
            let iu = span_u - pu + i;
            for (j, _) in dv[0].iter().enumerate() {
                let iv = span_v - pv + j;
                let idx = iu * nv + iv;
                let wf = T::from_f64(self.weights[idx]);
                let pt = self.control[idx];
                for k in 0..3usize {
                    for l in 0..3usize {
                        if k + l > 2 {
                            continue;
                        }
                        let cw = (du[k][i] * dv[l][j]) * wf;
                        ax[k][l] = ax[k][l] + cw * pt.x;
                        ay[k][l] = ay[k][l] + cw * pt.y;
                        az[k][l] = az[k][l] + cw * pt.z;
                        aw[k][l] = aw[k][l] + cw;
                    }
                }
            }
        }
        let two = T::from_f64(2.0);
        let w00 = aw[0][0];
        let s = Point3::new(ax[0][0] / w00, ay[0][0] / w00, az[0][0] / w00);
        let sv3 = Vec3::new(s.x, s.y, s.z);
        let s_u = (Vec3::new(ax[1][0], ay[1][0], az[1][0]) - sv3 * aw[1][0]) / w00;
        let s_v = (Vec3::new(ax[0][1], ay[0][1], az[0][1]) - sv3 * aw[0][1]) / w00;
        let s_uu =
            (Vec3::new(ax[2][0], ay[2][0], az[2][0]) - sv3 * aw[2][0] - s_u * (aw[1][0] * two))
                / w00;
        let s_vv =
            (Vec3::new(ax[0][2], ay[0][2], az[0][2]) - sv3 * aw[0][2] - s_v * (aw[0][1] * two))
                / w00;
        let s_uv = (Vec3::new(ax[1][1], ay[1][1], az[1][1])
            - sv3 * aw[1][1]
            - s_u * aw[0][1]
            - s_v * aw[1][0])
            / w00;
        SurfaceJet {
            point: s,
            du: s_u,
            dv: s_v,
            duu: s_uu,
            duv: s_uv,
            dvv: s_vv,
        }
    }

    /// Span-pair validity (structure check backing the poison
    /// totalization of the cores).
    fn spans_valid(&self, span_u: usize, span_v: usize) -> bool {
        span_u >= self.knots_u.first_span()
            && span_u <= self.knots_u.last_span()
            && span_v >= self.knots_v.first_span()
            && span_v <= self.knots_v.last_span()
    }

    /// The transposed surface: `u` and `v` swapped (knot vectors
    /// swapped, grid re-indexed). Involutive; the conjugation that
    /// gives every v-direction knot-algebra op from its u-direction
    /// implementation.
    pub fn transposed(&self) -> Self {
        let (nu, nv) = self.control_counts();
        let mut control = Vec::with_capacity(self.control.len());
        let mut weights = Vec::with_capacity(self.weights.len());
        for iv in 0..nv {
            for iu in 0..nu {
                // Indexing justified: iu < nu, iv < nv (construction).
                control.push(self.control[iu * nv + iv]);
                weights.push(self.weights[iu * nv + iv]);
            }
        }
        Self {
            knots_u: self.knots_v.clone(),
            knots_v: self.knots_u.clone(),
            control,
            weights,
        }
    }

    /// Extracts v-column `iv` as a (points, weights) pair — a curve in
    /// the u direction.
    fn u_column(&self, iv: usize) -> (Vec<Point3<T>>, Vec<f64>) {
        let (nu, nv) = self.control_counts();
        let mut pts = Vec::with_capacity(nu);
        let mut w = Vec::with_capacity(nu);
        for iu in 0..nu {
            pts.push(self.control[iu * nv + iv]);
            w.push(self.weights[iu * nv + iv]);
        }
        (pts, w)
    }

    /// Rebuilds a surface from per-column (points, weights) with a new
    /// u knot vector (columns share it by construction).
    fn from_u_columns(
        knots_u: KnotVector,
        knots_v: KnotVector,
        cols: Vec<(Vec<Point3<T>>, Vec<f64>)>,
    ) -> Self {
        let nu = knots_u.control_count();
        let nv = knots_v.control_count();
        let mut control = Vec::with_capacity(nu * nv);
        let mut weights = Vec::with_capacity(nu * nv);
        for iu in 0..nu {
            for (pts, w) in &cols {
                // Indexing justified: every column has nu entries (the
                // shared plan chain fixes the length).
                control.push(pts[iu]);
                weights.push(w[iu]);
            }
        }
        Self {
            knots_u,
            knots_v,
            control,
            weights,
        }
    }

    /// Applies one shared-schedule plan chain builder per v-column
    /// (module docs: per-column weights ⇒ per-column λs, shared knot
    /// schedule) and reassembles the grid.
    fn map_u_columns(
        &self,
        build: impl Fn(&KnotVector, &[f64]) -> Result<Vec<spline::CurvePlan>, KnotAlgebraError>,
    ) -> Result<Self, KnotAlgebraError> {
        let (_, nv) = self.control_counts();
        let mut cols = Vec::with_capacity(nv);
        let mut new_knots_u = self.knots_u.clone();
        for iv in 0..nv {
            let (mut pts, col_w) = self.u_column(iv);
            let plans = build(&self.knots_u, &col_w)?;
            let mut w = col_w;
            for plan in &plans {
                pts = plan.apply_points(&pts, Self::poison_point(), |x, y, l| {
                    x.lerp(y, T::from_f64(l))
                });
                w = plan.weights().to_vec();
            }
            if let Some(last) = plans.last() {
                new_knots_u = last.knots().clone();
            }
            cols.push((pts, w));
        }
        Ok(Self::from_u_columns(
            new_knots_u,
            self.knots_v.clone(),
            cols,
        ))
    }

    /// u-direction knot insertion (§5.2, per v-column). Evaluation-
    /// invariant in ℝ.
    ///
    /// # Errors
    ///
    /// As the curve op ([`geom_core::spline::algebra::insert_knot_plan`]).
    pub fn insert_knot_u(&self, u: f64, times: usize) -> Result<Self, KnotAlgebraError> {
        self.map_u_columns(|kv, w| spline::algebra::insert_knot_plan(kv, w, u, times))
    }

    /// v-direction knot insertion — the u op conjugated by transpose.
    ///
    /// # Errors
    ///
    /// As [`NurbsSurface::insert_knot_u`].
    pub fn insert_knot_v(&self, v: f64, times: usize) -> Result<Self, KnotAlgebraError> {
        Ok(self.transposed().insert_knot_u(v, times)?.transposed())
    }

    /// u-direction refinement (§5.3, ascending fold of insertions).
    ///
    /// # Errors
    ///
    /// As [`NurbsSurface::insert_knot_u`], against cumulative structure.
    pub fn refine_knots_u(&self, add: &[f64]) -> Result<Self, KnotAlgebraError> {
        self.map_u_columns(|kv, w| spline::algebra::refine_plan(kv, w, add))
    }

    /// v-direction refinement.
    ///
    /// # Errors
    ///
    /// As [`NurbsSurface::refine_knots_u`].
    pub fn refine_knots_v(&self, add: &[f64]) -> Result<Self, KnotAlgebraError> {
        Ok(self.transposed().refine_knots_u(add)?.transposed())
    }

    /// u-direction degree elevation (§5.5, Bézier route per column).
    ///
    /// # Errors
    ///
    /// As the curve op ([`geom_core::spline::algebra::elevate_plan`]).
    pub fn elevate_degree_u(&self, raise: usize) -> Result<Self, KnotAlgebraError> {
        let mut cur = self.clone();
        for _ in 0..raise {
            cur = cur.map_u_columns(|kv, w| spline::algebra::elevate_plan(kv, w))?;
        }
        Ok(cur)
    }

    /// v-direction degree elevation.
    ///
    /// # Errors
    ///
    /// As [`NurbsSurface::elevate_degree_u`].
    pub fn elevate_degree_v(&self, raise: usize) -> Result<Self, KnotAlgebraError> {
        Ok(self.transposed().elevate_degree_u(raise)?.transposed())
    }

    /// u-direction bounded knot removal (§5.4): removes `times` copies
    /// of the interior u-knot and returns the surface **with a
    /// sup-norm bound** on `|S − Ŝ|` over the whole domain — the
    /// curve bound's mechanism lifted to the grid (partition of unity
    /// holds in both directions, so the per-pass projected bound uses
    /// grid-wide maxima; passes add). See
    /// `geom_curves::nurbs::NurbsCurve3::remove_knot` for the
    /// projective derivation.
    ///
    /// # Errors
    ///
    /// As the curve op (`KnotNotPresent`, multiplicity, weight
    /// collapse, structure).
    pub fn remove_knot_u(&self, u: f64, times: usize) -> Result<(Self, T), KnotAlgebraError> {
        let mut cur = self.clone();
        let mut bound = T::zero();
        for _ in 0..times {
            // One pass per iteration: per-column removal plan plus its
            // exact reinsertion for the perturbation measurement.
            let (_, nv) = cur.control_counts();
            let mut removed_cols = Vec::with_capacity(nv);
            let mut reinserted_cols = Vec::with_capacity(nv);
            let mut new_knots_u = cur.knots_u.clone();
            for iv in 0..nv {
                let (pts, col_w) = cur.u_column(iv);
                let steps = spline::algebra::remove_knot_plan(&cur.knots_u, &col_w, u, 1)?;
                // remove_knot_plan(times = 1) yields exactly one step;
                // the refusal arm is unreachable but keeps this total
                // without an unwrap (D9: no panic path).
                let step = steps
                    .into_iter()
                    .next()
                    .ok_or(KnotAlgebraError::KnotNotPresent { u })?;
                let lerp = |x: Point3<T>, y: Point3<T>, l: f64| x.lerp(y, T::from_f64(l));
                let rem_pts = step.plan.apply_points(&pts, Self::poison_point(), lerp);
                let re_pts = step
                    .reinsert
                    .apply_points(&rem_pts, Self::poison_point(), lerp);
                new_knots_u = step.plan.knots().clone();
                removed_cols.push((rem_pts, step.plan.weights().to_vec()));
                reinserted_cols.push((re_pts, step.reinsert.weights().to_vec()));
            }
            let reinserted =
                Self::from_u_columns(cur.knots_u.clone(), cur.knots_v.clone(), reinserted_cols);
            bound = bound + Self::removal_pass_bound(&cur, &reinserted);
            cur = Self::from_u_columns(new_knots_u, cur.knots_v.clone(), removed_cols);
        }
        Ok((cur, bound))
    }

    /// v-direction bounded knot removal.
    ///
    /// # Errors
    ///
    /// As [`NurbsSurface::remove_knot_u`].
    pub fn remove_knot_v(&self, v: f64, times: usize) -> Result<(Self, T), KnotAlgebraError> {
        let (s, b) = self.transposed().remove_knot_u(v, times)?;
        Ok((s.transposed(), b))
    }

    /// One removal pass's projected perturbation bound over the grid
    /// (`orig` and `re` share knot vectors and grid shape by
    /// construction). Ascending-index `Real::max` folds — lattice
    /// value ops, never control flow.
    fn removal_pass_bound(orig: &Self, re: &Self) -> T {
        let origin = Point3::new(T::zero(), T::zero(), T::zero());
        let mut c_max = T::zero();
        let mut bwp = T::zero();
        let mut bw = 0.0f64;
        let mut w_min = f64::INFINITY;
        for (i, pt) in orig.control.iter().enumerate() {
            c_max = c_max.max((*pt - origin).norm());
            // Indexing justified: shared grid shape (fn docs).
            let (rp, rw) = (re.control[i], re.weights[i]);
            let dw = (orig.weights[i] - rw).abs();
            if dw > bw {
                bw = dw;
            }
            if rw < w_min {
                w_min = rw;
            }
            let d = (*pt - origin) * T::from_f64(orig.weights[i]) - (rp - origin) * T::from_f64(rw);
            bwp = bwp.max(d.norm());
        }
        (c_max * T::from_f64(bw) + bwp) * T::from_f64(1.0 / w_min)
    }
}

impl<T: SpanLocate> NurbsSurface<T> {
    /// The full jet at `(u, v)`: span selection per direction through
    /// the sealed [`SpanLocate`] seam, the core per overlapped span
    /// cell, channel-independent hulls across cells for
    /// interval-natured scalars (rectangle iteration: ascending u
    /// spans outer, v spans inner).
    pub fn ders(&self, u: T, v: T) -> SurfaceJet<T> {
        let su = u.locate_spans(&self.knots_u);
        let sv = v.locate_spans(&self.knots_v);
        let mut acc = self.ders_in_span(su.first, sv.first, u, v);
        for cu in su.first..=su.last {
            for cv in sv.first..=sv.last {
                if cu == su.first && cv == sv.first {
                    continue;
                }
                let jet = self.ders_in_span(cu, cv, u, v);
                acc = SurfaceJet {
                    point: hull_point(acc.point, jet.point),
                    du: hull_vec(acc.du, jet.du),
                    dv: hull_vec(acc.dv, jet.dv),
                    duu: hull_vec(acc.duu, jet.duu),
                    duv: hull_vec(acc.duv, jet.duv),
                    dvv: hull_vec(acc.dvv, jet.dvv),
                };
            }
        }
        acc
    }

    /// The point at `(u, v)` (span selection as [`NurbsSurface::ders`];
    /// point-only pass).
    pub fn eval(&self, u: T, v: T) -> Point3<T> {
        let su = u.locate_spans(&self.knots_u);
        let sv = v.locate_spans(&self.knots_v);
        let mut acc = self.eval_in_span(su.first, sv.first, u, v);
        for cu in su.first..=su.last {
            for cv in sv.first..=sv.last {
                if cu == su.first && cv == sv.first {
                    continue;
                }
                acc = hull_point(acc, self.eval_in_span(cu, cv, u, v));
            }
        }
        acc
    }
}

/// Channel-independent point hull (the seam's multi-span combination).
fn hull_point<T: SpanLocate>(a: Point3<T>, b: Point3<T>) -> Point3<T> {
    Point3::new(
        a.x.enclosure_hull(b.x),
        a.y.enclosure_hull(b.y),
        a.z.enclosure_hull(b.z),
    )
}

/// Channel-independent vector hull.
fn hull_vec<T: SpanLocate>(a: Vec3<T>, b: Vec3<T>) -> Vec3<T> {
    Vec3::new(
        a.x.enclosure_hull(b.x),
        a.y.enclosure_hull(b.y),
        a.z.enclosure_hull(b.z),
    )
}
