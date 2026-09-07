---
id: tess-budget-doc-finding-block-stale
kind: issue
title: docs/TESS-BUDGET.md's headline finding block is a hand-transcribed census of a sweep it no longer describes
status: closed
branch: meter/budget-doc-finding-block
opened: 2026-09-03
closed: 2026-09-07
refs: [report-header-column-phrases-unqualified]
track: K
---


## What

`docs/TESS-BUDGET.md`'s *"The finding"* block (the fenced block under that heading, `:244-255` at `a4eb03a`) is a census of the tour sweep, hand-transcribed into prose, and every figure in it disagrees with the committed baseline it is supposed to describe (`docs/tess-budget-data/tess-budget-baseline.csv`, cut `6e434ffdebe0` 2026-09-01, commit `a4eb03a`). Re-derived from that file, columns `face`/`triangles` and the sizing block's presence:

| the doc says | the committed baseline says |
|---|---|
| 1025 faces | 1306 |
| 1,149,528 triangles | 1,416,410 |
| 64 NURBS faces (6.2% of faces) | 64 (4.9%) |
| carrying 782,104 triangles (68.0% of the mesh) | 164,710 (11.6%) |
| 390,100 grid cells used | 46,019 |
| 95,090 at the cheapest split | 94,154 (`opt_cells`) |
| 154,129 sized per knot-span cell | 44,446 (`span_opt_cells`) |

The NURBS-triangle figure is off by a factor of ~4.7 and the grid-cell figure by ~8.5, and the first is the one the document's whole argument rests on — the claim that a small number of Hessian-sized faces carry most of the mesh. On the committed baseline they carry about an eighth of it.

**Not edited here** because `docs/` is outside this lane's fence, and because the right fix is a judgement call this lane cannot make: whether the block is stale against a re-cut (in which case re-derive it) or whether it describes a *different* sweep — a full deviation sweep at an earlier head, before per-knot-span sizing landed — in which case it needs to say which sweep and when, or stop being a number.

## Finding

**One mechanism, and it is the one Track K's `C15`/`D201` correction just closed one instrument over.** A census over a committed artefact, transcribed into prose, drifts at the rate the artefact is re-cut and nothing catches it. `tools/tess-lint`'s module docs carried the same shape (the face-identity census) and it had been wrong since 2026-09-01 with nothing to fire; the cure applied there was to give the census one executable home (`tools/tess-lint/tests/baseline_census.rs`) that re-derives it from the committed file on every `cargo test`, and to leave pointers everywhere else. The same cure is available here: the baseline is committed, the derivation is four columns of arithmetic, and `tools/tess-lint`'s own report already computes every one of these figures — `main.rs`'s report header prints faces, NURBS faces, their percentage and their triangle share. The document could cite that output rather than restate it.

**The document's own defence does not cover these figures.** `tools/tess-meter`'s module docs say `docs/TESS-BUDGET.md`'s deviation columns come from a `--deviation` run nothing re-takes and instruct a reader to *"read its sizing columns as live and its deviation columns as dated"*. Every figure in the table above is a SIZING figure — faces, triangles, grid cells, `opt_cells`, `span_opt_cells` — so it is the half the document is asserted to keep live that has drifted, and by factors of 4.7 and 8.5. `tools/tess-lint`'s own report header already prints the first four of them from the committed baseline, in one command, and disagrees with the document line for line.

**Confidence:** sure for the arithmetic (re-derived twice, once in Python over the raw CSV and once through `tess_lint::parse`); unsure about which sweep the doc's numbers came from, which is why this is an issue and not a diff.

**Raised by:** the Track K census re-derivation lane, as the sweep for *"other hand-transcribed counts over the committed baseline"*.

## Was

unrowed — found by a sweep, not placed by a track.

## Claimed by METER (2026-09-06)

