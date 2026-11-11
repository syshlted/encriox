# encri-firefox

A browser for an encri vault, at the study stage. Its downloads, extension packages and session snapshots would land in
the vault, and its live stores, such as cookies, history and the cache, would stay off disk or be encrypted with a key
only the unlocked vault can give it. In the scenario the study recommends, it reaches the vault through the
[encri-api](https://github.com/syshlted/encri-api) client library, as Encri Images does.

**Pre-release. Nothing is built, and nothing is decided.** [`docs/feasibility-study.md`](docs/feasibility-study.md)
weighs a Firefox fork, a Chromium fork, and encri's own code inside Chromium. See [`ROADMAP.md`](ROADMAP.md) for what
comes next.

The app will need a name of its own: neither "Firefox" nor "Chromium" may be in it.

## Licence

Not chosen yet. In a Firefox fork the files it modifies stay MPL-2.0, and encri-client is Apache-2.0.
