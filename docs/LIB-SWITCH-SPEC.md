# LIB-SWITCH spec — profiles-as-programs v2: the schema-v4 representation switch

Status: **BINDING, with dispatch hard-sequenced behind the G2
finisher (path.rs collision) and U8a (the Unit type).** The two
design inputs resolved on PR #263 (Evan, 2026-08-08):

1. **Program-anchored naming (approved)**: profile-entity naming
   for program loops anchors to PROGRAM-STRUCTURAL positions
   (step indices) — nothing geometric enters the index, so
   parameter edits cannot renumber, by construction. §V3 (round
   2) is the authority; SWITCH-E implements it. Read this spec's
   §6 under that resolution (the renumbering-class documentation
   collapses for program loops; the freeze doctrine remains the
   structural-edit backstop).
2. **Corpus representation (delegated; ruled here)**: corpus
   artifacts' representations are free to change as long as the
   geometry is representable. Ruling: (a) `circle_split(center,
   r, n, phase)` for closed carriers — boss migrates to it
   (Evan's lean, orchestrator's recommendation). The half-disc
   is MEASUREMENT-GATED per §5-1: if its equator vertex is not
   load-bearing for band naming, RE-AUTHOR at the clean break
   (preferred — no new vocabulary); if it is, the
   declared-subdivision step (a structural vertex, not a
   junction claim) is the fallback form. Read §5-1 under this
   ruling.

Mandate: implement docs/PROFILES-V2-DESIGN.md (RATIFIED #242; VQ1
RULED (b)-direct — chain-only v4 schema; VQ2–VQ9 recommendations
adopted). The program becomes the profile's definition; derived
segments become an unpersisted memo-layer cache; every continuous
step argument becomes Expr-bearing; SCHEMA_VERSION 3→4 as a clean
break (LQ7a, empty migration table). LB8 fold-in: U8b's per-literal
display-unit STORAGE lands inside this unit's one schema break.
Measured basis: the drafting recon of 2026-08-08 (facts cited
inline by file:line against main 41cb6c7); deviations are NUMBERED
and REPORTED; under-specified interactions are findings-back, never
silent fixes.

## 0. Discipline (absolute)

≤~150 lines per tool call; chunked reads; skeleton-first writes.
Every heavy cargo row `scripts/with-build-slot.sh -- cargo ...`,
synchronous FOREGROUND, long timeouts (≤590000), one at a time;
NEVER background or park. Clippy at default AND
`--features interval`, plus the discipline greps, BEFORE each PR.
Commit AND push per coherent chunk. NO Co-Authored-By, no model
names (blinding). Merge origin/main before opening each PR;
re-merge if main moves; confirm checks STARTED after every push.

## 1. Sequencing and footprints

- **HARD-SEQUENCE behind the G2 finisher.** As of this draft, main
  (41cb6c7) carries only G2's extraction PRs (#259/#261 —
  sugar.rs + fillet_select.rs); the G2 §3 algebra surface (path.rs
  arrival binders, rocker migration, PATHS §2b) has NOT landed
  (LIB-LOG LB3-correction: "§3 surface still to land"). PR-A
  rewrites the same path.rs impl blocks, and the wire vocabulary
  must be complete at the single schema break (LB8). Do not start
  PR-A until the G2 finisher merges.
- **PR-B sequences behind U8a** (in flight, LIB-3 slot 3): the
  display-unit field consumes U8a's unit table/parser/formatter.
  The WIRE shape of the field is THIS unit's to design (§4g); if
  U8a's unit type differs from what §4g assumes, adapt and report.
- U7-v1 (in flight) overlaps only at pncad re-exports; resolve by
  re-merging main; no shared rewrites expected.

## 2. Staging — three PRs, proposed as TWO A/B units

- **PR-A** (crates/profile only): the step vocabulary as data,
  record-as-you-lower, the replay driver, the differential pin.
  Reviewable alone as "driver semantics" (the LB1 boundary logic:
  algebra semantics vs mechanical plumbing); mergeable and
  valuable alone — it de-risks everything downstream.
- **PR-B** (editor-core): ProfileProgram, Expr-bearing steps, slot
  addressing, evaluation pipeline, schema v4 + display units,
  content keys, authoring-time checks, corpus + golden migration.
- **PR-C**: the v1→program lift tool, the parametric acceptance
  scene, per-doc disposition closeout.
- **A/B recommendation: two units.** SWITCH-P = PR-A (difficulty
  L); SWITCH-E = PR-B+PR-C (difficulty XL even alone). Different
  crates, different review competencies, an honest merge point
  between them. Single-unit XL with three PRs is the fallback if
  the orchestrator prefers one A/B row.

