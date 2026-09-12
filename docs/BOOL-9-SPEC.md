# BOOL-9 — issue 433 half (ii): the RawLoop demotion

**Binding at dispatch** (S-BOOL program, `docs/S-BOOL-PLAN.md`;
difficulty logged pre-draw: **L**). Read
`docs/prompts/implementer-discipline.md` in full before starting.
The primary specification is the Q1 ruling's half (ii) (Ev,
in-chat, 2026-09-01; `docs/S-BOOL-PLAN.md` §Rulings, quoted here
verbatim because it bounds the unit):

> `RawLoop` does not remain writable: the vertex table demotes to a
> CACHE — the materialized form intensional recipes evaluate into,
> like the kernel's other recipe→geometry seams — with authoring
> through the lattice only; in-repo writers migrate (lily onto the
> lattice through the public surface, retiring the tour's
> reach-around kernel dependency; fixtures to the lattice or a
> dev-only door per the LoopBuilder→test_support precedent;
> step-import as a materialization door marked as such): BOOL-9,
> survey-first. #433 closes when both land, with both sites' prose
> updated.

PATHS §4's companion sentence ("`RawLoop` is not an authoring door —
the vertex table is the materialized form intensional recipes
evaluate into") and the LoopBuilder→`test_support` precedent
(`docs/PATHS-DESIGN.md` ~:783; `crates/sweep/src/test_support.rs`)
are the shape. **Precondition: BOOL-12 has merged** — lily authors
through the lattice in every rotation and `demos/tour` no longer
depends on `profile`; half (i) of the ruling (BOOL-8, BOOL-11,
BOOL-12) is landed, so this unit is what closes issue 433.

## Situation

`profile::RawLoop` is the trait that lets any caller mint a
`ProfileLoop` from a vertex table (`new` / `polygon`). It is off the
PRESENTED surface (`pncad::profile` omits it) but on the kernel
vocabulary, and the tree writes through it in ~230 places across
nine crates and the demos — overwhelmingly fixtures and examples, a
handful of production sites. The ruling demotes the table to what
it is: the materialized form the lattice's emission layer and the
materialization doors (a persisted document, a STEP file) produce,
never something an author writes. `validate` stays the data checker
for materialized loops; the lattice stays the authoring checker.
The two rules never disagreed about geometry — they answered
different questions — and the prose at both sites says so at close.

## FIRST, before the build — the survey, reported

