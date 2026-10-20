//! **Joint elements**: the integer half of a pcurve row (C4).
//!
//! A face's row on a half-edge is two things. The **image** is the
//! edge's certified chart curve, a function of the edge and the chart
//! alone ([`crate::Body::pcurve`]). The **joint element** is the deck
//! transformation that carries the half-edge's image onto the end of
//! the image of the half-edge before it in its loop
//! ([`crate::Body::joint`]): a whole number of first-channel periods
//! (azimuth periods, a net's `u` knot lengths), a whole number of
//! second-channel periods where the chart is closed in that channel (a
//! torus's and a sphere's second angle, a net's `v` knot length), and
//! on a sphere the involution twin. A loop's lift — the chain of chart
//! curves a reader draws — is the images moved by the elements summed
//! along the loop ([`crate::Body::loop_lift`]), so no stored byte
//! depends on which half-edge is the loop's `first`.
//!
//! # The deck group
//!
//! A [`Deck`] `(u, v, twin)` is `T_u^u · T_v^v · σ^twin`, where `T_u`
//! moves the first chart channel one period, `T_v` the second (an angle
//! on the sphere and the torus, the knot domain on a net closed in `v`), and `σ` is the sphere chart's
//! involution `(u, v) ↦ (u + π, π − v)`, under which the sphere's map
//! is invariant. The relations are the chart's own:
//!
//! - `σ² = T_u` (two half turns are a period);
//! - `σ · T_v = T_v⁻¹ · σ` (the involution mirrors the second channel);
//! - `T_u` commutes with everything.
//!
//! So every element has the one normal form `(u, v, twin)`, and
//! [`Deck::compose`] and [`Deck::inverse`] are exact integer
//! arithmetic. On a chart with no involution `twin` is always `false`,
//! and on a chart not closed in its second channel `v` is always 0.
//!
//! # The reset marker
//!
//! A reset is written at a joint in exactly two cases
//! (`crate::pcurves`' `decide_joint`):
//!
//! - **on the chart's singular set** — a sphere's pole, a cone's apex,
//!   or a spline chart whose first-channel knot domain moves a point
//!   under the band — where the first channel has no lever: every
//!   azimuth names the same point, so no whole number of azimuth periods
//!   is decided there;
//! - **on a spline chart whose first-channel gate is undecided** (its
//!   `u` sweep in the band's sliver, `AzimuthRule::SplineUndecided`),
//!   where no azimuth integer is claimed and the joint's chart-space gap
//!   is what keeps the joint sound.
//!
//! An undecided SECOND-channel gate (a net closed in `v` whose `v` sweep
//! sits in the band's sliver) writes no reset: the joint refuses
//! (`crate::pcurves::PcurveMintError::NetStretchUndecided`), since a reset
//! quotients out the first channel, not the second.
//!
//! Such a joint stores [`JointElement::Reset`], which carries only the
//! part of the element that IS decided (the second-channel periods and
//! the twin, the element modulo `T_u`; `T_u` is central, so that
//! quotient is a group). The joint decision writes a reset with no twin
//! (`crate::pcurves`' `decide_joint`: at a pole either sheet names the
//! point, so its orbit integer is 0); a kill's sum
//! ([`JointElement::then`]) may carry one. The lift restarts the first channel after a
//! reset: the next image is placed with no azimuth periods
//! ([`JointElement::follow`]), and a loop's closure counts no azimuth
//! wrap across a reset ([`Winding`]). So the pass and the at-rest
//! check read a pole joint alike, and neither reads which half-edge is
//! `first`.
//!
//! # Kills are sums
//!
//! An edge's two halves on one face share one image, traversed in
//! opposite senses, so the exit of one half's image is the entry of
//! the other's, exactly. A kill that bridges `p → x … y → n` over the
//! killed halves writes `e(x) · e(n)` on the new joint
//! ([`JointElement::then`]); the joint between the killed halves at
//! their shared vertex carries nothing and is skipped. That is the
//! element the pass decides there: both carry `n`'s entry onto `p`'s
//! exit, and away from the singular set there is one such element.

use geom::Surface;
use geom_brep::Pcurve;
use geom_core::Decide;

