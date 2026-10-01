---
id: certification-gains-a-sqrt-door
kind: unit
title: Certification gains a sqrt door (C9 admits √, Ev #3517); the three private outward-rounded roots in certification files retire into it
status: open
priority: P2
cost: M
refs: [torus-meters-blocker-is-the-arithmetic-or-c9s-root-rule, 3517]
opened: 2026-09-30
---

Filed by CURVED after Ev's ruling on PR #3517 (2026-10-01): clause C9 of
`crates/geom-brep/README.md` now reads that certification arithmetic is
IEEE-754's correctly rounded operations — `±`, `×`, `÷`, integer powers
and `√` — and no transcendental. The clause changed; the code has not.

## The unit

1. `Certification::sqrt` in `crates/geom-core/src/interval/certification.rs`,
   delegating to the backend as `powi` does: a refused or
   possibly-negative radicand refuses through the decoration; a radicand
   known non-negative by an outside fact is clamped first with the
   existing `clamped_to` door, never by a permissive root. Door rows:
   refused in, straddling radicand refuses, an exactly representable
   root stays tight.
2. Retire the three private spellings into it — each currently asks
   `is_certified()` by hand before reading an endpoint, the hazard the
   doors exist to remove: `props/quad.rs:sqrt_enclosure`;
   `offset_meters.rs:sqrt_up`/`sqrt_down` (≈25 call sites across
   `quad`, `offset_meters`, `offset_fit`; note `sqrt_up(x)` returns `x`
   for `x ≤ 0`, so a negative radicand yields a negative "upper bound on
   √" — check every caller's radicand when retiring it); and the inline
   `hi().sqrt().next_up()` roots in `crates/mesh/src/chords.rs`. Bounds
   may tighten by an ulp where a root is representable — re-baseline and
   say what moved.
3. `scripts/gates/certification-doors.sh`'s "what to write instead"
   sentence names the door.

These files belong to several programs (PROPS/QUAD, ENCL, TESS); the
unit edits them by announced seam. The torus arms are NOT part of this
unit: `work/chart/torus-certificate-runs-root-free-through-its-quartic.md`
needs no root, and may use the door for its box enclosures once it
exists (the factored residual is the tighter spelling there).
