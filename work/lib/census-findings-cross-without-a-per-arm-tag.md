---
id: census-findings-cross-without-a-per-arm-tag
kind: issue
title: a tier-3' census finding crosses to Python as prose with no per-arm tag
status: open
opened: 2026-09-04
---


Disclosed by `tier-3-prime-findings-render-through-debug` (FIX), which
fixed the RENDERING of the tier-3′ census arms and deliberately did not
settle this. Filed as its own file because a residue disclosed only in a
closed item's prose is invisible to the re-homing sweep.

## The measurement

`Body::validate_pseudomanifold` raises one `ValidationError` carrying
every finding joined into one message, with exactly two structured
attributes — `door` and `failure_count`
(`crates/pncad-py/src/py/value.rs`, `run_validator`). Which coincidence
the census found is in the PROSE only. A caller who wants to branch on
"vertex-on-face" versus "edge-edge crossing" has to parse the message,
which is the thing tags exist to prevent.

That is stated, not accidental: `CensusContact` is `INTERIOR` in the
binding census and its row says why, and
`crates/pncad-py/tests/test_validate.py::TestTheRefusalsShape::test_no_per_arm_tag_crosses_and_the_census_says_so`
pins the absence — `kind` and `variant` are asserted NOT to exist on the
refusal. So the current state has a pin behind it and closing this issue
means turning that pin around, not merely adding an attribute.

## What a unit closing it would have to decide

1. **Whether a joined refusal can carry per-arm tags at all.** One raise
   carries N findings; `variant` is a scalar attribute everywhere else
   in `crate::tags`. Either the door raises one exception per finding
   (a behaviour change to a door with pinned `failure_count` semantics),
   or the tag becomes a sequence attribute — a shape no other typed
   refusal in the binding uses.
2. **Whether `CensusContact` leaves `INTERIOR`.** A per-arm tag is a
   committed vocabulary, and the census row's argument for INTERIOR is
   what would have to be withdrawn.

Adjacent but NOT this: `work/lib/next-payload-rung-under-the-cur3-cur4-carriages.md`
asks whether the payload TYPES are curated surface. This asks whether
the exception carries a discriminant. A door could answer either
without answering the other.

## Home

`crates/pncad-py/*` is LIB's territory and the census row is LIB's
document, so this is LIB's to schedule; it is filed in `work/issues/`
rather than on that slate because a unit branch does not file into
another program's directory.

## Re-homed (2026-09-04)

Moved from `work/issues/` to `work/lib/` in the tracker-wide
re-home sweep of 2026-09-04 (Ev's direction, in-chat), which read every
open `work/issues/` file against every open program's `paths` and
against the code-quality K–X fences. Id, body and header are unchanged;
the directory is the claim (`work/README.md`). Any `## Home` section
above naming `work/issues/` is superseded by this line and is kept as
the record of why the file was parked there.