/// A deck transformation of a chart, in the normal form
/// `T_u^u · T_v^v · σ^twin` (module docs).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct Deck {
    /// Whole periods of the first chart channel.
    pub u: i32,
    /// Whole periods of the second chart channel, where the chart is
    /// closed in it: `τ` on a sphere or torus, the knot domain's length
    /// on a spline net closed in `v`; 0 elsewhere.
    pub v: i32,
    /// The sphere chart's involution `(u, v) ↦ (u + π, π − v)`, applied
    /// first; `false` on every other chart.
    pub twin: bool,
}

impl Deck {
    /// The identity.
    pub const IDENTITY: Self = Self {
        u: 0,
        v: 0,
        twin: false,
    };

    /// `self · rhs`: `rhs` applied first, then `self`.
    #[must_use]
    pub fn compose(self, rhs: Self) -> Self {
        let carry = i32::from(self.twin && rhs.twin);
        let v = if self.twin {
            self.v - rhs.v
        } else {
            self.v + rhs.v
        };
        Self {
            u: self.u + rhs.u + carry,
            v,
            twin: self.twin != rhs.twin,
        }
    }

    /// The inverse: `(u, v)⁻¹ = (−u, −v)`, and `(u, v, σ)⁻¹ = (−u − 1, v,
    /// σ)` since `σ⁻¹ = T_u⁻¹ σ`.
    #[must_use]
    pub fn inverse(self) -> Self {
        if self.twin {
            Self {
                u: -self.u - 1,
                v: self.v,
                twin: true,
            }
        } else {
            Self {
                u: -self.u,
                v: -self.v,
                twin: false,
            }
        }
    }

    /// The element with its first-channel periods dropped: the
    /// representative of its class modulo `T_u` that a reset stores and
    /// the lift restarts from.
    #[must_use]
    pub fn without_u(self) -> Self {
        Self { u: 0, ..self }
    }

    /// `image` moved by this element on `surface`'s chart: the twin
    /// first, then the second-channel periods, then the first-channel
    /// periods, each in the period the chart owns for that channel
    /// (`periods`: `crate::pcurves`' `ChartPeriods`, decided at a band
    /// where the caller has one). A spline chart's periods are its knot
    /// domains' lengths (an element with `u ≠ 0` or `v ≠ 0` is decided
    /// only on a net closed in that channel); an analytic chart's first
    /// is `τ`, and its second `τ` where `v` is an angle (sphere, torus).
    ///
    /// `None` where the element does not apply to the image: a twin off
    /// a sphere chart, or on an image the twin has no form for
    /// ([`crate::pcurves::sphere_twin`]); periods in a channel `periods`
    /// gives none (a plane's `u`, a plane's, cylinder's or cone's `v`,
    /// a net's channel it is not decided closed in). No door decides
    /// such an element; one planted by hand is a gap in the lift, never
    /// a silently unmoved curve.
    #[must_use]
    pub(crate) fn apply<T: Decide>(
        self,
        image: &Pcurve<T>,
        surface: &Surface<T>,
        periods: crate::pcurves::ChartPeriods<T>,
    ) -> Option<Pcurve<T>> {
        let mut out = if self.twin {
            crate::pcurves::sphere_twin(surface, image)?
        } else {
            image.clone()
        };
        if self.v != 0 {
            out = crate::pcurves::shift_v_branch(&out, self.v, periods.v()?);
        }
        if self.u != 0 {
            out = out.shift_branch(self.u, periods.u()?);
        }
        Some(out)
    }
}

/// A joint's element (module docs): the deck transformation carrying a
/// half-edge's image onto the end of its predecessor's image, or — at a
/// joint on the chart's singular set — the reset marker.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum JointElement {
    /// An ordinary joint: the whole element.
    Shift(Deck),
    /// A joint where the first chart channel has no lever: the element
    /// modulo `T_u`, its `u` always 0. The lift restarts the first
    /// channel after it.
    Reset(Deck),
}

impl JointElement {
    /// The identity joint: two images that already meet.
    pub const IDENTITY: Self = Self::Shift(Deck::IDENTITY);

    /// The element as a deck transformation: a reset's representative
    /// with no azimuth periods.
    #[must_use]
    pub fn deck(self) -> Deck {
        match self {
            Self::Shift(deck) | Self::Reset(deck) => deck,
        }
    }

