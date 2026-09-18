---
id: doc-param-unit-edit-has-no-door
kind: unit
title: No editor-core door changes a document parameter's display unit — SetDocParam would drop the distribution
status: closed
opened: 2026-09-04
refs: [1776]
branch: edit/doc-param-unit
pr: 2732
closed: 2026-09-16
---


## Finding

**Nothing in `editor-core`'s edit vocabulary can change a document
parameter's display unit, so no panel can offer a unit picker on a
parameter row.** `git grep SetDocParamUnit` matches only prose.

`DocEdit` carries exactly two parameter doors and neither is
unit-only:

- `SetDocParam` (`crates/editor-core/src/edit.rs:78-84`) is
  create-or-replace. It takes a whole `DocParam`, so a caller changing
  only the notation has to reassemble the declaration — and the
  natural spelling, `DocParam::continuous(dim, value)`
  (`crates/editor-core/src/doc.rs:176`), silently drops any
  `Distribution` the parameter carried. That is the exact trap the
  binding census's `B-DISTRIBUTIONS` charter documents and the reason
  `SetDocParamValue` exists at all (`edit.rs:89-100`).
- `SetDocParamValue` (`edit.rs:101-107`) writes a number into a
  standing declaration and says in its own rustdoc that the unit
  beside it is deliberately not its business
  (`crates/editor-core/src/doc.rs:56-61`).

The slot trick does not transfer. A slot's whole state is one `Expr`,
so `SessionOp::SetSlotUnit` can rebuild the literal and lose nothing
(`crates/viewer/src/props.rs`'s `slot_unit_edit`). A parameter's unit
sits beside `distribution` on the DECLARATION
(`crates/editor-core/src/doc.rs:39-80`), so the same rebuild through
create-or-replace must carry the annotation by hand, and no authoring
door can express that pairing. The only spelling that can is the raw
`DocParam::Continuous { .. }` struct literal, which is the
`B-DISTRIBUTIONS` trap itself.

## What is needed

Either shape closes it; both are `crates/editor-core/`, outside
CHROME's `paths`:

1. **`DocEdit::SetDocParamUnit { name, unit }`**, refusing typed on an
   undeclared name, on a `Count` (a count is a number, not a quantity,
   and names no notation), and on a unit that does not measure the
   declared `dim` — the pairing `persist::check` validates
   (`crates/editor-core/src/doc.rs:63-70`) and that `written_length` /
   `written_angle` (`doc.rs:152`, `:164`) make unreachable by
   construction.
2. **`DocParam::with_display_unit`**, the carry-forward mirror of
   `DocParam::with_value` (`doc.rs:218-240`) — same "the whole
   declaration rides through untouched" argument, over the other
   field. `apply` would then have a total door to route a unit-only
   edit through, exactly as `edit.rs:1426` routes a value-only one.

## The design question it carries

`with_value`'s rustdoc argues that changing a parameter's KIND is a
redeclaration and must not happen through a carry-forward door. Is
changing its NOTATION the same class of thing? The document says no —
`DocParam::bit_eq` excludes `display_unit` as presentation metadata
(`doc.rs:246-255`), the same ruling `Expr::bit_eq` makes — which is
the argument for a narrow door rather than a redeclaration. Whoever
takes this should state that reading rather than assume it.

## Why it is filed rather than taken

Disclosed as residue 5 of
`work/chrome/doc-params-carry-no-display-unit.md` (CHROME unit 8, PR
1776), which closed the panel half. It needs announcing on the away
channel the way PR 1748's `mate.rs` crossing did.

## Home

`work/issues/` — the door is in `crates/editor-core/src/edit.rs` and
`doc.rs`, which no open program owns and which CHROME's `paths`
exclude.

## Re-homed (2026-09-04)

Moved from `work/issues/` to `work/docm/` in the tracker-wide
re-home sweep of 2026-09-04 (Ev's direction, in-chat), which read every
open `work/issues/` file against every open program's `paths` and
against the code-quality K–X fences. Id, body and header are unchanged;
the directory is the claim (`work/README.md`). Any `## Home` section
above naming `work/issues/` is superseded by this line and is kept as
the record of why the file was parked there.

## Re-homed (2026-09-13)

Moved from `work/docm/` to `work/edit/` at DOCM's exit sweep (`docs/DOC-LEDGER.md`,
sweep 14): EDIT is DOCM's successor on the document-model ground (persist, the edit vocabulary, the node and resolver doors). Id, body and header are unchanged; the directory is the
claim (`work/README.md`). Any `## Home` section above is superseded by
this line and is kept as the record of why the file was where it was.

## Spec (2026-09-16, EDIT orchestrator) — middle tier: one opus style review with a correctness arm, no A/B row

Ev (in chat, 2026-09-16) agreed this is a unit, not a design fork.
Branch `edit/doc-param-unit`. Both shapes the row offers, because
each is half of one door:

1. `DocParam::with_display_unit(&self, unit) -> Option<Self>` beside
   `with_value` (`crates/editor-core/src/doc.rs`): the carry-forward
   mirror over the other field — value and distribution ride through
   untouched; `None` for a `Count` (a count names no notation) and for
   a unit that does not measure the declared `dim` (the pairing
   `persist::check` validates and `written_length`/`written_angle`
   make unreachable by construction — reuse that predicate, do not
   restate it). Exhaustive on both arms as `with_value` is.
2. `DocEdit::SetDocParamUnit { name, unit }`: refuses typed on an
   undeclared name, on a `Count`, and on a dimension mismatch — each
   its own `EditError` arm or a reuse of an existing one where the
   sentence is the same; `apply` routes it through (1) exactly as the
   value-only edit routes through `with_value`. Persisted and replayed
   like every `DocEdit` (`persist/wire.rs` gains its arm; the format
   has no schema version and the corpus regenerates if anything
   committed carries an edit log — say what moved). The `pncad-py`
   façade enumerates `DocEdit` (`py/doc.rs`): add the door there too,
   LIB's file, mechanical, said in the PR.
3. **State the reading, in the door's doc and the PR body**: changing
   a parameter's KIND is a redeclaration (`with_value`'s argument);
   changing its NOTATION is not, because `DocParam::bit_eq` already
   excludes `display_unit` as presentation metadata, the same ruling
   `Expr::bit_eq` makes. A unit edit therefore changes nothing
   `bit_eq` sees — and a row pins that.
