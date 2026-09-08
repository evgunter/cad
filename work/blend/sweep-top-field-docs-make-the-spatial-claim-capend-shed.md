---
id: sweep-top-field-docs-make-the-spatial-claim-capend-shed
kind: unit
title: Extruded.top and Lofted.top doc comments make the spatial claim CapEnd shed under a signed distance
status: review
opened: 2026-09-04
branch: blend/2-top-cap-doc
pr: 2122
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

## Re-homed (2026-09-06)

Moved from `work/issues/` to `work/blend/` in the tracker-wide cut of 2026-09-06 (Ev's direction, in-chat), which read every open `work/issues/` file and every open code-quality row against every live program's `paths` and opened four programs for the ground none covered. Id, body and header are unchanged except as noted; the directory is the claim (`work/README.md`). `extrude.rs` is BLEND's; `loft.rs`'s twin sentence is S-BOOL's file and is announced.

## Landed

Prose only, on PR 2122. `Extruded::top` and `Extruded::bottom`
(`crates/sweep/src/extrude.rs`) now state themselves as the FAR and NEAR ends
of the sweep — `top` on the sketch plane translated by `w` with its outward
normal along `w`, so under `w · n < 0` it lies on the `−n` side of the sketch
plane; `bottom` on the sketch plane itself, outward opposite `w`. Neither doc
restates the winding rule: which cap carries the profile's canonical winding is
a direction convention and its one home is the crate docs
(`crates/sweep/src/lib.rs`, "Direction conventions (normative, stated once —
owned here)"), which already state it for both signs; the field docs link
there. Two review probe rows pin the sentences against built bodies for both
signs, through both extrusion doors, on an axis-aligned and a tilted sketch
plane (`crates/sweep/tests/review_blend_e2_r1_probes.rs`).

`crates/sweep/src/loft.rs`'s `Lofted::top`/`::bottom` needed no edit: they were
already end-relative ("section k−1's plane, outward along the stacking" /
"section 0's plane, outward against it"), so the item's second half is stale as
written. The file was touched for a different reason — one word of the
"raised" class, announced in `work/blend/program.md`'s keep_out.

**The rename was declined, and the count is why.** Renaming the two fields to
the sweep vector's ends would move **74 read sites across 13 files in three
crates** (`sweep`, `editor-core`, `verbs`), plus the 4 field declarations
themselves = 78 across 15 files. That number is a **floor**, not a ceiling: it
counts only receivers narrowable to `Extruded`/`Lofted` values by grep, and
cannot see a value arriving through a field, a destructuring, a closure
parameter, a trait method's `Self`, a macro expansion, or a helper whose return
type is an alias or `impl Trait`. Three programs' ground for a one-sentence
honesty problem the prose fixes outright, and the `CapEnd` seam the rename
argument names is EVAL's emitter, not this field pair. The full per-file table
is in PR 2122's body.
