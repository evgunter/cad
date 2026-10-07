# Review of PR 4207: "join: a pinch's cone vertices share the pierce point's key"

Frozen head `04bdeebe4`, base `3e9d1a96`. Full review lane; every build read through `outcome`
and tessellated with `check_mesh`, release, separate target dirs for main and head. Probes are in `REVIEW-4207-probes/`: PR 4139 r1's
example, extended with the sets `dbl3`, `two`, `touch` and `r2p`, plus the env-gated mutants.

**Verdict: APPROVE-WITH-FIXES** — MAJOR 0 · MINOR 3 · NOTE 5.
All five claims hold. The fixes are prose and filing, not code.

## Claims

**1. The class is decided by descent, not geometry: holds** (executed and inspected).
- `point_classes` (zip.rs:248-282) unions only the point keys named by `SeamCorrespondence`
  pairs. No position is read.
- The zips certify those pairs **bitwise** coincident (zip.rs:26-31, `fuse_by_joint`).
- I instrumented `share_points` (`R4_DIST`) to print the spread of stored points over each
  rebound class. Over 190 rebinds (dbl 9, r2p 180, two 1), every spread was **0.0**. 162 of
  the classes held 2 keys and 18 held 3.
- `two` probe: two pinch points on one cube face (1 728 lines). No key holds vertices at two
  places (`x0` on every line), so no classes chained across points.
- `Body::share_point` (body.rs:702) writes no coordinate. It also **checks none**, so it would
  rebind onto a key whose point differs (N1).
- The one position-decided input upstream is `weld_pinches`' `one_vertex` band test
  (finish.rs:570). It predates this PR, and no weld fired a non-zero spread here.

