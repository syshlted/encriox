# encri-firefox — roadmap

What is next, and in what order. Why is in `docs/decisions/`, and what happened is in `CHANGELOG.md`. When they
disagree, the decision records win.

## Severity

| | Meaning |
|---|---|
| **S1** | Data loss, a confidentiality breach, or a vault that will not open |
| **S2** | A user-visible wrong answer or failure, with no workaround |
| **S3** | Wrong under a narrow race, or an honest limit that needs disclosing |
| **S4** | Waste, cosmetics, or documentation that contradicts the code |

Severity is about the consequence if it happens, not the odds of it happening.

## Now

1. ~~**The user picks a scenario** from `docs/feasibility-study.md`, whose *Verdict* finds scenario 1, Firefox through
   encri-api, the most feasible. The pick is W1. Nothing below starts before it.~~ **Done 2026-11-13 (W1)**: scenario
   1, with MPL-2.0 for every file (W2).
2. ~~**This directory becomes a git repository**, with its siblings' conventions (`AGENTS.md`), before W1 is committed.~~
   **Done 2026-11-11**, with W as the decision prefix.

The study's *Recommended path* is the order, except that the spikes now come before the name (W1). Its reasons and
sources are there, and are not repeated here:

3. **Spike stage 1 on the phones**, under the throwaway package `com.systemhalted.encri.browser.spike` (W1, flag 1).
   Fenix against a published GeckoView, every tab private, downloads through `openWrite`. Measure download throughput
   through the pipe against MediaStore on the S9, the A20, the G6 and the S24, and confirm that nothing of Gecko's
   reaches `filesDir` or `cacheDir` in plaintext. The S24 only with its owner's word.
4. **Spike stage 2 before writing any of it**: GeckoView with Lockstore's prefs on, on each phone. Pick the branch,
   release or ESR, before it. It builds GeckoView, which needs at least 30 GB of disk by Mozilla's guide; the machine
   had 63 GB free on 2026-11-13.
5. **Ask Mozilla before carrying a patch**: an embedder-supplied KEK in GeckoView, beside the OS-keystore KEK of bug
   2055641. Who asks, and where, is open.

## Before the first release

- **Settle what is permanent**: the product's name, which cannot contain "Firefox"; its package, on which a grant is
  keyed for good (encri D120, D145); and its registry entry in encri-api. Moved after the spikes by W1.
- **A public remote** for this repository, before any build leaves the test phones (W2).

What the records left open, the study's *Open questions* included, is in `docs/decisions/open-questions.md`.
