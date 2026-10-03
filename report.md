# Census fold: what tier 3′'s census costs and catches on contact-free bodies

The question: should tier 3′ fold into tier 3, so that every finished body
runs the census against its own declarations, usually empty? Folding changes
what a contact-free body must pass in two ways. (a) It pays the census.
(b) Any census finding or census refusal now refuses the body. This report
measures both on real bodies.

## 0. Answer

- **Cost (a) is small on single solids and modest on multi-solid bodies.**
  Bodies that pass both gates fall into two groups:
  - Single solids: the census costs a median of **0.09×** tier 3
    (p90 0.35×, max 1.5×), about 13 µs per body.
  - Multi-solid bodies: a median of **0.44×** (p90 1.5–2.5×, max 12–17×),
    at most 8 ms.

  On the tour, the census totals 0.02 s against 16 s of tier 3 and 20 s of
  construction. Curved single solids are cheap partly because the census is
  blind there: it does not look at curved face pairs within one solid.
- **Consequence (b) bites only on non-fixture bodies, and on few of them.**
  Of 2,572 non-fixture contact-free bodies that pass tier 3 (ε 1e-9), 66
  fail the empty-contact 3′:
  - **(i) real undeclared touching that tier 3 misses: 8 bodies.** Five are
    pinch unions whose published record dropped the contact (filed already).
    One is a shell that silently built crossing walls (#1055). One is an
    Euler-op doubled face. One is a STEP file with twin solids, which the
    import already refuses through 3′.
  - **(ii) census refusals on geometry it cannot decide: 26 bodies.** All are
    `shell` results whose curved thin-wall solids are nested or sit side by
    side, and the cross-solid backstop answers `CensusUndecidable`.
    `topo::shell` gates its own result with `validate_geometric`, so under a
    fold **these shell calls would start failing**. At ε 1e-6 there are 3
    more: in-band escalations.
  - **(iii) bodies that do carry contacts, held elsewhere: 19 bodies.**
    These are boolean results whose caller gated the bare body, and assembly
    products. They are not contact-free, so a fold only means the call site
    must hand its records over.
  - **Contact-subject test fixtures that my filter missed: 12 bodies.**
  - **One ignored K-probe, at K≈1.1** (mixed (i)/(ii)).

- **The real fold cost is class (ii).** The census can't yet decide
  curved × curved or curved × planar pairs across solids. Today that hits
  multi-solid `shell` results, and it would hit any other op that emits
  several curved solids in one body. Every other non-fixture refusal is a
  defect the fold would catch, or a body whose contacts already exist.
- **DESIGN.md says the two gates agree on empty-contact results.** It isn't
  true of this population. 66 non-fixture bodies (2.6%) pass tier 3 and
  fail the empty-contact 3′ (§3).

## 1. Method

**Instrumentation.** The scratch kernel hook is
`probes/census-fold/kernel-hook.patch`. It was applied to the working tree
and is never committed to a kernel crate. With `CENSUS_FOLD_DIR` set, it
fires in two places:
- every body that passes `validate_geometric_certificate`, which is the door
  behind `validate_geometric`, `AtRestBody::validate` and
  `AtRestPolicy::gate_at_rest`;
- every body handed to `validate_pseudomanifold_certificate` with
  `ContactRecords::default()`.

For each such body it re-runs three gates, three times each, and logs the
medians, every error's `Debug` (first six), and the variant counts:
- `validate_geometric`;
- `validate_pseudomanifold(&body, &ContactRecords::default(), tol)`;
- the census alone, `census::census_and_certify` with the certified region
  lane. This is the cost an `AtRestBody` that already holds its tier-3
  verdict would pay.

**Identifying bodies.** A body is identified by a fingerprint: its Euler
counts, its vertex coordinates rounded to 1e-6, and its set of surface
kinds. Duplicates across tests are merged, and every origin is kept.

**The join.** The same patch logs the fingerprint of every body that is:
- minted with non-empty contacts (the 4 `BooleanBody` construction sites,
  and the `editor-core` product gather); or
- passed to the census with non-empty contacts.

That join separates "contact-free" from "carries contacts the caller didn't
pass".

**Runs.** Everything ran in release, with `nextest --release` and 3 test
threads:
- the `topo`, `sweep`, `verbs`, `step-import`, `editor-core` and `pncad`
  suites, all 7,014 tests, at `CAD_TOLERANCE_EPS` = 1e-9, 1e-6 and 1e-12
  (`run_suites.sh`);
- the tour, with `tour-hook.patch`, at the same three ε (`run_tour.sh`), plus
  one tour run without the hook for clean construction times.

**Measured vs estimated.**
- **Measured:** every pass/fail, variant and timing in §3–§5.
- **Estimated:** the "op time" for the tour. It is the wall time between
  scene visits, which is the construction of the stop, or of its whole stop
  group. No per-op timing exists for the suites.
- **Caveat on test failures:** the hook adds gate runs, so tests that count
  quadratures fail under it. That is 6 tests at 1e-9 and 7 at the other ε
  (`results/nextest-summary-*.txt`). Two of them are one ε row each and
  unrelated to the hook: `review_cleave_wrongarc` at 1e-6 and
  `rigid_map_near_eps_plane_nurbs` at 1e-12. Bodies logged before a failure
  are still in the data.
- **Caveat on timing:** absolute times are under 3-way parallel contention.
  Ratios are taken back to back on the same body.

## 2. Population

The population is distinct `f64` bodies that pass tier 3. Rows are in
`results/bodies-*.tsv.gz`, one per body, with origin, flags, timings and
the first error.

| ε | suites | tour | of which single-solid |
|---|---|---|---|
| 1e-9 | 5,424 | 99 | 2,575 |
| 1e-6 | 5,400 | 101 | 2,536 |
| 1e-12 | 5,417 | 99 | 2,566 |

**By crate of first origin (ε 1e-9):**

| crate | bodies |
|---|---|
| topo | 2,734 |
| sweep | 2,342 |
| editor-core | 235 |
| step-import | 101 |
| unattributed | 11 |
| verbs | 1 |

**By op family, matched on module name** (a body can count in several):

| family | bodies |
|---|---|
| sweep / extrude / revolve | 2,069 |
| split | 866 |
| boolean | 346 |
| shell | 270 |
| STEP import | 135, including the 75-file fixture corpus |
| blend | 112 |
| loft | 13 by name; most lofts are in other modules |

The tour is 53 stops. **Contact-free** here means the body was not passed
to the census with contacts and was not minted with contacts anywhere in the
run.

**Strata.**
- **"Fixture":** every origin is a test module whose subject is contacts,
  the census or assemblies. These bodies are built to touch and are gated at
  tier 3 as setup. The match is a module-name filter in `tables.py`.
- Everything else is split into single-solid and multi-solid.

## 3. Results per ε

The tables are copied from `results/tables-suites.md`. Variant columns count
bodies carrying the variant.

### Suites

**ε 1e-9:**

| stratum | bodies | pass 3′ (empty) | fail | contacts elsewhere | UndeclaredContact | CensusUndecidable | InstanceInterference | CensusEscalated | CensusUnsupported |
|---|---|---|---|---|---|---|---|---|---|
| non-fixture, 1 solid | 2,431 | 2,412 | 19 | 12 | 19 | 0 | 0 | 1 | 1 |
| non-fixture, multi-solid | 141 | 94 | 47 | 7 | 15 | 39 | 4 | 0 | 0 |
| fixtures | 2,852 | 259 | 2,593 | 83 | 2,570 | 656 | 290 | 10 | 0 |

**ε 1e-6:**

| stratum | bodies | pass 3′ (empty) | fail | contacts elsewhere | UndeclaredContact | CensusUndecidable | InstanceInterference | CensusEscalated | CensusUnsupported |
|---|---|---|---|---|---|---|---|---|---|
| non-fixture, 1 solid | 2,389 | 2,366 | 23 | 12 | 19 | 0 | 0 | 5 | 1 |
| non-fixture, multi-solid | 142 | 94 | 48 | 7 | 15 | 39 | 5 | 0 | 0 |
| fixtures | 2,869 | 265 | 2,604 | 86 | 2,580 | 662 | 292 | 16 | 0 |

**ε 1e-12:**

| stratum | bodies | pass 3′ (empty) | fail | contacts elsewhere | UndeclaredContact | CensusUndecidable | InstanceInterference | CensusEscalated | CensusUnsupported |
|---|---|---|---|---|---|---|---|---|---|
| non-fixture, 1 solid | 2,422 | 2,403 | 19 | 12 | 19 | 0 | 0 | 1 | 1 |
| non-fixture, multi-solid | 144 | 96 | 48 | 7 | 15 | 39 | 5 | 0 | 0 |
| fixtures | 2,851 | 262 | 2,589 | 83 | 2,570 | 654 | 290 | 5 | 0 |

`CensusLaneUnsupported` and `StaleContactDeclaration` never appear, at any ε.

### Tour (all three ε identical in shape)

- **91 single-solid bodies:** all pass the empty-contact 3′.
- **8 multi-solid states of the assembly (bench) scene:** 7 fail with
  `UndeclaredContact` (one also `InstanceInterference`). All 7 are class
  (iii): 6 join to the mates' declarations in the same run, and the 7th is
  the `m9_c1` seat state, declared in the suites.

The tour validates its boolean results at 3′ with their own records today.
Those with empty records (12) all pass.

### Across ε

5,306 fingerprints occur at all three ε. Of these, 5 flip pass/fail and 8
change their variant set. The non-fixture ones are:
- `sf2a_r1`: 2 bodies, refused only at 1e-6 with `CensusEscalated` on
  `pm_census_vf_residual`;
- `blend4_r1_probes::p2_slim_skews…`: 1 body, refused only at 1e-6 with
  `pm_census_ee_span`.

Everything else in §5 refuses at all three ε.

## 4. Timing

Census time as a fraction of tier-3 time, each the median of 3 runs.
`results/tables-*.md` has all strata.

### Bodies that pass both gates

The fold cost a valid body pays:

| ε | stratum | n | census/t3 median | p90 | max | census median | census max |
|---|---|---|---|---|---|---|---|
| 1e-9 | 1 solid | 2,543 | 0.09 | 0.35 | 1.31 | 13 µs | 0.9 ms |
| 1e-9 | multi-solid | 222 | 0.44 | 2.53 | 17.1 | 62 µs | 7.6 ms |
| 1e-6 | 1 solid | 2,502 | 0.09 | 0.36 | 1.51 | 13 µs | 1.0 ms |
| 1e-6 | multi-solid | 223 | 0.46 | 2.34 | 14.8 | 53 µs | 6.9 ms |
| 1e-12 | 1 solid | 2,537 | 0.10 | 0.37 | 1.39 | 13 µs | 0.7 ms |
| 1e-12 | multi-solid | 224 | 0.43 | 1.47 | 12.5 | 50 µs | 8.3 ms |

Split by geometry (ε 1e-9, all bodies including failures):
- **Single planar solids:** 0.28 median, 0.58 p90.
- **Single curved solids:** 0.07 median, 0.22 p90.

Tier 3 is the closed form on planar bodies and the certified quadrature on
curved ones, while the census does exact planar sweeps only.

### All bodies, failures included

All strata together give a median of 0.65–0.79, p90 about 8.4, and max 29–75.
The max is on contact fixtures, where building hundreds of findings costs
more than the sweep.

### Implementation choice

If the fold calls `validate_pseudomanifold` whole rather than "tier 3, then
the census over the kept verdict", the 3′ door costs:
- single planar solids: 1.47–1.51× tier 3;
- single curved solids: 1.12–1.16× tier 3.

Its battery runs the one-call path, not the composed one. The
`AtRestBody::validate_pseudomanifold` route pays the census alone.

### Against op time (tour, ε 1e-9, estimated)

| quantity | seconds |
|---|---|
| construction, unhooked | 20.0 |
| tier 3, summed over bodies | 16.4 (hollowtorus 7.2, budfillet 3.6, lily 3.3) |
| census, summed over bodies | 0.02 |

The census is about **0.1% of construction** and 0.1% of tier 3.

## 5. Classified failures (non-fixture, ε 1e-9; 1e-6 adds 3)

The classification is by producing test, read from its source. It is encoded
in `classify.py` and the result is `results/classified.md`. Per-body lists
are in `results/nonfixture-failures-eps-*.md`.

| class | 1-solid | multi | what |
|---|---|---|---|
| (i) real undeclared touching | 5 | 0 | **Pinch unions.** `editor-core::union_pinch_member_order` rows. The published record dropped the operands' contact, and the body still holds the pinch. The test pins it as `DROPPED_RECORDS` → `work/wire/a-boolean-drops-its-operands-own-contact-records.md`. |
| (i) | 0 | 1 | **`shell` builds crossing walls silently.** `sweep::shell5_r2_probes::r2_a_thin_curved_wall_shells_silently_into_crossing_walls`. Tier 3 is green, and 3′ finds `EdgeFacePierce` + `InstanceInterference`. Self-retiring row for SHELL-4's clearance certificate (#1055). |
| (i) | 1 | 0 | **Doubled face from an Euler op.** `topo::loop_reparenting_pcurve_rows::mfkrh_…`: `mfkrh` gives the plug face a new key on an equal plane, and it overlaps its parent cap (`VertexOnFace` ×8). |
| (i) | 0 | 1 | **Twin coincident solids from a STEP file.** `step-import::probe_dup` E2. The importer's own gate is already 3′ and refuses it. |
| (i)+(ii) | 1 (2 at 1e-6) | 0 | **K-probe thin prism.** `review_blend_k_rk_probes::print_rk_probe` is an `#[ignore]`d probe at K≈1.1. The extrude admitted a prism of width f·K·ε. The census finds coincidences within ε, band escalations, and `CensusUnsupported(Containment(RayExhausted))`. |
| (ii) census can't decide | 0 | 26 | **Shell results with curved solids.** These come from `sweep::shell7/8/9/10_*`. A hollow body shelled again gives nested thin solids, and multi-solid operands give curved solids side by side. The cross-solid backstop answers `CensusUndecidable` ("a curved face of one is within reach of the other"). |
| (ii), 1e-6 only | 3 | 0 | **In-band near-coincidences.** `sf2a_r1` ×2: the profile door decided the near-straight corner as definite (chord-side margin), but the census's `pm_census_vf_residual` lands in (ε, Kε]. `blend4_r1` ×1: a blend result with `pm_census_ee_span` in band. |
| (iii) contacts held elsewhere | 8 | 0 | **Boolean results with records** (`reach_continuation`, `join1_r2`, `pi_seam_and_kiss`, `m9_3_wall_door`, `m3_pr6_tier3prime`). The test gates `bb.body` at tier 3, and 3′ with `bb.contacts` passes. |
| (iii) | 4 | 0 | **Pinch unions minted with records** (the join finds a non-empty `BooleanBody`). |
| (iii) | 0 | 6 | **Assembly products** (`docm5_subject`, `node_labels`, `part_depth_bound`, `refusal_concision_chains`). The roots touch, and the touch is declared at the mate layer or refused by design. |
| (iii) | 0 | 1 | **Imported kiss with declarations** (`review_r1_tier_gate_probes`). |
| contact-subject fixtures the filter missed | 0 | 12 | `contfp_reads_arcs…` 3, `m5_pr9_boss_union` 4, `m9_2_census_door` 1, `m9_2b_r2` 1, `m9_c1_*` 3. |

### Minimal repros

`repros/census_fold_repros.rs` is copied into `crates/sweep/tests/` with a
`mod` line, and run at ε 1e-9.

- **(i)** The existing tests named above are the minimal witnesses. The
  shell one is 20 lines: a box with a coaxial cylindrical void, shelled at
  t = 0.15. Tier 3 is `Ok`, and 3′ (empty) refuses with `EdgeFacePierce`.
- **(ii)** `census_fold_repro_nested_shell` revolves a 1×2 cylinder, shells
  it at 0.2, then shells it again at 0.05. The result has 2 solids, 4 shells
  and 16 faces. Tier 3 is `Ok`, and 3′ (empty) gives
  `Err[33] CensusUndecidable` on every curved face pair.
  - Planar thin-ness is not a route here: the profile and extrude doors
    refuse anything below Kε (`…thin_prism`, `…thin_slab`).
- **(iii)** `census_fold_repro_corner_kiss` unions two unit boxes meeting at
  the point (1,1,0). The result is `Assembly`, 1 solid, 2 shells, with
  `vv=1` in its records.
  - tier 3: `Ok`;
  - 3′ (empty): `UndeclaredContact VertexVertex`;
  - 3′ with its own records: `Ok`.
- **Same-solid blindness** (`census_fold_repro_disjoint_union`). A cylinder
  and a box 1e-3 apart, unioned, give one solid with two shells, and the
  empty 3′ passes. The same pair as two solids would hit the backstop.
  Whether a fold refuses depends on solid vs shell bookkeeping, not on the
  geometry.

## 6. Census arms reachable from a contact-free body

Static reading of `census_with` (`topo::census`), checked against the
observed tally in `results/arms.md`.

| arm | reachable with empty records? | observed |
|---|---|---|
| v×v, v×e, v×f sweeps (`VertexVertex`, `VertexOnEdge`, `VertexOnFace`) | yes, body-wide over every entity | yes (non-fixture 24 / 4 / 7 bodies) |
| e×f sweep (`EdgeFacePierce`, `EdgeFaceOverlap`) | yes | yes (2 / 5) |
| e×e sweep (`EdgeEdgeOverlap`, `EdgeEdgeCross`) | yes. The crossing rung's backing consult answers `false` without records. | overlap yes (14); cross only in fixtures |
| sweep escalations (`CensusEscalated`: `pm_census_{vv_gap, ve_line_gap, ve_span, vf_residual, ef_residual, ee_span}`, `point_in_loop_side`, `bool_contact_*`) | yes, wherever geometry sits in (ε, Kε] | yes |
| `CensusUnsupported` (face unboundable, containment) | yes | yes (`Containment(RayExhausted)`, K-probe only) |
| conformal face-pair arm (`ConformalPatch`) | yes in principle: a same-key, opposed-sense curved pair inside one body | never, in about 5.5k bodies |
| `CensusLaneUnsupported` | only from the `_structural` door (no region lane). The certified door always hands one. | never |
| cross-solid backstop (`CensusUndecidable`, `InstanceInterference`) | only with ≥ 2 solids | yes: 35 non-fixture bodies undecidable, 1 interference |
| declared-record confirm arms (`StaleContactDeclaration`, `ContactContradicted`, curve/patch certifiers, crossing-rung backing) | no. They iterate the records, which are empty. | never |
| same-solid distinct-key curved pairs | not examined by any arm (module docs: "undetected until C9/C6") | n/a |

## 7. Where a fold would bite in production code

Non-test callers of the at-rest gate today:

| caller | gate | effect of a fold |
|---|---|---|
| `topo::shell` | `validate_geometric` on its result (`ShellError::NotValid`) | refuses the 26 class-(ii) shell results; catches the crossing-wall case |
| `editor-core` product gather | `gate_at_rest_kept`, tier 3 **before** the mates' declarations are minted; `assemble` then adds the census with them | the fold has to move the gate after the mint, or hand it the records, or every touching assembly refuses at the gather |
| `step-import` | per placed solid, where `per_part_gate_owed` asks: `validate_geometric`; the whole body: `validate_pseudomanifold_certificate` with its records | the whole-body gate already pays the census; the per-part gate would start paying it twice |
| `pncad-py` | user-invoked doors | no change |
| tour | per-scene gates | no change |

Boolean, extrude and loft exits run `validate_closed` only, and the document
evaluator gates no boolean node at rest. So the pinch-union defect in class
(i) passes evaluation because no tier-3/3′ gate runs there, not because
tier 3 passed it. A fold only catches it where a gate runs.

## 8. Files

All files are under `probes/census-fold/`:
- `kernel-hook.patch` and `tour-hook.patch` — scratch instrumentation, not
  for merging;
- `run_suites.sh` and `run_tour.sh`;
- `analyze.py`, `tables.py`, `classify.py`, `arms.py`, `nonfixture.py`,
  `population.py` and `export.py`;
- `repros/`;
- `results/`.

Raw JSONL was not committed. `export.py` keeps one row per body.