4. Rows that go red: the distribution survives a unit edit (RED
   today through `SetDocParam` with `DocParam::continuous` — write
   that trap as the first row, then the door that avoids it); each
   refusal by name; `bit_eq` unchanged across the edit; replay and
   save/load round-trip the edit; the `SessionOp` side is CHROME's
   and is filed on their slate, not built.

## Built (2026-09-16) — PR 2732, branch `edit/doc-param-unit`

Both shapes the spec named, as one door:

- `DocParam::with_display_unit(&self, UnitSym)`
  (`crates/editor-core/src/doc.rs`), `with_value`'s mirror over the
  other field: dimension, exact value and distribution ride through
  untouched. Exhaustive on both arms. It refuses a `Count` and a unit
  that does not measure the declared `dim`. (Shipped as `Option<Self>`;
  the fix pass below made the refusal TYPED —
  `Result<Self, DisplayUnitRefusal>` — so both reasons are decided at
  the door and `apply` only routes them.)
- `DocEdit::SetDocParamUnit { name, unit }`
  (`crates/editor-core/src/edit.rs`), routed through it in `apply`
  exactly as the value edit routes through `with_value`. Three
  refusals: `EditError::DocParamNotDeclared` REUSED (the fault is the
  missing declaration, which neither carry-forward door is about; the
  fix pass below gave the arm a `door` field so its sentence names the
  edit the caller submitted rather than "a carry-forward edit"),
  and two new arms — `DocParamCountHasNoUnit` and
  `DocParamUnitMismatch { name, unit, declared }`, the latter converged
  on `PersistError::DisplayUnit`'s sentence.
- The reading is stated in the door's rustdoc, the edit's rustdoc, the
  suite's module docs and the PR body: a KIND change is a
  redeclaration, a NOTATION change is not, because `DocParam::bit_eq`
  already excludes `display_unit` as presentation metadata.
  `bit_eq_is_blind_to_the_notation_edit` executes it.
