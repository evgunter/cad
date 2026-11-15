## For Ev

**Recommendation (likely).** Take the cross-check out of the decide path and make it an instrument a run asks for. A definite numeric sign returns at once and the tier is not consulted, which is what E12's numeric-first order and the code's own comment already say ("asking for [a form] is work whose answer is already known"). The half of the check that is free, and the only half that guards an answer the kernel actually returns, stays on in every run.

Terms. The *symbolic tier* (`Sym<T>`) carries an expression beside each number and can prove a margin identically zero by algebra (its *form*). *Numeric-first*: where the number already reads a definite sign, that sign is the answer. The *cross-check* asks the algebra anyway, after such an answer, whether it would have claimed zero. At `Sym<Interval>` (the *exact witness*: a certified bracket) a yes is a kernel bug and panics; at `Sym<f64>` (the *inexact witness*: one rounded number) a yes is counted as a *dispute*. A *registration* is a constructor's unchecked statement that two expressions are equal; a zero that needs one, under a definite sign, is counted as `registrations_contradicted`. The *receipt* is `SymCounts`, the per-run decision counts.

### The premise, corrected

- **The check protects no decision it runs on.** On a definite margin the numeric answer is returned whatever the algebra says. What the check does is test the algebra against the bracket on margins where the truth is known, to earn confidence for the margins where the algebra is believed. That is a differential test run inside the product on every decision; it is a test by function. (sure)
- **It is not the whole ladder.** It runs the first attempt only (plain form, rule form, top reduction; no retries, no registry). But a second walk the item does not name runs beside it: the registry check (`door_zero`) walks the registered form of every definite margin whenever the run holds a registration, in every profile. A fix confined to `contradicts` leaves that one. (sure on the code; its share of the measured time is unmeasured)
- **A repeated decision does not re-walk.** Forms are memoized per node per session, freezes included. The cost is first-time forms of margins nothing else asked about: on the slab, 95% of all rule-walk forms are the check's; over the chamber drive, 43 s of 162 s of walk time (`sym.rs`, "Cost"). (sure)
- **The check already leaks into the receipt, against ratified text.** `[failure]` row 5 says a `debug_assert`'s absence changes nothing shipped. This one writes `checks_expanded`/`checks_frozen` and puts its freezes into `frozen`, which the receipt defines as the nodes the run's *reasoning rested on*. The 96 frozen nodes per slab finish are all the check's, on decisions the tier never reasoned about. Turning release debug assertions off at publish (already on the "Before publishing" list) would change every receipt. (likely)
- **Provenance.** The assertion was added by an agent in a review-fix commit (`596e9eacd0`, 2026-09-04), in the same commit that introduced the shortcut it undoes. E12's sentence that the run "counts the contradiction" was written in an agent fix pass (`e80941f1bf`). Your D2 ruling (#2552) split *Disputed* from *Contradicted* at the registration door; I found nothing where you asked for a per-decision check. Release keeping its debug assertions is your directive and is untouched by this. (likely; searched `git log -S` on `sym.rs`, `ERROR-DESIGN.md`, `Cargo.toml`, and the fork log)
- **What it has caught.** I found two recorded firings, both at `Sym<f64>`, both false alarms (the theorem was right, the float was wrong); they are why the dispute column exists. I found no record of the exact-witness assertion catching an algebra defect. That is absence of a record, not proof it never did. (unsure)

### The recommended final state

