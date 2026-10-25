# bug2eval-skill

Portable agent skill for capturing, validating, and replaying Bug2Eval cases.

Any coding agent can pull this skill and use it to turn real fixed bugs into deterministic regression evaluations (`.b2e` bundles) or to solve existing cases under the exact before-fails / after-passes contract.

## What this is

This repository contains the skill definition (`SKILL.md` + supporting files) that agents load to work with Bug2Eval. It is a companion to the reference implementation at [jacksonjp0311-gif/bug2eval](https://github.com/jacksonjp0311-gif/bug2eval).

The skill teaches agents:

- When and how to capture a case from a Git range or directory snapshots
- How to validate the invariant (verifier fails on before, passes on after)
- How to replay a case against an agent harness
- How to solve a case without tampering with the verifier

## Rust direction

The reference tool is currently Python (zero runtime dependencies). A Rust-first companion binary is the preferred long-term distribution for agents: a single static binary, no interpreter required, stronger determinism guarantees, and easier embedding in constrained environments.

See the `rust/` directory for the emerging implementation.

## Usage in an agent

Place this skill where your agent harness discovers skills, or clone the repository and point the harness at `SKILL.md`. The skill is designed to work with any agent that can execute shell commands and manage isolated workspaces.

## License

MIT — same spirit as the original Bug2Eval project.