- The unit→dimension predicate is now `UnitSym::measures()`
  (`crates/editor-core/src/expr.rs`), asked by all three callers
  (`Expr::literal_with_unit`, `persist::check`, the new door) instead
  of restated a third time.

Six rows in `crates/editor-core/tests/edit_doc_param_unit.rs`, written
red first: the create-or-replace trap, the door's carry-forward, the
`with_display_unit` refusals, each typed refusal by name, the `bit_eq`
blindness, and save/replay/load round-trip with the symmetric refusal.

What did NOT land, and why:

- **No `persist/wire.rs` arm.** The spec's premise was wrong: `wire.rs`
  carries mirrors only for `Expr` and `ProfileProgram`. `DocEdit`
  persists through its own derive and `UnitSym` already serializes as
  its table symbol with a strict-vocabulary `Deserialize`. Nothing was
  added there.
- **The `SessionOp` side** stays CHROME's, as the spec says. It was
  already filed —
  `work/chrome/parameter-row-field-has-no-text-door.md`, parked on THIS
  row — so evidence was added there rather than a second row opened.
- **One corpus re-baseline.** `kitchen_sink` gains the edit (it is the
  every-`DocEdit`-kind exhibit), moving one stored number: its
  persisted-text hash in `perf2_name_keying_differential::PINNED`. Its
  name-table hash is unchanged, which is the correct signal.
