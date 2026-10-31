# CLEAR — the plan

the E7 clearance engine: where it lives, what it composes with, and what its refusals say

Opened 2026-09-20 by SHELL's priority-seam cut (`work/README.md`,
Track size). Its P2–P4 rows went to CLEARTAIL at the 2026-10-27 cut.

## Order

1. **Wait for INTENT's E (#5811).** It is the one open PR that edits
   `clearance.rs`. It reads the second selection's body into the
   first's frame (`framed_of`, `Framed::read_in`), splits `windows_of`
   around that, and reworks `verify_witness`. Those are the sites
   `SHELL-3` moves and the refusal rows edit. No CLEAR lane touches
   `clearance.rs` until E merges.
2. **Designers, now, in parallel with E.** These rows are design
   questions and none of them edits code:
   - `symbolic-tier-and-clearance-engine`. One of its two ways out is
     "make the engine generic in its lane scalar", and that decides
     what `SHELL-3` moves: an engine at `Interval`, or a generic one.
     It is weighed before `SHELL-3`'s spec is final.
   - `self-intersection-drops-every-vertex-sharing-face-pair-globally`
     (P0). It is a prerequisite of CARVE's loft certificate.
   - `min-separation-tightening-crosses-the-drive`. Its ruling (may a
     lane split be invisible to the census) is Ev's.
3. **`SHELL-3`** (P0), after E. The body-level half moves into `topo`
   behind `interval`. Framing stays in `editor-core`'s outer half, so
   the engine in `topo` takes bodies already read into one frame.
4. **The refusal batch.** One unit carries the four P1 refusal rows:
   `clearance-refusal-names-one-face-twice-across-bodies`,
   `clearance-reports-a-no-bodies-payload-as-a-bad-body-index`,
   `named-face-scope-refuses-not-a-face-without-naming-what-it-found`
   and `clearance-window-selection-asks-how-many-before-what`. It runs
   on the moved code and can run in parallel with step 5.
5. **On the moved engine:**
   - `window-of-refuses-an-untrimmed-iso-bounded-nurbs-patch` (P0);
   - the self-intersection row's build, as its designers land it;
   - then `SHELL-4`.

## Review posture

The repo-wide tiers (`memories/orchestration-model.md`). `SHELL-3` is
a move with no behaviour change: a single full review, with the
M10-5/M10-6 suites and goldens as the differential. The
self-intersection build is dual: tricky logic, and the loft
certificate rests on it.
