# Dogfood ship-ticket and review-change on boot ticket 09

Type: task
Label: wayfinder:task
Status: claimed

Blocked by: 03, 04, 05, 06

## Question

Prove both skills on a live ticket. Run `/ship-ticket .scratch/rust-hive-boot/issues/09-nextest-and-grant.md`. From a second session run `/review-change <PR number>`, then `/review-change <PR number> post` once the chat report reads right.

[POST a Load with Idempotency-Key](../../rust-hive-boot/issues/08-create-load-post.md) must be resolved first; [Prove nextest profiles and GRANT](../../rust-hive-boot/issues/09-nextest-and-grant.md) is blocked on it. If 08 is still open, wait. Do not ship 08 on this map.

Record every friction point as you go: a step the skill skipped, a command that failed, a rule that fired wrong or stayed silent, a finding with a wrong line, a TTSR false positive. Fix each in the skill, prompt, or rule file that caused it, on branch `hive-law/07-dogfood-on-boot-09`. Law files stay untouched.

Acceptance: the boot PR is merged by the code owner with ticket 09 resolved by the skill; the fixes PR lists each friction point and the file it changed; the answer names any TTSR rule deleted for false positives.
