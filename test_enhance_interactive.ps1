# 提示词增强工具交互式测试脚本 (PowerShell)

$ErrorActionPreference = "Stop"

Write-Host "=== 寸止提示词增强功能交互式测试 ===" -ForegroundColor Cyan
Write-Host ""

# 1. 确保编译
Write-Host "步骤 1: 确保项目已编译" -ForegroundColor Yellow
if (Test-Path ".\target\release\寸止.exe") {
    Write-Host "✓ 可执行文件存在" -ForegroundColor Green
} else {
    Write-Host "正在编译..." -ForegroundColor Yellow
    cargo build --release
    Write-Host "✓ 编译完成" -ForegroundColor Green
}
Write-Host ""

# 2. 创建完整的MCP测试会话
Write-Host "步骤 2: 测试MCP服务器通信" -ForegroundColor Yellow

# 创建测试输入文件
$testInput = @"
{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2024-11-05","capabilities":{},"clientInfo":{"name":"test-client","version":"1.0.0"}}}
{"jsonrpc":"2.0","id":2,"method":"tools/list"}
{"jsonrpc":"2.0","id":3,"method":"tools/call","params":{"name":"enhance","arguments":{"prompt":"帮我写一个用户登录接口","enable_pipeline":false,"enable_scoring":false}}}
"@

$testInput | Out-File -FilePath "test_input.txt" -Encoding UTF8 -NoNewline

Write-Host "发送测试请求到MCP服务器..." -ForegroundColor Cyan
$result = Get-Content "test_input.txt" | .\target\release\寸止.exe 2>&1 | Out-String

Write-Host ""
Write-Host "=== MCP服务器响应 ===" -ForegroundColor Cyan
Write-Host $result

# 3. 解析响应
Write-Host ""
Write-Host "步骤 3: 验证响应" -ForegroundColor Yellow

$tests = @(
    @{ Name = "服务器初始化"; Pattern = "protocol_version" }
    @{ Name = "工具列表包含zhi"; Pattern = '"name":\s*"zhi"' }
    @{ Name = "工具列表包含enhance"; Pattern = '"name":\s*"enhance"' }
    @{ Name = "enhance工具调用"; Pattern = "enhanced_prompt" }
)

$passed = 0
$total = $tests.Count

foreach ($test in $tests) {
    if ($result -match $test.Pattern) {
        Write-Host "✓ $($test.Name)" -ForegroundColor Green
        $passed++
    } else {
        Write-Host "✗ $($test.Name)" -ForegroundColor Red
    }
}

Write-Host ""
Write-Host "========================================" -ForegroundColor Cyan
Write-Host "测试结果: $passed / $total 通过" -ForegroundColor $(if ($passed -eq $total) { "Green" } else { "Yellow" })
Write-Host "========================================" -ForegroundColor Cyan

# 清理
Remove-Item "test_input.txt" -ErrorAction SilentlyContinue

# 4. 手动测试指南
Write-Host ""
Write-Host "=== 手动测试指南 ===" -ForegroundColor Yellow
Write-Host ""
Write-Host "在 Claude Desktop 或 Cursor 中配置 MCP 服务器:" -ForegroundColor Cyan
Write-Host @"
{
  "mcpServers": {
    "cunzhi": {
      "command": "E:\\cursor\\cunzhi-plus\\target\\release\\寸止.exe",
      "args": []
    }
  }
}
"@
Write-Host ""
Write-Host "然后在 AI 助手中使用:" -ForegroundColor Cyan
Write-Host "1. @enhance 帮我写一个用户登录接口" -ForegroundColor White
Write-Host "2. @enhance /e 实现数据统计功能" -ForegroundColor White
Write-Host "3. @zhi 确认是否继续?" -ForegroundColor White
Write-Host ""

if ($passed -eq $total) {
    exit 0
} else {
    exit 1
}

