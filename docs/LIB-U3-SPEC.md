# LIB-U3 spec — SectionSegments retirement (binding)

Mandate (LIBRARY-DESIGN.md §L5 U3; LQ2 RULED): retire
`SectionSegments` as an authoring surface — loft/sweep speak the
`ProfileLoop` vocabulary, one profile vocabulary for all four body
ops. Measured basis: `~/.local/share/cad-work/u3-census.md`
(executed 2026-08-08; cite it rather than re-deriving). This spec
is binding: deviations are REPORTED (numbered, with the executed
blocker), never improvised silently.

The census's structural headline, which the PR body should carry:
interior endpoint agreement is checked NOWHERE — the skin path
reads both endpoints (`segment_curve`) while the assembly path
reads `a`+bulge only (`end_profile`), so a mismatched interior
joint yields walls from one geometry and caps from another with no
refusal. Retiring the double-endpoint form closes this silent-
wrong-geometry door STRUCTURALLY (single-typed vertices make the
disagreement unrepresentable).

## 0. Output discipline (absolute)

≤~150 lines per tool call; chunked reads; skeleton-first writes;
report ≤150 lines. Every build/battery row synchronous FOREGROUND
(`local-scripts/with-build-slot.sh -- cargo ...`, long timeouts, one at
a time, read each result); NEVER background anything or park on
waits. You are the only build-running agent.

## 1. The fence

- In scope: `crates/sweep`, the section-consuming test corpora
  (sweep/mesh/step-export/step-import tests), `demos/tour`
  skinned scenes, `crates/editor-core/src/eval/wire.rs`'s
  adapter seam, `crates/pncad` re-exports.
- OUT: `tube_along_arc` (census: not a consumer); the PATHS
  algebra semantics (`profile::path`); U4's territory — exact 3-D
  path legs, pose vocabulary, `(section, place)` pairing
  (placements REMAIN parallel `&[Affine3<f64>]` arrays; census
  confirms they are cleanly separated); the step-export path-
  building use of `SketchSegment` via `segment_curve`
  (`common/mod.rs:868`) — that is U4's exact-arc-leg door; keep
  `segment_curve`/`SketchSegment` public as needed for it, note
  in the PR. No CI edits; no docs/M6-M7; no montage/render
  regeneration; no schema/persistence changes.

## 2. The shape (from the census, §3)

The internal contract is already ProfileLoop-shaped: sections die
into `NurbsCurve3` at `segment_curve` and back into
`ValidatedProfile` at `end_profile`; the `(a, b, bulge)` triple is
`(v[j].pos, v[(j+1)%n].pos, v[j].bulge)`.

- **Public doors** (`loft_geometry`, `sweep_geometry`,
  `loft_body`, `sweep_body`): sections become profile-vocabulary
  values. Whether the parameter is `&[Profile<f64>]` validated at
  the door or `&[ValidatedProfile<f64>]` is your measured call
  (report it): the door must FAIL LOUD on invalid loops either
  way, and multi-loop sections (the `Vec<Vec<..>>` outer/inner
  structure) must keep working.
- **Interior**: rewrite the `loft_geometry` inner loop to iterate
  vertices directly; DELETE the `ProfileLoop → SectionSegments →
  ProfileLoop` round-trip in `loft.rs`. Goal state:
  `SectionSegments` and its `chain_is_closed` exact-`==` check
  are GONE from the public surface (`sweep/src/lib.rs:132-137`
  re-exports removed); whether a private vestige survives inside
  `skin.rs` is your measured call per LQ2 — justify whatever
  remains.
- **`end_profile`** simplifies to identity-or-near (the end
  sections ARE profiles now).
- **editor-core seam**: `section_of` (`wire.rs:920-971`) stops
  synthesizing double-endpoint chains — it hands profiles (and
  its `(section, place)` return stays as-is for U4 to promote).

## 3. Behavior deltas — each explicitly designed, pinned, and reported

1. **Open chains become unrepresentable.** Their only consumers
   are the `SkinError::OpenClosedMixed` refusal arm and two probe
   tests (`review_m5_pr10.rs:386-433`). Retire the arm (public
   enum variant removal = clean break, LQ7a) and REWORK the two
   probes to pin the new story (the mixed case cannot be
   authored; the door signature is the proof). State this in the
   PR body prominently.
2. **Sections may now carry declared tangency.** Today
   `end_profile` builds empty `tangent_joints`, so an exactly-
   tangent section joint refuses `UndeclaredTangency`. Native
   profiles can declare. Requirement: NO corpus section changes
   validation outcome (byte-identity acceptance below); add one
   NEW test pinning that a declared-tangent section loop now
   builds (capability gain made explicit, not smuggled).
3. **The split-brain door closes.** Since the mismatch becomes
   unrepresentable there is nothing to check at runtime; pin the
   narrative with the door types themselves and say so in the PR.

## 4. Corpus migration

- 41 external call sites, 9 byte-identical `quad()` helpers, 3
  `chain()` copies, prism constants re-typed in 7 files (census
  §5). Call sites move to profile-vocabulary authoring (the PATHS
  algebra or `LoopBuilder`/`polygon` where sections are sharp
  quads — `pncad::authoring::polygon` and PR-2's `path_polygon`
  are prior art). The `quad()` clones COLLAPSE: within each crate
  use one shared helper (a test-support module per crate is fine);
  cross-crate constant dedup (the 1/16-offset table relation) is
  U6's, not yours — do not build new shared-crate machinery.
- Loft/sweep scenes in `demos/tour/src/skinned.rs` move to the
  profile vocabulary; the Gram–Schmidt frame block (:477-489)
  stays as-is (U4's P1).

## 5. Acceptance (executed, byte-wise)

- `SectionSegments` no longer nameable outside `crates/sweep`
  (grep pin in the PR body; if a private vestige remains, the
  type is not exported).
- Full batteries green: sweep, mesh, step-export, step-import,
  editor-core, profile, tour (3 ε rows).
- **Byte-identity**: tour export tree at all three ε rows AND the
  step-export fixture corpus byte-identical vs the merge-base
  (build the base in a scratch worktree inside your lane; diff
  yourself). Loft/sweep e2e volumes bit-equal. Any changed byte
  is a defect unless a numbered deviation explains why the
  geometry is provably unchanged.
- The PR-2 review's NOTE-3 rider: land ONE re-runnable
  differential row in-repo (a test that rebuilds a representative
  skinned body from profile-vocabulary sections and pins its
  volume/census against the recorded constants) so the zero-diff
  contract survives as a regression test, not just PR evidence.

## 6. PR discipline

Two PRs allowed if you find a clean seam (kernel doors first,
corpus migration second) — your call, reported. Commit AND push
after every coherent chunk. NO Co-Authored-By trailer, no model
names in commits (blinding). Merge origin/main immediately before
opening each PR and re-merge if main moves (CONFLICTING = no
checks); after any push confirm checks STARTED (`gh pr checks`).
PR body carries the full logical writeup: door-shape decision,
vestige disposition, the three behavior deltas, migration census,
byte-diff proof. Report ≤150 lines to
`~/.local/share/cad-work/lib-u3-report.md`. Open, do NOT merge.
Final message: PR number(s) + report path, nothing more. Genuine
design forks: state them in the report, pick nothing.
