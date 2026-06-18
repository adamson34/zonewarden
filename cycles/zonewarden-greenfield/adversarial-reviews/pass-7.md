# Adversarial Review — Pass 7
**Scope:** specs, `--scope=full` (full domain-spec corpus + product-brief)
**Mode:** Spec Review (fresh context)

## KEY QUESTION
**CRIT: 0, HIGH: 2**

This pass contains 0 CRIT and 2 HIGH findings. The two HIGH findings are internal contradictions (one a `usize`/`u64` core-type contradiction in the product brief, one a `RSTO`/`RSTR` severity-bucket contradiction between DI-017 and entities.md). Neither is a security bypass, but both would cause wrong implementation of correctness/security-relevant behavior. All five Part-A fixes are VERIFIED.

---

## Part A — Fix Verification

### Fix 1 — Directed-broadcast override restricted to prefix ≤ /30, /31 and /32 excluded
**VERIFIED.**
- `invariants.md` DI-016 step 2 reads: *"if the matched zone's IPv4 prefix length is ≤ 30 AND `dst` equals that zone's directed-broadcast (all-ones host) address, override the `Explicit` MatchKind to `MulticastBroadcast`"* and explicitly *"The /31 and /32 cases are excluded — they have no usable broadcast address, so a single-host (/32) or point-to-point (/31) member is never silently exempted (closes the silent-allow vector R-002)."* It also pins IPv4-only: *"Step 2 is IPv4-only — IPv6 has no broadcast, so a v6 dst is only exempt via the step-1 `ff00::/8` test."* (invariants.md:37)
- `edge-cases.md` DEC-030 carries the same guard: *"with prefix ≤ /30 … (IPv4 only; excluded for /31 and /32 … IPv6 never takes this branch)"* (edge-cases.md:48).
- No other section describes the override without the guard. `entities.md:27` (ResolvedEndpoint) references the override but defers to DI-016 by ID rather than restating the threshold, so it cannot drift. Grep for `/30|/31|/32|directed-broadcast` confirms the only normative statements of the override are DI-016 and DEC-030, both guarded.
- Consistency: DEC-023 ("a zone's host /32 equals another zone's network or broadcast address → resolves to the host's zone") is consistent with the /32 exclusion — a /32 host is never reinterpreted as a broadcast. No contradiction introduced.

### Fix 2 — Concrete max_flows ingest cap enforced at ST-3
**VERIFIED (with a MED caveat, see Part B).** FM-008 now reads: *"A concrete `max_flows` ingest cap — a configured constant (default pinned in PRD, e.g. `1_000_000_000`, far below `u64::MAX`) enforced at ST-3 ingest"* (failure-modes.md:30). The ST-3 anchor is present (the FM-008 Subsystem column is "ST-7 (aggregate)" for the overflow detection, but the cap text explicitly says "enforced at ST-3 ingest"). No literal "TBD" or bare "a cap" remains. The number `1_000_000_000` is concrete. Caveat: it is phrased as "e.g." with the authoritative value "pinned in PRD" — so the *domain spec itself* does not bind the constant. This is acceptable at L2 but is flagged below as MED (cap behavior at-exactly-vs-over is undefined).

### Fix 3 — Overflow aborts with exit code 2
**VERIFIED.** FM-008: *"on overflow abort with exit code 2 (the usage/internal-error code per ST-8) + diagnostic (never saturate, never silent wrap)."* (failure-modes.md:30). Consistent with events.md Exit semantics: *"`2` policy/usage/I-O error"* (events.md:42) and with FM-001/FM-003/FM-006 which all use exit code 2.

