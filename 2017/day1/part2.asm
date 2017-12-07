; ----------------------------------------------------------------------------------------
; Writes "Hello, World" to the console using only system calls. Runs on 64-bit Linux only.
; To assemble and run:
;
;     nasm -felf64 hello.asm && ld hello.o && ./a.out
; ----------------------------------------------------------------------------------------
        global main
        extern printf

        section .data
        ; kind of cheating by hard-coding the input here
input:  db      "516299281491169512719425276194596424291268712697155863651846937925928456958813624428156218468331423858422613471962165756423837756856519754524985759763747559711257977361228357678293572698839754444752898835313399815748562519958329927911861654784216355489319995566297499836295985943899373615223375271231128914745273184498915241488393761676799914385265459983923743146555465177886491979962465918888396664233693243983969412682561799628789569294374554575677368219724142536789649121758582991345537639888858113763738518511184439854223386868764189133964543721941169786274781775658991329331759679943342217578532643519615296424396487669451453728113114748217177826874953466435436129165295379157226345786756899935747336785161745487933721527239394118721517195849186676814232887413175587327214144876898248571248517121796766248817366614333915154796983612174281237846165129114988453188844745119798643314857871527757831265298846833327863781341559381238458322786192379487455671563757123534253463563421716138641611915686247343417126655317378639314168461345613427262786624689498485599942336813995725145169355942616672812792174556866436158375938988738721253664772584577384558696477546232189312287262439452141564522329987139692281984783513691857538335537553448919819545332125483128878925492334361562192621672993868479566688564752226111784486619789588318171745995253645886833872665447241245329935643883892447524286642296955354249478815116517315832179925494818748478164317669471654464867111924676961162162841232473474394739793968624974397916495667233337397241933765513777241916359166994384923869741468174653353541147616645393917694581811193977311981752554551499629219873391493426883886536219455848354426461562995284162323961773644581815633779762634745339565196798724847722781666948626231631632144371873154872575615636322965353254642186897127423352618879431499138418872356116624818733232445649188793318829748789349813295218673497291134164395739665667255443366383299669973689528188264386373591424149784473698487315316676637165317972648916116755224598519934598889627918883283534261513179931798591959489372165295"

format: db      "%ld", 0                    ; when we call printf, we'll use this format string
str:    db      "%c", 0

        section .text
main:
        mov     r15, 0                  ; r15 is reserved for the current character
        mov     r14, 0                  ; r14 is reserved for the opposite character
        mov     r10, 0                  ; initialize our sum to zero
        mov     rdi, 0                  ; initialize our loop index to zero

read_loop:
        mov     r15, [rdi + input]      ; load the character from the current index into r15
        and     r15, 0xff               ; we're only interested in one byte

        ; calculate the index of the other character
        mov     r8, rdi
        add     r8, 1041
        ; rdx:rax is the dividend; initialize it to have the value from r8 in its lower order word
        mov     rdx, 0
        mov     rax, r8
        mov     rcx, 2082
        idiv    rcx
        mov     r14, [rdx + input]      ; load the opposite character
        and     r14, 0xff               ; and truncate it to one byte

        ; print the string we read
        ;push    rdi
        ;push    rbx                     ; caller-save register
        ;lea     rdi, [str]           ; set format string
        ;mov     rsi, r15                ; set integer argument
        ;call    printf
        ;pop     rbx                     ; caller-save register
        ;pop     rdi

        ; if the two characters match, add them to the total in r10
        cmp     r14, r15        
        jnz     .no_match

        ; in the case of a match, add the integer value of the character in r15 to our total in r10
        sub     r15, 0x30               ; convert from digit char to equivalent int
        add     r10, r15

.no_match:
  
        ; when we get to the end of the input, terminate the loop
        cmp     rdi, 2082
        jz      final

        ; next loop iteration
        inc     rdi                     ; next item in the loop
        jmp     read_loop


final:

        ; print the string we read
        push    rax
        push    rbx                     ; caller-save register
        push    rcx
        lea     rdi, [format]           ; set format string
        mov     rsi, r10                ; set integer argument
        call    printf
        pop     rcx
        pop     rbx                     ; caller-save register
        pop     rax

        ret
