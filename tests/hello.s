.data

hello_text: "Hello World!"

.text

@start
    mov %a 4 ; syscall write
    mov %b 1 ; stdout
    mov %c hello_text ; buf ptr
    mov %d 12 ; buf len
    sys
    end
