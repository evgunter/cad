//! The planar arc carrier: a circle and a signed sweep on it, with no
//! endpoints.
//!
//! [`Arc2`] is the one arc value the sketch layers share: `profile`'s
//! stored and validated segments and `geom-brep`'s sketch segment all
//! hold one. It carries the carrier and the interval's signed length;
//! the endpoints belong to whatever holds it (a loop's vertex table, a
//! sketch segment's `a` and `b`), which is why its one evaluation,
//! [`Arc2::point_from`], takes the start point as an argument.

use crate::linalg::{Point2, Vec2};
use crate::real::SymRegistration;
use crate::{Real, Tol};

/// A circular arc in the plane, without its endpoints: the carrier
/// circle (`centre`, `radius`) and the signed sweep Δθ from the arc's
/// start to its end about `centre`, positive counterclockwise.
///
/// Plain data, checked nowhere at this type's door. The fields are
/// redundant with the endpoints of whatever holds the arc (they lie on
/// the circle, and the sweep turns the start into the end); that
/// consistency is the holder's to establish.
#[derive(Clone, Copy, Debug)]
pub struct Arc2<T: Real> {
    /// The carrier circle's centre.
    pub centre: Point2<T>,
    /// The carrier circle's radius (positive).
    pub radius: T,
    /// The signed sweep Δθ from the start to the end about `centre`,
    /// positive counterclockwise, with 0 < |Δθ| ≤ 2π (D1; a full turn
    /// is |Δθ| = 2π); the arc's parameter span is `|sweep|`.
    pub sweep: T,
}

impl<T: Real> Arc2<T> {
    /// The same arc read at another scalar: `f` applied to the centre's
    /// coordinates, then the radius, then the sweep. A structural map —
    /// no arithmetic, so it is exact whenever `f` is.
    #[must_use]
    pub fn map<U: Real>(self, f: impl Fn(T) -> U) -> Arc2<U> {
        Arc2 {
            centre: self.centre.map(&f),
            radius: f(self.radius),
            sweep: f(self.sweep),
        }
    }

    /// The same carrier traversed the other way: the sweep negated
    /// (as `0 − sweep`), the centre and radius untouched.
    #[must_use]
    pub fn reversed(self) -> Self {
        Self {
            sweep: T::zero() - self.sweep,
            ..self
        }
    }

    /// The point at normalized parameter `s ∈ [0, 1]` of this arc
    /// started at `a`: `a` rotated about `centre` by `s·sweep` — over
    /// the reals `centre + radius·(cos, sin)(θ₀ + s·sweep)`, θ₀ the
    /// start angle. Reads `centre` and `sweep`, never `radius`. Fixed
    /// orders as written (D9); total — degenerate data yields poison
    /// values.
    ///
    /// **The start is exact and the end is not, by choice.** At `s = 0`
    /// the rotation term is identically zero and `a` comes back as
    /// given (at `f64`, bit for bit); at `s = 1` the result is `a`
    /// turned by the whole sweep, which is the end over the reals and
    /// within the rotation's rounding of it here. A form exact at both
    /// ends exists without any comparison — the blend
    /// `(1 − s)·rot_a(s·Δθ) + s·rot_b((s − 1)·Δθ)`, anchored on both
    /// endpoints — and it is not the one used: it evaluates two
    /// rotations and sums their enclosures, where the anchored form
    /// below carries one, and it builds a second rotation's nodes at
    /// every `Sym` sample. Endpoint authority is held by the caller's
    /// endpoints, never this evaluation
    /// (`work/paths/sketch-segment-eval-could-be-exact-at-both-ends.md`).
    ///
    /// **The rotation is anchored on `a`, not on the centre**: the
    /// evaluated form is `a + (R − I)·v` (v = a − centre, R the
    /// rotation by s·sweep), which is the identity `centre + R·v` over
    /// the reals but does not mention `centre` outside a factor that
    /// vanishes with the rotation. `cos − 1` is spelled
    /// `−2·sin²(s·sweep/2)` so it carries no cancellation of its own.
    /// The centre-anchored form adds and subtracts `centre`, and at
    /// `T = Interval` that cancellation does not happen: the enclosure
    /// pays `width(centre)` twice, and a centre derived from a short
    /// chord carries the chord's relative width amplified by the
    /// radius — a factor ∝ 1/sin(θ/2), unbounded for short arcs, which
    /// a caller storing evaluated points back as endpoints would
    /// compound. The anchored form is exactly `width(a)` wide at s = 0
    /// (R − I is identically zero there), never wider than the
    /// centre-anchored form at s = 0, and tighter wherever `|s·sweep|`
    /// is small, because `|R − I| = 2·|sin(s·sweep/2)|` scales the
    /// centre's width down instead of doubling it.
    pub fn point_from(self, a: Point2<T>, s: T) -> Point2<T> {
        let Self { centre, sweep, .. } = self;
        let half = T::from_f64(0.5);
        let two = T::from_f64(2.0);
        let sin = (s * sweep).sin();
        // cos(s·Δθ) − 1, in the half-angle form that is exact
        // at s = 0 and free of the 1 − cos cancellation.
        let cos_m1 = -(two * (s * sweep * half).sin().powi(2));
        let v = a - centre;
        a + Vec2::new(v.x * cos_m1 - v.y * sin, v.x * sin + v.y * cos_m1)
    }

