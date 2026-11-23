# SECTTAIL, SECTRUNG and SECTSITE leave the tracker — 2026-11-02

Three tracks were cut from SECT, the section-first join:

- SECTTAIL, the join's P2/P3 remainder, on its priority seam on
  2026-10-25, in a refold approved in chat;
- SECTRUNG, the C5 dispatch and rung-3 residue behind the section
  object, the same way on 2026-10-28 at Ev's request, because SECTTAIL
  was too full to take those rows;
- SECTSITE, the one-site and on-carrier sections on curved faces (two
  P1 rows), on 2026-10-28 through SECTARMS, at Ev's request.

None was ever dispatched.

On 2026-11-02 the track budget came to count only the dispatchable P0,
P1 and unbanded rows (Ev, in chat). SECT, SECTTAIL, SECTRUNG and
SECTSITE then measured 3, 3, 8 and 16 points, 30 against 30 together.
Every row is read on the section object SECT built, so the tracker sweep
merged the three back into SECT. Their rows (9, 12 and 2) moved to
`work/sect/` by `git mv`, ids and bodies unchanged, each with a dated
Moved note. SECT's plan restates their order:

- read a row against SECT's and SECTARMS' progress before building it,
  since a pair moved onto the section value can close it by
  construction;
- then the closed edge in a curved face;
- then the C5 residue, once D10 releases its touching poses.

SECTARMS, active, stays its own track.

None of their plans set `## Exit criteria`, so they close without an
exit walk.

Recover SHA `82080f7c76a1e08644f69622cb64aae2a2a9cea6`.

    git show 82080f7c76a1e08644f69622cb64aae2a2a9cea6:work/secttail/<FILE>
    git show 82080f7c76a1e08644f69622cb64aae2a2a9cea6:work/sectrung/<FILE>
    git show 82080f7c76a1e08644f69622cb64aae2a2a9cea6:work/sectsite/<FILE>
