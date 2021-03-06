Accel: GPGPU Framework for Rust
================================

[![pipeline status](https://gitlab.com/termoshtt/accel/badges/master/pipeline.svg)](https://gitlab.com/termoshtt/accel/commits/master)

|crate       |crates.io                                                                   |docs.rs                                                                |GitLab Pages                                                                  |                                           |
|:-----------|:---------------------------------------------------------------------------|:----------------------------------------------------------------------|:-----------------------------------------------------------------------------|:------------------------------------------|
|accel       |[![Crate](http://meritbadge.herokuapp.com/accel)][crate/accel]              |[![docs.rs](https://docs.rs/accel/badge.svg)][docs/accel]              |[![cargo-doc](https://img.shields.io/badge/doc-master-blue)][dev/accel]       |CUDA-based GPGPU framework                 |
|accel-core  |[![Crate](http://meritbadge.herokuapp.com/accel-core)][crate/accel-core]    |[![docs.rs](https://docs.rs/accel-core/badge.svg)][docs/accel-core]    |[![cargo-doc](https://img.shields.io/badge/doc-master-blue)][dev/accel-core]  |Helper for writing device code             |
|accel-derive|[![Crate](http://meritbadge.herokuapp.com/accel-derive)][crate/accel-derive]|[![docs.rs](https://docs.rs/accel-derive/badge.svg)][docs/accel-derive]|[![cargo-doc](https://img.shields.io/badge/doc-master-blue)][dev/accel-derive]|Procedural macro for generating kernel code|

[crate/accel]:        https://crates.io/crates/accel/0.3.0-alpha.2
[crate/accel-core]:   https://crates.io/crates/accel-core/0.3.0-alpha.3
[crate/accel-derive]: https://crates.io/crates/accel-derive/0.3.0-alpha.2

[docs/accel]:        https://docs.rs/accel/0.3.0-alpha.2
[docs/accel-core]:   https://docs.rs/accel-core/0.3.0-alpha.3
[docs/accel-derive]: https://docs.rs/accel-derive/0.3.0-alpha.2

[dev/accel]:        https://termoshtt.gitlab.io/accel/accel/accel
[dev/accel-core]:   https://termoshtt.gitlab.io/accel/accel/accel_core
[dev/accel-derive]: https://termoshtt.gitlab.io/accel/accel/accel_derive

Requirements
------------
![minimum supported rust version](https://img.shields.io/badge/rustc-1.42+-red.svg)

- Minimum Supported Rust Version (MSRV) is 1.42.0
- Install [CUDA](https://developer.nvidia.com/cuda-downloads) on your system
  - accel depends on CUDA Runtime and Device APIs through [rust-cuda/cuda-sys](https://github.com/rust-cuda/cuda-sys)
- Setup NVPTX target of Rust
  - Install `nightly-2020-01-02` toolchain with  `nvptx64-nvidia-cuda` target, and [rust-ptx-linker](https://github.com/denzp/rust-ptx-linker)
  - There is an [setup script](setup_nvptx_toolchain.sh) for them:

```
curl -sSL https://gitlab.com/termoshtt/accel/raw/master/setup_nvptx_toolchain.sh | bash
```

Or, you can use [docekr container](./docker)

Contribution
------------
This project is developed on [GitLab](https://gitlab.com/termoshtt/accel) and mirrored to [GitHub](https://github.com/rust-accel/accel).

Licence
--------
Dual-licensed to be compatible with the Rust project.

- Apache License, Version 2.0 http://www.apache.org/licenses/LICENSE-2.0
- the MIT license http://opensource.org/licenses/MIT

In addition, you must refer [End User License Agreement](https://docs.nvidia.com/cuda/eula/index.html) for using CUDA.

Sponsors
--------
- [RICOS Co. Ltd](https://www.ricos.co.jp/)
  - GPU instances for CI and development