    /// The distance from `p` to the carrier's centre, `‖p − centre‖`:
    /// the rim at `p`, which is `radius` exactly when `p` lies on the
    /// carrier. The one spelling of it, so a consumer that asks about
    /// an endpoint's rim builds the node a registrant stated
    /// ([`Arc2::register_endpoints`]).
    pub fn rim(self, p: Point2<T>) -> T {
        (p - self.centre).norm()
    }

    /// Where this arc started at `a` ends: [`Arc2::point_from`] at
    /// `s = 1`. For a consistent arc from `a` to `b` it is `b` over the
    /// reals; the one spelling of that landing, for the same reason as
    /// [`Arc2::rim`].
    pub fn landing(self, a: Point2<T>) -> Point2<T> {
        self.point_from(a, T::one())
    }

    /// **Registers the four endpoint facts** of this arc between `a`
    /// and `b` ([`Real::register_equal`]): the rim at each end is the
    /// radius, the landing from `a` is `b`, and the reversed arc's
    /// landing from `b` is `a` — the last two per component, which is
    /// what a consumer's `distance` asks of them. Each derived node is
    /// registered against the held one (`rim` against `radius`, a
    /// landing against its endpoint), and each answer is handed back
    /// with the fact it states, for the caller to handle by arm.
    ///
    /// **An axiom, not a check.** The door's witness cannot tell an
    /// identity from a coincidence, so this is sound only where the
    /// CALLER built the arc so that the four facts hold over the reals
    /// at every value of its inputs, and its doc comment carries that
    /// proof. The type states the facts in its own spelling
    /// ([`Arc2::rim`], [`Arc2::landing`], [`Arc2::reversed`]) and
    /// proves none of them.
    #[must_use = "a registration can be REFUSED, and a refusal a caller \
                  drops is a lie nobody sees"]
    pub fn register_endpoints(
        self,
        a: Point2<T>,
        b: Point2<T>,
        tol: Tol,
    ) -> [(&'static str, SymRegistration); 6] {
        let forward = self.landing(a);
        let backward = self.reversed().landing(b);
        [
            (
                "the rim at the start",
                self.rim(a).register_equal(self.radius, tol),
            ),
            (
                "the rim at the end",
                self.rim(b).register_equal(self.radius, tol),
            ),
            ("the landing's x", forward.x.register_equal(b.x, tol)),
            ("the landing's y", forward.y.register_equal(b.y, tol)),
            (
                "the reversed landing's x",
                backward.x.register_equal(a.x, tol),
            ),
            (
                "the reversed landing's y",
                backward.y.register_equal(a.y, tol),
            ),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Bounds, Interval};

    fn quarter() -> Arc2<f64> {
        Arc2 {
            centre: Point2::new(0.25, -0.5),
            radius: 1.5,
            sweep: 0.75,
        }
    }

    #[test]
    fn reversed_negates_the_sweep_and_keeps_the_carrier() {
        let (arc, rev) = (quarter(), quarter().reversed());
        assert_eq!(
            (rev.centre.x, rev.centre.y, rev.radius, rev.sweep),
            (arc.centre.x, arc.centre.y, arc.radius, -arc.sweep),
        );
    }

    #[test]
    fn map_carries_each_field_to_its_own_place() {
        let lifted = quarter().map(Interval::from_f64);
        let at = |i: Interval| (i.lo(), i.hi());
        assert_eq!(at(lifted.centre.x), (0.25, 0.25), "centre.x");
        assert_eq!(at(lifted.centre.y), (-0.5, -0.5), "centre.y");
        assert_eq!(at(lifted.radius), (1.5, 1.5), "radius");
        assert_eq!(at(lifted.sweep), (0.75, 0.75), "sweep");
    }

    /// The start comes back bit for bit, and the far end is the start
    /// turned by the whole sweep about the centre.
    #[test]
    fn point_from_is_exact_at_the_start_and_turns_by_the_sweep() {
        let arc = quarter();
        let a = arc.centre + Vec2::new(arc.radius, 0.0);
        let p0 = arc.point_from(a, 0.0);
        assert_eq!((p0.x, p0.y), (a.x, a.y), "s = 0 returns the start");
        let p1 = arc.point_from(a, 1.0);
        let want = arc.centre + Vec2::new(arc.sweep.cos(), arc.sweep.sin()) * arc.radius;
        assert!(p1.distance(want) < 1e-15, "s = 1: {p1:?} vs {want:?}");
    }

    /// The first quarter of the unit circle about the origin, from
    /// (1, 0) to (0, 1): every coordinate and the sweep enclose their
    /// reals, so the four endpoint facts hold on the enclosures.
    fn quarter_circle<T: Real>() -> (Arc2<T>, Point2<T>, Point2<T>) {
        let arc = Arc2 {
            centre: Point2::new(T::zero(), T::zero()),
            radius: T::one(),
            sweep: T::pi() / T::from_f64(2.0),
        };
        let (a, b) = (
            Point2::new(T::one(), T::zero()),
            Point2::new(T::zero(), T::one()),
        );
        (arc, a, b)
    }

    /// **A planted lie is refused by the exact witness, typed.** The
    /// quarter circle's facts meet at `Interval`; a radius off by a half
    /// is disjoint from both rims and nothing else, and swapping the
    /// ends sends both landings to the wrong vertex.
    #[test]
    fn register_endpoints_refuses_a_planted_lie_at_the_exact_witness() {
        let answers = |arc: Arc2<Interval>, a, b| {
            arc.register_endpoints(a, b, Tol::witness())
                .map(|(_, answer)| answer)
        };
        let (arc, a, b) = quarter_circle::<Interval>();
        assert!(
            answers(arc, a, b)
                .iter()
                .all(|&r| r == SymRegistration::Witnessed),
            "a consistent arc is witnessed on every fact"
        );
        let wide = Arc2 {
            radius: Interval::from_f64(1.5),
            ..arc
        };
        let got = answers(wide, a, b);
        assert_eq!(
            got[..2],
            [SymRegistration::Contradicted; 2],
            "both rims against a planted radius"
        );
        assert!(
            got[2..].iter().all(|&r| r == SymRegistration::Witnessed),
            "the landings never read the radius: {got:?}"
        );
        let got = answers(arc, b, a);
        assert_eq!(
            [got[2], got[4]],
            [SymRegistration::Contradicted; 2],
            "a sweep that turns the wrong end onto the other: {got:?}"
        );
    }

    /// **A registered rim discharges through the Sym tier**, as a
    /// registered identity and inside a larger expression too.
    #[test]
    fn a_registered_rim_decides_zero_through_the_tier() {
        use crate::predicate::{Band, Sign};
        use crate::sym::with_session_rules;
        use crate::{Decide, ParamSymbol, Sym, SymBudget, SymRules};
        let budget = SymBudget {
            max_terms: 4096,
            max_degree: 128,
        };
        let band = Band::linear(Tol::witness()).expect("the witness band");
        let ((after, inside), counts) = with_session_rules(budget, SymRules::shipped(), || {
            let rho = Sym::param(ParamSymbol::of("rho"), Interval::from_bounds(1.0, 1.25));
            let zero = <Sym<Interval> as Real>::zero();
            let arc = Arc2 {
                centre: Point2::new(zero, zero),
                radius: rho,
                sweep: <Sym<Interval> as Real>::pi(),
            };
            let (a, b) = (Point2::new(rho, zero), Point2::new(zero - rho, zero));
            let sign = |m: Sym<Interval>| m.sign_within(band).map(|d| d.sign).ok();
            for (what, answer) in arc.register_endpoints(a, b, Tol::witness()) {
                assert!(
                    matches!(answer, SymRegistration::Recorded | SymRegistration::Already),
                    "{what}: {answer:?}"
                );
            }
            let after = sign(arc.rim(a) - arc.radius);
            let inside = sign((arc.rim(a) + rho) * rho - (arc.radius + rho) * rho);
            (after, inside)
        });
        assert_eq!(after, Some(Sign::Zero), "the registered rim: {counts:?}");
        assert_eq!(
            inside,
            Some(Sign::Zero),
            "inside a larger expression: {counts:?}"
        );
        assert!(counts.registered >= 2, "counted as registered: {counts:?}");
    }
}
