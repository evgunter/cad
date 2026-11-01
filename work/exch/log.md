# EXCH log

Newest entries at the bottom; the tail is the program's live status.
Plan: `work/exch/plan.md`. A/B band 2100–2199
(`docs/MODEL-AB-LOG.md` owns every live experiment number).

## Opening state (2026-09-03)

Opened on Ev's direction (in-chat, 2026-09-03: "proceed to actually
creating these tracks with their own directories in work/") from the
2026-09 work-track proposal, `docs/WORK-TRACKS-2026-09.md`, whose EXCH section is the
charter this plan restates. Opens now. Items re-homed into this
directory at opening, by header edit and `git mv` only (ids unchanged):

- `step-import-degree-one-line-promotion` from `work/issues/`
- `step-import-curve-recognition-named-exclusions` from `work/issues/`
- `rational-patch-flux-quadrature-budget` from `work/cert/`
- `stl-header-refuses-plausible-names` from `work/lib/`
- `step-writer-hardcodes-user-header-fields` from `work/lib/`
- `epsilon-has-no-type-of-its-own` from `work/lib/`

No unit is cut and no branch exists yet. The first dispatch claims its
ordinal from the band above and records it in `docs/MODEL-AB-LOG.md`.

## EXCH-H1 cut and dispatched (2026-09-03)

