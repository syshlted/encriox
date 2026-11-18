# encri-firefox — amendments to earlier decisions

Each entry corrects a record. Its bold lead names the record possessively ("W1's …") so that
`scripts/docs/decision-index.py` copies it into that record's footer.

## Amendments

- **W1's rejection of scenario 2** (2026-11-14, W3): Chromium is not this repository's browser, but the user wants it
  as a second one, encrium, a project of its own. W1 picked between the scenarios for one browser, and that pick
  stands.
- **W3's owed trademark search** (2026-11-17): run for encriox as a knock-out search, through TMview and Trademarkia,
  with the web, the app stores and the domain registries (`docs/trademark-search.md`). Nothing found blocks the name;
  the nearest live marks are ENCRY and MacPaw's ENCRYPTO, and Encrox, one letter off, is expired. encrium's is still
  owed, by its own project.
- **W3's name for the Chromium browser** (2026-11-18, W11): encrium is given up, because `encrium.com` appears to be a
  French technical-training academy's, and the browser is encrimium, *encri* and *(chro)mium*, with the package
  `com.systemhalted.encrimium`. Its knock-out search found nothing that blocks it (`docs/trademark-search.md`).
- **W4's first item of what stage 1 has to do** (2026-11-15, W5): the length a response promises is not enough to tell
  a stopped copy from a finished one. A download with no length, paused or cancelled, was saved cut short into the
  vault. The writer also asks whether the download is still running, and a retry starts from the start.
- **W12's fix to QuotaManager** (2026-11-18, W13): a few lines, not one. Built as stage 2's patch 1, it recorded the
  test origin as encrypted on the G6 and the S9, which had recorded it as plaintext, and their site storage read back.
- **W12's ways to turn SQLite's switch on** (2026-11-18, W13): editing the published GeckoView's prefs is not one.
  Gecko's launch guard reads the switch before any pref is loaded, so a profile a pref encrypted is refused at its
  next start, as the S9's was. A build turns it on by its default.
