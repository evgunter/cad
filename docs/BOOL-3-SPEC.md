# BOOL-3 — issue 1011, the torus arm: point_in_solid learns ray×torus

**Binding at dispatch** (S-BOOL program, `docs/S-BOOL-PLAN.md`;
difficulty logged pre-draw: **L**). Read
`docs/prompts/implementer-discipline.md` in full before starting.
Issue 1011 is the primary specification — the TORUS half, which
closes the issue at merge. BOOL-2 (PR #1425, merged) is the arm
pattern and the in-file precedent; read its cone arm, its suite,
and its ledgered review record before designing.

## Situation

`point_in_solid` still refuses `Torus` as `KindUnsupported`. The
issue's shape for this half: ray×torus quartic with a **certified
root-count posture at run tolerance** (tangential/grazing being the
hard half — the sphere closed-group discipline), and TWO chart-trim
windows (the torus's chart has two angles where the cone had
azimuth+height). The two red-on-landing probes in
`sweep/tests/verbs_gate_r1_probes.rs` are torus-shaped and flip in
THIS unit's PR (the file is VERBS-authored; the orchestrator posts
the coordination note — you make the flips and cite it).

## Deliverables

1. **The ray×torus arm**: the quartic solved with a certified
   root-count posture — the answer is trusted only when the root
   count is certain at run tolerance; uncertain counts ESCALATE
   typed, never guess. Grazing/tangential escalates; the two
   chart-trim windows (both angles) follow BOOL-2's
   `chart_azimuth_margin` pattern. State the degenerate-quartic
   ladder (leading coefficients vanishing as the ray aligns with
   symmetry axes) explicitly at the site, each arm typed.
2. **One home for the group scan FIRST** (the recorded BOOL-2
   review debt, scheduled to this unit): `wrapped_cone_group` is
   `closed_sphere_group`'s scan re-derived with rims exempted, and
   the torus needs a third copy. Extract the shared group-membership
   core (surface-key group, rings check, mate-adjacency closure)
   into one home with the per-kind differences as parameters, port
   the sphere and cone callers onto it (behavior-stable — their
   suites are the gate), THEN build the torus group on it. If the
   extraction turns out not to fit the torus's needs, stop and
   report before writing the third copy.
3. **Probe-offset derivation**: reuse BOOL-2's measured shell-law
   pattern (the √(K·ε·extent) discipline with a guard row that
   re-measures) rather than minting a fourth per-suite spelling;
   derive the torus's own shell if its conditioning differs, with
   the measurement.
4. **The two verbs_gate pins flip in this PR**: read their current
   state on main first (after BOOL-2 they assert the torus-band
   refusals), flip them to the admitted-and-answered behavior the
   arm makes true, and keep their narrative honest. The donut pin
   likewise re-reads.
5. **The refusal retires as D2-row-2**, classified in the PR; the
   `Nurbs`/`Approx` refusals stay. **Issue 1011 CLOSES at this
   merge** — say so in the PR (keyword hygiene still: the
   orchestrator does the closing).
6. **Own rows**: interior/exterior at both radii regimes (spindle
   vs ring torus if both are mintable through public doors — if
   spindle tori are not mintable, record that and cover ring tori),
   the four-root ray (through the hole), grazing-escalates, both
   trim windows, and a consumer-unlocking row red on main by
   `KindUnsupported`.
7. **ε posture** (issue-1356): quartic root certification is
   band-sensitive by construction — state the bands, three-outcome
   rows, trailer decision on the final head (BOOL-2 pinned
   lane=interval; follow its argument or improve it).
8. **Class sweep** (discipline §5): both spellings
   (`Surface::Torus` / `SurfaceKind::Torus`) over `crates/*/src` —
   BOOL-2's recorded blind spot, applied from the start.

## Acceptance

- The consumer row red on main, green under the arm; the root-count
  posture demonstrably escalates on an uncertain count (a row, not
  prose); both pins flipped and green; sphere+cone suites green on
  the extracted scan (byte-stable behavior).
- Hosted CI green on the final head; gate record per head in the PR.

## Hard rules

- NO `Co-Authored-By` trailer, no model names in commits. "issue
  1011" spelled out, no closing keywords.
- Scope fence: `crates/topo/src/boolean/solid_contain.rs`, the
  extracted scan's home (in `boolean/`), topo/sweep containment
  suites, the two pins + donut pin in `verbs_gate_r1_probes.rs`
  (the flips only). NOT: `splitting/`, `census.rs`, `geom-brep`
  (VERBS is live there), no `docs/MODEL-AB-LOG.md` /
  `docs/S-BOOL-*.md` / SMELL edits.
- Re-merge main before opening the PR; the file is hot.
