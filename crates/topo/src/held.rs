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
//! trusting site over topo, sweep and editor-core at both ε read it
//! `Zero` in every one of about 51,000 calls, `Interval` among them
//! (`work/onecopy/carrier-held-points-are-trusted-not-decided.md`), so
//! a decision would add samples and no information.

use geom::Surface;
use geom_core::{Band, Decide, Margin, MarginDiag, Point3, Sign};

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
    /// classified at `band` outside the log (F38), reads `Zero` for a
    /// constructed or decided point, and never definitely off for one
    /// held in band. Skipped only where the residual is honestly
    /// unavailable: a poison residual, which a spline or approximated
    /// carrier always reads, having no implicit form.
    ///
    /// **No lever.** The residual is a length, the point's offset from
    /// the carrier (linearized on a curved one), read against the band's
    /// own lengths, so no angular arm scales it. What could make a held
    /// point read past the zero band is the rounding of the point and of
    /// the residual, which grows with the coordinates; they are bounded
    /// by the session box (D4 ¶4, [`geom_core::tolerance::SESSION_BOX_EXTENT`],
    /// 1e3 m), where that rounding (about `1e3 · 2⁻⁵²` m, 2e-13) is below
    /// the zero band at every ratified ε. The probe that ruled this design
    /// read every constructed point `Zero`, at both ε and at `Interval`.
    ///
    /// # Panics
    ///
    /// In debug builds, where the premise does not hold: a construction
    /// that no longer puts its point on the carrier, a defect.
    #[track_caller]
    pub(crate) fn check(self, surface: &Surface<T>, band: Band) {
        if !cfg!(debug_assertions) {
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
        let holds = match self.premise {
            Premise::Constructed(_) | Premise::Decided(_) => matches!(read, Ok(Sign::Zero)),
            Premise::InBand => !matches!(read, Ok(Sign::Positive | Sign::Negative)),
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
