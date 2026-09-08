# EVAL-6 — the placers are shape-preserving over the value: `Transform` and `Pattern` over `Instances` (spec)

**Program:** EVAL (`work/eval/plan.md`, unit 6 — the build of the ruling on
PR 2137). **Item:** `work/eval/transform-refuses-a-patterns-instances-value.md`.
**Track:** E with a **correctness arm**: the unit changes what two node kinds
evaluate to, so one style review AND one correctness review (claims below),
fix pass, record-at-merge. No A/B draw.
**Branch:** `eval/6-placers-over-instances`. **Difficulty:** M.

## The ruling (Ev, PR 2137, 2026-09-08: "1 sounds good!")

**`Node::Transform` and `Node::Pattern` are shape-preserving over their
input's value: `Body → Body`, `Instances → Instances`.** A transform
applies its one rigid map to every body of an `Instances` value and yields
`Instances`; a pattern over `Instances` yields the N·M placed bodies as
`Instances`. No node payload changes, no schema moves. `Boolean` still
takes one body (D3: a boolean of several bodies is several booleans or one
union, and the recipe does not guess), and that asymmetry is stated at
`ValuePayload::Instances`'s doc as a decision.

## What lands

1. **The operand door.** Beside `body_operand` (`crates/editor-core/src/eval/wire.rs:557`),
   a `placeable_operand` returning an enum of the two admitted shapes —
   one body (the `Body` and non-empty-`Boolean` cases exactly as
   `body_operand` admits them) or `Instances` — refusing everything else
   with the same typed `WrongOperand`, `expected: "body or instances"`.
   `body_operand` stays for the consumers that genuinely take one body.
2. **`wire_transform`** over `Instances`: the one map, `transform_rigid`
   per body, `compose_placed(.., id, i)` per body with the body's index
   as its instance ordinal (distinct sources, as the pattern already
   does), and the input's name table passed through verbatim — the
   transform contributes no `RolePath` segment (D2 pass-through), which
   holds body-by-body because `transform_rigid` is key-stable and the
   table's body indices are the instance indices. Output `Instances`.
3. **`wire_pattern`** over `Instances`: the instances are placed as a
   multi-output-body master. **The layout, fixed here as a faithful
   elaboration of the ruling:** output body index `j·M + i` for outer
   placement `j` of inner body `i` (placement-major, the D9 order the
   pattern already walks); the name of inner entity `x` in inner body
   `i` under placement `j` is `Instance(j)` wrapping the inner name,
   which already carries its own `Instance(i)`. This is the layout
   `names::emit::name_pattern` (`crates/editor-core/src/names/emit.rs:159`)
   says it is waiting for ("admitting one would need a ratified
   instance×body layout") — its `e.body != 0` refusal goes, its doc
   states the layout, and totality is checked over every output body as
   it is now.
4. **The mate walk agrees.** MSOLVE-2's member chain (`Member { instance,
   copy, at }`) already numbers transform-of-pattern and nested-pattern
   members; the implementer reads `crate::mate`'s walk and states in the
   PR body whether its (instance, copy) numbering coincides with the flat
   index above — and if it does not, STOPS: that is a design mismatch the
   orchestrator takes to MSOLVE, not a thing to paper over with a
   conversion. The pins flip: `msolve1_transform_aware.rs::
   a3_pattern_of_transform_seats_and_transform_of_pattern_resolves`
   (b) and `::a10_a_nested_pattern_head_is_a_member` stop asserting
   `WrongOperand` and assert the document GATHERS, with the expected
   pose read off the solve — the row was written to pin both halves and
   now pins the one that holds.
5. **`Node::Part { Instance(k) }`** over a nested pattern's value indexes
   the flat list; DM3's out-of-range refusal reads the flat count. Say so
   at the arm (`eval/wire.rs:2467`).
6. **The viewer's seat gate is announced to CHROME, not edited here.**
   `viewer::combine::denotes_body` decides by node KIND and tracks
   `body_operand`; under this ruling a `Transform`'s value is a body iff
   its input's is, and a `Pattern`'s never is, so the gate must read
   through the placer chain (by node kind, recursively, through the
   document) — small, mechanical, CHROME's file. The two viewer pins
   (`combine_ops::several_bodies_are_not_one_body_at_a_seat`, `::the_body_seat_tracks_the_evaluators_operand_door`)
   are theirs to re-pin. Until CHROME lands it the seat still admits a
   `Transform` by kind and a transform-of-pattern pick refuses at
   evaluation instead of at the door — the honest interim, stated in the
   PR body and in the announcement.
7. **Docs.** `ValuePayload::Instances` says which consumers admit it
   (the placers, `Part`, the product gather) and which refuse (boolean,
   split, blends — one body each) and why. `body_operand`'s "Splits and
   patterns need PR 3's naming layer" sentence goes (Q4: DM3 answered
   the selection half, this unit the whole-value half).

## Correctness claims (the correctness arm)

1. **Transform over `Instances` is N transforms**: for a pattern of N
   bodies, `Transform(pattern)` evaluates bit-identically, body for body,
   to `Transform(Part(Instance(i)))` for every `i` — a row asserts this
   over the tour's or the corpus's simplest pattern at every lane/eps
   point (D9 makes it exact, not approximate).
2. **Names pass through**: `Transform(pattern)`'s table equals the
   pattern's table (same names, same keys, same body indices).
3. **Nested pattern layout**: `Pattern_M(Pattern_N(b))` has N·M bodies;
   body `j·M + i` is bit-identical to `stepped_map_outer(j)` applied to
   the inner instance `i`; every name is `Instance(j)` over `Instance(i)`
   over a master name; totality holds (every output entity named).
4. **The mate walk's numbering coincides** with the flat index (item 4),
   asserted by a row that mates to a copy of a nested pattern and reads
   the placed pose off the evaluated document — the gather the MSOLVE
   rows could not do.
5. **Nothing else moves**: every golden, verdict log and frame over a
   document with no placer-over-`Instances` is byte-identical (the
   reviewer diffs the corpus at base and head).

## Sweep

The class is "a consumer that matches `ValuePayload` and does not say
what it does with `Instances`". `rg -n 'ValuePayload::Instances|kind_name\(\)' crates/editor-core/src`;
hit list with per-hit disposition (admits / refuses typed and says why /
not this unit). State the blind spot.

## Review

Style lane per `docs/prompts/reviewer-style-lane.md`; correctness lane
takes claims 1–5 above as MAJOR-class. Emphasis: the layout in item 3 is
the dispatcher's elaboration, not the ruling's letter — check it against
`name_pattern`'s own argument and the mate walk before building on it.

## Records at merge

`work/eval/log.md` entry with the CHROME announcement and the MSOLVE
pin flips; the item `closed` with `pr:`; this spec deleted per
`docs/DOC-LEDGER.md`.
