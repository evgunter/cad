# BOOL-12 — the declared ARRIVAL at the seam: the mid-side seam and the tangent seam

**Binding at dispatch** (S-BOOL program, `docs/S-BOOL-PLAN.md`;
difficulty logged pre-draw: **M**). Read
`docs/prompts/implementer-discipline.md` in full before starting.
The primary specification is the Q1 fifth-round ruling (Evan,
in-chat, 2026-09-01; ratified PR #1539 — `docs/S-BOOL-PLAN.md`
§Rulings, "PQ4 reopened on BOOL-11's measured parity wall") together
with the REOPENED clause of PATHS §6's PQ4 entry
(`docs/PATHS-DESIGN.md`) and the BOOL-12 slate entry (widened to the
tangent seam at PR #1541 on Evan's "join-tangent-to-end"). BOOL-11's
PR #1520 record (§3, §6, §12) is the measured ground. Issue 433
context applies; the issue does not close here (BOOL-9 remains).
**Precondition: BOOL-13 has merged** — there is no schema version to
bump and no coordination to run; the wire vocabulary grows and the
corpus regenerates.

## Situation

BOOL-11 ended the seam wall's departure half: `continue_to(Start)`
closes when the seam is a CORNER. It measured, exhaustively over the
lily leaf family, that a corner seam is not always available — the
kite's corners are its tips, the rectangle's are its shoulders, the
sets are disjoint, and a loft pins one rotation for every section, so
one section always seams at a subdivision vertex. That seam is today
refused as the entry's undeclared zero-turn junction — PQ4 surfacing
through `junction_check`'s tangent arm as `SeamTangent`.

The seam is the one junction whose NEXT leg was authored first. Every
declaration that elsewhere rides the departing leg (`.tangent()` for
a G1 junction, the structural continuation for a G0 collinear one)
has no departing leg to ride at the seam: the entry's first side is
already authored, and the closing leg is the arriving one. So the
missing spelling is an ARRIVAL-side declaration, and it is one
family with two members, ruled together:

- **STRAIGHT arrival** (ruled, fifth round): the closing leg declares
  that it arrives straight into the entry's first side; the seam is a
  declared subdivision point. The kernel CHECKS, within ε through the
  funnel exactly as `continue_to` checks its target, that the entry's
  outgoing direction continues the arriving one.
- **TANGENT arrival** (Evan, in-chat 2026-09-01, "join-tangent-to-end"):
  the closing arc declares that it arrives G1 into the entry's
  outgoing direction with a sharp departure — the mirror of
  `tangent_arc_to`, and of the NURBS closer's existing tangent-seam
  form `Pn−1 := Start.pos − len_end·Start.dir`. The both-ends-tangent
  close stays the seam FILLET's (a circular arc cannot generically
  carry both tangencies).

