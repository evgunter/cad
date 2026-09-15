# S-TINT log

Newest entries at the bottom; the tail is the program's live status.
Plan: `work/tint/plan.md`.

## Opened (2026-09-11)

Opened on Ev's direction (in-chat, 2026-09-11) out of S-TCOST's re-sort
of its own board. The sort's question was which of that program's rows
survive the repository going public on 2026-09-03, and the answer split
the board three ways: rows whose subject is LATENCY (which survive), rows
that were buying BILLED MINUTES (which do not), and a third group that
was never about either. This program is that third group, given a
charter of its own rather than left on a cost program's slate.

**Thirty rows moved by `git mv`**, ids unchanged, each carrying a
`## Moved to S-TINT (2026-09-11)` record. The list was derived from the
tree, not from a document: the 42 live items in `work/tcost/` at
`origin/main` `f55ee213`, less the twelve that are cost rows or gate
mechanism. Those twelve stay, and are named here so the split is
readable from one side:

| stays with S-TCOST | why |
|---|---|
| `rust-cache-never-restores-across-branches` | a cold ~300-unit compile on the run's longest job — the pole itself |
| `nextest-shard-count-needs-remeasure` | re-opened 2026-09-11; the N=2 verdict was per-job billed-minute rounding |
| `tcost-area-pad-lever`, `offset-composite-lazy-sign-gate` | kernel constant factors the shipped program pays too |
| `edge-nurbs-computes-the-chart-image-and-discards-it` | a compute-and-discard candidate, unmeasured |
| `nightly-demotions-c1-c3-were-bought-with-billed-minutes` | the three landed demotions to re-cost |
| `gated-marker-omits-sibling-helper-imports`, `gated-marker-path-mount` | defects IN the per-file gate, which is S-TCOST's mechanism |
| `proptest-modules-in-src-ungated`, `r1-probe-seeds-are-not-on-the-fuzz-dial` | the fuzz-gating policy question, one ruling |
| `consider-proptest-for-randomized-sweeps` | parked on S-TCOST's own harness |
| `ci-filter-cites-a-path-the-ledger-recipe-cannot-open` | `scripts/ci-filter.py` is S-TCOST's file |

**The two territories overlap and that is deliberate.** Both programs
claim `crates/*/tests/*` and `crates/test-utils/*`; the fence is the
question, not the path, and `work.py territory` warns rather than blocks.
The rule is written into the plan's §*The fence with S-TCOST* so no lane
has to guess: a row justified by a second goes back to S-TCOST, a row
justified by a claim that cannot fail stays here, and the gate mechanism,
the fuzz-gating policy and everything under `scripts/` are S-TCOST's
whatever they look like.

**What this program inherits and does not re-open.** The re-home of
2026-09-04 that put these rows on S-TCOST routed them by path glob; one
of them (`malformed-ambient-eps-reds-review-m2-pr7-k`) says in its own
body that it *"is not a cost lever, so it does not belong on S-TCOST's
slate either"*. The re-home was not wrong to move them — they had no
other home — and nothing about any row's CONTENT is re-litigated by this
opening. Ids, titles, bodies, line citations and floors are unchanged;
several of those citations are frozen at a named SHA and are to be
re-derived at dispatch, not trusted.

**Nothing is dispatched.** No unit is cut, no branch is live, and the
slate's order is the first orchestrator's to set. The plan records the
two pairings worth not losing (`D383`+`S230`; `H12`/`S216`/`C18`/`D113`)
and the three rows that are Ev's decisions rather than work.

**Band 3500–3599** claimed in `docs/MODEL-AB-LOG.md` in this same
commit, per that entry's rule; the posture is inherited from S-TCOST's
test-only track, which records no A/B row, so the band is bookkeeping
until Ev says otherwise.

**(SYM orchestrator) Seam announced, 2026-09-13 — SYM-1** (`sym/1-profile`,
`docs/SYM-1-SPEC.md`): the profile inside `geom_core::sym` that
`work/sym/symbolic-tier-costs-95-percent-of-the-m10-3-drive` asks for
first. One new row file under `crates/editor-core/tests/m10_*`
(subject-named, `m10_sym_profile_interval.rs`), registered in
`tests/all.rs`, every row `#[ignore]` with a reason or gated behind a
test-only feature — the hosted gate's wall does not move, and the PR
states both runs' editor-core interval shard timings. S-TCOST's
measurement stays the result of record; nothing here re-takes it.

