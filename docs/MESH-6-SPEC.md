# MESH-6 — issue 897: the two uncovered S65 cases, measured

**Binding at dispatch** (S-MESH program, `docs/S-MESH-PLAN.md`;
difficulty logged pre-draw: **S**). Read
`docs/prompts/implementer-discipline.md` in full before starting.
Issue 897 is the primary specification; `docs/SMELL-SCAN-2026-08.md`
§S65 and the S-MESH Q1 ruling (`docs/S-MESH-PLAN.md` §Rulings —
Evan: the watertightness backstop STAYS COMPILED OUT; coverage for
the two named cases is discretionary) bound what this unit may ship.

## Situation

The #678 watertightness backstop (S65) is compiled out of every
build that ships a mesh, by ruling. Issue 897 names two cases the
backstop never covered even when built: the FULL-2π SEAM (a face
whose boundary closes around a full period) and CROSS-FACE
IDENTIFICATION (shared boundary vertices identified across faces).
`pole_columns` carries its own argument that `MAX_ANGULAR_STEP`
(π/4, `sizing.rs`) is a floor that makes the seam case safe. This
unit DECIDES BY MEASUREMENT whether covering the two cases is cheap
enough to add as debug-profile-only guards, or whether the honest
verdict is recorded at the site and the issue closes on it.

## Deliverables

1. **The measurement**: the cost of covering each case — what a
   guard would have to compute (per-face / per-junction work,
   allocations, whether it needs the identification map the walk
   already builds), measured on the tour corpus at the suites'
   three ε rows and at δ ∈ the budget instrument's range. Report
   the numbers, not an estimate.
2. **The `pole_columns` argument, verified or refuted**: does
   `MAX_ANGULAR_STEP` as a floor actually make the full-2π seam
   case unable to produce a non-watertight emission? Prove it (a
   row that would go red if the floor were lowered) or refute it
   with a constructed body; state which at the site.
3. **The decision, written at the site**: EITHER the two cases land
   as `debug_assertions`-only guards (compiled out of every shipping
   build — this is S65's ruled posture and the D2-row-5 shape) with
   red-first rows and the measured cost stated, OR the verdict is
   recorded at the site with the measurement and NO code ships.
   Either way the ruling's letter holds: no shipped guard.
4. **D9 discipline**: nothing this unit does may move a shipped mesh
   byte — the two-build instruments are in-tree (`r2_bytes`,
   `r1_probe_hash`, MESH-4's digest); run one and pin the result.
   A debug-only guard that changes control flow in release is a
   defect.
5. **Issue 897 CLOSES at this merge** on either branch of item 3 —
   say so in the PR (keyword hygiene: the orchestrator closes).
6. **ε posture** (issue-1356): which reads the guards (if any) make
   and at what band — through MESH-4's `Eps` operations, never a
   bare comparison (the inventory pin will red on a bare one).
7. **Class sweep** (discipline §5): the other S65-shaped
   compiled-out checks in `crates/mesh` — enumerate and disposition
   (covered / uncovered-and-why / not this issue's).

## Acceptance

- The measurement in the PR with numbers; the decision at the site;
  D9 pinned; hosted CI green; gate record per head.

## Hard rules

- NO `Co-Authored-By`, no model names. "issue 897" spelled out, no
  closing keywords.
- Scope fence: `crates/mesh` (the backstop's home, `pole_columns`,
  suites). NOT: the backstop's compiled-out STATUS (ruled), issue
  678's pole floor itself, `walk.rs` classification decisions,
  `docs/MODEL-AB-LOG.md` / `docs/S-MESH-*.md` / SMELL edits (S65's
  §D row is the orchestrator's to update at merge).
- Re-merge main before opening the PR.
