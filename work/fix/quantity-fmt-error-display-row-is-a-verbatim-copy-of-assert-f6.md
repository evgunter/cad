---
id: quantity-fmt-error-display-row-is-a-verbatim-copy-of-assert-f6
kind: issue
title: quantity's FmtQuantityError display row is a verbatim copy of assert_f6's body, panic wording included
status: closed
opened: 2026-09-19
branch: fix/quantity-f6-fold
pr: 2944
closed: 2026-09-21
---

Found by S-DUP's F6 fold of `crates/viewer/tests/panel_edits.rs`, whose
census re-derivation reached this file. **It sits on S-FIX's slate
rather than S-DUP's because S-DUP claims no paths** (`work/dup/plan.md`,
"Territory — none"); `python3 scripts/work.py territory` names S-FIX as
this path's sole owner.

## The finding

`crates/quantity/src/tests.rs`,
`fmt_quantity_error_display_names_its_content_not_its_struct`, spells
all four F6 clauses inline — **and spells them with
`test_utils::f6::assert_f6`'s own panic wording, character for
character**:

    "{err:?} renders as {shown:?}, missing {want:?}"
    "{err:?} renders as {shown:?} — that is the variant name, i.e. a struct dump"
    "{err:?} renders as {shown:?} — that is Debug punctuation, not a sentence"
    assert_ne!(shown, format!("{err:?}"));

That is not a paraphrase of the shared door; it is the door's body
transcribed. Every existing row in this class is about a **ban list**
that mirrors an enum; this is the whole predicate, one clause at a
time, and no sweep for `dumps`, `assert_f6`, `guts` or `contains('{')`
in a `tests/` directory reaches it, because it is inline mid-file in a
`src/` unit-test module.

## The drift it already carries

`dumps` here is a one-element list: `!shown.contains("NonFinite")`.
`FmtQuantityError`'s other arms' identifiers are not banned, so a
rendering that leaked a **sibling** arm's name would pass — the same
approximation S-DUP folded out of `panel_edits.rs` and proved mattered
by planting a sibling leak (green under the per-arm form, red under the
roster form). Unlike `viewer`'s, this list also has no compiler behind
it: a variant added to `FmtQuantityError` leaves the row asserting
about one identifier and saying nothing about the new one.

## What makes it awkward, and why it is a decision rather than a fold

`crates/quantity/Cargo.toml` has an **empty `[dependencies]`** with a
comment stating that as the design: *"the one home that serves both is
a leaf crate with no dependencies."* Folding onto
`test_utils::f6::assert_f6` adds `test-utils` to `[dev-dependencies]`
(where `proptest` already sits), so the stated leaf property is not
touched — but it is the kind of manifest edge that wants saying out
loud rather than assuming, and it is S-FIX's to decide, not S-DUP's.

Worth checking in the same breath: `crates/pncad-py`'s wheel closure is
the **non-dev** dependency closure (`docs/prompts/implementer-discipline.md`),
so a dev-dependency edge from `quantity` does not move which crates buy
the python suite. That is the reasoning, not a measurement — re-take it
at the merge base.

## Instrument that found it, and its blind spot

`git grep -nE '(contains|find|matches|starts_with)\s*\(\s*["'"'"']\s*\{'`
over **every tracked file, no path argument** — the brace literal in any
spelling, which is the one clause every full copy of F6 must contain.
Its blind spot: a copy that bans only identifiers and never the brace.
A name-shaped sweep (`assert_f6`, `dumps`, `guts`) finds nothing here.

## Ruled (FIX orchestrator, 2026-09-20): fold onto `test_utils::f6::assert_f6`

`crates/quantity/*` is FIX's by `paths` and territory names no other
owner, so the manifest call this row flagged is this seat's. The seat
rules **fold**, and adds `test-utils` to `[dev-dependencies]`.

