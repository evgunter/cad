---
id: a-typed-kernel-defect-ending-may-launder-a-bug-d2-forbids
kind: issue
title: workspace: about 110 sites append KERNEL_DEFECT_ENDING to a typed refusal; each is either reachable by input (and the ending's claim is false) or reachable only by a kernel bug (and D2 says it must panic)
status: open
opened: 2026-10-19
priority: P3
cost: M
---


(Filed by the ENCL orchestrator, 2026-10-19. Held until `[ev]` PR 5134 settled what a contradiction past the import door is. It merged 2026-10-19.)

## What

`geom_core::predicate::KERNEL_DEFECT_ENDING` ("There is no way through: this is a kernel defect; report it") is the ending of "a refusal that only a kernel defect reaches". `rg -c KERNEL_DEFECT_ENDING crates` counts 113 mentions on main 74c567c570. The largest counts, by file:

| Count | File |
|---|---|
| 9 | geom-brep `ssi.rs` |
| 8 | geom-brep `certify.rs` |
| 7 | topo `shell.rs` |
| 7 | topo `boolean/mod.rs` |
| 6 | geom-brep `offset_fit.rs` |
| 5 | editor-core `eval/mod.rs` |
| 4 | topo `boolean/ops.rs` |
| 4 | editor-core `names/emit.rs` |
| 4 | editor-core `eval/anchor.rs` |

`[failure]` (D2) splits states into two classes, and every site lies in one of them:

- **Input reaches it.** Then it is a typed refusal, but the ending's claim ("this is a kernel defect") is false and withholds the lever.
- **Only a kernel bug reaches it.** Then D2 says it MUST panic, and "downgrading one … to a typed error launders a bug into a supported outcome".

A typed error carrying this ending is therefore wrong whichever class the site is in. The exceptions, if any, are where the site cannot tell: a kernel approximation limit that `[tolerance.recourse]` gives a last-resort recourse. Those take that recourse, not this ending.

`[tolerance.recourse]` (iv), as revised by PR 5134, applies the same split to the sized decisions. The proven-zero implementation (`a-proven-zero-is-definite-yet-ends-as-no-size-at-rest`) does it for the `Unminted` side. This row is the rest of the tree.

## Shape of the work

- **Census.** One line per site: its error, its door, and which class it is in, with the reachability argument.
- **Class 2** becomes `unreachable!` / `debug_assert` per `[failure]`'s taxonomy, with its test rows moved to `#[should_panic]`.
- **Class 1** takes its decision's lever.
- **The import door** keeps the file's typed ending, as `KERNEL_OR_FILE_DEFECT_ENDING`'s at-rest sites do, until EXCH's import row closes.

The census probably splits by owner. Rows in another program's territory go to that program's slate with this row as parent. The constant itself (geom-core, SCALAR's territory) retires once no site needs it.

Related: `kernel-defect-endings-and-repair-labels-have-no-shared-home`.