1. **Decide path.** Definite sign: return it; no walk, no count. Indeterminate: the tier is asked, as today. (This is the state the code's comment describes and E12 ratifies.)
2. **The free half, always on.** Where the tier proves zero at an exact witness, the bracket must contain zero; if it does not, panic. One comparison, no walk, and it guards the answer being returned. Today nothing checks this (`an-in-band-enclosure-excluding-zero-is-stamped-proven-zero-unchecked`).
3. **The cross-check as an instrument**, installed by the run that wants it, like the tier's shape report and re-valuation instrument already are. It does what the check does now, all three events under one roof (algebra vs. exact bracket: panic; algebra vs. rounded number: dispute; registry vs. definite sign: contradicted), and because it is off the hot path it can afford the retry attempts too, which today are cross-checked in no profile (`a-retrys-zero-has-no-cross-check-against-the-numeric-channel`).
4. **The receipt.** `theorems_disputed`, `checks_expanded`, `checks_frozen` and `registrations_contradicted` leave `SymCounts` and become the instrument's own report, present only on a run that installed it. "Zero disputes" then cannot be read off a run that checked nothing: there is no number to read. `frozen` goes back to meaning what its doc says.
5. **Where it runs.** (a) Per PR: the rows that pin these columns today (the witness-kind rows, the far-placement table, the thread-count digests, the registrant pins) install it; they are its consumers. (b) Nightly: the `Sym<Interval>` document drives run with it on, through a dial on the drive, so the whole corpus is cross-checked daily. (c) A user's drive can turn the same dial on.

**What a run without it can no longer say:** "no proof of this run was contradicted by a definite sign." **What every run still says:** every definite answer is the numeric channel's and does not depend on the tier at all; every proven zero's bracket contains zero.

**What is traded:** a document outside the corpus that would have tripped the assertion in the field no longer does, unless its drive asks. The defect it would have flagged is an algebra bug on a margin whose answer was right anyway; the wrong answers such a bug produces are zeros on other margins, which the check never guarded.

**Worked example.** A ray against a cylinder wall reads `side_at_infinity_winding = [0.8, 1.3]`: positive, certified. Today the tier then expands that margin's expression through roots and turns, freezes ~45 nodes on coefficient size, concludes nothing, charges one `checks_frozen` and adds those nodes to `frozen`; 75 ms. Under the recommendation it returns `Positive`. Under the instrument it does today's work and reports it.

**Cost stated:** a definite decision costs the numeric channel's cost and nothing else. An indeterminate one costs what E12 already says.

**Ratified text that changes** (your sign-off): E12's sentence "where a registration WOULD have, the run counts the contradiction" becomes "a run under the cross-check counts it". `[simple.checks]` already points this way (three receipt columns and a walk path exist only so a check can be read) but does not settle the default; I would not add text there.

**Reversible:** yes; the dial's default is one line.

### The alternative that is defensible

**Keep it on every decision, bounded by a work allowance of its own.** The check stops after a fixed amount of algebra and records "abandoned". Pros: every run still tests the algebra a little; the pre-publish posture (every row-5 check live) is kept literally. Cons: a second budget, an abandon path and a fourth receipt column inside the tier, existing only for the check (`[simple.checks]`); what survives the allowance is the small forms, the ones least likely to be wrong; the receipt's `frozen` stays polluted; the registry walk needs the same treatment. I lean against (likely).

Rejected: sampling one decision in N (still unbounded per sampled decision; receipts get noisier); once per form (already true); deleting it with no instrument (loses the corpus-wide test for nothing).

**The one choice that is yours:** whether the drive's dial defaults off (my lean: the nightly carries the corpus) or on for certifying drives until publish, joining "turn release debug-assertions back off" on the pre-publish list. On keeps today's field coverage and today's cost in drives, and BOXFINISH's rows would then need the dial off in tests or the slow set.

## For the orchestrator

**Errors in the brief / item.**
- "Walks the tier's whole ladder": it is `Path::Contradiction`, first attempt only, no door, no retry. The title should say "the first attempt's walks".
- `door_zero` is the missed half: `door_form` (a third walk, own memo) on every definite margin when `rules.registered && rules.early && !registry.is_empty()`, unconditional in release, and it runs *before* `profile::set_origin(Origin::Assertion)`, so the cost profile charges its walks to the decision's origin. That is a small defect in the profile's attribution; file under SYM. Whether the registry was non-empty in M5's finishes is unmeasured; the four stack samples all sat in `contradicts`, which bounds nothing.
- "Whether a decision asked twice re-walks": no for the three walks (`memo.insert(id, f)` in `form_in`, frozen results included). The top rung's `algebra::reduce(plain, …)` in `rungs` is not memoized per root and re-runs per check of the same node; small.

**Not checked, assumed.**
- `SymbolicDials` (`editor-core/src/drive.rs`: `enabled`, the budget, the rules) is the drive's dial seat and fits; how a session below it carries the instrument (a field on the session, or a thread-local recorder like `revalue`'s) I left to the implementer.
- That the free half is sound under the decision read: a proven zero is zero where the build's decisions hold; at an exact witness those decisions held over the whole box, so the bracket of the margin over that box contains zero. Have the implementer state this at the assertion and run the corpus before trusting it; if a row trips, that is a finding, not a reason to drop it.
- That nothing shipped at `Sym<f64>` reads `theorems_disputed` outside tests: `drive.rs` prints it and `registrations_contradicted` only when non-zero; the other readers I found are test files (`sym11_*`, `m10_9_*`, the two thread-count digests, `revalue_corpus`). `tools/k-lint` reads none of the four.

**Pins that move under the recommendation** (final state, not cost): `frozen` on every pinned receipt whose freezes were the check's (M4's "96 per slab finish" returns to 0; the plate's 360 of 1,044 plain freezes go); the walk-ledger's `Early/Assertion` lines move under the instrument; `sym11_exact_channel_rows` and `m10_9_pins_interval` read the instrument's report instead of `SymCounts`.

**One level down, worth a measurement whatever Ev rules.** The same walks are what the *decision* path pays, with the retry ladder on top, the day one of these ray margins is indeterminate instead of definite (a refusing leaf). M5 shows the walks end in `Refusal::Ring` freezes after 75 to 150 ms. Unmeasured: whether that time is spent in products that are then refused (a pre-bound that refuses before multiplying would make the freeze cheap; `the-substituted-numerator-is-built-before-the-quotients-pre-bound-refuses-it` is the same shape) or in legitimately large forms below the frozen nodes. Capture forms per decision, largest form, and time before the first refusal on `side_at_infinity_winding`. This is SYM's, and it is not a substitute for the recommendation: on the slab, where nothing is expensive, the check is still 95% of the rule walk.

**Closes or re-states.** `an-in-band-enclosure-excluding-zero-…` (closed by item 2); `a-retrys-zero-has-no-cross-check-…` (answered by item 3); `far-placement-zero-dispute-cells-can-be-satisfied-by-freezes` (the coverage columns move to the instrument's report, the owed assertion is unchanged).

**Confidence I could not raise.** "What has the check ever caught" rests on grep over `work/*/log.md`, `work/sym`, `work/tier`, the dual-review log and the fork log; PR bodies and CI logs were not searched. If the other designer found a real catch at `Sym<Interval>`, weigh the default-on option higher.
