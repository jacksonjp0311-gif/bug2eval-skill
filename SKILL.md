---
name: bug2eval
description: "Capture a real fixed bug as a portable deterministic regression eval (.b2e), validate the before-fails/after-passes contract, and replay it against any coding agent. Use when turning a solved bug into a reusable evaluation case or when solving a Bug2Eval case."
type: workflow
lifecycle: active
---

# Bug2Eval Skill — Portable Regression Evals for Coding Agents

Turn a concrete fixed bug into a durable, checksummed evaluation case that any agent can replay. The case preserves the broken workspace, the fixed workspace, a protected verifier, and the exact contract that before fails and after passes.

Use this skill after you have a reproducible bug, a verifier that distinguishes the states, and both snapshots available. Do not create cases from unverified claims.

## Core Invariant

A valid case MUST satisfy:

```text
before_exit != 0
and
after_exit == 0
```

Never weaken the verifier or the contract to make a case pass.

## Workflow: Capture a Case

Prefer Git refs when the fix is committed.

```bash
bug2eval capture \
  --id <ID> \
  --title "<TITLE>" \
  --before-ref <BROKEN_REF> \
  --after-ref <FIXED_REF> \
  --verify "<VERIFIER_COMMAND>" \
  --protect <PATH_TO_VERIFIER_OR_HELPER>
```

Or from directories:

```bash
bug2eval capture \
  --id <ID> \
  --title "<TITLE>" \
  --before-dir <BROKEN_DIR> \
  --after-dir <FIXED_DIR> \
  --verify "<VERIFIER_COMMAND>"
```

Then validate:

```bash
bug2eval validate .bug2eval/cases/<ID> --json
```

Accept only when validation confirms the invariant and checksums.

## Workflow: Replay Against an Agent

```bash
bug2eval run <CASE_OR_B2E> \
  --agent-cmd '<AGENT_CMD using {workspace} and/or {task_file}>' \
  --json
```

The agent receives an isolated workspace starting from the before snapshot, the task prompt, and must produce a fix that makes the exact recorded verifier exit 0. Do not edit the case itself while evaluating.

## Workflow: Solve a Case

1. Start from the before workspace (extract workspace_before.tar.gz).
2. Read prompt.md or the generated BUG2EVAL_TASK.md.
3. Apply the smallest correct fix.
4. Never edit, remove, bypass, or neutralize the verifier or protected files.
5. Run the exact verification.argv (no shell).
6. Exit 0 only if the verifier exits 0.

## Portable Format

- metadata.json is the machine contract (schema_version 1.0).
- verification.argv is executed without a shell.
- All artifacts are SHA-256 checksummed.
- .b2e is a ZIP containing one case at its root.
- Protected files must match exactly between before and after and must not change at run time.

## Companion Tool

The reference implementation lives in https://github.com/jacksonjp0311-gif/bug2eval (Python, zero runtime deps).

A Rust-first companion binary is planned/available in this repository for single-static-binary distribution with no interpreter required. Agents may invoke either.

See references/AGENTS.md for full agent protocol and references/FORMAT.md for the case schema.
