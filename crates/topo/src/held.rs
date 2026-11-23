//! **A point its caller holds on a face's carrier**, and what holds it
//! there.
//!
//! A placement reads a point against a face's region; whether the point
//! is on the face's carrier at all is a separate question, decided where
//! the caller does not already know it (`contain`'s
//! `face_placement`, and the unheld placement beside it, where an
//! `OffCarrier` is a reading). A [`Held`] point is one the caller
//! knows: its construction put it there, or the caller decided it.
//! Each constructor names that premise, and the placement checks it in
//! debug builds rather than deciding it again: a probe of every
//! trusting site over topo, sweep and editor-core at both ε read none
//! of about 51,000 calls off its carrier, `Interval` among them
//! (`work/onecopy/carrier-held-points-are-trusted-not-decided.md`), so
//! a decision would add samples and no information, and where a held
//! point's enclosure is too wide to read `Zero` ([`Held::check`]) it
//! could only refuse a point the construction holds.

use geom::Surface;
use geom_core::{Band, Decide, Margin, MarginDiag, MarginKind, Point3, Sign};

/// **A point held on a face's carrier**, with the premise that holds it
/// ([`Held::constructed`], [`Held::decided`], [`Held::in_band`]).
#[derive(Clone, Copy, Debug)]
pub(crate) struct Held<T: geom_core::Real> {
    at: Point3<T>,
    premise: Premise,
}

/// What holds a [`Held`] point on its face's carrier.
#[derive(Clone, Copy, Debug)]
enum Premise {
    /// The caller's construction, named.
    Constructed(&'static str),
    /// The caller decided the point's margin from the carrier `Zero`.
    Decided(MarginDiag),
    /// The caller read the margin in band and goes on to place the point,
    /// refusing the coincidence after: it is not definitely off.
    InBand,
}

impl<T: Decide> Held<T> {
    /// A point the caller's construction put on the face's carrier:
    /// `by` names that construction ("a vertex of this face", "the foot
    /// of the ball on the face's carrier"), and the call says why it
    /// holds. Not for a point found near the face, which is not held.
    pub(crate) fn constructed(at: Point3<T>, by: &'static str) -> Self {
        Self {
            at,
            premise: Premise::Constructed(by),
        }
    }

    /// A point whose margin from the face's carrier the caller decided
    /// `Zero`, the margin it decided on beside it.
    pub(crate) fn decided(at: Point3<T>, margin: MarginDiag) -> Self {
        Self {
            at,
            premise: Premise::Decided(margin),
        }
    }

    /// A point whose margin from the face's carrier the caller read in
    /// band, and places all the same: not definitely off it.
    /// `boolean::vtxfac`'s held reading, which refuses the coincidence
    /// after; and the census's `meets_at`, whose point lies on another
    /// face's carrier, one with this face's within Door 1's levered
    /// carrier-equality band.
    pub(crate) fn in_band(at: Point3<T>) -> Self {
        Self {
            at,
            premise: Premise::InBand,
        }
    }

    /// The point.
    pub(crate) fn point(self) -> Point3<T> {
        self.at
    }

