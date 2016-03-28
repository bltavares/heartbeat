# hearbeat

`top` for services


## Goals

- [ ] Ping an url every X seconds
- [ ] Match the body against a fixed string
- [ ] Collect metrics of time that the service was down over the running time
- [ ] (Maybe) A graph view
- [ ] Write to file so it could be visualized later

## How to develop

After checking out the code, you can modify and test the changes like the following:

```
cargo run -- --help
cargo run -- http://example.com
```

## Unresolved and documented flaws

- [ ] Does not work with redirect
