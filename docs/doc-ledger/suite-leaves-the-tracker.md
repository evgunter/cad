# SUITE leaves the tracker — 2026-10-26

SUITE (suites, fixtures and the helpers they copy) opened on 2026-09-11
in the tracker cut of that day. It closed its six units on 2026-09-15:
`S391`, `D403`, the editor-core resolver stubs, `D114`, `S52` with its
passenger, and `S392`. Its exit walk, `docs/SUITE-EXIT-WALK.md`, was
ratified when Ev merged its `[ev]` PR 2685 on 2026-09-15. That merge
also opened S-DUP as SUITE's successor.

The sweep that should have followed never ran. On 2026-09-20 FIX's
design-free cut re-homed three rows onto the open directory:
`band-helper-duplicated-across-suites`,
`prune-report-rows-are-nine-copies-of-one-assertion` and
`threaded-seam-wait-loops-are-hand-copied-across-four-test-files`.
`needs_ev: true` stayed on `program.md` for the walk Ev had already
merged.

On 2026-10-26 the tracker sweep folded SUITE into FIXTURE, the program
for the same class on the same ground (test-side copies, no paths
claimed). The three live rows moved to `work/fixture/` with their ids
kept. The eight closed rows, `program.md`, `plan.md`, `log.md` and the
walk were deleted. FIXTURE's plan carries forward the two SUITE exit
criteria that still bind its units (a duplication closes by a shared
home and not by a deletion; a sweep states what it could not match).
The A/B band 4200–4299 was claimed and never drew an ordinal.

Recover SHA `bb25fa4f0b5777e23a2d9c68c6b3d147bccf1cf2`.

    git show bb25fa4f0b5777e23a2d9c68c6b3d147bccf1cf2:work/suite/<FILE>
    git show bb25fa4f0b5777e23a2d9c68c6b3d147bccf1cf2:docs/SUITE-EXIT-WALK.md
