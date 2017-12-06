; ----------------------------------------------------------------------------------------
; Writes "Hello, World" to the console using only system calls. Runs on 64-bit Linux only.
; To assemble and run:
;
;     nasm -felf64 hello.asm && ld hello.o && ./a.out
; ----------------------------------------------------------------------------------------

        global  _start

        section .text


_start:
        mov     rdi, 0x0                ; file descriptor = 0 = stdin
        lea     rsi, [rsp+8]            ; buffer = address to store the bytes read
        mov     rdx, 0x1                ; the number of bytes to read
        mov     rax, 0x0                ; SYSCALL number for reading
        syscall

        mov     rdi, 1                  ; file descriptor = 1 = stdout
        lea     rsi, [rsp+8]
        mov     rdx, 1
        mov     rax, 0x1                ; syscall code for write
        syscall

        ;; exit(0)
        mov     eax, 60                 ; system call 60 is exit
        xor     rdi, rdi                ; exit code 0
        syscall                         ; invoke operating system to exit

print_

        ;xor     rax, rax                ; clear off rax
        ;mov     rbx, 
        ;; write(1, message, 13)
        ;mov     rax, 1                  ; system call 1 is write
        ;mov     rdi, 1                  ; file handle 1 is stdout
        ;mov     rsi, message            ; address of string to output
        ;mov     rdx, 13                 ; number of bytes
        ;syscall                         ; invoke operating system to do the write

message:
        db      " "                     ; The characters we read will go here, one at a time