Undeclared collinear and undeclared tangent seams KEEP REFUSING.
Nothing is inferred from a value; the declaration legalizes the
banded check as authored-data consistency (the same argument BOOL-11
made for its target, PR #1520 §1–§2), and a seam past the band
refuses typed as inconsistent authored data.

**Uniformity, stated up front.** PATHS' rule is that `Start` goes
through ORDINARY verbs, and BOOL-11's addendum collapsed a
Start-only refusal for exactly that reason. This unit adds spellings
that exist ONLY at the seam. That is not a leak of the rule: the seam
is the one junction where the arriving leg is the later-authored one,
so an arrival-side declaration has no interior counterpart to be
uniform with — the interior spells the same facts on the departure
(`.tangent()`, `continue_to`). Say this at the site and in §6; a
reviewer will ask.

## Canonical fixture (Evan's D-shape)

`(0,0) → (0,2) — arc → (0,−2) → (0,−1) → (0,0)`: a semicircular arc on
one side, one straight side from `(0,−2)` up through `(0,0)` to
`(0,2)`, with the loop's entry at `(0,0)` — a subdivision point of
that straight side. The closing leg `(0,−1) → (0,0)` arrives straight
into the entry's first side `(0,0) → (0,2)`. Today this refuses at the
seam (`SeamTangent`); rotated so the entry is a corner it authors
(BOOL-11). After this unit it authors as written, with the declared
straight arrival, and `validate` is green with `tangent_joints` empty
(the arc's two junctions are corners; the seam is a subdivision, not
a tangent joint). This is the first row, both directions: the declared
spelling closes; the undeclared `line_to(Start)` / `continue_to(Start)`
keep refusing `SeamTangent`. The tangent member's canonical fixture is
the stadium (measurement 1 below).

## FIRST, before the build — two measurements, reported

1. **The stadium today.** A stadium closed with
   `.tangent().tangent_arc_to(Start)` — the closing semicircle
   departs tangent and ARRIVES tangent to the entry's outgoing
   straight. Measure what the seam does: does `tangent_arc_to_start`
   refuse the G1 arrival as undeclared (which arm, which refusal), or
   does it pass because the seam check only classifies the straight
   case? Report the exact behaviour with the row that pins it. This
   decides whether the tangent member is a NEW spelling or a check
   that today's spelling lacks.
2. **The loop-start reading.** PQ4's recorded rationale is the
   same-carrier discipline that "germ matching and the merge ladders
   lean on" (PATHS §6). Write down what those two consumers actually
   do at the loop START — where the seam's two half-edges meet a
   carrier that is now one carrier authored as two sides: does either
   consumer distinguish a seam vertex from an interior subdivision
   vertex, does either assume one authored side = one carrier, and
   what does the interior `line(len)` ruling (which already crossed
   that discipline, BOOL-8) tell us about whether the seam case is
   different. Cite the sites. This reading rides the PR for Evan with
   the §6 revision; the ruling required it before the build.

Report both to the orchestrator before building. If the reading finds
a consumer that genuinely depends on the seam being a carrier
boundary, STOP and report — that is a design fork, Evan's.

## Deliverables

1. **The spelling — design surface, argued.** Two candidates the
   slate names: closer VARIANTS (e.g. an arrival-declaring form of
   the closing verbs: the straight closer that declares continuation
   INTO the entry, the tangent-arc closer that declares its arrival
   G1) or an ENTRY-side declaration (the loop's first directed point
   carries how it will be arrived at). Argue one for Evan's eyes in
   the PR and in the §6 revision; the never-infer ladder, the
   directed-point axiom (§2c) and the LIB-RTABLE one-row-per-verb
   invariant bound the choice. Whichever you choose, each member is
   ONE `transition_table!` row and emits through the existing kernels
   (`emit_straight_leg_at`; the tangent-arc geometry).
2. **The straight arrival**: the closing leg declares it arrives
   straight into the entry's first side; the check is the entry's
   outgoing direction against the arriving one, banded through the
   funnel (`decide` on a named key; Zero accept, escalation in the
   band, definite refusal typed past ε_input). The datum is an
   ANGLE-class quantity, so — unlike BOOL-11's lateral miss — it needs
   a LEVER to be dimension-honest (D4; §4 item 1 is the precedent:
   the turn margin levered by an arm). State the lever and the D2
   row at the site.
3. **The tangent arrival**: the closing arc declares G1 arrival into
   the entry's outgoing direction with a sharp departure. Geometry:
   the arc through the departure point and `Start` whose end tangent
   is `Start.dir` — one circle, closed form; degenerate when the
   departure point lies on the entry's tangent line (refuse typed,
   with the recourse). The departure junction classifies as any
   departure does (`JunctionTangent` if it is undeclared-tangent;
   `SameCarrierJunction` if collinear — BOOL-11 addendum). Both-ends
   tangent is NOT this verb's: if the author also declares the
   departure tangent, the refusal names the seam fillet as the
   spelling.
4. **The refusals**: undeclared collinear seam keeps refusing
   `SeamTangent`; undeclared tangent seam keeps refusing (whatever
   measurement 1 finds — if it passes today, that is a gap this unit
   closes, red-first); a DECLARED arrival whose check fails refuses
   with a NEW typed refusal naming the declared intent and the
   measured miss (the `ContinuationTargetOffRay` shape). Red-first on
   both sides of each band.
