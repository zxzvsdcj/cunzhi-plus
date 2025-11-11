#!/bin/bash
# 提示词增强工具测试脚本

set -e

echo "=== 寸止提示词增强功能测试 ==="
echo ""

# 颜色定义
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

# 测试计数
TOTAL_TESTS=0
PASSED_TESTS=0

# 测试函数
run_test() {
    local test_name=$1
    local test_cmd=$2
    
    TOTAL_TESTS=$((TOTAL_TESTS + 1))
    echo -e "${BLUE}测试 $TOTAL_TESTS: $test_name${NC}"
    
    if eval "$test_cmd"; then
        echo -e "${GREEN}✓ 通过${NC}\n"
        PASSED_TESTS=$((PASSED_TESTS + 1))
        return 0
    else
        echo -e "${RED}✗ 失败${NC}\n"
        return 1
    fi
}

# 1. 编译项目
echo -e "${YELLOW}步骤 1: 编译项目${NC}"
echo "正在编译 Rust 项目..."
cargo build --release 2>&1 | tail -n 20
echo -e "${GREEN}✓ 编译完成${NC}\n"

# 2. 测试基础增强功能
echo -e "${YELLOW}步骤 2: 测试基础增强功能${NC}"

# 创建测试请求文件
cat > /tmp/test_enhance_basic.json <<EOF
{
    "jsonrpc": "2.0",
    "id": 1,
    "method": "tools/call",
    "params": {
        "name": "enhance",
        "arguments": {
            "prompt": "/e 帮我写一个用户登录接口",
            "enable_pipeline": false,
            "enable_scoring": false
        }
    }
}
EOF

run_test "基础提示词增强" \
    "echo '{\"jsonrpc\":\"2.0\",\"id\":1,\"method\":\"tools/call\",\"params\":{\"name\":\"enhance\",\"arguments\":{\"prompt\":\"/e 帮我写一个用户登录接口\",\"enable_pipeline\":false,\"enable_scoring\":false}}}' | ./target/release/寸止 2>&1 | grep -q 'enhanced_prompt'"

# 3. 测试四阶管线
echo -e "${YELLOW}步骤 3: 测试四阶管线${NC}"

run_test "四阶管线启用" \
    "echo '{\"jsonrpc\":\"2.0\",\"id\":2,\"method\":\"tools/call\",\"params\":{\"name\":\"enhance\",\"arguments\":{\"prompt\":\"实现一个数据统计功能\",\"enable_pipeline\":true,\"enable_scoring\":false}}}' | ./target/release/寸止 2>&1 | grep -q '需求分析'"

# 4. 测试评分闭环
echo -e "${YELLOW}步骤 4: 测试评分闭环${NC}"

run_test "评分闭环启用" \
    "echo '{\"jsonrpc\":\"2.0\",\"id\":3,\"method\":\"tools/call\",\"params\":{\"name\":\"enhance\",\"arguments\":{\"prompt\":\"写一个抽奖接口\",\"enable_pipeline\":true,\"enable_scoring\":true,\"target_score\":90}}}' | ./target/release/寸止 2>&1 | grep -q '代码生成'"

# 5. 测试工具列表
echo -e "${YELLOW}步骤 5: 测试工具注册${NC}"

run_test "enhance工具已注册" \
    "echo '{\"jsonrpc\":\"2.0\",\"id\":4,\"method\":\"tools/list\"}' | ./target/release/寸止 2>&1 | grep -q 'enhance'"

# 6. 测试与zhi工具的集成
echo -e "${YELLOW}步骤 6: 测试与寸止工具集成${NC}"

run_test "寸止工具仍可用" \
    "echo '{\"jsonrpc\":\"2.0\",\"id\":5,\"method\":\"tools/call\",\"params\":{\"name\":\"zhi\",\"arguments\":{\"message\":\"测试消息\"}}}' | ./target/release/寸止 2>&1 | grep -q 'user_input'"

# 7. 综合测试
echo -e "${YELLOW}步骤 7: 综合功能测试${NC}"

# 创建完整测试场景
cat > /tmp/test_enhance_full.json <<EOF
{
    "jsonrpc": "2.0",
    "id": 6,
    "method": "tools/call",
    "params": {
        "name": "enhance",
        "arguments": {
            "prompt": "实现一个用户认证系统，需要支持多种登录方式",
            "images": [],
            "enable_pipeline": true,
            "enable_scoring": true,
            "target_score": 85
        }
    }
}
EOF

run_test "完整功能流程" \
    "cat /tmp/test_enhance_full.json | ./target/release/寸止 2>&1 | grep -q 'metadata'"

# 测试总结
echo ""
echo "========================================"
echo -e "${BLUE}测试总结${NC}"
echo "========================================"
echo -e "总测试数: $TOTAL_TESTS"
echo -e "${GREEN}通过: $PASSED_TESTS${NC}"
echo -e "${RED}失败: $((TOTAL_TESTS - PASSED_TESTS))${NC}"
echo ""

if [ $PASSED_TESTS -eq $TOTAL_TESTS ]; then
    echo -e "${GREEN}✓ 所有测试通过！${NC}"
    exit 0
else
    echo -e "${RED}✗ 部分测试失败${NC}"
    exit 1
fi

