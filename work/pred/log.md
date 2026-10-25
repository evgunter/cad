# PRED log

Newest entries at the bottom; the tail is the program's live status.
Plan: `work/pred/plan.md`.

## Opened (2026-09-11)

Opened in the tracker cut of 2026-09-11 (Ev's direction, in-chat;
`docs/WORK-TRACKS-2026-09.md` addendum 3). Eleven rows moved in by
`git mv`, each with a `## Re-homed` record: one from `work/issues/`, ten
from `work/code-quality/`.

The class was visible in the 2026-08 scan and stayed unclaimed for the
reason the plan now states: each row is on four programs' files at once,
so no program could take one without reaching across three fences.

No branch exists yet. Two cheap first acts, neither a unit: verify
`S66`'s park (#862 merged, and its body says the deviation terms are
already gone), and put `S82`, `S65`, `S116p` and `S29` on the next
`[ev]` sitting.

## Rows arriving from FIX, 2026-09-20

Ev ruled in chat on 2026-09-20 that **FIX carries no design decisions**:
*"can you kick all the design decisions back to the track they actually
belong to, leaving fix design-free?"* FIX is the program for rows whose
fix is already written; three waves closed those and left a slate that
had drifted into decisions. Fourteen rows moved out by `git mv` to the
track owning the surface each decision is about, each carrying a
`## Re-homed` section stating the question, the routing basis, and what
was NOT decided for the receiver. Routing was taken from
`python3 scripts/work.py territory --files -` over every path the rows
cite, not from FIX's `keep_out` prose — two of that clause's fence
claims were stale and are corrected in the moved rows.

**Two rows, one family.** The linear band, which
`band-linear-spelling-not-swept` made one spelling wherever a band is
CONSTRUCTED, has two populations that sweep could not reach:

- `band-derivation-has-a-scalar-twin` — ~15 sites derive `(eps, K*eps)` as
  bare `f64`s and never build a `Band` at all. The decision: do these
  consult `Tol` directly (close the class by saying so once), or does the
  pair want a named door on `Tolerance`? Reaching it through
  `Band::linear` means routing a two-number computation through a fallible
  constructor to read the accessors straight back out.
- `fixed-band-literals-are-an-unscoped-class` — 55 sites hard-code
  `Band::new(1e-9, 1e-8)`, which asks a different question from the run's
  band at every eps row but the default. One site carries a reason; the
  other 54 are unexamined.

They land here because PRED is the program for *one numeric fact, decided
in several places, each with its own margin* — the shape your slate
already carries three times. The door, if one is minted, lands in
`crates/geom-core/src/predicate.rs`, which territory says is PROPS's;
the rows are classes rather than files, so that site is announced.

**Read the second row's "One framing to NOT carry forward" section first.**
The disclosing lane's sharper claim — that fixed bands collide with the
suites declaring *"eps posture: no eps literal"* — was MEASURED and is
false; the two populations are disjoint. The finding survives as an
unexamined class, not a live contradiction.

The third member of the family, `band-helper-duplicated-across-suites`,
went to SUITE in the same sweep: that one is about where a test wrapper
lives, not about what a band decides.

