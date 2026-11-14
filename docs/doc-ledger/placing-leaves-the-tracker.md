# PLACING leaves the tracker — 2026-11-14

PLACING covered how the viewer writes a placement: the mate tool, and the gestures that re-point, move or delete one. It was opened on 2026-11-01 by CHROME's cut, and it closed on 2026-11-14 at Ev's request in chat: "close placing and rehome the remaining handful of items to another program".

**What it delivered**, through Ev's `[ev]` PRs and the units built on them:
- the placement editor, which commits a placement's whole bundle;
- every placement is a copy (6580);
- a placement's target space as the editor's first row (6600);
- placements read their body through a source variable (VARIABLES VR4; 6672, built in 6856);
- the re-point offer: a fact at any depth, which nothing declines (6915, 6917);
- the editor's held-draft preview (6916);
- one check and one verb for every re-point button (6948).

The narrative and every adjudication are in its `log.md`, recoverable below.

**Residue, re-homed to `work/doors/`** by `git mv`, with ids and bodies kept. DOORS owns the authoring doors and the takeover on the same files:
- `band-refusal-still-badges-every-row` (P1, deferred)
- `a-family-placed-from-a-previewed-copy-is-not-carried` (P3)
- `a-re-point-button-checks-the-first-edit-not-the-takeovers-rebinds` (P3)
- `a-refusal-naming-a-value-slot-drops-its-axis` (P3; it needs a vocabulary ruling)
- `a-pick-opening-the-edit-door-starts-a-mate-row` (P3; a design fork)
- `a-rows-zero-slides-across-a-read-refuse-once-an-edit-turns-the-read-into-their-plane` (P3; a design fork)

There was no `plan.md` and so no `## Exit criteria`, so it closes without an exit walk.

Recover SHA `54edaddb03bb7127bc510f9e151f208673dbeb13`.

    git show 54edaddb03bb7127bc510f9e151f208673dbeb13:work/placing/<FILE>
