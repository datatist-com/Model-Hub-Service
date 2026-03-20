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
  - `user: { id, username, role, language, uiTheme }`
  - `role` enum: `model_developer`, `model_operator`, `platform_admin`
- Error cases:
  - invalid credentials
  - frozen/disabled account
- Confidence: Strongly Inferred
- Priority: Core

## POST /auth/logout
- Name: Logout
- Purpose: invalidate current session
- Frontend usage: user menu logout action
- Auth: Yes
- Request body: none
- Response body: `{ success: true }`
- Confidence: Strongly Inferred
- Priority: Core

## POST /auth/refresh-token
- Name: Refresh token
- Purpose: renew expired access token
- Frontend usage: implied by token-guarded routing and long-lived UI session
- Auth: refresh token or valid session cookie
- Request body: implementation-specific
- Response body: `{ accessToken: string }`
- Confidence: Uncertain
- Priority: Secondary

## GET /auth/current-user
- Name: Current user info
- Purpose: load role/preferences for shell initialization
- Frontend usage: layout init and menu permissions
- Auth: Yes
- Request params: none
- Response body:
  - `{ id, username, role, language, uiTheme }`
- Confidence: Strongly Inferred
- Priority: Core

## Notes and Assumptions
- Current frontend stores token/client-side session data in auth helpers.
- Backend should return role and display preferences in login or current-user response to avoid extra calls.
