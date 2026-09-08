# METER exit walk — criteria vs evidence

**STATUS: RULED — every question §8 put to Ev is answered; merging this PR
ratifies the walk.** Per the convention Ev confirmed for S-MATE (2026-09-04:
merging a PROPOSED walk is the ratification), this walk rides an `[ev]` PR.

**The ratification record.** Ev ruled on 2026-09-08 in the #2212 thread, in
three comments of his own: the request to elaborate the six questions
([`5588674349`](https://github.com/evgunter/cad/pull/2212#issuecomment-5588674349)),
the six answers
([`5588809742`](https://github.com/evgunter/cad/pull/2212#issuecomment-5588809742)
— *"1. open / 2. giving them to the successor sounds good / 3. huh i thought
track X had closed / 4. delete / 5. i would like to see the diff / 6. this
doesn't look like a question?"*), and the confirmation of the two corrections
his 3 and 6 forced
([`5588846576`](https://github.com/evgunter/cad/pull/2212#issuecomment-5588846576)
— *"the new 3, instr, and your plan all sound good!"*). §8 is now the record of
those rulings rather than a list of questions. **Three of the five came back
against the option the walk recommended**, and one of the three — `C15` — is a
reversal of a ruling §5 had already made, on a claim about the tree that turned
out to be false (§5's first row carries the whole correction). Each ruling is
folded into the section it governs as well, so no section of this document still
describes a settled question as open.

**The sequence he approved**, and the reason its order is load-bearing: **this
PR merges, which ratifies the walk → the
`docs/prompts/implementer-discipline.md` diff rides a second `[ev]` PR, because
he asked to see it → the successor PR opens `instr`, moves the twenty rows,
deletes `.cert1-notes/pr-body.md` and re-homes the S-TCOST row → the sweep
deletes `work/meter/` and this document and records both in
`docs/DOC-LEDGER.md` at the sweep SHA.** The discipline PR precedes the sweep
because §6's text survives the sweep only as a ledger SHA.

**The residue is NOT re-homed in this PR**, unlike S-CERT's and SEAT's walks:
opening a program is not a drafting lane's call, and this PR neither opens
`instr` nor moves a row nor deletes a file. It rules and records; the moves are
step 3's. Until step 3 lands, `work/meter/log.md`'s tail is the program's live
status.

METER = code-quality Track K's `tools/*` half (`work/meter/{program,plan,log}.md`;
opened 2026-09-06 in the tracker-wide cut, `docs/WORK-TRACKS-2026-09.md`
addendum 2; A/B band 3200–3299, claimed for bookkeeping — the program ran no
A/B protocol by Ev's posture ruling of 2026-09-07). Criteria are quoted
**verbatim** from the plan's *Exit shape* paragraph, one clause per row;
dispositions per the M5–M8 / ASM / S-QA / S-CERT convention: MET /
MET-WITH-RECORDED-HONESTY / CARRIED (named owner). Walked against `main`
`5ce54b35b` by `git show`, `git ls-tree`, `python3 scripts/work.py status`
and reads of the committed tree — not from memory, and not from the
orchestrator's counts, three of which this walk falsifies. **Every count was
re-derived a second time** when Ev's rulings were folded in, against this
branch's merge of `origin/main` `557897699` (branch SHA `f7385af7e`); the
sweeps are quoted where they are used and none of the figures moved.

The shape is `docs/S-CERT-EXIT-WALK.md`'s and `docs/GATES-EXIT-WALK.md`'s —
verbatim criteria table, then the evidence the criteria do not reach, then the
residue and its homes, then the rulings Ev made on it. The one deviation is
that the sections are numbered, because §3's census and §4's ruling are
cross-referenced from three places.

## 1. The walk

The plan's exit shape is one sentence: *"The twelve land (with `D201`'s answer
either built or ratified as not-now), Track K's `tools/*` half is empty; the
walk convention applies."*

| # | Criterion (verbatim) | Disposition | Evidence / honesty note |
|---|---|---|---|
| 1 | "The twelve land (with `D201`'s answer either built or ratified as not-now)" | MET-WITH-RECORDED-HONESTY | **Thirteen unit PRs landed, not twelve** — §2's table, one row per merged unit branch, numbered 0–12. `D201` was RULED (Ev, `[ev]` PR #2109, arm A) and then BUILT, not deferred: the budget CSV carries a `name` column at field index 2 and 286 of its 1353 rows are named (`docs/tess-budget-data/tess-budget-baseline.csv`, cut `3f55f361b22e`). **The honesty is the count itself.** `plan.md`'s own *Where the units ended* says "All twelve are merged"; `log.md`'s unit-10 entry says "Eleven units merged (0-12 less the two that became other things)". Both are wrong in the same direction and neither was taken from the record. The record is thirteen merges of thirteen distinct `meter/<unit>` branches; the command is in §3. |
| 2 | "Track K's `tools/*` half is empty" | **CARRIED — to `instr`, the successor Ev ruled open on 2026-09-08** | **Not met, and not close: 22 items are open, 19 of them on `tools/*`.** The slate opened at eleven inherited rows and stands at 37 item files, 15 closed and 22 open (§3, with the sweep that produced it and what that sweep cannot reach). It grew because every unit's mandatory sweep and every reviewer's class finding landed a real defect with a citation — 26 of the 37 were filed during the program's two working days — not because the fence drifted. `work/README.md` is explicit about this case: *"a dozen items on one territory are a successor's opening slate, and the closing program opens it"*, and `work/issues/` is a last resort where *"an unsorted pile of related items there is what the sweep exists to prevent."* Nineteen coherent rows on one territory are that pile twice over. §4 is the successor, ruled open; it opens at **twenty** rows — these nineteen plus `C15`, which §5 routes to it rather than to code-quality. |
| 3 | "the walk convention applies" | MET | This document, on an `[ev]` PR, per `CLAUDE.md` and `work/README.md`. The sweep it licenses deletes `work/meter/` and this file and records both in `docs/DOC-LEDGER.md` at the sweep SHA. |

## 2. The units — the done-state per unit

Every unit took one style review by default (`docs/prompts/reviewer-style-lane.md`);
`D206` and `k-report-baseline-fold-cert1-roster` took the program's two full
falsification reviews, as the posture assigned. No A/B rows were written; the
band 3200–3299 stays claimed for bookkeeping.

| unit | items closed | PR | state |
|---|---|---|---|
| 0 | `tess-lint-face-ordinal-join` | #2111 | An ungated re-key stays a **note**, argued from the disanalogy with rule 5 and written at the site with its cost and its cure. #738's `diefillet` permutation no longer reproduces — the lane ran the sweep instead of inheriting the claim. Third instance of the transcribed-census class found and moved to its one executable home |
| 1 | `D213`, `D214` | #2125 | The sizing block refused at `parse` in both directions (`SIZED_CHART_TAGS`), which makes `Row::is_sized` a function of `chart` and RETIRES `IDENTITY_COLUMNS`' block-presence entry rather than re-justifying it. Ten mutations over nine guards; refusal ORDER pinned. The cross-root roster pin was written rather than filed (`tools/tess-meter`, per-tag biconditional) |
| 2 | `tess-budget-doc-finding-block-stale` | #2114 | **The block was never stale.** The fix pass walked every committed blob and found the passage exact for the corpus it describes (`benchlayout/benchlayout` at 30 rows at `48559d61`, re-swept to 18 at `6d957702`); it is now labelled a frozen record. Three levels of correction on one item — orchestrator, then style review, then fix pass — each cured by reading the artefact's history rather than its current state |
| 3 | `D203` | #2132 | The cross-column rule given one home. Briefed with two instances; the sweep found seven. `CC4` was FALSE as first written and the style review broke it by construction; it is inverted, not conditionalised. The page moved to `tools/README.md` (a directory page for two crates), both crates `include_str!` it so a moved page stops compilation, and each asserts that every clause it cites is a heading and that no unseen clause exists |
| 4 | `cut-prefix-three-unpinned-spellings` | #2151 | `tools/tess-lint/tests/cut_line_pin.rs` holds the script's three prefix spellings to `CUT_PREFIX`, each located separately. The two languages' constraints were NOT the same — this crate admitted three spellings `scripts/tess_budget_cut.sh` cannot emit, and the asymmetry cost in the bad direction (a re-stamp over an existing cut). Truth table runs both readings side by side, the script's own regex extracted from its text. Two residues filed on `work/ciw/`, where the owner will see them |
| 5 | `k-lint-predicate-roster-unpinned` | #2115 | **The unit the style lane earned its place on**: the reviewer broke the first pin green three times (substring match, a parameter-borne `target_len`, a respelled mint site). All three closed, plus the converse check. `props_quad_last_round` excused by NAME rather than rostered — rostering it would be a distribution ruling with no distribution |
| 6 | `D206`, `D201` | #2167 | One re-cut, three separable commits, one sweep at `3f55f361b22e`. `SPLIT_SCAN_SAMPLES = 379` re-derived rather than inherited (5.0075% at 378, 4.9939% at 379). **The fix pass corrected what the unit CLAIMED, not what it did**: the licensing sentence was false — a `ceil`'d miss costs a whole division, exhibited at 7.6923% against a 4.9939% envelope — so `D206`'s premise survives its own closure and is re-filed. Two cannot-fail assertions the unit had introduced were deleted before merge |
| 7 | `k-report-baseline-fold-cert1-roster` | #2140 | Full falsification review. The CERT-1 roster fold was READ and **no CSV was re-cut**, which is the unit's judgement: the three witnesses were pointwise identical to M7 and rule 1 cuts an era on a moved DISTRIBUTION. What the review broke was WHEN the measurement was taken — at the branch's merge base, with `props/curved.rs` moved 129+/74− in between; re-swept at `c39a904e`, roster 279 → 281, verdict unchanged |
| 8 | (`C15` re-planned; left OPEN) | #2177 | Unit 6's column does not discharge `C15`: **0 of the 64 sized rows carry a name** and none of the 14 rows in the seven indistinguishable pairs does — coverage and defect are disjoint. What an undetected swap costs is a wrong-face attribution in a column nothing reads; the gate-invisibility now has an executable home in `tools/tess-lint/tests/baseline_census.rs` (`an_undetected_swap_costs_the_gate_nothing_on_the_committed_baseline`, `no_scene_carrying_a_sized_row_carries_a_name`), so a re-cut that makes a swap gate-visible fails and names it, and a scene becoming document-built fires and says `C15` became dischargeable. `C15` stays open by ruling |
| 9 | `report-header-column-phrases-unqualified` | #2180 | The report names its four cell totals by column with the two factors as formulas; `tools/tess-lint/tests/report_columns_pin.rs` gates the names, the order, the qualifiers and the completeness converse against the parser's own roster rather than against a spelling. Produced the program's citation rule (§6.4) |
| 10 | `fold-the-two-baseline-census-files` | #2187 | `baseline_sizing_census.rs` is gone; `tools/tess-lint/tests/baseline_census.rs` carries seven `#[test]`s behind one `include_str!`. The lane overruled item and plan on the NAME, on a pointer census (24 live mentions against 7), which is the right instinct for this program. A fold that drops a `#[test]` still goes green, so all seven were confirmed by name from `cargo test -- --list` and the reviewer mutated each one separately — seven runs, seven individual reds |
| 11 | `k-report-era-witnesses-have-no-guard` | #2158 | `tools/k-lint/tests/threshold_provenance.rs`'s `the_m7_era_still_carries_the_witnesses_the_report_names` asserts each witness's PREDICATE as well as its value, and EXHIBITS the string-comparison trap by running the selection both ways. Verified by corrupting the committed `.gz` rather than by editing a literal — the strong form: it catches corruption of the artefact |
| 12 | `tess-lint-twinned-csv-fixture` | #2179 | The fixture has one home, `tools/tess-lint/src/tests/csv_fixture.rs`, mounted into both roots with `#[path]`, so a value drift between the two sides can no longer be WRITTEN. **The unit overturned the finding that promoted it**: blanking the token was not silent (clippy caught it, by accident of reachability), and the failure mode nobody had measured — drifting the constant's VALUE on one side — was invisible to tests and clippy on both |

Two `[ev]` PRs rode beside the units: #2109 (`D201`'s design fork, ruled arm A)
and #2147 (`tools/README.md`'s ratification, ruled BROAD). Three orchestrator
state-sync PRs: #2110, #2128, #2164.

`C15` is the one planned unit whose ITEM did not close, and it did not close by
ruling rather than by omission (§5).

## 3. The slate — the census, re-derived

**Numbers taken from `main` `b28b13787`, not from `plan.md` or `log.md`, and
re-run unchanged against this branch's merge of `origin/main` `557897699`.** The
sweep:

```
for f in work/meter/*.md; do          # skip log.md plan.md program.md
  sed -n '1,25p' "$f" | grep -m1 '^status:'
done
python3 scripts/work.py status --program meter
```

**What it cannot reach**: it reads the `status:` line of files that exist in
`work/meter/` on `main` at that SHA. It cannot see an item still on an unmerged
branch, an item METER filed onto ANOTHER program's slate (two on `work/ciw/`
from unit 4 — `cut-regex-unanchored-admits-a-line-the-lint-refuses` and
`cut-script-header-claims-no-cross-language-gate-exists` — plus the
dead-code-quality-pointer row unit 8's sweep produced, which sits on
`work/meta/` on the unmerged `meter/orchestrator` branch; none of the three is
counted here), a residue disclosed in a merged PR body and never given a
file, or a defect nobody has found. It counts files, not work.

| | count |
|---|---|
| item files in `work/meter/` | **37** (plus `program.md`, `plan.md`, `log.md`) |
| inherited at the 2026-09-06 opening (`opened:` before that date) | **11** — exactly the eleven `work/code-quality/` rows the opening claimed |
| filed during the program (`opened:` 2026-09-07 or 2026-09-08) | **26** |
| closed | **15** (10 of the 11 inherited; 5 of the 26 filed) |
| **open** | **22** (1 inherited — `C15`; 21 filed) |

The second run of that sweep, at branch SHA `f7385af7e`, returns the same 40
files in `work/meter/` (37 items plus the three program files) and the same
**15 closed and 22 open**. It is the same slate item for item, not just the same
totals: the two runs' `<id> status:` listings were diffed
(`git ls-tree -r --name-only b28b13787 work/meter/` and `git show
b28b13787:<f>` against the working tree, one line per item, `diff` empty), and
50 commits of `origin/main` landed between the two SHAs without touching this
directory.

Three published figures are wrong and are corrected here, because a walk that
repeats them makes them harder to correct later:

- **`plan.md`: "the slate stands at 23 open items"** — 23 was true at unit 10's
  merge (`9678901b6`); `tools-readme-is-unratified-and-owes-a-design-row`
  closed at #2147 and the tip is **22**. The plan's fourth non-successor bullet
  ("blocked on Ev (#2147) and cannot be re-homed until he rules") is stale
  against its own branch tip, which is the commit that records the ruling.
- **`log.md`: "40 files, 24 open"** at unit 10's merge — 40 files is right
  (37 items + 3), 23 were open at that SHA.
- **`plan.md` "All twelve are merged" / `log.md` "Eleven units merged"** —
  thirteen, numbered 0–12:
  `git log origin/main --merges --pretty='%h|%s' | grep 'from evgunter/meter/'`,
  less the three `meter/orchestrator` merges and the two `[ev]` branches
  (`meter/d201-ev-question`, `meter/tools-readme-ratification`), leaves
  thirteen unit branches. That command cannot see a unit that landed under a
  non-`meter/` prefix, and cannot distinguish a unit PR from a docs-only PR on
  a `meter/` branch — both were checked by hand against §2.

### The 22 open, grouped by the territory that fixes them

**Instruments — 19.** Every one is closed by an edit inside METER's own `paths`
(`tools/*`, `docs/K-REPORT.md`, `docs/TESS-BUDGET.md`, and the two data
directories). Grouped by which instrument, then by whether the edit lands in
code or in the governing document:

- **`k-lint` — 6.** `k-lint-csv-header-unpinned-against-five-producers`,
  `k-lint-eps-coupled-criterion-unwritten`,
  `k-lint-gate-described-as-diffing-the-committed-baselines`,
  `k-lint-last-round-is-eps-coupled-but-unrostered`,
  `k-lint-roster-wants-a-kernel-side-vocabulary`,
  `k-lint-rule-1-prose-assumes-every-in-band-site-refuses`.
- **`tess-lint` / `tess-meter` code — 8.**
  `baseline-census-partition-assert-cannot-fail`,
  `cut-line-commit-names-no-baseline-change`,
  `gate-findings-name-no-columns-and-recoverable-has-two-aggregations`,
  `report-constraint-activity-line-names-no-columns`,
  `tess-lint-growth-margin-unprotected-from-ceil-quantisation`,
  `tess-lint-ungated-columns-fold-silently`,
  `tess-lint-zero-certificate-two-meanings`,
  `tess-meter-sampled-retune-figure-unreproducible`.
- **`docs/TESS-BUDGET.md` — 3.** `baseline-sizing-census-second-copy`,
  `tess-budget-doc-identity-column-list`, `tess-budget-doc-note-finding-rule`.
- **Straddling document and crate — 2.** `baseline-sizing-census-pointers-stale`
  (seven mentions across six files, one of them in `tools/tess-lint/tests/`),
  `tess-lint-recourse-quote-half-pinned` (the document is the unpinned half;
  the pin lands in `tools/`).

Two of the nineteen are blocked on a change outside the fence and are named as
such rather than re-homed, because the row-shaping half is the instrument's:
`tess-lint-zero-certificate-two-meanings` (the producer separating "certified
nothing" from "certified zero" is `crates/mesh/src/budget.rs`, S-MESH's) and
`k-lint-roster-wants-a-kernel-side-vocabulary` (a `const` slice from
`geom_core::k_stats` would let the roster pin shrink to an import; PROPS').
`k-lint-eps-coupled-criterion-unwritten` composes with the second — one PROPS
unit could land both. **Ev ruled on 2026-09-08 that all three stay on the
successor's slate** (*"giving them to the successor sounds good"*). The
rejected reading re-homed the two to S-MESH and PROPS and would have left the
successor at seventeen instrument rows — sixteen if the third had gone with
them. The code state is identical under either answer; what the ruling settles
is who is looking at them — here they sit with an orchestrator who understands
the instrument half and are blocked on a fence no lane here may cross, there
they are unblocked but land with orchestrators who did not write them.

**Not the instruments' — 3.** §5. One of the three, `C15`, is the successor's
anyway by Ev's ruling of 2026-09-08, so **the successor's opening slate is
twenty**: these nineteen instrument rows plus `C15`. **Twenty is the slate
count wherever this document names one** (§1, §4, §8); every *nineteen* here
counts something else — the rows whose fix lands inside METER's own `paths`,
which `C15`'s does not.

The orchestrator's brief said "roughly nineteen of the twenty-three" with four
non-successor items. The nineteen is right; both terms of the arithmetic are
not — 22 open and three of them not instrument rows, because the fourth closed
the same day. And after Ev's ruling only **two** of those three are not the
successor's, which is a third correction to that brief's arithmetic and not one
this walk could have made before he ruled.

## 4. The central ruling — `instr`, the successor program for the instruments

`work/README.md`: *"Residue is re-homed before the sweep, never left behind in
the closed directory: to a live program whose charter it fits, or to a new
program opened for it when the residue coheres into a track of its own (a dozen
items on one territory are a successor's opening slate, and the closing program
opens it). `work/issues/` is the last resort, for residue that genuinely coheres
with no live or new track — an unsorted pile of related items there is what the
sweep exists to prevent."*

No live program's `paths` covers `tools/*` — checked against every
`work/*/program.md` header on `main`, and CIW's and GATES' `keep_out` clauses
both say in as many words that `tools/*` is not theirs. So the two homes the
page offers are a new program or `work/issues/`, and nineteen rows on one
territory is precisely the pile the second sentence forbids. **The walk's
ruling was to open a successor, and Ev ruled it open on 2026-09-08** (*"1.
open"*), with the name `instr` and the plan below confirmed in the same thread
(*"the new 3, instr, and your plan all sound good!"*). Opening it is step 3 of
the sequence in the header, not this PR's act.

**The sibling case is the argument's control.** GATES opened the same day for
the same track's other half and closes it the same week;
`docs/GATES-EXIT-WALK.md` (on `main` at `5ce54b35b`) returns
`scripts/gates/*` to code-quality Track K *"where GATES claimed it from"* —
its residue table moves TWO rows there and records three already homed on other
programs' slates, with nothing to `work/issues/`. That is the right answer THERE
and the wrong one here, and the difference is only the count: two rows handed
back to a board that still holds a track of its own is a re-home; nineteen
handed back is re-opening a track code-quality closed on 2026-09-06 by giving it
away, and putting the board back in the position the cut was made to end.
`work/README.md` draws exactly this line at "a dozen". Two facts about that
board, both from the tree and both sharpening the argument: `work/code-quality/`
holds **112 item files** (`ls work/code-quality/*.md` less `program.md`,
`plan.md` and `log.md` — 115 less 3), and it has **no `paths:` field at all**,
carrying `blocks:` instead, because it is the waiting room where a structural
finding waits until a program claims it. Nineteen rows with a live, coherent
territory joining that pile would be waiting for a claimant that, by
construction, is the successor. (The count of the track code-quality still holds
is smaller than this walk first implied; §5 carries that correction, and it is
the reason `C15` does not go there either.)

The header the successor PR writes, field set matching `work/meter/program.md`
and `work/gates/program.md` exactly (`scripts/work.py`'s `SCHEMA` allows `area`,
`prefix`, `tag`, `ab_band`, `paths`, `keep_out`, `blocks` on a program; no
`blocks`, as METER carried none — new Track K row ids still come from
code-quality's block ledger, which stays there after a row leaves):

```yaml
---
id: instr
kind: program
title: INSTR — the instrument crates and the documents they feed
status: open
opened: <the ratification date>
area: infra
prefix: instr/
tag: (INSTR orchestrator)
ab_band: 3300-3399
paths: [tools/*, docs/K-REPORT.md, docs/TESS-BUDGET.md, docs/tess-budget-data/*, docs/k-report-data/*]
keep_out: [scripts/gates/* is GATES' — Track K was claimed whole by two programs and this one inherits the tools/* half only, scripts/tess_budget_cut.sh and tess_budget_sweep.sh and k_probe_sweep.sh are CIW's — the cut-line pin reads them as text and edits nothing and the two cut-script residues are already filed on work/ciw/, crates/mesh/* is S-MESH's — the worst_cert = 0 separation is a producer-side column and this program's row is blocked on it rather than owning it, crates/geom-brep/src/props/* and crates/geom-core/src/k_stats.rs are PROPS' — the k-lint roster reads those minted names and edits nothing there and the kernel-side vocabulary that would retire its hand-rolled parser is a PROPS unit, crates/test-utils/* is S-TCOST's — the Shared ledger row's audit is S-TCOST's and not this program's, demos/* is not this program's — demos/tour is in no program's paths at all (docs/WORK-TRACKS-2026-09.md:517 gives SHELL the scenes by courtesy of Track X) and C15 sits on this slate because the census that detects it is here rather than because the scene is; no lane here edits a scene, a re-cut of the committed docs/tess-budget-data/tess-budget-baseline.csv is a PROPS coordination and never a lane's call, tools/README.md's CC1-CC5 are ratified (Ev 2026-09-08) so a change to a clause is an [ev] design conversation, memories/tessellation-budget.md and perf-measurement-lane.md govern what an instrument may claim — nothing gates on a millisecond and no mesh is justified by its own size]
---
```

Notes on the fields, so ratification is not asked to take any of them on trust:

- **`paths` is METER's list unchanged.** Nineteen of the twenty rows land
  inside it, and every glob matches a tracked path today (`tools/*` → three
  crates and `tools/README.md`; `docs/tess-budget-data/*` → the baseline CSV;
  `docs/k-report-data/*` → the committed `m7-eps-*.csv.gz` eras).
  `scripts/work.py lint` enforces that. **The twentieth is `C15`, and its fix
  is deliberately outside**: what closes it is a `demos/tour` change, which is
  in no program's `paths` (§5). A row may sit on a slate whose fence it cannot
  cross — the two blocked instrument rows in §3 do the same — and the
  `keep_out` above says so at itself rather than leaving it to be inferred.
- **`ab_band` 3300–3399**, the next free band, put to Ev with the name and
  approved with it (*"instr, and your plan all sound good"*; he did not repeat
  the numbers, and this is the one field of the header taken from an approval
  of the whole rather than from words of his own).
  `docs/MODEL-AB-LOG.md`'s banding entry says *"with 3300+ unallocated"* and
  that the orchestrator opening the next program records the claim there **in
  the same commit that opens the program**. That commit is the successor PR's,
  not this walk's. Whether `instr` runs any A/B protocol is Ev's to set; METER's
  posture (none; band for bookkeeping) is the obvious inheritance.
- **The name is `instr`** — proposed as a placeholder Ev was invited to
  overrule, and kept because he approved it by name. It reads as "the
  instruments", which is what the charter is about; `meter` could not be reused,
  because a program keeps its directory only while it is open and that id is
  about to be a ledger row.
- **Charter, one paragraph**, to sit under the header: *The instruments MEASURE
  and REPORT; they never gate on a millisecond and never justify a mesh by its
  own size. METER fixed where an instrument's claim and its arithmetic had
  parted; what it left is the layer beneath — where an instrument's claim and
  its own EXECUTION have parted. Every row is a check that does not check, a
  figure with no home that can red, or a sentence that describes a tree that
  has moved — save one, `C15`, which is here because the census that would
  detect it is here, and which waits on a corpus change no lane here makes. The
  standing rule stays: a census has one executable home and every other site
  points at it.*

The twenty are the opening slate. Nineteen arrive already sorted into the
three lanes of §3; the twentieth, `C15`, arrives waiting on a corpus change
nobody here makes and guarded by execution meanwhile (§5). The natural first
lane is the `docs/TESS-BUDGET.md` three plus the two straddlers, because unit 10
has just deleted a file five of them point at.

## 5. The three rows that are not instrument rows

Each checked against the candidate owner's `paths:` on `main`, not guessed.
Two of the three go somewhere other than the successor; the first does not, and
that is a reversal of what this walk first wrote.

| item | ruling | check |
|---|---|---|
| `C15` (unit, open) | **the successor, `instr`** — Ev, 2026-09-08, **reversing this walk's first ruling of code-quality Track X** | **The walk had this wrong, and the correction is the walk's own, prompted by Ev's *"huh i thought track X had closed"*.** Two things it asserted are false as stated. (a) *Track X is nearly empty, not a live dispatching track*: five rows carry `track: X` and all five are in `work/code-quality/` — `D403` and `tour-scenes-lift-componentwise-not-through-map` **open**, `D79` **parked**, `D402` and `tour-suite-never-runs-wall-probes` **closed**. Two open rows and a parked one. (b) *`demos/tour` is in no program's `paths` at all* — `work/code-quality/program.md` has no `paths:` field (it carries `blocks:`; it is the waiting room), CIW's covers `demos/*.sh` and `demos/*.py` and no Rust scene code, and `docs/WORK-TRACKS-2026-09.md:517` gives SHELL *"`demos/tour` scenes **by courtesy of** Track X"* — courtesy, and SHELL's own `paths` names no `demos` path either. So "to Track X" meant a row for unowned ground, in a 112-item waiting room, under a track with two open rows. What is unchanged: the item's *Why this stays open* says what closes it — *"a scene that carries a pair becoming document-built, so that the sweep has an evaluation to take names from"* — that is still a `demos/tour` change, still one no lane on the successor may make, and `C15`'s residue is still guarded by execution in `tools/tess-lint/tests/baseline_census.rs`, which fires under either answer. The successor is simply the slate whose orchestrator knows why the row exists |
| `reader-census-shared-disposition-survives-partial-reversion` | **S-TCOST** (`work/tcost/`) | `work/tcost/program.md`'s `paths` carries `crates/test-utils/*`, and the fix is in `crates/test-utils/tests/reader_census.rs` and `crates/test-utils/src/source.rs`. It is a class, not an instance — every `Shared` row in the ledger gets the same one-substring audit, and strengthening the check means ruling on every row. That is S-TCOST's ruling to make. (The orchestrator's brief said "S-TCOST's or CIW's"; CIW's `paths` does not name `crates/test-utils` and its `keep_out` cedes test mechanism, so it is S-TCOST's) |
| `cert1-notes-pr-body-tracked-on-main` | **DELETE the file and close the row** — Ev, 2026-09-08, one word: *"delete"*. Not `work/issues/` | `.cert1-notes/pr-body.md` is in `git ls-files` on `main` and is a closed program's lane's PR draft. No live program's `paths` matches a dotfile directory at the repo root; S-CERT, whose lane wrote it, left the tracker at DOC-LEDGER sweep 7. The walk offered `work/issues/` as the genuine last-resort case and offered a one-line *"delete it"* as the faster answer; Ev took the second. The item asked for a disposition rather than for work, so the disposition closes it: **`git rm .cert1-notes/pr-body.md` in the successor PR (step 3), with the row closed citing this ruling** — not here, because this PR deletes nothing |

`tools-readme-is-unratified-and-owes-a-design-row` — the brief's fourth — is
**already closed** (#2147, `d28c127bb`). Verified: `tools/README.md`'s
`## Ratification` section reads *"Ratified (Ev, 2026-09-08.)"* and
`docs/DESIGN.md`'s companion table carries the row with the same date, with
Ev's scope ruling folded in — the page is the **reading-boundary** rule, `CC1`
and `CC5` scoped *"any reading"*, `CC2`/`CC3`/`CC4` scoped *"the per-column
admissions table"*. It needs no home.

**None of the three moves in this PR, and neither does any instrument row.**
This is a statement, not a question — Ev said so of §8's sixth item (*"this
doesn't look like a question?"*), and he was right; it has been dropped from §8
and belongs here. What this PR does is rule and record. The moves — the twenty
onto `work/instr/`, the S-TCOST re-home, the deletion of
`.cert1-notes/pr-body.md` — all happen in the successor PR, step 3 of the
sequence in the header, and the sweep after it only deletes and ledgers. The
reason is no longer that the dispositions depend on an unanswered question:
they are answered. It is that opening a program, moving a slate and deleting a
tracked file are acts, and this document's job is to say what the acts should
be.

## 6. What the program found — the findings that outlive the directory

`work/meter/log.md` dies with the directory and the DOC-LEDGER row is what
survives, so the findings are written out here in full rather than cited. None
of them is about the instruments; all four are about how a claim gets made.

### 6.1 The recurring defect class — assertions that cannot fail

The program's signature defect. **Nine instances are on the record.** The
count is a sweep of the program's own narrative
(`work/meter/log.md` at `f45a8bdcd` plus the item files on `main`) for
`cannot fail`, `could not fail`, `vacuous` and `no red`, reading each hit's
paragraph. **What it cannot reach**: an instance a lane fixed silently without
writing it into the log or an item, and the sibling family of assertions that
CAN fail but pass for the wrong reason (unit 5's three substring holes, unit 1's
disarmed roster guard) — those are counted separately below.

Seven inside `tools/`, all METER's own:

1. `noted.len() == 60` (unit 0) — dead, because 60 is `72 − 12` by
   construction. **Deleted rather than repaired**, and replaced by the twelve
   gating scenes asserted BY NAME.
2. The vacuous `if let Err(e) { assert!(!e.contains(..)) }` loop (unit 1's fix
   pass), replaced by `every_chart_tag_owes_the_sizing_block_or_refuses_it`,
   which reads both row shapes per tag.
3. `cut_line_pin.rs`'s coverage predicate over the const table
   (`TABLE.iter().any(|r| r.1) && TABLE.iter().any(|r| !r.1)`), subsumed by the
   per-row asserts (unit 4's fix pass).
4. `cut_line_pin.rs`'s containment `!(reads && !recognises)`, which read the
   table's own constants rather than the computed answers; it now runs on the
   computed pair and runs FIRST, so a real inversion reds with what it costs.
5. `baseline_census.rs`'s partition assert — `constant.len() + discriminating.len()
   == IDENTITY_COLUMNS.len()` over two filters that partition by construction.
   **Still open**, as `baseline-census-partition-assert-cannot-fail`; it is the
   successor's row.
6. and 7. `total_ceiling` and `every_token_is_one_non_empty_csv_field`, both
   introduced by unit 6 and both deleted by its own fix pass before merge
   (added at `3f55f361b`, removed at `391025b30`; neither string is anywhere
   under `tools/` today).

Two outside it: unit 9's verification chain (§6.2, instance 3), and the
pre-existing `found.len() >= 20` in `crates/test-utils/tests/reader_census.rs`,
which that file records deleting and which the partition item cites as the
class's other known member.

The class's shape, stated once: **a predicate over things that cannot vary at
runtime, standing beside the assertions that already subsume it.** Its mirror
image was named by unit 8's lane and is worth carrying with it: *an assertion
that goes red when the corpus IMPROVES is not a guard* — a pin on a movement
that would fire the day two rows came to agree.

### 6.2 Three fake greens — each a green that measured the harness

1. **Unit 5.** A mutation script failed its own assertion BEFORE writing the
   file. The suite then ran on an unmutated tree and came back green.
2. **Unit 11.** A mutation landed on a doc comment 110 lines above the
   executing literal.
3. **Unit 9.** The verification chain was
   `cargo test … | grep -E "^test result|FAILED" && …`, and **`grep` exits 0 on
   a match**, so a run containing `FAILED` satisfies the chain and the `&&`
   fires. Three failing tests passed straight through into a commit; the
   failures were real (four tests shared one `{pid}` fixture path and
   `fs::write` truncates before writing).

They have nothing in common but their shape, and in all three the green was
believed until someone asked what would have made it red. **The cure is not more
care: it is to break the thing on purpose first and watch it go red, then fix it
and watch it go green. A green you have not earned by first producing a red is a
measurement of your harness, not of the tree.**

Unit 9 also settled a reviewer question worth keeping: a finding a reviewer
cannot reproduce is still worth filing. The reviewer filed the fixture-path race
at `unsure` after 40 clean runs; the lane reproduced it at 1 in 25 on the shared
path and 0 in 25 with a path per call.

### 6.3 The day's finding

**Every error on 2026-09-08 — the four units' MAJORs and the orchestrator's own
six — was an inference from the SHAPE of a thing in place of a READING of it.**
Which function a doc list implied was gated; which spelling produced a measured
movement; which file a quoted figure lived in; what a census red means; how many
tests a drill reds. None needed cleverness to catch. Each needed one `grep`, one
control run, or one header read.

Its sub-rules, each earned by an instance:

- **A uniqueness claim is a sweep result**, so it is not writable without
  running the sweep — and *"my pattern could not match X"* is a work order to
  find X another way, never a boundary. Three reviews in one afternoon returned
  the same MAJOR in different words (units 8, 9 and 12), and three PRs disclosed
  a blind spot in their own sweep while claiming or implying it was empty.
- **A sound measurement with an invented attribution arrives wearing evidence.**
  Unit 8's `extrapolated_triangles` movement was real and survives; what was
  invented was that only one of two spellings produced it. An unmeasured claim
  is caught by asking *"did you measure this?"*; this one answers yes. The
  question that catches it is *"what did you measure, exactly, and what else
  would have produced the same number?"* It travelled through a doc comment, a
  lane report, a PR body, a reviewer brief and a summary to Ev before a reviewer
  stopped it by implementing both versions. **Evidence-shaped claims propagate
  faster than bare ones, because each reader takes the evidence as having been
  checked by the last.**
- **Commit every edit first, then take citations from the committed tree, then
  write them up.** Any other order re-creates the defect by construction — unit
  9's citation defect recurred inside the pass that was fixing it, because a
  gloss fix inserted lines above two assertions. Stronger than "check your
  citations", because it removes the window rather than asking for vigilance
  inside it. Its durable half: **a test name survives an edit and a line number
  does not**, so cite the enclosing symbol with the number as a hint.
- **A green unearned by a prior red is a measurement of the harness** (§6.2).
- **A sweep result is only as wide as the command actually typed.** Unit 10
  re-swept `work/ docs/` instead of the root and missed a site the same merge
  had created; the reviewer reproduced the arithmetic exactly, 9 hits under
  those two directories against 10 from the root.
- **A figure's scope comes from the claim, not the command.** `cargo test`
  fail-fasts at the lib binary, so a drill run without `--no-fail-fast` never
  sees the rest; unit 12 reported 1 where the crate-wide truth was 5. The same
  shape one level down: **a shared-literal list is a function of how you
  tokenize, so the tokenization is part of the claim.**

Two operational lessons from the same day, both cheap and both already costly
once: **concurrent lanes need one `git worktree` each** (three lanes shared
`/home/user/cad` and the orchestrator staged `work/` from `origin/main` onto a
lane's branch), and **`main` in an ephemeral container is not `origin/main`**
(this container's local `main` was 11,510 commits behind, and a lane correctly
reported an ancient snapshot as a possible sibling-lane conflict).

### 6.4 Where these go — ruled: a second `[ev]` PR before the sweep

They are process findings, not instrument findings. Nothing in §6.1–§6.3 is
specific to `tools/`, and `work/meter/log.md` — the only place they are written
down today — is deleted at close. The log itself flags this: *"a candidate for
the standing discipline docs rather than a METER row."*

**Ruled: into `docs/prompts/implementer-discipline.md`, and Ev sees the diff.**
The walk offered two routes and recommended the first: one item on `work/meta/`'s
slate carrying this text, or the text folded straight into the page. Ev took
neither and chose the alternative §8 named — *"5. i would like to see the
diff"* — so **the change to that page rides its own `[ev]` PR, opened after
this one merges and before the sweep** (step 2 of the header's sequence). That
is the stronger form of the walk's own argument for not folding it in here:
the page binds every lane in the repo, so a change to it should be seen as its
own change. `work/meta/program.md`'s `paths` carries `docs/prompts/*`, so META
is the owner by charter and not by convenience, and the `[ev]` PR is filed
against that fence.

**What that PR carries**, so the lane that writes it is not left to infer the
scope: §6.1's nine assertions that cannot fail, §6.2's three fake greens and
§6.3's shape-versus-reading finding with its six sub-rules. Three of them are
amendments to sections that already exist and would be strengthened rather than
lengthened — the quoted sentences below are on `main` at
`docs/prompts/implementer-discipline.md:140` and `:179`:

- §5 *Sweeps* already says *"a sweep whose blind spot is unstated is an
  unverified claim"*. The uniqueness rule and *"a sweep result is only as wide
  as the command actually typed"* are the same rule sharpened by six instances,
  and the second is the concrete failure §5 does not currently name.
- §2 *Verification* already says *"a build is not a test"*. The three fake
  greens are the next case down — a TEST is not a test either, if nothing
  checked that it can red — and the red-first rule belongs beside it.
- A short new subsection for the citation ordering rule and the
  evidence-shaped-claim question, which have no existing home.

None of it is written here: this PR rules and records, and the diff is its own
PR by Ev's ruling.

**It has to happen before the sweep**, because after the sweep this document is
a DOC-LEDGER row and §6 is recoverable only at a SHA. That is why the header's
sequence puts the discipline PR at step 2, ahead of the successor PR and the
sweep, rather than leaving it to follow them.

## 7. What this program leaves behind

Three instrument crates whose claims are now checked by execution rather than
asserted in prose: `tools/tess-lint` (the sizing block refused at `parse`, the
refusal order pinned, the cut line's two languages held to one grammar, the
report's columns named and gated, one CSV fixture with one home, and a
`baseline_census.rs` that re-derives every figure the corpus can move —
including `C15`'s residue, which is now guarded rather than argued);
`tools/tess-meter` (`SPLIT_SCAN_SAMPLES = 379` with its licence re-derived and
its false half named false and exhibited, a `StableName` column in the budget
CSV, and a cross-root roster pin that reds when either side moves); and
`tools/k-lint` (an `EPS_COUPLED_PREDICATES` roster pinned against the kernel's
minted names in the direction a pin can close, and an era guard that asserts
each witness's predicate as well as its value and survives corruption of the
committed artefact).

And one ratified page: `tools/README.md`, clauses `CC1`–`CC5` — the
reading-boundary rule (Ev, 2026-09-08), the second design page outside
`crates/<crate>/README.md` and the first governing two sibling crates. Both
crates `include_str!` it, so a moved page stops them compiling, and each
asserts that every clause it cites is a heading there and that the page carries
no clause it has not seen. `docs/DESIGN.md`'s companion table points at it. It
does not move at the sweep: it already lives beside the code it governs.

## 8. Ruled by Ev at ratification — the record

The five questions this section put to Ev on 2026-09-08 are answered, and its
sixth item was not a question and has been withdrawn. **Three of the five came
back against the option the walk recommended** — Q4 and Q5 by Ev taking
alternatives this section had itself named, and Q3 as an outright reversal that
the walk's own re-check then confirmed against the tree. They are recorded here
in the form that makes them checkable rather than in the form that makes the
walk look right. Ev's own words are quoted; where the answer is the walk's
correction rather than his ruling, that is said.

| # | Question as put | Ruling | Where it is folded in |
|---|---|---|---|
| 1 | The successor — is it wanted, under what name and band | **OPEN IT.** *"1. open"*, then *"the new 3, **instr**, and your plan all sound good!"* — the name `instr` was offered as a placeholder and is now the name. A/B band **3300–3399**, the next free one, approved as part of that same sentence rather than named by him | §1 criterion 2, §4 (header, `paths`/`ab_band`/name notes, charter) |
| 2 | The two blocked rows, and possibly a third | **They stay on the successor's slate.** *"2. giving them to the successor sounds good"* — `tess-lint-zero-certificate-two-meanings`, `k-lint-roster-wants-a-kernel-side-vocabulary` and the row composing with it are not re-homed to S-MESH and PROPS. The code state is identical either way; the ruling settles who is looking at them | §3, the paragraph after the instrument groups |
| 3 | `C15` to code-quality Track X | **REVERSED — it goes to the successor.** Ev's *"3. huh i thought track X had closed"* was a challenge, not a ruling, and the re-check it forced found the walk's claim wrong: Track X is nearly empty (two open rows, one parked, all five of its rows in `work/code-quality/`) and, more to the point, **`demos/tour` is in no program's `paths` at all**. He then took the corrected recommendation: *"the new 3 … sound good"* | §5's first row, which carries the whole correction; §3's slate count; §4's `keep_out` and charter |
| 4 | `cert1-notes-pr-body-tracked-on-main` to `work/issues/` | **DELETE.** *"4. delete"* — the one-word answer §8 offered as the faster route than a re-home. The file goes in the successor PR, not this one | §5's third row |
| 5 | Where §6's process findings go | **Ev sees the diff first.** *"5. i would like to see the diff"* — so the change to `docs/prompts/implementer-discipline.md` becomes a **second `[ev]` PR, after this one merges and before the sweep**, rather than a `work/meta/` item | §6.4, and step 2 of the header's sequence |
| 6 | *(withdrawn)* That the residue is not re-homed in this PR | **Not a question, and Ev said so**: *"6. this doesn't look like a question?"* It was a disclosure in a list headed *open with Ev*. It is now stated plainly at the end of §5 and in the header, and asks nothing of anyone | §5's closing paragraph |

**What is settled and what is not.** Settled: that `instr` opens, its name, its
slate at twenty, the four dispositions, and the order of the four PRs. Not
settled here, and left to the successor's orchestrator as METER's was:
**whether `instr` runs any A/B protocol at all**. METER claimed a band and ran
none, by Ev's posture ruling of 2026-09-07; `instr` inherits the band and the
question, and `docs/MODEL-AB-LOG.md` records the claim in the commit that opens
the program.

**The band number is the one field taken from an approval of the whole.** Ev
named the successor and the plan; he did not repeat *3300–3399* in his own
words. It is the next free band by `docs/MODEL-AB-LOG.md`'s own banding entry
(*"with 3300+ unallocated"*), it was in front of him in the sentence he
approved, and nothing depends on it that a later re-band could not fix — but it
is recorded as an inference rather than as a quotation, because that is the
difference between the two.