### Fix 4 — CAP-006/009/010 cite DI-020, DI-015, DI-018, DI-019; cited DIs exist
**VERIFIED.**
- CAP-006 cites DI-014, DI-007, **DI-020** ✓ (capabilities.md:27)
- CAP-009 cites **DI-015**, DI-009, **DI-018** ✓ (capabilities.md:30)
- CAP-010 cites DI-009, **DI-019** ✓ (capabilities.md:31)
- All four target DIs (DI-015, DI-018, DI-019, DI-020) exist as real rows in invariants.md (verified DI-001..DI-020 contiguous, exactly 20 rows). Each of the four cited DIs maps semantically to the citing capability (DI-020 PortSet ↔ CAP-006 matching; DI-015 accounting + DI-018 digest ↔ CAP-009 aggregate; DI-019 warning model ↔ CAP-010 render). Anchoring is semantically correct.

### Fix 5 — ConnState::Other defaults to Established
**VERIFIED.** entities.md Severity value-object: *"Flows with no `conn_state`, or `ConnState::Other(_)`, default to `Established` (conservative — never under-reports a breach)."* (entities.md:42). Consistent with DI-017 (*"`Established` otherwise (incl. when `conn_state` is absent — conservative)"*, invariants.md:38) and with the ubiquitous-language Severity entry (line 45). The ConnState enum entry (entities.md:41) and ST-6 (events.md:27) do not contradict it. Consistent.

---

## Part B — New Findings

### HIGH — Product brief contradicts entities.md on the canonical counter type (`usize` vs `u64`)
**Severity:** HIGH
**Location:** `product-brief.md:131` (Constraints) vs `entities.md:30` (ConformanceResult invariants)
**Problem:** The brief says *"64-bit targets only (tally counters assume 64-bit `usize`)."* entities.md says all tallies and `flow_index` are *"`u64` (canonical, not platform `usize`)"* and FM-008 reasons about `u64::MAX`. These are mutually exclusive design statements: one pins the canonical counter type to platform `usize` (and is the *reason* given for the 64-bit-only constraint), the other explicitly rejects `usize` in favor of `u64`.
**Why it matters:** The whole FM-008 overflow story (`checked_add`, `u64::MAX` headroom, `max_flows` cap "far below `u64::MAX`") and DI-009 byte-stable output are written against `u64`. If an implementer follows the brief and uses `usize`, then (a) the "64-bit only" constraint becomes load-bearing for correctness rather than incidental, (b) determinism across 32-bit targets is silently broken, and (c) the FM-008 reasoning about `u64::MAX` no longer matches the actual type. A core data-type contradiction at the root spec.
**Suggested fix:** Change product-brief.md:131 to *"tally counters and `flow_index` are canonical `u64` (not platform `usize`); 64-bit targets are the only tested platform but counter width does not depend on it."* This makes the brief consistent with entities.md/FM-008 and removes the false causal link between the platform constraint and the counter type.

### HIGH — DI-017 and entities.md disagree on the `RSTO`/`RSTR` severity bucket (reset-after-established)
**Severity:** HIGH
**Location:** `invariants.md:38` (DI-017) vs `entities.md:41` (ConnState enum)
**Problem:** entities.md classifies `RSTO`/`RSTR` as `Established` (*"connection was established then reset"*) and the BACKLOG note says *"the examples here fix the previously-wrong `RSTO` bucket"* — i.e. someone deliberately moved `RSTO` into `Established`. But DI-017, the *normative* invariant, only enumerates `S0`/`REJ` as `Attempted` and says *"`Established` otherwise."* That "otherwise" rule happens to land `RSTO`/`RSTR` in `Established`, so the buckets coincide *by accident of the default* — but DI-017 never states the rule that produces the result. Worse: Zeek `RSTOS0` and `RSTRH` are reset states where the originator/responder reset **without** completing the handshake (analogous to `S0`/`SH` = `Attempted`), yet under DI-017's "anything not `S0`/`REJ` is `Established`" rule they would be graded `Established`. entities.md lists `SH` as `Attempted` but DI-017 does not, so `SH` is graded `Established` by the invariant and `Attempted` by entities.md — a direct contradiction for at least one concrete Zeek state.
**Why it matters:** Severity grading is the security-evidence axis (`Attempted` = "the control held"; `Established` = "confirmed breach"). A flow that was actually only an attempted/rejected connection being graded `Established` over-reports a breach; the reverse under-reports. DI-017 and entities.md will be implemented by different people from different files and produce different severities for `SH` (and the `RSTOS0`/`RSTRH` family). The "BACKLOG: 13-state mapping pinned at PRD" note papers over the fact that the *domain invariant itself* gives a different answer than the entity examples for states that already appear in entities.md.
**Suggested fix:** Make DI-017 enumerate the `Attempted` set to match entities.md exactly (`S0`, `REJ`, `SH` → `Attempted`; `SF`, `RSTO`, `RSTR` → `Established`; all else default `Established`), or have DI-017 state the *rule* ("`Attempted` iff the handshake never completed") and have entities.md derive its examples from that rule. The two files must give the same verdict for every state named in either.

