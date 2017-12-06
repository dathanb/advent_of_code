; ----------------------------------------------------------------------------------------
; Writes "Hello, World" to the console using only system calls. Runs on 64-bit Linux only.
; To assemble and run:
;
;     nasm -felf64 hello.asm && ld hello.o && ./a.out
; ----------------------------------------------------------------------------------------

        section .data
msg:    db      " ", 0                     ; The characters we read will go here, one at a time


        section .text
        global  _start
_start:
        mov     rdi, 0x0                ; file descriptor = 0 = stdin
        lea     rsi, [msg]            ; buffer = address to store the bytes read
        mov     rdx, 0x1                ; the number of bytes to read
        mov     rax, 0x0                ; SYSCALL number for reading
        syscall

        lea     rdi, [msg]            ; first arg is string to write
        mov     rsi, 1                  ; second arg is length of string
        call    print_string

        ;; exit(0)
        mov     eax, 60                 ; system call 60 is exit
        xor     rdi, rdi                ; exit code 0
        syscall                         ; invoke operating system to exit

print_string:
        ; the calling convention is to pass arguments in registers first
        ; so we assume the address of the string to print is in rdi, and
        ; the length is in rsi
        push    rdx
        push    rsi
        push    rdi
        push    rax
        mov     rdx, rsi                ; arg2 is length, that's in rsi
        mov     rsi, rdi                ; arg1 is memory address, that's in rdi
        mov     rdi, 1                  ; print to stdout
        mov     rax, 1                  ; syscall code for write
        syscall
        pop rax
        pop rdi
        pop rsi
        pop rdx
        ret



