//! **An offered tolerance, executed** (D4 ¶1 (i)): a refusal that ends
//! "if this size is intended, tighten the tolerance below v m" makes a
//! claim about a re-run, and this module makes the re-run.
//!
//! The tolerance is committed once per process (`CAD_TOLERANCE_EPS`), so
//! a re-run is a process: the host test re-executes its own binary at
//! one `#[ignore]`d child row per case ([`run`]), the child raises the
//! refusal at the tolerance its environment names and prints what came
//! back on one tagged line ([`report`]), and [`execute`] follows the
//! chain.
//!
//! An offer is a claim about **its own decision**: a smaller tolerance
//! decides it (D4 ¶1 (i), "a smaller ε decides it"). Re-running the same
//! raise at `0.9 ×` the value offered, the offer is **true** when it
//! passes there (T1), or when a *different* decision refuses there (T2),
//! and false when:
//!
//! - its own decision refuses again (F1);
//! - a defect or an invariant is reached, there or further along (F2);
//! - the operation is one no tolerance gets past (F3, a frontier): no
//!   tolerance down to [`FLOOR_EPS`] passes it.
//!
//! Every refusal met further along owes its own true story, and that is
//! its obligation, not the earlier offer's. [`execute`] records each one
//! ([`Later`]): which decision, and whether its own text is true where
//! the chain can judge it ([`Story`]). The host decides which later
//! stories it accepts, and names the row that owns any it does not.
//!
//! Each case starts at [`DESIGN_EPS`], the tolerance its geometry's
//! fixed margins were chosen for, whatever the running process's own
//! tolerance is.

// Panicking is this harness's failure mechanism, as it is every test's.
#![allow(clippy::panic, clippy::expect_used)]

/// The tolerance every case's first raise runs at: the geometry's fixed
/// margins are chosen against its band.
pub const DESIGN_EPS: f64 = 1e-9;

/// The fraction of the offered value a re-run takes: just below it.
pub const BELOW: f64 = 0.9;

/// The longest chain [`execute`] follows before calling it unbounded.
const LONGEST_CHAIN: usize = 12;

/// The smallest tolerance [`execute`] descends to before it calls an
/// operation one no tolerance gets past (F3): three decades below the
/// smallest tolerance CI runs the suite at.
pub const FLOOR_EPS: f64 = 1e-15;

/// The factor [`execute`] descends by past a refusal that offers no
/// value, looking for the tolerance that passes.
const DESCENT: f64 = 0.1;

/// The tag a child's report line starts with.
const TAG: &str = "OFFER-OUTCOME";

/// What one raise did at one tolerance.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Outcome {
    /// The raise passed.
    Pass,
    /// The raise refused.
    Refused {
        /// Which decision refused, as the host names decisions: two
        /// refusals of one decision carry one key.
        key: String,
        /// Whether the refusal is the kernel's own defect ending.
        defect: bool,
        /// The refusal as the user reads it.
        text: String,
    },
}

impl Outcome {
    /// The value this outcome offers to tighten below, where it offers
    /// one (a refusal, not a defect, naming a value).
    #[must_use]
    pub fn offer(&self) -> Option<f64> {
        match self {
            Self::Refused {
                defect: false,
                text,
                ..
            } => offered_below(text),
            Self::Pass | Self::Refused { .. } => None,
        }
    }
}

/// The value a rendered refusal offers to tighten the tolerance below,
/// read off the sentence the user reads.
#[must_use]
pub fn offered_below(text: &str) -> Option<f64> {
    let (_, tail) = text.split_once("tighten the tolerance below ")?;
    let value = tail.split_whitespace().next()?;
    value.parse::<f64>().ok().filter(|v| *v > 0.0)
}

/// The child's half: prints `outcome` for `case` on one tagged line.
pub fn report(case: &str, outcome: &Outcome) {
    match outcome {
        Outcome::Pass => println!("{TAG}\t{case}\tPASS"),
        Outcome::Refused { key, defect, text } => println!(
            "{TAG}\t{case}\tREFUSED\t{key}\t{}\t{}",
            u8::from(*defect),
            text.replace(['\n', '\t'], " ")
        ),
    }
}

