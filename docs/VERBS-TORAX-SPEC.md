# VERBS-TORAX — the offset-axial torus arm

**Ratified 2026-09-01** by the VERBS orchestrator, created by the
adjudication of C5ARMS PR-1's fired STOP (option (c), VERBS-LOG
2026-09-01): the plane×torus section arm is held behind this unit
because every named consumer of that arm dies at the per-chart
corner-accumulation refusal (`ReanchorOffCarrier`), whose cure is
the axial door learning the torus kind. Pre-logged difficulty
**M**. One PR, branch `verbs/torax-1`. This unit **unblocks
C5ARMS PR-1 rows 3/4/8/12/13** (see that spec's HOLD note).

All line cites are main as of `1ebf2a9d9` and were read, not
recalled. `offset_axial.rs` = `crates/topo/src/offset_axial.rs`.

## What the unit is

`topo::shell`/`shell_open` on a torus-walled body falls to the
sequential per-chart loop because the simultaneous axial door
refuses the kind twice: `axial_frame`'s seed match knows
Cylinder/Cone/Sphere and refuses `other` (offset_axial.rs:571-583,
refusal at :577), and `classify`'s `(other, _)` arm refuses again
(:740-745). The per-chart loop transports each corner rigidly once
per chart and the difference is real geometry error — the C5ARMS
STOP measurement decoded all three consumers' gaps to ~13 digits
as one chart's rigid transport checked against a neighbour's
unmoved carrier (0.85–6.1 mm, ~9 orders above ε). This unit
widens the AXIAL door to `Surface::Torus` so those corners are
SOLVED, exactly, in the `(ρ, h)` half-plane — the SHELLFIX-2b
lineage (axial corners for plane/cylinder/cone/sphere), one kind
wider.

Two halves, and which consumers need which:

- **The meridian circle** (profile half): a coaxial torus
  constrains a corner to the circle centred `(R, h_c)` radius `r`
  in the `(ρ, h)` half-plane. Needed by ALL THREE consumers
  (torus_barrel, teapot wall 1, klein elbow).
- **The rim/carried-datum half**: the klein elbow's rim corners
  are revolve-rim seam vertices (torus wall × meridian-plane
  cap) — the shape SHELLFIX-2b solved for the sphere with the
  carried-azimuth / two-distinct-surfaces rim rule. Needed by the
  KLEIN ELBOW ONLY; barrel and teapot corners are full-revolve
  cap×wall corners (Station × circle) and never reach it.

## Opening measurement (before any code, in the PR body)

Re-run the C5ARMS probe's four operations at this unit's head
**with the C5 gate in place** (NO flag flip — the flip is
C5ARMS', not this unit's): klein elbow `shell_open` and sealed
`shell` (verbs_shell.rs:614's fixture), `torus_barrel` hollow
(demos/tour/tests/verbs_teapot.rs:196 fixture, t = 1/128), teapot
wall-1 hollow (`torus_belly`, t = 1/128). Quote each payload and
raising site. Expected doors today: the route/C5 gate
(`replace_face.rs:1615`) spelled as the pair refusal — NOT
`ReanchorOffCarrier`, which only appears once the pair gate is
scratch-lifted. Then state the unit's claim as a differential:
with a SCRATCH flip (stated in the PR body, never committed —
C5ARMS owns the flag), the same four operations at this unit's
final head land on certification/adjacency rows instead of
`ReanchorOffCarrier`. The C5ARMS STOP report's probe transcripts
(klein gap `8.472143145440653e-4`, barrel `6.0973089273993215e-3`,
teapot `4.422022405807807e-3`) are the BEFORE side of that
differential; cite them, re-measure if the fixtures moved.

## The work, numbered

1. **Frame seeding.** `axial_frame`'s seed match
   (offset_axial.rs:571-583) gains `Surface::Torus` — a torus
   carries `center` + `axis` exactly as a sphere does. Without
   this the all-torus-wall bodies (barrel) never even reach
   `classify`. The `Plane => None` skip and the `other =>`
   refusal keep their remaining meanings.

2. **`classify` gains the `(Torus, Torus)` arm**
   (offset_axial.rs:628, the `(other, _)` arm at :740 narrows).
   Coaxiality is decided the way the sphere arm decides it
   (:722-738): axis alignment as a sine levered by
   `frame.extent`, centre on the axis via `on_axis`. The
   constraint's NUMBERS read `moved`, the axis tests read
   `structural` — the module's own load-bearing law
   (:617-627), keep its comment's terms. New variant
   `Constraint::Torus { major: T /* R */, h_c: T, minor: T /* r */ }`
   (offset_axial.rs:201-212). No radius comparison anywhere in
   the arm: the spindle/horn net is upstream and stays there
   (construction `R > r > 0` per `geom/src/surfaces.rs:218-228`,
   tier-3 `DegenerateTorus`).

3. **The profile widening — AUTHORIZED, with a bit-identity
   fence.** Measured fact the adjudication's repair sketch got
   wrong: `Profile::Circle { h_c, r }` (offset_axial.rs:214-218)
   is implicitly centred ON the axis — its residual is
   `‖(ρ, h−h_c)‖ − r` (:228) — and the SAME ρ_c = 0 assumption
   is baked into the Line×Circle transversality (`d = n.1·h_c − c`,
   :1166-1173) and roots (`foot = (−n.0·d, h_c − n.1·d)`,
   :1186-1196). The torus meridian is centred at `(R, h_c)`,
   `R ≠ 0`, so `Profile::Circle` gains a `rho_c` field and the
   three sites generalize mechanically:
   residual `‖(ρ−ρ_c, h−h_c)‖ − r`;
   `d = n.0·ρ_c + n.1·h_c − c`;
   `foot = (ρ_c − n.0·d, h_c − n.1·d)`.
   The sphere mapping (:922) passes `rho_c = 0`; the new
   `Constraint::Torus` maps to `rho_c = R`. `Profile` is
   file-private, so this is an implementation widening, not a
   design one — but it touches shipped arithmetic, so: **the
   byte-dump harness (`shellfix1_bitdump.rs`, and the curved
   fixtures the axis-canonicalization note at offset_axial.rs:599
   cites) must report every existing fixture byte-identical.**
   Adding `n.0·0` and `0 − x` can flip a zero's sign in
   degenerate corners; if the general form moves any byte, keep
   the sphere path on the old expressions verbatim behind the
   same enum and say so at the site — two spellings with the
   reason is honest, one spelling with a moved byte is not.

4. **The pole arm must not accept a torus circle.** The pole
   station solve (:982-991) reads `Profile::Circle` assuming the
   circle can contain an axis pole. A torus meridian cannot reach
   the axis (`ρ ≥ R − r > 0` by the standing invariant), so the
   arm either structurally never sees `rho_c ≠ 0` (argue it from
   the caller) or refuses typed when it does. No silence.

5. **The elbow's rim corners.** The klein elbow's rim seam
   vertex carries ONE profile constraint (the torus circle) plus
   the cap's `Constraint::Meridian` — the profile solve alone
   refuses ("fewer than two profile constraints", :957-961). The
   sphere's partial-revolve rim has the SAME shape and
   SHELLFIX-2b solved it (the ledger row's "a revolve rim vertex
   is TWO distinct surfaces + the seam azimuth — carried-azimuth
   2-distinct solves"). The lane FIRST locates and cites the
   actual sphere-rim solve path at this head, then extends it to
   the torus wall by parity. The azimuth machinery itself
   (:1053-1110) is kind-agnostic and must not fork on kind.

6. **Acceptance.** The three measured consumers become the
   unit's fixtures, each corner solved EXACTLY:
   - **torus_barrel** (t = 1/128): cap×wall corners solve as
     Station × torus-circle roots; the closed form of the moved
     corner is the circle–line intersection at the offset radii
     (all dyadic inputs — state the solved `(ρ, h)` in closed
     form and assert it). The refused-form gap
     `5/64 − hypot(3/64 − 6/64, 1/128 − 1/16)` is the BEFORE
     side, quoted not re-derived.
   - **teapot wall 1** (`torus_belly`, t = 1/128): same shape,
     gap form `5/64 − hypot(3/64 − 7/64, 8/64 − 1/128 − 5/64)`.
   - **klein elbow** (R = 0.275, RLOOP = 1.2, t = 0.05, 90°):
     both `shell_open` and sealed `shell` build; the rim corners
     land on the offset torus circle AND the meridian cap
     (residuals at ε-scale, not 8.47e-4 =
     `√(1.475² + 0.05²) − 1.475`).
   Plus, all three: rigid-transport PARITY — the solved corner
   agrees with the unmoved chart's carrier by construction (the
   `offset_reanchor_on_carrier` decide at replace_face.rs goes
   `Zero`, never `Negative`). Plus a PLANTED RED: a hand-moved
   genuinely-off-carrier vertex still refuses
   `ReanchorOffCarrier` — the refusal this unit retires for
   correct geometry survives for wrong geometry. Plus the
   scratch-flip differential from the opening measurement,
   re-stated at the final head.

7. **Bit-identity block.** SHELLFIX-2b's axial rows must not
   move: `crates/sweep/tests/sf2b_axial.rs`, `sf2b_head.rs`,
   `sf2b_interval_probe.rs`, and `shellfix1_bitdump.rs` whole.
   The plane/cylinder/cone/sphere corner solves are those units'
   ratified arguments; a moved row is a change to its argument
   and needs its own adjudication. Item 3's harness receipt is
   part of this block.

## Fences

- **No section functions** — plane×torus/cyl×cone section curves
  are C5ARMS' layer (`crates/geom-brep`); this unit is
  `crates/topo` offset-orchestration only.
- **No flag flip** — `(Plane, Torus).implemented` stays false;
  C5ARMS PR-1 owns the flip and the routing acceptance.
- **No circle×circle profile machinery.** The Circle×Circle
  transversality/roots arms stay `None`/empty (:1174, :1197):
  no named consumer has a torus×sphere or torus×torus corner,
  and building unexercised root machinery is the D5-trap shape.
  Such a corner refuses typed exactly as today.
- No NURBS carrier, no marcher.
- The spindle/horn refusal net stays where it is; this unit adds
  no torus-validity logic.

## STOP conditions (pre-registered)

1. **Machinery-shape STOP**: if solving any NAMED consumer's
   corner needs profile machinery beyond item 3's three
   mechanical generalizations — circle×circle roots, a new
   constraint kind, or a solve that widens a PUBLIC type — STOP
   for adjudication: that is a design widening this spec
   deliberately does not authorize.
2. **Elbow-split STOP**: if the elbow's rim needs more than the
   SHELLFIX-2b carried-datum rule extended to the torus wall,
   STOP on the elbow and DELIVER the barrel/teapot half alone
   (items 1–4, 6a/6b, the planted red, the differential), with
   the elbow's measured blocker in the PR body — that split is a
   deliverable, and C5ARMS rows 12/13 unblock on the delivered
   half while row 3/4/8 wait with the elbow.

## Lane obligations

`docs/prompts/implementer-discipline.md` binds. No
Co-Authored-By trailer (blinding). Opening measurement before
any code, payloads and raising sites quoted — a refusal's text is
not evidence of its cause. Targeted local runs and probes only —
hosted CI verified at the STEP level is the suite evidence (read
the change filter's own output; a green job name is not
evidence; per Ev's 2026-08-31 method ruling the regular suite
is not re-run locally). Merge origin/main before opening the PR;
confirm runs actually fire (the silent faces); note inherited
main reds (#1449, #1296/#1304) and distinguish them from yours;
foreground-poll long jobs; kill detached jobs whose evidence is
superseded. Do not merge — the orchestrator merges with the
ledger row.
