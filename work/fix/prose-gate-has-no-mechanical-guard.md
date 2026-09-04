---
id: prose-gate-has-no-mechanical-guard
kind: issue
title: the prose gate is enforced only where someone remembered: no row renders every Display-reachable refusal at its struct-shaped payload variants
status: review
branch: fix/prose-gate-guard
opened: 2026-09-04
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
`{binding:?}` to the field type the binding is declared at, and asks
whether that type's `Debug` can carry the field-brace fingerprint. A
site is judged for the TYPE it renders, so which variant a sampler
would have constructed cannot change the answer — the `blend/mod.rs`
`seeds()` hole is structurally absent rather than avoided.

Two rows over the tree, plus one over this crate's own raise sites:

- `no_display_impl_renders_a_brace_shaped_payload_through_debug` —
  deny by default. `KNOWN_BRACED` holds the twelve sites that are
  brace-shaped today, each with its reason; every one is a known-live
  or undischarged defect on another program's ground, none is accepted
  behaviour, and an entry that stops naming a brace-shaped site fails
  the row exactly as a new site does.
- `every_site_the_resolver_cannot_type_is_named_in_the_census` —
  `UNDECIDED` names all 27 sites the resolver cannot type, and the row
  fails in both directions. The blind spot is a stated population that
  cannot grow in silence.
- `no_raise_site_composes_a_debug_rendering_into_its_message` — the
  other route into the message, over this crate's `typed_err` calls.
  One allowed site: `py/flush.rs`'s unknown-`ContactClass` arm, which
  `errors.rs:376` already warns about in prose.

Evidence, on the real tree and not only on fixtures: planting a struct
variant on `SeamSide` (`crates/geom/src/curves/compose.rs`, whose
`ComposeError` renders `{side:?}`) turns the first row red naming that
site — with no value of the planted variant ever constructed — and
reverting turns it green. Planting `format!("{err:?}")` at a
`typed_err` call in `py/doc.rs` reds the third row the same way. Four
further rows run the census over planted one-file trees on every
invocation, including the sampler-blind-spot shape and the nested-match
shape that hid the two live instances; a fifth renders a real
struct-variant value and asserts `reads_as_prose` rejects it, so the
static verdict and the runtime gate are pinned to each other.

### What the guard cannot see

- **Reachability.** It over-approximates to every `Display` in the
  tree rather than tracing raise paths, deliberately: a reachability
  analysis is the half that goes stale silently when a door is added.
  So a flagged site may be cosmetic rather than a live panic, and the
  reason string is where that is recorded.
- **The `Real` scalar.** A `{x:?}` on a generic scalar field is
  undecided because the answer depends on the lane: `geom-core`'s
  `Interval` wraps `interval_transcendentals::DInterval`, a
  named-field struct with a derived `Debug`, so those renderings carry
  the fingerprint in an interval build and not in a default one. Those
  sites are in `UNDECIDED`; the class is reported to the orchestrator.
- **`macro_rules!` bodies and `include!`d text**, which the shared
  lexer does not expand.
- **Cross-crate name collisions**, which resolve to braced if either
  definition is and to undecided otherwise — never to prose.

The two live point fixes stay FILLET's and EXCH's
(`work/issues/debug-in-prose-at-blend-and-step-import.md`); the
`editor-core` family stays DOCM's
(`work/docm/debug-in-prose-residue-after-finding-sink.md`). Two sites
this census found that were not previously filed —
`geom-brep/src/offset_fit.rs` (`SplineError`) and
`topo/src/boolean/voids.rs` (`RevertError`) — are reported to the
orchestrator per `docs/prompts/implementer-discipline.md` §6 rather
than filed on another program's slate.
