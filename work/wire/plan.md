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
| `wire-expected-phrases-spell-family-words-as-literals` | **E** | Nine literals swap to existing consts; the seven composed phrases are licensed to stay prose. | `crates/editor-core/src/eval/wire.rs`, `crates/editor-core/src/eval/mod.rs` (`family` consts at `:426`) |
| `names-flush-and-select-discard-a-refusal-with-map-err-underscore` | **E** | Two `map_err` sites; may add one variant to carry inner kind | `crates/editor-core/src/names/flush.rs:192`, `crates/editor-core/src/names/select.rs:719`, possibly `SelectRefusal` variant |
| `contact-class-has-two-content-tag-functions` | **E** | Appears already discharged: only `ContactClass::content_tag` survives in `eval/mod.rs`; verify and close. | `crates/editor-core/src/eval/mod.rs`, `crates/topo/src/contact.rs` |
| `frame-f64-placement-is-re-evaluated-per-profile` | **M** | One crate's eval module, but carries a correctness arm over values and possibly keys | `crates/editor-core/src/eval/mod.rs`, `crates/editor-core/src/eval/wire.rs`, `crates/editor-core/src/eval/slots.rs` (+ content keys) |
| `profile-has-no-scalar-lift-door` | **M** | Mint a door by an existing convention, but door and caller sit on two fences | `crates/profile/src/` (`map_scalar` on `ProfileVertex`, `ProfileLoop`, `Section` — `lib.rs`/`structure.rs`/`lift.rs`), `crates/sweep/src/loft.rs` (`end_profile`) |
| `profile-embed-lift-has-two-homes-anchor-and-loft` | **M** | New public lift API across three crates; owner undecided and D385 door shape must be settled | `crates/profile/src/*` (new `map_scalar`/lift door, `validate.rs`), `crates/sweep/src/loft.rs:225-245`, `crates/editor-core/tests/pinned_lift_validates_once.rs`, `crates/profile/tests/common/mod.rs` |
| `D364` | **M** | Two crates, shape known from PR 1475, but a new census must be built | `crates/profile/src/path/program.rs` (`Target` enum: tag + `ALL`), `crates/editor-core/src/program.rs` (`res_target`/`res_spec` `tgt` closure), new census test under `crates/editor-core/tests/` or `crates/profile/tests/` |
| `product-gather-refuses-a-split-root-whose-tie-spans-both-halves` | **M** | Needs a stated rule for a tie spanning one root's bodies; probe exists, fix is local. | `crates/editor-core/src/product.rs` (`carry_names` :786-816), `crates/editor-core/src/names/table.rs:186-189`, a regression test in `crates/editor-core/tests/` |
| `S40` | **H** | Four residues, each a D2/D9 design call or gated on a later wave | `crates/editor-core/src/eval/mod.rs` (`WitnessSlot`, `WitnessBifurcation`), `crates/geom-brep/src/props/curved.rs` (`unreachable_zero`, `Rim`, `du_of_rims`), `crates/topo/src/lib.rs` + `transform.rs` (HashSet) |
| `S195` | **H** | Four mirrored vocabularies across Track V and DOCM paths; one-census-or-four is a design call. | `crates/profile/src/path/{verbs.rs,program.rs}`, `crates/editor-core/src/program.rs` (`res_spec`), `crates/editor-core/src/persist/wire.rs`, new `ALL`+corpus census |
| `axis-flavoured-declarations-have-no-channel` | **H** | Needs a new placement-level identity channel; item itself calls it an `[ev]`-shaped design question | `crates/verbs/` (`ParamSource`, README §3 P1/P2), `crates/topo/src/boolean/join.rs` (`cs_pair_frame`, `CoaxialEvidence`), `crates/topo/src/source.rs`, germ/`pair_section_frame` dispatch |
| `two-verb-seats-do-not-compose` | **H** | Items (2)/(3) are an unratified design round on kernel identity; waits for a replay consumer | `crates/verbs/` (README §5, verb decls), `crates/topo/src/source.rs` birth records, `crates/editor-core/src/eval/` |

## Order

The four E rows open the program and are one PR each;
`contact-class-has-two-content-tag-functions` is a verify-and-close on
the reading that only `ContactClass::content_tag` survives, so it costs a
grep and a citation.

Then the lift pair in one sequence — `profile-has-no-scalar-lift-door`
mints the door, `profile-embed-lift-has-two-homes-anchor-and-loft`
retires the second home against it — because staffing them apart mints
the door twice, which is exactly the trap `docs/CODE-QUALITY-CONVENTIONS.md`'s ordering rule 5
names.

`frame-f64-placement-…` and `D364` follow. `S195` and `S40` are the H
tail: `S195` is four mirrored vocabularies and the one-census-or-four
question is a design call before it is a unit; `S40` is four residues,
each its own D2/D9 call.

**Not takeable, and deliberately.** `axis-flavoured-declarations-have-no-channel`
is the `[ev]`-shaped fork between a placement-level declaration
(CURVED's shape) and frame-level identity (TOPO's); it opens as an
`[ev]` PR, not as a unit. `two-verb-seats-do-not-compose` is `deferred`
— ratified not-now, waiting on a replay consumer — and no lane resolves
a deferred row by implementing it.

## Review posture

EVAL's, inherited: style review with a correctness arm where a unit
moves what a document evaluates to. The lift pair and `S195` take the
correctness arm by construction — they change a value or a key that
persists.

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
