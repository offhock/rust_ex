# Rust
[The Rust Programming Language](https://doc.rust-kr.org/)


## vscode
vscode는 workspace로 여는 것이 좋다. 각 프로젝트별 vscode가 제공하는 설정기능을 사용할 수 있다.
```bash
$ code .vscode/rust_ex.code-workspace
```

* 프로젝트 생성 순서
```bash
$ cargo new --name hello_world --bin 01-hello_world
$ cargo new --name guessing_game --bin 02-guessing_game
$ cargo new --name conditions --bin 03-conditions
$ cargo new --name ownership --bin 04-ownership
```

## New Project for binary
```bash
$ cargo new <프로젝트 이름> --bin
$ cargo new --name <프로젝트 이름> --bin <폴더 이름>
# 예시
$ cargo new --name conditoins --bin 03-conditions
$ tree -al 03-conditions/
03-conditions/
├── Cargo.toml
└── src
    └── main.rs

2 directories, 2 files
```
<details>
<summary>cargo new --help</summary> 

```bash
$ cargo new --help
Create a new cargo package at <path>

Usage: cargo new [OPTIONS] <PATH>

Arguments:
  <PATH>  

Options:
      --vcs <VCS>                Initialize a new repository for the given version control system, overriding a
                                 global configuration. [possible values: git, hg, pijul, fossil, none]
      --bin                      Use a binary (application) template [default]
      --lib                      Use a library template
      --edition <YEAR>           Edition to set for the crate generated [possible values: 2015, 2018, 2021, 2024]
      --name <NAME>              Set the resulting package name, defaults to the directory name
      --registry <REGISTRY>      Registry to use
  -v, --verbose...               Use verbose output (-vv very verbose/build.rs output)
  -q, --quiet                    Do not print cargo log messages
      --color <WHEN>             Coloring: auto, always, never
      --config <KEY=VALUE|PATH>  Override a configuration value
  -Z <FLAG>                      Unstable (nightly-only) flags to Cargo, see 'cargo -Z help' for details
  -h, --help                     Print help

Manifest Options:
      --locked   Assert that `Cargo.lock` will remain unchanged
      --offline  Run without accessing the network
      --frozen   Equivalent to specifying both --locked and --offline

Run `cargo help new` for more detailed information.
```
</details>

## Run
```bash
$ cargo run
```
<details>
<summary>cargo run --help</summary> 

```bash
$ cargo run --help
Run a binary or example of the local package

Usage: cargo run [OPTIONS] [ARGS]...

Arguments:cargo
  [ARGS]...  Arguments for the binary or example to run

Options:
      --message-format <FMT>     Error format
  -v, --verbose...               Use verbose output (-vv very verbose/build.rs output)
  -q, --quiet                    Do not print cargo log messages
      --color <WHEN>             Coloring: auto, always, never
      --config <KEY=VALUE|PATH>  Override a configuration value
  -Z <FLAG>                      Unstable (nightly-only) flags to Cargo, see 'cargo -Z help' for details
  -h, --help                     Print help

Package Selection:
  -p, --package [<SPEC>]  Package with the target to run

Target Selection:
      --bin [<NAME>]      Name of the bin target to run
      --example [<NAME>]  Name of the example target to run

Feature Selection:
  -F, --features <FEATURES>  Space or comma separated list of features to activate
      --all-features         Activate all available features
      --no-default-features  Do not activate the `default` feature

Compilation Options:
  -j, --jobs <N>                Number of parallel jobs, defaults to # of CPUs.
      --keep-going              Do not abort the build as soon as there is an error
  -r, --release                 Build artifacts in release mode, with optimizations
      --profile <PROFILE-NAME>  Build artifacts with the specified profile
      --target [<TRIPLE>]       Build for the target triple
      --target-dir <DIRECTORY>  Directory for all generated artifacts
      --unit-graph              Output build graph in JSON (unstable)
      --timings[=<FMTS>]        Timing output formats (unstable) (comma separated): html, json

Manifest Options:
      --manifest-path <PATH>  Path to Cargo.toml
      --lockfile-path <PATH>  Path to Cargo.lock (unstable)
      --ignore-rust-version   Ignore `rust-version` specification in packages
      --locked                Assert that `Cargo.lock` will remain unchanged
      --offline               Run without accessing the network
      --frozen                Equivalent to specifying both --locked and --offline

Run `cargo help run` for more detailed information.
```
</details>

## Check
```bash
$ cargo check
```
<details>
<summary>cargo check --help</summary> 

```bash
$ cargo check --help
Check a local package and all of its dependencies for errors

Usage: cargo check [OPTIONS]

Options:
      --future-incompat-report   Outputs a future incompatibility report at the end of the build
      --message-format <FMT>     Error format
  -v, --verbose...               Use verbose output (-vv very verbose/build.rs output)
  -q, --quiet                    Do not print cargo log messages
      --color <WHEN>             Coloring: auto, always, never
      --config <KEY=VALUE|PATH>  Override a configuration value
  -Z <FLAG>                      Unstable (nightly-only) flags to Cargo, see 'cargo -Z help' for details
  -h, --help                     Print help

Package Selection:
  -p, --package [<SPEC>]  Package(s) to check
      --workspace         Check all packages in the workspace
      --exclude <SPEC>    Exclude packages from the check
      --all               Alias for --workspace (deprecated)

Target Selection:
      --lib               Check only this package's library
      --bins              Check all binaries
      --bin [<NAME>]      Check only the specified binary
      --examples          Check all examples
      --example [<NAME>]  Check only the specified example
      --tests             Check all test targets
      --test [<NAME>]     Check only the specified test target
      --benches           Check all bench targets
      --bench [<NAME>]    Check only the specified bench target
      --all-targets       Check all targets

Feature Selection:
  -F, --features <FEATURES>  Space or comma separated list of features to activate
      --all-features         Activate all available features
      --no-default-features  Do not activate the `default` feature

Compilation Options:
  -j, --jobs <N>                Number of parallel jobs, defaults to # of CPUs.
      --keep-going              Do not abort the build as soon as there is an error
  -r, --release                 Check artifacts in release mode, with optimizations
      --profile <PROFILE-NAME>  Check artifacts with the specified profile
      --target [<TRIPLE>]       Check for the target triple
      --target-dir <DIRECTORY>  Directory for all generated artifacts
      --unit-graph              Output build graph in JSON (unstable)
      --timings[=<FMTS>]        Timing output formats (unstable) (comma separated): html, json

Manifest Options:
      --manifest-path <PATH>  Path to Cargo.toml
      --lockfile-path <PATH>  Path to Cargo.lock (unstable)
      --ignore-rust-version   Ignore `rust-version` specification in packages
      --locked                Assert that `Cargo.lock` will remain unchanged
      --offline               Run without accessing the network
      --frozen                Equivalent to specifying both --locked and --offline

Run `cargo help check` for more detailed information.
```
</details>

## Debug Build
```bash
$ cargo build
```
<details>
<summary>cargo build --help</summary> 

```bash
$ cargo build --help
Compile a local package and all of its dependencies

Usage: cargo build [OPTIONS]

Options:
      --future-incompat-report   Outputs a future incompatibility report at the end of the build
      --message-format <FMT>     Error format
  -v, --verbose...               Use verbose output (-vv very verbose/build.rs output)
  -q, --quiet                    Do not print cargo log messages
      --color <WHEN>             Coloring: auto, always, never
      --config <KEY=VALUE|PATH>  Override a configuration value
  -Z <FLAG>                      Unstable (nightly-only) flags to Cargo, see 'cargo -Z help' for details
  -h, --help                     Print help

Package Selection:
  -p, --package [<SPEC>]  Package to build (see `cargo help pkgid`)
      --workspace         Build all packages in the workspace
      --exclude <SPEC>    Exclude packages from the build
      --all               Alias for --workspace (deprecated)

Target Selection:
      --lib               Build only this package's library
      --bins              Build all binaries
      --bin [<NAME>]      Build only the specified binary
      --examples          Build all examples
      --example [<NAME>]  Build only the specified example
      --tests             Build all test targets
      --test [<NAME>]     Build only the specified test target
      --benches           Build all bench targets
      --bench [<NAME>]    Build only the specified bench target
      --all-targets       Build all targets

Feature Selection:
  -F, --features <FEATURES>  Space or comma separated list of features to activate
      --all-features         Activate all available features
      --no-default-features  Do not activate the `default` feature

Compilation Options:
  -r, --release                 Build artifacts in release mode, with optimizations
      --profile <PROFILE-NAME>  Build artifacts with the specified profile
  -j, --jobs <N>                Number of parallel jobs, defaults to # of CPUs.
      --keep-going              Do not abort the build as soon as there is an error
      --target [<TRIPLE>]       Build for the target triple
      --target-dir <DIRECTORY>  Directory for all generated artifacts
      --artifact-dir <PATH>     Copy final artifacts to this directory (unstable)
      --build-plan              Output the build plan in JSON (unstable)
      --unit-graph              Output build graph in JSON (unstable)
      --timings[=<FMTS>]        Timing output formats (unstable) (comma separated): html, json

Manifest Options:
      --manifest-path <PATH>  Path to Cargo.toml
      --lockfile-path <PATH>  Path to Cargo.lock (unstable)
      --ignore-rust-version   Ignore `rust-version` specification in packages
      --locked                Assert that `Cargo.lock` will remain unchanged
      --offline               Run without accessing the network
      --frozen                Equivalent to specifying both --locked and --offline

Run `cargo help build` for more detailed information.
```
</details>

## Release Build
```bash
$ cargo build --release
```

# Rust의 소유권 이해
* 기본적인 lvalue = rvalue 대입은 move 연산이다.
* stack에 저장 가능하는 데이터의 대입연산 copy 연산이다.
* 기본이 move이므로 let을 이용하여 재선언하면 소유권이 이전된다.
* 함수에 값을 전달하는 것도 소유권을 전달하는 방식과 동일하게 작동한다.
* 값의 반환도 소유권을 이동시킨다. (여러값을 반환할때는 튜플을 사용한다.)
* 참조자를 사용할 경우에는 다음의 둘중 한 경우만 가능하다.
  * 가변참조자 1개만 갖는다.
  * 불변참조자 여러개를 가진다.
* 참조자는 항상 유효해야 한다. (러스트는 댕글링 참조가 되지는 않는다.)
* 슬라이스를 통해서 컬렉센 전체가 아닌 컬렉션의 연속된 일련의 요소를 참조할 수 있다.
* 슬라이스를 가변참조자로 사용 중일 때 std::io등의 라이브러리가 가변참조자로 접근할 수 없다.
  * 이는 가변참조자는 항상 1개만 가질 수 있기 때문에 std등의 라이브러리가 가변참조자로 데이터를 접근할 경우 컴파일타임때 에러가 발생한다.
* 슬라이스를 불변참조자로 사용 중일때 std등의 라이브러리에서 가변참조자로 접근할 수 없다.
  * 이는 불변참조자는 가변참조자와 함께 사용하지 못하는 규칙이 있으므로 컴파일타임때 확인이 가능하다.

```rust
fn main() {
    let mut s = String::from("hello");

    let r1 = &s;      // 불변 참조자
    let r2 = &mut s;  // 가변 참조자

    println!("{}, {}", r1, r2);
}
--> 에러발생
```