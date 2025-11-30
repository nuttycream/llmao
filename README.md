<h1>llmao</h1>
<h6>Large Language Model Abstracted Operations</h6>

---

`llmao` is an abstraction layer for interfacing with different LLM providers.

The idea is that `llmao` creates a unified and generic way of communicating
between LLM providers. It defines the traits and types but does not include
provider implementations, you must bring your own provider client and implement
these traits for it.

## Design Goals

`llmao` is heavily inspired by the amazing
[embedded-hal](https://docs.rs/embedded-hal/latest/embedded_hal/) crate, and so
therefore will emulate the same sort of mental model as well as share similar
design goals.

`llmao` shall

- Serve as a foundation for creating an ecosystem of provider-agnostic LLM
  operations. An operation in this case, simply means calling an API endpoint.
- Erase provider-specific details. No OpenAI specific parameters, Claude
  formatting, or provider endpoints that would leak into this trait API.
- Use associated types for errors, allowing each provider implementation to
  define its own error type without forcing allocation or type erasure.
