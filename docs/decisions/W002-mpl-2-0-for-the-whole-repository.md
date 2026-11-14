# W2 — MPL-2.0 for the whole repository

*2026-11-13 · accepted on the user's word ("let's just keep the license as MPL-2.0"), and **flagged for veto on one
point** (below). The sections cited are the text in `LICENSE`, fetched from mozilla.org/MPL/2.0 the same day.*

#### What the user decided

- **The licence is MPL-2.0**, Firefox's own: "So this is a Firefox fork. So let's just keep the license as MPL-2.0."

#### What was settled

1. **Every file here is MPL-2.0.** The files the fork modifies have to stay MPL-2.0 (`AGENTS.md`, hard constraint 4).
   The files it adds could be licensed otherwise, file by file, and are MPL-2.0 too. `LICENSE` holds the licence's
   text.
2. **The source is public wherever a build goes.** Whoever receives a build must be able to get its source (§3.2).
   This repository has no remote yet, so a public one comes before any build leaves the test phones.
3. **encri-client stays Apache-2.0.** It is encri-api's library and a dependency here, combined as a Larger Work
   (§3.3). encri stays closed (encri D116), and the browser reaches it only through encri-api.
4. **The licence grants no trademark rights** (§2.3), so the name is still bound by `AGENTS.md`'s hard constraint 3.

#### Flagged for veto

1. **A new source file carries Exhibit A's notice at its top**, as Mozilla's own files do: *"This Source Code Form is
   subject to the terms of the Mozilla Public License, v. 2.0. If a copy of the MPL was not distributed with this
   file, You can obtain one at https://mozilla.org/MPL/2.0/."*

#### Rejected

- **Apache-2.0 for the files the fork adds**, as Encri Files and Encri Images are licensed. MPL-2.0 permits it, but
  the repository would hold two licences, and every file would need checking for which one it is under.