Signed (FIX orchestrator).
- 2026-10-09 — Seam note from ENCL (PR 4423, `encl/second-order-walk-one-home`, merged at `c981d151cb`): the edge-level second-order walk now has one home. `geom_brep::interior_stations` owns the interior station schedule (`1..CERT_SAMPLES-1` via `sample_param`, as `ders1`). `geom_brep::second_order_walk` (with the `StationHook` trait and `SecondOrderWalk::{Determinate, UnderDetermined, InBand, Stopped}`) owns the per-station reading and the one `"tangent_second_order"` decision. Both are `#[doc(hidden)]`: they are kernel-internal and public only for `topo`. `must_carry_over_edge` and tier 3's check 4 both descend through it. A source scan (`tier3_tests::check_4_routes_its_second_order_reading_through_the_one_walk`) reds on a second spelling in `validate.rs`. A new edge-level second-order reader should take the walker, not inline the loop. No answer moved (the k-stream is byte-identical). (ENCL orchestrator)
- 2026-10-09 — Seam note from ENCL: ENCL's scoping of `hand-minted-invalid-gates-in-topo` found four `solid_contain` sites that report a decided verdict as poison (INVALID): `narrower_than_period` at `:1286` and `:2983`, and `cone_nappe` at `:1934` and `:1939`. They render `PointInSolidError::Escalated` ("too close to call … declare/move") for something that was decided. They overlap your `cone-nappe-is-decided-in-five-places` and `period-headroom-margin-has-no-shared-home`; ENCL leaves them to those rows. (ENCL orchestrator)
- 2026-10-10 — Seam note from ENCL (PR 4497, merged at `4c7fb9b0fc`). A poisoned or contradicted margin now ends in a followable recourse:
  - at rest, the census coincidence menu (`validate::too_close` → `own_close`) gives the defect ending;
  - `ChartRegionError::Escalated`'s poisoned arm gives `defect_ending(Build)`;
  - `PointInSolidError::Escalated` gives the unnamed placement lever plus the reading's note, as `contfp`, `classify_point_in_solid` and the Boolean already did.

  `SliverSector` is unchanged, and its mints are on CLEAVE's split-escalations row. (ENCL orchestrator)
- 2026-10-10 — Seam note from ENCL (PR 4520, merged): the PlaneNurbs limbs are decided once, in the lane (`ssi::certify` `nurbs_limbs`). `geom_brep::certify::run_checks` no longer re-decides them, and the k-stream names `plane_nurbs_on_locus` and `plane_nurbs_hull_sup` are gone. The certificate's `max_residual` is bit-identical. (ENCL orchestrator) D46 now carries an evidence line: both names, cited at `docs/predicate-dimension-audit.md:708`, no longer exist.
- 2026-10-25 — (PRED orchestrator) Track taken (`status: active`), branch `pred/orchestrator`. Standing frame for this sitting: work the P1 rows that INTENT's remaining stages will not make irrelevant. INTENT and its successors (COINCIDE, REFS, INLANE) build editor-core's dependency, placement and coincidence model. PRED's rows are kernel numeric predicates in `geom-brep`, `topo`, `mesh` and `sweep`, so the refactor hold does not reach most of them. The exceptions are the editor-core sites in the two band rows and in the `(tol, band)` pair row. The live churn risk on PRED's sites comes from CHART (region doors, PR 5674 open), PCERT (`pcurve_cache`, PR 5500) and QUAD (props measure door, PR 5628), not from INTENT. Two scoping lanes re-census every row against main `2d5677e1` before anything is dispatched. S65 is closed: it was already ruled on PR 884 (row 5), and the cut missed that.
- 2026-10-25 — (PRED orchestrator) Re-census against main `2d5677e1` by two read-only scoping lanes. Each row now carries a `## Re-census` section and a current cost.
  - **No live wrong answer on any row.** Every period-headroom site decides the forward half, or reads a span that cannot be reversed.
  - **Closed as verified:**
    - `S58`: the one face predicate is built, `geom_brep::props::require_iso_rectangle`, and mesh asks it.
    - `S66`: the #862 over-width terms are gone and pinned.
    - `S82`: CERT-1 fixed #893, so the question is moot.
    - `S65`: already ruled on PR 884.
  - **Re-priced:**
    - `period-headroom` H → M, with `design: true` for where the home lives;
    - `D292` H → M, since the census mirror now shares the boolean's body;
    - `S18` → M; its one live sub-row is step-export's volume;
    - `lever-arm` → E;
    - `D46` → H; `D281` → M.
  - **Parked:**
    - `cone-nappe` on PRs 5500 and 5674, which rewrite its sites;
    - a new row, `contact-verify-spells-the-orientation-and-second-order-margins-beside-their-homes`, cut from lever-arm, on the D10 hold. `contact_verify` is declared-contact ground that stage 4 retires.
  - **`S116p` reframed.** "Permanent" is gone from the refusal. What is open is whether the two-run full revolve is built through the void door, as the holed revolve is. `needs_ev` is cleared until the designers have weighed it.
  - **Re-banded P1 → P3, unilaterally:** the two band rows. Every site is test code, so the risk is a test asking the wrong question, not entrenching architecture.
  - **Split.** The load stood at 39.5/30. The P3 and P4 rows (the two band rows, the ordering row, the `(tol, band)` row and `D281`) moved to a new program, PREDTAIL (`ready`, band 14600–14699), along the priority seam. PRED keeps 22 points.
