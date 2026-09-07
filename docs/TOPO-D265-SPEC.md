# TOPO-D265 — the merge door's arena-fault class, made true at the door (spec)

Item `work/topo/D265.md`. Branch `topo/d265-door-corruption-class`.
**Difficulty pre-logged M; task class STRUCTURAL** (block TOPO-B1 slot
1 — the arm was drawn with the block on 2026-09-05, before this spec,
so the difficulty guess is again recorded as made knowing the slot's
arm). Survey run 2026-09-06 against main `368089da`; every cite below
was re-derived on that head.

**One sentence.** `merge_coplanar_faces` promises that a refusal
reporting a torn arena escapes the group regime under both regimes,
and answers membership by asking the ENUM
(`EulerOpError::reports_tier1_corruption`, `crates/topo/src/euler.rs:928-968`)
— but the enum's line is "this variant means the body was already
torn", while the door's own sentence is about THIS door, where
`merge_group` (`crates/topo/src/merge_faces.rs:1227-1389`) has already
verified facts that make several NON-torn variants unreachable except
by corruption; the two questions are conflated, and the row asks which
variants the door's published sentence is false for.

---

## The claim

`MergeCoplanarError::is_arena_fault` (`merge_faces.rs:430-451`)
delegates to `reports_tier1_corruption` and says the class "is not
enumerated here … a second copy of the list in this file is exactly
how the door came to promise a rule it kept for two variants out of
nine". True as far as it goes. What it leaves out: the enum answers
`false` for `NotSameLoop`, `SameLoop`, `FaceHasRings`, `NotSameFace`,
`SameFace`, `CrossShell`, `CrossSolid`, … because each is "legal to
meet on a tier-1-valid body" — at SOME door. At this door,
`merge_group` establishes facts before calling `kev` (`:1384`) and
`kemr` (`:1389`): which of those variants can the calls still return
on a valid body, and which can only come back if the arena changed
under the door's feet (a contradiction of an established fact — a
kernel bug, exactly what the arena-fault escape exists for)? The row's
own measurement — zero arena faults in 3,033 door decisions — shows
the question is latent, not live; the sentence is what callers read.

## Phase 1 — measure, and write the table into the PR body

For each `EulerOpError` variant (27; `euler.rs:502-760`), for each
operator call `merge_group` makes (`kev`, `kemr`, and any other
operator or lookup that can raise one — enumerate them from the
function, do not assume two): can the call return it at this door on
a tier-1-valid body? Three columns, derived by reading the operator's
plan phase against the facts `merge_group` established before the
call: **(T)** torn arena by the enum's own line; **(C)** contradicts a
fact this door established (name the check, with its line); **(R)**
reachable on a valid body (name the input that reaches it). Every
variant lands in exactly one column per call, or the row says why the
call cannot raise it at all. Where the enum's `false` and the door's
(C) disagree, that is a variant for which the door's sentence is
false today. Confirm at least one such variant by EXECUTION: stage a
group that passes `merge_group`'s checks, corrupt the arena between
the check and the call in a test-only hook or by a planted mutant in
the operator, and show the door RECORDS the refusal as an inventory
skip (returns `Ok` under the recording regime) rather than escaping.
That executed case is the red-first row.

## Phase 2 — the change

The two questions get two homes, and the door asks both:

1. **The enum keeps its line.** `reports_tier1_corruption` stays what
   it is (a property of the variant, exhaustive). Do not widen it with
   door-specific facts.
2. **The door gets its own exhaustive classification of the variants
   its calls can return**, built from the Phase-1 table: an exhaustive
   match over `EulerOpError` at the door (so a new variant does not
   compile until placed here too), each arm one of {escape: torn
   (delegating to the enum), escape: contradicts an established fact
   (naming the fact), inventory: reachable (the regime's to place)},
   with the (R) arms carrying the input shape that reaches them. The
   door's rustdoc (`:434-451`, `:585-600`) is rewritten to state the
   two-part rule truthfully: torn by the enum's line, OR contradicting
   a fact this door established. The "not enumerated here" paragraph
   goes: the door now enumerates its OWN question and delegates the
   enum's, which is the one-home shape the paragraph wanted.
3. **Rows.** The executed Phase-1 case as red-first (a (C) variant now
   escapes under both regimes). A row per (C) variant is not required
   — say which are executed and which are argued, and why the argued
   ones cannot be minted. Keep `merge_faces.rs:1795-1830`'s existing
   torn/inventory row and extend it with the (C) set.

## Constraints, binding

`docs/prompts/implementer-discipline.md` in full. Fence:
`crates/topo/src/merge_faces.rs`, `crates/topo/src/euler.rs` (the
enum's docs only if a variant's doc is found false; the predicate
body does not move), the operator files only for a test-only
corruption hook if Phase 1 needs one (`euler_ring.rs`,
`euler_kill.rs` — say so). `D262` (the four predicate helpers'
silent lookups in the same file) is NOT this unit: do not touch
`planes_declared_equal`, `redundant_subdivision_vertex`,
`merged_outline_ring`, `loop_winding`. `D263` (`group_regime`'s
placeholder-reads-as-curved) is not this unit either. No trailer, no
empty commits, merge-only; merge `origin/main` before the PR and
whenever it moves; confirm the run's jobs are running and poll to
green in the foreground.

## Acceptance

The Phase-1 table in the PR body (27 × calls, three columns, cites);
at least one executed (C) case, red on the merge base; the door's
exhaustive match with every arm reasoned; the door's docs true; the
`reports_tier1_corruption` predicate and its coverage row unchanged;
hosted CI green on the full matrix; sweep receipt: every other door
in `topo/src` that asks `reports_tier1_corruption` (grep) — does it
have the same two-question shape? List them with a disposition (not
this unit unless inside the fence).

## Review

Protocol v6 dual on the frozen head. Claims: **C1** the table is
complete (27 variants × every call) and each cell's reason is true —
reviewers re-derive at least the (C) column independently; **C2** the
executed case reproduces on the merge base and is fixed at the head;
**C3** the door's match is exhaustive and no arm contradicts the
enum; **C4** no variant reachable on a valid body (R) escapes — an
escaped inventory refusal would turn a legal skip into a refused
call; **C5** the docs say only what the code does; **C6** the sweep
receipt's blind spots.
