//! **The identity walk**: whether two descriptions are one, read off a
//! list of data that are each zero exactly where the two agree
//! ([`identity_walk`]). The Boolean's carrier ladder and a torus pair's
//! coincidence ([`crate::tori_coincide`]) read their data through it.

use geom_core::k_stats::{decide_reported, detached, splice, splice_superseded};
use geom_core::{Band, Decide, Decided, Indeterminate, Margin, MarginDiag, Sign};

/// What a list of identity data reads ([`identity_walk`]).
#[derive(Clone, Copy, Debug)]
pub enum IdentityWalk {
    /// Some datum decided past the band: two things.
    Distinct,
    /// Every datum decided `Zero`: one thing, the data's decided margins
    /// taken jointly (`None` for no data).
    One(Option<MarginDiag>),
    /// No datum decided past the band and one did not decide: the first
    /// that cannot be read (its margin invalid), else the first in band.
    Undecided(Indeterminate),
}

/// **The identity rule**: two descriptions are one only where every
/// datum decides `Zero`, so a datum decided past the band answers
/// [`IdentityWalk::Distinct`] whatever another read before it, and the
/// readings it supersedes keep their verdicts and drop their
/// escalations. A datum in band or unreadable is kept, not returned, and
/// refuses only where no later datum decides the two apart; an
/// unreadable one is ranked first, being a limit no tolerance moves.
/// A datum that is not finite reads no sign: the funnel reads its
/// residual `x − x`, which is poison.
pub fn identity_walk<T: Decide>(
    data: impl IntoIterator<Item = (&'static str, Margin<T>)>,
    band: Band,
) -> IdentityWalk {
    let mut joint: Option<MarginDiag> = None;
    let (mut blocked, mut in_band) = (None, None);
    let mut kept_logs = Vec::new();
    for (name, margin) in data {
        let read_on = if geom_core::is_finite_length(margin.value()) {
            margin
        } else {
            Margin::of(margin.value() - margin.value())
        };
        let (read, log) = detached(|| decide_reported(name, read_on, band));
        match read {
            Ok(Decided {
                sign: Sign::Positive | Sign::Negative,
                ..
            }) => {
                kept_logs.into_iter().for_each(splice_superseded);
                splice(log);
                return IdentityWalk::Distinct;
            }
            Ok(Decided {
                sign: Sign::Zero,
                margin,
            }) => {
                splice(log);
                joint = Some(joint.map_or(margin, |first| first.jointly(margin)));
            }
            Err(diag) => {
                kept_logs.push(log);
                if diag.margin.is_invalid() {
                    blocked.get_or_insert(diag);
                } else {
                    in_band.get_or_insert(diag);
                }
            }
        }
    }
    kept_logs.into_iter().for_each(splice);
    match blocked.or(in_band) {
        Some(diag) => IdentityWalk::Undecided(diag),
        None => IdentityWalk::One(joint),
    }
}