- 2026-10-25 — (PRED orchestrator) Dispatched. Each tier is the orchestrator's call, with its reason:
  - **`D292`** on `pred/d292-directional-extremum`, **full single review**. It is a new enclosure door, and an under-bound is a silent soundness defect.
  - **`S18`**, the step-export volume, on `pred/s18-step-export-volume`, **full single review**. It changes which reading classifies a shell as a void.
  - **`lever-arm` remainder plus `D57`** on `pred/lever-arm-point-arm-and-d57`, **orchestrator's read**. It is a one-home swap with an agreement row, two doc corrections, and an audit paragraph re-sync.
  - **Designer pairs (one Opus, one Fable):**
    - **`S116p`**, a design fork headed for Ev. It is blinded per `docs/DESIGN-FORK-PROTOCOL.md`, with the byte on `analysis/design-fork/s116p-two-axis-runs`.
    - **`period-headroom-margin-has-no-shared-home`**, where the shared home lives. It goes to Ev only if the designers find a fork Ev cares about.
- 2026-10-25 — (PRED orchestrator) The designer pairs are in.
  - **S116p.** Both designers independently recommend building the two-run full revolve through the void door, as `revolve(filled) − Σ revolve(pocket)`. Agreement on the first reports, so no reconciliation was needed. Put to Ev as `[ev]` PR 5680, which widens `crates/topo/README.md`'s producer bullet (fork-log row 194, recommendation half). The unit that builds it waits on Ev.
  - **period-headroom.** Both designers converged: one rule in geom-core, `Turn::{Short, Whole, Over}`, with a typed rate and an `Extent` that cannot be reversed; `ConicArc::of`'s two-sided rule lifted; names kept per site; certified edges carry their turn. This changes no ratified text and is not an Ev fork, so the reconciled design is written into the item and self-merges with the unit. The one small difference between the pair is settled for B: no bare `magnitude` constructor, use `decide_signed`. A's argument for it covers only net quantities, and `decide_signed` serves those. Re-priced H, cut into two units.
  - **New rider.** Both designers found that certification admits `Zero` at the slow rate, so an eccentric ellipse wound past τ can certify. It is filed as `certify-admits-an-ellipse-wound-past-its-period-on-the-slow-rate` (P1; P0 if a door reaches it) and rides unit 1.
  - **Dispatched.** Unit 1 on `pred/period-turn-rule`. Its tier is **dual** (H: concurrent pair), because it is a broad architectural decision that would be hard to change later.
- 2026-10-25 — (PRED orchestrator) PR 5684 (S18), full single review: APPROVE-WITH-FIXES, no MAJOR. The adjudicated fix pass makes `step_string` take `&AtRestBody<f64>`. That removes the writer's conditional tier-3 gate, a partial second validation home, and `MultiShellNotAtRest`. It also fixes the false "one solid, two outer shells" prose and docs, panics on the unreachable arm, and makes the void refusal report the volume enclosure.
  - **Class finding, logged and not owned here:** doors that wrap `validate` findings in their own words instead of `topo::Unfinished`'s shared refusal texts. Instances: editor-core's `product.rs` and `eval/mod.rs` `Vec<ValidationError>` wrappers. This fix pass removes the step-export instance.
- 2026-10-25 — (PRED orchestrator) PR 5685 (D292), full single review: APPROVE-WITH-FIXES, one MAJOR found by execution. Past a full turn, the f64 door's `cos(min(h, π))` makes a crest antipodal to the span's middle an exact tie decided by rounding, so up to 2A is dropped. That is reachable through the new `pub` door, while stored edges are bounded by certification. The base's `geom_brep::extent` `crest_in_span` had the same hole. The fix pass decides a full-turn span unconditionally, restores the two deleted-probe cases, re-points the census-lane contract, and takes the cheap one-home style items. The reviewer also found `implicit::conic_arc_residual_range`, the D292 class itself, sampled and read by `reduce`'s `arc_margin`. It goes onto `harmonic-crest-in-span-has-spellings-beside-arc-window`.
  - **Friction:** this 4-CPU box has about 40 GB of writable disk for lanes. Three concurrent implementer or review lanes each hold a 3–11 GB target, which put free space under 6 GB twice. Two concurrent build lanes plus reviewers that reuse finished targets is the practical ceiling here. Past that, lanes should run in their own cloud sessions (`memories/orchestration-model.md`, 2026-10-02).
