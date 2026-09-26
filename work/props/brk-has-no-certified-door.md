---
id: brk-has-no-certified-door
kind: issue
title: Brk is a bracket carrier with no certified door, so extremal_angle_interval holds CertifiedEnclosure's NaN-end postcondition by hand
status: open
opened: 2026-09-26
---


## The finding

Raised by ENCL's H11 sweep (pointing the re-derived *"a `Some` never
carries a NaN end"* sites at `geom_core::CertifiedEnclosure`'s trait
docs, `crates/geom-core/src/real.rs`).

`geom`'s `curves::boxes::extremal_angle_interval`
(`crates/geom/src/curves/boxes.rs`) returns `Option<(f64, f64)>` over
the private `Brk` outward bracket, and keeps the certified door's
postcondition by hand: an entry guard `is_nan` on all four ends of its
two `Brk` operands, and a per-corner `is_nan` after `atan2`. H11 now
points that doc at the trait; it does not change the code, because the
one-line alternative — `impl CertifiedEnclosure for Brk`, and
`u.certified_bracket()?` at the guard — is not a doc edit.

**Why it is a design question and not a refactor.** Every `Brk` is
minted by `Brk::of<T: Bounds>`, the STORAGE bracket door. That door
does not certify: a `Trv` interval (`sqrt([−1, 4])` clamping to
`[0, 2]`) crosses it as a sound bracket with finite ends. So a
`CertifiedEnclosure` impl on `Brk` that refuses only on NaN would
certify values whose computation left its domain — the laundering the
trait exists to refuse. The module header says why the file reads
`[lo(), hi()]` and not the certified door: it is a sole-`Bounds`
box-constructor seam under the 2026-07-29 amendment, and poison flows
to the poison box, which overlaps everything.

So the choices are, undecided:

1. **Leave `Brk` a bracket carrier.** The guard stays, pointed at the
   trait as it now is. Nothing to do.
2. **Mint `Brk` through the certified door** (`Brk::of<T:
   CertifiedBounds>`, refusing on `None`), after which a
   `CertifiedEnclosure` impl on `Brk` is honest. This changes which
   inputs get a finite box (a `Trv` operand would reach the poison box
   where it gets a sound finite one today), and it changes the file's
   bound, which the Bounds scope rule governs.
3. **A bracket-carrier postcondition on `Bounds`-side helpers**, i.e.
   state "a `Some` never carries a NaN end" once for non-certifying
   bracket producers too, so a site like this points at a rule that is
   about it rather than at the certified door's.

Also noted, not a design question: the per-corner `c.is_nan()` guard
is unreachable once the entry guard has passed — `atan2` of two
non-NaN `f64`s (±1 times a non-NaN end) is never NaN. Deleting it or
keeping it as belt-and-braces is the owner's call.
