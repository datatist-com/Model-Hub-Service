# Auth API

Frontend usage evidence:
- `src/pages/LoginPage.tsx`
- `src/router/index.tsx`
- `src/auth/token.ts`
- `src/layouts/AppLayout.tsx`

## POST /auth/login
- Name: Login
- Purpose: exchange username/password for session token and user context
- Frontend usage: login form submit and post-login routing
- Auth: No
- Request body:
  - `username: string`
  - `password: string`
- Response body:
  - `accessToken: string`
  - `user: { id, username, realName, role, language, uiTheme }`
  - `role` enum: `model_developer`, `model_operator`, `platform_admin`
- Error cases:
  - invalid credentials
  - frozen/disabled account
- Priority: Core

## POST /auth/logout
- Name: Logout
- Purpose: stateless JWT logout (acknowledge, client discards token)
- Frontend usage: user menu logout action
- Auth: Yes
- Request body: none
- Response body: `{ success: true }`
- Priority: Core

## GET /auth/token
- Name: Token renewal + current user info
- Purpose: validate the current token, extend its DB expiry by 24 h from now,
  and return up-to-date user profile. The same `accessToken` string is returned
  (the token itself does not change, only its expiry in the DB is updated).
  Call this on app load or periodically to keep the session alive.
- Auth: Yes (any valid token via Authorization / X-Token header or ?token=)
- Request params: none
- Response body:
  - `accessToken: string` — same token with refreshed 24 h expiry in DB
  - `user: { id, username, realName, role, language, uiTheme }`
- Error cases:
  - token not found in DB / already revoked / expired → 401
  - account frozen → 403
  - user not found → 404
- Priority: Core

## Notes
- Tokens are stored in the `tokens` table with IP and device info recorded at
  login time. Logout marks the token as `revoked` in DB.
- There is no separate `refresh-token` endpoint. `GET /auth/token` serves both
  the "current user" and "token renewal" purposes in a single call.
- Token expiry is 24 hours from last `GET /auth/token` call; the frontend
  should call this on startup (or on each page navigation) to silently renew.