/// Runs the child row `row` (its libtest path, module included) of the
/// running test binary in a process of its own at tolerance `eps`, and
/// reads back what its case did.
///
/// # Panics
///
/// If the child cannot run, fails, or prints no report: a broken
/// harness, never an outcome.
#[must_use]
pub fn run(row: &str, eps: f64) -> Outcome {
    let exe = std::env::current_exe().expect("a running test binary has a path");
    let out = std::process::Command::new(&exe)
        .args([
            row,
            "--exact",
            "--ignored",
            "--nocapture",
            "--test-threads=1",
        ])
        .env("CAD_TOLERANCE_EPS", format!("{eps:e}"))
        .env_remove("CAD_AMBIGUITY_K")
        .output()
        .unwrap_or_else(|e| panic!("{row} at {eps:e}: the child did not spawn: {e}"));
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        out.status.success(),
        "{row} at {eps:e}: the child failed:\n{stdout}\n{}",
        String::from_utf8_lossy(&out.stderr)
    );
    let line = stdout
        .lines()
        .find_map(|l| l.split_once(TAG).map(|(_, rest)| rest))
        .unwrap_or_else(|| {
            panic!("{row} at {eps:e}: no report (did the filter match?):\n{stdout}")
        });
    let fields: Vec<&str> = line.trim_start_matches('\t').split('\t').collect();
    match fields.as_slice() {
        [_, "PASS"] => Outcome::Pass,
        [_, "REFUSED", key, defect, text] => Outcome::Refused {
            key: (*key).to_owned(),
            defect: *defect == "1",
            text: (*text).to_owned(),
        },
        _ => panic!("{row} at {eps:e}: a malformed report: {line:?}"),
    }
}

/// One re-run of a chain: the tolerance it ran at and what came back.
#[derive(Clone, Debug)]
pub struct Link {
    /// The tolerance the raise ran at.
    pub eps: f64,
    /// What it did there.
    pub outcome: Outcome,
}

/// What a refusal met further along a chain tells, judged by its own
/// obligation where the chain can judge it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Story {
    /// It offers a value, and re-run just below it, its own decision no
    /// longer refuses.
    TrueOffer,
    /// It offers a value, and re-run just below it, its own decision
    /// refuses again: its offer is false (its F1).
    FalseOffer,
    /// It offers no value: a lever alone, a frontier, or a menu with no
    /// value in it. Its text is the host's to judge.
    NoValue,
}

/// A refusal met after the first re-run, and its story.
#[derive(Clone, Debug)]
pub struct Later {
    /// Which decision refused.
    pub key: String,
    /// What its own text tells, judged.
    pub story: Story,
    /// The refusal as the user reads it.
    pub text: String,
}

/// A true offer, executed.
#[derive(Clone, Debug)]
pub enum Verdict {
    /// The first re-run passes.
    T1,
    /// The first re-run refuses on a different decision; the chain
    /// passes further down, past the refusals `laters`, in the order met.
    T2 {
        /// The later refusals, one per decision.
        laters: Vec<Later>,
    },
}

/// An executed offer: the chain, and its verdict.
#[derive(Clone, Debug)]
pub struct Executed {
    /// Every raise, the first included.
    pub chain: Vec<Link>,
    /// What the first re-run showed of the offer.
    pub verdict: Verdict,
}

/// **Judges the refusals an executed offer met further along**
/// ([`Verdict::T2`]): each tells a true story (its valued offer true, or
/// no value and none of the unvalued `menu` in its text, a lever alone),
/// or is one whose story a row owns, named by its decision's key in
/// `owned`. Returns the decisions whose rows are logged, each with its
/// row, in the order met.
///
/// # Errors
///
/// A later refusal whose story is not true and that no row owns.
pub fn judge_laters(
    laters: &[Later],
    menu: &str,
    owned: &[(&'static str, &'static str)],
) -> Result<Vec<(String, &'static str)>, String> {
    let mut logged = Vec::new();
    for later in laters {
        let untrue = match later.story {
            Story::TrueOffer => false,
            Story::FalseOffer => true,
            Story::NoValue => later.text.contains(menu),
        };
        if !untrue {
            continue;
        }
        match owned.iter().find(|(key, _)| *key == later.key) {
            Some(&(_, row)) => logged.push((later.key.clone(), row)),
            None => {
                return Err(format!(
                    "{}: its own story is not true ({:?}) and no row owns it: {}",
                    later.key, later.story, later.text
                ));
            }
        }
    }
    Ok(logged)
}

