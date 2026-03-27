# i18n Key 文档

所有 API 响应的 `message` 字段均使用 i18n key，前端根据 key（+ 可选 `params`）自动匹配翻译文本。

## 响应格式

### 成功响应

```json
{
  "code": "OK",
  "message": "message.auth.login.success",
  "params": { "username": "admin" },
  "data": { ... }
}
```

### 错误响应

```json
{
  "code": "BAD_REQUEST",
  "message": "error.users.invalid_role",
  "params": { "role": "invalid_value" }
}
```

> `params` 字段仅在需要动态参数时出现，无参数时不返回该字段。

---

## 成功消息 Key

| i18n_key | params | 说明 |
|---|---|---|
| `message.auth.login.success` | `username`: 用户名 | 登录成功（{username} 欢迎回来） |
| `message.auth.logout.success` | — | 登出成功 |
| `message.auth.token.success` | — | Token 续期成功 |
| `message.users.list.success` | — | 获取用户列表成功 |
| `message.users.create.success` | `username`: 新用户名 | 用户创建成功 |
| `message.users.update.success` | `username`: 被修改用户名 | 用户更新成功 |
| `message.users.delete.success` | — | 用户删除成功 |
| `message.profile.change_password.success` | — | 密码修改成功 |
| `message.logs.login_list.success` | — | 获取登录日志成功 |
| `message.logs.operation_list.success` | — | 获取操作日志成功 |
| `message.license.verify.success` | — | 许可证验证完成 |
| `message.license.activate.success` | — | 许可证激活成功 |
| `message.license.info.success` | — | 获取许可证信息成功 |

---

## 错误消息 Key

### 认证相关

| i18n_key | params | 说明 |
|---|---|---|
| `error.auth.credentials_required` | — | 用户名和密码必填 |
| `error.auth.invalid_credentials` | — | 用户名或密码错误 |
| `error.auth.account_frozen` | — | 账号已冻结 |
| `error.auth.missing_token` | — | 缺少认证 Token |
| `error.auth.token_invalid` | — | Token 无效或已过期 |
| `error.auth.admin_required` | — | 需要管理员权限 |

### 用户管理

| i18n_key | params | 说明 |
|---|---|---|
| `error.users.password_too_short` | — | 密码至少 6 位 |
| `error.users.invalid_role` | `role`: 传入的角色值 | 无效角色 |
| `error.users.invalid_status` | `status`: 传入的状态值 | 无效状态 |
| `error.users.cannot_delete_self` | — | 不能删除自己 |
| `error.users.not_found` | — | 用户不存在 |

### 个人设置

| i18n_key | params | 说明 |
|---|---|---|
| `error.profile.password_too_short` | — | 新密码至少 6 位 |

### 许可证

| i18n_key | params | 说明 |
|---|---|---|
| `error.license.key_required` | — | 许可证密钥必填 |
| `error.license.expired` | `expiresAt`: 过期时间 | 许可证已过期 |
| `error.license.invalid_format` | — | 许可证格式无效 |
| `error.license.unsupported_length` | `expected`, `got` | Token 长度不匹配 |
| `error.license.invalid_tag` | — | Tag 长度无效 |
| `error.license.verification_failed` | — | 许可证验证失败（篡改或无效） |
| `error.license.invalid_payload` | — | 许可证载荷无效 |
| `error.license.unsupported_version` | `version`: 版本号 | 不支持的许可证版本 |
| `error.license.invalid_encoding` | — | 许可证编码无效 |

### 通用

| i18n_key | params | 说明 |
|---|---|---|
| `error.resource.not_found` | — | 资源不存在 |
| `error.resource.conflict` | — | 资源已存在（唯一约束冲突） |
| `error.internal` | — | 内部服务错误 |
| `error.internal.password_hash_corrupted` | — | 密码哈希数据损坏 |
| `error.internal.password_hash_failed` | — | 密码哈希处理失败 |

---

## 操作日志 detail Key

操作日志 `detail` 字段存储 JSON 格式的国际化信息：`{"i18n_key": "...", "params": {...}}`

| i18n_key | params | 说明 |
|---|---|---|
| `operation.auth.logout` | — | 用户登出 |
| `operation.users.create_user` | `username`: 新用户名 | 管理员创建用户 |
| `operation.users.update_user` | `username`: 被修改用户名 | 管理员更新用户信息 |
| `operation.users.delete_user` | `username`: 被删除用户名 | 管理员删除用户 |
| `operation.profile.change_password` | — | 用户修改自己的密码 |

---

## 前端使用示例

