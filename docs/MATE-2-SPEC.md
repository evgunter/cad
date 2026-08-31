# MATE-2 — issue 1032: declared cylindrical Rest without a planar Rest beside it

**Binding at dispatch** (S-MATE program, `docs/S-MATE-PLAN.md`;
difficulty pre-logged in the plan's opening commit: **M**). Read
`docs/prompts/implementer-discipline.md` in full before starting.
Issue 1032 is the primary specification — its MEASUREMENT (four
spellings, the 3-arc face structure held constant) stands on its
own; its mechanism paragraph is explicitly a hypothesis, not a
diagnosis.

## Situation

A declared `Rest` contact on a cylindrical carrier does not reach
the rest lane when it is the mate's ONLY contact: the union refuses
`CurvedPierceUnsupported` in the reduction's curved-face arm before
any patch is discovered. M9-3's acceptance fixture passes only
because its planar `Rest` at the rim plane is silently load-bearing.
The class is ordinary — a shaft in a bore with no shoulder, flange,
or seat — and the issue's spelling (4) control shows the identical
mate working when the planar declaration is present.

The issue's hypothesis: the incidences that refuse are rim circles
of one operand's own PLANAR face (the annulus) lying on the shared
cylindrical carrier; the declared pair covers (cylinder wall ×
cylinder wall), not (annulus × cylinder wall), so the coverage test
in `curved_face_arm` (`crates/topo/src/boolean/reduce.rs` — M9-3
PR-A's declared-Rest rung) does not consider the incidence covered.

## Deliverables

1. **Measurement to the line, FIRST.** Diagnose the actual refusing
   site and the actual uncovered incidence on the issue's spelling
   (3) fixture (the one with fixture (i)'s own face structure).
   State the mechanism in the PR before any fix; if it contradicts
   the hypothesis, the fix follows the measurement.
2. **The fix, per the measurement.** The issue offers two shapes and
   the C8 posture decides between them:
   - **(a) preferred if sound**: widen the declared-Rest coverage
     test so an incidence lying ON a declared shared carrier is
     covered regardless of WHICH incident face the declaration
     names. The soundness argument must be stated at the site: the
     declaration was verified for the carrier pair, and everything
     the widened rung admits is still verified downstream — say
     exactly by what.
   - **(b) if (a) is unsound or grows past M**: the typed refusal at
     the declaration-validation door naming what is missing (the
     planar-contact requirement stated out loud, with recourse) —
     honest at the door rather than one stage downstream in a
     frontier that says something else. If (b) lands, file the (a)
     design conversation as an issue rather than absorbing it.
3. **Rows from the issue's own four spellings**: red-first from
   spelling (1) or (3) (quote the refusal from main in the PR),
   green after; spelling (4) (the control) pinned unchanged; the
   full-engagement spelling (2) covered or its residue stated.
4. **The lily pin** (`demos/tour/src/lily.rs`, wall probe 12 —
   "thread the corm onto the stem's foot"): it pins this refusal
   LIVE. If your fix flips it, update the pin to its new certified
   outcome (or its new refusal type under (b)) with the reasoning at
   the site, and SAY SO prominently in the PR — the pin is
   VERBS-measured ground and the orchestrator will coordinate the
   flip on the away channel. Do not restructure the scene.
5. **Class sweep** (discipline §5): the genus is "a coverage test
   keyed by the declared pair's face identities where the covered
   OBJECT is the shared carrier" — sweep the reduction and gate
   lanes for sibling coverage tests with the same keying; hit list
   with per-hit disposition, blind spots stated.

## Acceptance

- Red-first demonstrated from main; the new rows green; the control
  spelling unchanged; existing boolean/sweep/topo suites green.
- Any refusal minted, retired, or re-typed classified against the
  D2 addendum (row 2 is the expected class) in the PR body.
- ε posture (the issue-1356 discipline): the fixtures run at
  `Tol::witness()` and the coverage decision consults declared
  contacts under the band — say which CI lane/ε gated, whether
  drawn or asked (a `CI-Config: lane=interval` trailer on the head
  commit is appropriate if the fix's arms are band-sensitive), and
  what the band-sensitivity argument is.

## Hard rules

- NO `Co-Authored-By` trailer and no model names in lane commits.
- Keyword hygiene: write "issue 1032" spelled out; never a closing
  keyword before a `#`-reference. The orchestrator closes the issue
  after merge.
- Scope fence: `crates/topo/src/boolean/` (the reduction's coverage
  test and, under shape (b), the declaration-validation door),
  `crates/topo` and `crates/sweep` TEST files, and the one lily
  wall-probe-12 pin if it flips. Nothing else — no
  `geom-brep` (VERBS' germ ground), no `census.rs` (other units'),
  no `solid_contain.rs`/`splitting/` (S-BOOL's), no `editor-core`,
  no `docs/MODEL-AB-LOG.md`, no `docs/S-MATE-*.md`.
- Commit and push after every coherent unit of work (branch
  `mate/2-cyl-rest`).
