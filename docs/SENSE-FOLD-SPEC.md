# SENSE-FOLD — the hand multiplies of a normal by `sense_sign` fold onto `OutwardNormal`, and `Face::sense_sign` retires

**Status: ratified at dispatch (SCALAR orchestrator, 2026-09-15).** Binds
the implementer of unit `sense-sign-multiplies-fold-onto-outward-normal`;
deleted at merge per `docs/DOC-LEDGER.md`. Read
`docs/prompts/implementer-discipline.md` in full first. The item is
`work/scalar/sense-sign-multiplies-fold-onto-outward-normal.md`; the
ruling is `work/scalar/D6.md` §RATIFIED (PR 2457); the first unit landed
as PR 2649 (`crates/topo/src/face_normal.rs`'s module doc and
`crates/geom-brep/src/enters.rs` `OutwardNormal::from_chart` are the
principle's homes).

## 0. The ruling this executes

The sense is a bit (`Face::sense`), and the typed thing is what the bit
selects: `OutwardNormal<T>` where a normal is meant (`from_chart(chart_normal,
sense: bool)` is its only constructor), a conditional negation where a
scalar is genuinely signed. Never a ±1 type, and no `T` ±1 crossing a
function or struct boundary. `Face::sense_sign<T>()` retires here.

## 1. What this unit delivers

**The census, at `origin/main` `d71bb6a78` (the orchestrator's survey;
re-take it at your merge base and put the table in the PR body):**
twelve production call sites of `Face::sense_sign` in nine files, two
more `±1`-from-a-bit mints that no `sense_sign` grep sees, and six
test-side sites.

| # | site | disposition |
|---|---|---|
| 1 | `editor-core/src/names/emit_topo.rs:60` `face_plane` | `OutwardNormal::from_chart(*normal, face.sense)`; the consumer takes the type or `.vec()` at the door — say which |
| 2 | `sweep/src/blend/build.rs:344` `outward_of` | same; consider `Option<OutwardNormal<T>>` if every caller reads a normal |
| 3 | `sweep/src/blend/battery.rs:441` `outward` | `from_chart(g.normalize(), f.sense)` — the normalize is the site's own, unchanged (no decision added) |
| 4 | `topo/src/boolean/join.rs:1382` `ring_run_ccw` | `face_outward_normal(body, face)` (planar door, same crate) — the `desync` arm on a non-planar face must still fire |
| 5 | `topo/src/boolean/rest.rs:558/569` `face_carrier` plane arm | the planar door; the curved arms already carry the bit |
| 6–7 | `topo/src/boolean/solid_contain.rs:400`, `:616` | the planar door, keeping the typed `KindUnsupported`/`CorruptFace` refusals where the door returns `None` |
| 8 | `topo/src/merge_faces.rs:1697/1699` `planes_declared_equal` | the tuple already carries `f.sense`; drop the `±1` and let the downstream read the bit or an `OutwardNormal` |
| 9 | `topo/src/merge_faces.rs:2478` `merged_outline_ring` | takes a resolved `&Face`, not a key — add the by-value sibling of `face_outward_normal` IN `face_normal.rs` (the one home; the raw-text guard below must stay green) |
| 10 | `topo/src/validate.rs:4307` check 6 | the planar door; the checker reads the bit's MEANING through the door and falsifies the loop winding against it — say so in one sentence at the site |
| 11 | `mesh/src/walk.rs:794-797, :964` `loop_polygon` | NOT a normal: `chart_area = if sense { area } else { -area }` — the conditional-negation arm |
| 12–13 | `topo/src/boolean/contact_verify.rs:303-304` `sa`/`sb` | two face bits minted to `±1` with no `sense_sign` call — fold the same way as the sites they feed (door or conditional negation; read the consumer) |
| 14 | `topo/src/r2_probes.rs:26-32` `sgn` | probe-only in-src helper spelling the bit by hand to dodge the census — retire with the census |
| 15 | `sweep/src/blend/arms.rs:750, :882` `Meridian::trace`/`Ruling::trace` `side` | **decided below** |
| tests | `sweep/tests/common/orient.rs:97, :222`, `blend3_concave_chamfer.rs:111`, `blend3_r2_probes.rs:105`, `blend4_concave_fillet.rs:268`, `topo/tests/readback_sense_kind.rs:89` | the door (`from_chart(..).vec()`) or the conditional negation, site by site; TCOST/TINT ground, announced |

**The `arms.rs` decision.** Read `SupportTrace::contact` (`:572-581`)
and `sheet_center` (`:622-695`): `side` appears ONLY as a factor on the
blend radius (`radius * side`, `rr - radius * side`, `r_a - radius * s_a`,
`2Rr(σ_circle − σ_line(n̂·û))`). It is the `R ∓ r` selector — a bit
wearing a `T` — and the radius it multiplies is not known at `trace`
time, so the fold is: `SupportTrace::{Straight, Round}.side` becomes
`bool` (doc: "the material side — `true` when the ball sits on the
chart-normal side", or whatever `ball_side` means; say it once), and
every consumer spells `radius * side` as a conditional negation of the
radius (`let rho = if side { radius } else { -radius }`), including the
polynomial forms (`s_a - s_b * d` becomes `(if s_a { one } else { -one }) - (if s_b { d } else { -d })`
or the equivalent on the offsets `off_a = r_a - rho_a` the circle×circle
arm already uses). `±1` multiplies are exact in IEEE, so this is
bit-identical (NaN sign and signed zero excepted, as PR 2649 recorded)
— and the D9 differential over `sweep`'s suites plus the tour digest is
the proof. The `nappe` sign at `:782` (`copysign` of a DECIDED dot) is
a genuinely signed `T` and stays. `Convexity::signed`/`ball_side`
(`battery.rs:186-222`) are the bit's provenance and stay.

**Retirement.** `Face::sense_sign<T>()` (`topo/src/entity.rs:302-304`)
is deleted; with it `face_normal.rs`'s hand-kept census
`every_hand_multiply_of_the_face_sign_is_inventoried` (`:356-470`) — its
PINNED table empties and the type system is the guard from here (a new
`sense_sign` call does not compile). The OTHER guard,
`the_planar_sense_flip_lives_in_one_place` (`:305-354`, raw text: no
`topo/src` file but `face_normal.rs` may contain both `Surface::Plane {`
and `from_chart`), STAYS GREEN by construction: topo-internal planar
sites route through `face_outward_normal` and its by-value sibling, and
the `from_chart` text stays in `face_normal.rs` only; cross-crate sites
(1–3, 11) are outside its walk. If you find you must name `from_chart`
in another `topo/src` file, stop and say why in the PR body rather than
widening the guard. `entity.rs`'s doc for `sense` (`:250-300`) says what
the bit means and points at the two homes; the "`sense_sign() *
chart_normal`" sentence goes.

**Prose sweep.** `sense_sign` is named in ~40 doc comments across
`editor-core`, `geom-brep`, `mesh`, `step-export`, `sweep`, `topo`, and
in `docs/DESIGN.md:162` and `docs/PROPS-SPHERE-POLE-SIDE-SPEC.md`
(survey §1 lists them). Every one is re-worded to the bit and the door
or dispositioned in the PR body. `docs/DESIGN.md` is the ratified
design contract: a sentence re-worded because an approved change
retired the symbol it names is not a second decision (CLAUDE.md) — run
`git log -S'sense_sign' -- docs/DESIGN.md` first, cite the commit, and
change only the naming, not what the clause decides.

**What must not change:** every verdict, margin, normal and area, bit
for bit — a `from_chart` conditional negation is the same exact
operation as the `±1` multiply (NaN sign / signed zero excepted, said
once in the PR body); the k-lint gate's predicate counts do not move;
the tour digest (the zero-parameter recipe in
`work/scalar/rate-pair-in-geom-core.md` §Correction, listing and
narration) is identical before and after.

## 2. Docs

One sentence at each folded site is enough where the door's doc carries
the reason; no restatement of `from_chart`'s argument. No history.

## 3. The pin

- D9 differential: `topo`, `sweep`, `mesh`, `editor-core` suites green
  unchanged; the tour digests identical at merge base and head (take the
  base digest at your merge base, commit it before the change as
  RATE-PAIR did).
- A row per folded arm where the two senses give different answers
  (anti-vacuity) — reuse the fixtures PR 2649's rows built where they
  reach these sites (`the_two_senses_route_a_tangent_rim_to_different_arms`,
  the reversed-band ball) rather than new ones.
- The `arms.rs` fold: a row on one concave and one convex blend where
  the ball side differs, bits pinned against the merge base's values.
- `the_planar_sense_flip_lives_in_one_place` green unchanged.

## 4. Sweep

The class: a face sense read as a scalar `T` (a `±1` minted from
`Face::sense` or from a `sense: bool` parameter, stored in a field, a
tuple or a local, or multiplied inline). Patterns that found the census:
`sense_sign`, `if .*sense.* \{ T::one\(\) \} else \{ -T::one\(\) \}`,
`\{ 1\.0 \} else \{ -1\.0 \}` near `sense`. Re-take at your merge base
over `crates/*/src`, `crates/*/tests`, `demos/`, `tools/`, `benches/`;
disposition every hit in the PR body; state the blind spot (a bit
renamed at a boundary — `same_orient`, `reversed`, `inside` — is NOT a
sense unless its provenance is `Face::sense`; check provenance for each
`if x { one } else { -one }` you find and say which way it went).

## 5. Fence

This program claims no paths. This unit reaches TOPO's
`crates/topo/src/{merge_faces.rs,validate.rs}`, BOOL's and CURVED's
`crates/topo/src/boolean/{join.rs,rest.rs,solid_contain.rs,contact_verify.rs}`,
BLEND's `crates/sweep/src/blend/{arms.rs,battery.rs,build.rs}`, WIRE's
`crates/editor-core/src/names/emit_topo.rs`, S-MESH's
`crates/mesh/src/walk.rs`, TCOST/TINT's `crates/sweep/tests/*` and
`crates/topo/tests/readback_sense_kind.rs`, and the unowned
`crates/topo/src/{entity.rs,face_normal.rs,r2_probes.rs}` and
`crates/geom-brep/src/enters.rs`. Announced by the orchestrator; merge
`origin/main` before opening the PR; territory output in the PR body.

## 6. Verification and report

Local: `cargo nextest run -p topo -p sweep -p mesh -p editor-core` at
default features and `-p topo --features interval` if it exists; `cargo
clippy --workspace --all-targets -- -D warnings` at default features AND
`--all-features` (a `pub` method retires — every caller must compile,
including feature-gated probes) plus `demos/tour` and `demos/wild`
clippy; `scripts/doc-gate.sh` AND `scripts/doc-gate.sh --skip-viewer-toolkit`
(the arm CI runs). Hosted CI is the verification of record; poll to
conclusion in the foreground. Report ≤120 lines: the census table
with dispositions, the `arms.rs` decision and its row, the retirement
(what the census test became), the differential's and digest's
receipts, deviations, rows filed and where, PR number, head SHA, CI run
id and conclusion.
