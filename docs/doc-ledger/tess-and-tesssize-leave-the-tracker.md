# TESS and TESSSIZE leave the tracker — 2026-11-14

TESS (the tessellation kernel's reach: valid bodies the mesh refuses to
draw) opened on 2026-09-16 as S-MESH's successor for `crates/mesh` and
`topo::coherence`, and on 2026-10-28 cut itself along its semantic
seams into TESS, TESSLIE, TESSWALK, TESSSIZE, TESSLATTICE and TESSWORD.
TESSSIZE (the lanes' sizing: which density each lattice emits and the
theorem behind it) took TESSLATTICE back in on 2026-11-02
(`docs/doc-ledger/tesslattice-leaves-the-tracker.md`).

On 2026-11-14 the two measured 12 and 14 budget points, 26 against 30
together, and merged into a new program, TESSLANE (`work/tesslane/`),
rather than one into the other, so that TESS's 793-line log, most of it
other programs' seam notes, is left behind (Ev, in chat). Reach and
sizing are one track: a new arm of a lane owes its sizing theorem in
the same change.

All 56 item files, live and closed, moved to `work/tesslane/` by
`git mv` with ids and bodies unchanged, each with a dated note. Path
citations of `work/tess/` and `work/tesssize/` outside logs and
ledger notes now read `work/tesslane/`. Their `ab_band`s (TESS
5100–5199, TESSSIZE 16100–16199) leave with their headers.

Neither plan set `## Exit criteria`, so they close without an exit
walk.

What a reader of the old logs would come for:

- TESS's log holds every seam note other programs left on mesh ground
  from 2026-09 to 2026-10 (TOPO, ENCL, NURBS build C, ISO, QUAD, PRED),
  including the lattice metric (PR 5531), the column snap's retirement
  (PR 5551), the row raise (PR 5570) and the per-lane sizing (PR 5717).
- Its 2026-10-28 entries record the semantic-seam split, the sphere
  arm's dual review (PR 5920) and the degree-1 loft's slab split
  (PR 5923, with Ev's rulings on `[ev]` PR 5913).

Recover SHA `d6e1eb1f0717399fbff4b3daf8227cadd04f55b5`.

    git show d6e1eb1f0717399fbff4b3daf8227cadd04f55b5:work/tess/<FILE>
    git show d6e1eb1f0717399fbff4b3daf8227cadd04f55b5:work/tesssize/<FILE>
