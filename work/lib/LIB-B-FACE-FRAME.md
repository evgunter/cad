---
id: LIB-B-FACE-FRAME
kind: unit
title: binding census family B-FACE-FRAME
status: open
opened: 2026-09-04
---


Queued mechanical census family (the B-RESOLVE shape): sweep the
family's bindings against the census contract, construct the
previously unconstructible pins where the surface now allows, and
re-cut the census rows honestly. Families share the census/tags/test
files, so at most two run concurrently, staggered.

## Derived scope

`crates/pncad-py/tests/test_binding_census.py` charters `B-FACE-FRAME`
in `FAMILIES` (DOCM-1, PR 1829): `Datum.face_frame` (a `FaceFrame`
constructor taking the body node, a face `StableName` and a spin
angle), `Pose.sense` (the face's orientation sense beside its axis, so
a Python caller forms the outward normal as `sense * axis` exactly as
Rust does), and the carrier-kind read `face_carrier_kind` (a face name
in, its stored `SurfaceKind` tag out). One `NOT_BOUND` entry cites the
family today (`face_carrier_kind`); the other two names have no row,
which DOCM-1's review noted — the sweep decides whether the census
wants them listed or bound.

## Home

LIB's, filed by DOCM at DOCM-1's merge (the Python surface is outside
DOCM's fence). Same class, same shape, unscheduled alongside it:
B-DISTRIBUTIONS, B-MEASURES, B-NOTATION.
