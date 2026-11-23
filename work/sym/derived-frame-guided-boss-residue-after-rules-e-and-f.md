---
id: derived-frame-guided-boss-residue-after-rules-e-and-f
kind: issue
title: Three derived-frame documents still refuse under Guided after rules E and F: tilt-UV at every width, two stacked frames, and the boss on a revolved cap
status: open
opened: 2026-10-13
priority: P2
cost: H
---


## What

These are what is left of `derived-frame-placement-freezes-on-the-symbolic-lane`
(closed 2026-10-13) and `a-face-frame-on-a-revolved-cap-refuses-on-pcurve-loop-continuity`
(closed the same day). Measured on `main` at `621e036e19`
(`m10_derived_frame_tilted_interval`, release, ε default), they are
three derived-frame documents that still refuse under
`ProfileLift::Guided` with rule E and rule F on. All three certify
under `Pinned`.

| document | refusal under `Guided`, half `1e-3` |
| --- | --- |
| tilt about `u` and `v` (`tiltUV`) | `carrier_endpoint_start` at the boss's attachment gate, `[0, 1.583e-1]` (158× the half-width). Its ceiling is "certifies NaN, refuses 1e-4": it refuses at every width measured. Rule F moves nothing. |
| two derived frames stacked (`tiltV stacked-2`) | 1 refusal: the boss's stored interval span escalates, `[0, 2.292e1]` (4 with rule E off, 7 on plain `Interval`) |
| the boss on a `FaceFrame` from a revolved body's end cap (`tiltV revolved-cap`) | 1 refusal: the boss's `carrier_endpoint_start`, `[0, 1.398e-1]`, frozen 897. The revolve no longer refuses (PCERT retired `pcurve_loop_continuity`). With rule E off, 4 refusals. |

SYM-12's render already explains the tilt-UV document: a `Sub` freezes
on its kids' size, a cross-multiplication of `102·69 + 204·35` terms
over a degree-108 denominator, and no sign rule reaches it. The other
two are unrendered.

## Why P2, not the P0 it came from

The closed P0 was "a derived frame whose AXES carry a widened parameter
does not certify". That is answered:
- the document it was filed on certifies (half-width ceiling 1.55e-3
  under `Guided`, 5e-2 under `Pinned`);
- so does the tilt-`u` family on either cap and sign, and the `tiltNZ`
  document.

What remains is the tier's reach on three further shapes, all of which
certify under `Pinned`. That is interval propagation, the README's P2
band.

## What answers it

Render each document's refusing residual first, as SYM-12 did for
tilt-UV. Then decide per document whether the tier, the attachment
gate's own enclosure, or nothing answers it. FRAME's
`a-widened-derived-placement-normalises-a-straddling-newell-sum` (the
clause-1 Newell refusal) is a different predicate, and none of these
three refuses on it.
