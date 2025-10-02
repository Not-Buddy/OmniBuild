### Below command allows cargo to build C code by itself
```
cargo add --build cc
```


```
┌─────────────────────┐
│    Rust Binary     │
│  ┌───────────────┐  │
│  │ Rust Code     │  │
│  │ ↓ unsafe call │  │
│  │ C/C++/ASM     │  │ ← Same memory space
│  │ Functions     │  │ ← Same process
│  └───────────────┘  │ ← Linked at compile time
└─────────────────────┘
```
```
┌─────────────────────┐    ┌─────────────────────┐
│    Rust Binary     │    │    Java Process     │
│  ┌───────────────┐  │    │  ┌───────────────┐  │
│  │ Rust Code     │──┼────┼→│ JVM + Java    │  │
│  │ Command::new  │  │ IPC│  │ Bytecode      │  │
│  │ Process spawn │  │    │  └───────────────┘  │
│  └───────────────┘  │    │                     │
└─────────────────────┘    └─────────────────────┘
     Process 1                   Process 2
```