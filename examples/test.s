	.text
	.file	"main"
	.globl	main
	.p2align	4, 0x90
	.type	main,@function
main:
	.cfi_startproc
	movq	$0, -8(%rsp)
	movq	-8(%rsp), %rax
	movq	-8(%rsp), %rcx
	addq	$1, %rcx
	movq	%rcx, -8(%rsp)
	movq	%rax, -16(%rsp)
	movl	$1, %eax
	retq
.Lfunc_end0:
	.size	main, .Lfunc_end0-main
	.cfi_endproc

	.section	".note.GNU-stack","",@progbits
