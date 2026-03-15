# 测试数据文件

此目录存放 E2E 测试使用的测试数据文件。

## 文件说明

| 文件 | 用途 |
|-----|------|
| test.docx | Word 文档测试数据 |
| test.pdf | PDF 文档测试数据 |
| test.xlsx | Excel 表格测试数据 |
| test.txt | 纯文本测试数据 |

## 生成测试文件

可以使用以下方法生成测试文件：

### Word 文档 (test.docx)
使用 python-docx 或手动创建：
```python
from docx import Document
doc = Document()
doc.add_heading('测试文档', 0)
doc.add_paragraph('姓名：张三')
doc.add_paragraph('电话：13800138000')
doc.add_paragraph('身份证：110101199001011234')
doc.save('test.docx')
```

### Excel 表格 (test.xlsx)
使用 openpyxl 或手动创建：
```python
from openpyxl import Workbook
wb = Workbook()
ws = wb.active
ws['A1'] = '姓名'
ws['B1'] = '电话'
ws['A2'] = '张三'
ws['B2'] = '13800138000'
wb.save('test.xlsx')
```

### PDF 文档 (test.pdf)
使用 reportlab 或手动创建

### 纯文本 (test.txt)
```
姓名：张三
电话：13800138000
身份证：110101199001011234
邮箱：test@example.com
IP地址：192.168.1.100
```

## 注意事项

- 测试数据应包含可被脱敏规则识别的敏感信息
- 文件大小不宜过大，影响测试速度
- 定期更新测试数据以覆盖更多场景
