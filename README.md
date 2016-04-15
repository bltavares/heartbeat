# hearbeat

`top` for services

## Current UI

```
$ heartbeat --interval 1 http://localhost:7125
Total
Requests: 12 - Success: 12/100.0% - Failure: 0/0.0%

Last requests
http://localhost:7125/ -> Status: 200 OK, Response Time: PT0.001393750S
http://localhost:7125/ -> Status: 200 OK, Response Time: PT0.001327098S
http://localhost:7125/ -> Status: 200 OK, Response Time: PT0.001502680S
http://localhost:7125/ -> Status: 200 OK, Response Time: PT0.001075134S
http://localhost:7125/ -> Status: 200 OK, Response Time: PT0.001465644S
http://localhost:7125/ -> Status: 200 OK, Response Time: PT0.001142257S
http://localhost:7125/ -> Status: 200 OK, Response Time: PT0.001584939S
http://localhost:7125/ -> Status: 200 OK, Response Time: PT0.001471405S
http://localhost:7125/ -> Status: 200 OK, Response Time: PT0.001446609S
http://localhost:7125/ -> Status: 200 OK, Response Time: PT0.001394655S
```

## Goals

- [x] Ping an url every X seconds
- [x] Collect metrics of time that the service was down over the running time
- [ ] Match the body against a fixed string
- [ ] (Maybe) A graph view
- [ ] Write to file so it could be visualized later

## Installation

### Mac

You can use [homebrew](http://brew.sh/) to install it as a package:

```
brew tap bltavares/tap
brew install heartbeat
```

There are pre-compiled binaries available on the [Release page](https://github.com/bltavares/heartbeat/releases) as well.

### Linux

Download and unpack the latest version from the [Release page](https://github.com/bltavares/heartbeat/releases).

### Manually

Currently you need [Rust](https://www.rust-lang.org/) stable toolchain installed to produce a binary.

To produce a release binary, execute:

```bash
cargo build --release
```

You then can copy the binary to your PATH:

```bash
cp target/release/heartbeat /usr/local/bin
```

## How to develop

After checking out the code, you can modify and test the changes like the following:

```bash
cargo test # will execute some unit tests of the structures
cargo run -- --help
cargo run -- http://example.com
```

The repository also contains a Ruby interactive server where you can modify the response code and speed using a REPL.

You can start it using:

```bash
ruby -W0 test-server/server.rb
```

And from another terminal you may point `heartbeat` to it.

```bash
cargo run -- --interval 0 http://localhost:7125
```

While it is running, you can change the response using some provided methods [[docs](./test-server/README.md)]

## Unresolved and documented flaws

## Roadmap

- Wave 1: Stabilization
- Wave 2: More metrics calculated
- Wave 3: Multiple endpoints
- Wave 4: Response checks
- Wave 5: Pause/continue and output to file
- Wave 6: (Maybe?) interactive UI
