08000c36 <raylar_crcl_encoder_service::benchmark::nothing::h153434569c2cec30>:
 8000c36: b580         	push	{r7, lr}
 8000c38: 466f         	mov	r7, sp
 8000c3a: bd80         	pop	{r7, pc}

080012f0 <raylar_crcl_encoder_service::format::Encoder::data::ha65fe528befbf4e3>:
 80012f0: b5f0         	push	{r4, r5, r6, r7, lr}
 80012f2: af03         	add	r7, sp, #0xc
 80012f4: e92d 0f00    	push.w	{r8, r9, r10, r11}
 80012f8: b0a3         	sub	sp, #0x8c
 80012fa: 6f16         	ldr	r6, [r2, #0x70]
 80012fc: 2e06         	cmp	r6, #0x6
 80012fe: d137         	bne	0x8001370 <raylar_crcl_encoder_service::format::Encoder::data::ha65fe528befbf4e3+0x80> @ imm = #0x6e
 8001300: f892 908c    	ldrb.w	r9, [r2, #0x8c]
 8001304: 7e0e         	ldrb	r6, [r1, #0x18]
 8001306: 45b1         	cmp	r9, r6
 8001308: d132         	bne	0x8001370 <raylar_crcl_encoder_service::format::Encoder::data::ha65fe528befbf4e3+0x80> @ imm = #0x64
 800130a: 7e4e         	ldrb	r6, [r1, #0x19]
 800130c: f892 508d    	ldrb.w	r5, [r2, #0x8d]
 8001310: 42b5         	cmp	r5, r6
 8001312: d12d         	bne	0x8001370 <raylar_crcl_encoder_service::format::Encoder::data::ha65fe528befbf4e3+0x80> @ imm = #0x5a
 8001314: 694e         	ldr	r6, [r1, #0x14]
 8001316: f8d2 5088    	ldr.w	r5, [r2, #0x88]
 800131a: 42b5         	cmp	r5, r6
 800131c: d128         	bne	0x8001370 <raylar_crcl_encoder_service::format::Encoder::data::ha65fe528befbf4e3+0x80> @ imm = #0x50
 800131e: f892 508f    	ldrb.w	r5, [r2, #0x8f]
 8001322: 7ece         	ldrb	r6, [r1, #0x1b]
 8001324: 42b5         	cmp	r5, r6
 8001326: d123         	bne	0x8001370 <raylar_crcl_encoder_service::format::Encoder::data::ha65fe528befbf4e3+0x80> @ imm = #0x46
 8001328: 690e         	ldr	r6, [r1, #0x10]
 800132a: f8d2 4084    	ldr.w	r4, [r2, #0x84]
 800132e: 42b4         	cmp	r4, r6
 8001330: d11e         	bne	0x8001370 <raylar_crcl_encoder_service::format::Encoder::data::ha65fe528befbf4e3+0x80> @ imm = #0x3c
 8001332: 7e8e         	ldrb	r6, [r1, #0x1a]
 8001334: f892 408e    	ldrb.w	r4, [r2, #0x8e]
 8001338: 42b4         	cmp	r4, r6
 800133a: d119         	bne	0x8001370 <raylar_crcl_encoder_service::format::Encoder::data::ha65fe528befbf4e3+0x80> @ imm = #0x32
 800133c: 6c56         	ldr	r6, [r2, #0x44]
 800133e: f64a 2aab    	movw	r10, #0xaaab
 8001342: f6ca 2aaa    	movt	r10, #0xaaaa
 8001346: 9620         	str	r6, [sp, #0x80]
 8001348: f8d7 b008    	ldr.w	r11, [r7, #0x8]
 800134c: fba6 640a    	umull	r6, r4, r6, r10
 8001350: ea4f 0494    	lsr.w	r4, r4, #0x2
 8001354: 9421         	str	r4, [sp, #0x84]
 8001356: b195         	cbz	r5, 0x800137e <raylar_crcl_encoder_service::format::Encoder::data::ha65fe528befbf4e3+0x8e> @ imm = #0x24
 8001358: 4626         	mov	r6, r4
 800135a: 2c00         	cmp	r4, #0x0
 800135c: bf18         	it	ne
 800135e: 2601         	movne	r6, #0x1
 8001360: f04f 0e58    	mov.w	lr, #0x58
 8001364: 1ba6         	subs	r6, r4, r6
 8001366: f109 0402    	add.w	r4, r9, #0x2
 800136a: fb04 9806    	mla	r8, r4, r6, r9
 800136e: e00a         	b	0x8001386 <raylar_crcl_encoder_service::format::Encoder::data::ha65fe528befbf4e3+0x96> @ imm = #0x14
 8001370: f240 5101    	movw	r1, #0x501
 8001374: 8001         	strh	r1, [r0]
 8001376: b023         	add	sp, #0x8c
 8001378: e8bd 0f00    	pop.w	{r8, r9, r10, r11}
 800137c: bdf0         	pop	{r4, r5, r6, r7, pc}
 800137e: fb04 f809    	mul	r8, r4, r9
 8001382: f04f 0e60    	mov.w	lr, #0x60
 8001386: ea4f 06d8    	lsr.w	r6, r8, #0x3
 800138a: f04f 0c01    	mov.w	r12, #0x1
 800138e: ea5f 7448    	lsls.w	r4, r8, #0x1d
 8001392: bf18         	it	ne
 8001394: eb0c 06d8    	addne.w	r6, r12, r8, lsr #3
 8001398: eb06 0686    	add.w	r6, r6, r6, lsl #2
 800139c: 4476         	add	r6, lr
 800139e: 45b3         	cmp	r11, r6
 80013a0: d206         	bhs	0x80013b0 <raylar_crcl_encoder_service::format::Encoder::data::ha65fe528befbf4e3+0xc0> @ imm = #0xc
 80013a2: f240 2101    	movw	r1, #0x201
 80013a6: 8001         	strh	r1, [r0]
 80013a8: b023         	add	sp, #0x8c
 80013aa: e8bd 0f00    	pop.w	{r8, r9, r10, r11}
 80013ae: bdf0         	pop	{r4, r5, r6, r7, pc}
 80013b0: 688c         	ldr	r4, [r1, #0x8]
 80013b2: 1c66         	adds	r6, r4, #0x1
 80013b4: bf1c         	itt	ne
 80013b6: 68ce         	ldrne	r6, [r1, #0xc]
 80013b8: f116 0601    	addsne.w	r6, r6, #0x1
 80013bc: d106         	bne	0x80013cc <raylar_crcl_encoder_service::format::Encoder::data::ha65fe528befbf4e3+0xdc> @ imm = #0xc
 80013be: f240 4101    	movw	r1, #0x401
 80013c2: 8001         	strh	r1, [r0]
 80013c4: b023         	add	sp, #0x8c
 80013c6: e8bd 0f00    	pop.w	{r8, r9, r10, r11}
 80013ca: bdf0         	pop	{r4, r5, r6, r7, pc}
 80013cc: e9d1 ec00    	ldrd	lr, r12, [r1]
 80013d0: 2100         	movs	r1, #0x0
 80013d2: 9e21         	ldr	r6, [sp, #0x84]
 80013d4: eb16 060e    	adds.w	r6, r6, lr
 80013d8: f15c 0600    	adcs	r6, r12, #0x0
 80013dc: f151 0100    	adcs	r1, r1, #0x0
 80013e0: d1ed         	bne	0x80013be <raylar_crcl_encoder_service::format::Encoder::data::ha65fe528befbf4e3+0xce> @ imm = #-0x26
 80013e2: 4656         	mov	r6, r10
 80013e4: f04f 0a5c    	mov.w	r10, #0x5c
 80013e8: 2d00         	cmp	r5, #0x0
 80013ea: e9cd ec08    	strd	lr, r12, [sp, #32]
 80013ee: 940a         	str	r4, [sp, #0x28]
 80013f0: 9013         	str	r0, [sp, #0x4c]
 80013f2: 9507         	str	r5, [sp, #0x1c]
 80013f4: bf18         	it	ne
 80013f6: f04f 0a58    	movne.w	r10, #0x58
 80013fa: 45da         	cmp	r10, r11
 80013fc: f200 820a    	bhi.w	0x8001814 <raylar_crcl_encoder_service::format::Encoder::data::ha65fe528befbf4e3+0x524> @ imm = #0x414
 8001400: 6c10         	ldr	r0, [r2, #0x40]
 8001402: 4651         	mov	r1, r10
 8001404: 901d         	str	r0, [sp, #0x74]
 8001406: 4618         	mov	r0, r3
 8001408: e9cd 2317    	strd	r2, r3, [sp, #92]
 800140c: f00b fab9    	bl	0x800c982 <__aeabi_memclr> @ imm = #0xb572
 8001410: f1ab 001c    	sub.w	r0, r11, #0x1c
 8001414: 9b18         	ldr	r3, [sp, #0x60]
 8001416: f04f 0c48    	mov.w	r12, #0x48
 800141a: 2500         	movs	r5, #0x0
 800141c: fba0 0106    	umull	r0, r1, r0, r6
 8001420: f1ab 0018    	sub.w	r0, r11, #0x18
 8001424: fba0 0206    	umull	r0, r2, r0, r6
 8001428: 08c8         	lsrs	r0, r1, #0x3
 800142a: 9014         	str	r0, [sp, #0x50]
 800142c: 08d0         	lsrs	r0, r2, #0x3
 800142e: 9016         	str	r0, [sp, #0x58]
 8001430: f1c9 0020    	rsb.w	r0, r9, #0x20
 8001434: 9010         	str	r0, [sp, #0x40]
 8001436: 9817         	ldr	r0, [sp, #0x5c]
 8001438: f100 0148    	add.w	r1, r0, #0x48
 800143c: 201c         	movs	r0, #0x1c
 800143e: eb05 0245    	add.w	r2, r5, r5, lsl #1
 8001442: eb00 0082    	add.w	r0, r0, r2, lsl #2
 8001446: 4558         	cmp	r0, r11
 8001448: f080 81f4    	bhs.w	0x8001834 <raylar_crcl_encoder_service::format::Encoder::data::ha65fe528befbf4e3+0x544> @ imm = #0x3e8
 800144c: f891 8005    	ldrb.w	r8, [r1, #0x5]
 8001450: 0096         	lsls	r6, r2, #0x2
 8001452: 1caa         	adds	r2, r5, #0x2
 8001454: 541a         	strb	r2, [r3, r0]
 8001456: f106 001d    	add.w	r0, r6, #0x1d
 800145a: 4558         	cmp	r0, r11
 800145c: f080 81f1    	bhs.w	0x8001842 <raylar_crcl_encoder_service::format::Encoder::data::ha65fe528befbf4e3+0x552> @ imm = #0x3e2
 8001460: f803 8000    	strb.w	r8, [r3, r0]
 8001464: f106 001f    	add.w	r0, r6, #0x1f
 8001468: 4558         	cmp	r0, r11
 800146a: f8cd c064    	str.w	r12, [sp, #0x64]
 800146e: f080 81ef    	bhs.w	0x8001850 <raylar_crcl_encoder_service::format::Encoder::data::ha65fe528befbf4e3+0x560> @ imm = #0x3de
 8001472: 790a         	ldrb	r2, [r1, #0x4]
 8001474: f8dd e080    	ldr.w	lr, [sp, #0x80]
 8001478: 541a         	strb	r2, [r3, r0]
 800147a: f106 0020    	add.w	r0, r6, #0x20
 800147e: 921b         	str	r2, [sp, #0x6c]
 8001480: 9a16         	ldr	r2, [sp, #0x58]
 8001482: 4295         	cmp	r5, r2
 8001484: f000 8195    	beq.w	0x80017b2 <raylar_crcl_encoder_service::format::Encoder::data::ha65fe528befbf4e3+0x4c2> @ imm = #0x32a
 8001488: 6809         	ldr	r1, [r1]
 800148a: 45d3         	cmp	r11, r10
 800148c: 5019         	str	r1, [r3, r0]
 800148e: f0c0 81b8    	blo.w	0x8001802 <raylar_crcl_encoder_service::format::Encoder::data::ha65fe528befbf4e3+0x512> @ imm = #0x370
 8001492: ebab 0c0a    	sub.w	r12, r11, r10
 8001496: 4453         	add	r3, r10
 8001498: 2000         	movs	r0, #0x0
 800149a: ebb0 7f5c    	cmp.w	r0, r12, lsr #29
 800149e: bf14         	ite	ne
 80014a0: f04f 31ff    	movne.w	r1, #0xffffffff
 80014a4: ea4f 01cc    	lsleq.w	r1, r12, #0x3
 80014a8: 1c68         	adds	r0, r5, #0x1
 80014aa: f1be 0f06    	cmp.w	lr, #0x6
 80014ae: 901c         	str	r0, [sp, #0x70]
 80014b0: d235         	bhs	0x800151e <raylar_crcl_encoder_service::format::Encoder::data::ha65fe528befbf4e3+0x22e> @ imm = #0x6a
 80014b2: 2000         	movs	r0, #0x0
 80014b4: 2100         	movs	r1, #0x0
 80014b6: f04f 0b00    	mov.w	r11, #0x0
 80014ba: ea5f 724b    	lsls.w	r2, r11, #0x1d
 80014be: ea4f 04db    	lsr.w	r4, r11, #0x3
 80014c2: f04f 0201    	mov.w	r2, #0x1
 80014c6: bf18         	it	ne
 80014c8: eb02 04db    	addne.w	r4, r2, r11, lsr #3
 80014cc: eb14 0801    	adds.w	r8, r4, r1
 80014d0: f080 817c    	bhs.w	0x80017cc <raylar_crcl_encoder_service::format::Encoder::data::ha65fe528befbf4e3+0x4dc> @ imm = #0x2f8
 80014d4: 45e0         	cmp	r8, r12
 80014d6: f200 8179    	bhi.w	0x80017cc <raylar_crcl_encoder_service::format::Encoder::data::ha65fe528befbf4e3+0x4dc> @ imm = #0x2f2
 80014da: f8d7 b008    	ldr.w	r11, [r7, #0x8]
 80014de: 2c05         	cmp	r4, #0x5
 80014e0: 9022         	str	r0, [sp, #0x88]
 80014e2: f080 817c    	bhs.w	0x80017de <raylar_crcl_encoder_service::format::Encoder::data::ha65fe528befbf4e3+0x4ee> @ imm = #0x2f8
 80014e6: 1858         	adds	r0, r3, r1
 80014e8: a922         	add	r1, sp, #0x88
 80014ea: 4622         	mov	r2, r4
 80014ec: f00b f8d6    	bl	0x800c69c <__aeabi_memcpy> @ imm = #0xb1ac
 80014f0: 9a19         	ldr	r2, [sp, #0x64]
 80014f2: f106 0024    	add.w	r0, r6, #0x24
 80014f6: 9914         	ldr	r1, [sp, #0x50]
 80014f8: 428d         	cmp	r5, r1
 80014fa: f000 8179    	beq.w	0x80017f0 <raylar_crcl_encoder_service::format::Encoder::data::ha65fe528befbf4e3+0x500> @ imm = #0x2f2
 80014fe: 9b18         	ldr	r3, [sp, #0x60]
 8001500: f102 0c08    	add.w	r12, r2, #0x8
 8001504: 9d1c         	ldr	r5, [sp, #0x70]
 8001506: 44c2         	add	r10, r8
 8001508: f1bc 0f70    	cmp.w	r12, #0x70
 800150c: f843 8000    	str.w	r8, [r3, r0]
 8001510: 9817         	ldr	r0, [sp, #0x5c]
 8001512: eb00 010c    	add.w	r1, r0, r12
 8001516: f04f 001c    	mov.w	r0, #0x1c
 800151a: d190         	bne	0x800143e <raylar_crcl_encoder_service::format::Encoder::data::ha65fe528befbf4e3+0x14e> @ imm = #-0xe0
 800151c: e12b         	b	0x8001776 <raylar_crcl_encoder_service::format::Encoder::data::ha65fe528befbf4e3+0x486> @ imm = #0x256
 800151e: e9cd c11e    	strd	r12, r1, [sp, #120]
 8001522: 2001         	movs	r0, #0x1
 8001524: 991b         	ldr	r1, [sp, #0x6c]
 8001526: f04f 0c00    	mov.w	r12, #0x0
 800152a: f8cd a034    	str.w	r10, [sp, #0x34]
 800152e: f04f 0b00    	mov.w	r11, #0x0
 8001532: f8dd a070    	ldr.w	r10, [sp, #0x70]
 8001536: 4088         	lsls	r0, r1
 8001538: 9315         	str	r3, [sp, #0x54]
 800153a: 9012         	str	r0, [sp, #0x48]
 800153c: 3801         	subs	r0, #0x1
 800153e: 9011         	str	r0, [sp, #0x44]
 8001540: f1c1 001f    	rsb.w	r0, r1, #0x1f
 8001544: e9cd 650b    	strd	r6, r5, [sp, #44]
 8001548: 2300         	movs	r3, #0x0
 800154a: 900f         	str	r0, [sp, #0x3c]
 800154c: 1c48         	adds	r0, r1, #0x1
 800154e: 2100         	movs	r1, #0x0
 8001550: 2500         	movs	r5, #0x0
 8001552: 901a         	str	r0, [sp, #0x68]
 8001554: e005         	b	0x8001562 <raylar_crcl_encoder_service::format::Encoder::data::ha65fe528befbf4e3+0x272> @ imm = #0xa
 8001556: 44cb         	add	r11, r9
 8001558: 9821         	ldr	r0, [sp, #0x84]
 800155a: 4623         	mov	r3, r4
 800155c: 4284         	cmp	r4, r0
 800155e: f000 80e0    	beq.w	0x8001722 <raylar_crcl_encoder_service::format::Encoder::data::ha65fe528befbf4e3+0x432> @ imm = #0x1c0
 8001562: eb03 0043    	add.w	r0, r3, r3, lsl #1
 8001566: eb0a 0040    	add.w	r0, r10, r0, lsl #1
 800156a: 4570         	cmp	r0, lr
 800156c: f080 815b    	bhs.w	0x8001826 <raylar_crcl_encoder_service::format::Encoder::data::ha65fe528befbf4e3+0x536> @ imm = #0x2b6
 8001570: 9a1d         	ldr	r2, [sp, #0x74]
 8001572: 4666         	mov	r6, r12
 8001574: 1c5c         	adds	r4, r3, #0x1
 8001576: f852 c020    	ldr.w	r12, [r2, r0, lsl #2]
 800157a: 4660         	mov	r0, r12
 800157c: e8df f008    	tbb	[pc, r8]
 8001580: 02 06 03 37  	.word	0x37030602
 8001584: e7e8         	b	0x8001558 <raylar_crcl_encoder_service::format::Encoder::data::ha65fe528befbf4e3+0x268> @ imm = #-0x30
 8001586: b393         	cbz	r3, 0x80015ee <raylar_crcl_encoder_service::format::Encoder::data::ha65fe528befbf4e3+0x2fe> @ imm = #0x64
 8001588: ebac 0006    	sub.w	r0, r12, r6
 800158c: 9b1b         	ldr	r3, [sp, #0x6c]
 800158e: 2b1f         	cmp	r3, #0x1f
 8001590: f200 80e0    	bhi.w	0x8001754 <raylar_crcl_encoder_service::format::Encoder::data::ha65fe528befbf4e3+0x464> @ imm = #0x1c0
 8001594: 0042         	lsls	r2, r0, #0x1
 8001596: ea82 7ae0    	eor.w	r10, r2, r0, asr #31
 800159a: 981a         	ldr	r0, [sp, #0x68]
 800159c: fa2a f603    	lsr.w	r6, r10, r3
 80015a0: 2302         	movs	r3, #0x2
 80015a2: 1980         	adds	r0, r0, r6
 80015a4: f080 80d7    	bhs.w	0x8001756 <raylar_crcl_encoder_service::format::Encoder::data::ha65fe528befbf4e3+0x466> @ imm = #0x1ae
 80015a8: 9a1f         	ldr	r2, [sp, #0x7c]
 80015aa: 4282         	cmp	r2, r0
 80015ac: f0c0 80d3    	blo.w	0x8001756 <raylar_crcl_encoder_service::format::Encoder::data::ha65fe528befbf4e3+0x466> @ imm = #0x1a6
 80015b0: 1a12         	subs	r2, r2, r0
 80015b2: 921f         	str	r2, [sp, #0x7c]
 80015b4: 9a11         	ldr	r2, [sp, #0x44]
 80015b6: 2821         	cmp	r0, #0x21
 80015b8: 9b12         	ldr	r3, [sp, #0x48]
 80015ba: ea02 020a    	and.w	r2, r2, r10
 80015be: ea43 0302    	orr.w	r3, r3, r2
 80015c2: d244         	bhs	0x800164e <raylar_crcl_encoder_service::format::Encoder::data::ha65fe528befbf4e3+0x35e> @ imm = #0x88
 80015c4: f8dd e080    	ldr.w	lr, [sp, #0x80]
 80015c8: 2800         	cmp	r0, #0x0
 80015ca: f8dd a070    	ldr.w	r10, [sp, #0x70]
 80015ce: d0c3         	beq	0x8001558 <raylar_crcl_encoder_service::format::Encoder::data::ha65fe528befbf4e3+0x268> @ imm = #-0x7a
 80015d0: f1c0 0220    	rsb.w	r2, r0, #0x20
 80015d4: 930e         	str	r3, [sp, #0x38]
 80015d6: fa03 f202    	lsl.w	r2, r3, r2
 80015da: f00b 031f    	and	r3, r11, #0x1f
 80015de: 40da         	lsrs	r2, r3
 80015e0: f1cb 0320    	rsb.w	r3, r11, #0x20
 80015e4: 4315         	orrs	r5, r2
 80015e6: 4298         	cmp	r0, r3
 80015e8: d255         	bhs	0x8001696 <raylar_crcl_encoder_service::format::Encoder::data::ha65fe528befbf4e3+0x3a6> @ imm = #0xaa
 80015ea: 4483         	add	r11, r0
 80015ec: e7b4         	b	0x8001558 <raylar_crcl_encoder_service::format::Encoder::data::ha65fe528befbf4e3+0x268> @ imm = #-0x98
 80015ee: 981f         	ldr	r0, [sp, #0x7c]
 80015f0: f1b9 0f20    	cmp.w	r9, #0x20
 80015f4: f200 80b7    	bhi.w	0x8001766 <raylar_crcl_encoder_service::format::Encoder::data::ha65fe528befbf4e3+0x476> @ imm = #0x16e
 80015f8: 4548         	cmp	r0, r9
 80015fa: f0c0 80b4    	blo.w	0x8001766 <raylar_crcl_encoder_service::format::Encoder::data::ha65fe528befbf4e3+0x476> @ imm = #0x168
 80015fe: eba0 0009    	sub.w	r0, r0, r9
 8001602: f1b9 0f00    	cmp.w	r9, #0x0
 8001606: 901f         	str	r0, [sp, #0x7c]
 8001608: d0a6         	beq	0x8001558 <raylar_crcl_encoder_service::format::Encoder::data::ha65fe528befbf4e3+0x268> @ imm = #-0xb4
 800160a: 9810         	ldr	r0, [sp, #0x40]
 800160c: f00b 021f    	and	r2, r11, #0x1f
 8001610: fa0c f000    	lsl.w	r0, r12, r0
 8001614: 40d0         	lsrs	r0, r2
 8001616: 4305         	orrs	r5, r0
 8001618: f1cb 0020    	rsb.w	r0, r11, #0x20
 800161c: 4581         	cmp	r9, r0
 800161e: d39a         	blo	0x8001556 <raylar_crcl_encoder_service::format::Encoder::data::ha65fe528befbf4e3+0x266> @ imm = #-0xcc
 8001620: 9b15         	ldr	r3, [sp, #0x54]
 8001622: 1d0e         	adds	r6, r1, #0x4
 8001624: f111 0f05    	cmn.w	r1, #0x5
 8001628: bf9c         	itt	ls
 800162a: 9a1e         	ldrls	r2, [sp, #0x78]
 800162c: 4296         	cmpls	r6, r2
 800162e: f200 8088    	bhi.w	0x8001742 <raylar_crcl_encoder_service::format::Encoder::data::ha65fe528befbf4e3+0x452> @ imm = #0x110
 8001632: eba9 0b00    	sub.w	r11, r9, r0
 8001636: ba2a         	rev	r2, r5
 8001638: 505a         	str	r2, [r3, r1]
 800163a: f1cb 0100    	rsb.w	r1, r11, #0x0
 800163e: ebb9 0500    	subs.w	r5, r9, r0
 8001642: f001 011f    	and	r1, r1, #0x1f
 8001646: bf18         	it	ne
 8001648: fa0c f501    	lslne.w	r5, r12, r1
 800164c: e067         	b	0x800171e <raylar_crcl_encoder_service::format::Encoder::data::ha65fe528befbf4e3+0x42e> @ imm = #0xce
 800164e: 930e         	str	r3, [sp, #0x38]
 8001650: 4608         	mov	r0, r1
 8001652: f8dd e080    	ldr.w	lr, [sp, #0x80]
 8001656: 2e20         	cmp	r6, #0x20
 8001658: f8dd a070    	ldr.w	r10, [sp, #0x70]
 800165c: 9b15         	ldr	r3, [sp, #0x54]
 800165e: d313         	blo	0x8001688 <raylar_crcl_encoder_service::format::Encoder::data::ha65fe528befbf4e3+0x398> @ imm = #0x26
 8001660: f1bb 0f20    	cmp.w	r11, #0x20
 8001664: d902         	bls	0x800166c <raylar_crcl_encoder_service::format::Encoder::data::ha65fe528befbf4e3+0x37c> @ imm = #0x4
 8001666: f10b 0b20    	add.w	r11, r11, #0x20
 800166a: e00a         	b	0x8001682 <raylar_crcl_encoder_service::format::Encoder::data::ha65fe528befbf4e3+0x392> @ imm = #0x14
 800166c: 1d01         	adds	r1, r0, #0x4
 800166e: f110 0f05    	cmn.w	r0, #0x5
 8001672: bf9c         	itt	ls
 8001674: 9a1e         	ldrls	r2, [sp, #0x78]
 8001676: 4291         	cmpls	r1, r2
 8001678: d85c         	bhi	0x8001734 <raylar_crcl_encoder_service::format::Encoder::data::ha65fe528befbf4e3+0x444> @ imm = #0xb8
 800167a: ba2a         	rev	r2, r5
 800167c: 501a         	str	r2, [r3, r0]
 800167e: 2500         	movs	r5, #0x0
 8001680: 4608         	mov	r0, r1
 8001682: 3e20         	subs	r6, #0x20
 8001684: 2e1f         	cmp	r6, #0x1f
 8001686: d8eb         	bhi	0x8001660 <raylar_crcl_encoder_service::format::Encoder::data::ha65fe528befbf4e3+0x370> @ imm = #-0x2a
 8001688: b32e         	cbz	r6, 0x80016d6 <raylar_crcl_encoder_service::format::Encoder::data::ha65fe528befbf4e3+0x3e6> @ imm = #0x4a
 800168a: f1cb 0020    	rsb.w	r0, r11, #0x20
 800168e: 4286         	cmp	r6, r0
 8001690: d213         	bhs	0x80016ba <raylar_crcl_encoder_service::format::Encoder::data::ha65fe528befbf4e3+0x3ca> @ imm = #0x26
 8001692: 44b3         	add	r11, r6
 8001694: e01f         	b	0x80016d6 <raylar_crcl_encoder_service::format::Encoder::data::ha65fe528befbf4e3+0x3e6> @ imm = #0x3e
 8001696: 1d0e         	adds	r6, r1, #0x4
 8001698: f111 0f05    	cmn.w	r1, #0x5
 800169c: bf9c         	itt	ls
 800169e: 9a1e         	ldrls	r2, [sp, #0x78]
 80016a0: 4296         	cmpls	r6, r2
 80016a2: d84e         	bhi	0x8001742 <raylar_crcl_encoder_service::format::Encoder::data::ha65fe528befbf4e3+0x452> @ imm = #0x9c
 80016a4: ba2a         	rev	r2, r5
 80016a6: 9d15         	ldr	r5, [sp, #0x54]
 80016a8: eba0 0b03    	sub.w	r11, r0, r3
 80016ac: 506a         	str	r2, [r5, r1]
 80016ae: f1cb 0100    	rsb.w	r1, r11, #0x0
 80016b2: f001 011f    	and	r1, r1, #0x1f
 80016b6: 1ac5         	subs	r5, r0, r3
 80016b8: e02d         	b	0x8001716 <raylar_crcl_encoder_service::format::Encoder::data::ha65fe528befbf4e3+0x426> @ imm = #0x5a
 80016ba: f101 0b04    	add.w	r11, r1, #0x4
 80016be: f111 0f05    	cmn.w	r1, #0x5
 80016c2: bf9c         	itt	ls
 80016c4: 9a1e         	ldrls	r2, [sp, #0x78]
 80016c6: 4593         	cmpls	r11, r2
 80016c8: d86a         	bhi	0x80017a0 <raylar_crcl_encoder_service::format::Encoder::data::ha65fe528befbf4e3+0x4b0> @ imm = #0xd4
 80016ca: ba2a         	rev	r2, r5
 80016cc: 505a         	str	r2, [r3, r1]
 80016ce: 4659         	mov	r1, r11
 80016d0: eba6 0b00    	sub.w	r11, r6, r0
 80016d4: 2500         	movs	r5, #0x0
 80016d6: e9dd 200e    	ldrd	r2, r0, [sp, #56]
 80016da: fa02 f000    	lsl.w	r0, r2, r0
 80016de: f00b 021f    	and	r2, r11, #0x1f
 80016e2: 40d0         	lsrs	r0, r2
 80016e4: 9a1a         	ldr	r2, [sp, #0x68]
 80016e6: 4305         	orrs	r5, r0
 80016e8: f1cb 0020    	rsb.w	r0, r11, #0x20
 80016ec: 4282         	cmp	r2, r0
 80016ee: d201         	bhs	0x80016f4 <raylar_crcl_encoder_service::format::Encoder::data::ha65fe528befbf4e3+0x404> @ imm = #0x2
 80016f0: 4493         	add	r11, r2
 80016f2: e731         	b	0x8001558 <raylar_crcl_encoder_service::format::Encoder::data::ha65fe528befbf4e3+0x268> @ imm = #-0x19e
 80016f4: 1d0e         	adds	r6, r1, #0x4
 80016f6: f111 0f05    	cmn.w	r1, #0x5
 80016fa: bf9c         	itt	ls
 80016fc: 9a1e         	ldrls	r2, [sp, #0x78]
 80016fe: 4296         	cmpls	r6, r2
 8001700: d81f         	bhi	0x8001742 <raylar_crcl_encoder_service::format::Encoder::data::ha65fe528befbf4e3+0x452> @ imm = #0x3e
 8001702: ba2a         	rev	r2, r5
 8001704: 505a         	str	r2, [r3, r1]
 8001706: 9a1a         	ldr	r2, [sp, #0x68]
 8001708: eba2 0b00    	sub.w	r11, r2, r0
 800170c: 1a15         	subs	r5, r2, r0
 800170e: f1cb 0100    	rsb.w	r1, r11, #0x0
 8001712: f001 011f    	and	r1, r1, #0x1f
 8001716: 980e         	ldr	r0, [sp, #0x38]
 8001718: bf18         	it	ne
 800171a: fa00 f501    	lslne.w	r5, r0, r1
 800171e: 4631         	mov	r1, r6
 8001720: e71a         	b	0x8001558 <raylar_crcl_encoder_service::format::Encoder::data::ha65fe528befbf4e3+0x268> @ imm = #-0x1cc
 8001722: ba28         	rev	r0, r5
 8001724: 9b15         	ldr	r3, [sp, #0x54]
 8001726: f8dd a034    	ldr.w	r10, [sp, #0x34]
 800172a: e9dd 650b    	ldrd	r6, r5, [sp, #44]
 800172e: f8dd c078    	ldr.w	r12, [sp, #0x78]
 8001732: e6c2         	b	0x80014ba <raylar_crcl_encoder_service::format::Encoder::data::ha65fe528befbf4e3+0x1ca> @ imm = #-0x27c
 8001734: 9a1e         	ldr	r2, [sp, #0x78]
 8001736: f24d 63a0    	movw	r3, #0xd6a0
 800173a: f6c0 0300    	movt	r3, #0x800
 800173e: f00a fa7c    	bl	0x800bc3a <core::slice::index::slice_index_fail> @ imm = #0xa4f8
 8001742: 9a1e         	ldr	r2, [sp, #0x78]
 8001744: f24d 63a0    	movw	r3, #0xd6a0
 8001748: 4608         	mov	r0, r1
 800174a: f6c0 0300    	movt	r3, #0x800
 800174e: 4631         	mov	r1, r6
 8001750: f00a fa73    	bl	0x800bc3a <core::slice::index::slice_index_fail> @ imm = #0xa4e6
 8001754: 2300         	movs	r3, #0x0
 8001756: 9913         	ldr	r1, [sp, #0x4c]
 8001758: 2001         	movs	r0, #0x1
 800175a: 7008         	strb	r0, [r1]
 800175c: 704b         	strb	r3, [r1, #0x1]
 800175e: b023         	add	sp, #0x8c
 8001760: e8bd 0f00    	pop.w	{r8, r9, r10, r11}
 8001764: bdf0         	pop	{r4, r5, r6, r7, pc}
 8001766: 9913         	ldr	r1, [sp, #0x4c]
 8001768: f240 2001    	movw	r0, #0x201
 800176c: 8008         	strh	r0, [r1]
 800176e: b023         	add	sp, #0x8c
 8001770: e8bd 0f00    	pop.w	{r8, r9, r10, r11}
 8001774: bdf0         	pop	{r4, r5, r6, r7, pc}
 8001776: 2001         	movs	r0, #0x1
 8001778: 9907         	ldr	r1, [sp, #0x1c]
 800177a: 9000         	str	r0, [sp]
 800177c: 461a         	mov	r2, r3
 800177e: 980a         	ldr	r0, [sp, #0x28]
 8001780: 465b         	mov	r3, r11
 8001782: 9001         	str	r0, [sp, #0x4]
 8001784: 9808         	ldr	r0, [sp, #0x20]
 8001786: 9002         	str	r0, [sp, #0x8]
 8001788: 9809         	ldr	r0, [sp, #0x24]
 800178a: 9003         	str	r0, [sp, #0xc]
 800178c: 9821         	ldr	r0, [sp, #0x84]
 800178e: e9cd 0a04    	strd	r0, r10, [sp, #16]
 8001792: 9813         	ldr	r0, [sp, #0x4c]
 8001794: f7ff fc95    	bl	0x80010c2 <raylar_crcl_encoder_service::format::finish_record::hdf4f4fad6aa1e26d> @ imm = #-0x6d6
 8001798: b023         	add	sp, #0x8c
 800179a: e8bd 0f00    	pop.w	{r8, r9, r10, r11}
 800179e: bdf0         	pop	{r4, r5, r6, r7, pc}
 80017a0: 9a1e         	ldr	r2, [sp, #0x78]
 80017a2: f24d 63a0    	movw	r3, #0xd6a0
 80017a6: 4608         	mov	r0, r1
 80017a8: f6c0 0300    	movt	r3, #0x800
 80017ac: 4659         	mov	r1, r11
 80017ae: f00a fa44    	bl	0x800bc3a <core::slice::index::slice_index_fail> @ imm = #0xa488
 80017b2: 9916         	ldr	r1, [sp, #0x58]
 80017b4: 2224         	movs	r2, #0x24
 80017b6: eb01 0141    	add.w	r1, r1, r1, lsl #1
 80017ba: eb02 0181    	add.w	r1, r2, r1, lsl #2
 80017be: f24d 7340    	movw	r3, #0xd740
 80017c2: 465a         	mov	r2, r11
 80017c4: f6c0 0300    	movt	r3, #0x800
 80017c8: f00a fa37    	bl	0x800bc3a <core::slice::index::slice_index_fail> @ imm = #0xa46e
 80017cc: f24d 63c0    	movw	r3, #0xd6c0
 80017d0: 4608         	mov	r0, r1
 80017d2: f6c0 0300    	movt	r3, #0x800
 80017d6: 4641         	mov	r1, r8
 80017d8: 4662         	mov	r2, r12
 80017da: f00a fa2e    	bl	0x800bc3a <core::slice::index::slice_index_fail> @ imm = #0xa45c
 80017de: f24d 63b0    	movw	r3, #0xd6b0
 80017e2: 2000         	movs	r0, #0x0
 80017e4: f6c0 0300    	movt	r3, #0x800
 80017e8: 4621         	mov	r1, r4
 80017ea: 2204         	movs	r2, #0x4
 80017ec: f00a fa25    	bl	0x800bc3a <core::slice::index::slice_index_fail> @ imm = #0xa44a
 80017f0: f106 0128    	add.w	r1, r6, #0x28
 80017f4: f24d 7340    	movw	r3, #0xd740
 80017f8: 465a         	mov	r2, r11
 80017fa: f6c0 0300    	movt	r3, #0x800
 80017fe: f00a fa1c    	bl	0x800bc3a <core::slice::index::slice_index_fail> @ imm = #0xa438
 8001802: f64d 3390    	movw	r3, #0xdb90
 8001806: 4650         	mov	r0, r10
 8001808: f6c0 0300    	movt	r3, #0x800
 800180c: 4659         	mov	r1, r11
 800180e: 465a         	mov	r2, r11
 8001810: f00a fa13    	bl	0x800bc3a <core::slice::index::slice_index_fail> @ imm = #0xa426
 8001814: f64d 33a0    	movw	r3, #0xdba0
 8001818: 2000         	movs	r0, #0x0
 800181a: f6c0 0300    	movt	r3, #0x800
 800181e: 4651         	mov	r1, r10
 8001820: 465a         	mov	r2, r11
 8001822: f00a fa0a    	bl	0x800bc3a <core::slice::index::slice_index_fail> @ imm = #0xa414
 8001826: f64d 3280    	movw	r2, #0xdb80
 800182a: 4671         	mov	r1, lr
 800182c: f6c0 0200    	movt	r2, #0x800
 8001830: f009 ff16    	bl	0x800b660 <core::panicking::panic_bounds_check> @ imm = #0x9e2c
 8001834: f64d 3250    	movw	r2, #0xdb50
 8001838: 4659         	mov	r1, r11
 800183a: f6c0 0200    	movt	r2, #0x800
 800183e: f009 ff0f    	bl	0x800b660 <core::panicking::panic_bounds_check> @ imm = #0x9e1e
 8001842: f64d 3260    	movw	r2, #0xdb60
 8001846: 4659         	mov	r1, r11
 8001848: f6c0 0200    	movt	r2, #0x800
 800184c: f009 ff08    	bl	0x800b660 <core::panicking::panic_bounds_check> @ imm = #0x9e10
 8001850: f64d 3270    	movw	r2, #0xdb70
 8001854: 4659         	mov	r1, r11
 8001856: f6c0 0200    	movt	r2, #0x800
 800185a: f009 ff01    	bl	0x800b660 <core::panicking::panic_bounds_check> @ imm = #0x9e02

0800185e <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3>:
 800185e: b5f0         	push	{r4, r5, r6, r7, lr}
 8001860: af03         	add	r7, sp, #0xc
 8001862: e92d 0f00    	push.w	{r8, r9, r10, r11}
 8001866: b087         	sub	sp, #0x1c
 8001868: f64a 23ab    	movw	r3, #0xaaab
 800186c: f890 208f    	ldrb.w	r2, [r0, #0x8f]
 8001870: f6ca 23aa    	movt	r3, #0xaaaa
 8001874: 2a01         	cmp	r2, #0x1
 8001876: f040 80c9    	bne.w	0x8001a0c <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x1ae> @ imm = #0x192
 800187a: f8d0 b070    	ldr.w	r11, [r0, #0x70]
 800187e: 2900         	cmp	r1, #0x0
 8001880: f000 80c1    	beq.w	0x8001a06 <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x1a8> @ imm = #0x182
 8001884: f1bb 0f05    	cmp.w	r11, #0x5
 8001888: f200 80bd    	bhi.w	0x8001a06 <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x1a8> @ imm = #0x17a
 800188c: 6c42         	ldr	r2, [r0, #0x44]
 800188e: f1a1 0c01    	sub.w	r12, r1, #0x1
 8001892: f04f 0801    	mov.w	r8, #0x1
 8001896: f04f 0901    	mov.w	r9, #0x1
 800189a: fba2 2303    	umull	r2, r3, r2, r3
 800189e: f890 2091    	ldrb.w	r2, [r0, #0x91]
 80018a2: ea4f 0a93    	lsr.w	r10, r3, #0x2
 80018a6: f1aa 0e01    	sub.w	lr, r10, #0x1
 80018aa: 6f45         	ldr	r5, [r0, #0x74]
 80018ac: 07d2         	lsls	r2, r2, #0x1f
 80018ae: 6c41         	ldr	r1, [r0, #0x44]
 80018b0: eb05 0345    	add.w	r3, r5, r5, lsl #1
 80018b4: eb0b 0343    	add.w	r3, r11, r3, lsl #1
 80018b8: d032         	beq	0x8001920 <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0xc2> @ imm = #0x64
 80018ba: 428b         	cmp	r3, r1
 80018bc: f080 8360    	bhs.w	0x8001f80 <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x722> @ imm = #0x6c0
 80018c0: f890 1090    	ldrb.w	r1, [r0, #0x90]
 80018c4: 6c06         	ldr	r6, [r0, #0x40]
 80018c6: 1e4c         	subs	r4, r1, #0x1
 80018c8: 2900         	cmp	r1, #0x0
 80018ca: f004 041f    	and	r4, r4, #0x1f
 80018ce: f856 2023    	ldr.w	r2, [r6, r3, lsl #2]
 80018d2: fa08 f404    	lsl.w	r4, r8, r4
 80018d6: bf18         	it	ne
 80018d8: 4422         	addne	r2, r4
 80018da: f001 041f    	and	r4, r1, #0x1f
 80018de: fa42 f204    	asr.w	r2, r2, r4
 80018e2: f846 2023    	str.w	r2, [r6, r3, lsl #2]
 80018e6: b145         	cbz	r5, 0x80018fa <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x9c> @ imm = #0x10
 80018e8: e9d0 341f    	ldrd	r3, r4, [r0, #124]
 80018ec: 1ad3         	subs	r3, r2, r3
 80018ee: 005e         	lsls	r6, r3, #0x1
 80018f0: ea86 73e3    	eor.w	r3, r6, r3, asr #31
 80018f4: 4423         	add	r3, r4
 80018f6: f8c0 3080    	str.w	r3, [r0, #0x80]
 80018fa: 1c6b         	adds	r3, r5, #0x1
 80018fc: 6743         	str	r3, [r0, #0x74]
 80018fe: 4553         	cmp	r3, r10
 8001900: 67c2         	str	r2, [r0, #0x7c]
 8001902: d14e         	bne	0x80019a2 <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x144> @ imm = #0x9c
 8001904: f8d0 3080    	ldr.w	r3, [r0, #0x80]
 8001908: 2200         	movs	r2, #0x0
 800190a: f1be 0f00    	cmp.w	lr, #0x0
 800190e: 6742         	str	r2, [r0, #0x74]
 8001910: bf18         	it	ne
 8001912: 2b00         	cmpne	r3, #0x0
 8001914: d149         	bne	0x80019aa <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x14c> @ imm = #0x92
 8001916: f1ab 0301    	sub.w	r3, r11, #0x1
 800191a: 2b04         	cmp	r3, #0x4
 800191c: d955         	bls	0x80019ca <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x16c> @ imm = #0xaa
 800191e: e34e         	b	0x8001fbe <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x760> @ imm = #0x69c
 8001920: 428b         	cmp	r3, r1
 8001922: f080 8326    	bhs.w	0x8001f72 <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x714> @ imm = #0x64c
 8001926: 6c01         	ldr	r1, [r0, #0x40]
 8001928: f8d0 2088    	ldr.w	r2, [r0, #0x88]
 800192c: f851 1023    	ldr.w	r1, [r1, r3, lsl #2]
 8001930: 4253         	rsbs	r3, r2, #0
 8001932: 4299         	cmp	r1, r3
 8001934: f2c0 8308    	blt.w	0x8001f48 <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x6ea> @ imm = #0x610
 8001938: 4291         	cmp	r1, r2
 800193a: f04f 0201    	mov.w	r2, #0x1
 800193e: f300 82fd    	bgt.w	0x8001f3c <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x6de> @ imm = #0x5fa
 8001942: 6f83         	ldr	r3, [r0, #0x78]
 8001944: 1c6a         	adds	r2, r5, #0x1
 8001946: 6742         	str	r2, [r0, #0x74]
 8001948: 2900         	cmp	r1, #0x0
 800194a: bf48         	it	mi
 800194c: 4249         	rsbmi	r1, r1, #0
 800194e: 4299         	cmp	r1, r3
 8001950: bfd8         	it	le
 8001952: 4619         	movle	r1, r3
 8001954: 2300         	movs	r3, #0x0
 8001956: 4552         	cmp	r2, r10
 8001958: 6781         	str	r1, [r0, #0x78]
 800195a: d124         	bne	0x80019a6 <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x148> @ imm = #0x48
 800195c: f890 208c    	ldrb.w	r2, [r0, #0x8c]
 8001960: fab1 f581    	clz	r5, r1
 8001964: f1c5 0520    	rsb.w	r5, r5, #0x20
 8001968: 6743         	str	r3, [r0, #0x74]
 800196a: 3a01         	subs	r2, #0x1
 800196c: b2d6         	uxtb	r6, r2
 800196e: 42b5         	cmp	r5, r6
 8001970: d90c         	bls	0x800198c <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x12e> @ imm = #0x18
 8001972: 1bab         	subs	r3, r5, r6
 8001974: f002 021f    	and	r2, r2, #0x1f
 8001978: 1e5c         	subs	r4, r3, #0x1
 800197a: fa08 f404    	lsl.w	r4, r8, r4
 800197e: 4421         	add	r1, r4
 8001980: f003 041f    	and	r4, r3, #0x1f
 8001984: 40e1         	lsrs	r1, r4
 8001986: 40d1         	lsrs	r1, r2
 8001988: bf18         	it	ne
 800198a: 3301         	addne	r3, #0x1
 800198c: f890 108d    	ldrb.w	r1, [r0, #0x8d]
 8001990: 2201         	movs	r2, #0x1
 8001992: f880 2091    	strb.w	r2, [r0, #0x91]
 8001996: 4299         	cmp	r1, r3
 8001998: bf88         	it	hi
 800199a: 460b         	movhi	r3, r1
 800199c: f880 3090    	strb.w	r3, [r0, #0x90]
 80019a0: e028         	b	0x80019f4 <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x196> @ imm = #0x50
 80019a2: 2201         	movs	r2, #0x1
 80019a4: e026         	b	0x80019f4 <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x196> @ imm = #0x4c
 80019a6: 2200         	movs	r2, #0x0
 80019a8: e024         	b	0x80019f4 <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x196> @ imm = #0x48
 80019aa: fbb3 f2fe    	udiv	r2, r3, lr
 80019ae: fb02 331e    	mls	r3, r2, lr, r3
 80019b2: 2b00         	cmp	r3, #0x0
 80019b4: bf18         	it	ne
 80019b6: 3201         	addne	r2, #0x1
 80019b8: fab2 f282    	clz	r2, r2
 80019bc: f1c2 021f    	rsb.w	r2, r2, #0x1f
 80019c0: f1ab 0301    	sub.w	r3, r11, #0x1
 80019c4: 2b04         	cmp	r3, #0x4
 80019c6: f200 82fa    	bhi.w	0x8001fbe <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x760> @ imm = #0x5f4
 80019ca: f100 0448    	add.w	r4, r0, #0x48
 80019ce: f844 1033    	str.w	r1, [r4, r3, lsl #3]
 80019d2: eb04 01c3    	add.w	r1, r4, r3, lsl #3
 80019d6: 2402         	movs	r4, #0x2
 80019d8: 6f03         	ldr	r3, [r0, #0x70]
 80019da: 710a         	strb	r2, [r1, #0x4]
 80019dc: 2200         	movs	r2, #0x0
 80019de: 714c         	strb	r4, [r1, #0x5]
 80019e0: f103 0b01    	add.w	r11, r3, #0x1
 80019e4: f880 2091    	strb.w	r2, [r0, #0x91]
 80019e8: e9c0 221e    	strd	r2, r2, [r0, #120]
 80019ec: f8c0 2080    	str.w	r2, [r0, #0x80]
 80019f0: f8c0 b070    	str.w	r11, [r0, #0x70]
 80019f4: f1ac 0101    	sub.w	r1, r12, #0x1
 80019f8: 4561         	cmp	r1, r12
 80019fa: d204         	bhs	0x8001a06 <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x1a8> @ imm = #0x8
 80019fc: 468c         	mov	r12, r1
 80019fe: f1bb 0f06    	cmp.w	r11, #0x6
 8001a02: f4ff af52    	blo.w	0x80018aa <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x4c> @ imm = #-0x15c
 8001a06: f1ab 0006    	sub.w	r0, r11, #0x6
 8001a0a: e292         	b	0x8001f32 <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x6d4> @ imm = #0x524
 8001a0c: 6c46         	ldr	r6, [r0, #0x44]
 8001a0e: 2900         	cmp	r1, #0x0
 8001a10: 6f02         	ldr	r2, [r0, #0x70]
 8001a12: fba6 6303    	umull	r6, r3, r6, r3
 8001a16: f000 828b    	beq.w	0x8001f30 <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x6d2> @ imm = #0x516
 8001a1a: 2a05         	cmp	r2, #0x5
 8001a1c: f200 8288    	bhi.w	0x8001f30 <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x6d2> @ imm = #0x510
 8001a20: ea4f 0893    	lsr.w	r8, r3, #0x2
 8001a24: f04f 3bff    	mov.w	r11, #0xffffffff
 8001a28: f1a8 0301    	sub.w	r3, r8, #0x1
 8001a2c: 9306         	str	r3, [sp, #0x18]
 8001a2e: f8cd 8008    	str.w	r8, [sp, #0x8]
 8001a32: 6f46         	ldr	r6, [r0, #0x74]
 8001a34: 6c43         	ldr	r3, [r0, #0x44]
 8001a36: f890 5091    	ldrb.w	r5, [r0, #0x91]
 8001a3a: eb06 0646    	add.w	r6, r6, r6, lsl #1
 8001a3e: eb02 0c46    	add.w	r12, r2, r6, lsl #1
 8001a42: b39d         	cbz	r5, 0x8001aac <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x24e> @ imm = #0x66
 8001a44: 459c         	cmp	r12, r3
 8001a46: f080 82aa    	bhs.w	0x8001f9e <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x740> @ imm = #0x554
 8001a4a: f890 508c    	ldrb.w	r5, [r0, #0x8c]
 8001a4e: 2201         	movs	r2, #0x1
 8001a50: f04f 0900    	mov.w	r9, #0x0
 8001a54: 2d0c         	cmp	r5, #0xc
 8001a56: f200 8271    	bhi.w	0x8001f3c <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x6de> @ imm = #0x4e2
 8001a5a: 2301         	movs	r3, #0x1
 8001a5c: 40ab         	lsls	r3, r5
 8001a5e: f413 5faa    	tst.w	r3, #0x1540
 8001a62: f000 826b    	beq.w	0x8001f3c <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x6de> @ imm = #0x4d6
 8001a66: 6f83         	ldr	r3, [r0, #0x78]
 8001a68: 2b00         	cmp	r3, #0x0
 8001a6a: f100 8267    	bmi.w	0x8001f3c <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x6de> @ imm = #0x4ce
 8001a6e: f8d0 9040    	ldr.w	r9, [r0, #0x40]
 8001a72: f859 402c    	ldr.w	r4, [r9, r12, lsl #2]
 8001a76: f1b4 3fff    	cmp.w	r4, #0xffffffff
 8001a7a: 46a2         	mov	r10, r4
 8001a7c: dc05         	bgt	0x8001a8a <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x22c> @ imm = #0xa
 8001a7e: f1b4 4f00    	cmp.w	r4, #0x80000000
 8001a82: f000 8263    	beq.w	0x8001f4c <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x6ee> @ imm = #0x4c6
 8001a86: f1c4 0a00    	rsb.w	r10, r4, #0x0
 8001a8a: 459a         	cmp	r10, r3
 8001a8c: f200 825e    	bhi.w	0x8001f4c <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x6ee> @ imm = #0x4bc
 8001a90: b3e3         	cbz	r3, 0x8001b0c <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x2ae> @ imm = #0x78
 8001a92: 1e6a         	subs	r2, r5, #0x1
 8001a94: 459a         	cmp	r10, r3
 8001a96: b2d5         	uxtb	r5, r2
 8001a98: fa0b f605    	lsl.w	r6, r11, r5
 8001a9c: ea6f 0e06    	mvn.w	lr, r6
 8001aa0: d137         	bne	0x8001b12 <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x2b4> @ imm = #0x6e
 8001aa2: 2c00         	cmp	r4, #0x0
 8001aa4: bf48         	it	mi
 8001aa6: f106 0e01    	addmi.w	lr, r6, #0x1
 8001aaa: e045         	b	0x8001b38 <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x2da> @ imm = #0x8a
 8001aac: ea4f 0e46    	lsl.w	lr, r6, #0x1
 8001ab0: 459e         	cmp	lr, r3
 8001ab2: f080 827c    	bhs.w	0x8001fae <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x750> @ imm = #0x4f8
 8001ab6: 459c         	cmp	r12, r3
 8001ab8: f080 8269    	bhs.w	0x8001f8e <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x730> @ imm = #0x4d2
 8001abc: 6c04         	ldr	r4, [r0, #0x40]
 8001abe: f04f 0901    	mov.w	r9, #0x1
 8001ac2: f8d0 2088    	ldr.w	r2, [r0, #0x88]
 8001ac6: f854 502c    	ldr.w	r5, [r4, r12, lsl #2]
 8001aca: 4295         	cmp	r5, r2
 8001acc: f300 823c    	bgt.w	0x8001f48 <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x6ea> @ imm = #0x478
 8001ad0: 4253         	rsbs	r3, r2, #0
 8001ad2: 429d         	cmp	r5, r3
 8001ad4: f2c0 8238    	blt.w	0x8001f48 <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x6ea> @ imm = #0x470
 8001ad8: f854 602e    	ldr.w	r6, [r4, lr, lsl #2]
 8001adc: 429e         	cmp	r6, r3
 8001ade: f2c0 8233    	blt.w	0x8001f48 <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x6ea> @ imm = #0x466
 8001ae2: 4296         	cmp	r6, r2
 8001ae4: f04f 0201    	mov.w	r2, #0x1
 8001ae8: f300 8228    	bgt.w	0x8001f3c <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x6de> @ imm = #0x450
 8001aec: 1baa         	subs	r2, r5, r6
 8001aee: f844 202c    	str.w	r2, [r4, r12, lsl #2]
 8001af2: e9d0 431d    	ldrd	r4, r3, [r0, #116]
 8001af6: bf48         	it	mi
 8001af8: 4252         	rsbmi	r2, r2, #0
 8001afa: 429a         	cmp	r2, r3
 8001afc: bfd8         	it	le
 8001afe: 461a         	movle	r2, r3
 8001b00: 6782         	str	r2, [r0, #0x78]
 8001b02: 1c62         	adds	r2, r4, #0x1
 8001b04: 6742         	str	r2, [r0, #0x74]
 8001b06: 4542         	cmp	r2, r8
 8001b08: d03f         	beq	0x8001b8a <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x32c> @ imm = #0x7e
 8001b0a: e18f         	b	0x8001e2c <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x5ce> @ imm = #0x31e
 8001b0c: f04f 0e00    	mov.w	lr, #0x0
 8001b10: e012         	b	0x8001b38 <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x2da> @ imm = #0x24
 8001b12: fb8a 620e    	smull	r6, r2, r10, lr
 8001b16: ebb2 7fe6    	cmp.w	r2, r6, asr #31
 8001b1a: f040 818e    	bne.w	0x8001e3a <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x5dc> @ imm = #0x31c
 8001b1e: fb96 fef3    	sdiv	lr, r6, r3
 8001b22: fb0e 6513    	mls	r5, lr, r3, r6
 8001b26: 1b5b         	subs	r3, r3, r5
 8001b28: 429d         	cmp	r5, r3
 8001b2a: bfa8         	it	ge
 8001b2c: f10e 0e01    	addge.w	lr, lr, #0x1
 8001b30: 2c00         	cmp	r4, #0x0
 8001b32: bf48         	it	mi
 8001b34: f1ce 0e00    	rsbmi.w	lr, lr, #0x0
 8001b38: f849 e02c    	str.w	lr, [r9, r12, lsl #2]
 8001b3c: 1d02         	adds	r2, r0, #0x4
 8001b3e: f890 608e    	ldrb.w	r6, [r0, #0x8e]
 8001b42: 2e01         	cmp	r6, #0x1
 8001b44: d009         	beq	0x8001b5a <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x2fc> @ imm = #0x12
 8001b46: ea4f 044e    	lsl.w	r4, lr, #0x1
 8001b4a: 6803         	ldr	r3, [r0]
 8001b4c: ea84 75ee    	eor.w	r5, r4, lr, asr #31
 8001b50: ebb4 7fee    	cmp.w	r4, lr, asr #31
 8001b54: 442b         	add	r3, r5
 8001b56: 6003         	str	r3, [r0]
 8001b58: d13a         	bne	0x8001bd0 <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x372> @ imm = #0x74
 8001b5a: 6f44         	ldr	r4, [r0, #0x74]
 8001b5c: 2e00         	cmp	r6, #0x0
 8001b5e: bf18         	it	ne
 8001b60: 2c00         	cmpne	r4, #0x0
 8001b62: d00b         	beq	0x8001b7c <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x31e> @ imm = #0x16
 8001b64: 6fc6         	ldr	r6, [r0, #0x7c]
 8001b66: 6a03         	ldr	r3, [r0, #0x20]
 8001b68: ebae 0506    	sub.w	r5, lr, r6
 8001b6c: 006a         	lsls	r2, r5, #0x1
 8001b6e: ea82 76e5    	eor.w	r6, r2, r5, asr #31
 8001b72: 4433         	add	r3, r6
 8001b74: ebb2 7fe5    	cmp.w	r2, r5, asr #31
 8001b78: 6203         	str	r3, [r0, #0x20]
 8001b7a: d166         	bne	0x8001c4a <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x3ec> @ imm = #0xcc
 8001b7c: f8c0 e07c    	str.w	lr, [r0, #0x7c]
 8001b80: 1c62         	adds	r2, r4, #0x1
 8001b82: 6742         	str	r2, [r0, #0x74]
 8001b84: 4542         	cmp	r2, r8
 8001b86: f040 8151    	bne.w	0x8001e2c <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x5ce> @ imm = #0x2a2
 8001b8a: f890 2091    	ldrb.w	r2, [r0, #0x91]
 8001b8e: 2300         	movs	r3, #0x0
 8001b90: 6743         	str	r3, [r0, #0x74]
 8001b92: 2a00         	cmp	r2, #0x0
 8001b94: f000 80a0    	beq.w	0x8001cd8 <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x47a> @ imm = #0x140
 8001b98: f890 e08c    	ldrb.w	lr, [r0, #0x8c]
 8001b9c: 6f84         	ldr	r4, [r0, #0x78]
 8001b9e: fb08 f20e    	mul	r2, r8, lr
 8001ba2: 0753         	lsls	r3, r2, #0x1d
 8001ba4: ea4f 06d2    	lsr.w	r6, r2, #0x3
 8001ba8: f04f 0301    	mov.w	r3, #0x1
 8001bac: bf18         	it	ne
 8001bae: eb03 06d2    	addne.w	r6, r3, r2, lsr #3
 8001bb2: 2c00         	cmp	r4, #0x0
 8001bb4: f000 8094    	beq.w	0x8001ce0 <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x482> @ imm = #0x128
 8001bb8: f890 208e    	ldrb.w	r2, [r0, #0x8e]
 8001bbc: 9404         	str	r4, [sp, #0x10]
 8001bbe: 2a01         	cmp	r2, #0x1
 8001bc0: 9605         	str	r6, [sp, #0x14]
 8001bc2: f040 8091    	bne.w	0x8001ce8 <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x48a> @ imm = #0x122
 8001bc6: 2203         	movs	r2, #0x3
 8001bc8: f04f 0c00    	mov.w	r12, #0x0
 8001bcc: 9203         	str	r2, [sp, #0xc]
 8001bce: e0c4         	b	0x8001d5a <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x4fc> @ imm = #0x188
 8001bd0: fa95 f3a5    	rbit	r3, r5
 8001bd4: fab3 f383    	clz	r3, r3
 8001bd8: 2b0c         	cmp	r3, #0xc
 8001bda: f200 81ba    	bhi.w	0x8001f52 <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x6f4> @ imm = #0x374
 8001bde: f832 4013    	ldrh.w	r4, [r2, r3, lsl #1]
 8001be2: 3401         	adds	r4, #0x1
 8001be4: f822 4013    	strh.w	r4, [r2, r3, lsl #1]
 8001be8: 1e6b         	subs	r3, r5, #0x1
 8001bea: 401d         	ands	r5, r3
 8001bec: d0b5         	beq	0x8001b5a <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x2fc> @ imm = #-0x96
 8001bee: fa95 f3a5    	rbit	r3, r5
 8001bf2: fab3 f383    	clz	r3, r3
 8001bf6: 2b0c         	cmp	r3, #0xc
 8001bf8: f200 81ab    	bhi.w	0x8001f52 <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x6f4> @ imm = #0x356
 8001bfc: f832 4013    	ldrh.w	r4, [r2, r3, lsl #1]
 8001c00: 3401         	adds	r4, #0x1
 8001c02: f822 4013    	strh.w	r4, [r2, r3, lsl #1]
 8001c06: 1e6b         	subs	r3, r5, #0x1
 8001c08: 401d         	ands	r5, r3
 8001c0a: d0a6         	beq	0x8001b5a <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x2fc> @ imm = #-0xb4
 8001c0c: fa95 f3a5    	rbit	r3, r5
 8001c10: fab3 f383    	clz	r3, r3
 8001c14: 2b0c         	cmp	r3, #0xc
 8001c16: f200 819c    	bhi.w	0x8001f52 <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x6f4> @ imm = #0x338
 8001c1a: f832 4013    	ldrh.w	r4, [r2, r3, lsl #1]
 8001c1e: 3401         	adds	r4, #0x1
 8001c20: f822 4013    	strh.w	r4, [r2, r3, lsl #1]
 8001c24: 1e6b         	subs	r3, r5, #0x1
 8001c26: 401d         	ands	r5, r3
 8001c28: d097         	beq	0x8001b5a <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x2fc> @ imm = #-0xd2
 8001c2a: fa95 f3a5    	rbit	r3, r5
 8001c2e: fab3 f383    	clz	r3, r3
 8001c32: 2b0c         	cmp	r3, #0xc
 8001c34: f200 818d    	bhi.w	0x8001f52 <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x6f4> @ imm = #0x31a
 8001c38: f832 4013    	ldrh.w	r4, [r2, r3, lsl #1]
 8001c3c: 3401         	adds	r4, #0x1
 8001c3e: f822 4013    	strh.w	r4, [r2, r3, lsl #1]
 8001c42: 1e6b         	subs	r3, r5, #0x1
 8001c44: 401d         	ands	r5, r3
 8001c46: d1c3         	bne	0x8001bd0 <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x372> @ imm = #-0x7a
 8001c48: e787         	b	0x8001b5a <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x2fc> @ imm = #-0xf2
 8001c4a: fa96 f2a6    	rbit	r2, r6
 8001c4e: fab2 f382    	clz	r3, r2
 8001c52: 2b0c         	cmp	r3, #0xc
 8001c54: f200 817d    	bhi.w	0x8001f52 <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x6f4> @ imm = #0x2fa
 8001c58: f100 0524    	add.w	r5, r0, #0x24
 8001c5c: f835 2013    	ldrh.w	r2, [r5, r3, lsl #1]
 8001c60: 3201         	adds	r2, #0x1
 8001c62: f825 2013    	strh.w	r2, [r5, r3, lsl #1]
 8001c66: 1e72         	subs	r2, r6, #0x1
 8001c68: 4016         	ands	r6, r2
 8001c6a: d087         	beq	0x8001b7c <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x31e> @ imm = #-0xf2
 8001c6c: fa96 f2a6    	rbit	r2, r6
 8001c70: fab2 f382    	clz	r3, r2
 8001c74: 2b0c         	cmp	r3, #0xc
 8001c76: f200 816c    	bhi.w	0x8001f52 <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x6f4> @ imm = #0x2d8
 8001c7a: f100 0524    	add.w	r5, r0, #0x24
 8001c7e: f835 2013    	ldrh.w	r2, [r5, r3, lsl #1]
 8001c82: 3201         	adds	r2, #0x1
 8001c84: f825 2013    	strh.w	r2, [r5, r3, lsl #1]
 8001c88: 1e72         	subs	r2, r6, #0x1
 8001c8a: 4016         	ands	r6, r2
 8001c8c: f43f af76    	beq.w	0x8001b7c <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x31e> @ imm = #-0x114
 8001c90: fa96 f2a6    	rbit	r2, r6
 8001c94: fab2 f382    	clz	r3, r2
 8001c98: 2b0c         	cmp	r3, #0xc
 8001c9a: f200 815a    	bhi.w	0x8001f52 <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x6f4> @ imm = #0x2b4
 8001c9e: f100 0524    	add.w	r5, r0, #0x24
 8001ca2: f835 2013    	ldrh.w	r2, [r5, r3, lsl #1]
 8001ca6: 3201         	adds	r2, #0x1
 8001ca8: f825 2013    	strh.w	r2, [r5, r3, lsl #1]
 8001cac: 1e72         	subs	r2, r6, #0x1
 8001cae: 4016         	ands	r6, r2
 8001cb0: f43f af64    	beq.w	0x8001b7c <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x31e> @ imm = #-0x138
 8001cb4: fa96 f2a6    	rbit	r2, r6
 8001cb8: fab2 f382    	clz	r3, r2
 8001cbc: 2b0c         	cmp	r3, #0xc
 8001cbe: f200 8148    	bhi.w	0x8001f52 <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x6f4> @ imm = #0x290
 8001cc2: f100 0524    	add.w	r5, r0, #0x24
 8001cc6: f835 2013    	ldrh.w	r2, [r5, r3, lsl #1]
 8001cca: 3201         	adds	r2, #0x1
 8001ccc: f825 2013    	strh.w	r2, [r5, r3, lsl #1]
 8001cd0: 1e72         	subs	r2, r6, #0x1
 8001cd2: 4016         	ands	r6, r2
 8001cd4: d1b9         	bne	0x8001c4a <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x3ec> @ imm = #-0x8e
 8001cd6: e751         	b	0x8001b7c <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x31e> @ imm = #-0x15e
 8001cd8: 2201         	movs	r2, #0x1
 8001cda: f880 2091    	strb.w	r2, [r0, #0x91]
 8001cde: e0a5         	b	0x8001e2c <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x5ce> @ imm = #0x14a
 8001ce0: 2500         	movs	r5, #0x0
 8001ce2: f04f 0c00    	mov.w	r12, #0x0
 8001ce6: e079         	b	0x8001ddc <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x57e> @ imm = #0xf2
 8001ce8: f8d0 a000    	ldr.w	r10, [r0]
 8001cec: f04f 0c00    	mov.w	r12, #0x0
 8001cf0: f04f 3bff    	mov.w	r11, #0xffffffff
 8001cf4: 2400         	movs	r4, #0x0
 8001cf6: 9203         	str	r2, [sp, #0xc]
 8001cf8: b2e3         	uxtb	r3, r4
 8001cfa: 2b0c         	cmp	r3, #0xc
 8001cfc: f200 8131    	bhi.w	0x8001f62 <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x704> @ imm = #0x262
 8001d00: 1c5a         	adds	r2, r3, #0x1
 8001d02: 2601         	movs	r6, #0x1
 8001d04: fb02 a208    	mla	r2, r2, r8, r10
 8001d08: 08d5         	lsrs	r5, r2, #0x3
 8001d0a: ea5f 7942    	lsls.w	r9, r2, #0x1d
 8001d0e: bf18         	it	ne
 8001d10: eb06 05d2    	addne.w	r5, r6, r2, lsr #3
 8001d14: 455d         	cmp	r5, r11
 8001d16: f04f 0200    	mov.w	r2, #0x0
 8001d1a: bf3c         	itt	lo
 8001d1c: 46ab         	movlo	r11, r5
 8001d1e: 46a4         	movlo	r12, r4
 8001d20: 4573         	cmp	r3, lr
 8001d22: bf38         	it	lo
 8001d24: 2201         	movlo	r2, #0x1
 8001d26: d20a         	bhs	0x8001d3e <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x4e0> @ imm = #0x14
 8001d28: 1d05         	adds	r5, r0, #0x4
 8001d2a: 4414         	add	r4, r2
 8001d2c: f835 3013    	ldrh.w	r3, [r5, r3, lsl #1]
 8001d30: ebaa 0203    	sub.w	r2, r10, r3
 8001d34: ea4f 0a52    	lsr.w	r10, r2, #0x1
 8001d38: b2e2         	uxtb	r2, r4
 8001d3a: 4572         	cmp	r2, lr
 8001d3c: d9dc         	bls	0x8001cf8 <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x49a> @ imm = #-0x48
 8001d3e: 9b05         	ldr	r3, [sp, #0x14]
 8001d40: 2200         	movs	r2, #0x0
 8001d42: 2503         	movs	r5, #0x3
 8001d44: 459b         	cmp	r11, r3
 8001d46: bf2c         	ite	hs
 8001d48: 4694         	movhs	r12, r2
 8001d4a: 465b         	movlo	r3, r11
 8001d4c: bf38         	it	lo
 8001d4e: 2501         	movlo	r5, #0x1
 8001d50: 9a03         	ldr	r2, [sp, #0xc]
 8001d52: 2a00         	cmp	r2, #0x0
 8001d54: d03f         	beq	0x8001dd6 <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x578> @ imm = #0x7e
 8001d56: 9503         	str	r5, [sp, #0xc]
 8001d58: 9305         	str	r3, [sp, #0x14]
 8001d5a: f100 0220    	add.w	r2, r0, #0x20
 8001d5e: f04f 0b00    	mov.w	r11, #0x0
 8001d62: f04f 35ff    	mov.w	r5, #0xffffffff
 8001d66: f04f 0900    	mov.w	r9, #0x0
 8001d6a: f8d2 8000    	ldr.w	r8, [r2]
 8001d6e: fa5f f389    	uxtb.w	r3, r9
 8001d72: 2b0c         	cmp	r3, #0xc
 8001d74: f200 80f5    	bhi.w	0x8001f62 <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x704> @ imm = #0x1ea
 8001d78: 9c06         	ldr	r4, [sp, #0x18]
 8001d7a: f103 0a01    	add.w	r10, r3, #0x1
 8001d7e: eb08 020e    	add.w	r2, r8, lr
 8001d82: fb0a 2204    	mla	r2, r10, r4, r2
 8001d86: 0756         	lsls	r6, r2, #0x1d
 8001d88: ea4f 04d2    	lsr.w	r4, r2, #0x3
 8001d8c: f04f 0601    	mov.w	r6, #0x1
 8001d90: bf18         	it	ne
 8001d92: eb06 04d2    	addne.w	r4, r6, r2, lsr #3
 8001d96: 42ac         	cmp	r4, r5
 8001d98: f04f 0200    	mov.w	r2, #0x0
 8001d9c: bf3c         	itt	lo
 8001d9e: 4625         	movlo	r5, r4
 8001da0: 46cb         	movlo	r11, r9
 8001da2: 4573         	cmp	r3, lr
 8001da4: bf38         	it	lo
 8001da6: 2201         	movlo	r2, #0x1
 8001da8: d20c         	bhs	0x8001dc4 <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x566> @ imm = #0x18
 8001daa: f100 0424    	add.w	r4, r0, #0x24
 8001dae: 4491         	add	r9, r2
 8001db0: f834 3013    	ldrh.w	r3, [r4, r3, lsl #1]
 8001db4: eba8 0203    	sub.w	r2, r8, r3
 8001db8: ea4f 0852    	lsr.w	r8, r2, #0x1
 8001dbc: fa5f f289    	uxtb.w	r2, r9
 8001dc0: 4572         	cmp	r2, lr
 8001dc2: d9d4         	bls	0x8001d6e <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x510> @ imm = #-0x58
 8001dc4: 9a05         	ldr	r2, [sp, #0x14]
 8001dc6: 4295         	cmp	r5, r2
 8001dc8: bf38         	it	lo
 8001dca: 46dc         	movlo	r12, r11
 8001dcc: 9d03         	ldr	r5, [sp, #0xc]
 8001dce: bf38         	it	lo
 8001dd0: 2502         	movlo	r5, #0x2
 8001dd2: f8dd 8008    	ldr.w	r8, [sp, #0x8]
 8001dd6: f04f 3bff    	mov.w	r11, #0xffffffff
 8001dda: 9c04         	ldr	r4, [sp, #0x10]
 8001ddc: 6f02         	ldr	r2, [r0, #0x70]
 8001dde: 1e53         	subs	r3, r2, #0x1
 8001de0: 2b05         	cmp	r3, #0x5
 8001de2: f080 80f5    	bhs.w	0x8001fd0 <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x772> @ imm = #0x1ea
 8001de6: f100 0248    	add.w	r2, r0, #0x48
 8001dea: 2600         	movs	r6, #0x0
 8001dec: f842 4033    	str.w	r4, [r2, r3, lsl #3]
 8001df0: eb02 02c3    	add.w	r2, r2, r3, lsl #3
 8001df4: 6f03         	ldr	r3, [r0, #0x70]
 8001df6: 7155         	strb	r5, [r2, #0x5]
 8001df8: f882 c004    	strb.w	r12, [r2, #0x4]
 8001dfc: 1c5a         	adds	r2, r3, #0x1
 8001dfe: 6702         	str	r2, [r0, #0x70]
 8001e00: f100 0220    	add.w	r2, r0, #0x20
 8001e04: f880 6091    	strb.w	r6, [r0, #0x91]
 8001e08: 6786         	str	r6, [r0, #0x78]
 8001e0a: 6006         	str	r6, [r0]
 8001e0c: 67c6         	str	r6, [r0, #0x7c]
 8001e0e: e9c0 6601    	strd	r6, r6, [r0, #4]
 8001e12: e9c0 6603    	strd	r6, r6, [r0, #12]
 8001e16: e9c0 6605    	strd	r6, r6, [r0, #20]
 8001e1a: 8386         	strh	r6, [r0, #0x1c]
 8001e1c: e9c2 6600    	strd	r6, r6, [r2]
 8001e20: e9c2 6602    	strd	r6, r6, [r2, #8]
 8001e24: e9c2 6604    	strd	r6, r6, [r2, #16]
 8001e28: 6196         	str	r6, [r2, #0x18]
 8001e2a: 8396         	strh	r6, [r2, #0x1c]
 8001e2c: 6f02         	ldr	r2, [r0, #0x70]
 8001e2e: 3901         	subs	r1, #0x1
 8001e30: d07e         	beq	0x8001f30 <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x6d2> @ imm = #0xfc
 8001e32: 2a06         	cmp	r2, #0x6
 8001e34: f4ff adfd    	blo.w	0x8001a32 <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x1d4> @ imm = #-0x406
 8001e38: e07a         	b	0x8001f30 <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x6d2> @ imm = #0xf4
 8001e3a: 2d01         	cmp	r5, #0x1
 8001e3c: f04f 0201    	mov.w	r2, #0x1
 8001e40: bf88         	it	hi
 8001e42: 462a         	movhi	r2, r5
 8001e44: f002 0603    	and	r6, r2, #0x3
 8001e48: 2d04         	cmp	r5, #0x4
 8001e4a: 9601         	str	r6, [sp, #0x4]
 8001e4c: d204         	bhs	0x8001e58 <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x5fa> @ imm = #0x8
 8001e4e: f04f 0e00    	mov.w	lr, #0x0
 8001e52: 4656         	mov	r6, r10
 8001e54: 9a01         	ldr	r2, [sp, #0x4]
 8001e56: e041         	b	0x8001edc <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x67e> @ imm = #0x82
 8001e58: f002 02fc    	and	r2, r2, #0xfc
 8001e5c: 9203         	str	r2, [sp, #0xc]
 8001e5e: 2200         	movs	r2, #0x0
 8001e60: f04f 0e00    	mov.w	lr, #0x0
 8001e64: 4656         	mov	r6, r10
 8001e66: f8cd 9000    	str.w	r9, [sp]
 8001e6a: 9205         	str	r2, [sp, #0x14]
 8001e6c: 1b9d         	subs	r5, r3, r6
 8001e6e: ea4f 098e    	lsl.w	r9, lr, #0x2
 8001e72: 1b75         	subs	r5, r6, r5
 8001e74: f109 0902    	add.w	r9, r9, #0x2
 8001e78: bfb8         	it	lt
 8001e7a: 0075         	lsllt	r5, r6, #0x1
 8001e7c: eba3 0205    	sub.w	r2, r3, r5
 8001e80: ea4f 0645    	lsl.w	r6, r5, #0x1
 8001e84: bfb8         	it	lt
 8001e86: ea4f 098e    	lsllt.w	r9, lr, #0x2
 8001e8a: 4295         	cmp	r5, r2
 8001e8c: bfa8         	it	ge
 8001e8e: 1aae         	subge	r6, r5, r2
 8001e90: eba3 0206    	sub.w	r2, r3, r6
 8001e94: bfa8         	it	ge
 8001e96: f109 0901    	addge.w	r9, r9, #0x1
 8001e9a: 1ab5         	subs	r5, r6, r2
 8001e9c: 4296         	cmp	r6, r2
 8001e9e: bfb8         	it	lt
 8001ea0: 0075         	lsllt	r5, r6, #0x1
 8001ea2: eba3 0205    	sub.w	r2, r3, r5
 8001ea6: 9204         	str	r2, [sp, #0x10]
 8001ea8: ea4f 0289    	lsl.w	r2, r9, #0x2
 8001eac: f102 0e02    	add.w	lr, r2, #0x2
 8001eb0: ea4f 0645    	lsl.w	r6, r5, #0x1
 8001eb4: bfb8         	it	lt
 8001eb6: ea4f 0e89    	lsllt.w	lr, r9, #0x2
 8001eba: 9a04         	ldr	r2, [sp, #0x10]
 8001ebc: 4295         	cmp	r5, r2
 8001ebe: bfa4         	itt	ge
 8001ec0: 1aae         	subge	r6, r5, r2
 8001ec2: f10e 0e01    	addge.w	lr, lr, #0x1
 8001ec6: 9a05         	ldr	r2, [sp, #0x14]
 8001ec8: 3204         	adds	r2, #0x4
 8001eca: 9205         	str	r2, [sp, #0x14]
 8001ecc: b2d5         	uxtb	r5, r2
 8001ece: 9a03         	ldr	r2, [sp, #0xc]
 8001ed0: 4295         	cmp	r5, r2
 8001ed2: d1cb         	bne	0x8001e6c <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x60e> @ imm = #-0x6a
 8001ed4: 9a01         	ldr	r2, [sp, #0x4]
 8001ed6: f8dd 9000    	ldr.w	r9, [sp]
 8001eda: b1fa         	cbz	r2, 0x8001f1c <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x6be> @ imm = #0x3e
 8001edc: 1b9d         	subs	r5, r3, r6
 8001ede: ea4f 0e4e    	lsl.w	lr, lr, #0x1
 8001ee2: 1b75         	subs	r5, r6, r5
 8001ee4: bfb8         	it	lt
 8001ee6: 0075         	lsllt	r5, r6, #0x1
 8001ee8: bfa8         	it	ge
 8001eea: f10e 0e01    	addge.w	lr, lr, #0x1
 8001eee: 2a01         	cmp	r2, #0x1
 8001ef0: d013         	beq	0x8001f1a <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x6bc> @ imm = #0x26
 8001ef2: 1b5e         	subs	r6, r3, r5
 8001ef4: ea4f 0e4e    	lsl.w	lr, lr, #0x1
 8001ef8: 1bae         	subs	r6, r5, r6
 8001efa: bfb8         	it	lt
 8001efc: 006e         	lsllt	r6, r5, #0x1
 8001efe: bfa8         	it	ge
 8001f00: f10e 0e01    	addge.w	lr, lr, #0x1
 8001f04: 2a02         	cmp	r2, #0x2
 8001f06: d009         	beq	0x8001f1c <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x6be> @ imm = #0x12
 8001f08: 1b9d         	subs	r5, r3, r6
 8001f0a: ea4f 0e4e    	lsl.w	lr, lr, #0x1
 8001f0e: 1b75         	subs	r5, r6, r5
 8001f10: bfb8         	it	lt
 8001f12: 0075         	lsllt	r5, r6, #0x1
 8001f14: bfa8         	it	ge
 8001f16: f10e 0e01    	addge.w	lr, lr, #0x1
 8001f1a: 462e         	mov	r6, r5
 8001f1c: ebb6 050a    	subs.w	r5, r6, r10
 8001f20: bfb8         	it	lt
 8001f22: 441d         	addlt	r5, r3
 8001f24: eba3 0305    	sub.w	r3, r3, r5
 8001f28: bfb8         	it	lt
 8001f2a: f1ae 0e01    	sublt.w	lr, lr, #0x1
 8001f2e: e5fb         	b	0x8001b28 <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x2ca> @ imm = #-0x40a
 8001f30: 1f90         	subs	r0, r2, #0x6
 8001f32: fab0 f080    	clz	r0, r0
 8001f36: 2200         	movs	r2, #0x0
 8001f38: ea4f 1950    	lsr.w	r9, r0, #0x5
 8001f3c: 4610         	mov	r0, r2
 8001f3e: 4649         	mov	r1, r9
 8001f40: b007         	add	sp, #0x1c
 8001f42: e8bd 0f00    	pop.w	{r8, r9, r10, r11}
 8001f46: bdf0         	pop	{r4, r5, r6, r7, pc}
 8001f48: 2201         	movs	r2, #0x1
 8001f4a: e7f7         	b	0x8001f3c <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x6de> @ imm = #-0x12
 8001f4c: f04f 0901    	mov.w	r9, #0x1
 8001f50: e7f4         	b	0x8001f3c <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x6de> @ imm = #-0x18
 8001f52: f64d 32dc    	movw	r2, #0xdbdc
 8001f56: f6c0 0200    	movt	r2, #0x800
 8001f5a: 4618         	mov	r0, r3
 8001f5c: 210d         	movs	r1, #0xd
 8001f5e: f009 fb7f    	bl	0x800b660 <core::panicking::panic_bounds_check> @ imm = #0x96fe
 8001f62: f64d 32ec    	movw	r2, #0xdbec
 8001f66: f6c0 0200    	movt	r2, #0x800
 8001f6a: 4618         	mov	r0, r3
 8001f6c: 210d         	movs	r1, #0xd
 8001f6e: f009 fb77    	bl	0x800b660 <core::panicking::panic_bounds_check> @ imm = #0x96ee
 8001f72: f24d 62f0    	movw	r2, #0xd6f0
 8001f76: 4618         	mov	r0, r3
 8001f78: f6c0 0200    	movt	r2, #0x800
 8001f7c: f009 fb70    	bl	0x800b660 <core::panicking::panic_bounds_check> @ imm = #0x96e0
 8001f80: f24d 7200    	movw	r2, #0xd700
 8001f84: 4618         	mov	r0, r3
 8001f86: f6c0 0200    	movt	r2, #0x800
 8001f8a: f009 fb69    	bl	0x800b660 <core::panicking::panic_bounds_check> @ imm = #0x96d2
 8001f8e: f64d 421c    	movw	r2, #0xdc1c
 8001f92: f6c0 0200    	movt	r2, #0x800
 8001f96: 4660         	mov	r0, r12
 8001f98: 4619         	mov	r1, r3
 8001f9a: f009 fb61    	bl	0x800b660 <core::panicking::panic_bounds_check> @ imm = #0x96c2
 8001f9e: f64d 422c    	movw	r2, #0xdc2c
 8001fa2: f6c0 0200    	movt	r2, #0x800
 8001fa6: 4660         	mov	r0, r12
 8001fa8: 4619         	mov	r1, r3
 8001faa: f009 fb59    	bl	0x800b660 <core::panicking::panic_bounds_check> @ imm = #0x96b2
 8001fae: f64d 420c    	movw	r2, #0xdc0c
 8001fb2: 4670         	mov	r0, lr
 8001fb4: f6c0 0200    	movt	r2, #0x800
 8001fb8: 4619         	mov	r1, r3
 8001fba: f009 fb51    	bl	0x800b660 <core::panicking::panic_bounds_check> @ imm = #0x96a2
 8001fbe: f24d 7210    	movw	r2, #0xd710
 8001fc2: f04f 30ff    	mov.w	r0, #0xffffffff
 8001fc6: f6c0 0200    	movt	r2, #0x800
 8001fca: 2105         	movs	r1, #0x5
 8001fcc: f009 fb48    	bl	0x800b660 <core::panicking::panic_bounds_check> @ imm = #0x9690
 8001fd0: f64d 32fc    	movw	r2, #0xdbfc
 8001fd4: 4618         	mov	r0, r3
 8001fd6: f6c0 0200    	movt	r2, #0x800
 8001fda: 2105         	movs	r1, #0x5
 8001fdc: f009 fb40    	bl	0x800b660 <core::panicking::panic_bounds_check> @ imm = #0x9680

08001fe0 <raylar_crcl_encoder_service::benchmark::shift_only::h394b3429a2171dca>:
 8001fe0: b5f0         	push	{r4, r5, r6, r7, lr}
 8001fe2: af03         	add	r7, sp, #0xc
 8001fe4: e92d 0b00    	push.w	{r8, r9, r11}
 8001fe8: b082         	sub	sp, #0x8
 8001fea: 6801         	ldr	r1, [r0]
 8001fec: f44f 6380    	mov.w	r3, #0x400
 8001ff0: 9101         	str	r1, [sp, #0x4]
 8001ff2: a901         	add	r1, sp, #0x4
 8001ff4: f645 716c    	movw	r1, #0x5f6c
 8001ff8: 9a01         	ldr	r2, [sp, #0x4]
 8001ffa: 4401         	add	r1, r0
 8001ffc: f002 021f    	and	r2, r2, #0x1f
 8002000: f8d1 60c0    	ldr.w	r6, [r1, #0xc0]
 8002004: 3b10         	subs	r3, #0x10
 8002006: f8d1 50d8    	ldr.w	r5, [r1, #0xd8]
 800200a: f8d1 40f0    	ldr.w	r4, [r1, #0xf0]
 800200e: fa46 f602    	asr.w	r6, r6, r2
 8002012: f8d1 c108    	ldr.w	r12, [r1, #0x108]
 8002016: f8c1 60c0    	str.w	r6, [r1, #0xc0]
 800201a: fa45 f502    	asr.w	r5, r5, r2
 800201e: f8d1 6150    	ldr.w	r6, [r1, #0x150]
 8002022: fa44 f402    	asr.w	r4, r4, r2
 8002026: f8c1 50d8    	str.w	r5, [r1, #0xd8]
 800202a: f8d1 5168    	ldr.w	r5, [r1, #0x168]
 800202e: f8d1 9180    	ldr.w	r9, [r1, #0x180]
 8002032: fa46 f602    	asr.w	r6, r6, r2
 8002036: f8c1 40f0    	str.w	r4, [r1, #0xf0]
 800203a: fa4c f402    	asr.w	r4, r12, r2
 800203e: f8d1 c198    	ldr.w	r12, [r1, #0x198]
 8002042: f8c1 6150    	str.w	r6, [r1, #0x150]
 8002046: fa45 f602    	asr.w	r6, r5, r2
 800204a: f8c1 6168    	str.w	r6, [r1, #0x168]
 800204e: fa49 f602    	asr.w	r6, r9, r2
 8002052: f8c1 6180    	str.w	r6, [r1, #0x180]
 8002056: fa4c f602    	asr.w	r6, r12, r2
 800205a: f8c1 6198    	str.w	r6, [r1, #0x198]
 800205e: f8d1 61b0    	ldr.w	r6, [r1, #0x1b0]
 8002062: f8d1 e120    	ldr.w	lr, [r1, #0x120]
 8002066: f8d1 8138    	ldr.w	r8, [r1, #0x138]
 800206a: fa46 f602    	asr.w	r6, r6, r2
 800206e: f8c1 4108    	str.w	r4, [r1, #0x108]
 8002072: f8c1 61b0    	str.w	r6, [r1, #0x1b0]
 8002076: fa4e f402    	asr.w	r4, lr, r2
 800207a: f8d1 61c8    	ldr.w	r6, [r1, #0x1c8]
 800207e: f8c1 4120    	str.w	r4, [r1, #0x120]
 8002082: fa48 f402    	asr.w	r4, r8, r2
 8002086: f8c1 4138    	str.w	r4, [r1, #0x138]
 800208a: fa46 f602    	asr.w	r6, r6, r2
 800208e: f8c1 61c8    	str.w	r6, [r1, #0x1c8]
 8002092: f8d1 61e0    	ldr.w	r6, [r1, #0x1e0]
 8002096: fa46 f602    	asr.w	r6, r6, r2
 800209a: f8c1 61e0    	str.w	r6, [r1, #0x1e0]
 800209e: f8d1 61f8    	ldr.w	r6, [r1, #0x1f8]
 80020a2: fa46 f602    	asr.w	r6, r6, r2
 80020a6: f8c1 61f8    	str.w	r6, [r1, #0x1f8]
 80020aa: f8d1 6210    	ldr.w	r6, [r1, #0x210]
 80020ae: fa46 f602    	asr.w	r6, r6, r2
 80020b2: f8c1 6210    	str.w	r6, [r1, #0x210]
 80020b6: f8d1 6228    	ldr.w	r6, [r1, #0x228]
 80020ba: fa46 f602    	asr.w	r6, r6, r2
 80020be: f8c1 6228    	str.w	r6, [r1, #0x228]
 80020c2: f501 71c0    	add.w	r1, r1, #0x180
 80020c6: d19b         	bne	0x8002000 <raylar_crcl_encoder_service::benchmark::shift_only::h394b3429a2171dca+0x20> @ imm = #-0xca
 80020c8: 6881         	ldr	r1, [r0, #0x8]
 80020ca: f44f 6380    	mov.w	r3, #0x400
 80020ce: 9101         	str	r1, [sp, #0x4]
 80020d0: a901         	add	r1, sp, #0x4
 80020d2: f645 61b0    	movw	r1, #0x5eb0
 80020d6: 9a01         	ldr	r2, [sp, #0x4]
 80020d8: 4401         	add	r1, r0
 80020da: f002 021f    	and	r2, r2, #0x1f
 80020de: f8d1 c180    	ldr.w	r12, [r1, #0x180]
 80020e2: 3b10         	subs	r3, #0x10
 80020e4: f8d1 e198    	ldr.w	lr, [r1, #0x198]
 80020e8: f8d1 41b0    	ldr.w	r4, [r1, #0x1b0]
 80020ec: fa4c f602    	asr.w	r6, r12, r2
 80020f0: f8d1 51c8    	ldr.w	r5, [r1, #0x1c8]
 80020f4: f8c1 6180    	str.w	r6, [r1, #0x180]
 80020f8: fa4e f602    	asr.w	r6, lr, r2
 80020fc: f8c1 6198    	str.w	r6, [r1, #0x198]
 8002100: fa44 f402    	asr.w	r4, r4, r2
 8002104: f8d1 6228    	ldr.w	r6, [r1, #0x228]
 8002108: f8d1 9240    	ldr.w	r9, [r1, #0x240]
 800210c: f8c1 41b0    	str.w	r4, [r1, #0x1b0]
 8002110: fa45 f402    	asr.w	r4, r5, r2
 8002114: f8d1 5258    	ldr.w	r5, [r1, #0x258]
 8002118: fa46 f602    	asr.w	r6, r6, r2
 800211c: f8c1 6228    	str.w	r6, [r1, #0x228]
 8002120: fa49 f602    	asr.w	r6, r9, r2
 8002124: f8c1 6240    	str.w	r6, [r1, #0x240]
 8002128: fa45 f602    	asr.w	r6, r5, r2
 800212c: f8d1 81e0    	ldr.w	r8, [r1, #0x1e0]
 8002130: f8c1 6258    	str.w	r6, [r1, #0x258]
 8002134: f8d1 6270    	ldr.w	r6, [r1, #0x270]
 8002138: f8d1 c1f8    	ldr.w	r12, [r1, #0x1f8]
 800213c: f8d1 e210    	ldr.w	lr, [r1, #0x210]
 8002140: fa46 f602    	asr.w	r6, r6, r2
 8002144: f8c1 41c8    	str.w	r4, [r1, #0x1c8]
 8002148: f8c1 6270    	str.w	r6, [r1, #0x270]
 800214c: fa48 f402    	asr.w	r4, r8, r2
 8002150: f8d1 6288    	ldr.w	r6, [r1, #0x288]
 8002154: f8c1 41e0    	str.w	r4, [r1, #0x1e0]
 8002158: fa4c f402    	asr.w	r4, r12, r2
 800215c: f8c1 41f8    	str.w	r4, [r1, #0x1f8]
 8002160: fa4e f402    	asr.w	r4, lr, r2
 8002164: fa46 f602    	asr.w	r6, r6, r2
 8002168: f8c1 4210    	str.w	r4, [r1, #0x210]
 800216c: f8c1 6288    	str.w	r6, [r1, #0x288]
 8002170: f8d1 62a0    	ldr.w	r6, [r1, #0x2a0]
 8002174: fa46 f602    	asr.w	r6, r6, r2
 8002178: f8c1 62a0    	str.w	r6, [r1, #0x2a0]
 800217c: f8d1 62b8    	ldr.w	r6, [r1, #0x2b8]
 8002180: fa46 f602    	asr.w	r6, r6, r2
 8002184: f8c1 62b8    	str.w	r6, [r1, #0x2b8]
 8002188: f8d1 62d0    	ldr.w	r6, [r1, #0x2d0]
 800218c: fa46 f602    	asr.w	r6, r6, r2
 8002190: f8c1 62d0    	str.w	r6, [r1, #0x2d0]
 8002194: f8d1 62e8    	ldr.w	r6, [r1, #0x2e8]
 8002198: fa46 f602    	asr.w	r6, r6, r2
 800219c: f8c1 62e8    	str.w	r6, [r1, #0x2e8]
 80021a0: f501 71c0    	add.w	r1, r1, #0x180
 80021a4: d19b         	bne	0x80020de <raylar_crcl_encoder_service::benchmark::shift_only::h394b3429a2171dca+0xfe> @ imm = #-0xca
 80021a6: 6901         	ldr	r1, [r0, #0x10]
 80021a8: f44f 6380    	mov.w	r3, #0x400
 80021ac: 9101         	str	r1, [sp, #0x4]
 80021ae: a901         	add	r1, sp, #0x4
 80021b0: f645 7174    	movw	r1, #0x5f74
 80021b4: 9a01         	ldr	r2, [sp, #0x4]
 80021b6: 4401         	add	r1, r0
 80021b8: f002 021f    	and	r2, r2, #0x1f
 80021bc: f8d1 60c0    	ldr.w	r6, [r1, #0xc0]
 80021c0: 3b10         	subs	r3, #0x10
 80021c2: f8d1 50d8    	ldr.w	r5, [r1, #0xd8]
 80021c6: f8d1 40f0    	ldr.w	r4, [r1, #0xf0]
 80021ca: fa46 f602    	asr.w	r6, r6, r2
 80021ce: f8d1 c108    	ldr.w	r12, [r1, #0x108]
 80021d2: f8c1 60c0    	str.w	r6, [r1, #0xc0]
 80021d6: fa45 f502    	asr.w	r5, r5, r2
 80021da: f8d1 6150    	ldr.w	r6, [r1, #0x150]
 80021de: fa44 f402    	asr.w	r4, r4, r2
 80021e2: f8c1 50d8    	str.w	r5, [r1, #0xd8]
 80021e6: f8d1 5168    	ldr.w	r5, [r1, #0x168]
 80021ea: f8d1 9180    	ldr.w	r9, [r1, #0x180]
 80021ee: fa46 f602    	asr.w	r6, r6, r2
 80021f2: f8c1 40f0    	str.w	r4, [r1, #0xf0]
 80021f6: fa4c f402    	asr.w	r4, r12, r2
 80021fa: f8d1 c198    	ldr.w	r12, [r1, #0x198]
 80021fe: f8c1 6150    	str.w	r6, [r1, #0x150]
 8002202: fa45 f602    	asr.w	r6, r5, r2
 8002206: f8c1 6168    	str.w	r6, [r1, #0x168]
 800220a: fa49 f602    	asr.w	r6, r9, r2
 800220e: f8c1 6180    	str.w	r6, [r1, #0x180]
 8002212: fa4c f602    	asr.w	r6, r12, r2
 8002216: f8c1 6198    	str.w	r6, [r1, #0x198]
 800221a: f8d1 61b0    	ldr.w	r6, [r1, #0x1b0]
 800221e: f8d1 e120    	ldr.w	lr, [r1, #0x120]
 8002222: f8d1 8138    	ldr.w	r8, [r1, #0x138]
 8002226: fa46 f602    	asr.w	r6, r6, r2
 800222a: f8c1 4108    	str.w	r4, [r1, #0x108]
 800222e: f8c1 61b0    	str.w	r6, [r1, #0x1b0]
 8002232: fa4e f402    	asr.w	r4, lr, r2
 8002236: f8d1 61c8    	ldr.w	r6, [r1, #0x1c8]
 800223a: f8c1 4120    	str.w	r4, [r1, #0x120]
 800223e: fa48 f402    	asr.w	r4, r8, r2
 8002242: f8c1 4138    	str.w	r4, [r1, #0x138]
 8002246: fa46 f602    	asr.w	r6, r6, r2
 800224a: f8c1 61c8    	str.w	r6, [r1, #0x1c8]
 800224e: f8d1 61e0    	ldr.w	r6, [r1, #0x1e0]
 8002252: fa46 f602    	asr.w	r6, r6, r2
 8002256: f8c1 61e0    	str.w	r6, [r1, #0x1e0]
 800225a: f8d1 61f8    	ldr.w	r6, [r1, #0x1f8]
 800225e: fa46 f602    	asr.w	r6, r6, r2
 8002262: f8c1 61f8    	str.w	r6, [r1, #0x1f8]
 8002266: f8d1 6210    	ldr.w	r6, [r1, #0x210]
 800226a: fa46 f602    	asr.w	r6, r6, r2
 800226e: f8c1 6210    	str.w	r6, [r1, #0x210]
 8002272: f8d1 6228    	ldr.w	r6, [r1, #0x228]
 8002276: fa46 f602    	asr.w	r6, r6, r2
 800227a: f8c1 6228    	str.w	r6, [r1, #0x228]
 800227e: f501 71c0    	add.w	r1, r1, #0x180
 8002282: d19b         	bne	0x80021bc <raylar_crcl_encoder_service::benchmark::shift_only::h394b3429a2171dca+0x1dc> @ imm = #-0xca
 8002284: 6981         	ldr	r1, [r0, #0x18]
 8002286: f44f 6380    	mov.w	r3, #0x400
 800228a: 9101         	str	r1, [sp, #0x4]
 800228c: a901         	add	r1, sp, #0x4
 800228e: f645 61b8    	movw	r1, #0x5eb8
 8002292: 9a01         	ldr	r2, [sp, #0x4]
 8002294: 4401         	add	r1, r0
 8002296: f002 021f    	and	r2, r2, #0x1f
 800229a: f8d1 6180    	ldr.w	r6, [r1, #0x180]
 800229e: 3b10         	subs	r3, #0x10
 80022a0: f8d1 5198    	ldr.w	r5, [r1, #0x198]
 80022a4: f8d1 41b0    	ldr.w	r4, [r1, #0x1b0]
 80022a8: fa46 f602    	asr.w	r6, r6, r2
 80022ac: f8d1 c1c8    	ldr.w	r12, [r1, #0x1c8]
 80022b0: f8c1 6180    	str.w	r6, [r1, #0x180]
 80022b4: fa45 f502    	asr.w	r5, r5, r2
 80022b8: f8d1 6210    	ldr.w	r6, [r1, #0x210]
 80022bc: fa44 f402    	asr.w	r4, r4, r2
 80022c0: f8c1 5198    	str.w	r5, [r1, #0x198]
 80022c4: f8d1 5228    	ldr.w	r5, [r1, #0x228]
 80022c8: f8d1 9240    	ldr.w	r9, [r1, #0x240]
 80022cc: fa46 f602    	asr.w	r6, r6, r2
 80022d0: f8c1 41b0    	str.w	r4, [r1, #0x1b0]
 80022d4: fa4c f402    	asr.w	r4, r12, r2
 80022d8: f8d1 c258    	ldr.w	r12, [r1, #0x258]
 80022dc: f8c1 6210    	str.w	r6, [r1, #0x210]
 80022e0: fa45 f602    	asr.w	r6, r5, r2
 80022e4: f8c1 6228    	str.w	r6, [r1, #0x228]
 80022e8: fa49 f602    	asr.w	r6, r9, r2
 80022ec: f8c1 6240    	str.w	r6, [r1, #0x240]
 80022f0: fa4c f602    	asr.w	r6, r12, r2
 80022f4: f8c1 6258    	str.w	r6, [r1, #0x258]
 80022f8: f8d1 6270    	ldr.w	r6, [r1, #0x270]
 80022fc: f8d1 e1e0    	ldr.w	lr, [r1, #0x1e0]
 8002300: f8d1 81f8    	ldr.w	r8, [r1, #0x1f8]
 8002304: fa46 f602    	asr.w	r6, r6, r2
 8002308: f8c1 41c8    	str.w	r4, [r1, #0x1c8]
 800230c: f8c1 6270    	str.w	r6, [r1, #0x270]
 8002310: fa4e f402    	asr.w	r4, lr, r2
 8002314: f8d1 6288    	ldr.w	r6, [r1, #0x288]
 8002318: f8c1 41e0    	str.w	r4, [r1, #0x1e0]
 800231c: fa48 f402    	asr.w	r4, r8, r2
 8002320: f8c1 41f8    	str.w	r4, [r1, #0x1f8]
 8002324: fa46 f602    	asr.w	r6, r6, r2
 8002328: f8c1 6288    	str.w	r6, [r1, #0x288]
 800232c: f8d1 62a0    	ldr.w	r6, [r1, #0x2a0]
 8002330: fa46 f602    	asr.w	r6, r6, r2
 8002334: f8c1 62a0    	str.w	r6, [r1, #0x2a0]
 8002338: f8d1 62b8    	ldr.w	r6, [r1, #0x2b8]
 800233c: fa46 f602    	asr.w	r6, r6, r2
 8002340: f8c1 62b8    	str.w	r6, [r1, #0x2b8]
 8002344: f8d1 62d0    	ldr.w	r6, [r1, #0x2d0]
 8002348: fa46 f602    	asr.w	r6, r6, r2
 800234c: f8c1 62d0    	str.w	r6, [r1, #0x2d0]
 8002350: f8d1 62e8    	ldr.w	r6, [r1, #0x2e8]
 8002354: fa46 f602    	asr.w	r6, r6, r2
 8002358: f8c1 62e8    	str.w	r6, [r1, #0x2e8]
 800235c: f501 71c0    	add.w	r1, r1, #0x180
 8002360: d19b         	bne	0x800229a <raylar_crcl_encoder_service::benchmark::shift_only::h394b3429a2171dca+0x2ba> @ imm = #-0xca
 8002362: 6a01         	ldr	r1, [r0, #0x20]
 8002364: f44f 6380    	mov.w	r3, #0x400
 8002368: 9101         	str	r1, [sp, #0x4]
 800236a: a901         	add	r1, sp, #0x4
 800236c: f645 717c    	movw	r1, #0x5f7c
 8002370: 9a01         	ldr	r2, [sp, #0x4]
 8002372: 4401         	add	r1, r0
 8002374: f002 021f    	and	r2, r2, #0x1f
 8002378: f8d1 60c0    	ldr.w	r6, [r1, #0xc0]
 800237c: 3b10         	subs	r3, #0x10
 800237e: f8d1 50d8    	ldr.w	r5, [r1, #0xd8]
 8002382: f8d1 40f0    	ldr.w	r4, [r1, #0xf0]
 8002386: fa46 f602    	asr.w	r6, r6, r2
 800238a: f8d1 c108    	ldr.w	r12, [r1, #0x108]
 800238e: f8c1 60c0    	str.w	r6, [r1, #0xc0]
 8002392: fa45 f502    	asr.w	r5, r5, r2
 8002396: f8d1 6150    	ldr.w	r6, [r1, #0x150]
 800239a: fa44 f402    	asr.w	r4, r4, r2
 800239e: f8c1 50d8    	str.w	r5, [r1, #0xd8]
 80023a2: f8d1 5168    	ldr.w	r5, [r1, #0x168]
 80023a6: f8d1 9180    	ldr.w	r9, [r1, #0x180]
 80023aa: fa46 f602    	asr.w	r6, r6, r2
 80023ae: f8c1 40f0    	str.w	r4, [r1, #0xf0]
 80023b2: fa4c f402    	asr.w	r4, r12, r2
 80023b6: f8d1 c198    	ldr.w	r12, [r1, #0x198]
 80023ba: f8c1 6150    	str.w	r6, [r1, #0x150]
 80023be: fa45 f602    	asr.w	r6, r5, r2
 80023c2: f8c1 6168    	str.w	r6, [r1, #0x168]
 80023c6: fa49 f602    	asr.w	r6, r9, r2
 80023ca: f8c1 6180    	str.w	r6, [r1, #0x180]
 80023ce: fa4c f602    	asr.w	r6, r12, r2
 80023d2: f8c1 6198    	str.w	r6, [r1, #0x198]
 80023d6: f8d1 61b0    	ldr.w	r6, [r1, #0x1b0]
 80023da: f8d1 e120    	ldr.w	lr, [r1, #0x120]
 80023de: f8d1 8138    	ldr.w	r8, [r1, #0x138]
 80023e2: fa46 f602    	asr.w	r6, r6, r2
 80023e6: f8c1 4108    	str.w	r4, [r1, #0x108]
 80023ea: f8c1 61b0    	str.w	r6, [r1, #0x1b0]
 80023ee: fa4e f402    	asr.w	r4, lr, r2
 80023f2: f8d1 61c8    	ldr.w	r6, [r1, #0x1c8]
 80023f6: f8c1 4120    	str.w	r4, [r1, #0x120]
 80023fa: fa48 f402    	asr.w	r4, r8, r2
 80023fe: f8c1 4138    	str.w	r4, [r1, #0x138]
 8002402: fa46 f602    	asr.w	r6, r6, r2
 8002406: f8c1 61c8    	str.w	r6, [r1, #0x1c8]
 800240a: f8d1 61e0    	ldr.w	r6, [r1, #0x1e0]
 800240e: fa46 f602    	asr.w	r6, r6, r2
 8002412: f8c1 61e0    	str.w	r6, [r1, #0x1e0]
 8002416: f8d1 61f8    	ldr.w	r6, [r1, #0x1f8]
 800241a: fa46 f602    	asr.w	r6, r6, r2
 800241e: f8c1 61f8    	str.w	r6, [r1, #0x1f8]
 8002422: f8d1 6210    	ldr.w	r6, [r1, #0x210]
 8002426: fa46 f602    	asr.w	r6, r6, r2
 800242a: f8c1 6210    	str.w	r6, [r1, #0x210]
 800242e: f8d1 6228    	ldr.w	r6, [r1, #0x228]
 8002432: fa46 f602    	asr.w	r6, r6, r2
 8002436: f8c1 6228    	str.w	r6, [r1, #0x228]
 800243a: f501 71c0    	add.w	r1, r1, #0x180
 800243e: d19b         	bne	0x8002378 <raylar_crcl_encoder_service::benchmark::shift_only::h394b3429a2171dca+0x398> @ imm = #-0xca
 8002440: f246 0128    	movw	r1, #0x6028
 8002444: 4401         	add	r1, r0
 8002446: 9100         	str	r1, [sp]
 8002448: 4669         	mov	r1, sp
 800244a: f246 012c    	movw	r1, #0x602c
 800244e: 5840         	ldr	r0, [r0, r1]
 8002450: b002         	add	sp, #0x8
 8002452: e8bd 0b00    	pop.w	{r8, r9, r11}
 8002456: bdf0         	pop	{r4, r5, r6, r7, pc}

08002458 <raylar_crcl_encoder_service::benchmark::encode_bits::h865b9e06648afa8c>:
 8002458: b5f0         	push	{r4, r5, r6, r7, lr}
 800245a: af03         	add	r7, sp, #0xc
 800245c: e92d 0f00    	push.w	{r8, r9, r10, r11}
 8002460: b097         	sub	sp, #0x5c
 8002462: e9cd 1210    	strd	r1, r2, [sp, #64]
 8002466: 4601         	mov	r1, r0
 8002468: f500 30b8    	add.w	r0, r0, #0x17000
 800246c: f501 33e0    	add.w	r3, r1, #0x1c000
 8002470: 3010         	adds	r0, #0x10
 8002472: 900e         	str	r0, [sp, #0x38]
 8002474: f24c 0028    	movw	r0, #0xc028
 8002478: 180a         	adds	r2, r1, r0
 800247a: 2058         	movs	r0, #0x58
 800247c: f641 6658    	movw	r6, #0x1e58
 8002480: f641 6556    	movw	r5, #0x1e56
 8002484: f04f 3cff    	mov.w	r12, #0xffffffff
 8002488: f04f 0a00    	mov.w	r10, #0x0
 800248c: 9012         	str	r0, [sp, #0x48]
 800248e: 9812         	ldr	r0, [sp, #0x48]
 8002490: 42b0         	cmp	r0, r6
 8002492: f200 81d9    	bhi.w	0x8002848 <raylar_crcl_encoder_service::benchmark::encode_bits::h865b9e06648afa8c+0x3f0> @ imm = #0x3b2
 8002496: 9812         	ldr	r0, [sp, #0x48]
 8002498: 42a8         	cmp	r0, r5
 800249a: f200 8191    	bhi.w	0x80027c0 <raylar_crcl_encoder_service::benchmark::encode_bits::h865b9e06648afa8c+0x368> @ imm = #0x322
 800249e: eb01 00ca    	add.w	r0, r1, r10, lsl #3
 80024a2: 920b         	str	r2, [sp, #0x2c]
 80024a4: f04f 0800    	mov.w	r8, #0x0
 80024a8: 9104         	str	r1, [sp, #0x10]
 80024aa: 7900         	ldrb	r0, [r0, #0x4]
 80024ac: 900d         	str	r0, [sp, #0x34]
 80024ae: f10a 0001    	add.w	r0, r10, #0x1
 80024b2: 9003         	str	r0, [sp, #0xc]
 80024b4: f852 2020    	ldr.w	r2, [r2, r0, lsl #2]
 80024b8: 9812         	ldr	r0, [sp, #0x48]
 80024ba: eba6 0e00    	sub.w	lr, r6, r0
 80024be: eb03 0900    	add.w	r9, r3, r0
 80024c2: 260a         	movs	r6, #0xa
 80024c4: f018 0407    	ands	r4, r8, #0x7
 80024c8: ea4f 03d8    	lsr.w	r3, r8, #0x3
 80024cc: d105         	bne	0x80024da <raylar_crcl_encoder_service::benchmark::encode_bits::h865b9e06648afa8c+0x82> @ imm = #0xa
 80024ce: 4573         	cmp	r3, lr
 80024d0: f080 81cd    	bhs.w	0x800286e <raylar_crcl_encoder_service::benchmark::encode_bits::h865b9e06648afa8c+0x416> @ imm = #0x39a
 80024d4: 2000         	movs	r0, #0x0
 80024d6: f809 0003    	strb.w	r0, [r9, r3]
 80024da: 4573         	cmp	r3, lr
 80024dc: f080 81bf    	bhs.w	0x800285e <raylar_crcl_encoder_service::benchmark::encode_bits::h865b9e06648afa8c+0x406> @ imm = #0x37e
 80024e0: f1c4 0008    	rsb.w	r0, r4, #0x8
 80024e4: 4634         	mov	r4, r6
 80024e6: 42b0         	cmp	r0, r6
 80024e8: bf38         	it	lo
 80024ea: 4604         	movlo	r4, r0
 80024ec: 1b36         	subs	r6, r6, r4
 80024ee: eba0 0004    	sub.w	r0, r0, r4
 80024f2: f006 011f    	and	r1, r6, #0x1f
 80024f6: fa0c f504    	lsl.w	r5, r12, r4
 80024fa: f819 b003    	ldrb.w	r11, [r9, r3]
 80024fe: f000 0007    	and	r0, r0, #0x7
 8002502: fa22 f101    	lsr.w	r1, r2, r1
 8002506: ea21 0105    	bic.w	r1, r1, r5
 800250a: 44a0         	add	r8, r4
 800250c: fa01 f000    	lsl.w	r0, r1, r0
 8002510: ea40 000b    	orr.w	r0, r0, r11
 8002514: f809 0003    	strb.w	r0, [r9, r3]
 8002518: d1d4         	bne	0x80024c4 <raylar_crcl_encoder_service::benchmark::encode_bits::h865b9e06648afa8c+0x6c> @ imm = #-0x58
 800251a: 980b         	ldr	r0, [sp, #0x2c]
 800251c: f10a 0101    	add.w	r1, r10, #0x1
 8002520: f04f 0b01    	mov.w	r11, #0x1
 8002524: f850 3021    	ldr.w	r3, [r0, r1, lsl #2]
 8002528: eb00 008a    	add.w	r0, r0, r10, lsl #2
 800252c: 900a         	str	r0, [sp, #0x28]
 800252e: ebca 208a    	rsb	r0, r10, r10, lsl #10
 8002532: 990e         	ldr	r1, [sp, #0x38]
 8002534: eb01 0080    	add.w	r0, r1, r0, lsl #2
 8002538: 990d         	ldr	r1, [sp, #0x34]
 800253a: 9009         	str	r0, [sp, #0x24]
 800253c: f001 041f    	and	r4, r1, #0x1f
 8002540: 9408         	str	r4, [sp, #0x20]
 8002542: fa0c f004    	lsl.w	r0, r12, r4
 8002546: 43c0         	mvns	r0, r0
 8002548: 9005         	str	r0, [sp, #0x14]
 800254a: ea4f 00ce    	lsl.w	r0, lr, #0x3
 800254e: 900f         	str	r0, [sp, #0x3c]
 8002550: 1c48         	adds	r0, r1, #0x1
 8002552: 900c         	str	r0, [sp, #0x30]
 8002554: fa0b f004    	lsl.w	r0, r11, r4
 8002558: 9007         	str	r0, [sp, #0x1c]
 800255a: 3801         	subs	r0, #0x1
 800255c: 9006         	str	r0, [sp, #0x18]
 800255e: 9811         	ldr	r0, [sp, #0x44]
 8002560: b178         	cbz	r0, 0x8002582 <raylar_crcl_encoder_service::benchmark::encode_bits::h865b9e06648afa8c+0x12a> @ imm = #0x1e
 8002562: eb0b 004b    	add.w	r0, r11, r11, lsl #1
 8002566: 990a         	ldr	r1, [sp, #0x28]
 8002568: eb01 00c0    	add.w	r0, r1, r0, lsl #3
 800256c: 6842         	ldr	r2, [r0, #0x4]
 800256e: 1ad0         	subs	r0, r2, r3
 8002570: 0043         	lsls	r3, r0, #0x1
 8002572: ea83 70e0    	eor.w	r0, r3, r0, asr #31
 8002576: 4613         	mov	r3, r2
 8002578: 9910         	ldr	r1, [sp, #0x40]
 800257a: fa20 fa04    	lsr.w	r10, r0, r4
 800257e: b941         	cbnz	r1, 0x8002592 <raylar_crcl_encoder_service::benchmark::encode_bits::h865b9e06648afa8c+0x13a> @ imm = #0x10
 8002580: e04e         	b	0x8002620 <raylar_crcl_encoder_service::benchmark::encode_bits::h865b9e06648afa8c+0x1c8> @ imm = #0x9c
 8002582: 9809         	ldr	r0, [sp, #0x24]
 8002584: f850 002b    	ldr.w	r0, [r0, r11, lsl #2]
 8002588: 9910         	ldr	r1, [sp, #0x40]
 800258a: fa20 fa04    	lsr.w	r10, r0, r4
 800258e: 2900         	cmp	r1, #0x0
 8002590: d046         	beq	0x8002620 <raylar_crcl_encoder_service::benchmark::encode_bits::h865b9e06648afa8c+0x1c8> @ imm = #0x8c
 8002592: 990c         	ldr	r1, [sp, #0x30]
 8002594: eb01 060a    	add.w	r6, r1, r10
 8002598: 2e21         	cmp	r6, #0x21
 800259a: d241         	bhs	0x8002620 <raylar_crcl_encoder_service::benchmark::encode_bits::h865b9e06648afa8c+0x1c8> @ imm = #0x82
 800259c: eb16 0208    	adds.w	r2, r6, r8
 80025a0: f080 8126    	bhs.w	0x80027f0 <raylar_crcl_encoder_service::benchmark::encode_bits::h865b9e06648afa8c+0x398> @ imm = #0x24c
 80025a4: 990f         	ldr	r1, [sp, #0x3c]
 80025a6: 428a         	cmp	r2, r1
 80025a8: f200 8122    	bhi.w	0x80027f0 <raylar_crcl_encoder_service::benchmark::encode_bits::h865b9e06648afa8c+0x398> @ imm = #0x244
 80025ac: 9302         	str	r3, [sp, #0x8]
 80025ae: b37e         	cbz	r6, 0x8002610 <raylar_crcl_encoder_service::benchmark::encode_bits::h865b9e06648afa8c+0x1b8> @ imm = #0x5e
 80025b0: 9906         	ldr	r1, [sp, #0x18]
 80025b2: 4008         	ands	r0, r1
 80025b4: 9907         	ldr	r1, [sp, #0x1c]
 80025b6: ea40 0a01    	orr.w	r10, r0, r1
 80025ba: f018 0007    	ands	r0, r8, #0x7
 80025be: ea4f 03d8    	lsr.w	r3, r8, #0x3
 80025c2: d105         	bne	0x80025d0 <raylar_crcl_encoder_service::benchmark::encode_bits::h865b9e06648afa8c+0x178> @ imm = #0xa
 80025c4: 4573         	cmp	r3, lr
 80025c6: f080 8152    	bhs.w	0x800286e <raylar_crcl_encoder_service::benchmark::encode_bits::h865b9e06648afa8c+0x416> @ imm = #0x2a4
 80025ca: 2100         	movs	r1, #0x0
 80025cc: f809 1003    	strb.w	r1, [r9, r3]
 80025d0: 4573         	cmp	r3, lr
 80025d2: f080 8144    	bhs.w	0x800285e <raylar_crcl_encoder_service::benchmark::encode_bits::h865b9e06648afa8c+0x406> @ imm = #0x288
 80025d6: f1c0 0008    	rsb.w	r0, r0, #0x8
 80025da: 4634         	mov	r4, r6
 80025dc: 42b0         	cmp	r0, r6
 80025de: bf38         	it	lo
 80025e0: 4604         	movlo	r4, r0
 80025e2: 1b36         	subs	r6, r6, r4
 80025e4: eba0 0004    	sub.w	r0, r0, r4
 80025e8: f006 011f    	and	r1, r6, #0x1f
 80025ec: fa0c f504    	lsl.w	r5, r12, r4
 80025f0: f819 2003    	ldrb.w	r2, [r9, r3]
 80025f4: f000 0007    	and	r0, r0, #0x7
 80025f8: fa2a f101    	lsr.w	r1, r10, r1
 80025fc: ea21 0105    	bic.w	r1, r1, r5
 8002600: 44a0         	add	r8, r4
 8002602: fa01 f000    	lsl.w	r0, r1, r0
 8002606: ea40 0002    	orr.w	r0, r0, r2
 800260a: f809 0003    	strb.w	r0, [r9, r3]
 800260e: d1d4         	bne	0x80025ba <raylar_crcl_encoder_service::benchmark::encode_bits::h865b9e06648afa8c+0x162> @ imm = #-0x58
 8002610: 9b02         	ldr	r3, [sp, #0x8]
 8002612: f10b 0b01    	add.w	r11, r11, #0x1
 8002616: 9c08         	ldr	r4, [sp, #0x20]
 8002618: f5bb 6f80    	cmp.w	r11, #0x400
 800261c: d19f         	bne	0x800255e <raylar_crcl_encoder_service::benchmark::encode_bits::h865b9e06648afa8c+0x106> @ imm = #-0xc2
 800261e: e090         	b	0x8002742 <raylar_crcl_encoder_service::benchmark::encode_bits::h865b9e06648afa8c+0x2ea> @ imm = #0x120
 8002620: 990f         	ldr	r1, [sp, #0x3c]
 8002622: eba1 0208    	sub.w	r2, r1, r8
 8002626: 4592         	cmp	r10, r2
 8002628: f080 80d6    	bhs.w	0x80027d8 <raylar_crcl_encoder_service::benchmark::encode_bits::h865b9e06648afa8c+0x380> @ imm = #0x1ac
 800262c: f1ba 0f00    	cmp.w	r10, #0x0
 8002630: d028         	beq	0x8002684 <raylar_crcl_encoder_service::benchmark::encode_bits::h865b9e06648afa8c+0x22c> @ imm = #0x50
 8002632: f1ba 0f20    	cmp.w	r10, #0x20
 8002636: 4656         	mov	r6, r10
 8002638: bf28         	it	hs
 800263a: 2620         	movhs	r6, #0x20
 800263c: eb18 0206    	adds.w	r2, r8, r6
 8002640: f080 80ca    	bhs.w	0x80027d8 <raylar_crcl_encoder_service::benchmark::encode_bits::h865b9e06648afa8c+0x380> @ imm = #0x194
 8002644: 990f         	ldr	r1, [sp, #0x3c]
 8002646: 428a         	cmp	r2, r1
 8002648: f200 80c6    	bhi.w	0x80027d8 <raylar_crcl_encoder_service::benchmark::encode_bits::h865b9e06648afa8c+0x380> @ imm = #0x18c
 800264c: 461d         	mov	r5, r3
 800264e: 4632         	mov	r2, r6
 8002650: f018 0407    	ands	r4, r8, #0x7
 8002654: ea4f 03d8    	lsr.w	r3, r8, #0x3
 8002658: d105         	bne	0x8002666 <raylar_crcl_encoder_service::benchmark::encode_bits::h865b9e06648afa8c+0x20e> @ imm = #0xa
 800265a: 4573         	cmp	r3, lr
 800265c: f080 8107    	bhs.w	0x800286e <raylar_crcl_encoder_service::benchmark::encode_bits::h865b9e06648afa8c+0x416> @ imm = #0x20e
 8002660: 2100         	movs	r1, #0x0
 8002662: f809 1003    	strb.w	r1, [r9, r3]
 8002666: 4573         	cmp	r3, lr
 8002668: f080 80f9    	bhs.w	0x800285e <raylar_crcl_encoder_service::benchmark::encode_bits::h865b9e06648afa8c+0x406> @ imm = #0x1f2
 800266c: f1c4 0308    	rsb.w	r3, r4, #0x8
 8002670: 4293         	cmp	r3, r2
 8002672: bf28         	it	hs
 8002674: 4613         	movhs	r3, r2
 8002676: 4498         	add	r8, r3
 8002678: 1ad2         	subs	r2, r2, r3
 800267a: d1e9         	bne	0x8002650 <raylar_crcl_encoder_service::benchmark::encode_bits::h865b9e06648afa8c+0x1f8> @ imm = #-0x2e
 800267c: ebba 0a06    	subs.w	r10, r10, r6
 8002680: 462b         	mov	r3, r5
 8002682: d1d6         	bne	0x8002632 <raylar_crcl_encoder_service::benchmark::encode_bits::h865b9e06648afa8c+0x1da> @ imm = #-0x54
 8002684: 990f         	ldr	r1, [sp, #0x3c]
 8002686: 4588         	cmp	r8, r1
 8002688: f080 80a6    	bhs.w	0x80027d8 <raylar_crcl_encoder_service::benchmark::encode_bits::h865b9e06648afa8c+0x380> @ imm = #0x14c
 800268c: f018 0207    	ands	r2, r8, #0x7
 8002690: 9302         	str	r3, [sp, #0x8]
 8002692: ea4f 03d8    	lsr.w	r3, r8, #0x3
 8002696: d105         	bne	0x80026a4 <raylar_crcl_encoder_service::benchmark::encode_bits::h865b9e06648afa8c+0x24c> @ imm = #0xa
 8002698: 4573         	cmp	r3, lr
 800269a: f080 80e8    	bhs.w	0x800286e <raylar_crcl_encoder_service::benchmark::encode_bits::h865b9e06648afa8c+0x416> @ imm = #0x1d0
 800269e: 2100         	movs	r1, #0x0
 80026a0: f809 1003    	strb.w	r1, [r9, r3]
 80026a4: 4573         	cmp	r3, lr
 80026a6: f080 80da    	bhs.w	0x800285e <raylar_crcl_encoder_service::benchmark::encode_bits::h865b9e06648afa8c+0x406> @ imm = #0x1b4
 80026aa: f819 1003    	ldrb.w	r1, [r9, r3]
 80026ae: f082 0207    	eor	r2, r2, #0x7
 80026b2: 2401         	movs	r4, #0x1
 80026b4: fa04 f202    	lsl.w	r2, r4, r2
 80026b8: 4311         	orrs	r1, r2
 80026ba: f809 1003    	strb.w	r1, [r9, r3]
 80026be: 990d         	ldr	r1, [sp, #0x34]
 80026c0: 2920         	cmp	r1, #0x20
 80026c2: f200 80a9    	bhi.w	0x8002818 <raylar_crcl_encoder_service::benchmark::encode_bits::h865b9e06648afa8c+0x3c0> @ imm = #0x152
 80026c6: f108 0301    	add.w	r3, r8, #0x1
 80026ca: 185a         	adds	r2, r3, r1
 80026cc: 4542         	cmp	r2, r8
 80026ce: f240 80a3    	bls.w	0x8002818 <raylar_crcl_encoder_service::benchmark::encode_bits::h865b9e06648afa8c+0x3c0> @ imm = #0x146
 80026d2: 9c0f         	ldr	r4, [sp, #0x3c]
 80026d4: 42a2         	cmp	r2, r4
 80026d6: f200 809f    	bhi.w	0x8002818 <raylar_crcl_encoder_service::benchmark::encode_bits::h865b9e06648afa8c+0x3c0> @ imm = #0x13e
 80026da: b381         	cbz	r1, 0x800273e <raylar_crcl_encoder_service::benchmark::encode_bits::h865b9e06648afa8c+0x2e6> @ imm = #0x60
 80026dc: 9a05         	ldr	r2, [sp, #0x14]
 80026de: 4698         	mov	r8, r3
 80026e0: ea00 0a02    	and.w	r10, r0, r2
 80026e4: 460a         	mov	r2, r1
 80026e6: f018 0007    	ands	r0, r8, #0x7
 80026ea: ea4f 03d8    	lsr.w	r3, r8, #0x3
 80026ee: d105         	bne	0x80026fc <raylar_crcl_encoder_service::benchmark::encode_bits::h865b9e06648afa8c+0x2a4> @ imm = #0xa
 80026f0: 4573         	cmp	r3, lr
 80026f2: f080 80bc    	bhs.w	0x800286e <raylar_crcl_encoder_service::benchmark::encode_bits::h865b9e06648afa8c+0x416> @ imm = #0x178
 80026f6: 2100         	movs	r1, #0x0
 80026f8: f809 1003    	strb.w	r1, [r9, r3]
 80026fc: 4573         	cmp	r3, lr
 80026fe: f080 80ae    	bhs.w	0x800285e <raylar_crcl_encoder_service::benchmark::encode_bits::h865b9e06648afa8c+0x406> @ imm = #0x15c
 8002702: f1c0 0008    	rsb.w	r0, r0, #0x8
 8002706: 4611         	mov	r1, r2
 8002708: 4290         	cmp	r0, r2
 800270a: bf38         	it	lo
 800270c: 4601         	movlo	r1, r0
 800270e: 1a52         	subs	r2, r2, r1
 8002710: eba0 0001    	sub.w	r0, r0, r1
 8002714: f002 051f    	and	r5, r2, #0x1f
 8002718: fa0c f401    	lsl.w	r4, r12, r1
 800271c: f819 6003    	ldrb.w	r6, [r9, r3]
 8002720: f000 0007    	and	r0, r0, #0x7
 8002724: fa2a f505    	lsr.w	r5, r10, r5
 8002728: ea25 0404    	bic.w	r4, r5, r4
 800272c: 4488         	add	r8, r1
 800272e: fa04 f000    	lsl.w	r0, r4, r0
 8002732: ea40 0006    	orr.w	r0, r0, r6
 8002736: f809 0003    	strb.w	r0, [r9, r3]
 800273a: d1d4         	bne	0x80026e6 <raylar_crcl_encoder_service::benchmark::encode_bits::h865b9e06648afa8c+0x28e> @ imm = #-0x58
 800273c: e768         	b	0x8002610 <raylar_crcl_encoder_service::benchmark::encode_bits::h865b9e06648afa8c+0x1b8> @ imm = #-0x130
 800273e: 4698         	mov	r8, r3
 8002740: e766         	b	0x8002610 <raylar_crcl_encoder_service::benchmark::encode_bits::h865b9e06648afa8c+0x1b8> @ imm = #-0x134
 8002742: f1c8 0000    	rsb.w	r0, r8, #0x0
 8002746: f000 0107    	and	r1, r0, #0x7
 800274a: eb18 0001    	adds.w	r0, r8, r1
 800274e: d26f         	bhs	0x8002830 <raylar_crcl_encoder_service::benchmark::encode_bits::h865b9e06648afa8c+0x3d8> @ imm = #0xde
 8002750: 9a0f         	ldr	r2, [sp, #0x3c]
 8002752: 4290         	cmp	r0, r2
 8002754: d86c         	bhi	0x8002830 <raylar_crcl_encoder_service::benchmark::encode_bits::h865b9e06648afa8c+0x3d8> @ imm = #0xd8
 8002756: b1a1         	cbz	r1, 0x8002782 <raylar_crcl_encoder_service::benchmark::encode_bits::h865b9e06648afa8c+0x32a> @ imm = #0x28
 8002758: f018 0007    	ands	r0, r8, #0x7
 800275c: ea4f 03d8    	lsr.w	r3, r8, #0x3
 8002760: d105         	bne	0x800276e <raylar_crcl_encoder_service::benchmark::encode_bits::h865b9e06648afa8c+0x316> @ imm = #0xa
 8002762: 4573         	cmp	r3, lr
 8002764: f080 8083    	bhs.w	0x800286e <raylar_crcl_encoder_service::benchmark::encode_bits::h865b9e06648afa8c+0x416> @ imm = #0x106
 8002768: 2200         	movs	r2, #0x0
 800276a: f809 2003    	strb.w	r2, [r9, r3]
 800276e: 4573         	cmp	r3, lr
 8002770: d275         	bhs	0x800285e <raylar_crcl_encoder_service::benchmark::encode_bits::h865b9e06648afa8c+0x406> @ imm = #0xea
 8002772: f1c0 0008    	rsb.w	r0, r0, #0x8
 8002776: 4288         	cmp	r0, r1
 8002778: bf28         	it	hs
 800277a: 4608         	movhs	r0, r1
 800277c: 4480         	add	r8, r0
 800277e: 1a09         	subs	r1, r1, r0
 8002780: d1ea         	bne	0x8002758 <raylar_crcl_encoder_service::benchmark::encode_bits::h865b9e06648afa8c+0x300> @ imm = #-0x2c
 8002782: 9904         	ldr	r1, [sp, #0x10]
 8002784: f641 6658    	movw	r6, #0x1e58
 8002788: f8dd a00c    	ldr.w	r10, [sp, #0xc]
 800278c: f641 6556    	movw	r5, #0x1e56
 8002790: 9a0b         	ldr	r2, [sp, #0x2c]
 8002792: f501 33e0    	add.w	r3, r1, #0x1c000
 8002796: 9812         	ldr	r0, [sp, #0x48]
 8002798: f1ba 0f05    	cmp.w	r10, #0x5
 800279c: eb00 00d8    	add.w	r0, r0, r8, lsr #3
 80027a0: 9012         	str	r0, [sp, #0x48]
 80027a2: f47f ae74    	bne.w	0x800248e <raylar_crcl_encoder_service::benchmark::encode_bits::h865b9e06648afa8c+0x36> @ imm = #-0x318
 80027a6: 9812         	ldr	r0, [sp, #0x48]
 80027a8: f641 6159    	movw	r1, #0x1e59
 80027ac: 4288         	cmp	r0, r1
 80027ae: d266         	bhs	0x800287e <raylar_crcl_encoder_service::benchmark::encode_bits::h865b9e06648afa8c+0x426> @ imm = #0xcc
 80027b0: 9812         	ldr	r0, [sp, #0x48]
 80027b2: a915         	add	r1, sp, #0x54
 80027b4: e9cd 3015    	strd	r3, r0, [sp, #84]
 80027b8: b017         	add	sp, #0x5c
 80027ba: e8bd 0f00    	pop.w	{r8, r9, r10, r11}
 80027be: bdf0         	pop	{r4, r5, r6, r7, pc}
 80027c0: 2002         	movs	r0, #0x2
 80027c2: f64d 434c    	movw	r3, #0xdc4c
 80027c6: f807 0c29    	strb	r0, [r7, #-41]
 80027ca: f64d 409c    	movw	r0, #0xdc9c
 80027ce: f6c0 0000    	movt	r0, #0x800
 80027d2: f1a7 0229    	sub.w	r2, r7, #0x29
 80027d6: e015         	b	0x8002804 <raylar_crcl_encoder_service::benchmark::encode_bits::h865b9e06648afa8c+0x3ac> @ imm = #0x2a
 80027d8: 2002         	movs	r0, #0x2
 80027da: f64d 434c    	movw	r3, #0xdc4c
 80027de: f807 0c27    	strb	r0, [r7, #-39]
 80027e2: f64d 407c    	movw	r0, #0xdc7c
 80027e6: f6c0 0000    	movt	r0, #0x800
 80027ea: f1a7 0227    	sub.w	r2, r7, #0x27
 80027ee: e009         	b	0x8002804 <raylar_crcl_encoder_service::benchmark::encode_bits::h865b9e06648afa8c+0x3ac> @ imm = #0x12
 80027f0: 2002         	movs	r0, #0x2
 80027f2: f64d 434c    	movw	r3, #0xdc4c
 80027f6: f88d 0050    	strb.w	r0, [sp, #0x50]
 80027fa: f64d 408c    	movw	r0, #0xdc8c
 80027fe: f6c0 0000    	movt	r0, #0x800
 8002802: aa14         	add	r2, sp, #0x50
 8002804: 9000         	str	r0, [sp]
 8002806: f64d 30b0    	movw	r0, #0xdbb0
 800280a: f6c0 0000    	movt	r0, #0x800
 800280e: f6c0 0300    	movt	r3, #0x800
 8002812: 212b         	movs	r1, #0x2b
 8002814: f008 ff06    	bl	0x800b624 <core::result::unwrap_failed> @ imm = #0x8e0c
 8002818: 2002         	movs	r0, #0x2
 800281a: f64d 434c    	movw	r3, #0xdc4c
 800281e: f807 0c26    	strb	r0, [r7, #-38]
 8002822: f64d 406c    	movw	r0, #0xdc6c
 8002826: f6c0 0000    	movt	r0, #0x800
 800282a: f1a7 0226    	sub.w	r2, r7, #0x26
 800282e: e7e9         	b	0x8002804 <raylar_crcl_encoder_service::benchmark::encode_bits::h865b9e06648afa8c+0x3ac> @ imm = #-0x2e
 8002830: 2002         	movs	r0, #0x2
 8002832: f64d 434c    	movw	r3, #0xdc4c
 8002836: f807 0c25    	strb	r0, [r7, #-37]
 800283a: f64d 405c    	movw	r0, #0xdc5c
 800283e: f6c0 0000    	movt	r0, #0x800
 8002842: f1a7 0225    	sub.w	r2, r7, #0x25
 8002846: e7dd         	b	0x8002804 <raylar_crcl_encoder_service::benchmark::encode_bits::h865b9e06648afa8c+0x3ac> @ imm = #-0x46
 8002848: 9812         	ldr	r0, [sp, #0x48]
 800284a: f64d 43ac    	movw	r3, #0xdcac
 800284e: f6c0 0300    	movt	r3, #0x800
 8002852: f641 6158    	movw	r1, #0x1e58
 8002856: f641 6258    	movw	r2, #0x1e58
 800285a: f009 f9ee    	bl	0x800bc3a <core::slice::index::slice_index_fail> @ imm = #0x93dc
 800285e: f24d 62e0    	movw	r2, #0xd6e0
 8002862: f6c0 0200    	movt	r2, #0x800
 8002866: 4618         	mov	r0, r3
 8002868: 4671         	mov	r1, lr
 800286a: f008 fef9    	bl	0x800b660 <core::panicking::panic_bounds_check> @ imm = #0x8df2
 800286e: f24d 62d0    	movw	r2, #0xd6d0
 8002872: f6c0 0200    	movt	r2, #0x800
 8002876: 4618         	mov	r0, r3
 8002878: 4671         	mov	r1, lr
 800287a: f008 fef1    	bl	0x800b660 <core::panicking::panic_bounds_check> @ imm = #0x8de2
 800287e: 9912         	ldr	r1, [sp, #0x48]
 8002880: f64d 433c    	movw	r3, #0xdc3c
 8002884: f6c0 0300    	movt	r3, #0x800
 8002888: 2000         	movs	r0, #0x0
 800288a: f641 6258    	movw	r2, #0x1e58
 800288e: f009 f9d4    	bl	0x800bc3a <core::slice::index::slice_index_fail> @ imm = #0x93a8

08002892 <raylar_crcl_encoder_service::benchmark::pack_header::haa3cc0f32481b1e3>:
 8002892: b5f0         	push	{r4, r5, r6, r7, lr}
 8002894: af03         	add	r7, sp, #0xc
 8002896: f84d 8d04    	str	r8, [sp, #-4]!
 800289a: b082         	sub	sp, #0x8
 800289c: 4604         	mov	r4, r0
 800289e: f500 30e0    	add.w	r0, r0, #0x1c000
 80028a2: 3004         	adds	r0, #0x4
 80028a4: 214c         	movs	r1, #0x4c
 80028a6: f00a f922    	bl	0x800caee <__aeabi_memclr4> @ imm = #0xa244
 80028aa: f245 2043    	movw	r0, #0x5243
 80028ae: f44f 31e0    	mov.w	r1, #0x1c000
 80028b2: f6c4 3042    	movt	r0, #0x4b42
 80028b6: 2201         	movs	r2, #0x1
 80028b8: 5060         	str	r0, [r4, r1]
 80028ba: f64f 41c4    	movw	r1, #0xfcc4
 80028be: f504 30e0    	add.w	r0, r4, #0x1c000
 80028c2: f2c0 0101    	movt	r1, #0x1
 80028c6: f44f 6380    	mov.w	r3, #0x400
 80028ca: f854 c001    	ldr.w	r12, [r4, r1]
 80028ce: 4421         	add	r1, r4
 80028d0: 7102         	strb	r2, [r0, #0x4]
 80028d2: f240 2202    	movw	r2, #0x202
 80028d6: 8303         	strh	r3, [r0, #0x18]
 80028d8: 7b23         	ldrb	r3, [r4, #0xc]
 80028da: f240 2503    	movw	r5, #0x203
 80028de: 8382         	strh	r2, [r0, #0x1c]
 80028e0: 2205         	movs	r2, #0x5
 80028e2: f880 302b    	strb.w	r3, [r0, #0x2b]
 80028e6: f44f 7301    	mov.w	r3, #0x204
 80028ea: 7682         	strb	r2, [r0, #0x1a]
 80028ec: f04f 0858    	mov.w	r8, #0x58
 80028f0: 694a         	ldr	r2, [r1, #0x14]
 80028f2: 8683         	strh	r3, [r0, #0x34]
 80028f4: f240 2305    	movw	r3, #0x205
 80028f8: f8d4 e000    	ldr.w	lr, [r4]
 80028fc: 8505         	strh	r5, [r0, #0x28]
 80028fe: 68a5         	ldr	r5, [r4, #0x8]
 8002900: 6082         	str	r2, [r0, #0x8]
 8002902: 7922         	ldrb	r2, [r4, #0x4]
 8002904: f8a0 3040    	strh.w	r3, [r0, #0x40]
 8002908: f240 2306    	movw	r3, #0x206
 800290c: e9c0 ec08    	strd	lr, r12, [r0, #32]
 8002910: f8a0 304c    	strh.w	r3, [r0, #0x4c]
 8002914: 7d23         	ldrb	r3, [r4, #0x14]
 8002916: 77c2         	strb	r2, [r0, #0x1f]
 8002918: 62c5         	str	r5, [r0, #0x2c]
 800291a: e9d1 5201    	ldrd	r5, r2, [r1, #4]
 800291e: f8d1 c00c    	ldr.w	r12, [r1, #0xc]
 8002922: 6909         	ldr	r1, [r1, #0x10]
 8002924: 6305         	str	r5, [r0, #0x30]
 8002926: 6925         	ldr	r5, [r4, #0x10]
 8002928: f880 3037    	strb.w	r3, [r0, #0x37]
 800292c: 7f23         	ldrb	r3, [r4, #0x1c]
 800292e: 69a6         	ldr	r6, [r4, #0x18]
 8002930: e9c0 520e    	strd	r5, r2, [r0, #56]
 8002934: f894 2024    	ldrb.w	r2, [r4, #0x24]
 8002938: f880 3043    	strb.w	r3, [r0, #0x43]
 800293c: 6a23         	ldr	r3, [r4, #0x20]
 800293e: f8a0 8006    	strh.w	r8, [r0, #0x6]
 8002942: e9c0 6c11    	strd	r6, r12, [r0, #68]
 8002946: f880 204f    	strb.w	r2, [r0, #0x4f]
 800294a: e9c0 3114    	strd	r3, r1, [r0, #80]
 800294e: e9cd 0800    	strd	r0, r8, [sp]
 8002952: 4668         	mov	r0, sp
 8002954: 2058         	movs	r0, #0x58
 8002956: b002         	add	sp, #0x8
 8002958: f85d 8b04    	ldr	r8, [sp], #4
 800295c: bdf0         	pop	{r4, r5, r6, r7, pc}

0800295e <raylar_crcl_encoder_service::benchmark::encode_words::h522ea4cb3ed82c82>:
 800295e: b5f0         	push	{r4, r5, r6, r7, lr}
 8002960: af03         	add	r7, sp, #0xc
 8002962: e92d 0f00    	push.w	{r8, r9, r10, r11}
 8002966: b091         	sub	sp, #0x44
 8002968: f500 31b8    	add.w	r1, r0, #0x17000
 800296c: f500 3ee0    	add.w	lr, r0, #0x1c000
 8002970: f101 0614    	add.w	r6, r1, #0x14
 8002974: f24c 0128    	movw	r1, #0xc028
 8002978: 1845         	adds	r5, r0, r1
 800297a: 9005         	str	r0, [sp, #0x14]
 800297c: 2000         	movs	r0, #0x0
 800297e: 2258         	movs	r2, #0x58
 8002980: f641 6458    	movw	r4, #0x1e58
 8002984: f04f 38ff    	mov.w	r8, #0xffffffff
 8002988: f04f 0c01    	mov.w	r12, #0x1
 800298c: e88d 4060    	stm.w	sp, {r5, r6, lr}
 8002990: 42a2         	cmp	r2, r4
 8002992: f200 8147    	bhi.w	0x8002c24 <raylar_crcl_encoder_service::benchmark::encode_words::h522ea4cb3ed82c82+0x2c6> @ imm = #0x28e
 8002996: 9905         	ldr	r1, [sp, #0x14]
 8002998: eb0e 0902    	add.w	r9, lr, r2
 800299c: 9204         	str	r2, [sp, #0x10]
 800299e: 1aa2         	subs	r2, r4, r2
 80029a0: eb01 01c0    	add.w	r1, r1, r0, lsl #3
 80029a4: f04f 0b0a    	mov.w	r11, #0xa
 80029a8: f04f 0a00    	mov.w	r10, #0x0
 80029ac: 790b         	ldrb	r3, [r1, #0x4]
 80029ae: 1c41         	adds	r1, r0, #0x1
 80029b0: ebc0 2080    	rsb	r0, r0, r0, lsl #10
 80029b4: 9103         	str	r1, [sp, #0xc]
 80029b6: f855 1021    	ldr.w	r1, [r5, r1, lsl #2]
 80029ba: eb06 0080    	add.w	r0, r6, r0, lsl #2
 80029be: f003 061f    	and	r6, r3, #0x1f
 80029c2: 900d         	str	r0, [sp, #0x34]
 80029c4: 9309         	str	r3, [sp, #0x24]
 80029c6: fa08 f006    	lsl.w	r0, r8, r6
 80029ca: 058c         	lsls	r4, r1, #0x16
 80029cc: 43c0         	mvns	r0, r0
 80029ce: 1c59         	adds	r1, r3, #0x1
 80029d0: 9006         	str	r0, [sp, #0x18]
 80029d2: fa0c f006    	lsl.w	r0, r12, r6
 80029d6: 910b         	str	r1, [sp, #0x2c]
 80029d8: 2100         	movs	r1, #0x0
 80029da: 9008         	str	r0, [sp, #0x20]
 80029dc: 3801         	subs	r0, #0x1
 80029de: 960c         	str	r6, [sp, #0x30]
 80029e0: 9007         	str	r0, [sp, #0x1c]
 80029e2: 980d         	ldr	r0, [sp, #0x34]
 80029e4: f850 602a    	ldr.w	r6, [r0, r10, lsl #2]
 80029e8: 980c         	ldr	r0, [sp, #0x30]
 80029ea: fa26 f500    	lsr.w	r5, r6, r0
 80029ee: 980b         	ldr	r0, [sp, #0x2c]
 80029f0: 1943         	adds	r3, r0, r5
 80029f2: 2b21         	cmp	r3, #0x21
 80029f4: d23c         	bhs	0x8002a70 <raylar_crcl_encoder_service::benchmark::encode_words::h522ea4cb3ed82c82+0x112> @ imm = #0x78
 80029f6: 2b00         	cmp	r3, #0x0
 80029f8: f000 80d2    	beq.w	0x8002ba0 <raylar_crcl_encoder_service::benchmark::encode_words::h522ea4cb3ed82c82+0x242> @ imm = #0x1a4
 80029fc: 9807         	ldr	r0, [sp, #0x1c]
 80029fe: 4030         	ands	r0, r6
 8002a00: 9e08         	ldr	r6, [sp, #0x20]
 8002a02: ea40 0c06    	orr.w	r12, r0, r6
 8002a06: 4608         	mov	r0, r1
 8002a08: e002         	b	0x8002a10 <raylar_crcl_encoder_service::benchmark::encode_words::h522ea4cb3ed82c82+0xb2> @ imm = #0x4
 8002a0a: 2b00         	cmp	r3, #0x0
 8002a0c: f000 80c8    	beq.w	0x8002ba0 <raylar_crcl_encoder_service::benchmark::encode_words::h522ea4cb3ed82c82+0x242> @ imm = #0x190
 8002a10: f1cb 0520    	rsb.w	r5, r11, #0x20
 8002a14: 429d         	cmp	r5, r3
 8002a16: bf28         	it	hs
 8002a18: 461d         	movhs	r5, r3
 8002a1a: 2d20         	cmp	r5, #0x20
 8002a1c: d102         	bne	0x8002a24 <raylar_crcl_encoder_service::benchmark::encode_words::h522ea4cb3ed82c82+0xc6> @ imm = #0x4
 8002a1e: 3b20         	subs	r3, #0x20
 8002a20: 4666         	mov	r6, r12
 8002a22: e00a         	b	0x8002a3a <raylar_crcl_encoder_service::benchmark::encode_words::h522ea4cb3ed82c82+0xdc> @ imm = #0x14
 8002a24: f005 061f    	and	r6, r5, #0x1f
 8002a28: 1b5b         	subs	r3, r3, r5
 8002a2a: fa08 fe06    	lsl.w	lr, r8, r6
 8002a2e: f003 061f    	and	r6, r3, #0x1f
 8002a32: fa2c f606    	lsr.w	r6, r12, r6
 8002a36: ea26 060e    	bic.w	r6, r6, lr
 8002a3a: 44ab         	add	r11, r5
 8002a3c: f1cb 0500    	rsb.w	r5, r11, #0x0
 8002a40: f1bb 0f20    	cmp.w	r11, #0x20
 8002a44: f005 051f    	and	r5, r5, #0x1f
 8002a48: fa06 f605    	lsl.w	r6, r6, r5
 8002a4c: ea44 0406    	orr.w	r4, r4, r6
 8002a50: d1db         	bne	0x8002a0a <raylar_crcl_encoder_service::benchmark::encode_words::h522ea4cb3ed82c82+0xac> @ imm = #-0x4a
 8002a52: 1d01         	adds	r1, r0, #0x4
 8002a54: f110 0f05    	cmn.w	r0, #0x5
 8002a58: bf98         	it	ls
 8002a5a: 4291         	cmpls	r1, r2
 8002a5c: f200 80dc    	bhi.w	0x8002c18 <raylar_crcl_encoder_service::benchmark::encode_words::h522ea4cb3ed82c82+0x2ba> @ imm = #0x1b8
 8002a60: ba26         	rev	r6, r4
 8002a62: f849 6000    	str.w	r6, [r9, r0]
 8002a66: 2400         	movs	r4, #0x0
 8002a68: 4608         	mov	r0, r1
 8002a6a: f04f 0b00    	mov.w	r11, #0x0
 8002a6e: e7cc         	b	0x8002a0a <raylar_crcl_encoder_service::benchmark::encode_words::h522ea4cb3ed82c82+0xac> @ imm = #-0x68
 8002a70: 960a         	str	r6, [sp, #0x28]
 8002a72: b32d         	cbz	r5, 0x8002ac0 <raylar_crcl_encoder_service::benchmark::encode_words::h522ea4cb3ed82c82+0x162> @ imm = #0x4a
 8002a74: 46ae         	mov	lr, r5
 8002a76: 2d20         	cmp	r5, #0x20
 8002a78: bf28         	it	hs
 8002a7a: f04f 0e20    	movhs.w	lr, #0x20
 8002a7e: 460e         	mov	r6, r1
 8002a80: 4670         	mov	r0, lr
 8002a82: e002         	b	0x8002a8a <raylar_crcl_encoder_service::benchmark::encode_words::h522ea4cb3ed82c82+0x12c> @ imm = #0x4
 8002a84: ebb0 000c    	subs.w	r0, r0, r12
 8002a88: d017         	beq	0x8002aba <raylar_crcl_encoder_service::benchmark::encode_words::h522ea4cb3ed82c82+0x15c> @ imm = #0x2e
 8002a8a: f1cb 0c20    	rsb.w	r12, r11, #0x20
 8002a8e: 4584         	cmp	r12, r0
 8002a90: bf28         	it	hs
 8002a92: 4684         	movhs	r12, r0
 8002a94: 44e3         	add	r11, r12
 8002a96: f1bb 0f20    	cmp.w	r11, #0x20
 8002a9a: d1f3         	bne	0x8002a84 <raylar_crcl_encoder_service::benchmark::encode_words::h522ea4cb3ed82c82+0x126> @ imm = #-0x1a
 8002a9c: 1d31         	adds	r1, r6, #0x4
 8002a9e: f116 0f05    	cmn.w	r6, #0x5
 8002aa2: bf98         	it	ls
 8002aa4: 4291         	cmpls	r1, r2
 8002aa6: f200 80b0    	bhi.w	0x8002c0a <raylar_crcl_encoder_service::benchmark::encode_words::h522ea4cb3ed82c82+0x2ac> @ imm = #0x160
 8002aaa: ba23         	rev	r3, r4
 8002aac: f849 3006    	str.w	r3, [r9, r6]
 8002ab0: 2400         	movs	r4, #0x0
 8002ab2: 460e         	mov	r6, r1
 8002ab4: f04f 0b00    	mov.w	r11, #0x0
 8002ab8: e7e4         	b	0x8002a84 <raylar_crcl_encoder_service::benchmark::encode_words::h522ea4cb3ed82c82+0x126> @ imm = #-0x38
 8002aba: ebb5 050e    	subs.w	r5, r5, lr
 8002abe: d1d9         	bne	0x8002a74 <raylar_crcl_encoder_service::benchmark::encode_words::h522ea4cb3ed82c82+0x116> @ imm = #-0x4e
 8002ac0: f8dd e024    	ldr.w	lr, [sp, #0x24]
 8002ac4: 460e         	mov	r6, r1
 8002ac6: f04f 0c01    	mov.w	r12, #0x1
 8002aca: e003         	b	0x8002ad4 <raylar_crcl_encoder_service::benchmark::encode_words::h522ea4cb3ed82c82+0x176> @ imm = #0x6
 8002acc: f1bb 0f20    	cmp.w	r11, #0x20
 8002ad0: 46ab         	mov	r11, r5
 8002ad2: d126         	bne	0x8002b22 <raylar_crcl_encoder_service::benchmark::encode_words::h522ea4cb3ed82c82+0x1c4> @ imm = #0x4c
 8002ad4: f1ab 0520    	sub.w	r5, r11, #0x20
 8002ad8: f1bb 0020    	subs.w	r0, r11, #0x20
 8002adc: fab5 f585    	clz	r5, r5
 8002ae0: bf18         	it	ne
 8002ae2: 2001         	movne	r0, #0x1
 8002ae4: fa08 f300    	lsl.w	r3, r8, r0
 8002ae8: 096d         	lsrs	r5, r5, #0x5
 8002aea: fa2c f505    	lsr.w	r5, r12, r5
 8002aee: ea25 0303    	bic.w	r3, r5, r3
 8002af2: eb0b 0500    	add.w	r5, r11, r0
 8002af6: 4268         	rsbs	r0, r5, #0
 8002af8: 2d20         	cmp	r5, #0x20
 8002afa: f000 001f    	and	r0, r0, #0x1f
 8002afe: fa03 f000    	lsl.w	r0, r3, r0
 8002b02: ea44 0400    	orr.w	r4, r4, r0
 8002b06: d1e1         	bne	0x8002acc <raylar_crcl_encoder_service::benchmark::encode_words::h522ea4cb3ed82c82+0x16e> @ imm = #-0x3e
 8002b08: 1d31         	adds	r1, r6, #0x4
 8002b0a: f116 0f05    	cmn.w	r6, #0x5
 8002b0e: bf98         	it	ls
 8002b10: 4291         	cmpls	r1, r2
 8002b12: d87a         	bhi	0x8002c0a <raylar_crcl_encoder_service::benchmark::encode_words::h522ea4cb3ed82c82+0x2ac> @ imm = #0xf4
 8002b14: ba20         	rev	r0, r4
 8002b16: f849 0006    	str.w	r0, [r9, r6]
 8002b1a: 2400         	movs	r4, #0x0
 8002b1c: 460e         	mov	r6, r1
 8002b1e: 2500         	movs	r5, #0x0
 8002b20: e7d4         	b	0x8002acc <raylar_crcl_encoder_service::benchmark::encode_words::h522ea4cb3ed82c82+0x16e> @ imm = #-0x58
 8002b22: f1be 0f00    	cmp.w	lr, #0x0
 8002b26: d03a         	beq	0x8002b9e <raylar_crcl_encoder_service::benchmark::encode_words::h522ea4cb3ed82c82+0x240> @ imm = #0x74
 8002b28: 9806         	ldr	r0, [sp, #0x18]
 8002b2a: 46ab         	mov	r11, r5
 8002b2c: 9b0a         	ldr	r3, [sp, #0x28]
 8002b2e: ea03 0c00    	and.w	r12, r3, r0
 8002b32: 4608         	mov	r0, r1
 8002b34: e002         	b	0x8002b3c <raylar_crcl_encoder_service::benchmark::encode_words::h522ea4cb3ed82c82+0x1de> @ imm = #0x4
 8002b36: f1be 0f00    	cmp.w	lr, #0x0
 8002b3a: d031         	beq	0x8002ba0 <raylar_crcl_encoder_service::benchmark::encode_words::h522ea4cb3ed82c82+0x242> @ imm = #0x62
 8002b3c: f1cb 0520    	rsb.w	r5, r11, #0x20
 8002b40: 4575         	cmp	r5, lr
 8002b42: bf28         	it	hs
 8002b44: 4675         	movhs	r5, lr
 8002b46: 2d20         	cmp	r5, #0x20
 8002b48: d103         	bne	0x8002b52 <raylar_crcl_encoder_service::benchmark::encode_words::h522ea4cb3ed82c82+0x1f4> @ imm = #0x6
 8002b4a: f1ae 0e20    	sub.w	lr, lr, #0x20
 8002b4e: 4663         	mov	r3, r12
 8002b50: e00b         	b	0x8002b6a <raylar_crcl_encoder_service::benchmark::encode_words::h522ea4cb3ed82c82+0x20c> @ imm = #0x16
 8002b52: f005 031f    	and	r3, r5, #0x1f
 8002b56: ebae 0e05    	sub.w	lr, lr, r5
 8002b5a: f00e 061f    	and	r6, lr, #0x1f
 8002b5e: fa08 f303    	lsl.w	r3, r8, r3
 8002b62: fa2c f606    	lsr.w	r6, r12, r6
 8002b66: ea26 0303    	bic.w	r3, r6, r3
 8002b6a: 44ab         	add	r11, r5
 8002b6c: f1cb 0500    	rsb.w	r5, r11, #0x0
 8002b70: f1bb 0f20    	cmp.w	r11, #0x20
 8002b74: f005 051f    	and	r5, r5, #0x1f
 8002b78: fa03 f305    	lsl.w	r3, r3, r5
 8002b7c: ea44 0403    	orr.w	r4, r4, r3
 8002b80: d1d9         	bne	0x8002b36 <raylar_crcl_encoder_service::benchmark::encode_words::h522ea4cb3ed82c82+0x1d8> @ imm = #-0x4e
 8002b82: 1d01         	adds	r1, r0, #0x4
 8002b84: f110 0f05    	cmn.w	r0, #0x5
 8002b88: bf98         	it	ls
 8002b8a: 4291         	cmpls	r1, r2
 8002b8c: d844         	bhi	0x8002c18 <raylar_crcl_encoder_service::benchmark::encode_words::h522ea4cb3ed82c82+0x2ba> @ imm = #0x88
 8002b8e: ba23         	rev	r3, r4
 8002b90: f849 3000    	str.w	r3, [r9, r0]
 8002b94: 2400         	movs	r4, #0x0
 8002b96: 4608         	mov	r0, r1
 8002b98: f04f 0b00    	mov.w	r11, #0x0
 8002b9c: e7cb         	b	0x8002b36 <raylar_crcl_encoder_service::benchmark::encode_words::h522ea4cb3ed82c82+0x1d8> @ imm = #-0x6a
 8002b9e: 46ab         	mov	r11, r5
 8002ba0: f10a 0a01    	add.w	r10, r10, #0x1
 8002ba4: 2301         	movs	r3, #0x1
 8002ba6: f240 30ff    	movw	r0, #0x3ff
 8002baa: 4582         	cmp	r10, r0
 8002bac: f47f af19    	bne.w	0x80029e2 <raylar_crcl_encoder_service::benchmark::encode_words::h522ea4cb3ed82c82+0x84> @ imm = #-0x1ce
 8002bb0: ea4f 05db    	lsr.w	r5, r11, #0x3
 8002bb4: ea5f 704b    	lsls.w	r0, r11, #0x1d
 8002bb8: bf18         	it	ne
 8002bba: eb03 05db    	addne.w	r5, r3, r11, lsr #3
 8002bbe: 186e         	adds	r6, r5, r1
 8002bc0: d23b         	bhs	0x8002c3a <raylar_crcl_encoder_service::benchmark::encode_words::h522ea4cb3ed82c82+0x2dc> @ imm = #0x76
 8002bc2: 4296         	cmp	r6, r2
 8002bc4: d839         	bhi	0x8002c3a <raylar_crcl_encoder_service::benchmark::encode_words::h522ea4cb3ed82c82+0x2dc> @ imm = #0x72
 8002bc6: ba20         	rev	r0, r4
 8002bc8: 2d05         	cmp	r5, #0x5
 8002bca: 9010         	str	r0, [sp, #0x40]
 8002bcc: d23d         	bhs	0x8002c4a <raylar_crcl_encoder_service::benchmark::encode_words::h522ea4cb3ed82c82+0x2ec> @ imm = #0x7a
 8002bce: eb09 0001    	add.w	r0, r9, r1
 8002bd2: a910         	add	r1, sp, #0x40
 8002bd4: 462a         	mov	r2, r5
 8002bd6: 461c         	mov	r4, r3
 8002bd8: f009 fd60    	bl	0x800c69c <__aeabi_memcpy> @ imm = #0x9ac0
 8002bdc: 9a04         	ldr	r2, [sp, #0x10]
 8002bde: 46a4         	mov	r12, r4
 8002be0: 9803         	ldr	r0, [sp, #0xc]
 8002be2: f641 6458    	movw	r4, #0x1e58
 8002be6: 4432         	add	r2, r6
 8002be8: e89d 4060    	ldm.w	sp, {r5, r6, lr}
 8002bec: 2805         	cmp	r0, #0x5
 8002bee: f47f aecf    	bne.w	0x8002990 <raylar_crcl_encoder_service::benchmark::encode_words::h522ea4cb3ed82c82+0x32> @ imm = #-0x262
 8002bf2: f641 6059    	movw	r0, #0x1e59
 8002bf6: 4282         	cmp	r2, r0
 8002bf8: d230         	bhs	0x8002c5c <raylar_crcl_encoder_service::benchmark::encode_words::h522ea4cb3ed82c82+0x2fe> @ imm = #0x60
 8002bfa: e9cd e20e    	strd	lr, r2, [sp, #56]
 8002bfe: a80e         	add	r0, sp, #0x38
 8002c00: 4610         	mov	r0, r2
 8002c02: b011         	add	sp, #0x44
 8002c04: e8bd 0f00    	pop.w	{r8, r9, r10, r11}
 8002c08: bdf0         	pop	{r4, r5, r6, r7, pc}
 8002c0a: f64d 53e0    	movw	r3, #0xdde0
 8002c0e: 4630         	mov	r0, r6
 8002c10: f6c0 0300    	movt	r3, #0x800
 8002c14: f009 f811    	bl	0x800bc3a <core::slice::index::slice_index_fail> @ imm = #0x9022
 8002c18: f64d 53e0    	movw	r3, #0xdde0
 8002c1c: f6c0 0300    	movt	r3, #0x800
 8002c20: f009 f80b    	bl	0x800bc3a <core::slice::index::slice_index_fail> @ imm = #0x9016
 8002c24: f64d 43cc    	movw	r3, #0xdccc
 8002c28: 4610         	mov	r0, r2
 8002c2a: f6c0 0300    	movt	r3, #0x800
 8002c2e: f641 6158    	movw	r1, #0x1e58
 8002c32: f641 6258    	movw	r2, #0x1e58
 8002c36: f009 f800    	bl	0x800bc3a <core::slice::index::slice_index_fail> @ imm = #0x9000
 8002c3a: f64d 6300    	movw	r3, #0xde00
 8002c3e: 4608         	mov	r0, r1
 8002c40: f6c0 0300    	movt	r3, #0x800
 8002c44: 4631         	mov	r1, r6
 8002c46: f008 fff8    	bl	0x800bc3a <core::slice::index::slice_index_fail> @ imm = #0x8ff0
 8002c4a: f64d 53f0    	movw	r3, #0xddf0
 8002c4e: 2000         	movs	r0, #0x0
 8002c50: f6c0 0300    	movt	r3, #0x800
 8002c54: 4629         	mov	r1, r5
 8002c56: 2204         	movs	r2, #0x4
 8002c58: f008 ffef    	bl	0x800bc3a <core::slice::index::slice_index_fail> @ imm = #0x8fde
 8002c5c: f64d 43bc    	movw	r3, #0xdcbc
 8002c60: 4611         	mov	r1, r2
 8002c62: f6c0 0300    	movt	r3, #0x800
 8002c66: 2000         	movs	r0, #0x0
 8002c68: f641 6258    	movw	r2, #0x1e58
 8002c6c: f008 ffe5    	bl	0x800bc3a <core::slice::index::slice_index_fail> @ imm = #0x8fca

08002c70 <raylar_crcl_encoder_service::benchmark::encode_folded::h7ac7123b2c9324bf>:
 8002c70: b580         	push	{r7, lr}
 8002c72: 466f         	mov	r7, sp
 8002c74: 2100         	movs	r1, #0x0
 8002c76: 2200         	movs	r2, #0x0
 8002c78: e8bd 4080    	pop.w	{r7, lr}
 8002c7c: f7ff bbec    	b.w	0x8002458 <raylar_crcl_encoder_service::benchmark::encode_bits::h865b9e06648afa8c> @ imm = #-0x828

08002c80 <raylar_crcl_encoder_service::benchmark::encode_payload::hfef902a78aec660e>:
 8002c80: b580         	push	{r7, lr}
 8002c82: 466f         	mov	r7, sp
 8002c84: 2100         	movs	r1, #0x0
 8002c86: 2201         	movs	r2, #0x1
 8002c88: e8bd 4080    	pop.w	{r7, lr}
 8002c8c: f7ff bbe4    	b.w	0x8002458 <raylar_crcl_encoder_service::benchmark::encode_bits::h865b9e06648afa8c> @ imm = #-0x838

08002c90 <raylar_crcl_encoder_service::benchmark::encode_codeword::h30aff6e5abaf4d69>:
 8002c90: b580         	push	{r7, lr}
 8002c92: 466f         	mov	r7, sp
 8002c94: 2101         	movs	r1, #0x1
 8002c96: 2200         	movs	r2, #0x0
 8002c98: e8bd 4080    	pop.w	{r7, lr}
 8002c9c: f7ff bbdc    	b.w	0x8002458 <raylar_crcl_encoder_service::benchmark::encode_bits::h865b9e06648afa8c> @ imm = #-0x848

08002ca0 <raylar_crcl_encoder_service::benchmark::split_transform::h9238bb9046f68a6e>:
 8002ca0: b5f0         	push	{r4, r5, r6, r7, lr}
 8002ca2: af03         	add	r7, sp, #0xc
 8002ca4: e92d 0f00    	push.w	{r8, r9, r10, r11}
 8002ca8: f2ad 5d04    	subw	sp, sp, #0x504
 8002cac: 9002         	str	r0, [sp, #0x8]
 8002cae: a817         	add	r0, sp, #0x5c
 8002cb0: 460e         	mov	r6, r1
 8002cb2: f44f 7101    	mov.w	r1, #0x204
 8002cb6: 4614         	mov	r4, r2
 8002cb8: f009 ff19    	bl	0x800caee <__aeabi_memclr4> @ imm = #0x9e32
 8002cbc: a898         	add	r0, sp, #0x260
 8002cbe: f44f 7101    	mov.w	r1, #0x204
 8002cc2: f009 ff14    	bl	0x800caee <__aeabi_memclr4> @ imm = #0x9e28
 8002cc6: f24c 0028    	movw	r0, #0xc028
 8002cca: 1833         	adds	r3, r6, r0
 8002ccc: f246 0028    	movw	r0, #0x6028
 8002cd0: f1a7 0bbc    	sub.w	r11, r7, #0xbc
 8002cd4: 4430         	add	r0, r6
 8002cd6: 9009         	str	r0, [sp, #0x24]
 8002cd8: f10b 0084    	add.w	r0, r11, #0x84
 8002cdc: 9008         	str	r0, [sp, #0x20]
 8002cde: f10b 0074    	add.w	r0, r11, #0x74
 8002ce2: 9007         	str	r0, [sp, #0x1c]
 8002ce4: f10b 0048    	add.w	r0, r11, #0x48
 8002ce8: 9006         	str	r0, [sp, #0x18]
 8002cea: f10b 0020    	add.w	r0, r11, #0x20
 8002cee: f1a7 0924    	sub.w	r9, r7, #0x24
 8002cf2: 9005         	str	r0, [sp, #0x14]
 8002cf4: f106 0028    	add.w	r0, r6, #0x28
 8002cf8: f06f 0607    	mvn	r6, #0x7
 8002cfc: f44f 6580    	mov.w	r5, #0x400
 8002d00: 9004         	str	r0, [sp, #0x10]
 8002d02: 9303         	str	r3, [sp, #0xc]
 8002d04: e008         	b	0x8002d18 <raylar_crcl_encoder_service::benchmark::split_transform::h9238bb9046f68a6e+0x78> @ imm = #0x10
 8002d06: 3601         	adds	r6, #0x1
 8002d08: 46d9         	mov	r9, r11
 8002d0a: f1a7 0bbc    	sub.w	r11, r7, #0xbc
 8002d0e: f44f 6580    	mov.w	r5, #0x400
 8002d12: 2e81         	cmp	r6, #0x81
 8002d14: f000 8145    	beq.w	0x8002fa2 <raylar_crcl_encoder_service::benchmark::split_transform::h9238bb9046f68a6e+0x302> @ imm = #0x28a
 8002d18: 9616         	str	r6, [sp, #0x58]
 8002d1a: f44f 42c0    	mov.w	r2, #0x6000
 8002d1e: 9e09         	ldr	r6, [sp, #0x24]
 8002d20: 9904         	ldr	r1, [sp, #0x10]
 8002d22: 4630         	mov	r0, r6
 8002d24: f009 fe95    	bl	0x800ca52 <__aeabi_memcpy4> @ imm = #0x9d2a
 8002d28: 9806         	ldr	r0, [sp, #0x18]
 8002d2a: 2128         	movs	r1, #0x28
 8002d2c: f009 fedf    	bl	0x800caee <__aeabi_memclr4> @ imm = #0x9dbe
 8002d30: 9807         	ldr	r0, [sp, #0x1c]
 8002d32: 2100         	movs	r1, #0x0
 8002d34: f44f 0200    	mov.w	r2, #0x800000
 8002d38: e947 112f    	strd	r1, r1, [r7, #-188]
 8002d3c: e947 112d    	strd	r1, r1, [r7, #-180]
 8002d40: e9c0 1100    	strd	r1, r1, [r0]
 8002d44: e9c0 1102    	strd	r1, r1, [r0, #8]
 8002d48: 9805         	ldr	r0, [sp, #0x14]
 8002d4a: e947 112b    	strd	r1, r1, [r7, #-172]
 8002d4e: f847 1ca4    	str	r1, [r7, #-164]
 8002d52: e9c0 1100    	strd	r1, r1, [r0]
 8002d56: e9c0 1102    	strd	r1, r1, [r0, #8]
 8002d5a: e9c0 1104    	strd	r1, r1, [r0, #16]
 8002d5e: 6181         	str	r1, [r0, #0x18]
 8002d60: 8381         	strh	r1, [r0, #0x1c]
 8002d62: 9808         	ldr	r0, [sp, #0x20]
 8002d64: f827 1ca0    	strh	r1, [r7, #-160]
 8002d68: f827 1c2c    	strh	r1, [r7, #-44]
 8002d6c: 6042         	str	r2, [r0, #0x4]
 8002d6e: f240 620a    	movw	r2, #0x60a
 8002d72: f2c0 1201    	movt	r2, #0x101
 8002d76: 6005         	str	r5, [r0]
 8002d78: 6082         	str	r2, [r0, #0x8]
 8002d7a: 2001         	movs	r0, #0x1
 8002d7c: f847 0c4c    	str	r0, [r7, #-76]
 8002d80: f44f 50c0    	mov.w	r0, #0x1800
 8002d84: e947 601f    	strd	r6, r0, [r7, #-124]
 8002d88: 47a0         	blx	r4
 8002d8a: f847 5c24    	str	r5, [r7, #-36]
 8002d8e: 4682         	mov	r10, r0
 8002d90: 4658         	mov	r0, r11
 8002d92: f857 1c24    	ldr	r1, [r7, #-36]
 8002d96: f7fe fd62    	bl	0x800185e <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3> @ imm = #-0x153c
 8002d9a: 07c0         	lsls	r0, r0, #0x1f
 8002d9c: f040 80d7    	bne.w	0x8002f4e <raylar_crcl_encoder_service::benchmark::split_transform::h9238bb9046f68a6e+0x2ae> @ imm = #0x1ae
 8002da0: f001 0001    	and	r0, r1, #0x1
 8002da4: f807 0c24    	strb	r0, [r7, #-36]
 8002da8: 47a0         	blx	r4
 8002daa: f847 5c24    	str	r5, [r7, #-36]
 8002dae: 4606         	mov	r6, r0
 8002db0: 4658         	mov	r0, r11
 8002db2: f857 1c24    	ldr	r1, [r7, #-36]
 8002db6: f7fe fd52    	bl	0x800185e <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3> @ imm = #-0x155c
 8002dba: 07c0         	lsls	r0, r0, #0x1f
 8002dbc: f040 80ce    	bne.w	0x8002f5c <raylar_crcl_encoder_service::benchmark::split_transform::h9238bb9046f68a6e+0x2bc> @ imm = #0x19c
 8002dc0: f001 0001    	and	r0, r1, #0x1
 8002dc4: 9615         	str	r6, [sp, #0x54]
 8002dc6: f807 0c24    	strb	r0, [r7, #-36]
 8002dca: 47a0         	blx	r4
 8002dcc: 4680         	mov	r8, r0
 8002dce: 47a0         	blx	r4
 8002dd0: f847 5c24    	str	r5, [r7, #-36]
 8002dd4: 4606         	mov	r6, r0
 8002dd6: 4658         	mov	r0, r11
 8002dd8: f857 1c24    	ldr	r1, [r7, #-36]
 8002ddc: f7fe fd3f    	bl	0x800185e <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3> @ imm = #-0x1582
 8002de0: 07c0         	lsls	r0, r0, #0x1f
 8002de2: f040 80b4    	bne.w	0x8002f4e <raylar_crcl_encoder_service::benchmark::split_transform::h9238bb9046f68a6e+0x2ae> @ imm = #0x168
 8002de6: f001 0001    	and	r0, r1, #0x1
 8002dea: f807 0c24    	strb	r0, [r7, #-36]
 8002dee: 47a0         	blx	r4
 8002df0: 9014         	str	r0, [sp, #0x50]
 8002df2: 4658         	mov	r0, r11
 8002df4: f847 5c24    	str	r5, [r7, #-36]
 8002df8: f857 1c24    	ldr	r1, [r7, #-36]
 8002dfc: f7fe fd2f    	bl	0x800185e <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3> @ imm = #-0x15a2
 8002e00: 07c0         	lsls	r0, r0, #0x1f
 8002e02: f040 80ab    	bne.w	0x8002f5c <raylar_crcl_encoder_service::benchmark::split_transform::h9238bb9046f68a6e+0x2bc> @ imm = #0x156
 8002e06: f001 0001    	and	r0, r1, #0x1
 8002e0a: f807 0c24    	strb	r0, [r7, #-36]
 8002e0e: 47a0         	blx	r4
 8002e10: 9013         	str	r0, [sp, #0x4c]
 8002e12: 47a0         	blx	r4
 8002e14: 9012         	str	r0, [sp, #0x48]
 8002e16: 4658         	mov	r0, r11
 8002e18: f847 5c24    	str	r5, [r7, #-36]
 8002e1c: f857 1c24    	ldr	r1, [r7, #-36]
 8002e20: f7fe fd1d    	bl	0x800185e <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3> @ imm = #-0x15c6
 8002e24: 07c0         	lsls	r0, r0, #0x1f
 8002e26: f040 8092    	bne.w	0x8002f4e <raylar_crcl_encoder_service::benchmark::split_transform::h9238bb9046f68a6e+0x2ae> @ imm = #0x124
 8002e2a: f001 0001    	and	r0, r1, #0x1
 8002e2e: f807 0c24    	strb	r0, [r7, #-36]
 8002e32: 47a0         	blx	r4
 8002e34: 9011         	str	r0, [sp, #0x44]
 8002e36: 4658         	mov	r0, r11
 8002e38: f847 5c24    	str	r5, [r7, #-36]
 8002e3c: f857 1c24    	ldr	r1, [r7, #-36]
 8002e40: f7fe fd0d    	bl	0x800185e <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3> @ imm = #-0x15e6
 8002e44: 07c0         	lsls	r0, r0, #0x1f
 8002e46: f040 8089    	bne.w	0x8002f5c <raylar_crcl_encoder_service::benchmark::split_transform::h9238bb9046f68a6e+0x2bc> @ imm = #0x112
 8002e4a: f001 0001    	and	r0, r1, #0x1
 8002e4e: f807 0c24    	strb	r0, [r7, #-36]
 8002e52: 47a0         	blx	r4
 8002e54: 9010         	str	r0, [sp, #0x40]
 8002e56: 47a0         	blx	r4
 8002e58: 900f         	str	r0, [sp, #0x3c]
 8002e5a: 4658         	mov	r0, r11
 8002e5c: f847 5c24    	str	r5, [r7, #-36]
 8002e60: f857 1c24    	ldr	r1, [r7, #-36]
 8002e64: f7fe fcfb    	bl	0x800185e <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3> @ imm = #-0x160a
 8002e68: 07c0         	lsls	r0, r0, #0x1f
 8002e6a: d170         	bne	0x8002f4e <raylar_crcl_encoder_service::benchmark::split_transform::h9238bb9046f68a6e+0x2ae> @ imm = #0xe0
 8002e6c: f001 0001    	and	r0, r1, #0x1
 8002e70: f807 0c24    	strb	r0, [r7, #-36]
 8002e74: 47a0         	blx	r4
 8002e76: 900e         	str	r0, [sp, #0x38]
 8002e78: 4658         	mov	r0, r11
 8002e7a: f847 5c24    	str	r5, [r7, #-36]
 8002e7e: f857 1c24    	ldr	r1, [r7, #-36]
 8002e82: f7fe fcec    	bl	0x800185e <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3> @ imm = #-0x1628
 8002e86: 07c0         	lsls	r0, r0, #0x1f
 8002e88: d168         	bne	0x8002f5c <raylar_crcl_encoder_service::benchmark::split_transform::h9238bb9046f68a6e+0x2bc> @ imm = #0xd0
 8002e8a: f001 0001    	and	r0, r1, #0x1
 8002e8e: f807 0c24    	strb	r0, [r7, #-36]
 8002e92: 47a0         	blx	r4
 8002e94: 900d         	str	r0, [sp, #0x34]
 8002e96: 47a0         	blx	r4
 8002e98: 900c         	str	r0, [sp, #0x30]
 8002e9a: 4658         	mov	r0, r11
 8002e9c: f847 5c24    	str	r5, [r7, #-36]
 8002ea0: f857 1c24    	ldr	r1, [r7, #-36]
 8002ea4: f7fe fcdb    	bl	0x800185e <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3> @ imm = #-0x164a
 8002ea8: 07c0         	lsls	r0, r0, #0x1f
 8002eaa: d150         	bne	0x8002f4e <raylar_crcl_encoder_service::benchmark::split_transform::h9238bb9046f68a6e+0x2ae> @ imm = #0xa0
 8002eac: f001 0001    	and	r0, r1, #0x1
 8002eb0: e9cd 680a    	strd	r6, r8, [sp, #40]
 8002eb4: f807 0c24    	strb	r0, [r7, #-36]
 8002eb8: 47a0         	blx	r4
 8002eba: f847 5c24    	str	r5, [r7, #-36]
 8002ebe: 4680         	mov	r8, r0
 8002ec0: 4658         	mov	r0, r11
 8002ec2: f857 1c24    	ldr	r1, [r7, #-36]
 8002ec6: f7fe fcca    	bl	0x800185e <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3> @ imm = #-0x166c
 8002eca: 07c0         	lsls	r0, r0, #0x1f
 8002ecc: d146         	bne	0x8002f5c <raylar_crcl_encoder_service::benchmark::split_transform::h9238bb9046f68a6e+0x2bc> @ imm = #0x8c
 8002ece: f001 0001    	and	r0, r1, #0x1
 8002ed2: f807 0c24    	strb	r0, [r7, #-36]
 8002ed6: 9d15         	ldr	r5, [sp, #0x54]
 8002ed8: 46cb         	mov	r11, r9
 8002eda: 47a0         	blx	r4
 8002edc: 9903         	ldr	r1, [sp, #0xc]
 8002ede: 4681         	mov	r9, r0
 8002ee0: f857 0c78    	ldr	r0, [r7, #-120]
 8002ee4: f847 1c28    	str	r1, [r7, #-40]
 8002ee8: f5b0 5fc0    	cmp.w	r0, #0x1800
 8002eec: d14a         	bne	0x8002f84 <raylar_crcl_encoder_service::benchmark::split_transform::h9238bb9046f68a6e+0x2e4> @ imm = #0x94
 8002eee: f857 0c7c    	ldr	r0, [r7, #-124]
 8002ef2: f44f 42c0    	mov.w	r2, #0x6000
 8002ef6: f009 fb97    	bl	0x800c628 <memcmp>      @ imm = #0x972e
 8002efa: 2800         	cmp	r0, #0x0
 8002efc: d142         	bne	0x8002f84 <raylar_crcl_encoder_service::benchmark::split_transform::h9238bb9046f68a6e+0x2e4> @ imm = #0x84
 8002efe: 9e16         	ldr	r6, [sp, #0x58]
 8002f00: f106 0008    	add.w	r0, r6, #0x8
 8002f04: 2807         	cmp	r0, #0x7
 8002f06: f67f aefe    	bls.w	0x8002d06 <raylar_crcl_encoder_service::benchmark::split_transform::h9238bb9046f68a6e+0x66> @ imm = #-0x204
 8002f0a: 9a0a         	ldr	r2, [sp, #0x28]
 8002f0c: 9814         	ldr	r0, [sp, #0x50]
 8002f0e: eb0a 0102    	add.w	r1, r10, r2
 8002f12: 9a11         	ldr	r2, [sp, #0x44]
 8002f14: 4428         	add	r0, r5
 8002f16: 9d13         	ldr	r5, [sp, #0x4c]
 8002f18: 4410         	add	r0, r2
 8002f1a: 9a0b         	ldr	r2, [sp, #0x2c]
 8002f1c: 442a         	add	r2, r5
 8002f1e: 9d12         	ldr	r5, [sp, #0x48]
 8002f20: 4429         	add	r1, r5
 8002f22: 9d0e         	ldr	r5, [sp, #0x38]
 8002f24: 4428         	add	r0, r5
 8002f26: 9d10         	ldr	r5, [sp, #0x40]
 8002f28: 442a         	add	r2, r5
 8002f2a: 9d0f         	ldr	r5, [sp, #0x3c]
 8002f2c: 4429         	add	r1, r5
 8002f2e: 9d0c         	ldr	r5, [sp, #0x30]
 8002f30: 4429         	add	r1, r5
 8002f32: 9d0d         	ldr	r5, [sp, #0x34]
 8002f34: 1a41         	subs	r1, r0, r1
 8002f36: 4440         	add	r0, r8
 8002f38: 442a         	add	r2, r5
 8002f3a: 4441         	add	r1, r8
 8002f3c: 1a10         	subs	r0, r2, r0
 8002f3e: aa98         	add	r2, sp, #0x260
 8002f40: 4448         	add	r0, r9
 8002f42: f842 0026    	str.w	r0, [r2, r6, lsl #2]
 8002f46: a817         	add	r0, sp, #0x5c
 8002f48: f840 1026    	str.w	r1, [r0, r6, lsl #2]
 8002f4c: e6db         	b	0x8002d06 <raylar_crcl_encoder_service::benchmark::split_transform::h9238bb9046f68a6e+0x66> @ imm = #-0x24a
 8002f4e: f64d 40fc    	movw	r0, #0xdcfc
 8002f52: f807 1c24    	strb	r1, [r7, #-36]
 8002f56: f6c0 0000    	movt	r0, #0x800
 8002f5a: e005         	b	0x8002f68 <raylar_crcl_encoder_service::benchmark::split_transform::h9238bb9046f68a6e+0x2c8> @ imm = #0xa
 8002f5c: f64d 40ec    	movw	r0, #0xdcec
 8002f60: f807 1c24    	strb	r1, [r7, #-36]
 8002f64: f6c0 0000    	movt	r0, #0x800
 8002f68: f64d 434c    	movw	r3, #0xdc4c
 8002f6c: 9000         	str	r0, [sp]
 8002f6e: f64d 30b0    	movw	r0, #0xdbb0
 8002f72: f1a7 0224    	sub.w	r2, r7, #0x24
 8002f76: f6c0 0000    	movt	r0, #0x800
 8002f7a: f6c0 0300    	movt	r3, #0x800
 8002f7e: 212b         	movs	r1, #0x2b
 8002f80: f008 fb50    	bl	0x800b624 <core::result::unwrap_failed> @ imm = #0x86a0
 8002f84: f64d 40dc    	movw	r0, #0xdcdc
 8002f88: f1a7 0228    	sub.w	r2, r7, #0x28
 8002f8c: f6c0 0000    	movt	r0, #0x800
 8002f90: 2300         	movs	r3, #0x0
 8002f92: 9001         	str	r0, [sp, #0x4]
 8002f94: f1a7 00bc    	sub.w	r0, r7, #0xbc
 8002f98: f100 0140    	add.w	r1, r0, #0x40
 8002f9c: 2000         	movs	r0, #0x0
 8002f9e: f005 fc71    	bl	0x8008884 <core::panicking::assert_failed::h08239383d2bf65a5> @ imm = #0x58e2
 8002fa2: f1a7 041d    	sub.w	r4, r7, #0x1d
 8002fa6: a817         	add	r0, sp, #0x5c
 8002fa8: 2181         	movs	r1, #0x81
 8002faa: 4622         	mov	r2, r4
 8002fac: f004 fd59    	bl	0x8007a62 <core::slice::sort::unstable::ipnsort::hd8a371dd4e708cab> @ imm = #0x4ab2
 8002fb0: a898         	add	r0, sp, #0x260
 8002fb2: 2181         	movs	r1, #0x81
 8002fb4: 4622         	mov	r2, r4
 8002fb6: f004 fd54    	bl	0x8007a62 <core::slice::sort::unstable::ipnsort::hd8a371dd4e708cab> @ imm = #0x4aa8
 8002fba: 9802         	ldr	r0, [sp, #0x8]
 8002fbc: f64d 5a28    	movw	r10, #0xdd28
 8002fc0: f04f 091f    	mov.w	r9, #0x1f
 8002fc4: f6c0 0a00    	movt	r10, #0x800
 8002fc8: f8dd c260    	ldr.w	r12, [sp, #0x260]
 8002fcc: f04f 0b01    	mov.w	r11, #0x1
 8002fd0: e9c0 a908    	strd	r10, r9, [r0, #32]
 8002fd4: f100 0928    	add.w	r9, r0, #0x28
 8002fd8: f8dd 342c    	ldr.w	r3, [sp, #0x42c]
 8002fdc: f04f 0800    	mov.w	r8, #0x0
 8002fe0: f8dd e360    	ldr.w	lr, [sp, #0x360]
 8002fe4: e889 5800    	stm.w	r9, {r11, r12, lr}
 8002fe8: f100 0c34    	add.w	r12, r0, #0x34
 8002fec: f8dd 6460    	ldr.w	r6, [sp, #0x460]
 8002ff0: e88c 0148    	stm.w	r12, {r3, r6, r8}
 8002ff4: f64d 530c    	movw	r3, #0xdd0c
 8002ff8: f6c0 0300    	movt	r3, #0x800
 8002ffc: 9a57         	ldr	r2, [sp, #0x15c]
 8002ffe: 9997         	ldr	r1, [sp, #0x25c]
 8003000: 9c17         	ldr	r4, [sp, #0x5c]
 8003002: 9d8a         	ldr	r5, [sp, #0x228]
 8003004: 6003         	str	r3, [r0]
 8003006: 231c         	movs	r3, #0x1c
 8003008: e9c0 3b01    	strd	r3, r11, [r0, #4]
 800300c: e9c0 4203    	strd	r4, r2, [r0, #12]
 8003010: e9c0 5105    	strd	r5, r1, [r0, #20]
 8003014: f8c0 801c    	str.w	r8, [r0, #0x1c]
 8003018: f20d 5d04    	addw	sp, sp, #0x504
 800301c: e8bd 0f00    	pop.w	{r8, r9, r10, r11}
 8003020: bdf0         	pop	{r4, r5, r6, r7, pc}

08003022 <raylar_crcl_encoder_service::benchmark::transform_budgeted::h9752c8449280daff>:
 8003022: b5f0         	push	{r4, r5, r6, r7, lr}
 8003024: af03         	add	r7, sp, #0xc
 8003026: f84d 8d04    	str	r8, [sp, #-4]!
 800302a: b0aa         	sub	sp, #0xa8
 800302c: f240 610a    	movw	r1, #0x60a
 8003030: f2c0 1101    	movt	r1, #0x101
 8003034: 9129         	str	r1, [sp, #0xa4]
 8003036: f44f 0100    	mov.w	r1, #0x800000
 800303a: 9128         	str	r1, [sp, #0xa0]
 800303c: f44f 6180    	mov.w	r1, #0x400
 8003040: 9127         	str	r1, [sp, #0x9c]
 8003042: a927         	add	r1, sp, #0x9c
 8003044: 9927         	ldr	r1, [sp, #0x9c]
 8003046: 1e4a         	subs	r2, r1, #0x1
 8003048: ea81 0302    	eor.w	r3, r1, r2
 800304c: 4293         	cmp	r3, r2
 800304e: d916         	bls	0x800307e <raylar_crcl_encoder_service::benchmark::transform_budgeted::h9752c8449280daff+0x5c> @ imm = #0x2c
 8003050: fa91 f2a1    	rbit	r2, r1
 8003054: fab2 f282    	clz	r2, r2
 8003058: 3a08         	subs	r2, #0x8
 800305a: 2a04         	cmp	r2, #0x4
 800305c: bf9c         	itt	ls
 800305e: f89d 20a4    	ldrbls.w	r2, [sp, #0xa4]
 8003062: 2a0c         	cmpls	r2, #0xc
 8003064: d80b         	bhi	0x800307e <raylar_crcl_encoder_service::benchmark::transform_budgeted::h9752c8449280daff+0x5c> @ imm = #0x16
 8003066: 2301         	movs	r3, #0x1
 8003068: fa03 f202    	lsl.w	r2, r3, r2
 800306c: f412 5faa    	tst.w	r2, #0x1540
 8003070: bf1e         	ittt	ne
 8003072: 9a28         	ldrne	r2, [sp, #0xa0]
 8003074: f102 4240    	addne.w	r2, r2, #0xc0000000
 8003078: f1b2 4f40    	cmpne.w	r2, #0xc0000000
 800307c: d813         	bhi	0x80030a6 <raylar_crcl_encoder_service::benchmark::transform_budgeted::h9752c8449280daff+0x84> @ imm = #0x26
 800307e: 2000         	movs	r0, #0x0
 8003080: f64d 434c    	movw	r3, #0xdc4c
 8003084: f88d 0008    	strb.w	r0, [sp, #0x8]
 8003088: f64d 5058    	movw	r0, #0xdd58
 800308c: f6c0 0000    	movt	r0, #0x800
 8003090: aa02         	add	r2, sp, #0x8
 8003092: 9000         	str	r0, [sp]
 8003094: f64d 30b0    	movw	r0, #0xdbb0
 8003098: f6c0 0000    	movt	r0, #0x800
 800309c: f6c0 0300    	movt	r3, #0x800
 80030a0: 212b         	movs	r1, #0x2b
 80030a2: f008 fabf    	bl	0x800b624 <core::result::unwrap_failed> @ imm = #0x857e
 80030a6: f89d 20a5    	ldrb.w	r2, [sp, #0xa5]
 80030aa: 2a1e         	cmp	r2, #0x1e
 80030ac: d8e7         	bhi	0x800307e <raylar_crcl_encoder_service::benchmark::transform_budgeted::h9752c8449280daff+0x5c> @ imm = #-0x32
 80030ae: f89d 30a7    	ldrb.w	r3, [sp, #0xa7]
 80030b2: 2b01         	cmp	r3, #0x1
 80030b4: d107         	bne	0x80030c6 <raylar_crcl_encoder_service::benchmark::transform_budgeted::h9752c8449280daff+0xa4> @ imm = #0xe
 80030b6: f5b1 6f80    	cmp.w	r1, #0x400
 80030ba: d3e0         	blo	0x800307e <raylar_crcl_encoder_service::benchmark::transform_budgeted::h9752c8449280daff+0x5c> @ imm = #-0x40
 80030bc: f89d 10a6    	ldrb.w	r1, [sp, #0xa6]
 80030c0: 2901         	cmp	r1, #0x1
 80030c2: d005         	beq	0x80030d0 <raylar_crcl_encoder_service::benchmark::transform_budgeted::h9752c8449280daff+0xae> @ imm = #0xa
 80030c4: e7db         	b	0x800307e <raylar_crcl_encoder_service::benchmark::transform_budgeted::h9752c8449280daff+0x5c> @ imm = #-0x4a
 80030c6: f5b1 6f80    	cmp.w	r1, #0x400
 80030ca: d3d8         	blo	0x800307e <raylar_crcl_encoder_service::benchmark::transform_budgeted::h9752c8449280daff+0x5c> @ imm = #-0x50
 80030cc: 2a00         	cmp	r2, #0x0
 80030ce: d1d6         	bne	0x800307e <raylar_crcl_encoder_service::benchmark::transform_budgeted::h9752c8449280daff+0x5c> @ imm = #-0x54
 80030d0: ac02         	add	r4, sp, #0x8
 80030d2: f246 0128    	movw	r1, #0x6028
 80030d6: 1845         	adds	r5, r0, r1
 80030d8: f104 0048    	add.w	r0, r4, #0x48
 80030dc: 2128         	movs	r1, #0x28
 80030de: f009 fd06    	bl	0x800caee <__aeabi_memclr4> @ imm = #0x9a0c
 80030e2: ab27         	add	r3, sp, #0x9c
 80030e4: 9512         	str	r5, [sp, #0x48]
 80030e6: f44f 56c0    	mov.w	r6, #0x1800
 80030ea: f104 0840    	add.w	r8, r4, #0x40
 80030ee: cb0e         	ldm	r3, {r1, r2, r3}
 80030f0: ad27         	add	r5, sp, #0x9c
 80030f2: 2000         	movs	r0, #0x0
 80030f4: 9613         	str	r6, [sp, #0x4c]
 80030f6: f44f 7680    	mov.w	r6, #0x100
 80030fa: e9cd 2324    	strd	r2, r3, [sp, #144]
 80030fe: 2201         	movs	r2, #0x1
 8003100: f8ad 0024    	strh.w	r0, [sp, #0x24]
 8003104: e9cd 0007    	strd	r0, r0, [sp, #28]
 8003108: e9cd 0005    	strd	r0, r0, [sp, #20]
 800310c: e9cd 0003    	strd	r0, r0, [sp, #12]
 8003110: e9cd 000b    	strd	r0, r0, [sp, #44]
 8003114: e9cd 000d    	strd	r0, r0, [sp, #52]
 8003118: e9cd 000f    	strd	r0, r0, [sp, #60]
 800311c: f8ad 0044    	strh.w	r0, [sp, #0x44]
 8003120: e9cd 001f    	strd	r0, r0, [sp, #124]
 8003124: e9cd 0021    	strd	r0, r0, [sp, #132]
 8003128: 9002         	str	r0, [sp, #0x8]
 800312a: 900a         	str	r0, [sp, #0x28]
 800312c: 921e         	str	r2, [sp, #0x78]
 800312e: 9123         	str	r1, [sp, #0x8c]
 8003130: f8ad 0098    	strh.w	r0, [sp, #0x98]
 8003134: 9627         	str	r6, [sp, #0x9c]
 8003136: 4620         	mov	r0, r4
 8003138: 9927         	ldr	r1, [sp, #0x9c]
 800313a: f7fe fb90    	bl	0x800185e <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3> @ imm = #-0x18e0
 800313e: 07c0         	lsls	r0, r0, #0x1f
 8003140: d109         	bne	0x8003156 <raylar_crcl_encoder_service::benchmark::transform_budgeted::h9752c8449280daff+0x134> @ imm = #0x12
 8003142: 07c8         	lsls	r0, r1, #0x1f
 8003144: d0f6         	beq	0x8003134 <raylar_crcl_encoder_service::benchmark::transform_budgeted::h9752c8449280daff+0x112> @ imm = #-0x14
 8003146: f8cd 809c    	str.w	r8, [sp, #0x9c]
 800314a: a827         	add	r0, sp, #0x9c
 800314c: 9814         	ldr	r0, [sp, #0x50]
 800314e: b02a         	add	sp, #0xa8
 8003150: f85d 8b04    	ldr	r8, [sp], #4
 8003154: bdf0         	pop	{r4, r5, r6, r7, pc}
 8003156: f64d 5048    	movw	r0, #0xdd48
 800315a: f64d 434c    	movw	r3, #0xdc4c
 800315e: f6c0 0000    	movt	r0, #0x800
 8003162: aa27         	add	r2, sp, #0x9c
 8003164: 9000         	str	r0, [sp]
 8003166: f64d 30b0    	movw	r0, #0xdbb0
 800316a: f88d 109c    	strb.w	r1, [sp, #0x9c]
 800316e: f6c0 0000    	movt	r0, #0x800
 8003172: f6c0 0300    	movt	r3, #0x800
 8003176: 212b         	movs	r1, #0x2b
 8003178: f008 fa54    	bl	0x800b624 <core::result::unwrap_failed> @ imm = #0x84a8

0800317c <raylar_crcl_encoder_service::benchmark::serialize_production::h9a703500dd702791>:
 800317c: b5f0         	push	{r4, r5, r6, r7, lr}
 800317e: af03         	add	r7, sp, #0xc
 8003180: e92d 0b00    	push.w	{r8, r9, r11}
 8003184: b0b8         	sub	sp, #0xe0
 8003186: f240 6c0a    	movw	r12, #0x60a
 800318a: f44f 0100    	mov.w	r1, #0x800000
 800318e: f2c0 1c01    	movt	r12, #0x101
 8003192: 9127         	str	r1, [sp, #0x9c]
 8003194: f44f 6180    	mov.w	r1, #0x400
 8003198: f8cd c0a0    	str.w	r12, [sp, #0xa0]
 800319c: 9126         	str	r1, [sp, #0x98]
 800319e: f10d 0898    	add.w	r8, sp, #0x98
 80031a2: 9926         	ldr	r1, [sp, #0x98]
 80031a4: 1e4a         	subs	r2, r1, #0x1
 80031a6: ea81 0302    	eor.w	r3, r1, r2
 80031aa: 4293         	cmp	r3, r2
 80031ac: d916         	bls	0x80031dc <raylar_crcl_encoder_service::benchmark::serialize_production::h9a703500dd702791+0x60> @ imm = #0x2c
 80031ae: fa91 f2a1    	rbit	r2, r1
 80031b2: fab2 f282    	clz	r2, r2
 80031b6: 3a08         	subs	r2, #0x8
 80031b8: 2a04         	cmp	r2, #0x4
 80031ba: bf9c         	itt	ls
 80031bc: f89d 20a0    	ldrbls.w	r2, [sp, #0xa0]
 80031c0: 2a0c         	cmpls	r2, #0xc
 80031c2: d80b         	bhi	0x80031dc <raylar_crcl_encoder_service::benchmark::serialize_production::h9a703500dd702791+0x60> @ imm = #0x16
 80031c4: 2301         	movs	r3, #0x1
 80031c6: fa03 f202    	lsl.w	r2, r3, r2
 80031ca: f412 5faa    	tst.w	r2, #0x1540
 80031ce: bf1e         	ittt	ne
 80031d0: 9a27         	ldrne	r2, [sp, #0x9c]
 80031d2: f102 4240    	addne.w	r2, r2, #0xc0000000
 80031d6: f1b2 4f40    	cmpne.w	r2, #0xc0000000
 80031da: d813         	bhi	0x8003204 <raylar_crcl_encoder_service::benchmark::serialize_production::h9a703500dd702791+0x88> @ imm = #0x26
 80031dc: 2000         	movs	r0, #0x0
 80031de: f88d 0098    	strb.w	r0, [sp, #0x98]
 80031e2: f64d 5098    	movw	r0, #0xdd98
 80031e6: f6c0 0000    	movt	r0, #0x800
 80031ea: f64d 434c    	movw	r3, #0xdc4c
 80031ee: 9000         	str	r0, [sp]
 80031f0: f64d 30b0    	movw	r0, #0xdbb0
 80031f4: aa26         	add	r2, sp, #0x98
 80031f6: f6c0 0000    	movt	r0, #0x800
 80031fa: f6c0 0300    	movt	r3, #0x800
 80031fe: 212b         	movs	r1, #0x2b
 8003200: f008 fa10    	bl	0x800b624 <core::result::unwrap_failed> @ imm = #0x8420
 8003204: f89d 20a1    	ldrb.w	r2, [sp, #0xa1]
 8003208: 2a1e         	cmp	r2, #0x1e
 800320a: d8e7         	bhi	0x80031dc <raylar_crcl_encoder_service::benchmark::serialize_production::h9a703500dd702791+0x60> @ imm = #-0x32
 800320c: f89d 30a3    	ldrb.w	r3, [sp, #0xa3]
 8003210: 4681         	mov	r9, r0
 8003212: 2b01         	cmp	r3, #0x1
 8003214: d107         	bne	0x8003226 <raylar_crcl_encoder_service::benchmark::serialize_production::h9a703500dd702791+0xaa> @ imm = #0xe
 8003216: f5b1 6f80    	cmp.w	r1, #0x400
 800321a: d3df         	blo	0x80031dc <raylar_crcl_encoder_service::benchmark::serialize_production::h9a703500dd702791+0x60> @ imm = #-0x42
 800321c: f89d 10a2    	ldrb.w	r1, [sp, #0xa2]
 8003220: 2901         	cmp	r1, #0x1
 8003222: d005         	beq	0x8003230 <raylar_crcl_encoder_service::benchmark::serialize_production::h9a703500dd702791+0xb4> @ imm = #0xa
 8003224: e7da         	b	0x80031dc <raylar_crcl_encoder_service::benchmark::serialize_production::h9a703500dd702791+0x60> @ imm = #-0x4c
 8003226: f5b1 6f80    	cmp.w	r1, #0x400
 800322a: d3d7         	blo	0x80031dc <raylar_crcl_encoder_service::benchmark::serialize_production::h9a703500dd702791+0x60> @ imm = #-0x52
 800322c: 2a00         	cmp	r2, #0x0
 800322e: d1d5         	bne	0x80031dc <raylar_crcl_encoder_service::benchmark::serialize_production::h9a703500dd702791+0x60> @ imm = #-0x56
 8003230: f246 0128    	movw	r1, #0x6028
 8003234: eb09 0201    	add.w	r2, r9, r1
 8003238: 9211         	str	r2, [sp, #0x44]
 800323a: f44f 52c0    	mov.w	r2, #0x1800
 800323e: f04f 0e00    	mov.w	lr, #0x0
 8003242: e9dd 3626    	ldrd	r3, r6, [sp, #152]
 8003246: 9d28         	ldr	r5, [sp, #0xa0]
 8003248: 9212         	str	r2, [sp, #0x48]
 800324a: 2206         	movs	r2, #0x6
 800324c: e9cd 3622    	strd	r3, r6, [sp, #136]
 8003250: 464b         	mov	r3, r9
 8003252: 921d         	str	r2, [sp, #0x74]
 8003254: aa01         	add	r2, sp, #0x4
 8003256: f8ad e020    	strh.w	lr, [sp, #0x20]
 800325a: 3248         	adds	r2, #0x48
 800325c: e9cd ee06    	strd	lr, lr, [sp, #24]
 8003260: e9cd ee04    	strd	lr, lr, [sp, #16]
 8003264: e9cd ee02    	strd	lr, lr, [sp, #8]
 8003268: e9cd ee0a    	strd	lr, lr, [sp, #40]
 800326c: e9cd ee0c    	strd	lr, lr, [sp, #48]
 8003270: e9cd ee0e    	strd	lr, lr, [sp, #56]
 8003274: f8ad e040    	strh.w	lr, [sp, #0x40]
 8003278: e9cd ee1e    	strd	lr, lr, [sp, #120]
 800327c: e9cd ee20    	strd	lr, lr, [sp, #128]
 8003280: f8cd e004    	str.w	lr, [sp, #0x4]
 8003284: f8cd e024    	str.w	lr, [sp, #0x24]
 8003288: 9524         	str	r5, [sp, #0x90]
 800328a: f8ad e094    	strh.w	lr, [sp, #0x94]
 800328e: cb73         	ldm	r3!, {r0, r1, r4, r5, r6}
 8003290: c273         	stm	r2!, {r0, r1, r4, r5, r6}
 8003292: e893 0073    	ldm.w	r3, {r0, r1, r4, r5, r6}
 8003296: c273         	stm	r2!, {r0, r1, r4, r5, r6}
 8003298: f44f 0000    	mov.w	r0, #0x800000
 800329c: 9036         	str	r0, [sp, #0xd8]
 800329e: f44f 6080    	mov.w	r0, #0x400
 80032a2: 9035         	str	r0, [sp, #0xd4]
 80032a4: a835         	add	r0, sp, #0xd4
 80032a6: f8cd c0dc    	str.w	r12, [sp, #0xdc]
 80032aa: 9835         	ldr	r0, [sp, #0xd4]
 80032ac: 1e41         	subs	r1, r0, #0x1
 80032ae: ea80 0201    	eor.w	r2, r0, r1
 80032b2: 428a         	cmp	r2, r1
 80032b4: d90a         	bls	0x80032cc <raylar_crcl_encoder_service::benchmark::serialize_production::h9a703500dd702791+0x150> @ imm = #0x14
 80032b6: fa90 f0a0    	rbit	r0, r0
 80032ba: fab0 f080    	clz	r0, r0
 80032be: 3808         	subs	r0, #0x8
 80032c0: 2804         	cmp	r0, #0x4
 80032c2: bf9c         	itt	ls
 80032c4: f89d 00dc    	ldrbls.w	r0, [sp, #0xdc]
 80032c8: 280c         	cmpls	r0, #0xc
 80032ca: d906         	bls	0x80032da <raylar_crcl_encoder_service::benchmark::serialize_production::h9a703500dd702791+0x15e> @ imm = #0xc
 80032cc: f64d 5088    	movw	r0, #0xdd88
 80032d0: f88d e098    	strb.w	lr, [sp, #0x98]
 80032d4: f6c0 0000    	movt	r0, #0x800
 80032d8: e787         	b	0x80031ea <raylar_crcl_encoder_service::benchmark::serialize_production::h9a703500dd702791+0x6e> @ imm = #-0xf2
 80032da: 2101         	movs	r1, #0x1
 80032dc: fa01 f000    	lsl.w	r0, r1, r0
 80032e0: f410 5faa    	tst.w	r0, #0x1540
 80032e4: d0f2         	beq	0x80032cc <raylar_crcl_encoder_service::benchmark::serialize_production::h9a703500dd702791+0x150> @ imm = #-0x1c
 80032e6: 9936         	ldr	r1, [sp, #0xd8]
 80032e8: 2001         	movs	r0, #0x1
 80032ea: f2cc 0000    	movt	r0, #0xc000
 80032ee: f101 4140    	add.w	r1, r1, #0xc0000000
 80032f2: 4281         	cmp	r1, r0
 80032f4: d3ea         	blo	0x80032cc <raylar_crcl_encoder_service::benchmark::serialize_production::h9a703500dd702791+0x150> @ imm = #-0x2c
 80032f6: f89d 00dd    	ldrb.w	r0, [sp, #0xdd]
 80032fa: 281e         	cmp	r0, #0x1e
 80032fc: d8e6         	bhi	0x80032cc <raylar_crcl_encoder_service::benchmark::serialize_production::h9a703500dd702791+0x150> @ imm = #-0x34
 80032fe: f89d 60df    	ldrb.w	r6, [sp, #0xdf]
 8003302: 2e01         	cmp	r6, #0x1
 8003304: d104         	bne	0x8003310 <raylar_crcl_encoder_service::benchmark::serialize_production::h9a703500dd702791+0x194> @ imm = #0x8
 8003306: f89d 00de    	ldrb.w	r0, [sp, #0xde]
 800330a: 2801         	cmp	r0, #0x1
 800330c: d002         	beq	0x8003314 <raylar_crcl_encoder_service::benchmark::serialize_production::h9a703500dd702791+0x198> @ imm = #0x4
 800330e: e7dd         	b	0x80032cc <raylar_crcl_encoder_service::benchmark::serialize_production::h9a703500dd702791+0x150> @ imm = #-0x46
 8003310: 2800         	cmp	r0, #0x0
 8003312: d1db         	bne	0x80032cc <raylar_crcl_encoder_service::benchmark::serialize_production::h9a703500dd702791+0x150> @ imm = #-0x4a
 8003314: e9dd 0135    	ldrd	r0, r1, [sp, #212]
 8003318: f8dd 20db    	ldr.w	r2, [sp, #0xdb]
 800331c: f8cd 00c7    	str.w	r0, [sp, #0xc7]
 8003320: 2000         	movs	r0, #0x0
 8003322: f8cd 20ce    	str.w	r2, [sp, #0xce]
 8003326: 221a         	movs	r2, #0x1a
 8003328: f8cd 10cb    	str.w	r1, [sp, #0xcb]
 800332c: a92e         	add	r1, sp, #0xb8
 800332e: 9030         	str	r0, [sp, #0xc0]
 8003330: e9cd 002e    	strd	r0, r0, [sp, #184]
 8003334: f8cd 00c3    	str.w	r0, [sp, #0xc3]
 8003338: f88d 0098    	strb.w	r0, [sp, #0x98]
 800333c: f108 0001    	add.w	r0, r8, #0x1
 8003340: f009 f9ac    	bl	0x800c69c <__aeabi_memcpy> @ imm = #0x9358
 8003344: f641 6058    	movw	r0, #0x1e58
 8003348: f509 34e0    	add.w	r4, r9, #0x1c000
 800334c: 9000         	str	r0, [sp]
 800334e: a82e         	add	r0, sp, #0xb8
 8003350: aa01         	add	r2, sp, #0x4
 8003352: 4641         	mov	r1, r8
 8003354: 4623         	mov	r3, r4
 8003356: f88d 60b3    	strb.w	r6, [sp, #0xb3]
 800335a: f7fd ffc9    	bl	0x80012f0 <raylar_crcl_encoder_service::format::Encoder::data::ha65fe528befbf4e3> @ imm = #-0x206e
 800335e: f89d 00b8    	ldrb.w	r0, [sp, #0xb8]
 8003362: 2801         	cmp	r0, #0x1
 8003364: d015         	beq	0x8003392 <raylar_crcl_encoder_service::benchmark::serialize_production::h9a703500dd702791+0x216> @ imm = #0x2a
 8003366: 992f         	ldr	r1, [sp, #0xbc]
 8003368: f641 6059    	movw	r0, #0x1e59
 800336c: 4281         	cmp	r1, r0
 800336e: d207         	bhs	0x8003380 <raylar_crcl_encoder_service::benchmark::serialize_production::h9a703500dd702791+0x204> @ imm = #0xe
 8003370: e9cd 4126    	strd	r4, r1, [sp, #152]
 8003374: a826         	add	r0, sp, #0x98
 8003376: 4608         	mov	r0, r1
 8003378: b038         	add	sp, #0xe0
 800337a: e8bd 0b00    	pop.w	{r8, r9, r11}
 800337e: bdf0         	pop	{r4, r5, r6, r7, pc}
 8003380: f64d 5368    	movw	r3, #0xdd68
 8003384: 2000         	movs	r0, #0x0
 8003386: f6c0 0300    	movt	r3, #0x800
 800338a: f641 6258    	movw	r2, #0x1e58
 800338e: f008 fc54    	bl	0x800bc3a <core::slice::index::slice_index_fail> @ imm = #0x88a8
 8003392: f89d 00b9    	ldrb.w	r0, [sp, #0xb9]
 8003396: f64d 434c    	movw	r3, #0xdc4c
 800339a: f88d 00d4    	strb.w	r0, [sp, #0xd4]
 800339e: f64d 5078    	movw	r0, #0xdd78
 80033a2: f6c0 0000    	movt	r0, #0x800
 80033a6: aa35         	add	r2, sp, #0xd4
 80033a8: 9000         	str	r0, [sp]
 80033aa: f64d 30b0    	movw	r0, #0xdbb0
 80033ae: f6c0 0000    	movt	r0, #0x800
 80033b2: f6c0 0300    	movt	r3, #0x800
 80033b6: 212b         	movs	r1, #0x2b
 80033b8: f008 f934    	bl	0x800b624 <core::result::unwrap_failed> @ imm = #0x8268

080033bc <raylar_crcl_encoder_service::benchmark::transform_production::he43415fd0806c08a>:
 80033bc: b5f0         	push	{r4, r5, r6, r7, lr}
 80033be: af03         	add	r7, sp, #0xc
 80033c0: f84d 8d04    	str	r8, [sp, #-4]!
 80033c4: b0aa         	sub	sp, #0xa8
 80033c6: f240 610a    	movw	r1, #0x60a
 80033ca: f2c0 1101    	movt	r1, #0x101
 80033ce: 9129         	str	r1, [sp, #0xa4]
 80033d0: f44f 0100    	mov.w	r1, #0x800000
 80033d4: 9128         	str	r1, [sp, #0xa0]
 80033d6: f44f 6180    	mov.w	r1, #0x400
 80033da: 9127         	str	r1, [sp, #0x9c]
 80033dc: a927         	add	r1, sp, #0x9c
 80033de: 9927         	ldr	r1, [sp, #0x9c]
 80033e0: 1e4a         	subs	r2, r1, #0x1
 80033e2: ea81 0302    	eor.w	r3, r1, r2
 80033e6: 4293         	cmp	r3, r2
 80033e8: d916         	bls	0x8003418 <raylar_crcl_encoder_service::benchmark::transform_production::he43415fd0806c08a+0x5c> @ imm = #0x2c
 80033ea: fa91 f2a1    	rbit	r2, r1
 80033ee: fab2 f282    	clz	r2, r2
 80033f2: 3a08         	subs	r2, #0x8
 80033f4: 2a04         	cmp	r2, #0x4
 80033f6: bf9c         	itt	ls
 80033f8: f89d 20a4    	ldrbls.w	r2, [sp, #0xa4]
 80033fc: 2a0c         	cmpls	r2, #0xc
 80033fe: d80b         	bhi	0x8003418 <raylar_crcl_encoder_service::benchmark::transform_production::he43415fd0806c08a+0x5c> @ imm = #0x16
 8003400: 2301         	movs	r3, #0x1
 8003402: fa03 f202    	lsl.w	r2, r3, r2
 8003406: f412 5faa    	tst.w	r2, #0x1540
 800340a: bf1e         	ittt	ne
 800340c: 9a28         	ldrne	r2, [sp, #0xa0]
 800340e: f102 4240    	addne.w	r2, r2, #0xc0000000
 8003412: f1b2 4f40    	cmpne.w	r2, #0xc0000000
 8003416: d813         	bhi	0x8003440 <raylar_crcl_encoder_service::benchmark::transform_production::he43415fd0806c08a+0x84> @ imm = #0x26
 8003418: 2000         	movs	r0, #0x0
 800341a: f64d 434c    	movw	r3, #0xdc4c
 800341e: f88d 0008    	strb.w	r0, [sp, #0x8]
 8003422: f64d 50b8    	movw	r0, #0xddb8
 8003426: f6c0 0000    	movt	r0, #0x800
 800342a: aa02         	add	r2, sp, #0x8
 800342c: 9000         	str	r0, [sp]
 800342e: f64d 30b0    	movw	r0, #0xdbb0
 8003432: f6c0 0000    	movt	r0, #0x800
 8003436: f6c0 0300    	movt	r3, #0x800
 800343a: 212b         	movs	r1, #0x2b
 800343c: f008 f8f2    	bl	0x800b624 <core::result::unwrap_failed> @ imm = #0x81e4
 8003440: f89d 20a5    	ldrb.w	r2, [sp, #0xa5]
 8003444: 2a1e         	cmp	r2, #0x1e
 8003446: d8e7         	bhi	0x8003418 <raylar_crcl_encoder_service::benchmark::transform_production::he43415fd0806c08a+0x5c> @ imm = #-0x32
 8003448: f89d 30a7    	ldrb.w	r3, [sp, #0xa7]
 800344c: 2b01         	cmp	r3, #0x1
 800344e: d107         	bne	0x8003460 <raylar_crcl_encoder_service::benchmark::transform_production::he43415fd0806c08a+0xa4> @ imm = #0xe
 8003450: f5b1 6f80    	cmp.w	r1, #0x400
 8003454: d3e0         	blo	0x8003418 <raylar_crcl_encoder_service::benchmark::transform_production::he43415fd0806c08a+0x5c> @ imm = #-0x40
 8003456: f89d 10a6    	ldrb.w	r1, [sp, #0xa6]
 800345a: 2901         	cmp	r1, #0x1
 800345c: d005         	beq	0x800346a <raylar_crcl_encoder_service::benchmark::transform_production::he43415fd0806c08a+0xae> @ imm = #0xa
 800345e: e7db         	b	0x8003418 <raylar_crcl_encoder_service::benchmark::transform_production::he43415fd0806c08a+0x5c> @ imm = #-0x4a
 8003460: f5b1 6f80    	cmp.w	r1, #0x400
 8003464: d3d8         	blo	0x8003418 <raylar_crcl_encoder_service::benchmark::transform_production::he43415fd0806c08a+0x5c> @ imm = #-0x50
 8003466: 2a00         	cmp	r2, #0x0
 8003468: d1d6         	bne	0x8003418 <raylar_crcl_encoder_service::benchmark::transform_production::he43415fd0806c08a+0x5c> @ imm = #-0x54
 800346a: ac02         	add	r4, sp, #0x8
 800346c: f246 0128    	movw	r1, #0x6028
 8003470: 1845         	adds	r5, r0, r1
 8003472: f104 0048    	add.w	r0, r4, #0x48
 8003476: 2128         	movs	r1, #0x28
 8003478: f009 fb39    	bl	0x800caee <__aeabi_memclr4> @ imm = #0x9672
 800347c: ab27         	add	r3, sp, #0x9c
 800347e: 9512         	str	r5, [sp, #0x48]
 8003480: f44f 56c0    	mov.w	r6, #0x1800
 8003484: f104 0840    	add.w	r8, r4, #0x40
 8003488: cb0e         	ldm	r3, {r1, r2, r3}
 800348a: ad27         	add	r5, sp, #0x9c
 800348c: 2000         	movs	r0, #0x0
 800348e: 9613         	str	r6, [sp, #0x4c]
 8003490: f04f 36ff    	mov.w	r6, #0xffffffff
 8003494: e9cd 2324    	strd	r2, r3, [sp, #144]
 8003498: 2201         	movs	r2, #0x1
 800349a: f8ad 0024    	strh.w	r0, [sp, #0x24]
 800349e: e9cd 0007    	strd	r0, r0, [sp, #28]
 80034a2: e9cd 0005    	strd	r0, r0, [sp, #20]
 80034a6: e9cd 0003    	strd	r0, r0, [sp, #12]
 80034aa: e9cd 000b    	strd	r0, r0, [sp, #44]
 80034ae: e9cd 000d    	strd	r0, r0, [sp, #52]
 80034b2: e9cd 000f    	strd	r0, r0, [sp, #60]
 80034b6: f8ad 0044    	strh.w	r0, [sp, #0x44]
 80034ba: e9cd 001f    	strd	r0, r0, [sp, #124]
 80034be: e9cd 0021    	strd	r0, r0, [sp, #132]
 80034c2: 9002         	str	r0, [sp, #0x8]
 80034c4: 900a         	str	r0, [sp, #0x28]
 80034c6: 921e         	str	r2, [sp, #0x78]
 80034c8: 9123         	str	r1, [sp, #0x8c]
 80034ca: f8ad 0098    	strh.w	r0, [sp, #0x98]
 80034ce: 9627         	str	r6, [sp, #0x9c]
 80034d0: 4620         	mov	r0, r4
 80034d2: 9927         	ldr	r1, [sp, #0x9c]
 80034d4: f7fe f9c3    	bl	0x800185e <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3> @ imm = #-0x1c7a
 80034d8: 07c0         	lsls	r0, r0, #0x1f
 80034da: d109         	bne	0x80034f0 <raylar_crcl_encoder_service::benchmark::transform_production::he43415fd0806c08a+0x134> @ imm = #0x12
 80034dc: 07c8         	lsls	r0, r1, #0x1f
 80034de: d0f6         	beq	0x80034ce <raylar_crcl_encoder_service::benchmark::transform_production::he43415fd0806c08a+0x112> @ imm = #-0x14
 80034e0: f8cd 809c    	str.w	r8, [sp, #0x9c]
 80034e4: a827         	add	r0, sp, #0x9c
 80034e6: 9814         	ldr	r0, [sp, #0x50]
 80034e8: b02a         	add	sp, #0xa8
 80034ea: f85d 8b04    	ldr	r8, [sp], #4
 80034ee: bdf0         	pop	{r4, r5, r6, r7, pc}
 80034f0: f64d 50a8    	movw	r0, #0xdda8
 80034f4: f64d 434c    	movw	r3, #0xdc4c
 80034f8: f6c0 0000    	movt	r0, #0x800
 80034fc: aa27         	add	r2, sp, #0x9c
 80034fe: 9000         	str	r0, [sp]
 8003500: f64d 30b0    	movw	r0, #0xdbb0
 8003504: f88d 109c    	strb.w	r1, [sp, #0x9c]
 8003508: f6c0 0000    	movt	r0, #0x800
 800350c: f6c0 0300    	movt	r3, #0x800
 8003510: 212b         	movs	r1, #0x2b
 8003512: f008 f887    	bl	0x800b624 <core::result::unwrap_failed> @ imm = #0x810e

08003516 <raylar_crcl_encoder_service::benchmark::raw::hd2aad415d886ff2c>:
 8003516: b580         	push	{r7, lr}
 8003518: 466f         	mov	r7, sp
 800351a: f246 0128    	movw	r1, #0x6028
 800351e: 1842         	adds	r2, r0, r1
 8003520: f100 0128    	add.w	r1, r0, #0x28
 8003524: 4610         	mov	r0, r2
 8003526: f44f 42c0    	mov.w	r2, #0x6000
 800352a: f009 fa92    	bl	0x800ca52 <__aeabi_memcpy4> @ imm = #0x9524
 800352e: bd80         	pop	{r7, pc}

08003530 <raylar_crcl_encoder_service::benchmark::copy::hf53d4786c081c67d>:
 8003530: b5f0         	push	{r4, r5, r6, r7, lr}
 8003532: af03         	add	r7, sp, #0xc
 8003534: f84d bd04    	str	r11, [sp, #-4]!
 8003538: b082         	sub	sp, #0x8
 800353a: f246 0628    	movw	r6, #0x6028
 800353e: 1985         	adds	r5, r0, r6
 8003540: f100 0128    	add.w	r1, r0, #0x28
 8003544: 4604         	mov	r4, r0
 8003546: 4628         	mov	r0, r5
 8003548: f44f 42c0    	mov.w	r2, #0x6000
 800354c: f009 fa81    	bl	0x800ca52 <__aeabi_memcpy4> @ imm = #0x9502
 8003550: 9501         	str	r5, [sp, #0x4]
 8003552: a801         	add	r0, sp, #0x4
 8003554: 59a0         	ldr	r0, [r4, r6]
 8003556: b002         	add	sp, #0x8
 8003558: f85d bb04    	ldr	r11, [sp], #4
 800355c: bdf0         	pop	{r4, r5, r6, r7, pc}

0800355e <raylar_crcl_encoder_service::benchmark::peak::he0ae015b97d7ff8e>:
 800355e: b5f0         	push	{r4, r5, r6, r7, lr}
 8003560: af03         	add	r7, sp, #0xc
 8003562: e92d 0f00    	push.w	{r8, r9, r10, r11}
 8003566: b087         	sub	sp, #0x1c
 8003568: f04f 0a00    	mov.w	r10, #0x0
 800356c: f46f 0900    	mvn	r9, #0x800000
 8003570: f06f 7880    	mvn	r8, #0x1000000
 8003574: f04f 0b00    	mov.w	r11, #0x0
 8003578: f246 018c    	movw	r1, #0x608c
 800357c: 9004         	str	r0, [sp, #0x10]
 800357e: 4408         	add	r0, r1
 8003580: 9005         	str	r0, [sp, #0x14]
 8003582: 9805         	ldr	r0, [sp, #0x14]
 8003584: f64f 72fe    	movw	r2, #0xfffe
 8003588: f6cf 62ff    	movt	r2, #0xfeff
 800358c: eb00 060b    	add.w	r6, r0, r11
 8003590: f856 0c60    	ldr	r0, [r6, #-96]
 8003594: eb00 0109    	add.w	r1, r0, r9
 8003598: 4291         	cmp	r1, r2
 800359a: f240 8281    	bls.w	0x8003aa0 <raylar_crcl_encoder_service::benchmark::peak::he0ae015b97d7ff8e+0x542> @ imm = #0x502
 800359e: f856 1c48    	ldr	r1, [r6, #-72]
 80035a2: eb01 0209    	add.w	r2, r1, r9
 80035a6: 4542         	cmp	r2, r8
 80035a8: f0c0 827a    	blo.w	0x8003aa0 <raylar_crcl_encoder_service::benchmark::peak::he0ae015b97d7ff8e+0x542> @ imm = #0x4f4
 80035ac: f856 5c30    	ldr	r5, [r6, #-48]
 80035b0: eb05 0209    	add.w	r2, r5, r9
 80035b4: 4542         	cmp	r2, r8
 80035b6: bf22         	ittt	hs
 80035b8: f856 2c18    	ldrhs	r2, [r6, #-24]
 80035bc: eb02 0309    	addhs.w	r3, r2, r9
 80035c0: 4543         	cmphs	r3, r8
 80035c2: f0c0 826d    	blo.w	0x8003aa0 <raylar_crcl_encoder_service::benchmark::peak::he0ae015b97d7ff8e+0x542> @ imm = #0x4da
 80035c6: 9b05         	ldr	r3, [sp, #0x14]
 80035c8: f853 300b    	ldr.w	r3, [r3, r11]
 80035cc: eb03 0409    	add.w	r4, r3, r9
 80035d0: 4544         	cmp	r4, r8
 80035d2: bf22         	ittt	hs
 80035d4: f8d6 e018    	ldrhs.w	lr, [r6, #0x18]
 80035d8: eb0e 0409    	addhs.w	r4, lr, r9
 80035dc: 4544         	cmphs	r4, r8
 80035de: f0c0 825f    	blo.w	0x8003aa0 <raylar_crcl_encoder_service::benchmark::peak::he0ae015b97d7ff8e+0x542> @ imm = #0x4be
 80035e2: f8d6 c030    	ldr.w	r12, [r6, #0x30]
 80035e6: eb0c 0409    	add.w	r4, r12, r9
 80035ea: 4544         	cmp	r4, r8
 80035ec: bf22         	ittt	hs
 80035ee: 6cb6         	ldrhs	r6, [r6, #0x48]
 80035f0: eb06 0409    	addhs.w	r4, r6, r9
 80035f4: 4544         	cmphs	r4, r8
 80035f6: f0c0 8253    	blo.w	0x8003aa0 <raylar_crcl_encoder_service::benchmark::peak::he0ae015b97d7ff8e+0x542> @ imm = #0x4a6
 80035fa: 2800         	cmp	r0, #0x0
 80035fc: bf48         	it	mi
 80035fe: 4240         	rsbmi	r0, r0, #0
 8003600: 4550         	cmp	r0, r10
 8003602: bf98         	it	ls
 8003604: 4650         	movls	r0, r10
 8003606: 2900         	cmp	r1, #0x0
 8003608: bf48         	it	mi
 800360a: 4249         	rsbmi	r1, r1, #0
 800360c: 4281         	cmp	r1, r0
 800360e: bf98         	it	ls
 8003610: 4601         	movls	r1, r0
 8003612: 2d00         	cmp	r5, #0x0
 8003614: bf48         	it	mi
 8003616: 426d         	rsbmi	r5, r5, #0
 8003618: 428d         	cmp	r5, r1
 800361a: bf98         	it	ls
 800361c: 460d         	movls	r5, r1
 800361e: 2a00         	cmp	r2, #0x0
 8003620: bf48         	it	mi
 8003622: 4252         	rsbmi	r2, r2, #0
 8003624: 42aa         	cmp	r2, r5
 8003626: bf98         	it	ls
 8003628: 462a         	movls	r2, r5
 800362a: 2b00         	cmp	r3, #0x0
 800362c: bf48         	it	mi
 800362e: 425b         	rsbmi	r3, r3, #0
 8003630: 4293         	cmp	r3, r2
 8003632: bf98         	it	ls
 8003634: 4613         	movls	r3, r2
 8003636: f1be 0f00    	cmp.w	lr, #0x0
 800363a: bf48         	it	mi
 800363c: f1ce 0e00    	rsbmi.w	lr, lr, #0x0
 8003640: 459e         	cmp	lr, r3
 8003642: bf98         	it	ls
 8003644: 469e         	movls	lr, r3
 8003646: f1bc 0f00    	cmp.w	r12, #0x0
 800364a: bf48         	it	mi
 800364c: f1cc 0c00    	rsbmi.w	r12, r12, #0x0
 8003650: 45f4         	cmp	r12, lr
 8003652: bf98         	it	ls
 8003654: 46f4         	movls	r12, lr
 8003656: 2e00         	cmp	r6, #0x0
 8003658: bf48         	it	mi
 800365a: 4276         	rsbmi	r6, r6, #0
 800365c: f10b 0bc0    	add.w	r11, r11, #0xc0
 8003660: 4566         	cmp	r6, r12
 8003662: bf98         	it	ls
 8003664: 4666         	movls	r6, r12
 8003666: 46b2         	mov	r10, r6
 8003668: f5bb 4fc0    	cmp.w	r11, #0x6000
 800366c: d189         	bne	0x8003582 <raylar_crcl_encoder_service::benchmark::peak::he0ae015b97d7ff8e+0x24> @ imm = #-0xee
 800366e: 9606         	str	r6, [sp, #0x18]
 8003670: a806         	add	r0, sp, #0x18
 8003672: f246 0090    	movw	r0, #0x6090
 8003676: f04f 0900    	mov.w	r9, #0x0
 800367a: 9906         	ldr	r1, [sp, #0x18]
 800367c: f46f 0e00    	mvn	lr, #0x800000
 8003680: 9103         	str	r1, [sp, #0xc]
 8003682: f06f 7880    	mvn	r8, #0x1000000
 8003686: 9904         	ldr	r1, [sp, #0x10]
 8003688: f04f 0a00    	mov.w	r10, #0x0
 800368c: 4408         	add	r0, r1
 800368e: 9005         	str	r0, [sp, #0x14]
 8003690: 9805         	ldr	r0, [sp, #0x14]
 8003692: eb00 060a    	add.w	r6, r0, r10
 8003696: f856 0c60    	ldr	r0, [r6, #-96]
 800369a: eb00 010e    	add.w	r1, r0, lr
 800369e: 4541         	cmp	r1, r8
 80036a0: bf22         	ittt	hs
 80036a2: f856 1c48    	ldrhs	r1, [r6, #-72]
 80036a6: eb01 020e    	addhs.w	r2, r1, lr
 80036aa: 4542         	cmphs	r2, r8
 80036ac: f0c0 81f8    	blo.w	0x8003aa0 <raylar_crcl_encoder_service::benchmark::peak::he0ae015b97d7ff8e+0x542> @ imm = #0x3f0
 80036b0: f856 2c30    	ldr	r2, [r6, #-48]
 80036b4: eb02 030e    	add.w	r3, r2, lr
 80036b8: 4543         	cmp	r3, r8
 80036ba: bf22         	ittt	hs
 80036bc: f856 3c18    	ldrhs	r3, [r6, #-24]
 80036c0: eb03 040e    	addhs.w	r4, r3, lr
 80036c4: 4544         	cmphs	r4, r8
 80036c6: f0c0 81eb    	blo.w	0x8003aa0 <raylar_crcl_encoder_service::benchmark::peak::he0ae015b97d7ff8e+0x542> @ imm = #0x3d6
 80036ca: 9d05         	ldr	r5, [sp, #0x14]
 80036cc: f855 400a    	ldr.w	r4, [r5, r10]
 80036d0: eb04 050e    	add.w	r5, r4, lr
 80036d4: 4545         	cmp	r5, r8
 80036d6: bf22         	ittt	hs
 80036d8: f8d6 c018    	ldrhs.w	r12, [r6, #0x18]
 80036dc: eb0c 050e    	addhs.w	r5, r12, lr
 80036e0: 4545         	cmphs	r5, r8
 80036e2: f0c0 81dd    	blo.w	0x8003aa0 <raylar_crcl_encoder_service::benchmark::peak::he0ae015b97d7ff8e+0x542> @ imm = #0x3ba
 80036e6: f8d6 b030    	ldr.w	r11, [r6, #0x30]
 80036ea: eb0b 050e    	add.w	r5, r11, lr
 80036ee: 4545         	cmp	r5, r8
 80036f0: bf22         	ittt	hs
 80036f2: 6cb6         	ldrhs	r6, [r6, #0x48]
 80036f4: eb06 050e    	addhs.w	r5, r6, lr
 80036f8: 4545         	cmphs	r5, r8
 80036fa: f0c0 81d1    	blo.w	0x8003aa0 <raylar_crcl_encoder_service::benchmark::peak::he0ae015b97d7ff8e+0x542> @ imm = #0x3a2
 80036fe: 2800         	cmp	r0, #0x0
 8003700: bf48         	it	mi
 8003702: 4240         	rsbmi	r0, r0, #0
 8003704: 4548         	cmp	r0, r9
 8003706: bf98         	it	ls
 8003708: 4648         	movls	r0, r9
 800370a: 2900         	cmp	r1, #0x0
 800370c: bf48         	it	mi
 800370e: 4249         	rsbmi	r1, r1, #0
 8003710: 4281         	cmp	r1, r0
 8003712: bf98         	it	ls
 8003714: 4601         	movls	r1, r0
 8003716: 2a00         	cmp	r2, #0x0
 8003718: bf48         	it	mi
 800371a: 4252         	rsbmi	r2, r2, #0
 800371c: 428a         	cmp	r2, r1
 800371e: bf98         	it	ls
 8003720: 460a         	movls	r2, r1
 8003722: 2b00         	cmp	r3, #0x0
 8003724: bf48         	it	mi
 8003726: 425b         	rsbmi	r3, r3, #0
 8003728: 4293         	cmp	r3, r2
 800372a: bf98         	it	ls
 800372c: 4613         	movls	r3, r2
 800372e: 2c00         	cmp	r4, #0x0
 8003730: bf48         	it	mi
 8003732: 4264         	rsbmi	r4, r4, #0
 8003734: 429c         	cmp	r4, r3
 8003736: bf98         	it	ls
 8003738: 461c         	movls	r4, r3
 800373a: f1bc 0f00    	cmp.w	r12, #0x0
 800373e: bf48         	it	mi
 8003740: f1cc 0c00    	rsbmi.w	r12, r12, #0x0
 8003744: 45a4         	cmp	r12, r4
 8003746: bf98         	it	ls
 8003748: 46a4         	movls	r12, r4
 800374a: f1bb 0f00    	cmp.w	r11, #0x0
 800374e: bf48         	it	mi
 8003750: f1cb 0b00    	rsbmi.w	r11, r11, #0x0
 8003754: 45e3         	cmp	r11, r12
 8003756: bf98         	it	ls
 8003758: 46e3         	movls	r11, r12
 800375a: 2e00         	cmp	r6, #0x0
 800375c: bf48         	it	mi
 800375e: 4276         	rsbmi	r6, r6, #0
 8003760: f10a 0ac0    	add.w	r10, r10, #0xc0
 8003764: 455e         	cmp	r6, r11
 8003766: bf98         	it	ls
 8003768: 465e         	movls	r6, r11
 800376a: 46b1         	mov	r9, r6
 800376c: f5ba 4fc0    	cmp.w	r10, #0x6000
 8003770: d18e         	bne	0x8003690 <raylar_crcl_encoder_service::benchmark::peak::he0ae015b97d7ff8e+0x132> @ imm = #-0xe4
 8003772: 9606         	str	r6, [sp, #0x18]
 8003774: a806         	add	r0, sp, #0x18
 8003776: f246 0094    	movw	r0, #0x6094
 800377a: f04f 0900    	mov.w	r9, #0x0
 800377e: 9906         	ldr	r1, [sp, #0x18]
 8003780: f46f 0e00    	mvn	lr, #0x800000
 8003784: 9102         	str	r1, [sp, #0x8]
 8003786: f06f 7880    	mvn	r8, #0x1000000
 800378a: 9904         	ldr	r1, [sp, #0x10]
 800378c: f04f 0a00    	mov.w	r10, #0x0
 8003790: 4408         	add	r0, r1
 8003792: 9005         	str	r0, [sp, #0x14]
 8003794: 9805         	ldr	r0, [sp, #0x14]
 8003796: eb00 060a    	add.w	r6, r0, r10
 800379a: f856 0c60    	ldr	r0, [r6, #-96]
 800379e: eb00 010e    	add.w	r1, r0, lr
 80037a2: 4541         	cmp	r1, r8
 80037a4: bf22         	ittt	hs
 80037a6: f856 1c48    	ldrhs	r1, [r6, #-72]
 80037aa: eb01 020e    	addhs.w	r2, r1, lr
 80037ae: 4542         	cmphs	r2, r8
 80037b0: f0c0 8176    	blo.w	0x8003aa0 <raylar_crcl_encoder_service::benchmark::peak::he0ae015b97d7ff8e+0x542> @ imm = #0x2ec
 80037b4: f856 2c30    	ldr	r2, [r6, #-48]
 80037b8: eb02 030e    	add.w	r3, r2, lr
 80037bc: 4543         	cmp	r3, r8
 80037be: bf22         	ittt	hs
 80037c0: f856 3c18    	ldrhs	r3, [r6, #-24]
 80037c4: eb03 040e    	addhs.w	r4, r3, lr
 80037c8: 4544         	cmphs	r4, r8
 80037ca: f0c0 8169    	blo.w	0x8003aa0 <raylar_crcl_encoder_service::benchmark::peak::he0ae015b97d7ff8e+0x542> @ imm = #0x2d2
 80037ce: 9d05         	ldr	r5, [sp, #0x14]
 80037d0: f855 500a    	ldr.w	r5, [r5, r10]
 80037d4: eb05 040e    	add.w	r4, r5, lr
 80037d8: 4544         	cmp	r4, r8
 80037da: bf22         	ittt	hs
 80037dc: f8d6 c018    	ldrhs.w	r12, [r6, #0x18]
 80037e0: eb0c 040e    	addhs.w	r4, r12, lr
 80037e4: 4544         	cmphs	r4, r8
 80037e6: f0c0 815b    	blo.w	0x8003aa0 <raylar_crcl_encoder_service::benchmark::peak::he0ae015b97d7ff8e+0x542> @ imm = #0x2b6
 80037ea: f8d6 b030    	ldr.w	r11, [r6, #0x30]
 80037ee: eb0b 040e    	add.w	r4, r11, lr
 80037f2: 4544         	cmp	r4, r8
 80037f4: bf22         	ittt	hs
 80037f6: 6cb6         	ldrhs	r6, [r6, #0x48]
 80037f8: eb06 040e    	addhs.w	r4, r6, lr
 80037fc: 4544         	cmphs	r4, r8
 80037fe: f0c0 814f    	blo.w	0x8003aa0 <raylar_crcl_encoder_service::benchmark::peak::he0ae015b97d7ff8e+0x542> @ imm = #0x29e
 8003802: 2800         	cmp	r0, #0x0
 8003804: bf48         	it	mi
 8003806: 4240         	rsbmi	r0, r0, #0
 8003808: 4548         	cmp	r0, r9
 800380a: bf98         	it	ls
 800380c: 4648         	movls	r0, r9
 800380e: 2900         	cmp	r1, #0x0
 8003810: bf48         	it	mi
 8003812: 4249         	rsbmi	r1, r1, #0
 8003814: 4281         	cmp	r1, r0
 8003816: bf98         	it	ls
 8003818: 4601         	movls	r1, r0
 800381a: 2a00         	cmp	r2, #0x0
 800381c: bf48         	it	mi
 800381e: 4252         	rsbmi	r2, r2, #0
 8003820: 428a         	cmp	r2, r1
 8003822: bf98         	it	ls
 8003824: 460a         	movls	r2, r1
 8003826: 2b00         	cmp	r3, #0x0
 8003828: bf48         	it	mi
 800382a: 425b         	rsbmi	r3, r3, #0
 800382c: 4293         	cmp	r3, r2
 800382e: bf98         	it	ls
 8003830: 4613         	movls	r3, r2
 8003832: 2d00         	cmp	r5, #0x0
 8003834: bf48         	it	mi
 8003836: 426d         	rsbmi	r5, r5, #0
 8003838: 429d         	cmp	r5, r3
 800383a: bf98         	it	ls
 800383c: 461d         	movls	r5, r3
 800383e: f1bc 0f00    	cmp.w	r12, #0x0
 8003842: bf48         	it	mi
 8003844: f1cc 0c00    	rsbmi.w	r12, r12, #0x0
 8003848: 45ac         	cmp	r12, r5
 800384a: bf98         	it	ls
 800384c: 46ac         	movls	r12, r5
 800384e: f1bb 0f00    	cmp.w	r11, #0x0
 8003852: bf48         	it	mi
 8003854: f1cb 0b00    	rsbmi.w	r11, r11, #0x0
 8003858: 45e3         	cmp	r11, r12
 800385a: bf98         	it	ls
 800385c: 46e3         	movls	r11, r12
 800385e: 2e00         	cmp	r6, #0x0
 8003860: bf48         	it	mi
 8003862: 4276         	rsbmi	r6, r6, #0
 8003864: f10a 0ac0    	add.w	r10, r10, #0xc0
 8003868: 455e         	cmp	r6, r11
 800386a: bf98         	it	ls
 800386c: 465e         	movls	r6, r11
 800386e: 46b1         	mov	r9, r6
 8003870: f5ba 4fc0    	cmp.w	r10, #0x6000
 8003874: d18e         	bne	0x8003794 <raylar_crcl_encoder_service::benchmark::peak::he0ae015b97d7ff8e+0x236> @ imm = #-0xe4
 8003876: 9606         	str	r6, [sp, #0x18]
 8003878: a806         	add	r0, sp, #0x18
 800387a: f246 00e0    	movw	r0, #0x60e0
 800387e: f44f 6a80    	mov.w	r10, #0x400
 8003882: 9906         	ldr	r1, [sp, #0x18]
 8003884: f46f 0b00    	mvn	r11, #0x800000
 8003888: 9101         	str	r1, [sp, #0x4]
 800388a: f06f 7380    	mvn	r3, #0x1000000
 800388e: 9904         	ldr	r1, [sp, #0x10]
 8003890: 4401         	add	r1, r0
 8003892: 2000         	movs	r0, #0x0
 8003894: f851 4ca8    	ldr	r4, [r1, #-168]
 8003898: eb04 020b    	add.w	r2, r4, r11
 800389c: 429a         	cmp	r2, r3
 800389e: f0c0 80ff    	blo.w	0x8003aa0 <raylar_crcl_encoder_service::benchmark::peak::he0ae015b97d7ff8e+0x542> @ imm = #0x1fe
 80038a2: f851 5c90    	ldr	r5, [r1, #-144]
 80038a6: eb05 020b    	add.w	r2, r5, r11
 80038aa: 429a         	cmp	r2, r3
 80038ac: bf22         	ittt	hs
 80038ae: f851 2c78    	ldrhs	r2, [r1, #-120]
 80038b2: eb02 060b    	addhs.w	r6, r2, r11
 80038b6: 429e         	cmphs	r6, r3
 80038b8: f0c0 80f2    	blo.w	0x8003aa0 <raylar_crcl_encoder_service::benchmark::peak::he0ae015b97d7ff8e+0x542> @ imm = #0x1e4
 80038bc: f851 cc60    	ldr	r12, [r1, #-96]
 80038c0: eb0c 060b    	add.w	r6, r12, r11
 80038c4: 429e         	cmp	r6, r3
 80038c6: bf22         	ittt	hs
 80038c8: f851 ec48    	ldrhs	lr, [r1, #-72]
 80038cc: eb0e 060b    	addhs.w	r6, lr, r11
 80038d0: 429e         	cmphs	r6, r3
 80038d2: f0c0 80e5    	blo.w	0x8003aa0 <raylar_crcl_encoder_service::benchmark::peak::he0ae015b97d7ff8e+0x542> @ imm = #0x1ca
 80038d6: f851 8c30    	ldr	r8, [r1, #-48]
 80038da: eb08 060b    	add.w	r6, r8, r11
 80038de: 429e         	cmp	r6, r3
 80038e0: bf22         	ittt	hs
 80038e2: f851 9c18    	ldrhs	r9, [r1, #-24]
 80038e6: eb09 060b    	addhs.w	r6, r9, r11
 80038ea: 429e         	cmphs	r6, r3
 80038ec: f0c0 80d8    	blo.w	0x8003aa0 <raylar_crcl_encoder_service::benchmark::peak::he0ae015b97d7ff8e+0x542> @ imm = #0x1b0
 80038f0: 680e         	ldr	r6, [r1]
 80038f2: f46f 0b00    	mvn	r11, #0x800000
 80038f6: f8cd a014    	str.w	r10, [sp, #0x14]
 80038fa: 4682         	mov	r10, r0
 80038fc: eb06 000b    	add.w	r0, r6, r11
 8003900: 4298         	cmp	r0, r3
 8003902: 4650         	mov	r0, r10
 8003904: f8dd a014    	ldr.w	r10, [sp, #0x14]
 8003908: f0c0 80ca    	blo.w	0x8003aa0 <raylar_crcl_encoder_service::benchmark::peak::he0ae015b97d7ff8e+0x542> @ imm = #0x194
 800390c: 2c00         	cmp	r4, #0x0
 800390e: bf48         	it	mi
 8003910: 4264         	rsbmi	r4, r4, #0
 8003912: 4284         	cmp	r4, r0
 8003914: bf98         	it	ls
 8003916: 4604         	movls	r4, r0
 8003918: 2d00         	cmp	r5, #0x0
 800391a: bf48         	it	mi
 800391c: 426d         	rsbmi	r5, r5, #0
 800391e: 42a5         	cmp	r5, r4
 8003920: bf98         	it	ls
 8003922: 4625         	movls	r5, r4
 8003924: 2a00         	cmp	r2, #0x0
 8003926: bf48         	it	mi
 8003928: 4252         	rsbmi	r2, r2, #0
 800392a: 42aa         	cmp	r2, r5
 800392c: bf98         	it	ls
 800392e: 462a         	movls	r2, r5
 8003930: f1bc 0f00    	cmp.w	r12, #0x0
 8003934: bf48         	it	mi
 8003936: f1cc 0c00    	rsbmi.w	r12, r12, #0x0
 800393a: 4594         	cmp	r12, r2
 800393c: bf98         	it	ls
 800393e: 4694         	movls	r12, r2
 8003940: f1be 0f00    	cmp.w	lr, #0x0
 8003944: bf48         	it	mi
 8003946: f1ce 0e00    	rsbmi.w	lr, lr, #0x0
 800394a: 45e6         	cmp	lr, r12
 800394c: bf98         	it	ls
 800394e: 46e6         	movls	lr, r12
 8003950: f1b8 0f00    	cmp.w	r8, #0x0
 8003954: bf48         	it	mi
 8003956: f1c8 0800    	rsbmi.w	r8, r8, #0x0
 800395a: 45f0         	cmp	r8, lr
 800395c: bf98         	it	ls
 800395e: 46f0         	movls	r8, lr
 8003960: f1b9 0f00    	cmp.w	r9, #0x0
 8003964: bf48         	it	mi
 8003966: f1c9 0900    	rsbmi.w	r9, r9, #0x0
 800396a: 45c1         	cmp	r9, r8
 800396c: bf98         	it	ls
 800396e: 46c1         	movls	r9, r8
 8003970: 2e00         	cmp	r6, #0x0
 8003972: bf48         	it	mi
 8003974: 4276         	rsbmi	r6, r6, #0
 8003976: 454e         	cmp	r6, r9
 8003978: bf98         	it	ls
 800397a: 464e         	movls	r6, r9
 800397c: 31c0         	adds	r1, #0xc0
 800397e: f1ba 0a08    	subs.w	r10, r10, #0x8
 8003982: 4630         	mov	r0, r6
 8003984: d186         	bne	0x8003894 <raylar_crcl_encoder_service::benchmark::peak::he0ae015b97d7ff8e+0x336> @ imm = #-0xf4
 8003986: 9606         	str	r6, [sp, #0x18]
 8003988: a806         	add	r0, sp, #0x18
 800398a: f246 009c    	movw	r0, #0x609c
 800398e: f04f 0a00    	mov.w	r10, #0x0
 8003992: 9906         	ldr	r1, [sp, #0x18]
 8003994: f46f 0800    	mvn	r8, #0x800000
 8003998: 9100         	str	r1, [sp]
 800399a: f06f 7980    	mvn	r9, #0x1000000
 800399e: 9904         	ldr	r1, [sp, #0x10]
 80039a0: f04f 0b00    	mov.w	r11, #0x0
 80039a4: 4408         	add	r0, r1
 80039a6: 9005         	str	r0, [sp, #0x14]
 80039a8: 9805         	ldr	r0, [sp, #0x14]
 80039aa: eb00 030b    	add.w	r3, r0, r11
 80039ae: f853 1c60    	ldr	r1, [r3, #-96]
 80039b2: eb01 0008    	add.w	r0, r1, r8
 80039b6: 4548         	cmp	r0, r9
 80039b8: bf22         	ittt	hs
 80039ba: f853 0c48    	ldrhs	r0, [r3, #-72]
 80039be: eb00 0208    	addhs.w	r2, r0, r8
 80039c2: 454a         	cmphs	r2, r9
 80039c4: d36c         	blo	0x8003aa0 <raylar_crcl_encoder_service::benchmark::peak::he0ae015b97d7ff8e+0x542> @ imm = #0xd8
 80039c6: f853 2c30    	ldr	r2, [r3, #-48]
 80039ca: eb02 0608    	add.w	r6, r2, r8
 80039ce: 454e         	cmp	r6, r9
 80039d0: bf22         	ittt	hs
 80039d2: f853 4c18    	ldrhs	r4, [r3, #-24]
 80039d6: eb04 0608    	addhs.w	r6, r4, r8
 80039da: 454e         	cmphs	r6, r9
 80039dc: d360         	blo	0x8003aa0 <raylar_crcl_encoder_service::benchmark::peak::he0ae015b97d7ff8e+0x542> @ imm = #0xc0
 80039de: 9e05         	ldr	r6, [sp, #0x14]
 80039e0: f856 600b    	ldr.w	r6, [r6, r11]
 80039e4: eb06 0508    	add.w	r5, r6, r8
 80039e8: 454d         	cmp	r5, r9
 80039ea: bf22         	ittt	hs
 80039ec: f8d3 e018    	ldrhs.w	lr, [r3, #0x18]
 80039f0: eb0e 0508    	addhs.w	r5, lr, r8
 80039f4: 454d         	cmphs	r5, r9
 80039f6: d353         	blo	0x8003aa0 <raylar_crcl_encoder_service::benchmark::peak::he0ae015b97d7ff8e+0x542> @ imm = #0xa6
 80039f8: f8d3 c030    	ldr.w	r12, [r3, #0x30]
 80039fc: eb0c 0508    	add.w	r5, r12, r8
 8003a00: 454d         	cmp	r5, r9
 8003a02: bf22         	ittt	hs
 8003a04: 6c9b         	ldrhs	r3, [r3, #0x48]
 8003a06: eb03 0508    	addhs.w	r5, r3, r8
 8003a0a: 454d         	cmphs	r5, r9
 8003a0c: d348         	blo	0x8003aa0 <raylar_crcl_encoder_service::benchmark::peak::he0ae015b97d7ff8e+0x542> @ imm = #0x90
 8003a0e: 2900         	cmp	r1, #0x0
 8003a10: bf48         	it	mi
 8003a12: 4249         	rsbmi	r1, r1, #0
 8003a14: 4551         	cmp	r1, r10
 8003a16: bf98         	it	ls
 8003a18: 4651         	movls	r1, r10
 8003a1a: 2800         	cmp	r0, #0x0
 8003a1c: bf48         	it	mi
 8003a1e: 4240         	rsbmi	r0, r0, #0
 8003a20: 4288         	cmp	r0, r1
 8003a22: bf98         	it	ls
 8003a24: 4608         	movls	r0, r1
 8003a26: 2a00         	cmp	r2, #0x0
 8003a28: bf48         	it	mi
 8003a2a: 4252         	rsbmi	r2, r2, #0
 8003a2c: 4282         	cmp	r2, r0
 8003a2e: bf98         	it	ls
 8003a30: 4602         	movls	r2, r0
 8003a32: 2c00         	cmp	r4, #0x0
 8003a34: bf48         	it	mi
 8003a36: 4264         	rsbmi	r4, r4, #0
 8003a38: 4294         	cmp	r4, r2
 8003a3a: bf98         	it	ls
 8003a3c: 4614         	movls	r4, r2
 8003a3e: 2e00         	cmp	r6, #0x0
 8003a40: bf48         	it	mi
 8003a42: 4276         	rsbmi	r6, r6, #0
 8003a44: 42a6         	cmp	r6, r4
 8003a46: bf98         	it	ls
 8003a48: 4626         	movls	r6, r4
 8003a4a: f1be 0f00    	cmp.w	lr, #0x0
 8003a4e: bf48         	it	mi
 8003a50: f1ce 0e00    	rsbmi.w	lr, lr, #0x0
 8003a54: 45b6         	cmp	lr, r6
 8003a56: bf98         	it	ls
 8003a58: 46b6         	movls	lr, r6
 8003a5a: f1bc 0f00    	cmp.w	r12, #0x0
 8003a5e: bf48         	it	mi
 8003a60: f1cc 0c00    	rsbmi.w	r12, r12, #0x0
 8003a64: 45f4         	cmp	r12, lr
 8003a66: bf98         	it	ls
 8003a68: 46f4         	movls	r12, lr
 8003a6a: 2b00         	cmp	r3, #0x0
 8003a6c: bf48         	it	mi
 8003a6e: 425b         	rsbmi	r3, r3, #0
 8003a70: f10b 0bc0    	add.w	r11, r11, #0xc0
 8003a74: 4563         	cmp	r3, r12
 8003a76: bf98         	it	ls
 8003a78: 4663         	movls	r3, r12
 8003a7a: 469a         	mov	r10, r3
 8003a7c: f5bb 4fc0    	cmp.w	r11, #0x6000
 8003a80: d192         	bne	0x80039a8 <raylar_crcl_encoder_service::benchmark::peak::he0ae015b97d7ff8e+0x44a> @ imm = #-0xdc
 8003a82: 9306         	str	r3, [sp, #0x18]
 8003a84: a806         	add	r0, sp, #0x18
 8003a86: e9dd 1002    	ldrd	r1, r0, [sp, #8]
 8003a8a: 9a00         	ldr	r2, [sp]
 8003a8c: 4048         	eors	r0, r1
 8003a8e: 9901         	ldr	r1, [sp, #0x4]
 8003a90: 4048         	eors	r0, r1
 8003a92: 9906         	ldr	r1, [sp, #0x18]
 8003a94: 4050         	eors	r0, r2
 8003a96: 4048         	eors	r0, r1
 8003a98: b007         	add	sp, #0x1c
 8003a9a: e8bd 0f00    	pop.w	{r8, r9, r10, r11}
 8003a9e: bdf0         	pop	{r4, r5, r6, r7, pc}
 8003aa0: f64d 50c8    	movw	r0, #0xddc8
 8003aa4: f64d 52d0    	movw	r2, #0xddd0
 8003aa8: f6c0 0000    	movt	r0, #0x800
 8003aac: f6c0 0200    	movt	r2, #0x800
 8003ab0: 210b         	movs	r1, #0xb
 8003ab2: f007 fe37    	bl	0x800b724 <core::panicking::panic_fmt> @ imm = #0x7c6e

08003ab6 <raylar_crcl_encoder_service::benchmark::codec::h6d81b9c8b86ce9a2>:
 8003ab6: b5f0         	push	{r4, r5, r6, r7, lr}
 8003ab8: af03         	add	r7, sp, #0xc
 8003aba: e92d 0b00    	push.w	{r8, r9, r11}
 8003abe: b0d6         	sub	sp, #0x158
 8003ac0: f240 690a    	movw	r9, #0x60a
 8003ac4: 4680         	mov	r8, r0
 8003ac6: f44f 0000    	mov.w	r0, #0x800000
 8003aca: f2c0 1901    	movt	r9, #0x101
 8003ace: 900b         	str	r0, [sp, #0x2c]
 8003ad0: f44f 6080    	mov.w	r0, #0x400
 8003ad4: f8cd 9030    	str.w	r9, [sp, #0x30]
 8003ad8: ad0a         	add	r5, sp, #0x28
 8003ada: 900a         	str	r0, [sp, #0x28]
 8003adc: 980a         	ldr	r0, [sp, #0x28]
 8003ade: 1e41         	subs	r1, r0, #0x1
 8003ae0: ea80 0201    	eor.w	r2, r0, r1
 8003ae4: 428a         	cmp	r2, r1
 8003ae6: d916         	bls	0x8003b16 <raylar_crcl_encoder_service::benchmark::codec::h6d81b9c8b86ce9a2+0x60> @ imm = #0x2c
 8003ae8: fa90 f0a0    	rbit	r0, r0
 8003aec: fab0 f080    	clz	r0, r0
 8003af0: 3808         	subs	r0, #0x8
 8003af2: 2804         	cmp	r0, #0x4
 8003af4: bf9c         	itt	ls
 8003af6: f89d 0030    	ldrbls.w	r0, [sp, #0x30]
 8003afa: 280c         	cmpls	r0, #0xc
 8003afc: d80b         	bhi	0x8003b16 <raylar_crcl_encoder_service::benchmark::codec::h6d81b9c8b86ce9a2+0x60> @ imm = #0x16
 8003afe: 2101         	movs	r1, #0x1
 8003b00: fa01 f000    	lsl.w	r0, r1, r0
 8003b04: f410 5faa    	tst.w	r0, #0x1540
 8003b08: bf1e         	ittt	ne
 8003b0a: 980b         	ldrne	r0, [sp, #0x2c]
 8003b0c: f100 4040    	addne.w	r0, r0, #0xc0000000
 8003b10: f1b0 4f40    	cmpne.w	r0, #0xc0000000
 8003b14: d813         	bhi	0x8003b3e <raylar_crcl_encoder_service::benchmark::codec::h6d81b9c8b86ce9a2+0x88> @ imm = #0x26
 8003b16: 2000         	movs	r0, #0x0
 8003b18: f88d 00c4    	strb.w	r0, [sp, #0xc4]
 8003b1c: f64d 6050    	movw	r0, #0xde50
 8003b20: f6c0 0000    	movt	r0, #0x800
 8003b24: f64d 434c    	movw	r3, #0xdc4c
 8003b28: 9000         	str	r0, [sp]
 8003b2a: f64d 30b0    	movw	r0, #0xdbb0
 8003b2e: aa31         	add	r2, sp, #0xc4
 8003b30: f6c0 0000    	movt	r0, #0x800
 8003b34: f6c0 0300    	movt	r3, #0x800
 8003b38: 212b         	movs	r1, #0x2b
 8003b3a: f007 fd73    	bl	0x800b624 <core::result::unwrap_failed> @ imm = #0x7ae6
 8003b3e: f89d 0031    	ldrb.w	r0, [sp, #0x31]
 8003b42: 281e         	cmp	r0, #0x1e
 8003b44: d8e7         	bhi	0x8003b16 <raylar_crcl_encoder_service::benchmark::codec::h6d81b9c8b86ce9a2+0x60> @ imm = #-0x32
 8003b46: f89d 4033    	ldrb.w	r4, [sp, #0x33]
 8003b4a: 2c01         	cmp	r4, #0x1
 8003b4c: d104         	bne	0x8003b58 <raylar_crcl_encoder_service::benchmark::codec::h6d81b9c8b86ce9a2+0xa2> @ imm = #0x8
 8003b4e: f89d 0032    	ldrb.w	r0, [sp, #0x32]
 8003b52: 2801         	cmp	r0, #0x1
 8003b54: d002         	beq	0x8003b5c <raylar_crcl_encoder_service::benchmark::codec::h6d81b9c8b86ce9a2+0xa6> @ imm = #0x4
 8003b56: e7de         	b	0x8003b16 <raylar_crcl_encoder_service::benchmark::codec::h6d81b9c8b86ce9a2+0x60> @ imm = #-0x44
 8003b58: 2800         	cmp	r0, #0x0
 8003b5a: d1dc         	bne	0x8003b16 <raylar_crcl_encoder_service::benchmark::codec::h6d81b9c8b86ce9a2+0x60> @ imm = #-0x48
 8003b5c: e9dd 010a    	ldrd	r0, r1, [sp, #40]
 8003b60: 2600         	movs	r6, #0x0
 8003b62: f8dd 202f    	ldr.w	r2, [sp, #0x2f]
 8003b66: f8cd 00d3    	str.w	r0, [sp, #0xd3]
 8003b6a: a802         	add	r0, sp, #0x8
 8003b6c: f8cd 20da    	str.w	r2, [sp, #0xda]
 8003b70: 3001         	adds	r0, #0x1
 8003b72: f8cd 10d7    	str.w	r1, [sp, #0xd7]
 8003b76: a931         	add	r1, sp, #0xc4
 8003b78: 221a         	movs	r2, #0x1a
 8003b7a: 9633         	str	r6, [sp, #0xcc]
 8003b7c: e9cd 6631    	strd	r6, r6, [sp, #196]
 8003b80: f8cd 60cf    	str.w	r6, [sp, #0xcf]
 8003b84: f88d 6008    	strb.w	r6, [sp, #0x8]
 8003b88: f008 fd88    	bl	0x800c69c <__aeabi_memcpy> @ imm = #0x8b10
 8003b8c: f44f 0000    	mov.w	r0, #0x800000
 8003b90: f88d 4023    	strb.w	r4, [sp, #0x23]
 8003b94: 9032         	str	r0, [sp, #0xc8]
 8003b96: f44f 6080    	mov.w	r0, #0x400
 8003b9a: 9031         	str	r0, [sp, #0xc4]
 8003b9c: a831         	add	r0, sp, #0xc4
 8003b9e: f8cd 90cc    	str.w	r9, [sp, #0xcc]
 8003ba2: 9831         	ldr	r0, [sp, #0xc4]
 8003ba4: 1e41         	subs	r1, r0, #0x1
 8003ba6: ea80 0201    	eor.w	r2, r0, r1
 8003baa: 428a         	cmp	r2, r1
 8003bac: d90a         	bls	0x8003bc4 <raylar_crcl_encoder_service::benchmark::codec::h6d81b9c8b86ce9a2+0x10e> @ imm = #0x14
 8003bae: fa90 f1a0    	rbit	r1, r0
 8003bb2: fab1 f181    	clz	r1, r1
 8003bb6: 3908         	subs	r1, #0x8
 8003bb8: 2904         	cmp	r1, #0x4
 8003bba: bf9c         	itt	ls
 8003bbc: f89d 10cc    	ldrbls.w	r1, [sp, #0xcc]
 8003bc0: 290c         	cmpls	r1, #0xc
 8003bc2: d906         	bls	0x8003bd2 <raylar_crcl_encoder_service::benchmark::codec::h6d81b9c8b86ce9a2+0x11c> @ imm = #0xc
 8003bc4: f64d 6040    	movw	r0, #0xde40
 8003bc8: f88d 60c4    	strb.w	r6, [sp, #0xc4]
 8003bcc: f6c0 0000    	movt	r0, #0x800
 8003bd0: e7a8         	b	0x8003b24 <raylar_crcl_encoder_service::benchmark::codec::h6d81b9c8b86ce9a2+0x6e> @ imm = #-0xb0
 8003bd2: 2201         	movs	r2, #0x1
 8003bd4: fa02 f101    	lsl.w	r1, r2, r1
 8003bd8: f411 5faa    	tst.w	r1, #0x1540
 8003bdc: d0f2         	beq	0x8003bc4 <raylar_crcl_encoder_service::benchmark::codec::h6d81b9c8b86ce9a2+0x10e> @ imm = #-0x1c
 8003bde: 9a32         	ldr	r2, [sp, #0xc8]
 8003be0: 2101         	movs	r1, #0x1
 8003be2: f2cc 0100    	movt	r1, #0xc000
 8003be6: f102 4240    	add.w	r2, r2, #0xc0000000
 8003bea: 428a         	cmp	r2, r1
 8003bec: d3ea         	blo	0x8003bc4 <raylar_crcl_encoder_service::benchmark::codec::h6d81b9c8b86ce9a2+0x10e> @ imm = #-0x2c
 8003bee: f89d 10cd    	ldrb.w	r1, [sp, #0xcd]
 8003bf2: 291e         	cmp	r1, #0x1e
 8003bf4: d8e6         	bhi	0x8003bc4 <raylar_crcl_encoder_service::benchmark::codec::h6d81b9c8b86ce9a2+0x10e> @ imm = #-0x34
 8003bf6: f89d 20cf    	ldrb.w	r2, [sp, #0xcf]
 8003bfa: 2a01         	cmp	r2, #0x1
 8003bfc: d107         	bne	0x8003c0e <raylar_crcl_encoder_service::benchmark::codec::h6d81b9c8b86ce9a2+0x158> @ imm = #0xe
 8003bfe: f5b0 6f80    	cmp.w	r0, #0x400
 8003c02: d3df         	blo	0x8003bc4 <raylar_crcl_encoder_service::benchmark::codec::h6d81b9c8b86ce9a2+0x10e> @ imm = #-0x42
 8003c04: f89d 00ce    	ldrb.w	r0, [sp, #0xce]
 8003c08: 2801         	cmp	r0, #0x1
 8003c0a: d005         	beq	0x8003c18 <raylar_crcl_encoder_service::benchmark::codec::h6d81b9c8b86ce9a2+0x162> @ imm = #0xa
 8003c0c: e7da         	b	0x8003bc4 <raylar_crcl_encoder_service::benchmark::codec::h6d81b9c8b86ce9a2+0x10e> @ imm = #-0x4c
 8003c0e: f5b0 6f80    	cmp.w	r0, #0x400
 8003c12: d3d7         	blo	0x8003bc4 <raylar_crcl_encoder_service::benchmark::codec::h6d81b9c8b86ce9a2+0x10e> @ imm = #-0x52
 8003c14: 2900         	cmp	r1, #0x0
 8003c16: d1d5         	bne	0x8003bc4 <raylar_crcl_encoder_service::benchmark::codec::h6d81b9c8b86ce9a2+0x10e> @ imm = #-0x56
 8003c18: f246 0028    	movw	r0, #0x6028
 8003c1c: eb08 0400    	add.w	r4, r8, r0
 8003c20: f105 0048    	add.w	r0, r5, #0x48
 8003c24: 2128         	movs	r1, #0x28
 8003c26: f008 ff62    	bl	0x800caee <__aeabi_memclr4> @ imm = #0x8ec4
 8003c2a: ab31         	add	r3, sp, #0xc4
 8003c2c: ae31         	add	r6, sp, #0xc4
 8003c2e: 2000         	movs	r0, #0x0
 8003c30: 941a         	str	r4, [sp, #0x68]
 8003c32: cb0e         	ldm	r3, {r1, r2, r3}
 8003c34: f44f 7480    	mov.w	r4, #0x100
 8003c38: 932d         	str	r3, [sp, #0xb4]
 8003c3a: f44f 53c0    	mov.w	r3, #0x1800
 8003c3e: e9cd 122b    	strd	r1, r2, [sp, #172]
 8003c42: 2101         	movs	r1, #0x1
 8003c44: f8ad 0044    	strh.w	r0, [sp, #0x44]
 8003c48: e9cd 000f    	strd	r0, r0, [sp, #60]
 8003c4c: e9cd 000d    	strd	r0, r0, [sp, #52]
 8003c50: e9cd 000b    	strd	r0, r0, [sp, #44]
 8003c54: e9cd 0013    	strd	r0, r0, [sp, #76]
 8003c58: e9cd 0015    	strd	r0, r0, [sp, #84]
 8003c5c: e9cd 0017    	strd	r0, r0, [sp, #92]
 8003c60: f8ad 0064    	strh.w	r0, [sp, #0x64]
 8003c64: e9cd 0027    	strd	r0, r0, [sp, #156]
 8003c68: e9cd 0029    	strd	r0, r0, [sp, #164]
 8003c6c: 900a         	str	r0, [sp, #0x28]
 8003c6e: 9012         	str	r0, [sp, #0x48]
 8003c70: 931b         	str	r3, [sp, #0x6c]
 8003c72: 9126         	str	r1, [sp, #0x98]
 8003c74: f8ad 00b8    	strh.w	r0, [sp, #0xb8]
 8003c78: 9431         	str	r4, [sp, #0xc4]
 8003c7a: 4628         	mov	r0, r5
 8003c7c: 9931         	ldr	r1, [sp, #0xc4]
 8003c7e: f7fd fdee    	bl	0x800185e <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3> @ imm = #-0x2424
 8003c82: 07c0         	lsls	r0, r0, #0x1f
 8003c84: d123         	bne	0x8003cce <raylar_crcl_encoder_service::benchmark::codec::h6d81b9c8b86ce9a2+0x218> @ imm = #0x46
 8003c86: 07c8         	lsls	r0, r1, #0x1f
 8003c88: d0f6         	beq	0x8003c78 <raylar_crcl_encoder_service::benchmark::codec::h6d81b9c8b86ce9a2+0x1c2> @ imm = #-0x14
 8003c8a: ad31         	add	r5, sp, #0xc4
 8003c8c: a90a         	add	r1, sp, #0x28
 8003c8e: 2294         	movs	r2, #0x94
 8003c90: 4628         	mov	r0, r5
 8003c92: f008 fede    	bl	0x800ca52 <__aeabi_memcpy4> @ imm = #0x8dbc
 8003c96: f641 6058    	movw	r0, #0x1e58
 8003c9a: f508 34e0    	add.w	r4, r8, #0x1c000
 8003c9e: 9000         	str	r0, [sp]
 8003ca0: a82f         	add	r0, sp, #0xbc
 8003ca2: a902         	add	r1, sp, #0x8
 8003ca4: 462a         	mov	r2, r5
 8003ca6: 4623         	mov	r3, r4
 8003ca8: f7fd fb22    	bl	0x80012f0 <raylar_crcl_encoder_service::format::Encoder::data::ha65fe528befbf4e3> @ imm = #-0x29bc
 8003cac: f89d 00bc    	ldrb.w	r0, [sp, #0xbc]
 8003cb0: 2801         	cmp	r0, #0x1
 8003cb2: d01c         	beq	0x8003cee <raylar_crcl_encoder_service::benchmark::codec::h6d81b9c8b86ce9a2+0x238> @ imm = #0x38
 8003cb4: 9930         	ldr	r1, [sp, #0xc0]
 8003cb6: f641 6059    	movw	r0, #0x1e59
 8003cba: 4281         	cmp	r1, r0
 8003cbc: d20e         	bhs	0x8003cdc <raylar_crcl_encoder_service::benchmark::codec::h6d81b9c8b86ce9a2+0x226> @ imm = #0x1c
 8003cbe: e9cd 4131    	strd	r4, r1, [sp, #196]
 8003cc2: a831         	add	r0, sp, #0xc4
 8003cc4: 4608         	mov	r0, r1
 8003cc6: b056         	add	sp, #0x158
 8003cc8: e8bd 0b00    	pop.w	{r8, r9, r11}
 8003ccc: bdf0         	pop	{r4, r5, r6, r7, pc}
 8003cce: f64d 6030    	movw	r0, #0xde30
 8003cd2: f88d 10c4    	strb.w	r1, [sp, #0xc4]
 8003cd6: f6c0 0000    	movt	r0, #0x800
 8003cda: e723         	b	0x8003b24 <raylar_crcl_encoder_service::benchmark::codec::h6d81b9c8b86ce9a2+0x6e> @ imm = #-0x1ba
 8003cdc: f64d 6310    	movw	r3, #0xde10
 8003ce0: 2000         	movs	r0, #0x0
 8003ce2: f6c0 0300    	movt	r3, #0x800
 8003ce6: f641 6258    	movw	r2, #0x1e58
 8003cea: f007 ffa6    	bl	0x800bc3a <core::slice::index::slice_index_fail> @ imm = #0x7f4c
 8003cee: f89d 00bd    	ldrb.w	r0, [sp, #0xbd]
 8003cf2: f88d 00c4    	strb.w	r0, [sp, #0xc4]
 8003cf6: f64d 6020    	movw	r0, #0xde20
 8003cfa: f6c0 0000    	movt	r0, #0x800
 8003cfe: e711         	b	0x8003b24 <raylar_crcl_encoder_service::benchmark::codec::h6d81b9c8b86ce9a2+0x6e> @ imm = #-0x1de

08003d00 <raylar_crcl_encoder_service::benchmark::empty::h49addc3cc3cef455>:
 8003d00: b580         	push	{r7, lr}
 8003d02: 466f         	mov	r7, sp
 8003d04: b081         	sub	sp, #0x4
 8003d06: 2000         	movs	r0, #0x0
 8003d08: 9000         	str	r0, [sp]
 8003d0a: 4668         	mov	r0, sp
 8003d0c: f85d 0b04    	ldr	r0, [sp], #4
 8003d10: bd80         	pop	{r7, pc}

08003d12 <raylar_crcl_encoder_service::benchmark::fused::h66edd02a0d627444>:
 8003d12: b5f0         	push	{r4, r5, r6, r7, lr}
 8003d14: af03         	add	r7, sp, #0xc
 8003d16: e92d 0f00    	push.w	{r8, r9, r10, r11}
 8003d1a: b08a         	sub	sp, #0x28
 8003d1c: 6801         	ldr	r1, [r0]
 8003d1e: f246 052c    	movw	r5, #0x602c
 8003d22: 9108         	str	r1, [sp, #0x20]
 8003d24: a908         	add	r1, sp, #0x20
 8003d26: 2601         	movs	r6, #0x1
 8003d28: 9005         	str	r0, [sp, #0x14]
 8003d2a: 9908         	ldr	r1, [sp, #0x20]
 8003d2c: f04f 0e00    	mov.w	lr, #0x0
 8003d30: 5943         	ldr	r3, [r0, r5]
 8003d32: 1e4a         	subs	r2, r1, #0x1
 8003d34: f002 021f    	and	r2, r2, #0x1f
 8003d38: fa06 fc02    	lsl.w	r12, r6, r2
 8003d3c: eb03 060c    	add.w	r6, r3, r12
 8003d40: f001 031f    	and	r3, r1, #0x1f
 8003d44: 1941         	adds	r1, r0, r5
 8003d46: 9104         	str	r1, [sp, #0x10]
 8003d48: fa46 fa03    	asr.w	r10, r6, r3
 8003d4c: f840 a005    	str.w	r10, [r0, r5]
 8003d50: f240 30ff    	movw	r0, #0x3ff
 8003d54: 6b0d         	ldr	r5, [r1, #0x30]
 8003d56: 6c8c         	ldr	r4, [r1, #0x48]
 8003d58: 4465         	add	r5, r12
 8003d5a: 9006         	str	r0, [sp, #0x18]
 8003d5c: 4464         	add	r4, r12
 8003d5e: f8d1 9018    	ldr.w	r9, [r1, #0x18]
 8003d62: 411d         	asrs	r5, r3
 8003d64: f8d1 8060    	ldr.w	r8, [r1, #0x60]
 8003d68: 411c         	asrs	r4, r3
 8003d6a: 1b60         	subs	r0, r4, r5
 8003d6c: 630d         	str	r5, [r1, #0x30]
 8003d6e: 0046         	lsls	r6, r0, #0x1
 8003d70: ea86 7be0    	eor.w	r11, r6, r0, asr #31
 8003d74: eb09 060c    	add.w	r6, r9, r12
 8003d78: 648c         	str	r4, [r1, #0x48]
 8003d7a: f8d1 2108    	ldr.w	r2, [r1, #0x108]
 8003d7e: 411e         	asrs	r6, r3
 8003d80: 1bad         	subs	r5, r5, r6
 8003d82: 618e         	str	r6, [r1, #0x18]
 8003d84: 4462         	add	r2, r12
 8003d86: 0068         	lsls	r0, r5, #0x1
 8003d88: ea80 70e5    	eor.w	r0, r0, r5, asr #31
 8003d8c: eba6 050a    	sub.w	r5, r6, r10
 8003d90: fa42 fa03    	asr.w	r10, r2, r3
 8003d94: 006e         	lsls	r6, r5, #0x1
 8003d96: ea86 75e5    	eor.w	r5, r6, r5, asr #31
 8003d9a: eb0e 0605    	add.w	r6, lr, r5
 8003d9e: eb08 050c    	add.w	r5, r8, r12
 8003da2: 4430         	add	r0, r6
 8003da4: f8d1 e078    	ldr.w	lr, [r1, #0x78]
 8003da8: 411d         	asrs	r5, r3
 8003daa: 1b2c         	subs	r4, r5, r4
 8003dac: 4458         	add	r0, r11
 8003dae: 660d         	str	r5, [r1, #0x60]
 8003db0: 0066         	lsls	r6, r4, #0x1
 8003db2: ea86 74e4    	eor.w	r4, r6, r4, asr #31
 8003db6: 4420         	add	r0, r4
 8003db8: eb0e 040c    	add.w	r4, lr, r12
 8003dbc: f8d1 8090    	ldr.w	r8, [r1, #0x90]
 8003dc0: 411c         	asrs	r4, r3
 8003dc2: 1b65         	subs	r5, r4, r5
 8003dc4: 678c         	str	r4, [r1, #0x78]
 8003dc6: 006e         	lsls	r6, r5, #0x1
 8003dc8: f8d1 e0a8    	ldr.w	lr, [r1, #0xa8]
 8003dcc: ea86 76e5    	eor.w	r6, r6, r5, asr #31
 8003dd0: f8c1 a108    	str.w	r10, [r1, #0x108]
 8003dd4: 4430         	add	r0, r6
 8003dd6: eb08 060c    	add.w	r6, r8, r12
 8003dda: 411e         	asrs	r6, r3
 8003ddc: 1b34         	subs	r4, r6, r4
 8003dde: f8c1 6090    	str.w	r6, [r1, #0x90]
 8003de2: 0065         	lsls	r5, r4, #0x1
 8003de4: ea85 74e4    	eor.w	r4, r5, r4, asr #31
 8003de8: eb00 0804    	add.w	r8, r0, r4
 8003dec: eb0e 040c    	add.w	r4, lr, r12
 8003df0: f8d1 50f0    	ldr.w	r5, [r1, #0xf0]
 8003df4: fa44 f003    	asr.w	r0, r4, r3
 8003df8: 1b86         	subs	r6, r0, r6
 8003dfa: 4465         	add	r5, r12
 8003dfc: f8c1 00a8    	str.w	r0, [r1, #0xa8]
 8003e00: 0074         	lsls	r4, r6, #0x1
 8003e02: ea84 7ee6    	eor.w	lr, r4, r6, asr #31
 8003e06: f8d1 60c0    	ldr.w	r6, [r1, #0xc0]
 8003e0a: fa45 f403    	asr.w	r4, r5, r3
 8003e0e: f8d1 50d8    	ldr.w	r5, [r1, #0xd8]
 8003e12: 44c6         	add	lr, r8
 8003e14: 4466         	add	r6, r12
 8003e16: f8c1 40f0    	str.w	r4, [r1, #0xf0]
 8003e1a: 4465         	add	r5, r12
 8003e1c: 411e         	asrs	r6, r3
 8003e1e: 1a30         	subs	r0, r6, r0
 8003e20: 411d         	asrs	r5, r3
 8003e22: f8c1 60c0    	str.w	r6, [r1, #0xc0]
 8003e26: 0042         	lsls	r2, r0, #0x1
 8003e28: ea82 70e0    	eor.w	r0, r2, r0, asr #31
 8003e2c: 1baa         	subs	r2, r5, r6
 8003e2e: 4470         	add	r0, lr
 8003e30: f8c1 50d8    	str.w	r5, [r1, #0xd8]
 8003e34: f501 7184    	add.w	r1, r1, #0x108
 8003e38: 0056         	lsls	r6, r2, #0x1
 8003e3a: ea86 72e2    	eor.w	r2, r6, r2, asr #31
 8003e3e: 4410         	add	r0, r2
 8003e40: 1b62         	subs	r2, r4, r5
 8003e42: 0056         	lsls	r6, r2, #0x1
 8003e44: ea86 72e2    	eor.w	r2, r6, r2, asr #31
 8003e48: 4410         	add	r0, r2
 8003e4a: ebaa 0204    	sub.w	r2, r10, r4
 8003e4e: 0056         	lsls	r6, r2, #0x1
 8003e50: ea86 72e2    	eor.w	r2, r6, r2, asr #31
 8003e54: eb00 0e02    	add.w	lr, r0, r2
 8003e58: 9806         	ldr	r0, [sp, #0x18]
 8003e5a: 380b         	subs	r0, #0xb
 8003e5c: f47f af7a    	bne.w	0x8003d54 <raylar_crcl_encoder_service::benchmark::fused::h66edd02a0d627444+0x42> @ imm = #-0x10c
 8003e60: f8cd e024    	str.w	lr, [sp, #0x24]
 8003e64: a909         	add	r1, sp, #0x24
 8003e66: 2501         	movs	r5, #0x1
 8003e68: f04f 0900    	mov.w	r9, #0x0
 8003e6c: 9809         	ldr	r0, [sp, #0x24]
 8003e6e: f240 3eff    	movw	lr, #0x3ff
 8003e72: 9003         	str	r0, [sp, #0xc]
 8003e74: 9805         	ldr	r0, [sp, #0x14]
 8003e76: 6881         	ldr	r1, [r0, #0x8]
 8003e78: 9108         	str	r1, [sp, #0x20]
 8003e7a: a908         	add	r1, sp, #0x20
 8003e7c: 9908         	ldr	r1, [sp, #0x20]
 8003e7e: 9a04         	ldr	r2, [sp, #0x10]
 8003e80: 1e4b         	subs	r3, r1, #0x1
 8003e82: f001 041f    	and	r4, r1, #0x1f
 8003e86: f003 031f    	and	r3, r3, #0x1f
 8003e8a: f852 6f04    	ldr	r6, [r2, #4]!
 8003e8e: fa05 f303    	lsl.w	r3, r5, r3
 8003e92: 441e         	add	r6, r3
 8003e94: fa46 fb04    	asr.w	r11, r6, r4
 8003e98: f8c2 b000    	str.w	r11, [r2]
 8003e9c: 6b15         	ldr	r5, [r2, #0x30]
 8003e9e: f1be 0e0b    	subs.w	lr, lr, #0xb
 8003ea2: 6c96         	ldr	r6, [r2, #0x48]
 8003ea4: f8d2 a018    	ldr.w	r10, [r2, #0x18]
 8003ea8: 441d         	add	r5, r3
 8003eaa: 6e10         	ldr	r0, [r2, #0x60]
 8003eac: 441e         	add	r6, r3
 8003eae: 9006         	str	r0, [sp, #0x18]
 8003eb0: eb0a 0003    	add.w	r0, r10, r3
 8003eb4: fa45 f504    	asr.w	r5, r5, r4
 8003eb8: fa46 f604    	asr.w	r6, r6, r4
 8003ebc: eba6 0c05    	sub.w	r12, r6, r5
 8003ec0: fa40 f004    	asr.w	r0, r0, r4
 8003ec4: 6315         	str	r5, [r2, #0x30]
 8003ec6: eba5 0500    	sub.w	r5, r5, r0
 8003eca: 6190         	str	r0, [r2, #0x18]
 8003ecc: eba0 000b    	sub.w	r0, r0, r11
 8003ed0: ea4f 0145    	lsl.w	r1, r5, #0x1
 8003ed4: 6496         	str	r6, [r2, #0x48]
 8003ed6: ea81 71e5    	eor.w	r1, r1, r5, asr #31
 8003eda: ea4f 0540    	lsl.w	r5, r0, #0x1
 8003ede: ea85 70e0    	eor.w	r0, r5, r0, asr #31
 8003ee2: ea4f 084c    	lsl.w	r8, r12, #0x1
 8003ee6: 4448         	add	r0, r9
 8003ee8: ea88 7cec    	eor.w	r12, r8, r12, asr #31
 8003eec: 4408         	add	r0, r1
 8003eee: 9906         	ldr	r1, [sp, #0x18]
 8003ef0: f8d2 8078    	ldr.w	r8, [r2, #0x78]
 8003ef4: 4460         	add	r0, r12
 8003ef6: eb01 0503    	add.w	r5, r1, r3
 8003efa: f8d2 c090    	ldr.w	r12, [r2, #0x90]
 8003efe: fa45 f504    	asr.w	r5, r5, r4
 8003f02: eba5 0606    	sub.w	r6, r5, r6
 8003f06: 6615         	str	r5, [r2, #0x60]
 8003f08: ea4f 0146    	lsl.w	r1, r6, #0x1
 8003f0c: ea81 71e6    	eor.w	r1, r1, r6, asr #31
 8003f10: 4408         	add	r0, r1
 8003f12: eb08 0103    	add.w	r1, r8, r3
 8003f16: f8d2 80a8    	ldr.w	r8, [r2, #0xa8]
 8003f1a: fa41 f104    	asr.w	r1, r1, r4
 8003f1e: eba1 0505    	sub.w	r5, r1, r5
 8003f22: 6791         	str	r1, [r2, #0x78]
 8003f24: ea4f 0645    	lsl.w	r6, r5, #0x1
 8003f28: ea86 75e5    	eor.w	r5, r6, r5, asr #31
 8003f2c: 4428         	add	r0, r5
 8003f2e: eb0c 0503    	add.w	r5, r12, r3
 8003f32: fa45 f504    	asr.w	r5, r5, r4
 8003f36: eba5 0101    	sub.w	r1, r5, r1
 8003f3a: f8c2 5090    	str.w	r5, [r2, #0x90]
 8003f3e: ea4f 0641    	lsl.w	r6, r1, #0x1
 8003f42: ea86 71e1    	eor.w	r1, r6, r1, asr #31
 8003f46: eb00 0c01    	add.w	r12, r0, r1
 8003f4a: eb08 0103    	add.w	r1, r8, r3
 8003f4e: f8d2 60f0    	ldr.w	r6, [r2, #0xf0]
 8003f52: fa41 f104    	asr.w	r1, r1, r4
 8003f56: eba1 0505    	sub.w	r5, r1, r5
 8003f5a: f8c2 10a8    	str.w	r1, [r2, #0xa8]
 8003f5e: ea4f 0045    	lsl.w	r0, r5, #0x1
 8003f62: ea80 78e5    	eor.w	r8, r0, r5, asr #31
 8003f66: eb06 0503    	add.w	r5, r6, r3
 8003f6a: f8d2 00c0    	ldr.w	r0, [r2, #0xc0]
 8003f6e: f8d2 60d8    	ldr.w	r6, [r2, #0xd8]
 8003f72: 44c4         	add	r12, r8
 8003f74: fa45 f904    	asr.w	r9, r5, r4
 8003f78: 4418         	add	r0, r3
 8003f7a: f8d2 5108    	ldr.w	r5, [r2, #0x108]
 8003f7e: 441e         	add	r6, r3
 8003f80: fa40 f004    	asr.w	r0, r0, r4
 8003f84: eba0 0101    	sub.w	r1, r0, r1
 8003f88: 441d         	add	r5, r3
 8003f8a: fa46 f604    	asr.w	r6, r6, r4
 8003f8e: f8c2 00c0    	str.w	r0, [r2, #0xc0]
 8003f92: eba6 0000    	sub.w	r0, r6, r0
 8003f96: fa45 fb04    	asr.w	r11, r5, r4
 8003f9a: ea4f 0541    	lsl.w	r5, r1, #0x1
 8003f9e: ea85 71e1    	eor.w	r1, r5, r1, asr #31
 8003fa2: ea4f 0540    	lsl.w	r5, r0, #0x1
 8003fa6: 4461         	add	r1, r12
 8003fa8: ea85 70e0    	eor.w	r0, r5, r0, asr #31
 8003fac: 4408         	add	r0, r1
 8003fae: eba9 0106    	sub.w	r1, r9, r6
 8003fb2: f8c2 60d8    	str.w	r6, [r2, #0xd8]
 8003fb6: ea4f 0641    	lsl.w	r6, r1, #0x1
 8003fba: ea86 71e1    	eor.w	r1, r6, r1, asr #31
 8003fbe: 4408         	add	r0, r1
 8003fc0: ebab 0109    	sub.w	r1, r11, r9
 8003fc4: f8c2 90f0    	str.w	r9, [r2, #0xf0]
 8003fc8: ea4f 0641    	lsl.w	r6, r1, #0x1
 8003fcc: ea86 71e1    	eor.w	r1, r6, r1, asr #31
 8003fd0: f8c2 b108    	str.w	r11, [r2, #0x108]
 8003fd4: eb00 0901    	add.w	r9, r0, r1
 8003fd8: f502 7284    	add.w	r2, r2, #0x108
 8003fdc: f47f af5e    	bne.w	0x8003e9c <raylar_crcl_encoder_service::benchmark::fused::h66edd02a0d627444+0x18a> @ imm = #-0x144
 8003fe0: f8cd 9024    	str.w	r9, [sp, #0x24]
 8003fe4: a809         	add	r0, sp, #0x24
 8003fe6: 2601         	movs	r6, #0x1
 8003fe8: f04f 0800    	mov.w	r8, #0x0
 8003fec: 9809         	ldr	r0, [sp, #0x24]
 8003fee: f240 3cff    	movw	r12, #0x3ff
 8003ff2: 9002         	str	r0, [sp, #0x8]
 8003ff4: 9805         	ldr	r0, [sp, #0x14]
 8003ff6: 6900         	ldr	r0, [r0, #0x10]
 8003ff8: 9008         	str	r0, [sp, #0x20]
 8003ffa: a808         	add	r0, sp, #0x20
 8003ffc: 9808         	ldr	r0, [sp, #0x20]
 8003ffe: 9a04         	ldr	r2, [sp, #0x10]
 8004000: 1e43         	subs	r3, r0, #0x1
 8004002: f000 041f    	and	r4, r0, #0x1f
 8004006: f003 031f    	and	r3, r3, #0x1f
 800400a: f852 1f08    	ldr	r1, [r2, #8]!
 800400e: fa06 f303    	lsl.w	r3, r6, r3
 8004012: 4419         	add	r1, r3
 8004014: fa41 fb04    	asr.w	r11, r1, r4
 8004018: f8c2 b000    	str.w	r11, [r2]
 800401c: 6b15         	ldr	r5, [r2, #0x30]
 800401e: f1bc 0c0b    	subs.w	r12, r12, #0xb
 8004022: 6c96         	ldr	r6, [r2, #0x48]
 8004024: 441d         	add	r5, r3
 8004026: f8d2 a018    	ldr.w	r10, [r2, #0x18]
 800402a: 441e         	add	r6, r3
 800402c: f8d2 9060    	ldr.w	r9, [r2, #0x60]
 8004030: fa45 f504    	asr.w	r5, r5, r4
 8004034: fa46 f604    	asr.w	r6, r6, r4
 8004038: eba6 0005    	sub.w	r0, r6, r5
 800403c: 6315         	str	r5, [r2, #0x30]
 800403e: ea4f 0140    	lsl.w	r1, r0, #0x1
 8004042: ea81 7ee0    	eor.w	lr, r1, r0, asr #31
 8004046: eb0a 0103    	add.w	r1, r10, r3
 800404a: 6496         	str	r6, [r2, #0x48]
 800404c: fa41 f104    	asr.w	r1, r1, r4
 8004050: eba5 0501    	sub.w	r5, r5, r1
 8004054: 6191         	str	r1, [r2, #0x18]
 8004056: eba1 010b    	sub.w	r1, r1, r11
 800405a: ea4f 0045    	lsl.w	r0, r5, #0x1
 800405e: ea80 70e5    	eor.w	r0, r0, r5, asr #31
 8004062: ea4f 0541    	lsl.w	r5, r1, #0x1
 8004066: ea85 71e1    	eor.w	r1, r5, r1, asr #31
 800406a: eb09 0503    	add.w	r5, r9, r3
 800406e: 4441         	add	r1, r8
 8004070: f8d2 8078    	ldr.w	r8, [r2, #0x78]
 8004074: fa45 f504    	asr.w	r5, r5, r4
 8004078: eba5 0606    	sub.w	r6, r5, r6
 800407c: 4408         	add	r0, r1
 800407e: 6615         	str	r5, [r2, #0x60]
 8004080: ea4f 0146    	lsl.w	r1, r6, #0x1
 8004084: 4470         	add	r0, lr
 8004086: ea81 71e6    	eor.w	r1, r1, r6, asr #31
 800408a: f8d2 e090    	ldr.w	lr, [r2, #0x90]
 800408e: 4408         	add	r0, r1
 8004090: eb08 0103    	add.w	r1, r8, r3
 8004094: f8d2 80a8    	ldr.w	r8, [r2, #0xa8]
 8004098: fa41 f104    	asr.w	r1, r1, r4
 800409c: eba1 0505    	sub.w	r5, r1, r5
 80040a0: 6791         	str	r1, [r2, #0x78]
 80040a2: ea4f 0645    	lsl.w	r6, r5, #0x1
 80040a6: ea86 75e5    	eor.w	r5, r6, r5, asr #31
 80040aa: 4428         	add	r0, r5
 80040ac: eb0e 0503    	add.w	r5, lr, r3
 80040b0: fa45 f504    	asr.w	r5, r5, r4
 80040b4: eba5 0101    	sub.w	r1, r5, r1
 80040b8: f8c2 5090    	str.w	r5, [r2, #0x90]
 80040bc: ea4f 0641    	lsl.w	r6, r1, #0x1
 80040c0: ea86 71e1    	eor.w	r1, r6, r1, asr #31
 80040c4: eb00 0e01    	add.w	lr, r0, r1
 80040c8: eb08 0103    	add.w	r1, r8, r3
 80040cc: f8d2 60f0    	ldr.w	r6, [r2, #0xf0]
 80040d0: fa41 f104    	asr.w	r1, r1, r4
 80040d4: eba1 0505    	sub.w	r5, r1, r5
 80040d8: f8c2 10a8    	str.w	r1, [r2, #0xa8]
 80040dc: ea4f 0045    	lsl.w	r0, r5, #0x1
 80040e0: ea80 78e5    	eor.w	r8, r0, r5, asr #31
 80040e4: eb06 0503    	add.w	r5, r6, r3
 80040e8: f8d2 00c0    	ldr.w	r0, [r2, #0xc0]
 80040ec: f8d2 6108    	ldr.w	r6, [r2, #0x108]
 80040f0: 44c6         	add	lr, r8
 80040f2: fa45 f904    	asr.w	r9, r5, r4
 80040f6: f8d2 50d8    	ldr.w	r5, [r2, #0xd8]
 80040fa: 4418         	add	r0, r3
 80040fc: 441e         	add	r6, r3
 80040fe: 441d         	add	r5, r3
 8004100: f8c2 90f0    	str.w	r9, [r2, #0xf0]
 8004104: fa40 f004    	asr.w	r0, r0, r4
 8004108: eba0 0101    	sub.w	r1, r0, r1
 800410c: fa45 f504    	asr.w	r5, r5, r4
 8004110: f8c2 00c0    	str.w	r0, [r2, #0xc0]
 8004114: eba5 0000    	sub.w	r0, r5, r0
 8004118: fa46 fb04    	asr.w	r11, r6, r4
 800411c: ea4f 0641    	lsl.w	r6, r1, #0x1
 8004120: ea86 71e1    	eor.w	r1, r6, r1, asr #31
 8004124: ea4f 0640    	lsl.w	r6, r0, #0x1
 8004128: 4471         	add	r1, lr
 800412a: ea86 70e0    	eor.w	r0, r6, r0, asr #31
 800412e: f8c2 50d8    	str.w	r5, [r2, #0xd8]
 8004132: 4408         	add	r0, r1
 8004134: eba9 0105    	sub.w	r1, r9, r5
 8004138: f8c2 b108    	str.w	r11, [r2, #0x108]
 800413c: f502 7284    	add.w	r2, r2, #0x108
 8004140: ea4f 0641    	lsl.w	r6, r1, #0x1
 8004144: ea86 71e1    	eor.w	r1, r6, r1, asr #31
 8004148: 4408         	add	r0, r1
 800414a: ebab 0109    	sub.w	r1, r11, r9
 800414e: ea4f 0641    	lsl.w	r6, r1, #0x1
 8004152: ea86 71e1    	eor.w	r1, r6, r1, asr #31
 8004156: eb00 0801    	add.w	r8, r0, r1
 800415a: f47f af5f    	bne.w	0x800401c <raylar_crcl_encoder_service::benchmark::fused::h66edd02a0d627444+0x30a> @ imm = #-0x142
 800415e: f8cd 8024    	str.w	r8, [sp, #0x24]
 8004162: a809         	add	r0, sp, #0x24
 8004164: 2301         	movs	r3, #0x1
 8004166: f04f 0e00    	mov.w	lr, #0x0
 800416a: 9809         	ldr	r0, [sp, #0x24]
 800416c: f240 3cff    	movw	r12, #0x3ff
 8004170: 9001         	str	r0, [sp, #0x4]
 8004172: 9805         	ldr	r0, [sp, #0x14]
 8004174: 6980         	ldr	r0, [r0, #0x18]
 8004176: 9008         	str	r0, [sp, #0x20]
 8004178: a808         	add	r0, sp, #0x20
 800417a: 9808         	ldr	r0, [sp, #0x20]
 800417c: 9e04         	ldr	r6, [sp, #0x10]
 800417e: 1e42         	subs	r2, r0, #0x1
 8004180: f000 041f    	and	r4, r0, #0x1f
 8004184: f002 021f    	and	r2, r2, #0x1f
 8004188: f506 758a    	add.w	r5, r6, #0x114
 800418c: 68f1         	ldr	r1, [r6, #0xc]
 800418e: 4093         	lsls	r3, r2
 8004190: 4419         	add	r1, r3
 8004192: fa41 fb04    	asr.w	r11, r1, r4
 8004196: f8c6 b00c    	str.w	r11, [r6, #0xc]
 800419a: f855 6cd8    	ldr	r6, [r5, #-216]
 800419e: f1bc 0c0b    	subs.w	r12, r12, #0xb
 80041a2: f855 2cc0    	ldr	r2, [r5, #-192]
 80041a6: 441e         	add	r6, r3
 80041a8: f855 acf0    	ldr	r10, [r5, #-240]
 80041ac: 441a         	add	r2, r3
 80041ae: f855 8ca8    	ldr	r8, [r5, #-168]
 80041b2: fa46 f604    	asr.w	r6, r6, r4
 80041b6: fa42 f204    	asr.w	r2, r2, r4
 80041ba: eba2 0006    	sub.w	r0, r2, r6
 80041be: f845 6cd8    	str	r6, [r5, #-216]
 80041c2: ea4f 0140    	lsl.w	r1, r0, #0x1
 80041c6: ea81 79e0    	eor.w	r9, r1, r0, asr #31
 80041ca: eb0a 0103    	add.w	r1, r10, r3
 80041ce: f845 2cc0    	str	r2, [r5, #-192]
 80041d2: fa41 f104    	asr.w	r1, r1, r4
 80041d6: eba6 0601    	sub.w	r6, r6, r1
 80041da: f845 1cf0    	str	r1, [r5, #-240]
 80041de: eba1 010b    	sub.w	r1, r1, r11
 80041e2: ea4f 0046    	lsl.w	r0, r6, #0x1
 80041e6: ea80 70e6    	eor.w	r0, r0, r6, asr #31
 80041ea: ea4f 0641    	lsl.w	r6, r1, #0x1
 80041ee: ea86 71e1    	eor.w	r1, r6, r1, asr #31
 80041f2: eb08 0603    	add.w	r6, r8, r3
 80041f6: 4471         	add	r1, lr
 80041f8: f855 ec90    	ldr	lr, [r5, #-144]
 80041fc: fa46 f604    	asr.w	r6, r6, r4
 8004200: eba6 0202    	sub.w	r2, r6, r2
 8004204: 4408         	add	r0, r1
 8004206: f845 6ca8    	str	r6, [r5, #-168]
 800420a: ea4f 0142    	lsl.w	r1, r2, #0x1
 800420e: 4448         	add	r0, r9
 8004210: ea81 71e2    	eor.w	r1, r1, r2, asr #31
 8004214: f855 8c78    	ldr	r8, [r5, #-120]
 8004218: 4408         	add	r0, r1
 800421a: eb0e 0103    	add.w	r1, lr, r3
 800421e: f855 ec60    	ldr	lr, [r5, #-96]
 8004222: fa41 f104    	asr.w	r1, r1, r4
 8004226: eba1 0606    	sub.w	r6, r1, r6
 800422a: f845 1c90    	str	r1, [r5, #-144]
 800422e: ea4f 0246    	lsl.w	r2, r6, #0x1
 8004232: ea82 72e6    	eor.w	r2, r2, r6, asr #31
 8004236: 4410         	add	r0, r2
 8004238: eb08 0203    	add.w	r2, r8, r3
 800423c: fa42 f204    	asr.w	r2, r2, r4
 8004240: eba2 0101    	sub.w	r1, r2, r1
 8004244: f845 2c78    	str	r2, [r5, #-120]
 8004248: ea4f 0641    	lsl.w	r6, r1, #0x1
 800424c: ea86 71e1    	eor.w	r1, r6, r1, asr #31
 8004250: eb00 0801    	add.w	r8, r0, r1
 8004254: eb0e 0103    	add.w	r1, lr, r3
 8004258: f855 6c18    	ldr	r6, [r5, #-24]
 800425c: fa41 f104    	asr.w	r1, r1, r4
 8004260: eba1 0202    	sub.w	r2, r1, r2
 8004264: f845 1c60    	str	r1, [r5, #-96]
 8004268: ea4f 0042    	lsl.w	r0, r2, #0x1
 800426c: ea80 7ee2    	eor.w	lr, r0, r2, asr #31
 8004270: eb06 0203    	add.w	r2, r6, r3
 8004274: f855 0c48    	ldr	r0, [r5, #-72]
 8004278: f855 6c30    	ldr	r6, [r5, #-48]
 800427c: 44c6         	add	lr, r8
 800427e: fa42 f904    	asr.w	r9, r2, r4
 8004282: 4418         	add	r0, r3
 8004284: 682a         	ldr	r2, [r5]
 8004286: 441e         	add	r6, r3
 8004288: fa40 f004    	asr.w	r0, r0, r4
 800428c: eba0 0101    	sub.w	r1, r0, r1
 8004290: 441a         	add	r2, r3
 8004292: fa46 f604    	asr.w	r6, r6, r4
 8004296: f845 0c48    	str	r0, [r5, #-72]
 800429a: eba6 0000    	sub.w	r0, r6, r0
 800429e: fa42 fb04    	asr.w	r11, r2, r4
 80042a2: ea4f 0241    	lsl.w	r2, r1, #0x1
 80042a6: ea82 71e1    	eor.w	r1, r2, r1, asr #31
 80042aa: ea4f 0240    	lsl.w	r2, r0, #0x1
 80042ae: 4471         	add	r1, lr
 80042b0: ea82 70e0    	eor.w	r0, r2, r0, asr #31
 80042b4: 4408         	add	r0, r1
 80042b6: eba9 0106    	sub.w	r1, r9, r6
 80042ba: f845 6c30    	str	r6, [r5, #-48]
 80042be: ea4f 0241    	lsl.w	r2, r1, #0x1
 80042c2: ea82 71e1    	eor.w	r1, r2, r1, asr #31
 80042c6: 4408         	add	r0, r1
 80042c8: ebab 0109    	sub.w	r1, r11, r9
 80042cc: f845 9c18    	str	r9, [r5, #-24]
 80042d0: ea4f 0241    	lsl.w	r2, r1, #0x1
 80042d4: ea82 71e1    	eor.w	r1, r2, r1, asr #31
 80042d8: f8c5 b000    	str.w	r11, [r5]
 80042dc: eb00 0e01    	add.w	lr, r0, r1
 80042e0: f505 7584    	add.w	r5, r5, #0x108
 80042e4: f47f af59    	bne.w	0x800419a <raylar_crcl_encoder_service::benchmark::fused::h66edd02a0d627444+0x488> @ imm = #-0x14e
 80042e8: f8cd e024    	str.w	lr, [sp, #0x24]
 80042ec: a809         	add	r0, sp, #0x24
 80042ee: 2601         	movs	r6, #0x1
 80042f0: f04f 0900    	mov.w	r9, #0x0
 80042f4: 9809         	ldr	r0, [sp, #0x24]
 80042f6: 9000         	str	r0, [sp]
 80042f8: 9805         	ldr	r0, [sp, #0x14]
 80042fa: 6a00         	ldr	r0, [r0, #0x20]
 80042fc: 9008         	str	r0, [sp, #0x20]
 80042fe: a808         	add	r0, sp, #0x20
 8004300: 9808         	ldr	r0, [sp, #0x20]
 8004302: f8dd e010    	ldr.w	lr, [sp, #0x10]
 8004306: 1e42         	subs	r2, r0, #0x1
 8004308: f000 051f    	and	r5, r0, #0x1f
 800430c: f002 021f    	and	r2, r2, #0x1f
 8004310: f240 30ff    	movw	r0, #0x3ff
 8004314: f85e 1f10    	ldr	r1, [lr, #16]!
 8004318: fa06 f402    	lsl.w	r4, r6, r2
 800431c: 4421         	add	r1, r4
 800431e: fa41 fc05    	asr.w	r12, r1, r5
 8004322: f8ce c000    	str.w	r12, [lr]
 8004326: f8de 3030    	ldr.w	r3, [lr, #0x30]
 800432a: f8de 6048    	ldr.w	r6, [lr, #0x48]
 800432e: 4423         	add	r3, r4
 8004330: f8de a018    	ldr.w	r10, [lr, #0x18]
 8004334: 4426         	add	r6, r4
 8004336: 9006         	str	r0, [sp, #0x18]
 8004338: 412b         	asrs	r3, r5
 800433a: f8de 8060    	ldr.w	r8, [lr, #0x60]
 800433e: 412e         	asrs	r6, r5
 8004340: 1af2         	subs	r2, r6, r3
 8004342: f8ce 3030    	str.w	r3, [lr, #0x30]
 8004346: 0050         	lsls	r0, r2, #0x1
 8004348: ea80 7be2    	eor.w	r11, r0, r2, asr #31
 800434c: eb0a 0204    	add.w	r2, r10, r4
 8004350: f8ce 6048    	str.w	r6, [lr, #0x48]
 8004354: 412a         	asrs	r2, r5
 8004356: 1a9b         	subs	r3, r3, r2
 8004358: eba2 010c    	sub.w	r1, r2, r12
 800435c: f8ce 2018    	str.w	r2, [lr, #0x18]
 8004360: 0058         	lsls	r0, r3, #0x1
 8004362: 004a         	lsls	r2, r1, #0x1
 8004364: ea82 71e1    	eor.w	r1, r2, r1, asr #31
 8004368: ea80 70e3    	eor.w	r0, r0, r3, asr #31
 800436c: 4449         	add	r1, r9
 800436e: 4408         	add	r0, r1
 8004370: f8de 1078    	ldr.w	r1, [lr, #0x78]
 8004374: eb08 0204    	add.w	r2, r8, r4
 8004378: 4458         	add	r0, r11
 800437a: 4421         	add	r1, r4
 800437c: 412a         	asrs	r2, r5
 800437e: 1b93         	subs	r3, r2, r6
 8004380: 4129         	asrs	r1, r5
 8004382: f8ce 2060    	str.w	r2, [lr, #0x60]
 8004386: 005e         	lsls	r6, r3, #0x1
 8004388: 1a8a         	subs	r2, r1, r2
 800438a: ea86 73e3    	eor.w	r3, r6, r3, asr #31
 800438e: f8de 6090    	ldr.w	r6, [lr, #0x90]
 8004392: 4418         	add	r0, r3
 8004394: 0053         	lsls	r3, r2, #0x1
 8004396: ea83 72e2    	eor.w	r2, r3, r2, asr #31
 800439a: f8ce 1078    	str.w	r1, [lr, #0x78]
 800439e: 4410         	add	r0, r2
 80043a0: 1932         	adds	r2, r6, r4
 80043a2: f8de 30a8    	ldr.w	r3, [lr, #0xa8]
 80043a6: 412a         	asrs	r2, r5
 80043a8: 1a51         	subs	r1, r2, r1
 80043aa: f8ce 2090    	str.w	r2, [lr, #0x90]
 80043ae: 004e         	lsls	r6, r1, #0x1
 80043b0: ea86 71e1    	eor.w	r1, r6, r1, asr #31
 80043b4: eb00 0801    	add.w	r8, r0, r1
 80043b8: 1919         	adds	r1, r3, r4
 80043ba: f8de 60f0    	ldr.w	r6, [lr, #0xf0]
 80043be: fa41 f305    	asr.w	r3, r1, r5
 80043c2: 1a99         	subs	r1, r3, r2
 80043c4: f8ce 30a8    	str.w	r3, [lr, #0xa8]
 80043c8: 004a         	lsls	r2, r1, #0x1
 80043ca: ea82 79e1    	eor.w	r9, r2, r1, asr #31
 80043ce: 1931         	adds	r1, r6, r4
 80043d0: 44c8         	add	r8, r9
 80043d2: fa41 f205    	asr.w	r2, r1, r5
 80043d6: f8de 10d8    	ldr.w	r1, [lr, #0xd8]
 80043da: f8ce 20f0    	str.w	r2, [lr, #0xf0]
 80043de: 4421         	add	r1, r4
 80043e0: fa41 f605    	asr.w	r6, r1, r5
 80043e4: f8de 10c0    	ldr.w	r1, [lr, #0xc0]
 80043e8: f8ce 60d8    	str.w	r6, [lr, #0xd8]
 80043ec: 4421         	add	r1, r4
 80043ee: fa41 f005    	asr.w	r0, r1, r5
 80043f2: f8de 1108    	ldr.w	r1, [lr, #0x108]
 80043f6: 1ac3         	subs	r3, r0, r3
 80043f8: f8ce 00c0    	str.w	r0, [lr, #0xc0]
 80043fc: 4421         	add	r1, r4
 80043fe: 1a30         	subs	r0, r6, r0
 8004400: fa41 fc05    	asr.w	r12, r1, r5
 8004404: 0059         	lsls	r1, r3, #0x1
 8004406: ea81 71e3    	eor.w	r1, r1, r3, asr #31
 800440a: 0043         	lsls	r3, r0, #0x1
 800440c: 4441         	add	r1, r8
 800440e: ea83 70e0    	eor.w	r0, r3, r0, asr #31
 8004412: 4408         	add	r0, r1
 8004414: 1b91         	subs	r1, r2, r6
 8004416: f8ce c108    	str.w	r12, [lr, #0x108]
 800441a: f50e 7e84    	add.w	lr, lr, #0x108
 800441e: 004b         	lsls	r3, r1, #0x1
 8004420: ea83 71e1    	eor.w	r1, r3, r1, asr #31
 8004424: 4408         	add	r0, r1
 8004426: ebac 0102    	sub.w	r1, r12, r2
 800442a: 004a         	lsls	r2, r1, #0x1
 800442c: ea82 71e1    	eor.w	r1, r2, r1, asr #31
 8004430: eb00 0901    	add.w	r9, r0, r1
 8004434: 9806         	ldr	r0, [sp, #0x18]
 8004436: 380b         	subs	r0, #0xb
 8004438: f47f af75    	bne.w	0x8004326 <raylar_crcl_encoder_service::benchmark::fused::h66edd02a0d627444+0x614> @ imm = #-0x116
 800443c: f8cd 9024    	str.w	r9, [sp, #0x24]
 8004440: a809         	add	r0, sp, #0x24
 8004442: f246 0128    	movw	r1, #0x6028
 8004446: 9a05         	ldr	r2, [sp, #0x14]
 8004448: 9809         	ldr	r0, [sp, #0x24]
 800444a: 4411         	add	r1, r2
 800444c: 9107         	str	r1, [sp, #0x1c]
 800444e: a907         	add	r1, sp, #0x1c
 8004450: e9dd 2102    	ldrd	r2, r1, [sp, #8]
 8004454: 4051         	eors	r1, r2
 8004456: 9a01         	ldr	r2, [sp, #0x4]
 8004458: 4051         	eors	r1, r2
 800445a: 9a00         	ldr	r2, [sp]
 800445c: 4051         	eors	r1, r2
 800445e: 4048         	eors	r0, r1
 8004460: b00a         	add	sp, #0x28
 8004462: e8bd 0f00    	pop.w	{r8, r9, r10, r11}
 8004466: bdf0         	pop	{r4, r5, r6, r7, pc}

08004468 <raylar_crcl_encoder_service::benchmark::quant::h5a540275f2df62e3>:
 8004468: b580         	push	{r7, lr}
 800446a: 466f         	mov	r7, sp
 800446c: f246 0128    	movw	r1, #0x6028
 8004470: 1842         	adds	r2, r0, r1
 8004472: f24c 0128    	movw	r1, #0xc028
 8004476: 4401         	add	r1, r0
 8004478: 4610         	mov	r0, r2
 800447a: f44f 42c0    	mov.w	r2, #0x6000
 800447e: f008 fae8    	bl	0x800ca52 <__aeabi_memcpy4> @ imm = #0x85d0
 8004482: bd80         	pop	{r7, pc}

08004484 <raylar_crcl_encoder_service::benchmark::scale::h44b3d035bda7af78>:
 8004484: b5f0         	push	{r4, r5, r6, r7, lr}
 8004486: af03         	add	r7, sp, #0xc
 8004488: e92d 0f00    	push.w	{r8, r9, r10, r11}
 800448c: b086         	sub	sp, #0x18
 800448e: f10d 0e0c    	add.w	lr, sp, #0xc
 8004492: f1a7 0122    	sub.w	r1, r7, #0x22
 8004496: f1a7 0b21    	sub.w	r11, r7, #0x21
 800449a: 2000         	movs	r0, #0x0
 800449c: f44f 7380    	mov.w	r3, #0x100
 80044a0: f04f 080a    	mov.w	r8, #0xa
 80044a4: f04f 0c06    	mov.w	r12, #0x6
 80044a8: e030         	b	0x800450c <raylar_crcl_encoder_service::benchmark::scale::h44b3d035bda7af78+0x88> @ imm = #0x60
 80044aa: eba5 0806    	sub.w	r8, r5, r6
 80044ae: 2101         	movs	r1, #0x1
 80044b0: f1a8 0501    	sub.w	r5, r8, #0x1
 80044b4: f003 031f    	and	r3, r3, #0x1f
 80044b8: fa01 f505    	lsl.w	r5, r1, r5
 80044bc: 442a         	add	r2, r5
 80044be: f008 051f    	and	r5, r8, #0x1f
 80044c2: 40ea         	lsrs	r2, r5
 80044c4: 40da         	lsrs	r2, r3
 80044c6: bf18         	it	ne
 80044c8: f108 0801    	addne.w	r8, r8, #0x1
 80044cc: e9dd 1301    	ldrd	r1, r3, [sp, #4]
 80044d0: 46a6         	mov	lr, r4
 80044d2: 4048         	eors	r0, r1
 80044d4: 9900         	ldr	r1, [sp]
 80044d6: 4048         	eors	r0, r1
 80044d8: a905         	add	r1, sp, #0x14
 80044da: ea80 000b    	eor.w	r0, r0, r11
 80044de: 46cb         	mov	r11, r9
 80044e0: ea80 000c    	eor.w	r0, r0, r12
 80044e4: f04f 0c06    	mov.w	r12, #0x6
 80044e8: f807 cc21    	strb	r12, [r7, #-33]
 80044ec: f817 2c21    	ldrb	r2, [r7, #-33]
 80044f0: 4542         	cmp	r2, r8
 80044f2: bf88         	it	hi
 80044f4: 4690         	movhi	r8, r2
 80044f6: f8cd 8014    	str.w	r8, [sp, #0x14]
 80044fa: 3b01         	subs	r3, #0x1
 80044fc: f04f 080a    	mov.w	r8, #0xa
 8004500: 4651         	mov	r1, r10
 8004502: 9a05         	ldr	r2, [sp, #0x14]
 8004504: ea80 0002    	eor.w	r0, r0, r2
 8004508: f000 80c2    	beq.w	0x8004690 <raylar_crcl_encoder_service::benchmark::scale::h44b3d035bda7af78+0x20c> @ imm = #0x184
 800450c: f245 2288    	movw	r2, #0x5288
 8004510: 9302         	str	r3, [sp, #0x8]
 8004512: 9203         	str	r2, [sp, #0xc]
 8004514: f807 8c22    	strb	r8, [r7, #-34]
 8004518: 9b03         	ldr	r3, [sp, #0xc]
 800451a: f817 6c22    	ldrb	r6, [r7, #-34]
 800451e: fab3 f283    	clz	r2, r3
 8004522: 1e75         	subs	r5, r6, #0x1
 8004524: f1c2 0220    	rsb.w	r2, r2, #0x20
 8004528: b2ee         	uxtb	r6, r5
 800452a: 42b2         	cmp	r2, r6
 800452c: d90e         	bls	0x800454c <raylar_crcl_encoder_service::benchmark::scale::h44b3d035bda7af78+0xc8> @ imm = #0x1c
 800452e: 1b92         	subs	r2, r2, r6
 8004530: 2401         	movs	r4, #0x1
 8004532: 1e56         	subs	r6, r2, #0x1
 8004534: fa04 f606    	lsl.w	r6, r4, r6
 8004538: 4433         	add	r3, r6
 800453a: f002 061f    	and	r6, r2, #0x1f
 800453e: 40f3         	lsrs	r3, r6
 8004540: f005 061f    	and	r6, r5, #0x1f
 8004544: 40f3         	lsrs	r3, r6
 8004546: bf18         	it	ne
 8004548: 3201         	addne	r2, #0x1
 800454a: e000         	b	0x800454e <raylar_crcl_encoder_service::benchmark::scale::h44b3d035bda7af78+0xca> @ imm = #0x0
 800454c: 2200         	movs	r2, #0x0
 800454e: f807 cc21    	strb	r12, [r7, #-33]
 8004552: f10d 0a14    	add.w	r10, sp, #0x14
 8004556: f817 3c21    	ldrb	r3, [r7, #-33]
 800455a: 4293         	cmp	r3, r2
 800455c: bf88         	it	hi
 800455e: 461a         	movhi	r2, r3
 8004560: 9205         	str	r2, [sp, #0x14]
 8004562: 9a05         	ldr	r2, [sp, #0x14]
 8004564: 9201         	str	r2, [sp, #0x4]
 8004566: f245 6270    	movw	r2, #0x5670
 800456a: 9203         	str	r2, [sp, #0xc]
 800456c: f807 8c22    	strb	r8, [r7, #-34]
 8004570: 9b03         	ldr	r3, [sp, #0xc]
 8004572: f817 6c22    	ldrb	r6, [r7, #-34]
 8004576: fab3 f283    	clz	r2, r3
 800457a: 1e75         	subs	r5, r6, #0x1
 800457c: f1c2 0220    	rsb.w	r2, r2, #0x20
 8004580: b2ee         	uxtb	r6, r5
 8004582: 42b2         	cmp	r2, r6
 8004584: d90e         	bls	0x80045a4 <raylar_crcl_encoder_service::benchmark::scale::h44b3d035bda7af78+0x120> @ imm = #0x1c
 8004586: 1b92         	subs	r2, r2, r6
 8004588: 2401         	movs	r4, #0x1
 800458a: 1e56         	subs	r6, r2, #0x1
 800458c: fa04 f606    	lsl.w	r6, r4, r6
 8004590: 4433         	add	r3, r6
 8004592: f002 061f    	and	r6, r2, #0x1f
 8004596: 40f3         	lsrs	r3, r6
 8004598: f005 061f    	and	r6, r5, #0x1f
 800459c: 40f3         	lsrs	r3, r6
 800459e: bf18         	it	ne
 80045a0: 3201         	addne	r2, #0x1
 80045a2: e000         	b	0x80045a6 <raylar_crcl_encoder_service::benchmark::scale::h44b3d035bda7af78+0x122> @ imm = #0x0
 80045a4: 2200         	movs	r2, #0x0
 80045a6: f807 cc21    	strb	r12, [r7, #-33]
 80045aa: f817 3c21    	ldrb	r3, [r7, #-33]
 80045ae: 4293         	cmp	r3, r2
 80045b0: bf88         	it	hi
 80045b2: 461a         	movhi	r2, r3
 80045b4: 9205         	str	r2, [sp, #0x14]
 80045b6: 9a05         	ldr	r2, [sp, #0x14]
 80045b8: 9200         	str	r2, [sp]
 80045ba: f645 2258    	movw	r2, #0x5a58
 80045be: 9203         	str	r2, [sp, #0xc]
 80045c0: f807 8c22    	strb	r8, [r7, #-34]
 80045c4: 9b03         	ldr	r3, [sp, #0xc]
 80045c6: f817 6c22    	ldrb	r6, [r7, #-34]
 80045ca: fab3 f283    	clz	r2, r3
 80045ce: 1e75         	subs	r5, r6, #0x1
 80045d0: f1c2 0220    	rsb.w	r2, r2, #0x20
 80045d4: b2ee         	uxtb	r6, r5
 80045d6: 42b2         	cmp	r2, r6
 80045d8: d90e         	bls	0x80045f8 <raylar_crcl_encoder_service::benchmark::scale::h44b3d035bda7af78+0x174> @ imm = #0x1c
 80045da: 1b92         	subs	r2, r2, r6
 80045dc: 2401         	movs	r4, #0x1
 80045de: 1e56         	subs	r6, r2, #0x1
 80045e0: fa04 f606    	lsl.w	r6, r4, r6
 80045e4: 4433         	add	r3, r6
 80045e6: f002 061f    	and	r6, r2, #0x1f
 80045ea: 40f3         	lsrs	r3, r6
 80045ec: f005 061f    	and	r6, r5, #0x1f
 80045f0: 40f3         	lsrs	r3, r6
 80045f2: bf18         	it	ne
 80045f4: 3201         	addne	r2, #0x1
 80045f6: e000         	b	0x80045fa <raylar_crcl_encoder_service::benchmark::scale::h44b3d035bda7af78+0x176> @ imm = #0x0
 80045f8: 2200         	movs	r2, #0x0
 80045fa: f807 cc21    	strb	r12, [r7, #-33]
 80045fe: 46d9         	mov	r9, r11
 8004600: f817 3c21    	ldrb	r3, [r7, #-33]
 8004604: 4293         	cmp	r3, r2
 8004606: bf88         	it	hi
 8004608: 461a         	movhi	r2, r3
 800460a: 9205         	str	r2, [sp, #0x14]
 800460c: f645 6240    	movw	r2, #0x5e40
 8004610: 9203         	str	r2, [sp, #0xc]
 8004612: f8dd b014    	ldr.w	r11, [sp, #0x14]
 8004616: f807 8c22    	strb	r8, [r7, #-34]
 800461a: 9b03         	ldr	r3, [sp, #0xc]
 800461c: f817 6c22    	ldrb	r6, [r7, #-34]
 8004620: fab3 f283    	clz	r2, r3
 8004624: 1e75         	subs	r5, r6, #0x1
 8004626: f1c2 0220    	rsb.w	r2, r2, #0x20
 800462a: b2ee         	uxtb	r6, r5
 800462c: 42b2         	cmp	r2, r6
 800462e: d90e         	bls	0x800464e <raylar_crcl_encoder_service::benchmark::scale::h44b3d035bda7af78+0x1ca> @ imm = #0x1c
 8004630: 1b92         	subs	r2, r2, r6
 8004632: 2401         	movs	r4, #0x1
 8004634: 1e56         	subs	r6, r2, #0x1
 8004636: fa04 f606    	lsl.w	r6, r4, r6
 800463a: 4433         	add	r3, r6
 800463c: f002 061f    	and	r6, r2, #0x1f
 8004640: 40f3         	lsrs	r3, r6
 8004642: f005 061f    	and	r6, r5, #0x1f
 8004646: 40f3         	lsrs	r3, r6
 8004648: bf18         	it	ne
 800464a: 3201         	addne	r2, #0x1
 800464c: e000         	b	0x8004650 <raylar_crcl_encoder_service::benchmark::scale::h44b3d035bda7af78+0x1cc> @ imm = #0x0
 800464e: 2200         	movs	r2, #0x0
 8004650: f807 cc21    	strb	r12, [r7, #-33]
 8004654: 4674         	mov	r4, lr
 8004656: f817 3c21    	ldrb	r3, [r7, #-33]
 800465a: 4293         	cmp	r3, r2
 800465c: bf88         	it	hi
 800465e: 461a         	movhi	r2, r3
 8004660: 9205         	str	r2, [sp, #0x14]
 8004662: f246 2228    	movw	r2, #0x6228
 8004666: 9203         	str	r2, [sp, #0xc]
 8004668: 468a         	mov	r10, r1
 800466a: f8dd c014    	ldr.w	r12, [sp, #0x14]
 800466e: f807 8c22    	strb	r8, [r7, #-34]
 8004672: 9a03         	ldr	r2, [sp, #0xc]
 8004674: f817 3c22    	ldrb	r3, [r7, #-34]
 8004678: fab2 f582    	clz	r5, r2
 800467c: 3b01         	subs	r3, #0x1
 800467e: f1c5 0520    	rsb.w	r5, r5, #0x20
 8004682: b2de         	uxtb	r6, r3
 8004684: 42b5         	cmp	r5, r6
 8004686: f63f af10    	bhi.w	0x80044aa <raylar_crcl_encoder_service::benchmark::scale::h44b3d035bda7af78+0x26> @ imm = #-0x1e0
 800468a: f04f 0800    	mov.w	r8, #0x0
 800468e: e71d         	b	0x80044cc <raylar_crcl_encoder_service::benchmark::scale::h44b3d035bda7af78+0x48> @ imm = #-0x1c6
 8004690: b006         	add	sp, #0x18
 8004692: e8bd 0f00    	pop.w	{r8, r9, r10, r11}
 8004696: bdf0         	pop	{r4, r5, r6, r7, pc}

08004698 <raylar_crcl_encoder_service::benchmark::rice_k::h2637d4bce634509f>:
 8004698: b5f0         	push	{r4, r5, r6, r7, lr}
 800469a: af03         	add	r7, sp, #0xc
 800469c: e92d 0f00    	push.w	{r8, r9, r10, r11}
 80046a0: b08a         	sub	sp, #0x28
 80046a2: f64f 41b0    	movw	r1, #0xfcb0
 80046a6: f10d 081c    	add.w	r8, sp, #0x1c
 80046aa: f2c0 0101    	movt	r1, #0x1
 80046ae: 1842         	adds	r2, r0, r1
 80046b0: 5840         	ldr	r0, [r0, r1]
 80046b2: f10d 0e20    	add.w	lr, sp, #0x20
 80046b6: 6851         	ldr	r1, [r2, #0x4]
 80046b8: 9103         	str	r1, [sp, #0xc]
 80046ba: ac09         	add	r4, sp, #0x24
 80046bc: 6891         	ldr	r1, [r2, #0x8]
 80046be: f44f 7980    	mov.w	r9, #0x100
 80046c2: 9102         	str	r1, [sp, #0x8]
 80046c4: f240 3bff    	movw	r11, #0x3ff
 80046c8: 68d1         	ldr	r1, [r2, #0xc]
 80046ca: 9004         	str	r0, [sp, #0x10]
 80046cc: 2000         	movs	r0, #0x0
 80046ce: 9101         	str	r1, [sp, #0x4]
 80046d0: 6911         	ldr	r1, [r2, #0x10]
 80046d2: 9100         	str	r1, [sp]
 80046d4: e017         	b	0x8004706 <raylar_crcl_encoder_service::benchmark::rice_k::h2637d4bce634509f+0x6e> @ imm = #0x2e
 80046d6: 9906         	ldr	r1, [sp, #0x18]
 80046d8: f1b9 0901    	subs.w	r9, r9, #0x1
 80046dc: f8cd 8024    	str.w	r8, [sp, #0x24]
 80046e0: 462c         	mov	r4, r5
 80046e2: ea80 0001    	eor.w	r0, r0, r1
 80046e6: 9905         	ldr	r1, [sp, #0x14]
 80046e8: 46d0         	mov	r8, r10
 80046ea: 4693         	mov	r11, r2
 80046ec: ea80 0001    	eor.w	r0, r0, r1
 80046f0: 9909         	ldr	r1, [sp, #0x24]
 80046f2: ea80 000c    	eor.w	r0, r0, r12
 80046f6: ea80 000e    	eor.w	r0, r0, lr
 80046fa: f10d 0e20    	add.w	lr, sp, #0x20
 80046fe: ea80 0001    	eor.w	r0, r0, r1
 8004702: f000 8087    	beq.w	0x8004814 <raylar_crcl_encoder_service::benchmark::rice_k::h2637d4bce634509f+0x17c> @ imm = #0x10e
 8004706: 9904         	ldr	r1, [sp, #0x10]
 8004708: 2200         	movs	r2, #0x0
 800470a: 9107         	str	r1, [sp, #0x1c]
 800470c: f8cd b020    	str.w	r11, [sp, #0x20]
 8004710: 9e07         	ldr	r6, [sp, #0x1c]
 8004712: 2e00         	cmp	r6, #0x0
 8004714: bf1c         	itt	ne
 8004716: 9b08         	ldrne	r3, [sp, #0x20]
 8004718: 2b00         	cmpne	r3, #0x0
 800471a: d00b         	beq	0x8004734 <raylar_crcl_encoder_service::benchmark::rice_k::h2637d4bce634509f+0x9c> @ imm = #0x16
 800471c: fbb6 f1f3    	udiv	r1, r6, r3
 8004720: fb01 6213    	mls	r2, r1, r3, r6
 8004724: 2a00         	cmp	r2, #0x0
 8004726: bf18         	it	ne
 8004728: 3101         	addne	r1, #0x1
 800472a: fab1 f181    	clz	r1, r1
 800472e: f1c1 011f    	rsb.w	r1, r1, #0x1f
 8004732: b2ca         	uxtb	r2, r1
 8004734: 9209         	str	r2, [sp, #0x24]
 8004736: 2500         	movs	r5, #0x0
 8004738: 9909         	ldr	r1, [sp, #0x24]
 800473a: 9106         	str	r1, [sp, #0x18]
 800473c: 9903         	ldr	r1, [sp, #0xc]
 800473e: 9107         	str	r1, [sp, #0x1c]
 8004740: f8cd b020    	str.w	r11, [sp, #0x20]
 8004744: 9e07         	ldr	r6, [sp, #0x1c]
 8004746: 2e00         	cmp	r6, #0x0
 8004748: bf1c         	itt	ne
 800474a: 9b08         	ldrne	r3, [sp, #0x20]
 800474c: 2b00         	cmpne	r3, #0x0
 800474e: d00b         	beq	0x8004768 <raylar_crcl_encoder_service::benchmark::rice_k::h2637d4bce634509f+0xd0> @ imm = #0x16
 8004750: fbb6 f1f3    	udiv	r1, r6, r3
 8004754: fb01 6313    	mls	r3, r1, r3, r6
 8004758: 2b00         	cmp	r3, #0x0
 800475a: bf18         	it	ne
 800475c: 3101         	addne	r1, #0x1
 800475e: fab1 f181    	clz	r1, r1
 8004762: f1c1 011f    	rsb.w	r1, r1, #0x1f
 8004766: b2cd         	uxtb	r5, r1
 8004768: 9509         	str	r5, [sp, #0x24]
 800476a: 2600         	movs	r6, #0x0
 800476c: 9909         	ldr	r1, [sp, #0x24]
 800476e: 9105         	str	r1, [sp, #0x14]
 8004770: 9902         	ldr	r1, [sp, #0x8]
 8004772: 9107         	str	r1, [sp, #0x1c]
 8004774: f8cd b020    	str.w	r11, [sp, #0x20]
 8004778: 9b07         	ldr	r3, [sp, #0x1c]
 800477a: 2b00         	cmp	r3, #0x0
 800477c: bf1c         	itt	ne
 800477e: 9908         	ldrne	r1, [sp, #0x20]
 8004780: 2900         	cmpne	r1, #0x0
 8004782: d00b         	beq	0x800479c <raylar_crcl_encoder_service::benchmark::rice_k::h2637d4bce634509f+0x104> @ imm = #0x16
 8004784: fbb3 f6f1    	udiv	r6, r3, r1
 8004788: fb06 3111    	mls	r1, r6, r1, r3
 800478c: 2900         	cmp	r1, #0x0
 800478e: bf18         	it	ne
 8004790: 3601         	addne	r6, #0x1
 8004792: fab6 f186    	clz	r1, r6
 8004796: f1c1 011f    	rsb.w	r1, r1, #0x1f
 800479a: b2ce         	uxtb	r6, r1
 800479c: 9609         	str	r6, [sp, #0x24]
 800479e: 4625         	mov	r5, r4
 80047a0: 2600         	movs	r6, #0x0
 80047a2: 9901         	ldr	r1, [sp, #0x4]
 80047a4: 9107         	str	r1, [sp, #0x1c]
 80047a6: f8dd c024    	ldr.w	r12, [sp, #0x24]
 80047aa: 9b07         	ldr	r3, [sp, #0x1c]
 80047ac: f8cd b020    	str.w	r11, [sp, #0x20]
 80047b0: 2b00         	cmp	r3, #0x0
 80047b2: bf1c         	itt	ne
 80047b4: 9908         	ldrne	r1, [sp, #0x20]
 80047b6: 2900         	cmpne	r1, #0x0
 80047b8: d00b         	beq	0x80047d2 <raylar_crcl_encoder_service::benchmark::rice_k::h2637d4bce634509f+0x13a> @ imm = #0x16
 80047ba: fbb3 f6f1    	udiv	r6, r3, r1
 80047be: fb06 3111    	mls	r1, r6, r1, r3
 80047c2: 2900         	cmp	r1, #0x0
 80047c4: bf18         	it	ne
 80047c6: 3601         	addne	r6, #0x1
 80047c8: fab6 f186    	clz	r1, r6
 80047cc: f1c1 011f    	rsb.w	r1, r1, #0x1f
 80047d0: b2ce         	uxtb	r6, r1
 80047d2: 9609         	str	r6, [sp, #0x24]
 80047d4: 46c2         	mov	r10, r8
 80047d6: 465a         	mov	r2, r11
 80047d8: 9900         	ldr	r1, [sp]
 80047da: 9107         	str	r1, [sp, #0x1c]
 80047dc: a908         	add	r1, sp, #0x20
 80047de: f8dd e024    	ldr.w	lr, [sp, #0x24]
 80047e2: f04f 0800    	mov.w	r8, #0x0
 80047e6: 9b07         	ldr	r3, [sp, #0x1c]
 80047e8: f8cd b020    	str.w	r11, [sp, #0x20]
 80047ec: 2b00         	cmp	r3, #0x0
 80047ee: bf1c         	itt	ne
 80047f0: 9908         	ldrne	r1, [sp, #0x20]
 80047f2: 2900         	cmpne	r1, #0x0
 80047f4: f43f af6f    	beq.w	0x80046d6 <raylar_crcl_encoder_service::benchmark::rice_k::h2637d4bce634509f+0x3e> @ imm = #-0x122
 80047f8: fbb3 f6f1    	udiv	r6, r3, r1
 80047fc: fb06 3111    	mls	r1, r6, r1, r3
 8004800: 2900         	cmp	r1, #0x0
 8004802: bf18         	it	ne
 8004804: 3601         	addne	r6, #0x1
 8004806: fab6 f186    	clz	r1, r6
 800480a: f1c1 011f    	rsb.w	r1, r1, #0x1f
 800480e: fa5f f881    	uxtb.w	r8, r1
 8004812: e760         	b	0x80046d6 <raylar_crcl_encoder_service::benchmark::rice_k::h2637d4bce634509f+0x3e> @ imm = #-0x140
 8004814: b00a         	add	sp, #0x28
 8004816: e8bd 0f00    	pop.w	{r8, r9, r10, r11}
 800481a: bdf0         	pop	{r4, r5, r6, r7, pc}

0800481c <raylar_crcl_encoder_service::benchmark::dcblock::h5d1170af065dd1eb>:
 800481c: b5f0         	push	{r4, r5, r6, r7, lr}
 800481e: af03         	add	r7, sp, #0xc
 8004820: e92d 0f00    	push.w	{r8, r9, r10, r11}
 8004824: b083         	sub	sp, #0xc
 8004826: f645 716c    	movw	r1, #0x5f6c
 800482a: 2600         	movs	r6, #0x0
 800482c: 4401         	add	r1, r0
 800482e: f44f 6280    	mov.w	r2, #0x400
 8004832: 2300         	movs	r3, #0x0
 8004834: 9001         	str	r0, [sp, #0x4]
 8004836: eb03 2326    	add.w	r3, r3, r6, asr #8
 800483a: f8d1 00c0    	ldr.w	r0, [r1, #0xc0]
 800483e: 1af3         	subs	r3, r6, r3
 8004840: f8d1 50d8    	ldr.w	r5, [r1, #0xd8]
 8004844: 4418         	add	r0, r3
 8004846: f8c1 00c0    	str.w	r0, [r1, #0xc0]
 800484a: f8d1 40f0    	ldr.w	r4, [r1, #0xf0]
 800484e: 3a10         	subs	r2, #0x10
 8004850: eba3 2020    	sub.w	r0, r3, r0, asr #8
 8004854: f8d1 8108    	ldr.w	r8, [r1, #0x108]
 8004858: eb00 0305    	add.w	r3, r0, r5
 800485c: f8c1 30d8    	str.w	r3, [r1, #0xd8]
 8004860: f8d1 6120    	ldr.w	r6, [r1, #0x120]
 8004864: eba0 2023    	sub.w	r0, r0, r3, asr #8
 8004868: f8d1 5168    	ldr.w	r5, [r1, #0x168]
 800486c: eb00 0304    	add.w	r3, r0, r4
 8004870: f8c1 30f0    	str.w	r3, [r1, #0xf0]
 8004874: f8d1 4150    	ldr.w	r4, [r1, #0x150]
 8004878: eba0 2023    	sub.w	r0, r0, r3, asr #8
 800487c: f8d1 e180    	ldr.w	lr, [r1, #0x180]
 8004880: eb00 0308    	add.w	r3, r0, r8
 8004884: f8c1 3108    	str.w	r3, [r1, #0x108]
 8004888: f8d1 c198    	ldr.w	r12, [r1, #0x198]
 800488c: eba0 2023    	sub.w	r0, r0, r3, asr #8
 8004890: eb00 0306    	add.w	r3, r0, r6
 8004894: f8d1 6138    	ldr.w	r6, [r1, #0x138]
 8004898: f8c1 3120    	str.w	r3, [r1, #0x120]
 800489c: eba0 2023    	sub.w	r0, r0, r3, asr #8
 80048a0: eb00 0306    	add.w	r3, r0, r6
 80048a4: f8c1 3138    	str.w	r3, [r1, #0x138]
 80048a8: f8d1 61b0    	ldr.w	r6, [r1, #0x1b0]
 80048ac: eba0 2023    	sub.w	r0, r0, r3, asr #8
 80048b0: eb00 0304    	add.w	r3, r0, r4
 80048b4: f8c1 3150    	str.w	r3, [r1, #0x150]
 80048b8: eba0 2023    	sub.w	r0, r0, r3, asr #8
 80048bc: eb00 0305    	add.w	r3, r0, r5
 80048c0: f8c1 3168    	str.w	r3, [r1, #0x168]
 80048c4: eba0 2023    	sub.w	r0, r0, r3, asr #8
 80048c8: eb00 030e    	add.w	r3, r0, lr
 80048cc: f8c1 3180    	str.w	r3, [r1, #0x180]
 80048d0: eba0 2023    	sub.w	r0, r0, r3, asr #8
 80048d4: eb00 030c    	add.w	r3, r0, r12
 80048d8: f8c1 3198    	str.w	r3, [r1, #0x198]
 80048dc: eba0 2023    	sub.w	r0, r0, r3, asr #8
 80048e0: eb00 0306    	add.w	r3, r0, r6
 80048e4: f8d1 61c8    	ldr.w	r6, [r1, #0x1c8]
 80048e8: f8c1 31b0    	str.w	r3, [r1, #0x1b0]
 80048ec: eba0 2023    	sub.w	r0, r0, r3, asr #8
 80048f0: eb00 0306    	add.w	r3, r0, r6
 80048f4: f8d1 61e0    	ldr.w	r6, [r1, #0x1e0]
 80048f8: f8c1 31c8    	str.w	r3, [r1, #0x1c8]
 80048fc: eba0 2023    	sub.w	r0, r0, r3, asr #8
 8004900: eb00 0306    	add.w	r3, r0, r6
 8004904: f8d1 61f8    	ldr.w	r6, [r1, #0x1f8]
 8004908: f8c1 31e0    	str.w	r3, [r1, #0x1e0]
 800490c: eba0 2023    	sub.w	r0, r0, r3, asr #8
 8004910: eb00 0306    	add.w	r3, r0, r6
 8004914: f8d1 6210    	ldr.w	r6, [r1, #0x210]
 8004918: f8c1 31f8    	str.w	r3, [r1, #0x1f8]
 800491c: eba0 2023    	sub.w	r0, r0, r3, asr #8
 8004920: f8d1 3228    	ldr.w	r3, [r1, #0x228]
 8004924: 4406         	add	r6, r0
 8004926: f8c1 6210    	str.w	r6, [r1, #0x210]
 800492a: eba0 2026    	sub.w	r0, r0, r6, asr #8
 800492e: eb00 0603    	add.w	r6, r0, r3
 8004932: f8c1 6228    	str.w	r6, [r1, #0x228]
 8004936: f501 71c0    	add.w	r1, r1, #0x180
 800493a: f47f af7c    	bne.w	0x8004836 <raylar_crcl_encoder_service::benchmark::dcblock::h5d1170af065dd1eb+0x1a> @ imm = #-0x108
 800493e: 9801         	ldr	r0, [sp, #0x4]
 8004940: f645 61b0    	movw	r1, #0x5eb0
 8004944: f04f 0e00    	mov.w	lr, #0x0
 8004948: f44f 6280    	mov.w	r2, #0x400
 800494c: 4401         	add	r1, r0
 800494e: 2300         	movs	r3, #0x0
 8004950: 9600         	str	r6, [sp]
 8004952: eb03 232e    	add.w	r3, r3, lr, asr #8
 8004956: f8d1 4180    	ldr.w	r4, [r1, #0x180]
 800495a: ebae 0303    	sub.w	r3, lr, r3
 800495e: f8d1 5198    	ldr.w	r5, [r1, #0x198]
 8004962: 441c         	add	r4, r3
 8004964: f8c1 4180    	str.w	r4, [r1, #0x180]
 8004968: f8d1 61b0    	ldr.w	r6, [r1, #0x1b0]
 800496c: 3a10         	subs	r2, #0x10
 800496e: eba3 2324    	sub.w	r3, r3, r4, asr #8
 8004972: f8d1 81c8    	ldr.w	r8, [r1, #0x1c8]
 8004976: eb03 0405    	add.w	r4, r3, r5
 800497a: f8c1 4198    	str.w	r4, [r1, #0x198]
 800497e: f8d1 51e0    	ldr.w	r5, [r1, #0x1e0]
 8004982: eba3 2324    	sub.w	r3, r3, r4, asr #8
 8004986: f8d1 a228    	ldr.w	r10, [r1, #0x228]
 800498a: eb03 0406    	add.w	r4, r3, r6
 800498e: f8c1 41b0    	str.w	r4, [r1, #0x1b0]
 8004992: f8d1 6210    	ldr.w	r6, [r1, #0x210]
 8004996: eba3 2324    	sub.w	r3, r3, r4, asr #8
 800499a: f8d1 e240    	ldr.w	lr, [r1, #0x240]
 800499e: eb03 0408    	add.w	r4, r3, r8
 80049a2: f8c1 41c8    	str.w	r4, [r1, #0x1c8]
 80049a6: f8d1 9258    	ldr.w	r9, [r1, #0x258]
 80049aa: eba3 2324    	sub.w	r3, r3, r4, asr #8
 80049ae: eb03 0405    	add.w	r4, r3, r5
 80049b2: f8d1 51f8    	ldr.w	r5, [r1, #0x1f8]
 80049b6: f8c1 41e0    	str.w	r4, [r1, #0x1e0]
 80049ba: eba3 2324    	sub.w	r3, r3, r4, asr #8
 80049be: eb03 0405    	add.w	r4, r3, r5
 80049c2: f8c1 41f8    	str.w	r4, [r1, #0x1f8]
 80049c6: f8d1 5270    	ldr.w	r5, [r1, #0x270]
 80049ca: eba3 2324    	sub.w	r3, r3, r4, asr #8
 80049ce: eb03 0406    	add.w	r4, r3, r6
 80049d2: f8c1 4210    	str.w	r4, [r1, #0x210]
 80049d6: eba3 2324    	sub.w	r3, r3, r4, asr #8
 80049da: eb03 040a    	add.w	r4, r3, r10
 80049de: f8c1 4228    	str.w	r4, [r1, #0x228]
 80049e2: eba3 2324    	sub.w	r3, r3, r4, asr #8
 80049e6: eb03 060e    	add.w	r6, r3, lr
 80049ea: f8c1 6240    	str.w	r6, [r1, #0x240]
 80049ee: eba3 2326    	sub.w	r3, r3, r6, asr #8
 80049f2: eb03 0609    	add.w	r6, r3, r9
 80049f6: f8c1 6258    	str.w	r6, [r1, #0x258]
 80049fa: eba3 2326    	sub.w	r3, r3, r6, asr #8
 80049fe: eb03 0605    	add.w	r6, r3, r5
 8004a02: f8d1 5288    	ldr.w	r5, [r1, #0x288]
 8004a06: f8c1 6270    	str.w	r6, [r1, #0x270]
 8004a0a: eba3 2326    	sub.w	r3, r3, r6, asr #8
 8004a0e: eb03 0605    	add.w	r6, r3, r5
 8004a12: f8d1 52a0    	ldr.w	r5, [r1, #0x2a0]
 8004a16: f8c1 6288    	str.w	r6, [r1, #0x288]
 8004a1a: eba3 2326    	sub.w	r3, r3, r6, asr #8
 8004a1e: eb03 0605    	add.w	r6, r3, r5
 8004a22: f8d1 52b8    	ldr.w	r5, [r1, #0x2b8]
 8004a26: f8c1 62a0    	str.w	r6, [r1, #0x2a0]
 8004a2a: eba3 2326    	sub.w	r3, r3, r6, asr #8
 8004a2e: eb03 0605    	add.w	r6, r3, r5
 8004a32: f8d1 52d0    	ldr.w	r5, [r1, #0x2d0]
 8004a36: f8c1 62b8    	str.w	r6, [r1, #0x2b8]
 8004a3a: eba3 2626    	sub.w	r6, r3, r6, asr #8
 8004a3e: f8d1 32e8    	ldr.w	r3, [r1, #0x2e8]
 8004a42: 4435         	add	r5, r6
 8004a44: f8c1 52d0    	str.w	r5, [r1, #0x2d0]
 8004a48: eba6 2625    	sub.w	r6, r6, r5, asr #8
 8004a4c: eb06 0e03    	add.w	lr, r6, r3
 8004a50: f8c1 e2e8    	str.w	lr, [r1, #0x2e8]
 8004a54: f501 71c0    	add.w	r1, r1, #0x180
 8004a58: f47f af7b    	bne.w	0x8004952 <raylar_crcl_encoder_service::benchmark::dcblock::h5d1170af065dd1eb+0x136> @ imm = #-0x10a
 8004a5c: f645 7174    	movw	r1, #0x5f74
 8004a60: f04f 0800    	mov.w	r8, #0x0
 8004a64: 4401         	add	r1, r0
 8004a66: f44f 6280    	mov.w	r2, #0x400
 8004a6a: 2400         	movs	r4, #0x0
 8004a6c: eb04 2428    	add.w	r4, r4, r8, asr #8
 8004a70: f8d1 50c0    	ldr.w	r5, [r1, #0xc0]
 8004a74: eba8 0404    	sub.w	r4, r8, r4
 8004a78: f8d1 60d8    	ldr.w	r6, [r1, #0xd8]
 8004a7c: 4425         	add	r5, r4
 8004a7e: f8c1 50c0    	str.w	r5, [r1, #0xc0]
 8004a82: f8d1 30f0    	ldr.w	r3, [r1, #0xf0]
 8004a86: 3a10         	subs	r2, #0x10
 8004a88: eba4 2425    	sub.w	r4, r4, r5, asr #8
 8004a8c: f8d1 9108    	ldr.w	r9, [r1, #0x108]
 8004a90: eb04 0506    	add.w	r5, r4, r6
 8004a94: f8d1 6120    	ldr.w	r6, [r1, #0x120]
 8004a98: f8c1 50d8    	str.w	r5, [r1, #0xd8]
 8004a9c: eba4 2425    	sub.w	r4, r4, r5, asr #8
 8004aa0: f8d1 5150    	ldr.w	r5, [r1, #0x150]
 8004aa4: 4423         	add	r3, r4
 8004aa6: f8c1 30f0    	str.w	r3, [r1, #0xf0]
 8004aaa: f8d1 b168    	ldr.w	r11, [r1, #0x168]
 8004aae: eba4 2323    	sub.w	r3, r4, r3, asr #8
 8004ab2: f8d1 8180    	ldr.w	r8, [r1, #0x180]
 8004ab6: eb03 0409    	add.w	r4, r3, r9
 8004aba: f8c1 4108    	str.w	r4, [r1, #0x108]
 8004abe: f8d1 a198    	ldr.w	r10, [r1, #0x198]
 8004ac2: eba3 2324    	sub.w	r3, r3, r4, asr #8
 8004ac6: eb03 0406    	add.w	r4, r3, r6
 8004aca: f8d1 6138    	ldr.w	r6, [r1, #0x138]
 8004ace: f8c1 4120    	str.w	r4, [r1, #0x120]
 8004ad2: eba3 2324    	sub.w	r3, r3, r4, asr #8
 8004ad6: eb03 0406    	add.w	r4, r3, r6
 8004ada: f8c1 4138    	str.w	r4, [r1, #0x138]
 8004ade: eba3 2324    	sub.w	r3, r3, r4, asr #8
 8004ae2: eb03 0405    	add.w	r4, r3, r5
 8004ae6: f8c1 4150    	str.w	r4, [r1, #0x150]
 8004aea: f8d1 51b0    	ldr.w	r5, [r1, #0x1b0]
 8004aee: eba3 2324    	sub.w	r3, r3, r4, asr #8
 8004af2: eb03 040b    	add.w	r4, r3, r11
 8004af6: f8c1 4168    	str.w	r4, [r1, #0x168]
 8004afa: eba3 2324    	sub.w	r3, r3, r4, asr #8
 8004afe: eb03 0408    	add.w	r4, r3, r8
 8004b02: f8c1 4180    	str.w	r4, [r1, #0x180]
 8004b06: eba3 2324    	sub.w	r3, r3, r4, asr #8
 8004b0a: eb03 040a    	add.w	r4, r3, r10
 8004b0e: f8c1 4198    	str.w	r4, [r1, #0x198]
 8004b12: eba3 2324    	sub.w	r3, r3, r4, asr #8
 8004b16: eb03 0405    	add.w	r4, r3, r5
 8004b1a: f8d1 51c8    	ldr.w	r5, [r1, #0x1c8]
 8004b1e: f8c1 41b0    	str.w	r4, [r1, #0x1b0]
 8004b22: eba3 2324    	sub.w	r3, r3, r4, asr #8
 8004b26: eb03 0405    	add.w	r4, r3, r5
 8004b2a: f8d1 51e0    	ldr.w	r5, [r1, #0x1e0]
 8004b2e: f8c1 41c8    	str.w	r4, [r1, #0x1c8]
 8004b32: eba3 2324    	sub.w	r3, r3, r4, asr #8
 8004b36: eb03 0405    	add.w	r4, r3, r5
 8004b3a: f8d1 51f8    	ldr.w	r5, [r1, #0x1f8]
 8004b3e: f8c1 41e0    	str.w	r4, [r1, #0x1e0]
 8004b42: eba3 2324    	sub.w	r3, r3, r4, asr #8
 8004b46: eb03 0405    	add.w	r4, r3, r5
 8004b4a: f8d1 5210    	ldr.w	r5, [r1, #0x210]
 8004b4e: f8c1 41f8    	str.w	r4, [r1, #0x1f8]
 8004b52: eba3 2324    	sub.w	r3, r3, r4, asr #8
 8004b56: f8d1 4228    	ldr.w	r4, [r1, #0x228]
 8004b5a: 441d         	add	r5, r3
 8004b5c: f8c1 5210    	str.w	r5, [r1, #0x210]
 8004b60: eba3 2325    	sub.w	r3, r3, r5, asr #8
 8004b64: eb03 0804    	add.w	r8, r3, r4
 8004b68: f8c1 8228    	str.w	r8, [r1, #0x228]
 8004b6c: f501 71c0    	add.w	r1, r1, #0x180
 8004b70: f47f af7c    	bne.w	0x8004a6c <raylar_crcl_encoder_service::benchmark::dcblock::h5d1170af065dd1eb+0x250> @ imm = #-0x108
 8004b74: f645 61b8    	movw	r1, #0x5eb8
 8004b78: 1842         	adds	r2, r0, r1
 8004b7a: f04f 0900    	mov.w	r9, #0x0
 8004b7e: f44f 6c80    	mov.w	r12, #0x400
 8004b82: 2500         	movs	r5, #0x0
 8004b84: eb05 2529    	add.w	r5, r5, r9, asr #8
 8004b88: f8d2 3180    	ldr.w	r3, [r2, #0x180]
 8004b8c: eba9 0505    	sub.w	r5, r9, r5
 8004b90: f8d2 6198    	ldr.w	r6, [r2, #0x198]
 8004b94: 442b         	add	r3, r5
 8004b96: f8c2 3180    	str.w	r3, [r2, #0x180]
 8004b9a: f8d2 11b0    	ldr.w	r1, [r2, #0x1b0]
 8004b9e: f1bc 0c10    	subs.w	r12, r12, #0x10
 8004ba2: eba5 2323    	sub.w	r3, r5, r3, asr #8
 8004ba6: f8d2 a1c8    	ldr.w	r10, [r2, #0x1c8]
 8004baa: eb03 0506    	add.w	r5, r3, r6
 8004bae: f8d2 41e0    	ldr.w	r4, [r2, #0x1e0]
 8004bb2: f8c2 5198    	str.w	r5, [r2, #0x198]
 8004bb6: eba3 2325    	sub.w	r3, r3, r5, asr #8
 8004bba: f8d2 5210    	ldr.w	r5, [r2, #0x210]
 8004bbe: 4419         	add	r1, r3
 8004bc0: f8c2 11b0    	str.w	r1, [r2, #0x1b0]
 8004bc4: f8d2 6228    	ldr.w	r6, [r2, #0x228]
 8004bc8: eba3 2121    	sub.w	r1, r3, r1, asr #8
 8004bcc: f8d2 9240    	ldr.w	r9, [r2, #0x240]
 8004bd0: eb01 030a    	add.w	r3, r1, r10
 8004bd4: f8c2 31c8    	str.w	r3, [r2, #0x1c8]
 8004bd8: f8d2 b258    	ldr.w	r11, [r2, #0x258]
 8004bdc: eba1 2123    	sub.w	r1, r1, r3, asr #8
 8004be0: eb01 0304    	add.w	r3, r1, r4
 8004be4: f8d2 41f8    	ldr.w	r4, [r2, #0x1f8]
 8004be8: f8c2 31e0    	str.w	r3, [r2, #0x1e0]
 8004bec: eba1 2123    	sub.w	r1, r1, r3, asr #8
 8004bf0: eb01 0304    	add.w	r3, r1, r4
 8004bf4: f8c2 31f8    	str.w	r3, [r2, #0x1f8]
 8004bf8: f8d2 4270    	ldr.w	r4, [r2, #0x270]
 8004bfc: eba1 2123    	sub.w	r1, r1, r3, asr #8
 8004c00: eb01 0305    	add.w	r3, r1, r5
 8004c04: f8c2 3210    	str.w	r3, [r2, #0x210]
 8004c08: f8d2 52e8    	ldr.w	r5, [r2, #0x2e8]
 8004c0c: eba1 2123    	sub.w	r1, r1, r3, asr #8
 8004c10: eb01 0306    	add.w	r3, r1, r6
 8004c14: f8c2 3228    	str.w	r3, [r2, #0x228]
 8004c18: eba1 2123    	sub.w	r1, r1, r3, asr #8
 8004c1c: eb01 0309    	add.w	r3, r1, r9
 8004c20: f8c2 3240    	str.w	r3, [r2, #0x240]
 8004c24: eba1 2123    	sub.w	r1, r1, r3, asr #8
 8004c28: eb01 030b    	add.w	r3, r1, r11
 8004c2c: f8c2 3258    	str.w	r3, [r2, #0x258]
 8004c30: eba1 2123    	sub.w	r1, r1, r3, asr #8
 8004c34: eb01 0304    	add.w	r3, r1, r4
 8004c38: f8d2 4288    	ldr.w	r4, [r2, #0x288]
 8004c3c: f8c2 3270    	str.w	r3, [r2, #0x270]
 8004c40: eba1 2123    	sub.w	r1, r1, r3, asr #8
 8004c44: eb01 0304    	add.w	r3, r1, r4
 8004c48: f8d2 42a0    	ldr.w	r4, [r2, #0x2a0]
 8004c4c: f8c2 3288    	str.w	r3, [r2, #0x288]
 8004c50: eba1 2123    	sub.w	r1, r1, r3, asr #8
 8004c54: eb01 0304    	add.w	r3, r1, r4
 8004c58: f8d2 42b8    	ldr.w	r4, [r2, #0x2b8]
 8004c5c: f8c2 32a0    	str.w	r3, [r2, #0x2a0]
 8004c60: eba1 2123    	sub.w	r1, r1, r3, asr #8
 8004c64: eb01 0304    	add.w	r3, r1, r4
 8004c68: f8d2 42d0    	ldr.w	r4, [r2, #0x2d0]
 8004c6c: f8c2 32b8    	str.w	r3, [r2, #0x2b8]
 8004c70: eba1 2123    	sub.w	r1, r1, r3, asr #8
 8004c74: eb01 0304    	add.w	r3, r1, r4
 8004c78: f8c2 32d0    	str.w	r3, [r2, #0x2d0]
 8004c7c: eba1 2123    	sub.w	r1, r1, r3, asr #8
 8004c80: eb01 0905    	add.w	r9, r1, r5
 8004c84: f8c2 92e8    	str.w	r9, [r2, #0x2e8]
 8004c88: f502 72c0    	add.w	r2, r2, #0x180
 8004c8c: f47f af7a    	bne.w	0x8004b84 <raylar_crcl_encoder_service::benchmark::dcblock::h5d1170af065dd1eb+0x368> @ imm = #-0x10c
 8004c90: f645 717c    	movw	r1, #0x5f7c
 8004c94: 1842         	adds	r2, r0, r1
 8004c96: 2500         	movs	r5, #0x0
 8004c98: f44f 6480    	mov.w	r4, #0x400
 8004c9c: 2600         	movs	r6, #0x0
 8004c9e: eb06 2625    	add.w	r6, r6, r5, asr #8
 8004ca2: f8d2 10c0    	ldr.w	r1, [r2, #0xc0]
 8004ca6: 1bae         	subs	r6, r5, r6
 8004ca8: f8d2 30d8    	ldr.w	r3, [r2, #0xd8]
 8004cac: 4431         	add	r1, r6
 8004cae: f8c2 10c0    	str.w	r1, [r2, #0xc0]
 8004cb2: f8d2 00f0    	ldr.w	r0, [r2, #0xf0]
 8004cb6: 3c10         	subs	r4, #0x10
 8004cb8: eba6 2121    	sub.w	r1, r6, r1, asr #8
 8004cbc: f8d2 c108    	ldr.w	r12, [r2, #0x108]
 8004cc0: 440b         	add	r3, r1
 8004cc2: f8d2 5120    	ldr.w	r5, [r2, #0x120]
 8004cc6: f8c2 30d8    	str.w	r3, [r2, #0xd8]
 8004cca: eba1 2123    	sub.w	r1, r1, r3, asr #8
 8004cce: f8d2 3150    	ldr.w	r3, [r2, #0x150]
 8004cd2: 4408         	add	r0, r1
 8004cd4: f8c2 00f0    	str.w	r0, [r2, #0xf0]
 8004cd8: f8d2 6168    	ldr.w	r6, [r2, #0x168]
 8004cdc: eba1 2020    	sub.w	r0, r1, r0, asr #8
 8004ce0: f8d2 b180    	ldr.w	r11, [r2, #0x180]
 8004ce4: eb00 010c    	add.w	r1, r0, r12
 8004ce8: f8c2 1108    	str.w	r1, [r2, #0x108]
 8004cec: f8d2 a198    	ldr.w	r10, [r2, #0x198]
 8004cf0: eba0 2021    	sub.w	r0, r0, r1, asr #8
 8004cf4: eb00 0105    	add.w	r1, r0, r5
 8004cf8: f8d2 5138    	ldr.w	r5, [r2, #0x138]
 8004cfc: f8c2 1120    	str.w	r1, [r2, #0x120]
 8004d00: eba0 2021    	sub.w	r0, r0, r1, asr #8
 8004d04: eb00 0105    	add.w	r1, r0, r5
 8004d08: f8c2 1138    	str.w	r1, [r2, #0x138]
 8004d0c: eba0 2021    	sub.w	r0, r0, r1, asr #8
 8004d10: eb00 0103    	add.w	r1, r0, r3
 8004d14: f8c2 1150    	str.w	r1, [r2, #0x150]
 8004d18: f8d2 31b0    	ldr.w	r3, [r2, #0x1b0]
 8004d1c: eba0 2021    	sub.w	r0, r0, r1, asr #8
 8004d20: eb00 0106    	add.w	r1, r0, r6
 8004d24: f8c2 1168    	str.w	r1, [r2, #0x168]
 8004d28: f8d2 6228    	ldr.w	r6, [r2, #0x228]
 8004d2c: eba0 2021    	sub.w	r0, r0, r1, asr #8
 8004d30: eb00 010b    	add.w	r1, r0, r11
 8004d34: f8c2 1180    	str.w	r1, [r2, #0x180]
 8004d38: eba0 2021    	sub.w	r0, r0, r1, asr #8
 8004d3c: eb00 010a    	add.w	r1, r0, r10
 8004d40: f8c2 1198    	str.w	r1, [r2, #0x198]
 8004d44: eba0 2021    	sub.w	r0, r0, r1, asr #8
 8004d48: eb00 0103    	add.w	r1, r0, r3
 8004d4c: f8d2 31c8    	ldr.w	r3, [r2, #0x1c8]
 8004d50: f8c2 11b0    	str.w	r1, [r2, #0x1b0]
 8004d54: eba0 2021    	sub.w	r0, r0, r1, asr #8
 8004d58: eb00 0103    	add.w	r1, r0, r3
 8004d5c: f8d2 31e0    	ldr.w	r3, [r2, #0x1e0]
 8004d60: f8c2 11c8    	str.w	r1, [r2, #0x1c8]
 8004d64: eba0 2021    	sub.w	r0, r0, r1, asr #8
 8004d68: eb00 0103    	add.w	r1, r0, r3
 8004d6c: f8d2 31f8    	ldr.w	r3, [r2, #0x1f8]
 8004d70: f8c2 11e0    	str.w	r1, [r2, #0x1e0]
 8004d74: eba0 2021    	sub.w	r0, r0, r1, asr #8
 8004d78: eb00 0103    	add.w	r1, r0, r3
 8004d7c: f8d2 3210    	ldr.w	r3, [r2, #0x210]
 8004d80: f8c2 11f8    	str.w	r1, [r2, #0x1f8]
 8004d84: eba0 2021    	sub.w	r0, r0, r1, asr #8
 8004d88: eb00 0103    	add.w	r1, r0, r3
 8004d8c: f8c2 1210    	str.w	r1, [r2, #0x210]
 8004d90: eba0 2021    	sub.w	r0, r0, r1, asr #8
 8004d94: eb00 0506    	add.w	r5, r0, r6
 8004d98: f8c2 5228    	str.w	r5, [r2, #0x228]
 8004d9c: f502 72c0    	add.w	r2, r2, #0x180
 8004da0: f47f af7d    	bne.w	0x8004c9e <raylar_crcl_encoder_service::benchmark::dcblock::h5d1170af065dd1eb+0x482> @ imm = #-0x106
 8004da4: 9901         	ldr	r1, [sp, #0x4]
 8004da6: f246 0028    	movw	r0, #0x6028
 8004daa: 4408         	add	r0, r1
 8004dac: 9002         	str	r0, [sp, #0x8]
 8004dae: a802         	add	r0, sp, #0x8
 8004db0: 9800         	ldr	r0, [sp]
 8004db2: ea80 000e    	eor.w	r0, r0, lr
 8004db6: ea80 0008    	eor.w	r0, r0, r8
 8004dba: ea80 0009    	eor.w	r0, r0, r9
 8004dbe: 4068         	eors	r0, r5
 8004dc0: b003         	add	sp, #0xc
 8004dc2: e8bd 0f00    	pop.w	{r8, r9, r10, r11}
 8004dc6: bdf0         	pop	{r4, r5, r6, r7, pc}

08004dc8 <raylar_crcl_encoder_service::benchmark::measure::he296905d366d7d56>:
 8004dc8: b5f0         	push	{r4, r5, r6, r7, lr}
 8004dca: af03         	add	r7, sp, #0xc
 8004dcc: e92d 0f00    	push.w	{r8, r9, r10, r11}
 8004dd0: f5ad 7d05    	sub.w	sp, sp, #0x214
 8004dd4: 9001         	str	r0, [sp, #0x4]
 8004dd6: a802         	add	r0, sp, #0x8
 8004dd8: 460d         	mov	r5, r1
 8004dda: f44f 7101    	mov.w	r1, #0x204
 8004dde: 9300         	str	r3, [sp]
 8004de0: 4616         	mov	r6, r2
 8004de2: f007 fe84    	bl	0x800caee <__aeabi_memclr4> @ imm = #0x7d08
 8004de6: f50d 7a03    	add.w	r10, sp, #0x20c
 8004dea: f04f 0b00    	mov.w	r11, #0x0
 8004dee: f04f 0800    	mov.w	r8, #0x0
 8004df2: e004         	b	0x8004dfe <raylar_crcl_encoder_service::benchmark::measure::he296905d366d7d56+0x36> @ imm = #0x8
 8004df4: f108 0801    	add.w	r8, r8, #0x1
 8004df8: f1b8 0f89    	cmp.w	r8, #0x89
 8004dfc: d018         	beq	0x8004e30 <raylar_crcl_encoder_service::benchmark::measure::he296905d366d7d56+0x68> @ imm = #0x30
 8004dfe: 6939         	ldr	r1, [r7, #0x10]
 8004e00: 4628         	mov	r0, r5
 8004e02: 4788         	blx	r1
 8004e04: 47b0         	blx	r6
 8004e06: 9583         	str	r5, [sp, #0x20c]
 8004e08: 4604         	mov	r4, r0
 8004e0a: 6979         	ldr	r1, [r7, #0x14]
 8004e0c: 9883         	ldr	r0, [sp, #0x20c]
 8004e0e: 4788         	blx	r1
 8004e10: 9083         	str	r0, [sp, #0x20c]
 8004e12: f8dd 920c    	ldr.w	r9, [sp, #0x20c]
 8004e16: 47b0         	blx	r6
 8004e18: f1b8 0f07    	cmp.w	r8, #0x7
 8004e1c: d9ea         	bls	0x8004df4 <raylar_crcl_encoder_service::benchmark::measure::he296905d366d7d56+0x2c> @ imm = #-0x2c
 8004e1e: a902         	add	r1, sp, #0x8
 8004e20: 1b00         	subs	r0, r0, r4
 8004e22: eb01 0188    	add.w	r1, r1, r8, lsl #2
 8004e26: ea8b 0b09    	eor.w	r11, r11, r9
 8004e2a: f841 0c20    	str	r0, [r1, #-32]
 8004e2e: e7e1         	b	0x8004df4 <raylar_crcl_encoder_service::benchmark::measure::he296905d366d7d56+0x2c> @ imm = #-0x3e
 8004e30: a802         	add	r0, sp, #0x8
 8004e32: f1a7 021d    	sub.w	r2, r7, #0x1d
 8004e36: 2181         	movs	r1, #0x81
 8004e38: f002 fe13    	bl	0x8007a62 <core::slice::sort::unstable::ipnsort::hd8a371dd4e708cab> @ imm = #0x2c26
 8004e3c: e9dd 5600    	ldrd	r5, r6, [sp]
 8004e40: 9802         	ldr	r0, [sp, #0x8]
 8004e42: 6035         	str	r5, [r6]
 8004e44: 68bd         	ldr	r5, [r7, #0x8]
 8004e46: 6075         	str	r5, [r6, #0x4]
 8004e48: 68fd         	ldr	r5, [r7, #0xc]
 8004e4a: 9942         	ldr	r1, [sp, #0x108]
 8004e4c: e9c6 5002    	strd	r5, r0, [r6, #8]
 8004e50: f106 0010    	add.w	r0, r6, #0x10
 8004e54: 9a75         	ldr	r2, [sp, #0x1d4]
 8004e56: 9b82         	ldr	r3, [sp, #0x208]
 8004e58: e880 080e    	stm.w	r0, {r1, r2, r3, r11}
 8004e5c: f50d 7d05    	add.w	sp, sp, #0x214
 8004e60: e8bd 0f00    	pop.w	{r8, r9, r10, r11}
 8004e64: bdf0         	pop	{r4, r5, r6, r7, pc}

08004e66 <raylar_crcl_encoder_service::benchmark::prepare::hb9734b8d5dc7f78e>:
 8004e66: b5f0         	push	{r4, r5, r6, r7, lr}
 8004e68: af03         	add	r7, sp, #0xc
 8004e6a: e92d 0f00    	push.w	{r8, r9, r10, r11}
 8004e6e: b0f9         	sub	sp, #0x1e4
 8004e70: 4601         	mov	r1, r0
 8004e72: f500 30ee    	add.w	r0, r0, #0x1dc00
 8004e76: f500 701f    	add.w	r0, r0, #0x27c
 8004e7a: 9004         	str	r0, [sp, #0x10]
 8004e7c: f501 30f8    	add.w	r0, r1, #0x1f000
 8004e80: f245 6578    	movw	r5, #0x5678
 8004e84: f500 604b    	add.w	r0, r0, #0xcb0
 8004e88: f64d 68a0    	movw	r8, #0xdea0
 8004e8c: 9005         	str	r0, [sp, #0x14]
 8004e8e: f101 0028    	add.w	r0, r1, #0x28
 8004e92: 9003         	str	r0, [sp, #0xc]
 8004e94: 2000         	movs	r0, #0x0
 8004e96: f2c1 2534    	movt	r5, #0x1234
 8004e9a: f06f 0917    	mvn	r9, #0x17
 8004e9e: f6c0 0800    	movt	r8, #0x800
 8004ea2: f644 6a20    	movw	r10, #0x4e20
 8004ea6: f645 5bc0    	movw	r11, #0x5dc0
 8004eaa: f645 14d8    	movw	r4, #0x59d8
 8004eae: 9124         	str	r1, [sp, #0x90]
 8004eb0: f000 031f    	and	r3, r0, #0x1f
 8004eb4: f100 060a    	add.w	r6, r0, #0xa
 8004eb8: f006 061f    	and	r6, r6, #0x1f
 8004ebc: f100 0c02    	add.w	r12, r0, #0x2
 8004ec0: f858 3023    	ldr.w	r3, [r8, r3, lsl #2]
 8004ec4: ea85 3545    	eor.w	r5, r5, r5, lsl #13
 8004ec8: f858 6026    	ldr.w	r6, [r8, r6, lsl #2]
 8004ecc: f246 11a8    	movw	r1, #0x61a8
 8004ed0: ea85 4555    	eor.w	r5, r5, r5, lsr #17
 8004ed4: fb03 fe0a    	mul	lr, r3, r10
 8004ed8: f00c 031f    	and	r3, r12, #0x1f
 8004edc: ea85 1545    	eor.w	r5, r5, r5, lsl #5
 8004ee0: f858 2023    	ldr.w	r2, [r8, r3, lsl #2]
 8004ee4: fb06 fc01    	mul	r12, r6, r1
 8004ee8: f245 2108    	movw	r1, #0x5208
 8004eec: b2eb         	uxtb	r3, r5
 8004eee: 434a         	muls	r2, r1, r2
 8004ef0: ea4f 76ee    	asr.w	r6, lr, #0x1f
 8004ef4: eb0e 4656    	add.w	r6, lr, r6, lsr #17
 8004ef8: 9924         	ldr	r1, [sp, #0x90]
 8004efa: eb03 33e6    	add.w	r3, r3, r6, asr #15
 8004efe: eb01 0609    	add.w	r6, r1, r9
 8004f02: 3b80         	subs	r3, #0x80
 8004f04: f245 51f0    	movw	r1, #0x55f0
 8004f08: f109 0918    	add.w	r9, r9, #0x18
 8004f0c: 6433         	str	r3, [r6, #0x40]
 8004f0e: 17d3         	asrs	r3, r2, #0x1f
 8004f10: eb02 4253    	add.w	r2, r2, r3, lsr #17
 8004f14: ea85 3345    	eor.w	r3, r5, r5, lsl #13
 8004f18: ea83 4353    	eor.w	r3, r3, r3, lsr #17
 8004f1c: ea83 1343    	eor.w	r3, r3, r3, lsl #5
 8004f20: b2dd         	uxtb	r5, r3
 8004f22: eb05 32e2    	add.w	r2, r5, r2, asr #15
 8004f26: 1d05         	adds	r5, r0, #0x4
 8004f28: ea83 3343    	eor.w	r3, r3, r3, lsl #13
 8004f2c: f005 051f    	and	r5, r5, #0x1f
 8004f30: 3a80         	subs	r2, #0x80
 8004f32: ea83 4353    	eor.w	r3, r3, r3, lsr #17
 8004f36: 6472         	str	r2, [r6, #0x44]
 8004f38: f858 5025    	ldr.w	r5, [r8, r5, lsl #2]
 8004f3c: ea83 1343    	eor.w	r3, r3, r3, lsl #5
 8004f40: 434d         	muls	r5, r1, r5
 8004f42: 1d81         	adds	r1, r0, #0x6
 8004f44: f001 011f    	and	r1, r1, #0x1f
 8004f48: f858 1021    	ldr.w	r1, [r8, r1, lsl #2]
 8004f4c: 17ea         	asrs	r2, r5, #0x1f
 8004f4e: 4361         	muls	r1, r4, r1
 8004f50: eb05 4252    	add.w	r2, r5, r2, lsr #17
 8004f54: b2dd         	uxtb	r5, r3
 8004f56: eb05 32e2    	add.w	r2, r5, r2, asr #15
 8004f5a: f100 0508    	add.w	r5, r0, #0x8
 8004f5e: f005 051f    	and	r5, r5, #0x1f
 8004f62: 3a80         	subs	r2, #0x80
 8004f64: 64b2         	str	r2, [r6, #0x48]
 8004f66: 3001         	adds	r0, #0x1
 8004f68: f858 5025    	ldr.w	r5, [r8, r5, lsl #2]
 8004f6c: f5b0 6f80    	cmp.w	r0, #0x400
 8004f70: ea4f 72e1    	asr.w	r2, r1, #0x1f
 8004f74: eb01 4152    	add.w	r1, r1, r2, lsr #17
 8004f78: ea83 3243    	eor.w	r2, r3, r3, lsl #13
 8004f7c: ea82 4252    	eor.w	r2, r2, r2, lsr #17
 8004f80: fb05 f50b    	mul	r5, r5, r11
 8004f84: ea82 1242    	eor.w	r2, r2, r2, lsl #5
 8004f88: b2d3         	uxtb	r3, r2
 8004f8a: ea82 3242    	eor.w	r2, r2, r2, lsl #13
 8004f8e: eb03 31e1    	add.w	r1, r3, r1, asr #15
 8004f92: ea82 4252    	eor.w	r2, r2, r2, lsr #17
 8004f96: f1a1 0180    	sub.w	r1, r1, #0x80
 8004f9a: 64f1         	str	r1, [r6, #0x4c]
 8004f9c: ea4f 71e5    	asr.w	r1, r5, #0x1f
 8004fa0: ea82 1242    	eor.w	r2, r2, r2, lsl #5
 8004fa4: eb05 4151    	add.w	r1, r5, r1, lsr #17
 8004fa8: b2d3         	uxtb	r3, r2
 8004faa: ea82 3242    	eor.w	r2, r2, r2, lsl #13
 8004fae: eb03 31e1    	add.w	r1, r3, r1, asr #15
 8004fb2: f1a1 0180    	sub.w	r1, r1, #0x80
 8004fb6: ea82 4252    	eor.w	r2, r2, r2, lsr #17
 8004fba: 6531         	str	r1, [r6, #0x50]
 8004fbc: ea4f 71ec    	asr.w	r1, r12, #0x1f
 8004fc0: ea82 1542    	eor.w	r5, r2, r2, lsl #5
 8004fc4: eb0c 4151    	add.w	r1, r12, r1, lsr #17
 8004fc8: b2ea         	uxtb	r2, r5
 8004fca: eb02 31e1    	add.w	r1, r2, r1, asr #15
 8004fce: f1a1 0180    	sub.w	r1, r1, #0x80
 8004fd2: 6571         	str	r1, [r6, #0x54]
 8004fd4: f47f af6c    	bne.w	0x8004eb0 <raylar_crcl_encoder_service::benchmark::prepare::hb9734b8d5dc7f78e+0x4a> @ imm = #-0x128
 8004fd8: 9924         	ldr	r1, [sp, #0x90]
 8004fda: f246 0028    	movw	r0, #0x6028
 8004fde: f44f 42c0    	mov.w	r2, #0x6000
 8004fe2: eb01 0900    	add.w	r9, r1, r0
 8004fe6: 3128         	adds	r1, #0x28
 8004fe8: 4648         	mov	r0, r9
 8004fea: f007 fd32    	bl	0x800ca52 <__aeabi_memcpy4> @ imm = #0x7a64
 8004fee: ad25         	add	r5, sp, #0x94
 8004ff0: 2128         	movs	r1, #0x28
 8004ff2: f105 0848    	add.w	r8, r5, #0x48
 8004ff6: 4640         	mov	r0, r8
 8004ff8: f007 fd79    	bl	0x800caee <__aeabi_memclr4> @ imm = #0x7af2
 8004ffc: 2101         	movs	r1, #0x1
 8004ffe: 2000         	movs	r0, #0x0
 8005000: 9141         	str	r1, [sp, #0x104]
 8005002: f44f 51c0    	mov.w	r1, #0x1800
 8005006: e9cd 9135    	strd	r9, r1, [sp, #212]
 800500a: f44f 6180    	mov.w	r1, #0x400
 800500e: 9146         	str	r1, [sp, #0x118]
 8005010: f44f 0100    	mov.w	r1, #0x800000
 8005014: 9147         	str	r1, [sp, #0x11c]
 8005016: f240 610a    	movw	r1, #0x60a
 800501a: f2c0 1101    	movt	r1, #0x101
 800501e: e9cd 0044    	strd	r0, r0, [sp, #272]
 8005022: e9cd 0042    	strd	r0, r0, [sp, #264]
 8005026: e9cd 0025    	strd	r0, r0, [sp, #148]
 800502a: e9cd 0027    	strd	r0, r0, [sp, #156]
 800502e: e9cd 0029    	strd	r0, r0, [sp, #164]
 8005032: 902b         	str	r0, [sp, #0xac]
 8005034: f8ad 00b0    	strh.w	r0, [sp, #0xb0]
 8005038: e9cd 002d    	strd	r0, r0, [sp, #180]
 800503c: e9cd 002f    	strd	r0, r0, [sp, #188]
 8005040: e9cd 0031    	strd	r0, r0, [sp, #196]
 8005044: 9033         	str	r0, [sp, #0xcc]
 8005046: f8ad 00d0    	strh.w	r0, [sp, #0xd0]
 800504a: 9148         	str	r1, [sp, #0x120]
 800504c: f8ad 0124    	strh.w	r0, [sp, #0x124]
 8005050: 4628         	mov	r0, r5
 8005052: f04f 31ff    	mov.w	r1, #0xffffffff
 8005056: f7fc fc02    	bl	0x800185e <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3> @ imm = #-0x37fc
 800505a: 07c0         	lsls	r0, r0, #0x1f
 800505c: f041 804d    	bne.w	0x80060fa <raylar_crcl_encoder_service::benchmark::prepare::hb9734b8d5dc7f78e+0x1294> @ imm = #0x109a
 8005060: 07c8         	lsls	r0, r1, #0x1f
 8005062: d0f5         	beq	0x8005050 <raylar_crcl_encoder_service::benchmark::prepare::hb9734b8d5dc7f78e+0x1ea> @ imm = #-0x16
 8005064: 9c24         	ldr	r4, [sp, #0x90]
 8005066: e8b8 006e    	ldm.w	r8!, {r1, r2, r3, r5, r6}
 800506a: 4620         	mov	r0, r4
 800506c: c06e         	stm	r0!, {r1, r2, r3, r5, r6}
 800506e: e898 006e    	ldm.w	r8, {r1, r2, r3, r5, r6}
 8005072: c06e         	stm	r0!, {r1, r2, r3, r5, r6}
 8005074: f640 2100    	movw	r1, #0xa00
 8005078: f2c0 1106    	movt	r1, #0x106
 800507c: f8cd 1166    	str.w	r1, [sp, #0x166]
 8005080: f44f 0100    	mov.w	r1, #0x800000
 8005084: 2000         	movs	r0, #0x0
 8005086: f8cd 1163    	str.w	r1, [sp, #0x163]
 800508a: f44f 6180    	mov.w	r1, #0x400
 800508e: ad4c         	add	r5, sp, #0x130
 8005090: f8cd 115f    	str.w	r1, [sp, #0x15f]
 8005094: a954         	add	r1, sp, #0x150
 8005096: 9056         	str	r0, [sp, #0x158]
 8005098: 221a         	movs	r2, #0x1a
 800509a: f88d 0130    	strb.w	r0, [sp, #0x130]
 800509e: e9cd 0054    	strd	r0, r0, [sp, #336]
 80050a2: f8cd 015b    	str.w	r0, [sp, #0x15b]
 80050a6: 1c68         	adds	r0, r5, #0x1
 80050a8: f007 faf8    	bl	0x800c69c <__aeabi_memcpy> @ imm = #0x75f0
 80050ac: ae54         	add	r6, sp, #0x150
 80050ae: 2001         	movs	r0, #0x1
 80050b0: a925         	add	r1, sp, #0x94
 80050b2: f88d 014b    	strb.w	r0, [sp, #0x14b]
 80050b6: 4630         	mov	r0, r6
 80050b8: 2294         	movs	r2, #0x94
 80050ba: f007 fcca    	bl	0x800ca52 <__aeabi_memcpy4> @ imm = #0x7994
 80050be: f641 6058    	movw	r0, #0x1e58
 80050c2: 4629         	mov	r1, r5
 80050c4: 9000         	str	r0, [sp]
 80050c6: f504 30ec    	add.w	r0, r4, #0x1d800
 80050ca: f500 63cb    	add.w	r3, r0, #0x658
 80050ce: a84a         	add	r0, sp, #0x128
 80050d0: 4632         	mov	r2, r6
 80050d2: f7fc f90d    	bl	0x80012f0 <raylar_crcl_encoder_service::format::Encoder::data::ha65fe528befbf4e3> @ imm = #-0x3de6
 80050d6: f89d 0128    	ldrb.w	r0, [sp, #0x128]
 80050da: 2801         	cmp	r0, #0x1
 80050dc: f001 8034    	beq.w	0x8006148 <raylar_crcl_encoder_service::benchmark::prepare::hb9734b8d5dc7f78e+0x12e2> @ imm = #0x1068
 80050e0: 9905         	ldr	r1, [sp, #0x14]
 80050e2: f44f 42c0    	mov.w	r2, #0x6000
 80050e6: 984b         	ldr	r0, [sp, #0x12c]
 80050e8: 6288         	str	r0, [r1, #0x28]
 80050ea: f24c 0028    	movw	r0, #0xc028
 80050ee: 9924         	ldr	r1, [sp, #0x90]
 80050f0: 4408         	add	r0, r1
 80050f2: 4649         	mov	r1, r9
 80050f4: 9002         	str	r0, [sp, #0x8]
 80050f6: f007 fcac    	bl	0x800ca52 <__aeabi_memcpy4> @ imm = #0x7958
 80050fa: f641 71fc    	movw	r1, #0x1ffc
 80050fe: f247 0910    	movw	r9, #0x7010
 8005102: f2c0 0101    	movt	r1, #0x1
 8005106: f101 0020    	add.w	r0, r1, #0x20
 800510a: 9018         	str	r0, [sp, #0x60]
 800510c: f101 001c    	add.w	r0, r1, #0x1c
 8005110: 9020         	str	r0, [sp, #0x80]
 8005112: 2000         	movs	r0, #0x0
 8005114: 9019         	str	r0, [sp, #0x64]
 8005116: f64b 70fc    	movw	r0, #0xbffc
 800511a: 9017         	str	r0, [sp, #0x5c]
 800511c: f44f 3090    	mov.w	r0, #0x12000
 8005120: 9023         	str	r0, [sp, #0x8c]
 8005122: f64b 70e4    	movw	r0, #0xbfe4
 8005126: 9016         	str	r0, [sp, #0x58]
 8005128: f64b 706c    	movw	r0, #0xbf6c
 800512c: 9022         	str	r0, [sp, #0x88]
 800512e: f64b 70cc    	movw	r0, #0xbfcc
 8005132: 9021         	str	r0, [sp, #0x84]
 8005134: f64b 70b4    	movw	r0, #0xbfb4
 8005138: 901f         	str	r0, [sp, #0x7c]
 800513a: f64b 7084    	movw	r0, #0xbf84
 800513e: 901e         	str	r0, [sp, #0x78]
 8005140: f64b 709c    	movw	r0, #0xbf9c
 8005144: 901d         	str	r0, [sp, #0x74]
 8005146: f101 0014    	add.w	r0, r1, #0x14
 800514a: 901c         	str	r0, [sp, #0x70]
 800514c: f101 0010    	add.w	r0, r1, #0x10
 8005150: 901b         	str	r0, [sp, #0x6c]
 8005152: f101 000c    	add.w	r0, r1, #0xc
 8005156: 901a         	str	r0, [sp, #0x68]
 8005158: f24f 0004    	movw	r0, #0xf004
 800515c: f101 0c28    	add.w	r12, r1, #0x28
 8005160: f101 0624    	add.w	r6, r1, #0x24
 8005164: f101 0318    	add.w	r3, r1, #0x18
 8005168: f2c0 0901    	movt	r9, #0x1
 800516c: f64b 7224    	movw	r2, #0xbf24
 8005170: f64b 7a54    	movw	r10, #0xbf54
 8005174: f6cf 70ff    	movt	r0, #0xffff
 8005178: 9c24         	ldr	r4, [sp, #0x90]
 800517a: 4698         	mov	r8, r3
 800517c: e9cd 2912    	strd	r2, r9, [sp, #72]
 8005180: eb04 0e02    	add.w	lr, r4, r2
 8005184: e9cd 3114    	strd	r3, r1, [sp, #80]
 8005188: 1863         	adds	r3, r4, r1
 800518a: 44a1         	add	r9, r4
 800518c: f8de 2108    	ldr.w	r2, [lr, #0x108]
 8005190: f8de 5120    	ldr.w	r5, [lr, #0x120]
 8005194: e9cd 0a10    	strd	r0, r10, [sp, #64]
 8005198: 1aaa         	subs	r2, r5, r2
 800519a: 62da         	str	r2, [r3, #0x2c]
 800519c: f8dd b05c    	ldr.w	r11, [sp, #0x5c]
 80051a0: 0053         	lsls	r3, r2, #0x1
 80051a2: ea83 71e2    	eor.w	r1, r3, r2, asr #31
 80051a6: eb04 030a    	add.w	r3, r4, r10
 80051aa: f8c9 1004    	str.w	r1, [r9, #0x4]
 80051ae: 910f         	str	r1, [sp, #0x3c]
 80051b0: f8de 2120    	ldr.w	r2, [lr, #0x120]
 80051b4: f8d3 5108    	ldr.w	r5, [r3, #0x108]
 80051b8: 9923         	ldr	r1, [sp, #0x8c]
 80051ba: 1aaa         	subs	r2, r5, r2
 80051bc: f8dd a058    	ldr.w	r10, [sp, #0x58]
 80051c0: 1865         	adds	r5, r4, r1
 80051c2: 62ea         	str	r2, [r5, #0x2c]
 80051c4: 0055         	lsls	r5, r2, #0x1
 80051c6: ea85 71e2    	eor.w	r1, r5, r2, asr #31
 80051ca: f8c9 1008    	str.w	r1, [r9, #0x8]
 80051ce: 910e         	str	r1, [sp, #0x38]
 80051d0: 9922         	ldr	r1, [sp, #0x88]
 80051d2: f8d3 2108    	ldr.w	r2, [r3, #0x108]
 80051d6: 1863         	adds	r3, r4, r1
 80051d8: f8d3 5108    	ldr.w	r5, [r3, #0x108]
 80051dc: 1aaa         	subs	r2, r5, r2
 80051de: 1825         	adds	r5, r4, r0
 80051e0: f243 002c    	movw	r0, #0x302c
 80051e4: f2c0 0001    	movt	r0, #0x1
 80051e8: 502a         	str	r2, [r5, r0]
 80051ea: 0055         	lsls	r5, r2, #0x1
 80051ec: ea85 70e2    	eor.w	r0, r5, r2, asr #31
 80051f0: f8c9 000c    	str.w	r0, [r9, #0xc]
 80051f4: 4665         	mov	r5, r12
 80051f6: 900d         	str	r0, [sp, #0x34]
 80051f8: 981e         	ldr	r0, [sp, #0x78]
 80051fa: f8d3 2108    	ldr.w	r2, [r3, #0x108]
 80051fe: 1823         	adds	r3, r4, r0
 8005200: 981a         	ldr	r0, [sp, #0x68]
 8005202: f8d3 1108    	ldr.w	r1, [r3, #0x108]
 8005206: 1a89         	subs	r1, r1, r2
 8005208: 1822         	adds	r2, r4, r0
 800520a: 62d1         	str	r1, [r2, #0x2c]
 800520c: 004a         	lsls	r2, r1, #0x1
 800520e: ea82 70e1    	eor.w	r0, r2, r1, asr #31
 8005212: f8c9 0010    	str.w	r0, [r9, #0x10]
 8005216: 900c         	str	r0, [sp, #0x30]
 8005218: 981d         	ldr	r0, [sp, #0x74]
 800521a: f8d3 1108    	ldr.w	r1, [r3, #0x108]
 800521e: 1822         	adds	r2, r4, r0
 8005220: 981b         	ldr	r0, [sp, #0x6c]
 8005222: f8d2 3108    	ldr.w	r3, [r2, #0x108]
 8005226: 1a59         	subs	r1, r3, r1
 8005228: 1823         	adds	r3, r4, r0
 800522a: 62d9         	str	r1, [r3, #0x2c]
 800522c: 004b         	lsls	r3, r1, #0x1
 800522e: ea83 70e1    	eor.w	r0, r3, r1, asr #31
 8005232: f8c9 0014    	str.w	r0, [r9, #0x14]
 8005236: 900b         	str	r0, [sp, #0x2c]
 8005238: 981f         	ldr	r0, [sp, #0x7c]
 800523a: f8d2 1108    	ldr.w	r1, [r2, #0x108]
 800523e: 1822         	adds	r2, r4, r0
 8005240: 981c         	ldr	r0, [sp, #0x70]
 8005242: f8d2 3108    	ldr.w	r3, [r2, #0x108]
 8005246: 1a59         	subs	r1, r3, r1
 8005248: 1823         	adds	r3, r4, r0
 800524a: 62d9         	str	r1, [r3, #0x2c]
 800524c: 004b         	lsls	r3, r1, #0x1
 800524e: ea83 70e1    	eor.w	r0, r3, r1, asr #31
 8005252: f8c9 0018    	str.w	r0, [r9, #0x18]
 8005256: 900a         	str	r0, [sp, #0x28]
 8005258: 9821         	ldr	r0, [sp, #0x84]
 800525a: f8d2 1108    	ldr.w	r1, [r2, #0x108]
 800525e: 1822         	adds	r2, r4, r0
 8005260: f8d2 3108    	ldr.w	r3, [r2, #0x108]
 8005264: 1a59         	subs	r1, r3, r1
 8005266: eb04 0308    	add.w	r3, r4, r8
 800526a: f8dd 8060    	ldr.w	r8, [sp, #0x60]
 800526e: 62d9         	str	r1, [r3, #0x2c]
 8005270: 004b         	lsls	r3, r1, #0x1
 8005272: ea83 70e1    	eor.w	r0, r3, r1, asr #31
 8005276: eb04 030a    	add.w	r3, r4, r10
 800527a: f8c9 001c    	str.w	r0, [r9, #0x1c]
 800527e: f50a 7a84    	add.w	r10, r10, #0x108
 8005282: 9009         	str	r0, [sp, #0x24]
 8005284: f8d2 1108    	ldr.w	r1, [r2, #0x108]
 8005288: f8d3 2108    	ldr.w	r2, [r3, #0x108]
 800528c: 9820         	ldr	r0, [sp, #0x80]
 800528e: 1a51         	subs	r1, r2, r1
 8005290: f8cd a058    	str.w	r10, [sp, #0x58]
 8005294: 1822         	adds	r2, r4, r0
 8005296: f8dd a044    	ldr.w	r10, [sp, #0x44]
 800529a: 62d1         	str	r1, [r2, #0x2c]
 800529c: 004a         	lsls	r2, r1, #0x1
 800529e: ea82 72e1    	eor.w	r2, r2, r1, asr #31
 80052a2: f8c9 2020    	str.w	r2, [r9, #0x20]
 80052a6: f8d3 1108    	ldr.w	r1, [r3, #0x108]
 80052aa: eb04 030b    	add.w	r3, r4, r11
 80052ae: f50a 7a84    	add.w	r10, r10, #0x108
 80052b2: f50b 7b84    	add.w	r11, r11, #0x108
 80052b6: f8d3 c108    	ldr.w	r12, [r3, #0x108]
 80052ba: f8cd b05c    	str.w	r11, [sp, #0x5c]
 80052be: ebac 0101    	sub.w	r1, r12, r1
 80052c2: 46ac         	mov	r12, r5
 80052c4: eb04 0508    	add.w	r5, r4, r8
 80052c8: f108 082c    	add.w	r8, r8, #0x2c
 80052cc: f8cd 8060    	str.w	r8, [sp, #0x60]
 80052d0: 62e9         	str	r1, [r5, #0x2c]
 80052d2: 004d         	lsls	r5, r1, #0x1
 80052d4: ea85 71e1    	eor.w	r1, r5, r1, asr #31
 80052d8: f8c9 1024    	str.w	r1, [r9, #0x24]
 80052dc: f8d3 3108    	ldr.w	r3, [r3, #0x108]
 80052e0: f8de 51f8    	ldr.w	r5, [lr, #0x1f8]
 80052e4: 1aeb         	subs	r3, r5, r3
 80052e6: 19a5         	adds	r5, r4, r6
 80052e8: 62eb         	str	r3, [r5, #0x2c]
 80052ea: 005d         	lsls	r5, r3, #0x1
 80052ec: ea85 73e3    	eor.w	r3, r5, r3, asr #31
 80052f0: f8c9 3028    	str.w	r3, [r9, #0x28]
 80052f4: eb04 050c    	add.w	r5, r4, r12
 80052f8: 4634         	mov	r4, r6
 80052fa: f8de 61f8    	ldr.w	r6, [lr, #0x1f8]
 80052fe: f10c 0c2c    	add.w	r12, r12, #0x2c
 8005302: f8de 0210    	ldr.w	r0, [lr, #0x210]
 8005306: 1b80         	subs	r0, r0, r6
 8005308: 62e8         	str	r0, [r5, #0x2c]
 800530a: 9d19         	ldr	r5, [sp, #0x64]
 800530c: 0046         	lsls	r6, r0, #0x1
 800530e: ea86 70e0    	eor.w	r0, r6, r0, asr #31
 8005312: 9e0f         	ldr	r6, [sp, #0x3c]
 8005314: f8c9 002c    	str.w	r0, [r9, #0x2c]
 8005318: 442e         	add	r6, r5
 800531a: 9d0e         	ldr	r5, [sp, #0x38]
 800531c: f8dd 904c    	ldr.w	r9, [sp, #0x4c]
 8005320: 442e         	add	r6, r5
 8005322: 9d0d         	ldr	r5, [sp, #0x34]
 8005324: f109 092c    	add.w	r9, r9, #0x2c
 8005328: 442e         	add	r6, r5
 800532a: 9d0c         	ldr	r5, [sp, #0x30]
 800532c: 442e         	add	r6, r5
 800532e: 9d0b         	ldr	r5, [sp, #0x2c]
 8005330: 442e         	add	r6, r5
 8005332: 9d0a         	ldr	r5, [sp, #0x28]
 8005334: 442e         	add	r6, r5
 8005336: 9d09         	ldr	r5, [sp, #0x24]
 8005338: 442e         	add	r6, r5
 800533a: 9d14         	ldr	r5, [sp, #0x50]
 800533c: 4432         	add	r2, r6
 800533e: f104 062c    	add.w	r6, r4, #0x2c
 8005342: 4411         	add	r1, r2
 8005344: 9a12         	ldr	r2, [sp, #0x48]
 8005346: 4419         	add	r1, r3
 8005348: f105 032c    	add.w	r3, r5, #0x2c
 800534c: 4408         	add	r0, r1
 800534e: 9019         	str	r0, [sp, #0x64]
 8005350: 981a         	ldr	r0, [sp, #0x68]
 8005352: f502 7284    	add.w	r2, r2, #0x108
 8005356: 9915         	ldr	r1, [sp, #0x54]
 8005358: 302c         	adds	r0, #0x2c
 800535a: 901a         	str	r0, [sp, #0x68]
 800535c: 981d         	ldr	r0, [sp, #0x74]
 800535e: 312c         	adds	r1, #0x2c
 8005360: 9d23         	ldr	r5, [sp, #0x8c]
 8005362: f500 7084    	add.w	r0, r0, #0x108
 8005366: 901d         	str	r0, [sp, #0x74]
 8005368: 981b         	ldr	r0, [sp, #0x6c]
 800536a: 352c         	adds	r5, #0x2c
 800536c: 9523         	str	r5, [sp, #0x8c]
 800536e: 302c         	adds	r0, #0x2c
 8005370: 901b         	str	r0, [sp, #0x6c]
 8005372: 981e         	ldr	r0, [sp, #0x78]
 8005374: f500 7084    	add.w	r0, r0, #0x108
 8005378: 901e         	str	r0, [sp, #0x78]
 800537a: 981f         	ldr	r0, [sp, #0x7c]
 800537c: f500 7084    	add.w	r0, r0, #0x108
 8005380: 901f         	str	r0, [sp, #0x7c]
 8005382: 981c         	ldr	r0, [sp, #0x70]
 8005384: 302c         	adds	r0, #0x2c
 8005386: 901c         	str	r0, [sp, #0x70]
 8005388: 9821         	ldr	r0, [sp, #0x84]
 800538a: f500 7084    	add.w	r0, r0, #0x108
 800538e: 9021         	str	r0, [sp, #0x84]
 8005390: 9822         	ldr	r0, [sp, #0x88]
 8005392: f500 7084    	add.w	r0, r0, #0x108
 8005396: 9022         	str	r0, [sp, #0x88]
 8005398: 9820         	ldr	r0, [sp, #0x80]
 800539a: 302c         	adds	r0, #0x2c
 800539c: 9020         	str	r0, [sp, #0x80]
 800539e: 9810         	ldr	r0, [sp, #0x40]
 80053a0: 302c         	adds	r0, #0x2c
 80053a2: f47f aee9    	bne.w	0x8005178 <raylar_crcl_encoder_service::benchmark::prepare::hb9734b8d5dc7f78e+0x312> @ imm = #-0x22e
 80053a6: 9805         	ldr	r0, [sp, #0x14]
 80053a8: 9919         	ldr	r1, [sp, #0x64]
 80053aa: 6001         	str	r1, [r0]
 80053ac: f241 0005    	movw	r0, #0x1005
 80053b0: f2c0 0040    	movt	r0, #0x40
 80053b4: b191         	cbz	r1, 0x80053dc <raylar_crcl_encoder_service::benchmark::prepare::hb9734b8d5dc7f78e+0x576> @ imm = #0x24
 80053b6: 9e19         	ldr	r6, [sp, #0x64]
 80053b8: fba6 0100    	umull	r0, r1, r6, r0
 80053bc: 1a70         	subs	r0, r6, r1
 80053be: eb01 0050    	add.w	r0, r1, r0, lsr #1
 80053c2: 2101         	movs	r1, #0x1
 80053c4: 0a42         	lsrs	r2, r0, #0x9
 80053c6: eba2 2382    	sub.w	r3, r2, r2, lsl #10
 80053ca: 199b         	adds	r3, r3, r6
 80053cc: bf18         	it	ne
 80053ce: eb01 2250    	addne.w	r2, r1, r0, lsr #9
 80053d2: fab2 f082    	clz	r0, r2
 80053d6: f1c0 001f    	rsb.w	r0, r0, #0x1f
 80053da: e000         	b	0x80053de <raylar_crcl_encoder_service::benchmark::prepare::hb9734b8d5dc7f78e+0x578> @ imm = #0x0
 80053dc: 2000         	movs	r0, #0x0
 80053de: 9924         	ldr	r1, [sp, #0x90]
 80053e0: f24f 0204    	movw	r2, #0xf004
 80053e4: f6cf 72ff    	movt	r2, #0xffff
 80053e8: f88d 0150    	strb.w	r0, [sp, #0x150]
 80053ec: b2c0         	uxtb	r0, r0
 80053ee: 7909         	ldrb	r1, [r1, #0x4]
 80053f0: 4288         	cmp	r0, r1
 80053f2: f040 86be    	bne.w	0x8006172 <raylar_crcl_encoder_service::benchmark::prepare::hb9734b8d5dc7f78e+0x130c> @ imm = #0xd7c
 80053f6: 9804         	ldr	r0, [sp, #0x10]
 80053f8: f248 040c    	movw	r4, #0x800c
 80053fc: 9905         	ldr	r1, [sp, #0x14]
 80053fe: f04f 0800    	mov.w	r8, #0x0
 8005402: f2c0 0401    	movt	r4, #0x1
 8005406: f64b 7e58    	movw	lr, #0xbf58
 800540a: 6800         	ldr	r0, [r0]
 800540c: 4694         	mov	r12, r2
 800540e: 6148         	str	r0, [r1, #0x14]
 8005410: f642 70f8    	movw	r0, #0x2ff8
 8005414: f2c0 0001    	movt	r0, #0x1
 8005418: f100 0128    	add.w	r1, r0, #0x28
 800541c: 9119         	str	r1, [sp, #0x64]
 800541e: f44f 4140    	mov.w	r1, #0xc000
 8005422: 9118         	str	r1, [sp, #0x60]
 8005424: f64b 7170    	movw	r1, #0xbf70
 8005428: 9123         	str	r1, [sp, #0x8c]
 800542a: f64b 71e8    	movw	r1, #0xbfe8
 800542e: 9117         	str	r1, [sp, #0x5c]
 8005430: f44f 3198    	mov.w	r1, #0x13000
 8005434: 9122         	str	r1, [sp, #0x88]
 8005436: f64b 71d0    	movw	r1, #0xbfd0
 800543a: f100 0324    	add.w	r3, r0, #0x24
 800543e: 9116         	str	r1, [sp, #0x58]
 8005440: f64b 7188    	movw	r1, #0xbf88
 8005444: 931b         	str	r3, [sp, #0x6c]
 8005446: f100 0320    	add.w	r3, r0, #0x20
 800544a: 9121         	str	r1, [sp, #0x84]
 800544c: f64b 71b8    	movw	r1, #0xbfb8
 8005450: f100 061c    	add.w	r6, r0, #0x1c
 8005454: f100 0518    	add.w	r5, r0, #0x18
 8005458: f100 0914    	add.w	r9, r0, #0x14
 800545c: f100 0b04    	add.w	r11, r0, #0x4
 8005460: 931a         	str	r3, [sp, #0x68]
 8005462: f64b 7328    	movw	r3, #0xbf28
 8005466: 9120         	str	r1, [sp, #0x80]
 8005468: f64b 71a0    	movw	r1, #0xbfa0
 800546c: 911f         	str	r1, [sp, #0x7c]
 800546e: f100 0110    	add.w	r1, r0, #0x10
 8005472: 911e         	str	r1, [sp, #0x78]
 8005474: a912         	add	r1, sp, #0x48
 8005476: 46aa         	mov	r10, r5
 8005478: e881 0218    	stm.w	r1, {r3, r4, r9}
 800547c: 9924         	ldr	r1, [sp, #0x90]
 800547e: 9515         	str	r5, [sp, #0x54]
 8005480: 18cd         	adds	r5, r1, r3
 8005482: 901d         	str	r0, [sp, #0x74]
 8005484: 981d         	ldr	r0, [sp, #0x74]
 8005486: 961c         	str	r6, [sp, #0x70]
 8005488: f8d5 3108    	ldr.w	r3, [r5, #0x108]
 800548c: 4408         	add	r0, r1
 800548e: f8d5 6120    	ldr.w	r6, [r5, #0x120]
 8005492: f8cd 8038    	str.w	r8, [sp, #0x38]
 8005496: eb01 0804    	add.w	r8, r1, r4
 800549a: 1af3         	subs	r3, r6, r3
 800549c: 62c3         	str	r3, [r0, #0x2c]
 800549e: 9a23         	ldr	r2, [sp, #0x8c]
 80054a0: 0058         	lsls	r0, r3, #0x1
 80054a2: e9cd be10    	strd	r11, lr, [sp, #64]
 80054a6: ea80 70e3    	eor.w	r0, r0, r3, asr #31
 80054aa: eb01 030e    	add.w	r3, r1, lr
 80054ae: f8c8 0004    	str.w	r0, [r8, #0x4]
 80054b2: 900d         	str	r0, [sp, #0x34]
 80054b4: f8d5 0120    	ldr.w	r0, [r5, #0x120]
 80054b8: f8d3 6108    	ldr.w	r6, [r3, #0x108]
 80054bc: 9c17         	ldr	r4, [sp, #0x5c]
 80054be: 1a30         	subs	r0, r6, r0
 80054c0: eb01 060b    	add.w	r6, r1, r11
 80054c4: f8dd b058    	ldr.w	r11, [sp, #0x58]
 80054c8: 62f0         	str	r0, [r6, #0x2c]
 80054ca: 0046         	lsls	r6, r0, #0x1
 80054cc: ea86 70e0    	eor.w	r0, r6, r0, asr #31
 80054d0: f8c8 0008    	str.w	r0, [r8, #0x8]
 80054d4: 900c         	str	r0, [sp, #0x30]
 80054d6: f8d3 0108    	ldr.w	r0, [r3, #0x108]
 80054da: 188b         	adds	r3, r1, r2
 80054dc: 9a22         	ldr	r2, [sp, #0x88]
 80054de: f8d3 6108    	ldr.w	r6, [r3, #0x108]
 80054e2: f8dd e068    	ldr.w	lr, [sp, #0x68]
 80054e6: 1a30         	subs	r0, r6, r0
 80054e8: 188e         	adds	r6, r1, r2
 80054ea: 9a21         	ldr	r2, [sp, #0x84]
 80054ec: 62f0         	str	r0, [r6, #0x2c]
 80054ee: 0046         	lsls	r6, r0, #0x1
 80054f0: ea86 70e0    	eor.w	r0, r6, r0, asr #31
 80054f4: f8c8 000c    	str.w	r0, [r8, #0xc]
 80054f8: 900b         	str	r0, [sp, #0x2c]
 80054fa: f244 062c    	movw	r6, #0x402c
 80054fe: f8d3 0108    	ldr.w	r0, [r3, #0x108]
 8005502: 188b         	adds	r3, r1, r2
 8005504: f2c0 0601    	movt	r6, #0x1
 8005508: f8cd c03c    	str.w	r12, [sp, #0x3c]
 800550c: f8d3 2108    	ldr.w	r2, [r3, #0x108]
 8005510: 1a10         	subs	r0, r2, r0
 8005512: eb01 020c    	add.w	r2, r1, r12
 8005516: f8dd c06c    	ldr.w	r12, [sp, #0x6c]
 800551a: 5190         	str	r0, [r2, r6]
 800551c: 0042         	lsls	r2, r0, #0x1
 800551e: ea82 70e0    	eor.w	r0, r2, r0, asr #31
 8005522: 9a1f         	ldr	r2, [sp, #0x7c]
 8005524: eb01 060e    	add.w	r6, r1, lr
 8005528: f8c8 0010    	str.w	r0, [r8, #0x10]
 800552c: f10e 0e2c    	add.w	lr, lr, #0x2c
 8005530: 440a         	add	r2, r1
 8005532: 900a         	str	r0, [sp, #0x28]
 8005534: f8d3 0108    	ldr.w	r0, [r3, #0x108]
 8005538: f8d2 3108    	ldr.w	r3, [r2, #0x108]
 800553c: f8cd e068    	str.w	lr, [sp, #0x68]
 8005540: 1a18         	subs	r0, r3, r0
 8005542: 9b1e         	ldr	r3, [sp, #0x78]
 8005544: f8dd e044    	ldr.w	lr, [sp, #0x44]
 8005548: 440b         	add	r3, r1
 800554a: f50e 7e84    	add.w	lr, lr, #0x108
 800554e: 62d8         	str	r0, [r3, #0x2c]
 8005550: 0043         	lsls	r3, r0, #0x1
 8005552: ea83 70e0    	eor.w	r0, r3, r0, asr #31
 8005556: f8c8 0014    	str.w	r0, [r8, #0x14]
 800555a: 9009         	str	r0, [sp, #0x24]
 800555c: f8d2 0108    	ldr.w	r0, [r2, #0x108]
 8005560: 9a20         	ldr	r2, [sp, #0x80]
 8005562: 440a         	add	r2, r1
 8005564: f8d2 3108    	ldr.w	r3, [r2, #0x108]
 8005568: 1a18         	subs	r0, r3, r0
 800556a: eb01 0309    	add.w	r3, r1, r9
 800556e: 62d8         	str	r0, [r3, #0x2c]
 8005570: 0043         	lsls	r3, r0, #0x1
 8005572: ea83 70e0    	eor.w	r0, r3, r0, asr #31
 8005576: f8c8 0018    	str.w	r0, [r8, #0x18]
 800557a: 9008         	str	r0, [sp, #0x20]
 800557c: f8d2 0108    	ldr.w	r0, [r2, #0x108]
 8005580: eb01 020b    	add.w	r2, r1, r11
 8005584: f50b 7b84    	add.w	r11, r11, #0x108
 8005588: f8cd b058    	str.w	r11, [sp, #0x58]
 800558c: f8d2 3108    	ldr.w	r3, [r2, #0x108]
 8005590: f8dd b040    	ldr.w	r11, [sp, #0x40]
 8005594: 1a18         	subs	r0, r3, r0
 8005596: eb01 030a    	add.w	r3, r1, r10
 800559a: f8dd a060    	ldr.w	r10, [sp, #0x60]
 800559e: f10b 0b2c    	add.w	r11, r11, #0x2c
 80055a2: 62d8         	str	r0, [r3, #0x2c]
 80055a4: 0043         	lsls	r3, r0, #0x1
 80055a6: ea83 70e0    	eor.w	r0, r3, r0, asr #31
 80055aa: f8c8 001c    	str.w	r0, [r8, #0x1c]
 80055ae: 9007         	str	r0, [sp, #0x1c]
 80055b0: f8d2 0108    	ldr.w	r0, [r2, #0x108]
 80055b4: 190a         	adds	r2, r1, r4
 80055b6: f504 7484    	add.w	r4, r4, #0x108
 80055ba: 9417         	str	r4, [sp, #0x5c]
 80055bc: f8d2 3108    	ldr.w	r3, [r2, #0x108]
 80055c0: 9c13         	ldr	r4, [sp, #0x4c]
 80055c2: 1a18         	subs	r0, r3, r0
 80055c4: 9b1c         	ldr	r3, [sp, #0x70]
 80055c6: 342c         	adds	r4, #0x2c
 80055c8: 440b         	add	r3, r1
 80055ca: 62d8         	str	r0, [r3, #0x2c]
 80055cc: 0043         	lsls	r3, r0, #0x1
 80055ce: ea83 73e0    	eor.w	r3, r3, r0, asr #31
 80055d2: f8c8 3020    	str.w	r3, [r8, #0x20]
 80055d6: f8d2 0108    	ldr.w	r0, [r2, #0x108]
 80055da: eb01 020a    	add.w	r2, r1, r10
 80055de: f50a 7a84    	add.w	r10, r10, #0x108
 80055e2: f8cd a060    	str.w	r10, [sp, #0x60]
 80055e6: f8d2 9108    	ldr.w	r9, [r2, #0x108]
 80055ea: eba9 0000    	sub.w	r0, r9, r0
 80055ee: 62f0         	str	r0, [r6, #0x2c]
 80055f0: f8dd 9064    	ldr.w	r9, [sp, #0x64]
 80055f4: 0046         	lsls	r6, r0, #0x1
 80055f6: ea86 70e0    	eor.w	r0, r6, r0, asr #31
 80055fa: f8c8 0024    	str.w	r0, [r8, #0x24]
 80055fe: f8d2 2108    	ldr.w	r2, [r2, #0x108]
 8005602: f8d5 61f8    	ldr.w	r6, [r5, #0x1f8]
 8005606: 1ab2         	subs	r2, r6, r2
 8005608: eb01 060c    	add.w	r6, r1, r12
 800560c: f10c 0c2c    	add.w	r12, r12, #0x2c
 8005610: f8cd c06c    	str.w	r12, [sp, #0x6c]
 8005614: 62f2         	str	r2, [r6, #0x2c]
 8005616: 0056         	lsls	r6, r2, #0x1
 8005618: ea86 72e2    	eor.w	r2, r6, r2, asr #31
 800561c: f8c8 2028    	str.w	r2, [r8, #0x28]
 8005620: eb01 0609    	add.w	r6, r1, r9
 8005624: f8d5 11f8    	ldr.w	r1, [r5, #0x1f8]
 8005628: f8d5 5210    	ldr.w	r5, [r5, #0x210]
 800562c: f8dd c03c    	ldr.w	r12, [sp, #0x3c]
 8005630: 1a69         	subs	r1, r5, r1
 8005632: 62f1         	str	r1, [r6, #0x2c]
 8005634: f11c 0c2c    	adds.w	r12, r12, #0x2c
 8005638: ea4f 0541    	lsl.w	r5, r1, #0x1
 800563c: ea85 71e1    	eor.w	r1, r5, r1, asr #31
 8005640: e9dd 650d    	ldrd	r6, r5, [sp, #52]
 8005644: f8c8 102c    	str.w	r1, [r8, #0x2c]
 8005648: 4435         	add	r5, r6
 800564a: 9e0c         	ldr	r6, [sp, #0x30]
 800564c: 4435         	add	r5, r6
 800564e: 9e0b         	ldr	r6, [sp, #0x2c]
 8005650: 4435         	add	r5, r6
 8005652: 9e0a         	ldr	r6, [sp, #0x28]
 8005654: 442e         	add	r6, r5
 8005656: 9d09         	ldr	r5, [sp, #0x24]
 8005658: 442e         	add	r6, r5
 800565a: 9d08         	ldr	r5, [sp, #0x20]
 800565c: 442e         	add	r6, r5
 800565e: 9d07         	ldr	r5, [sp, #0x1c]
 8005660: 442e         	add	r6, r5
 8005662: 9d15         	ldr	r5, [sp, #0x54]
 8005664: 4433         	add	r3, r6
 8005666: 4418         	add	r0, r3
 8005668: 9b12         	ldr	r3, [sp, #0x48]
 800566a: 4410         	add	r0, r2
 800566c: 464a         	mov	r2, r9
 800566e: eb01 0800    	add.w	r8, r1, r0
 8005672: 991f         	ldr	r1, [sp, #0x7c]
 8005674: e9dd 601c    	ldrd	r6, r0, [sp, #112]
 8005678: f105 052c    	add.w	r5, r5, #0x2c
 800567c: f501 7184    	add.w	r1, r1, #0x108
 8005680: 911f         	str	r1, [sp, #0x7c]
 8005682: 991e         	ldr	r1, [sp, #0x78]
 8005684: f106 062c    	add.w	r6, r6, #0x2c
 8005688: f8dd 9050    	ldr.w	r9, [sp, #0x50]
 800568c: f503 7384    	add.w	r3, r3, #0x108
 8005690: f101 012c    	add.w	r1, r1, #0x2c
 8005694: 911e         	str	r1, [sp, #0x78]
 8005696: 9920         	ldr	r1, [sp, #0x80]
 8005698: f109 092c    	add.w	r9, r9, #0x2c
 800569c: f100 002c    	add.w	r0, r0, #0x2c
 80056a0: f102 022c    	add.w	r2, r2, #0x2c
 80056a4: f501 7184    	add.w	r1, r1, #0x108
 80056a8: 9120         	str	r1, [sp, #0x80]
 80056aa: 9921         	ldr	r1, [sp, #0x84]
 80056ac: 9219         	str	r2, [sp, #0x64]
 80056ae: f501 7184    	add.w	r1, r1, #0x108
 80056b2: 9121         	str	r1, [sp, #0x84]
 80056b4: 9922         	ldr	r1, [sp, #0x88]
 80056b6: f101 012c    	add.w	r1, r1, #0x2c
 80056ba: 9122         	str	r1, [sp, #0x88]
 80056bc: 9923         	ldr	r1, [sp, #0x8c]
 80056be: f501 7184    	add.w	r1, r1, #0x108
 80056c2: 9123         	str	r1, [sp, #0x8c]
 80056c4: f47f aed6    	bne.w	0x8005474 <raylar_crcl_encoder_service::benchmark::prepare::hb9734b8d5dc7f78e+0x60e> @ imm = #-0x254
 80056c8: 9805         	ldr	r0, [sp, #0x14]
 80056ca: f1b8 0f00    	cmp.w	r8, #0x0
 80056ce: f8c0 8004    	str.w	r8, [r0, #0x4]
 80056d2: d017         	beq	0x8005704 <raylar_crcl_encoder_service::benchmark::prepare::hb9734b8d5dc7f78e+0x89e> @ imm = #0x2e
 80056d4: f241 0005    	movw	r0, #0x1005
 80056d8: f2c0 0040    	movt	r0, #0x40
 80056dc: fba8 0100    	umull	r0, r1, r8, r0
 80056e0: eba8 0001    	sub.w	r0, r8, r1
 80056e4: eb01 0050    	add.w	r0, r1, r0, lsr #1
 80056e8: 2101         	movs	r1, #0x1
 80056ea: 0a42         	lsrs	r2, r0, #0x9
 80056ec: eba2 2382    	sub.w	r3, r2, r2, lsl #10
 80056f0: eb13 0308    	adds.w	r3, r3, r8
 80056f4: bf18         	it	ne
 80056f6: eb01 2250    	addne.w	r2, r1, r0, lsr #9
 80056fa: fab2 f082    	clz	r0, r2
 80056fe: f1c0 001f    	rsb.w	r0, r0, #0x1f
 8005702: e000         	b	0x8005706 <raylar_crcl_encoder_service::benchmark::prepare::hb9734b8d5dc7f78e+0x8a0> @ imm = #0x0
 8005704: 2000         	movs	r0, #0x0
 8005706: 9924         	ldr	r1, [sp, #0x90]
 8005708: f24f 0204    	movw	r2, #0xf004
 800570c: f6cf 72ff    	movt	r2, #0xffff
 8005710: f88d 0150    	strb.w	r0, [sp, #0x150]
 8005714: b2c0         	uxtb	r0, r0
 8005716: 7b09         	ldrb	r1, [r1, #0xc]
 8005718: 4288         	cmp	r0, r1
 800571a: f040 851e    	bne.w	0x800615a <raylar_crcl_encoder_service::benchmark::prepare::hb9734b8d5dc7f78e+0x12f4> @ imm = #0xa3c
 800571e: 9804         	ldr	r0, [sp, #0x10]
 8005720: f249 0408    	movw	r4, #0x9008
 8005724: 9905         	ldr	r1, [sp, #0x14]
 8005726: f04f 0800    	mov.w	r8, #0x0
 800572a: f2c0 0401    	movt	r4, #0x1
 800572e: f64b 7e5c    	movw	lr, #0xbf5c
 8005732: 68c0         	ldr	r0, [r0, #0xc]
 8005734: 4694         	mov	r12, r2
 8005736: 6188         	str	r0, [r1, #0x18]
 8005738: f643 70f4    	movw	r0, #0x3ff4
 800573c: f2c0 0001    	movt	r0, #0x1
 8005740: f100 0128    	add.w	r1, r0, #0x28
 8005744: 9119         	str	r1, [sp, #0x64]
 8005746: f24c 0104    	movw	r1, #0xc004
 800574a: 9118         	str	r1, [sp, #0x60]
 800574c: f64b 7174    	movw	r1, #0xbf74
 8005750: 9123         	str	r1, [sp, #0x8c]
 8005752: f64b 71ec    	movw	r1, #0xbfec
 8005756: 9117         	str	r1, [sp, #0x5c]
 8005758: f64b 71d4    	movw	r1, #0xbfd4
 800575c: 9116         	str	r1, [sp, #0x58]
 800575e: f64b 718c    	movw	r1, #0xbf8c
 8005762: f100 0324    	add.w	r3, r0, #0x24
 8005766: 9122         	str	r1, [sp, #0x88]
 8005768: f64b 71bc    	movw	r1, #0xbfbc
 800576c: 931b         	str	r3, [sp, #0x6c]
 800576e: f100 0320    	add.w	r3, r0, #0x20
 8005772: 9121         	str	r1, [sp, #0x84]
 8005774: f44f 31a0    	mov.w	r1, #0x14000
 8005778: f100 061c    	add.w	r6, r0, #0x1c
 800577c: f100 0518    	add.w	r5, r0, #0x18
 8005780: f100 0914    	add.w	r9, r0, #0x14
 8005784: f100 0b08    	add.w	r11, r0, #0x8
 8005788: 931a         	str	r3, [sp, #0x68]
 800578a: f64b 732c    	movw	r3, #0xbf2c
 800578e: 9120         	str	r1, [sp, #0x80]
 8005790: f64b 71a4    	movw	r1, #0xbfa4
 8005794: 911f         	str	r1, [sp, #0x7c]
 8005796: 1d01         	adds	r1, r0, #0x4
 8005798: 911e         	str	r1, [sp, #0x78]
 800579a: a90f         	add	r1, sp, #0x3c
 800579c: 46aa         	mov	r10, r5
 800579e: e881 4900    	stm.w	r1, {r8, r11, lr}
 80057a2: a912         	add	r1, sp, #0x48
 80057a4: e881 0218    	stm.w	r1, {r3, r4, r9}
 80057a8: 9924         	ldr	r1, [sp, #0x90]
 80057aa: 9515         	str	r5, [sp, #0x54]
 80057ac: 18cd         	adds	r5, r1, r3
 80057ae: 901d         	str	r0, [sp, #0x74]
 80057b0: 981d         	ldr	r0, [sp, #0x74]
 80057b2: eb01 0804    	add.w	r8, r1, r4
 80057b6: 961c         	str	r6, [sp, #0x70]
 80057b8: f8d5 3108    	ldr.w	r3, [r5, #0x108]
 80057bc: 4408         	add	r0, r1
 80057be: f8d5 6120    	ldr.w	r6, [r5, #0x120]
 80057c2: 9a1e         	ldr	r2, [sp, #0x78]
 80057c4: 1af3         	subs	r3, r6, r3
 80057c6: 62c3         	str	r3, [r0, #0x2c]
 80057c8: 9c18         	ldr	r4, [sp, #0x60]
 80057ca: 0058         	lsls	r0, r3, #0x1
 80057cc: f8cd c038    	str.w	r12, [sp, #0x38]
 80057d0: ea80 70e3    	eor.w	r0, r0, r3, asr #31
 80057d4: eb01 030e    	add.w	r3, r1, lr
 80057d8: f8c8 0004    	str.w	r0, [r8, #0x4]
 80057dc: 900d         	str	r0, [sp, #0x34]
 80057de: f8d5 0120    	ldr.w	r0, [r5, #0x120]
 80057e2: f8d3 6108    	ldr.w	r6, [r3, #0x108]
 80057e6: f8dd e068    	ldr.w	lr, [sp, #0x68]
 80057ea: 1a30         	subs	r0, r6, r0
 80057ec: 188e         	adds	r6, r1, r2
 80057ee: 9a23         	ldr	r2, [sp, #0x8c]
 80057f0: 62f0         	str	r0, [r6, #0x2c]
 80057f2: 0046         	lsls	r6, r0, #0x1
 80057f4: ea86 70e0    	eor.w	r0, r6, r0, asr #31
 80057f8: f8c8 0008    	str.w	r0, [r8, #0x8]
 80057fc: 900c         	str	r0, [sp, #0x30]
 80057fe: f8d3 0108    	ldr.w	r0, [r3, #0x108]
 8005802: 188b         	adds	r3, r1, r2
 8005804: 9a22         	ldr	r2, [sp, #0x88]
 8005806: f8d3 6108    	ldr.w	r6, [r3, #0x108]
 800580a: 1a30         	subs	r0, r6, r0
 800580c: eb01 060b    	add.w	r6, r1, r11
 8005810: f8dd b058    	ldr.w	r11, [sp, #0x58]
 8005814: 62f0         	str	r0, [r6, #0x2c]
 8005816: 0046         	lsls	r6, r0, #0x1
 8005818: ea86 70e0    	eor.w	r0, r6, r0, asr #31
 800581c: f8c8 000c    	str.w	r0, [r8, #0xc]
 8005820: 900b         	str	r0, [sp, #0x2c]
 8005822: f245 062c    	movw	r6, #0x502c
 8005826: f8d3 0108    	ldr.w	r0, [r3, #0x108]
 800582a: 188b         	adds	r3, r1, r2
 800582c: f2c0 0601    	movt	r6, #0x1
 8005830: f8d3 2108    	ldr.w	r2, [r3, #0x108]
 8005834: 1a10         	subs	r0, r2, r0
 8005836: 9a20         	ldr	r2, [sp, #0x80]
 8005838: 440a         	add	r2, r1
 800583a: 62d0         	str	r0, [r2, #0x2c]
 800583c: 0042         	lsls	r2, r0, #0x1
 800583e: ea82 70e0    	eor.w	r0, r2, r0, asr #31
 8005842: 9a1f         	ldr	r2, [sp, #0x7c]
 8005844: f8c8 0010    	str.w	r0, [r8, #0x10]
 8005848: 440a         	add	r2, r1
 800584a: 900a         	str	r0, [sp, #0x28]
 800584c: f8d3 0108    	ldr.w	r0, [r3, #0x108]
 8005850: f8d2 3108    	ldr.w	r3, [r2, #0x108]
 8005854: 1a18         	subs	r0, r3, r0
 8005856: eb01 030c    	add.w	r3, r1, r12
 800585a: f8dd c06c    	ldr.w	r12, [sp, #0x6c]
 800585e: 5198         	str	r0, [r3, r6]
 8005860: 0043         	lsls	r3, r0, #0x1
 8005862: ea83 70e0    	eor.w	r0, r3, r0, asr #31
 8005866: f8c8 0014    	str.w	r0, [r8, #0x14]
 800586a: eb01 060e    	add.w	r6, r1, lr
 800586e: 9009         	str	r0, [sp, #0x24]
 8005870: f10e 0e2c    	add.w	lr, lr, #0x2c
 8005874: f8d2 0108    	ldr.w	r0, [r2, #0x108]
 8005878: 9a21         	ldr	r2, [sp, #0x84]
 800587a: f8cd e068    	str.w	lr, [sp, #0x68]
 800587e: 440a         	add	r2, r1
 8005880: f8dd e044    	ldr.w	lr, [sp, #0x44]
 8005884: f8d2 3108    	ldr.w	r3, [r2, #0x108]
 8005888: f50e 7e84    	add.w	lr, lr, #0x108
 800588c: 1a18         	subs	r0, r3, r0
 800588e: eb01 0309    	add.w	r3, r1, r9
 8005892: 62d8         	str	r0, [r3, #0x2c]
 8005894: 0043         	lsls	r3, r0, #0x1
 8005896: ea83 70e0    	eor.w	r0, r3, r0, asr #31
 800589a: f8c8 0018    	str.w	r0, [r8, #0x18]
 800589e: 9008         	str	r0, [sp, #0x20]
 80058a0: f8d2 0108    	ldr.w	r0, [r2, #0x108]
 80058a4: eb01 020b    	add.w	r2, r1, r11
 80058a8: f50b 7b84    	add.w	r11, r11, #0x108
 80058ac: f8cd b058    	str.w	r11, [sp, #0x58]
 80058b0: f8d2 3108    	ldr.w	r3, [r2, #0x108]
 80058b4: f8dd b040    	ldr.w	r11, [sp, #0x40]
 80058b8: 1a18         	subs	r0, r3, r0
 80058ba: eb01 030a    	add.w	r3, r1, r10
 80058be: f8dd a05c    	ldr.w	r10, [sp, #0x5c]
 80058c2: f10b 0b2c    	add.w	r11, r11, #0x2c
 80058c6: 62d8         	str	r0, [r3, #0x2c]
 80058c8: 0043         	lsls	r3, r0, #0x1
 80058ca: ea83 70e0    	eor.w	r0, r3, r0, asr #31
 80058ce: f8c8 001c    	str.w	r0, [r8, #0x1c]
 80058d2: 9007         	str	r0, [sp, #0x1c]
 80058d4: f8d2 0108    	ldr.w	r0, [r2, #0x108]
 80058d8: eb01 020a    	add.w	r2, r1, r10
 80058dc: f50a 7a84    	add.w	r10, r10, #0x108
 80058e0: f8cd a05c    	str.w	r10, [sp, #0x5c]
 80058e4: f8d2 3108    	ldr.w	r3, [r2, #0x108]
 80058e8: 1a18         	subs	r0, r3, r0
 80058ea: 9b1c         	ldr	r3, [sp, #0x70]
 80058ec: 440b         	add	r3, r1
 80058ee: 62d8         	str	r0, [r3, #0x2c]
 80058f0: 0043         	lsls	r3, r0, #0x1
 80058f2: ea83 73e0    	eor.w	r3, r3, r0, asr #31
 80058f6: f8c8 3020    	str.w	r3, [r8, #0x20]
 80058fa: f8d2 0108    	ldr.w	r0, [r2, #0x108]
 80058fe: 190a         	adds	r2, r1, r4
 8005900: f504 7484    	add.w	r4, r4, #0x108
 8005904: 9418         	str	r4, [sp, #0x60]
 8005906: f8d2 9108    	ldr.w	r9, [r2, #0x108]
 800590a: 9c13         	ldr	r4, [sp, #0x4c]
 800590c: eba9 0000    	sub.w	r0, r9, r0
 8005910: 62f0         	str	r0, [r6, #0x2c]
 8005912: f8dd 9064    	ldr.w	r9, [sp, #0x64]
 8005916: 342c         	adds	r4, #0x2c
 8005918: 0046         	lsls	r6, r0, #0x1
 800591a: ea86 70e0    	eor.w	r0, r6, r0, asr #31
 800591e: f8c8 0024    	str.w	r0, [r8, #0x24]
 8005922: f8d2 2108    	ldr.w	r2, [r2, #0x108]
 8005926: f8d5 61f8    	ldr.w	r6, [r5, #0x1f8]
 800592a: 1ab2         	subs	r2, r6, r2
 800592c: eb01 060c    	add.w	r6, r1, r12
 8005930: f10c 0c2c    	add.w	r12, r12, #0x2c
 8005934: f8cd c06c    	str.w	r12, [sp, #0x6c]
 8005938: 62f2         	str	r2, [r6, #0x2c]
 800593a: 0056         	lsls	r6, r2, #0x1
 800593c: ea86 72e2    	eor.w	r2, r6, r2, asr #31
 8005940: f8c8 2028    	str.w	r2, [r8, #0x28]
 8005944: eb01 0609    	add.w	r6, r1, r9
 8005948: f8d5 11f8    	ldr.w	r1, [r5, #0x1f8]
 800594c: f8d5 5210    	ldr.w	r5, [r5, #0x210]
 8005950: f8dd c038    	ldr.w	r12, [sp, #0x38]
 8005954: 1a69         	subs	r1, r5, r1
 8005956: 62f1         	str	r1, [r6, #0x2c]
 8005958: 9e0d         	ldr	r6, [sp, #0x34]
 800595a: f11c 0c2c    	adds.w	r12, r12, #0x2c
 800595e: ea4f 0541    	lsl.w	r5, r1, #0x1
 8005962: ea85 71e1    	eor.w	r1, r5, r1, asr #31
 8005966: 9d0f         	ldr	r5, [sp, #0x3c]
 8005968: f8c8 102c    	str.w	r1, [r8, #0x2c]
 800596c: 4435         	add	r5, r6
 800596e: 9e0c         	ldr	r6, [sp, #0x30]
 8005970: 4435         	add	r5, r6
 8005972: 9e0b         	ldr	r6, [sp, #0x2c]
 8005974: 4435         	add	r5, r6
 8005976: 9e0a         	ldr	r6, [sp, #0x28]
 8005978: 442e         	add	r6, r5
 800597a: 9d09         	ldr	r5, [sp, #0x24]
 800597c: 442e         	add	r6, r5
 800597e: 9d08         	ldr	r5, [sp, #0x20]
 8005980: 442e         	add	r6, r5
 8005982: 9d07         	ldr	r5, [sp, #0x1c]
 8005984: 442e         	add	r6, r5
 8005986: 9d15         	ldr	r5, [sp, #0x54]
 8005988: 4433         	add	r3, r6
 800598a: 4418         	add	r0, r3
 800598c: 9b12         	ldr	r3, [sp, #0x48]
 800598e: 4410         	add	r0, r2
 8005990: 464a         	mov	r2, r9
 8005992: eb01 0800    	add.w	r8, r1, r0
 8005996: 991f         	ldr	r1, [sp, #0x7c]
 8005998: e9dd 601c    	ldrd	r6, r0, [sp, #112]
 800599c: f105 052c    	add.w	r5, r5, #0x2c
 80059a0: f501 7184    	add.w	r1, r1, #0x108
 80059a4: 911f         	str	r1, [sp, #0x7c]
 80059a6: 9920         	ldr	r1, [sp, #0x80]
 80059a8: f106 062c    	add.w	r6, r6, #0x2c
 80059ac: f8dd 9050    	ldr.w	r9, [sp, #0x50]
 80059b0: f503 7384    	add.w	r3, r3, #0x108
 80059b4: f101 012c    	add.w	r1, r1, #0x2c
 80059b8: 9120         	str	r1, [sp, #0x80]
 80059ba: 9921         	ldr	r1, [sp, #0x84]
 80059bc: f109 092c    	add.w	r9, r9, #0x2c
 80059c0: f100 002c    	add.w	r0, r0, #0x2c
 80059c4: f102 022c    	add.w	r2, r2, #0x2c
 80059c8: f501 7184    	add.w	r1, r1, #0x108
 80059cc: 9121         	str	r1, [sp, #0x84]
 80059ce: 9922         	ldr	r1, [sp, #0x88]
 80059d0: 9219         	str	r2, [sp, #0x64]
 80059d2: f501 7184    	add.w	r1, r1, #0x108
 80059d6: 9122         	str	r1, [sp, #0x88]
 80059d8: 9923         	ldr	r1, [sp, #0x8c]
 80059da: f501 7184    	add.w	r1, r1, #0x108
 80059de: 9123         	str	r1, [sp, #0x8c]
 80059e0: 991e         	ldr	r1, [sp, #0x78]
 80059e2: f101 012c    	add.w	r1, r1, #0x2c
 80059e6: 911e         	str	r1, [sp, #0x78]
 80059e8: f47f aed7    	bne.w	0x800579a <raylar_crcl_encoder_service::benchmark::prepare::hb9734b8d5dc7f78e+0x934> @ imm = #-0x252
 80059ec: 9805         	ldr	r0, [sp, #0x14]
 80059ee: f1b8 0f00    	cmp.w	r8, #0x0
 80059f2: f8c0 8008    	str.w	r8, [r0, #0x8]
 80059f6: d017         	beq	0x8005a28 <raylar_crcl_encoder_service::benchmark::prepare::hb9734b8d5dc7f78e+0xbc2> @ imm = #0x2e
 80059f8: f241 0005    	movw	r0, #0x1005
 80059fc: f2c0 0040    	movt	r0, #0x40
 8005a00: fba8 0100    	umull	r0, r1, r8, r0
 8005a04: eba8 0001    	sub.w	r0, r8, r1
 8005a08: eb01 0050    	add.w	r0, r1, r0, lsr #1
 8005a0c: 2101         	movs	r1, #0x1
 8005a0e: 0a42         	lsrs	r2, r0, #0x9
 8005a10: eba2 2382    	sub.w	r3, r2, r2, lsl #10
 8005a14: eb13 0308    	adds.w	r3, r3, r8
 8005a18: bf18         	it	ne
 8005a1a: eb01 2250    	addne.w	r2, r1, r0, lsr #9
 8005a1e: fab2 f082    	clz	r0, r2
 8005a22: f1c0 001f    	rsb.w	r0, r0, #0x1f
 8005a26: e000         	b	0x8005a2a <raylar_crcl_encoder_service::benchmark::prepare::hb9734b8d5dc7f78e+0xbc4> @ imm = #0x0
 8005a28: 2000         	movs	r0, #0x0
 8005a2a: 9924         	ldr	r1, [sp, #0x90]
 8005a2c: f24f 0204    	movw	r2, #0xf004
 8005a30: f6cf 72ff    	movt	r2, #0xffff
 8005a34: f88d 0150    	strb.w	r0, [sp, #0x150]
 8005a38: b2c0         	uxtb	r0, r0
 8005a3a: 7d09         	ldrb	r1, [r1, #0x14]
 8005a3c: 4288         	cmp	r0, r1
 8005a3e: f040 838f    	bne.w	0x8006160 <raylar_crcl_encoder_service::benchmark::prepare::hb9734b8d5dc7f78e+0x12fa> @ imm = #0x71e
 8005a42: 9804         	ldr	r0, [sp, #0x10]
 8005a44: f644 7ef0    	movw	lr, #0x4ff0
 8005a48: 9905         	ldr	r1, [sp, #0x14]
 8005a4a: f2c0 0e01    	movt	lr, #0x1
 8005a4e: f10e 0320    	add.w	r3, lr, #0x20
 8005a52: f10e 061c    	add.w	r6, lr, #0x1c
 8005a56: 6980         	ldr	r0, [r0, #0x18]
 8005a58: f10e 0908    	add.w	r9, lr, #0x8
 8005a5c: 61c8         	str	r0, [r1, #0x1c]
 8005a5e: f24c 0108    	movw	r1, #0xc008
 8005a62: f10e 0028    	add.w	r0, lr, #0x28
 8005a66: 9117         	str	r1, [sp, #0x5c]
 8005a68: f64b 7178    	movw	r1, #0xbf78
 8005a6c: 901a         	str	r0, [sp, #0x68]
 8005a6e: f10e 0024    	add.w	r0, lr, #0x24
 8005a72: 9122         	str	r1, [sp, #0x88]
 8005a74: f64b 71f0    	movw	r1, #0xbff0
 8005a78: 9019         	str	r0, [sp, #0x64]
 8005a7a: f10e 0018    	add.w	r0, lr, #0x18
 8005a7e: 9116         	str	r1, [sp, #0x58]
 8005a80: f64b 7190    	movw	r1, #0xbf90
 8005a84: 9018         	str	r0, [sp, #0x60]
 8005a86: f10e 000c    	add.w	r0, lr, #0xc
 8005a8a: 9121         	str	r1, [sp, #0x84]
 8005a8c: f64b 71c0    	movw	r1, #0xbfc0
 8005a90: 9023         	str	r0, [sp, #0x8c]
 8005a92: 2000         	movs	r0, #0x0
 8005a94: 9115         	str	r1, [sp, #0x54]
 8005a96: f44f 31a8    	mov.w	r1, #0x15000
 8005a9a: f10e 0804    	add.w	r8, lr, #0x4
 8005a9e: 901b         	str	r0, [sp, #0x6c]
 8005aa0: f64b 7030    	movw	r0, #0xbf30
 8005aa4: 9120         	str	r1, [sp, #0x80]
 8005aa6: f64b 71a8    	movw	r1, #0xbfa8
 8005aaa: f64b 7560    	movw	r5, #0xbf60
 8005aae: f64b 7ad8    	movw	r10, #0xbfd8
 8005ab2: 4694         	mov	r12, r2
 8005ab4: 911f         	str	r1, [sp, #0x7c]
 8005ab6: 9c24         	ldr	r4, [sp, #0x90]
 8005ab8: e9cd 631c    	strd	r6, r3, [sp, #112]
 8005abc: 1826         	adds	r6, r4, r0
 8005abe: f8cd e078    	str.w	lr, [sp, #0x78]
 8005ac2: e9cd 5013    	strd	r5, r0, [sp, #76]
 8005ac6: eb04 0e0c    	add.w	lr, r4, r12
 8005aca: f8d6 0108    	ldr.w	r0, [r6, #0x108]
 8005ace: f8d6 1120    	ldr.w	r1, [r6, #0x120]
 8005ad2: 9a1e         	ldr	r2, [sp, #0x78]
 8005ad4: 1a08         	subs	r0, r1, r0
 8005ad6: 960b         	str	r6, [sp, #0x2c]
 8005ad8: 18a3         	adds	r3, r4, r2
 8005ada: f8dd b054    	ldr.w	r11, [sp, #0x54]
 8005ade: 0041         	lsls	r1, r0, #0x1
 8005ae0: e9cd 9a11    	strd	r9, r10, [sp, #68]
 8005ae4: ea81 71e0    	eor.w	r1, r1, r0, asr #31
 8005ae8: 62d8         	str	r0, [r3, #0x2c]
 8005aea: f24b 0004    	movw	r0, #0xb004
 8005aee: 910e         	str	r1, [sp, #0x38]
 8005af0: f2c0 0001    	movt	r0, #0x1
 8005af4: e9cd c80f    	strd	r12, r8, [sp, #60]
 8005af8: f84e 1000    	str.w	r1, [lr, r0]
 8005afc: 1961         	adds	r1, r4, r5
 8005afe: f8d6 0120    	ldr.w	r0, [r6, #0x120]
 8005b02: f8d1 3108    	ldr.w	r3, [r1, #0x108]
 8005b06: 1a18         	subs	r0, r3, r0
 8005b08: eb04 0308    	add.w	r3, r4, r8
 8005b0c: f8dd 8058    	ldr.w	r8, [sp, #0x58]
 8005b10: 62d8         	str	r0, [r3, #0x2c]
 8005b12: 0043         	lsls	r3, r0, #0x1
 8005b14: ea83 72e0    	eor.w	r2, r3, r0, asr #31
 8005b18: f24b 0008    	movw	r0, #0xb008
 8005b1c: f2c0 0001    	movt	r0, #0x1
 8005b20: 920d         	str	r2, [sp, #0x34]
 8005b22: f84e 2000    	str.w	r2, [lr, r0]
 8005b26: f8d1 0108    	ldr.w	r0, [r1, #0x108]
 8005b2a: 9922         	ldr	r1, [sp, #0x88]
 8005b2c: 4421         	add	r1, r4
 8005b2e: f8d1 3108    	ldr.w	r3, [r1, #0x108]
 8005b32: 1a18         	subs	r0, r3, r0
 8005b34: eb04 0309    	add.w	r3, r4, r9
 8005b38: f8dd 9060    	ldr.w	r9, [sp, #0x60]
 8005b3c: 62d8         	str	r0, [r3, #0x2c]
 8005b3e: 0043         	lsls	r3, r0, #0x1
 8005b40: ea83 72e0    	eor.w	r2, r3, r0, asr #31
 8005b44: f24b 000c    	movw	r0, #0xb00c
 8005b48: f2c0 0001    	movt	r0, #0x1
 8005b4c: 920c         	str	r2, [sp, #0x30]
 8005b4e: f84e 2000    	str.w	r2, [lr, r0]
 8005b52: f8d1 0108    	ldr.w	r0, [r1, #0x108]
 8005b56: 9921         	ldr	r1, [sp, #0x84]
 8005b58: 9a23         	ldr	r2, [sp, #0x8c]
 8005b5a: 4421         	add	r1, r4
 8005b5c: f8d1 5108    	ldr.w	r5, [r1, #0x108]
 8005b60: 1a28         	subs	r0, r5, r0
 8005b62: 18a5         	adds	r5, r4, r2
 8005b64: 62e8         	str	r0, [r5, #0x2c]
 8005b66: 0045         	lsls	r5, r0, #0x1
 8005b68: ea85 72e0    	eor.w	r2, r5, r0, asr #31
 8005b6c: f24b 0010    	movw	r0, #0xb010
 8005b70: f2c0 0001    	movt	r0, #0x1
 8005b74: 920a         	str	r2, [sp, #0x28]
 8005b76: f84e 2000    	str.w	r2, [lr, r0]
 8005b7a: f8d1 0108    	ldr.w	r0, [r1, #0x108]
 8005b7e: 991f         	ldr	r1, [sp, #0x7c]
 8005b80: 9a20         	ldr	r2, [sp, #0x80]
 8005b82: 4421         	add	r1, r4
 8005b84: f8d1 6108    	ldr.w	r6, [r1, #0x108]
 8005b88: 1a30         	subs	r0, r6, r0
 8005b8a: 18a6         	adds	r6, r4, r2
 8005b8c: 62f0         	str	r0, [r6, #0x2c]
 8005b8e: 0046         	lsls	r6, r0, #0x1
 8005b90: ea86 72e0    	eor.w	r2, r6, r0, asr #31
 8005b94: f24b 0014    	movw	r0, #0xb014
 8005b98: f2c0 0001    	movt	r0, #0x1
 8005b9c: 9209         	str	r2, [sp, #0x24]
 8005b9e: f84e 2000    	str.w	r2, [lr, r0]
 8005ba2: f246 022c    	movw	r2, #0x602c
 8005ba6: f2c0 0201    	movt	r2, #0x1
 8005baa: f8d1 0108    	ldr.w	r0, [r1, #0x108]
 8005bae: eb04 010b    	add.w	r1, r4, r11
 8005bb2: 9e1d         	ldr	r6, [sp, #0x74]
 8005bb4: f50b 7b84    	add.w	r11, r11, #0x108
 8005bb8: f8d1 3108    	ldr.w	r3, [r1, #0x108]
 8005bbc: f8cd b054    	str.w	r11, [sp, #0x54]
 8005bc0: 1a18         	subs	r0, r3, r0
 8005bc2: f84e 0002    	str.w	r0, [lr, r2]
 8005bc6: 0043         	lsls	r3, r0, #0x1
 8005bc8: ea83 72e0    	eor.w	r2, r3, r0, asr #31
 8005bcc: f24b 0018    	movw	r0, #0xb018
 8005bd0: eb04 030a    	add.w	r3, r4, r10
 8005bd4: f2c0 0001    	movt	r0, #0x1
 8005bd8: 9208         	str	r2, [sp, #0x20]
 8005bda: f84e 2000    	str.w	r2, [lr, r0]
 8005bde: f8d1 0108    	ldr.w	r0, [r1, #0x108]
 8005be2: f8d3 1108    	ldr.w	r1, [r3, #0x108]
 8005be6: 9a1c         	ldr	r2, [sp, #0x70]
 8005be8: 1a08         	subs	r0, r1, r0
 8005bea: eb04 0109    	add.w	r1, r4, r9
 8005bee: 4422         	add	r2, r4
 8005bf0: f109 092c    	add.w	r9, r9, #0x2c
 8005bf4: 62c8         	str	r0, [r1, #0x2c]
 8005bf6: 0041         	lsls	r1, r0, #0x1
 8005bf8: ea81 71e0    	eor.w	r1, r1, r0, asr #31
 8005bfc: f24b 001c    	movw	r0, #0xb01c
 8005c00: f2c0 0001    	movt	r0, #0x1
 8005c04: f8cd 9060    	str.w	r9, [sp, #0x60]
 8005c08: f84e 1000    	str.w	r1, [lr, r0]
 8005c0c: f8d3 0108    	ldr.w	r0, [r3, #0x108]
 8005c10: eb04 0308    	add.w	r3, r4, r8
 8005c14: f508 7884    	add.w	r8, r8, #0x108
 8005c18: f8dd 9044    	ldr.w	r9, [sp, #0x44]
 8005c1c: f8d3 c108    	ldr.w	r12, [r3, #0x108]
 8005c20: f8cd 8058    	str.w	r8, [sp, #0x58]
 8005c24: f109 092c    	add.w	r9, r9, #0x2c
 8005c28: ebac 0000    	sub.w	r0, r12, r0
 8005c2c: 62d0         	str	r0, [r2, #0x2c]
 8005c2e: f8dd c064    	ldr.w	r12, [sp, #0x64]
 8005c32: 0042         	lsls	r2, r0, #0x1
 8005c34: f8dd 8040    	ldr.w	r8, [sp, #0x40]
 8005c38: ea82 72e0    	eor.w	r2, r2, r0, asr #31
 8005c3c: f24b 0020    	movw	r0, #0xb020
 8005c40: f2c0 0001    	movt	r0, #0x1
 8005c44: 9207         	str	r2, [sp, #0x1c]
 8005c46: f84e 2000    	str.w	r2, [lr, r0]
 8005c4a: 4620         	mov	r0, r4
 8005c4c: f108 082c    	add.w	r8, r8, #0x2c
 8005c50: 9c17         	ldr	r4, [sp, #0x5c]
 8005c52: f8d3 2108    	ldr.w	r2, [r3, #0x108]
 8005c56: 1903         	adds	r3, r0, r4
 8005c58: f504 7484    	add.w	r4, r4, #0x108
 8005c5c: 9417         	str	r4, [sp, #0x5c]
 8005c5e: f8d3 5108    	ldr.w	r5, [r3, #0x108]
 8005c62: 1aaa         	subs	r2, r5, r2
 8005c64: 1985         	adds	r5, r0, r6
 8005c66: 9e0b         	ldr	r6, [sp, #0x2c]
 8005c68: 62ea         	str	r2, [r5, #0x2c]
 8005c6a: 0055         	lsls	r5, r2, #0x1
 8005c6c: ea85 7ae2    	eor.w	r10, r5, r2, asr #31
 8005c70: f24b 0224    	movw	r2, #0xb024
 8005c74: f2c0 0201    	movt	r2, #0x1
 8005c78: f84e a002    	str.w	r10, [lr, r2]
 8005c7c: f24b 0228    	movw	r2, #0xb028
 8005c80: f2c0 0201    	movt	r2, #0x1
 8005c84: f8d3 3108    	ldr.w	r3, [r3, #0x108]
 8005c88: f8d6 51f8    	ldr.w	r5, [r6, #0x1f8]
 8005c8c: 1aeb         	subs	r3, r5, r3
 8005c8e: eb00 050c    	add.w	r5, r0, r12
 8005c92: f10c 0c2c    	add.w	r12, r12, #0x2c
 8005c96: f8cd c064    	str.w	r12, [sp, #0x64]
 8005c9a: 62eb         	str	r3, [r5, #0x2c]
 8005c9c: 005d         	lsls	r5, r3, #0x1
 8005c9e: ea85 73e3    	eor.w	r3, r5, r3, asr #31
 8005ca2: f84e 3002    	str.w	r3, [lr, r2]
 8005ca6: 9a1a         	ldr	r2, [sp, #0x68]
 8005ca8: f8dd c03c    	ldr.w	r12, [sp, #0x3c]
 8005cac: 1885         	adds	r5, r0, r2
 8005cae: f8d6 01f8    	ldr.w	r0, [r6, #0x1f8]
 8005cb2: f8d6 6210    	ldr.w	r6, [r6, #0x210]
 8005cb6: f11c 0c2c    	adds.w	r12, r12, #0x2c
 8005cba: f102 022c    	add.w	r2, r2, #0x2c
 8005cbe: 921a         	str	r2, [sp, #0x68]
 8005cc0: eba6 0000    	sub.w	r0, r6, r0
 8005cc4: 62e8         	str	r0, [r5, #0x2c]
 8005cc6: 9e1b         	ldr	r6, [sp, #0x6c]
 8005cc8: ea4f 0540    	lsl.w	r5, r0, #0x1
 8005ccc: ea85 70e0    	eor.w	r0, r5, r0, asr #31
 8005cd0: f24b 052c    	movw	r5, #0xb02c
 8005cd4: f2c0 0501    	movt	r5, #0x1
 8005cd8: f84e 0005    	str.w	r0, [lr, r5]
 8005cdc: 9d0e         	ldr	r5, [sp, #0x38]
 8005cde: f8dd e078    	ldr.w	lr, [sp, #0x78]
 8005ce2: 442e         	add	r6, r5
 8005ce4: 9d0d         	ldr	r5, [sp, #0x34]
 8005ce6: f10e 0e2c    	add.w	lr, lr, #0x2c
 8005cea: 442e         	add	r6, r5
 8005cec: 9d0c         	ldr	r5, [sp, #0x30]
 8005cee: 442e         	add	r6, r5
 8005cf0: 9d0a         	ldr	r5, [sp, #0x28]
 8005cf2: 442e         	add	r6, r5
 8005cf4: 9d09         	ldr	r5, [sp, #0x24]
 8005cf6: 442e         	add	r6, r5
 8005cf8: 9d08         	ldr	r5, [sp, #0x20]
 8005cfa: 442e         	add	r6, r5
 8005cfc: 9d1d         	ldr	r5, [sp, #0x74]
 8005cfe: 4431         	add	r1, r6
 8005d00: 9e07         	ldr	r6, [sp, #0x1c]
 8005d02: 4431         	add	r1, r6
 8005d04: 9e1c         	ldr	r6, [sp, #0x70]
 8005d06: 4451         	add	r1, r10
 8005d08: f8dd a048    	ldr.w	r10, [sp, #0x48]
 8005d0c: 4419         	add	r1, r3
 8005d0e: 9b22         	ldr	r3, [sp, #0x88]
 8005d10: 4408         	add	r0, r1
 8005d12: 991f         	ldr	r1, [sp, #0x7c]
 8005d14: 901b         	str	r0, [sp, #0x6c]
 8005d16: f503 7384    	add.w	r3, r3, #0x108
 8005d1a: f501 7184    	add.w	r1, r1, #0x108
 8005d1e: 911f         	str	r1, [sp, #0x7c]
 8005d20: 9920         	ldr	r1, [sp, #0x80]
 8005d22: f50a 7a84    	add.w	r10, r10, #0x108
 8005d26: 9814         	ldr	r0, [sp, #0x50]
 8005d28: f106 062c    	add.w	r6, r6, #0x2c
 8005d2c: f101 012c    	add.w	r1, r1, #0x2c
 8005d30: 9120         	str	r1, [sp, #0x80]
 8005d32: 9923         	ldr	r1, [sp, #0x8c]
 8005d34: f500 7084    	add.w	r0, r0, #0x108
 8005d38: 9322         	str	r3, [sp, #0x88]
 8005d3a: f105 032c    	add.w	r3, r5, #0x2c
 8005d3e: f101 012c    	add.w	r1, r1, #0x2c
 8005d42: 9123         	str	r1, [sp, #0x8c]
 8005d44: 9921         	ldr	r1, [sp, #0x84]
 8005d46: f501 7184    	add.w	r1, r1, #0x108
 8005d4a: 9121         	str	r1, [sp, #0x84]
 8005d4c: 9913         	ldr	r1, [sp, #0x4c]
 8005d4e: f501 7584    	add.w	r5, r1, #0x108
 8005d52: f47f aeb0    	bne.w	0x8005ab6 <raylar_crcl_encoder_service::benchmark::prepare::hb9734b8d5dc7f78e+0xc50> @ imm = #-0x2a0
 8005d56: 9805         	ldr	r0, [sp, #0x14]
 8005d58: 991b         	ldr	r1, [sp, #0x6c]
 8005d5a: 60c1         	str	r1, [r0, #0xc]
 8005d5c: b1b1         	cbz	r1, 0x8005d8c <raylar_crcl_encoder_service::benchmark::prepare::hb9734b8d5dc7f78e+0xf26> @ imm = #0x2c
 8005d5e: 9e1b         	ldr	r6, [sp, #0x6c]
 8005d60: f241 0005    	movw	r0, #0x1005
 8005d64: f2c0 0040    	movt	r0, #0x40
 8005d68: fba6 0100    	umull	r0, r1, r6, r0
 8005d6c: 1a70         	subs	r0, r6, r1
 8005d6e: eb01 0050    	add.w	r0, r1, r0, lsr #1
 8005d72: 2101         	movs	r1, #0x1
 8005d74: 0a42         	lsrs	r2, r0, #0x9
 8005d76: eba2 2382    	sub.w	r3, r2, r2, lsl #10
 8005d7a: 199b         	adds	r3, r3, r6
 8005d7c: bf18         	it	ne
 8005d7e: eb01 2250    	addne.w	r2, r1, r0, lsr #9
 8005d82: fab2 f082    	clz	r0, r2
 8005d86: f1c0 001f    	rsb.w	r0, r0, #0x1f
 8005d8a: e000         	b	0x8005d8e <raylar_crcl_encoder_service::benchmark::prepare::hb9734b8d5dc7f78e+0xf28> @ imm = #0x0
 8005d8c: 2000         	movs	r0, #0x0
 8005d8e: 9924         	ldr	r1, [sp, #0x90]
 8005d90: f24f 0c04    	movw	r12, #0xf004
 8005d94: f6cf 7cff    	movt	r12, #0xffff
 8005d98: f88d 0150    	strb.w	r0, [sp, #0x150]
 8005d9c: b2c0         	uxtb	r0, r0
 8005d9e: 7f09         	ldrb	r1, [r1, #0x1c]
 8005da0: 4288         	cmp	r0, r1
 8005da2: f040 81e0    	bne.w	0x8006166 <raylar_crcl_encoder_service::benchmark::prepare::hb9734b8d5dc7f78e+0x1300> @ imm = #0x3c0
 8005da6: 9804         	ldr	r0, [sp, #0x10]
 8005da8: f64b 7364    	movw	r3, #0xbf64
 8005dac: 9905         	ldr	r1, [sp, #0x14]
 8005dae: f44f 34d8    	mov.w	r4, #0x1b000
 8005db2: 9322         	str	r3, [sp, #0x88]
 8005db4: f24c 030c    	movw	r3, #0xc00c
 8005db8: 6a40         	ldr	r0, [r0, #0x24]
 8005dba: f64b 7adc    	movw	r10, #0xbfdc
 8005dbe: 6208         	str	r0, [r1, #0x20]
 8005dc0: f645 70ec    	movw	r0, #0x5fec
 8005dc4: f2c0 0001    	movt	r0, #0x1
 8005dc8: f100 0128    	add.w	r1, r0, #0x28
 8005dcc: 9318         	str	r3, [sp, #0x60]
 8005dce: f64b 737c    	movw	r3, #0xbf7c
 8005dd2: 911b         	str	r1, [sp, #0x6c]
 8005dd4: f100 0124    	add.w	r1, r0, #0x24
 8005dd8: 9321         	str	r3, [sp, #0x84]
 8005dda: f64b 73f4    	movw	r3, #0xbff4
 8005dde: 911a         	str	r1, [sp, #0x68]
 8005de0: f100 0120    	add.w	r1, r0, #0x20
 8005de4: 9317         	str	r3, [sp, #0x5c]
 8005de6: f64b 7394    	movw	r3, #0xbf94
 8005dea: 9119         	str	r1, [sp, #0x64]
 8005dec: f100 0110    	add.w	r1, r0, #0x10
 8005df0: 9320         	str	r3, [sp, #0x80]
 8005df2: f44f 33b0    	mov.w	r3, #0x16000
 8005df6: 9123         	str	r1, [sp, #0x8c]
 8005df8: 2100         	movs	r1, #0x0
 8005dfa: 9316         	str	r3, [sp, #0x58]
 8005dfc: f64b 73c4    	movw	r3, #0xbfc4
 8005e00: f100 081c    	add.w	r8, r0, #0x1c
 8005e04: f100 060c    	add.w	r6, r0, #0xc
 8005e08: f100 0b08    	add.w	r11, r0, #0x8
 8005e0c: f100 0904    	add.w	r9, r0, #0x4
 8005e10: 911c         	str	r1, [sp, #0x70]
 8005e12: f64b 7134    	movw	r1, #0xbf34
 8005e16: 931f         	str	r3, [sp, #0x7c]
 8005e18: f64b 73ac    	movw	r3, #0xbfac
 8005e1c: 931e         	str	r3, [sp, #0x78]
 8005e1e: 9b24         	ldr	r3, [sp, #0x90]
 8005e20: 46d6         	mov	lr, r10
 8005e22: e9cd a110    	strd	r10, r1, [sp, #64]
 8005e26: 4632         	mov	r2, r6
 8005e28: eb03 0a01    	add.w	r10, r3, r1
 8005e2c: e9cd 6813    	strd	r6, r8, [sp, #76]
 8005e30: 1819         	adds	r1, r3, r0
 8005e32: 9015         	str	r0, [sp, #0x54]
 8005e34: f8da 6108    	ldr.w	r6, [r10, #0x108]
 8005e38: f8da 5120    	ldr.w	r5, [r10, #0x120]
 8005e3c: e9cd 9b0e    	strd	r9, r11, [sp, #56]
 8005e40: 1bae         	subs	r6, r5, r6
 8005e42: 62ce         	str	r6, [r1, #0x2c]
 8005e44: f8cd c074    	str.w	r12, [sp, #0x74]
 8005e48: 46c4         	mov	r12, r8
 8005e4a: 0071         	lsls	r1, r6, #0x1
 8005e4c: ea81 70e6    	eor.w	r0, r1, r6, asr #31
 8005e50: 191e         	adds	r6, r3, r4
 8005e52: 900d         	str	r0, [sp, #0x34]
 8005e54: f8dd 805c    	ldr.w	r8, [sp, #0x5c]
 8005e58: 6070         	str	r0, [r6, #0x4]
 8005e5a: 9822         	ldr	r0, [sp, #0x88]
 8005e5c: f8da 1120    	ldr.w	r1, [r10, #0x120]
 8005e60: 181d         	adds	r5, r3, r0
 8005e62: 9412         	str	r4, [sp, #0x48]
 8005e64: 9c19         	ldr	r4, [sp, #0x64]
 8005e66: f8d5 0108    	ldr.w	r0, [r5, #0x108]
 8005e6a: 1a40         	subs	r0, r0, r1
 8005e6c: eb03 0109    	add.w	r1, r3, r9
 8005e70: 62c8         	str	r0, [r1, #0x2c]
 8005e72: 0041         	lsls	r1, r0, #0x1
 8005e74: ea81 70e0    	eor.w	r0, r1, r0, asr #31
 8005e78: 9921         	ldr	r1, [sp, #0x84]
 8005e7a: 60b0         	str	r0, [r6, #0x8]
 8005e7c: 4419         	add	r1, r3
 8005e7e: 900c         	str	r0, [sp, #0x30]
 8005e80: f8d5 0108    	ldr.w	r0, [r5, #0x108]
 8005e84: f8d1 5108    	ldr.w	r5, [r1, #0x108]
 8005e88: 1a28         	subs	r0, r5, r0
 8005e8a: eb03 050b    	add.w	r5, r3, r11
 8005e8e: f8dd b058    	ldr.w	r11, [sp, #0x58]
 8005e92: 62e8         	str	r0, [r5, #0x2c]
 8005e94: 0045         	lsls	r5, r0, #0x1
 8005e96: ea85 70e0    	eor.w	r0, r5, r0, asr #31
 8005e9a: 60f0         	str	r0, [r6, #0xc]
 8005e9c: 900b         	str	r0, [sp, #0x2c]
 8005e9e: f8d1 0108    	ldr.w	r0, [r1, #0x108]
 8005ea2: 9920         	ldr	r1, [sp, #0x80]
 8005ea4: 4419         	add	r1, r3
 8005ea6: f8d1 5108    	ldr.w	r5, [r1, #0x108]
 8005eaa: 1a28         	subs	r0, r5, r0
 8005eac: 189d         	adds	r5, r3, r2
 8005eae: 62e8         	str	r0, [r5, #0x2c]
 8005eb0: 0045         	lsls	r5, r0, #0x1
 8005eb2: ea85 70e0    	eor.w	r0, r5, r0, asr #31
 8005eb6: 6130         	str	r0, [r6, #0x10]
 8005eb8: 900a         	str	r0, [sp, #0x28]
 8005eba: f247 052c    	movw	r5, #0x702c
 8005ebe: f8d1 0108    	ldr.w	r0, [r1, #0x108]
 8005ec2: f2c0 0501    	movt	r5, #0x1
 8005ec6: 991e         	ldr	r1, [sp, #0x78]
 8005ec8: 4419         	add	r1, r3
 8005eca: f8d1 2108    	ldr.w	r2, [r1, #0x108]
 8005ece: 1a10         	subs	r0, r2, r0
 8005ed0: 9a23         	ldr	r2, [sp, #0x8c]
 8005ed2: 441a         	add	r2, r3
 8005ed4: 62d0         	str	r0, [r2, #0x2c]
 8005ed6: 0042         	lsls	r2, r0, #0x1
 8005ed8: ea82 70e0    	eor.w	r0, r2, r0, asr #31
 8005edc: 6170         	str	r0, [r6, #0x14]
 8005ede: 9009         	str	r0, [sp, #0x24]
 8005ee0: f8d1 0108    	ldr.w	r0, [r1, #0x108]
 8005ee4: 991f         	ldr	r1, [sp, #0x7c]
 8005ee6: 4419         	add	r1, r3
 8005ee8: f8d1 2108    	ldr.w	r2, [r1, #0x108]
 8005eec: 1a10         	subs	r0, r2, r0
 8005eee: eb03 020b    	add.w	r2, r3, r11
 8005ef2: f10b 0b2c    	add.w	r11, r11, #0x2c
 8005ef6: f8cd b058    	str.w	r11, [sp, #0x58]
 8005efa: 62d0         	str	r0, [r2, #0x2c]
 8005efc: 0042         	lsls	r2, r0, #0x1
 8005efe: ea82 70e0    	eor.w	r0, r2, r0, asr #31
 8005f02: 61b0         	str	r0, [r6, #0x18]
 8005f04: 9008         	str	r0, [sp, #0x20]
 8005f06: f8d1 0108    	ldr.w	r0, [r1, #0x108]
 8005f0a: eb03 010e    	add.w	r1, r3, lr
 8005f0e: f8dd e068    	ldr.w	lr, [sp, #0x68]
 8005f12: f8d1 2108    	ldr.w	r2, [r1, #0x108]
 8005f16: 1a10         	subs	r0, r2, r0
 8005f18: 9a1d         	ldr	r2, [sp, #0x74]
 8005f1a: 441a         	add	r2, r3
 8005f1c: 5150         	str	r0, [r2, r5]
 8005f1e: 0042         	lsls	r2, r0, #0x1
 8005f20: ea82 70e0    	eor.w	r0, r2, r0, asr #31
 8005f24: 61f0         	str	r0, [r6, #0x1c]
 8005f26: 9007         	str	r0, [sp, #0x1c]
 8005f28: f8d1 0108    	ldr.w	r0, [r1, #0x108]
 8005f2c: eb03 0108    	add.w	r1, r3, r8
 8005f30: 9d18         	ldr	r5, [sp, #0x60]
 8005f32: f508 7884    	add.w	r8, r8, #0x108
 8005f36: f8d1 2108    	ldr.w	r2, [r1, #0x108]
 8005f3a: f8cd 805c    	str.w	r8, [sp, #0x5c]
 8005f3e: 1a10         	subs	r0, r2, r0
 8005f40: eb03 020c    	add.w	r2, r3, r12
 8005f44: f8dd c06c    	ldr.w	r12, [sp, #0x6c]
 8005f48: 62d0         	str	r0, [r2, #0x2c]
 8005f4a: 0042         	lsls	r2, r0, #0x1
 8005f4c: ea82 70e0    	eor.w	r0, r2, r0, asr #31
 8005f50: 6230         	str	r0, [r6, #0x20]
 8005f52: 9006         	str	r0, [sp, #0x18]
 8005f54: f8d1 0108    	ldr.w	r0, [r1, #0x108]
 8005f58: 1959         	adds	r1, r3, r5
 8005f5a: f505 7584    	add.w	r5, r5, #0x108
 8005f5e: 9518         	str	r5, [sp, #0x60]
 8005f60: f8d1 2108    	ldr.w	r2, [r1, #0x108]
 8005f64: 1a10         	subs	r0, r2, r0
 8005f66: 191a         	adds	r2, r3, r4
 8005f68: 342c         	adds	r4, #0x2c
 8005f6a: 9419         	str	r4, [sp, #0x64]
 8005f6c: 62d0         	str	r0, [r2, #0x2c]
 8005f6e: 0042         	lsls	r2, r0, #0x1
 8005f70: ea82 79e0    	eor.w	r9, r2, r0, asr #31
 8005f74: f8c6 9024    	str.w	r9, [r6, #0x24]
 8005f78: f8d1 1108    	ldr.w	r1, [r1, #0x108]
 8005f7c: f8da 21f8    	ldr.w	r2, [r10, #0x1f8]
 8005f80: 9c12         	ldr	r4, [sp, #0x48]
 8005f82: 1a51         	subs	r1, r2, r1
 8005f84: eb03 020e    	add.w	r2, r3, lr
 8005f88: 342c         	adds	r4, #0x2c
 8005f8a: f10e 0e2c    	add.w	lr, lr, #0x2c
 8005f8e: 62d1         	str	r1, [r2, #0x2c]
 8005f90: 004a         	lsls	r2, r1, #0x1
 8005f92: ea82 71e1    	eor.w	r1, r2, r1, asr #31
 8005f96: 62b1         	str	r1, [r6, #0x28]
 8005f98: eb03 020c    	add.w	r2, r3, r12
 8005f9c: f8da 01f8    	ldr.w	r0, [r10, #0x1f8]
 8005fa0: f8da 3210    	ldr.w	r3, [r10, #0x210]
 8005fa4: f10c 0c2c    	add.w	r12, r12, #0x2c
 8005fa8: f8dd a040    	ldr.w	r10, [sp, #0x40]
 8005fac: 1a18         	subs	r0, r3, r0
 8005fae: 62d0         	str	r0, [r2, #0x2c]
 8005fb0: 9b0d         	ldr	r3, [sp, #0x34]
 8005fb2: f50a 7a84    	add.w	r10, r10, #0x108
 8005fb6: 0042         	lsls	r2, r0, #0x1
 8005fb8: ea82 70e0    	eor.w	r0, r2, r0, asr #31
 8005fbc: 9a1c         	ldr	r2, [sp, #0x70]
 8005fbe: 62f0         	str	r0, [r6, #0x2c]
 8005fc0: 441a         	add	r2, r3
 8005fc2: 9b0c         	ldr	r3, [sp, #0x30]
 8005fc4: 9e13         	ldr	r6, [sp, #0x4c]
 8005fc6: 441a         	add	r2, r3
 8005fc8: 9b0b         	ldr	r3, [sp, #0x2c]
 8005fca: f8cd c06c    	str.w	r12, [sp, #0x6c]
 8005fce: 362c         	adds	r6, #0x2c
 8005fd0: 441a         	add	r2, r3
 8005fd2: 9b0a         	ldr	r3, [sp, #0x28]
 8005fd4: f8dd c074    	ldr.w	r12, [sp, #0x74]
 8005fd8: 441a         	add	r2, r3
 8005fda: 9b09         	ldr	r3, [sp, #0x24]
 8005fdc: f11c 0c2c    	adds.w	r12, r12, #0x2c
 8005fe0: f8cd e068    	str.w	lr, [sp, #0x68]
 8005fe4: 441a         	add	r2, r3
 8005fe6: 9b08         	ldr	r3, [sp, #0x20]
 8005fe8: 441a         	add	r2, r3
 8005fea: 9b07         	ldr	r3, [sp, #0x1c]
 8005fec: 441a         	add	r2, r3
 8005fee: 9b06         	ldr	r3, [sp, #0x18]
 8005ff0: 441a         	add	r2, r3
 8005ff2: 9b14         	ldr	r3, [sp, #0x50]
 8005ff4: 444a         	add	r2, r9
 8005ff6: e9dd 9b0e    	ldrd	r9, r11, [sp, #56]
 8005ffa: 4411         	add	r1, r2
 8005ffc: 9a1e         	ldr	r2, [sp, #0x78]
 8005ffe: 4408         	add	r0, r1
 8006000: f103 082c    	add.w	r8, r3, #0x2c
 8006004: f502 7284    	add.w	r2, r2, #0x108
 8006008: 921e         	str	r2, [sp, #0x78]
 800600a: 9a23         	ldr	r2, [sp, #0x8c]
 800600c: f10b 0b2c    	add.w	r11, r11, #0x2c
 8006010: 9b21         	ldr	r3, [sp, #0x84]
 8006012: f109 092c    	add.w	r9, r9, #0x2c
 8006016: f102 022c    	add.w	r2, r2, #0x2c
 800601a: 9223         	str	r2, [sp, #0x8c]
 800601c: 9a1f         	ldr	r2, [sp, #0x7c]
 800601e: f503 7384    	add.w	r3, r3, #0x108
 8006022: 9911         	ldr	r1, [sp, #0x44]
 8006024: 901c         	str	r0, [sp, #0x70]
 8006026: f502 7284    	add.w	r2, r2, #0x108
 800602a: 9815         	ldr	r0, [sp, #0x54]
 800602c: f501 7184    	add.w	r1, r1, #0x108
 8006030: 921f         	str	r2, [sp, #0x7c]
 8006032: 9a20         	ldr	r2, [sp, #0x80]
 8006034: f100 002c    	add.w	r0, r0, #0x2c
 8006038: 9321         	str	r3, [sp, #0x84]
 800603a: 9b22         	ldr	r3, [sp, #0x88]
 800603c: f502 7284    	add.w	r2, r2, #0x108
 8006040: 9220         	str	r2, [sp, #0x80]
 8006042: f503 7384    	add.w	r3, r3, #0x108
 8006046: 9322         	str	r3, [sp, #0x88]
 8006048: f47f aee9    	bne.w	0x8005e1e <raylar_crcl_encoder_service::benchmark::prepare::hb9734b8d5dc7f78e+0xfb8> @ imm = #-0x22e
 800604c: 9805         	ldr	r0, [sp, #0x14]
 800604e: 991c         	ldr	r1, [sp, #0x70]
 8006050: 6101         	str	r1, [r0, #0x10]
 8006052: b1b1         	cbz	r1, 0x8006082 <raylar_crcl_encoder_service::benchmark::prepare::hb9734b8d5dc7f78e+0x121c> @ imm = #0x2c
 8006054: 9e1c         	ldr	r6, [sp, #0x70]
 8006056: f241 0005    	movw	r0, #0x1005
 800605a: f2c0 0040    	movt	r0, #0x40
 800605e: fba6 0100    	umull	r0, r1, r6, r0
 8006062: 1a70         	subs	r0, r6, r1
 8006064: eb01 0050    	add.w	r0, r1, r0, lsr #1
 8006068: 2101         	movs	r1, #0x1
 800606a: 0a42         	lsrs	r2, r0, #0x9
 800606c: eba2 2382    	sub.w	r3, r2, r2, lsl #10
 8006070: 199b         	adds	r3, r3, r6
 8006072: bf18         	it	ne
 8006074: eb01 2250    	addne.w	r2, r1, r0, lsr #9
 8006078: fab2 f082    	clz	r0, r2
 800607c: f1c0 001f    	rsb.w	r0, r0, #0x1f
 8006080: e000         	b	0x8006084 <raylar_crcl_encoder_service::benchmark::prepare::hb9734b8d5dc7f78e+0x121e> @ imm = #0x0
 8006082: 2000         	movs	r0, #0x0
 8006084: 9924         	ldr	r1, [sp, #0x90]
 8006086: f88d 0150    	strb.w	r0, [sp, #0x150]
 800608a: b2c0         	uxtb	r0, r0
 800608c: f891 1024    	ldrb.w	r1, [r1, #0x24]
 8006090: 4288         	cmp	r0, r1
 8006092: d16b         	bne	0x800616c <raylar_crcl_encoder_service::benchmark::prepare::hb9734b8d5dc7f78e+0x1306> @ imm = #0xd6
 8006094: 9804         	ldr	r0, [sp, #0x10]
 8006096: f24c 0288    	movw	r2, #0xc088
 800609a: 9905         	ldr	r1, [sp, #0x14]
 800609c: f641 0c06    	movw	r12, #0x1806
 80060a0: 6b00         	ldr	r0, [r0, #0x30]
 80060a2: 6248         	str	r0, [r1, #0x24]
 80060a4: 2000         	movs	r0, #0x0
 80060a6: 9924         	ldr	r1, [sp, #0x90]
 80060a8: 188e         	adds	r6, r1, r2
 80060aa: eb01 0380    	add.w	r3, r1, r0, lsl #2
 80060ae: f856 5c60    	ldr	r5, [r6, #-96]
 80060b2: 6a99         	ldr	r1, [r3, #0x28]
 80060b4: 42a9         	cmp	r1, r5
 80060b6: d138         	bne	0x800612a <raylar_crcl_encoder_service::benchmark::prepare::hb9734b8d5dc7f78e+0x12c4> @ imm = #0x70
 80060b8: f856 1c48    	ldr	r1, [r6, #-72]
 80060bc: 6c1d         	ldr	r5, [r3, #0x40]
 80060be: 428d         	cmp	r5, r1
 80060c0: d12e         	bne	0x8006120 <raylar_crcl_encoder_service::benchmark::prepare::hb9734b8d5dc7f78e+0x12ba> @ imm = #0x5c
 80060c2: f856 1c30    	ldr	r1, [r6, #-48]
 80060c6: 6d9d         	ldr	r5, [r3, #0x58]
 80060c8: 428d         	cmp	r5, r1
 80060ca: d12b         	bne	0x8006124 <raylar_crcl_encoder_service::benchmark::prepare::hb9734b8d5dc7f78e+0x12be> @ imm = #0x56
 80060cc: f856 1c18    	ldr	r1, [r6, #-24]
 80060d0: 6f1e         	ldr	r6, [r3, #0x70]
 80060d2: 428e         	cmp	r6, r1
 80060d4: d128         	bne	0x8006128 <raylar_crcl_encoder_service::benchmark::prepare::hb9734b8d5dc7f78e+0x12c2> @ imm = #0x50
 80060d6: 301e         	adds	r0, #0x1e
 80060d8: 4560         	cmp	r0, r12
 80060da: d00a         	beq	0x80060f2 <raylar_crcl_encoder_service::benchmark::prepare::hb9734b8d5dc7f78e+0x128c> @ imm = #0x14
 80060dc: 9e24         	ldr	r6, [sp, #0x90]
 80060de: f102 0178    	add.w	r1, r2, #0x78
 80060e2: f8d3 3088    	ldr.w	r3, [r3, #0x88]
 80060e6: 58b2         	ldr	r2, [r6, r2]
 80060e8: 4293         	cmp	r3, r2
 80060ea: 460a         	mov	r2, r1
 80060ec: d0db         	beq	0x80060a6 <raylar_crcl_encoder_service::benchmark::prepare::hb9734b8d5dc7f78e+0x1240> @ imm = #-0x4a
 80060ee: 3806         	subs	r0, #0x6
 80060f0: e01b         	b	0x800612a <raylar_crcl_encoder_service::benchmark::prepare::hb9734b8d5dc7f78e+0x12c4> @ imm = #0x36
 80060f2: b079         	add	sp, #0x1e4
 80060f4: e8bd 0f00    	pop.w	{r8, r9, r10, r11}
 80060f8: bdf0         	pop	{r4, r5, r6, r7, pc}
 80060fa: f64d 6090    	movw	r0, #0xde90
 80060fe: f88d 1150    	strb.w	r1, [sp, #0x150]
 8006102: f6c0 0000    	movt	r0, #0x800
 8006106: f64d 434c    	movw	r3, #0xdc4c
 800610a: 9000         	str	r0, [sp]
 800610c: f64d 30b0    	movw	r0, #0xdbb0
 8006110: aa54         	add	r2, sp, #0x150
 8006112: f6c0 0000    	movt	r0, #0x800
 8006116: f6c0 0300    	movt	r3, #0x800
 800611a: 212b         	movs	r1, #0x2b
 800611c: f005 fa82    	bl	0x800b624 <core::result::unwrap_failed> @ imm = #0x5504
 8006120: 3006         	adds	r0, #0x6
 8006122: e002         	b	0x800612a <raylar_crcl_encoder_service::benchmark::prepare::hb9734b8d5dc7f78e+0x12c4> @ imm = #0x4
 8006124: 300c         	adds	r0, #0xc
 8006126: e000         	b	0x800612a <raylar_crcl_encoder_service::benchmark::prepare::hb9734b8d5dc7f78e+0x12c4> @ imm = #0x0
 8006128: 3012         	adds	r0, #0x12
 800612a: f64d 6160    	movw	r1, #0xde60
 800612e: 9a02         	ldr	r2, [sp, #0x8]
 8006130: f6c0 0100    	movt	r1, #0x800
 8006134: 2300         	movs	r3, #0x0
 8006136: 9101         	str	r1, [sp, #0x4]
 8006138: eb02 0280    	add.w	r2, r2, r0, lsl #2
 800613c: 9903         	ldr	r1, [sp, #0xc]
 800613e: eb01 0180    	add.w	r1, r1, r0, lsl #2
 8006142: 2000         	movs	r0, #0x0
 8006144: f002 fbbe    	bl	0x80088c4 <core::panicking::assert_failed::hef9a1b4b671d0724> @ imm = #0x277c
 8006148: f89d 0129    	ldrb.w	r0, [sp, #0x129]
 800614c: f88d 0150    	strb.w	r0, [sp, #0x150]
 8006150: f64d 6080    	movw	r0, #0xde80
 8006154: f6c0 0000    	movt	r0, #0x800
 8006158: e7d5         	b	0x8006106 <raylar_crcl_encoder_service::benchmark::prepare::hb9734b8d5dc7f78e+0x12a0> @ imm = #-0x56
 800615a: 9824         	ldr	r0, [sp, #0x90]
 800615c: 3008         	adds	r0, #0x8
 800615e: e007         	b	0x8006170 <raylar_crcl_encoder_service::benchmark::prepare::hb9734b8d5dc7f78e+0x130a> @ imm = #0xe
 8006160: 9824         	ldr	r0, [sp, #0x90]
 8006162: 3010         	adds	r0, #0x10
 8006164: e004         	b	0x8006170 <raylar_crcl_encoder_service::benchmark::prepare::hb9734b8d5dc7f78e+0x130a> @ imm = #0x8
 8006166: 9824         	ldr	r0, [sp, #0x90]
 8006168: 3018         	adds	r0, #0x18
 800616a: e001         	b	0x8006170 <raylar_crcl_encoder_service::benchmark::prepare::hb9734b8d5dc7f78e+0x130a> @ imm = #0x2
 800616c: 9824         	ldr	r0, [sp, #0x90]
 800616e: 3020         	adds	r0, #0x20
 8006170: 9024         	str	r0, [sp, #0x90]
 8006172: f64d 6070    	movw	r0, #0xde70
 8006176: a954         	add	r1, sp, #0x150
 8006178: f6c0 0000    	movt	r0, #0x800
 800617c: 2300         	movs	r3, #0x0
 800617e: 9001         	str	r0, [sp, #0x4]
 8006180: 9824         	ldr	r0, [sp, #0x90]
 8006182: 1d02         	adds	r2, r0, #0x4
 8006184: 2000         	movs	r0, #0x0
 8006186: f002 fb8f    	bl	0x80088a8 <core::panicking::assert_failed::h4992df13491a1642> @ imm = #0x271e

0800618a <raylar_crcl_encoder_service::benchmark::fold_sum::h16b920496cf2076e>:
 800618a: b5f0         	push	{r4, r5, r6, r7, lr}
 800618c: af03         	add	r7, sp, #0xc
 800618e: e92d 0f00    	push.w	{r8, r9, r10, r11}
 8006192: b083         	sub	sp, #0xc
 8006194: f500 3190    	add.w	r1, r0, #0x12000
 8006198: f04f 0800    	mov.w	r8, #0x0
 800619c: 3124         	adds	r1, #0x24
 800619e: f240 3eff    	movw	lr, #0x3ff
 80061a2: f851 cf84    	ldr	r12, [r1, #132]!
 80061a6: f1be 0e21    	subs.w	lr, lr, #0x21
 80061aa: e951 5420    	ldrd	r5, r4, [r1, #-128]
 80061ae: ea4f 0644    	lsl.w	r6, r4, #0x1
 80061b2: e951 321e    	ldrd	r3, r2, [r1, #-120]
 80061b6: ea86 76e4    	eor.w	r6, r6, r4, asr #31
 80061ba: f851 9c08    	ldr	r9, [r1, #-8]
 80061be: ea4f 0445    	lsl.w	r4, r5, #0x1
 80061c2: ea84 74e5    	eor.w	r4, r4, r5, asr #31
 80061c6: eb04 0508    	add.w	r5, r4, r8
 80061ca: ea4f 0443    	lsl.w	r4, r3, #0x1
 80061ce: ea84 73e3    	eor.w	r3, r4, r3, asr #31
 80061d2: f851 4c70    	ldr	r4, [r1, #-112]
 80061d6: 442e         	add	r6, r5
 80061d8: ea4f 0542    	lsl.w	r5, r2, #0x1
 80061dc: 4433         	add	r3, r6
 80061de: ea85 72e2    	eor.w	r2, r5, r2, asr #31
 80061e2: f851 5c6c    	ldr	r5, [r1, #-108]
 80061e6: 441a         	add	r2, r3
 80061e8: ea4f 0344    	lsl.w	r3, r4, #0x1
 80061ec: f851 6c0c    	ldr	r6, [r1, #-12]
 80061f0: ea83 73e4    	eor.w	r3, r3, r4, asr #31
 80061f4: f851 4c68    	ldr	r4, [r1, #-104]
 80061f8: 441a         	add	r2, r3
 80061fa: ea4f 0345    	lsl.w	r3, r5, #0x1
 80061fe: ea83 73e5    	eor.w	r3, r3, r5, asr #31
 8006202: f851 5c64    	ldr	r5, [r1, #-100]
 8006206: 441a         	add	r2, r3
 8006208: ea4f 0344    	lsl.w	r3, r4, #0x1
 800620c: ea83 73e4    	eor.w	r3, r3, r4, asr #31
 8006210: f851 4c60    	ldr	r4, [r1, #-96]
 8006214: 441a         	add	r2, r3
 8006216: ea4f 0345    	lsl.w	r3, r5, #0x1
 800621a: ea83 73e5    	eor.w	r3, r3, r5, asr #31
 800621e: f851 5c5c    	ldr	r5, [r1, #-92]
 8006222: 441a         	add	r2, r3
 8006224: ea4f 0344    	lsl.w	r3, r4, #0x1
 8006228: ea83 73e4    	eor.w	r3, r3, r4, asr #31
 800622c: f851 4c58    	ldr	r4, [r1, #-88]
 8006230: 441a         	add	r2, r3
 8006232: ea4f 0345    	lsl.w	r3, r5, #0x1
 8006236: ea83 73e5    	eor.w	r3, r3, r5, asr #31
 800623a: f851 5c54    	ldr	r5, [r1, #-84]
 800623e: 441a         	add	r2, r3
 8006240: ea4f 0344    	lsl.w	r3, r4, #0x1
 8006244: ea83 73e4    	eor.w	r3, r3, r4, asr #31
 8006248: f851 4c50    	ldr	r4, [r1, #-80]
 800624c: 441a         	add	r2, r3
 800624e: ea4f 0345    	lsl.w	r3, r5, #0x1
 8006252: ea83 73e5    	eor.w	r3, r3, r5, asr #31
 8006256: f851 5c4c    	ldr	r5, [r1, #-76]
 800625a: 441a         	add	r2, r3
 800625c: ea4f 0344    	lsl.w	r3, r4, #0x1
 8006260: ea83 73e4    	eor.w	r3, r3, r4, asr #31
 8006264: f851 4c48    	ldr	r4, [r1, #-72]
 8006268: 441a         	add	r2, r3
 800626a: ea4f 0345    	lsl.w	r3, r5, #0x1
 800626e: ea83 73e5    	eor.w	r3, r3, r5, asr #31
 8006272: f851 5c44    	ldr	r5, [r1, #-68]
 8006276: 441a         	add	r2, r3
 8006278: ea4f 0344    	lsl.w	r3, r4, #0x1
 800627c: ea83 73e4    	eor.w	r3, r3, r4, asr #31
 8006280: f851 4c40    	ldr	r4, [r1, #-64]
 8006284: 441a         	add	r2, r3
 8006286: ea4f 0345    	lsl.w	r3, r5, #0x1
 800628a: ea83 73e5    	eor.w	r3, r3, r5, asr #31
 800628e: f851 5c3c    	ldr	r5, [r1, #-60]
 8006292: 441a         	add	r2, r3
 8006294: ea4f 0344    	lsl.w	r3, r4, #0x1
 8006298: ea83 73e4    	eor.w	r3, r3, r4, asr #31
 800629c: f851 4c38    	ldr	r4, [r1, #-56]
 80062a0: 441a         	add	r2, r3
 80062a2: ea4f 0345    	lsl.w	r3, r5, #0x1
 80062a6: ea83 73e5    	eor.w	r3, r3, r5, asr #31
 80062aa: f851 5c34    	ldr	r5, [r1, #-52]
 80062ae: 441a         	add	r2, r3
 80062b0: ea4f 0344    	lsl.w	r3, r4, #0x1
 80062b4: ea83 73e4    	eor.w	r3, r3, r4, asr #31
 80062b8: f851 4c30    	ldr	r4, [r1, #-48]
 80062bc: 441a         	add	r2, r3
 80062be: ea4f 0345    	lsl.w	r3, r5, #0x1
 80062c2: ea83 73e5    	eor.w	r3, r3, r5, asr #31
 80062c6: f851 5c2c    	ldr	r5, [r1, #-44]
 80062ca: 441a         	add	r2, r3
 80062cc: ea4f 0344    	lsl.w	r3, r4, #0x1
 80062d0: ea83 73e4    	eor.w	r3, r3, r4, asr #31
 80062d4: f851 4c28    	ldr	r4, [r1, #-40]
 80062d8: 441a         	add	r2, r3
 80062da: ea4f 0345    	lsl.w	r3, r5, #0x1
 80062de: ea83 73e5    	eor.w	r3, r3, r5, asr #31
 80062e2: f851 5c24    	ldr	r5, [r1, #-36]
 80062e6: 441a         	add	r2, r3
 80062e8: ea4f 0344    	lsl.w	r3, r4, #0x1
 80062ec: ea83 73e4    	eor.w	r3, r3, r4, asr #31
 80062f0: f851 4c20    	ldr	r4, [r1, #-32]
 80062f4: 441a         	add	r2, r3
 80062f6: ea4f 0345    	lsl.w	r3, r5, #0x1
 80062fa: ea83 73e5    	eor.w	r3, r3, r5, asr #31
 80062fe: f851 5c1c    	ldr	r5, [r1, #-28]
 8006302: 441a         	add	r2, r3
 8006304: ea4f 0344    	lsl.w	r3, r4, #0x1
 8006308: ea83 73e4    	eor.w	r3, r3, r4, asr #31
 800630c: f851 4c18    	ldr	r4, [r1, #-24]
 8006310: 441a         	add	r2, r3
 8006312: ea4f 0345    	lsl.w	r3, r5, #0x1
 8006316: ea83 73e5    	eor.w	r3, r3, r5, asr #31
 800631a: f851 5c14    	ldr	r5, [r1, #-20]
 800631e: 441a         	add	r2, r3
 8006320: ea4f 0344    	lsl.w	r3, r4, #0x1
 8006324: ea83 73e4    	eor.w	r3, r3, r4, asr #31
 8006328: f851 4c10    	ldr	r4, [r1, #-16]
 800632c: 441a         	add	r2, r3
 800632e: ea4f 0345    	lsl.w	r3, r5, #0x1
 8006332: ea83 73e5    	eor.w	r3, r3, r5, asr #31
 8006336: f851 8c04    	ldr	r8, [r1, #-4]
 800633a: 441a         	add	r2, r3
 800633c: ea4f 0344    	lsl.w	r3, r4, #0x1
 8006340: ea83 73e4    	eor.w	r3, r3, r4, asr #31
 8006344: 441a         	add	r2, r3
 8006346: ea4f 0346    	lsl.w	r3, r6, #0x1
 800634a: ea83 73e6    	eor.w	r3, r3, r6, asr #31
 800634e: 441a         	add	r2, r3
 8006350: ea4f 0349    	lsl.w	r3, r9, #0x1
 8006354: ea83 73e9    	eor.w	r3, r3, r9, asr #31
 8006358: 441a         	add	r2, r3
 800635a: ea4f 0348    	lsl.w	r3, r8, #0x1
 800635e: ea83 73e8    	eor.w	r3, r3, r8, asr #31
 8006362: 441a         	add	r2, r3
 8006364: ea4f 034c    	lsl.w	r3, r12, #0x1
 8006368: ea83 73ec    	eor.w	r3, r3, r12, asr #31
 800636c: eb03 0802    	add.w	r8, r3, r2
 8006370: f47f af17    	bne.w	0x80061a2 <raylar_crcl_encoder_service::benchmark::fold_sum::h16b920496cf2076e+0x18> @ imm = #-0x1d2
 8006374: f8cd 8008    	str.w	r8, [sp, #0x8]
 8006378: a902         	add	r1, sp, #0x8
 800637a: f04f 0900    	mov.w	r9, #0x0
 800637e: f240 38ff    	movw	r8, #0x3ff
 8006382: 9902         	ldr	r1, [sp, #0x8]
 8006384: 9101         	str	r1, [sp, #0x4]
 8006386: f500 3190    	add.w	r1, r0, #0x12000
 800638a: f501 5181    	add.w	r1, r1, #0x1020
 800638e: f851 ef84    	ldr	lr, [r1, #132]!
 8006392: f1b8 0821    	subs.w	r8, r8, #0x21
 8006396: e951 5420    	ldrd	r5, r4, [r1, #-128]
 800639a: ea4f 0644    	lsl.w	r6, r4, #0x1
 800639e: e951 321e    	ldrd	r3, r2, [r1, #-120]
 80063a2: ea86 76e4    	eor.w	r6, r6, r4, asr #31
 80063a6: f851 ac08    	ldr	r10, [r1, #-8]
 80063aa: ea4f 0445    	lsl.w	r4, r5, #0x1
 80063ae: ea84 74e5    	eor.w	r4, r4, r5, asr #31
 80063b2: eb04 0509    	add.w	r5, r4, r9
 80063b6: ea4f 0443    	lsl.w	r4, r3, #0x1
 80063ba: ea84 73e3    	eor.w	r3, r4, r3, asr #31
 80063be: f851 4c70    	ldr	r4, [r1, #-112]
 80063c2: 442e         	add	r6, r5
 80063c4: ea4f 0542    	lsl.w	r5, r2, #0x1
 80063c8: 4433         	add	r3, r6
 80063ca: ea85 72e2    	eor.w	r2, r5, r2, asr #31
 80063ce: f851 5c6c    	ldr	r5, [r1, #-108]
 80063d2: 441a         	add	r2, r3
 80063d4: ea4f 0344    	lsl.w	r3, r4, #0x1
 80063d8: f851 6c0c    	ldr	r6, [r1, #-12]
 80063dc: ea83 73e4    	eor.w	r3, r3, r4, asr #31
 80063e0: f851 4c68    	ldr	r4, [r1, #-104]
 80063e4: 441a         	add	r2, r3
 80063e6: ea4f 0345    	lsl.w	r3, r5, #0x1
 80063ea: ea83 73e5    	eor.w	r3, r3, r5, asr #31
 80063ee: f851 5c64    	ldr	r5, [r1, #-100]
 80063f2: 441a         	add	r2, r3
 80063f4: ea4f 0344    	lsl.w	r3, r4, #0x1
 80063f8: ea83 73e4    	eor.w	r3, r3, r4, asr #31
 80063fc: f851 4c60    	ldr	r4, [r1, #-96]
 8006400: 441a         	add	r2, r3
 8006402: ea4f 0345    	lsl.w	r3, r5, #0x1
 8006406: ea83 73e5    	eor.w	r3, r3, r5, asr #31
 800640a: f851 5c5c    	ldr	r5, [r1, #-92]
 800640e: 441a         	add	r2, r3
 8006410: ea4f 0344    	lsl.w	r3, r4, #0x1
 8006414: ea83 73e4    	eor.w	r3, r3, r4, asr #31
 8006418: f851 4c58    	ldr	r4, [r1, #-88]
 800641c: 441a         	add	r2, r3
 800641e: ea4f 0345    	lsl.w	r3, r5, #0x1
 8006422: ea83 73e5    	eor.w	r3, r3, r5, asr #31
 8006426: f851 5c54    	ldr	r5, [r1, #-84]
 800642a: 441a         	add	r2, r3
 800642c: ea4f 0344    	lsl.w	r3, r4, #0x1
 8006430: ea83 73e4    	eor.w	r3, r3, r4, asr #31
 8006434: f851 4c50    	ldr	r4, [r1, #-80]
 8006438: 441a         	add	r2, r3
 800643a: ea4f 0345    	lsl.w	r3, r5, #0x1
 800643e: ea83 73e5    	eor.w	r3, r3, r5, asr #31
 8006442: f851 5c4c    	ldr	r5, [r1, #-76]
 8006446: 441a         	add	r2, r3
 8006448: ea4f 0344    	lsl.w	r3, r4, #0x1
 800644c: ea83 73e4    	eor.w	r3, r3, r4, asr #31
 8006450: f851 4c48    	ldr	r4, [r1, #-72]
 8006454: 441a         	add	r2, r3
 8006456: ea4f 0345    	lsl.w	r3, r5, #0x1
 800645a: ea83 73e5    	eor.w	r3, r3, r5, asr #31
 800645e: f851 5c44    	ldr	r5, [r1, #-68]
 8006462: 441a         	add	r2, r3
 8006464: ea4f 0344    	lsl.w	r3, r4, #0x1
 8006468: ea83 73e4    	eor.w	r3, r3, r4, asr #31
 800646c: f851 4c40    	ldr	r4, [r1, #-64]
 8006470: 441a         	add	r2, r3
 8006472: ea4f 0345    	lsl.w	r3, r5, #0x1
 8006476: ea83 73e5    	eor.w	r3, r3, r5, asr #31
 800647a: f851 5c3c    	ldr	r5, [r1, #-60]
 800647e: 441a         	add	r2, r3
 8006480: ea4f 0344    	lsl.w	r3, r4, #0x1
 8006484: ea83 73e4    	eor.w	r3, r3, r4, asr #31
 8006488: f851 4c38    	ldr	r4, [r1, #-56]
 800648c: 441a         	add	r2, r3
 800648e: ea4f 0345    	lsl.w	r3, r5, #0x1
 8006492: ea83 73e5    	eor.w	r3, r3, r5, asr #31
 8006496: f851 5c34    	ldr	r5, [r1, #-52]
 800649a: 441a         	add	r2, r3
 800649c: ea4f 0344    	lsl.w	r3, r4, #0x1
 80064a0: ea83 73e4    	eor.w	r3, r3, r4, asr #31
 80064a4: f851 4c30    	ldr	r4, [r1, #-48]
 80064a8: 441a         	add	r2, r3
 80064aa: ea4f 0345    	lsl.w	r3, r5, #0x1
 80064ae: ea83 73e5    	eor.w	r3, r3, r5, asr #31
 80064b2: f851 5c2c    	ldr	r5, [r1, #-44]
 80064b6: 441a         	add	r2, r3
 80064b8: ea4f 0344    	lsl.w	r3, r4, #0x1
 80064bc: ea83 73e4    	eor.w	r3, r3, r4, asr #31
 80064c0: f851 4c28    	ldr	r4, [r1, #-40]
 80064c4: 441a         	add	r2, r3
 80064c6: ea4f 0345    	lsl.w	r3, r5, #0x1
 80064ca: ea83 73e5    	eor.w	r3, r3, r5, asr #31
 80064ce: f851 5c24    	ldr	r5, [r1, #-36]
 80064d2: 441a         	add	r2, r3
 80064d4: ea4f 0344    	lsl.w	r3, r4, #0x1
 80064d8: ea83 73e4    	eor.w	r3, r3, r4, asr #31
 80064dc: f851 4c20    	ldr	r4, [r1, #-32]
 80064e0: 441a         	add	r2, r3
 80064e2: ea4f 0345    	lsl.w	r3, r5, #0x1
 80064e6: ea83 73e5    	eor.w	r3, r3, r5, asr #31
 80064ea: f851 5c1c    	ldr	r5, [r1, #-28]
 80064ee: 441a         	add	r2, r3
 80064f0: ea4f 0344    	lsl.w	r3, r4, #0x1
 80064f4: ea83 73e4    	eor.w	r3, r3, r4, asr #31
 80064f8: f851 4c18    	ldr	r4, [r1, #-24]
 80064fc: 441a         	add	r2, r3
 80064fe: ea4f 0345    	lsl.w	r3, r5, #0x1
 8006502: ea83 73e5    	eor.w	r3, r3, r5, asr #31
 8006506: f851 5c14    	ldr	r5, [r1, #-20]
 800650a: 441a         	add	r2, r3
 800650c: ea4f 0344    	lsl.w	r3, r4, #0x1
 8006510: ea83 73e4    	eor.w	r3, r3, r4, asr #31
 8006514: f851 4c10    	ldr	r4, [r1, #-16]
 8006518: 441a         	add	r2, r3
 800651a: ea4f 0345    	lsl.w	r3, r5, #0x1
 800651e: ea83 73e5    	eor.w	r3, r3, r5, asr #31
 8006522: f851 9c04    	ldr	r9, [r1, #-4]
 8006526: 441a         	add	r2, r3
 8006528: ea4f 0344    	lsl.w	r3, r4, #0x1
 800652c: ea83 73e4    	eor.w	r3, r3, r4, asr #31
 8006530: 441a         	add	r2, r3
 8006532: ea4f 0346    	lsl.w	r3, r6, #0x1
 8006536: ea83 73e6    	eor.w	r3, r3, r6, asr #31
 800653a: 441a         	add	r2, r3
 800653c: ea4f 034a    	lsl.w	r3, r10, #0x1
 8006540: ea83 73ea    	eor.w	r3, r3, r10, asr #31
 8006544: 441a         	add	r2, r3
 8006546: ea4f 0349    	lsl.w	r3, r9, #0x1
 800654a: ea83 73e9    	eor.w	r3, r3, r9, asr #31
 800654e: 441a         	add	r2, r3
 8006550: ea4f 034e    	lsl.w	r3, lr, #0x1
 8006554: ea83 73ee    	eor.w	r3, r3, lr, asr #31
 8006558: eb03 0902    	add.w	r9, r3, r2
 800655c: f47f af17    	bne.w	0x800638e <raylar_crcl_encoder_service::benchmark::fold_sum::h16b920496cf2076e+0x204> @ imm = #-0x1d2
 8006560: f8cd 9008    	str.w	r9, [sp, #0x8]
 8006564: a902         	add	r1, sp, #0x8
 8006566: f04f 0a00    	mov.w	r10, #0x0
 800656a: f240 39ff    	movw	r9, #0x3ff
 800656e: 9902         	ldr	r1, [sp, #0x8]
 8006570: 9100         	str	r1, [sp]
 8006572: f500 31a0    	add.w	r1, r0, #0x14000
 8006576: 311c         	adds	r1, #0x1c
 8006578: f851 8f84    	ldr	r8, [r1, #132]!
 800657c: f1b9 0921    	subs.w	r9, r9, #0x21
 8006580: e951 5620    	ldrd	r5, r6, [r1, #-128]
 8006584: ea4f 0446    	lsl.w	r4, r6, #0x1
 8006588: e951 231e    	ldrd	r2, r3, [r1, #-120]
 800658c: ea84 76e6    	eor.w	r6, r4, r6, asr #31
 8006590: f851 bc08    	ldr	r11, [r1, #-8]
 8006594: ea4f 0445    	lsl.w	r4, r5, #0x1
 8006598: ea84 74e5    	eor.w	r4, r4, r5, asr #31
 800659c: eb04 050a    	add.w	r5, r4, r10
 80065a0: ea4f 0442    	lsl.w	r4, r2, #0x1
 80065a4: ea84 72e2    	eor.w	r2, r4, r2, asr #31
 80065a8: f851 4c70    	ldr	r4, [r1, #-112]
 80065ac: 442e         	add	r6, r5
 80065ae: ea4f 0543    	lsl.w	r5, r3, #0x1
 80065b2: 4432         	add	r2, r6
 80065b4: ea85 73e3    	eor.w	r3, r5, r3, asr #31
 80065b8: f851 5c6c    	ldr	r5, [r1, #-108]
 80065bc: 441a         	add	r2, r3
 80065be: ea4f 0344    	lsl.w	r3, r4, #0x1
 80065c2: f851 6c0c    	ldr	r6, [r1, #-12]
 80065c6: ea83 73e4    	eor.w	r3, r3, r4, asr #31
 80065ca: f851 4c68    	ldr	r4, [r1, #-104]
 80065ce: 441a         	add	r2, r3
 80065d0: ea4f 0345    	lsl.w	r3, r5, #0x1
 80065d4: ea83 73e5    	eor.w	r3, r3, r5, asr #31
 80065d8: f851 5c64    	ldr	r5, [r1, #-100]
 80065dc: 441a         	add	r2, r3
 80065de: ea4f 0344    	lsl.w	r3, r4, #0x1
 80065e2: ea83 73e4    	eor.w	r3, r3, r4, asr #31
 80065e6: f851 4c60    	ldr	r4, [r1, #-96]
 80065ea: 441a         	add	r2, r3
 80065ec: ea4f 0345    	lsl.w	r3, r5, #0x1
 80065f0: ea83 73e5    	eor.w	r3, r3, r5, asr #31
 80065f4: f851 5c5c    	ldr	r5, [r1, #-92]
 80065f8: 441a         	add	r2, r3
 80065fa: ea4f 0344    	lsl.w	r3, r4, #0x1
 80065fe: ea83 73e4    	eor.w	r3, r3, r4, asr #31
 8006602: f851 4c58    	ldr	r4, [r1, #-88]
 8006606: 441a         	add	r2, r3
 8006608: ea4f 0345    	lsl.w	r3, r5, #0x1
 800660c: ea83 73e5    	eor.w	r3, r3, r5, asr #31
 8006610: f851 5c54    	ldr	r5, [r1, #-84]
 8006614: 441a         	add	r2, r3
 8006616: ea4f 0344    	lsl.w	r3, r4, #0x1
 800661a: ea83 73e4    	eor.w	r3, r3, r4, asr #31
 800661e: f851 4c50    	ldr	r4, [r1, #-80]
 8006622: 441a         	add	r2, r3
 8006624: ea4f 0345    	lsl.w	r3, r5, #0x1
 8006628: ea83 73e5    	eor.w	r3, r3, r5, asr #31
 800662c: f851 5c4c    	ldr	r5, [r1, #-76]
 8006630: 441a         	add	r2, r3
 8006632: ea4f 0344    	lsl.w	r3, r4, #0x1
 8006636: ea83 73e4    	eor.w	r3, r3, r4, asr #31
 800663a: f851 4c48    	ldr	r4, [r1, #-72]
 800663e: 441a         	add	r2, r3
 8006640: ea4f 0345    	lsl.w	r3, r5, #0x1
 8006644: ea83 73e5    	eor.w	r3, r3, r5, asr #31
 8006648: f851 5c44    	ldr	r5, [r1, #-68]
 800664c: 441a         	add	r2, r3
 800664e: ea4f 0344    	lsl.w	r3, r4, #0x1
 8006652: ea83 73e4    	eor.w	r3, r3, r4, asr #31
 8006656: f851 4c40    	ldr	r4, [r1, #-64]
 800665a: 441a         	add	r2, r3
 800665c: ea4f 0345    	lsl.w	r3, r5, #0x1
 8006660: ea83 73e5    	eor.w	r3, r3, r5, asr #31
 8006664: f851 5c3c    	ldr	r5, [r1, #-60]
 8006668: 441a         	add	r2, r3
 800666a: ea4f 0344    	lsl.w	r3, r4, #0x1
 800666e: ea83 73e4    	eor.w	r3, r3, r4, asr #31
 8006672: f851 4c38    	ldr	r4, [r1, #-56]
 8006676: 441a         	add	r2, r3
 8006678: ea4f 0345    	lsl.w	r3, r5, #0x1
 800667c: ea83 73e5    	eor.w	r3, r3, r5, asr #31
 8006680: f851 5c34    	ldr	r5, [r1, #-52]
 8006684: 441a         	add	r2, r3
 8006686: ea4f 0344    	lsl.w	r3, r4, #0x1
 800668a: ea83 73e4    	eor.w	r3, r3, r4, asr #31
 800668e: f851 4c30    	ldr	r4, [r1, #-48]
 8006692: 441a         	add	r2, r3
 8006694: ea4f 0345    	lsl.w	r3, r5, #0x1
 8006698: ea83 73e5    	eor.w	r3, r3, r5, asr #31
 800669c: f851 5c2c    	ldr	r5, [r1, #-44]
 80066a0: 441a         	add	r2, r3
 80066a2: ea4f 0344    	lsl.w	r3, r4, #0x1
 80066a6: ea83 73e4    	eor.w	r3, r3, r4, asr #31
 80066aa: f851 4c28    	ldr	r4, [r1, #-40]
 80066ae: 441a         	add	r2, r3
 80066b0: ea4f 0345    	lsl.w	r3, r5, #0x1
 80066b4: ea83 73e5    	eor.w	r3, r3, r5, asr #31
 80066b8: f851 5c24    	ldr	r5, [r1, #-36]
 80066bc: 441a         	add	r2, r3
 80066be: ea4f 0344    	lsl.w	r3, r4, #0x1
 80066c2: ea83 73e4    	eor.w	r3, r3, r4, asr #31
 80066c6: f851 4c20    	ldr	r4, [r1, #-32]
 80066ca: 441a         	add	r2, r3
 80066cc: ea4f 0345    	lsl.w	r3, r5, #0x1
 80066d0: ea83 73e5    	eor.w	r3, r3, r5, asr #31
 80066d4: f851 5c1c    	ldr	r5, [r1, #-28]
 80066d8: 441a         	add	r2, r3
 80066da: ea4f 0344    	lsl.w	r3, r4, #0x1
 80066de: ea83 73e4    	eor.w	r3, r3, r4, asr #31
 80066e2: f851 4c18    	ldr	r4, [r1, #-24]
 80066e6: 441a         	add	r2, r3
 80066e8: ea4f 0345    	lsl.w	r3, r5, #0x1
 80066ec: ea83 73e5    	eor.w	r3, r3, r5, asr #31
 80066f0: f851 5c14    	ldr	r5, [r1, #-20]
 80066f4: 441a         	add	r2, r3
 80066f6: ea4f 0344    	lsl.w	r3, r4, #0x1
 80066fa: ea83 73e4    	eor.w	r3, r3, r4, asr #31
 80066fe: f851 4c10    	ldr	r4, [r1, #-16]
 8006702: 441a         	add	r2, r3
 8006704: ea4f 0345    	lsl.w	r3, r5, #0x1
 8006708: ea83 73e5    	eor.w	r3, r3, r5, asr #31
 800670c: f851 ac04    	ldr	r10, [r1, #-4]
 8006710: 441a         	add	r2, r3
 8006712: ea4f 0344    	lsl.w	r3, r4, #0x1
 8006716: ea83 73e4    	eor.w	r3, r3, r4, asr #31
 800671a: 441a         	add	r2, r3
 800671c: ea4f 0346    	lsl.w	r3, r6, #0x1
 8006720: ea83 73e6    	eor.w	r3, r3, r6, asr #31
 8006724: 441a         	add	r2, r3
 8006726: ea4f 034b    	lsl.w	r3, r11, #0x1
 800672a: ea83 73eb    	eor.w	r3, r3, r11, asr #31
 800672e: 441a         	add	r2, r3
 8006730: ea4f 034a    	lsl.w	r3, r10, #0x1
 8006734: ea83 73ea    	eor.w	r3, r3, r10, asr #31
 8006738: 441a         	add	r2, r3
 800673a: ea4f 0348    	lsl.w	r3, r8, #0x1
 800673e: ea83 73e8    	eor.w	r3, r3, r8, asr #31
 8006742: eb03 0a02    	add.w	r10, r3, r2
 8006746: f47f af17    	bne.w	0x8006578 <raylar_crcl_encoder_service::benchmark::fold_sum::h16b920496cf2076e+0x3ee> @ imm = #-0x1d2
 800674a: f8cd a008    	str.w	r10, [sp, #0x8]
 800674e: a902         	add	r1, sp, #0x8
 8006750: f500 31a8    	add.w	r1, r0, #0x15000
 8006754: f04f 0b00    	mov.w	r11, #0x0
 8006758: f8dd 8008    	ldr.w	r8, [sp, #0x8]
 800675c: 3118         	adds	r1, #0x18
 800675e: f240 3aff    	movw	r10, #0x3ff
 8006762: f851 9f84    	ldr	r9, [r1, #132]!
 8006766: f1ba 0a21    	subs.w	r10, r10, #0x21
 800676a: e951 5620    	ldrd	r5, r6, [r1, #-128]
 800676e: e951 421e    	ldrd	r4, r2, [r1, #-120]
 8006772: ea4f 0346    	lsl.w	r3, r6, #0x1
 8006776: ea83 73e6    	eor.w	r3, r3, r6, asr #31
 800677a: ea4f 0c42    	lsl.w	r12, r2, #0x1
 800677e: ea4f 0645    	lsl.w	r6, r5, #0x1
 8006782: ea86 75e5    	eor.w	r5, r6, r5, asr #31
 8006786: eb05 060b    	add.w	r6, r5, r11
 800678a: ea4f 0544    	lsl.w	r5, r4, #0x1
 800678e: ea85 74e4    	eor.w	r4, r5, r4, asr #31
 8006792: f851 5c70    	ldr	r5, [r1, #-112]
 8006796: 4433         	add	r3, r6
 8006798: ea8c 7ce2    	eor.w	r12, r12, r2, asr #31
 800679c: f851 2c6c    	ldr	r2, [r1, #-108]
 80067a0: 441c         	add	r4, r3
 80067a2: ea4f 0645    	lsl.w	r6, r5, #0x1
 80067a6: 4464         	add	r4, r12
 80067a8: ea86 76e5    	eor.w	r6, r6, r5, asr #31
 80067ac: f851 5c68    	ldr	r5, [r1, #-104]
 80067b0: 4426         	add	r6, r4
 80067b2: ea4f 0442    	lsl.w	r4, r2, #0x1
 80067b6: ea84 72e2    	eor.w	r2, r4, r2, asr #31
 80067ba: f851 4c64    	ldr	r4, [r1, #-100]
 80067be: 4432         	add	r2, r6
 80067c0: ea4f 0645    	lsl.w	r6, r5, #0x1
 80067c4: ea86 76e5    	eor.w	r6, r6, r5, asr #31
 80067c8: f851 5c60    	ldr	r5, [r1, #-96]
 80067cc: 4432         	add	r2, r6
 80067ce: ea4f 0644    	lsl.w	r6, r4, #0x1
 80067d2: ea86 76e4    	eor.w	r6, r6, r4, asr #31
 80067d6: f851 4c5c    	ldr	r4, [r1, #-92]
 80067da: 4432         	add	r2, r6
 80067dc: ea4f 0645    	lsl.w	r6, r5, #0x1
 80067e0: ea86 76e5    	eor.w	r6, r6, r5, asr #31
 80067e4: f851 5c58    	ldr	r5, [r1, #-88]
 80067e8: 4432         	add	r2, r6
 80067ea: ea4f 0644    	lsl.w	r6, r4, #0x1
 80067ee: ea86 76e4    	eor.w	r6, r6, r4, asr #31
 80067f2: f851 4c54    	ldr	r4, [r1, #-84]
 80067f6: 4432         	add	r2, r6
 80067f8: ea4f 0645    	lsl.w	r6, r5, #0x1
 80067fc: ea86 76e5    	eor.w	r6, r6, r5, asr #31
 8006800: f851 5c50    	ldr	r5, [r1, #-80]
 8006804: 4432         	add	r2, r6
 8006806: ea4f 0644    	lsl.w	r6, r4, #0x1
 800680a: ea86 76e4    	eor.w	r6, r6, r4, asr #31
 800680e: f851 4c4c    	ldr	r4, [r1, #-76]
 8006812: 4432         	add	r2, r6
 8006814: ea4f 0645    	lsl.w	r6, r5, #0x1
 8006818: ea86 76e5    	eor.w	r6, r6, r5, asr #31
 800681c: f851 5c48    	ldr	r5, [r1, #-72]
 8006820: 4432         	add	r2, r6
 8006822: ea4f 0644    	lsl.w	r6, r4, #0x1
 8006826: ea86 76e4    	eor.w	r6, r6, r4, asr #31
 800682a: f851 4c44    	ldr	r4, [r1, #-68]
 800682e: 4432         	add	r2, r6
 8006830: ea4f 0645    	lsl.w	r6, r5, #0x1
 8006834: ea86 76e5    	eor.w	r6, r6, r5, asr #31
 8006838: f851 5c40    	ldr	r5, [r1, #-64]
 800683c: 4432         	add	r2, r6
 800683e: ea4f 0644    	lsl.w	r6, r4, #0x1
 8006842: ea86 76e4    	eor.w	r6, r6, r4, asr #31
 8006846: f851 4c3c    	ldr	r4, [r1, #-60]
 800684a: 4432         	add	r2, r6
 800684c: ea4f 0645    	lsl.w	r6, r5, #0x1
 8006850: ea86 76e5    	eor.w	r6, r6, r5, asr #31
 8006854: f851 5c38    	ldr	r5, [r1, #-56]
 8006858: 4432         	add	r2, r6
 800685a: ea4f 0644    	lsl.w	r6, r4, #0x1
 800685e: ea86 76e4    	eor.w	r6, r6, r4, asr #31
 8006862: f851 4c34    	ldr	r4, [r1, #-52]
 8006866: 4432         	add	r2, r6
 8006868: ea4f 0645    	lsl.w	r6, r5, #0x1
 800686c: ea86 76e5    	eor.w	r6, r6, r5, asr #31
 8006870: f851 5c30    	ldr	r5, [r1, #-48]
 8006874: 4432         	add	r2, r6
 8006876: ea4f 0644    	lsl.w	r6, r4, #0x1
 800687a: ea86 76e4    	eor.w	r6, r6, r4, asr #31
 800687e: f851 4c2c    	ldr	r4, [r1, #-44]
 8006882: 4432         	add	r2, r6
 8006884: ea4f 0645    	lsl.w	r6, r5, #0x1
 8006888: ea86 76e5    	eor.w	r6, r6, r5, asr #31
 800688c: f851 5c28    	ldr	r5, [r1, #-40]
 8006890: 4432         	add	r2, r6
 8006892: ea4f 0644    	lsl.w	r6, r4, #0x1
 8006896: ea86 76e4    	eor.w	r6, r6, r4, asr #31
 800689a: f851 4c24    	ldr	r4, [r1, #-36]
 800689e: 4432         	add	r2, r6
 80068a0: ea4f 0645    	lsl.w	r6, r5, #0x1
 80068a4: ea86 76e5    	eor.w	r6, r6, r5, asr #31
 80068a8: f851 5c20    	ldr	r5, [r1, #-32]
 80068ac: 4432         	add	r2, r6
 80068ae: ea4f 0644    	lsl.w	r6, r4, #0x1
 80068b2: ea86 76e4    	eor.w	r6, r6, r4, asr #31
 80068b6: f851 4c1c    	ldr	r4, [r1, #-28]
 80068ba: 4432         	add	r2, r6
 80068bc: ea4f 0645    	lsl.w	r6, r5, #0x1
 80068c0: ea86 76e5    	eor.w	r6, r6, r5, asr #31
 80068c4: f851 5c18    	ldr	r5, [r1, #-24]
 80068c8: 4432         	add	r2, r6
 80068ca: ea4f 0644    	lsl.w	r6, r4, #0x1
 80068ce: ea86 76e4    	eor.w	r6, r6, r4, asr #31
 80068d2: f851 4c14    	ldr	r4, [r1, #-20]
 80068d6: 4432         	add	r2, r6
 80068d8: ea4f 0645    	lsl.w	r6, r5, #0x1
 80068dc: ea86 76e5    	eor.w	r6, r6, r5, asr #31
 80068e0: f851 5c10    	ldr	r5, [r1, #-16]
 80068e4: 4432         	add	r2, r6
 80068e6: ea4f 0644    	lsl.w	r6, r4, #0x1
 80068ea: ea86 76e4    	eor.w	r6, r6, r4, asr #31
 80068ee: f851 3c0c    	ldr	r3, [r1, #-12]
 80068f2: 4432         	add	r2, r6
 80068f4: ea4f 0645    	lsl.w	r6, r5, #0x1
 80068f8: f851 ec08    	ldr	lr, [r1, #-8]
 80068fc: ea86 76e5    	eor.w	r6, r6, r5, asr #31
 8006900: 4432         	add	r2, r6
 8006902: ea4f 0643    	lsl.w	r6, r3, #0x1
 8006906: ea86 73e3    	eor.w	r3, r6, r3, asr #31
 800690a: f851 bc04    	ldr	r11, [r1, #-4]
 800690e: 441a         	add	r2, r3
 8006910: ea4f 034e    	lsl.w	r3, lr, #0x1
 8006914: ea83 73ee    	eor.w	r3, r3, lr, asr #31
 8006918: 441a         	add	r2, r3
 800691a: ea4f 034b    	lsl.w	r3, r11, #0x1
 800691e: ea83 73eb    	eor.w	r3, r3, r11, asr #31
 8006922: 441a         	add	r2, r3
 8006924: ea4f 0349    	lsl.w	r3, r9, #0x1
 8006928: ea83 73e9    	eor.w	r3, r3, r9, asr #31
 800692c: eb03 0b02    	add.w	r11, r3, r2
 8006930: f47f af17    	bne.w	0x8006762 <raylar_crcl_encoder_service::benchmark::fold_sum::h16b920496cf2076e+0x5d8> @ imm = #-0x1d2
 8006934: f8cd b008    	str.w	r11, [sp, #0x8]
 8006938: a902         	add	r1, sp, #0x8
 800693a: f500 30b0    	add.w	r0, r0, #0x16000
 800693e: 2300         	movs	r3, #0x0
 8006940: f8dd c008    	ldr.w	r12, [sp, #0x8]
 8006944: 3014         	adds	r0, #0x14
 8006946: f240 39ff    	movw	r9, #0x3ff
 800694a: f850 ef84    	ldr	lr, [r0, #132]!
 800694e: f1b9 0921    	subs.w	r9, r9, #0x21
 8006952: e950 6520    	ldrd	r6, r5, [r0, #-128]
 8006956: ea4f 0245    	lsl.w	r2, r5, #0x1
 800695a: e950 141e    	ldrd	r1, r4, [r0, #-120]
 800695e: ea82 72e5    	eor.w	r2, r2, r5, asr #31
 8006962: f850 ac04    	ldr	r10, [r0, #-4]
 8006966: ea4f 0546    	lsl.w	r5, r6, #0x1
 800696a: ea85 75e6    	eor.w	r5, r5, r6, asr #31
 800696e: f850 6c08    	ldr	r6, [r0, #-8]
 8006972: 442b         	add	r3, r5
 8006974: f850 5c70    	ldr	r5, [r0, #-112]
 8006978: 441a         	add	r2, r3
 800697a: ea4f 0341    	lsl.w	r3, r1, #0x1
 800697e: ea83 71e1    	eor.w	r1, r3, r1, asr #31
 8006982: f850 3c0c    	ldr	r3, [r0, #-12]
 8006986: 4411         	add	r1, r2
 8006988: ea4f 0244    	lsl.w	r2, r4, #0x1
 800698c: ea82 72e4    	eor.w	r2, r2, r4, asr #31
 8006990: f850 4c6c    	ldr	r4, [r0, #-108]
 8006994: 4411         	add	r1, r2
 8006996: ea4f 0245    	lsl.w	r2, r5, #0x1
 800699a: ea82 72e5    	eor.w	r2, r2, r5, asr #31
 800699e: f850 5c68    	ldr	r5, [r0, #-104]
 80069a2: 4411         	add	r1, r2
 80069a4: ea4f 0244    	lsl.w	r2, r4, #0x1
 80069a8: ea82 72e4    	eor.w	r2, r2, r4, asr #31
 80069ac: f850 4c64    	ldr	r4, [r0, #-100]
 80069b0: 4411         	add	r1, r2
 80069b2: ea4f 0245    	lsl.w	r2, r5, #0x1
 80069b6: ea82 72e5    	eor.w	r2, r2, r5, asr #31
 80069ba: f850 5c60    	ldr	r5, [r0, #-96]
 80069be: 4411         	add	r1, r2
 80069c0: ea4f 0244    	lsl.w	r2, r4, #0x1
 80069c4: ea82 72e4    	eor.w	r2, r2, r4, asr #31
 80069c8: f850 4c5c    	ldr	r4, [r0, #-92]
 80069cc: 4411         	add	r1, r2
 80069ce: ea4f 0245    	lsl.w	r2, r5, #0x1
 80069d2: ea82 72e5    	eor.w	r2, r2, r5, asr #31
 80069d6: f850 5c58    	ldr	r5, [r0, #-88]
 80069da: 4411         	add	r1, r2
 80069dc: ea4f 0244    	lsl.w	r2, r4, #0x1
 80069e0: ea82 72e4    	eor.w	r2, r2, r4, asr #31
 80069e4: f850 4c54    	ldr	r4, [r0, #-84]
 80069e8: 4411         	add	r1, r2
 80069ea: ea4f 0245    	lsl.w	r2, r5, #0x1
 80069ee: ea82 72e5    	eor.w	r2, r2, r5, asr #31
 80069f2: f850 5c50    	ldr	r5, [r0, #-80]
 80069f6: 4411         	add	r1, r2
 80069f8: ea4f 0244    	lsl.w	r2, r4, #0x1
 80069fc: ea82 72e4    	eor.w	r2, r2, r4, asr #31
 8006a00: f850 4c4c    	ldr	r4, [r0, #-76]
 8006a04: 4411         	add	r1, r2
 8006a06: ea4f 0245    	lsl.w	r2, r5, #0x1
 8006a0a: ea82 72e5    	eor.w	r2, r2, r5, asr #31
 8006a0e: f850 5c48    	ldr	r5, [r0, #-72]
 8006a12: 4411         	add	r1, r2
 8006a14: ea4f 0244    	lsl.w	r2, r4, #0x1
 8006a18: ea82 72e4    	eor.w	r2, r2, r4, asr #31
 8006a1c: f850 4c44    	ldr	r4, [r0, #-68]
 8006a20: 4411         	add	r1, r2
 8006a22: ea4f 0245    	lsl.w	r2, r5, #0x1
 8006a26: ea82 72e5    	eor.w	r2, r2, r5, asr #31
 8006a2a: f850 5c40    	ldr	r5, [r0, #-64]
 8006a2e: 4411         	add	r1, r2
 8006a30: ea4f 0244    	lsl.w	r2, r4, #0x1
 8006a34: ea82 72e4    	eor.w	r2, r2, r4, asr #31
 8006a38: f850 4c3c    	ldr	r4, [r0, #-60]
 8006a3c: 4411         	add	r1, r2
 8006a3e: ea4f 0245    	lsl.w	r2, r5, #0x1
 8006a42: ea82 72e5    	eor.w	r2, r2, r5, asr #31
 8006a46: f850 5c38    	ldr	r5, [r0, #-56]
 8006a4a: 4411         	add	r1, r2
 8006a4c: ea4f 0244    	lsl.w	r2, r4, #0x1
 8006a50: ea82 72e4    	eor.w	r2, r2, r4, asr #31
 8006a54: f850 4c34    	ldr	r4, [r0, #-52]
 8006a58: 4411         	add	r1, r2
 8006a5a: ea4f 0245    	lsl.w	r2, r5, #0x1
 8006a5e: ea82 72e5    	eor.w	r2, r2, r5, asr #31
 8006a62: f850 5c30    	ldr	r5, [r0, #-48]
 8006a66: 4411         	add	r1, r2
 8006a68: ea4f 0244    	lsl.w	r2, r4, #0x1
 8006a6c: ea82 72e4    	eor.w	r2, r2, r4, asr #31
 8006a70: f850 4c2c    	ldr	r4, [r0, #-44]
 8006a74: 4411         	add	r1, r2
 8006a76: ea4f 0245    	lsl.w	r2, r5, #0x1
 8006a7a: ea82 72e5    	eor.w	r2, r2, r5, asr #31
 8006a7e: f850 5c28    	ldr	r5, [r0, #-40]
 8006a82: 4411         	add	r1, r2
 8006a84: ea4f 0244    	lsl.w	r2, r4, #0x1
 8006a88: ea82 72e4    	eor.w	r2, r2, r4, asr #31
 8006a8c: f850 4c24    	ldr	r4, [r0, #-36]
 8006a90: 4411         	add	r1, r2
 8006a92: ea4f 0245    	lsl.w	r2, r5, #0x1
 8006a96: ea82 72e5    	eor.w	r2, r2, r5, asr #31
 8006a9a: f850 5c20    	ldr	r5, [r0, #-32]
 8006a9e: 4411         	add	r1, r2
 8006aa0: ea4f 0244    	lsl.w	r2, r4, #0x1
 8006aa4: ea82 72e4    	eor.w	r2, r2, r4, asr #31
 8006aa8: f850 4c1c    	ldr	r4, [r0, #-28]
 8006aac: 4411         	add	r1, r2
 8006aae: ea4f 0245    	lsl.w	r2, r5, #0x1
 8006ab2: ea82 72e5    	eor.w	r2, r2, r5, asr #31
 8006ab6: f850 5c18    	ldr	r5, [r0, #-24]
 8006aba: 4411         	add	r1, r2
 8006abc: ea4f 0244    	lsl.w	r2, r4, #0x1
 8006ac0: ea82 72e4    	eor.w	r2, r2, r4, asr #31
 8006ac4: f850 4c14    	ldr	r4, [r0, #-20]
 8006ac8: 4411         	add	r1, r2
 8006aca: ea4f 0245    	lsl.w	r2, r5, #0x1
 8006ace: ea82 72e5    	eor.w	r2, r2, r5, asr #31
 8006ad2: f850 5c10    	ldr	r5, [r0, #-16]
 8006ad6: 4411         	add	r1, r2
 8006ad8: ea4f 0244    	lsl.w	r2, r4, #0x1
 8006adc: ea82 72e4    	eor.w	r2, r2, r4, asr #31
 8006ae0: 4411         	add	r1, r2
 8006ae2: ea4f 0245    	lsl.w	r2, r5, #0x1
 8006ae6: ea82 72e5    	eor.w	r2, r2, r5, asr #31
 8006aea: 4411         	add	r1, r2
 8006aec: ea4f 0243    	lsl.w	r2, r3, #0x1
 8006af0: ea82 72e3    	eor.w	r2, r2, r3, asr #31
 8006af4: 4411         	add	r1, r2
 8006af6: ea4f 0246    	lsl.w	r2, r6, #0x1
 8006afa: ea82 72e6    	eor.w	r2, r2, r6, asr #31
 8006afe: 4411         	add	r1, r2
 8006b00: ea4f 024a    	lsl.w	r2, r10, #0x1
 8006b04: ea82 72ea    	eor.w	r2, r2, r10, asr #31
 8006b08: 4411         	add	r1, r2
 8006b0a: ea4f 024e    	lsl.w	r2, lr, #0x1
 8006b0e: ea82 72ee    	eor.w	r2, r2, lr, asr #31
 8006b12: eb02 0301    	add.w	r3, r2, r1
 8006b16: f47f af18    	bne.w	0x800694a <raylar_crcl_encoder_service::benchmark::fold_sum::h16b920496cf2076e+0x7c0> @ imm = #-0x1d0
 8006b1a: 9302         	str	r3, [sp, #0x8]
 8006b1c: a802         	add	r0, sp, #0x8
 8006b1e: e9dd 1000    	ldrd	r1, r0, [sp]
 8006b22: 4048         	eors	r0, r1
 8006b24: 9902         	ldr	r1, [sp, #0x8]
 8006b26: ea80 0008    	eor.w	r0, r0, r8
 8006b2a: ea80 000c    	eor.w	r0, r0, r12
 8006b2e: 4048         	eors	r0, r1
 8006b30: b003         	add	sp, #0xc
 8006b32: e8bd 0f00    	pop.w	{r8, r9, r10, r11}
 8006b36: bdf0         	pop	{r4, r5, r6, r7, pc}

08006b38 <raylar_crcl_encoder_service::benchmark::rounding::h32a8b1c6cb68e077>:
 8006b38: b5f0         	push	{r4, r5, r6, r7, lr}
 8006b3a: af03         	add	r7, sp, #0xc
 8006b3c: e92d 0b00    	push.w	{r8, r9, r11}
 8006b40: b082         	sub	sp, #0x8
 8006b42: 6801         	ldr	r1, [r0]
 8006b44: 9101         	str	r1, [sp, #0x4]
 8006b46: a901         	add	r1, sp, #0x4
 8006b48: f645 716c    	movw	r1, #0x5f6c
 8006b4c: 9b01         	ldr	r3, [sp, #0x4]
 8006b4e: 4401         	add	r1, r0
 8006b50: 1e5a         	subs	r2, r3, #0x1
 8006b52: f003 031f    	and	r3, r3, #0x1f
 8006b56: f002 0c1f    	and	r12, r2, #0x1f
 8006b5a: 2201         	movs	r2, #0x1
 8006b5c: fa02 f20c    	lsl.w	r2, r2, r12
 8006b60: f44f 6c80    	mov.w	r12, #0x400
 8006b64: f8d1 40d8    	ldr.w	r4, [r1, #0xd8]
 8006b68: f1bc 0c10    	subs.w	r12, r12, #0x10
 8006b6c: f8d1 50f0    	ldr.w	r5, [r1, #0xf0]
 8006b70: 4414         	add	r4, r2
 8006b72: f8d1 8108    	ldr.w	r8, [r1, #0x108]
 8006b76: f8d1 60c0    	ldr.w	r6, [r1, #0xc0]
 8006b7a: fa44 f403    	asr.w	r4, r4, r3
 8006b7e: f8d1 e198    	ldr.w	lr, [r1, #0x198]
 8006b82: f8c1 40d8    	str.w	r4, [r1, #0xd8]
 8006b86: eb05 0402    	add.w	r4, r5, r2
 8006b8a: 4416         	add	r6, r2
 8006b8c: f8d1 5120    	ldr.w	r5, [r1, #0x120]
 8006b90: fa44 f403    	asr.w	r4, r4, r3
 8006b94: f8d1 9180    	ldr.w	r9, [r1, #0x180]
 8006b98: f8c1 40f0    	str.w	r4, [r1, #0xf0]
 8006b9c: eb08 0402    	add.w	r4, r8, r2
 8006ba0: fa46 f603    	asr.w	r6, r6, r3
 8006ba4: fa44 f403    	asr.w	r4, r4, r3
 8006ba8: f8c1 60c0    	str.w	r6, [r1, #0xc0]
 8006bac: f8c1 4108    	str.w	r4, [r1, #0x108]
 8006bb0: eb05 0402    	add.w	r4, r5, r2
 8006bb4: f8d1 6138    	ldr.w	r6, [r1, #0x138]
 8006bb8: fa44 f403    	asr.w	r4, r4, r3
 8006bbc: f8d1 5150    	ldr.w	r5, [r1, #0x150]
 8006bc0: f8c1 4120    	str.w	r4, [r1, #0x120]
 8006bc4: eb06 0402    	add.w	r4, r6, r2
 8006bc8: f8d1 6168    	ldr.w	r6, [r1, #0x168]
 8006bcc: fa44 f403    	asr.w	r4, r4, r3
 8006bd0: f8c1 4138    	str.w	r4, [r1, #0x138]
 8006bd4: eb05 0402    	add.w	r4, r5, r2
 8006bd8: fa44 f403    	asr.w	r4, r4, r3
 8006bdc: f8c1 4150    	str.w	r4, [r1, #0x150]
 8006be0: eb06 0402    	add.w	r4, r6, r2
 8006be4: eb0e 0602    	add.w	r6, lr, r2
 8006be8: fa44 f403    	asr.w	r4, r4, r3
 8006bec: fa46 f603    	asr.w	r6, r6, r3
 8006bf0: f8c1 4168    	str.w	r4, [r1, #0x168]
 8006bf4: f8c1 6198    	str.w	r6, [r1, #0x198]
 8006bf8: eb09 0402    	add.w	r4, r9, r2
 8006bfc: f8d1 61b0    	ldr.w	r6, [r1, #0x1b0]
 8006c00: fa44 f403    	asr.w	r4, r4, r3
 8006c04: 4416         	add	r6, r2
 8006c06: f8c1 4180    	str.w	r4, [r1, #0x180]
 8006c0a: fa46 f603    	asr.w	r6, r6, r3
 8006c0e: f8c1 61b0    	str.w	r6, [r1, #0x1b0]
 8006c12: f8d1 61c8    	ldr.w	r6, [r1, #0x1c8]
 8006c16: 4416         	add	r6, r2
 8006c18: fa46 f603    	asr.w	r6, r6, r3
 8006c1c: f8c1 61c8    	str.w	r6, [r1, #0x1c8]
 8006c20: f8d1 61e0    	ldr.w	r6, [r1, #0x1e0]
 8006c24: 4416         	add	r6, r2
 8006c26: fa46 f603    	asr.w	r6, r6, r3
 8006c2a: f8c1 61e0    	str.w	r6, [r1, #0x1e0]
 8006c2e: f8d1 61f8    	ldr.w	r6, [r1, #0x1f8]
 8006c32: 4416         	add	r6, r2
 8006c34: fa46 f603    	asr.w	r6, r6, r3
 8006c38: f8c1 61f8    	str.w	r6, [r1, #0x1f8]
 8006c3c: f8d1 6210    	ldr.w	r6, [r1, #0x210]
 8006c40: 4416         	add	r6, r2
 8006c42: fa46 f603    	asr.w	r6, r6, r3
 8006c46: f8c1 6210    	str.w	r6, [r1, #0x210]
 8006c4a: f8d1 6228    	ldr.w	r6, [r1, #0x228]
 8006c4e: 4416         	add	r6, r2
 8006c50: fa46 f603    	asr.w	r6, r6, r3
 8006c54: f8c1 6228    	str.w	r6, [r1, #0x228]
 8006c58: f501 71c0    	add.w	r1, r1, #0x180
 8006c5c: d182         	bne	0x8006b64 <raylar_crcl_encoder_service::benchmark::rounding::h32a8b1c6cb68e077+0x2c> @ imm = #-0xfc
 8006c5e: 6881         	ldr	r1, [r0, #0x8]
 8006c60: 9101         	str	r1, [sp, #0x4]
 8006c62: a901         	add	r1, sp, #0x4
 8006c64: f645 61b0    	movw	r1, #0x5eb0
 8006c68: 9b01         	ldr	r3, [sp, #0x4]
 8006c6a: 4401         	add	r1, r0
 8006c6c: 1e5a         	subs	r2, r3, #0x1
 8006c6e: f003 031f    	and	r3, r3, #0x1f
 8006c72: f002 0c1f    	and	r12, r2, #0x1f
 8006c76: 2201         	movs	r2, #0x1
 8006c78: fa02 f20c    	lsl.w	r2, r2, r12
 8006c7c: f44f 6c80    	mov.w	r12, #0x400
 8006c80: f8d1 4198    	ldr.w	r4, [r1, #0x198]
 8006c84: f1bc 0c10    	subs.w	r12, r12, #0x10
 8006c88: f8d1 51b0    	ldr.w	r5, [r1, #0x1b0]
 8006c8c: 4414         	add	r4, r2
 8006c8e: f8d1 81c8    	ldr.w	r8, [r1, #0x1c8]
 8006c92: f8d1 e180    	ldr.w	lr, [r1, #0x180]
 8006c96: fa44 f403    	asr.w	r4, r4, r3
 8006c9a: f8d1 9240    	ldr.w	r9, [r1, #0x240]
 8006c9e: f8c1 4198    	str.w	r4, [r1, #0x198]
 8006ca2: eb05 0402    	add.w	r4, r5, r2
 8006ca6: eb0e 0602    	add.w	r6, lr, r2
 8006caa: f8d1 51e0    	ldr.w	r5, [r1, #0x1e0]
 8006cae: fa44 f403    	asr.w	r4, r4, r3
 8006cb2: f8d1 e258    	ldr.w	lr, [r1, #0x258]
 8006cb6: f8c1 41b0    	str.w	r4, [r1, #0x1b0]
 8006cba: eb08 0402    	add.w	r4, r8, r2
 8006cbe: fa46 f603    	asr.w	r6, r6, r3
 8006cc2: fa44 f403    	asr.w	r4, r4, r3
 8006cc6: f8c1 6180    	str.w	r6, [r1, #0x180]
 8006cca: f8c1 41c8    	str.w	r4, [r1, #0x1c8]
 8006cce: eb05 0402    	add.w	r4, r5, r2
 8006cd2: f8d1 61f8    	ldr.w	r6, [r1, #0x1f8]
 8006cd6: fa44 f403    	asr.w	r4, r4, r3
 8006cda: f8d1 5210    	ldr.w	r5, [r1, #0x210]
 8006cde: f8c1 41e0    	str.w	r4, [r1, #0x1e0]
 8006ce2: eb06 0402    	add.w	r4, r6, r2
 8006ce6: f8d1 6228    	ldr.w	r6, [r1, #0x228]
 8006cea: fa44 f403    	asr.w	r4, r4, r3
 8006cee: f8c1 41f8    	str.w	r4, [r1, #0x1f8]
 8006cf2: eb05 0402    	add.w	r4, r5, r2
 8006cf6: fa44 f403    	asr.w	r4, r4, r3
 8006cfa: f8c1 4210    	str.w	r4, [r1, #0x210]
 8006cfe: eb06 0402    	add.w	r4, r6, r2
 8006d02: eb0e 0602    	add.w	r6, lr, r2
 8006d06: fa44 f403    	asr.w	r4, r4, r3
 8006d0a: fa46 f603    	asr.w	r6, r6, r3
 8006d0e: f8c1 4228    	str.w	r4, [r1, #0x228]
 8006d12: f8c1 6258    	str.w	r6, [r1, #0x258]
 8006d16: eb09 0402    	add.w	r4, r9, r2
 8006d1a: f8d1 6270    	ldr.w	r6, [r1, #0x270]
 8006d1e: fa44 f403    	asr.w	r4, r4, r3
 8006d22: 4416         	add	r6, r2
 8006d24: f8c1 4240    	str.w	r4, [r1, #0x240]
 8006d28: fa46 f603    	asr.w	r6, r6, r3
 8006d2c: f8c1 6270    	str.w	r6, [r1, #0x270]
 8006d30: f8d1 6288    	ldr.w	r6, [r1, #0x288]
 8006d34: 4416         	add	r6, r2
 8006d36: fa46 f603    	asr.w	r6, r6, r3
 8006d3a: f8c1 6288    	str.w	r6, [r1, #0x288]
 8006d3e: f8d1 62a0    	ldr.w	r6, [r1, #0x2a0]
 8006d42: 4416         	add	r6, r2
 8006d44: fa46 f603    	asr.w	r6, r6, r3
 8006d48: f8c1 62a0    	str.w	r6, [r1, #0x2a0]
 8006d4c: f8d1 62b8    	ldr.w	r6, [r1, #0x2b8]
 8006d50: 4416         	add	r6, r2
 8006d52: fa46 f603    	asr.w	r6, r6, r3
 8006d56: f8c1 62b8    	str.w	r6, [r1, #0x2b8]
 8006d5a: f8d1 62d0    	ldr.w	r6, [r1, #0x2d0]
 8006d5e: 4416         	add	r6, r2
 8006d60: fa46 f603    	asr.w	r6, r6, r3
 8006d64: f8c1 62d0    	str.w	r6, [r1, #0x2d0]
 8006d68: f8d1 62e8    	ldr.w	r6, [r1, #0x2e8]
 8006d6c: 4416         	add	r6, r2
 8006d6e: fa46 f603    	asr.w	r6, r6, r3
 8006d72: f8c1 62e8    	str.w	r6, [r1, #0x2e8]
 8006d76: f501 71c0    	add.w	r1, r1, #0x180
 8006d7a: d181         	bne	0x8006c80 <raylar_crcl_encoder_service::benchmark::rounding::h32a8b1c6cb68e077+0x148> @ imm = #-0xfe
 8006d7c: 6901         	ldr	r1, [r0, #0x10]
 8006d7e: 2601         	movs	r6, #0x1
 8006d80: 9101         	str	r1, [sp, #0x4]
 8006d82: a901         	add	r1, sp, #0x4
 8006d84: f645 7174    	movw	r1, #0x5f74
 8006d88: f44f 6c80    	mov.w	r12, #0x400
 8006d8c: 9b01         	ldr	r3, [sp, #0x4]
 8006d8e: 4401         	add	r1, r0
 8006d90: 1e5a         	subs	r2, r3, #0x1
 8006d92: f003 031f    	and	r3, r3, #0x1f
 8006d96: f002 021f    	and	r2, r2, #0x1f
 8006d9a: fa06 f202    	lsl.w	r2, r6, r2
 8006d9e: f8d1 40d8    	ldr.w	r4, [r1, #0xd8]
 8006da2: f1bc 0c10    	subs.w	r12, r12, #0x10
 8006da6: f8d1 50f0    	ldr.w	r5, [r1, #0xf0]
 8006daa: 4414         	add	r4, r2
 8006dac: f8d1 8108    	ldr.w	r8, [r1, #0x108]
 8006db0: f8d1 60c0    	ldr.w	r6, [r1, #0xc0]
 8006db4: fa44 f403    	asr.w	r4, r4, r3
 8006db8: f8d1 e198    	ldr.w	lr, [r1, #0x198]
 8006dbc: f8c1 40d8    	str.w	r4, [r1, #0xd8]
 8006dc0: eb05 0402    	add.w	r4, r5, r2
 8006dc4: 4416         	add	r6, r2
 8006dc6: f8d1 5120    	ldr.w	r5, [r1, #0x120]
 8006dca: fa44 f403    	asr.w	r4, r4, r3
 8006dce: f8d1 9180    	ldr.w	r9, [r1, #0x180]
 8006dd2: f8c1 40f0    	str.w	r4, [r1, #0xf0]
 8006dd6: eb08 0402    	add.w	r4, r8, r2
 8006dda: fa46 f603    	asr.w	r6, r6, r3
 8006dde: fa44 f403    	asr.w	r4, r4, r3
 8006de2: f8c1 60c0    	str.w	r6, [r1, #0xc0]
 8006de6: f8c1 4108    	str.w	r4, [r1, #0x108]
 8006dea: eb05 0402    	add.w	r4, r5, r2
 8006dee: f8d1 6138    	ldr.w	r6, [r1, #0x138]
 8006df2: fa44 f403    	asr.w	r4, r4, r3
 8006df6: f8d1 5150    	ldr.w	r5, [r1, #0x150]
 8006dfa: f8c1 4120    	str.w	r4, [r1, #0x120]
 8006dfe: eb06 0402    	add.w	r4, r6, r2
 8006e02: f8d1 6168    	ldr.w	r6, [r1, #0x168]
 8006e06: fa44 f403    	asr.w	r4, r4, r3
 8006e0a: f8c1 4138    	str.w	r4, [r1, #0x138]
 8006e0e: eb05 0402    	add.w	r4, r5, r2
 8006e12: fa44 f403    	asr.w	r4, r4, r3
 8006e16: f8c1 4150    	str.w	r4, [r1, #0x150]
 8006e1a: eb06 0402    	add.w	r4, r6, r2
 8006e1e: eb0e 0602    	add.w	r6, lr, r2
 8006e22: fa44 f403    	asr.w	r4, r4, r3
 8006e26: fa46 f603    	asr.w	r6, r6, r3
 8006e2a: f8c1 4168    	str.w	r4, [r1, #0x168]
 8006e2e: f8c1 6198    	str.w	r6, [r1, #0x198]
 8006e32: eb09 0402    	add.w	r4, r9, r2
 8006e36: f8d1 61b0    	ldr.w	r6, [r1, #0x1b0]
 8006e3a: fa44 f403    	asr.w	r4, r4, r3
 8006e3e: 4416         	add	r6, r2
 8006e40: f8c1 4180    	str.w	r4, [r1, #0x180]
 8006e44: fa46 f603    	asr.w	r6, r6, r3
 8006e48: f8c1 61b0    	str.w	r6, [r1, #0x1b0]
 8006e4c: f8d1 61c8    	ldr.w	r6, [r1, #0x1c8]
 8006e50: 4416         	add	r6, r2
 8006e52: fa46 f603    	asr.w	r6, r6, r3
 8006e56: f8c1 61c8    	str.w	r6, [r1, #0x1c8]
 8006e5a: f8d1 61e0    	ldr.w	r6, [r1, #0x1e0]
 8006e5e: 4416         	add	r6, r2
 8006e60: fa46 f603    	asr.w	r6, r6, r3
 8006e64: f8c1 61e0    	str.w	r6, [r1, #0x1e0]
 8006e68: f8d1 61f8    	ldr.w	r6, [r1, #0x1f8]
 8006e6c: 4416         	add	r6, r2
 8006e6e: fa46 f603    	asr.w	r6, r6, r3
 8006e72: f8c1 61f8    	str.w	r6, [r1, #0x1f8]
 8006e76: f8d1 6210    	ldr.w	r6, [r1, #0x210]
 8006e7a: 4416         	add	r6, r2
 8006e7c: fa46 f603    	asr.w	r6, r6, r3
 8006e80: f8c1 6210    	str.w	r6, [r1, #0x210]
 8006e84: f8d1 6228    	ldr.w	r6, [r1, #0x228]
 8006e88: 4416         	add	r6, r2
 8006e8a: fa46 f603    	asr.w	r6, r6, r3
 8006e8e: f8c1 6228    	str.w	r6, [r1, #0x228]
 8006e92: f501 71c0    	add.w	r1, r1, #0x180
 8006e96: d182         	bne	0x8006d9e <raylar_crcl_encoder_service::benchmark::rounding::h32a8b1c6cb68e077+0x266> @ imm = #-0xfc
 8006e98: 6981         	ldr	r1, [r0, #0x18]
 8006e9a: 2601         	movs	r6, #0x1
 8006e9c: 9101         	str	r1, [sp, #0x4]
 8006e9e: a901         	add	r1, sp, #0x4
 8006ea0: f645 61b8    	movw	r1, #0x5eb8
 8006ea4: f44f 6c80    	mov.w	r12, #0x400
 8006ea8: 9b01         	ldr	r3, [sp, #0x4]
 8006eaa: 4401         	add	r1, r0
 8006eac: 1e5a         	subs	r2, r3, #0x1
 8006eae: f003 031f    	and	r3, r3, #0x1f
 8006eb2: f002 021f    	and	r2, r2, #0x1f
 8006eb6: fa06 f202    	lsl.w	r2, r6, r2
 8006eba: f8d1 4198    	ldr.w	r4, [r1, #0x198]
 8006ebe: f1bc 0c10    	subs.w	r12, r12, #0x10
 8006ec2: f8d1 51b0    	ldr.w	r5, [r1, #0x1b0]
 8006ec6: 4414         	add	r4, r2
 8006ec8: f8d1 81c8    	ldr.w	r8, [r1, #0x1c8]
 8006ecc: f8d1 6180    	ldr.w	r6, [r1, #0x180]
 8006ed0: fa44 f403    	asr.w	r4, r4, r3
 8006ed4: f8d1 e258    	ldr.w	lr, [r1, #0x258]
 8006ed8: f8c1 4198    	str.w	r4, [r1, #0x198]
 8006edc: eb05 0402    	add.w	r4, r5, r2
 8006ee0: 4416         	add	r6, r2
 8006ee2: f8d1 51e0    	ldr.w	r5, [r1, #0x1e0]
 8006ee6: fa44 f403    	asr.w	r4, r4, r3
 8006eea: f8d1 9240    	ldr.w	r9, [r1, #0x240]
 8006eee: f8c1 41b0    	str.w	r4, [r1, #0x1b0]
 8006ef2: eb08 0402    	add.w	r4, r8, r2
 8006ef6: fa46 f603    	asr.w	r6, r6, r3
 8006efa: fa44 f403    	asr.w	r4, r4, r3
 8006efe: f8c1 6180    	str.w	r6, [r1, #0x180]
 8006f02: f8c1 41c8    	str.w	r4, [r1, #0x1c8]
 8006f06: eb05 0402    	add.w	r4, r5, r2
 8006f0a: f8d1 61f8    	ldr.w	r6, [r1, #0x1f8]
 8006f0e: fa44 f403    	asr.w	r4, r4, r3
 8006f12: f8d1 5210    	ldr.w	r5, [r1, #0x210]
 8006f16: f8c1 41e0    	str.w	r4, [r1, #0x1e0]
 8006f1a: eb06 0402    	add.w	r4, r6, r2
 8006f1e: f8d1 6228    	ldr.w	r6, [r1, #0x228]
 8006f22: fa44 f403    	asr.w	r4, r4, r3
 8006f26: f8c1 41f8    	str.w	r4, [r1, #0x1f8]
 8006f2a: eb05 0402    	add.w	r4, r5, r2
 8006f2e: fa44 f403    	asr.w	r4, r4, r3
 8006f32: f8c1 4210    	str.w	r4, [r1, #0x210]
 8006f36: eb06 0402    	add.w	r4, r6, r2
 8006f3a: eb0e 0602    	add.w	r6, lr, r2
 8006f3e: fa44 f403    	asr.w	r4, r4, r3
 8006f42: fa46 f603    	asr.w	r6, r6, r3
 8006f46: f8c1 4228    	str.w	r4, [r1, #0x228]
 8006f4a: f8c1 6258    	str.w	r6, [r1, #0x258]
 8006f4e: eb09 0402    	add.w	r4, r9, r2
 8006f52: f8d1 6270    	ldr.w	r6, [r1, #0x270]
 8006f56: fa44 f403    	asr.w	r4, r4, r3
 8006f5a: 4416         	add	r6, r2
 8006f5c: f8c1 4240    	str.w	r4, [r1, #0x240]
 8006f60: fa46 f603    	asr.w	r6, r6, r3
 8006f64: f8c1 6270    	str.w	r6, [r1, #0x270]
 8006f68: f8d1 6288    	ldr.w	r6, [r1, #0x288]
 8006f6c: 4416         	add	r6, r2
 8006f6e: fa46 f603    	asr.w	r6, r6, r3
 8006f72: f8c1 6288    	str.w	r6, [r1, #0x288]
 8006f76: f8d1 62a0    	ldr.w	r6, [r1, #0x2a0]
 8006f7a: 4416         	add	r6, r2
 8006f7c: fa46 f603    	asr.w	r6, r6, r3
 8006f80: f8c1 62a0    	str.w	r6, [r1, #0x2a0]
 8006f84: f8d1 62b8    	ldr.w	r6, [r1, #0x2b8]
 8006f88: 4416         	add	r6, r2
 8006f8a: fa46 f603    	asr.w	r6, r6, r3
 8006f8e: f8c1 62b8    	str.w	r6, [r1, #0x2b8]
 8006f92: f8d1 62d0    	ldr.w	r6, [r1, #0x2d0]
 8006f96: 4416         	add	r6, r2
 8006f98: fa46 f603    	asr.w	r6, r6, r3
 8006f9c: f8c1 62d0    	str.w	r6, [r1, #0x2d0]
 8006fa0: f8d1 62e8    	ldr.w	r6, [r1, #0x2e8]
 8006fa4: 4416         	add	r6, r2
 8006fa6: fa46 f603    	asr.w	r6, r6, r3
 8006faa: f8c1 62e8    	str.w	r6, [r1, #0x2e8]
 8006fae: f501 71c0    	add.w	r1, r1, #0x180
 8006fb2: d182         	bne	0x8006eba <raylar_crcl_encoder_service::benchmark::rounding::h32a8b1c6cb68e077+0x382> @ imm = #-0xfc
 8006fb4: 6a01         	ldr	r1, [r0, #0x20]
 8006fb6: 2601         	movs	r6, #0x1
 8006fb8: 9101         	str	r1, [sp, #0x4]
 8006fba: a901         	add	r1, sp, #0x4
 8006fbc: f645 717c    	movw	r1, #0x5f7c
 8006fc0: f44f 6c80    	mov.w	r12, #0x400
 8006fc4: 9b01         	ldr	r3, [sp, #0x4]
 8006fc6: 4401         	add	r1, r0
 8006fc8: 1e5a         	subs	r2, r3, #0x1
 8006fca: f003 031f    	and	r3, r3, #0x1f
 8006fce: f002 021f    	and	r2, r2, #0x1f
 8006fd2: fa06 f202    	lsl.w	r2, r6, r2
 8006fd6: f8d1 40d8    	ldr.w	r4, [r1, #0xd8]
 8006fda: f1bc 0c10    	subs.w	r12, r12, #0x10
 8006fde: f8d1 50f0    	ldr.w	r5, [r1, #0xf0]
 8006fe2: 4414         	add	r4, r2
 8006fe4: f8d1 8108    	ldr.w	r8, [r1, #0x108]
 8006fe8: f8d1 60c0    	ldr.w	r6, [r1, #0xc0]
 8006fec: fa44 f403    	asr.w	r4, r4, r3
 8006ff0: f8d1 e198    	ldr.w	lr, [r1, #0x198]
 8006ff4: f8c1 40d8    	str.w	r4, [r1, #0xd8]
 8006ff8: eb05 0402    	add.w	r4, r5, r2
 8006ffc: 4416         	add	r6, r2
 8006ffe: f8d1 5120    	ldr.w	r5, [r1, #0x120]
 8007002: fa44 f403    	asr.w	r4, r4, r3
 8007006: f8d1 9180    	ldr.w	r9, [r1, #0x180]
 800700a: f8c1 40f0    	str.w	r4, [r1, #0xf0]
 800700e: eb08 0402    	add.w	r4, r8, r2
 8007012: fa46 f603    	asr.w	r6, r6, r3
 8007016: fa44 f403    	asr.w	r4, r4, r3
 800701a: f8c1 60c0    	str.w	r6, [r1, #0xc0]
 800701e: f8c1 4108    	str.w	r4, [r1, #0x108]
 8007022: eb05 0402    	add.w	r4, r5, r2
 8007026: f8d1 6138    	ldr.w	r6, [r1, #0x138]
 800702a: fa44 f403    	asr.w	r4, r4, r3
 800702e: f8d1 5150    	ldr.w	r5, [r1, #0x150]
 8007032: f8c1 4120    	str.w	r4, [r1, #0x120]
 8007036: eb06 0402    	add.w	r4, r6, r2
 800703a: f8d1 6168    	ldr.w	r6, [r1, #0x168]
 800703e: fa44 f403    	asr.w	r4, r4, r3
 8007042: f8c1 4138    	str.w	r4, [r1, #0x138]
 8007046: eb05 0402    	add.w	r4, r5, r2
 800704a: fa44 f403    	asr.w	r4, r4, r3
 800704e: f8c1 4150    	str.w	r4, [r1, #0x150]
 8007052: eb06 0402    	add.w	r4, r6, r2
 8007056: eb0e 0602    	add.w	r6, lr, r2
 800705a: fa44 f403    	asr.w	r4, r4, r3
 800705e: fa46 f603    	asr.w	r6, r6, r3
 8007062: f8c1 4168    	str.w	r4, [r1, #0x168]
 8007066: f8c1 6198    	str.w	r6, [r1, #0x198]
 800706a: eb09 0402    	add.w	r4, r9, r2
 800706e: f8d1 61b0    	ldr.w	r6, [r1, #0x1b0]
 8007072: fa44 f403    	asr.w	r4, r4, r3
 8007076: 4416         	add	r6, r2
 8007078: f8c1 4180    	str.w	r4, [r1, #0x180]
 800707c: fa46 f603    	asr.w	r6, r6, r3
 8007080: f8c1 61b0    	str.w	r6, [r1, #0x1b0]
 8007084: f8d1 61c8    	ldr.w	r6, [r1, #0x1c8]
 8007088: 4416         	add	r6, r2
 800708a: fa46 f603    	asr.w	r6, r6, r3
 800708e: f8c1 61c8    	str.w	r6, [r1, #0x1c8]
 8007092: f8d1 61e0    	ldr.w	r6, [r1, #0x1e0]
 8007096: 4416         	add	r6, r2
 8007098: fa46 f603    	asr.w	r6, r6, r3
 800709c: f8c1 61e0    	str.w	r6, [r1, #0x1e0]
 80070a0: f8d1 61f8    	ldr.w	r6, [r1, #0x1f8]
 80070a4: 4416         	add	r6, r2
 80070a6: fa46 f603    	asr.w	r6, r6, r3
 80070aa: f8c1 61f8    	str.w	r6, [r1, #0x1f8]
 80070ae: f8d1 6210    	ldr.w	r6, [r1, #0x210]
 80070b2: 4416         	add	r6, r2
 80070b4: fa46 f603    	asr.w	r6, r6, r3
 80070b8: f8c1 6210    	str.w	r6, [r1, #0x210]
 80070bc: f8d1 6228    	ldr.w	r6, [r1, #0x228]
 80070c0: 4416         	add	r6, r2
 80070c2: fa46 f603    	asr.w	r6, r6, r3
 80070c6: f8c1 6228    	str.w	r6, [r1, #0x228]
 80070ca: f501 71c0    	add.w	r1, r1, #0x180
 80070ce: d182         	bne	0x8006fd6 <raylar_crcl_encoder_service::benchmark::rounding::h32a8b1c6cb68e077+0x49e> @ imm = #-0xfc
 80070d0: f246 0128    	movw	r1, #0x6028
 80070d4: 4401         	add	r1, r0
 80070d6: 9100         	str	r1, [sp]
 80070d8: 4669         	mov	r1, sp
 80070da: f246 012c    	movw	r1, #0x602c
 80070de: 5840         	ldr	r0, [r0, r1]
 80070e0: b002         	add	sp, #0x8
 80070e2: e8bd 0b00    	pop.w	{r8, r9, r11}
 80070e6: bdf0         	pop	{r4, r5, r6, r7, pc}

080070e8 <raylar_crcl_encoder_service::benchmark::sum_only::he5272faae351e30d>:
 80070e8: b5f0         	push	{r4, r5, r6, r7, lr}
 80070ea: af03         	add	r7, sp, #0xc
 80070ec: e92d 0f00    	push.w	{r8, r9, r10, r11}
 80070f0: b08a         	sub	sp, #0x28
 80070f2: 9004         	str	r0, [sp, #0x10]
 80070f4: f500 30b8    	add.w	r0, r0, #0x17000
 80070f8: 3010         	adds	r0, #0x10
 80070fa: 2500         	movs	r5, #0x0
 80070fc: f240 39ff    	movw	r9, #0x3ff
 8007100: f850 1f84    	ldr	r1, [r0, #132]!
 8007104: f1b9 0921    	subs.w	r9, r9, #0x21
 8007108: f1a0 0c78    	sub.w	r12, r0, #0x78
 800710c: 9108         	str	r1, [sp, #0x20]
 800710e: e950 4220    	ldrd	r4, r2, [r0, #-128]
 8007112: f1a0 0e60    	sub.w	lr, r0, #0x60
 8007116: e89c 1048    	ldm.w	r12, {r3, r6, r12}
 800711a: 442c         	add	r4, r5
 800711c: 4422         	add	r2, r4
 800711e: f850 1c04    	ldr	r1, [r0, #-4]
 8007122: 441a         	add	r2, r3
 8007124: 9107         	str	r1, [sp, #0x1c]
 8007126: f850 1c08    	ldr	r1, [r0, #-8]
 800712a: 4432         	add	r2, r6
 800712c: 9106         	str	r1, [sp, #0x18]
 800712e: f850 1c0c    	ldr	r1, [r0, #-12]
 8007132: e950 ab1b    	ldrd	r10, r11, [r0, #-108]
 8007136: 9105         	str	r1, [sp, #0x14]
 8007138: eb0c 0102    	add.w	r1, r12, r2
 800713c: 4451         	add	r1, r10
 800713e: f850 8c64    	ldr	r8, [r0, #-100]
 8007142: 4459         	add	r1, r11
 8007144: e89e 545c    	ldm.w	lr, {r2, r3, r4, r6, r10, r12, lr}
 8007148: 4441         	add	r1, r8
 800714a: 4411         	add	r1, r2
 800714c: f850 5c44    	ldr	r5, [r0, #-68]
 8007150: 4419         	add	r1, r3
 8007152: f1a0 0b2c    	sub.w	r11, r0, #0x2c
 8007156: 4421         	add	r1, r4
 8007158: 4431         	add	r1, r6
 800715a: f1a0 0640    	sub.w	r6, r0, #0x40
 800715e: 4451         	add	r1, r10
 8007160: e89b 0d00    	ldm.w	r11, {r8, r10, r11}
 8007164: 4461         	add	r1, r12
 8007166: 4471         	add	r1, lr
 8007168: ce4c         	ldm	r6, {r2, r3, r6}
 800716a: 4429         	add	r1, r5
 800716c: 4411         	add	r1, r2
 800716e: e950 4c0d    	ldrd	r4, r12, [r0, #-52]
 8007172: 4419         	add	r1, r3
 8007174: f850 2c20    	ldr	r2, [r0, #-32]
 8007178: 4431         	add	r1, r6
 800717a: f850 3c1c    	ldr	r3, [r0, #-28]
 800717e: 4421         	add	r1, r4
 8007180: f850 6c18    	ldr	r6, [r0, #-24]
 8007184: 4461         	add	r1, r12
 8007186: 4441         	add	r1, r8
 8007188: 4451         	add	r1, r10
 800718a: 4459         	add	r1, r11
 800718c: 4411         	add	r1, r2
 800718e: f850 2c14    	ldr	r2, [r0, #-20]
 8007192: 4419         	add	r1, r3
 8007194: f850 3c10    	ldr	r3, [r0, #-16]
 8007198: 4431         	add	r1, r6
 800719a: 4411         	add	r1, r2
 800719c: 9a05         	ldr	r2, [sp, #0x14]
 800719e: 4419         	add	r1, r3
 80071a0: 4411         	add	r1, r2
 80071a2: 9a06         	ldr	r2, [sp, #0x18]
 80071a4: 4411         	add	r1, r2
 80071a6: 9a07         	ldr	r2, [sp, #0x1c]
 80071a8: 4411         	add	r1, r2
 80071aa: 9a08         	ldr	r2, [sp, #0x20]
 80071ac: eb02 0501    	add.w	r5, r2, r1
 80071b0: d1a6         	bne	0x8007100 <raylar_crcl_encoder_service::benchmark::sum_only::he5272faae351e30d+0x18> @ imm = #-0xb4
 80071b2: 9509         	str	r5, [sp, #0x24]
 80071b4: a809         	add	r0, sp, #0x24
 80071b6: 2500         	movs	r5, #0x0
 80071b8: f240 39ff    	movw	r9, #0x3ff
 80071bc: 9809         	ldr	r0, [sp, #0x24]
 80071be: 9003         	str	r0, [sp, #0xc]
 80071c0: 9804         	ldr	r0, [sp, #0x10]
 80071c2: f500 30c0    	add.w	r0, r0, #0x18000
 80071c6: 300c         	adds	r0, #0xc
 80071c8: f850 1f84    	ldr	r1, [r0, #132]!
 80071cc: f1b9 0921    	subs.w	r9, r9, #0x21
 80071d0: f1a0 0c78    	sub.w	r12, r0, #0x78
 80071d4: 9108         	str	r1, [sp, #0x20]
 80071d6: e950 4220    	ldrd	r4, r2, [r0, #-128]
 80071da: f1a0 0e60    	sub.w	lr, r0, #0x60
 80071de: e89c 1048    	ldm.w	r12, {r3, r6, r12}
 80071e2: 442c         	add	r4, r5
 80071e4: 4422         	add	r2, r4
 80071e6: f850 1c04    	ldr	r1, [r0, #-4]
 80071ea: 441a         	add	r2, r3
 80071ec: 9107         	str	r1, [sp, #0x1c]
 80071ee: f850 1c08    	ldr	r1, [r0, #-8]
 80071f2: 4432         	add	r2, r6
 80071f4: 9106         	str	r1, [sp, #0x18]
 80071f6: f850 1c0c    	ldr	r1, [r0, #-12]
 80071fa: e950 ab1b    	ldrd	r10, r11, [r0, #-108]
 80071fe: 9105         	str	r1, [sp, #0x14]
 8007200: eb0c 0102    	add.w	r1, r12, r2
 8007204: 4451         	add	r1, r10
 8007206: f850 8c64    	ldr	r8, [r0, #-100]
 800720a: 4459         	add	r1, r11
 800720c: e89e 545c    	ldm.w	lr, {r2, r3, r4, r6, r10, r12, lr}
 8007210: 4441         	add	r1, r8
 8007212: 4411         	add	r1, r2
 8007214: f850 5c44    	ldr	r5, [r0, #-68]
 8007218: 4419         	add	r1, r3
 800721a: f1a0 0b2c    	sub.w	r11, r0, #0x2c
 800721e: 4421         	add	r1, r4
 8007220: 4431         	add	r1, r6
 8007222: f1a0 0640    	sub.w	r6, r0, #0x40
 8007226: 4451         	add	r1, r10
 8007228: e89b 0d00    	ldm.w	r11, {r8, r10, r11}
 800722c: 4461         	add	r1, r12
 800722e: 4471         	add	r1, lr
 8007230: ce4c         	ldm	r6, {r2, r3, r6}
 8007232: 4429         	add	r1, r5
 8007234: 4411         	add	r1, r2
 8007236: e950 4c0d    	ldrd	r4, r12, [r0, #-52]
 800723a: 4419         	add	r1, r3
 800723c: f850 2c20    	ldr	r2, [r0, #-32]
 8007240: 4431         	add	r1, r6
 8007242: f850 3c1c    	ldr	r3, [r0, #-28]
 8007246: 4421         	add	r1, r4
 8007248: f850 6c18    	ldr	r6, [r0, #-24]
 800724c: 4461         	add	r1, r12
 800724e: 4441         	add	r1, r8
 8007250: 4451         	add	r1, r10
 8007252: 4459         	add	r1, r11
 8007254: 4411         	add	r1, r2
 8007256: f850 2c14    	ldr	r2, [r0, #-20]
 800725a: 4419         	add	r1, r3
 800725c: f850 3c10    	ldr	r3, [r0, #-16]
 8007260: 4431         	add	r1, r6
 8007262: 4411         	add	r1, r2
 8007264: 9a05         	ldr	r2, [sp, #0x14]
 8007266: 4419         	add	r1, r3
 8007268: 4411         	add	r1, r2
 800726a: 9a06         	ldr	r2, [sp, #0x18]
 800726c: 4411         	add	r1, r2
 800726e: 9a07         	ldr	r2, [sp, #0x1c]
 8007270: 4411         	add	r1, r2
 8007272: 9a08         	ldr	r2, [sp, #0x20]
 8007274: eb02 0501    	add.w	r5, r2, r1
 8007278: d1a6         	bne	0x80071c8 <raylar_crcl_encoder_service::benchmark::sum_only::he5272faae351e30d+0xe0> @ imm = #-0xb4
 800727a: 9509         	str	r5, [sp, #0x24]
 800727c: a809         	add	r0, sp, #0x24
 800727e: 2500         	movs	r5, #0x0
 8007280: f240 39ff    	movw	r9, #0x3ff
 8007284: 9809         	ldr	r0, [sp, #0x24]
 8007286: 9002         	str	r0, [sp, #0x8]
 8007288: 9804         	ldr	r0, [sp, #0x10]
 800728a: f500 30c8    	add.w	r0, r0, #0x19000
 800728e: 3008         	adds	r0, #0x8
 8007290: f850 1f84    	ldr	r1, [r0, #132]!
 8007294: f1b9 0921    	subs.w	r9, r9, #0x21
 8007298: f1a0 0c78    	sub.w	r12, r0, #0x78
 800729c: 9108         	str	r1, [sp, #0x20]
 800729e: e950 4220    	ldrd	r4, r2, [r0, #-128]
 80072a2: f1a0 0e60    	sub.w	lr, r0, #0x60
 80072a6: e89c 1048    	ldm.w	r12, {r3, r6, r12}
 80072aa: 442c         	add	r4, r5
 80072ac: 4422         	add	r2, r4
 80072ae: f850 1c04    	ldr	r1, [r0, #-4]
 80072b2: 441a         	add	r2, r3
 80072b4: 9107         	str	r1, [sp, #0x1c]
 80072b6: f850 1c08    	ldr	r1, [r0, #-8]
 80072ba: 4432         	add	r2, r6
 80072bc: 9106         	str	r1, [sp, #0x18]
 80072be: f850 1c0c    	ldr	r1, [r0, #-12]
 80072c2: e950 ab1b    	ldrd	r10, r11, [r0, #-108]
 80072c6: 9105         	str	r1, [sp, #0x14]
 80072c8: eb0c 0102    	add.w	r1, r12, r2
 80072cc: 4451         	add	r1, r10
 80072ce: f850 8c64    	ldr	r8, [r0, #-100]
 80072d2: 4459         	add	r1, r11
 80072d4: e89e 545c    	ldm.w	lr, {r2, r3, r4, r6, r10, r12, lr}
 80072d8: 4441         	add	r1, r8
 80072da: 4411         	add	r1, r2
 80072dc: f850 5c44    	ldr	r5, [r0, #-68]
 80072e0: 4419         	add	r1, r3
 80072e2: f1a0 0b2c    	sub.w	r11, r0, #0x2c
 80072e6: 4421         	add	r1, r4
 80072e8: 4431         	add	r1, r6
 80072ea: f1a0 0640    	sub.w	r6, r0, #0x40
 80072ee: 4451         	add	r1, r10
 80072f0: e89b 0d00    	ldm.w	r11, {r8, r10, r11}
 80072f4: 4461         	add	r1, r12
 80072f6: 4471         	add	r1, lr
 80072f8: ce4c         	ldm	r6, {r2, r3, r6}
 80072fa: 4429         	add	r1, r5
 80072fc: 4411         	add	r1, r2
 80072fe: e950 4c0d    	ldrd	r4, r12, [r0, #-52]
 8007302: 4419         	add	r1, r3
 8007304: f850 2c20    	ldr	r2, [r0, #-32]
 8007308: 4431         	add	r1, r6
 800730a: f850 3c1c    	ldr	r3, [r0, #-28]
 800730e: 4421         	add	r1, r4
 8007310: f850 6c18    	ldr	r6, [r0, #-24]
 8007314: 4461         	add	r1, r12
 8007316: 4441         	add	r1, r8
 8007318: 4451         	add	r1, r10
 800731a: 4459         	add	r1, r11
 800731c: 4411         	add	r1, r2
 800731e: f850 2c14    	ldr	r2, [r0, #-20]
 8007322: 4419         	add	r1, r3
 8007324: f850 3c10    	ldr	r3, [r0, #-16]
 8007328: 4431         	add	r1, r6
 800732a: 4411         	add	r1, r2
 800732c: 9a05         	ldr	r2, [sp, #0x14]
 800732e: 4419         	add	r1, r3
 8007330: 4411         	add	r1, r2
 8007332: 9a06         	ldr	r2, [sp, #0x18]
 8007334: 4411         	add	r1, r2
 8007336: 9a07         	ldr	r2, [sp, #0x1c]
 8007338: 4411         	add	r1, r2
 800733a: 9a08         	ldr	r2, [sp, #0x20]
 800733c: eb02 0501    	add.w	r5, r2, r1
 8007340: d1a6         	bne	0x8007290 <raylar_crcl_encoder_service::benchmark::sum_only::he5272faae351e30d+0x1a8> @ imm = #-0xb4
 8007342: 9509         	str	r5, [sp, #0x24]
 8007344: a809         	add	r0, sp, #0x24
 8007346: f24f 0104    	movw	r1, #0xf004
 800734a: f24b 046c    	movw	r4, #0xb06c
 800734e: 9a04         	ldr	r2, [sp, #0x10]
 8007350: f24b 0574    	movw	r5, #0xb074
 8007354: f24b 0c70    	movw	r12, #0xb070
 8007358: 9809         	ldr	r0, [sp, #0x24]
 800735a: f04f 0b00    	mov.w	r11, #0x0
 800735e: f6cf 71ff    	movt	r1, #0xffff
 8007362: f2c0 0401    	movt	r4, #0x1
 8007366: f2c0 0501    	movt	r5, #0x1
 800736a: f2c0 0c01    	movt	r12, #0x1
 800736e: 9001         	str	r0, [sp, #0x4]
 8007370: f24b 0384    	movw	r3, #0xb084
 8007374: 1850         	adds	r0, r2, r1
 8007376: f2c0 0301    	movt	r3, #0x1
 800737a: 3184         	adds	r1, #0x84
 800737c: f850 e003    	ldr.w	lr, [r0, r3]
 8007380: f24b 0380    	movw	r3, #0xb080
 8007384: f2c0 0301    	movt	r3, #0x1
 8007388: f850 8003    	ldr.w	r8, [r0, r3]
 800738c: f24b 037c    	movw	r3, #0xb07c
 8007390: f2c0 0301    	movt	r3, #0x1
 8007394: f850 9003    	ldr.w	r9, [r0, r3]
 8007398: f24b 0378    	movw	r3, #0xb078
 800739c: f2c0 0301    	movt	r3, #0x1
 80073a0: f850 a003    	ldr.w	r10, [r0, r3]
 80073a4: f24b 0308    	movw	r3, #0xb008
 80073a8: f2c0 0301    	movt	r3, #0x1
 80073ac: 58c6         	ldr	r6, [r0, r3]
 80073ae: f24b 0304    	movw	r3, #0xb004
 80073b2: f2c0 0301    	movt	r3, #0x1
 80073b6: 58c3         	ldr	r3, [r0, r3]
 80073b8: 445b         	add	r3, r11
 80073ba: 4433         	add	r3, r6
 80073bc: f24b 060c    	movw	r6, #0xb00c
 80073c0: f2c0 0601    	movt	r6, #0x1
 80073c4: 5986         	ldr	r6, [r0, r6]
 80073c6: 4433         	add	r3, r6
 80073c8: f24b 0610    	movw	r6, #0xb010
 80073cc: f2c0 0601    	movt	r6, #0x1
 80073d0: 5986         	ldr	r6, [r0, r6]
 80073d2: 4433         	add	r3, r6
 80073d4: f24b 0614    	movw	r6, #0xb014
 80073d8: f2c0 0601    	movt	r6, #0x1
 80073dc: 5986         	ldr	r6, [r0, r6]
 80073de: 4433         	add	r3, r6
 80073e0: f24b 0618    	movw	r6, #0xb018
 80073e4: f2c0 0601    	movt	r6, #0x1
 80073e8: 5986         	ldr	r6, [r0, r6]
 80073ea: 4433         	add	r3, r6
 80073ec: f24b 061c    	movw	r6, #0xb01c
 80073f0: f2c0 0601    	movt	r6, #0x1
 80073f4: 5986         	ldr	r6, [r0, r6]
 80073f6: 4433         	add	r3, r6
 80073f8: f24b 0620    	movw	r6, #0xb020
 80073fc: f2c0 0601    	movt	r6, #0x1
 8007400: 5986         	ldr	r6, [r0, r6]
 8007402: 4433         	add	r3, r6
 8007404: f24b 0624    	movw	r6, #0xb024
 8007408: f2c0 0601    	movt	r6, #0x1
 800740c: 5986         	ldr	r6, [r0, r6]
 800740e: 4433         	add	r3, r6
 8007410: f24b 0628    	movw	r6, #0xb028
 8007414: f2c0 0601    	movt	r6, #0x1
 8007418: 5986         	ldr	r6, [r0, r6]
 800741a: 4433         	add	r3, r6
 800741c: f24b 062c    	movw	r6, #0xb02c
 8007420: f2c0 0601    	movt	r6, #0x1
 8007424: 5986         	ldr	r6, [r0, r6]
 8007426: 4433         	add	r3, r6
 8007428: f24b 0630    	movw	r6, #0xb030
 800742c: f2c0 0601    	movt	r6, #0x1
 8007430: 5986         	ldr	r6, [r0, r6]
 8007432: 4433         	add	r3, r6
 8007434: f24b 0634    	movw	r6, #0xb034
 8007438: f2c0 0601    	movt	r6, #0x1
 800743c: 5986         	ldr	r6, [r0, r6]
 800743e: 4433         	add	r3, r6
 8007440: f24b 0638    	movw	r6, #0xb038
 8007444: f2c0 0601    	movt	r6, #0x1
 8007448: 5986         	ldr	r6, [r0, r6]
 800744a: 4433         	add	r3, r6
 800744c: f24b 063c    	movw	r6, #0xb03c
 8007450: f2c0 0601    	movt	r6, #0x1
 8007454: 5986         	ldr	r6, [r0, r6]
 8007456: 4433         	add	r3, r6
 8007458: f24b 0640    	movw	r6, #0xb040
 800745c: f2c0 0601    	movt	r6, #0x1
 8007460: 5986         	ldr	r6, [r0, r6]
 8007462: 4433         	add	r3, r6
 8007464: f24b 0644    	movw	r6, #0xb044
 8007468: f2c0 0601    	movt	r6, #0x1
 800746c: 5986         	ldr	r6, [r0, r6]
 800746e: 4433         	add	r3, r6
 8007470: f24b 0648    	movw	r6, #0xb048
 8007474: f2c0 0601    	movt	r6, #0x1
 8007478: 5986         	ldr	r6, [r0, r6]
 800747a: 4433         	add	r3, r6
 800747c: f24b 064c    	movw	r6, #0xb04c
 8007480: f2c0 0601    	movt	r6, #0x1
 8007484: 5986         	ldr	r6, [r0, r6]
 8007486: 4433         	add	r3, r6
 8007488: f24b 0650    	movw	r6, #0xb050
 800748c: f2c0 0601    	movt	r6, #0x1
 8007490: 5986         	ldr	r6, [r0, r6]
 8007492: 4433         	add	r3, r6
 8007494: f24b 0654    	movw	r6, #0xb054
 8007498: f2c0 0601    	movt	r6, #0x1
 800749c: 5986         	ldr	r6, [r0, r6]
 800749e: 4433         	add	r3, r6
 80074a0: f24b 0658    	movw	r6, #0xb058
 80074a4: f2c0 0601    	movt	r6, #0x1
 80074a8: 5986         	ldr	r6, [r0, r6]
 80074aa: 4433         	add	r3, r6
 80074ac: f24b 065c    	movw	r6, #0xb05c
 80074b0: f2c0 0601    	movt	r6, #0x1
 80074b4: 5986         	ldr	r6, [r0, r6]
 80074b6: 4433         	add	r3, r6
 80074b8: f24b 0660    	movw	r6, #0xb060
 80074bc: f2c0 0601    	movt	r6, #0x1
 80074c0: 5986         	ldr	r6, [r0, r6]
 80074c2: 4433         	add	r3, r6
 80074c4: f24b 0664    	movw	r6, #0xb064
 80074c8: f2c0 0601    	movt	r6, #0x1
 80074cc: 5986         	ldr	r6, [r0, r6]
 80074ce: 4433         	add	r3, r6
 80074d0: f24b 0668    	movw	r6, #0xb068
 80074d4: f2c0 0601    	movt	r6, #0x1
 80074d8: 5986         	ldr	r6, [r0, r6]
 80074da: 4433         	add	r3, r6
 80074dc: 5906         	ldr	r6, [r0, r4]
 80074de: 4433         	add	r3, r6
 80074e0: 5946         	ldr	r6, [r0, r5]
 80074e2: f850 000c    	ldr.w	r0, [r0, r12]
 80074e6: 4418         	add	r0, r3
 80074e8: 4430         	add	r0, r6
 80074ea: 4450         	add	r0, r10
 80074ec: 4448         	add	r0, r9
 80074ee: 4440         	add	r0, r8
 80074f0: eb0e 0b00    	add.w	r11, lr, r0
 80074f4: f47f af3c    	bne.w	0x8007370 <raylar_crcl_encoder_service::benchmark::sum_only::he5272faae351e30d+0x288> @ imm = #-0x188
 80074f8: f8cd b024    	str.w	r11, [sp, #0x24]
 80074fc: a809         	add	r0, sp, #0x24
 80074fe: f502 31d8    	add.w	r1, r2, #0x1b000
 8007502: 2400         	movs	r4, #0x0
 8007504: 9809         	ldr	r0, [sp, #0x24]
 8007506: f240 3aff    	movw	r10, #0x3ff
 800750a: 9000         	str	r0, [sp]
 800750c: f851 0f84    	ldr	r0, [r1, #132]!
 8007510: f1ba 0a21    	subs.w	r10, r10, #0x21
 8007514: f1a1 0680    	sub.w	r6, r1, #0x80
 8007518: 9008         	str	r0, [sp, #0x20]
 800751a: e951 5c1d    	ldrd	r5, r12, [r1, #-116]
 800751e: f1a1 0e60    	sub.w	lr, r1, #0x60
 8007522: ce49         	ldm	r6, {r0, r3, r6}
 8007524: 4420         	add	r0, r4
 8007526: f851 2c04    	ldr	r2, [r1, #-4]
 800752a: 4418         	add	r0, r3
 800752c: e951 b81b    	ldrd	r11, r8, [r1, #-108]
 8007530: 4430         	add	r0, r6
 8007532: 9207         	str	r2, [sp, #0x1c]
 8007534: 4428         	add	r0, r5
 8007536: f851 2c08    	ldr	r2, [r1, #-8]
 800753a: 4460         	add	r0, r12
 800753c: 9206         	str	r2, [sp, #0x18]
 800753e: 4458         	add	r0, r11
 8007540: f851 2c0c    	ldr	r2, [r1, #-12]
 8007544: f851 9c64    	ldr	r9, [r1, #-100]
 8007548: 4440         	add	r0, r8
 800754a: 9205         	str	r2, [sp, #0x14]
 800754c: e89e 586c    	ldm.w	lr, {r2, r3, r5, r6, r11, r12, lr}
 8007550: 4448         	add	r0, r9
 8007552: 4410         	add	r0, r2
 8007554: f851 4c44    	ldr	r4, [r1, #-68]
 8007558: 4418         	add	r0, r3
 800755a: 4428         	add	r0, r5
 800755c: 4430         	add	r0, r6
 800755e: f1a1 0640    	sub.w	r6, r1, #0x40
 8007562: 4458         	add	r0, r11
 8007564: f1a1 0b2c    	sub.w	r11, r1, #0x2c
 8007568: 4460         	add	r0, r12
 800756a: ce4c         	ldm	r6, {r2, r3, r6}
 800756c: 4470         	add	r0, lr
 800756e: 4420         	add	r0, r4
 8007570: e951 5c0d    	ldrd	r5, r12, [r1, #-52]
 8007574: 4410         	add	r0, r2
 8007576: e89b 0b00    	ldm.w	r11, {r8, r9, r11}
 800757a: 4418         	add	r0, r3
 800757c: 4430         	add	r0, r6
 800757e: f851 2c20    	ldr	r2, [r1, #-32]
 8007582: 4428         	add	r0, r5
 8007584: f851 3c1c    	ldr	r3, [r1, #-28]
 8007588: 4460         	add	r0, r12
 800758a: f851 6c18    	ldr	r6, [r1, #-24]
 800758e: 4440         	add	r0, r8
 8007590: 4448         	add	r0, r9
 8007592: 4458         	add	r0, r11
 8007594: 4410         	add	r0, r2
 8007596: f851 2c14    	ldr	r2, [r1, #-20]
 800759a: 4418         	add	r0, r3
 800759c: f851 3c10    	ldr	r3, [r1, #-16]
 80075a0: 4430         	add	r0, r6
 80075a2: 4410         	add	r0, r2
 80075a4: 9a05         	ldr	r2, [sp, #0x14]
 80075a6: 4418         	add	r0, r3
 80075a8: 4410         	add	r0, r2
 80075aa: 9a06         	ldr	r2, [sp, #0x18]
 80075ac: 4410         	add	r0, r2
 80075ae: 9a07         	ldr	r2, [sp, #0x1c]
 80075b0: 4410         	add	r0, r2
 80075b2: 9a08         	ldr	r2, [sp, #0x20]
 80075b4: eb02 0400    	add.w	r4, r2, r0
 80075b8: d1a8         	bne	0x800750c <raylar_crcl_encoder_service::benchmark::sum_only::he5272faae351e30d+0x424> @ imm = #-0xb0
 80075ba: 9409         	str	r4, [sp, #0x24]
 80075bc: a809         	add	r0, sp, #0x24
 80075be: e9dd 1002    	ldrd	r1, r0, [sp, #8]
 80075c2: 9a00         	ldr	r2, [sp]
 80075c4: 4048         	eors	r0, r1
 80075c6: 9901         	ldr	r1, [sp, #0x4]
 80075c8: 4048         	eors	r0, r1
 80075ca: 9909         	ldr	r1, [sp, #0x24]
 80075cc: 4050         	eors	r0, r2
 80075ce: 4048         	eors	r0, r1
 80075d0: b00a         	add	sp, #0x28
 80075d2: e8bd 0f00    	pop.w	{r8, r9, r10, r11}
 80075d6: bdf0         	pop	{r4, r5, r6, r7, pc}

080075d8 <raylar_crcl_encoder_service::benchmark::temporal::h98ba012ed5b5e4c9>:
 80075d8: b5f0         	push	{r4, r5, r6, r7, lr}
 80075da: af03         	add	r7, sp, #0xc
 80075dc: f84d bd04    	str	r11, [sp, #-4]!
 80075e0: b081         	sub	sp, #0x4
 80075e2: f24c 012c    	movw	r1, #0xc02c
 80075e6: f500 3290    	add.w	r2, r0, #0x12000
 80075ea: 5844         	ldr	r4, [r0, r1]
 80075ec: eb00 0e01    	add.w	lr, r0, r1
 80075f0: f500 3190    	add.w	r1, r0, #0x12000
 80075f4: f102 0324    	add.w	r3, r2, #0x24
 80075f8: f101 0c28    	add.w	r12, r1, #0x28
 80075fc: f240 31ff    	movw	r1, #0x3ff
 8007600: 4672         	mov	r2, lr
 8007602: 6995         	ldr	r5, [r2, #0x18]
 8007604: 390b         	subs	r1, #0xb
 8007606: eba5 0404    	sub.w	r4, r5, r4
 800760a: 605c         	str	r4, [r3, #0x4]
 800760c: 6b14         	ldr	r4, [r2, #0x30]
 800760e: eba4 0505    	sub.w	r5, r4, r5
 8007612: 609d         	str	r5, [r3, #0x8]
 8007614: 6c95         	ldr	r5, [r2, #0x48]
 8007616: eba5 0404    	sub.w	r4, r5, r4
 800761a: 60dc         	str	r4, [r3, #0xc]
 800761c: 6e14         	ldr	r4, [r2, #0x60]
 800761e: eba4 0505    	sub.w	r5, r4, r5
 8007622: 611d         	str	r5, [r3, #0x10]
 8007624: 6f95         	ldr	r5, [r2, #0x78]
 8007626: eba5 0404    	sub.w	r4, r5, r4
 800762a: 615c         	str	r4, [r3, #0x14]
 800762c: f8d2 4090    	ldr.w	r4, [r2, #0x90]
 8007630: eba4 0505    	sub.w	r5, r4, r5
 8007634: 619d         	str	r5, [r3, #0x18]
 8007636: f8d2 50a8    	ldr.w	r5, [r2, #0xa8]
 800763a: eba5 0404    	sub.w	r4, r5, r4
 800763e: 61dc         	str	r4, [r3, #0x1c]
 8007640: f8d2 40c0    	ldr.w	r4, [r2, #0xc0]
 8007644: eba4 0505    	sub.w	r5, r4, r5
 8007648: 621d         	str	r5, [r3, #0x20]
 800764a: f8d2 50d8    	ldr.w	r5, [r2, #0xd8]
 800764e: eba5 0404    	sub.w	r4, r5, r4
 8007652: 625c         	str	r4, [r3, #0x24]
 8007654: f8d2 60f0    	ldr.w	r6, [r2, #0xf0]
 8007658: eba6 0405    	sub.w	r4, r6, r5
 800765c: 629c         	str	r4, [r3, #0x28]
 800765e: f8d2 4108    	ldr.w	r4, [r2, #0x108]
 8007662: f502 7284    	add.w	r2, r2, #0x108
 8007666: eba4 0506    	sub.w	r5, r4, r6
 800766a: f843 5f2c    	str	r5, [r3, #44]!
 800766e: d1c8         	bne	0x8007602 <raylar_crcl_encoder_service::benchmark::temporal::h98ba012ed5b5e4c9+0x2a> @ imm = #-0x70
 8007670: f500 3190    	add.w	r1, r0, #0x12000
 8007674: f240 32ff    	movw	r2, #0x3ff
 8007678: f501 5381    	add.w	r3, r1, #0x1020
 800767c: 4671         	mov	r1, lr
 800767e: f851 4f04    	ldr	r4, [r1, #4]!
 8007682: 698d         	ldr	r5, [r1, #0x18]
 8007684: 3a0b         	subs	r2, #0xb
 8007686: eba5 0404    	sub.w	r4, r5, r4
 800768a: 605c         	str	r4, [r3, #0x4]
 800768c: 6b0c         	ldr	r4, [r1, #0x30]
 800768e: eba4 0505    	sub.w	r5, r4, r5
 8007692: 609d         	str	r5, [r3, #0x8]
 8007694: 6c8d         	ldr	r5, [r1, #0x48]
 8007696: eba5 0404    	sub.w	r4, r5, r4
 800769a: 60dc         	str	r4, [r3, #0xc]
 800769c: 6e0c         	ldr	r4, [r1, #0x60]
 800769e: eba4 0505    	sub.w	r5, r4, r5
 80076a2: 611d         	str	r5, [r3, #0x10]
 80076a4: 6f8d         	ldr	r5, [r1, #0x78]
 80076a6: eba5 0404    	sub.w	r4, r5, r4
 80076aa: 615c         	str	r4, [r3, #0x14]
 80076ac: f8d1 4090    	ldr.w	r4, [r1, #0x90]
 80076b0: eba4 0505    	sub.w	r5, r4, r5
 80076b4: 619d         	str	r5, [r3, #0x18]
 80076b6: f8d1 50a8    	ldr.w	r5, [r1, #0xa8]
 80076ba: eba5 0404    	sub.w	r4, r5, r4
 80076be: 61dc         	str	r4, [r3, #0x1c]
 80076c0: f8d1 40c0    	ldr.w	r4, [r1, #0xc0]
 80076c4: eba4 0505    	sub.w	r5, r4, r5
 80076c8: 621d         	str	r5, [r3, #0x20]
 80076ca: f8d1 50d8    	ldr.w	r5, [r1, #0xd8]
 80076ce: eba5 0404    	sub.w	r4, r5, r4
 80076d2: 625c         	str	r4, [r3, #0x24]
 80076d4: f8d1 60f0    	ldr.w	r6, [r1, #0xf0]
 80076d8: eba6 0405    	sub.w	r4, r6, r5
 80076dc: 629c         	str	r4, [r3, #0x28]
 80076de: f8d1 4108    	ldr.w	r4, [r1, #0x108]
 80076e2: f501 7184    	add.w	r1, r1, #0x108
 80076e6: eba4 0506    	sub.w	r5, r4, r6
 80076ea: f843 5f2c    	str	r5, [r3, #44]!
 80076ee: d1c8         	bne	0x8007682 <raylar_crcl_encoder_service::benchmark::temporal::h98ba012ed5b5e4c9+0xaa> @ imm = #-0x70
 80076f0: f500 31a0    	add.w	r1, r0, #0x14000
 80076f4: f240 32ff    	movw	r2, #0x3ff
 80076f8: f101 031c    	add.w	r3, r1, #0x1c
 80076fc: 4671         	mov	r1, lr
 80076fe: f851 4f08    	ldr	r4, [r1, #8]!
 8007702: 698d         	ldr	r5, [r1, #0x18]
 8007704: 3a0b         	subs	r2, #0xb
 8007706: eba5 0404    	sub.w	r4, r5, r4
 800770a: 605c         	str	r4, [r3, #0x4]
 800770c: 6b0c         	ldr	r4, [r1, #0x30]
 800770e: eba4 0505    	sub.w	r5, r4, r5
 8007712: 609d         	str	r5, [r3, #0x8]
 8007714: 6c8d         	ldr	r5, [r1, #0x48]
 8007716: eba5 0404    	sub.w	r4, r5, r4
 800771a: 60dc         	str	r4, [r3, #0xc]
 800771c: 6e0c         	ldr	r4, [r1, #0x60]
 800771e: eba4 0505    	sub.w	r5, r4, r5
 8007722: 611d         	str	r5, [r3, #0x10]
 8007724: 6f8d         	ldr	r5, [r1, #0x78]
 8007726: eba5 0404    	sub.w	r4, r5, r4
 800772a: 615c         	str	r4, [r3, #0x14]
 800772c: f8d1 4090    	ldr.w	r4, [r1, #0x90]
 8007730: eba4 0505    	sub.w	r5, r4, r5
 8007734: 619d         	str	r5, [r3, #0x18]
 8007736: f8d1 50a8    	ldr.w	r5, [r1, #0xa8]
 800773a: eba5 0404    	sub.w	r4, r5, r4
 800773e: 61dc         	str	r4, [r3, #0x1c]
 8007740: f8d1 40c0    	ldr.w	r4, [r1, #0xc0]
 8007744: eba4 0505    	sub.w	r5, r4, r5
 8007748: 621d         	str	r5, [r3, #0x20]
 800774a: f8d1 50d8    	ldr.w	r5, [r1, #0xd8]
 800774e: eba5 0404    	sub.w	r4, r5, r4
 8007752: 625c         	str	r4, [r3, #0x24]
 8007754: f8d1 60f0    	ldr.w	r6, [r1, #0xf0]
 8007758: eba6 0405    	sub.w	r4, r6, r5
 800775c: 629c         	str	r4, [r3, #0x28]
 800775e: f8d1 4108    	ldr.w	r4, [r1, #0x108]
 8007762: f501 7184    	add.w	r1, r1, #0x108
 8007766: eba4 0506    	sub.w	r5, r4, r6
 800776a: f843 5f2c    	str	r5, [r3, #44]!
 800776e: d1c8         	bne	0x8007702 <raylar_crcl_encoder_service::benchmark::temporal::h98ba012ed5b5e4c9+0x12a> @ imm = #-0x70
 8007770: f500 31a8    	add.w	r1, r0, #0x15000
 8007774: f240 32ff    	movw	r2, #0x3ff
 8007778: f101 0318    	add.w	r3, r1, #0x18
 800777c: 4671         	mov	r1, lr
 800777e: f851 4f0c    	ldr	r4, [r1, #12]!
 8007782: 698d         	ldr	r5, [r1, #0x18]
 8007784: 3a0b         	subs	r2, #0xb
 8007786: eba5 0404    	sub.w	r4, r5, r4
 800778a: 605c         	str	r4, [r3, #0x4]
 800778c: 6b0c         	ldr	r4, [r1, #0x30]
 800778e: eba4 0505    	sub.w	r5, r4, r5
 8007792: 609d         	str	r5, [r3, #0x8]
 8007794: 6c8d         	ldr	r5, [r1, #0x48]
 8007796: eba5 0404    	sub.w	r4, r5, r4
 800779a: 60dc         	str	r4, [r3, #0xc]
 800779c: 6e0c         	ldr	r4, [r1, #0x60]
 800779e: eba4 0505    	sub.w	r5, r4, r5
 80077a2: 611d         	str	r5, [r3, #0x10]
 80077a4: 6f8d         	ldr	r5, [r1, #0x78]
 80077a6: eba5 0404    	sub.w	r4, r5, r4
 80077aa: 615c         	str	r4, [r3, #0x14]
 80077ac: f8d1 4090    	ldr.w	r4, [r1, #0x90]
 80077b0: eba4 0505    	sub.w	r5, r4, r5
 80077b4: 619d         	str	r5, [r3, #0x18]
 80077b6: f8d1 50a8    	ldr.w	r5, [r1, #0xa8]
 80077ba: eba5 0404    	sub.w	r4, r5, r4
 80077be: 61dc         	str	r4, [r3, #0x1c]
 80077c0: f8d1 40c0    	ldr.w	r4, [r1, #0xc0]
 80077c4: eba4 0505    	sub.w	r5, r4, r5
 80077c8: 621d         	str	r5, [r3, #0x20]
 80077ca: f8d1 50d8    	ldr.w	r5, [r1, #0xd8]
 80077ce: eba5 0404    	sub.w	r4, r5, r4
 80077d2: 625c         	str	r4, [r3, #0x24]
 80077d4: f8d1 60f0    	ldr.w	r6, [r1, #0xf0]
 80077d8: eba6 0405    	sub.w	r4, r6, r5
 80077dc: 629c         	str	r4, [r3, #0x28]
 80077de: f8d1 4108    	ldr.w	r4, [r1, #0x108]
 80077e2: f501 7184    	add.w	r1, r1, #0x108
 80077e6: eba4 0506    	sub.w	r5, r4, r6
 80077ea: f843 5f2c    	str	r5, [r3, #44]!
 80077ee: d1c8         	bne	0x8007782 <raylar_crcl_encoder_service::benchmark::temporal::h98ba012ed5b5e4c9+0x1aa> @ imm = #-0x70
 80077f0: f500 30b0    	add.w	r0, r0, #0x16000
 80077f4: f85e 2f10    	ldr	r2, [lr, #16]!
 80077f8: 3014         	adds	r0, #0x14
 80077fa: f240 31ff    	movw	r1, #0x3ff
 80077fe: f8de 3018    	ldr.w	r3, [lr, #0x18]
 8007802: 390b         	subs	r1, #0xb
 8007804: eba3 0202    	sub.w	r2, r3, r2
 8007808: 6042         	str	r2, [r0, #0x4]
 800780a: f8de 2030    	ldr.w	r2, [lr, #0x30]
 800780e: eba2 0303    	sub.w	r3, r2, r3
 8007812: 6083         	str	r3, [r0, #0x8]
 8007814: f8de 3048    	ldr.w	r3, [lr, #0x48]
 8007818: eba3 0202    	sub.w	r2, r3, r2
 800781c: 60c2         	str	r2, [r0, #0xc]
 800781e: f8de 2060    	ldr.w	r2, [lr, #0x60]
 8007822: eba2 0303    	sub.w	r3, r2, r3
 8007826: 6103         	str	r3, [r0, #0x10]
 8007828: f8de 3078    	ldr.w	r3, [lr, #0x78]
 800782c: eba3 0202    	sub.w	r2, r3, r2
 8007830: 6142         	str	r2, [r0, #0x14]
 8007832: f8de 2090    	ldr.w	r2, [lr, #0x90]
 8007836: eba2 0303    	sub.w	r3, r2, r3
 800783a: 6183         	str	r3, [r0, #0x18]
 800783c: f8de 30a8    	ldr.w	r3, [lr, #0xa8]
 8007840: eba3 0202    	sub.w	r2, r3, r2
 8007844: 61c2         	str	r2, [r0, #0x1c]
 8007846: f8de 20c0    	ldr.w	r2, [lr, #0xc0]
 800784a: eba2 0303    	sub.w	r3, r2, r3
 800784e: 6203         	str	r3, [r0, #0x20]
 8007850: f8de 30d8    	ldr.w	r3, [lr, #0xd8]
 8007854: eba3 0202    	sub.w	r2, r3, r2
 8007858: 6242         	str	r2, [r0, #0x24]
 800785a: f8de 40f0    	ldr.w	r4, [lr, #0xf0]
 800785e: eba4 0203    	sub.w	r2, r4, r3
 8007862: 6282         	str	r2, [r0, #0x28]
 8007864: f8de 2108    	ldr.w	r2, [lr, #0x108]
 8007868: f50e 7e84    	add.w	lr, lr, #0x108
 800786c: eba2 0304    	sub.w	r3, r2, r4
 8007870: f840 3f2c    	str	r3, [r0, #44]!
 8007874: d1c3         	bne	0x80077fe <raylar_crcl_encoder_service::benchmark::temporal::h98ba012ed5b5e4c9+0x226> @ imm = #-0x7a
 8007876: f8cd c000    	str.w	r12, [sp]
 800787a: 4668         	mov	r0, sp
 800787c: f8dc 0000    	ldr.w	r0, [r12]
 8007880: b001         	add	sp, #0x4
 8007882: f85d bb04    	ldr	r11, [sp], #4
 8007886: bdf0         	pop	{r4, r5, r6, r7, pc}

08007888 <raylar_crcl_encoder_service::benchmark::pack_copy::ha831ca28ff45cfb7>:
 8007888: b5b0         	push	{r4, r5, r7, lr}
 800788a: af02         	add	r7, sp, #0x8
 800788c: b082         	sub	sp, #0x8
 800788e: 4604         	mov	r4, r0
 8007890: f64f 40d8    	movw	r0, #0xfcd8
 8007894: f2c0 0001    	movt	r0, #0x1
 8007898: 5821         	ldr	r1, [r4, r0]
 800789a: f1a1 0258    	sub.w	r2, r1, #0x58
 800789e: f5b2 5ff0    	cmp.w	r2, #0x1e00
 80078a2: d819         	bhi	0x80078d8 <raylar_crcl_encoder_service::benchmark::pack_copy::ha831ca28ff45cfb7+0x50> @ imm = #0x32
 80078a4: f504 31e8    	add.w	r1, r4, #0x1d000
 80078a8: 1825         	adds	r5, r4, r0
 80078aa: f504 30e0    	add.w	r0, r4, #0x1c000
 80078ae: f501 616b    	add.w	r1, r1, #0xeb0
 80078b2: 3058         	adds	r0, #0x58
 80078b4: f005 f8cd    	bl	0x800ca52 <__aeabi_memcpy4> @ imm = #0x519a
 80078b8: 4620         	mov	r0, r4
 80078ba: f7fa ffea    	bl	0x8002892 <raylar_crcl_encoder_service::benchmark::pack_header::haa3cc0f32481b1e3> @ imm = #-0x502c
 80078be: 6829         	ldr	r1, [r5]
 80078c0: f641 6059    	movw	r0, #0x1e59
 80078c4: 4281         	cmp	r1, r0
 80078c6: d210         	bhs	0x80078ea <raylar_crcl_encoder_service::benchmark::pack_copy::ha831ca28ff45cfb7+0x62> @ imm = #0x20
 80078c8: f504 30e0    	add.w	r0, r4, #0x1c000
 80078cc: 9000         	str	r0, [sp]
 80078ce: 9101         	str	r1, [sp, #0x4]
 80078d0: 4668         	mov	r0, sp
 80078d2: 6828         	ldr	r0, [r5]
 80078d4: b002         	add	sp, #0x8
 80078d6: bdb0         	pop	{r4, r5, r7, pc}
 80078d8: f64d 7330    	movw	r3, #0xdf30
 80078dc: 2058         	movs	r0, #0x58
 80078de: f6c0 0300    	movt	r3, #0x800
 80078e2: f641 6258    	movw	r2, #0x1e58
 80078e6: f004 f9a8    	bl	0x800bc3a <core::slice::index::slice_index_fail> @ imm = #0x4350
 80078ea: f64d 7320    	movw	r3, #0xdf20
 80078ee: 2000         	movs	r0, #0x0
 80078f0: f6c0 0300    	movt	r3, #0x800
 80078f4: f641 6258    	movw	r2, #0x1e58
 80078f8: f004 f99f    	bl	0x800bc3a <core::slice::index::slice_index_fail> @ imm = #0x433e