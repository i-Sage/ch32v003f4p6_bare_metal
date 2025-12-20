.section .init, "ax"
.global _start

_start:
    # Set up stack pointer
    la sp, _stack_top
    
    # Clear BSS section (uninitialized variables)
    la a0, _sbss
    la a1, _ebss
    bgeu a0, a1, 2f
1:
    sw zero, (a0)
    addi a0, a0, 4
    bltu a0, a1, 1b
2:
    
    # Copy data section from flash to RAM
    la a0, _sidata
    la a1, _sdata
    la a2, _edata
    bgeu a1, a2, 2f
1:
    lw a3, (a0)
    sw a3, (a1)
    addi a0, a0, 4
    addi a1, a1, 4
    bltu a1, a2, 1b
2:
    
    # Jump to main
    call main
    
    # If main returns (it shouldn't), loop forever
3:
    j 3b

# Reserve space for stack at end of RAM
.section .bss
.align 4
_stack_bottom:
    .space 512  # 512 bytes for stack
_stack_top:
