
.section .text
.global _start
.align 2

_start: mov	X0, #1
	adr	X1, helloworld
	mov	X2, #13
	mov	X16, #4
	svc	#0x80

	mov     X0, #0
	mov     X16, #1
	svc     #0x80

.sectio .data
helloworld:      .ascii  "Hello World!\n"
        
; source_filename = "hello_world.s"

; @helloworld = private unnamed_addr constant [14 x i8] c"Hello World!\0A\00"

; declare i64 @write(i32, i8*, i64)
; declare void @exit(i32)

; define i32 @main() {
; entry:
;     ; Write "Hello World!\n" to stdout
;     %1 = call i64 @write(i32 1, i8* getelementptr inbounds ([14 x i8], [14 x i8]* @helloworld, i64 0, i64 0), i64 13)

;     ; Exit with status code 0
;     call void @exit(i32 0)
;     unreachable
; }
