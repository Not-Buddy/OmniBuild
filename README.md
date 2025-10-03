***

# OmniBuild: Multi-Language Calculator

This project demonstrates advanced polyglot integration with Rust as the main language, seamlessly linking native C/C++/Assembly functions and high-level Java (JNI) logic. The build and code organization allows all targets to run in the same process (except Java, which embeds a JVM), giving high performance and flexibility.

## Language Integration

### C, C++, Assembly

- **Rust FFI**: C, C++, and Assembly code are compiled and *statically linked* into the Rust binary using the [`cc`](https://crates.io/crates/cc) crate.
- **Usage**: Rust code calls C/C++/ASM routines with `unsafe` blocks. All execute inside the Rust process in the same memory space.
- **Build System**: The following command ensures the `cc` crate is ready for build scripts:
  ```
  cargo add --build cc
  ```
- **Diagram:**
  ```
  ┌─────────────────────┐
  │   Rust Binary      │
  │ ┌───────────────┐  │
  │ │ Rust Code     │  │
  │ │ ↓ unsafe call │  │
  │ │ C/C++/ASM     │  │ ← Same memory space
  │ │ Functions     │  │ ← Same process
  │ └───────────────┘  │ ← Linked at compile time
  └─────────────────────┘
  ```

### Java

- **JNI Integration**: The Rust binary launches a JVM inside the process using JNI. Rust calls Java via wrappers (`jni` crate).
- **How It Works**: Rust finds and loads the JVM (using `JAVA_HOME` and search-path), then calls Java static methods as needed. The Java classes are compiled at build time, and Rust invokes their bytecode via JNI.
- **Diagram:**
  ```
  ┌─────────────────────┐    ┌─────────────────────┐
  │   Rust Binary      │    │    Java Process     │
  │ ┌───────────────┐  │    │  ┌───────────────┐  │
  │ │ Rust Code     │──┼────┼→│ JVM + Java     │  │
  │ │ JNI           │  │ IPC│  │ Bytecode      │  │
  │ │ JVM embed     │  │    │  └───────────────┘  │
  │ └───────────────┘  │    │                     │
  └─────────────────────┘    └─────────────────────┘
      Process 1                  JVM (Process 2, embedded)
  ```

***

## Build Structure

- **build.rs**: Main build script, now split for modularity.
    - **gcc.rs**: Compiles C code via `cc` crate.
    - **gpp.rs**: Compiles C++ code via `cc`/`cpp` mode.
    - **asm.rs**: Compiles assembly code, handling `.asm` (Windows), `.s` (Linux x86_64), and `.s` (ARM64).
    - **javac.rs**: Invokes `javac` to build Java classes at compile time.

***

## How to Build and Run

1. **Build:**
    ```
    cargo build --release
    ```
    This automatically finds, compiles, and links all languages.

2. **Run:**
    ```
    # Ensure JAVA_HOME and PATH are set for JVM support!
    .\target\release\OmniBuild.exe
    ```

    Example for Windows:
    ```
    $env:JAVA_HOME="C:\Program Files\Java\jdk-xx"
    $env:PATH="$env:JAVA_HOME\bin\server;$env:PATH"
    .\target\release\OmniBuild.exe
    ```
    On Linux/macOS, set JAVA_HOME accordingly.

***

## Features

- Fast and safe integration of C/C++/ASM with Rust via FFI (same process/memory space).
- JNI embedding enables direct calling of optimized Java bytecode.
- Modular build system: Each language’s logic resides in its own file for maintainability.

***

## Requirements

- Rust (stable)
- C/C++ toolchains (MSVC, GCC/Clang)
- JAVA_HOME set and Java JDK installed
- Assembly supported on target (platform-dependent)
- [`cc`](https://crates.io/crates/cc) and [`jni`](https://crates.io/crates/jni) crates

***

**This project is a fully working template for modern cross-language calculation using Rust as the integration platform.**

---