## 3. PR-A — steps, recording, replay driver (crates/profile)

**3a. The step vocabulary** (design-fixed, V1+V2+PATHS §2a). A
`path`-adjacent module defines `Step<T>` — one variant per
authoring verb, storing AUTHORED data only, never derived values
(§2a exactness contract 1): At(p), Angle(θ), Toward{dx,dy},
Tangent, Turn(δ), Line(len), LineTo(tgt), ArcTo{tgt,bulge},
ArcVia{via,tgt}, ArcCenter{center,tgt,winding},
TangentArcTo(tgt), Fillet{r}, FarEndTo(p) (the `.to(anchor)`
form), plus G2's arrival binders (at_on etc. — ENUMERATE from
post-G2 path.rs impl blocks at implementation; drift from this
list is expected and REPORTED, not a deviation). Targets are
`Point2<T> | Start` (structural). `Circle{center, r}` is a
one-step complete-loop program form (path.rs:1425 is the door).
ArcVia/ArcCenter are first-class steps storing their authored
points — the bulge is derived at REPLAY (path.rs:1927/1943);
storing it would re-type a computed value and kill its
parametricity. VQ5's expand-at-authoring rule applies to BUILDER
sugar only (polygon/rect/motifs expand into LineTo-class steps,
sharing Expr subtrees at the document layer); every verb that
consumes authored data is stored as itself.

**3b. Recording (record-as-you-lower).** `Core` (path.rs:901-910)
additionally accumulates `Vec<Step<T>>`; each binder pushes
exactly its own step. Closing verbs currently consume the path
and return `ProfileLoop<T>` directly (Core::build, path.rs:1009);
they change to return a pair-shaped `ClosedLoop<T> { loop_:
ProfileLoop<T>, program: Vec<Step<T>> }` (naming free) with
`From<ClosedLoop<T>> for ProfileLoop<T>` so kernel-direct call
sites adapt mechanically. Alternative terminal shapes are the
implementer's measured call, REPORTED, under two requirements:
(R1) no second spelling of any verb; (R2) one chain yields both
the lowered loop and its program. `circle()` returns loop +
one-step program the same way.

**3c. The replay driver** (V1 drift-proofing, ratified round 1).
`replay<T: Decide>(steps: &[Step<T>]) -> Result<ProfileLoop<T>,
ReplayError<T>>`. The in-flight tip is an enum over the four
lattice states, each variant holding the TYPED PartialPath value
(schematically `DynTip { Open(..), PlainPoint(..), Directed(..),
Angle(..) }`, flavors per the markers); applying a step is a
match on (variant, verb) whose arm bodies can only call the one
typed binder well-typed at that state — binder bodies never
duplicated, the lattice never re-stated as data; the only
writable mistake is a missing arm (over-strict), which the
differential pin catches. `ReplayError<T> { step: usize, kind:
Transition{state, verb} | Path(PathError<T>) }` — Transition is
the lattice-violation class (corrupt-file, reachable only from a
hand-edited wire form); Path is the geometry-refusal class,
legal at rest under some bindings (V1 class 2). A chain program
not ending in a Start-targeting verb is the Transition class; a
Circle program is exactly one step. Serde plays no role here:
the crate stays serde-free (Cargo.toml: geom-core only — G1
layering, verified at drafting).

**3d. The differential pin (mandatory).** Every typed-surface
chain in the profile test corpus, the tour authoring sites, and
a generator: the recorded program replays to a BIT-IDENTICAL
loop (verts/bulges/joints exact bits). The subtle semantics ride
along free because the driver calls the binders — pin them
anyway with dedicated rows: far-end zero-fit declaration
suppression (path.rs:1274-1276), seam-fillet retrim of vertex 0
(path.rs:1246-1258), the G2 arrival family, circle's two-pole
lowering. Zero new [[test]] binaries.

**3e. PR-A fence.** crates/profile only; path.rs + new
module(s); sugar.rs / fillet_select.rs / validate.rs semantics
untouched; no serde, no editor-core, no demos, no schema.

## 4. PR-B — editor-core: the program becomes the representation

**4a. ProfileProgram** replaces `ProfileDesc(Profile<f64>)`
(profile_desc.rs:30) as `Node::Profile`'s payload (node.rs:176).
Shape: `{ plane: SketchPlane<f64>, loops: Vec<LoopProgram> }` —
placement stays stored f64 in its own struct so the U4 seam is
visible (VQ8; WireProfile already splits plane, wire.rs:187).
`LoopProgram` = a chain step list OR a circle form, mirroring
Step; continuous args are `Expr` (dimensions per V2's table:
coordinates/lengths/radii Length; angle/turn Angle; bulge
Scalar; toward components Scalar — ratio only), structural args
literal (verb tags, winding, Start). Dimension checking at
construction, `DimensionError` typed — the node-slot pattern.

