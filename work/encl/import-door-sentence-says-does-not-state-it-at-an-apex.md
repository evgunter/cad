---
id: import-door-sentence-says-does-not-state-it-at-an-apex
kind: issue
title: geom-core: the import door's 'the file does not state it' is false of a cone apex the file states exactly
status: closed
opened: 2026-10-13
priority: P3
cost: E
branch: encl/import-door-apex-sentence
pr: 5029
closed: 2026-10-13
---


(Filed by the ENCL implementer of `import-door-appends-a-defect-note-to-a-sub-eps-in-zero-span`, from its Answered section's "Filed separately" list. The sentence lives in `crates/geom-core/src/predicate.rs`, which `work.py territory` gives to SCALAR. The row is on ENCL because the question is the import door's wording, which ENCL's adoption work (D7's ε_in door) owns; no SCALAR mechanism is in question.)

## What

`MarginDiag::sized_recourse_in_file` (`crates/geom-core/src/predicate.rs`) opens every band-decided reading whose nearer end is at or below ε_in with "This {size} is below the file's declared coincidence distance ε_in = … m, so the file does not state it." At a cone apex the dihedral arm reads exactly 0.0 (`geom_brep::dihedral` test `a_cone_apex_at_the_import_door_keeps_its_lever_and_loses_its_note`), which the file does state, exactly. Since that unit, the door no longer appends `DIHEDRAL_ARM`'s note, so nothing contradicts the sentence any more, but the sentence is still not true of an apex.

## Question

Can the door's sentence be worded so that it is true of an exact stored zero too (for example "is below … so the file cannot tell it from zero"), without forking the message below ε_input (D4 ¶1 (i), (iii))?

## Ruled (ENCL orchestrator, 2026-10-13)

Yes. Word the one sentence so it is true of every reading that reaches it, an exact stored zero included. The sentence should say what the file cannot do, which is tell this size from zero at its declared ε_in, not claim the file left the size unstated. Keep it one sentence below ε_in, with no fork by class (D4 ¶1 (i), (iii)). Pin the cone-apex row and an in-band nonzero row whole.
