---
id: measurements-have-no-mechanical-guard
kind: issue
title: Measurements have no mechanical guard — the size_of pin from PR 646 is the only one in the tree
status: open
opened: 2026-08-19
github: 651
refs: [613, 614, 646, 667, 681]
---

## From GitHub issue 651

Opened 2026-08-19; 2 comments.

**Class finding, raised by the style review of #646 and discharged here rather than absorbed into that PR** (`docs/REVIEW-STYLE-BRIEF.md` §4, the class-not-instance rule: *"the fix pass either sweeps or records why not"*).

#646 turned one remembered measurement — PR #291 MAJOR-2, *"`Lit` is one `f64` plus a one-byte code"* — into `const _: () = assert!(size_of::<Lit>() == 16, …)`. Re-inlining the 32-byte `UnitDef` row now fails the build with a message naming the PR. That is a good precedent. It is also, as far as the tree goes, **unique**.

## The measurement

- `size_of` appears **nowhere else** in `crates/*/src` (`grep -rn size_of crates/*/src` → only `expr.rs`, after #646).
- The workspace has exactly **two** other `const _: () = assert!` — `mesh/src/probe_stats.rs:151` and `mesh/src/budget.rs:719` — and both pin **feature-flag state** (`!armed()`), not a measurement.
- **136 lines** in `crates/*/src` contain the word "measured". Approximately none of them has a mechanical guard.

So the repo's standing pattern is: measure, write the number into a comment or a doc row, and rely on a reader noticing when it stops being true. §C14 already names the adjacent failure (*"pins guard the invariant as it was reachable then"*); this is the weaker case where there is no pin at all.

## The question to ask each row

**"What goes red if this stops being true?"** Where the answer is "nothing", either add the guard or delete the number, because a stale measurement in a comment reads as evidence rather than as history.

## Named places to look

Not exhaustive — these are where the style review pointed:

- **`Cargo.toml:165-175`** — the `spade` / `mesh` `opt-level = 2` block, *"measured 2026-07-21"* (one washer test: 91.7s at opt 0 vs ~1.2s optimized). Nothing detects its removal, and the symptom is a slow suite that nobody attributes to the missing block. Arguably the highest-value row here: it is cheap to guard and expensive to lose silently.
- `geom-brep/src/props/quad.rs:2052`, `:3213`, `:3256`
- `ssi/march.rs:146`
- `geom-core/src/dual.rs:1431`
- `pcurve_cache.rs:2036`
- Every row in `PERF-SCAN`, `PERF-PLAN`, `GENERICS-BUILD-COST`, `TESS-BUDGET` and `K-REPORT`.

## What this is not

Not a demand that every measurement become an assertion. Several will be genuinely unguardable, or guardable only by a flaky timing row that is worse than the comment. **A recorded "unguardable, and here is why" is a complete answer for a row** — the defect is the absence of the question, not the absence of a guard.

Refs #613, #614, #646.

## Comments

**2026-08-19** — comment:

## The sweep — partial, prioritised, 9 rows (14 sites + one 6-document class)

Ratified by Ev as **a rule plus a sweep**. The rule landed in
`docs/REVIEW-STYLE-BRIEF.md` §Q6; this is the sweep.

**Amended 2026-08-19** after the style review of #663 — three corrections,
each marked inline: row 9's verdict word, the "exactly one extraction"
negative, and the "five of eight" tally.

The question asked of every row is the one this issue names — **"what goes
red if this stops being true?"** — and "nothing can, because X" is a
complete answer. Verdict vocabulary:

- **GUARDED** — something already goes red. The row's defect was that the
  guard was not named at the claim site, so a reader could not tell.
- **GUARD LANDED** — nothing went red; something does now.
- **UNGUARDABLE (reason)** — nothing can go red, and here is why.

---

### 1. `Cargo.toml:174-177` — the `spade`/`mesh` `opt-level = 2` block. **GUARD LANDED.**

The highest-value row, and now doubly so: **#470's deferral is gated on this
measurement** (*"It only becomes interesting if the opt-level-2 revert lands
and the ε rows get expensive again"*). If the block disappears, #470's "why
not now" silently becomes wrong, and nothing anywhere says so.

What goes red now: `crates/mesh/tests/profile_overrides.rs`,
`dev_profile_still_optimizes_spade_and_mesh`. It reads the workspace
manifest and fails if either `[profile.dev.package.{spade,mesh}]` section is
gone or stops saying `opt-level = 2`, with a message carrying the 2026-07-21
measurement (91.7 s at opt 0 vs ~1.2 s optimized) and the instruction to
re-measure rather than quietly restore. It carries an anti-vacuity assertion
in the `pncad-py` house style: the scanner keys on a literal header line, so
it first proves `[workspace]` is findable, and fails loudly if the manifest
format changed underneath it.

**Be precise about what it pins: the DECISION, not the timing.** A timing
cannot be asserted here — it is box-dependent and belongs to the reporting,
never-gating perf lane (`memories/perf-measurement-lane.md`). What vanishes
silently is the *stanza*, and that is what this row catches. Widening the
block does not fail the row; the scope note beside it already records why
extending it to the geom crates was measured and rejected (#52).

Manifest-text guard, with two in-house precedents:
`pncad-py`'s `crate_lints_match_the_workspace_minus_unsafe_code` and this
crate's own `every_suite_file_is_aggregated`. No new test binary — it is a
`#[path]` module in `mesh`'s aggregated `tests/all.rs`.

### 2. `props/quad.rs:2052` — "the practical envelope (measured; every row re-measured)". **GUARDED.**

Every one of the ten carrier rows in that table is a named `const` in
`crates/geom-brep/tests/review_r1_rational_probes.rs:74-91`
(`MOEBIUS_FLOOR`, `QUARTER_TORUS_FLOOR`, `HALF_CYLINDER_FLOOR`, …), and
`pin_floor` (`:134`) asserts each **two-sidedly** at 0.1 % relative — *"a
moved floor is a changed enclosure — re-measure and re-pin deliberately,
never widen this bound"*. The degenerate row is pinned separately by
`pin_degenerate`. This is the strongest guard in the sweep and the doc
comment already names the file. **No action.**

### 3. `props/quad.rs:3213` — "MEASURED: 64 → 4 moves the oracle by at most 2.3e-13". **GUARDED, by construction.**

The claim is that dropping the oracle's v-samples 64 → 4 is free. It sits
inside the test whose posture assertion (`eps_posture(…, 1e-5)` at `:3266`)
is exactly the thing that goes red if it were not: the oracle it feeds is
the comparand. The 2.3e-13 figure itself is unpinned, but nothing rests on
the *number* — only on the conclusion, which the enclosing row tests
directly. **No action.**

### 4. `props/quad.rs:3256` — "2048 u-samples … 2.6e-7 off it (O(h²), measured)". **GUARDED, same mechanism, same row.** The 1e-5 slack is stated as 40× that headroom; if the discretization error grew past ~1e-5 the assertion fails. **No action.**

### 5. `ssi/march.rs:146` — "conservative … by a factor measured at roughly 4–20× on the M5 fixtures". **UNGUARDABLE, and correctly so.**

It is a ratio between an enclosure and a truth over a fixture *set*, at no
single point — there is no expression to assert. It also does not need one,
and the comment three lines above already says why: *"the certificate —
never this constant — is what refuses when the target is missed"*.
`SSI_STEP_DEVIATION = 0.02` is set well below ε **because** the factor is
untrusted; if it degrades, the certificate refuses rather than the constant
being wrong. The measurement is motivation for a conservative constant, not
a premise anything computes with. **No action** — the fail-safe is the
guard, and it is stated.

### 6. `geom-core/src/dual.rs:1431` — "the census in `real.rs` measured their divergence at ≤ 4 ulps over 20k samples". **GUARDED — remotely, and it was not said where.**

`libm_vs_std_divergence_census` (`real.rs:1004`) is a live test, and it does
not merely report: `real.rs:1020` is `assert!(d <= 4, "libm and std diverge
by {d} ulps at x = {x} — beyond sanity bound")`. So the 4-ulp number is one
of the better-guarded measurements in the tree, and the reader of `dual.rs`
had no way to know — which is the whole failure mode this issue describes,
in its mildest form. **Landed: the comment now names the test and says it
asserts rather than reports.** (This is the only row where the fix is a
pointer, and it is the only row where the guard existed but was unfindable
from the claim.)

### 7. `pcurve_cache.rs:2036` — "measured at 7 orders on an attach-path `pl.x = 1 + 0.6e-9`". **GUARDED, and correctly named already.**

The comment names `envelope_dominates_a_winding_snapped_pcurve`, which
exists at `pcurve_cache.rs:4041`. The snap-slack term the measurement
motivates is also *exactly zero on the minted path* by construction, so the
live exposure is the foreign-input path the probe drives. **No action** —
and this row is the model for what §6 was missing: name the probe at the
claim.

### 8. `crates/mesh/src/lib.rs:148-175` — the CDT timing prose. **UNGUARDABLE, and it has already drifted.**

Not on the issue's list; found by the sweep and included because it is the
class's best evidence. The section derived *"~10⁴× more CDT time"* per 100×
δ tightening. It is wrong by ~150× **on its own datapoints** (the real
figure is 63×), and it was wrong for three milestones. Nothing went red.
What caught it was `docs/PERF-SCAN-2026-08.md` finding 7b — a scan, a
milestone later, run for another reason. The corrections are now inline in
the module docs.

This is the honest shape of an unguarded wall-clock claim: it does not
fail, it just quietly stops being true, and the recovery mechanism is
someone re-running the measurement for an unrelated purpose.

### 9. The documentary registers, as a class: `PERF-SCAN-2026-08.md`, `PERF-PLAN.md`, `GENERICS-BUILD-COST.md`, `LOCAL-BUILD-PERF.md`, `TESS-BUDGET.md`, `K-REPORT.md`. **UN-ASSERTABLE — and the rule does not bind them.** (Was "unguardable by construction"; corrected below.)

Asked as a class, per §4's class-not-instance rule, because row-by-row is
the wrong altitude here: these six documents contain hundreds of numbers and
**not one of them is a standing claim**. Each is a *dated observation on a
named box* — which is precisely the posture
`memories/perf-measurement-lane.md` ratifies (*"a timing is worth nothing
without its box; history is append-only, reporting never gating"*). A dated
observation cannot go stale, so there is nothing for a guard to catch. Every
one of the six says so in its own header: **REPORT ONLY** (PERF-SCAN,
SMELL-SCAN), **MERGED AND ADVISORY** (PERF-PLAN), **FINAL** (K-REPORT),
**measurement complete** (TESS-BUDGET), *"measured findings (date)"*
(GENERICS-BUILD-COST, LOCAL-BUILD-PERF).

**Their actual guard is the periodic re-scan, and it demonstrably works:**
PERF-SCAN §6 retired four of PERF-PLAN §1.3's six rankings and annotated
each expired claim inline with a dated `[STALE …]` / `[SUPERSEDED …]`
marker, leaving the original prose intact. That is a real mechanism. Its
weakness is not that it is prose — it is that **it is unscheduled**. Nobody
owes the next one, and row 8 shows the latency: three milestones.

**Correction (2026-08-19): "unguardable BY CONSTRUCTION" was too strong,
and the counterexample is this branch's own tip commit.**
`.github/workflows/ci.yml:1316-1420` runs `rebuild latency (reporting)` on
every merge to `main`: it measures, compares against a committed structural
baseline (`crates/editor-core/tests/m4_pr8_latency.rs:74`, `:203`), and
appends the result to `docs/perf-data/rebuild-latency/` as a commit. That is
a **scheduled, mechanical, non-gating measurement register** — neither a
compile-time assert nor a written excuse — which is precisely the mechanism
this row says the registers lack. So the accurate word is
**un-ASSERTABLE**: a dated observation cannot be asserted, but it can be
re-measured on a schedule, and the scheduling problem this row calls the
real weakness **has a solved in-tree instance** nobody had cited. This does
not propose wiring the six documents into one job — six documents' worth of
numbers is not a per-merge measurement, and their content is mostly one-shot
investigation rather than a repeatable figure. It corrects a claim of
impossibility that is disproven twenty lines from where this branch commits
its own measurements. §Q6 now names the option as the third answer.

**The guardable residue is not in the documents. It is wherever a document's
number was extracted into a live decision** — a constant, a manifest stanza,
a schedule. That is the useful output of asking the class question: not
"guard the registers", but "find the code that inherited a register's
number and forgot to say so".

**Correction (2026-08-19): "exactly one such extraction" was an unverified
negative.** §C15 applies to the *extraction* search as much as to the
`measured` grep, and the extraction search had no stated pattern. Stated
now: `grep -rn 'PERF-PLAN\|K-REPORT\|TESS-BUDGET\|GENERICS-BUILD-COST\|LOCAL-BUILD-PERF\|PERF-SCAN' crates/*/src`
— 22 hits, most citing a *plan* rather than a *number*, and reaching sites
the `measured` grep cannot (row 1's own site among them). Its blind spot is
the one that matters: an extraction that **dropped the document's name** is
invisible to it, and that is the likeliest shape of a forgotten one. It
surfaces at least one further unguarded extraction of row 1's shape —
`geom-core/src/tolerance.rs:58` and its re-export `predicate.rs:127`, where
`DEFAULT_K = 10.0` is retained on K-REPORT's finding of *no empirical
pressure to move it*. So the honest claim is: **row 1 is the extraction this
sweep guarded, not the only one that exists.**

---

## Where this stopped, and what is left

**Stopped after row 9, deliberately.** The count in the issue has moved:
`grep -rn measured crates/*/src` is now **141** lines, not 136. This sweep
addresses **14 of them directly** plus the six-document class — roughly a
tenth, chosen as the issue's own priority order plus one addition (row 8).

The remainder is a **known** quantity, not an unknown one. Ranked by density
of the word, the unswept concentrations are:

| file | "measured" lines |
|---|---|
| `mesh/src/nurbs_cert.rs` | 11 |
| `profile/src/sugar.rs` | 7 |
| `geom-brep/src/edge_nurbs.rs` | 7 |
| `step-import/src/{recognize_curve,recognize,entities}.rs` | 15 |
| `mesh/src/{probe_stats,chords,curved,budget}.rs` | 17 |
| `editor-core/src/mate/coset.rs` | 5 |

**One caveat on that grep, per §C15** (*a sweep's result is worth nothing
without a statement of what its pattern cannot match*): `measured` is a
weak proxy. It over-matches — several hits are `the measured width` as a
field description, which is a *name*, not a claim — and it under-matches
badly, missing `benchmarked`, `profiled`, `timed at`, `we found`, bare
numeric constants with a provenance comment, and every measurement that
was written down as a number with no adjective at all. **The real
population is larger than 141 and this sweep does not know how much
larger.** A follow-on that wants completeness should start by fixing the
pattern, not by continuing down this list.

## What the sweep found, as a shape

**Only one row was genuinely naked** (row 1); two are genuinely unguardable
(rows 5, 8), one class is (row 9), and the rest have *something* that goes
red. **Do not lean on a tally.** The style review of #663 priced rows 3 and
4: each is pinned by an enclosing assertion whose slack is ~4e7× (row 3) and
~40× (row 4) looser than the figure the row states, so they guard the
*conclusion* against collapsing, not the *measurement* against drifting.
Counting them beside rows 2, 6 and 7 as "guarded" flattens a real
difference, so the earlier **"five of eight" phrasing is withdrawn**.

The conclusion survives the pricing: the tree is in better shape than the
headline count suggests. And note
*where* the failures cluster: in rows 2, 6 and 7 the difference between a
good row and a bad one was **whether the claim site named its guard**.
Row 7 named its probe; row 6 did not, and its guard is stronger than most.
That is cheaper than adding guards and it is most of the value the rule is
after — which is why the rule's second clause is about writing things down
rather than about asserting them.

Refs #613, #614, #646, #470.

**2026-08-20** — comment:

# The measured-claim sweep, continued (#667) — the ~90% #663 tabled

Continues [#663's sweep](https://github.com/evgunter/cad/issues/651#issuecomment-5344413746)
on the same issue, as its second comment rather than in a new document
(§C3: a prose register nobody executes is not a register, and a second one
is worse). #663's rows are carried forward with their classifications
unchanged; nothing below re-litigates them.

---

## 1. The pattern, fixed first

#667's stated first unit, and §C15's obligation. `grep -rn measured
crates/*/src` is **146** lines today (136 when #651 opened, 141 when #663
closed — it drifts upward under normal work, which is on its own a reason
not to quote it as a population).

**The replacement instrument.** The runnable script is in
**[#681](https://github.com/evgunter/cad/issues/681)**, not in a PR body
and not in-tree — a one-shot grep nobody runs is §C3's failure mode, and a
script that lives only in a PR description is how the next lane ends up
re-deriving it from prose (§C15). Its shape:

* **provenance vocabulary**, word-bounded, not a substring: all forms of
  `measur*`; `benchmark*`; `profiled`/`profiling` (`\b`-anchored —
  unanchored `profil` matches `ProfileDoc` 40 times); `timed at`,
  `wall-clock`, `wall time`; `empiric*`; `calibrat*`; `hand-tuned`;
  `in practice`; `observed at|max|min|worst|to be`, `we observed`,
  `observed on`; `experimentally`; `in the wild`; `on this|the box`,
  `on the M<n>|CI`; `speedup`, `slowdown`; `<n>x faster|slower|cheaper|…`;
  a bare `<n> ms|µs|ns|sec|minutes`; and the six register document names
  plus `perf-data`, `TESS-SPAN`, `rebuild latency`.
* **restricted to comment text** (everything after `//` on the line), which
  is where a *claim* lives.
* **deduplicated to the contiguous comment BLOCK**, because a claim is a
  paragraph, not a line.
* **numeral test**: a block containing no digit is not resting on a
  measurement.

**The pipeline, measured at #667's merge base:** 346 matching comment lines
→ **257** blocks carrying a provenance word → **197** of those carrying a
numeral. All 197 triaged.

*(The listing this triage was worked from said 256/196. The one-block
difference is a cut detail in the no-claim tail and moves no row — but it
is the reason #681 carries the script rather than the numbers.)*

**Which filter actually does the work — correcting this comment's first
version.** It credited the comment restriction. Measured, on the same tree:

| variant | matching lines |
|---|---|
| unanchored vocabulary, unrestricted | 1 599 |
| unanchored vocabulary, comments only | 940 |
| **`\b`-anchored, unrestricted** | **361** |
| `\b`-anchored, comments only | 346 |

The **word-anchoring** is the filter (1 599 → 361); the comment restriction
adds ~4% on top. The "478 hits unrestricted" figure quoted earlier does not
reproduce under any variant and is withdrawn. This matters for #681: the
`//` restriction is cheap to drop, so its `.md` legs are more affordable
than the original framing implied.

**Against #663's instrument**: it covers 140 of the 146 `measured` lines,
and adds **206 lines the baseline could not see**. The 6 it drops are all
non-comment (two `format!` strings, one `fn` name, `unmeasured`) — see
blind spot 1.

### What this pattern still cannot match

1. **Claims outside comments.** Deliberate, and it costs real rows:
   `edge_nurbs.rs:192` asserts a measured value inside a runtime message.
2. **A measurement with no provenance word and no unit** — the dominant
   hole, and the same one #663 named. `MAX_GRID_RETRIES = 6`
   (`mesh/src/trimmed.rs:107`) was findable only because its comment
   happens to say *"6 since TESS-SPAN (was 4)"*. A constant retuned with a
   silent comment is unreachable by **any** textual pattern; this is not a
   pattern that can be improved, it is a class only a reviewer's question
   finds.
3. **The bare-number arm is TIME UNITS ONLY** —
   `ms|µs|us|ns|sec|secs|seconds|minutes`. A measurement in **bytes,
   percent, counts, or a bare factor** ("35×" with no following word) has
   no unit arm at all and is reachable only through the vocabulary. So
   hole 2 as stated above reads narrower than the instrument really is:
   the unguarded shape is not just "no provenance word and no unit", it is
   "no provenance word and no *time* unit". #646's `size_of` pin
   (`editor-core/src/expr.rs:320`, *"32 bytes … took it to 40"*) is exactly
   that shape and was caught only because its comment also says
   *"measurement"*. #651's founding instance would have been invisible to
   the unit arm.
4. **Block dilution both ways.** The numeral test is block-scoped, so a
   200-line module header passes on an unrelated number; and a one-line
   comment whose number sits three lines below in *code* fails.
5. **Everything outside `crates/*/src`** — the largest known hole in both
   sweeps. `tests/`, `tools/`, `scripts/`, `demos/`, `docs/`,
   `.github/workflows/`, `crates/pncad-py`'s Python, and the manifests.
   **#663's one confirmed unguarded extraction was in `Cargo.toml`, i.e.
   outside the population either sweep searched.** It was found by
   accident. Filed as **#681**. Two additions to that list found while
   writing it up, because "everything outside `crates/*/src`" is not a
   scope statement anyone can act on:
   * **`interval-transcendentals/`** is a whole 1 405-line Rust crate
     outside `crates/*/`, `exclude`d from the workspace and reached as a
     path dependency — invisible to every sweep so far *and* to
     `--workspace`. Measured: 3 blocks in `src/`, 0 in `tests/` and
     `examples/`; **one real claim** (`src/lib.rs:45`, ≤1/≤4 ulp vs
     inari, *"measured in the harness"*, whose named harness is behind
     the non-default `oracle-inari` feature). The other two are a new
     over-match: `\bon\ the\ box\b` fires on *"continuous on the box"*
     (`invtrig.rs:87`, `:121`), where "box" is an interval box.
   * **`crates/pncad/src/guide.rs:25-43`** pulls `docs/GUIDE.md` and three
     more files into rustdoc via `#![doc = include_str!]`. A measured
     claim in those four files is a **live doc-comment claim on a public
     module**, not "docs/ prose", and no `crates/*/src` scan can see it.
6. **Synonyms nobody has used yet** — "clocked at", "we timed", "on my
   machine", "reproduced at". Zero hits today. The pattern is a list, not a
   concept.
7. **An extraction that dropped the document's name** stays invisible —
   #663 named this for the extraction search and it is unfixed; my
   vocabulary includes the six document names, so it inherits the hole.

*(Checked and not a hole: `/* */` block comments. Three in `crates/*/src`,
none claim-bearing.)*

---

## 2. What the 197 blocks are

Triaged all 197 at block level; opened the file at 24 sites. **160 carry no
claim at all** — they are the field-name and geometric senses of "measure"
(*"the predicate measured a clash"*, *"angles measured from the rim's own
start"*), which is #663's §C15 over-match concern, now quantified.

**The other 37 blocks carry 39 claims.** The unit is the claim, not the
block: two blocks each carry a second, weaker claim beside the one they are
counted for — `profile/src/sugar.rs:862-898` (the 300 000-corner table
beside `LEVER_ULPS`) and `mesh/src/nurbs_cert.rs:356` (the *0.60·δ* tour
observation beside `SAFE_ASPECT`). Both are called out below. **37 blocks /
39 claims** is the reconciliation; #681's earlier "37 real claims" was the
block count carrying the claim label.

The over-match is *concentrated*, which matters for anyone continuing:

| #667's tabled concentration | blocks | real claims |
|---|---|---|
| `editor-core/src/mate/coset.rs` | 3 | **0 of 5** `measured` lines — all are *"the predicate measured a clash"* / *"its measured margin"*: a runtime value's NAME, never a standing claim |
| `mesh/src/walk.rs` | 7 | **0 of 6** — geometric, e.g. *"the detector below measures the gap in metres and gates nothing"* |
| `geom-brep/src/edge_nurbs.rs` | 4 | **1** of its 11 `measured` lines |
| `mesh/src/nurbs_cert.rs` | 9 | 6 |
| `profile/src/sugar.rs` | 3 | 3 |
| `step-import/{recognize_curve,recognize,entities,units,chart}.rs` | 16 | 9 |
| `mesh/src/{probe_stats,chords,curved,budget,trimmed}.rs` | 25 | 7 |

So #651's density table ranked by the wrong quantity: `coset.rs` and
`walk.rs` were pure noise, and the densest real cluster (`step-import`'s
corpus claims) was the one nobody flagged as interesting.

---

## 3. The classification

**39 claims: 22 guarded · 4 scheduled register · 12 unguardable with a
written reason · 1 unguarded.** Seven of the 39 are #663's rows, unchanged.

#663's shape holds and gets stronger: **the defect is almost never a
missing guard. It is a claim site that does not name the guard it has.**
Of the 32 new rows, exactly one had nothing at all — and that one is
unguarded rather than unguardable, which is a different (worse) thing.

### 3a. THE CORRECTION THAT MATTERS — there are THREE registers, two of them GATE, and each is NARROWER than the document it produced

#663 found `ci.yml`'s `rebuild latency (reporting)` and called it *the*
in-tree register, non-gating. It is one of three. **But the correction has
a second half, and the first version of this comment shipped only the
first: a register re-takes some COLUMNS of its document, not the
document.** Naming a register and then crediting the whole writeup to it is
the same defect this sweep exists to find, one level up.

| register | job | gates? | what it re-takes per merge | what it does NOT |
|---|---|---|---|---|
| `rebuild latency (reporting)` | own job, `ci.yml:1338` | no | per-document rebuild timings, one appended entry per merge in `docs/perf-data/rebuild-latency/`, each with its environment block | nothing compares a prose figure to the fresh entry. The quantity is live; the quotations are not checked against it |
| `tessellation-budget sweep` + `tessellation-budget lint` | inside `k-lint (gate)`, `ci.yml:1644` | **YES** | triangle counts and the **sizing** columns (`grid_cells / span_opt_cells`) for every tour face, against `docs/tess-budget-data/tess-budget-baseline.csv` — *a grown budget fails this row* | **the deviation half.** CI passes `--sizing-only` (`ci.yml:1645`), which skips the \|S − Π\| resample (`tess_budget_sweep.sh:25-34`), so `worst_dev` is empty and `tess_lint::Row::total_slack` (`tools/tess-lint/src/lib.rs:149-156`) is `None` on every fresh row. `docs/TESS-BUDGET.md`'s `total` column and its total-slack factors come from a `--deviation` run **nothing re-takes** |
| `K-telemetry probe sweep` + `large-K lint` | inside `k-lint (gate)`, `ci.yml:1651` | **YES** | the flag rules, evaluated against a fresh sweep of the Band-4 corpus + demo tour at ε ∈ {1e-6, 1e-9, 1e-12} — *a flagged margin fails this row* | **any comparison to `docs/K-REPORT.md`.** `tools/k-lint/src/lib.rs:61-71` says it outright: the lint *"lints the fresh rows it was handed and never compares them to the committed files"*, and *"the baseline is re-cut when the DISTRIBUTION moves … not on every merge and not on a rename."* K-REPORT's histogram, percentile re-argument and roster are a **snapshot** — the roster has already drifted 233 → 231 |

`k-lint (gate)` runs *"whenever anything builds at all"* by its own job
comment (`scripts/ci-filter.py:314` resolves its `if:` to `"false" if tier
== "docs" else "true"`), and neither gate row carries `continue-on-error`.
So both gates are real and per-merge — over the columns above and no
wider.

**Consequence for #663's row 9**, restated precisely. Its verdict on the
six documentary registers was *"their actual guard is the periodic
re-scan, and its weakness is that it is UNSCHEDULED — nobody owes the next
one"*. Corrections, in both directions:

* It is **wrong for the sizing half of `docs/TESS-BUDGET.md`** and for the
  **flag rules** the K sweep re-evaluates: those are gated per merge.
* It **stands for the deviation half of TESS-BUDGET, and for K-REPORT's
  histogram/percentile/roster content** — nothing re-takes those.
* And it needs narrowing on the other four too. `docs/PERF-PLAN.md:553-568`
  and `docs/PERF-SCAN-2026-08.md:98-127, :367-370` quote **rebuild-latency**
  figures, and since 2026-08-17 that quantity *is* re-measured and
  committed per merge (#663 identified this itself). So row 9 stands for
  four documents **minus their rebuild-latency rows** — where the honest
  statement is "re-measurable and re-measured, but not re-checked against
  the prose", which is weaker than a gate and stronger than a dated
  observation.

**Consequence for the row #667 carried forward as open.**
`DEFAULT_K = 10.0` (`geom-core/src/tolerance.rs:81`, re-exported at
`predicate.rs:116`) was #663's second candidate unguarded extraction. It is
a **scheduled register** row, and a gating one. **How much of the lint
actually moves when K moves** — settled, because it is not obvious from the
tree and a register credited wider than it reads is this section's own
defect. `escalate` = K·`zero`, so:

* rule (1) (in-band outcomes) tracks K at **all three** ε rows;
* rule (2)-above is `min(10²·Kε, BASELINE_FLOOR_MARGIN)`
  (`k-lint/src/lib.rs:19-24`, `:267`), so it tracks K only while the cap is
  inert — true at ε = 1e-9 and 1e-12 (the cap would need K ≥ 400 / K ≥
  4e5), **false at ε = 1e-6, where the cap already binds at the ratified
  K = 10** and keeps binding for any K ≥ 0.4;
* rules (2)-below, (3) and (4) key off `zero` = ε and
  `BASELINE_FLOOR_MARGIN` and do not read K at all.

So the register is real, gating, and narrower than *"re-measures the
distribution K was chosen against"*: K-sensitive at three ε rows through
rule (1), at two of them through rule (2). **Landed:** both sites now say
that, with the job, the sweep script, and where the thresholds are pinned.

### 3b. Guards that existed and were not named — the pointers landed

| claim site | what goes red | was it named? |
|---|---|---|
| `profile/src/sugar.rs:898` — `LEVER_ULPS = 128`, *"the ONE empirical number"*, backed by a 300 000-corner table | `review_s2::an_uncertifiable_tangent_point_refuses_instead_of_being_returned` carries the implied ε-crossover (≈2.53e-10) as a **deliberate tripwire literal** | no |
| `profile/src/sugar.rs:717` — `tangent_point`'s *"measured spoke"* | `review_s2::an_ill_conditioned_corner_lands_its_tangent_point_on_the_carrier` — 8 ulps on the worst gate-accepted corner, and *"restoring `R/ρ` fails it immediately"* | named the suite, not the row |
| `mesh/src/nurbs_cert.rs:356` — `SAFE_ASPECT = 5.0`, explicitly *"a MEASURED constant, not a derived one"* | three, none of which re-runs the tour measurement, and **each narrower than it reads** — `band_schedule_snaps_on_realized_aspect` fails only UPWARD (at realized aspect 9.09 and above; lowering the constant leaves every assertion in it passing); `tessellation-budget lint` catches the downward direction, but as mesh GROWTH rather than as a claim about the constant, and only through its sizing columns; `mesh certificate falsifier (feature = probe-stats)` falsifies under-certification over **four NURBS fixtures at two δ**, not the tour. Jointly a bracket; individually none of them is one | no |
| `mesh/src/budget.rs` — the meter, *"the diff-gate on growth, and the committed render cells"* | the two `k-lint (gate)` rows above, by name, **and the `--sizing-only` limit spelled out** | described, never located |
| `geom-core/src/tolerance.rs:81` + `predicate.rs:116` | `large-K lint`, with the K-sensitivity scoped as in 3a | no |

Already correct, no action: `geom-curves/src/nurbs.rs:85`
(`RATIONAL_METER_SPLITS = 16` names `tests/m5_pr7_speed_meter.rs`),
`profile/src/lift.rs:224` (`VALUE_EQUAL_ULPS` names the census suite as the
real backstop and says these constants are not it),
`topo/src/review_m1_pr4.rs:1320` (the *~0.1% of steps* / *~1 run in 150*
figures sit in a coverage row that fails loudly),
`topo/src/sector_shape.rs:152` (`the_rungs_are_decided_in_one_place`),
`mesh/src/chords.rs:748`, `mesh/src/curved.rs:993` and `:1296`,
`step-import/src/recognize_curve.rs:874` and `recognize.rs:756` (both sit
inside the assertion that consumes them), `editor-core/src/expr.rs:320`
(#646's own `const _: () = assert!`).

### 3c. THE EXTRACTION — and a guard landed for it

`step-import/src/chart.rs:9`. *"FreeCAD 1.1.2 never writes
`FACE_OUTER_BOUND` at all (0 occurrences in the 13 measured files)."*

This is the interesting kind, and it is louder than #663's `Cargo.toml`
row: **the entire geometric outerness-inference lane — chart inversion,
shoelace orientation, three typed refusals — exists because of that
sentence.** Nothing computes with the number 13; what computes with the
measurement is the decision to build the lane at all, and the scope of what
it must handle. It was taken once, by hand, in M7-2, and nothing re-took it.

`step-import/src/units.rs:6` and `:24` are the **same class, same corpus**
(§4's class-not-instance rule): *"FreeCAD 1.1.2 writes
`SI_UNIT(.MILLI.,.METRE.)` on every file it emits, and its declared
uncertainty (`1.E-07`) is in those millimeters too"* — the reason the prefix
table is data rather than a millimetre special case — and *"a `.MILLI.
.RADIAN.` context is … absent from every file measured"*, which is why the
angle path refuses a prefixed SI angle instead of folding in a second scale.

**Landed as one guard over the class, not three over the instance**:
`crates/step-import/tests/freecad.rs`'s
`the_committed_freecad_corpus_still_says_what_chart_and_units_quote`.
Three things it is careful about, each of which the first version got
wrong:

* it asserts `FREECAD_FIXTURES.len() == 13` **directly**, not only that the
  fixtures directory and the array agree. A correspondence check stays
  green on the exact scenario the row exists for — commit a fourteenth
  file *and* list it, and *"the 13 measured files"* is false with nothing
  red;
* every dialect check is an **absence** test, not an existence one. The
  corpus already contains the separating case: `twobody_importexport.step`
  declares **three** unit contexts, so `contains("SI_UNIT($,.RADIAN.)")`
  would pass on a file carrying a prefixed radian context beside an
  unprefixed one — the exact thing `units` claims cannot happen. The row
  now enumerates every `SI_UNIT` and `LENGTH_MEASURE` occurrence and
  requires all of them to agree, plus at least one, so a file that dropped
  its unit context cannot pass vacuously;
* whitespace is stripped before scanning, because ISO-10303-21 permits a
  line fold anywhere outside a string literal — a regenerated fixture could
  split `FACE_OUTER_BOUND` across two lines and hide the marker from an
  absence check.

**Sibling, named:** `wild.rs:346`'s
`the_committed_corpus_still_carries_the_dialects_it_was_chosen_for` is the
same job in the same crate over the wild corpus. It asks `any(corpus
contains X)`, right for its claim (*"each gap is present in something
committed"*); this row asks all-or-none per file, because `chart`'s and
`units`' claims are universally quantified. Two shapes, one class — the
class already had a member, and each test now points at the other.

Both module headers name the new row.

### 3d. Unguardable, with the reason now at the claim site

Twelve rows. Two were already complete (#663's `ssi/march.rs:146` and
`mesh/src/lib.rs:148` — whose §Q6 pointer 50 lines down at `:199` still
named the retired `docs/REVIEW-STYLE-BRIEF.md`, fixed here). Three more
already carried their reason: `mesh/src/trimmed.rs:107` (*"the
typed-refusal backstop, not a tuning knob"*), `mesh/src/curved.rs:1196`
(*"orders of magnitude, so no calibration question arises"*), the
`step-import` wild-corpus counts (inert).

**Landed**, because the reason was missing:

* `geom-brep/src/edge_nurbs.rs:382` — *"Measured, not assumed … the cubic
  image measures 9.8e-11 m and the piecewise-linear one 1.1e-13 m"*, which
  chose `PXN_IMAGE_DEGREE = 1`. Unguardable **completely**: the cubic arm
  the comparison is against does not exist in the tree, so there is nothing
  to re-measure and nothing computes with either figure. Now says so.
* `profile/src/sugar.rs:862`'s nine-sweep table — the 300 000 corners are a
  one-shot adversarial search; the tripwire in 3b guards the *constant*, not
  the table. Now says so, in the same edit.
* `mesh/src/nurbs_cert.rs:356`'s *"worst tour face certificate 0.60·δ"* — a
  one-shot corpus observation beside three real guards, and easy to
  over-read as covered by them. Now separated out explicitly.

### 3e. Unguarded, and now recorded — one row

`crates/pncad/src/prelude.rs:3`: *"The inventory is **measured**, not chosen
by taste. It is what the eighteen tour scenes, the STEP-export corpus, and
the document-layer corpus actually import."*

Split, because the two halves have different answers and the prose hides it:

* **Sufficiency is guarded.** `pncad/tests/all.rs` authors the whole ladder
  — profile → body → booleans → validate → mesh → export — through the
  prelude *alone*, so a name this list drops that a journey still needs
  fails to compile.
* **Minimality is not.** The corpus-frequency measurement that chose the
  cut was taken once, by hand. A nineteenth scene, or a name that quietly
  stopped being corpus-wide, moves the answer with nothing going red.

That is **unguarded, not unguardable** — re-running the import census would
guard it. Recorded at the claim site rather than deferred silently; not
scheduled, and I am not pretending it is.

---

## 4. Where this stopped

**Reached:** all of `crates/*/src`, at comment-block granularity, under the
new pattern. That is the population #651 named and #663 covered a tenth of.

**Not reached:** everything outside it, enumerated in **#681** with the
expected-yield ordering, the runnable script, the note that the instrument
does **not** transfer verbatim to any of those surfaces (none of them uses
`//`), the holes above carried across, and a done condition. #663's one
confirmed unguarded extraction lived in `Cargo.toml`, outside the searched
population, and was found by accident — so this remainder is not a
low-yield tail.

**#667 can close** on this: its stated first unit (fix the pattern) is done
with its blind spots stated, its density list is worked through, and its one
carried-forward open row (`DEFAULT_K`) is resolved — to a scheduled
register, by reading `ci.yml` rather than by adding anything. The remainder
is #681, a numbered issue; #651 stays open.

Refs #613, #614, #646, #651, #663, #666, #681.

## Home

Code quality: S-QA's exit walk names Track W as the carrier of the tests-leg measured-claim sweep **with issue 651 as class home**, and Track W is one of this program's blocks.
