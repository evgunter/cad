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
