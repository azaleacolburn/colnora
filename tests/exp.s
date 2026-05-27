@start
    mov %1 10
    mov %2 2
    bl @exp
    eq %3 100
    assert

    mov %1 10
    mov %2 0
    bl @exp
    eq %3 1
    assert

    mov %1 5
    mov %2 3
    bl @exp
    eq %3 125
    assert



    end

; Put `a` in `%1`
; Put `b` in `%2`
; Sets `%2` equal to `0`
; Leaves `%1` unchanged
; Returns `a^b` into `%3`
@exp
    mov %3 1
    @loop
        eq %2 0
        put %2
        retif
        sub %2 %2 1
        mul %3 %3 %1
        br @loop
