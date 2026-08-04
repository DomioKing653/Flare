# AI Contribution Policy

vertex doesn't ban AI tools, but it does ban AI slop. This document draws the line.

## The rule

You can use an AI assistant to help you write a PR. You cannot use one to write a PR *for* you and skip understanding it. If you can't explain what your diff does and why, in your own words, it's not ready to submit - regardless of what wrote it.

## Not allowed

- **Unreviewed AI output.** Pasting in a model's answer to "add a stdlib function for X" without reading, testing, and understanding every line it touched.
- **Hallucinated APIs.** Code that calls functions, types, or crates that don't exist in this workspace because a model invented them. Check against the actual source, not against what "sounds right."
- **Mass-generated issues or PRs.** Batches of low-effort, near-duplicate, or speculative issues/PRs clearly produced by prompting a model repeatedly with no human judgment in between.
- **Fabricated bug reports.** "Bugs" that are actually a model misunderstanding rl-lang's syntax or semantics, submitted without first reproducing the behavior yourself.
- **AI-written comments/docs that don't match the code.** Generated doc entries, comments, or commit messages that describe what the code was *supposed* to do rather than what it actually does.

## Allowed, and expected to be normal

- Using AI to draft code you then read, test, and take responsibility for.
- Using AI to explain unfamiliar Rust patterns, debug a compiler error, or brainstorm approaches.
- Using AI to help write docs, then verifying the described behavior against the real implementation.
- Using AI for translation, wording, or cleanup of an explanation you already understand.

## What we ask in a PR

- Run `cargo test --all-features` and `cargo clippy -- -D warnings` yourself before opening the PR - don't rely on a model telling you it "should pass."
- Be able to answer follow-up questions about your own diff in review. "I'm not sure, an AI wrote that part" is a sign the PR isn't ready yet, not an acceptable answer.

## Why this exists

Maintainer time is the scarcest resource in a project like this. Reviewing code is more expensive than writing it. A PR or issue that looks plausible but is subtly wrong (a hallucinated function signature, a "bug" that's just a misunderstanding, a doc entry describing behavior the code doesn't have) costs more time to catch than it would have taken to write correctly by hand. This policy exists to keep that cost on the person submitting, not the person reviewing.

## Enforcement

Maintainers may close PRs/issues that show clear signs of unreviewed AI generation (nonsensical diffs, fabricated APIs, boilerplate that doesn't apply to this codebase) without extensive explanation. Repeat submissions of this kind may result in being asked to slow down or, in persistent cases, blocked from the repository.
