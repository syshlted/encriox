# encri-firefox — changelog

What happened, newest first. Why is in `docs/decisions/`, and what is next is in `ROADMAP.md`.

## Unreleased

### The stage 1 spike, on the G6 (2026-11-14)

Fenix at Firefox 160.0 builds with Gradle against the GeckoView Mozilla published, with six patches and encri-api from
source (`scripts/spike/stage-1/`). On the G6, a private session left no page's marker on disk, where a normal one left
it in ten files, and a download went into the vault at about 35 MiB/s, a third of Fenix's own writer (W4). Private
thumbnails, a paused download and Gecko prefs need work in stage 1. The S9, the A20 and the S24 are owed.

### encriox and encrium (2026-11-14)

The user wants a second browser, on Chromium, so the two needed names that tell them apart. This one is encriox and
the Chromium one encrium (W3), each keeping *encri* whole and following the forks' habit of animal names for Firefox
and element names for Chromium. W1's rejection of Chromium is amended: it is not this repository's browser, but it is
the second one, a project of its own.

### A Firefox fork, licensed MPL-2.0 (2026-11-13)

The user picked the study's scenario 1, a Firefox fork reaching the vault through encri-api (W1), and MPL-2.0 for
every file (W2). `LICENSE` holds the licence's text. The stage 1 spike comes first and the stage 2 spike next, ahead
of the name and the package, which the study had put first. The study's open questions moved to
`docs/decisions/open-questions.md`, and `scripts/docs/` came from encri-backup, reading W numbers, to generate the
decision index.

### The documentation takes the suite's common shape (2026-11-11)

The directory moved into the suite's superproject, whose `docs/documentation.md` sets one layout for every
repository. `AGENTS.md`, empty until now, is the level-0 guide, holding only what the study sourced. `ROADMAP.md`
follows the study's recommended path, after the pick it waits on. `README.md` and this file are new. The directory
became a git repository, on `master` with its siblings' signing settings and no remote, and W is the decision prefix,
both at the user's word.

### A browser for encri, studied (2026-11-10)

`docs/feasibility-study.md` weighs three scenarios and finds scenario 1, a Firefox fork reaching the vault through
encri-api, the most feasible: its first stage builds with Gradle against Mozilla's GeckoView, and its second rides on
the at-rest encryption Mozilla is writing. Nothing is decided until the user picks.
