	.section	__TEXT,__text,regular,pure_instructions
	.globl	_umulx_u32
	.p2align	4, 0x90
	_umulx_u32:
	.cfi_startproc
	pushq	%rbp
	Ltmp0:
	.cfi_def_cfa_offset 16
	Ltmp1:
	.cfi_offset %rbp, -16
	movq	%rsp, %rbp
	Ltmp2:
	.cfi_def_cfa_register %rbp
	popq	%rbp
	jmp	__ZN59_$LT$u32$u20$as$u20$bitintr..alg..x86..bmi2..mulx..MULX$GT$4mulx17hea94817c060b5d20E
	.cfi_endproc

	.globl	_umulx_u64
	.p2align	4, 0x90
	_umulx_u64:
	.cfi_startproc
	pushq	%rbp
	Ltmp3:
	.cfi_def_cfa_offset 16
	Ltmp4:
	.cfi_offset %rbp, -16
	movq	%rsp, %rbp
	Ltmp5:
	.cfi_def_cfa_register %rbp
	pushq	%rbx
	pushq	%rax
	Ltmp6:
	.cfi_offset %rbx, -24
	movq	%rdi, %rbx
	callq	__ZN59_$LT$u64$u20$as$u20$bitintr..alg..x86..bmi2..mulx..MULX$GT$4mulx17h6efc2a80f3beee24E
	movq	%rbx, %rax
	addq	$8, %rsp
	popq	%rbx
	popq	%rbp
	retq
	.cfi_endproc


	.subsections_via_symbols