- 2026-10-25 — (PRED orchestrator) PR 5681 merged on the orchestrator's read: `lever-arm-fold-six-hand-rolled-siblings` and `D57` closed. `point_arm` and `folded_lever_arm` are different quantities, documented as such and pinned where they agree. The audit's never-reach-the-funnel paragraph is re-synced.
- 2026-10-25 — (PRED orchestrator) **Ev ruled S116p on PR 5680: "sounds good!", meaning build it.** PR 5680 merged with the widened producer bullet in `crates/topo/README.md` and fork-log row 194 completed. S116p closes as ruled, and the build is a new unit, `full-revolve-builds-a-cavity-per-axis-gap` (P1, M). It is dispatched once a build slot frees.
  - **Classifier outage (about 05:30–16:00Z).** The server-side safety classifier stopped returning verdicts, which killed three lanes mid-work. None lost work:
    - D292's fix pass had its edits complete in its working tree, uncommitted;
    - S18's fix pass was mid-edit;
    - period-turn unit 1 had commits pushed and no PR yet.

    All three were resumed in place once it recovered. S18's target `incremental/` (8.8 GB) was reclaimed to make room. Friction finding: a lane killed this way leaves no hand-back. Committing and pushing at every working state is what made each one resumable.
- 2026-10-25 — (PRED orchestrator) Period-turn unit 1 is PR 5686, frozen review-ready head `e7d55cf3a6`, CI green.
  - **What it built.** `Period`, `WalkStop` and `BranchMiss` moved to `geom_core::period`, re-exported by `pcurve_cache`. The rule is `Period::turn`. Certification and `ConicArc::of` decide through it, and the certificate carries a `CertifiedTurn`.
  - **The rider.** It was red-first on `pred/period-turn-rule-red`; the edge now escalates. Its census finds no shipped door that reaches it (STEP import wraps `t1` within about 9e-16 rad), so it stays P1.
  - **Main-red the lane reported.** `m4_pr3_names_ci::die_name_tables_are_golden` is red at the base too. It is already INTENT's `main-red-the-seat-and-name-table-goldens-moved-at-dprime2`.
  - **The dual pair waits for disk.** Two fresh full-rebuild targets do not fit beside the two running fix passes. The pair dispatches, concurrently and on the same frozen head, once those lanes report. Following the protocol, the duals are reduced rather than one reviewer's method narrowed.
- 2026-10-25 — (PRED orchestrator) PR 5684 (S18) merged after a full single review and its fix pass: the export door takes an `AtRestBody`, and `volume.rs` is gone. S18 is closed.
- 2026-10-25 — (PRED orchestrator) PR 5685 (D292) merged after a full single review (APPROVE-WITH-FIXES, with one MAJOR fixed in the fix pass and the counterexample pinned). D292 is closed.
- 2026-10-25 — (PRED orchestrator) Dual review of PR 5686 is in. Both reviewers returned APPROVE-WITH-FIXES with no MAJOR, so there is no tally candidate. Both were uninterrupted with no relaxations, so the pair is fair. The correspondence pre-note goes in the row at merge. Bilateral findings:
  - `Extent::width` admits a reversed or bare extent. Both executed it.
  - Kept `ConicArc` arcs record escalations.
  - The span is carried twice, with two dispositions for a missing turn.
  - The tangent spiric escalates.
  - The census depends on ε.

  The union went to the original implementer as a 13-item fix pass. Also dispatched: `full-revolve-builds-a-cavity-per-axis-gap` on `pred/full-revolve-cavity-per-axis-gap`. Its tier is a **full single review**: a construction change behind a ratified door, with the geometry argued by both designers. The design itself was weighed and ratified on PR 5680.
