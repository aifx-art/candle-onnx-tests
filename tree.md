test-candle-onnx v0.1.0 (/home/pro/code/lls/candle-onnx-tests)
├── anyhow v1.0.97
├── candle-core v0.8.4
│   ├── byteorder v1.5.0
│   ├── gemm v0.17.1
│   │   ├── dyn-stack v0.10.0
│   │   │   ├── bytemuck v1.22.0
│   │   │   │   └── bytemuck_derive v1.9.3 (proc-macro)
│   │   │   │       ├── proc-macro2 v1.0.94
│   │   │   │       │   └── unicode-ident v1.0.18
│   │   │   │       ├── quote v1.0.40
│   │   │   │       │   └── proc-macro2 v1.0.94 (*)
│   │   │   │       └── syn v2.0.100
│   │   │   │           ├── proc-macro2 v1.0.94 (*)
│   │   │   │           ├── quote v1.0.40 (*)
│   │   │   │           └── unicode-ident v1.0.18
│   │   │   └── reborrow v0.5.5
│   │   ├── gemm-c32 v0.17.1
│   │   │   ├── dyn-stack v0.10.0 (*)
│   │   │   ├── gemm-common v0.17.1
│   │   │   │   ├── bytemuck v1.22.0 (*)
│   │   │   │   ├── dyn-stack v0.10.0 (*)
│   │   │   │   ├── half v2.6.0
│   │   │   │   │   ├── bytemuck v1.22.0 (*)
│   │   │   │   │   ├── cfg-if v1.0.0
│   │   │   │   │   ├── num-traits v0.2.19
│   │   │   │   │   │   └── libm v0.2.11
│   │   │   │   │   │   [build-dependencies]
│   │   │   │   │   │   └── autocfg v1.4.0
│   │   │   │   │   ├── rand v0.9.0
│   │   │   │   │   │   ├── rand_chacha v0.9.0
│   │   │   │   │   │   │   ├── ppv-lite86 v0.2.21
│   │   │   │   │   │   │   │   └── zerocopy v0.8.24
│   │   │   │   │   │   │   └── rand_core v0.9.3
│   │   │   │   │   │   │       └── getrandom v0.3.2
│   │   │   │   │   │   │           ├── cfg-if v1.0.0
│   │   │   │   │   │   │           └── libc v0.2.171
│   │   │   │   │   │   ├── rand_core v0.9.3 (*)
│   │   │   │   │   │   └── zerocopy v0.8.24
│   │   │   │   │   └── rand_distr v0.5.1
│   │   │   │   │       ├── num-traits v0.2.19 (*)
│   │   │   │   │       └── rand v0.9.0 (*)
│   │   │   │   ├── num-complex v0.4.6
│   │   │   │   │   ├── bytemuck v1.22.0 (*)
│   │   │   │   │   └── num-traits v0.2.19 (*)
│   │   │   │   ├── num-traits v0.2.19 (*)
│   │   │   │   ├── once_cell v1.21.3
│   │   │   │   ├── paste v1.0.15 (proc-macro)
│   │   │   │   ├── pulp v0.18.22
│   │   │   │   │   ├── bytemuck v1.22.0 (*)
│   │   │   │   │   ├── libm v0.2.11
│   │   │   │   │   ├── num-complex v0.4.6 (*)
│   │   │   │   │   └── reborrow v0.5.5
│   │   │   │   ├── raw-cpuid v10.7.0
│   │   │   │   │   └── bitflags v1.3.2
│   │   │   │   ├── rayon v1.10.0
│   │   │   │   │   ├── either v1.15.0
│   │   │   │   │   └── rayon-core v1.12.1
│   │   │   │   │       ├── crossbeam-deque v0.8.6
│   │   │   │   │       │   ├── crossbeam-epoch v0.9.18
│   │   │   │   │       │   │   └── crossbeam-utils v0.8.21
│   │   │   │   │       │   └── crossbeam-utils v0.8.21
│   │   │   │   │       └── crossbeam-utils v0.8.21
│   │   │   │   └── seq-macro v0.3.6 (proc-macro)
│   │   │   ├── num-complex v0.4.6 (*)
│   │   │   ├── num-traits v0.2.19 (*)
│   │   │   ├── paste v1.0.15 (proc-macro)
│   │   │   ├── raw-cpuid v10.7.0 (*)
│   │   │   └── seq-macro v0.3.6 (proc-macro)
│   │   ├── gemm-c64 v0.17.1
│   │   │   ├── dyn-stack v0.10.0 (*)
│   │   │   ├── gemm-common v0.17.1 (*)
│   │   │   ├── num-complex v0.4.6 (*)
│   │   │   ├── num-traits v0.2.19 (*)
│   │   │   ├── paste v1.0.15 (proc-macro)
│   │   │   ├── raw-cpuid v10.7.0 (*)
│   │   │   └── seq-macro v0.3.6 (proc-macro)
│   │   ├── gemm-common v0.17.1 (*)
│   │   ├── gemm-f16 v0.17.1
│   │   │   ├── dyn-stack v0.10.0 (*)
│   │   │   ├── gemm-common v0.17.1 (*)
│   │   │   ├── gemm-f32 v0.17.1
│   │   │   │   ├── dyn-stack v0.10.0 (*)
│   │   │   │   ├── gemm-common v0.17.1 (*)
│   │   │   │   ├── num-complex v0.4.6 (*)
│   │   │   │   ├── num-traits v0.2.19 (*)
│   │   │   │   ├── paste v1.0.15 (proc-macro)
│   │   │   │   ├── raw-cpuid v10.7.0 (*)
│   │   │   │   └── seq-macro v0.3.6 (proc-macro)
│   │   │   ├── half v2.6.0 (*)
│   │   │   ├── num-complex v0.4.6 (*)
│   │   │   ├── num-traits v0.2.19 (*)
│   │   │   ├── paste v1.0.15 (proc-macro)
│   │   │   ├── raw-cpuid v10.7.0 (*)
│   │   │   ├── rayon v1.10.0 (*)
│   │   │   └── seq-macro v0.3.6 (proc-macro)
│   │   ├── gemm-f32 v0.17.1 (*)
│   │   ├── gemm-f64 v0.17.1
│   │   │   ├── dyn-stack v0.10.0 (*)
│   │   │   ├── gemm-common v0.17.1 (*)
│   │   │   ├── num-complex v0.4.6 (*)
│   │   │   ├── num-traits v0.2.19 (*)
│   │   │   ├── paste v1.0.15 (proc-macro)
│   │   │   ├── raw-cpuid v10.7.0 (*)
│   │   │   └── seq-macro v0.3.6 (proc-macro)
│   │   ├── num-complex v0.4.6 (*)
│   │   ├── num-traits v0.2.19 (*)
│   │   ├── paste v1.0.15 (proc-macro)
│   │   ├── raw-cpuid v10.7.0 (*)
│   │   └── seq-macro v0.3.6 (proc-macro)
│   ├── half v2.6.0 (*)
│   ├── memmap2 v0.9.5
│   │   ├── libc v0.2.171
│   │   └── stable_deref_trait v1.2.0
│   ├── num-traits v0.2.19 (*)
│   ├── num_cpus v1.16.0
│   │   └── libc v0.2.171
│   ├── rand v0.9.0 (*)
│   ├── rand_distr v0.5.1 (*)
│   ├── rayon v1.10.0 (*)
│   ├── safetensors v0.4.5
│   │   ├── serde v1.0.219
│   │   │   └── serde_derive v1.0.219 (proc-macro)
│   │   │       ├── proc-macro2 v1.0.94 (*)
│   │   │       ├── quote v1.0.40 (*)
│   │   │       └── syn v2.0.100 (*)
│   │   └── serde_json v1.0.140
│   │       ├── itoa v1.0.15
│   │       ├── memchr v2.7.4
│   │       ├── ryu v1.0.20
│   │       └── serde v1.0.219 (*)
│   ├── thiserror v1.0.69
│   │   └── thiserror-impl v1.0.69 (proc-macro)
│   │       ├── proc-macro2 v1.0.94 (*)
│   │       ├── quote v1.0.40 (*)
│   │       └── syn v2.0.100 (*)
│   ├── ug v0.1.0
│   │   ├── gemm v0.18.2
│   │   │   ├── dyn-stack v0.13.0
│   │   │   │   └── bytemuck v1.22.0 (*)
│   │   │   ├── gemm-c32 v0.18.2
│   │   │   │   ├── dyn-stack v0.13.0 (*)
│   │   │   │   ├── gemm-common v0.18.2
│   │   │   │   │   ├── bytemuck v1.22.0 (*)
│   │   │   │   │   ├── dyn-stack v0.13.0 (*)
│   │   │   │   │   ├── half v2.6.0 (*)
│   │   │   │   │   ├── libm v0.2.11
│   │   │   │   │   ├── num-complex v0.4.6 (*)
│   │   │   │   │   ├── num-traits v0.2.19 (*)
│   │   │   │   │   ├── once_cell v1.21.3
│   │   │   │   │   ├── paste v1.0.15 (proc-macro)
│   │   │   │   │   ├── pulp v0.21.4
│   │   │   │   │   │   ├── bytemuck v1.22.0 (*)
│   │   │   │   │   │   ├── cfg-if v1.0.0
│   │   │   │   │   │   ├── libm v0.2.11
│   │   │   │   │   │   ├── num-complex v0.4.6 (*)
│   │   │   │   │   │   └── reborrow v0.5.5
│   │   │   │   │   │   [build-dependencies]
│   │   │   │   │   │   └── version_check v0.9.5
│   │   │   │   │   ├── raw-cpuid v11.5.0
│   │   │   │   │   │   └── bitflags v2.9.0
│   │   │   │   │   ├── rayon v1.10.0 (*)
│   │   │   │   │   └── seq-macro v0.3.6 (proc-macro)
│   │   │   │   ├── num-complex v0.4.6 (*)
│   │   │   │   ├── num-traits v0.2.19 (*)
│   │   │   │   ├── paste v1.0.15 (proc-macro)
│   │   │   │   ├── raw-cpuid v11.5.0 (*)
│   │   │   │   └── seq-macro v0.3.6 (proc-macro)
│   │   │   ├── gemm-c64 v0.18.2
│   │   │   │   ├── dyn-stack v0.13.0 (*)
│   │   │   │   ├── gemm-common v0.18.2 (*)
│   │   │   │   ├── num-complex v0.4.6 (*)
│   │   │   │   ├── num-traits v0.2.19 (*)
│   │   │   │   ├── paste v1.0.15 (proc-macro)
│   │   │   │   ├── raw-cpuid v11.5.0 (*)
│   │   │   │   └── seq-macro v0.3.6 (proc-macro)
│   │   │   ├── gemm-common v0.18.2 (*)
│   │   │   ├── gemm-f16 v0.18.2
│   │   │   │   ├── dyn-stack v0.13.0 (*)
│   │   │   │   ├── gemm-common v0.18.2 (*)
│   │   │   │   ├── gemm-f32 v0.18.2
│   │   │   │   │   ├── dyn-stack v0.13.0 (*)
│   │   │   │   │   ├── gemm-common v0.18.2 (*)
│   │   │   │   │   ├── num-complex v0.4.6 (*)
│   │   │   │   │   ├── num-traits v0.2.19 (*)
│   │   │   │   │   ├── paste v1.0.15 (proc-macro)
│   │   │   │   │   ├── raw-cpuid v11.5.0 (*)
│   │   │   │   │   └── seq-macro v0.3.6 (proc-macro)
│   │   │   │   ├── half v2.6.0 (*)
│   │   │   │   ├── num-complex v0.4.6 (*)
│   │   │   │   ├── num-traits v0.2.19 (*)
│   │   │   │   ├── paste v1.0.15 (proc-macro)
│   │   │   │   ├── raw-cpuid v11.5.0 (*)
│   │   │   │   ├── rayon v1.10.0 (*)
│   │   │   │   └── seq-macro v0.3.6 (proc-macro)
│   │   │   ├── gemm-f32 v0.18.2 (*)
│   │   │   ├── gemm-f64 v0.18.2
│   │   │   │   ├── dyn-stack v0.13.0 (*)
│   │   │   │   ├── gemm-common v0.18.2 (*)
│   │   │   │   ├── num-complex v0.4.6 (*)
│   │   │   │   ├── num-traits v0.2.19 (*)
│   │   │   │   ├── paste v1.0.15 (proc-macro)
│   │   │   │   ├── raw-cpuid v11.5.0 (*)
│   │   │   │   └── seq-macro v0.3.6 (proc-macro)
│   │   │   ├── num-complex v0.4.6 (*)
│   │   │   ├── num-traits v0.2.19 (*)
│   │   │   ├── paste v1.0.15 (proc-macro)
│   │   │   ├── raw-cpuid v11.5.0 (*)
│   │   │   └── seq-macro v0.3.6 (proc-macro)
│   │   ├── half v2.6.0 (*)
│   │   ├── libloading v0.8.6
│   │   │   └── cfg-if v1.0.0
│   │   ├── memmap2 v0.9.5 (*)
│   │   ├── num v0.4.3
│   │   │   ├── num-bigint v0.4.6
│   │   │   │   ├── num-integer v0.1.46
│   │   │   │   │   └── num-traits v0.2.19 (*)
│   │   │   │   └── num-traits v0.2.19 (*)
│   │   │   ├── num-complex v0.4.6 (*)
│   │   │   ├── num-integer v0.1.46 (*)
│   │   │   ├── num-iter v0.1.45
│   │   │   │   ├── num-integer v0.1.46 (*)
│   │   │   │   └── num-traits v0.2.19 (*)
│   │   │   │   [build-dependencies]
│   │   │   │   └── autocfg v1.4.0
│   │   │   ├── num-rational v0.4.2
│   │   │   │   ├── num-bigint v0.4.6 (*)
│   │   │   │   ├── num-integer v0.1.46 (*)
│   │   │   │   └── num-traits v0.2.19 (*)
│   │   │   └── num-traits v0.2.19 (*)
│   │   ├── num-traits v0.2.19 (*)
│   │   ├── num_cpus v1.16.0 (*)
│   │   ├── rayon v1.10.0 (*)
│   │   ├── safetensors v0.4.5 (*)
│   │   ├── serde v1.0.219 (*)
│   │   ├── thiserror v1.0.69 (*)
│   │   ├── tracing v0.1.41
│   │   │   ├── pin-project-lite v0.2.16
│   │   │   ├── tracing-attributes v0.1.28 (proc-macro)
│   │   │   │   ├── proc-macro2 v1.0.94 (*)
│   │   │   │   ├── quote v1.0.40 (*)
│   │   │   │   └── syn v2.0.100 (*)
│   │   │   └── tracing-core v0.1.33
│   │   │       └── once_cell v1.21.3
│   │   └── yoke v0.7.5
│   │       ├── stable_deref_trait v1.2.0
│   │       ├── yoke-derive v0.7.5 (proc-macro)
│   │       │   ├── proc-macro2 v1.0.94 (*)
│   │       │   ├── quote v1.0.40 (*)
│   │       │   ├── syn v2.0.100 (*)
│   │       │   └── synstructure v0.13.1
│   │       │       ├── proc-macro2 v1.0.94 (*)
│   │       │       ├── quote v1.0.40 (*)
│   │       │       └── syn v2.0.100 (*)
│   │       └── zerofrom v0.1.6
│   │           └── zerofrom-derive v0.1.6 (proc-macro)
│   │               ├── proc-macro2 v1.0.94 (*)
│   │               ├── quote v1.0.40 (*)
│   │               ├── syn v2.0.100 (*)
│   │               └── synstructure v0.13.1 (*)
│   ├── yoke v0.7.5 (*)
│   └── zip v1.1.4
│       ├── crc32fast v1.4.2
│       │   └── cfg-if v1.0.0
│       ├── displaydoc v0.2.5 (proc-macro)
│       │   ├── proc-macro2 v1.0.94 (*)
│       │   ├── quote v1.0.40 (*)
│       │   └── syn v2.0.100 (*)
│       ├── indexmap v2.9.0
│       │   ├── equivalent v1.0.2
│       │   └── hashbrown v0.15.2
│       ├── num_enum v0.7.3
│       │   └── num_enum_derive v0.7.3 (proc-macro)
│       │       ├── proc-macro-crate v3.3.0
│       │       │   └── toml_edit v0.22.24
│       │       │       ├── indexmap v2.9.0 (*)
│       │       │       ├── toml_datetime v0.6.8
│       │       │       └── winnow v0.7.6
│       │       ├── proc-macro2 v1.0.94 (*)
│       │       ├── quote v1.0.40 (*)
│       │       └── syn v2.0.100 (*)
│       └── thiserror v1.0.69 (*)
├── candle-onnx v0.8.4
│   ├── candle-core v0.8.4 (*)
│   ├── candle-nn v0.8.4
│   │   ├── candle-core v0.8.4 (*)
│   │   ├── half v2.6.0 (*)
│   │   ├── num-traits v0.2.19 (*)
│   │   ├── rayon v1.10.0 (*)
│   │   ├── safetensors v0.4.5 (*)
│   │   ├── serde v1.0.219 (*)
│   │   └── thiserror v1.0.69 (*)
│   └── prost v0.12.6
│       ├── bytes v1.10.1
│       └── prost-derive v0.12.6 (proc-macro)
│           ├── anyhow v1.0.97
│           ├── itertools v0.12.1
│           │   └── either v1.15.0
│           ├── proc-macro2 v1.0.94 (*)
│           ├── quote v1.0.40 (*)
│           └── syn v2.0.100 (*)
│   [build-dependencies]
│   └── prost-build v0.12.6
│       ├── bytes v1.10.1
│       ├── heck v0.5.0
│       ├── itertools v0.12.1 (*)
│       ├── log v0.4.27
│       ├── multimap v0.10.0
│       ├── once_cell v1.21.3
│       ├── petgraph v0.6.5
│       │   ├── fixedbitset v0.4.2
│       │   └── indexmap v2.9.0 (*)
│       ├── prettyplease v0.2.32
│       │   ├── proc-macro2 v1.0.94 (*)
│       │   └── syn v2.0.100 (*)
│       ├── prost v0.12.6
│       │   ├── bytes v1.10.1
│       │   └── prost-derive v0.12.6 (proc-macro) (*)
│       ├── prost-types v0.12.6
│       │   └── prost v0.12.6 (*)
│       ├── regex v1.11.1
│       │   ├── regex-automata v0.4.9
│       │   │   └── regex-syntax v0.8.5
│       │   └── regex-syntax v0.8.5
│       ├── syn v2.0.100 (*)
│       └── tempfile v3.19.1
│           ├── fastrand v2.3.0
│           ├── getrandom v0.3.2
│           │   ├── cfg-if v1.0.0
│           │   └── libc v0.2.171
│           ├── once_cell v1.21.3
│           └── rustix v1.0.5
│               ├── bitflags v2.9.0
│               └── linux-raw-sys v0.9.3
├── clap v4.5.35
│   ├── clap_builder v4.5.35
│   │   ├── anstream v0.6.18
│   │   │   ├── anstyle v1.0.10
│   │   │   ├── anstyle-parse v0.2.6
│   │   │   │   └── utf8parse v0.2.2
│   │   │   ├── anstyle-query v1.1.2
│   │   │   ├── colorchoice v1.0.3
│   │   │   ├── is_terminal_polyfill v1.70.1
│   │   │   └── utf8parse v0.2.2
│   │   ├── anstyle v1.0.10
│   │   ├── clap_lex v0.7.4
│   │   └── strsim v0.11.1
│   └── clap_derive v4.5.32 (proc-macro)
│       ├── heck v0.5.0
│       ├── proc-macro2 v1.0.94 (*)
│       ├── quote v1.0.40 (*)
│       └── syn v2.0.100 (*)
├── cpal v0.15.3
│   ├── alsa v0.9.1
│   │   ├── alsa-sys v0.3.1
│   │   │   └── libc v0.2.171
│   │   │   [build-dependencies]
│   │   │   └── pkg-config v0.3.32
│   │   ├── bitflags v2.9.0
│   │   ├── cfg-if v1.0.0
│   │   └── libc v0.2.171
│   ├── dasp_sample v0.11.0
│   └── libc v0.2.171
├── image v0.25.6
│   ├── bytemuck v1.22.0 (*)
│   ├── byteorder-lite v0.1.0
│   ├── color_quant v1.1.0
│   ├── exr v1.73.0
│   │   ├── bit_field v0.10.2
│   │   ├── half v2.6.0 (*)
│   │   ├── lebe v0.5.2
│   │   ├── miniz_oxide v0.8.8
│   │   │   ├── adler2 v2.0.0
│   │   │   └── simd-adler32 v0.3.7
│   │   ├── rayon-core v1.12.1 (*)
│   │   ├── smallvec v1.15.0
│   │   └── zune-inflate v0.2.54
│   │       └── simd-adler32 v0.3.7
│   ├── gif v0.13.1
│   │   ├── color_quant v1.1.0
│   │   └── weezl v0.1.8
│   ├── image-webp v0.2.1
│   │   ├── byteorder-lite v0.1.0
│   │   └── quick-error v2.0.1
│   ├── num-traits v0.2.19 (*)
│   ├── png v0.17.16
│   │   ├── bitflags v1.3.2
│   │   ├── crc32fast v1.4.2 (*)
│   │   ├── fdeflate v0.3.7
│   │   │   └── simd-adler32 v0.3.7
│   │   ├── flate2 v1.1.1
│   │   │   ├── crc32fast v1.4.2 (*)
│   │   │   └── miniz_oxide v0.8.8 (*)
│   │   └── miniz_oxide v0.8.8 (*)
│   ├── qoi v0.4.1
│   │   └── bytemuck v1.22.0 (*)
│   ├── ravif v0.11.11
│   │   ├── avif-serialize v0.8.3
│   │   │   └── arrayvec v0.7.6
│   │   ├── imgref v1.11.0
│   │   ├── loop9 v0.1.5
│   │   │   └── imgref v1.11.0
│   │   ├── quick-error v2.0.1
│   │   ├── rav1e v0.7.1
│   │   │   ├── arg_enum_proc_macro v0.3.4 (proc-macro)
│   │   │   │   ├── proc-macro2 v1.0.94 (*)
│   │   │   │   ├── quote v1.0.40 (*)
│   │   │   │   └── syn v2.0.100 (*)
│   │   │   ├── arrayvec v0.7.6
│   │   │   ├── av1-grain v0.2.3
│   │   │   │   ├── anyhow v1.0.97
│   │   │   │   ├── arrayvec v0.7.6
│   │   │   │   ├── log v0.4.27
│   │   │   │   ├── nom v7.1.3
│   │   │   │   │   ├── memchr v2.7.4
│   │   │   │   │   └── minimal-lexical v0.2.1
│   │   │   │   ├── num-rational v0.4.2 (*)
│   │   │   │   └── v_frame v0.3.8
│   │   │   │       ├── aligned-vec v0.5.0
│   │   │   │       └── num-traits v0.2.19 (*)
│   │   │   ├── bitstream-io v2.6.0
│   │   │   ├── cfg-if v1.0.0
│   │   │   ├── itertools v0.12.1
│   │   │   │   └── either v1.15.0
│   │   │   ├── libc v0.2.171
│   │   │   ├── log v0.4.27
│   │   │   ├── maybe-rayon v0.1.1
│   │   │   │   ├── cfg-if v1.0.0
│   │   │   │   └── rayon v1.10.0 (*)
│   │   │   ├── new_debug_unreachable v1.0.6
│   │   │   ├── noop_proc_macro v0.3.0 (proc-macro)
│   │   │   ├── num-derive v0.4.2 (proc-macro)
│   │   │   │   ├── proc-macro2 v1.0.94 (*)
│   │   │   │   ├── quote v1.0.40 (*)
│   │   │   │   └── syn v2.0.100 (*)
│   │   │   ├── num-traits v0.2.19 (*)
│   │   │   ├── once_cell v1.21.3
│   │   │   ├── paste v1.0.15 (proc-macro)
│   │   │   ├── profiling v1.0.16
│   │   │   │   └── profiling-procmacros v1.0.16 (proc-macro)
│   │   │   │       ├── quote v1.0.40 (*)
│   │   │   │       └── syn v2.0.100 (*)
│   │   │   ├── simd_helpers v0.1.0 (proc-macro)
│   │   │   │   └── quote v1.0.40 (*)
│   │   │   ├── thiserror v1.0.69 (*)
│   │   │   └── v_frame v0.3.8 (*)
│   │   │   [build-dependencies]
│   │   │   └── built v0.7.7
│   │   ├── rayon v1.10.0 (*)
│   │   └── rgb v0.8.50
│   ├── rayon v1.10.0 (*)
│   ├── rgb v0.8.50
│   ├── tiff v0.9.1
│   │   ├── flate2 v1.1.1 (*)
│   │   ├── jpeg-decoder v0.3.1
│   │   └── weezl v0.1.8
│   ├── zune-core v0.4.12
│   └── zune-jpeg v0.4.14
│       └── zune-core v0.4.12
└── rubato v0.16.2
    ├── num-complex v0.4.6 (*)
    ├── num-integer v0.1.46 (*)
    ├── num-traits v0.2.19 (*)
    └── realfft v3.4.0
        └── rustfft v6.2.0
            ├── num-complex v0.4.6 (*)
            ├── num-integer v0.1.46 (*)
            ├── num-traits v0.2.19 (*)
            ├── primal-check v0.3.4
            │   └── num-integer v0.1.46 (*)
            ├── strength_reduce v0.2.4
            └── transpose v0.2.3
                ├── num-integer v0.1.46 (*)
                └── strength_reduce v0.2.4
            [build-dependencies]
            └── version_check v0.9.5