    /// **The premise, checked in debug builds**: the point's implicit
    /// residual against `surface` ([`geom_brep::implicit_residual`]),
    /// classified at `band` outside the log (F38), places the point within
    /// the zero band of the carrier for a constructed or decided point,
    /// and never definitely off it for one held in band. Skipped only
    /// where the residual is honestly unavailable: a carrier with no
    /// implicit form, a spline or an approximated one (not
    /// [`geom_brep::is_analytic`]), or a poison residual.
    ///
    /// **What each read can show.** The residual is a length, the point's
    /// offset from the carrier (linearized on a curved one), read against
    /// the band's own lengths, so no angular arm scales it. The true offset
    /// of a constructed point is its construction's rounding, and of a
    /// decided one inside the zero band; the two kinds of read stand for it
    /// differently.
    ///
    /// - A point margin (`f64`) is the offset to within the residual's
    ///   rounding, which grows with the coordinates and is bounded by the
    ///   session box (D4 ¶4, [`geom_core::tolerance::SESSION_BOX_EXTENT`],
    ///   1e3 m) at about `1e3 · 2⁻⁵²` m, 2e-13, below the zero band at
    ///   every ratified ε, so it reads `Zero`. A decided point's margin is
    ///   the same offset from the same frame, re-read: bit for bit at
    ///   `vtxfac`'s two arms and `reduce`'s curved one (up to its sign), and
    ///   by another evaluation at `reduce`'s planar arm
    ///   ([`geom::plane_offset_rounded`]), whose margin decided within the
    ///   two evaluations' rounding of `±zero` can re-read just past it.
    /// - An enclosure (`Interval`) holds the true offset, but its width is
    ///   not that rounding. The point and the carrier are enclosures of one
    ///   construction (a station is the carrier's own frame carried through
    ///   its chart), and the residual's evaluation forgets that they move
    ///   together, so the width grows with the coordinates and with the
    ///   carrier's own enclosure, past the zero band inside the box (a
    ///   station of the void's wall on `sweep`'s plate built
    ///   `(100, 100, 100)` off the origin, measured in
    ///   `work/charttail/a-station-far-from-the-origin-reads-indeterminate-on-its-own-carrier-and-panics.md`).
    ///   So an enclosure holds the premise where it meets `[−zero, zero]`,
    ///   read at the band whose zero and escalate edges are both `zero`:
    ///   one that lies wholly past it, however near, does not.
    ///
    /// An uncertified read ([`geom_core::MarginKind::Invalid`] on a residual
    /// that is not poison, a domain violation in its evaluation) does not
    /// hold a constructed or decided premise, which its point should meet
    /// exactly; a point held in band only asks not to be definitely off,
    /// and an uncertified read is not.
    ///
    /// # Panics
    ///
    /// In debug builds, where the premise does not hold: a construction
    /// that no longer puts its point on the carrier, a defect.
    #[track_caller]
    pub(crate) fn check(self, surface: &Surface<T>, band: Band) {
        if !cfg!(debug_assertions) || !geom_brep::is_analytic(surface) {
            return;
        }
        let residual = geom_brep::implicit_residual(surface, self.at);
        if residual.is_poison() {
            return;
        }
        let read = geom_core::k_stats::check_unlogged(
            "held_on_carrier",
            Margin::of(residual).value(),
            band,
            "F38",
        );
        let holds = match (self.premise, &read) {
            (_, Ok(Sign::Positive | Sign::Negative)) => false,
            (_, Ok(Sign::Zero)) | (Premise::InBand, Err(_)) => true,
            (Premise::Constructed(_) | Premise::Decided(_), Err(undecided)) => {
                // The kind stands for what the read's distance from the
                // true offset is: a rounding bound below the zero band
                // (`Value`, which had to read `Zero`), or an enclosure of
                // any width holding it, which holds where it meets the band.
                match undecided.margin.kind() {
                    MarginKind::Enclosure => !matches!(
                        geom_core::k_stats::check_unlogged(
                            "held_on_carrier",
                            Margin::of(residual).value(),
                            zero_edge(band),
                            "F38",
                        ),
                        Ok(Sign::Positive | Sign::Negative)
                    ),
                    MarginKind::Value | MarginKind::Invalid => false,
                }
            }
        };
        if !holds {
            let premise = match self.premise {
                Premise::Constructed(by) => format!("constructed as {by}"),
                Premise::Decided(margin) => format!("decided Zero at {margin:?}"),
                Premise::InBand => "read in band".to_string(),
            };
            debug_assert!(
                holds,
                "a point held on its face's carrier ({premise}) reads {read:?} against it"
            );
        }
    }
}

/// The band whose zero and escalate edges both stand at `band`'s zero:
/// a read is definite at it exactly where it lies wholly outside
/// `[−zero, zero]`.
fn zero_edge(band: Band) -> Band {
    match Band::new(band.zero(), band.zero().next_up()) {
        Ok(edge) => edge,
        // A valid band's zero is finite and positive, so its next float
        // is finite and above it: the constructor cannot refuse.
        Err(_) => unreachable!("a valid band's zero edge makes a band"),
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::Held;
    use geom::Surface;
    use geom_core::k_stats::check_unlogged;
    use geom_core::{Band, Interval, Margin, Point3, Real, Sign, Tol};

    /// The band at ε = 1e-12, the row whose zero band a far station's
    /// enclosure outgrows.
    fn band() -> Band {
        Band::linear_at(Tol::witness(), 1e-12).unwrap()
    }

    /// A wall of the void of `sweep`'s plate built `o` off the origin: a
    /// Newell plane through an apex and two base corners, copied from
    /// `void_touching_plate_at`
    /// (`crates/sweep/tests/self_intersection_proves_injectivity.rs`),
    /// which points back here.
    fn wall_at<T: geom_core::Decide>(o: f64) -> Surface<T> {
        let corners = [[1.9, 0.9, 0.5], [1.4, 1.3, 0.5], [1.5, 1.0, 1.0]].map(|[x, y, z]| {
            Point3::new(T::from_f64(x + o), T::from_f64(y + o), T::from_f64(z + o))
        });
        geom_brep::newell_plane(&corners, corners[1] - corners[0], band()).unwrap()
    }

    /// The wall of the plate built `(100, 100, 100)` off the origin, where
    /// its frame's enclosure is about 1.8e-12 wide.
    fn wall<T: geom_core::Decide>() -> Surface<T> {
        wall_at(100.0)
    }

    const STATIONS: [(f64, f64); 5] = [
        (0.0, 0.0),
        (0.3, 0.1),
        (-0.2, 0.25),
        (0.15, -0.3),
        (-0.3, -0.2),
    ];

    /// **A station of a far wall's own chart holds at `Interval` where its
    /// enclosure is too wide to read `Zero`.** Red where the check asks a
    /// constructed point's enclosure for `Zero`, or to lie inside the zero
    /// band: the stations that read undecided at 1e-12 panic.
    #[test]
    fn a_far_station_whose_enclosure_straddles_the_zero_band_holds() {
        let wall = wall::<Interval>();
        let mut undecided = 0;
        for (u, v) in STATIONS {
            let station = wall.eval_at(u, v);
            let residual = geom_brep::implicit_residual(&wall, station);
            let read = check_unlogged("held_test", Margin::of(residual).value(), band(), "F38");
            assert!(
                !matches!(read, Ok(Sign::Positive | Sign::Negative)),
                "station ({u}, {v}) reads definitely off its own carrier: {read:?}"
            );
            undecided += usize::from(read.is_err());
            Held::constructed(station, "a station of the wall's own chart").check(&wall, band());
        }
        assert!(
            undecided > 0,
            "some station reads undecided, so the row asks the enclosure arm"
        );
    }

    /// **An `Interval` point definitely off its carrier still fails the
    /// check**: the far wall's station moved 1e-10 along the normal, past
    /// the escalate band and its own enclosure. Green where the check
    /// passes any undecided read and a definite one alike.
    #[test]
    #[cfg_attr(
        debug_assertions,
        should_panic(expected = "(constructed as a point off the wall) reads Ok")
    )]
    fn an_interval_point_off_its_carrier_fails_the_check() {
        let wall = wall::<Interval>();
        let Surface::Plane { frame, .. } = wall else {
            unreachable!("a Newell wall is a plane")
        };
        let off = wall.eval_at(0.3, 0.1) + frame.axis() * Interval::from_f64(1e-10);
        Held::constructed(off, "a point off the wall").check(&wall, band());
    }

