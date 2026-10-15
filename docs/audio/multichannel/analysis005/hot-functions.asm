08000de0 <raylar_crcl_encoder_service::benchmark::nothing::h153434569c2cec30>:
 8000de0: b580         	push	{r7, lr}
 8000de2: 466f         	mov	r7, sp
 8000de4: bd80         	pop	{r7, pc}

08001680 <raylar_crcl_encoder_service::format::Encoder::data::ha65fe528befbf4e3>:
 8001680: b5f0         	push	{r4, r5, r6, r7, lr}
 8001682: af03         	add	r7, sp, #0xc
 8001684: e92d 0f00    	push.w	{r8, r9, r10, r11}
 8001688: b0a3         	sub	sp, #0x8c
 800168a: 6f16         	ldr	r6, [r2, #0x70]
 800168c: 2e06         	cmp	r6, #0x6
 800168e: d137         	bne	0x8001700 <raylar_crcl_encoder_service::format::Encoder::data::ha65fe528befbf4e3+0x80> @ imm = #0x6e
 8001690: f892 908c    	ldrb.w	r9, [r2, #0x8c]
 8001694: 7e0e         	ldrb	r6, [r1, #0x18]
 8001696: 45b1         	cmp	r9, r6
 8001698: d132         	bne	0x8001700 <raylar_crcl_encoder_service::format::Encoder::data::ha65fe528befbf4e3+0x80> @ imm = #0x64
 800169a: 7e4e         	ldrb	r6, [r1, #0x19]
 800169c: f892 508d    	ldrb.w	r5, [r2, #0x8d]
 80016a0: 42b5         	cmp	r5, r6
 80016a2: d12d         	bne	0x8001700 <raylar_crcl_encoder_service::format::Encoder::data::ha65fe528befbf4e3+0x80> @ imm = #0x5a
 80016a4: 694e         	ldr	r6, [r1, #0x14]
 80016a6: f8d2 5088    	ldr.w	r5, [r2, #0x88]
 80016aa: 42b5         	cmp	r5, r6
 80016ac: d128         	bne	0x8001700 <raylar_crcl_encoder_service::format::Encoder::data::ha65fe528befbf4e3+0x80> @ imm = #0x50
 80016ae: f892 508f    	ldrb.w	r5, [r2, #0x8f]
 80016b2: 7ece         	ldrb	r6, [r1, #0x1b]
 80016b4: 42b5         	cmp	r5, r6
 80016b6: d123         	bne	0x8001700 <raylar_crcl_encoder_service::format::Encoder::data::ha65fe528befbf4e3+0x80> @ imm = #0x46
 80016b8: 690e         	ldr	r6, [r1, #0x10]
 80016ba: f8d2 4084    	ldr.w	r4, [r2, #0x84]
 80016be: 42b4         	cmp	r4, r6
 80016c0: d11e         	bne	0x8001700 <raylar_crcl_encoder_service::format::Encoder::data::ha65fe528befbf4e3+0x80> @ imm = #0x3c
 80016c2: 7e8e         	ldrb	r6, [r1, #0x1a]
 80016c4: f892 408e    	ldrb.w	r4, [r2, #0x8e]
 80016c8: 42b4         	cmp	r4, r6
 80016ca: d119         	bne	0x8001700 <raylar_crcl_encoder_service::format::Encoder::data::ha65fe528befbf4e3+0x80> @ imm = #0x32
 80016cc: 6c56         	ldr	r6, [r2, #0x44]
 80016ce: f64a 2aab    	movw	r10, #0xaaab
 80016d2: f6ca 2aaa    	movt	r10, #0xaaaa
 80016d6: 9620         	str	r6, [sp, #0x80]
 80016d8: f8d7 b008    	ldr.w	r11, [r7, #0x8]
 80016dc: fba6 640a    	umull	r6, r4, r6, r10
 80016e0: ea4f 0494    	lsr.w	r4, r4, #0x2
 80016e4: 9421         	str	r4, [sp, #0x84]
 80016e6: b195         	cbz	r5, 0x800170e <raylar_crcl_encoder_service::format::Encoder::data::ha65fe528befbf4e3+0x8e> @ imm = #0x24
 80016e8: 4626         	mov	r6, r4
 80016ea: 2c00         	cmp	r4, #0x0
 80016ec: bf18         	it	ne
 80016ee: 2601         	movne	r6, #0x1
 80016f0: f04f 0e58    	mov.w	lr, #0x58
 80016f4: 1ba6         	subs	r6, r4, r6
 80016f6: f109 0402    	add.w	r4, r9, #0x2
 80016fa: fb04 9806    	mla	r8, r4, r6, r9
 80016fe: e00a         	b	0x8001716 <raylar_crcl_encoder_service::format::Encoder::data::ha65fe528befbf4e3+0x96> @ imm = #0x14
 8001700: f240 5101    	movw	r1, #0x501
 8001704: 8001         	strh	r1, [r0]
 8001706: b023         	add	sp, #0x8c
 8001708: e8bd 0f00    	pop.w	{r8, r9, r10, r11}
 800170c: bdf0         	pop	{r4, r5, r6, r7, pc}
 800170e: fb04 f809    	mul	r8, r4, r9
 8001712: f04f 0e60    	mov.w	lr, #0x60
 8001716: ea4f 06d8    	lsr.w	r6, r8, #0x3
 800171a: f04f 0c01    	mov.w	r12, #0x1
 800171e: ea5f 7448    	lsls.w	r4, r8, #0x1d
 8001722: bf18         	it	ne
 8001724: eb0c 06d8    	addne.w	r6, r12, r8, lsr #3
 8001728: eb06 0686    	add.w	r6, r6, r6, lsl #2
 800172c: 4476         	add	r6, lr
 800172e: 45b3         	cmp	r11, r6
 8001730: d206         	bhs	0x8001740 <raylar_crcl_encoder_service::format::Encoder::data::ha65fe528befbf4e3+0xc0> @ imm = #0xc
 8001732: f240 2101    	movw	r1, #0x201
 8001736: 8001         	strh	r1, [r0]
 8001738: b023         	add	sp, #0x8c
 800173a: e8bd 0f00    	pop.w	{r8, r9, r10, r11}
 800173e: bdf0         	pop	{r4, r5, r6, r7, pc}
 8001740: 688c         	ldr	r4, [r1, #0x8]
 8001742: 1c66         	adds	r6, r4, #0x1
 8001744: bf1c         	itt	ne
 8001746: 68ce         	ldrne	r6, [r1, #0xc]
 8001748: f116 0601    	addsne.w	r6, r6, #0x1
 800174c: d106         	bne	0x800175c <raylar_crcl_encoder_service::format::Encoder::data::ha65fe528befbf4e3+0xdc> @ imm = #0xc
 800174e: f240 4101    	movw	r1, #0x401
 8001752: 8001         	strh	r1, [r0]
 8001754: b023         	add	sp, #0x8c
 8001756: e8bd 0f00    	pop.w	{r8, r9, r10, r11}
 800175a: bdf0         	pop	{r4, r5, r6, r7, pc}
 800175c: e9d1 ec00    	ldrd	lr, r12, [r1]
 8001760: 2100         	movs	r1, #0x0
 8001762: 9e21         	ldr	r6, [sp, #0x84]
 8001764: eb16 060e    	adds.w	r6, r6, lr
 8001768: f15c 0600    	adcs	r6, r12, #0x0
 800176c: f151 0100    	adcs	r1, r1, #0x0
 8001770: d1ed         	bne	0x800174e <raylar_crcl_encoder_service::format::Encoder::data::ha65fe528befbf4e3+0xce> @ imm = #-0x26
 8001772: 4656         	mov	r6, r10
 8001774: f04f 0a5c    	mov.w	r10, #0x5c
 8001778: 2d00         	cmp	r5, #0x0
 800177a: e9cd ec08    	strd	lr, r12, [sp, #32]
 800177e: 940a         	str	r4, [sp, #0x28]
 8001780: 9013         	str	r0, [sp, #0x4c]
 8001782: 9507         	str	r5, [sp, #0x1c]
 8001784: bf18         	it	ne
 8001786: f04f 0a58    	movne.w	r10, #0x58
 800178a: 45da         	cmp	r10, r11
 800178c: f200 820a    	bhi.w	0x8001ba4 <raylar_crcl_encoder_service::format::Encoder::data::ha65fe528befbf4e3+0x524> @ imm = #0x414
 8001790: 6c10         	ldr	r0, [r2, #0x40]
 8001792: 4651         	mov	r1, r10
 8001794: 901d         	str	r0, [sp, #0x74]
 8001796: 4618         	mov	r0, r3
 8001798: e9cd 2317    	strd	r2, r3, [sp, #92]
 800179c: f00b fd33    	bl	0x800d206 <__aeabi_memclr> @ imm = #0xba66
 80017a0: f1ab 001c    	sub.w	r0, r11, #0x1c
 80017a4: 9b18         	ldr	r3, [sp, #0x60]
 80017a6: f04f 0c48    	mov.w	r12, #0x48
 80017aa: 2500         	movs	r5, #0x0
 80017ac: fba0 0106    	umull	r0, r1, r0, r6
 80017b0: f1ab 0018    	sub.w	r0, r11, #0x18
 80017b4: fba0 0206    	umull	r0, r2, r0, r6
 80017b8: 08c8         	lsrs	r0, r1, #0x3
 80017ba: 9014         	str	r0, [sp, #0x50]
 80017bc: 08d0         	lsrs	r0, r2, #0x3
 80017be: 9016         	str	r0, [sp, #0x58]
 80017c0: f1c9 0020    	rsb.w	r0, r9, #0x20
 80017c4: 9010         	str	r0, [sp, #0x40]
 80017c6: 9817         	ldr	r0, [sp, #0x5c]
 80017c8: f100 0148    	add.w	r1, r0, #0x48
 80017cc: 201c         	movs	r0, #0x1c
 80017ce: eb05 0245    	add.w	r2, r5, r5, lsl #1
 80017d2: eb00 0082    	add.w	r0, r0, r2, lsl #2
 80017d6: 4558         	cmp	r0, r11
 80017d8: f080 81f4    	bhs.w	0x8001bc4 <raylar_crcl_encoder_service::format::Encoder::data::ha65fe528befbf4e3+0x544> @ imm = #0x3e8
 80017dc: f891 8005    	ldrb.w	r8, [r1, #0x5]
 80017e0: 0096         	lsls	r6, r2, #0x2
 80017e2: 1caa         	adds	r2, r5, #0x2
 80017e4: 541a         	strb	r2, [r3, r0]
 80017e6: f106 001d    	add.w	r0, r6, #0x1d
 80017ea: 4558         	cmp	r0, r11
 80017ec: f080 81f1    	bhs.w	0x8001bd2 <raylar_crcl_encoder_service::format::Encoder::data::ha65fe528befbf4e3+0x552> @ imm = #0x3e2
 80017f0: f803 8000    	strb.w	r8, [r3, r0]
 80017f4: f106 001f    	add.w	r0, r6, #0x1f
 80017f8: 4558         	cmp	r0, r11
 80017fa: f8cd c064    	str.w	r12, [sp, #0x64]
 80017fe: f080 81ef    	bhs.w	0x8001be0 <raylar_crcl_encoder_service::format::Encoder::data::ha65fe528befbf4e3+0x560> @ imm = #0x3de
 8001802: 790a         	ldrb	r2, [r1, #0x4]
 8001804: f8dd e080    	ldr.w	lr, [sp, #0x80]
 8001808: 541a         	strb	r2, [r3, r0]
 800180a: f106 0020    	add.w	r0, r6, #0x20
 800180e: 921b         	str	r2, [sp, #0x6c]
 8001810: 9a16         	ldr	r2, [sp, #0x58]
 8001812: 4295         	cmp	r5, r2
 8001814: f000 8195    	beq.w	0x8001b42 <raylar_crcl_encoder_service::format::Encoder::data::ha65fe528befbf4e3+0x4c2> @ imm = #0x32a
 8001818: 6809         	ldr	r1, [r1]
 800181a: 45d3         	cmp	r11, r10
 800181c: 5019         	str	r1, [r3, r0]
 800181e: f0c0 81b8    	blo.w	0x8001b92 <raylar_crcl_encoder_service::format::Encoder::data::ha65fe528befbf4e3+0x512> @ imm = #0x370
 8001822: ebab 0c0a    	sub.w	r12, r11, r10
 8001826: 4453         	add	r3, r10
 8001828: 2000         	movs	r0, #0x0
 800182a: ebb0 7f5c    	cmp.w	r0, r12, lsr #29
 800182e: bf14         	ite	ne
 8001830: f04f 31ff    	movne.w	r1, #0xffffffff
 8001834: ea4f 01cc    	lsleq.w	r1, r12, #0x3
 8001838: 1c68         	adds	r0, r5, #0x1
 800183a: f1be 0f06    	cmp.w	lr, #0x6
 800183e: 901c         	str	r0, [sp, #0x70]
 8001840: d235         	bhs	0x80018ae <raylar_crcl_encoder_service::format::Encoder::data::ha65fe528befbf4e3+0x22e> @ imm = #0x6a
 8001842: 2000         	movs	r0, #0x0
 8001844: 2100         	movs	r1, #0x0
 8001846: f04f 0b00    	mov.w	r11, #0x0
 800184a: ea5f 724b    	lsls.w	r2, r11, #0x1d
 800184e: ea4f 04db    	lsr.w	r4, r11, #0x3
 8001852: f04f 0201    	mov.w	r2, #0x1
 8001856: bf18         	it	ne
 8001858: eb02 04db    	addne.w	r4, r2, r11, lsr #3
 800185c: eb14 0801    	adds.w	r8, r4, r1
 8001860: f080 817c    	bhs.w	0x8001b5c <raylar_crcl_encoder_service::format::Encoder::data::ha65fe528befbf4e3+0x4dc> @ imm = #0x2f8
 8001864: 45e0         	cmp	r8, r12
 8001866: f200 8179    	bhi.w	0x8001b5c <raylar_crcl_encoder_service::format::Encoder::data::ha65fe528befbf4e3+0x4dc> @ imm = #0x2f2
 800186a: f8d7 b008    	ldr.w	r11, [r7, #0x8]
 800186e: 2c05         	cmp	r4, #0x5
 8001870: 9022         	str	r0, [sp, #0x88]
 8001872: f080 817c    	bhs.w	0x8001b6e <raylar_crcl_encoder_service::format::Encoder::data::ha65fe528befbf4e3+0x4ee> @ imm = #0x2f8
 8001876: 1858         	adds	r0, r3, r1
 8001878: a922         	add	r1, sp, #0x88
 800187a: 4622         	mov	r2, r4
 800187c: f00b fb50    	bl	0x800cf20 <__aeabi_memcpy> @ imm = #0xb6a0
 8001880: 9a19         	ldr	r2, [sp, #0x64]
 8001882: f106 0024    	add.w	r0, r6, #0x24
 8001886: 9914         	ldr	r1, [sp, #0x50]
 8001888: 428d         	cmp	r5, r1
 800188a: f000 8179    	beq.w	0x8001b80 <raylar_crcl_encoder_service::format::Encoder::data::ha65fe528befbf4e3+0x500> @ imm = #0x2f2
 800188e: 9b18         	ldr	r3, [sp, #0x60]
 8001890: f102 0c08    	add.w	r12, r2, #0x8
 8001894: 9d1c         	ldr	r5, [sp, #0x70]
 8001896: 44c2         	add	r10, r8
 8001898: f1bc 0f70    	cmp.w	r12, #0x70
 800189c: f843 8000    	str.w	r8, [r3, r0]
 80018a0: 9817         	ldr	r0, [sp, #0x5c]
 80018a2: eb00 010c    	add.w	r1, r0, r12
 80018a6: f04f 001c    	mov.w	r0, #0x1c
 80018aa: d190         	bne	0x80017ce <raylar_crcl_encoder_service::format::Encoder::data::ha65fe528befbf4e3+0x14e> @ imm = #-0xe0
 80018ac: e12b         	b	0x8001b06 <raylar_crcl_encoder_service::format::Encoder::data::ha65fe528befbf4e3+0x486> @ imm = #0x256
 80018ae: e9cd c11e    	strd	r12, r1, [sp, #120]
 80018b2: 2001         	movs	r0, #0x1
 80018b4: 991b         	ldr	r1, [sp, #0x6c]
 80018b6: f04f 0c00    	mov.w	r12, #0x0
 80018ba: f8cd a034    	str.w	r10, [sp, #0x34]
 80018be: f04f 0b00    	mov.w	r11, #0x0
 80018c2: f8dd a070    	ldr.w	r10, [sp, #0x70]
 80018c6: 4088         	lsls	r0, r1
 80018c8: 9315         	str	r3, [sp, #0x54]
 80018ca: 9012         	str	r0, [sp, #0x48]
 80018cc: 3801         	subs	r0, #0x1
 80018ce: 9011         	str	r0, [sp, #0x44]
 80018d0: f1c1 001f    	rsb.w	r0, r1, #0x1f
 80018d4: e9cd 650b    	strd	r6, r5, [sp, #44]
 80018d8: 2300         	movs	r3, #0x0
 80018da: 900f         	str	r0, [sp, #0x3c]
 80018dc: 1c48         	adds	r0, r1, #0x1
 80018de: 2100         	movs	r1, #0x0
 80018e0: 2500         	movs	r5, #0x0
 80018e2: 901a         	str	r0, [sp, #0x68]
 80018e4: e005         	b	0x80018f2 <raylar_crcl_encoder_service::format::Encoder::data::ha65fe528befbf4e3+0x272> @ imm = #0xa
 80018e6: 44cb         	add	r11, r9
 80018e8: 9821         	ldr	r0, [sp, #0x84]
 80018ea: 4623         	mov	r3, r4
 80018ec: 4284         	cmp	r4, r0
 80018ee: f000 80e0    	beq.w	0x8001ab2 <raylar_crcl_encoder_service::format::Encoder::data::ha65fe528befbf4e3+0x432> @ imm = #0x1c0
 80018f2: eb03 0043    	add.w	r0, r3, r3, lsl #1
 80018f6: eb0a 0040    	add.w	r0, r10, r0, lsl #1
 80018fa: 4570         	cmp	r0, lr
 80018fc: f080 815b    	bhs.w	0x8001bb6 <raylar_crcl_encoder_service::format::Encoder::data::ha65fe528befbf4e3+0x536> @ imm = #0x2b6
 8001900: 9a1d         	ldr	r2, [sp, #0x74]
 8001902: 4666         	mov	r6, r12
 8001904: 1c5c         	adds	r4, r3, #0x1
 8001906: f852 c020    	ldr.w	r12, [r2, r0, lsl #2]
 800190a: 4660         	mov	r0, r12
 800190c: e8df f008    	tbb	[pc, r8]
 8001910: 02 06 03 37  	.word	0x37030602
 8001914: e7e8         	b	0x80018e8 <raylar_crcl_encoder_service::format::Encoder::data::ha65fe528befbf4e3+0x268> @ imm = #-0x30
 8001916: b393         	cbz	r3, 0x800197e <raylar_crcl_encoder_service::format::Encoder::data::ha65fe528befbf4e3+0x2fe> @ imm = #0x64
 8001918: ebac 0006    	sub.w	r0, r12, r6
 800191c: 9b1b         	ldr	r3, [sp, #0x6c]
 800191e: 2b1f         	cmp	r3, #0x1f
 8001920: f200 80e0    	bhi.w	0x8001ae4 <raylar_crcl_encoder_service::format::Encoder::data::ha65fe528befbf4e3+0x464> @ imm = #0x1c0
 8001924: 0042         	lsls	r2, r0, #0x1
 8001926: ea82 7ae0    	eor.w	r10, r2, r0, asr #31
 800192a: 981a         	ldr	r0, [sp, #0x68]
 800192c: fa2a f603    	lsr.w	r6, r10, r3
 8001930: 2302         	movs	r3, #0x2
 8001932: 1980         	adds	r0, r0, r6
 8001934: f080 80d7    	bhs.w	0x8001ae6 <raylar_crcl_encoder_service::format::Encoder::data::ha65fe528befbf4e3+0x466> @ imm = #0x1ae
 8001938: 9a1f         	ldr	r2, [sp, #0x7c]
 800193a: 4282         	cmp	r2, r0
 800193c: f0c0 80d3    	blo.w	0x8001ae6 <raylar_crcl_encoder_service::format::Encoder::data::ha65fe528befbf4e3+0x466> @ imm = #0x1a6
 8001940: 1a12         	subs	r2, r2, r0
 8001942: 921f         	str	r2, [sp, #0x7c]
 8001944: 9a11         	ldr	r2, [sp, #0x44]
 8001946: 2821         	cmp	r0, #0x21
 8001948: 9b12         	ldr	r3, [sp, #0x48]
 800194a: ea02 020a    	and.w	r2, r2, r10
 800194e: ea43 0302    	orr.w	r3, r3, r2
 8001952: d244         	bhs	0x80019de <raylar_crcl_encoder_service::format::Encoder::data::ha65fe528befbf4e3+0x35e> @ imm = #0x88
 8001954: f8dd e080    	ldr.w	lr, [sp, #0x80]
 8001958: 2800         	cmp	r0, #0x0
 800195a: f8dd a070    	ldr.w	r10, [sp, #0x70]
 800195e: d0c3         	beq	0x80018e8 <raylar_crcl_encoder_service::format::Encoder::data::ha65fe528befbf4e3+0x268> @ imm = #-0x7a
 8001960: f1c0 0220    	rsb.w	r2, r0, #0x20
 8001964: 930e         	str	r3, [sp, #0x38]
 8001966: fa03 f202    	lsl.w	r2, r3, r2
 800196a: f00b 031f    	and	r3, r11, #0x1f
 800196e: 40da         	lsrs	r2, r3
 8001970: f1cb 0320    	rsb.w	r3, r11, #0x20
 8001974: 4315         	orrs	r5, r2
 8001976: 4298         	cmp	r0, r3
 8001978: d255         	bhs	0x8001a26 <raylar_crcl_encoder_service::format::Encoder::data::ha65fe528befbf4e3+0x3a6> @ imm = #0xaa
 800197a: 4483         	add	r11, r0
 800197c: e7b4         	b	0x80018e8 <raylar_crcl_encoder_service::format::Encoder::data::ha65fe528befbf4e3+0x268> @ imm = #-0x98
 800197e: 981f         	ldr	r0, [sp, #0x7c]
 8001980: f1b9 0f20    	cmp.w	r9, #0x20
 8001984: f200 80b7    	bhi.w	0x8001af6 <raylar_crcl_encoder_service::format::Encoder::data::ha65fe528befbf4e3+0x476> @ imm = #0x16e
 8001988: 4548         	cmp	r0, r9
 800198a: f0c0 80b4    	blo.w	0x8001af6 <raylar_crcl_encoder_service::format::Encoder::data::ha65fe528befbf4e3+0x476> @ imm = #0x168
 800198e: eba0 0009    	sub.w	r0, r0, r9
 8001992: f1b9 0f00    	cmp.w	r9, #0x0
 8001996: 901f         	str	r0, [sp, #0x7c]
 8001998: d0a6         	beq	0x80018e8 <raylar_crcl_encoder_service::format::Encoder::data::ha65fe528befbf4e3+0x268> @ imm = #-0xb4
 800199a: 9810         	ldr	r0, [sp, #0x40]
 800199c: f00b 021f    	and	r2, r11, #0x1f
 80019a0: fa0c f000    	lsl.w	r0, r12, r0
 80019a4: 40d0         	lsrs	r0, r2
 80019a6: 4305         	orrs	r5, r0
 80019a8: f1cb 0020    	rsb.w	r0, r11, #0x20
 80019ac: 4581         	cmp	r9, r0
 80019ae: d39a         	blo	0x80018e6 <raylar_crcl_encoder_service::format::Encoder::data::ha65fe528befbf4e3+0x266> @ imm = #-0xcc
 80019b0: 9b15         	ldr	r3, [sp, #0x54]
 80019b2: 1d0e         	adds	r6, r1, #0x4
 80019b4: f111 0f05    	cmn.w	r1, #0x5
 80019b8: bf9c         	itt	ls
 80019ba: 9a1e         	ldrls	r2, [sp, #0x78]
 80019bc: 4296         	cmpls	r6, r2
 80019be: f200 8088    	bhi.w	0x8001ad2 <raylar_crcl_encoder_service::format::Encoder::data::ha65fe528befbf4e3+0x452> @ imm = #0x110
 80019c2: eba9 0b00    	sub.w	r11, r9, r0
 80019c6: ba2a         	rev	r2, r5
 80019c8: 505a         	str	r2, [r3, r1]
 80019ca: f1cb 0100    	rsb.w	r1, r11, #0x0
 80019ce: ebb9 0500    	subs.w	r5, r9, r0
 80019d2: f001 011f    	and	r1, r1, #0x1f
 80019d6: bf18         	it	ne
 80019d8: fa0c f501    	lslne.w	r5, r12, r1
 80019dc: e067         	b	0x8001aae <raylar_crcl_encoder_service::format::Encoder::data::ha65fe528befbf4e3+0x42e> @ imm = #0xce
 80019de: 930e         	str	r3, [sp, #0x38]
 80019e0: 4608         	mov	r0, r1
 80019e2: f8dd e080    	ldr.w	lr, [sp, #0x80]
 80019e6: 2e20         	cmp	r6, #0x20
 80019e8: f8dd a070    	ldr.w	r10, [sp, #0x70]
 80019ec: 9b15         	ldr	r3, [sp, #0x54]
 80019ee: d313         	blo	0x8001a18 <raylar_crcl_encoder_service::format::Encoder::data::ha65fe528befbf4e3+0x398> @ imm = #0x26
 80019f0: f1bb 0f20    	cmp.w	r11, #0x20
 80019f4: d902         	bls	0x80019fc <raylar_crcl_encoder_service::format::Encoder::data::ha65fe528befbf4e3+0x37c> @ imm = #0x4
 80019f6: f10b 0b20    	add.w	r11, r11, #0x20
 80019fa: e00a         	b	0x8001a12 <raylar_crcl_encoder_service::format::Encoder::data::ha65fe528befbf4e3+0x392> @ imm = #0x14
 80019fc: 1d01         	adds	r1, r0, #0x4
 80019fe: f110 0f05    	cmn.w	r0, #0x5
 8001a02: bf9c         	itt	ls
 8001a04: 9a1e         	ldrls	r2, [sp, #0x78]
 8001a06: 4291         	cmpls	r1, r2
 8001a08: d85c         	bhi	0x8001ac4 <raylar_crcl_encoder_service::format::Encoder::data::ha65fe528befbf4e3+0x444> @ imm = #0xb8
 8001a0a: ba2a         	rev	r2, r5
 8001a0c: 501a         	str	r2, [r3, r0]
 8001a0e: 2500         	movs	r5, #0x0
 8001a10: 4608         	mov	r0, r1
 8001a12: 3e20         	subs	r6, #0x20
 8001a14: 2e1f         	cmp	r6, #0x1f
 8001a16: d8eb         	bhi	0x80019f0 <raylar_crcl_encoder_service::format::Encoder::data::ha65fe528befbf4e3+0x370> @ imm = #-0x2a
 8001a18: b32e         	cbz	r6, 0x8001a66 <raylar_crcl_encoder_service::format::Encoder::data::ha65fe528befbf4e3+0x3e6> @ imm = #0x4a
 8001a1a: f1cb 0020    	rsb.w	r0, r11, #0x20
 8001a1e: 4286         	cmp	r6, r0
 8001a20: d213         	bhs	0x8001a4a <raylar_crcl_encoder_service::format::Encoder::data::ha65fe528befbf4e3+0x3ca> @ imm = #0x26
 8001a22: 44b3         	add	r11, r6
 8001a24: e01f         	b	0x8001a66 <raylar_crcl_encoder_service::format::Encoder::data::ha65fe528befbf4e3+0x3e6> @ imm = #0x3e
 8001a26: 1d0e         	adds	r6, r1, #0x4
 8001a28: f111 0f05    	cmn.w	r1, #0x5
 8001a2c: bf9c         	itt	ls
 8001a2e: 9a1e         	ldrls	r2, [sp, #0x78]
 8001a30: 4296         	cmpls	r6, r2
 8001a32: d84e         	bhi	0x8001ad2 <raylar_crcl_encoder_service::format::Encoder::data::ha65fe528befbf4e3+0x452> @ imm = #0x9c
 8001a34: ba2a         	rev	r2, r5
 8001a36: 9d15         	ldr	r5, [sp, #0x54]
 8001a38: eba0 0b03    	sub.w	r11, r0, r3
 8001a3c: 506a         	str	r2, [r5, r1]
 8001a3e: f1cb 0100    	rsb.w	r1, r11, #0x0
 8001a42: f001 011f    	and	r1, r1, #0x1f
 8001a46: 1ac5         	subs	r5, r0, r3
 8001a48: e02d         	b	0x8001aa6 <raylar_crcl_encoder_service::format::Encoder::data::ha65fe528befbf4e3+0x426> @ imm = #0x5a
 8001a4a: f101 0b04    	add.w	r11, r1, #0x4
 8001a4e: f111 0f05    	cmn.w	r1, #0x5
 8001a52: bf9c         	itt	ls
 8001a54: 9a1e         	ldrls	r2, [sp, #0x78]
 8001a56: 4593         	cmpls	r11, r2
 8001a58: d86a         	bhi	0x8001b30 <raylar_crcl_encoder_service::format::Encoder::data::ha65fe528befbf4e3+0x4b0> @ imm = #0xd4
 8001a5a: ba2a         	rev	r2, r5
 8001a5c: 505a         	str	r2, [r3, r1]
 8001a5e: 4659         	mov	r1, r11
 8001a60: eba6 0b00    	sub.w	r11, r6, r0
 8001a64: 2500         	movs	r5, #0x0
 8001a66: e9dd 200e    	ldrd	r2, r0, [sp, #56]
 8001a6a: fa02 f000    	lsl.w	r0, r2, r0
 8001a6e: f00b 021f    	and	r2, r11, #0x1f
 8001a72: 40d0         	lsrs	r0, r2
 8001a74: 9a1a         	ldr	r2, [sp, #0x68]
 8001a76: 4305         	orrs	r5, r0
 8001a78: f1cb 0020    	rsb.w	r0, r11, #0x20
 8001a7c: 4282         	cmp	r2, r0
 8001a7e: d201         	bhs	0x8001a84 <raylar_crcl_encoder_service::format::Encoder::data::ha65fe528befbf4e3+0x404> @ imm = #0x2
 8001a80: 4493         	add	r11, r2
 8001a82: e731         	b	0x80018e8 <raylar_crcl_encoder_service::format::Encoder::data::ha65fe528befbf4e3+0x268> @ imm = #-0x19e
 8001a84: 1d0e         	adds	r6, r1, #0x4
 8001a86: f111 0f05    	cmn.w	r1, #0x5
 8001a8a: bf9c         	itt	ls
 8001a8c: 9a1e         	ldrls	r2, [sp, #0x78]
 8001a8e: 4296         	cmpls	r6, r2
 8001a90: d81f         	bhi	0x8001ad2 <raylar_crcl_encoder_service::format::Encoder::data::ha65fe528befbf4e3+0x452> @ imm = #0x3e
 8001a92: ba2a         	rev	r2, r5
 8001a94: 505a         	str	r2, [r3, r1]
 8001a96: 9a1a         	ldr	r2, [sp, #0x68]
 8001a98: eba2 0b00    	sub.w	r11, r2, r0
 8001a9c: 1a15         	subs	r5, r2, r0
 8001a9e: f1cb 0100    	rsb.w	r1, r11, #0x0
 8001aa2: f001 011f    	and	r1, r1, #0x1f
 8001aa6: 980e         	ldr	r0, [sp, #0x38]
 8001aa8: bf18         	it	ne
 8001aaa: fa00 f501    	lslne.w	r5, r0, r1
 8001aae: 4631         	mov	r1, r6
 8001ab0: e71a         	b	0x80018e8 <raylar_crcl_encoder_service::format::Encoder::data::ha65fe528befbf4e3+0x268> @ imm = #-0x1cc
 8001ab2: ba28         	rev	r0, r5
 8001ab4: 9b15         	ldr	r3, [sp, #0x54]
 8001ab6: f8dd a034    	ldr.w	r10, [sp, #0x34]
 8001aba: e9dd 650b    	ldrd	r6, r5, [sp, #44]
 8001abe: f8dd c078    	ldr.w	r12, [sp, #0x78]
 8001ac2: e6c2         	b	0x800184a <raylar_crcl_encoder_service::format::Encoder::data::ha65fe528befbf4e3+0x1ca> @ imm = #-0x27c
 8001ac4: 9a1e         	ldr	r2, [sp, #0x78]
 8001ac6: f64d 73c0    	movw	r3, #0xdfc0
 8001aca: f6c0 0300    	movt	r3, #0x800
 8001ace: f00a fcf6    	bl	0x800c4be <core::slice::index::slice_index_fail> @ imm = #0xa9ec
 8001ad2: 9a1e         	ldr	r2, [sp, #0x78]
 8001ad4: f64d 73c0    	movw	r3, #0xdfc0
 8001ad8: 4608         	mov	r0, r1
 8001ada: f6c0 0300    	movt	r3, #0x800
 8001ade: 4631         	mov	r1, r6
 8001ae0: f00a fced    	bl	0x800c4be <core::slice::index::slice_index_fail> @ imm = #0xa9da
 8001ae4: 2300         	movs	r3, #0x0
 8001ae6: 9913         	ldr	r1, [sp, #0x4c]
 8001ae8: 2001         	movs	r0, #0x1
 8001aea: 7008         	strb	r0, [r1]
 8001aec: 704b         	strb	r3, [r1, #0x1]
 8001aee: b023         	add	sp, #0x8c
 8001af0: e8bd 0f00    	pop.w	{r8, r9, r10, r11}
 8001af4: bdf0         	pop	{r4, r5, r6, r7, pc}
 8001af6: 9913         	ldr	r1, [sp, #0x4c]
 8001af8: f240 2001    	movw	r0, #0x201
 8001afc: 8008         	strh	r0, [r1]
 8001afe: b023         	add	sp, #0x8c
 8001b00: e8bd 0f00    	pop.w	{r8, r9, r10, r11}
 8001b04: bdf0         	pop	{r4, r5, r6, r7, pc}
 8001b06: 2001         	movs	r0, #0x1
 8001b08: 9907         	ldr	r1, [sp, #0x1c]
 8001b0a: 9000         	str	r0, [sp]
 8001b0c: 461a         	mov	r2, r3
 8001b0e: 980a         	ldr	r0, [sp, #0x28]
 8001b10: 465b         	mov	r3, r11
 8001b12: 9001         	str	r0, [sp, #0x4]
 8001b14: 9808         	ldr	r0, [sp, #0x20]
 8001b16: 9002         	str	r0, [sp, #0x8]
 8001b18: 9809         	ldr	r0, [sp, #0x24]
 8001b1a: 9003         	str	r0, [sp, #0xc]
 8001b1c: 9821         	ldr	r0, [sp, #0x84]
 8001b1e: e9cd 0a04    	strd	r0, r10, [sp, #16]
 8001b22: 9813         	ldr	r0, [sp, #0x4c]
 8001b24: f7ff fc96    	bl	0x8001454 <raylar_crcl_encoder_service::format::finish_record::hdf4f4fad6aa1e26d> @ imm = #-0x6d4
 8001b28: b023         	add	sp, #0x8c
 8001b2a: e8bd 0f00    	pop.w	{r8, r9, r10, r11}
 8001b2e: bdf0         	pop	{r4, r5, r6, r7, pc}
 8001b30: 9a1e         	ldr	r2, [sp, #0x78]
 8001b32: f64d 73c0    	movw	r3, #0xdfc0
 8001b36: 4608         	mov	r0, r1
 8001b38: f6c0 0300    	movt	r3, #0x800
 8001b3c: 4659         	mov	r1, r11
 8001b3e: f00a fcbe    	bl	0x800c4be <core::slice::index::slice_index_fail> @ imm = #0xa97c
 8001b42: 9916         	ldr	r1, [sp, #0x58]
 8001b44: 2224         	movs	r2, #0x24
 8001b46: eb01 0141    	add.w	r1, r1, r1, lsl #1
 8001b4a: eb02 0181    	add.w	r1, r2, r1, lsl #2
 8001b4e: f24e 0390    	movw	r3, #0xe090
 8001b52: 465a         	mov	r2, r11
 8001b54: f6c0 0300    	movt	r3, #0x800
 8001b58: f00a fcb1    	bl	0x800c4be <core::slice::index::slice_index_fail> @ imm = #0xa962
 8001b5c: f64d 73e0    	movw	r3, #0xdfe0
 8001b60: 4608         	mov	r0, r1
 8001b62: f6c0 0300    	movt	r3, #0x800
 8001b66: 4641         	mov	r1, r8
 8001b68: 4662         	mov	r2, r12
 8001b6a: f00a fca8    	bl	0x800c4be <core::slice::index::slice_index_fail> @ imm = #0xa950
 8001b6e: f64d 73d0    	movw	r3, #0xdfd0
 8001b72: 2000         	movs	r0, #0x0
 8001b74: f6c0 0300    	movt	r3, #0x800
 8001b78: 4621         	mov	r1, r4
 8001b7a: 2204         	movs	r2, #0x4
 8001b7c: f00a fc9f    	bl	0x800c4be <core::slice::index::slice_index_fail> @ imm = #0xa93e
 8001b80: f106 0128    	add.w	r1, r6, #0x28
 8001b84: f24e 0390    	movw	r3, #0xe090
 8001b88: 465a         	mov	r2, r11
 8001b8a: f6c0 0300    	movt	r3, #0x800
 8001b8e: f00a fc96    	bl	0x800c4be <core::slice::index::slice_index_fail> @ imm = #0xa92c
 8001b92: f24e 43e0    	movw	r3, #0xe4e0
 8001b96: 4650         	mov	r0, r10
 8001b98: f6c0 0300    	movt	r3, #0x800
 8001b9c: 4659         	mov	r1, r11
 8001b9e: 465a         	mov	r2, r11
 8001ba0: f00a fc8d    	bl	0x800c4be <core::slice::index::slice_index_fail> @ imm = #0xa91a
 8001ba4: f24e 43f0    	movw	r3, #0xe4f0
 8001ba8: 2000         	movs	r0, #0x0
 8001baa: f6c0 0300    	movt	r3, #0x800
 8001bae: 4651         	mov	r1, r10
 8001bb0: 465a         	mov	r2, r11
 8001bb2: f00a fc84    	bl	0x800c4be <core::slice::index::slice_index_fail> @ imm = #0xa908
 8001bb6: f24e 42d0    	movw	r2, #0xe4d0
 8001bba: 4671         	mov	r1, lr
 8001bbc: f6c0 0200    	movt	r2, #0x800
 8001bc0: f00a f990    	bl	0x800bee4 <core::panicking::panic_bounds_check> @ imm = #0xa320
 8001bc4: f24e 42a0    	movw	r2, #0xe4a0
 8001bc8: 4659         	mov	r1, r11
 8001bca: f6c0 0200    	movt	r2, #0x800
 8001bce: f00a f989    	bl	0x800bee4 <core::panicking::panic_bounds_check> @ imm = #0xa312
 8001bd2: f24e 42b0    	movw	r2, #0xe4b0
 8001bd6: 4659         	mov	r1, r11
 8001bd8: f6c0 0200    	movt	r2, #0x800
 8001bdc: f00a f982    	bl	0x800bee4 <core::panicking::panic_bounds_check> @ imm = #0xa304
 8001be0: f24e 42c0    	movw	r2, #0xe4c0
 8001be4: 4659         	mov	r1, r11
 8001be6: f6c0 0200    	movt	r2, #0x800
 8001bea: f00a f97b    	bl	0x800bee4 <core::panicking::panic_bounds_check> @ imm = #0xa2f6

08001bee <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3>:
 8001bee: b5f0         	push	{r4, r5, r6, r7, lr}
 8001bf0: af03         	add	r7, sp, #0xc
 8001bf2: e92d 0f00    	push.w	{r8, r9, r10, r11}
 8001bf6: b087         	sub	sp, #0x1c
 8001bf8: f890 208f    	ldrb.w	r2, [r0, #0x8f]
 8001bfc: 468a         	mov	r10, r1
 8001bfe: 2a01         	cmp	r2, #0x1
 8001c00: f040 8169    	bne.w	0x8001ed6 <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x2e8> @ imm = #0x2d2
 8001c04: 6c42         	ldr	r2, [r0, #0x44]
 8001c06: f64a 23ab    	movw	r3, #0xaaab
 8001c0a: f6ca 23aa    	movt	r3, #0xaaaa
 8001c0e: 6f06         	ldr	r6, [r0, #0x70]
 8001c10: f1ba 0f00    	cmp.w	r10, #0x0
 8001c14: fba2 3203    	umull	r3, r2, r2, r3
 8001c18: f000 815b    	beq.w	0x8001ed2 <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x2e4> @ imm = #0x2b6
 8001c1c: 2e05         	cmp	r6, #0x5
 8001c1e: f200 8158    	bhi.w	0x8001ed2 <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x2e4> @ imm = #0x2b0
 8001c22: 0894         	lsrs	r4, r2, #0x2
 8001c24: f890 2091    	ldrb.w	r2, [r0, #0x91]
 8001c28: 1e61         	subs	r1, r4, #0x1
 8001c2a: e9cd 1402    	strd	r1, r4, [sp, #8]
 8001c2e: f8d0 9074    	ldr.w	r9, [r0, #0x74]
 8001c32: eba4 0109    	sub.w	r1, r4, r9
 8001c36: 4551         	cmp	r1, r10
 8001c38: bf28         	it	hs
 8001c3a: 4651         	movhs	r1, r10
 8001c3c: eb01 0509    	add.w	r5, r1, r9
 8001c40: 07d2         	lsls	r2, r2, #0x1f
 8001c42: d028         	beq	0x8001c96 <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0xa8> @ imm = #0x50
 8001c44: f890 b090    	ldrb.w	r11, [r0, #0x90]
 8001c48: 9106         	str	r1, [sp, #0x18]
 8001c4a: 2101         	movs	r1, #0x1
 8001c4c: f1ab 0301    	sub.w	r3, r11, #0x1
 8001c50: 6c42         	ldr	r2, [r0, #0x44]
 8001c52: f003 031f    	and	r3, r3, #0x1f
 8001c56: 46d8         	mov	r8, r11
 8001c58: f1bb 0f00    	cmp.w	r11, #0x0
 8001c5c: bf18         	it	ne
 8001c5e: fa01 f803    	lslne.w	r8, r1, r3
 8001c62: eb09 0349    	add.w	r3, r9, r9, lsl #1
 8001c66: eb06 0e43    	add.w	lr, r6, r3, lsl #1
 8001c6a: eb05 0345    	add.w	r3, r5, r5, lsl #1
 8001c6e: ea4f 0c43    	lsl.w	r12, r3, #0x1
 8001c72: ebbe 0f43    	cmp.w	lr, r3, lsl #1
 8001c76: bf98         	it	ls
 8001c78: 4594         	cmpls	r12, r2
 8001c7a: f200 83bc    	bhi.w	0x80023f6 <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x808> @ imm = #0x778
 8001c7e: e9cd a504    	strd	r10, r5, [sp, #16]
 8001c82: 00db         	lsls	r3, r3, #0x3
 8001c84: e9d0 a21f    	ldrd	r10, r2, [r0, #124]
 8001c88: ebbe 0f93    	cmp.w	lr, r3, lsr #2
 8001c8c: f040 80ad    	bne.w	0x8001dea <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x1fc> @ imm = #0x15a
 8001c90: 46b6         	mov	lr, r6
 8001c92: 4653         	mov	r3, r10
 8001c94: e0d0         	b	0x8001e38 <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x24a> @ imm = #0x1a0
 8001c96: eb09 0349    	add.w	r3, r9, r9, lsl #1
 8001c9a: 6c42         	ldr	r2, [r0, #0x44]
 8001c9c: eb06 0c43    	add.w	r12, r6, r3, lsl #1
 8001ca0: eb05 0345    	add.w	r3, r5, r5, lsl #1
 8001ca4: ea4f 0e43    	lsl.w	lr, r3, #0x1
 8001ca8: 45e6         	cmp	lr, r12
 8001caa: f0c0 83c0    	blo.w	0x800242e <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x840> @ imm = #0x780
 8001cae: 4596         	cmp	lr, r2
 8001cb0: f200 83bd    	bhi.w	0x800242e <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x840> @ imm = #0x77a
 8001cb4: f8d0 8078    	ldr.w	r8, [r0, #0x78]
 8001cb8: 00da         	lsls	r2, r3, #0x3
 8001cba: ebbc 0f92    	cmp.w	r12, r2, lsr #2
 8001cbe: d12d         	bne	0x8001d1c <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x12e> @ imm = #0x5a
 8001cc0: ebaa 0a01    	sub.w	r10, r10, r1
 8001cc4: 42a5         	cmp	r5, r4
 8001cc6: 6745         	str	r5, [r0, #0x74]
 8001cc8: f040 80fc    	bne.w	0x8001ec4 <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x2d6> @ imm = #0x1f8
 8001ccc: f890 108c    	ldrb.w	r1, [r0, #0x8c]
 8001cd0: fab8 f288    	clz	r2, r8
 8001cd4: 4634         	mov	r4, r6
 8001cd6: f1c2 0620    	rsb.w	r6, r2, #0x20
 8001cda: 1e4a         	subs	r2, r1, #0x1
 8001cdc: 2300         	movs	r3, #0x0
 8001cde: 6743         	str	r3, [r0, #0x74]
 8001ce0: b2d5         	uxtb	r5, r2
 8001ce2: 42ae         	cmp	r6, r5
 8001ce4: d90d         	bls	0x8001d02 <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x114> @ imm = #0x1a
 8001ce6: 1b73         	subs	r3, r6, r5
 8001ce8: 2601         	movs	r6, #0x1
 8001cea: 1e59         	subs	r1, r3, #0x1
 8001cec: f002 021f    	and	r2, r2, #0x1f
 8001cf0: fa06 f101    	lsl.w	r1, r6, r1
 8001cf4: f003 061f    	and	r6, r3, #0x1f
 8001cf8: 4441         	add	r1, r8
 8001cfa: 40f1         	lsrs	r1, r6
 8001cfc: 40d1         	lsrs	r1, r2
 8001cfe: bf18         	it	ne
 8001d00: 3301         	addne	r3, #0x1
 8001d02: f890 108d    	ldrb.w	r1, [r0, #0x8d]
 8001d06: 2201         	movs	r2, #0x1
 8001d08: f880 2091    	strb.w	r2, [r0, #0x91]
 8001d0c: 4626         	mov	r6, r4
 8001d0e: 4299         	cmp	r1, r3
 8001d10: bf88         	it	hi
 8001d12: 460b         	movhi	r3, r1
 8001d14: 9c03         	ldr	r4, [sp, #0xc]
 8001d16: f880 3090    	strb.w	r3, [r0, #0x90]
 8001d1a: e0d4         	b	0x8001ec6 <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x2d8> @ imm = #0x1a8
 8001d1c: 6c02         	ldr	r2, [r0, #0x40]
 8001d1e: 460b         	mov	r3, r1
 8001d20: 4629         	mov	r1, r5
 8001d22: f8d0 e088    	ldr.w	lr, [r0, #0x88]
 8001d26: eb02 028c    	add.w	r2, r2, r12, lsl #2
 8001d2a: 469b         	mov	r11, r3
 8001d2c: f102 0530    	add.w	r5, r2, #0x30
 8001d30: eb03 0243    	add.w	r2, r3, r3, lsl #1
 8001d34: f1ce 0c00    	rsb.w	r12, lr, #0x0
 8001d38: f8cd a010    	str.w	r10, [sp, #0x10]
 8001d3c: 00d2         	lsls	r2, r2, #0x3
 8001d3e: f04f 0a00    	mov.w	r10, #0x0
 8001d42: eba2 0286    	sub.w	r2, r2, r6, lsl #2
 8001d46: f102 0314    	add.w	r3, r2, #0x14
 8001d4a: f855 2c30    	ldr	r2, [r5, #-48]
 8001d4e: 4562         	cmp	r2, r12
 8001d50: f2c0 8361    	blt.w	0x8002416 <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x828> @ imm = #0x6c2
 8001d54: 4572         	cmp	r2, lr
 8001d56: f300 835e    	bgt.w	0x8002416 <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x828> @ imm = #0x6bc
 8001d5a: 2a00         	cmp	r2, #0x0
 8001d5c: bf48         	it	mi
 8001d5e: 4252         	rsbmi	r2, r2, #0
 8001d60: 4542         	cmp	r2, r8
 8001d62: bfc8         	it	gt
 8001d64: 4690         	movgt	r8, r2
 8001d66: f1a3 0218    	sub.w	r2, r3, #0x18
 8001d6a: 2a15         	cmp	r2, #0x15
 8001d6c: f0c0 80a0    	blo.w	0x8001eb0 <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x2c2> @ imm = #0x140
 8001d70: f855 2c18    	ldr	r2, [r5, #-24]
 8001d74: 4562         	cmp	r2, r12
 8001d76: f2c0 8349    	blt.w	0x800240c <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x81e> @ imm = #0x692
 8001d7a: 4572         	cmp	r2, lr
 8001d7c: f300 8346    	bgt.w	0x800240c <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x81e> @ imm = #0x68c
 8001d80: 2a00         	cmp	r2, #0x0
 8001d82: bf48         	it	mi
 8001d84: 4252         	rsbmi	r2, r2, #0
 8001d86: 4542         	cmp	r2, r8
 8001d88: bfc8         	it	gt
 8001d8a: 4690         	movgt	r8, r2
 8001d8c: f1a3 0230    	sub.w	r2, r3, #0x30
 8001d90: 2a15         	cmp	r2, #0x15
 8001d92: f0c0 808d    	blo.w	0x8001eb0 <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x2c2> @ imm = #0x11a
 8001d96: 682a         	ldr	r2, [r5]
 8001d98: 4562         	cmp	r2, r12
 8001d9a: f2c0 833a    	blt.w	0x8002412 <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x824> @ imm = #0x674
 8001d9e: 4572         	cmp	r2, lr
 8001da0: f300 8337    	bgt.w	0x8002412 <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x824> @ imm = #0x66e
 8001da4: 2a00         	cmp	r2, #0x0
 8001da6: bf48         	it	mi
 8001da8: 4252         	rsbmi	r2, r2, #0
 8001daa: 4542         	cmp	r2, r8
 8001dac: f1a3 0448    	sub.w	r4, r3, #0x48
 8001db0: bfd8         	it	le
 8001db2: 4642         	movle	r2, r8
 8001db4: 2c15         	cmp	r4, #0x15
 8001db6: d37a         	blo	0x8001eae <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x2c0> @ imm = #0xf4
 8001db8: f8d5 8018    	ldr.w	r8, [r5, #0x18]
 8001dbc: 45e0         	cmp	r8, r12
 8001dbe: f2c0 832c    	blt.w	0x800241a <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x82c> @ imm = #0x658
 8001dc2: 45f0         	cmp	r8, lr
 8001dc4: f300 8329    	bgt.w	0x800241a <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x82c> @ imm = #0x652
 8001dc8: f1b8 0f00    	cmp.w	r8, #0x0
 8001dcc: f105 0560    	add.w	r5, r5, #0x60
 8001dd0: f10a 0a04    	add.w	r10, r10, #0x4
 8001dd4: f1a3 0360    	sub.w	r3, r3, #0x60
 8001dd8: bf48         	it	mi
 8001dda: f1c8 0800    	rsbmi.w	r8, r8, #0x0
 8001dde: 4590         	cmp	r8, r2
 8001de0: bfd8         	it	le
 8001de2: 4690         	movle	r8, r2
 8001de4: 2b14         	cmp	r3, #0x14
 8001de6: d8b0         	bhi	0x8001d4a <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x15c> @ imm = #-0xa0
 8001de8: e062         	b	0x8001eb0 <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x2c2> @ imm = #0xc4
 8001dea: 6c03         	ldr	r3, [r0, #0x40]
 8001dec: f00b 0c1f    	and	r12, r11, #0x1f
 8001df0: 9906         	ldr	r1, [sp, #0x18]
 8001df2: eb03 038e    	add.w	r3, r3, lr, lsl #2
 8001df6: 46b6         	mov	lr, r6
 8001df8: f1a3 0518    	sub.w	r5, r3, #0x18
 8001dfc: eb01 0341    	add.w	r3, r1, r1, lsl #1
 8001e00: 00db         	lsls	r3, r3, #0x3
 8001e02: eba3 0386    	sub.w	r3, r3, r6, lsl #2
 8001e06: f103 0414    	add.w	r4, r3, #0x14
 8001e0a: f855 3f18    	ldr	r3, [r5, #24]!
 8001e0e: f1b9 0f00    	cmp.w	r9, #0x0
 8001e12: f109 0901    	add.w	r9, r9, #0x1
 8001e16: f1a4 0418    	sub.w	r4, r4, #0x18
 8001e1a: 4443         	add	r3, r8
 8001e1c: fa43 f30c    	asr.w	r3, r3, r12
 8001e20: eba3 060a    	sub.w	r6, r3, r10
 8001e24: 469a         	mov	r10, r3
 8001e26: 602b         	str	r3, [r5]
 8001e28: ea4f 0146    	lsl.w	r1, r6, #0x1
 8001e2c: ea81 71e6    	eor.w	r1, r1, r6, asr #31
 8001e30: bf18         	it	ne
 8001e32: 440a         	addne	r2, r1
 8001e34: 2c14         	cmp	r4, #0x14
 8001e36: d8e8         	bhi	0x8001e0a <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x21c> @ imm = #-0x30
 8001e38: e9c0 321f    	strd	r3, r2, [r0, #124]
 8001e3c: f8dd a010    	ldr.w	r10, [sp, #0x10]
 8001e40: 9b06         	ldr	r3, [sp, #0x18]
 8001e42: 9c03         	ldr	r4, [sp, #0xc]
 8001e44: 9905         	ldr	r1, [sp, #0x14]
 8001e46: ebaa 0a03    	sub.w	r10, r10, r3
 8001e4a: 6741         	str	r1, [r0, #0x74]
 8001e4c: 42a1         	cmp	r1, r4
 8001e4e: d12b         	bne	0x8001ea8 <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x2ba> @ imm = #0x56
 8001e50: 9902         	ldr	r1, [sp, #0x8]
 8001e52: 2300         	movs	r3, #0x0
 8001e54: 6743         	str	r3, [r0, #0x74]
 8001e56: 2900         	cmp	r1, #0x0
 8001e58: bf18         	it	ne
 8001e5a: 2a00         	cmpne	r2, #0x0
 8001e5c: d00b         	beq	0x8001e76 <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x288> @ imm = #0x16
 8001e5e: 9b02         	ldr	r3, [sp, #0x8]
 8001e60: fbb2 f1f3    	udiv	r1, r2, r3
 8001e64: fb01 2213    	mls	r2, r1, r3, r2
 8001e68: 2a00         	cmp	r2, #0x0
 8001e6a: bf18         	it	ne
 8001e6c: 3101         	addne	r1, #0x1
 8001e6e: fab1 f181    	clz	r1, r1
 8001e72: f1c1 031f    	rsb.w	r3, r1, #0x1f
 8001e76: f1ae 0201    	sub.w	r2, lr, #0x1
 8001e7a: 2a04         	cmp	r2, #0x4
 8001e7c: f200 830f    	bhi.w	0x800249e <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x8b0> @ imm = #0x61e
 8001e80: f100 0148    	add.w	r1, r0, #0x48
 8001e84: f841 b032    	str.w	r11, [r1, r2, lsl #3]
 8001e88: eb01 01c2    	add.w	r1, r1, r2, lsl #3
 8001e8c: 2202         	movs	r2, #0x2
 8001e8e: 6f06         	ldr	r6, [r0, #0x70]
 8001e90: 714a         	strb	r2, [r1, #0x5]
 8001e92: 2200         	movs	r2, #0x0
 8001e94: 710b         	strb	r3, [r1, #0x4]
 8001e96: 3601         	adds	r6, #0x1
 8001e98: f880 2091    	strb.w	r2, [r0, #0x91]
 8001e9c: e9c0 221e    	strd	r2, r2, [r0, #120]
 8001ea0: f8c0 2080    	str.w	r2, [r0, #0x80]
 8001ea4: 6706         	str	r6, [r0, #0x70]
 8001ea6: e00e         	b	0x8001ec6 <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x2d8> @ imm = #0x1c
 8001ea8: 2201         	movs	r2, #0x1
 8001eaa: 4676         	mov	r6, lr
 8001eac: e00b         	b	0x8001ec6 <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x2d8> @ imm = #0x16
 8001eae: 4690         	mov	r8, r2
 8001eb0: f8dd a010    	ldr.w	r10, [sp, #0x10]
 8001eb4: 9c03         	ldr	r4, [sp, #0xc]
 8001eb6: ebaa 0a0b    	sub.w	r10, r10, r11
 8001eba: e9c0 181d    	strd	r1, r8, [r0, #116]
 8001ebe: 42a1         	cmp	r1, r4
 8001ec0: f43f af04    	beq.w	0x8001ccc <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0xde> @ imm = #-0x1f8
 8001ec4: 2200         	movs	r2, #0x0
 8001ec6: f1ba 0f00    	cmp.w	r10, #0x0
 8001eca: d002         	beq	0x8001ed2 <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x2e4> @ imm = #0x4
 8001ecc: 2e06         	cmp	r6, #0x6
 8001ece: f4ff aeae    	blo.w	0x8001c2e <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x40> @ imm = #-0x2a4
 8001ed2: 1fb0         	subs	r0, r6, #0x6
 8001ed4: e284         	b	0x80023e0 <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x7f2> @ imm = #0x508
 8001ed6: 6c41         	ldr	r1, [r0, #0x44]
 8001ed8: f64a 23ab    	movw	r3, #0xaaab
 8001edc: f6ca 23aa    	movt	r3, #0xaaaa
 8001ee0: 6f02         	ldr	r2, [r0, #0x70]
 8001ee2: f1ba 0f00    	cmp.w	r10, #0x0
 8001ee6: fba1 1303    	umull	r1, r3, r1, r3
 8001eea: f000 8278    	beq.w	0x80023de <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x7f0> @ imm = #0x4f0
 8001eee: 2a05         	cmp	r2, #0x5
 8001ef0: f200 8275    	bhi.w	0x80023de <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x7f0> @ imm = #0x4ea
 8001ef4: f100 0948    	add.w	r9, r0, #0x48
 8001ef8: f100 0b24    	add.w	r11, r0, #0x24
 8001efc: f100 0820    	add.w	r8, r0, #0x20
 8001f00: 0899         	lsrs	r1, r3, #0x2
 8001f02: 9105         	str	r1, [sp, #0x14]
 8001f04: 3901         	subs	r1, #0x1
 8001f06: 9106         	str	r1, [sp, #0x18]
 8001f08: 6f41         	ldr	r1, [r0, #0x74]
 8001f0a: f8d0 e044    	ldr.w	lr, [r0, #0x44]
 8001f0e: f890 4091    	ldrb.w	r4, [r0, #0x91]
 8001f12: eb01 0341    	add.w	r3, r1, r1, lsl #1
 8001f16: eb02 0c43    	add.w	r12, r2, r3, lsl #1
 8001f1a: b3b4         	cbz	r4, 0x8001f8a <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x39c> @ imm = #0x6c
 8001f1c: 45f4         	cmp	r12, lr
 8001f1e: f080 82a6    	bhs.w	0x800246e <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x880> @ imm = #0x54c
 8001f22: f890 508c    	ldrb.w	r5, [r0, #0x8c]
 8001f26: 2201         	movs	r2, #0x1
 8001f28: f04f 0e00    	mov.w	lr, #0x0
 8001f2c: 2d0c         	cmp	r5, #0xc
 8001f2e: f200 825c    	bhi.w	0x80023ea <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x7fc> @ imm = #0x4b8
 8001f32: 2101         	movs	r1, #0x1
 8001f34: 40a9         	lsls	r1, r5
 8001f36: f411 5faa    	tst.w	r1, #0x1540
 8001f3a: f000 8256    	beq.w	0x80023ea <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x7fc> @ imm = #0x4ac
 8001f3e: 6f83         	ldr	r3, [r0, #0x78]
 8001f40: 2b00         	cmp	r3, #0x0
 8001f42: f100 8252    	bmi.w	0x80023ea <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x7fc> @ imm = #0x4a4
 8001f46: f8d0 e040    	ldr.w	lr, [r0, #0x40]
 8001f4a: f8cd a010    	str.w	r10, [sp, #0x10]
 8001f4e: f85e 402c    	ldr.w	r4, [lr, r12, lsl #2]
 8001f52: f1b4 3fff    	cmp.w	r4, #0xffffffff
 8001f56: 46a2         	mov	r10, r4
 8001f58: dc05         	bgt	0x8001f66 <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x378> @ imm = #0xa
 8001f5a: f1b4 4f00    	cmp.w	r4, #0x80000000
 8001f5e: f000 8252    	beq.w	0x8002406 <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x818> @ imm = #0x4a4
 8001f62: f1c4 0a00    	rsb.w	r10, r4, #0x0
 8001f66: 459a         	cmp	r10, r3
 8001f68: f200 824d    	bhi.w	0x8002406 <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x818> @ imm = #0x49a
 8001f6c: b3c3         	cbz	r3, 0x8001fe0 <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x3f2> @ imm = #0x70
 8001f6e: 1e69         	subs	r1, r5, #0x1
 8001f70: 459a         	cmp	r10, r3
 8001f72: b2cd         	uxtb	r5, r1
 8001f74: f04f 31ff    	mov.w	r1, #0xffffffff
 8001f78: fa01 f605    	lsl.w	r6, r1, r5
 8001f7c: ea6f 0206    	mvn.w	r2, r6
 8001f80: d130         	bne	0x8001fe4 <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x3f6> @ imm = #0x60
 8001f82: 2c00         	cmp	r4, #0x0
 8001f84: bf48         	it	mi
 8001f86: 1c72         	addmi	r2, r6, #0x1
 8001f88: e03d         	b	0x8002006 <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x418> @ imm = #0x7a
 8001f8a: 005b         	lsls	r3, r3, #0x1
 8001f8c: 4573         	cmp	r3, lr
 8001f8e: f080 8276    	bhs.w	0x800247e <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x890> @ imm = #0x4ec
 8001f92: 45f4         	cmp	r12, lr
 8001f94: f080 8263    	bhs.w	0x800245e <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x870> @ imm = #0x4c6
 8001f98: 6c04         	ldr	r4, [r0, #0x40]
 8001f9a: f04f 0e01    	mov.w	lr, #0x1
 8001f9e: f8d0 2088    	ldr.w	r2, [r0, #0x88]
 8001fa2: f854 502c    	ldr.w	r5, [r4, r12, lsl #2]
 8001fa6: 4295         	cmp	r5, r2
 8001fa8: f300 823f    	bgt.w	0x800242a <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x83c> @ imm = #0x47e
 8001fac: 4256         	rsbs	r6, r2, #0
 8001fae: 42b5         	cmp	r5, r6
 8001fb0: f2c0 823b    	blt.w	0x800242a <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x83c> @ imm = #0x476
 8001fb4: f854 3023    	ldr.w	r3, [r4, r3, lsl #2]
 8001fb8: 42b3         	cmp	r3, r6
 8001fba: f2c0 8236    	blt.w	0x800242a <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x83c> @ imm = #0x46c
 8001fbe: 4293         	cmp	r3, r2
 8001fc0: f04f 0201    	mov.w	r2, #0x1
 8001fc4: f300 8211    	bgt.w	0x80023ea <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x7fc> @ imm = #0x422
 8001fc8: 1ae9         	subs	r1, r5, r3
 8001fca: f844 102c    	str.w	r1, [r4, r12, lsl #2]
 8001fce: e9d0 421d    	ldrd	r4, r2, [r0, #116]
 8001fd2: bf48         	it	mi
 8001fd4: 4249         	rsbmi	r1, r1, #0
 8001fd6: 4291         	cmp	r1, r2
 8001fd8: bfd8         	it	le
 8001fda: 4611         	movle	r1, r2
 8001fdc: 6781         	str	r1, [r0, #0x78]
 8001fde: e034         	b	0x800204a <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x45c> @ imm = #0x68
 8001fe0: 2200         	movs	r2, #0x0
 8001fe2: e010         	b	0x8002006 <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x418> @ imm = #0x20
 8001fe4: fb8a 6102    	smull	r6, r1, r10, r2
 8001fe8: ebb1 7fe6    	cmp.w	r1, r6, asr #31
 8001fec: f040 8189    	bne.w	0x8002302 <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x714> @ imm = #0x312
 8001ff0: fb96 f2f3    	sdiv	r2, r6, r3
 8001ff4: fb02 6113    	mls	r1, r2, r3, r6
 8001ff8: 1a5b         	subs	r3, r3, r1
 8001ffa: 4299         	cmp	r1, r3
 8001ffc: bfa8         	it	ge
 8001ffe: 3201         	addge	r2, #0x1
 8002000: 2c00         	cmp	r4, #0x0
 8002002: bf48         	it	mi
 8002004: 4252         	rsbmi	r2, r2, #0
 8002006: f84e 202c    	str.w	r2, [lr, r12, lsl #2]
 800200a: f890 608e    	ldrb.w	r6, [r0, #0x8e]
 800200e: f8dd a010    	ldr.w	r10, [sp, #0x10]
 8002012: 2e01         	cmp	r6, #0x1
 8002014: d008         	beq	0x8002028 <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x43a> @ imm = #0x10
 8002016: 0053         	lsls	r3, r2, #0x1
 8002018: 6801         	ldr	r1, [r0]
 800201a: ea83 75e2    	eor.w	r5, r3, r2, asr #31
 800201e: ebb3 7fe2    	cmp.w	r3, r2, asr #31
 8002022: 4429         	add	r1, r5
 8002024: 6001         	str	r1, [r0]
 8002026: d13c         	bne	0x80020a2 <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x4b4> @ imm = #0x78
 8002028: 6f44         	ldr	r4, [r0, #0x74]
 800202a: 2e00         	cmp	r6, #0x0
 800202c: bf18         	it	ne
 800202e: 2c00         	cmpne	r4, #0x0
 8002030: d00a         	beq	0x8002048 <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x45a> @ imm = #0x14
 8002032: 6fc3         	ldr	r3, [r0, #0x7c]
 8002034: 6a01         	ldr	r1, [r0, #0x20]
 8002036: 1ad3         	subs	r3, r2, r3
 8002038: 005d         	lsls	r5, r3, #0x1
 800203a: ea85 76e3    	eor.w	r6, r5, r3, asr #31
 800203e: 4431         	add	r1, r6
 8002040: ebb5 7fe3    	cmp.w	r5, r3, asr #31
 8002044: 6201         	str	r1, [r0, #0x20]
 8002046: d16d         	bne	0x8002124 <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x536> @ imm = #0xda
 8002048: 67c2         	str	r2, [r0, #0x7c]
 800204a: 9a05         	ldr	r2, [sp, #0x14]
 800204c: 1c61         	adds	r1, r4, #0x1
 800204e: 6741         	str	r1, [r0, #0x74]
 8002050: 4291         	cmp	r1, r2
 8002052: f040 814b    	bne.w	0x80022ec <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x6fe> @ imm = #0x296
 8002056: f890 1091    	ldrb.w	r1, [r0, #0x91]
 800205a: 2200         	movs	r2, #0x0
 800205c: 6742         	str	r2, [r0, #0x74]
 800205e: 2900         	cmp	r1, #0x0
 8002060: f000 80a0    	beq.w	0x80021a4 <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x5b6> @ imm = #0x140
 8002064: f890 e08c    	ldrb.w	lr, [r0, #0x8c]
 8002068: 9905         	ldr	r1, [sp, #0x14]
 800206a: 6f84         	ldr	r4, [r0, #0x78]
 800206c: fb01 f10e    	mul	r1, r1, lr
 8002070: 074a         	lsls	r2, r1, #0x1d
 8002072: ea4f 03d1    	lsr.w	r3, r1, #0x3
 8002076: f04f 0201    	mov.w	r2, #0x1
 800207a: bf18         	it	ne
 800207c: eb02 03d1    	addne.w	r3, r2, r1, lsr #3
 8002080: 2c00         	cmp	r4, #0x0
 8002082: f000 8093    	beq.w	0x80021ac <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x5be> @ imm = #0x126
 8002086: f890 908e    	ldrb.w	r9, [r0, #0x8e]
 800208a: 9402         	str	r4, [sp, #0x8]
 800208c: f1b9 0f01    	cmp.w	r9, #0x1
 8002090: e9cd 3a03    	strd	r3, r10, [sp, #12]
 8002094: f040 808e    	bne.w	0x80021b4 <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x5c6> @ imm = #0x11c
 8002098: 2103         	movs	r1, #0x3
 800209a: f04f 0c00    	mov.w	r12, #0x0
 800209e: 9101         	str	r1, [sp, #0x4]
 80020a0: e0c3         	b	0x800222a <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x63c> @ imm = #0x186
 80020a2: fa95 f1a5    	rbit	r1, r5
 80020a6: fab1 f381    	clz	r3, r1
 80020aa: 2b0c         	cmp	r3, #0xc
 80020ac: f200 81c7    	bhi.w	0x800243e <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x850> @ imm = #0x38e
 80020b0: 1d04         	adds	r4, r0, #0x4
 80020b2: f834 1013    	ldrh.w	r1, [r4, r3, lsl #1]
 80020b6: 3101         	adds	r1, #0x1
 80020b8: f824 1013    	strh.w	r1, [r4, r3, lsl #1]
 80020bc: 1e69         	subs	r1, r5, #0x1
 80020be: 400d         	ands	r5, r1
 80020c0: d0b2         	beq	0x8002028 <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x43a> @ imm = #-0x9c
 80020c2: fa95 f1a5    	rbit	r1, r5
 80020c6: fab1 f381    	clz	r3, r1
 80020ca: 2b0c         	cmp	r3, #0xc
 80020cc: f200 81b7    	bhi.w	0x800243e <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x850> @ imm = #0x36e
 80020d0: 1d04         	adds	r4, r0, #0x4
 80020d2: f834 1013    	ldrh.w	r1, [r4, r3, lsl #1]
 80020d6: 3101         	adds	r1, #0x1
 80020d8: f824 1013    	strh.w	r1, [r4, r3, lsl #1]
 80020dc: 1e69         	subs	r1, r5, #0x1
 80020de: 400d         	ands	r5, r1
 80020e0: d0a2         	beq	0x8002028 <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x43a> @ imm = #-0xbc
 80020e2: fa95 f1a5    	rbit	r1, r5
 80020e6: fab1 f381    	clz	r3, r1
 80020ea: 2b0c         	cmp	r3, #0xc
 80020ec: f200 81a7    	bhi.w	0x800243e <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x850> @ imm = #0x34e
 80020f0: 1d04         	adds	r4, r0, #0x4
 80020f2: f834 1013    	ldrh.w	r1, [r4, r3, lsl #1]
 80020f6: 3101         	adds	r1, #0x1
 80020f8: f824 1013    	strh.w	r1, [r4, r3, lsl #1]
 80020fc: 1e69         	subs	r1, r5, #0x1
 80020fe: 400d         	ands	r5, r1
 8002100: d092         	beq	0x8002028 <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x43a> @ imm = #-0xdc
 8002102: fa95 f1a5    	rbit	r1, r5
 8002106: fab1 f381    	clz	r3, r1
 800210a: 2b0c         	cmp	r3, #0xc
 800210c: f200 8197    	bhi.w	0x800243e <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x850> @ imm = #0x32e
 8002110: 1d04         	adds	r4, r0, #0x4
 8002112: f834 1013    	ldrh.w	r1, [r4, r3, lsl #1]
 8002116: 3101         	adds	r1, #0x1
 8002118: f824 1013    	strh.w	r1, [r4, r3, lsl #1]
 800211c: 1e69         	subs	r1, r5, #0x1
 800211e: 400d         	ands	r5, r1
 8002120: d1bf         	bne	0x80020a2 <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x4b4> @ imm = #-0x82
 8002122: e781         	b	0x8002028 <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x43a> @ imm = #-0xfe
 8002124: fa96 f1a6    	rbit	r1, r6
 8002128: fab1 f381    	clz	r3, r1
 800212c: 2b0c         	cmp	r3, #0xc
 800212e: f200 8186    	bhi.w	0x800243e <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x850> @ imm = #0x30c
 8002132: f83b 1013    	ldrh.w	r1, [r11, r3, lsl #1]
 8002136: 3101         	adds	r1, #0x1
 8002138: f82b 1013    	strh.w	r1, [r11, r3, lsl #1]
 800213c: 1e71         	subs	r1, r6, #0x1
 800213e: 400e         	ands	r6, r1
 8002140: f43f af82    	beq.w	0x8002048 <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x45a> @ imm = #-0xfc
 8002144: fa96 f1a6    	rbit	r1, r6
 8002148: fab1 f381    	clz	r3, r1
 800214c: 2b0c         	cmp	r3, #0xc
 800214e: f200 8176    	bhi.w	0x800243e <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x850> @ imm = #0x2ec
 8002152: f83b 1013    	ldrh.w	r1, [r11, r3, lsl #1]
 8002156: 3101         	adds	r1, #0x1
 8002158: f82b 1013    	strh.w	r1, [r11, r3, lsl #1]
 800215c: 1e71         	subs	r1, r6, #0x1
 800215e: 400e         	ands	r6, r1
 8002160: f43f af72    	beq.w	0x8002048 <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x45a> @ imm = #-0x11c
 8002164: fa96 f1a6    	rbit	r1, r6
 8002168: fab1 f381    	clz	r3, r1
 800216c: 2b0c         	cmp	r3, #0xc
 800216e: f200 8166    	bhi.w	0x800243e <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x850> @ imm = #0x2cc
 8002172: f83b 1013    	ldrh.w	r1, [r11, r3, lsl #1]
 8002176: 3101         	adds	r1, #0x1
 8002178: f82b 1013    	strh.w	r1, [r11, r3, lsl #1]
 800217c: 1e71         	subs	r1, r6, #0x1
 800217e: 400e         	ands	r6, r1
 8002180: f43f af62    	beq.w	0x8002048 <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x45a> @ imm = #-0x13c
 8002184: fa96 f1a6    	rbit	r1, r6
 8002188: fab1 f381    	clz	r3, r1
 800218c: 2b0c         	cmp	r3, #0xc
 800218e: f200 8156    	bhi.w	0x800243e <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x850> @ imm = #0x2ac
 8002192: f83b 1013    	ldrh.w	r1, [r11, r3, lsl #1]
 8002196: 3101         	adds	r1, #0x1
 8002198: f82b 1013    	strh.w	r1, [r11, r3, lsl #1]
 800219c: 1e71         	subs	r1, r6, #0x1
 800219e: 400e         	ands	r6, r1
 80021a0: d1c0         	bne	0x8002124 <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x536> @ imm = #-0x80
 80021a2: e751         	b	0x8002048 <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x45a> @ imm = #-0x15e
 80021a4: 2101         	movs	r1, #0x1
 80021a6: f880 1091    	strb.w	r1, [r0, #0x91]
 80021aa: e09f         	b	0x80022ec <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x6fe> @ imm = #0x13e
 80021ac: 2500         	movs	r5, #0x0
 80021ae: f04f 0c00    	mov.w	r12, #0x0
 80021b2: e075         	b	0x80022a0 <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x6b2> @ imm = #0xea
 80021b4: f8d0 a000    	ldr.w	r10, [r0]
 80021b8: f04f 0c00    	mov.w	r12, #0x0
 80021bc: f04f 36ff    	mov.w	r6, #0xffffffff
 80021c0: 2400         	movs	r4, #0x0
 80021c2: b2e3         	uxtb	r3, r4
 80021c4: 2b0c         	cmp	r3, #0xc
 80021c6: f200 8142    	bhi.w	0x800244e <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x860> @ imm = #0x284
 80021ca: 9a05         	ldr	r2, [sp, #0x14]
 80021cc: 1c59         	adds	r1, r3, #0x1
 80021ce: fb01 a102    	mla	r1, r1, r2, r10
 80021d2: 074d         	lsls	r5, r1, #0x1d
 80021d4: ea4f 02d1    	lsr.w	r2, r1, #0x3
 80021d8: f04f 0501    	mov.w	r5, #0x1
 80021dc: bf18         	it	ne
 80021de: eb05 02d1    	addne.w	r2, r5, r1, lsr #3
 80021e2: 42b2         	cmp	r2, r6
 80021e4: bf3c         	itt	lo
 80021e6: 4616         	movlo	r6, r2
 80021e8: 46a4         	movlo	r12, r4
 80021ea: 2200         	movs	r2, #0x0
 80021ec: 4573         	cmp	r3, lr
 80021ee: bf38         	it	lo
 80021f0: 2201         	movlo	r2, #0x1
 80021f2: d20a         	bhs	0x800220a <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x61c> @ imm = #0x14
 80021f4: 1d01         	adds	r1, r0, #0x4
 80021f6: 4414         	add	r4, r2
 80021f8: f831 1013    	ldrh.w	r1, [r1, r3, lsl #1]
 80021fc: ebaa 0101    	sub.w	r1, r10, r1
 8002200: ea4f 0a51    	lsr.w	r10, r1, #0x1
 8002204: b2e1         	uxtb	r1, r4
 8002206: 4571         	cmp	r1, lr
 8002208: d9db         	bls	0x80021c2 <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x5d4> @ imm = #-0x4a
 800220a: 9a03         	ldr	r2, [sp, #0xc]
 800220c: 2503         	movs	r5, #0x3
 800220e: 2100         	movs	r1, #0x0
 8002210: 4296         	cmp	r6, r2
 8002212: bf2c         	ite	hs
 8002214: 468c         	movhs	r12, r1
 8002216: 4632         	movlo	r2, r6
 8002218: bf38         	it	lo
 800221a: 2501         	movlo	r5, #0x1
 800221c: f8dd a010    	ldr.w	r10, [sp, #0x10]
 8002220: f1b9 0f00    	cmp.w	r9, #0x0
 8002224: d06a         	beq	0x80022fc <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x70e> @ imm = #0xd4
 8002226: 9501         	str	r5, [sp, #0x4]
 8002228: 9203         	str	r2, [sp, #0xc]
 800222a: f8d8 8000    	ldr.w	r8, [r8]
 800222e: 2600         	movs	r6, #0x0
 8002230: f04f 35ff    	mov.w	r5, #0xffffffff
 8002234: f04f 0900    	mov.w	r9, #0x0
 8002238: fa5f f389    	uxtb.w	r3, r9
 800223c: 2b0c         	cmp	r3, #0xc
 800223e: f200 8106    	bhi.w	0x800244e <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x860> @ imm = #0x20c
 8002242: 9c06         	ldr	r4, [sp, #0x18]
 8002244: 1c5a         	adds	r2, r3, #0x1
 8002246: eb08 010e    	add.w	r1, r8, lr
 800224a: fb02 1104    	mla	r1, r2, r4, r1
 800224e: 2401         	movs	r4, #0x1
 8002250: 08ca         	lsrs	r2, r1, #0x3
 8002252: ea5f 7a41    	lsls.w	r10, r1, #0x1d
 8002256: bf18         	it	ne
 8002258: eb04 02d1    	addne.w	r2, r4, r1, lsr #3
 800225c: 42aa         	cmp	r2, r5
 800225e: bf3c         	itt	lo
 8002260: 4615         	movlo	r5, r2
 8002262: 464e         	movlo	r6, r9
 8002264: 2200         	movs	r2, #0x0
 8002266: 4573         	cmp	r3, lr
 8002268: bf38         	it	lo
 800226a: 2201         	movlo	r2, #0x1
 800226c: d20a         	bhs	0x8002284 <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x696> @ imm = #0x14
 800226e: f83b 1013    	ldrh.w	r1, [r11, r3, lsl #1]
 8002272: 4491         	add	r9, r2
 8002274: eba8 0101    	sub.w	r1, r8, r1
 8002278: ea4f 0851    	lsr.w	r8, r1, #0x1
 800227c: fa5f f189    	uxtb.w	r1, r9
 8002280: 4571         	cmp	r1, lr
 8002282: d9d9         	bls	0x8002238 <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x64a> @ imm = #-0x4e
 8002284: 9903         	ldr	r1, [sp, #0xc]
 8002286: f100 0948    	add.w	r9, r0, #0x48
 800228a: f100 0820    	add.w	r8, r0, #0x20
 800228e: 428d         	cmp	r5, r1
 8002290: bf38         	it	lo
 8002292: 46b4         	movlo	r12, r6
 8002294: 9d01         	ldr	r5, [sp, #0x4]
 8002296: bf38         	it	lo
 8002298: 2502         	movlo	r5, #0x2
 800229a: f8dd a010    	ldr.w	r10, [sp, #0x10]
 800229e: 9c02         	ldr	r4, [sp, #0x8]
 80022a0: 6f01         	ldr	r1, [r0, #0x70]
 80022a2: 1e4b         	subs	r3, r1, #0x1
 80022a4: 2b05         	cmp	r3, #0x5
 80022a6: f080 80f2    	bhs.w	0x800248e <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x8a0> @ imm = #0x1e4
 80022aa: f849 4033    	str.w	r4, [r9, r3, lsl #3]
 80022ae: eb09 01c3    	add.w	r1, r9, r3, lsl #3
 80022b2: 2600         	movs	r6, #0x0
 80022b4: 6f02         	ldr	r2, [r0, #0x70]
 80022b6: f880 6091    	strb.w	r6, [r0, #0x91]
 80022ba: 6786         	str	r6, [r0, #0x78]
 80022bc: 6006         	str	r6, [r0]
 80022be: 714d         	strb	r5, [r1, #0x5]
 80022c0: f881 c004    	strb.w	r12, [r1, #0x4]
 80022c4: 1c51         	adds	r1, r2, #0x1
 80022c6: 67c6         	str	r6, [r0, #0x7c]
 80022c8: 6701         	str	r1, [r0, #0x70]
 80022ca: e9c0 6601    	strd	r6, r6, [r0, #4]
 80022ce: e9c0 6603    	strd	r6, r6, [r0, #12]
 80022d2: e9c0 6605    	strd	r6, r6, [r0, #20]
 80022d6: 8386         	strh	r6, [r0, #0x1c]
 80022d8: e9c8 6600    	strd	r6, r6, [r8]
 80022dc: e9c8 6602    	strd	r6, r6, [r8, #8]
 80022e0: e9c8 6604    	strd	r6, r6, [r8, #16]
 80022e4: f8c8 6018    	str.w	r6, [r8, #0x18]
 80022e8: f8a8 601c    	strh.w	r6, [r8, #0x1c]
 80022ec: 6f02         	ldr	r2, [r0, #0x70]
 80022ee: f1ba 0a01    	subs.w	r10, r10, #0x1
 80022f2: d074         	beq	0x80023de <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x7f0> @ imm = #0xe8
 80022f4: 2a06         	cmp	r2, #0x6
 80022f6: f4ff ae07    	blo.w	0x8001f08 <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x31a> @ imm = #-0x3f2
 80022fa: e070         	b	0x80023de <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x7f0> @ imm = #0xe0
 80022fc: f100 0948    	add.w	r9, r0, #0x48
 8002300: e7cd         	b	0x800229e <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x6b0> @ imm = #-0x66
 8002302: 2d01         	cmp	r5, #0x1
 8002304: f04f 0201    	mov.w	r2, #0x1
 8002308: bf88         	it	hi
 800230a: 462a         	movhi	r2, r5
 800230c: f002 0103    	and	r1, r2, #0x3
 8002310: 2d04         	cmp	r5, #0x4
 8002312: 9100         	str	r1, [sp]
 8002314: d202         	bhs	0x800231c <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x72e> @ imm = #0x4
 8002316: 2200         	movs	r2, #0x0
 8002318: 4656         	mov	r6, r10
 800231a: e039         	b	0x8002390 <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x7a2> @ imm = #0x72
 800231c: f002 01fc    	and	r1, r2, #0xfc
 8002320: 9101         	str	r1, [sp, #0x4]
 8002322: 2100         	movs	r1, #0x0
 8002324: 2200         	movs	r2, #0x0
 8002326: 4656         	mov	r6, r10
 8002328: 9103         	str	r1, [sp, #0xc]
 800232a: 1b99         	subs	r1, r3, r6
 800232c: ea4f 0982    	lsl.w	r9, r2, #0x2
 8002330: 1a71         	subs	r1, r6, r1
 8002332: bfb8         	it	lt
 8002334: 0071         	lsllt	r1, r6, #0x1
 8002336: eba3 0501    	sub.w	r5, r3, r1
 800233a: 9502         	str	r5, [sp, #0x8]
 800233c: f109 0502    	add.w	r5, r9, #0x2
 8002340: bfb8         	it	lt
 8002342: 0095         	lsllt	r5, r2, #0x2
 8002344: 9a02         	ldr	r2, [sp, #0x8]
 8002346: 004e         	lsls	r6, r1, #0x1
 8002348: 4291         	cmp	r1, r2
 800234a: bfa8         	it	ge
 800234c: 1a8e         	subge	r6, r1, r2
 800234e: eba3 0206    	sub.w	r2, r3, r6
 8002352: bfa8         	it	ge
 8002354: 3501         	addge	r5, #0x1
 8002356: 1ab1         	subs	r1, r6, r2
 8002358: 4296         	cmp	r6, r2
 800235a: ea4f 0285    	lsl.w	r2, r5, #0x2
 800235e: bfb8         	it	lt
 8002360: 0071         	lsllt	r1, r6, #0x1
 8002362: f102 0202    	add.w	r2, r2, #0x2
 8002366: eba3 0901    	sub.w	r9, r3, r1
 800236a: bfb8         	it	lt
 800236c: 00aa         	lsllt	r2, r5, #0x2
 800236e: 004e         	lsls	r6, r1, #0x1
 8002370: 4549         	cmp	r1, r9
 8002372: bfa4         	itt	ge
 8002374: eba1 0609    	subge.w	r6, r1, r9
 8002378: 3201         	addge	r2, #0x1
 800237a: 9903         	ldr	r1, [sp, #0xc]
 800237c: 9d01         	ldr	r5, [sp, #0x4]
 800237e: 3104         	adds	r1, #0x4
 8002380: 9103         	str	r1, [sp, #0xc]
 8002382: b2c9         	uxtb	r1, r1
 8002384: 42a9         	cmp	r1, r5
 8002386: d1d0         	bne	0x800232a <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x73c> @ imm = #-0x60
 8002388: 9900         	ldr	r1, [sp]
 800238a: f100 0948    	add.w	r9, r0, #0x48
 800238e: b1e9         	cbz	r1, 0x80023cc <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x7de> @ imm = #0x3a
 8002390: 1b99         	subs	r1, r3, r6
 8002392: 0052         	lsls	r2, r2, #0x1
 8002394: 1a75         	subs	r5, r6, r1
 8002396: bfb8         	it	lt
 8002398: 0075         	lsllt	r5, r6, #0x1
 800239a: bfa8         	it	ge
 800239c: 3201         	addge	r2, #0x1
 800239e: 9900         	ldr	r1, [sp]
 80023a0: 2901         	cmp	r1, #0x1
 80023a2: d101         	bne	0x80023a8 <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x7ba> @ imm = #0x2
 80023a4: 462e         	mov	r6, r5
 80023a6: e011         	b	0x80023cc <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x7de> @ imm = #0x22
 80023a8: 1b59         	subs	r1, r3, r5
 80023aa: 0052         	lsls	r2, r2, #0x1
 80023ac: 1a6e         	subs	r6, r5, r1
 80023ae: bfb8         	it	lt
 80023b0: 006e         	lsllt	r6, r5, #0x1
 80023b2: bfa8         	it	ge
 80023b4: 3201         	addge	r2, #0x1
 80023b6: 9900         	ldr	r1, [sp]
 80023b8: 2902         	cmp	r1, #0x2
 80023ba: d007         	beq	0x80023cc <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x7de> @ imm = #0xe
 80023bc: 1b99         	subs	r1, r3, r6
 80023be: 0052         	lsls	r2, r2, #0x1
 80023c0: 1a71         	subs	r1, r6, r1
 80023c2: bfb8         	it	lt
 80023c4: 0071         	lsllt	r1, r6, #0x1
 80023c6: 460e         	mov	r6, r1
 80023c8: bfa8         	it	ge
 80023ca: 3201         	addge	r2, #0x1
 80023cc: ebb6 010a    	subs.w	r1, r6, r10
 80023d0: bfb8         	it	lt
 80023d2: 4419         	addlt	r1, r3
 80023d4: eba3 0301    	sub.w	r3, r3, r1
 80023d8: bfb8         	it	lt
 80023da: 3a01         	sublt	r2, #0x1
 80023dc: e60d         	b	0x8001ffa <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x40c> @ imm = #-0x3e6
 80023de: 1f90         	subs	r0, r2, #0x6
 80023e0: fab0 f080    	clz	r0, r0
 80023e4: 2200         	movs	r2, #0x0
 80023e6: ea4f 1e50    	lsr.w	lr, r0, #0x5
 80023ea: 4610         	mov	r0, r2
 80023ec: 4671         	mov	r1, lr
 80023ee: b007         	add	sp, #0x1c
 80023f0: e8bd 0f00    	pop.w	{r8, r9, r10, r11}
 80023f4: bdf0         	pop	{r4, r5, r6, r7, pc}
 80023f6: f24e 0330    	movw	r3, #0xe030
 80023fa: 4670         	mov	r0, lr
 80023fc: f6c0 0300    	movt	r3, #0x800
 8002400: 4661         	mov	r1, r12
 8002402: f00a f85c    	bl	0x800c4be <core::slice::index::slice_index_fail> @ imm = #0xa0b8
 8002406: f04f 0e01    	mov.w	lr, #0x1
 800240a: e7ee         	b	0x80023ea <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x7fc> @ imm = #-0x24
 800240c: f10a 0a01    	add.w	r10, r10, #0x1
 8002410: e001         	b	0x8002416 <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x828> @ imm = #0x2
 8002412: f10a 0a02    	add.w	r10, r10, #0x2
 8002416: 4642         	mov	r2, r8
 8002418: e001         	b	0x800241e <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x830> @ imm = #0x2
 800241a: f10a 0a03    	add.w	r10, r10, #0x3
 800241e: eb0a 0109    	add.w	r1, r10, r9
 8002422: e9c0 121d    	strd	r1, r2, [r0, #116]
 8002426: f04f 0e01    	mov.w	lr, #0x1
 800242a: 2201         	movs	r2, #0x1
 800242c: e7dd         	b	0x80023ea <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x7fc> @ imm = #-0x46
 800242e: f24e 0310    	movw	r3, #0xe010
 8002432: 4660         	mov	r0, r12
 8002434: f6c0 0300    	movt	r3, #0x800
 8002438: 4671         	mov	r1, lr
 800243a: f00a f840    	bl	0x800c4be <core::slice::index::slice_index_fail> @ imm = #0xa080
 800243e: f24e 522c    	movw	r2, #0xe52c
 8002442: f6c0 0200    	movt	r2, #0x800
 8002446: 4618         	mov	r0, r3
 8002448: 210d         	movs	r1, #0xd
 800244a: f009 fd4b    	bl	0x800bee4 <core::panicking::panic_bounds_check> @ imm = #0x9a96
 800244e: f24e 523c    	movw	r2, #0xe53c
 8002452: f6c0 0200    	movt	r2, #0x800
 8002456: 4618         	mov	r0, r3
 8002458: 210d         	movs	r1, #0xd
 800245a: f009 fd43    	bl	0x800bee4 <core::panicking::panic_bounds_check> @ imm = #0x9a86
 800245e: f24e 526c    	movw	r2, #0xe56c
 8002462: f6c0 0200    	movt	r2, #0x800
 8002466: 4660         	mov	r0, r12
 8002468: 4671         	mov	r1, lr
 800246a: f009 fd3b    	bl	0x800bee4 <core::panicking::panic_bounds_check> @ imm = #0x9a76
 800246e: f24e 527c    	movw	r2, #0xe57c
 8002472: f6c0 0200    	movt	r2, #0x800
 8002476: 4660         	mov	r0, r12
 8002478: 4671         	mov	r1, lr
 800247a: f009 fd33    	bl	0x800bee4 <core::panicking::panic_bounds_check> @ imm = #0x9a66
 800247e: f24e 525c    	movw	r2, #0xe55c
 8002482: 4618         	mov	r0, r3
 8002484: f6c0 0200    	movt	r2, #0x800
 8002488: 4671         	mov	r1, lr
 800248a: f009 fd2b    	bl	0x800bee4 <core::panicking::panic_bounds_check> @ imm = #0x9a56
 800248e: f24e 524c    	movw	r2, #0xe54c
 8002492: 4618         	mov	r0, r3
 8002494: f6c0 0200    	movt	r2, #0x800
 8002498: 2105         	movs	r1, #0x5
 800249a: f009 fd23    	bl	0x800bee4 <core::panicking::panic_bounds_check> @ imm = #0x9a46
 800249e: f24e 0220    	movw	r2, #0xe020
 80024a2: f04f 30ff    	mov.w	r0, #0xffffffff
 80024a6: f6c0 0200    	movt	r2, #0x800
 80024aa: 2105         	movs	r1, #0x5
 80024ac: f009 fd1a    	bl	0x800bee4 <core::panicking::panic_bounds_check> @ imm = #0x9a34

080024b0 <raylar_crcl_encoder_service::benchmark::shift_only::h394b3429a2171dca>:
 80024b0: b5f0         	push	{r4, r5, r6, r7, lr}
 80024b2: af03         	add	r7, sp, #0xc
 80024b4: e92d 0b00    	push.w	{r8, r9, r11}
 80024b8: b082         	sub	sp, #0x8
 80024ba: 6801         	ldr	r1, [r0]
 80024bc: f44f 6380    	mov.w	r3, #0x400
 80024c0: 9101         	str	r1, [sp, #0x4]
 80024c2: a901         	add	r1, sp, #0x4
 80024c4: f645 716c    	movw	r1, #0x5f6c
 80024c8: 9a01         	ldr	r2, [sp, #0x4]
 80024ca: 4401         	add	r1, r0
 80024cc: f002 021f    	and	r2, r2, #0x1f
 80024d0: f8d1 60c0    	ldr.w	r6, [r1, #0xc0]
 80024d4: 3b10         	subs	r3, #0x10
 80024d6: f8d1 50d8    	ldr.w	r5, [r1, #0xd8]
 80024da: f8d1 40f0    	ldr.w	r4, [r1, #0xf0]
 80024de: fa46 f602    	asr.w	r6, r6, r2
 80024e2: f8d1 c108    	ldr.w	r12, [r1, #0x108]
 80024e6: f8c1 60c0    	str.w	r6, [r1, #0xc0]
 80024ea: fa45 f502    	asr.w	r5, r5, r2
 80024ee: f8d1 6150    	ldr.w	r6, [r1, #0x150]
 80024f2: fa44 f402    	asr.w	r4, r4, r2
 80024f6: f8c1 50d8    	str.w	r5, [r1, #0xd8]
 80024fa: f8d1 5168    	ldr.w	r5, [r1, #0x168]
 80024fe: f8d1 9180    	ldr.w	r9, [r1, #0x180]
 8002502: fa46 f602    	asr.w	r6, r6, r2
 8002506: f8c1 40f0    	str.w	r4, [r1, #0xf0]
 800250a: fa4c f402    	asr.w	r4, r12, r2
 800250e: f8d1 c198    	ldr.w	r12, [r1, #0x198]
 8002512: f8c1 6150    	str.w	r6, [r1, #0x150]
 8002516: fa45 f602    	asr.w	r6, r5, r2
 800251a: f8c1 6168    	str.w	r6, [r1, #0x168]
 800251e: fa49 f602    	asr.w	r6, r9, r2
 8002522: f8c1 6180    	str.w	r6, [r1, #0x180]
 8002526: fa4c f602    	asr.w	r6, r12, r2
 800252a: f8c1 6198    	str.w	r6, [r1, #0x198]
 800252e: f8d1 61b0    	ldr.w	r6, [r1, #0x1b0]
 8002532: f8d1 e120    	ldr.w	lr, [r1, #0x120]
 8002536: f8d1 8138    	ldr.w	r8, [r1, #0x138]
 800253a: fa46 f602    	asr.w	r6, r6, r2
 800253e: f8c1 4108    	str.w	r4, [r1, #0x108]
 8002542: f8c1 61b0    	str.w	r6, [r1, #0x1b0]
 8002546: fa4e f402    	asr.w	r4, lr, r2
 800254a: f8d1 61c8    	ldr.w	r6, [r1, #0x1c8]
 800254e: f8c1 4120    	str.w	r4, [r1, #0x120]
 8002552: fa48 f402    	asr.w	r4, r8, r2
 8002556: f8c1 4138    	str.w	r4, [r1, #0x138]
 800255a: fa46 f602    	asr.w	r6, r6, r2
 800255e: f8c1 61c8    	str.w	r6, [r1, #0x1c8]
 8002562: f8d1 61e0    	ldr.w	r6, [r1, #0x1e0]
 8002566: fa46 f602    	asr.w	r6, r6, r2
 800256a: f8c1 61e0    	str.w	r6, [r1, #0x1e0]
 800256e: f8d1 61f8    	ldr.w	r6, [r1, #0x1f8]
 8002572: fa46 f602    	asr.w	r6, r6, r2
 8002576: f8c1 61f8    	str.w	r6, [r1, #0x1f8]
 800257a: f8d1 6210    	ldr.w	r6, [r1, #0x210]
 800257e: fa46 f602    	asr.w	r6, r6, r2
 8002582: f8c1 6210    	str.w	r6, [r1, #0x210]
 8002586: f8d1 6228    	ldr.w	r6, [r1, #0x228]
 800258a: fa46 f602    	asr.w	r6, r6, r2
 800258e: f8c1 6228    	str.w	r6, [r1, #0x228]
 8002592: f501 71c0    	add.w	r1, r1, #0x180
 8002596: d19b         	bne	0x80024d0 <raylar_crcl_encoder_service::benchmark::shift_only::h394b3429a2171dca+0x20> @ imm = #-0xca
 8002598: 6881         	ldr	r1, [r0, #0x8]
 800259a: f44f 6380    	mov.w	r3, #0x400
 800259e: 9101         	str	r1, [sp, #0x4]
 80025a0: a901         	add	r1, sp, #0x4
 80025a2: f645 61b0    	movw	r1, #0x5eb0
 80025a6: 9a01         	ldr	r2, [sp, #0x4]
 80025a8: 4401         	add	r1, r0
 80025aa: f002 021f    	and	r2, r2, #0x1f
 80025ae: f8d1 c180    	ldr.w	r12, [r1, #0x180]
 80025b2: 3b10         	subs	r3, #0x10
 80025b4: f8d1 e198    	ldr.w	lr, [r1, #0x198]
 80025b8: f8d1 41b0    	ldr.w	r4, [r1, #0x1b0]
 80025bc: fa4c f602    	asr.w	r6, r12, r2
 80025c0: f8d1 51c8    	ldr.w	r5, [r1, #0x1c8]
 80025c4: f8c1 6180    	str.w	r6, [r1, #0x180]
 80025c8: fa4e f602    	asr.w	r6, lr, r2
 80025cc: f8c1 6198    	str.w	r6, [r1, #0x198]
 80025d0: fa44 f402    	asr.w	r4, r4, r2
 80025d4: f8d1 6228    	ldr.w	r6, [r1, #0x228]
 80025d8: f8d1 9240    	ldr.w	r9, [r1, #0x240]
 80025dc: f8c1 41b0    	str.w	r4, [r1, #0x1b0]
 80025e0: fa45 f402    	asr.w	r4, r5, r2
 80025e4: f8d1 5258    	ldr.w	r5, [r1, #0x258]
 80025e8: fa46 f602    	asr.w	r6, r6, r2
 80025ec: f8c1 6228    	str.w	r6, [r1, #0x228]
 80025f0: fa49 f602    	asr.w	r6, r9, r2
 80025f4: f8c1 6240    	str.w	r6, [r1, #0x240]
 80025f8: fa45 f602    	asr.w	r6, r5, r2
 80025fc: f8d1 81e0    	ldr.w	r8, [r1, #0x1e0]
 8002600: f8c1 6258    	str.w	r6, [r1, #0x258]
 8002604: f8d1 6270    	ldr.w	r6, [r1, #0x270]
 8002608: f8d1 c1f8    	ldr.w	r12, [r1, #0x1f8]
 800260c: f8d1 e210    	ldr.w	lr, [r1, #0x210]
 8002610: fa46 f602    	asr.w	r6, r6, r2
 8002614: f8c1 41c8    	str.w	r4, [r1, #0x1c8]
 8002618: f8c1 6270    	str.w	r6, [r1, #0x270]
 800261c: fa48 f402    	asr.w	r4, r8, r2
 8002620: f8d1 6288    	ldr.w	r6, [r1, #0x288]
 8002624: f8c1 41e0    	str.w	r4, [r1, #0x1e0]
 8002628: fa4c f402    	asr.w	r4, r12, r2
 800262c: f8c1 41f8    	str.w	r4, [r1, #0x1f8]
 8002630: fa4e f402    	asr.w	r4, lr, r2
 8002634: fa46 f602    	asr.w	r6, r6, r2
 8002638: f8c1 4210    	str.w	r4, [r1, #0x210]
 800263c: f8c1 6288    	str.w	r6, [r1, #0x288]
 8002640: f8d1 62a0    	ldr.w	r6, [r1, #0x2a0]
 8002644: fa46 f602    	asr.w	r6, r6, r2
 8002648: f8c1 62a0    	str.w	r6, [r1, #0x2a0]
 800264c: f8d1 62b8    	ldr.w	r6, [r1, #0x2b8]
 8002650: fa46 f602    	asr.w	r6, r6, r2
 8002654: f8c1 62b8    	str.w	r6, [r1, #0x2b8]
 8002658: f8d1 62d0    	ldr.w	r6, [r1, #0x2d0]
 800265c: fa46 f602    	asr.w	r6, r6, r2
 8002660: f8c1 62d0    	str.w	r6, [r1, #0x2d0]
 8002664: f8d1 62e8    	ldr.w	r6, [r1, #0x2e8]
 8002668: fa46 f602    	asr.w	r6, r6, r2
 800266c: f8c1 62e8    	str.w	r6, [r1, #0x2e8]
 8002670: f501 71c0    	add.w	r1, r1, #0x180
 8002674: d19b         	bne	0x80025ae <raylar_crcl_encoder_service::benchmark::shift_only::h394b3429a2171dca+0xfe> @ imm = #-0xca
 8002676: 6901         	ldr	r1, [r0, #0x10]
 8002678: f44f 6380    	mov.w	r3, #0x400
 800267c: 9101         	str	r1, [sp, #0x4]
 800267e: a901         	add	r1, sp, #0x4
 8002680: f645 7174    	movw	r1, #0x5f74
 8002684: 9a01         	ldr	r2, [sp, #0x4]
 8002686: 4401         	add	r1, r0
 8002688: f002 021f    	and	r2, r2, #0x1f
 800268c: f8d1 60c0    	ldr.w	r6, [r1, #0xc0]
 8002690: 3b10         	subs	r3, #0x10
 8002692: f8d1 50d8    	ldr.w	r5, [r1, #0xd8]
 8002696: f8d1 40f0    	ldr.w	r4, [r1, #0xf0]
 800269a: fa46 f602    	asr.w	r6, r6, r2
 800269e: f8d1 c108    	ldr.w	r12, [r1, #0x108]
 80026a2: f8c1 60c0    	str.w	r6, [r1, #0xc0]
 80026a6: fa45 f502    	asr.w	r5, r5, r2
 80026aa: f8d1 6150    	ldr.w	r6, [r1, #0x150]
 80026ae: fa44 f402    	asr.w	r4, r4, r2
 80026b2: f8c1 50d8    	str.w	r5, [r1, #0xd8]
 80026b6: f8d1 5168    	ldr.w	r5, [r1, #0x168]
 80026ba: f8d1 9180    	ldr.w	r9, [r1, #0x180]
 80026be: fa46 f602    	asr.w	r6, r6, r2
 80026c2: f8c1 40f0    	str.w	r4, [r1, #0xf0]
 80026c6: fa4c f402    	asr.w	r4, r12, r2
 80026ca: f8d1 c198    	ldr.w	r12, [r1, #0x198]
 80026ce: f8c1 6150    	str.w	r6, [r1, #0x150]
 80026d2: fa45 f602    	asr.w	r6, r5, r2
 80026d6: f8c1 6168    	str.w	r6, [r1, #0x168]
 80026da: fa49 f602    	asr.w	r6, r9, r2
 80026de: f8c1 6180    	str.w	r6, [r1, #0x180]
 80026e2: fa4c f602    	asr.w	r6, r12, r2
 80026e6: f8c1 6198    	str.w	r6, [r1, #0x198]
 80026ea: f8d1 61b0    	ldr.w	r6, [r1, #0x1b0]
 80026ee: f8d1 e120    	ldr.w	lr, [r1, #0x120]
 80026f2: f8d1 8138    	ldr.w	r8, [r1, #0x138]
 80026f6: fa46 f602    	asr.w	r6, r6, r2
 80026fa: f8c1 4108    	str.w	r4, [r1, #0x108]
 80026fe: f8c1 61b0    	str.w	r6, [r1, #0x1b0]
 8002702: fa4e f402    	asr.w	r4, lr, r2
 8002706: f8d1 61c8    	ldr.w	r6, [r1, #0x1c8]
 800270a: f8c1 4120    	str.w	r4, [r1, #0x120]
 800270e: fa48 f402    	asr.w	r4, r8, r2
 8002712: f8c1 4138    	str.w	r4, [r1, #0x138]
 8002716: fa46 f602    	asr.w	r6, r6, r2
 800271a: f8c1 61c8    	str.w	r6, [r1, #0x1c8]
 800271e: f8d1 61e0    	ldr.w	r6, [r1, #0x1e0]
 8002722: fa46 f602    	asr.w	r6, r6, r2
 8002726: f8c1 61e0    	str.w	r6, [r1, #0x1e0]
 800272a: f8d1 61f8    	ldr.w	r6, [r1, #0x1f8]
 800272e: fa46 f602    	asr.w	r6, r6, r2
 8002732: f8c1 61f8    	str.w	r6, [r1, #0x1f8]
 8002736: f8d1 6210    	ldr.w	r6, [r1, #0x210]
 800273a: fa46 f602    	asr.w	r6, r6, r2
 800273e: f8c1 6210    	str.w	r6, [r1, #0x210]
 8002742: f8d1 6228    	ldr.w	r6, [r1, #0x228]
 8002746: fa46 f602    	asr.w	r6, r6, r2
 800274a: f8c1 6228    	str.w	r6, [r1, #0x228]
 800274e: f501 71c0    	add.w	r1, r1, #0x180
 8002752: d19b         	bne	0x800268c <raylar_crcl_encoder_service::benchmark::shift_only::h394b3429a2171dca+0x1dc> @ imm = #-0xca
 8002754: 6981         	ldr	r1, [r0, #0x18]
 8002756: f44f 6380    	mov.w	r3, #0x400
 800275a: 9101         	str	r1, [sp, #0x4]
 800275c: a901         	add	r1, sp, #0x4
 800275e: f645 61b8    	movw	r1, #0x5eb8
 8002762: 9a01         	ldr	r2, [sp, #0x4]
 8002764: 4401         	add	r1, r0
 8002766: f002 021f    	and	r2, r2, #0x1f
 800276a: f8d1 6180    	ldr.w	r6, [r1, #0x180]
 800276e: 3b10         	subs	r3, #0x10
 8002770: f8d1 5198    	ldr.w	r5, [r1, #0x198]
 8002774: f8d1 41b0    	ldr.w	r4, [r1, #0x1b0]
 8002778: fa46 f602    	asr.w	r6, r6, r2
 800277c: f8d1 c1c8    	ldr.w	r12, [r1, #0x1c8]
 8002780: f8c1 6180    	str.w	r6, [r1, #0x180]
 8002784: fa45 f502    	asr.w	r5, r5, r2
 8002788: f8d1 6210    	ldr.w	r6, [r1, #0x210]
 800278c: fa44 f402    	asr.w	r4, r4, r2
 8002790: f8c1 5198    	str.w	r5, [r1, #0x198]
 8002794: f8d1 5228    	ldr.w	r5, [r1, #0x228]
 8002798: f8d1 9240    	ldr.w	r9, [r1, #0x240]
 800279c: fa46 f602    	asr.w	r6, r6, r2
 80027a0: f8c1 41b0    	str.w	r4, [r1, #0x1b0]
 80027a4: fa4c f402    	asr.w	r4, r12, r2
 80027a8: f8d1 c258    	ldr.w	r12, [r1, #0x258]
 80027ac: f8c1 6210    	str.w	r6, [r1, #0x210]
 80027b0: fa45 f602    	asr.w	r6, r5, r2
 80027b4: f8c1 6228    	str.w	r6, [r1, #0x228]
 80027b8: fa49 f602    	asr.w	r6, r9, r2
 80027bc: f8c1 6240    	str.w	r6, [r1, #0x240]
 80027c0: fa4c f602    	asr.w	r6, r12, r2
 80027c4: f8c1 6258    	str.w	r6, [r1, #0x258]
 80027c8: f8d1 6270    	ldr.w	r6, [r1, #0x270]
 80027cc: f8d1 e1e0    	ldr.w	lr, [r1, #0x1e0]
 80027d0: f8d1 81f8    	ldr.w	r8, [r1, #0x1f8]
 80027d4: fa46 f602    	asr.w	r6, r6, r2
 80027d8: f8c1 41c8    	str.w	r4, [r1, #0x1c8]
 80027dc: f8c1 6270    	str.w	r6, [r1, #0x270]
 80027e0: fa4e f402    	asr.w	r4, lr, r2
 80027e4: f8d1 6288    	ldr.w	r6, [r1, #0x288]
 80027e8: f8c1 41e0    	str.w	r4, [r1, #0x1e0]
 80027ec: fa48 f402    	asr.w	r4, r8, r2
 80027f0: f8c1 41f8    	str.w	r4, [r1, #0x1f8]
 80027f4: fa46 f602    	asr.w	r6, r6, r2
 80027f8: f8c1 6288    	str.w	r6, [r1, #0x288]
 80027fc: f8d1 62a0    	ldr.w	r6, [r1, #0x2a0]
 8002800: fa46 f602    	asr.w	r6, r6, r2
 8002804: f8c1 62a0    	str.w	r6, [r1, #0x2a0]
 8002808: f8d1 62b8    	ldr.w	r6, [r1, #0x2b8]
 800280c: fa46 f602    	asr.w	r6, r6, r2
 8002810: f8c1 62b8    	str.w	r6, [r1, #0x2b8]
 8002814: f8d1 62d0    	ldr.w	r6, [r1, #0x2d0]
 8002818: fa46 f602    	asr.w	r6, r6, r2
 800281c: f8c1 62d0    	str.w	r6, [r1, #0x2d0]
 8002820: f8d1 62e8    	ldr.w	r6, [r1, #0x2e8]
 8002824: fa46 f602    	asr.w	r6, r6, r2
 8002828: f8c1 62e8    	str.w	r6, [r1, #0x2e8]
 800282c: f501 71c0    	add.w	r1, r1, #0x180
 8002830: d19b         	bne	0x800276a <raylar_crcl_encoder_service::benchmark::shift_only::h394b3429a2171dca+0x2ba> @ imm = #-0xca
 8002832: 6a01         	ldr	r1, [r0, #0x20]
 8002834: f44f 6380    	mov.w	r3, #0x400
 8002838: 9101         	str	r1, [sp, #0x4]
 800283a: a901         	add	r1, sp, #0x4
 800283c: f645 717c    	movw	r1, #0x5f7c
 8002840: 9a01         	ldr	r2, [sp, #0x4]
 8002842: 4401         	add	r1, r0
 8002844: f002 021f    	and	r2, r2, #0x1f
 8002848: f8d1 60c0    	ldr.w	r6, [r1, #0xc0]
 800284c: 3b10         	subs	r3, #0x10
 800284e: f8d1 50d8    	ldr.w	r5, [r1, #0xd8]
 8002852: f8d1 40f0    	ldr.w	r4, [r1, #0xf0]
 8002856: fa46 f602    	asr.w	r6, r6, r2
 800285a: f8d1 c108    	ldr.w	r12, [r1, #0x108]
 800285e: f8c1 60c0    	str.w	r6, [r1, #0xc0]
 8002862: fa45 f502    	asr.w	r5, r5, r2
 8002866: f8d1 6150    	ldr.w	r6, [r1, #0x150]
 800286a: fa44 f402    	asr.w	r4, r4, r2
 800286e: f8c1 50d8    	str.w	r5, [r1, #0xd8]
 8002872: f8d1 5168    	ldr.w	r5, [r1, #0x168]
 8002876: f8d1 9180    	ldr.w	r9, [r1, #0x180]
 800287a: fa46 f602    	asr.w	r6, r6, r2
 800287e: f8c1 40f0    	str.w	r4, [r1, #0xf0]
 8002882: fa4c f402    	asr.w	r4, r12, r2
 8002886: f8d1 c198    	ldr.w	r12, [r1, #0x198]
 800288a: f8c1 6150    	str.w	r6, [r1, #0x150]
 800288e: fa45 f602    	asr.w	r6, r5, r2
 8002892: f8c1 6168    	str.w	r6, [r1, #0x168]
 8002896: fa49 f602    	asr.w	r6, r9, r2
 800289a: f8c1 6180    	str.w	r6, [r1, #0x180]
 800289e: fa4c f602    	asr.w	r6, r12, r2
 80028a2: f8c1 6198    	str.w	r6, [r1, #0x198]
 80028a6: f8d1 61b0    	ldr.w	r6, [r1, #0x1b0]
 80028aa: f8d1 e120    	ldr.w	lr, [r1, #0x120]
 80028ae: f8d1 8138    	ldr.w	r8, [r1, #0x138]
 80028b2: fa46 f602    	asr.w	r6, r6, r2
 80028b6: f8c1 4108    	str.w	r4, [r1, #0x108]
 80028ba: f8c1 61b0    	str.w	r6, [r1, #0x1b0]
 80028be: fa4e f402    	asr.w	r4, lr, r2
 80028c2: f8d1 61c8    	ldr.w	r6, [r1, #0x1c8]
 80028c6: f8c1 4120    	str.w	r4, [r1, #0x120]
 80028ca: fa48 f402    	asr.w	r4, r8, r2
 80028ce: f8c1 4138    	str.w	r4, [r1, #0x138]
 80028d2: fa46 f602    	asr.w	r6, r6, r2
 80028d6: f8c1 61c8    	str.w	r6, [r1, #0x1c8]
 80028da: f8d1 61e0    	ldr.w	r6, [r1, #0x1e0]
 80028de: fa46 f602    	asr.w	r6, r6, r2
 80028e2: f8c1 61e0    	str.w	r6, [r1, #0x1e0]
 80028e6: f8d1 61f8    	ldr.w	r6, [r1, #0x1f8]
 80028ea: fa46 f602    	asr.w	r6, r6, r2
 80028ee: f8c1 61f8    	str.w	r6, [r1, #0x1f8]
 80028f2: f8d1 6210    	ldr.w	r6, [r1, #0x210]
 80028f6: fa46 f602    	asr.w	r6, r6, r2
 80028fa: f8c1 6210    	str.w	r6, [r1, #0x210]
 80028fe: f8d1 6228    	ldr.w	r6, [r1, #0x228]
 8002902: fa46 f602    	asr.w	r6, r6, r2
 8002906: f8c1 6228    	str.w	r6, [r1, #0x228]
 800290a: f501 71c0    	add.w	r1, r1, #0x180
 800290e: d19b         	bne	0x8002848 <raylar_crcl_encoder_service::benchmark::shift_only::h394b3429a2171dca+0x398> @ imm = #-0xca
 8002910: f246 0128    	movw	r1, #0x6028
 8002914: 4401         	add	r1, r0
 8002916: 9100         	str	r1, [sp]
 8002918: 4669         	mov	r1, sp
 800291a: f246 012c    	movw	r1, #0x602c
 800291e: 5840         	ldr	r0, [r0, r1]
 8002920: b002         	add	sp, #0x8
 8002922: e8bd 0b00    	pop.w	{r8, r9, r11}
 8002926: bdf0         	pop	{r4, r5, r6, r7, pc}

08002928 <raylar_crcl_encoder_service::benchmark::encode_bits::h865b9e06648afa8c>:
 8002928: b5f0         	push	{r4, r5, r6, r7, lr}
 800292a: af03         	add	r7, sp, #0xc
 800292c: e92d 0f00    	push.w	{r8, r9, r10, r11}
 8002930: b097         	sub	sp, #0x5c
 8002932: e9cd 1210    	strd	r1, r2, [sp, #64]
 8002936: 4601         	mov	r1, r0
 8002938: f500 30b8    	add.w	r0, r0, #0x17000
 800293c: f501 33e0    	add.w	r3, r1, #0x1c000
 8002940: 3010         	adds	r0, #0x10
 8002942: 900e         	str	r0, [sp, #0x38]
 8002944: f24c 0028    	movw	r0, #0xc028
 8002948: 180a         	adds	r2, r1, r0
 800294a: 2058         	movs	r0, #0x58
 800294c: f641 6658    	movw	r6, #0x1e58
 8002950: f641 6556    	movw	r5, #0x1e56
 8002954: f04f 3cff    	mov.w	r12, #0xffffffff
 8002958: f04f 0a00    	mov.w	r10, #0x0
 800295c: 9012         	str	r0, [sp, #0x48]
 800295e: 9812         	ldr	r0, [sp, #0x48]
 8002960: 42b0         	cmp	r0, r6
 8002962: f200 81d9    	bhi.w	0x8002d18 <raylar_crcl_encoder_service::benchmark::encode_bits::h865b9e06648afa8c+0x3f0> @ imm = #0x3b2
 8002966: 9812         	ldr	r0, [sp, #0x48]
 8002968: 42a8         	cmp	r0, r5
 800296a: f200 8191    	bhi.w	0x8002c90 <raylar_crcl_encoder_service::benchmark::encode_bits::h865b9e06648afa8c+0x368> @ imm = #0x322
 800296e: eb01 00ca    	add.w	r0, r1, r10, lsl #3
 8002972: 920b         	str	r2, [sp, #0x2c]
 8002974: f04f 0800    	mov.w	r8, #0x0
 8002978: 9104         	str	r1, [sp, #0x10]
 800297a: 7900         	ldrb	r0, [r0, #0x4]
 800297c: 900d         	str	r0, [sp, #0x34]
 800297e: f10a 0001    	add.w	r0, r10, #0x1
 8002982: 9003         	str	r0, [sp, #0xc]
 8002984: f852 2020    	ldr.w	r2, [r2, r0, lsl #2]
 8002988: 9812         	ldr	r0, [sp, #0x48]
 800298a: eba6 0e00    	sub.w	lr, r6, r0
 800298e: eb03 0900    	add.w	r9, r3, r0
 8002992: 260a         	movs	r6, #0xa
 8002994: f018 0407    	ands	r4, r8, #0x7
 8002998: ea4f 03d8    	lsr.w	r3, r8, #0x3
 800299c: d105         	bne	0x80029aa <raylar_crcl_encoder_service::benchmark::encode_bits::h865b9e06648afa8c+0x82> @ imm = #0xa
 800299e: 4573         	cmp	r3, lr
 80029a0: f080 81cd    	bhs.w	0x8002d3e <raylar_crcl_encoder_service::benchmark::encode_bits::h865b9e06648afa8c+0x416> @ imm = #0x39a
 80029a4: 2000         	movs	r0, #0x0
 80029a6: f809 0003    	strb.w	r0, [r9, r3]
 80029aa: 4573         	cmp	r3, lr
 80029ac: f080 81bf    	bhs.w	0x8002d2e <raylar_crcl_encoder_service::benchmark::encode_bits::h865b9e06648afa8c+0x406> @ imm = #0x37e
 80029b0: f1c4 0008    	rsb.w	r0, r4, #0x8
 80029b4: 4634         	mov	r4, r6
 80029b6: 42b0         	cmp	r0, r6
 80029b8: bf38         	it	lo
 80029ba: 4604         	movlo	r4, r0
 80029bc: 1b36         	subs	r6, r6, r4
 80029be: eba0 0004    	sub.w	r0, r0, r4
 80029c2: f006 011f    	and	r1, r6, #0x1f
 80029c6: fa0c f504    	lsl.w	r5, r12, r4
 80029ca: f819 b003    	ldrb.w	r11, [r9, r3]
 80029ce: f000 0007    	and	r0, r0, #0x7
 80029d2: fa22 f101    	lsr.w	r1, r2, r1
 80029d6: ea21 0105    	bic.w	r1, r1, r5
 80029da: 44a0         	add	r8, r4
 80029dc: fa01 f000    	lsl.w	r0, r1, r0
 80029e0: ea40 000b    	orr.w	r0, r0, r11
 80029e4: f809 0003    	strb.w	r0, [r9, r3]
 80029e8: d1d4         	bne	0x8002994 <raylar_crcl_encoder_service::benchmark::encode_bits::h865b9e06648afa8c+0x6c> @ imm = #-0x58
 80029ea: 980b         	ldr	r0, [sp, #0x2c]
 80029ec: f10a 0101    	add.w	r1, r10, #0x1
 80029f0: f04f 0b01    	mov.w	r11, #0x1
 80029f4: f850 3021    	ldr.w	r3, [r0, r1, lsl #2]
 80029f8: eb00 008a    	add.w	r0, r0, r10, lsl #2
 80029fc: 900a         	str	r0, [sp, #0x28]
 80029fe: ebca 208a    	rsb	r0, r10, r10, lsl #10
 8002a02: 990e         	ldr	r1, [sp, #0x38]
 8002a04: eb01 0080    	add.w	r0, r1, r0, lsl #2
 8002a08: 990d         	ldr	r1, [sp, #0x34]
 8002a0a: 9009         	str	r0, [sp, #0x24]
 8002a0c: f001 041f    	and	r4, r1, #0x1f
 8002a10: 9408         	str	r4, [sp, #0x20]
 8002a12: fa0c f004    	lsl.w	r0, r12, r4
 8002a16: 43c0         	mvns	r0, r0
 8002a18: 9005         	str	r0, [sp, #0x14]
 8002a1a: ea4f 00ce    	lsl.w	r0, lr, #0x3
 8002a1e: 900f         	str	r0, [sp, #0x3c]
 8002a20: 1c48         	adds	r0, r1, #0x1
 8002a22: 900c         	str	r0, [sp, #0x30]
 8002a24: fa0b f004    	lsl.w	r0, r11, r4
 8002a28: 9007         	str	r0, [sp, #0x1c]
 8002a2a: 3801         	subs	r0, #0x1
 8002a2c: 9006         	str	r0, [sp, #0x18]
 8002a2e: 9811         	ldr	r0, [sp, #0x44]
 8002a30: b178         	cbz	r0, 0x8002a52 <raylar_crcl_encoder_service::benchmark::encode_bits::h865b9e06648afa8c+0x12a> @ imm = #0x1e
 8002a32: eb0b 004b    	add.w	r0, r11, r11, lsl #1
 8002a36: 990a         	ldr	r1, [sp, #0x28]
 8002a38: eb01 00c0    	add.w	r0, r1, r0, lsl #3
 8002a3c: 6842         	ldr	r2, [r0, #0x4]
 8002a3e: 1ad0         	subs	r0, r2, r3
 8002a40: 0043         	lsls	r3, r0, #0x1
 8002a42: ea83 70e0    	eor.w	r0, r3, r0, asr #31
 8002a46: 4613         	mov	r3, r2
 8002a48: 9910         	ldr	r1, [sp, #0x40]
 8002a4a: fa20 fa04    	lsr.w	r10, r0, r4
 8002a4e: b941         	cbnz	r1, 0x8002a62 <raylar_crcl_encoder_service::benchmark::encode_bits::h865b9e06648afa8c+0x13a> @ imm = #0x10
 8002a50: e04e         	b	0x8002af0 <raylar_crcl_encoder_service::benchmark::encode_bits::h865b9e06648afa8c+0x1c8> @ imm = #0x9c
 8002a52: 9809         	ldr	r0, [sp, #0x24]
 8002a54: f850 002b    	ldr.w	r0, [r0, r11, lsl #2]
 8002a58: 9910         	ldr	r1, [sp, #0x40]
 8002a5a: fa20 fa04    	lsr.w	r10, r0, r4
 8002a5e: 2900         	cmp	r1, #0x0
 8002a60: d046         	beq	0x8002af0 <raylar_crcl_encoder_service::benchmark::encode_bits::h865b9e06648afa8c+0x1c8> @ imm = #0x8c
 8002a62: 990c         	ldr	r1, [sp, #0x30]
 8002a64: eb01 060a    	add.w	r6, r1, r10
 8002a68: 2e21         	cmp	r6, #0x21
 8002a6a: d241         	bhs	0x8002af0 <raylar_crcl_encoder_service::benchmark::encode_bits::h865b9e06648afa8c+0x1c8> @ imm = #0x82
 8002a6c: eb16 0208    	adds.w	r2, r6, r8
 8002a70: f080 8126    	bhs.w	0x8002cc0 <raylar_crcl_encoder_service::benchmark::encode_bits::h865b9e06648afa8c+0x398> @ imm = #0x24c
 8002a74: 990f         	ldr	r1, [sp, #0x3c]
 8002a76: 428a         	cmp	r2, r1
 8002a78: f200 8122    	bhi.w	0x8002cc0 <raylar_crcl_encoder_service::benchmark::encode_bits::h865b9e06648afa8c+0x398> @ imm = #0x244
 8002a7c: 9302         	str	r3, [sp, #0x8]
 8002a7e: b37e         	cbz	r6, 0x8002ae0 <raylar_crcl_encoder_service::benchmark::encode_bits::h865b9e06648afa8c+0x1b8> @ imm = #0x5e
 8002a80: 9906         	ldr	r1, [sp, #0x18]
 8002a82: 4008         	ands	r0, r1
 8002a84: 9907         	ldr	r1, [sp, #0x1c]
 8002a86: ea40 0a01    	orr.w	r10, r0, r1
 8002a8a: f018 0007    	ands	r0, r8, #0x7
 8002a8e: ea4f 03d8    	lsr.w	r3, r8, #0x3
 8002a92: d105         	bne	0x8002aa0 <raylar_crcl_encoder_service::benchmark::encode_bits::h865b9e06648afa8c+0x178> @ imm = #0xa
 8002a94: 4573         	cmp	r3, lr
 8002a96: f080 8152    	bhs.w	0x8002d3e <raylar_crcl_encoder_service::benchmark::encode_bits::h865b9e06648afa8c+0x416> @ imm = #0x2a4
 8002a9a: 2100         	movs	r1, #0x0
 8002a9c: f809 1003    	strb.w	r1, [r9, r3]
 8002aa0: 4573         	cmp	r3, lr
 8002aa2: f080 8144    	bhs.w	0x8002d2e <raylar_crcl_encoder_service::benchmark::encode_bits::h865b9e06648afa8c+0x406> @ imm = #0x288
 8002aa6: f1c0 0008    	rsb.w	r0, r0, #0x8
 8002aaa: 4634         	mov	r4, r6
 8002aac: 42b0         	cmp	r0, r6
 8002aae: bf38         	it	lo
 8002ab0: 4604         	movlo	r4, r0
 8002ab2: 1b36         	subs	r6, r6, r4
 8002ab4: eba0 0004    	sub.w	r0, r0, r4
 8002ab8: f006 011f    	and	r1, r6, #0x1f
 8002abc: fa0c f504    	lsl.w	r5, r12, r4
 8002ac0: f819 2003    	ldrb.w	r2, [r9, r3]
 8002ac4: f000 0007    	and	r0, r0, #0x7
 8002ac8: fa2a f101    	lsr.w	r1, r10, r1
 8002acc: ea21 0105    	bic.w	r1, r1, r5
 8002ad0: 44a0         	add	r8, r4
 8002ad2: fa01 f000    	lsl.w	r0, r1, r0
 8002ad6: ea40 0002    	orr.w	r0, r0, r2
 8002ada: f809 0003    	strb.w	r0, [r9, r3]
 8002ade: d1d4         	bne	0x8002a8a <raylar_crcl_encoder_service::benchmark::encode_bits::h865b9e06648afa8c+0x162> @ imm = #-0x58
 8002ae0: 9b02         	ldr	r3, [sp, #0x8]
 8002ae2: f10b 0b01    	add.w	r11, r11, #0x1
 8002ae6: 9c08         	ldr	r4, [sp, #0x20]
 8002ae8: f5bb 6f80    	cmp.w	r11, #0x400
 8002aec: d19f         	bne	0x8002a2e <raylar_crcl_encoder_service::benchmark::encode_bits::h865b9e06648afa8c+0x106> @ imm = #-0xc2
 8002aee: e090         	b	0x8002c12 <raylar_crcl_encoder_service::benchmark::encode_bits::h865b9e06648afa8c+0x2ea> @ imm = #0x120
 8002af0: 990f         	ldr	r1, [sp, #0x3c]
 8002af2: eba1 0208    	sub.w	r2, r1, r8
 8002af6: 4592         	cmp	r10, r2
 8002af8: f080 80d6    	bhs.w	0x8002ca8 <raylar_crcl_encoder_service::benchmark::encode_bits::h865b9e06648afa8c+0x380> @ imm = #0x1ac
 8002afc: f1ba 0f00    	cmp.w	r10, #0x0
 8002b00: d028         	beq	0x8002b54 <raylar_crcl_encoder_service::benchmark::encode_bits::h865b9e06648afa8c+0x22c> @ imm = #0x50
 8002b02: f1ba 0f20    	cmp.w	r10, #0x20
 8002b06: 4656         	mov	r6, r10
 8002b08: bf28         	it	hs
 8002b0a: 2620         	movhs	r6, #0x20
 8002b0c: eb18 0206    	adds.w	r2, r8, r6
 8002b10: f080 80ca    	bhs.w	0x8002ca8 <raylar_crcl_encoder_service::benchmark::encode_bits::h865b9e06648afa8c+0x380> @ imm = #0x194
 8002b14: 990f         	ldr	r1, [sp, #0x3c]
 8002b16: 428a         	cmp	r2, r1
 8002b18: f200 80c6    	bhi.w	0x8002ca8 <raylar_crcl_encoder_service::benchmark::encode_bits::h865b9e06648afa8c+0x380> @ imm = #0x18c
 8002b1c: 461d         	mov	r5, r3
 8002b1e: 4632         	mov	r2, r6
 8002b20: f018 0407    	ands	r4, r8, #0x7
 8002b24: ea4f 03d8    	lsr.w	r3, r8, #0x3
 8002b28: d105         	bne	0x8002b36 <raylar_crcl_encoder_service::benchmark::encode_bits::h865b9e06648afa8c+0x20e> @ imm = #0xa
 8002b2a: 4573         	cmp	r3, lr
 8002b2c: f080 8107    	bhs.w	0x8002d3e <raylar_crcl_encoder_service::benchmark::encode_bits::h865b9e06648afa8c+0x416> @ imm = #0x20e
 8002b30: 2100         	movs	r1, #0x0
 8002b32: f809 1003    	strb.w	r1, [r9, r3]
 8002b36: 4573         	cmp	r3, lr
 8002b38: f080 80f9    	bhs.w	0x8002d2e <raylar_crcl_encoder_service::benchmark::encode_bits::h865b9e06648afa8c+0x406> @ imm = #0x1f2
 8002b3c: f1c4 0308    	rsb.w	r3, r4, #0x8
 8002b40: 4293         	cmp	r3, r2
 8002b42: bf28         	it	hs
 8002b44: 4613         	movhs	r3, r2
 8002b46: 4498         	add	r8, r3
 8002b48: 1ad2         	subs	r2, r2, r3
 8002b4a: d1e9         	bne	0x8002b20 <raylar_crcl_encoder_service::benchmark::encode_bits::h865b9e06648afa8c+0x1f8> @ imm = #-0x2e
 8002b4c: ebba 0a06    	subs.w	r10, r10, r6
 8002b50: 462b         	mov	r3, r5
 8002b52: d1d6         	bne	0x8002b02 <raylar_crcl_encoder_service::benchmark::encode_bits::h865b9e06648afa8c+0x1da> @ imm = #-0x54
 8002b54: 990f         	ldr	r1, [sp, #0x3c]
 8002b56: 4588         	cmp	r8, r1
 8002b58: f080 80a6    	bhs.w	0x8002ca8 <raylar_crcl_encoder_service::benchmark::encode_bits::h865b9e06648afa8c+0x380> @ imm = #0x14c
 8002b5c: f018 0207    	ands	r2, r8, #0x7
 8002b60: 9302         	str	r3, [sp, #0x8]
 8002b62: ea4f 03d8    	lsr.w	r3, r8, #0x3
 8002b66: d105         	bne	0x8002b74 <raylar_crcl_encoder_service::benchmark::encode_bits::h865b9e06648afa8c+0x24c> @ imm = #0xa
 8002b68: 4573         	cmp	r3, lr
 8002b6a: f080 80e8    	bhs.w	0x8002d3e <raylar_crcl_encoder_service::benchmark::encode_bits::h865b9e06648afa8c+0x416> @ imm = #0x1d0
 8002b6e: 2100         	movs	r1, #0x0
 8002b70: f809 1003    	strb.w	r1, [r9, r3]
 8002b74: 4573         	cmp	r3, lr
 8002b76: f080 80da    	bhs.w	0x8002d2e <raylar_crcl_encoder_service::benchmark::encode_bits::h865b9e06648afa8c+0x406> @ imm = #0x1b4
 8002b7a: f819 1003    	ldrb.w	r1, [r9, r3]
 8002b7e: f082 0207    	eor	r2, r2, #0x7
 8002b82: 2401         	movs	r4, #0x1
 8002b84: fa04 f202    	lsl.w	r2, r4, r2
 8002b88: 4311         	orrs	r1, r2
 8002b8a: f809 1003    	strb.w	r1, [r9, r3]
 8002b8e: 990d         	ldr	r1, [sp, #0x34]
 8002b90: 2920         	cmp	r1, #0x20
 8002b92: f200 80a9    	bhi.w	0x8002ce8 <raylar_crcl_encoder_service::benchmark::encode_bits::h865b9e06648afa8c+0x3c0> @ imm = #0x152
 8002b96: f108 0301    	add.w	r3, r8, #0x1
 8002b9a: 185a         	adds	r2, r3, r1
 8002b9c: 4542         	cmp	r2, r8
 8002b9e: f240 80a3    	bls.w	0x8002ce8 <raylar_crcl_encoder_service::benchmark::encode_bits::h865b9e06648afa8c+0x3c0> @ imm = #0x146
 8002ba2: 9c0f         	ldr	r4, [sp, #0x3c]
 8002ba4: 42a2         	cmp	r2, r4
 8002ba6: f200 809f    	bhi.w	0x8002ce8 <raylar_crcl_encoder_service::benchmark::encode_bits::h865b9e06648afa8c+0x3c0> @ imm = #0x13e
 8002baa: b381         	cbz	r1, 0x8002c0e <raylar_crcl_encoder_service::benchmark::encode_bits::h865b9e06648afa8c+0x2e6> @ imm = #0x60
 8002bac: 9a05         	ldr	r2, [sp, #0x14]
 8002bae: 4698         	mov	r8, r3
 8002bb0: ea00 0a02    	and.w	r10, r0, r2
 8002bb4: 460a         	mov	r2, r1
 8002bb6: f018 0007    	ands	r0, r8, #0x7
 8002bba: ea4f 03d8    	lsr.w	r3, r8, #0x3
 8002bbe: d105         	bne	0x8002bcc <raylar_crcl_encoder_service::benchmark::encode_bits::h865b9e06648afa8c+0x2a4> @ imm = #0xa
 8002bc0: 4573         	cmp	r3, lr
 8002bc2: f080 80bc    	bhs.w	0x8002d3e <raylar_crcl_encoder_service::benchmark::encode_bits::h865b9e06648afa8c+0x416> @ imm = #0x178
 8002bc6: 2100         	movs	r1, #0x0
 8002bc8: f809 1003    	strb.w	r1, [r9, r3]
 8002bcc: 4573         	cmp	r3, lr
 8002bce: f080 80ae    	bhs.w	0x8002d2e <raylar_crcl_encoder_service::benchmark::encode_bits::h865b9e06648afa8c+0x406> @ imm = #0x15c
 8002bd2: f1c0 0008    	rsb.w	r0, r0, #0x8
 8002bd6: 4611         	mov	r1, r2
 8002bd8: 4290         	cmp	r0, r2
 8002bda: bf38         	it	lo
 8002bdc: 4601         	movlo	r1, r0
 8002bde: 1a52         	subs	r2, r2, r1
 8002be0: eba0 0001    	sub.w	r0, r0, r1
 8002be4: f002 051f    	and	r5, r2, #0x1f
 8002be8: fa0c f401    	lsl.w	r4, r12, r1
 8002bec: f819 6003    	ldrb.w	r6, [r9, r3]
 8002bf0: f000 0007    	and	r0, r0, #0x7
 8002bf4: fa2a f505    	lsr.w	r5, r10, r5
 8002bf8: ea25 0404    	bic.w	r4, r5, r4
 8002bfc: 4488         	add	r8, r1
 8002bfe: fa04 f000    	lsl.w	r0, r4, r0
 8002c02: ea40 0006    	orr.w	r0, r0, r6
 8002c06: f809 0003    	strb.w	r0, [r9, r3]
 8002c0a: d1d4         	bne	0x8002bb6 <raylar_crcl_encoder_service::benchmark::encode_bits::h865b9e06648afa8c+0x28e> @ imm = #-0x58
 8002c0c: e768         	b	0x8002ae0 <raylar_crcl_encoder_service::benchmark::encode_bits::h865b9e06648afa8c+0x1b8> @ imm = #-0x130
 8002c0e: 4698         	mov	r8, r3
 8002c10: e766         	b	0x8002ae0 <raylar_crcl_encoder_service::benchmark::encode_bits::h865b9e06648afa8c+0x1b8> @ imm = #-0x134
 8002c12: f1c8 0000    	rsb.w	r0, r8, #0x0
 8002c16: f000 0107    	and	r1, r0, #0x7
 8002c1a: eb18 0001    	adds.w	r0, r8, r1
 8002c1e: d26f         	bhs	0x8002d00 <raylar_crcl_encoder_service::benchmark::encode_bits::h865b9e06648afa8c+0x3d8> @ imm = #0xde
 8002c20: 9a0f         	ldr	r2, [sp, #0x3c]
 8002c22: 4290         	cmp	r0, r2
 8002c24: d86c         	bhi	0x8002d00 <raylar_crcl_encoder_service::benchmark::encode_bits::h865b9e06648afa8c+0x3d8> @ imm = #0xd8
 8002c26: b1a1         	cbz	r1, 0x8002c52 <raylar_crcl_encoder_service::benchmark::encode_bits::h865b9e06648afa8c+0x32a> @ imm = #0x28
 8002c28: f018 0007    	ands	r0, r8, #0x7
 8002c2c: ea4f 03d8    	lsr.w	r3, r8, #0x3
 8002c30: d105         	bne	0x8002c3e <raylar_crcl_encoder_service::benchmark::encode_bits::h865b9e06648afa8c+0x316> @ imm = #0xa
 8002c32: 4573         	cmp	r3, lr
 8002c34: f080 8083    	bhs.w	0x8002d3e <raylar_crcl_encoder_service::benchmark::encode_bits::h865b9e06648afa8c+0x416> @ imm = #0x106
 8002c38: 2200         	movs	r2, #0x0
 8002c3a: f809 2003    	strb.w	r2, [r9, r3]
 8002c3e: 4573         	cmp	r3, lr
 8002c40: d275         	bhs	0x8002d2e <raylar_crcl_encoder_service::benchmark::encode_bits::h865b9e06648afa8c+0x406> @ imm = #0xea
 8002c42: f1c0 0008    	rsb.w	r0, r0, #0x8
 8002c46: 4288         	cmp	r0, r1
 8002c48: bf28         	it	hs
 8002c4a: 4608         	movhs	r0, r1
 8002c4c: 4480         	add	r8, r0
 8002c4e: 1a09         	subs	r1, r1, r0
 8002c50: d1ea         	bne	0x8002c28 <raylar_crcl_encoder_service::benchmark::encode_bits::h865b9e06648afa8c+0x300> @ imm = #-0x2c
 8002c52: 9904         	ldr	r1, [sp, #0x10]
 8002c54: f641 6658    	movw	r6, #0x1e58
 8002c58: f8dd a00c    	ldr.w	r10, [sp, #0xc]
 8002c5c: f641 6556    	movw	r5, #0x1e56
 8002c60: 9a0b         	ldr	r2, [sp, #0x2c]
 8002c62: f501 33e0    	add.w	r3, r1, #0x1c000
 8002c66: 9812         	ldr	r0, [sp, #0x48]
 8002c68: f1ba 0f05    	cmp.w	r10, #0x5
 8002c6c: eb00 00d8    	add.w	r0, r0, r8, lsr #3
 8002c70: 9012         	str	r0, [sp, #0x48]
 8002c72: f47f ae74    	bne.w	0x800295e <raylar_crcl_encoder_service::benchmark::encode_bits::h865b9e06648afa8c+0x36> @ imm = #-0x318
 8002c76: 9812         	ldr	r0, [sp, #0x48]
 8002c78: f641 6159    	movw	r1, #0x1e59
 8002c7c: 4288         	cmp	r0, r1
 8002c7e: d266         	bhs	0x8002d4e <raylar_crcl_encoder_service::benchmark::encode_bits::h865b9e06648afa8c+0x426> @ imm = #0xcc
 8002c80: 9812         	ldr	r0, [sp, #0x48]
 8002c82: a915         	add	r1, sp, #0x54
 8002c84: e9cd 3015    	strd	r3, r0, [sp, #84]
 8002c88: b017         	add	sp, #0x5c
 8002c8a: e8bd 0f00    	pop.w	{r8, r9, r10, r11}
 8002c8e: bdf0         	pop	{r4, r5, r6, r7, pc}
 8002c90: 2002         	movs	r0, #0x2
 8002c92: f24e 539c    	movw	r3, #0xe59c
 8002c96: f807 0c29    	strb	r0, [r7, #-41]
 8002c9a: f24e 50ec    	movw	r0, #0xe5ec
 8002c9e: f6c0 0000    	movt	r0, #0x800
 8002ca2: f1a7 0229    	sub.w	r2, r7, #0x29
 8002ca6: e015         	b	0x8002cd4 <raylar_crcl_encoder_service::benchmark::encode_bits::h865b9e06648afa8c+0x3ac> @ imm = #0x2a
 8002ca8: 2002         	movs	r0, #0x2
 8002caa: f24e 539c    	movw	r3, #0xe59c
 8002cae: f807 0c27    	strb	r0, [r7, #-39]
 8002cb2: f24e 50cc    	movw	r0, #0xe5cc
 8002cb6: f6c0 0000    	movt	r0, #0x800
 8002cba: f1a7 0227    	sub.w	r2, r7, #0x27
 8002cbe: e009         	b	0x8002cd4 <raylar_crcl_encoder_service::benchmark::encode_bits::h865b9e06648afa8c+0x3ac> @ imm = #0x12
 8002cc0: 2002         	movs	r0, #0x2
 8002cc2: f24e 539c    	movw	r3, #0xe59c
 8002cc6: f88d 0050    	strb.w	r0, [sp, #0x50]
 8002cca: f24e 50dc    	movw	r0, #0xe5dc
 8002cce: f6c0 0000    	movt	r0, #0x800
 8002cd2: aa14         	add	r2, sp, #0x50
 8002cd4: 9000         	str	r0, [sp]
 8002cd6: f24e 5000    	movw	r0, #0xe500
 8002cda: f6c0 0000    	movt	r0, #0x800
 8002cde: f6c0 0300    	movt	r3, #0x800
 8002ce2: 212b         	movs	r1, #0x2b
 8002ce4: f009 f8e0    	bl	0x800bea8 <core::result::unwrap_failed> @ imm = #0x91c0
 8002ce8: 2002         	movs	r0, #0x2
 8002cea: f24e 539c    	movw	r3, #0xe59c
 8002cee: f807 0c26    	strb	r0, [r7, #-38]
 8002cf2: f24e 50bc    	movw	r0, #0xe5bc
 8002cf6: f6c0 0000    	movt	r0, #0x800
 8002cfa: f1a7 0226    	sub.w	r2, r7, #0x26
 8002cfe: e7e9         	b	0x8002cd4 <raylar_crcl_encoder_service::benchmark::encode_bits::h865b9e06648afa8c+0x3ac> @ imm = #-0x2e
 8002d00: 2002         	movs	r0, #0x2
 8002d02: f24e 539c    	movw	r3, #0xe59c
 8002d06: f807 0c25    	strb	r0, [r7, #-37]
 8002d0a: f24e 50ac    	movw	r0, #0xe5ac
 8002d0e: f6c0 0000    	movt	r0, #0x800
 8002d12: f1a7 0225    	sub.w	r2, r7, #0x25
 8002d16: e7dd         	b	0x8002cd4 <raylar_crcl_encoder_service::benchmark::encode_bits::h865b9e06648afa8c+0x3ac> @ imm = #-0x46
 8002d18: 9812         	ldr	r0, [sp, #0x48]
 8002d1a: f24e 53fc    	movw	r3, #0xe5fc
 8002d1e: f6c0 0300    	movt	r3, #0x800
 8002d22: f641 6158    	movw	r1, #0x1e58
 8002d26: f641 6258    	movw	r2, #0x1e58
 8002d2a: f009 fbc8    	bl	0x800c4be <core::slice::index::slice_index_fail> @ imm = #0x9790
 8002d2e: f24e 0200    	movw	r2, #0xe000
 8002d32: f6c0 0200    	movt	r2, #0x800
 8002d36: 4618         	mov	r0, r3
 8002d38: 4671         	mov	r1, lr
 8002d3a: f009 f8d3    	bl	0x800bee4 <core::panicking::panic_bounds_check> @ imm = #0x91a6
 8002d3e: f64d 72f0    	movw	r2, #0xdff0
 8002d42: f6c0 0200    	movt	r2, #0x800
 8002d46: 4618         	mov	r0, r3
 8002d48: 4671         	mov	r1, lr
 8002d4a: f009 f8cb    	bl	0x800bee4 <core::panicking::panic_bounds_check> @ imm = #0x9196
 8002d4e: 9912         	ldr	r1, [sp, #0x48]
 8002d50: f24e 538c    	movw	r3, #0xe58c
 8002d54: f6c0 0300    	movt	r3, #0x800
 8002d58: 2000         	movs	r0, #0x0
 8002d5a: f641 6258    	movw	r2, #0x1e58
 8002d5e: f009 fbae    	bl	0x800c4be <core::slice::index::slice_index_fail> @ imm = #0x975c

08002d62 <raylar_crcl_encoder_service::benchmark::pack_header::haa3cc0f32481b1e3>:
 8002d62: b5f0         	push	{r4, r5, r6, r7, lr}
 8002d64: af03         	add	r7, sp, #0xc
 8002d66: f84d 8d04    	str	r8, [sp, #-4]!
 8002d6a: b082         	sub	sp, #0x8
 8002d6c: 4604         	mov	r4, r0
 8002d6e: f500 30e0    	add.w	r0, r0, #0x1c000
 8002d72: 3004         	adds	r0, #0x4
 8002d74: 214c         	movs	r1, #0x4c
 8002d76: f00a fafc    	bl	0x800d372 <__aeabi_memclr4> @ imm = #0xa5f8
 8002d7a: f245 2043    	movw	r0, #0x5243
 8002d7e: f44f 31e0    	mov.w	r1, #0x1c000
 8002d82: f6c4 3042    	movt	r0, #0x4b42
 8002d86: 2201         	movs	r2, #0x1
 8002d88: 5060         	str	r0, [r4, r1]
 8002d8a: f64f 41c4    	movw	r1, #0xfcc4
 8002d8e: f504 30e0    	add.w	r0, r4, #0x1c000
 8002d92: f2c0 0101    	movt	r1, #0x1
 8002d96: f44f 6380    	mov.w	r3, #0x400
 8002d9a: f854 c001    	ldr.w	r12, [r4, r1]
 8002d9e: 4421         	add	r1, r4
 8002da0: 7102         	strb	r2, [r0, #0x4]
 8002da2: f240 2202    	movw	r2, #0x202
 8002da6: 8303         	strh	r3, [r0, #0x18]
 8002da8: 7b23         	ldrb	r3, [r4, #0xc]
 8002daa: f240 2503    	movw	r5, #0x203
 8002dae: 8382         	strh	r2, [r0, #0x1c]
 8002db0: 2205         	movs	r2, #0x5
 8002db2: f880 302b    	strb.w	r3, [r0, #0x2b]
 8002db6: f44f 7301    	mov.w	r3, #0x204
 8002dba: 7682         	strb	r2, [r0, #0x1a]
 8002dbc: f04f 0858    	mov.w	r8, #0x58
 8002dc0: 694a         	ldr	r2, [r1, #0x14]
 8002dc2: 8683         	strh	r3, [r0, #0x34]
 8002dc4: f240 2305    	movw	r3, #0x205
 8002dc8: f8d4 e000    	ldr.w	lr, [r4]
 8002dcc: 8505         	strh	r5, [r0, #0x28]
 8002dce: 68a5         	ldr	r5, [r4, #0x8]
 8002dd0: 6082         	str	r2, [r0, #0x8]
 8002dd2: 7922         	ldrb	r2, [r4, #0x4]
 8002dd4: f8a0 3040    	strh.w	r3, [r0, #0x40]
 8002dd8: f240 2306    	movw	r3, #0x206
 8002ddc: e9c0 ec08    	strd	lr, r12, [r0, #32]
 8002de0: f8a0 304c    	strh.w	r3, [r0, #0x4c]
 8002de4: 7d23         	ldrb	r3, [r4, #0x14]
 8002de6: 77c2         	strb	r2, [r0, #0x1f]
 8002de8: 62c5         	str	r5, [r0, #0x2c]
 8002dea: e9d1 5201    	ldrd	r5, r2, [r1, #4]
 8002dee: f8d1 c00c    	ldr.w	r12, [r1, #0xc]
 8002df2: 6909         	ldr	r1, [r1, #0x10]
 8002df4: 6305         	str	r5, [r0, #0x30]
 8002df6: 6925         	ldr	r5, [r4, #0x10]
 8002df8: f880 3037    	strb.w	r3, [r0, #0x37]
 8002dfc: 7f23         	ldrb	r3, [r4, #0x1c]
 8002dfe: 69a6         	ldr	r6, [r4, #0x18]
 8002e00: e9c0 520e    	strd	r5, r2, [r0, #56]
 8002e04: f894 2024    	ldrb.w	r2, [r4, #0x24]
 8002e08: f880 3043    	strb.w	r3, [r0, #0x43]
 8002e0c: 6a23         	ldr	r3, [r4, #0x20]
 8002e0e: f8a0 8006    	strh.w	r8, [r0, #0x6]
 8002e12: e9c0 6c11    	strd	r6, r12, [r0, #68]
 8002e16: f880 204f    	strb.w	r2, [r0, #0x4f]
 8002e1a: e9c0 3114    	strd	r3, r1, [r0, #80]
 8002e1e: e9cd 0800    	strd	r0, r8, [sp]
 8002e22: 4668         	mov	r0, sp
 8002e24: 2058         	movs	r0, #0x58
 8002e26: b002         	add	sp, #0x8
 8002e28: f85d 8b04    	ldr	r8, [sp], #4
 8002e2c: bdf0         	pop	{r4, r5, r6, r7, pc}

08002e2e <raylar_crcl_encoder_service::benchmark::encode_words::h522ea4cb3ed82c82>:
 8002e2e: b5f0         	push	{r4, r5, r6, r7, lr}
 8002e30: af03         	add	r7, sp, #0xc
 8002e32: e92d 0f00    	push.w	{r8, r9, r10, r11}
 8002e36: b091         	sub	sp, #0x44
 8002e38: f500 31b8    	add.w	r1, r0, #0x17000
 8002e3c: f500 3ee0    	add.w	lr, r0, #0x1c000
 8002e40: f101 0614    	add.w	r6, r1, #0x14
 8002e44: f24c 0128    	movw	r1, #0xc028
 8002e48: 1845         	adds	r5, r0, r1
 8002e4a: 9005         	str	r0, [sp, #0x14]
 8002e4c: 2000         	movs	r0, #0x0
 8002e4e: 2258         	movs	r2, #0x58
 8002e50: f641 6458    	movw	r4, #0x1e58
 8002e54: f04f 38ff    	mov.w	r8, #0xffffffff
 8002e58: f04f 0c01    	mov.w	r12, #0x1
 8002e5c: e88d 4060    	stm.w	sp, {r5, r6, lr}
 8002e60: 42a2         	cmp	r2, r4
 8002e62: f200 8147    	bhi.w	0x80030f4 <raylar_crcl_encoder_service::benchmark::encode_words::h522ea4cb3ed82c82+0x2c6> @ imm = #0x28e
 8002e66: 9905         	ldr	r1, [sp, #0x14]
 8002e68: eb0e 0902    	add.w	r9, lr, r2
 8002e6c: 9204         	str	r2, [sp, #0x10]
 8002e6e: 1aa2         	subs	r2, r4, r2
 8002e70: eb01 01c0    	add.w	r1, r1, r0, lsl #3
 8002e74: f04f 0b0a    	mov.w	r11, #0xa
 8002e78: f04f 0a00    	mov.w	r10, #0x0
 8002e7c: 790b         	ldrb	r3, [r1, #0x4]
 8002e7e: 1c41         	adds	r1, r0, #0x1
 8002e80: ebc0 2080    	rsb	r0, r0, r0, lsl #10
 8002e84: 9103         	str	r1, [sp, #0xc]
 8002e86: f855 1021    	ldr.w	r1, [r5, r1, lsl #2]
 8002e8a: eb06 0080    	add.w	r0, r6, r0, lsl #2
 8002e8e: f003 061f    	and	r6, r3, #0x1f
 8002e92: 900d         	str	r0, [sp, #0x34]
 8002e94: 9309         	str	r3, [sp, #0x24]
 8002e96: fa08 f006    	lsl.w	r0, r8, r6
 8002e9a: 058c         	lsls	r4, r1, #0x16
 8002e9c: 43c0         	mvns	r0, r0
 8002e9e: 1c59         	adds	r1, r3, #0x1
 8002ea0: 9006         	str	r0, [sp, #0x18]
 8002ea2: fa0c f006    	lsl.w	r0, r12, r6
 8002ea6: 910b         	str	r1, [sp, #0x2c]
 8002ea8: 2100         	movs	r1, #0x0
 8002eaa: 9008         	str	r0, [sp, #0x20]
 8002eac: 3801         	subs	r0, #0x1
 8002eae: 960c         	str	r6, [sp, #0x30]
 8002eb0: 9007         	str	r0, [sp, #0x1c]
 8002eb2: 980d         	ldr	r0, [sp, #0x34]
 8002eb4: f850 602a    	ldr.w	r6, [r0, r10, lsl #2]
 8002eb8: 980c         	ldr	r0, [sp, #0x30]
 8002eba: fa26 f500    	lsr.w	r5, r6, r0
 8002ebe: 980b         	ldr	r0, [sp, #0x2c]
 8002ec0: 1943         	adds	r3, r0, r5
 8002ec2: 2b21         	cmp	r3, #0x21
 8002ec4: d23c         	bhs	0x8002f40 <raylar_crcl_encoder_service::benchmark::encode_words::h522ea4cb3ed82c82+0x112> @ imm = #0x78
 8002ec6: 2b00         	cmp	r3, #0x0
 8002ec8: f000 80d2    	beq.w	0x8003070 <raylar_crcl_encoder_service::benchmark::encode_words::h522ea4cb3ed82c82+0x242> @ imm = #0x1a4
 8002ecc: 9807         	ldr	r0, [sp, #0x1c]
 8002ece: 4030         	ands	r0, r6
 8002ed0: 9e08         	ldr	r6, [sp, #0x20]
 8002ed2: ea40 0c06    	orr.w	r12, r0, r6
 8002ed6: 4608         	mov	r0, r1
 8002ed8: e002         	b	0x8002ee0 <raylar_crcl_encoder_service::benchmark::encode_words::h522ea4cb3ed82c82+0xb2> @ imm = #0x4
 8002eda: 2b00         	cmp	r3, #0x0
 8002edc: f000 80c8    	beq.w	0x8003070 <raylar_crcl_encoder_service::benchmark::encode_words::h522ea4cb3ed82c82+0x242> @ imm = #0x190
 8002ee0: f1cb 0520    	rsb.w	r5, r11, #0x20
 8002ee4: 429d         	cmp	r5, r3
 8002ee6: bf28         	it	hs
 8002ee8: 461d         	movhs	r5, r3
 8002eea: 2d20         	cmp	r5, #0x20
 8002eec: d102         	bne	0x8002ef4 <raylar_crcl_encoder_service::benchmark::encode_words::h522ea4cb3ed82c82+0xc6> @ imm = #0x4
 8002eee: 3b20         	subs	r3, #0x20
 8002ef0: 4666         	mov	r6, r12
 8002ef2: e00a         	b	0x8002f0a <raylar_crcl_encoder_service::benchmark::encode_words::h522ea4cb3ed82c82+0xdc> @ imm = #0x14
 8002ef4: f005 061f    	and	r6, r5, #0x1f
 8002ef8: 1b5b         	subs	r3, r3, r5
 8002efa: fa08 fe06    	lsl.w	lr, r8, r6
 8002efe: f003 061f    	and	r6, r3, #0x1f
 8002f02: fa2c f606    	lsr.w	r6, r12, r6
 8002f06: ea26 060e    	bic.w	r6, r6, lr
 8002f0a: 44ab         	add	r11, r5
 8002f0c: f1cb 0500    	rsb.w	r5, r11, #0x0
 8002f10: f1bb 0f20    	cmp.w	r11, #0x20
 8002f14: f005 051f    	and	r5, r5, #0x1f
 8002f18: fa06 f605    	lsl.w	r6, r6, r5
 8002f1c: ea44 0406    	orr.w	r4, r4, r6
 8002f20: d1db         	bne	0x8002eda <raylar_crcl_encoder_service::benchmark::encode_words::h522ea4cb3ed82c82+0xac> @ imm = #-0x4a
 8002f22: 1d01         	adds	r1, r0, #0x4
 8002f24: f110 0f05    	cmn.w	r0, #0x5
 8002f28: bf98         	it	ls
 8002f2a: 4291         	cmpls	r1, r2
 8002f2c: f200 80dc    	bhi.w	0x80030e8 <raylar_crcl_encoder_service::benchmark::encode_words::h522ea4cb3ed82c82+0x2ba> @ imm = #0x1b8
 8002f30: ba26         	rev	r6, r4
 8002f32: f849 6000    	str.w	r6, [r9, r0]
 8002f36: 2400         	movs	r4, #0x0
 8002f38: 4608         	mov	r0, r1
 8002f3a: f04f 0b00    	mov.w	r11, #0x0
 8002f3e: e7cc         	b	0x8002eda <raylar_crcl_encoder_service::benchmark::encode_words::h522ea4cb3ed82c82+0xac> @ imm = #-0x68
 8002f40: 960a         	str	r6, [sp, #0x28]
 8002f42: b32d         	cbz	r5, 0x8002f90 <raylar_crcl_encoder_service::benchmark::encode_words::h522ea4cb3ed82c82+0x162> @ imm = #0x4a
 8002f44: 46ae         	mov	lr, r5
 8002f46: 2d20         	cmp	r5, #0x20
 8002f48: bf28         	it	hs
 8002f4a: f04f 0e20    	movhs.w	lr, #0x20
 8002f4e: 460e         	mov	r6, r1
 8002f50: 4670         	mov	r0, lr
 8002f52: e002         	b	0x8002f5a <raylar_crcl_encoder_service::benchmark::encode_words::h522ea4cb3ed82c82+0x12c> @ imm = #0x4
 8002f54: ebb0 000c    	subs.w	r0, r0, r12
 8002f58: d017         	beq	0x8002f8a <raylar_crcl_encoder_service::benchmark::encode_words::h522ea4cb3ed82c82+0x15c> @ imm = #0x2e
 8002f5a: f1cb 0c20    	rsb.w	r12, r11, #0x20
 8002f5e: 4584         	cmp	r12, r0
 8002f60: bf28         	it	hs
 8002f62: 4684         	movhs	r12, r0
 8002f64: 44e3         	add	r11, r12
 8002f66: f1bb 0f20    	cmp.w	r11, #0x20
 8002f6a: d1f3         	bne	0x8002f54 <raylar_crcl_encoder_service::benchmark::encode_words::h522ea4cb3ed82c82+0x126> @ imm = #-0x1a
 8002f6c: 1d31         	adds	r1, r6, #0x4
 8002f6e: f116 0f05    	cmn.w	r6, #0x5
 8002f72: bf98         	it	ls
 8002f74: 4291         	cmpls	r1, r2
 8002f76: f200 80b0    	bhi.w	0x80030da <raylar_crcl_encoder_service::benchmark::encode_words::h522ea4cb3ed82c82+0x2ac> @ imm = #0x160
 8002f7a: ba23         	rev	r3, r4
 8002f7c: f849 3006    	str.w	r3, [r9, r6]
 8002f80: 2400         	movs	r4, #0x0
 8002f82: 460e         	mov	r6, r1
 8002f84: f04f 0b00    	mov.w	r11, #0x0
 8002f88: e7e4         	b	0x8002f54 <raylar_crcl_encoder_service::benchmark::encode_words::h522ea4cb3ed82c82+0x126> @ imm = #-0x38
 8002f8a: ebb5 050e    	subs.w	r5, r5, lr
 8002f8e: d1d9         	bne	0x8002f44 <raylar_crcl_encoder_service::benchmark::encode_words::h522ea4cb3ed82c82+0x116> @ imm = #-0x4e
 8002f90: f8dd e024    	ldr.w	lr, [sp, #0x24]
 8002f94: 460e         	mov	r6, r1
 8002f96: f04f 0c01    	mov.w	r12, #0x1
 8002f9a: e003         	b	0x8002fa4 <raylar_crcl_encoder_service::benchmark::encode_words::h522ea4cb3ed82c82+0x176> @ imm = #0x6
 8002f9c: f1bb 0f20    	cmp.w	r11, #0x20
 8002fa0: 46ab         	mov	r11, r5
 8002fa2: d126         	bne	0x8002ff2 <raylar_crcl_encoder_service::benchmark::encode_words::h522ea4cb3ed82c82+0x1c4> @ imm = #0x4c
 8002fa4: f1ab 0520    	sub.w	r5, r11, #0x20
 8002fa8: f1bb 0020    	subs.w	r0, r11, #0x20
 8002fac: fab5 f585    	clz	r5, r5
 8002fb0: bf18         	it	ne
 8002fb2: 2001         	movne	r0, #0x1
 8002fb4: fa08 f300    	lsl.w	r3, r8, r0
 8002fb8: 096d         	lsrs	r5, r5, #0x5
 8002fba: fa2c f505    	lsr.w	r5, r12, r5
 8002fbe: ea25 0303    	bic.w	r3, r5, r3
 8002fc2: eb0b 0500    	add.w	r5, r11, r0
 8002fc6: 4268         	rsbs	r0, r5, #0
 8002fc8: 2d20         	cmp	r5, #0x20
 8002fca: f000 001f    	and	r0, r0, #0x1f
 8002fce: fa03 f000    	lsl.w	r0, r3, r0
 8002fd2: ea44 0400    	orr.w	r4, r4, r0
 8002fd6: d1e1         	bne	0x8002f9c <raylar_crcl_encoder_service::benchmark::encode_words::h522ea4cb3ed82c82+0x16e> @ imm = #-0x3e
 8002fd8: 1d31         	adds	r1, r6, #0x4
 8002fda: f116 0f05    	cmn.w	r6, #0x5
 8002fde: bf98         	it	ls
 8002fe0: 4291         	cmpls	r1, r2
 8002fe2: d87a         	bhi	0x80030da <raylar_crcl_encoder_service::benchmark::encode_words::h522ea4cb3ed82c82+0x2ac> @ imm = #0xf4
 8002fe4: ba20         	rev	r0, r4
 8002fe6: f849 0006    	str.w	r0, [r9, r6]
 8002fea: 2400         	movs	r4, #0x0
 8002fec: 460e         	mov	r6, r1
 8002fee: 2500         	movs	r5, #0x0
 8002ff0: e7d4         	b	0x8002f9c <raylar_crcl_encoder_service::benchmark::encode_words::h522ea4cb3ed82c82+0x16e> @ imm = #-0x58
 8002ff2: f1be 0f00    	cmp.w	lr, #0x0
 8002ff6: d03a         	beq	0x800306e <raylar_crcl_encoder_service::benchmark::encode_words::h522ea4cb3ed82c82+0x240> @ imm = #0x74
 8002ff8: 9806         	ldr	r0, [sp, #0x18]
 8002ffa: 46ab         	mov	r11, r5
 8002ffc: 9b0a         	ldr	r3, [sp, #0x28]
 8002ffe: ea03 0c00    	and.w	r12, r3, r0
 8003002: 4608         	mov	r0, r1
 8003004: e002         	b	0x800300c <raylar_crcl_encoder_service::benchmark::encode_words::h522ea4cb3ed82c82+0x1de> @ imm = #0x4
 8003006: f1be 0f00    	cmp.w	lr, #0x0
 800300a: d031         	beq	0x8003070 <raylar_crcl_encoder_service::benchmark::encode_words::h522ea4cb3ed82c82+0x242> @ imm = #0x62
 800300c: f1cb 0520    	rsb.w	r5, r11, #0x20
 8003010: 4575         	cmp	r5, lr
 8003012: bf28         	it	hs
 8003014: 4675         	movhs	r5, lr
 8003016: 2d20         	cmp	r5, #0x20
 8003018: d103         	bne	0x8003022 <raylar_crcl_encoder_service::benchmark::encode_words::h522ea4cb3ed82c82+0x1f4> @ imm = #0x6
 800301a: f1ae 0e20    	sub.w	lr, lr, #0x20
 800301e: 4663         	mov	r3, r12
 8003020: e00b         	b	0x800303a <raylar_crcl_encoder_service::benchmark::encode_words::h522ea4cb3ed82c82+0x20c> @ imm = #0x16
 8003022: f005 031f    	and	r3, r5, #0x1f
 8003026: ebae 0e05    	sub.w	lr, lr, r5
 800302a: f00e 061f    	and	r6, lr, #0x1f
 800302e: fa08 f303    	lsl.w	r3, r8, r3
 8003032: fa2c f606    	lsr.w	r6, r12, r6
 8003036: ea26 0303    	bic.w	r3, r6, r3
 800303a: 44ab         	add	r11, r5
 800303c: f1cb 0500    	rsb.w	r5, r11, #0x0
 8003040: f1bb 0f20    	cmp.w	r11, #0x20
 8003044: f005 051f    	and	r5, r5, #0x1f
 8003048: fa03 f305    	lsl.w	r3, r3, r5
 800304c: ea44 0403    	orr.w	r4, r4, r3
 8003050: d1d9         	bne	0x8003006 <raylar_crcl_encoder_service::benchmark::encode_words::h522ea4cb3ed82c82+0x1d8> @ imm = #-0x4e
 8003052: 1d01         	adds	r1, r0, #0x4
 8003054: f110 0f05    	cmn.w	r0, #0x5
 8003058: bf98         	it	ls
 800305a: 4291         	cmpls	r1, r2
 800305c: d844         	bhi	0x80030e8 <raylar_crcl_encoder_service::benchmark::encode_words::h522ea4cb3ed82c82+0x2ba> @ imm = #0x88
 800305e: ba23         	rev	r3, r4
 8003060: f849 3000    	str.w	r3, [r9, r0]
 8003064: 2400         	movs	r4, #0x0
 8003066: 4608         	mov	r0, r1
 8003068: f04f 0b00    	mov.w	r11, #0x0
 800306c: e7cb         	b	0x8003006 <raylar_crcl_encoder_service::benchmark::encode_words::h522ea4cb3ed82c82+0x1d8> @ imm = #-0x6a
 800306e: 46ab         	mov	r11, r5
 8003070: f10a 0a01    	add.w	r10, r10, #0x1
 8003074: 2301         	movs	r3, #0x1
 8003076: f240 30ff    	movw	r0, #0x3ff
 800307a: 4582         	cmp	r10, r0
 800307c: f47f af19    	bne.w	0x8002eb2 <raylar_crcl_encoder_service::benchmark::encode_words::h522ea4cb3ed82c82+0x84> @ imm = #-0x1ce
 8003080: ea4f 05db    	lsr.w	r5, r11, #0x3
 8003084: ea5f 704b    	lsls.w	r0, r11, #0x1d
 8003088: bf18         	it	ne
 800308a: eb03 05db    	addne.w	r5, r3, r11, lsr #3
 800308e: 186e         	adds	r6, r5, r1
 8003090: d23b         	bhs	0x800310a <raylar_crcl_encoder_service::benchmark::encode_words::h522ea4cb3ed82c82+0x2dc> @ imm = #0x76
 8003092: 4296         	cmp	r6, r2
 8003094: d839         	bhi	0x800310a <raylar_crcl_encoder_service::benchmark::encode_words::h522ea4cb3ed82c82+0x2dc> @ imm = #0x72
 8003096: ba20         	rev	r0, r4
 8003098: 2d05         	cmp	r5, #0x5
 800309a: 9010         	str	r0, [sp, #0x40]
 800309c: d23d         	bhs	0x800311a <raylar_crcl_encoder_service::benchmark::encode_words::h522ea4cb3ed82c82+0x2ec> @ imm = #0x7a
 800309e: eb09 0001    	add.w	r0, r9, r1
 80030a2: a910         	add	r1, sp, #0x40
 80030a4: 462a         	mov	r2, r5
 80030a6: 461c         	mov	r4, r3
 80030a8: f009 ff3a    	bl	0x800cf20 <__aeabi_memcpy> @ imm = #0x9e74
 80030ac: 9a04         	ldr	r2, [sp, #0x10]
 80030ae: 46a4         	mov	r12, r4
 80030b0: 9803         	ldr	r0, [sp, #0xc]
 80030b2: f641 6458    	movw	r4, #0x1e58
 80030b6: 4432         	add	r2, r6
 80030b8: e89d 4060    	ldm.w	sp, {r5, r6, lr}
 80030bc: 2805         	cmp	r0, #0x5
 80030be: f47f aecf    	bne.w	0x8002e60 <raylar_crcl_encoder_service::benchmark::encode_words::h522ea4cb3ed82c82+0x32> @ imm = #-0x262
 80030c2: f641 6059    	movw	r0, #0x1e59
 80030c6: 4282         	cmp	r2, r0
 80030c8: d230         	bhs	0x800312c <raylar_crcl_encoder_service::benchmark::encode_words::h522ea4cb3ed82c82+0x2fe> @ imm = #0x60
 80030ca: e9cd e20e    	strd	lr, r2, [sp, #56]
 80030ce: a80e         	add	r0, sp, #0x38
 80030d0: 4610         	mov	r0, r2
 80030d2: b011         	add	sp, #0x44
 80030d4: e8bd 0f00    	pop.w	{r8, r9, r10, r11}
 80030d8: bdf0         	pop	{r4, r5, r6, r7, pc}
 80030da: f24e 73a0    	movw	r3, #0xe7a0
 80030de: 4630         	mov	r0, r6
 80030e0: f6c0 0300    	movt	r3, #0x800
 80030e4: f009 f9eb    	bl	0x800c4be <core::slice::index::slice_index_fail> @ imm = #0x93d6
 80030e8: f24e 73a0    	movw	r3, #0xe7a0
 80030ec: f6c0 0300    	movt	r3, #0x800
 80030f0: f009 f9e5    	bl	0x800c4be <core::slice::index::slice_index_fail> @ imm = #0x93ca
 80030f4: f24e 631c    	movw	r3, #0xe61c
 80030f8: 4610         	mov	r0, r2
 80030fa: f6c0 0300    	movt	r3, #0x800
 80030fe: f641 6158    	movw	r1, #0x1e58
 8003102: f641 6258    	movw	r2, #0x1e58
 8003106: f009 f9da    	bl	0x800c4be <core::slice::index::slice_index_fail> @ imm = #0x93b4
 800310a: f24e 73c0    	movw	r3, #0xe7c0
 800310e: 4608         	mov	r0, r1
 8003110: f6c0 0300    	movt	r3, #0x800
 8003114: 4631         	mov	r1, r6
 8003116: f009 f9d2    	bl	0x800c4be <core::slice::index::slice_index_fail> @ imm = #0x93a4
 800311a: f24e 73b0    	movw	r3, #0xe7b0
 800311e: 2000         	movs	r0, #0x0
 8003120: f6c0 0300    	movt	r3, #0x800
 8003124: 4629         	mov	r1, r5
 8003126: 2204         	movs	r2, #0x4
 8003128: f009 f9c9    	bl	0x800c4be <core::slice::index::slice_index_fail> @ imm = #0x9392
 800312c: f24e 630c    	movw	r3, #0xe60c
 8003130: 4611         	mov	r1, r2
 8003132: f6c0 0300    	movt	r3, #0x800
 8003136: 2000         	movs	r0, #0x0
 8003138: f641 6258    	movw	r2, #0x1e58
 800313c: f009 f9bf    	bl	0x800c4be <core::slice::index::slice_index_fail> @ imm = #0x937e

08003140 <raylar_crcl_encoder_service::benchmark::encode_folded::h7ac7123b2c9324bf>:
 8003140: b580         	push	{r7, lr}
 8003142: 466f         	mov	r7, sp
 8003144: 2100         	movs	r1, #0x0
 8003146: 2200         	movs	r2, #0x0
 8003148: e8bd 4080    	pop.w	{r7, lr}
 800314c: f7ff bbec    	b.w	0x8002928 <raylar_crcl_encoder_service::benchmark::encode_bits::h865b9e06648afa8c> @ imm = #-0x828

08003150 <raylar_crcl_encoder_service::benchmark::encode_payload::hfef902a78aec660e>:
 8003150: b580         	push	{r7, lr}
 8003152: 466f         	mov	r7, sp
 8003154: 2100         	movs	r1, #0x0
 8003156: 2201         	movs	r2, #0x1
 8003158: e8bd 4080    	pop.w	{r7, lr}
 800315c: f7ff bbe4    	b.w	0x8002928 <raylar_crcl_encoder_service::benchmark::encode_bits::h865b9e06648afa8c> @ imm = #-0x838

08003160 <raylar_crcl_encoder_service::benchmark::encode_codeword::h30aff6e5abaf4d69>:
 8003160: b580         	push	{r7, lr}
 8003162: 466f         	mov	r7, sp
 8003164: 2101         	movs	r1, #0x1
 8003166: 2200         	movs	r2, #0x0
 8003168: e8bd 4080    	pop.w	{r7, lr}
 800316c: f7ff bbdc    	b.w	0x8002928 <raylar_crcl_encoder_service::benchmark::encode_bits::h865b9e06648afa8c> @ imm = #-0x848

08003170 <raylar_crcl_encoder_service::benchmark::reference_codec::h861d4cc00b2897f3>:
 8003170: b5f0         	push	{r4, r5, r6, r7, lr}
 8003172: af03         	add	r7, sp, #0xc
 8003174: f84d 8d04    	str	r8, [sp, #-4]!
 8003178: b0ba         	sub	sp, #0xe8
 800317a: f240 660a    	movw	r6, #0x60a
 800317e: 4680         	mov	r8, r0
 8003180: f44f 0000    	mov.w	r0, #0x800000
 8003184: f2c0 1601    	movt	r6, #0x101
 8003188: 9013         	str	r0, [sp, #0x4c]
 800318a: f44f 6080    	mov.w	r0, #0x400
 800318e: 9012         	str	r0, [sp, #0x48]
 8003190: a812         	add	r0, sp, #0x48
 8003192: 9614         	str	r6, [sp, #0x50]
 8003194: 9812         	ldr	r0, [sp, #0x48]
 8003196: 1e41         	subs	r1, r0, #0x1
 8003198: ea80 0201    	eor.w	r2, r0, r1
 800319c: 428a         	cmp	r2, r1
 800319e: d916         	bls	0x80031ce <raylar_crcl_encoder_service::benchmark::reference_codec::h861d4cc00b2897f3+0x5e> @ imm = #0x2c
 80031a0: fa90 f0a0    	rbit	r0, r0
 80031a4: fab0 f080    	clz	r0, r0
 80031a8: 3808         	subs	r0, #0x8
 80031aa: 2804         	cmp	r0, #0x4
 80031ac: bf9c         	itt	ls
 80031ae: f89d 0050    	ldrbls.w	r0, [sp, #0x50]
 80031b2: 280c         	cmpls	r0, #0xc
 80031b4: d80b         	bhi	0x80031ce <raylar_crcl_encoder_service::benchmark::reference_codec::h861d4cc00b2897f3+0x5e> @ imm = #0x16
 80031b6: 2101         	movs	r1, #0x1
 80031b8: fa01 f000    	lsl.w	r0, r1, r0
 80031bc: f410 5faa    	tst.w	r0, #0x1540
 80031c0: bf1e         	ittt	ne
 80031c2: 9813         	ldrne	r0, [sp, #0x4c]
 80031c4: f100 4040    	addne.w	r0, r0, #0xc0000000
 80031c8: f1b0 4f40    	cmpne.w	r0, #0xc0000000
 80031cc: d813         	bhi	0x80031f6 <raylar_crcl_encoder_service::benchmark::reference_codec::h861d4cc00b2897f3+0x86> @ imm = #0x26
 80031ce: 2000         	movs	r0, #0x0
 80031d0: f24e 539c    	movw	r3, #0xe59c
 80031d4: f88d 0008    	strb.w	r0, [sp, #0x8]
 80031d8: f24e 606c    	movw	r0, #0xe66c
 80031dc: f6c0 0000    	movt	r0, #0x800
 80031e0: aa02         	add	r2, sp, #0x8
 80031e2: 9000         	str	r0, [sp]
 80031e4: f24e 5000    	movw	r0, #0xe500
 80031e8: f6c0 0000    	movt	r0, #0x800
 80031ec: f6c0 0300    	movt	r3, #0x800
 80031f0: 212b         	movs	r1, #0x2b
 80031f2: f008 fe59    	bl	0x800bea8 <core::result::unwrap_failed> @ imm = #0x8cb2
 80031f6: f89d 0051    	ldrb.w	r0, [sp, #0x51]
 80031fa: 281e         	cmp	r0, #0x1e
 80031fc: d8e7         	bhi	0x80031ce <raylar_crcl_encoder_service::benchmark::reference_codec::h861d4cc00b2897f3+0x5e> @ imm = #-0x32
 80031fe: f89d 4053    	ldrb.w	r4, [sp, #0x53]
 8003202: 2c01         	cmp	r4, #0x1
 8003204: d104         	bne	0x8003210 <raylar_crcl_encoder_service::benchmark::reference_codec::h861d4cc00b2897f3+0xa0> @ imm = #0x8
 8003206: f89d 0052    	ldrb.w	r0, [sp, #0x52]
 800320a: 2801         	cmp	r0, #0x1
 800320c: d002         	beq	0x8003214 <raylar_crcl_encoder_service::benchmark::reference_codec::h861d4cc00b2897f3+0xa4> @ imm = #0x4
 800320e: e7de         	b	0x80031ce <raylar_crcl_encoder_service::benchmark::reference_codec::h861d4cc00b2897f3+0x5e> @ imm = #-0x44
 8003210: 2800         	cmp	r0, #0x0
 8003212: d1dc         	bne	0x80031ce <raylar_crcl_encoder_service::benchmark::reference_codec::h861d4cc00b2897f3+0x5e> @ imm = #-0x48
 8003214: e9dd 0112    	ldrd	r0, r1, [sp, #72]
 8003218: 2500         	movs	r5, #0x0
 800321a: f8dd 204f    	ldr.w	r2, [sp, #0x4f]
 800321e: f8cd 003b    	str.w	r0, [sp, #0x3b]
 8003222: a802         	add	r0, sp, #0x8
 8003224: f8cd 2042    	str.w	r2, [sp, #0x42]
 8003228: 3001         	adds	r0, #0x1
 800322a: f8cd 103f    	str.w	r1, [sp, #0x3f]
 800322e: a90b         	add	r1, sp, #0x2c
 8003230: 221a         	movs	r2, #0x1a
 8003232: 950d         	str	r5, [sp, #0x34]
 8003234: e9cd 550b    	strd	r5, r5, [sp, #44]
 8003238: f8cd 5037    	str.w	r5, [sp, #0x37]
 800323c: f88d 5008    	strb.w	r5, [sp, #0x8]
 8003240: f009 fe6e    	bl	0x800cf20 <__aeabi_memcpy> @ imm = #0x9cdc
 8003244: f44f 0000    	mov.w	r0, #0x800000
 8003248: f88d 4023    	strb.w	r4, [sp, #0x23]
 800324c: 900c         	str	r0, [sp, #0x30]
 800324e: f44f 6080    	mov.w	r0, #0x400
 8003252: 900b         	str	r0, [sp, #0x2c]
 8003254: a80b         	add	r0, sp, #0x2c
 8003256: 960d         	str	r6, [sp, #0x34]
 8003258: 980b         	ldr	r0, [sp, #0x2c]
 800325a: 1e41         	subs	r1, r0, #0x1
 800325c: ea80 0201    	eor.w	r2, r0, r1
 8003260: 428a         	cmp	r2, r1
 8003262: d90a         	bls	0x800327a <raylar_crcl_encoder_service::benchmark::reference_codec::h861d4cc00b2897f3+0x10a> @ imm = #0x14
 8003264: fa90 f1a0    	rbit	r1, r0
 8003268: fab1 f181    	clz	r1, r1
 800326c: 3908         	subs	r1, #0x8
 800326e: 2904         	cmp	r1, #0x4
 8003270: bf9c         	itt	ls
 8003272: f89d 1034    	ldrbls.w	r1, [sp, #0x34]
 8003276: 290c         	cmpls	r1, #0xc
 8003278: d912         	bls	0x80032a0 <raylar_crcl_encoder_service::benchmark::reference_codec::h861d4cc00b2897f3+0x130> @ imm = #0x24
 800327a: f24e 605c    	movw	r0, #0xe65c
 800327e: f88d 502c    	strb.w	r5, [sp, #0x2c]
 8003282: f6c0 0000    	movt	r0, #0x800
 8003286: f24e 539c    	movw	r3, #0xe59c
 800328a: 9000         	str	r0, [sp]
 800328c: f24e 5000    	movw	r0, #0xe500
 8003290: aa0b         	add	r2, sp, #0x2c
 8003292: f6c0 0000    	movt	r0, #0x800
 8003296: f6c0 0300    	movt	r3, #0x800
 800329a: 212b         	movs	r1, #0x2b
 800329c: f008 fe04    	bl	0x800bea8 <core::result::unwrap_failed> @ imm = #0x8c08
 80032a0: 2201         	movs	r2, #0x1
 80032a2: fa02 f101    	lsl.w	r1, r2, r1
 80032a6: f411 5faa    	tst.w	r1, #0x1540
 80032aa: d0e6         	beq	0x800327a <raylar_crcl_encoder_service::benchmark::reference_codec::h861d4cc00b2897f3+0x10a> @ imm = #-0x34
 80032ac: 9a0c         	ldr	r2, [sp, #0x30]
 80032ae: 2101         	movs	r1, #0x1
 80032b0: f2cc 0100    	movt	r1, #0xc000
 80032b4: f102 4240    	add.w	r2, r2, #0xc0000000
 80032b8: 428a         	cmp	r2, r1
 80032ba: d3de         	blo	0x800327a <raylar_crcl_encoder_service::benchmark::reference_codec::h861d4cc00b2897f3+0x10a> @ imm = #-0x44
 80032bc: f89d 1035    	ldrb.w	r1, [sp, #0x35]
 80032c0: 291e         	cmp	r1, #0x1e
 80032c2: d8da         	bhi	0x800327a <raylar_crcl_encoder_service::benchmark::reference_codec::h861d4cc00b2897f3+0x10a> @ imm = #-0x4c
 80032c4: f89d 2037    	ldrb.w	r2, [sp, #0x37]
 80032c8: 2a01         	cmp	r2, #0x1
 80032ca: d107         	bne	0x80032dc <raylar_crcl_encoder_service::benchmark::reference_codec::h861d4cc00b2897f3+0x16c> @ imm = #0xe
 80032cc: f5b0 6f80    	cmp.w	r0, #0x400
 80032d0: d3d3         	blo	0x800327a <raylar_crcl_encoder_service::benchmark::reference_codec::h861d4cc00b2897f3+0x10a> @ imm = #-0x5a
 80032d2: f89d 0036    	ldrb.w	r0, [sp, #0x36]
 80032d6: 2801         	cmp	r0, #0x1
 80032d8: d005         	beq	0x80032e6 <raylar_crcl_encoder_service::benchmark::reference_codec::h861d4cc00b2897f3+0x176> @ imm = #0xa
 80032da: e7ce         	b	0x800327a <raylar_crcl_encoder_service::benchmark::reference_codec::h861d4cc00b2897f3+0x10a> @ imm = #-0x64
 80032dc: f5b0 6f80    	cmp.w	r0, #0x400
 80032e0: d3cb         	blo	0x800327a <raylar_crcl_encoder_service::benchmark::reference_codec::h861d4cc00b2897f3+0x10a> @ imm = #-0x6a
 80032e2: 2900         	cmp	r1, #0x0
 80032e4: d1c9         	bne	0x800327a <raylar_crcl_encoder_service::benchmark::reference_codec::h861d4cc00b2897f3+0x10a> @ imm = #-0x6e
 80032e6: f246 0028    	movw	r0, #0x6028
 80032ea: ad15         	add	r5, sp, #0x54
 80032ec: eb08 0400    	add.w	r4, r8, r0
 80032f0: f105 0048    	add.w	r0, r5, #0x48
 80032f4: 2128         	movs	r1, #0x28
 80032f6: f00a f83c    	bl	0x800d372 <__aeabi_memclr4> @ imm = #0xa078
 80032fa: ab0b         	add	r3, sp, #0x2c
 80032fc: ae0b         	add	r6, sp, #0x2c
 80032fe: 2000         	movs	r0, #0x0
 8003300: 9425         	str	r4, [sp, #0x94]
 8003302: cb0e         	ldm	r3, {r1, r2, r3}
 8003304: f44f 7480    	mov.w	r4, #0x100
 8003308: 9338         	str	r3, [sp, #0xe0]
 800330a: f44f 53c0    	mov.w	r3, #0x1800
 800330e: e9cd 1236    	strd	r1, r2, [sp, #216]
 8003312: 2101         	movs	r1, #0x1
 8003314: f8ad 0070    	strh.w	r0, [sp, #0x70]
 8003318: e9cd 001a    	strd	r0, r0, [sp, #104]
 800331c: e9cd 0018    	strd	r0, r0, [sp, #96]
 8003320: e9cd 0016    	strd	r0, r0, [sp, #88]
 8003324: e9cd 001e    	strd	r0, r0, [sp, #120]
 8003328: e9cd 0020    	strd	r0, r0, [sp, #128]
 800332c: e9cd 0022    	strd	r0, r0, [sp, #136]
 8003330: f8ad 0090    	strh.w	r0, [sp, #0x90]
 8003334: e9cd 0032    	strd	r0, r0, [sp, #200]
 8003338: e9cd 0034    	strd	r0, r0, [sp, #208]
 800333c: 9015         	str	r0, [sp, #0x54]
 800333e: 901d         	str	r0, [sp, #0x74]
 8003340: 9326         	str	r3, [sp, #0x98]
 8003342: 9131         	str	r1, [sp, #0xc4]
 8003344: f8ad 00e4    	strh.w	r0, [sp, #0xe4]
 8003348: 940b         	str	r4, [sp, #0x2c]
 800334a: 4628         	mov	r0, r5
 800334c: 990b         	ldr	r1, [sp, #0x2c]
 800334e: f7fd ff8c    	bl	0x800126a <raylar_crcl_encoder_service::cheap::reference::_$LT$impl$u20$raylar_crcl_encoder_service..quantize..Transform$GT$::advance_cheap_reference::hc3bf4874397b54bd> @ imm = #-0x20e8
 8003352: 07c0         	lsls	r0, r0, #0x1f
 8003354: d11d         	bne	0x8003392 <raylar_crcl_encoder_service::benchmark::reference_codec::h861d4cc00b2897f3+0x222> @ imm = #0x3a
 8003356: 07c8         	lsls	r0, r1, #0x1f
 8003358: d0f6         	beq	0x8003348 <raylar_crcl_encoder_service::benchmark::reference_codec::h861d4cc00b2897f3+0x1d8> @ imm = #-0x14
 800335a: f641 6058    	movw	r0, #0x1e58
 800335e: f508 34e0    	add.w	r4, r8, #0x1c000
 8003362: 9000         	str	r0, [sp]
 8003364: a80b         	add	r0, sp, #0x2c
 8003366: a902         	add	r1, sp, #0x8
 8003368: aa15         	add	r2, sp, #0x54
 800336a: 4623         	mov	r3, r4
 800336c: f7fe f988    	bl	0x8001680 <raylar_crcl_encoder_service::format::Encoder::data::ha65fe528befbf4e3> @ imm = #-0x1cf0
 8003370: f89d 002c    	ldrb.w	r0, [sp, #0x2c]
 8003374: 2801         	cmp	r0, #0x1
 8003376: d01c         	beq	0x80033b2 <raylar_crcl_encoder_service::benchmark::reference_codec::h861d4cc00b2897f3+0x242> @ imm = #0x38
 8003378: 990c         	ldr	r1, [sp, #0x30]
 800337a: f641 6059    	movw	r0, #0x1e59
 800337e: 4281         	cmp	r1, r0
 8003380: d20e         	bhs	0x80033a0 <raylar_crcl_encoder_service::benchmark::reference_codec::h861d4cc00b2897f3+0x230> @ imm = #0x1c
 8003382: e9cd 410b    	strd	r4, r1, [sp, #44]
 8003386: a80b         	add	r0, sp, #0x2c
 8003388: 4608         	mov	r0, r1
 800338a: b03a         	add	sp, #0xe8
 800338c: f85d 8b04    	ldr	r8, [sp], #4
 8003390: bdf0         	pop	{r4, r5, r6, r7, pc}
 8003392: f24e 604c    	movw	r0, #0xe64c
 8003396: f88d 102c    	strb.w	r1, [sp, #0x2c]
 800339a: f6c0 0000    	movt	r0, #0x800
 800339e: e772         	b	0x8003286 <raylar_crcl_encoder_service::benchmark::reference_codec::h861d4cc00b2897f3+0x116> @ imm = #-0x11c
 80033a0: f24e 632c    	movw	r3, #0xe62c
 80033a4: 2000         	movs	r0, #0x0
 80033a6: f6c0 0300    	movt	r3, #0x800
 80033aa: f641 6258    	movw	r2, #0x1e58
 80033ae: f009 f886    	bl	0x800c4be <core::slice::index::slice_index_fail> @ imm = #0x910c
 80033b2: f89d 002d    	ldrb.w	r0, [sp, #0x2d]
 80033b6: f24e 539c    	movw	r3, #0xe59c
 80033ba: f88d 0048    	strb.w	r0, [sp, #0x48]
 80033be: f24e 603c    	movw	r0, #0xe63c
 80033c2: f6c0 0000    	movt	r0, #0x800
 80033c6: aa12         	add	r2, sp, #0x48
 80033c8: e70b         	b	0x80031e2 <raylar_crcl_encoder_service::benchmark::reference_codec::h861d4cc00b2897f3+0x72> @ imm = #-0x1ea

080033ca <raylar_crcl_encoder_service::benchmark::split_transform::h9238bb9046f68a6e>:
 80033ca: b5f0         	push	{r4, r5, r6, r7, lr}
 80033cc: af03         	add	r7, sp, #0xc
 80033ce: e92d 0f00    	push.w	{r8, r9, r10, r11}
 80033d2: f2ad 5d04    	subw	sp, sp, #0x504
 80033d6: 9002         	str	r0, [sp, #0x8]
 80033d8: a817         	add	r0, sp, #0x5c
 80033da: 460e         	mov	r6, r1
 80033dc: f44f 7101    	mov.w	r1, #0x204
 80033e0: 4614         	mov	r4, r2
 80033e2: f009 ffc6    	bl	0x800d372 <__aeabi_memclr4> @ imm = #0x9f8c
 80033e6: a898         	add	r0, sp, #0x260
 80033e8: f44f 7101    	mov.w	r1, #0x204
 80033ec: f009 ffc1    	bl	0x800d372 <__aeabi_memclr4> @ imm = #0x9f82
 80033f0: f24c 0028    	movw	r0, #0xc028
 80033f4: 1833         	adds	r3, r6, r0
 80033f6: f246 0028    	movw	r0, #0x6028
 80033fa: f1a7 0bbc    	sub.w	r11, r7, #0xbc
 80033fe: 4430         	add	r0, r6
 8003400: 9009         	str	r0, [sp, #0x24]
 8003402: f10b 0084    	add.w	r0, r11, #0x84
 8003406: 9008         	str	r0, [sp, #0x20]
 8003408: f10b 0074    	add.w	r0, r11, #0x74
 800340c: 9007         	str	r0, [sp, #0x1c]
 800340e: f10b 0048    	add.w	r0, r11, #0x48
 8003412: 9006         	str	r0, [sp, #0x18]
 8003414: f10b 0020    	add.w	r0, r11, #0x20
 8003418: f1a7 0924    	sub.w	r9, r7, #0x24
 800341c: 9005         	str	r0, [sp, #0x14]
 800341e: f106 0028    	add.w	r0, r6, #0x28
 8003422: f06f 0607    	mvn	r6, #0x7
 8003426: f44f 6580    	mov.w	r5, #0x400
 800342a: 9004         	str	r0, [sp, #0x10]
 800342c: 9303         	str	r3, [sp, #0xc]
 800342e: e008         	b	0x8003442 <raylar_crcl_encoder_service::benchmark::split_transform::h9238bb9046f68a6e+0x78> @ imm = #0x10
 8003430: 3601         	adds	r6, #0x1
 8003432: 46d9         	mov	r9, r11
 8003434: f1a7 0bbc    	sub.w	r11, r7, #0xbc
 8003438: f44f 6580    	mov.w	r5, #0x400
 800343c: 2e81         	cmp	r6, #0x81
 800343e: f000 8145    	beq.w	0x80036cc <raylar_crcl_encoder_service::benchmark::split_transform::h9238bb9046f68a6e+0x302> @ imm = #0x28a
 8003442: 9616         	str	r6, [sp, #0x58]
 8003444: f44f 42c0    	mov.w	r2, #0x6000
 8003448: 9e09         	ldr	r6, [sp, #0x24]
 800344a: 9904         	ldr	r1, [sp, #0x10]
 800344c: 4630         	mov	r0, r6
 800344e: f009 ff42    	bl	0x800d2d6 <__aeabi_memcpy4> @ imm = #0x9e84
 8003452: 9806         	ldr	r0, [sp, #0x18]
 8003454: 2128         	movs	r1, #0x28
 8003456: f009 ff8c    	bl	0x800d372 <__aeabi_memclr4> @ imm = #0x9f18
 800345a: 9807         	ldr	r0, [sp, #0x1c]
 800345c: 2100         	movs	r1, #0x0
 800345e: f44f 0200    	mov.w	r2, #0x800000
 8003462: e947 112f    	strd	r1, r1, [r7, #-188]
 8003466: e947 112d    	strd	r1, r1, [r7, #-180]
 800346a: e9c0 1100    	strd	r1, r1, [r0]
 800346e: e9c0 1102    	strd	r1, r1, [r0, #8]
 8003472: 9805         	ldr	r0, [sp, #0x14]
 8003474: e947 112b    	strd	r1, r1, [r7, #-172]
 8003478: f847 1ca4    	str	r1, [r7, #-164]
 800347c: e9c0 1100    	strd	r1, r1, [r0]
 8003480: e9c0 1102    	strd	r1, r1, [r0, #8]
 8003484: e9c0 1104    	strd	r1, r1, [r0, #16]
 8003488: 6181         	str	r1, [r0, #0x18]
 800348a: 8381         	strh	r1, [r0, #0x1c]
 800348c: 9808         	ldr	r0, [sp, #0x20]
 800348e: f827 1ca0    	strh	r1, [r7, #-160]
 8003492: f827 1c2c    	strh	r1, [r7, #-44]
 8003496: 6042         	str	r2, [r0, #0x4]
 8003498: f240 620a    	movw	r2, #0x60a
 800349c: f2c0 1201    	movt	r2, #0x101
 80034a0: 6005         	str	r5, [r0]
 80034a2: 6082         	str	r2, [r0, #0x8]
 80034a4: 2001         	movs	r0, #0x1
 80034a6: f847 0c4c    	str	r0, [r7, #-76]
 80034aa: f44f 50c0    	mov.w	r0, #0x1800
 80034ae: e947 601f    	strd	r6, r0, [r7, #-124]
 80034b2: 47a0         	blx	r4
 80034b4: f847 5c24    	str	r5, [r7, #-36]
 80034b8: 4682         	mov	r10, r0
 80034ba: 4658         	mov	r0, r11
 80034bc: f857 1c24    	ldr	r1, [r7, #-36]
 80034c0: f7fe fb95    	bl	0x8001bee <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3> @ imm = #-0x18d6
 80034c4: 07c0         	lsls	r0, r0, #0x1f
 80034c6: f040 80d7    	bne.w	0x8003678 <raylar_crcl_encoder_service::benchmark::split_transform::h9238bb9046f68a6e+0x2ae> @ imm = #0x1ae
 80034ca: f001 0001    	and	r0, r1, #0x1
 80034ce: f807 0c24    	strb	r0, [r7, #-36]
 80034d2: 47a0         	blx	r4
 80034d4: f847 5c24    	str	r5, [r7, #-36]
 80034d8: 4606         	mov	r6, r0
 80034da: 4658         	mov	r0, r11
 80034dc: f857 1c24    	ldr	r1, [r7, #-36]
 80034e0: f7fe fb85    	bl	0x8001bee <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3> @ imm = #-0x18f6
 80034e4: 07c0         	lsls	r0, r0, #0x1f
 80034e6: f040 80ce    	bne.w	0x8003686 <raylar_crcl_encoder_service::benchmark::split_transform::h9238bb9046f68a6e+0x2bc> @ imm = #0x19c
 80034ea: f001 0001    	and	r0, r1, #0x1
 80034ee: 9615         	str	r6, [sp, #0x54]
 80034f0: f807 0c24    	strb	r0, [r7, #-36]
 80034f4: 47a0         	blx	r4
 80034f6: 4680         	mov	r8, r0
 80034f8: 47a0         	blx	r4
 80034fa: f847 5c24    	str	r5, [r7, #-36]
 80034fe: 4606         	mov	r6, r0
 8003500: 4658         	mov	r0, r11
 8003502: f857 1c24    	ldr	r1, [r7, #-36]
 8003506: f7fe fb72    	bl	0x8001bee <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3> @ imm = #-0x191c
 800350a: 07c0         	lsls	r0, r0, #0x1f
 800350c: f040 80b4    	bne.w	0x8003678 <raylar_crcl_encoder_service::benchmark::split_transform::h9238bb9046f68a6e+0x2ae> @ imm = #0x168
 8003510: f001 0001    	and	r0, r1, #0x1
 8003514: f807 0c24    	strb	r0, [r7, #-36]
 8003518: 47a0         	blx	r4
 800351a: 9014         	str	r0, [sp, #0x50]
 800351c: 4658         	mov	r0, r11
 800351e: f847 5c24    	str	r5, [r7, #-36]
 8003522: f857 1c24    	ldr	r1, [r7, #-36]
 8003526: f7fe fb62    	bl	0x8001bee <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3> @ imm = #-0x193c
 800352a: 07c0         	lsls	r0, r0, #0x1f
 800352c: f040 80ab    	bne.w	0x8003686 <raylar_crcl_encoder_service::benchmark::split_transform::h9238bb9046f68a6e+0x2bc> @ imm = #0x156
 8003530: f001 0001    	and	r0, r1, #0x1
 8003534: f807 0c24    	strb	r0, [r7, #-36]
 8003538: 47a0         	blx	r4
 800353a: 9013         	str	r0, [sp, #0x4c]
 800353c: 47a0         	blx	r4
 800353e: 9012         	str	r0, [sp, #0x48]
 8003540: 4658         	mov	r0, r11
 8003542: f847 5c24    	str	r5, [r7, #-36]
 8003546: f857 1c24    	ldr	r1, [r7, #-36]
 800354a: f7fe fb50    	bl	0x8001bee <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3> @ imm = #-0x1960
 800354e: 07c0         	lsls	r0, r0, #0x1f
 8003550: f040 8092    	bne.w	0x8003678 <raylar_crcl_encoder_service::benchmark::split_transform::h9238bb9046f68a6e+0x2ae> @ imm = #0x124
 8003554: f001 0001    	and	r0, r1, #0x1
 8003558: f807 0c24    	strb	r0, [r7, #-36]
 800355c: 47a0         	blx	r4
 800355e: 9011         	str	r0, [sp, #0x44]
 8003560: 4658         	mov	r0, r11
 8003562: f847 5c24    	str	r5, [r7, #-36]
 8003566: f857 1c24    	ldr	r1, [r7, #-36]
 800356a: f7fe fb40    	bl	0x8001bee <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3> @ imm = #-0x1980
 800356e: 07c0         	lsls	r0, r0, #0x1f
 8003570: f040 8089    	bne.w	0x8003686 <raylar_crcl_encoder_service::benchmark::split_transform::h9238bb9046f68a6e+0x2bc> @ imm = #0x112
 8003574: f001 0001    	and	r0, r1, #0x1
 8003578: f807 0c24    	strb	r0, [r7, #-36]
 800357c: 47a0         	blx	r4
 800357e: 9010         	str	r0, [sp, #0x40]
 8003580: 47a0         	blx	r4
 8003582: 900f         	str	r0, [sp, #0x3c]
 8003584: 4658         	mov	r0, r11
 8003586: f847 5c24    	str	r5, [r7, #-36]
 800358a: f857 1c24    	ldr	r1, [r7, #-36]
 800358e: f7fe fb2e    	bl	0x8001bee <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3> @ imm = #-0x19a4
 8003592: 07c0         	lsls	r0, r0, #0x1f
 8003594: d170         	bne	0x8003678 <raylar_crcl_encoder_service::benchmark::split_transform::h9238bb9046f68a6e+0x2ae> @ imm = #0xe0
 8003596: f001 0001    	and	r0, r1, #0x1
 800359a: f807 0c24    	strb	r0, [r7, #-36]
 800359e: 47a0         	blx	r4
 80035a0: 900e         	str	r0, [sp, #0x38]
 80035a2: 4658         	mov	r0, r11
 80035a4: f847 5c24    	str	r5, [r7, #-36]
 80035a8: f857 1c24    	ldr	r1, [r7, #-36]
 80035ac: f7fe fb1f    	bl	0x8001bee <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3> @ imm = #-0x19c2
 80035b0: 07c0         	lsls	r0, r0, #0x1f
 80035b2: d168         	bne	0x8003686 <raylar_crcl_encoder_service::benchmark::split_transform::h9238bb9046f68a6e+0x2bc> @ imm = #0xd0
 80035b4: f001 0001    	and	r0, r1, #0x1
 80035b8: f807 0c24    	strb	r0, [r7, #-36]
 80035bc: 47a0         	blx	r4
 80035be: 900d         	str	r0, [sp, #0x34]
 80035c0: 47a0         	blx	r4
 80035c2: 900c         	str	r0, [sp, #0x30]
 80035c4: 4658         	mov	r0, r11
 80035c6: f847 5c24    	str	r5, [r7, #-36]
 80035ca: f857 1c24    	ldr	r1, [r7, #-36]
 80035ce: f7fe fb0e    	bl	0x8001bee <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3> @ imm = #-0x19e4
 80035d2: 07c0         	lsls	r0, r0, #0x1f
 80035d4: d150         	bne	0x8003678 <raylar_crcl_encoder_service::benchmark::split_transform::h9238bb9046f68a6e+0x2ae> @ imm = #0xa0
 80035d6: f001 0001    	and	r0, r1, #0x1
 80035da: e9cd 680a    	strd	r6, r8, [sp, #40]
 80035de: f807 0c24    	strb	r0, [r7, #-36]
 80035e2: 47a0         	blx	r4
 80035e4: f847 5c24    	str	r5, [r7, #-36]
 80035e8: 4680         	mov	r8, r0
 80035ea: 4658         	mov	r0, r11
 80035ec: f857 1c24    	ldr	r1, [r7, #-36]
 80035f0: f7fe fafd    	bl	0x8001bee <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3> @ imm = #-0x1a06
 80035f4: 07c0         	lsls	r0, r0, #0x1f
 80035f6: d146         	bne	0x8003686 <raylar_crcl_encoder_service::benchmark::split_transform::h9238bb9046f68a6e+0x2bc> @ imm = #0x8c
 80035f8: f001 0001    	and	r0, r1, #0x1
 80035fc: f807 0c24    	strb	r0, [r7, #-36]
 8003600: 9d15         	ldr	r5, [sp, #0x54]
 8003602: 46cb         	mov	r11, r9
 8003604: 47a0         	blx	r4
 8003606: 9903         	ldr	r1, [sp, #0xc]
 8003608: 4681         	mov	r9, r0
 800360a: f857 0c78    	ldr	r0, [r7, #-120]
 800360e: f847 1c28    	str	r1, [r7, #-40]
 8003612: f5b0 5fc0    	cmp.w	r0, #0x1800
 8003616: d14a         	bne	0x80036ae <raylar_crcl_encoder_service::benchmark::split_transform::h9238bb9046f68a6e+0x2e4> @ imm = #0x94
 8003618: f857 0c7c    	ldr	r0, [r7, #-124]
 800361c: f44f 42c0    	mov.w	r2, #0x6000
 8003620: f009 fc44    	bl	0x800ceac <memcmp>      @ imm = #0x9888
 8003624: 2800         	cmp	r0, #0x0
 8003626: d142         	bne	0x80036ae <raylar_crcl_encoder_service::benchmark::split_transform::h9238bb9046f68a6e+0x2e4> @ imm = #0x84
 8003628: 9e16         	ldr	r6, [sp, #0x58]
 800362a: f106 0008    	add.w	r0, r6, #0x8
 800362e: 2807         	cmp	r0, #0x7
 8003630: f67f aefe    	bls.w	0x8003430 <raylar_crcl_encoder_service::benchmark::split_transform::h9238bb9046f68a6e+0x66> @ imm = #-0x204
 8003634: 9a0a         	ldr	r2, [sp, #0x28]
 8003636: 9814         	ldr	r0, [sp, #0x50]
 8003638: eb0a 0102    	add.w	r1, r10, r2
 800363c: 9a11         	ldr	r2, [sp, #0x44]
 800363e: 4428         	add	r0, r5
 8003640: 9d13         	ldr	r5, [sp, #0x4c]
 8003642: 4410         	add	r0, r2
 8003644: 9a0b         	ldr	r2, [sp, #0x2c]
 8003646: 442a         	add	r2, r5
 8003648: 9d12         	ldr	r5, [sp, #0x48]
 800364a: 4429         	add	r1, r5
 800364c: 9d0e         	ldr	r5, [sp, #0x38]
 800364e: 4428         	add	r0, r5
 8003650: 9d10         	ldr	r5, [sp, #0x40]
 8003652: 442a         	add	r2, r5
 8003654: 9d0f         	ldr	r5, [sp, #0x3c]
 8003656: 4429         	add	r1, r5
 8003658: 9d0c         	ldr	r5, [sp, #0x30]
 800365a: 4429         	add	r1, r5
 800365c: 9d0d         	ldr	r5, [sp, #0x34]
 800365e: 1a41         	subs	r1, r0, r1
 8003660: 4440         	add	r0, r8
 8003662: 442a         	add	r2, r5
 8003664: 4441         	add	r1, r8
 8003666: 1a10         	subs	r0, r2, r0
 8003668: aa98         	add	r2, sp, #0x260
 800366a: 4448         	add	r0, r9
 800366c: f842 0026    	str.w	r0, [r2, r6, lsl #2]
 8003670: a817         	add	r0, sp, #0x5c
 8003672: f840 1026    	str.w	r1, [r0, r6, lsl #2]
 8003676: e6db         	b	0x8003430 <raylar_crcl_encoder_service::benchmark::split_transform::h9238bb9046f68a6e+0x66> @ imm = #-0x24a
 8003678: f24e 609c    	movw	r0, #0xe69c
 800367c: f807 1c24    	strb	r1, [r7, #-36]
 8003680: f6c0 0000    	movt	r0, #0x800
 8003684: e005         	b	0x8003692 <raylar_crcl_encoder_service::benchmark::split_transform::h9238bb9046f68a6e+0x2c8> @ imm = #0xa
 8003686: f24e 608c    	movw	r0, #0xe68c
 800368a: f807 1c24    	strb	r1, [r7, #-36]
 800368e: f6c0 0000    	movt	r0, #0x800
 8003692: f24e 539c    	movw	r3, #0xe59c
 8003696: 9000         	str	r0, [sp]
 8003698: f24e 5000    	movw	r0, #0xe500
 800369c: f1a7 0224    	sub.w	r2, r7, #0x24
 80036a0: f6c0 0000    	movt	r0, #0x800
 80036a4: f6c0 0300    	movt	r3, #0x800
 80036a8: 212b         	movs	r1, #0x2b
 80036aa: f008 fbfd    	bl	0x800bea8 <core::result::unwrap_failed> @ imm = #0x87fa
 80036ae: f24e 607c    	movw	r0, #0xe67c
 80036b2: f1a7 0228    	sub.w	r2, r7, #0x28
 80036b6: f6c0 0000    	movt	r0, #0x800
 80036ba: 2300         	movs	r3, #0x0
 80036bc: 9001         	str	r0, [sp, #0x4]
 80036be: f1a7 00bc    	sub.w	r0, r7, #0xbc
 80036c2: f100 0140    	add.w	r1, r0, #0x40
 80036c6: 2000         	movs	r0, #0x0
 80036c8: f005 fd1e    	bl	0x8009108 <core::panicking::assert_failed::h08239383d2bf65a5> @ imm = #0x5a3c
 80036cc: f1a7 041d    	sub.w	r4, r7, #0x1d
 80036d0: a817         	add	r0, sp, #0x5c
 80036d2: 2181         	movs	r1, #0x81
 80036d4: 4622         	mov	r2, r4
 80036d6: f004 fe06    	bl	0x80082e6 <core::slice::sort::unstable::ipnsort::hd8a371dd4e708cab> @ imm = #0x4c0c
 80036da: a898         	add	r0, sp, #0x260
 80036dc: 2181         	movs	r1, #0x81
 80036de: 4622         	mov	r2, r4
 80036e0: f004 fe01    	bl	0x80082e6 <core::slice::sort::unstable::ipnsort::hd8a371dd4e708cab> @ imm = #0x4c02
 80036e4: 9802         	ldr	r0, [sp, #0x8]
 80036e6: f24e 6ac8    	movw	r10, #0xe6c8
 80036ea: f04f 091f    	mov.w	r9, #0x1f
 80036ee: f6c0 0a00    	movt	r10, #0x800
 80036f2: f8dd c260    	ldr.w	r12, [sp, #0x260]
 80036f6: f04f 0b01    	mov.w	r11, #0x1
 80036fa: e9c0 a908    	strd	r10, r9, [r0, #32]
 80036fe: f100 0928    	add.w	r9, r0, #0x28
 8003702: f8dd 342c    	ldr.w	r3, [sp, #0x42c]
 8003706: f04f 0800    	mov.w	r8, #0x0
 800370a: f8dd e360    	ldr.w	lr, [sp, #0x360]
 800370e: e889 5800    	stm.w	r9, {r11, r12, lr}
 8003712: f100 0c34    	add.w	r12, r0, #0x34
 8003716: f8dd 6460    	ldr.w	r6, [sp, #0x460]
 800371a: e88c 0148    	stm.w	r12, {r3, r6, r8}
 800371e: f24e 63ac    	movw	r3, #0xe6ac
 8003722: f6c0 0300    	movt	r3, #0x800
 8003726: 9a57         	ldr	r2, [sp, #0x15c]
 8003728: 9997         	ldr	r1, [sp, #0x25c]
 800372a: 9c17         	ldr	r4, [sp, #0x5c]
 800372c: 9d8a         	ldr	r5, [sp, #0x228]
 800372e: 6003         	str	r3, [r0]
 8003730: 231c         	movs	r3, #0x1c
 8003732: e9c0 3b01    	strd	r3, r11, [r0, #4]
 8003736: e9c0 4203    	strd	r4, r2, [r0, #12]
 800373a: e9c0 5105    	strd	r5, r1, [r0, #20]
 800373e: f8c0 801c    	str.w	r8, [r0, #0x1c]
 8003742: f20d 5d04    	addw	sp, sp, #0x504
 8003746: e8bd 0f00    	pop.w	{r8, r9, r10, r11}
 800374a: bdf0         	pop	{r4, r5, r6, r7, pc}

0800374c <raylar_crcl_encoder_service::benchmark::transform_budgeted::h9752c8449280daff>:
 800374c: b5f0         	push	{r4, r5, r6, r7, lr}
 800374e: af03         	add	r7, sp, #0xc
 8003750: f84d 8d04    	str	r8, [sp, #-4]!
 8003754: b0aa         	sub	sp, #0xa8
 8003756: f240 610a    	movw	r1, #0x60a
 800375a: f2c0 1101    	movt	r1, #0x101
 800375e: 9129         	str	r1, [sp, #0xa4]
 8003760: f44f 0100    	mov.w	r1, #0x800000
 8003764: 9128         	str	r1, [sp, #0xa0]
 8003766: f44f 6180    	mov.w	r1, #0x400
 800376a: 9127         	str	r1, [sp, #0x9c]
 800376c: a927         	add	r1, sp, #0x9c
 800376e: 9927         	ldr	r1, [sp, #0x9c]
 8003770: 1e4a         	subs	r2, r1, #0x1
 8003772: ea81 0302    	eor.w	r3, r1, r2
 8003776: 4293         	cmp	r3, r2
 8003778: d916         	bls	0x80037a8 <raylar_crcl_encoder_service::benchmark::transform_budgeted::h9752c8449280daff+0x5c> @ imm = #0x2c
 800377a: fa91 f2a1    	rbit	r2, r1
 800377e: fab2 f282    	clz	r2, r2
 8003782: 3a08         	subs	r2, #0x8
 8003784: 2a04         	cmp	r2, #0x4
 8003786: bf9c         	itt	ls
 8003788: f89d 20a4    	ldrbls.w	r2, [sp, #0xa4]
 800378c: 2a0c         	cmpls	r2, #0xc
 800378e: d80b         	bhi	0x80037a8 <raylar_crcl_encoder_service::benchmark::transform_budgeted::h9752c8449280daff+0x5c> @ imm = #0x16
 8003790: 2301         	movs	r3, #0x1
 8003792: fa03 f202    	lsl.w	r2, r3, r2
 8003796: f412 5faa    	tst.w	r2, #0x1540
 800379a: bf1e         	ittt	ne
 800379c: 9a28         	ldrne	r2, [sp, #0xa0]
 800379e: f102 4240    	addne.w	r2, r2, #0xc0000000
 80037a2: f1b2 4f40    	cmpne.w	r2, #0xc0000000
 80037a6: d813         	bhi	0x80037d0 <raylar_crcl_encoder_service::benchmark::transform_budgeted::h9752c8449280daff+0x84> @ imm = #0x26
 80037a8: 2000         	movs	r0, #0x0
 80037aa: f24e 539c    	movw	r3, #0xe59c
 80037ae: f88d 0008    	strb.w	r0, [sp, #0x8]
 80037b2: f24e 60f8    	movw	r0, #0xe6f8
 80037b6: f6c0 0000    	movt	r0, #0x800
 80037ba: aa02         	add	r2, sp, #0x8
 80037bc: 9000         	str	r0, [sp]
 80037be: f24e 5000    	movw	r0, #0xe500
 80037c2: f6c0 0000    	movt	r0, #0x800
 80037c6: f6c0 0300    	movt	r3, #0x800
 80037ca: 212b         	movs	r1, #0x2b
 80037cc: f008 fb6c    	bl	0x800bea8 <core::result::unwrap_failed> @ imm = #0x86d8
 80037d0: f89d 20a5    	ldrb.w	r2, [sp, #0xa5]
 80037d4: 2a1e         	cmp	r2, #0x1e
 80037d6: d8e7         	bhi	0x80037a8 <raylar_crcl_encoder_service::benchmark::transform_budgeted::h9752c8449280daff+0x5c> @ imm = #-0x32
 80037d8: f89d 30a7    	ldrb.w	r3, [sp, #0xa7]
 80037dc: 2b01         	cmp	r3, #0x1
 80037de: d107         	bne	0x80037f0 <raylar_crcl_encoder_service::benchmark::transform_budgeted::h9752c8449280daff+0xa4> @ imm = #0xe
 80037e0: f5b1 6f80    	cmp.w	r1, #0x400
 80037e4: d3e0         	blo	0x80037a8 <raylar_crcl_encoder_service::benchmark::transform_budgeted::h9752c8449280daff+0x5c> @ imm = #-0x40
 80037e6: f89d 10a6    	ldrb.w	r1, [sp, #0xa6]
 80037ea: 2901         	cmp	r1, #0x1
 80037ec: d005         	beq	0x80037fa <raylar_crcl_encoder_service::benchmark::transform_budgeted::h9752c8449280daff+0xae> @ imm = #0xa
 80037ee: e7db         	b	0x80037a8 <raylar_crcl_encoder_service::benchmark::transform_budgeted::h9752c8449280daff+0x5c> @ imm = #-0x4a
 80037f0: f5b1 6f80    	cmp.w	r1, #0x400
 80037f4: d3d8         	blo	0x80037a8 <raylar_crcl_encoder_service::benchmark::transform_budgeted::h9752c8449280daff+0x5c> @ imm = #-0x50
 80037f6: 2a00         	cmp	r2, #0x0
 80037f8: d1d6         	bne	0x80037a8 <raylar_crcl_encoder_service::benchmark::transform_budgeted::h9752c8449280daff+0x5c> @ imm = #-0x54
 80037fa: ac02         	add	r4, sp, #0x8
 80037fc: f246 0128    	movw	r1, #0x6028
 8003800: 1845         	adds	r5, r0, r1
 8003802: f104 0048    	add.w	r0, r4, #0x48
 8003806: 2128         	movs	r1, #0x28
 8003808: f009 fdb3    	bl	0x800d372 <__aeabi_memclr4> @ imm = #0x9b66
 800380c: ab27         	add	r3, sp, #0x9c
 800380e: 9512         	str	r5, [sp, #0x48]
 8003810: f44f 56c0    	mov.w	r6, #0x1800
 8003814: f104 0840    	add.w	r8, r4, #0x40
 8003818: cb0e         	ldm	r3, {r1, r2, r3}
 800381a: ad27         	add	r5, sp, #0x9c
 800381c: 2000         	movs	r0, #0x0
 800381e: 9613         	str	r6, [sp, #0x4c]
 8003820: f44f 7680    	mov.w	r6, #0x100
 8003824: e9cd 2324    	strd	r2, r3, [sp, #144]
 8003828: 2201         	movs	r2, #0x1
 800382a: f8ad 0024    	strh.w	r0, [sp, #0x24]
 800382e: e9cd 0007    	strd	r0, r0, [sp, #28]
 8003832: e9cd 0005    	strd	r0, r0, [sp, #20]
 8003836: e9cd 0003    	strd	r0, r0, [sp, #12]
 800383a: e9cd 000b    	strd	r0, r0, [sp, #44]
 800383e: e9cd 000d    	strd	r0, r0, [sp, #52]
 8003842: e9cd 000f    	strd	r0, r0, [sp, #60]
 8003846: f8ad 0044    	strh.w	r0, [sp, #0x44]
 800384a: e9cd 001f    	strd	r0, r0, [sp, #124]
 800384e: e9cd 0021    	strd	r0, r0, [sp, #132]
 8003852: 9002         	str	r0, [sp, #0x8]
 8003854: 900a         	str	r0, [sp, #0x28]
 8003856: 921e         	str	r2, [sp, #0x78]
 8003858: 9123         	str	r1, [sp, #0x8c]
 800385a: f8ad 0098    	strh.w	r0, [sp, #0x98]
 800385e: 9627         	str	r6, [sp, #0x9c]
 8003860: 4620         	mov	r0, r4
 8003862: 9927         	ldr	r1, [sp, #0x9c]
 8003864: f7fe f9c3    	bl	0x8001bee <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3> @ imm = #-0x1c7a
 8003868: 07c0         	lsls	r0, r0, #0x1f
 800386a: d109         	bne	0x8003880 <raylar_crcl_encoder_service::benchmark::transform_budgeted::h9752c8449280daff+0x134> @ imm = #0x12
 800386c: 07c8         	lsls	r0, r1, #0x1f
 800386e: d0f6         	beq	0x800385e <raylar_crcl_encoder_service::benchmark::transform_budgeted::h9752c8449280daff+0x112> @ imm = #-0x14
 8003870: f8cd 809c    	str.w	r8, [sp, #0x9c]
 8003874: a827         	add	r0, sp, #0x9c
 8003876: 9814         	ldr	r0, [sp, #0x50]
 8003878: b02a         	add	sp, #0xa8
 800387a: f85d 8b04    	ldr	r8, [sp], #4
 800387e: bdf0         	pop	{r4, r5, r6, r7, pc}
 8003880: f24e 60e8    	movw	r0, #0xe6e8
 8003884: f24e 539c    	movw	r3, #0xe59c
 8003888: f6c0 0000    	movt	r0, #0x800
 800388c: aa27         	add	r2, sp, #0x9c
 800388e: 9000         	str	r0, [sp]
 8003890: f24e 5000    	movw	r0, #0xe500
 8003894: f88d 109c    	strb.w	r1, [sp, #0x9c]
 8003898: f6c0 0000    	movt	r0, #0x800
 800389c: f6c0 0300    	movt	r3, #0x800
 80038a0: 212b         	movs	r1, #0x2b
 80038a2: f008 fb01    	bl	0x800bea8 <core::result::unwrap_failed> @ imm = #0x8602

080038a6 <raylar_crcl_encoder_service::benchmark::reference_transform::hacbcea1c31d8ae68>:
 80038a6: b5f0         	push	{r4, r5, r6, r7, lr}
 80038a8: af03         	add	r7, sp, #0xc
 80038aa: f84d 8d04    	str	r8, [sp, #-4]!
 80038ae: b0aa         	sub	sp, #0xa8
 80038b0: f240 610a    	movw	r1, #0x60a
 80038b4: f2c0 1101    	movt	r1, #0x101
 80038b8: 9129         	str	r1, [sp, #0xa4]
 80038ba: f44f 0100    	mov.w	r1, #0x800000
 80038be: 9128         	str	r1, [sp, #0xa0]
 80038c0: f44f 6180    	mov.w	r1, #0x400
 80038c4: 9127         	str	r1, [sp, #0x9c]
 80038c6: a927         	add	r1, sp, #0x9c
 80038c8: 9927         	ldr	r1, [sp, #0x9c]
 80038ca: 1e4a         	subs	r2, r1, #0x1
 80038cc: ea81 0302    	eor.w	r3, r1, r2
 80038d0: 4293         	cmp	r3, r2
 80038d2: d916         	bls	0x8003902 <raylar_crcl_encoder_service::benchmark::reference_transform::hacbcea1c31d8ae68+0x5c> @ imm = #0x2c
 80038d4: fa91 f2a1    	rbit	r2, r1
 80038d8: fab2 f282    	clz	r2, r2
 80038dc: 3a08         	subs	r2, #0x8
 80038de: 2a04         	cmp	r2, #0x4
 80038e0: bf9c         	itt	ls
 80038e2: f89d 20a4    	ldrbls.w	r2, [sp, #0xa4]
 80038e6: 2a0c         	cmpls	r2, #0xc
 80038e8: d80b         	bhi	0x8003902 <raylar_crcl_encoder_service::benchmark::reference_transform::hacbcea1c31d8ae68+0x5c> @ imm = #0x16
 80038ea: 2301         	movs	r3, #0x1
 80038ec: fa03 f202    	lsl.w	r2, r3, r2
 80038f0: f412 5faa    	tst.w	r2, #0x1540
 80038f4: bf1e         	ittt	ne
 80038f6: 9a28         	ldrne	r2, [sp, #0xa0]
 80038f8: f102 4240    	addne.w	r2, r2, #0xc0000000
 80038fc: f1b2 4f40    	cmpne.w	r2, #0xc0000000
 8003900: d813         	bhi	0x800392a <raylar_crcl_encoder_service::benchmark::reference_transform::hacbcea1c31d8ae68+0x84> @ imm = #0x26
 8003902: 2000         	movs	r0, #0x0
 8003904: f24e 539c    	movw	r3, #0xe59c
 8003908: f88d 0008    	strb.w	r0, [sp, #0x8]
 800390c: f24e 7018    	movw	r0, #0xe718
 8003910: f6c0 0000    	movt	r0, #0x800
 8003914: aa02         	add	r2, sp, #0x8
 8003916: 9000         	str	r0, [sp]
 8003918: f24e 5000    	movw	r0, #0xe500
 800391c: f6c0 0000    	movt	r0, #0x800
 8003920: f6c0 0300    	movt	r3, #0x800
 8003924: 212b         	movs	r1, #0x2b
 8003926: f008 fabf    	bl	0x800bea8 <core::result::unwrap_failed> @ imm = #0x857e
 800392a: f89d 20a5    	ldrb.w	r2, [sp, #0xa5]
 800392e: 2a1e         	cmp	r2, #0x1e
 8003930: d8e7         	bhi	0x8003902 <raylar_crcl_encoder_service::benchmark::reference_transform::hacbcea1c31d8ae68+0x5c> @ imm = #-0x32
 8003932: f89d 30a7    	ldrb.w	r3, [sp, #0xa7]
 8003936: 2b01         	cmp	r3, #0x1
 8003938: d107         	bne	0x800394a <raylar_crcl_encoder_service::benchmark::reference_transform::hacbcea1c31d8ae68+0xa4> @ imm = #0xe
 800393a: f5b1 6f80    	cmp.w	r1, #0x400
 800393e: d3e0         	blo	0x8003902 <raylar_crcl_encoder_service::benchmark::reference_transform::hacbcea1c31d8ae68+0x5c> @ imm = #-0x40
 8003940: f89d 10a6    	ldrb.w	r1, [sp, #0xa6]
 8003944: 2901         	cmp	r1, #0x1
 8003946: d005         	beq	0x8003954 <raylar_crcl_encoder_service::benchmark::reference_transform::hacbcea1c31d8ae68+0xae> @ imm = #0xa
 8003948: e7db         	b	0x8003902 <raylar_crcl_encoder_service::benchmark::reference_transform::hacbcea1c31d8ae68+0x5c> @ imm = #-0x4a
 800394a: f5b1 6f80    	cmp.w	r1, #0x400
 800394e: d3d8         	blo	0x8003902 <raylar_crcl_encoder_service::benchmark::reference_transform::hacbcea1c31d8ae68+0x5c> @ imm = #-0x50
 8003950: 2a00         	cmp	r2, #0x0
 8003952: d1d6         	bne	0x8003902 <raylar_crcl_encoder_service::benchmark::reference_transform::hacbcea1c31d8ae68+0x5c> @ imm = #-0x54
 8003954: ac02         	add	r4, sp, #0x8
 8003956: f246 0128    	movw	r1, #0x6028
 800395a: 1845         	adds	r5, r0, r1
 800395c: f104 0048    	add.w	r0, r4, #0x48
 8003960: 2128         	movs	r1, #0x28
 8003962: f009 fd06    	bl	0x800d372 <__aeabi_memclr4> @ imm = #0x9a0c
 8003966: ab27         	add	r3, sp, #0x9c
 8003968: 9512         	str	r5, [sp, #0x48]
 800396a: f44f 56c0    	mov.w	r6, #0x1800
 800396e: f104 0840    	add.w	r8, r4, #0x40
 8003972: cb0e         	ldm	r3, {r1, r2, r3}
 8003974: ad27         	add	r5, sp, #0x9c
 8003976: 2000         	movs	r0, #0x0
 8003978: 9613         	str	r6, [sp, #0x4c]
 800397a: f44f 7680    	mov.w	r6, #0x100
 800397e: e9cd 2324    	strd	r2, r3, [sp, #144]
 8003982: 2201         	movs	r2, #0x1
 8003984: f8ad 0024    	strh.w	r0, [sp, #0x24]
 8003988: e9cd 0007    	strd	r0, r0, [sp, #28]
 800398c: e9cd 0005    	strd	r0, r0, [sp, #20]
 8003990: e9cd 0003    	strd	r0, r0, [sp, #12]
 8003994: e9cd 000b    	strd	r0, r0, [sp, #44]
 8003998: e9cd 000d    	strd	r0, r0, [sp, #52]
 800399c: e9cd 000f    	strd	r0, r0, [sp, #60]
 80039a0: f8ad 0044    	strh.w	r0, [sp, #0x44]
 80039a4: e9cd 001f    	strd	r0, r0, [sp, #124]
 80039a8: e9cd 0021    	strd	r0, r0, [sp, #132]
 80039ac: 9002         	str	r0, [sp, #0x8]
 80039ae: 900a         	str	r0, [sp, #0x28]
 80039b0: 921e         	str	r2, [sp, #0x78]
 80039b2: 9123         	str	r1, [sp, #0x8c]
 80039b4: f8ad 0098    	strh.w	r0, [sp, #0x98]
 80039b8: 9627         	str	r6, [sp, #0x9c]
 80039ba: 4620         	mov	r0, r4
 80039bc: 9927         	ldr	r1, [sp, #0x9c]
 80039be: f7fd fc54    	bl	0x800126a <raylar_crcl_encoder_service::cheap::reference::_$LT$impl$u20$raylar_crcl_encoder_service..quantize..Transform$GT$::advance_cheap_reference::hc3bf4874397b54bd> @ imm = #-0x2758
 80039c2: 07c0         	lsls	r0, r0, #0x1f
 80039c4: d109         	bne	0x80039da <raylar_crcl_encoder_service::benchmark::reference_transform::hacbcea1c31d8ae68+0x134> @ imm = #0x12
 80039c6: 07c8         	lsls	r0, r1, #0x1f
 80039c8: d0f6         	beq	0x80039b8 <raylar_crcl_encoder_service::benchmark::reference_transform::hacbcea1c31d8ae68+0x112> @ imm = #-0x14
 80039ca: f8cd 809c    	str.w	r8, [sp, #0x9c]
 80039ce: a827         	add	r0, sp, #0x9c
 80039d0: 9814         	ldr	r0, [sp, #0x50]
 80039d2: b02a         	add	sp, #0xa8
 80039d4: f85d 8b04    	ldr	r8, [sp], #4
 80039d8: bdf0         	pop	{r4, r5, r6, r7, pc}
 80039da: f24e 7008    	movw	r0, #0xe708
 80039de: f24e 539c    	movw	r3, #0xe59c
 80039e2: f6c0 0000    	movt	r0, #0x800
 80039e6: aa27         	add	r2, sp, #0x9c
 80039e8: 9000         	str	r0, [sp]
 80039ea: f24e 5000    	movw	r0, #0xe500
 80039ee: f88d 109c    	strb.w	r1, [sp, #0x9c]
 80039f2: f6c0 0000    	movt	r0, #0x800
 80039f6: f6c0 0300    	movt	r3, #0x800
 80039fa: 212b         	movs	r1, #0x2b
 80039fc: f008 fa54    	bl	0x800bea8 <core::result::unwrap_failed> @ imm = #0x84a8

08003a00 <raylar_crcl_encoder_service::benchmark::serialize_production::h9a703500dd702791>:
 8003a00: b5f0         	push	{r4, r5, r6, r7, lr}
 8003a02: af03         	add	r7, sp, #0xc
 8003a04: e92d 0b00    	push.w	{r8, r9, r11}
 8003a08: b0b8         	sub	sp, #0xe0
 8003a0a: f240 6c0a    	movw	r12, #0x60a
 8003a0e: f44f 0100    	mov.w	r1, #0x800000
 8003a12: f2c0 1c01    	movt	r12, #0x101
 8003a16: 9127         	str	r1, [sp, #0x9c]
 8003a18: f44f 6180    	mov.w	r1, #0x400
 8003a1c: f8cd c0a0    	str.w	r12, [sp, #0xa0]
 8003a20: 9126         	str	r1, [sp, #0x98]
 8003a22: f10d 0898    	add.w	r8, sp, #0x98
 8003a26: 9926         	ldr	r1, [sp, #0x98]
 8003a28: 1e4a         	subs	r2, r1, #0x1
 8003a2a: ea81 0302    	eor.w	r3, r1, r2
 8003a2e: 4293         	cmp	r3, r2
 8003a30: d916         	bls	0x8003a60 <raylar_crcl_encoder_service::benchmark::serialize_production::h9a703500dd702791+0x60> @ imm = #0x2c
 8003a32: fa91 f2a1    	rbit	r2, r1
 8003a36: fab2 f282    	clz	r2, r2
 8003a3a: 3a08         	subs	r2, #0x8
 8003a3c: 2a04         	cmp	r2, #0x4
 8003a3e: bf9c         	itt	ls
 8003a40: f89d 20a0    	ldrbls.w	r2, [sp, #0xa0]
 8003a44: 2a0c         	cmpls	r2, #0xc
 8003a46: d80b         	bhi	0x8003a60 <raylar_crcl_encoder_service::benchmark::serialize_production::h9a703500dd702791+0x60> @ imm = #0x16
 8003a48: 2301         	movs	r3, #0x1
 8003a4a: fa03 f202    	lsl.w	r2, r3, r2
 8003a4e: f412 5faa    	tst.w	r2, #0x1540
 8003a52: bf1e         	ittt	ne
 8003a54: 9a27         	ldrne	r2, [sp, #0x9c]
 8003a56: f102 4240    	addne.w	r2, r2, #0xc0000000
 8003a5a: f1b2 4f40    	cmpne.w	r2, #0xc0000000
 8003a5e: d813         	bhi	0x8003a88 <raylar_crcl_encoder_service::benchmark::serialize_production::h9a703500dd702791+0x88> @ imm = #0x26
 8003a60: 2000         	movs	r0, #0x0
 8003a62: f88d 0098    	strb.w	r0, [sp, #0x98]
 8003a66: f24e 7058    	movw	r0, #0xe758
 8003a6a: f6c0 0000    	movt	r0, #0x800
 8003a6e: f24e 539c    	movw	r3, #0xe59c
 8003a72: 9000         	str	r0, [sp]
 8003a74: f24e 5000    	movw	r0, #0xe500
 8003a78: aa26         	add	r2, sp, #0x98
 8003a7a: f6c0 0000    	movt	r0, #0x800
 8003a7e: f6c0 0300    	movt	r3, #0x800
 8003a82: 212b         	movs	r1, #0x2b
 8003a84: f008 fa10    	bl	0x800bea8 <core::result::unwrap_failed> @ imm = #0x8420
 8003a88: f89d 20a1    	ldrb.w	r2, [sp, #0xa1]
 8003a8c: 2a1e         	cmp	r2, #0x1e
 8003a8e: d8e7         	bhi	0x8003a60 <raylar_crcl_encoder_service::benchmark::serialize_production::h9a703500dd702791+0x60> @ imm = #-0x32
 8003a90: f89d 30a3    	ldrb.w	r3, [sp, #0xa3]
 8003a94: 4681         	mov	r9, r0
 8003a96: 2b01         	cmp	r3, #0x1
 8003a98: d107         	bne	0x8003aaa <raylar_crcl_encoder_service::benchmark::serialize_production::h9a703500dd702791+0xaa> @ imm = #0xe
 8003a9a: f5b1 6f80    	cmp.w	r1, #0x400
 8003a9e: d3df         	blo	0x8003a60 <raylar_crcl_encoder_service::benchmark::serialize_production::h9a703500dd702791+0x60> @ imm = #-0x42
 8003aa0: f89d 10a2    	ldrb.w	r1, [sp, #0xa2]
 8003aa4: 2901         	cmp	r1, #0x1
 8003aa6: d005         	beq	0x8003ab4 <raylar_crcl_encoder_service::benchmark::serialize_production::h9a703500dd702791+0xb4> @ imm = #0xa
 8003aa8: e7da         	b	0x8003a60 <raylar_crcl_encoder_service::benchmark::serialize_production::h9a703500dd702791+0x60> @ imm = #-0x4c
 8003aaa: f5b1 6f80    	cmp.w	r1, #0x400
 8003aae: d3d7         	blo	0x8003a60 <raylar_crcl_encoder_service::benchmark::serialize_production::h9a703500dd702791+0x60> @ imm = #-0x52
 8003ab0: 2a00         	cmp	r2, #0x0
 8003ab2: d1d5         	bne	0x8003a60 <raylar_crcl_encoder_service::benchmark::serialize_production::h9a703500dd702791+0x60> @ imm = #-0x56
 8003ab4: f246 0128    	movw	r1, #0x6028
 8003ab8: eb09 0201    	add.w	r2, r9, r1
 8003abc: 9211         	str	r2, [sp, #0x44]
 8003abe: f44f 52c0    	mov.w	r2, #0x1800
 8003ac2: f04f 0e00    	mov.w	lr, #0x0
 8003ac6: e9dd 3626    	ldrd	r3, r6, [sp, #152]
 8003aca: 9d28         	ldr	r5, [sp, #0xa0]
 8003acc: 9212         	str	r2, [sp, #0x48]
 8003ace: 2206         	movs	r2, #0x6
 8003ad0: e9cd 3622    	strd	r3, r6, [sp, #136]
 8003ad4: 464b         	mov	r3, r9
 8003ad6: 921d         	str	r2, [sp, #0x74]
 8003ad8: aa01         	add	r2, sp, #0x4
 8003ada: f8ad e020    	strh.w	lr, [sp, #0x20]
 8003ade: 3248         	adds	r2, #0x48
 8003ae0: e9cd ee06    	strd	lr, lr, [sp, #24]
 8003ae4: e9cd ee04    	strd	lr, lr, [sp, #16]
 8003ae8: e9cd ee02    	strd	lr, lr, [sp, #8]
 8003aec: e9cd ee0a    	strd	lr, lr, [sp, #40]
 8003af0: e9cd ee0c    	strd	lr, lr, [sp, #48]
 8003af4: e9cd ee0e    	strd	lr, lr, [sp, #56]
 8003af8: f8ad e040    	strh.w	lr, [sp, #0x40]
 8003afc: e9cd ee1e    	strd	lr, lr, [sp, #120]
 8003b00: e9cd ee20    	strd	lr, lr, [sp, #128]
 8003b04: f8cd e004    	str.w	lr, [sp, #0x4]
 8003b08: f8cd e024    	str.w	lr, [sp, #0x24]
 8003b0c: 9524         	str	r5, [sp, #0x90]
 8003b0e: f8ad e094    	strh.w	lr, [sp, #0x94]
 8003b12: cb73         	ldm	r3!, {r0, r1, r4, r5, r6}
 8003b14: c273         	stm	r2!, {r0, r1, r4, r5, r6}
 8003b16: e893 0073    	ldm.w	r3, {r0, r1, r4, r5, r6}
 8003b1a: c273         	stm	r2!, {r0, r1, r4, r5, r6}
 8003b1c: f44f 0000    	mov.w	r0, #0x800000
 8003b20: 9036         	str	r0, [sp, #0xd8]
 8003b22: f44f 6080    	mov.w	r0, #0x400
 8003b26: 9035         	str	r0, [sp, #0xd4]
 8003b28: a835         	add	r0, sp, #0xd4
 8003b2a: f8cd c0dc    	str.w	r12, [sp, #0xdc]
 8003b2e: 9835         	ldr	r0, [sp, #0xd4]
 8003b30: 1e41         	subs	r1, r0, #0x1
 8003b32: ea80 0201    	eor.w	r2, r0, r1
 8003b36: 428a         	cmp	r2, r1
 8003b38: d90a         	bls	0x8003b50 <raylar_crcl_encoder_service::benchmark::serialize_production::h9a703500dd702791+0x150> @ imm = #0x14
 8003b3a: fa90 f0a0    	rbit	r0, r0
 8003b3e: fab0 f080    	clz	r0, r0
 8003b42: 3808         	subs	r0, #0x8
 8003b44: 2804         	cmp	r0, #0x4
 8003b46: bf9c         	itt	ls
 8003b48: f89d 00dc    	ldrbls.w	r0, [sp, #0xdc]
 8003b4c: 280c         	cmpls	r0, #0xc
 8003b4e: d906         	bls	0x8003b5e <raylar_crcl_encoder_service::benchmark::serialize_production::h9a703500dd702791+0x15e> @ imm = #0xc
 8003b50: f24e 7048    	movw	r0, #0xe748
 8003b54: f88d e098    	strb.w	lr, [sp, #0x98]
 8003b58: f6c0 0000    	movt	r0, #0x800
 8003b5c: e787         	b	0x8003a6e <raylar_crcl_encoder_service::benchmark::serialize_production::h9a703500dd702791+0x6e> @ imm = #-0xf2
 8003b5e: 2101         	movs	r1, #0x1
 8003b60: fa01 f000    	lsl.w	r0, r1, r0
 8003b64: f410 5faa    	tst.w	r0, #0x1540
 8003b68: d0f2         	beq	0x8003b50 <raylar_crcl_encoder_service::benchmark::serialize_production::h9a703500dd702791+0x150> @ imm = #-0x1c
 8003b6a: 9936         	ldr	r1, [sp, #0xd8]
 8003b6c: 2001         	movs	r0, #0x1
 8003b6e: f2cc 0000    	movt	r0, #0xc000
 8003b72: f101 4140    	add.w	r1, r1, #0xc0000000
 8003b76: 4281         	cmp	r1, r0
 8003b78: d3ea         	blo	0x8003b50 <raylar_crcl_encoder_service::benchmark::serialize_production::h9a703500dd702791+0x150> @ imm = #-0x2c
 8003b7a: f89d 00dd    	ldrb.w	r0, [sp, #0xdd]
 8003b7e: 281e         	cmp	r0, #0x1e
 8003b80: d8e6         	bhi	0x8003b50 <raylar_crcl_encoder_service::benchmark::serialize_production::h9a703500dd702791+0x150> @ imm = #-0x34
 8003b82: f89d 60df    	ldrb.w	r6, [sp, #0xdf]
 8003b86: 2e01         	cmp	r6, #0x1
 8003b88: d104         	bne	0x8003b94 <raylar_crcl_encoder_service::benchmark::serialize_production::h9a703500dd702791+0x194> @ imm = #0x8
 8003b8a: f89d 00de    	ldrb.w	r0, [sp, #0xde]
 8003b8e: 2801         	cmp	r0, #0x1
 8003b90: d002         	beq	0x8003b98 <raylar_crcl_encoder_service::benchmark::serialize_production::h9a703500dd702791+0x198> @ imm = #0x4
 8003b92: e7dd         	b	0x8003b50 <raylar_crcl_encoder_service::benchmark::serialize_production::h9a703500dd702791+0x150> @ imm = #-0x46
 8003b94: 2800         	cmp	r0, #0x0
 8003b96: d1db         	bne	0x8003b50 <raylar_crcl_encoder_service::benchmark::serialize_production::h9a703500dd702791+0x150> @ imm = #-0x4a
 8003b98: e9dd 0135    	ldrd	r0, r1, [sp, #212]
 8003b9c: f8dd 20db    	ldr.w	r2, [sp, #0xdb]
 8003ba0: f8cd 00c7    	str.w	r0, [sp, #0xc7]
 8003ba4: 2000         	movs	r0, #0x0
 8003ba6: f8cd 20ce    	str.w	r2, [sp, #0xce]
 8003baa: 221a         	movs	r2, #0x1a
 8003bac: f8cd 10cb    	str.w	r1, [sp, #0xcb]
 8003bb0: a92e         	add	r1, sp, #0xb8
 8003bb2: 9030         	str	r0, [sp, #0xc0]
 8003bb4: e9cd 002e    	strd	r0, r0, [sp, #184]
 8003bb8: f8cd 00c3    	str.w	r0, [sp, #0xc3]
 8003bbc: f88d 0098    	strb.w	r0, [sp, #0x98]
 8003bc0: f108 0001    	add.w	r0, r8, #0x1
 8003bc4: f009 f9ac    	bl	0x800cf20 <__aeabi_memcpy> @ imm = #0x9358
 8003bc8: f641 6058    	movw	r0, #0x1e58
 8003bcc: f509 34e0    	add.w	r4, r9, #0x1c000
 8003bd0: 9000         	str	r0, [sp]
 8003bd2: a82e         	add	r0, sp, #0xb8
 8003bd4: aa01         	add	r2, sp, #0x4
 8003bd6: 4641         	mov	r1, r8
 8003bd8: 4623         	mov	r3, r4
 8003bda: f88d 60b3    	strb.w	r6, [sp, #0xb3]
 8003bde: f7fd fd4f    	bl	0x8001680 <raylar_crcl_encoder_service::format::Encoder::data::ha65fe528befbf4e3> @ imm = #-0x2562
 8003be2: f89d 00b8    	ldrb.w	r0, [sp, #0xb8]
 8003be6: 2801         	cmp	r0, #0x1
 8003be8: d015         	beq	0x8003c16 <raylar_crcl_encoder_service::benchmark::serialize_production::h9a703500dd702791+0x216> @ imm = #0x2a
 8003bea: 992f         	ldr	r1, [sp, #0xbc]
 8003bec: f641 6059    	movw	r0, #0x1e59
 8003bf0: 4281         	cmp	r1, r0
 8003bf2: d207         	bhs	0x8003c04 <raylar_crcl_encoder_service::benchmark::serialize_production::h9a703500dd702791+0x204> @ imm = #0xe
 8003bf4: e9cd 4126    	strd	r4, r1, [sp, #152]
 8003bf8: a826         	add	r0, sp, #0x98
 8003bfa: 4608         	mov	r0, r1
 8003bfc: b038         	add	sp, #0xe0
 8003bfe: e8bd 0b00    	pop.w	{r8, r9, r11}
 8003c02: bdf0         	pop	{r4, r5, r6, r7, pc}
 8003c04: f24e 7328    	movw	r3, #0xe728
 8003c08: 2000         	movs	r0, #0x0
 8003c0a: f6c0 0300    	movt	r3, #0x800
 8003c0e: f641 6258    	movw	r2, #0x1e58
 8003c12: f008 fc54    	bl	0x800c4be <core::slice::index::slice_index_fail> @ imm = #0x88a8
 8003c16: f89d 00b9    	ldrb.w	r0, [sp, #0xb9]
 8003c1a: f24e 539c    	movw	r3, #0xe59c
 8003c1e: f88d 00d4    	strb.w	r0, [sp, #0xd4]
 8003c22: f24e 7038    	movw	r0, #0xe738
 8003c26: f6c0 0000    	movt	r0, #0x800
 8003c2a: aa35         	add	r2, sp, #0xd4
 8003c2c: 9000         	str	r0, [sp]
 8003c2e: f24e 5000    	movw	r0, #0xe500
 8003c32: f6c0 0000    	movt	r0, #0x800
 8003c36: f6c0 0300    	movt	r3, #0x800
 8003c3a: 212b         	movs	r1, #0x2b
 8003c3c: f008 f934    	bl	0x800bea8 <core::result::unwrap_failed> @ imm = #0x8268

08003c40 <raylar_crcl_encoder_service::benchmark::transform_production::he43415fd0806c08a>:
 8003c40: b5f0         	push	{r4, r5, r6, r7, lr}
 8003c42: af03         	add	r7, sp, #0xc
 8003c44: f84d 8d04    	str	r8, [sp, #-4]!
 8003c48: b0aa         	sub	sp, #0xa8
 8003c4a: f240 610a    	movw	r1, #0x60a
 8003c4e: f2c0 1101    	movt	r1, #0x101
 8003c52: 9129         	str	r1, [sp, #0xa4]
 8003c54: f44f 0100    	mov.w	r1, #0x800000
 8003c58: 9128         	str	r1, [sp, #0xa0]
 8003c5a: f44f 6180    	mov.w	r1, #0x400
 8003c5e: 9127         	str	r1, [sp, #0x9c]
 8003c60: a927         	add	r1, sp, #0x9c
 8003c62: 9927         	ldr	r1, [sp, #0x9c]
 8003c64: 1e4a         	subs	r2, r1, #0x1
 8003c66: ea81 0302    	eor.w	r3, r1, r2
 8003c6a: 4293         	cmp	r3, r2
 8003c6c: d916         	bls	0x8003c9c <raylar_crcl_encoder_service::benchmark::transform_production::he43415fd0806c08a+0x5c> @ imm = #0x2c
 8003c6e: fa91 f2a1    	rbit	r2, r1
 8003c72: fab2 f282    	clz	r2, r2
 8003c76: 3a08         	subs	r2, #0x8
 8003c78: 2a04         	cmp	r2, #0x4
 8003c7a: bf9c         	itt	ls
 8003c7c: f89d 20a4    	ldrbls.w	r2, [sp, #0xa4]
 8003c80: 2a0c         	cmpls	r2, #0xc
 8003c82: d80b         	bhi	0x8003c9c <raylar_crcl_encoder_service::benchmark::transform_production::he43415fd0806c08a+0x5c> @ imm = #0x16
 8003c84: 2301         	movs	r3, #0x1
 8003c86: fa03 f202    	lsl.w	r2, r3, r2
 8003c8a: f412 5faa    	tst.w	r2, #0x1540
 8003c8e: bf1e         	ittt	ne
 8003c90: 9a28         	ldrne	r2, [sp, #0xa0]
 8003c92: f102 4240    	addne.w	r2, r2, #0xc0000000
 8003c96: f1b2 4f40    	cmpne.w	r2, #0xc0000000
 8003c9a: d813         	bhi	0x8003cc4 <raylar_crcl_encoder_service::benchmark::transform_production::he43415fd0806c08a+0x84> @ imm = #0x26
 8003c9c: 2000         	movs	r0, #0x0
 8003c9e: f24e 539c    	movw	r3, #0xe59c
 8003ca2: f88d 0008    	strb.w	r0, [sp, #0x8]
 8003ca6: f24e 7078    	movw	r0, #0xe778
 8003caa: f6c0 0000    	movt	r0, #0x800
 8003cae: aa02         	add	r2, sp, #0x8
 8003cb0: 9000         	str	r0, [sp]
 8003cb2: f24e 5000    	movw	r0, #0xe500
 8003cb6: f6c0 0000    	movt	r0, #0x800
 8003cba: f6c0 0300    	movt	r3, #0x800
 8003cbe: 212b         	movs	r1, #0x2b
 8003cc0: f008 f8f2    	bl	0x800bea8 <core::result::unwrap_failed> @ imm = #0x81e4
 8003cc4: f89d 20a5    	ldrb.w	r2, [sp, #0xa5]
 8003cc8: 2a1e         	cmp	r2, #0x1e
 8003cca: d8e7         	bhi	0x8003c9c <raylar_crcl_encoder_service::benchmark::transform_production::he43415fd0806c08a+0x5c> @ imm = #-0x32
 8003ccc: f89d 30a7    	ldrb.w	r3, [sp, #0xa7]
 8003cd0: 2b01         	cmp	r3, #0x1
 8003cd2: d107         	bne	0x8003ce4 <raylar_crcl_encoder_service::benchmark::transform_production::he43415fd0806c08a+0xa4> @ imm = #0xe
 8003cd4: f5b1 6f80    	cmp.w	r1, #0x400
 8003cd8: d3e0         	blo	0x8003c9c <raylar_crcl_encoder_service::benchmark::transform_production::he43415fd0806c08a+0x5c> @ imm = #-0x40
 8003cda: f89d 10a6    	ldrb.w	r1, [sp, #0xa6]
 8003cde: 2901         	cmp	r1, #0x1
 8003ce0: d005         	beq	0x8003cee <raylar_crcl_encoder_service::benchmark::transform_production::he43415fd0806c08a+0xae> @ imm = #0xa
 8003ce2: e7db         	b	0x8003c9c <raylar_crcl_encoder_service::benchmark::transform_production::he43415fd0806c08a+0x5c> @ imm = #-0x4a
 8003ce4: f5b1 6f80    	cmp.w	r1, #0x400
 8003ce8: d3d8         	blo	0x8003c9c <raylar_crcl_encoder_service::benchmark::transform_production::he43415fd0806c08a+0x5c> @ imm = #-0x50
 8003cea: 2a00         	cmp	r2, #0x0
 8003cec: d1d6         	bne	0x8003c9c <raylar_crcl_encoder_service::benchmark::transform_production::he43415fd0806c08a+0x5c> @ imm = #-0x54
 8003cee: ac02         	add	r4, sp, #0x8
 8003cf0: f246 0128    	movw	r1, #0x6028
 8003cf4: 1845         	adds	r5, r0, r1
 8003cf6: f104 0048    	add.w	r0, r4, #0x48
 8003cfa: 2128         	movs	r1, #0x28
 8003cfc: f009 fb39    	bl	0x800d372 <__aeabi_memclr4> @ imm = #0x9672
 8003d00: ab27         	add	r3, sp, #0x9c
 8003d02: 9512         	str	r5, [sp, #0x48]
 8003d04: f44f 56c0    	mov.w	r6, #0x1800
 8003d08: f104 0840    	add.w	r8, r4, #0x40
 8003d0c: cb0e         	ldm	r3, {r1, r2, r3}
 8003d0e: ad27         	add	r5, sp, #0x9c
 8003d10: 2000         	movs	r0, #0x0
 8003d12: 9613         	str	r6, [sp, #0x4c]
 8003d14: f04f 36ff    	mov.w	r6, #0xffffffff
 8003d18: e9cd 2324    	strd	r2, r3, [sp, #144]
 8003d1c: 2201         	movs	r2, #0x1
 8003d1e: f8ad 0024    	strh.w	r0, [sp, #0x24]
 8003d22: e9cd 0007    	strd	r0, r0, [sp, #28]
 8003d26: e9cd 0005    	strd	r0, r0, [sp, #20]
 8003d2a: e9cd 0003    	strd	r0, r0, [sp, #12]
 8003d2e: e9cd 000b    	strd	r0, r0, [sp, #44]
 8003d32: e9cd 000d    	strd	r0, r0, [sp, #52]
 8003d36: e9cd 000f    	strd	r0, r0, [sp, #60]
 8003d3a: f8ad 0044    	strh.w	r0, [sp, #0x44]
 8003d3e: e9cd 001f    	strd	r0, r0, [sp, #124]
 8003d42: e9cd 0021    	strd	r0, r0, [sp, #132]
 8003d46: 9002         	str	r0, [sp, #0x8]
 8003d48: 900a         	str	r0, [sp, #0x28]
 8003d4a: 921e         	str	r2, [sp, #0x78]
 8003d4c: 9123         	str	r1, [sp, #0x8c]
 8003d4e: f8ad 0098    	strh.w	r0, [sp, #0x98]
 8003d52: 9627         	str	r6, [sp, #0x9c]
 8003d54: 4620         	mov	r0, r4
 8003d56: 9927         	ldr	r1, [sp, #0x9c]
 8003d58: f7fd ff49    	bl	0x8001bee <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3> @ imm = #-0x216e
 8003d5c: 07c0         	lsls	r0, r0, #0x1f
 8003d5e: d109         	bne	0x8003d74 <raylar_crcl_encoder_service::benchmark::transform_production::he43415fd0806c08a+0x134> @ imm = #0x12
 8003d60: 07c8         	lsls	r0, r1, #0x1f
 8003d62: d0f6         	beq	0x8003d52 <raylar_crcl_encoder_service::benchmark::transform_production::he43415fd0806c08a+0x112> @ imm = #-0x14
 8003d64: f8cd 809c    	str.w	r8, [sp, #0x9c]
 8003d68: a827         	add	r0, sp, #0x9c
 8003d6a: 9814         	ldr	r0, [sp, #0x50]
 8003d6c: b02a         	add	sp, #0xa8
 8003d6e: f85d 8b04    	ldr	r8, [sp], #4
 8003d72: bdf0         	pop	{r4, r5, r6, r7, pc}
 8003d74: f24e 7068    	movw	r0, #0xe768
 8003d78: f24e 539c    	movw	r3, #0xe59c
 8003d7c: f6c0 0000    	movt	r0, #0x800
 8003d80: aa27         	add	r2, sp, #0x9c
 8003d82: 9000         	str	r0, [sp]
 8003d84: f24e 5000    	movw	r0, #0xe500
 8003d88: f88d 109c    	strb.w	r1, [sp, #0x9c]
 8003d8c: f6c0 0000    	movt	r0, #0x800
 8003d90: f6c0 0300    	movt	r3, #0x800
 8003d94: 212b         	movs	r1, #0x2b
 8003d96: f008 f887    	bl	0x800bea8 <core::result::unwrap_failed> @ imm = #0x810e

08003d9a <raylar_crcl_encoder_service::benchmark::raw::hd2aad415d886ff2c>:
 8003d9a: b580         	push	{r7, lr}
 8003d9c: 466f         	mov	r7, sp
 8003d9e: f246 0128    	movw	r1, #0x6028
 8003da2: 1842         	adds	r2, r0, r1
 8003da4: f100 0128    	add.w	r1, r0, #0x28
 8003da8: 4610         	mov	r0, r2
 8003daa: f44f 42c0    	mov.w	r2, #0x6000
 8003dae: f009 fa92    	bl	0x800d2d6 <__aeabi_memcpy4> @ imm = #0x9524
 8003db2: bd80         	pop	{r7, pc}

08003db4 <raylar_crcl_encoder_service::benchmark::copy::hf53d4786c081c67d>:
 8003db4: b5f0         	push	{r4, r5, r6, r7, lr}
 8003db6: af03         	add	r7, sp, #0xc
 8003db8: f84d bd04    	str	r11, [sp, #-4]!
 8003dbc: b082         	sub	sp, #0x8
 8003dbe: f246 0628    	movw	r6, #0x6028
 8003dc2: 1985         	adds	r5, r0, r6
 8003dc4: f100 0128    	add.w	r1, r0, #0x28
 8003dc8: 4604         	mov	r4, r0
 8003dca: 4628         	mov	r0, r5
 8003dcc: f44f 42c0    	mov.w	r2, #0x6000
 8003dd0: f009 fa81    	bl	0x800d2d6 <__aeabi_memcpy4> @ imm = #0x9502
 8003dd4: 9501         	str	r5, [sp, #0x4]
 8003dd6: a801         	add	r0, sp, #0x4
 8003dd8: 59a0         	ldr	r0, [r4, r6]
 8003dda: b002         	add	sp, #0x8
 8003ddc: f85d bb04    	ldr	r11, [sp], #4
 8003de0: bdf0         	pop	{r4, r5, r6, r7, pc}

08003de2 <raylar_crcl_encoder_service::benchmark::peak::he0ae015b97d7ff8e>:
 8003de2: b5f0         	push	{r4, r5, r6, r7, lr}
 8003de4: af03         	add	r7, sp, #0xc
 8003de6: e92d 0f00    	push.w	{r8, r9, r10, r11}
 8003dea: b087         	sub	sp, #0x1c
 8003dec: f04f 0a00    	mov.w	r10, #0x0
 8003df0: f46f 0900    	mvn	r9, #0x800000
 8003df4: f06f 7880    	mvn	r8, #0x1000000
 8003df8: f04f 0b00    	mov.w	r11, #0x0
 8003dfc: f246 018c    	movw	r1, #0x608c
 8003e00: 9004         	str	r0, [sp, #0x10]
 8003e02: 4408         	add	r0, r1
 8003e04: 9005         	str	r0, [sp, #0x14]
 8003e06: 9805         	ldr	r0, [sp, #0x14]
 8003e08: f64f 72fe    	movw	r2, #0xfffe
 8003e0c: f6cf 62ff    	movt	r2, #0xfeff
 8003e10: eb00 060b    	add.w	r6, r0, r11
 8003e14: f856 0c60    	ldr	r0, [r6, #-96]
 8003e18: eb00 0109    	add.w	r1, r0, r9
 8003e1c: 4291         	cmp	r1, r2
 8003e1e: f240 8281    	bls.w	0x8004324 <raylar_crcl_encoder_service::benchmark::peak::he0ae015b97d7ff8e+0x542> @ imm = #0x502
 8003e22: f856 1c48    	ldr	r1, [r6, #-72]
 8003e26: eb01 0209    	add.w	r2, r1, r9
 8003e2a: 4542         	cmp	r2, r8
 8003e2c: f0c0 827a    	blo.w	0x8004324 <raylar_crcl_encoder_service::benchmark::peak::he0ae015b97d7ff8e+0x542> @ imm = #0x4f4
 8003e30: f856 5c30    	ldr	r5, [r6, #-48]
 8003e34: eb05 0209    	add.w	r2, r5, r9
 8003e38: 4542         	cmp	r2, r8
 8003e3a: bf22         	ittt	hs
 8003e3c: f856 2c18    	ldrhs	r2, [r6, #-24]
 8003e40: eb02 0309    	addhs.w	r3, r2, r9
 8003e44: 4543         	cmphs	r3, r8
 8003e46: f0c0 826d    	blo.w	0x8004324 <raylar_crcl_encoder_service::benchmark::peak::he0ae015b97d7ff8e+0x542> @ imm = #0x4da
 8003e4a: 9b05         	ldr	r3, [sp, #0x14]
 8003e4c: f853 300b    	ldr.w	r3, [r3, r11]
 8003e50: eb03 0409    	add.w	r4, r3, r9
 8003e54: 4544         	cmp	r4, r8
 8003e56: bf22         	ittt	hs
 8003e58: f8d6 e018    	ldrhs.w	lr, [r6, #0x18]
 8003e5c: eb0e 0409    	addhs.w	r4, lr, r9
 8003e60: 4544         	cmphs	r4, r8
 8003e62: f0c0 825f    	blo.w	0x8004324 <raylar_crcl_encoder_service::benchmark::peak::he0ae015b97d7ff8e+0x542> @ imm = #0x4be
 8003e66: f8d6 c030    	ldr.w	r12, [r6, #0x30]
 8003e6a: eb0c 0409    	add.w	r4, r12, r9
 8003e6e: 4544         	cmp	r4, r8
 8003e70: bf22         	ittt	hs
 8003e72: 6cb6         	ldrhs	r6, [r6, #0x48]
 8003e74: eb06 0409    	addhs.w	r4, r6, r9
 8003e78: 4544         	cmphs	r4, r8
 8003e7a: f0c0 8253    	blo.w	0x8004324 <raylar_crcl_encoder_service::benchmark::peak::he0ae015b97d7ff8e+0x542> @ imm = #0x4a6
 8003e7e: 2800         	cmp	r0, #0x0
 8003e80: bf48         	it	mi
 8003e82: 4240         	rsbmi	r0, r0, #0
 8003e84: 4550         	cmp	r0, r10
 8003e86: bf98         	it	ls
 8003e88: 4650         	movls	r0, r10
 8003e8a: 2900         	cmp	r1, #0x0
 8003e8c: bf48         	it	mi
 8003e8e: 4249         	rsbmi	r1, r1, #0
 8003e90: 4281         	cmp	r1, r0
 8003e92: bf98         	it	ls
 8003e94: 4601         	movls	r1, r0
 8003e96: 2d00         	cmp	r5, #0x0
 8003e98: bf48         	it	mi
 8003e9a: 426d         	rsbmi	r5, r5, #0
 8003e9c: 428d         	cmp	r5, r1
 8003e9e: bf98         	it	ls
 8003ea0: 460d         	movls	r5, r1
 8003ea2: 2a00         	cmp	r2, #0x0
 8003ea4: bf48         	it	mi
 8003ea6: 4252         	rsbmi	r2, r2, #0
 8003ea8: 42aa         	cmp	r2, r5
 8003eaa: bf98         	it	ls
 8003eac: 462a         	movls	r2, r5
 8003eae: 2b00         	cmp	r3, #0x0
 8003eb0: bf48         	it	mi
 8003eb2: 425b         	rsbmi	r3, r3, #0
 8003eb4: 4293         	cmp	r3, r2
 8003eb6: bf98         	it	ls
 8003eb8: 4613         	movls	r3, r2
 8003eba: f1be 0f00    	cmp.w	lr, #0x0
 8003ebe: bf48         	it	mi
 8003ec0: f1ce 0e00    	rsbmi.w	lr, lr, #0x0
 8003ec4: 459e         	cmp	lr, r3
 8003ec6: bf98         	it	ls
 8003ec8: 469e         	movls	lr, r3
 8003eca: f1bc 0f00    	cmp.w	r12, #0x0
 8003ece: bf48         	it	mi
 8003ed0: f1cc 0c00    	rsbmi.w	r12, r12, #0x0
 8003ed4: 45f4         	cmp	r12, lr
 8003ed6: bf98         	it	ls
 8003ed8: 46f4         	movls	r12, lr
 8003eda: 2e00         	cmp	r6, #0x0
 8003edc: bf48         	it	mi
 8003ede: 4276         	rsbmi	r6, r6, #0
 8003ee0: f10b 0bc0    	add.w	r11, r11, #0xc0
 8003ee4: 4566         	cmp	r6, r12
 8003ee6: bf98         	it	ls
 8003ee8: 4666         	movls	r6, r12
 8003eea: 46b2         	mov	r10, r6
 8003eec: f5bb 4fc0    	cmp.w	r11, #0x6000
 8003ef0: d189         	bne	0x8003e06 <raylar_crcl_encoder_service::benchmark::peak::he0ae015b97d7ff8e+0x24> @ imm = #-0xee
 8003ef2: 9606         	str	r6, [sp, #0x18]
 8003ef4: a806         	add	r0, sp, #0x18
 8003ef6: f246 0090    	movw	r0, #0x6090
 8003efa: f04f 0900    	mov.w	r9, #0x0
 8003efe: 9906         	ldr	r1, [sp, #0x18]
 8003f00: f46f 0e00    	mvn	lr, #0x800000
 8003f04: 9103         	str	r1, [sp, #0xc]
 8003f06: f06f 7880    	mvn	r8, #0x1000000
 8003f0a: 9904         	ldr	r1, [sp, #0x10]
 8003f0c: f04f 0a00    	mov.w	r10, #0x0
 8003f10: 4408         	add	r0, r1
 8003f12: 9005         	str	r0, [sp, #0x14]
 8003f14: 9805         	ldr	r0, [sp, #0x14]
 8003f16: eb00 060a    	add.w	r6, r0, r10
 8003f1a: f856 0c60    	ldr	r0, [r6, #-96]
 8003f1e: eb00 010e    	add.w	r1, r0, lr
 8003f22: 4541         	cmp	r1, r8
 8003f24: bf22         	ittt	hs
 8003f26: f856 1c48    	ldrhs	r1, [r6, #-72]
 8003f2a: eb01 020e    	addhs.w	r2, r1, lr
 8003f2e: 4542         	cmphs	r2, r8
 8003f30: f0c0 81f8    	blo.w	0x8004324 <raylar_crcl_encoder_service::benchmark::peak::he0ae015b97d7ff8e+0x542> @ imm = #0x3f0
 8003f34: f856 2c30    	ldr	r2, [r6, #-48]
 8003f38: eb02 030e    	add.w	r3, r2, lr
 8003f3c: 4543         	cmp	r3, r8
 8003f3e: bf22         	ittt	hs
 8003f40: f856 3c18    	ldrhs	r3, [r6, #-24]
 8003f44: eb03 040e    	addhs.w	r4, r3, lr
 8003f48: 4544         	cmphs	r4, r8
 8003f4a: f0c0 81eb    	blo.w	0x8004324 <raylar_crcl_encoder_service::benchmark::peak::he0ae015b97d7ff8e+0x542> @ imm = #0x3d6
 8003f4e: 9d05         	ldr	r5, [sp, #0x14]
 8003f50: f855 400a    	ldr.w	r4, [r5, r10]
 8003f54: eb04 050e    	add.w	r5, r4, lr
 8003f58: 4545         	cmp	r5, r8
 8003f5a: bf22         	ittt	hs
 8003f5c: f8d6 c018    	ldrhs.w	r12, [r6, #0x18]
 8003f60: eb0c 050e    	addhs.w	r5, r12, lr
 8003f64: 4545         	cmphs	r5, r8
 8003f66: f0c0 81dd    	blo.w	0x8004324 <raylar_crcl_encoder_service::benchmark::peak::he0ae015b97d7ff8e+0x542> @ imm = #0x3ba
 8003f6a: f8d6 b030    	ldr.w	r11, [r6, #0x30]
 8003f6e: eb0b 050e    	add.w	r5, r11, lr
 8003f72: 4545         	cmp	r5, r8
 8003f74: bf22         	ittt	hs
 8003f76: 6cb6         	ldrhs	r6, [r6, #0x48]
 8003f78: eb06 050e    	addhs.w	r5, r6, lr
 8003f7c: 4545         	cmphs	r5, r8
 8003f7e: f0c0 81d1    	blo.w	0x8004324 <raylar_crcl_encoder_service::benchmark::peak::he0ae015b97d7ff8e+0x542> @ imm = #0x3a2
 8003f82: 2800         	cmp	r0, #0x0
 8003f84: bf48         	it	mi
 8003f86: 4240         	rsbmi	r0, r0, #0
 8003f88: 4548         	cmp	r0, r9
 8003f8a: bf98         	it	ls
 8003f8c: 4648         	movls	r0, r9
 8003f8e: 2900         	cmp	r1, #0x0
 8003f90: bf48         	it	mi
 8003f92: 4249         	rsbmi	r1, r1, #0
 8003f94: 4281         	cmp	r1, r0
 8003f96: bf98         	it	ls
 8003f98: 4601         	movls	r1, r0
 8003f9a: 2a00         	cmp	r2, #0x0
 8003f9c: bf48         	it	mi
 8003f9e: 4252         	rsbmi	r2, r2, #0
 8003fa0: 428a         	cmp	r2, r1
 8003fa2: bf98         	it	ls
 8003fa4: 460a         	movls	r2, r1
 8003fa6: 2b00         	cmp	r3, #0x0
 8003fa8: bf48         	it	mi
 8003faa: 425b         	rsbmi	r3, r3, #0
 8003fac: 4293         	cmp	r3, r2
 8003fae: bf98         	it	ls
 8003fb0: 4613         	movls	r3, r2
 8003fb2: 2c00         	cmp	r4, #0x0
 8003fb4: bf48         	it	mi
 8003fb6: 4264         	rsbmi	r4, r4, #0
 8003fb8: 429c         	cmp	r4, r3
 8003fba: bf98         	it	ls
 8003fbc: 461c         	movls	r4, r3
 8003fbe: f1bc 0f00    	cmp.w	r12, #0x0
 8003fc2: bf48         	it	mi
 8003fc4: f1cc 0c00    	rsbmi.w	r12, r12, #0x0
 8003fc8: 45a4         	cmp	r12, r4
 8003fca: bf98         	it	ls
 8003fcc: 46a4         	movls	r12, r4
 8003fce: f1bb 0f00    	cmp.w	r11, #0x0
 8003fd2: bf48         	it	mi
 8003fd4: f1cb 0b00    	rsbmi.w	r11, r11, #0x0
 8003fd8: 45e3         	cmp	r11, r12
 8003fda: bf98         	it	ls
 8003fdc: 46e3         	movls	r11, r12
 8003fde: 2e00         	cmp	r6, #0x0
 8003fe0: bf48         	it	mi
 8003fe2: 4276         	rsbmi	r6, r6, #0
 8003fe4: f10a 0ac0    	add.w	r10, r10, #0xc0
 8003fe8: 455e         	cmp	r6, r11
 8003fea: bf98         	it	ls
 8003fec: 465e         	movls	r6, r11
 8003fee: 46b1         	mov	r9, r6
 8003ff0: f5ba 4fc0    	cmp.w	r10, #0x6000
 8003ff4: d18e         	bne	0x8003f14 <raylar_crcl_encoder_service::benchmark::peak::he0ae015b97d7ff8e+0x132> @ imm = #-0xe4
 8003ff6: 9606         	str	r6, [sp, #0x18]
 8003ff8: a806         	add	r0, sp, #0x18
 8003ffa: f246 0094    	movw	r0, #0x6094
 8003ffe: f04f 0900    	mov.w	r9, #0x0
 8004002: 9906         	ldr	r1, [sp, #0x18]
 8004004: f46f 0e00    	mvn	lr, #0x800000
 8004008: 9102         	str	r1, [sp, #0x8]
 800400a: f06f 7880    	mvn	r8, #0x1000000
 800400e: 9904         	ldr	r1, [sp, #0x10]
 8004010: f04f 0a00    	mov.w	r10, #0x0
 8004014: 4408         	add	r0, r1
 8004016: 9005         	str	r0, [sp, #0x14]
 8004018: 9805         	ldr	r0, [sp, #0x14]
 800401a: eb00 060a    	add.w	r6, r0, r10
 800401e: f856 0c60    	ldr	r0, [r6, #-96]
 8004022: eb00 010e    	add.w	r1, r0, lr
 8004026: 4541         	cmp	r1, r8
 8004028: bf22         	ittt	hs
 800402a: f856 1c48    	ldrhs	r1, [r6, #-72]
 800402e: eb01 020e    	addhs.w	r2, r1, lr
 8004032: 4542         	cmphs	r2, r8
 8004034: f0c0 8176    	blo.w	0x8004324 <raylar_crcl_encoder_service::benchmark::peak::he0ae015b97d7ff8e+0x542> @ imm = #0x2ec
 8004038: f856 2c30    	ldr	r2, [r6, #-48]
 800403c: eb02 030e    	add.w	r3, r2, lr
 8004040: 4543         	cmp	r3, r8
 8004042: bf22         	ittt	hs
 8004044: f856 3c18    	ldrhs	r3, [r6, #-24]
 8004048: eb03 040e    	addhs.w	r4, r3, lr
 800404c: 4544         	cmphs	r4, r8
 800404e: f0c0 8169    	blo.w	0x8004324 <raylar_crcl_encoder_service::benchmark::peak::he0ae015b97d7ff8e+0x542> @ imm = #0x2d2
 8004052: 9d05         	ldr	r5, [sp, #0x14]
 8004054: f855 500a    	ldr.w	r5, [r5, r10]
 8004058: eb05 040e    	add.w	r4, r5, lr
 800405c: 4544         	cmp	r4, r8
 800405e: bf22         	ittt	hs
 8004060: f8d6 c018    	ldrhs.w	r12, [r6, #0x18]
 8004064: eb0c 040e    	addhs.w	r4, r12, lr
 8004068: 4544         	cmphs	r4, r8
 800406a: f0c0 815b    	blo.w	0x8004324 <raylar_crcl_encoder_service::benchmark::peak::he0ae015b97d7ff8e+0x542> @ imm = #0x2b6
 800406e: f8d6 b030    	ldr.w	r11, [r6, #0x30]
 8004072: eb0b 040e    	add.w	r4, r11, lr
 8004076: 4544         	cmp	r4, r8
 8004078: bf22         	ittt	hs
 800407a: 6cb6         	ldrhs	r6, [r6, #0x48]
 800407c: eb06 040e    	addhs.w	r4, r6, lr
 8004080: 4544         	cmphs	r4, r8
 8004082: f0c0 814f    	blo.w	0x8004324 <raylar_crcl_encoder_service::benchmark::peak::he0ae015b97d7ff8e+0x542> @ imm = #0x29e
 8004086: 2800         	cmp	r0, #0x0
 8004088: bf48         	it	mi
 800408a: 4240         	rsbmi	r0, r0, #0
 800408c: 4548         	cmp	r0, r9
 800408e: bf98         	it	ls
 8004090: 4648         	movls	r0, r9
 8004092: 2900         	cmp	r1, #0x0
 8004094: bf48         	it	mi
 8004096: 4249         	rsbmi	r1, r1, #0
 8004098: 4281         	cmp	r1, r0
 800409a: bf98         	it	ls
 800409c: 4601         	movls	r1, r0
 800409e: 2a00         	cmp	r2, #0x0
 80040a0: bf48         	it	mi
 80040a2: 4252         	rsbmi	r2, r2, #0
 80040a4: 428a         	cmp	r2, r1
 80040a6: bf98         	it	ls
 80040a8: 460a         	movls	r2, r1
 80040aa: 2b00         	cmp	r3, #0x0
 80040ac: bf48         	it	mi
 80040ae: 425b         	rsbmi	r3, r3, #0
 80040b0: 4293         	cmp	r3, r2
 80040b2: bf98         	it	ls
 80040b4: 4613         	movls	r3, r2
 80040b6: 2d00         	cmp	r5, #0x0
 80040b8: bf48         	it	mi
 80040ba: 426d         	rsbmi	r5, r5, #0
 80040bc: 429d         	cmp	r5, r3
 80040be: bf98         	it	ls
 80040c0: 461d         	movls	r5, r3
 80040c2: f1bc 0f00    	cmp.w	r12, #0x0
 80040c6: bf48         	it	mi
 80040c8: f1cc 0c00    	rsbmi.w	r12, r12, #0x0
 80040cc: 45ac         	cmp	r12, r5
 80040ce: bf98         	it	ls
 80040d0: 46ac         	movls	r12, r5
 80040d2: f1bb 0f00    	cmp.w	r11, #0x0
 80040d6: bf48         	it	mi
 80040d8: f1cb 0b00    	rsbmi.w	r11, r11, #0x0
 80040dc: 45e3         	cmp	r11, r12
 80040de: bf98         	it	ls
 80040e0: 46e3         	movls	r11, r12
 80040e2: 2e00         	cmp	r6, #0x0
 80040e4: bf48         	it	mi
 80040e6: 4276         	rsbmi	r6, r6, #0
 80040e8: f10a 0ac0    	add.w	r10, r10, #0xc0
 80040ec: 455e         	cmp	r6, r11
 80040ee: bf98         	it	ls
 80040f0: 465e         	movls	r6, r11
 80040f2: 46b1         	mov	r9, r6
 80040f4: f5ba 4fc0    	cmp.w	r10, #0x6000
 80040f8: d18e         	bne	0x8004018 <raylar_crcl_encoder_service::benchmark::peak::he0ae015b97d7ff8e+0x236> @ imm = #-0xe4
 80040fa: 9606         	str	r6, [sp, #0x18]
 80040fc: a806         	add	r0, sp, #0x18
 80040fe: f246 00e0    	movw	r0, #0x60e0
 8004102: f44f 6a80    	mov.w	r10, #0x400
 8004106: 9906         	ldr	r1, [sp, #0x18]
 8004108: f46f 0b00    	mvn	r11, #0x800000
 800410c: 9101         	str	r1, [sp, #0x4]
 800410e: f06f 7380    	mvn	r3, #0x1000000
 8004112: 9904         	ldr	r1, [sp, #0x10]
 8004114: 4401         	add	r1, r0
 8004116: 2000         	movs	r0, #0x0
 8004118: f851 4ca8    	ldr	r4, [r1, #-168]
 800411c: eb04 020b    	add.w	r2, r4, r11
 8004120: 429a         	cmp	r2, r3
 8004122: f0c0 80ff    	blo.w	0x8004324 <raylar_crcl_encoder_service::benchmark::peak::he0ae015b97d7ff8e+0x542> @ imm = #0x1fe
 8004126: f851 5c90    	ldr	r5, [r1, #-144]
 800412a: eb05 020b    	add.w	r2, r5, r11
 800412e: 429a         	cmp	r2, r3
 8004130: bf22         	ittt	hs
 8004132: f851 2c78    	ldrhs	r2, [r1, #-120]
 8004136: eb02 060b    	addhs.w	r6, r2, r11
 800413a: 429e         	cmphs	r6, r3
 800413c: f0c0 80f2    	blo.w	0x8004324 <raylar_crcl_encoder_service::benchmark::peak::he0ae015b97d7ff8e+0x542> @ imm = #0x1e4
 8004140: f851 cc60    	ldr	r12, [r1, #-96]
 8004144: eb0c 060b    	add.w	r6, r12, r11
 8004148: 429e         	cmp	r6, r3
 800414a: bf22         	ittt	hs
 800414c: f851 ec48    	ldrhs	lr, [r1, #-72]
 8004150: eb0e 060b    	addhs.w	r6, lr, r11
 8004154: 429e         	cmphs	r6, r3
 8004156: f0c0 80e5    	blo.w	0x8004324 <raylar_crcl_encoder_service::benchmark::peak::he0ae015b97d7ff8e+0x542> @ imm = #0x1ca
 800415a: f851 8c30    	ldr	r8, [r1, #-48]
 800415e: eb08 060b    	add.w	r6, r8, r11
 8004162: 429e         	cmp	r6, r3
 8004164: bf22         	ittt	hs
 8004166: f851 9c18    	ldrhs	r9, [r1, #-24]
 800416a: eb09 060b    	addhs.w	r6, r9, r11
 800416e: 429e         	cmphs	r6, r3
 8004170: f0c0 80d8    	blo.w	0x8004324 <raylar_crcl_encoder_service::benchmark::peak::he0ae015b97d7ff8e+0x542> @ imm = #0x1b0
 8004174: 680e         	ldr	r6, [r1]
 8004176: f46f 0b00    	mvn	r11, #0x800000
 800417a: f8cd a014    	str.w	r10, [sp, #0x14]
 800417e: 4682         	mov	r10, r0
 8004180: eb06 000b    	add.w	r0, r6, r11
 8004184: 4298         	cmp	r0, r3
 8004186: 4650         	mov	r0, r10
 8004188: f8dd a014    	ldr.w	r10, [sp, #0x14]
 800418c: f0c0 80ca    	blo.w	0x8004324 <raylar_crcl_encoder_service::benchmark::peak::he0ae015b97d7ff8e+0x542> @ imm = #0x194
 8004190: 2c00         	cmp	r4, #0x0
 8004192: bf48         	it	mi
 8004194: 4264         	rsbmi	r4, r4, #0
 8004196: 4284         	cmp	r4, r0
 8004198: bf98         	it	ls
 800419a: 4604         	movls	r4, r0
 800419c: 2d00         	cmp	r5, #0x0
 800419e: bf48         	it	mi
 80041a0: 426d         	rsbmi	r5, r5, #0
 80041a2: 42a5         	cmp	r5, r4
 80041a4: bf98         	it	ls
 80041a6: 4625         	movls	r5, r4
 80041a8: 2a00         	cmp	r2, #0x0
 80041aa: bf48         	it	mi
 80041ac: 4252         	rsbmi	r2, r2, #0
 80041ae: 42aa         	cmp	r2, r5
 80041b0: bf98         	it	ls
 80041b2: 462a         	movls	r2, r5
 80041b4: f1bc 0f00    	cmp.w	r12, #0x0
 80041b8: bf48         	it	mi
 80041ba: f1cc 0c00    	rsbmi.w	r12, r12, #0x0
 80041be: 4594         	cmp	r12, r2
 80041c0: bf98         	it	ls
 80041c2: 4694         	movls	r12, r2
 80041c4: f1be 0f00    	cmp.w	lr, #0x0
 80041c8: bf48         	it	mi
 80041ca: f1ce 0e00    	rsbmi.w	lr, lr, #0x0
 80041ce: 45e6         	cmp	lr, r12
 80041d0: bf98         	it	ls
 80041d2: 46e6         	movls	lr, r12
 80041d4: f1b8 0f00    	cmp.w	r8, #0x0
 80041d8: bf48         	it	mi
 80041da: f1c8 0800    	rsbmi.w	r8, r8, #0x0
 80041de: 45f0         	cmp	r8, lr
 80041e0: bf98         	it	ls
 80041e2: 46f0         	movls	r8, lr
 80041e4: f1b9 0f00    	cmp.w	r9, #0x0
 80041e8: bf48         	it	mi
 80041ea: f1c9 0900    	rsbmi.w	r9, r9, #0x0
 80041ee: 45c1         	cmp	r9, r8
 80041f0: bf98         	it	ls
 80041f2: 46c1         	movls	r9, r8
 80041f4: 2e00         	cmp	r6, #0x0
 80041f6: bf48         	it	mi
 80041f8: 4276         	rsbmi	r6, r6, #0
 80041fa: 454e         	cmp	r6, r9
 80041fc: bf98         	it	ls
 80041fe: 464e         	movls	r6, r9
 8004200: 31c0         	adds	r1, #0xc0
 8004202: f1ba 0a08    	subs.w	r10, r10, #0x8
 8004206: 4630         	mov	r0, r6
 8004208: d186         	bne	0x8004118 <raylar_crcl_encoder_service::benchmark::peak::he0ae015b97d7ff8e+0x336> @ imm = #-0xf4
 800420a: 9606         	str	r6, [sp, #0x18]
 800420c: a806         	add	r0, sp, #0x18
 800420e: f246 009c    	movw	r0, #0x609c
 8004212: f04f 0a00    	mov.w	r10, #0x0
 8004216: 9906         	ldr	r1, [sp, #0x18]
 8004218: f46f 0800    	mvn	r8, #0x800000
 800421c: 9100         	str	r1, [sp]
 800421e: f06f 7980    	mvn	r9, #0x1000000
 8004222: 9904         	ldr	r1, [sp, #0x10]
 8004224: f04f 0b00    	mov.w	r11, #0x0
 8004228: 4408         	add	r0, r1
 800422a: 9005         	str	r0, [sp, #0x14]
 800422c: 9805         	ldr	r0, [sp, #0x14]
 800422e: eb00 030b    	add.w	r3, r0, r11
 8004232: f853 1c60    	ldr	r1, [r3, #-96]
 8004236: eb01 0008    	add.w	r0, r1, r8
 800423a: 4548         	cmp	r0, r9
 800423c: bf22         	ittt	hs
 800423e: f853 0c48    	ldrhs	r0, [r3, #-72]
 8004242: eb00 0208    	addhs.w	r2, r0, r8
 8004246: 454a         	cmphs	r2, r9
 8004248: d36c         	blo	0x8004324 <raylar_crcl_encoder_service::benchmark::peak::he0ae015b97d7ff8e+0x542> @ imm = #0xd8
 800424a: f853 2c30    	ldr	r2, [r3, #-48]
 800424e: eb02 0608    	add.w	r6, r2, r8
 8004252: 454e         	cmp	r6, r9
 8004254: bf22         	ittt	hs
 8004256: f853 4c18    	ldrhs	r4, [r3, #-24]
 800425a: eb04 0608    	addhs.w	r6, r4, r8
 800425e: 454e         	cmphs	r6, r9
 8004260: d360         	blo	0x8004324 <raylar_crcl_encoder_service::benchmark::peak::he0ae015b97d7ff8e+0x542> @ imm = #0xc0
 8004262: 9e05         	ldr	r6, [sp, #0x14]
 8004264: f856 600b    	ldr.w	r6, [r6, r11]
 8004268: eb06 0508    	add.w	r5, r6, r8
 800426c: 454d         	cmp	r5, r9
 800426e: bf22         	ittt	hs
 8004270: f8d3 e018    	ldrhs.w	lr, [r3, #0x18]
 8004274: eb0e 0508    	addhs.w	r5, lr, r8
 8004278: 454d         	cmphs	r5, r9
 800427a: d353         	blo	0x8004324 <raylar_crcl_encoder_service::benchmark::peak::he0ae015b97d7ff8e+0x542> @ imm = #0xa6
 800427c: f8d3 c030    	ldr.w	r12, [r3, #0x30]
 8004280: eb0c 0508    	add.w	r5, r12, r8
 8004284: 454d         	cmp	r5, r9
 8004286: bf22         	ittt	hs
 8004288: 6c9b         	ldrhs	r3, [r3, #0x48]
 800428a: eb03 0508    	addhs.w	r5, r3, r8
 800428e: 454d         	cmphs	r5, r9
 8004290: d348         	blo	0x8004324 <raylar_crcl_encoder_service::benchmark::peak::he0ae015b97d7ff8e+0x542> @ imm = #0x90
 8004292: 2900         	cmp	r1, #0x0
 8004294: bf48         	it	mi
 8004296: 4249         	rsbmi	r1, r1, #0
 8004298: 4551         	cmp	r1, r10
 800429a: bf98         	it	ls
 800429c: 4651         	movls	r1, r10
 800429e: 2800         	cmp	r0, #0x0
 80042a0: bf48         	it	mi
 80042a2: 4240         	rsbmi	r0, r0, #0
 80042a4: 4288         	cmp	r0, r1
 80042a6: bf98         	it	ls
 80042a8: 4608         	movls	r0, r1
 80042aa: 2a00         	cmp	r2, #0x0
 80042ac: bf48         	it	mi
 80042ae: 4252         	rsbmi	r2, r2, #0
 80042b0: 4282         	cmp	r2, r0
 80042b2: bf98         	it	ls
 80042b4: 4602         	movls	r2, r0
 80042b6: 2c00         	cmp	r4, #0x0
 80042b8: bf48         	it	mi
 80042ba: 4264         	rsbmi	r4, r4, #0
 80042bc: 4294         	cmp	r4, r2
 80042be: bf98         	it	ls
 80042c0: 4614         	movls	r4, r2
 80042c2: 2e00         	cmp	r6, #0x0
 80042c4: bf48         	it	mi
 80042c6: 4276         	rsbmi	r6, r6, #0
 80042c8: 42a6         	cmp	r6, r4
 80042ca: bf98         	it	ls
 80042cc: 4626         	movls	r6, r4
 80042ce: f1be 0f00    	cmp.w	lr, #0x0
 80042d2: bf48         	it	mi
 80042d4: f1ce 0e00    	rsbmi.w	lr, lr, #0x0
 80042d8: 45b6         	cmp	lr, r6
 80042da: bf98         	it	ls
 80042dc: 46b6         	movls	lr, r6
 80042de: f1bc 0f00    	cmp.w	r12, #0x0
 80042e2: bf48         	it	mi
 80042e4: f1cc 0c00    	rsbmi.w	r12, r12, #0x0
 80042e8: 45f4         	cmp	r12, lr
 80042ea: bf98         	it	ls
 80042ec: 46f4         	movls	r12, lr
 80042ee: 2b00         	cmp	r3, #0x0
 80042f0: bf48         	it	mi
 80042f2: 425b         	rsbmi	r3, r3, #0
 80042f4: f10b 0bc0    	add.w	r11, r11, #0xc0
 80042f8: 4563         	cmp	r3, r12
 80042fa: bf98         	it	ls
 80042fc: 4663         	movls	r3, r12
 80042fe: 469a         	mov	r10, r3
 8004300: f5bb 4fc0    	cmp.w	r11, #0x6000
 8004304: d192         	bne	0x800422c <raylar_crcl_encoder_service::benchmark::peak::he0ae015b97d7ff8e+0x44a> @ imm = #-0xdc
 8004306: 9306         	str	r3, [sp, #0x18]
 8004308: a806         	add	r0, sp, #0x18
 800430a: e9dd 1002    	ldrd	r1, r0, [sp, #8]
 800430e: 9a00         	ldr	r2, [sp]
 8004310: 4048         	eors	r0, r1
 8004312: 9901         	ldr	r1, [sp, #0x4]
 8004314: 4048         	eors	r0, r1
 8004316: 9906         	ldr	r1, [sp, #0x18]
 8004318: 4050         	eors	r0, r2
 800431a: 4048         	eors	r0, r1
 800431c: b007         	add	sp, #0x1c
 800431e: e8bd 0f00    	pop.w	{r8, r9, r10, r11}
 8004322: bdf0         	pop	{r4, r5, r6, r7, pc}
 8004324: f24e 7088    	movw	r0, #0xe788
 8004328: f24e 7290    	movw	r2, #0xe790
 800432c: f6c0 0000    	movt	r0, #0x800
 8004330: f6c0 0200    	movt	r2, #0x800
 8004334: 210b         	movs	r1, #0xb
 8004336: f007 fe37    	bl	0x800bfa8 <core::panicking::panic_fmt> @ imm = #0x7c6e

0800433a <raylar_crcl_encoder_service::benchmark::codec::h6d81b9c8b86ce9a2>:
 800433a: b5f0         	push	{r4, r5, r6, r7, lr}
 800433c: af03         	add	r7, sp, #0xc
 800433e: e92d 0b00    	push.w	{r8, r9, r11}
 8004342: b0d6         	sub	sp, #0x158
 8004344: f240 690a    	movw	r9, #0x60a
 8004348: 4680         	mov	r8, r0
 800434a: f44f 0000    	mov.w	r0, #0x800000
 800434e: f2c0 1901    	movt	r9, #0x101
 8004352: 900b         	str	r0, [sp, #0x2c]
 8004354: f44f 6080    	mov.w	r0, #0x400
 8004358: f8cd 9030    	str.w	r9, [sp, #0x30]
 800435c: ad0a         	add	r5, sp, #0x28
 800435e: 900a         	str	r0, [sp, #0x28]
 8004360: 980a         	ldr	r0, [sp, #0x28]
 8004362: 1e41         	subs	r1, r0, #0x1
 8004364: ea80 0201    	eor.w	r2, r0, r1
 8004368: 428a         	cmp	r2, r1
 800436a: d916         	bls	0x800439a <raylar_crcl_encoder_service::benchmark::codec::h6d81b9c8b86ce9a2+0x60> @ imm = #0x2c
 800436c: fa90 f0a0    	rbit	r0, r0
 8004370: fab0 f080    	clz	r0, r0
 8004374: 3808         	subs	r0, #0x8
 8004376: 2804         	cmp	r0, #0x4
 8004378: bf9c         	itt	ls
 800437a: f89d 0030    	ldrbls.w	r0, [sp, #0x30]
 800437e: 280c         	cmpls	r0, #0xc
 8004380: d80b         	bhi	0x800439a <raylar_crcl_encoder_service::benchmark::codec::h6d81b9c8b86ce9a2+0x60> @ imm = #0x16
 8004382: 2101         	movs	r1, #0x1
 8004384: fa01 f000    	lsl.w	r0, r1, r0
 8004388: f410 5faa    	tst.w	r0, #0x1540
 800438c: bf1e         	ittt	ne
 800438e: 980b         	ldrne	r0, [sp, #0x2c]
 8004390: f100 4040    	addne.w	r0, r0, #0xc0000000
 8004394: f1b0 4f40    	cmpne.w	r0, #0xc0000000
 8004398: d813         	bhi	0x80043c2 <raylar_crcl_encoder_service::benchmark::codec::h6d81b9c8b86ce9a2+0x88> @ imm = #0x26
 800439a: 2000         	movs	r0, #0x0
 800439c: f88d 00c4    	strb.w	r0, [sp, #0xc4]
 80043a0: f64e 0010    	movw	r0, #0xe810
 80043a4: f6c0 0000    	movt	r0, #0x800
 80043a8: f24e 539c    	movw	r3, #0xe59c
 80043ac: 9000         	str	r0, [sp]
 80043ae: f24e 5000    	movw	r0, #0xe500
 80043b2: aa31         	add	r2, sp, #0xc4
 80043b4: f6c0 0000    	movt	r0, #0x800
 80043b8: f6c0 0300    	movt	r3, #0x800
 80043bc: 212b         	movs	r1, #0x2b
 80043be: f007 fd73    	bl	0x800bea8 <core::result::unwrap_failed> @ imm = #0x7ae6
 80043c2: f89d 0031    	ldrb.w	r0, [sp, #0x31]
 80043c6: 281e         	cmp	r0, #0x1e
 80043c8: d8e7         	bhi	0x800439a <raylar_crcl_encoder_service::benchmark::codec::h6d81b9c8b86ce9a2+0x60> @ imm = #-0x32
 80043ca: f89d 4033    	ldrb.w	r4, [sp, #0x33]
 80043ce: 2c01         	cmp	r4, #0x1
 80043d0: d104         	bne	0x80043dc <raylar_crcl_encoder_service::benchmark::codec::h6d81b9c8b86ce9a2+0xa2> @ imm = #0x8
 80043d2: f89d 0032    	ldrb.w	r0, [sp, #0x32]
 80043d6: 2801         	cmp	r0, #0x1
 80043d8: d002         	beq	0x80043e0 <raylar_crcl_encoder_service::benchmark::codec::h6d81b9c8b86ce9a2+0xa6> @ imm = #0x4
 80043da: e7de         	b	0x800439a <raylar_crcl_encoder_service::benchmark::codec::h6d81b9c8b86ce9a2+0x60> @ imm = #-0x44
 80043dc: 2800         	cmp	r0, #0x0
 80043de: d1dc         	bne	0x800439a <raylar_crcl_encoder_service::benchmark::codec::h6d81b9c8b86ce9a2+0x60> @ imm = #-0x48
 80043e0: e9dd 010a    	ldrd	r0, r1, [sp, #40]
 80043e4: 2600         	movs	r6, #0x0
 80043e6: f8dd 202f    	ldr.w	r2, [sp, #0x2f]
 80043ea: f8cd 00d3    	str.w	r0, [sp, #0xd3]
 80043ee: a802         	add	r0, sp, #0x8
 80043f0: f8cd 20da    	str.w	r2, [sp, #0xda]
 80043f4: 3001         	adds	r0, #0x1
 80043f6: f8cd 10d7    	str.w	r1, [sp, #0xd7]
 80043fa: a931         	add	r1, sp, #0xc4
 80043fc: 221a         	movs	r2, #0x1a
 80043fe: 9633         	str	r6, [sp, #0xcc]
 8004400: e9cd 6631    	strd	r6, r6, [sp, #196]
 8004404: f8cd 60cf    	str.w	r6, [sp, #0xcf]
 8004408: f88d 6008    	strb.w	r6, [sp, #0x8]
 800440c: f008 fd88    	bl	0x800cf20 <__aeabi_memcpy> @ imm = #0x8b10
 8004410: f44f 0000    	mov.w	r0, #0x800000
 8004414: f88d 4023    	strb.w	r4, [sp, #0x23]
 8004418: 9032         	str	r0, [sp, #0xc8]
 800441a: f44f 6080    	mov.w	r0, #0x400
 800441e: 9031         	str	r0, [sp, #0xc4]
 8004420: a831         	add	r0, sp, #0xc4
 8004422: f8cd 90cc    	str.w	r9, [sp, #0xcc]
 8004426: 9831         	ldr	r0, [sp, #0xc4]
 8004428: 1e41         	subs	r1, r0, #0x1
 800442a: ea80 0201    	eor.w	r2, r0, r1
 800442e: 428a         	cmp	r2, r1
 8004430: d90a         	bls	0x8004448 <raylar_crcl_encoder_service::benchmark::codec::h6d81b9c8b86ce9a2+0x10e> @ imm = #0x14
 8004432: fa90 f1a0    	rbit	r1, r0
 8004436: fab1 f181    	clz	r1, r1
 800443a: 3908         	subs	r1, #0x8
 800443c: 2904         	cmp	r1, #0x4
 800443e: bf9c         	itt	ls
 8004440: f89d 10cc    	ldrbls.w	r1, [sp, #0xcc]
 8004444: 290c         	cmpls	r1, #0xc
 8004446: d906         	bls	0x8004456 <raylar_crcl_encoder_service::benchmark::codec::h6d81b9c8b86ce9a2+0x11c> @ imm = #0xc
 8004448: f64e 0000    	movw	r0, #0xe800
 800444c: f88d 60c4    	strb.w	r6, [sp, #0xc4]
 8004450: f6c0 0000    	movt	r0, #0x800
 8004454: e7a8         	b	0x80043a8 <raylar_crcl_encoder_service::benchmark::codec::h6d81b9c8b86ce9a2+0x6e> @ imm = #-0xb0
 8004456: 2201         	movs	r2, #0x1
 8004458: fa02 f101    	lsl.w	r1, r2, r1
 800445c: f411 5faa    	tst.w	r1, #0x1540
 8004460: d0f2         	beq	0x8004448 <raylar_crcl_encoder_service::benchmark::codec::h6d81b9c8b86ce9a2+0x10e> @ imm = #-0x1c
 8004462: 9a32         	ldr	r2, [sp, #0xc8]
 8004464: 2101         	movs	r1, #0x1
 8004466: f2cc 0100    	movt	r1, #0xc000
 800446a: f102 4240    	add.w	r2, r2, #0xc0000000
 800446e: 428a         	cmp	r2, r1
 8004470: d3ea         	blo	0x8004448 <raylar_crcl_encoder_service::benchmark::codec::h6d81b9c8b86ce9a2+0x10e> @ imm = #-0x2c
 8004472: f89d 10cd    	ldrb.w	r1, [sp, #0xcd]
 8004476: 291e         	cmp	r1, #0x1e
 8004478: d8e6         	bhi	0x8004448 <raylar_crcl_encoder_service::benchmark::codec::h6d81b9c8b86ce9a2+0x10e> @ imm = #-0x34
 800447a: f89d 20cf    	ldrb.w	r2, [sp, #0xcf]
 800447e: 2a01         	cmp	r2, #0x1
 8004480: d107         	bne	0x8004492 <raylar_crcl_encoder_service::benchmark::codec::h6d81b9c8b86ce9a2+0x158> @ imm = #0xe
 8004482: f5b0 6f80    	cmp.w	r0, #0x400
 8004486: d3df         	blo	0x8004448 <raylar_crcl_encoder_service::benchmark::codec::h6d81b9c8b86ce9a2+0x10e> @ imm = #-0x42
 8004488: f89d 00ce    	ldrb.w	r0, [sp, #0xce]
 800448c: 2801         	cmp	r0, #0x1
 800448e: d005         	beq	0x800449c <raylar_crcl_encoder_service::benchmark::codec::h6d81b9c8b86ce9a2+0x162> @ imm = #0xa
 8004490: e7da         	b	0x8004448 <raylar_crcl_encoder_service::benchmark::codec::h6d81b9c8b86ce9a2+0x10e> @ imm = #-0x4c
 8004492: f5b0 6f80    	cmp.w	r0, #0x400
 8004496: d3d7         	blo	0x8004448 <raylar_crcl_encoder_service::benchmark::codec::h6d81b9c8b86ce9a2+0x10e> @ imm = #-0x52
 8004498: 2900         	cmp	r1, #0x0
 800449a: d1d5         	bne	0x8004448 <raylar_crcl_encoder_service::benchmark::codec::h6d81b9c8b86ce9a2+0x10e> @ imm = #-0x56
 800449c: f246 0028    	movw	r0, #0x6028
 80044a0: eb08 0400    	add.w	r4, r8, r0
 80044a4: f105 0048    	add.w	r0, r5, #0x48
 80044a8: 2128         	movs	r1, #0x28
 80044aa: f008 ff62    	bl	0x800d372 <__aeabi_memclr4> @ imm = #0x8ec4
 80044ae: ab31         	add	r3, sp, #0xc4
 80044b0: ae31         	add	r6, sp, #0xc4
 80044b2: 2000         	movs	r0, #0x0
 80044b4: 941a         	str	r4, [sp, #0x68]
 80044b6: cb0e         	ldm	r3, {r1, r2, r3}
 80044b8: f44f 7480    	mov.w	r4, #0x100
 80044bc: 932d         	str	r3, [sp, #0xb4]
 80044be: f44f 53c0    	mov.w	r3, #0x1800
 80044c2: e9cd 122b    	strd	r1, r2, [sp, #172]
 80044c6: 2101         	movs	r1, #0x1
 80044c8: f8ad 0044    	strh.w	r0, [sp, #0x44]
 80044cc: e9cd 000f    	strd	r0, r0, [sp, #60]
 80044d0: e9cd 000d    	strd	r0, r0, [sp, #52]
 80044d4: e9cd 000b    	strd	r0, r0, [sp, #44]
 80044d8: e9cd 0013    	strd	r0, r0, [sp, #76]
 80044dc: e9cd 0015    	strd	r0, r0, [sp, #84]
 80044e0: e9cd 0017    	strd	r0, r0, [sp, #92]
 80044e4: f8ad 0064    	strh.w	r0, [sp, #0x64]
 80044e8: e9cd 0027    	strd	r0, r0, [sp, #156]
 80044ec: e9cd 0029    	strd	r0, r0, [sp, #164]
 80044f0: 900a         	str	r0, [sp, #0x28]
 80044f2: 9012         	str	r0, [sp, #0x48]
 80044f4: 931b         	str	r3, [sp, #0x6c]
 80044f6: 9126         	str	r1, [sp, #0x98]
 80044f8: f8ad 00b8    	strh.w	r0, [sp, #0xb8]
 80044fc: 9431         	str	r4, [sp, #0xc4]
 80044fe: 4628         	mov	r0, r5
 8004500: 9931         	ldr	r1, [sp, #0xc4]
 8004502: f7fd fb74    	bl	0x8001bee <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3> @ imm = #-0x2918
 8004506: 07c0         	lsls	r0, r0, #0x1f
 8004508: d123         	bne	0x8004552 <raylar_crcl_encoder_service::benchmark::codec::h6d81b9c8b86ce9a2+0x218> @ imm = #0x46
 800450a: 07c8         	lsls	r0, r1, #0x1f
 800450c: d0f6         	beq	0x80044fc <raylar_crcl_encoder_service::benchmark::codec::h6d81b9c8b86ce9a2+0x1c2> @ imm = #-0x14
 800450e: ad31         	add	r5, sp, #0xc4
 8004510: a90a         	add	r1, sp, #0x28
 8004512: 2294         	movs	r2, #0x94
 8004514: 4628         	mov	r0, r5
 8004516: f008 fede    	bl	0x800d2d6 <__aeabi_memcpy4> @ imm = #0x8dbc
 800451a: f641 6058    	movw	r0, #0x1e58
 800451e: f508 34e0    	add.w	r4, r8, #0x1c000
 8004522: 9000         	str	r0, [sp]
 8004524: a82f         	add	r0, sp, #0xbc
 8004526: a902         	add	r1, sp, #0x8
 8004528: 462a         	mov	r2, r5
 800452a: 4623         	mov	r3, r4
 800452c: f7fd f8a8    	bl	0x8001680 <raylar_crcl_encoder_service::format::Encoder::data::ha65fe528befbf4e3> @ imm = #-0x2eb0
 8004530: f89d 00bc    	ldrb.w	r0, [sp, #0xbc]
 8004534: 2801         	cmp	r0, #0x1
 8004536: d01c         	beq	0x8004572 <raylar_crcl_encoder_service::benchmark::codec::h6d81b9c8b86ce9a2+0x238> @ imm = #0x38
 8004538: 9930         	ldr	r1, [sp, #0xc0]
 800453a: f641 6059    	movw	r0, #0x1e59
 800453e: 4281         	cmp	r1, r0
 8004540: d20e         	bhs	0x8004560 <raylar_crcl_encoder_service::benchmark::codec::h6d81b9c8b86ce9a2+0x226> @ imm = #0x1c
 8004542: e9cd 4131    	strd	r4, r1, [sp, #196]
 8004546: a831         	add	r0, sp, #0xc4
 8004548: 4608         	mov	r0, r1
 800454a: b056         	add	sp, #0x158
 800454c: e8bd 0b00    	pop.w	{r8, r9, r11}
 8004550: bdf0         	pop	{r4, r5, r6, r7, pc}
 8004552: f24e 70f0    	movw	r0, #0xe7f0
 8004556: f88d 10c4    	strb.w	r1, [sp, #0xc4]
 800455a: f6c0 0000    	movt	r0, #0x800
 800455e: e723         	b	0x80043a8 <raylar_crcl_encoder_service::benchmark::codec::h6d81b9c8b86ce9a2+0x6e> @ imm = #-0x1ba
 8004560: f24e 73d0    	movw	r3, #0xe7d0
 8004564: 2000         	movs	r0, #0x0
 8004566: f6c0 0300    	movt	r3, #0x800
 800456a: f641 6258    	movw	r2, #0x1e58
 800456e: f007 ffa6    	bl	0x800c4be <core::slice::index::slice_index_fail> @ imm = #0x7f4c
 8004572: f89d 00bd    	ldrb.w	r0, [sp, #0xbd]
 8004576: f88d 00c4    	strb.w	r0, [sp, #0xc4]
 800457a: f24e 70e0    	movw	r0, #0xe7e0
 800457e: f6c0 0000    	movt	r0, #0x800
 8004582: e711         	b	0x80043a8 <raylar_crcl_encoder_service::benchmark::codec::h6d81b9c8b86ce9a2+0x6e> @ imm = #-0x1de

08004584 <raylar_crcl_encoder_service::benchmark::empty::h49addc3cc3cef455>:
 8004584: b580         	push	{r7, lr}
 8004586: 466f         	mov	r7, sp
 8004588: b081         	sub	sp, #0x4
 800458a: 2000         	movs	r0, #0x0
 800458c: 9000         	str	r0, [sp]
 800458e: 4668         	mov	r0, sp
 8004590: f85d 0b04    	ldr	r0, [sp], #4
 8004594: bd80         	pop	{r7, pc}

08004596 <raylar_crcl_encoder_service::benchmark::fused::h66edd02a0d627444>:
 8004596: b5f0         	push	{r4, r5, r6, r7, lr}
 8004598: af03         	add	r7, sp, #0xc
 800459a: e92d 0f00    	push.w	{r8, r9, r10, r11}
 800459e: b08a         	sub	sp, #0x28
 80045a0: 6801         	ldr	r1, [r0]
 80045a2: f246 052c    	movw	r5, #0x602c
 80045a6: 9108         	str	r1, [sp, #0x20]
 80045a8: a908         	add	r1, sp, #0x20
 80045aa: 2601         	movs	r6, #0x1
 80045ac: 9005         	str	r0, [sp, #0x14]
 80045ae: 9908         	ldr	r1, [sp, #0x20]
 80045b0: f04f 0e00    	mov.w	lr, #0x0
 80045b4: 5943         	ldr	r3, [r0, r5]
 80045b6: 1e4a         	subs	r2, r1, #0x1
 80045b8: f002 021f    	and	r2, r2, #0x1f
 80045bc: fa06 fc02    	lsl.w	r12, r6, r2
 80045c0: eb03 060c    	add.w	r6, r3, r12
 80045c4: f001 031f    	and	r3, r1, #0x1f
 80045c8: 1941         	adds	r1, r0, r5
 80045ca: 9104         	str	r1, [sp, #0x10]
 80045cc: fa46 fa03    	asr.w	r10, r6, r3
 80045d0: f840 a005    	str.w	r10, [r0, r5]
 80045d4: f240 30ff    	movw	r0, #0x3ff
 80045d8: 6b0d         	ldr	r5, [r1, #0x30]
 80045da: 6c8c         	ldr	r4, [r1, #0x48]
 80045dc: 4465         	add	r5, r12
 80045de: 9006         	str	r0, [sp, #0x18]
 80045e0: 4464         	add	r4, r12
 80045e2: f8d1 9018    	ldr.w	r9, [r1, #0x18]
 80045e6: 411d         	asrs	r5, r3
 80045e8: f8d1 8060    	ldr.w	r8, [r1, #0x60]
 80045ec: 411c         	asrs	r4, r3
 80045ee: 1b60         	subs	r0, r4, r5
 80045f0: 630d         	str	r5, [r1, #0x30]
 80045f2: 0046         	lsls	r6, r0, #0x1
 80045f4: ea86 7be0    	eor.w	r11, r6, r0, asr #31
 80045f8: eb09 060c    	add.w	r6, r9, r12
 80045fc: 648c         	str	r4, [r1, #0x48]
 80045fe: f8d1 2108    	ldr.w	r2, [r1, #0x108]
 8004602: 411e         	asrs	r6, r3
 8004604: 1bad         	subs	r5, r5, r6
 8004606: 618e         	str	r6, [r1, #0x18]
 8004608: 4462         	add	r2, r12
 800460a: 0068         	lsls	r0, r5, #0x1
 800460c: ea80 70e5    	eor.w	r0, r0, r5, asr #31
 8004610: eba6 050a    	sub.w	r5, r6, r10
 8004614: fa42 fa03    	asr.w	r10, r2, r3
 8004618: 006e         	lsls	r6, r5, #0x1
 800461a: ea86 75e5    	eor.w	r5, r6, r5, asr #31
 800461e: eb0e 0605    	add.w	r6, lr, r5
 8004622: eb08 050c    	add.w	r5, r8, r12
 8004626: 4430         	add	r0, r6
 8004628: f8d1 e078    	ldr.w	lr, [r1, #0x78]
 800462c: 411d         	asrs	r5, r3
 800462e: 1b2c         	subs	r4, r5, r4
 8004630: 4458         	add	r0, r11
 8004632: 660d         	str	r5, [r1, #0x60]
 8004634: 0066         	lsls	r6, r4, #0x1
 8004636: ea86 74e4    	eor.w	r4, r6, r4, asr #31
 800463a: 4420         	add	r0, r4
 800463c: eb0e 040c    	add.w	r4, lr, r12
 8004640: f8d1 8090    	ldr.w	r8, [r1, #0x90]
 8004644: 411c         	asrs	r4, r3
 8004646: 1b65         	subs	r5, r4, r5
 8004648: 678c         	str	r4, [r1, #0x78]
 800464a: 006e         	lsls	r6, r5, #0x1
 800464c: f8d1 e0a8    	ldr.w	lr, [r1, #0xa8]
 8004650: ea86 76e5    	eor.w	r6, r6, r5, asr #31
 8004654: f8c1 a108    	str.w	r10, [r1, #0x108]
 8004658: 4430         	add	r0, r6
 800465a: eb08 060c    	add.w	r6, r8, r12
 800465e: 411e         	asrs	r6, r3
 8004660: 1b34         	subs	r4, r6, r4
 8004662: f8c1 6090    	str.w	r6, [r1, #0x90]
 8004666: 0065         	lsls	r5, r4, #0x1
 8004668: ea85 74e4    	eor.w	r4, r5, r4, asr #31
 800466c: eb00 0804    	add.w	r8, r0, r4
 8004670: eb0e 040c    	add.w	r4, lr, r12
 8004674: f8d1 50f0    	ldr.w	r5, [r1, #0xf0]
 8004678: fa44 f003    	asr.w	r0, r4, r3
 800467c: 1b86         	subs	r6, r0, r6
 800467e: 4465         	add	r5, r12
 8004680: f8c1 00a8    	str.w	r0, [r1, #0xa8]
 8004684: 0074         	lsls	r4, r6, #0x1
 8004686: ea84 7ee6    	eor.w	lr, r4, r6, asr #31
 800468a: f8d1 60c0    	ldr.w	r6, [r1, #0xc0]
 800468e: fa45 f403    	asr.w	r4, r5, r3
 8004692: f8d1 50d8    	ldr.w	r5, [r1, #0xd8]
 8004696: 44c6         	add	lr, r8
 8004698: 4466         	add	r6, r12
 800469a: f8c1 40f0    	str.w	r4, [r1, #0xf0]
 800469e: 4465         	add	r5, r12
 80046a0: 411e         	asrs	r6, r3
 80046a2: 1a30         	subs	r0, r6, r0
 80046a4: 411d         	asrs	r5, r3
 80046a6: f8c1 60c0    	str.w	r6, [r1, #0xc0]
 80046aa: 0042         	lsls	r2, r0, #0x1
 80046ac: ea82 70e0    	eor.w	r0, r2, r0, asr #31
 80046b0: 1baa         	subs	r2, r5, r6
 80046b2: 4470         	add	r0, lr
 80046b4: f8c1 50d8    	str.w	r5, [r1, #0xd8]
 80046b8: f501 7184    	add.w	r1, r1, #0x108
 80046bc: 0056         	lsls	r6, r2, #0x1
 80046be: ea86 72e2    	eor.w	r2, r6, r2, asr #31
 80046c2: 4410         	add	r0, r2
 80046c4: 1b62         	subs	r2, r4, r5
 80046c6: 0056         	lsls	r6, r2, #0x1
 80046c8: ea86 72e2    	eor.w	r2, r6, r2, asr #31
 80046cc: 4410         	add	r0, r2
 80046ce: ebaa 0204    	sub.w	r2, r10, r4
 80046d2: 0056         	lsls	r6, r2, #0x1
 80046d4: ea86 72e2    	eor.w	r2, r6, r2, asr #31
 80046d8: eb00 0e02    	add.w	lr, r0, r2
 80046dc: 9806         	ldr	r0, [sp, #0x18]
 80046de: 380b         	subs	r0, #0xb
 80046e0: f47f af7a    	bne.w	0x80045d8 <raylar_crcl_encoder_service::benchmark::fused::h66edd02a0d627444+0x42> @ imm = #-0x10c
 80046e4: f8cd e024    	str.w	lr, [sp, #0x24]
 80046e8: a909         	add	r1, sp, #0x24
 80046ea: 2501         	movs	r5, #0x1
 80046ec: f04f 0900    	mov.w	r9, #0x0
 80046f0: 9809         	ldr	r0, [sp, #0x24]
 80046f2: f240 3eff    	movw	lr, #0x3ff
 80046f6: 9003         	str	r0, [sp, #0xc]
 80046f8: 9805         	ldr	r0, [sp, #0x14]
 80046fa: 6881         	ldr	r1, [r0, #0x8]
 80046fc: 9108         	str	r1, [sp, #0x20]
 80046fe: a908         	add	r1, sp, #0x20
 8004700: 9908         	ldr	r1, [sp, #0x20]
 8004702: 9a04         	ldr	r2, [sp, #0x10]
 8004704: 1e4b         	subs	r3, r1, #0x1
 8004706: f001 041f    	and	r4, r1, #0x1f
 800470a: f003 031f    	and	r3, r3, #0x1f
 800470e: f852 6f04    	ldr	r6, [r2, #4]!
 8004712: fa05 f303    	lsl.w	r3, r5, r3
 8004716: 441e         	add	r6, r3
 8004718: fa46 fb04    	asr.w	r11, r6, r4
 800471c: f8c2 b000    	str.w	r11, [r2]
 8004720: 6b15         	ldr	r5, [r2, #0x30]
 8004722: f1be 0e0b    	subs.w	lr, lr, #0xb
 8004726: 6c96         	ldr	r6, [r2, #0x48]
 8004728: f8d2 a018    	ldr.w	r10, [r2, #0x18]
 800472c: 441d         	add	r5, r3
 800472e: 6e10         	ldr	r0, [r2, #0x60]
 8004730: 441e         	add	r6, r3
 8004732: 9006         	str	r0, [sp, #0x18]
 8004734: eb0a 0003    	add.w	r0, r10, r3
 8004738: fa45 f504    	asr.w	r5, r5, r4
 800473c: fa46 f604    	asr.w	r6, r6, r4
 8004740: eba6 0c05    	sub.w	r12, r6, r5
 8004744: fa40 f004    	asr.w	r0, r0, r4
 8004748: 6315         	str	r5, [r2, #0x30]
 800474a: eba5 0500    	sub.w	r5, r5, r0
 800474e: 6190         	str	r0, [r2, #0x18]
 8004750: eba0 000b    	sub.w	r0, r0, r11
 8004754: ea4f 0145    	lsl.w	r1, r5, #0x1
 8004758: 6496         	str	r6, [r2, #0x48]
 800475a: ea81 71e5    	eor.w	r1, r1, r5, asr #31
 800475e: ea4f 0540    	lsl.w	r5, r0, #0x1
 8004762: ea85 70e0    	eor.w	r0, r5, r0, asr #31
 8004766: ea4f 084c    	lsl.w	r8, r12, #0x1
 800476a: 4448         	add	r0, r9
 800476c: ea88 7cec    	eor.w	r12, r8, r12, asr #31
 8004770: 4408         	add	r0, r1
 8004772: 9906         	ldr	r1, [sp, #0x18]
 8004774: f8d2 8078    	ldr.w	r8, [r2, #0x78]
 8004778: 4460         	add	r0, r12
 800477a: eb01 0503    	add.w	r5, r1, r3
 800477e: f8d2 c090    	ldr.w	r12, [r2, #0x90]
 8004782: fa45 f504    	asr.w	r5, r5, r4
 8004786: eba5 0606    	sub.w	r6, r5, r6
 800478a: 6615         	str	r5, [r2, #0x60]
 800478c: ea4f 0146    	lsl.w	r1, r6, #0x1
 8004790: ea81 71e6    	eor.w	r1, r1, r6, asr #31
 8004794: 4408         	add	r0, r1
 8004796: eb08 0103    	add.w	r1, r8, r3
 800479a: f8d2 80a8    	ldr.w	r8, [r2, #0xa8]
 800479e: fa41 f104    	asr.w	r1, r1, r4
 80047a2: eba1 0505    	sub.w	r5, r1, r5
 80047a6: 6791         	str	r1, [r2, #0x78]
 80047a8: ea4f 0645    	lsl.w	r6, r5, #0x1
 80047ac: ea86 75e5    	eor.w	r5, r6, r5, asr #31
 80047b0: 4428         	add	r0, r5
 80047b2: eb0c 0503    	add.w	r5, r12, r3
 80047b6: fa45 f504    	asr.w	r5, r5, r4
 80047ba: eba5 0101    	sub.w	r1, r5, r1
 80047be: f8c2 5090    	str.w	r5, [r2, #0x90]
 80047c2: ea4f 0641    	lsl.w	r6, r1, #0x1
 80047c6: ea86 71e1    	eor.w	r1, r6, r1, asr #31
 80047ca: eb00 0c01    	add.w	r12, r0, r1
 80047ce: eb08 0103    	add.w	r1, r8, r3
 80047d2: f8d2 60f0    	ldr.w	r6, [r2, #0xf0]
 80047d6: fa41 f104    	asr.w	r1, r1, r4
 80047da: eba1 0505    	sub.w	r5, r1, r5
 80047de: f8c2 10a8    	str.w	r1, [r2, #0xa8]
 80047e2: ea4f 0045    	lsl.w	r0, r5, #0x1
 80047e6: ea80 78e5    	eor.w	r8, r0, r5, asr #31
 80047ea: eb06 0503    	add.w	r5, r6, r3
 80047ee: f8d2 00c0    	ldr.w	r0, [r2, #0xc0]
 80047f2: f8d2 60d8    	ldr.w	r6, [r2, #0xd8]
 80047f6: 44c4         	add	r12, r8
 80047f8: fa45 f904    	asr.w	r9, r5, r4
 80047fc: 4418         	add	r0, r3
 80047fe: f8d2 5108    	ldr.w	r5, [r2, #0x108]
 8004802: 441e         	add	r6, r3
 8004804: fa40 f004    	asr.w	r0, r0, r4
 8004808: eba0 0101    	sub.w	r1, r0, r1
 800480c: 441d         	add	r5, r3
 800480e: fa46 f604    	asr.w	r6, r6, r4
 8004812: f8c2 00c0    	str.w	r0, [r2, #0xc0]
 8004816: eba6 0000    	sub.w	r0, r6, r0
 800481a: fa45 fb04    	asr.w	r11, r5, r4
 800481e: ea4f 0541    	lsl.w	r5, r1, #0x1
 8004822: ea85 71e1    	eor.w	r1, r5, r1, asr #31
 8004826: ea4f 0540    	lsl.w	r5, r0, #0x1
 800482a: 4461         	add	r1, r12
 800482c: ea85 70e0    	eor.w	r0, r5, r0, asr #31
 8004830: 4408         	add	r0, r1
 8004832: eba9 0106    	sub.w	r1, r9, r6
 8004836: f8c2 60d8    	str.w	r6, [r2, #0xd8]
 800483a: ea4f 0641    	lsl.w	r6, r1, #0x1
 800483e: ea86 71e1    	eor.w	r1, r6, r1, asr #31
 8004842: 4408         	add	r0, r1
 8004844: ebab 0109    	sub.w	r1, r11, r9
 8004848: f8c2 90f0    	str.w	r9, [r2, #0xf0]
 800484c: ea4f 0641    	lsl.w	r6, r1, #0x1
 8004850: ea86 71e1    	eor.w	r1, r6, r1, asr #31
 8004854: f8c2 b108    	str.w	r11, [r2, #0x108]
 8004858: eb00 0901    	add.w	r9, r0, r1
 800485c: f502 7284    	add.w	r2, r2, #0x108
 8004860: f47f af5e    	bne.w	0x8004720 <raylar_crcl_encoder_service::benchmark::fused::h66edd02a0d627444+0x18a> @ imm = #-0x144
 8004864: f8cd 9024    	str.w	r9, [sp, #0x24]
 8004868: a809         	add	r0, sp, #0x24
 800486a: 2601         	movs	r6, #0x1
 800486c: f04f 0800    	mov.w	r8, #0x0
 8004870: 9809         	ldr	r0, [sp, #0x24]
 8004872: f240 3cff    	movw	r12, #0x3ff
 8004876: 9002         	str	r0, [sp, #0x8]
 8004878: 9805         	ldr	r0, [sp, #0x14]
 800487a: 6900         	ldr	r0, [r0, #0x10]
 800487c: 9008         	str	r0, [sp, #0x20]
 800487e: a808         	add	r0, sp, #0x20
 8004880: 9808         	ldr	r0, [sp, #0x20]
 8004882: 9a04         	ldr	r2, [sp, #0x10]
 8004884: 1e43         	subs	r3, r0, #0x1
 8004886: f000 041f    	and	r4, r0, #0x1f
 800488a: f003 031f    	and	r3, r3, #0x1f
 800488e: f852 1f08    	ldr	r1, [r2, #8]!
 8004892: fa06 f303    	lsl.w	r3, r6, r3
 8004896: 4419         	add	r1, r3
 8004898: fa41 fb04    	asr.w	r11, r1, r4
 800489c: f8c2 b000    	str.w	r11, [r2]
 80048a0: 6b15         	ldr	r5, [r2, #0x30]
 80048a2: f1bc 0c0b    	subs.w	r12, r12, #0xb
 80048a6: 6c96         	ldr	r6, [r2, #0x48]
 80048a8: 441d         	add	r5, r3
 80048aa: f8d2 a018    	ldr.w	r10, [r2, #0x18]
 80048ae: 441e         	add	r6, r3
 80048b0: f8d2 9060    	ldr.w	r9, [r2, #0x60]
 80048b4: fa45 f504    	asr.w	r5, r5, r4
 80048b8: fa46 f604    	asr.w	r6, r6, r4
 80048bc: eba6 0005    	sub.w	r0, r6, r5
 80048c0: 6315         	str	r5, [r2, #0x30]
 80048c2: ea4f 0140    	lsl.w	r1, r0, #0x1
 80048c6: ea81 7ee0    	eor.w	lr, r1, r0, asr #31
 80048ca: eb0a 0103    	add.w	r1, r10, r3
 80048ce: 6496         	str	r6, [r2, #0x48]
 80048d0: fa41 f104    	asr.w	r1, r1, r4
 80048d4: eba5 0501    	sub.w	r5, r5, r1
 80048d8: 6191         	str	r1, [r2, #0x18]
 80048da: eba1 010b    	sub.w	r1, r1, r11
 80048de: ea4f 0045    	lsl.w	r0, r5, #0x1
 80048e2: ea80 70e5    	eor.w	r0, r0, r5, asr #31
 80048e6: ea4f 0541    	lsl.w	r5, r1, #0x1
 80048ea: ea85 71e1    	eor.w	r1, r5, r1, asr #31
 80048ee: eb09 0503    	add.w	r5, r9, r3
 80048f2: 4441         	add	r1, r8
 80048f4: f8d2 8078    	ldr.w	r8, [r2, #0x78]
 80048f8: fa45 f504    	asr.w	r5, r5, r4
 80048fc: eba5 0606    	sub.w	r6, r5, r6
 8004900: 4408         	add	r0, r1
 8004902: 6615         	str	r5, [r2, #0x60]
 8004904: ea4f 0146    	lsl.w	r1, r6, #0x1
 8004908: 4470         	add	r0, lr
 800490a: ea81 71e6    	eor.w	r1, r1, r6, asr #31
 800490e: f8d2 e090    	ldr.w	lr, [r2, #0x90]
 8004912: 4408         	add	r0, r1
 8004914: eb08 0103    	add.w	r1, r8, r3
 8004918: f8d2 80a8    	ldr.w	r8, [r2, #0xa8]
 800491c: fa41 f104    	asr.w	r1, r1, r4
 8004920: eba1 0505    	sub.w	r5, r1, r5
 8004924: 6791         	str	r1, [r2, #0x78]
 8004926: ea4f 0645    	lsl.w	r6, r5, #0x1
 800492a: ea86 75e5    	eor.w	r5, r6, r5, asr #31
 800492e: 4428         	add	r0, r5
 8004930: eb0e 0503    	add.w	r5, lr, r3
 8004934: fa45 f504    	asr.w	r5, r5, r4
 8004938: eba5 0101    	sub.w	r1, r5, r1
 800493c: f8c2 5090    	str.w	r5, [r2, #0x90]
 8004940: ea4f 0641    	lsl.w	r6, r1, #0x1
 8004944: ea86 71e1    	eor.w	r1, r6, r1, asr #31
 8004948: eb00 0e01    	add.w	lr, r0, r1
 800494c: eb08 0103    	add.w	r1, r8, r3
 8004950: f8d2 60f0    	ldr.w	r6, [r2, #0xf0]
 8004954: fa41 f104    	asr.w	r1, r1, r4
 8004958: eba1 0505    	sub.w	r5, r1, r5
 800495c: f8c2 10a8    	str.w	r1, [r2, #0xa8]
 8004960: ea4f 0045    	lsl.w	r0, r5, #0x1
 8004964: ea80 78e5    	eor.w	r8, r0, r5, asr #31
 8004968: eb06 0503    	add.w	r5, r6, r3
 800496c: f8d2 00c0    	ldr.w	r0, [r2, #0xc0]
 8004970: f8d2 6108    	ldr.w	r6, [r2, #0x108]
 8004974: 44c6         	add	lr, r8
 8004976: fa45 f904    	asr.w	r9, r5, r4
 800497a: f8d2 50d8    	ldr.w	r5, [r2, #0xd8]
 800497e: 4418         	add	r0, r3
 8004980: 441e         	add	r6, r3
 8004982: 441d         	add	r5, r3
 8004984: f8c2 90f0    	str.w	r9, [r2, #0xf0]
 8004988: fa40 f004    	asr.w	r0, r0, r4
 800498c: eba0 0101    	sub.w	r1, r0, r1
 8004990: fa45 f504    	asr.w	r5, r5, r4
 8004994: f8c2 00c0    	str.w	r0, [r2, #0xc0]
 8004998: eba5 0000    	sub.w	r0, r5, r0
 800499c: fa46 fb04    	asr.w	r11, r6, r4
 80049a0: ea4f 0641    	lsl.w	r6, r1, #0x1
 80049a4: ea86 71e1    	eor.w	r1, r6, r1, asr #31
 80049a8: ea4f 0640    	lsl.w	r6, r0, #0x1
 80049ac: 4471         	add	r1, lr
 80049ae: ea86 70e0    	eor.w	r0, r6, r0, asr #31
 80049b2: f8c2 50d8    	str.w	r5, [r2, #0xd8]
 80049b6: 4408         	add	r0, r1
 80049b8: eba9 0105    	sub.w	r1, r9, r5
 80049bc: f8c2 b108    	str.w	r11, [r2, #0x108]
 80049c0: f502 7284    	add.w	r2, r2, #0x108
 80049c4: ea4f 0641    	lsl.w	r6, r1, #0x1
 80049c8: ea86 71e1    	eor.w	r1, r6, r1, asr #31
 80049cc: 4408         	add	r0, r1
 80049ce: ebab 0109    	sub.w	r1, r11, r9
 80049d2: ea4f 0641    	lsl.w	r6, r1, #0x1
 80049d6: ea86 71e1    	eor.w	r1, r6, r1, asr #31
 80049da: eb00 0801    	add.w	r8, r0, r1
 80049de: f47f af5f    	bne.w	0x80048a0 <raylar_crcl_encoder_service::benchmark::fused::h66edd02a0d627444+0x30a> @ imm = #-0x142
 80049e2: f8cd 8024    	str.w	r8, [sp, #0x24]
 80049e6: a809         	add	r0, sp, #0x24
 80049e8: 2301         	movs	r3, #0x1
 80049ea: f04f 0e00    	mov.w	lr, #0x0
 80049ee: 9809         	ldr	r0, [sp, #0x24]
 80049f0: f240 3cff    	movw	r12, #0x3ff
 80049f4: 9001         	str	r0, [sp, #0x4]
 80049f6: 9805         	ldr	r0, [sp, #0x14]
 80049f8: 6980         	ldr	r0, [r0, #0x18]
 80049fa: 9008         	str	r0, [sp, #0x20]
 80049fc: a808         	add	r0, sp, #0x20
 80049fe: 9808         	ldr	r0, [sp, #0x20]
 8004a00: 9e04         	ldr	r6, [sp, #0x10]
 8004a02: 1e42         	subs	r2, r0, #0x1
 8004a04: f000 041f    	and	r4, r0, #0x1f
 8004a08: f002 021f    	and	r2, r2, #0x1f
 8004a0c: f506 758a    	add.w	r5, r6, #0x114
 8004a10: 68f1         	ldr	r1, [r6, #0xc]
 8004a12: 4093         	lsls	r3, r2
 8004a14: 4419         	add	r1, r3
 8004a16: fa41 fb04    	asr.w	r11, r1, r4
 8004a1a: f8c6 b00c    	str.w	r11, [r6, #0xc]
 8004a1e: f855 6cd8    	ldr	r6, [r5, #-216]
 8004a22: f1bc 0c0b    	subs.w	r12, r12, #0xb
 8004a26: f855 2cc0    	ldr	r2, [r5, #-192]
 8004a2a: 441e         	add	r6, r3
 8004a2c: f855 acf0    	ldr	r10, [r5, #-240]
 8004a30: 441a         	add	r2, r3
 8004a32: f855 8ca8    	ldr	r8, [r5, #-168]
 8004a36: fa46 f604    	asr.w	r6, r6, r4
 8004a3a: fa42 f204    	asr.w	r2, r2, r4
 8004a3e: eba2 0006    	sub.w	r0, r2, r6
 8004a42: f845 6cd8    	str	r6, [r5, #-216]
 8004a46: ea4f 0140    	lsl.w	r1, r0, #0x1
 8004a4a: ea81 79e0    	eor.w	r9, r1, r0, asr #31
 8004a4e: eb0a 0103    	add.w	r1, r10, r3
 8004a52: f845 2cc0    	str	r2, [r5, #-192]
 8004a56: fa41 f104    	asr.w	r1, r1, r4
 8004a5a: eba6 0601    	sub.w	r6, r6, r1
 8004a5e: f845 1cf0    	str	r1, [r5, #-240]
 8004a62: eba1 010b    	sub.w	r1, r1, r11
 8004a66: ea4f 0046    	lsl.w	r0, r6, #0x1
 8004a6a: ea80 70e6    	eor.w	r0, r0, r6, asr #31
 8004a6e: ea4f 0641    	lsl.w	r6, r1, #0x1
 8004a72: ea86 71e1    	eor.w	r1, r6, r1, asr #31
 8004a76: eb08 0603    	add.w	r6, r8, r3
 8004a7a: 4471         	add	r1, lr
 8004a7c: f855 ec90    	ldr	lr, [r5, #-144]
 8004a80: fa46 f604    	asr.w	r6, r6, r4
 8004a84: eba6 0202    	sub.w	r2, r6, r2
 8004a88: 4408         	add	r0, r1
 8004a8a: f845 6ca8    	str	r6, [r5, #-168]
 8004a8e: ea4f 0142    	lsl.w	r1, r2, #0x1
 8004a92: 4448         	add	r0, r9
 8004a94: ea81 71e2    	eor.w	r1, r1, r2, asr #31
 8004a98: f855 8c78    	ldr	r8, [r5, #-120]
 8004a9c: 4408         	add	r0, r1
 8004a9e: eb0e 0103    	add.w	r1, lr, r3
 8004aa2: f855 ec60    	ldr	lr, [r5, #-96]
 8004aa6: fa41 f104    	asr.w	r1, r1, r4
 8004aaa: eba1 0606    	sub.w	r6, r1, r6
 8004aae: f845 1c90    	str	r1, [r5, #-144]
 8004ab2: ea4f 0246    	lsl.w	r2, r6, #0x1
 8004ab6: ea82 72e6    	eor.w	r2, r2, r6, asr #31
 8004aba: 4410         	add	r0, r2
 8004abc: eb08 0203    	add.w	r2, r8, r3
 8004ac0: fa42 f204    	asr.w	r2, r2, r4
 8004ac4: eba2 0101    	sub.w	r1, r2, r1
 8004ac8: f845 2c78    	str	r2, [r5, #-120]
 8004acc: ea4f 0641    	lsl.w	r6, r1, #0x1
 8004ad0: ea86 71e1    	eor.w	r1, r6, r1, asr #31
 8004ad4: eb00 0801    	add.w	r8, r0, r1
 8004ad8: eb0e 0103    	add.w	r1, lr, r3
 8004adc: f855 6c18    	ldr	r6, [r5, #-24]
 8004ae0: fa41 f104    	asr.w	r1, r1, r4
 8004ae4: eba1 0202    	sub.w	r2, r1, r2
 8004ae8: f845 1c60    	str	r1, [r5, #-96]
 8004aec: ea4f 0042    	lsl.w	r0, r2, #0x1
 8004af0: ea80 7ee2    	eor.w	lr, r0, r2, asr #31
 8004af4: eb06 0203    	add.w	r2, r6, r3
 8004af8: f855 0c48    	ldr	r0, [r5, #-72]
 8004afc: f855 6c30    	ldr	r6, [r5, #-48]
 8004b00: 44c6         	add	lr, r8
 8004b02: fa42 f904    	asr.w	r9, r2, r4
 8004b06: 4418         	add	r0, r3
 8004b08: 682a         	ldr	r2, [r5]
 8004b0a: 441e         	add	r6, r3
 8004b0c: fa40 f004    	asr.w	r0, r0, r4
 8004b10: eba0 0101    	sub.w	r1, r0, r1
 8004b14: 441a         	add	r2, r3
 8004b16: fa46 f604    	asr.w	r6, r6, r4
 8004b1a: f845 0c48    	str	r0, [r5, #-72]
 8004b1e: eba6 0000    	sub.w	r0, r6, r0
 8004b22: fa42 fb04    	asr.w	r11, r2, r4
 8004b26: ea4f 0241    	lsl.w	r2, r1, #0x1
 8004b2a: ea82 71e1    	eor.w	r1, r2, r1, asr #31
 8004b2e: ea4f 0240    	lsl.w	r2, r0, #0x1
 8004b32: 4471         	add	r1, lr
 8004b34: ea82 70e0    	eor.w	r0, r2, r0, asr #31
 8004b38: 4408         	add	r0, r1
 8004b3a: eba9 0106    	sub.w	r1, r9, r6
 8004b3e: f845 6c30    	str	r6, [r5, #-48]
 8004b42: ea4f 0241    	lsl.w	r2, r1, #0x1
 8004b46: ea82 71e1    	eor.w	r1, r2, r1, asr #31
 8004b4a: 4408         	add	r0, r1
 8004b4c: ebab 0109    	sub.w	r1, r11, r9
 8004b50: f845 9c18    	str	r9, [r5, #-24]
 8004b54: ea4f 0241    	lsl.w	r2, r1, #0x1
 8004b58: ea82 71e1    	eor.w	r1, r2, r1, asr #31
 8004b5c: f8c5 b000    	str.w	r11, [r5]
 8004b60: eb00 0e01    	add.w	lr, r0, r1
 8004b64: f505 7584    	add.w	r5, r5, #0x108
 8004b68: f47f af59    	bne.w	0x8004a1e <raylar_crcl_encoder_service::benchmark::fused::h66edd02a0d627444+0x488> @ imm = #-0x14e
 8004b6c: f8cd e024    	str.w	lr, [sp, #0x24]
 8004b70: a809         	add	r0, sp, #0x24
 8004b72: 2601         	movs	r6, #0x1
 8004b74: f04f 0900    	mov.w	r9, #0x0
 8004b78: 9809         	ldr	r0, [sp, #0x24]
 8004b7a: 9000         	str	r0, [sp]
 8004b7c: 9805         	ldr	r0, [sp, #0x14]
 8004b7e: 6a00         	ldr	r0, [r0, #0x20]
 8004b80: 9008         	str	r0, [sp, #0x20]
 8004b82: a808         	add	r0, sp, #0x20
 8004b84: 9808         	ldr	r0, [sp, #0x20]
 8004b86: f8dd e010    	ldr.w	lr, [sp, #0x10]
 8004b8a: 1e42         	subs	r2, r0, #0x1
 8004b8c: f000 051f    	and	r5, r0, #0x1f
 8004b90: f002 021f    	and	r2, r2, #0x1f
 8004b94: f240 30ff    	movw	r0, #0x3ff
 8004b98: f85e 1f10    	ldr	r1, [lr, #16]!
 8004b9c: fa06 f402    	lsl.w	r4, r6, r2
 8004ba0: 4421         	add	r1, r4
 8004ba2: fa41 fc05    	asr.w	r12, r1, r5
 8004ba6: f8ce c000    	str.w	r12, [lr]
 8004baa: f8de 3030    	ldr.w	r3, [lr, #0x30]
 8004bae: f8de 6048    	ldr.w	r6, [lr, #0x48]
 8004bb2: 4423         	add	r3, r4
 8004bb4: f8de a018    	ldr.w	r10, [lr, #0x18]
 8004bb8: 4426         	add	r6, r4
 8004bba: 9006         	str	r0, [sp, #0x18]
 8004bbc: 412b         	asrs	r3, r5
 8004bbe: f8de 8060    	ldr.w	r8, [lr, #0x60]
 8004bc2: 412e         	asrs	r6, r5
 8004bc4: 1af2         	subs	r2, r6, r3
 8004bc6: f8ce 3030    	str.w	r3, [lr, #0x30]
 8004bca: 0050         	lsls	r0, r2, #0x1
 8004bcc: ea80 7be2    	eor.w	r11, r0, r2, asr #31
 8004bd0: eb0a 0204    	add.w	r2, r10, r4
 8004bd4: f8ce 6048    	str.w	r6, [lr, #0x48]
 8004bd8: 412a         	asrs	r2, r5
 8004bda: 1a9b         	subs	r3, r3, r2
 8004bdc: eba2 010c    	sub.w	r1, r2, r12
 8004be0: f8ce 2018    	str.w	r2, [lr, #0x18]
 8004be4: 0058         	lsls	r0, r3, #0x1
 8004be6: 004a         	lsls	r2, r1, #0x1
 8004be8: ea82 71e1    	eor.w	r1, r2, r1, asr #31
 8004bec: ea80 70e3    	eor.w	r0, r0, r3, asr #31
 8004bf0: 4449         	add	r1, r9
 8004bf2: 4408         	add	r0, r1
 8004bf4: f8de 1078    	ldr.w	r1, [lr, #0x78]
 8004bf8: eb08 0204    	add.w	r2, r8, r4
 8004bfc: 4458         	add	r0, r11
 8004bfe: 4421         	add	r1, r4
 8004c00: 412a         	asrs	r2, r5
 8004c02: 1b93         	subs	r3, r2, r6
 8004c04: 4129         	asrs	r1, r5
 8004c06: f8ce 2060    	str.w	r2, [lr, #0x60]
 8004c0a: 005e         	lsls	r6, r3, #0x1
 8004c0c: 1a8a         	subs	r2, r1, r2
 8004c0e: ea86 73e3    	eor.w	r3, r6, r3, asr #31
 8004c12: f8de 6090    	ldr.w	r6, [lr, #0x90]
 8004c16: 4418         	add	r0, r3
 8004c18: 0053         	lsls	r3, r2, #0x1
 8004c1a: ea83 72e2    	eor.w	r2, r3, r2, asr #31
 8004c1e: f8ce 1078    	str.w	r1, [lr, #0x78]
 8004c22: 4410         	add	r0, r2
 8004c24: 1932         	adds	r2, r6, r4
 8004c26: f8de 30a8    	ldr.w	r3, [lr, #0xa8]
 8004c2a: 412a         	asrs	r2, r5
 8004c2c: 1a51         	subs	r1, r2, r1
 8004c2e: f8ce 2090    	str.w	r2, [lr, #0x90]
 8004c32: 004e         	lsls	r6, r1, #0x1
 8004c34: ea86 71e1    	eor.w	r1, r6, r1, asr #31
 8004c38: eb00 0801    	add.w	r8, r0, r1
 8004c3c: 1919         	adds	r1, r3, r4
 8004c3e: f8de 60f0    	ldr.w	r6, [lr, #0xf0]
 8004c42: fa41 f305    	asr.w	r3, r1, r5
 8004c46: 1a99         	subs	r1, r3, r2
 8004c48: f8ce 30a8    	str.w	r3, [lr, #0xa8]
 8004c4c: 004a         	lsls	r2, r1, #0x1
 8004c4e: ea82 79e1    	eor.w	r9, r2, r1, asr #31
 8004c52: 1931         	adds	r1, r6, r4
 8004c54: 44c8         	add	r8, r9
 8004c56: fa41 f205    	asr.w	r2, r1, r5
 8004c5a: f8de 10d8    	ldr.w	r1, [lr, #0xd8]
 8004c5e: f8ce 20f0    	str.w	r2, [lr, #0xf0]
 8004c62: 4421         	add	r1, r4
 8004c64: fa41 f605    	asr.w	r6, r1, r5
 8004c68: f8de 10c0    	ldr.w	r1, [lr, #0xc0]
 8004c6c: f8ce 60d8    	str.w	r6, [lr, #0xd8]
 8004c70: 4421         	add	r1, r4
 8004c72: fa41 f005    	asr.w	r0, r1, r5
 8004c76: f8de 1108    	ldr.w	r1, [lr, #0x108]
 8004c7a: 1ac3         	subs	r3, r0, r3
 8004c7c: f8ce 00c0    	str.w	r0, [lr, #0xc0]
 8004c80: 4421         	add	r1, r4
 8004c82: 1a30         	subs	r0, r6, r0
 8004c84: fa41 fc05    	asr.w	r12, r1, r5
 8004c88: 0059         	lsls	r1, r3, #0x1
 8004c8a: ea81 71e3    	eor.w	r1, r1, r3, asr #31
 8004c8e: 0043         	lsls	r3, r0, #0x1
 8004c90: 4441         	add	r1, r8
 8004c92: ea83 70e0    	eor.w	r0, r3, r0, asr #31
 8004c96: 4408         	add	r0, r1
 8004c98: 1b91         	subs	r1, r2, r6
 8004c9a: f8ce c108    	str.w	r12, [lr, #0x108]
 8004c9e: f50e 7e84    	add.w	lr, lr, #0x108
 8004ca2: 004b         	lsls	r3, r1, #0x1
 8004ca4: ea83 71e1    	eor.w	r1, r3, r1, asr #31
 8004ca8: 4408         	add	r0, r1
 8004caa: ebac 0102    	sub.w	r1, r12, r2
 8004cae: 004a         	lsls	r2, r1, #0x1
 8004cb0: ea82 71e1    	eor.w	r1, r2, r1, asr #31
 8004cb4: eb00 0901    	add.w	r9, r0, r1
 8004cb8: 9806         	ldr	r0, [sp, #0x18]
 8004cba: 380b         	subs	r0, #0xb
 8004cbc: f47f af75    	bne.w	0x8004baa <raylar_crcl_encoder_service::benchmark::fused::h66edd02a0d627444+0x614> @ imm = #-0x116
 8004cc0: f8cd 9024    	str.w	r9, [sp, #0x24]
 8004cc4: a809         	add	r0, sp, #0x24
 8004cc6: f246 0128    	movw	r1, #0x6028
 8004cca: 9a05         	ldr	r2, [sp, #0x14]
 8004ccc: 9809         	ldr	r0, [sp, #0x24]
 8004cce: 4411         	add	r1, r2
 8004cd0: 9107         	str	r1, [sp, #0x1c]
 8004cd2: a907         	add	r1, sp, #0x1c
 8004cd4: e9dd 2102    	ldrd	r2, r1, [sp, #8]
 8004cd8: 4051         	eors	r1, r2
 8004cda: 9a01         	ldr	r2, [sp, #0x4]
 8004cdc: 4051         	eors	r1, r2
 8004cde: 9a00         	ldr	r2, [sp]
 8004ce0: 4051         	eors	r1, r2
 8004ce2: 4048         	eors	r0, r1
 8004ce4: b00a         	add	sp, #0x28
 8004ce6: e8bd 0f00    	pop.w	{r8, r9, r10, r11}
 8004cea: bdf0         	pop	{r4, r5, r6, r7, pc}

08004cec <raylar_crcl_encoder_service::benchmark::quant::h5a540275f2df62e3>:
 8004cec: b580         	push	{r7, lr}
 8004cee: 466f         	mov	r7, sp
 8004cf0: f246 0128    	movw	r1, #0x6028
 8004cf4: 1842         	adds	r2, r0, r1
 8004cf6: f24c 0128    	movw	r1, #0xc028
 8004cfa: 4401         	add	r1, r0
 8004cfc: 4610         	mov	r0, r2
 8004cfe: f44f 42c0    	mov.w	r2, #0x6000
 8004d02: f008 fae8    	bl	0x800d2d6 <__aeabi_memcpy4> @ imm = #0x85d0
 8004d06: bd80         	pop	{r7, pc}

08004d08 <raylar_crcl_encoder_service::benchmark::scale::h44b3d035bda7af78>:
 8004d08: b5f0         	push	{r4, r5, r6, r7, lr}
 8004d0a: af03         	add	r7, sp, #0xc
 8004d0c: e92d 0f00    	push.w	{r8, r9, r10, r11}
 8004d10: b086         	sub	sp, #0x18
 8004d12: f10d 0e0c    	add.w	lr, sp, #0xc
 8004d16: f1a7 0122    	sub.w	r1, r7, #0x22
 8004d1a: f1a7 0b21    	sub.w	r11, r7, #0x21
 8004d1e: 2000         	movs	r0, #0x0
 8004d20: f44f 7380    	mov.w	r3, #0x100
 8004d24: f04f 080a    	mov.w	r8, #0xa
 8004d28: f04f 0c06    	mov.w	r12, #0x6
 8004d2c: e030         	b	0x8004d90 <raylar_crcl_encoder_service::benchmark::scale::h44b3d035bda7af78+0x88> @ imm = #0x60
 8004d2e: eba5 0806    	sub.w	r8, r5, r6
 8004d32: 2101         	movs	r1, #0x1
 8004d34: f1a8 0501    	sub.w	r5, r8, #0x1
 8004d38: f003 031f    	and	r3, r3, #0x1f
 8004d3c: fa01 f505    	lsl.w	r5, r1, r5
 8004d40: 442a         	add	r2, r5
 8004d42: f008 051f    	and	r5, r8, #0x1f
 8004d46: 40ea         	lsrs	r2, r5
 8004d48: 40da         	lsrs	r2, r3
 8004d4a: bf18         	it	ne
 8004d4c: f108 0801    	addne.w	r8, r8, #0x1
 8004d50: e9dd 1301    	ldrd	r1, r3, [sp, #4]
 8004d54: 46a6         	mov	lr, r4
 8004d56: 4048         	eors	r0, r1
 8004d58: 9900         	ldr	r1, [sp]
 8004d5a: 4048         	eors	r0, r1
 8004d5c: a905         	add	r1, sp, #0x14
 8004d5e: ea80 000b    	eor.w	r0, r0, r11
 8004d62: 46cb         	mov	r11, r9
 8004d64: ea80 000c    	eor.w	r0, r0, r12
 8004d68: f04f 0c06    	mov.w	r12, #0x6
 8004d6c: f807 cc21    	strb	r12, [r7, #-33]
 8004d70: f817 2c21    	ldrb	r2, [r7, #-33]
 8004d74: 4542         	cmp	r2, r8
 8004d76: bf88         	it	hi
 8004d78: 4690         	movhi	r8, r2
 8004d7a: f8cd 8014    	str.w	r8, [sp, #0x14]
 8004d7e: 3b01         	subs	r3, #0x1
 8004d80: f04f 080a    	mov.w	r8, #0xa
 8004d84: 4651         	mov	r1, r10
 8004d86: 9a05         	ldr	r2, [sp, #0x14]
 8004d88: ea80 0002    	eor.w	r0, r0, r2
 8004d8c: f000 80c2    	beq.w	0x8004f14 <raylar_crcl_encoder_service::benchmark::scale::h44b3d035bda7af78+0x20c> @ imm = #0x184
 8004d90: f245 2288    	movw	r2, #0x5288
 8004d94: 9302         	str	r3, [sp, #0x8]
 8004d96: 9203         	str	r2, [sp, #0xc]
 8004d98: f807 8c22    	strb	r8, [r7, #-34]
 8004d9c: 9b03         	ldr	r3, [sp, #0xc]
 8004d9e: f817 6c22    	ldrb	r6, [r7, #-34]
 8004da2: fab3 f283    	clz	r2, r3
 8004da6: 1e75         	subs	r5, r6, #0x1
 8004da8: f1c2 0220    	rsb.w	r2, r2, #0x20
 8004dac: b2ee         	uxtb	r6, r5
 8004dae: 42b2         	cmp	r2, r6
 8004db0: d90e         	bls	0x8004dd0 <raylar_crcl_encoder_service::benchmark::scale::h44b3d035bda7af78+0xc8> @ imm = #0x1c
 8004db2: 1b92         	subs	r2, r2, r6
 8004db4: 2401         	movs	r4, #0x1
 8004db6: 1e56         	subs	r6, r2, #0x1
 8004db8: fa04 f606    	lsl.w	r6, r4, r6
 8004dbc: 4433         	add	r3, r6
 8004dbe: f002 061f    	and	r6, r2, #0x1f
 8004dc2: 40f3         	lsrs	r3, r6
 8004dc4: f005 061f    	and	r6, r5, #0x1f
 8004dc8: 40f3         	lsrs	r3, r6
 8004dca: bf18         	it	ne
 8004dcc: 3201         	addne	r2, #0x1
 8004dce: e000         	b	0x8004dd2 <raylar_crcl_encoder_service::benchmark::scale::h44b3d035bda7af78+0xca> @ imm = #0x0
 8004dd0: 2200         	movs	r2, #0x0
 8004dd2: f807 cc21    	strb	r12, [r7, #-33]
 8004dd6: f10d 0a14    	add.w	r10, sp, #0x14
 8004dda: f817 3c21    	ldrb	r3, [r7, #-33]
 8004dde: 4293         	cmp	r3, r2
 8004de0: bf88         	it	hi
 8004de2: 461a         	movhi	r2, r3
 8004de4: 9205         	str	r2, [sp, #0x14]
 8004de6: 9a05         	ldr	r2, [sp, #0x14]
 8004de8: 9201         	str	r2, [sp, #0x4]
 8004dea: f245 6270    	movw	r2, #0x5670
 8004dee: 9203         	str	r2, [sp, #0xc]
 8004df0: f807 8c22    	strb	r8, [r7, #-34]
 8004df4: 9b03         	ldr	r3, [sp, #0xc]
 8004df6: f817 6c22    	ldrb	r6, [r7, #-34]
 8004dfa: fab3 f283    	clz	r2, r3
 8004dfe: 1e75         	subs	r5, r6, #0x1
 8004e00: f1c2 0220    	rsb.w	r2, r2, #0x20
 8004e04: b2ee         	uxtb	r6, r5
 8004e06: 42b2         	cmp	r2, r6
 8004e08: d90e         	bls	0x8004e28 <raylar_crcl_encoder_service::benchmark::scale::h44b3d035bda7af78+0x120> @ imm = #0x1c
 8004e0a: 1b92         	subs	r2, r2, r6
 8004e0c: 2401         	movs	r4, #0x1
 8004e0e: 1e56         	subs	r6, r2, #0x1
 8004e10: fa04 f606    	lsl.w	r6, r4, r6
 8004e14: 4433         	add	r3, r6
 8004e16: f002 061f    	and	r6, r2, #0x1f
 8004e1a: 40f3         	lsrs	r3, r6
 8004e1c: f005 061f    	and	r6, r5, #0x1f
 8004e20: 40f3         	lsrs	r3, r6
 8004e22: bf18         	it	ne
 8004e24: 3201         	addne	r2, #0x1
 8004e26: e000         	b	0x8004e2a <raylar_crcl_encoder_service::benchmark::scale::h44b3d035bda7af78+0x122> @ imm = #0x0
 8004e28: 2200         	movs	r2, #0x0
 8004e2a: f807 cc21    	strb	r12, [r7, #-33]
 8004e2e: f817 3c21    	ldrb	r3, [r7, #-33]
 8004e32: 4293         	cmp	r3, r2
 8004e34: bf88         	it	hi
 8004e36: 461a         	movhi	r2, r3
 8004e38: 9205         	str	r2, [sp, #0x14]
 8004e3a: 9a05         	ldr	r2, [sp, #0x14]
 8004e3c: 9200         	str	r2, [sp]
 8004e3e: f645 2258    	movw	r2, #0x5a58
 8004e42: 9203         	str	r2, [sp, #0xc]
 8004e44: f807 8c22    	strb	r8, [r7, #-34]
 8004e48: 9b03         	ldr	r3, [sp, #0xc]
 8004e4a: f817 6c22    	ldrb	r6, [r7, #-34]
 8004e4e: fab3 f283    	clz	r2, r3
 8004e52: 1e75         	subs	r5, r6, #0x1
 8004e54: f1c2 0220    	rsb.w	r2, r2, #0x20
 8004e58: b2ee         	uxtb	r6, r5
 8004e5a: 42b2         	cmp	r2, r6
 8004e5c: d90e         	bls	0x8004e7c <raylar_crcl_encoder_service::benchmark::scale::h44b3d035bda7af78+0x174> @ imm = #0x1c
 8004e5e: 1b92         	subs	r2, r2, r6
 8004e60: 2401         	movs	r4, #0x1
 8004e62: 1e56         	subs	r6, r2, #0x1
 8004e64: fa04 f606    	lsl.w	r6, r4, r6
 8004e68: 4433         	add	r3, r6
 8004e6a: f002 061f    	and	r6, r2, #0x1f
 8004e6e: 40f3         	lsrs	r3, r6
 8004e70: f005 061f    	and	r6, r5, #0x1f
 8004e74: 40f3         	lsrs	r3, r6
 8004e76: bf18         	it	ne
 8004e78: 3201         	addne	r2, #0x1
 8004e7a: e000         	b	0x8004e7e <raylar_crcl_encoder_service::benchmark::scale::h44b3d035bda7af78+0x176> @ imm = #0x0
 8004e7c: 2200         	movs	r2, #0x0
 8004e7e: f807 cc21    	strb	r12, [r7, #-33]
 8004e82: 46d9         	mov	r9, r11
 8004e84: f817 3c21    	ldrb	r3, [r7, #-33]
 8004e88: 4293         	cmp	r3, r2
 8004e8a: bf88         	it	hi
 8004e8c: 461a         	movhi	r2, r3
 8004e8e: 9205         	str	r2, [sp, #0x14]
 8004e90: f645 6240    	movw	r2, #0x5e40
 8004e94: 9203         	str	r2, [sp, #0xc]
 8004e96: f8dd b014    	ldr.w	r11, [sp, #0x14]
 8004e9a: f807 8c22    	strb	r8, [r7, #-34]
 8004e9e: 9b03         	ldr	r3, [sp, #0xc]
 8004ea0: f817 6c22    	ldrb	r6, [r7, #-34]
 8004ea4: fab3 f283    	clz	r2, r3
 8004ea8: 1e75         	subs	r5, r6, #0x1
 8004eaa: f1c2 0220    	rsb.w	r2, r2, #0x20
 8004eae: b2ee         	uxtb	r6, r5
 8004eb0: 42b2         	cmp	r2, r6
 8004eb2: d90e         	bls	0x8004ed2 <raylar_crcl_encoder_service::benchmark::scale::h44b3d035bda7af78+0x1ca> @ imm = #0x1c
 8004eb4: 1b92         	subs	r2, r2, r6
 8004eb6: 2401         	movs	r4, #0x1
 8004eb8: 1e56         	subs	r6, r2, #0x1
 8004eba: fa04 f606    	lsl.w	r6, r4, r6
 8004ebe: 4433         	add	r3, r6
 8004ec0: f002 061f    	and	r6, r2, #0x1f
 8004ec4: 40f3         	lsrs	r3, r6
 8004ec6: f005 061f    	and	r6, r5, #0x1f
 8004eca: 40f3         	lsrs	r3, r6
 8004ecc: bf18         	it	ne
 8004ece: 3201         	addne	r2, #0x1
 8004ed0: e000         	b	0x8004ed4 <raylar_crcl_encoder_service::benchmark::scale::h44b3d035bda7af78+0x1cc> @ imm = #0x0
 8004ed2: 2200         	movs	r2, #0x0
 8004ed4: f807 cc21    	strb	r12, [r7, #-33]
 8004ed8: 4674         	mov	r4, lr
 8004eda: f817 3c21    	ldrb	r3, [r7, #-33]
 8004ede: 4293         	cmp	r3, r2
 8004ee0: bf88         	it	hi
 8004ee2: 461a         	movhi	r2, r3
 8004ee4: 9205         	str	r2, [sp, #0x14]
 8004ee6: f246 2228    	movw	r2, #0x6228
 8004eea: 9203         	str	r2, [sp, #0xc]
 8004eec: 468a         	mov	r10, r1
 8004eee: f8dd c014    	ldr.w	r12, [sp, #0x14]
 8004ef2: f807 8c22    	strb	r8, [r7, #-34]
 8004ef6: 9a03         	ldr	r2, [sp, #0xc]
 8004ef8: f817 3c22    	ldrb	r3, [r7, #-34]
 8004efc: fab2 f582    	clz	r5, r2
 8004f00: 3b01         	subs	r3, #0x1
 8004f02: f1c5 0520    	rsb.w	r5, r5, #0x20
 8004f06: b2de         	uxtb	r6, r3
 8004f08: 42b5         	cmp	r5, r6
 8004f0a: f63f af10    	bhi.w	0x8004d2e <raylar_crcl_encoder_service::benchmark::scale::h44b3d035bda7af78+0x26> @ imm = #-0x1e0
 8004f0e: f04f 0800    	mov.w	r8, #0x0
 8004f12: e71d         	b	0x8004d50 <raylar_crcl_encoder_service::benchmark::scale::h44b3d035bda7af78+0x48> @ imm = #-0x1c6
 8004f14: b006         	add	sp, #0x18
 8004f16: e8bd 0f00    	pop.w	{r8, r9, r10, r11}
 8004f1a: bdf0         	pop	{r4, r5, r6, r7, pc}

08004f1c <raylar_crcl_encoder_service::benchmark::rice_k::h2637d4bce634509f>:
 8004f1c: b5f0         	push	{r4, r5, r6, r7, lr}
 8004f1e: af03         	add	r7, sp, #0xc
 8004f20: e92d 0f00    	push.w	{r8, r9, r10, r11}
 8004f24: b08a         	sub	sp, #0x28
 8004f26: f64f 41b0    	movw	r1, #0xfcb0
 8004f2a: f10d 081c    	add.w	r8, sp, #0x1c
 8004f2e: f2c0 0101    	movt	r1, #0x1
 8004f32: 1842         	adds	r2, r0, r1
 8004f34: 5840         	ldr	r0, [r0, r1]
 8004f36: f10d 0e20    	add.w	lr, sp, #0x20
 8004f3a: 6851         	ldr	r1, [r2, #0x4]
 8004f3c: 9103         	str	r1, [sp, #0xc]
 8004f3e: ac09         	add	r4, sp, #0x24
 8004f40: 6891         	ldr	r1, [r2, #0x8]
 8004f42: f44f 7980    	mov.w	r9, #0x100
 8004f46: 9102         	str	r1, [sp, #0x8]
 8004f48: f240 3bff    	movw	r11, #0x3ff
 8004f4c: 68d1         	ldr	r1, [r2, #0xc]
 8004f4e: 9004         	str	r0, [sp, #0x10]
 8004f50: 2000         	movs	r0, #0x0
 8004f52: 9101         	str	r1, [sp, #0x4]
 8004f54: 6911         	ldr	r1, [r2, #0x10]
 8004f56: 9100         	str	r1, [sp]
 8004f58: e017         	b	0x8004f8a <raylar_crcl_encoder_service::benchmark::rice_k::h2637d4bce634509f+0x6e> @ imm = #0x2e
 8004f5a: 9906         	ldr	r1, [sp, #0x18]
 8004f5c: f1b9 0901    	subs.w	r9, r9, #0x1
 8004f60: f8cd 8024    	str.w	r8, [sp, #0x24]
 8004f64: 462c         	mov	r4, r5
 8004f66: ea80 0001    	eor.w	r0, r0, r1
 8004f6a: 9905         	ldr	r1, [sp, #0x14]
 8004f6c: 46d0         	mov	r8, r10
 8004f6e: 4693         	mov	r11, r2
 8004f70: ea80 0001    	eor.w	r0, r0, r1
 8004f74: 9909         	ldr	r1, [sp, #0x24]
 8004f76: ea80 000c    	eor.w	r0, r0, r12
 8004f7a: ea80 000e    	eor.w	r0, r0, lr
 8004f7e: f10d 0e20    	add.w	lr, sp, #0x20
 8004f82: ea80 0001    	eor.w	r0, r0, r1
 8004f86: f000 8087    	beq.w	0x8005098 <raylar_crcl_encoder_service::benchmark::rice_k::h2637d4bce634509f+0x17c> @ imm = #0x10e
 8004f8a: 9904         	ldr	r1, [sp, #0x10]
 8004f8c: 2200         	movs	r2, #0x0
 8004f8e: 9107         	str	r1, [sp, #0x1c]
 8004f90: f8cd b020    	str.w	r11, [sp, #0x20]
 8004f94: 9e07         	ldr	r6, [sp, #0x1c]
 8004f96: 2e00         	cmp	r6, #0x0
 8004f98: bf1c         	itt	ne
 8004f9a: 9b08         	ldrne	r3, [sp, #0x20]
 8004f9c: 2b00         	cmpne	r3, #0x0
 8004f9e: d00b         	beq	0x8004fb8 <raylar_crcl_encoder_service::benchmark::rice_k::h2637d4bce634509f+0x9c> @ imm = #0x16
 8004fa0: fbb6 f1f3    	udiv	r1, r6, r3
 8004fa4: fb01 6213    	mls	r2, r1, r3, r6
 8004fa8: 2a00         	cmp	r2, #0x0
 8004faa: bf18         	it	ne
 8004fac: 3101         	addne	r1, #0x1
 8004fae: fab1 f181    	clz	r1, r1
 8004fb2: f1c1 011f    	rsb.w	r1, r1, #0x1f
 8004fb6: b2ca         	uxtb	r2, r1
 8004fb8: 9209         	str	r2, [sp, #0x24]
 8004fba: 2500         	movs	r5, #0x0
 8004fbc: 9909         	ldr	r1, [sp, #0x24]
 8004fbe: 9106         	str	r1, [sp, #0x18]
 8004fc0: 9903         	ldr	r1, [sp, #0xc]
 8004fc2: 9107         	str	r1, [sp, #0x1c]
 8004fc4: f8cd b020    	str.w	r11, [sp, #0x20]
 8004fc8: 9e07         	ldr	r6, [sp, #0x1c]
 8004fca: 2e00         	cmp	r6, #0x0
 8004fcc: bf1c         	itt	ne
 8004fce: 9b08         	ldrne	r3, [sp, #0x20]
 8004fd0: 2b00         	cmpne	r3, #0x0
 8004fd2: d00b         	beq	0x8004fec <raylar_crcl_encoder_service::benchmark::rice_k::h2637d4bce634509f+0xd0> @ imm = #0x16
 8004fd4: fbb6 f1f3    	udiv	r1, r6, r3
 8004fd8: fb01 6313    	mls	r3, r1, r3, r6
 8004fdc: 2b00         	cmp	r3, #0x0
 8004fde: bf18         	it	ne
 8004fe0: 3101         	addne	r1, #0x1
 8004fe2: fab1 f181    	clz	r1, r1
 8004fe6: f1c1 011f    	rsb.w	r1, r1, #0x1f
 8004fea: b2cd         	uxtb	r5, r1
 8004fec: 9509         	str	r5, [sp, #0x24]
 8004fee: 2600         	movs	r6, #0x0
 8004ff0: 9909         	ldr	r1, [sp, #0x24]
 8004ff2: 9105         	str	r1, [sp, #0x14]
 8004ff4: 9902         	ldr	r1, [sp, #0x8]
 8004ff6: 9107         	str	r1, [sp, #0x1c]
 8004ff8: f8cd b020    	str.w	r11, [sp, #0x20]
 8004ffc: 9b07         	ldr	r3, [sp, #0x1c]
 8004ffe: 2b00         	cmp	r3, #0x0
 8005000: bf1c         	itt	ne
 8005002: 9908         	ldrne	r1, [sp, #0x20]
 8005004: 2900         	cmpne	r1, #0x0
 8005006: d00b         	beq	0x8005020 <raylar_crcl_encoder_service::benchmark::rice_k::h2637d4bce634509f+0x104> @ imm = #0x16
 8005008: fbb3 f6f1    	udiv	r6, r3, r1
 800500c: fb06 3111    	mls	r1, r6, r1, r3
 8005010: 2900         	cmp	r1, #0x0
 8005012: bf18         	it	ne
 8005014: 3601         	addne	r6, #0x1
 8005016: fab6 f186    	clz	r1, r6
 800501a: f1c1 011f    	rsb.w	r1, r1, #0x1f
 800501e: b2ce         	uxtb	r6, r1
 8005020: 9609         	str	r6, [sp, #0x24]
 8005022: 4625         	mov	r5, r4
 8005024: 2600         	movs	r6, #0x0
 8005026: 9901         	ldr	r1, [sp, #0x4]
 8005028: 9107         	str	r1, [sp, #0x1c]
 800502a: f8dd c024    	ldr.w	r12, [sp, #0x24]
 800502e: 9b07         	ldr	r3, [sp, #0x1c]
 8005030: f8cd b020    	str.w	r11, [sp, #0x20]
 8005034: 2b00         	cmp	r3, #0x0
 8005036: bf1c         	itt	ne
 8005038: 9908         	ldrne	r1, [sp, #0x20]
 800503a: 2900         	cmpne	r1, #0x0
 800503c: d00b         	beq	0x8005056 <raylar_crcl_encoder_service::benchmark::rice_k::h2637d4bce634509f+0x13a> @ imm = #0x16
 800503e: fbb3 f6f1    	udiv	r6, r3, r1
 8005042: fb06 3111    	mls	r1, r6, r1, r3
 8005046: 2900         	cmp	r1, #0x0
 8005048: bf18         	it	ne
 800504a: 3601         	addne	r6, #0x1
 800504c: fab6 f186    	clz	r1, r6
 8005050: f1c1 011f    	rsb.w	r1, r1, #0x1f
 8005054: b2ce         	uxtb	r6, r1
 8005056: 9609         	str	r6, [sp, #0x24]
 8005058: 46c2         	mov	r10, r8
 800505a: 465a         	mov	r2, r11
 800505c: 9900         	ldr	r1, [sp]
 800505e: 9107         	str	r1, [sp, #0x1c]
 8005060: a908         	add	r1, sp, #0x20
 8005062: f8dd e024    	ldr.w	lr, [sp, #0x24]
 8005066: f04f 0800    	mov.w	r8, #0x0
 800506a: 9b07         	ldr	r3, [sp, #0x1c]
 800506c: f8cd b020    	str.w	r11, [sp, #0x20]
 8005070: 2b00         	cmp	r3, #0x0
 8005072: bf1c         	itt	ne
 8005074: 9908         	ldrne	r1, [sp, #0x20]
 8005076: 2900         	cmpne	r1, #0x0
 8005078: f43f af6f    	beq.w	0x8004f5a <raylar_crcl_encoder_service::benchmark::rice_k::h2637d4bce634509f+0x3e> @ imm = #-0x122
 800507c: fbb3 f6f1    	udiv	r6, r3, r1
 8005080: fb06 3111    	mls	r1, r6, r1, r3
 8005084: 2900         	cmp	r1, #0x0
 8005086: bf18         	it	ne
 8005088: 3601         	addne	r6, #0x1
 800508a: fab6 f186    	clz	r1, r6
 800508e: f1c1 011f    	rsb.w	r1, r1, #0x1f
 8005092: fa5f f881    	uxtb.w	r8, r1
 8005096: e760         	b	0x8004f5a <raylar_crcl_encoder_service::benchmark::rice_k::h2637d4bce634509f+0x3e> @ imm = #-0x140
 8005098: b00a         	add	sp, #0x28
 800509a: e8bd 0f00    	pop.w	{r8, r9, r10, r11}
 800509e: bdf0         	pop	{r4, r5, r6, r7, pc}

080050a0 <raylar_crcl_encoder_service::benchmark::dcblock::h5d1170af065dd1eb>:
 80050a0: b5f0         	push	{r4, r5, r6, r7, lr}
 80050a2: af03         	add	r7, sp, #0xc
 80050a4: e92d 0f00    	push.w	{r8, r9, r10, r11}
 80050a8: b083         	sub	sp, #0xc
 80050aa: f645 716c    	movw	r1, #0x5f6c
 80050ae: 2600         	movs	r6, #0x0
 80050b0: 4401         	add	r1, r0
 80050b2: f44f 6280    	mov.w	r2, #0x400
 80050b6: 2300         	movs	r3, #0x0
 80050b8: 9001         	str	r0, [sp, #0x4]
 80050ba: eb03 2326    	add.w	r3, r3, r6, asr #8
 80050be: f8d1 00c0    	ldr.w	r0, [r1, #0xc0]
 80050c2: 1af3         	subs	r3, r6, r3
 80050c4: f8d1 50d8    	ldr.w	r5, [r1, #0xd8]
 80050c8: 4418         	add	r0, r3
 80050ca: f8c1 00c0    	str.w	r0, [r1, #0xc0]
 80050ce: f8d1 40f0    	ldr.w	r4, [r1, #0xf0]
 80050d2: 3a10         	subs	r2, #0x10
 80050d4: eba3 2020    	sub.w	r0, r3, r0, asr #8
 80050d8: f8d1 8108    	ldr.w	r8, [r1, #0x108]
 80050dc: eb00 0305    	add.w	r3, r0, r5
 80050e0: f8c1 30d8    	str.w	r3, [r1, #0xd8]
 80050e4: f8d1 6120    	ldr.w	r6, [r1, #0x120]
 80050e8: eba0 2023    	sub.w	r0, r0, r3, asr #8
 80050ec: f8d1 5168    	ldr.w	r5, [r1, #0x168]
 80050f0: eb00 0304    	add.w	r3, r0, r4
 80050f4: f8c1 30f0    	str.w	r3, [r1, #0xf0]
 80050f8: f8d1 4150    	ldr.w	r4, [r1, #0x150]
 80050fc: eba0 2023    	sub.w	r0, r0, r3, asr #8
 8005100: f8d1 e180    	ldr.w	lr, [r1, #0x180]
 8005104: eb00 0308    	add.w	r3, r0, r8
 8005108: f8c1 3108    	str.w	r3, [r1, #0x108]
 800510c: f8d1 c198    	ldr.w	r12, [r1, #0x198]
 8005110: eba0 2023    	sub.w	r0, r0, r3, asr #8
 8005114: eb00 0306    	add.w	r3, r0, r6
 8005118: f8d1 6138    	ldr.w	r6, [r1, #0x138]
 800511c: f8c1 3120    	str.w	r3, [r1, #0x120]
 8005120: eba0 2023    	sub.w	r0, r0, r3, asr #8
 8005124: eb00 0306    	add.w	r3, r0, r6
 8005128: f8c1 3138    	str.w	r3, [r1, #0x138]
 800512c: f8d1 61b0    	ldr.w	r6, [r1, #0x1b0]
 8005130: eba0 2023    	sub.w	r0, r0, r3, asr #8
 8005134: eb00 0304    	add.w	r3, r0, r4
 8005138: f8c1 3150    	str.w	r3, [r1, #0x150]
 800513c: eba0 2023    	sub.w	r0, r0, r3, asr #8
 8005140: eb00 0305    	add.w	r3, r0, r5
 8005144: f8c1 3168    	str.w	r3, [r1, #0x168]
 8005148: eba0 2023    	sub.w	r0, r0, r3, asr #8
 800514c: eb00 030e    	add.w	r3, r0, lr
 8005150: f8c1 3180    	str.w	r3, [r1, #0x180]
 8005154: eba0 2023    	sub.w	r0, r0, r3, asr #8
 8005158: eb00 030c    	add.w	r3, r0, r12
 800515c: f8c1 3198    	str.w	r3, [r1, #0x198]
 8005160: eba0 2023    	sub.w	r0, r0, r3, asr #8
 8005164: eb00 0306    	add.w	r3, r0, r6
 8005168: f8d1 61c8    	ldr.w	r6, [r1, #0x1c8]
 800516c: f8c1 31b0    	str.w	r3, [r1, #0x1b0]
 8005170: eba0 2023    	sub.w	r0, r0, r3, asr #8
 8005174: eb00 0306    	add.w	r3, r0, r6
 8005178: f8d1 61e0    	ldr.w	r6, [r1, #0x1e0]
 800517c: f8c1 31c8    	str.w	r3, [r1, #0x1c8]
 8005180: eba0 2023    	sub.w	r0, r0, r3, asr #8
 8005184: eb00 0306    	add.w	r3, r0, r6
 8005188: f8d1 61f8    	ldr.w	r6, [r1, #0x1f8]
 800518c: f8c1 31e0    	str.w	r3, [r1, #0x1e0]
 8005190: eba0 2023    	sub.w	r0, r0, r3, asr #8
 8005194: eb00 0306    	add.w	r3, r0, r6
 8005198: f8d1 6210    	ldr.w	r6, [r1, #0x210]
 800519c: f8c1 31f8    	str.w	r3, [r1, #0x1f8]
 80051a0: eba0 2023    	sub.w	r0, r0, r3, asr #8
 80051a4: f8d1 3228    	ldr.w	r3, [r1, #0x228]
 80051a8: 4406         	add	r6, r0
 80051aa: f8c1 6210    	str.w	r6, [r1, #0x210]
 80051ae: eba0 2026    	sub.w	r0, r0, r6, asr #8
 80051b2: eb00 0603    	add.w	r6, r0, r3
 80051b6: f8c1 6228    	str.w	r6, [r1, #0x228]
 80051ba: f501 71c0    	add.w	r1, r1, #0x180
 80051be: f47f af7c    	bne.w	0x80050ba <raylar_crcl_encoder_service::benchmark::dcblock::h5d1170af065dd1eb+0x1a> @ imm = #-0x108
 80051c2: 9801         	ldr	r0, [sp, #0x4]
 80051c4: f645 61b0    	movw	r1, #0x5eb0
 80051c8: f04f 0e00    	mov.w	lr, #0x0
 80051cc: f44f 6280    	mov.w	r2, #0x400
 80051d0: 4401         	add	r1, r0
 80051d2: 2300         	movs	r3, #0x0
 80051d4: 9600         	str	r6, [sp]
 80051d6: eb03 232e    	add.w	r3, r3, lr, asr #8
 80051da: f8d1 4180    	ldr.w	r4, [r1, #0x180]
 80051de: ebae 0303    	sub.w	r3, lr, r3
 80051e2: f8d1 5198    	ldr.w	r5, [r1, #0x198]
 80051e6: 441c         	add	r4, r3
 80051e8: f8c1 4180    	str.w	r4, [r1, #0x180]
 80051ec: f8d1 61b0    	ldr.w	r6, [r1, #0x1b0]
 80051f0: 3a10         	subs	r2, #0x10
 80051f2: eba3 2324    	sub.w	r3, r3, r4, asr #8
 80051f6: f8d1 81c8    	ldr.w	r8, [r1, #0x1c8]
 80051fa: eb03 0405    	add.w	r4, r3, r5
 80051fe: f8c1 4198    	str.w	r4, [r1, #0x198]
 8005202: f8d1 51e0    	ldr.w	r5, [r1, #0x1e0]
 8005206: eba3 2324    	sub.w	r3, r3, r4, asr #8
 800520a: f8d1 a228    	ldr.w	r10, [r1, #0x228]
 800520e: eb03 0406    	add.w	r4, r3, r6
 8005212: f8c1 41b0    	str.w	r4, [r1, #0x1b0]
 8005216: f8d1 6210    	ldr.w	r6, [r1, #0x210]
 800521a: eba3 2324    	sub.w	r3, r3, r4, asr #8
 800521e: f8d1 e240    	ldr.w	lr, [r1, #0x240]
 8005222: eb03 0408    	add.w	r4, r3, r8
 8005226: f8c1 41c8    	str.w	r4, [r1, #0x1c8]
 800522a: f8d1 9258    	ldr.w	r9, [r1, #0x258]
 800522e: eba3 2324    	sub.w	r3, r3, r4, asr #8
 8005232: eb03 0405    	add.w	r4, r3, r5
 8005236: f8d1 51f8    	ldr.w	r5, [r1, #0x1f8]
 800523a: f8c1 41e0    	str.w	r4, [r1, #0x1e0]
 800523e: eba3 2324    	sub.w	r3, r3, r4, asr #8
 8005242: eb03 0405    	add.w	r4, r3, r5
 8005246: f8c1 41f8    	str.w	r4, [r1, #0x1f8]
 800524a: f8d1 5270    	ldr.w	r5, [r1, #0x270]
 800524e: eba3 2324    	sub.w	r3, r3, r4, asr #8
 8005252: eb03 0406    	add.w	r4, r3, r6
 8005256: f8c1 4210    	str.w	r4, [r1, #0x210]
 800525a: eba3 2324    	sub.w	r3, r3, r4, asr #8
 800525e: eb03 040a    	add.w	r4, r3, r10
 8005262: f8c1 4228    	str.w	r4, [r1, #0x228]
 8005266: eba3 2324    	sub.w	r3, r3, r4, asr #8
 800526a: eb03 060e    	add.w	r6, r3, lr
 800526e: f8c1 6240    	str.w	r6, [r1, #0x240]
 8005272: eba3 2326    	sub.w	r3, r3, r6, asr #8
 8005276: eb03 0609    	add.w	r6, r3, r9
 800527a: f8c1 6258    	str.w	r6, [r1, #0x258]
 800527e: eba3 2326    	sub.w	r3, r3, r6, asr #8
 8005282: eb03 0605    	add.w	r6, r3, r5
 8005286: f8d1 5288    	ldr.w	r5, [r1, #0x288]
 800528a: f8c1 6270    	str.w	r6, [r1, #0x270]
 800528e: eba3 2326    	sub.w	r3, r3, r6, asr #8
 8005292: eb03 0605    	add.w	r6, r3, r5
 8005296: f8d1 52a0    	ldr.w	r5, [r1, #0x2a0]
 800529a: f8c1 6288    	str.w	r6, [r1, #0x288]
 800529e: eba3 2326    	sub.w	r3, r3, r6, asr #8
 80052a2: eb03 0605    	add.w	r6, r3, r5
 80052a6: f8d1 52b8    	ldr.w	r5, [r1, #0x2b8]
 80052aa: f8c1 62a0    	str.w	r6, [r1, #0x2a0]
 80052ae: eba3 2326    	sub.w	r3, r3, r6, asr #8
 80052b2: eb03 0605    	add.w	r6, r3, r5
 80052b6: f8d1 52d0    	ldr.w	r5, [r1, #0x2d0]
 80052ba: f8c1 62b8    	str.w	r6, [r1, #0x2b8]
 80052be: eba3 2626    	sub.w	r6, r3, r6, asr #8
 80052c2: f8d1 32e8    	ldr.w	r3, [r1, #0x2e8]
 80052c6: 4435         	add	r5, r6
 80052c8: f8c1 52d0    	str.w	r5, [r1, #0x2d0]
 80052cc: eba6 2625    	sub.w	r6, r6, r5, asr #8
 80052d0: eb06 0e03    	add.w	lr, r6, r3
 80052d4: f8c1 e2e8    	str.w	lr, [r1, #0x2e8]
 80052d8: f501 71c0    	add.w	r1, r1, #0x180
 80052dc: f47f af7b    	bne.w	0x80051d6 <raylar_crcl_encoder_service::benchmark::dcblock::h5d1170af065dd1eb+0x136> @ imm = #-0x10a
 80052e0: f645 7174    	movw	r1, #0x5f74
 80052e4: f04f 0800    	mov.w	r8, #0x0
 80052e8: 4401         	add	r1, r0
 80052ea: f44f 6280    	mov.w	r2, #0x400
 80052ee: 2400         	movs	r4, #0x0
 80052f0: eb04 2428    	add.w	r4, r4, r8, asr #8
 80052f4: f8d1 50c0    	ldr.w	r5, [r1, #0xc0]
 80052f8: eba8 0404    	sub.w	r4, r8, r4
 80052fc: f8d1 60d8    	ldr.w	r6, [r1, #0xd8]
 8005300: 4425         	add	r5, r4
 8005302: f8c1 50c0    	str.w	r5, [r1, #0xc0]
 8005306: f8d1 30f0    	ldr.w	r3, [r1, #0xf0]
 800530a: 3a10         	subs	r2, #0x10
 800530c: eba4 2425    	sub.w	r4, r4, r5, asr #8
 8005310: f8d1 9108    	ldr.w	r9, [r1, #0x108]
 8005314: eb04 0506    	add.w	r5, r4, r6
 8005318: f8d1 6120    	ldr.w	r6, [r1, #0x120]
 800531c: f8c1 50d8    	str.w	r5, [r1, #0xd8]
 8005320: eba4 2425    	sub.w	r4, r4, r5, asr #8
 8005324: f8d1 5150    	ldr.w	r5, [r1, #0x150]
 8005328: 4423         	add	r3, r4
 800532a: f8c1 30f0    	str.w	r3, [r1, #0xf0]
 800532e: f8d1 b168    	ldr.w	r11, [r1, #0x168]
 8005332: eba4 2323    	sub.w	r3, r4, r3, asr #8
 8005336: f8d1 8180    	ldr.w	r8, [r1, #0x180]
 800533a: eb03 0409    	add.w	r4, r3, r9
 800533e: f8c1 4108    	str.w	r4, [r1, #0x108]
 8005342: f8d1 a198    	ldr.w	r10, [r1, #0x198]
 8005346: eba3 2324    	sub.w	r3, r3, r4, asr #8
 800534a: eb03 0406    	add.w	r4, r3, r6
 800534e: f8d1 6138    	ldr.w	r6, [r1, #0x138]
 8005352: f8c1 4120    	str.w	r4, [r1, #0x120]
 8005356: eba3 2324    	sub.w	r3, r3, r4, asr #8
 800535a: eb03 0406    	add.w	r4, r3, r6
 800535e: f8c1 4138    	str.w	r4, [r1, #0x138]
 8005362: eba3 2324    	sub.w	r3, r3, r4, asr #8
 8005366: eb03 0405    	add.w	r4, r3, r5
 800536a: f8c1 4150    	str.w	r4, [r1, #0x150]
 800536e: f8d1 51b0    	ldr.w	r5, [r1, #0x1b0]
 8005372: eba3 2324    	sub.w	r3, r3, r4, asr #8
 8005376: eb03 040b    	add.w	r4, r3, r11
 800537a: f8c1 4168    	str.w	r4, [r1, #0x168]
 800537e: eba3 2324    	sub.w	r3, r3, r4, asr #8
 8005382: eb03 0408    	add.w	r4, r3, r8
 8005386: f8c1 4180    	str.w	r4, [r1, #0x180]
 800538a: eba3 2324    	sub.w	r3, r3, r4, asr #8
 800538e: eb03 040a    	add.w	r4, r3, r10
 8005392: f8c1 4198    	str.w	r4, [r1, #0x198]
 8005396: eba3 2324    	sub.w	r3, r3, r4, asr #8
 800539a: eb03 0405    	add.w	r4, r3, r5
 800539e: f8d1 51c8    	ldr.w	r5, [r1, #0x1c8]
 80053a2: f8c1 41b0    	str.w	r4, [r1, #0x1b0]
 80053a6: eba3 2324    	sub.w	r3, r3, r4, asr #8
 80053aa: eb03 0405    	add.w	r4, r3, r5
 80053ae: f8d1 51e0    	ldr.w	r5, [r1, #0x1e0]
 80053b2: f8c1 41c8    	str.w	r4, [r1, #0x1c8]
 80053b6: eba3 2324    	sub.w	r3, r3, r4, asr #8
 80053ba: eb03 0405    	add.w	r4, r3, r5
 80053be: f8d1 51f8    	ldr.w	r5, [r1, #0x1f8]
 80053c2: f8c1 41e0    	str.w	r4, [r1, #0x1e0]
 80053c6: eba3 2324    	sub.w	r3, r3, r4, asr #8
 80053ca: eb03 0405    	add.w	r4, r3, r5
 80053ce: f8d1 5210    	ldr.w	r5, [r1, #0x210]
 80053d2: f8c1 41f8    	str.w	r4, [r1, #0x1f8]
 80053d6: eba3 2324    	sub.w	r3, r3, r4, asr #8
 80053da: f8d1 4228    	ldr.w	r4, [r1, #0x228]
 80053de: 441d         	add	r5, r3
 80053e0: f8c1 5210    	str.w	r5, [r1, #0x210]
 80053e4: eba3 2325    	sub.w	r3, r3, r5, asr #8
 80053e8: eb03 0804    	add.w	r8, r3, r4
 80053ec: f8c1 8228    	str.w	r8, [r1, #0x228]
 80053f0: f501 71c0    	add.w	r1, r1, #0x180
 80053f4: f47f af7c    	bne.w	0x80052f0 <raylar_crcl_encoder_service::benchmark::dcblock::h5d1170af065dd1eb+0x250> @ imm = #-0x108
 80053f8: f645 61b8    	movw	r1, #0x5eb8
 80053fc: 1842         	adds	r2, r0, r1
 80053fe: f04f 0900    	mov.w	r9, #0x0
 8005402: f44f 6c80    	mov.w	r12, #0x400
 8005406: 2500         	movs	r5, #0x0
 8005408: eb05 2529    	add.w	r5, r5, r9, asr #8
 800540c: f8d2 3180    	ldr.w	r3, [r2, #0x180]
 8005410: eba9 0505    	sub.w	r5, r9, r5
 8005414: f8d2 6198    	ldr.w	r6, [r2, #0x198]
 8005418: 442b         	add	r3, r5
 800541a: f8c2 3180    	str.w	r3, [r2, #0x180]
 800541e: f8d2 11b0    	ldr.w	r1, [r2, #0x1b0]
 8005422: f1bc 0c10    	subs.w	r12, r12, #0x10
 8005426: eba5 2323    	sub.w	r3, r5, r3, asr #8
 800542a: f8d2 a1c8    	ldr.w	r10, [r2, #0x1c8]
 800542e: eb03 0506    	add.w	r5, r3, r6
 8005432: f8d2 41e0    	ldr.w	r4, [r2, #0x1e0]
 8005436: f8c2 5198    	str.w	r5, [r2, #0x198]
 800543a: eba3 2325    	sub.w	r3, r3, r5, asr #8
 800543e: f8d2 5210    	ldr.w	r5, [r2, #0x210]
 8005442: 4419         	add	r1, r3
 8005444: f8c2 11b0    	str.w	r1, [r2, #0x1b0]
 8005448: f8d2 6228    	ldr.w	r6, [r2, #0x228]
 800544c: eba3 2121    	sub.w	r1, r3, r1, asr #8
 8005450: f8d2 9240    	ldr.w	r9, [r2, #0x240]
 8005454: eb01 030a    	add.w	r3, r1, r10
 8005458: f8c2 31c8    	str.w	r3, [r2, #0x1c8]
 800545c: f8d2 b258    	ldr.w	r11, [r2, #0x258]
 8005460: eba1 2123    	sub.w	r1, r1, r3, asr #8
 8005464: eb01 0304    	add.w	r3, r1, r4
 8005468: f8d2 41f8    	ldr.w	r4, [r2, #0x1f8]
 800546c: f8c2 31e0    	str.w	r3, [r2, #0x1e0]
 8005470: eba1 2123    	sub.w	r1, r1, r3, asr #8
 8005474: eb01 0304    	add.w	r3, r1, r4
 8005478: f8c2 31f8    	str.w	r3, [r2, #0x1f8]
 800547c: f8d2 4270    	ldr.w	r4, [r2, #0x270]
 8005480: eba1 2123    	sub.w	r1, r1, r3, asr #8
 8005484: eb01 0305    	add.w	r3, r1, r5
 8005488: f8c2 3210    	str.w	r3, [r2, #0x210]
 800548c: f8d2 52e8    	ldr.w	r5, [r2, #0x2e8]
 8005490: eba1 2123    	sub.w	r1, r1, r3, asr #8
 8005494: eb01 0306    	add.w	r3, r1, r6
 8005498: f8c2 3228    	str.w	r3, [r2, #0x228]
 800549c: eba1 2123    	sub.w	r1, r1, r3, asr #8
 80054a0: eb01 0309    	add.w	r3, r1, r9
 80054a4: f8c2 3240    	str.w	r3, [r2, #0x240]
 80054a8: eba1 2123    	sub.w	r1, r1, r3, asr #8
 80054ac: eb01 030b    	add.w	r3, r1, r11
 80054b0: f8c2 3258    	str.w	r3, [r2, #0x258]
 80054b4: eba1 2123    	sub.w	r1, r1, r3, asr #8
 80054b8: eb01 0304    	add.w	r3, r1, r4
 80054bc: f8d2 4288    	ldr.w	r4, [r2, #0x288]
 80054c0: f8c2 3270    	str.w	r3, [r2, #0x270]
 80054c4: eba1 2123    	sub.w	r1, r1, r3, asr #8
 80054c8: eb01 0304    	add.w	r3, r1, r4
 80054cc: f8d2 42a0    	ldr.w	r4, [r2, #0x2a0]
 80054d0: f8c2 3288    	str.w	r3, [r2, #0x288]
 80054d4: eba1 2123    	sub.w	r1, r1, r3, asr #8
 80054d8: eb01 0304    	add.w	r3, r1, r4
 80054dc: f8d2 42b8    	ldr.w	r4, [r2, #0x2b8]
 80054e0: f8c2 32a0    	str.w	r3, [r2, #0x2a0]
 80054e4: eba1 2123    	sub.w	r1, r1, r3, asr #8
 80054e8: eb01 0304    	add.w	r3, r1, r4
 80054ec: f8d2 42d0    	ldr.w	r4, [r2, #0x2d0]
 80054f0: f8c2 32b8    	str.w	r3, [r2, #0x2b8]
 80054f4: eba1 2123    	sub.w	r1, r1, r3, asr #8
 80054f8: eb01 0304    	add.w	r3, r1, r4
 80054fc: f8c2 32d0    	str.w	r3, [r2, #0x2d0]
 8005500: eba1 2123    	sub.w	r1, r1, r3, asr #8
 8005504: eb01 0905    	add.w	r9, r1, r5
 8005508: f8c2 92e8    	str.w	r9, [r2, #0x2e8]
 800550c: f502 72c0    	add.w	r2, r2, #0x180
 8005510: f47f af7a    	bne.w	0x8005408 <raylar_crcl_encoder_service::benchmark::dcblock::h5d1170af065dd1eb+0x368> @ imm = #-0x10c
 8005514: f645 717c    	movw	r1, #0x5f7c
 8005518: 1842         	adds	r2, r0, r1
 800551a: 2500         	movs	r5, #0x0
 800551c: f44f 6480    	mov.w	r4, #0x400
 8005520: 2600         	movs	r6, #0x0
 8005522: eb06 2625    	add.w	r6, r6, r5, asr #8
 8005526: f8d2 10c0    	ldr.w	r1, [r2, #0xc0]
 800552a: 1bae         	subs	r6, r5, r6
 800552c: f8d2 30d8    	ldr.w	r3, [r2, #0xd8]
 8005530: 4431         	add	r1, r6
 8005532: f8c2 10c0    	str.w	r1, [r2, #0xc0]
 8005536: f8d2 00f0    	ldr.w	r0, [r2, #0xf0]
 800553a: 3c10         	subs	r4, #0x10
 800553c: eba6 2121    	sub.w	r1, r6, r1, asr #8
 8005540: f8d2 c108    	ldr.w	r12, [r2, #0x108]
 8005544: 440b         	add	r3, r1
 8005546: f8d2 5120    	ldr.w	r5, [r2, #0x120]
 800554a: f8c2 30d8    	str.w	r3, [r2, #0xd8]
 800554e: eba1 2123    	sub.w	r1, r1, r3, asr #8
 8005552: f8d2 3150    	ldr.w	r3, [r2, #0x150]
 8005556: 4408         	add	r0, r1
 8005558: f8c2 00f0    	str.w	r0, [r2, #0xf0]
 800555c: f8d2 6168    	ldr.w	r6, [r2, #0x168]
 8005560: eba1 2020    	sub.w	r0, r1, r0, asr #8
 8005564: f8d2 b180    	ldr.w	r11, [r2, #0x180]
 8005568: eb00 010c    	add.w	r1, r0, r12
 800556c: f8c2 1108    	str.w	r1, [r2, #0x108]
 8005570: f8d2 a198    	ldr.w	r10, [r2, #0x198]
 8005574: eba0 2021    	sub.w	r0, r0, r1, asr #8
 8005578: eb00 0105    	add.w	r1, r0, r5
 800557c: f8d2 5138    	ldr.w	r5, [r2, #0x138]
 8005580: f8c2 1120    	str.w	r1, [r2, #0x120]
 8005584: eba0 2021    	sub.w	r0, r0, r1, asr #8
 8005588: eb00 0105    	add.w	r1, r0, r5
 800558c: f8c2 1138    	str.w	r1, [r2, #0x138]
 8005590: eba0 2021    	sub.w	r0, r0, r1, asr #8
 8005594: eb00 0103    	add.w	r1, r0, r3
 8005598: f8c2 1150    	str.w	r1, [r2, #0x150]
 800559c: f8d2 31b0    	ldr.w	r3, [r2, #0x1b0]
 80055a0: eba0 2021    	sub.w	r0, r0, r1, asr #8
 80055a4: eb00 0106    	add.w	r1, r0, r6
 80055a8: f8c2 1168    	str.w	r1, [r2, #0x168]
 80055ac: f8d2 6228    	ldr.w	r6, [r2, #0x228]
 80055b0: eba0 2021    	sub.w	r0, r0, r1, asr #8
 80055b4: eb00 010b    	add.w	r1, r0, r11
 80055b8: f8c2 1180    	str.w	r1, [r2, #0x180]
 80055bc: eba0 2021    	sub.w	r0, r0, r1, asr #8
 80055c0: eb00 010a    	add.w	r1, r0, r10
 80055c4: f8c2 1198    	str.w	r1, [r2, #0x198]
 80055c8: eba0 2021    	sub.w	r0, r0, r1, asr #8
 80055cc: eb00 0103    	add.w	r1, r0, r3
 80055d0: f8d2 31c8    	ldr.w	r3, [r2, #0x1c8]
 80055d4: f8c2 11b0    	str.w	r1, [r2, #0x1b0]
 80055d8: eba0 2021    	sub.w	r0, r0, r1, asr #8
 80055dc: eb00 0103    	add.w	r1, r0, r3
 80055e0: f8d2 31e0    	ldr.w	r3, [r2, #0x1e0]
 80055e4: f8c2 11c8    	str.w	r1, [r2, #0x1c8]
 80055e8: eba0 2021    	sub.w	r0, r0, r1, asr #8
 80055ec: eb00 0103    	add.w	r1, r0, r3
 80055f0: f8d2 31f8    	ldr.w	r3, [r2, #0x1f8]
 80055f4: f8c2 11e0    	str.w	r1, [r2, #0x1e0]
 80055f8: eba0 2021    	sub.w	r0, r0, r1, asr #8
 80055fc: eb00 0103    	add.w	r1, r0, r3
 8005600: f8d2 3210    	ldr.w	r3, [r2, #0x210]
 8005604: f8c2 11f8    	str.w	r1, [r2, #0x1f8]
 8005608: eba0 2021    	sub.w	r0, r0, r1, asr #8
 800560c: eb00 0103    	add.w	r1, r0, r3
 8005610: f8c2 1210    	str.w	r1, [r2, #0x210]
 8005614: eba0 2021    	sub.w	r0, r0, r1, asr #8
 8005618: eb00 0506    	add.w	r5, r0, r6
 800561c: f8c2 5228    	str.w	r5, [r2, #0x228]
 8005620: f502 72c0    	add.w	r2, r2, #0x180
 8005624: f47f af7d    	bne.w	0x8005522 <raylar_crcl_encoder_service::benchmark::dcblock::h5d1170af065dd1eb+0x482> @ imm = #-0x106
 8005628: 9901         	ldr	r1, [sp, #0x4]
 800562a: f246 0028    	movw	r0, #0x6028
 800562e: 4408         	add	r0, r1
 8005630: 9002         	str	r0, [sp, #0x8]
 8005632: a802         	add	r0, sp, #0x8
 8005634: 9800         	ldr	r0, [sp]
 8005636: ea80 000e    	eor.w	r0, r0, lr
 800563a: ea80 0008    	eor.w	r0, r0, r8
 800563e: ea80 0009    	eor.w	r0, r0, r9
 8005642: 4068         	eors	r0, r5
 8005644: b003         	add	sp, #0xc
 8005646: e8bd 0f00    	pop.w	{r8, r9, r10, r11}
 800564a: bdf0         	pop	{r4, r5, r6, r7, pc}

0800564c <raylar_crcl_encoder_service::benchmark::measure::he296905d366d7d56>:
 800564c: b5f0         	push	{r4, r5, r6, r7, lr}
 800564e: af03         	add	r7, sp, #0xc
 8005650: e92d 0f00    	push.w	{r8, r9, r10, r11}
 8005654: f5ad 7d05    	sub.w	sp, sp, #0x214
 8005658: 9001         	str	r0, [sp, #0x4]
 800565a: a802         	add	r0, sp, #0x8
 800565c: 460d         	mov	r5, r1
 800565e: f44f 7101    	mov.w	r1, #0x204
 8005662: 9300         	str	r3, [sp]
 8005664: 4616         	mov	r6, r2
 8005666: f007 fe84    	bl	0x800d372 <__aeabi_memclr4> @ imm = #0x7d08
 800566a: f50d 7a03    	add.w	r10, sp, #0x20c
 800566e: f04f 0b00    	mov.w	r11, #0x0
 8005672: f04f 0800    	mov.w	r8, #0x0
 8005676: e004         	b	0x8005682 <raylar_crcl_encoder_service::benchmark::measure::he296905d366d7d56+0x36> @ imm = #0x8
 8005678: f108 0801    	add.w	r8, r8, #0x1
 800567c: f1b8 0f89    	cmp.w	r8, #0x89
 8005680: d018         	beq	0x80056b4 <raylar_crcl_encoder_service::benchmark::measure::he296905d366d7d56+0x68> @ imm = #0x30
 8005682: 6939         	ldr	r1, [r7, #0x10]
 8005684: 4628         	mov	r0, r5
 8005686: 4788         	blx	r1
 8005688: 47b0         	blx	r6
 800568a: 9583         	str	r5, [sp, #0x20c]
 800568c: 4604         	mov	r4, r0
 800568e: 6979         	ldr	r1, [r7, #0x14]
 8005690: 9883         	ldr	r0, [sp, #0x20c]
 8005692: 4788         	blx	r1
 8005694: 9083         	str	r0, [sp, #0x20c]
 8005696: f8dd 920c    	ldr.w	r9, [sp, #0x20c]
 800569a: 47b0         	blx	r6
 800569c: f1b8 0f07    	cmp.w	r8, #0x7
 80056a0: d9ea         	bls	0x8005678 <raylar_crcl_encoder_service::benchmark::measure::he296905d366d7d56+0x2c> @ imm = #-0x2c
 80056a2: a902         	add	r1, sp, #0x8
 80056a4: 1b00         	subs	r0, r0, r4
 80056a6: eb01 0188    	add.w	r1, r1, r8, lsl #2
 80056aa: ea8b 0b09    	eor.w	r11, r11, r9
 80056ae: f841 0c20    	str	r0, [r1, #-32]
 80056b2: e7e1         	b	0x8005678 <raylar_crcl_encoder_service::benchmark::measure::he296905d366d7d56+0x2c> @ imm = #-0x3e
 80056b4: a802         	add	r0, sp, #0x8
 80056b6: f1a7 021d    	sub.w	r2, r7, #0x1d
 80056ba: 2181         	movs	r1, #0x81
 80056bc: f002 fe13    	bl	0x80082e6 <core::slice::sort::unstable::ipnsort::hd8a371dd4e708cab> @ imm = #0x2c26
 80056c0: e9dd 5600    	ldrd	r5, r6, [sp]
 80056c4: 9802         	ldr	r0, [sp, #0x8]
 80056c6: 6035         	str	r5, [r6]
 80056c8: 68bd         	ldr	r5, [r7, #0x8]
 80056ca: 6075         	str	r5, [r6, #0x4]
 80056cc: 68fd         	ldr	r5, [r7, #0xc]
 80056ce: 9942         	ldr	r1, [sp, #0x108]
 80056d0: e9c6 5002    	strd	r5, r0, [r6, #8]
 80056d4: f106 0010    	add.w	r0, r6, #0x10
 80056d8: 9a75         	ldr	r2, [sp, #0x1d4]
 80056da: 9b82         	ldr	r3, [sp, #0x208]
 80056dc: e880 080e    	stm.w	r0, {r1, r2, r3, r11}
 80056e0: f50d 7d05    	add.w	sp, sp, #0x214
 80056e4: e8bd 0f00    	pop.w	{r8, r9, r10, r11}
 80056e8: bdf0         	pop	{r4, r5, r6, r7, pc}

080056ea <raylar_crcl_encoder_service::benchmark::prepare::hb9734b8d5dc7f78e>:
 80056ea: b5f0         	push	{r4, r5, r6, r7, lr}
 80056ec: af03         	add	r7, sp, #0xc
 80056ee: e92d 0f00    	push.w	{r8, r9, r10, r11}
 80056f2: b0f9         	sub	sp, #0x1e4
 80056f4: 4601         	mov	r1, r0
 80056f6: f500 30ee    	add.w	r0, r0, #0x1dc00
 80056fa: f500 701f    	add.w	r0, r0, #0x27c
 80056fe: 9004         	str	r0, [sp, #0x10]
 8005700: f501 30f8    	add.w	r0, r1, #0x1f000
 8005704: f245 6578    	movw	r5, #0x5678
 8005708: f500 604b    	add.w	r0, r0, #0xcb0
 800570c: f64e 0860    	movw	r8, #0xe860
 8005710: 9005         	str	r0, [sp, #0x14]
 8005712: f101 0028    	add.w	r0, r1, #0x28
 8005716: 9003         	str	r0, [sp, #0xc]
 8005718: 2000         	movs	r0, #0x0
 800571a: f2c1 2534    	movt	r5, #0x1234
 800571e: f06f 0917    	mvn	r9, #0x17
 8005722: f6c0 0800    	movt	r8, #0x800
 8005726: f644 6a20    	movw	r10, #0x4e20
 800572a: f645 5bc0    	movw	r11, #0x5dc0
 800572e: f645 14d8    	movw	r4, #0x59d8
 8005732: 9124         	str	r1, [sp, #0x90]
 8005734: f000 031f    	and	r3, r0, #0x1f
 8005738: f100 060a    	add.w	r6, r0, #0xa
 800573c: f006 061f    	and	r6, r6, #0x1f
 8005740: f100 0c02    	add.w	r12, r0, #0x2
 8005744: f858 3023    	ldr.w	r3, [r8, r3, lsl #2]
 8005748: ea85 3545    	eor.w	r5, r5, r5, lsl #13
 800574c: f858 6026    	ldr.w	r6, [r8, r6, lsl #2]
 8005750: f246 11a8    	movw	r1, #0x61a8
 8005754: ea85 4555    	eor.w	r5, r5, r5, lsr #17
 8005758: fb03 fe0a    	mul	lr, r3, r10
 800575c: f00c 031f    	and	r3, r12, #0x1f
 8005760: ea85 1545    	eor.w	r5, r5, r5, lsl #5
 8005764: f858 2023    	ldr.w	r2, [r8, r3, lsl #2]
 8005768: fb06 fc01    	mul	r12, r6, r1
 800576c: f245 2108    	movw	r1, #0x5208
 8005770: b2eb         	uxtb	r3, r5
 8005772: 434a         	muls	r2, r1, r2
 8005774: ea4f 76ee    	asr.w	r6, lr, #0x1f
 8005778: eb0e 4656    	add.w	r6, lr, r6, lsr #17
 800577c: 9924         	ldr	r1, [sp, #0x90]
 800577e: eb03 33e6    	add.w	r3, r3, r6, asr #15
 8005782: eb01 0609    	add.w	r6, r1, r9
 8005786: 3b80         	subs	r3, #0x80
 8005788: f245 51f0    	movw	r1, #0x55f0
 800578c: f109 0918    	add.w	r9, r9, #0x18
 8005790: 6433         	str	r3, [r6, #0x40]
 8005792: 17d3         	asrs	r3, r2, #0x1f
 8005794: eb02 4253    	add.w	r2, r2, r3, lsr #17
 8005798: ea85 3345    	eor.w	r3, r5, r5, lsl #13
 800579c: ea83 4353    	eor.w	r3, r3, r3, lsr #17
 80057a0: ea83 1343    	eor.w	r3, r3, r3, lsl #5
 80057a4: b2dd         	uxtb	r5, r3
 80057a6: eb05 32e2    	add.w	r2, r5, r2, asr #15
 80057aa: 1d05         	adds	r5, r0, #0x4
 80057ac: ea83 3343    	eor.w	r3, r3, r3, lsl #13
 80057b0: f005 051f    	and	r5, r5, #0x1f
 80057b4: 3a80         	subs	r2, #0x80
 80057b6: ea83 4353    	eor.w	r3, r3, r3, lsr #17
 80057ba: 6472         	str	r2, [r6, #0x44]
 80057bc: f858 5025    	ldr.w	r5, [r8, r5, lsl #2]
 80057c0: ea83 1343    	eor.w	r3, r3, r3, lsl #5
 80057c4: 434d         	muls	r5, r1, r5
 80057c6: 1d81         	adds	r1, r0, #0x6
 80057c8: f001 011f    	and	r1, r1, #0x1f
 80057cc: f858 1021    	ldr.w	r1, [r8, r1, lsl #2]
 80057d0: 17ea         	asrs	r2, r5, #0x1f
 80057d2: 4361         	muls	r1, r4, r1
 80057d4: eb05 4252    	add.w	r2, r5, r2, lsr #17
 80057d8: b2dd         	uxtb	r5, r3
 80057da: eb05 32e2    	add.w	r2, r5, r2, asr #15
 80057de: f100 0508    	add.w	r5, r0, #0x8
 80057e2: f005 051f    	and	r5, r5, #0x1f
 80057e6: 3a80         	subs	r2, #0x80
 80057e8: 64b2         	str	r2, [r6, #0x48]
 80057ea: 3001         	adds	r0, #0x1
 80057ec: f858 5025    	ldr.w	r5, [r8, r5, lsl #2]
 80057f0: f5b0 6f80    	cmp.w	r0, #0x400
 80057f4: ea4f 72e1    	asr.w	r2, r1, #0x1f
 80057f8: eb01 4152    	add.w	r1, r1, r2, lsr #17
 80057fc: ea83 3243    	eor.w	r2, r3, r3, lsl #13
 8005800: ea82 4252    	eor.w	r2, r2, r2, lsr #17
 8005804: fb05 f50b    	mul	r5, r5, r11
 8005808: ea82 1242    	eor.w	r2, r2, r2, lsl #5
 800580c: b2d3         	uxtb	r3, r2
 800580e: ea82 3242    	eor.w	r2, r2, r2, lsl #13
 8005812: eb03 31e1    	add.w	r1, r3, r1, asr #15
 8005816: ea82 4252    	eor.w	r2, r2, r2, lsr #17
 800581a: f1a1 0180    	sub.w	r1, r1, #0x80
 800581e: 64f1         	str	r1, [r6, #0x4c]
 8005820: ea4f 71e5    	asr.w	r1, r5, #0x1f
 8005824: ea82 1242    	eor.w	r2, r2, r2, lsl #5
 8005828: eb05 4151    	add.w	r1, r5, r1, lsr #17
 800582c: b2d3         	uxtb	r3, r2
 800582e: ea82 3242    	eor.w	r2, r2, r2, lsl #13
 8005832: eb03 31e1    	add.w	r1, r3, r1, asr #15
 8005836: f1a1 0180    	sub.w	r1, r1, #0x80
 800583a: ea82 4252    	eor.w	r2, r2, r2, lsr #17
 800583e: 6531         	str	r1, [r6, #0x50]
 8005840: ea4f 71ec    	asr.w	r1, r12, #0x1f
 8005844: ea82 1542    	eor.w	r5, r2, r2, lsl #5
 8005848: eb0c 4151    	add.w	r1, r12, r1, lsr #17
 800584c: b2ea         	uxtb	r2, r5
 800584e: eb02 31e1    	add.w	r1, r2, r1, asr #15
 8005852: f1a1 0180    	sub.w	r1, r1, #0x80
 8005856: 6571         	str	r1, [r6, #0x54]
 8005858: f47f af6c    	bne.w	0x8005734 <raylar_crcl_encoder_service::benchmark::prepare::hb9734b8d5dc7f78e+0x4a> @ imm = #-0x128
 800585c: 9924         	ldr	r1, [sp, #0x90]
 800585e: f246 0028    	movw	r0, #0x6028
 8005862: f44f 42c0    	mov.w	r2, #0x6000
 8005866: eb01 0900    	add.w	r9, r1, r0
 800586a: 3128         	adds	r1, #0x28
 800586c: 4648         	mov	r0, r9
 800586e: f007 fd32    	bl	0x800d2d6 <__aeabi_memcpy4> @ imm = #0x7a64
 8005872: ad25         	add	r5, sp, #0x94
 8005874: 2128         	movs	r1, #0x28
 8005876: f105 0848    	add.w	r8, r5, #0x48
 800587a: 4640         	mov	r0, r8
 800587c: f007 fd79    	bl	0x800d372 <__aeabi_memclr4> @ imm = #0x7af2
 8005880: 2101         	movs	r1, #0x1
 8005882: 2000         	movs	r0, #0x0
 8005884: 9141         	str	r1, [sp, #0x104]
 8005886: f44f 51c0    	mov.w	r1, #0x1800
 800588a: e9cd 9135    	strd	r9, r1, [sp, #212]
 800588e: f44f 6180    	mov.w	r1, #0x400
 8005892: 9146         	str	r1, [sp, #0x118]
 8005894: f44f 0100    	mov.w	r1, #0x800000
 8005898: 9147         	str	r1, [sp, #0x11c]
 800589a: f240 610a    	movw	r1, #0x60a
 800589e: f2c0 1101    	movt	r1, #0x101
 80058a2: e9cd 0044    	strd	r0, r0, [sp, #272]
 80058a6: e9cd 0042    	strd	r0, r0, [sp, #264]
 80058aa: e9cd 0025    	strd	r0, r0, [sp, #148]
 80058ae: e9cd 0027    	strd	r0, r0, [sp, #156]
 80058b2: e9cd 0029    	strd	r0, r0, [sp, #164]
 80058b6: 902b         	str	r0, [sp, #0xac]
 80058b8: f8ad 00b0    	strh.w	r0, [sp, #0xb0]
 80058bc: e9cd 002d    	strd	r0, r0, [sp, #180]
 80058c0: e9cd 002f    	strd	r0, r0, [sp, #188]
 80058c4: e9cd 0031    	strd	r0, r0, [sp, #196]
 80058c8: 9033         	str	r0, [sp, #0xcc]
 80058ca: f8ad 00d0    	strh.w	r0, [sp, #0xd0]
 80058ce: 9148         	str	r1, [sp, #0x120]
 80058d0: f8ad 0124    	strh.w	r0, [sp, #0x124]
 80058d4: 4628         	mov	r0, r5
 80058d6: f04f 31ff    	mov.w	r1, #0xffffffff
 80058da: f7fc f988    	bl	0x8001bee <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3> @ imm = #-0x3cf0
 80058de: 07c0         	lsls	r0, r0, #0x1f
 80058e0: f041 804d    	bne.w	0x800697e <raylar_crcl_encoder_service::benchmark::prepare::hb9734b8d5dc7f78e+0x1294> @ imm = #0x109a
 80058e4: 07c8         	lsls	r0, r1, #0x1f
 80058e6: d0f5         	beq	0x80058d4 <raylar_crcl_encoder_service::benchmark::prepare::hb9734b8d5dc7f78e+0x1ea> @ imm = #-0x16
 80058e8: 9c24         	ldr	r4, [sp, #0x90]
 80058ea: e8b8 006e    	ldm.w	r8!, {r1, r2, r3, r5, r6}
 80058ee: 4620         	mov	r0, r4
 80058f0: c06e         	stm	r0!, {r1, r2, r3, r5, r6}
 80058f2: e898 006e    	ldm.w	r8, {r1, r2, r3, r5, r6}
 80058f6: c06e         	stm	r0!, {r1, r2, r3, r5, r6}
 80058f8: f640 2100    	movw	r1, #0xa00
 80058fc: f2c0 1106    	movt	r1, #0x106
 8005900: f8cd 1166    	str.w	r1, [sp, #0x166]
 8005904: f44f 0100    	mov.w	r1, #0x800000
 8005908: 2000         	movs	r0, #0x0
 800590a: f8cd 1163    	str.w	r1, [sp, #0x163]
 800590e: f44f 6180    	mov.w	r1, #0x400
 8005912: ad4c         	add	r5, sp, #0x130
 8005914: f8cd 115f    	str.w	r1, [sp, #0x15f]
 8005918: a954         	add	r1, sp, #0x150
 800591a: 9056         	str	r0, [sp, #0x158]
 800591c: 221a         	movs	r2, #0x1a
 800591e: f88d 0130    	strb.w	r0, [sp, #0x130]
 8005922: e9cd 0054    	strd	r0, r0, [sp, #336]
 8005926: f8cd 015b    	str.w	r0, [sp, #0x15b]
 800592a: 1c68         	adds	r0, r5, #0x1
 800592c: f007 faf8    	bl	0x800cf20 <__aeabi_memcpy> @ imm = #0x75f0
 8005930: ae54         	add	r6, sp, #0x150
 8005932: 2001         	movs	r0, #0x1
 8005934: a925         	add	r1, sp, #0x94
 8005936: f88d 014b    	strb.w	r0, [sp, #0x14b]
 800593a: 4630         	mov	r0, r6
 800593c: 2294         	movs	r2, #0x94
 800593e: f007 fcca    	bl	0x800d2d6 <__aeabi_memcpy4> @ imm = #0x7994
 8005942: f641 6058    	movw	r0, #0x1e58
 8005946: 4629         	mov	r1, r5
 8005948: 9000         	str	r0, [sp]
 800594a: f504 30ec    	add.w	r0, r4, #0x1d800
 800594e: f500 63cb    	add.w	r3, r0, #0x658
 8005952: a84a         	add	r0, sp, #0x128
 8005954: 4632         	mov	r2, r6
 8005956: f7fb fe93    	bl	0x8001680 <raylar_crcl_encoder_service::format::Encoder::data::ha65fe528befbf4e3> @ imm = #-0x42da
 800595a: f89d 0128    	ldrb.w	r0, [sp, #0x128]
 800595e: 2801         	cmp	r0, #0x1
 8005960: f001 8034    	beq.w	0x80069cc <raylar_crcl_encoder_service::benchmark::prepare::hb9734b8d5dc7f78e+0x12e2> @ imm = #0x1068
 8005964: 9905         	ldr	r1, [sp, #0x14]
 8005966: f44f 42c0    	mov.w	r2, #0x6000
 800596a: 984b         	ldr	r0, [sp, #0x12c]
 800596c: 6288         	str	r0, [r1, #0x28]
 800596e: f24c 0028    	movw	r0, #0xc028
 8005972: 9924         	ldr	r1, [sp, #0x90]
 8005974: 4408         	add	r0, r1
 8005976: 4649         	mov	r1, r9
 8005978: 9002         	str	r0, [sp, #0x8]
 800597a: f007 fcac    	bl	0x800d2d6 <__aeabi_memcpy4> @ imm = #0x7958
 800597e: f641 71fc    	movw	r1, #0x1ffc
 8005982: f247 0910    	movw	r9, #0x7010
 8005986: f2c0 0101    	movt	r1, #0x1
 800598a: f101 0020    	add.w	r0, r1, #0x20
 800598e: 9018         	str	r0, [sp, #0x60]
 8005990: f101 001c    	add.w	r0, r1, #0x1c
 8005994: 9020         	str	r0, [sp, #0x80]
 8005996: 2000         	movs	r0, #0x0
 8005998: 9019         	str	r0, [sp, #0x64]
 800599a: f64b 70fc    	movw	r0, #0xbffc
 800599e: 9017         	str	r0, [sp, #0x5c]
 80059a0: f44f 3090    	mov.w	r0, #0x12000
 80059a4: 9023         	str	r0, [sp, #0x8c]
 80059a6: f64b 70e4    	movw	r0, #0xbfe4
 80059aa: 9016         	str	r0, [sp, #0x58]
 80059ac: f64b 706c    	movw	r0, #0xbf6c
 80059b0: 9022         	str	r0, [sp, #0x88]
 80059b2: f64b 70cc    	movw	r0, #0xbfcc
 80059b6: 9021         	str	r0, [sp, #0x84]
 80059b8: f64b 70b4    	movw	r0, #0xbfb4
 80059bc: 901f         	str	r0, [sp, #0x7c]
 80059be: f64b 7084    	movw	r0, #0xbf84
 80059c2: 901e         	str	r0, [sp, #0x78]
 80059c4: f64b 709c    	movw	r0, #0xbf9c
 80059c8: 901d         	str	r0, [sp, #0x74]
 80059ca: f101 0014    	add.w	r0, r1, #0x14
 80059ce: 901c         	str	r0, [sp, #0x70]
 80059d0: f101 0010    	add.w	r0, r1, #0x10
 80059d4: 901b         	str	r0, [sp, #0x6c]
 80059d6: f101 000c    	add.w	r0, r1, #0xc
 80059da: 901a         	str	r0, [sp, #0x68]
 80059dc: f24f 0004    	movw	r0, #0xf004
 80059e0: f101 0c28    	add.w	r12, r1, #0x28
 80059e4: f101 0624    	add.w	r6, r1, #0x24
 80059e8: f101 0318    	add.w	r3, r1, #0x18
 80059ec: f2c0 0901    	movt	r9, #0x1
 80059f0: f64b 7224    	movw	r2, #0xbf24
 80059f4: f64b 7a54    	movw	r10, #0xbf54
 80059f8: f6cf 70ff    	movt	r0, #0xffff
 80059fc: 9c24         	ldr	r4, [sp, #0x90]
 80059fe: 4698         	mov	r8, r3
 8005a00: e9cd 2912    	strd	r2, r9, [sp, #72]
 8005a04: eb04 0e02    	add.w	lr, r4, r2
 8005a08: e9cd 3114    	strd	r3, r1, [sp, #80]
 8005a0c: 1863         	adds	r3, r4, r1
 8005a0e: 44a1         	add	r9, r4
 8005a10: f8de 2108    	ldr.w	r2, [lr, #0x108]
 8005a14: f8de 5120    	ldr.w	r5, [lr, #0x120]
 8005a18: e9cd 0a10    	strd	r0, r10, [sp, #64]
 8005a1c: 1aaa         	subs	r2, r5, r2
 8005a1e: 62da         	str	r2, [r3, #0x2c]
 8005a20: f8dd b05c    	ldr.w	r11, [sp, #0x5c]
 8005a24: 0053         	lsls	r3, r2, #0x1
 8005a26: ea83 71e2    	eor.w	r1, r3, r2, asr #31
 8005a2a: eb04 030a    	add.w	r3, r4, r10
 8005a2e: f8c9 1004    	str.w	r1, [r9, #0x4]
 8005a32: 910f         	str	r1, [sp, #0x3c]
 8005a34: f8de 2120    	ldr.w	r2, [lr, #0x120]
 8005a38: f8d3 5108    	ldr.w	r5, [r3, #0x108]
 8005a3c: 9923         	ldr	r1, [sp, #0x8c]
 8005a3e: 1aaa         	subs	r2, r5, r2
 8005a40: f8dd a058    	ldr.w	r10, [sp, #0x58]
 8005a44: 1865         	adds	r5, r4, r1
 8005a46: 62ea         	str	r2, [r5, #0x2c]
 8005a48: 0055         	lsls	r5, r2, #0x1
 8005a4a: ea85 71e2    	eor.w	r1, r5, r2, asr #31
 8005a4e: f8c9 1008    	str.w	r1, [r9, #0x8]
 8005a52: 910e         	str	r1, [sp, #0x38]
 8005a54: 9922         	ldr	r1, [sp, #0x88]
 8005a56: f8d3 2108    	ldr.w	r2, [r3, #0x108]
 8005a5a: 1863         	adds	r3, r4, r1
 8005a5c: f8d3 5108    	ldr.w	r5, [r3, #0x108]
 8005a60: 1aaa         	subs	r2, r5, r2
 8005a62: 1825         	adds	r5, r4, r0
 8005a64: f243 002c    	movw	r0, #0x302c
 8005a68: f2c0 0001    	movt	r0, #0x1
 8005a6c: 502a         	str	r2, [r5, r0]
 8005a6e: 0055         	lsls	r5, r2, #0x1
 8005a70: ea85 70e2    	eor.w	r0, r5, r2, asr #31
 8005a74: f8c9 000c    	str.w	r0, [r9, #0xc]
 8005a78: 4665         	mov	r5, r12
 8005a7a: 900d         	str	r0, [sp, #0x34]
 8005a7c: 981e         	ldr	r0, [sp, #0x78]
 8005a7e: f8d3 2108    	ldr.w	r2, [r3, #0x108]
 8005a82: 1823         	adds	r3, r4, r0
 8005a84: 981a         	ldr	r0, [sp, #0x68]
 8005a86: f8d3 1108    	ldr.w	r1, [r3, #0x108]
 8005a8a: 1a89         	subs	r1, r1, r2
 8005a8c: 1822         	adds	r2, r4, r0
 8005a8e: 62d1         	str	r1, [r2, #0x2c]
 8005a90: 004a         	lsls	r2, r1, #0x1
 8005a92: ea82 70e1    	eor.w	r0, r2, r1, asr #31
 8005a96: f8c9 0010    	str.w	r0, [r9, #0x10]
 8005a9a: 900c         	str	r0, [sp, #0x30]
 8005a9c: 981d         	ldr	r0, [sp, #0x74]
 8005a9e: f8d3 1108    	ldr.w	r1, [r3, #0x108]
 8005aa2: 1822         	adds	r2, r4, r0
 8005aa4: 981b         	ldr	r0, [sp, #0x6c]
 8005aa6: f8d2 3108    	ldr.w	r3, [r2, #0x108]
 8005aaa: 1a59         	subs	r1, r3, r1
 8005aac: 1823         	adds	r3, r4, r0
 8005aae: 62d9         	str	r1, [r3, #0x2c]
 8005ab0: 004b         	lsls	r3, r1, #0x1
 8005ab2: ea83 70e1    	eor.w	r0, r3, r1, asr #31
 8005ab6: f8c9 0014    	str.w	r0, [r9, #0x14]
 8005aba: 900b         	str	r0, [sp, #0x2c]
 8005abc: 981f         	ldr	r0, [sp, #0x7c]
 8005abe: f8d2 1108    	ldr.w	r1, [r2, #0x108]
 8005ac2: 1822         	adds	r2, r4, r0
 8005ac4: 981c         	ldr	r0, [sp, #0x70]
 8005ac6: f8d2 3108    	ldr.w	r3, [r2, #0x108]
 8005aca: 1a59         	subs	r1, r3, r1
 8005acc: 1823         	adds	r3, r4, r0
 8005ace: 62d9         	str	r1, [r3, #0x2c]
 8005ad0: 004b         	lsls	r3, r1, #0x1
 8005ad2: ea83 70e1    	eor.w	r0, r3, r1, asr #31
 8005ad6: f8c9 0018    	str.w	r0, [r9, #0x18]
 8005ada: 900a         	str	r0, [sp, #0x28]
 8005adc: 9821         	ldr	r0, [sp, #0x84]
 8005ade: f8d2 1108    	ldr.w	r1, [r2, #0x108]
 8005ae2: 1822         	adds	r2, r4, r0
 8005ae4: f8d2 3108    	ldr.w	r3, [r2, #0x108]
 8005ae8: 1a59         	subs	r1, r3, r1
 8005aea: eb04 0308    	add.w	r3, r4, r8
 8005aee: f8dd 8060    	ldr.w	r8, [sp, #0x60]
 8005af2: 62d9         	str	r1, [r3, #0x2c]
 8005af4: 004b         	lsls	r3, r1, #0x1
 8005af6: ea83 70e1    	eor.w	r0, r3, r1, asr #31
 8005afa: eb04 030a    	add.w	r3, r4, r10
 8005afe: f8c9 001c    	str.w	r0, [r9, #0x1c]
 8005b02: f50a 7a84    	add.w	r10, r10, #0x108
 8005b06: 9009         	str	r0, [sp, #0x24]
 8005b08: f8d2 1108    	ldr.w	r1, [r2, #0x108]
 8005b0c: f8d3 2108    	ldr.w	r2, [r3, #0x108]
 8005b10: 9820         	ldr	r0, [sp, #0x80]
 8005b12: 1a51         	subs	r1, r2, r1
 8005b14: f8cd a058    	str.w	r10, [sp, #0x58]
 8005b18: 1822         	adds	r2, r4, r0
 8005b1a: f8dd a044    	ldr.w	r10, [sp, #0x44]
 8005b1e: 62d1         	str	r1, [r2, #0x2c]
 8005b20: 004a         	lsls	r2, r1, #0x1
 8005b22: ea82 72e1    	eor.w	r2, r2, r1, asr #31
 8005b26: f8c9 2020    	str.w	r2, [r9, #0x20]
 8005b2a: f8d3 1108    	ldr.w	r1, [r3, #0x108]
 8005b2e: eb04 030b    	add.w	r3, r4, r11
 8005b32: f50a 7a84    	add.w	r10, r10, #0x108
 8005b36: f50b 7b84    	add.w	r11, r11, #0x108
 8005b3a: f8d3 c108    	ldr.w	r12, [r3, #0x108]
 8005b3e: f8cd b05c    	str.w	r11, [sp, #0x5c]
 8005b42: ebac 0101    	sub.w	r1, r12, r1
 8005b46: 46ac         	mov	r12, r5
 8005b48: eb04 0508    	add.w	r5, r4, r8
 8005b4c: f108 082c    	add.w	r8, r8, #0x2c
 8005b50: f8cd 8060    	str.w	r8, [sp, #0x60]
 8005b54: 62e9         	str	r1, [r5, #0x2c]
 8005b56: 004d         	lsls	r5, r1, #0x1
 8005b58: ea85 71e1    	eor.w	r1, r5, r1, asr #31
 8005b5c: f8c9 1024    	str.w	r1, [r9, #0x24]
 8005b60: f8d3 3108    	ldr.w	r3, [r3, #0x108]
 8005b64: f8de 51f8    	ldr.w	r5, [lr, #0x1f8]
 8005b68: 1aeb         	subs	r3, r5, r3
 8005b6a: 19a5         	adds	r5, r4, r6
 8005b6c: 62eb         	str	r3, [r5, #0x2c]
 8005b6e: 005d         	lsls	r5, r3, #0x1
 8005b70: ea85 73e3    	eor.w	r3, r5, r3, asr #31
 8005b74: f8c9 3028    	str.w	r3, [r9, #0x28]
 8005b78: eb04 050c    	add.w	r5, r4, r12
 8005b7c: 4634         	mov	r4, r6
 8005b7e: f8de 61f8    	ldr.w	r6, [lr, #0x1f8]
 8005b82: f10c 0c2c    	add.w	r12, r12, #0x2c
 8005b86: f8de 0210    	ldr.w	r0, [lr, #0x210]
 8005b8a: 1b80         	subs	r0, r0, r6
 8005b8c: 62e8         	str	r0, [r5, #0x2c]
 8005b8e: 9d19         	ldr	r5, [sp, #0x64]
 8005b90: 0046         	lsls	r6, r0, #0x1
 8005b92: ea86 70e0    	eor.w	r0, r6, r0, asr #31
 8005b96: 9e0f         	ldr	r6, [sp, #0x3c]
 8005b98: f8c9 002c    	str.w	r0, [r9, #0x2c]
 8005b9c: 442e         	add	r6, r5
 8005b9e: 9d0e         	ldr	r5, [sp, #0x38]
 8005ba0: f8dd 904c    	ldr.w	r9, [sp, #0x4c]
 8005ba4: 442e         	add	r6, r5
 8005ba6: 9d0d         	ldr	r5, [sp, #0x34]
 8005ba8: f109 092c    	add.w	r9, r9, #0x2c
 8005bac: 442e         	add	r6, r5
 8005bae: 9d0c         	ldr	r5, [sp, #0x30]
 8005bb0: 442e         	add	r6, r5
 8005bb2: 9d0b         	ldr	r5, [sp, #0x2c]
 8005bb4: 442e         	add	r6, r5
 8005bb6: 9d0a         	ldr	r5, [sp, #0x28]
 8005bb8: 442e         	add	r6, r5
 8005bba: 9d09         	ldr	r5, [sp, #0x24]
 8005bbc: 442e         	add	r6, r5
 8005bbe: 9d14         	ldr	r5, [sp, #0x50]
 8005bc0: 4432         	add	r2, r6
 8005bc2: f104 062c    	add.w	r6, r4, #0x2c
 8005bc6: 4411         	add	r1, r2
 8005bc8: 9a12         	ldr	r2, [sp, #0x48]
 8005bca: 4419         	add	r1, r3
 8005bcc: f105 032c    	add.w	r3, r5, #0x2c
 8005bd0: 4408         	add	r0, r1
 8005bd2: 9019         	str	r0, [sp, #0x64]
 8005bd4: 981a         	ldr	r0, [sp, #0x68]
 8005bd6: f502 7284    	add.w	r2, r2, #0x108
 8005bda: 9915         	ldr	r1, [sp, #0x54]
 8005bdc: 302c         	adds	r0, #0x2c
 8005bde: 901a         	str	r0, [sp, #0x68]
 8005be0: 981d         	ldr	r0, [sp, #0x74]
 8005be2: 312c         	adds	r1, #0x2c
 8005be4: 9d23         	ldr	r5, [sp, #0x8c]
 8005be6: f500 7084    	add.w	r0, r0, #0x108
 8005bea: 901d         	str	r0, [sp, #0x74]
 8005bec: 981b         	ldr	r0, [sp, #0x6c]
 8005bee: 352c         	adds	r5, #0x2c
 8005bf0: 9523         	str	r5, [sp, #0x8c]
 8005bf2: 302c         	adds	r0, #0x2c
 8005bf4: 901b         	str	r0, [sp, #0x6c]
 8005bf6: 981e         	ldr	r0, [sp, #0x78]
 8005bf8: f500 7084    	add.w	r0, r0, #0x108
 8005bfc: 901e         	str	r0, [sp, #0x78]
 8005bfe: 981f         	ldr	r0, [sp, #0x7c]
 8005c00: f500 7084    	add.w	r0, r0, #0x108
 8005c04: 901f         	str	r0, [sp, #0x7c]
 8005c06: 981c         	ldr	r0, [sp, #0x70]
 8005c08: 302c         	adds	r0, #0x2c
 8005c0a: 901c         	str	r0, [sp, #0x70]
 8005c0c: 9821         	ldr	r0, [sp, #0x84]
 8005c0e: f500 7084    	add.w	r0, r0, #0x108
 8005c12: 9021         	str	r0, [sp, #0x84]
 8005c14: 9822         	ldr	r0, [sp, #0x88]
 8005c16: f500 7084    	add.w	r0, r0, #0x108
 8005c1a: 9022         	str	r0, [sp, #0x88]
 8005c1c: 9820         	ldr	r0, [sp, #0x80]
 8005c1e: 302c         	adds	r0, #0x2c
 8005c20: 9020         	str	r0, [sp, #0x80]
 8005c22: 9810         	ldr	r0, [sp, #0x40]
 8005c24: 302c         	adds	r0, #0x2c
 8005c26: f47f aee9    	bne.w	0x80059fc <raylar_crcl_encoder_service::benchmark::prepare::hb9734b8d5dc7f78e+0x312> @ imm = #-0x22e
 8005c2a: 9805         	ldr	r0, [sp, #0x14]
 8005c2c: 9919         	ldr	r1, [sp, #0x64]
 8005c2e: 6001         	str	r1, [r0]
 8005c30: f241 0005    	movw	r0, #0x1005
 8005c34: f2c0 0040    	movt	r0, #0x40
 8005c38: b191         	cbz	r1, 0x8005c60 <raylar_crcl_encoder_service::benchmark::prepare::hb9734b8d5dc7f78e+0x576> @ imm = #0x24
 8005c3a: 9e19         	ldr	r6, [sp, #0x64]
 8005c3c: fba6 0100    	umull	r0, r1, r6, r0
 8005c40: 1a70         	subs	r0, r6, r1
 8005c42: eb01 0050    	add.w	r0, r1, r0, lsr #1
 8005c46: 2101         	movs	r1, #0x1
 8005c48: 0a42         	lsrs	r2, r0, #0x9
 8005c4a: eba2 2382    	sub.w	r3, r2, r2, lsl #10
 8005c4e: 199b         	adds	r3, r3, r6
 8005c50: bf18         	it	ne
 8005c52: eb01 2250    	addne.w	r2, r1, r0, lsr #9
 8005c56: fab2 f082    	clz	r0, r2
 8005c5a: f1c0 001f    	rsb.w	r0, r0, #0x1f
 8005c5e: e000         	b	0x8005c62 <raylar_crcl_encoder_service::benchmark::prepare::hb9734b8d5dc7f78e+0x578> @ imm = #0x0
 8005c60: 2000         	movs	r0, #0x0
 8005c62: 9924         	ldr	r1, [sp, #0x90]
 8005c64: f24f 0204    	movw	r2, #0xf004
 8005c68: f6cf 72ff    	movt	r2, #0xffff
 8005c6c: f88d 0150    	strb.w	r0, [sp, #0x150]
 8005c70: b2c0         	uxtb	r0, r0
 8005c72: 7909         	ldrb	r1, [r1, #0x4]
 8005c74: 4288         	cmp	r0, r1
 8005c76: f040 86be    	bne.w	0x80069f6 <raylar_crcl_encoder_service::benchmark::prepare::hb9734b8d5dc7f78e+0x130c> @ imm = #0xd7c
 8005c7a: 9804         	ldr	r0, [sp, #0x10]
 8005c7c: f248 040c    	movw	r4, #0x800c
 8005c80: 9905         	ldr	r1, [sp, #0x14]
 8005c82: f04f 0800    	mov.w	r8, #0x0
 8005c86: f2c0 0401    	movt	r4, #0x1
 8005c8a: f64b 7e58    	movw	lr, #0xbf58
 8005c8e: 6800         	ldr	r0, [r0]
 8005c90: 4694         	mov	r12, r2
 8005c92: 6148         	str	r0, [r1, #0x14]
 8005c94: f642 70f8    	movw	r0, #0x2ff8
 8005c98: f2c0 0001    	movt	r0, #0x1
 8005c9c: f100 0128    	add.w	r1, r0, #0x28
 8005ca0: 9119         	str	r1, [sp, #0x64]
 8005ca2: f44f 4140    	mov.w	r1, #0xc000
 8005ca6: 9118         	str	r1, [sp, #0x60]
 8005ca8: f64b 7170    	movw	r1, #0xbf70
 8005cac: 9123         	str	r1, [sp, #0x8c]
 8005cae: f64b 71e8    	movw	r1, #0xbfe8
 8005cb2: 9117         	str	r1, [sp, #0x5c]
 8005cb4: f44f 3198    	mov.w	r1, #0x13000
 8005cb8: 9122         	str	r1, [sp, #0x88]
 8005cba: f64b 71d0    	movw	r1, #0xbfd0
 8005cbe: f100 0324    	add.w	r3, r0, #0x24
 8005cc2: 9116         	str	r1, [sp, #0x58]
 8005cc4: f64b 7188    	movw	r1, #0xbf88
 8005cc8: 931b         	str	r3, [sp, #0x6c]
 8005cca: f100 0320    	add.w	r3, r0, #0x20
 8005cce: 9121         	str	r1, [sp, #0x84]
 8005cd0: f64b 71b8    	movw	r1, #0xbfb8
 8005cd4: f100 061c    	add.w	r6, r0, #0x1c
 8005cd8: f100 0518    	add.w	r5, r0, #0x18
 8005cdc: f100 0914    	add.w	r9, r0, #0x14
 8005ce0: f100 0b04    	add.w	r11, r0, #0x4
 8005ce4: 931a         	str	r3, [sp, #0x68]
 8005ce6: f64b 7328    	movw	r3, #0xbf28
 8005cea: 9120         	str	r1, [sp, #0x80]
 8005cec: f64b 71a0    	movw	r1, #0xbfa0
 8005cf0: 911f         	str	r1, [sp, #0x7c]
 8005cf2: f100 0110    	add.w	r1, r0, #0x10
 8005cf6: 911e         	str	r1, [sp, #0x78]
 8005cf8: a912         	add	r1, sp, #0x48
 8005cfa: 46aa         	mov	r10, r5
 8005cfc: e881 0218    	stm.w	r1, {r3, r4, r9}
 8005d00: 9924         	ldr	r1, [sp, #0x90]
 8005d02: 9515         	str	r5, [sp, #0x54]
 8005d04: 18cd         	adds	r5, r1, r3
 8005d06: 901d         	str	r0, [sp, #0x74]
 8005d08: 981d         	ldr	r0, [sp, #0x74]
 8005d0a: 961c         	str	r6, [sp, #0x70]
 8005d0c: f8d5 3108    	ldr.w	r3, [r5, #0x108]
 8005d10: 4408         	add	r0, r1
 8005d12: f8d5 6120    	ldr.w	r6, [r5, #0x120]
 8005d16: f8cd 8038    	str.w	r8, [sp, #0x38]
 8005d1a: eb01 0804    	add.w	r8, r1, r4
 8005d1e: 1af3         	subs	r3, r6, r3
 8005d20: 62c3         	str	r3, [r0, #0x2c]
 8005d22: 9a23         	ldr	r2, [sp, #0x8c]
 8005d24: 0058         	lsls	r0, r3, #0x1
 8005d26: e9cd be10    	strd	r11, lr, [sp, #64]
 8005d2a: ea80 70e3    	eor.w	r0, r0, r3, asr #31
 8005d2e: eb01 030e    	add.w	r3, r1, lr
 8005d32: f8c8 0004    	str.w	r0, [r8, #0x4]
 8005d36: 900d         	str	r0, [sp, #0x34]
 8005d38: f8d5 0120    	ldr.w	r0, [r5, #0x120]
 8005d3c: f8d3 6108    	ldr.w	r6, [r3, #0x108]
 8005d40: 9c17         	ldr	r4, [sp, #0x5c]
 8005d42: 1a30         	subs	r0, r6, r0
 8005d44: eb01 060b    	add.w	r6, r1, r11
 8005d48: f8dd b058    	ldr.w	r11, [sp, #0x58]
 8005d4c: 62f0         	str	r0, [r6, #0x2c]
 8005d4e: 0046         	lsls	r6, r0, #0x1
 8005d50: ea86 70e0    	eor.w	r0, r6, r0, asr #31
 8005d54: f8c8 0008    	str.w	r0, [r8, #0x8]
 8005d58: 900c         	str	r0, [sp, #0x30]
 8005d5a: f8d3 0108    	ldr.w	r0, [r3, #0x108]
 8005d5e: 188b         	adds	r3, r1, r2
 8005d60: 9a22         	ldr	r2, [sp, #0x88]
 8005d62: f8d3 6108    	ldr.w	r6, [r3, #0x108]
 8005d66: f8dd e068    	ldr.w	lr, [sp, #0x68]
 8005d6a: 1a30         	subs	r0, r6, r0
 8005d6c: 188e         	adds	r6, r1, r2
 8005d6e: 9a21         	ldr	r2, [sp, #0x84]
 8005d70: 62f0         	str	r0, [r6, #0x2c]
 8005d72: 0046         	lsls	r6, r0, #0x1
 8005d74: ea86 70e0    	eor.w	r0, r6, r0, asr #31
 8005d78: f8c8 000c    	str.w	r0, [r8, #0xc]
 8005d7c: 900b         	str	r0, [sp, #0x2c]
 8005d7e: f244 062c    	movw	r6, #0x402c
 8005d82: f8d3 0108    	ldr.w	r0, [r3, #0x108]
 8005d86: 188b         	adds	r3, r1, r2
 8005d88: f2c0 0601    	movt	r6, #0x1
 8005d8c: f8cd c03c    	str.w	r12, [sp, #0x3c]
 8005d90: f8d3 2108    	ldr.w	r2, [r3, #0x108]
 8005d94: 1a10         	subs	r0, r2, r0
 8005d96: eb01 020c    	add.w	r2, r1, r12
 8005d9a: f8dd c06c    	ldr.w	r12, [sp, #0x6c]
 8005d9e: 5190         	str	r0, [r2, r6]
 8005da0: 0042         	lsls	r2, r0, #0x1
 8005da2: ea82 70e0    	eor.w	r0, r2, r0, asr #31
 8005da6: 9a1f         	ldr	r2, [sp, #0x7c]
 8005da8: eb01 060e    	add.w	r6, r1, lr
 8005dac: f8c8 0010    	str.w	r0, [r8, #0x10]
 8005db0: f10e 0e2c    	add.w	lr, lr, #0x2c
 8005db4: 440a         	add	r2, r1
 8005db6: 900a         	str	r0, [sp, #0x28]
 8005db8: f8d3 0108    	ldr.w	r0, [r3, #0x108]
 8005dbc: f8d2 3108    	ldr.w	r3, [r2, #0x108]
 8005dc0: f8cd e068    	str.w	lr, [sp, #0x68]
 8005dc4: 1a18         	subs	r0, r3, r0
 8005dc6: 9b1e         	ldr	r3, [sp, #0x78]
 8005dc8: f8dd e044    	ldr.w	lr, [sp, #0x44]
 8005dcc: 440b         	add	r3, r1
 8005dce: f50e 7e84    	add.w	lr, lr, #0x108
 8005dd2: 62d8         	str	r0, [r3, #0x2c]
 8005dd4: 0043         	lsls	r3, r0, #0x1
 8005dd6: ea83 70e0    	eor.w	r0, r3, r0, asr #31
 8005dda: f8c8 0014    	str.w	r0, [r8, #0x14]
 8005dde: 9009         	str	r0, [sp, #0x24]
 8005de0: f8d2 0108    	ldr.w	r0, [r2, #0x108]
 8005de4: 9a20         	ldr	r2, [sp, #0x80]
 8005de6: 440a         	add	r2, r1
 8005de8: f8d2 3108    	ldr.w	r3, [r2, #0x108]
 8005dec: 1a18         	subs	r0, r3, r0
 8005dee: eb01 0309    	add.w	r3, r1, r9
 8005df2: 62d8         	str	r0, [r3, #0x2c]
 8005df4: 0043         	lsls	r3, r0, #0x1
 8005df6: ea83 70e0    	eor.w	r0, r3, r0, asr #31
 8005dfa: f8c8 0018    	str.w	r0, [r8, #0x18]
 8005dfe: 9008         	str	r0, [sp, #0x20]
 8005e00: f8d2 0108    	ldr.w	r0, [r2, #0x108]
 8005e04: eb01 020b    	add.w	r2, r1, r11
 8005e08: f50b 7b84    	add.w	r11, r11, #0x108
 8005e0c: f8cd b058    	str.w	r11, [sp, #0x58]
 8005e10: f8d2 3108    	ldr.w	r3, [r2, #0x108]
 8005e14: f8dd b040    	ldr.w	r11, [sp, #0x40]
 8005e18: 1a18         	subs	r0, r3, r0
 8005e1a: eb01 030a    	add.w	r3, r1, r10
 8005e1e: f8dd a060    	ldr.w	r10, [sp, #0x60]
 8005e22: f10b 0b2c    	add.w	r11, r11, #0x2c
 8005e26: 62d8         	str	r0, [r3, #0x2c]
 8005e28: 0043         	lsls	r3, r0, #0x1
 8005e2a: ea83 70e0    	eor.w	r0, r3, r0, asr #31
 8005e2e: f8c8 001c    	str.w	r0, [r8, #0x1c]
 8005e32: 9007         	str	r0, [sp, #0x1c]
 8005e34: f8d2 0108    	ldr.w	r0, [r2, #0x108]
 8005e38: 190a         	adds	r2, r1, r4
 8005e3a: f504 7484    	add.w	r4, r4, #0x108
 8005e3e: 9417         	str	r4, [sp, #0x5c]
 8005e40: f8d2 3108    	ldr.w	r3, [r2, #0x108]
 8005e44: 9c13         	ldr	r4, [sp, #0x4c]
 8005e46: 1a18         	subs	r0, r3, r0
 8005e48: 9b1c         	ldr	r3, [sp, #0x70]
 8005e4a: 342c         	adds	r4, #0x2c
 8005e4c: 440b         	add	r3, r1
 8005e4e: 62d8         	str	r0, [r3, #0x2c]
 8005e50: 0043         	lsls	r3, r0, #0x1
 8005e52: ea83 73e0    	eor.w	r3, r3, r0, asr #31
 8005e56: f8c8 3020    	str.w	r3, [r8, #0x20]
 8005e5a: f8d2 0108    	ldr.w	r0, [r2, #0x108]
 8005e5e: eb01 020a    	add.w	r2, r1, r10
 8005e62: f50a 7a84    	add.w	r10, r10, #0x108
 8005e66: f8cd a060    	str.w	r10, [sp, #0x60]
 8005e6a: f8d2 9108    	ldr.w	r9, [r2, #0x108]
 8005e6e: eba9 0000    	sub.w	r0, r9, r0
 8005e72: 62f0         	str	r0, [r6, #0x2c]
 8005e74: f8dd 9064    	ldr.w	r9, [sp, #0x64]
 8005e78: 0046         	lsls	r6, r0, #0x1
 8005e7a: ea86 70e0    	eor.w	r0, r6, r0, asr #31
 8005e7e: f8c8 0024    	str.w	r0, [r8, #0x24]
 8005e82: f8d2 2108    	ldr.w	r2, [r2, #0x108]
 8005e86: f8d5 61f8    	ldr.w	r6, [r5, #0x1f8]
 8005e8a: 1ab2         	subs	r2, r6, r2
 8005e8c: eb01 060c    	add.w	r6, r1, r12
 8005e90: f10c 0c2c    	add.w	r12, r12, #0x2c
 8005e94: f8cd c06c    	str.w	r12, [sp, #0x6c]
 8005e98: 62f2         	str	r2, [r6, #0x2c]
 8005e9a: 0056         	lsls	r6, r2, #0x1
 8005e9c: ea86 72e2    	eor.w	r2, r6, r2, asr #31
 8005ea0: f8c8 2028    	str.w	r2, [r8, #0x28]
 8005ea4: eb01 0609    	add.w	r6, r1, r9
 8005ea8: f8d5 11f8    	ldr.w	r1, [r5, #0x1f8]
 8005eac: f8d5 5210    	ldr.w	r5, [r5, #0x210]
 8005eb0: f8dd c03c    	ldr.w	r12, [sp, #0x3c]
 8005eb4: 1a69         	subs	r1, r5, r1
 8005eb6: 62f1         	str	r1, [r6, #0x2c]
 8005eb8: f11c 0c2c    	adds.w	r12, r12, #0x2c
 8005ebc: ea4f 0541    	lsl.w	r5, r1, #0x1
 8005ec0: ea85 71e1    	eor.w	r1, r5, r1, asr #31
 8005ec4: e9dd 650d    	ldrd	r6, r5, [sp, #52]
 8005ec8: f8c8 102c    	str.w	r1, [r8, #0x2c]
 8005ecc: 4435         	add	r5, r6
 8005ece: 9e0c         	ldr	r6, [sp, #0x30]
 8005ed0: 4435         	add	r5, r6
 8005ed2: 9e0b         	ldr	r6, [sp, #0x2c]
 8005ed4: 4435         	add	r5, r6
 8005ed6: 9e0a         	ldr	r6, [sp, #0x28]
 8005ed8: 442e         	add	r6, r5
 8005eda: 9d09         	ldr	r5, [sp, #0x24]
 8005edc: 442e         	add	r6, r5
 8005ede: 9d08         	ldr	r5, [sp, #0x20]
 8005ee0: 442e         	add	r6, r5
 8005ee2: 9d07         	ldr	r5, [sp, #0x1c]
 8005ee4: 442e         	add	r6, r5
 8005ee6: 9d15         	ldr	r5, [sp, #0x54]
 8005ee8: 4433         	add	r3, r6
 8005eea: 4418         	add	r0, r3
 8005eec: 9b12         	ldr	r3, [sp, #0x48]
 8005eee: 4410         	add	r0, r2
 8005ef0: 464a         	mov	r2, r9
 8005ef2: eb01 0800    	add.w	r8, r1, r0
 8005ef6: 991f         	ldr	r1, [sp, #0x7c]
 8005ef8: e9dd 601c    	ldrd	r6, r0, [sp, #112]
 8005efc: f105 052c    	add.w	r5, r5, #0x2c
 8005f00: f501 7184    	add.w	r1, r1, #0x108
 8005f04: 911f         	str	r1, [sp, #0x7c]
 8005f06: 991e         	ldr	r1, [sp, #0x78]
 8005f08: f106 062c    	add.w	r6, r6, #0x2c
 8005f0c: f8dd 9050    	ldr.w	r9, [sp, #0x50]
 8005f10: f503 7384    	add.w	r3, r3, #0x108
 8005f14: f101 012c    	add.w	r1, r1, #0x2c
 8005f18: 911e         	str	r1, [sp, #0x78]
 8005f1a: 9920         	ldr	r1, [sp, #0x80]
 8005f1c: f109 092c    	add.w	r9, r9, #0x2c
 8005f20: f100 002c    	add.w	r0, r0, #0x2c
 8005f24: f102 022c    	add.w	r2, r2, #0x2c
 8005f28: f501 7184    	add.w	r1, r1, #0x108
 8005f2c: 9120         	str	r1, [sp, #0x80]
 8005f2e: 9921         	ldr	r1, [sp, #0x84]
 8005f30: 9219         	str	r2, [sp, #0x64]
 8005f32: f501 7184    	add.w	r1, r1, #0x108
 8005f36: 9121         	str	r1, [sp, #0x84]
 8005f38: 9922         	ldr	r1, [sp, #0x88]
 8005f3a: f101 012c    	add.w	r1, r1, #0x2c
 8005f3e: 9122         	str	r1, [sp, #0x88]
 8005f40: 9923         	ldr	r1, [sp, #0x8c]
 8005f42: f501 7184    	add.w	r1, r1, #0x108
 8005f46: 9123         	str	r1, [sp, #0x8c]
 8005f48: f47f aed6    	bne.w	0x8005cf8 <raylar_crcl_encoder_service::benchmark::prepare::hb9734b8d5dc7f78e+0x60e> @ imm = #-0x254
 8005f4c: 9805         	ldr	r0, [sp, #0x14]
 8005f4e: f1b8 0f00    	cmp.w	r8, #0x0
 8005f52: f8c0 8004    	str.w	r8, [r0, #0x4]
 8005f56: d017         	beq	0x8005f88 <raylar_crcl_encoder_service::benchmark::prepare::hb9734b8d5dc7f78e+0x89e> @ imm = #0x2e
 8005f58: f241 0005    	movw	r0, #0x1005
 8005f5c: f2c0 0040    	movt	r0, #0x40
 8005f60: fba8 0100    	umull	r0, r1, r8, r0
 8005f64: eba8 0001    	sub.w	r0, r8, r1
 8005f68: eb01 0050    	add.w	r0, r1, r0, lsr #1
 8005f6c: 2101         	movs	r1, #0x1
 8005f6e: 0a42         	lsrs	r2, r0, #0x9
 8005f70: eba2 2382    	sub.w	r3, r2, r2, lsl #10
 8005f74: eb13 0308    	adds.w	r3, r3, r8
 8005f78: bf18         	it	ne
 8005f7a: eb01 2250    	addne.w	r2, r1, r0, lsr #9
 8005f7e: fab2 f082    	clz	r0, r2
 8005f82: f1c0 001f    	rsb.w	r0, r0, #0x1f
 8005f86: e000         	b	0x8005f8a <raylar_crcl_encoder_service::benchmark::prepare::hb9734b8d5dc7f78e+0x8a0> @ imm = #0x0
 8005f88: 2000         	movs	r0, #0x0
 8005f8a: 9924         	ldr	r1, [sp, #0x90]
 8005f8c: f24f 0204    	movw	r2, #0xf004
 8005f90: f6cf 72ff    	movt	r2, #0xffff
 8005f94: f88d 0150    	strb.w	r0, [sp, #0x150]
 8005f98: b2c0         	uxtb	r0, r0
 8005f9a: 7b09         	ldrb	r1, [r1, #0xc]
 8005f9c: 4288         	cmp	r0, r1
 8005f9e: f040 851e    	bne.w	0x80069de <raylar_crcl_encoder_service::benchmark::prepare::hb9734b8d5dc7f78e+0x12f4> @ imm = #0xa3c
 8005fa2: 9804         	ldr	r0, [sp, #0x10]
 8005fa4: f249 0408    	movw	r4, #0x9008
 8005fa8: 9905         	ldr	r1, [sp, #0x14]
 8005faa: f04f 0800    	mov.w	r8, #0x0
 8005fae: f2c0 0401    	movt	r4, #0x1
 8005fb2: f64b 7e5c    	movw	lr, #0xbf5c
 8005fb6: 68c0         	ldr	r0, [r0, #0xc]
 8005fb8: 4694         	mov	r12, r2
 8005fba: 6188         	str	r0, [r1, #0x18]
 8005fbc: f643 70f4    	movw	r0, #0x3ff4
 8005fc0: f2c0 0001    	movt	r0, #0x1
 8005fc4: f100 0128    	add.w	r1, r0, #0x28
 8005fc8: 9119         	str	r1, [sp, #0x64]
 8005fca: f24c 0104    	movw	r1, #0xc004
 8005fce: 9118         	str	r1, [sp, #0x60]
 8005fd0: f64b 7174    	movw	r1, #0xbf74
 8005fd4: 9123         	str	r1, [sp, #0x8c]
 8005fd6: f64b 71ec    	movw	r1, #0xbfec
 8005fda: 9117         	str	r1, [sp, #0x5c]
 8005fdc: f64b 71d4    	movw	r1, #0xbfd4
 8005fe0: 9116         	str	r1, [sp, #0x58]
 8005fe2: f64b 718c    	movw	r1, #0xbf8c
 8005fe6: f100 0324    	add.w	r3, r0, #0x24
 8005fea: 9122         	str	r1, [sp, #0x88]
 8005fec: f64b 71bc    	movw	r1, #0xbfbc
 8005ff0: 931b         	str	r3, [sp, #0x6c]
 8005ff2: f100 0320    	add.w	r3, r0, #0x20
 8005ff6: 9121         	str	r1, [sp, #0x84]
 8005ff8: f44f 31a0    	mov.w	r1, #0x14000
 8005ffc: f100 061c    	add.w	r6, r0, #0x1c
 8006000: f100 0518    	add.w	r5, r0, #0x18
 8006004: f100 0914    	add.w	r9, r0, #0x14
 8006008: f100 0b08    	add.w	r11, r0, #0x8
 800600c: 931a         	str	r3, [sp, #0x68]
 800600e: f64b 732c    	movw	r3, #0xbf2c
 8006012: 9120         	str	r1, [sp, #0x80]
 8006014: f64b 71a4    	movw	r1, #0xbfa4
 8006018: 911f         	str	r1, [sp, #0x7c]
 800601a: 1d01         	adds	r1, r0, #0x4
 800601c: 911e         	str	r1, [sp, #0x78]
 800601e: a90f         	add	r1, sp, #0x3c
 8006020: 46aa         	mov	r10, r5
 8006022: e881 4900    	stm.w	r1, {r8, r11, lr}
 8006026: a912         	add	r1, sp, #0x48
 8006028: e881 0218    	stm.w	r1, {r3, r4, r9}
 800602c: 9924         	ldr	r1, [sp, #0x90]
 800602e: 9515         	str	r5, [sp, #0x54]
 8006030: 18cd         	adds	r5, r1, r3
 8006032: 901d         	str	r0, [sp, #0x74]
 8006034: 981d         	ldr	r0, [sp, #0x74]
 8006036: eb01 0804    	add.w	r8, r1, r4
 800603a: 961c         	str	r6, [sp, #0x70]
 800603c: f8d5 3108    	ldr.w	r3, [r5, #0x108]
 8006040: 4408         	add	r0, r1
 8006042: f8d5 6120    	ldr.w	r6, [r5, #0x120]
 8006046: 9a1e         	ldr	r2, [sp, #0x78]
 8006048: 1af3         	subs	r3, r6, r3
 800604a: 62c3         	str	r3, [r0, #0x2c]
 800604c: 9c18         	ldr	r4, [sp, #0x60]
 800604e: 0058         	lsls	r0, r3, #0x1
 8006050: f8cd c038    	str.w	r12, [sp, #0x38]
 8006054: ea80 70e3    	eor.w	r0, r0, r3, asr #31
 8006058: eb01 030e    	add.w	r3, r1, lr
 800605c: f8c8 0004    	str.w	r0, [r8, #0x4]
 8006060: 900d         	str	r0, [sp, #0x34]
 8006062: f8d5 0120    	ldr.w	r0, [r5, #0x120]
 8006066: f8d3 6108    	ldr.w	r6, [r3, #0x108]
 800606a: f8dd e068    	ldr.w	lr, [sp, #0x68]
 800606e: 1a30         	subs	r0, r6, r0
 8006070: 188e         	adds	r6, r1, r2
 8006072: 9a23         	ldr	r2, [sp, #0x8c]
 8006074: 62f0         	str	r0, [r6, #0x2c]
 8006076: 0046         	lsls	r6, r0, #0x1
 8006078: ea86 70e0    	eor.w	r0, r6, r0, asr #31
 800607c: f8c8 0008    	str.w	r0, [r8, #0x8]
 8006080: 900c         	str	r0, [sp, #0x30]
 8006082: f8d3 0108    	ldr.w	r0, [r3, #0x108]
 8006086: 188b         	adds	r3, r1, r2
 8006088: 9a22         	ldr	r2, [sp, #0x88]
 800608a: f8d3 6108    	ldr.w	r6, [r3, #0x108]
 800608e: 1a30         	subs	r0, r6, r0
 8006090: eb01 060b    	add.w	r6, r1, r11
 8006094: f8dd b058    	ldr.w	r11, [sp, #0x58]
 8006098: 62f0         	str	r0, [r6, #0x2c]
 800609a: 0046         	lsls	r6, r0, #0x1
 800609c: ea86 70e0    	eor.w	r0, r6, r0, asr #31
 80060a0: f8c8 000c    	str.w	r0, [r8, #0xc]
 80060a4: 900b         	str	r0, [sp, #0x2c]
 80060a6: f245 062c    	movw	r6, #0x502c
 80060aa: f8d3 0108    	ldr.w	r0, [r3, #0x108]
 80060ae: 188b         	adds	r3, r1, r2
 80060b0: f2c0 0601    	movt	r6, #0x1
 80060b4: f8d3 2108    	ldr.w	r2, [r3, #0x108]
 80060b8: 1a10         	subs	r0, r2, r0
 80060ba: 9a20         	ldr	r2, [sp, #0x80]
 80060bc: 440a         	add	r2, r1
 80060be: 62d0         	str	r0, [r2, #0x2c]
 80060c0: 0042         	lsls	r2, r0, #0x1
 80060c2: ea82 70e0    	eor.w	r0, r2, r0, asr #31
 80060c6: 9a1f         	ldr	r2, [sp, #0x7c]
 80060c8: f8c8 0010    	str.w	r0, [r8, #0x10]
 80060cc: 440a         	add	r2, r1
 80060ce: 900a         	str	r0, [sp, #0x28]
 80060d0: f8d3 0108    	ldr.w	r0, [r3, #0x108]
 80060d4: f8d2 3108    	ldr.w	r3, [r2, #0x108]
 80060d8: 1a18         	subs	r0, r3, r0
 80060da: eb01 030c    	add.w	r3, r1, r12
 80060de: f8dd c06c    	ldr.w	r12, [sp, #0x6c]
 80060e2: 5198         	str	r0, [r3, r6]
 80060e4: 0043         	lsls	r3, r0, #0x1
 80060e6: ea83 70e0    	eor.w	r0, r3, r0, asr #31
 80060ea: f8c8 0014    	str.w	r0, [r8, #0x14]
 80060ee: eb01 060e    	add.w	r6, r1, lr
 80060f2: 9009         	str	r0, [sp, #0x24]
 80060f4: f10e 0e2c    	add.w	lr, lr, #0x2c
 80060f8: f8d2 0108    	ldr.w	r0, [r2, #0x108]
 80060fc: 9a21         	ldr	r2, [sp, #0x84]
 80060fe: f8cd e068    	str.w	lr, [sp, #0x68]
 8006102: 440a         	add	r2, r1
 8006104: f8dd e044    	ldr.w	lr, [sp, #0x44]
 8006108: f8d2 3108    	ldr.w	r3, [r2, #0x108]
 800610c: f50e 7e84    	add.w	lr, lr, #0x108
 8006110: 1a18         	subs	r0, r3, r0
 8006112: eb01 0309    	add.w	r3, r1, r9
 8006116: 62d8         	str	r0, [r3, #0x2c]
 8006118: 0043         	lsls	r3, r0, #0x1
 800611a: ea83 70e0    	eor.w	r0, r3, r0, asr #31
 800611e: f8c8 0018    	str.w	r0, [r8, #0x18]
 8006122: 9008         	str	r0, [sp, #0x20]
 8006124: f8d2 0108    	ldr.w	r0, [r2, #0x108]
 8006128: eb01 020b    	add.w	r2, r1, r11
 800612c: f50b 7b84    	add.w	r11, r11, #0x108
 8006130: f8cd b058    	str.w	r11, [sp, #0x58]
 8006134: f8d2 3108    	ldr.w	r3, [r2, #0x108]
 8006138: f8dd b040    	ldr.w	r11, [sp, #0x40]
 800613c: 1a18         	subs	r0, r3, r0
 800613e: eb01 030a    	add.w	r3, r1, r10
 8006142: f8dd a05c    	ldr.w	r10, [sp, #0x5c]
 8006146: f10b 0b2c    	add.w	r11, r11, #0x2c
 800614a: 62d8         	str	r0, [r3, #0x2c]
 800614c: 0043         	lsls	r3, r0, #0x1
 800614e: ea83 70e0    	eor.w	r0, r3, r0, asr #31
 8006152: f8c8 001c    	str.w	r0, [r8, #0x1c]
 8006156: 9007         	str	r0, [sp, #0x1c]
 8006158: f8d2 0108    	ldr.w	r0, [r2, #0x108]
 800615c: eb01 020a    	add.w	r2, r1, r10
 8006160: f50a 7a84    	add.w	r10, r10, #0x108
 8006164: f8cd a05c    	str.w	r10, [sp, #0x5c]
 8006168: f8d2 3108    	ldr.w	r3, [r2, #0x108]
 800616c: 1a18         	subs	r0, r3, r0
 800616e: 9b1c         	ldr	r3, [sp, #0x70]
 8006170: 440b         	add	r3, r1
 8006172: 62d8         	str	r0, [r3, #0x2c]
 8006174: 0043         	lsls	r3, r0, #0x1
 8006176: ea83 73e0    	eor.w	r3, r3, r0, asr #31
 800617a: f8c8 3020    	str.w	r3, [r8, #0x20]
 800617e: f8d2 0108    	ldr.w	r0, [r2, #0x108]
 8006182: 190a         	adds	r2, r1, r4
 8006184: f504 7484    	add.w	r4, r4, #0x108
 8006188: 9418         	str	r4, [sp, #0x60]
 800618a: f8d2 9108    	ldr.w	r9, [r2, #0x108]
 800618e: 9c13         	ldr	r4, [sp, #0x4c]
 8006190: eba9 0000    	sub.w	r0, r9, r0
 8006194: 62f0         	str	r0, [r6, #0x2c]
 8006196: f8dd 9064    	ldr.w	r9, [sp, #0x64]
 800619a: 342c         	adds	r4, #0x2c
 800619c: 0046         	lsls	r6, r0, #0x1
 800619e: ea86 70e0    	eor.w	r0, r6, r0, asr #31
 80061a2: f8c8 0024    	str.w	r0, [r8, #0x24]
 80061a6: f8d2 2108    	ldr.w	r2, [r2, #0x108]
 80061aa: f8d5 61f8    	ldr.w	r6, [r5, #0x1f8]
 80061ae: 1ab2         	subs	r2, r6, r2
 80061b0: eb01 060c    	add.w	r6, r1, r12
 80061b4: f10c 0c2c    	add.w	r12, r12, #0x2c
 80061b8: f8cd c06c    	str.w	r12, [sp, #0x6c]
 80061bc: 62f2         	str	r2, [r6, #0x2c]
 80061be: 0056         	lsls	r6, r2, #0x1
 80061c0: ea86 72e2    	eor.w	r2, r6, r2, asr #31
 80061c4: f8c8 2028    	str.w	r2, [r8, #0x28]
 80061c8: eb01 0609    	add.w	r6, r1, r9
 80061cc: f8d5 11f8    	ldr.w	r1, [r5, #0x1f8]
 80061d0: f8d5 5210    	ldr.w	r5, [r5, #0x210]
 80061d4: f8dd c038    	ldr.w	r12, [sp, #0x38]
 80061d8: 1a69         	subs	r1, r5, r1
 80061da: 62f1         	str	r1, [r6, #0x2c]
 80061dc: 9e0d         	ldr	r6, [sp, #0x34]
 80061de: f11c 0c2c    	adds.w	r12, r12, #0x2c
 80061e2: ea4f 0541    	lsl.w	r5, r1, #0x1
 80061e6: ea85 71e1    	eor.w	r1, r5, r1, asr #31
 80061ea: 9d0f         	ldr	r5, [sp, #0x3c]
 80061ec: f8c8 102c    	str.w	r1, [r8, #0x2c]
 80061f0: 4435         	add	r5, r6
 80061f2: 9e0c         	ldr	r6, [sp, #0x30]
 80061f4: 4435         	add	r5, r6
 80061f6: 9e0b         	ldr	r6, [sp, #0x2c]
 80061f8: 4435         	add	r5, r6
 80061fa: 9e0a         	ldr	r6, [sp, #0x28]
 80061fc: 442e         	add	r6, r5
 80061fe: 9d09         	ldr	r5, [sp, #0x24]
 8006200: 442e         	add	r6, r5
 8006202: 9d08         	ldr	r5, [sp, #0x20]
 8006204: 442e         	add	r6, r5
 8006206: 9d07         	ldr	r5, [sp, #0x1c]
 8006208: 442e         	add	r6, r5
 800620a: 9d15         	ldr	r5, [sp, #0x54]
 800620c: 4433         	add	r3, r6
 800620e: 4418         	add	r0, r3
 8006210: 9b12         	ldr	r3, [sp, #0x48]
 8006212: 4410         	add	r0, r2
 8006214: 464a         	mov	r2, r9
 8006216: eb01 0800    	add.w	r8, r1, r0
 800621a: 991f         	ldr	r1, [sp, #0x7c]
 800621c: e9dd 601c    	ldrd	r6, r0, [sp, #112]
 8006220: f105 052c    	add.w	r5, r5, #0x2c
 8006224: f501 7184    	add.w	r1, r1, #0x108
 8006228: 911f         	str	r1, [sp, #0x7c]
 800622a: 9920         	ldr	r1, [sp, #0x80]
 800622c: f106 062c    	add.w	r6, r6, #0x2c
 8006230: f8dd 9050    	ldr.w	r9, [sp, #0x50]
 8006234: f503 7384    	add.w	r3, r3, #0x108
 8006238: f101 012c    	add.w	r1, r1, #0x2c
 800623c: 9120         	str	r1, [sp, #0x80]
 800623e: 9921         	ldr	r1, [sp, #0x84]
 8006240: f109 092c    	add.w	r9, r9, #0x2c
 8006244: f100 002c    	add.w	r0, r0, #0x2c
 8006248: f102 022c    	add.w	r2, r2, #0x2c
 800624c: f501 7184    	add.w	r1, r1, #0x108
 8006250: 9121         	str	r1, [sp, #0x84]
 8006252: 9922         	ldr	r1, [sp, #0x88]
 8006254: 9219         	str	r2, [sp, #0x64]
 8006256: f501 7184    	add.w	r1, r1, #0x108
 800625a: 9122         	str	r1, [sp, #0x88]
 800625c: 9923         	ldr	r1, [sp, #0x8c]
 800625e: f501 7184    	add.w	r1, r1, #0x108
 8006262: 9123         	str	r1, [sp, #0x8c]
 8006264: 991e         	ldr	r1, [sp, #0x78]
 8006266: f101 012c    	add.w	r1, r1, #0x2c
 800626a: 911e         	str	r1, [sp, #0x78]
 800626c: f47f aed7    	bne.w	0x800601e <raylar_crcl_encoder_service::benchmark::prepare::hb9734b8d5dc7f78e+0x934> @ imm = #-0x252
 8006270: 9805         	ldr	r0, [sp, #0x14]
 8006272: f1b8 0f00    	cmp.w	r8, #0x0
 8006276: f8c0 8008    	str.w	r8, [r0, #0x8]
 800627a: d017         	beq	0x80062ac <raylar_crcl_encoder_service::benchmark::prepare::hb9734b8d5dc7f78e+0xbc2> @ imm = #0x2e
 800627c: f241 0005    	movw	r0, #0x1005
 8006280: f2c0 0040    	movt	r0, #0x40
 8006284: fba8 0100    	umull	r0, r1, r8, r0
 8006288: eba8 0001    	sub.w	r0, r8, r1
 800628c: eb01 0050    	add.w	r0, r1, r0, lsr #1
 8006290: 2101         	movs	r1, #0x1
 8006292: 0a42         	lsrs	r2, r0, #0x9
 8006294: eba2 2382    	sub.w	r3, r2, r2, lsl #10
 8006298: eb13 0308    	adds.w	r3, r3, r8
 800629c: bf18         	it	ne
 800629e: eb01 2250    	addne.w	r2, r1, r0, lsr #9
 80062a2: fab2 f082    	clz	r0, r2
 80062a6: f1c0 001f    	rsb.w	r0, r0, #0x1f
 80062aa: e000         	b	0x80062ae <raylar_crcl_encoder_service::benchmark::prepare::hb9734b8d5dc7f78e+0xbc4> @ imm = #0x0
 80062ac: 2000         	movs	r0, #0x0
 80062ae: 9924         	ldr	r1, [sp, #0x90]
 80062b0: f24f 0204    	movw	r2, #0xf004
 80062b4: f6cf 72ff    	movt	r2, #0xffff
 80062b8: f88d 0150    	strb.w	r0, [sp, #0x150]
 80062bc: b2c0         	uxtb	r0, r0
 80062be: 7d09         	ldrb	r1, [r1, #0x14]
 80062c0: 4288         	cmp	r0, r1
 80062c2: f040 838f    	bne.w	0x80069e4 <raylar_crcl_encoder_service::benchmark::prepare::hb9734b8d5dc7f78e+0x12fa> @ imm = #0x71e
 80062c6: 9804         	ldr	r0, [sp, #0x10]
 80062c8: f644 7ef0    	movw	lr, #0x4ff0
 80062cc: 9905         	ldr	r1, [sp, #0x14]
 80062ce: f2c0 0e01    	movt	lr, #0x1
 80062d2: f10e 0320    	add.w	r3, lr, #0x20
 80062d6: f10e 061c    	add.w	r6, lr, #0x1c
 80062da: 6980         	ldr	r0, [r0, #0x18]
 80062dc: f10e 0908    	add.w	r9, lr, #0x8
 80062e0: 61c8         	str	r0, [r1, #0x1c]
 80062e2: f24c 0108    	movw	r1, #0xc008
 80062e6: f10e 0028    	add.w	r0, lr, #0x28
 80062ea: 9117         	str	r1, [sp, #0x5c]
 80062ec: f64b 7178    	movw	r1, #0xbf78
 80062f0: 901a         	str	r0, [sp, #0x68]
 80062f2: f10e 0024    	add.w	r0, lr, #0x24
 80062f6: 9122         	str	r1, [sp, #0x88]
 80062f8: f64b 71f0    	movw	r1, #0xbff0
 80062fc: 9019         	str	r0, [sp, #0x64]
 80062fe: f10e 0018    	add.w	r0, lr, #0x18
 8006302: 9116         	str	r1, [sp, #0x58]
 8006304: f64b 7190    	movw	r1, #0xbf90
 8006308: 9018         	str	r0, [sp, #0x60]
 800630a: f10e 000c    	add.w	r0, lr, #0xc
 800630e: 9121         	str	r1, [sp, #0x84]
 8006310: f64b 71c0    	movw	r1, #0xbfc0
 8006314: 9023         	str	r0, [sp, #0x8c]
 8006316: 2000         	movs	r0, #0x0
 8006318: 9115         	str	r1, [sp, #0x54]
 800631a: f44f 31a8    	mov.w	r1, #0x15000
 800631e: f10e 0804    	add.w	r8, lr, #0x4
 8006322: 901b         	str	r0, [sp, #0x6c]
 8006324: f64b 7030    	movw	r0, #0xbf30
 8006328: 9120         	str	r1, [sp, #0x80]
 800632a: f64b 71a8    	movw	r1, #0xbfa8
 800632e: f64b 7560    	movw	r5, #0xbf60
 8006332: f64b 7ad8    	movw	r10, #0xbfd8
 8006336: 4694         	mov	r12, r2
 8006338: 911f         	str	r1, [sp, #0x7c]
 800633a: 9c24         	ldr	r4, [sp, #0x90]
 800633c: e9cd 631c    	strd	r6, r3, [sp, #112]
 8006340: 1826         	adds	r6, r4, r0
 8006342: f8cd e078    	str.w	lr, [sp, #0x78]
 8006346: e9cd 5013    	strd	r5, r0, [sp, #76]
 800634a: eb04 0e0c    	add.w	lr, r4, r12
 800634e: f8d6 0108    	ldr.w	r0, [r6, #0x108]
 8006352: f8d6 1120    	ldr.w	r1, [r6, #0x120]
 8006356: 9a1e         	ldr	r2, [sp, #0x78]
 8006358: 1a08         	subs	r0, r1, r0
 800635a: 960b         	str	r6, [sp, #0x2c]
 800635c: 18a3         	adds	r3, r4, r2
 800635e: f8dd b054    	ldr.w	r11, [sp, #0x54]
 8006362: 0041         	lsls	r1, r0, #0x1
 8006364: e9cd 9a11    	strd	r9, r10, [sp, #68]
 8006368: ea81 71e0    	eor.w	r1, r1, r0, asr #31
 800636c: 62d8         	str	r0, [r3, #0x2c]
 800636e: f24b 0004    	movw	r0, #0xb004
 8006372: 910e         	str	r1, [sp, #0x38]
 8006374: f2c0 0001    	movt	r0, #0x1
 8006378: e9cd c80f    	strd	r12, r8, [sp, #60]
 800637c: f84e 1000    	str.w	r1, [lr, r0]
 8006380: 1961         	adds	r1, r4, r5
 8006382: f8d6 0120    	ldr.w	r0, [r6, #0x120]
 8006386: f8d1 3108    	ldr.w	r3, [r1, #0x108]
 800638a: 1a18         	subs	r0, r3, r0
 800638c: eb04 0308    	add.w	r3, r4, r8
 8006390: f8dd 8058    	ldr.w	r8, [sp, #0x58]
 8006394: 62d8         	str	r0, [r3, #0x2c]
 8006396: 0043         	lsls	r3, r0, #0x1
 8006398: ea83 72e0    	eor.w	r2, r3, r0, asr #31
 800639c: f24b 0008    	movw	r0, #0xb008
 80063a0: f2c0 0001    	movt	r0, #0x1
 80063a4: 920d         	str	r2, [sp, #0x34]
 80063a6: f84e 2000    	str.w	r2, [lr, r0]
 80063aa: f8d1 0108    	ldr.w	r0, [r1, #0x108]
 80063ae: 9922         	ldr	r1, [sp, #0x88]
 80063b0: 4421         	add	r1, r4
 80063b2: f8d1 3108    	ldr.w	r3, [r1, #0x108]
 80063b6: 1a18         	subs	r0, r3, r0
 80063b8: eb04 0309    	add.w	r3, r4, r9
 80063bc: f8dd 9060    	ldr.w	r9, [sp, #0x60]
 80063c0: 62d8         	str	r0, [r3, #0x2c]
 80063c2: 0043         	lsls	r3, r0, #0x1
 80063c4: ea83 72e0    	eor.w	r2, r3, r0, asr #31
 80063c8: f24b 000c    	movw	r0, #0xb00c
 80063cc: f2c0 0001    	movt	r0, #0x1
 80063d0: 920c         	str	r2, [sp, #0x30]
 80063d2: f84e 2000    	str.w	r2, [lr, r0]
 80063d6: f8d1 0108    	ldr.w	r0, [r1, #0x108]
 80063da: 9921         	ldr	r1, [sp, #0x84]
 80063dc: 9a23         	ldr	r2, [sp, #0x8c]
 80063de: 4421         	add	r1, r4
 80063e0: f8d1 5108    	ldr.w	r5, [r1, #0x108]
 80063e4: 1a28         	subs	r0, r5, r0
 80063e6: 18a5         	adds	r5, r4, r2
 80063e8: 62e8         	str	r0, [r5, #0x2c]
 80063ea: 0045         	lsls	r5, r0, #0x1
 80063ec: ea85 72e0    	eor.w	r2, r5, r0, asr #31
 80063f0: f24b 0010    	movw	r0, #0xb010
 80063f4: f2c0 0001    	movt	r0, #0x1
 80063f8: 920a         	str	r2, [sp, #0x28]
 80063fa: f84e 2000    	str.w	r2, [lr, r0]
 80063fe: f8d1 0108    	ldr.w	r0, [r1, #0x108]
 8006402: 991f         	ldr	r1, [sp, #0x7c]
 8006404: 9a20         	ldr	r2, [sp, #0x80]
 8006406: 4421         	add	r1, r4
 8006408: f8d1 6108    	ldr.w	r6, [r1, #0x108]
 800640c: 1a30         	subs	r0, r6, r0
 800640e: 18a6         	adds	r6, r4, r2
 8006410: 62f0         	str	r0, [r6, #0x2c]
 8006412: 0046         	lsls	r6, r0, #0x1
 8006414: ea86 72e0    	eor.w	r2, r6, r0, asr #31
 8006418: f24b 0014    	movw	r0, #0xb014
 800641c: f2c0 0001    	movt	r0, #0x1
 8006420: 9209         	str	r2, [sp, #0x24]
 8006422: f84e 2000    	str.w	r2, [lr, r0]
 8006426: f246 022c    	movw	r2, #0x602c
 800642a: f2c0 0201    	movt	r2, #0x1
 800642e: f8d1 0108    	ldr.w	r0, [r1, #0x108]
 8006432: eb04 010b    	add.w	r1, r4, r11
 8006436: 9e1d         	ldr	r6, [sp, #0x74]
 8006438: f50b 7b84    	add.w	r11, r11, #0x108
 800643c: f8d1 3108    	ldr.w	r3, [r1, #0x108]
 8006440: f8cd b054    	str.w	r11, [sp, #0x54]
 8006444: 1a18         	subs	r0, r3, r0
 8006446: f84e 0002    	str.w	r0, [lr, r2]
 800644a: 0043         	lsls	r3, r0, #0x1
 800644c: ea83 72e0    	eor.w	r2, r3, r0, asr #31
 8006450: f24b 0018    	movw	r0, #0xb018
 8006454: eb04 030a    	add.w	r3, r4, r10
 8006458: f2c0 0001    	movt	r0, #0x1
 800645c: 9208         	str	r2, [sp, #0x20]
 800645e: f84e 2000    	str.w	r2, [lr, r0]
 8006462: f8d1 0108    	ldr.w	r0, [r1, #0x108]
 8006466: f8d3 1108    	ldr.w	r1, [r3, #0x108]
 800646a: 9a1c         	ldr	r2, [sp, #0x70]
 800646c: 1a08         	subs	r0, r1, r0
 800646e: eb04 0109    	add.w	r1, r4, r9
 8006472: 4422         	add	r2, r4
 8006474: f109 092c    	add.w	r9, r9, #0x2c
 8006478: 62c8         	str	r0, [r1, #0x2c]
 800647a: 0041         	lsls	r1, r0, #0x1
 800647c: ea81 71e0    	eor.w	r1, r1, r0, asr #31
 8006480: f24b 001c    	movw	r0, #0xb01c
 8006484: f2c0 0001    	movt	r0, #0x1
 8006488: f8cd 9060    	str.w	r9, [sp, #0x60]
 800648c: f84e 1000    	str.w	r1, [lr, r0]
 8006490: f8d3 0108    	ldr.w	r0, [r3, #0x108]
 8006494: eb04 0308    	add.w	r3, r4, r8
 8006498: f508 7884    	add.w	r8, r8, #0x108
 800649c: f8dd 9044    	ldr.w	r9, [sp, #0x44]
 80064a0: f8d3 c108    	ldr.w	r12, [r3, #0x108]
 80064a4: f8cd 8058    	str.w	r8, [sp, #0x58]
 80064a8: f109 092c    	add.w	r9, r9, #0x2c
 80064ac: ebac 0000    	sub.w	r0, r12, r0
 80064b0: 62d0         	str	r0, [r2, #0x2c]
 80064b2: f8dd c064    	ldr.w	r12, [sp, #0x64]
 80064b6: 0042         	lsls	r2, r0, #0x1
 80064b8: f8dd 8040    	ldr.w	r8, [sp, #0x40]
 80064bc: ea82 72e0    	eor.w	r2, r2, r0, asr #31
 80064c0: f24b 0020    	movw	r0, #0xb020
 80064c4: f2c0 0001    	movt	r0, #0x1
 80064c8: 9207         	str	r2, [sp, #0x1c]
 80064ca: f84e 2000    	str.w	r2, [lr, r0]
 80064ce: 4620         	mov	r0, r4
 80064d0: f108 082c    	add.w	r8, r8, #0x2c
 80064d4: 9c17         	ldr	r4, [sp, #0x5c]
 80064d6: f8d3 2108    	ldr.w	r2, [r3, #0x108]
 80064da: 1903         	adds	r3, r0, r4
 80064dc: f504 7484    	add.w	r4, r4, #0x108
 80064e0: 9417         	str	r4, [sp, #0x5c]
 80064e2: f8d3 5108    	ldr.w	r5, [r3, #0x108]
 80064e6: 1aaa         	subs	r2, r5, r2
 80064e8: 1985         	adds	r5, r0, r6
 80064ea: 9e0b         	ldr	r6, [sp, #0x2c]
 80064ec: 62ea         	str	r2, [r5, #0x2c]
 80064ee: 0055         	lsls	r5, r2, #0x1
 80064f0: ea85 7ae2    	eor.w	r10, r5, r2, asr #31
 80064f4: f24b 0224    	movw	r2, #0xb024
 80064f8: f2c0 0201    	movt	r2, #0x1
 80064fc: f84e a002    	str.w	r10, [lr, r2]
 8006500: f24b 0228    	movw	r2, #0xb028
 8006504: f2c0 0201    	movt	r2, #0x1
 8006508: f8d3 3108    	ldr.w	r3, [r3, #0x108]
 800650c: f8d6 51f8    	ldr.w	r5, [r6, #0x1f8]
 8006510: 1aeb         	subs	r3, r5, r3
 8006512: eb00 050c    	add.w	r5, r0, r12
 8006516: f10c 0c2c    	add.w	r12, r12, #0x2c
 800651a: f8cd c064    	str.w	r12, [sp, #0x64]
 800651e: 62eb         	str	r3, [r5, #0x2c]
 8006520: 005d         	lsls	r5, r3, #0x1
 8006522: ea85 73e3    	eor.w	r3, r5, r3, asr #31
 8006526: f84e 3002    	str.w	r3, [lr, r2]
 800652a: 9a1a         	ldr	r2, [sp, #0x68]
 800652c: f8dd c03c    	ldr.w	r12, [sp, #0x3c]
 8006530: 1885         	adds	r5, r0, r2
 8006532: f8d6 01f8    	ldr.w	r0, [r6, #0x1f8]
 8006536: f8d6 6210    	ldr.w	r6, [r6, #0x210]
 800653a: f11c 0c2c    	adds.w	r12, r12, #0x2c
 800653e: f102 022c    	add.w	r2, r2, #0x2c
 8006542: 921a         	str	r2, [sp, #0x68]
 8006544: eba6 0000    	sub.w	r0, r6, r0
 8006548: 62e8         	str	r0, [r5, #0x2c]
 800654a: 9e1b         	ldr	r6, [sp, #0x6c]
 800654c: ea4f 0540    	lsl.w	r5, r0, #0x1
 8006550: ea85 70e0    	eor.w	r0, r5, r0, asr #31
 8006554: f24b 052c    	movw	r5, #0xb02c
 8006558: f2c0 0501    	movt	r5, #0x1
 800655c: f84e 0005    	str.w	r0, [lr, r5]
 8006560: 9d0e         	ldr	r5, [sp, #0x38]
 8006562: f8dd e078    	ldr.w	lr, [sp, #0x78]
 8006566: 442e         	add	r6, r5
 8006568: 9d0d         	ldr	r5, [sp, #0x34]
 800656a: f10e 0e2c    	add.w	lr, lr, #0x2c
 800656e: 442e         	add	r6, r5
 8006570: 9d0c         	ldr	r5, [sp, #0x30]
 8006572: 442e         	add	r6, r5
 8006574: 9d0a         	ldr	r5, [sp, #0x28]
 8006576: 442e         	add	r6, r5
 8006578: 9d09         	ldr	r5, [sp, #0x24]
 800657a: 442e         	add	r6, r5
 800657c: 9d08         	ldr	r5, [sp, #0x20]
 800657e: 442e         	add	r6, r5
 8006580: 9d1d         	ldr	r5, [sp, #0x74]
 8006582: 4431         	add	r1, r6
 8006584: 9e07         	ldr	r6, [sp, #0x1c]
 8006586: 4431         	add	r1, r6
 8006588: 9e1c         	ldr	r6, [sp, #0x70]
 800658a: 4451         	add	r1, r10
 800658c: f8dd a048    	ldr.w	r10, [sp, #0x48]
 8006590: 4419         	add	r1, r3
 8006592: 9b22         	ldr	r3, [sp, #0x88]
 8006594: 4408         	add	r0, r1
 8006596: 991f         	ldr	r1, [sp, #0x7c]
 8006598: 901b         	str	r0, [sp, #0x6c]
 800659a: f503 7384    	add.w	r3, r3, #0x108
 800659e: f501 7184    	add.w	r1, r1, #0x108
 80065a2: 911f         	str	r1, [sp, #0x7c]
 80065a4: 9920         	ldr	r1, [sp, #0x80]
 80065a6: f50a 7a84    	add.w	r10, r10, #0x108
 80065aa: 9814         	ldr	r0, [sp, #0x50]
 80065ac: f106 062c    	add.w	r6, r6, #0x2c
 80065b0: f101 012c    	add.w	r1, r1, #0x2c
 80065b4: 9120         	str	r1, [sp, #0x80]
 80065b6: 9923         	ldr	r1, [sp, #0x8c]
 80065b8: f500 7084    	add.w	r0, r0, #0x108
 80065bc: 9322         	str	r3, [sp, #0x88]
 80065be: f105 032c    	add.w	r3, r5, #0x2c
 80065c2: f101 012c    	add.w	r1, r1, #0x2c
 80065c6: 9123         	str	r1, [sp, #0x8c]
 80065c8: 9921         	ldr	r1, [sp, #0x84]
 80065ca: f501 7184    	add.w	r1, r1, #0x108
 80065ce: 9121         	str	r1, [sp, #0x84]
 80065d0: 9913         	ldr	r1, [sp, #0x4c]
 80065d2: f501 7584    	add.w	r5, r1, #0x108
 80065d6: f47f aeb0    	bne.w	0x800633a <raylar_crcl_encoder_service::benchmark::prepare::hb9734b8d5dc7f78e+0xc50> @ imm = #-0x2a0
 80065da: 9805         	ldr	r0, [sp, #0x14]
 80065dc: 991b         	ldr	r1, [sp, #0x6c]
 80065de: 60c1         	str	r1, [r0, #0xc]
 80065e0: b1b1         	cbz	r1, 0x8006610 <raylar_crcl_encoder_service::benchmark::prepare::hb9734b8d5dc7f78e+0xf26> @ imm = #0x2c
 80065e2: 9e1b         	ldr	r6, [sp, #0x6c]
 80065e4: f241 0005    	movw	r0, #0x1005
 80065e8: f2c0 0040    	movt	r0, #0x40
 80065ec: fba6 0100    	umull	r0, r1, r6, r0
 80065f0: 1a70         	subs	r0, r6, r1
 80065f2: eb01 0050    	add.w	r0, r1, r0, lsr #1
 80065f6: 2101         	movs	r1, #0x1
 80065f8: 0a42         	lsrs	r2, r0, #0x9
 80065fa: eba2 2382    	sub.w	r3, r2, r2, lsl #10
 80065fe: 199b         	adds	r3, r3, r6
 8006600: bf18         	it	ne
 8006602: eb01 2250    	addne.w	r2, r1, r0, lsr #9
 8006606: fab2 f082    	clz	r0, r2
 800660a: f1c0 001f    	rsb.w	r0, r0, #0x1f
 800660e: e000         	b	0x8006612 <raylar_crcl_encoder_service::benchmark::prepare::hb9734b8d5dc7f78e+0xf28> @ imm = #0x0
 8006610: 2000         	movs	r0, #0x0
 8006612: 9924         	ldr	r1, [sp, #0x90]
 8006614: f24f 0c04    	movw	r12, #0xf004
 8006618: f6cf 7cff    	movt	r12, #0xffff
 800661c: f88d 0150    	strb.w	r0, [sp, #0x150]
 8006620: b2c0         	uxtb	r0, r0
 8006622: 7f09         	ldrb	r1, [r1, #0x1c]
 8006624: 4288         	cmp	r0, r1
 8006626: f040 81e0    	bne.w	0x80069ea <raylar_crcl_encoder_service::benchmark::prepare::hb9734b8d5dc7f78e+0x1300> @ imm = #0x3c0
 800662a: 9804         	ldr	r0, [sp, #0x10]
 800662c: f64b 7364    	movw	r3, #0xbf64
 8006630: 9905         	ldr	r1, [sp, #0x14]
 8006632: f44f 34d8    	mov.w	r4, #0x1b000
 8006636: 9322         	str	r3, [sp, #0x88]
 8006638: f24c 030c    	movw	r3, #0xc00c
 800663c: 6a40         	ldr	r0, [r0, #0x24]
 800663e: f64b 7adc    	movw	r10, #0xbfdc
 8006642: 6208         	str	r0, [r1, #0x20]
 8006644: f645 70ec    	movw	r0, #0x5fec
 8006648: f2c0 0001    	movt	r0, #0x1
 800664c: f100 0128    	add.w	r1, r0, #0x28
 8006650: 9318         	str	r3, [sp, #0x60]
 8006652: f64b 737c    	movw	r3, #0xbf7c
 8006656: 911b         	str	r1, [sp, #0x6c]
 8006658: f100 0124    	add.w	r1, r0, #0x24
 800665c: 9321         	str	r3, [sp, #0x84]
 800665e: f64b 73f4    	movw	r3, #0xbff4
 8006662: 911a         	str	r1, [sp, #0x68]
 8006664: f100 0120    	add.w	r1, r0, #0x20
 8006668: 9317         	str	r3, [sp, #0x5c]
 800666a: f64b 7394    	movw	r3, #0xbf94
 800666e: 9119         	str	r1, [sp, #0x64]
 8006670: f100 0110    	add.w	r1, r0, #0x10
 8006674: 9320         	str	r3, [sp, #0x80]
 8006676: f44f 33b0    	mov.w	r3, #0x16000
 800667a: 9123         	str	r1, [sp, #0x8c]
 800667c: 2100         	movs	r1, #0x0
 800667e: 9316         	str	r3, [sp, #0x58]
 8006680: f64b 73c4    	movw	r3, #0xbfc4
 8006684: f100 081c    	add.w	r8, r0, #0x1c
 8006688: f100 060c    	add.w	r6, r0, #0xc
 800668c: f100 0b08    	add.w	r11, r0, #0x8
 8006690: f100 0904    	add.w	r9, r0, #0x4
 8006694: 911c         	str	r1, [sp, #0x70]
 8006696: f64b 7134    	movw	r1, #0xbf34
 800669a: 931f         	str	r3, [sp, #0x7c]
 800669c: f64b 73ac    	movw	r3, #0xbfac
 80066a0: 931e         	str	r3, [sp, #0x78]
 80066a2: 9b24         	ldr	r3, [sp, #0x90]
 80066a4: 46d6         	mov	lr, r10
 80066a6: e9cd a110    	strd	r10, r1, [sp, #64]
 80066aa: 4632         	mov	r2, r6
 80066ac: eb03 0a01    	add.w	r10, r3, r1
 80066b0: e9cd 6813    	strd	r6, r8, [sp, #76]
 80066b4: 1819         	adds	r1, r3, r0
 80066b6: 9015         	str	r0, [sp, #0x54]
 80066b8: f8da 6108    	ldr.w	r6, [r10, #0x108]
 80066bc: f8da 5120    	ldr.w	r5, [r10, #0x120]
 80066c0: e9cd 9b0e    	strd	r9, r11, [sp, #56]
 80066c4: 1bae         	subs	r6, r5, r6
 80066c6: 62ce         	str	r6, [r1, #0x2c]
 80066c8: f8cd c074    	str.w	r12, [sp, #0x74]
 80066cc: 46c4         	mov	r12, r8
 80066ce: 0071         	lsls	r1, r6, #0x1
 80066d0: ea81 70e6    	eor.w	r0, r1, r6, asr #31
 80066d4: 191e         	adds	r6, r3, r4
 80066d6: 900d         	str	r0, [sp, #0x34]
 80066d8: f8dd 805c    	ldr.w	r8, [sp, #0x5c]
 80066dc: 6070         	str	r0, [r6, #0x4]
 80066de: 9822         	ldr	r0, [sp, #0x88]
 80066e0: f8da 1120    	ldr.w	r1, [r10, #0x120]
 80066e4: 181d         	adds	r5, r3, r0
 80066e6: 9412         	str	r4, [sp, #0x48]
 80066e8: 9c19         	ldr	r4, [sp, #0x64]
 80066ea: f8d5 0108    	ldr.w	r0, [r5, #0x108]
 80066ee: 1a40         	subs	r0, r0, r1
 80066f0: eb03 0109    	add.w	r1, r3, r9
 80066f4: 62c8         	str	r0, [r1, #0x2c]
 80066f6: 0041         	lsls	r1, r0, #0x1
 80066f8: ea81 70e0    	eor.w	r0, r1, r0, asr #31
 80066fc: 9921         	ldr	r1, [sp, #0x84]
 80066fe: 60b0         	str	r0, [r6, #0x8]
 8006700: 4419         	add	r1, r3
 8006702: 900c         	str	r0, [sp, #0x30]
 8006704: f8d5 0108    	ldr.w	r0, [r5, #0x108]
 8006708: f8d1 5108    	ldr.w	r5, [r1, #0x108]
 800670c: 1a28         	subs	r0, r5, r0
 800670e: eb03 050b    	add.w	r5, r3, r11
 8006712: f8dd b058    	ldr.w	r11, [sp, #0x58]
 8006716: 62e8         	str	r0, [r5, #0x2c]
 8006718: 0045         	lsls	r5, r0, #0x1
 800671a: ea85 70e0    	eor.w	r0, r5, r0, asr #31
 800671e: 60f0         	str	r0, [r6, #0xc]
 8006720: 900b         	str	r0, [sp, #0x2c]
 8006722: f8d1 0108    	ldr.w	r0, [r1, #0x108]
 8006726: 9920         	ldr	r1, [sp, #0x80]
 8006728: 4419         	add	r1, r3
 800672a: f8d1 5108    	ldr.w	r5, [r1, #0x108]
 800672e: 1a28         	subs	r0, r5, r0
 8006730: 189d         	adds	r5, r3, r2
 8006732: 62e8         	str	r0, [r5, #0x2c]
 8006734: 0045         	lsls	r5, r0, #0x1
 8006736: ea85 70e0    	eor.w	r0, r5, r0, asr #31
 800673a: 6130         	str	r0, [r6, #0x10]
 800673c: 900a         	str	r0, [sp, #0x28]
 800673e: f247 052c    	movw	r5, #0x702c
 8006742: f8d1 0108    	ldr.w	r0, [r1, #0x108]
 8006746: f2c0 0501    	movt	r5, #0x1
 800674a: 991e         	ldr	r1, [sp, #0x78]
 800674c: 4419         	add	r1, r3
 800674e: f8d1 2108    	ldr.w	r2, [r1, #0x108]
 8006752: 1a10         	subs	r0, r2, r0
 8006754: 9a23         	ldr	r2, [sp, #0x8c]
 8006756: 441a         	add	r2, r3
 8006758: 62d0         	str	r0, [r2, #0x2c]
 800675a: 0042         	lsls	r2, r0, #0x1
 800675c: ea82 70e0    	eor.w	r0, r2, r0, asr #31
 8006760: 6170         	str	r0, [r6, #0x14]
 8006762: 9009         	str	r0, [sp, #0x24]
 8006764: f8d1 0108    	ldr.w	r0, [r1, #0x108]
 8006768: 991f         	ldr	r1, [sp, #0x7c]
 800676a: 4419         	add	r1, r3
 800676c: f8d1 2108    	ldr.w	r2, [r1, #0x108]
 8006770: 1a10         	subs	r0, r2, r0
 8006772: eb03 020b    	add.w	r2, r3, r11
 8006776: f10b 0b2c    	add.w	r11, r11, #0x2c
 800677a: f8cd b058    	str.w	r11, [sp, #0x58]
 800677e: 62d0         	str	r0, [r2, #0x2c]
 8006780: 0042         	lsls	r2, r0, #0x1
 8006782: ea82 70e0    	eor.w	r0, r2, r0, asr #31
 8006786: 61b0         	str	r0, [r6, #0x18]
 8006788: 9008         	str	r0, [sp, #0x20]
 800678a: f8d1 0108    	ldr.w	r0, [r1, #0x108]
 800678e: eb03 010e    	add.w	r1, r3, lr
 8006792: f8dd e068    	ldr.w	lr, [sp, #0x68]
 8006796: f8d1 2108    	ldr.w	r2, [r1, #0x108]
 800679a: 1a10         	subs	r0, r2, r0
 800679c: 9a1d         	ldr	r2, [sp, #0x74]
 800679e: 441a         	add	r2, r3
 80067a0: 5150         	str	r0, [r2, r5]
 80067a2: 0042         	lsls	r2, r0, #0x1
 80067a4: ea82 70e0    	eor.w	r0, r2, r0, asr #31
 80067a8: 61f0         	str	r0, [r6, #0x1c]
 80067aa: 9007         	str	r0, [sp, #0x1c]
 80067ac: f8d1 0108    	ldr.w	r0, [r1, #0x108]
 80067b0: eb03 0108    	add.w	r1, r3, r8
 80067b4: 9d18         	ldr	r5, [sp, #0x60]
 80067b6: f508 7884    	add.w	r8, r8, #0x108
 80067ba: f8d1 2108    	ldr.w	r2, [r1, #0x108]
 80067be: f8cd 805c    	str.w	r8, [sp, #0x5c]
 80067c2: 1a10         	subs	r0, r2, r0
 80067c4: eb03 020c    	add.w	r2, r3, r12
 80067c8: f8dd c06c    	ldr.w	r12, [sp, #0x6c]
 80067cc: 62d0         	str	r0, [r2, #0x2c]
 80067ce: 0042         	lsls	r2, r0, #0x1
 80067d0: ea82 70e0    	eor.w	r0, r2, r0, asr #31
 80067d4: 6230         	str	r0, [r6, #0x20]
 80067d6: 9006         	str	r0, [sp, #0x18]
 80067d8: f8d1 0108    	ldr.w	r0, [r1, #0x108]
 80067dc: 1959         	adds	r1, r3, r5
 80067de: f505 7584    	add.w	r5, r5, #0x108
 80067e2: 9518         	str	r5, [sp, #0x60]
 80067e4: f8d1 2108    	ldr.w	r2, [r1, #0x108]
 80067e8: 1a10         	subs	r0, r2, r0
 80067ea: 191a         	adds	r2, r3, r4
 80067ec: 342c         	adds	r4, #0x2c
 80067ee: 9419         	str	r4, [sp, #0x64]
 80067f0: 62d0         	str	r0, [r2, #0x2c]
 80067f2: 0042         	lsls	r2, r0, #0x1
 80067f4: ea82 79e0    	eor.w	r9, r2, r0, asr #31
 80067f8: f8c6 9024    	str.w	r9, [r6, #0x24]
 80067fc: f8d1 1108    	ldr.w	r1, [r1, #0x108]
 8006800: f8da 21f8    	ldr.w	r2, [r10, #0x1f8]
 8006804: 9c12         	ldr	r4, [sp, #0x48]
 8006806: 1a51         	subs	r1, r2, r1
 8006808: eb03 020e    	add.w	r2, r3, lr
 800680c: 342c         	adds	r4, #0x2c
 800680e: f10e 0e2c    	add.w	lr, lr, #0x2c
 8006812: 62d1         	str	r1, [r2, #0x2c]
 8006814: 004a         	lsls	r2, r1, #0x1
 8006816: ea82 71e1    	eor.w	r1, r2, r1, asr #31
 800681a: 62b1         	str	r1, [r6, #0x28]
 800681c: eb03 020c    	add.w	r2, r3, r12
 8006820: f8da 01f8    	ldr.w	r0, [r10, #0x1f8]
 8006824: f8da 3210    	ldr.w	r3, [r10, #0x210]
 8006828: f10c 0c2c    	add.w	r12, r12, #0x2c
 800682c: f8dd a040    	ldr.w	r10, [sp, #0x40]
 8006830: 1a18         	subs	r0, r3, r0
 8006832: 62d0         	str	r0, [r2, #0x2c]
 8006834: 9b0d         	ldr	r3, [sp, #0x34]
 8006836: f50a 7a84    	add.w	r10, r10, #0x108
 800683a: 0042         	lsls	r2, r0, #0x1
 800683c: ea82 70e0    	eor.w	r0, r2, r0, asr #31
 8006840: 9a1c         	ldr	r2, [sp, #0x70]
 8006842: 62f0         	str	r0, [r6, #0x2c]
 8006844: 441a         	add	r2, r3
 8006846: 9b0c         	ldr	r3, [sp, #0x30]
 8006848: 9e13         	ldr	r6, [sp, #0x4c]
 800684a: 441a         	add	r2, r3
 800684c: 9b0b         	ldr	r3, [sp, #0x2c]
 800684e: f8cd c06c    	str.w	r12, [sp, #0x6c]
 8006852: 362c         	adds	r6, #0x2c
 8006854: 441a         	add	r2, r3
 8006856: 9b0a         	ldr	r3, [sp, #0x28]
 8006858: f8dd c074    	ldr.w	r12, [sp, #0x74]
 800685c: 441a         	add	r2, r3
 800685e: 9b09         	ldr	r3, [sp, #0x24]
 8006860: f11c 0c2c    	adds.w	r12, r12, #0x2c
 8006864: f8cd e068    	str.w	lr, [sp, #0x68]
 8006868: 441a         	add	r2, r3
 800686a: 9b08         	ldr	r3, [sp, #0x20]
 800686c: 441a         	add	r2, r3
 800686e: 9b07         	ldr	r3, [sp, #0x1c]
 8006870: 441a         	add	r2, r3
 8006872: 9b06         	ldr	r3, [sp, #0x18]
 8006874: 441a         	add	r2, r3
 8006876: 9b14         	ldr	r3, [sp, #0x50]
 8006878: 444a         	add	r2, r9
 800687a: e9dd 9b0e    	ldrd	r9, r11, [sp, #56]
 800687e: 4411         	add	r1, r2
 8006880: 9a1e         	ldr	r2, [sp, #0x78]
 8006882: 4408         	add	r0, r1
 8006884: f103 082c    	add.w	r8, r3, #0x2c
 8006888: f502 7284    	add.w	r2, r2, #0x108
 800688c: 921e         	str	r2, [sp, #0x78]
 800688e: 9a23         	ldr	r2, [sp, #0x8c]
 8006890: f10b 0b2c    	add.w	r11, r11, #0x2c
 8006894: 9b21         	ldr	r3, [sp, #0x84]
 8006896: f109 092c    	add.w	r9, r9, #0x2c
 800689a: f102 022c    	add.w	r2, r2, #0x2c
 800689e: 9223         	str	r2, [sp, #0x8c]
 80068a0: 9a1f         	ldr	r2, [sp, #0x7c]
 80068a2: f503 7384    	add.w	r3, r3, #0x108
 80068a6: 9911         	ldr	r1, [sp, #0x44]
 80068a8: 901c         	str	r0, [sp, #0x70]
 80068aa: f502 7284    	add.w	r2, r2, #0x108
 80068ae: 9815         	ldr	r0, [sp, #0x54]
 80068b0: f501 7184    	add.w	r1, r1, #0x108
 80068b4: 921f         	str	r2, [sp, #0x7c]
 80068b6: 9a20         	ldr	r2, [sp, #0x80]
 80068b8: f100 002c    	add.w	r0, r0, #0x2c
 80068bc: 9321         	str	r3, [sp, #0x84]
 80068be: 9b22         	ldr	r3, [sp, #0x88]
 80068c0: f502 7284    	add.w	r2, r2, #0x108
 80068c4: 9220         	str	r2, [sp, #0x80]
 80068c6: f503 7384    	add.w	r3, r3, #0x108
 80068ca: 9322         	str	r3, [sp, #0x88]
 80068cc: f47f aee9    	bne.w	0x80066a2 <raylar_crcl_encoder_service::benchmark::prepare::hb9734b8d5dc7f78e+0xfb8> @ imm = #-0x22e
 80068d0: 9805         	ldr	r0, [sp, #0x14]
 80068d2: 991c         	ldr	r1, [sp, #0x70]
 80068d4: 6101         	str	r1, [r0, #0x10]
 80068d6: b1b1         	cbz	r1, 0x8006906 <raylar_crcl_encoder_service::benchmark::prepare::hb9734b8d5dc7f78e+0x121c> @ imm = #0x2c
 80068d8: 9e1c         	ldr	r6, [sp, #0x70]
 80068da: f241 0005    	movw	r0, #0x1005
 80068de: f2c0 0040    	movt	r0, #0x40
 80068e2: fba6 0100    	umull	r0, r1, r6, r0
 80068e6: 1a70         	subs	r0, r6, r1
 80068e8: eb01 0050    	add.w	r0, r1, r0, lsr #1
 80068ec: 2101         	movs	r1, #0x1
 80068ee: 0a42         	lsrs	r2, r0, #0x9
 80068f0: eba2 2382    	sub.w	r3, r2, r2, lsl #10
 80068f4: 199b         	adds	r3, r3, r6
 80068f6: bf18         	it	ne
 80068f8: eb01 2250    	addne.w	r2, r1, r0, lsr #9
 80068fc: fab2 f082    	clz	r0, r2
 8006900: f1c0 001f    	rsb.w	r0, r0, #0x1f
 8006904: e000         	b	0x8006908 <raylar_crcl_encoder_service::benchmark::prepare::hb9734b8d5dc7f78e+0x121e> @ imm = #0x0
 8006906: 2000         	movs	r0, #0x0
 8006908: 9924         	ldr	r1, [sp, #0x90]
 800690a: f88d 0150    	strb.w	r0, [sp, #0x150]
 800690e: b2c0         	uxtb	r0, r0
 8006910: f891 1024    	ldrb.w	r1, [r1, #0x24]
 8006914: 4288         	cmp	r0, r1
 8006916: d16b         	bne	0x80069f0 <raylar_crcl_encoder_service::benchmark::prepare::hb9734b8d5dc7f78e+0x1306> @ imm = #0xd6
 8006918: 9804         	ldr	r0, [sp, #0x10]
 800691a: f24c 0288    	movw	r2, #0xc088
 800691e: 9905         	ldr	r1, [sp, #0x14]
 8006920: f641 0c06    	movw	r12, #0x1806
 8006924: 6b00         	ldr	r0, [r0, #0x30]
 8006926: 6248         	str	r0, [r1, #0x24]
 8006928: 2000         	movs	r0, #0x0
 800692a: 9924         	ldr	r1, [sp, #0x90]
 800692c: 188e         	adds	r6, r1, r2
 800692e: eb01 0380    	add.w	r3, r1, r0, lsl #2
 8006932: f856 5c60    	ldr	r5, [r6, #-96]
 8006936: 6a99         	ldr	r1, [r3, #0x28]
 8006938: 42a9         	cmp	r1, r5
 800693a: d138         	bne	0x80069ae <raylar_crcl_encoder_service::benchmark::prepare::hb9734b8d5dc7f78e+0x12c4> @ imm = #0x70
 800693c: f856 1c48    	ldr	r1, [r6, #-72]
 8006940: 6c1d         	ldr	r5, [r3, #0x40]
 8006942: 428d         	cmp	r5, r1
 8006944: d12e         	bne	0x80069a4 <raylar_crcl_encoder_service::benchmark::prepare::hb9734b8d5dc7f78e+0x12ba> @ imm = #0x5c
 8006946: f856 1c30    	ldr	r1, [r6, #-48]
 800694a: 6d9d         	ldr	r5, [r3, #0x58]
 800694c: 428d         	cmp	r5, r1
 800694e: d12b         	bne	0x80069a8 <raylar_crcl_encoder_service::benchmark::prepare::hb9734b8d5dc7f78e+0x12be> @ imm = #0x56
 8006950: f856 1c18    	ldr	r1, [r6, #-24]
 8006954: 6f1e         	ldr	r6, [r3, #0x70]
 8006956: 428e         	cmp	r6, r1
 8006958: d128         	bne	0x80069ac <raylar_crcl_encoder_service::benchmark::prepare::hb9734b8d5dc7f78e+0x12c2> @ imm = #0x50
 800695a: 301e         	adds	r0, #0x1e
 800695c: 4560         	cmp	r0, r12
 800695e: d00a         	beq	0x8006976 <raylar_crcl_encoder_service::benchmark::prepare::hb9734b8d5dc7f78e+0x128c> @ imm = #0x14
 8006960: 9e24         	ldr	r6, [sp, #0x90]
 8006962: f102 0178    	add.w	r1, r2, #0x78
 8006966: f8d3 3088    	ldr.w	r3, [r3, #0x88]
 800696a: 58b2         	ldr	r2, [r6, r2]
 800696c: 4293         	cmp	r3, r2
 800696e: 460a         	mov	r2, r1
 8006970: d0db         	beq	0x800692a <raylar_crcl_encoder_service::benchmark::prepare::hb9734b8d5dc7f78e+0x1240> @ imm = #-0x4a
 8006972: 3806         	subs	r0, #0x6
 8006974: e01b         	b	0x80069ae <raylar_crcl_encoder_service::benchmark::prepare::hb9734b8d5dc7f78e+0x12c4> @ imm = #0x36
 8006976: b079         	add	sp, #0x1e4
 8006978: e8bd 0f00    	pop.w	{r8, r9, r10, r11}
 800697c: bdf0         	pop	{r4, r5, r6, r7, pc}
 800697e: f64e 0050    	movw	r0, #0xe850
 8006982: f88d 1150    	strb.w	r1, [sp, #0x150]
 8006986: f6c0 0000    	movt	r0, #0x800
 800698a: f24e 539c    	movw	r3, #0xe59c
 800698e: 9000         	str	r0, [sp]
 8006990: f24e 5000    	movw	r0, #0xe500
 8006994: aa54         	add	r2, sp, #0x150
 8006996: f6c0 0000    	movt	r0, #0x800
 800699a: f6c0 0300    	movt	r3, #0x800
 800699e: 212b         	movs	r1, #0x2b
 80069a0: f005 fa82    	bl	0x800bea8 <core::result::unwrap_failed> @ imm = #0x5504
 80069a4: 3006         	adds	r0, #0x6
 80069a6: e002         	b	0x80069ae <raylar_crcl_encoder_service::benchmark::prepare::hb9734b8d5dc7f78e+0x12c4> @ imm = #0x4
 80069a8: 300c         	adds	r0, #0xc
 80069aa: e000         	b	0x80069ae <raylar_crcl_encoder_service::benchmark::prepare::hb9734b8d5dc7f78e+0x12c4> @ imm = #0x0
 80069ac: 3012         	adds	r0, #0x12
 80069ae: f64e 0120    	movw	r1, #0xe820
 80069b2: 9a02         	ldr	r2, [sp, #0x8]
 80069b4: f6c0 0100    	movt	r1, #0x800
 80069b8: 2300         	movs	r3, #0x0
 80069ba: 9101         	str	r1, [sp, #0x4]
 80069bc: eb02 0280    	add.w	r2, r2, r0, lsl #2
 80069c0: 9903         	ldr	r1, [sp, #0xc]
 80069c2: eb01 0180    	add.w	r1, r1, r0, lsl #2
 80069c6: 2000         	movs	r0, #0x0
 80069c8: f002 fbbe    	bl	0x8009148 <core::panicking::assert_failed::hef9a1b4b671d0724> @ imm = #0x277c
 80069cc: f89d 0129    	ldrb.w	r0, [sp, #0x129]
 80069d0: f88d 0150    	strb.w	r0, [sp, #0x150]
 80069d4: f64e 0040    	movw	r0, #0xe840
 80069d8: f6c0 0000    	movt	r0, #0x800
 80069dc: e7d5         	b	0x800698a <raylar_crcl_encoder_service::benchmark::prepare::hb9734b8d5dc7f78e+0x12a0> @ imm = #-0x56
 80069de: 9824         	ldr	r0, [sp, #0x90]
 80069e0: 3008         	adds	r0, #0x8
 80069e2: e007         	b	0x80069f4 <raylar_crcl_encoder_service::benchmark::prepare::hb9734b8d5dc7f78e+0x130a> @ imm = #0xe
 80069e4: 9824         	ldr	r0, [sp, #0x90]
 80069e6: 3010         	adds	r0, #0x10
 80069e8: e004         	b	0x80069f4 <raylar_crcl_encoder_service::benchmark::prepare::hb9734b8d5dc7f78e+0x130a> @ imm = #0x8
 80069ea: 9824         	ldr	r0, [sp, #0x90]
 80069ec: 3018         	adds	r0, #0x18
 80069ee: e001         	b	0x80069f4 <raylar_crcl_encoder_service::benchmark::prepare::hb9734b8d5dc7f78e+0x130a> @ imm = #0x2
 80069f0: 9824         	ldr	r0, [sp, #0x90]
 80069f2: 3020         	adds	r0, #0x20
 80069f4: 9024         	str	r0, [sp, #0x90]
 80069f6: f64e 0030    	movw	r0, #0xe830
 80069fa: a954         	add	r1, sp, #0x150
 80069fc: f6c0 0000    	movt	r0, #0x800
 8006a00: 2300         	movs	r3, #0x0
 8006a02: 9001         	str	r0, [sp, #0x4]
 8006a04: 9824         	ldr	r0, [sp, #0x90]
 8006a06: 1d02         	adds	r2, r0, #0x4
 8006a08: 2000         	movs	r0, #0x0
 8006a0a: f002 fb8f    	bl	0x800912c <core::panicking::assert_failed::h4992df13491a1642> @ imm = #0x271e

08006a0e <raylar_crcl_encoder_service::benchmark::fold_sum::h16b920496cf2076e>:
 8006a0e: b5f0         	push	{r4, r5, r6, r7, lr}
 8006a10: af03         	add	r7, sp, #0xc
 8006a12: e92d 0f00    	push.w	{r8, r9, r10, r11}
 8006a16: b083         	sub	sp, #0xc
 8006a18: f500 3190    	add.w	r1, r0, #0x12000
 8006a1c: f04f 0800    	mov.w	r8, #0x0
 8006a20: 3124         	adds	r1, #0x24
 8006a22: f240 3eff    	movw	lr, #0x3ff
 8006a26: f851 cf84    	ldr	r12, [r1, #132]!
 8006a2a: f1be 0e21    	subs.w	lr, lr, #0x21
 8006a2e: e951 5420    	ldrd	r5, r4, [r1, #-128]
 8006a32: ea4f 0644    	lsl.w	r6, r4, #0x1
 8006a36: e951 321e    	ldrd	r3, r2, [r1, #-120]
 8006a3a: ea86 76e4    	eor.w	r6, r6, r4, asr #31
 8006a3e: f851 9c08    	ldr	r9, [r1, #-8]
 8006a42: ea4f 0445    	lsl.w	r4, r5, #0x1
 8006a46: ea84 74e5    	eor.w	r4, r4, r5, asr #31
 8006a4a: eb04 0508    	add.w	r5, r4, r8
 8006a4e: ea4f 0443    	lsl.w	r4, r3, #0x1
 8006a52: ea84 73e3    	eor.w	r3, r4, r3, asr #31
 8006a56: f851 4c70    	ldr	r4, [r1, #-112]
 8006a5a: 442e         	add	r6, r5
 8006a5c: ea4f 0542    	lsl.w	r5, r2, #0x1
 8006a60: 4433         	add	r3, r6
 8006a62: ea85 72e2    	eor.w	r2, r5, r2, asr #31
 8006a66: f851 5c6c    	ldr	r5, [r1, #-108]
 8006a6a: 441a         	add	r2, r3
 8006a6c: ea4f 0344    	lsl.w	r3, r4, #0x1
 8006a70: f851 6c0c    	ldr	r6, [r1, #-12]
 8006a74: ea83 73e4    	eor.w	r3, r3, r4, asr #31
 8006a78: f851 4c68    	ldr	r4, [r1, #-104]
 8006a7c: 441a         	add	r2, r3
 8006a7e: ea4f 0345    	lsl.w	r3, r5, #0x1
 8006a82: ea83 73e5    	eor.w	r3, r3, r5, asr #31
 8006a86: f851 5c64    	ldr	r5, [r1, #-100]
 8006a8a: 441a         	add	r2, r3
 8006a8c: ea4f 0344    	lsl.w	r3, r4, #0x1
 8006a90: ea83 73e4    	eor.w	r3, r3, r4, asr #31
 8006a94: f851 4c60    	ldr	r4, [r1, #-96]
 8006a98: 441a         	add	r2, r3
 8006a9a: ea4f 0345    	lsl.w	r3, r5, #0x1
 8006a9e: ea83 73e5    	eor.w	r3, r3, r5, asr #31
 8006aa2: f851 5c5c    	ldr	r5, [r1, #-92]
 8006aa6: 441a         	add	r2, r3
 8006aa8: ea4f 0344    	lsl.w	r3, r4, #0x1
 8006aac: ea83 73e4    	eor.w	r3, r3, r4, asr #31
 8006ab0: f851 4c58    	ldr	r4, [r1, #-88]
 8006ab4: 441a         	add	r2, r3
 8006ab6: ea4f 0345    	lsl.w	r3, r5, #0x1
 8006aba: ea83 73e5    	eor.w	r3, r3, r5, asr #31
 8006abe: f851 5c54    	ldr	r5, [r1, #-84]
 8006ac2: 441a         	add	r2, r3
 8006ac4: ea4f 0344    	lsl.w	r3, r4, #0x1
 8006ac8: ea83 73e4    	eor.w	r3, r3, r4, asr #31
 8006acc: f851 4c50    	ldr	r4, [r1, #-80]
 8006ad0: 441a         	add	r2, r3
 8006ad2: ea4f 0345    	lsl.w	r3, r5, #0x1
 8006ad6: ea83 73e5    	eor.w	r3, r3, r5, asr #31
 8006ada: f851 5c4c    	ldr	r5, [r1, #-76]
 8006ade: 441a         	add	r2, r3
 8006ae0: ea4f 0344    	lsl.w	r3, r4, #0x1
 8006ae4: ea83 73e4    	eor.w	r3, r3, r4, asr #31
 8006ae8: f851 4c48    	ldr	r4, [r1, #-72]
 8006aec: 441a         	add	r2, r3
 8006aee: ea4f 0345    	lsl.w	r3, r5, #0x1
 8006af2: ea83 73e5    	eor.w	r3, r3, r5, asr #31
 8006af6: f851 5c44    	ldr	r5, [r1, #-68]
 8006afa: 441a         	add	r2, r3
 8006afc: ea4f 0344    	lsl.w	r3, r4, #0x1
 8006b00: ea83 73e4    	eor.w	r3, r3, r4, asr #31
 8006b04: f851 4c40    	ldr	r4, [r1, #-64]
 8006b08: 441a         	add	r2, r3
 8006b0a: ea4f 0345    	lsl.w	r3, r5, #0x1
 8006b0e: ea83 73e5    	eor.w	r3, r3, r5, asr #31
 8006b12: f851 5c3c    	ldr	r5, [r1, #-60]
 8006b16: 441a         	add	r2, r3
 8006b18: ea4f 0344    	lsl.w	r3, r4, #0x1
 8006b1c: ea83 73e4    	eor.w	r3, r3, r4, asr #31
 8006b20: f851 4c38    	ldr	r4, [r1, #-56]
 8006b24: 441a         	add	r2, r3
 8006b26: ea4f 0345    	lsl.w	r3, r5, #0x1
 8006b2a: ea83 73e5    	eor.w	r3, r3, r5, asr #31
 8006b2e: f851 5c34    	ldr	r5, [r1, #-52]
 8006b32: 441a         	add	r2, r3
 8006b34: ea4f 0344    	lsl.w	r3, r4, #0x1
 8006b38: ea83 73e4    	eor.w	r3, r3, r4, asr #31
 8006b3c: f851 4c30    	ldr	r4, [r1, #-48]
 8006b40: 441a         	add	r2, r3
 8006b42: ea4f 0345    	lsl.w	r3, r5, #0x1
 8006b46: ea83 73e5    	eor.w	r3, r3, r5, asr #31
 8006b4a: f851 5c2c    	ldr	r5, [r1, #-44]
 8006b4e: 441a         	add	r2, r3
 8006b50: ea4f 0344    	lsl.w	r3, r4, #0x1
 8006b54: ea83 73e4    	eor.w	r3, r3, r4, asr #31
 8006b58: f851 4c28    	ldr	r4, [r1, #-40]
 8006b5c: 441a         	add	r2, r3
 8006b5e: ea4f 0345    	lsl.w	r3, r5, #0x1
 8006b62: ea83 73e5    	eor.w	r3, r3, r5, asr #31
 8006b66: f851 5c24    	ldr	r5, [r1, #-36]
 8006b6a: 441a         	add	r2, r3
 8006b6c: ea4f 0344    	lsl.w	r3, r4, #0x1
 8006b70: ea83 73e4    	eor.w	r3, r3, r4, asr #31
 8006b74: f851 4c20    	ldr	r4, [r1, #-32]
 8006b78: 441a         	add	r2, r3
 8006b7a: ea4f 0345    	lsl.w	r3, r5, #0x1
 8006b7e: ea83 73e5    	eor.w	r3, r3, r5, asr #31
 8006b82: f851 5c1c    	ldr	r5, [r1, #-28]
 8006b86: 441a         	add	r2, r3
 8006b88: ea4f 0344    	lsl.w	r3, r4, #0x1
 8006b8c: ea83 73e4    	eor.w	r3, r3, r4, asr #31
 8006b90: f851 4c18    	ldr	r4, [r1, #-24]
 8006b94: 441a         	add	r2, r3
 8006b96: ea4f 0345    	lsl.w	r3, r5, #0x1
 8006b9a: ea83 73e5    	eor.w	r3, r3, r5, asr #31
 8006b9e: f851 5c14    	ldr	r5, [r1, #-20]
 8006ba2: 441a         	add	r2, r3
 8006ba4: ea4f 0344    	lsl.w	r3, r4, #0x1
 8006ba8: ea83 73e4    	eor.w	r3, r3, r4, asr #31
 8006bac: f851 4c10    	ldr	r4, [r1, #-16]
 8006bb0: 441a         	add	r2, r3
 8006bb2: ea4f 0345    	lsl.w	r3, r5, #0x1
 8006bb6: ea83 73e5    	eor.w	r3, r3, r5, asr #31
 8006bba: f851 8c04    	ldr	r8, [r1, #-4]
 8006bbe: 441a         	add	r2, r3
 8006bc0: ea4f 0344    	lsl.w	r3, r4, #0x1
 8006bc4: ea83 73e4    	eor.w	r3, r3, r4, asr #31
 8006bc8: 441a         	add	r2, r3
 8006bca: ea4f 0346    	lsl.w	r3, r6, #0x1
 8006bce: ea83 73e6    	eor.w	r3, r3, r6, asr #31
 8006bd2: 441a         	add	r2, r3
 8006bd4: ea4f 0349    	lsl.w	r3, r9, #0x1
 8006bd8: ea83 73e9    	eor.w	r3, r3, r9, asr #31
 8006bdc: 441a         	add	r2, r3
 8006bde: ea4f 0348    	lsl.w	r3, r8, #0x1
 8006be2: ea83 73e8    	eor.w	r3, r3, r8, asr #31
 8006be6: 441a         	add	r2, r3
 8006be8: ea4f 034c    	lsl.w	r3, r12, #0x1
 8006bec: ea83 73ec    	eor.w	r3, r3, r12, asr #31
 8006bf0: eb03 0802    	add.w	r8, r3, r2
 8006bf4: f47f af17    	bne.w	0x8006a26 <raylar_crcl_encoder_service::benchmark::fold_sum::h16b920496cf2076e+0x18> @ imm = #-0x1d2
 8006bf8: f8cd 8008    	str.w	r8, [sp, #0x8]
 8006bfc: a902         	add	r1, sp, #0x8
 8006bfe: f04f 0900    	mov.w	r9, #0x0
 8006c02: f240 38ff    	movw	r8, #0x3ff
 8006c06: 9902         	ldr	r1, [sp, #0x8]
 8006c08: 9101         	str	r1, [sp, #0x4]
 8006c0a: f500 3190    	add.w	r1, r0, #0x12000
 8006c0e: f501 5181    	add.w	r1, r1, #0x1020
 8006c12: f851 ef84    	ldr	lr, [r1, #132]!
 8006c16: f1b8 0821    	subs.w	r8, r8, #0x21
 8006c1a: e951 5420    	ldrd	r5, r4, [r1, #-128]
 8006c1e: ea4f 0644    	lsl.w	r6, r4, #0x1
 8006c22: e951 321e    	ldrd	r3, r2, [r1, #-120]
 8006c26: ea86 76e4    	eor.w	r6, r6, r4, asr #31
 8006c2a: f851 ac08    	ldr	r10, [r1, #-8]
 8006c2e: ea4f 0445    	lsl.w	r4, r5, #0x1
 8006c32: ea84 74e5    	eor.w	r4, r4, r5, asr #31
 8006c36: eb04 0509    	add.w	r5, r4, r9
 8006c3a: ea4f 0443    	lsl.w	r4, r3, #0x1
 8006c3e: ea84 73e3    	eor.w	r3, r4, r3, asr #31
 8006c42: f851 4c70    	ldr	r4, [r1, #-112]
 8006c46: 442e         	add	r6, r5
 8006c48: ea4f 0542    	lsl.w	r5, r2, #0x1
 8006c4c: 4433         	add	r3, r6
 8006c4e: ea85 72e2    	eor.w	r2, r5, r2, asr #31
 8006c52: f851 5c6c    	ldr	r5, [r1, #-108]
 8006c56: 441a         	add	r2, r3
 8006c58: ea4f 0344    	lsl.w	r3, r4, #0x1
 8006c5c: f851 6c0c    	ldr	r6, [r1, #-12]
 8006c60: ea83 73e4    	eor.w	r3, r3, r4, asr #31
 8006c64: f851 4c68    	ldr	r4, [r1, #-104]
 8006c68: 441a         	add	r2, r3
 8006c6a: ea4f 0345    	lsl.w	r3, r5, #0x1
 8006c6e: ea83 73e5    	eor.w	r3, r3, r5, asr #31
 8006c72: f851 5c64    	ldr	r5, [r1, #-100]
 8006c76: 441a         	add	r2, r3
 8006c78: ea4f 0344    	lsl.w	r3, r4, #0x1
 8006c7c: ea83 73e4    	eor.w	r3, r3, r4, asr #31
 8006c80: f851 4c60    	ldr	r4, [r1, #-96]
 8006c84: 441a         	add	r2, r3
 8006c86: ea4f 0345    	lsl.w	r3, r5, #0x1
 8006c8a: ea83 73e5    	eor.w	r3, r3, r5, asr #31
 8006c8e: f851 5c5c    	ldr	r5, [r1, #-92]
 8006c92: 441a         	add	r2, r3
 8006c94: ea4f 0344    	lsl.w	r3, r4, #0x1
 8006c98: ea83 73e4    	eor.w	r3, r3, r4, asr #31
 8006c9c: f851 4c58    	ldr	r4, [r1, #-88]
 8006ca0: 441a         	add	r2, r3
 8006ca2: ea4f 0345    	lsl.w	r3, r5, #0x1
 8006ca6: ea83 73e5    	eor.w	r3, r3, r5, asr #31
 8006caa: f851 5c54    	ldr	r5, [r1, #-84]
 8006cae: 441a         	add	r2, r3
 8006cb0: ea4f 0344    	lsl.w	r3, r4, #0x1
 8006cb4: ea83 73e4    	eor.w	r3, r3, r4, asr #31
 8006cb8: f851 4c50    	ldr	r4, [r1, #-80]
 8006cbc: 441a         	add	r2, r3
 8006cbe: ea4f 0345    	lsl.w	r3, r5, #0x1
 8006cc2: ea83 73e5    	eor.w	r3, r3, r5, asr #31
 8006cc6: f851 5c4c    	ldr	r5, [r1, #-76]
 8006cca: 441a         	add	r2, r3
 8006ccc: ea4f 0344    	lsl.w	r3, r4, #0x1
 8006cd0: ea83 73e4    	eor.w	r3, r3, r4, asr #31
 8006cd4: f851 4c48    	ldr	r4, [r1, #-72]
 8006cd8: 441a         	add	r2, r3
 8006cda: ea4f 0345    	lsl.w	r3, r5, #0x1
 8006cde: ea83 73e5    	eor.w	r3, r3, r5, asr #31
 8006ce2: f851 5c44    	ldr	r5, [r1, #-68]
 8006ce6: 441a         	add	r2, r3
 8006ce8: ea4f 0344    	lsl.w	r3, r4, #0x1
 8006cec: ea83 73e4    	eor.w	r3, r3, r4, asr #31
 8006cf0: f851 4c40    	ldr	r4, [r1, #-64]
 8006cf4: 441a         	add	r2, r3
 8006cf6: ea4f 0345    	lsl.w	r3, r5, #0x1
 8006cfa: ea83 73e5    	eor.w	r3, r3, r5, asr #31
 8006cfe: f851 5c3c    	ldr	r5, [r1, #-60]
 8006d02: 441a         	add	r2, r3
 8006d04: ea4f 0344    	lsl.w	r3, r4, #0x1
 8006d08: ea83 73e4    	eor.w	r3, r3, r4, asr #31
 8006d0c: f851 4c38    	ldr	r4, [r1, #-56]
 8006d10: 441a         	add	r2, r3
 8006d12: ea4f 0345    	lsl.w	r3, r5, #0x1
 8006d16: ea83 73e5    	eor.w	r3, r3, r5, asr #31
 8006d1a: f851 5c34    	ldr	r5, [r1, #-52]
 8006d1e: 441a         	add	r2, r3
 8006d20: ea4f 0344    	lsl.w	r3, r4, #0x1
 8006d24: ea83 73e4    	eor.w	r3, r3, r4, asr #31
 8006d28: f851 4c30    	ldr	r4, [r1, #-48]
 8006d2c: 441a         	add	r2, r3
 8006d2e: ea4f 0345    	lsl.w	r3, r5, #0x1
 8006d32: ea83 73e5    	eor.w	r3, r3, r5, asr #31
 8006d36: f851 5c2c    	ldr	r5, [r1, #-44]
 8006d3a: 441a         	add	r2, r3
 8006d3c: ea4f 0344    	lsl.w	r3, r4, #0x1
 8006d40: ea83 73e4    	eor.w	r3, r3, r4, asr #31
 8006d44: f851 4c28    	ldr	r4, [r1, #-40]
 8006d48: 441a         	add	r2, r3
 8006d4a: ea4f 0345    	lsl.w	r3, r5, #0x1
 8006d4e: ea83 73e5    	eor.w	r3, r3, r5, asr #31
 8006d52: f851 5c24    	ldr	r5, [r1, #-36]
 8006d56: 441a         	add	r2, r3
 8006d58: ea4f 0344    	lsl.w	r3, r4, #0x1
 8006d5c: ea83 73e4    	eor.w	r3, r3, r4, asr #31
 8006d60: f851 4c20    	ldr	r4, [r1, #-32]
 8006d64: 441a         	add	r2, r3
 8006d66: ea4f 0345    	lsl.w	r3, r5, #0x1
 8006d6a: ea83 73e5    	eor.w	r3, r3, r5, asr #31
 8006d6e: f851 5c1c    	ldr	r5, [r1, #-28]
 8006d72: 441a         	add	r2, r3
 8006d74: ea4f 0344    	lsl.w	r3, r4, #0x1
 8006d78: ea83 73e4    	eor.w	r3, r3, r4, asr #31
 8006d7c: f851 4c18    	ldr	r4, [r1, #-24]
 8006d80: 441a         	add	r2, r3
 8006d82: ea4f 0345    	lsl.w	r3, r5, #0x1
 8006d86: ea83 73e5    	eor.w	r3, r3, r5, asr #31
 8006d8a: f851 5c14    	ldr	r5, [r1, #-20]
 8006d8e: 441a         	add	r2, r3
 8006d90: ea4f 0344    	lsl.w	r3, r4, #0x1
 8006d94: ea83 73e4    	eor.w	r3, r3, r4, asr #31
 8006d98: f851 4c10    	ldr	r4, [r1, #-16]
 8006d9c: 441a         	add	r2, r3
 8006d9e: ea4f 0345    	lsl.w	r3, r5, #0x1
 8006da2: ea83 73e5    	eor.w	r3, r3, r5, asr #31
 8006da6: f851 9c04    	ldr	r9, [r1, #-4]
 8006daa: 441a         	add	r2, r3
 8006dac: ea4f 0344    	lsl.w	r3, r4, #0x1
 8006db0: ea83 73e4    	eor.w	r3, r3, r4, asr #31
 8006db4: 441a         	add	r2, r3
 8006db6: ea4f 0346    	lsl.w	r3, r6, #0x1
 8006dba: ea83 73e6    	eor.w	r3, r3, r6, asr #31
 8006dbe: 441a         	add	r2, r3
 8006dc0: ea4f 034a    	lsl.w	r3, r10, #0x1
 8006dc4: ea83 73ea    	eor.w	r3, r3, r10, asr #31
 8006dc8: 441a         	add	r2, r3
 8006dca: ea4f 0349    	lsl.w	r3, r9, #0x1
 8006dce: ea83 73e9    	eor.w	r3, r3, r9, asr #31
 8006dd2: 441a         	add	r2, r3
 8006dd4: ea4f 034e    	lsl.w	r3, lr, #0x1
 8006dd8: ea83 73ee    	eor.w	r3, r3, lr, asr #31
 8006ddc: eb03 0902    	add.w	r9, r3, r2
 8006de0: f47f af17    	bne.w	0x8006c12 <raylar_crcl_encoder_service::benchmark::fold_sum::h16b920496cf2076e+0x204> @ imm = #-0x1d2
 8006de4: f8cd 9008    	str.w	r9, [sp, #0x8]
 8006de8: a902         	add	r1, sp, #0x8
 8006dea: f04f 0a00    	mov.w	r10, #0x0
 8006dee: f240 39ff    	movw	r9, #0x3ff
 8006df2: 9902         	ldr	r1, [sp, #0x8]
 8006df4: 9100         	str	r1, [sp]
 8006df6: f500 31a0    	add.w	r1, r0, #0x14000
 8006dfa: 311c         	adds	r1, #0x1c
 8006dfc: f851 8f84    	ldr	r8, [r1, #132]!
 8006e00: f1b9 0921    	subs.w	r9, r9, #0x21
 8006e04: e951 5620    	ldrd	r5, r6, [r1, #-128]
 8006e08: ea4f 0446    	lsl.w	r4, r6, #0x1
 8006e0c: e951 231e    	ldrd	r2, r3, [r1, #-120]
 8006e10: ea84 76e6    	eor.w	r6, r4, r6, asr #31
 8006e14: f851 bc08    	ldr	r11, [r1, #-8]
 8006e18: ea4f 0445    	lsl.w	r4, r5, #0x1
 8006e1c: ea84 74e5    	eor.w	r4, r4, r5, asr #31
 8006e20: eb04 050a    	add.w	r5, r4, r10
 8006e24: ea4f 0442    	lsl.w	r4, r2, #0x1
 8006e28: ea84 72e2    	eor.w	r2, r4, r2, asr #31
 8006e2c: f851 4c70    	ldr	r4, [r1, #-112]
 8006e30: 442e         	add	r6, r5
 8006e32: ea4f 0543    	lsl.w	r5, r3, #0x1
 8006e36: 4432         	add	r2, r6
 8006e38: ea85 73e3    	eor.w	r3, r5, r3, asr #31
 8006e3c: f851 5c6c    	ldr	r5, [r1, #-108]
 8006e40: 441a         	add	r2, r3
 8006e42: ea4f 0344    	lsl.w	r3, r4, #0x1
 8006e46: f851 6c0c    	ldr	r6, [r1, #-12]
 8006e4a: ea83 73e4    	eor.w	r3, r3, r4, asr #31
 8006e4e: f851 4c68    	ldr	r4, [r1, #-104]
 8006e52: 441a         	add	r2, r3
 8006e54: ea4f 0345    	lsl.w	r3, r5, #0x1
 8006e58: ea83 73e5    	eor.w	r3, r3, r5, asr #31
 8006e5c: f851 5c64    	ldr	r5, [r1, #-100]
 8006e60: 441a         	add	r2, r3
 8006e62: ea4f 0344    	lsl.w	r3, r4, #0x1
 8006e66: ea83 73e4    	eor.w	r3, r3, r4, asr #31
 8006e6a: f851 4c60    	ldr	r4, [r1, #-96]
 8006e6e: 441a         	add	r2, r3
 8006e70: ea4f 0345    	lsl.w	r3, r5, #0x1
 8006e74: ea83 73e5    	eor.w	r3, r3, r5, asr #31
 8006e78: f851 5c5c    	ldr	r5, [r1, #-92]
 8006e7c: 441a         	add	r2, r3
 8006e7e: ea4f 0344    	lsl.w	r3, r4, #0x1
 8006e82: ea83 73e4    	eor.w	r3, r3, r4, asr #31
 8006e86: f851 4c58    	ldr	r4, [r1, #-88]
 8006e8a: 441a         	add	r2, r3
 8006e8c: ea4f 0345    	lsl.w	r3, r5, #0x1
 8006e90: ea83 73e5    	eor.w	r3, r3, r5, asr #31
 8006e94: f851 5c54    	ldr	r5, [r1, #-84]
 8006e98: 441a         	add	r2, r3
 8006e9a: ea4f 0344    	lsl.w	r3, r4, #0x1
 8006e9e: ea83 73e4    	eor.w	r3, r3, r4, asr #31
 8006ea2: f851 4c50    	ldr	r4, [r1, #-80]
 8006ea6: 441a         	add	r2, r3
 8006ea8: ea4f 0345    	lsl.w	r3, r5, #0x1
 8006eac: ea83 73e5    	eor.w	r3, r3, r5, asr #31
 8006eb0: f851 5c4c    	ldr	r5, [r1, #-76]
 8006eb4: 441a         	add	r2, r3
 8006eb6: ea4f 0344    	lsl.w	r3, r4, #0x1
 8006eba: ea83 73e4    	eor.w	r3, r3, r4, asr #31
 8006ebe: f851 4c48    	ldr	r4, [r1, #-72]
 8006ec2: 441a         	add	r2, r3
 8006ec4: ea4f 0345    	lsl.w	r3, r5, #0x1
 8006ec8: ea83 73e5    	eor.w	r3, r3, r5, asr #31
 8006ecc: f851 5c44    	ldr	r5, [r1, #-68]
 8006ed0: 441a         	add	r2, r3
 8006ed2: ea4f 0344    	lsl.w	r3, r4, #0x1
 8006ed6: ea83 73e4    	eor.w	r3, r3, r4, asr #31
 8006eda: f851 4c40    	ldr	r4, [r1, #-64]
 8006ede: 441a         	add	r2, r3
 8006ee0: ea4f 0345    	lsl.w	r3, r5, #0x1
 8006ee4: ea83 73e5    	eor.w	r3, r3, r5, asr #31
 8006ee8: f851 5c3c    	ldr	r5, [r1, #-60]
 8006eec: 441a         	add	r2, r3
 8006eee: ea4f 0344    	lsl.w	r3, r4, #0x1
 8006ef2: ea83 73e4    	eor.w	r3, r3, r4, asr #31
 8006ef6: f851 4c38    	ldr	r4, [r1, #-56]
 8006efa: 441a         	add	r2, r3
 8006efc: ea4f 0345    	lsl.w	r3, r5, #0x1
 8006f00: ea83 73e5    	eor.w	r3, r3, r5, asr #31
 8006f04: f851 5c34    	ldr	r5, [r1, #-52]
 8006f08: 441a         	add	r2, r3
 8006f0a: ea4f 0344    	lsl.w	r3, r4, #0x1
 8006f0e: ea83 73e4    	eor.w	r3, r3, r4, asr #31
 8006f12: f851 4c30    	ldr	r4, [r1, #-48]
 8006f16: 441a         	add	r2, r3
 8006f18: ea4f 0345    	lsl.w	r3, r5, #0x1
 8006f1c: ea83 73e5    	eor.w	r3, r3, r5, asr #31
 8006f20: f851 5c2c    	ldr	r5, [r1, #-44]
 8006f24: 441a         	add	r2, r3
 8006f26: ea4f 0344    	lsl.w	r3, r4, #0x1
 8006f2a: ea83 73e4    	eor.w	r3, r3, r4, asr #31
 8006f2e: f851 4c28    	ldr	r4, [r1, #-40]
 8006f32: 441a         	add	r2, r3
 8006f34: ea4f 0345    	lsl.w	r3, r5, #0x1
 8006f38: ea83 73e5    	eor.w	r3, r3, r5, asr #31
 8006f3c: f851 5c24    	ldr	r5, [r1, #-36]
 8006f40: 441a         	add	r2, r3
 8006f42: ea4f 0344    	lsl.w	r3, r4, #0x1
 8006f46: ea83 73e4    	eor.w	r3, r3, r4, asr #31
 8006f4a: f851 4c20    	ldr	r4, [r1, #-32]
 8006f4e: 441a         	add	r2, r3
 8006f50: ea4f 0345    	lsl.w	r3, r5, #0x1
 8006f54: ea83 73e5    	eor.w	r3, r3, r5, asr #31
 8006f58: f851 5c1c    	ldr	r5, [r1, #-28]
 8006f5c: 441a         	add	r2, r3
 8006f5e: ea4f 0344    	lsl.w	r3, r4, #0x1
 8006f62: ea83 73e4    	eor.w	r3, r3, r4, asr #31
 8006f66: f851 4c18    	ldr	r4, [r1, #-24]
 8006f6a: 441a         	add	r2, r3
 8006f6c: ea4f 0345    	lsl.w	r3, r5, #0x1
 8006f70: ea83 73e5    	eor.w	r3, r3, r5, asr #31
 8006f74: f851 5c14    	ldr	r5, [r1, #-20]
 8006f78: 441a         	add	r2, r3
 8006f7a: ea4f 0344    	lsl.w	r3, r4, #0x1
 8006f7e: ea83 73e4    	eor.w	r3, r3, r4, asr #31
 8006f82: f851 4c10    	ldr	r4, [r1, #-16]
 8006f86: 441a         	add	r2, r3
 8006f88: ea4f 0345    	lsl.w	r3, r5, #0x1
 8006f8c: ea83 73e5    	eor.w	r3, r3, r5, asr #31
 8006f90: f851 ac04    	ldr	r10, [r1, #-4]
 8006f94: 441a         	add	r2, r3
 8006f96: ea4f 0344    	lsl.w	r3, r4, #0x1
 8006f9a: ea83 73e4    	eor.w	r3, r3, r4, asr #31
 8006f9e: 441a         	add	r2, r3
 8006fa0: ea4f 0346    	lsl.w	r3, r6, #0x1
 8006fa4: ea83 73e6    	eor.w	r3, r3, r6, asr #31
 8006fa8: 441a         	add	r2, r3
 8006faa: ea4f 034b    	lsl.w	r3, r11, #0x1
 8006fae: ea83 73eb    	eor.w	r3, r3, r11, asr #31
 8006fb2: 441a         	add	r2, r3
 8006fb4: ea4f 034a    	lsl.w	r3, r10, #0x1
 8006fb8: ea83 73ea    	eor.w	r3, r3, r10, asr #31
 8006fbc: 441a         	add	r2, r3
 8006fbe: ea4f 0348    	lsl.w	r3, r8, #0x1
 8006fc2: ea83 73e8    	eor.w	r3, r3, r8, asr #31
 8006fc6: eb03 0a02    	add.w	r10, r3, r2
 8006fca: f47f af17    	bne.w	0x8006dfc <raylar_crcl_encoder_service::benchmark::fold_sum::h16b920496cf2076e+0x3ee> @ imm = #-0x1d2
 8006fce: f8cd a008    	str.w	r10, [sp, #0x8]
 8006fd2: a902         	add	r1, sp, #0x8
 8006fd4: f500 31a8    	add.w	r1, r0, #0x15000
 8006fd8: f04f 0b00    	mov.w	r11, #0x0
 8006fdc: f8dd 8008    	ldr.w	r8, [sp, #0x8]
 8006fe0: 3118         	adds	r1, #0x18
 8006fe2: f240 3aff    	movw	r10, #0x3ff
 8006fe6: f851 9f84    	ldr	r9, [r1, #132]!
 8006fea: f1ba 0a21    	subs.w	r10, r10, #0x21
 8006fee: e951 5620    	ldrd	r5, r6, [r1, #-128]
 8006ff2: e951 421e    	ldrd	r4, r2, [r1, #-120]
 8006ff6: ea4f 0346    	lsl.w	r3, r6, #0x1
 8006ffa: ea83 73e6    	eor.w	r3, r3, r6, asr #31
 8006ffe: ea4f 0c42    	lsl.w	r12, r2, #0x1
 8007002: ea4f 0645    	lsl.w	r6, r5, #0x1
 8007006: ea86 75e5    	eor.w	r5, r6, r5, asr #31
 800700a: eb05 060b    	add.w	r6, r5, r11
 800700e: ea4f 0544    	lsl.w	r5, r4, #0x1
 8007012: ea85 74e4    	eor.w	r4, r5, r4, asr #31
 8007016: f851 5c70    	ldr	r5, [r1, #-112]
 800701a: 4433         	add	r3, r6
 800701c: ea8c 7ce2    	eor.w	r12, r12, r2, asr #31
 8007020: f851 2c6c    	ldr	r2, [r1, #-108]
 8007024: 441c         	add	r4, r3
 8007026: ea4f 0645    	lsl.w	r6, r5, #0x1
 800702a: 4464         	add	r4, r12
 800702c: ea86 76e5    	eor.w	r6, r6, r5, asr #31
 8007030: f851 5c68    	ldr	r5, [r1, #-104]
 8007034: 4426         	add	r6, r4
 8007036: ea4f 0442    	lsl.w	r4, r2, #0x1
 800703a: ea84 72e2    	eor.w	r2, r4, r2, asr #31
 800703e: f851 4c64    	ldr	r4, [r1, #-100]
 8007042: 4432         	add	r2, r6
 8007044: ea4f 0645    	lsl.w	r6, r5, #0x1
 8007048: ea86 76e5    	eor.w	r6, r6, r5, asr #31
 800704c: f851 5c60    	ldr	r5, [r1, #-96]
 8007050: 4432         	add	r2, r6
 8007052: ea4f 0644    	lsl.w	r6, r4, #0x1
 8007056: ea86 76e4    	eor.w	r6, r6, r4, asr #31
 800705a: f851 4c5c    	ldr	r4, [r1, #-92]
 800705e: 4432         	add	r2, r6
 8007060: ea4f 0645    	lsl.w	r6, r5, #0x1
 8007064: ea86 76e5    	eor.w	r6, r6, r5, asr #31
 8007068: f851 5c58    	ldr	r5, [r1, #-88]
 800706c: 4432         	add	r2, r6
 800706e: ea4f 0644    	lsl.w	r6, r4, #0x1
 8007072: ea86 76e4    	eor.w	r6, r6, r4, asr #31
 8007076: f851 4c54    	ldr	r4, [r1, #-84]
 800707a: 4432         	add	r2, r6
 800707c: ea4f 0645    	lsl.w	r6, r5, #0x1
 8007080: ea86 76e5    	eor.w	r6, r6, r5, asr #31
 8007084: f851 5c50    	ldr	r5, [r1, #-80]
 8007088: 4432         	add	r2, r6
 800708a: ea4f 0644    	lsl.w	r6, r4, #0x1
 800708e: ea86 76e4    	eor.w	r6, r6, r4, asr #31
 8007092: f851 4c4c    	ldr	r4, [r1, #-76]
 8007096: 4432         	add	r2, r6
 8007098: ea4f 0645    	lsl.w	r6, r5, #0x1
 800709c: ea86 76e5    	eor.w	r6, r6, r5, asr #31
 80070a0: f851 5c48    	ldr	r5, [r1, #-72]
 80070a4: 4432         	add	r2, r6
 80070a6: ea4f 0644    	lsl.w	r6, r4, #0x1
 80070aa: ea86 76e4    	eor.w	r6, r6, r4, asr #31
 80070ae: f851 4c44    	ldr	r4, [r1, #-68]
 80070b2: 4432         	add	r2, r6
 80070b4: ea4f 0645    	lsl.w	r6, r5, #0x1
 80070b8: ea86 76e5    	eor.w	r6, r6, r5, asr #31
 80070bc: f851 5c40    	ldr	r5, [r1, #-64]
 80070c0: 4432         	add	r2, r6
 80070c2: ea4f 0644    	lsl.w	r6, r4, #0x1
 80070c6: ea86 76e4    	eor.w	r6, r6, r4, asr #31
 80070ca: f851 4c3c    	ldr	r4, [r1, #-60]
 80070ce: 4432         	add	r2, r6
 80070d0: ea4f 0645    	lsl.w	r6, r5, #0x1
 80070d4: ea86 76e5    	eor.w	r6, r6, r5, asr #31
 80070d8: f851 5c38    	ldr	r5, [r1, #-56]
 80070dc: 4432         	add	r2, r6
 80070de: ea4f 0644    	lsl.w	r6, r4, #0x1
 80070e2: ea86 76e4    	eor.w	r6, r6, r4, asr #31
 80070e6: f851 4c34    	ldr	r4, [r1, #-52]
 80070ea: 4432         	add	r2, r6
 80070ec: ea4f 0645    	lsl.w	r6, r5, #0x1
 80070f0: ea86 76e5    	eor.w	r6, r6, r5, asr #31
 80070f4: f851 5c30    	ldr	r5, [r1, #-48]
 80070f8: 4432         	add	r2, r6
 80070fa: ea4f 0644    	lsl.w	r6, r4, #0x1
 80070fe: ea86 76e4    	eor.w	r6, r6, r4, asr #31
 8007102: f851 4c2c    	ldr	r4, [r1, #-44]
 8007106: 4432         	add	r2, r6
 8007108: ea4f 0645    	lsl.w	r6, r5, #0x1
 800710c: ea86 76e5    	eor.w	r6, r6, r5, asr #31
 8007110: f851 5c28    	ldr	r5, [r1, #-40]
 8007114: 4432         	add	r2, r6
 8007116: ea4f 0644    	lsl.w	r6, r4, #0x1
 800711a: ea86 76e4    	eor.w	r6, r6, r4, asr #31
 800711e: f851 4c24    	ldr	r4, [r1, #-36]
 8007122: 4432         	add	r2, r6
 8007124: ea4f 0645    	lsl.w	r6, r5, #0x1
 8007128: ea86 76e5    	eor.w	r6, r6, r5, asr #31
 800712c: f851 5c20    	ldr	r5, [r1, #-32]
 8007130: 4432         	add	r2, r6
 8007132: ea4f 0644    	lsl.w	r6, r4, #0x1
 8007136: ea86 76e4    	eor.w	r6, r6, r4, asr #31
 800713a: f851 4c1c    	ldr	r4, [r1, #-28]
 800713e: 4432         	add	r2, r6
 8007140: ea4f 0645    	lsl.w	r6, r5, #0x1
 8007144: ea86 76e5    	eor.w	r6, r6, r5, asr #31
 8007148: f851 5c18    	ldr	r5, [r1, #-24]
 800714c: 4432         	add	r2, r6
 800714e: ea4f 0644    	lsl.w	r6, r4, #0x1
 8007152: ea86 76e4    	eor.w	r6, r6, r4, asr #31
 8007156: f851 4c14    	ldr	r4, [r1, #-20]
 800715a: 4432         	add	r2, r6
 800715c: ea4f 0645    	lsl.w	r6, r5, #0x1
 8007160: ea86 76e5    	eor.w	r6, r6, r5, asr #31
 8007164: f851 5c10    	ldr	r5, [r1, #-16]
 8007168: 4432         	add	r2, r6
 800716a: ea4f 0644    	lsl.w	r6, r4, #0x1
 800716e: ea86 76e4    	eor.w	r6, r6, r4, asr #31
 8007172: f851 3c0c    	ldr	r3, [r1, #-12]
 8007176: 4432         	add	r2, r6
 8007178: ea4f 0645    	lsl.w	r6, r5, #0x1
 800717c: f851 ec08    	ldr	lr, [r1, #-8]
 8007180: ea86 76e5    	eor.w	r6, r6, r5, asr #31
 8007184: 4432         	add	r2, r6
 8007186: ea4f 0643    	lsl.w	r6, r3, #0x1
 800718a: ea86 73e3    	eor.w	r3, r6, r3, asr #31
 800718e: f851 bc04    	ldr	r11, [r1, #-4]
 8007192: 441a         	add	r2, r3
 8007194: ea4f 034e    	lsl.w	r3, lr, #0x1
 8007198: ea83 73ee    	eor.w	r3, r3, lr, asr #31
 800719c: 441a         	add	r2, r3
 800719e: ea4f 034b    	lsl.w	r3, r11, #0x1
 80071a2: ea83 73eb    	eor.w	r3, r3, r11, asr #31
 80071a6: 441a         	add	r2, r3
 80071a8: ea4f 0349    	lsl.w	r3, r9, #0x1
 80071ac: ea83 73e9    	eor.w	r3, r3, r9, asr #31
 80071b0: eb03 0b02    	add.w	r11, r3, r2
 80071b4: f47f af17    	bne.w	0x8006fe6 <raylar_crcl_encoder_service::benchmark::fold_sum::h16b920496cf2076e+0x5d8> @ imm = #-0x1d2
 80071b8: f8cd b008    	str.w	r11, [sp, #0x8]
 80071bc: a902         	add	r1, sp, #0x8
 80071be: f500 30b0    	add.w	r0, r0, #0x16000
 80071c2: 2300         	movs	r3, #0x0
 80071c4: f8dd c008    	ldr.w	r12, [sp, #0x8]
 80071c8: 3014         	adds	r0, #0x14
 80071ca: f240 39ff    	movw	r9, #0x3ff
 80071ce: f850 ef84    	ldr	lr, [r0, #132]!
 80071d2: f1b9 0921    	subs.w	r9, r9, #0x21
 80071d6: e950 6520    	ldrd	r6, r5, [r0, #-128]
 80071da: ea4f 0245    	lsl.w	r2, r5, #0x1
 80071de: e950 141e    	ldrd	r1, r4, [r0, #-120]
 80071e2: ea82 72e5    	eor.w	r2, r2, r5, asr #31
 80071e6: f850 ac04    	ldr	r10, [r0, #-4]
 80071ea: ea4f 0546    	lsl.w	r5, r6, #0x1
 80071ee: ea85 75e6    	eor.w	r5, r5, r6, asr #31
 80071f2: f850 6c08    	ldr	r6, [r0, #-8]
 80071f6: 442b         	add	r3, r5
 80071f8: f850 5c70    	ldr	r5, [r0, #-112]
 80071fc: 441a         	add	r2, r3
 80071fe: ea4f 0341    	lsl.w	r3, r1, #0x1
 8007202: ea83 71e1    	eor.w	r1, r3, r1, asr #31
 8007206: f850 3c0c    	ldr	r3, [r0, #-12]
 800720a: 4411         	add	r1, r2
 800720c: ea4f 0244    	lsl.w	r2, r4, #0x1
 8007210: ea82 72e4    	eor.w	r2, r2, r4, asr #31
 8007214: f850 4c6c    	ldr	r4, [r0, #-108]
 8007218: 4411         	add	r1, r2
 800721a: ea4f 0245    	lsl.w	r2, r5, #0x1
 800721e: ea82 72e5    	eor.w	r2, r2, r5, asr #31
 8007222: f850 5c68    	ldr	r5, [r0, #-104]
 8007226: 4411         	add	r1, r2
 8007228: ea4f 0244    	lsl.w	r2, r4, #0x1
 800722c: ea82 72e4    	eor.w	r2, r2, r4, asr #31
 8007230: f850 4c64    	ldr	r4, [r0, #-100]
 8007234: 4411         	add	r1, r2
 8007236: ea4f 0245    	lsl.w	r2, r5, #0x1
 800723a: ea82 72e5    	eor.w	r2, r2, r5, asr #31
 800723e: f850 5c60    	ldr	r5, [r0, #-96]
 8007242: 4411         	add	r1, r2
 8007244: ea4f 0244    	lsl.w	r2, r4, #0x1
 8007248: ea82 72e4    	eor.w	r2, r2, r4, asr #31
 800724c: f850 4c5c    	ldr	r4, [r0, #-92]
 8007250: 4411         	add	r1, r2
 8007252: ea4f 0245    	lsl.w	r2, r5, #0x1
 8007256: ea82 72e5    	eor.w	r2, r2, r5, asr #31
 800725a: f850 5c58    	ldr	r5, [r0, #-88]
 800725e: 4411         	add	r1, r2
 8007260: ea4f 0244    	lsl.w	r2, r4, #0x1
 8007264: ea82 72e4    	eor.w	r2, r2, r4, asr #31
 8007268: f850 4c54    	ldr	r4, [r0, #-84]
 800726c: 4411         	add	r1, r2
 800726e: ea4f 0245    	lsl.w	r2, r5, #0x1
 8007272: ea82 72e5    	eor.w	r2, r2, r5, asr #31
 8007276: f850 5c50    	ldr	r5, [r0, #-80]
 800727a: 4411         	add	r1, r2
 800727c: ea4f 0244    	lsl.w	r2, r4, #0x1
 8007280: ea82 72e4    	eor.w	r2, r2, r4, asr #31
 8007284: f850 4c4c    	ldr	r4, [r0, #-76]
 8007288: 4411         	add	r1, r2
 800728a: ea4f 0245    	lsl.w	r2, r5, #0x1
 800728e: ea82 72e5    	eor.w	r2, r2, r5, asr #31
 8007292: f850 5c48    	ldr	r5, [r0, #-72]
 8007296: 4411         	add	r1, r2
 8007298: ea4f 0244    	lsl.w	r2, r4, #0x1
 800729c: ea82 72e4    	eor.w	r2, r2, r4, asr #31
 80072a0: f850 4c44    	ldr	r4, [r0, #-68]
 80072a4: 4411         	add	r1, r2
 80072a6: ea4f 0245    	lsl.w	r2, r5, #0x1
 80072aa: ea82 72e5    	eor.w	r2, r2, r5, asr #31
 80072ae: f850 5c40    	ldr	r5, [r0, #-64]
 80072b2: 4411         	add	r1, r2
 80072b4: ea4f 0244    	lsl.w	r2, r4, #0x1
 80072b8: ea82 72e4    	eor.w	r2, r2, r4, asr #31
 80072bc: f850 4c3c    	ldr	r4, [r0, #-60]
 80072c0: 4411         	add	r1, r2
 80072c2: ea4f 0245    	lsl.w	r2, r5, #0x1
 80072c6: ea82 72e5    	eor.w	r2, r2, r5, asr #31
 80072ca: f850 5c38    	ldr	r5, [r0, #-56]
 80072ce: 4411         	add	r1, r2
 80072d0: ea4f 0244    	lsl.w	r2, r4, #0x1
 80072d4: ea82 72e4    	eor.w	r2, r2, r4, asr #31
 80072d8: f850 4c34    	ldr	r4, [r0, #-52]
 80072dc: 4411         	add	r1, r2
 80072de: ea4f 0245    	lsl.w	r2, r5, #0x1
 80072e2: ea82 72e5    	eor.w	r2, r2, r5, asr #31
 80072e6: f850 5c30    	ldr	r5, [r0, #-48]
 80072ea: 4411         	add	r1, r2
 80072ec: ea4f 0244    	lsl.w	r2, r4, #0x1
 80072f0: ea82 72e4    	eor.w	r2, r2, r4, asr #31
 80072f4: f850 4c2c    	ldr	r4, [r0, #-44]
 80072f8: 4411         	add	r1, r2
 80072fa: ea4f 0245    	lsl.w	r2, r5, #0x1
 80072fe: ea82 72e5    	eor.w	r2, r2, r5, asr #31
 8007302: f850 5c28    	ldr	r5, [r0, #-40]
 8007306: 4411         	add	r1, r2
 8007308: ea4f 0244    	lsl.w	r2, r4, #0x1
 800730c: ea82 72e4    	eor.w	r2, r2, r4, asr #31
 8007310: f850 4c24    	ldr	r4, [r0, #-36]
 8007314: 4411         	add	r1, r2
 8007316: ea4f 0245    	lsl.w	r2, r5, #0x1
 800731a: ea82 72e5    	eor.w	r2, r2, r5, asr #31
 800731e: f850 5c20    	ldr	r5, [r0, #-32]
 8007322: 4411         	add	r1, r2
 8007324: ea4f 0244    	lsl.w	r2, r4, #0x1
 8007328: ea82 72e4    	eor.w	r2, r2, r4, asr #31
 800732c: f850 4c1c    	ldr	r4, [r0, #-28]
 8007330: 4411         	add	r1, r2
 8007332: ea4f 0245    	lsl.w	r2, r5, #0x1
 8007336: ea82 72e5    	eor.w	r2, r2, r5, asr #31
 800733a: f850 5c18    	ldr	r5, [r0, #-24]
 800733e: 4411         	add	r1, r2
 8007340: ea4f 0244    	lsl.w	r2, r4, #0x1
 8007344: ea82 72e4    	eor.w	r2, r2, r4, asr #31
 8007348: f850 4c14    	ldr	r4, [r0, #-20]
 800734c: 4411         	add	r1, r2
 800734e: ea4f 0245    	lsl.w	r2, r5, #0x1
 8007352: ea82 72e5    	eor.w	r2, r2, r5, asr #31
 8007356: f850 5c10    	ldr	r5, [r0, #-16]
 800735a: 4411         	add	r1, r2
 800735c: ea4f 0244    	lsl.w	r2, r4, #0x1
 8007360: ea82 72e4    	eor.w	r2, r2, r4, asr #31
 8007364: 4411         	add	r1, r2
 8007366: ea4f 0245    	lsl.w	r2, r5, #0x1
 800736a: ea82 72e5    	eor.w	r2, r2, r5, asr #31
 800736e: 4411         	add	r1, r2
 8007370: ea4f 0243    	lsl.w	r2, r3, #0x1
 8007374: ea82 72e3    	eor.w	r2, r2, r3, asr #31
 8007378: 4411         	add	r1, r2
 800737a: ea4f 0246    	lsl.w	r2, r6, #0x1
 800737e: ea82 72e6    	eor.w	r2, r2, r6, asr #31
 8007382: 4411         	add	r1, r2
 8007384: ea4f 024a    	lsl.w	r2, r10, #0x1
 8007388: ea82 72ea    	eor.w	r2, r2, r10, asr #31
 800738c: 4411         	add	r1, r2
 800738e: ea4f 024e    	lsl.w	r2, lr, #0x1
 8007392: ea82 72ee    	eor.w	r2, r2, lr, asr #31
 8007396: eb02 0301    	add.w	r3, r2, r1
 800739a: f47f af18    	bne.w	0x80071ce <raylar_crcl_encoder_service::benchmark::fold_sum::h16b920496cf2076e+0x7c0> @ imm = #-0x1d0
 800739e: 9302         	str	r3, [sp, #0x8]
 80073a0: a802         	add	r0, sp, #0x8
 80073a2: e9dd 1000    	ldrd	r1, r0, [sp]
 80073a6: 4048         	eors	r0, r1
 80073a8: 9902         	ldr	r1, [sp, #0x8]
 80073aa: ea80 0008    	eor.w	r0, r0, r8
 80073ae: ea80 000c    	eor.w	r0, r0, r12
 80073b2: 4048         	eors	r0, r1
 80073b4: b003         	add	sp, #0xc
 80073b6: e8bd 0f00    	pop.w	{r8, r9, r10, r11}
 80073ba: bdf0         	pop	{r4, r5, r6, r7, pc}

080073bc <raylar_crcl_encoder_service::benchmark::rounding::h32a8b1c6cb68e077>:
 80073bc: b5f0         	push	{r4, r5, r6, r7, lr}
 80073be: af03         	add	r7, sp, #0xc
 80073c0: e92d 0b00    	push.w	{r8, r9, r11}
 80073c4: b082         	sub	sp, #0x8
 80073c6: 6801         	ldr	r1, [r0]
 80073c8: 9101         	str	r1, [sp, #0x4]
 80073ca: a901         	add	r1, sp, #0x4
 80073cc: f645 716c    	movw	r1, #0x5f6c
 80073d0: 9b01         	ldr	r3, [sp, #0x4]
 80073d2: 4401         	add	r1, r0
 80073d4: 1e5a         	subs	r2, r3, #0x1
 80073d6: f003 031f    	and	r3, r3, #0x1f
 80073da: f002 0c1f    	and	r12, r2, #0x1f
 80073de: 2201         	movs	r2, #0x1
 80073e0: fa02 f20c    	lsl.w	r2, r2, r12
 80073e4: f44f 6c80    	mov.w	r12, #0x400
 80073e8: f8d1 40d8    	ldr.w	r4, [r1, #0xd8]
 80073ec: f1bc 0c10    	subs.w	r12, r12, #0x10
 80073f0: f8d1 50f0    	ldr.w	r5, [r1, #0xf0]
 80073f4: 4414         	add	r4, r2
 80073f6: f8d1 8108    	ldr.w	r8, [r1, #0x108]
 80073fa: f8d1 60c0    	ldr.w	r6, [r1, #0xc0]
 80073fe: fa44 f403    	asr.w	r4, r4, r3
 8007402: f8d1 e198    	ldr.w	lr, [r1, #0x198]
 8007406: f8c1 40d8    	str.w	r4, [r1, #0xd8]
 800740a: eb05 0402    	add.w	r4, r5, r2
 800740e: 4416         	add	r6, r2
 8007410: f8d1 5120    	ldr.w	r5, [r1, #0x120]
 8007414: fa44 f403    	asr.w	r4, r4, r3
 8007418: f8d1 9180    	ldr.w	r9, [r1, #0x180]
 800741c: f8c1 40f0    	str.w	r4, [r1, #0xf0]
 8007420: eb08 0402    	add.w	r4, r8, r2
 8007424: fa46 f603    	asr.w	r6, r6, r3
 8007428: fa44 f403    	asr.w	r4, r4, r3
 800742c: f8c1 60c0    	str.w	r6, [r1, #0xc0]
 8007430: f8c1 4108    	str.w	r4, [r1, #0x108]
 8007434: eb05 0402    	add.w	r4, r5, r2
 8007438: f8d1 6138    	ldr.w	r6, [r1, #0x138]
 800743c: fa44 f403    	asr.w	r4, r4, r3
 8007440: f8d1 5150    	ldr.w	r5, [r1, #0x150]
 8007444: f8c1 4120    	str.w	r4, [r1, #0x120]
 8007448: eb06 0402    	add.w	r4, r6, r2
 800744c: f8d1 6168    	ldr.w	r6, [r1, #0x168]
 8007450: fa44 f403    	asr.w	r4, r4, r3
 8007454: f8c1 4138    	str.w	r4, [r1, #0x138]
 8007458: eb05 0402    	add.w	r4, r5, r2
 800745c: fa44 f403    	asr.w	r4, r4, r3
 8007460: f8c1 4150    	str.w	r4, [r1, #0x150]
 8007464: eb06 0402    	add.w	r4, r6, r2
 8007468: eb0e 0602    	add.w	r6, lr, r2
 800746c: fa44 f403    	asr.w	r4, r4, r3
 8007470: fa46 f603    	asr.w	r6, r6, r3
 8007474: f8c1 4168    	str.w	r4, [r1, #0x168]
 8007478: f8c1 6198    	str.w	r6, [r1, #0x198]
 800747c: eb09 0402    	add.w	r4, r9, r2
 8007480: f8d1 61b0    	ldr.w	r6, [r1, #0x1b0]
 8007484: fa44 f403    	asr.w	r4, r4, r3
 8007488: 4416         	add	r6, r2
 800748a: f8c1 4180    	str.w	r4, [r1, #0x180]
 800748e: fa46 f603    	asr.w	r6, r6, r3
 8007492: f8c1 61b0    	str.w	r6, [r1, #0x1b0]
 8007496: f8d1 61c8    	ldr.w	r6, [r1, #0x1c8]
 800749a: 4416         	add	r6, r2
 800749c: fa46 f603    	asr.w	r6, r6, r3
 80074a0: f8c1 61c8    	str.w	r6, [r1, #0x1c8]
 80074a4: f8d1 61e0    	ldr.w	r6, [r1, #0x1e0]
 80074a8: 4416         	add	r6, r2
 80074aa: fa46 f603    	asr.w	r6, r6, r3
 80074ae: f8c1 61e0    	str.w	r6, [r1, #0x1e0]
 80074b2: f8d1 61f8    	ldr.w	r6, [r1, #0x1f8]
 80074b6: 4416         	add	r6, r2
 80074b8: fa46 f603    	asr.w	r6, r6, r3
 80074bc: f8c1 61f8    	str.w	r6, [r1, #0x1f8]
 80074c0: f8d1 6210    	ldr.w	r6, [r1, #0x210]
 80074c4: 4416         	add	r6, r2
 80074c6: fa46 f603    	asr.w	r6, r6, r3
 80074ca: f8c1 6210    	str.w	r6, [r1, #0x210]
 80074ce: f8d1 6228    	ldr.w	r6, [r1, #0x228]
 80074d2: 4416         	add	r6, r2
 80074d4: fa46 f603    	asr.w	r6, r6, r3
 80074d8: f8c1 6228    	str.w	r6, [r1, #0x228]
 80074dc: f501 71c0    	add.w	r1, r1, #0x180
 80074e0: d182         	bne	0x80073e8 <raylar_crcl_encoder_service::benchmark::rounding::h32a8b1c6cb68e077+0x2c> @ imm = #-0xfc
 80074e2: 6881         	ldr	r1, [r0, #0x8]
 80074e4: 9101         	str	r1, [sp, #0x4]
 80074e6: a901         	add	r1, sp, #0x4
 80074e8: f645 61b0    	movw	r1, #0x5eb0
 80074ec: 9b01         	ldr	r3, [sp, #0x4]
 80074ee: 4401         	add	r1, r0
 80074f0: 1e5a         	subs	r2, r3, #0x1
 80074f2: f003 031f    	and	r3, r3, #0x1f
 80074f6: f002 0c1f    	and	r12, r2, #0x1f
 80074fa: 2201         	movs	r2, #0x1
 80074fc: fa02 f20c    	lsl.w	r2, r2, r12
 8007500: f44f 6c80    	mov.w	r12, #0x400
 8007504: f8d1 4198    	ldr.w	r4, [r1, #0x198]
 8007508: f1bc 0c10    	subs.w	r12, r12, #0x10
 800750c: f8d1 51b0    	ldr.w	r5, [r1, #0x1b0]
 8007510: 4414         	add	r4, r2
 8007512: f8d1 81c8    	ldr.w	r8, [r1, #0x1c8]
 8007516: f8d1 e180    	ldr.w	lr, [r1, #0x180]
 800751a: fa44 f403    	asr.w	r4, r4, r3
 800751e: f8d1 9240    	ldr.w	r9, [r1, #0x240]
 8007522: f8c1 4198    	str.w	r4, [r1, #0x198]
 8007526: eb05 0402    	add.w	r4, r5, r2
 800752a: eb0e 0602    	add.w	r6, lr, r2
 800752e: f8d1 51e0    	ldr.w	r5, [r1, #0x1e0]
 8007532: fa44 f403    	asr.w	r4, r4, r3
 8007536: f8d1 e258    	ldr.w	lr, [r1, #0x258]
 800753a: f8c1 41b0    	str.w	r4, [r1, #0x1b0]
 800753e: eb08 0402    	add.w	r4, r8, r2
 8007542: fa46 f603    	asr.w	r6, r6, r3
 8007546: fa44 f403    	asr.w	r4, r4, r3
 800754a: f8c1 6180    	str.w	r6, [r1, #0x180]
 800754e: f8c1 41c8    	str.w	r4, [r1, #0x1c8]
 8007552: eb05 0402    	add.w	r4, r5, r2
 8007556: f8d1 61f8    	ldr.w	r6, [r1, #0x1f8]
 800755a: fa44 f403    	asr.w	r4, r4, r3
 800755e: f8d1 5210    	ldr.w	r5, [r1, #0x210]
 8007562: f8c1 41e0    	str.w	r4, [r1, #0x1e0]
 8007566: eb06 0402    	add.w	r4, r6, r2
 800756a: f8d1 6228    	ldr.w	r6, [r1, #0x228]
 800756e: fa44 f403    	asr.w	r4, r4, r3
 8007572: f8c1 41f8    	str.w	r4, [r1, #0x1f8]
 8007576: eb05 0402    	add.w	r4, r5, r2
 800757a: fa44 f403    	asr.w	r4, r4, r3
 800757e: f8c1 4210    	str.w	r4, [r1, #0x210]
 8007582: eb06 0402    	add.w	r4, r6, r2
 8007586: eb0e 0602    	add.w	r6, lr, r2
 800758a: fa44 f403    	asr.w	r4, r4, r3
 800758e: fa46 f603    	asr.w	r6, r6, r3
 8007592: f8c1 4228    	str.w	r4, [r1, #0x228]
 8007596: f8c1 6258    	str.w	r6, [r1, #0x258]
 800759a: eb09 0402    	add.w	r4, r9, r2
 800759e: f8d1 6270    	ldr.w	r6, [r1, #0x270]
 80075a2: fa44 f403    	asr.w	r4, r4, r3
 80075a6: 4416         	add	r6, r2
 80075a8: f8c1 4240    	str.w	r4, [r1, #0x240]
 80075ac: fa46 f603    	asr.w	r6, r6, r3
 80075b0: f8c1 6270    	str.w	r6, [r1, #0x270]
 80075b4: f8d1 6288    	ldr.w	r6, [r1, #0x288]
 80075b8: 4416         	add	r6, r2
 80075ba: fa46 f603    	asr.w	r6, r6, r3
 80075be: f8c1 6288    	str.w	r6, [r1, #0x288]
 80075c2: f8d1 62a0    	ldr.w	r6, [r1, #0x2a0]
 80075c6: 4416         	add	r6, r2
 80075c8: fa46 f603    	asr.w	r6, r6, r3
 80075cc: f8c1 62a0    	str.w	r6, [r1, #0x2a0]
 80075d0: f8d1 62b8    	ldr.w	r6, [r1, #0x2b8]
 80075d4: 4416         	add	r6, r2
 80075d6: fa46 f603    	asr.w	r6, r6, r3
 80075da: f8c1 62b8    	str.w	r6, [r1, #0x2b8]
 80075de: f8d1 62d0    	ldr.w	r6, [r1, #0x2d0]
 80075e2: 4416         	add	r6, r2
 80075e4: fa46 f603    	asr.w	r6, r6, r3
 80075e8: f8c1 62d0    	str.w	r6, [r1, #0x2d0]
 80075ec: f8d1 62e8    	ldr.w	r6, [r1, #0x2e8]
 80075f0: 4416         	add	r6, r2
 80075f2: fa46 f603    	asr.w	r6, r6, r3
 80075f6: f8c1 62e8    	str.w	r6, [r1, #0x2e8]
 80075fa: f501 71c0    	add.w	r1, r1, #0x180
 80075fe: d181         	bne	0x8007504 <raylar_crcl_encoder_service::benchmark::rounding::h32a8b1c6cb68e077+0x148> @ imm = #-0xfe
 8007600: 6901         	ldr	r1, [r0, #0x10]
 8007602: 2601         	movs	r6, #0x1
 8007604: 9101         	str	r1, [sp, #0x4]
 8007606: a901         	add	r1, sp, #0x4
 8007608: f645 7174    	movw	r1, #0x5f74
 800760c: f44f 6c80    	mov.w	r12, #0x400
 8007610: 9b01         	ldr	r3, [sp, #0x4]
 8007612: 4401         	add	r1, r0
 8007614: 1e5a         	subs	r2, r3, #0x1
 8007616: f003 031f    	and	r3, r3, #0x1f
 800761a: f002 021f    	and	r2, r2, #0x1f
 800761e: fa06 f202    	lsl.w	r2, r6, r2
 8007622: f8d1 40d8    	ldr.w	r4, [r1, #0xd8]
 8007626: f1bc 0c10    	subs.w	r12, r12, #0x10
 800762a: f8d1 50f0    	ldr.w	r5, [r1, #0xf0]
 800762e: 4414         	add	r4, r2
 8007630: f8d1 8108    	ldr.w	r8, [r1, #0x108]
 8007634: f8d1 60c0    	ldr.w	r6, [r1, #0xc0]
 8007638: fa44 f403    	asr.w	r4, r4, r3
 800763c: f8d1 e198    	ldr.w	lr, [r1, #0x198]
 8007640: f8c1 40d8    	str.w	r4, [r1, #0xd8]
 8007644: eb05 0402    	add.w	r4, r5, r2
 8007648: 4416         	add	r6, r2
 800764a: f8d1 5120    	ldr.w	r5, [r1, #0x120]
 800764e: fa44 f403    	asr.w	r4, r4, r3
 8007652: f8d1 9180    	ldr.w	r9, [r1, #0x180]
 8007656: f8c1 40f0    	str.w	r4, [r1, #0xf0]
 800765a: eb08 0402    	add.w	r4, r8, r2
 800765e: fa46 f603    	asr.w	r6, r6, r3
 8007662: fa44 f403    	asr.w	r4, r4, r3
 8007666: f8c1 60c0    	str.w	r6, [r1, #0xc0]
 800766a: f8c1 4108    	str.w	r4, [r1, #0x108]
 800766e: eb05 0402    	add.w	r4, r5, r2
 8007672: f8d1 6138    	ldr.w	r6, [r1, #0x138]
 8007676: fa44 f403    	asr.w	r4, r4, r3
 800767a: f8d1 5150    	ldr.w	r5, [r1, #0x150]
 800767e: f8c1 4120    	str.w	r4, [r1, #0x120]
 8007682: eb06 0402    	add.w	r4, r6, r2
 8007686: f8d1 6168    	ldr.w	r6, [r1, #0x168]
 800768a: fa44 f403    	asr.w	r4, r4, r3
 800768e: f8c1 4138    	str.w	r4, [r1, #0x138]
 8007692: eb05 0402    	add.w	r4, r5, r2
 8007696: fa44 f403    	asr.w	r4, r4, r3
 800769a: f8c1 4150    	str.w	r4, [r1, #0x150]
 800769e: eb06 0402    	add.w	r4, r6, r2
 80076a2: eb0e 0602    	add.w	r6, lr, r2
 80076a6: fa44 f403    	asr.w	r4, r4, r3
 80076aa: fa46 f603    	asr.w	r6, r6, r3
 80076ae: f8c1 4168    	str.w	r4, [r1, #0x168]
 80076b2: f8c1 6198    	str.w	r6, [r1, #0x198]
 80076b6: eb09 0402    	add.w	r4, r9, r2
 80076ba: f8d1 61b0    	ldr.w	r6, [r1, #0x1b0]
 80076be: fa44 f403    	asr.w	r4, r4, r3
 80076c2: 4416         	add	r6, r2
 80076c4: f8c1 4180    	str.w	r4, [r1, #0x180]
 80076c8: fa46 f603    	asr.w	r6, r6, r3
 80076cc: f8c1 61b0    	str.w	r6, [r1, #0x1b0]
 80076d0: f8d1 61c8    	ldr.w	r6, [r1, #0x1c8]
 80076d4: 4416         	add	r6, r2
 80076d6: fa46 f603    	asr.w	r6, r6, r3
 80076da: f8c1 61c8    	str.w	r6, [r1, #0x1c8]
 80076de: f8d1 61e0    	ldr.w	r6, [r1, #0x1e0]
 80076e2: 4416         	add	r6, r2
 80076e4: fa46 f603    	asr.w	r6, r6, r3
 80076e8: f8c1 61e0    	str.w	r6, [r1, #0x1e0]
 80076ec: f8d1 61f8    	ldr.w	r6, [r1, #0x1f8]
 80076f0: 4416         	add	r6, r2
 80076f2: fa46 f603    	asr.w	r6, r6, r3
 80076f6: f8c1 61f8    	str.w	r6, [r1, #0x1f8]
 80076fa: f8d1 6210    	ldr.w	r6, [r1, #0x210]
 80076fe: 4416         	add	r6, r2
 8007700: fa46 f603    	asr.w	r6, r6, r3
 8007704: f8c1 6210    	str.w	r6, [r1, #0x210]
 8007708: f8d1 6228    	ldr.w	r6, [r1, #0x228]
 800770c: 4416         	add	r6, r2
 800770e: fa46 f603    	asr.w	r6, r6, r3
 8007712: f8c1 6228    	str.w	r6, [r1, #0x228]
 8007716: f501 71c0    	add.w	r1, r1, #0x180
 800771a: d182         	bne	0x8007622 <raylar_crcl_encoder_service::benchmark::rounding::h32a8b1c6cb68e077+0x266> @ imm = #-0xfc
 800771c: 6981         	ldr	r1, [r0, #0x18]
 800771e: 2601         	movs	r6, #0x1
 8007720: 9101         	str	r1, [sp, #0x4]
 8007722: a901         	add	r1, sp, #0x4
 8007724: f645 61b8    	movw	r1, #0x5eb8
 8007728: f44f 6c80    	mov.w	r12, #0x400
 800772c: 9b01         	ldr	r3, [sp, #0x4]
 800772e: 4401         	add	r1, r0
 8007730: 1e5a         	subs	r2, r3, #0x1
 8007732: f003 031f    	and	r3, r3, #0x1f
 8007736: f002 021f    	and	r2, r2, #0x1f
 800773a: fa06 f202    	lsl.w	r2, r6, r2
 800773e: f8d1 4198    	ldr.w	r4, [r1, #0x198]
 8007742: f1bc 0c10    	subs.w	r12, r12, #0x10
 8007746: f8d1 51b0    	ldr.w	r5, [r1, #0x1b0]
 800774a: 4414         	add	r4, r2
 800774c: f8d1 81c8    	ldr.w	r8, [r1, #0x1c8]
 8007750: f8d1 6180    	ldr.w	r6, [r1, #0x180]
 8007754: fa44 f403    	asr.w	r4, r4, r3
 8007758: f8d1 e258    	ldr.w	lr, [r1, #0x258]
 800775c: f8c1 4198    	str.w	r4, [r1, #0x198]
 8007760: eb05 0402    	add.w	r4, r5, r2
 8007764: 4416         	add	r6, r2
 8007766: f8d1 51e0    	ldr.w	r5, [r1, #0x1e0]
 800776a: fa44 f403    	asr.w	r4, r4, r3
 800776e: f8d1 9240    	ldr.w	r9, [r1, #0x240]
 8007772: f8c1 41b0    	str.w	r4, [r1, #0x1b0]
 8007776: eb08 0402    	add.w	r4, r8, r2
 800777a: fa46 f603    	asr.w	r6, r6, r3
 800777e: fa44 f403    	asr.w	r4, r4, r3
 8007782: f8c1 6180    	str.w	r6, [r1, #0x180]
 8007786: f8c1 41c8    	str.w	r4, [r1, #0x1c8]
 800778a: eb05 0402    	add.w	r4, r5, r2
 800778e: f8d1 61f8    	ldr.w	r6, [r1, #0x1f8]
 8007792: fa44 f403    	asr.w	r4, r4, r3
 8007796: f8d1 5210    	ldr.w	r5, [r1, #0x210]
 800779a: f8c1 41e0    	str.w	r4, [r1, #0x1e0]
 800779e: eb06 0402    	add.w	r4, r6, r2
 80077a2: f8d1 6228    	ldr.w	r6, [r1, #0x228]
 80077a6: fa44 f403    	asr.w	r4, r4, r3
 80077aa: f8c1 41f8    	str.w	r4, [r1, #0x1f8]
 80077ae: eb05 0402    	add.w	r4, r5, r2
 80077b2: fa44 f403    	asr.w	r4, r4, r3
 80077b6: f8c1 4210    	str.w	r4, [r1, #0x210]
 80077ba: eb06 0402    	add.w	r4, r6, r2
 80077be: eb0e 0602    	add.w	r6, lr, r2
 80077c2: fa44 f403    	asr.w	r4, r4, r3
 80077c6: fa46 f603    	asr.w	r6, r6, r3
 80077ca: f8c1 4228    	str.w	r4, [r1, #0x228]
 80077ce: f8c1 6258    	str.w	r6, [r1, #0x258]
 80077d2: eb09 0402    	add.w	r4, r9, r2
 80077d6: f8d1 6270    	ldr.w	r6, [r1, #0x270]
 80077da: fa44 f403    	asr.w	r4, r4, r3
 80077de: 4416         	add	r6, r2
 80077e0: f8c1 4240    	str.w	r4, [r1, #0x240]
 80077e4: fa46 f603    	asr.w	r6, r6, r3
 80077e8: f8c1 6270    	str.w	r6, [r1, #0x270]
 80077ec: f8d1 6288    	ldr.w	r6, [r1, #0x288]
 80077f0: 4416         	add	r6, r2
 80077f2: fa46 f603    	asr.w	r6, r6, r3
 80077f6: f8c1 6288    	str.w	r6, [r1, #0x288]
 80077fa: f8d1 62a0    	ldr.w	r6, [r1, #0x2a0]
 80077fe: 4416         	add	r6, r2
 8007800: fa46 f603    	asr.w	r6, r6, r3
 8007804: f8c1 62a0    	str.w	r6, [r1, #0x2a0]
 8007808: f8d1 62b8    	ldr.w	r6, [r1, #0x2b8]
 800780c: 4416         	add	r6, r2
 800780e: fa46 f603    	asr.w	r6, r6, r3
 8007812: f8c1 62b8    	str.w	r6, [r1, #0x2b8]
 8007816: f8d1 62d0    	ldr.w	r6, [r1, #0x2d0]
 800781a: 4416         	add	r6, r2
 800781c: fa46 f603    	asr.w	r6, r6, r3
 8007820: f8c1 62d0    	str.w	r6, [r1, #0x2d0]
 8007824: f8d1 62e8    	ldr.w	r6, [r1, #0x2e8]
 8007828: 4416         	add	r6, r2
 800782a: fa46 f603    	asr.w	r6, r6, r3
 800782e: f8c1 62e8    	str.w	r6, [r1, #0x2e8]
 8007832: f501 71c0    	add.w	r1, r1, #0x180
 8007836: d182         	bne	0x800773e <raylar_crcl_encoder_service::benchmark::rounding::h32a8b1c6cb68e077+0x382> @ imm = #-0xfc
 8007838: 6a01         	ldr	r1, [r0, #0x20]
 800783a: 2601         	movs	r6, #0x1
 800783c: 9101         	str	r1, [sp, #0x4]
 800783e: a901         	add	r1, sp, #0x4
 8007840: f645 717c    	movw	r1, #0x5f7c
 8007844: f44f 6c80    	mov.w	r12, #0x400
 8007848: 9b01         	ldr	r3, [sp, #0x4]
 800784a: 4401         	add	r1, r0
 800784c: 1e5a         	subs	r2, r3, #0x1
 800784e: f003 031f    	and	r3, r3, #0x1f
 8007852: f002 021f    	and	r2, r2, #0x1f
 8007856: fa06 f202    	lsl.w	r2, r6, r2
 800785a: f8d1 40d8    	ldr.w	r4, [r1, #0xd8]
 800785e: f1bc 0c10    	subs.w	r12, r12, #0x10
 8007862: f8d1 50f0    	ldr.w	r5, [r1, #0xf0]
 8007866: 4414         	add	r4, r2
 8007868: f8d1 8108    	ldr.w	r8, [r1, #0x108]
 800786c: f8d1 60c0    	ldr.w	r6, [r1, #0xc0]
 8007870: fa44 f403    	asr.w	r4, r4, r3
 8007874: f8d1 e198    	ldr.w	lr, [r1, #0x198]
 8007878: f8c1 40d8    	str.w	r4, [r1, #0xd8]
 800787c: eb05 0402    	add.w	r4, r5, r2
 8007880: 4416         	add	r6, r2
 8007882: f8d1 5120    	ldr.w	r5, [r1, #0x120]
 8007886: fa44 f403    	asr.w	r4, r4, r3
 800788a: f8d1 9180    	ldr.w	r9, [r1, #0x180]
 800788e: f8c1 40f0    	str.w	r4, [r1, #0xf0]
 8007892: eb08 0402    	add.w	r4, r8, r2
 8007896: fa46 f603    	asr.w	r6, r6, r3
 800789a: fa44 f403    	asr.w	r4, r4, r3
 800789e: f8c1 60c0    	str.w	r6, [r1, #0xc0]
 80078a2: f8c1 4108    	str.w	r4, [r1, #0x108]
 80078a6: eb05 0402    	add.w	r4, r5, r2
 80078aa: f8d1 6138    	ldr.w	r6, [r1, #0x138]
 80078ae: fa44 f403    	asr.w	r4, r4, r3
 80078b2: f8d1 5150    	ldr.w	r5, [r1, #0x150]
 80078b6: f8c1 4120    	str.w	r4, [r1, #0x120]
 80078ba: eb06 0402    	add.w	r4, r6, r2
 80078be: f8d1 6168    	ldr.w	r6, [r1, #0x168]
 80078c2: fa44 f403    	asr.w	r4, r4, r3
 80078c6: f8c1 4138    	str.w	r4, [r1, #0x138]
 80078ca: eb05 0402    	add.w	r4, r5, r2
 80078ce: fa44 f403    	asr.w	r4, r4, r3
 80078d2: f8c1 4150    	str.w	r4, [r1, #0x150]
 80078d6: eb06 0402    	add.w	r4, r6, r2
 80078da: eb0e 0602    	add.w	r6, lr, r2
 80078de: fa44 f403    	asr.w	r4, r4, r3
 80078e2: fa46 f603    	asr.w	r6, r6, r3
 80078e6: f8c1 4168    	str.w	r4, [r1, #0x168]
 80078ea: f8c1 6198    	str.w	r6, [r1, #0x198]
 80078ee: eb09 0402    	add.w	r4, r9, r2
 80078f2: f8d1 61b0    	ldr.w	r6, [r1, #0x1b0]
 80078f6: fa44 f403    	asr.w	r4, r4, r3
 80078fa: 4416         	add	r6, r2
 80078fc: f8c1 4180    	str.w	r4, [r1, #0x180]
 8007900: fa46 f603    	asr.w	r6, r6, r3
 8007904: f8c1 61b0    	str.w	r6, [r1, #0x1b0]
 8007908: f8d1 61c8    	ldr.w	r6, [r1, #0x1c8]
 800790c: 4416         	add	r6, r2
 800790e: fa46 f603    	asr.w	r6, r6, r3
 8007912: f8c1 61c8    	str.w	r6, [r1, #0x1c8]
 8007916: f8d1 61e0    	ldr.w	r6, [r1, #0x1e0]
 800791a: 4416         	add	r6, r2
 800791c: fa46 f603    	asr.w	r6, r6, r3
 8007920: f8c1 61e0    	str.w	r6, [r1, #0x1e0]
 8007924: f8d1 61f8    	ldr.w	r6, [r1, #0x1f8]
 8007928: 4416         	add	r6, r2
 800792a: fa46 f603    	asr.w	r6, r6, r3
 800792e: f8c1 61f8    	str.w	r6, [r1, #0x1f8]
 8007932: f8d1 6210    	ldr.w	r6, [r1, #0x210]
 8007936: 4416         	add	r6, r2
 8007938: fa46 f603    	asr.w	r6, r6, r3
 800793c: f8c1 6210    	str.w	r6, [r1, #0x210]
 8007940: f8d1 6228    	ldr.w	r6, [r1, #0x228]
 8007944: 4416         	add	r6, r2
 8007946: fa46 f603    	asr.w	r6, r6, r3
 800794a: f8c1 6228    	str.w	r6, [r1, #0x228]
 800794e: f501 71c0    	add.w	r1, r1, #0x180
 8007952: d182         	bne	0x800785a <raylar_crcl_encoder_service::benchmark::rounding::h32a8b1c6cb68e077+0x49e> @ imm = #-0xfc
 8007954: f246 0128    	movw	r1, #0x6028
 8007958: 4401         	add	r1, r0
 800795a: 9100         	str	r1, [sp]
 800795c: 4669         	mov	r1, sp
 800795e: f246 012c    	movw	r1, #0x602c
 8007962: 5840         	ldr	r0, [r0, r1]
 8007964: b002         	add	sp, #0x8
 8007966: e8bd 0b00    	pop.w	{r8, r9, r11}
 800796a: bdf0         	pop	{r4, r5, r6, r7, pc}

0800796c <raylar_crcl_encoder_service::benchmark::sum_only::he5272faae351e30d>:
 800796c: b5f0         	push	{r4, r5, r6, r7, lr}
 800796e: af03         	add	r7, sp, #0xc
 8007970: e92d 0f00    	push.w	{r8, r9, r10, r11}
 8007974: b08a         	sub	sp, #0x28
 8007976: 9004         	str	r0, [sp, #0x10]
 8007978: f500 30b8    	add.w	r0, r0, #0x17000
 800797c: 3010         	adds	r0, #0x10
 800797e: 2500         	movs	r5, #0x0
 8007980: f240 39ff    	movw	r9, #0x3ff
 8007984: f850 1f84    	ldr	r1, [r0, #132]!
 8007988: f1b9 0921    	subs.w	r9, r9, #0x21
 800798c: f1a0 0c78    	sub.w	r12, r0, #0x78
 8007990: 9108         	str	r1, [sp, #0x20]
 8007992: e950 4220    	ldrd	r4, r2, [r0, #-128]
 8007996: f1a0 0e60    	sub.w	lr, r0, #0x60
 800799a: e89c 1048    	ldm.w	r12, {r3, r6, r12}
 800799e: 442c         	add	r4, r5
 80079a0: 4422         	add	r2, r4
 80079a2: f850 1c04    	ldr	r1, [r0, #-4]
 80079a6: 441a         	add	r2, r3
 80079a8: 9107         	str	r1, [sp, #0x1c]
 80079aa: f850 1c08    	ldr	r1, [r0, #-8]
 80079ae: 4432         	add	r2, r6
 80079b0: 9106         	str	r1, [sp, #0x18]
 80079b2: f850 1c0c    	ldr	r1, [r0, #-12]
 80079b6: e950 ab1b    	ldrd	r10, r11, [r0, #-108]
 80079ba: 9105         	str	r1, [sp, #0x14]
 80079bc: eb0c 0102    	add.w	r1, r12, r2
 80079c0: 4451         	add	r1, r10
 80079c2: f850 8c64    	ldr	r8, [r0, #-100]
 80079c6: 4459         	add	r1, r11
 80079c8: e89e 545c    	ldm.w	lr, {r2, r3, r4, r6, r10, r12, lr}
 80079cc: 4441         	add	r1, r8
 80079ce: 4411         	add	r1, r2
 80079d0: f850 5c44    	ldr	r5, [r0, #-68]
 80079d4: 4419         	add	r1, r3
 80079d6: f1a0 0b2c    	sub.w	r11, r0, #0x2c
 80079da: 4421         	add	r1, r4
 80079dc: 4431         	add	r1, r6
 80079de: f1a0 0640    	sub.w	r6, r0, #0x40
 80079e2: 4451         	add	r1, r10
 80079e4: e89b 0d00    	ldm.w	r11, {r8, r10, r11}
 80079e8: 4461         	add	r1, r12
 80079ea: 4471         	add	r1, lr
 80079ec: ce4c         	ldm	r6, {r2, r3, r6}
 80079ee: 4429         	add	r1, r5
 80079f0: 4411         	add	r1, r2
 80079f2: e950 4c0d    	ldrd	r4, r12, [r0, #-52]
 80079f6: 4419         	add	r1, r3
 80079f8: f850 2c20    	ldr	r2, [r0, #-32]
 80079fc: 4431         	add	r1, r6
 80079fe: f850 3c1c    	ldr	r3, [r0, #-28]
 8007a02: 4421         	add	r1, r4
 8007a04: f850 6c18    	ldr	r6, [r0, #-24]
 8007a08: 4461         	add	r1, r12
 8007a0a: 4441         	add	r1, r8
 8007a0c: 4451         	add	r1, r10
 8007a0e: 4459         	add	r1, r11
 8007a10: 4411         	add	r1, r2
 8007a12: f850 2c14    	ldr	r2, [r0, #-20]
 8007a16: 4419         	add	r1, r3
 8007a18: f850 3c10    	ldr	r3, [r0, #-16]
 8007a1c: 4431         	add	r1, r6
 8007a1e: 4411         	add	r1, r2
 8007a20: 9a05         	ldr	r2, [sp, #0x14]
 8007a22: 4419         	add	r1, r3
 8007a24: 4411         	add	r1, r2
 8007a26: 9a06         	ldr	r2, [sp, #0x18]
 8007a28: 4411         	add	r1, r2
 8007a2a: 9a07         	ldr	r2, [sp, #0x1c]
 8007a2c: 4411         	add	r1, r2
 8007a2e: 9a08         	ldr	r2, [sp, #0x20]
 8007a30: eb02 0501    	add.w	r5, r2, r1
 8007a34: d1a6         	bne	0x8007984 <raylar_crcl_encoder_service::benchmark::sum_only::he5272faae351e30d+0x18> @ imm = #-0xb4
 8007a36: 9509         	str	r5, [sp, #0x24]
 8007a38: a809         	add	r0, sp, #0x24
 8007a3a: 2500         	movs	r5, #0x0
 8007a3c: f240 39ff    	movw	r9, #0x3ff
 8007a40: 9809         	ldr	r0, [sp, #0x24]
 8007a42: 9003         	str	r0, [sp, #0xc]
 8007a44: 9804         	ldr	r0, [sp, #0x10]
 8007a46: f500 30c0    	add.w	r0, r0, #0x18000
 8007a4a: 300c         	adds	r0, #0xc
 8007a4c: f850 1f84    	ldr	r1, [r0, #132]!
 8007a50: f1b9 0921    	subs.w	r9, r9, #0x21
 8007a54: f1a0 0c78    	sub.w	r12, r0, #0x78
 8007a58: 9108         	str	r1, [sp, #0x20]
 8007a5a: e950 4220    	ldrd	r4, r2, [r0, #-128]
 8007a5e: f1a0 0e60    	sub.w	lr, r0, #0x60
 8007a62: e89c 1048    	ldm.w	r12, {r3, r6, r12}
 8007a66: 442c         	add	r4, r5
 8007a68: 4422         	add	r2, r4
 8007a6a: f850 1c04    	ldr	r1, [r0, #-4]
 8007a6e: 441a         	add	r2, r3
 8007a70: 9107         	str	r1, [sp, #0x1c]
 8007a72: f850 1c08    	ldr	r1, [r0, #-8]
 8007a76: 4432         	add	r2, r6
 8007a78: 9106         	str	r1, [sp, #0x18]
 8007a7a: f850 1c0c    	ldr	r1, [r0, #-12]
 8007a7e: e950 ab1b    	ldrd	r10, r11, [r0, #-108]
 8007a82: 9105         	str	r1, [sp, #0x14]
 8007a84: eb0c 0102    	add.w	r1, r12, r2
 8007a88: 4451         	add	r1, r10
 8007a8a: f850 8c64    	ldr	r8, [r0, #-100]
 8007a8e: 4459         	add	r1, r11
 8007a90: e89e 545c    	ldm.w	lr, {r2, r3, r4, r6, r10, r12, lr}
 8007a94: 4441         	add	r1, r8
 8007a96: 4411         	add	r1, r2
 8007a98: f850 5c44    	ldr	r5, [r0, #-68]
 8007a9c: 4419         	add	r1, r3
 8007a9e: f1a0 0b2c    	sub.w	r11, r0, #0x2c
 8007aa2: 4421         	add	r1, r4
 8007aa4: 4431         	add	r1, r6
 8007aa6: f1a0 0640    	sub.w	r6, r0, #0x40
 8007aaa: 4451         	add	r1, r10
 8007aac: e89b 0d00    	ldm.w	r11, {r8, r10, r11}
 8007ab0: 4461         	add	r1, r12
 8007ab2: 4471         	add	r1, lr
 8007ab4: ce4c         	ldm	r6, {r2, r3, r6}
 8007ab6: 4429         	add	r1, r5
 8007ab8: 4411         	add	r1, r2
 8007aba: e950 4c0d    	ldrd	r4, r12, [r0, #-52]
 8007abe: 4419         	add	r1, r3
 8007ac0: f850 2c20    	ldr	r2, [r0, #-32]
 8007ac4: 4431         	add	r1, r6
 8007ac6: f850 3c1c    	ldr	r3, [r0, #-28]
 8007aca: 4421         	add	r1, r4
 8007acc: f850 6c18    	ldr	r6, [r0, #-24]
 8007ad0: 4461         	add	r1, r12
 8007ad2: 4441         	add	r1, r8
 8007ad4: 4451         	add	r1, r10
 8007ad6: 4459         	add	r1, r11
 8007ad8: 4411         	add	r1, r2
 8007ada: f850 2c14    	ldr	r2, [r0, #-20]
 8007ade: 4419         	add	r1, r3
 8007ae0: f850 3c10    	ldr	r3, [r0, #-16]
 8007ae4: 4431         	add	r1, r6
 8007ae6: 4411         	add	r1, r2
 8007ae8: 9a05         	ldr	r2, [sp, #0x14]
 8007aea: 4419         	add	r1, r3
 8007aec: 4411         	add	r1, r2
 8007aee: 9a06         	ldr	r2, [sp, #0x18]
 8007af0: 4411         	add	r1, r2
 8007af2: 9a07         	ldr	r2, [sp, #0x1c]
 8007af4: 4411         	add	r1, r2
 8007af6: 9a08         	ldr	r2, [sp, #0x20]
 8007af8: eb02 0501    	add.w	r5, r2, r1
 8007afc: d1a6         	bne	0x8007a4c <raylar_crcl_encoder_service::benchmark::sum_only::he5272faae351e30d+0xe0> @ imm = #-0xb4
 8007afe: 9509         	str	r5, [sp, #0x24]
 8007b00: a809         	add	r0, sp, #0x24
 8007b02: 2500         	movs	r5, #0x0
 8007b04: f240 39ff    	movw	r9, #0x3ff
 8007b08: 9809         	ldr	r0, [sp, #0x24]
 8007b0a: 9002         	str	r0, [sp, #0x8]
 8007b0c: 9804         	ldr	r0, [sp, #0x10]
 8007b0e: f500 30c8    	add.w	r0, r0, #0x19000
 8007b12: 3008         	adds	r0, #0x8
 8007b14: f850 1f84    	ldr	r1, [r0, #132]!
 8007b18: f1b9 0921    	subs.w	r9, r9, #0x21
 8007b1c: f1a0 0c78    	sub.w	r12, r0, #0x78
 8007b20: 9108         	str	r1, [sp, #0x20]
 8007b22: e950 4220    	ldrd	r4, r2, [r0, #-128]
 8007b26: f1a0 0e60    	sub.w	lr, r0, #0x60
 8007b2a: e89c 1048    	ldm.w	r12, {r3, r6, r12}
 8007b2e: 442c         	add	r4, r5
 8007b30: 4422         	add	r2, r4
 8007b32: f850 1c04    	ldr	r1, [r0, #-4]
 8007b36: 441a         	add	r2, r3
 8007b38: 9107         	str	r1, [sp, #0x1c]
 8007b3a: f850 1c08    	ldr	r1, [r0, #-8]
 8007b3e: 4432         	add	r2, r6
 8007b40: 9106         	str	r1, [sp, #0x18]
 8007b42: f850 1c0c    	ldr	r1, [r0, #-12]
 8007b46: e950 ab1b    	ldrd	r10, r11, [r0, #-108]
 8007b4a: 9105         	str	r1, [sp, #0x14]
 8007b4c: eb0c 0102    	add.w	r1, r12, r2
 8007b50: 4451         	add	r1, r10
 8007b52: f850 8c64    	ldr	r8, [r0, #-100]
 8007b56: 4459         	add	r1, r11
 8007b58: e89e 545c    	ldm.w	lr, {r2, r3, r4, r6, r10, r12, lr}
 8007b5c: 4441         	add	r1, r8
 8007b5e: 4411         	add	r1, r2
 8007b60: f850 5c44    	ldr	r5, [r0, #-68]
 8007b64: 4419         	add	r1, r3
 8007b66: f1a0 0b2c    	sub.w	r11, r0, #0x2c
 8007b6a: 4421         	add	r1, r4
 8007b6c: 4431         	add	r1, r6
 8007b6e: f1a0 0640    	sub.w	r6, r0, #0x40
 8007b72: 4451         	add	r1, r10
 8007b74: e89b 0d00    	ldm.w	r11, {r8, r10, r11}
 8007b78: 4461         	add	r1, r12
 8007b7a: 4471         	add	r1, lr
 8007b7c: ce4c         	ldm	r6, {r2, r3, r6}
 8007b7e: 4429         	add	r1, r5
 8007b80: 4411         	add	r1, r2
 8007b82: e950 4c0d    	ldrd	r4, r12, [r0, #-52]
 8007b86: 4419         	add	r1, r3
 8007b88: f850 2c20    	ldr	r2, [r0, #-32]
 8007b8c: 4431         	add	r1, r6
 8007b8e: f850 3c1c    	ldr	r3, [r0, #-28]
 8007b92: 4421         	add	r1, r4
 8007b94: f850 6c18    	ldr	r6, [r0, #-24]
 8007b98: 4461         	add	r1, r12
 8007b9a: 4441         	add	r1, r8
 8007b9c: 4451         	add	r1, r10
 8007b9e: 4459         	add	r1, r11
 8007ba0: 4411         	add	r1, r2
 8007ba2: f850 2c14    	ldr	r2, [r0, #-20]
 8007ba6: 4419         	add	r1, r3
 8007ba8: f850 3c10    	ldr	r3, [r0, #-16]
 8007bac: 4431         	add	r1, r6
 8007bae: 4411         	add	r1, r2
 8007bb0: 9a05         	ldr	r2, [sp, #0x14]
 8007bb2: 4419         	add	r1, r3
 8007bb4: 4411         	add	r1, r2
 8007bb6: 9a06         	ldr	r2, [sp, #0x18]
 8007bb8: 4411         	add	r1, r2
 8007bba: 9a07         	ldr	r2, [sp, #0x1c]
 8007bbc: 4411         	add	r1, r2
 8007bbe: 9a08         	ldr	r2, [sp, #0x20]
 8007bc0: eb02 0501    	add.w	r5, r2, r1
 8007bc4: d1a6         	bne	0x8007b14 <raylar_crcl_encoder_service::benchmark::sum_only::he5272faae351e30d+0x1a8> @ imm = #-0xb4
 8007bc6: 9509         	str	r5, [sp, #0x24]
 8007bc8: a809         	add	r0, sp, #0x24
 8007bca: f24f 0104    	movw	r1, #0xf004
 8007bce: f24b 046c    	movw	r4, #0xb06c
 8007bd2: 9a04         	ldr	r2, [sp, #0x10]
 8007bd4: f24b 0574    	movw	r5, #0xb074
 8007bd8: f24b 0c70    	movw	r12, #0xb070
 8007bdc: 9809         	ldr	r0, [sp, #0x24]
 8007bde: f04f 0b00    	mov.w	r11, #0x0
 8007be2: f6cf 71ff    	movt	r1, #0xffff
 8007be6: f2c0 0401    	movt	r4, #0x1
 8007bea: f2c0 0501    	movt	r5, #0x1
 8007bee: f2c0 0c01    	movt	r12, #0x1
 8007bf2: 9001         	str	r0, [sp, #0x4]
 8007bf4: f24b 0384    	movw	r3, #0xb084
 8007bf8: 1850         	adds	r0, r2, r1
 8007bfa: f2c0 0301    	movt	r3, #0x1
 8007bfe: 3184         	adds	r1, #0x84
 8007c00: f850 e003    	ldr.w	lr, [r0, r3]
 8007c04: f24b 0380    	movw	r3, #0xb080
 8007c08: f2c0 0301    	movt	r3, #0x1
 8007c0c: f850 8003    	ldr.w	r8, [r0, r3]
 8007c10: f24b 037c    	movw	r3, #0xb07c
 8007c14: f2c0 0301    	movt	r3, #0x1
 8007c18: f850 9003    	ldr.w	r9, [r0, r3]
 8007c1c: f24b 0378    	movw	r3, #0xb078
 8007c20: f2c0 0301    	movt	r3, #0x1
 8007c24: f850 a003    	ldr.w	r10, [r0, r3]
 8007c28: f24b 0308    	movw	r3, #0xb008
 8007c2c: f2c0 0301    	movt	r3, #0x1
 8007c30: 58c6         	ldr	r6, [r0, r3]
 8007c32: f24b 0304    	movw	r3, #0xb004
 8007c36: f2c0 0301    	movt	r3, #0x1
 8007c3a: 58c3         	ldr	r3, [r0, r3]
 8007c3c: 445b         	add	r3, r11
 8007c3e: 4433         	add	r3, r6
 8007c40: f24b 060c    	movw	r6, #0xb00c
 8007c44: f2c0 0601    	movt	r6, #0x1
 8007c48: 5986         	ldr	r6, [r0, r6]
 8007c4a: 4433         	add	r3, r6
 8007c4c: f24b 0610    	movw	r6, #0xb010
 8007c50: f2c0 0601    	movt	r6, #0x1
 8007c54: 5986         	ldr	r6, [r0, r6]
 8007c56: 4433         	add	r3, r6
 8007c58: f24b 0614    	movw	r6, #0xb014
 8007c5c: f2c0 0601    	movt	r6, #0x1
 8007c60: 5986         	ldr	r6, [r0, r6]
 8007c62: 4433         	add	r3, r6
 8007c64: f24b 0618    	movw	r6, #0xb018
 8007c68: f2c0 0601    	movt	r6, #0x1
 8007c6c: 5986         	ldr	r6, [r0, r6]
 8007c6e: 4433         	add	r3, r6
 8007c70: f24b 061c    	movw	r6, #0xb01c
 8007c74: f2c0 0601    	movt	r6, #0x1
 8007c78: 5986         	ldr	r6, [r0, r6]
 8007c7a: 4433         	add	r3, r6
 8007c7c: f24b 0620    	movw	r6, #0xb020
 8007c80: f2c0 0601    	movt	r6, #0x1
 8007c84: 5986         	ldr	r6, [r0, r6]
 8007c86: 4433         	add	r3, r6
 8007c88: f24b 0624    	movw	r6, #0xb024
 8007c8c: f2c0 0601    	movt	r6, #0x1
 8007c90: 5986         	ldr	r6, [r0, r6]
 8007c92: 4433         	add	r3, r6
 8007c94: f24b 0628    	movw	r6, #0xb028
 8007c98: f2c0 0601    	movt	r6, #0x1
 8007c9c: 5986         	ldr	r6, [r0, r6]
 8007c9e: 4433         	add	r3, r6
 8007ca0: f24b 062c    	movw	r6, #0xb02c
 8007ca4: f2c0 0601    	movt	r6, #0x1
 8007ca8: 5986         	ldr	r6, [r0, r6]
 8007caa: 4433         	add	r3, r6
 8007cac: f24b 0630    	movw	r6, #0xb030
 8007cb0: f2c0 0601    	movt	r6, #0x1
 8007cb4: 5986         	ldr	r6, [r0, r6]
 8007cb6: 4433         	add	r3, r6
 8007cb8: f24b 0634    	movw	r6, #0xb034
 8007cbc: f2c0 0601    	movt	r6, #0x1
 8007cc0: 5986         	ldr	r6, [r0, r6]
 8007cc2: 4433         	add	r3, r6
 8007cc4: f24b 0638    	movw	r6, #0xb038
 8007cc8: f2c0 0601    	movt	r6, #0x1
 8007ccc: 5986         	ldr	r6, [r0, r6]
 8007cce: 4433         	add	r3, r6
 8007cd0: f24b 063c    	movw	r6, #0xb03c
 8007cd4: f2c0 0601    	movt	r6, #0x1
 8007cd8: 5986         	ldr	r6, [r0, r6]
 8007cda: 4433         	add	r3, r6
 8007cdc: f24b 0640    	movw	r6, #0xb040
 8007ce0: f2c0 0601    	movt	r6, #0x1
 8007ce4: 5986         	ldr	r6, [r0, r6]
 8007ce6: 4433         	add	r3, r6
 8007ce8: f24b 0644    	movw	r6, #0xb044
 8007cec: f2c0 0601    	movt	r6, #0x1
 8007cf0: 5986         	ldr	r6, [r0, r6]
 8007cf2: 4433         	add	r3, r6
 8007cf4: f24b 0648    	movw	r6, #0xb048
 8007cf8: f2c0 0601    	movt	r6, #0x1
 8007cfc: 5986         	ldr	r6, [r0, r6]
 8007cfe: 4433         	add	r3, r6
 8007d00: f24b 064c    	movw	r6, #0xb04c
 8007d04: f2c0 0601    	movt	r6, #0x1
 8007d08: 5986         	ldr	r6, [r0, r6]
 8007d0a: 4433         	add	r3, r6
 8007d0c: f24b 0650    	movw	r6, #0xb050
 8007d10: f2c0 0601    	movt	r6, #0x1
 8007d14: 5986         	ldr	r6, [r0, r6]
 8007d16: 4433         	add	r3, r6
 8007d18: f24b 0654    	movw	r6, #0xb054
 8007d1c: f2c0 0601    	movt	r6, #0x1
 8007d20: 5986         	ldr	r6, [r0, r6]
 8007d22: 4433         	add	r3, r6
 8007d24: f24b 0658    	movw	r6, #0xb058
 8007d28: f2c0 0601    	movt	r6, #0x1
 8007d2c: 5986         	ldr	r6, [r0, r6]
 8007d2e: 4433         	add	r3, r6
 8007d30: f24b 065c    	movw	r6, #0xb05c
 8007d34: f2c0 0601    	movt	r6, #0x1
 8007d38: 5986         	ldr	r6, [r0, r6]
 8007d3a: 4433         	add	r3, r6
 8007d3c: f24b 0660    	movw	r6, #0xb060
 8007d40: f2c0 0601    	movt	r6, #0x1
 8007d44: 5986         	ldr	r6, [r0, r6]
 8007d46: 4433         	add	r3, r6
 8007d48: f24b 0664    	movw	r6, #0xb064
 8007d4c: f2c0 0601    	movt	r6, #0x1
 8007d50: 5986         	ldr	r6, [r0, r6]
 8007d52: 4433         	add	r3, r6
 8007d54: f24b 0668    	movw	r6, #0xb068
 8007d58: f2c0 0601    	movt	r6, #0x1
 8007d5c: 5986         	ldr	r6, [r0, r6]
 8007d5e: 4433         	add	r3, r6
 8007d60: 5906         	ldr	r6, [r0, r4]
 8007d62: 4433         	add	r3, r6
 8007d64: 5946         	ldr	r6, [r0, r5]
 8007d66: f850 000c    	ldr.w	r0, [r0, r12]
 8007d6a: 4418         	add	r0, r3
 8007d6c: 4430         	add	r0, r6
 8007d6e: 4450         	add	r0, r10
 8007d70: 4448         	add	r0, r9
 8007d72: 4440         	add	r0, r8
 8007d74: eb0e 0b00    	add.w	r11, lr, r0
 8007d78: f47f af3c    	bne.w	0x8007bf4 <raylar_crcl_encoder_service::benchmark::sum_only::he5272faae351e30d+0x288> @ imm = #-0x188
 8007d7c: f8cd b024    	str.w	r11, [sp, #0x24]
 8007d80: a809         	add	r0, sp, #0x24
 8007d82: f502 31d8    	add.w	r1, r2, #0x1b000
 8007d86: 2400         	movs	r4, #0x0
 8007d88: 9809         	ldr	r0, [sp, #0x24]
 8007d8a: f240 3aff    	movw	r10, #0x3ff
 8007d8e: 9000         	str	r0, [sp]
 8007d90: f851 0f84    	ldr	r0, [r1, #132]!
 8007d94: f1ba 0a21    	subs.w	r10, r10, #0x21
 8007d98: f1a1 0680    	sub.w	r6, r1, #0x80
 8007d9c: 9008         	str	r0, [sp, #0x20]
 8007d9e: e951 5c1d    	ldrd	r5, r12, [r1, #-116]
 8007da2: f1a1 0e60    	sub.w	lr, r1, #0x60
 8007da6: ce49         	ldm	r6, {r0, r3, r6}
 8007da8: 4420         	add	r0, r4
 8007daa: f851 2c04    	ldr	r2, [r1, #-4]
 8007dae: 4418         	add	r0, r3
 8007db0: e951 b81b    	ldrd	r11, r8, [r1, #-108]
 8007db4: 4430         	add	r0, r6
 8007db6: 9207         	str	r2, [sp, #0x1c]
 8007db8: 4428         	add	r0, r5
 8007dba: f851 2c08    	ldr	r2, [r1, #-8]
 8007dbe: 4460         	add	r0, r12
 8007dc0: 9206         	str	r2, [sp, #0x18]
 8007dc2: 4458         	add	r0, r11
 8007dc4: f851 2c0c    	ldr	r2, [r1, #-12]
 8007dc8: f851 9c64    	ldr	r9, [r1, #-100]
 8007dcc: 4440         	add	r0, r8
 8007dce: 9205         	str	r2, [sp, #0x14]
 8007dd0: e89e 586c    	ldm.w	lr, {r2, r3, r5, r6, r11, r12, lr}
 8007dd4: 4448         	add	r0, r9
 8007dd6: 4410         	add	r0, r2
 8007dd8: f851 4c44    	ldr	r4, [r1, #-68]
 8007ddc: 4418         	add	r0, r3
 8007dde: 4428         	add	r0, r5
 8007de0: 4430         	add	r0, r6
 8007de2: f1a1 0640    	sub.w	r6, r1, #0x40
 8007de6: 4458         	add	r0, r11
 8007de8: f1a1 0b2c    	sub.w	r11, r1, #0x2c
 8007dec: 4460         	add	r0, r12
 8007dee: ce4c         	ldm	r6, {r2, r3, r6}
 8007df0: 4470         	add	r0, lr
 8007df2: 4420         	add	r0, r4
 8007df4: e951 5c0d    	ldrd	r5, r12, [r1, #-52]
 8007df8: 4410         	add	r0, r2
 8007dfa: e89b 0b00    	ldm.w	r11, {r8, r9, r11}
 8007dfe: 4418         	add	r0, r3
 8007e00: 4430         	add	r0, r6
 8007e02: f851 2c20    	ldr	r2, [r1, #-32]
 8007e06: 4428         	add	r0, r5
 8007e08: f851 3c1c    	ldr	r3, [r1, #-28]
 8007e0c: 4460         	add	r0, r12
 8007e0e: f851 6c18    	ldr	r6, [r1, #-24]
 8007e12: 4440         	add	r0, r8
 8007e14: 4448         	add	r0, r9
 8007e16: 4458         	add	r0, r11
 8007e18: 4410         	add	r0, r2
 8007e1a: f851 2c14    	ldr	r2, [r1, #-20]
 8007e1e: 4418         	add	r0, r3
 8007e20: f851 3c10    	ldr	r3, [r1, #-16]
 8007e24: 4430         	add	r0, r6
 8007e26: 4410         	add	r0, r2
 8007e28: 9a05         	ldr	r2, [sp, #0x14]
 8007e2a: 4418         	add	r0, r3
 8007e2c: 4410         	add	r0, r2
 8007e2e: 9a06         	ldr	r2, [sp, #0x18]
 8007e30: 4410         	add	r0, r2
 8007e32: 9a07         	ldr	r2, [sp, #0x1c]
 8007e34: 4410         	add	r0, r2
 8007e36: 9a08         	ldr	r2, [sp, #0x20]
 8007e38: eb02 0400    	add.w	r4, r2, r0
 8007e3c: d1a8         	bne	0x8007d90 <raylar_crcl_encoder_service::benchmark::sum_only::he5272faae351e30d+0x424> @ imm = #-0xb0
 8007e3e: 9409         	str	r4, [sp, #0x24]
 8007e40: a809         	add	r0, sp, #0x24
 8007e42: e9dd 1002    	ldrd	r1, r0, [sp, #8]
 8007e46: 9a00         	ldr	r2, [sp]
 8007e48: 4048         	eors	r0, r1
 8007e4a: 9901         	ldr	r1, [sp, #0x4]
 8007e4c: 4048         	eors	r0, r1
 8007e4e: 9909         	ldr	r1, [sp, #0x24]
 8007e50: 4050         	eors	r0, r2
 8007e52: 4048         	eors	r0, r1
 8007e54: b00a         	add	sp, #0x28
 8007e56: e8bd 0f00    	pop.w	{r8, r9, r10, r11}
 8007e5a: bdf0         	pop	{r4, r5, r6, r7, pc}

08007e5c <raylar_crcl_encoder_service::benchmark::temporal::h98ba012ed5b5e4c9>:
 8007e5c: b5f0         	push	{r4, r5, r6, r7, lr}
 8007e5e: af03         	add	r7, sp, #0xc
 8007e60: f84d bd04    	str	r11, [sp, #-4]!
 8007e64: b081         	sub	sp, #0x4
 8007e66: f24c 012c    	movw	r1, #0xc02c
 8007e6a: f500 3290    	add.w	r2, r0, #0x12000
 8007e6e: 5844         	ldr	r4, [r0, r1]
 8007e70: eb00 0e01    	add.w	lr, r0, r1
 8007e74: f500 3190    	add.w	r1, r0, #0x12000
 8007e78: f102 0324    	add.w	r3, r2, #0x24
 8007e7c: f101 0c28    	add.w	r12, r1, #0x28
 8007e80: f240 31ff    	movw	r1, #0x3ff
 8007e84: 4672         	mov	r2, lr
 8007e86: 6995         	ldr	r5, [r2, #0x18]
 8007e88: 390b         	subs	r1, #0xb
 8007e8a: eba5 0404    	sub.w	r4, r5, r4
 8007e8e: 605c         	str	r4, [r3, #0x4]
 8007e90: 6b14         	ldr	r4, [r2, #0x30]
 8007e92: eba4 0505    	sub.w	r5, r4, r5
 8007e96: 609d         	str	r5, [r3, #0x8]
 8007e98: 6c95         	ldr	r5, [r2, #0x48]
 8007e9a: eba5 0404    	sub.w	r4, r5, r4
 8007e9e: 60dc         	str	r4, [r3, #0xc]
 8007ea0: 6e14         	ldr	r4, [r2, #0x60]
 8007ea2: eba4 0505    	sub.w	r5, r4, r5
 8007ea6: 611d         	str	r5, [r3, #0x10]
 8007ea8: 6f95         	ldr	r5, [r2, #0x78]
 8007eaa: eba5 0404    	sub.w	r4, r5, r4
 8007eae: 615c         	str	r4, [r3, #0x14]
 8007eb0: f8d2 4090    	ldr.w	r4, [r2, #0x90]
 8007eb4: eba4 0505    	sub.w	r5, r4, r5
 8007eb8: 619d         	str	r5, [r3, #0x18]
 8007eba: f8d2 50a8    	ldr.w	r5, [r2, #0xa8]
 8007ebe: eba5 0404    	sub.w	r4, r5, r4
 8007ec2: 61dc         	str	r4, [r3, #0x1c]
 8007ec4: f8d2 40c0    	ldr.w	r4, [r2, #0xc0]
 8007ec8: eba4 0505    	sub.w	r5, r4, r5
 8007ecc: 621d         	str	r5, [r3, #0x20]
 8007ece: f8d2 50d8    	ldr.w	r5, [r2, #0xd8]
 8007ed2: eba5 0404    	sub.w	r4, r5, r4
 8007ed6: 625c         	str	r4, [r3, #0x24]
 8007ed8: f8d2 60f0    	ldr.w	r6, [r2, #0xf0]
 8007edc: eba6 0405    	sub.w	r4, r6, r5
 8007ee0: 629c         	str	r4, [r3, #0x28]
 8007ee2: f8d2 4108    	ldr.w	r4, [r2, #0x108]
 8007ee6: f502 7284    	add.w	r2, r2, #0x108
 8007eea: eba4 0506    	sub.w	r5, r4, r6
 8007eee: f843 5f2c    	str	r5, [r3, #44]!
 8007ef2: d1c8         	bne	0x8007e86 <raylar_crcl_encoder_service::benchmark::temporal::h98ba012ed5b5e4c9+0x2a> @ imm = #-0x70
 8007ef4: f500 3190    	add.w	r1, r0, #0x12000
 8007ef8: f240 32ff    	movw	r2, #0x3ff
 8007efc: f501 5381    	add.w	r3, r1, #0x1020
 8007f00: 4671         	mov	r1, lr
 8007f02: f851 4f04    	ldr	r4, [r1, #4]!
 8007f06: 698d         	ldr	r5, [r1, #0x18]
 8007f08: 3a0b         	subs	r2, #0xb
 8007f0a: eba5 0404    	sub.w	r4, r5, r4
 8007f0e: 605c         	str	r4, [r3, #0x4]
 8007f10: 6b0c         	ldr	r4, [r1, #0x30]
 8007f12: eba4 0505    	sub.w	r5, r4, r5
 8007f16: 609d         	str	r5, [r3, #0x8]
 8007f18: 6c8d         	ldr	r5, [r1, #0x48]
 8007f1a: eba5 0404    	sub.w	r4, r5, r4
 8007f1e: 60dc         	str	r4, [r3, #0xc]
 8007f20: 6e0c         	ldr	r4, [r1, #0x60]
 8007f22: eba4 0505    	sub.w	r5, r4, r5
 8007f26: 611d         	str	r5, [r3, #0x10]
 8007f28: 6f8d         	ldr	r5, [r1, #0x78]
 8007f2a: eba5 0404    	sub.w	r4, r5, r4
 8007f2e: 615c         	str	r4, [r3, #0x14]
 8007f30: f8d1 4090    	ldr.w	r4, [r1, #0x90]
 8007f34: eba4 0505    	sub.w	r5, r4, r5
 8007f38: 619d         	str	r5, [r3, #0x18]
 8007f3a: f8d1 50a8    	ldr.w	r5, [r1, #0xa8]
 8007f3e: eba5 0404    	sub.w	r4, r5, r4
 8007f42: 61dc         	str	r4, [r3, #0x1c]
 8007f44: f8d1 40c0    	ldr.w	r4, [r1, #0xc0]
 8007f48: eba4 0505    	sub.w	r5, r4, r5
 8007f4c: 621d         	str	r5, [r3, #0x20]
 8007f4e: f8d1 50d8    	ldr.w	r5, [r1, #0xd8]
 8007f52: eba5 0404    	sub.w	r4, r5, r4
 8007f56: 625c         	str	r4, [r3, #0x24]
 8007f58: f8d1 60f0    	ldr.w	r6, [r1, #0xf0]
 8007f5c: eba6 0405    	sub.w	r4, r6, r5
 8007f60: 629c         	str	r4, [r3, #0x28]
 8007f62: f8d1 4108    	ldr.w	r4, [r1, #0x108]
 8007f66: f501 7184    	add.w	r1, r1, #0x108
 8007f6a: eba4 0506    	sub.w	r5, r4, r6
 8007f6e: f843 5f2c    	str	r5, [r3, #44]!
 8007f72: d1c8         	bne	0x8007f06 <raylar_crcl_encoder_service::benchmark::temporal::h98ba012ed5b5e4c9+0xaa> @ imm = #-0x70
 8007f74: f500 31a0    	add.w	r1, r0, #0x14000
 8007f78: f240 32ff    	movw	r2, #0x3ff
 8007f7c: f101 031c    	add.w	r3, r1, #0x1c
 8007f80: 4671         	mov	r1, lr
 8007f82: f851 4f08    	ldr	r4, [r1, #8]!
 8007f86: 698d         	ldr	r5, [r1, #0x18]
 8007f88: 3a0b         	subs	r2, #0xb
 8007f8a: eba5 0404    	sub.w	r4, r5, r4
 8007f8e: 605c         	str	r4, [r3, #0x4]
 8007f90: 6b0c         	ldr	r4, [r1, #0x30]
 8007f92: eba4 0505    	sub.w	r5, r4, r5
 8007f96: 609d         	str	r5, [r3, #0x8]
 8007f98: 6c8d         	ldr	r5, [r1, #0x48]
 8007f9a: eba5 0404    	sub.w	r4, r5, r4
 8007f9e: 60dc         	str	r4, [r3, #0xc]
 8007fa0: 6e0c         	ldr	r4, [r1, #0x60]
 8007fa2: eba4 0505    	sub.w	r5, r4, r5
 8007fa6: 611d         	str	r5, [r3, #0x10]
 8007fa8: 6f8d         	ldr	r5, [r1, #0x78]
 8007faa: eba5 0404    	sub.w	r4, r5, r4
 8007fae: 615c         	str	r4, [r3, #0x14]
 8007fb0: f8d1 4090    	ldr.w	r4, [r1, #0x90]
 8007fb4: eba4 0505    	sub.w	r5, r4, r5
 8007fb8: 619d         	str	r5, [r3, #0x18]
 8007fba: f8d1 50a8    	ldr.w	r5, [r1, #0xa8]
 8007fbe: eba5 0404    	sub.w	r4, r5, r4
 8007fc2: 61dc         	str	r4, [r3, #0x1c]
 8007fc4: f8d1 40c0    	ldr.w	r4, [r1, #0xc0]
 8007fc8: eba4 0505    	sub.w	r5, r4, r5
 8007fcc: 621d         	str	r5, [r3, #0x20]
 8007fce: f8d1 50d8    	ldr.w	r5, [r1, #0xd8]
 8007fd2: eba5 0404    	sub.w	r4, r5, r4
 8007fd6: 625c         	str	r4, [r3, #0x24]
 8007fd8: f8d1 60f0    	ldr.w	r6, [r1, #0xf0]
 8007fdc: eba6 0405    	sub.w	r4, r6, r5
 8007fe0: 629c         	str	r4, [r3, #0x28]
 8007fe2: f8d1 4108    	ldr.w	r4, [r1, #0x108]
 8007fe6: f501 7184    	add.w	r1, r1, #0x108
 8007fea: eba4 0506    	sub.w	r5, r4, r6
 8007fee: f843 5f2c    	str	r5, [r3, #44]!
 8007ff2: d1c8         	bne	0x8007f86 <raylar_crcl_encoder_service::benchmark::temporal::h98ba012ed5b5e4c9+0x12a> @ imm = #-0x70
 8007ff4: f500 31a8    	add.w	r1, r0, #0x15000
 8007ff8: f240 32ff    	movw	r2, #0x3ff
 8007ffc: f101 0318    	add.w	r3, r1, #0x18
 8008000: 4671         	mov	r1, lr
 8008002: f851 4f0c    	ldr	r4, [r1, #12]!
 8008006: 698d         	ldr	r5, [r1, #0x18]
 8008008: 3a0b         	subs	r2, #0xb
 800800a: eba5 0404    	sub.w	r4, r5, r4
 800800e: 605c         	str	r4, [r3, #0x4]
 8008010: 6b0c         	ldr	r4, [r1, #0x30]
 8008012: eba4 0505    	sub.w	r5, r4, r5
 8008016: 609d         	str	r5, [r3, #0x8]
 8008018: 6c8d         	ldr	r5, [r1, #0x48]
 800801a: eba5 0404    	sub.w	r4, r5, r4
 800801e: 60dc         	str	r4, [r3, #0xc]
 8008020: 6e0c         	ldr	r4, [r1, #0x60]
 8008022: eba4 0505    	sub.w	r5, r4, r5
 8008026: 611d         	str	r5, [r3, #0x10]
 8008028: 6f8d         	ldr	r5, [r1, #0x78]
 800802a: eba5 0404    	sub.w	r4, r5, r4
 800802e: 615c         	str	r4, [r3, #0x14]
 8008030: f8d1 4090    	ldr.w	r4, [r1, #0x90]
 8008034: eba4 0505    	sub.w	r5, r4, r5
 8008038: 619d         	str	r5, [r3, #0x18]
 800803a: f8d1 50a8    	ldr.w	r5, [r1, #0xa8]
 800803e: eba5 0404    	sub.w	r4, r5, r4
 8008042: 61dc         	str	r4, [r3, #0x1c]
 8008044: f8d1 40c0    	ldr.w	r4, [r1, #0xc0]
 8008048: eba4 0505    	sub.w	r5, r4, r5
 800804c: 621d         	str	r5, [r3, #0x20]
 800804e: f8d1 50d8    	ldr.w	r5, [r1, #0xd8]
 8008052: eba5 0404    	sub.w	r4, r5, r4
 8008056: 625c         	str	r4, [r3, #0x24]
 8008058: f8d1 60f0    	ldr.w	r6, [r1, #0xf0]
 800805c: eba6 0405    	sub.w	r4, r6, r5
 8008060: 629c         	str	r4, [r3, #0x28]
 8008062: f8d1 4108    	ldr.w	r4, [r1, #0x108]
 8008066: f501 7184    	add.w	r1, r1, #0x108
 800806a: eba4 0506    	sub.w	r5, r4, r6
 800806e: f843 5f2c    	str	r5, [r3, #44]!
 8008072: d1c8         	bne	0x8008006 <raylar_crcl_encoder_service::benchmark::temporal::h98ba012ed5b5e4c9+0x1aa> @ imm = #-0x70
 8008074: f500 30b0    	add.w	r0, r0, #0x16000
 8008078: f85e 2f10    	ldr	r2, [lr, #16]!
 800807c: 3014         	adds	r0, #0x14
 800807e: f240 31ff    	movw	r1, #0x3ff
 8008082: f8de 3018    	ldr.w	r3, [lr, #0x18]
 8008086: 390b         	subs	r1, #0xb
 8008088: eba3 0202    	sub.w	r2, r3, r2
 800808c: 6042         	str	r2, [r0, #0x4]
 800808e: f8de 2030    	ldr.w	r2, [lr, #0x30]
 8008092: eba2 0303    	sub.w	r3, r2, r3
 8008096: 6083         	str	r3, [r0, #0x8]
 8008098: f8de 3048    	ldr.w	r3, [lr, #0x48]
 800809c: eba3 0202    	sub.w	r2, r3, r2
 80080a0: 60c2         	str	r2, [r0, #0xc]
 80080a2: f8de 2060    	ldr.w	r2, [lr, #0x60]
 80080a6: eba2 0303    	sub.w	r3, r2, r3
 80080aa: 6103         	str	r3, [r0, #0x10]
 80080ac: f8de 3078    	ldr.w	r3, [lr, #0x78]
 80080b0: eba3 0202    	sub.w	r2, r3, r2
 80080b4: 6142         	str	r2, [r0, #0x14]
 80080b6: f8de 2090    	ldr.w	r2, [lr, #0x90]
 80080ba: eba2 0303    	sub.w	r3, r2, r3
 80080be: 6183         	str	r3, [r0, #0x18]
 80080c0: f8de 30a8    	ldr.w	r3, [lr, #0xa8]
 80080c4: eba3 0202    	sub.w	r2, r3, r2
 80080c8: 61c2         	str	r2, [r0, #0x1c]
 80080ca: f8de 20c0    	ldr.w	r2, [lr, #0xc0]
 80080ce: eba2 0303    	sub.w	r3, r2, r3
 80080d2: 6203         	str	r3, [r0, #0x20]
 80080d4: f8de 30d8    	ldr.w	r3, [lr, #0xd8]
 80080d8: eba3 0202    	sub.w	r2, r3, r2
 80080dc: 6242         	str	r2, [r0, #0x24]
 80080de: f8de 40f0    	ldr.w	r4, [lr, #0xf0]
 80080e2: eba4 0203    	sub.w	r2, r4, r3
 80080e6: 6282         	str	r2, [r0, #0x28]
 80080e8: f8de 2108    	ldr.w	r2, [lr, #0x108]
 80080ec: f50e 7e84    	add.w	lr, lr, #0x108
 80080f0: eba2 0304    	sub.w	r3, r2, r4
 80080f4: f840 3f2c    	str	r3, [r0, #44]!
 80080f8: d1c3         	bne	0x8008082 <raylar_crcl_encoder_service::benchmark::temporal::h98ba012ed5b5e4c9+0x226> @ imm = #-0x7a
 80080fa: f8cd c000    	str.w	r12, [sp]
 80080fe: 4668         	mov	r0, sp
 8008100: f8dc 0000    	ldr.w	r0, [r12]
 8008104: b001         	add	sp, #0x4
 8008106: f85d bb04    	ldr	r11, [sp], #4
 800810a: bdf0         	pop	{r4, r5, r6, r7, pc}

0800810c <raylar_crcl_encoder_service::benchmark::pack_copy::ha831ca28ff45cfb7>:
 800810c: b5b0         	push	{r4, r5, r7, lr}
 800810e: af02         	add	r7, sp, #0x8
 8008110: b082         	sub	sp, #0x8
 8008112: 4604         	mov	r4, r0
 8008114: f64f 40d8    	movw	r0, #0xfcd8
 8008118: f2c0 0001    	movt	r0, #0x1
 800811c: 5821         	ldr	r1, [r4, r0]
 800811e: f1a1 0258    	sub.w	r2, r1, #0x58
 8008122: f5b2 5ff0    	cmp.w	r2, #0x1e00
 8008126: d819         	bhi	0x800815c <raylar_crcl_encoder_service::benchmark::pack_copy::ha831ca28ff45cfb7+0x50> @ imm = #0x32
 8008128: f504 31e8    	add.w	r1, r4, #0x1d000
 800812c: 1825         	adds	r5, r4, r0
 800812e: f504 30e0    	add.w	r0, r4, #0x1c000
 8008132: f501 616b    	add.w	r1, r1, #0xeb0
 8008136: 3058         	adds	r0, #0x58
 8008138: f005 f8cd    	bl	0x800d2d6 <__aeabi_memcpy4> @ imm = #0x519a
 800813c: 4620         	mov	r0, r4
 800813e: f7fa fe10    	bl	0x8002d62 <raylar_crcl_encoder_service::benchmark::pack_header::haa3cc0f32481b1e3> @ imm = #-0x53e0
 8008142: 6829         	ldr	r1, [r5]
 8008144: f641 6059    	movw	r0, #0x1e59
 8008148: 4281         	cmp	r1, r0
 800814a: d210         	bhs	0x800816e <raylar_crcl_encoder_service::benchmark::pack_copy::ha831ca28ff45cfb7+0x62> @ imm = #0x20
 800814c: f504 30e0    	add.w	r0, r4, #0x1c000
 8008150: 9000         	str	r0, [sp]
 8008152: 9101         	str	r1, [sp, #0x4]
 8008154: 4668         	mov	r0, sp
 8008156: 6828         	ldr	r0, [r5]
 8008158: b002         	add	sp, #0x8
 800815a: bdb0         	pop	{r4, r5, r7, pc}
 800815c: f64e 03f0    	movw	r3, #0xe8f0
 8008160: 2058         	movs	r0, #0x58
 8008162: f6c0 0300    	movt	r3, #0x800
 8008166: f641 6258    	movw	r2, #0x1e58
 800816a: f004 f9a8    	bl	0x800c4be <core::slice::index::slice_index_fail> @ imm = #0x4350
 800816e: f64e 03e0    	movw	r3, #0xe8e0
 8008172: 2000         	movs	r0, #0x0
 8008174: f6c0 0300    	movt	r3, #0x800
 8008178: f641 6258    	movw	r2, #0x1e58
 800817c: f004 f99f    	bl	0x800c4be <core::slice::index::slice_index_fail> @ imm = #0x433e