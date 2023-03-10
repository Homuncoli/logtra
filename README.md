# Logtra

[![Publish](https://github.com/Homuncoli/logtra/actions/workflows/publish.yml/badge.svg?branch=master)](https://github.com/Homuncoli/logtra/actions/workflows/publish.yml)
[![Version](https://github.com/Homuncoli/logtra/actions/workflows/version.yml/badge.svg?branch=master)](https://github.com/Homuncoli/logtra/actions/workflows/version.yml)
[![Lint, Build and Test](https://github.com/Homuncoli/logtra/actions/workflows/build-test.yml/badge.svg?branch=master)](https://github.com/Homuncoli/logtra/actions/workflows/build-test.yml)

logtra is a logging library for Rust.

## Features
- [ ] Sinks
    - [x] Register a sink
    - [ ] Unregister a sink
- [ ] Log
  - [ ] Formatting
    - [x] *t*imestamp
    - [x] *c*urrent ThreadId
    - [x] *m*odule
    - [x] *s*everity
    - [ ] *n*ame of sink 
    - [x] *f*ile
    - [x] *l*ine 
    - [x] *c*olor
    - [x] *m*essage 
  - [ ] Macro
    - [x] Different Log Intensities
    - [x] Expressions/Evaluations
    - [ ] Asserts
      - [x] Evaluating asserts (assert) 
      - [ ] Conditional logs (cassert)
      - [ ] Panicking asserts (passert)

## Usage
logtra is almost entirely macro based.

## Example
```rust
fn main() {
    sink!(
        ConsoleSink::new(
            SinkDeclaration {
                name: "console".to_string(),
                severity: Logseverity::Trace,
                module: "*".to_string(),
                template: "[%t][%c][%[%i%]][%s][%f:%l]: %m\n".to_string(),
            }
        )
    )

    trace!("Hello World: Trace!");
    debug!("Hello World: Debug!");
    info!("Hello World: Info!");
    warn!("Hello World: Warn!");
    error!("Hello World: Error!");
    fatal!("Hello World: Fatal!");
    log!(Info, &obj);
}
```