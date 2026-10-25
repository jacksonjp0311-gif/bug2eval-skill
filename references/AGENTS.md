# Bug2Eval Agent Protocol

## Product Invariant

A valid case must show the verifier failing on the **before** workspace and passing on the **after** workspace:

```
before_exit != 0
and
after_exit == 0
```

Do not weaken this invariant.

## Stable Interface

- `metadata.json` is the canonical machine-readable case contract.
- `schema_version` is currently `1.0`.
- `verification.argv` is an argument array executed without a shell.
- `artifacts.*.sha256` must match before extraction or execution.
- `.b2e` is a ZIP archive containing one case at its root.
- Verifier exit code `0` means pass; any other code means fail.

## When Solving a Case

1. Start from `workspace_before.tar.gz`.
2. Read `prompt.md` / `BUG2EVAL_TASK.md`.
3. Make the smallest correct fix.
4. Do not edit, remove, bypass, or neutralize the verifier or protected files.
5. Run the exact verification `argv`.
6. Return the workspace with the real fix applied.

## Integrity Rules

- Protected files must be identical in both snapshots and unchanged at evaluation time.
- Never extract archives without path traversal checks.
- A solver that returns nonzero or times out cannot receive a passing score even if the final workspace would pass the verifier.
