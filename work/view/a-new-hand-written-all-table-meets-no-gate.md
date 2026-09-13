---
id: a-new-hand-written-all-table-meets-no-gate
kind: issue
title: nothing mechanical stops a new hand-written const ALL table appearing in the viewer
status: closed
opened: 2026-09-06
closed: 2026-09-07
branch: view/all-gate
pr: 2106
---


Filed by the `const-all` unit as the disclosed non-take its own PR
owed a schedule for (PR 2046, finding S13). §Q6: a disclosed non-take
owes a named unit, and a README paragraph is not one.

## What is and is not held

The nine converted vocabularies are held by the compiler: their `ALL`
is projected from the enum's declaration by
`crates/viewer/src/vocab.rs`'s `vocabulary!`, so a variant cannot reach
the enum without reaching the list. **Nothing holds the next one.** An
author who writes

    impl NewChoice {
        pub const ALL: [Self; 3] = [Self::A, Self::B, Self::C];
    }

meets no compile error, no clippy lint and no gate — only
`crates/viewer/README.md`'s **Closed vocabularies are declared once**,
if they read it. That is the standard this crate rejected when it built
`scripts/gates/viewer-module-kinds.sh`, whose own header
(`scripts/gates/viewer-module-kinds.sh:10-18`) exists because a rule
sold as "mechanically checkable" spent its first life with nothing
reading one.

## The gate, and why it was not written in 2046

The scan is cheap and the reviewer of 2046 described it: **hit on any
`const ALL: [Self; N] = [ … ]` (and the un-named shape: any `const`
array literal of two or more `Type::Variant` entries) under
`crates/viewer/src`, allowlist the three kinds the README section
ratifies** — a struct-constant registry (`Theme::ALL`), a deliberately
partial list, a mirror of an enum declared in another crate.

The reason it is an item rather than a hunk of that PR is siting, not
size. A gate must fire on its own inputs (Ev, 2026-08-20, on S61), and
this one's inputs are `crates/viewer/src` plus the README section that
holds its allowlist — so it needs a `ci.yml` step, a
`scripts/gates/gate-roster.sh` registration, a line in
`local-scripts/ci-local.sh`, and a `scripts/check-ci-mirror-parity.py`
decision about whether it is TIER-blind (its allowlist lives in a
README, so a docs-only change can falsify it — which is exactly the
argument that put `viewer-module-kinds.sh` in the `mirror` job). None
of those is a question about `const ALL`, and answering them inside a
PR that already converted nine enums is how a review loses the thread.

## Hazards for whoever takes it

- `scripts/gates/lib.sh:113-141` forbids a trailing `|| true` on a
  scanning pipeline: it swallows grep's exit 2 (a real error) along
  with exit 1 (no match). This program has already re-minted that
  defect once, at #1953.
- The allowlist must be READ from the README section, not restated in
  the script — `viewer-module-kinds.sh`'s own contract, and the reason
  its rosters have not gone stale.
- `crates/viewer/tests/` holds four hand-written complete variant lists
  already (`viewer-suites-hold-hand-written-complete-variant-lists`),
  so whether the scan covers the suites is a decision, not an
  oversight. They are a different shape — inline arrays in rows, not
  `const ALL` tables — and a scan tuned for one will not see the other.

## Closed (2026-09-07)

`scripts/gates/viewer-vocab-declared-once.sh` is the gate, sited in
ci.yml's `mirror` job, in `local-scripts/ci-local.sh`'s
`tier_blind_rows`, and named in `scripts/check-ci-mirror-parity.py`'s
`TIER_BLIND`. `gate-roster.sh` needed no edit — it derives the roster
from the directory — and now reports 21 gates rather than 20, which is
the registration.

**The branch crosses two other programs' territory, ruled in rather
than avoided**: `scripts/gates/*` is GATES', and the three wiring
surfaces are CIW's. `work/gates/program.md`'s own `keep_out` admits a
new gate's wiring row as one announced line, and the parity entry
cannot be split off — a TIER-blind gate with no `TIER_BLIND` row reds
parity itself. The announcement and the undrawn half of the fence are
`work/issues/gate-wiring-fence-is-undrawn-for-the-parity-entry`;
neither program's `program.md` is edited here.

**One finding went out of fence**: `scripts/gates/lib.sh`'s statement
view splits a `const` whose type is an array at the `;` inside
`[T; N]`, so the initialiser lands in a record with no `const` in it.
The gate carries a bracket-depth item reader as the workaround and says
so at its own site;
`work/issues/gate-rust-reader-splits-an-array-type-at-its-semicolon` is
where the fix belongs.

**TIER-BLIND, and the argument is the one this item forecast.** Half
the gate's subject is `crates/viewer/README.md`: the allowlist rows,
the vocabulary of kinds a row may claim, and the table's own shape. A
change set of only that file classifies TIER=docs, on which every
`if: run_build` job is skipped — so sited in `discipline` the arms that
exist for a table edit could not fire on a table edit. It is the
`viewer-module-kinds.sh` argument with a sharper subject, because that
gate's README dependence is two of its checks and this one's is its
whole allowlist.

**The allowlist is read, not restated.** The README section gained a
`#### The lists that stay hand-written` table — four rows, `List` /
`Module` / `Kind` — and the gate reads the rows from it and the KINDS
from the section's own bolded bullets, both scoped to that section. The
gate holds the NUMBER of bullets as its own constant, so a fourth kind
costs an edit there as well as here and cannot arrive as a docs-tier
table cell; reading the bullets alone left a fourth bullet plus a row
claiming it internally consistent and green. The roster retires itself
in both directions — a list added without a row reds, and a row whose
list has been converted reds too — and does not spread: one row
ratifies exactly one list, so two rows for one list and one row for a
module declaring two lists under that name both red.

**What the pre-merge correctness review moved.** Three defects, none
of which a green run could show: the `awk` that decides what a hit IS
ran unguarded after the guarded `const_items`, and with no data rows in
the roster it printed `OK` over two planted breaches; `const` also
opens a GENERIC parameter, so an unanchored opening began accumulating
inside `fn stack<const N: usize>` and swallowed the `const ALL` three
lines below it; and a row keyed on module and name ratified any number
of lists answering to both. The reader population is now stated as a
rule and guarded per pipeline STAGE, the item opening is anchored to a
declaration at the start of a line (`static` too), and every row must
match exactly one hit. `work/view/log.md` carries the full account.

**Both shapes are in scope.** Three of the four hand-written lists in
the crate are un-named (`BOOLEAN_OPS`, `MATE_PRIMITIVES`,
`SUBJECTS_WITH_AN_EXPIRY_ISSUER`) and one is named (`Theme::ALL`), so a
named-only gate would be evaded by calling the next table `KINDS`.

**`crates/viewer/tests/` is out of scope, deliberately.** The scan is
anchored on `const` and `static` items and the suites' lists are inline
arrays in a row, so it would not find one of the four instances
`viewer-suites-hold-hand-written-complete-variant-lists` names if it
looked there. Widening it adds exactly one hit — `tests/theme.rs`'s
`KINDS`, a deliberately partial list already argued in place — and
would leave a reader believing the suites are covered. **That item
stays open**; the argument is at the gate's header and in this PR's
body.
