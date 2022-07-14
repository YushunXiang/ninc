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

### Campus card

```bash
ninc ecard
```

Show balance, monthly consumption and recent consumption records.

```bash
ninc -d [--begin [date]] [--end [date]] [--limit [max_display_count]]
```

Show consumption records in the specified date range.

Date format is `YYYY-MM-DD`. If not specified `--limit`, it will be 10, if not specified `--end`, it will be today, if not specified `--begin`, it will be 30 days ago.

### Help

```bash
ninc -h
ninc --help
ninc help
```

You can get help information in the above three ways.

For example, both `ninc login -h` and `ninc help login` can get help information related to login.
