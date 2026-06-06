.data

test_one: "113"
test_three: "-531095"

.text

@start
    mov %a test_one
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

    mov %a test_three
    bl @atoi
    put %b
    eq %b -531095
    assert

    end

; Put the pointer to the null-termineted string in `%a`
; Returns the resultant number in `%b`
; All other registers are guaranteed to not be modified
@atoi
    ; Save register values
    push %c
    push %d

    mov %b 0 ; Initialize sum

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

    ; Restore register values
    pop %d
    pop %c

    ret
