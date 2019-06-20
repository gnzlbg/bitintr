_umulx_u32:
 	pushq	%rbp
	movq	%rsp, %rbp
	movl	%edi, %ecx
	movl	%esi, %eax
	imulq	%rcx, %rax
	movq	%rax, %rdx
	shrq	$32, %rdx
	popq	%rbp
	retq
_umulx_u64:
  pushq	%rbp
	movq	%rsp, %rbp
	movq	%rsi, %rax
	mulq	%rdi
	popq	%rbp
	retq
