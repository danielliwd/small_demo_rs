# commandline_options_and_config

## run demo

```bash
cargo run -- -d -t -c example.yml -c example2.yml
```

will produce output:

```txt
conf: Config { version: 1, daemon: true,  override:2 }
opt: Opt { daemon: true, test: true, conf: ["example.yml", "example2.yml"] }
```

multiple configs will override by key. This feature is useful for `.env.yml`.

## how to copy this repo

1. copy `configurations` to you project
2. run command

   ```bash
   cargo add configurations --path ./configurations
   ```

3. modify configurations for your project
