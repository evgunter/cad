# WIRE — the evaluation seat (plan)

**STATUS: OPEN (2026-09-11).** Successor to EVAL (closed 2026-09-08,
`docs/DOC-LEDGER.md` sweep 9), opened in the tracker cut of 2026-09-11
(`docs/WORK-TRACKS-2026-09.md` addendum 3). Live state is
`work/wire/log.md`'s tail and the item files beside this plan.

Branch prefix: **`wire/`** — unit branches `wire/<unit>-<slug>`. The
retired prefix `eval/` names EVAL's branches and is not reused.
Away-channel tag `(WIRE orchestrator)`. A/B ordinal band
**WIRE = 3700–3799**, claimed in `docs/MODEL-AB-LOG.md` in the opening
commit.

## Charter

EVAL's exit left the seat's ground with no live program and named its
own residue. That residue is this program's opening slate and the ledger
says so in as many words: `two-verb-seats-do-not-compose` (deferred),
`frame-f64-placement-is-re-evaluated-per-profile` and
`wire-expected-phrases-spell-family-words-as-literals` went to
`work/issues/` at the sweep with `profile-embed-lift-has-two-homes-anchor-and-loft`
and `placement-lifts-its-affine-by-hand-beside-affine3-map` already
there, as *"the successor's opening slate"*.

Around them this program takes the rest of the seat: the two `names`
refusals that discard what they caught, the content-tag pair, the
`Witness*` residues EVAL disclosed and did not carry, the two profile
lift doors, and the two vocabularies — `Target`'s tag and the arc-mode
spelling — that are mirrored by hand between `crates/profile` and
`crates/editor-core`.

The substrate is ratified and is not re-litigated here:
`docs/DESIGN.md` Band 1 (the content key's inputs, as PR 2201 amended
it), `crates/profile/README.md` V6 (the validated lift), and
`crates/verbs/README.md`'s `Verb` convention.

## Territory and the seams

