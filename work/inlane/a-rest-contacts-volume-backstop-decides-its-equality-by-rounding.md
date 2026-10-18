---
id: a-rest-contacts-volume-backstop-decides-its-equality-by-rounding
kind: issue
title: A rest contact's volume backstop decides its equality case by rounding
status: open
opened: 2026-10-18
priority: P2
cost: M
---

`topo/src/boolean/ops.rs` (around 2334): `volume_backstop_violation`, read under `Posture::Bound` at the exact band, is exactly 0 at the nominal on a union of a box and a boss resting flush on its top cap (the corpus `face_sketch` since INTENT stage 3 D, which builds the boss as a copy placed on the cap and unioned). Moving every corner variable by 1e-7 makes it −5.8e-18 (Negative). The revalued run then takes an extra `volume_backstop`, and the build departs at decision 7364 of 8721. So a rest contact's equality case is decided by the rounding sign of a volume difference. Found by the D diagnosis lane, probe `p3_`.

`revalue_corpus` drops `face_sketch` from its compared rows (beside `kiss_carry`) until this is settled.

**Done when** the backstop's verdict at a flush rest is a function of the contact (decided structurally, or Zero within a band its arithmetic justifies), so that `face_sketch` revalues on one branch under the apart move and its row comes back.
