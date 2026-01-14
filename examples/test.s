	.text
	.file	"main"
	.globl	main
	.p2align	4, 0x90
	.type	main,@function
main:
	.cfi_startproc
	movq	$1, -8(%rsp)
	movq	$2, -16(%rsp)
	movq	$0, -24(%rsp)
	movq	-8(%rsp), %rax
	cmpq	-16(%rsp), %rax
	setg	%al
	andb	$1, %al
	movzbl	%al, %eax
	cmpq	$0, %rax
	je	.LBB0_2
	movl	$5, %eax
	movq	%rax, -40(%rsp)
	jmp	.LBB0_3
.LBB0_2:
	cmpq	$0, -24(%rsp)
	jne	.LBB0_4
	jmp	.LBB0_5
.LBB0_3:
	movq	-40(%rsp), %rax
	movq	%rax, -32(%rsp)
	movq	-32(%rsp), %rax
	retq
.LBB0_4:
	movl	$6, %eax
	movq	%rax, -48(%rsp)
	jmp	.LBB0_6
.LBB0_5:
	movl	$7, %eax
	movq	%rax, -48(%rsp)
	jmp	.LBB0_6
.LBB0_6:
	movq	-48(%rsp), %rax
	movq	%rax, -40(%rsp)
	jmp	.LBB0_3
.Lfunc_end0:
	.size	main, .Lfunc_end0-main
	.cfi_endproc

	.section	".note.GNU-stack","",@progbits
