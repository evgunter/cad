---
id: fuzz-rows-discard-trials-against-a-floor-that-counts-them
kind: issue
title: A fuzz row's continue shrinks its own sample while its coverage floor is written against the attempted trial count: twelve candidate files
status: open
opened: 2026-09-11
priority: P3
cost: E
---


**Filed for S-TINT at the landing of the `cert10` fix (2026-09-11), by
the lane that hit the instance.** This program opened the same day, for
test-suite integrity, and this is that class exactly: rows that are
sound in what they assert and wrong about how much they assert it over.

## The shape

A randomized row draws `trials` cases, `continue`s past the ones its
subject refuses, accumulates evidence only from the survivors, and then
floors that evidence against **`trials`** — the count it attempted, not
the count it measured. Nothing records the discard rate, so:

- the floor silently tightens as the discard rate rises;
- the failure message names a denominator the run never reached;
- and the row's own claim about its breadth ("a sweep of N random nets")
  is false by whatever the discard rate happens to be.

It fails as a *transient* red, which is the worst way for it to fail:
it reads as a flake and gets re-run, and the re-run passes.

## The worked instance, closed

`crates/mesh/src/nurbs_cert_fuzz.rs` —
`work/mesh/mesh-cert10-fold-fuzz-row-flakes-on-a-fresh-seed` has the full
measurement. Two rows in one file discarding **44%** and **32%** of their
trials, one of them reddening CI twice (2026-09-06, 2026-09-11) and being
diagnosed as a seed both times, with a failure message reporting 300
comparisons where 120 ran.

## The candidates, unmeasured

`continue` inside a `fuzz::`-driven loop, tree-wide, **not yet checked
one by one** — this list is a grep, not a finding, and each needs the
same ten minutes of instrumentation before anything is claimed about it:

- `crates/profile/tests/review_s8_probe.rs`, `review_s2.rs`
- `crates/sweep/tests/review_d8_consumer_differential.rs`
- `crates/geom/tests/curves/review_m5_pr4_adversarial.rs`, `r2_lt_probes.rs`
- `crates/geom-core/tests/ring_interval_differential.rs`, `d8_knot_queries_adversarial.rs`
- `crates/bvh/tests/ray_r2.rs`
- `crates/viewer/tests/review_gui2_r1.rs`, `review_gui2_r2.rs`
- `crates/editor-core/tests/gui1_pick_r2.rs`

A row only belongs to this class if it BOTH discards and floors against
the attempted count; a discard with no floor is a breadth question, and a
floor with no discard is fine. `cert10`'s sibling shows the middle case:
a discard under a **max**-shaped floor, which cannot fail the arithmetic
way but still quietly narrows the sweep.

## The cheap instrument

Counting is the whole fix, and it is three lines: count what the loop
actually measured, assert it against `trials`, and spell every reported
denominator from the counter rather than from `trials`. Whether that
wants a shared helper in `test-utils` — `fuzz::Sweep` holding the
attempted/measured pair and the floor — is this program's call and is
the reason this is one issue rather than twelve.

**Fences**: the candidate files are seven programs' ground. Each fix is
test-side and local, so it announces rather than asks — but the list is
not this program's to sweep unilaterally without saying so first.

## Re-derived (2026-09-15, lane B)

**VERDICT: PARTIAL** — the worked instance is still fixed, and the
twelve-file candidate list has now been checked one by one by READING.
Six rows in four files meet the row's own two-part test; the rest do not,
and two of the twelve carry the remedy this row asks for. No test was run,
so no discard RATE below is measured — every rate claim here is still
open.

### The worked instance — still fixed

`crates/mesh/src/nurbs_cert_fuzz.rs` carries both halves of the cheap
instrument at both rows: a `compared` counter asserted against `trials`
(*"{} of {trials} trials produced no bound, so this sweep is narrower
than …"*), and the denominator spelled from the counter — the comment
says so in as many words: *"The denominator is `compared * 5`, the
comparisons this run PERFORMED — never `trials * 5`"*. The 44% figure and
the `0xdcc78227f392d565` reproduction are both recorded at the site.

### The candidates, now dispositioned

The row's test: **BOTH** discards **and** floors against the attempted
count.

**MEMBERS (6 floors, 4 files)** — each is a floor whose denominator is the
attempted count while a `continue` shrinks the numerator's population:

| row (by name) | floor | attempted |
| --- | --- | --- |
| `profile/tests/review_s8_probe.rs::…arc_arc` (`dominance_arc_arc` sweep) | `stats.0 * 300 > trials` | `fuzz::scaled(18_750)`, 3 `continue`s |
| `profile/tests/review_s8_probe.rs::…line_arc` (`dominance_line_arc` sweep) | `stats.0 * 300 > trials` | `fuzz::scaled(18_750)`, 2 `continue`s |
| `profile/tests/review_s2.rs::offset_carrier_tangency_and_bulge` | `n_ok * 40 > corners` **and** `n_arc_arc * 400 > corners` | `corners = fuzz::scaled(1_500)`, 3 `continue`s |
| `geom-core/tests/ring_interval_differential.rs`, the `dinterval` lane | `t.wider + t.identical + t.tighter > 3 * n` | `n` rounds, 2 skip-filter `continue`s |
| `geom-core/tests/ring_interval_differential.rs`, the `interval_scalar` lane | same expression | same shape |

`review_s8_probe.rs` also carries the **second** symptom the row names —
a denominator the run never reached — in a `println!` rather than a
failure message: `"[s8 arc x arc] {trials} trials, {} two-survivor
corners"`, where the substantive assertions (`stats.1 == 0`,
`stats.2 == 0`) only ever saw the survivors of three geometric filters.
`ring_interval_differential.rs`'s message is the same shape:
`"lane ran too thin: {} verdicts over {n} rounds"`.

**Mitigation that is real and does not clear them:** in all six the floor's
NUMERATOR is itself a measured survivor count printed in the message, so a
failure names both numbers. That is half of the three-line instrument; the
missing half is the assertion on the discard rate itself, which is what
kept `cert10` reading as a flake.

**TWO already carry the remedy — not members:**

- `geom/tests/curves/r2_lt_probes.rs::the_meter_is_sound_on_random_integral_nets`
  counts the discards (`built += 1`) and asserts them against the attempt:
  `assert!(built * 4 >= cases * 3, "fuzz rot: only {built}/{cases} nets
  constructed …")`, with the comment *"COVERAGE FLOOR, proportional to the
  case count: three quarters of the draws used to yield a constructible
  net (300/400)"*. **Residue worth noting:** a SECOND `continue` (the
  `m.is_nan()` poison abstention) is not counted, and the soundness
  assertion runs only past it, so the instrument covers the first discard
  and not the second.
