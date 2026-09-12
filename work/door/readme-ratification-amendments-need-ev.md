---
id: readme-ratification-amendments-need-ev
kind: ruling
title: Does the viewer README kind-retirement merged in PR 2387 stand, now that README ratifications need Ev's sign-off?
status: open
opened: 2026-09-12
needs_ev: true
---


## The rule (Ev, 2026-09-12, in-chat)

**README-level ratification changes generally need Ev's sign-off.** A
crate README beside the code it governs is a design doc — `CLAUDE.md`
says so — and retiring or amending a ratified clause in one is a design
decision, not a lane's call, even when the code change forces it.

Recorded on this program's `keep_out` and here. It is a rule for every
program, not just DOOR; DOOR records it because DOOR is where it was
asked and where it was already broken once.

## What was already merged without it

PR #2387 (`topo::BooleanOp::ALL`, merged `3ad715d`) amended
`crates/viewer/README.md`:

- the ratified sentence *"**Three** kinds of list stay hand-written"*
  became *"**Two** kinds…"*;
- the bullet **"A mirror of an enum declared in another crate"** was
  retired and replaced by a prose paragraph arguing it is not a kind;
- the roster row `| BOOLEAN_OPS | forms | A mirror of an enum declared
  in another crate |` was deleted;
- `scripts/gates/viewer-vocab-declared-once.sh`'s `KIND_ANCHOR` and
  `KIND_COUNT` moved with it.

**It was not gratuitous.** The gate reads that README's roster and reds
on a row whose list no longer exists, so retiring `BOOLEAN_OPS` forced
the roster edit; the kind it claimed then had zero instances. The README
documents the amendment procedure for ADDING a kind and the lane ran it
in reverse, argued in prose, with the gate and its `--selftest` green
under three independent checks. The orchestrator flagged the change to
Ev at the time and merged on the reading that a crate README is not
`docs/DESIGN.md`. That reading is now superseded.

## The question

**Does the amendment stand?** Three answers are live and the row does
not presume one:

1. **Ratify as merged.** The kind had no instances left and the gate
   forced the edit; the tree is consistent today.
2. **Ratify the mechanics, amend the argument.** The replacement
   paragraph claims generally that *"a mirror claiming completeness has
   an answer one crate over"*. The style review of #2391 found that
   false in one live case: `work/door/viewport-pointer-buttons-mirror-a-toolkit-enum-by-hand`
   documents a complete mirror of `egui::PointerButton`, whose declaring
   crate cannot publish an `ALL`. So the retired kind's *reason* still
   holds for at least one list inside `crates/viewer/src` — the gate does
   not see it only because it is an inline array rather than a `const`.
   On this reading the kind was vacated of rostered instances, not
   falsified, and the paragraph overstates.
3. **Restore the kind**, with the pointer-button list rostered under it.

The orchestrator's read is **(2)**: the mechanics were right and forced,
and the sentence claims more than the tree supports. But this is the
call the new rule reserves for Ev, which is the point of the row.

## Not a precedent for reverting

Nothing here proposes undoing the merge. `work/README.md` and this
repo's merge-only rule make a merged amendment a fact to be ratified or
amended forward, not rewound.
