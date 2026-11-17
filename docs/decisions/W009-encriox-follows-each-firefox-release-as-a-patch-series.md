# W9 — encriox follows each Firefox release, as a series of patches, and says which it is built on

*2026-11-17 · accepted on the user's word (below), and **flagged for veto on three points** (below). Answers
`open-questions.md`'s *How the fork is carried*. Nothing of it is built yet: the spike's series is applied by hand.*

#### What the user decided

- "I want to keep encriox's repo to be able to 'automatically' pull from firefox upstream, when patches and updates
  come out, and build new releases against them."
- "We also need to maintain the firefox patch level info somewhere in the app's about info, as well as in the
  documentation."

#### What was settled

1. **encriox follows Firefox's release, every one**: each major, every two weeks since Fx155, and each dot release
   between them (the study, *What carrying the fork costs*). W7 put stage 2 on release for the same reason.
2. **One file names the release encriox is built on**: the Firefox tag and its commit, and, while encriox builds
   against a published GeckoView, that GeckoView's version, as the spike's `versions` does. Everything else that says
   which release reads it: the build, the About screen and the documentation.
3. **Following is automatic up to a tested build, and stops at the first thing it cannot do alone.** A script finds the
   newest release tag, and the GeckoView published for it, applies the series to the tag, builds, and runs the tests.
   A patch that no longer applies, a build that fails or a test that fails stops it with a message, and a person
   rebases the series. It runs by hand or on a schedule until the suite has a CI host, and as a job there after (encri
   B114 to B116). A tag with no GeckoView published for it, as 160.0.1 was on 2026-11-17 (W7), is waited for, unless
   encriox builds its own Gecko by then.
4. **Publishing is not automatic.** A release is signed on the key machine and its signed outputs never reach a commit
   (encri D141), and encri-api's registry entry decides whether a user's grant survives it (W8, encri D130).
5. **The About screen names the Firefox release encriox is built on**, beside what Fenix's already shows: its own
   commit, the GeckoView version and build ID, and application-services' version (`AboutFragment`). **The
   documentation names it too**: `README.md` the release the tip builds on, and each release's `CHANGELOG.md` entry the
   release it was built on.

#### Flagged for veto

1. **A series of patches on a release tag, not a fork of Firefox's history.** This repository holds the patches and
   the scripts, as the spike does (W4 to W6), and each release applies them again with `git am`. A fork of the history
   would instead merge each release tag into a branch of Firefox's whole tree. The series keeps what encriox changes
   readable as files, one change to a patch, keeps this repository small where Firefox's checkout is 15 GB, and makes
   the source W2 publishes the patches and the tag they apply to. How it fares against a two-week rebase is still
   unmeasured.
2. **The About screen's line reads *Firefox 160.0.1 (FIREFOX_160_0_1_RELEASE)***, with the tag's commit, in the same
   list as the GeckoView line.
3. **Dot releases are followed as majors are.** 160.0.1 carries Android fixes (W7).

#### Rejected

- **A fork of Firefox's history**: flag 1.
- **Publishing each build as it passes**: item 4.
- **Holding a release for the next ESR**: W7.
