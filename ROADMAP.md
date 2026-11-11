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

1. **The user picks a scenario** from `docs/feasibility-study.md`, whose *Verdict* finds scenario 1, Firefox through
   encri-api, the most feasible. The pick is W1. Nothing below starts before it.
2. ~~**This directory becomes a git repository**, with its siblings' conventions (`AGENTS.md`), before W1 is committed.~~
   **Done 2026-11-11**, with W as the decision prefix.

If the pick is scenario 1, the study's *Recommended path* is the order. Its reasons and sources are there, and are not
repeated here:

3. **Settle what is permanent first**: the product's name, which cannot contain "Firefox"; its package, on which a
   grant is keyed for good (encri D120, D145); and its registry entry in encri-api.
4. **Spike stage 1 on the phones**: Fenix against a published GeckoView, every tab private, downloads through
   `openWrite`. Measure download throughput through the pipe against MediaStore on the S9, the A20, the G6 and the
   S24, and confirm that nothing of Gecko's reaches `filesDir` or `cacheDir` in plaintext.
5. **Ask Mozilla before carrying a patch**: an embedder-supplied KEK in GeckoView, beside the OS-keystore KEK of bug
   2055641.
6. **Spike stage 2 before writing any of it**: GeckoView with Lockstore's prefs on, on each phone. Pick the branch,
   release or ESR, before stage 2 and not before stage 1.

The study's *Open questions* (history in stage 1, application-services, web compatibility, what a lock does to open
tabs) are answered by W1 or by the spikes, and stay in the study until a record answers them.
