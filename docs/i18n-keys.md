# 操作日志 i18n Key 文档

操作日志的 `detail` 字段存储 JSON 格式的国际化信息，前端根据 `i18n_key` + `params` 自动匹配生成用户可读文本。

## JSON 格式

```json
{
  "i18n_key": "operation.<module>.<action>",
  "params": {
    "<field>": "<value>"
  }
}
```

## Key 列表

| i18n_key | 模块 | 操作 | params 字段 | 说明 |
|---|---|---|---|---|
| `operation.auth.logout` | auth | logout | _(无)_ | 用户登出 |
| `operation.users.create_user` | users | create_user | `username`: 新用户名 | 管理员创建用户 |
| `operation.users.update_user` | users | update_user | `username`: 被修改用户名 | 管理员更新用户信息 |
| `operation.users.delete_user` | users | delete_user | `username`: 被删除用户名 | 管理员删除用户 |
| `operation.profile.change_password` | profile | change_password | _(无)_ | 用户修改自己的密码 |

## 前端使用示例

```typescript
// detail JSON 示例
const detail = {
  i18n_key: "operation.users.create_user",
  params: { username: "alice" }
};

// 中文翻译模板
const zhCN = {
  "operation.auth.logout": "用户登出",
  "operation.users.create_user": "创建用户 {username}",
  "operation.users.update_user": "更新用户 {username}",
  "operation.users.delete_user": "删除用户 {username}",
  "operation.profile.change_password": "修改密码",
};

// 英文翻译模板
const enUS = {
  "operation.auth.logout": "User logged out",
  "operation.users.create_user": "Created user {username}",
  "operation.users.update_user": "Updated user {username}",
  "operation.users.delete_user": "Deleted user {username}",
  "operation.profile.change_password": "Changed password",
};

// 渲染函数
function renderDetail(detail: { i18n_key: string; params: Record<string, string> }, locale: Record<string, string>): string {
  let text = locale[detail.i18n_key] || detail.i18n_key;
  for (const [key, value] of Object.entries(detail.params)) {
    text = text.replace(`{${key}}`, value);
  }
  return text;
}
```

## 命名规则

- 格式：`operation.<module>.<action>`
- `module` 对应 `operation_logs` 表的 `module` 字段
- `action` 对应 `operation_logs` 表的 `action` 字段
- 新增操作时，按此规则在本文档中添加对应条目
