# License API

Frontend usage evidence:
- `src/components/license/LicenseCenterModal.tsx`
- `src/pages/LoginPage.tsx`
- `src/layouts/AppLayout.tsx`

## GET /license/info
- Name: License info
- Purpose: render current license status and metadata
- Frontend usage: login and app shell license panel
- Auth: Usually Yes (may be partially available pre-auth)
- Response body:
  - `status: active | expired | missing`
  - `licenseKeyMasked`
  - `licensee`
  - `activatedAt`
  - `expiresAt`
- Confidence: Strongly Inferred
- Priority: Core

## POST /license/activate
- Name: Activate or replace license
- Purpose: submit new license key
- Frontend usage: license center modal activation/update action
- Auth: Uncertain (login page also exposes activation)
- Request body:
  - `licenseKey: string`
- Response body:
  - refreshed license info object
- Error cases:
  - invalid key
  - expired/unsupported key
- Confidence: Strongly Inferred
- Priority: Core

## GET /license/validate
- Name: Validate license capability
- Purpose: enable/disable modules by license
- Frontend usage: implied by environment/license gating UX
- Auth: Uncertain
- Response body:
  - `valid: boolean`
  - `status`
  - optional `enabledModules: string[]`
- Confidence: Uncertain
- Priority: Secondary

## Notes
- Frontend strongly suggests status-driven UX (`active`, `expired`, `missing`).
- Backend should expose machine-readable license state and timestamps.
