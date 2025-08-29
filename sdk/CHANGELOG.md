# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.0](https://github.com/ngergs/crossplane-fn-sdk-rs-unofficial/releases/tag/crossplane-fn-sdk-unofficial-v0.1.0) - 2025-08-29

### Added

- [**breaking**] make k8s-openapi feature non-default
- blanket implementation of `CompositeFunction` for fitting `Fn`
- [**breaking**] `run_server` not uses custom `CompositeFunction`-trait as argument
- make --tls-certs-dir flag optional (ignored anyways) if --insecure is active
- dependency updates (tonic/prost v0.14)
- [**breaking**] make server package private, export only `run_server`-function (moved to sdk root)
- [**breaking**] do not re-export prost-types
- [**breaking**] make sdk error package non-public
- [**breaking**] consolidate sdk traits
- [**breaking**] rework k8s-openapi dependency handling in sdk
- extension trait for more intuitive mapping from request to response meta
- [**breaking**] add try_from_optional_resource trait, drop take_from_observed_composite sdk function
- sdk helper function to get the composite struct from `observed.composite`
- [**breaking**] rename response meta mapper func
- sdk function to translate metadata from request to response
- simplify Ready mapping by impl From<bool> for Ready
- [**breaking**] make try_from_resource more intuitive to use
- move type mapping from example into sdk
- crossplane-v2 support
- working split in sdk and example

### Fixed

- clippy
- move codegen to extra crate
- document codegen
- rename file for generated crossplane rust types
- cleanup crossplane codegen (do not generate client or transport code)
- cleanup superfluous Sized requirement for `TryFromResource`
- move codegen to extra sdk binary command
- [**breaking**] cleanup dependencies, rename sdk errors package to error
- dependency updates
- workaround integer type loss from crossplane using structpb.Struct for unstructured.Unstructured composite
- clippy

### Other

- docs typo
- readme
- update sdk changelog
- drop -rs semi-suffix from crate name
- adjust compile time dependency docs
- prepare release
- disable clippy for generated code, move profile settings to workspace
- docs
- cargo workspace
- docs formulation
- tests for sdk-internal json_value_cast_float_to_i64
- better errors for missing kopium compile-time dependency
- codegen docs
- clippy (cfg codegen import)
- readme
- docs typo
- readme
- readme
- rename crate(unpublished atm)/repo
- example doc for CompositeFunction-trait
- readme
- readme
- docs
- cargo metadata
- link rust-docs in readme
- commit generated sdk code
- fmt
- example for `run_server`
- docs
- docs
- dev-dependencies for sdk (k8s_openapi-dependency without v1_* feature in the sdk itself)
- readme
- rename repo and functions to `crossplane-rs-unofficial` (rs instead of rust)
- readme
- *(readme)* fmt and compile-time dependencies reformulation
- fmt
- build.rs comment
- readme fmt
- readme titles
- mv crossplane types to sdk
- re-add crossplane submodule to sdk
- restructure to separate sdk and example
