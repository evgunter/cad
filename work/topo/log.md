# TOPO log

Newest entries at the bottom; the tail is the program's live status.
Plan: `work/topo/plan.md`. A/B band 2700–2799
(`docs/MODEL-AB-LOG.md` owns every live experiment number).

## Opened (2026-09-04)

Opened on Ev's direction (in-chat, 2026-09-04) after a re-read of
`work/issues/` and the code-quality K–X partition against every open
program's `paths`. The measurement that produced this program:

- **55 of the 102 `.rs` files under `crates/topo/src/` are in no open
  program's `paths`** (re-derived 2026-09-05; the opening commit said
  47, off by the two subdirectories a directory-grouped scan dropped) —
  the largest unowned kernel territory in the tree, and the one
  `docs/WORK-TRACKS-2026-09.md` named ("37 `crates/topo/src` files")
  and then cut eleven tracks around without closing.
- **code-quality Track P has never had a lane.** Fourteen open rows,
  fence `euler*.rs`/`validate.rs`/`live.rs`/`seqgen.rs`/`merge_faces.rs`
  and the review-and-fixture readers, listed as claimed by code-quality
  itself; `git branch -r` shows `smell/k-*`, `smell/x-*` and
  `smell/t-*` lanes and no `smell/p-*` ever. The rows were waiting on
  an owner, not on a ruling.
- Two `work/issues/` files name this ground as unowned **in their own
  `## Home` sections** — `validate-tier3-curved-boundary-containment`
  ("`crates/topo/src/validate.rs` is in no open program's `paths`") and
  `no-public-census-or-genus-query` ("beside `euler.rs`/`fixtures.rs`/
  `seqgen.rs`, which no open program's `paths` covers").

Seventeen items at opening: three issues, re-homed by header edit and
`git mv` with ids unchanged, and Track P's fourteen rows, which keep
their ids and their `track: P` letter per `work/code-quality/program.md`
("a row leaves the moment a program claims it, by moving into that
program's directory keeping its id, `track:` letter and body").

**The fence is an enumeration, not a glob**, and the reason is on file:
`crates/topo/src/*` would double-claim five programs' ground, and
`scripts/work.py territory` is blind to a double claim
(`work/meta/territory-cannot-see-a-path-two-programs-both-claim`, the
FIX/SHELL collision on `transform.rs`). The 35 remaining `topo/src`
files are recorded as unowned-and-not-finished rather than swept in.

No unit is cut and no branch exists yet. The first dispatch is `S331`
(the vacuous green through `validate_pcurves`) and claims its ordinal
from the band above, recorded in `docs/MODEL-AB-LOG.md`.

**One seam to announce before it is crossed**: `face-kind-read-has-two-homes`
is on this slate because `readback.rs` is this program's, but its other
door is `query.rs`, which is **SEAT's**. The ruling can be made here;
the edit on SEAT's side is announced on SEAT's board and agreed there
before it lands. Nothing has been taken from SEAT.