First unit. `EXCH-H1` (`exch/h1-degree-one-line`) takes both halves
of `step-import-degree-one-line-promotion`: the certified line limb
in `recognize_curve` and the `ExtrudedPoint`/`PlacedSegment` rung in
`nurbs_iso_derive` — one unit, because a rung with no promoted line
has no witness and a promotion with no rung refuses strictly earlier
than today (the #327-measured result). Spec `docs/EXCH-H1-SPEC.md`
(M / NUMERIC, logged pre-draw). Recon corrections folded into the
spec rather than the plan: the zero-radius composite is prose-only
today (the limb builds it; `compose` needs no edit — radius 0 is
exact), dm1's tier rows are three cells since the 2026-08-13 audit,
and the "37 polyline carriers" is an uncommitted count Phase 1
censuses.

Seams, announced here since TRIM has no live orchestrator (opens at
CURVED's exit): `topo/src/pcurves.rs` edited at the one-arm rung seam
per both keep_outs (EXCH dispatched first; TRIM consumes the rung);
no edit under `geom-core/src/spline/` (S-CERT's glob — unit 2's
derivative channel will be filed as an S-CERT row when that unit is
cut).

Block EXCH-B1 drawn at dispatch; the draw record stays branch-side
(`exch/b1-block`) until the block concludes. Ordinal 2100 to be
claimed on main at review dispatch. Option-surface design (`[ev]`
items 4–6) is in discussion with Ev in-chat; no `[ev]` PR opened yet.

Cross-program note at dispatch: main's tour row was red from SHELL's
`Shelled` return with the fix (#1770) in flight — inherited, not
EXCH's; the lane merges main when it lands.

## EXCH-H1 re-scoped at Phase 1 (2026-09-04)

The lane executed Phase 1 as bound and stop clause 2 fired with the
right evidence: the spec mis-cited the blocker. Measured on the lane
(full battery differential against merge base): `nurbs_iso_derive`
needs no new arm at all — the missing limb is `run_iso_checks`'
seam-class `Curve3::Line` carrier limb in
`geom-brep/src/pcurve_cache.rs` (TRIM's file, Track Q rows riding),
without which the promotion regresses a first-class native arc-prism
round trip. Orchestrator's ruling, recorded as
`docs/EXCH-H1-SPEC.md` §"Re-scope at Phase 1": the announced TRIM
seam extends to exactly that one limb (the keep_outs'
whichever-dispatches-first rule in spirit; TRIM dormant until
CURVED's exit), Q's rows untouched. M / NUMERIC unchanged,
re-logged here. Phase-1 yield worth naming: dm1's degree-1 census is
now a measured table (37 carriers, √sup ∈ [1.9e-10, 6.2e-9] m
against eps_in 1e-5), and `#389`'s gap mechanism is a
control-order-reversed wall column — the adoption Line-column
candidate (the spec's disclosed contingency) is what hands it a
candidate.

## EXCH-E1 cut and dispatched in parallel (2026-09-04)

While EXCH-H1's fix lane works its coarse-band red, the E tail
starts: `EXCH-E1` (`exch/d343-typed-payloads`) executes Track U's
`D343` with its two riders — disjoint files from H1
(`error.rs`, `step-export/lib.rs`, `writer.rs` against H1's
`recognize_curve`/`adopt`/`pcurve_cache`), so the lanes cannot
conflict. E build, single style review, no A/B row (FILLET's E1–E3
the precedent; dispatched opus, model choice free outside the
experiment). H1 process note for the record: the first implementer
was killed by the account's 5h usage limit mid-fix and its resume
wedged on an orphaned build; a fresh same-arm lane finished the
takeover with the predecessor's uncommitted diff preserved as
evidence (`exch-h1-predecessor-wip.patch`) — annotate on the A/B row
at merge. Unit-2 spec recon runs in the background against main.

## EXCH-H1 dual concluded — adjudication (2026-09-05)

Both reviews delivered on frozen `431c6ba40`: **R1 MERGEABLE 0/3/3**
(rubric 4/5/3/4/5), **R2 MERGEABLE 0/3/3** (rubric 4/4/4/4/5).
Verdicts CONVERGE; no MAJOR either arm. **Both reviews were
interrupted by account usage limits and resumed — the pair is
EXCLUDED from the v6 item-3 tally per the fair-pair rule 3(e)
(recorded, not tallied; no candidate existed anyway).** Two
isolation events disclosed (R2: a doc-grep dumped one unrelated
A/B row, no arm info; R1: process-table command lines of the
sibling lane, no findings text) — adjudicated harmless, flagged on
exposure per protocol.

BILATERAL, both executed independently: the PR mutant table's
INV-C4 row is FALSE (dropping the excursion channel greens the full
suite; R1 proved hull ≥ excursion structurally on clamped knots) —
the channel is dead weight under INV-C5 with a load-bearing-sounding
doc. Also bilateral in substance: the eps_in/ambient two-dial corner
(R2 reasoned + R1 executed probe: a column bent between the dials
loses its only candidate — latent, conservative-refuse, dm1 safe
structurally) and the Greville-hull copy class (3 production homes +
2 test re-derivations; natural home is S-CERT's fenced glob).
UNILATERAL R1: no red row for the unit-weight gate (its P2 is the
missing row, adopted), the self-referential re-pins (roundtrip/m7_3
branch on line_promotions on both sides), the full-column hull
superset + pl.y premise notes, the duplicate STEP parser in
r1_dm1_probe. UNILATERAL R2: four stale #389-gap sentences, the
tautological re-export assert, chord ≤ eps_in as an unstated closed
class, the census header contradiction.

Fix pass (implementer-inherited arm), union adjudicated: (1) delete
the INV-C4 excursion channel with docs corrected (both arms' proof);
(2) adopt R1's P2 rational-gate row; reconcile the two unit-weight
spellings or argue them; (3) sweep the stale sentences (the four
#389 ones, census header, r1_dm1_probe:43); (4) anchor the
re-pins with absolute promotion counts and fix the tautological
assert; (5) state the chord ≤ eps_in closed class and the pl.y
premise at their sites; (6) file the owed items: two-dial-strand
class (exch), circle-limb map obligation (exch, refs unit 2),
nurbs_iso_derive:683 wrong-channel mis-mint and arc-rim frontier
(work/issues/, TRIM territory), Greville-hull shared-home class
(work/issues/, S-CERT coordination flagged). DECLINED with reason:
the forward-first degenerate-match withhold (defended at the site;
a behavior change without a consumer), the r1_dm1_probe parser
extraction (its comment names the copy; a test-support home is its
own cleanup). Fix pass dispatches at the next usage-window reset;
merge, state-sync and the A/B row follow it.
## Announced seam from PROPS (2026-09-06): every STEP fixture's `DIRECTION` `u_ref` records re-bless with the sign-hull unit

`docs/PROPS-SIGN-HULL-SPEC.md` (branch `props/sign-hull`) changes
`Vec3::orthonormal_basis` to cross the normal with a decided world axis
(Ev's option-1 ruling on #1944). Every stored `u_ref` changes, so every
`u_ref` `DIRECTION` record in `crates/step-export/tests/fixtures/*.step`
re-blesses once, each with a locus-invariance receipt (origin and
normal bit-identical) in the PR; `step-import/src/recognize.rs:228`
is re-read for an assumption about the old frame, not re-spelled.
Announced by the spec §Seams. Signed (PROPS orchestrator).

## Announced seam from TOPO (2026-09-14): the import door marks the origin channel's import arm

TOPO's `geom-source-absence-conflates-four-origins` (branch
`topo/geom-source-typed-absence`) makes provenance absence say which
absence it is: `topo::GeomOrigin` is a total read beside the N6
`GeomSource` maps, with arms `Recipe`, `Imported`, `KernelDirect` and
`Cleared`. `Imported` has exactly one producer that can write it —
`import_step`, the only door that knows a body came out of a file —
so `crates/step-import/src/lib.rs` gains one call,
`body.mark_imported()`, on the `StepImport::Solid` arm after the
materialization loop, with the comment saying why it sits there
(each copy's `transform_rigid` has nothing to clear on an adopted
description, so no `Cleared` trace precedes it). `Wireframe` carries
no `Body` and is untouched. Nothing else in the crate moves, and no
`GeomSource` is written: N6 decides exactly what it decided.

This is step 1 of the sequence
`work/exch/step-import-discards-the-entity-ids-that-are-its-identity-channel`
is step 2 of — the arm that row fills with real content is the one
this call writes. The mark is a unit variant today; giving it a
payload is that row's business.

Signed (TOPO implementer lane, `geom-source-absence-conflates-four-origins`).

## Addendum from TOPO's fix pass (2026-09-14): the stamp over an import is lossy

The typed-absence unit's review pass collapsed the two maps into one
`topo::GeomOrigin` row per description, and that makes one direction
explicit that EXCH's step 2 will be the first to reach.
`Body::set_surface_source` and its siblings write `GeomOrigin::Recipe`
over whatever the description carried, **including `Imported`** — the
import fact is then gone for good, and a later `clear_geom_sources`
leaves the description `Cleared`, never `Imported`. That is the right
precedence (a recipe is the finer identity), it is unreachable today
because nothing in the tree stamps an adopted body, and it is stated
at `set_surface_source`'s doc and characterised by
`crates/topo/tests/geom_origin_rows.rs`'s
`stamping_an_imported_body_erases_the_import_fact`.

`step-import-discards-the-entity-ids-that-are-its-identity-channel` is
the row that puts content in the `Imported` arm. When it does, a taker
that also wants an adopted body to survive the recipe layer's stamp
has to say what an `Imported` description carrying a recipe source
means — the current type cannot hold both, by construction.

Also changed in the same pass: `Body::mark_imported` now marks the
`KernelDirect` arm only, leaving `Cleared` and `Recipe` alone (a
public door that turned the defect arm into a legitimate origin
re-opened the hole one door over). `import_step`'s body is entirely
`KernelDirect` at the call, so the shipped behaviour is unchanged, and
the 15-line comment at that call is now five.

Signed (TOPO fix-pass lane, `geom-source-absence-conflates-four-origins`).

## Option surface fully ruled (2026-09-17)

`stl-header-refuses-plausible-names` closed won't-fix with the
record (Ev, in-chat — the library door and wide sniff stay; the
demos keep the loud panic as evidence; the caller-side fallback
pattern is recorded in the item for whenever a consumer needs it).
With C13/#741 (no ε type, 09-04) and C14/#742 (STEP header fields
wait for a use case, 09-03) this closes all three D items — the
option surface LIB held for two weeks is now fully ruled, none of it
needing an implementation lane. EXCH's remaining slate: the H1
fix-pass landing in flight, E1's fix landing in flight, then units
2–3 and the E tail.
## EXCH-E1 landed (2026-09-17)

PR [#1854](https://github.com/evgunter/cad/pull/1854) merged at the
full-matrix-green head `065fc3ee3`. D343 executed over both STEP
crates with both riders (the closed_shell is_empty guard; the
UnsupportedCurve refusal test made red-capable via printable_carrier);
two conversions were live-panic fixes on a public door, the eight
arena-key spellings LEFT with the fire-before-emission argument, and
the review's kfmrh correction completed the face-killing door census.
Single style review, adjudicated from its notes across two
usage-limit interruptions; fix pass absorbed ~7350 commits of drift
including one silent KnotVector::unit_segment API break caught by
re-running the suites. Row `D343` closes with the unit; the
export-naming residue is scheduled as
`step-export-refusals-cannot-name-entities`.

## EXCH-H1 landed (2026-09-17)

PR [#1798](https://github.com/evgunter/cad/pull/1798) merged at
`6ebcef1fd`, hosted green at the full twelve-point matrix. Degree-1
line promotion is in: the recognition limb (INV-C3 composite +
INV-C5 Greville map obligation; the INV-C4 excursion channel deleted
at fix per both reviewers' executed proof), the banded wall-column
candidate, and the seam-class Line limb. `#389` holds a candidate;
on the merged tree main's check-7 change had made it dm1's
every-band refusal, so this unit moved dm1's frontier to the arc-rim
`MapResidual` at all three bands — re-pinned with the measured
values. The dual's union landed in full (six items, two argued
declines); five residue items filed. Unit and parent issue closed;
the spec is deleted per the doc ledger. The A/B row (ordinal 2100)
is recorded in `docs/MODEL-AB-LOG.md` at merge, with the
process-incident annotations; block EXCH-B1 slots 1–2 stay banked
branch-side.

## Post-gap reorientation and EXCH-E2 cut (2026-09-17)

Measured against main after both landings: the rational-flux
refusal is retired (TCOST-K3's check-7 sign certificate — not the
dial, which is PROPS' and unturned), so dm1's only remaining
refusal is the arc-rim MapResidual — TRIM's ground on both files,
its fix already filed in work/issues/ (consumer note appended;
EXCH wires the flip's pins when TRIM lands it). The compose glob
and the M7-6 lane are PROPS' since sweeps 7/11 — EXCH's keep_out
corrected (two stale fences, the moot ε clause dropped, tcost/tint
test-glob fence added), and unit 2's derivative-channel /
tensor-hull rows file with PROPS when cut. Unit 3's item carries a
premise correction: route 2 survives but its ownership conversation
is PROPS-shaped. The one scoped, unblocked, unambiguously-EXCH item
is FIX's re-homed coherence half — cut as `EXCH-E2`
(`exch/e2-coherence-consumer`), E build, dispatched now on an opus
lane (outside the A/B rows; EXCH-B1 slots 1–2 stay banked for the
next kernel units).

## EXCH-E2 landed (2026-09-18)

PR [#2837](https://github.com/evgunter/cad/pull/2837) merged at
`31d4a1268`, full matrix green twice (implementation and fix pass).
The chart-coherence channel is wired measured-first: monomorphic
call, three-state design, hazard priced (absent through this door),
not-a-gate proven bitwise. Single style review MERGEABLE; the fix
union landed whole including the reviewer's pre-named class fix
(both test-support copies homed) and one honest deviation (the
distinct-metres falsifier premise measured false; a door-order pin
shipped instead). Cross-program residue at merge: TESS summoned for
the coherence types' Display gap (second data point: it keeps
StructureRead out of the prelude); MESH notified their corpus
blind-spot sentence is now false. The E tail is done; EXCH's
remaining slate is units 2–3, both PROPS-entangled, plus consuming
TRIM's dm1-frontier row when it lands.
- 2026-09-28 — Seam note from ENCL: PR 3351 (merged `e39a5c4cc4`) routes certification refusals per D4 ¶1 as Ev ruled on PR 3352. Recourse belongs to the decision (`CertCheck::ending()`, one table) and the reading belongs to the door: `geom_brep::certify::recourse(check, RefusedArm, Reading::{Build, AtRest, Adopt})`. `CertifyError`/`PlaneNurbsRefusal` `Display` is now payload-only. Each door appends `ending(reading)`. The `certification: ` prefix is gone. step-import `Adoption` and `Assembly` render at `Reading::Adopt`: no loosen, no tighten (no ε_in lever), `KERNEL_OR_FILE_DEFECT_ENDING` on definite contradictions and residuals. `Placement`/`Instance` stay at Build. tier_gate re-pinned, with `assert_adoption_reading` as a guard. The adoption ladder certifying at the kernel `tol`, not ε_in, is filed as `work/encl/adoption-certification-has-no-eps-in-lever.md` and goes to a design fork. (ENCL orchestrator)
- 2026-09-28 — Seam note from ENCL: PR 3382 (merged `9bf495c768`) adds `geom_brep::recourse`, the one table for sized decisions. `Reading`/`RefusedArm` moved there from `certify`, alongside `SizedPass`, `SizedDecision` and `Classified`. certify and the offset meters both route through it. The shared unreadable-margin note now reads "an unreadable or collapsed margin may indicate a kernel bug worth reporting". `step-import/src/error.rs`: `Reading` path only. (ENCL orchestrator)
- 2026-09-28 — Seam note from ENCL: PR 3392 (merged `277dcb052b`) splits certify's conflated Zero/Negative verdicts: `IntervalNotForward { verdict }`, `WindingExceeded` routed as sign-certain, the tangent tube as its own `CertifyError::TubeNotSeparated` (`CertCheck::TangentTube`, lever alone at every reading), and `TubeStraddles { verdict: Refused }`. `RefusedArm::ZeroOrNegative` is deleted; `geom_brep::recourse` now holds `Refused` and `Definite`. A zero span stays a defect at every reading (Ev, e1600790f9). step-import `tier_gate`: the poleband cells re-pin to `IntervalNotForward{Zero}` with the defect ending; see `[ev]` PR 3380 for the import reading. (ENCL orchestrator)
- 2026-09-29 — From ENCL, on Ev's ruling on `[ev]` PR 3380: filed `adoption-rebuilds-caches-at-eps-from-eps-in-interpretation` on this slate at **P1/H** (Ev: "the real fix should be filed as p1"). This is D7's unbuilt rebuild stage: interpret at ε_in, rebuild caches at ε. `step-import-eps-in-ambient-two-dial-strand` and `step-import-adopts-frame-directions-on-eps-in-…` are its symptoms. Until it lands, D4 ¶1's set-ε-to-ε_in stopgap covers the message; ENCL implements it in `adoption-certification-reads-as-at-rest-with-the-eps-in-stopgap`. If Ev prefers this as its own program, it moves. (ENCL orchestrator)
- 2026-09-29 — Seam note from ENCL: PR 3418 (merged `3094222a13`) implements Ev's ruling on `[ev]` PR 3402. `Decide::sign_within` returns `Decided { sign, margin }` on every outcome. `k_stats::decide` still returns `Sign`, and its sibling `decide_reported` returns `Decided`; the two share one classify and one log write. `MarginDiag` is now opaque and for error reporting only: no variant to match, no field, no ordering, no f64 conversion. The recourse wording comes from `sized_recourse`, and the only numeric door is `diagnostic_f64_for_error_text()`. `scripts/gates/reporting-margin-door.sh` pins door calls, mints, `sized_recourse` callers and `terminal_sliver: true` per file. `Indeterminate` gains `terminal_sliver`, decided at classify time. If your code matched `MarginDiag::Value/Enclosure/Invalid` or read its numbers, it now uses `kind()`, `is_invalid()` or the error-text door; this PR touched those sites mechanically. (ENCL orchestrator)
- 2026-09-29 — Seam note from TOPO: PR 3467 (`topo/sense-reads-same-chart`, not yet merged) implements Ev's D1 ruling (PR 3480): `FaceSurface::New { surface, sense }` and `Shared { key, sense }` state the new face's bit; on the parent's chart `mef` derives the parent's bit and `mfkrh` its negation, and a contradicting stated bit is refused (`EulerOpError::SenseContradictsChart`); `set_face_surface` takes the same spec and `set_face_surface_and_sense` is gone; `Body::mvfs` and `Body::mfkrh_plug` take the seed's provisional bit. Paths: `step-export/tests/export.rs`, `step-export/tests/m5_pr13_curved.rs`, `step-import/src/adopt.rs`, `step-import/src/assemble.rs`, `step-import/src/lib.rs`, `step-import/tests/cert_n2r2_consumer_probes.rs`. `adopt.rs` states `same_sense` in the spec; the `mvfs`/`mfkrh_plug` placeholder calls state `true`, provisional until the surfaces attach. (TOPO implementer)
- 2026-10-01 — **Seam note from PROPS (PR 3524, `props/convex-insert`, DUAL review, not yet merged): the plane-NURBS seam's ε boundary moved from 1e-9 to 1e-13.** `geom_core::spline::compose`'s `insert_once_ring` goes from the lerp form to the convex form `c_{i−1}·β + c_i·α` with `β` derived from the knots, which closes site 5 of PROPS' `f64-refinement-inside-an-enclosure-has-five-more-sites`. The seam's certified between-samples sup is assembled over a Bézier decomposition, so it carried the lerp form's compounding insertion width, and it fell about two orders. Found by a nightly dispatched on the branch (run 36820204785): the default-eps gate was green, the 1e-12 row was not.
  - `crates/sweep/tests/m8_4_intersection_iso.rs` `seam_at_eps` (three rows ride it): sup ~6.22e-12 → **5.4680709999176037e-14** m, guard `eps >= 1e-9` → `eps >= 1e-13`, refusal guard `eps < 1e-9` → `eps < 1e-13`.
  - `crates/sweep/tests/review_probes_m8_4.rs` `probe_e_reversed_chart_takes_the_backward_candidate`: sup ~6.22e-12 → **5.472048160373179e-14** m, same guard move.
  - `crates/step-import/tests/recognize_pins.rs` `the_integral_mixed_body_imports_first_class_with_a_charted_seam` and `the_mixed_arc_prism_imports_first_class_over_the_intersection_pcurve_arm`: sup ~6.2e-12 / 6.3156e-12 → **3.5528237131349995e-14** m, same guard move; the `6.3156e-12` quoted in the payload comment goes with it.
  - Boundary measured by driving each row at `CAD_TOLERANCE_EPS` 1e-12, 1e-13 and 1e-14 in release: first-class at 1e-13 and coarser, typed refusal at 1e-14 with a number strictly above ε. The refusals are unchanged in KIND and still carry their own number; only where they start moved.
  - **The residue, filed rather than absorbed**: `work/tint/the-plane-nurbs-seam-refusal-cell-is-below-every-gated-eps-row.md`. `nightly.yml`'s eps matrix is default / 1e-6 / 1e-12, all now on the first-class side, so the refusal cell these rows exist to pin is exercised by nothing the gate or the nightly runs — the 1e-12 row reached it before. Each row's prose now records that; the durable fix (a 1e-14 matrix row, or a fixture-driven refusal) is the owners' call.
  - **The recommendation, made explicit so the owners can act on it (coordinator's ruling, 2026-10-01).** PROPS files this rather than fixing it: adding a nightly ε row spends another program's CI budget on their gate, which is not PROPS' call. What PROPS would do is **option 1 — one line in `nightly.yml`'s eps matrix (`for eps in default 1e-6 1e-12` → add `1e-14`)**, which restores the refusal cell for every row of this shape at once rather than four at a time. `1e-14` is the known-clean candidate: PROPS drove `m8_4_intersection_iso` at 1e-12, 1e-13, 1e-14, 1e-15 and 1e-16, and only 1e-15 and 1e-16 fail (for a reason unrelated to this change and not investigated). Option 2 (a fixture whose sup misses at every gated ε) is more durable and more work; option 3 (prose only) is what the PR did as a stopgap.
- 2026-10-01 — From CLEAVE: claimed `a-placed-step-instance-with-a-plane-nurbs-edge-refuses-at-transform` (moved to `work/cleave/`, parent `graft-recertifies-through-the-narrow-lane`) and closing it with PR 3678. That PR makes `transform_rigid` read the plane × NURBS lane from `AtRestPolicy`, and its row `crates/step-import/tests/placed_m7_8_instance.rs` is the test the item asked for: a placed instance carrying the M7-8 class imports at head, and refuses at `Placement` with the lane taken out of the transform. No step-import source changed. Evidence for `no-public-door-reaches-a-body-carrying-the-m7-8-class` added to that row. (CLEAVE lane)
- 2026-10-08 — Seam note from ENCL (`encl/adoption-at-rest-eps-in`, row `adoption-certification-reads-as-at-rest-with-the-eps-in-stopgap`, Ev's ruling on `[ev]` PR 3380). This touches step-import. `StepImportError::Adoption` and `Assembly` each gain a `file: geom_core::FileCoincidence` field: the resolved ε_in, read against the run's `Tol`. It is threaded from `import_step` through `build_one_solid`, `assemble_solid`, `Builder` and `adopt::finish`. Both variants render through `topo::EulerOpError::render_in_file`, and `geom_brep::recourse::Reading::Adopt` is deleted. The effect is that certification at import reads as at rest:
  - A band-decided size within ε_in withholds the tighten offer and says the file does not state that size.
  - A residual's miss within ε_in but beyond ε names setting ε to ε_in as a stopgap.
  - The `Adoption` preamble says a lever applies to the source model, which is then re-exported.

  `tier_gate` is re-pinned. `RefusedEnding` is gone, and `assert_adoption_reading` now checks each tighten offer against the file's ε_in. Exhaustive patterns on the two variants in your tests now read `..` or bind `file`. Filed on your slate: `step-import-placement-pcurve-and-rim-refusals-skip-the-import-doors-reading`. (ENCL implementer)
- 2026-10-08 — Seam note from ENCL (PR 4331, `encl/adoption-at-rest-eps-in`, merged): `Reading::Adopt` is gone, and adoption certification reads as at rest. At the import door, the file's ε_in picks the words, for error text only (`geom_core::FileCoincidence`; `MarginDiag::sized_recourse_in_file`; `FileCoincidence::miss_recourse_in_file`; `geom_brep::certify::recourse_in_file`; `SizedDecision::recourse_in_file`).
  - A size at or below ε_in reads "This {size} is below the file's declared coincidence distance ε_in = X m, so the file does not state it", or "may lie below … may not state it" where only the nearer end is. It ends in the lever plus "re-export … declared below {near} m and tighten below {near/K} m" where a value exists.
  - A miss within ε_in names re-exporting, plus the set-ε-to-ε_in stopgap.
  - `CertifyError::ResidualExceeded` now carries `margin: MarginDiag`.
  - The `reporting-margin-door.sh` gate pins the two new sentence functions; recourse.rs is at 5 sites.
  - DESIGN.md D4 commitment 3 is reworded (the magnitude carrier moved). (ENCL orchestrator)
  - EXCH: your row `step-import-placement-pcurve-and-rim-refusals-skip-the-import-doors-reading` now also lists `TierInvalid` (topo validate's sized endings reach import without the door's words). (ENCL orchestrator)
- 2026-10-09 — Seam note from ENCL (PR 4457, merged at `dcff36e784`): the import-door twins `ending_in_file`, `render_in_file` and `recourse_in_file` are removed. The one body now takes `at: impl Into<ReadAt>`, so the import door is the same call without the suffix: `err.render(file)`, `err.ending(file)`, `decision.recourse(arm, file)`. An in-flight branch that calls a `*_in_file` will fail to compile at merge; drop the suffix. `Unsized::residual_in_file` and geom-core's `sized_recourse_in_file` / `miss_recourse_in_file` keep their names. (ENCL orchestrator)
- 2026-10-10 — Seam note from ENCL (PR 4475, merged at `61977302ae`): a poisoned margin's note now depends on the door that reads it. At a build it is "an unreadable margin may indicate a kernel bug worth reporting" ("or collapsed" is gone). At rest or at the import door it is "…a kernel or file defect worth reporting". `geom_brep::recourse::unreadable_margin_note(reading)` picks it. geom-core's `SizedWords` gained `unreadable: &str`, and `Indeterminate::ending_noted(levers, unreadable)` exists beside `ending`. An at-rest reader of a nested refusal composes its own ending (fork-log row 9): it does not add `ending(Reading)` to the type. `topo::props::SHELL_ROLE`, `ShellClassifyError::arm()`, `JoinUndecided::diag()` and `JOIN_SUBJECT`/`JOIN_LEVER` are public for that. (ENCL orchestrator)
- 2026-10-10 — Seam note from ENCL (PR 4504, merged): `geom_brep::recourse::Unsized::LastResort` is renamed `Unsized::Fit`, with the same behaviour, and a new `Unsized::Bound` ends a refusal on a certified upper bound in the kernel-limit last resort at every reading. At the import door, `MissReading::Bound` names the bound. `CertCheck::bounds_a_miss()` answers whether a check is a bound. A table on your ground that refuses on a bound, not a measured miss, should take `Bound`. (ENCL orchestrator)
- 2026-10-11 — Seam note from ENCL (PR 4538, merged): C2 limb 2 (`ssi_hull_sup`, `ssi_hull_sup_chart`) now subdivides its Bernstein composite before refusing. It stops when the whole-carrier bound clears ε, when a break value is past the band, after two rounds that set no new lowest bound, or when the budget runs out (`SSI_HULL_WORK` 4096 spans recomputed, `SSI_HULL_CUTS` 1024). It reads the limb on the lowest round's hull. Two new refusals: on the plane × NURBS arm, `SsiLimb::HullValueChart` → `CertCheck::PlaneNurbsHullValue` (Bound ending: the carrier against its pcurve's surface point bounds the miss from above), and on the analytic arm, `SsiLimb::HullValue` → `CertCheck::AnalyticBreakResidual` (Fit). Refusals that used to read as a loose hull bound can now read as a measured miss, or certify. (ENCL orchestrator)
- 2026-10-11 — Seam note from ENCL (PR 4612, merged): seven geom-brep Displays now say "is undecided" where they said "is too close to call": edge_nurbs, newell, nurbs_iso, offset, offset_meters (now "whether this face can be offset is undecided"), and ssi's `Escalated` and `CertificateEscalated`. Tests that assert those sentences must use the new words. (ENCL orchestrator)
- 2026-10-11 — Note from ENCL: main is red on step-import `nurbs_import::arc_loft_natively_computes_its_rational_volume`. ENCL bisected it to NURBS #4553. The cause is a face-order dependence in `fold_runs` that #4553 exposed; #4553 itself is not wrong. Filed `work/exch/arc-loft-round-trip-volume-differs-by-face-order.md` (P1) with the mechanism and two repair shapes. (ENCL orchestrator)
- 2026-10-11 — Note from ENCL: PR 4682 merged at `d46cdaa15b` (repair (b) of `arc-loft-round-trip-volume-differs-by-face-order`, test-only, to unblock main). The arc-loft volumes now agree within faces−1 ulps and the pad stays bit-identical. The row stays open for FLUX's repair (a). If you'd rather have a different shape, it's your test: change it freely. (ENCL orchestrator)
- 2026-10-11 — Seam note from ENCL (PR 4629, merged): `geom_core::Indeterminate::ending` now carries its margin kind's lever. A straddling enclosure that is not a terminal sliver gets "subdivide the parameter box for a tighter enclosure, or …". An unreadable margin with no reader's note (a build) gets "check the operation's inputs upstream, then …". `ending_noted(levers, note)` (at-rest and import readers) keeps the reader's note and adds no inputs lever. `under()` is byte-identical except on a terminal sliver, which no longer offers subdivision. Whole-escalation renders take the one undecided shape: `cause.undecided(subject, cause.ending(recourse))`. (ENCL orchestrator)
- 2026-10-12 — Seam note from ENCL: PR 4874 merged (`encl/fitted-escalated-keeps-decision`).
  - `pcurve_cache`'s `FittedEscalated` keeps its SSI decision (`SsiDecision {Trace, Limb}`) and ends through `SsiDecision::ending`, which takes `impl Into<ReadAt>`.
  - Its Display and `PcurveCertifyError::Escalated`'s now end in the decision's own labelled recourse at `Reading::Build`, so the doors that print the Display (`PcurveMintError::Certify`, step-import, `replace_face`, loft, blend) carry one ending each. Both texts are pinned literally.
  - `PlaneNurbsRefusal::ending` ends `Escalated` through `SsiDecision::Limb`.
  - Rows filed:
    - PCERT `a-villarceau-tangent-parts-gated-zero-ends-as-a-kernel-defect`;
    - CLEAVE `chart-pcurve-and-props-escalations-lose-their-check-at-split-join-and-point-in-solid`;
    - `work/issues/describe-maps-every-chart-pcurve-escalation-to-chart-image`;
    - ENCL `unsized-recourse-takes-a-gate-tagged-arm-and-renders-no-gate-word`.
  - Pointers were added on the PCTAIL coincidence-menu row, the CHROME escalations-forwarded-whole row and the EXCH step-import row. (ENCL orchestrator)

- **2026-10-12 — seam note from NURBS: PR 4904 merged** (one rational surface bound: `geom_core::spline::rational_stretch_sup`, `NurbsSurface::stretch_sup`, `RationalPairTerm`).
  - `step-import` `recognize.rs` `derivative_sup` is deleted. `enveloped_residual_sup` reads `NurbsSurface::stretch_sup`.
  - The bound is ≤ the old recipe A per term, so certification only gets easier. Recognize probes p7, p9 and p10 are sensitive to it. — (NURBS orchestrator)
- 2026-10-13 — Seam note from ENCL: PR 4925 merged. ε now has a floor: geom-core `tolerance::SESSION_BOX_EXTENT` = 1 km and `EPS_MIN` = box · 2⁻⁵² ≈ 2.2e-13 m, per D4 ¶4 (Ev, PR 4836).
  - Every door that sets a run's ε refuses below it. `tolerance::EpsFloor` is the one wording of the refusal.
  - `Band::offer_deciding(m)` is the one offer rule: no tighten offer at or below ε_min. Tests that predict an offer should call it rather than restate `|m|/K`.
  - At CI's 1e-12 row, every zero-band offer now withholds.
  - A run below the floor (for example `CAD_TOLERANCE_EPS=1e-14`) is refused. Outside geom-core it silently falls back to the default; that is filed on SCALAR. (ENCL orchestrator)
- 2026-10-13 — Seam note from ENCL: PR 4943 merged. A sized decision's ending now depends on one classification of the reading, `MarginDiag::shows` (geom-core): Offer, NoSize, Floor, Unresolved, Past or Unreadable.
  - The decision's note (`SizedWords::no_size`, renamed from `at_zero`/`otherwise`; geom-brep `NoSize`, renamed from `AtZero`) appears only on NoSize: an exact zero, the refused side, or a theorem.
  - A positive reading below the floor gets the one `floor_note` sentence.
  - A reading that touches or straddles zero gets the lever alone.
  - The import door gives its sentence plus the lever, and never a decision note.
  - PR 4855's rejected-sign filter is gone.
  - New sized decisions take `no_size:`.
  - Tests that predict an ending should call `MarginDiag::sized_recourse`, not restate the rule. (ENCL orchestrator)
- 2026-10-13 — Seam note from ENCL: PR 5003 merged. geom-core now has `EndingKind { Sized, Lever(&'static str) }`, with `EndingKind::end` and `Indeterminate::ending_by(kind, named)`, as the one by-kind dispatch for an escalation ending (lever-only through `Indeterminate::lever_ending`; missing row gives `MissingRecourse(named)`). profile `validate` and editor-core `decision` call it. `check_rigid` now refuses a non-finite linear entry as `RigidMotionError::NonFinite(MotionPart::Linear)` before reading any margin. (ENCL orchestrator)
- 2026-10-13 — **Seam (ENCL):** PR 5029 merged. In `crates/step-import/tests/tier_gate.rs`, the import-door constants are renamed `*_UNSTATED`/`*_STATED` → `*_UNTOLD`/`*_TOLD`. Their sentence now reads "is within … so the file cannot tell it from zero", and the WITHHELD tally counts "might not tell". (ENCL orchestrator)
- 2026-10-14 — **Seam (ENCL):** filed `step-import-reads-a-negative-surface-radius-unchecked-before-edge-certification` here. `entities.rs` reads cylinder, sphere and torus radii with no sign check, so a negative radius reaches edge certification before check 1. It bears on `[ev]` PR 5134. (ENCL orchestrator)
- 2026-10-14 — **Seam (ENCL):** filed design question `can-an-import-reach-a-state-only-a-kernel-bug-otherwise-reaches` here at Ev's request (`[ev]` PR 5134: "can imports reach otherwise unreachable invalid states / when/how are imports validated", to be its own design question). It carries the adoption survey and needs the designer pair. `[ev]` PR 5134 waits on it. Ev says the same question came up on another PR; please link it. (ENCL orchestrator)
- 2026-10-14 — **Seam (ENCL):** PR 5138 merged. `CertCheck::ParamSpan`'s lever now reads "move the geometry so this edge runs forward over a clear length", one word longer. Re-baselined the four `tier_gate.rs` span constants. (ENCL orchestrator)

- 2026-10-23 — **Seam (QUAD):** PR 5589 (`quad/measure-a-lanes`, measure door unit A) re-words `step-import/src/lib.rs` and two tests, where they named the budget exit. No behaviour change in the import. (QUAD orchestrator's lane)
- 2026-10-23 — Closed `torus-rim-mint-abandons-a-half-applied-split` as moot: `full_torus` was deleted in `28b96a9dd6` (the one-face torus adopts as stated).

- 2026-10-24 — **Seam (QUAD):** PR 5628 (measure door unit B) touches step-import's tests in your territory:
  - `nurbs_import.rs` loses its `Escalated{props_quad_converged}` posture, since no convergence predicate is minted any more, and reads `TargetUnreached::bracket` as the plain field it now is;
  - `tier_gate.rs` drops the dead `QUAD_CONVERGED_ESCALATED` constant;
  - `tcost_k3_import_certificate.rs` counts face-lane rounds.

  No import verdict moved. (QUAD unit B lane)

- 2026-10-26 — **Seam (QUAD):** measure door unit C (`quad/measure-c-no-default`) touches this program's ground. Every public number door now takes a required predicate and answers `topo::MassMeasurement { enclosure, stop }`: `topo::mass_properties(body, tol, settle)`, `SignCertificate::measure(settle)`, `HeldCertificate::measure(body, settle)` (step-import's `StepImport::Solid { certificate, .. }` replaces `enclosure`), and Python's `Body.mass_properties(settle)`, `validate_geometric_measured(settle)` and `ImportReport.measure(settle)` (`pncad.Settle`, `pncad.MassMeasurement`). `PropsError::QuadratureBudget`, `TargetUnreached`, `VolumeReading`, `refine_to_target`, `PropsCheck::Converged` and `QUAD_TARGET_LEN_FACTOR` are gone; an unmet resolution is a bracket with `stop` `Floor`/`Wall`, not a refusal. `ShellClassification`'s four number fields are one `enclosure: MassProperties`. Tests reading a number pass `topo::test_support::pinned_resolution(tol)` (1024·ε, where their pins were taken). (QUAD unit C lane)
- 2026-10-25 — **Band rule (work/README.md, Priority):** `import-normalizes-the-rim-only-cap` re-banded P3→P0, it gates `restfront`'s `validity-refuses-an-interior-chart-singularity` (P0), which lands after it or import starts refusing files it accepts today. Legacy `cost: D` re-priced `M` with `design: true`: whether the seamed statement (`seamed.step`) is accepted, normalized or stays refused is still this row's question. (band-rule triage)

- 2026-10-29 — **Split along semantic seams** (Ev approved the division in chat). EXCH measured 133 budget points against 30 (122 once the 25 legacy `D` and unpriced rows were re-priced, which this split did). Rows moved by `git mv` with ids and bodies unchanged: 9 to INTERP (D7: the file read at ε_in, caches built at ε), 10 to FOREIGN (what a foreign face becomes at the door; holds the P0), 9 to REIMPORT (native bodies out and back, and the writers), 10 to IMPORTGATE (what the door validates and how it refuses). EXCH keeps the 9 recognition and promotion rows. Path citations `work/exch/<id>` outside the logs were re-pointed. The D10 refactor hold, applied here for the first time: `the-per-instance-tier-3-gate-reads-every-assembly-face-twice` is parked on `d10-one-way-to-say-intent-is-unbuilt` (IMPORTGATE); nothing else on this ground is D10's, and the two rows a D10 change may have moved are noted in REIMPORT's and INTERP's plans. Drive-by: `step-import-reads-area-signs-raw-outside-decide` carried its priority and cost in its body, not its header. (EXCH orchestrator)

- 2026-10-29 — **Seam (FOREIGN):** `[ev]` PR 6014 ruled the wrap tip at a chart singularity (one wrap edge, walked out and back; DESIGN `[topology]` seam clause and tiers 2/3) and, for import's C⁰-knot split, option (i): promotion first, the exact split, per-span quadrature schedules. Rows filed or updated on your slate from it are listed in that PR's body; the wrap-tip rows are parked on `work/issues/loop-readers-each-derive-a-corners-turn-with-no-shared-loop-walk.md`, which lands first (Ev: keep the complexity down). (FOREIGN orchestrator)

- 2026-10-29 — **Seam (FOREIGN):** re-banded your row P0 at Ev's word: FOREIGN's draft PR 6013 (the C⁰-knot split at the import door) waits on QUAD's per-span quadrature row, and EXCH's promotion row is its companion. (FOREIGN orchestrator)
