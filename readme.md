# 功能
- 传入markdown文件
- 打开浏览器，在网页中进行各种操作。
- 读取功能
- 保存功能
- 能作为md文件的默认打开方式
# 用法
```bash
./editor /home/xxx/文档/abc.md
```
# 高级用法
首先，运行指令生成单个可执行文件。
```bash
cargo build
```
然后，在 ***/home/xxx/.local/share/applications***/ 下新建desktop文件。
```bash
[Desktop Entry]
Name=Editor
Exec=/home/xxx/文档/editor %f
Terminal=true
Type=Application
MimeType=text/markdown;
```
最后，右键md打开方式，选择自己的app即可。

![这是图片](/docs/image.png "Magic Gardens")