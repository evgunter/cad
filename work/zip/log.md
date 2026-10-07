# ZIP — the log

## 2026-09-20 — opened

Cut out of REACH, which was carrying 151 budget points, when Ev
ratified the priority and track-size conventions in chat the same day.
The cut followed REACH's PRIORITY seam per `work/README.md` "Track
size", and was made into several tracks at once so they can run in
PARALLEL — Ev, in chat: *"for these high priority tracks it's ideal to
have several components that can be worked on in parallel."*

7 rows arrived by `git mv` with their ids, bodies and history
unchanged. REACH keeps its band 6000-6099; band 7200-7299 claimed for
this program in the same commit (`docs/MODEL-AB-LOG.md`). Nothing
dispatched.

## Announced seam from TOPO (2026-09-24)

TOPO's `kevs-fan-merge-needs-a-re-describing-kill-door` (branch
`topo/kev-describing-door`; PR title "TOPO: kev refuses a merge that
would strand a carrier; kev_describing takes the re-descriptions")
lands Ev's ruling (c) on PR 2527. `Body::kev` stays keys-only and now
refuses, before mutating, a fan merge that would re-base a certified
edge onto the surviving vertex (`EulerOpError::MergeRebasesCarriers`,
naming every such member) or move one end of a null edge
(`RebasedNullEdge`). It carries a merge that moves nothing: an empty
fan, or a killed null edge. `Body::kev_describing(he, &[(EdgeKey,
EdgeCurveSpec<T>)], tol)` is the kill that takes a band and the merged
members' re-descriptions. A listed member is certified at the merged
endpoints. An unlisted one passes the re-basing gate `mev`'s fan site
passes. `kev_describing(he, &[], tol)` is the kill with a band and
nothing re-described.