**(SYM orchestrator) Seam announced, 2026-09-14 — SYM-5** (`sym/5-unit-vector`,
`docs/SYM-5-SPEC.md`): one new row file under `crates/editor-core/tests/m10_*`
(`m10_derived_frame_interval.rs`, DOCM's two red derived-frame rows
ported and, once answered, un-ignored as the unit's pins at the
nominal and one width), registered in `tests/all.rs`; the evidence
rows stay `#[ignore]`; the pins' cost is stated in the PR body.

## First orchestration session opens (2026-09-15)

S-TINT has run no lane since it opened: no `tint/` branch exists, no PR
carries the prefix, every one of the 42 rows is `open`, and no row in
this directory sets `needs_ev`. So nothing was waiting on Ev at the
start of this session, and the slate's order is still unset.

**Review posture ratified (Ev, in-chat, 2026-09-15).** The plan's
§*Review posture* recorded the S-TCOST inheritance as *"open for Ev to
reset"*; Ev's answer resets nothing and confirms both halves: **no A/B
row and no A/B protocol**, and **one style review per unit** against
`docs/prompts/reviewer-style-lane.md` by path, with a full
(claims-falsification) review reserved for **the hardest units only** —
a per-unit judgement named in the unit's PR with its reason, not a
default. The band 3500–3599 stays bookkeeping.

**A row's body is not evidence about today's tree, and the first
spot-check proved it.** `m10-4-bore-pin-row-red-at-interval-1e-6` (filed
2026-09-03, GitHub 1646) describes an absolute `1e-9` slack that is
vacuous at ε = 1e-12 and red at ε = 1e-6. That assertion is **gone from
the tree**: `crates/editor-core/tests/m10_4_r2_probes_interval.rs`'s
`the_bore_pin_fit_as_a_consumer_reads_it` now pins the hull at both ends
against `0.2 ± half`, states the padding per half-width, and carries a
comment repairing exactly this defect and citing issue 1646 by number —
*"No absolute slack: an ε-independent term says nothing at the tight rows
and the wrong thing at the loose ones"*. The M10 lane closed it and the
row on this slate never learned. A second spot-check
(`torus-tangency-shell-floor-does-not-scale-with-k`) still reproduces:
`away()` is still a fixed floor and `bool3_torus_doors.rs`'s clearance
assertion still tells its reader to raise it.

One stale row in two reads is not a rate, and neither reading closes
anything. What it settles is the ORDER: the plan's instruction to
re-derive a frozen citation *at dispatch* is too late when the question
is whether a row is a row at all, so **the first act of this program is a
re-derivation pass over all 42 rows against the current tree** — does the
defect still reproduce, and is the row's floor still its floor — before
any unit is cut. The pass is bookkeeping about the slate, not work on the
suite, and it lands as item-file edits (closures with their evidence, or
re-derived citations) rather than as a unit.

## D113 ruled and closed (2026-09-15)

**Ev, in-chat: an alternate citation format would not actually prevent
drift, so do nothing.** `D113` asked what an intra-doc link in a
`tests/` file is and undertook to land the mechanism the answer needed.
The answer is that it is prose, checked by nothing, and that is
acceptable: no ban, no de-linking sweep, no gate. Closed.

The row's own fork — *"the form stops being used, or something is built
that resolves it"* — read as two live options and was one. Converting
`[Foo]` to `` `Foo` `` removes a promise of a hyperlink that never
rendered and adds no check; a stale link and a stale backtick are the
same defect, and the spelling is not what rots. So the real fork was
always *build a checker or don't*, and **a checker would key on an
identifier in a doc line whichever brackets are around it** — it never
needed this decision. The format question is retired as a red herring.

**The measurement that closed the other half** (`work/tint/D113.md`
carries it in full, taken this session): `cargo doc` has no test-target
selection at all; `cargo rustdoc` does, but cargo will not pass
`--extern` for a package's own lib when documenting its test target, so
the doc unit has to be assembled by hand-injecting an rmeta chosen by
trial and sensitive to feature unification. And **rustdoc does not
document `#[test]` items** — verified twice, once by the lane and once
independently here with a four-link control crate, where the module doc,
a plain `fn` and a `const` were all reported and the `#[test]` one was
silently dropped. That is **282 of 1086** candidates invisible, one of
this row's own six named breakages among them. Cost, for the record,
since it was the question asked: ~+170 s on a 4-core box, ~3–6 minutes
added to the hosted `fmt` job — affordable, and not the reason to
decline.

