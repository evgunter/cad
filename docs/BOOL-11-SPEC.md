# BOOL-11 — the declared point-target continuation and the structural closer

**Binding at dispatch** (S-BOOL program, `docs/S-BOOL-PLAN.md`;
difficulty logged pre-draw: **M**). Read
`docs/prompts/implementer-discipline.md` in full before starting.
The Q1 third-round ruling (Evan, in-chat, 2026-09-01; ratified PR
#1512 — `docs/S-BOOL-PLAN.md` §Rulings) is the primary
specification, together with PATHS §4's ruled seam paragraph
(`docs/PATHS-DESIGN.md`, "The seam, measured here and RULED") and
BOOL-8's PR #1508 record. Issue 433 context applies but the issue
does not close here (BOOL-9 remains).

## Situation

BOOL-8 landed the interior straight continuation and measured the
seam wall: a straight run crossing the seam is unauthorable in
either rotation, forced by the corner/subdivision alternation
(bilaterally review-confirmed — a 64-spelling exhaustive search
found zero closures). The ruling: the continuation gains a
**declared point-target form** — a leg declared to be the straight
continuation landing on a NAMED point, any authored point, with the
kernel CHECKING the target lies on the departing directed point's
ray and refusing when it does not — and the **structural closer**
(`Start` as target) is the special case that ends the seam wall.
Declared and checked, never inferred from a value coincidence.

## The check — RULED (Evan, in-chat, 2026-09-01, fourth round)

"Target lies on the ray" means, as ever, TO WITHIN ε.
Exact-or-refuse is out. The declaration is what legalizes the
band: with the intent authored, the ε comparison is authored-data
CONSISTENCY (the arc verbs' consistency-refusal class), not the
value-inference the ladder refuses — that reads intent off a
coincidence nobody declared. A target past the band refuses TYPED
as inconsistent authored data, message naming the declared intent
and the measured miss.

What remains yours to design, stated at the site and in the §4
text for Evan's eyes: WHICH ε (the input-quality band ε_input is
the natural home — argue it against the run band), the LEVER the
miss is metered by (the leg is a length; the miss is a lateral
distance — state the metering so the comparison is
dimension-honest per the D4 discipline), and the D2 row (row 1/3
input-quality is the natural classification). Note BOOL-8's
measurement as your first fixture: lily's corner sits 7.85e-17
off the closing ray — deep inside any sane band — so the closer
must accept it, and your band boundary needs a row on BOTH sides.

## Deliverables

1. **The verb form** (naming yours; one `transition_table!` row per
   the LIB-RTABLE invariant; emit through the shared
   `emit_straight_leg` kernel BOOL-8 factored): declared
   point-target straight continuation from a directed point, the
   on-ray check per your design decision, a NEW typed refusal for
   the off-ray case (message naming the declared intent and the
   measured miss).
2. **The closer**: `Start` as target closes the loop through the
   same check; the seam junction it produces is a corner junction
   (PQ4 untouched). BOOL-8's two seam-wall rows FLIP from refusal
   pins to the demonstration (their current docs anticipate this).
3. **Wire/schema**: a new step variant reaches the wire enum,
   which is matched exhaustively — if this forces a schema bump,
   COORDINATE ON THE AWAY CHANNEL before landing (schema is
   contended ground; BOOL-10's entry records the same protocol).
   If the addition is representable without a bump, say how.
4. **Lily migrates fully**: the section authors through the
   lattice (interior continuations + the closer), `RawLoop` and
   the second kernel dependency leave `demos/tour`, the named-gap
   comment RETIRES (both halves closed), render byte-stable or
   the delta measured. This is the ruling's demonstration.
5. **§4 re-records as landed**: the ruled seam paragraph updates
   (the wall paragraph becomes history, the verb-table row lands,
   the f64 decision's argument is written in); the
   `TangentLineClose` recourse names the landed spelling.
6. **Refusal-site separability** (BOOL-8 fix-pass substrate, the
   adjudicated deferral): consider a site tag in the relevant
   refusal payloads so the two seam mechanisms pin apart — this
   unit owns those rows now; do it or record why the margin-only
   payload stays.
7. **Rows**: the continuation-to-target accepted (red on main);
   off-ray target refused typed (both sides of your check's
   boundary pinned); the closer closing lily's section; the
   keep-refusing set from BOOL-8 stays green; declared-vs-
   undeclared contrast rows.
8. **ε posture** (issue-1356): your check's band story (exact or
   banded) stated per-band; three-ε + interval battery; trailer
   decision argued (BOOL-8's interval-asked precedent with the
   Dual64-corrected reason).
9. **Class sweep** (discipline §5): every consumer of the seam /
   closer path in `crates/profile`; the #433-stance prose sites
   updated to "both lattice halves landed, raw door remains
   (BOOL-9)".

## Acceptance

- Lily authors through the public surface with `RawLoop` gone from
  the demo; red-first on the new verb both directions; §4 carries
  the decided text; hosted CI green; gate record per head.

## Hard rules

- NO `Co-Authored-By`, no model names. "issue 433" spelled out, no
  closing keywords.
- **The PR does not merge on green** — PATHS §4/verb table is
  design surface; the PR carries your f64 decision's text for
  Evan's eyes; the orchestrator holds the merge for the sign-off.
- Scope fence: `crates/profile/src/path.rs` + `path/program.rs`,
  profile suites, `demos/tour/src/lily.rs` + `Cargo.toml`,
  `docs/PATHS-DESIGN.md`; wire/persist ONLY per deliverable 3's
  coordination. NOT: `RawLoop`'s demotion (BOOL-9),
  `arc_continue` (BOOL-10), `validate` semantics, `lift.rs`
  (report the `repair_same_carrier` observation forward if your
  work touches its ground — do not act on it). `crates/profile`
  is SMELL track V fence ground — stop and report if reached.
- Re-merge main before opening the PR.
