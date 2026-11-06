# TaigiKeyboard role workspace

Use en-US for replies and documentation. Read the checkout's own AGENTS.md and
applicable contribution guides before working. Those project rules still apply.
These roles describe responsibilities, not filesystem access controls.

Determine your role from the checkout path:

- `main/`: primary repository on `main`, holding shared Git metadata. Keep this
  checkout available as the baseline; do assigned feature work in its role checkout.
- `release/`: integrate reviewed, tested commits and prepare releases. Record
  the exact commit and test evidence. Publishing, tagging, pushing, or installing
  a release requires a user request covering that action.
- `developer/<feature>/`: implement only the assigned feature on its own branch.
  Keep dependencies, generated artifacts, and build outputs in this checkout.
  Report changes, relevant validation, and any unverified platforms.
- `tester/`: validate the requested committed revision. Record the commit,
  environment, commands, outcomes, and reproducible failures. This checkout is
  detached; send proposed fixes to a developer or debugger instead of changing
  production code as part of testing.
- `code-reviewer/`: review the requested committed revision against its base.
  Report actionable findings with severity, file/line, impact, and evidence.
  Identify test gaps. This checkout is detached; do not silently implement fixes.
- `debugger/`: reproduce failures, identify root cause at file/line, add a focused
  regression check where appropriate, and implement the assigned fix.

Do not switch, reset, clean, or modify another role's checkout. Do not change
shared Git configuration or hooks while another role is working. Do not merge
or cherry-pick another task's commits unless the current user request includes
integration. Worktrees share refs, objects, remotes, and most Git configuration;
uncommitted edits are private to each checkout. Hand off committed revisions.

Use `./wt` at the workspace root to manage worktrees. Removal never uses force.
Do not start additional agents merely because multiple worktrees exist.
