# yeet-mesh
Yeet mesh is a way to run distributed systems tests in a deterministic way

## Motivation

Provide a non-invasive, test-env aware framework that is easy to enable and disable for tests and provides a lot of convenient access methods.

## Why not...?

### Madsim

It has a weird configuration which requires you to have a madsim rustflag, consequently it doesn't get picked up by IDEs and in general is a bit inconveninent.

Beyond that, it doesn't actually offer that much in terms of customisability. It is simply a way to wire things together, and it does that in a really inconvenient way.

### Tungstenite

Support for it seems to be waning, with most new projects using Madsim instead.

### Jepsen

Jepsen is great, but it's integration requiring deployments and is clojure.

### Antithesis

It's a great commercial product that is worth having alongside this.
This is for fast unit-test-like performance of cluster tests.

## How does Yeet-Mesh improve on the existing libraries/frameworks

### Doesn't replace async runtimes

You can use whatever runtime you want, as long as your code uses the same trait for the async runtime interaction.

### Test-wide convenient property checks like invariants

If you want to confirm a property is true across a cluster at any point in time, you can use property checking on each test, such as invariants or ordering guarantees.


