// src/ASM_Code/bitwise_ops_arm64.s
// Fast bitwise operations in ARM64 assembly (AArch64) for Linux

    .text
    .global asm_fast_and
    .global asm_fast_or
    .global asm_fast_xor
    .global asm_fast_shift_left
    .global asm_fast_shift_right
    .global asm_count_bits
    .global asm_reverse_bits

// Fast bitwise AND: asm_fast_and(uint64_t a, uint64_t b) -> uint64_t
asm_fast_and:
    and x0, x0, x1
    ret

// Fast bitwise OR: asm_fast_or(uint64_t a, uint64_t b) -> uint64_t
asm_fast_or:
    orr x0, x0, x1
    ret

// Fast bitwise XOR: asm_fast_xor(uint64_t a, uint64_t b) -> uint64_t
asm_fast_xor:
    eor x0, x0, x1
    ret

// Fast left shift: asm_fast_shift_left(uint64_t value, uint64_t shift) -> uint64_t
asm_fast_shift_left:
    // shift amount in x1 (only lower 6 bits used)
    lsl x0, x0, x1
    ret

// Fast right shift: asm_fast_shift_right(uint64_t value, uint64_t shift) -> uint64_t
asm_fast_shift_right:
    // Logical shift right
    lsr x0, x0, x1
    ret

// Count set bits (population count): asm_count_bits(uint64_t value) -> uint64_t
asm_count_bits:
    // Use ARM64's builtin cnt instruction sequence for population count
    // Break value into bytes, then count bits
    // We'll implement using popcount instruction available in ARMv8.2+
    // If not available, fall back to software loop or use compiler intrinsic in Rust/C.
    // For maximum compatibility, here's a loop method:

    mov x1, #0          // counter = 0
count_loop:
    cbz x0, count_done  // if x0 == 0 jump done
    add x1, x1, #1      // counter++
    sub x0, x0, #1
    and x0, x0, x0, lsl #1   // clear lowest set bit (Brian Kernighan's algorithm)
    b count_loop
count_done:
    mov x0, x1
    ret

// Reverse bits in a 64-bit integer: asm_reverse_bits(uint64_t value) -> uint64_t
asm_reverse_bits:
    mov x1, x0          // input value
    mov x0, #0          // result = 0
    mov x2, #64         // bit counter
reverse_loop:
    cbz x2, reverse_done
    lsr x3, x1, #1      // shift right input by 1
    and x4, x1, #1      // get lowest bit
    lsl x0, x0, #1      // result << 1
    orr x0, x0, x4      // result |= lowest bit
    mov x1, x3          // input = shifted input
    subs x2, x2, #1     // decrement bit counter
    bne reverse_loop
reverse_done:
    ret

