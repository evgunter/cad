---
id: two-rules-disagree-on-when-a-fixture-leaves-a-suite
kind: issue
title: sweep's src/test_support header and its tests/common routing rule disagree about where a fixture with a second in-crate suite belongs
status: open
opened: 2026-09-20
needs_ev: true
priority: P4
cost: E
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

### What `git log -S` says, re-taken

`CLAUDE.md` says to run `git log -S` before waiting on Ev. Run over
every ref:

```
git log --all -S'or a second suite inside it does' \
    -- crates/sweep/src/test_support.rs
git log --all -S'narrowest one all of its consumers can reach' \
    -- crates/sweep/tests/common/mod.rs
```

Each returns **108 commits**. The two sets are **not** the same set —
they differ by six either way. And **107 of the 108 in each are
parentless**: this is a shallow clone, so at every graft boundary the
whole tree reads as added, `-S` sees the string appear, and
`--name-only` duly shows the file. That is 107 non-answers per query
wearing the shape of an answer, which is a sharper demonstration that
the instrument is unusable as-run than any small number would have
been.

**One filter recovers it.** In each set exactly **one** commit has
parents, and in each case that commit is the one that ADDED the clause:

| clause | the one non-graft commit | what it did |
| --- | --- | --- |
| `src/test_support.rs`'s disjunction | `b52d478ae` | added `//!   crate needs it or a second suite inside it does; the narrower` |
| `tests/common/mod.rs`'s narrowest-home rule | `9d73d71d9` | added `//! the narrowest one all of its consumers can reach:` |

**Both are agent commits, not Ev's.** So the check does complete, and
its answer is that **neither clause is ratified** — each was written by
a lane, and no ruling is attached to either.

`CLAUDE.md` says that where no ratification turns up there is none —
so the next section is the whole of why this row still carries
`needs_ev`, and it does not rest on either clause being ratified.

## Why it is Ev's

Both sentences are standing instructions handed to future lanes by
path — the same shape as `docs/prompts/`, one level down — and which
one governs decides where every future `sweep` fixture goes. That is
the test `CLAUDE.md` states for what waits: **text that binds future
work rather than describing this change**, which is a wider set than
the ratified-decision case and does not require either clause to have
been ratified. A lane picking one silently is how the pair stays
unreconciled; a lane rewriting one is a lane changing a standing
instruction. Neither document should be reworded until the answer is
settled.

**The argument against, stated so it is not hidden**: nothing here is
ratified, and `CLAUDE.md`'s procedure for that case is to proceed. A
reader who finds that decisive should clear `needs_ev` and let a lane
reconcile the pair. This row records the conflict and the measurement
either way; the measurement is the part that was missing.

## Why it sits on S-DUP's slate

`scripts/work.py territory` puts `crates/sweep/src/` on S-BLEND's
ground and `crates/sweep/tests/` on S-TCOST's and S-TINT's, so no one
owner covers the pair. The finding is about where a SHARED fixture
lives, which is this program's subject, and S-DUP claims no territory
by design (`plan.md`). One row rather than three.
