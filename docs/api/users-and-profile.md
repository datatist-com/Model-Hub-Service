# Users and Profile API

Frontend usage evidence:
- `src/pages/UsersPage.tsx`
- `src/pages/ProfilePage.tsx`
- `src/auth/roles.ts`

## GET /users
- Name: List users
- Purpose: user administration table
- Frontend usage: users list page with filters/actions
- Auth: Yes (platform_admin only)
- Request params:
  - `page`, `pageSize`
  - optional `role` (enum: `model_developer`, `model_operator`, `platform_admin`), `status`, `keyword`
- Response body:
  - paginated items with `id`, `username`, `realName`, `role`, `status`, `createdAt`
- Confidence: Strongly Inferred
- Priority: Core

## POST /users
- Name: Create user
- Purpose: add account
- Frontend usage: create user modal/form
- Auth: Yes
- Request body:
  - `username`, `realName`, `password`, `role`
- Response body: created user object
- Confidence: Strongly Inferred
- Priority: Core

## PUT /users/{id}
- Name: Update user
- Purpose: modify user attributes/status
- Frontend usage: edit modal/actions
- Auth: Yes
- Path params: `id`
- Request body: editable subset such as `realName`, `role`, `status`
- Response body: updated user object
- Confidence: Strongly Inferred
- Priority: Core

## DELETE /users/{id}
- Name: Delete user
- Purpose: remove account
- Frontend usage: delete action with confirmation
- Auth: Yes
- Path params: `id`
- Request body: optional admin password/verification data
- Response body: `{ success: true }`
- Confidence: Strongly Inferred
- Priority: Core

## PUT /profile/password
- Name: Change password
- Purpose: security update for current user
- Frontend usage: profile/change-password flow
- Auth: Yes
- Request body:
  - `currentPassword`
  - `newPassword`
- Response body: `{ success: true }`
- Confidence: Strongly Inferred
- Priority: Core

## GET /profile/login-history
- Name: Login history
- Purpose: show recent sign-in records
- Frontend usage: profile page records table
- Auth: Yes
- Request params: `page`, `pageSize`
- Response body: paginated events with `time`, `ip`, `location`, `result`
- Confidence: Strongly Inferred
- Priority: Secondary

## GET /profile/action-history
- Name: Action history
- Purpose: show user activity audit
- Frontend usage: profile page action records
- Auth: Yes
- Request params: `page`, `pageSize`
- Response body: paginated action logs
- Confidence: Strongly Inferred
- Priority: Secondary