/// **Executes the offer the case `row` makes at [`DESIGN_EPS`]**: its
/// first raise must refuse on `key` with a valued offer, and its re-run
/// at [`BELOW`] × that offer must pass (T1) or refuse on a different
/// decision (T2), past which the chain descends to a pass: at [`BELOW`]
/// × each later refusal's own offer, or by a decade past one that offers
/// none, never below [`FLOOR_EPS`]. `same` says which keys the host
/// counts as one decision (a gate and the reading it meters, say).
///
/// Returns the chain and its verdict, with each later refusal's story.
///
/// # Errors
///
/// A first raise that is not `key`'s valued refusal, or a chain that
/// meets F1, F2 or F3 (module docs) or runs past its bound.
pub fn execute(
    row: &str,
    key: &str,
    same: impl Fn(&str, &str) -> bool,
) -> Result<Executed, String> {
    let first = run(row, DESIGN_EPS);
    let mut chain = vec![Link {
        eps: DESIGN_EPS,
        outcome: first.clone(),
    }];
    let offer = match &first {
        Outcome::Refused { key: got, .. } if got == key => match first.offer() {
            Some(v) => v,
            None => return Err(format!("{row}: the first raise offers no value: {first:?}")),
        },
        _ => {
            return Err(format!(
                "{row}: the first raise is not {key}'s refusal: {first:?}"
            ));
        }
    };
    let eps = BELOW * offer;
    let mut current = run(row, eps);
    chain.push(Link {
        eps,
        outcome: current.clone(),
    });
    let tell = |why: &str, chain: &[Link]| format!("{row}: {why} (offer {offer:e}): {chain:#?}");
    match &current {
        Outcome::Pass => {
            return Ok(Executed {
                chain,
                verdict: Verdict::T1,
            });
        }
        Outcome::Refused { defect: true, .. } => {
            return Err(tell("F2, a defect at 0.9 × the offer", &chain));
        }
        Outcome::Refused { key: got, .. } if same(got, key) => {
            return Err(tell(
                "F1, its own decision refuses at 0.9 × the offer",
                &chain,
            ));
        }
        Outcome::Refused { .. } => {}
    }
    let mut laters: Vec<Later> = Vec::new();
    let mut at = eps;
    for _ in 0..LONGEST_CHAIN {
        let Outcome::Refused {
            key: got,
            defect,
            text,
        } = current.clone()
        else {
            return Ok(Executed {
                chain,
                verdict: Verdict::T2 { laters },
            });
        };
        if defect {
            return Err(tell("F2, a defect further along", &chain));
        }
        let offered = current.offer();
        let next = offered.map_or(DESCENT * at, |v| BELOW * v);
        if next < FLOOR_EPS {
            return Err(tell(
                &format!("F3, no tolerance down to {FLOOR_EPS:e} gets past {got}"),
                &chain,
            ));
        }
        let outcome = run(row, next);
        chain.push(Link {
            eps: next,
            outcome: outcome.clone(),
        });
        let story = match (offered, &outcome) {
            (None, _) => Story::NoValue,
            (Some(_), Outcome::Refused { key: again, .. }) if same(again, &got) => {
                Story::FalseOffer
            }
            (Some(_), _) => Story::TrueOffer,
        };
        match laters.iter_mut().find(|l| l.key == got) {
            Some(seen) if story == Story::FalseOffer => seen.story = story,
            Some(_) => {}
            None => laters.push(Later {
                key: got,
                story,
                text,
            }),
        }
        (current, at) = (outcome, next);
    }
    Err(tell(
        &format!("no pass within {LONGEST_CHAIN} re-runs"),
        &chain,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn an_offer_is_read_off_the_sentence() {
        let text = "x is undecided: margin 5e-9 lies inside the ambiguity band (1e-9, 1e-8). \
                    Recourse: move it, or, if this gap is intended, tighten the tolerance below \
                    5.5e-10 m";
        assert_eq!(offered_below(text), Some(5.5e-10));
        assert_eq!(offered_below("Recourse: move it"), None);
        assert_eq!(offered_below("… lower the tolerance"), None);
        let refused = |defect| Outcome::Refused {
            key: "k".into(),
            defect,
            text: text.into(),
        };
        assert_eq!(refused(false).offer(), Some(5.5e-10));
        assert_eq!(refused(true).offer(), None, "a defect offers nothing");
        assert_eq!(Outcome::Pass.offer(), None);
    }
}
