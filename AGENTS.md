# encri-firefox — agent guide

> **Level 0 of four**, loaded into every session: only what is true for every component, laid out as the suite's
> `docs/documentation.md` says (encri D127). The rest loads when you touch a component or is read when needed. Keep it
> under 200 lines. Dates: YYYY-MM-DD.

## What this is

**encriox, a browser for encri: a fork of Firefox for Android that reaches the vault through encri-api** (W1,
2026-11-13), licensed MPL-2.0 throughout (W2), and named in W3. Its sibling on Chromium is encrimium, a project of
its own, after encriox's spikes (W3, W11). `docs/feasibility-study.md` (2026-11-10) weighed it against the same on Chromium
and against encri's own code inside Chromium, and its *Scenario 1* is the design W1 adopted. **Nothing of encriox is
built yet.** The stage 1 spike, a throwaway, is built and measured on three phones (`scripts/spike/stage-1/`, W4 to W6);
the stage 2 spike's first round ran on its build, with prefs alone (`scripts/spike/stage-2/`, W12), and stage 2 now
builds its own Gecko, on which all three of Lockstore's switches work (W13) and the profile's keys are kept under a
secret from the vault's key (W14), which does not start without the vault (W15), and whose encryption of SQLite was
measured (W16), and which closes at a lock, letting Gecko write what it holds first (W20). encriox's first usable
build is planned and accepted (W17; `ROADMAP.md`, item 6).

## Relationship to the other repositories

Siblings, each with its own decision log, side by side in the suite's superproject: `../encri-core` (encri itself,
D1, …), `../encri-api` (the contract and its client, L1, …), `../encri-files` (F1, …), `../encri-images` (I1, …) and
`../encri-backup` (K1, …).

- **Decisions here are numbered W1, W2, …**, by the user's choice of 2026-11-11. The others' are cited as
  *encri D120*, *encri-api L11*, *encri-images I3*. B is encri's bug numbering, and S1 to S4 are severities.
- **About the browser, this repository wins.** About the contract, encri-api's log wins. About encri's internals,
  encri's log wins, and this one links to it rather than restating it.
- **The browser is an ordinary companion**, as Encri Images is (encri D145), and needs no change to encri or to the
  contract for either of its stages (W1; the study, *Encri's side*).

## How the project's knowledge is laid out

| Level | Where | Loads | Holds |
|---|---|---|---|
| 0 | `AGENTS.md` | every session | constraints, hazards, conventions, this map |
| 1 | `.claude/rules/<component>.md` | when a file matching its `paths:` is read or edited | one component's rules, traps and measurements (below) |
| 2 | `docs/decisions/INDEX.md` | on demand | one line per decision, generated |
| 3 | `docs/decisions/Wnnn-<slug>.md` | on demand | the records, W1 and on |

`docs/decisions/open-questions.md` is what the records left open, and `amendments.md` beside it their corrections.
`docs/feasibility-study.md` is a study, not a decision: what each scenario costs, with a source for every browser fact
and *unverified* on anything not checked against one. `docs/trademark-search.md` is the knock-out search for the
name (2026-11-17), kept the same way. Where either and a record disagree, the record wins. `ROADMAP.md` is
what is next.

## Components (level 1)

| Brief | Covers |
|---|---|
| `spike.md` | the spikes, stages 1 and 2, `scripts/spike/**`, and `LICENSE` until encriox's own build has a brief |

## Hard constraints

These are external facts, each sourced in the study. They are not negotiable.

1. **A browser's live stores cannot live in the vault.** Cookies, history, site storage and the HTTP cache are written
   in place, page by page, many times a second. An encri container is never changed in place (encri D26), and a write
   through the contract commits only on a clean close and is abandoned by a lock (encri-api L11, encri D120). So in
   every scenario the live stores are memory-only, or encrypted inside the browser with a key that only the unlocked
   vault can produce. Downloads, extension packages and session snapshots are written once, whole, and can.
2. **Plaintext handed out is plaintext gone** (encri's hard constraint 2). The browser sees every byte of every page,
   and a key it holds sits in its memory while it runs.
3. **The product's name cannot contain "Firefox"**, nor follow the "[Something]-fox" pattern Mozilla's policy names,
   nor contain "Chrome", "Chromium" or "Blink" on the other engine: each vendor's trademark policy requires the fork's
   own branding. The app is encriox (W3); `encri-firefox` is a repository name, not an app name.
4. **Files a Firefox fork modifies stay MPL-2.0**, and their source must be published.
5. **A grant is keyed on the package for good** (encri D120, D145), and lapses with every update encri's registry has
   not reviewed (encri D120, D130). At Firefox's two-week cadence an unlisted browser would ask the user again twice a
   month.

## Hazards

- **Never run encri's `:app:connectedDebugAndroidTest` on a phone that holds a vault.** It destroys the vault (encri
  D39). The study's spikes run on the phones that hold encri's test vaults.
- **The S24 is a phone in daily use.** Run nothing on it without its owner's word, and capture nothing from its screen
  but encri's and the app under test's (encri B139; encri-images' `docs/dev-environment.md`).
- **The project's earlier name is never written**: not in code, docs, commits or any repository (encri D124).

## What this project keeps finding

encri's lessons, which hold here before this project has any of its own.

- **Measure before recording a conclusion.** Record what was not measured as unmeasured. The study marks its estimates
  *unverified* for this reason.
- **A test that has never failed has not been tested.** Revert the fix and watch the new test fail before trusting it.

## Repository conventions

- **A git repository since 2026-11-11.** It follows its siblings' conventions. Its public remote is
  `git@syshlt-github:syshlted/encriox.git` (2026-11-20), as `origin`, before any build leaves the test phones (W2);
  nothing of this history is pushed to it yet.
- **Every file is MPL-2.0** (W2), and a new source file begins with Exhibit A's notice, as Mozilla's own files do.
- **Default branch is `master`**, by SystemHalted organization policy. Never rename it to `main`.
- **Commits are signed** (key `E483D747728C1C9EA213FB29615692825276E947`, repo-local `commit.gpgsign` and
  `tag.gpgsign`) **and Conventional**, with a subject of at most 72 characters.
- **Every commit builds and passes its own tests, not just the branch tip.**

## Recording what is decided

1. **A settled design question becomes a decision record**, not only an answer in chat. It goes in
   `docs/decisions/Wnnn-<slug>.md` under the next free number, beginning `# Wnnn — <title>` then
   `*YYYY-MM-DD · accepted…*`. It says what was rejected and what was measured, and is *flagged for veto* where the
   user has not confirmed it.
2. **Regenerate the index** with `python3 scripts/docs/decision-index.py .`, never by hand. The scripts are
   encri-backup's, reading W numbers. Check that the briefs cover the code with
   `python3 scripts/docs/brief-coverage.py .`.
3. **Amend earlier records in place**, with strike-through and "amended by Wnnn", and add a line to
   `docs/decisions/amendments.md`. What is next goes in `ROADMAP.md`; what landed goes in `CHANGELOG.md`.

**When they disagree, the decision records win**, and the briefs or `ROADMAP.md` are what get fixed.
