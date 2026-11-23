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
    /// classified at `band` outside the log (F38), is never definitely
    /// off the carrier, and a constructed or decided point read as a
    /// point margin reads `Zero`. Skipped only where the residual is
    /// honestly unavailable: a carrier with no implicit form, a spline or
    /// an approximated one (not [`geom_brep::is_analytic`]), or a poison
    /// residual.
    ///
    /// **What each read can show.** The residual is a length, the point's
    /// offset from the carrier (linearized on a curved one), read against
    /// the band's own lengths, so no angular arm scales it. A point margin
    /// (`f64`) differs from the true offset by the residual's rounding,
    /// which grows with the coordinates and is bounded by the session box
    /// (D4 ¶4, [`geom_core::tolerance::SESSION_BOX_EXTENT`], 1e3 m) at
    /// about `1e3 · 2⁻⁵²` m, 2e-13, below the zero band at every ratified
    /// ε: a constructed or decided point reads `Zero`. An enclosure
    /// (`Interval`) holds the true offset, which is zero for a constructed
    /// point and inside the zero band for a decided one, but its width is
    /// not that rounding. The point and the carrier are enclosures of one
    /// construction (a station is the carrier's own frame, carried through
    /// its chart), and the residual's evaluation forgets that they move
    /// together, so the width grows with the coordinates and with the
    /// carrier's own enclosure, past the zero band inside the box: a
    /// station of a Newell wall 170 m off the origin reads about ±5e-12,
    /// centred on zero to 1e-14. A correct construction's enclosure
    /// therefore reads `Zero` or undecided, never a definite sign, and only
    /// a definite sign contradicts it.
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
                undecided.margin.kind() == MarginKind::Enclosure
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

    /// A wall of the void of `sweep`'s plate built `(100, 100, 100)` off
    /// the origin: a Newell plane through an apex and two base corners.
    fn wall<T: geom_core::Decide>() -> Surface<T> {
        let corners = [
            [101.9, 100.9, 100.5],
            [101.4, 101.3, 100.5],
            [101.5, 101.0, 101.0],
        ]
        .map(|[x, y, z]| Point3::new(T::from_f64(x), T::from_f64(y), T::from_f64(z)));
        geom_brep::newell_plane(&corners, corners[1] - corners[0], band()).unwrap()
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
    /// constructed point's enclosure for `Zero`: the stations that read
    /// undecided at 1e-12 panic.
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
}
