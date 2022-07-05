## ninc

English | [中文](./docs/README_zh.md)

A command line interface for NWPU ecampus.

### Sign in

```bash
ninc login -u [username] -p [password] [--save]
```

After specifying the option `--save`, the username and password will be saved to the file `config.toml`.

When the username already exists in the config file, it is not necessary to specify the `-u` option, and it is same for the password.

After sign in, the token will be saved to the file `storage.json`.

### Epidemic situation report

```bash
ninc esrep [-y]
```

The content to be reported will be read from the config file. If option `esrep.report` does not exist in the config file, a default value will be generated. Please make sure that all information is accurate by comparing with the [document](./docs/report_form.md).

If it has been reported today, you will be asked whether to repeat it. After specifying the option `-y`, the query will be skipped and the filling will be repeated directly.

### Help

```bash
ninc -h
ninc --help
ninc help
```

You can get help information in the above three ways.

For example, both `ninc login -h` and `ninc help login` can get help information related to login.
