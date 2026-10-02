---
id: a-word-sketched-as-one-profile-refuses-multiple-outer-loops
kind: issue
title: A word sketched as one profile refuses MultipleOuterLoops, so each glyph is its own profile, extrude and boolean
status: open
opened: 2026-10-02
priority: P3
cost: M
---


## What

A user engraving a word sketches it once: one profile holding a loop
per glyph, extruded once, subtracted once. `Profile::validate`
refuses that profile `MultipleOuterLoops { outer_loops: [0, 1, 2] }`
(`crates/profile/src/validate.rs`, whose doc on the variant says
"multi-region profiles are deferred past M2"): a profile is one face
region.

Met by the `tiltedcut` scene (`demos/tour/src/curvedcut.rs`), whose
lettering is three disjoint glyphs, C, U and T, on one sketch plane.
The scene validates, extrudes and subtracts each glyph separately
(`build`'s loop over `lettering`), three booleans where the user meant
one. No ratified decision rules multi-region profiles out; the variant
names a deferral.

## The shape of a fix

Either `validate` accepts several outer loops (each with its holes)
and the sweep doors extrude a multi-region profile to a multi-lump
body the boolean takes as one operand, or the authoring surface offers
a door from several loops to several profiles so the per-glyph loop
is the library's rather than every caller's.
