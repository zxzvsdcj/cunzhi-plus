# 提示词增强工具测试脚本 (PowerShell)

$ErrorActionPreference = "Stop"

Write-Host "=== 寸止提示词增强功能测试 ===" -ForegroundColor Cyan
Write-Host ""

# 测试计数
$TotalTests = 0
$PassedTests = 0

# 测试函数
function Run-Test {
    param(
        [string]$TestName,
        [scriptblock]$TestCommand
    )
    
    $script:TotalTests++
    Write-Host "测试 $TotalTests : $TestName" -ForegroundColor Blue
    
    try {
        & $TestCommand
        Write-Host "✓ 通过" -ForegroundColor Green
        Write-Host ""
        $script:PassedTests++
        return $true
    }
    catch {
        Write-Host "✗ 失败: $_" -ForegroundColor Red
        Write-Host ""
        return $false
    }
}

# 1. 编译项目
Write-Host "步骤 1: 编译项目" -ForegroundColor Yellow
Write-Host "正在编译 Rust 项目..."
cargo build --release 2>&1 | Select-Object -Last 20
Write-Host "✓ 编译完成" -ForegroundColor Green
Write-Host ""

# 2. 测试基础增强功能
Write-Host "步骤 2: 测试基础增强功能" -ForegroundColor Yellow

Run-Test "基础提示词增强" {
    $json = @'
{"jsonrpc":"2.0","id":1,"method":"tools/call","params":{"name":"enhance","arguments":{"prompt":"/e 帮我写一个用户登录接口","enable_pipeline":false,"enable_scoring":false}}}
'@
    $result = $json | .\target\release\寸止.exe 2>&1 | Out-String
    if ($result -notmatch "enhanced_prompt") {
        throw "未找到enhanced_prompt"
    }
}

# 3. 测试四阶管线
Write-Host "步骤 3: 测试四阶管线" -ForegroundColor Yellow

Run-Test "四阶管线启用" {
    $json = @'
{"jsonrpc":"2.0","id":2,"method":"tools/call","params":{"name":"enhance","arguments":{"prompt":"实现一个数据统计功能","enable_pipeline":true,"enable_scoring":false}}}
'@
    $result = $json | .\target\release\寸止.exe 2>&1 | Out-String
    if ($result -notmatch "需求分析") {
        throw "未找到需求分析"
    }
}

# 4. 测试评分闭环
Write-Host "步骤 4: 测试评分闭环" -ForegroundColor Yellow

Run-Test "评分闭环启用" {
    $json = @'
{"jsonrpc":"2.0","id":3,"method":"tools/call","params":{"name":"enhance","arguments":{"prompt":"写一个抽奖接口","enable_pipeline":true,"enable_scoring":true,"target_score":90}}}
'@
    $result = $json | .\target\release\寸止.exe 2>&1 | Out-String
    if ($result -notmatch "代码生成") {
        throw "未找到代码生成"
    }
}

# 5. 测试工具列表
Write-Host "步骤 5: 测试工具注册" -ForegroundColor Yellow

Run-Test "enhance工具已注册" {
    $json = '{"jsonrpc":"2.0","id":4,"method":"tools/list"}'
    $result = $json | .\target\release\寸止.exe 2>&1 | Out-String
    if ($result -notmatch "enhance") {
        throw "enhance工具未注册"
    }
}

# 6. 测试与zhi工具的集成
Write-Host "步骤 6: 测试与寸止工具集成" -ForegroundColor Yellow

Run-Test "寸止工具仍可用" {
    $json = '{"jsonrpc":"2.0","id":5,"method":"tools/call","params":{"name":"zhi","arguments":{"message":"测试消息"}}}'
    $result = $json | .\target\release\寸止.exe 2>&1 | Out-String
    if ($result -notmatch "user_input") {
        throw "寸止工具不可用"
    }
}

# 7. 综合测试
Write-Host "步骤 7: 综合功能测试" -ForegroundColor Yellow

Run-Test "完整功能流程" {
    $json = @'
{"jsonrpc":"2.0","id":6,"method":"tools/call","params":{"name":"enhance","arguments":{"prompt":"实现一个用户认证系统，需要支持多种登录方式","images":[],"enable_pipeline":true,"enable_scoring":true,"target_score":85}}}
'@
    $result = $json | .\target\release\寸止.exe 2>&1 | Out-String
    if ($result -notmatch "metadata") {
        throw "未找到metadata"
    }
}

# 测试总结
Write-Host ""
Write-Host "========================================" -ForegroundColor Cyan
Write-Host "测试总结" -ForegroundColor Blue
Write-Host "========================================" -ForegroundColor Cyan
Write-Host "总测试数: $TotalTests"
Write-Host "通过: $PassedTests" -ForegroundColor Green
Write-Host "失败: $($TotalTests - $PassedTests)" -ForegroundColor Red
Write-Host ""

if ($PassedTests -eq $TotalTests) {
    Write-Host "✓ 所有测试通过！" -ForegroundColor Green
    exit 0
}
else {
    Write-Host "✗ 部分测试失败" -ForegroundColor Red
    exit 1
}

