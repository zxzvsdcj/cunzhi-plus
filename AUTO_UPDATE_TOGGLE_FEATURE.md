# 自动更新开关功能说明

## 概述

为 cunzhi-plus 添加了自动检查更新的开关功能，用户可以在设置页面控制是否启用自动更新检查。

## 功能特性

### 1. 配置管理

- **配置文件**：在 `config.json` 中新增 `updater_config` 配置项
- **默认行为**：默认启用自动检查更新（`auto_check_updates: true`）
- **持久化**：配置更改会立即保存到配置文件

### 2. 用户界面

#### 位置
设置 → 版本检查 → 顶部开关

#### UI 组件
- 卡片样式的开关区域
- 清晰的标题和说明文字
- Naive UI Switch 组件
- 响应式布局，支持深色模式

#### 交互体验
- 即时保存：切换开关后立即保存配置
- 反馈提示：显示"已启用自动检查更新"或"已禁用自动检查更新"
- 错误处理：如果保存失败，会回滚状态并提示错误

### 3. 后端实现

#### 新增 Rust 结构和函数

**配置结构** (`src/rust/config/settings.rs`)
```rust
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UpdaterConfig {
    #[serde(default = "default_auto_check_updates")]
    pub auto_check_updates: bool,
}

pub fn default_auto_check_updates() -> bool {
    true // 默认启用
}
```

**Tauri 命令**
- `get_updater_config()`: 获取更新器配置
- `update_auto_check_updates(enabled: bool)`: 更新自动检查配置

#### 更新检查逻辑

修改 `check_for_updates` 函数 (`src/rust/ui/updater.rs`)：

1. 首先检查配置中的 `auto_check_updates` 开关
2. 如果禁用，立即返回表示"已禁用"的结果
3. 如果启用，继续正常的更新检查流程

```rust
// 检查是否启用了自动检查更新
let state = app.state::<AppState>();
let auto_check_enabled = {
    let config = state.config.lock().map_err(|e| format!("获取配置失败: {}", e))?;
    config.updater_config.auto_check_updates
};

if !auto_check_enabled {
    log::info!("⏸️ 自动检查更新已禁用，跳过检查");
    return Ok(UpdateInfo {
        available: false,
        current_version: env!("CARGO_PKG_VERSION").to_string(),
        latest_version: env!("CARGO_PKG_VERSION").to_string(),
        release_notes: "自动检查更新已禁用".to_string(),
        download_url: String::new(),
    });
}
```

### 4. 前端实现

#### 组件修改 (`src/frontend/components/settings/VersionChecker.vue`)

**新增状态**
```typescript
const autoCheckEnabled = ref(true) // 自动检查更新开关
```

**新增函数**
- `loadAutoCheckConfig()`: 从后端加载配置
- `handleAutoCheckToggle(enabled)`: 处理开关切换

**UI 模板**
```vue
<div class="flex items-center justify-between p-3 bg-surface-50 dark:bg-surface-800 rounded-lg border border-surface-200 dark:border-surface-700">
  <div class="flex-1">
    <div class="text-sm font-medium text-on-surface dark:text-on-surface mb-1">
      自动检查更新
    </div>
    <div class="text-xs text-on-surface-secondary dark:text-on-surface-secondary">
      启用后将在应用启动时自动检查是否有新版本
    </div>
  </div>
  <n-switch
    :value="autoCheckEnabled"
    @update:value="handleAutoCheckToggle"
  />
</div>
```

## 使用说明

### 启用自动检查更新

1. 打开应用设置
2. 导航到"版本检查"部分
3. 确保"自动检查更新"开关处于开启状态（默认）
4. 应用将在启动时自动检查更新

### 禁用自动检查更新

1. 打开应用设置
2. 导航到"版本检查"部分
3. 关闭"自动检查更新"开关
4. 应用将不再自动检查更新
5. 您仍然可以通过"检查更新"按钮手动检查

## 技术细节

### 配置文件格式

```json
{
  "ui_config": { ... },
  "audio_config": { ... },
  "reply_config": { ... },
  "mcp_config": { ... },
  "telegram_config": { ... },
  "custom_prompt_config": { ... },
  "shortcut_config": { ... },
  "updater_config": {
    "auto_check_updates": true
  }
}
```

### API 调用流程

1. **前端加载配置**
   ```typescript
   const config = await invoke('get_updater_config')
   autoCheckEnabled.value = config.auto_check_updates
   ```

2. **用户切换开关**
   ```typescript
   await invoke('update_auto_check_updates', { enabled })
   ```

3. **应用启动时检查更新**
   ```rust
   // 在 check_for_updates 中首先检查配置
   if !auto_check_enabled {
       // 跳过更新检查
   }
   ```

## 兼容性

- **向后兼容**：旧配置文件会自动使用默认值 `true`
- **配置迁移**：无需手动迁移，应用会自动添加新配置项
- **跨平台**：支持 Windows、macOS、Linux

## 日志记录

启用日志记录以便调试：

- **禁用时**：`⏸️ 自动检查更新已禁用，跳过检查`
- **启用时**：`🔍 开始检查更新`

## 测试建议

1. **首次启动测试**
   - 删除配置文件，重新启动应用
   - 验证默认行为（应该自动检查更新）

2. **开关切换测试**
   - 关闭自动更新，重启应用
   - 验证不会自动检查更新
   - 打开自动更新，重启应用
   - 验证会自动检查更新

3. **手动检查测试**
   - 即使关闭自动更新
   - 手动点击"检查更新"按钮
   - 验证仍然可以手动检查

4. **配置持久化测试**
   - 修改开关状态
   - 关闭应用
   - 重新打开应用
   - 验证状态保持不变

## 未来改进

1. **更新频率控制**：可以添加更新检查间隔设置（如每天、每周）
2. **后台更新**：支持后台静默下载更新
3. **更新通知**：可选的桌面通知
4. **更新日志预览**：在通知中直接显示更新内容摘要

## 相关文件

### Rust 后端
- `src/rust/config/settings.rs` - 配置结构定义
- `src/rust/ui/commands.rs` - Tauri 命令实现
- `src/rust/ui/updater.rs` - 更新检查逻辑
- `src/rust/app/builder.rs` - 命令注册

### 前端
- `src/frontend/components/settings/VersionChecker.vue` - 版本检查组件
- `src/frontend/components/tabs/SettingsTab.vue` - 设置标签页

## 维护说明

- 配置默认值在 `src/rust/config/settings.rs` 的 `default_auto_check_updates()` 函数中定义
- UI 文本可以在 `VersionChecker.vue` 的模板部分修改
- 更新检查逻辑在 `src/rust/ui/updater.rs` 的 `check_for_updates()` 函数中

---

**版本**: 1.0.0  
**作者**: cunzhi-plus Team  
**日期**: 2025-01-XX

