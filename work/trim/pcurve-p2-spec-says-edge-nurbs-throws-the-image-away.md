---
id: pcurve-p2-spec-says-edge-nurbs-throws-the-image-away
kind: issue
title: PCURVE-P2-SPEC says edge_nurbs THROWS IT AWAY; the image is the certificate's input and the advice it gives was already taken
status: open
opened: 2026-09-11
---


Filed 2026-09-11 by S-TCOST, onto TRIM's slate because
`docs/PCURVE-P2-SPEC.md` and `crates/geom-brep/src/edge_nurbs.rs` are
both in this program's `paths` (`work/README.md`: file a finding
straight onto the owning program's slate). Nothing here is dispatched
and nothing is asked of TRIM's order — it is one false sentence, with
the measurement that falsifies it.

## The sentence

`docs/PCURVE-P2-SPEC.md:55-62`, in the "producers already exist and
should be considered before writing a third" note:

> `geom-brep/src/edge_nurbs.rs:330-336` already derives exactly this
> image (33 certified foot points, interpolated on the carrier's own
> parameter, `on_carrier_domain`-lifted) at EDGE certification time and
> **then THROWS IT AWAY**, returning only `PlaneNurbsLimbs` scalars

Two things in it are false at `d6a9b948`, and a third has gone stale.

**1. It does not throw it away.** `edge_nurbs.rs:354-362` passes the
image into the certificate as `Some(&pcurve)`, and
`ssi/certify.rs:795 certify_branch` refuses `UnsupportedCertificate`
without it in two places — `:810-815` (*"a NURBS operand's limbs need
the traced pcurve"*, limbs 1 and 2 on the wall side) and `:851-854`
(limb 3's chart uniqueness tube). Every `PlaneNurbsLimbs` field except
`min_sin_theta` comes out of that certificate. The image is the lane's
INPUT, not its discard: remove it and the lane returns nothing.

**2. The line numbers drifted.** `:330-336` now lands inside the
transversality hook's tail, not on the derivation. The derivation is
`:308-336` and the producer itself is `:407-450`.

**3. The advice was already taken, which is why the sentence now reads
backwards.** `chart_image` IS the shared `pub(crate)` producer the note
was asking someone to prefer over writing a third — its own doc comment
carries the "Two consumers, one producer" contract, and
`edge_nurbs.rs:301-303` says so at the call: *"ONE derivation of this
image exists in the tree, and this lane certifies the same bits the
mint stores."* The note describes the tree before that refactor.

## What it cost, so the fix is worth the edit

The sentence was read as a finding and filed as one:
`work/tcost/edge-nurbs-computes-the-chart-image-and-discards-it`,
opened 2026-09-03 as a compute-and-discard candidate on the strength of
this line, carried on S-TCOST's board for eight days, and closed
2026-09-11 only after a lane instrumented the call to find out that the
premise was false. That item's `## Closed` section holds the
measurement and is the citation for everything above.

## What this is not

Not a claim that anything in the code should change. The
cross-pass redundancy that DOES exist — the mint
(`pcurve_cache.rs:1229 general_image_lane`) derives its own image, and
the certifier re-derives rather than reading the stored one — is
deliberate under `topo/src/validate.rs:3297-3299` (*"Re-certification
re-derives; it never trusts the stored certificate"*), and touching it
is an `[ev]` design conversation, not a prose fix. Measured share of an
edge certification, LOCAL and not a result of record: ~9.5 % release,
~12.6 % dev, ~0.46 ms per plane×NURBS edge in release.

## The fix

Rewrite the sentence to what the tree does: `chart_image` is the shared
producer, both consumers already go through it, and the image is the
rung-3 certificate's input on the wall side. Re-derive the line numbers
at the edit rather than copying the ones above — they are frozen at
`d6a9b948`. If the spec is due for deletion under
`docs/DOC-LEDGER.md`'s spec lifecycle before anyone would read it
again, saying so and closing this row is an equally good answer.