    /// The form the body stores: a reset with its azimuth periods
    /// dropped, which it does not carry (module docs), and an ordinary
    /// joint as it is. So [`JointElement::inverse`] is an involution on
    /// every stored element.
    #[must_use]
    pub fn canonical(self) -> Self {
        match self {
            Self::Shift(_) => self,
            Self::Reset(deck) => Self::Reset(deck.without_u()),
        }
    }

    /// Whether this is a reset.
    #[must_use]
    pub fn is_reset(self) -> bool {
        matches!(self, Self::Reset(_))
    }

    /// The element of the joint a kill bridges: `self` the element into
    /// the first killed half, `next` the element out of the last
    /// (module docs, "Kills are sums"). A reset on either side is a
    /// reset: the bridged joint sits at the same vertex, on the singular
    /// set, and the first channel carries nothing through it.
    #[must_use]
    pub fn then(self, next: Self) -> Self {
        let deck = self.deck().compose(next.deck());
        if self.is_reset() || next.is_reset() {
            Self::Reset(deck.without_u())
        } else {
            Self::Shift(deck)
        }
    }

    /// The element of the same joint read from the other side, as a loop
    /// reversal reads it ([`crate::Body::revert`]): the inverse, a reset
    /// staying one.
    #[must_use]
    pub fn inverse(self) -> Self {
        match self {
            Self::Shift(deck) => Self::Shift(deck.inverse()),
            Self::Reset(deck) => Self::Reset(deck.inverse().without_u()),
        }
    }

    /// The lift of the half-edge after this joint, from the lift `prev`
    /// of the half-edge before it: `prev · e`, the first channel
    /// restarted at a reset.
    #[must_use]
    pub fn follow(self, prev: Deck) -> Deck {
        match self {
            Self::Shift(deck) => prev.compose(deck),
            Self::Reset(deck) => prev.compose(deck).without_u(),
        }
    }
}

/// **How far a loop winds**: the elements of its joints composed once
/// around, read as the closure reads them. `azimuth` is `None` where a
/// reset breaks the first channel (a loop through a pole or an apex has
/// no azimuth winding to count).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Winding {
    /// The composed element.
    pub deck: Deck,
    /// Whether any joint of the loop is a reset.
    pub reset: bool,
}

impl Winding {
    /// The winding of a loop whose joints, in cycle order from any
    /// member, carry `elements`.
    pub fn of(elements: impl IntoIterator<Item = JointElement>) -> Self {
        let mut deck = Deck::IDENTITY;
        let mut reset = false;
        for element in elements {
            reset |= element.is_reset();
            deck = deck.compose(element.deck());
        }
        Self { deck, reset }
    }

    /// **Whether the loop closes**: it winds the chart's first channel
    /// at most one period (none counted across a reset), its second
    /// channel at most one period where the chart is closed in it, and on
    /// a sphere it may close through the involution (a loop over a pole
    /// that meets its start on the twin representation). Read off the
    /// composed element's conjugation invariants — `twin`, `u` plus half
    /// a period per twin, and `|v|` off the twin — so the answer does not
    /// depend on which member the composition starts from.
    #[must_use]
    pub fn closes(self) -> bool {
        let Deck { u, v, twin } = self.deck;
        if twin {
            // `u + ½` periods is the invariant azimuth advance, so `u` in
            // `−2..=1` admits an advance of `±π` or `±3π`: the twin's own
            // half turn, plus at most one period either way.
            self.reset || (-2..=1).contains(&u)
        } else {
            (self.reset || (-1..=1).contains(&u)) && (-1..=1).contains(&v)
        }
    }

    /// Whether the loop winds nothing: it closes on its start exactly,
    /// the only loop a closed chart polygon describes.
    #[must_use]
    pub fn is_zero(self) -> bool {
        !self.reset && self.deck == Deck::IDENTITY
    }
}

#[cfg(test)]
mod tests {
    use super::{Deck, JointElement, Winding};

    fn all() -> Vec<Deck> {
        let mut out = Vec::new();
        for u in -2..=2 {
            for v in -2..=2 {
                for twin in [false, true] {
                    out.push(Deck { u, v, twin });
                }
            }
        }
        out
    }

