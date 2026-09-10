---
id: mc-lanes-draws-are-not-reproducible-from-outside-the-crate
kind: issue
title: the MC lane's draws are not reproducible from outside editor-core
status: open
opened: 2026-09-10
---


Found while building a demo cell whose subject is the MC lane's own
population: a density picture of the two-hole plate, every sample of
`monte_carlo`'s run drawn on one figure, so that a reader can SEE the
cloud the advisory mean summarizes. The picture is only worth drawing
if it is that run's samples and not merely samples from the same laws,
and a consumer outside `editor-core` cannot produce them.

Two halves, both real, and the second is the sharper one.

**1. The unit draw is private.** `mc::Rng::next_u64`
(`crates/editor-core/src/mc.rs:618`) is public and `mc::sample_stream`
(`:640`) hands out a sample's stream, but `Rng::unit`
(`:628`) — the `[0, 1)` value the lane actually feeds
`analysis::sample_offset` — is private. So a consumer holding the seed
and the index gets the raw `u64` sequence and then has to transcribe
`(x >> 11) as f64 * 2^-53` to reach the draw. This module's own header
argues against exactly that: `sample_stream` exists *because* "the two
transcriptions of `xorshift64*` in this tree cannot depend on each
other, so a test is what keeps them equal, and a test needs a door"
(`:633`). A third transcription in a consumer would have no such test.

**2. The lane advertises itself as ungated and its replay door is
gated.** `crates/pncad/src/analysis.rs:99` re-exports `monte_carlo`
UNCONDITIONALLY, with a comment that says why: it "shipped behind
`interval` in M10-6's first pass, which made the advisory lane
unreachable in a default build; R2's MINOR-9 caught it. A caller with
no certified scalar still gets the labeled estimate, which is the
whole point of an advisory lane."

But the lane places a sample through `ParamBox::from_axes` over
degenerate `BoxAxis::Varying { lo: offset, hi: offset }`
(`crates/editor-core/src/mc.rs:437`) — and BOTH `ParamBox` and
`BoxAxis` are re-exported only under `#[cfg(feature = "interval")]`
(`crates/pncad/src/analysis.rs:63-67`). So on a default build a
consumer can ask for the summary and cannot reach one sample's
document state: the lane hands out a mean over a population it will
not let anyone else construct a member of.

The mc module names that door as the only one there is — "a DEGENERATE
box is how a point sample reaches the evaluation service: `AxisScalar
for f64` admits an axis whose two ends are bit-equal and refuses every
other, so this is the one door a point-scalar replay over a parameter
value has, and the MC lane uses it rather than a second binding path"
(`:430`) — which is precisely why the gating matters. If it is the one
door, a consumer of an ungated lane needs it ungated.

**What a fix looks like.** Smallest: make `Rng::unit` public and
re-export `mc::{Rng, sample_stream}` from `pncad::analysis` beside
`monte_carlo`, and carry `ParamBox`/`BoxAxis` unconditionally the way
`ParamBoxError` already is (`crates/pncad/src/analysis.rs:49`, whose
argument — "a payload type reachable only under a feature is a refusal
a default-feature consumer can match and cannot name" — is the same
argument one level over). Better: one door, `mc::sample_draws(analyzed,
config, index) -> BTreeMap<ParamName, f64>`, so a consumer asks for
sample `i`'s parameter offsets and never touches the stream at all;
`monte_carlo` would call it too, which is what keeps it honest.

**The workaround the demo takes, stated rather than hidden.** The cell
places each sample by an ordinary `DocEdit::SetDocParamValue` of
`nominal + offset` rather than through a degenerate box, and then
MEASURES the agreement: it runs `monte_carlo` beside its own replay and
holds the web measure's mean, sigma, min and max to the report's. If
those agree the two paths coincide on this document and the cell says
so; if they do not, the difference is the finding and the cell says
that instead. What it cannot work around is half 1 — without a public
`unit()` the replay is a different population, so the picture would be
"samples from the same laws" rather than "this run's samples".


**Residue after `mc::sample_offsets` landed.** Two halves are left and
both are named rather than folded into a "closed":

* the GATING half above is untouched — `ParamBox`/`BoxAxis` are still
  `interval`-only in the façade while `monte_carlo` is not, so the
  lane's own replay path stays unreachable on a default build;
* `sample_offsets` is a Rust door and is **not bound in Python**, so a
  `pncad-py` consumer is still where every consumer was: able to ask
  for the summary and not for a member of the population. The demo
  that motivated the door is a Rust tour cell, so nothing forced the
  binding yet; the moment a Python row wants to draw or inspect one
  sample, it will. The binding census caught this on the first CI run
  and is where the debt now lives: `B-MC-DRAWS` in
  `crates/pncad-py/tests/test_binding_census.py`'s `FAMILIES`, with
  the charter saying what closing it delivers. That is the gate
  working — a curated façade name may be bound or dispositioned, and
  nothing else.
