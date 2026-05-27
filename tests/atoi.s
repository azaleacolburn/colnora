@start
    push 49 ; 1
    push 49 ; 1
    push 51 ; 3
    push 0 ; NULL
    sub %1 %stack 4
    bl @atoi
    eq %2 113
    assert

    push 57 ; 9
    push 56 ; 8
    push 57 ; 9
    push 54 ; 6
    push 53 ; 5
    push 53 ; 5
    push 0 ; NULL
    sub %1 %stack 7
    bl @atoi
    eq %2 989655
    assert

    push 45 ; '-'
    push 53 ; 5
    push 51 ; 3
    push 49 ; 1
    push 48 ; 0
    push 57 ; 9
    push 53 ; 5
    push 0 ; NULL
    sub %1 %stack 8
    bl @atoi
    put %2
    eq %2 -531095
    assert

    end

; Put the pointer to the null-termineted string in `%1`
; Returns the resultant number in `%2`
@atoi
    mov %2 0 ; Sum

    ; Check sign
    mov %3 [%1]
    neq %3 45 ; Ascii value of '-'
    brif @read_str
    add %1 %1 1

    @read_str
        sub %4 [%1] 48 ; Ascii value of '0'
        mul %2 %2 10
        add %2 %2 %4 ; Add our digit to our number
        add %1 %1 1 ; Increment our address
        neq [%1] 0 ; If we hit the null terminal, we're done
        brif @read_str

    ; If the first character is '-', number is negative
    neq %3 45
    brif @end
    neg %2 %2
    @end
    ret
