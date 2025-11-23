<div align="center">
  <h1>LLMAO</h1>
  <h6>Large Language Model API Ops</h6>
</div>

## Intro

`llmao` is an abstraction layer for interfacing with different LLM providers.

The idea is that `llmao` creates a unified and generic way of communicating
between LLM providers. It defines the traits and types but does not include
provider implementations, you must bring your own provider client and implement
these traits for it. With that being said, if you just want high-level crates
that already implement provider specific protocols, see
[high level crates](#high-level-crates).

## Design Goals

`llmao` is heavily inspired by the amazing
[embedded-hal](https://docs.rs/embedded-hal/latest/embedded_hal/) crate, and so
therefore will emulate the same sort of mental model as well as share similar
design goals.

- Async by default (for now), since these requests are likely going over the
  wire using HTTPS, async will be the default implementation for these traits.
  Ideally, in the future, I'd like to add blocking variants for the async
  functions.
- Where possible must not be tied to a specific asynchronous model. This mean it
  should work regardless of async runtimes
  ([tokio](https://github.com/tokio-rs/tokio),
  [async-std](https://github.com/async-rs/async-std),
  [smol](https://github.com/smol-rs/smol)).
- Must be minimal, and thus easy to implement and zero cost, yet highly
  composable.
- Serve as a foundation for creating an ecosystem of provider-agnostic LLM
  operations. An operation in this case, simply means calling an API endpoint.
- Trait methods must be fallible so that they can be used in any possible
  situation.
- Must erase provider-specific details. No OpenAI parameters, Claude-specific
  formatting, or provider endpoints that would leak into this trait API.
- Must use associated types for errors, allowing each provider implementation to
  define its own error type without forcing allocation or type erasure.
- Schema/structured output support should use the provider's native capabilities
  (JSON schema parameter).

## Out of Scope

- Agentic specific implementations
- Model Context Protocol (MCP) specific implementations

## High Level Crates

- [0xPlaygrounds/rig](https://github.com/0xplaygrounds/rig) - `rig-core`
  implements their own generic trait system that allows you to build and
  configure their suppported clients.