### MED — `max_flows` cap behavior is undefined at exactly the cap vs over the cap, and the abort path's exit code conflicts with the violations exit code
**Severity:** MED
**Location:** `failure-modes.md:30` (FM-008) + `events.md:40-43` (Exit semantics)
**Problem:** FM-008 says the cap "bounds total flows so the abort is an unreachable defense-in-depth backstop" but never states (a) whether hitting exactly `max_flows` is allowed or rejected (boundary: is the cap inclusive `>=` or exclusive `>`?), or (b) what the *response* is when the cap is reached during ST-3 ingest — it only describes the *overflow `checked_add`* response (abort exit 2). The cap is supposed to fire *before* overflow, so it needs its own defined detection+response. As written, the cap has detection ("enforced at ST-3") but no defined response distinct from the overflow abort. Additionally, if the cap-reached response is "abort with exit code 2," that collides with a legitimately large conformant-or-violating run: a run that genuinely has 1,000,000,001 flows would exit 2 (interpreted by CI as "policy/usage error") rather than 0/1 (conformant/violations).
**Why it matters:** This is exactly the "behavior at exactly the cap vs over the cap" gap flagged in the task. An implementer cannot know whether `max_flows` flows is OK or already an error, nor whether reaching the cap is a usage error (exit 2), a graceful truncation-with-warning, or a hard abort. The exit-code overload means a large-but-valid input is indistinguishable from a malformed-policy error to a CI consumer.
**Suggested fix:** In FM-008 state explicitly: cap is inclusive (`total_flows > max_flows` triggers; exactly `max_flows` is allowed), the cap-reached response is "abort before classification with a dedicated diagnostic," and either reuse exit 2 *with a distinct, greppable message* or introduce a distinct exit code for "input exceeds configured `max_flows`" so CI can tell it apart from a config error.

### MED — FM-008 Subsystem column says ST-7 but the mandated enforcement is ST-3; the cap's home stage is ambiguous
**Severity:** MED
**Location:** `failure-modes.md:30` (FM-008 Subsystem column = "ST-7 (aggregate)")
**Problem:** FM-008's Subsystem field is "ST-7 (aggregate)" (where tally overflow would occur) but the new cap text says enforcement is at ST-3 ingest, and ST-3's own row (events.md:24) and FM-007 (also ST-3) describe ingest. The single FM row now spans two stages (ST-3 cap + ST-7 overflow) but is filed under only ST-7. A reader scanning failure modes by stage will not find the `max_flows` cap when looking at ST-3.
**Why it matters:** Traceability by subsystem is how architect/test-writer consumers slice this table (per L2-INDEX Document Map). A cap enforced at ST-3 that is filed under ST-7 is a discoverability gap and could lead to the ST-3 enforcement being missed in story decomposition.
**Suggested fix:** Either split FM-008 into FM-008 (ST-7 overflow abort) and a new FM (ST-3 `max_flows` cap), or change the Subsystem cell to "ST-3 ingest / ST-7 aggregate" so both enforcement points are indexed.

