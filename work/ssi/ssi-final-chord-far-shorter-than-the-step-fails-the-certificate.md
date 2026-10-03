---
id: ssi-final-chord-far-shorter-than-the-step-fails-the-certificate
kind: issue
title: ssi: a branch whose last marched state lands 2e-11 to 1e-7 m inside the domain boundary fails its certificate at ε 1e-12 (a final chord far shorter than the step)
status: open
opened: 2026-10-01
priority: P2
cost: M
---


(SSI implementer `ssi-short`, from the open-end analysis of PR "SSI: a
short branch is re-marched at its own length, and BranchUndersampled
retires", 2026-10-01.)

## What

`ssi/march.rs`'s `march` steps on from a state whose
`ssi_branch_open_end` margin is definitely positive, and `push_boundary`
then appends the crossing it bisects, however close to that state it
lies (it drops only an exact duplicate). When the last marched state
sits a little inside the domain's boundary, the trace's final chord is
that distance long while every other chord is a full step, and the
cubic interpolated through it (`ssi.rs`'s `fit_branch`) does not
certify at a fine tolerance.

Fixture: `m5_pr7_ssi.rs`'s
`a_marched_state_in_band_of_the_domain_boundary_escalates_the_open_end`
(the plane `x = 0.5` across a flat wall of height
`(31/32 + δ)·256/255`, extent 1 m, so the 31st state sits `δ` below the
top edge; steps are 3.1 cm). Measured on the PR's head:

| δ | ε 1e-6 | ε 1e-9 | ε 1e-12 |
|---|---|---|---|
| 1e-11 | certifies | certifies | escalates the open end (in band) |
| 2e-11 | not run | not run | `CertificateLimb { OnLocus, 3.1e-11 }` |
| 1e-10 | certifies | certifies | `CertificateEscalated { OnLocus }` |
| 1e-9 | certifies | escalates the open end (in band) | `CertificateLimb { HullSup, 1.3e-10 }` |
| 1e-8 | certifies | escalates the open end (in band) | `CertificateLimb { HullSup, 7.6e-11 }` |
| 1e-7 | certifies | certifies | `CertificateEscalated { HullSup }` |
| 1e-6 | escalates the open end (in band) | certifies | certifies |
| 1e-4, 1e-3 | certifies | certifies | certifies |

The first march here runs at the caller's extent, which the PR leaves
unchanged, so this predates it. The same mechanism is why reading an
in-band open end as "inside" refuses where reading it as "at the
boundary" certifies (the PR body has that table), so it bears on
whether `ssi_branch_open_end` may resolve its band without escalating.

## Open

Whether the final crossing should replace a last state that lies
within some fraction of a step of it (as the closure path already does
for a state essentially on the seed), or the fit should take the
uneven chord some other way. Either is a stepper or fit choice.

## The plane × NURBS lane (PR 3862)

The boundary pass retired `push_boundary` on this lane: a branch ends at
the crossing the pass certified, and `ssi/ends.rs`'s `close_at` lets a
last marched state nearer that crossing than half its own step give way
to it. The fixture above (as `m5_pr7_ssi.rs`'s
`a_branch_whose_last_state_lands_a_hair_inside_the_wall_certifies`)
certifies at every δ in the table at ε 1e-6, 1e-9 and 1e-12. What is
left is the ℝ³ lane, whose `push_boundary` still appends the bisected
slab crossing however close it lies to the last state; that lane's slab
is itself in question (`ssi-r3-slab-is-not-geometry`).
