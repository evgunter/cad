---
id: coaxial-cones-a-hair-apart-refuse-on-the-sweep
kind: issue
title: Two coaxial cones whose apexes stand a hair apart refuse on the value's sweep, where the retired coaxial arm read the parallels
status: open
opened: 2026-11-24
priority: P2
cost: M
---

A narrowing disclosed by U2 (`the-section-certificate-reads-the-cone-pairs-on-the-value`, PR 7637), pinned by `topo` `section_cert_cone_pair_rows::coaxial_cones_a_hair_apart_refuse_on_the_sweep`.

**The pose.** `cone_search` drew it with seed `0xbe01201603b110c4`. The first cone has `α ≈ 0.944`, about 80 m from the origin. The coaxial partner has `α₂ ≈ 0.327`, and its apex stands `2.1·10⁻⁷` m up the first cone's axis.
- The retired coaxial arm read the two parallels in closed form, essential on both.
- The value decides the apexes apart. Its sweep then meets the discriminant in the band (`ruling_discriminant`), because the sheets pass all but through the near apex.

The value now refuses such poses wherever they stand within the general search's must-answer distance, and `cone_search` excuses that refusal (`cone_pair_search::must_answer`). That excuse is the narrowing this row schedules.

**What would close it:** the value reads a coaxial pair's parallels in closed form, or the sweep resolves a sheet near an apex it does not pass. Then `cone_search`'s excuse for cylinder and cone partners narrows back to `4·escalate`.
