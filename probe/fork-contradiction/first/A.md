## For Ev

**Recommendation (likely).** Move the check onto the answer the kernel actually ships, and take it off the answers it doesn't use.
1. **Where the symbolic tier answers `Zero`** (every rung, every retry attempt, the registry door), check that answer against the exact witness: at `Sym<Interval>` the certified enclosure must contain 0, or a channel is unsound. It's one comparison. It runs as a plain `assert!` in every profile, because if it fails a wrong verdict ships (D9: "a load-bearing check panics in release").
2. **Where the numeric channel reads a definite sign**, the tier is not asked anything. That sign is the answer, and the decision costs what the numeric channel costs, plus one witness-kind comparison and a deferral lookup. Both of today's after-the-fact walks go: `contradicts` (the theorem check) and `door_zero` (the registry check).
3. **The old cross-check becomes a test instrument.** It asked: does the tier's algebra prove zero a margin that the exact channel proved non-zero? It moves behind a test-only cargo feature (e.g. `sym-check-testing`, after the existing `sym-revalue-testing`). A nightly lane runs it over the corpus with the whole ladder, retries included, and asserts it finds nothing. Its counts are in that instrument's report, not in the kernel's receipt.
4. **`SymCounts` loses four columns:** `theorems_disputed`, `checks_expanded`, `checks_frozen` and `registrations_contradicted`. Their readers go or move to the instrument's rows.