```typescript
// 中文翻译模板
const zhCN: Record<string, string> = {
  // 成功消息
  "message.auth.login.success": "{username} 欢迎回来",
  "message.auth.logout.success": "登出成功",
  "message.auth.token.success": "Token 续期成功",
  "message.users.list.success": "获取用户列表成功",
  "message.users.create.success": "用户 {username} 创建成功",
  "message.users.update.success": "用户 {username} 更新成功",
  "message.users.delete.success": "用户删除成功",
  "message.profile.change_password.success": "密码修改成功",
  "message.logs.login_list.success": "获取登录日志成功",
  "message.logs.operation_list.success": "获取操作日志成功",
  "message.license.verify.success": "许可证验证完成",
  "message.license.activate.success": "许可证激活成功",
  "message.license.info.success": "获取许可证信息成功",

  // 错误消息
  "error.auth.credentials_required": "用户名和密码必填",
  "error.auth.invalid_credentials": "用户名或密码错误",
  "error.auth.account_frozen": "账号已冻结",
  "error.auth.missing_token": "缺少认证 Token",
  "error.auth.token_invalid": "Token 无效或已过期",
  "error.auth.admin_required": "需要管理员权限",
  "error.users.password_too_short": "密码至少 6 位",
  "error.users.invalid_role": "无效角色：{role}",
  "error.users.invalid_status": "无效状态：{status}",
  "error.users.cannot_delete_self": "不能删除自己",
  "error.users.not_found": "用户不存在",
  "error.profile.password_too_short": "新密码至少 6 位",
  "error.license.key_required": "许可证密钥必填",
  "error.license.expired": "许可证已过期（{expiresAt}）",
  "error.license.invalid_format": "许可证格式无效",
  "error.license.unsupported_length": "Token 长度不匹配（期望 {expected}，实际 {got}）",
  "error.license.invalid_tag": "许可证 Tag 长度无效",
  "error.license.verification_failed": "许可证验证失败（无效或被篡改）",
  "error.license.invalid_payload": "许可证载荷无效",
  "error.license.unsupported_version": "不支持的许可证版本 {version}",
  "error.license.invalid_encoding": "许可证编码无效",
  "error.resource.not_found": "资源不存在",
  "error.resource.conflict": "资源已存在",
  "error.internal": "内部服务错误",
  "error.internal.password_hash_corrupted": "密码数据异常",
  "error.internal.password_hash_failed": "密码处理失败",

  // 操作日志详情
  "operation.auth.logout": "用户登出",
  "operation.users.create_user": "创建用户 {username}",
  "operation.users.update_user": "更新用户 {username}",
  "operation.users.delete_user": "删除用户 {username}",
  "operation.profile.change_password": "修改密码",
};

// 英文翻译模板
const enUS: Record<string, string> = {
  // Success messages
  "message.auth.login.success": "Welcome back, {username}",
  "message.auth.logout.success": "Logged out successfully",
  "message.auth.token.success": "Token renewed",
  "message.users.list.success": "User list fetched",
  "message.users.create.success": "User {username} created",
  "message.users.update.success": "User {username} updated",
  "message.users.delete.success": "User deleted",
  "message.profile.change_password.success": "Password changed",
  "message.logs.login_list.success": "Login logs fetched",
  "message.logs.operation_list.success": "Operation logs fetched",
  "message.license.verify.success": "License verified",
  "message.license.activate.success": "License activated",
  "message.license.info.success": "License info fetched",

  // Error messages
  "error.auth.credentials_required": "Username and password are required",
  "error.auth.invalid_credentials": "Invalid username or password",
  "error.auth.account_frozen": "Account is frozen",
  "error.auth.missing_token": "Authentication token is missing",
  "error.auth.token_invalid": "Token is invalid or expired",
  "error.auth.admin_required": "Admin access required",
  "error.users.password_too_short": "Password must be at least 6 characters",
  "error.users.invalid_role": "Invalid role: {role}",
  "error.users.invalid_status": "Invalid status: {status}",
  "error.users.cannot_delete_self": "Cannot delete yourself",
  "error.users.not_found": "User not found",
  "error.profile.password_too_short": "New password must be at least 6 characters",
  "error.license.key_required": "License key is required",
  "error.license.expired": "License has expired ({expiresAt})",
  "error.license.invalid_format": "Invalid license format",
  "error.license.unsupported_length": "Unsupported token length (expected {expected}, got {got})",
  "error.license.invalid_tag": "Invalid token tag length",
  "error.license.verification_failed": "License verification failed (invalid or tampered)",
  "error.license.invalid_payload": "Invalid license payload",
  "error.license.unsupported_version": "Unsupported license version {version}",
  "error.license.invalid_encoding": "Invalid license encoding",
  "error.resource.not_found": "Resource not found",
  "error.resource.conflict": "Resource already exists",
  "error.internal": "Internal server error",
  "error.internal.password_hash_corrupted": "Password data corrupted",
  "error.internal.password_hash_failed": "Password processing failed",

  // Operation log details
  "operation.auth.logout": "User logged out",
  "operation.users.create_user": "Created user {username}",
  "operation.users.update_user": "Updated user {username}",
  "operation.users.delete_user": "Deleted user {username}",
  "operation.profile.change_password": "Changed password",
};

// 渲染函数
function renderMessage(
  message: string,
  params: Record<string, string> | undefined,
  locale: Record<string, string>
): string {
  let text = locale[message] || message;
  if (params) {
    for (const [key, value] of Object.entries(params)) {
      text = text.replace(`{${key}}`, value);
    }
  }
  return text;
}
```

## 命名规则

- **成功消息**：`message.<module>.<action>.success`
- **错误消息**：`error.<module>.<description>`
- **操作日志详情**：`operation.<module>.<action>`
- 新增接口时，按此规则在本文档中添加对应条目
