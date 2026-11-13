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
//! # The singular set
//!
//! A joint whose vertex is on the chart's singular set — a sphere's
//! pole, a cone's apex, a spindle sheet's apex — is a side of the
//! face's region in the chart: the whole singular latitude is the one
//! point, and the boundary runs along it from the arriving image's
//! exit to the leaving image's entry. Its element is an integer deck
//! element like every other, the one that puts the leaving image's
//! entry **the face's turn** past the arriving image's exit
//! (`crate::pcurves`' `decide_joint`, `singular_turn`). The turn is the
//! angle about the chart's axis from the arriving edge's tangent at
//! the point to the leaving edge's, swept through the face: in
//! `(0, τ)` or `(−τ, 0)` by the face's sense and the side of the
//! singular latitude the face is on, and a whole turn where the loop
//! leaves along the edge it arrived on. The tangents are read off the
//! edges' carriers; an image's azimuth at the point is its edge's
//! tangent azimuth, or half a turn off it on the chart's twin sheet,
//! which the element's twin bit takes up.
//!
//! So a loop's lift runs on through the point as the face's boundary,
//! and the element there is a function of the two edges, the chart and
//! the face's sense: a door that moves a loop to a face of the opposite
//! sense on one chart decides it again, and a loop reversal inverts it.
//! One case reads more: two distinct edges that leave the point along
//! one direction (a slit's tip) turn by nothing or by a whole turn, and
//! which is read off the winding of the rest of their loop.
//!
//! Every stored element records a decision. A joint whose element is
//! not decided past the band stores none: its door refuses
//! (`crate::pcurves::PcurveMintError`).
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
//! exit. On the singular set the two turns either side of a killed
//! strut sum to the turn across both: the strut lies in the face's
//! sector, and splits it.

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
/// half-edge's image onto the end of its predecessor's image.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct JointElement(pub Deck);

impl JointElement {
    /// The identity joint: two images that already meet.
    pub const IDENTITY: Self = Self(Deck::IDENTITY);

    /// The element as a deck transformation.
    #[must_use]
    pub fn deck(self) -> Deck {
        self.0
    }

    /// The element of the joint a kill bridges: `self` the element into
    /// the first killed half, `next` the element out of the last
    /// (module docs, "Kills are sums").
    #[must_use]
    pub fn then(self, next: Self) -> Self {
        Self(self.0.compose(next.0))
    }

    /// The element of the same joint read from the other side, as a loop
    /// reversal reads it ([`crate::Body::revert`]): the inverse.
    #[must_use]
    pub fn inverse(self) -> Self {
        Self(self.0.inverse())
    }

    /// The lift of the half-edge after this joint, from the lift `prev`
    /// of the half-edge before it: `prev · e`.
    #[must_use]
    pub fn follow(self, prev: Deck) -> Deck {
        prev.compose(self.0)
    }
}

/// **How far a loop winds**: the elements of its joints composed once
/// around, read as the closure reads them.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Winding {
    /// The composed element.
    pub deck: Deck,
}

impl Winding {
    /// The winding of a loop whose joints, in cycle order from any
    /// member, carry `elements`.
    pub fn of(elements: impl IntoIterator<Item = JointElement>) -> Self {
        let deck = elements
            .into_iter()
            .fold(Deck::IDENTITY, |deck, element| deck.compose(element.deck()));
        Self { deck }
    }

    /// **Whether the loop closes**: it winds the chart's first channel
    /// at most one period, its second channel at most one period where
    /// the chart is closed in it, and on a sphere it may close through
    /// the involution (a loop over a pole that meets its start on the
    /// twin representation). Read off the composed element's
    /// conjugation invariants — `twin`, `u` plus half a period per twin,
    /// and `|v|` off the twin — so the answer does not depend on which
    /// member the composition starts from.
    #[must_use]
    pub fn closes(self) -> bool {
        let Deck { u, v, twin } = self.deck;
        if twin {
            // `u + ½` periods is the invariant azimuth advance, so `u` in
            // `−2..=1` admits an advance of `±π` or `±3π`: the twin's own
            // half turn, plus at most one period either way.
            (-2..=1).contains(&u)
        } else {
            (-1..=1).contains(&u) && (-1..=1).contains(&v)
        }
    }

    /// Whether the loop winds nothing: it closes on its start exactly,
    /// the only loop a closed chart polygon describes.
    #[must_use]
    pub fn is_zero(self) -> bool {
        self.deck == Deck::IDENTITY
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
        let elements: Vec<JointElement> = all().into_iter().map(JointElement).collect();
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

    /// A kill's sum is the composition along the bridged path.
    #[test]
    fn a_kill_sums_its_two_elements() {
        let a = JointElement(Deck {
            u: 1,
            v: 0,
            twin: false,
        });
        let b = JointElement(Deck {
            u: -1,
            v: 1,
            twin: false,
        });
        assert_eq!(
            a.then(b),
            JointElement(Deck {
                u: 0,
                v: 1,
                twin: false
            })
        );
        for deck in all() {
            let e = JointElement(deck);
            assert_eq!(e.inverse().inverse(), e, "{e:?}");
            assert_eq!(e.then(e.inverse()), JointElement::IDENTITY, "{e:?}");
        }
    }

    /// **Which windings close**, by value: a loop advances its azimuth at
    /// most one period, its second channel at most one period, and
    /// through the twin by its half turn plus at most one period either
    /// way.
    #[test]
    fn the_closure_admits_one_period_either_way_and_no_more() {
        let winding = |u, v, twin| {
            Winding {
                deck: Deck { u, v, twin },
            }
            .closes()
        };
        for u in -2..=1 {
            assert!(winding(u, 0, true), "twin u = {u} closes");
        }
        for u in [-3, 2] {
            assert!(!winding(u, 0, true), "twin u = {u} does not close");
        }
        for u in -1..=1 {
            assert!(winding(u, 0, false), "u = {u} closes");
        }
        for u in [-2, 2] {
            assert!(!winding(u, 0, false), "u = {u} does not close");
        }
        for v in [-2, 2] {
            assert!(!winding(0, v, false), "v = {v} does not close");
        }
        assert!(winding(0, 1, false), "v = 1 closes");
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
