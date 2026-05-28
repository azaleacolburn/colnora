.text

@start
    push 49 ; 1
    push 49 ; 1
    push 51 ; 3
    push 0 ; NULL
    sub %a %sp 4
    bl @atoi
    eq %b 113
    assert

    push 57 ; 9
    push 56 ; 8
    push 57 ; 9
    push 54 ; 6
    push 53 ; 5
    push 53 ; 5
    push 0 ; NULL
    sub %a %sp 7
    bl @atoi
    eq %b 989655
    assert

    push 45 ; '-'
    push 53 ; 5
    push 51 ; 3
    push 49 ; 1
    push 48 ; 0
    push 57 ; 9
    push 53 ; 5
    push 0 ; NULL
    sub %a %sp 8
    bl @atoi
    put %b
    eq %b -531095
    assert

    end

; Put the pointer to the null-termineted string in `%a`
; Returns the resultant number in `%b`
@atoi
    mov %b 0 ; Sum

    ; Check sign
    mov %c [%a]
    neq %c 45 ; Ascii value of '-'
    brif @read_str
    add %a %a 1

    @read_str
        sub %d [%a] 48 ; Ascii value of '0'
        mul %b %b 10
        add %b %b %d ; Add our digit to our number
        add %a %a 1 ; Increment our address
        neq [%a] 0 ; If we hit the null terminal, we're done
        brif @read_str

    ; If the first character is '-', number is negative
    neq %c 45
    brif @end
    neg %b %b
    @end
    ret