**Two things are deliberately NOT scheduled**, so that nobody
re-discovers half of this and re-opens it. The ~62 identifier-shaped
citations under `tests/` that do not resolve today stay unfixed: a
one-time cleanup with no guard behind it re-rots, which is Ev's own
argument applied to the cleanup rather than to the format. And this
row's census is stale by ~2.5× (465 files / 294 candidates at
`cfdc1c6f`, against 1146 tracked files today) and is not being
re-derived, because nothing now consumes it. Both facts are recorded on
the row, which dies with this directory when the program closes; if the
trade is re-taken it is re-taken as *write the checker*, never as *fix
the 62*.

**The four-sided pairing the plan recorded is dissolved and three sides
remain.** `H12`, `S216` and `C18` share `D113`'s root cause — rustdoc
collects nothing from a `tests/` target — but none is a citation-format
question, and `H12` is sharper than anything `D113` was about: eleven
`compile_fail,EXXXX` blocks in `geom-core/tests/review_m0_pr2.rs` and
`review_m0_pr3.rs` are negative proofs no tier has ever evaluated, which
start passing silently the day a bound is loosened. Shape 1, not shape 4.
Plan updated.

## D70 ruled and closed; two kernel defects filed off-slate (2026-09-15)

**Ev, in-chat: *"if the reframed D70 is 'should they be conditional?'
then the answer is no, we should fix the defect (or, like, you would
file the issue for fixing the defect and move it off your slate)."***
Done, and the row is closed with nothing scheduled here.

The thirteen silent stand-downs are **two kernel defects wearing one
test-suite costume**, which is why the row read as test work for a
month. Twelve stand down on the plane × NURBS certificate — limb 2's
bound is a function of a fixed sample schedule (`PXN_FIT_SAMPLES = 33`,
`CERT_SAMPLES = 9`, `PXN_IMAGE_DEGREE = 1`, all `const`) and does not
refine with ε, on a carrier that is EXACT by construction: both walls
come from one `loft_body` call, the seam edge is simultaneously the loft
of a profile vertex and a boundary iso-curve of the bowed patch, and
every control point of that corner has `y = −scale` exactly. Filed on
TRIM, which owns `edge_nurbs.rs`. The thirteenth stands down on
`SSI_MAX_FIT_SAMPLES`, a `const` cap on a sample count the marcher grows
as ε shrinks; filed in `work/issues/` because no open program's `paths`
claim `crates/geom-brep/src/ssi.rs`, with both readings (resource cap vs
class boundary) and the measurement that distinguishes them.

**The route to the answer is worth recording, because the row's own
framing hid it.** Ev's question was *"if it's a valid intensional
description then it should be valid under any decrease in epsilon
(unless it's ulp-scale)"* — and at metre coordinates ε = 1e-12 is ~4500
ulps, so it is not. That test is what turned a test-hygiene row into a
kernel row in one step. Everything this program had proposed before it
(announce the skips; re-mine the fixture onto `INTERIOR_COLUMN_SCALE`)
was a way of LIVING WITH the ε-conditionality, and one of them —
shrinking the model until a fixed bound fits — had already been taken
once under #1167 and would have propagated a workaround into eleven more
fixtures. **A row that says "this test stands down" is worth asking the
kernel about before asking the suite about.**

**What stays here.** `work/tint/loud-stand-down-announcements-are-discarded-by-the-gate`
— found while gathering D70's context, not about D70, and it survives
those thirteen disappearing. Nothing else from D70 is carried: its
"13 is a FLOOR" qualifier, its 116-`continue`-site exclusion and its
`#[ignore]`d-collection-run standing note were framing for a question
that is now answered, and they die with the row rather than being
re-filed as work nobody asked for.

**Board: two of the slate's three decision rows are now closed** and
neither cost a unit. The remaining one is "any change to a `memories/`
clause", which is not a row. 40 items live.
