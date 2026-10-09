---
id: pcurve-image-mismatch-at-the-body-mint-asks-to-re-mint
kind: issue
title: pcurve certification: ImageMismatch reached through the body's pcurve mint asks the caller to re-mint the image the mint itself built
status: open
opened: 2026-10-09
priority: P3
cost: E
---

(ENCL, from the sweep of `offset-fit-nan-residual-at-the-mint-asks-the-caller-to-re-fit`:
the same shape, a certifier's refusal written for a caller handing in
an image, forwarded by a mint that built the image itself.)

## What

`PcurveCertifyError::ImageMismatch`'s `Display`
(`crates/geom-brep/src/pcurve_cache.rs`, the `ImageMismatch` arm) ends
"Recourse: re-mint the image from the carrier (chart_pcurve, or the
body's pcurve mint) rather than building it by hand". That is right for
a caller that built an image by hand and offered it to the certifier.

`topo`'s pcurve mint (`crates/topo/src/pcurves.rs`, `mint_pcurves` and
the `certify` closures that wrap the door's refusal in
`PcurveMintError::Certify`) forwards the certifier's refusal whole, and
`PcurveMintError::Certify`'s `Display` renders it verbatim. If the mint
can reach `ImageMismatch`, its user reads "re-mint the image ... rather
than building it by hand" about an image the mint built, and the re-mint
is the call that refused. A mismatch there is the kernel's defect.

Not checked: whether any mint path can actually raise `ImageMismatch`.
The mint builds its images from the carrier, so the arm may be
unreachable from it.

## Repair shape

Decide reachability first. If the mint reaches it, it ends in the
kernel-defect ending on that route, told apart from the hand-built
image by its door rather than its text (the offset fit's `MintLimb`
next to `Limb` is the same split).