- 2026-10-25 — (PRED orchestrator) PR 5686, period-turn unit 1, merged: dual pair, fix pass of the union, row `docs/dual-review-log/rows/PR-5686.md` (tally 0; the coding was the orchestrator's unblinded pre-note, recorded as such). The rider closed. `period-headroom-margin-has-no-shared-home` reopened for unit 2.
- 2026-10-25 — (PRED orchestrator) PR 5684 (S18) merged once CI was green on its main-merged head: GitHub had shown it `dirty` because of the log's union merge. PR 5674 merged, so `cone-nappe` now waits on PR 5500 alone. Period-headroom unit 2 is dispatched on `pred/period-turn-sites`. Its tier is a **full single review**: unit 1's dual review settled the architecture, and unit 2 applies it at every remaining site, with verdict deltas at the bounded-rate sites.
- 2026-10-25 — (PRED orchestrator) The two-run revolve is PR 5707, frozen at `bfb00b26aa`, CI green. A full single review is running on that head, reusing the finished lane's target. A designer pair is weighing S29's sizing policy, a fork that goes to Ev; it is blinded on `analysis/design-fork/s29-sizing-policy` (byte 24). D46 waits for unit 2 to land, because both edit `docs/predicate-dimension-audit.md`.
- 2026-10-25 — (PRED orchestrator) **S29 put to Ev as `[ev]` PR 5708.** The designer pair (blinded, byte 24: Opus A) disagreed in round 1: one global σ = ½ against a per-lane rule, and A = 16 NURBS-only against every lane. In round 2, each shown the other's report, both moved and converged. The rule is per lane: the worst admissible triangle certifies at δ by a pinned theorem, with no global factor. Sphere and cone rims are tightened to the grid. Unproved lanes carry named headroom. A = 16 is the NURBS split's regularizer, not a shape promise. `[tolerance.chordal]` gains a no-count, no-shape sentence, in a new `crates/mesh/README.md` `[mesh.sizing]`. Fork-log row 195 holds the recommendation half. Filed from the weighing: TESS's `a-cone-rim-chord-joined-to-a-higher-row-certifies-above-delta-s`.
- 2026-10-25 — (PRED orchestrator) PR 5707 (two-run revolve), full single review: APPROVE-WITH-FIXES. All eight claims survived 11 probe rows: winding, carried containment, run direction, `AxisGapEscalated` reachability, the sentinel, goldens, naming and the sweep. The fix pass covers:
  - the pocket path breaking the swept-order docs;
  - a second, direction-dependent collapse of loop 0's runs;
  - three docs;
  - three style items (the `Option` invariant, the sentinel, the naming test).

  **Class finding, logged:** a sub-chain of a loop re-collapsed separately from the loop's own collapse, which is two spellings of one loop's runs. Look for it wherever a stretch is re-collapsed.

  **Probable kernel defect, out of fence:** a near-collinear kink on a single-run shell passes validation and then refuses at the Euler attachment gate as "a kernel defect". The lane reproduces it on main and files it.
- 2026-10-25 — (PRED orchestrator) **Ev ruled S29 on PR 5708**, with three amendments on the thread:
  - The rule is "as big as possible without any extra work": each lane takes the coarsest step its own pinned worst-triangle theorem allows. There is no safety factor and no δ/2 bound.
  - A = 16 is a heuristic, open to replacement by what it means.
  - The `[tolerance.chordal]` sentence was dropped as redundant, and the README was rewritten plainly.

  PR 5708 merged. Fork-log row 195 is complete (Opus A, Fable B). S29 is closed. Filed:
  - the build, `mesh-sizes-each-lane-by-its-worst-admissible-triangle` (P1 H, PRED);
  - `the-aspect-cap-stands-in-for-the-sliver-criterion` (P3, PREDTAIL).

  The build waits for disk: 4.3 GB is free with two lanes building.
- 2026-10-25 — (PRED orchestrator) Period-headroom unit 2 is PR 5710, frozen at `bf9bc61f73`, CI green. Every remaining site decides through `Period::turn`, and the certified readers read the stored turn. The 'junction-end pick precedent' was found unratified and filed on BAND. A full single review is running on that head.
- 2026-10-25 — (PRED orchestrator) Dispatched `mesh-sizes-each-lane-by-its-worst-admissible-triangle` on `pred/mesh-sizing-rule` as a **full single review** unit. Lane order: measure first; the cone rim-chord row runs on main before the change. A mis-mesh there would be P0.
