# src/ASM_Code/bitwise_ops.s
# Fast bitwise operations in x86-64 assembly

.section .text
.globl asm_fast_and
.globl asm_fast_or
.globl asm_fast_xor
.globl asm_fast_shift_left
.globl asm_fast_shift_right
.globl asm_count_bits
.globl asm_reverse_bits

# Fast bitwise AND: asm_fast_and(a: u64, b: u64) -> u64
asm_fast_and:
    movq %rdi, %rax    # Move first argument (a) to return register
    andq %rsi, %rax    # Bitwise AND with second argument (b)
    ret                # Return result in %rax

# Fast bitwise OR: asm_fast_or(a: u64, b: u64) -> u64  
asm_fast_or:
    movq %rdi, %rax    # Move first argument to return register
    orq %rsi, %rax     # Bitwise OR with second argument
    ret

# Fast bitwise XOR: asm_fast_xor(a: u64, b: u64) -> u64
asm_fast_xor:
    movq %rdi, %rax    # Move first argument to return register
    xorq %rsi, %rax    # Bitwise XOR with second argument  
    ret

# Fast left shift: asm_fast_shift_left(value: u64, shift: u64) -> u64
asm_fast_shift_left:
    movq %rdi, %rax    # Move value to return register
    movq %rsi, %rcx    # Move shift amount to %rcx (required for shift ops)
    salq %cl, %rax     # Shift left by %cl bits (lower 8 bits of %rcx)
    ret

# Fast right shift: asm_fast_shift_right(value: u64, shift: u64) -> u64
asm_fast_shift_right:
    movq %rdi, %rax    # Move value to return register
    movq %rsi, %rcx    # Move shift amount to %rcx
    shrq %cl, %rax     # Logical right shift by %cl bits
    ret

# Count set bits (population count): asm_count_bits(value: u64) -> u64
asm_count_bits:
    movq %rdi, %rax    # Move input to working register
    xorq %rdx, %rdx    # Clear counter
count_loop:
    testq %rax, %rax   # Test if any bits left
    jz count_done      # Jump to done if zero
    incq %rdx          # Increment counter
    decq %rcx          # Decrement for next iteration
    movq %rax, %rcx    # Copy value
    decq %rcx          # Subtract 1
    andq %rcx, %rax    # Clear lowest set bit
    jmp count_loop     # Continue loop
count_done:
    movq %rdx, %rax    # Move result to return register
    ret

# Reverse bits in a 64-bit integer: asm_reverse_bits(value: u64) -> u64  
asm_reverse_bits:
    movq %rdi, %rax    # Move input to working register
    movq $63, %rcx     # Counter for bit position
    xorq %rdx, %rdx    # Clear result register
reverse_loop:
    testq %rax, %rax   # Check if any bits left to process
    jz reverse_done    # Jump if done
    shrq $1, %rax      # Shift input right by 1
    rclq $1, %rdx      # Rotate result left through carry
    loop reverse_loop  # Decrement %rcx and loop if not zero
reverse_done:
    movq %rdx, %rax    # Move result to return register
    ret

