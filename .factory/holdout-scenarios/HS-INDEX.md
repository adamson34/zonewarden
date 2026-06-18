---
document_type: holdout-scenario-index
level: ops
version: "1.0"
status: draft
producer: story-writer
timestamp: 2026-06-17T00:00:00
phase: 2
traces_to: specs/prd.md
---

# Holdout Scenario Index: zonewarden

**Total scenarios:** 10
**All scenarios:** must-pass (all P0 BCs; no P1/P2 scope in MVP)

## Scenario Table

| ID | Title | Category | BCs | Wave Gate | Must Pass |
|----|-------|----------|-----|-----------|-----------|
| HS-001 | Deny-by-Default — No Conduit = Violation | behavioral-subtleties | BC-1.04.002, BC-1.04.003 | Wave 4 | true |
| HS-002 | IDMZ Bypass — Allowed Flow Still Flags Security Violation | security-probes | BC-1.04.007, BC-1.04.008 | Wave 4 | true |
| HS-003 | Multicast Traffic Is Exempt — No False Violations | edge-case-combinations | BC-1.03.003, BC-1.04.011 | Wave 4 | true |
| HS-004 | Deterministic Output — Identical Inputs Produce Identical Output | behavioral-subtleties | BC-1.05.002, BC-1.05.003 | Wave 5 | true |
| HS-005 | Unspecified Address Flows Are Skipped With Warning | security-probes | BC-1.02.003, BC-1.06.005 | Wave 2 | true |
| HS-006 | Portless Protocol Matches Only "Any" PortSet | edge-case-combinations | BC-1.04.006, BC-1.01.009 | Wave 4 | true |
| HS-007 | Ingest Cap Breach Aborts Cleanly — No Partial Output | behavioral-subtleties | BC-1.02.006 | Wave 2 | true |
| HS-008 | Accounting Identity Holds Across All Verdict Kinds | behavioral-subtleties | BC-1.05.001, BC-1.04.010 | Wave 5 | true |
| HS-009 | zonewarden Never Opens a Network Socket | security-probes | BC-1.06.007 | Wave 5 | true |
| HS-010 | Directionality — WrongDirection vs NoMatchingConduit | behavioral-subtleties | BC-1.04.004, BC-1.04.005 | Wave 4 | true |

## Holdout Coverage by Concern

| Security/Quality Concern | Scenarios |
|--------------------------|-----------|
| Deny-by-default enforcement | HS-001 |
| IDMZ bypass (headline security control) | HS-002 |
| Multicast exemption (false positive prevention) | HS-003 |
| Determinism / reproducible evidence | HS-004 |
| Unspecified address skip (DI-013) | HS-005 |
| PortSet Any vs explicit range semantics | HS-006 |
| Ingest cap + clean abort | HS-007 |
| Accounting identity (DI-015) | HS-008 |
| Offline invariant (DI-012) | HS-009 |
| Directionality (WrongDirection vs NoMatchingConduit) | HS-010 |

## Files

| File | Scenario |
|------|----------|
| `wave-scenarios/HS-001-deny-by-default.md` | HS-001 |
| `wave-scenarios/HS-002-idmz-bypass.md` | HS-002 |
| `wave-scenarios/HS-003-multicast-exemption.md` | HS-003 |
| `wave-scenarios/HS-004-determinism.md` | HS-004 |
| `wave-scenarios/HS-005-unspecified-address-skip.md` | HS-005 |
| `wave-scenarios/HS-006-portset-semantics.md` | HS-006 |
| `wave-scenarios/HS-007-ingest-cap.md` | HS-007 |
| `wave-scenarios/HS-008-accounting-identity.md` | HS-008 |
| `wave-scenarios/HS-009-offline-invariant.md` | HS-009 |
| `wave-scenarios/HS-010-wrong-direction.md` | HS-010 |
