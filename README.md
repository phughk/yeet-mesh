# Yeet Mesh

Yeet Mesh is a way to run distributed systems tests in a deterministic way

## Motivation

Provide a non-invasive, test-env aware framework that is easy to enable and disable for tests and provides a lot of
convenient access methods.

## Why not...?

### Madsim

[Madsim](https://github.com/madsim-rs/madsim) has a weird configuration which requires you to have a madsim rustflag,
consequently it doesn't get picked up by
IDEs
and in general is a bit inconvenient.

Beyond that, it doesn't actually offer that much in terms of customisability. It is simply a way to wire things
together, and it does that in a really inconvenient way.

### Tungstenite

Support for [Tungstenite](https://github.com/snapview/tungstenite-rs) seems to be waning, with most new projects using
Madsim instead.

### Jepsen

[Jepsen](https://github.com/jepsen-io/jepsen) is great, but it's integration requiring deployments and is clojure.

### Antithesis

[Antithesis](https://antithesis.com/) is a great commercial product that is worth having alongside this.
Yeet Mesh is for fast unit-test-like performance of cluster tests.
Antithesis is a general purpose VM-style way of running tests.
Antithesis is certainly more powerful and practical for catching edge cases.

## How does Yeet-Mesh improve on the existing libraries/frameworks

### Doesn't replace async runtimes

You can use whatever runtime you want, as long as your code uses the same Runtime trait for the async runtime
interaction.
To add support for a custom async runtime, implement the Yeet Mesh Runtime trait.
A tokio implementation will be provided.

### Test-wide convenient property checks like invariants

If you want to confirm a property is true across a cluster at any point in time, you can use property checking on each
test, such as invariants or ordering guarantees.

# How to use

See the [HOW-TO.md](HOW-TO.md) for more information on how to use Yeet Mesh.