    /// **An `f64` point that reads undecided fails a constructed
    /// premise**: a point margin is the residual's rounding, below the
    /// zero band in the session box, so a constructed point 3e-12 off the
    /// wall at ε = 1e-12 is a defect. Green where an undecided point margin
    /// passes as an enclosure does.
    #[test]
    #[cfg_attr(
        debug_assertions,
        should_panic(expected = "(constructed as a point near the wall) reads Err")
    )]
    fn an_f64_point_that_reads_undecided_fails_the_check() {
        let wall = wall::<f64>();
        let Surface::Plane { frame, .. } = wall else {
            unreachable!("a Newell wall is a plane")
        };
        let near = wall.eval_at(0.3, 0.1) + frame.axis() * 3e-12;
        Held::constructed(near, "a point near the wall").check(&wall, band());
    }

    /// **An `Interval` point a few ε off its carrier fails the check
    /// though it reads undecided**: a station of the wall built at the
    /// origin, whose enclosure is tight, moved 3e-12 along the normal
    /// reads inside the sliver `(zero, escalate)`, wholly past the zero
    /// band. Green where any undecided enclosure passes.
    #[test]
    #[cfg_attr(
        debug_assertions,
        should_panic(expected = "(constructed as a point a few eps off the wall) reads Err")
    )]
    fn an_interval_point_a_few_eps_off_its_carrier_fails_the_check() {
        let wall = wall_at::<Interval>(0.0);
        let Surface::Plane { frame, .. } = wall else {
            unreachable!("a Newell wall is a plane")
        };
        let off = wall.eval_at(0.3, 0.1) + frame.axis() * Interval::from_f64(3e-12);
        let residual = geom_brep::implicit_residual(&wall, off);
        let read = check_unlogged("held_test", Margin::of(residual).value(), band(), "F38");
        assert!(
            read.is_err(),
            "the point reads undecided, not definite: {read:?}"
        );
        Held::constructed(off, "a point a few eps off the wall").check(&wall, band());
    }

    /// A station of the far wall whose `x` carries a domain violation (a
    /// square root clamped at zero, decoration `Trv`): its residual is not
    /// poison, and reads uncertified.
    fn uncertified_station() -> (Surface<Interval>, Point3<Interval>) {
        let wall = wall::<Interval>();
        let at = wall.eval_at(0.3, 0.1);
        let violated = Interval::from_bounds(-1.0, 4.0).sqrt().powi(0) - Interval::from_f64(1.0);
        (wall, Point3::new(at.x + violated, at.y, at.z))
    }

    /// **An uncertified read does not hold a constructed premise.** Green
    /// where an uncertified enclosure passes as an undecided one does.
    #[test]
    #[cfg_attr(
        debug_assertions,
        should_panic(expected = "(constructed as an uncertified station) reads Err")
    )]
    fn an_uncertified_read_fails_a_constructed_premise() {
        let (wall, at) = uncertified_station();
        Held::constructed(at, "an uncertified station").check(&wall, band());
    }

    /// **An uncertified read holds a point held in band**: it is not
    /// definitely off. Red where the in-band premise asks for a certified
    /// read.
    #[test]
    fn an_uncertified_read_holds_a_point_held_in_band() {
        let (wall, at) = uncertified_station();
        Held::in_band(at).check(&wall, band());
    }
}
