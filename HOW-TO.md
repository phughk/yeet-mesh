# Installation

```sh
cargo add yeet-mesh
cargo add yeet-mesh --features=test-mode --dev
```

# Usage in service

```rust
pub fn main() {
    // We initialise a real runtime with real access to IO
    let runtime: Runtime = new_runtime();
    // Then we initialise our app - you can configure and initialise here as necessary
    let app = App::new(runtime);
    // Then we start the app - during tests, the harness does this for us
    app.main_testable(runtime);
}

// Our App has state, which can be reported to the runtime during tests
struct App {}

impl App {
    pub fn main_testable(runtime: Runtime) -> (Future<>) {}

    async fn main(&self) {
        for i in 0..10 {}
    }

    // State should be synchronous - using locks is discouraged since it may block in the async runtime
    // Instead, you should use channels and collect the state measured
    pub fn get_state(&self) -> State {}
}

```