Terms. *Numeric channel*: the lane value carried beside the expression (an `Interval` in the driver's replay, an `f64` in unit tests). *Exact witness*: a value channel whose definite sign is a proof, which means `Interval`. *Inexact*: one rounded number (`f64`, `Probe`). *Walk / ladder*: building the margin's normal form, rung by rung, to see whether it is zero. *Receipt*: `SymCounts`, a session's tally of its decisions.

**Premise check: the check is on the wrong population (sure).** Today it runs only where the numeric channel was definite, which is exactly where the tier's answer is thrown away. Where the tier's answer is used (the margin's numeric reading is indeterminate and the tier returns `Zero`), nothing cross-checks it:
- a retry attempt's zero is never checked (open item `a-retrys-zero-has-no-cross-check-against-the-numeric-channel`);
- an exact in-band enclosure that excludes 0 can be stamped proven zero unchecked (open item `an-in-band-enclosure-excluding-zero-is-stamped-proven-zero-unchecked`).

So the check is a property test of the algebra. It samples a population where the true answer is known and nothing depends on the result, and it is wired into every decision. That is why it costs so much. It is not the BOXFINISH curved rays alone. The tier's own `# Cost` notes in `sym.rs` measure it at 95% of the early walk's forms on the M10-3 slab, and on the chamber drive at 3.18 M of 3.35 M early forms and 43 s of 162 s in the walks. BOXFINISH's eight decisions are just where it became visible.

**Why not the build profile (sure).** Turning release debug-assertions off doesn't help. The 1.0 s rows run in the test profile, where they stay on. It would also leave a receipt column that means different things per build: at `Interval`, the check columns read 0 when assertions are off.

**What a run can no longer claim.** It can't claim that "the tier's algebra was cross-checked against this document's definite margins." It can claim something stronger about its own output: every symbolic `Zero` it returned agrees with its certified enclosure, now including retries, the door and in-band readings.

One defect class is caught less often: a false algebraic zero on a margin whose enclosure straddles 0. No numeric check in any run can see that case. The old check caught the same *bug* only if it also showed up on a definite margin of the same run. The nightly instrument catches it across the corpus. **Confidence: likely.** The suite and history show no exact-witness firing of the assertion. Every recorded firing was the inexact arm's false positive (`sym-f64-far-placement-trips-the-theorem-vs-numeric-assert`, which became `theorems_disputed`). I could not run a count.

**Each answer as a final state.**
- **A (recommended).** As above. Pros:
  - a definite decision costs a bounded, stated amount;
  - every shipped zero is cross-checked;
  - no receipt column changes meaning by build;
  - two open soundness items close;
  - [simple.checks] holds: no receipt column exists only for a check;
  - the inexact arm's "disputes" disappear. They were always an artefact of asking a point reading to refute a theorem.
  
  Con: the algebra's soundness on definite margins becomes a nightly claim, not a per-run one. Reversible: the instrument is the same predicate behind a feature.
- **B. Keep the per-decision check, with its own budget** (plain rung only, or a term cap). This keeps the per-run claim, but the cost is still per decision and unmeasured on curved margins (plain-form sizes per decision were never captured). It keeps the wrong population, and the check columns remain. I'd reject it.
- **C. Sample one definite margin in N, by node id.** This is the retry item's option 1. Each sampled walk still costs 75–150 ms on a curved ray, and the receipt would report a sampling rate as coverage. I'd reject it.

**The registry door (likely, less sure than the rest).** `door_zero` asks, on a definite sign, whether the registry would have folded this margin. That catches a registrant's lie where nothing used it. Under A, a door-rung `Zero` that actually decides is checked against the enclosure, and the definite-margin version moves to the instrument. The alternative is to keep `registrations_contradicted` per run as a cheaper door-only walk. I lean to moving it: no measurement separates its cost from `contradicts`, since both walk the same memoized ladder.

**Ratified text.**
- `[simple.checks]` already decides this and needs no change.
- E12's registry bullet in `ERROR-DESIGN.md` ("where a registration WOULD have, the run counts the contradiction (`SymCounts::registrations_contradicted`) instead of folding it away") changes to say a door zero is checked against the exact enclosure where it decides. Provenance: written by the M10-9 fix pass (e80941f1bf, 2026-09-06), agent text under E12's "lgtm" on #1712, not your words.
- D9 row 5 needs no change, but the new assert is row 5's "load-bearing check" case.

**Does this close BOXFINISH's cost row?**
- The cylinder-walled body: likely. Its eight slow decisions were numeric 24/24 and 12/12, so they walk nothing.
- The revolved circle (2.76 s): unsure. `curve_pair_span_end_chord` (559 ms) isn't listed as always-numeric. The decision path could still be paying there, which would be the tier's decision-path cost, a separate question.

## For the orchestrator

- **Unverified, worth one look before the unit:**
  - Whether the check's plain-walk freezes feed `SymCounts::frozen`. `form_in` increments it on every non-early freeze, whatever the origin. If they do, today's `frozen` and the leaf NEED include freezes made by a check whose answer is discarded, and they change with debug-assertions. A removes that. M4's "frozen = 192 as built" on the derived-frame row may be partly the check's.
  - How the probe's `Origin::Assertion` split charges `door_zero` vs `contradicts`. Whichever runs first pays the shared memoized walk; the cylinder body likely has registrations, so `door_zero` runs.
- **Readers to re-point or delete under A:**
  - `editor-core/src/drive.rs` (report lines for `registrations_contradicted`, `theorems_disputed`);
  - `sweep/tests/mass_props_are_thread_count_invariant.rs` (the digest's disp=0 coverage guard on `checks_expanded`);
  - `continuation_is_thread_count_invariant.rs`;
  - `editor-core/tests/m10_9_pins_interval.rs` (`theorems_disputed == 0`), `sym11_exact_channel_rows.rs`, `revalue_corpus.rs`;
  - `geom-core/tests/sym11_witness_kind{,_interval}_rows.rs`, `sweep/tests/sym11_far_placement_rows.rs`;
  - `real.rs` doc refs; `sym.rs` `# Cost` and the `SymCounts` docs.
  
  The far-placement and witness-kind rows measured the check's own false positives. Most become instrument rows or go.
- **Items A closes or supersedes:**
  - `work/sym/a-retrys-zero-has-no-cross-check-against-the-numeric-channel`, `an-in-band-enclosure-excluding-zero-is-stamped-proven-zero-unchecked`, `a-dispute-names-no-predicate-on-the-receipt`, `far-placement-zero-dispute-cells-can-be-satisfied-by-freezes`;
  - `work/nurbs/disp-zero-can-be-satisfied-by-freezes`, `the-thread-count-digest-cannot-see-dispute-coverage`.
  
  `work/tier/symbolic-tier-costs-95-percent-of-the-m10-3-drive` should be re-measured after A: per `sym.rs` `# Cost`, much of that 95% is this check.
- **The pin to flip:** `predicate::tests::a_proven_zero_is_offered_no_tolerance` uses an unsound fixture (an excluding enclosure stamped proven). Under A it must become a straddling enclosure.
- **The new assert:** at `Witness::Inexact`, nothing is checked on the decision path, because a point reading in band can't refute a zero. `note_witness` already fixes one kind per session.
- **Assumed, not checked:** that `enclosure_probe()` or an equivalent gives the decision path the exact enclosure at `Sym<Interval>` cheaply. I didn't fetch `boxfinish/probe-switch-off` or read `probe/M4-REPORT.md` itself; I relied on the item's M4/M5 sections.
- **Off-question:** none filed. The brief was sufficient.