**The leaf property the manifest comment states is not what is at
stake.** *"The one home that serves both is a leaf crate with no
dependencies"* is a claim about what `quantity` makes its DEPENDENTS
carry; `proptest` already sits in `[dev-dependencies]` beside where
this edge goes, so the stated property is untouched by construction. A
test-only edge that buys the shared predicate is the cheap side of that
trade.

**What the lane re-takes rather than inherits.** The row reasons that
`pncad-py`'s wheel closure is the **non-dev** dependency closure
(`docs/prompts/implementer-discipline.md`), so a dev edge from
`quantity` does not move which crates buy the python suite — and says
plainly that this is reasoning, not a measurement. Re-take it at the
merge base and read the `change filter` job's log, which prints both
the seed set and `RUN_PNCAD_PY`, so the run says which way it went.

**The drift is the reason the fold is worth more than a tidy-up**, and
the lane should keep it in the PR body: `dumps` here is a one-element
list (`!shown.contains("NonFinite")`), so a rendering that leaked a
SIBLING arm's name passes today, and nothing puts a compiler behind
that list. S-DUP proved that gap mattered on `viewer`'s copy by
planting a sibling leak — green under the per-arm form, red under the
roster form. Plant the same leak here and show it red before the fold
lands, or say why you could not.

**Fence:** `crates/test-utils/*` is S-TCOST's and S-TINT's. The
dev-dependency edge is announced there, in the PR body and on their
log.

## Closed (2026-09-21) — PR 2944, and the lane improved on the ruling

The row is folded onto `test-utils`' shared F6 door, with `test-utils`
added to `crates/quantity`'s `[dev-dependencies]` and the manifest
comment now stating why the leaf property is untouched.

**The lane landed `assert_f6_every_variant` + `f6_variants!`, not the
bare `assert_f6` the ruling named, and it was right to.** Folding onto
the bare form takes a `dumps` argument, which for this enum would have
been a hand-typed `&["NonFinite"]` — re-minting the exact drift this row
exists to close (*"nothing puts a compiler behind that list"*) and
re-spelling a hand-written mirror of an error enum, which
`work/tint/assert-f6-dump-lists-are-hand-written-mirrors-of-error-enums`
closed and which the weld's own docs forbid. **The ruling was the
narrower reading; the census form is the stronger one.** A deviation
better than the letter owes nothing further, and the lane flagged it for
adjudication rather than burying it, which is what made it cheap to
accept.

**The red-first was EXECUTED, in three commits on the branch**, not
argued — this program shipped one argued-rather-than-executed half in
wave 3 and it was a mistake. `FmtQuantityError` has one arm, so the
sibling leak had to be planted: green under the old inline form (the
blind spot measured, not asserted), then `error[E0004]: non-exhaustive
patterns` under the census form (the compiler half the inline row never
had), then runtime red naming the leak. Plant removed; the net diff
carries neither.

**The closure measurement was taken honestly and the lane said which
half was which.** CI printed `RUN_PNCAD_PY=true`, but that is
fail-closed: the diff touches `Cargo.lock`, so the filter falls to
`TIER=all` and never reaches the seed arithmetic. The real measurement
is `ci-filter.py`'s own `pncad_py_seeds` called on the tree before and
after the manifest edit — **16 members both times, `quantity` in,
`test-utils` out**, matching the set CI printed. The row's reasoning
about the non-dev closure is now measured rather than inferred.

**Fence:** `crates/test-utils/*` is S-TCOST's and S-TINT's — the
dev-dependency edge, plus one module-doc clause in `source.rs` that
listed `quantity` among the crates NOT dev-depending on `test-utils` and
became false with this change. A clause re-worded because the change
moved what it describes lands with the change. Announced on both logs.

**Filed:** no new row. Evidence appended to
`work/census/the-field-brace-fingerprint-is-spelled-at-eight-sites-in-six-crates`,
whose *"where else to look"* asks for exactly the `contains('{')` sweep
this unit owed — an append rather than a second file, per §6's
grep-the-program's-directory-first.
