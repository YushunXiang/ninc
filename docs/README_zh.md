## ninc

[English](../README.md) | 中文

翱翔门户命令行工具。

### 登录

```bash
ninc login -u [学号/手机号/邮箱/身份证号] -p [密码] [--save]
```

指定可选选项 `--save` 后将保存用户名和密码至配置文件 `config.toml` 中。

当配置文件中存在用户名信息时可以不用指定 `-u` 参数，密码同理。

登录后凭证将保存至 `storage.json` 文件中。

### 疫情填报

```bash
ninc esrep [-y]
```

填报内容从配置文件中读取，若配置文件中不存在 `esrep.report` 则生成默认值，请务必对照[文档](./report_form.md)确保各项信息准确。

若当日已经填报过，将询问是否重复填报。指定可选选项 `-y` 后将跳过询问直接重复填报。

### 帮助

```bash
ninc -h
ninc --help
ninc help
```

你可以通过以上三种方式获得帮助信息。

例如 `ninc login -h` 和 `ninc help login` 都可以获得登录相关的帮助信息。