    /// The deck group's laws, over a box of elements: associativity,
    /// identity and inverse, and the chart's relations.
    #[test]
    fn the_deck_group_is_a_group_with_the_charts_relations() {
        let sigma = Deck {
            twin: true,
            ..Deck::IDENTITY
        };
        let tu = Deck {
            u: 1,
            ..Deck::IDENTITY
        };
        let tv = Deck {
            v: 1,
            ..Deck::IDENTITY
        };
        assert_eq!(sigma.compose(sigma), tu, "σ² = T_u");
        assert_eq!(
            sigma.compose(tv),
            tv.inverse().compose(sigma),
            "σ T_v = T_v⁻¹ σ"
        );
        for a in all() {
            assert_eq!(a.compose(Deck::IDENTITY), a);
            assert_eq!(Deck::IDENTITY.compose(a), a);
            assert_eq!(a.compose(a.inverse()), Deck::IDENTITY, "{a:?}");
            assert_eq!(a.inverse().compose(a), Deck::IDENTITY, "{a:?}");
            assert_eq!(a.inverse().inverse(), a);
            for b in all() {
                for c in all() {
                    assert_eq!(a.compose(b).compose(c), a.compose(b.compose(c)));
                }
            }
        }
    }

    /// A loop's closure verdict does not depend on which member the
    /// composition starts from: every rotation of a cycle of elements is
    /// a conjugate of the first, and the verdict reads conjugation
    /// invariants.
    #[test]
    fn the_closure_verdict_is_the_same_from_every_member() {
        let elements: Vec<JointElement> = all().into_iter().map(JointElement::Shift).collect();
        for (i, &a) in elements.iter().enumerate().step_by(3) {
            for &b in elements.iter().skip(i % 7).step_by(5) {
                for &c in elements.iter().skip(i % 5).step_by(7) {
                    let loops = [[a, b, c], [b, c, a], [c, a, b]];
                    let verdicts: Vec<bool> =
                        loops.iter().map(|l| Winding::of(*l).closes()).collect();
                    assert!(
                        verdicts.iter().all(|&v| v == verdicts[0]),
                        "{a:?} {b:?} {c:?}: {verdicts:?}"
                    );
                }
            }
        }
    }

    /// A kill's sum is the composition along the bridged path, and a
    /// reset on either side of it is a reset that keeps the second
    /// channel and the twin.
    #[test]
    fn a_kill_sums_its_two_elements() {
        let a = JointElement::Shift(Deck {
            u: 1,
            v: 0,
            twin: false,
        });
        let b = JointElement::Shift(Deck {
            u: -1,
            v: 1,
            twin: false,
        });
        assert_eq!(
            a.then(b),
            JointElement::Shift(Deck {
                u: 0,
                v: 1,
                twin: false
            })
        );
        let r = JointElement::Reset(Deck {
            u: 0,
            v: 1,
            twin: false,
        });
        assert_eq!(
            r.then(a),
            JointElement::Reset(Deck {
                u: 0,
                v: 1,
                twin: false
            })
        );
        assert_eq!(a.inverse().inverse(), a);
        assert_eq!(r.inverse().inverse(), r);
    }

    /// A reset carries no azimuth periods: its inverse, the form a loop
    /// reversal writes, drops the `T_u` that inverting a twin produces,
    /// so `inverse` is an involution on every stored reset; and the
    /// canonical form the body stores drops a planted one.
    #[test]
    fn a_reset_never_carries_azimuth_periods() {
        for deck in all() {
            let reset = JointElement::Reset(deck.without_u());
            assert_eq!(reset.inverse().deck().u, 0, "inverse of {reset:?}");
            assert_eq!(reset.inverse().inverse(), reset, "{reset:?}");
            assert_eq!(
                JointElement::Reset(deck).canonical(),
                reset,
                "canonical {deck:?}"
            );
            assert_eq!(
                JointElement::Shift(deck).canonical(),
                JointElement::Shift(deck)
            );
        }
    }

    /// **Which windings close**, by value: a loop advances its azimuth at
    /// most one period (none counted across a reset), its second
    /// channel at most one period, and through the twin by its half
    /// turn plus at most one period either way.
    #[test]
    fn the_closure_admits_one_period_either_way_and_no_more() {
        let winding = |u, v, twin, reset| {
            Winding {
                deck: Deck { u, v, twin },
                reset,
            }
            .closes()
        };
        for u in -2..=1 {
            assert!(winding(u, 0, true, false), "twin u = {u} closes");
        }
        for u in [-3, 2] {
            assert!(!winding(u, 0, true, false), "twin u = {u} does not close");
        }
        for u in -1..=1 {
            assert!(winding(u, 0, false, false), "u = {u} closes");
        }
        for u in [-2, 2] {
            assert!(!winding(u, 0, false, false), "u = {u} does not close");
            assert!(winding(u, 0, false, true), "across a reset u = {u} closes");
        }
        for v in [-2, 2] {
            assert!(!winding(0, v, false, false), "v = {v} does not close");
            assert!(
                !winding(0, v, false, true),
                "a reset with v = {v} does not close"
            );
        }
        assert!(winding(0, 1, false, true), "a reset with v = 1 closes");
    }

