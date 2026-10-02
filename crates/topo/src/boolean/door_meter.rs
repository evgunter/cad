//! **The door's tier-3 meter** (feature `door-tier3-meter`): what tier 3
//! would cost, and what it would refuse, if `ops::gate` ran it.
//!
//! `ops::gate` runs tiers 1 and 2 on every finished result. With this
//! feature on, the door also runs the scalar's at-rest gate
//! ([`crate::AtRestPolicy::gate_at_rest`]) on the result and on both
//! operands after the volume backstop, and appends one tab-separated
//! line per result to `door-tier3-meter.tsv` in the system temporary
//! directory:
//!
//! `test op scalar faces op_µs tier3_µs backstop_µs backstop_ok
//! tier3_verdict a_tier3_ok b_tier3_ok`
//!
//! It changes no result: the verdicts are recorded and the door goes on
//! exactly as it would without them. `scripts/door-tier3-meter.py` runs
//! the suites with it on and prints the table
//! `work/reach/boolean-door-tier-3-waits-on-the-description-gap.md`
//! reports.

use std::io::Write as _;
use std::time::{Duration, Instant};

use crate::body::Body;
use crate::props::AtRestPolicy;

use super::BooleanOp;

/// One result's timings, taken as the door runs.
pub(super) struct Meter {
    op: Duration,
    backstop_start: Instant,
}

impl Meter {
    /// The op's time so far (from `start`), and the backstop's start.
    pub(super) fn start(start: Instant) -> Self {
        Self {
            op: start.elapsed(),
            backstop_start: Instant::now(),
        }
    }

    /// Runs tier 3 on the result and both operands and appends the
    /// line. A file that will not open drops the line: the meter never
    /// changes what the door does.
    pub(super) fn record<T: AtRestPolicy>(
        self,
        op: BooleanOp,
        a: &Body<T>,
        b: &Body<T>,
        result: &Body<T>,
        backstop_ok: bool,
        tol: geom_core::Tol,
    ) {
        let backstop = self.backstop_start.elapsed();
        let start = Instant::now();
        let tier3 = T::gate_at_rest(result, tol);
        let tier3_time = start.elapsed();
        let verdict = match &tier3 {
            Ok(_) => "ok".to_string(),
            Err(errors) => format!(
                "{:?}",
                errors
                    .iter()
                    .map(|e| format!("{e:?}").chars().take(160).collect::<String>())
                    .collect::<Vec<_>>()
            ),
        };
        let line = format!(
            "{}\t{op:?}\t{}\t{}\t{}\t{}\t{}\t{backstop_ok}\t{verdict}\t{}\t{}\n",
            std::thread::current().name().unwrap_or("?"),
            T::NAME,
            result.faces().count(),
            self.op.as_micros(),
            tier3_time.as_micros(),
            backstop.as_micros(),
            T::gate_at_rest(a, tol).is_ok(),
            T::gate_at_rest(b, tol).is_ok(),
        );
        let path = std::env::temp_dir().join("door-tier3-meter.tsv");
        if let Ok(mut file) = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)
        {
            let _ = file.write_all(line.as_bytes());
        }
    }
}
