---
id: ellipse-edge-parameters-overstate-the-endpoint-miss
kind: issue
title: step-import: an ellipse edge's parameters are a scaled angle, not the vertex's nearest point, so the endpoint residual overstates the vertex's distance by up to a/b
status: open
opened: 2026-10-20
priority: P3
cost: M
---


(Filed by the ENCL implementer of PR 5454, `encl/one-culprit-chooser`, from its review.)

## What

`geometry::endpoint_params` (`crates/step-import/src/geometry.rs`, ~106-108) gives an ellipse edge's `t0` and `t1` as `atan2(w·v / b, w·u / a)`: the eccentric anomaly of the vertex's offset from the centre, scaled per axis. That is the exact parameter of a point on the ellipse. It is not the nearest point's parameter for a vertex off it.

Edge certification's endpoint residual (`CertCheck::EndpointStart`/`EndpointEnd`, `crates/geom-brep/src/certify.rs`) measures `|carrier(t0) − vertex|`. For an ellipse with semi-axes a ≥ b, that overstates the vertex's distance from the carrier by up to a/b: 1.25× for a 2:1 ellipse. A file whose vertex lies within its own ε_in of the ellipse can therefore read a miss past ε_in. It is then told it is damaged.

## Not the file's to answer

The import door cannot honestly blame the file for this miss: the kernel derived the parameter it reads. So `recourse::Provenance` reads every unsized check, the endpoint residuals included, as `Derived` at the import door, and the refusal names the kernel or the file (PR 5454).

## Repair shape

Give a vertex off the carrier its nearest-point parameter (a certified foot-point on the ellipse), or bound the scaled angle's overstatement and read the residual against it.

Note (PR 5464, 2026-10-20): Ev ruled that the import door names the file for every defect ending a bad file reaches, stored or derived, and `recourse::Provenance` is gone. Past ε_in this refusal now ends in `DAMAGED_FILE_ENDING` ("the file is damaged"), so a sound file refused here is told it is damaged: the defect now also makes the message false, which raises its cost. The paragraph above describes the state before that PR. (ENCL implementer)
