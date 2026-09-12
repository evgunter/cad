---
id: prose-census-undecided-residue
kind: issue
title: the prose census leaves 28 renderings undecided: positional {:?} over untyped expressions, and the Real scalar
status: open
opened: 2026-09-04
refs: [census-cannot-type-a-nested-pattern-binding, error-census-keyed-on-bare-type-name]
---



Cut with the guard it belongs to (`prose-gate-has-no-mechanical-guard`,
PR 1809). `crates/pncad-py/src/prose_census.rs`'s `UNDECIDED` roster
names every rendering the census cannot decide, with the reason it
could not. **The roster is loud — a new undecided site reds the row
until someone writes its line — but nothing shrinks it**, and
`work/README.md` is explicit that a disclosed residue owes a file
rather than a sentence. This is that file.

## The three classes, and what each would take

**1. A positional `{:?}` over an expression the census does not type**
— the largest class. The resolver reads a bare binding, `self.field`
and `ident.0`; anything else (a method call, an index, a nested field
walk) is undecided. Two directions are available and they are not the
same unit: teach the resolver the remaining expression shapes, or
require an inline `{binding:?}` at the site so the binding is named
where it is rendered. The second is a legibility improvement in its own
right and it is what makes the first unnecessary, but it edits other
programs' files.

**2. The `Real` scalar parameter.** `geom-core`'s `Interval` wraps
`interval_transcendentals::DInterval`, a named-field struct with a
derived `Debug`, so a `{x:?}` on a `T`-typed field renders a brace in
an interval build and prose in a default one. The census cannot see
the lane. **If these are live, the repair is one manual `Debug` impl
on `Interval` — not N site fixes** — and that is `geom-core`'s ground
(S-CERT's while it is live), with `interval-transcendentals/` a
separate root. What is owed first is the reachability question: does a
`T`-generic refusal reach `py::typed_err` under `--features interval`?
Nobody has traced it.

**3. Names with rival declarations.** The type table is keyed on the
bare name and indexes every `struct` and `enum` in the tree, function
bodies included, so `Verb` collects `profile`'s (macro-declared) and
`crates/verbs/src/verb.rs`'s `Verb<T>`. Disagreeing declarations answer
undecided, which is honest but coarse. Resolving a bare name through
its file's `use` items would decide most of them.

## Why it is an issue and not a unit yet

Which of the three to take first is a scope decision, and class 2's
first step is a question (reachability) rather than a diff. A taker
should read `prose_census.rs`'s module docs — the blind spots are
enumerated there beside the code, not here — and cut one class at a
time.

## Class 3 is closed; classes 1 and 2 remain (2026-09-12)

Taken as part of `error-census-keyed-on-bare-type-name`, on
`fix/census-declaring-path-key`, because one repair closes both: the
type table is now keyed on the DECLARING PATH, and a written type
resolves through its module's `use` items and through re-exports
(`as` renames included) to the module that declares it. A name that
resolves to no declaration falls back to the bare-name reading, so
failing to resolve costs precision and never soundness.

**It shrank this roster by nothing, and the class-3 paragraph above
overstated what it would do.** Three things that paragraph got wrong,
each re-derived at `1818266` rather than taken from it:

- *"Resolving a bare name through its file's `use` items would decide
  most of them"* — class 3 is **one row of the twenty-eight**, not
  most. Only `crates/editor-core/src/persist/check.rs`'s
  `ProgramFault`/`verb` ever reached the type table with a name that
  had rivals.
- **That one row is still undecided after the repair.** Its
  `Option<profile::Verb>` now resolves to
  `profile::path::program::Verb` — which is declared inside a
  `macro_rules!` body, which the shared lexer does not expand. The
  verdict is unchanged and the roster line's REASON is now true
  instead of blaming a collision.
- **Seven rows were misattributed to a class that does not exist.**
  Their stated reason was *"declared at a type this tree does not
  declare under that name — an alias, a re-export, or one out of
  tree"*; all seven have an empty candidate list, meaning the census
  never typed the binding at all and never consulted the table. They
  are a fourth class — bindings introduced by a nested pattern, a
  closure parameter or a catch-all arm — filed as
  `census-cannot-type-a-nested-pattern-binding`. The reason strings
  were corrected in the same PR.

So the roster's composition is 7 positional (class 1), 13 `Real`
scalar (class 2), 7 untypable bindings (the new row), 1 macro-declared
(was class 3). **Class 1 and class 2 are what is left on this row**,
unchanged and untouched, and class 2 still owes its reachability
question first.

Nothing here checks a reason string: the pin compares
`(file, display type, binding)` and a count, so a roster line's stated
cause is honest only because someone read the site. That is worth
knowing before the next taker trusts one.
