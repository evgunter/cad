//! Constructor sugar: human-friendly arc forms lowered to bulges.
//!
//! The **stored** form is always the bulge chain (the crate docs'
//! zero-consistency-conditions rule); sugar may take direction hints
//! ([`ArcSweep`]) or through-points, but nothing beyond the computed
//! bulge survives into the data.
//!
//! Sugar is *evaluation code*: total, comparison-free, no decisions.
//! Degenerate inputs (a through-point collinear-outside its chord, a
//! zero-radius center) produce well-defined poison or degenerate values
//! that [`crate::Profile::validate`] rejects with typed errors — the
//! sugar never guesses and never panics.

use geom_core::{Point2, Real};

use crate::{ProfileLoop, ProfileVertex};

/// The sweep direction hint for [`bulge_from_center`] /
/// [`LoopBuilder::close_arc_center`]: which way the arc winds about its
/// center (a hint consumed by sugar — the stored bulge carries the same
/// information as its sign).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ArcSweep {
    /// Counterclockwise sweep (positive included angle; positive bulge).
    Ccw,
    /// Clockwise sweep (negative included angle; negative bulge).
    Cw,
}

/// The bulge of the arc from `a` to `b` passing through `via`.
///
/// By the inscribed-angle theorem the arc's included angle is
/// θ = 2·Δ, where Δ is the signed turn from chord `a`→`via` to chord
/// `via`→`b` (independent of where on the arc `via` sits); the bulge is
/// tan(θ/4) = tan(Δ/2). Computed exactly in that form (fixed order,
/// D9): Δ = atan2(perp_dot(d₁, d₂), d₁·d₂), result = tan(Δ/2).
///
/// **Degenerate inputs, honestly:** `via` collinear between `a` and `b`
/// gives Δ = 0 ⇒ bulge 0 (the line segment the three points describe);
/// `via` collinear *outside* the chord gives Δ = ±π ⇒ tan(±π/2), an
/// infinite/huge value whose downstream geometry validation rejects;
/// `via` coincident with `a` or `b` makes one chord zero and the turn
/// ill-defined — atan2(0, 0) = 0 at `f64`, so the result degrades to a
/// line-ish bulge that validation judges on its merits. Total, never a
/// panic; the sugar does not decide (no predicates in evaluation code).
///
/// **Session-box gap (deferred D4 ¶4 item — see the crate docs):** a
/// *near*-collinear-outside `via` produces a finite but astronomically
/// large bulge — a carrier of ~1e15 m radius that today validates if
/// the loop is simple. Until kernel-wide session-box enforcement
/// lands, callers own the sanity of through-points.
pub fn bulge_from_via<T: Real>(a: Point2<T>, via: Point2<T>, b: Point2<T>) -> T {
    let d1 = via - a;
    let d2 = b - via;
    let turn = d1.perp_dot(d2).atan2(d1.dot(d2));
    (turn / T::from_f64(2.0)).tan()
}

/// The bulge of the arc from `a` to `b` about `center`, sweeping in the
/// `sweep` direction.
///
/// The included angle is the angular displacement from `a` to `b` as
/// seen from `center`, reduced into [0, 2π) for [`ArcSweep::Ccw`] (and
/// its negative-period mirror for [`ArcSweep::Cw`]) via
/// [`Real::reduce_periodic`]; the bulge is tan(θ/4). Fixed evaluation
/// order as written (D9).
///
/// **The center is a hint, not stored data**: the stored segment is
/// chord + bulge, whose implied center is the perpendicular-bisector
/// point at the implied radius. If `b` does not lie on the circle
/// through `a` about `center`, the stored arc still runs `a`→`b` with
/// the computed sweep — the intent's angles, the chord's geometry.
/// Coincident endpoints (θ that reduces to 0) or a center coincident
/// with an endpoint produce degenerate/poison values for validation to
/// reject; total, never a panic.
pub fn bulge_from_center<T: Real>(
    a: Point2<T>,
    b: Point2<T>,
    center: Point2<T>,
    sweep: ArcSweep,
) -> T {
    let va = a - center;
    let vb = b - center;
    let phi_a = va.y.atan2(va.x);
    let phi_b = vb.y.atan2(vb.x);
    let ccw = (phi_b - phi_a).reduce_periodic(T::tau());
    let theta = match sweep {
        ArcSweep::Ccw => ccw,
        ArcSweep::Cw => ccw - T::tau(),
    };
    (theta / T::from_f64(4.0)).tan()
}

