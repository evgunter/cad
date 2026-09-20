---
id: two-rules-disagree-on-when-a-fixture-leaves-a-suite
kind: issue
title: sweep's src/test_support header and its tests/common routing rule disagree about where a fixture with a second in-crate suite belongs
status: open
opened: 2026-09-20
needs_ev: true
---


## Finding

- **Where**: `crates/sweep/src/test_support.rs`'s module header and
  `crates/sweep/tests/common/mod.rs`'s routing rule.
- **Importance**: medium — both are standing instructions that bind
  every future lane routing a `sweep` fixture, and they give opposite
  answers for the commonest case
- **Confidence**: sure; both clauses quoted below, read at
  `b29fe8bd1`
- **Raised by**: the `dup/one-line-fixture-wrappers` lane, 2026-09-20,
  which had to pick one and did so silently in its first draft

`src/test_support.rs` says:

> A fixture only earns a place here once a consumer OUTSIDE this crate
> needs it **or a second suite inside it does**; the narrower homes,
> and the rule that routes between them, are stated in `sweep`'s own
> `tests/common` module.

`tests/common/mod.rs` says:

> These are the places a `sweep` suite can share from, and an item
> lives at the **narrowest one all of its consumers can reach**:
> `sweep::test_support` — fixtures the LIBRARY can build, reachable
> from in-crate tests, from here, and (behind the same dev-only
> feature) from another crate's suites, **which is where a fixture
> with consumers OUTSIDE this crate has to live** …

**They disagree on one case and it is the common one.** A fixture two
in-crate suites want satisfies `src`'s second disjunct, which says it
earns a place in `src/test_support`. It also has a narrower home all
of its consumers can reach — `tests/common` — which the second rule
says is where it lives, and whose own clause seats `src/test_support`
for OUT-of-crate consumers only.

The conflict is not cosmetic. The two homes differ in what they cost
and in what they expose: `src/test_support` is library code behind a
dev-only feature, reachable from another crate's suites and from the
in-crate `mod tests` pins, and adding to it puts a `pub fn` under
`crates/*/src` where the `witness-not-ambient` gate and the
`review_m1_pr5_internal` door tables can see it (the
`dup/cyl-rim-builder` unit paid exactly that cost). `tests/common` is
a helper module of one test binary, reachable from nothing else, and
costs a `pub mod` line.

## What this unit did, and why

The case: **six box fixtures wanted by twelve `crates/sweep/tests`
suites and by nothing outside the crate.** They went to
`crates/sweep/tests/common/operands.rs`.

The rule followed is `tests/common`'s narrowest-home rule, for three
reasons, none of which is a ruling:

- it is the rule `src`'s own sentence defers to — *"the narrower
  homes, and the rule that routes between them, are stated in
  `sweep`'s own `tests/common` module"*;
- `src`'s clause is stated as what *earns a place*, which reads as a
  floor on eligibility rather than an instruction to take it, whereas
  `tests/common`'s is stated as where an item *lives*;
- the narrower home is reachable by every consumer and costs nothing,
  and it keeps a fixture with no library consumer out of `src`.

**That reading is a lane's, not a ratification**, which is why this row
exists.

**And the usual check cannot settle it here.**
`git log --all -S'or a second suite inside it does' -- crates/sweep/src/test_support.rs`
and the same for `-S'narrowest one all of its consumers can reach' --
crates/sweep/tests/common/mod.rs` return the **same five commits**,
none of which touches either file — the shallow-clone graft artefact
`CLAUDE.md` warns about, where every file reads as added at the graft.
So the instrument that would normally find the author of a sentence
returns nothing usable for either of these two, and neither clause can
be shown ratified OR unratified from this checkout. That is a second
reason the answer is Ev's rather than a lane's.

## Why it is Ev's

Both sentences are standing instructions handed to future lanes by
path — the same shape as `docs/prompts/`, one level down — and which
one governs decides where every future `sweep` fixture goes. A lane
picking one silently is how the pair stays unreconciled; a lane
rewriting one is a lane changing a standing instruction. Neither
document should be reworded until the answer is settled.

## Why it sits on S-DUP's slate

`scripts/work.py territory` puts `crates/sweep/src/` on S-BLEND's
ground and `crates/sweep/tests/` on S-TCOST's and S-TINT's, so no one
owner covers the pair. The finding is about where a SHARED fixture
lives, which is this program's subject, and S-DUP claims no territory
by design (`plan.md`). One row rather than three.
