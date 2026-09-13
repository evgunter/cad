---
id: a-new-kind-pair-arrives-unguarded-by-default
kind: issue
title: a new error/kind pair arrives unguarded by default, and the two purpose-built guards that exist have never fired
status: open
opened: 2026-09-12
refs: [kind-mirrors-have-no-single-declaration, 2417]
---



(FIX orchestrator, 2026-09-12) The residue of
`kind-mirrors-have-no-single-declaration`, **declined by Ev in-chat on
2026-09-12** after its scale check came back feasible. Filed at the
moment of disclosure, because the declined row's own `## Closed` prose
is a record of work done and invisible to the re-homing sweep
(`work/README.md`).

## The gap that survives the decline

A fieldless kind enum beside a payload-carrying error is two
hand-written declarations plus a hand-written projection. The compiler
sees one direction: an arm added to the ERROR reds the exhaustive
`kind()`. The other two hazards — a variant added to the KIND alone
(a phantom nothing constructs), and an arm projected to the WRONG kind
(`Self::Merge(_) => Kind::Join` type-checks) — are caught only by
whatever each pair remembered to buy.

**So a new pair arrives unguarded by default.** That is the durable
finding of the declined row, it is untouched by declining the macro,
and it does not need a code generator.

## The cheap answer, and it is probably the right one

**A sentence in the error-type convention**: a new fieldless kind enum
owes an exhaustive consumer in its owning crate, and the doc on the
kind enum names it. Two of the four pairs already satisfy this for
free, which is the argument that the bar is reachable:

- `PathErrorKind` → `path_error_tag` (`crates/pncad-py/src/tags.rs`),
  the **FFI tag map**, which exists for its own job and catches
  phantoms as a side effect its own doc states;
- `AttrKind` → `pncad-py/src/tags.rs:300-302` and
  `editor-core/src/appearance.rs:112`'s `noun()`, both real consumers.

Where to write it is the open half: `docs/DESIGN.md` and the
`crates/<crate>/README.md` design pages are Ev's to amend, so the
convention's home wants naming before the sentence is drafted.

## The second question, which this row also carries

**Do the two purpose-built guards earn their keep?**
`crates/topo/src/boolean/mod.rs:2878` and
`crates/editor-core/src/product.rs:963` are the only guards written
FOR this purpose, at roughly 230 lines with their `label()` tables and
`sample_errors()`.

**Measured while declining the macro: neither has a recorded firing.**
No instance of a phantom or a wrong pairing appears anywhere in the
tracker or `docs/`. That cuts both ways and the row does not prejudge
it — a cheap guard against a SILENT class is often worth keeping
unfired, and silence is exactly what makes this class expensive when it
does land. But "these guards have never caught anything" is a fact
worth holding while deciding whether a third pair should copy them.

**If the answer is that they earn it**, the convention sentence above
is what generalises them, and the hand-copying (PR 1806 wrote the
two-part shape, PR 2344 re-derived it from scratch) is what the
sentence is for. **If it is that they do not**, deleting them is a
separate small unit and this row says so rather than assuming.

## What is NOT owed

The macro. `docs/FIX-ERRKINDS-SPEC.md` is retired
(`docs/DOC-LEDGER.md`), and the declined row records why on its own
evidence. Do not re-open it from the feasibility answer — the macro can
be written; the decision was that it should not be.
