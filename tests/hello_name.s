.data

hello_text_one: "Hello "
hello_text_two: ", welcome to my program!"

.text

@start
    add %sp %sp 64 ; Allocate 64 bytes on the stack

    ; Setup Read Syscall
    mov %a 3 ; Read
    mov %b 0 ; Stdin
    mov %c %sp ; Buffer is on the stack, read writes backwards to buffer
    mov %d 10
    sys

    ; Store bottom of stack for printing
    sub %e %c 6

    ; Setup Write Syscall
    mov %a 4 ; Write
    mov %b 1 ; Stdout
    mov %c hello_text_one ; Text buffer
    mov %d 7 ; Size of buffer (including null)
    sys

    bl @print_stack

    ; Setup Write Syscall
    mov %a 4 ; Write
    mov %b 1 ; Stdout
    mov %c hello_text_two ; Text buffer
    mov %d 25 ; Size of buffer (including null)
    sys

; Put ptr in `%e`
; `%e` will be modified
@print_stack
    mov %d 0
    @count_stack
        add %d %d 1
        add %e %e 1
        neq [%e] 0
        brif @count_stack

    mov %a 4 ; Write
    mov %b 1 ; Stdout
    sub %c %e %f ; Text buffer is bottom of stack
    ; Size of buffer (including null) already found
    sys
    ret
