---
id: the-c2-curved-bullet-describes-a-box-reading-the-touch-analysis-does-not-do
kind: issue
title: README C2 says the touch analysis reads a curved face of a touch through its reach box and certifies a rest on it; the analysis refuses every curved face (TouchUnreadable)
status: open
opened: 2026-10-19
priority: P3
cost: E
design: true
---

Filed by RESTREAD's link-reading lane (branch `restread/link-touch-reading`).

`crates/topo/README.md` C2 ("Face-pair procedure"), second bullet: a
cross-solid pair with a curved side clears "where the pair meets, by the
touch analysis, which reads every curved face of a touch's star through
that box (its corners' signed distances from the candidate plane): a box
on its side certifies the Rest". `census.rs`'s touch analysis builds a
face's sector only for a planar, line-bounded face (`Sector::of`:
`planar_face` and `FaceGeo::polygon`, else `TouchVerdict::Unreadable`),
before the link reading and since; no box reading of a curved face
exists, and "star" and "candidate plane" name what the link reading
retired.

Owed: decide whether the clause records a reading still owed (then file
it as the work) or is stale (then re-word it, a ratified clause, with Ev).
