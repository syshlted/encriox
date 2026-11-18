# encri-firefox

**encriox** is a browser for an encri vault: a fork of Firefox for Android. Its downloads, extension packages and session snapshots
would land in the vault, and its live stores, such as cookies, history and the cache, would stay off disk or be
encrypted with a key only the unlocked vault can give it. It reaches the vault through the
[encri-api](https://github.com/syshlted/encri-api) client library, as Encri Images does.

**Pre-release. Nothing of encriox is built yet.** What is built is the stage 1 spike, a throwaway, measured on three
phones ([`scripts/spike/stage-1/`](scripts/spike/stage-1/README.md)). The design is the Firefox fork that
[`docs/feasibility-study.md`](docs/feasibility-study.md) found the most feasible of three, chosen in
[W1](docs/decisions/W001-a-firefox-fork-reaching-the-vault-through-encri-api.md). See [`ROADMAP.md`](ROADMAP.md) for
what comes next.

Its sibling on Chromium is encrimium, a project of its own, still to come.

## Licence

Licensed under the [Mozilla Public License 2.0](LICENSE), every file
([W2](docs/decisions/W002-mpl-2-0-for-the-whole-repository.md)). encri-client, the library it reaches the vault
through, is Apache-2.0.
