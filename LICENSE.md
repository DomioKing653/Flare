# License

vertex is dual-licensed under your choice of either:

- [MIT License](LICENSE-MIT.md)
- [Apache License, Version 2.0](LICENSE-APACHE.md)

You may use, modify, and distribute this project under the terms of **either** license - you don't need to comply with both, just pick whichever fits your situation.

## Why dual-license?

This is the same approach most of the Rust ecosystem uses (`serde`, `tokio`, `rand`, and the Rust compiler itself all do this), for a practical reason: MIT and Apache-2.0 are both permissive, but they're not identical, and different downstream users care about different things.

| | MIT | Apache 2.0 |
|---|---|---|
| **Length / complexity** | Short, simple | Longer, more detailed |
| **Patent grant** | None | Explicit - contributors grant you a license to any patents they hold that read on their contribution |
| **Patent retaliation clause** | None | Yes - if you sue anyone over patent infringement related to the code, your license under Apache-2.0 terminates |
| **Attribution requirement** | Must keep the copyright notice and license text | Must keep copyright notice, license text, and a NOTICE file if one exists; must state significant changes you made |
| **Compatibility** | Compatible with almost everything, including GPL | Compatible with GPLv3, but **not** GPLv2 |

**In short:**
- If you want the simplest possible license with no extra machinery, pick **MIT**.
- If you (or your company) want an explicit patent grant and protection from patent litigation, pick **Apache-2.0**.
- If you're bundling rl-lang into a GPLv2 project, **MIT** is the one that's compatible - Apache-2.0 is not.

Whichever you pick, you're covered - you don't need permission from us, and you don't need to ask which one is "correct" for your use case. That's the entire point of offering both.

## For contributors

By submitting a contribution to this repository, you agree that your contribution is licensed under both of the above licenses, matching the rest of the codebase. You don't need to add your own copyright notice per-file - the project's license files at the root cover the whole repository.