Moved from `work/code-quality/` to `work/meter/` in the tracker-wide cut of 2026-09-06 (Ev's direction, in-chat), which read every open `work/issues/` file and every open code-quality row against every live program's `paths` and opened four programs for the ground none covered. Id, `track:` letter and body unchanged. Track K; `docs/TESS-BUDGET.md` is METER's territory.


## Orchestrator note (2026-09-07) — RETRACTED 2026-09-07, it was wrong

**The note below is kept as the record and its conclusion is false.**
It read three of the block's cell lines against today's columns of the
same NAME, concluded the block had drifted a second time, and told the
lane the judgement was settled and the only open question was the form
of the fix. The lane checked the premise instead of building on it and
falsified it; `## Closed` below carries the real finding.

**What the note missed, in one line:** the block has FOUR cell lines
and the note (like the item's own table above it) compared only three.
The fourth is `44,457 with both`, and today's `span_opt_cells` is
**44,446** — a difference of 0.02%. `95,090 at the cheapest split`
against today's `opt_cells` **94,154** is 1.0%. Those two are the
schedule-INDEPENDENT optima, and they have not moved. The columns that
moved several-fold are the schedule-DEPENDENT ones, which is what a
change of SIZING RULE looks like — a re-cut driven by corpus growth or
by a certificate change moves all four together. (That is as far as
the tell reaches; it does not on its own separate the fix landing from
a later unrecorded schedule change, since a re-cut taken after one and
the change landing are the same event. `## Closed` says what does.)

**The note's reasoning was also unsound where it was most confident.**
"Stale against two successive re-cuts in the same direction" does not
discriminate between the two hypotheses at all: under either one the
gap grows monotonically as the corpus grows. It read like evidence and
was not. What discriminates is WHICH columns moved, and the note never
asked.

The note is left standing rather than deleted because the item's own
`## What` section above makes the same mis-mapping, and a reader who
finds only the corrected text will not understand why two successive
readers reached the wrong answer. Its retraction is the point.


### The retracted note, as written

Re-derived at this program's opening, against the baseline as committed
today — cut `aba2625f8f84`, 2026-09-04, a re-cut later than the `a4eb03a`
this item read:

| the doc says | item said (`a4eb03a`) | baseline now (`aba2625f8f84`) |
|---|---|---|
| 1025 faces | 1306 | **1353** |
| 1,149,528 triangles | 1,416,410 | **1,552,822** |
| 64 NURBS faces (6.2%) | 64 (4.9%) | 64 (**4.7%**) |
| carrying 782,104 triangles (68.0%) | 164,710 (11.6%) | 164,710 (**10.6%**) |

`grid_cells` 46019, `opt_cells` 94154 and `span_opt_cells` 44446 are
unmoved. **This is the item's own mechanism arriving on schedule**: a
census transcribed into prose drifted again inside four days, and the
transcription that drifted this time is the one in this file. It settles
the judgement the item declined to make — the block is stale against
re-cuts, not a description of a different sweep, because it is now
stale against two of them in the same direction — and it settles the
form of the fix: **cite, never restate.** A re-derived table committed
here would be the third copy to go stale.

**One reconciliation the lane owes before either side can cite the
other.** `tools/tess-lint/src/main.rs`'s report header prints
`grid_cells` / `patch_cells` / `span_opt_cells` as "used /
whole-patch counterfactual / cheapest split per cell", while the doc
block labels `opt_cells` "at the cheapest split" and `span_opt_cells`
"sized per knot-span cell". The two documents disagree about which
column "the cheapest split" names, so a citation written without
noticing would point at the wrong number while looking right.

**Confidence:** sure for the arithmetic (re-derived over the raw CSV);
sure that the cut line reads `aba2625f8f84 2026-09-04`.

## Closed (2026-09-07, METER unit 2)

**The item's premise is wrong and that is the unit's finding.** The
block is not stale against a re-cut and does not describe a different
*sweep* in the sense the item meant: it is the #547 PRE-FIX
measurement, and every figure in it is correct for the tree it was
taken from. What this item's table, and the orchestrator's
re-derivation of it at the program's opening, both did was compare a
pre-TESS-SPAN vocabulary against post-TESS-SPLIT columns of the same
name.

Re-derived over `docs/tess-budget-data/tess-budget-baseline.csv` at
cut `aba2625f8f84`, through `tess_lint::parse` and `SceneTotals` (the
gate's own fold), and independently in Python over the raw CSV:

| the block says | today | why it moved |
|---|---|---|
| 1025 faces | 1353 | the tour grew; all of the growth is analytic (`nurbs` rows are 64 in both) |
| 1,149,528 triangles | 1,552,822 | the tour grew and the NURBS meshes shrank |
| 64 NURBS faces (6.2%) carrying 782,104 (68.0%) | 64 (4.7%) carrying 164,710 (10.6%) | 4.75x fewer triangles on the same 64 faces — the two fixes landing |
| 390,100 `grid cells used` | `patch_cells` **110,811** | the block's numerator is `uniform_cells`, the retired whole-patch grid, which is today's `patch_cells` column and no longer the column called `grid_cells` (46,019) |
| 95,090 `at the cheapest split` | `opt_cells` **94,154** | −1.0%: a schedule-INDEPENDENT optimum over the same 64 faces |
| 154,129 `sized per knot-span cell` | the removed `span_cells` | the item mapped this onto `span_opt_cells` (44,446). It is the column that was deleted for being identically `grid_cells`; TESS-SPAN realised it at 163,182 |
| 44,457 `with both` | `span_opt_cells` **44,446** | −0.02%. This is the line `span_opt_cells` actually answers, and it was sitting one row below the one the item compared |

**Two optima within 1% while the two shipped-schedule columns moved
~3.4x** — 390,100 → 110,811 (3.52x) on the whole-patch counterfactual
and 154,129 → 46,019 (3.35x) on the grid the lane builds. (The 8.5x
that reads off these figures is 390,100 / 46,019, the pre-fix
whole-patch numerator over today's per-cell denominator: the
mis-pairing itself, not a factor. "3–8x" over the pair, written here
and in the census file until the fix pass, restated it.)

**What that separates, exactly.** A change of SIZING RULE from
everything else — and not a fix from a drift. Corpus growth and
certificate changes move the optima too, so two columns still at 1%
says the 64 faces and their bounds are still the block's and that
whatever moved was a schedule; it does not say WHICH schedule change,
because a re-cut taken after one and the change landing are the same
event. What settles that is the dated record:
`docs/MODEL-AB-LOG.md:1578` (TESS-SPLIT), written at the time —
*"tour NURBS cells 163,182 → 46,102, leaf_a 84,524 → 43,798 tris atop
TESS-SPAN's 261,780 → 84,524"* — where 261,780 is exactly the
`lily/lily_leaf_a` figure in the block's scene table and 46,102 is
what the committed file carried from TESS-SPLIT's cut onward.

**The document already said HALF of it, 155 lines above the block**
(*"It is NOT the cut this document's measurement was taken from and
its numbers are not the ones quoted below"*). That clause is the
corroboration and it is sound. The rest of the same sentence is not:
it juxtaposed the block's *"1,025 faces and 390,100 grid cells"*
against *"the committed file's own `grid_cells` sum … 46,102"* — the
exact mis-pairing this item blames for three readers' error, written
by the document in its own voice, and the pairing a reader would then
read as an 8.5x drift. Its 46,102 was correctly dated ("at the
TESS-SPLIT re-cut") and was correct there; what was false beside it
was *"and grows with the tour like the row count"* — `grid_cells` does
not grow with the tour, since the growth is analytic, and its one move
since was CERT-10's −83 to 46,019 at `a4eb03ae`. So: half a
corroboration, and the other half one more instance of the defect,
which is why the sentence was cut rather than kept.

**The mechanism the item named is real; it is one level up from where
the item put it.** What drifted is not the numbers but the COLUMN
NAMES they are read against: `grid_cells` was redefined by TESS-SPAN,
`span_cells` was removed, and "the cheapest split" names `opt_cells`
in one place and `span_opt_cells` in another. Three readers in a row
re-derived the same disagreement and each read it as staleness.

**The fix, therefore, is cite-and-label rather than cite-or-restate.**
`docs/TESS-BUDGET.md` gains: a `## The census today` section pointing
at the one command and at the census's executable home; a heading and
preamble on the block naming it the pre-fix sweep, with the tell that
distinguishes a fix from a drift; a four-row table joining the block's
retired phrases to today's columns; the "cheapest split" collision
stated in both directions; and no live count over the committed file
anywhere in the document (the 1,075 / 46,102 / 163,182 / 1,025 /
390,100 restatements in the committed-baseline paragraph are gone).
The pre-fix literals stay literals, and the reason they may is written
beside them: no artefact holds them and nothing can re-derive them.

`tools/tess-lint/tests/baseline_sizing_census.rs` is the executable
home, on the `baseline_census.rs` precedent and beside it. What it
ends up asserting is **triangles, NURBS triangles, the four cell sums
and the two factors**, all folded through `SceneTotals` so the census
counts what the gate counts. It asserts no row count, no sized-row
count and neither triangle-share percentage: those are the
neighbour's, and the dedupe that removed them from this file
(`1e91d7c98`) is the one-home rule applied to the pair. A re-cut that
moves any of those figures fails it and names what moved; a re-cut
that moves only the descriptive per-face columns fails NEITHER census,
and the runbook now says which is which rather than claiming both
tests catch everything.

**Residue:** the report header names its cell columns by phrase with
no column name attached, which is the mis-read's actual source and is
not fixed here — `work/meter/report-header-column-phrases-unqualified.md`.

**Outside this program's fence, reported not filed:**
`crates/mesh/src/nurbs_cert.rs:632-634` justifies `SAFE_ASPECT = 5.0`
with a *"measured margin (worst tour face certificate 0.60·δ)"*. That
is a hand-transcribed reading of this baseline's `worst_cert` column;
re-derived over the committed file, the tour's worst is **0.125·δ**,
stale by ~5x in the safe direction. `crates/mesh/*` is S-MESH's by
METER's `keep_out`.
