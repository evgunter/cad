//! **The identity ladder**: a question answered by several banded data
//! read in order, each required Zero for the identity to hold — a chart
//! that is the torus an image names, a circle that is a torus's
//! Villarceau circle.
//!
//! The rule, stated once: **any definite datum answers; an in-band one
//! is kept, and refuses only where no later datum decides.** A ladder
//! that returned on its first in-band datum would escalate where a
//! datum it had not yet read already answers, and the refusal would
//! name a tolerance that cannot change the answer.
//!
//! [`read_identity`] is that reading for any per-datum reader, and owns
//! the recording: a kept datum's run is held, and spliced as superseded
//! ([`geom_core::k_stats::splice_superseded`]) when a later datum
//! answers, so its escalation does not read as one the answer rests on.
//! Its readers are the callers of [`read_identity`].

use geom_core::k_stats::{Detached, detached, splice, splice_superseded};
use geom_core::predicate::{Decided, Indeterminate};
use geom_core::{Margin, MarginDiag, Sign};

/// What an identity ladder's data answer.
#[derive(Clone, Debug, PartialEq)]
pub enum IdentityLadder<E> {
    /// A datum decided definitely: the identity does not hold. `at` is
    /// its index in the data, `sign` its sign.
    Apart {
        /// The answering datum's index.
        at: usize,
        /// Its sign (never [`Sign::Zero`]).
        sign: Sign,
    },
    /// Every datum decided Zero: the identity holds, with the margins
    /// the band decided joined (`None` for no data).
    Zero(Option<MarginDiag>),
    /// No datum decided definitely and at least one decided nothing:
    /// the readings that did not decide, in data order.
    Undecided(Kept<E>),
}

/// The readings an [`IdentityLadder::Undecided`] kept, in data order:
/// never empty.
#[derive(Clone, Debug, PartialEq)]
pub struct Kept<E> {
    /// The first reading kept.
    pub first: E,
    /// The ones kept after it.
    pub rest: Vec<E>,
}

impl<E> Kept<E> {
    /// How many readings were kept.
    #[must_use]
    pub fn len(&self) -> usize {
        1 + self.rest.len()
    }

    /// Never: a ladder that kept nothing is not undecided.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        false
    }

    /// The kept readings, in data order.
    #[must_use]
    pub fn into_vec(self) -> Vec<E> {
        let mut all = vec![self.first];
        all.extend(self.rest);
        all
    }
}

impl Kept<Indeterminate> {
    /// The one cause a ladder of [`geom_core::k_stats::decide_reported`]
    /// readings refuses on: the first unreadable (poisoned) margin, a
    /// limit no tolerance moves, ahead of the first in-band one.
    #[must_use]
    pub fn refusing_cause(self) -> Indeterminate {
        let first = self.first;
        core::iter::once(first)
            .chain(self.rest)
            .find(|cause| cause.margin.is_invalid())
            .unwrap_or(first)
    }
}

