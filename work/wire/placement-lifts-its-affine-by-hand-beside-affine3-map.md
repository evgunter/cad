---
id: placement-lifts-its-affine-by-hand-beside-affine3-map
kind: issue
title: Placement::linear and Placement::affine lift twelve components by hand from [[f64;3];3] + [f64;3] into Mat3<T>/Affine3<T>, one construction away from self.affine::<f64>().map(T::from_f64)
status: closed
opened: 2026-09-08
refs: [2139]
pr: 2375
branch: wire/placement-affine-map
closed: 2026-09-11
---

(EVAL orchestrator) Filed from EVAL-1's sweep (PR 2139), which retired
`editor-core`'s second home of the per-coordinate affine walk into
`Affine3::map` / `SketchPlane::map`. The same shape survives in
`crates/editor-core/src/placement.rs:202` (`Placement::linear`) and
`:215`–`:218` (`Placement::affine`): a hand lift of the nine matrix
components and the three translation components from the stored `f64`
arrays into `Mat3<T>` / `Affine3<T>`, where `self.affine::<f64>().map(T::from_f64)`
is one construction. `placement.rs` is in no program's `paths`
(`work/eval/program.md` keep_out lists it among the unowned,
unfinished files), so the finding waits here; the program that draws
that fence takes it. Citations accurate at `bd2fe4289`.

## Re-homed to WIRE (2026-09-11, the cut in `docs/WORK-TRACKS-2026-09.md` addendum 3)

WIRE is the evaluation seat's successor. `docs/DOC-LEDGER.md` sweep 9
parked three of these rows in `work/issues/` as "the successor's opening
slate" when EVAL closed on 2026-09-08, naming two more already there;
this program is that successor, and it takes the rest of the seat's
ground with them.

Its class at the cut was **E** — `Mat3::map`/`Affine3::map` already
exist; two functions collapse to one call. The class is a dispatch
estimate made by reading the row against the tree on 2026-09-11, not a
verdict on the finding, and a lane that finds it wrong says so in its
PR. The id, the `track:` letter where the row carries one, and the body
above are unchanged by the move.

## Closed (2026-09-11) — PR 2375, merged

`crates/editor-core/src/placement.rs` now has **one** place where the
stored arrays become geometry. Two private doors (`linear_f64`,
`affine_f64`); `linear<T>`, `affine<T>` and `determinant` each one step
off them; and `compose`'s general arm is
`Frame::from_affine(self.affine_f64() * inner.affine_f64())`, which also
retired a hand re-derivation of `Affine3`'s own `Mul`. Public signatures
unchanged.

**The item's suggested spelling does not compile.** `self.affine::<f64>().map(T::from_f64)`
as the body of `affine` is unbounded recursion at `T = f64`; the f64
value has to come from a non-generic door. Recorded because the row
asserted it as "one construction".

**Full review** (raised from the light posture because the unit went
past its item onto ASM-4 D-3's bit-exact path): APPROVE-WITH-FIXES,
**0 MAJOR**. All five claims survived, including a 200 000-pair
differential test of old-vs-new `compose` over `f64::from_bits` inputs.
Two mutations the unit's own tests did **not** catch were fixed: a
fixture with no `-0.0` in its columns (so a `+ 0.0` snap passed), and a
compose oracle built from `Mat3::Mul`, the operator under test. The
oracle is now thirty-six explicit scalar operations sharing no operator
with its subject, and a pure reassociation reddens it.

### Residues, each with a file

- `work/props/geom-core-linalg-has-no-array-doors.md` — the absent
  `[f64;3] ↔ Vec3` / `[[f64;3];3] ↔ Mat3` conversions, which are why
  this file's two remaining lowerings and `mate.rs:141` cannot collapse.
  Five non-test sites, one of which (`geom-brep/src/ssi/system.rs:92,96`)
  has already written the door privately by hand.
- `work/props/nan-sign-is-not-stable-under-code-motion-so-d9s-fixed-order-covers-non-nan-only.md`
  — found by the differential test in release mode.
- `frame-linear-generic-door-has-no-consumers` — `pub fn linear<T>` now
  has zero call sites workspace-wide.
- `placement-rs-states-its-exactness-rule-in-nine-paragraphs` — S6/S8/S9
  from the reviewer's whole-file read.
- `work/issues/inert-allow-attributes-on-test-modules-are-house-style-and-half-are-unneeded.md`
  — the class behind this unit's own inert `#[allow]`.
