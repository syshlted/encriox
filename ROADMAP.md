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
   reaches `filesDir` or `cacheDir` in plaintext. The S24 only with its owner's word. ~~The G6~~ **Done on the G6
   2026-11-14 (W4)**: built (`scripts/spike/stage-1/`), no page in plaintext after a private session, downloads into the
   vault at about 35 MiB/s. ~~Owed: the S9, the A20 and the S24, with `check-plaintext` and `measure`.~~ **Done on the
   S9 and the A20 2026-11-15 (W5)**: no page in plaintext there either; downloads into the vault at about 57–65 MiB/s
   on the S9 and 28–35 MiB/s on the A20, a third to a half of MediaStore's speed; and a download stopped part way,
   which the vault had kept cut short, now leaves nothing. **Then W6, 2026-11-16**: a private tab's thumbnail is kept
   in memory, so a private session leaves nothing of a page on disk on all three; and the logins' and the cards' keys
   are sealed with a key of the browser's own in the vault, `encriox.key`, measured on the S9 and the A20, with the
   locked path on the G6. Owed:
   - **the S24**, with `check-plaintext`, `measure`, `check-stopped` and `check-keys`, on its owner's word;
   - **patch 0010's locked path, on the G6**, once its owner has unlocked its vault, which the spike locked and the
     passphrase on file does not open (W5): open the gate, lock with `--ez lock true`, and open it again, which must
     ask to unlock.
4. **Spike stage 2 before writing any of it**: GeckoView with Lockstore's prefs on, on each phone. Pick the branch,
   release or ESR, before it. It builds GeckoView, which needs at least 30 GB of disk by Mozilla's guide; the machine
   had 63 GB free on 2026-11-13, and 42 GB on 2026-11-15. **Lockstore is in the GeckoView Mozilla published** (W5,
   read in its `libxul`), so on release its first questions could be asked of stage 1's build with prefs alone; only
   a key-encryption key from the vault needs a Gecko build. Unmeasured.
5. **Ask Mozilla before carrying a patch**: an embedder-supplied KEK in GeckoView, beside the OS-keystore KEK of bug
   2055641. Who asks, and where, is open. Theirs to know too: Android Components' logins and autofill stores wait on
   themselves for good once their key is lost, which the spike carries a fix for (W6, patch 0018).

## Before the first release

- **Settle what is permanent**: ~~the product's name, which cannot contain "Firefox";~~ **named encriox
  2026-11-14 (W3)**; its package, on which a grant is keyed for good (encri D120, D145); and its registry entry in
  encri-api. Moved after the spikes by W1.
- **A trademark search for encriox**, in the US, EU, international and Japanese registers (W3).
- **A public remote** for this repository, before any build leaves the test phones (W2).
- **How encriox keeps its own key**: a document in the vault, as the spike does (W6, flag 1), the same wrapped by a
  Keystore key of its own, or wrapped by encri for its package, which is a change to the contract
  (`open-questions.md`).

## Bugs

- **S3 — A lock leaves the logins' and the cards' keys in memory** until the process ends: the stores keep the keys
  they were given, and the spike wipes only its own at a lock (W6, flag 3). What a lock should do is
  `open-questions.md`'s *Lock behaviour*.
- **S4 — A download into the vault that stopped shows as failed**, a cancelled one too, where Fenix's own writer shows
  it paused or cancelled (W5, flag 2). *Try again* starts it from the start.

What the records left open, the study's *Open questions* included, is in `docs/decisions/open-questions.md`.
