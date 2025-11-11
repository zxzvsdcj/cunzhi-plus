#!/usr/bin/env python3
"""Fix rmcp API breaking changes"""

import os
import re
from pathlib import Path

def fix_file(filepath):
    """修复单个文件"""
    with open(filepath, 'r', encoding='utf-8') as f:
        content = f.read()
    
    # 替换 Error as McpError
    content = content.replace('Error as McpError', 'ErrorData as McpError')
    
    # 修复 Tool 结构体初始化
    # 需要添加: title, icons, meta, output_schema
    tool_pattern = r'(Tool\s*\{[^}]*?input_schema:\s*Arc::new\(schema_map\),\s*annotations:\s*(?:Some\([^)]+\)|None),)\s*(\})'
    
    def add_fields(match):
        before = match.group(1)
        after = match.group(2)
        return f'{before}\n                icons: None,\n                meta: None,\n                output_schema: None,\n                title: None,\n            {after}'
    
    content = re.sub(tool_pattern, add_fields, content, flags=re.DOTALL)
    
    # 修复 CallToolResult
    result_pattern = r'CallToolResult\s*\{\s*content:\s*vec!\[Content::text\(([^]]+)\)\],\s*is_error:\s*(Some\([^)]+\)|None)\s*\}'
    
    def add_result_fields(match):
        text_part = match.group(1)
        error_part = match.group(2)
        return f'''CallToolResult {{ 
                content: vec![Content::text({text_part})], 
                is_error: {error_part},
                meta: None,
                structured_content: None,
            }}'''
    
    content = re.sub(result_pattern, add_result_fields, content)
    
    # 修复 Implementation
    impl_pattern = r'(Implementation\s*\{[^}]*?version:\s*[^,]+,)\s*(\})'
    
    def add_impl_fields(match):
        before = match.group(1)
        after = match.group(2)
        return f'{before}\n                title: None,\n                website_url: None,\n                icons: None,\n            {after}'
    
    content = re.sub(impl_pattern, add_impl_fields, content)
    
    with open(filepath, 'w', encoding='utf-8') as f:
        f.write(content)
    
    print(f"Fixed: {filepath}")

def main():
    """主函数"""
    src_dir = Path('src/rust/mcp')
    
    for rs_file in src_dir.rglob('*.rs'):
        fix_file(rs_file)
    
    print("All files fixed!")

if __name__ == '__main__':
    main()

