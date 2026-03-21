# License API

All license endpoints are **public** (no authentication required).

Base path: `POST|GET /api/v1/license`

---

## GET /api/v1/license

Return the current active license, or a `missing` placeholder when none has been activated.

**Auth:** None

**Response `200`:**
```json
{
  "status": "active",
  "licenseKeyMasked": "ABCDEFGH****WXYZ",
  "projectName": "MyProject",
  "activatedAt": "2024-01-15T08:00:00Z",
  "expiresAt": "2025-01-15T00:00:00Z"
}
```

`status` values:
- `active` — license is present and not expired
- `expired` — license present but `expires_at` is in the past
- `missing` — no license row exists

When `missing`, all other fields are `null`.

---

## POST /api/v1/license/verify

Verify a license key without persisting it.

**Auth:** None

**Request body:**
```json
{ "licenseKey": "<64-char base64url token>" }
```

**Response `200`:**
```json
{
  "valid": true,
  "expired": false,
  "projectName": "MyProject",
  "expiresAt": "2025-01-15T00:00:00Z"
}
```

Returns `400` if the key cannot be decrypted or has an unsupported version.

---

## POST /api/v1/license/activate

Activate (or replace) the instance license.

**Auth:** None

**Request body:**
```json
{ "licenseKey": "<64-char base64url token>" }
```

**Response `200`** — same shape as `GET /api/v1/license` with `status = active`.

**Error cases:**
- `400 Bad Request` — key cannot be decrypted or version ≠ 2
- `422 Unprocessable Entity` — key decrypted but already expired

---

## License format (internal)

| Segment | Bytes | Description |
|---------|-------|-------------|
| nonce   | 12    | AES-GCM nonce |
| ciphertext | 25 | encrypted payload |
| tag     | 11    | truncated GCM auth tag |

Decoded token is 48 bytes from 64-char base64url (no padding).

Plaintext payload (25 bytes):

| Offset | Size | Description |
|--------|------|-------------|
| 0      | 1    | Version — must be `2` |
| 1–4    | 4    | Expiry — big-endian u32 Unix timestamp |
| 5–24   | 20   | Project name — UTF-16LE, null-padded |

Encryption: AES-256-GCM with embedded key; tag is 11 bytes (padded to 16 bytes at decryption time).
