# PATHS leaves the tracker — 2026-10-21

PATHS covered the profile lattice (`crates/profile`): the authoring algebra's carriers, closers and joints, the lift onto a sketch plane, and validation's pair pass. It opened 2026-09-16 at S-BOOL's exit, as S-BOOL's successor for the eight lattice rows. It closed 2026-10-21 on the exit walk Ev ratified on PR #5561 (`docs/PATHS-EXIT-WALK.md`, recoverable below).

**What the walk found.** Four of the plan's six exit clauses were met: one carrier spelling (#3504, #3527, #3774, #5403), every closer declaring its arrival (#5384, Ev's #5374), the sketch plane's witness and one lift rule (#5068, Ev's #3775 and ruling A), and the typed over-full angle (#3774, #5115). The lift comparator half of the third clause was met (#3780). `carriers_are_identical` was not: it still decides `path_carrier_identity`, and its row is open on ROUND. "Every unit merged on its own green hosted head" was not met: #5068 merged over three inherited `1e-12` reds. Beyond the charter, PATHS built most of the #3218 lowering (units 1, 2, 3, 5a, 5b, the shared `Arc2` and the wrap edge), closed the validation P0s its sweeps found (#4259, #4264, #4276, #4290, #5032, #5377), built `arc_onto` and the arc read-back tripwire, and fixed the near-full-turn and hairpin cancellations.

**Where its 49 live rows went.** All moved with their ids, each with a dated "Re-homed at PATHS's close" section:
- **Four tracks opened at this close: 48 rows,** cut along their priority seam so that none opened over budget (`work/README.md`, Track size). Each opened `ready` with no orchestrator.
  - **PROFILE** (10 rows, 30/30) holds the goal: the #3218 lowering epic (parked on unit 4, which is parked on CLEAVE's P0 `a-boss-over-a-hole-tangent-inside-its-rim-at-its-vertex-unites-as-two-shells`), the check-9 arm-5 and ray-crossing dedupes, the fillet-refusal step, and the validation P2s.
  - **PROFILEREST** (13 rows, 29.5/30) holds the P2 remainder and validation's P3 reads. It includes `arc-fillet-corner-circle-pair-cancels-at-unequal-radii`, which waits on Ev's ruling on LB4.
  - **PROFILETAIL** (14 rows, 27.5/30) holds the path door's and the lift's P3s, and the two P3 rows PR 5552's review filed on PATHS on the day it closed (`circle-side-reads-a-far-segment-ends-rim-uncharged`, `computed-points-read-uncharged-where-err-means-no-contact`).
  - **PROFILEPROSE** (8 rows, 12.5/30) holds the refusal-text P3/P4s. It includes `arc-carrier-refusal-register-misses-two-format-arms`, whose design half waits on a ratification.
- **INLANE**: `a-path-sweeps-station-turn-at-interval-widens-its-walls-past-the-band-at-the-tightest-eps`. Its site is `crates/sweep/src/skin.rs`, and the question is the lane scalar's.

PATHS's A/B band 5000–5099 closes with it.

**Residue the sweep could not re-home, recorded so it is not lost:**
- **`refs:` entries that named closed PATHS rows were dropped** from eight rows: two in PROFILE, one in PROFILETAIL, and one each in RESTFRONT, REFS, SSIMARCH, RULES and CLEAVE.
- **Citations of the moved rows were rewritten** to their new paths, in code comments, docs and other programs' rows. Logs were left as written.
- **Citations of closed PATHS rows in shipped docs now dangle.** They are added to `work/issues/dead-work-citations-from-shipped-code-and-docs.md`.

Recover with `git show 4abacc6fba:work/paths/<file>`, using `plan.md`, `log.md`, `program.md` or an item id, and `git show 4abacc6fba:docs/PATHS-EXIT-WALK.md`.
