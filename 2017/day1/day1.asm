; ----------------------------------------------------------------------------------------
; Writes "Hello, World" to the console using only system calls. Runs on 64-bit Linux only.
; To assemble and run:
;
;     nasm -felf64 hello.asm && ld hello.o && ./a.out
; ----------------------------------------------------------------------------------------
        global main
        extern printf

        section .data
msg:    db      " ", 0                     ; The characters we read will go here, one at a time
format: db      "%d"                    ; when we call printf, we'll use this format string
str:    db      "%s"

        section .text
main:
        call    read_char

        ; print the string we read
        push    rbx                     ; caller-save register
        lea     rdi, [str]              ; set format string
        lea     rsi, [msg]              ; set string argument
        call    printf
        pop     rbx                     ; caller-save register

        ret

read_char:
        ; Read a single char from standard input into the msg buffer
        mov     rdi, 0x0                ; file descriptor = 0 = stdin
        lea     rsi, [msg]            ; buffer = address to store the bytes read
        mov     rdx, 0x1                ; the number of bytes to read
        mov     rax, 0x0                ; SYSCALL number for reading
        syscall
        ret
