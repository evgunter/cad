---
id: census-cannot-type-a-nested-pattern-binding
kind: issue
title: the prose census types no binding introduced by a nested pattern, a closure parameter or a catch-all arm
status: open
opened: 2026-09-12
refs: [prose-census-undecided-residue, error-census-keyed-on-bare-type-name]
---

Found while re-deriving `prose-census-undecided-residue`'s classes for
`error-census-keyed-on-bare-type-name`. **A fourth class of undecided
site, which that row does not name and seven of its rows were
misattributed to.**

## What the seven rows actually were

`crates/pncad-py/src/prose_census.rs`'s `UNDECIDED` roster carried
seven rows whose stated reason was *"declared at a type this tree does
not declare under that name — an alias, a re-export, or one out of
tree"*. None of them is that. Every one has an EMPTY candidate list:
`census` never resolved the binding to any type at all, so the type
table was never consulted and no name was looked up. The reasons were
corrected in place in the PR that found them; the defect underneath is
this row.

`pattern_bindings` (`crates/pncad-py/src/prose_census.rs:1130-1189`)
reads one level of variant pattern. It requires the alternative to
contain `::` — so it names a variant — and then matches each
comma-separated item of the body against that variant's declared
fields. A binding one level further in is invisible to it, and so is a
pattern that names no variant.

| site | binding | how it is bound |
|---|---|---|
| `crates/editor-core/src/persist/check.rs` | `arg` | `slot: SlotId::Profile { loop_, step, arg }` |
| `crates/profile/src/path/program.rs` | `verb` | `verb: Some(verb)` |
| `crates/topo/src/splitting/mod.rs` | `u`, `v` | `endpoints: (u, v)` |
| `crates/topo/src/replace_face.rs` | `e` | inner arm `Some(e) =>` |
| `crates/sweep/src/blend/mod.rs` | `other` | catch-all arm `other =>` |
| `crates/step-import/src/error.rs` | `e` | closure parameter `.map(\|e\| ..)` |

Seven rows of the roster's twenty-eight, which makes this the second
largest class after the positional `{:?}` one — and unlike that one it
is decidable without editing another program's files.

## The repair

Descend. A field pattern's bound half is itself a pattern, and its
shapes are already in hand: `Some(x)` over `Option<T>` binds `x: T`,
`(u, v)` over a tuple field binds each element, and a nested
`Variant { .. }` is the same walk one level down. A catch-all arm binds
the whole scrutinee, which the census would have to carry to type; a
closure parameter is a different question again and may be worth
leaving undecided with a truthful reason.

**Not `error-census-keyed-on-bare-type-name`'s unit and not a class of
`prose-census-undecided-residue`'s three.** Re-keying the census on the
declaring path does not reach any of these, because none of them ever
reaches a type name.
