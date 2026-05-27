@start
    mov %1 10
    mov %2 2
    jmp @exp
@start_return
    eq %1 100
    assert
    end


; Put `a` in `%1`
; Put `b` in `%2`
; Returns `a^b` into `%1`
@exp
    mov %3 %2
    @loop
        sub %1 %1 1
        mul %2 %2 %3
        neq %1 0
        jmpif @loop
    jmp @start_return
