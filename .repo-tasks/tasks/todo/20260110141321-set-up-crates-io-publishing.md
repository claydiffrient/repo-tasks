---
ID: '20260110141321'
Title: Set up crates.io publishing
Priority: High
Tags:
- devops
- cd
- crates-io
---

Add crates.io publishing to release workflow. Get API token from crates.io, add as GitHub secret (CARGO_TOKEN), and configure workflow to run 'cargo publish'. Should trigger on version tags after successful binary builds.