Enumerate EVERY `RawLoop` writer in the repo (`crates/**`,
`demos/**`, examples, benches, doc-tests), production and test alike,
and report a table before building: file, count, what the loop is
(a fixture square; a loft section; a persisted-document read; a STEP
face; a probe's synthetic wobble), and the DISPOSITION per the
ruling —

- **lattice**: the loop is authorable and the site becomes a lattice
  program (the migrated site must produce a bit-identical vertex
  table — measure it, do not assume it; BOOL-12's lily precedent);
- **dev-only door**: the loop is a deliberate non-authorable datum
  (a synthetic wobble, a degenerate table a refusal row needs) and
  the site moves behind the `test_support`-class door;
- **materialization door**: production code that evaluates a recipe
  or reads a persisted/imported form into the table (the lattice's
  emission layer, `editor-core`'s persist read, `step-import`) —
  stays, marked as such at the type;
- **delete**: the site is dead or duplicates a lattice fixture.

Include the count of sites per disposition and the estimated
mechanical size; if the lattice cannot spell a fixture the ruling
expects it to (a loop no verb produces), say which and why — that is
a finding for the PATHS vocabulary, not a reason to keep the door
open. Report also every OTHER raw door found on the way (a
`LoopBuilder` shim survivor, a `ProfileLoop` constructor reachable
from `pncad-py`, a `From<Vec<Vertex>>`).

## Deliverables

1. **The demotion at the type**: `RawLoop`'s constructors leave the
   public kernel vocabulary. What replaces them, naming yours: the
   materialized form is minted by (a) the lattice's emission layer,
   (b) the materialization doors, each marked as one at the site
   (`step-import`'s face loops; `editor-core`'s persisted-document
   read; anything the survey adds), and (c) a dev-only door for
   fixtures behind the `test_support` precedent's gate
   (`cfg(test)` / a dev feature — the precedent's own choice,
   stated). A downstream crate cannot mint a table by hand in a
   shipped build; the compile error is the enforcement, and a row
   proves it the way `switch_program_vocabulary`'s censuses do.
2. **Every writer migrated per the survey's disposition**, the
   lattice migrations proven bit-identical against the raw table
   they replace (a per-site receipt, or one instrument over all of
   them), the dev-door migrations behind the gate, the
   materialization doors marked.
3. **`validate` remains the data checker for materialized loops** —
   restated at `validate.rs`'s header and at the lattice's junction
   rule (PATHS §4 / `path.rs`'s module header): the lattice checks
   AUTHORING (declarations against authored data); `validate` checks
   the MATERIALIZED table (tangent_joints as data). Issue 433's
   original "disagreement" is written up at both sites as two
   questions, not one rule with two answers.
4. **The lift layer** (BOOL-12's forward observation, this unit's
   ground): `lift.rs`'s `AllJointsDeclared` ("no sharp joint to seam
   the chain at") is now false — the algebra authors the all-tangent
   stadium and `lift` refuses it. Decide: the lift seams at the
   declared seam (joint 0 carries the arrival declaration) or refuses
   with a corrected reason; red-first; the observation retires.
5. **Issue 433 CLOSES at this merge** — both halves landed, both
   sites' prose updated; say so in the PR (keyword hygiene: the
   orchestrator closes). PATHS §4's #433-stance sites re-record as
   "closed: both lattice halves, the seam, and the raw door".
6. **D9 / behaviour**: no geometry moves. The tour renders
   byte-stable (BOOL-12's `diff -rq` instrument); every migrated
   fixture's table is bit-identical; the suites' row counts change
   only by the rows this unit adds.
7. **ε posture** (issue 1356): this unit adds no comparand; state
   it; three-ε battery; the trailer decision (default lane unless a
   lattice migration touches an ε-deciding verb — argue).
8. **Class sweep** (discipline §5): every other recipe→geometry seam
   in the kernel that still has a writable materialized form
   (enumerate, disposition, do not act); `pncad-py`'s surface for any
   loop constructor; the `LoopBuilder` shim's status.

## Acceptance

- The survey reported BEFORE the build; no `RawLoop` writer outside
  the emission layer, the marked materialization doors and the
  dev-only gate; every migration bit-identical; the lift observation
  retired; both sites' prose; issue 433 closes; hosted CI green; gate
  record per head.

## Hard rules

- NO `Co-Authored-By`, no model names. "issue 433" spelled out; no
  closing keywords (the orchestrator closes).
- Scope fence: `crates/profile` (`lib.rs`'s `RawLoop`, `lift.rs`,
  the module headers; NOT the verbs' semantics), `crates/sweep`'s
  `test_support` and its fixtures, the fixture/example call sites in
  `mesh`, `stl`, `step-export`, `step-import`, `editor-core`,
  `sweep`, `pncad` (mechanical migration only — no behaviour change
  at any site), `pncad`'s façade docs, `docs/PATHS-DESIGN.md`'s #433
  sites, `crates/profile/src/validate.rs`'s HEADER only. NOT:
  `arc_continue` (BOOL-10), `validate`'s semantics, any lattice verb,
  `step-import`'s parsing (mark the door, do not move it). `crates/
  profile` and `crates/editor-core` are SMELL Track V fence ground —
  disclose any row's file reached. A migration that would change a
  fixture's geometry is a STOP: report, do not adjust.
- The PR merges on green after the dual (no design surface beyond
  the prose the ruling already ordered) — unless the survey finds a
  loop the lattice cannot spell, which goes to Ev first.
- Re-merge main before opening the PR.