- `geom/tests/curves/review_m5_pr4_adversarial.rs`, the F1(a) fit-bound row
  — `fits += 1` then `assert!(fits * 2 > cases, "too few successful fits to
  mean anything: {fits}/{cases}")`, with the ratio also printed.
  Same shape, same completeness.

**NOT THE CLASS (discard without a floor against the attempted count):**

- `sweep/tests/review_d8_consumer_differential.rs` — `continue`s in the
  skin and fit lanes; its only floor is `assert!(groups >= 5, "only
  {groups} section groups were compatible")`, an ABSOLUTE constant. That is
  the middle case the row already describes: it cannot fail the arithmetic
  way, but it still narrows the sweep silently.
- `geom-core/tests/d8_knot_queries_adversarial.rs` — its `continue`s are in
  corpus-building helpers, and its floor `cases.len() > 100` is over a
  constructed vector, not a fuzz draw.
- `viewer/tests/review_gui2_r1.rs` — one `continue` (*"behind the eye: not
  this row's subject"*), no coverage floor at all. A breadth question.
- `viewer/tests/review_gui2_r2.rs`, the cursor sweep — `continue` at the
  pick miss, and its floor is `assert!(hits >= witness.len(), "the static
  witness cursors did not all hit")`. **A STATIC WITNESS**, which is the
  correct shape; not a member.
- `bvh/tests/ray_r2.rs` — `continue` at a legal true miss; floors are three
  `seen.require(…)` per-category `Exposure` calls, not against the attempt.
- `editor-core/tests/gui1_pick_r2.rs` — three `seen.require(…)` per-category
  floors; its only `continue` is inside a source-lexing helper, not a fuzz
  loop. Not a member at all.

**New floor: 6 floors in 4 files**, down from twelve candidate files — and
two of the twelve are worked examples of the fix.

### Blind spot

This was settled by reading, not by instrumenting: **no discard rate below
was measured**, so "the floor silently tightens as the discard rate rises"
remains a shape argument at all six sites. The ten minutes of
instrumentation the row asks for is still owed at each. The candidate list
itself was inherited, not re-swept — I did not re-run `continue`-inside-a-
`fuzz::`-loop tree-wide, so a row that acquired the shape since the list
was written is not here.

### Recommendation

Do not close; narrow it. The row is now six named floors rather than twelve
unmeasured files, and `r2_lt_probes.rs` / `review_m5_pr4_adversarial.rs`
show the fix costs three lines in this tree's own idiom. Whether that
wants `fuzz::Sweep` in `test-utils` is still this program's call.

## A half-instrumented member the sweep classified as fixed (S-TINT orchestrator, 2026-09-15)

Lane B classified `crates/geom/tests/curves/r2_lt_probes.rs` as already
carrying the fix (`built * 4 >= cases * 3` floors the counted discard),
and that is right about the FIRST discard and wrong about the row. In
`the_meter_is_sound_on_random_integral_nets` a **second** `continue` —
the `m.is_nan()` poison abstention — is uncounted, and the soundness
assertion runs only past it. So the row counts one of its two discards
and floors against the counted one.

**This is the class's most dangerous shape, not an exception to it**: a
row that has been instrumented once reads as done, and the second
discard is invisible precisely because the first was fixed. Any sweep
keyed on "does this row count its discards" answers yes here. The
instrument has to be "does it count ALL of them", which is a read and
not a grep — and that is this row's blind spot, now stated.

The row this rode on, `mesh-cert10-fold-fuzz-row-flakes-on-a-fresh-seed` (closed on S-MESH's slate), left the tracker with `work/mesh/` at DOC-LEDGER sweep 16; it is recoverable at `git show 9f043ec2712b:work/mesh/mesh-cert10-fold-fuzz-row-flakes-on-a-fresh-seed.md`.
