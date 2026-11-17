# W10 — encriox's key is a hidden document in the vault, for now

*2026-11-17 · accepted on the user's word ("Let's use a hidden file in the vault for now"), and **flagged for veto on
two points** (below). Answers, for now, `open-questions.md`'s *How the browser's own key is kept*, and keeps W6's
flag 1 with the name hidden. Nothing of encriox is built; the spike keeps `encriox.key` (W6).*

#### What the user decided

- **The key is a document in the vault, under a hidden name**: the first of `open-questions.md`'s three ways, as the
  spike's is.
- **App-level security controls come later**: "I think we will want to add app-level security controls in the future.
  Users will definitely benefit from that." They are an item for later (`ROADMAP.md`).

#### What was settled

1. **The document is `.encriox.key`**: 32 random bytes at the top of the folder the vault allows encriox, read after
   each unlock and wiped from memory at a lock. The spike's rules hold (W6; its brief): never written over, and
   refused if it is the wrong length, there are two of the name, or it does not read back.
2. **A hidden name hides it only from apps that hide such names, and from no app allowed its folder.** It keeps the key
   out of the way of someone looking through the folder, and so less likely to be deleted by mistake, which wipes
   encriox's logins and cards (W6, flag 2). It does not keep it from another app the user has allowed that folder or
   the whole vault: that app can read it (W6, flag 1).
3. **The other two ways stay open**: the document wrapped by a Keystore key of encriox's own, and the key wrapped by
   encri for its package, which changes the contract. They are weighed with the app-level controls.
4. **Nothing in encri or the contract changes.**

#### Checked (2026-11-17, in the source)

- **encri takes the name.** Its catalog refuses a name only if it is blank or longer than its limit in UTF-8 bytes
  (`Catalog.requireName`), so a leading dot is allowed.
- **Nothing in the suite hides it.** Encri Files and Encri Images filter no name that begins with a dot, so each lists
  `.encriox.key` like any other document. Whether the system's file picker hides one that encri's provider lists is
  *unverified*.

#### Flagged for veto

1. **The name, `.encriox.key`.**
2. **Hiding it in Encri Files is Encri Files' to decide**, and is not done. Until it is, the name is hidden in no app
   of the suite (`open-questions.md`).

#### Rejected

- **A Keystore-wrapped or an encri-wrapped key, for now**: the user's "for now".
- **Renaming the spike's key.** The S9's and the A20's test vaults hold `encriox.key`, and the spike's logins and
  cards are sealed with it; a new name makes a new key and wipes them (W6, flag 2), and would measure nothing new.
