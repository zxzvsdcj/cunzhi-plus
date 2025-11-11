# MCP协议正确测试脚本

$ErrorActionPreference = "Stop"

Write-Host "=== MCP协议测试 ===" -ForegroundColor Cyan
Write-Host ""

# 创建符合MCP协议的测试输入
$testSequence = @(
    '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2024-11-05","capabilities":{},"clientInfo":{"name":"test-client","version":"1.0.0"}}}'
    '{"jsonrpc":"2.0","method":"notifications/initialized"}'
    '{"jsonrpc":"2.0","id":2,"method":"tools/list"}'
) -join "`n"

Write-Host "测试序列:" -ForegroundColor Yellow
Write-Host $testSequence
Write-Host ""

# 发送测试
Write-Host "发送到MCP服务器..." -ForegroundColor Cyan
$result = $testSequence | .\target\release\寸止.exe 2>&1 | Out-String

Write-Host ""
Write-Host "=== 服务器响应 ===" -ForegroundColor Cyan
Write-Host $result

# 验证
Write-Host ""
Write-Host "=== 验证结果 ===" -ForegroundColor Yellow

$checks = @{
    "初始化成功" = ($result -match "protocolVersion")
    "工具列表返回" = ($result -match '"tools"')
    "包含zhi工具" = ($result -match '"name":\s*"zhi"')
    "包含enhance工具" = ($result -match '"name":\s*"enhance"')
}

$passed = 0
foreach ($check in $checks.GetEnumerator()) {
    if ($check.Value) {
        Write-Host "✓ $($check.Key)" -ForegroundColor Green
        $passed++
    } else {
        Write-Host "✗ $($check.Key)" -ForegroundColor Red
    }
}

Write-Host ""
Write-Host "结果: $passed / $($checks.Count) 通过" -ForegroundColor $(if ($passed -eq $checks.Count) { "Green" } else { "Yellow" })

if ($passed -eq $checks.Count) {
    Write-Host ""
    Write-Host "✓ MCP服务器工作正常!" -ForegroundColor Green
    Write-Host ""
    Write-Host "=== 配置示例 ===" -ForegroundColor Cyan
    Write-Host "添加到 Claude Desktop 或 Cursor 的 mcp.json:" -ForegroundColor Yellow
    Write-Host @"
{
  "mcpServers": {
    "cunzhi-plus": {
      "command": "E:\\cursor\\cunzhi-plus\\target\\release\\寸止.exe",
      "args": [],
      "env": {}
    }
  }
}
"@
    Write-Host ""
    Write-Host "=== 可用工具 ===" -ForegroundColor Cyan
    Write-Host "1. zhi    - 智能交互工具(原寸止功能)" -ForegroundColor White
    Write-Host "2. enhance - 提示词增强工具(新功能)" -ForegroundColor Green
    Write-Host "3. ji     - 记忆管理工具" -ForegroundColor White
    Write-Host ""
    Write-Host "=== 使用示例 ===" -ForegroundColor Cyan
    Write-Host "在 AI 助手中:" -ForegroundColor Yellow
    Write-Host ""
    Write-Host "@enhance 帮我实现一个用户登录接口" -ForegroundColor White
    Write-Host "  → 自动提示词增强 + 需求分析 + 代码生成" -ForegroundColor Gray
    Write-Host ""
    Write-Host "@enhance /e 写一个数据统计功能" -ForegroundColor White
    Write-Host "  → 完整四阶管线 + 评分闭环" -ForegroundColor Gray
    Write-Host ""
    Write-Host "@zhi 是否要继续执行?" -ForegroundColor White
    Write-Host "  → 交互式确认对话" -ForegroundColor Gray
    Write-Host ""
    exit 0
} else {
    Write-Host ""
    Write-Host "✗ 测试失败，请检查日志" -ForegroundColor Red
    exit 1
}

