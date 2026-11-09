---
id: carrier-standing-leftovers-f37-tie-and-a-recourse-restated-by-variant
kind: issue
title: PR 6629's leftovers: F37's tie is unpinned and unmeasured, the touch sentence restates the kernel-limit recourse by variant, and MeetKind parallels BoundaryRefusal
status: closed
opened: 2026-11-09
closed: 2026-11-09
pr: 6648
branch: onecopy/carrier-standing-2
---


Found by the delta review of PR 6629 (`onecopy/carrier-standing`, b458842e). All NOTE-grade; the PR merged with them open.

1. **F37 (span `pieces_near`'s halve, now `choose`) lacks what F35 and F36 both carry:**
   - a measurement of ties over the suites;
   - a row pinning the tie. A mutant that halves on a tie stays green.

   The ledger row also omits the disclosure that the halve used to be logged (through `prove`) and no longer is.
2. **`TouchUnread::sentence` picks the walk's recourse by matching `EdgeTangent(Unwalked(Exhausted))`.** It spells "Recourse: loosen the tolerance, " + `kernel_limit_last_resort!()` instead of reading `EdgeTangentUnread::recourse()`, because `concat!` needs literals. A future reason whose recourse is the kernel limit would fall silently into "not yet". The row enumerates NoCurve, Unwalked×2 and OffInterior, but not NoReach or NoLength. This is the same class as 6629's F4: one fact restated per variant.
3. **`MeetKind` (`Escalated`/`Touch`/`Unread`/`Fault{what}`) runs parallel to `BoundaryRefusal`'s meeting arms**, and `BoundaryRefusal::of` is now a 1:1 rename of it. Two near-parallel enums, with no single home for the classification.
4. **Two fault-path texts changed and are unpinned:**
   - stacking's Desync `readings` ("a carrier the meeting table misread");
   - ring_path's `unreachable!` messages.