/// Reads `data` in order through `read` under the identity ladder's
/// rule (module docs), recording as a serial read of the same data
/// would, in data order.
///
/// Each kept reading's run is held until the ladder's outcome is known:
/// spliced whole where no later datum decides (the escalation is what
/// the outcome rests on), spliced superseded where one does.
pub fn read_identity<T, E>(
    data: impl IntoIterator<Item = (&'static str, Margin<T>)>,
    mut read: impl FnMut(&'static str, Margin<T>) -> Result<Decided, E>,
) -> IdentityLadder<E> {
    let mut joined: Option<MarginDiag> = None;
    let mut kept: Vec<E> = Vec::new();
    // The runs read since the first kept one, in data order, each with
    // whether it was kept: held so the order survives the splice.
    let mut held: Vec<(Detached, bool)> = Vec::new();
    for (at, (name, margin)) in data.into_iter().enumerate() {
        let (got, log) = detached(|| read(name, margin));
        match got {
            Ok(Decided {
                sign: sign @ (Sign::Positive | Sign::Negative),
                ..
            }) => {
                for (run, was_kept) in held {
                    if was_kept {
                        splice_superseded(run);
                    } else {
                        splice(run);
                    }
                }
                splice(log);
                return IdentityLadder::Apart { at, sign };
            }
            Ok(Decided {
                sign: Sign::Zero,
                margin,
            }) => {
                joined = Some(joined.map_or(margin, |first| first.jointly(margin)));
                if held.is_empty() {
                    splice(log);
                } else {
                    held.push((log, false));
                }
            }
            Err(cause) => {
                kept.push(cause);
                held.push((log, true));
            }
        }
    }
    held.into_iter().for_each(|(run, _)| splice(run));
    let mut kept = kept.into_iter();
    match kept.next() {
        None => IdentityLadder::Zero(joined),
        Some(first) => IdentityLadder::Undecided(Kept {
            first,
            rest: kept.collect(),
        }),
    }
}

#[cfg(test)]
#[allow(clippy::expect_used, clippy::panic)]
mod tests {
    use geom_core::k_stats::{Bracket, decide_reported};
    use geom_core::predicate::Band;
    use geom_core::{Margin, Sign};

    use super::{IdentityLadder, read_identity};

    fn band() -> Band {
        Band::new(1e-9, 1e-8).expect("a valid band")
    }

    /// The ladder read through the kernel's funnel, under a bracket:
    /// its outcome, and the names its run recorded as verdicts and as
    /// escalations.
    fn read(
        data: &[(&'static str, f64)],
    ) -> (
        IdentityLadder<geom_core::predicate::Indeterminate>,
        Vec<&'static str>,
        Vec<&'static str>,
    ) {
        let bracket = Bracket::open();
        let got = read_identity(data.iter().map(|&(n, m)| (n, Margin::of(m))), |n, m| {
            decide_reported(n, m, band())
        });
        let recorded = bracket.finish();
        (
            got,
            recorded.verdicts.iter().map(|v| v.predicate).collect(),
            recorded
                .escalations
                .iter()
                .map(geom_core::k_stats::Escalation::predicate)
                .collect(),
        )
    }

    /// **A later definite datum answers past an earlier in-band one**,
    /// and the kept reading's escalation leaves the log, since the
    /// answer does not rest on it: the masking the pcurve ladders had
    /// when they returned on their first in-band gate.
    #[test]
    fn a_later_definite_datum_answers_past_an_in_band_one() {
        let (got, verdicts, escalations) = read(&[("a", 0.0), ("b", 5e-9), ("c", 0.0), ("d", 1.0)]);
        assert_eq!(
            got,
            IdentityLadder::Apart {
                at: 3,
                sign: Sign::Positive
            }
        );
        assert_eq!(
            verdicts,
            ["a", "c", "d"],
            "every decided reading, in data order"
        );
        assert!(
            escalations.is_empty(),
            "the superseded escalation: {escalations:?}"
        );
    }

    /// **An in-band datum no later datum decides refuses on it**, its
    /// escalation recorded: no definite datum answers, so the in-band
    /// one is what the outcome rests on.
    #[test]
    fn an_in_band_datum_refuses_where_no_later_one_decides() {
        let (got, verdicts, escalations) = read(&[("a", 0.0), ("b", 5e-9), ("c", 0.0)]);
        let IdentityLadder::Undecided(kept) = got else {
            panic!("undecided: {got:?}")
        };
        assert_eq!(kept.len(), 1);
        assert_eq!(kept.refusing_cause().predicate, Some("b"));
        assert_eq!(verdicts, ["a", "c"]);
        assert_eq!(escalations, ["b"]);
    }

    /// **The first definite datum answers, and nothing after it is
    /// read**; all Zero is the identity, its margins joined.
    #[test]
    fn the_first_definite_datum_answers_and_all_zero_holds() {
        let (got, verdicts, _) = read(&[("a", -1.0), ("b", 5e-9), ("c", 1.0)]);
        assert_eq!(
            got,
            IdentityLadder::Apart {
                at: 0,
                sign: Sign::Negative
            }
        );
        assert_eq!(verdicts, ["a"], "nothing past the answer is read");
        let (got, _, _) = read(&[("a", 0.0), ("b", 5e-10)]);
        assert!(matches!(got, IdentityLadder::Zero(Some(_))), "{got:?}");
        let (got, _, _) = read(&[]);
        assert_eq!(got, IdentityLadder::Zero(None));
    }

    /// **An unreadable datum is the refusal's cause ahead of an in-band
    /// one**, whichever was read first: a poisoned margin is a limit no
    /// tolerance moves.
    #[test]
    fn an_unreadable_datum_refuses_ahead_of_an_in_band_one() {
        let (got, _, _) = read(&[("a", 5e-9), ("b", f64::NAN), ("c", 0.0)]);
        let IdentityLadder::Undecided(kept) = got else {
            panic!("undecided: {got:?}")
        };
        assert_eq!(kept.len(), 2);
        assert_eq!(kept.refusing_cause().predicate, Some("b"));
    }
}
