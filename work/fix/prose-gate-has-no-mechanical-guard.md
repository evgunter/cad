---
id: prose-gate-has-no-mechanical-guard
kind: issue
title: the prose gate is enforced only where someone remembered: no row renders every Display-reachable refusal at its struct-shaped payload variants
status: review
branch: fix/prose-gate-guard
opened: 2026-09-04
pr: 1809
---


Cut by the FIX orchestrator from PR 1779's style review, which found
**two live instances** of a panic that unit had just closed once. The
point of this item is that a fourth, fifth and sixth instance are
cheaper to prevent than to find.

## The gate, and how it is enforced today

`crates/pncad-py/src/errors.rs`'s `reads_as_prose` rejects a message
containing the field-brace fingerprint `" { "`, and `py::typed_err`
asserts it on **every** raise — live under release, since the root
manifest keeps `debug_assert` on. So a kernel refusal whose `Display`
renders a struct-shaped payload through `Debug` does not degrade: **it
panics the binding**, where the arm meant to refuse gracefully.

Enforcement is a `debug_assert` at the raise. That catches an instance
**when someone runs the door that raises it**. Nothing enumerates the
doors.

## Three instances, one fix

- `ValidationError::UndeclaredContact` / `StaleContactDeclaration` —
  panicked on the first honest call of `validate_pseudomanifold`;
  closed by PR 1779.
- `BlendError::Escalated { site: BlendSite }` — `Link` and `Joint`
  carry braces; a fillet or chamfer escalation panics. **Live.**
- `StepImportError::Placement` / `Instance` — `{source:?}` on types
  that already have a `Display`. **Live.**

`errors.rs:376` already carries the general warning in prose: *"A
future STRUCT variant of that kernel enum would trip this assertion and
panic where that arm means to refuse gracefully."* The warning is
correct, it is written down, and it did not stop three instances.

## What this unit builds

A row that, for every refusal type reachable through `typed_err`,
constructs each **struct-shaped payload variant** and asserts
`reads_as_prose` on the rendered message.

The hard part is the enumeration, and it is the reason this is a unit
rather than a chore:

1. **A hand-written roster re-creates the defect.** `blend/mod.rs:1195`
   is the proof — a `seeds()` list that looks exhaustive over
   `BlendError` and samples `Escalated` with `BlendSite::Chain`, the one
   brace-free variant. A roster that picks its own samples excludes the
   failing mode by construction. Whatever this unit builds must not be
   another such list, or it will pass for the same reason.
2. **The resolver sweep is the method that works**, and PR 1779's lane
   ran one: every `impl fmt::Display` in `crates/`, resolving each
   `{ident:?}` to its declared field type and asking whether that type
   is brace-shaped. 370 sites, 32 brace-shaped. Its stated blind spots
   are the scope question here — chiefly the **51 positional `{:?}`
   sites it could not type at all**.
3. **Reachability, not just shape, decides severity.** A brace-shaped
   payload that never reaches `typed_err` is cosmetic; one that does is
   a panic. The two live instances were found by tracing the raise path,
   not by matching the rendering.

Whether the guard is a test, a lint, or a `#[test]` over a derived
roster is the unit's to decide. What it may not be is a list somebody
maintains by hand.

## Scope note

The two live point fixes are **not** this unit's: `blend` is FILLET's
ground and `step-import` is EXCH's, filed together at
`work/issues/debug-in-prose-at-blend-and-step-import.md` and routed. A
live panic on a public door should not wait on a test. This unit is what
stops the fourth one.

## Closed

`crates/pncad-py/src/prose_census.rs` — a source census, not a roster
of samples. It reads every `impl Display` in the tree, resolves each
`{binding:?}` in each format string to the field type the binding is
declared at, and asks whether that type's `Debug` can carry the
field-brace fingerprint. A site is judged for the TYPE it renders, so
which variant a sampler would have constructed cannot change the
answer — the `blend/mod.rs` `seeds()` hole is structurally absent
rather than avoided.

Three rows over the tree:

