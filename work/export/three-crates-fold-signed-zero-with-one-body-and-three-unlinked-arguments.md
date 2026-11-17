---
id: three-crates-fold-signed-zero-with-one-body-and-three-unlinked-arguments
kind: issue
title: One signed-zero fold body in three crates, each with its own paragraph of argument, none naming the others
status: review
opened: 2026-09-15
priority: P1
cost: D
branch: export/walks-and-zeros
pr: 7067
---


## Finding

Found in the S415 style review (2026-09-15).

Three crates fold `-0.0` to `0.0` with the **identical body**:

| site | function | its argument |
| --- | --- | --- |
| `crates/step-import/src/signed_zero.rs` | `plus_zero_scalar` | the writer round-trip is a fixed point, and `adopt`'s bitwise record comparison restores writer-side key sharing |
| `crates/pncad-py/src/py/doc.rs` | `fold_zero` | a hash must match the IEEE equality it mirrors (`-0.0 == 0.0`) |
| `crates/editor-core/src/distribution.rs` | `fold_signed_zeros` (its inner `f`) | normalizing a distribution's bounds |

Each is `if v == 0.0 { 0.0 } else { v }`. **None names either other**,
and `signed_zero.rs` — the module that exists to state this argument
once — is unaware that two other crates state it too.

## The filer's reading: this wants DISCLOSURE, not one home

The same verdict S415 reached about the Part 21 band, and for the same
reason. The three arguments are about **different invariants** — a
file-format fixed point, a hash/equality contract, a value
normalization — and they are not required to move together. A shared
helper would put one name on three reasons and make the day one of
them changes look like a change to all three. The bodies are one line;
collapsing them buys nothing and costs a dependency edge from two
crates to a third.

What is owed is that each site names the others as an
independently-argued copy, so a reader who finds one knows the set and
does not "fix" it into a shared constant — and so a change to the
IEEE-equality reasoning is seen by whoever owns the other two.

## Territory

Filed on EXCH because `signed_zero.rs` is the module that exists to
hold this argument, and because it is the only one of the three whose
stated job is to be the single home. The other two files are LIB's
(`crates/pncad-py/*`) and PROPS' (`crates/editor-core/src/distribution.rs`)
— the disclosure lands in three programs' territory whoever takes it.

## Where to look

- `crates/step-import/src/signed_zero.rs` — `plus_zero_scalar`, and
  the header paragraph that claims to be the one statement.
- `crates/pncad-py/src/py/doc.rs` — `fold_zero`.
- `crates/editor-core/src/distribution.rs` — `fold_signed_zeros`.

## Resolution

The fix is one home, not disclosure. The filer's reading rested on
two premises, and the PR 7067 review showed both were false:
- **No new dependency edge.** step-import, editor-core and sweep already
  depend on geom-core, and pncad-py reaches it through
  `pncad::geom_core`. geom-core also held a copy of its own
  (`predicate.rs`'s `readable_number`).
- **The invariants are not all different.** Three of the copies served
  one contract, a hash that agrees with IEEE equality: pncad-py's two
  `fold_zero`s and `Distribution::fold_signed_zeros`. `FreeVar.__hash__`
  spelled that fold two ways, four lines apart.

Disclosure would also have minted a hand-kept cross-reference set at
every site, which is the class this item exists to close.

So the fold is `geom_core::fold_signed_zero`. Every production copy
calls it, and so does each test copy whose crate reaches geom-core.
Each call site's reason stays where it was, at that site.
`interval-transcendentals` keeps its own copy: it is a separate root
that cannot depend on the kernel.
