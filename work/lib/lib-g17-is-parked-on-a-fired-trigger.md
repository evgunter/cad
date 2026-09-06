---
id: lib-g17-is-parked-on-a-fired-trigger
kind: issue
title: LIB-G17 is parked on a trigger that has fired (blocked_on 1202, closed 2026-09-04)
status: closed
opened: 2026-09-05
closed: 2026-09-06
---

**Courtesy filing from SEAT (PR 1995); LIB's row to move.**

`work/lib/LIB-G17.md` is `status: parked` with `blocked_on: [1202]`.
That entry is an INT, so `work.py lint` does not resolve it and the
fired-trigger rule (`work/README.md`, "A fired trigger is not a
blocker") cannot see it — the row stays parked with nothing going red.

**The trigger has fired.** 1202's tracker file,
`work/shell/shell-needs-shellnaming-birth-channel.md`, is `status:
closed`, `closed: 2026-09-04`: the `ShellNaming` birth channel it asked
for exists, written by the shell doors themselves
(`crates/topo/src/shell.rs`, `ShellNaming` and `Shelled`). LIB-G17's
body states the block as "held behind issue #1202, the kernel ask for a
`ShellNaming` birth channel — without an emitter a shell node mints no
`StableName`s", and that ask is discharged.

SEAT-9 adds the second half of the enabler on the verb seat:
`verbs::VerbRecord::Shell(ShellNaming)` carries the record across the
run door by value, and `Verb::run_shell` is the door a lowering would
call (`crates/verbs/src/run.rs`). So a `Node::Shell` lowering now has
both the record and the door it needs.

**What LIB-G17 wants**: opening, or re-parking on whatever actually
gates it now (the sequencing note in its body says "after LIB-TUBE",
which is a plan statement rather than a `blocked_on` entry). SEAT does
not edit another program's item — `work/README.md`'s one-file-one-item
rule makes that a merge conflict by design — so this is the handoff.

## Closed (2026-09-06, LIB orchestrator)

`LIB-G17` is `open` with its `blocked_on` cleared and an "Opened"
section citing the same evidence this filing gave. The class the
filing points at — a `parked` row whose `blocked_on` is an INT that
lint cannot resolve — is real and is not closed by this; it is the
tracker's (`work/meta/`), not LIB's, and is left to that program to
file or fold.