- `no_display_impl_renders_a_brace_shaped_payload_through_debug` —
  deny by default. `KNOWN_BRACED` holds the ten brace-shaped sites,
  each with its reason and **its count**; every one is a known-live or
  undischarged defect on another program's ground, none is accepted
  behaviour. The roster is compared for equality, so an entry that
  stops naming a site reds, and so does a partial repair that moves a
  count.
- `every_site_this_census_cannot_decide_is_named_with_its_reason` —
  `UNDECIDED` names all 28 undecided sites **with the reason each could
  not be decided**, compared for equality in both directions.
- `no_raise_site_composes_a_debug_rendering_into_its_message` — the
  other route into the message, over this crate's `typed_err` calls,
  following the message through a local `let`. The allowance is keyed
  on the SITE (file plus message expression), not the file.

### The style review round

The design survived review; the hand-written item and type grammar did
not. Three executed breaks of the "cannot silently miss a variant"
claim, each now a permanent row:

- an **or-pattern** was resolved at its last alternative only, so a
  brace-shaped payload written anywhere but last was cleared as prose —
  a silent miss of a variant, which is this item's own constraint;
- the item-head scanner counted angle brackets with no bracket
  tracking and no arrow exception, so `Fn(u32) -> bool` in a bound and
  a `where` clause both made it answer **prose** over a named-field
  payload. It now uses `test_utils::source::angle_end`, whose own
  documentation names both cases — a third hand-rolled angle scanner
  in this tree was the defect, and importing the shared one is the fix;
- **every string literal** in a `Display` body was read as a format
  string, inventing rendering sites out of ordinary prose. The scan is
  now keyed on formatting-macro calls.

Four more, all executed and all fixed: a `Display` matching on a
`kind` field typed none of its bindings (every error type with such a
field shared the hole); generic arguments were never substituted;
rival declarations of one name produced a verdict from the collision
rather than from the type — which had put **two phantom defects** in
`KNOWN_BRACED` whose stated cause was false, both now correctly
undecided; and `pub(crate)` visibility was not stripped, pushing real
sites into the blind spot.

The rule the module is now written against: **Undecided is never a
pass, and no arm that cannot answer may guess prose.** An unparseable
item and a disagreement between rival declarations both answer
undecided rather than "no fields, so prose".

### Evidence

On the real tree, not only on fixtures: planting a struct variant on
`SeamSide` (`crates/geom/src/curves/compose.rs`, whose `ComposeError`
renders `{side:?}`) reds the first row naming that site — with no value
of the planted variant ever constructed — and reverting greens it.
Adding a seventh `{slot:?}` to `EditError` reds it on the count alone.
A `Debug` rendering behind a local `let` at a `typed_err` call, and a
second `Debug` raise appended to the one allowed file, each red the
third row. Twelve further rows run planted one-file trees on every
invocation, including the or-pattern, arrow-in-bound, `where Fn()`,
non-format-literal, `kind`-field, generic-substitution and
rival-name shapes; one renders a real struct-variant value and asserts
`reads_as_prose` rejects it, pinning the static verdict to the runtime
gate.

### What the guard cannot see

- **Reachability.** It over-approximates to every `Display` in the tree
  rather than tracing raise paths, deliberately: a reachability
  analysis is the half that goes stale silently when a door is added.
  A flagged site may be cosmetic rather than a live panic, and the
  reason string records that where it is unknown.
- **The 28 undecided sites**, whose reduction is filed as
  `work/fix/prose-census-undecided-residue.md` — positional `{:?}`
  over expressions it does not type, the `Real` scalar (lane-dependent:
  `Interval` wraps a named-field struct), and names with rival
  declarations.
- **A message bound by anything other than a local `let`** at a raise
  site; **`macro_rules!` bodies and `include!`d text**; **type aliases
  and re-exports**, which resolve to no declaration and answer
  undecided.

The two live point fixes stay FILLET's and EXCH's
(`work/issues/debug-in-prose-at-blend-and-step-import.md`); the
`editor-core` family stays DOCM's
(`work/docm/debug-in-prose-residue-after-finding-sink.md`). Two sites
this census found that were not previously filed —
`geom-brep/src/offset_fit.rs` (`SplineError`) and
`topo/src/boolean/voids.rs` (`RevertError`) — were reported to the
orchestrator, who is filing them.