**Your files, and what changed: `crates/topo/src/boolean/zip.rs` (`record_kev`) and `crates/topo/src/boolean/rest.rs` (the slit fuse's kill in `zip_folded`), the latter shared with TANG.** Each of these kills merges
two vertices that the section or the fuse put a band apart, across a
certified circle, so the merged fan's carriers still end where they
land. Each kill now takes `kev_describing(he, &[], tol)`, which
re-certifies every member under the run's band. The keys-only kill
takes no band, so it would refuse these merges. Measured by
instrumenting `kev` across `cargo test -p topo` and
`cargo test -p sweep` on the merge base: every member at these sites
re-certifies within band, including the zip's 58 ulp-distinct merges.
The strut-undo kill in `boolean/rest.rs` kills a null edge, whose two
vertices hold one point, so it stays keys-only.

Also `crates/topo/src/merge_faces.rs`, which TOPO and ZIP both claim.
The three new variants go into `OpPlacement::of`'s enum-verdict arm.
The `kev` row of its site table now says the fan-merge refusals cannot
fire at `strut_tip`'s site, whose far vertex has valence one.
## 2026-09-26 — note from CONTACT (CONTACT-2, PR 3250)

CONTACT-2's lane filed or edited three rows on your slate:

- `an-edge-lying-in-a-cutter-face-past-its-end-wall-leaves-loose-ends-unpaired`
  (new). The axis-coincident box lap's remaining refusal; it
  reproduces on an all-planar diamond at the base.
- `role-resolution-interior-tiers-certify-only-planar-region-faces`
  (new). Its review turned the open question into a reproduced
  defect: a curved edge's chord-midpoint anchor reads both loops alike
  and the join refuses `SectionLoopMixed`
  (`crates/sweep/tests/axis_lap.rs`
  `a_chord_midpoint_probe_reads_both_loops_alike`). The lane set it to
  P0/H. Re-band it if you read it differently. It refuses loudly
  today; it is not a wrong answer.
- The top-entry blind D pocket stays on
  `blind-d-pocket-subtract-refuses-with-join-internal-words`. The
  bottom-entry pocket now builds, pinned in `axis_lap.rs`.

CONTACT-2 also touched `boolean/join.rs`, where the anchor tiers are
now named by an `Anchor` enum.

Signed: (CONTACT orchestrator)
- 2026-09-29 — Seam note from ENCL: PR 3418 (merged `3094222a13`) implements Ev's ruling on `[ev]` PR 3402. `Decide::sign_within` returns `Decided { sign, margin }` on every outcome. `k_stats::decide` still returns `Sign`, and its sibling `decide_reported` returns `Decided`; the two share one classify and one log write. `MarginDiag` is now opaque and for error reporting only: no variant to match, no field, no ordering, no f64 conversion. The recourse wording comes from `sized_recourse`, and the only numeric door is `diagnostic_f64_for_error_text()`. `scripts/gates/reporting-margin-door.sh` pins door calls, mints, `sized_recourse` callers and `terminal_sliver: true` per file. `Indeterminate` gains `terminal_sliver`, decided at classify time. If your code matched `MarginDiag::Value/Enclosure/Invalid` or read its numbers, it now uses `kind()`, `is_invalid()` or the error-text door; this PR touched those sites mechanically. (ENCL orchestrator)
- 2026-09-29 — Seam note from TOPO: PR 3160 (`half-edge-minting-euler-ops-leave-a-minted-curved-face-incomplete`, branch `topo/mint-rows-at-the-mint-site`) adds one variant to `EulerOpError`, `PcurveMint` (an Euler operator refusing to add a half-edge to a face whose pcurve rows are complete on a spline chart), and so one arm to `OpPlacement`'s exhaustive match in `crates/topo/src/merge_faces.rs`, beside `PcurveSplit`. Nothing else in the file moved; the path is a double claim (TOPO and ZIP). (TOPO lane)
- 2026-09-29 — Seam note from TOPO: PR 3493 (branch `topo/route-refusal-subjects`) routes the Boolean's escalated and contradicted refusals by closed decision types (D4 ¶1 (i), PR 3352). `join.rs` and `rest.rs` construct their `BooleanError::Escalated` through `BooleanError::coincidence` (the decision `BooleanDecision::Coincidence`); `merge_faces.rs`'s `DeclarationContradicted` carries the `Contradiction` its declared rung set. (TOPO implementer)
- 2026-09-29 — Seam note from TOPO: PR 3467 (`topo/sense-reads-same-chart`, not yet merged) implements Ev's D1 ruling (PR 3480): `FaceSurface::New { surface, sense }` and `Shared { key, sense }` state the new face's bit; on the parent's chart `mef` derives the parent's bit and `mfkrh` its negation, and a contradicting stated bit is refused (`EulerOpError::SenseContradictsChart`); `set_face_surface` takes the same spec and `set_face_surface_and_sense` is gone; `Body::mvfs` and `Body::mfkrh_plug` take the seed's provisional bit. Paths: `topo/src/boolean/rest.rs` and `topo/src/merge_faces.rs` (a double claim, TOPO and ZIP). `boolean/rest.rs`'s transient `mfkrh(Inherit)` promotions now mint the parent's bit negated; `merge_faces.rs`'s `OpPlacement` match gains one arm, `SenseContradictsChart`. No row moved. The PR also files `slit-zip-band-run-across-two-loops-is-reached-by-no-row` on this slate: `slit_zip`'s band-run promotion is reached by no row. (TOPO implementer)
- 2026-09-30 — Seam note from EMIT: PR 3241 (branch `emit/union-face-names`) adds `BooleanNaming::discards` (`crates/topo/src/boolean/discard.rs`, new): every face a boolean discards, in its operand's clone keys, with the kept-side ends of each stretch it bordered a kept face along and the split lineage of its other boundary edges. `finish.rs` records the section path's discards after the selection (`discarded`, reading the null-edge `below_end`/`above_end` copies and the B graft); `rest.rs` records the REST union's contact patches before the glue (`patch_discards`). The paths that keep or drop whole operands split no face and record none. A section vertex with no null-edge copy, or a kept vertex missing from the graft, refuses `JoinDesync`. The naming layer reads the record to name a split face's pieces by the walls between them (N2 `Borders`); nothing in the kernel reads it. The PR also files `a-round-tube-standing-on-a-plate-refuses-seam-orientation` on this slate. (EMIT implementer)
- 2026-09-30 — Seam note from TOPO: PR 3506 (branch `topo/torus-and-merge-one-story`, not yet merged) edits `boolean/rest.rs` and `merge_faces.rs` (a double claim). The Rest verify's declared pairs route a plane-rung escalation by `PlaneDoor::Declared` (a defect on the unreadable norm, `PLANE_ORIENTATION` on orientation). `MergeCoplanarError::Escalated` carries a `MergeDecision`, and the declared pair's orientation is the merge's own decision (it passes only on a same-facing pair), which `DeclaredOppositeOrientation` now ends in as its definite arm, with no label or face keys. (TOPO, PR 3506 fix pass)
- 2026-09-30 — Seam note from TOPO: In PR 3513 (branch `topo/every-escalation-names-its-decision`), the join's matching escalations name `Coincide::Join`, `join::frame_refusal` routes a section pose's escalation to `BooleanDecision::Proximity(Coincide::Section)` (the pose reads parameter sources, no face-pair declaration) and the radius guards to `BooleanDecision::Radius`; `rest.rs`'s `RestZipUnsupported` carries a closed `RestZipFrontier` for its `what`, each ending in the lever that reaches past it or `NOT_YET_ENDING`. The join's matching decisions are filed at `work/topo/boolean-coincidence-route-still-holds-join-and-self-check-decisions.md`. (TOPO implementer)
- 2026-09-30 — Seam note from TOPO: In PR 3513's second fix pass (branch `topo/every-escalation-names-its-decision`), `crates/topo/src/boolean/rest.rs`'s REST seam walk states `DeclarationRead::Spent(ContactClass::Rest)`. Filed here: `rest-zip-drops-the-euler-operators-refusal` (the zip's 18 `map_err(|_| …)` discards of `EulerOpError`, predating PR 3513). (TOPO implementer)

## 2026-10-01 — note from REACH: a row filed on your slate

`a-union-glues-same-sense-cosurface-walls-without-merging-them` (P0):
a union leaves same-sense cosurface wall pairs unmerged, and its own
next op rejects the result. It is coupled to REACH's open fork on
`cosurface-disjoint-curved-walls-refuse`, whose answer decides whether
the pair should merge or refuse. — (REACH orchestrator)

## 2026-10-02 — cut along the layer seam

ZIP measured 75.5 budget points against its 30, about 78 once its
legacy and unpriced rows were priced. No two-way cut fits the budget,
so Ev, in chat, chose three tracks by layer: **JOIN** (`work/join/`,
the join's chord matching, loose-end pairing and role resolution),
**FUSE** (`work/fuse/`, the merge door and the rebuild), and ZIP keeps
the declared-REST zip and the seam zip. The rows moved by `git mv` with
their ids and bodies unchanged. ZIP's `paths` narrow to
`boolean/rest.rs` and `boolean/zip.rs`.

Priced in the same commit: `a-round-tube-standing-on-a-plate-refuses-seam-orientation`
P0/H (an ordinary union refusing in kernel-bug words), and
`rest-zip-drops-the-euler-operators-refusal` P3/M (its repair shape is
written, across 18 sites).

Signed (JOIN orchestrator, at the cut).

## 2026-10-02 — note from JOIN: a row filed on your slate

`rest-zip-segments-read-a-straight-chord-facing-test-and-a-vertex-pair-identity`
(P0/H): the dumbbell's REST-lane blocker, measured by JOIN's in-face
probe (`join/inface-probe`). Its identity half may be answered by
JOIN's open design fork on how a segment that coincides with an
existing edge is identified. — (JOIN orchestrator)
- 2026-10-02 — Seam note from TQUERY: PR 3768 (merged) types `SplitPlane.normal` as `geom_core::UnitVec3`. Mint one with `topo::test_support::split_plane(origin, dir, tol)` in tests, or `UnitVec3::new(v, site, band)` in code. A `SplitPlane { normal: Vec3 }` literal on an open branch stops compiling. The section join lanes carry the witness end to end, so `chord_join::SectionPlane` is gone. The boolean decides each germ plane's normal at the read (`BOOL_GERM_PLANE_NORMAL`), and a degenerate germ normal refuses `JoinDesync`. Paths touched on your ground are listed in the PR body. (TQUERY orchestrator)

## 2026-10-02 — a new P0 row from SHOW

`an-engraved-annular-sector-refuses-seam-orientation` landed on this
slate from SHOW's `tiltedcut` engraving (PR 3819): a one-arc annular
sector refuses `SeamOrientation` as a pocket, a through-cut and a
boss, on a cylinder and on a box. It may share a root with
`a-round-tube-standing-on-a-plate-refuses-seam-orientation`, but that
is unproven.

## 2026-10-02 — note from JOIN: a row claimed

`rest-zip-segments-read-a-straight-chord-facing-test-and-a-vertex-pair-identity`
moved to `work/join/` under JOIN-2 (`docs/JOIN-2-SPEC.md`): the REST
zip reads the join's segments, and `enumerate_segments` and
`fan_edge_between` go. JOIN-2 edits `boolean/rest.rs`, which is shared
with TANG, and will announce the seam in its PR. — (JOIN orchestrator)

## 2026-10-06 — picked up; cut along the priority seam

An orchestrator holds the track again (`status: active`). ZIP measured
48 budget points against its 30. Per `work/README.md` Track size it
splits along its priority seam: the eight P3 rows moved by `git mv` to
**ZIPTAIL** (`work/ziptail/`, 18.5 points, `ready`, nobody on it), and
ZIP keeps its P0 to P2 rows. `a-declared-continuation-across-a-rabbet-step-leaves-six-loose-ends`
was unpriced; priced P1/M (a declared union refusing on ordinary
blocks, its remaining shape unmeasured on main).

FUSE's `fuse/cell-pair-contacts` (PR 3955, open) edits `rest.rs` and
`zip.rs` and files `a-rest-lane-slit-zip-kills-seam-edges-with-no-substitution-row`
(P3/E) here; it goes to ZIPTAIL once it lands. TOPO's
`topo/face-boundary-walks-one-home` (PR 4099, open) edits `rest.rs`.

## 2026-10-06 — dispatched

- **Measure first, no code**: the seam-chord cluster (`zip/chord-probe`:
  the four `mint_chord` rows, the straight-chord fallback on a curved
  host, and whether the join's in-face insertion is the same job);
  the round tube (`zip/tube-seam-orientation`, with JOIN's engraved
  annular sector beside it); the reflex wrong volume and the rabbet's
  fold order (`zip/rest-admission`: what `try_rest_union` verifies
  before it admits a union). Each lane stops at its measurement.
- **Built**: `survivor-folds-a-corrupt-fusion-list-onto-a-dead-key-outside-the-contact-remap`
  and `a-rest-zip-refusal-tells-a-declared-contact-to-declare-the-coincidence`
  as one unit (`zip/survivor-and-recourse`). Review tier: **single,
  style**. Both are small, and they can be read and believed: one
  closes a debug-only guard, the other checks prose against
  `RestZipFrontier`'s endings, which postdate the row.

## 2026-10-06 — the reflex row measured: the REST zip's admission

`zip/rest-admission` measured on main `3f1e3b0d03`. The reflex
batteries ship no wrong body, and none of their runs enters the REST
zip, so the row's original bar is met. Its second cause is live,
though. `try_rest_union` assumes the interiors are disjoint and never
checks that the seam bounds the contact patch. A join refusal with a
lever (a post resting on `a`, beside the reflex pose) lets the zip
admit a transverse contact, and 16 of 16 runs ship `vol a + vol b`.
Two of them pass every gate. The fix is in the same lane, phase 2: the
zip declines unless every matched segment bounds a patch face on both
solids. The rabbet's fold-order shape builds SOUND in all six orders.
That row closes with a pin. Two things are filed or checked at
phase 2: the edge-in-face dip that the segment check cannot see (the
gate refuses it, in the wrong words), and whether the boolean gate's
omission of tier 3′ is ratified. Review tier for phase 2: **single,
full**. The claim to falsify is that the check declines no pure REST
union.

## 2026-10-06 — the seam-chord cluster measured; designers weighing

`zip/chord-probe` measured on main `3f1e3b0d`. Of the four `mint_chord` rows:
- **Boss-flush does not reproduce.** The flush walls are reported first, and accepting the offers builds the union by the general join. It closes with a pin (`zip/chord-rows`).
- **Two rows are down to one scene each:**
  - The blind shaft is down to the shaft lying wholly inside the bore. The join refuses first at `chart_ring_side`'s full-period window, then the zip refuses `ChordEndpointRevisited`.
  - The split collar is down to the through span. The join refuses `RingHomingAmbiguous`, and the zip declines at `mirror_edges` because a vertex has no counterpart.
- **The straight-chord fallback is reached by none of about 1700 calls.**

The finding that matters is a class: the join chooses each segment's corner from the geometry at strut insertion and carries it as `HalfGerm.he`. The REST zip drops that, undoes the struts, and re-derives the corner from loop structure, which cannot answer at a vertex the loop visits twice. Same job, two implementations.

A designer pair is weighing the zip's seam realization: blinding on `analysis/design-fork/rest-zip-seam-realization`, problem statement handed to both unchanged. Whether it goes to Ev depends on what they find.

## 2026-10-06 — the round tube closes; friction: disk

`a-round-tube-standing-on-a-plate-refuses-seam-orientation` builds on main in every order and is closed by PR 4126. A 2160-boolean variant sweep around it found no `SeamOrientation` refusal. 4126 merged over an inherited red: JOIN's `pinch-tessellate-row-escalates-at-eps-1e-6`, annotated on the PR.

**Friction (finding).** This remote container's disk allowance is about 40 GB. One lane's debug target grows to 7–13 GB once it has built probes and a branch or two, so three concurrent cargo lanes fill it. Two lanes hit ENOSPC or near it today. The width-1 build slot also serialized designer probes behind lane suites for 20+ minutes. In this environment, more than about three building lanes belong in their own cloud sessions (`memories/orchestration-model.md`, the 2026-10-02 rule), not as subagents here.

## 2026-10-06 — what the day closed, and the D10 hold

**Closed:** the round tube (PR 4126), the flush boss (PR 4130), the reflex wrong volume (PR 4127), the rabbet fold order (PR 4127), and the seam zip's half-minted wall (already fixed by PR 3531). On merge, PR 4116 closes `Fusions` and the recourse row.

**PR 4127** (single full review, then a fix pass) makes the REST zip decline a mate whose seam does not bound its contact patches. That retires a live wrong body: `vol a + vol b` behind a join lever, which passed every gate in 2 of 16 runs.
- The reviewer's numbers: 95 zip entries and 91 builds unchanged; a 277-run lever grid with 0 declines of a pure contact; 58 wrong bodies without the check.
- Not caught, and parked on one row: edge-in-face contacts with no section segment. A dip refuses in the gate's words. A kiss builds at the right volume but fails tier 3′, which the door's gate does not run (REACH's `boolean-door-runs-the-census-over-its-result`).

**PR 4116** (single full review, then a fix pass, then a delta review) makes a fusion list unrepresentable unless it is well-ordered. Its reviews filed:
- on FUSE: the face-lineage half of the same class, and `fused_into` naming vertices that are not live in the result, in about 1 of 8 lattice results;
- on WIRE: `emit_topo` reads a fusion chain one hop. Two-hop chains do reach it, in 632 of 3,639 results.

**The D10 hold.** JOIN's log carries Ev's 2026-10-03 hold: no new unit that meaningfully uses declared pairs or declared contact. It covers the declared-REST zip, which only declared coincident faces reach. ZIP's lanes were dispatched before this orchestrator read it.
- On Ev's ruling in chat, the admission fix (a live wrong body) finished as a started unit.
- **The designers' fork.** Two blinded designers converged independently: retire the zip and give the join what it builds today. That went to INTENT as stage-4 input (`the-declared-rest-zip-retires-at-stage-4-and-the-join-needs-three-arms`), with no `[ev]` PR (Ev, in chat). The blinding byte is on `analysis/design-fork/rest-zip-seam-realization`. The fork never went to Ev as a PR, so it is not a design-fork log row.
- Every remaining REST-lane row is parked on `d10-one-way-to-say-intent-is-unbuilt`, and ZIPTAIL folded back in (`docs/doc-ledger/ziptail-leaves-the-tracker.md`).
- ZIP is `blocked`, with nobody on it.

**Class findings without a row:**
- About ten per-file `vol` helper copies across `crates/sweep/tests` (P4; noted in PR 4127's body).
- A one-line slow-set filter in `.config/nextest.toml` that every PR adding a slow test conflicts on. Both are friction, not defects.

## 2026-10-06 — handed back, blocked

PR 4116 merged (`Fusions`; single full review, fix pass, delta review), closing its two rows. The remaining 12 live rows are all parked on the D10 hold, so ZIP is `blocked`, with nobody on it. It fires when INTENT closes `d10-one-way-to-say-intent-is-unbuilt`. — (ZIP orchestrator)

## 2026-10-06 — seam note from CARVE: a ZIP row was red on main at ε = 1e-6 (since passing)

`crates/sweep/tests/rest_zip_admission.rs`
`the_tangent_lever_keeps_building_pure_contacts` failed at
`CAD_TOLERANCE_EPS=1e-6` on `origin/main` `a9c038c37`. It hit a
`bool_contact_vertex` escalation, with margin 2.29e-6 in the band 1e-6
to 1e-5. The row arrived with `290d95a31` (the admission fix pass), and
CARVE's PR 4186 found it, because the per-PR gate runs the 1e-6 row only
for a diff touching `sweep`. CARVE's surface-pair lane reports that the
row passes on a later main. Recorded so that ZIP can confirm which
change fixed it.

Signed: (CARVE orchestrator)
