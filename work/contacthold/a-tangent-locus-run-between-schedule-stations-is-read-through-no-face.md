---
id: a-tangent-locus-run-between-schedule-stations-is-read-through-no-face
kind: issue
title: line_through_both_faces reads a non-line tangent locus at schedule stations, so a run of both faces' interiors shorter than their spacing reads 'through no face' rather than refusing
status: open
opened: 2026-11-03
priority: P2
cost: M
refs: [census-confirms-a-two-carrier-face-pair-at-one-midpoint, the-tangent-interiors-schedule-escalates-on-a-sample-it-places-in-band-of-a-corner]
---

Filed by SPLICE's `splice/kind-reads` lane (PR 6437), from its sweep for
sites where a sampled refutation answers a question about part of a curve:
the class of that PR's review finding, where one sample of a spline read
off a plane answered "lies in the face along no piece".

## Finding

`crates/topo/src/boolean/mod.rs` `line_through_both_faces` answers
whether a tangent locus runs through two faces' interiors: some point of
it over `overlap` strictly inside both. It cuts the overlap where either
trim's boundary can meet the locus (`boundary_cuts`), exact for a line
against trims whose carriers meet it in closed form. Where the locus is
not a line, or a trim edge is a spiric or a spline, or a meeting is not
placed, the cuts are the certificate schedule's parameters
(`geom_brep::sample_param` over `CERT_SAMPLES`), and each run between two
is read at its middle only. Its doc states the consequence: "a run of that
face's interior shorter than their spacing can be missed". Every station
read out of one face then returns `Ok(false)`: the locus runs through no
face, a definite answer, where the truth is unread.

Two readers act on that `false`: the op's glue door
(`locus_through_both_faces`, a declared tangency's run through both
faces) and the census's face-face confirm (`census::confirm_face_pair`).
A false `false` there drops a tangency's face-face contact (no row, no
refusal) wherever the faces' shared interior along the locus is a short
run between stations: a circle tangent locus whose run inside both
trims is shorter than the overlap over `CERT_SAMPLES - 1`.

## Fix

Where the cuts are not exact, an answer of no station in both faces is
not `false`: refuse typed (the run is unread), or cut at a certificate of
the trims' meetings with the locus (the curve-pair reading against each
trim edge, as the line lane's closed forms cut it), with a row whose run
through both faces falls between two schedule stations.

