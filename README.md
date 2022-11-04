campus_network_login
===

campus_network_login是用于校园网认证的程序，支持扬州大学等

## 使用

点击右侧Releases下载一个最新的发布并双击

### 如何获取IP

![img_to_get](https://raw.github.com/abgelehnt/campus_network_login/main/img.png)

## 进阶使用

命令行输入`campus_network_login.exe -e`修改配置信息

命令行输入`campus_network_login.exe -v`获取日志

### 配置文件格式

用户的配置信息保存在`%AppData%/Chi/CampusNetworkLogin/config/config.json`文件中

```
{
"user_id": "用户学号",
"password": "用户密码（支持加密形式）",
"ip": "登录网站的IP",
"service": "使用的运营商"
}
```