**4b. Evaluation** (V2, design-fixed). The `wire_profile` arm
(eval/wire.rs:232, dispatched :47) becomes: resolve every Expr
at f64 via `doc.param_env::<f64>()` (doc.rs:221 — the verified
asymmetry: profile geometry is f64-pinned because C6 structure
selection must be lane-identical; node magnitude slots stay
lane-live; inherited, not invented) → `profile::replay` per loop
→ `Profile<f64>` → `embed::<T>` → `validate` under the run
tolerance, exactly today's call. Requires plumbing the param env
into the profile arm (today wire_profile takes only the desc).
Errors: eval errors and ReplayError map to a new typed
NodeErrorKind variant carrying (loop index, step index);
validate errors unchanged. VQ6 CLOSED here: the replay-time
junction checks run under the evaluation's pinned tolerance —
the same `Tolerance::get()` the validate call already uses;
state it in rustdoc (F2's residue).

**4c. Slot addressing** (VQ3, sharpened). `SlotId` (node.rs:55)
gains `Profile { loop_: u32, step: u32, arg: StepArg }` — NOTE
the loop coordinate, ABSENT from the design's `(step, arg)`
sketch; a profile is plane + several loops, so the address needs
it (report as a VQ3 sharpening, not a deviation). `StepArg` =
closed per-verb role enum (TargetX/TargetY, ViaX/ViaY,
CenterX/CenterY, Length, Radius, AngleVal, TurnVal, Bulge,
DirX/DirY, …). `SlotId::dimension()` (node.rs:95) extends per
the V2 table; `is_structural()` stays false for all StepArgs.
`ExprPath` and `Expr::descend`'s u8 sub-paths reused unchanged
(expr.rs:530, :402). `edit::apply`'s SetParam/SetExpression arms
(edit.rs:635/655) and `Doc::expr_at` (doc.rs:211) gain
Profile-slot routing. Step indices are stable because program
structure changes only by re-authoring (the frozen-selection
argument; V2).

**4d. Authoring-time checks** (VQ9, design-fixed). Edits that
CREATE or MODIFY a profile program (InsertNode carrying one;
SetParam/SetExpression addressing its slots) resolve + replay +
validate under the CURRENT param env and refuse typed at the
edit door. `SetDocParam` NEVER refuses for downstream profile
breakage — that surfaces at evaluation as the node's typed error
(V1 class 2: refusing programs may exist at rest). State the
rule in rustdoc; pin both directions by test.

**4e. Content/naming keys.** The profile node's structural
payload (the desc.tokens() stream, memo.rs:879-898) is replaced
by: per loop a LoopStart tag; per step the verb tag + structural
payload tags (winding, Start, target kind) + the
resolved-at-f64 bit pattern of each continuous arg
(write_f64_bits) — the same resolved-value convention node slots
use today (memo.rs:917-924). Any edit that can change segments
changes the key (structure via tags; Exprs and params via
resolved bits; ε already in the key). Derived segment floats
LEAVE the key (V3); display units NEVER enter it (D7). NamingKey
formula unchanged.

**4f. Caches** (V3/VQ2, design-fixed). Replayed segments are the
node's evaluated payload in the existing per-node prior-
Evaluation memo (reuse condition eval/mod.rs:781-789) — NO new
cache machinery, NOTHING persisted (persist/mod.rs:38-46 already
lists the non-persisted derived set; segments join it). D9 makes
rebuild exact; load-time replay is a linear pass over tens of
steps.

**4g. LB8 display units (U8b folded in).** Per-LITERAL display
unit: `ExprKind::Literal` (expr.rs:124) gains
`display_unit: Option<Unit>` (U8a's unit type/code);
`WireExpr::Literal` gains the optional field
(deny_unknown_fields kept); U8a's parser sets it from the typed
suffix; U8a's formatter consumes it. HARD RULES (D7):
`literal_bits` (expr.rs:433) and `bit_eq` (:462) EXCLUDE it;
content/naming keys exclude it; eval ignores it; the canonical
value stays meters/radians f64. The unit round-trips through
save/load; the Doc-level metadata map (doc.rs:110-113) stays
as-is (a document-default display unit remains its business).
Per-SLOT units: NOT in v4 — additive later if wanted; report if
U8a's design pulls the other way. Acceptance: the full "25 mm"
row — parse → literal(0.025, Length, unit=mm) → persist v4 →
load → format → "25 mm"; bit_eq/key blindness pinned both
directions (two docs differing only in display units are
bit_eq-equal and key-equal).

**4h. Schema v4.** `SCHEMA_VERSION = 4` (persist/mod.rs:114);
the migration table STAYS `&[]` (persist/mod.rs:352; LQ7a) —
extend the :335-349 doc comment with the v4 entry.
`v3_golden.cad` joins v1/v2 as a SchemaTooOld REFUSAL fixture;
`v4_golden.cad` is minted via the M4_PR6_BLESS_GOLDEN flow —
this ratified spec IS the authorization its panic message
demands (m4_pr6_golden.rs:191-208); the golden() document
re-authors program-form, content-preserving. The wire program
form mirrors §4a structurally; deserialization CANNOT mint a
ProfileLoop — loops exist only through the driver at evaluation
(serde is transport, the driver is the door; wire.rs's strict
rule verbatim). The load door refuses malformed structure
(bad indices, non-finite floats) as today; lattice violations
surface at the shared validator via a replay probe under the
document's params (persist::check runs for BOTH save and load,
persist/mod.rs:373/434 — the strict door stays two-sided).

**4i. In-repo authoring migration.** 15 corpus documents
(tests/corpus/mod.rs:119-165, all DocEdit-log-authored) move to
program authoring; `fixture::desc` (tests/fixture/mod.rs:52-68)
is the choke point — it becomes a program-polygon helper, so the
polygon docs migrate centrally; ~35 ad-hoc test files migrate
mechanically. `demos/tour/src/heatsink.rs` (the ONE Doc-layer
tour scene) migrates; all kernel-layer tour scenes are UNTOUCHED
(§9). pncad re-exports the program types (module-level; prelude
curation is the implementer's call, reported).

## 5. Explicit behavior deltas — designed, pinned, reported

1. **Same-carrier splits in persisted docs are chain-unauthorable
   BY DESIGN** — the drafting recon's central finding. Boss's
   3×120° arc loop (corpus boss.rs:46-57) and the
   die_pips/die_composed two-quarter half-disc
   (die_pips.rs:150-163, die_composed.rs:189-202): consecutive
   same-carrier arcs refuse (refuse_identical_carriers,
   path.rs:1102), and `circle()` deliberately offers no split
   control (PATHS §2a: "a raw-chain question, not an authoring
   one"). These loops CANNOT migrate verbatim, which DENTS
   VQ1(b)'s "persisted corpus authors fully" premise —
   ORCHESTRATOR/EVAN EYES REQUIRED before PR-B merges (this is a
   finding-back to PROFILES-V2 §V7 VQ1, not a local call).
   Proposed disposition: boss re-authors as `circle()` and its
   rim-seam-count-dependent assertions rework; the half-disc
   re-authors as semicircle + diameter UNLESS the equator vertex
   is load-bearing for band naming — MEASURE first; if
   load-bearing, that is a vocabulary finding (an
   authored-vertex-on-carrier need), not a hack site. Numbered
   deviations either way.
2. **Programs can refuse at rest under a binding** (V1 class 2)
   — new NodeErrorKind rows pinned; the honest cost stated in
   the PR body.
3. **Profile nodes gain expression slots** (node.rs:424 flips
   from empty) — diff/edit/persist-check surfaces follow; pin
   the new slot enumeration.
4. **Naming renumbering under continuous edits** — see §6.

## 6. The naming measurement (V3's mandated verification — EXECUTED at drafting; the spec carries the consequence)

Measured: names hold indices only, never floats
(names/role.rs:95/108; names/mod.rs N2) — BUT the indices are
CANONICAL: validate.rs canonicalize_loop (:1305) rotates each
loop to its lexicographic-min vertex (:856-858, lex_less :1263)
and orients by shoelace sign. Today no DocEdit can move profile
vertices (profiles are slot-free, node.rs:424), so V3's "a
parameter edit moves vertices; it never renumbers them" holds
VACUOUSLY. Under v2 it is FALSE in general: a parameter edit
that changes which vertex is lex-min, or flips orientation,
renumbers segment/vertex indices and repoints StableNames.
Consequences (per the design's own orchestrator note — revision,
not silent fix): (a) FINDING-BACK to PROFILES-V2 §V3 before any
doc claims stable-names-across-continuous-edits; the
canonicalization/naming seam redesign (if wanted) is its own
follow-up unit; (b) the acceptance scene pins stability for ITS
edits (a circle loop's lex-min is the −x pole; radius edits with
fixed center never move it — pin by test); (c) one
demonstration row documents the renumbering class (a rectangle
whose param edit swaps lex-min) as known behavior, cited in the
finding. Canonicalization code itself is OUT of this unit's
fence.

## 7. PR-C — the lift tool (V5) and the parametric scene

- **Lift**: dev-side, in-repo, NEVER a load path.
  `lift(&ProfileLoop<f64>) -> Result<Vec<Step<f64>>,
  LiftRefusal>` (home: a dev-facing module in crates/profile —
  measured call, reported). Declared junctions → Tangent steps;
  everything else sharp LineTo/ArcTo; seam last; uses G1/G2
  modes where they fit; prefers chord/toward spellings over
  angle (W1/VQ4). Refusal classes NAMED in the enum: the
  same-carrier/closed-carrier-split class (§5-1), plus the walls
  of record. Anchor-inconsistency (F10): the tool REPORTS
  bit-identical vs value-equal per loop (PR-2's per-scene care).
- **Differential harness**: lift → replay → bit-compare vs the
  source loop; the harness doubles as the chain-coverage meter —
  a refusal census per run is the acceptance instrument for
  vocabulary growth (V5 firm).
- **The parametric scene** (V8): bodies::plate is KERNEL-layer
  (recon), so the scene is a NEW corpus document `plate_param`
  (plate outline + hole circles, Doc-layer): hole radius an Expr
  referencing a DocParam. Rows: edit param → re-evaluate → new
  geometry (volume/census pinned); r→0 → replay refusal naming
  loop+step (NonpositiveCircleRadius); r→overlap → validate
  refusal; an authoring-time refusal at the edit door (§4d).

## 8. Acceptance (executed, byte-wise)

- PR-A: differential pin battery green (record→replay
  bit-identical for every typed chain); full profile battery.
- PR-B: the whole corpus round-trips author → persist v4 → load
  → replay → validate; evaluation payloads and export trees
  byte-identical vs the merge-base (scratch-worktree diff, the
  U3/G2 method) EXCEPT the §5-1 docs (numbered deviations with
  measured dispositions); v1/v2/v3 goldens refuse SchemaTooOld;
  v4 golden byte-stable across save/load; the 25 mm row and
  display-unit blindness rows (§4g); ε rows at the three
  tolerances.
- PR-C: lift coverage census recorded (expected: polygons,
  circles, filleted and declared-tangent loops lift;
  same-carrier splits refuse with named walls); the parametric
  rows above. D9 note: U9's future cross-platform doctest gains
  a parametric profile (banked, not built here).
- Full batteries per PR: profile, editor-core (+interval), tour
  3 ε rows, sweep/mesh/step-export where the corpus touches.

## 9. The fence (V6, operationally)

UNTOUCHED: the #101 verify layer and the validate ladder
(including canonicalization — §6 is a finding, not a fix);
`LoopBuilder` (stays public, stays the kernel raw layer; kernel
tour scenes stay as-authored, incl. the PERMANENTLY-raw bowtie,
bodies.rs:403-418 — kernel-layer, never persisted, no schema
seat); junction predicates and the k_stats funnel; downstream
ops consume ValidatedProfile only, never programs; PQ4 and the
§6 rulings (circle is a program FORM, not a PQ4 relaxation — the
flagged sentence stands); sugar/fillet_select internals; names/
code; U8a's parser/formatter internals (consumed only); no CI
edits; no render/montage regeneration; no new [[test]] binaries;
serde stays out of kernel crates; NURBS legs stay out (VQ7 —
the step vocabulary carries them the day the segment layer
does).

## 10. PR discipline

Per-PR: merge origin/main immediately before opening; CONFLICTING
= no checks; confirm checks STARTED after every push. PR bodies
carry the full logical writeup (the sanitized record lives in the
PR, not commits). Reports ≤150 lines each to
~/.local/share/cad-work/lib-switch-{a,b,c}-report.md. Open, do
NOT merge PR-B before the §5-1 ruling lands. Genuine design forks
(terminal-door shape §3b, StepArg enumeration §4c, half-disc
disposition §5-1, lift-tool home §7): report, pick nothing beyond
the smallest faithful reading, flag. Findings-back: §5-1, §6, any
drift between §3a's step table and post-G2 path.rs.

## 11. Difficulty

XL as a single unit. RECOMMENDED: two A/B units per §2 —
SWITCH-P (PR-A, difficulty L) and SWITCH-E (PR-B + PR-C,
difficulty XL): the crates split cleanly, the differential pin
gives SWITCH-E a hard floor to build on, and each half gets a
review sized to its risk (driver/lattice semantics vs
schema/eval/edit surface).
