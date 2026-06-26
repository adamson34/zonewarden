# Wave 5 Demo Evidence — zonewarden CLI (S-6.01/02/03)

Captured 2026-06-26 from `target/release/zonewarden` against test fixtures.
Demonstrates the full pipeline end-to-end: load → validate → ingest → classify → aggregate → report.

### AC: conformant → exit 0 (text)
```
$ zonewarden --policy zonewarden/tests/fixtures/cli_policy.yaml --flows zonewarden/tests/fixtures/conformant.log
Summary:
  policy_digest: 72ea20b58141a644de50d9ae32060caf295daf385b6ec7c92052cbdec5212d12
  total_flows: 2
  intra_zone: 1
  allowed: 1
  no_matching_conduit: 0
  wrong_direction: 0
  multicast_exempt: 0
  idmz_bypasses: 0
  distinct_violating_flows: 0
  external_endpoints: 0
  skipped: 0
No violations found
exit=0
```

### AC: violations → exit 1 (json)
```
$ zonewarden --policy zonewarden/tests/fixtures/cli_policy.yaml --flows zonewarden/tests/fixtures/violating.log --format json
{"schema_version":"1.0","policy_digest":"72ea20b58141a644de50d9ae32060caf295daf385b6ec7c92052cbdec5212d12","total_flows":1,"intra_zone":0,"allowed":0,"no_matching_conduit":1,"wrong_direction":0,"multicast_exempt":0,"idmz_bypasses":0,"distinct_violating_flows":1,"external_endpoints":0,"skipped":0,"warnings":[],"violations":[{"flow_index":0,"src_zone":"plc","dst_zone":"hist","kind":"NoMatchingConduit","severity":"Established","explanation":"no conduit permits flow plc -> hist","src_ip":"10.0.1.5","dst_ip":"10.0.3.9","src_port":40000,"dst_port":9999,"proto":"tcp","service_source":"Unknown","conn_state":"Established"}]}
exit=1
```

### AC: IDMZ bypass → exit 1 (text, heuristic-flagged)
```
$ zonewarden --policy zonewarden/tests/fixtures/cli_policy.yaml --flows zonewarden/tests/fixtures/idmz_bypass.log
Summary:
  policy_digest: 72ea20b58141a644de50d9ae32060caf295daf385b6ec7c92052cbdec5212d12
  total_flows: 1
  intra_zone: 0
  allowed: 1
  no_matching_conduit: 0
  wrong_direction: 0
  multicast_exempt: 0
  idmz_bypasses: 1
  distinct_violating_flows: 1
  external_endpoints: 0
  skipped: 0
VIOLATION [Established] flow_index=0 plc -> it [IdmzBypass] 10.0.1.5:40000 -> 10.0.5.9:502 tcp Modbus [heuristic] — direct OT-IT flow without an IDMZ endpoint
exit=1
```

### AC: Mermaid topology (violations highlighted)
```
$ zonewarden --policy zonewarden/tests/fixtures/cli_policy.yaml --flows zonewarden/tests/fixtures/violating.log --format mermaid
graph LR
    z0["hist (L3)"]:::violation
    z1["it (L4)"]
    z2["plc (L1)"]:::violation
    z2 --> z0
    z2 --> z1
    classDef violation fill:#f66
exit=1
```

### AC: ingest cap breach → exit 2
```
$ zonewarden --policy zonewarden/tests/fixtures/cli_policy.yaml --flows zonewarden/tests/fixtures/huge.log --max-flows 5
error: E-SYS-001: ingest cap exceeded: max_flows = 5. Run aborted after 5 flows. Re-run with a higher --max-flows cap or split the input.
exit=2
```

### AC: --max-flows 0 usage error → exit 2
```
$ zonewarden --policy zonewarden/tests/fixtures/cli_policy.yaml --flows zonewarden/tests/fixtures/conformant.log --max-flows 0
error: E-SYS-002: --max-flows must be > 0
exit=2
```

### AC: policy error → exit 2
```
$ zonewarden --policy zonewarden/tests/fixtures/invalid_direction.yaml --flows zonewarden/tests/fixtures/conformant.log
error: E-POL-003: invalid value for `direction`: `both`
exit=2
```

### AC: empty flows → exit 0 (all-zero)
```
$ zonewarden --policy zonewarden/tests/fixtures/cli_policy.yaml --flows zonewarden/tests/fixtures/empty_flows.log --format json
{"schema_version":"1.0","policy_digest":"72ea20b58141a644de50d9ae32060caf295daf385b6ec7c92052cbdec5212d12","total_flows":0,"intra_zone":0,"allowed":0,"no_matching_conduit":0,"wrong_direction":0,"multicast_exempt":0,"idmz_bypasses":0,"distinct_violating_flows":0,"external_endpoints":0,"skipped":0,"warnings":[],"violations":[]}
exit=0
```

