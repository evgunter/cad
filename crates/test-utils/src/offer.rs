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
//! An offer is **true** when the same raise, re-run at `0.9 ×` the value
//! offered, either passes (T1), or refuses on a *different* decision
//! whose own offer is true in the same sense, so that the chain ends in
//! a pass (T2). It is **false** when the re-run refuses on the same
//! decision again (F1), on a defect (F2), or on anything that offers no
//! tolerance at all (F3): a frontier, a lever-alone decision, a refused
//! declaration. Each case starts at [`DESIGN_EPS`], the tolerance its
//! geometry's fixed margins were chosen for, whatever the running
//! process's own tolerance is.

// Panicking is this harness's failure mechanism, as it is every test's.
#![allow(clippy::panic, clippy::expect_used)]

/// The tolerance every case's first raise runs at: the geometry's fixed
/// margins are chosen against its band.
pub const DESIGN_EPS: f64 = 1e-9;

/// The fraction of the offered value a re-run takes: just below it.
pub const BELOW: f64 = 0.9;

/// The longest chain [`execute`] follows before calling it unbounded.
const LONGEST_CHAIN: usize = 6;

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

/// **Executes the offer the case `row` makes at [`DESIGN_EPS`]**: its
/// first raise must refuse on `key` with a valued offer, and every
/// re-run at [`BELOW`] × the last offer must pass or refuse on a new
/// decision that offers a value in turn, until one passes. `same` says
/// which keys the host counts as one decision (a gate and the reading
/// it meters, say).
///
/// Returns the chain, first raise included, or the sentence saying why
/// the offer is false.
///
/// # Errors
///
/// A first raise that is not `key`'s valued refusal, or a chain that
/// meets F1, F2 or F3 (module docs) or runs past its bound.
pub fn execute(
    row: &str,
    key: &str,
    same: impl Fn(&str, &str) -> bool,
) -> Result<Vec<Link>, String> {
    let first = run(row, DESIGN_EPS);
    let mut chain = vec![Link {
        eps: DESIGN_EPS,
        outcome: first.clone(),
    }];
    let (mut last_key, mut offer) = match &first {
        Outcome::Refused { key: got, .. } if got == key => match first.offer() {
            Some(v) => (got.clone(), v),
            None => return Err(format!("{row}: the first raise offers no value: {first:?}")),
        },
        _ => {
            return Err(format!(
                "{row}: the first raise is not {key}'s refusal: {first:?}"
            ));
        }
    };
    for _ in 0..LONGEST_CHAIN {
        let eps = BELOW * offer;
        let outcome = run(row, eps);
        chain.push(Link {
            eps,
            outcome: outcome.clone(),
        });
        let tell = |why: &str| format!("{row}: {why} at {eps:e} (0.9 × {offer:e}): {chain:#?}");
        match &outcome {
            Outcome::Pass => return Ok(chain),
            Outcome::Refused { defect: true, .. } => return Err(tell("F2, a defect")),
            Outcome::Refused { key: got, .. } if same(got, &last_key) => {
                return Err(tell("F1, the same decision refuses again"));
            }
            Outcome::Refused { key: got, .. } => match outcome.offer() {
                Some(v) => (last_key, offer) = (got.clone(), v),
                None => return Err(tell("F3, a refusal no tolerance passes")),
            },
        }
    }
    Err(format!(
        "{row}: no pass within {LONGEST_CHAIN} re-runs: {chain:#?}"
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
