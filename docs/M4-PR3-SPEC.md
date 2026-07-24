# M4 PR 3 binding spec — naming part 1: RolePath, name tables, the CI invariant

Orchestrator-authored binding spec (2026-07-24). Deviations REPORTED,
never improvised. Charter: M4-PLAN PR 3 = NAMING-DESIGN N1–N4 made
concrete (the doc is RATIFIED #74 and binding — re-read N1/N2/N3/N4
before coding; this spec pins only what the doc left as
implementation). Branch: `ev/m4-3-names`.

## Scope

IN: the StableName type made real (replacing PR 1's placeholder);
RolePath enums per op; eager per-node name-table emission in the
evaluation service; N2 split discriminators (covariant margined
predicates, tie marks); N3 merge retirement names; the N4 CI
name-table invariant (golden tests, f64 AND Interval). OUT:
resolution/ResolveError/Diagnosis/Rebind (PR 4); GeomSource (PR 5);
hit-testing inversion (PR 4, with resolution).

## D1 — Type home and shape (binding)

`StableName` lives in editor-core (a document-layer concept; the
kernel never sees it — kernel ops emit enough birth information for
editor-core's wiring layer to NAME entities, they do not name
things themselves). Shape per N1:
`StableName { node: RecipeNodeId, path: RolePath }` with entity-kind
phantom typing (`StableName<FaceKind>` etc. or a runtime kind tag —
choose for ergonomics + serializability, REPORT the choice).
PR 1's placeholder in Declare migrates to the real type (Declare
pairs were already StableName-shaped; keep edit-time existence
validation).

## D2 — RolePath vocabulary per op (binding; closed enums)

`RolePath = Vec<RoleSeg>` with `RoleSeg` a closed enum whose
variants are op-scoped (one enum with op-grouped variants or
per-op enums composed — REPORT the choice; versioned with the op
contracts). Required coverage, derived from how each wire op mints
entities (read the wire layer + kernel provenance before writing
the enums — the roles must be DERIVABLE from birth data the ops
already produce, or from data the wire layer can compute at mint
time without heuristics):
- Extrude: Cap(Top|Bottom), Lateral(profile-edge locator),
  RimEdge(cap, profile-edge locator), LateralEdge(profile-vertex
  locator), plus vertex roles. The profile-edge locator is the
  profile's OWN combinatorial identity (the profile crate's
  canonical-form ordering — profile edges have stable indices from
  the exact-order band; verify and cite, do not invent).
- Revolve: the M2 band/pole/seam taxonomy (read the revolve
  emitter's structure; bands indexed by profile edge, poles by
  axis-contact vertices, seam by the F13 seam role).
- Booleans: FromA(inner name), FromB(inner name), Seam(fA-name,
  fB-name) for zip-minted entities; carved fragments of an operand
  face are Fragment(parent-name, Qualifier) per N2.
- Split: Fragment(parent, Qualifier) + section-face roles
  (Above|Below, section polygon index by deterministic construction
  order — REPORT if construction order is not recipe-covariant and
  propose the covariant alternative).
- Transform: identity-preserving (names pass through unchanged —
  the transformed entity keeps its pre-transform name; the
  transform node contributes NO RolePath segment; document why:
  key-stable arenas, N1 derivation-path semantics).
- Pattern: Instance(i) wrapping the master's names (i = the
  D8-structural index).
- Datum/Profile: their own small vocabularies (Profile defers to
  the profile crate's combinatorial identities).

## D3 — N2 discriminators (binding)

Fragment qualifiers are sign vectors of NAMED trilean predicates
(`name_frag_*` family through k_stats) against recipe-covariant
references per N2 — for boolean/split fragments the discriminating
geometry is the cutting entity's own oriented carrier (side-of
plane; order-along oriented line for collinear chains). NO bare
indices, NO (u,v) values in names. Where no covariant qualifier
separates two fragments (genuine symmetric tie): both entries get
the N2 tie mark in the table; naming succeeds, and the table
records the tie for PR 4's Ambiguous error. Indeterminate qualifier
margins escalate typed (never a silent pick) — surface as the
node's Failed result, consistent with every other in-band refusal.

## D4 — Table emission and shape (binding)

- `NameTable` fills the PR 2 `NameTableSlot` stub: bidirectional
  (name → entity key; entity key → name) per node result, covering
  EVERY boundary entity (faces, edges, vertices, bodies) of that
  node's output bodies — eager per N4. Bodies get names too (Q-h:
  node + output-role).
- Emission lives in the WIRE layer (editor-core), consuming kernel
  provenance + op output structure. If any op's output cannot be
  named without heuristic matching (a role the birth data cannot
  justify), STOP and report — that is a kernel-emission gap to fix
  in the kernel (like PR 2's transform_rigid clause), not a
  guess-in-the-wire-layer site.
- Injectivity is enforced at emission: a would-be duplicate name
  (outside tie marks) is a BUG — assert loudly (this is the
  no-silent-aliasing guarantee N1 rests on).
- Content-key transfer: the table is part of NodeValue, so memo
  reuse carries names with geometry automatically — pin with a test
  (memoized reuse produces bit-identical tables).

## D5 — The CI invariant (binding, N4 verbatim)

The name table is a function of (recipe structure, structural
params, predicate verdict vector) ONLY. Tests:
- Golden test: the die document's full name table (or a stable
  digest of it) pinned; joins the replay-identity family.
- Parameter-motion invariance: continuous-param edits that flip NO
  verdicts leave every name resolving to the "same" entity
  (correspondence via the memo/scratch bit-identity from PR 2 where
  reusable, and via role-equality where recomputed).
- f64/Interval agreement: same verdicts ⇒ identical tables at both
  scalar types (run the boolean corpus doc at both, compare tables).
- Discriminator flip localization: an edit that flips exactly one
  fragment qualifier changes exactly the names whose derivations
  pass through it — counted, not vibes.

## D6 — Acceptance (binding)

`crates/editor-core/tests/m4_pr3_names.rs` (+ siblings): the D5
battery; per-op role coverage tests (every RoleSeg variant minted
by at least one test document and found in a table); the tie-mark
fixture (a symmetric boolean whose two fragments tie — verify both
marked, naming total); Declare pairs resolving against the new
tables (existence validation upgraded from node-level to
name-level where cheap — REPORT if deferred to PR 4); the PR 1/2
suites green unchanged.

## D7 — Process (binding)

OUTPUT DISCIPLINE per convention. Branch from origin/main
(≥ `8aec775`). Push after every commit. Gate: ONE synchronous
foreground `scripts/gate.sh <merged sha>` (600000ms; on cutoff READ
the output file, never relaunch); **derive any band-edge test
probes from the ambient Tolerance — never hard-code ε values (the
PR 2 lesson: the ε matrix WILL catch you)**. NEVER export RUSTFLAGS;
sccache from first build OK. Open the PR ("M4 PR 3: naming part 1 —
RolePath, name tables, the CI invariant"); do NOT merge;
adversarial review follows.
