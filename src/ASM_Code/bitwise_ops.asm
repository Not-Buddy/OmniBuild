; src/ASM_Code/bitwise_ops.asm
; Fast bitwise operations in x86-64 assembly for MASM ml64

.code

; Fast bitwise AND: asm_fast_and(a: u64, b: u64) -> u64
asm_fast_and PROC
    mov     rax, rcx        ; first argument in rcx (Windows x64 calling convention)
    and     rax, rdx        ; second argument in rdx
    ret
asm_fast_and ENDP

; Fast bitwise OR: asm_fast_or(a: u64, b: u64) -> u64
asm_fast_or PROC
    mov     rax, rcx
    or      rax, rdx
    ret
asm_fast_or ENDP

; Fast bitwise XOR: asm_fast_xor(a: u64, b: u64) -> u64
asm_fast_xor PROC
    mov     rax, rcx
    xor     rax, rdx
    ret
asm_fast_xor ENDP

; Fast left shift: asm_fast_shift_left(value: u64, shift: u64) -> u64
asm_fast_shift_left PROC
    mov     rax, rcx
    mov     cl, dl          ; shift amount in lower 8 bits of dl
    shl     rax, cl
    ret
asm_fast_shift_left ENDP

; Fast right shift: asm_fast_shift_right(value: u64, shift: u64) -> u64
asm_fast_shift_right PROC
    mov     rax, rcx
    mov     cl, dl
    shr     rax, cl
    ret
asm_fast_shift_right ENDP

; Count set bits (population count): asm_count_bits(value: u64) -> u64
asm_count_bits PROC
    mov     rax, rcx        ; input value
    xor     rdx, rdx        ; counter = 0
count_loop:
    test    rax, rax
    jz      count_done
    inc     rdx
    lea     rcx, [rax - 1]
    and     rax, rcx
    jmp     count_loop
count_done:
    mov     rax, rdx
    ret
asm_count_bits ENDP

; Reverse bits in a 64-bit integer: asm_reverse_bits(value: u64) -> u64
asm_reverse_bits PROC
    mov     rax, rcx
    mov     rcx, 63         ; bit position counter
    xor     rdx, rdx        ; result = 0
reverse_loop:
    test    rax, rax
    jz      reverse_done
    shr     rax, 1
    rcl     rdx, 1
    loop    reverse_loop
reverse_done:
    mov     rax, rdx
    ret
asm_reverse_bits ENDP

END
