# 🥗 准备工作
## 开发目标
grep （缩写来自Globally search a Regular Expression and Print），Unix/Linux 下鼎鼎大名的文本搜索工具，它能使用特定模式匹配（包括正则表达式）搜索文本，并默认输出匹配行。

我们这一章就山寨下 grep 的功能，基于 rust 实现简易版的文本搜索，基本语法：`grep.exe <PATTERN> <PATH>`，支持额外项：

* **-c**：统计目标词出现次数
* **-n**：显示目标词出现的行及对应的行号（默认开启）

使用示例：
```shell
# 显示目标词在文件中出现的行及其行号
./grep.exe 集成显卡 README.md
# 统计“集成显卡”出现的次数
./grep.exe -c 集成显卡 README.md
```
