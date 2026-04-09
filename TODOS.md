# iLEAP v1.1.0 Implementation TODOs

Tracked spec issues and open implementation questions discovered during v1.1.0 conformance work.

---

## Spec Issues

### TODO-1: `emissionFactorWTW` / `emissionFactorTTW` field-level optionality vs context-level requirement

**Location:** `ileap-data-model/src/lib.rs` – `EnergyCarrier` struct

**Issue:** The spec defines `emissionFactorWTW` and `emissionFactorTTW` as `O` (optional) at the field level, but the narrative text states they MUST be defined when `EnergyCarrier` is used in the context of a TOC or HOC. The TAD context does not require them.

**Current behaviour:** Both fields are kept as required (`WrappedDecimal`, non-optional) for simplicity. This means TAD `energyCarriers` must also include these fields even though the spec does not mandate them for TAD.

**Resolution needed:** Either:
a) Make them `Option<WrappedDecimal>` and add application-layer validation based on context (TOC/HOC vs TAD), or
b) Confirm with the spec that "O at field level, M in TOC/HOC context" maps to "always required" in the Rust type and the spec text is context-sensitive documentation only.

---

### TODO-2: `ModeSpecificReport.transportMode` field name inconsistency

**Location:** `ileap-data-model/src/lib.rs` – `ModeSpecificReport` struct

**Issue:** The spec example in Section 7.7 (HTTP response body) uses the field name `"mode"`, while the data type table in Section 6.7.10 specifies `"transportMode"`. These are contradictory.

**Current behaviour:** Implemented as `transportMode` (camelCase, per the data type table definition).

**Resolution needed:** Clarify with spec authors which field name is authoritative — `"mode"` (as in the example) or `"transportMode"` (as in the table). Update the implementation accordingly.

---

### TODO-3: `pactPds` and `comment` proposed attributes

**Location:** `ileap-data-model/src/lib.rs` – `ShipmentFootprint`, `Toc`, `Hoc` structs

**Issue:** Both `pactPds` (primary data share) and `comment` appear in the spec with an "Issue" tag, indicating they are proposed attributes pending community consensus. They may be renamed, removed, or promoted before the final spec.

**Current behaviour:** Implemented as optional fields (`Option<WrappedDecimal>` and `Option<String>` respectively) with `skip_serializing_if = "Option::is_none"`.

**Resolution needed:** Monitor spec issue tracker; update or remove these fields once community decision is made.

---

### TODO-4: `secondaryEmissionFactorSources` proposed attribute

**Location:** `ileap-data-model/src/lib.rs` – `ShipmentFootprint`, `Toc`, `Hoc` structs

**Issue:** Same as TODO-3 — this field also carries an "Issue" tag in the spec, indicating pending community input.

**Current behaviour:** Implemented as `Option<Vec<SecondaryEmissionFactorSource>>` with `skip_serializing_if = "Option::is_none"`.

**Resolution needed:** Same as TODO-3.

---

## Implementation Gaps

### TODO-5: Standalone demo data does not fully populate M* fields

**Location:** `demo-api/src/main.rs` – `standalone_shipments()`, `standalone_tocs()`, `standalone_hocs()`

**Issue:** The standalone API endpoints (`/v1/ileap/shipments`, `/v1/ileap/tocs`, `/v1/ileap/hocs`) serve data derived from `PCF_DEMO_DATA` with only `spec_version`, `company_name`, and `created_at` populated. Other M* fields — `reference_period_start`, `reference_period_end` — are not populated from the PACT wrapper.

**Resolution needed:** Either:
a) Add dedicated standalone-only data objects with all M* fields set, or
b) Map all M* fields from their PACT equivalents (e.g., `reference_period_start` from `pcf.reference_period_start`).

---

### TODO-6: `/v1/ileap/aed` demo data is minimal

**Location:** `demo-api/src/sample_data.rs` – `ILEAP_AED_DEMO_DATA`

**Issue:** Only one `AggregatedReport` example is provided. Real-world testing would benefit from multiple reports covering different scenarios (multi-modal, with hub operations, deprecated reports, etc.).

**Resolution needed:** Expand demo data with additional `AggregatedReport` examples.

---

### TODO-7: PACT-based endpoint deprecation notice

**Location:** `demo-api/src/main.rs` – `/2/footprints`, `/2/footprints/<id>`

**Issue:** Per v1.1.0 spec, the PACT-based `/2/footprints` endpoint is deprecated and will be removed in v2.0.0. The demo API should signal this deprecation (e.g., via `Deprecation` response header or OpenAPI `deprecated: true`).

**Resolution needed:** Add deprecation markers to the `/2/footprints` endpoints.

---

### TODO-8: Filter support for standalone endpoints is not spec-conformant

**Location:** `demo-api/src/main.rs` – `get_shipments()`, `get_tocs()`, `get_hocs()`

**Issue:** The iLEAP standalone spec defines a filter syntax using `$name=value` query parameters with dot notation for nested fields. The current demo implementation reuses the flattened-JSON filter approach from the TAD endpoint, which may not match the spec's filter semantics exactly.

**Resolution needed:** Verify filter syntax against spec Section 7 and align implementation.