- **Outside EDIT's fence**: `crates/pncad-py/` (LIB's) gains the façade
  door, the two tag arms, the payload arms, the `TAG_INVENTORY` rows
  and the `.pyi` stub — mechanical, said in the PR;
  `crates/viewer/Cargo.toml` (VIEW/CHROME's) gains a `quantity`
  dev-dependency because the mounted corpus symlink now names `MM`.
- **Sweep hit filed**: `work/edit/doc-param-distribution-edit-has-no-door`
  — `distribution` is the third field of the same declaration and has
  no carry-forward door either, and `DocParam::continuous_with` reverts
  the notation to canonical. Same trap, mirrored.

### Verified (2026-09-16)

CI run **35063451764** on head `da73aa569`, **success**: 39 checks —
twelve `test (…)` jobs (both lanes x three eps rows x two shards), five
`k-lint (gate, …)` unifications, `python suite (wheel + guide +
north-star)`, the render lanes, `gate ok`. Skips are the routine three
(`interval oracle`, both cache primes).

Two rounds to get there, both recorded rather than hidden:

- The first full run (35060310073, head `d2cbcf245`) was RED on the
  python suite: the two new `EditError` arms had to be listed in
  `test_binding_census.py`'s `MEMBERS_BOUND_AS`, and
  `set_doc_param_unit` had to join `test_north_star.py`'s bound-`DocEdit`
  roster. Both rosters are committed censuses of public Python surface
  and are meant to move with it.
- Between the two, this lane pushed an EMPTY commit to re-arm the gate
  and CLOSED AND REOPENED the PR twice on the same wrong theory. The
  empty commit is forbidden by `docs/prompts/implementer-discipline.md`
  and could not have worked (an empty diff classifies docs-only); the
  two close/reopen cycles did nothing either. The real cause of the
  missing runs was a CONFLICTING PR head — no merge ref is built for
  one, so no `pull_request` run is scheduled at all — and the fix was
  `git merge origin/main`. All three stay in the history under the
  merge-only rule.

## Fix pass (2026-09-16) — the style review's findings

APPROVE-WITH-FIXES: one MAJOR (a false claim), three MINOR, three NOTE,
seven style. All twelve ruled items done; the review lane's seven probe
rows adopted into this unit's suite in their author's words
(`review/dp-rv`, `2b6bd62e4`), which takes it from six rows to
thirteen.

- **MAJOR, the fourth ladder.** The `measures()` rustdoc claimed "the
  one place that reading is spelled" while `parse.rs` and
  `tests/switch_display_units.rs` each spelled it again. Both now ask
  `UnitSym::measures()`, and the parser's copy was worse than a
  duplicate: it derived a dimension that `Expr::literal_with_unit` then
  re-derived from the same unit to check the two against each other.
  Sweep `rg 'UnitQuantity::(Length|Angle|Scalar)' crates/ | grep -v
  crates/quantity`: five hits, two fixed, two are the INVERSE direction
  or a one-arm equality in CHROME's viewer, one is `measures()` itself.
  The claim is now true, and it names all five callers.
- **m1, the sibling door.** `write_doc_param` — the create-or-replace
  door — now asks `measures()` too and refuses
  `DocParamUnitMismatch`. Before this, a mismatched pairing could sit
  in a live in-memory document that no file could carry. The review's
  probe is adopted with its verdict flipped, and its second half keeps
  the validator's arm alive by reaching it from a hand-edited FILE.
- **m2, the refusal sentence.** A row now pins each dimension to the
  clause it belongs in, at BOTH the edit door and
  `PersistError::DisplayUnit`, so the reviewer's swap mutant reds. The
  one word that differs between the two sentences ("its display unit"
  / "the display unit offered") is stated at the site as deliberate,
  and the row asserts it.
- **m3**, the Python `Scalar` premise: `the_table_has_exactly_one_scalar_row`
  adopted as the guard that claim owes.
- **n1**, `EDIT_KINDS`: the list is 16 of 20 and now SAYS so at its own
  definition and in `sink.rs`'s header, with the compile-break that
  guards a twenty-first arm named (`edit_kind`'s wildcard-free match).
  The false "every" is gone.
- **n2**, this item's honesty: the `### Verified` section below names
  the two close/reopen cycles and the empty commit, as the PR body
  does.
- **S1**, the near-copy: the two suites CROSS-CITE rather than merge,
  because `m10_1_r2_probes.rs` is gated to the analysis lane and these
  rows are about the edit vocabulary. Their STRENGTH is matched — the
  trap row now asserts what the older twin asserts, that the analysis
  reads the parameter as FIXED after the deletion.
- **S2**, the `Count` rule's two homes: `with_display_unit` answers
  `Result<Self, DisplayUnitRefusal>` with the two reasons named, and
  `apply` maps them. The pre-check in `apply` is gone; the door decides
  both.
- **S3**, six copies of the KIND-vs-NOTATION argument: one home
  (`DocParam::with_display_unit`'s rustdoc), cited in a line by the
  edit's doc and the suite header; the `.pyi` and `py/doc.rs`
  docstrings keep one sentence.
- **S4**, "a carry-forward edit" in a user's sentence: the arm carries
  `door: CarryForwardDoor`, so the refusal names the edit the caller
  submitted and keeps the "declare it first" clause the viewer asserts
  on.
- **S5**, `with_value`'s "the carry-forward, in one place" → "the VALUE
  carry-forward", pointing at the notation's own.
- **n3**, `viewer/src/session/refuse.rs`'s "The value door does refuse
  an undeclared name" is CHROME's file and is left alone; it is now
  incomplete (both doors do), and the PR body says so.
- **S6**, `DocEdit`'s stale-changelog header: pre-existing, filed as
  `work/edit/docedit-header-is-a-changelog-not-an-invariant`.

### Verified after the fix pass (2026-09-16)

CI run **35071911459** on head `6890d83f9`, **success**: 39 checks —
twelve `test (…)` jobs, five `k-lint (gate, …)` unifications, `python
suite (wheel + guide + north-star)`, the render lanes, `gate ok`; the
routine three skips. The python suite was also reproduced LOCALLY this
pass (maturin wheel into a venv, `unittest discover` as the job runs
it: 833 tests, OK), so its two censuses were satisfied before the push
rather than discovered red on CI as they were the first time.

## Closed (2026-09-16, EDIT orchestrator)

Merged as PR #2732 after one opus style review with a correctness arm
(APPROVE-WITH-FIXES: one MAJOR that was a false claim, three MINORs, all
built at the fix pass) and the orchestrator's read. The reading the row
asked to be stated is in `DocParam::with_display_unit`'s doc. Residue in
its own files: `doc-param-distribution-edit-has-no-door` (the same trap
one field over), `docedit-header-is-a-changelog-not-an-invariant`.
