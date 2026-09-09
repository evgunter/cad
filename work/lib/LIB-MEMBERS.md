---
id: LIB-MEMBERS
kind: unit
title: the census's name match accounts members, not the type
status: review
opened: 2026-09-09
branch: lib/members
refs: [datum-crosses-name-for-name-as-two-types]
---



Closes `datum-crosses-name-for-name-as-two-types` under Ev's ruling
(D): rule 1's name match stays, and a match accounts MEMBERS, not the
type.

## The count, first

Run over the merge base before any row was written: **104** curated
names that rule 1 matches resolve to a `pub enum` or `pub struct`
with named members, carrying **616** members between them. **420** of
those members over **60** types are not spelled by their Python
namesake and needed a row.

`Node`, `DocEdit` and `Datum` do NOT dominate it — they are 18 rows of
the 420. What dominates is the refusal enums, whose arms cross as tag
WORDS rather than as attributes: `ValidationError` 71, `EditError` 58,
`PathError` 30, `StepImportError` 22, `TessellateError` 15. That is
the measurement the ruling's cost estimate did not have, and it is
recorded rather than used to narrow the rule.

## Delivered

- `scripts/payload-rung-sweep.py` — `declared_members`, the second
  reader on the same declarations: an enum's variant names, a struct's
  bare-`pub` field names. Its blind spots are stated at the function
  ((e) and (h) unchanged; a tuple struct's fields have no names and so
  owe nothing). Selftest rows for every shape it draws, and two
  fixture types (`Wrapped`, `Marker`) that move the fixture's declared
  count 12 -> 14 and nothing else. **Nothing in the sweep's own report
  changed**: `--check` passes and the disposition tables are
  untouched.
- `crates/pncad-py/tests/test_binding_census.py` — the rule, its two
  rosters and four checks:
  - rule 1's docstring gains the member clause and states four blind
    spots: a coincidental snake-case match (four of
    `editor_core::Evaluation`'s ten fields are accounted that way),
    the resolver's (b)/(d) alias and generic arms, its crate-not-
    module (h), and a tuple struct's unnamed fields;
  - `resolver()` loads the sweep BY PATH (a hyphenated script is not
    an importable module) — one implementation, no re-derivation;
  - `MEMBERS_BOUND_AS` (385 rows) and `MEMBERS_NOT_BOUND` (35), keyed
    `Type::Member`. **A second pair of tables rather than
    `Type::Member` keys in `BOUND_AS`**, because every check over that
    roster reads its keys as CURATED NAMES and `Node::Union` is not
    one — the decay checks would have had to learn which keys they
    were about, which is two questions in one table;
  - `test_every_member_of_a_matched_type_is_spelled_or_listed`,
    `test_the_member_rosters_decay`,
    `test_the_member_rule_catches_a_spelling_that_moves` (the
    falsifier, run in-file against a stub surface with `Node.extrude`
    removed), plus member floors in `test_the_census_is_not_vacuous`
    and the member rosters folded into the stub-spelling, family and
    gap-id checks.
- The three findings the first run produced, each a `gap:` row citing
  a `FAMILIES` charter this file now owns, and each filed:
  `work/lib/two-datum-arms-have-no-node-constructor.md`
  (`B-DATUM-DOORS`),
  `work/lib/five-doc-edit-arms-have-no-python-door.md`
  (`B-DOC-EDITS`), and
  `work/lib/mesh-boundary-polylines-have-no-python-door.md`
  (`B-MESH-BOUNDARIES`).

## Decisions and deviations

- **`DimensionError` is a second same-spelled pair**, found by this
  rule and not by the ruling: `pncad.pyi`'s is the quantity boundary's
  refusal, the curated name is `editor_core`'s document-layer one. Its
  ten arms cross at `ParseError.kind`. Recorded at the rows.
- **Refusal arms get rows like every other member.** Their words are
  already minted by exhaustive matches in `crates/pncad-py/src/
  tags.rs`, so the rows restate a compile-time guarantee in the other
  direction (arm -> word). The ruling admits no exception and none was
  taken; the count above is what a narrowing would be argued from.
- No `pncad.pyi` change of any kind. No new binding: every member
  found unbound is a row with its reason, and the three that looked
  like real gaps are filed rather than closed here.

## Closed

`datum-crosses-name-for-name-as-two-types` is closed by this unit.
