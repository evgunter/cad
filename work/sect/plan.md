# SECT — the plan

The section-first join: one certified section per germ face pair, read
by both operands and the matcher, into which the per-pair section
frames and per-kind-pair join lanes retire.

Opened 2026-10-10 by JOIN's close (`docs/doc-ledger/join-leaves-the-tracker.md`;
Ev approved the plan in chat on 2026-10-10). Every row here moved from
`work/join/` by `git mv` with its id and body unchanged. Nothing is
dispatched.

## The design

Ratified in `[ev]` PR 4313 (`docs/DESIGN-FORK-LOG.md` row 100). The
shape is "The shape to give" in
`work/sect/cylinder-sphere-germ-pair-has-no-join-lane.md`, and it is not
re-litigated here:

- a chord is a sub-arc of its germ face pair's section at the pair's C5
  rung, computed once; foot parameters replace frames, and pcurves are
  derived (C4's projected image where no closed form exists);
- every island closes by the chord's own curve, so `RingClosure`'s
  section plane and the radical-plane code retire;
- `GermLane` carries a section datum per side, and kind dispatch lives
  only in C5, so torus and cone pairs refuse at C5, not in the join;
- coincident and tangent sections (coaxial, an offset or a tangency
  decided Zero, in band) are decided at the section door. They refuse
  there until INTENT stage 4's PR E (`booleans-glue-on-zero`), and from
  E on they glue and record there.

The props and mesher fitted-boundary arms, and PCERT's spline-carrier
mint (fork row 89), are prerequisites for these poses' rows going
`SOUND`, not for the build.

**Ownership.** SECT owns the curved section-frame and lane rows
outright. No per-pair stopgap lane is built on another track: a stopgap
rebuilds what this design retires. Ev left the choice open on
2026-10-10, and JOIN's orchestrator chose ownership.

**The gate has fired.** The build was ordered after INTENT stage 4's
PR A (`the-join-builds-what-the-rest-zip-builds`, PR 4364), which
rewrote the join's ring re-homing on a wall chart and deleted the REST
zip. PR 4364 merged 2026-10-09T21:06Z.

## Where it stands

Handed back 2026-10-31 (the sitting's P0 and P1 rows are done):

- **Every torus pair has a section and passes the germ mint**: sphere ×
  torus by its closed-form readings and the torus ring path (PRs 5924,
  5974), cylinder × torus and torus × torus by their doors (PR 5972), and
  every pose by its foot on the carried section (PR 6011, fork row 212),
  read at 1e-6 again (PR 6153). Teapot wall 2 stops at SPIRICARM's volume
  door. C5 now holds the ratified rule that an analytic pair decides
  tangency once, on its pose (PR 5975).
- **Reds closed**: the cs near-node march (PR 5961), the seam-region row's
  15× cost (PR 6062), PR 6011's 1e-6 rows (PR 6153). Decided-twice rows
  (PR 5994), the circle crossing (PR 6001), cylinder-face re-homing
  (PR 6047), the march counter (PR 6056).
- **Left**: the P2/P3/P4 rows in the slate. Elsewhere: the nightly
  `full-suite` job outgrowing 180 minutes (GREEN P1), and the per-op
  certificate cost of fitted chord edges (the cost rows that came back from SECTRUNG,
  below).

## The slate

| pri | item | cost | status | title |
|---|---|---|---|---|
| P0 | `main-red-cylinder-sphere-section-runs-out-of-cells-near-the-node-at-1e-6` | M | closed (PR 5961) | The cs section exhausts its cell budget near the node at ε 1e-6 since PR 5872 |
| P0 | `each-torus-pair-decides-tangency-on-one-pose` | H | closed (PR 5924) | One pose per torus pair, the critical gap |
| P0 | `torus-germ-pairs-have-no-section-frame` | H +design | closed (PRs 5972, 5974, 6011) | Torus walls against a sphere, cylinder or torus reach the join with no section (teapot wall 2) |
| P0 | `cylinder-torus-rung-three-arm-is-unretired` | M | closed (PR 5972) | The cylinder × torus door over a Positive gap |
| P0 | `torus-torus-rung-three-arm-is-unretired` | M | closed (PR 5972) | The torus × torus door over a Positive gap |
| P1 | `main-red-five-torus-section-rows-refuse-at-1e-6-since-components-by-foot` | M | dispatched | Five torus rows refuse at ε 1e-6 since PR 6011 (pinned red by GREEN) |
| P1 | `main-red-the-seam-region-door-row-runs-15x-slower-since-the-cylinder-sphere-chord` | M | dispatched | The seam-region door row runs 15× slower since PR 5894; held out of the nightly's 1e-12 row |
| P3 | `a-surface-pairs-march-is-repeated-per-operation-and-per-germ` | M | open (PR 6056 measured) | Cross-operation sharing of a carried section (≤ ~11 %; waits on canonical order) |
| P1 | `a-cylinder-face-re-homes-its-rings-on-a-chart-that-reads-no-spline-chord` | M | closed (PR 6047) | A cylinder face holding a bystander ring re-homes it on a chart that reads no spline chord |
| P0 | `a-general-torus-pose-lists-no-components` | H (decided) | closed (PR 6011) | A section's components are the march's; a site's component is its foot (fork row 212) |
| P1 | `torus-coincidence-is-decided-at-the-carrier-ladder-and-the-torus-pose` | M | closed (PR 5994) | Two tori decided one surface twice (C5: decided once) |
| P1 | `a-tube-circle-tangency-is-read-by-the-gap-and-by-the-glue-doors-tangency-lane` | M | closed (PR 5994) | A tube-circle tangency read twice (C5: decided once) |
| P1 | `circle-pair-crossing-is-spelled-three-times` | E | closed (PR 6001) | "Two circles cross transversally" is decided three times |

## Order

1. **The main red**, now: it is a red nightly, and a lane holds it.
2. **The torus poses' fix pass**, then merge (PR 5924). That opens the
   cylinder × torus and torus × torus door rows.
3. **Teapot wall 2** (`torus-germ-pairs-have-no-section-frame`, the sphere
   × torus half). It carries a general sphere × torus pose through the
   carried lane and island closure of step 1b onto a torus face. Step 1b
   limits spline arcs to cylinder and sphere faces, so the torus face's
   `ring_path` arm with a spline is the work. The row's design flag is
   read again then: the section object answers most of it.
4. **The two door rows**, then the cylinder and torus halves of the torus
   frames row.
5. **The two follow-ups** when their ground is open: the cylinder-face
   ring re-homing and the circle-crossing spelling.

## The rows merged back (2026-11-02)

SECTTAIL (cut 2026-10-25) and SECTRUNG (cut 2026-10-28) held this
track's P2/P3 rows, cut out on the priority seam, and SECTSITE (cut
2026-10-28 through SECTARMS) its two one-site and on-carrier P1 rows.
When the budget came to count only the P0, P1 and unbanded rows a
sitting dispatches (Ev, in chat, 2026-11-02), the four measured 3, 3, 8
and 16 points, 30 against 30 together, and the three merged back by
`git mv`, ids and bodies unchanged
(`docs/doc-ledger/secttail-sectrung-and-sectsite-merge-back.md`).

**SECTSITE's rows**, in order (the C5 residue re-banded P0 on 2026-11-03):

| pri | item | cost | title |
|---|---|---|---|
| P1 | `a-closed-edge-lying-in-a-cone-cylinder-or-torus-face-has-no-join-arm` | H | A closed edge lying in a cone, cylinder or torus face has no join arm |
| P0 | `c5-plane-torus-cone-cylinder-arms` | X (16) +design | The C5 plane × torus residue: the general rung (lily wall 8's stop) and U-H1 |

1. **The closed edge**, read on the section value's components. A conic
   lying in a curved face is a section component whose carrier is an
   edge of one operand.
2. **The C5 residue**, first now that it is P0 (2026-11-03): its general
   plane × torus rung is lily wall 8's stop
   (`work/issues/curved-demo-joins-build.md`). The D10 hold on its
   touching poses (U-H1) lifted when INTENT closed on 2026-10-30
   (#6103). Its design flag is read again at dispatch.

**The cone's rung-3 rows** (filed and re-banded P0 on 2026-11-03):

| pri | item | cost | title |
|---|---|---|---|
| P0 | `the-cone-has-no-implicit-enclosure-or-metres-conversion` | H +design | The cone has no implicit enclosure or metres conversion: every rung-3 pair with a cone refuses |
| P0 | `cone-torus-rung-three-arm-waits-on-the-cone` | M, parked on the row above | Cone × torus at rung 3 (lily wall 2, the lantern welded onto the arch) |

The cone row also gates SECTARMS' NURBS × cone stage, which klein wall
3 needs.

The P2/P3 rows carry no table here; `python3 scripts/work.py status --program sect`
lists them. What their plans said still holds:

- **Read first.** A pair moved onto the section value can close a
  dispatch, radius or retirement row by construction, so read each row
  against SECT's and SECTARMS' progress before building it. Matching by
  foot parameter may already have closed
  `along-edge-ring-on-a-curved-face-has-no-join-arm`,
  `a-bar-through-a-ball-refuses-at-a-door-that-moves-with-scale` and
  the two travel-slack rows. Build what does not close on the section
  object, never beside it.
- **Where a wrong answer would hide:** `r-tan-carries-the-arms-proof-gaps-as-tangent-carriers`
  (a refusal that misnames its cause) and
  `the-c5-kind-dispatch-is-spelled-twice` (two exhaustive matches that
  can drift).
- **The cost rows.** `a-ball-and-cylinder-boolean-costs-seconds-per-op-in-an-optimized-build`
  (P1, GREEN's slow-test census, 2026-11-02),
  `a-cylinder-sphere-march-near-its-node-costs-seconds-and-refuses-by-budget`
  and `torus-germ-rows-march-for-an-hour-at-a-fine-band` share the march;
  read PR 5961's near-node fix before them.

## Review posture

Per unit, at dispatch, by the review tiers of
`memories/orchestration-model.md` and `docs/DUAL-REVIEW-PROTOCOL.md`; the
log names each tier and its reason. A lane measures its row's repro on
current main before it builds. A slow-set row on ground a PR edits is run
locally by its lane, since the per-PR gate does not run it.

## Exit

The slate is closed or re-homed. This plan sets no `## Exit criteria`, so
the program closes without an exit walk.
