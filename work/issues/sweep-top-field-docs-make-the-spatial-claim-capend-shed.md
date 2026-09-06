---
id: sweep-top-field-docs-make-the-spatial-claim-capend-shed
kind: issue
title: Extruded.top and Lofted.top doc comments make the spatial claim CapEnd shed under a signed distance
status: open
opened: 2026-09-04
---


## What

Found by DOCM's `CapEnd` rename (PR 1851), reported from outside that
unit's fence: `crates/sweep/src/extrude.rs:127-130` documents
`Extruded.top` as the cap above the sketch plane, and
`crates/sweep/src/loft.rs:86-87` documents `Lofted.top` the same way,
while `extrude.rs:100` states the distance is signed along the sketch
plane's normal — under a negative distance the "top" cap lies below.
Same class the rename fixed one layer up (`CapEnd::{End, Start}` in
`crates/editor-core/src/names/role.rs`): a field name and doc making a
spatial claim the construction's own parameterization contradicts.
Prose-only fix, or a rename of the two fields to the sweep vector's
ends; the kernel sweep crate is not DOCM's ground, so filed here.

## Where it stands

`work/issues/` — `crates/sweep` is VERBS'/FILLET's territory by glob;
unowned until one claims it.

## Re-homed (2026-09-04)

Moved from `work/issues/` to `work/fillet/` in the tracker-wide
re-home sweep of 2026-09-04 (Ev's direction, in-chat), which read every
open `work/issues/` file against every open program's `paths` and
against the code-quality K–X fences. Id, body and header are unchanged;
the directory is the claim (`work/README.md`). Any `## Home` section
above naming `work/issues/` is superseded by this line and is kept as
the record of why the file was parked there.
