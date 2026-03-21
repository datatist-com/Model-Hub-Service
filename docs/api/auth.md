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
- Purpose: validate the current token, issue a fresh one (24h expiry reset),
  and return up-to-date user profile. Clients **must** replace their stored
  `accessToken` with the one returned. Call this on every app load or
  periodically to keep the session alive without a separate login.
- Auth: Yes (any valid token via Authorization / X-Token header or ?token=)
- Request params: none
- Response body:
  - `accessToken: string` — new token with refreshed expiry
  - `user: { id, username, realName, role, language, uiTheme }`
- Error cases:
  - expired / invalid token → 401
  - account frozen → 403
  - user not found → 404
- Priority: Core

## Notes
- There is no separate `refresh-token` endpoint. `GET /auth/token` serves both
  the "current user" and "token renewal" purposes in a single call.
- Token expiry is 24 hours; the frontend should call `GET /auth/token` on
  startup (or on each page navigation) to silently renew the session.
