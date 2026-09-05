---
id: blend-payloads-outside-the-margin-family
kind: issue
title: blend payloads outside the seven-field margin family: two dropped margins and the sweep's third blind spot
status: open
opened: 2026-09-04
refs: [bare-f64-margin-payload-family, 934, 931, 925]
---

Found by the FILLET-E3 review (PR 1763). E3 fixed the seven
`BlendError` fields that were **spelled** `margin: f64`. The reviewer
found the same class one arm over, in payloads the grep for that
spelling could not see. None of it is E3's to fix — E3's brief scopes
it to the seven fields — so it is filed rather than folded in.

The class, as #934 states it: *an error payload must carry the number
it measured in a shape that says what the number IS, and must never
manufacture a sentinel that collides with a real reading.*

## 1. A decided margin dropped on the floor — `support_coaxiality`

`crates/sweep/src/blend/battery.rs:899-911`. The routing predicate
`fillet3_support_coaxiality` classifies the departure of a curved
support pair from its shared axis, and then discards what it decided:

```rust
match decide("fillet3_support_coaxiality", Margin::of(departure), band)
    .map_err(|e| esc(BlendSite::Chain, e))?
{
    Sign::Zero => Ok(()),
    _ => Err(BlendError::SpineUnsupported { edge, supports }),
}
```

`BlendError::SpineUnsupported` (`blend/mod.rs`) carries `edge` and a
`supports: &'static str` and no number at all, so a caller is told
*that* the supports are not coaxial and never *by how much* — the one
fact that says whether the body is slightly off-axis (a modelling
tolerance question) or nowhere near it (a wrong-input question). The
decision is in hand at the refusal site; only the payload has nowhere
to put it.

Shape when this is taken: the same `ClassifiedMargin` E3 introduced,
on a new field or a new variant. `SpineUnsupported` is raised from two
other places with no margin to give (`arms.rs`'s table miss), so the
honest form is probably a sibling variant rather than an `Option` on
this one — the two situations are "no analytic arm exists for this
support pair" and "an arm exists but these supports are not coaxial to
`fillet3_support_coaxiality`'s satisfaction".

## 2. A neighbour's definite refusal folded into a margin-less tag — `corner_at`

`crates/sweep/src/blend/battery.rs:1497-1566`, the `Err(_) =>` arm at
1564. When a corner's neighbouring edge fails to resolve, the corner
reports `CornerConfig::Indeterminate` and the neighbour's own outcome
is dropped — including its margin, and including whether it was a
DEFINITE refusal or an escalation.

The fold itself is argued at the site (fix pass F6: one user
situation, one recourse sentence) and that argument is not disputed
here. What the comment already concedes is the item: *"What is
genuinely lost is the neighbour's own margin, which a future corner
taxonomy should carry as payload."* This issue is that future
taxonomy's row. Note the interaction with E3: now that a definite
refusal carries a `ClassifiedMargin` and an escalation carries
`Indeterminate`, the neighbour's outcome is a shape that could be
carried whole, which it was not when F6 was written.

## 3. The sweep's third blind spot — a margin under another field name

E3's sweep grepped two patterns: payload fields spelled `margin: f64`,
and `f64::NAN` in non-test `src/`. Its PR body discloses two blind
spots (a margin at scalar type; a sentinel that is not NaN). There is
a third, and it is the one that hides the most: **a margin that is not
called `margin`.** A field named for the quantity rather than for its
role is invisible to both patterns while being exactly the same class.

Live instance, and it says so itself:

- `crates/geom-brep/src/offset_meters.rs:196` — `headroom: f64`,
  documented on the line above as *"The classified margin `reach −
  |d|`, in metres."* A classified margin, projected to an `f64`, in a
  payload, under another name. **GEOM-BREP/CURVED's ground**, not
  FILLET's — filed here because E3's sweep is what turned it up, and it
  should be re-homed or cross-filed by whoever owns
  `offset_meters.rs`.

Second, weaker instance:

- `crates/geom-brep/src/props/quad.rs:3656` — `slack: f64`, a
  tolerance a test-support row carries. **PROPS' ground.** Listed for
  the census rather than as a defect: it is a threshold handed in, not
  a reading taken.

And the scalar-typed payload fields, which are E3's *first* disclosed
blind spot with concrete addresses attached — a payload generic over
the scalar holds `T`, so no `f64` appears and neither grep sees it:

- `crates/profile/src/path.rs:595, 611, 650, 688, 792` — five
  `margin: T` payload fields. **PROFILE's ground.**
- `crates/profile/src/sugar.rs:352, 378, 430` — three more.
  **PROFILE's ground.**
- `crates/topo/src/shell.rs:265` and
  `crates/topo/src/replace_face.rs:278` — `gap: T`. **TOPO's ground.**

These ten are listed **as census, not as defects**. Carrying the
scalar's own type is data-not-decision: unlike a projected `f64` it
loses nothing, and at `T = Interval` the enclosure is still whole. What
they lack is the *predicate and band* half of the shape — they say what
the number is but not what judged it — which is the weaker half of the
class and a separate decision for each owning program. The point of
listing them is that a future sweep for this class must search the
scalar-typed spelling too, and now has the addresses.

## 4. A façade question, not a code change — the prelude's matchability rule

`crates/pncad/src/prelude.rs:158-199` states the rule by which a type
enters the prelude: a caller who must MATCH on a payload needs its type
nameable without a deep path. E3 declined to add `ClassifiedMargin` to
the prelude, and the reviewer's NOTE-3 sharpens why that is a question
rather than a defect: `ClassifiedMargin` *is* nameable, at
`pncad::sweep::blend::ClassifiedMargin`, so nothing is unreachable —
what changed is that **seven `BlendError` arms now carry a payload the
prelude does not re-export**, where before they carried `f64`.

That is the prelude's own pre-existing rule meeting a wider surface,
not a hole E3 opened. The decision — extend the prelude with
`ClassifiedMargin` (and `Sign` and `MarginDiag` behind it), or state
that payload types are reached by path — is LIB's façade-curation call
and belongs with whoever next opens `prelude.rs`. Recorded here so it
is not rediscovered from scratch.

## Disposition

Rows 1 and 2 are FILLET's own ground and are the reason this is filed
under `work/fillet/`. Row 3's instances are named per row with the
program that owns each, and none of them should be edited by a FILLET
unit without an announced seam. Row 4 is LIB's.
