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

- **Step 1 is built.** PR 5872 built the section object (`geom_brep::pair_section`,
  `topo::boolean::section_table`, matching by foot parameter). PR 5894
  added the cylinder × sphere chords and island closure. All 54 pinned
  poses pass the join and stop typed downstream: props' `Projected` arm on
  SPIRICARM, and `edge_join`'s spline carrier on SPLICE.
- **The lily's wall 1 builds** (PR 5869, its tie fixed in PR 5925).
- **The torus certificate** (PR 5886) serves sphere × torus at rung 3.
- **The torus poses** decide tangency on one critical gap
  (`each-torus-pair-decides-tangency-on-one-pose`, PR 5924, in fix pass).
- **The section object's other arms are on SECTARMS**, and the C5 and
  rung-3 P2/P3 residue on SECTRUNG (split 2026-10-28, Ev in chat).

## The slate

| pri | item | cost | status | title |
|---|---|---|---|---|
| P0 | `main-red-cylinder-sphere-section-runs-out-of-cells-near-the-node-at-1e-6` | M | closed (PR 5961) | The cs section exhausts its cell budget near the node at ε 1e-6 since PR 5872 |
| P0 | `each-torus-pair-decides-tangency-on-one-pose` | H | closed (PR 5924) | One pose per torus pair, the critical gap |
| P0 | `torus-germ-pairs-have-no-section-frame` | H +design | closed (PRs 5972, 5974, 6011) | Torus walls against a sphere, cylinder or torus reach the join with no section (teapot wall 2) |
| P0 | `cylinder-torus-rung-three-arm-is-unretired` | M | closed (PR 5972) | The cylinder × torus door over a Positive gap |
| P0 | `torus-torus-rung-three-arm-is-unretired` | M | closed (PR 5972) | The torus × torus door over a Positive gap |
| P1 | `main-red-the-seam-region-door-row-runs-15x-slower-since-the-cylinder-sphere-chord` | M | dispatched | The seam-region door row runs 15× slower since PR 5894; held out of the nightly's 1e-12 row |
| P1 | `a-surface-pairs-march-is-repeated-per-operation-and-per-germ` | M | dispatched | One surface pair is marched about eight times per row |
| P1 | `a-cylinder-face-re-homes-its-rings-on-a-chart-that-reads-no-spline-chord` | M | dispatched | A cylinder face holding a bystander ring re-homes it on a chart that reads no spline chord |
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

## Review posture

Per unit, at dispatch, by the review tiers of
`memories/orchestration-model.md` and `docs/DUAL-REVIEW-PROTOCOL.md`; the
log names each tier and its reason. A lane measures its row's repro on
current main before it builds. A slow-set row on ground a PR edits is run
locally by its lane, since the per-PR gate does not run it.

## Exit

The slate is closed or re-homed. This plan sets no `## Exit criteria`, so
the program closes without an exit walk.
