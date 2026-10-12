# CONTACT leaves the tracker — 2026-10-09

CONTACT covered touches, overlaps and declared contacts: the ordinary
solids the boolean lane will not combine. It opened 2026-09-20 in
REACH's priority-seam cut and closed 2026-10-09. Ev asked in chat on
2026-09-29 for it to finish its two P0s, close, and move its remaining
rows to two or three successor programs. Both P0 tracks are done.
CONTACT-12 merged (PR 4372). CONTACT-13, the census's reading of every
declared meeting, is parked on the D10 hold (`work/intent/plan.md`).

Its plan set no `## Exit criteria`, so it closes without an exit walk
(`work/README.md`, "A closed program's directory is deleted").

## Where the rows went

CONTACT's live rows came to 99.5 budget points against 30 once the six
unpriced rows were priced. Four tracks of 30 is the least that holds
that, so three successors needed six rows re-homed by charter. The
rest were cut on the priority seam (`work/README.md`, Track size).
Every row moved by `git mv`, ids and bodies unchanged.

| program | band | pri | rows | points | what |
|---|---|---|---|---|---|
| RESTREAD | 11800–11899 | P0 | 11 | 29.5 | the census's five P0 near-band reads (crossing and edge-on-face lanes), the cross-solid curved pairs, the P1 point and seam readings |
| INSIDE | 11900–11999 | P1 | 11 | 30 | containment and point-in-solid: the decision an escalation carries, refused walls and charts; the P1 tail and every P2 |
| SECTOR | 12000–12099 | P3 | 15 | 28.5 | the P3 tail: sector-arm levers, self-touches and wedge joins, torn records, gate readers |
| CONTACTHOLD | 12100–12199 | P0 | 21 | 0 | the parked rows (20 on D10's stage-4 units, one on the plane × torus cone arms); opens `blocked` |
| REACHTAIL (live) | — | P4 | +4 | +5.5 | two P4 rows, one P4 spelling family, one P3 crossing decided twice |
| GAUGE (live) | — | P3 | +2 | +3.5 | a gate pair no row reaches; a source guard that misses a point-free call |

Six rows were priced at the cut:
- `census-containment-escalation-drops-its-decision`: P2, M.
- `point-in-solid-escalation-carries-no-decision`: P2, H.
- `contact-gate-readers-drop-the-arm-verdict-or-mint-invalid`: P3, M. Its `contact_verify` half is under the hold, which SECTOR's plan says.
- `full-turn-question-has-three-spellings`: P4, E.
- `edge-face-crossing-cut-and-pass-five-decide-one-crossing-twice`: P3, M.
- `census-edge-pass-reads-no-line-conic-crossing`: P3, M. It is parked on `declared-pairs-retire`, because its symptom is a declared seat no declaration can answer.

Live rows across the tracker that cited a closed CONTACT row in `refs`
now cite the PR that closed it. Two cited
`volume-door-reads-a-tiny-valid-boolean-result-wrong`, now PR 3977.
One cited `ray-wall-and-cone-near-root-cancels-over-a-small-lead`, now
PR 3755.

Path citations in code, docs and items were re-pointed:
- a moved row to its new directory;
- a closed row to its id and closing PR;
- the closed duplicate `a-notched-full-turn-wall-has-no-ray-trim` to the
  row that carries it.

Logs, `docs/DUAL-REVIEW-LOG.md` and the doc-ledger keep their paths,
which resolve at the SHA below.

Band **7100–7199** closes with CONTACT, UNSPENT: no A/B ordinal was
drawn from it (its dual reviews are DR rows in
`docs/DUAL-REVIEW-LOG.md`).

Recover SHA `2be0ae41440dd3b37954101d68b77b7385777081`.

    git show 2be0ae41440dd3b37954101d68b77b7385777081:work/contact/<FILE>

## What it left

- One local material-cone analysis reads every touch kind the census
  sees (CONTACT-1, PR 3253). It is read in metres over the touch
  point's finite star, and a Rest is built only by that check
  (CONTACT-7, PR 3383).
- The Planar join lane measures a conic edge against its section
  plane, so the axis-coincident box lap stops reaching an unreachable
  invariant (CONTACT-2, PR 3250).
- A curved wall's chart trim decides a non-iso wall exactly or refuses
  (CONTACT-3, PR 3331). Containment reads every loop on its carriers
  (CONTACT-4, PR 3345).
- The census backstop clears a meeting pair only through the touch
  analysis (CONTACT-5, PR 3341). `point_in_solid` stops answering a
  false Out inside a tilted-cut cylinder cavity (CONTACT-6, PR 3357).
- The merge deletes a seam edge left dangling inside a merged face at
  any angle, on Ev's ruling on PR 3350 (CONTACT-8, PR 3377).
- The boolean and splitting side codes decide in metres where a levered
  chord-direction Zero was a wrong verdict (CONTACT-9, PR 3415). The
  declared-only ruling is PR 3422.
- The containment refusals carry their decision (`ContainDecision`,
  `LoopDecision`) to every renderer (CONTACT-10, PR 4363).
- The torus chart-box check compares areas, so an L-shaped torus face
  refuses rather than trimming by its box (CONTACT-11, PR 4368).
- The edge-on-face overlap lane cuts at boundary crossings, and the
  touch analysis reads every cell (CONTACT-12, PR 4372).
