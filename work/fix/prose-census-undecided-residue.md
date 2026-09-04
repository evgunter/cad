---
id: prose-census-undecided-residue
kind: issue
title: the prose census leaves 28 renderings undecided: positional {:?} over untyped expressions, the Real scalar, and names with rival declarations
status: open
opened: 2026-09-04
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
