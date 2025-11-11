# 简单测试脚本 - 只测试初始化和工具列表

Write-Host "=== 简单MCP测试 ===" -ForegroundColor Cyan

# 创建测试输入
$input = @"
{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2024-11-05","capabilities":{},"clientInfo":{"name":"test","version":"1.0"}}}
{"jsonrpc":"2.0","method":"notifications/initialized"}
{"jsonrpc":"2.0","id":2,"method":"tools/list"}
"@

Write-Host "输入:" -ForegroundColor Yellow
$input

Write-Host "`n发送中..." -ForegroundColor Cyan

# 使用临时文件
$input | Out-File -FilePath "test.jsonl" -Encoding UTF8 -NoNewline
$output = Get-Content "test.jsonl" | .\target\release\寸止.exe 2>&1

Write-Host "`n完整输出:" -ForegroundColor Green
$output | ForEach-Object { Write-Host $_ }

Remove-Item "test.jsonl" -ErrorAction SilentlyContinue

Write-Host "`n分析:" -ForegroundColor Yellow
$jsonLines = $output | Where-Object { $_ -match '^\{' }
Write-Host "JSON响应数: $($jsonLines.Count)" -ForegroundColor Cyan

foreach ($line in $jsonLines) {
    try {
        $json = $line | ConvertFrom-Json
        Write-Host "`nJSON ID: $($json.id)" -ForegroundColor Magenta
        if ($json.result.tools) {
            Write-Host "工具列表:" -ForegroundColor Green
            $json.result.tools | ForEach-Object {
                Write-Host "  - $($_.name): $($_.description)" -ForegroundColor White
            }
        }
    }
    catch {
        Write-Host "解析失败: $line" -ForegroundColor Red
    }
}

