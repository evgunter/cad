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

Where "re-mint" is right: `topo/src/pcurves.rs`'s re-certification of a
STORED row (`restate`, called from the carried-rows pass beside it, which
wraps its refusal in `PcurveMintError::Certify` too). There the image was
not made by this call, so re-minting it from the carrier is a real
recourse. The two readings share one wrapper, which is why the door,
not the text, has to tell them apart.

Related rows on this slate:
`pcurve-certify-error-arms-with-no-ending.md` (the arms whose `ending`
is `None`, `ImageMismatch` among them) and
`recourse-chain-stops-at-pcurve-certify-error.md` (the chain that stops
at this carrier). A fix to either may settle this one's text too.

## Repair shape

Decide reachability first. If the mint reaches it, it ends in the
kernel-defect ending on that route, told apart from the hand-built
image by its door rather than its text (the offset fit's `MintLimb`
next to `Limb` is the same split).

- 2026-11-02 — moved from PCTAIL to PCERT by `git mv` when PCTAIL merged into PCERT, id and body unchanged (the track budget now counts only P0, P1 and unbanded rows, and PCTAIL had fallen far under it).
- 2026-11-03 — Measured on `pcert/certify-arms-end` (PR 6448). `ImageMismatch` no longer asks to re-mint: it ends by `PcurveCheck::ImageIdentity` (`Unsized::Defect`), the kernel's defect at a build and the stored culprit at rest. A public build door does reach it: `PcurveCache::certify` is public through pncad's `geom_brep` re-export, and `pcurve_general.rs`'s `the_closed_form_door_refuses_a_general_image` handed it a `General` image. That wrong-door case is now its own arm, `FittedGradeAtClosedFormDoor`, ending in the hand-off lever ("hand a Fitted or General image to certify_fitted or certify_general"). The remaining `ImageMismatch` sites, a caller's hand-built image among them, still end in the defect at a build: filed as `geom-brep-public-certify-doors-end-caller-input-as-the-kernels-defect-at-a-build`. Whether the body's mint reaches `ImageMismatch` is still unmeasured.