Territory: EVAL's `paths` less the files DOCM holds, plus
`crates/editor-core/src/placement.rs` and `product.rs`, which were in no
program's `paths` at this cut. The seams are in `program.md`'s
`keep_out`; the two that will actually bite are **`program.rs`**
(DOCM's — both vocabulary rows reach `res_spec`/`res_target` and
announce rather than land) and **`crates/profile/*`** (S-BOOL's glob —
the lift door is minted there by announced seam, exactly as EVAL's
EVAL-1 and FILLET's fillet door did).

## The slate

| item | class | what it is | where the work lands |
| --- | --- | --- | --- |
| `placement-lifts-its-affine-by-hand-beside-affine3-map` | **E** | `Mat3::map`/`Affine3::map` already exist; two functions collapse to one call | `crates/editor-core/src/placement.rs:202,215-218` |
| `wire-expected-phrases-spell-family-words-as-literals` | **E** | **SEVEN** literals swap to existing consts, not nine, and there are **eight** composed sites (six distinct strings), not seven — the lane re-counted against the tree and the item was also internally inconsistent before it moved. The composed ones stay prose. | `crates/editor-core/src/eval/wire.rs`, `crates/editor-core/src/eval/mod.rs` (`family` consts at `:426`) |
| `names-flush-and-select-discard-a-refusal-with-map-err-underscore` | **~~E~~ → H** | **The estimate was wrong.** Four sites not two, across two crates and three refusal surfaces, ten files — and the decision the item delegated to the owner could not be made without a measurement that refutes a doc in a third crate. That measurement, not the edit, was the unit. | `names/{flush,select,discriminate,geompred,emit}.rs`, `names/README.md`, `crates/pncad-py/src/*` (LIB, forced), `crates/editor-core/tests/display_contract.rs` |
| `frame-f64-placement-is-re-evaluated-per-profile` | **M** | One crate's eval module, but carries a correctness arm over values and possibly keys | `crates/editor-core/src/eval/mod.rs`, `crates/editor-core/src/eval/wire.rs`, `crates/editor-core/src/eval/slots.rs` (+ content keys) |
| `profile-has-no-scalar-lift-door` | **M** | Mint a door by an existing convention, but door and caller sit on two fences | `crates/profile/src/` (`map_scalar` on `ProfileVertex`, `ProfileLoop`, `Section` — `lib.rs`/`structure.rs`/`lift.rs`), `crates/sweep/src/loft.rs` (`end_profile`) |
| `profile-embed-lift-has-two-homes-anchor-and-loft` | **M** | New public lift API across three crates; owner undecided and D385 door shape must be settled | `crates/profile/src/*` (new `map_scalar`/lift door, `validate.rs`), `crates/sweep/src/loft.rs:225-245`, `crates/editor-core/tests/pinned_lift_validates_once.rs`, `crates/profile/tests/common/mod.rs` |
| `D364` | **M** | Two crates, shape known from PR 1475, but a new census must be built | `crates/profile/src/path/program.rs` (`Target` enum: tag + `ALL`), `crates/editor-core/src/program.rs` (`res_target`/`res_spec` `tgt` closure), new census test under `crates/editor-core/tests/` or `crates/profile/tests/` |
| `product-gather-refuses-a-split-root-whose-tie-spans-both-halves` | **M** | Needs a stated rule for a tie spanning one root's bodies; probe exists, fix is local. | `crates/editor-core/src/product.rs` (`carry_names` :786-816), `crates/editor-core/src/names/table.rs:186-189`, a regression test in `crates/editor-core/tests/` |
| `S40` | *(deferred)* | One residue after the split: `WitnessSlot` + `NodeErrorKind::WitnessBifurcation`, reserved for a solver M6 shipped without. The other three bullets are PROPS's and SHELL's now. | `crates/editor-core/src/eval/mod.rs` (`WitnessSlot`, `WitnessBifurcation`) |
| `interrogate-writes-the-family-vocabulary-a-third-time` | **E** | A fourth reader of the family vocabulary, in a file no program owns; the taker draws the fence in its PR | `crates/editor-core/src/names/interrogate.rs:438-446` |
| `wire-refusals-answer-found-with-a-negation-of-expected` | **E** | Two sites, but it moves user-visible refusal text, so it owes the assertion sweep | `crates/editor-core/src/eval/wire.rs:978,4191` |
| `composed-expected-phrases-are-hand-copied-across-sites` | **M** | Needs a licensing rule that covers narrower-than, wider-than and sentence cases, or a home for the repeated phrases | `crates/editor-core/src/eval/wire.rs`, `mate/member.rs:716`, `verbs/split.rs:157` |
| `frame-plane-lane-and-axis-frame-are-one-door` | **E** | Two functions, one destructure, one refusal; the comment saying so is the evidence | `crates/editor-core/src/eval/wire.rs:1011-1032,1072-1094` |
| `wire-rs-module-header-describes-five-sixths-of-the-file` | **E** | The header claim is falsified by a 720-line subsystem; correcting it is a sentence | `crates/editor-core/src/eval/wire.rs:1-5` |
| `frame-linear-generic-door-has-no-consumers` | **E** | Zero call sites workspace-wide; the decision is delete / keep-as-API / demote, and it is a public surface call | `crates/editor-core/src/placement.rs:230-232` |
| `placement-rs-states-its-exactness-rule-in-nine-paragraphs` | **M** | One rule, nine homes, plus a `#[must_use]` inconsistency and a 60% prose ratio | `crates/editor-core/src/placement.rs` |
| `emit-topo-destroys-the-edge-key-in-a-split-lineage-cycle` | **E** | One site, the kind stays honest and the locator dies; same class, weaker instance | `crates/editor-core/src/names/emit_topo.rs:127` |
| `S195` | **H** | Four mirrored vocabularies across Track V and DOCM paths; one-census-or-four is a design call. | `crates/profile/src/path/{verbs.rs,program.rs}`, `crates/editor-core/src/program.rs` (`res_spec`), `crates/editor-core/src/persist/wire.rs`, new `ALL`+corpus census |
| `axis-flavoured-declarations-have-no-channel` | **H** | Needs a new placement-level identity channel; item itself calls it an `[ev]`-shaped design question | `crates/verbs/` (`ParamSource`, README §3 P1/P2), `crates/topo/src/boolean/join.rs` (`cs_pair_frame`, `CoaxialEvidence`), `crates/topo/src/source.rs`, germ/`pair_section_frame` dispatch |
| `two-verb-seats-do-not-compose` | **H** | Items (2)/(3) are an unratified design round on kernel identity; waits for a replay consumer | `crates/verbs/` (README §5, verb decls), `crates/topo/src/source.rs` birth records, `crates/editor-core/src/eval/` |

## Order

The four E rows open the program and are one PR each.
`contact-class-has-two-content-tag-functions` **closed on 2026-09-11**
without a unit: the reading held, `contact_class_tag` is not in the tree,
and the persist module's string vocabulary is a second projection of one
enum over `ALL` rather than the twin the finding reported. The other
three dispatched together as the opening block.

Then the lift pair in one sequence — `profile-has-no-scalar-lift-door`
mints the door, `profile-embed-lift-has-two-homes-anchor-and-loft`
retires the second home against it — because staffing them apart mints
the door twice, which is the standing trap on work of this shape:
**the fix mints a fresh instance of the defect it closes**, and naming
that in your own PR body does not prevent it. Only a reader who did not
write the fix has ever caught it.

`frame-f64-placement-…` and `product-gather-…` follow.

**`D364` is a subset of `S195` and they are staffed as one sequence.**
`S195`'s finding names *"the same shape, in three smaller pairs"* —
`ProgramTarget`/`WireTarget`, `ArcSide`/`WireSide`,
`ArcSweep`/`WireWinding` — and the first of those three IS `D364`'s
vocabulary, at the same `res_spec`/`res_target` hop in
`crates/editor-core/src/program.rs`. Staffing them apart builds the
census twice, which is `plan.md`'s ordering rule 5 again, one vocabulary
family over. So: **`D364` runs first as the prototype** — its shape is
known from PR 1475, it is the smallest of the four, and it produces a
working `ALL`-anchored census — and **`S195` then generalises it** over
the remaining three pairs plus `ArcData`, answering one-census-or-four
with a built thing rather than an argument. If `D364`'s lane finds the
census does not generalise, that is a finding and this paragraph is
wrong; say so in the PR.

`S40` is no longer the H tail. It was four residues on three programs'
ground; it is now one, `deferred` against `docs/DESIGN.md`'s roadmap
sentence that the sketch solver re-opens as its own design pass when
constraint-driven sketches have a consumer. The other three bullets are
their own files on PROPS's and SHELL's slates — the split is recorded in
`work/wire/S40.md`.

**Not takeable, and deliberately.** `axis-flavoured-declarations-have-no-channel`
is the `[ev]`-shaped fork between a placement-level declaration
(CURVED's shape) and frame-level identity (TOPO's); it opens as an
`[ev]` PR, not as a unit. `two-verb-seats-do-not-compose` is `deferred`
— ratified not-now, waiting on a replay consumer — and no lane resolves
a deferred row by implementing it.

## Review posture

**Set by Ev in-chat, 2026-09-11, at the orchestrator's opening: a LIGHT
style review on every unit, and a FULL review on the units with a real
risk of being wrong.** That is a narrowing of EVAL's inherited posture
(style review with a correctness arm wherever a unit moves what a
document evaluates to) and it supersedes it for this program. The
**A/B protocol is not run here** (Ev, same direction), which matches
what `docs/MODEL-AB-LOG.md` already records for the eleven programs of
the 2026-09-11 cut: WIRE's band **3700–3799** was claimed at the opening
for bookkeeping, and no ordinal is drawn from it.

Which units are which, decided at the opening and correctable by any
lane that finds the call wrong:

| full review | why it can be wrong |
| --- | --- |
| `frame-f64-placement-is-re-evaluated-per-profile` | carries the placement on a `NodeResult` and makes the profile plane a READ — it moves values and possibly content keys |
| `placement-lifts-its-affine-by-hand-beside-affine3-map` *(raised at review time)* | the unit went past its item and swapped `compose`'s general arm for `Affine3`'s `Mul`, on ASM-4 D-3's bit-level path |
| `names-flush-and-select-discard-a-refusal-with-map-err-underscore` *(raised at review time)* | two public enum variants gain a field, the argument rests on subnormal-float reachability, and user-visible Python refusal text moves |
| `profile-has-no-scalar-lift-door` + `profile-embed-lift-has-two-homes-anchor-and-loft` | a new public lift API across three crates, and `end_profile` re-`validate`s the lift at `T` today; getting the door wrong changes what a loft builds |
| `product-gather-refuses-a-split-root-whose-tie-spans-both-halves` | a stated rule about the product's aggregate name table, and the rule decides whether a name survives |
| `D364` | a census is a claim about a vocabulary's completeness; a census with a hole reports green |
| ~~`S195`~~ **lowered to light at review time** | the row turned out to be three-quarters discharged by work that landed after it was filed, so the unit was dispatched as a MEASUREMENT and its diff is 58 lines of test on a failure path with the assertion untouched. The census it was going to build already exists; nothing here can report green over a hole. The measurement itself is the deliverable and the reviewer re-takes it, which a light review does |

Everything else is a light style review: the three E units, and any
prose or tracker pass. A light review is still a review — the reviewer
gets claims to falsify and
`docs/prompts/reviewer-style-lane.md` by path — it is just not paired
with a correctness arm of its own.

## How the class column is read

`E` / `M` / `H` is a **dispatch estimate**, made on 2026-09-11 by reading
each row against the tree, and it is the axis this program's order runs
on. It is not a verdict on the finding and it is not in any header: no
field carries it, `work.py` does not parse it, and this table is the only
place it lives. A lane that finds the estimate wrong says so in its PR
and this table is corrected in the same PR.

- **E** — the fix is written in the row or obvious from it: one or a few
  files, no design question, no ruling, small diff.
- **M** — multi-file, or a small design call (where a shared home lives,
  what a door looks like), or a census or instrument to build first.
- **H** — cross-cutting, numeric or algorithmic, gated on a ruling, or
  spanning several programs' territory.

The cut that opened this program is `docs/WORK-TRACKS-2026-09.md`
addendum 3; it is a survey, and this plan supersedes it as the charter.
