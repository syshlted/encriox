# W1 — A Firefox fork, reaching the vault through encri-api

*2026-11-13 · accepted on the user's word ("So this is a Firefox fork"), picking scenario 1 of
`docs/feasibility-study.md`, and **flagged for veto on two points** (below). Nothing is built or measured on a phone
yet.*

#### What the user decided

- **Scenario 1**: a fork of Firefox for Android that reaches the vault through encri-api, as any companion does
  (encri D112, D145). Scenarios 2 and 3 are rejected (below).
- **The stage 1 spike first** ("Let's go with this spike first"), **and the stage 2 spike next** ("Let's get this done
  next"). This departs from the study's *Recommended path*, which settled the name, the package and the registry entry
  first. They now come after the spikes, and the name is being looked into.
- **The licence is MPL-2.0** (W2).

#### What was settled

1. **The two stages are the study's** (*Scenario 1*). Stage 1 builds Fenix against a GeckoView Mozilla has published,
   keeps every tab private, and writes downloads into the vault. Stage 2 makes Lockstore's key-encryption key come
   from the vault. Neither needs a change to encri or the contract (the study, *Encri's side*).
2. **Stage 1 tracks release.** The branch for stage 2, release or ESR, is picked before stage 2 starts, not before
   stage 1 (the study, *Recommended path*, item 1).
3. **The spikes run on the phones that hold encri's test vaults**, and on the S24 only with its owner's word
   (`AGENTS.md`, *Hazards*).
4. **Before carrying a Gecko patch, Mozilla is asked** whether GeckoView would take an embedder-supplied KEK, beside
   the OS-keystore KEK of bug 2055641. If it would, the fork carries prefs and no Gecko patch.

#### Flagged for veto

1. **The spikes use a throwaway package, `com.systemhalted.encri.browser.spike`**, installed only on test phones and
   never published. encri keys a grant on the package for good (encri D120, D145), so the package that ships is chosen
   with the name and starts with no grant from the spikes. The spikes' grants end with their test vaults.
2. **Gecko's web compatibility is taken as acceptable.** The study could not weigh it (`open-questions.md`), and it
   is the one argument it found for Chromium. Picking Firefox accepts it.

#### Measured

Nothing yet. The study's facts were checked against their sources on 2026-11-10. The two spikes are its
measurements.

#### Rejected

- **Scenario 2, Chromium through encri-api.** It can be built, but it needs a full Chromium build for any change and
  at-rest encryption the fork writes and rebases every two weeks, and it has no password store or extensions on phones
  (the study, *Scenario 2*).
- **Scenario 3, encri's own code inside Chromium.** It costs everything scenario 2 does, plus a JNI bridge, a second
  vault and LGPL obligations. It would also put keys in a process with network access, which encri D70 rules out.
- **Settling the name and the package before the spikes**, as the study recommended. The user put the spikes first.
  Nothing a spike builds ships, so with a throwaway package the order costs nothing.
