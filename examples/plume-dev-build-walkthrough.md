# Plume Dev Build Plane Walkthrough

This note is the quickest way to read the extra review model in `plume-dev-build-plane`.

| Case | Focus | Score | Lane |
| --- | --- | ---: | --- |
| baseline | change width | 184 | ship |
| stress | diagnostic quality | 187 | ship |
| edge | review cost | 207 | ship |
| recovery | safe rewrite | 172 | ship |
| stale | change width | 200 | ship |

Start with `edge` and `recovery`. They create the widest contrast in this repository's fixture set, which makes them better review anchors than the middle cases.

`edge` is the optimistic case; use it to make sure the scoring path still rewards strong signal.