### MED — DI-016 step-1 broadcast test omits subnet-directed broadcast for v4 but the family-wide test only catches `255.255.255.255`; reserved/odd cases (e.g. `0.0.0.0`, class-D edge) unspecified
**Severity:** MED
**Location:** `invariants.md:37` (DI-016 step 1)
**Problem:** Step 1 ("family-wide, needs no zone") catches `224.0.0.0/4`, `255.255.255.255`, and `ff00::/8`. Step 2 catches directed broadcast only for the *matched zone* at ≤ /30. But a destination that is the directed broadcast of a subnet that is **not** declared as a zone (resolves to EXTERNAL) is explicitly stated to be "cannot be a directed broadcast" — correct. However, the spec does not address `dst == 0.0.0.0` (historical "this host"/all-zeros broadcast), nor multicast-to-EXTERNAL interplay when a *declared* zone CIDR includes part of `224.0.0.0/4` (a zone could legally declare `224.1.0.0/16` as a member; step 1 would exempt it before zone resolution, which may or may not be intended). The latter means a flow to a multicast address that a user deliberately put inside a managed zone is silently `MulticastExempt`, never IDMZ-checked.
**Why it matters:** An asset owner who models a multicast group as a zone member (plausible for OT I/O groups) would find those flows silently exempted from all conduit and IDMZ checking by step-1 precedence — a potential silent-allow that the user did not intend. `0.0.0.0` as a dst is undefined behavior.
**Suggested fix:** State explicitly that step-1 family-wide multicast/broadcast precedence applies *even if a declared zone CIDR overlaps `224.0.0.0/4`/`ff00::/8`* (and justify it, or reject such zone declarations at load like the /0 rule in DEC-029). Add `0.0.0.0` (and `::`) handling to DI-003/DI-016 (resolve to EXTERNAL or reject).

### MED — `external_endpoints` and `skipped` are excluded from the DI-015 identity, but there is no invariant guaranteeing `distinct_violating_flows ≤ total_flows` or relating it to the component tallies
**Severity:** MED
**Location:** `invariants.md:36` (DI-015) + `entities.md:30` (ConformanceResult)
**Problem:** DI-015 gives the identity `total_flows == intra_zone + allowed + no_matching_conduit + wrong_direction + multicast_exempt` and says `idmz_bypasses` and `distinct_violating_flows` are tallied separately. But there is no stated bound `distinct_violating_flows ≤ total_flows` and no relation `distinct_violating_flows ≤ no_matching_conduit + wrong_direction + idmz_bypasses`. Since a flow can be both `Allowed` and an `IdmzBypass`, the set of violating flows is `{no_matching_conduit} ∪ {wrong_direction} ∪ {idmz_bypass}` de-duped by flow_index — but the spec never asserts the de-dup invariant formally, so a property test cannot be derived for it from DI-015 alone.
**Why it matters:** `distinct_violating_flows` drives the exit code (violations present → exit 1). An off-by-one in the de-dup (e.g., double-counting a flow that is both WrongDirection and IdmzBypass) would mis-report the violation count and is not caught by the stated accounting identity. This is a correctness-of-the-headline-number gap.
**Suggested fix:** Add to DI-015 the bounds `0 ≤ distinct_violating_flows ≤ total_flows` and the exact relation `distinct_violating_flows == |{flow_index : kind ∈ {NoMatchingConduit, WrongDirection}} ∪ {flow_index : idmz_bypass}|`, and mark it as a property-test target.