/// A chain builder: `start` → (`line_to` | `arc_to` | `arc_to_via` |
/// `arc_to_center`)* → one `close*` call, yielding a [`ProfileLoop`].
///
/// Each step appends a vertex and sets the bulge of the segment
/// *arriving* at it on the previous vertex; the `close*` variants set
/// the implicit closing segment's bulge on the last vertex (closure
/// itself is by construction — there is no way to build an open chain).
#[derive(Clone, Debug)]
pub struct LoopBuilder<T: Real> {
    vertices: Vec<ProfileVertex<T>>,
}

impl<T: Real> LoopBuilder<T> {
    /// Starts a chain at `start`.
    pub fn start(start: Point2<T>) -> Self {
        Self {
            vertices: vec![ProfileVertex {
                pos: start,
                bulge: T::zero(),
            }],
        }
    }

    /// The current chain end (the last vertex's position).
    fn head(&self) -> Point2<T> {
        // The vector is nonempty by construction (`start` seeds it).
        self.vertices[self.vertices.len() - 1].pos
    }

    /// Sets the bulge of the segment leaving the current last vertex.
    fn set_leaving_bulge(&mut self, bulge: T) {
        let last = self.vertices.len() - 1;
        self.vertices[last].bulge = bulge;
    }

    /// Appends a straight segment to `p`.
    pub fn line_to(mut self, p: Point2<T>) -> Self {
        self.set_leaving_bulge(T::zero());
        self.vertices.push(ProfileVertex {
            pos: p,
            bulge: T::zero(),
        });
        self
    }

    /// Appends an arc segment to `p` with an explicit `bulge` (the raw
    /// form — see the crate docs for the sign convention).
    pub fn arc_to(mut self, p: Point2<T>, bulge: T) -> Self {
        self.set_leaving_bulge(bulge);
        self.vertices.push(ProfileVertex {
            pos: p,
            bulge: T::zero(),
        });
        self
    }

    /// Appends the arc through `via` ending at `p` (three-point form;
    /// see [`bulge_from_via`]).
    pub fn arc_to_via(self, via: Point2<T>, p: Point2<T>) -> Self {
        let bulge = bulge_from_via(self.head(), via, p);
        self.arc_to(p, bulge)
    }

    /// Appends the arc about `center` ending at `p`, sweeping `sweep`
    /// (center form; see [`bulge_from_center`]).
    pub fn arc_to_center(self, p: Point2<T>, center: Point2<T>, sweep: ArcSweep) -> Self {
        let bulge = bulge_from_center(self.head(), p, center, sweep);
        self.arc_to(p, bulge)
    }

    /// Closes the chain with a straight segment back to the start.
    pub fn close(mut self) -> ProfileLoop<T> {
        self.set_leaving_bulge(T::zero());
        ProfileLoop::new(self.vertices)
    }

    /// Closes the chain with an arc of the given `bulge` back to the
    /// start.
    pub fn close_with_bulge(mut self, bulge: T) -> ProfileLoop<T> {
        self.set_leaving_bulge(bulge);
        ProfileLoop::new(self.vertices)
    }

    /// Closes the chain with the arc through `via` back to the start.
    pub fn close_arc_via(self, via: Point2<T>) -> ProfileLoop<T> {
        let first = self.vertices[0].pos;
        let bulge = bulge_from_via(self.head(), via, first);
        self.close_with_bulge(bulge)
    }

