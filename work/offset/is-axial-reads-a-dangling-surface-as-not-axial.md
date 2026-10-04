---
id: is-axial-reads-a-dangling-surface-as-not-axial
kind: issue
title: the axis gate reads a dangling surface key as a definite 'not axial' (is_axial_in) or skips it (axial_frame)
status: open
opened: 2026-10-04
---


Found by PR 4014's fix pass, which made `is_axial_in`'s `axial_frame`
call propagate everything but `TogetherAxialUnsupported` (a torn point
used to read as `Ok(false)`). Two siblings of that swallow remain in
`crates/topo/src/offset_axial.rs`, both on a face in scope whose
surface key does not resolve — a torn body, not a geometric verdict:

- `is_axial_in`: `let Some(surface) = body.get_surface(f.surface) else
  { return Ok(false) };`. The gate answers a definite "not axial", so
  `shell`'s `offset_door` silently takes the per-chart branch — the
  "silent branch choice" `is_axial`'s own `# Errors` paragraph rules
  out (D4 ¶3, D2 addendum "Silent discard is never an answer").
- `axial_frame`: `let Some(surface) = body.get_surface(f.surface) else
  { continue };` in the seed walk. A torn first curved face is skipped
  and the axis seeds from the next one, or the scope reads as
  all-planar (`TogetherAxialUnsupported`, hence `Ok(false)` above).

The repair is the same shape as the point read's: refuse
`ReplaceFaceError::Corrupt` (or whatever 4006's split names a torn
input) at both sites. Witness: `offset_axial`'s test module now has a
`quarter_wedge()` builder; repoint one wall at a removed surface key
and assert `is_axial` is `Err`.
