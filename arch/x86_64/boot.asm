; MB header (https://www.gnu.org/software/grub/manual/multiboot2/multiboot.html)
section .multiboot
align 4
header:
    dd 0xe85250d6
    dd 0
    dd (header.end - header)
    dd -(0xe85250d6 + header.end - header)

    ; End tag
    dw 0
    dw 0
    dd 8
.end:

section .bss
align 4096
table:
.p4:
    resb 4096
.p3:
    resb 4096
.p2:
    resb 4096

stack:
    resb 16384 ; Reserve 16KiB for the stack.
.top:

section .rodata
gdt:
    dq 0
.code: equ $ - gdt
    dq (1 << 53) | (1 << 47) | (1 << 44) | (1 << 43)
.pointer:
    dw $ - gdt - 1
    dq gdt

section .text
bits 32

global start
start:
    ; Load the top of the stack into the stack pointer.
    mov esp, stack.top

    ; Check if we may enter long mode.
    call multiboot
    call cpuid
    call longmode

    ; Setup identity paging temporarly.
    call paging

    lgdt [gdt.pointer]
    jmp gdt.code:start64

multiboot:
    cmp eax, 0x36d76289
    jne error
    ret

cpuid:
    pushfd
    pop eax
    mov ecx, eax
    xor eax, (1 << 21)
    push eax
    popfd

    pushfd
    pop eax
    push ecx
    popfd
    xor eax, ecx
    jz error 
    ret

longmode:
    mov eax, 0x80000000
    cpuid
    cmp eax, 0x80000001
    jb error 

    mov eax, 0x80000001
    cpuid
    test edx, 1 << 29
    jz error
    ret

paging:
    mov eax, table.p3
    or eax, 0x3 ; Present & Writable
    mov [table.p4], eax

    mov eax, table.p2
    or eax, 0x3 ; Present & Writable
    mov [table.p3], eax

    ; Identity page the first GiB of the memory.
    mov ecx, 0
.loop:
    mov eax, 0x200000   ; 2MiB
    mul ecx
    or eax, 0x83        ; Present & Writable & Huge
    mov [table.p2 + ecx * 8], eax

    inc ecx
    cmp ecx, 512
    jne .loop

.enable:
    ; Load P4 into CR3
    mov eax, table.p4
    mov cr3, eax

    ; Enable PAE in CR4
    mov eax, cr4
    or eax, (1 << 5)
    mov cr4, eax

    ; Set Long Mode bit in EFER
    mov ecx, 0xC0000080
    rdmsr
    or eax, (1 << 8)
    wrmsr

    ; Enable Paging in CR0
    mov eax, cr0
    or eax, (1 << 31)
    mov cr0, eax
    
    ret

error:
    cli
    hlt
    jmp error 
    
bits 64
start64:
    mov ax, 0
    mov ss, ax
    mov ds, ax
    mov es, ax
    mov fs, ax
    mov gs, ax

    extern main
    call main
    
    mov rax, 0x5f593f415f4b3f4f
    mov qword [0xb8000], rax
    hlt
