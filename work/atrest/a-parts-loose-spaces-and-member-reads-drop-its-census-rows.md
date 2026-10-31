---
id: a-parts-loose-spaces-and-member-reads-drop-its-census-rows
kind: issue
title: A part's loose spaces never cross the seam, and a member read of an instance drops the part's census rows
status: open
opened: 2026-10-19
priority: P3
cost: M
refs: [5810, 5325]
---

Found by the review of PR #5325, which carries a part's census verdict
across the seam (`a-part-drops-its-census-findings-at-the-seam`).
Reviewer findings m2 and m4. Both predate that PR.

**A part's loose spaces.** A part's own `assemble` checks every loose
space it has (`gate_spaces`, `crates/editor-core/src/assembly.rs`):
- it reports a space's overlap as interference;
- it refuses a space's unrecorded touch or escalation
  (`AssemblyError::Space`).

`PartValue` is built from the part's world product alone
(`crates/editor-core/src/eval/parts.rs`, `resolve_and_evaluate`), and
`assembly::seam_verdict` never reads `product.spaces`. So:
- a document instantiating a part whose loose space refuses gates `Ok`;
- an overlap inside that space is reported zero times there.

Inspected, not run.

**A member read of an instance.** A keyed read of one member of an
instance's copies (`copy_of`, `crates/editor-core/src/eval/wire.rs`)
builds its value with `contacts: Arc::default(), carried: Arc::default()`.
When that member is itself an instance of a part Q, Q's census rows and
pairs are dropped there. The document placing the member then re-finds
Q's in-copy findings and refuses them unattributed (`AtRest`), which is
the defect PR #5325 closed for a whole-instance read. Contact records
drop on the same path already. Inspected, not run.

**The work.**
- Spaces: fold each space's verdict into `SeamVerdict`, routed by the
  space's root, or rule that a part's loose spaces are its own business
  and say so in ASSEMBLY.md A5.
- Member reads: carry the member's own rows (records, declarations,
  census rows and pairs) through `copy_of`. The member is a key-stable
  copy, so its pairs re-key as a whole instance's do.

INTENT's per-space census proposal
(`analysis/design-fork/a-space-decides-its-own-contacts`) would subsume
the first half if it is taken.


## Re-read at D′1 (2026-10-22)

The loose space is gone (`a-placement-names-its-target-space`): a copy
is in its placement's target space, and every bundle pins. With it went
`gate_spaces`, `AssemblyError::Space` and `Product::spaces`, so the
at-rest gate now checks the world alone, in a part and in the document
itself: a space other than the world (a `new_space()`, or a
construction's own) is census-checked nowhere. The first half above is
restated as that: no space but the world is checked, at the seam or
before it. The member-read half stands as written.

## Slate triage (2026-10-28, INTENT)

Moved from `work/intent/` by `git mv`, id and body unchanged: residue of INTENT's D10 build that belongs to ATREST's charter (at-rest census work; ATREST's stage 5 C makes the census a check at each `⊔`, the same door). INTENT keeps D10's core (G and its riders, the stage-3 umbrellas, the carry, the refactor rows, the roots rename); Ev asked for INTENT's slate to be cut to its charter and its residue handed on (in chat, 2026-10-28).
