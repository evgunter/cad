# MATE-4a — issue 973(a): the face rung reaches ef_bound_backed's interior arm

**Binding at dispatch** (S-MATE program, `docs/S-MATE-PLAN.md`;
difficulty pre-logged in the plan's opening commit: **M** — the
plan's MATE-4 entry, impl half). Read
`docs/prompts/implementer-discipline.md` in full before starting.
The primary specification is the RULING — `docs/S-MATE-PLAN.md`
§Rulings item 2, part (a) (Evan, in-chat 2026-08-31) — with issue
973's section (a) as the map and #969's U-R1 (the ratified
CENSUS-REST-CLOSURE gap-1 pattern) as the precedent this extends.
**Part (b) of issue 973 — the `EdgeEdgeCross`/`EdgeFacePierce`
classes — is EXPLICITLY OUT OF SCOPE** (its design pass is a
separate conversation); this unit must leave (b)'s behavior
byte-identical and pin that.

## Situation

`crates/topo/src/census.rs`'s `ef_bound_backed` returns false as
soon as the edge holds no vertex at the bound, BEFORE any backing
rung — on the D3 licence that the subordinate vertex-on-edge event
"hard-errors independently." #969 made that subordinate event
face-backable, so the licence is hollow: the edge-on-face overlap
is now the only thing left refusing. Reachable at rest today with
plain declared geometry: a post cap overhanging the shelf's side
edge, declared `post_cap ~ shelf_underside` — every vertex event
backs, and the shelf's boundary edge diving through the cap's
interior survives as a hard `Unattributed` `EdgeFaceOverlap`.
The residue is PINNED red-when-closed by
`crates/topo/tests/m9_c1_r1_probes.rs::the_lemma_probe_declared`.

## Deliverables

1. **The rung, at existing strength**: `ef_bound_backed`'s interior
   arm consults the face rung — a declared face pair holding the
   edge on one boundary and the (interior) bound's event
   face-backed per #969's rung — at exactly the structural-
   incidence, region-unconfined strength of the existing rungs
   (strengthening it is ruled out; that was CENSUS-REST Q3's
   settled answer). State the rung's sentence in the module docs
   where the D3 bullets live, and revise the "hard-errors
   independently" licence the same way #969 revised D4's sentence.
2. **The pinned probe flips**: `the_lemma_probe_declared` goes red
   by design when this closes — re-bless it onto the overhang
   seat's new certified outcome, measuring and stating what that
   outcome IS (certified/`Ok`, or the `Uncertified` frontier via a
   different door — do not assume; the probe's site comment gets
   the answer).
3. **Red-first from the issue's own geometry**: the overhang seat
   quoted refusing on main (the hard `Unattributed`
   `EdgeFaceOverlap`), green/typed after.
4. **The (b) fence, pinned**: issue 973(b)'s executed configuration
   (the cap straddling the shelf's `y = 0.30` boundary edge — two
   `EdgeEdgeCross` findings) still refuses BYTE-IDENTICALLY; a row
   asserts it so this unit provably did not leak into (b).
5. **Class sweep** (discipline §5): the genus is #969's — "a
   boolean-lane premise ('reduction refines first' / 'hard-errors
   independently') licensing a refusal at rest" — sweep census.rs
   for remaining instances beyond (b)'s named ones; hit list with
   per-hit disposition, blind spots stated.

## Acceptance

- Red-first demonstrated; the re-blessed probe green with its new
  outcome stated at the site; the (b) fence row green; existing
  topo census/at-rest suites green (`m9_2_census_door`,
  `m9_c1_*`, `asm_r2b_*` as the oracles).
- Any refusal reach that moves classified against the D2 addendum
  (row 2 expected: the over-loud refusal of a declared legal seat
  retires) in the PR body.
- ε posture (issue-1356): census backing consults declared
  contacts under the band — state the band-sensitivity argument
  and which point gated, drawn or asked.

## Hard rules

- NO `Co-Authored-By` trailer and no model names in lane commits.
- Keyword hygiene: write "issue 973" spelled out; never a closing
  keyword before a `#`-reference. The unit does NOT close issue
  973 ((b) remains); say so in the PR.
- Scope fence: `crates/topo/src/census.rs` (+ its module docs) and
  `crates/topo/tests/` only. Nothing else — no `boolean/`
  (reduce.rs just changed under MATE-2; merge main and do not
  touch it), no `chart_region.rs`, no `editor-core`, no demos, no
  `docs/MODEL-AB-LOG.md`, no `docs/S-MATE-*.md`.
- Sibling lanes run concurrently; work only inside your worktree
  (the shared session scratchpad is off-limits); merge main before
  opening the PR and whenever it moves.
- Commit and push after every coherent unit of work (branch
  `mate/4a-ef-bound-rung`).