### LOW — DEC-030 anchored to CAP-007 (Classify) but the directed-broadcast detection happens during zone resolution (CAP-005); DI-016's two-step detection straddles CAP-005/CAP-007
**Severity:** LOW
**Location:** `edge-cases.md:48` (DEC-030 Capability = CAP-007) vs `entities.md:27` (ResolvedEndpoint, a CAP-005 artifact) which states the override happens "after zone resolution"
**Problem:** DEC-030 is filed under CAP-007 (Classify). But the override mutates `MatchKind` on the `ResolvedEndpoint`, which is produced by CAP-005 (Resolve endpoints to zones) at ST-5, before ST-6 classification. DI-016 itself says "The override is the only case where resolution runs before the exemption verdict," explicitly acknowledging the resolution-stage coupling. So DEC-030's capability anchor (CAP-007) is arguably the consuming stage, not the owning stage.
**Why it matters:** Semantically awkward but technically valid (MulticastExempt *is* a verdict, a CAP-007 concern). Could confuse a story-writer into placing the directed-broadcast logic entirely in the classify module rather than the resolution module, where DI-016 step-2 says it executes. LOW because the verdict-precedence framing makes CAP-007 defensible.
**Suggested fix:** Either change DEC-030's Capability to CAP-005 (or "CAP-005/CAP-007"), or add a one-line note that the *detection* is at CAP-005 (ST-5) and the *verdict* at CAP-007 (ST-6).

### LOW — Glossary "Violation" lists three kinds but the brief's "zero false silence (deny-by-default)" headline and DI-006 additive model are not cross-linked from the glossary
**Severity:** LOW
**Location:** `ubiquitous-language.md:41` (Violation) / `:43` (idmz_bypass)
**Problem:** The glossary correctly says a Violation is `NoMatchingConduit | WrongDirection | IdmzBypass`, and idmz_bypass is "NOT a Verdict." This is internally consistent. The only nit: the glossary Violation entry does not note that one flow can produce up to two Violation entries (it's stated in entities.md:29 but not echoed in the glossary), so a reader using the glossary as the authoritative term source could assume 1 flow → ≤1 violation.
**Why it matters:** Purely editorial/discoverability; the authoritative statement exists in entities.md.
**Suggested fix:** Append to the glossary Violation entry: "one flow may produce up to two Violation entries (a conduit violation + an IdmzBypass); de-duped by flow_index in `distinct_violating_flows`."

### LOW — DI-010 enumerates legal `direction` tokens as `{forward, bidirectional, unidirectional(=forward alias)}` but entities.md/glossary present `unidirectional` only as an "accepted alias," not a canonical token — minor framing drift
**Severity:** LOW
**Location:** `invariants.md:31` (DI-010) vs `entities.md:37` / `ubiquitous-language.md:50`
**Problem:** DI-010 lists `unidirectional` inside the legal-token set on equal footing with `forward`/`bidirectional`; entities.md and the glossary call it an *alias* for `forward` and call `forward`/`bidirectional` the *canonical* tokens. Same behavior, slightly different status framing (legal token vs alias).
**Why it matters:** No functional difference, but the digest canonicalization (DI-018) fixes tokens to `"forward"/"bidirectional"` only — confirming `unidirectional` is non-canonical. The DI-010 phrasing could mislead someone into thinking `unidirectional` is a third canonical form that must round-trip through the digest.
**Suggested fix:** In DI-010 write `unidirectional` as "(input alias, normalized to `forward`)" to match entities.md/DI-018.

---

## Novelty Assessment
**Novelty: MEDIUM.** The two HIGH findings are genuinely new structural contradictions a careful single-file reader would miss — the `usize`/`u64` conflict spans the brief and entities.md (different documents, both plausible in isolation), and the `RSTO`/`SH` severity-bucket conflict requires cross-reading DI-017's "otherwise" default against entities.md's explicit enumeration plus knowledge of Zeek reset-state semantics. The MED findings (cap boundary/exit-code overload, FM-008 stage filing, multicast-inside-a-zone silent-allow, `distinct_violating_flows` bound) are real gaps, not rewordings. The LOWs are refinements. This is not a converged pass — the two HIGH contradictions should block convergence until reconciled.