5. **Lily migrates in every rotation the loft pins**: the section
   authors through the lattice for `shoulder = 0` AND `shoulder = 1`
   at the one rotation the loft fixes, `RawLoop` and the second
   kernel dependency leave `demos/tour` (`lily.rs`, `Cargo.toml`), the
   named-gap comment retires, render byte-stable or the delta
   measured. BOOL-11's parity-wall rows (`path_property.rs`'s
   rotation-2 row, the 64-ring parity measurement) FLIP to the
   demonstration. This is the ruling's demonstration.
6. **The wire spelling, carried from BOOL-11**: `ProgramStep` /
   `WireStep` gain `ContinueTo` AND the new arrival forms; the lifting
   door's `RecordedProgramError::VerbNotInDocumentVocabulary` retires
   if `ContinueTo` was its only member (keep the door and the
   `NOT_IN_DOCUMENT` roster mechanism if another member exists — the
   roster's falsifiability row tells you); `pncad-py`'s `NotBound`
   entry for this verb becomes a bound spelling or a disclosed gap.
   Post-BOOL-13 this is an additive vocabulary change: regenerate the
   checked-in corpus through the release tour build and say so. The
   persisted tour document authors through the new spellings.
7. **§6 PQ4 revision + §4 + the verb table**: PQ4's entry re-records
   as REVISED — the declared case admissible, the mechanism, the
   spelling, the loop-start reading, the undeclared case still
   refused, the uniformity paragraph above; §4's seam paragraph and
   the verb-table rows land. **Design surface — the PR is HELD for
   Evan's sign-off.**
8. **Rows**: the D-shape both directions; the stadium both
   directions; the declared check's band pinned on both sides, stated
   as multiples of the witness's ε; the lever's dimension-honesty row
   (the threshold does not drift with leg length); lily's two
   sections closing at the loft's rotation; a `coverage_corpus` chain
   replaying the new arms at `Dual64` and `Interval`; the declared-vs-
   undeclared contrast for each member; the both-ends-tangent
   refusal naming the fillet.
9. **ε posture** (issue 1356): the new key's band story per band;
   three-ε + interval battery; the trailer decision argued (BOOL-11's
   interval-asked precedent: `Dual`'s value channel is bit-identical
   to `f64`, so the interval lane is the only one where a decision can
   differ).
10. **Class sweep** (discipline §5): every closing site in
    `crates/profile/src/path.rs` (BOOL-11 §10 lists nine) dispositioned
    against the new family; every exhaustive match over `Step` / `Verb`
    / `PathErrorKind` / `ProgramStep` / `WireStep` found by compiling;
    the #433-stance prose sites updated to "both lattice halves and
    the seam landed; raw door remains (BOOL-9)".

## Acceptance

- The D-shape and the stadium author as written and `validate`
  green; lily authors through the public surface in both sections,
  `RawLoop` gone from the demo; red-first on each member both
  directions and on the undeclared seams; the loop-start reading and
  the stadium measurement reported BEFORE the build; §6 carries the
  revision text; hosted CI green; gate record per head.

## Hard rules

- NO `Co-Authored-By`, no model names. "issue 433" spelled out, no
  closing keywords; no issue closes here.
- **The PR does not merge on green** — PATHS §6 PQ4 / §4 / the verb
  table are design surface; the PR carries the spelling argument and
  the loop-start reading for Evan's eyes; the orchestrator holds the
  merge for the sign-off.
- Scope fence: `crates/profile/src/path.rs` + `path/program.rs` and
  the profile suites; `demos/tour/src/lily.rs` + `Cargo.toml`;
  `docs/PATHS-DESIGN.md`; the wire/document vocabulary sites for
  deliverable 6 (`editor-core` persist wire + eval vocabulary,
  `switch_program_vocabulary.rs`, `pncad`/`pncad-py` rosters and
  tags — minimal arms) and the regenerated corpus. NOT: `RawLoop`'s
  demotion (BOOL-9), `arc_continue` (BOOL-10), `validate` semantics,
  `lift.rs` beyond a forward observation, the seam FILLET's own
  forms. `crates/profile` and `crates/editor-core` are SMELL track V
  fence ground — disclose any Track V row's file you reach.
- Re-merge main before opening the PR.
