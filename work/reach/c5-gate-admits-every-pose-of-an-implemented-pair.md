---
id: c5-gate-admits-every-pose-of-an-implemented-pair
kind: issue
title: The C5 gate reads PairRoute::implemented per KIND pair - a pose the arm would refuse passes the gate; sibling arms divide unguarded
status: open
opened: 2026-09-05
refs: [VERBS-C5ARMS, VERBS-CONE, 1864]
---


## What

Two class findings from the C5ARMS PR-2 dual (both arms, and R2 S3):

1. `replace_face.rs`'s C5 gate reads `PairRoute::implemented` as a
   boolean per KIND pair. An implemented arm is configuration-scoped
   (coaxial cone×cylinder; axial plane×torus), so a tilted or offset
   pose the arm would refuse `RoutesToGeneralRung` passes the gate and
   relies on attach/validate to re-certify downstream. No revolve
   builds such a pose today; `PairRoute::implemented`'s own doc says
   "closed forms exist and are wired", and nothing is wired for the
   general pose. Same shape for `(Plane, Cone)` and `(Plane, Torus)`.
2. Sibling arms' guards are uneven: the new cone×cylinder arm guards
   `α ∈ (0, π/2)`; `plane_cone_section` divides by `sin α` and `cos α`
   with no guard (`intersect.rs`); `ConeOffset::new` / `apex_shift`
   are the next places to look. Either the siblings owe the guard or
   the new one is a comment doing work — decide once, sweep the file.

## Fix shape

(1) is the gate asking the arm (a pose-scoped `route` answer, or a
`PairRoute::implemented_for(&pose)`) — a `replace_face.rs` change,
CURVED's C5 ground, best taken with VERBS-CONE's operand lanes.
(2) is a one-file sweep with a red-first row per unguarded division.

## Home

CURVED — the C5 section arms and their gate are this program's
(`VERBS-C5ARMS`, `VERBS-CONE`).
