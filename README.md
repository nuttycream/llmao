# LLMAO

### Large Language Model API Ops

`llmao` is an abstraction layer for interfacing with different LLM providers.

The goal of this trait system is to create a unified and generic way of
communicating with LLM providers. This library defines the traits and types but
does not include provider implementations, you must bring your own provider
client and implement these traits for it.
