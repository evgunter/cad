# MESH-11 — issue 1571: the walk's arc premise, verified rather than inherited

**Binding at dispatch** (S-MESH program, `docs/S-MESH-PLAN.md`;
difficulty logged pre-draw: **M**). Read
`docs/prompts/implementer-discipline.md` in full before starting.
Issue 1571 is the primary specification (filed from MESH-7's review,
executed): props certifies each edge's CARRIER, not that the traversed
ARC stays on one chart meridian, so a great-circle meridian arc that
crosses a pole passes the shape door and the walk produces an `Ok`
non-watertight mesh with debug assertions off. MESH-7's retraction
(`crates/mesh/src/walk.rs`, the closure paragraph "CLOSED AS WORDED …
and only as worded"; `walk.rs`'s header sentence "inherited from
upstream rather than verified"), issue 723 (the same class on the
props side, fixed for the EXTENT by CERT-1), and MESH-8's coherence
examination (`topo::coherence`, whose `MeridianClosure` condition
REPORTS this body but gates nothing) are the ground. **Precondition:
MESH-8 and MESH-10 have merged** (the chart closed forms and the iso
classification live in `topo::chart` / `topo::chart_iso`; the torus
extent fold is in).

## Situation

Every consumer of a chart assumes each boundary edge is an iso curve
of the chart traversed on ONE branch: `topo::chart_iso::mid_azimuth`
reads the meridian carrier's midpoint through `Chart::u_of`, and a
pole-crossing arc puts that midpoint on the far branch (u jumps by π
mid-edge). The sphere's rim/meridian classification certifies the
carrier is a great circle; nothing certifies the arc's parameter span
is monotone and pole-free on the chart. Reach today: the Euler doors
(issue 1571's body is tier-3 refused at rest for construction
artefacts, not for the arc); a STEP file stating such an arc is the
unmeasured door (issue 723's half-cap was imported tier-3 green).

The two π-rad witnesses are both in tree: `mesh/tests/mesh7r1_probes.rs`'s
`pole_crossing_half_cap` (Euler doors) and
`crates/step-import/tests/fixtures/...` (issue 723's half-cap through
import; see `poleguard.rs`). `walk.rs:~803`'s ledger names both.

## FIRST, before the build — the door/walk split, reported

Decide and report WHERE the arc premise is verified, before building:

- **at the walk** (`mesh`): the walk checks, per meridian traversal,
  that the arc's chart image is monotone and pole-free — its span
  decided at the band with a lever (the arc's own radius: an angular
  span times R is a length), refusing typed (a new `TessellateError`
  arm, D2 addendum row 2 — valid input, lane not built) before any
  mesh is minted. Cheap, local, but a THIRD consumer re-deriving a
  chart fact and a refusal only `mesh` gives (props'
  `mass_properties` still returns volume 0.0 on the L-shaped
  complement — issue 1571's props-side finding).
- **at the door** (`props`): the per-kind boundary parse certifies the
  ARC span, not only the carrier — a meridian arc whose stored span
  crosses a pole refuses `NotIsoRectangle { what }` with a new named
  decide — which also serves `mass_properties` and
  `boundary_material_sign` (the L-face closes), and MESH-7's shape
  door refuses the body before the walk ever runs. One home, one
  predicate, every consumer flips together; but it changes the flux
  lane's admission set (CERT ground — the sphere's `sphere_meridian_
  span_levels` already folds the pole into the EXTENT, so the door
  today knowingly admits the crossing arc for area purposes).

Report the reading with the sites and the D2 row for each, and a
recommendation. The door is the expected answer under the S-MESH Q3
ruling (explicit doors; props' predicate is the one home) — but if the
flux lane's area closed form is RIGHT on the crossing arc (CERT-1 made
it so) and only the chart image is wrong, the honest shape may be a
door that certifies "one chart branch" as a SEPARATE named predicate
that `mesh` cites and `mass_properties` need not — say which, and
STOP for Evan if the two consumers genuinely need different answers
(that is a design fork).

## Deliverables

1. **The predicate**, at the home the report chose: "each boundary
   edge's traversed arc lies on one chart branch" — monotone,
   pole-free parameter span, decided at the band through the funnel
   with the arc's radius as the lever, D2 row stated; refusing typed
   with the offending edge and the measured overshoot; red-first on
   both witnesses; the quiet side pinned (an arc ending exactly at a
   pole is admitted — the walk's own inclusive pole rule, MESH-8's
   `a_pole_endpoint_is_not_measured_against_its_own_carrier`).
2. **The walk's header sentence retires** — "inherited from upstream
   rather than verified" becomes "verified at <home>", with the
   closure paragraph MESH-7 retracted re-recorded as CLOSED, this
   time truthfully; `walk.rs:~803`'s ledger updated (the two π-rad
   witnesses now refuse before the walk).
3. **Both witnesses, both directions**: the Euler-door body and the
   imported half-cap refuse typed at the new home; with the predicate
   removed, the walk's behaviour is as recorded (MESH-8's report and
   the issue-897 census). `poleguard.rs` follows (it asserts the
   current typed outcome + the coherence report — re-aim it to the
   new refusal, disclosed).
4. **The props-side finding** (volume 0.0 on the L-shaped complement
   of a closed sphere): closed by the door choice, or filed forward
   with the measurement if the walk choice is taken — say which.
5. **D9 / behaviour**: no mesh byte moves on any body that meshes
   today (two-build digest, three ε rows); the ONLY behaviour change
   is the new refusal on bodies that were non-watertight `Ok` with
   debug assertions off. The issue-653 sweep counts stay (254, 4)
   unless the fixture set contains a crossing arc — then say so.
6. **ε posture** (issue 1356): the new key's band story per band; the
   lever stated; three-ε battery; `CI-Config: lane=… eps=…` argued
   (the predicate is `Decide`-generic if it lands in props — ask for
   the interval lane; if in mesh, f64 only — ask for eps=1e-12).
7. **Class sweep** (discipline §5): every other "carrier certified,
   arc not" site — the cylinder/cone generator arcs (linear v, no
   pole: state why immune), the torus meridian (periodic v: can a
   minor-circle arc wrap past the extent? — measure), the coherence
   examination's `MeridianClosure` (now a redundant report on the
   refused bodies — say so at the condition). Dispositions, no action
   outside the fence.

## Acceptance

- The split reported BEFORE the build; the predicate red-first on both
  witnesses and quiet on the corpus; the walk's header truthful; D9
  identical elsewhere; hosted CI green; gate record per head; issue
  1571 CLOSES at merge (keyword hygiene: the orchestrator closes).

## Hard rules

- NO `Co-Authored-By`, no model names. "issue 1571" spelled out; no
  closing keywords.
- Scope fence: per the split — `crates/mesh` (walk.rs, types.rs, the
  suites) and/or `crates/geom-brep/src/props/curved.rs`'s per-kind
  parse + `require_iso_rectangle` (Track R ground on this program's
  leave; NOT `props/quad.rs`, `patch_bound.rs`, the area lanes —
  S-CERT's; NOT the closed-form area/volume derivations themselves);
  `crates/topo/src/chart_iso.rs` only if the predicate's arithmetic
  belongs beside `mid_azimuth` (disclose); `step-import/tests/
  poleguard.rs` (re-aim, disclosed); `topo::coherence`'s doc only.
  NOT: `topo::validate`'s tiers, `import_step`, the boolean, MESH-10's
  torus extent fold (landed), the walk's classification decisions
  beyond the new check.
- Re-merge main before opening the PR.
