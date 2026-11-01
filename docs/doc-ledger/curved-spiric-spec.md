# CURVED-SPIRIC-SPEC.md

CURVED-SPIRIC, the exact spiric rim carrier: `Curve3::Spiric` and `mint_carrier`'s arm (PR-1a, #2566) and `Pcurve::Spiric` with STEP export (PR-1b, #2861).

Deleted at PR-1b's merge, 2026-10-01; both PRs delivered.
Recover with `git show 981d7825a:docs/CURVED-SPIRIC-SPEC.md`.

Spec note at deletion: §3's sentence that `nurbs_tighten` "skips it as it skips `Harmonic`" was wrong — the harmonic arm answers a UV speed bound; PR-1b refused typed there instead, which was right. §5's export certificate is second-order where a cubic is fourth-order: `work/curved/spiric-step-spline-bound-is-second-order.md`.
