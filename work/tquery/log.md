# TQUERY — the log

## 2026-09-20 — opened

Cut out of TOPO, which was carrying 132 budget points in one directory
— about four and a half sittings — when Ev ratified the priority and
track-size conventions in chat the same day. The cut divided TOPO on
its PRIORITY seam rather than on another territory seam, per
`work/README.md` "Track size": five successors plus the Euler-operator
remainder TOPO keeps.

7 rows arrived, each by `git mv` with its id, body and history
unchanged. Band 6500-6599 claimed in this commit
(`docs/MODEL-AB-LOG.md`). Nothing dispatched.

## Ev ruled `rim-of-compares-point-bits-…`: a consumer (TOPO orchestrator, 2026-09-29)

On PR 3156 (2026-09-24), Ev answered "(b)": `query.rs`'s `same_bits` /
`same_point_bits` behind `rim_of`'s `same_circle` is a production
bit-identity coincidence check, which the retirement forbids. The
ruling and the repair shape (a `GeomSource` read first, allowlisted
`eq_bits` only where no recipe exists) are on the row. The gate half
is filed on GUARD.
- 2026-09-29 — Seam note from TOPO: PR 3467 (`topo/sense-reads-same-chart`, not yet merged) implements Ev's D1 ruling (PR 3480): `FaceSurface::New { surface, sense }` and `Shared { key, sense }` state the new face's bit; on the parent's chart `mef` derives the parent's bit and `mfkrh` its negation, and a contradicting stated bit is refused (`EulerOpError::SenseContradictsChart`); `set_face_surface` takes the same spec and `set_face_surface_and_sense` is gone; `Body::mvfs` and `Body::mfkrh_plug` take the seed's provisional bit. Paths: `topo/src/split.rs`. In your files every `New`/`Shared` spec, `mvfs` and `mfkrh_plug` call states the bit it carried before; no expected value moved. (TOPO implementer)

## 2026-10-02 — first sitting: track taken

An orchestrator holds the track (`status: active`). CLEAVE (active)
shares `topo::split`'s ground (`crates/topo/src/splitting/*`); lanes
here announce that seam in their PRs.

**Review posture, answered** (the plan left it open): single FULL
review (claims + style lane) on the two P0 units — both change what a
public door decides, so believing them takes more than reading them;
orchestrator's read for the `E` rows that ride along. No dual: neither
unit is a broad architectural choice, and Ev already ruled the shape
of the `rim_of` repair (PR 3156).

**Dispatch, wave 1** (both measure first, since main has moved a long
way under these rows since they were filed):
- `tquery/split-cyl-feature` — `split-refuses-cylindrical-feature-box`:
  re-measure both variants on current main, diagnose, fix.
- `tquery/rim-of-recipe` — `rim-of-compares-point-bits-…` (Ev's
  ruling) which also decides `rim-of-refuses-extruded-multi-arc-rims`;
  riders in the same file: `rim-of-flattens-a-dangling-curve-key`,
  `tquery-refusal-prose-outgrows-the-viewer`.
- 2026-10-02 — `split-edge-cannot-carry-a-fitted-or-general-pcurve-row`
  parked on PR 3759 (PCERT, "pcurve rows are mandatory at rest"),
  which rewrites `crates/topo/src/pcurves.rs`'s carry (`split_cache`'s
  home) and `tests/split_edge_pcurve_rows.rs`; building the
  Fitted/General carry under it would race it.
- 2026-10-02 — `curve-kind-placement-…` (the open ruling) sent to the
  designer pair; blinding record on
  `analysis/design-fork/tquery-curve-kind-placement`.
- 2026-10-02 — `split-edges-key-retention-direction-is-pinned-by-no-row`
  dispatched (`tquery/split-edge-retention`), orchestrator's-read tier:
  a test-only row whose content is written in the item.
