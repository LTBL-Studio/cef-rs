# cef-rs

Use CEF in Rust.

## Supported Targets

| Target | Linux | macOS | Windows |
| ------ | ----- | ----- | ------- |
| x86_64 | ✅    | ❎     | ✅      |
| ARM64  | ✅    | ❎     | ❎      |

## Usage

### Linux

#### Install

- execute `cef_install.sh` 
```
cef_install.sh 124.3.8
```

- Build and run the application

```
cargo run --example demo
```

## Contributing

Please see [CONTRIBUTING.md](CONTRIBUTING.md) for details.

## Roadmap

Cef-rs is looking for the best approach to use CEF. On Linux, this is achieved by using same local share directory.
So every application can share the same library. We are looking for the similar methods on macOS and Windows.
Welcome to open feature requests if the feature you look for isn't listed below.
But please understand that some requests might result into not planned.

### Planned

- [x] Add Linux ARM64 target
- [ ] Add macOS ARM64 target
- [x] Add Windows x86_64 target

### Not Planned

- Other package formats on Linux.
- Add all possible ergonomic interfaces (ie. builder types for attributes and settings.)
- Provide tools to bundle and distribute application.

