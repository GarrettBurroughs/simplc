	.text
	.file	"main"
	.globl	main
	.p2align	4, 0x90
	.type	main,@function
main:
	.cfi_startproc
	movq	$1, -8(%rsp)
	movq	$5, -16(%rsp)
	cmpq	$1, -8(%rsp)
	sete	%al
	andb	$1, %al
	movzbl	%al, %eax
	movl	%eax, %ecx
	xorl	%eax, %eax
	cmpq	$0, %rcx
	movq	%rax, -24(%rsp)
	je	.LBB0_3
	cmpq	$5, -16(%rsp)
	sete	%al
	andb	$1, %al
	movzbl	%al, %eax
	cmpq	$0, %rax
	setne	%al
	andb	$1, %al
	movzbl	%al, %eax
	movq	%rax, -24(%rsp)
.LBB0_3:
	movq	-24(%rsp), %rax
	retq
.Lfunc_end0:
	.size	main, .Lfunc_end0-main
	.cfi_endproc

	.section	".note.GNU-stack","",@progbits