- 2026-10-02 — designers agreed on the final state (one kind mirror per enum, in `geom`; sets stay in `topo::query`), split only on authoring (hand-written vs `strum` derive); `[ev]` PR 3763 opened carrying both reports as A/B (fork-log row 41).
- 2026-10-02 — `tquery/rim-of-recipe` STOPPED at a fork, nothing built.
  Ev's preferred repair (a `GeomSource` read) has nothing to read:
  every kernel-direct body's curves are `KernelDirect`, and editor-core
  stamps one source per curve description, so a rim's arcs are always
  distinct sources. The fallback (`eq_bits`) keeps the refusal.
  Measured: all arcs of an extruded rim share the same two surface keys
  (extrude decides one cylinder per run), and with the bit compare off
  the rim suites pass bar four rows that pin the bit rule or the
  winding contract. A CLASS finding: N6's recipe provenance does not
  reach kernel-direct bodies, so any repair that leans on it is
  editor-core-only. Sent to the designer pair
  (`analysis/design-fork/tquery-rim-identity`); the dangling-key and
  prose riders wait for the answer (same arms of `RimError`).
- 2026-10-02 — PR 3768 (split-cyl-feature): both variants were one cause, a non-unit `SplitPlane.normal` (the cutaway's raw 1.264-long direction) read as unit by the tilted plane×cylinder section; fixed by typing it `UnitVec3` (the ratified unit-vector ruling), which also takes the `SplitPlane` half of `split-plane-normal-…`. Full review dispatched. Both 3768 and 3773 wait on REACH's #3755 for the 1e-6 open-sign row (red on main); not ported, as it is another program's in-flight 18-file change. A CLASS note: a precondition held as prose ('unit, unchecked') was the live cause of a P0 — the witness ruling's remaining prose sites are worth the sweep it asks for.
- 2026-10-02 — Reviews adjudicated, fix passes out on all three units.
  - 3773 (rim_of): mergeable. The fix pass folds in the reviewer's
    43-body oracle (the only public-door row that pins the fixed side)
    and corrects the membership doc (crossing Villarceau circles
    refuse `Branches`). Two issues filed: no door for an edge's side
    surfaces (a CLASS, five spellings), and the PR gate skipping the
    demos job when a door the tour calls changes.
  - 3777 (kind mirrors): switched to the spec's hand-written fallback.
    The strum derive copies payload docs onto the fieldless kinds,
    which makes them false. The tripwire is an exhaustive `kind()`
    match; `ALL` comes from `VariantArray`.
  - 3768 (split): `chord_join::SectionPlane` re-minted the defect it
    closes (a bare "unit" normal into the plane×conic arms, latent on
    the boolean's germ planes). The fix: one witness end to end, and
    the boolean normalizes at the germ read.
  - A CLASS worth carrying: every fix PR in this wave minted a fresh
    instance of what it closed or left a twin beside it (SectionPlane;
    a second a/an helper; a second z-poled seed helper), and each time
    the reviewer, not the author, caught it — the style lane's §1
    trap, measured again.
- 2026-10-02 — `split-edges-key-retention-…` closed by PR 3761 (orchestrator's read; one comment line trimmed).
- 2026-10-02 — Ev ruled 3763 (one kind mirror per enum, in `geom`; authoring delegated → derived); unit `one-kind-mirror-per-geometry-enum` opened and dispatched (single full review).
- 2026-10-02 — Ev ruled 3767: the structural rim; `NotAnArc` kept with an in-code note that it extends to non-circles. Relayed to the build lane.
- 2026-10-02 — PR 3773 merged: the structural `rim_of` (Ev's 3767 ruling). Closes `rim-of-compares-point-bits-…`, `rim-of-refuses-extruded-multi-arc-rims` (P0), `rim-of-flattens-a-dangling-curve-key`, `tquery-refusal-prose-outgrows-the-viewer`. Single full review (mergeable, no MAJOR); fix pass took all eight items. Filed: `edge-side-surfaces-have-no-door` (here), CIW's demos-row gap, the f64 bit-compare issue in `work/issues/`.
- 2026-10-02 — PR 3777 merged: one kind mirror per geometry enum (Ev's 3763 ruling). Authoring went hand-written, not derived: strum copies payload docs onto the fieldless kinds (the spec's fallback trigger). Closes `one-kind-mirror-per-geometry-enum` and EXPORT's `step-export-carrier-kind-duplicates-curve-kind`. Single full review; fix pass took all items. The RimError adjective wording ("a straight curve") was surfaced to Ev as a taste call.
- 2026-10-02 — Ev's new rule (memories, today): lanes may run as their own cloud sessions when this container is the constraint. First use: `edge-side-surfaces-have-no-door` dispatched as cloud session `session_01768YaXKM7V4BQcsbzGPTdX` (`tquery/edge-side-door`), single review once its PR is up. PR 3797 (split self-validation, tier 2 always-on, typed) in review; its residue filed as `split-halves-have-no-contact-records-…`.
- 2026-10-02 — PR 3768 merged: `split-refuses-cylindrical-feature-box` (P0) closed. Single full review; the fix pass deleted `chord_join::SectionPlane` (it re-minted the defect) and moved the boolean's germ planes to a decided `UnitVec3` at the read (a degenerate germ normal refuses `JoinDesync`, the germ read's corrupt-carrier arm — accepted, not a new variant). The `SplitPlane` half and the germ-plane read of `split-plane-normal-…` are taken; `slab_extent` remains.
- 2026-10-02 — PR 3797 merged: split self-validates at tier 2 (always on, typed), measured first (0.02 ms/half; nothing green refuses). Closes `validate-passes-…` and HONE's `split-hands-out-a-body-without-running-tier-3` (claimed by git mv). Single review; fix pass took all items. Filed: CLEAVE `boolean-and-split-result-gates-are-one-gate-with-two-texts`, WIRE `split-half-side-words-are-spelled-at-three-sites`.
- 2026-10-02 — Pinch-contacts fork: the designer pair crossed TWICE (each held both answers; both agree the deciding question is identity-by-construction vs a declared, falsifiable record). Rather than a third ping-pong round, a fresh designer weighs both final reports (protocol rule 5). Six findings both designers surfaced, owed whichever way it goes, filed: WIRE `transform-and-pattern-drop-a-values-contact-records` (P1), CLEAVE `boolean-declares-no-touching-between-copies-of-one-operand-vertex`, CONTACT `contact-records-carry-operand-labels-into-the-at-rest-currency`, CLEAVE `split-band-on-at-a-concave-edge-may-mint-a-pinch-…`, and here `a-solid-is-documented-connected-…`, `split-vertex-pairs-orientation-flips-…`.
- 2026-10-02 — Pinch-contacts fork to Ev as PR 3813 (fork-log row 45): two crossings in the pair, then a further designer; shared point recommended by B and C, records by A, deciding question stated. Filed CONTACT `topo-surgery-verbs-drop-a-bodys-contact-records` (P1, the kernel side of WIRE's row).
- 2026-10-02 — PR 3809 merged on the orchestrator's read (it grew from E to a small M: a `UnitSpanBox` wrapper and the band threaded through `face_box`/`face_reach`/`face_tree`; same precedent as 3768's germ planes). Closes `split-plane-normal-and-slab-axis-carry-unitness-as-prose`. Filed BOXES `cone-and-torus-box-extents-read-the-carrier-axis-as-unit-by-prose`. It also carried a one-line rustfmt fix for `sweep/tests/run_walls_built.rs`, which main's ea80b402f left red at lint.
- 2026-10-02 — PR 3820 merged on the orchestrator's read. `vertex_pairs` was reversed for the whole-orbit strut's corner pair, which made editor-core naming of an ordinary Extrude + Split through a reflex corner refuse `Emission`. Fixed at the producer: every pair is `(copy, original)`, read off `at_vertex`. The `Solid` "connected volume" doc was an unratified M0 scaffold sentence that contradicts DESIGN's "disjoint unions … are tier-2-legal multi-shell bodies", so only the doc changed. Both rows closed.