    /// Closes the chain with the arc about `center` back to the start,
    /// sweeping `sweep`.
    pub fn close_arc_center(self, center: Point2<T>, sweep: ArcSweep) -> ProfileLoop<T> {
        let first = self.vertices[0].pos;
        let bulge = bulge_from_center(self.head(), first, center, sweep);
        self.close_with_bulge(bulge)
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use geom_core::Tolerance;

    use super::*;
    use crate::{Profile, SketchPlane};

    fn p2(x: f64, y: f64) -> Point2<f64> {
        Point2::new(x, y)
    }

    #[test]
    fn bulge_from_via_quarter_circle() {
        // Unit-circle quarter arc (1,0) → (0,1) through the apex.
        let b = bulge_from_via(
            p2(1.0, 0.0),
            p2(
                core::f64::consts::FRAC_1_SQRT_2,
                core::f64::consts::FRAC_1_SQRT_2,
            ),
            p2(0.0, 1.0),
        );
        assert!((b - (core::f64::consts::FRAC_PI_8).tan()).abs() < 1e-15);
    }

    #[test]
    fn bulge_from_via_is_via_position_independent() {
        // The inscribed-angle theorem: any via on the arc gives the
        // same bulge. Points on the unit circle at 10° and 80°.
        let at = |deg: f64| {
            let (s, c) = deg.to_radians().sin_cos();
            p2(c, s)
        };
        let b1 = bulge_from_via(p2(1.0, 0.0), at(10.0), p2(0.0, 1.0));
        let b2 = bulge_from_via(p2(1.0, 0.0), at(80.0), p2(0.0, 1.0));
        assert!((b1 - b2).abs() < 1e-14);
    }

    #[test]
    fn bulge_from_via_semicircle_and_sign() {
        // Through the lower apex: a counterclockwise semicircle,
        // bulge +1.
        let b = bulge_from_via(p2(0.0, 0.0), p2(1.0, -1.0), p2(2.0, 0.0));
        assert!((b - 1.0).abs() < 1e-15);
        // Mirrored via: clockwise, bulge −1.
        let b = bulge_from_via(p2(0.0, 0.0), p2(1.0, 1.0), p2(2.0, 0.0));
        assert!((b + 1.0).abs() < 1e-15);
    }

    #[test]
    fn bulge_from_via_degenerate_inputs_are_total() {
        // Collinear between: a line.
        assert_eq!(
            bulge_from_via(p2(0.0, 0.0), p2(1.0, 0.0), p2(2.0, 0.0)),
            0.0
        );
        // Collinear outside: tan(±π/2) — huge, for validation to
        // reject; never a panic.
        let b = bulge_from_via(p2(0.0, 0.0), p2(3.0, 0.0), p2(2.0, 0.0));
        assert!(b.abs() > 1e12);
    }

    #[test]
    fn bulge_from_center_quarter_arcs_both_ways() {
        let b = bulge_from_center(p2(1.0, 0.0), p2(0.0, 1.0), p2(0.0, 0.0), ArcSweep::Ccw);
        assert!((b - core::f64::consts::FRAC_PI_8.tan()).abs() < 1e-15);
        // Clockwise from (1,0) to (0,1) is the long way round:
        // θ = −3π/2, bulge = tan(−3π/8).
        let b = bulge_from_center(p2(1.0, 0.0), p2(0.0, 1.0), p2(0.0, 0.0), ArcSweep::Cw);
        assert!((b - (-3.0 * core::f64::consts::FRAC_PI_8).tan()).abs() < 1e-12);
    }

    #[test]
    fn builder_builds_the_two_arc_circle_by_all_three_forms() {
        let tol = Tolerance { eps: 1e-9 };
        let raw = ProfileLoop::builder(p2(-1.0, 0.0))
            .arc_to(p2(1.0, 0.0), 1.0)
            .close_with_bulge(1.0);
        let via = ProfileLoop::builder(p2(-1.0, 0.0))
            .arc_to_via(p2(0.0, -1.0), p2(1.0, 0.0))
            .close_arc_via(p2(0.0, 1.0));
        let center = ProfileLoop::builder(p2(-1.0, 0.0))
            .arc_to_center(p2(1.0, 0.0), p2(0.0, 0.0), ArcSweep::Ccw)
            .close_arc_center(p2(0.0, 0.0), ArcSweep::Ccw);
        for lp in [raw, via, center] {
            assert_eq!(lp.vertices.len(), 2);
            for v in &lp.vertices {
                assert!((v.bulge - 1.0).abs() < 1e-12, "bulge {}", v.bulge);
            }
            let vp = Profile::new(SketchPlane::xy(), vec![lp])
                .validate(tol)
                .expect("the built circle must validate");
            match vp.loops()[0].segments()[0].kind {
                crate::SegmentKind::Arc { center, radius, .. } => {
                    assert!(center.x.abs() < 1e-12 && center.y.abs() < 1e-12);
                    assert!((radius - 1.0).abs() < 1e-12);
                }
                crate::SegmentKind::Line => panic!("must classify as an arc"),
            }
        }
    }

    #[test]
    fn builder_line_and_arc_mix() {
        // A stadium: two straight sides, two semicircular caps.
        let tol = Tolerance { eps: 1e-9 };
        let lp = ProfileLoop::builder(p2(0.0, 0.0))
            .line_to(p2(2.0, 0.0))
            .arc_to_center(p2(2.0, 1.0), p2(2.0, 0.5), ArcSweep::Ccw)
            .line_to(p2(0.0, 1.0))
            .close_arc_center(p2(0.0, 0.5), ArcSweep::Ccw);
        let vp = Profile::new(SketchPlane::xy(), vec![lp])
            .validate(tol)
            .expect("the stadium must validate");
        let kinds: Vec<bool> = vp.loops()[0]
            .segments()
            .iter()
            .map(|s| matches!(s.kind, crate::SegmentKind::Line))
            .collect();
        // Canonical start is (0, 0); chain: line, arc, line, arc.
        assert_eq!(kinds, vec![true, false, true, false]);
    }
}
