---
summary: 'Inventory of known route mode/icon/line mislabels with normalization status.'
read_when:
  - Auditing mode/icon correctness.
  - Planning or validating normalization passes.
---

# Mislabel Inventory

## Scope

- Focus: itinerary legs with wrong mode/icon/line naming in UI.
- Source: reproducible route metadata patterns seen in Swedish regional feeds.

## Inventory

| ID | Context | Repro pattern | Wrong render | Expected render | Status |
| --- | --- | --- | --- | --- | --- |
| `MI-001` | Stockholm SL metro | `mode=FERRY`, `routeShortName in {10,11,13,14,17,18,19}`, SL agency context | Ferry icon/text | Subway icon/text | DONE |
| `MI-002` | Stockholm SL local rail | `mode in {FERRY,SUBURBAN}`, `routeShortName in {25..29}`, SL agency context | Ferry/Suburban icon/text | Rail icon + Roslagsbanan/Saltsjöbanan naming | DONE |
| `MI-003` | Gothenburg Västtrafik tram | `mode in {FERRY,BUS}`, tram line or tram text, Västtrafik context | Ferry/Bus icon/text | Tram icon/text (`Spårvagn ...`) | DONE |

## Pass 2026-02-14

- Applied normalization for `MI-002`: `SUBURBAN -> RAIL` in preprocessing flow.
- Applied normalization for `MI-003`: `BUS -> TRAM` when tram textual signals are present.
- Regression tests:
- `ui/src/lib/stockholmMetro.test.ts`
- `ui/src/lib/preprocessItinerary.test.ts`