**2. No wrong body ships: holds** (executed).
- The exact arrangement counter (PR 4139 r1's) was run on every set. **0** SOUND lines show
  a cone mismatch in dbl, two, cyl, dbl3, or my exact replay of r2's pinched operand.
- The 75 r2p lines that move are **all** vertices = cones at the pinch: 41 lines with 2/2 and
  34 with 3/3. Each sits on one key and meshes.
- The 9 dbl lines go from `c2v1!CONEMISMATCH` to `c2v2`. Two mesh panics and two
  `Triangulation` refusals are gone.
- `two`: 1 line goes BAD → SOUND at `c2v2, c1v1`.
- `dbl3`: an operand of three cones on **3 keys**, 72 lines, byte-identical to main. 40 lines
  stay BAD on `UndeclaredContact VertexVertex` (share_points does not reach them, M2), 24
  refuse `JoinDesync`, and 8 are SOUND with one cone.
- Curved faces (`cyl`, 3 600 lines): byte-identical. **Zero** rebinds fire, so this PR's code
  never runs on a curved pose. That evidence is vacuous for curved faces, not positive.

**3. Nothing else moved: holds** (executed). Main vs head, byte-identical outside the claimed
lines:

`rc_wide` (4 shards) 40 344; `pierce_runs_battery` 4 542; `pinch_runs_battery` 3 030; r2 tri-cone
774; r1 `multi` 10 080, `pair` 9 000, `x4` 34 560, `stair3` 2 160, `islnotch` 4 320, `nt` 7 200,
`cyl` 3 600; `dbl3` 72; `touch` 84.

- `dbl`: exactly the 9 claimed lines move.
- r2 pinched operand: exactly 75 lines move, all BAD → SOUND.
- SOUND → refusal: 0.
- editor-core on head under `cargo test`: 2 465 pass. The 2 failures are
  `msolve8_levered_clash::c4_*`, which panic with "first commit in this process:
  AlreadyInitialized". They pass when run alone, so they need a process per test, as the
  repo's nextest gives. Not this PR.

**4. The pin can see a regression: holds, with one blind spot** (executed; patch in probes).

| mutant | the row | other effects |
|---|---|---|
| `share_points` skipped | **red** (`Ltop asym 2296 fib21 xy U` BAD) | dbl and r2p revert to main line for line |
| `point_classes` drops entries with several correspondents | **red** | 14 dbl lines move; r2p `i=22 psi=1.1 cp U` goes SOUND → BAD |
| largest key instead of smallest | green | identical to head everywhere; harmless (N3) |
| `point_classes` keeps only each vertex's first correspondent | green | identical to head on dbl and r2p (N3) |

**5. The 120 still-BAD lines: holds per line** (executed, my exact r2p replay, tier 3′ via
`R1_OPERR`).
- Main: 195 BAD lines. Head: 120 BAD lines.
- On head each of the 120 fails **only** on `StaleContactDeclaration { VertexOnFace }`: 64
  lines with one, 56 with two.
- On main the same 120 lines carry the **same stale count, line for line**. Main's other 73
  lines on that set add `VertexVertex` (37 ×1, 36 ×2). Main's total `VertexVertex` is 236,
  as claimed.
- The class is filed: `work/fuse/a-boolean-result-ships-contact-records-its-geometry-no-longer-confirms.md`,
  parked on D10.

## Findings

- **M1 (MINOR, sure, inspection).** The row's doc, join_pierce_runs_sweep.rs:1829-1831, says
  "one cone's vertex descends from the pinched operand's own two vertices… the other's from
  the cube's pierce copies". That is the cause the PR body and the row file corrected: both
  surviving keys are the operand's, linked through the cube's third key. `share_points`' doc
  (zip.rs:289-292) also lists "a pierce's ring copies… in the other operand's arena" as a
  surviving-key source. No measured line has one.
- **M2 (MINOR, sure, executed).** Residue that is neither filed nor scheduled (Q6):
  - Operands pinched on three keys (`dbl3`) still refuse `VertexVertex` on 40 lines.
  - A point-touch union (`touch`, both orders, 28 lines) ships its pinch on **two keys**,
    SOUND only through its returned contact. The ruling's "several vertices on one key" so
    holds only where seams link the keys. Any such result reused as an operand loses the
    contact: the `dbl` shape.
- **M3 (MINOR, sure, executed).** The PR body says main has "140
  `StaleContactDeclaration { VertexOnFace }`" on r2p. I count **176**, all on the same 120
  lines. The per-line conclusion stands; the figure does not.
- **N1 (NOTE, sure, inspection).** `share_point` (body.rs:693-701) rests on "the caller
  vouches". Nothing checks it, and a wrong caller moves a vertex silently. Measured spread is
  0 today.
- **N2 (NOTE, sure, executed).** `pierce_runs_battery` and `cyl` fire no rebinds, so
  "byte-identical" there exercises nothing new.
- **N3 (NOTE, likely, executed).** The largest-key and first-correspondent mutants survive.
  The first is behaviourally neutral. The second means no row holds a vertex whose non-first
  correspondent alone links a class.
- **N4 (NOTE, sure, executed).** The row runs in 0.1 s; no CI cost.
- **N5 (NOTE, sure, inspection).** The territory and D10 hold are respected. The class is
  read from null-pair records, and `edge_join.rs` and the declared paths are untouched.

## Style

| Q | finding | confidence |
|---|---|---|
| Q1 | zip.rs now holds **two** hand-rolled union-finds: PointKey at :258-275 and VertexKey in `split_cones` at :411-423. A third is `DeclaredSurfaceEq` (merge_faces.rs:1408-1432). Same find loop, three spellings, no shared home. | sure |
| Q1/Q4 | `move_vertices`' doc (body.rs:663) still says "**The one door that moves vertices**". `share_point` changes which point a vertex sits on, a second door; it is position-neutral only by the caller's vouch. Neither doc names the other as its sibling. | likely |
| Q2 | `share_point`'s "the caller vouches, by its own records" (body.rs:697-701) and `share_points`' "The classes are disjoint, so one index serves them all" (zip.rs:304) assert invariants nothing enforces. The second holds by construction of `point_classes`; the first holds only by measurement. | sure |
| Q3 | `point_key_finding` (common/pinch_cones.rs:322) passes vacuously with 0 or 1 vertex at the point. The row stays honest only because `cone_finding` runs first. | likely |
| Q5 | zip.rs's module header (:32-37) describes the pinch as split per cone "on its own point key", and never mentions that classes are re-keyed after the zips. A reader of the header cannot find `share_points`. | likely |
| Q7 | Classes are read before `split_cones` and the zips, then applied after both have mutated the body. That couples correctness to "keys survive the zips". I would read the classes off the zips' own fusions (`desc`) instead. Taste. | unsure |
| Q7 | `share_points`' `ZipCorrespondence` "a pinch vertex no longer resolves" (zip.rs:319-322) is unreachable: the index was built from live vertices a few lines up. A typed error for an impossible path. | likely |
| Q8 | Read zip.rs's header, the new region, `split_cones`, `fuse_by_joint` and `Fusions` in full; the rest (`align`, `sigma`, the `zip_seam` body) skimmed. Nothing accumulated beyond the Q1 and Q5 items. | likely |
| Q4, Q6 | Q4: the `edge_join` guard premise at edge_join.rs:20 still holds. Q6: M2 is the unscheduled residue. | sure |

Questions exercised: Q1, Q2, Q3, Q4, Q5, Q6, Q7, Q8 (Q8 partially, as above).

REVIEW COMPLETE
