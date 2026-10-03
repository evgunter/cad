//! **The boolean door's meter** (feature `door-tier3-meter`): what one
//! call through [`super::boolean_op_with`] costs, and what the at-rest
//! gate over its result costs and says.
//!
//! One tab-separated line per public call, appended to the file the
//! build names in `CAD_DOOR_TIER3_METER_OUT`. The path is fixed when the
//! crate is compiled (`option_env!`), so the kernel reads no environment
//! at run time and the caller — `scripts/door-tier3-meter.py`, which
//! names a fresh file per run and refuses an empty table — chooses the
//! sink. A build that names none (an `--all-features` build, say) leaves
//! the meter inert: it times nothing and writes nothing.
//!
//! `test op scalar faces door_µs gate_µs outcome gate_verdict`
//!
//! `door_µs` is the whole call, the gate included; `gate_µs` is the
//! result's tier-3′ gate (tier 3, then the census over the result's own
//! contact records), zero where the call built no body. `outcome` is
//! `empty`, `body`, or the refusal's variant name; `gate_verdict` is
//! `ok`, `-` where no gate ran, or the gate's findings.
//!
//! It changes no result. A line it cannot write panics, so the table
//! never undercounts in silence.

use std::io::Write as _;
use std::time::Duration;

use super::{BooleanError, BooleanOp, BooleanResult};
use crate::props::AtRestPolicy;

/// The gate's cost and findings over one result, as the door ran it.
pub(super) struct GateReading {
    /// Time in the gate.
    pub(super) time: Duration,
    /// `ok`, or the findings, each its `Debug` cut to 200 characters.
    pub(super) verdict: String,
}

impl GateReading {
    /// The reading of one gate run.
    pub(super) fn of<E: core::fmt::Debug>(time: Duration, verdict: Result<(), &[E]>) -> Self {
        let verdict = match verdict {
            Ok(()) => "ok".to_string(),
            Err(errors) => errors
                .iter()
                .map(|e| format!("{e:?}").chars().take(200).collect::<String>())
                .collect::<Vec<_>>()
                .join(" ; "),
        };
        Self { time, verdict }
    }
}

/// Appends one call's line.
///
/// # Panics
///
/// When the sink will not open or take the line: an instrument that
/// drops a record reports a table it did not measure.
#[allow(clippy::panic)]
pub(super) fn record<T: AtRestPolicy>(
    op: BooleanOp,
    result: &Result<BooleanResult<T>, BooleanError>,
    door: Duration,
    gate: Option<&GateReading>,
) {
    let Some(path) = option_env!("CAD_DOOR_TIER3_METER_OUT") else {
        return;
    };
    let (faces, outcome) = match result {
        Ok(BooleanResult::Empty) => (0, "empty".to_string()),
        Ok(BooleanResult::Body(r)) => (r.body.faces().count(), "body".to_string()),
        Err(e) => (
            0,
            format!("{e:?}")
                .split(|c: char| !c.is_alphanumeric())
                .next()
                .unwrap_or("?")
                .to_string(),
        ),
    };
    let (gate_us, verdict) = gate.map_or((0, "-"), |g| (g.time.as_micros(), g.verdict.as_str()));
    let line = format!(
        "{}\t{op:?}\t{}\t{faces}\t{}\t{gate_us}\t{outcome}\t{verdict}\n",
        std::thread::current().name().unwrap_or("?"),
        T::NAME,
        door.as_micros(),
    );
    let written = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .and_then(|mut file| file.write_all(line.as_bytes()));
    if let Err(e) = written {
        panic!("door-tier3-meter: the sink {path} refused a line: {e}");
    }
}
