# BOOL-1 — issue 1152: coplanar-split section boundaries cite non-adjacent surfaces

**Binding at dispatch** (S-BOOL program, `docs/S-BOOL-PLAN.md`;
difficulty logged pre-draw: **S/M**, recorded numeric M). Read
`docs/prompts/implementer-discipline.md` in full before starting.
Issue 1152 is the primary specification.

## Situation

`topo::split` at a plane coplanar with a face yields a `below`
product whose section-boundary edges carry `Intersection`
descriptions naming surfaces that are not the edge's two adjacent
faces — tier 3 reports `DescriptionNotAdjacent` ×3. Reproduced
byte-identically on main; pre-existing (the pcurve collapse changed
no `split` code).

The issue's stated lead (a lead, not a diagnosis — verify before
building on it): `describe_section_boundary` in
`crates/topo/src/splitting/finish.rs` (fn near line 350, called near
line 301) upgrades a section-boundary edge to `Intersection` on a
transverse dihedral, and its smooth arm is empty ("Smooth: the
conventional chord stays (D2)", near line 446). On a face-coplanar
cut the section boundary lies IN an existing face, so the pair a
citation would name is not the pair the edge ends between — the same
empty-arm genus P-1b's `extrude` fix just closed (read that fix,
PR 1107's thread and diff, before designing this one).

The reproduction is already committed:
`crates/sweep/tests/p1b_r1_probes.rs`,
`coplanar_split_products_carry_no_scaffold_at_rest`, landed
`#[ignore = "pre-existing topo::split defect, filed as #1152 …"]`
with "Un-ignore it when #1152 lands" at the site. The battery missed
the defect because `m3_pr3_split::notched_block_end_to_end`'s
coplanar row asserts tier 2 only.

## Deliverables

1. **Root-cause first**: establish whether the citation is stale
   (wrong pair recorded) rather than merely absent (the issue's own
   first question); state the mechanism in the PR before the fix.
2. **Fix** `describe_section_boundary` (or the true site the
   root-cause names) so a face-coplanar section boundary's edges
   carry descriptions consistent with their adjacent faces — the
   honest description class per the current taxonomy, not a special
   case. The empty smooth arm either gains its correct behavior or
   its comment gains the argument for why empty is right and the fix
   lands elsewhere.
3. **Un-ignore the probe** (its own site instructs it) and show it
   green; keep its body as-landed unless the fix's contract
   genuinely differs, in which case say so in the PR.
4. **Upgrade `notched_block_end_to_end`'s coplanar row to tier 3**
   so the class cannot re-enter silently.
5. **Class sweep** (discipline §5): the genus is "empty
   classification arm, faithful no-op under the old taxonomy" —
   sweep `splitting/` (and `finish.rs`'s siblings) for description
   upgrade paths with unhandled arms; hit list with per-hit
   disposition in the PR body, blind spots stated.

## Acceptance

- The un-ignored probe green; the tier-3 row green; the tier-3
  `DescriptionNotAdjacent` ×3 signature demonstrated red on the old
  code in the PR body (it is the committed repro — cite its run).
- Existing split/boolean suites green.
- ε posture (the issue-1356 discipline): say in the PR which CI
  lane/ε the gate drew, and whether the unit argues
  band-independence or pinned a lane with a `CI-Config:` trailer on
  the head commit.

## Hard rules

- NO `Co-Authored-By` trailer and no model names in lane commits.
- Keyword hygiene: write "issue 1152" spelled out; never a closing
  keyword before a `#`-reference. The orchestrator closes the issue
  after merge.
- Scope fence: `crates/topo/src/splitting/` (the fix),
  `crates/topo`'s own split test suites, the one `#[ignore]` line in
  `crates/sweep/tests/p1b_r1_probes.rs`, and the
  `m3_pr3_split` tier upgrade. Nothing else — no `boolean/`, no
  `census.rs`, no `geom-brep`, no `docs/MODEL-AB-LOG.md`, no
  `docs/S-BOOL-*.md`, no SMELL table edits (no §D row names this
  defect).
- Any refusal minted or changed is classified against the D2
  addendum in the PR body.
