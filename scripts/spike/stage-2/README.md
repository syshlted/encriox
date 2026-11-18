# The stage 2 spike

Stage 1's build (`../stage-1/`) with Lockstore's switches on, Gecko's at-rest encryption, which is in the GeckoView
Mozilla published for 160 (W5, W7; the study's *Stage 2*). Nothing new is built: `geckoview-config.yaml` turns the
switches on through the debug config a debuggable GeckoView reads. What it found is in W12.

## On a phone

The phones and the rules are stage 1's (`../stage-1/README.md`): the ones that hold encri's test vaults, and the S24
only with its owner's word. `ADB` works as it does there. No vault needs to be open.

- `check-lockstore <serial>`: gives the spike a fresh Gecko profile, runs stage 1's `check-plaintext <serial> normal`
  with this directory's config, prints the header of every SQLite file in the profile and each origin's flags in
  QuotaManager, then starts the app again and reads back, store by store, what the first session wrote.
- `fresh-profile <serial>`: deletes the spike's Gecko profile and its cache, and nothing else. A profile's SQLite
  switch is set when it is made, so run it before going back to stage 1's config too.
