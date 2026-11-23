# SSI closes into three successors — 2026-10-08

SSI (the plane × NURBS surface intersection: its bounds, its tubes and
the diagnoses that survive a bad one) closed on 2026-10-08 (Ev, in
chat: "if you could wind down and open successor programs for the
rest"). It finished its P0 slate and all but one P1 row; the P1 row
left, `plane-nurbs-certificate-bound-does-not-refine-with-eps`, is
parked on FLUX's enclosure-refinement row and PR 3524.

Its plan has no `## Exit criteria`, so no exit walk is owed. The
done-state of record is the closing entry of `work/ssi/log.md`.

Sweep SHA `7eef48182b0e9b8b19cb4a3b2254e1044f93237f`.

    git show 7eef48182b0e9b8b19cb4a3b2254e1044f93237f:work/ssi/<FILE>

| program | title | closed | done-state of record |
| --- | --- | --- | --- |
| `ssi` | SSI — the plane×NURBS surface intersection | 2026-10-08 | the closing entry of `work/ssi/log.md` |

**Where the residue went.** SSI's thirty open rows moved with `git mv`,
keeping their ids, into three programs opened the same day:

| program | priority | rows | budget points |
| --- | --- | --- | --- |
| `ssiedge` | P2 | 9 (one parked) | 22.5 |
| `ssiarith` | P3 | 11 | 24.5 |
| `ssimarch` | P3 | 10 | 25 |

`ssi-unsupported-certificate-carries-faults-that-are-not-boundaries`
had no priority and took P3 at the move.

**What is only in git now.** The 39 closed rows went with the
directory. Comments in `crates/geom-brep` and a few open rows still cite
some of them by their `work/ssi/` path (for instance
`rational-chart-sup-speed-grows-with-translation`,
`plane-nurbs-ssi-does-not-certify-a-curved-dome` and
`ssi-transversality-at-a-point-is-spelled-three-ways`); those paths
resolve at the SHA above. Rows that named a closed SSI row in `refs`
dropped the id, since lint resolves `refs` against the live tree.
