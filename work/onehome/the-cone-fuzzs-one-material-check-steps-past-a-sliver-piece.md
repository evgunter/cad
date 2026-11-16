---
id: the-cone-fuzzs-one-material-check-steps-past-a-sliver-piece
kind: issue
title: Main red at random: the cone fuzz's one-material check steps 1e-4 past a sliver piece and calls a sound cone a fixture defect
status: closed
opened: 2026-11-16
priority: P1
cost: E
closed: 2026-11-16
branch: onehome/cone-fuzz-material
---

Found by ONEHOME's local topo run on `onehome/strut-followup` (PR 7020),
2026-11-16; not that PR's (its diff touches nothing under
`crates/topo/src/link/`, and the seed reproduces without it).

**Measured.** `topo link::cone_fuzz::the_consumers_paths_never_contradict_the_exact_oracle`
draws a fresh seed each run, and failed about one run in thirty locally.
`CAD_FUZZ_SEED=0x255c5ef70a16a1d0 CAD_FUZZ_EFFORT=1` reads, at
`crates/topo/src/link/cone_fuzz.rs:2096`:

> ε 1e-12, cone 2, split: the drawn cone bounds no one material (a fixture defect)

So main can go red at random on any run that draws such a cone.

**Generator, check or kernel: read from the drawn geometry.** The kernel
is not in this reading: `one_material` asks the exact oracle
(`cone_oracle`) alone. Dumped, the cone's 17 sectors read In behind and Out
in front of every piece at a `1e-6` step. At the fixed `1e-4` step all but
one do too; the exception is face 3's first piece, a sliver `6.7e-4` rad
wide, whose point `1e-4` behind its middle reads Out. The material behind
a sliver that thin is thinner than `1e-4`, so the step crossed the
neighbouring face. The cone bounds one material; the check's fixed step
read it wrongly. A defect of the fixture's check, neither the generator's
nor the kernel's.

## Closed (2026-11-16): the step is the piece's own scale

`one_material` steps `MATERIAL_STEP = 1e-3` of the piece's chord on the
unit sphere either side of its middle, and no more than `1e-4`, so every
piece wider than `0.1` rad reads as before. The found cone is pinned whole
as `a_sliver_piece_bounds_one_material`, red against the fixed `1e-4` step
and against a step of the whole chord. The seed now runs its probes through
and passes.

