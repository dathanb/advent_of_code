; ----------------------------------------------------------------------------------------
; Writes "Hello, World" to the console using only system calls. Runs on 64-bit Linux only.
; To assemble and run:
;
;     nasm -felf64 hello.asm && ld hello.o && ./a.out
; ----------------------------------------------------------------------------------------
        global main
        extern printf

        section .data
msg:    db      " ", 0                  ; The characters we read will go here, one at a time
format: db      "%ld", 0                    ; when we call printf, we'll use this format string
str:    db      "%s", 0

        section .text
main:
        ; we reserve r8 for the 1st char
        ; we reserve r9 for the n-1st char
        ; we reserve r10 for the nth char
        ; we reserve r15 for the total
        xor     r15, r15

        call    read_char               ; read first character
        mov     r8, [msg]               ; save the first character for comparison at the end
        and     r8, 0xff                ; truncate to one byte
        mov     r10, r8                 ; prep r10 for our first iteration

read_loop:
        mov     r9, r10
        call    read_char
        mov     r10, [msg]              ; move the read character into r10
        and     r10, 0xff               ; truncate to one byte
        cmp     r10, 0x0a               
        jz      final                   ; if we just read the newline character, we're done with input
        cmp     r9, r10
        jnz     .no_match
        mov     r11, r9                 ; prep for conversion from '1', '2', etc. to integer equivalent
        sub     r11, 0x30               ; convert from '0', '1', '2', etc. to 0,1,2,...
        add     r15, r11
.no_match:
        jmp     read_loop

final: ; the end of input
        ; first character read is in r8
        ; last character read is in r9
        cmp     r8, r9
        jnz     .no_match
        ; convert from '0', '1', '2', etc. to 0,1,2,...
        mov     r11, r9
        sub     r11, 0x30
        add     r15, r11

.no_match:

        ; print the string we read
        push    rax
        push    rbx                     ; caller-save register
        push    rcx
        lea     rdi, [format]           ; set format string
        mov     rsi, r15                ; set string argument
        call    printf
        pop     rcx
        pop     rbx                     ; caller-save register
        pop     rax

        ret

read_char:
        ; Read a single char from standard input into the msg buffer
        mov     rdi, 0x0                ; file descriptor = 0 = stdin
        lea     rsi, [msg]              ; buffer = address to store the bytes read
        mov     rdx, 0x1                ; the number of bytes to read
        mov     rax, 0x0                ; SYSCALL number for reading
        syscall
        ret
