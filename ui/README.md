Build UI (the `-r` is important to also update the OpenAPI client):

```bash
pnpm -r build
```

Generate OpenAPI client (when openapi.yaml has been changed, included in `pnpm -r build`):

```bash
pnpm update-api
```

To publish a new version to npmjs:

```bash
cd src/lib/api
pnpm build
pnpm version patch --no-git-tag-version
pnpm publish --access public
```

## City Transit Normalization Rules

The UI applies a small normalization pass before rendering itinerary legs so misclassified local rail/tram legs do not show ferry icons.

### Source Fields Used

- `mode`
- `routeShortName`
- `routeLongName`
- `displayName`
- `headsign`
- `tripShortName`
- `agencyId`
- `agencyName`
- `routeId`

### Active Rules

1. Stockholm (SL) metro:

- If `mode = FERRY` and line is `10/11/13/14/17/18/19` in SL context, map to `SUBWAY`.

2. Stockholm (SL) local rail:

- If `mode = FERRY` and line is `25-29` (or text indicates `Roslagsbanan`/`Saltsjöbanan`) in SL context, map to `RAIL`.

3. Gothenburg (Västtrafik) tram:

- If `mode = FERRY` and Västtrafik context + tram line hint (`1-11` or `13`, or tram text hint), map to `TRAM`.

### Fallback Behavior

- If rule context is unclear, keep original `mode` and `displayName`.
- If only part of metadata is present, apply the rule only when there is enough confidence from agency + line/text hints.

## Language Toggle

- Supported in UI toggle: `en` and `sv`.
- Default fallback language: `en`.
- Selection precedence: URL `?language=...` -> saved `localStorage` value -> browser preference -> default.
