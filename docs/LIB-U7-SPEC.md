# LIB-U7 spec — structural selectors + name doors (binding)

Mandate: LIBRARY-DESIGN §L5 U7, SCOPED BY LB7 (LIB-LOG):
**structural-first** — role-path-shape selectors over the ratified
M6-5 vocabulary plus the missing façade doors. GEOMETRIC predicates
(carrier kind, adjacent-surface pairs, convexity, position) are OUT
— deferred to a designed follow-up (decided-predicate sites under
DESIGN.md's margins discipline; GQ7 interaction). Measured basis:
`~/.local/share/cad-work/u7-census.md` — read it completely; cite
it, don't re-survey. Deviations numbered and REPORTED.

## 0. Discipline (absolute)

≤~150 lines per tool call; chunked reads; skeleton-first writes;
report ≤150 lines. Every heavy cargo row
`scripts/with-build-slot.sh -- cargo ...`, synchronous FOREGROUND,
long timeouts (≤590000), one at a time; NEVER background or park.
Clippy default AND `--features interval` + discipline greps BEFORE
opening. Commit AND push per chunk. NO Co-Authored-By, no model
names. Merge origin/main before opening; confirm checks STARTED.
Other lanes are building — slot waits are normal.

## 1. Fence

In scope: `crates/editor-core` (names/ additive only), a new
selector module in `crates/pncad`, `demos/tour` (the migration
below), pncad prelude. OUT: `crates/profile` (G2's active lane!),
Node/schema/persist changes of ANY kind (the freeze doctrine and
Vec<StableName> payload are ratified — you consume them), CI,
docs/M*-*, renders. The census's naming caution: do NOT name
anything `fillet_select` (taken by profile's 2-D corner selection).

## 2. Deliverables

1. **Materializer siblings** (editor-core, beside `all_edges`,
   names/mod.rs:41-70): `all_faces`, `all_vertices`, `all_bodies`
   — same contract (filter NameTable by kind, sort, dedup; empty
   for valueless nodes), pinned like m6_5_downstream.rs does.
2. **The structural selector** (pncad, library-side sugar per G1
   layering — consumes `Evaluation` + `NameTable`, returns
   `Vec<StableName>`, ALWAYS a materializer): a small query value
   over role-path SHAPE — match on EntityKind + RoleSeg patterns
   (e.g. rim edges of a given CapEnd; Seam{a,b} where a-kind/
   b-kind match; per-op groups per the RoleSeg enum,
   role.rs:226-430). Shape it as data (a matcher enum/builder),
   not closures, so bindings can speak it later. It must be able
   to express: "the cap rim of the top face" and "every
   Seam{Cap,Band} edge" (the census's two motivating examples).
   No geometric fields anywhere in the vocabulary.
3. **Façade doors** (pncad prelude): export `NameTable`,
   `EntityKind`, `EntityRef`, `all_edges` + the new siblings,
   `edge_name`/`face_name`/`entity_name` — StableName stops being
   write-only at the façade (census §4).
4. **Migration proof**: the die_composed corpus document's 14
   HAND-AUTHORED fillet names (corpus/die_composed.rs:74-160)
   become a selector call + materialize; the resulting
   Vec<StableName> must be IDENTICAL to the hand-authored set
   (the existing resolution test keeps passing untouched). This
   is the acceptance: P10's document-layer relocation is
   actually removed for the structural case. The kernel-side
   geometric filters in diefillet.rs STAY (deferred scope) —
   add a one-line comment naming the deferral.

## 3. Acceptance

- editor-core + pncad + tour batteries green; zero geometry or
  export diffs (this unit computes names, never geometry — any
  changed export byte is a defect).
- The selector-materializer equivalence pin (deliverable 4).
- Doctest each public selector form (pncad doctest conventions
  from U1).
- Zero new [[test]] binaries.

## 4. PR discipline

One PR. Report ≤150 lines to
`~/.local/share/cad-work/lib-u7-report.md`, per-phase figures.
Open, do NOT merge. Final message: PR number + report path only.
Genuine forks: report, pick nothing beyond the smallest faithful
reading (the matcher vocabulary's exact shape is the likely
spot — keep it minimal; growth is additive later).