    /// **An element's `v` periods are the chart's own.** On a net closed
    /// in `v` over a knot domain of length 3 (neither 1 nor `τ`),
    /// `Deck { v: 1 }` moves an image by 3 along `v` and none along `u`;
    /// `Deck { v: −2 }` by −6. On a torus it moves by `τ`. On a cylinder,
    /// whose `v` is a length, the element does not apply. On a net OPEN
    /// in `v` it does not apply either, in the periods a band decides:
    /// the gap the lift reports, where the band-free reading a reader
    /// with no band takes (`ChartPeriods::stated`) moves it by the knot
    /// length (`work/pcert/chart-channel-periods-are-spelled-in-several-places.md`).
    #[test]
    #[allow(clippy::unwrap_used)]
    fn an_elements_v_periods_are_the_charts_own() {
        use crate::pcurves::ChartPeriods;
        use geom::Surface;
        use geom_brep::Pcurve;
        use geom_core::{Band, Point2, Point3, Tol, Vec2, Vec3};
        let band = Band::linear(Tol::witness()).unwrap();
        // `u` the height; `v` across three rows, the last the first's
        // points where `closed`.
        let net = |closed: bool| {
            let ku = geom_core::spline::KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
            let kv =
                geom_core::spline::KnotVector::clamped(vec![0.0, 0.0, 1.5, 3.0, 3.0], 1).unwrap();
            let last = if closed { 0.0 } else { 2.0 };
            let control = [0.0, 1.0]
                .into_iter()
                .flat_map(|z| {
                    [
                        Point3::new(0.0, 0.0, z),
                        Point3::new(1.0, 1.0, z),
                        Point3::new(last, 0.0, z),
                    ]
                })
                .collect();
            Surface::Nurbs(std::sync::Arc::new(
                geom::NurbsSurface::new(ku, kv, control, vec![1.0; 6]).unwrap(),
            ))
        };
        let image = Pcurve::IsoLine {
            p0: Point2::new(0.25, 0.5),
            pl: Vec2::new(0.0, 1.0),
        };
        let moved_in = |deck: Deck, s: &Surface<f64>, periods: ChartPeriods<f64>| {
            deck.apply(&image, s, periods).map(|p| {
                let (a, b) = (image.eval(0.0), p.eval(0.0));
                (b.x - a.x, b.y - a.y)
            })
        };
        let moved = |deck: Deck, s: &Surface<f64>| moved_in(deck, s, ChartPeriods::of(s, band));
        let up = Deck {
            u: 0,
            v: 1,
            twin: false,
        };
        let closed = net(true);
        assert_eq!(moved(up, &closed), Some((0.0, 3.0)), "one knot length");
        assert_eq!(
            moved(Deck { v: -2, ..up }, &closed),
            Some((0.0, -6.0)),
            "two knot lengths back"
        );
        let open = net(false);
        assert_eq!(moved(up, &open), None, "an open net has no v period");
        assert_eq!(
            moved_in(up, &open, ChartPeriods::stated(&open)),
            Some((0.0, 3.0)),
            "the band-free reading states one"
        );
        let torus = Surface::Torus {
            center: Point3::origin(),
            axis: Vec3::unit_z(),
            major_radius: 2.0,
            minor_radius: 0.5,
            u_ref: Vec3::unit_x(),
        };
        assert_eq!(
            moved(up, &torus),
            Some((0.0, core::f64::consts::TAU)),
            "a torus's second angle"
        );
        let cylinder = Surface::Cylinder {
            origin: Point3::origin(),
            axis: Vec3::unit_z(),
            radius: 1.0,
            u_ref: Vec3::unit_x(),
        };
        assert_eq!(moved(up, &cylinder), None, "a cylinder's v is a length");
    }
}
