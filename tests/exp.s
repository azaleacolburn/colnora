.text

@start
    mov %a 10
    mov %b 2
    bl @exp
    eq %c 100
    assert

    mov %a 10
    mov %b 0
    bl @exp
    eq %c 1
    assert

    mov %a 5
    mov %b 3
    bl @exp
    eq %c 125
    assert



    end

; Put `a` in `%a`
; Put `b` in `%b`
; Sets `%b` equal to `0`
; Leaves `%a` unchanged
; Returns `a^b` into `%c`
@exp
    mov %c 1
    @loop
        eq %b 0
        put %b
        retif
        sub %b %b 1
        mul %c %c %a
        br @loop
