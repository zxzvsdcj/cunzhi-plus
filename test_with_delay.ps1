# 带延迟的测试脚本

Write-Host "=== MCP测试(带延迟) ===" -ForegroundColor Cyan

# 分步发送
$requests = @(
    '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2024-11-05","capabilities":{},"clientInfo":{"name":"test","version":"1.0"}}}'
    '{"jsonrpc":"2.0","method":"notifications/initialized"}'
    '{"jsonrpc":"2.0","id":2,"method":"tools/list"}'
    '{"jsonrpc":"2.0","id":3,"method":"tools/call","params":{"name":"enhance","arguments":{"prompt":"test","enable_pipeline":false,"enable_scoring":false}}}'
)

$allRequests = $requests -join "`n"
Write-Host "发送所有请求..." -ForegroundColor Yellow
$allRequests | Out-File -FilePath "test_all.jsonl" -Encoding UTF8 -NoNewline

# 启动服务器并捕获输出
$process = Start-Process -FilePath ".\target\release\寸止.exe" `
    -RedirectStandardInput "test_all.jsonl" `
    -RedirectStandardOutput "test_output.txt" `
    -RedirectStandardError "test_error.txt" `
    -NoNewWindow `
    -PassThru `
    -Wait

Write-Host "`n=== 标准输出 ===" -ForegroundColor Green
Get-Content "test_output.txt" | Write-Host

Write-Host "`n=== 错误输出 ===" -ForegroundColor Red
Get-Content "test_error.txt" | Write-Host

Write-Host "`n=== 分析工具 ===" -ForegroundColor Cyan
$stdout = Get-Content "test_output.txt" -Raw

# 检查 enhance 工具
if ($stdout -match '"name":\s*"enhance"') {
    Write-Host "✓ enhance 工具已注册!" -ForegroundColor Green
    
    # 提取工具描述
    if ($stdout -match '"name":\s*"enhance"[^}]*"description":\s*"([^"]*)"') {
        Write-Host "  描述: $($matches[1])" -ForegroundColor White
    }
} else {
    Write-Host "✗ enhance 工具未找到" -ForegroundColor Red
}

# 检查 zhi 工具
if ($stdout -match '"name":\s*"zhi"') {
    Write-Host "✓ zhi 工具已注册!" -ForegroundColor Green
} else {
    Write-Host "✗ zhi 工具未找到" -ForegroundColor Red
}

# 清理
Remove-Item "test_all.jsonl", "test_output.txt", "test_error.txt" -ErrorAction SilentlyContinue

