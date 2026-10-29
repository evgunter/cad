---
id: d10-exit-walk-report
kind: issue
title: D10 exit walk report (2026-10-29, against main at 0cc1eaf9)
status: closed
opened: 2026-10-29
closed: 2026-10-29
parent: d10-exit-walk
---

The INTENT exit walk (`d10-exit-walk`, `plan.md` "Exit criteria"),
checked against main at `0cc1eaf9`. The list walked is DESIGN.md's
"What retires as the build reaches it" (State of the build, `:51-63`).
Stage 6's mechanisms (a trace and its tangent continuation, coaxiality
through one `Axis`) are out of scope.

**Counts:** 15 clauses. 13 are retired. 1 is still implemented as a
mechanism under a new reading (C4's bridging rung), and its row asks
whether that reading is D10's. 1 is retired in behaviour but leaves a
dead refusal variant. 3 companion pages are stale, plus one row of
DESIGN's own companion table.

## The superseded clauses

| # | Clause | Status | Citation or row |
|---|---|---|---|
| 1 | CONTACT-DESIGN C4's `BooleanCoincidence` on booleans | **Seat retired. The bridging rung is still implemented.** | The seat: #4496 (stage 4 E, glue on Zero) and #4641 (stage 4 F, the declare surface at every layer). `BooleanCoincidence`, `DeclaredPair`, `BooleanDeclarations` and `FacePairDeclaration` appear nowhere in the code. The rung: the glue table is still read through `PlaneIdentity::declared` → `carrier_eq::declared_reading` (`carrier_eq.rs:700`), which bridges an in-band displacement (`ContactVerdict::Bridged`, `:790`). Filed `contacthold/a-glued-pair-is-re-read-through-the-declared-rung-which-bridges-in-band` (P1, design). Name residue already filed: `contacthold/boolean-refusals-for-door-glued-pairs-keep-declaration-names` (P3), `atrest/a-declared-contacts-class-admits-continuation` |
| 2 | A mate's `ContactClass` | Retired | #5637 (stage 4 I, `atrest/mates-declare-no-contact`). `ContactClass`, `class_admission` and `Mate.class` appear nowhere in the code. ASSEMBLY A3: "A mate declares no contact" (`ASSEMBLY.md:607`) |
| 3 | C6's `Fit` as a class | Retired | #5637 (the `ContactClass` list went with I; stage 4 spec §10). `GlueClass` is `Rest`/`Continuation`/`Tangent` (`topo/src/contact.rs:38`). Interference between copies is a finding (#4560). topo README C6: "No class states a fit" |
| 4 | The undeclared-coincidence refusal | Retired | #4496 (`UndeclaredCoincidence`, every spelling). No code or Python tag carries it |
| 5 | The undeclared-contact refusal | **Retired. Dead vocabulary remains.** | Between copies: #4733 (stage 4 J, `UnrecordedContact` rows, `FindingSubject::Copies`). `topo::ContactRefusal::Undeclared` (`contact.rs:183`) is constructed only by `test_support_samples.rs:183`. Filed `contacthold/contact-refusal-undeclared-is-raised-by-no-door` (P4). The recourse still says "declare the named contact class": already filed as `atrest/the-contact-recourse-names-a-class-nobody-can-declare` (P3) |
| 6 | The undeclared-tangency refusal | Retired | #4405 (stage 4 G). `UndeclaredTangency` survives only in two test doc comments (`profile/tests/bool12r2_probes.rs:486`, `bool12_r1_probes.rs:163`). `PathError::SeamTangent`/`JunctionTangent` stay by ruling: they are the authoring lattice's corner question, not a declaration (`docs/INTENT-STAGE4-SPEC.md:443`) |
| 7 | ASSEMBLY A5's hard error on an unattributed contact | Retired | #4733. `Attribution::Unattributed` is gone. A5 (`ASSEMBLY.md:435`) reads the kept census and refuses nothing for it |
| 8 | The axis declaration channel | Retired | #4496 (E) with stage 4 C (#4628). `CoaxialEvidence`, `AxisSource` and `docs/AXIS-DECLARATION-DESIGN.md` are gone. Coaxiality is one `Axis` read twice, or a recorded Zero (stage 4 spec `:521`) |
| 9 | `ParamSource` | Retired | Stage 1: #4027 (variables replace the table) and #4146 (a slot holds a `VarId`). The type appears nowhere in the code. `param_source.rs:291` keeps only the retired tag bytes out of reuse |
| 10 | NAMES N6's recipe-source stamps (`GeomSource`, `AxisSource`) | Retired | #4496 ("provenance retires"). N6 now reads "A cell's construction is read from the document … The kernel carries no recipe provenance of a description" (`names/README.md:828`). `GeomSource` survives only in four editor-core test doc comments (`seat4_verb_lowering.rs:226`, `:431`; `seat7_sweep_lowering.rs:298`; `seat8_split_lowering.rs:223`) |
| 11 | PARAM-LINT's declared-distinct record | Retired (never built) | `docs/PARAM-LINT-SPEC.md:3` is a draft never signed off, and `:125` says D10 retires the declared-distinct arm. No code |
| 12 | The profiles' stored tangent-joint flags | Retired | #4405. `ProfileLoop.tangent_joints` is gone. The set is derived at validation (`profile/src/validate.rs:1696`, `ValidatedLoop::tangent_joints`, from constructors and junction verdicts). profile README V5/V6 state the derived set |
| 13 | ASSEMBLY A3 | Retired in code. **The page fence is stale.** | Stage 3: B #5232, C #5267, D #5506, F #5299, G #5954. The text under the fence now describes D10's placement. Filed `intent/assembly-md-fences-a3-as-the-code-before-d10` (P4) |
| 14 | A10's gathered product and sink rule | Retired | #4359 (stage 2 C) on FORK-2b (#4220). The product is a slot (`Doc::product`, `DocEdit::SetProduct`, `ASSEMBLY.md:279`). `mc.rs:469`'s "sinks" are the Monte Carlo summary's measures and assertions, not a product rule |
| 15 | A12's reading edges | Retired | #4566 (stage 2 F). A12 reads "Mate sides are reads" (`ASSEMBLY.md:141`). No code says reading edge |

## Companion pages

The pages stage 4 spec §10 names as rewritten, plus a sweep of the
companion table for "declar", "undeclared", "sink" and the retired
symbols.

| Page | Status | Citation or row |
|---|---|---|
| `crates/topo/README.md` (CONTACT-DESIGN) | **Stale** | `:51` (a declaration is recipe data), `:422` (mate declarations at rest), `:949-1010` (the "Until the intent refactor lands" fence and C4 as declared contact, though stages 4 and 5 landed), `:377` and `:1184`. Filed `contacthold/the-topo-readme-still-states-declared-contact` (P4) |
| `crates/editor-core/ASSEMBLY.md` | **Stale** | A3 is fenced as pre-D10 (`:11-14`, `:567`). It spells `InstantiatePart` without its `target` (`:574`). Filed `intent/assembly-md-fences-a3-as-the-code-before-d10` (P4). A5, A10 and A12 state D10 |
| `docs/LIBRARY-DESIGN.md` | **Stale** | U6's flush-declaration helper (`:203-210`). Filed `lib/library-design-u6-promotes-a-flush-declaration-helper` (P4) |
| `crates/editor-core/src/names/README.md` | States D10 | N6 `:828` |
| `crates/profile/README.md` | States D10 | V5 `:164`, V6 `:186` (a constructed tangency is verified, and a value-decided junction is recorded) |
| `crates/verbs/README.md` | States D10 | §3 P1 `:294` ("whether two stored radii are one is the margin's to decide (D10)") |
| `crates/editor-core/REFERENCES.md`, `CHECKS.md`, `VARIABLES.md`, `IDENTITY.md`, `README.md`; `docs/SELECT-DESIGN.md` | No superseded clause found | `CHECKS.md:13` and `SELECT-DESIGN.md:187` state the retirement |
| `docs/DESIGN.md` companion table, ASSEMBLY row | **Stale (Ev's text)** | "A3 and A5 fenced until INTENT retires them". A5 is unfenced, and A3's fence is the row above. Not edited here |

## Released rows (stage 3 close)

No "Released 2026-10-29 (INTENT)" note is on main or on any `intent/`
branch. The six rows were found by their triggers instead:
`patterns-are-index-variables` (#5954) and `intent-stage3-is-built`.

| Row | Disposition | Why |
|---|---|---|
| `scalar/the-box-driver-carries-no-part-resolver` | Open | `DriveConfig` (`drive.rs:145`) has no resolver, and `lane_opts` (`:1515`) is `EvalOptions::default()` |
| `seamrank/a-crossing-of-a-seam-edge-with-no-first-side-refuses` | Closed | The refusal is gone. `edge_reading` names the equal-pair seam (`emit_topo.rs:5127`). The witness the row asked for is unbuilt |
| `wirehold/a-pattern-count-has-no-upper-bound-at-the-loop-that-uses-it` | Open | The count is checked before the loop now, but it admits up to `u32::MAX` (`eval/members.rs:110-120`) and is not cancellable inside the node |
| `wirehold/emit-rs-shared-helpers-raise-emission-for-torn-reads` | Open | The held helpers are gone. `name_family` (`emit.rs:1157`) and `name_assemble` (`:1255`, `:1264`) raise the same class |
| `wirehold/missing-slot-stays-typed-on-the-held-placement-datum-and-pattern-readers` | Open | `need_scalar`/`need_angle` (`wire.rs:1201`, `:1209`), `instance_of` (`:2560`) and `member_of` (`:2590`) still raise typed |
| `wirehold/transform-and-pattern-drop-a-values-contact-records` | Closed | `Transform` retired (#5506), and a family carries each member's records (`members.rs:331-345`). The fired trigger was pruned from `atrest/the-in-arrival-census-check-retires` |

## Notes for the orchestrator

- DESIGN.md's state of the build (`:44-46`) still lists "the rest of
  placement (stage 3 — a constraint is still a mate's payload)" under
  *Not built*. That is Ev's text; it is not edited here.
- Six test doc comments name retired types (`GeomSource`,
  `UndeclaredTangency`), cited in rows 6 and 10. That is archaeology,
  P4. They can ride any PR touching those files, and no row was filed.
