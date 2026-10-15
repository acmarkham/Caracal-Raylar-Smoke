08000c36 <raylar_crcl_encoder_service::benchmark::nothing::h153434569c2cec30>:
 8000c36: b580         	push	{r7, lr}
 8000c38: 466f         	mov	r7, sp
 8000c3a: bd80         	pop	{r7, pc}

080010c2 <raylar_crcl_encoder_service::bits::Bits::unary::he0d732a6251feea8>:
 80010c2: b5f0         	push	{r4, r5, r6, r7, lr}
 80010c4: af03         	add	r7, sp, #0xc
 80010c6: e92d 0700    	push.w	{r8, r9, r10}
 80010ca: 468e         	mov	lr, r1
 80010cc: 6841         	ldr	r1, [r0, #0x4]
 80010ce: f1b1 5f00    	cmp.w	r1, #0x20000000
 80010d2: ea4f 0cc1    	lsl.w	r12, r1, #0x3
 80010d6: 4663         	mov	r3, r12
 80010d8: bf28         	it	hs
 80010da: f04f 33ff    	movhs.w	r3, #0xffffffff
 80010de: 6882         	ldr	r2, [r0, #0x8]
 80010e0: 1a9b         	subs	r3, r3, r2
 80010e2: 459e         	cmp	lr, r3
 80010e4: f080 808b    	bhs.w	0x80011fe <raylar_crcl_encoder_service::bits::Bits::unary::he0d732a6251feea8+0x13c> @ imm = #0x116
 80010e8: f1be 0f00    	cmp.w	lr, #0x0
 80010ec: d02d         	beq	0x800114a <raylar_crcl_encoder_service::bits::Bits::unary::he0d732a6251feea8+0x88> @ imm = #0x5a
 80010ee: f8d0 a000    	ldr.w	r10, [r0]
 80010f2: f04f 0800    	mov.w	r8, #0x0
 80010f6: f1b1 5f00    	cmp.w	r1, #0x20000000
 80010fa: d237         	bhs	0x800116c <raylar_crcl_encoder_service::bits::Bits::unary::he0d732a6251feea8+0xaa> @ imm = #0x6e
 80010fc: f1be 0f20    	cmp.w	lr, #0x20
 8001100: 46f1         	mov	r9, lr
 8001102: bf28         	it	hs
 8001104: f04f 0920    	movhs.w	r9, #0x20
 8001108: eb12 0409    	adds.w	r4, r2, r9
 800110c: f04f 0302    	mov.w	r3, #0x2
 8001110: d224         	bhs	0x800115c <raylar_crcl_encoder_service::bits::Bits::unary::he0d732a6251feea8+0x9a> @ imm = #0x48
 8001112: 4564         	cmp	r4, r12
 8001114: d822         	bhi	0x800115c <raylar_crcl_encoder_service::bits::Bits::unary::he0d732a6251feea8+0x9a> @ imm = #0x44
 8001116: 464e         	mov	r6, r9
 8001118: f012 0507    	ands	r5, r2, #0x7
 800111c: ea4f 03d2    	lsr.w	r3, r2, #0x3
 8001120: d104         	bne	0x800112c <raylar_crcl_encoder_service::bits::Bits::unary::he0d732a6251feea8+0x6a> @ imm = #0x8
 8001122: 428b         	cmp	r3, r1
 8001124: f080 8090    	bhs.w	0x8001248 <raylar_crcl_encoder_service::bits::Bits::unary::he0d732a6251feea8+0x186> @ imm = #0x120
 8001128: f80a 8003    	strb.w	r8, [r10, r3]
 800112c: 428b         	cmp	r3, r1
 800112e: f080 8084    	bhs.w	0x800123a <raylar_crcl_encoder_service::bits::Bits::unary::he0d732a6251feea8+0x178> @ imm = #0x108
 8001132: f1c5 0308    	rsb.w	r3, r5, #0x8
 8001136: 42b3         	cmp	r3, r6
 8001138: bf28         	it	hs
 800113a: 4633         	movhs	r3, r6
 800113c: 441a         	add	r2, r3
 800113e: 1af6         	subs	r6, r6, r3
 8001140: 6082         	str	r2, [r0, #0x8]
 8001142: d1e9         	bne	0x8001118 <raylar_crcl_encoder_service::bits::Bits::unary::he0d732a6251feea8+0x56> @ imm = #-0x2e
 8001144: ebbe 0e09    	subs.w	lr, lr, r9
 8001148: d1d8         	bne	0x80010fc <raylar_crcl_encoder_service::bits::Bits::unary::he0d732a6251feea8+0x3a> @ imm = #-0x50
 800114a: 1c56         	adds	r6, r2, #0x1
 800114c: d057         	beq	0x80011fe <raylar_crcl_encoder_service::bits::Bits::unary::he0d732a6251feea8+0x13c> @ imm = #0xae
 800114e: 2300         	movs	r3, #0x0
 8001150: ebb3 7f51    	cmp.w	r3, r1, lsr #29
 8001154: d158         	bne	0x8001208 <raylar_crcl_encoder_service::bits::Bits::unary::he0d732a6251feea8+0x146> @ imm = #0xb0
 8001156: 4562         	cmp	r2, r12
 8001158: d356         	blo	0x8001208 <raylar_crcl_encoder_service::bits::Bits::unary::he0d732a6251feea8+0x146> @ imm = #0xac
 800115a: 2302         	movs	r3, #0x2
 800115c: 4618         	mov	r0, r3
 800115e: e8bd 0700    	pop.w	{r8, r9, r10}
 8001162: bdf0         	pop	{r4, r5, r6, r7, pc}
 8001164: ebbe 0e09    	subs.w	lr, lr, r9
 8001168: 6082         	str	r2, [r0, #0x8]
 800116a: d0ee         	beq	0x800114a <raylar_crcl_encoder_service::bits::Bits::unary::he0d732a6251feea8+0x88> @ imm = #-0x24
 800116c: f1be 0f20    	cmp.w	lr, #0x20
 8001170: 46f1         	mov	r9, lr
 8001172: bf28         	it	hs
 8001174: f04f 0920    	movhs.w	r9, #0x20
 8001178: eb12 0409    	adds.w	r4, r2, r9
 800117c: d23f         	bhs	0x80011fe <raylar_crcl_encoder_service::bits::Bits::unary::he0d732a6251feea8+0x13c> @ imm = #0x7e
 800117e: 464d         	mov	r5, r9
 8001180: f012 0407    	ands	r4, r2, #0x7
 8001184: bf04         	itt	eq
 8001186: 08d6         	lsreq	r6, r2, #0x3
 8001188: f80a 8006    	strbeq.w	r8, [r10, r6]
 800118c: f1c4 0408    	rsb.w	r4, r4, #0x8
 8001190: 462e         	mov	r6, r5
 8001192: 42ac         	cmp	r4, r5
 8001194: bf38         	it	lo
 8001196: 4626         	movlo	r6, r4
 8001198: 4432         	add	r2, r6
 800119a: 42a5         	cmp	r5, r4
 800119c: d9e2         	bls	0x8001164 <raylar_crcl_encoder_service::bits::Bits::unary::he0d732a6251feea8+0xa2> @ imm = #-0x3c
 800119e: 1bae         	subs	r6, r5, r6
 80011a0: f012 0407    	ands	r4, r2, #0x7
 80011a4: bf04         	itt	eq
 80011a6: 08d3         	lsreq	r3, r2, #0x3
 80011a8: f80a 8003    	strbeq.w	r8, [r10, r3]
 80011ac: f1c4 0308    	rsb.w	r3, r4, #0x8
 80011b0: 4635         	mov	r5, r6
 80011b2: 42b3         	cmp	r3, r6
 80011b4: bf38         	it	lo
 80011b6: 461d         	movlo	r5, r3
 80011b8: 442a         	add	r2, r5
 80011ba: 429e         	cmp	r6, r3
 80011bc: d9d2         	bls	0x8001164 <raylar_crcl_encoder_service::bits::Bits::unary::he0d732a6251feea8+0xa2> @ imm = #-0x5c
 80011be: 1b76         	subs	r6, r6, r5
 80011c0: f012 0407    	ands	r4, r2, #0x7
 80011c4: bf04         	itt	eq
 80011c6: 08d3         	lsreq	r3, r2, #0x3
 80011c8: f80a 8003    	strbeq.w	r8, [r10, r3]
 80011cc: f1c4 0308    	rsb.w	r3, r4, #0x8
 80011d0: 4635         	mov	r5, r6
 80011d2: 42b3         	cmp	r3, r6
 80011d4: bf38         	it	lo
 80011d6: 461d         	movlo	r5, r3
 80011d8: 442a         	add	r2, r5
 80011da: 429e         	cmp	r6, r3
 80011dc: d9c2         	bls	0x8001164 <raylar_crcl_encoder_service::bits::Bits::unary::he0d732a6251feea8+0xa2> @ imm = #-0x7c
 80011de: f012 0407    	ands	r4, r2, #0x7
 80011e2: bf04         	itt	eq
 80011e4: 08d3         	lsreq	r3, r2, #0x3
 80011e6: f80a 8003    	strbeq.w	r8, [r10, r3]
 80011ea: 1b73         	subs	r3, r6, r5
 80011ec: f1c4 0408    	rsb.w	r4, r4, #0x8
 80011f0: 429c         	cmp	r4, r3
 80011f2: bf28         	it	hs
 80011f4: 461c         	movhs	r4, r3
 80011f6: 4422         	add	r2, r4
 80011f8: 1b1d         	subs	r5, r3, r4
 80011fa: d1c1         	bne	0x8001180 <raylar_crcl_encoder_service::bits::Bits::unary::he0d732a6251feea8+0xbe> @ imm = #-0x7e
 80011fc: e7b2         	b	0x8001164 <raylar_crcl_encoder_service::bits::Bits::unary::he0d732a6251feea8+0xa2> @ imm = #-0x9c
 80011fe: 2302         	movs	r3, #0x2
 8001200: 4618         	mov	r0, r3
 8001202: e8bd 0700    	pop.w	{r8, r9, r10}
 8001206: bdf0         	pop	{r4, r5, r6, r7, pc}
 8001208: 6805         	ldr	r5, [r0]
 800120a: 08d3         	lsrs	r3, r2, #0x3
 800120c: f012 0207    	ands	r2, r2, #0x7
 8001210: d103         	bne	0x800121a <raylar_crcl_encoder_service::bits::Bits::unary::he0d732a6251feea8+0x158> @ imm = #0x6
 8001212: 428b         	cmp	r3, r1
 8001214: d218         	bhs	0x8001248 <raylar_crcl_encoder_service::bits::Bits::unary::he0d732a6251feea8+0x186> @ imm = #0x30
 8001216: 2400         	movs	r4, #0x0
 8001218: 54ec         	strb	r4, [r5, r3]
 800121a: 428b         	cmp	r3, r1
 800121c: d20d         	bhs	0x800123a <raylar_crcl_encoder_service::bits::Bits::unary::he0d732a6251feea8+0x178> @ imm = #0x1a
 800121e: 5ce9         	ldrb	r1, [r5, r3]
 8001220: f082 0207    	eor	r2, r2, #0x7
 8001224: 2401         	movs	r4, #0x1
 8001226: fa04 f202    	lsl.w	r2, r4, r2
 800122a: 6086         	str	r6, [r0, #0x8]
 800122c: 4311         	orrs	r1, r2
 800122e: 54e9         	strb	r1, [r5, r3]
 8001230: 2306         	movs	r3, #0x6
 8001232: 4618         	mov	r0, r3
 8001234: e8bd 0700    	pop.w	{r8, r9, r10}
 8001238: bdf0         	pop	{r4, r5, r6, r7, pc}
 800123a: f64d 0278    	movw	r2, #0xd878
 800123e: 4618         	mov	r0, r3
 8001240: f6c0 0200    	movt	r2, #0x800
 8001244: f00a fb06    	bl	0x800b854 <core::panicking::panic_bounds_check> @ imm = #0xa60c
 8001248: f64d 0268    	movw	r2, #0xd868
 800124c: 4618         	mov	r0, r3
 800124e: f6c0 0200    	movt	r2, #0x800
 8001252: f00a faff    	bl	0x800b854 <core::panicking::panic_bounds_check> @ imm = #0xa5fe

08001484 <raylar_crcl_encoder_service::format::Encoder::data::ha65fe528befbf4e3>:
 8001484: b5f0         	push	{r4, r5, r6, r7, lr}
 8001486: af03         	add	r7, sp, #0xc
 8001488: e92d 0f00    	push.w	{r8, r9, r10, r11}
 800148c: b0a1         	sub	sp, #0x84
 800148e: 6f16         	ldr	r6, [r2, #0x70]
 8001490: 2e06         	cmp	r6, #0x6
 8001492: d138         	bne	0x8001506 <raylar_crcl_encoder_service::format::Encoder::data::ha65fe528befbf4e3+0x82> @ imm = #0x70
 8001494: f892 408c    	ldrb.w	r4, [r2, #0x8c]
 8001498: 7e0e         	ldrb	r6, [r1, #0x18]
 800149a: 42b4         	cmp	r4, r6
 800149c: d133         	bne	0x8001506 <raylar_crcl_encoder_service::format::Encoder::data::ha65fe528befbf4e3+0x82> @ imm = #0x66
 800149e: 7e4e         	ldrb	r6, [r1, #0x19]
 80014a0: f892 508d    	ldrb.w	r5, [r2, #0x8d]
 80014a4: 42b5         	cmp	r5, r6
 80014a6: d12e         	bne	0x8001506 <raylar_crcl_encoder_service::format::Encoder::data::ha65fe528befbf4e3+0x82> @ imm = #0x5c
 80014a8: 694e         	ldr	r6, [r1, #0x14]
 80014aa: f8d2 5088    	ldr.w	r5, [r2, #0x88]
 80014ae: 42b5         	cmp	r5, r6
 80014b0: d129         	bne	0x8001506 <raylar_crcl_encoder_service::format::Encoder::data::ha65fe528befbf4e3+0x82> @ imm = #0x52
 80014b2: f892 b08f    	ldrb.w	r11, [r2, #0x8f]
 80014b6: 7ece         	ldrb	r6, [r1, #0x1b]
 80014b8: 45b3         	cmp	r11, r6
 80014ba: d124         	bne	0x8001506 <raylar_crcl_encoder_service::format::Encoder::data::ha65fe528befbf4e3+0x82> @ imm = #0x48
 80014bc: 690e         	ldr	r6, [r1, #0x10]
 80014be: f8d2 5084    	ldr.w	r5, [r2, #0x84]
 80014c2: 42b5         	cmp	r5, r6
 80014c4: d11f         	bne	0x8001506 <raylar_crcl_encoder_service::format::Encoder::data::ha65fe528befbf4e3+0x82> @ imm = #0x3e
 80014c6: 7e8e         	ldrb	r6, [r1, #0x1a]
 80014c8: f892 508e    	ldrb.w	r5, [r2, #0x8e]
 80014cc: 42b5         	cmp	r5, r6
 80014ce: d11a         	bne	0x8001506 <raylar_crcl_encoder_service::format::Encoder::data::ha65fe528befbf4e3+0x82> @ imm = #0x34
 80014d0: 6c56         	ldr	r6, [r2, #0x44]
 80014d2: f64a 29ab    	movw	r9, #0xaaab
 80014d6: f6ca 29aa    	movt	r9, #0xaaaa
 80014da: 961b         	str	r6, [sp, #0x6c]
 80014dc: f8d7 8008    	ldr.w	r8, [r7, #0x8]
 80014e0: f1bb 0f00    	cmp.w	r11, #0x0
 80014e4: fba6 6509    	umull	r6, r5, r6, r9
 80014e8: ea4f 0595    	lsr.w	r5, r5, #0x2
 80014ec: 951a         	str	r5, [sp, #0x68]
 80014ee: d011         	beq	0x8001514 <raylar_crcl_encoder_service::format::Encoder::data::ha65fe528befbf4e3+0x90> @ imm = #0x22
 80014f0: 462e         	mov	r6, r5
 80014f2: 2d00         	cmp	r5, #0x0
 80014f4: bf18         	it	ne
 80014f6: 2601         	movne	r6, #0x1
 80014f8: f04f 0e58    	mov.w	lr, #0x58
 80014fc: 1bae         	subs	r6, r5, r6
 80014fe: 1ca5         	adds	r5, r4, #0x2
 8001500: fb05 4a06    	mla	r10, r5, r6, r4
 8001504: e00a         	b	0x800151c <raylar_crcl_encoder_service::format::Encoder::data::ha65fe528befbf4e3+0x98> @ imm = #0x14
 8001506: f240 5101    	movw	r1, #0x501
 800150a: 8001         	strh	r1, [r0]
 800150c: b021         	add	sp, #0x84
 800150e: e8bd 0f00    	pop.w	{r8, r9, r10, r11}
 8001512: bdf0         	pop	{r4, r5, r6, r7, pc}
 8001514: fb05 fa04    	mul	r10, r5, r4
 8001518: f04f 0e60    	mov.w	lr, #0x60
 800151c: ea4f 06da    	lsr.w	r6, r10, #0x3
 8001520: f04f 0c01    	mov.w	r12, #0x1
 8001524: ea5f 754a    	lsls.w	r5, r10, #0x1d
 8001528: bf18         	it	ne
 800152a: eb0c 06da    	addne.w	r6, r12, r10, lsr #3
 800152e: eb06 0686    	add.w	r6, r6, r6, lsl #2
 8001532: 4476         	add	r6, lr
 8001534: 45b0         	cmp	r8, r6
 8001536: d206         	bhs	0x8001546 <raylar_crcl_encoder_service::format::Encoder::data::ha65fe528befbf4e3+0xc2> @ imm = #0xc
 8001538: f240 2101    	movw	r1, #0x201
 800153c: 8001         	strh	r1, [r0]
 800153e: b021         	add	sp, #0x84
 8001540: e8bd 0f00    	pop.w	{r8, r9, r10, r11}
 8001544: bdf0         	pop	{r4, r5, r6, r7, pc}
 8001546: 688e         	ldr	r6, [r1, #0x8]
 8001548: 9610         	str	r6, [sp, #0x40]
 800154a: 3601         	adds	r6, #0x1
 800154c: bf1c         	itt	ne
 800154e: 68ce         	ldrne	r6, [r1, #0xc]
 8001550: f116 0601    	addsne.w	r6, r6, #0x1
 8001554: d106         	bne	0x8001564 <raylar_crcl_encoder_service::format::Encoder::data::ha65fe528befbf4e3+0xe0> @ imm = #0xc
 8001556: f240 4101    	movw	r1, #0x401
 800155a: 8001         	strh	r1, [r0]
 800155c: b021         	add	sp, #0x84
 800155e: e8bd 0f00    	pop.w	{r8, r9, r10, r11}
 8001562: bdf0         	pop	{r4, r5, r6, r7, pc}
 8001564: e9d1 c500    	ldrd	r12, r5, [r1]
 8001568: 2100         	movs	r1, #0x0
 800156a: 9e1a         	ldr	r6, [sp, #0x68]
 800156c: eb16 060c    	adds.w	r6, r6, r12
 8001570: f155 0600    	adcs	r6, r5, #0x0
 8001574: f151 0100    	adcs	r1, r1, #0x0
 8001578: d1ed         	bne	0x8001556 <raylar_crcl_encoder_service::format::Encoder::data::ha65fe528befbf4e3+0xd2> @ imm = #-0x26
 800157a: 215c         	movs	r1, #0x5c
 800157c: f1bb 0f00    	cmp.w	r11, #0x0
 8001580: e9cd c507    	strd	r12, r5, [sp, #28]
 8001584: 9012         	str	r0, [sp, #0x48]
 8001586: bf18         	it	ne
 8001588: 2158         	movne	r1, #0x58
 800158a: 4541         	cmp	r1, r8
 800158c: f200 81f0    	bhi.w	0x8001970 <raylar_crcl_encoder_service::format::Encoder::data::ha65fe528befbf4e3+0x4ec> @ imm = #0x3e0
 8001590: 6c10         	ldr	r0, [r2, #0x40]
 8001592: 460d         	mov	r5, r1
 8001594: 9017         	str	r0, [sp, #0x5c]
 8001596: 4618         	mov	r0, r3
 8001598: 930f         	str	r3, [sp, #0x3c]
 800159a: 9216         	str	r2, [sp, #0x58]
 800159c: f00b faeb    	bl	0x800cb76 <__aeabi_memclr> @ imm = #0xb5d6
 80015a0: f1a8 001c    	sub.w	r0, r8, #0x1c
 80015a4: 9a0f         	ldr	r2, [sp, #0x3c]
 80015a6: f04f 0e00    	mov.w	lr, #0x0
 80015aa: fba0 0109    	umull	r0, r1, r0, r9
 80015ae: f1a8 0018    	sub.w	r0, r8, #0x18
 80015b2: fba0 0309    	umull	r0, r3, r0, r9
 80015b6: 08c8         	lsrs	r0, r1, #0x3
 80015b8: 2148         	movs	r1, #0x48
 80015ba: 900e         	str	r0, [sp, #0x38]
 80015bc: 9816         	ldr	r0, [sp, #0x58]
 80015be: ea4f 09d3    	lsr.w	r9, r3, #0x3
 80015c2: f100 0348    	add.w	r3, r0, #0x48
 80015c6: 201c         	movs	r0, #0x1c
 80015c8: e9cd 9b0c    	strd	r9, r11, [sp, #48]
 80015cc: eb0e 064e    	add.w	r6, lr, lr, lsl #1
 80015d0: 9115         	str	r1, [sp, #0x54]
 80015d2: eb00 0186    	add.w	r1, r0, r6, lsl #2
 80015d6: 4541         	cmp	r1, r8
 80015d8: f080 8202    	bhs.w	0x80019e0 <raylar_crcl_encoder_service::format::Encoder::data::ha65fe528befbf4e3+0x55c> @ imm = #0x404
 80015dc: f893 a005    	ldrb.w	r10, [r3, #0x5]
 80015e0: ea4f 0c86    	lsl.w	r12, r6, #0x2
 80015e4: f10e 0602    	add.w	r6, lr, #0x2
 80015e8: 5456         	strb	r6, [r2, r1]
 80015ea: f10c 011d    	add.w	r1, r12, #0x1d
 80015ee: 4541         	cmp	r1, r8
 80015f0: f8cd e064    	str.w	lr, [sp, #0x64]
 80015f4: f080 81fc    	bhs.w	0x80019f0 <raylar_crcl_encoder_service::format::Encoder::data::ha65fe528befbf4e3+0x56c> @ imm = #0x3f8
 80015f8: f802 a001    	strb.w	r10, [r2, r1]
 80015fc: f10c 011f    	add.w	r1, r12, #0x1f
 8001600: 4541         	cmp	r1, r8
 8001602: f080 81fd    	bhs.w	0x8001a00 <raylar_crcl_encoder_service::format::Encoder::data::ha65fe528befbf4e3+0x57c> @ imm = #0x3fa
 8001606: 7918         	ldrb	r0, [r3, #0x4]
 8001608: f10c 0620    	add.w	r6, r12, #0x20
 800160c: 5450         	strb	r0, [r2, r1]
 800160e: 9919         	ldr	r1, [sp, #0x64]
 8001610: f8cd c050    	str.w	r12, [sp, #0x50]
 8001614: 4549         	cmp	r1, r9
 8001616: f000 819e    	beq.w	0x8001956 <raylar_crcl_encoder_service::format::Encoder::data::ha65fe528befbf4e3+0x4d2> @ imm = #0x33c
 800161a: 6819         	ldr	r1, [r3]
 800161c: 45a8         	cmp	r8, r5
 800161e: 5191         	str	r1, [r2, r6]
 8001620: f0c0 81ae    	blo.w	0x8001980 <raylar_crcl_encoder_service::format::Encoder::data::ha65fe528befbf4e3+0x4fc> @ imm = #0x35c
 8001624: 9e19         	ldr	r6, [sp, #0x64]
 8001626: eba8 0105    	sub.w	r1, r8, r5
 800162a: f04f 0b00    	mov.w	r11, #0x0
 800162e: 911f         	str	r1, [sp, #0x7c]
 8001630: 1c73         	adds	r3, r6, #0x1
 8001632: 9318         	str	r3, [sp, #0x60]
 8001634: 1953         	adds	r3, r2, r5
 8001636: 931e         	str	r3, [sp, #0x78]
 8001638: 9b1b         	ldr	r3, [sp, #0x6c]
 800163a: f8cd b080    	str.w	r11, [sp, #0x80]
 800163e: 2b06         	cmp	r3, #0x6
 8001640: 9513         	str	r5, [sp, #0x4c]
 8001642: d21c         	bhs	0x800167e <raylar_crcl_encoder_service::format::Encoder::data::ha65fe528befbf4e3+0x1fa> @ imm = #0x38
 8001644: 9b14         	ldr	r3, [sp, #0x50]
 8001646: 9d15         	ldr	r5, [sp, #0x54]
 8001648: 990e         	ldr	r1, [sp, #0x38]
 800164a: f103 0c24    	add.w	r12, r3, #0x24
 800164e: 428e         	cmp	r6, r1
 8001650: f000 819f    	beq.w	0x8001992 <raylar_crcl_encoder_service::format::Encoder::data::ha65fe528befbf4e3+0x50e> @ imm = #0x33e
 8001654: 3508         	adds	r5, #0x8
 8001656: 9816         	ldr	r0, [sp, #0x58]
 8001658: ea4f 01db    	lsr.w	r1, r11, #0x3
 800165c: f8dd e060    	ldr.w	lr, [sp, #0x60]
 8001660: f842 100c    	str.w	r1, [r2, r12]
 8001664: 1943         	adds	r3, r0, r5
 8001666: 4629         	mov	r1, r5
 8001668: 9d13         	ldr	r5, [sp, #0x4c]
 800166a: 201c         	movs	r0, #0x1c
 800166c: f8d7 8008    	ldr.w	r8, [r7, #0x8]
 8001670: 2970         	cmp	r1, #0x70
 8001672: eb05 05db    	add.w	r5, r5, r11, lsr #3
 8001676: e9dd 9b0c    	ldrd	r9, r11, [sp, #48]
 800167a: d1a7         	bne	0x80015cc <raylar_crcl_encoder_service::format::Encoder::data::ha65fe528befbf4e3+0x148> @ imm = #-0xb2
 800167c: e155         	b	0x800192a <raylar_crcl_encoder_service::format::Encoder::data::ha65fe528befbf4e3+0x4a6> @ imm = #0x2aa
 800167e: f000 081f    	and	r8, r0, #0x1f
 8001682: f04f 32ff    	mov.w	r2, #0xffffffff
 8001686: 900b         	str	r0, [sp, #0x2c]
 8001688: 2600         	movs	r6, #0x0
 800168a: fa02 f208    	lsl.w	r2, r2, r8
 800168e: f8cd a070    	str.w	r10, [sp, #0x70]
 8001692: 43d0         	mvns	r0, r2
 8001694: 9009         	str	r0, [sp, #0x24]
 8001696: 2000         	movs	r0, #0x0
 8001698: e006         	b	0x80016a8 <raylar_crcl_encoder_service::format::Encoder::data::ha65fe528befbf4e3+0x224> @ imm = #0xc
 800169a: f8dd 8028    	ldr.w	r8, [sp, #0x28]
 800169e: 9e1d         	ldr	r6, [sp, #0x74]
 80016a0: 9a1a         	ldr	r2, [sp, #0x68]
 80016a2: 4290         	cmp	r0, r2
 80016a4: f000 810b    	beq.w	0x80018be <raylar_crcl_encoder_service::format::Encoder::data::ha65fe528befbf4e3+0x43a> @ imm = #0x216
 80016a8: eb00 0340    	add.w	r3, r0, r0, lsl #1
 80016ac: 9a18         	ldr	r2, [sp, #0x60]
 80016ae: eb02 0343    	add.w	r3, r2, r3, lsl #1
 80016b2: 9a1b         	ldr	r2, [sp, #0x6c]
 80016b4: 4293         	cmp	r3, r2
 80016b6: f080 818b    	bhs.w	0x80019d0 <raylar_crcl_encoder_service::format::Encoder::data::ha65fe528befbf4e3+0x54c> @ imm = #0x316
 80016ba: 9a17         	ldr	r2, [sp, #0x5c]
 80016bc: 46b4         	mov	r12, r6
 80016be: 4605         	mov	r5, r0
 80016c0: 3001         	adds	r0, #0x1
 80016c2: f852 6023    	ldr.w	r6, [r2, r3, lsl #2]
 80016c6: 4633         	mov	r3, r6
 80016c8: e8df f00a    	tbb	[pc, r10]
 80016cc: 02 08 03 68  	.word	0x68030802
 80016d0: e7e6         	b	0x80016a0 <raylar_crcl_encoder_service::format::Encoder::data::ha65fe528befbf4e3+0x21c> @ imm = #-0x34
 80016d2: 2d00         	cmp	r5, #0x0
 80016d4: f000 80ab    	beq.w	0x800182e <raylar_crcl_encoder_service::format::Encoder::data::ha65fe528befbf4e3+0x3aa> @ imm = #0x156
 80016d8: eba6 030c    	sub.w	r3, r6, r12
 80016dc: 9011         	str	r0, [sp, #0x44]
 80016de: 0058         	lsls	r0, r3, #0x1
 80016e0: ea80 75e3    	eor.w	r5, r0, r3, asr #31
 80016e4: a81e         	add	r0, sp, #0x78
 80016e6: 961d         	str	r6, [sp, #0x74]
 80016e8: fa25 f108    	lsr.w	r1, r5, r8
 80016ec: f8cd 8028    	str.w	r8, [sp, #0x28]
 80016f0: f7ff fce7    	bl	0x80010c2 <raylar_crcl_encoder_service::bits::Bits::unary::he0d732a6251feea8> @ imm = #-0x632
 80016f4: 4601         	mov	r1, r0
 80016f6: b2c0         	uxtb	r0, r0
 80016f8: 2806         	cmp	r0, #0x6
 80016fa: f040 810e    	bne.w	0x800191a <raylar_crcl_encoder_service::format::Encoder::data::ha65fe528befbf4e3+0x496> @ imm = #0x21c
 80016fe: 9e0b         	ldr	r6, [sp, #0x2c]
 8001700: 9812         	ldr	r0, [sp, #0x48]
 8001702: 2e20         	cmp	r6, #0x20
 8001704: f200 8125    	bhi.w	0x8001952 <raylar_crcl_encoder_service::format::Encoder::data::ha65fe528befbf4e3+0x4ce> @ imm = #0x24a
 8001708: f8dd b080    	ldr.w	r11, [sp, #0x80]
 800170c: eb1b 0206    	adds.w	r2, r11, r6
 8001710: f080 8102    	bhs.w	0x8001918 <raylar_crcl_encoder_service::format::Encoder::data::ha65fe528befbf4e3+0x494> @ imm = #0x204
 8001714: 991f         	ldr	r1, [sp, #0x7c]
 8001716: 2300         	movs	r3, #0x0
 8001718: 9811         	ldr	r0, [sp, #0x44]
 800171a: ebb3 7f51    	cmp.w	r3, r1, lsr #29
 800171e: d103         	bne	0x8001728 <raylar_crcl_encoder_service::format::Encoder::data::ha65fe528befbf4e3+0x2a4> @ imm = #0x6
 8001720: 00cb         	lsls	r3, r1, #0x3
 8001722: 429a         	cmp	r2, r3
 8001724: f200 80f8    	bhi.w	0x8001918 <raylar_crcl_encoder_service::format::Encoder::data::ha65fe528befbf4e3+0x494> @ imm = #0x1f0
 8001728: 2e00         	cmp	r6, #0x0
 800172a: d0b6         	beq	0x800169a <raylar_crcl_encoder_service::format::Encoder::data::ha65fe528befbf4e3+0x216> @ imm = #-0x94
 800172c: 9809         	ldr	r0, [sp, #0x24]
 800172e: 4632         	mov	r2, r6
 8001730: f8dd e078    	ldr.w	lr, [sp, #0x78]
 8001734: ea05 0c00    	and.w	r12, r5, r0
 8001738: f01b 0507    	ands	r5, r11, #0x7
 800173c: ea4f 0adb    	lsr.w	r10, r11, #0x3
 8001740: d105         	bne	0x800174e <raylar_crcl_encoder_service::format::Encoder::data::ha65fe528befbf4e3+0x2ca> @ imm = #0xa
 8001742: 458a         	cmp	r10, r1
 8001744: f080 8136    	bhs.w	0x80019b4 <raylar_crcl_encoder_service::format::Encoder::data::ha65fe528befbf4e3+0x530> @ imm = #0x26c
 8001748: 2300         	movs	r3, #0x0
 800174a: f80e 300a    	strb.w	r3, [lr, r10]
 800174e: 458a         	cmp	r10, r1
 8001750: f080 8129    	bhs.w	0x80019a6 <raylar_crcl_encoder_service::format::Encoder::data::ha65fe528befbf4e3+0x522> @ imm = #0x252
 8001754: f1c5 0508    	rsb.w	r5, r5, #0x8
 8001758: 4616         	mov	r6, r2
 800175a: 4295         	cmp	r5, r2
 800175c: bf38         	it	lo
 800175e: 462e         	movlo	r6, r5
 8001760: f04f 33ff    	mov.w	r3, #0xffffffff
 8001764: 1b92         	subs	r2, r2, r6
 8001766: fa03 f906    	lsl.w	r9, r3, r6
 800176a: f002 031f    	and	r3, r2, #0x1f
 800176e: eba5 0506    	sub.w	r5, r5, r6
 8001772: f81e 800a    	ldrb.w	r8, [lr, r10]
 8001776: fa2c f303    	lsr.w	r3, r12, r3
 800177a: ea23 0309    	bic.w	r3, r3, r9
 800177e: f005 0507    	and	r5, r5, #0x7
 8001782: 44b3         	add	r11, r6
 8001784: fa03 f305    	lsl.w	r3, r3, r5
 8001788: ea43 0308    	orr.w	r3, r3, r8
 800178c: f80e 300a    	strb.w	r3, [lr, r10]
 8001790: d1d2         	bne	0x8001738 <raylar_crcl_encoder_service::format::Encoder::data::ha65fe528befbf4e3+0x2b4> @ imm = #-0x5c
 8001792: f8dd a070    	ldr.w	r10, [sp, #0x70]
 8001796: f8dd 8028    	ldr.w	r8, [sp, #0x28]
 800179a: e08c         	b	0x80018b6 <raylar_crcl_encoder_service::format::Encoder::data::ha65fe528befbf4e3+0x432> @ imm = #0x118
 800179c: 2c20         	cmp	r4, #0x20
 800179e: bf98         	it	ls
 80017a0: eb1b 0204    	addsls.w	r2, r11, r4
 80017a4: f080 80b8    	bhs.w	0x8001918 <raylar_crcl_encoder_service::format::Encoder::data::ha65fe528befbf4e3+0x494> @ imm = #0x170
 80017a8: 2300         	movs	r3, #0x0
 80017aa: ebb3 7f51    	cmp.w	r3, r1, lsr #29
 80017ae: d103         	bne	0x80017b8 <raylar_crcl_encoder_service::format::Encoder::data::ha65fe528befbf4e3+0x334> @ imm = #0x6
 80017b0: 00cb         	lsls	r3, r1, #0x3
 80017b2: 429a         	cmp	r2, r3
 80017b4: f200 80b0    	bhi.w	0x8001918 <raylar_crcl_encoder_service::format::Encoder::data::ha65fe528befbf4e3+0x494> @ imm = #0x160
 80017b8: 2c00         	cmp	r4, #0x0
 80017ba: f43f af71    	beq.w	0x80016a0 <raylar_crcl_encoder_service::format::Encoder::data::ha65fe528befbf4e3+0x21c> @ imm = #-0x11e
 80017be: f8dd c078    	ldr.w	r12, [sp, #0x78]
 80017c2: 961d         	str	r6, [sp, #0x74]
 80017c4: 4626         	mov	r6, r4
 80017c6: 9011         	str	r0, [sp, #0x44]
 80017c8: 4640         	mov	r0, r8
 80017ca: f01b 0207    	ands	r2, r11, #0x7
 80017ce: ea4f 08db    	lsr.w	r8, r11, #0x3
 80017d2: d105         	bne	0x80017e0 <raylar_crcl_encoder_service::format::Encoder::data::ha65fe528befbf4e3+0x35c> @ imm = #0xa
 80017d4: 4588         	cmp	r8, r1
 80017d6: f080 811b    	bhs.w	0x8001a10 <raylar_crcl_encoder_service::format::Encoder::data::ha65fe528befbf4e3+0x58c> @ imm = #0x236
 80017da: 2500         	movs	r5, #0x0
 80017dc: f80c 5008    	strb.w	r5, [r12, r8]
 80017e0: 4588         	cmp	r8, r1
 80017e2: f080 80ee    	bhs.w	0x80019c2 <raylar_crcl_encoder_service::format::Encoder::data::ha65fe528befbf4e3+0x53e> @ imm = #0x1dc
 80017e6: f1c2 0208    	rsb.w	r2, r2, #0x8
 80017ea: 4635         	mov	r5, r6
 80017ec: 42b2         	cmp	r2, r6
 80017ee: f04f 33ff    	mov.w	r3, #0xffffffff
 80017f2: bf38         	it	lo
 80017f4: 4615         	movlo	r5, r2
 80017f6: fa03 fe05    	lsl.w	lr, r3, r5
 80017fa: 1b76         	subs	r6, r6, r5
 80017fc: 9b1d         	ldr	r3, [sp, #0x74]
 80017fe: f006 091f    	and	r9, r6, #0x1f
 8001802: eba2 0205    	sub.w	r2, r2, r5
 8001806: f81c a008    	ldrb.w	r10, [r12, r8]
 800180a: f002 0207    	and	r2, r2, #0x7
 800180e: fa23 f909    	lsr.w	r9, r3, r9
 8001812: ea29 0e0e    	bic.w	lr, r9, lr
 8001816: 44ab         	add	r11, r5
 8001818: fa0e f202    	lsl.w	r2, lr, r2
 800181c: ea42 020a    	orr.w	r2, r2, r10
 8001820: f8dd a070    	ldr.w	r10, [sp, #0x70]
 8001824: f80c 2008    	strb.w	r2, [r12, r8]
 8001828: d1cf         	bne	0x80017ca <raylar_crcl_encoder_service::format::Encoder::data::ha65fe528befbf4e3+0x346> @ imm = #-0x62
 800182a: 4680         	mov	r8, r0
 800182c: e043         	b	0x80018b6 <raylar_crcl_encoder_service::format::Encoder::data::ha65fe528befbf4e3+0x432> @ imm = #0x86
 800182e: 2c20         	cmp	r4, #0x20
 8001830: bf98         	it	ls
 8001832: eb1b 0204    	addsls.w	r2, r11, r4
 8001836: d26f         	bhs	0x8001918 <raylar_crcl_encoder_service::format::Encoder::data::ha65fe528befbf4e3+0x494> @ imm = #0xde
 8001838: 2300         	movs	r3, #0x0
 800183a: ebb3 7f51    	cmp.w	r3, r1, lsr #29
 800183e: d102         	bne	0x8001846 <raylar_crcl_encoder_service::format::Encoder::data::ha65fe528befbf4e3+0x3c2> @ imm = #0x4
 8001840: 00cb         	lsls	r3, r1, #0x3
 8001842: 429a         	cmp	r2, r3
 8001844: d868         	bhi	0x8001918 <raylar_crcl_encoder_service::format::Encoder::data::ha65fe528befbf4e3+0x494> @ imm = #0xd0
 8001846: 2c00         	cmp	r4, #0x0
 8001848: f43f af2a    	beq.w	0x80016a0 <raylar_crcl_encoder_service::format::Encoder::data::ha65fe528befbf4e3+0x21c> @ imm = #-0x1ac
 800184c: f8dd c078    	ldr.w	r12, [sp, #0x78]
 8001850: 961d         	str	r6, [sp, #0x74]
 8001852: 4626         	mov	r6, r4
 8001854: 9011         	str	r0, [sp, #0x44]
 8001856: f01b 0207    	ands	r2, r11, #0x7
 800185a: ea4f 09db    	lsr.w	r9, r11, #0x3
 800185e: d105         	bne	0x800186c <raylar_crcl_encoder_service::format::Encoder::data::ha65fe528befbf4e3+0x3e8> @ imm = #0xa
 8001860: 4589         	cmp	r9, r1
 8001862: f080 80e3    	bhs.w	0x8001a2c <raylar_crcl_encoder_service::format::Encoder::data::ha65fe528befbf4e3+0x5a8> @ imm = #0x1c6
 8001866: 2300         	movs	r3, #0x0
 8001868: f80c 3009    	strb.w	r3, [r12, r9]
 800186c: 4589         	cmp	r9, r1
 800186e: f080 80d6    	bhs.w	0x8001a1e <raylar_crcl_encoder_service::format::Encoder::data::ha65fe528befbf4e3+0x59a> @ imm = #0x1ac
 8001872: f1c2 0208    	rsb.w	r2, r2, #0x8
 8001876: 4635         	mov	r5, r6
 8001878: 42b2         	cmp	r2, r6
 800187a: bf38         	it	lo
 800187c: 4615         	movlo	r5, r2
 800187e: f04f 33ff    	mov.w	r3, #0xffffffff
 8001882: 1b76         	subs	r6, r6, r5
 8001884: 981d         	ldr	r0, [sp, #0x74]
 8001886: fa03 fe05    	lsl.w	lr, r3, r5
 800188a: f006 031f    	and	r3, r6, #0x1f
 800188e: eba2 0205    	sub.w	r2, r2, r5
 8001892: f81c a009    	ldrb.w	r10, [r12, r9]
 8001896: f002 0207    	and	r2, r2, #0x7
 800189a: fa20 f303    	lsr.w	r3, r0, r3
 800189e: ea23 030e    	bic.w	r3, r3, lr
 80018a2: 44ab         	add	r11, r5
 80018a4: fa03 f202    	lsl.w	r2, r3, r2
 80018a8: ea42 020a    	orr.w	r2, r2, r10
 80018ac: f8dd a070    	ldr.w	r10, [sp, #0x70]
 80018b0: f80c 2009    	strb.w	r2, [r12, r9]
 80018b4: d1cf         	bne	0x8001856 <raylar_crcl_encoder_service::format::Encoder::data::ha65fe528befbf4e3+0x3d2> @ imm = #-0x62
 80018b6: 9811         	ldr	r0, [sp, #0x44]
 80018b8: f8cd b080    	str.w	r11, [sp, #0x80]
 80018bc: e6ef         	b	0x800169e <raylar_crcl_encoder_service::format::Encoder::data::ha65fe528befbf4e3+0x21a> @ imm = #-0x222
 80018be: f1cb 0000    	rsb.w	r0, r11, #0x0
 80018c2: f000 0307    	and	r3, r0, #0x7
 80018c6: eb1b 0003    	adds.w	r0, r11, r3
 80018ca: d225         	bhs	0x8001918 <raylar_crcl_encoder_service::format::Encoder::data::ha65fe528befbf4e3+0x494> @ imm = #0x4a
 80018cc: f8dd c078    	ldr.w	r12, [sp, #0x78]
 80018d0: 2200         	movs	r2, #0x0
 80018d2: ebb2 7f51    	cmp.w	r2, r1, lsr #29
 80018d6: d102         	bne	0x80018de <raylar_crcl_encoder_service::format::Encoder::data::ha65fe528befbf4e3+0x45a> @ imm = #0x4
 80018d8: 00ca         	lsls	r2, r1, #0x3
 80018da: 4290         	cmp	r0, r2
 80018dc: d81c         	bhi	0x8001918 <raylar_crcl_encoder_service::format::Encoder::data::ha65fe528befbf4e3+0x494> @ imm = #0x38
 80018de: b1c3         	cbz	r3, 0x8001912 <raylar_crcl_encoder_service::format::Encoder::data::ha65fe528befbf4e3+0x48e> @ imm = #0x30
 80018e0: f01b 0207    	ands	r2, r11, #0x7
 80018e4: ea4f 00db    	lsr.w	r0, r11, #0x3
 80018e8: d105         	bne	0x80018f6 <raylar_crcl_encoder_service::format::Encoder::data::ha65fe528befbf4e3+0x472> @ imm = #0xa
 80018ea: 4288         	cmp	r0, r1
 80018ec: f080 80a5    	bhs.w	0x8001a3a <raylar_crcl_encoder_service::format::Encoder::data::ha65fe528befbf4e3+0x5b6> @ imm = #0x14a
 80018f0: 2600         	movs	r6, #0x0
 80018f2: f80c 6000    	strb.w	r6, [r12, r0]
 80018f6: 4288         	cmp	r0, r1
 80018f8: f080 80a5    	bhs.w	0x8001a46 <raylar_crcl_encoder_service::format::Encoder::data::ha65fe528befbf4e3+0x5c2> @ imm = #0x14a
 80018fc: f1c2 0008    	rsb.w	r0, r2, #0x8
 8001900: 4298         	cmp	r0, r3
 8001902: bf28         	it	hs
 8001904: 4618         	movhs	r0, r3
 8001906: 9a0f         	ldr	r2, [sp, #0x3c]
 8001908: 4483         	add	r11, r0
 800190a: 9e19         	ldr	r6, [sp, #0x64]
 800190c: 1a1b         	subs	r3, r3, r0
 800190e: d1e7         	bne	0x80018e0 <raylar_crcl_encoder_service::format::Encoder::data::ha65fe528befbf4e3+0x45c> @ imm = #-0x32
 8001910: e698         	b	0x8001644 <raylar_crcl_encoder_service::format::Encoder::data::ha65fe528befbf4e3+0x1c0> @ imm = #-0x2d0
 8001912: 9a0f         	ldr	r2, [sp, #0x3c]
 8001914: 9e19         	ldr	r6, [sp, #0x64]
 8001916: e695         	b	0x8001644 <raylar_crcl_encoder_service::format::Encoder::data::ha65fe528befbf4e3+0x1c0> @ imm = #-0x2d6
 8001918: 2102         	movs	r1, #0x2
 800191a: 9812         	ldr	r0, [sp, #0x48]
 800191c: 2201         	movs	r2, #0x1
 800191e: 7041         	strb	r1, [r0, #0x1]
 8001920: 7002         	strb	r2, [r0]
 8001922: b021         	add	sp, #0x84
 8001924: e8bd 0f00    	pop.w	{r8, r9, r10, r11}
 8001928: bdf0         	pop	{r4, r5, r6, r7, pc}
 800192a: 2101         	movs	r1, #0x1
 800192c: 9810         	ldr	r0, [sp, #0x40]
 800192e: 9100         	str	r1, [sp]
 8001930: 4643         	mov	r3, r8
 8001932: 9907         	ldr	r1, [sp, #0x1c]
 8001934: 9001         	str	r0, [sp, #0x4]
 8001936: 9102         	str	r1, [sp, #0x8]
 8001938: 9908         	ldr	r1, [sp, #0x20]
 800193a: 9812         	ldr	r0, [sp, #0x48]
 800193c: 9103         	str	r1, [sp, #0xc]
 800193e: 991a         	ldr	r1, [sp, #0x68]
 8001940: e9cd 1504    	strd	r1, r5, [sp, #16]
 8001944: 4659         	mov	r1, r11
 8001946: f7ff fc86    	bl	0x8001256 <raylar_crcl_encoder_service::format::finish_record::hdf4f4fad6aa1e26d> @ imm = #-0x6f4
 800194a: b021         	add	sp, #0x84
 800194c: e8bd 0f00    	pop.w	{r8, r9, r10, r11}
 8001950: bdf0         	pop	{r4, r5, r6, r7, pc}
 8001952: 2102         	movs	r1, #0x2
 8001954: e7e2         	b	0x800191c <raylar_crcl_encoder_service::format::Encoder::data::ha65fe528befbf4e3+0x498> @ imm = #-0x3c
 8001956: eb09 0049    	add.w	r0, r9, r9, lsl #1
 800195a: 2124         	movs	r1, #0x24
 800195c: f64d 03d8    	movw	r3, #0xd8d8
 8001960: 4642         	mov	r2, r8
 8001962: eb01 0180    	add.w	r1, r1, r0, lsl #2
 8001966: f6c0 0300    	movt	r3, #0x800
 800196a: 4630         	mov	r0, r6
 800196c: f00a fa5f    	bl	0x800be2e <core::slice::index::slice_index_fail> @ imm = #0xa4be
 8001970: f64d 5338    	movw	r3, #0xdd38
 8001974: 2000         	movs	r0, #0x0
 8001976: f6c0 0300    	movt	r3, #0x800
 800197a: 4642         	mov	r2, r8
 800197c: f00a fa57    	bl	0x800be2e <core::slice::index::slice_index_fail> @ imm = #0xa4ae
 8001980: f64d 5328    	movw	r3, #0xdd28
 8001984: 4628         	mov	r0, r5
 8001986: f6c0 0300    	movt	r3, #0x800
 800198a: 4641         	mov	r1, r8
 800198c: 4642         	mov	r2, r8
 800198e: f00a fa4e    	bl	0x800be2e <core::slice::index::slice_index_fail> @ imm = #0xa49c
 8001992: 68ba         	ldr	r2, [r7, #0x8]
 8001994: f103 0128    	add.w	r1, r3, #0x28
 8001998: f64d 03d8    	movw	r3, #0xd8d8
 800199c: 4660         	mov	r0, r12
 800199e: f6c0 0300    	movt	r3, #0x800
 80019a2: f00a fa44    	bl	0x800be2e <core::slice::index::slice_index_fail> @ imm = #0xa488
 80019a6: f64d 0278    	movw	r2, #0xd878
 80019aa: 4650         	mov	r0, r10
 80019ac: f6c0 0200    	movt	r2, #0x800
 80019b0: f009 ff50    	bl	0x800b854 <core::panicking::panic_bounds_check> @ imm = #0x9ea0
 80019b4: f64d 0268    	movw	r2, #0xd868
 80019b8: 4650         	mov	r0, r10
 80019ba: f6c0 0200    	movt	r2, #0x800
 80019be: f009 ff49    	bl	0x800b854 <core::panicking::panic_bounds_check> @ imm = #0x9e92
 80019c2: f64d 0278    	movw	r2, #0xd878
 80019c6: 4640         	mov	r0, r8
 80019c8: f6c0 0200    	movt	r2, #0x800
 80019cc: f009 ff42    	bl	0x800b854 <core::panicking::panic_bounds_check> @ imm = #0x9e84
 80019d0: 991b         	ldr	r1, [sp, #0x6c]
 80019d2: f64d 5218    	movw	r2, #0xdd18
 80019d6: f6c0 0200    	movt	r2, #0x800
 80019da: 4618         	mov	r0, r3
 80019dc: f009 ff3a    	bl	0x800b854 <core::panicking::panic_bounds_check> @ imm = #0x9e74
 80019e0: f64d 42e8    	movw	r2, #0xdce8
 80019e4: f6c0 0200    	movt	r2, #0x800
 80019e8: 4608         	mov	r0, r1
 80019ea: 4641         	mov	r1, r8
 80019ec: f009 ff32    	bl	0x800b854 <core::panicking::panic_bounds_check> @ imm = #0x9e64
 80019f0: f64d 42f8    	movw	r2, #0xdcf8
 80019f4: f6c0 0200    	movt	r2, #0x800
 80019f8: 4608         	mov	r0, r1
 80019fa: 4641         	mov	r1, r8
 80019fc: f009 ff2a    	bl	0x800b854 <core::panicking::panic_bounds_check> @ imm = #0x9e54
 8001a00: f64d 5208    	movw	r2, #0xdd08
 8001a04: f6c0 0200    	movt	r2, #0x800
 8001a08: 4608         	mov	r0, r1
 8001a0a: 4641         	mov	r1, r8
 8001a0c: f009 ff22    	bl	0x800b854 <core::panicking::panic_bounds_check> @ imm = #0x9e44
 8001a10: f64d 0268    	movw	r2, #0xd868
 8001a14: 4640         	mov	r0, r8
 8001a16: f6c0 0200    	movt	r2, #0x800
 8001a1a: f009 ff1b    	bl	0x800b854 <core::panicking::panic_bounds_check> @ imm = #0x9e36
 8001a1e: f64d 0278    	movw	r2, #0xd878
 8001a22: 4648         	mov	r0, r9
 8001a24: f6c0 0200    	movt	r2, #0x800
 8001a28: f009 ff14    	bl	0x800b854 <core::panicking::panic_bounds_check> @ imm = #0x9e28
 8001a2c: f64d 0268    	movw	r2, #0xd868
 8001a30: 4648         	mov	r0, r9
 8001a32: f6c0 0200    	movt	r2, #0x800
 8001a36: f009 ff0d    	bl	0x800b854 <core::panicking::panic_bounds_check> @ imm = #0x9e1a
 8001a3a: f64d 0268    	movw	r2, #0xd868
 8001a3e: f6c0 0200    	movt	r2, #0x800
 8001a42: f009 ff07    	bl	0x800b854 <core::panicking::panic_bounds_check> @ imm = #0x9e0e
 8001a46: f64d 0278    	movw	r2, #0xd878
 8001a4a: f6c0 0200    	movt	r2, #0x800
 8001a4e: f009 ff01    	bl	0x800b854 <core::panicking::panic_bounds_check> @ imm = #0x9e02

08001a52 <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3>:
 8001a52: b5f0         	push	{r4, r5, r6, r7, lr}
 8001a54: af03         	add	r7, sp, #0xc
 8001a56: e92d 0f00    	push.w	{r8, r9, r10, r11}
 8001a5a: b087         	sub	sp, #0x1c
 8001a5c: f64a 23ab    	movw	r3, #0xaaab
 8001a60: f890 208f    	ldrb.w	r2, [r0, #0x8f]
 8001a64: f6ca 23aa    	movt	r3, #0xaaaa
 8001a68: 2a01         	cmp	r2, #0x1
 8001a6a: f040 80c9    	bne.w	0x8001c00 <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x1ae> @ imm = #0x192
 8001a6e: f8d0 b070    	ldr.w	r11, [r0, #0x70]
 8001a72: 2900         	cmp	r1, #0x0
 8001a74: f000 80c1    	beq.w	0x8001bfa <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x1a8> @ imm = #0x182
 8001a78: f1bb 0f05    	cmp.w	r11, #0x5
 8001a7c: f200 80bd    	bhi.w	0x8001bfa <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x1a8> @ imm = #0x17a
 8001a80: 6c42         	ldr	r2, [r0, #0x44]
 8001a82: f1a1 0c01    	sub.w	r12, r1, #0x1
 8001a86: f04f 0801    	mov.w	r8, #0x1
 8001a8a: f04f 0901    	mov.w	r9, #0x1
 8001a8e: fba2 2303    	umull	r2, r3, r2, r3
 8001a92: f890 2091    	ldrb.w	r2, [r0, #0x91]
 8001a96: ea4f 0a93    	lsr.w	r10, r3, #0x2
 8001a9a: f1aa 0e01    	sub.w	lr, r10, #0x1
 8001a9e: 6f45         	ldr	r5, [r0, #0x74]
 8001aa0: 07d2         	lsls	r2, r2, #0x1f
 8001aa2: 6c41         	ldr	r1, [r0, #0x44]
 8001aa4: eb05 0345    	add.w	r3, r5, r5, lsl #1
 8001aa8: eb0b 0343    	add.w	r3, r11, r3, lsl #1
 8001aac: d032         	beq	0x8001b14 <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0xc2> @ imm = #0x64
 8001aae: 428b         	cmp	r3, r1
 8001ab0: f080 8360    	bhs.w	0x8002174 <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x722> @ imm = #0x6c0
 8001ab4: f890 1090    	ldrb.w	r1, [r0, #0x90]
 8001ab8: 6c06         	ldr	r6, [r0, #0x40]
 8001aba: 1e4c         	subs	r4, r1, #0x1
 8001abc: 2900         	cmp	r1, #0x0
 8001abe: f004 041f    	and	r4, r4, #0x1f
 8001ac2: f856 2023    	ldr.w	r2, [r6, r3, lsl #2]
 8001ac6: fa08 f404    	lsl.w	r4, r8, r4
 8001aca: bf18         	it	ne
 8001acc: 4422         	addne	r2, r4
 8001ace: f001 041f    	and	r4, r1, #0x1f
 8001ad2: fa42 f204    	asr.w	r2, r2, r4
 8001ad6: f846 2023    	str.w	r2, [r6, r3, lsl #2]
 8001ada: b145         	cbz	r5, 0x8001aee <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x9c> @ imm = #0x10
 8001adc: e9d0 341f    	ldrd	r3, r4, [r0, #124]
 8001ae0: 1ad3         	subs	r3, r2, r3
 8001ae2: 005e         	lsls	r6, r3, #0x1
 8001ae4: ea86 73e3    	eor.w	r3, r6, r3, asr #31
 8001ae8: 4423         	add	r3, r4
 8001aea: f8c0 3080    	str.w	r3, [r0, #0x80]
 8001aee: 1c6b         	adds	r3, r5, #0x1
 8001af0: 6743         	str	r3, [r0, #0x74]
 8001af2: 4553         	cmp	r3, r10
 8001af4: 67c2         	str	r2, [r0, #0x7c]
 8001af6: d14e         	bne	0x8001b96 <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x144> @ imm = #0x9c
 8001af8: f8d0 3080    	ldr.w	r3, [r0, #0x80]
 8001afc: 2200         	movs	r2, #0x0
 8001afe: f1be 0f00    	cmp.w	lr, #0x0
 8001b02: 6742         	str	r2, [r0, #0x74]
 8001b04: bf18         	it	ne
 8001b06: 2b00         	cmpne	r3, #0x0
 8001b08: d149         	bne	0x8001b9e <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x14c> @ imm = #0x92
 8001b0a: f1ab 0301    	sub.w	r3, r11, #0x1
 8001b0e: 2b04         	cmp	r3, #0x4
 8001b10: d955         	bls	0x8001bbe <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x16c> @ imm = #0xaa
 8001b12: e34e         	b	0x80021b2 <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x760> @ imm = #0x69c
 8001b14: 428b         	cmp	r3, r1
 8001b16: f080 8326    	bhs.w	0x8002166 <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x714> @ imm = #0x64c
 8001b1a: 6c01         	ldr	r1, [r0, #0x40]
 8001b1c: f8d0 2088    	ldr.w	r2, [r0, #0x88]
 8001b20: f851 1023    	ldr.w	r1, [r1, r3, lsl #2]
 8001b24: 4253         	rsbs	r3, r2, #0
 8001b26: 4299         	cmp	r1, r3
 8001b28: f2c0 8308    	blt.w	0x800213c <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x6ea> @ imm = #0x610
 8001b2c: 4291         	cmp	r1, r2
 8001b2e: f04f 0201    	mov.w	r2, #0x1
 8001b32: f300 82fd    	bgt.w	0x8002130 <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x6de> @ imm = #0x5fa
 8001b36: 6f83         	ldr	r3, [r0, #0x78]
 8001b38: 1c6a         	adds	r2, r5, #0x1
 8001b3a: 6742         	str	r2, [r0, #0x74]
 8001b3c: 2900         	cmp	r1, #0x0
 8001b3e: bf48         	it	mi
 8001b40: 4249         	rsbmi	r1, r1, #0
 8001b42: 4299         	cmp	r1, r3
 8001b44: bfd8         	it	le
 8001b46: 4619         	movle	r1, r3
 8001b48: 2300         	movs	r3, #0x0
 8001b4a: 4552         	cmp	r2, r10
 8001b4c: 6781         	str	r1, [r0, #0x78]
 8001b4e: d124         	bne	0x8001b9a <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x148> @ imm = #0x48
 8001b50: f890 208c    	ldrb.w	r2, [r0, #0x8c]
 8001b54: fab1 f581    	clz	r5, r1
 8001b58: f1c5 0520    	rsb.w	r5, r5, #0x20
 8001b5c: 6743         	str	r3, [r0, #0x74]
 8001b5e: 3a01         	subs	r2, #0x1
 8001b60: b2d6         	uxtb	r6, r2
 8001b62: 42b5         	cmp	r5, r6
 8001b64: d90c         	bls	0x8001b80 <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x12e> @ imm = #0x18
 8001b66: 1bab         	subs	r3, r5, r6
 8001b68: f002 021f    	and	r2, r2, #0x1f
 8001b6c: 1e5c         	subs	r4, r3, #0x1
 8001b6e: fa08 f404    	lsl.w	r4, r8, r4
 8001b72: 4421         	add	r1, r4
 8001b74: f003 041f    	and	r4, r3, #0x1f
 8001b78: 40e1         	lsrs	r1, r4
 8001b7a: 40d1         	lsrs	r1, r2
 8001b7c: bf18         	it	ne
 8001b7e: 3301         	addne	r3, #0x1
 8001b80: f890 108d    	ldrb.w	r1, [r0, #0x8d]
 8001b84: 2201         	movs	r2, #0x1
 8001b86: f880 2091    	strb.w	r2, [r0, #0x91]
 8001b8a: 4299         	cmp	r1, r3
 8001b8c: bf88         	it	hi
 8001b8e: 460b         	movhi	r3, r1
 8001b90: f880 3090    	strb.w	r3, [r0, #0x90]
 8001b94: e028         	b	0x8001be8 <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x196> @ imm = #0x50
 8001b96: 2201         	movs	r2, #0x1
 8001b98: e026         	b	0x8001be8 <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x196> @ imm = #0x4c
 8001b9a: 2200         	movs	r2, #0x0
 8001b9c: e024         	b	0x8001be8 <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x196> @ imm = #0x48
 8001b9e: fbb3 f2fe    	udiv	r2, r3, lr
 8001ba2: fb02 331e    	mls	r3, r2, lr, r3
 8001ba6: 2b00         	cmp	r3, #0x0
 8001ba8: bf18         	it	ne
 8001baa: 3201         	addne	r2, #0x1
 8001bac: fab2 f282    	clz	r2, r2
 8001bb0: f1c2 021f    	rsb.w	r2, r2, #0x1f
 8001bb4: f1ab 0301    	sub.w	r3, r11, #0x1
 8001bb8: 2b04         	cmp	r3, #0x4
 8001bba: f200 82fa    	bhi.w	0x80021b2 <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x760> @ imm = #0x5f4
 8001bbe: f100 0448    	add.w	r4, r0, #0x48
 8001bc2: f844 1033    	str.w	r1, [r4, r3, lsl #3]
 8001bc6: eb04 01c3    	add.w	r1, r4, r3, lsl #3
 8001bca: 2402         	movs	r4, #0x2
 8001bcc: 6f03         	ldr	r3, [r0, #0x70]
 8001bce: 710a         	strb	r2, [r1, #0x4]
 8001bd0: 2200         	movs	r2, #0x0
 8001bd2: 714c         	strb	r4, [r1, #0x5]
 8001bd4: f103 0b01    	add.w	r11, r3, #0x1
 8001bd8: f880 2091    	strb.w	r2, [r0, #0x91]
 8001bdc: e9c0 221e    	strd	r2, r2, [r0, #120]
 8001be0: f8c0 2080    	str.w	r2, [r0, #0x80]
 8001be4: f8c0 b070    	str.w	r11, [r0, #0x70]
 8001be8: f1ac 0101    	sub.w	r1, r12, #0x1
 8001bec: 4561         	cmp	r1, r12
 8001bee: d204         	bhs	0x8001bfa <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x1a8> @ imm = #0x8
 8001bf0: 468c         	mov	r12, r1
 8001bf2: f1bb 0f06    	cmp.w	r11, #0x6
 8001bf6: f4ff af52    	blo.w	0x8001a9e <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x4c> @ imm = #-0x15c
 8001bfa: f1ab 0006    	sub.w	r0, r11, #0x6
 8001bfe: e292         	b	0x8002126 <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x6d4> @ imm = #0x524
 8001c00: 6c46         	ldr	r6, [r0, #0x44]
 8001c02: 2900         	cmp	r1, #0x0
 8001c04: 6f02         	ldr	r2, [r0, #0x70]
 8001c06: fba6 6303    	umull	r6, r3, r6, r3
 8001c0a: f000 828b    	beq.w	0x8002124 <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x6d2> @ imm = #0x516
 8001c0e: 2a05         	cmp	r2, #0x5
 8001c10: f200 8288    	bhi.w	0x8002124 <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x6d2> @ imm = #0x510
 8001c14: ea4f 0893    	lsr.w	r8, r3, #0x2
 8001c18: f04f 3bff    	mov.w	r11, #0xffffffff
 8001c1c: f1a8 0301    	sub.w	r3, r8, #0x1
 8001c20: 9306         	str	r3, [sp, #0x18]
 8001c22: f8cd 8008    	str.w	r8, [sp, #0x8]
 8001c26: 6f46         	ldr	r6, [r0, #0x74]
 8001c28: 6c43         	ldr	r3, [r0, #0x44]
 8001c2a: f890 5091    	ldrb.w	r5, [r0, #0x91]
 8001c2e: eb06 0646    	add.w	r6, r6, r6, lsl #1
 8001c32: eb02 0c46    	add.w	r12, r2, r6, lsl #1
 8001c36: b39d         	cbz	r5, 0x8001ca0 <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x24e> @ imm = #0x66
 8001c38: 459c         	cmp	r12, r3
 8001c3a: f080 82aa    	bhs.w	0x8002192 <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x740> @ imm = #0x554
 8001c3e: f890 508c    	ldrb.w	r5, [r0, #0x8c]
 8001c42: 2201         	movs	r2, #0x1
 8001c44: f04f 0900    	mov.w	r9, #0x0
 8001c48: 2d0c         	cmp	r5, #0xc
 8001c4a: f200 8271    	bhi.w	0x8002130 <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x6de> @ imm = #0x4e2
 8001c4e: 2301         	movs	r3, #0x1
 8001c50: 40ab         	lsls	r3, r5
 8001c52: f413 5faa    	tst.w	r3, #0x1540
 8001c56: f000 826b    	beq.w	0x8002130 <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x6de> @ imm = #0x4d6
 8001c5a: 6f83         	ldr	r3, [r0, #0x78]
 8001c5c: 2b00         	cmp	r3, #0x0
 8001c5e: f100 8267    	bmi.w	0x8002130 <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x6de> @ imm = #0x4ce
 8001c62: f8d0 9040    	ldr.w	r9, [r0, #0x40]
 8001c66: f859 402c    	ldr.w	r4, [r9, r12, lsl #2]
 8001c6a: f1b4 3fff    	cmp.w	r4, #0xffffffff
 8001c6e: 46a2         	mov	r10, r4
 8001c70: dc05         	bgt	0x8001c7e <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x22c> @ imm = #0xa
 8001c72: f1b4 4f00    	cmp.w	r4, #0x80000000
 8001c76: f000 8263    	beq.w	0x8002140 <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x6ee> @ imm = #0x4c6
 8001c7a: f1c4 0a00    	rsb.w	r10, r4, #0x0
 8001c7e: 459a         	cmp	r10, r3
 8001c80: f200 825e    	bhi.w	0x8002140 <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x6ee> @ imm = #0x4bc
 8001c84: b3e3         	cbz	r3, 0x8001d00 <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x2ae> @ imm = #0x78
 8001c86: 1e6a         	subs	r2, r5, #0x1
 8001c88: 459a         	cmp	r10, r3
 8001c8a: b2d5         	uxtb	r5, r2
 8001c8c: fa0b f605    	lsl.w	r6, r11, r5
 8001c90: ea6f 0e06    	mvn.w	lr, r6
 8001c94: d137         	bne	0x8001d06 <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x2b4> @ imm = #0x6e
 8001c96: 2c00         	cmp	r4, #0x0
 8001c98: bf48         	it	mi
 8001c9a: f106 0e01    	addmi.w	lr, r6, #0x1
 8001c9e: e045         	b	0x8001d2c <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x2da> @ imm = #0x8a
 8001ca0: ea4f 0e46    	lsl.w	lr, r6, #0x1
 8001ca4: 459e         	cmp	lr, r3
 8001ca6: f080 827c    	bhs.w	0x80021a2 <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x750> @ imm = #0x4f8
 8001caa: 459c         	cmp	r12, r3
 8001cac: f080 8269    	bhs.w	0x8002182 <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x730> @ imm = #0x4d2
 8001cb0: 6c04         	ldr	r4, [r0, #0x40]
 8001cb2: f04f 0901    	mov.w	r9, #0x1
 8001cb6: f8d0 2088    	ldr.w	r2, [r0, #0x88]
 8001cba: f854 502c    	ldr.w	r5, [r4, r12, lsl #2]
 8001cbe: 4295         	cmp	r5, r2
 8001cc0: f300 823c    	bgt.w	0x800213c <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x6ea> @ imm = #0x478
 8001cc4: 4253         	rsbs	r3, r2, #0
 8001cc6: 429d         	cmp	r5, r3
 8001cc8: f2c0 8238    	blt.w	0x800213c <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x6ea> @ imm = #0x470
 8001ccc: f854 602e    	ldr.w	r6, [r4, lr, lsl #2]
 8001cd0: 429e         	cmp	r6, r3
 8001cd2: f2c0 8233    	blt.w	0x800213c <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x6ea> @ imm = #0x466
 8001cd6: 4296         	cmp	r6, r2
 8001cd8: f04f 0201    	mov.w	r2, #0x1
 8001cdc: f300 8228    	bgt.w	0x8002130 <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x6de> @ imm = #0x450
 8001ce0: 1baa         	subs	r2, r5, r6
 8001ce2: f844 202c    	str.w	r2, [r4, r12, lsl #2]
 8001ce6: e9d0 431d    	ldrd	r4, r3, [r0, #116]
 8001cea: bf48         	it	mi
 8001cec: 4252         	rsbmi	r2, r2, #0
 8001cee: 429a         	cmp	r2, r3
 8001cf0: bfd8         	it	le
 8001cf2: 461a         	movle	r2, r3
 8001cf4: 6782         	str	r2, [r0, #0x78]
 8001cf6: 1c62         	adds	r2, r4, #0x1
 8001cf8: 6742         	str	r2, [r0, #0x74]
 8001cfa: 4542         	cmp	r2, r8
 8001cfc: d03f         	beq	0x8001d7e <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x32c> @ imm = #0x7e
 8001cfe: e18f         	b	0x8002020 <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x5ce> @ imm = #0x31e
 8001d00: f04f 0e00    	mov.w	lr, #0x0
 8001d04: e012         	b	0x8001d2c <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x2da> @ imm = #0x24
 8001d06: fb8a 620e    	smull	r6, r2, r10, lr
 8001d0a: ebb2 7fe6    	cmp.w	r2, r6, asr #31
 8001d0e: f040 818e    	bne.w	0x800202e <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x5dc> @ imm = #0x31c
 8001d12: fb96 fef3    	sdiv	lr, r6, r3
 8001d16: fb0e 6513    	mls	r5, lr, r3, r6
 8001d1a: 1b5b         	subs	r3, r3, r5
 8001d1c: 429d         	cmp	r5, r3
 8001d1e: bfa8         	it	ge
 8001d20: f10e 0e01    	addge.w	lr, lr, #0x1
 8001d24: 2c00         	cmp	r4, #0x0
 8001d26: bf48         	it	mi
 8001d28: f1ce 0e00    	rsbmi.w	lr, lr, #0x0
 8001d2c: f849 e02c    	str.w	lr, [r9, r12, lsl #2]
 8001d30: 1d02         	adds	r2, r0, #0x4
 8001d32: f890 608e    	ldrb.w	r6, [r0, #0x8e]
 8001d36: 2e01         	cmp	r6, #0x1
 8001d38: d009         	beq	0x8001d4e <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x2fc> @ imm = #0x12
 8001d3a: ea4f 044e    	lsl.w	r4, lr, #0x1
 8001d3e: 6803         	ldr	r3, [r0]
 8001d40: ea84 75ee    	eor.w	r5, r4, lr, asr #31
 8001d44: ebb4 7fee    	cmp.w	r4, lr, asr #31
 8001d48: 442b         	add	r3, r5
 8001d4a: 6003         	str	r3, [r0]
 8001d4c: d13a         	bne	0x8001dc4 <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x372> @ imm = #0x74
 8001d4e: 6f44         	ldr	r4, [r0, #0x74]
 8001d50: 2e00         	cmp	r6, #0x0
 8001d52: bf18         	it	ne
 8001d54: 2c00         	cmpne	r4, #0x0
 8001d56: d00b         	beq	0x8001d70 <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x31e> @ imm = #0x16
 8001d58: 6fc6         	ldr	r6, [r0, #0x7c]
 8001d5a: 6a03         	ldr	r3, [r0, #0x20]
 8001d5c: ebae 0506    	sub.w	r5, lr, r6
 8001d60: 006a         	lsls	r2, r5, #0x1
 8001d62: ea82 76e5    	eor.w	r6, r2, r5, asr #31
 8001d66: 4433         	add	r3, r6
 8001d68: ebb2 7fe5    	cmp.w	r2, r5, asr #31
 8001d6c: 6203         	str	r3, [r0, #0x20]
 8001d6e: d166         	bne	0x8001e3e <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x3ec> @ imm = #0xcc
 8001d70: f8c0 e07c    	str.w	lr, [r0, #0x7c]
 8001d74: 1c62         	adds	r2, r4, #0x1
 8001d76: 6742         	str	r2, [r0, #0x74]
 8001d78: 4542         	cmp	r2, r8
 8001d7a: f040 8151    	bne.w	0x8002020 <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x5ce> @ imm = #0x2a2
 8001d7e: f890 2091    	ldrb.w	r2, [r0, #0x91]
 8001d82: 2300         	movs	r3, #0x0
 8001d84: 6743         	str	r3, [r0, #0x74]
 8001d86: 2a00         	cmp	r2, #0x0
 8001d88: f000 80a0    	beq.w	0x8001ecc <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x47a> @ imm = #0x140
 8001d8c: f890 e08c    	ldrb.w	lr, [r0, #0x8c]
 8001d90: 6f84         	ldr	r4, [r0, #0x78]
 8001d92: fb08 f20e    	mul	r2, r8, lr
 8001d96: 0753         	lsls	r3, r2, #0x1d
 8001d98: ea4f 06d2    	lsr.w	r6, r2, #0x3
 8001d9c: f04f 0301    	mov.w	r3, #0x1
 8001da0: bf18         	it	ne
 8001da2: eb03 06d2    	addne.w	r6, r3, r2, lsr #3
 8001da6: 2c00         	cmp	r4, #0x0
 8001da8: f000 8094    	beq.w	0x8001ed4 <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x482> @ imm = #0x128
 8001dac: f890 208e    	ldrb.w	r2, [r0, #0x8e]
 8001db0: 9404         	str	r4, [sp, #0x10]
 8001db2: 2a01         	cmp	r2, #0x1
 8001db4: 9605         	str	r6, [sp, #0x14]
 8001db6: f040 8091    	bne.w	0x8001edc <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x48a> @ imm = #0x122
 8001dba: 2203         	movs	r2, #0x3
 8001dbc: f04f 0c00    	mov.w	r12, #0x0
 8001dc0: 9203         	str	r2, [sp, #0xc]
 8001dc2: e0c4         	b	0x8001f4e <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x4fc> @ imm = #0x188
 8001dc4: fa95 f3a5    	rbit	r3, r5
 8001dc8: fab3 f383    	clz	r3, r3
 8001dcc: 2b0c         	cmp	r3, #0xc
 8001dce: f200 81ba    	bhi.w	0x8002146 <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x6f4> @ imm = #0x374
 8001dd2: f832 4013    	ldrh.w	r4, [r2, r3, lsl #1]
 8001dd6: 3401         	adds	r4, #0x1
 8001dd8: f822 4013    	strh.w	r4, [r2, r3, lsl #1]
 8001ddc: 1e6b         	subs	r3, r5, #0x1
 8001dde: 401d         	ands	r5, r3
 8001de0: d0b5         	beq	0x8001d4e <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x2fc> @ imm = #-0x96
 8001de2: fa95 f3a5    	rbit	r3, r5
 8001de6: fab3 f383    	clz	r3, r3
 8001dea: 2b0c         	cmp	r3, #0xc
 8001dec: f200 81ab    	bhi.w	0x8002146 <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x6f4> @ imm = #0x356
 8001df0: f832 4013    	ldrh.w	r4, [r2, r3, lsl #1]
 8001df4: 3401         	adds	r4, #0x1
 8001df6: f822 4013    	strh.w	r4, [r2, r3, lsl #1]
 8001dfa: 1e6b         	subs	r3, r5, #0x1
 8001dfc: 401d         	ands	r5, r3
 8001dfe: d0a6         	beq	0x8001d4e <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x2fc> @ imm = #-0xb4
 8001e00: fa95 f3a5    	rbit	r3, r5
 8001e04: fab3 f383    	clz	r3, r3
 8001e08: 2b0c         	cmp	r3, #0xc
 8001e0a: f200 819c    	bhi.w	0x8002146 <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x6f4> @ imm = #0x338
 8001e0e: f832 4013    	ldrh.w	r4, [r2, r3, lsl #1]
 8001e12: 3401         	adds	r4, #0x1
 8001e14: f822 4013    	strh.w	r4, [r2, r3, lsl #1]
 8001e18: 1e6b         	subs	r3, r5, #0x1
 8001e1a: 401d         	ands	r5, r3
 8001e1c: d097         	beq	0x8001d4e <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x2fc> @ imm = #-0xd2
 8001e1e: fa95 f3a5    	rbit	r3, r5
 8001e22: fab3 f383    	clz	r3, r3
 8001e26: 2b0c         	cmp	r3, #0xc
 8001e28: f200 818d    	bhi.w	0x8002146 <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x6f4> @ imm = #0x31a
 8001e2c: f832 4013    	ldrh.w	r4, [r2, r3, lsl #1]
 8001e30: 3401         	adds	r4, #0x1
 8001e32: f822 4013    	strh.w	r4, [r2, r3, lsl #1]
 8001e36: 1e6b         	subs	r3, r5, #0x1
 8001e38: 401d         	ands	r5, r3
 8001e3a: d1c3         	bne	0x8001dc4 <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x372> @ imm = #-0x7a
 8001e3c: e787         	b	0x8001d4e <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x2fc> @ imm = #-0xf2
 8001e3e: fa96 f2a6    	rbit	r2, r6
 8001e42: fab2 f382    	clz	r3, r2
 8001e46: 2b0c         	cmp	r3, #0xc
 8001e48: f200 817d    	bhi.w	0x8002146 <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x6f4> @ imm = #0x2fa
 8001e4c: f100 0524    	add.w	r5, r0, #0x24
 8001e50: f835 2013    	ldrh.w	r2, [r5, r3, lsl #1]
 8001e54: 3201         	adds	r2, #0x1
 8001e56: f825 2013    	strh.w	r2, [r5, r3, lsl #1]
 8001e5a: 1e72         	subs	r2, r6, #0x1
 8001e5c: 4016         	ands	r6, r2
 8001e5e: d087         	beq	0x8001d70 <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x31e> @ imm = #-0xf2
 8001e60: fa96 f2a6    	rbit	r2, r6
 8001e64: fab2 f382    	clz	r3, r2
 8001e68: 2b0c         	cmp	r3, #0xc
 8001e6a: f200 816c    	bhi.w	0x8002146 <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x6f4> @ imm = #0x2d8
 8001e6e: f100 0524    	add.w	r5, r0, #0x24
 8001e72: f835 2013    	ldrh.w	r2, [r5, r3, lsl #1]
 8001e76: 3201         	adds	r2, #0x1
 8001e78: f825 2013    	strh.w	r2, [r5, r3, lsl #1]
 8001e7c: 1e72         	subs	r2, r6, #0x1
 8001e7e: 4016         	ands	r6, r2
 8001e80: f43f af76    	beq.w	0x8001d70 <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x31e> @ imm = #-0x114
 8001e84: fa96 f2a6    	rbit	r2, r6
 8001e88: fab2 f382    	clz	r3, r2
 8001e8c: 2b0c         	cmp	r3, #0xc
 8001e8e: f200 815a    	bhi.w	0x8002146 <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x6f4> @ imm = #0x2b4
 8001e92: f100 0524    	add.w	r5, r0, #0x24
 8001e96: f835 2013    	ldrh.w	r2, [r5, r3, lsl #1]
 8001e9a: 3201         	adds	r2, #0x1
 8001e9c: f825 2013    	strh.w	r2, [r5, r3, lsl #1]
 8001ea0: 1e72         	subs	r2, r6, #0x1
 8001ea2: 4016         	ands	r6, r2
 8001ea4: f43f af64    	beq.w	0x8001d70 <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x31e> @ imm = #-0x138
 8001ea8: fa96 f2a6    	rbit	r2, r6
 8001eac: fab2 f382    	clz	r3, r2
 8001eb0: 2b0c         	cmp	r3, #0xc
 8001eb2: f200 8148    	bhi.w	0x8002146 <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x6f4> @ imm = #0x290
 8001eb6: f100 0524    	add.w	r5, r0, #0x24
 8001eba: f835 2013    	ldrh.w	r2, [r5, r3, lsl #1]
 8001ebe: 3201         	adds	r2, #0x1
 8001ec0: f825 2013    	strh.w	r2, [r5, r3, lsl #1]
 8001ec4: 1e72         	subs	r2, r6, #0x1
 8001ec6: 4016         	ands	r6, r2
 8001ec8: d1b9         	bne	0x8001e3e <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x3ec> @ imm = #-0x8e
 8001eca: e751         	b	0x8001d70 <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x31e> @ imm = #-0x15e
 8001ecc: 2201         	movs	r2, #0x1
 8001ece: f880 2091    	strb.w	r2, [r0, #0x91]
 8001ed2: e0a5         	b	0x8002020 <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x5ce> @ imm = #0x14a
 8001ed4: 2500         	movs	r5, #0x0
 8001ed6: f04f 0c00    	mov.w	r12, #0x0
 8001eda: e079         	b	0x8001fd0 <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x57e> @ imm = #0xf2
 8001edc: f8d0 a000    	ldr.w	r10, [r0]
 8001ee0: f04f 0c00    	mov.w	r12, #0x0
 8001ee4: f04f 3bff    	mov.w	r11, #0xffffffff
 8001ee8: 2400         	movs	r4, #0x0
 8001eea: 9203         	str	r2, [sp, #0xc]
 8001eec: b2e3         	uxtb	r3, r4
 8001eee: 2b0c         	cmp	r3, #0xc
 8001ef0: f200 8131    	bhi.w	0x8002156 <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x704> @ imm = #0x262
 8001ef4: 1c5a         	adds	r2, r3, #0x1
 8001ef6: 2601         	movs	r6, #0x1
 8001ef8: fb02 a208    	mla	r2, r2, r8, r10
 8001efc: 08d5         	lsrs	r5, r2, #0x3
 8001efe: ea5f 7942    	lsls.w	r9, r2, #0x1d
 8001f02: bf18         	it	ne
 8001f04: eb06 05d2    	addne.w	r5, r6, r2, lsr #3
 8001f08: 455d         	cmp	r5, r11
 8001f0a: f04f 0200    	mov.w	r2, #0x0
 8001f0e: bf3c         	itt	lo
 8001f10: 46ab         	movlo	r11, r5
 8001f12: 46a4         	movlo	r12, r4
 8001f14: 4573         	cmp	r3, lr
 8001f16: bf38         	it	lo
 8001f18: 2201         	movlo	r2, #0x1
 8001f1a: d20a         	bhs	0x8001f32 <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x4e0> @ imm = #0x14
 8001f1c: 1d05         	adds	r5, r0, #0x4
 8001f1e: 4414         	add	r4, r2
 8001f20: f835 3013    	ldrh.w	r3, [r5, r3, lsl #1]
 8001f24: ebaa 0203    	sub.w	r2, r10, r3
 8001f28: ea4f 0a52    	lsr.w	r10, r2, #0x1
 8001f2c: b2e2         	uxtb	r2, r4
 8001f2e: 4572         	cmp	r2, lr
 8001f30: d9dc         	bls	0x8001eec <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x49a> @ imm = #-0x48
 8001f32: 9b05         	ldr	r3, [sp, #0x14]
 8001f34: 2200         	movs	r2, #0x0
 8001f36: 2503         	movs	r5, #0x3
 8001f38: 459b         	cmp	r11, r3
 8001f3a: bf2c         	ite	hs
 8001f3c: 4694         	movhs	r12, r2
 8001f3e: 465b         	movlo	r3, r11
 8001f40: bf38         	it	lo
 8001f42: 2501         	movlo	r5, #0x1
 8001f44: 9a03         	ldr	r2, [sp, #0xc]
 8001f46: 2a00         	cmp	r2, #0x0
 8001f48: d03f         	beq	0x8001fca <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x578> @ imm = #0x7e
 8001f4a: 9503         	str	r5, [sp, #0xc]
 8001f4c: 9305         	str	r3, [sp, #0x14]
 8001f4e: f100 0220    	add.w	r2, r0, #0x20
 8001f52: f04f 0b00    	mov.w	r11, #0x0
 8001f56: f04f 35ff    	mov.w	r5, #0xffffffff
 8001f5a: f04f 0900    	mov.w	r9, #0x0
 8001f5e: f8d2 8000    	ldr.w	r8, [r2]
 8001f62: fa5f f389    	uxtb.w	r3, r9
 8001f66: 2b0c         	cmp	r3, #0xc
 8001f68: f200 80f5    	bhi.w	0x8002156 <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x704> @ imm = #0x1ea
 8001f6c: 9c06         	ldr	r4, [sp, #0x18]
 8001f6e: f103 0a01    	add.w	r10, r3, #0x1
 8001f72: eb08 020e    	add.w	r2, r8, lr
 8001f76: fb0a 2204    	mla	r2, r10, r4, r2
 8001f7a: 0756         	lsls	r6, r2, #0x1d
 8001f7c: ea4f 04d2    	lsr.w	r4, r2, #0x3
 8001f80: f04f 0601    	mov.w	r6, #0x1
 8001f84: bf18         	it	ne
 8001f86: eb06 04d2    	addne.w	r4, r6, r2, lsr #3
 8001f8a: 42ac         	cmp	r4, r5
 8001f8c: f04f 0200    	mov.w	r2, #0x0
 8001f90: bf3c         	itt	lo
 8001f92: 4625         	movlo	r5, r4
 8001f94: 46cb         	movlo	r11, r9
 8001f96: 4573         	cmp	r3, lr
 8001f98: bf38         	it	lo
 8001f9a: 2201         	movlo	r2, #0x1
 8001f9c: d20c         	bhs	0x8001fb8 <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x566> @ imm = #0x18
 8001f9e: f100 0424    	add.w	r4, r0, #0x24
 8001fa2: 4491         	add	r9, r2
 8001fa4: f834 3013    	ldrh.w	r3, [r4, r3, lsl #1]
 8001fa8: eba8 0203    	sub.w	r2, r8, r3
 8001fac: ea4f 0852    	lsr.w	r8, r2, #0x1
 8001fb0: fa5f f289    	uxtb.w	r2, r9
 8001fb4: 4572         	cmp	r2, lr
 8001fb6: d9d4         	bls	0x8001f62 <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x510> @ imm = #-0x58
 8001fb8: 9a05         	ldr	r2, [sp, #0x14]
 8001fba: 4295         	cmp	r5, r2
 8001fbc: bf38         	it	lo
 8001fbe: 46dc         	movlo	r12, r11
 8001fc0: 9d03         	ldr	r5, [sp, #0xc]
 8001fc2: bf38         	it	lo
 8001fc4: 2502         	movlo	r5, #0x2
 8001fc6: f8dd 8008    	ldr.w	r8, [sp, #0x8]
 8001fca: f04f 3bff    	mov.w	r11, #0xffffffff
 8001fce: 9c04         	ldr	r4, [sp, #0x10]
 8001fd0: 6f02         	ldr	r2, [r0, #0x70]
 8001fd2: 1e53         	subs	r3, r2, #0x1
 8001fd4: 2b05         	cmp	r3, #0x5
 8001fd6: f080 80f5    	bhs.w	0x80021c4 <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x772> @ imm = #0x1ea
 8001fda: f100 0248    	add.w	r2, r0, #0x48
 8001fde: 2600         	movs	r6, #0x0
 8001fe0: f842 4033    	str.w	r4, [r2, r3, lsl #3]
 8001fe4: eb02 02c3    	add.w	r2, r2, r3, lsl #3
 8001fe8: 6f03         	ldr	r3, [r0, #0x70]
 8001fea: 7155         	strb	r5, [r2, #0x5]
 8001fec: f882 c004    	strb.w	r12, [r2, #0x4]
 8001ff0: 1c5a         	adds	r2, r3, #0x1
 8001ff2: 6702         	str	r2, [r0, #0x70]
 8001ff4: f100 0220    	add.w	r2, r0, #0x20
 8001ff8: f880 6091    	strb.w	r6, [r0, #0x91]
 8001ffc: 6786         	str	r6, [r0, #0x78]
 8001ffe: 6006         	str	r6, [r0]
 8002000: 67c6         	str	r6, [r0, #0x7c]
 8002002: e9c0 6601    	strd	r6, r6, [r0, #4]
 8002006: e9c0 6603    	strd	r6, r6, [r0, #12]
 800200a: e9c0 6605    	strd	r6, r6, [r0, #20]
 800200e: 8386         	strh	r6, [r0, #0x1c]
 8002010: e9c2 6600    	strd	r6, r6, [r2]
 8002014: e9c2 6602    	strd	r6, r6, [r2, #8]
 8002018: e9c2 6604    	strd	r6, r6, [r2, #16]
 800201c: 6196         	str	r6, [r2, #0x18]
 800201e: 8396         	strh	r6, [r2, #0x1c]
 8002020: 6f02         	ldr	r2, [r0, #0x70]
 8002022: 3901         	subs	r1, #0x1
 8002024: d07e         	beq	0x8002124 <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x6d2> @ imm = #0xfc
 8002026: 2a06         	cmp	r2, #0x6
 8002028: f4ff adfd    	blo.w	0x8001c26 <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x1d4> @ imm = #-0x406
 800202c: e07a         	b	0x8002124 <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x6d2> @ imm = #0xf4
 800202e: 2d01         	cmp	r5, #0x1
 8002030: f04f 0201    	mov.w	r2, #0x1
 8002034: bf88         	it	hi
 8002036: 462a         	movhi	r2, r5
 8002038: f002 0603    	and	r6, r2, #0x3
 800203c: 2d04         	cmp	r5, #0x4
 800203e: 9601         	str	r6, [sp, #0x4]
 8002040: d204         	bhs	0x800204c <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x5fa> @ imm = #0x8
 8002042: f04f 0e00    	mov.w	lr, #0x0
 8002046: 4656         	mov	r6, r10
 8002048: 9a01         	ldr	r2, [sp, #0x4]
 800204a: e041         	b	0x80020d0 <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x67e> @ imm = #0x82
 800204c: f002 02fc    	and	r2, r2, #0xfc
 8002050: 9203         	str	r2, [sp, #0xc]
 8002052: 2200         	movs	r2, #0x0
 8002054: f04f 0e00    	mov.w	lr, #0x0
 8002058: 4656         	mov	r6, r10
 800205a: f8cd 9000    	str.w	r9, [sp]
 800205e: 9205         	str	r2, [sp, #0x14]
 8002060: 1b9d         	subs	r5, r3, r6
 8002062: ea4f 098e    	lsl.w	r9, lr, #0x2
 8002066: 1b75         	subs	r5, r6, r5
 8002068: f109 0902    	add.w	r9, r9, #0x2
 800206c: bfb8         	it	lt
 800206e: 0075         	lsllt	r5, r6, #0x1
 8002070: eba3 0205    	sub.w	r2, r3, r5
 8002074: ea4f 0645    	lsl.w	r6, r5, #0x1
 8002078: bfb8         	it	lt
 800207a: ea4f 098e    	lsllt.w	r9, lr, #0x2
 800207e: 4295         	cmp	r5, r2
 8002080: bfa8         	it	ge
 8002082: 1aae         	subge	r6, r5, r2
 8002084: eba3 0206    	sub.w	r2, r3, r6
 8002088: bfa8         	it	ge
 800208a: f109 0901    	addge.w	r9, r9, #0x1
 800208e: 1ab5         	subs	r5, r6, r2
 8002090: 4296         	cmp	r6, r2
 8002092: bfb8         	it	lt
 8002094: 0075         	lsllt	r5, r6, #0x1
 8002096: eba3 0205    	sub.w	r2, r3, r5
 800209a: 9204         	str	r2, [sp, #0x10]
 800209c: ea4f 0289    	lsl.w	r2, r9, #0x2
 80020a0: f102 0e02    	add.w	lr, r2, #0x2
 80020a4: ea4f 0645    	lsl.w	r6, r5, #0x1
 80020a8: bfb8         	it	lt
 80020aa: ea4f 0e89    	lsllt.w	lr, r9, #0x2
 80020ae: 9a04         	ldr	r2, [sp, #0x10]
 80020b0: 4295         	cmp	r5, r2
 80020b2: bfa4         	itt	ge
 80020b4: 1aae         	subge	r6, r5, r2
 80020b6: f10e 0e01    	addge.w	lr, lr, #0x1
 80020ba: 9a05         	ldr	r2, [sp, #0x14]
 80020bc: 3204         	adds	r2, #0x4
 80020be: 9205         	str	r2, [sp, #0x14]
 80020c0: b2d5         	uxtb	r5, r2
 80020c2: 9a03         	ldr	r2, [sp, #0xc]
 80020c4: 4295         	cmp	r5, r2
 80020c6: d1cb         	bne	0x8002060 <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x60e> @ imm = #-0x6a
 80020c8: 9a01         	ldr	r2, [sp, #0x4]
 80020ca: f8dd 9000    	ldr.w	r9, [sp]
 80020ce: b1fa         	cbz	r2, 0x8002110 <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x6be> @ imm = #0x3e
 80020d0: 1b9d         	subs	r5, r3, r6
 80020d2: ea4f 0e4e    	lsl.w	lr, lr, #0x1
 80020d6: 1b75         	subs	r5, r6, r5
 80020d8: bfb8         	it	lt
 80020da: 0075         	lsllt	r5, r6, #0x1
 80020dc: bfa8         	it	ge
 80020de: f10e 0e01    	addge.w	lr, lr, #0x1
 80020e2: 2a01         	cmp	r2, #0x1
 80020e4: d013         	beq	0x800210e <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x6bc> @ imm = #0x26
 80020e6: 1b5e         	subs	r6, r3, r5
 80020e8: ea4f 0e4e    	lsl.w	lr, lr, #0x1
 80020ec: 1bae         	subs	r6, r5, r6
 80020ee: bfb8         	it	lt
 80020f0: 006e         	lsllt	r6, r5, #0x1
 80020f2: bfa8         	it	ge
 80020f4: f10e 0e01    	addge.w	lr, lr, #0x1
 80020f8: 2a02         	cmp	r2, #0x2
 80020fa: d009         	beq	0x8002110 <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x6be> @ imm = #0x12
 80020fc: 1b9d         	subs	r5, r3, r6
 80020fe: ea4f 0e4e    	lsl.w	lr, lr, #0x1
 8002102: 1b75         	subs	r5, r6, r5
 8002104: bfb8         	it	lt
 8002106: 0075         	lsllt	r5, r6, #0x1
 8002108: bfa8         	it	ge
 800210a: f10e 0e01    	addge.w	lr, lr, #0x1
 800210e: 462e         	mov	r6, r5
 8002110: ebb6 050a    	subs.w	r5, r6, r10
 8002114: bfb8         	it	lt
 8002116: 441d         	addlt	r5, r3
 8002118: eba3 0305    	sub.w	r3, r3, r5
 800211c: bfb8         	it	lt
 800211e: f1ae 0e01    	sublt.w	lr, lr, #0x1
 8002122: e5fb         	b	0x8001d1c <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x2ca> @ imm = #-0x40a
 8002124: 1f90         	subs	r0, r2, #0x6
 8002126: fab0 f080    	clz	r0, r0
 800212a: 2200         	movs	r2, #0x0
 800212c: ea4f 1950    	lsr.w	r9, r0, #0x5
 8002130: 4610         	mov	r0, r2
 8002132: 4649         	mov	r1, r9
 8002134: b007         	add	sp, #0x1c
 8002136: e8bd 0f00    	pop.w	{r8, r9, r10, r11}
 800213a: bdf0         	pop	{r4, r5, r6, r7, pc}
 800213c: 2201         	movs	r2, #0x1
 800213e: e7f7         	b	0x8002130 <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x6de> @ imm = #-0x12
 8002140: f04f 0901    	mov.w	r9, #0x1
 8002144: e7f4         	b	0x8002130 <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3+0x6de> @ imm = #-0x18
 8002146: f64d 5274    	movw	r2, #0xdd74
 800214a: f6c0 0200    	movt	r2, #0x800
 800214e: 4618         	mov	r0, r3
 8002150: 210d         	movs	r1, #0xd
 8002152: f009 fb7f    	bl	0x800b854 <core::panicking::panic_bounds_check> @ imm = #0x96fe
 8002156: f64d 5284    	movw	r2, #0xdd84
 800215a: f6c0 0200    	movt	r2, #0x800
 800215e: 4618         	mov	r0, r3
 8002160: 210d         	movs	r1, #0xd
 8002162: f009 fb77    	bl	0x800b854 <core::panicking::panic_bounds_check> @ imm = #0x96ee
 8002166: f64d 0288    	movw	r2, #0xd888
 800216a: 4618         	mov	r0, r3
 800216c: f6c0 0200    	movt	r2, #0x800
 8002170: f009 fb70    	bl	0x800b854 <core::panicking::panic_bounds_check> @ imm = #0x96e0
 8002174: f64d 0298    	movw	r2, #0xd898
 8002178: 4618         	mov	r0, r3
 800217a: f6c0 0200    	movt	r2, #0x800
 800217e: f009 fb69    	bl	0x800b854 <core::panicking::panic_bounds_check> @ imm = #0x96d2
 8002182: f64d 52b4    	movw	r2, #0xddb4
 8002186: f6c0 0200    	movt	r2, #0x800
 800218a: 4660         	mov	r0, r12
 800218c: 4619         	mov	r1, r3
 800218e: f009 fb61    	bl	0x800b854 <core::panicking::panic_bounds_check> @ imm = #0x96c2
 8002192: f64d 52c4    	movw	r2, #0xddc4
 8002196: f6c0 0200    	movt	r2, #0x800
 800219a: 4660         	mov	r0, r12
 800219c: 4619         	mov	r1, r3
 800219e: f009 fb59    	bl	0x800b854 <core::panicking::panic_bounds_check> @ imm = #0x96b2
 80021a2: f64d 52a4    	movw	r2, #0xdda4
 80021a6: 4670         	mov	r0, lr
 80021a8: f6c0 0200    	movt	r2, #0x800
 80021ac: 4619         	mov	r1, r3
 80021ae: f009 fb51    	bl	0x800b854 <core::panicking::panic_bounds_check> @ imm = #0x96a2
 80021b2: f64d 02a8    	movw	r2, #0xd8a8
 80021b6: f04f 30ff    	mov.w	r0, #0xffffffff
 80021ba: f6c0 0200    	movt	r2, #0x800
 80021be: 2105         	movs	r1, #0x5
 80021c0: f009 fb48    	bl	0x800b854 <core::panicking::panic_bounds_check> @ imm = #0x9690
 80021c4: f64d 5294    	movw	r2, #0xdd94
 80021c8: 4618         	mov	r0, r3
 80021ca: f6c0 0200    	movt	r2, #0x800
 80021ce: 2105         	movs	r1, #0x5
 80021d0: f009 fb40    	bl	0x800b854 <core::panicking::panic_bounds_check> @ imm = #0x9680

080021d4 <raylar_crcl_encoder_service::benchmark::shift_only::h394b3429a2171dca>:
 80021d4: b5f0         	push	{r4, r5, r6, r7, lr}
 80021d6: af03         	add	r7, sp, #0xc
 80021d8: e92d 0b00    	push.w	{r8, r9, r11}
 80021dc: b082         	sub	sp, #0x8
 80021de: 6801         	ldr	r1, [r0]
 80021e0: f44f 6380    	mov.w	r3, #0x400
 80021e4: 9101         	str	r1, [sp, #0x4]
 80021e6: a901         	add	r1, sp, #0x4
 80021e8: f645 716c    	movw	r1, #0x5f6c
 80021ec: 9a01         	ldr	r2, [sp, #0x4]
 80021ee: 4401         	add	r1, r0
 80021f0: f002 021f    	and	r2, r2, #0x1f
 80021f4: f8d1 60c0    	ldr.w	r6, [r1, #0xc0]
 80021f8: 3b10         	subs	r3, #0x10
 80021fa: f8d1 50d8    	ldr.w	r5, [r1, #0xd8]
 80021fe: f8d1 40f0    	ldr.w	r4, [r1, #0xf0]
 8002202: fa46 f602    	asr.w	r6, r6, r2
 8002206: f8d1 c108    	ldr.w	r12, [r1, #0x108]
 800220a: f8c1 60c0    	str.w	r6, [r1, #0xc0]
 800220e: fa45 f502    	asr.w	r5, r5, r2
 8002212: f8d1 6150    	ldr.w	r6, [r1, #0x150]
 8002216: fa44 f402    	asr.w	r4, r4, r2
 800221a: f8c1 50d8    	str.w	r5, [r1, #0xd8]
 800221e: f8d1 5168    	ldr.w	r5, [r1, #0x168]
 8002222: f8d1 9180    	ldr.w	r9, [r1, #0x180]
 8002226: fa46 f602    	asr.w	r6, r6, r2
 800222a: f8c1 40f0    	str.w	r4, [r1, #0xf0]
 800222e: fa4c f402    	asr.w	r4, r12, r2
 8002232: f8d1 c198    	ldr.w	r12, [r1, #0x198]
 8002236: f8c1 6150    	str.w	r6, [r1, #0x150]
 800223a: fa45 f602    	asr.w	r6, r5, r2
 800223e: f8c1 6168    	str.w	r6, [r1, #0x168]
 8002242: fa49 f602    	asr.w	r6, r9, r2
 8002246: f8c1 6180    	str.w	r6, [r1, #0x180]
 800224a: fa4c f602    	asr.w	r6, r12, r2
 800224e: f8c1 6198    	str.w	r6, [r1, #0x198]
 8002252: f8d1 61b0    	ldr.w	r6, [r1, #0x1b0]
 8002256: f8d1 e120    	ldr.w	lr, [r1, #0x120]
 800225a: f8d1 8138    	ldr.w	r8, [r1, #0x138]
 800225e: fa46 f602    	asr.w	r6, r6, r2
 8002262: f8c1 4108    	str.w	r4, [r1, #0x108]
 8002266: f8c1 61b0    	str.w	r6, [r1, #0x1b0]
 800226a: fa4e f402    	asr.w	r4, lr, r2
 800226e: f8d1 61c8    	ldr.w	r6, [r1, #0x1c8]
 8002272: f8c1 4120    	str.w	r4, [r1, #0x120]
 8002276: fa48 f402    	asr.w	r4, r8, r2
 800227a: f8c1 4138    	str.w	r4, [r1, #0x138]
 800227e: fa46 f602    	asr.w	r6, r6, r2
 8002282: f8c1 61c8    	str.w	r6, [r1, #0x1c8]
 8002286: f8d1 61e0    	ldr.w	r6, [r1, #0x1e0]
 800228a: fa46 f602    	asr.w	r6, r6, r2
 800228e: f8c1 61e0    	str.w	r6, [r1, #0x1e0]
 8002292: f8d1 61f8    	ldr.w	r6, [r1, #0x1f8]
 8002296: fa46 f602    	asr.w	r6, r6, r2
 800229a: f8c1 61f8    	str.w	r6, [r1, #0x1f8]
 800229e: f8d1 6210    	ldr.w	r6, [r1, #0x210]
 80022a2: fa46 f602    	asr.w	r6, r6, r2
 80022a6: f8c1 6210    	str.w	r6, [r1, #0x210]
 80022aa: f8d1 6228    	ldr.w	r6, [r1, #0x228]
 80022ae: fa46 f602    	asr.w	r6, r6, r2
 80022b2: f8c1 6228    	str.w	r6, [r1, #0x228]
 80022b6: f501 71c0    	add.w	r1, r1, #0x180
 80022ba: d19b         	bne	0x80021f4 <raylar_crcl_encoder_service::benchmark::shift_only::h394b3429a2171dca+0x20> @ imm = #-0xca
 80022bc: 6881         	ldr	r1, [r0, #0x8]
 80022be: f44f 6380    	mov.w	r3, #0x400
 80022c2: 9101         	str	r1, [sp, #0x4]
 80022c4: a901         	add	r1, sp, #0x4
 80022c6: f645 61b0    	movw	r1, #0x5eb0
 80022ca: 9a01         	ldr	r2, [sp, #0x4]
 80022cc: 4401         	add	r1, r0
 80022ce: f002 021f    	and	r2, r2, #0x1f
 80022d2: f8d1 c180    	ldr.w	r12, [r1, #0x180]
 80022d6: 3b10         	subs	r3, #0x10
 80022d8: f8d1 e198    	ldr.w	lr, [r1, #0x198]
 80022dc: f8d1 41b0    	ldr.w	r4, [r1, #0x1b0]
 80022e0: fa4c f602    	asr.w	r6, r12, r2
 80022e4: f8d1 51c8    	ldr.w	r5, [r1, #0x1c8]
 80022e8: f8c1 6180    	str.w	r6, [r1, #0x180]
 80022ec: fa4e f602    	asr.w	r6, lr, r2
 80022f0: f8c1 6198    	str.w	r6, [r1, #0x198]
 80022f4: fa44 f402    	asr.w	r4, r4, r2
 80022f8: f8d1 6228    	ldr.w	r6, [r1, #0x228]
 80022fc: f8d1 9240    	ldr.w	r9, [r1, #0x240]
 8002300: f8c1 41b0    	str.w	r4, [r1, #0x1b0]
 8002304: fa45 f402    	asr.w	r4, r5, r2
 8002308: f8d1 5258    	ldr.w	r5, [r1, #0x258]
 800230c: fa46 f602    	asr.w	r6, r6, r2
 8002310: f8c1 6228    	str.w	r6, [r1, #0x228]
 8002314: fa49 f602    	asr.w	r6, r9, r2
 8002318: f8c1 6240    	str.w	r6, [r1, #0x240]
 800231c: fa45 f602    	asr.w	r6, r5, r2
 8002320: f8d1 81e0    	ldr.w	r8, [r1, #0x1e0]
 8002324: f8c1 6258    	str.w	r6, [r1, #0x258]
 8002328: f8d1 6270    	ldr.w	r6, [r1, #0x270]
 800232c: f8d1 c1f8    	ldr.w	r12, [r1, #0x1f8]
 8002330: f8d1 e210    	ldr.w	lr, [r1, #0x210]
 8002334: fa46 f602    	asr.w	r6, r6, r2
 8002338: f8c1 41c8    	str.w	r4, [r1, #0x1c8]
 800233c: f8c1 6270    	str.w	r6, [r1, #0x270]
 8002340: fa48 f402    	asr.w	r4, r8, r2
 8002344: f8d1 6288    	ldr.w	r6, [r1, #0x288]
 8002348: f8c1 41e0    	str.w	r4, [r1, #0x1e0]
 800234c: fa4c f402    	asr.w	r4, r12, r2
 8002350: f8c1 41f8    	str.w	r4, [r1, #0x1f8]
 8002354: fa4e f402    	asr.w	r4, lr, r2
 8002358: fa46 f602    	asr.w	r6, r6, r2
 800235c: f8c1 4210    	str.w	r4, [r1, #0x210]
 8002360: f8c1 6288    	str.w	r6, [r1, #0x288]
 8002364: f8d1 62a0    	ldr.w	r6, [r1, #0x2a0]
 8002368: fa46 f602    	asr.w	r6, r6, r2
 800236c: f8c1 62a0    	str.w	r6, [r1, #0x2a0]
 8002370: f8d1 62b8    	ldr.w	r6, [r1, #0x2b8]
 8002374: fa46 f602    	asr.w	r6, r6, r2
 8002378: f8c1 62b8    	str.w	r6, [r1, #0x2b8]
 800237c: f8d1 62d0    	ldr.w	r6, [r1, #0x2d0]
 8002380: fa46 f602    	asr.w	r6, r6, r2
 8002384: f8c1 62d0    	str.w	r6, [r1, #0x2d0]
 8002388: f8d1 62e8    	ldr.w	r6, [r1, #0x2e8]
 800238c: fa46 f602    	asr.w	r6, r6, r2
 8002390: f8c1 62e8    	str.w	r6, [r1, #0x2e8]
 8002394: f501 71c0    	add.w	r1, r1, #0x180
 8002398: d19b         	bne	0x80022d2 <raylar_crcl_encoder_service::benchmark::shift_only::h394b3429a2171dca+0xfe> @ imm = #-0xca
 800239a: 6901         	ldr	r1, [r0, #0x10]
 800239c: f44f 6380    	mov.w	r3, #0x400
 80023a0: 9101         	str	r1, [sp, #0x4]
 80023a2: a901         	add	r1, sp, #0x4
 80023a4: f645 7174    	movw	r1, #0x5f74
 80023a8: 9a01         	ldr	r2, [sp, #0x4]
 80023aa: 4401         	add	r1, r0
 80023ac: f002 021f    	and	r2, r2, #0x1f
 80023b0: f8d1 60c0    	ldr.w	r6, [r1, #0xc0]
 80023b4: 3b10         	subs	r3, #0x10
 80023b6: f8d1 50d8    	ldr.w	r5, [r1, #0xd8]
 80023ba: f8d1 40f0    	ldr.w	r4, [r1, #0xf0]
 80023be: fa46 f602    	asr.w	r6, r6, r2
 80023c2: f8d1 c108    	ldr.w	r12, [r1, #0x108]
 80023c6: f8c1 60c0    	str.w	r6, [r1, #0xc0]
 80023ca: fa45 f502    	asr.w	r5, r5, r2
 80023ce: f8d1 6150    	ldr.w	r6, [r1, #0x150]
 80023d2: fa44 f402    	asr.w	r4, r4, r2
 80023d6: f8c1 50d8    	str.w	r5, [r1, #0xd8]
 80023da: f8d1 5168    	ldr.w	r5, [r1, #0x168]
 80023de: f8d1 9180    	ldr.w	r9, [r1, #0x180]
 80023e2: fa46 f602    	asr.w	r6, r6, r2
 80023e6: f8c1 40f0    	str.w	r4, [r1, #0xf0]
 80023ea: fa4c f402    	asr.w	r4, r12, r2
 80023ee: f8d1 c198    	ldr.w	r12, [r1, #0x198]
 80023f2: f8c1 6150    	str.w	r6, [r1, #0x150]
 80023f6: fa45 f602    	asr.w	r6, r5, r2
 80023fa: f8c1 6168    	str.w	r6, [r1, #0x168]
 80023fe: fa49 f602    	asr.w	r6, r9, r2
 8002402: f8c1 6180    	str.w	r6, [r1, #0x180]
 8002406: fa4c f602    	asr.w	r6, r12, r2
 800240a: f8c1 6198    	str.w	r6, [r1, #0x198]
 800240e: f8d1 61b0    	ldr.w	r6, [r1, #0x1b0]
 8002412: f8d1 e120    	ldr.w	lr, [r1, #0x120]
 8002416: f8d1 8138    	ldr.w	r8, [r1, #0x138]
 800241a: fa46 f602    	asr.w	r6, r6, r2
 800241e: f8c1 4108    	str.w	r4, [r1, #0x108]
 8002422: f8c1 61b0    	str.w	r6, [r1, #0x1b0]
 8002426: fa4e f402    	asr.w	r4, lr, r2
 800242a: f8d1 61c8    	ldr.w	r6, [r1, #0x1c8]
 800242e: f8c1 4120    	str.w	r4, [r1, #0x120]
 8002432: fa48 f402    	asr.w	r4, r8, r2
 8002436: f8c1 4138    	str.w	r4, [r1, #0x138]
 800243a: fa46 f602    	asr.w	r6, r6, r2
 800243e: f8c1 61c8    	str.w	r6, [r1, #0x1c8]
 8002442: f8d1 61e0    	ldr.w	r6, [r1, #0x1e0]
 8002446: fa46 f602    	asr.w	r6, r6, r2
 800244a: f8c1 61e0    	str.w	r6, [r1, #0x1e0]
 800244e: f8d1 61f8    	ldr.w	r6, [r1, #0x1f8]
 8002452: fa46 f602    	asr.w	r6, r6, r2
 8002456: f8c1 61f8    	str.w	r6, [r1, #0x1f8]
 800245a: f8d1 6210    	ldr.w	r6, [r1, #0x210]
 800245e: fa46 f602    	asr.w	r6, r6, r2
 8002462: f8c1 6210    	str.w	r6, [r1, #0x210]
 8002466: f8d1 6228    	ldr.w	r6, [r1, #0x228]
 800246a: fa46 f602    	asr.w	r6, r6, r2
 800246e: f8c1 6228    	str.w	r6, [r1, #0x228]
 8002472: f501 71c0    	add.w	r1, r1, #0x180
 8002476: d19b         	bne	0x80023b0 <raylar_crcl_encoder_service::benchmark::shift_only::h394b3429a2171dca+0x1dc> @ imm = #-0xca
 8002478: 6981         	ldr	r1, [r0, #0x18]
 800247a: f44f 6380    	mov.w	r3, #0x400
 800247e: 9101         	str	r1, [sp, #0x4]
 8002480: a901         	add	r1, sp, #0x4
 8002482: f645 61b8    	movw	r1, #0x5eb8
 8002486: 9a01         	ldr	r2, [sp, #0x4]
 8002488: 4401         	add	r1, r0
 800248a: f002 021f    	and	r2, r2, #0x1f
 800248e: f8d1 6180    	ldr.w	r6, [r1, #0x180]
 8002492: 3b10         	subs	r3, #0x10
 8002494: f8d1 5198    	ldr.w	r5, [r1, #0x198]
 8002498: f8d1 41b0    	ldr.w	r4, [r1, #0x1b0]
 800249c: fa46 f602    	asr.w	r6, r6, r2
 80024a0: f8d1 c1c8    	ldr.w	r12, [r1, #0x1c8]
 80024a4: f8c1 6180    	str.w	r6, [r1, #0x180]
 80024a8: fa45 f502    	asr.w	r5, r5, r2
 80024ac: f8d1 6210    	ldr.w	r6, [r1, #0x210]
 80024b0: fa44 f402    	asr.w	r4, r4, r2
 80024b4: f8c1 5198    	str.w	r5, [r1, #0x198]
 80024b8: f8d1 5228    	ldr.w	r5, [r1, #0x228]
 80024bc: f8d1 9240    	ldr.w	r9, [r1, #0x240]
 80024c0: fa46 f602    	asr.w	r6, r6, r2
 80024c4: f8c1 41b0    	str.w	r4, [r1, #0x1b0]
 80024c8: fa4c f402    	asr.w	r4, r12, r2
 80024cc: f8d1 c258    	ldr.w	r12, [r1, #0x258]
 80024d0: f8c1 6210    	str.w	r6, [r1, #0x210]
 80024d4: fa45 f602    	asr.w	r6, r5, r2
 80024d8: f8c1 6228    	str.w	r6, [r1, #0x228]
 80024dc: fa49 f602    	asr.w	r6, r9, r2
 80024e0: f8c1 6240    	str.w	r6, [r1, #0x240]
 80024e4: fa4c f602    	asr.w	r6, r12, r2
 80024e8: f8c1 6258    	str.w	r6, [r1, #0x258]
 80024ec: f8d1 6270    	ldr.w	r6, [r1, #0x270]
 80024f0: f8d1 e1e0    	ldr.w	lr, [r1, #0x1e0]
 80024f4: f8d1 81f8    	ldr.w	r8, [r1, #0x1f8]
 80024f8: fa46 f602    	asr.w	r6, r6, r2
 80024fc: f8c1 41c8    	str.w	r4, [r1, #0x1c8]
 8002500: f8c1 6270    	str.w	r6, [r1, #0x270]
 8002504: fa4e f402    	asr.w	r4, lr, r2
 8002508: f8d1 6288    	ldr.w	r6, [r1, #0x288]
 800250c: f8c1 41e0    	str.w	r4, [r1, #0x1e0]
 8002510: fa48 f402    	asr.w	r4, r8, r2
 8002514: f8c1 41f8    	str.w	r4, [r1, #0x1f8]
 8002518: fa46 f602    	asr.w	r6, r6, r2
 800251c: f8c1 6288    	str.w	r6, [r1, #0x288]
 8002520: f8d1 62a0    	ldr.w	r6, [r1, #0x2a0]
 8002524: fa46 f602    	asr.w	r6, r6, r2
 8002528: f8c1 62a0    	str.w	r6, [r1, #0x2a0]
 800252c: f8d1 62b8    	ldr.w	r6, [r1, #0x2b8]
 8002530: fa46 f602    	asr.w	r6, r6, r2
 8002534: f8c1 62b8    	str.w	r6, [r1, #0x2b8]
 8002538: f8d1 62d0    	ldr.w	r6, [r1, #0x2d0]
 800253c: fa46 f602    	asr.w	r6, r6, r2
 8002540: f8c1 62d0    	str.w	r6, [r1, #0x2d0]
 8002544: f8d1 62e8    	ldr.w	r6, [r1, #0x2e8]
 8002548: fa46 f602    	asr.w	r6, r6, r2
 800254c: f8c1 62e8    	str.w	r6, [r1, #0x2e8]
 8002550: f501 71c0    	add.w	r1, r1, #0x180
 8002554: d19b         	bne	0x800248e <raylar_crcl_encoder_service::benchmark::shift_only::h394b3429a2171dca+0x2ba> @ imm = #-0xca
 8002556: 6a01         	ldr	r1, [r0, #0x20]
 8002558: f44f 6380    	mov.w	r3, #0x400
 800255c: 9101         	str	r1, [sp, #0x4]
 800255e: a901         	add	r1, sp, #0x4
 8002560: f645 717c    	movw	r1, #0x5f7c
 8002564: 9a01         	ldr	r2, [sp, #0x4]
 8002566: 4401         	add	r1, r0
 8002568: f002 021f    	and	r2, r2, #0x1f
 800256c: f8d1 60c0    	ldr.w	r6, [r1, #0xc0]
 8002570: 3b10         	subs	r3, #0x10
 8002572: f8d1 50d8    	ldr.w	r5, [r1, #0xd8]
 8002576: f8d1 40f0    	ldr.w	r4, [r1, #0xf0]
 800257a: fa46 f602    	asr.w	r6, r6, r2
 800257e: f8d1 c108    	ldr.w	r12, [r1, #0x108]
 8002582: f8c1 60c0    	str.w	r6, [r1, #0xc0]
 8002586: fa45 f502    	asr.w	r5, r5, r2
 800258a: f8d1 6150    	ldr.w	r6, [r1, #0x150]
 800258e: fa44 f402    	asr.w	r4, r4, r2
 8002592: f8c1 50d8    	str.w	r5, [r1, #0xd8]
 8002596: f8d1 5168    	ldr.w	r5, [r1, #0x168]
 800259a: f8d1 9180    	ldr.w	r9, [r1, #0x180]
 800259e: fa46 f602    	asr.w	r6, r6, r2
 80025a2: f8c1 40f0    	str.w	r4, [r1, #0xf0]
 80025a6: fa4c f402    	asr.w	r4, r12, r2
 80025aa: f8d1 c198    	ldr.w	r12, [r1, #0x198]
 80025ae: f8c1 6150    	str.w	r6, [r1, #0x150]
 80025b2: fa45 f602    	asr.w	r6, r5, r2
 80025b6: f8c1 6168    	str.w	r6, [r1, #0x168]
 80025ba: fa49 f602    	asr.w	r6, r9, r2
 80025be: f8c1 6180    	str.w	r6, [r1, #0x180]
 80025c2: fa4c f602    	asr.w	r6, r12, r2
 80025c6: f8c1 6198    	str.w	r6, [r1, #0x198]
 80025ca: f8d1 61b0    	ldr.w	r6, [r1, #0x1b0]
 80025ce: f8d1 e120    	ldr.w	lr, [r1, #0x120]
 80025d2: f8d1 8138    	ldr.w	r8, [r1, #0x138]
 80025d6: fa46 f602    	asr.w	r6, r6, r2
 80025da: f8c1 4108    	str.w	r4, [r1, #0x108]
 80025de: f8c1 61b0    	str.w	r6, [r1, #0x1b0]
 80025e2: fa4e f402    	asr.w	r4, lr, r2
 80025e6: f8d1 61c8    	ldr.w	r6, [r1, #0x1c8]
 80025ea: f8c1 4120    	str.w	r4, [r1, #0x120]
 80025ee: fa48 f402    	asr.w	r4, r8, r2
 80025f2: f8c1 4138    	str.w	r4, [r1, #0x138]
 80025f6: fa46 f602    	asr.w	r6, r6, r2
 80025fa: f8c1 61c8    	str.w	r6, [r1, #0x1c8]
 80025fe: f8d1 61e0    	ldr.w	r6, [r1, #0x1e0]
 8002602: fa46 f602    	asr.w	r6, r6, r2
 8002606: f8c1 61e0    	str.w	r6, [r1, #0x1e0]
 800260a: f8d1 61f8    	ldr.w	r6, [r1, #0x1f8]
 800260e: fa46 f602    	asr.w	r6, r6, r2
 8002612: f8c1 61f8    	str.w	r6, [r1, #0x1f8]
 8002616: f8d1 6210    	ldr.w	r6, [r1, #0x210]
 800261a: fa46 f602    	asr.w	r6, r6, r2
 800261e: f8c1 6210    	str.w	r6, [r1, #0x210]
 8002622: f8d1 6228    	ldr.w	r6, [r1, #0x228]
 8002626: fa46 f602    	asr.w	r6, r6, r2
 800262a: f8c1 6228    	str.w	r6, [r1, #0x228]
 800262e: f501 71c0    	add.w	r1, r1, #0x180
 8002632: d19b         	bne	0x800256c <raylar_crcl_encoder_service::benchmark::shift_only::h394b3429a2171dca+0x398> @ imm = #-0xca
 8002634: f246 0128    	movw	r1, #0x6028
 8002638: 4401         	add	r1, r0
 800263a: 9100         	str	r1, [sp]
 800263c: 4669         	mov	r1, sp
 800263e: f246 012c    	movw	r1, #0x602c
 8002642: 5840         	ldr	r0, [r0, r1]
 8002644: b002         	add	sp, #0x8
 8002646: e8bd 0b00    	pop.w	{r8, r9, r11}
 800264a: bdf0         	pop	{r4, r5, r6, r7, pc}

0800264c <raylar_crcl_encoder_service::benchmark::encode_bits::h865b9e06648afa8c>:
 800264c: b5f0         	push	{r4, r5, r6, r7, lr}
 800264e: af03         	add	r7, sp, #0xc
 8002650: e92d 0f00    	push.w	{r8, r9, r10, r11}
 8002654: b097         	sub	sp, #0x5c
 8002656: e9cd 1210    	strd	r1, r2, [sp, #64]
 800265a: 4601         	mov	r1, r0
 800265c: f500 30b8    	add.w	r0, r0, #0x17000
 8002660: f501 33e0    	add.w	r3, r1, #0x1c000
 8002664: 3010         	adds	r0, #0x10
 8002666: 900e         	str	r0, [sp, #0x38]
 8002668: f24c 0028    	movw	r0, #0xc028
 800266c: 180a         	adds	r2, r1, r0
 800266e: 2058         	movs	r0, #0x58
 8002670: f641 6658    	movw	r6, #0x1e58
 8002674: f641 6556    	movw	r5, #0x1e56
 8002678: f04f 3cff    	mov.w	r12, #0xffffffff
 800267c: f04f 0a00    	mov.w	r10, #0x0
 8002680: 9012         	str	r0, [sp, #0x48]
 8002682: 9812         	ldr	r0, [sp, #0x48]
 8002684: 42b0         	cmp	r0, r6
 8002686: f200 81d9    	bhi.w	0x8002a3c <raylar_crcl_encoder_service::benchmark::encode_bits::h865b9e06648afa8c+0x3f0> @ imm = #0x3b2
 800268a: 9812         	ldr	r0, [sp, #0x48]
 800268c: 42a8         	cmp	r0, r5
 800268e: f200 8191    	bhi.w	0x80029b4 <raylar_crcl_encoder_service::benchmark::encode_bits::h865b9e06648afa8c+0x368> @ imm = #0x322
 8002692: eb01 00ca    	add.w	r0, r1, r10, lsl #3
 8002696: 920b         	str	r2, [sp, #0x2c]
 8002698: f04f 0800    	mov.w	r8, #0x0
 800269c: 9104         	str	r1, [sp, #0x10]
 800269e: 7900         	ldrb	r0, [r0, #0x4]
 80026a0: 900d         	str	r0, [sp, #0x34]
 80026a2: f10a 0001    	add.w	r0, r10, #0x1
 80026a6: 9003         	str	r0, [sp, #0xc]
 80026a8: f852 2020    	ldr.w	r2, [r2, r0, lsl #2]
 80026ac: 9812         	ldr	r0, [sp, #0x48]
 80026ae: eba6 0e00    	sub.w	lr, r6, r0
 80026b2: eb03 0900    	add.w	r9, r3, r0
 80026b6: 260a         	movs	r6, #0xa
 80026b8: f018 0407    	ands	r4, r8, #0x7
 80026bc: ea4f 03d8    	lsr.w	r3, r8, #0x3
 80026c0: d105         	bne	0x80026ce <raylar_crcl_encoder_service::benchmark::encode_bits::h865b9e06648afa8c+0x82> @ imm = #0xa
 80026c2: 4573         	cmp	r3, lr
 80026c4: f080 81cd    	bhs.w	0x8002a62 <raylar_crcl_encoder_service::benchmark::encode_bits::h865b9e06648afa8c+0x416> @ imm = #0x39a
 80026c8: 2000         	movs	r0, #0x0
 80026ca: f809 0003    	strb.w	r0, [r9, r3]
 80026ce: 4573         	cmp	r3, lr
 80026d0: f080 81bf    	bhs.w	0x8002a52 <raylar_crcl_encoder_service::benchmark::encode_bits::h865b9e06648afa8c+0x406> @ imm = #0x37e
 80026d4: f1c4 0008    	rsb.w	r0, r4, #0x8
 80026d8: 4634         	mov	r4, r6
 80026da: 42b0         	cmp	r0, r6
 80026dc: bf38         	it	lo
 80026de: 4604         	movlo	r4, r0
 80026e0: 1b36         	subs	r6, r6, r4
 80026e2: eba0 0004    	sub.w	r0, r0, r4
 80026e6: f006 011f    	and	r1, r6, #0x1f
 80026ea: fa0c f504    	lsl.w	r5, r12, r4
 80026ee: f819 b003    	ldrb.w	r11, [r9, r3]
 80026f2: f000 0007    	and	r0, r0, #0x7
 80026f6: fa22 f101    	lsr.w	r1, r2, r1
 80026fa: ea21 0105    	bic.w	r1, r1, r5
 80026fe: 44a0         	add	r8, r4
 8002700: fa01 f000    	lsl.w	r0, r1, r0
 8002704: ea40 000b    	orr.w	r0, r0, r11
 8002708: f809 0003    	strb.w	r0, [r9, r3]
 800270c: d1d4         	bne	0x80026b8 <raylar_crcl_encoder_service::benchmark::encode_bits::h865b9e06648afa8c+0x6c> @ imm = #-0x58
 800270e: 980b         	ldr	r0, [sp, #0x2c]
 8002710: f10a 0101    	add.w	r1, r10, #0x1
 8002714: f04f 0b01    	mov.w	r11, #0x1
 8002718: f850 3021    	ldr.w	r3, [r0, r1, lsl #2]
 800271c: eb00 008a    	add.w	r0, r0, r10, lsl #2
 8002720: 900a         	str	r0, [sp, #0x28]
 8002722: ebca 208a    	rsb	r0, r10, r10, lsl #10
 8002726: 990e         	ldr	r1, [sp, #0x38]
 8002728: eb01 0080    	add.w	r0, r1, r0, lsl #2
 800272c: 990d         	ldr	r1, [sp, #0x34]
 800272e: 9009         	str	r0, [sp, #0x24]
 8002730: f001 041f    	and	r4, r1, #0x1f
 8002734: 9408         	str	r4, [sp, #0x20]
 8002736: fa0c f004    	lsl.w	r0, r12, r4
 800273a: 43c0         	mvns	r0, r0
 800273c: 9005         	str	r0, [sp, #0x14]
 800273e: ea4f 00ce    	lsl.w	r0, lr, #0x3
 8002742: 900f         	str	r0, [sp, #0x3c]
 8002744: 1c48         	adds	r0, r1, #0x1
 8002746: 900c         	str	r0, [sp, #0x30]
 8002748: fa0b f004    	lsl.w	r0, r11, r4
 800274c: 9007         	str	r0, [sp, #0x1c]
 800274e: 3801         	subs	r0, #0x1
 8002750: 9006         	str	r0, [sp, #0x18]
 8002752: 9811         	ldr	r0, [sp, #0x44]
 8002754: b178         	cbz	r0, 0x8002776 <raylar_crcl_encoder_service::benchmark::encode_bits::h865b9e06648afa8c+0x12a> @ imm = #0x1e
 8002756: eb0b 004b    	add.w	r0, r11, r11, lsl #1
 800275a: 990a         	ldr	r1, [sp, #0x28]
 800275c: eb01 00c0    	add.w	r0, r1, r0, lsl #3
 8002760: 6842         	ldr	r2, [r0, #0x4]
 8002762: 1ad0         	subs	r0, r2, r3
 8002764: 0043         	lsls	r3, r0, #0x1
 8002766: ea83 70e0    	eor.w	r0, r3, r0, asr #31
 800276a: 4613         	mov	r3, r2
 800276c: 9910         	ldr	r1, [sp, #0x40]
 800276e: fa20 fa04    	lsr.w	r10, r0, r4
 8002772: b941         	cbnz	r1, 0x8002786 <raylar_crcl_encoder_service::benchmark::encode_bits::h865b9e06648afa8c+0x13a> @ imm = #0x10
 8002774: e04e         	b	0x8002814 <raylar_crcl_encoder_service::benchmark::encode_bits::h865b9e06648afa8c+0x1c8> @ imm = #0x9c
 8002776: 9809         	ldr	r0, [sp, #0x24]
 8002778: f850 002b    	ldr.w	r0, [r0, r11, lsl #2]
 800277c: 9910         	ldr	r1, [sp, #0x40]
 800277e: fa20 fa04    	lsr.w	r10, r0, r4
 8002782: 2900         	cmp	r1, #0x0
 8002784: d046         	beq	0x8002814 <raylar_crcl_encoder_service::benchmark::encode_bits::h865b9e06648afa8c+0x1c8> @ imm = #0x8c
 8002786: 990c         	ldr	r1, [sp, #0x30]
 8002788: eb01 060a    	add.w	r6, r1, r10
 800278c: 2e21         	cmp	r6, #0x21
 800278e: d241         	bhs	0x8002814 <raylar_crcl_encoder_service::benchmark::encode_bits::h865b9e06648afa8c+0x1c8> @ imm = #0x82
 8002790: eb16 0208    	adds.w	r2, r6, r8
 8002794: f080 8126    	bhs.w	0x80029e4 <raylar_crcl_encoder_service::benchmark::encode_bits::h865b9e06648afa8c+0x398> @ imm = #0x24c
 8002798: 990f         	ldr	r1, [sp, #0x3c]
 800279a: 428a         	cmp	r2, r1
 800279c: f200 8122    	bhi.w	0x80029e4 <raylar_crcl_encoder_service::benchmark::encode_bits::h865b9e06648afa8c+0x398> @ imm = #0x244
 80027a0: 9302         	str	r3, [sp, #0x8]
 80027a2: b37e         	cbz	r6, 0x8002804 <raylar_crcl_encoder_service::benchmark::encode_bits::h865b9e06648afa8c+0x1b8> @ imm = #0x5e
 80027a4: 9906         	ldr	r1, [sp, #0x18]
 80027a6: 4008         	ands	r0, r1
 80027a8: 9907         	ldr	r1, [sp, #0x1c]
 80027aa: ea40 0a01    	orr.w	r10, r0, r1
 80027ae: f018 0007    	ands	r0, r8, #0x7
 80027b2: ea4f 03d8    	lsr.w	r3, r8, #0x3
 80027b6: d105         	bne	0x80027c4 <raylar_crcl_encoder_service::benchmark::encode_bits::h865b9e06648afa8c+0x178> @ imm = #0xa
 80027b8: 4573         	cmp	r3, lr
 80027ba: f080 8152    	bhs.w	0x8002a62 <raylar_crcl_encoder_service::benchmark::encode_bits::h865b9e06648afa8c+0x416> @ imm = #0x2a4
 80027be: 2100         	movs	r1, #0x0
 80027c0: f809 1003    	strb.w	r1, [r9, r3]
 80027c4: 4573         	cmp	r3, lr
 80027c6: f080 8144    	bhs.w	0x8002a52 <raylar_crcl_encoder_service::benchmark::encode_bits::h865b9e06648afa8c+0x406> @ imm = #0x288
 80027ca: f1c0 0008    	rsb.w	r0, r0, #0x8
 80027ce: 4634         	mov	r4, r6
 80027d0: 42b0         	cmp	r0, r6
 80027d2: bf38         	it	lo
 80027d4: 4604         	movlo	r4, r0
 80027d6: 1b36         	subs	r6, r6, r4
 80027d8: eba0 0004    	sub.w	r0, r0, r4
 80027dc: f006 011f    	and	r1, r6, #0x1f
 80027e0: fa0c f504    	lsl.w	r5, r12, r4
 80027e4: f819 2003    	ldrb.w	r2, [r9, r3]
 80027e8: f000 0007    	and	r0, r0, #0x7
 80027ec: fa2a f101    	lsr.w	r1, r10, r1
 80027f0: ea21 0105    	bic.w	r1, r1, r5
 80027f4: 44a0         	add	r8, r4
 80027f6: fa01 f000    	lsl.w	r0, r1, r0
 80027fa: ea40 0002    	orr.w	r0, r0, r2
 80027fe: f809 0003    	strb.w	r0, [r9, r3]
 8002802: d1d4         	bne	0x80027ae <raylar_crcl_encoder_service::benchmark::encode_bits::h865b9e06648afa8c+0x162> @ imm = #-0x58
 8002804: 9b02         	ldr	r3, [sp, #0x8]
 8002806: f10b 0b01    	add.w	r11, r11, #0x1
 800280a: 9c08         	ldr	r4, [sp, #0x20]
 800280c: f5bb 6f80    	cmp.w	r11, #0x400
 8002810: d19f         	bne	0x8002752 <raylar_crcl_encoder_service::benchmark::encode_bits::h865b9e06648afa8c+0x106> @ imm = #-0xc2
 8002812: e090         	b	0x8002936 <raylar_crcl_encoder_service::benchmark::encode_bits::h865b9e06648afa8c+0x2ea> @ imm = #0x120
 8002814: 990f         	ldr	r1, [sp, #0x3c]
 8002816: eba1 0208    	sub.w	r2, r1, r8
 800281a: 4592         	cmp	r10, r2
 800281c: f080 80d6    	bhs.w	0x80029cc <raylar_crcl_encoder_service::benchmark::encode_bits::h865b9e06648afa8c+0x380> @ imm = #0x1ac
 8002820: f1ba 0f00    	cmp.w	r10, #0x0
 8002824: d028         	beq	0x8002878 <raylar_crcl_encoder_service::benchmark::encode_bits::h865b9e06648afa8c+0x22c> @ imm = #0x50
 8002826: f1ba 0f20    	cmp.w	r10, #0x20
 800282a: 4656         	mov	r6, r10
 800282c: bf28         	it	hs
 800282e: 2620         	movhs	r6, #0x20
 8002830: eb18 0206    	adds.w	r2, r8, r6
 8002834: f080 80ca    	bhs.w	0x80029cc <raylar_crcl_encoder_service::benchmark::encode_bits::h865b9e06648afa8c+0x380> @ imm = #0x194
 8002838: 990f         	ldr	r1, [sp, #0x3c]
 800283a: 428a         	cmp	r2, r1
 800283c: f200 80c6    	bhi.w	0x80029cc <raylar_crcl_encoder_service::benchmark::encode_bits::h865b9e06648afa8c+0x380> @ imm = #0x18c
 8002840: 461d         	mov	r5, r3
 8002842: 4632         	mov	r2, r6
 8002844: f018 0407    	ands	r4, r8, #0x7
 8002848: ea4f 03d8    	lsr.w	r3, r8, #0x3
 800284c: d105         	bne	0x800285a <raylar_crcl_encoder_service::benchmark::encode_bits::h865b9e06648afa8c+0x20e> @ imm = #0xa
 800284e: 4573         	cmp	r3, lr
 8002850: f080 8107    	bhs.w	0x8002a62 <raylar_crcl_encoder_service::benchmark::encode_bits::h865b9e06648afa8c+0x416> @ imm = #0x20e
 8002854: 2100         	movs	r1, #0x0
 8002856: f809 1003    	strb.w	r1, [r9, r3]
 800285a: 4573         	cmp	r3, lr
 800285c: f080 80f9    	bhs.w	0x8002a52 <raylar_crcl_encoder_service::benchmark::encode_bits::h865b9e06648afa8c+0x406> @ imm = #0x1f2
 8002860: f1c4 0308    	rsb.w	r3, r4, #0x8
 8002864: 4293         	cmp	r3, r2
 8002866: bf28         	it	hs
 8002868: 4613         	movhs	r3, r2
 800286a: 4498         	add	r8, r3
 800286c: 1ad2         	subs	r2, r2, r3
 800286e: d1e9         	bne	0x8002844 <raylar_crcl_encoder_service::benchmark::encode_bits::h865b9e06648afa8c+0x1f8> @ imm = #-0x2e
 8002870: ebba 0a06    	subs.w	r10, r10, r6
 8002874: 462b         	mov	r3, r5
 8002876: d1d6         	bne	0x8002826 <raylar_crcl_encoder_service::benchmark::encode_bits::h865b9e06648afa8c+0x1da> @ imm = #-0x54
 8002878: 990f         	ldr	r1, [sp, #0x3c]
 800287a: 4588         	cmp	r8, r1
 800287c: f080 80a6    	bhs.w	0x80029cc <raylar_crcl_encoder_service::benchmark::encode_bits::h865b9e06648afa8c+0x380> @ imm = #0x14c
 8002880: f018 0207    	ands	r2, r8, #0x7
 8002884: 9302         	str	r3, [sp, #0x8]
 8002886: ea4f 03d8    	lsr.w	r3, r8, #0x3
 800288a: d105         	bne	0x8002898 <raylar_crcl_encoder_service::benchmark::encode_bits::h865b9e06648afa8c+0x24c> @ imm = #0xa
 800288c: 4573         	cmp	r3, lr
 800288e: f080 80e8    	bhs.w	0x8002a62 <raylar_crcl_encoder_service::benchmark::encode_bits::h865b9e06648afa8c+0x416> @ imm = #0x1d0
 8002892: 2100         	movs	r1, #0x0
 8002894: f809 1003    	strb.w	r1, [r9, r3]
 8002898: 4573         	cmp	r3, lr
 800289a: f080 80da    	bhs.w	0x8002a52 <raylar_crcl_encoder_service::benchmark::encode_bits::h865b9e06648afa8c+0x406> @ imm = #0x1b4
 800289e: f819 1003    	ldrb.w	r1, [r9, r3]
 80028a2: f082 0207    	eor	r2, r2, #0x7
 80028a6: 2401         	movs	r4, #0x1
 80028a8: fa04 f202    	lsl.w	r2, r4, r2
 80028ac: 4311         	orrs	r1, r2
 80028ae: f809 1003    	strb.w	r1, [r9, r3]
 80028b2: 990d         	ldr	r1, [sp, #0x34]
 80028b4: 2920         	cmp	r1, #0x20
 80028b6: f200 80a9    	bhi.w	0x8002a0c <raylar_crcl_encoder_service::benchmark::encode_bits::h865b9e06648afa8c+0x3c0> @ imm = #0x152
 80028ba: f108 0301    	add.w	r3, r8, #0x1
 80028be: 185a         	adds	r2, r3, r1
 80028c0: 4542         	cmp	r2, r8
 80028c2: f240 80a3    	bls.w	0x8002a0c <raylar_crcl_encoder_service::benchmark::encode_bits::h865b9e06648afa8c+0x3c0> @ imm = #0x146
 80028c6: 9c0f         	ldr	r4, [sp, #0x3c]
 80028c8: 42a2         	cmp	r2, r4
 80028ca: f200 809f    	bhi.w	0x8002a0c <raylar_crcl_encoder_service::benchmark::encode_bits::h865b9e06648afa8c+0x3c0> @ imm = #0x13e
 80028ce: b381         	cbz	r1, 0x8002932 <raylar_crcl_encoder_service::benchmark::encode_bits::h865b9e06648afa8c+0x2e6> @ imm = #0x60
 80028d0: 9a05         	ldr	r2, [sp, #0x14]
 80028d2: 4698         	mov	r8, r3
 80028d4: ea00 0a02    	and.w	r10, r0, r2
 80028d8: 460a         	mov	r2, r1
 80028da: f018 0007    	ands	r0, r8, #0x7
 80028de: ea4f 03d8    	lsr.w	r3, r8, #0x3
 80028e2: d105         	bne	0x80028f0 <raylar_crcl_encoder_service::benchmark::encode_bits::h865b9e06648afa8c+0x2a4> @ imm = #0xa
 80028e4: 4573         	cmp	r3, lr
 80028e6: f080 80bc    	bhs.w	0x8002a62 <raylar_crcl_encoder_service::benchmark::encode_bits::h865b9e06648afa8c+0x416> @ imm = #0x178
 80028ea: 2100         	movs	r1, #0x0
 80028ec: f809 1003    	strb.w	r1, [r9, r3]
 80028f0: 4573         	cmp	r3, lr
 80028f2: f080 80ae    	bhs.w	0x8002a52 <raylar_crcl_encoder_service::benchmark::encode_bits::h865b9e06648afa8c+0x406> @ imm = #0x15c
 80028f6: f1c0 0008    	rsb.w	r0, r0, #0x8
 80028fa: 4611         	mov	r1, r2
 80028fc: 4290         	cmp	r0, r2
 80028fe: bf38         	it	lo
 8002900: 4601         	movlo	r1, r0
 8002902: 1a52         	subs	r2, r2, r1
 8002904: eba0 0001    	sub.w	r0, r0, r1
 8002908: f002 051f    	and	r5, r2, #0x1f
 800290c: fa0c f401    	lsl.w	r4, r12, r1
 8002910: f819 6003    	ldrb.w	r6, [r9, r3]
 8002914: f000 0007    	and	r0, r0, #0x7
 8002918: fa2a f505    	lsr.w	r5, r10, r5
 800291c: ea25 0404    	bic.w	r4, r5, r4
 8002920: 4488         	add	r8, r1
 8002922: fa04 f000    	lsl.w	r0, r4, r0
 8002926: ea40 0006    	orr.w	r0, r0, r6
 800292a: f809 0003    	strb.w	r0, [r9, r3]
 800292e: d1d4         	bne	0x80028da <raylar_crcl_encoder_service::benchmark::encode_bits::h865b9e06648afa8c+0x28e> @ imm = #-0x58
 8002930: e768         	b	0x8002804 <raylar_crcl_encoder_service::benchmark::encode_bits::h865b9e06648afa8c+0x1b8> @ imm = #-0x130
 8002932: 4698         	mov	r8, r3
 8002934: e766         	b	0x8002804 <raylar_crcl_encoder_service::benchmark::encode_bits::h865b9e06648afa8c+0x1b8> @ imm = #-0x134
 8002936: f1c8 0000    	rsb.w	r0, r8, #0x0
 800293a: f000 0107    	and	r1, r0, #0x7
 800293e: eb18 0001    	adds.w	r0, r8, r1
 8002942: d26f         	bhs	0x8002a24 <raylar_crcl_encoder_service::benchmark::encode_bits::h865b9e06648afa8c+0x3d8> @ imm = #0xde
 8002944: 9a0f         	ldr	r2, [sp, #0x3c]
 8002946: 4290         	cmp	r0, r2
 8002948: d86c         	bhi	0x8002a24 <raylar_crcl_encoder_service::benchmark::encode_bits::h865b9e06648afa8c+0x3d8> @ imm = #0xd8
 800294a: b1a1         	cbz	r1, 0x8002976 <raylar_crcl_encoder_service::benchmark::encode_bits::h865b9e06648afa8c+0x32a> @ imm = #0x28
 800294c: f018 0007    	ands	r0, r8, #0x7
 8002950: ea4f 03d8    	lsr.w	r3, r8, #0x3
 8002954: d105         	bne	0x8002962 <raylar_crcl_encoder_service::benchmark::encode_bits::h865b9e06648afa8c+0x316> @ imm = #0xa
 8002956: 4573         	cmp	r3, lr
 8002958: f080 8083    	bhs.w	0x8002a62 <raylar_crcl_encoder_service::benchmark::encode_bits::h865b9e06648afa8c+0x416> @ imm = #0x106
 800295c: 2200         	movs	r2, #0x0
 800295e: f809 2003    	strb.w	r2, [r9, r3]
 8002962: 4573         	cmp	r3, lr
 8002964: d275         	bhs	0x8002a52 <raylar_crcl_encoder_service::benchmark::encode_bits::h865b9e06648afa8c+0x406> @ imm = #0xea
 8002966: f1c0 0008    	rsb.w	r0, r0, #0x8
 800296a: 4288         	cmp	r0, r1
 800296c: bf28         	it	hs
 800296e: 4608         	movhs	r0, r1
 8002970: 4480         	add	r8, r0
 8002972: 1a09         	subs	r1, r1, r0
 8002974: d1ea         	bne	0x800294c <raylar_crcl_encoder_service::benchmark::encode_bits::h865b9e06648afa8c+0x300> @ imm = #-0x2c
 8002976: 9904         	ldr	r1, [sp, #0x10]
 8002978: f641 6658    	movw	r6, #0x1e58
 800297c: f8dd a00c    	ldr.w	r10, [sp, #0xc]
 8002980: f641 6556    	movw	r5, #0x1e56
 8002984: 9a0b         	ldr	r2, [sp, #0x2c]
 8002986: f501 33e0    	add.w	r3, r1, #0x1c000
 800298a: 9812         	ldr	r0, [sp, #0x48]
 800298c: f1ba 0f05    	cmp.w	r10, #0x5
 8002990: eb00 00d8    	add.w	r0, r0, r8, lsr #3
 8002994: 9012         	str	r0, [sp, #0x48]
 8002996: f47f ae74    	bne.w	0x8002682 <raylar_crcl_encoder_service::benchmark::encode_bits::h865b9e06648afa8c+0x36> @ imm = #-0x318
 800299a: 9812         	ldr	r0, [sp, #0x48]
 800299c: f641 6159    	movw	r1, #0x1e59
 80029a0: 4288         	cmp	r0, r1
 80029a2: d266         	bhs	0x8002a72 <raylar_crcl_encoder_service::benchmark::encode_bits::h865b9e06648afa8c+0x426> @ imm = #0xcc
 80029a4: 9812         	ldr	r0, [sp, #0x48]
 80029a6: a915         	add	r1, sp, #0x54
 80029a8: e9cd 3015    	strd	r3, r0, [sp, #84]
 80029ac: b017         	add	sp, #0x5c
 80029ae: e8bd 0f00    	pop.w	{r8, r9, r10, r11}
 80029b2: bdf0         	pop	{r4, r5, r6, r7, pc}
 80029b4: 2002         	movs	r0, #0x2
 80029b6: f64d 53e4    	movw	r3, #0xdde4
 80029ba: f807 0c29    	strb	r0, [r7, #-41]
 80029be: f64d 6034    	movw	r0, #0xde34
 80029c2: f6c0 0000    	movt	r0, #0x800
 80029c6: f1a7 0229    	sub.w	r2, r7, #0x29
 80029ca: e015         	b	0x80029f8 <raylar_crcl_encoder_service::benchmark::encode_bits::h865b9e06648afa8c+0x3ac> @ imm = #0x2a
 80029cc: 2002         	movs	r0, #0x2
 80029ce: f64d 53e4    	movw	r3, #0xdde4
 80029d2: f807 0c27    	strb	r0, [r7, #-39]
 80029d6: f64d 6014    	movw	r0, #0xde14
 80029da: f6c0 0000    	movt	r0, #0x800
 80029de: f1a7 0227    	sub.w	r2, r7, #0x27
 80029e2: e009         	b	0x80029f8 <raylar_crcl_encoder_service::benchmark::encode_bits::h865b9e06648afa8c+0x3ac> @ imm = #0x12
 80029e4: 2002         	movs	r0, #0x2
 80029e6: f64d 53e4    	movw	r3, #0xdde4
 80029ea: f88d 0050    	strb.w	r0, [sp, #0x50]
 80029ee: f64d 6024    	movw	r0, #0xde24
 80029f2: f6c0 0000    	movt	r0, #0x800
 80029f6: aa14         	add	r2, sp, #0x50
 80029f8: 9000         	str	r0, [sp]
 80029fa: f64d 5048    	movw	r0, #0xdd48
 80029fe: f6c0 0000    	movt	r0, #0x800
 8002a02: f6c0 0300    	movt	r3, #0x800
 8002a06: 212b         	movs	r1, #0x2b
 8002a08: f008 ff06    	bl	0x800b818 <core::result::unwrap_failed> @ imm = #0x8e0c
 8002a0c: 2002         	movs	r0, #0x2
 8002a0e: f64d 53e4    	movw	r3, #0xdde4
 8002a12: f807 0c26    	strb	r0, [r7, #-38]
 8002a16: f64d 6004    	movw	r0, #0xde04
 8002a1a: f6c0 0000    	movt	r0, #0x800
 8002a1e: f1a7 0226    	sub.w	r2, r7, #0x26
 8002a22: e7e9         	b	0x80029f8 <raylar_crcl_encoder_service::benchmark::encode_bits::h865b9e06648afa8c+0x3ac> @ imm = #-0x2e
 8002a24: 2002         	movs	r0, #0x2
 8002a26: f64d 53e4    	movw	r3, #0xdde4
 8002a2a: f807 0c25    	strb	r0, [r7, #-37]
 8002a2e: f64d 50f4    	movw	r0, #0xddf4
 8002a32: f6c0 0000    	movt	r0, #0x800
 8002a36: f1a7 0225    	sub.w	r2, r7, #0x25
 8002a3a: e7dd         	b	0x80029f8 <raylar_crcl_encoder_service::benchmark::encode_bits::h865b9e06648afa8c+0x3ac> @ imm = #-0x46
 8002a3c: 9812         	ldr	r0, [sp, #0x48]
 8002a3e: f64d 6344    	movw	r3, #0xde44
 8002a42: f6c0 0300    	movt	r3, #0x800
 8002a46: f641 6158    	movw	r1, #0x1e58
 8002a4a: f641 6258    	movw	r2, #0x1e58
 8002a4e: f009 f9ee    	bl	0x800be2e <core::slice::index::slice_index_fail> @ imm = #0x93dc
 8002a52: f64d 0278    	movw	r2, #0xd878
 8002a56: f6c0 0200    	movt	r2, #0x800
 8002a5a: 4618         	mov	r0, r3
 8002a5c: 4671         	mov	r1, lr
 8002a5e: f008 fef9    	bl	0x800b854 <core::panicking::panic_bounds_check> @ imm = #0x8df2
 8002a62: f64d 0268    	movw	r2, #0xd868
 8002a66: f6c0 0200    	movt	r2, #0x800
 8002a6a: 4618         	mov	r0, r3
 8002a6c: 4671         	mov	r1, lr
 8002a6e: f008 fef1    	bl	0x800b854 <core::panicking::panic_bounds_check> @ imm = #0x8de2
 8002a72: 9912         	ldr	r1, [sp, #0x48]
 8002a74: f64d 53d4    	movw	r3, #0xddd4
 8002a78: f6c0 0300    	movt	r3, #0x800
 8002a7c: 2000         	movs	r0, #0x0
 8002a7e: f641 6258    	movw	r2, #0x1e58
 8002a82: f009 f9d4    	bl	0x800be2e <core::slice::index::slice_index_fail> @ imm = #0x93a8

08002a86 <raylar_crcl_encoder_service::benchmark::pack_header::haa3cc0f32481b1e3>:
 8002a86: b5f0         	push	{r4, r5, r6, r7, lr}
 8002a88: af03         	add	r7, sp, #0xc
 8002a8a: f84d 8d04    	str	r8, [sp, #-4]!
 8002a8e: b082         	sub	sp, #0x8
 8002a90: 4604         	mov	r4, r0
 8002a92: f500 30e0    	add.w	r0, r0, #0x1c000
 8002a96: 3004         	adds	r0, #0x4
 8002a98: 214c         	movs	r1, #0x4c
 8002a9a: f00a f922    	bl	0x800cce2 <__aeabi_memclr4> @ imm = #0xa244
 8002a9e: f245 2043    	movw	r0, #0x5243
 8002aa2: f44f 31e0    	mov.w	r1, #0x1c000
 8002aa6: f6c4 3042    	movt	r0, #0x4b42
 8002aaa: 2201         	movs	r2, #0x1
 8002aac: 5060         	str	r0, [r4, r1]
 8002aae: f64f 41c4    	movw	r1, #0xfcc4
 8002ab2: f504 30e0    	add.w	r0, r4, #0x1c000
 8002ab6: f2c0 0101    	movt	r1, #0x1
 8002aba: f44f 6380    	mov.w	r3, #0x400
 8002abe: f854 c001    	ldr.w	r12, [r4, r1]
 8002ac2: 4421         	add	r1, r4
 8002ac4: 7102         	strb	r2, [r0, #0x4]
 8002ac6: f240 2202    	movw	r2, #0x202
 8002aca: 8303         	strh	r3, [r0, #0x18]
 8002acc: 7b23         	ldrb	r3, [r4, #0xc]
 8002ace: f240 2503    	movw	r5, #0x203
 8002ad2: 8382         	strh	r2, [r0, #0x1c]
 8002ad4: 2205         	movs	r2, #0x5
 8002ad6: f880 302b    	strb.w	r3, [r0, #0x2b]
 8002ada: f44f 7301    	mov.w	r3, #0x204
 8002ade: 7682         	strb	r2, [r0, #0x1a]
 8002ae0: f04f 0858    	mov.w	r8, #0x58
 8002ae4: 694a         	ldr	r2, [r1, #0x14]
 8002ae6: 8683         	strh	r3, [r0, #0x34]
 8002ae8: f240 2305    	movw	r3, #0x205
 8002aec: f8d4 e000    	ldr.w	lr, [r4]
 8002af0: 8505         	strh	r5, [r0, #0x28]
 8002af2: 68a5         	ldr	r5, [r4, #0x8]
 8002af4: 6082         	str	r2, [r0, #0x8]
 8002af6: 7922         	ldrb	r2, [r4, #0x4]
 8002af8: f8a0 3040    	strh.w	r3, [r0, #0x40]
 8002afc: f240 2306    	movw	r3, #0x206
 8002b00: e9c0 ec08    	strd	lr, r12, [r0, #32]
 8002b04: f8a0 304c    	strh.w	r3, [r0, #0x4c]
 8002b08: 7d23         	ldrb	r3, [r4, #0x14]
 8002b0a: 77c2         	strb	r2, [r0, #0x1f]
 8002b0c: 62c5         	str	r5, [r0, #0x2c]
 8002b0e: e9d1 5201    	ldrd	r5, r2, [r1, #4]
 8002b12: f8d1 c00c    	ldr.w	r12, [r1, #0xc]
 8002b16: 6909         	ldr	r1, [r1, #0x10]
 8002b18: 6305         	str	r5, [r0, #0x30]
 8002b1a: 6925         	ldr	r5, [r4, #0x10]
 8002b1c: f880 3037    	strb.w	r3, [r0, #0x37]
 8002b20: 7f23         	ldrb	r3, [r4, #0x1c]
 8002b22: 69a6         	ldr	r6, [r4, #0x18]
 8002b24: e9c0 520e    	strd	r5, r2, [r0, #56]
 8002b28: f894 2024    	ldrb.w	r2, [r4, #0x24]
 8002b2c: f880 3043    	strb.w	r3, [r0, #0x43]
 8002b30: 6a23         	ldr	r3, [r4, #0x20]
 8002b32: f8a0 8006    	strh.w	r8, [r0, #0x6]
 8002b36: e9c0 6c11    	strd	r6, r12, [r0, #68]
 8002b3a: f880 204f    	strb.w	r2, [r0, #0x4f]
 8002b3e: e9c0 3114    	strd	r3, r1, [r0, #80]
 8002b42: e9cd 0800    	strd	r0, r8, [sp]
 8002b46: 4668         	mov	r0, sp
 8002b48: 2058         	movs	r0, #0x58
 8002b4a: b002         	add	sp, #0x8
 8002b4c: f85d 8b04    	ldr	r8, [sp], #4
 8002b50: bdf0         	pop	{r4, r5, r6, r7, pc}

08002b52 <raylar_crcl_encoder_service::benchmark::encode_words::h522ea4cb3ed82c82>:
 8002b52: b5f0         	push	{r4, r5, r6, r7, lr}
 8002b54: af03         	add	r7, sp, #0xc
 8002b56: e92d 0f00    	push.w	{r8, r9, r10, r11}
 8002b5a: b091         	sub	sp, #0x44
 8002b5c: f500 31b8    	add.w	r1, r0, #0x17000
 8002b60: f500 3ee0    	add.w	lr, r0, #0x1c000
 8002b64: f101 0614    	add.w	r6, r1, #0x14
 8002b68: f24c 0128    	movw	r1, #0xc028
 8002b6c: 1845         	adds	r5, r0, r1
 8002b6e: 9005         	str	r0, [sp, #0x14]
 8002b70: 2000         	movs	r0, #0x0
 8002b72: 2258         	movs	r2, #0x58
 8002b74: f641 6458    	movw	r4, #0x1e58
 8002b78: f04f 38ff    	mov.w	r8, #0xffffffff
 8002b7c: f04f 0c01    	mov.w	r12, #0x1
 8002b80: e88d 4060    	stm.w	sp, {r5, r6, lr}
 8002b84: 42a2         	cmp	r2, r4
 8002b86: f200 8147    	bhi.w	0x8002e18 <raylar_crcl_encoder_service::benchmark::encode_words::h522ea4cb3ed82c82+0x2c6> @ imm = #0x28e
 8002b8a: 9905         	ldr	r1, [sp, #0x14]
 8002b8c: eb0e 0902    	add.w	r9, lr, r2
 8002b90: 9204         	str	r2, [sp, #0x10]
 8002b92: 1aa2         	subs	r2, r4, r2
 8002b94: eb01 01c0    	add.w	r1, r1, r0, lsl #3
 8002b98: f04f 0b0a    	mov.w	r11, #0xa
 8002b9c: f04f 0a00    	mov.w	r10, #0x0
 8002ba0: 790b         	ldrb	r3, [r1, #0x4]
 8002ba2: 1c41         	adds	r1, r0, #0x1
 8002ba4: ebc0 2080    	rsb	r0, r0, r0, lsl #10
 8002ba8: 9103         	str	r1, [sp, #0xc]
 8002baa: f855 1021    	ldr.w	r1, [r5, r1, lsl #2]
 8002bae: eb06 0080    	add.w	r0, r6, r0, lsl #2
 8002bb2: f003 061f    	and	r6, r3, #0x1f
 8002bb6: 900d         	str	r0, [sp, #0x34]
 8002bb8: 9309         	str	r3, [sp, #0x24]
 8002bba: fa08 f006    	lsl.w	r0, r8, r6
 8002bbe: 058c         	lsls	r4, r1, #0x16
 8002bc0: 43c0         	mvns	r0, r0
 8002bc2: 1c59         	adds	r1, r3, #0x1
 8002bc4: 9006         	str	r0, [sp, #0x18]
 8002bc6: fa0c f006    	lsl.w	r0, r12, r6
 8002bca: 910b         	str	r1, [sp, #0x2c]
 8002bcc: 2100         	movs	r1, #0x0
 8002bce: 9008         	str	r0, [sp, #0x20]
 8002bd0: 3801         	subs	r0, #0x1
 8002bd2: 960c         	str	r6, [sp, #0x30]
 8002bd4: 9007         	str	r0, [sp, #0x1c]
 8002bd6: 980d         	ldr	r0, [sp, #0x34]
 8002bd8: f850 602a    	ldr.w	r6, [r0, r10, lsl #2]
 8002bdc: 980c         	ldr	r0, [sp, #0x30]
 8002bde: fa26 f500    	lsr.w	r5, r6, r0
 8002be2: 980b         	ldr	r0, [sp, #0x2c]
 8002be4: 1943         	adds	r3, r0, r5
 8002be6: 2b21         	cmp	r3, #0x21
 8002be8: d23c         	bhs	0x8002c64 <raylar_crcl_encoder_service::benchmark::encode_words::h522ea4cb3ed82c82+0x112> @ imm = #0x78
 8002bea: 2b00         	cmp	r3, #0x0
 8002bec: f000 80d2    	beq.w	0x8002d94 <raylar_crcl_encoder_service::benchmark::encode_words::h522ea4cb3ed82c82+0x242> @ imm = #0x1a4
 8002bf0: 9807         	ldr	r0, [sp, #0x1c]
 8002bf2: 4030         	ands	r0, r6
 8002bf4: 9e08         	ldr	r6, [sp, #0x20]
 8002bf6: ea40 0c06    	orr.w	r12, r0, r6
 8002bfa: 4608         	mov	r0, r1
 8002bfc: e002         	b	0x8002c04 <raylar_crcl_encoder_service::benchmark::encode_words::h522ea4cb3ed82c82+0xb2> @ imm = #0x4
 8002bfe: 2b00         	cmp	r3, #0x0
 8002c00: f000 80c8    	beq.w	0x8002d94 <raylar_crcl_encoder_service::benchmark::encode_words::h522ea4cb3ed82c82+0x242> @ imm = #0x190
 8002c04: f1cb 0520    	rsb.w	r5, r11, #0x20
 8002c08: 429d         	cmp	r5, r3
 8002c0a: bf28         	it	hs
 8002c0c: 461d         	movhs	r5, r3
 8002c0e: 2d20         	cmp	r5, #0x20
 8002c10: d102         	bne	0x8002c18 <raylar_crcl_encoder_service::benchmark::encode_words::h522ea4cb3ed82c82+0xc6> @ imm = #0x4
 8002c12: 3b20         	subs	r3, #0x20
 8002c14: 4666         	mov	r6, r12
 8002c16: e00a         	b	0x8002c2e <raylar_crcl_encoder_service::benchmark::encode_words::h522ea4cb3ed82c82+0xdc> @ imm = #0x14
 8002c18: f005 061f    	and	r6, r5, #0x1f
 8002c1c: 1b5b         	subs	r3, r3, r5
 8002c1e: fa08 fe06    	lsl.w	lr, r8, r6
 8002c22: f003 061f    	and	r6, r3, #0x1f
 8002c26: fa2c f606    	lsr.w	r6, r12, r6
 8002c2a: ea26 060e    	bic.w	r6, r6, lr
 8002c2e: 44ab         	add	r11, r5
 8002c30: f1cb 0500    	rsb.w	r5, r11, #0x0
 8002c34: f1bb 0f20    	cmp.w	r11, #0x20
 8002c38: f005 051f    	and	r5, r5, #0x1f
 8002c3c: fa06 f605    	lsl.w	r6, r6, r5
 8002c40: ea44 0406    	orr.w	r4, r4, r6
 8002c44: d1db         	bne	0x8002bfe <raylar_crcl_encoder_service::benchmark::encode_words::h522ea4cb3ed82c82+0xac> @ imm = #-0x4a
 8002c46: 1d01         	adds	r1, r0, #0x4
 8002c48: f110 0f05    	cmn.w	r0, #0x5
 8002c4c: bf98         	it	ls
 8002c4e: 4291         	cmpls	r1, r2
 8002c50: f200 80dc    	bhi.w	0x8002e0c <raylar_crcl_encoder_service::benchmark::encode_words::h522ea4cb3ed82c82+0x2ba> @ imm = #0x1b8
 8002c54: ba26         	rev	r6, r4
 8002c56: f849 6000    	str.w	r6, [r9, r0]
 8002c5a: 2400         	movs	r4, #0x0
 8002c5c: 4608         	mov	r0, r1
 8002c5e: f04f 0b00    	mov.w	r11, #0x0
 8002c62: e7cc         	b	0x8002bfe <raylar_crcl_encoder_service::benchmark::encode_words::h522ea4cb3ed82c82+0xac> @ imm = #-0x68
 8002c64: 960a         	str	r6, [sp, #0x28]
 8002c66: b32d         	cbz	r5, 0x8002cb4 <raylar_crcl_encoder_service::benchmark::encode_words::h522ea4cb3ed82c82+0x162> @ imm = #0x4a
 8002c68: 46ae         	mov	lr, r5
 8002c6a: 2d20         	cmp	r5, #0x20
 8002c6c: bf28         	it	hs
 8002c6e: f04f 0e20    	movhs.w	lr, #0x20
 8002c72: 460e         	mov	r6, r1
 8002c74: 4670         	mov	r0, lr
 8002c76: e002         	b	0x8002c7e <raylar_crcl_encoder_service::benchmark::encode_words::h522ea4cb3ed82c82+0x12c> @ imm = #0x4
 8002c78: ebb0 000c    	subs.w	r0, r0, r12
 8002c7c: d017         	beq	0x8002cae <raylar_crcl_encoder_service::benchmark::encode_words::h522ea4cb3ed82c82+0x15c> @ imm = #0x2e
 8002c7e: f1cb 0c20    	rsb.w	r12, r11, #0x20
 8002c82: 4584         	cmp	r12, r0
 8002c84: bf28         	it	hs
 8002c86: 4684         	movhs	r12, r0
 8002c88: 44e3         	add	r11, r12
 8002c8a: f1bb 0f20    	cmp.w	r11, #0x20
 8002c8e: d1f3         	bne	0x8002c78 <raylar_crcl_encoder_service::benchmark::encode_words::h522ea4cb3ed82c82+0x126> @ imm = #-0x1a
 8002c90: 1d31         	adds	r1, r6, #0x4
 8002c92: f116 0f05    	cmn.w	r6, #0x5
 8002c96: bf98         	it	ls
 8002c98: 4291         	cmpls	r1, r2
 8002c9a: f200 80b0    	bhi.w	0x8002dfe <raylar_crcl_encoder_service::benchmark::encode_words::h522ea4cb3ed82c82+0x2ac> @ imm = #0x160
 8002c9e: ba23         	rev	r3, r4
 8002ca0: f849 3006    	str.w	r3, [r9, r6]
 8002ca4: 2400         	movs	r4, #0x0
 8002ca6: 460e         	mov	r6, r1
 8002ca8: f04f 0b00    	mov.w	r11, #0x0
 8002cac: e7e4         	b	0x8002c78 <raylar_crcl_encoder_service::benchmark::encode_words::h522ea4cb3ed82c82+0x126> @ imm = #-0x38
 8002cae: ebb5 050e    	subs.w	r5, r5, lr
 8002cb2: d1d9         	bne	0x8002c68 <raylar_crcl_encoder_service::benchmark::encode_words::h522ea4cb3ed82c82+0x116> @ imm = #-0x4e
 8002cb4: f8dd e024    	ldr.w	lr, [sp, #0x24]
 8002cb8: 460e         	mov	r6, r1
 8002cba: f04f 0c01    	mov.w	r12, #0x1
 8002cbe: e003         	b	0x8002cc8 <raylar_crcl_encoder_service::benchmark::encode_words::h522ea4cb3ed82c82+0x176> @ imm = #0x6
 8002cc0: f1bb 0f20    	cmp.w	r11, #0x20
 8002cc4: 46ab         	mov	r11, r5
 8002cc6: d126         	bne	0x8002d16 <raylar_crcl_encoder_service::benchmark::encode_words::h522ea4cb3ed82c82+0x1c4> @ imm = #0x4c
 8002cc8: f1ab 0520    	sub.w	r5, r11, #0x20
 8002ccc: f1bb 0020    	subs.w	r0, r11, #0x20
 8002cd0: fab5 f585    	clz	r5, r5
 8002cd4: bf18         	it	ne
 8002cd6: 2001         	movne	r0, #0x1
 8002cd8: fa08 f300    	lsl.w	r3, r8, r0
 8002cdc: 096d         	lsrs	r5, r5, #0x5
 8002cde: fa2c f505    	lsr.w	r5, r12, r5
 8002ce2: ea25 0303    	bic.w	r3, r5, r3
 8002ce6: eb0b 0500    	add.w	r5, r11, r0
 8002cea: 4268         	rsbs	r0, r5, #0
 8002cec: 2d20         	cmp	r5, #0x20
 8002cee: f000 001f    	and	r0, r0, #0x1f
 8002cf2: fa03 f000    	lsl.w	r0, r3, r0
 8002cf6: ea44 0400    	orr.w	r4, r4, r0
 8002cfa: d1e1         	bne	0x8002cc0 <raylar_crcl_encoder_service::benchmark::encode_words::h522ea4cb3ed82c82+0x16e> @ imm = #-0x3e
 8002cfc: 1d31         	adds	r1, r6, #0x4
 8002cfe: f116 0f05    	cmn.w	r6, #0x5
 8002d02: bf98         	it	ls
 8002d04: 4291         	cmpls	r1, r2
 8002d06: d87a         	bhi	0x8002dfe <raylar_crcl_encoder_service::benchmark::encode_words::h522ea4cb3ed82c82+0x2ac> @ imm = #0xf4
 8002d08: ba20         	rev	r0, r4
 8002d0a: f849 0006    	str.w	r0, [r9, r6]
 8002d0e: 2400         	movs	r4, #0x0
 8002d10: 460e         	mov	r6, r1
 8002d12: 2500         	movs	r5, #0x0
 8002d14: e7d4         	b	0x8002cc0 <raylar_crcl_encoder_service::benchmark::encode_words::h522ea4cb3ed82c82+0x16e> @ imm = #-0x58
 8002d16: f1be 0f00    	cmp.w	lr, #0x0
 8002d1a: d03a         	beq	0x8002d92 <raylar_crcl_encoder_service::benchmark::encode_words::h522ea4cb3ed82c82+0x240> @ imm = #0x74
 8002d1c: 9806         	ldr	r0, [sp, #0x18]
 8002d1e: 46ab         	mov	r11, r5
 8002d20: 9b0a         	ldr	r3, [sp, #0x28]
 8002d22: ea03 0c00    	and.w	r12, r3, r0
 8002d26: 4608         	mov	r0, r1
 8002d28: e002         	b	0x8002d30 <raylar_crcl_encoder_service::benchmark::encode_words::h522ea4cb3ed82c82+0x1de> @ imm = #0x4
 8002d2a: f1be 0f00    	cmp.w	lr, #0x0
 8002d2e: d031         	beq	0x8002d94 <raylar_crcl_encoder_service::benchmark::encode_words::h522ea4cb3ed82c82+0x242> @ imm = #0x62
 8002d30: f1cb 0520    	rsb.w	r5, r11, #0x20
 8002d34: 4575         	cmp	r5, lr
 8002d36: bf28         	it	hs
 8002d38: 4675         	movhs	r5, lr
 8002d3a: 2d20         	cmp	r5, #0x20
 8002d3c: d103         	bne	0x8002d46 <raylar_crcl_encoder_service::benchmark::encode_words::h522ea4cb3ed82c82+0x1f4> @ imm = #0x6
 8002d3e: f1ae 0e20    	sub.w	lr, lr, #0x20
 8002d42: 4663         	mov	r3, r12
 8002d44: e00b         	b	0x8002d5e <raylar_crcl_encoder_service::benchmark::encode_words::h522ea4cb3ed82c82+0x20c> @ imm = #0x16
 8002d46: f005 031f    	and	r3, r5, #0x1f
 8002d4a: ebae 0e05    	sub.w	lr, lr, r5
 8002d4e: f00e 061f    	and	r6, lr, #0x1f
 8002d52: fa08 f303    	lsl.w	r3, r8, r3
 8002d56: fa2c f606    	lsr.w	r6, r12, r6
 8002d5a: ea26 0303    	bic.w	r3, r6, r3
 8002d5e: 44ab         	add	r11, r5
 8002d60: f1cb 0500    	rsb.w	r5, r11, #0x0
 8002d64: f1bb 0f20    	cmp.w	r11, #0x20
 8002d68: f005 051f    	and	r5, r5, #0x1f
 8002d6c: fa03 f305    	lsl.w	r3, r3, r5
 8002d70: ea44 0403    	orr.w	r4, r4, r3
 8002d74: d1d9         	bne	0x8002d2a <raylar_crcl_encoder_service::benchmark::encode_words::h522ea4cb3ed82c82+0x1d8> @ imm = #-0x4e
 8002d76: 1d01         	adds	r1, r0, #0x4
 8002d78: f110 0f05    	cmn.w	r0, #0x5
 8002d7c: bf98         	it	ls
 8002d7e: 4291         	cmpls	r1, r2
 8002d80: d844         	bhi	0x8002e0c <raylar_crcl_encoder_service::benchmark::encode_words::h522ea4cb3ed82c82+0x2ba> @ imm = #0x88
 8002d82: ba23         	rev	r3, r4
 8002d84: f849 3000    	str.w	r3, [r9, r0]
 8002d88: 2400         	movs	r4, #0x0
 8002d8a: 4608         	mov	r0, r1
 8002d8c: f04f 0b00    	mov.w	r11, #0x0
 8002d90: e7cb         	b	0x8002d2a <raylar_crcl_encoder_service::benchmark::encode_words::h522ea4cb3ed82c82+0x1d8> @ imm = #-0x6a
 8002d92: 46ab         	mov	r11, r5
 8002d94: f10a 0a01    	add.w	r10, r10, #0x1
 8002d98: 2301         	movs	r3, #0x1
 8002d9a: f240 30ff    	movw	r0, #0x3ff
 8002d9e: 4582         	cmp	r10, r0
 8002da0: f47f af19    	bne.w	0x8002bd6 <raylar_crcl_encoder_service::benchmark::encode_words::h522ea4cb3ed82c82+0x84> @ imm = #-0x1ce
 8002da4: ea4f 05db    	lsr.w	r5, r11, #0x3
 8002da8: ea5f 704b    	lsls.w	r0, r11, #0x1d
 8002dac: bf18         	it	ne
 8002dae: eb03 05db    	addne.w	r5, r3, r11, lsr #3
 8002db2: 186e         	adds	r6, r5, r1
 8002db4: d23b         	bhs	0x8002e2e <raylar_crcl_encoder_service::benchmark::encode_words::h522ea4cb3ed82c82+0x2dc> @ imm = #0x76
 8002db6: 4296         	cmp	r6, r2
 8002db8: d839         	bhi	0x8002e2e <raylar_crcl_encoder_service::benchmark::encode_words::h522ea4cb3ed82c82+0x2dc> @ imm = #0x72
 8002dba: ba20         	rev	r0, r4
 8002dbc: 2d05         	cmp	r5, #0x5
 8002dbe: 9010         	str	r0, [sp, #0x40]
 8002dc0: d23d         	bhs	0x8002e3e <raylar_crcl_encoder_service::benchmark::encode_words::h522ea4cb3ed82c82+0x2ec> @ imm = #0x7a
 8002dc2: eb09 0001    	add.w	r0, r9, r1
 8002dc6: a910         	add	r1, sp, #0x40
 8002dc8: 462a         	mov	r2, r5
 8002dca: 461c         	mov	r4, r3
 8002dcc: f009 fd60    	bl	0x800c890 <__aeabi_memcpy> @ imm = #0x9ac0
 8002dd0: 9a04         	ldr	r2, [sp, #0x10]
 8002dd2: 46a4         	mov	r12, r4
 8002dd4: 9803         	ldr	r0, [sp, #0xc]
 8002dd6: f641 6458    	movw	r4, #0x1e58
 8002dda: 4432         	add	r2, r6
 8002ddc: e89d 4060    	ldm.w	sp, {r5, r6, lr}
 8002de0: 2805         	cmp	r0, #0x5
 8002de2: f47f aecf    	bne.w	0x8002b84 <raylar_crcl_encoder_service::benchmark::encode_words::h522ea4cb3ed82c82+0x32> @ imm = #-0x262
 8002de6: f641 6059    	movw	r0, #0x1e59
 8002dea: 4282         	cmp	r2, r0
 8002dec: d230         	bhs	0x8002e50 <raylar_crcl_encoder_service::benchmark::encode_words::h522ea4cb3ed82c82+0x2fe> @ imm = #0x60
 8002dee: e9cd e20e    	strd	lr, r2, [sp, #56]
 8002df2: a80e         	add	r0, sp, #0x38
 8002df4: 4610         	mov	r0, r2
 8002df6: b011         	add	sp, #0x44
 8002df8: e8bd 0f00    	pop.w	{r8, r9, r10, r11}
 8002dfc: bdf0         	pop	{r4, r5, r6, r7, pc}
 8002dfe: f64d 7378    	movw	r3, #0xdf78
 8002e02: 4630         	mov	r0, r6
 8002e04: f6c0 0300    	movt	r3, #0x800
 8002e08: f009 f811    	bl	0x800be2e <core::slice::index::slice_index_fail> @ imm = #0x9022
 8002e0c: f64d 7378    	movw	r3, #0xdf78
 8002e10: f6c0 0300    	movt	r3, #0x800
 8002e14: f009 f80b    	bl	0x800be2e <core::slice::index::slice_index_fail> @ imm = #0x9016
 8002e18: f64d 6364    	movw	r3, #0xde64
 8002e1c: 4610         	mov	r0, r2
 8002e1e: f6c0 0300    	movt	r3, #0x800
 8002e22: f641 6158    	movw	r1, #0x1e58
 8002e26: f641 6258    	movw	r2, #0x1e58
 8002e2a: f009 f800    	bl	0x800be2e <core::slice::index::slice_index_fail> @ imm = #0x9000
 8002e2e: f64d 7398    	movw	r3, #0xdf98
 8002e32: 4608         	mov	r0, r1
 8002e34: f6c0 0300    	movt	r3, #0x800
 8002e38: 4631         	mov	r1, r6
 8002e3a: f008 fff8    	bl	0x800be2e <core::slice::index::slice_index_fail> @ imm = #0x8ff0
 8002e3e: f64d 7388    	movw	r3, #0xdf88
 8002e42: 2000         	movs	r0, #0x0
 8002e44: f6c0 0300    	movt	r3, #0x800
 8002e48: 4629         	mov	r1, r5
 8002e4a: 2204         	movs	r2, #0x4
 8002e4c: f008 ffef    	bl	0x800be2e <core::slice::index::slice_index_fail> @ imm = #0x8fde
 8002e50: f64d 6354    	movw	r3, #0xde54
 8002e54: 4611         	mov	r1, r2
 8002e56: f6c0 0300    	movt	r3, #0x800
 8002e5a: 2000         	movs	r0, #0x0
 8002e5c: f641 6258    	movw	r2, #0x1e58
 8002e60: f008 ffe5    	bl	0x800be2e <core::slice::index::slice_index_fail> @ imm = #0x8fca

08002e64 <raylar_crcl_encoder_service::benchmark::encode_folded::h7ac7123b2c9324bf>:
 8002e64: b580         	push	{r7, lr}
 8002e66: 466f         	mov	r7, sp
 8002e68: 2100         	movs	r1, #0x0
 8002e6a: 2200         	movs	r2, #0x0
 8002e6c: e8bd 4080    	pop.w	{r7, lr}
 8002e70: f7ff bbec    	b.w	0x800264c <raylar_crcl_encoder_service::benchmark::encode_bits::h865b9e06648afa8c> @ imm = #-0x828

08002e74 <raylar_crcl_encoder_service::benchmark::encode_payload::hfef902a78aec660e>:
 8002e74: b580         	push	{r7, lr}
 8002e76: 466f         	mov	r7, sp
 8002e78: 2100         	movs	r1, #0x0
 8002e7a: 2201         	movs	r2, #0x1
 8002e7c: e8bd 4080    	pop.w	{r7, lr}
 8002e80: f7ff bbe4    	b.w	0x800264c <raylar_crcl_encoder_service::benchmark::encode_bits::h865b9e06648afa8c> @ imm = #-0x838

08002e84 <raylar_crcl_encoder_service::benchmark::encode_codeword::h30aff6e5abaf4d69>:
 8002e84: b580         	push	{r7, lr}
 8002e86: 466f         	mov	r7, sp
 8002e88: 2101         	movs	r1, #0x1
 8002e8a: 2200         	movs	r2, #0x0
 8002e8c: e8bd 4080    	pop.w	{r7, lr}
 8002e90: f7ff bbdc    	b.w	0x800264c <raylar_crcl_encoder_service::benchmark::encode_bits::h865b9e06648afa8c> @ imm = #-0x848

08002e94 <raylar_crcl_encoder_service::benchmark::split_transform::h9238bb9046f68a6e>:
 8002e94: b5f0         	push	{r4, r5, r6, r7, lr}
 8002e96: af03         	add	r7, sp, #0xc
 8002e98: e92d 0f00    	push.w	{r8, r9, r10, r11}
 8002e9c: f2ad 5d04    	subw	sp, sp, #0x504
 8002ea0: 9002         	str	r0, [sp, #0x8]
 8002ea2: a817         	add	r0, sp, #0x5c
 8002ea4: 460e         	mov	r6, r1
 8002ea6: f44f 7101    	mov.w	r1, #0x204
 8002eaa: 4614         	mov	r4, r2
 8002eac: f009 ff19    	bl	0x800cce2 <__aeabi_memclr4> @ imm = #0x9e32
 8002eb0: a898         	add	r0, sp, #0x260
 8002eb2: f44f 7101    	mov.w	r1, #0x204
 8002eb6: f009 ff14    	bl	0x800cce2 <__aeabi_memclr4> @ imm = #0x9e28
 8002eba: f24c 0028    	movw	r0, #0xc028
 8002ebe: 1833         	adds	r3, r6, r0
 8002ec0: f246 0028    	movw	r0, #0x6028
 8002ec4: f1a7 0bbc    	sub.w	r11, r7, #0xbc
 8002ec8: 4430         	add	r0, r6
 8002eca: 9009         	str	r0, [sp, #0x24]
 8002ecc: f10b 0084    	add.w	r0, r11, #0x84
 8002ed0: 9008         	str	r0, [sp, #0x20]
 8002ed2: f10b 0074    	add.w	r0, r11, #0x74
 8002ed6: 9007         	str	r0, [sp, #0x1c]
 8002ed8: f10b 0048    	add.w	r0, r11, #0x48
 8002edc: 9006         	str	r0, [sp, #0x18]
 8002ede: f10b 0020    	add.w	r0, r11, #0x20
 8002ee2: f1a7 0924    	sub.w	r9, r7, #0x24
 8002ee6: 9005         	str	r0, [sp, #0x14]
 8002ee8: f106 0028    	add.w	r0, r6, #0x28
 8002eec: f06f 0607    	mvn	r6, #0x7
 8002ef0: f44f 6580    	mov.w	r5, #0x400
 8002ef4: 9004         	str	r0, [sp, #0x10]
 8002ef6: 9303         	str	r3, [sp, #0xc]
 8002ef8: e008         	b	0x8002f0c <raylar_crcl_encoder_service::benchmark::split_transform::h9238bb9046f68a6e+0x78> @ imm = #0x10
 8002efa: 3601         	adds	r6, #0x1
 8002efc: 46d9         	mov	r9, r11
 8002efe: f1a7 0bbc    	sub.w	r11, r7, #0xbc
 8002f02: f44f 6580    	mov.w	r5, #0x400
 8002f06: 2e81         	cmp	r6, #0x81
 8002f08: f000 8145    	beq.w	0x8003196 <raylar_crcl_encoder_service::benchmark::split_transform::h9238bb9046f68a6e+0x302> @ imm = #0x28a
 8002f0c: 9616         	str	r6, [sp, #0x58]
 8002f0e: f44f 42c0    	mov.w	r2, #0x6000
 8002f12: 9e09         	ldr	r6, [sp, #0x24]
 8002f14: 9904         	ldr	r1, [sp, #0x10]
 8002f16: 4630         	mov	r0, r6
 8002f18: f009 fe95    	bl	0x800cc46 <__aeabi_memcpy4> @ imm = #0x9d2a
 8002f1c: 9806         	ldr	r0, [sp, #0x18]
 8002f1e: 2128         	movs	r1, #0x28
 8002f20: f009 fedf    	bl	0x800cce2 <__aeabi_memclr4> @ imm = #0x9dbe
 8002f24: 9807         	ldr	r0, [sp, #0x1c]
 8002f26: 2100         	movs	r1, #0x0
 8002f28: f44f 0200    	mov.w	r2, #0x800000
 8002f2c: e947 112f    	strd	r1, r1, [r7, #-188]
 8002f30: e947 112d    	strd	r1, r1, [r7, #-180]
 8002f34: e9c0 1100    	strd	r1, r1, [r0]
 8002f38: e9c0 1102    	strd	r1, r1, [r0, #8]
 8002f3c: 9805         	ldr	r0, [sp, #0x14]
 8002f3e: e947 112b    	strd	r1, r1, [r7, #-172]
 8002f42: f847 1ca4    	str	r1, [r7, #-164]
 8002f46: e9c0 1100    	strd	r1, r1, [r0]
 8002f4a: e9c0 1102    	strd	r1, r1, [r0, #8]
 8002f4e: e9c0 1104    	strd	r1, r1, [r0, #16]
 8002f52: 6181         	str	r1, [r0, #0x18]
 8002f54: 8381         	strh	r1, [r0, #0x1c]
 8002f56: 9808         	ldr	r0, [sp, #0x20]
 8002f58: f827 1ca0    	strh	r1, [r7, #-160]
 8002f5c: f827 1c2c    	strh	r1, [r7, #-44]
 8002f60: 6042         	str	r2, [r0, #0x4]
 8002f62: f240 620a    	movw	r2, #0x60a
 8002f66: f2c0 1201    	movt	r2, #0x101
 8002f6a: 6005         	str	r5, [r0]
 8002f6c: 6082         	str	r2, [r0, #0x8]
 8002f6e: 2001         	movs	r0, #0x1
 8002f70: f847 0c4c    	str	r0, [r7, #-76]
 8002f74: f44f 50c0    	mov.w	r0, #0x1800
 8002f78: e947 601f    	strd	r6, r0, [r7, #-124]
 8002f7c: 47a0         	blx	r4
 8002f7e: f847 5c24    	str	r5, [r7, #-36]
 8002f82: 4682         	mov	r10, r0
 8002f84: 4658         	mov	r0, r11
 8002f86: f857 1c24    	ldr	r1, [r7, #-36]
 8002f8a: f7fe fd62    	bl	0x8001a52 <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3> @ imm = #-0x153c
 8002f8e: 07c0         	lsls	r0, r0, #0x1f
 8002f90: f040 80d7    	bne.w	0x8003142 <raylar_crcl_encoder_service::benchmark::split_transform::h9238bb9046f68a6e+0x2ae> @ imm = #0x1ae
 8002f94: f001 0001    	and	r0, r1, #0x1
 8002f98: f807 0c24    	strb	r0, [r7, #-36]
 8002f9c: 47a0         	blx	r4
 8002f9e: f847 5c24    	str	r5, [r7, #-36]
 8002fa2: 4606         	mov	r6, r0
 8002fa4: 4658         	mov	r0, r11
 8002fa6: f857 1c24    	ldr	r1, [r7, #-36]
 8002faa: f7fe fd52    	bl	0x8001a52 <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3> @ imm = #-0x155c
 8002fae: 07c0         	lsls	r0, r0, #0x1f
 8002fb0: f040 80ce    	bne.w	0x8003150 <raylar_crcl_encoder_service::benchmark::split_transform::h9238bb9046f68a6e+0x2bc> @ imm = #0x19c
 8002fb4: f001 0001    	and	r0, r1, #0x1
 8002fb8: 9615         	str	r6, [sp, #0x54]
 8002fba: f807 0c24    	strb	r0, [r7, #-36]
 8002fbe: 47a0         	blx	r4
 8002fc0: 4680         	mov	r8, r0
 8002fc2: 47a0         	blx	r4
 8002fc4: f847 5c24    	str	r5, [r7, #-36]
 8002fc8: 4606         	mov	r6, r0
 8002fca: 4658         	mov	r0, r11
 8002fcc: f857 1c24    	ldr	r1, [r7, #-36]
 8002fd0: f7fe fd3f    	bl	0x8001a52 <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3> @ imm = #-0x1582
 8002fd4: 07c0         	lsls	r0, r0, #0x1f
 8002fd6: f040 80b4    	bne.w	0x8003142 <raylar_crcl_encoder_service::benchmark::split_transform::h9238bb9046f68a6e+0x2ae> @ imm = #0x168
 8002fda: f001 0001    	and	r0, r1, #0x1
 8002fde: f807 0c24    	strb	r0, [r7, #-36]
 8002fe2: 47a0         	blx	r4
 8002fe4: 9014         	str	r0, [sp, #0x50]
 8002fe6: 4658         	mov	r0, r11
 8002fe8: f847 5c24    	str	r5, [r7, #-36]
 8002fec: f857 1c24    	ldr	r1, [r7, #-36]
 8002ff0: f7fe fd2f    	bl	0x8001a52 <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3> @ imm = #-0x15a2
 8002ff4: 07c0         	lsls	r0, r0, #0x1f
 8002ff6: f040 80ab    	bne.w	0x8003150 <raylar_crcl_encoder_service::benchmark::split_transform::h9238bb9046f68a6e+0x2bc> @ imm = #0x156
 8002ffa: f001 0001    	and	r0, r1, #0x1
 8002ffe: f807 0c24    	strb	r0, [r7, #-36]
 8003002: 47a0         	blx	r4
 8003004: 9013         	str	r0, [sp, #0x4c]
 8003006: 47a0         	blx	r4
 8003008: 9012         	str	r0, [sp, #0x48]
 800300a: 4658         	mov	r0, r11
 800300c: f847 5c24    	str	r5, [r7, #-36]
 8003010: f857 1c24    	ldr	r1, [r7, #-36]
 8003014: f7fe fd1d    	bl	0x8001a52 <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3> @ imm = #-0x15c6
 8003018: 07c0         	lsls	r0, r0, #0x1f
 800301a: f040 8092    	bne.w	0x8003142 <raylar_crcl_encoder_service::benchmark::split_transform::h9238bb9046f68a6e+0x2ae> @ imm = #0x124
 800301e: f001 0001    	and	r0, r1, #0x1
 8003022: f807 0c24    	strb	r0, [r7, #-36]
 8003026: 47a0         	blx	r4
 8003028: 9011         	str	r0, [sp, #0x44]
 800302a: 4658         	mov	r0, r11
 800302c: f847 5c24    	str	r5, [r7, #-36]
 8003030: f857 1c24    	ldr	r1, [r7, #-36]
 8003034: f7fe fd0d    	bl	0x8001a52 <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3> @ imm = #-0x15e6
 8003038: 07c0         	lsls	r0, r0, #0x1f
 800303a: f040 8089    	bne.w	0x8003150 <raylar_crcl_encoder_service::benchmark::split_transform::h9238bb9046f68a6e+0x2bc> @ imm = #0x112
 800303e: f001 0001    	and	r0, r1, #0x1
 8003042: f807 0c24    	strb	r0, [r7, #-36]
 8003046: 47a0         	blx	r4
 8003048: 9010         	str	r0, [sp, #0x40]
 800304a: 47a0         	blx	r4
 800304c: 900f         	str	r0, [sp, #0x3c]
 800304e: 4658         	mov	r0, r11
 8003050: f847 5c24    	str	r5, [r7, #-36]
 8003054: f857 1c24    	ldr	r1, [r7, #-36]
 8003058: f7fe fcfb    	bl	0x8001a52 <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3> @ imm = #-0x160a
 800305c: 07c0         	lsls	r0, r0, #0x1f
 800305e: d170         	bne	0x8003142 <raylar_crcl_encoder_service::benchmark::split_transform::h9238bb9046f68a6e+0x2ae> @ imm = #0xe0
 8003060: f001 0001    	and	r0, r1, #0x1
 8003064: f807 0c24    	strb	r0, [r7, #-36]
 8003068: 47a0         	blx	r4
 800306a: 900e         	str	r0, [sp, #0x38]
 800306c: 4658         	mov	r0, r11
 800306e: f847 5c24    	str	r5, [r7, #-36]
 8003072: f857 1c24    	ldr	r1, [r7, #-36]
 8003076: f7fe fcec    	bl	0x8001a52 <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3> @ imm = #-0x1628
 800307a: 07c0         	lsls	r0, r0, #0x1f
 800307c: d168         	bne	0x8003150 <raylar_crcl_encoder_service::benchmark::split_transform::h9238bb9046f68a6e+0x2bc> @ imm = #0xd0
 800307e: f001 0001    	and	r0, r1, #0x1
 8003082: f807 0c24    	strb	r0, [r7, #-36]
 8003086: 47a0         	blx	r4
 8003088: 900d         	str	r0, [sp, #0x34]
 800308a: 47a0         	blx	r4
 800308c: 900c         	str	r0, [sp, #0x30]
 800308e: 4658         	mov	r0, r11
 8003090: f847 5c24    	str	r5, [r7, #-36]
 8003094: f857 1c24    	ldr	r1, [r7, #-36]
 8003098: f7fe fcdb    	bl	0x8001a52 <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3> @ imm = #-0x164a
 800309c: 07c0         	lsls	r0, r0, #0x1f
 800309e: d150         	bne	0x8003142 <raylar_crcl_encoder_service::benchmark::split_transform::h9238bb9046f68a6e+0x2ae> @ imm = #0xa0
 80030a0: f001 0001    	and	r0, r1, #0x1
 80030a4: e9cd 680a    	strd	r6, r8, [sp, #40]
 80030a8: f807 0c24    	strb	r0, [r7, #-36]
 80030ac: 47a0         	blx	r4
 80030ae: f847 5c24    	str	r5, [r7, #-36]
 80030b2: 4680         	mov	r8, r0
 80030b4: 4658         	mov	r0, r11
 80030b6: f857 1c24    	ldr	r1, [r7, #-36]
 80030ba: f7fe fcca    	bl	0x8001a52 <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3> @ imm = #-0x166c
 80030be: 07c0         	lsls	r0, r0, #0x1f
 80030c0: d146         	bne	0x8003150 <raylar_crcl_encoder_service::benchmark::split_transform::h9238bb9046f68a6e+0x2bc> @ imm = #0x8c
 80030c2: f001 0001    	and	r0, r1, #0x1
 80030c6: f807 0c24    	strb	r0, [r7, #-36]
 80030ca: 9d15         	ldr	r5, [sp, #0x54]
 80030cc: 46cb         	mov	r11, r9
 80030ce: 47a0         	blx	r4
 80030d0: 9903         	ldr	r1, [sp, #0xc]
 80030d2: 4681         	mov	r9, r0
 80030d4: f857 0c78    	ldr	r0, [r7, #-120]
 80030d8: f847 1c28    	str	r1, [r7, #-40]
 80030dc: f5b0 5fc0    	cmp.w	r0, #0x1800
 80030e0: d14a         	bne	0x8003178 <raylar_crcl_encoder_service::benchmark::split_transform::h9238bb9046f68a6e+0x2e4> @ imm = #0x94
 80030e2: f857 0c7c    	ldr	r0, [r7, #-124]
 80030e6: f44f 42c0    	mov.w	r2, #0x6000
 80030ea: f009 fb97    	bl	0x800c81c <memcmp>      @ imm = #0x972e
 80030ee: 2800         	cmp	r0, #0x0
 80030f0: d142         	bne	0x8003178 <raylar_crcl_encoder_service::benchmark::split_transform::h9238bb9046f68a6e+0x2e4> @ imm = #0x84
 80030f2: 9e16         	ldr	r6, [sp, #0x58]
 80030f4: f106 0008    	add.w	r0, r6, #0x8
 80030f8: 2807         	cmp	r0, #0x7
 80030fa: f67f aefe    	bls.w	0x8002efa <raylar_crcl_encoder_service::benchmark::split_transform::h9238bb9046f68a6e+0x66> @ imm = #-0x204
 80030fe: 9a0a         	ldr	r2, [sp, #0x28]
 8003100: 9814         	ldr	r0, [sp, #0x50]
 8003102: eb0a 0102    	add.w	r1, r10, r2
 8003106: 9a11         	ldr	r2, [sp, #0x44]
 8003108: 4428         	add	r0, r5
 800310a: 9d13         	ldr	r5, [sp, #0x4c]
 800310c: 4410         	add	r0, r2
 800310e: 9a0b         	ldr	r2, [sp, #0x2c]
 8003110: 442a         	add	r2, r5
 8003112: 9d12         	ldr	r5, [sp, #0x48]
 8003114: 4429         	add	r1, r5
 8003116: 9d0e         	ldr	r5, [sp, #0x38]
 8003118: 4428         	add	r0, r5
 800311a: 9d10         	ldr	r5, [sp, #0x40]
 800311c: 442a         	add	r2, r5
 800311e: 9d0f         	ldr	r5, [sp, #0x3c]
 8003120: 4429         	add	r1, r5
 8003122: 9d0c         	ldr	r5, [sp, #0x30]
 8003124: 4429         	add	r1, r5
 8003126: 9d0d         	ldr	r5, [sp, #0x34]
 8003128: 1a41         	subs	r1, r0, r1
 800312a: 4440         	add	r0, r8
 800312c: 442a         	add	r2, r5
 800312e: 4441         	add	r1, r8
 8003130: 1a10         	subs	r0, r2, r0
 8003132: aa98         	add	r2, sp, #0x260
 8003134: 4448         	add	r0, r9
 8003136: f842 0026    	str.w	r0, [r2, r6, lsl #2]
 800313a: a817         	add	r0, sp, #0x5c
 800313c: f840 1026    	str.w	r1, [r0, r6, lsl #2]
 8003140: e6db         	b	0x8002efa <raylar_crcl_encoder_service::benchmark::split_transform::h9238bb9046f68a6e+0x66> @ imm = #-0x24a
 8003142: f64d 6094    	movw	r0, #0xde94
 8003146: f807 1c24    	strb	r1, [r7, #-36]
 800314a: f6c0 0000    	movt	r0, #0x800
 800314e: e005         	b	0x800315c <raylar_crcl_encoder_service::benchmark::split_transform::h9238bb9046f68a6e+0x2c8> @ imm = #0xa
 8003150: f64d 6084    	movw	r0, #0xde84
 8003154: f807 1c24    	strb	r1, [r7, #-36]
 8003158: f6c0 0000    	movt	r0, #0x800
 800315c: f64d 53e4    	movw	r3, #0xdde4
 8003160: 9000         	str	r0, [sp]
 8003162: f64d 5048    	movw	r0, #0xdd48
 8003166: f1a7 0224    	sub.w	r2, r7, #0x24
 800316a: f6c0 0000    	movt	r0, #0x800
 800316e: f6c0 0300    	movt	r3, #0x800
 8003172: 212b         	movs	r1, #0x2b
 8003174: f008 fb50    	bl	0x800b818 <core::result::unwrap_failed> @ imm = #0x86a0
 8003178: f64d 6074    	movw	r0, #0xde74
 800317c: f1a7 0228    	sub.w	r2, r7, #0x28
 8003180: f6c0 0000    	movt	r0, #0x800
 8003184: 2300         	movs	r3, #0x0
 8003186: 9001         	str	r0, [sp, #0x4]
 8003188: f1a7 00bc    	sub.w	r0, r7, #0xbc
 800318c: f100 0140    	add.w	r1, r0, #0x40
 8003190: 2000         	movs	r0, #0x0
 8003192: f005 fc71    	bl	0x8008a78 <core::panicking::assert_failed::h08239383d2bf65a5> @ imm = #0x58e2
 8003196: f1a7 041d    	sub.w	r4, r7, #0x1d
 800319a: a817         	add	r0, sp, #0x5c
 800319c: 2181         	movs	r1, #0x81
 800319e: 4622         	mov	r2, r4
 80031a0: f004 fd59    	bl	0x8007c56 <core::slice::sort::unstable::ipnsort::hd8a371dd4e708cab> @ imm = #0x4ab2
 80031a4: a898         	add	r0, sp, #0x260
 80031a6: 2181         	movs	r1, #0x81
 80031a8: 4622         	mov	r2, r4
 80031aa: f004 fd54    	bl	0x8007c56 <core::slice::sort::unstable::ipnsort::hd8a371dd4e708cab> @ imm = #0x4aa8
 80031ae: 9802         	ldr	r0, [sp, #0x8]
 80031b0: f64d 6ac0    	movw	r10, #0xdec0
 80031b4: f04f 091f    	mov.w	r9, #0x1f
 80031b8: f6c0 0a00    	movt	r10, #0x800
 80031bc: f8dd c260    	ldr.w	r12, [sp, #0x260]
 80031c0: f04f 0b01    	mov.w	r11, #0x1
 80031c4: e9c0 a908    	strd	r10, r9, [r0, #32]
 80031c8: f100 0928    	add.w	r9, r0, #0x28
 80031cc: f8dd 342c    	ldr.w	r3, [sp, #0x42c]
 80031d0: f04f 0800    	mov.w	r8, #0x0
 80031d4: f8dd e360    	ldr.w	lr, [sp, #0x360]
 80031d8: e889 5800    	stm.w	r9, {r11, r12, lr}
 80031dc: f100 0c34    	add.w	r12, r0, #0x34
 80031e0: f8dd 6460    	ldr.w	r6, [sp, #0x460]
 80031e4: e88c 0148    	stm.w	r12, {r3, r6, r8}
 80031e8: f64d 63a4    	movw	r3, #0xdea4
 80031ec: f6c0 0300    	movt	r3, #0x800
 80031f0: 9a57         	ldr	r2, [sp, #0x15c]
 80031f2: 9997         	ldr	r1, [sp, #0x25c]
 80031f4: 9c17         	ldr	r4, [sp, #0x5c]
 80031f6: 9d8a         	ldr	r5, [sp, #0x228]
 80031f8: 6003         	str	r3, [r0]
 80031fa: 231c         	movs	r3, #0x1c
 80031fc: e9c0 3b01    	strd	r3, r11, [r0, #4]
 8003200: e9c0 4203    	strd	r4, r2, [r0, #12]
 8003204: e9c0 5105    	strd	r5, r1, [r0, #20]
 8003208: f8c0 801c    	str.w	r8, [r0, #0x1c]
 800320c: f20d 5d04    	addw	sp, sp, #0x504
 8003210: e8bd 0f00    	pop.w	{r8, r9, r10, r11}
 8003214: bdf0         	pop	{r4, r5, r6, r7, pc}

08003216 <raylar_crcl_encoder_service::benchmark::transform_budgeted::h9752c8449280daff>:
 8003216: b5f0         	push	{r4, r5, r6, r7, lr}
 8003218: af03         	add	r7, sp, #0xc
 800321a: f84d 8d04    	str	r8, [sp, #-4]!
 800321e: b0aa         	sub	sp, #0xa8
 8003220: f240 610a    	movw	r1, #0x60a
 8003224: f2c0 1101    	movt	r1, #0x101
 8003228: 9129         	str	r1, [sp, #0xa4]
 800322a: f44f 0100    	mov.w	r1, #0x800000
 800322e: 9128         	str	r1, [sp, #0xa0]
 8003230: f44f 6180    	mov.w	r1, #0x400
 8003234: 9127         	str	r1, [sp, #0x9c]
 8003236: a927         	add	r1, sp, #0x9c
 8003238: 9927         	ldr	r1, [sp, #0x9c]
 800323a: 1e4a         	subs	r2, r1, #0x1
 800323c: ea81 0302    	eor.w	r3, r1, r2
 8003240: 4293         	cmp	r3, r2
 8003242: d916         	bls	0x8003272 <raylar_crcl_encoder_service::benchmark::transform_budgeted::h9752c8449280daff+0x5c> @ imm = #0x2c
 8003244: fa91 f2a1    	rbit	r2, r1
 8003248: fab2 f282    	clz	r2, r2
 800324c: 3a08         	subs	r2, #0x8
 800324e: 2a04         	cmp	r2, #0x4
 8003250: bf9c         	itt	ls
 8003252: f89d 20a4    	ldrbls.w	r2, [sp, #0xa4]
 8003256: 2a0c         	cmpls	r2, #0xc
 8003258: d80b         	bhi	0x8003272 <raylar_crcl_encoder_service::benchmark::transform_budgeted::h9752c8449280daff+0x5c> @ imm = #0x16
 800325a: 2301         	movs	r3, #0x1
 800325c: fa03 f202    	lsl.w	r2, r3, r2
 8003260: f412 5faa    	tst.w	r2, #0x1540
 8003264: bf1e         	ittt	ne
 8003266: 9a28         	ldrne	r2, [sp, #0xa0]
 8003268: f102 4240    	addne.w	r2, r2, #0xc0000000
 800326c: f1b2 4f40    	cmpne.w	r2, #0xc0000000
 8003270: d813         	bhi	0x800329a <raylar_crcl_encoder_service::benchmark::transform_budgeted::h9752c8449280daff+0x84> @ imm = #0x26
 8003272: 2000         	movs	r0, #0x0
 8003274: f64d 53e4    	movw	r3, #0xdde4
 8003278: f88d 0008    	strb.w	r0, [sp, #0x8]
 800327c: f64d 60f0    	movw	r0, #0xdef0
 8003280: f6c0 0000    	movt	r0, #0x800
 8003284: aa02         	add	r2, sp, #0x8
 8003286: 9000         	str	r0, [sp]
 8003288: f64d 5048    	movw	r0, #0xdd48
 800328c: f6c0 0000    	movt	r0, #0x800
 8003290: f6c0 0300    	movt	r3, #0x800
 8003294: 212b         	movs	r1, #0x2b
 8003296: f008 fabf    	bl	0x800b818 <core::result::unwrap_failed> @ imm = #0x857e
 800329a: f89d 20a5    	ldrb.w	r2, [sp, #0xa5]
 800329e: 2a1e         	cmp	r2, #0x1e
 80032a0: d8e7         	bhi	0x8003272 <raylar_crcl_encoder_service::benchmark::transform_budgeted::h9752c8449280daff+0x5c> @ imm = #-0x32
 80032a2: f89d 30a7    	ldrb.w	r3, [sp, #0xa7]
 80032a6: 2b01         	cmp	r3, #0x1
 80032a8: d107         	bne	0x80032ba <raylar_crcl_encoder_service::benchmark::transform_budgeted::h9752c8449280daff+0xa4> @ imm = #0xe
 80032aa: f5b1 6f80    	cmp.w	r1, #0x400
 80032ae: d3e0         	blo	0x8003272 <raylar_crcl_encoder_service::benchmark::transform_budgeted::h9752c8449280daff+0x5c> @ imm = #-0x40
 80032b0: f89d 10a6    	ldrb.w	r1, [sp, #0xa6]
 80032b4: 2901         	cmp	r1, #0x1
 80032b6: d005         	beq	0x80032c4 <raylar_crcl_encoder_service::benchmark::transform_budgeted::h9752c8449280daff+0xae> @ imm = #0xa
 80032b8: e7db         	b	0x8003272 <raylar_crcl_encoder_service::benchmark::transform_budgeted::h9752c8449280daff+0x5c> @ imm = #-0x4a
 80032ba: f5b1 6f80    	cmp.w	r1, #0x400
 80032be: d3d8         	blo	0x8003272 <raylar_crcl_encoder_service::benchmark::transform_budgeted::h9752c8449280daff+0x5c> @ imm = #-0x50
 80032c0: 2a00         	cmp	r2, #0x0
 80032c2: d1d6         	bne	0x8003272 <raylar_crcl_encoder_service::benchmark::transform_budgeted::h9752c8449280daff+0x5c> @ imm = #-0x54
 80032c4: ac02         	add	r4, sp, #0x8
 80032c6: f246 0128    	movw	r1, #0x6028
 80032ca: 1845         	adds	r5, r0, r1
 80032cc: f104 0048    	add.w	r0, r4, #0x48
 80032d0: 2128         	movs	r1, #0x28
 80032d2: f009 fd06    	bl	0x800cce2 <__aeabi_memclr4> @ imm = #0x9a0c
 80032d6: ab27         	add	r3, sp, #0x9c
 80032d8: 9512         	str	r5, [sp, #0x48]
 80032da: f44f 56c0    	mov.w	r6, #0x1800
 80032de: f104 0840    	add.w	r8, r4, #0x40
 80032e2: cb0e         	ldm	r3, {r1, r2, r3}
 80032e4: ad27         	add	r5, sp, #0x9c
 80032e6: 2000         	movs	r0, #0x0
 80032e8: 9613         	str	r6, [sp, #0x4c]
 80032ea: f44f 7680    	mov.w	r6, #0x100
 80032ee: e9cd 2324    	strd	r2, r3, [sp, #144]
 80032f2: 2201         	movs	r2, #0x1
 80032f4: f8ad 0024    	strh.w	r0, [sp, #0x24]
 80032f8: e9cd 0007    	strd	r0, r0, [sp, #28]
 80032fc: e9cd 0005    	strd	r0, r0, [sp, #20]
 8003300: e9cd 0003    	strd	r0, r0, [sp, #12]
 8003304: e9cd 000b    	strd	r0, r0, [sp, #44]
 8003308: e9cd 000d    	strd	r0, r0, [sp, #52]
 800330c: e9cd 000f    	strd	r0, r0, [sp, #60]
 8003310: f8ad 0044    	strh.w	r0, [sp, #0x44]
 8003314: e9cd 001f    	strd	r0, r0, [sp, #124]
 8003318: e9cd 0021    	strd	r0, r0, [sp, #132]
 800331c: 9002         	str	r0, [sp, #0x8]
 800331e: 900a         	str	r0, [sp, #0x28]
 8003320: 921e         	str	r2, [sp, #0x78]
 8003322: 9123         	str	r1, [sp, #0x8c]
 8003324: f8ad 0098    	strh.w	r0, [sp, #0x98]
 8003328: 9627         	str	r6, [sp, #0x9c]
 800332a: 4620         	mov	r0, r4
 800332c: 9927         	ldr	r1, [sp, #0x9c]
 800332e: f7fe fb90    	bl	0x8001a52 <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3> @ imm = #-0x18e0
 8003332: 07c0         	lsls	r0, r0, #0x1f
 8003334: d109         	bne	0x800334a <raylar_crcl_encoder_service::benchmark::transform_budgeted::h9752c8449280daff+0x134> @ imm = #0x12
 8003336: 07c8         	lsls	r0, r1, #0x1f
 8003338: d0f6         	beq	0x8003328 <raylar_crcl_encoder_service::benchmark::transform_budgeted::h9752c8449280daff+0x112> @ imm = #-0x14
 800333a: f8cd 809c    	str.w	r8, [sp, #0x9c]
 800333e: a827         	add	r0, sp, #0x9c
 8003340: 9814         	ldr	r0, [sp, #0x50]
 8003342: b02a         	add	sp, #0xa8
 8003344: f85d 8b04    	ldr	r8, [sp], #4
 8003348: bdf0         	pop	{r4, r5, r6, r7, pc}
 800334a: f64d 60e0    	movw	r0, #0xdee0
 800334e: f64d 53e4    	movw	r3, #0xdde4
 8003352: f6c0 0000    	movt	r0, #0x800
 8003356: aa27         	add	r2, sp, #0x9c
 8003358: 9000         	str	r0, [sp]
 800335a: f64d 5048    	movw	r0, #0xdd48
 800335e: f88d 109c    	strb.w	r1, [sp, #0x9c]
 8003362: f6c0 0000    	movt	r0, #0x800
 8003366: f6c0 0300    	movt	r3, #0x800
 800336a: 212b         	movs	r1, #0x2b
 800336c: f008 fa54    	bl	0x800b818 <core::result::unwrap_failed> @ imm = #0x84a8

08003370 <raylar_crcl_encoder_service::benchmark::serialize_production::h9a703500dd702791>:
 8003370: b5f0         	push	{r4, r5, r6, r7, lr}
 8003372: af03         	add	r7, sp, #0xc
 8003374: e92d 0b00    	push.w	{r8, r9, r11}
 8003378: b0b8         	sub	sp, #0xe0
 800337a: f240 6c0a    	movw	r12, #0x60a
 800337e: f44f 0100    	mov.w	r1, #0x800000
 8003382: f2c0 1c01    	movt	r12, #0x101
 8003386: 9127         	str	r1, [sp, #0x9c]
 8003388: f44f 6180    	mov.w	r1, #0x400
 800338c: f8cd c0a0    	str.w	r12, [sp, #0xa0]
 8003390: 9126         	str	r1, [sp, #0x98]
 8003392: f10d 0898    	add.w	r8, sp, #0x98
 8003396: 9926         	ldr	r1, [sp, #0x98]
 8003398: 1e4a         	subs	r2, r1, #0x1
 800339a: ea81 0302    	eor.w	r3, r1, r2
 800339e: 4293         	cmp	r3, r2
 80033a0: d916         	bls	0x80033d0 <raylar_crcl_encoder_service::benchmark::serialize_production::h9a703500dd702791+0x60> @ imm = #0x2c
 80033a2: fa91 f2a1    	rbit	r2, r1
 80033a6: fab2 f282    	clz	r2, r2
 80033aa: 3a08         	subs	r2, #0x8
 80033ac: 2a04         	cmp	r2, #0x4
 80033ae: bf9c         	itt	ls
 80033b0: f89d 20a0    	ldrbls.w	r2, [sp, #0xa0]
 80033b4: 2a0c         	cmpls	r2, #0xc
 80033b6: d80b         	bhi	0x80033d0 <raylar_crcl_encoder_service::benchmark::serialize_production::h9a703500dd702791+0x60> @ imm = #0x16
 80033b8: 2301         	movs	r3, #0x1
 80033ba: fa03 f202    	lsl.w	r2, r3, r2
 80033be: f412 5faa    	tst.w	r2, #0x1540
 80033c2: bf1e         	ittt	ne
 80033c4: 9a27         	ldrne	r2, [sp, #0x9c]
 80033c6: f102 4240    	addne.w	r2, r2, #0xc0000000
 80033ca: f1b2 4f40    	cmpne.w	r2, #0xc0000000
 80033ce: d813         	bhi	0x80033f8 <raylar_crcl_encoder_service::benchmark::serialize_production::h9a703500dd702791+0x88> @ imm = #0x26
 80033d0: 2000         	movs	r0, #0x0
 80033d2: f88d 0098    	strb.w	r0, [sp, #0x98]
 80033d6: f64d 7030    	movw	r0, #0xdf30
 80033da: f6c0 0000    	movt	r0, #0x800
 80033de: f64d 53e4    	movw	r3, #0xdde4
 80033e2: 9000         	str	r0, [sp]
 80033e4: f64d 5048    	movw	r0, #0xdd48
 80033e8: aa26         	add	r2, sp, #0x98
 80033ea: f6c0 0000    	movt	r0, #0x800
 80033ee: f6c0 0300    	movt	r3, #0x800
 80033f2: 212b         	movs	r1, #0x2b
 80033f4: f008 fa10    	bl	0x800b818 <core::result::unwrap_failed> @ imm = #0x8420
 80033f8: f89d 20a1    	ldrb.w	r2, [sp, #0xa1]
 80033fc: 2a1e         	cmp	r2, #0x1e
 80033fe: d8e7         	bhi	0x80033d0 <raylar_crcl_encoder_service::benchmark::serialize_production::h9a703500dd702791+0x60> @ imm = #-0x32
 8003400: f89d 30a3    	ldrb.w	r3, [sp, #0xa3]
 8003404: 4681         	mov	r9, r0
 8003406: 2b01         	cmp	r3, #0x1
 8003408: d107         	bne	0x800341a <raylar_crcl_encoder_service::benchmark::serialize_production::h9a703500dd702791+0xaa> @ imm = #0xe
 800340a: f5b1 6f80    	cmp.w	r1, #0x400
 800340e: d3df         	blo	0x80033d0 <raylar_crcl_encoder_service::benchmark::serialize_production::h9a703500dd702791+0x60> @ imm = #-0x42
 8003410: f89d 10a2    	ldrb.w	r1, [sp, #0xa2]
 8003414: 2901         	cmp	r1, #0x1
 8003416: d005         	beq	0x8003424 <raylar_crcl_encoder_service::benchmark::serialize_production::h9a703500dd702791+0xb4> @ imm = #0xa
 8003418: e7da         	b	0x80033d0 <raylar_crcl_encoder_service::benchmark::serialize_production::h9a703500dd702791+0x60> @ imm = #-0x4c
 800341a: f5b1 6f80    	cmp.w	r1, #0x400
 800341e: d3d7         	blo	0x80033d0 <raylar_crcl_encoder_service::benchmark::serialize_production::h9a703500dd702791+0x60> @ imm = #-0x52
 8003420: 2a00         	cmp	r2, #0x0
 8003422: d1d5         	bne	0x80033d0 <raylar_crcl_encoder_service::benchmark::serialize_production::h9a703500dd702791+0x60> @ imm = #-0x56
 8003424: f246 0128    	movw	r1, #0x6028
 8003428: eb09 0201    	add.w	r2, r9, r1
 800342c: 9211         	str	r2, [sp, #0x44]
 800342e: f44f 52c0    	mov.w	r2, #0x1800
 8003432: f04f 0e00    	mov.w	lr, #0x0
 8003436: e9dd 3626    	ldrd	r3, r6, [sp, #152]
 800343a: 9d28         	ldr	r5, [sp, #0xa0]
 800343c: 9212         	str	r2, [sp, #0x48]
 800343e: 2206         	movs	r2, #0x6
 8003440: e9cd 3622    	strd	r3, r6, [sp, #136]
 8003444: 464b         	mov	r3, r9
 8003446: 921d         	str	r2, [sp, #0x74]
 8003448: aa01         	add	r2, sp, #0x4
 800344a: f8ad e020    	strh.w	lr, [sp, #0x20]
 800344e: 3248         	adds	r2, #0x48
 8003450: e9cd ee06    	strd	lr, lr, [sp, #24]
 8003454: e9cd ee04    	strd	lr, lr, [sp, #16]
 8003458: e9cd ee02    	strd	lr, lr, [sp, #8]
 800345c: e9cd ee0a    	strd	lr, lr, [sp, #40]
 8003460: e9cd ee0c    	strd	lr, lr, [sp, #48]
 8003464: e9cd ee0e    	strd	lr, lr, [sp, #56]
 8003468: f8ad e040    	strh.w	lr, [sp, #0x40]
 800346c: e9cd ee1e    	strd	lr, lr, [sp, #120]
 8003470: e9cd ee20    	strd	lr, lr, [sp, #128]
 8003474: f8cd e004    	str.w	lr, [sp, #0x4]
 8003478: f8cd e024    	str.w	lr, [sp, #0x24]
 800347c: 9524         	str	r5, [sp, #0x90]
 800347e: f8ad e094    	strh.w	lr, [sp, #0x94]
 8003482: cb73         	ldm	r3!, {r0, r1, r4, r5, r6}
 8003484: c273         	stm	r2!, {r0, r1, r4, r5, r6}
 8003486: e893 0073    	ldm.w	r3, {r0, r1, r4, r5, r6}
 800348a: c273         	stm	r2!, {r0, r1, r4, r5, r6}
 800348c: f44f 0000    	mov.w	r0, #0x800000
 8003490: 9036         	str	r0, [sp, #0xd8]
 8003492: f44f 6080    	mov.w	r0, #0x400
 8003496: 9035         	str	r0, [sp, #0xd4]
 8003498: a835         	add	r0, sp, #0xd4
 800349a: f8cd c0dc    	str.w	r12, [sp, #0xdc]
 800349e: 9835         	ldr	r0, [sp, #0xd4]
 80034a0: 1e41         	subs	r1, r0, #0x1
 80034a2: ea80 0201    	eor.w	r2, r0, r1
 80034a6: 428a         	cmp	r2, r1
 80034a8: d90a         	bls	0x80034c0 <raylar_crcl_encoder_service::benchmark::serialize_production::h9a703500dd702791+0x150> @ imm = #0x14
 80034aa: fa90 f0a0    	rbit	r0, r0
 80034ae: fab0 f080    	clz	r0, r0
 80034b2: 3808         	subs	r0, #0x8
 80034b4: 2804         	cmp	r0, #0x4
 80034b6: bf9c         	itt	ls
 80034b8: f89d 00dc    	ldrbls.w	r0, [sp, #0xdc]
 80034bc: 280c         	cmpls	r0, #0xc
 80034be: d906         	bls	0x80034ce <raylar_crcl_encoder_service::benchmark::serialize_production::h9a703500dd702791+0x15e> @ imm = #0xc
 80034c0: f64d 7020    	movw	r0, #0xdf20
 80034c4: f88d e098    	strb.w	lr, [sp, #0x98]
 80034c8: f6c0 0000    	movt	r0, #0x800
 80034cc: e787         	b	0x80033de <raylar_crcl_encoder_service::benchmark::serialize_production::h9a703500dd702791+0x6e> @ imm = #-0xf2
 80034ce: 2101         	movs	r1, #0x1
 80034d0: fa01 f000    	lsl.w	r0, r1, r0
 80034d4: f410 5faa    	tst.w	r0, #0x1540
 80034d8: d0f2         	beq	0x80034c0 <raylar_crcl_encoder_service::benchmark::serialize_production::h9a703500dd702791+0x150> @ imm = #-0x1c
 80034da: 9936         	ldr	r1, [sp, #0xd8]
 80034dc: 2001         	movs	r0, #0x1
 80034de: f2cc 0000    	movt	r0, #0xc000
 80034e2: f101 4140    	add.w	r1, r1, #0xc0000000
 80034e6: 4281         	cmp	r1, r0
 80034e8: d3ea         	blo	0x80034c0 <raylar_crcl_encoder_service::benchmark::serialize_production::h9a703500dd702791+0x150> @ imm = #-0x2c
 80034ea: f89d 00dd    	ldrb.w	r0, [sp, #0xdd]
 80034ee: 281e         	cmp	r0, #0x1e
 80034f0: d8e6         	bhi	0x80034c0 <raylar_crcl_encoder_service::benchmark::serialize_production::h9a703500dd702791+0x150> @ imm = #-0x34
 80034f2: f89d 60df    	ldrb.w	r6, [sp, #0xdf]
 80034f6: 2e01         	cmp	r6, #0x1
 80034f8: d104         	bne	0x8003504 <raylar_crcl_encoder_service::benchmark::serialize_production::h9a703500dd702791+0x194> @ imm = #0x8
 80034fa: f89d 00de    	ldrb.w	r0, [sp, #0xde]
 80034fe: 2801         	cmp	r0, #0x1
 8003500: d002         	beq	0x8003508 <raylar_crcl_encoder_service::benchmark::serialize_production::h9a703500dd702791+0x198> @ imm = #0x4
 8003502: e7dd         	b	0x80034c0 <raylar_crcl_encoder_service::benchmark::serialize_production::h9a703500dd702791+0x150> @ imm = #-0x46
 8003504: 2800         	cmp	r0, #0x0
 8003506: d1db         	bne	0x80034c0 <raylar_crcl_encoder_service::benchmark::serialize_production::h9a703500dd702791+0x150> @ imm = #-0x4a
 8003508: e9dd 0135    	ldrd	r0, r1, [sp, #212]
 800350c: f8dd 20db    	ldr.w	r2, [sp, #0xdb]
 8003510: f8cd 00c7    	str.w	r0, [sp, #0xc7]
 8003514: 2000         	movs	r0, #0x0
 8003516: f8cd 20ce    	str.w	r2, [sp, #0xce]
 800351a: 221a         	movs	r2, #0x1a
 800351c: f8cd 10cb    	str.w	r1, [sp, #0xcb]
 8003520: a92e         	add	r1, sp, #0xb8
 8003522: 9030         	str	r0, [sp, #0xc0]
 8003524: e9cd 002e    	strd	r0, r0, [sp, #184]
 8003528: f8cd 00c3    	str.w	r0, [sp, #0xc3]
 800352c: f88d 0098    	strb.w	r0, [sp, #0x98]
 8003530: f108 0001    	add.w	r0, r8, #0x1
 8003534: f009 f9ac    	bl	0x800c890 <__aeabi_memcpy> @ imm = #0x9358
 8003538: f641 6058    	movw	r0, #0x1e58
 800353c: f509 34e0    	add.w	r4, r9, #0x1c000
 8003540: 9000         	str	r0, [sp]
 8003542: a82e         	add	r0, sp, #0xb8
 8003544: aa01         	add	r2, sp, #0x4
 8003546: 4641         	mov	r1, r8
 8003548: 4623         	mov	r3, r4
 800354a: f88d 60b3    	strb.w	r6, [sp, #0xb3]
 800354e: f7fd ff99    	bl	0x8001484 <raylar_crcl_encoder_service::format::Encoder::data::ha65fe528befbf4e3> @ imm = #-0x20ce
 8003552: f89d 00b8    	ldrb.w	r0, [sp, #0xb8]
 8003556: 2801         	cmp	r0, #0x1
 8003558: d015         	beq	0x8003586 <raylar_crcl_encoder_service::benchmark::serialize_production::h9a703500dd702791+0x216> @ imm = #0x2a
 800355a: 992f         	ldr	r1, [sp, #0xbc]
 800355c: f641 6059    	movw	r0, #0x1e59
 8003560: 4281         	cmp	r1, r0
 8003562: d207         	bhs	0x8003574 <raylar_crcl_encoder_service::benchmark::serialize_production::h9a703500dd702791+0x204> @ imm = #0xe
 8003564: e9cd 4126    	strd	r4, r1, [sp, #152]
 8003568: a826         	add	r0, sp, #0x98
 800356a: 4608         	mov	r0, r1
 800356c: b038         	add	sp, #0xe0
 800356e: e8bd 0b00    	pop.w	{r8, r9, r11}
 8003572: bdf0         	pop	{r4, r5, r6, r7, pc}
 8003574: f64d 7300    	movw	r3, #0xdf00
 8003578: 2000         	movs	r0, #0x0
 800357a: f6c0 0300    	movt	r3, #0x800
 800357e: f641 6258    	movw	r2, #0x1e58
 8003582: f008 fc54    	bl	0x800be2e <core::slice::index::slice_index_fail> @ imm = #0x88a8
 8003586: f89d 00b9    	ldrb.w	r0, [sp, #0xb9]
 800358a: f64d 53e4    	movw	r3, #0xdde4
 800358e: f88d 00d4    	strb.w	r0, [sp, #0xd4]
 8003592: f64d 7010    	movw	r0, #0xdf10
 8003596: f6c0 0000    	movt	r0, #0x800
 800359a: aa35         	add	r2, sp, #0xd4
 800359c: 9000         	str	r0, [sp]
 800359e: f64d 5048    	movw	r0, #0xdd48
 80035a2: f6c0 0000    	movt	r0, #0x800
 80035a6: f6c0 0300    	movt	r3, #0x800
 80035aa: 212b         	movs	r1, #0x2b
 80035ac: f008 f934    	bl	0x800b818 <core::result::unwrap_failed> @ imm = #0x8268

080035b0 <raylar_crcl_encoder_service::benchmark::transform_production::he43415fd0806c08a>:
 80035b0: b5f0         	push	{r4, r5, r6, r7, lr}
 80035b2: af03         	add	r7, sp, #0xc
 80035b4: f84d 8d04    	str	r8, [sp, #-4]!
 80035b8: b0aa         	sub	sp, #0xa8
 80035ba: f240 610a    	movw	r1, #0x60a
 80035be: f2c0 1101    	movt	r1, #0x101
 80035c2: 9129         	str	r1, [sp, #0xa4]
 80035c4: f44f 0100    	mov.w	r1, #0x800000
 80035c8: 9128         	str	r1, [sp, #0xa0]
 80035ca: f44f 6180    	mov.w	r1, #0x400
 80035ce: 9127         	str	r1, [sp, #0x9c]
 80035d0: a927         	add	r1, sp, #0x9c
 80035d2: 9927         	ldr	r1, [sp, #0x9c]
 80035d4: 1e4a         	subs	r2, r1, #0x1
 80035d6: ea81 0302    	eor.w	r3, r1, r2
 80035da: 4293         	cmp	r3, r2
 80035dc: d916         	bls	0x800360c <raylar_crcl_encoder_service::benchmark::transform_production::he43415fd0806c08a+0x5c> @ imm = #0x2c
 80035de: fa91 f2a1    	rbit	r2, r1
 80035e2: fab2 f282    	clz	r2, r2
 80035e6: 3a08         	subs	r2, #0x8
 80035e8: 2a04         	cmp	r2, #0x4
 80035ea: bf9c         	itt	ls
 80035ec: f89d 20a4    	ldrbls.w	r2, [sp, #0xa4]
 80035f0: 2a0c         	cmpls	r2, #0xc
 80035f2: d80b         	bhi	0x800360c <raylar_crcl_encoder_service::benchmark::transform_production::he43415fd0806c08a+0x5c> @ imm = #0x16
 80035f4: 2301         	movs	r3, #0x1
 80035f6: fa03 f202    	lsl.w	r2, r3, r2
 80035fa: f412 5faa    	tst.w	r2, #0x1540
 80035fe: bf1e         	ittt	ne
 8003600: 9a28         	ldrne	r2, [sp, #0xa0]
 8003602: f102 4240    	addne.w	r2, r2, #0xc0000000
 8003606: f1b2 4f40    	cmpne.w	r2, #0xc0000000
 800360a: d813         	bhi	0x8003634 <raylar_crcl_encoder_service::benchmark::transform_production::he43415fd0806c08a+0x84> @ imm = #0x26
 800360c: 2000         	movs	r0, #0x0
 800360e: f64d 53e4    	movw	r3, #0xdde4
 8003612: f88d 0008    	strb.w	r0, [sp, #0x8]
 8003616: f64d 7050    	movw	r0, #0xdf50
 800361a: f6c0 0000    	movt	r0, #0x800
 800361e: aa02         	add	r2, sp, #0x8
 8003620: 9000         	str	r0, [sp]
 8003622: f64d 5048    	movw	r0, #0xdd48
 8003626: f6c0 0000    	movt	r0, #0x800
 800362a: f6c0 0300    	movt	r3, #0x800
 800362e: 212b         	movs	r1, #0x2b
 8003630: f008 f8f2    	bl	0x800b818 <core::result::unwrap_failed> @ imm = #0x81e4
 8003634: f89d 20a5    	ldrb.w	r2, [sp, #0xa5]
 8003638: 2a1e         	cmp	r2, #0x1e
 800363a: d8e7         	bhi	0x800360c <raylar_crcl_encoder_service::benchmark::transform_production::he43415fd0806c08a+0x5c> @ imm = #-0x32
 800363c: f89d 30a7    	ldrb.w	r3, [sp, #0xa7]
 8003640: 2b01         	cmp	r3, #0x1
 8003642: d107         	bne	0x8003654 <raylar_crcl_encoder_service::benchmark::transform_production::he43415fd0806c08a+0xa4> @ imm = #0xe
 8003644: f5b1 6f80    	cmp.w	r1, #0x400
 8003648: d3e0         	blo	0x800360c <raylar_crcl_encoder_service::benchmark::transform_production::he43415fd0806c08a+0x5c> @ imm = #-0x40
 800364a: f89d 10a6    	ldrb.w	r1, [sp, #0xa6]
 800364e: 2901         	cmp	r1, #0x1
 8003650: d005         	beq	0x800365e <raylar_crcl_encoder_service::benchmark::transform_production::he43415fd0806c08a+0xae> @ imm = #0xa
 8003652: e7db         	b	0x800360c <raylar_crcl_encoder_service::benchmark::transform_production::he43415fd0806c08a+0x5c> @ imm = #-0x4a
 8003654: f5b1 6f80    	cmp.w	r1, #0x400
 8003658: d3d8         	blo	0x800360c <raylar_crcl_encoder_service::benchmark::transform_production::he43415fd0806c08a+0x5c> @ imm = #-0x50
 800365a: 2a00         	cmp	r2, #0x0
 800365c: d1d6         	bne	0x800360c <raylar_crcl_encoder_service::benchmark::transform_production::he43415fd0806c08a+0x5c> @ imm = #-0x54
 800365e: ac02         	add	r4, sp, #0x8
 8003660: f246 0128    	movw	r1, #0x6028
 8003664: 1845         	adds	r5, r0, r1
 8003666: f104 0048    	add.w	r0, r4, #0x48
 800366a: 2128         	movs	r1, #0x28
 800366c: f009 fb39    	bl	0x800cce2 <__aeabi_memclr4> @ imm = #0x9672
 8003670: ab27         	add	r3, sp, #0x9c
 8003672: 9512         	str	r5, [sp, #0x48]
 8003674: f44f 56c0    	mov.w	r6, #0x1800
 8003678: f104 0840    	add.w	r8, r4, #0x40
 800367c: cb0e         	ldm	r3, {r1, r2, r3}
 800367e: ad27         	add	r5, sp, #0x9c
 8003680: 2000         	movs	r0, #0x0
 8003682: 9613         	str	r6, [sp, #0x4c]
 8003684: f04f 36ff    	mov.w	r6, #0xffffffff
 8003688: e9cd 2324    	strd	r2, r3, [sp, #144]
 800368c: 2201         	movs	r2, #0x1
 800368e: f8ad 0024    	strh.w	r0, [sp, #0x24]
 8003692: e9cd 0007    	strd	r0, r0, [sp, #28]
 8003696: e9cd 0005    	strd	r0, r0, [sp, #20]
 800369a: e9cd 0003    	strd	r0, r0, [sp, #12]
 800369e: e9cd 000b    	strd	r0, r0, [sp, #44]
 80036a2: e9cd 000d    	strd	r0, r0, [sp, #52]
 80036a6: e9cd 000f    	strd	r0, r0, [sp, #60]
 80036aa: f8ad 0044    	strh.w	r0, [sp, #0x44]
 80036ae: e9cd 001f    	strd	r0, r0, [sp, #124]
 80036b2: e9cd 0021    	strd	r0, r0, [sp, #132]
 80036b6: 9002         	str	r0, [sp, #0x8]
 80036b8: 900a         	str	r0, [sp, #0x28]
 80036ba: 921e         	str	r2, [sp, #0x78]
 80036bc: 9123         	str	r1, [sp, #0x8c]
 80036be: f8ad 0098    	strh.w	r0, [sp, #0x98]
 80036c2: 9627         	str	r6, [sp, #0x9c]
 80036c4: 4620         	mov	r0, r4
 80036c6: 9927         	ldr	r1, [sp, #0x9c]
 80036c8: f7fe f9c3    	bl	0x8001a52 <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3> @ imm = #-0x1c7a
 80036cc: 07c0         	lsls	r0, r0, #0x1f
 80036ce: d109         	bne	0x80036e4 <raylar_crcl_encoder_service::benchmark::transform_production::he43415fd0806c08a+0x134> @ imm = #0x12
 80036d0: 07c8         	lsls	r0, r1, #0x1f
 80036d2: d0f6         	beq	0x80036c2 <raylar_crcl_encoder_service::benchmark::transform_production::he43415fd0806c08a+0x112> @ imm = #-0x14
 80036d4: f8cd 809c    	str.w	r8, [sp, #0x9c]
 80036d8: a827         	add	r0, sp, #0x9c
 80036da: 9814         	ldr	r0, [sp, #0x50]
 80036dc: b02a         	add	sp, #0xa8
 80036de: f85d 8b04    	ldr	r8, [sp], #4
 80036e2: bdf0         	pop	{r4, r5, r6, r7, pc}
 80036e4: f64d 7040    	movw	r0, #0xdf40
 80036e8: f64d 53e4    	movw	r3, #0xdde4
 80036ec: f6c0 0000    	movt	r0, #0x800
 80036f0: aa27         	add	r2, sp, #0x9c
 80036f2: 9000         	str	r0, [sp]
 80036f4: f64d 5048    	movw	r0, #0xdd48
 80036f8: f88d 109c    	strb.w	r1, [sp, #0x9c]
 80036fc: f6c0 0000    	movt	r0, #0x800
 8003700: f6c0 0300    	movt	r3, #0x800
 8003704: 212b         	movs	r1, #0x2b
 8003706: f008 f887    	bl	0x800b818 <core::result::unwrap_failed> @ imm = #0x810e

0800370a <raylar_crcl_encoder_service::benchmark::raw::hd2aad415d886ff2c>:
 800370a: b580         	push	{r7, lr}
 800370c: 466f         	mov	r7, sp
 800370e: f246 0128    	movw	r1, #0x6028
 8003712: 1842         	adds	r2, r0, r1
 8003714: f100 0128    	add.w	r1, r0, #0x28
 8003718: 4610         	mov	r0, r2
 800371a: f44f 42c0    	mov.w	r2, #0x6000
 800371e: f009 fa92    	bl	0x800cc46 <__aeabi_memcpy4> @ imm = #0x9524
 8003722: bd80         	pop	{r7, pc}

08003724 <raylar_crcl_encoder_service::benchmark::copy::hf53d4786c081c67d>:
 8003724: b5f0         	push	{r4, r5, r6, r7, lr}
 8003726: af03         	add	r7, sp, #0xc
 8003728: f84d bd04    	str	r11, [sp, #-4]!
 800372c: b082         	sub	sp, #0x8
 800372e: f246 0628    	movw	r6, #0x6028
 8003732: 1985         	adds	r5, r0, r6
 8003734: f100 0128    	add.w	r1, r0, #0x28
 8003738: 4604         	mov	r4, r0
 800373a: 4628         	mov	r0, r5
 800373c: f44f 42c0    	mov.w	r2, #0x6000
 8003740: f009 fa81    	bl	0x800cc46 <__aeabi_memcpy4> @ imm = #0x9502
 8003744: 9501         	str	r5, [sp, #0x4]
 8003746: a801         	add	r0, sp, #0x4
 8003748: 59a0         	ldr	r0, [r4, r6]
 800374a: b002         	add	sp, #0x8
 800374c: f85d bb04    	ldr	r11, [sp], #4
 8003750: bdf0         	pop	{r4, r5, r6, r7, pc}

08003752 <raylar_crcl_encoder_service::benchmark::peak::he0ae015b97d7ff8e>:
 8003752: b5f0         	push	{r4, r5, r6, r7, lr}
 8003754: af03         	add	r7, sp, #0xc
 8003756: e92d 0f00    	push.w	{r8, r9, r10, r11}
 800375a: b087         	sub	sp, #0x1c
 800375c: f04f 0a00    	mov.w	r10, #0x0
 8003760: f46f 0900    	mvn	r9, #0x800000
 8003764: f06f 7880    	mvn	r8, #0x1000000
 8003768: f04f 0b00    	mov.w	r11, #0x0
 800376c: f246 018c    	movw	r1, #0x608c
 8003770: 9004         	str	r0, [sp, #0x10]
 8003772: 4408         	add	r0, r1
 8003774: 9005         	str	r0, [sp, #0x14]
 8003776: 9805         	ldr	r0, [sp, #0x14]
 8003778: f64f 72fe    	movw	r2, #0xfffe
 800377c: f6cf 62ff    	movt	r2, #0xfeff
 8003780: eb00 060b    	add.w	r6, r0, r11
 8003784: f856 0c60    	ldr	r0, [r6, #-96]
 8003788: eb00 0109    	add.w	r1, r0, r9
 800378c: 4291         	cmp	r1, r2
 800378e: f240 8281    	bls.w	0x8003c94 <raylar_crcl_encoder_service::benchmark::peak::he0ae015b97d7ff8e+0x542> @ imm = #0x502
 8003792: f856 1c48    	ldr	r1, [r6, #-72]
 8003796: eb01 0209    	add.w	r2, r1, r9
 800379a: 4542         	cmp	r2, r8
 800379c: f0c0 827a    	blo.w	0x8003c94 <raylar_crcl_encoder_service::benchmark::peak::he0ae015b97d7ff8e+0x542> @ imm = #0x4f4
 80037a0: f856 5c30    	ldr	r5, [r6, #-48]
 80037a4: eb05 0209    	add.w	r2, r5, r9
 80037a8: 4542         	cmp	r2, r8
 80037aa: bf22         	ittt	hs
 80037ac: f856 2c18    	ldrhs	r2, [r6, #-24]
 80037b0: eb02 0309    	addhs.w	r3, r2, r9
 80037b4: 4543         	cmphs	r3, r8
 80037b6: f0c0 826d    	blo.w	0x8003c94 <raylar_crcl_encoder_service::benchmark::peak::he0ae015b97d7ff8e+0x542> @ imm = #0x4da
 80037ba: 9b05         	ldr	r3, [sp, #0x14]
 80037bc: f853 300b    	ldr.w	r3, [r3, r11]
 80037c0: eb03 0409    	add.w	r4, r3, r9
 80037c4: 4544         	cmp	r4, r8
 80037c6: bf22         	ittt	hs
 80037c8: f8d6 e018    	ldrhs.w	lr, [r6, #0x18]
 80037cc: eb0e 0409    	addhs.w	r4, lr, r9
 80037d0: 4544         	cmphs	r4, r8
 80037d2: f0c0 825f    	blo.w	0x8003c94 <raylar_crcl_encoder_service::benchmark::peak::he0ae015b97d7ff8e+0x542> @ imm = #0x4be
 80037d6: f8d6 c030    	ldr.w	r12, [r6, #0x30]
 80037da: eb0c 0409    	add.w	r4, r12, r9
 80037de: 4544         	cmp	r4, r8
 80037e0: bf22         	ittt	hs
 80037e2: 6cb6         	ldrhs	r6, [r6, #0x48]
 80037e4: eb06 0409    	addhs.w	r4, r6, r9
 80037e8: 4544         	cmphs	r4, r8
 80037ea: f0c0 8253    	blo.w	0x8003c94 <raylar_crcl_encoder_service::benchmark::peak::he0ae015b97d7ff8e+0x542> @ imm = #0x4a6
 80037ee: 2800         	cmp	r0, #0x0
 80037f0: bf48         	it	mi
 80037f2: 4240         	rsbmi	r0, r0, #0
 80037f4: 4550         	cmp	r0, r10
 80037f6: bf98         	it	ls
 80037f8: 4650         	movls	r0, r10
 80037fa: 2900         	cmp	r1, #0x0
 80037fc: bf48         	it	mi
 80037fe: 4249         	rsbmi	r1, r1, #0
 8003800: 4281         	cmp	r1, r0
 8003802: bf98         	it	ls
 8003804: 4601         	movls	r1, r0
 8003806: 2d00         	cmp	r5, #0x0
 8003808: bf48         	it	mi
 800380a: 426d         	rsbmi	r5, r5, #0
 800380c: 428d         	cmp	r5, r1
 800380e: bf98         	it	ls
 8003810: 460d         	movls	r5, r1
 8003812: 2a00         	cmp	r2, #0x0
 8003814: bf48         	it	mi
 8003816: 4252         	rsbmi	r2, r2, #0
 8003818: 42aa         	cmp	r2, r5
 800381a: bf98         	it	ls
 800381c: 462a         	movls	r2, r5
 800381e: 2b00         	cmp	r3, #0x0
 8003820: bf48         	it	mi
 8003822: 425b         	rsbmi	r3, r3, #0
 8003824: 4293         	cmp	r3, r2
 8003826: bf98         	it	ls
 8003828: 4613         	movls	r3, r2
 800382a: f1be 0f00    	cmp.w	lr, #0x0
 800382e: bf48         	it	mi
 8003830: f1ce 0e00    	rsbmi.w	lr, lr, #0x0
 8003834: 459e         	cmp	lr, r3
 8003836: bf98         	it	ls
 8003838: 469e         	movls	lr, r3
 800383a: f1bc 0f00    	cmp.w	r12, #0x0
 800383e: bf48         	it	mi
 8003840: f1cc 0c00    	rsbmi.w	r12, r12, #0x0
 8003844: 45f4         	cmp	r12, lr
 8003846: bf98         	it	ls
 8003848: 46f4         	movls	r12, lr
 800384a: 2e00         	cmp	r6, #0x0
 800384c: bf48         	it	mi
 800384e: 4276         	rsbmi	r6, r6, #0
 8003850: f10b 0bc0    	add.w	r11, r11, #0xc0
 8003854: 4566         	cmp	r6, r12
 8003856: bf98         	it	ls
 8003858: 4666         	movls	r6, r12
 800385a: 46b2         	mov	r10, r6
 800385c: f5bb 4fc0    	cmp.w	r11, #0x6000
 8003860: d189         	bne	0x8003776 <raylar_crcl_encoder_service::benchmark::peak::he0ae015b97d7ff8e+0x24> @ imm = #-0xee
 8003862: 9606         	str	r6, [sp, #0x18]
 8003864: a806         	add	r0, sp, #0x18
 8003866: f246 0090    	movw	r0, #0x6090
 800386a: f04f 0900    	mov.w	r9, #0x0
 800386e: 9906         	ldr	r1, [sp, #0x18]
 8003870: f46f 0e00    	mvn	lr, #0x800000
 8003874: 9103         	str	r1, [sp, #0xc]
 8003876: f06f 7880    	mvn	r8, #0x1000000
 800387a: 9904         	ldr	r1, [sp, #0x10]
 800387c: f04f 0a00    	mov.w	r10, #0x0
 8003880: 4408         	add	r0, r1
 8003882: 9005         	str	r0, [sp, #0x14]
 8003884: 9805         	ldr	r0, [sp, #0x14]
 8003886: eb00 060a    	add.w	r6, r0, r10
 800388a: f856 0c60    	ldr	r0, [r6, #-96]
 800388e: eb00 010e    	add.w	r1, r0, lr
 8003892: 4541         	cmp	r1, r8
 8003894: bf22         	ittt	hs
 8003896: f856 1c48    	ldrhs	r1, [r6, #-72]
 800389a: eb01 020e    	addhs.w	r2, r1, lr
 800389e: 4542         	cmphs	r2, r8
 80038a0: f0c0 81f8    	blo.w	0x8003c94 <raylar_crcl_encoder_service::benchmark::peak::he0ae015b97d7ff8e+0x542> @ imm = #0x3f0
 80038a4: f856 2c30    	ldr	r2, [r6, #-48]
 80038a8: eb02 030e    	add.w	r3, r2, lr
 80038ac: 4543         	cmp	r3, r8
 80038ae: bf22         	ittt	hs
 80038b0: f856 3c18    	ldrhs	r3, [r6, #-24]
 80038b4: eb03 040e    	addhs.w	r4, r3, lr
 80038b8: 4544         	cmphs	r4, r8
 80038ba: f0c0 81eb    	blo.w	0x8003c94 <raylar_crcl_encoder_service::benchmark::peak::he0ae015b97d7ff8e+0x542> @ imm = #0x3d6
 80038be: 9d05         	ldr	r5, [sp, #0x14]
 80038c0: f855 400a    	ldr.w	r4, [r5, r10]
 80038c4: eb04 050e    	add.w	r5, r4, lr
 80038c8: 4545         	cmp	r5, r8
 80038ca: bf22         	ittt	hs
 80038cc: f8d6 c018    	ldrhs.w	r12, [r6, #0x18]
 80038d0: eb0c 050e    	addhs.w	r5, r12, lr
 80038d4: 4545         	cmphs	r5, r8
 80038d6: f0c0 81dd    	blo.w	0x8003c94 <raylar_crcl_encoder_service::benchmark::peak::he0ae015b97d7ff8e+0x542> @ imm = #0x3ba
 80038da: f8d6 b030    	ldr.w	r11, [r6, #0x30]
 80038de: eb0b 050e    	add.w	r5, r11, lr
 80038e2: 4545         	cmp	r5, r8
 80038e4: bf22         	ittt	hs
 80038e6: 6cb6         	ldrhs	r6, [r6, #0x48]
 80038e8: eb06 050e    	addhs.w	r5, r6, lr
 80038ec: 4545         	cmphs	r5, r8
 80038ee: f0c0 81d1    	blo.w	0x8003c94 <raylar_crcl_encoder_service::benchmark::peak::he0ae015b97d7ff8e+0x542> @ imm = #0x3a2
 80038f2: 2800         	cmp	r0, #0x0
 80038f4: bf48         	it	mi
 80038f6: 4240         	rsbmi	r0, r0, #0
 80038f8: 4548         	cmp	r0, r9
 80038fa: bf98         	it	ls
 80038fc: 4648         	movls	r0, r9
 80038fe: 2900         	cmp	r1, #0x0
 8003900: bf48         	it	mi
 8003902: 4249         	rsbmi	r1, r1, #0
 8003904: 4281         	cmp	r1, r0
 8003906: bf98         	it	ls
 8003908: 4601         	movls	r1, r0
 800390a: 2a00         	cmp	r2, #0x0
 800390c: bf48         	it	mi
 800390e: 4252         	rsbmi	r2, r2, #0
 8003910: 428a         	cmp	r2, r1
 8003912: bf98         	it	ls
 8003914: 460a         	movls	r2, r1
 8003916: 2b00         	cmp	r3, #0x0
 8003918: bf48         	it	mi
 800391a: 425b         	rsbmi	r3, r3, #0
 800391c: 4293         	cmp	r3, r2
 800391e: bf98         	it	ls
 8003920: 4613         	movls	r3, r2
 8003922: 2c00         	cmp	r4, #0x0
 8003924: bf48         	it	mi
 8003926: 4264         	rsbmi	r4, r4, #0
 8003928: 429c         	cmp	r4, r3
 800392a: bf98         	it	ls
 800392c: 461c         	movls	r4, r3
 800392e: f1bc 0f00    	cmp.w	r12, #0x0
 8003932: bf48         	it	mi
 8003934: f1cc 0c00    	rsbmi.w	r12, r12, #0x0
 8003938: 45a4         	cmp	r12, r4
 800393a: bf98         	it	ls
 800393c: 46a4         	movls	r12, r4
 800393e: f1bb 0f00    	cmp.w	r11, #0x0
 8003942: bf48         	it	mi
 8003944: f1cb 0b00    	rsbmi.w	r11, r11, #0x0
 8003948: 45e3         	cmp	r11, r12
 800394a: bf98         	it	ls
 800394c: 46e3         	movls	r11, r12
 800394e: 2e00         	cmp	r6, #0x0
 8003950: bf48         	it	mi
 8003952: 4276         	rsbmi	r6, r6, #0
 8003954: f10a 0ac0    	add.w	r10, r10, #0xc0
 8003958: 455e         	cmp	r6, r11
 800395a: bf98         	it	ls
 800395c: 465e         	movls	r6, r11
 800395e: 46b1         	mov	r9, r6
 8003960: f5ba 4fc0    	cmp.w	r10, #0x6000
 8003964: d18e         	bne	0x8003884 <raylar_crcl_encoder_service::benchmark::peak::he0ae015b97d7ff8e+0x132> @ imm = #-0xe4
 8003966: 9606         	str	r6, [sp, #0x18]
 8003968: a806         	add	r0, sp, #0x18
 800396a: f246 0094    	movw	r0, #0x6094
 800396e: f04f 0900    	mov.w	r9, #0x0
 8003972: 9906         	ldr	r1, [sp, #0x18]
 8003974: f46f 0e00    	mvn	lr, #0x800000
 8003978: 9102         	str	r1, [sp, #0x8]
 800397a: f06f 7880    	mvn	r8, #0x1000000
 800397e: 9904         	ldr	r1, [sp, #0x10]
 8003980: f04f 0a00    	mov.w	r10, #0x0
 8003984: 4408         	add	r0, r1
 8003986: 9005         	str	r0, [sp, #0x14]
 8003988: 9805         	ldr	r0, [sp, #0x14]
 800398a: eb00 060a    	add.w	r6, r0, r10
 800398e: f856 0c60    	ldr	r0, [r6, #-96]
 8003992: eb00 010e    	add.w	r1, r0, lr
 8003996: 4541         	cmp	r1, r8
 8003998: bf22         	ittt	hs
 800399a: f856 1c48    	ldrhs	r1, [r6, #-72]
 800399e: eb01 020e    	addhs.w	r2, r1, lr
 80039a2: 4542         	cmphs	r2, r8
 80039a4: f0c0 8176    	blo.w	0x8003c94 <raylar_crcl_encoder_service::benchmark::peak::he0ae015b97d7ff8e+0x542> @ imm = #0x2ec
 80039a8: f856 2c30    	ldr	r2, [r6, #-48]
 80039ac: eb02 030e    	add.w	r3, r2, lr
 80039b0: 4543         	cmp	r3, r8
 80039b2: bf22         	ittt	hs
 80039b4: f856 3c18    	ldrhs	r3, [r6, #-24]
 80039b8: eb03 040e    	addhs.w	r4, r3, lr
 80039bc: 4544         	cmphs	r4, r8
 80039be: f0c0 8169    	blo.w	0x8003c94 <raylar_crcl_encoder_service::benchmark::peak::he0ae015b97d7ff8e+0x542> @ imm = #0x2d2
 80039c2: 9d05         	ldr	r5, [sp, #0x14]
 80039c4: f855 500a    	ldr.w	r5, [r5, r10]
 80039c8: eb05 040e    	add.w	r4, r5, lr
 80039cc: 4544         	cmp	r4, r8
 80039ce: bf22         	ittt	hs
 80039d0: f8d6 c018    	ldrhs.w	r12, [r6, #0x18]
 80039d4: eb0c 040e    	addhs.w	r4, r12, lr
 80039d8: 4544         	cmphs	r4, r8
 80039da: f0c0 815b    	blo.w	0x8003c94 <raylar_crcl_encoder_service::benchmark::peak::he0ae015b97d7ff8e+0x542> @ imm = #0x2b6
 80039de: f8d6 b030    	ldr.w	r11, [r6, #0x30]
 80039e2: eb0b 040e    	add.w	r4, r11, lr
 80039e6: 4544         	cmp	r4, r8
 80039e8: bf22         	ittt	hs
 80039ea: 6cb6         	ldrhs	r6, [r6, #0x48]
 80039ec: eb06 040e    	addhs.w	r4, r6, lr
 80039f0: 4544         	cmphs	r4, r8
 80039f2: f0c0 814f    	blo.w	0x8003c94 <raylar_crcl_encoder_service::benchmark::peak::he0ae015b97d7ff8e+0x542> @ imm = #0x29e
 80039f6: 2800         	cmp	r0, #0x0
 80039f8: bf48         	it	mi
 80039fa: 4240         	rsbmi	r0, r0, #0
 80039fc: 4548         	cmp	r0, r9
 80039fe: bf98         	it	ls
 8003a00: 4648         	movls	r0, r9
 8003a02: 2900         	cmp	r1, #0x0
 8003a04: bf48         	it	mi
 8003a06: 4249         	rsbmi	r1, r1, #0
 8003a08: 4281         	cmp	r1, r0
 8003a0a: bf98         	it	ls
 8003a0c: 4601         	movls	r1, r0
 8003a0e: 2a00         	cmp	r2, #0x0
 8003a10: bf48         	it	mi
 8003a12: 4252         	rsbmi	r2, r2, #0
 8003a14: 428a         	cmp	r2, r1
 8003a16: bf98         	it	ls
 8003a18: 460a         	movls	r2, r1
 8003a1a: 2b00         	cmp	r3, #0x0
 8003a1c: bf48         	it	mi
 8003a1e: 425b         	rsbmi	r3, r3, #0
 8003a20: 4293         	cmp	r3, r2
 8003a22: bf98         	it	ls
 8003a24: 4613         	movls	r3, r2
 8003a26: 2d00         	cmp	r5, #0x0
 8003a28: bf48         	it	mi
 8003a2a: 426d         	rsbmi	r5, r5, #0
 8003a2c: 429d         	cmp	r5, r3
 8003a2e: bf98         	it	ls
 8003a30: 461d         	movls	r5, r3
 8003a32: f1bc 0f00    	cmp.w	r12, #0x0
 8003a36: bf48         	it	mi
 8003a38: f1cc 0c00    	rsbmi.w	r12, r12, #0x0
 8003a3c: 45ac         	cmp	r12, r5
 8003a3e: bf98         	it	ls
 8003a40: 46ac         	movls	r12, r5
 8003a42: f1bb 0f00    	cmp.w	r11, #0x0
 8003a46: bf48         	it	mi
 8003a48: f1cb 0b00    	rsbmi.w	r11, r11, #0x0
 8003a4c: 45e3         	cmp	r11, r12
 8003a4e: bf98         	it	ls
 8003a50: 46e3         	movls	r11, r12
 8003a52: 2e00         	cmp	r6, #0x0
 8003a54: bf48         	it	mi
 8003a56: 4276         	rsbmi	r6, r6, #0
 8003a58: f10a 0ac0    	add.w	r10, r10, #0xc0
 8003a5c: 455e         	cmp	r6, r11
 8003a5e: bf98         	it	ls
 8003a60: 465e         	movls	r6, r11
 8003a62: 46b1         	mov	r9, r6
 8003a64: f5ba 4fc0    	cmp.w	r10, #0x6000
 8003a68: d18e         	bne	0x8003988 <raylar_crcl_encoder_service::benchmark::peak::he0ae015b97d7ff8e+0x236> @ imm = #-0xe4
 8003a6a: 9606         	str	r6, [sp, #0x18]
 8003a6c: a806         	add	r0, sp, #0x18
 8003a6e: f246 00e0    	movw	r0, #0x60e0
 8003a72: f44f 6a80    	mov.w	r10, #0x400
 8003a76: 9906         	ldr	r1, [sp, #0x18]
 8003a78: f46f 0b00    	mvn	r11, #0x800000
 8003a7c: 9101         	str	r1, [sp, #0x4]
 8003a7e: f06f 7380    	mvn	r3, #0x1000000
 8003a82: 9904         	ldr	r1, [sp, #0x10]
 8003a84: 4401         	add	r1, r0
 8003a86: 2000         	movs	r0, #0x0
 8003a88: f851 4ca8    	ldr	r4, [r1, #-168]
 8003a8c: eb04 020b    	add.w	r2, r4, r11
 8003a90: 429a         	cmp	r2, r3
 8003a92: f0c0 80ff    	blo.w	0x8003c94 <raylar_crcl_encoder_service::benchmark::peak::he0ae015b97d7ff8e+0x542> @ imm = #0x1fe
 8003a96: f851 5c90    	ldr	r5, [r1, #-144]
 8003a9a: eb05 020b    	add.w	r2, r5, r11
 8003a9e: 429a         	cmp	r2, r3
 8003aa0: bf22         	ittt	hs
 8003aa2: f851 2c78    	ldrhs	r2, [r1, #-120]
 8003aa6: eb02 060b    	addhs.w	r6, r2, r11
 8003aaa: 429e         	cmphs	r6, r3
 8003aac: f0c0 80f2    	blo.w	0x8003c94 <raylar_crcl_encoder_service::benchmark::peak::he0ae015b97d7ff8e+0x542> @ imm = #0x1e4
 8003ab0: f851 cc60    	ldr	r12, [r1, #-96]
 8003ab4: eb0c 060b    	add.w	r6, r12, r11
 8003ab8: 429e         	cmp	r6, r3
 8003aba: bf22         	ittt	hs
 8003abc: f851 ec48    	ldrhs	lr, [r1, #-72]
 8003ac0: eb0e 060b    	addhs.w	r6, lr, r11
 8003ac4: 429e         	cmphs	r6, r3
 8003ac6: f0c0 80e5    	blo.w	0x8003c94 <raylar_crcl_encoder_service::benchmark::peak::he0ae015b97d7ff8e+0x542> @ imm = #0x1ca
 8003aca: f851 8c30    	ldr	r8, [r1, #-48]
 8003ace: eb08 060b    	add.w	r6, r8, r11
 8003ad2: 429e         	cmp	r6, r3
 8003ad4: bf22         	ittt	hs
 8003ad6: f851 9c18    	ldrhs	r9, [r1, #-24]
 8003ada: eb09 060b    	addhs.w	r6, r9, r11
 8003ade: 429e         	cmphs	r6, r3
 8003ae0: f0c0 80d8    	blo.w	0x8003c94 <raylar_crcl_encoder_service::benchmark::peak::he0ae015b97d7ff8e+0x542> @ imm = #0x1b0
 8003ae4: 680e         	ldr	r6, [r1]
 8003ae6: f46f 0b00    	mvn	r11, #0x800000
 8003aea: f8cd a014    	str.w	r10, [sp, #0x14]
 8003aee: 4682         	mov	r10, r0
 8003af0: eb06 000b    	add.w	r0, r6, r11
 8003af4: 4298         	cmp	r0, r3
 8003af6: 4650         	mov	r0, r10
 8003af8: f8dd a014    	ldr.w	r10, [sp, #0x14]
 8003afc: f0c0 80ca    	blo.w	0x8003c94 <raylar_crcl_encoder_service::benchmark::peak::he0ae015b97d7ff8e+0x542> @ imm = #0x194
 8003b00: 2c00         	cmp	r4, #0x0
 8003b02: bf48         	it	mi
 8003b04: 4264         	rsbmi	r4, r4, #0
 8003b06: 4284         	cmp	r4, r0
 8003b08: bf98         	it	ls
 8003b0a: 4604         	movls	r4, r0
 8003b0c: 2d00         	cmp	r5, #0x0
 8003b0e: bf48         	it	mi
 8003b10: 426d         	rsbmi	r5, r5, #0
 8003b12: 42a5         	cmp	r5, r4
 8003b14: bf98         	it	ls
 8003b16: 4625         	movls	r5, r4
 8003b18: 2a00         	cmp	r2, #0x0
 8003b1a: bf48         	it	mi
 8003b1c: 4252         	rsbmi	r2, r2, #0
 8003b1e: 42aa         	cmp	r2, r5
 8003b20: bf98         	it	ls
 8003b22: 462a         	movls	r2, r5
 8003b24: f1bc 0f00    	cmp.w	r12, #0x0
 8003b28: bf48         	it	mi
 8003b2a: f1cc 0c00    	rsbmi.w	r12, r12, #0x0
 8003b2e: 4594         	cmp	r12, r2
 8003b30: bf98         	it	ls
 8003b32: 4694         	movls	r12, r2
 8003b34: f1be 0f00    	cmp.w	lr, #0x0
 8003b38: bf48         	it	mi
 8003b3a: f1ce 0e00    	rsbmi.w	lr, lr, #0x0
 8003b3e: 45e6         	cmp	lr, r12
 8003b40: bf98         	it	ls
 8003b42: 46e6         	movls	lr, r12
 8003b44: f1b8 0f00    	cmp.w	r8, #0x0
 8003b48: bf48         	it	mi
 8003b4a: f1c8 0800    	rsbmi.w	r8, r8, #0x0
 8003b4e: 45f0         	cmp	r8, lr
 8003b50: bf98         	it	ls
 8003b52: 46f0         	movls	r8, lr
 8003b54: f1b9 0f00    	cmp.w	r9, #0x0
 8003b58: bf48         	it	mi
 8003b5a: f1c9 0900    	rsbmi.w	r9, r9, #0x0
 8003b5e: 45c1         	cmp	r9, r8
 8003b60: bf98         	it	ls
 8003b62: 46c1         	movls	r9, r8
 8003b64: 2e00         	cmp	r6, #0x0
 8003b66: bf48         	it	mi
 8003b68: 4276         	rsbmi	r6, r6, #0
 8003b6a: 454e         	cmp	r6, r9
 8003b6c: bf98         	it	ls
 8003b6e: 464e         	movls	r6, r9
 8003b70: 31c0         	adds	r1, #0xc0
 8003b72: f1ba 0a08    	subs.w	r10, r10, #0x8
 8003b76: 4630         	mov	r0, r6
 8003b78: d186         	bne	0x8003a88 <raylar_crcl_encoder_service::benchmark::peak::he0ae015b97d7ff8e+0x336> @ imm = #-0xf4
 8003b7a: 9606         	str	r6, [sp, #0x18]
 8003b7c: a806         	add	r0, sp, #0x18
 8003b7e: f246 009c    	movw	r0, #0x609c
 8003b82: f04f 0a00    	mov.w	r10, #0x0
 8003b86: 9906         	ldr	r1, [sp, #0x18]
 8003b88: f46f 0800    	mvn	r8, #0x800000
 8003b8c: 9100         	str	r1, [sp]
 8003b8e: f06f 7980    	mvn	r9, #0x1000000
 8003b92: 9904         	ldr	r1, [sp, #0x10]
 8003b94: f04f 0b00    	mov.w	r11, #0x0
 8003b98: 4408         	add	r0, r1
 8003b9a: 9005         	str	r0, [sp, #0x14]
 8003b9c: 9805         	ldr	r0, [sp, #0x14]
 8003b9e: eb00 030b    	add.w	r3, r0, r11
 8003ba2: f853 1c60    	ldr	r1, [r3, #-96]
 8003ba6: eb01 0008    	add.w	r0, r1, r8
 8003baa: 4548         	cmp	r0, r9
 8003bac: bf22         	ittt	hs
 8003bae: f853 0c48    	ldrhs	r0, [r3, #-72]
 8003bb2: eb00 0208    	addhs.w	r2, r0, r8
 8003bb6: 454a         	cmphs	r2, r9
 8003bb8: d36c         	blo	0x8003c94 <raylar_crcl_encoder_service::benchmark::peak::he0ae015b97d7ff8e+0x542> @ imm = #0xd8
 8003bba: f853 2c30    	ldr	r2, [r3, #-48]
 8003bbe: eb02 0608    	add.w	r6, r2, r8
 8003bc2: 454e         	cmp	r6, r9
 8003bc4: bf22         	ittt	hs
 8003bc6: f853 4c18    	ldrhs	r4, [r3, #-24]
 8003bca: eb04 0608    	addhs.w	r6, r4, r8
 8003bce: 454e         	cmphs	r6, r9
 8003bd0: d360         	blo	0x8003c94 <raylar_crcl_encoder_service::benchmark::peak::he0ae015b97d7ff8e+0x542> @ imm = #0xc0
 8003bd2: 9e05         	ldr	r6, [sp, #0x14]
 8003bd4: f856 600b    	ldr.w	r6, [r6, r11]
 8003bd8: eb06 0508    	add.w	r5, r6, r8
 8003bdc: 454d         	cmp	r5, r9
 8003bde: bf22         	ittt	hs
 8003be0: f8d3 e018    	ldrhs.w	lr, [r3, #0x18]
 8003be4: eb0e 0508    	addhs.w	r5, lr, r8
 8003be8: 454d         	cmphs	r5, r9
 8003bea: d353         	blo	0x8003c94 <raylar_crcl_encoder_service::benchmark::peak::he0ae015b97d7ff8e+0x542> @ imm = #0xa6
 8003bec: f8d3 c030    	ldr.w	r12, [r3, #0x30]
 8003bf0: eb0c 0508    	add.w	r5, r12, r8
 8003bf4: 454d         	cmp	r5, r9
 8003bf6: bf22         	ittt	hs
 8003bf8: 6c9b         	ldrhs	r3, [r3, #0x48]
 8003bfa: eb03 0508    	addhs.w	r5, r3, r8
 8003bfe: 454d         	cmphs	r5, r9
 8003c00: d348         	blo	0x8003c94 <raylar_crcl_encoder_service::benchmark::peak::he0ae015b97d7ff8e+0x542> @ imm = #0x90
 8003c02: 2900         	cmp	r1, #0x0
 8003c04: bf48         	it	mi
 8003c06: 4249         	rsbmi	r1, r1, #0
 8003c08: 4551         	cmp	r1, r10
 8003c0a: bf98         	it	ls
 8003c0c: 4651         	movls	r1, r10
 8003c0e: 2800         	cmp	r0, #0x0
 8003c10: bf48         	it	mi
 8003c12: 4240         	rsbmi	r0, r0, #0
 8003c14: 4288         	cmp	r0, r1
 8003c16: bf98         	it	ls
 8003c18: 4608         	movls	r0, r1
 8003c1a: 2a00         	cmp	r2, #0x0
 8003c1c: bf48         	it	mi
 8003c1e: 4252         	rsbmi	r2, r2, #0
 8003c20: 4282         	cmp	r2, r0
 8003c22: bf98         	it	ls
 8003c24: 4602         	movls	r2, r0
 8003c26: 2c00         	cmp	r4, #0x0
 8003c28: bf48         	it	mi
 8003c2a: 4264         	rsbmi	r4, r4, #0
 8003c2c: 4294         	cmp	r4, r2
 8003c2e: bf98         	it	ls
 8003c30: 4614         	movls	r4, r2
 8003c32: 2e00         	cmp	r6, #0x0
 8003c34: bf48         	it	mi
 8003c36: 4276         	rsbmi	r6, r6, #0
 8003c38: 42a6         	cmp	r6, r4
 8003c3a: bf98         	it	ls
 8003c3c: 4626         	movls	r6, r4
 8003c3e: f1be 0f00    	cmp.w	lr, #0x0
 8003c42: bf48         	it	mi
 8003c44: f1ce 0e00    	rsbmi.w	lr, lr, #0x0
 8003c48: 45b6         	cmp	lr, r6
 8003c4a: bf98         	it	ls
 8003c4c: 46b6         	movls	lr, r6
 8003c4e: f1bc 0f00    	cmp.w	r12, #0x0
 8003c52: bf48         	it	mi
 8003c54: f1cc 0c00    	rsbmi.w	r12, r12, #0x0
 8003c58: 45f4         	cmp	r12, lr
 8003c5a: bf98         	it	ls
 8003c5c: 46f4         	movls	r12, lr
 8003c5e: 2b00         	cmp	r3, #0x0
 8003c60: bf48         	it	mi
 8003c62: 425b         	rsbmi	r3, r3, #0
 8003c64: f10b 0bc0    	add.w	r11, r11, #0xc0
 8003c68: 4563         	cmp	r3, r12
 8003c6a: bf98         	it	ls
 8003c6c: 4663         	movls	r3, r12
 8003c6e: 469a         	mov	r10, r3
 8003c70: f5bb 4fc0    	cmp.w	r11, #0x6000
 8003c74: d192         	bne	0x8003b9c <raylar_crcl_encoder_service::benchmark::peak::he0ae015b97d7ff8e+0x44a> @ imm = #-0xdc
 8003c76: 9306         	str	r3, [sp, #0x18]
 8003c78: a806         	add	r0, sp, #0x18
 8003c7a: e9dd 1002    	ldrd	r1, r0, [sp, #8]
 8003c7e: 9a00         	ldr	r2, [sp]
 8003c80: 4048         	eors	r0, r1
 8003c82: 9901         	ldr	r1, [sp, #0x4]
 8003c84: 4048         	eors	r0, r1
 8003c86: 9906         	ldr	r1, [sp, #0x18]
 8003c88: 4050         	eors	r0, r2
 8003c8a: 4048         	eors	r0, r1
 8003c8c: b007         	add	sp, #0x1c
 8003c8e: e8bd 0f00    	pop.w	{r8, r9, r10, r11}
 8003c92: bdf0         	pop	{r4, r5, r6, r7, pc}
 8003c94: f64d 7060    	movw	r0, #0xdf60
 8003c98: f64d 7268    	movw	r2, #0xdf68
 8003c9c: f6c0 0000    	movt	r0, #0x800
 8003ca0: f6c0 0200    	movt	r2, #0x800
 8003ca4: 210b         	movs	r1, #0xb
 8003ca6: f007 fe37    	bl	0x800b918 <core::panicking::panic_fmt> @ imm = #0x7c6e

08003caa <raylar_crcl_encoder_service::benchmark::codec::h6d81b9c8b86ce9a2>:
 8003caa: b5f0         	push	{r4, r5, r6, r7, lr}
 8003cac: af03         	add	r7, sp, #0xc
 8003cae: e92d 0b00    	push.w	{r8, r9, r11}
 8003cb2: b0d6         	sub	sp, #0x158
 8003cb4: f240 690a    	movw	r9, #0x60a
 8003cb8: 4680         	mov	r8, r0
 8003cba: f44f 0000    	mov.w	r0, #0x800000
 8003cbe: f2c0 1901    	movt	r9, #0x101
 8003cc2: 900b         	str	r0, [sp, #0x2c]
 8003cc4: f44f 6080    	mov.w	r0, #0x400
 8003cc8: f8cd 9030    	str.w	r9, [sp, #0x30]
 8003ccc: ad0a         	add	r5, sp, #0x28
 8003cce: 900a         	str	r0, [sp, #0x28]
 8003cd0: 980a         	ldr	r0, [sp, #0x28]
 8003cd2: 1e41         	subs	r1, r0, #0x1
 8003cd4: ea80 0201    	eor.w	r2, r0, r1
 8003cd8: 428a         	cmp	r2, r1
 8003cda: d916         	bls	0x8003d0a <raylar_crcl_encoder_service::benchmark::codec::h6d81b9c8b86ce9a2+0x60> @ imm = #0x2c
 8003cdc: fa90 f0a0    	rbit	r0, r0
 8003ce0: fab0 f080    	clz	r0, r0
 8003ce4: 3808         	subs	r0, #0x8
 8003ce6: 2804         	cmp	r0, #0x4
 8003ce8: bf9c         	itt	ls
 8003cea: f89d 0030    	ldrbls.w	r0, [sp, #0x30]
 8003cee: 280c         	cmpls	r0, #0xc
 8003cf0: d80b         	bhi	0x8003d0a <raylar_crcl_encoder_service::benchmark::codec::h6d81b9c8b86ce9a2+0x60> @ imm = #0x16
 8003cf2: 2101         	movs	r1, #0x1
 8003cf4: fa01 f000    	lsl.w	r0, r1, r0
 8003cf8: f410 5faa    	tst.w	r0, #0x1540
 8003cfc: bf1e         	ittt	ne
 8003cfe: 980b         	ldrne	r0, [sp, #0x2c]
 8003d00: f100 4040    	addne.w	r0, r0, #0xc0000000
 8003d04: f1b0 4f40    	cmpne.w	r0, #0xc0000000
 8003d08: d813         	bhi	0x8003d32 <raylar_crcl_encoder_service::benchmark::codec::h6d81b9c8b86ce9a2+0x88> @ imm = #0x26
 8003d0a: 2000         	movs	r0, #0x0
 8003d0c: f88d 00c4    	strb.w	r0, [sp, #0xc4]
 8003d10: f64d 70e8    	movw	r0, #0xdfe8
 8003d14: f6c0 0000    	movt	r0, #0x800
 8003d18: f64d 53e4    	movw	r3, #0xdde4
 8003d1c: 9000         	str	r0, [sp]
 8003d1e: f64d 5048    	movw	r0, #0xdd48
 8003d22: aa31         	add	r2, sp, #0xc4
 8003d24: f6c0 0000    	movt	r0, #0x800
 8003d28: f6c0 0300    	movt	r3, #0x800
 8003d2c: 212b         	movs	r1, #0x2b
 8003d2e: f007 fd73    	bl	0x800b818 <core::result::unwrap_failed> @ imm = #0x7ae6
 8003d32: f89d 0031    	ldrb.w	r0, [sp, #0x31]
 8003d36: 281e         	cmp	r0, #0x1e
 8003d38: d8e7         	bhi	0x8003d0a <raylar_crcl_encoder_service::benchmark::codec::h6d81b9c8b86ce9a2+0x60> @ imm = #-0x32
 8003d3a: f89d 4033    	ldrb.w	r4, [sp, #0x33]
 8003d3e: 2c01         	cmp	r4, #0x1
 8003d40: d104         	bne	0x8003d4c <raylar_crcl_encoder_service::benchmark::codec::h6d81b9c8b86ce9a2+0xa2> @ imm = #0x8
 8003d42: f89d 0032    	ldrb.w	r0, [sp, #0x32]
 8003d46: 2801         	cmp	r0, #0x1
 8003d48: d002         	beq	0x8003d50 <raylar_crcl_encoder_service::benchmark::codec::h6d81b9c8b86ce9a2+0xa6> @ imm = #0x4
 8003d4a: e7de         	b	0x8003d0a <raylar_crcl_encoder_service::benchmark::codec::h6d81b9c8b86ce9a2+0x60> @ imm = #-0x44
 8003d4c: 2800         	cmp	r0, #0x0
 8003d4e: d1dc         	bne	0x8003d0a <raylar_crcl_encoder_service::benchmark::codec::h6d81b9c8b86ce9a2+0x60> @ imm = #-0x48
 8003d50: e9dd 010a    	ldrd	r0, r1, [sp, #40]
 8003d54: 2600         	movs	r6, #0x0
 8003d56: f8dd 202f    	ldr.w	r2, [sp, #0x2f]
 8003d5a: f8cd 00d3    	str.w	r0, [sp, #0xd3]
 8003d5e: a802         	add	r0, sp, #0x8
 8003d60: f8cd 20da    	str.w	r2, [sp, #0xda]
 8003d64: 3001         	adds	r0, #0x1
 8003d66: f8cd 10d7    	str.w	r1, [sp, #0xd7]
 8003d6a: a931         	add	r1, sp, #0xc4
 8003d6c: 221a         	movs	r2, #0x1a
 8003d6e: 9633         	str	r6, [sp, #0xcc]
 8003d70: e9cd 6631    	strd	r6, r6, [sp, #196]
 8003d74: f8cd 60cf    	str.w	r6, [sp, #0xcf]
 8003d78: f88d 6008    	strb.w	r6, [sp, #0x8]
 8003d7c: f008 fd88    	bl	0x800c890 <__aeabi_memcpy> @ imm = #0x8b10
 8003d80: f44f 0000    	mov.w	r0, #0x800000
 8003d84: f88d 4023    	strb.w	r4, [sp, #0x23]
 8003d88: 9032         	str	r0, [sp, #0xc8]
 8003d8a: f44f 6080    	mov.w	r0, #0x400
 8003d8e: 9031         	str	r0, [sp, #0xc4]
 8003d90: a831         	add	r0, sp, #0xc4
 8003d92: f8cd 90cc    	str.w	r9, [sp, #0xcc]
 8003d96: 9831         	ldr	r0, [sp, #0xc4]
 8003d98: 1e41         	subs	r1, r0, #0x1
 8003d9a: ea80 0201    	eor.w	r2, r0, r1
 8003d9e: 428a         	cmp	r2, r1
 8003da0: d90a         	bls	0x8003db8 <raylar_crcl_encoder_service::benchmark::codec::h6d81b9c8b86ce9a2+0x10e> @ imm = #0x14
 8003da2: fa90 f1a0    	rbit	r1, r0
 8003da6: fab1 f181    	clz	r1, r1
 8003daa: 3908         	subs	r1, #0x8
 8003dac: 2904         	cmp	r1, #0x4
 8003dae: bf9c         	itt	ls
 8003db0: f89d 10cc    	ldrbls.w	r1, [sp, #0xcc]
 8003db4: 290c         	cmpls	r1, #0xc
 8003db6: d906         	bls	0x8003dc6 <raylar_crcl_encoder_service::benchmark::codec::h6d81b9c8b86ce9a2+0x11c> @ imm = #0xc
 8003db8: f64d 70d8    	movw	r0, #0xdfd8
 8003dbc: f88d 60c4    	strb.w	r6, [sp, #0xc4]
 8003dc0: f6c0 0000    	movt	r0, #0x800
 8003dc4: e7a8         	b	0x8003d18 <raylar_crcl_encoder_service::benchmark::codec::h6d81b9c8b86ce9a2+0x6e> @ imm = #-0xb0
 8003dc6: 2201         	movs	r2, #0x1
 8003dc8: fa02 f101    	lsl.w	r1, r2, r1
 8003dcc: f411 5faa    	tst.w	r1, #0x1540
 8003dd0: d0f2         	beq	0x8003db8 <raylar_crcl_encoder_service::benchmark::codec::h6d81b9c8b86ce9a2+0x10e> @ imm = #-0x1c
 8003dd2: 9a32         	ldr	r2, [sp, #0xc8]
 8003dd4: 2101         	movs	r1, #0x1
 8003dd6: f2cc 0100    	movt	r1, #0xc000
 8003dda: f102 4240    	add.w	r2, r2, #0xc0000000
 8003dde: 428a         	cmp	r2, r1
 8003de0: d3ea         	blo	0x8003db8 <raylar_crcl_encoder_service::benchmark::codec::h6d81b9c8b86ce9a2+0x10e> @ imm = #-0x2c
 8003de2: f89d 10cd    	ldrb.w	r1, [sp, #0xcd]
 8003de6: 291e         	cmp	r1, #0x1e
 8003de8: d8e6         	bhi	0x8003db8 <raylar_crcl_encoder_service::benchmark::codec::h6d81b9c8b86ce9a2+0x10e> @ imm = #-0x34
 8003dea: f89d 20cf    	ldrb.w	r2, [sp, #0xcf]
 8003dee: 2a01         	cmp	r2, #0x1
 8003df0: d107         	bne	0x8003e02 <raylar_crcl_encoder_service::benchmark::codec::h6d81b9c8b86ce9a2+0x158> @ imm = #0xe
 8003df2: f5b0 6f80    	cmp.w	r0, #0x400
 8003df6: d3df         	blo	0x8003db8 <raylar_crcl_encoder_service::benchmark::codec::h6d81b9c8b86ce9a2+0x10e> @ imm = #-0x42
 8003df8: f89d 00ce    	ldrb.w	r0, [sp, #0xce]
 8003dfc: 2801         	cmp	r0, #0x1
 8003dfe: d005         	beq	0x8003e0c <raylar_crcl_encoder_service::benchmark::codec::h6d81b9c8b86ce9a2+0x162> @ imm = #0xa
 8003e00: e7da         	b	0x8003db8 <raylar_crcl_encoder_service::benchmark::codec::h6d81b9c8b86ce9a2+0x10e> @ imm = #-0x4c
 8003e02: f5b0 6f80    	cmp.w	r0, #0x400
 8003e06: d3d7         	blo	0x8003db8 <raylar_crcl_encoder_service::benchmark::codec::h6d81b9c8b86ce9a2+0x10e> @ imm = #-0x52
 8003e08: 2900         	cmp	r1, #0x0
 8003e0a: d1d5         	bne	0x8003db8 <raylar_crcl_encoder_service::benchmark::codec::h6d81b9c8b86ce9a2+0x10e> @ imm = #-0x56
 8003e0c: f246 0028    	movw	r0, #0x6028
 8003e10: eb08 0400    	add.w	r4, r8, r0
 8003e14: f105 0048    	add.w	r0, r5, #0x48
 8003e18: 2128         	movs	r1, #0x28
 8003e1a: f008 ff62    	bl	0x800cce2 <__aeabi_memclr4> @ imm = #0x8ec4
 8003e1e: ab31         	add	r3, sp, #0xc4
 8003e20: ae31         	add	r6, sp, #0xc4
 8003e22: 2000         	movs	r0, #0x0
 8003e24: 941a         	str	r4, [sp, #0x68]
 8003e26: cb0e         	ldm	r3, {r1, r2, r3}
 8003e28: f44f 7480    	mov.w	r4, #0x100
 8003e2c: 932d         	str	r3, [sp, #0xb4]
 8003e2e: f44f 53c0    	mov.w	r3, #0x1800
 8003e32: e9cd 122b    	strd	r1, r2, [sp, #172]
 8003e36: 2101         	movs	r1, #0x1
 8003e38: f8ad 0044    	strh.w	r0, [sp, #0x44]
 8003e3c: e9cd 000f    	strd	r0, r0, [sp, #60]
 8003e40: e9cd 000d    	strd	r0, r0, [sp, #52]
 8003e44: e9cd 000b    	strd	r0, r0, [sp, #44]
 8003e48: e9cd 0013    	strd	r0, r0, [sp, #76]
 8003e4c: e9cd 0015    	strd	r0, r0, [sp, #84]
 8003e50: e9cd 0017    	strd	r0, r0, [sp, #92]
 8003e54: f8ad 0064    	strh.w	r0, [sp, #0x64]
 8003e58: e9cd 0027    	strd	r0, r0, [sp, #156]
 8003e5c: e9cd 0029    	strd	r0, r0, [sp, #164]
 8003e60: 900a         	str	r0, [sp, #0x28]
 8003e62: 9012         	str	r0, [sp, #0x48]
 8003e64: 931b         	str	r3, [sp, #0x6c]
 8003e66: 9126         	str	r1, [sp, #0x98]
 8003e68: f8ad 00b8    	strh.w	r0, [sp, #0xb8]
 8003e6c: 9431         	str	r4, [sp, #0xc4]
 8003e6e: 4628         	mov	r0, r5
 8003e70: 9931         	ldr	r1, [sp, #0xc4]
 8003e72: f7fd fdee    	bl	0x8001a52 <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3> @ imm = #-0x2424
 8003e76: 07c0         	lsls	r0, r0, #0x1f
 8003e78: d123         	bne	0x8003ec2 <raylar_crcl_encoder_service::benchmark::codec::h6d81b9c8b86ce9a2+0x218> @ imm = #0x46
 8003e7a: 07c8         	lsls	r0, r1, #0x1f
 8003e7c: d0f6         	beq	0x8003e6c <raylar_crcl_encoder_service::benchmark::codec::h6d81b9c8b86ce9a2+0x1c2> @ imm = #-0x14
 8003e7e: ad31         	add	r5, sp, #0xc4
 8003e80: a90a         	add	r1, sp, #0x28
 8003e82: 2294         	movs	r2, #0x94
 8003e84: 4628         	mov	r0, r5
 8003e86: f008 fede    	bl	0x800cc46 <__aeabi_memcpy4> @ imm = #0x8dbc
 8003e8a: f641 6058    	movw	r0, #0x1e58
 8003e8e: f508 34e0    	add.w	r4, r8, #0x1c000
 8003e92: 9000         	str	r0, [sp]
 8003e94: a82f         	add	r0, sp, #0xbc
 8003e96: a902         	add	r1, sp, #0x8
 8003e98: 462a         	mov	r2, r5
 8003e9a: 4623         	mov	r3, r4
 8003e9c: f7fd faf2    	bl	0x8001484 <raylar_crcl_encoder_service::format::Encoder::data::ha65fe528befbf4e3> @ imm = #-0x2a1c
 8003ea0: f89d 00bc    	ldrb.w	r0, [sp, #0xbc]
 8003ea4: 2801         	cmp	r0, #0x1
 8003ea6: d01c         	beq	0x8003ee2 <raylar_crcl_encoder_service::benchmark::codec::h6d81b9c8b86ce9a2+0x238> @ imm = #0x38
 8003ea8: 9930         	ldr	r1, [sp, #0xc0]
 8003eaa: f641 6059    	movw	r0, #0x1e59
 8003eae: 4281         	cmp	r1, r0
 8003eb0: d20e         	bhs	0x8003ed0 <raylar_crcl_encoder_service::benchmark::codec::h6d81b9c8b86ce9a2+0x226> @ imm = #0x1c
 8003eb2: e9cd 4131    	strd	r4, r1, [sp, #196]
 8003eb6: a831         	add	r0, sp, #0xc4
 8003eb8: 4608         	mov	r0, r1
 8003eba: b056         	add	sp, #0x158
 8003ebc: e8bd 0b00    	pop.w	{r8, r9, r11}
 8003ec0: bdf0         	pop	{r4, r5, r6, r7, pc}
 8003ec2: f64d 70c8    	movw	r0, #0xdfc8
 8003ec6: f88d 10c4    	strb.w	r1, [sp, #0xc4]
 8003eca: f6c0 0000    	movt	r0, #0x800
 8003ece: e723         	b	0x8003d18 <raylar_crcl_encoder_service::benchmark::codec::h6d81b9c8b86ce9a2+0x6e> @ imm = #-0x1ba
 8003ed0: f64d 73a8    	movw	r3, #0xdfa8
 8003ed4: 2000         	movs	r0, #0x0
 8003ed6: f6c0 0300    	movt	r3, #0x800
 8003eda: f641 6258    	movw	r2, #0x1e58
 8003ede: f007 ffa6    	bl	0x800be2e <core::slice::index::slice_index_fail> @ imm = #0x7f4c
 8003ee2: f89d 00bd    	ldrb.w	r0, [sp, #0xbd]
 8003ee6: f88d 00c4    	strb.w	r0, [sp, #0xc4]
 8003eea: f64d 70b8    	movw	r0, #0xdfb8
 8003eee: f6c0 0000    	movt	r0, #0x800
 8003ef2: e711         	b	0x8003d18 <raylar_crcl_encoder_service::benchmark::codec::h6d81b9c8b86ce9a2+0x6e> @ imm = #-0x1de

08003ef4 <raylar_crcl_encoder_service::benchmark::empty::h49addc3cc3cef455>:
 8003ef4: b580         	push	{r7, lr}
 8003ef6: 466f         	mov	r7, sp
 8003ef8: b081         	sub	sp, #0x4
 8003efa: 2000         	movs	r0, #0x0
 8003efc: 9000         	str	r0, [sp]
 8003efe: 4668         	mov	r0, sp
 8003f00: f85d 0b04    	ldr	r0, [sp], #4
 8003f04: bd80         	pop	{r7, pc}

08003f06 <raylar_crcl_encoder_service::benchmark::fused::h66edd02a0d627444>:
 8003f06: b5f0         	push	{r4, r5, r6, r7, lr}
 8003f08: af03         	add	r7, sp, #0xc
 8003f0a: e92d 0f00    	push.w	{r8, r9, r10, r11}
 8003f0e: b08a         	sub	sp, #0x28
 8003f10: 6801         	ldr	r1, [r0]
 8003f12: f246 052c    	movw	r5, #0x602c
 8003f16: 9108         	str	r1, [sp, #0x20]
 8003f18: a908         	add	r1, sp, #0x20
 8003f1a: 2601         	movs	r6, #0x1
 8003f1c: 9005         	str	r0, [sp, #0x14]
 8003f1e: 9908         	ldr	r1, [sp, #0x20]
 8003f20: f04f 0e00    	mov.w	lr, #0x0
 8003f24: 5943         	ldr	r3, [r0, r5]
 8003f26: 1e4a         	subs	r2, r1, #0x1
 8003f28: f002 021f    	and	r2, r2, #0x1f
 8003f2c: fa06 fc02    	lsl.w	r12, r6, r2
 8003f30: eb03 060c    	add.w	r6, r3, r12
 8003f34: f001 031f    	and	r3, r1, #0x1f
 8003f38: 1941         	adds	r1, r0, r5
 8003f3a: 9104         	str	r1, [sp, #0x10]
 8003f3c: fa46 fa03    	asr.w	r10, r6, r3
 8003f40: f840 a005    	str.w	r10, [r0, r5]
 8003f44: f240 30ff    	movw	r0, #0x3ff
 8003f48: 6b0d         	ldr	r5, [r1, #0x30]
 8003f4a: 6c8c         	ldr	r4, [r1, #0x48]
 8003f4c: 4465         	add	r5, r12
 8003f4e: 9006         	str	r0, [sp, #0x18]
 8003f50: 4464         	add	r4, r12
 8003f52: f8d1 9018    	ldr.w	r9, [r1, #0x18]
 8003f56: 411d         	asrs	r5, r3
 8003f58: f8d1 8060    	ldr.w	r8, [r1, #0x60]
 8003f5c: 411c         	asrs	r4, r3
 8003f5e: 1b60         	subs	r0, r4, r5
 8003f60: 630d         	str	r5, [r1, #0x30]
 8003f62: 0046         	lsls	r6, r0, #0x1
 8003f64: ea86 7be0    	eor.w	r11, r6, r0, asr #31
 8003f68: eb09 060c    	add.w	r6, r9, r12
 8003f6c: 648c         	str	r4, [r1, #0x48]
 8003f6e: f8d1 2108    	ldr.w	r2, [r1, #0x108]
 8003f72: 411e         	asrs	r6, r3
 8003f74: 1bad         	subs	r5, r5, r6
 8003f76: 618e         	str	r6, [r1, #0x18]
 8003f78: 4462         	add	r2, r12
 8003f7a: 0068         	lsls	r0, r5, #0x1
 8003f7c: ea80 70e5    	eor.w	r0, r0, r5, asr #31
 8003f80: eba6 050a    	sub.w	r5, r6, r10
 8003f84: fa42 fa03    	asr.w	r10, r2, r3
 8003f88: 006e         	lsls	r6, r5, #0x1
 8003f8a: ea86 75e5    	eor.w	r5, r6, r5, asr #31
 8003f8e: eb0e 0605    	add.w	r6, lr, r5
 8003f92: eb08 050c    	add.w	r5, r8, r12
 8003f96: 4430         	add	r0, r6
 8003f98: f8d1 e078    	ldr.w	lr, [r1, #0x78]
 8003f9c: 411d         	asrs	r5, r3
 8003f9e: 1b2c         	subs	r4, r5, r4
 8003fa0: 4458         	add	r0, r11
 8003fa2: 660d         	str	r5, [r1, #0x60]
 8003fa4: 0066         	lsls	r6, r4, #0x1
 8003fa6: ea86 74e4    	eor.w	r4, r6, r4, asr #31
 8003faa: 4420         	add	r0, r4
 8003fac: eb0e 040c    	add.w	r4, lr, r12
 8003fb0: f8d1 8090    	ldr.w	r8, [r1, #0x90]
 8003fb4: 411c         	asrs	r4, r3
 8003fb6: 1b65         	subs	r5, r4, r5
 8003fb8: 678c         	str	r4, [r1, #0x78]
 8003fba: 006e         	lsls	r6, r5, #0x1
 8003fbc: f8d1 e0a8    	ldr.w	lr, [r1, #0xa8]
 8003fc0: ea86 76e5    	eor.w	r6, r6, r5, asr #31
 8003fc4: f8c1 a108    	str.w	r10, [r1, #0x108]
 8003fc8: 4430         	add	r0, r6
 8003fca: eb08 060c    	add.w	r6, r8, r12
 8003fce: 411e         	asrs	r6, r3
 8003fd0: 1b34         	subs	r4, r6, r4
 8003fd2: f8c1 6090    	str.w	r6, [r1, #0x90]
 8003fd6: 0065         	lsls	r5, r4, #0x1
 8003fd8: ea85 74e4    	eor.w	r4, r5, r4, asr #31
 8003fdc: eb00 0804    	add.w	r8, r0, r4
 8003fe0: eb0e 040c    	add.w	r4, lr, r12
 8003fe4: f8d1 50f0    	ldr.w	r5, [r1, #0xf0]
 8003fe8: fa44 f003    	asr.w	r0, r4, r3
 8003fec: 1b86         	subs	r6, r0, r6
 8003fee: 4465         	add	r5, r12
 8003ff0: f8c1 00a8    	str.w	r0, [r1, #0xa8]
 8003ff4: 0074         	lsls	r4, r6, #0x1
 8003ff6: ea84 7ee6    	eor.w	lr, r4, r6, asr #31
 8003ffa: f8d1 60c0    	ldr.w	r6, [r1, #0xc0]
 8003ffe: fa45 f403    	asr.w	r4, r5, r3
 8004002: f8d1 50d8    	ldr.w	r5, [r1, #0xd8]
 8004006: 44c6         	add	lr, r8
 8004008: 4466         	add	r6, r12
 800400a: f8c1 40f0    	str.w	r4, [r1, #0xf0]
 800400e: 4465         	add	r5, r12
 8004010: 411e         	asrs	r6, r3
 8004012: 1a30         	subs	r0, r6, r0
 8004014: 411d         	asrs	r5, r3
 8004016: f8c1 60c0    	str.w	r6, [r1, #0xc0]
 800401a: 0042         	lsls	r2, r0, #0x1
 800401c: ea82 70e0    	eor.w	r0, r2, r0, asr #31
 8004020: 1baa         	subs	r2, r5, r6
 8004022: 4470         	add	r0, lr
 8004024: f8c1 50d8    	str.w	r5, [r1, #0xd8]
 8004028: f501 7184    	add.w	r1, r1, #0x108
 800402c: 0056         	lsls	r6, r2, #0x1
 800402e: ea86 72e2    	eor.w	r2, r6, r2, asr #31
 8004032: 4410         	add	r0, r2
 8004034: 1b62         	subs	r2, r4, r5
 8004036: 0056         	lsls	r6, r2, #0x1
 8004038: ea86 72e2    	eor.w	r2, r6, r2, asr #31
 800403c: 4410         	add	r0, r2
 800403e: ebaa 0204    	sub.w	r2, r10, r4
 8004042: 0056         	lsls	r6, r2, #0x1
 8004044: ea86 72e2    	eor.w	r2, r6, r2, asr #31
 8004048: eb00 0e02    	add.w	lr, r0, r2
 800404c: 9806         	ldr	r0, [sp, #0x18]
 800404e: 380b         	subs	r0, #0xb
 8004050: f47f af7a    	bne.w	0x8003f48 <raylar_crcl_encoder_service::benchmark::fused::h66edd02a0d627444+0x42> @ imm = #-0x10c
 8004054: f8cd e024    	str.w	lr, [sp, #0x24]
 8004058: a909         	add	r1, sp, #0x24
 800405a: 2501         	movs	r5, #0x1
 800405c: f04f 0900    	mov.w	r9, #0x0
 8004060: 9809         	ldr	r0, [sp, #0x24]
 8004062: f240 3eff    	movw	lr, #0x3ff
 8004066: 9003         	str	r0, [sp, #0xc]
 8004068: 9805         	ldr	r0, [sp, #0x14]
 800406a: 6881         	ldr	r1, [r0, #0x8]
 800406c: 9108         	str	r1, [sp, #0x20]
 800406e: a908         	add	r1, sp, #0x20
 8004070: 9908         	ldr	r1, [sp, #0x20]
 8004072: 9a04         	ldr	r2, [sp, #0x10]
 8004074: 1e4b         	subs	r3, r1, #0x1
 8004076: f001 041f    	and	r4, r1, #0x1f
 800407a: f003 031f    	and	r3, r3, #0x1f
 800407e: f852 6f04    	ldr	r6, [r2, #4]!
 8004082: fa05 f303    	lsl.w	r3, r5, r3
 8004086: 441e         	add	r6, r3
 8004088: fa46 fb04    	asr.w	r11, r6, r4
 800408c: f8c2 b000    	str.w	r11, [r2]
 8004090: 6b15         	ldr	r5, [r2, #0x30]
 8004092: f1be 0e0b    	subs.w	lr, lr, #0xb
 8004096: 6c96         	ldr	r6, [r2, #0x48]
 8004098: f8d2 a018    	ldr.w	r10, [r2, #0x18]
 800409c: 441d         	add	r5, r3
 800409e: 6e10         	ldr	r0, [r2, #0x60]
 80040a0: 441e         	add	r6, r3
 80040a2: 9006         	str	r0, [sp, #0x18]
 80040a4: eb0a 0003    	add.w	r0, r10, r3
 80040a8: fa45 f504    	asr.w	r5, r5, r4
 80040ac: fa46 f604    	asr.w	r6, r6, r4
 80040b0: eba6 0c05    	sub.w	r12, r6, r5
 80040b4: fa40 f004    	asr.w	r0, r0, r4
 80040b8: 6315         	str	r5, [r2, #0x30]
 80040ba: eba5 0500    	sub.w	r5, r5, r0
 80040be: 6190         	str	r0, [r2, #0x18]
 80040c0: eba0 000b    	sub.w	r0, r0, r11
 80040c4: ea4f 0145    	lsl.w	r1, r5, #0x1
 80040c8: 6496         	str	r6, [r2, #0x48]
 80040ca: ea81 71e5    	eor.w	r1, r1, r5, asr #31
 80040ce: ea4f 0540    	lsl.w	r5, r0, #0x1
 80040d2: ea85 70e0    	eor.w	r0, r5, r0, asr #31
 80040d6: ea4f 084c    	lsl.w	r8, r12, #0x1
 80040da: 4448         	add	r0, r9
 80040dc: ea88 7cec    	eor.w	r12, r8, r12, asr #31
 80040e0: 4408         	add	r0, r1
 80040e2: 9906         	ldr	r1, [sp, #0x18]
 80040e4: f8d2 8078    	ldr.w	r8, [r2, #0x78]
 80040e8: 4460         	add	r0, r12
 80040ea: eb01 0503    	add.w	r5, r1, r3
 80040ee: f8d2 c090    	ldr.w	r12, [r2, #0x90]
 80040f2: fa45 f504    	asr.w	r5, r5, r4
 80040f6: eba5 0606    	sub.w	r6, r5, r6
 80040fa: 6615         	str	r5, [r2, #0x60]
 80040fc: ea4f 0146    	lsl.w	r1, r6, #0x1
 8004100: ea81 71e6    	eor.w	r1, r1, r6, asr #31
 8004104: 4408         	add	r0, r1
 8004106: eb08 0103    	add.w	r1, r8, r3
 800410a: f8d2 80a8    	ldr.w	r8, [r2, #0xa8]
 800410e: fa41 f104    	asr.w	r1, r1, r4
 8004112: eba1 0505    	sub.w	r5, r1, r5
 8004116: 6791         	str	r1, [r2, #0x78]
 8004118: ea4f 0645    	lsl.w	r6, r5, #0x1
 800411c: ea86 75e5    	eor.w	r5, r6, r5, asr #31
 8004120: 4428         	add	r0, r5
 8004122: eb0c 0503    	add.w	r5, r12, r3
 8004126: fa45 f504    	asr.w	r5, r5, r4
 800412a: eba5 0101    	sub.w	r1, r5, r1
 800412e: f8c2 5090    	str.w	r5, [r2, #0x90]
 8004132: ea4f 0641    	lsl.w	r6, r1, #0x1
 8004136: ea86 71e1    	eor.w	r1, r6, r1, asr #31
 800413a: eb00 0c01    	add.w	r12, r0, r1
 800413e: eb08 0103    	add.w	r1, r8, r3
 8004142: f8d2 60f0    	ldr.w	r6, [r2, #0xf0]
 8004146: fa41 f104    	asr.w	r1, r1, r4
 800414a: eba1 0505    	sub.w	r5, r1, r5
 800414e: f8c2 10a8    	str.w	r1, [r2, #0xa8]
 8004152: ea4f 0045    	lsl.w	r0, r5, #0x1
 8004156: ea80 78e5    	eor.w	r8, r0, r5, asr #31
 800415a: eb06 0503    	add.w	r5, r6, r3
 800415e: f8d2 00c0    	ldr.w	r0, [r2, #0xc0]
 8004162: f8d2 60d8    	ldr.w	r6, [r2, #0xd8]
 8004166: 44c4         	add	r12, r8
 8004168: fa45 f904    	asr.w	r9, r5, r4
 800416c: 4418         	add	r0, r3
 800416e: f8d2 5108    	ldr.w	r5, [r2, #0x108]
 8004172: 441e         	add	r6, r3
 8004174: fa40 f004    	asr.w	r0, r0, r4
 8004178: eba0 0101    	sub.w	r1, r0, r1
 800417c: 441d         	add	r5, r3
 800417e: fa46 f604    	asr.w	r6, r6, r4
 8004182: f8c2 00c0    	str.w	r0, [r2, #0xc0]
 8004186: eba6 0000    	sub.w	r0, r6, r0
 800418a: fa45 fb04    	asr.w	r11, r5, r4
 800418e: ea4f 0541    	lsl.w	r5, r1, #0x1
 8004192: ea85 71e1    	eor.w	r1, r5, r1, asr #31
 8004196: ea4f 0540    	lsl.w	r5, r0, #0x1
 800419a: 4461         	add	r1, r12
 800419c: ea85 70e0    	eor.w	r0, r5, r0, asr #31
 80041a0: 4408         	add	r0, r1
 80041a2: eba9 0106    	sub.w	r1, r9, r6
 80041a6: f8c2 60d8    	str.w	r6, [r2, #0xd8]
 80041aa: ea4f 0641    	lsl.w	r6, r1, #0x1
 80041ae: ea86 71e1    	eor.w	r1, r6, r1, asr #31
 80041b2: 4408         	add	r0, r1
 80041b4: ebab 0109    	sub.w	r1, r11, r9
 80041b8: f8c2 90f0    	str.w	r9, [r2, #0xf0]
 80041bc: ea4f 0641    	lsl.w	r6, r1, #0x1
 80041c0: ea86 71e1    	eor.w	r1, r6, r1, asr #31
 80041c4: f8c2 b108    	str.w	r11, [r2, #0x108]
 80041c8: eb00 0901    	add.w	r9, r0, r1
 80041cc: f502 7284    	add.w	r2, r2, #0x108
 80041d0: f47f af5e    	bne.w	0x8004090 <raylar_crcl_encoder_service::benchmark::fused::h66edd02a0d627444+0x18a> @ imm = #-0x144
 80041d4: f8cd 9024    	str.w	r9, [sp, #0x24]
 80041d8: a809         	add	r0, sp, #0x24
 80041da: 2601         	movs	r6, #0x1
 80041dc: f04f 0800    	mov.w	r8, #0x0
 80041e0: 9809         	ldr	r0, [sp, #0x24]
 80041e2: f240 3cff    	movw	r12, #0x3ff
 80041e6: 9002         	str	r0, [sp, #0x8]
 80041e8: 9805         	ldr	r0, [sp, #0x14]
 80041ea: 6900         	ldr	r0, [r0, #0x10]
 80041ec: 9008         	str	r0, [sp, #0x20]
 80041ee: a808         	add	r0, sp, #0x20
 80041f0: 9808         	ldr	r0, [sp, #0x20]
 80041f2: 9a04         	ldr	r2, [sp, #0x10]
 80041f4: 1e43         	subs	r3, r0, #0x1
 80041f6: f000 041f    	and	r4, r0, #0x1f
 80041fa: f003 031f    	and	r3, r3, #0x1f
 80041fe: f852 1f08    	ldr	r1, [r2, #8]!
 8004202: fa06 f303    	lsl.w	r3, r6, r3
 8004206: 4419         	add	r1, r3
 8004208: fa41 fb04    	asr.w	r11, r1, r4
 800420c: f8c2 b000    	str.w	r11, [r2]
 8004210: 6b15         	ldr	r5, [r2, #0x30]
 8004212: f1bc 0c0b    	subs.w	r12, r12, #0xb
 8004216: 6c96         	ldr	r6, [r2, #0x48]
 8004218: 441d         	add	r5, r3
 800421a: f8d2 a018    	ldr.w	r10, [r2, #0x18]
 800421e: 441e         	add	r6, r3
 8004220: f8d2 9060    	ldr.w	r9, [r2, #0x60]
 8004224: fa45 f504    	asr.w	r5, r5, r4
 8004228: fa46 f604    	asr.w	r6, r6, r4
 800422c: eba6 0005    	sub.w	r0, r6, r5
 8004230: 6315         	str	r5, [r2, #0x30]
 8004232: ea4f 0140    	lsl.w	r1, r0, #0x1
 8004236: ea81 7ee0    	eor.w	lr, r1, r0, asr #31
 800423a: eb0a 0103    	add.w	r1, r10, r3
 800423e: 6496         	str	r6, [r2, #0x48]
 8004240: fa41 f104    	asr.w	r1, r1, r4
 8004244: eba5 0501    	sub.w	r5, r5, r1
 8004248: 6191         	str	r1, [r2, #0x18]
 800424a: eba1 010b    	sub.w	r1, r1, r11
 800424e: ea4f 0045    	lsl.w	r0, r5, #0x1
 8004252: ea80 70e5    	eor.w	r0, r0, r5, asr #31
 8004256: ea4f 0541    	lsl.w	r5, r1, #0x1
 800425a: ea85 71e1    	eor.w	r1, r5, r1, asr #31
 800425e: eb09 0503    	add.w	r5, r9, r3
 8004262: 4441         	add	r1, r8
 8004264: f8d2 8078    	ldr.w	r8, [r2, #0x78]
 8004268: fa45 f504    	asr.w	r5, r5, r4
 800426c: eba5 0606    	sub.w	r6, r5, r6
 8004270: 4408         	add	r0, r1
 8004272: 6615         	str	r5, [r2, #0x60]
 8004274: ea4f 0146    	lsl.w	r1, r6, #0x1
 8004278: 4470         	add	r0, lr
 800427a: ea81 71e6    	eor.w	r1, r1, r6, asr #31
 800427e: f8d2 e090    	ldr.w	lr, [r2, #0x90]
 8004282: 4408         	add	r0, r1
 8004284: eb08 0103    	add.w	r1, r8, r3
 8004288: f8d2 80a8    	ldr.w	r8, [r2, #0xa8]
 800428c: fa41 f104    	asr.w	r1, r1, r4
 8004290: eba1 0505    	sub.w	r5, r1, r5
 8004294: 6791         	str	r1, [r2, #0x78]
 8004296: ea4f 0645    	lsl.w	r6, r5, #0x1
 800429a: ea86 75e5    	eor.w	r5, r6, r5, asr #31
 800429e: 4428         	add	r0, r5
 80042a0: eb0e 0503    	add.w	r5, lr, r3
 80042a4: fa45 f504    	asr.w	r5, r5, r4
 80042a8: eba5 0101    	sub.w	r1, r5, r1
 80042ac: f8c2 5090    	str.w	r5, [r2, #0x90]
 80042b0: ea4f 0641    	lsl.w	r6, r1, #0x1
 80042b4: ea86 71e1    	eor.w	r1, r6, r1, asr #31
 80042b8: eb00 0e01    	add.w	lr, r0, r1
 80042bc: eb08 0103    	add.w	r1, r8, r3
 80042c0: f8d2 60f0    	ldr.w	r6, [r2, #0xf0]
 80042c4: fa41 f104    	asr.w	r1, r1, r4
 80042c8: eba1 0505    	sub.w	r5, r1, r5
 80042cc: f8c2 10a8    	str.w	r1, [r2, #0xa8]
 80042d0: ea4f 0045    	lsl.w	r0, r5, #0x1
 80042d4: ea80 78e5    	eor.w	r8, r0, r5, asr #31
 80042d8: eb06 0503    	add.w	r5, r6, r3
 80042dc: f8d2 00c0    	ldr.w	r0, [r2, #0xc0]
 80042e0: f8d2 6108    	ldr.w	r6, [r2, #0x108]
 80042e4: 44c6         	add	lr, r8
 80042e6: fa45 f904    	asr.w	r9, r5, r4
 80042ea: f8d2 50d8    	ldr.w	r5, [r2, #0xd8]
 80042ee: 4418         	add	r0, r3
 80042f0: 441e         	add	r6, r3
 80042f2: 441d         	add	r5, r3
 80042f4: f8c2 90f0    	str.w	r9, [r2, #0xf0]
 80042f8: fa40 f004    	asr.w	r0, r0, r4
 80042fc: eba0 0101    	sub.w	r1, r0, r1
 8004300: fa45 f504    	asr.w	r5, r5, r4
 8004304: f8c2 00c0    	str.w	r0, [r2, #0xc0]
 8004308: eba5 0000    	sub.w	r0, r5, r0
 800430c: fa46 fb04    	asr.w	r11, r6, r4
 8004310: ea4f 0641    	lsl.w	r6, r1, #0x1
 8004314: ea86 71e1    	eor.w	r1, r6, r1, asr #31
 8004318: ea4f 0640    	lsl.w	r6, r0, #0x1
 800431c: 4471         	add	r1, lr
 800431e: ea86 70e0    	eor.w	r0, r6, r0, asr #31
 8004322: f8c2 50d8    	str.w	r5, [r2, #0xd8]
 8004326: 4408         	add	r0, r1
 8004328: eba9 0105    	sub.w	r1, r9, r5
 800432c: f8c2 b108    	str.w	r11, [r2, #0x108]
 8004330: f502 7284    	add.w	r2, r2, #0x108
 8004334: ea4f 0641    	lsl.w	r6, r1, #0x1
 8004338: ea86 71e1    	eor.w	r1, r6, r1, asr #31
 800433c: 4408         	add	r0, r1
 800433e: ebab 0109    	sub.w	r1, r11, r9
 8004342: ea4f 0641    	lsl.w	r6, r1, #0x1
 8004346: ea86 71e1    	eor.w	r1, r6, r1, asr #31
 800434a: eb00 0801    	add.w	r8, r0, r1
 800434e: f47f af5f    	bne.w	0x8004210 <raylar_crcl_encoder_service::benchmark::fused::h66edd02a0d627444+0x30a> @ imm = #-0x142
 8004352: f8cd 8024    	str.w	r8, [sp, #0x24]
 8004356: a809         	add	r0, sp, #0x24
 8004358: 2301         	movs	r3, #0x1
 800435a: f04f 0e00    	mov.w	lr, #0x0
 800435e: 9809         	ldr	r0, [sp, #0x24]
 8004360: f240 3cff    	movw	r12, #0x3ff
 8004364: 9001         	str	r0, [sp, #0x4]
 8004366: 9805         	ldr	r0, [sp, #0x14]
 8004368: 6980         	ldr	r0, [r0, #0x18]
 800436a: 9008         	str	r0, [sp, #0x20]
 800436c: a808         	add	r0, sp, #0x20
 800436e: 9808         	ldr	r0, [sp, #0x20]
 8004370: 9e04         	ldr	r6, [sp, #0x10]
 8004372: 1e42         	subs	r2, r0, #0x1
 8004374: f000 041f    	and	r4, r0, #0x1f
 8004378: f002 021f    	and	r2, r2, #0x1f
 800437c: f506 758a    	add.w	r5, r6, #0x114
 8004380: 68f1         	ldr	r1, [r6, #0xc]
 8004382: 4093         	lsls	r3, r2
 8004384: 4419         	add	r1, r3
 8004386: fa41 fb04    	asr.w	r11, r1, r4
 800438a: f8c6 b00c    	str.w	r11, [r6, #0xc]
 800438e: f855 6cd8    	ldr	r6, [r5, #-216]
 8004392: f1bc 0c0b    	subs.w	r12, r12, #0xb
 8004396: f855 2cc0    	ldr	r2, [r5, #-192]
 800439a: 441e         	add	r6, r3
 800439c: f855 acf0    	ldr	r10, [r5, #-240]
 80043a0: 441a         	add	r2, r3
 80043a2: f855 8ca8    	ldr	r8, [r5, #-168]
 80043a6: fa46 f604    	asr.w	r6, r6, r4
 80043aa: fa42 f204    	asr.w	r2, r2, r4
 80043ae: eba2 0006    	sub.w	r0, r2, r6
 80043b2: f845 6cd8    	str	r6, [r5, #-216]
 80043b6: ea4f 0140    	lsl.w	r1, r0, #0x1
 80043ba: ea81 79e0    	eor.w	r9, r1, r0, asr #31
 80043be: eb0a 0103    	add.w	r1, r10, r3
 80043c2: f845 2cc0    	str	r2, [r5, #-192]
 80043c6: fa41 f104    	asr.w	r1, r1, r4
 80043ca: eba6 0601    	sub.w	r6, r6, r1
 80043ce: f845 1cf0    	str	r1, [r5, #-240]
 80043d2: eba1 010b    	sub.w	r1, r1, r11
 80043d6: ea4f 0046    	lsl.w	r0, r6, #0x1
 80043da: ea80 70e6    	eor.w	r0, r0, r6, asr #31
 80043de: ea4f 0641    	lsl.w	r6, r1, #0x1
 80043e2: ea86 71e1    	eor.w	r1, r6, r1, asr #31
 80043e6: eb08 0603    	add.w	r6, r8, r3
 80043ea: 4471         	add	r1, lr
 80043ec: f855 ec90    	ldr	lr, [r5, #-144]
 80043f0: fa46 f604    	asr.w	r6, r6, r4
 80043f4: eba6 0202    	sub.w	r2, r6, r2
 80043f8: 4408         	add	r0, r1
 80043fa: f845 6ca8    	str	r6, [r5, #-168]
 80043fe: ea4f 0142    	lsl.w	r1, r2, #0x1
 8004402: 4448         	add	r0, r9
 8004404: ea81 71e2    	eor.w	r1, r1, r2, asr #31
 8004408: f855 8c78    	ldr	r8, [r5, #-120]
 800440c: 4408         	add	r0, r1
 800440e: eb0e 0103    	add.w	r1, lr, r3
 8004412: f855 ec60    	ldr	lr, [r5, #-96]
 8004416: fa41 f104    	asr.w	r1, r1, r4
 800441a: eba1 0606    	sub.w	r6, r1, r6
 800441e: f845 1c90    	str	r1, [r5, #-144]
 8004422: ea4f 0246    	lsl.w	r2, r6, #0x1
 8004426: ea82 72e6    	eor.w	r2, r2, r6, asr #31
 800442a: 4410         	add	r0, r2
 800442c: eb08 0203    	add.w	r2, r8, r3
 8004430: fa42 f204    	asr.w	r2, r2, r4
 8004434: eba2 0101    	sub.w	r1, r2, r1
 8004438: f845 2c78    	str	r2, [r5, #-120]
 800443c: ea4f 0641    	lsl.w	r6, r1, #0x1
 8004440: ea86 71e1    	eor.w	r1, r6, r1, asr #31
 8004444: eb00 0801    	add.w	r8, r0, r1
 8004448: eb0e 0103    	add.w	r1, lr, r3
 800444c: f855 6c18    	ldr	r6, [r5, #-24]
 8004450: fa41 f104    	asr.w	r1, r1, r4
 8004454: eba1 0202    	sub.w	r2, r1, r2
 8004458: f845 1c60    	str	r1, [r5, #-96]
 800445c: ea4f 0042    	lsl.w	r0, r2, #0x1
 8004460: ea80 7ee2    	eor.w	lr, r0, r2, asr #31
 8004464: eb06 0203    	add.w	r2, r6, r3
 8004468: f855 0c48    	ldr	r0, [r5, #-72]
 800446c: f855 6c30    	ldr	r6, [r5, #-48]
 8004470: 44c6         	add	lr, r8
 8004472: fa42 f904    	asr.w	r9, r2, r4
 8004476: 4418         	add	r0, r3
 8004478: 682a         	ldr	r2, [r5]
 800447a: 441e         	add	r6, r3
 800447c: fa40 f004    	asr.w	r0, r0, r4
 8004480: eba0 0101    	sub.w	r1, r0, r1
 8004484: 441a         	add	r2, r3
 8004486: fa46 f604    	asr.w	r6, r6, r4
 800448a: f845 0c48    	str	r0, [r5, #-72]
 800448e: eba6 0000    	sub.w	r0, r6, r0
 8004492: fa42 fb04    	asr.w	r11, r2, r4
 8004496: ea4f 0241    	lsl.w	r2, r1, #0x1
 800449a: ea82 71e1    	eor.w	r1, r2, r1, asr #31
 800449e: ea4f 0240    	lsl.w	r2, r0, #0x1
 80044a2: 4471         	add	r1, lr
 80044a4: ea82 70e0    	eor.w	r0, r2, r0, asr #31
 80044a8: 4408         	add	r0, r1
 80044aa: eba9 0106    	sub.w	r1, r9, r6
 80044ae: f845 6c30    	str	r6, [r5, #-48]
 80044b2: ea4f 0241    	lsl.w	r2, r1, #0x1
 80044b6: ea82 71e1    	eor.w	r1, r2, r1, asr #31
 80044ba: 4408         	add	r0, r1
 80044bc: ebab 0109    	sub.w	r1, r11, r9
 80044c0: f845 9c18    	str	r9, [r5, #-24]
 80044c4: ea4f 0241    	lsl.w	r2, r1, #0x1
 80044c8: ea82 71e1    	eor.w	r1, r2, r1, asr #31
 80044cc: f8c5 b000    	str.w	r11, [r5]
 80044d0: eb00 0e01    	add.w	lr, r0, r1
 80044d4: f505 7584    	add.w	r5, r5, #0x108
 80044d8: f47f af59    	bne.w	0x800438e <raylar_crcl_encoder_service::benchmark::fused::h66edd02a0d627444+0x488> @ imm = #-0x14e
 80044dc: f8cd e024    	str.w	lr, [sp, #0x24]
 80044e0: a809         	add	r0, sp, #0x24
 80044e2: 2601         	movs	r6, #0x1
 80044e4: f04f 0900    	mov.w	r9, #0x0
 80044e8: 9809         	ldr	r0, [sp, #0x24]
 80044ea: 9000         	str	r0, [sp]
 80044ec: 9805         	ldr	r0, [sp, #0x14]
 80044ee: 6a00         	ldr	r0, [r0, #0x20]
 80044f0: 9008         	str	r0, [sp, #0x20]
 80044f2: a808         	add	r0, sp, #0x20
 80044f4: 9808         	ldr	r0, [sp, #0x20]
 80044f6: f8dd e010    	ldr.w	lr, [sp, #0x10]
 80044fa: 1e42         	subs	r2, r0, #0x1
 80044fc: f000 051f    	and	r5, r0, #0x1f
 8004500: f002 021f    	and	r2, r2, #0x1f
 8004504: f240 30ff    	movw	r0, #0x3ff
 8004508: f85e 1f10    	ldr	r1, [lr, #16]!
 800450c: fa06 f402    	lsl.w	r4, r6, r2
 8004510: 4421         	add	r1, r4
 8004512: fa41 fc05    	asr.w	r12, r1, r5
 8004516: f8ce c000    	str.w	r12, [lr]
 800451a: f8de 3030    	ldr.w	r3, [lr, #0x30]
 800451e: f8de 6048    	ldr.w	r6, [lr, #0x48]
 8004522: 4423         	add	r3, r4
 8004524: f8de a018    	ldr.w	r10, [lr, #0x18]
 8004528: 4426         	add	r6, r4
 800452a: 9006         	str	r0, [sp, #0x18]
 800452c: 412b         	asrs	r3, r5
 800452e: f8de 8060    	ldr.w	r8, [lr, #0x60]
 8004532: 412e         	asrs	r6, r5
 8004534: 1af2         	subs	r2, r6, r3
 8004536: f8ce 3030    	str.w	r3, [lr, #0x30]
 800453a: 0050         	lsls	r0, r2, #0x1
 800453c: ea80 7be2    	eor.w	r11, r0, r2, asr #31
 8004540: eb0a 0204    	add.w	r2, r10, r4
 8004544: f8ce 6048    	str.w	r6, [lr, #0x48]
 8004548: 412a         	asrs	r2, r5
 800454a: 1a9b         	subs	r3, r3, r2
 800454c: eba2 010c    	sub.w	r1, r2, r12
 8004550: f8ce 2018    	str.w	r2, [lr, #0x18]
 8004554: 0058         	lsls	r0, r3, #0x1
 8004556: 004a         	lsls	r2, r1, #0x1
 8004558: ea82 71e1    	eor.w	r1, r2, r1, asr #31
 800455c: ea80 70e3    	eor.w	r0, r0, r3, asr #31
 8004560: 4449         	add	r1, r9
 8004562: 4408         	add	r0, r1
 8004564: f8de 1078    	ldr.w	r1, [lr, #0x78]
 8004568: eb08 0204    	add.w	r2, r8, r4
 800456c: 4458         	add	r0, r11
 800456e: 4421         	add	r1, r4
 8004570: 412a         	asrs	r2, r5
 8004572: 1b93         	subs	r3, r2, r6
 8004574: 4129         	asrs	r1, r5
 8004576: f8ce 2060    	str.w	r2, [lr, #0x60]
 800457a: 005e         	lsls	r6, r3, #0x1
 800457c: 1a8a         	subs	r2, r1, r2
 800457e: ea86 73e3    	eor.w	r3, r6, r3, asr #31
 8004582: f8de 6090    	ldr.w	r6, [lr, #0x90]
 8004586: 4418         	add	r0, r3
 8004588: 0053         	lsls	r3, r2, #0x1
 800458a: ea83 72e2    	eor.w	r2, r3, r2, asr #31
 800458e: f8ce 1078    	str.w	r1, [lr, #0x78]
 8004592: 4410         	add	r0, r2
 8004594: 1932         	adds	r2, r6, r4
 8004596: f8de 30a8    	ldr.w	r3, [lr, #0xa8]
 800459a: 412a         	asrs	r2, r5
 800459c: 1a51         	subs	r1, r2, r1
 800459e: f8ce 2090    	str.w	r2, [lr, #0x90]
 80045a2: 004e         	lsls	r6, r1, #0x1
 80045a4: ea86 71e1    	eor.w	r1, r6, r1, asr #31
 80045a8: eb00 0801    	add.w	r8, r0, r1
 80045ac: 1919         	adds	r1, r3, r4
 80045ae: f8de 60f0    	ldr.w	r6, [lr, #0xf0]
 80045b2: fa41 f305    	asr.w	r3, r1, r5
 80045b6: 1a99         	subs	r1, r3, r2
 80045b8: f8ce 30a8    	str.w	r3, [lr, #0xa8]
 80045bc: 004a         	lsls	r2, r1, #0x1
 80045be: ea82 79e1    	eor.w	r9, r2, r1, asr #31
 80045c2: 1931         	adds	r1, r6, r4
 80045c4: 44c8         	add	r8, r9
 80045c6: fa41 f205    	asr.w	r2, r1, r5
 80045ca: f8de 10d8    	ldr.w	r1, [lr, #0xd8]
 80045ce: f8ce 20f0    	str.w	r2, [lr, #0xf0]
 80045d2: 4421         	add	r1, r4
 80045d4: fa41 f605    	asr.w	r6, r1, r5
 80045d8: f8de 10c0    	ldr.w	r1, [lr, #0xc0]
 80045dc: f8ce 60d8    	str.w	r6, [lr, #0xd8]
 80045e0: 4421         	add	r1, r4
 80045e2: fa41 f005    	asr.w	r0, r1, r5
 80045e6: f8de 1108    	ldr.w	r1, [lr, #0x108]
 80045ea: 1ac3         	subs	r3, r0, r3
 80045ec: f8ce 00c0    	str.w	r0, [lr, #0xc0]
 80045f0: 4421         	add	r1, r4
 80045f2: 1a30         	subs	r0, r6, r0
 80045f4: fa41 fc05    	asr.w	r12, r1, r5
 80045f8: 0059         	lsls	r1, r3, #0x1
 80045fa: ea81 71e3    	eor.w	r1, r1, r3, asr #31
 80045fe: 0043         	lsls	r3, r0, #0x1
 8004600: 4441         	add	r1, r8
 8004602: ea83 70e0    	eor.w	r0, r3, r0, asr #31
 8004606: 4408         	add	r0, r1
 8004608: 1b91         	subs	r1, r2, r6
 800460a: f8ce c108    	str.w	r12, [lr, #0x108]
 800460e: f50e 7e84    	add.w	lr, lr, #0x108
 8004612: 004b         	lsls	r3, r1, #0x1
 8004614: ea83 71e1    	eor.w	r1, r3, r1, asr #31
 8004618: 4408         	add	r0, r1
 800461a: ebac 0102    	sub.w	r1, r12, r2
 800461e: 004a         	lsls	r2, r1, #0x1
 8004620: ea82 71e1    	eor.w	r1, r2, r1, asr #31
 8004624: eb00 0901    	add.w	r9, r0, r1
 8004628: 9806         	ldr	r0, [sp, #0x18]
 800462a: 380b         	subs	r0, #0xb
 800462c: f47f af75    	bne.w	0x800451a <raylar_crcl_encoder_service::benchmark::fused::h66edd02a0d627444+0x614> @ imm = #-0x116
 8004630: f8cd 9024    	str.w	r9, [sp, #0x24]
 8004634: a809         	add	r0, sp, #0x24
 8004636: f246 0128    	movw	r1, #0x6028
 800463a: 9a05         	ldr	r2, [sp, #0x14]
 800463c: 9809         	ldr	r0, [sp, #0x24]
 800463e: 4411         	add	r1, r2
 8004640: 9107         	str	r1, [sp, #0x1c]
 8004642: a907         	add	r1, sp, #0x1c
 8004644: e9dd 2102    	ldrd	r2, r1, [sp, #8]
 8004648: 4051         	eors	r1, r2
 800464a: 9a01         	ldr	r2, [sp, #0x4]
 800464c: 4051         	eors	r1, r2
 800464e: 9a00         	ldr	r2, [sp]
 8004650: 4051         	eors	r1, r2
 8004652: 4048         	eors	r0, r1
 8004654: b00a         	add	sp, #0x28
 8004656: e8bd 0f00    	pop.w	{r8, r9, r10, r11}
 800465a: bdf0         	pop	{r4, r5, r6, r7, pc}

0800465c <raylar_crcl_encoder_service::benchmark::quant::h5a540275f2df62e3>:
 800465c: b580         	push	{r7, lr}
 800465e: 466f         	mov	r7, sp
 8004660: f246 0128    	movw	r1, #0x6028
 8004664: 1842         	adds	r2, r0, r1
 8004666: f24c 0128    	movw	r1, #0xc028
 800466a: 4401         	add	r1, r0
 800466c: 4610         	mov	r0, r2
 800466e: f44f 42c0    	mov.w	r2, #0x6000
 8004672: f008 fae8    	bl	0x800cc46 <__aeabi_memcpy4> @ imm = #0x85d0
 8004676: bd80         	pop	{r7, pc}

08004678 <raylar_crcl_encoder_service::benchmark::scale::h44b3d035bda7af78>:
 8004678: b5f0         	push	{r4, r5, r6, r7, lr}
 800467a: af03         	add	r7, sp, #0xc
 800467c: e92d 0f00    	push.w	{r8, r9, r10, r11}
 8004680: b086         	sub	sp, #0x18
 8004682: f10d 0e0c    	add.w	lr, sp, #0xc
 8004686: f1a7 0122    	sub.w	r1, r7, #0x22
 800468a: f1a7 0b21    	sub.w	r11, r7, #0x21
 800468e: 2000         	movs	r0, #0x0
 8004690: f44f 7380    	mov.w	r3, #0x100
 8004694: f04f 080a    	mov.w	r8, #0xa
 8004698: f04f 0c06    	mov.w	r12, #0x6
 800469c: e030         	b	0x8004700 <raylar_crcl_encoder_service::benchmark::scale::h44b3d035bda7af78+0x88> @ imm = #0x60
 800469e: eba5 0806    	sub.w	r8, r5, r6
 80046a2: 2101         	movs	r1, #0x1
 80046a4: f1a8 0501    	sub.w	r5, r8, #0x1
 80046a8: f003 031f    	and	r3, r3, #0x1f
 80046ac: fa01 f505    	lsl.w	r5, r1, r5
 80046b0: 442a         	add	r2, r5
 80046b2: f008 051f    	and	r5, r8, #0x1f
 80046b6: 40ea         	lsrs	r2, r5
 80046b8: 40da         	lsrs	r2, r3
 80046ba: bf18         	it	ne
 80046bc: f108 0801    	addne.w	r8, r8, #0x1
 80046c0: e9dd 1301    	ldrd	r1, r3, [sp, #4]
 80046c4: 46a6         	mov	lr, r4
 80046c6: 4048         	eors	r0, r1
 80046c8: 9900         	ldr	r1, [sp]
 80046ca: 4048         	eors	r0, r1
 80046cc: a905         	add	r1, sp, #0x14
 80046ce: ea80 000b    	eor.w	r0, r0, r11
 80046d2: 46cb         	mov	r11, r9
 80046d4: ea80 000c    	eor.w	r0, r0, r12
 80046d8: f04f 0c06    	mov.w	r12, #0x6
 80046dc: f807 cc21    	strb	r12, [r7, #-33]
 80046e0: f817 2c21    	ldrb	r2, [r7, #-33]
 80046e4: 4542         	cmp	r2, r8
 80046e6: bf88         	it	hi
 80046e8: 4690         	movhi	r8, r2
 80046ea: f8cd 8014    	str.w	r8, [sp, #0x14]
 80046ee: 3b01         	subs	r3, #0x1
 80046f0: f04f 080a    	mov.w	r8, #0xa
 80046f4: 4651         	mov	r1, r10
 80046f6: 9a05         	ldr	r2, [sp, #0x14]
 80046f8: ea80 0002    	eor.w	r0, r0, r2
 80046fc: f000 80c2    	beq.w	0x8004884 <raylar_crcl_encoder_service::benchmark::scale::h44b3d035bda7af78+0x20c> @ imm = #0x184
 8004700: f245 2288    	movw	r2, #0x5288
 8004704: 9302         	str	r3, [sp, #0x8]
 8004706: 9203         	str	r2, [sp, #0xc]
 8004708: f807 8c22    	strb	r8, [r7, #-34]
 800470c: 9b03         	ldr	r3, [sp, #0xc]
 800470e: f817 6c22    	ldrb	r6, [r7, #-34]
 8004712: fab3 f283    	clz	r2, r3
 8004716: 1e75         	subs	r5, r6, #0x1
 8004718: f1c2 0220    	rsb.w	r2, r2, #0x20
 800471c: b2ee         	uxtb	r6, r5
 800471e: 42b2         	cmp	r2, r6
 8004720: d90e         	bls	0x8004740 <raylar_crcl_encoder_service::benchmark::scale::h44b3d035bda7af78+0xc8> @ imm = #0x1c
 8004722: 1b92         	subs	r2, r2, r6
 8004724: 2401         	movs	r4, #0x1
 8004726: 1e56         	subs	r6, r2, #0x1
 8004728: fa04 f606    	lsl.w	r6, r4, r6
 800472c: 4433         	add	r3, r6
 800472e: f002 061f    	and	r6, r2, #0x1f
 8004732: 40f3         	lsrs	r3, r6
 8004734: f005 061f    	and	r6, r5, #0x1f
 8004738: 40f3         	lsrs	r3, r6
 800473a: bf18         	it	ne
 800473c: 3201         	addne	r2, #0x1
 800473e: e000         	b	0x8004742 <raylar_crcl_encoder_service::benchmark::scale::h44b3d035bda7af78+0xca> @ imm = #0x0
 8004740: 2200         	movs	r2, #0x0
 8004742: f807 cc21    	strb	r12, [r7, #-33]
 8004746: f10d 0a14    	add.w	r10, sp, #0x14
 800474a: f817 3c21    	ldrb	r3, [r7, #-33]
 800474e: 4293         	cmp	r3, r2
 8004750: bf88         	it	hi
 8004752: 461a         	movhi	r2, r3
 8004754: 9205         	str	r2, [sp, #0x14]
 8004756: 9a05         	ldr	r2, [sp, #0x14]
 8004758: 9201         	str	r2, [sp, #0x4]
 800475a: f245 6270    	movw	r2, #0x5670
 800475e: 9203         	str	r2, [sp, #0xc]
 8004760: f807 8c22    	strb	r8, [r7, #-34]
 8004764: 9b03         	ldr	r3, [sp, #0xc]
 8004766: f817 6c22    	ldrb	r6, [r7, #-34]
 800476a: fab3 f283    	clz	r2, r3
 800476e: 1e75         	subs	r5, r6, #0x1
 8004770: f1c2 0220    	rsb.w	r2, r2, #0x20
 8004774: b2ee         	uxtb	r6, r5
 8004776: 42b2         	cmp	r2, r6
 8004778: d90e         	bls	0x8004798 <raylar_crcl_encoder_service::benchmark::scale::h44b3d035bda7af78+0x120> @ imm = #0x1c
 800477a: 1b92         	subs	r2, r2, r6
 800477c: 2401         	movs	r4, #0x1
 800477e: 1e56         	subs	r6, r2, #0x1
 8004780: fa04 f606    	lsl.w	r6, r4, r6
 8004784: 4433         	add	r3, r6
 8004786: f002 061f    	and	r6, r2, #0x1f
 800478a: 40f3         	lsrs	r3, r6
 800478c: f005 061f    	and	r6, r5, #0x1f
 8004790: 40f3         	lsrs	r3, r6
 8004792: bf18         	it	ne
 8004794: 3201         	addne	r2, #0x1
 8004796: e000         	b	0x800479a <raylar_crcl_encoder_service::benchmark::scale::h44b3d035bda7af78+0x122> @ imm = #0x0
 8004798: 2200         	movs	r2, #0x0
 800479a: f807 cc21    	strb	r12, [r7, #-33]
 800479e: f817 3c21    	ldrb	r3, [r7, #-33]
 80047a2: 4293         	cmp	r3, r2
 80047a4: bf88         	it	hi
 80047a6: 461a         	movhi	r2, r3
 80047a8: 9205         	str	r2, [sp, #0x14]
 80047aa: 9a05         	ldr	r2, [sp, #0x14]
 80047ac: 9200         	str	r2, [sp]
 80047ae: f645 2258    	movw	r2, #0x5a58
 80047b2: 9203         	str	r2, [sp, #0xc]
 80047b4: f807 8c22    	strb	r8, [r7, #-34]
 80047b8: 9b03         	ldr	r3, [sp, #0xc]
 80047ba: f817 6c22    	ldrb	r6, [r7, #-34]
 80047be: fab3 f283    	clz	r2, r3
 80047c2: 1e75         	subs	r5, r6, #0x1
 80047c4: f1c2 0220    	rsb.w	r2, r2, #0x20
 80047c8: b2ee         	uxtb	r6, r5
 80047ca: 42b2         	cmp	r2, r6
 80047cc: d90e         	bls	0x80047ec <raylar_crcl_encoder_service::benchmark::scale::h44b3d035bda7af78+0x174> @ imm = #0x1c
 80047ce: 1b92         	subs	r2, r2, r6
 80047d0: 2401         	movs	r4, #0x1
 80047d2: 1e56         	subs	r6, r2, #0x1
 80047d4: fa04 f606    	lsl.w	r6, r4, r6
 80047d8: 4433         	add	r3, r6
 80047da: f002 061f    	and	r6, r2, #0x1f
 80047de: 40f3         	lsrs	r3, r6
 80047e0: f005 061f    	and	r6, r5, #0x1f
 80047e4: 40f3         	lsrs	r3, r6
 80047e6: bf18         	it	ne
 80047e8: 3201         	addne	r2, #0x1
 80047ea: e000         	b	0x80047ee <raylar_crcl_encoder_service::benchmark::scale::h44b3d035bda7af78+0x176> @ imm = #0x0
 80047ec: 2200         	movs	r2, #0x0
 80047ee: f807 cc21    	strb	r12, [r7, #-33]
 80047f2: 46d9         	mov	r9, r11
 80047f4: f817 3c21    	ldrb	r3, [r7, #-33]
 80047f8: 4293         	cmp	r3, r2
 80047fa: bf88         	it	hi
 80047fc: 461a         	movhi	r2, r3
 80047fe: 9205         	str	r2, [sp, #0x14]
 8004800: f645 6240    	movw	r2, #0x5e40
 8004804: 9203         	str	r2, [sp, #0xc]
 8004806: f8dd b014    	ldr.w	r11, [sp, #0x14]
 800480a: f807 8c22    	strb	r8, [r7, #-34]
 800480e: 9b03         	ldr	r3, [sp, #0xc]
 8004810: f817 6c22    	ldrb	r6, [r7, #-34]
 8004814: fab3 f283    	clz	r2, r3
 8004818: 1e75         	subs	r5, r6, #0x1
 800481a: f1c2 0220    	rsb.w	r2, r2, #0x20
 800481e: b2ee         	uxtb	r6, r5
 8004820: 42b2         	cmp	r2, r6
 8004822: d90e         	bls	0x8004842 <raylar_crcl_encoder_service::benchmark::scale::h44b3d035bda7af78+0x1ca> @ imm = #0x1c
 8004824: 1b92         	subs	r2, r2, r6
 8004826: 2401         	movs	r4, #0x1
 8004828: 1e56         	subs	r6, r2, #0x1
 800482a: fa04 f606    	lsl.w	r6, r4, r6
 800482e: 4433         	add	r3, r6
 8004830: f002 061f    	and	r6, r2, #0x1f
 8004834: 40f3         	lsrs	r3, r6
 8004836: f005 061f    	and	r6, r5, #0x1f
 800483a: 40f3         	lsrs	r3, r6
 800483c: bf18         	it	ne
 800483e: 3201         	addne	r2, #0x1
 8004840: e000         	b	0x8004844 <raylar_crcl_encoder_service::benchmark::scale::h44b3d035bda7af78+0x1cc> @ imm = #0x0
 8004842: 2200         	movs	r2, #0x0
 8004844: f807 cc21    	strb	r12, [r7, #-33]
 8004848: 4674         	mov	r4, lr
 800484a: f817 3c21    	ldrb	r3, [r7, #-33]
 800484e: 4293         	cmp	r3, r2
 8004850: bf88         	it	hi
 8004852: 461a         	movhi	r2, r3
 8004854: 9205         	str	r2, [sp, #0x14]
 8004856: f246 2228    	movw	r2, #0x6228
 800485a: 9203         	str	r2, [sp, #0xc]
 800485c: 468a         	mov	r10, r1
 800485e: f8dd c014    	ldr.w	r12, [sp, #0x14]
 8004862: f807 8c22    	strb	r8, [r7, #-34]
 8004866: 9a03         	ldr	r2, [sp, #0xc]
 8004868: f817 3c22    	ldrb	r3, [r7, #-34]
 800486c: fab2 f582    	clz	r5, r2
 8004870: 3b01         	subs	r3, #0x1
 8004872: f1c5 0520    	rsb.w	r5, r5, #0x20
 8004876: b2de         	uxtb	r6, r3
 8004878: 42b5         	cmp	r5, r6
 800487a: f63f af10    	bhi.w	0x800469e <raylar_crcl_encoder_service::benchmark::scale::h44b3d035bda7af78+0x26> @ imm = #-0x1e0
 800487e: f04f 0800    	mov.w	r8, #0x0
 8004882: e71d         	b	0x80046c0 <raylar_crcl_encoder_service::benchmark::scale::h44b3d035bda7af78+0x48> @ imm = #-0x1c6
 8004884: b006         	add	sp, #0x18
 8004886: e8bd 0f00    	pop.w	{r8, r9, r10, r11}
 800488a: bdf0         	pop	{r4, r5, r6, r7, pc}

0800488c <raylar_crcl_encoder_service::benchmark::rice_k::h2637d4bce634509f>:
 800488c: b5f0         	push	{r4, r5, r6, r7, lr}
 800488e: af03         	add	r7, sp, #0xc
 8004890: e92d 0f00    	push.w	{r8, r9, r10, r11}
 8004894: b08a         	sub	sp, #0x28
 8004896: f64f 41b0    	movw	r1, #0xfcb0
 800489a: f10d 081c    	add.w	r8, sp, #0x1c
 800489e: f2c0 0101    	movt	r1, #0x1
 80048a2: 1842         	adds	r2, r0, r1
 80048a4: 5840         	ldr	r0, [r0, r1]
 80048a6: f10d 0e20    	add.w	lr, sp, #0x20
 80048aa: 6851         	ldr	r1, [r2, #0x4]
 80048ac: 9103         	str	r1, [sp, #0xc]
 80048ae: ac09         	add	r4, sp, #0x24
 80048b0: 6891         	ldr	r1, [r2, #0x8]
 80048b2: f44f 7980    	mov.w	r9, #0x100
 80048b6: 9102         	str	r1, [sp, #0x8]
 80048b8: f240 3bff    	movw	r11, #0x3ff
 80048bc: 68d1         	ldr	r1, [r2, #0xc]
 80048be: 9004         	str	r0, [sp, #0x10]
 80048c0: 2000         	movs	r0, #0x0
 80048c2: 9101         	str	r1, [sp, #0x4]
 80048c4: 6911         	ldr	r1, [r2, #0x10]
 80048c6: 9100         	str	r1, [sp]
 80048c8: e017         	b	0x80048fa <raylar_crcl_encoder_service::benchmark::rice_k::h2637d4bce634509f+0x6e> @ imm = #0x2e
 80048ca: 9906         	ldr	r1, [sp, #0x18]
 80048cc: f1b9 0901    	subs.w	r9, r9, #0x1
 80048d0: f8cd 8024    	str.w	r8, [sp, #0x24]
 80048d4: 462c         	mov	r4, r5
 80048d6: ea80 0001    	eor.w	r0, r0, r1
 80048da: 9905         	ldr	r1, [sp, #0x14]
 80048dc: 46d0         	mov	r8, r10
 80048de: 4693         	mov	r11, r2
 80048e0: ea80 0001    	eor.w	r0, r0, r1
 80048e4: 9909         	ldr	r1, [sp, #0x24]
 80048e6: ea80 000c    	eor.w	r0, r0, r12
 80048ea: ea80 000e    	eor.w	r0, r0, lr
 80048ee: f10d 0e20    	add.w	lr, sp, #0x20
 80048f2: ea80 0001    	eor.w	r0, r0, r1
 80048f6: f000 8087    	beq.w	0x8004a08 <raylar_crcl_encoder_service::benchmark::rice_k::h2637d4bce634509f+0x17c> @ imm = #0x10e
 80048fa: 9904         	ldr	r1, [sp, #0x10]
 80048fc: 2200         	movs	r2, #0x0
 80048fe: 9107         	str	r1, [sp, #0x1c]
 8004900: f8cd b020    	str.w	r11, [sp, #0x20]
 8004904: 9e07         	ldr	r6, [sp, #0x1c]
 8004906: 2e00         	cmp	r6, #0x0
 8004908: bf1c         	itt	ne
 800490a: 9b08         	ldrne	r3, [sp, #0x20]
 800490c: 2b00         	cmpne	r3, #0x0
 800490e: d00b         	beq	0x8004928 <raylar_crcl_encoder_service::benchmark::rice_k::h2637d4bce634509f+0x9c> @ imm = #0x16
 8004910: fbb6 f1f3    	udiv	r1, r6, r3
 8004914: fb01 6213    	mls	r2, r1, r3, r6
 8004918: 2a00         	cmp	r2, #0x0
 800491a: bf18         	it	ne
 800491c: 3101         	addne	r1, #0x1
 800491e: fab1 f181    	clz	r1, r1
 8004922: f1c1 011f    	rsb.w	r1, r1, #0x1f
 8004926: b2ca         	uxtb	r2, r1
 8004928: 9209         	str	r2, [sp, #0x24]
 800492a: 2500         	movs	r5, #0x0
 800492c: 9909         	ldr	r1, [sp, #0x24]
 800492e: 9106         	str	r1, [sp, #0x18]
 8004930: 9903         	ldr	r1, [sp, #0xc]
 8004932: 9107         	str	r1, [sp, #0x1c]
 8004934: f8cd b020    	str.w	r11, [sp, #0x20]
 8004938: 9e07         	ldr	r6, [sp, #0x1c]
 800493a: 2e00         	cmp	r6, #0x0
 800493c: bf1c         	itt	ne
 800493e: 9b08         	ldrne	r3, [sp, #0x20]
 8004940: 2b00         	cmpne	r3, #0x0
 8004942: d00b         	beq	0x800495c <raylar_crcl_encoder_service::benchmark::rice_k::h2637d4bce634509f+0xd0> @ imm = #0x16
 8004944: fbb6 f1f3    	udiv	r1, r6, r3
 8004948: fb01 6313    	mls	r3, r1, r3, r6
 800494c: 2b00         	cmp	r3, #0x0
 800494e: bf18         	it	ne
 8004950: 3101         	addne	r1, #0x1
 8004952: fab1 f181    	clz	r1, r1
 8004956: f1c1 011f    	rsb.w	r1, r1, #0x1f
 800495a: b2cd         	uxtb	r5, r1
 800495c: 9509         	str	r5, [sp, #0x24]
 800495e: 2600         	movs	r6, #0x0
 8004960: 9909         	ldr	r1, [sp, #0x24]
 8004962: 9105         	str	r1, [sp, #0x14]
 8004964: 9902         	ldr	r1, [sp, #0x8]
 8004966: 9107         	str	r1, [sp, #0x1c]
 8004968: f8cd b020    	str.w	r11, [sp, #0x20]
 800496c: 9b07         	ldr	r3, [sp, #0x1c]
 800496e: 2b00         	cmp	r3, #0x0
 8004970: bf1c         	itt	ne
 8004972: 9908         	ldrne	r1, [sp, #0x20]
 8004974: 2900         	cmpne	r1, #0x0
 8004976: d00b         	beq	0x8004990 <raylar_crcl_encoder_service::benchmark::rice_k::h2637d4bce634509f+0x104> @ imm = #0x16
 8004978: fbb3 f6f1    	udiv	r6, r3, r1
 800497c: fb06 3111    	mls	r1, r6, r1, r3
 8004980: 2900         	cmp	r1, #0x0
 8004982: bf18         	it	ne
 8004984: 3601         	addne	r6, #0x1
 8004986: fab6 f186    	clz	r1, r6
 800498a: f1c1 011f    	rsb.w	r1, r1, #0x1f
 800498e: b2ce         	uxtb	r6, r1
 8004990: 9609         	str	r6, [sp, #0x24]
 8004992: 4625         	mov	r5, r4
 8004994: 2600         	movs	r6, #0x0
 8004996: 9901         	ldr	r1, [sp, #0x4]
 8004998: 9107         	str	r1, [sp, #0x1c]
 800499a: f8dd c024    	ldr.w	r12, [sp, #0x24]
 800499e: 9b07         	ldr	r3, [sp, #0x1c]
 80049a0: f8cd b020    	str.w	r11, [sp, #0x20]
 80049a4: 2b00         	cmp	r3, #0x0
 80049a6: bf1c         	itt	ne
 80049a8: 9908         	ldrne	r1, [sp, #0x20]
 80049aa: 2900         	cmpne	r1, #0x0
 80049ac: d00b         	beq	0x80049c6 <raylar_crcl_encoder_service::benchmark::rice_k::h2637d4bce634509f+0x13a> @ imm = #0x16
 80049ae: fbb3 f6f1    	udiv	r6, r3, r1
 80049b2: fb06 3111    	mls	r1, r6, r1, r3
 80049b6: 2900         	cmp	r1, #0x0
 80049b8: bf18         	it	ne
 80049ba: 3601         	addne	r6, #0x1
 80049bc: fab6 f186    	clz	r1, r6
 80049c0: f1c1 011f    	rsb.w	r1, r1, #0x1f
 80049c4: b2ce         	uxtb	r6, r1
 80049c6: 9609         	str	r6, [sp, #0x24]
 80049c8: 46c2         	mov	r10, r8
 80049ca: 465a         	mov	r2, r11
 80049cc: 9900         	ldr	r1, [sp]
 80049ce: 9107         	str	r1, [sp, #0x1c]
 80049d0: a908         	add	r1, sp, #0x20
 80049d2: f8dd e024    	ldr.w	lr, [sp, #0x24]
 80049d6: f04f 0800    	mov.w	r8, #0x0
 80049da: 9b07         	ldr	r3, [sp, #0x1c]
 80049dc: f8cd b020    	str.w	r11, [sp, #0x20]
 80049e0: 2b00         	cmp	r3, #0x0
 80049e2: bf1c         	itt	ne
 80049e4: 9908         	ldrne	r1, [sp, #0x20]
 80049e6: 2900         	cmpne	r1, #0x0
 80049e8: f43f af6f    	beq.w	0x80048ca <raylar_crcl_encoder_service::benchmark::rice_k::h2637d4bce634509f+0x3e> @ imm = #-0x122
 80049ec: fbb3 f6f1    	udiv	r6, r3, r1
 80049f0: fb06 3111    	mls	r1, r6, r1, r3
 80049f4: 2900         	cmp	r1, #0x0
 80049f6: bf18         	it	ne
 80049f8: 3601         	addne	r6, #0x1
 80049fa: fab6 f186    	clz	r1, r6
 80049fe: f1c1 011f    	rsb.w	r1, r1, #0x1f
 8004a02: fa5f f881    	uxtb.w	r8, r1
 8004a06: e760         	b	0x80048ca <raylar_crcl_encoder_service::benchmark::rice_k::h2637d4bce634509f+0x3e> @ imm = #-0x140
 8004a08: b00a         	add	sp, #0x28
 8004a0a: e8bd 0f00    	pop.w	{r8, r9, r10, r11}
 8004a0e: bdf0         	pop	{r4, r5, r6, r7, pc}

08004a10 <raylar_crcl_encoder_service::benchmark::dcblock::h5d1170af065dd1eb>:
 8004a10: b5f0         	push	{r4, r5, r6, r7, lr}
 8004a12: af03         	add	r7, sp, #0xc
 8004a14: e92d 0f00    	push.w	{r8, r9, r10, r11}
 8004a18: b083         	sub	sp, #0xc
 8004a1a: f645 716c    	movw	r1, #0x5f6c
 8004a1e: 2600         	movs	r6, #0x0
 8004a20: 4401         	add	r1, r0
 8004a22: f44f 6280    	mov.w	r2, #0x400
 8004a26: 2300         	movs	r3, #0x0
 8004a28: 9001         	str	r0, [sp, #0x4]
 8004a2a: eb03 2326    	add.w	r3, r3, r6, asr #8
 8004a2e: f8d1 00c0    	ldr.w	r0, [r1, #0xc0]
 8004a32: 1af3         	subs	r3, r6, r3
 8004a34: f8d1 50d8    	ldr.w	r5, [r1, #0xd8]
 8004a38: 4418         	add	r0, r3
 8004a3a: f8c1 00c0    	str.w	r0, [r1, #0xc0]
 8004a3e: f8d1 40f0    	ldr.w	r4, [r1, #0xf0]
 8004a42: 3a10         	subs	r2, #0x10
 8004a44: eba3 2020    	sub.w	r0, r3, r0, asr #8
 8004a48: f8d1 8108    	ldr.w	r8, [r1, #0x108]
 8004a4c: eb00 0305    	add.w	r3, r0, r5
 8004a50: f8c1 30d8    	str.w	r3, [r1, #0xd8]
 8004a54: f8d1 6120    	ldr.w	r6, [r1, #0x120]
 8004a58: eba0 2023    	sub.w	r0, r0, r3, asr #8
 8004a5c: f8d1 5168    	ldr.w	r5, [r1, #0x168]
 8004a60: eb00 0304    	add.w	r3, r0, r4
 8004a64: f8c1 30f0    	str.w	r3, [r1, #0xf0]
 8004a68: f8d1 4150    	ldr.w	r4, [r1, #0x150]
 8004a6c: eba0 2023    	sub.w	r0, r0, r3, asr #8
 8004a70: f8d1 e180    	ldr.w	lr, [r1, #0x180]
 8004a74: eb00 0308    	add.w	r3, r0, r8
 8004a78: f8c1 3108    	str.w	r3, [r1, #0x108]
 8004a7c: f8d1 c198    	ldr.w	r12, [r1, #0x198]
 8004a80: eba0 2023    	sub.w	r0, r0, r3, asr #8
 8004a84: eb00 0306    	add.w	r3, r0, r6
 8004a88: f8d1 6138    	ldr.w	r6, [r1, #0x138]
 8004a8c: f8c1 3120    	str.w	r3, [r1, #0x120]
 8004a90: eba0 2023    	sub.w	r0, r0, r3, asr #8
 8004a94: eb00 0306    	add.w	r3, r0, r6
 8004a98: f8c1 3138    	str.w	r3, [r1, #0x138]
 8004a9c: f8d1 61b0    	ldr.w	r6, [r1, #0x1b0]
 8004aa0: eba0 2023    	sub.w	r0, r0, r3, asr #8
 8004aa4: eb00 0304    	add.w	r3, r0, r4
 8004aa8: f8c1 3150    	str.w	r3, [r1, #0x150]
 8004aac: eba0 2023    	sub.w	r0, r0, r3, asr #8
 8004ab0: eb00 0305    	add.w	r3, r0, r5
 8004ab4: f8c1 3168    	str.w	r3, [r1, #0x168]
 8004ab8: eba0 2023    	sub.w	r0, r0, r3, asr #8
 8004abc: eb00 030e    	add.w	r3, r0, lr
 8004ac0: f8c1 3180    	str.w	r3, [r1, #0x180]
 8004ac4: eba0 2023    	sub.w	r0, r0, r3, asr #8
 8004ac8: eb00 030c    	add.w	r3, r0, r12
 8004acc: f8c1 3198    	str.w	r3, [r1, #0x198]
 8004ad0: eba0 2023    	sub.w	r0, r0, r3, asr #8
 8004ad4: eb00 0306    	add.w	r3, r0, r6
 8004ad8: f8d1 61c8    	ldr.w	r6, [r1, #0x1c8]
 8004adc: f8c1 31b0    	str.w	r3, [r1, #0x1b0]
 8004ae0: eba0 2023    	sub.w	r0, r0, r3, asr #8
 8004ae4: eb00 0306    	add.w	r3, r0, r6
 8004ae8: f8d1 61e0    	ldr.w	r6, [r1, #0x1e0]
 8004aec: f8c1 31c8    	str.w	r3, [r1, #0x1c8]
 8004af0: eba0 2023    	sub.w	r0, r0, r3, asr #8
 8004af4: eb00 0306    	add.w	r3, r0, r6
 8004af8: f8d1 61f8    	ldr.w	r6, [r1, #0x1f8]
 8004afc: f8c1 31e0    	str.w	r3, [r1, #0x1e0]
 8004b00: eba0 2023    	sub.w	r0, r0, r3, asr #8
 8004b04: eb00 0306    	add.w	r3, r0, r6
 8004b08: f8d1 6210    	ldr.w	r6, [r1, #0x210]
 8004b0c: f8c1 31f8    	str.w	r3, [r1, #0x1f8]
 8004b10: eba0 2023    	sub.w	r0, r0, r3, asr #8
 8004b14: f8d1 3228    	ldr.w	r3, [r1, #0x228]
 8004b18: 4406         	add	r6, r0
 8004b1a: f8c1 6210    	str.w	r6, [r1, #0x210]
 8004b1e: eba0 2026    	sub.w	r0, r0, r6, asr #8
 8004b22: eb00 0603    	add.w	r6, r0, r3
 8004b26: f8c1 6228    	str.w	r6, [r1, #0x228]
 8004b2a: f501 71c0    	add.w	r1, r1, #0x180
 8004b2e: f47f af7c    	bne.w	0x8004a2a <raylar_crcl_encoder_service::benchmark::dcblock::h5d1170af065dd1eb+0x1a> @ imm = #-0x108
 8004b32: 9801         	ldr	r0, [sp, #0x4]
 8004b34: f645 61b0    	movw	r1, #0x5eb0
 8004b38: f04f 0e00    	mov.w	lr, #0x0
 8004b3c: f44f 6280    	mov.w	r2, #0x400
 8004b40: 4401         	add	r1, r0
 8004b42: 2300         	movs	r3, #0x0
 8004b44: 9600         	str	r6, [sp]
 8004b46: eb03 232e    	add.w	r3, r3, lr, asr #8
 8004b4a: f8d1 4180    	ldr.w	r4, [r1, #0x180]
 8004b4e: ebae 0303    	sub.w	r3, lr, r3
 8004b52: f8d1 5198    	ldr.w	r5, [r1, #0x198]
 8004b56: 441c         	add	r4, r3
 8004b58: f8c1 4180    	str.w	r4, [r1, #0x180]
 8004b5c: f8d1 61b0    	ldr.w	r6, [r1, #0x1b0]
 8004b60: 3a10         	subs	r2, #0x10
 8004b62: eba3 2324    	sub.w	r3, r3, r4, asr #8
 8004b66: f8d1 81c8    	ldr.w	r8, [r1, #0x1c8]
 8004b6a: eb03 0405    	add.w	r4, r3, r5
 8004b6e: f8c1 4198    	str.w	r4, [r1, #0x198]
 8004b72: f8d1 51e0    	ldr.w	r5, [r1, #0x1e0]
 8004b76: eba3 2324    	sub.w	r3, r3, r4, asr #8
 8004b7a: f8d1 a228    	ldr.w	r10, [r1, #0x228]
 8004b7e: eb03 0406    	add.w	r4, r3, r6
 8004b82: f8c1 41b0    	str.w	r4, [r1, #0x1b0]
 8004b86: f8d1 6210    	ldr.w	r6, [r1, #0x210]
 8004b8a: eba3 2324    	sub.w	r3, r3, r4, asr #8
 8004b8e: f8d1 e240    	ldr.w	lr, [r1, #0x240]
 8004b92: eb03 0408    	add.w	r4, r3, r8
 8004b96: f8c1 41c8    	str.w	r4, [r1, #0x1c8]
 8004b9a: f8d1 9258    	ldr.w	r9, [r1, #0x258]
 8004b9e: eba3 2324    	sub.w	r3, r3, r4, asr #8
 8004ba2: eb03 0405    	add.w	r4, r3, r5
 8004ba6: f8d1 51f8    	ldr.w	r5, [r1, #0x1f8]
 8004baa: f8c1 41e0    	str.w	r4, [r1, #0x1e0]
 8004bae: eba3 2324    	sub.w	r3, r3, r4, asr #8
 8004bb2: eb03 0405    	add.w	r4, r3, r5
 8004bb6: f8c1 41f8    	str.w	r4, [r1, #0x1f8]
 8004bba: f8d1 5270    	ldr.w	r5, [r1, #0x270]
 8004bbe: eba3 2324    	sub.w	r3, r3, r4, asr #8
 8004bc2: eb03 0406    	add.w	r4, r3, r6
 8004bc6: f8c1 4210    	str.w	r4, [r1, #0x210]
 8004bca: eba3 2324    	sub.w	r3, r3, r4, asr #8
 8004bce: eb03 040a    	add.w	r4, r3, r10
 8004bd2: f8c1 4228    	str.w	r4, [r1, #0x228]
 8004bd6: eba3 2324    	sub.w	r3, r3, r4, asr #8
 8004bda: eb03 060e    	add.w	r6, r3, lr
 8004bde: f8c1 6240    	str.w	r6, [r1, #0x240]
 8004be2: eba3 2326    	sub.w	r3, r3, r6, asr #8
 8004be6: eb03 0609    	add.w	r6, r3, r9
 8004bea: f8c1 6258    	str.w	r6, [r1, #0x258]
 8004bee: eba3 2326    	sub.w	r3, r3, r6, asr #8
 8004bf2: eb03 0605    	add.w	r6, r3, r5
 8004bf6: f8d1 5288    	ldr.w	r5, [r1, #0x288]
 8004bfa: f8c1 6270    	str.w	r6, [r1, #0x270]
 8004bfe: eba3 2326    	sub.w	r3, r3, r6, asr #8
 8004c02: eb03 0605    	add.w	r6, r3, r5
 8004c06: f8d1 52a0    	ldr.w	r5, [r1, #0x2a0]
 8004c0a: f8c1 6288    	str.w	r6, [r1, #0x288]
 8004c0e: eba3 2326    	sub.w	r3, r3, r6, asr #8
 8004c12: eb03 0605    	add.w	r6, r3, r5
 8004c16: f8d1 52b8    	ldr.w	r5, [r1, #0x2b8]
 8004c1a: f8c1 62a0    	str.w	r6, [r1, #0x2a0]
 8004c1e: eba3 2326    	sub.w	r3, r3, r6, asr #8
 8004c22: eb03 0605    	add.w	r6, r3, r5
 8004c26: f8d1 52d0    	ldr.w	r5, [r1, #0x2d0]
 8004c2a: f8c1 62b8    	str.w	r6, [r1, #0x2b8]
 8004c2e: eba3 2626    	sub.w	r6, r3, r6, asr #8
 8004c32: f8d1 32e8    	ldr.w	r3, [r1, #0x2e8]
 8004c36: 4435         	add	r5, r6
 8004c38: f8c1 52d0    	str.w	r5, [r1, #0x2d0]
 8004c3c: eba6 2625    	sub.w	r6, r6, r5, asr #8
 8004c40: eb06 0e03    	add.w	lr, r6, r3
 8004c44: f8c1 e2e8    	str.w	lr, [r1, #0x2e8]
 8004c48: f501 71c0    	add.w	r1, r1, #0x180
 8004c4c: f47f af7b    	bne.w	0x8004b46 <raylar_crcl_encoder_service::benchmark::dcblock::h5d1170af065dd1eb+0x136> @ imm = #-0x10a
 8004c50: f645 7174    	movw	r1, #0x5f74
 8004c54: f04f 0800    	mov.w	r8, #0x0
 8004c58: 4401         	add	r1, r0
 8004c5a: f44f 6280    	mov.w	r2, #0x400
 8004c5e: 2400         	movs	r4, #0x0
 8004c60: eb04 2428    	add.w	r4, r4, r8, asr #8
 8004c64: f8d1 50c0    	ldr.w	r5, [r1, #0xc0]
 8004c68: eba8 0404    	sub.w	r4, r8, r4
 8004c6c: f8d1 60d8    	ldr.w	r6, [r1, #0xd8]
 8004c70: 4425         	add	r5, r4
 8004c72: f8c1 50c0    	str.w	r5, [r1, #0xc0]
 8004c76: f8d1 30f0    	ldr.w	r3, [r1, #0xf0]
 8004c7a: 3a10         	subs	r2, #0x10
 8004c7c: eba4 2425    	sub.w	r4, r4, r5, asr #8
 8004c80: f8d1 9108    	ldr.w	r9, [r1, #0x108]
 8004c84: eb04 0506    	add.w	r5, r4, r6
 8004c88: f8d1 6120    	ldr.w	r6, [r1, #0x120]
 8004c8c: f8c1 50d8    	str.w	r5, [r1, #0xd8]
 8004c90: eba4 2425    	sub.w	r4, r4, r5, asr #8
 8004c94: f8d1 5150    	ldr.w	r5, [r1, #0x150]
 8004c98: 4423         	add	r3, r4
 8004c9a: f8c1 30f0    	str.w	r3, [r1, #0xf0]
 8004c9e: f8d1 b168    	ldr.w	r11, [r1, #0x168]
 8004ca2: eba4 2323    	sub.w	r3, r4, r3, asr #8
 8004ca6: f8d1 8180    	ldr.w	r8, [r1, #0x180]
 8004caa: eb03 0409    	add.w	r4, r3, r9
 8004cae: f8c1 4108    	str.w	r4, [r1, #0x108]
 8004cb2: f8d1 a198    	ldr.w	r10, [r1, #0x198]
 8004cb6: eba3 2324    	sub.w	r3, r3, r4, asr #8
 8004cba: eb03 0406    	add.w	r4, r3, r6
 8004cbe: f8d1 6138    	ldr.w	r6, [r1, #0x138]
 8004cc2: f8c1 4120    	str.w	r4, [r1, #0x120]
 8004cc6: eba3 2324    	sub.w	r3, r3, r4, asr #8
 8004cca: eb03 0406    	add.w	r4, r3, r6
 8004cce: f8c1 4138    	str.w	r4, [r1, #0x138]
 8004cd2: eba3 2324    	sub.w	r3, r3, r4, asr #8
 8004cd6: eb03 0405    	add.w	r4, r3, r5
 8004cda: f8c1 4150    	str.w	r4, [r1, #0x150]
 8004cde: f8d1 51b0    	ldr.w	r5, [r1, #0x1b0]
 8004ce2: eba3 2324    	sub.w	r3, r3, r4, asr #8
 8004ce6: eb03 040b    	add.w	r4, r3, r11
 8004cea: f8c1 4168    	str.w	r4, [r1, #0x168]
 8004cee: eba3 2324    	sub.w	r3, r3, r4, asr #8
 8004cf2: eb03 0408    	add.w	r4, r3, r8
 8004cf6: f8c1 4180    	str.w	r4, [r1, #0x180]
 8004cfa: eba3 2324    	sub.w	r3, r3, r4, asr #8
 8004cfe: eb03 040a    	add.w	r4, r3, r10
 8004d02: f8c1 4198    	str.w	r4, [r1, #0x198]
 8004d06: eba3 2324    	sub.w	r3, r3, r4, asr #8
 8004d0a: eb03 0405    	add.w	r4, r3, r5
 8004d0e: f8d1 51c8    	ldr.w	r5, [r1, #0x1c8]
 8004d12: f8c1 41b0    	str.w	r4, [r1, #0x1b0]
 8004d16: eba3 2324    	sub.w	r3, r3, r4, asr #8
 8004d1a: eb03 0405    	add.w	r4, r3, r5
 8004d1e: f8d1 51e0    	ldr.w	r5, [r1, #0x1e0]
 8004d22: f8c1 41c8    	str.w	r4, [r1, #0x1c8]
 8004d26: eba3 2324    	sub.w	r3, r3, r4, asr #8
 8004d2a: eb03 0405    	add.w	r4, r3, r5
 8004d2e: f8d1 51f8    	ldr.w	r5, [r1, #0x1f8]
 8004d32: f8c1 41e0    	str.w	r4, [r1, #0x1e0]
 8004d36: eba3 2324    	sub.w	r3, r3, r4, asr #8
 8004d3a: eb03 0405    	add.w	r4, r3, r5
 8004d3e: f8d1 5210    	ldr.w	r5, [r1, #0x210]
 8004d42: f8c1 41f8    	str.w	r4, [r1, #0x1f8]
 8004d46: eba3 2324    	sub.w	r3, r3, r4, asr #8
 8004d4a: f8d1 4228    	ldr.w	r4, [r1, #0x228]
 8004d4e: 441d         	add	r5, r3
 8004d50: f8c1 5210    	str.w	r5, [r1, #0x210]
 8004d54: eba3 2325    	sub.w	r3, r3, r5, asr #8
 8004d58: eb03 0804    	add.w	r8, r3, r4
 8004d5c: f8c1 8228    	str.w	r8, [r1, #0x228]
 8004d60: f501 71c0    	add.w	r1, r1, #0x180
 8004d64: f47f af7c    	bne.w	0x8004c60 <raylar_crcl_encoder_service::benchmark::dcblock::h5d1170af065dd1eb+0x250> @ imm = #-0x108
 8004d68: f645 61b8    	movw	r1, #0x5eb8
 8004d6c: 1842         	adds	r2, r0, r1
 8004d6e: f04f 0900    	mov.w	r9, #0x0
 8004d72: f44f 6c80    	mov.w	r12, #0x400
 8004d76: 2500         	movs	r5, #0x0
 8004d78: eb05 2529    	add.w	r5, r5, r9, asr #8
 8004d7c: f8d2 3180    	ldr.w	r3, [r2, #0x180]
 8004d80: eba9 0505    	sub.w	r5, r9, r5
 8004d84: f8d2 6198    	ldr.w	r6, [r2, #0x198]
 8004d88: 442b         	add	r3, r5
 8004d8a: f8c2 3180    	str.w	r3, [r2, #0x180]
 8004d8e: f8d2 11b0    	ldr.w	r1, [r2, #0x1b0]
 8004d92: f1bc 0c10    	subs.w	r12, r12, #0x10
 8004d96: eba5 2323    	sub.w	r3, r5, r3, asr #8
 8004d9a: f8d2 a1c8    	ldr.w	r10, [r2, #0x1c8]
 8004d9e: eb03 0506    	add.w	r5, r3, r6
 8004da2: f8d2 41e0    	ldr.w	r4, [r2, #0x1e0]
 8004da6: f8c2 5198    	str.w	r5, [r2, #0x198]
 8004daa: eba3 2325    	sub.w	r3, r3, r5, asr #8
 8004dae: f8d2 5210    	ldr.w	r5, [r2, #0x210]
 8004db2: 4419         	add	r1, r3
 8004db4: f8c2 11b0    	str.w	r1, [r2, #0x1b0]
 8004db8: f8d2 6228    	ldr.w	r6, [r2, #0x228]
 8004dbc: eba3 2121    	sub.w	r1, r3, r1, asr #8
 8004dc0: f8d2 9240    	ldr.w	r9, [r2, #0x240]
 8004dc4: eb01 030a    	add.w	r3, r1, r10
 8004dc8: f8c2 31c8    	str.w	r3, [r2, #0x1c8]
 8004dcc: f8d2 b258    	ldr.w	r11, [r2, #0x258]
 8004dd0: eba1 2123    	sub.w	r1, r1, r3, asr #8
 8004dd4: eb01 0304    	add.w	r3, r1, r4
 8004dd8: f8d2 41f8    	ldr.w	r4, [r2, #0x1f8]
 8004ddc: f8c2 31e0    	str.w	r3, [r2, #0x1e0]
 8004de0: eba1 2123    	sub.w	r1, r1, r3, asr #8
 8004de4: eb01 0304    	add.w	r3, r1, r4
 8004de8: f8c2 31f8    	str.w	r3, [r2, #0x1f8]
 8004dec: f8d2 4270    	ldr.w	r4, [r2, #0x270]
 8004df0: eba1 2123    	sub.w	r1, r1, r3, asr #8
 8004df4: eb01 0305    	add.w	r3, r1, r5
 8004df8: f8c2 3210    	str.w	r3, [r2, #0x210]
 8004dfc: f8d2 52e8    	ldr.w	r5, [r2, #0x2e8]
 8004e00: eba1 2123    	sub.w	r1, r1, r3, asr #8
 8004e04: eb01 0306    	add.w	r3, r1, r6
 8004e08: f8c2 3228    	str.w	r3, [r2, #0x228]
 8004e0c: eba1 2123    	sub.w	r1, r1, r3, asr #8
 8004e10: eb01 0309    	add.w	r3, r1, r9
 8004e14: f8c2 3240    	str.w	r3, [r2, #0x240]
 8004e18: eba1 2123    	sub.w	r1, r1, r3, asr #8
 8004e1c: eb01 030b    	add.w	r3, r1, r11
 8004e20: f8c2 3258    	str.w	r3, [r2, #0x258]
 8004e24: eba1 2123    	sub.w	r1, r1, r3, asr #8
 8004e28: eb01 0304    	add.w	r3, r1, r4
 8004e2c: f8d2 4288    	ldr.w	r4, [r2, #0x288]
 8004e30: f8c2 3270    	str.w	r3, [r2, #0x270]
 8004e34: eba1 2123    	sub.w	r1, r1, r3, asr #8
 8004e38: eb01 0304    	add.w	r3, r1, r4
 8004e3c: f8d2 42a0    	ldr.w	r4, [r2, #0x2a0]
 8004e40: f8c2 3288    	str.w	r3, [r2, #0x288]
 8004e44: eba1 2123    	sub.w	r1, r1, r3, asr #8
 8004e48: eb01 0304    	add.w	r3, r1, r4
 8004e4c: f8d2 42b8    	ldr.w	r4, [r2, #0x2b8]
 8004e50: f8c2 32a0    	str.w	r3, [r2, #0x2a0]
 8004e54: eba1 2123    	sub.w	r1, r1, r3, asr #8
 8004e58: eb01 0304    	add.w	r3, r1, r4
 8004e5c: f8d2 42d0    	ldr.w	r4, [r2, #0x2d0]
 8004e60: f8c2 32b8    	str.w	r3, [r2, #0x2b8]
 8004e64: eba1 2123    	sub.w	r1, r1, r3, asr #8
 8004e68: eb01 0304    	add.w	r3, r1, r4
 8004e6c: f8c2 32d0    	str.w	r3, [r2, #0x2d0]
 8004e70: eba1 2123    	sub.w	r1, r1, r3, asr #8
 8004e74: eb01 0905    	add.w	r9, r1, r5
 8004e78: f8c2 92e8    	str.w	r9, [r2, #0x2e8]
 8004e7c: f502 72c0    	add.w	r2, r2, #0x180
 8004e80: f47f af7a    	bne.w	0x8004d78 <raylar_crcl_encoder_service::benchmark::dcblock::h5d1170af065dd1eb+0x368> @ imm = #-0x10c
 8004e84: f645 717c    	movw	r1, #0x5f7c
 8004e88: 1842         	adds	r2, r0, r1
 8004e8a: 2500         	movs	r5, #0x0
 8004e8c: f44f 6480    	mov.w	r4, #0x400
 8004e90: 2600         	movs	r6, #0x0
 8004e92: eb06 2625    	add.w	r6, r6, r5, asr #8
 8004e96: f8d2 10c0    	ldr.w	r1, [r2, #0xc0]
 8004e9a: 1bae         	subs	r6, r5, r6
 8004e9c: f8d2 30d8    	ldr.w	r3, [r2, #0xd8]
 8004ea0: 4431         	add	r1, r6
 8004ea2: f8c2 10c0    	str.w	r1, [r2, #0xc0]
 8004ea6: f8d2 00f0    	ldr.w	r0, [r2, #0xf0]
 8004eaa: 3c10         	subs	r4, #0x10
 8004eac: eba6 2121    	sub.w	r1, r6, r1, asr #8
 8004eb0: f8d2 c108    	ldr.w	r12, [r2, #0x108]
 8004eb4: 440b         	add	r3, r1
 8004eb6: f8d2 5120    	ldr.w	r5, [r2, #0x120]
 8004eba: f8c2 30d8    	str.w	r3, [r2, #0xd8]
 8004ebe: eba1 2123    	sub.w	r1, r1, r3, asr #8
 8004ec2: f8d2 3150    	ldr.w	r3, [r2, #0x150]
 8004ec6: 4408         	add	r0, r1
 8004ec8: f8c2 00f0    	str.w	r0, [r2, #0xf0]
 8004ecc: f8d2 6168    	ldr.w	r6, [r2, #0x168]
 8004ed0: eba1 2020    	sub.w	r0, r1, r0, asr #8
 8004ed4: f8d2 b180    	ldr.w	r11, [r2, #0x180]
 8004ed8: eb00 010c    	add.w	r1, r0, r12
 8004edc: f8c2 1108    	str.w	r1, [r2, #0x108]
 8004ee0: f8d2 a198    	ldr.w	r10, [r2, #0x198]
 8004ee4: eba0 2021    	sub.w	r0, r0, r1, asr #8
 8004ee8: eb00 0105    	add.w	r1, r0, r5
 8004eec: f8d2 5138    	ldr.w	r5, [r2, #0x138]
 8004ef0: f8c2 1120    	str.w	r1, [r2, #0x120]
 8004ef4: eba0 2021    	sub.w	r0, r0, r1, asr #8
 8004ef8: eb00 0105    	add.w	r1, r0, r5
 8004efc: f8c2 1138    	str.w	r1, [r2, #0x138]
 8004f00: eba0 2021    	sub.w	r0, r0, r1, asr #8
 8004f04: eb00 0103    	add.w	r1, r0, r3
 8004f08: f8c2 1150    	str.w	r1, [r2, #0x150]
 8004f0c: f8d2 31b0    	ldr.w	r3, [r2, #0x1b0]
 8004f10: eba0 2021    	sub.w	r0, r0, r1, asr #8
 8004f14: eb00 0106    	add.w	r1, r0, r6
 8004f18: f8c2 1168    	str.w	r1, [r2, #0x168]
 8004f1c: f8d2 6228    	ldr.w	r6, [r2, #0x228]
 8004f20: eba0 2021    	sub.w	r0, r0, r1, asr #8
 8004f24: eb00 010b    	add.w	r1, r0, r11
 8004f28: f8c2 1180    	str.w	r1, [r2, #0x180]
 8004f2c: eba0 2021    	sub.w	r0, r0, r1, asr #8
 8004f30: eb00 010a    	add.w	r1, r0, r10
 8004f34: f8c2 1198    	str.w	r1, [r2, #0x198]
 8004f38: eba0 2021    	sub.w	r0, r0, r1, asr #8
 8004f3c: eb00 0103    	add.w	r1, r0, r3
 8004f40: f8d2 31c8    	ldr.w	r3, [r2, #0x1c8]
 8004f44: f8c2 11b0    	str.w	r1, [r2, #0x1b0]
 8004f48: eba0 2021    	sub.w	r0, r0, r1, asr #8
 8004f4c: eb00 0103    	add.w	r1, r0, r3
 8004f50: f8d2 31e0    	ldr.w	r3, [r2, #0x1e0]
 8004f54: f8c2 11c8    	str.w	r1, [r2, #0x1c8]
 8004f58: eba0 2021    	sub.w	r0, r0, r1, asr #8
 8004f5c: eb00 0103    	add.w	r1, r0, r3
 8004f60: f8d2 31f8    	ldr.w	r3, [r2, #0x1f8]
 8004f64: f8c2 11e0    	str.w	r1, [r2, #0x1e0]
 8004f68: eba0 2021    	sub.w	r0, r0, r1, asr #8
 8004f6c: eb00 0103    	add.w	r1, r0, r3
 8004f70: f8d2 3210    	ldr.w	r3, [r2, #0x210]
 8004f74: f8c2 11f8    	str.w	r1, [r2, #0x1f8]
 8004f78: eba0 2021    	sub.w	r0, r0, r1, asr #8
 8004f7c: eb00 0103    	add.w	r1, r0, r3
 8004f80: f8c2 1210    	str.w	r1, [r2, #0x210]
 8004f84: eba0 2021    	sub.w	r0, r0, r1, asr #8
 8004f88: eb00 0506    	add.w	r5, r0, r6
 8004f8c: f8c2 5228    	str.w	r5, [r2, #0x228]
 8004f90: f502 72c0    	add.w	r2, r2, #0x180
 8004f94: f47f af7d    	bne.w	0x8004e92 <raylar_crcl_encoder_service::benchmark::dcblock::h5d1170af065dd1eb+0x482> @ imm = #-0x106
 8004f98: 9901         	ldr	r1, [sp, #0x4]
 8004f9a: f246 0028    	movw	r0, #0x6028
 8004f9e: 4408         	add	r0, r1
 8004fa0: 9002         	str	r0, [sp, #0x8]
 8004fa2: a802         	add	r0, sp, #0x8
 8004fa4: 9800         	ldr	r0, [sp]
 8004fa6: ea80 000e    	eor.w	r0, r0, lr
 8004faa: ea80 0008    	eor.w	r0, r0, r8
 8004fae: ea80 0009    	eor.w	r0, r0, r9
 8004fb2: 4068         	eors	r0, r5
 8004fb4: b003         	add	sp, #0xc
 8004fb6: e8bd 0f00    	pop.w	{r8, r9, r10, r11}
 8004fba: bdf0         	pop	{r4, r5, r6, r7, pc}

08004fbc <raylar_crcl_encoder_service::benchmark::measure::he296905d366d7d56>:
 8004fbc: b5f0         	push	{r4, r5, r6, r7, lr}
 8004fbe: af03         	add	r7, sp, #0xc
 8004fc0: e92d 0f00    	push.w	{r8, r9, r10, r11}
 8004fc4: f5ad 7d05    	sub.w	sp, sp, #0x214
 8004fc8: 9001         	str	r0, [sp, #0x4]
 8004fca: a802         	add	r0, sp, #0x8
 8004fcc: 460d         	mov	r5, r1
 8004fce: f44f 7101    	mov.w	r1, #0x204
 8004fd2: 9300         	str	r3, [sp]
 8004fd4: 4616         	mov	r6, r2
 8004fd6: f007 fe84    	bl	0x800cce2 <__aeabi_memclr4> @ imm = #0x7d08
 8004fda: f50d 7a03    	add.w	r10, sp, #0x20c
 8004fde: f04f 0b00    	mov.w	r11, #0x0
 8004fe2: f04f 0800    	mov.w	r8, #0x0
 8004fe6: e004         	b	0x8004ff2 <raylar_crcl_encoder_service::benchmark::measure::he296905d366d7d56+0x36> @ imm = #0x8
 8004fe8: f108 0801    	add.w	r8, r8, #0x1
 8004fec: f1b8 0f89    	cmp.w	r8, #0x89
 8004ff0: d018         	beq	0x8005024 <raylar_crcl_encoder_service::benchmark::measure::he296905d366d7d56+0x68> @ imm = #0x30
 8004ff2: 6939         	ldr	r1, [r7, #0x10]
 8004ff4: 4628         	mov	r0, r5
 8004ff6: 4788         	blx	r1
 8004ff8: 47b0         	blx	r6
 8004ffa: 9583         	str	r5, [sp, #0x20c]
 8004ffc: 4604         	mov	r4, r0
 8004ffe: 6979         	ldr	r1, [r7, #0x14]
 8005000: 9883         	ldr	r0, [sp, #0x20c]
 8005002: 4788         	blx	r1
 8005004: 9083         	str	r0, [sp, #0x20c]
 8005006: f8dd 920c    	ldr.w	r9, [sp, #0x20c]
 800500a: 47b0         	blx	r6
 800500c: f1b8 0f07    	cmp.w	r8, #0x7
 8005010: d9ea         	bls	0x8004fe8 <raylar_crcl_encoder_service::benchmark::measure::he296905d366d7d56+0x2c> @ imm = #-0x2c
 8005012: a902         	add	r1, sp, #0x8
 8005014: 1b00         	subs	r0, r0, r4
 8005016: eb01 0188    	add.w	r1, r1, r8, lsl #2
 800501a: ea8b 0b09    	eor.w	r11, r11, r9
 800501e: f841 0c20    	str	r0, [r1, #-32]
 8005022: e7e1         	b	0x8004fe8 <raylar_crcl_encoder_service::benchmark::measure::he296905d366d7d56+0x2c> @ imm = #-0x3e
 8005024: a802         	add	r0, sp, #0x8
 8005026: f1a7 021d    	sub.w	r2, r7, #0x1d
 800502a: 2181         	movs	r1, #0x81
 800502c: f002 fe13    	bl	0x8007c56 <core::slice::sort::unstable::ipnsort::hd8a371dd4e708cab> @ imm = #0x2c26
 8005030: e9dd 5600    	ldrd	r5, r6, [sp]
 8005034: 9802         	ldr	r0, [sp, #0x8]
 8005036: 6035         	str	r5, [r6]
 8005038: 68bd         	ldr	r5, [r7, #0x8]
 800503a: 6075         	str	r5, [r6, #0x4]
 800503c: 68fd         	ldr	r5, [r7, #0xc]
 800503e: 9942         	ldr	r1, [sp, #0x108]
 8005040: e9c6 5002    	strd	r5, r0, [r6, #8]
 8005044: f106 0010    	add.w	r0, r6, #0x10
 8005048: 9a75         	ldr	r2, [sp, #0x1d4]
 800504a: 9b82         	ldr	r3, [sp, #0x208]
 800504c: e880 080e    	stm.w	r0, {r1, r2, r3, r11}
 8005050: f50d 7d05    	add.w	sp, sp, #0x214
 8005054: e8bd 0f00    	pop.w	{r8, r9, r10, r11}
 8005058: bdf0         	pop	{r4, r5, r6, r7, pc}

0800505a <raylar_crcl_encoder_service::benchmark::prepare::hb9734b8d5dc7f78e>:
 800505a: b5f0         	push	{r4, r5, r6, r7, lr}
 800505c: af03         	add	r7, sp, #0xc
 800505e: e92d 0f00    	push.w	{r8, r9, r10, r11}
 8005062: b0f9         	sub	sp, #0x1e4
 8005064: 4601         	mov	r1, r0
 8005066: f500 30ee    	add.w	r0, r0, #0x1dc00
 800506a: f500 701f    	add.w	r0, r0, #0x27c
 800506e: 9004         	str	r0, [sp, #0x10]
 8005070: f501 30f8    	add.w	r0, r1, #0x1f000
 8005074: f245 6578    	movw	r5, #0x5678
 8005078: f500 604b    	add.w	r0, r0, #0xcb0
 800507c: f24e 0838    	movw	r8, #0xe038
 8005080: 9005         	str	r0, [sp, #0x14]
 8005082: f101 0028    	add.w	r0, r1, #0x28
 8005086: 9003         	str	r0, [sp, #0xc]
 8005088: 2000         	movs	r0, #0x0
 800508a: f2c1 2534    	movt	r5, #0x1234
 800508e: f06f 0917    	mvn	r9, #0x17
 8005092: f6c0 0800    	movt	r8, #0x800
 8005096: f644 6a20    	movw	r10, #0x4e20
 800509a: f645 5bc0    	movw	r11, #0x5dc0
 800509e: f645 14d8    	movw	r4, #0x59d8
 80050a2: 9124         	str	r1, [sp, #0x90]
 80050a4: f000 031f    	and	r3, r0, #0x1f
 80050a8: f100 060a    	add.w	r6, r0, #0xa
 80050ac: f006 061f    	and	r6, r6, #0x1f
 80050b0: f100 0c02    	add.w	r12, r0, #0x2
 80050b4: f858 3023    	ldr.w	r3, [r8, r3, lsl #2]
 80050b8: ea85 3545    	eor.w	r5, r5, r5, lsl #13
 80050bc: f858 6026    	ldr.w	r6, [r8, r6, lsl #2]
 80050c0: f246 11a8    	movw	r1, #0x61a8
 80050c4: ea85 4555    	eor.w	r5, r5, r5, lsr #17
 80050c8: fb03 fe0a    	mul	lr, r3, r10
 80050cc: f00c 031f    	and	r3, r12, #0x1f
 80050d0: ea85 1545    	eor.w	r5, r5, r5, lsl #5
 80050d4: f858 2023    	ldr.w	r2, [r8, r3, lsl #2]
 80050d8: fb06 fc01    	mul	r12, r6, r1
 80050dc: f245 2108    	movw	r1, #0x5208
 80050e0: b2eb         	uxtb	r3, r5
 80050e2: 434a         	muls	r2, r1, r2
 80050e4: ea4f 76ee    	asr.w	r6, lr, #0x1f
 80050e8: eb0e 4656    	add.w	r6, lr, r6, lsr #17
 80050ec: 9924         	ldr	r1, [sp, #0x90]
 80050ee: eb03 33e6    	add.w	r3, r3, r6, asr #15
 80050f2: eb01 0609    	add.w	r6, r1, r9
 80050f6: 3b80         	subs	r3, #0x80
 80050f8: f245 51f0    	movw	r1, #0x55f0
 80050fc: f109 0918    	add.w	r9, r9, #0x18
 8005100: 6433         	str	r3, [r6, #0x40]
 8005102: 17d3         	asrs	r3, r2, #0x1f
 8005104: eb02 4253    	add.w	r2, r2, r3, lsr #17
 8005108: ea85 3345    	eor.w	r3, r5, r5, lsl #13
 800510c: ea83 4353    	eor.w	r3, r3, r3, lsr #17
 8005110: ea83 1343    	eor.w	r3, r3, r3, lsl #5
 8005114: b2dd         	uxtb	r5, r3
 8005116: eb05 32e2    	add.w	r2, r5, r2, asr #15
 800511a: 1d05         	adds	r5, r0, #0x4
 800511c: ea83 3343    	eor.w	r3, r3, r3, lsl #13
 8005120: f005 051f    	and	r5, r5, #0x1f
 8005124: 3a80         	subs	r2, #0x80
 8005126: ea83 4353    	eor.w	r3, r3, r3, lsr #17
 800512a: 6472         	str	r2, [r6, #0x44]
 800512c: f858 5025    	ldr.w	r5, [r8, r5, lsl #2]
 8005130: ea83 1343    	eor.w	r3, r3, r3, lsl #5
 8005134: 434d         	muls	r5, r1, r5
 8005136: 1d81         	adds	r1, r0, #0x6
 8005138: f001 011f    	and	r1, r1, #0x1f
 800513c: f858 1021    	ldr.w	r1, [r8, r1, lsl #2]
 8005140: 17ea         	asrs	r2, r5, #0x1f
 8005142: 4361         	muls	r1, r4, r1
 8005144: eb05 4252    	add.w	r2, r5, r2, lsr #17
 8005148: b2dd         	uxtb	r5, r3
 800514a: eb05 32e2    	add.w	r2, r5, r2, asr #15
 800514e: f100 0508    	add.w	r5, r0, #0x8
 8005152: f005 051f    	and	r5, r5, #0x1f
 8005156: 3a80         	subs	r2, #0x80
 8005158: 64b2         	str	r2, [r6, #0x48]
 800515a: 3001         	adds	r0, #0x1
 800515c: f858 5025    	ldr.w	r5, [r8, r5, lsl #2]
 8005160: f5b0 6f80    	cmp.w	r0, #0x400
 8005164: ea4f 72e1    	asr.w	r2, r1, #0x1f
 8005168: eb01 4152    	add.w	r1, r1, r2, lsr #17
 800516c: ea83 3243    	eor.w	r2, r3, r3, lsl #13
 8005170: ea82 4252    	eor.w	r2, r2, r2, lsr #17
 8005174: fb05 f50b    	mul	r5, r5, r11
 8005178: ea82 1242    	eor.w	r2, r2, r2, lsl #5
 800517c: b2d3         	uxtb	r3, r2
 800517e: ea82 3242    	eor.w	r2, r2, r2, lsl #13
 8005182: eb03 31e1    	add.w	r1, r3, r1, asr #15
 8005186: ea82 4252    	eor.w	r2, r2, r2, lsr #17
 800518a: f1a1 0180    	sub.w	r1, r1, #0x80
 800518e: 64f1         	str	r1, [r6, #0x4c]
 8005190: ea4f 71e5    	asr.w	r1, r5, #0x1f
 8005194: ea82 1242    	eor.w	r2, r2, r2, lsl #5
 8005198: eb05 4151    	add.w	r1, r5, r1, lsr #17
 800519c: b2d3         	uxtb	r3, r2
 800519e: ea82 3242    	eor.w	r2, r2, r2, lsl #13
 80051a2: eb03 31e1    	add.w	r1, r3, r1, asr #15
 80051a6: f1a1 0180    	sub.w	r1, r1, #0x80
 80051aa: ea82 4252    	eor.w	r2, r2, r2, lsr #17
 80051ae: 6531         	str	r1, [r6, #0x50]
 80051b0: ea4f 71ec    	asr.w	r1, r12, #0x1f
 80051b4: ea82 1542    	eor.w	r5, r2, r2, lsl #5
 80051b8: eb0c 4151    	add.w	r1, r12, r1, lsr #17
 80051bc: b2ea         	uxtb	r2, r5
 80051be: eb02 31e1    	add.w	r1, r2, r1, asr #15
 80051c2: f1a1 0180    	sub.w	r1, r1, #0x80
 80051c6: 6571         	str	r1, [r6, #0x54]
 80051c8: f47f af6c    	bne.w	0x80050a4 <raylar_crcl_encoder_service::benchmark::prepare::hb9734b8d5dc7f78e+0x4a> @ imm = #-0x128
 80051cc: 9924         	ldr	r1, [sp, #0x90]
 80051ce: f246 0028    	movw	r0, #0x6028
 80051d2: f44f 42c0    	mov.w	r2, #0x6000
 80051d6: eb01 0900    	add.w	r9, r1, r0
 80051da: 3128         	adds	r1, #0x28
 80051dc: 4648         	mov	r0, r9
 80051de: f007 fd32    	bl	0x800cc46 <__aeabi_memcpy4> @ imm = #0x7a64
 80051e2: ad25         	add	r5, sp, #0x94
 80051e4: 2128         	movs	r1, #0x28
 80051e6: f105 0848    	add.w	r8, r5, #0x48
 80051ea: 4640         	mov	r0, r8
 80051ec: f007 fd79    	bl	0x800cce2 <__aeabi_memclr4> @ imm = #0x7af2
 80051f0: 2101         	movs	r1, #0x1
 80051f2: 2000         	movs	r0, #0x0
 80051f4: 9141         	str	r1, [sp, #0x104]
 80051f6: f44f 51c0    	mov.w	r1, #0x1800
 80051fa: e9cd 9135    	strd	r9, r1, [sp, #212]
 80051fe: f44f 6180    	mov.w	r1, #0x400
 8005202: 9146         	str	r1, [sp, #0x118]
 8005204: f44f 0100    	mov.w	r1, #0x800000
 8005208: 9147         	str	r1, [sp, #0x11c]
 800520a: f240 610a    	movw	r1, #0x60a
 800520e: f2c0 1101    	movt	r1, #0x101
 8005212: e9cd 0044    	strd	r0, r0, [sp, #272]
 8005216: e9cd 0042    	strd	r0, r0, [sp, #264]
 800521a: e9cd 0025    	strd	r0, r0, [sp, #148]
 800521e: e9cd 0027    	strd	r0, r0, [sp, #156]
 8005222: e9cd 0029    	strd	r0, r0, [sp, #164]
 8005226: 902b         	str	r0, [sp, #0xac]
 8005228: f8ad 00b0    	strh.w	r0, [sp, #0xb0]
 800522c: e9cd 002d    	strd	r0, r0, [sp, #180]
 8005230: e9cd 002f    	strd	r0, r0, [sp, #188]
 8005234: e9cd 0031    	strd	r0, r0, [sp, #196]
 8005238: 9033         	str	r0, [sp, #0xcc]
 800523a: f8ad 00d0    	strh.w	r0, [sp, #0xd0]
 800523e: 9148         	str	r1, [sp, #0x120]
 8005240: f8ad 0124    	strh.w	r0, [sp, #0x124]
 8005244: 4628         	mov	r0, r5
 8005246: f04f 31ff    	mov.w	r1, #0xffffffff
 800524a: f7fc fc02    	bl	0x8001a52 <raylar_crcl_encoder_service::quantize::Transform::advance::h31016a74c93d75d3> @ imm = #-0x37fc
 800524e: 07c0         	lsls	r0, r0, #0x1f
 8005250: f041 804d    	bne.w	0x80062ee <raylar_crcl_encoder_service::benchmark::prepare::hb9734b8d5dc7f78e+0x1294> @ imm = #0x109a
 8005254: 07c8         	lsls	r0, r1, #0x1f
 8005256: d0f5         	beq	0x8005244 <raylar_crcl_encoder_service::benchmark::prepare::hb9734b8d5dc7f78e+0x1ea> @ imm = #-0x16
 8005258: 9c24         	ldr	r4, [sp, #0x90]
 800525a: e8b8 006e    	ldm.w	r8!, {r1, r2, r3, r5, r6}
 800525e: 4620         	mov	r0, r4
 8005260: c06e         	stm	r0!, {r1, r2, r3, r5, r6}
 8005262: e898 006e    	ldm.w	r8, {r1, r2, r3, r5, r6}
 8005266: c06e         	stm	r0!, {r1, r2, r3, r5, r6}
 8005268: f640 2100    	movw	r1, #0xa00
 800526c: f2c0 1106    	movt	r1, #0x106
 8005270: f8cd 1166    	str.w	r1, [sp, #0x166]
 8005274: f44f 0100    	mov.w	r1, #0x800000
 8005278: 2000         	movs	r0, #0x0
 800527a: f8cd 1163    	str.w	r1, [sp, #0x163]
 800527e: f44f 6180    	mov.w	r1, #0x400
 8005282: ad4c         	add	r5, sp, #0x130
 8005284: f8cd 115f    	str.w	r1, [sp, #0x15f]
 8005288: a954         	add	r1, sp, #0x150
 800528a: 9056         	str	r0, [sp, #0x158]
 800528c: 221a         	movs	r2, #0x1a
 800528e: f88d 0130    	strb.w	r0, [sp, #0x130]
 8005292: e9cd 0054    	strd	r0, r0, [sp, #336]
 8005296: f8cd 015b    	str.w	r0, [sp, #0x15b]
 800529a: 1c68         	adds	r0, r5, #0x1
 800529c: f007 faf8    	bl	0x800c890 <__aeabi_memcpy> @ imm = #0x75f0
 80052a0: ae54         	add	r6, sp, #0x150
 80052a2: 2001         	movs	r0, #0x1
 80052a4: a925         	add	r1, sp, #0x94
 80052a6: f88d 014b    	strb.w	r0, [sp, #0x14b]
 80052aa: 4630         	mov	r0, r6
 80052ac: 2294         	movs	r2, #0x94
 80052ae: f007 fcca    	bl	0x800cc46 <__aeabi_memcpy4> @ imm = #0x7994
 80052b2: f641 6058    	movw	r0, #0x1e58
 80052b6: 4629         	mov	r1, r5
 80052b8: 9000         	str	r0, [sp]
 80052ba: f504 30ec    	add.w	r0, r4, #0x1d800
 80052be: f500 63cb    	add.w	r3, r0, #0x658
 80052c2: a84a         	add	r0, sp, #0x128
 80052c4: 4632         	mov	r2, r6
 80052c6: f7fc f8dd    	bl	0x8001484 <raylar_crcl_encoder_service::format::Encoder::data::ha65fe528befbf4e3> @ imm = #-0x3e46
 80052ca: f89d 0128    	ldrb.w	r0, [sp, #0x128]
 80052ce: 2801         	cmp	r0, #0x1
 80052d0: f001 8034    	beq.w	0x800633c <raylar_crcl_encoder_service::benchmark::prepare::hb9734b8d5dc7f78e+0x12e2> @ imm = #0x1068
 80052d4: 9905         	ldr	r1, [sp, #0x14]
 80052d6: f44f 42c0    	mov.w	r2, #0x6000
 80052da: 984b         	ldr	r0, [sp, #0x12c]
 80052dc: 6288         	str	r0, [r1, #0x28]
 80052de: f24c 0028    	movw	r0, #0xc028
 80052e2: 9924         	ldr	r1, [sp, #0x90]
 80052e4: 4408         	add	r0, r1
 80052e6: 4649         	mov	r1, r9
 80052e8: 9002         	str	r0, [sp, #0x8]
 80052ea: f007 fcac    	bl	0x800cc46 <__aeabi_memcpy4> @ imm = #0x7958
 80052ee: f641 71fc    	movw	r1, #0x1ffc
 80052f2: f247 0910    	movw	r9, #0x7010
 80052f6: f2c0 0101    	movt	r1, #0x1
 80052fa: f101 0020    	add.w	r0, r1, #0x20
 80052fe: 9018         	str	r0, [sp, #0x60]
 8005300: f101 001c    	add.w	r0, r1, #0x1c
 8005304: 9020         	str	r0, [sp, #0x80]
 8005306: 2000         	movs	r0, #0x0
 8005308: 9019         	str	r0, [sp, #0x64]
 800530a: f64b 70fc    	movw	r0, #0xbffc
 800530e: 9017         	str	r0, [sp, #0x5c]
 8005310: f44f 3090    	mov.w	r0, #0x12000
 8005314: 9023         	str	r0, [sp, #0x8c]
 8005316: f64b 70e4    	movw	r0, #0xbfe4
 800531a: 9016         	str	r0, [sp, #0x58]
 800531c: f64b 706c    	movw	r0, #0xbf6c
 8005320: 9022         	str	r0, [sp, #0x88]
 8005322: f64b 70cc    	movw	r0, #0xbfcc
 8005326: 9021         	str	r0, [sp, #0x84]
 8005328: f64b 70b4    	movw	r0, #0xbfb4
 800532c: 901f         	str	r0, [sp, #0x7c]
 800532e: f64b 7084    	movw	r0, #0xbf84
 8005332: 901e         	str	r0, [sp, #0x78]
 8005334: f64b 709c    	movw	r0, #0xbf9c
 8005338: 901d         	str	r0, [sp, #0x74]
 800533a: f101 0014    	add.w	r0, r1, #0x14
 800533e: 901c         	str	r0, [sp, #0x70]
 8005340: f101 0010    	add.w	r0, r1, #0x10
 8005344: 901b         	str	r0, [sp, #0x6c]
 8005346: f101 000c    	add.w	r0, r1, #0xc
 800534a: 901a         	str	r0, [sp, #0x68]
 800534c: f24f 0004    	movw	r0, #0xf004
 8005350: f101 0c28    	add.w	r12, r1, #0x28
 8005354: f101 0624    	add.w	r6, r1, #0x24
 8005358: f101 0318    	add.w	r3, r1, #0x18
 800535c: f2c0 0901    	movt	r9, #0x1
 8005360: f64b 7224    	movw	r2, #0xbf24
 8005364: f64b 7a54    	movw	r10, #0xbf54
 8005368: f6cf 70ff    	movt	r0, #0xffff
 800536c: 9c24         	ldr	r4, [sp, #0x90]
 800536e: 4698         	mov	r8, r3
 8005370: e9cd 2912    	strd	r2, r9, [sp, #72]
 8005374: eb04 0e02    	add.w	lr, r4, r2
 8005378: e9cd 3114    	strd	r3, r1, [sp, #80]
 800537c: 1863         	adds	r3, r4, r1
 800537e: 44a1         	add	r9, r4
 8005380: f8de 2108    	ldr.w	r2, [lr, #0x108]
 8005384: f8de 5120    	ldr.w	r5, [lr, #0x120]
 8005388: e9cd 0a10    	strd	r0, r10, [sp, #64]
 800538c: 1aaa         	subs	r2, r5, r2
 800538e: 62da         	str	r2, [r3, #0x2c]
 8005390: f8dd b05c    	ldr.w	r11, [sp, #0x5c]
 8005394: 0053         	lsls	r3, r2, #0x1
 8005396: ea83 71e2    	eor.w	r1, r3, r2, asr #31
 800539a: eb04 030a    	add.w	r3, r4, r10
 800539e: f8c9 1004    	str.w	r1, [r9, #0x4]
 80053a2: 910f         	str	r1, [sp, #0x3c]
 80053a4: f8de 2120    	ldr.w	r2, [lr, #0x120]
 80053a8: f8d3 5108    	ldr.w	r5, [r3, #0x108]
 80053ac: 9923         	ldr	r1, [sp, #0x8c]
 80053ae: 1aaa         	subs	r2, r5, r2
 80053b0: f8dd a058    	ldr.w	r10, [sp, #0x58]
 80053b4: 1865         	adds	r5, r4, r1
 80053b6: 62ea         	str	r2, [r5, #0x2c]
 80053b8: 0055         	lsls	r5, r2, #0x1
 80053ba: ea85 71e2    	eor.w	r1, r5, r2, asr #31
 80053be: f8c9 1008    	str.w	r1, [r9, #0x8]
 80053c2: 910e         	str	r1, [sp, #0x38]
 80053c4: 9922         	ldr	r1, [sp, #0x88]
 80053c6: f8d3 2108    	ldr.w	r2, [r3, #0x108]
 80053ca: 1863         	adds	r3, r4, r1
 80053cc: f8d3 5108    	ldr.w	r5, [r3, #0x108]
 80053d0: 1aaa         	subs	r2, r5, r2
 80053d2: 1825         	adds	r5, r4, r0
 80053d4: f243 002c    	movw	r0, #0x302c
 80053d8: f2c0 0001    	movt	r0, #0x1
 80053dc: 502a         	str	r2, [r5, r0]
 80053de: 0055         	lsls	r5, r2, #0x1
 80053e0: ea85 70e2    	eor.w	r0, r5, r2, asr #31
 80053e4: f8c9 000c    	str.w	r0, [r9, #0xc]
 80053e8: 4665         	mov	r5, r12
 80053ea: 900d         	str	r0, [sp, #0x34]
 80053ec: 981e         	ldr	r0, [sp, #0x78]
 80053ee: f8d3 2108    	ldr.w	r2, [r3, #0x108]
 80053f2: 1823         	adds	r3, r4, r0
 80053f4: 981a         	ldr	r0, [sp, #0x68]
 80053f6: f8d3 1108    	ldr.w	r1, [r3, #0x108]
 80053fa: 1a89         	subs	r1, r1, r2
 80053fc: 1822         	adds	r2, r4, r0
 80053fe: 62d1         	str	r1, [r2, #0x2c]
 8005400: 004a         	lsls	r2, r1, #0x1
 8005402: ea82 70e1    	eor.w	r0, r2, r1, asr #31
 8005406: f8c9 0010    	str.w	r0, [r9, #0x10]
 800540a: 900c         	str	r0, [sp, #0x30]
 800540c: 981d         	ldr	r0, [sp, #0x74]
 800540e: f8d3 1108    	ldr.w	r1, [r3, #0x108]
 8005412: 1822         	adds	r2, r4, r0
 8005414: 981b         	ldr	r0, [sp, #0x6c]
 8005416: f8d2 3108    	ldr.w	r3, [r2, #0x108]
 800541a: 1a59         	subs	r1, r3, r1
 800541c: 1823         	adds	r3, r4, r0
 800541e: 62d9         	str	r1, [r3, #0x2c]
 8005420: 004b         	lsls	r3, r1, #0x1
 8005422: ea83 70e1    	eor.w	r0, r3, r1, asr #31
 8005426: f8c9 0014    	str.w	r0, [r9, #0x14]
 800542a: 900b         	str	r0, [sp, #0x2c]
 800542c: 981f         	ldr	r0, [sp, #0x7c]
 800542e: f8d2 1108    	ldr.w	r1, [r2, #0x108]
 8005432: 1822         	adds	r2, r4, r0
 8005434: 981c         	ldr	r0, [sp, #0x70]
 8005436: f8d2 3108    	ldr.w	r3, [r2, #0x108]
 800543a: 1a59         	subs	r1, r3, r1
 800543c: 1823         	adds	r3, r4, r0
 800543e: 62d9         	str	r1, [r3, #0x2c]
 8005440: 004b         	lsls	r3, r1, #0x1
 8005442: ea83 70e1    	eor.w	r0, r3, r1, asr #31
 8005446: f8c9 0018    	str.w	r0, [r9, #0x18]
 800544a: 900a         	str	r0, [sp, #0x28]
 800544c: 9821         	ldr	r0, [sp, #0x84]
 800544e: f8d2 1108    	ldr.w	r1, [r2, #0x108]
 8005452: 1822         	adds	r2, r4, r0
 8005454: f8d2 3108    	ldr.w	r3, [r2, #0x108]
 8005458: 1a59         	subs	r1, r3, r1
 800545a: eb04 0308    	add.w	r3, r4, r8
 800545e: f8dd 8060    	ldr.w	r8, [sp, #0x60]
 8005462: 62d9         	str	r1, [r3, #0x2c]
 8005464: 004b         	lsls	r3, r1, #0x1
 8005466: ea83 70e1    	eor.w	r0, r3, r1, asr #31
 800546a: eb04 030a    	add.w	r3, r4, r10
 800546e: f8c9 001c    	str.w	r0, [r9, #0x1c]
 8005472: f50a 7a84    	add.w	r10, r10, #0x108
 8005476: 9009         	str	r0, [sp, #0x24]
 8005478: f8d2 1108    	ldr.w	r1, [r2, #0x108]
 800547c: f8d3 2108    	ldr.w	r2, [r3, #0x108]
 8005480: 9820         	ldr	r0, [sp, #0x80]
 8005482: 1a51         	subs	r1, r2, r1
 8005484: f8cd a058    	str.w	r10, [sp, #0x58]
 8005488: 1822         	adds	r2, r4, r0
 800548a: f8dd a044    	ldr.w	r10, [sp, #0x44]
 800548e: 62d1         	str	r1, [r2, #0x2c]
 8005490: 004a         	lsls	r2, r1, #0x1
 8005492: ea82 72e1    	eor.w	r2, r2, r1, asr #31
 8005496: f8c9 2020    	str.w	r2, [r9, #0x20]
 800549a: f8d3 1108    	ldr.w	r1, [r3, #0x108]
 800549e: eb04 030b    	add.w	r3, r4, r11
 80054a2: f50a 7a84    	add.w	r10, r10, #0x108
 80054a6: f50b 7b84    	add.w	r11, r11, #0x108
 80054aa: f8d3 c108    	ldr.w	r12, [r3, #0x108]
 80054ae: f8cd b05c    	str.w	r11, [sp, #0x5c]
 80054b2: ebac 0101    	sub.w	r1, r12, r1
 80054b6: 46ac         	mov	r12, r5
 80054b8: eb04 0508    	add.w	r5, r4, r8
 80054bc: f108 082c    	add.w	r8, r8, #0x2c
 80054c0: f8cd 8060    	str.w	r8, [sp, #0x60]
 80054c4: 62e9         	str	r1, [r5, #0x2c]
 80054c6: 004d         	lsls	r5, r1, #0x1
 80054c8: ea85 71e1    	eor.w	r1, r5, r1, asr #31
 80054cc: f8c9 1024    	str.w	r1, [r9, #0x24]
 80054d0: f8d3 3108    	ldr.w	r3, [r3, #0x108]
 80054d4: f8de 51f8    	ldr.w	r5, [lr, #0x1f8]
 80054d8: 1aeb         	subs	r3, r5, r3
 80054da: 19a5         	adds	r5, r4, r6
 80054dc: 62eb         	str	r3, [r5, #0x2c]
 80054de: 005d         	lsls	r5, r3, #0x1
 80054e0: ea85 73e3    	eor.w	r3, r5, r3, asr #31
 80054e4: f8c9 3028    	str.w	r3, [r9, #0x28]
 80054e8: eb04 050c    	add.w	r5, r4, r12
 80054ec: 4634         	mov	r4, r6
 80054ee: f8de 61f8    	ldr.w	r6, [lr, #0x1f8]
 80054f2: f10c 0c2c    	add.w	r12, r12, #0x2c
 80054f6: f8de 0210    	ldr.w	r0, [lr, #0x210]
 80054fa: 1b80         	subs	r0, r0, r6
 80054fc: 62e8         	str	r0, [r5, #0x2c]
 80054fe: 9d19         	ldr	r5, [sp, #0x64]
 8005500: 0046         	lsls	r6, r0, #0x1
 8005502: ea86 70e0    	eor.w	r0, r6, r0, asr #31
 8005506: 9e0f         	ldr	r6, [sp, #0x3c]
 8005508: f8c9 002c    	str.w	r0, [r9, #0x2c]
 800550c: 442e         	add	r6, r5
 800550e: 9d0e         	ldr	r5, [sp, #0x38]
 8005510: f8dd 904c    	ldr.w	r9, [sp, #0x4c]
 8005514: 442e         	add	r6, r5
 8005516: 9d0d         	ldr	r5, [sp, #0x34]
 8005518: f109 092c    	add.w	r9, r9, #0x2c
 800551c: 442e         	add	r6, r5
 800551e: 9d0c         	ldr	r5, [sp, #0x30]
 8005520: 442e         	add	r6, r5
 8005522: 9d0b         	ldr	r5, [sp, #0x2c]
 8005524: 442e         	add	r6, r5
 8005526: 9d0a         	ldr	r5, [sp, #0x28]
 8005528: 442e         	add	r6, r5
 800552a: 9d09         	ldr	r5, [sp, #0x24]
 800552c: 442e         	add	r6, r5
 800552e: 9d14         	ldr	r5, [sp, #0x50]
 8005530: 4432         	add	r2, r6
 8005532: f104 062c    	add.w	r6, r4, #0x2c
 8005536: 4411         	add	r1, r2
 8005538: 9a12         	ldr	r2, [sp, #0x48]
 800553a: 4419         	add	r1, r3
 800553c: f105 032c    	add.w	r3, r5, #0x2c
 8005540: 4408         	add	r0, r1
 8005542: 9019         	str	r0, [sp, #0x64]
 8005544: 981a         	ldr	r0, [sp, #0x68]
 8005546: f502 7284    	add.w	r2, r2, #0x108
 800554a: 9915         	ldr	r1, [sp, #0x54]
 800554c: 302c         	adds	r0, #0x2c
 800554e: 901a         	str	r0, [sp, #0x68]
 8005550: 981d         	ldr	r0, [sp, #0x74]
 8005552: 312c         	adds	r1, #0x2c
 8005554: 9d23         	ldr	r5, [sp, #0x8c]
 8005556: f500 7084    	add.w	r0, r0, #0x108
 800555a: 901d         	str	r0, [sp, #0x74]
 800555c: 981b         	ldr	r0, [sp, #0x6c]
 800555e: 352c         	adds	r5, #0x2c
 8005560: 9523         	str	r5, [sp, #0x8c]
 8005562: 302c         	adds	r0, #0x2c
 8005564: 901b         	str	r0, [sp, #0x6c]
 8005566: 981e         	ldr	r0, [sp, #0x78]
 8005568: f500 7084    	add.w	r0, r0, #0x108
 800556c: 901e         	str	r0, [sp, #0x78]
 800556e: 981f         	ldr	r0, [sp, #0x7c]
 8005570: f500 7084    	add.w	r0, r0, #0x108
 8005574: 901f         	str	r0, [sp, #0x7c]
 8005576: 981c         	ldr	r0, [sp, #0x70]
 8005578: 302c         	adds	r0, #0x2c
 800557a: 901c         	str	r0, [sp, #0x70]
 800557c: 9821         	ldr	r0, [sp, #0x84]
 800557e: f500 7084    	add.w	r0, r0, #0x108
 8005582: 9021         	str	r0, [sp, #0x84]
 8005584: 9822         	ldr	r0, [sp, #0x88]
 8005586: f500 7084    	add.w	r0, r0, #0x108
 800558a: 9022         	str	r0, [sp, #0x88]
 800558c: 9820         	ldr	r0, [sp, #0x80]
 800558e: 302c         	adds	r0, #0x2c
 8005590: 9020         	str	r0, [sp, #0x80]
 8005592: 9810         	ldr	r0, [sp, #0x40]
 8005594: 302c         	adds	r0, #0x2c
 8005596: f47f aee9    	bne.w	0x800536c <raylar_crcl_encoder_service::benchmark::prepare::hb9734b8d5dc7f78e+0x312> @ imm = #-0x22e
 800559a: 9805         	ldr	r0, [sp, #0x14]
 800559c: 9919         	ldr	r1, [sp, #0x64]
 800559e: 6001         	str	r1, [r0]
 80055a0: f241 0005    	movw	r0, #0x1005
 80055a4: f2c0 0040    	movt	r0, #0x40
 80055a8: b191         	cbz	r1, 0x80055d0 <raylar_crcl_encoder_service::benchmark::prepare::hb9734b8d5dc7f78e+0x576> @ imm = #0x24
 80055aa: 9e19         	ldr	r6, [sp, #0x64]
 80055ac: fba6 0100    	umull	r0, r1, r6, r0
 80055b0: 1a70         	subs	r0, r6, r1
 80055b2: eb01 0050    	add.w	r0, r1, r0, lsr #1
 80055b6: 2101         	movs	r1, #0x1
 80055b8: 0a42         	lsrs	r2, r0, #0x9
 80055ba: eba2 2382    	sub.w	r3, r2, r2, lsl #10
 80055be: 199b         	adds	r3, r3, r6
 80055c0: bf18         	it	ne
 80055c2: eb01 2250    	addne.w	r2, r1, r0, lsr #9
 80055c6: fab2 f082    	clz	r0, r2
 80055ca: f1c0 001f    	rsb.w	r0, r0, #0x1f
 80055ce: e000         	b	0x80055d2 <raylar_crcl_encoder_service::benchmark::prepare::hb9734b8d5dc7f78e+0x578> @ imm = #0x0
 80055d0: 2000         	movs	r0, #0x0
 80055d2: 9924         	ldr	r1, [sp, #0x90]
 80055d4: f24f 0204    	movw	r2, #0xf004
 80055d8: f6cf 72ff    	movt	r2, #0xffff
 80055dc: f88d 0150    	strb.w	r0, [sp, #0x150]
 80055e0: b2c0         	uxtb	r0, r0
 80055e2: 7909         	ldrb	r1, [r1, #0x4]
 80055e4: 4288         	cmp	r0, r1
 80055e6: f040 86be    	bne.w	0x8006366 <raylar_crcl_encoder_service::benchmark::prepare::hb9734b8d5dc7f78e+0x130c> @ imm = #0xd7c
 80055ea: 9804         	ldr	r0, [sp, #0x10]
 80055ec: f248 040c    	movw	r4, #0x800c
 80055f0: 9905         	ldr	r1, [sp, #0x14]
 80055f2: f04f 0800    	mov.w	r8, #0x0
 80055f6: f2c0 0401    	movt	r4, #0x1
 80055fa: f64b 7e58    	movw	lr, #0xbf58
 80055fe: 6800         	ldr	r0, [r0]
 8005600: 4694         	mov	r12, r2
 8005602: 6148         	str	r0, [r1, #0x14]
 8005604: f642 70f8    	movw	r0, #0x2ff8
 8005608: f2c0 0001    	movt	r0, #0x1
 800560c: f100 0128    	add.w	r1, r0, #0x28
 8005610: 9119         	str	r1, [sp, #0x64]
 8005612: f44f 4140    	mov.w	r1, #0xc000
 8005616: 9118         	str	r1, [sp, #0x60]
 8005618: f64b 7170    	movw	r1, #0xbf70
 800561c: 9123         	str	r1, [sp, #0x8c]
 800561e: f64b 71e8    	movw	r1, #0xbfe8
 8005622: 9117         	str	r1, [sp, #0x5c]
 8005624: f44f 3198    	mov.w	r1, #0x13000
 8005628: 9122         	str	r1, [sp, #0x88]
 800562a: f64b 71d0    	movw	r1, #0xbfd0
 800562e: f100 0324    	add.w	r3, r0, #0x24
 8005632: 9116         	str	r1, [sp, #0x58]
 8005634: f64b 7188    	movw	r1, #0xbf88
 8005638: 931b         	str	r3, [sp, #0x6c]
 800563a: f100 0320    	add.w	r3, r0, #0x20
 800563e: 9121         	str	r1, [sp, #0x84]
 8005640: f64b 71b8    	movw	r1, #0xbfb8
 8005644: f100 061c    	add.w	r6, r0, #0x1c
 8005648: f100 0518    	add.w	r5, r0, #0x18
 800564c: f100 0914    	add.w	r9, r0, #0x14
 8005650: f100 0b04    	add.w	r11, r0, #0x4
 8005654: 931a         	str	r3, [sp, #0x68]
 8005656: f64b 7328    	movw	r3, #0xbf28
 800565a: 9120         	str	r1, [sp, #0x80]
 800565c: f64b 71a0    	movw	r1, #0xbfa0
 8005660: 911f         	str	r1, [sp, #0x7c]
 8005662: f100 0110    	add.w	r1, r0, #0x10
 8005666: 911e         	str	r1, [sp, #0x78]
 8005668: a912         	add	r1, sp, #0x48
 800566a: 46aa         	mov	r10, r5
 800566c: e881 0218    	stm.w	r1, {r3, r4, r9}
 8005670: 9924         	ldr	r1, [sp, #0x90]
 8005672: 9515         	str	r5, [sp, #0x54]
 8005674: 18cd         	adds	r5, r1, r3
 8005676: 901d         	str	r0, [sp, #0x74]
 8005678: 981d         	ldr	r0, [sp, #0x74]
 800567a: 961c         	str	r6, [sp, #0x70]
 800567c: f8d5 3108    	ldr.w	r3, [r5, #0x108]
 8005680: 4408         	add	r0, r1
 8005682: f8d5 6120    	ldr.w	r6, [r5, #0x120]
 8005686: f8cd 8038    	str.w	r8, [sp, #0x38]
 800568a: eb01 0804    	add.w	r8, r1, r4
 800568e: 1af3         	subs	r3, r6, r3
 8005690: 62c3         	str	r3, [r0, #0x2c]
 8005692: 9a23         	ldr	r2, [sp, #0x8c]
 8005694: 0058         	lsls	r0, r3, #0x1
 8005696: e9cd be10    	strd	r11, lr, [sp, #64]
 800569a: ea80 70e3    	eor.w	r0, r0, r3, asr #31
 800569e: eb01 030e    	add.w	r3, r1, lr
 80056a2: f8c8 0004    	str.w	r0, [r8, #0x4]
 80056a6: 900d         	str	r0, [sp, #0x34]
 80056a8: f8d5 0120    	ldr.w	r0, [r5, #0x120]
 80056ac: f8d3 6108    	ldr.w	r6, [r3, #0x108]
 80056b0: 9c17         	ldr	r4, [sp, #0x5c]
 80056b2: 1a30         	subs	r0, r6, r0
 80056b4: eb01 060b    	add.w	r6, r1, r11
 80056b8: f8dd b058    	ldr.w	r11, [sp, #0x58]
 80056bc: 62f0         	str	r0, [r6, #0x2c]
 80056be: 0046         	lsls	r6, r0, #0x1
 80056c0: ea86 70e0    	eor.w	r0, r6, r0, asr #31
 80056c4: f8c8 0008    	str.w	r0, [r8, #0x8]
 80056c8: 900c         	str	r0, [sp, #0x30]
 80056ca: f8d3 0108    	ldr.w	r0, [r3, #0x108]
 80056ce: 188b         	adds	r3, r1, r2
 80056d0: 9a22         	ldr	r2, [sp, #0x88]
 80056d2: f8d3 6108    	ldr.w	r6, [r3, #0x108]
 80056d6: f8dd e068    	ldr.w	lr, [sp, #0x68]
 80056da: 1a30         	subs	r0, r6, r0
 80056dc: 188e         	adds	r6, r1, r2
 80056de: 9a21         	ldr	r2, [sp, #0x84]
 80056e0: 62f0         	str	r0, [r6, #0x2c]
 80056e2: 0046         	lsls	r6, r0, #0x1
 80056e4: ea86 70e0    	eor.w	r0, r6, r0, asr #31
 80056e8: f8c8 000c    	str.w	r0, [r8, #0xc]
 80056ec: 900b         	str	r0, [sp, #0x2c]
 80056ee: f244 062c    	movw	r6, #0x402c
 80056f2: f8d3 0108    	ldr.w	r0, [r3, #0x108]
 80056f6: 188b         	adds	r3, r1, r2
 80056f8: f2c0 0601    	movt	r6, #0x1
 80056fc: f8cd c03c    	str.w	r12, [sp, #0x3c]
 8005700: f8d3 2108    	ldr.w	r2, [r3, #0x108]
 8005704: 1a10         	subs	r0, r2, r0
 8005706: eb01 020c    	add.w	r2, r1, r12
 800570a: f8dd c06c    	ldr.w	r12, [sp, #0x6c]
 800570e: 5190         	str	r0, [r2, r6]
 8005710: 0042         	lsls	r2, r0, #0x1
 8005712: ea82 70e0    	eor.w	r0, r2, r0, asr #31
 8005716: 9a1f         	ldr	r2, [sp, #0x7c]
 8005718: eb01 060e    	add.w	r6, r1, lr
 800571c: f8c8 0010    	str.w	r0, [r8, #0x10]
 8005720: f10e 0e2c    	add.w	lr, lr, #0x2c
 8005724: 440a         	add	r2, r1
 8005726: 900a         	str	r0, [sp, #0x28]
 8005728: f8d3 0108    	ldr.w	r0, [r3, #0x108]
 800572c: f8d2 3108    	ldr.w	r3, [r2, #0x108]
 8005730: f8cd e068    	str.w	lr, [sp, #0x68]
 8005734: 1a18         	subs	r0, r3, r0
 8005736: 9b1e         	ldr	r3, [sp, #0x78]
 8005738: f8dd e044    	ldr.w	lr, [sp, #0x44]
 800573c: 440b         	add	r3, r1
 800573e: f50e 7e84    	add.w	lr, lr, #0x108
 8005742: 62d8         	str	r0, [r3, #0x2c]
 8005744: 0043         	lsls	r3, r0, #0x1
 8005746: ea83 70e0    	eor.w	r0, r3, r0, asr #31
 800574a: f8c8 0014    	str.w	r0, [r8, #0x14]
 800574e: 9009         	str	r0, [sp, #0x24]
 8005750: f8d2 0108    	ldr.w	r0, [r2, #0x108]
 8005754: 9a20         	ldr	r2, [sp, #0x80]
 8005756: 440a         	add	r2, r1
 8005758: f8d2 3108    	ldr.w	r3, [r2, #0x108]
 800575c: 1a18         	subs	r0, r3, r0
 800575e: eb01 0309    	add.w	r3, r1, r9
 8005762: 62d8         	str	r0, [r3, #0x2c]
 8005764: 0043         	lsls	r3, r0, #0x1
 8005766: ea83 70e0    	eor.w	r0, r3, r0, asr #31
 800576a: f8c8 0018    	str.w	r0, [r8, #0x18]
 800576e: 9008         	str	r0, [sp, #0x20]
 8005770: f8d2 0108    	ldr.w	r0, [r2, #0x108]
 8005774: eb01 020b    	add.w	r2, r1, r11
 8005778: f50b 7b84    	add.w	r11, r11, #0x108
 800577c: f8cd b058    	str.w	r11, [sp, #0x58]
 8005780: f8d2 3108    	ldr.w	r3, [r2, #0x108]
 8005784: f8dd b040    	ldr.w	r11, [sp, #0x40]
 8005788: 1a18         	subs	r0, r3, r0
 800578a: eb01 030a    	add.w	r3, r1, r10
 800578e: f8dd a060    	ldr.w	r10, [sp, #0x60]
 8005792: f10b 0b2c    	add.w	r11, r11, #0x2c
 8005796: 62d8         	str	r0, [r3, #0x2c]
 8005798: 0043         	lsls	r3, r0, #0x1
 800579a: ea83 70e0    	eor.w	r0, r3, r0, asr #31
 800579e: f8c8 001c    	str.w	r0, [r8, #0x1c]
 80057a2: 9007         	str	r0, [sp, #0x1c]
 80057a4: f8d2 0108    	ldr.w	r0, [r2, #0x108]
 80057a8: 190a         	adds	r2, r1, r4
 80057aa: f504 7484    	add.w	r4, r4, #0x108
 80057ae: 9417         	str	r4, [sp, #0x5c]
 80057b0: f8d2 3108    	ldr.w	r3, [r2, #0x108]
 80057b4: 9c13         	ldr	r4, [sp, #0x4c]
 80057b6: 1a18         	subs	r0, r3, r0
 80057b8: 9b1c         	ldr	r3, [sp, #0x70]
 80057ba: 342c         	adds	r4, #0x2c
 80057bc: 440b         	add	r3, r1
 80057be: 62d8         	str	r0, [r3, #0x2c]
 80057c0: 0043         	lsls	r3, r0, #0x1
 80057c2: ea83 73e0    	eor.w	r3, r3, r0, asr #31
 80057c6: f8c8 3020    	str.w	r3, [r8, #0x20]
 80057ca: f8d2 0108    	ldr.w	r0, [r2, #0x108]
 80057ce: eb01 020a    	add.w	r2, r1, r10
 80057d2: f50a 7a84    	add.w	r10, r10, #0x108
 80057d6: f8cd a060    	str.w	r10, [sp, #0x60]
 80057da: f8d2 9108    	ldr.w	r9, [r2, #0x108]
 80057de: eba9 0000    	sub.w	r0, r9, r0
 80057e2: 62f0         	str	r0, [r6, #0x2c]
 80057e4: f8dd 9064    	ldr.w	r9, [sp, #0x64]
 80057e8: 0046         	lsls	r6, r0, #0x1
 80057ea: ea86 70e0    	eor.w	r0, r6, r0, asr #31
 80057ee: f8c8 0024    	str.w	r0, [r8, #0x24]
 80057f2: f8d2 2108    	ldr.w	r2, [r2, #0x108]
 80057f6: f8d5 61f8    	ldr.w	r6, [r5, #0x1f8]
 80057fa: 1ab2         	subs	r2, r6, r2
 80057fc: eb01 060c    	add.w	r6, r1, r12
 8005800: f10c 0c2c    	add.w	r12, r12, #0x2c
 8005804: f8cd c06c    	str.w	r12, [sp, #0x6c]
 8005808: 62f2         	str	r2, [r6, #0x2c]
 800580a: 0056         	lsls	r6, r2, #0x1
 800580c: ea86 72e2    	eor.w	r2, r6, r2, asr #31
 8005810: f8c8 2028    	str.w	r2, [r8, #0x28]
 8005814: eb01 0609    	add.w	r6, r1, r9
 8005818: f8d5 11f8    	ldr.w	r1, [r5, #0x1f8]
 800581c: f8d5 5210    	ldr.w	r5, [r5, #0x210]
 8005820: f8dd c03c    	ldr.w	r12, [sp, #0x3c]
 8005824: 1a69         	subs	r1, r5, r1
 8005826: 62f1         	str	r1, [r6, #0x2c]
 8005828: f11c 0c2c    	adds.w	r12, r12, #0x2c
 800582c: ea4f 0541    	lsl.w	r5, r1, #0x1
 8005830: ea85 71e1    	eor.w	r1, r5, r1, asr #31
 8005834: e9dd 650d    	ldrd	r6, r5, [sp, #52]
 8005838: f8c8 102c    	str.w	r1, [r8, #0x2c]
 800583c: 4435         	add	r5, r6
 800583e: 9e0c         	ldr	r6, [sp, #0x30]
 8005840: 4435         	add	r5, r6
 8005842: 9e0b         	ldr	r6, [sp, #0x2c]
 8005844: 4435         	add	r5, r6
 8005846: 9e0a         	ldr	r6, [sp, #0x28]
 8005848: 442e         	add	r6, r5
 800584a: 9d09         	ldr	r5, [sp, #0x24]
 800584c: 442e         	add	r6, r5
 800584e: 9d08         	ldr	r5, [sp, #0x20]
 8005850: 442e         	add	r6, r5
 8005852: 9d07         	ldr	r5, [sp, #0x1c]
 8005854: 442e         	add	r6, r5
 8005856: 9d15         	ldr	r5, [sp, #0x54]
 8005858: 4433         	add	r3, r6
 800585a: 4418         	add	r0, r3
 800585c: 9b12         	ldr	r3, [sp, #0x48]
 800585e: 4410         	add	r0, r2
 8005860: 464a         	mov	r2, r9
 8005862: eb01 0800    	add.w	r8, r1, r0
 8005866: 991f         	ldr	r1, [sp, #0x7c]
 8005868: e9dd 601c    	ldrd	r6, r0, [sp, #112]
 800586c: f105 052c    	add.w	r5, r5, #0x2c
 8005870: f501 7184    	add.w	r1, r1, #0x108
 8005874: 911f         	str	r1, [sp, #0x7c]
 8005876: 991e         	ldr	r1, [sp, #0x78]
 8005878: f106 062c    	add.w	r6, r6, #0x2c
 800587c: f8dd 9050    	ldr.w	r9, [sp, #0x50]
 8005880: f503 7384    	add.w	r3, r3, #0x108
 8005884: f101 012c    	add.w	r1, r1, #0x2c
 8005888: 911e         	str	r1, [sp, #0x78]
 800588a: 9920         	ldr	r1, [sp, #0x80]
 800588c: f109 092c    	add.w	r9, r9, #0x2c
 8005890: f100 002c    	add.w	r0, r0, #0x2c
 8005894: f102 022c    	add.w	r2, r2, #0x2c
 8005898: f501 7184    	add.w	r1, r1, #0x108
 800589c: 9120         	str	r1, [sp, #0x80]
 800589e: 9921         	ldr	r1, [sp, #0x84]
 80058a0: 9219         	str	r2, [sp, #0x64]
 80058a2: f501 7184    	add.w	r1, r1, #0x108
 80058a6: 9121         	str	r1, [sp, #0x84]
 80058a8: 9922         	ldr	r1, [sp, #0x88]
 80058aa: f101 012c    	add.w	r1, r1, #0x2c
 80058ae: 9122         	str	r1, [sp, #0x88]
 80058b0: 9923         	ldr	r1, [sp, #0x8c]
 80058b2: f501 7184    	add.w	r1, r1, #0x108
 80058b6: 9123         	str	r1, [sp, #0x8c]
 80058b8: f47f aed6    	bne.w	0x8005668 <raylar_crcl_encoder_service::benchmark::prepare::hb9734b8d5dc7f78e+0x60e> @ imm = #-0x254
 80058bc: 9805         	ldr	r0, [sp, #0x14]
 80058be: f1b8 0f00    	cmp.w	r8, #0x0
 80058c2: f8c0 8004    	str.w	r8, [r0, #0x4]
 80058c6: d017         	beq	0x80058f8 <raylar_crcl_encoder_service::benchmark::prepare::hb9734b8d5dc7f78e+0x89e> @ imm = #0x2e
 80058c8: f241 0005    	movw	r0, #0x1005
 80058cc: f2c0 0040    	movt	r0, #0x40
 80058d0: fba8 0100    	umull	r0, r1, r8, r0
 80058d4: eba8 0001    	sub.w	r0, r8, r1
 80058d8: eb01 0050    	add.w	r0, r1, r0, lsr #1
 80058dc: 2101         	movs	r1, #0x1
 80058de: 0a42         	lsrs	r2, r0, #0x9
 80058e0: eba2 2382    	sub.w	r3, r2, r2, lsl #10
 80058e4: eb13 0308    	adds.w	r3, r3, r8
 80058e8: bf18         	it	ne
 80058ea: eb01 2250    	addne.w	r2, r1, r0, lsr #9
 80058ee: fab2 f082    	clz	r0, r2
 80058f2: f1c0 001f    	rsb.w	r0, r0, #0x1f
 80058f6: e000         	b	0x80058fa <raylar_crcl_encoder_service::benchmark::prepare::hb9734b8d5dc7f78e+0x8a0> @ imm = #0x0
 80058f8: 2000         	movs	r0, #0x0
 80058fa: 9924         	ldr	r1, [sp, #0x90]
 80058fc: f24f 0204    	movw	r2, #0xf004
 8005900: f6cf 72ff    	movt	r2, #0xffff
 8005904: f88d 0150    	strb.w	r0, [sp, #0x150]
 8005908: b2c0         	uxtb	r0, r0
 800590a: 7b09         	ldrb	r1, [r1, #0xc]
 800590c: 4288         	cmp	r0, r1
 800590e: f040 851e    	bne.w	0x800634e <raylar_crcl_encoder_service::benchmark::prepare::hb9734b8d5dc7f78e+0x12f4> @ imm = #0xa3c
 8005912: 9804         	ldr	r0, [sp, #0x10]
 8005914: f249 0408    	movw	r4, #0x9008
 8005918: 9905         	ldr	r1, [sp, #0x14]
 800591a: f04f 0800    	mov.w	r8, #0x0
 800591e: f2c0 0401    	movt	r4, #0x1
 8005922: f64b 7e5c    	movw	lr, #0xbf5c
 8005926: 68c0         	ldr	r0, [r0, #0xc]
 8005928: 4694         	mov	r12, r2
 800592a: 6188         	str	r0, [r1, #0x18]
 800592c: f643 70f4    	movw	r0, #0x3ff4
 8005930: f2c0 0001    	movt	r0, #0x1
 8005934: f100 0128    	add.w	r1, r0, #0x28
 8005938: 9119         	str	r1, [sp, #0x64]
 800593a: f24c 0104    	movw	r1, #0xc004
 800593e: 9118         	str	r1, [sp, #0x60]
 8005940: f64b 7174    	movw	r1, #0xbf74
 8005944: 9123         	str	r1, [sp, #0x8c]
 8005946: f64b 71ec    	movw	r1, #0xbfec
 800594a: 9117         	str	r1, [sp, #0x5c]
 800594c: f64b 71d4    	movw	r1, #0xbfd4
 8005950: 9116         	str	r1, [sp, #0x58]
 8005952: f64b 718c    	movw	r1, #0xbf8c
 8005956: f100 0324    	add.w	r3, r0, #0x24
 800595a: 9122         	str	r1, [sp, #0x88]
 800595c: f64b 71bc    	movw	r1, #0xbfbc
 8005960: 931b         	str	r3, [sp, #0x6c]
 8005962: f100 0320    	add.w	r3, r0, #0x20
 8005966: 9121         	str	r1, [sp, #0x84]
 8005968: f44f 31a0    	mov.w	r1, #0x14000
 800596c: f100 061c    	add.w	r6, r0, #0x1c
 8005970: f100 0518    	add.w	r5, r0, #0x18
 8005974: f100 0914    	add.w	r9, r0, #0x14
 8005978: f100 0b08    	add.w	r11, r0, #0x8
 800597c: 931a         	str	r3, [sp, #0x68]
 800597e: f64b 732c    	movw	r3, #0xbf2c
 8005982: 9120         	str	r1, [sp, #0x80]
 8005984: f64b 71a4    	movw	r1, #0xbfa4
 8005988: 911f         	str	r1, [sp, #0x7c]
 800598a: 1d01         	adds	r1, r0, #0x4
 800598c: 911e         	str	r1, [sp, #0x78]
 800598e: a90f         	add	r1, sp, #0x3c
 8005990: 46aa         	mov	r10, r5
 8005992: e881 4900    	stm.w	r1, {r8, r11, lr}
 8005996: a912         	add	r1, sp, #0x48
 8005998: e881 0218    	stm.w	r1, {r3, r4, r9}
 800599c: 9924         	ldr	r1, [sp, #0x90]
 800599e: 9515         	str	r5, [sp, #0x54]
 80059a0: 18cd         	adds	r5, r1, r3
 80059a2: 901d         	str	r0, [sp, #0x74]
 80059a4: 981d         	ldr	r0, [sp, #0x74]
 80059a6: eb01 0804    	add.w	r8, r1, r4
 80059aa: 961c         	str	r6, [sp, #0x70]
 80059ac: f8d5 3108    	ldr.w	r3, [r5, #0x108]
 80059b0: 4408         	add	r0, r1
 80059b2: f8d5 6120    	ldr.w	r6, [r5, #0x120]
 80059b6: 9a1e         	ldr	r2, [sp, #0x78]
 80059b8: 1af3         	subs	r3, r6, r3
 80059ba: 62c3         	str	r3, [r0, #0x2c]
 80059bc: 9c18         	ldr	r4, [sp, #0x60]
 80059be: 0058         	lsls	r0, r3, #0x1
 80059c0: f8cd c038    	str.w	r12, [sp, #0x38]
 80059c4: ea80 70e3    	eor.w	r0, r0, r3, asr #31
 80059c8: eb01 030e    	add.w	r3, r1, lr
 80059cc: f8c8 0004    	str.w	r0, [r8, #0x4]
 80059d0: 900d         	str	r0, [sp, #0x34]
 80059d2: f8d5 0120    	ldr.w	r0, [r5, #0x120]
 80059d6: f8d3 6108    	ldr.w	r6, [r3, #0x108]
 80059da: f8dd e068    	ldr.w	lr, [sp, #0x68]
 80059de: 1a30         	subs	r0, r6, r0
 80059e0: 188e         	adds	r6, r1, r2
 80059e2: 9a23         	ldr	r2, [sp, #0x8c]
 80059e4: 62f0         	str	r0, [r6, #0x2c]
 80059e6: 0046         	lsls	r6, r0, #0x1
 80059e8: ea86 70e0    	eor.w	r0, r6, r0, asr #31
 80059ec: f8c8 0008    	str.w	r0, [r8, #0x8]
 80059f0: 900c         	str	r0, [sp, #0x30]
 80059f2: f8d3 0108    	ldr.w	r0, [r3, #0x108]
 80059f6: 188b         	adds	r3, r1, r2
 80059f8: 9a22         	ldr	r2, [sp, #0x88]
 80059fa: f8d3 6108    	ldr.w	r6, [r3, #0x108]
 80059fe: 1a30         	subs	r0, r6, r0
 8005a00: eb01 060b    	add.w	r6, r1, r11
 8005a04: f8dd b058    	ldr.w	r11, [sp, #0x58]
 8005a08: 62f0         	str	r0, [r6, #0x2c]
 8005a0a: 0046         	lsls	r6, r0, #0x1
 8005a0c: ea86 70e0    	eor.w	r0, r6, r0, asr #31
 8005a10: f8c8 000c    	str.w	r0, [r8, #0xc]
 8005a14: 900b         	str	r0, [sp, #0x2c]
 8005a16: f245 062c    	movw	r6, #0x502c
 8005a1a: f8d3 0108    	ldr.w	r0, [r3, #0x108]
 8005a1e: 188b         	adds	r3, r1, r2
 8005a20: f2c0 0601    	movt	r6, #0x1
 8005a24: f8d3 2108    	ldr.w	r2, [r3, #0x108]
 8005a28: 1a10         	subs	r0, r2, r0
 8005a2a: 9a20         	ldr	r2, [sp, #0x80]
 8005a2c: 440a         	add	r2, r1
 8005a2e: 62d0         	str	r0, [r2, #0x2c]
 8005a30: 0042         	lsls	r2, r0, #0x1
 8005a32: ea82 70e0    	eor.w	r0, r2, r0, asr #31
 8005a36: 9a1f         	ldr	r2, [sp, #0x7c]
 8005a38: f8c8 0010    	str.w	r0, [r8, #0x10]
 8005a3c: 440a         	add	r2, r1
 8005a3e: 900a         	str	r0, [sp, #0x28]
 8005a40: f8d3 0108    	ldr.w	r0, [r3, #0x108]
 8005a44: f8d2 3108    	ldr.w	r3, [r2, #0x108]
 8005a48: 1a18         	subs	r0, r3, r0
 8005a4a: eb01 030c    	add.w	r3, r1, r12
 8005a4e: f8dd c06c    	ldr.w	r12, [sp, #0x6c]
 8005a52: 5198         	str	r0, [r3, r6]
 8005a54: 0043         	lsls	r3, r0, #0x1
 8005a56: ea83 70e0    	eor.w	r0, r3, r0, asr #31
 8005a5a: f8c8 0014    	str.w	r0, [r8, #0x14]
 8005a5e: eb01 060e    	add.w	r6, r1, lr
 8005a62: 9009         	str	r0, [sp, #0x24]
 8005a64: f10e 0e2c    	add.w	lr, lr, #0x2c
 8005a68: f8d2 0108    	ldr.w	r0, [r2, #0x108]
 8005a6c: 9a21         	ldr	r2, [sp, #0x84]
 8005a6e: f8cd e068    	str.w	lr, [sp, #0x68]
 8005a72: 440a         	add	r2, r1
 8005a74: f8dd e044    	ldr.w	lr, [sp, #0x44]
 8005a78: f8d2 3108    	ldr.w	r3, [r2, #0x108]
 8005a7c: f50e 7e84    	add.w	lr, lr, #0x108
 8005a80: 1a18         	subs	r0, r3, r0
 8005a82: eb01 0309    	add.w	r3, r1, r9
 8005a86: 62d8         	str	r0, [r3, #0x2c]
 8005a88: 0043         	lsls	r3, r0, #0x1
 8005a8a: ea83 70e0    	eor.w	r0, r3, r0, asr #31
 8005a8e: f8c8 0018    	str.w	r0, [r8, #0x18]
 8005a92: 9008         	str	r0, [sp, #0x20]
 8005a94: f8d2 0108    	ldr.w	r0, [r2, #0x108]
 8005a98: eb01 020b    	add.w	r2, r1, r11
 8005a9c: f50b 7b84    	add.w	r11, r11, #0x108
 8005aa0: f8cd b058    	str.w	r11, [sp, #0x58]
 8005aa4: f8d2 3108    	ldr.w	r3, [r2, #0x108]
 8005aa8: f8dd b040    	ldr.w	r11, [sp, #0x40]
 8005aac: 1a18         	subs	r0, r3, r0
 8005aae: eb01 030a    	add.w	r3, r1, r10
 8005ab2: f8dd a05c    	ldr.w	r10, [sp, #0x5c]
 8005ab6: f10b 0b2c    	add.w	r11, r11, #0x2c
 8005aba: 62d8         	str	r0, [r3, #0x2c]
 8005abc: 0043         	lsls	r3, r0, #0x1
 8005abe: ea83 70e0    	eor.w	r0, r3, r0, asr #31
 8005ac2: f8c8 001c    	str.w	r0, [r8, #0x1c]
 8005ac6: 9007         	str	r0, [sp, #0x1c]
 8005ac8: f8d2 0108    	ldr.w	r0, [r2, #0x108]
 8005acc: eb01 020a    	add.w	r2, r1, r10
 8005ad0: f50a 7a84    	add.w	r10, r10, #0x108
 8005ad4: f8cd a05c    	str.w	r10, [sp, #0x5c]
 8005ad8: f8d2 3108    	ldr.w	r3, [r2, #0x108]
 8005adc: 1a18         	subs	r0, r3, r0
 8005ade: 9b1c         	ldr	r3, [sp, #0x70]
 8005ae0: 440b         	add	r3, r1
 8005ae2: 62d8         	str	r0, [r3, #0x2c]
 8005ae4: 0043         	lsls	r3, r0, #0x1
 8005ae6: ea83 73e0    	eor.w	r3, r3, r0, asr #31
 8005aea: f8c8 3020    	str.w	r3, [r8, #0x20]
 8005aee: f8d2 0108    	ldr.w	r0, [r2, #0x108]
 8005af2: 190a         	adds	r2, r1, r4
 8005af4: f504 7484    	add.w	r4, r4, #0x108
 8005af8: 9418         	str	r4, [sp, #0x60]
 8005afa: f8d2 9108    	ldr.w	r9, [r2, #0x108]
 8005afe: 9c13         	ldr	r4, [sp, #0x4c]
 8005b00: eba9 0000    	sub.w	r0, r9, r0
 8005b04: 62f0         	str	r0, [r6, #0x2c]
 8005b06: f8dd 9064    	ldr.w	r9, [sp, #0x64]
 8005b0a: 342c         	adds	r4, #0x2c
 8005b0c: 0046         	lsls	r6, r0, #0x1
 8005b0e: ea86 70e0    	eor.w	r0, r6, r0, asr #31
 8005b12: f8c8 0024    	str.w	r0, [r8, #0x24]
 8005b16: f8d2 2108    	ldr.w	r2, [r2, #0x108]
 8005b1a: f8d5 61f8    	ldr.w	r6, [r5, #0x1f8]
 8005b1e: 1ab2         	subs	r2, r6, r2
 8005b20: eb01 060c    	add.w	r6, r1, r12
 8005b24: f10c 0c2c    	add.w	r12, r12, #0x2c
 8005b28: f8cd c06c    	str.w	r12, [sp, #0x6c]
 8005b2c: 62f2         	str	r2, [r6, #0x2c]
 8005b2e: 0056         	lsls	r6, r2, #0x1
 8005b30: ea86 72e2    	eor.w	r2, r6, r2, asr #31
 8005b34: f8c8 2028    	str.w	r2, [r8, #0x28]
 8005b38: eb01 0609    	add.w	r6, r1, r9
 8005b3c: f8d5 11f8    	ldr.w	r1, [r5, #0x1f8]
 8005b40: f8d5 5210    	ldr.w	r5, [r5, #0x210]
 8005b44: f8dd c038    	ldr.w	r12, [sp, #0x38]
 8005b48: 1a69         	subs	r1, r5, r1
 8005b4a: 62f1         	str	r1, [r6, #0x2c]
 8005b4c: 9e0d         	ldr	r6, [sp, #0x34]
 8005b4e: f11c 0c2c    	adds.w	r12, r12, #0x2c
 8005b52: ea4f 0541    	lsl.w	r5, r1, #0x1
 8005b56: ea85 71e1    	eor.w	r1, r5, r1, asr #31
 8005b5a: 9d0f         	ldr	r5, [sp, #0x3c]
 8005b5c: f8c8 102c    	str.w	r1, [r8, #0x2c]
 8005b60: 4435         	add	r5, r6
 8005b62: 9e0c         	ldr	r6, [sp, #0x30]
 8005b64: 4435         	add	r5, r6
 8005b66: 9e0b         	ldr	r6, [sp, #0x2c]
 8005b68: 4435         	add	r5, r6
 8005b6a: 9e0a         	ldr	r6, [sp, #0x28]
 8005b6c: 442e         	add	r6, r5
 8005b6e: 9d09         	ldr	r5, [sp, #0x24]
 8005b70: 442e         	add	r6, r5
 8005b72: 9d08         	ldr	r5, [sp, #0x20]
 8005b74: 442e         	add	r6, r5
 8005b76: 9d07         	ldr	r5, [sp, #0x1c]
 8005b78: 442e         	add	r6, r5
 8005b7a: 9d15         	ldr	r5, [sp, #0x54]
 8005b7c: 4433         	add	r3, r6
 8005b7e: 4418         	add	r0, r3
 8005b80: 9b12         	ldr	r3, [sp, #0x48]
 8005b82: 4410         	add	r0, r2
 8005b84: 464a         	mov	r2, r9
 8005b86: eb01 0800    	add.w	r8, r1, r0
 8005b8a: 991f         	ldr	r1, [sp, #0x7c]
 8005b8c: e9dd 601c    	ldrd	r6, r0, [sp, #112]
 8005b90: f105 052c    	add.w	r5, r5, #0x2c
 8005b94: f501 7184    	add.w	r1, r1, #0x108
 8005b98: 911f         	str	r1, [sp, #0x7c]
 8005b9a: 9920         	ldr	r1, [sp, #0x80]
 8005b9c: f106 062c    	add.w	r6, r6, #0x2c
 8005ba0: f8dd 9050    	ldr.w	r9, [sp, #0x50]
 8005ba4: f503 7384    	add.w	r3, r3, #0x108
 8005ba8: f101 012c    	add.w	r1, r1, #0x2c
 8005bac: 9120         	str	r1, [sp, #0x80]
 8005bae: 9921         	ldr	r1, [sp, #0x84]
 8005bb0: f109 092c    	add.w	r9, r9, #0x2c
 8005bb4: f100 002c    	add.w	r0, r0, #0x2c
 8005bb8: f102 022c    	add.w	r2, r2, #0x2c
 8005bbc: f501 7184    	add.w	r1, r1, #0x108
 8005bc0: 9121         	str	r1, [sp, #0x84]
 8005bc2: 9922         	ldr	r1, [sp, #0x88]
 8005bc4: 9219         	str	r2, [sp, #0x64]
 8005bc6: f501 7184    	add.w	r1, r1, #0x108
 8005bca: 9122         	str	r1, [sp, #0x88]
 8005bcc: 9923         	ldr	r1, [sp, #0x8c]
 8005bce: f501 7184    	add.w	r1, r1, #0x108
 8005bd2: 9123         	str	r1, [sp, #0x8c]
 8005bd4: 991e         	ldr	r1, [sp, #0x78]
 8005bd6: f101 012c    	add.w	r1, r1, #0x2c
 8005bda: 911e         	str	r1, [sp, #0x78]
 8005bdc: f47f aed7    	bne.w	0x800598e <raylar_crcl_encoder_service::benchmark::prepare::hb9734b8d5dc7f78e+0x934> @ imm = #-0x252
 8005be0: 9805         	ldr	r0, [sp, #0x14]
 8005be2: f1b8 0f00    	cmp.w	r8, #0x0
 8005be6: f8c0 8008    	str.w	r8, [r0, #0x8]
 8005bea: d017         	beq	0x8005c1c <raylar_crcl_encoder_service::benchmark::prepare::hb9734b8d5dc7f78e+0xbc2> @ imm = #0x2e
 8005bec: f241 0005    	movw	r0, #0x1005
 8005bf0: f2c0 0040    	movt	r0, #0x40
 8005bf4: fba8 0100    	umull	r0, r1, r8, r0
 8005bf8: eba8 0001    	sub.w	r0, r8, r1
 8005bfc: eb01 0050    	add.w	r0, r1, r0, lsr #1
 8005c00: 2101         	movs	r1, #0x1
 8005c02: 0a42         	lsrs	r2, r0, #0x9
 8005c04: eba2 2382    	sub.w	r3, r2, r2, lsl #10
 8005c08: eb13 0308    	adds.w	r3, r3, r8
 8005c0c: bf18         	it	ne
 8005c0e: eb01 2250    	addne.w	r2, r1, r0, lsr #9
 8005c12: fab2 f082    	clz	r0, r2
 8005c16: f1c0 001f    	rsb.w	r0, r0, #0x1f
 8005c1a: e000         	b	0x8005c1e <raylar_crcl_encoder_service::benchmark::prepare::hb9734b8d5dc7f78e+0xbc4> @ imm = #0x0
 8005c1c: 2000         	movs	r0, #0x0
 8005c1e: 9924         	ldr	r1, [sp, #0x90]
 8005c20: f24f 0204    	movw	r2, #0xf004
 8005c24: f6cf 72ff    	movt	r2, #0xffff
 8005c28: f88d 0150    	strb.w	r0, [sp, #0x150]
 8005c2c: b2c0         	uxtb	r0, r0
 8005c2e: 7d09         	ldrb	r1, [r1, #0x14]
 8005c30: 4288         	cmp	r0, r1
 8005c32: f040 838f    	bne.w	0x8006354 <raylar_crcl_encoder_service::benchmark::prepare::hb9734b8d5dc7f78e+0x12fa> @ imm = #0x71e
 8005c36: 9804         	ldr	r0, [sp, #0x10]
 8005c38: f644 7ef0    	movw	lr, #0x4ff0
 8005c3c: 9905         	ldr	r1, [sp, #0x14]
 8005c3e: f2c0 0e01    	movt	lr, #0x1
 8005c42: f10e 0320    	add.w	r3, lr, #0x20
 8005c46: f10e 061c    	add.w	r6, lr, #0x1c
 8005c4a: 6980         	ldr	r0, [r0, #0x18]
 8005c4c: f10e 0908    	add.w	r9, lr, #0x8
 8005c50: 61c8         	str	r0, [r1, #0x1c]
 8005c52: f24c 0108    	movw	r1, #0xc008
 8005c56: f10e 0028    	add.w	r0, lr, #0x28
 8005c5a: 9117         	str	r1, [sp, #0x5c]
 8005c5c: f64b 7178    	movw	r1, #0xbf78
 8005c60: 901a         	str	r0, [sp, #0x68]
 8005c62: f10e 0024    	add.w	r0, lr, #0x24
 8005c66: 9122         	str	r1, [sp, #0x88]
 8005c68: f64b 71f0    	movw	r1, #0xbff0
 8005c6c: 9019         	str	r0, [sp, #0x64]
 8005c6e: f10e 0018    	add.w	r0, lr, #0x18
 8005c72: 9116         	str	r1, [sp, #0x58]
 8005c74: f64b 7190    	movw	r1, #0xbf90
 8005c78: 9018         	str	r0, [sp, #0x60]
 8005c7a: f10e 000c    	add.w	r0, lr, #0xc
 8005c7e: 9121         	str	r1, [sp, #0x84]
 8005c80: f64b 71c0    	movw	r1, #0xbfc0
 8005c84: 9023         	str	r0, [sp, #0x8c]
 8005c86: 2000         	movs	r0, #0x0
 8005c88: 9115         	str	r1, [sp, #0x54]
 8005c8a: f44f 31a8    	mov.w	r1, #0x15000
 8005c8e: f10e 0804    	add.w	r8, lr, #0x4
 8005c92: 901b         	str	r0, [sp, #0x6c]
 8005c94: f64b 7030    	movw	r0, #0xbf30
 8005c98: 9120         	str	r1, [sp, #0x80]
 8005c9a: f64b 71a8    	movw	r1, #0xbfa8
 8005c9e: f64b 7560    	movw	r5, #0xbf60
 8005ca2: f64b 7ad8    	movw	r10, #0xbfd8
 8005ca6: 4694         	mov	r12, r2
 8005ca8: 911f         	str	r1, [sp, #0x7c]
 8005caa: 9c24         	ldr	r4, [sp, #0x90]
 8005cac: e9cd 631c    	strd	r6, r3, [sp, #112]
 8005cb0: 1826         	adds	r6, r4, r0
 8005cb2: f8cd e078    	str.w	lr, [sp, #0x78]
 8005cb6: e9cd 5013    	strd	r5, r0, [sp, #76]
 8005cba: eb04 0e0c    	add.w	lr, r4, r12
 8005cbe: f8d6 0108    	ldr.w	r0, [r6, #0x108]
 8005cc2: f8d6 1120    	ldr.w	r1, [r6, #0x120]
 8005cc6: 9a1e         	ldr	r2, [sp, #0x78]
 8005cc8: 1a08         	subs	r0, r1, r0
 8005cca: 960b         	str	r6, [sp, #0x2c]
 8005ccc: 18a3         	adds	r3, r4, r2
 8005cce: f8dd b054    	ldr.w	r11, [sp, #0x54]
 8005cd2: 0041         	lsls	r1, r0, #0x1
 8005cd4: e9cd 9a11    	strd	r9, r10, [sp, #68]
 8005cd8: ea81 71e0    	eor.w	r1, r1, r0, asr #31
 8005cdc: 62d8         	str	r0, [r3, #0x2c]
 8005cde: f24b 0004    	movw	r0, #0xb004
 8005ce2: 910e         	str	r1, [sp, #0x38]
 8005ce4: f2c0 0001    	movt	r0, #0x1
 8005ce8: e9cd c80f    	strd	r12, r8, [sp, #60]
 8005cec: f84e 1000    	str.w	r1, [lr, r0]
 8005cf0: 1961         	adds	r1, r4, r5
 8005cf2: f8d6 0120    	ldr.w	r0, [r6, #0x120]
 8005cf6: f8d1 3108    	ldr.w	r3, [r1, #0x108]
 8005cfa: 1a18         	subs	r0, r3, r0
 8005cfc: eb04 0308    	add.w	r3, r4, r8
 8005d00: f8dd 8058    	ldr.w	r8, [sp, #0x58]
 8005d04: 62d8         	str	r0, [r3, #0x2c]
 8005d06: 0043         	lsls	r3, r0, #0x1
 8005d08: ea83 72e0    	eor.w	r2, r3, r0, asr #31
 8005d0c: f24b 0008    	movw	r0, #0xb008
 8005d10: f2c0 0001    	movt	r0, #0x1
 8005d14: 920d         	str	r2, [sp, #0x34]
 8005d16: f84e 2000    	str.w	r2, [lr, r0]
 8005d1a: f8d1 0108    	ldr.w	r0, [r1, #0x108]
 8005d1e: 9922         	ldr	r1, [sp, #0x88]
 8005d20: 4421         	add	r1, r4
 8005d22: f8d1 3108    	ldr.w	r3, [r1, #0x108]
 8005d26: 1a18         	subs	r0, r3, r0
 8005d28: eb04 0309    	add.w	r3, r4, r9
 8005d2c: f8dd 9060    	ldr.w	r9, [sp, #0x60]
 8005d30: 62d8         	str	r0, [r3, #0x2c]
 8005d32: 0043         	lsls	r3, r0, #0x1
 8005d34: ea83 72e0    	eor.w	r2, r3, r0, asr #31
 8005d38: f24b 000c    	movw	r0, #0xb00c
 8005d3c: f2c0 0001    	movt	r0, #0x1
 8005d40: 920c         	str	r2, [sp, #0x30]
 8005d42: f84e 2000    	str.w	r2, [lr, r0]
 8005d46: f8d1 0108    	ldr.w	r0, [r1, #0x108]
 8005d4a: 9921         	ldr	r1, [sp, #0x84]
 8005d4c: 9a23         	ldr	r2, [sp, #0x8c]
 8005d4e: 4421         	add	r1, r4
 8005d50: f8d1 5108    	ldr.w	r5, [r1, #0x108]
 8005d54: 1a28         	subs	r0, r5, r0
 8005d56: 18a5         	adds	r5, r4, r2
 8005d58: 62e8         	str	r0, [r5, #0x2c]
 8005d5a: 0045         	lsls	r5, r0, #0x1
 8005d5c: ea85 72e0    	eor.w	r2, r5, r0, asr #31
 8005d60: f24b 0010    	movw	r0, #0xb010
 8005d64: f2c0 0001    	movt	r0, #0x1
 8005d68: 920a         	str	r2, [sp, #0x28]
 8005d6a: f84e 2000    	str.w	r2, [lr, r0]
 8005d6e: f8d1 0108    	ldr.w	r0, [r1, #0x108]
 8005d72: 991f         	ldr	r1, [sp, #0x7c]
 8005d74: 9a20         	ldr	r2, [sp, #0x80]
 8005d76: 4421         	add	r1, r4
 8005d78: f8d1 6108    	ldr.w	r6, [r1, #0x108]
 8005d7c: 1a30         	subs	r0, r6, r0
 8005d7e: 18a6         	adds	r6, r4, r2
 8005d80: 62f0         	str	r0, [r6, #0x2c]
 8005d82: 0046         	lsls	r6, r0, #0x1
 8005d84: ea86 72e0    	eor.w	r2, r6, r0, asr #31
 8005d88: f24b 0014    	movw	r0, #0xb014
 8005d8c: f2c0 0001    	movt	r0, #0x1
 8005d90: 9209         	str	r2, [sp, #0x24]
 8005d92: f84e 2000    	str.w	r2, [lr, r0]
 8005d96: f246 022c    	movw	r2, #0x602c
 8005d9a: f2c0 0201    	movt	r2, #0x1
 8005d9e: f8d1 0108    	ldr.w	r0, [r1, #0x108]
 8005da2: eb04 010b    	add.w	r1, r4, r11
 8005da6: 9e1d         	ldr	r6, [sp, #0x74]
 8005da8: f50b 7b84    	add.w	r11, r11, #0x108
 8005dac: f8d1 3108    	ldr.w	r3, [r1, #0x108]
 8005db0: f8cd b054    	str.w	r11, [sp, #0x54]
 8005db4: 1a18         	subs	r0, r3, r0
 8005db6: f84e 0002    	str.w	r0, [lr, r2]
 8005dba: 0043         	lsls	r3, r0, #0x1
 8005dbc: ea83 72e0    	eor.w	r2, r3, r0, asr #31
 8005dc0: f24b 0018    	movw	r0, #0xb018
 8005dc4: eb04 030a    	add.w	r3, r4, r10
 8005dc8: f2c0 0001    	movt	r0, #0x1
 8005dcc: 9208         	str	r2, [sp, #0x20]
 8005dce: f84e 2000    	str.w	r2, [lr, r0]
 8005dd2: f8d1 0108    	ldr.w	r0, [r1, #0x108]
 8005dd6: f8d3 1108    	ldr.w	r1, [r3, #0x108]
 8005dda: 9a1c         	ldr	r2, [sp, #0x70]
 8005ddc: 1a08         	subs	r0, r1, r0
 8005dde: eb04 0109    	add.w	r1, r4, r9
 8005de2: 4422         	add	r2, r4
 8005de4: f109 092c    	add.w	r9, r9, #0x2c
 8005de8: 62c8         	str	r0, [r1, #0x2c]
 8005dea: 0041         	lsls	r1, r0, #0x1
 8005dec: ea81 71e0    	eor.w	r1, r1, r0, asr #31
 8005df0: f24b 001c    	movw	r0, #0xb01c
 8005df4: f2c0 0001    	movt	r0, #0x1
 8005df8: f8cd 9060    	str.w	r9, [sp, #0x60]
 8005dfc: f84e 1000    	str.w	r1, [lr, r0]
 8005e00: f8d3 0108    	ldr.w	r0, [r3, #0x108]
 8005e04: eb04 0308    	add.w	r3, r4, r8
 8005e08: f508 7884    	add.w	r8, r8, #0x108
 8005e0c: f8dd 9044    	ldr.w	r9, [sp, #0x44]
 8005e10: f8d3 c108    	ldr.w	r12, [r3, #0x108]
 8005e14: f8cd 8058    	str.w	r8, [sp, #0x58]
 8005e18: f109 092c    	add.w	r9, r9, #0x2c
 8005e1c: ebac 0000    	sub.w	r0, r12, r0
 8005e20: 62d0         	str	r0, [r2, #0x2c]
 8005e22: f8dd c064    	ldr.w	r12, [sp, #0x64]
 8005e26: 0042         	lsls	r2, r0, #0x1
 8005e28: f8dd 8040    	ldr.w	r8, [sp, #0x40]
 8005e2c: ea82 72e0    	eor.w	r2, r2, r0, asr #31
 8005e30: f24b 0020    	movw	r0, #0xb020
 8005e34: f2c0 0001    	movt	r0, #0x1
 8005e38: 9207         	str	r2, [sp, #0x1c]
 8005e3a: f84e 2000    	str.w	r2, [lr, r0]
 8005e3e: 4620         	mov	r0, r4
 8005e40: f108 082c    	add.w	r8, r8, #0x2c
 8005e44: 9c17         	ldr	r4, [sp, #0x5c]
 8005e46: f8d3 2108    	ldr.w	r2, [r3, #0x108]
 8005e4a: 1903         	adds	r3, r0, r4
 8005e4c: f504 7484    	add.w	r4, r4, #0x108
 8005e50: 9417         	str	r4, [sp, #0x5c]
 8005e52: f8d3 5108    	ldr.w	r5, [r3, #0x108]
 8005e56: 1aaa         	subs	r2, r5, r2
 8005e58: 1985         	adds	r5, r0, r6
 8005e5a: 9e0b         	ldr	r6, [sp, #0x2c]
 8005e5c: 62ea         	str	r2, [r5, #0x2c]
 8005e5e: 0055         	lsls	r5, r2, #0x1
 8005e60: ea85 7ae2    	eor.w	r10, r5, r2, asr #31
 8005e64: f24b 0224    	movw	r2, #0xb024
 8005e68: f2c0 0201    	movt	r2, #0x1
 8005e6c: f84e a002    	str.w	r10, [lr, r2]
 8005e70: f24b 0228    	movw	r2, #0xb028
 8005e74: f2c0 0201    	movt	r2, #0x1
 8005e78: f8d3 3108    	ldr.w	r3, [r3, #0x108]
 8005e7c: f8d6 51f8    	ldr.w	r5, [r6, #0x1f8]
 8005e80: 1aeb         	subs	r3, r5, r3
 8005e82: eb00 050c    	add.w	r5, r0, r12
 8005e86: f10c 0c2c    	add.w	r12, r12, #0x2c
 8005e8a: f8cd c064    	str.w	r12, [sp, #0x64]
 8005e8e: 62eb         	str	r3, [r5, #0x2c]
 8005e90: 005d         	lsls	r5, r3, #0x1
 8005e92: ea85 73e3    	eor.w	r3, r5, r3, asr #31
 8005e96: f84e 3002    	str.w	r3, [lr, r2]
 8005e9a: 9a1a         	ldr	r2, [sp, #0x68]
 8005e9c: f8dd c03c    	ldr.w	r12, [sp, #0x3c]
 8005ea0: 1885         	adds	r5, r0, r2
 8005ea2: f8d6 01f8    	ldr.w	r0, [r6, #0x1f8]
 8005ea6: f8d6 6210    	ldr.w	r6, [r6, #0x210]
 8005eaa: f11c 0c2c    	adds.w	r12, r12, #0x2c
 8005eae: f102 022c    	add.w	r2, r2, #0x2c
 8005eb2: 921a         	str	r2, [sp, #0x68]
 8005eb4: eba6 0000    	sub.w	r0, r6, r0
 8005eb8: 62e8         	str	r0, [r5, #0x2c]
 8005eba: 9e1b         	ldr	r6, [sp, #0x6c]
 8005ebc: ea4f 0540    	lsl.w	r5, r0, #0x1
 8005ec0: ea85 70e0    	eor.w	r0, r5, r0, asr #31
 8005ec4: f24b 052c    	movw	r5, #0xb02c
 8005ec8: f2c0 0501    	movt	r5, #0x1
 8005ecc: f84e 0005    	str.w	r0, [lr, r5]
 8005ed0: 9d0e         	ldr	r5, [sp, #0x38]
 8005ed2: f8dd e078    	ldr.w	lr, [sp, #0x78]
 8005ed6: 442e         	add	r6, r5
 8005ed8: 9d0d         	ldr	r5, [sp, #0x34]
 8005eda: f10e 0e2c    	add.w	lr, lr, #0x2c
 8005ede: 442e         	add	r6, r5
 8005ee0: 9d0c         	ldr	r5, [sp, #0x30]
 8005ee2: 442e         	add	r6, r5
 8005ee4: 9d0a         	ldr	r5, [sp, #0x28]
 8005ee6: 442e         	add	r6, r5
 8005ee8: 9d09         	ldr	r5, [sp, #0x24]
 8005eea: 442e         	add	r6, r5
 8005eec: 9d08         	ldr	r5, [sp, #0x20]
 8005eee: 442e         	add	r6, r5
 8005ef0: 9d1d         	ldr	r5, [sp, #0x74]
 8005ef2: 4431         	add	r1, r6
 8005ef4: 9e07         	ldr	r6, [sp, #0x1c]
 8005ef6: 4431         	add	r1, r6
 8005ef8: 9e1c         	ldr	r6, [sp, #0x70]
 8005efa: 4451         	add	r1, r10
 8005efc: f8dd a048    	ldr.w	r10, [sp, #0x48]
 8005f00: 4419         	add	r1, r3
 8005f02: 9b22         	ldr	r3, [sp, #0x88]
 8005f04: 4408         	add	r0, r1
 8005f06: 991f         	ldr	r1, [sp, #0x7c]
 8005f08: 901b         	str	r0, [sp, #0x6c]
 8005f0a: f503 7384    	add.w	r3, r3, #0x108
 8005f0e: f501 7184    	add.w	r1, r1, #0x108
 8005f12: 911f         	str	r1, [sp, #0x7c]
 8005f14: 9920         	ldr	r1, [sp, #0x80]
 8005f16: f50a 7a84    	add.w	r10, r10, #0x108
 8005f1a: 9814         	ldr	r0, [sp, #0x50]
 8005f1c: f106 062c    	add.w	r6, r6, #0x2c
 8005f20: f101 012c    	add.w	r1, r1, #0x2c
 8005f24: 9120         	str	r1, [sp, #0x80]
 8005f26: 9923         	ldr	r1, [sp, #0x8c]
 8005f28: f500 7084    	add.w	r0, r0, #0x108
 8005f2c: 9322         	str	r3, [sp, #0x88]
 8005f2e: f105 032c    	add.w	r3, r5, #0x2c
 8005f32: f101 012c    	add.w	r1, r1, #0x2c
 8005f36: 9123         	str	r1, [sp, #0x8c]
 8005f38: 9921         	ldr	r1, [sp, #0x84]
 8005f3a: f501 7184    	add.w	r1, r1, #0x108
 8005f3e: 9121         	str	r1, [sp, #0x84]
 8005f40: 9913         	ldr	r1, [sp, #0x4c]
 8005f42: f501 7584    	add.w	r5, r1, #0x108
 8005f46: f47f aeb0    	bne.w	0x8005caa <raylar_crcl_encoder_service::benchmark::prepare::hb9734b8d5dc7f78e+0xc50> @ imm = #-0x2a0
 8005f4a: 9805         	ldr	r0, [sp, #0x14]
 8005f4c: 991b         	ldr	r1, [sp, #0x6c]
 8005f4e: 60c1         	str	r1, [r0, #0xc]
 8005f50: b1b1         	cbz	r1, 0x8005f80 <raylar_crcl_encoder_service::benchmark::prepare::hb9734b8d5dc7f78e+0xf26> @ imm = #0x2c
 8005f52: 9e1b         	ldr	r6, [sp, #0x6c]
 8005f54: f241 0005    	movw	r0, #0x1005
 8005f58: f2c0 0040    	movt	r0, #0x40
 8005f5c: fba6 0100    	umull	r0, r1, r6, r0
 8005f60: 1a70         	subs	r0, r6, r1
 8005f62: eb01 0050    	add.w	r0, r1, r0, lsr #1
 8005f66: 2101         	movs	r1, #0x1
 8005f68: 0a42         	lsrs	r2, r0, #0x9
 8005f6a: eba2 2382    	sub.w	r3, r2, r2, lsl #10
 8005f6e: 199b         	adds	r3, r3, r6
 8005f70: bf18         	it	ne
 8005f72: eb01 2250    	addne.w	r2, r1, r0, lsr #9
 8005f76: fab2 f082    	clz	r0, r2
 8005f7a: f1c0 001f    	rsb.w	r0, r0, #0x1f
 8005f7e: e000         	b	0x8005f82 <raylar_crcl_encoder_service::benchmark::prepare::hb9734b8d5dc7f78e+0xf28> @ imm = #0x0
 8005f80: 2000         	movs	r0, #0x0
 8005f82: 9924         	ldr	r1, [sp, #0x90]
 8005f84: f24f 0c04    	movw	r12, #0xf004
 8005f88: f6cf 7cff    	movt	r12, #0xffff
 8005f8c: f88d 0150    	strb.w	r0, [sp, #0x150]
 8005f90: b2c0         	uxtb	r0, r0
 8005f92: 7f09         	ldrb	r1, [r1, #0x1c]
 8005f94: 4288         	cmp	r0, r1
 8005f96: f040 81e0    	bne.w	0x800635a <raylar_crcl_encoder_service::benchmark::prepare::hb9734b8d5dc7f78e+0x1300> @ imm = #0x3c0
 8005f9a: 9804         	ldr	r0, [sp, #0x10]
 8005f9c: f64b 7364    	movw	r3, #0xbf64
 8005fa0: 9905         	ldr	r1, [sp, #0x14]
 8005fa2: f44f 34d8    	mov.w	r4, #0x1b000
 8005fa6: 9322         	str	r3, [sp, #0x88]
 8005fa8: f24c 030c    	movw	r3, #0xc00c
 8005fac: 6a40         	ldr	r0, [r0, #0x24]
 8005fae: f64b 7adc    	movw	r10, #0xbfdc
 8005fb2: 6208         	str	r0, [r1, #0x20]
 8005fb4: f645 70ec    	movw	r0, #0x5fec
 8005fb8: f2c0 0001    	movt	r0, #0x1
 8005fbc: f100 0128    	add.w	r1, r0, #0x28
 8005fc0: 9318         	str	r3, [sp, #0x60]
 8005fc2: f64b 737c    	movw	r3, #0xbf7c
 8005fc6: 911b         	str	r1, [sp, #0x6c]
 8005fc8: f100 0124    	add.w	r1, r0, #0x24
 8005fcc: 9321         	str	r3, [sp, #0x84]
 8005fce: f64b 73f4    	movw	r3, #0xbff4
 8005fd2: 911a         	str	r1, [sp, #0x68]
 8005fd4: f100 0120    	add.w	r1, r0, #0x20
 8005fd8: 9317         	str	r3, [sp, #0x5c]
 8005fda: f64b 7394    	movw	r3, #0xbf94
 8005fde: 9119         	str	r1, [sp, #0x64]
 8005fe0: f100 0110    	add.w	r1, r0, #0x10
 8005fe4: 9320         	str	r3, [sp, #0x80]
 8005fe6: f44f 33b0    	mov.w	r3, #0x16000
 8005fea: 9123         	str	r1, [sp, #0x8c]
 8005fec: 2100         	movs	r1, #0x0
 8005fee: 9316         	str	r3, [sp, #0x58]
 8005ff0: f64b 73c4    	movw	r3, #0xbfc4
 8005ff4: f100 081c    	add.w	r8, r0, #0x1c
 8005ff8: f100 060c    	add.w	r6, r0, #0xc
 8005ffc: f100 0b08    	add.w	r11, r0, #0x8
 8006000: f100 0904    	add.w	r9, r0, #0x4
 8006004: 911c         	str	r1, [sp, #0x70]
 8006006: f64b 7134    	movw	r1, #0xbf34
 800600a: 931f         	str	r3, [sp, #0x7c]
 800600c: f64b 73ac    	movw	r3, #0xbfac
 8006010: 931e         	str	r3, [sp, #0x78]
 8006012: 9b24         	ldr	r3, [sp, #0x90]
 8006014: 46d6         	mov	lr, r10
 8006016: e9cd a110    	strd	r10, r1, [sp, #64]
 800601a: 4632         	mov	r2, r6
 800601c: eb03 0a01    	add.w	r10, r3, r1
 8006020: e9cd 6813    	strd	r6, r8, [sp, #76]
 8006024: 1819         	adds	r1, r3, r0
 8006026: 9015         	str	r0, [sp, #0x54]
 8006028: f8da 6108    	ldr.w	r6, [r10, #0x108]
 800602c: f8da 5120    	ldr.w	r5, [r10, #0x120]
 8006030: e9cd 9b0e    	strd	r9, r11, [sp, #56]
 8006034: 1bae         	subs	r6, r5, r6
 8006036: 62ce         	str	r6, [r1, #0x2c]
 8006038: f8cd c074    	str.w	r12, [sp, #0x74]
 800603c: 46c4         	mov	r12, r8
 800603e: 0071         	lsls	r1, r6, #0x1
 8006040: ea81 70e6    	eor.w	r0, r1, r6, asr #31
 8006044: 191e         	adds	r6, r3, r4
 8006046: 900d         	str	r0, [sp, #0x34]
 8006048: f8dd 805c    	ldr.w	r8, [sp, #0x5c]
 800604c: 6070         	str	r0, [r6, #0x4]
 800604e: 9822         	ldr	r0, [sp, #0x88]
 8006050: f8da 1120    	ldr.w	r1, [r10, #0x120]
 8006054: 181d         	adds	r5, r3, r0
 8006056: 9412         	str	r4, [sp, #0x48]
 8006058: 9c19         	ldr	r4, [sp, #0x64]
 800605a: f8d5 0108    	ldr.w	r0, [r5, #0x108]
 800605e: 1a40         	subs	r0, r0, r1
 8006060: eb03 0109    	add.w	r1, r3, r9
 8006064: 62c8         	str	r0, [r1, #0x2c]
 8006066: 0041         	lsls	r1, r0, #0x1
 8006068: ea81 70e0    	eor.w	r0, r1, r0, asr #31
 800606c: 9921         	ldr	r1, [sp, #0x84]
 800606e: 60b0         	str	r0, [r6, #0x8]
 8006070: 4419         	add	r1, r3
 8006072: 900c         	str	r0, [sp, #0x30]
 8006074: f8d5 0108    	ldr.w	r0, [r5, #0x108]
 8006078: f8d1 5108    	ldr.w	r5, [r1, #0x108]
 800607c: 1a28         	subs	r0, r5, r0
 800607e: eb03 050b    	add.w	r5, r3, r11
 8006082: f8dd b058    	ldr.w	r11, [sp, #0x58]
 8006086: 62e8         	str	r0, [r5, #0x2c]
 8006088: 0045         	lsls	r5, r0, #0x1
 800608a: ea85 70e0    	eor.w	r0, r5, r0, asr #31
 800608e: 60f0         	str	r0, [r6, #0xc]
 8006090: 900b         	str	r0, [sp, #0x2c]
 8006092: f8d1 0108    	ldr.w	r0, [r1, #0x108]
 8006096: 9920         	ldr	r1, [sp, #0x80]
 8006098: 4419         	add	r1, r3
 800609a: f8d1 5108    	ldr.w	r5, [r1, #0x108]
 800609e: 1a28         	subs	r0, r5, r0
 80060a0: 189d         	adds	r5, r3, r2
 80060a2: 62e8         	str	r0, [r5, #0x2c]
 80060a4: 0045         	lsls	r5, r0, #0x1
 80060a6: ea85 70e0    	eor.w	r0, r5, r0, asr #31
 80060aa: 6130         	str	r0, [r6, #0x10]
 80060ac: 900a         	str	r0, [sp, #0x28]
 80060ae: f247 052c    	movw	r5, #0x702c
 80060b2: f8d1 0108    	ldr.w	r0, [r1, #0x108]
 80060b6: f2c0 0501    	movt	r5, #0x1
 80060ba: 991e         	ldr	r1, [sp, #0x78]
 80060bc: 4419         	add	r1, r3
 80060be: f8d1 2108    	ldr.w	r2, [r1, #0x108]
 80060c2: 1a10         	subs	r0, r2, r0
 80060c4: 9a23         	ldr	r2, [sp, #0x8c]
 80060c6: 441a         	add	r2, r3
 80060c8: 62d0         	str	r0, [r2, #0x2c]
 80060ca: 0042         	lsls	r2, r0, #0x1
 80060cc: ea82 70e0    	eor.w	r0, r2, r0, asr #31
 80060d0: 6170         	str	r0, [r6, #0x14]
 80060d2: 9009         	str	r0, [sp, #0x24]
 80060d4: f8d1 0108    	ldr.w	r0, [r1, #0x108]
 80060d8: 991f         	ldr	r1, [sp, #0x7c]
 80060da: 4419         	add	r1, r3
 80060dc: f8d1 2108    	ldr.w	r2, [r1, #0x108]
 80060e0: 1a10         	subs	r0, r2, r0
 80060e2: eb03 020b    	add.w	r2, r3, r11
 80060e6: f10b 0b2c    	add.w	r11, r11, #0x2c
 80060ea: f8cd b058    	str.w	r11, [sp, #0x58]
 80060ee: 62d0         	str	r0, [r2, #0x2c]
 80060f0: 0042         	lsls	r2, r0, #0x1
 80060f2: ea82 70e0    	eor.w	r0, r2, r0, asr #31
 80060f6: 61b0         	str	r0, [r6, #0x18]
 80060f8: 9008         	str	r0, [sp, #0x20]
 80060fa: f8d1 0108    	ldr.w	r0, [r1, #0x108]
 80060fe: eb03 010e    	add.w	r1, r3, lr
 8006102: f8dd e068    	ldr.w	lr, [sp, #0x68]
 8006106: f8d1 2108    	ldr.w	r2, [r1, #0x108]
 800610a: 1a10         	subs	r0, r2, r0
 800610c: 9a1d         	ldr	r2, [sp, #0x74]
 800610e: 441a         	add	r2, r3
 8006110: 5150         	str	r0, [r2, r5]
 8006112: 0042         	lsls	r2, r0, #0x1
 8006114: ea82 70e0    	eor.w	r0, r2, r0, asr #31
 8006118: 61f0         	str	r0, [r6, #0x1c]
 800611a: 9007         	str	r0, [sp, #0x1c]
 800611c: f8d1 0108    	ldr.w	r0, [r1, #0x108]
 8006120: eb03 0108    	add.w	r1, r3, r8
 8006124: 9d18         	ldr	r5, [sp, #0x60]
 8006126: f508 7884    	add.w	r8, r8, #0x108
 800612a: f8d1 2108    	ldr.w	r2, [r1, #0x108]
 800612e: f8cd 805c    	str.w	r8, [sp, #0x5c]
 8006132: 1a10         	subs	r0, r2, r0
 8006134: eb03 020c    	add.w	r2, r3, r12
 8006138: f8dd c06c    	ldr.w	r12, [sp, #0x6c]
 800613c: 62d0         	str	r0, [r2, #0x2c]
 800613e: 0042         	lsls	r2, r0, #0x1
 8006140: ea82 70e0    	eor.w	r0, r2, r0, asr #31
 8006144: 6230         	str	r0, [r6, #0x20]
 8006146: 9006         	str	r0, [sp, #0x18]
 8006148: f8d1 0108    	ldr.w	r0, [r1, #0x108]
 800614c: 1959         	adds	r1, r3, r5
 800614e: f505 7584    	add.w	r5, r5, #0x108
 8006152: 9518         	str	r5, [sp, #0x60]
 8006154: f8d1 2108    	ldr.w	r2, [r1, #0x108]
 8006158: 1a10         	subs	r0, r2, r0
 800615a: 191a         	adds	r2, r3, r4
 800615c: 342c         	adds	r4, #0x2c
 800615e: 9419         	str	r4, [sp, #0x64]
 8006160: 62d0         	str	r0, [r2, #0x2c]
 8006162: 0042         	lsls	r2, r0, #0x1
 8006164: ea82 79e0    	eor.w	r9, r2, r0, asr #31
 8006168: f8c6 9024    	str.w	r9, [r6, #0x24]
 800616c: f8d1 1108    	ldr.w	r1, [r1, #0x108]
 8006170: f8da 21f8    	ldr.w	r2, [r10, #0x1f8]
 8006174: 9c12         	ldr	r4, [sp, #0x48]
 8006176: 1a51         	subs	r1, r2, r1
 8006178: eb03 020e    	add.w	r2, r3, lr
 800617c: 342c         	adds	r4, #0x2c
 800617e: f10e 0e2c    	add.w	lr, lr, #0x2c
 8006182: 62d1         	str	r1, [r2, #0x2c]
 8006184: 004a         	lsls	r2, r1, #0x1
 8006186: ea82 71e1    	eor.w	r1, r2, r1, asr #31
 800618a: 62b1         	str	r1, [r6, #0x28]
 800618c: eb03 020c    	add.w	r2, r3, r12
 8006190: f8da 01f8    	ldr.w	r0, [r10, #0x1f8]
 8006194: f8da 3210    	ldr.w	r3, [r10, #0x210]
 8006198: f10c 0c2c    	add.w	r12, r12, #0x2c
 800619c: f8dd a040    	ldr.w	r10, [sp, #0x40]
 80061a0: 1a18         	subs	r0, r3, r0
 80061a2: 62d0         	str	r0, [r2, #0x2c]
 80061a4: 9b0d         	ldr	r3, [sp, #0x34]
 80061a6: f50a 7a84    	add.w	r10, r10, #0x108
 80061aa: 0042         	lsls	r2, r0, #0x1
 80061ac: ea82 70e0    	eor.w	r0, r2, r0, asr #31
 80061b0: 9a1c         	ldr	r2, [sp, #0x70]
 80061b2: 62f0         	str	r0, [r6, #0x2c]
 80061b4: 441a         	add	r2, r3
 80061b6: 9b0c         	ldr	r3, [sp, #0x30]
 80061b8: 9e13         	ldr	r6, [sp, #0x4c]
 80061ba: 441a         	add	r2, r3
 80061bc: 9b0b         	ldr	r3, [sp, #0x2c]
 80061be: f8cd c06c    	str.w	r12, [sp, #0x6c]
 80061c2: 362c         	adds	r6, #0x2c
 80061c4: 441a         	add	r2, r3
 80061c6: 9b0a         	ldr	r3, [sp, #0x28]
 80061c8: f8dd c074    	ldr.w	r12, [sp, #0x74]
 80061cc: 441a         	add	r2, r3
 80061ce: 9b09         	ldr	r3, [sp, #0x24]
 80061d0: f11c 0c2c    	adds.w	r12, r12, #0x2c
 80061d4: f8cd e068    	str.w	lr, [sp, #0x68]
 80061d8: 441a         	add	r2, r3
 80061da: 9b08         	ldr	r3, [sp, #0x20]
 80061dc: 441a         	add	r2, r3
 80061de: 9b07         	ldr	r3, [sp, #0x1c]
 80061e0: 441a         	add	r2, r3
 80061e2: 9b06         	ldr	r3, [sp, #0x18]
 80061e4: 441a         	add	r2, r3
 80061e6: 9b14         	ldr	r3, [sp, #0x50]
 80061e8: 444a         	add	r2, r9
 80061ea: e9dd 9b0e    	ldrd	r9, r11, [sp, #56]
 80061ee: 4411         	add	r1, r2
 80061f0: 9a1e         	ldr	r2, [sp, #0x78]
 80061f2: 4408         	add	r0, r1
 80061f4: f103 082c    	add.w	r8, r3, #0x2c
 80061f8: f502 7284    	add.w	r2, r2, #0x108
 80061fc: 921e         	str	r2, [sp, #0x78]
 80061fe: 9a23         	ldr	r2, [sp, #0x8c]
 8006200: f10b 0b2c    	add.w	r11, r11, #0x2c
 8006204: 9b21         	ldr	r3, [sp, #0x84]
 8006206: f109 092c    	add.w	r9, r9, #0x2c
 800620a: f102 022c    	add.w	r2, r2, #0x2c
 800620e: 9223         	str	r2, [sp, #0x8c]
 8006210: 9a1f         	ldr	r2, [sp, #0x7c]
 8006212: f503 7384    	add.w	r3, r3, #0x108
 8006216: 9911         	ldr	r1, [sp, #0x44]
 8006218: 901c         	str	r0, [sp, #0x70]
 800621a: f502 7284    	add.w	r2, r2, #0x108
 800621e: 9815         	ldr	r0, [sp, #0x54]
 8006220: f501 7184    	add.w	r1, r1, #0x108
 8006224: 921f         	str	r2, [sp, #0x7c]
 8006226: 9a20         	ldr	r2, [sp, #0x80]
 8006228: f100 002c    	add.w	r0, r0, #0x2c
 800622c: 9321         	str	r3, [sp, #0x84]
 800622e: 9b22         	ldr	r3, [sp, #0x88]
 8006230: f502 7284    	add.w	r2, r2, #0x108
 8006234: 9220         	str	r2, [sp, #0x80]
 8006236: f503 7384    	add.w	r3, r3, #0x108
 800623a: 9322         	str	r3, [sp, #0x88]
 800623c: f47f aee9    	bne.w	0x8006012 <raylar_crcl_encoder_service::benchmark::prepare::hb9734b8d5dc7f78e+0xfb8> @ imm = #-0x22e
 8006240: 9805         	ldr	r0, [sp, #0x14]
 8006242: 991c         	ldr	r1, [sp, #0x70]
 8006244: 6101         	str	r1, [r0, #0x10]
 8006246: b1b1         	cbz	r1, 0x8006276 <raylar_crcl_encoder_service::benchmark::prepare::hb9734b8d5dc7f78e+0x121c> @ imm = #0x2c
 8006248: 9e1c         	ldr	r6, [sp, #0x70]
 800624a: f241 0005    	movw	r0, #0x1005
 800624e: f2c0 0040    	movt	r0, #0x40
 8006252: fba6 0100    	umull	r0, r1, r6, r0
 8006256: 1a70         	subs	r0, r6, r1
 8006258: eb01 0050    	add.w	r0, r1, r0, lsr #1
 800625c: 2101         	movs	r1, #0x1
 800625e: 0a42         	lsrs	r2, r0, #0x9
 8006260: eba2 2382    	sub.w	r3, r2, r2, lsl #10
 8006264: 199b         	adds	r3, r3, r6
 8006266: bf18         	it	ne
 8006268: eb01 2250    	addne.w	r2, r1, r0, lsr #9
 800626c: fab2 f082    	clz	r0, r2
 8006270: f1c0 001f    	rsb.w	r0, r0, #0x1f
 8006274: e000         	b	0x8006278 <raylar_crcl_encoder_service::benchmark::prepare::hb9734b8d5dc7f78e+0x121e> @ imm = #0x0
 8006276: 2000         	movs	r0, #0x0
 8006278: 9924         	ldr	r1, [sp, #0x90]
 800627a: f88d 0150    	strb.w	r0, [sp, #0x150]
 800627e: b2c0         	uxtb	r0, r0
 8006280: f891 1024    	ldrb.w	r1, [r1, #0x24]
 8006284: 4288         	cmp	r0, r1
 8006286: d16b         	bne	0x8006360 <raylar_crcl_encoder_service::benchmark::prepare::hb9734b8d5dc7f78e+0x1306> @ imm = #0xd6
 8006288: 9804         	ldr	r0, [sp, #0x10]
 800628a: f24c 0288    	movw	r2, #0xc088
 800628e: 9905         	ldr	r1, [sp, #0x14]
 8006290: f641 0c06    	movw	r12, #0x1806
 8006294: 6b00         	ldr	r0, [r0, #0x30]
 8006296: 6248         	str	r0, [r1, #0x24]
 8006298: 2000         	movs	r0, #0x0
 800629a: 9924         	ldr	r1, [sp, #0x90]
 800629c: 188e         	adds	r6, r1, r2
 800629e: eb01 0380    	add.w	r3, r1, r0, lsl #2
 80062a2: f856 5c60    	ldr	r5, [r6, #-96]
 80062a6: 6a99         	ldr	r1, [r3, #0x28]
 80062a8: 42a9         	cmp	r1, r5
 80062aa: d138         	bne	0x800631e <raylar_crcl_encoder_service::benchmark::prepare::hb9734b8d5dc7f78e+0x12c4> @ imm = #0x70
 80062ac: f856 1c48    	ldr	r1, [r6, #-72]
 80062b0: 6c1d         	ldr	r5, [r3, #0x40]
 80062b2: 428d         	cmp	r5, r1
 80062b4: d12e         	bne	0x8006314 <raylar_crcl_encoder_service::benchmark::prepare::hb9734b8d5dc7f78e+0x12ba> @ imm = #0x5c
 80062b6: f856 1c30    	ldr	r1, [r6, #-48]
 80062ba: 6d9d         	ldr	r5, [r3, #0x58]
 80062bc: 428d         	cmp	r5, r1
 80062be: d12b         	bne	0x8006318 <raylar_crcl_encoder_service::benchmark::prepare::hb9734b8d5dc7f78e+0x12be> @ imm = #0x56
 80062c0: f856 1c18    	ldr	r1, [r6, #-24]
 80062c4: 6f1e         	ldr	r6, [r3, #0x70]
 80062c6: 428e         	cmp	r6, r1
 80062c8: d128         	bne	0x800631c <raylar_crcl_encoder_service::benchmark::prepare::hb9734b8d5dc7f78e+0x12c2> @ imm = #0x50
 80062ca: 301e         	adds	r0, #0x1e
 80062cc: 4560         	cmp	r0, r12
 80062ce: d00a         	beq	0x80062e6 <raylar_crcl_encoder_service::benchmark::prepare::hb9734b8d5dc7f78e+0x128c> @ imm = #0x14
 80062d0: 9e24         	ldr	r6, [sp, #0x90]
 80062d2: f102 0178    	add.w	r1, r2, #0x78
 80062d6: f8d3 3088    	ldr.w	r3, [r3, #0x88]
 80062da: 58b2         	ldr	r2, [r6, r2]
 80062dc: 4293         	cmp	r3, r2
 80062de: 460a         	mov	r2, r1
 80062e0: d0db         	beq	0x800629a <raylar_crcl_encoder_service::benchmark::prepare::hb9734b8d5dc7f78e+0x1240> @ imm = #-0x4a
 80062e2: 3806         	subs	r0, #0x6
 80062e4: e01b         	b	0x800631e <raylar_crcl_encoder_service::benchmark::prepare::hb9734b8d5dc7f78e+0x12c4> @ imm = #0x36
 80062e6: b079         	add	sp, #0x1e4
 80062e8: e8bd 0f00    	pop.w	{r8, r9, r10, r11}
 80062ec: bdf0         	pop	{r4, r5, r6, r7, pc}
 80062ee: f24e 0028    	movw	r0, #0xe028
 80062f2: f88d 1150    	strb.w	r1, [sp, #0x150]
 80062f6: f6c0 0000    	movt	r0, #0x800
 80062fa: f64d 53e4    	movw	r3, #0xdde4
 80062fe: 9000         	str	r0, [sp]
 8006300: f64d 5048    	movw	r0, #0xdd48
 8006304: aa54         	add	r2, sp, #0x150
 8006306: f6c0 0000    	movt	r0, #0x800
 800630a: f6c0 0300    	movt	r3, #0x800
 800630e: 212b         	movs	r1, #0x2b
 8006310: f005 fa82    	bl	0x800b818 <core::result::unwrap_failed> @ imm = #0x5504
 8006314: 3006         	adds	r0, #0x6
 8006316: e002         	b	0x800631e <raylar_crcl_encoder_service::benchmark::prepare::hb9734b8d5dc7f78e+0x12c4> @ imm = #0x4
 8006318: 300c         	adds	r0, #0xc
 800631a: e000         	b	0x800631e <raylar_crcl_encoder_service::benchmark::prepare::hb9734b8d5dc7f78e+0x12c4> @ imm = #0x0
 800631c: 3012         	adds	r0, #0x12
 800631e: f64d 71f8    	movw	r1, #0xdff8
 8006322: 9a02         	ldr	r2, [sp, #0x8]
 8006324: f6c0 0100    	movt	r1, #0x800
 8006328: 2300         	movs	r3, #0x0
 800632a: 9101         	str	r1, [sp, #0x4]
 800632c: eb02 0280    	add.w	r2, r2, r0, lsl #2
 8006330: 9903         	ldr	r1, [sp, #0xc]
 8006332: eb01 0180    	add.w	r1, r1, r0, lsl #2
 8006336: 2000         	movs	r0, #0x0
 8006338: f002 fbbe    	bl	0x8008ab8 <core::panicking::assert_failed::hef9a1b4b671d0724> @ imm = #0x277c
 800633c: f89d 0129    	ldrb.w	r0, [sp, #0x129]
 8006340: f88d 0150    	strb.w	r0, [sp, #0x150]
 8006344: f24e 0018    	movw	r0, #0xe018
 8006348: f6c0 0000    	movt	r0, #0x800
 800634c: e7d5         	b	0x80062fa <raylar_crcl_encoder_service::benchmark::prepare::hb9734b8d5dc7f78e+0x12a0> @ imm = #-0x56
 800634e: 9824         	ldr	r0, [sp, #0x90]
 8006350: 3008         	adds	r0, #0x8
 8006352: e007         	b	0x8006364 <raylar_crcl_encoder_service::benchmark::prepare::hb9734b8d5dc7f78e+0x130a> @ imm = #0xe
 8006354: 9824         	ldr	r0, [sp, #0x90]
 8006356: 3010         	adds	r0, #0x10
 8006358: e004         	b	0x8006364 <raylar_crcl_encoder_service::benchmark::prepare::hb9734b8d5dc7f78e+0x130a> @ imm = #0x8
 800635a: 9824         	ldr	r0, [sp, #0x90]
 800635c: 3018         	adds	r0, #0x18
 800635e: e001         	b	0x8006364 <raylar_crcl_encoder_service::benchmark::prepare::hb9734b8d5dc7f78e+0x130a> @ imm = #0x2
 8006360: 9824         	ldr	r0, [sp, #0x90]
 8006362: 3020         	adds	r0, #0x20
 8006364: 9024         	str	r0, [sp, #0x90]
 8006366: f24e 0008    	movw	r0, #0xe008
 800636a: a954         	add	r1, sp, #0x150
 800636c: f6c0 0000    	movt	r0, #0x800
 8006370: 2300         	movs	r3, #0x0
 8006372: 9001         	str	r0, [sp, #0x4]
 8006374: 9824         	ldr	r0, [sp, #0x90]
 8006376: 1d02         	adds	r2, r0, #0x4
 8006378: 2000         	movs	r0, #0x0
 800637a: f002 fb8f    	bl	0x8008a9c <core::panicking::assert_failed::h4992df13491a1642> @ imm = #0x271e

0800637e <raylar_crcl_encoder_service::benchmark::fold_sum::h16b920496cf2076e>:
 800637e: b5f0         	push	{r4, r5, r6, r7, lr}
 8006380: af03         	add	r7, sp, #0xc
 8006382: e92d 0f00    	push.w	{r8, r9, r10, r11}
 8006386: b083         	sub	sp, #0xc
 8006388: f500 3190    	add.w	r1, r0, #0x12000
 800638c: f04f 0800    	mov.w	r8, #0x0
 8006390: 3124         	adds	r1, #0x24
 8006392: f240 3eff    	movw	lr, #0x3ff
 8006396: f851 cf84    	ldr	r12, [r1, #132]!
 800639a: f1be 0e21    	subs.w	lr, lr, #0x21
 800639e: e951 5420    	ldrd	r5, r4, [r1, #-128]
 80063a2: ea4f 0644    	lsl.w	r6, r4, #0x1
 80063a6: e951 321e    	ldrd	r3, r2, [r1, #-120]
 80063aa: ea86 76e4    	eor.w	r6, r6, r4, asr #31
 80063ae: f851 9c08    	ldr	r9, [r1, #-8]
 80063b2: ea4f 0445    	lsl.w	r4, r5, #0x1
 80063b6: ea84 74e5    	eor.w	r4, r4, r5, asr #31
 80063ba: eb04 0508    	add.w	r5, r4, r8
 80063be: ea4f 0443    	lsl.w	r4, r3, #0x1
 80063c2: ea84 73e3    	eor.w	r3, r4, r3, asr #31
 80063c6: f851 4c70    	ldr	r4, [r1, #-112]
 80063ca: 442e         	add	r6, r5
 80063cc: ea4f 0542    	lsl.w	r5, r2, #0x1
 80063d0: 4433         	add	r3, r6
 80063d2: ea85 72e2    	eor.w	r2, r5, r2, asr #31
 80063d6: f851 5c6c    	ldr	r5, [r1, #-108]
 80063da: 441a         	add	r2, r3
 80063dc: ea4f 0344    	lsl.w	r3, r4, #0x1
 80063e0: f851 6c0c    	ldr	r6, [r1, #-12]
 80063e4: ea83 73e4    	eor.w	r3, r3, r4, asr #31
 80063e8: f851 4c68    	ldr	r4, [r1, #-104]
 80063ec: 441a         	add	r2, r3
 80063ee: ea4f 0345    	lsl.w	r3, r5, #0x1
 80063f2: ea83 73e5    	eor.w	r3, r3, r5, asr #31
 80063f6: f851 5c64    	ldr	r5, [r1, #-100]
 80063fa: 441a         	add	r2, r3
 80063fc: ea4f 0344    	lsl.w	r3, r4, #0x1
 8006400: ea83 73e4    	eor.w	r3, r3, r4, asr #31
 8006404: f851 4c60    	ldr	r4, [r1, #-96]
 8006408: 441a         	add	r2, r3
 800640a: ea4f 0345    	lsl.w	r3, r5, #0x1
 800640e: ea83 73e5    	eor.w	r3, r3, r5, asr #31
 8006412: f851 5c5c    	ldr	r5, [r1, #-92]
 8006416: 441a         	add	r2, r3
 8006418: ea4f 0344    	lsl.w	r3, r4, #0x1
 800641c: ea83 73e4    	eor.w	r3, r3, r4, asr #31
 8006420: f851 4c58    	ldr	r4, [r1, #-88]
 8006424: 441a         	add	r2, r3
 8006426: ea4f 0345    	lsl.w	r3, r5, #0x1
 800642a: ea83 73e5    	eor.w	r3, r3, r5, asr #31
 800642e: f851 5c54    	ldr	r5, [r1, #-84]
 8006432: 441a         	add	r2, r3
 8006434: ea4f 0344    	lsl.w	r3, r4, #0x1
 8006438: ea83 73e4    	eor.w	r3, r3, r4, asr #31
 800643c: f851 4c50    	ldr	r4, [r1, #-80]
 8006440: 441a         	add	r2, r3
 8006442: ea4f 0345    	lsl.w	r3, r5, #0x1
 8006446: ea83 73e5    	eor.w	r3, r3, r5, asr #31
 800644a: f851 5c4c    	ldr	r5, [r1, #-76]
 800644e: 441a         	add	r2, r3
 8006450: ea4f 0344    	lsl.w	r3, r4, #0x1
 8006454: ea83 73e4    	eor.w	r3, r3, r4, asr #31
 8006458: f851 4c48    	ldr	r4, [r1, #-72]
 800645c: 441a         	add	r2, r3
 800645e: ea4f 0345    	lsl.w	r3, r5, #0x1
 8006462: ea83 73e5    	eor.w	r3, r3, r5, asr #31
 8006466: f851 5c44    	ldr	r5, [r1, #-68]
 800646a: 441a         	add	r2, r3
 800646c: ea4f 0344    	lsl.w	r3, r4, #0x1
 8006470: ea83 73e4    	eor.w	r3, r3, r4, asr #31
 8006474: f851 4c40    	ldr	r4, [r1, #-64]
 8006478: 441a         	add	r2, r3
 800647a: ea4f 0345    	lsl.w	r3, r5, #0x1
 800647e: ea83 73e5    	eor.w	r3, r3, r5, asr #31
 8006482: f851 5c3c    	ldr	r5, [r1, #-60]
 8006486: 441a         	add	r2, r3
 8006488: ea4f 0344    	lsl.w	r3, r4, #0x1
 800648c: ea83 73e4    	eor.w	r3, r3, r4, asr #31
 8006490: f851 4c38    	ldr	r4, [r1, #-56]
 8006494: 441a         	add	r2, r3
 8006496: ea4f 0345    	lsl.w	r3, r5, #0x1
 800649a: ea83 73e5    	eor.w	r3, r3, r5, asr #31
 800649e: f851 5c34    	ldr	r5, [r1, #-52]
 80064a2: 441a         	add	r2, r3
 80064a4: ea4f 0344    	lsl.w	r3, r4, #0x1
 80064a8: ea83 73e4    	eor.w	r3, r3, r4, asr #31
 80064ac: f851 4c30    	ldr	r4, [r1, #-48]
 80064b0: 441a         	add	r2, r3
 80064b2: ea4f 0345    	lsl.w	r3, r5, #0x1
 80064b6: ea83 73e5    	eor.w	r3, r3, r5, asr #31
 80064ba: f851 5c2c    	ldr	r5, [r1, #-44]
 80064be: 441a         	add	r2, r3
 80064c0: ea4f 0344    	lsl.w	r3, r4, #0x1
 80064c4: ea83 73e4    	eor.w	r3, r3, r4, asr #31
 80064c8: f851 4c28    	ldr	r4, [r1, #-40]
 80064cc: 441a         	add	r2, r3
 80064ce: ea4f 0345    	lsl.w	r3, r5, #0x1
 80064d2: ea83 73e5    	eor.w	r3, r3, r5, asr #31
 80064d6: f851 5c24    	ldr	r5, [r1, #-36]
 80064da: 441a         	add	r2, r3
 80064dc: ea4f 0344    	lsl.w	r3, r4, #0x1
 80064e0: ea83 73e4    	eor.w	r3, r3, r4, asr #31
 80064e4: f851 4c20    	ldr	r4, [r1, #-32]
 80064e8: 441a         	add	r2, r3
 80064ea: ea4f 0345    	lsl.w	r3, r5, #0x1
 80064ee: ea83 73e5    	eor.w	r3, r3, r5, asr #31
 80064f2: f851 5c1c    	ldr	r5, [r1, #-28]
 80064f6: 441a         	add	r2, r3
 80064f8: ea4f 0344    	lsl.w	r3, r4, #0x1
 80064fc: ea83 73e4    	eor.w	r3, r3, r4, asr #31
 8006500: f851 4c18    	ldr	r4, [r1, #-24]
 8006504: 441a         	add	r2, r3
 8006506: ea4f 0345    	lsl.w	r3, r5, #0x1
 800650a: ea83 73e5    	eor.w	r3, r3, r5, asr #31
 800650e: f851 5c14    	ldr	r5, [r1, #-20]
 8006512: 441a         	add	r2, r3
 8006514: ea4f 0344    	lsl.w	r3, r4, #0x1
 8006518: ea83 73e4    	eor.w	r3, r3, r4, asr #31
 800651c: f851 4c10    	ldr	r4, [r1, #-16]
 8006520: 441a         	add	r2, r3
 8006522: ea4f 0345    	lsl.w	r3, r5, #0x1
 8006526: ea83 73e5    	eor.w	r3, r3, r5, asr #31
 800652a: f851 8c04    	ldr	r8, [r1, #-4]
 800652e: 441a         	add	r2, r3
 8006530: ea4f 0344    	lsl.w	r3, r4, #0x1
 8006534: ea83 73e4    	eor.w	r3, r3, r4, asr #31
 8006538: 441a         	add	r2, r3
 800653a: ea4f 0346    	lsl.w	r3, r6, #0x1
 800653e: ea83 73e6    	eor.w	r3, r3, r6, asr #31
 8006542: 441a         	add	r2, r3
 8006544: ea4f 0349    	lsl.w	r3, r9, #0x1
 8006548: ea83 73e9    	eor.w	r3, r3, r9, asr #31
 800654c: 441a         	add	r2, r3
 800654e: ea4f 0348    	lsl.w	r3, r8, #0x1
 8006552: ea83 73e8    	eor.w	r3, r3, r8, asr #31
 8006556: 441a         	add	r2, r3
 8006558: ea4f 034c    	lsl.w	r3, r12, #0x1
 800655c: ea83 73ec    	eor.w	r3, r3, r12, asr #31
 8006560: eb03 0802    	add.w	r8, r3, r2
 8006564: f47f af17    	bne.w	0x8006396 <raylar_crcl_encoder_service::benchmark::fold_sum::h16b920496cf2076e+0x18> @ imm = #-0x1d2
 8006568: f8cd 8008    	str.w	r8, [sp, #0x8]
 800656c: a902         	add	r1, sp, #0x8
 800656e: f04f 0900    	mov.w	r9, #0x0
 8006572: f240 38ff    	movw	r8, #0x3ff
 8006576: 9902         	ldr	r1, [sp, #0x8]
 8006578: 9101         	str	r1, [sp, #0x4]
 800657a: f500 3190    	add.w	r1, r0, #0x12000
 800657e: f501 5181    	add.w	r1, r1, #0x1020
 8006582: f851 ef84    	ldr	lr, [r1, #132]!
 8006586: f1b8 0821    	subs.w	r8, r8, #0x21
 800658a: e951 5420    	ldrd	r5, r4, [r1, #-128]
 800658e: ea4f 0644    	lsl.w	r6, r4, #0x1
 8006592: e951 321e    	ldrd	r3, r2, [r1, #-120]
 8006596: ea86 76e4    	eor.w	r6, r6, r4, asr #31
 800659a: f851 ac08    	ldr	r10, [r1, #-8]
 800659e: ea4f 0445    	lsl.w	r4, r5, #0x1
 80065a2: ea84 74e5    	eor.w	r4, r4, r5, asr #31
 80065a6: eb04 0509    	add.w	r5, r4, r9
 80065aa: ea4f 0443    	lsl.w	r4, r3, #0x1
 80065ae: ea84 73e3    	eor.w	r3, r4, r3, asr #31
 80065b2: f851 4c70    	ldr	r4, [r1, #-112]
 80065b6: 442e         	add	r6, r5
 80065b8: ea4f 0542    	lsl.w	r5, r2, #0x1
 80065bc: 4433         	add	r3, r6
 80065be: ea85 72e2    	eor.w	r2, r5, r2, asr #31
 80065c2: f851 5c6c    	ldr	r5, [r1, #-108]
 80065c6: 441a         	add	r2, r3
 80065c8: ea4f 0344    	lsl.w	r3, r4, #0x1
 80065cc: f851 6c0c    	ldr	r6, [r1, #-12]
 80065d0: ea83 73e4    	eor.w	r3, r3, r4, asr #31
 80065d4: f851 4c68    	ldr	r4, [r1, #-104]
 80065d8: 441a         	add	r2, r3
 80065da: ea4f 0345    	lsl.w	r3, r5, #0x1
 80065de: ea83 73e5    	eor.w	r3, r3, r5, asr #31
 80065e2: f851 5c64    	ldr	r5, [r1, #-100]
 80065e6: 441a         	add	r2, r3
 80065e8: ea4f 0344    	lsl.w	r3, r4, #0x1
 80065ec: ea83 73e4    	eor.w	r3, r3, r4, asr #31
 80065f0: f851 4c60    	ldr	r4, [r1, #-96]
 80065f4: 441a         	add	r2, r3
 80065f6: ea4f 0345    	lsl.w	r3, r5, #0x1
 80065fa: ea83 73e5    	eor.w	r3, r3, r5, asr #31
 80065fe: f851 5c5c    	ldr	r5, [r1, #-92]
 8006602: 441a         	add	r2, r3
 8006604: ea4f 0344    	lsl.w	r3, r4, #0x1
 8006608: ea83 73e4    	eor.w	r3, r3, r4, asr #31
 800660c: f851 4c58    	ldr	r4, [r1, #-88]
 8006610: 441a         	add	r2, r3
 8006612: ea4f 0345    	lsl.w	r3, r5, #0x1
 8006616: ea83 73e5    	eor.w	r3, r3, r5, asr #31
 800661a: f851 5c54    	ldr	r5, [r1, #-84]
 800661e: 441a         	add	r2, r3
 8006620: ea4f 0344    	lsl.w	r3, r4, #0x1
 8006624: ea83 73e4    	eor.w	r3, r3, r4, asr #31
 8006628: f851 4c50    	ldr	r4, [r1, #-80]
 800662c: 441a         	add	r2, r3
 800662e: ea4f 0345    	lsl.w	r3, r5, #0x1
 8006632: ea83 73e5    	eor.w	r3, r3, r5, asr #31
 8006636: f851 5c4c    	ldr	r5, [r1, #-76]
 800663a: 441a         	add	r2, r3
 800663c: ea4f 0344    	lsl.w	r3, r4, #0x1
 8006640: ea83 73e4    	eor.w	r3, r3, r4, asr #31
 8006644: f851 4c48    	ldr	r4, [r1, #-72]
 8006648: 441a         	add	r2, r3
 800664a: ea4f 0345    	lsl.w	r3, r5, #0x1
 800664e: ea83 73e5    	eor.w	r3, r3, r5, asr #31
 8006652: f851 5c44    	ldr	r5, [r1, #-68]
 8006656: 441a         	add	r2, r3
 8006658: ea4f 0344    	lsl.w	r3, r4, #0x1
 800665c: ea83 73e4    	eor.w	r3, r3, r4, asr #31
 8006660: f851 4c40    	ldr	r4, [r1, #-64]
 8006664: 441a         	add	r2, r3
 8006666: ea4f 0345    	lsl.w	r3, r5, #0x1
 800666a: ea83 73e5    	eor.w	r3, r3, r5, asr #31
 800666e: f851 5c3c    	ldr	r5, [r1, #-60]
 8006672: 441a         	add	r2, r3
 8006674: ea4f 0344    	lsl.w	r3, r4, #0x1
 8006678: ea83 73e4    	eor.w	r3, r3, r4, asr #31
 800667c: f851 4c38    	ldr	r4, [r1, #-56]
 8006680: 441a         	add	r2, r3
 8006682: ea4f 0345    	lsl.w	r3, r5, #0x1
 8006686: ea83 73e5    	eor.w	r3, r3, r5, asr #31
 800668a: f851 5c34    	ldr	r5, [r1, #-52]
 800668e: 441a         	add	r2, r3
 8006690: ea4f 0344    	lsl.w	r3, r4, #0x1
 8006694: ea83 73e4    	eor.w	r3, r3, r4, asr #31
 8006698: f851 4c30    	ldr	r4, [r1, #-48]
 800669c: 441a         	add	r2, r3
 800669e: ea4f 0345    	lsl.w	r3, r5, #0x1
 80066a2: ea83 73e5    	eor.w	r3, r3, r5, asr #31
 80066a6: f851 5c2c    	ldr	r5, [r1, #-44]
 80066aa: 441a         	add	r2, r3
 80066ac: ea4f 0344    	lsl.w	r3, r4, #0x1
 80066b0: ea83 73e4    	eor.w	r3, r3, r4, asr #31
 80066b4: f851 4c28    	ldr	r4, [r1, #-40]
 80066b8: 441a         	add	r2, r3
 80066ba: ea4f 0345    	lsl.w	r3, r5, #0x1
 80066be: ea83 73e5    	eor.w	r3, r3, r5, asr #31
 80066c2: f851 5c24    	ldr	r5, [r1, #-36]
 80066c6: 441a         	add	r2, r3
 80066c8: ea4f 0344    	lsl.w	r3, r4, #0x1
 80066cc: ea83 73e4    	eor.w	r3, r3, r4, asr #31
 80066d0: f851 4c20    	ldr	r4, [r1, #-32]
 80066d4: 441a         	add	r2, r3
 80066d6: ea4f 0345    	lsl.w	r3, r5, #0x1
 80066da: ea83 73e5    	eor.w	r3, r3, r5, asr #31
 80066de: f851 5c1c    	ldr	r5, [r1, #-28]
 80066e2: 441a         	add	r2, r3
 80066e4: ea4f 0344    	lsl.w	r3, r4, #0x1
 80066e8: ea83 73e4    	eor.w	r3, r3, r4, asr #31
 80066ec: f851 4c18    	ldr	r4, [r1, #-24]
 80066f0: 441a         	add	r2, r3
 80066f2: ea4f 0345    	lsl.w	r3, r5, #0x1
 80066f6: ea83 73e5    	eor.w	r3, r3, r5, asr #31
 80066fa: f851 5c14    	ldr	r5, [r1, #-20]
 80066fe: 441a         	add	r2, r3
 8006700: ea4f 0344    	lsl.w	r3, r4, #0x1
 8006704: ea83 73e4    	eor.w	r3, r3, r4, asr #31
 8006708: f851 4c10    	ldr	r4, [r1, #-16]
 800670c: 441a         	add	r2, r3
 800670e: ea4f 0345    	lsl.w	r3, r5, #0x1
 8006712: ea83 73e5    	eor.w	r3, r3, r5, asr #31
 8006716: f851 9c04    	ldr	r9, [r1, #-4]
 800671a: 441a         	add	r2, r3
 800671c: ea4f 0344    	lsl.w	r3, r4, #0x1
 8006720: ea83 73e4    	eor.w	r3, r3, r4, asr #31
 8006724: 441a         	add	r2, r3
 8006726: ea4f 0346    	lsl.w	r3, r6, #0x1
 800672a: ea83 73e6    	eor.w	r3, r3, r6, asr #31
 800672e: 441a         	add	r2, r3
 8006730: ea4f 034a    	lsl.w	r3, r10, #0x1
 8006734: ea83 73ea    	eor.w	r3, r3, r10, asr #31
 8006738: 441a         	add	r2, r3
 800673a: ea4f 0349    	lsl.w	r3, r9, #0x1
 800673e: ea83 73e9    	eor.w	r3, r3, r9, asr #31
 8006742: 441a         	add	r2, r3
 8006744: ea4f 034e    	lsl.w	r3, lr, #0x1
 8006748: ea83 73ee    	eor.w	r3, r3, lr, asr #31
 800674c: eb03 0902    	add.w	r9, r3, r2
 8006750: f47f af17    	bne.w	0x8006582 <raylar_crcl_encoder_service::benchmark::fold_sum::h16b920496cf2076e+0x204> @ imm = #-0x1d2
 8006754: f8cd 9008    	str.w	r9, [sp, #0x8]
 8006758: a902         	add	r1, sp, #0x8
 800675a: f04f 0a00    	mov.w	r10, #0x0
 800675e: f240 39ff    	movw	r9, #0x3ff
 8006762: 9902         	ldr	r1, [sp, #0x8]
 8006764: 9100         	str	r1, [sp]
 8006766: f500 31a0    	add.w	r1, r0, #0x14000
 800676a: 311c         	adds	r1, #0x1c
 800676c: f851 8f84    	ldr	r8, [r1, #132]!
 8006770: f1b9 0921    	subs.w	r9, r9, #0x21
 8006774: e951 5620    	ldrd	r5, r6, [r1, #-128]
 8006778: ea4f 0446    	lsl.w	r4, r6, #0x1
 800677c: e951 231e    	ldrd	r2, r3, [r1, #-120]
 8006780: ea84 76e6    	eor.w	r6, r4, r6, asr #31
 8006784: f851 bc08    	ldr	r11, [r1, #-8]
 8006788: ea4f 0445    	lsl.w	r4, r5, #0x1
 800678c: ea84 74e5    	eor.w	r4, r4, r5, asr #31
 8006790: eb04 050a    	add.w	r5, r4, r10
 8006794: ea4f 0442    	lsl.w	r4, r2, #0x1
 8006798: ea84 72e2    	eor.w	r2, r4, r2, asr #31
 800679c: f851 4c70    	ldr	r4, [r1, #-112]
 80067a0: 442e         	add	r6, r5
 80067a2: ea4f 0543    	lsl.w	r5, r3, #0x1
 80067a6: 4432         	add	r2, r6
 80067a8: ea85 73e3    	eor.w	r3, r5, r3, asr #31
 80067ac: f851 5c6c    	ldr	r5, [r1, #-108]
 80067b0: 441a         	add	r2, r3
 80067b2: ea4f 0344    	lsl.w	r3, r4, #0x1
 80067b6: f851 6c0c    	ldr	r6, [r1, #-12]
 80067ba: ea83 73e4    	eor.w	r3, r3, r4, asr #31
 80067be: f851 4c68    	ldr	r4, [r1, #-104]
 80067c2: 441a         	add	r2, r3
 80067c4: ea4f 0345    	lsl.w	r3, r5, #0x1
 80067c8: ea83 73e5    	eor.w	r3, r3, r5, asr #31
 80067cc: f851 5c64    	ldr	r5, [r1, #-100]
 80067d0: 441a         	add	r2, r3
 80067d2: ea4f 0344    	lsl.w	r3, r4, #0x1
 80067d6: ea83 73e4    	eor.w	r3, r3, r4, asr #31
 80067da: f851 4c60    	ldr	r4, [r1, #-96]
 80067de: 441a         	add	r2, r3
 80067e0: ea4f 0345    	lsl.w	r3, r5, #0x1
 80067e4: ea83 73e5    	eor.w	r3, r3, r5, asr #31
 80067e8: f851 5c5c    	ldr	r5, [r1, #-92]
 80067ec: 441a         	add	r2, r3
 80067ee: ea4f 0344    	lsl.w	r3, r4, #0x1
 80067f2: ea83 73e4    	eor.w	r3, r3, r4, asr #31
 80067f6: f851 4c58    	ldr	r4, [r1, #-88]
 80067fa: 441a         	add	r2, r3
 80067fc: ea4f 0345    	lsl.w	r3, r5, #0x1
 8006800: ea83 73e5    	eor.w	r3, r3, r5, asr #31
 8006804: f851 5c54    	ldr	r5, [r1, #-84]
 8006808: 441a         	add	r2, r3
 800680a: ea4f 0344    	lsl.w	r3, r4, #0x1
 800680e: ea83 73e4    	eor.w	r3, r3, r4, asr #31
 8006812: f851 4c50    	ldr	r4, [r1, #-80]
 8006816: 441a         	add	r2, r3
 8006818: ea4f 0345    	lsl.w	r3, r5, #0x1
 800681c: ea83 73e5    	eor.w	r3, r3, r5, asr #31
 8006820: f851 5c4c    	ldr	r5, [r1, #-76]
 8006824: 441a         	add	r2, r3
 8006826: ea4f 0344    	lsl.w	r3, r4, #0x1
 800682a: ea83 73e4    	eor.w	r3, r3, r4, asr #31
 800682e: f851 4c48    	ldr	r4, [r1, #-72]
 8006832: 441a         	add	r2, r3
 8006834: ea4f 0345    	lsl.w	r3, r5, #0x1
 8006838: ea83 73e5    	eor.w	r3, r3, r5, asr #31
 800683c: f851 5c44    	ldr	r5, [r1, #-68]
 8006840: 441a         	add	r2, r3
 8006842: ea4f 0344    	lsl.w	r3, r4, #0x1
 8006846: ea83 73e4    	eor.w	r3, r3, r4, asr #31
 800684a: f851 4c40    	ldr	r4, [r1, #-64]
 800684e: 441a         	add	r2, r3
 8006850: ea4f 0345    	lsl.w	r3, r5, #0x1
 8006854: ea83 73e5    	eor.w	r3, r3, r5, asr #31
 8006858: f851 5c3c    	ldr	r5, [r1, #-60]
 800685c: 441a         	add	r2, r3
 800685e: ea4f 0344    	lsl.w	r3, r4, #0x1
 8006862: ea83 73e4    	eor.w	r3, r3, r4, asr #31
 8006866: f851 4c38    	ldr	r4, [r1, #-56]
 800686a: 441a         	add	r2, r3
 800686c: ea4f 0345    	lsl.w	r3, r5, #0x1
 8006870: ea83 73e5    	eor.w	r3, r3, r5, asr #31
 8006874: f851 5c34    	ldr	r5, [r1, #-52]
 8006878: 441a         	add	r2, r3
 800687a: ea4f 0344    	lsl.w	r3, r4, #0x1
 800687e: ea83 73e4    	eor.w	r3, r3, r4, asr #31
 8006882: f851 4c30    	ldr	r4, [r1, #-48]
 8006886: 441a         	add	r2, r3
 8006888: ea4f 0345    	lsl.w	r3, r5, #0x1
 800688c: ea83 73e5    	eor.w	r3, r3, r5, asr #31
 8006890: f851 5c2c    	ldr	r5, [r1, #-44]
 8006894: 441a         	add	r2, r3
 8006896: ea4f 0344    	lsl.w	r3, r4, #0x1
 800689a: ea83 73e4    	eor.w	r3, r3, r4, asr #31
 800689e: f851 4c28    	ldr	r4, [r1, #-40]
 80068a2: 441a         	add	r2, r3
 80068a4: ea4f 0345    	lsl.w	r3, r5, #0x1
 80068a8: ea83 73e5    	eor.w	r3, r3, r5, asr #31
 80068ac: f851 5c24    	ldr	r5, [r1, #-36]
 80068b0: 441a         	add	r2, r3
 80068b2: ea4f 0344    	lsl.w	r3, r4, #0x1
 80068b6: ea83 73e4    	eor.w	r3, r3, r4, asr #31
 80068ba: f851 4c20    	ldr	r4, [r1, #-32]
 80068be: 441a         	add	r2, r3
 80068c0: ea4f 0345    	lsl.w	r3, r5, #0x1
 80068c4: ea83 73e5    	eor.w	r3, r3, r5, asr #31
 80068c8: f851 5c1c    	ldr	r5, [r1, #-28]
 80068cc: 441a         	add	r2, r3
 80068ce: ea4f 0344    	lsl.w	r3, r4, #0x1
 80068d2: ea83 73e4    	eor.w	r3, r3, r4, asr #31
 80068d6: f851 4c18    	ldr	r4, [r1, #-24]
 80068da: 441a         	add	r2, r3
 80068dc: ea4f 0345    	lsl.w	r3, r5, #0x1
 80068e0: ea83 73e5    	eor.w	r3, r3, r5, asr #31
 80068e4: f851 5c14    	ldr	r5, [r1, #-20]
 80068e8: 441a         	add	r2, r3
 80068ea: ea4f 0344    	lsl.w	r3, r4, #0x1
 80068ee: ea83 73e4    	eor.w	r3, r3, r4, asr #31
 80068f2: f851 4c10    	ldr	r4, [r1, #-16]
 80068f6: 441a         	add	r2, r3
 80068f8: ea4f 0345    	lsl.w	r3, r5, #0x1
 80068fc: ea83 73e5    	eor.w	r3, r3, r5, asr #31
 8006900: f851 ac04    	ldr	r10, [r1, #-4]
 8006904: 441a         	add	r2, r3
 8006906: ea4f 0344    	lsl.w	r3, r4, #0x1
 800690a: ea83 73e4    	eor.w	r3, r3, r4, asr #31
 800690e: 441a         	add	r2, r3
 8006910: ea4f 0346    	lsl.w	r3, r6, #0x1
 8006914: ea83 73e6    	eor.w	r3, r3, r6, asr #31
 8006918: 441a         	add	r2, r3
 800691a: ea4f 034b    	lsl.w	r3, r11, #0x1
 800691e: ea83 73eb    	eor.w	r3, r3, r11, asr #31
 8006922: 441a         	add	r2, r3
 8006924: ea4f 034a    	lsl.w	r3, r10, #0x1
 8006928: ea83 73ea    	eor.w	r3, r3, r10, asr #31
 800692c: 441a         	add	r2, r3
 800692e: ea4f 0348    	lsl.w	r3, r8, #0x1
 8006932: ea83 73e8    	eor.w	r3, r3, r8, asr #31
 8006936: eb03 0a02    	add.w	r10, r3, r2
 800693a: f47f af17    	bne.w	0x800676c <raylar_crcl_encoder_service::benchmark::fold_sum::h16b920496cf2076e+0x3ee> @ imm = #-0x1d2
 800693e: f8cd a008    	str.w	r10, [sp, #0x8]
 8006942: a902         	add	r1, sp, #0x8
 8006944: f500 31a8    	add.w	r1, r0, #0x15000
 8006948: f04f 0b00    	mov.w	r11, #0x0
 800694c: f8dd 8008    	ldr.w	r8, [sp, #0x8]
 8006950: 3118         	adds	r1, #0x18
 8006952: f240 3aff    	movw	r10, #0x3ff
 8006956: f851 9f84    	ldr	r9, [r1, #132]!
 800695a: f1ba 0a21    	subs.w	r10, r10, #0x21
 800695e: e951 5620    	ldrd	r5, r6, [r1, #-128]
 8006962: e951 421e    	ldrd	r4, r2, [r1, #-120]
 8006966: ea4f 0346    	lsl.w	r3, r6, #0x1
 800696a: ea83 73e6    	eor.w	r3, r3, r6, asr #31
 800696e: ea4f 0c42    	lsl.w	r12, r2, #0x1
 8006972: ea4f 0645    	lsl.w	r6, r5, #0x1
 8006976: ea86 75e5    	eor.w	r5, r6, r5, asr #31
 800697a: eb05 060b    	add.w	r6, r5, r11
 800697e: ea4f 0544    	lsl.w	r5, r4, #0x1
 8006982: ea85 74e4    	eor.w	r4, r5, r4, asr #31
 8006986: f851 5c70    	ldr	r5, [r1, #-112]
 800698a: 4433         	add	r3, r6
 800698c: ea8c 7ce2    	eor.w	r12, r12, r2, asr #31
 8006990: f851 2c6c    	ldr	r2, [r1, #-108]
 8006994: 441c         	add	r4, r3
 8006996: ea4f 0645    	lsl.w	r6, r5, #0x1
 800699a: 4464         	add	r4, r12
 800699c: ea86 76e5    	eor.w	r6, r6, r5, asr #31
 80069a0: f851 5c68    	ldr	r5, [r1, #-104]
 80069a4: 4426         	add	r6, r4
 80069a6: ea4f 0442    	lsl.w	r4, r2, #0x1
 80069aa: ea84 72e2    	eor.w	r2, r4, r2, asr #31
 80069ae: f851 4c64    	ldr	r4, [r1, #-100]
 80069b2: 4432         	add	r2, r6
 80069b4: ea4f 0645    	lsl.w	r6, r5, #0x1
 80069b8: ea86 76e5    	eor.w	r6, r6, r5, asr #31
 80069bc: f851 5c60    	ldr	r5, [r1, #-96]
 80069c0: 4432         	add	r2, r6
 80069c2: ea4f 0644    	lsl.w	r6, r4, #0x1
 80069c6: ea86 76e4    	eor.w	r6, r6, r4, asr #31
 80069ca: f851 4c5c    	ldr	r4, [r1, #-92]
 80069ce: 4432         	add	r2, r6
 80069d0: ea4f 0645    	lsl.w	r6, r5, #0x1
 80069d4: ea86 76e5    	eor.w	r6, r6, r5, asr #31
 80069d8: f851 5c58    	ldr	r5, [r1, #-88]
 80069dc: 4432         	add	r2, r6
 80069de: ea4f 0644    	lsl.w	r6, r4, #0x1
 80069e2: ea86 76e4    	eor.w	r6, r6, r4, asr #31
 80069e6: f851 4c54    	ldr	r4, [r1, #-84]
 80069ea: 4432         	add	r2, r6
 80069ec: ea4f 0645    	lsl.w	r6, r5, #0x1
 80069f0: ea86 76e5    	eor.w	r6, r6, r5, asr #31
 80069f4: f851 5c50    	ldr	r5, [r1, #-80]
 80069f8: 4432         	add	r2, r6
 80069fa: ea4f 0644    	lsl.w	r6, r4, #0x1
 80069fe: ea86 76e4    	eor.w	r6, r6, r4, asr #31
 8006a02: f851 4c4c    	ldr	r4, [r1, #-76]
 8006a06: 4432         	add	r2, r6
 8006a08: ea4f 0645    	lsl.w	r6, r5, #0x1
 8006a0c: ea86 76e5    	eor.w	r6, r6, r5, asr #31
 8006a10: f851 5c48    	ldr	r5, [r1, #-72]
 8006a14: 4432         	add	r2, r6
 8006a16: ea4f 0644    	lsl.w	r6, r4, #0x1
 8006a1a: ea86 76e4    	eor.w	r6, r6, r4, asr #31
 8006a1e: f851 4c44    	ldr	r4, [r1, #-68]
 8006a22: 4432         	add	r2, r6
 8006a24: ea4f 0645    	lsl.w	r6, r5, #0x1
 8006a28: ea86 76e5    	eor.w	r6, r6, r5, asr #31
 8006a2c: f851 5c40    	ldr	r5, [r1, #-64]
 8006a30: 4432         	add	r2, r6
 8006a32: ea4f 0644    	lsl.w	r6, r4, #0x1
 8006a36: ea86 76e4    	eor.w	r6, r6, r4, asr #31
 8006a3a: f851 4c3c    	ldr	r4, [r1, #-60]
 8006a3e: 4432         	add	r2, r6
 8006a40: ea4f 0645    	lsl.w	r6, r5, #0x1
 8006a44: ea86 76e5    	eor.w	r6, r6, r5, asr #31
 8006a48: f851 5c38    	ldr	r5, [r1, #-56]
 8006a4c: 4432         	add	r2, r6
 8006a4e: ea4f 0644    	lsl.w	r6, r4, #0x1
 8006a52: ea86 76e4    	eor.w	r6, r6, r4, asr #31
 8006a56: f851 4c34    	ldr	r4, [r1, #-52]
 8006a5a: 4432         	add	r2, r6
 8006a5c: ea4f 0645    	lsl.w	r6, r5, #0x1
 8006a60: ea86 76e5    	eor.w	r6, r6, r5, asr #31
 8006a64: f851 5c30    	ldr	r5, [r1, #-48]
 8006a68: 4432         	add	r2, r6
 8006a6a: ea4f 0644    	lsl.w	r6, r4, #0x1
 8006a6e: ea86 76e4    	eor.w	r6, r6, r4, asr #31
 8006a72: f851 4c2c    	ldr	r4, [r1, #-44]
 8006a76: 4432         	add	r2, r6
 8006a78: ea4f 0645    	lsl.w	r6, r5, #0x1
 8006a7c: ea86 76e5    	eor.w	r6, r6, r5, asr #31
 8006a80: f851 5c28    	ldr	r5, [r1, #-40]
 8006a84: 4432         	add	r2, r6
 8006a86: ea4f 0644    	lsl.w	r6, r4, #0x1
 8006a8a: ea86 76e4    	eor.w	r6, r6, r4, asr #31
 8006a8e: f851 4c24    	ldr	r4, [r1, #-36]
 8006a92: 4432         	add	r2, r6
 8006a94: ea4f 0645    	lsl.w	r6, r5, #0x1
 8006a98: ea86 76e5    	eor.w	r6, r6, r5, asr #31
 8006a9c: f851 5c20    	ldr	r5, [r1, #-32]
 8006aa0: 4432         	add	r2, r6
 8006aa2: ea4f 0644    	lsl.w	r6, r4, #0x1
 8006aa6: ea86 76e4    	eor.w	r6, r6, r4, asr #31
 8006aaa: f851 4c1c    	ldr	r4, [r1, #-28]
 8006aae: 4432         	add	r2, r6
 8006ab0: ea4f 0645    	lsl.w	r6, r5, #0x1
 8006ab4: ea86 76e5    	eor.w	r6, r6, r5, asr #31
 8006ab8: f851 5c18    	ldr	r5, [r1, #-24]
 8006abc: 4432         	add	r2, r6
 8006abe: ea4f 0644    	lsl.w	r6, r4, #0x1
 8006ac2: ea86 76e4    	eor.w	r6, r6, r4, asr #31
 8006ac6: f851 4c14    	ldr	r4, [r1, #-20]
 8006aca: 4432         	add	r2, r6
 8006acc: ea4f 0645    	lsl.w	r6, r5, #0x1
 8006ad0: ea86 76e5    	eor.w	r6, r6, r5, asr #31
 8006ad4: f851 5c10    	ldr	r5, [r1, #-16]
 8006ad8: 4432         	add	r2, r6
 8006ada: ea4f 0644    	lsl.w	r6, r4, #0x1
 8006ade: ea86 76e4    	eor.w	r6, r6, r4, asr #31
 8006ae2: f851 3c0c    	ldr	r3, [r1, #-12]
 8006ae6: 4432         	add	r2, r6
 8006ae8: ea4f 0645    	lsl.w	r6, r5, #0x1
 8006aec: f851 ec08    	ldr	lr, [r1, #-8]
 8006af0: ea86 76e5    	eor.w	r6, r6, r5, asr #31
 8006af4: 4432         	add	r2, r6
 8006af6: ea4f 0643    	lsl.w	r6, r3, #0x1
 8006afa: ea86 73e3    	eor.w	r3, r6, r3, asr #31
 8006afe: f851 bc04    	ldr	r11, [r1, #-4]
 8006b02: 441a         	add	r2, r3
 8006b04: ea4f 034e    	lsl.w	r3, lr, #0x1
 8006b08: ea83 73ee    	eor.w	r3, r3, lr, asr #31
 8006b0c: 441a         	add	r2, r3
 8006b0e: ea4f 034b    	lsl.w	r3, r11, #0x1
 8006b12: ea83 73eb    	eor.w	r3, r3, r11, asr #31
 8006b16: 441a         	add	r2, r3
 8006b18: ea4f 0349    	lsl.w	r3, r9, #0x1
 8006b1c: ea83 73e9    	eor.w	r3, r3, r9, asr #31
 8006b20: eb03 0b02    	add.w	r11, r3, r2
 8006b24: f47f af17    	bne.w	0x8006956 <raylar_crcl_encoder_service::benchmark::fold_sum::h16b920496cf2076e+0x5d8> @ imm = #-0x1d2
 8006b28: f8cd b008    	str.w	r11, [sp, #0x8]
 8006b2c: a902         	add	r1, sp, #0x8
 8006b2e: f500 30b0    	add.w	r0, r0, #0x16000
 8006b32: 2300         	movs	r3, #0x0
 8006b34: f8dd c008    	ldr.w	r12, [sp, #0x8]
 8006b38: 3014         	adds	r0, #0x14
 8006b3a: f240 39ff    	movw	r9, #0x3ff
 8006b3e: f850 ef84    	ldr	lr, [r0, #132]!
 8006b42: f1b9 0921    	subs.w	r9, r9, #0x21
 8006b46: e950 6520    	ldrd	r6, r5, [r0, #-128]
 8006b4a: ea4f 0245    	lsl.w	r2, r5, #0x1
 8006b4e: e950 141e    	ldrd	r1, r4, [r0, #-120]
 8006b52: ea82 72e5    	eor.w	r2, r2, r5, asr #31
 8006b56: f850 ac04    	ldr	r10, [r0, #-4]
 8006b5a: ea4f 0546    	lsl.w	r5, r6, #0x1
 8006b5e: ea85 75e6    	eor.w	r5, r5, r6, asr #31
 8006b62: f850 6c08    	ldr	r6, [r0, #-8]
 8006b66: 442b         	add	r3, r5
 8006b68: f850 5c70    	ldr	r5, [r0, #-112]
 8006b6c: 441a         	add	r2, r3
 8006b6e: ea4f 0341    	lsl.w	r3, r1, #0x1
 8006b72: ea83 71e1    	eor.w	r1, r3, r1, asr #31
 8006b76: f850 3c0c    	ldr	r3, [r0, #-12]
 8006b7a: 4411         	add	r1, r2
 8006b7c: ea4f 0244    	lsl.w	r2, r4, #0x1
 8006b80: ea82 72e4    	eor.w	r2, r2, r4, asr #31
 8006b84: f850 4c6c    	ldr	r4, [r0, #-108]
 8006b88: 4411         	add	r1, r2
 8006b8a: ea4f 0245    	lsl.w	r2, r5, #0x1
 8006b8e: ea82 72e5    	eor.w	r2, r2, r5, asr #31
 8006b92: f850 5c68    	ldr	r5, [r0, #-104]
 8006b96: 4411         	add	r1, r2
 8006b98: ea4f 0244    	lsl.w	r2, r4, #0x1
 8006b9c: ea82 72e4    	eor.w	r2, r2, r4, asr #31
 8006ba0: f850 4c64    	ldr	r4, [r0, #-100]
 8006ba4: 4411         	add	r1, r2
 8006ba6: ea4f 0245    	lsl.w	r2, r5, #0x1
 8006baa: ea82 72e5    	eor.w	r2, r2, r5, asr #31
 8006bae: f850 5c60    	ldr	r5, [r0, #-96]
 8006bb2: 4411         	add	r1, r2
 8006bb4: ea4f 0244    	lsl.w	r2, r4, #0x1
 8006bb8: ea82 72e4    	eor.w	r2, r2, r4, asr #31
 8006bbc: f850 4c5c    	ldr	r4, [r0, #-92]
 8006bc0: 4411         	add	r1, r2
 8006bc2: ea4f 0245    	lsl.w	r2, r5, #0x1
 8006bc6: ea82 72e5    	eor.w	r2, r2, r5, asr #31
 8006bca: f850 5c58    	ldr	r5, [r0, #-88]
 8006bce: 4411         	add	r1, r2
 8006bd0: ea4f 0244    	lsl.w	r2, r4, #0x1
 8006bd4: ea82 72e4    	eor.w	r2, r2, r4, asr #31
 8006bd8: f850 4c54    	ldr	r4, [r0, #-84]
 8006bdc: 4411         	add	r1, r2
 8006bde: ea4f 0245    	lsl.w	r2, r5, #0x1
 8006be2: ea82 72e5    	eor.w	r2, r2, r5, asr #31
 8006be6: f850 5c50    	ldr	r5, [r0, #-80]
 8006bea: 4411         	add	r1, r2
 8006bec: ea4f 0244    	lsl.w	r2, r4, #0x1
 8006bf0: ea82 72e4    	eor.w	r2, r2, r4, asr #31
 8006bf4: f850 4c4c    	ldr	r4, [r0, #-76]
 8006bf8: 4411         	add	r1, r2
 8006bfa: ea4f 0245    	lsl.w	r2, r5, #0x1
 8006bfe: ea82 72e5    	eor.w	r2, r2, r5, asr #31
 8006c02: f850 5c48    	ldr	r5, [r0, #-72]
 8006c06: 4411         	add	r1, r2
 8006c08: ea4f 0244    	lsl.w	r2, r4, #0x1
 8006c0c: ea82 72e4    	eor.w	r2, r2, r4, asr #31
 8006c10: f850 4c44    	ldr	r4, [r0, #-68]
 8006c14: 4411         	add	r1, r2
 8006c16: ea4f 0245    	lsl.w	r2, r5, #0x1
 8006c1a: ea82 72e5    	eor.w	r2, r2, r5, asr #31
 8006c1e: f850 5c40    	ldr	r5, [r0, #-64]
 8006c22: 4411         	add	r1, r2
 8006c24: ea4f 0244    	lsl.w	r2, r4, #0x1
 8006c28: ea82 72e4    	eor.w	r2, r2, r4, asr #31
 8006c2c: f850 4c3c    	ldr	r4, [r0, #-60]
 8006c30: 4411         	add	r1, r2
 8006c32: ea4f 0245    	lsl.w	r2, r5, #0x1
 8006c36: ea82 72e5    	eor.w	r2, r2, r5, asr #31
 8006c3a: f850 5c38    	ldr	r5, [r0, #-56]
 8006c3e: 4411         	add	r1, r2
 8006c40: ea4f 0244    	lsl.w	r2, r4, #0x1
 8006c44: ea82 72e4    	eor.w	r2, r2, r4, asr #31
 8006c48: f850 4c34    	ldr	r4, [r0, #-52]
 8006c4c: 4411         	add	r1, r2
 8006c4e: ea4f 0245    	lsl.w	r2, r5, #0x1
 8006c52: ea82 72e5    	eor.w	r2, r2, r5, asr #31
 8006c56: f850 5c30    	ldr	r5, [r0, #-48]
 8006c5a: 4411         	add	r1, r2
 8006c5c: ea4f 0244    	lsl.w	r2, r4, #0x1
 8006c60: ea82 72e4    	eor.w	r2, r2, r4, asr #31
 8006c64: f850 4c2c    	ldr	r4, [r0, #-44]
 8006c68: 4411         	add	r1, r2
 8006c6a: ea4f 0245    	lsl.w	r2, r5, #0x1
 8006c6e: ea82 72e5    	eor.w	r2, r2, r5, asr #31
 8006c72: f850 5c28    	ldr	r5, [r0, #-40]
 8006c76: 4411         	add	r1, r2
 8006c78: ea4f 0244    	lsl.w	r2, r4, #0x1
 8006c7c: ea82 72e4    	eor.w	r2, r2, r4, asr #31
 8006c80: f850 4c24    	ldr	r4, [r0, #-36]
 8006c84: 4411         	add	r1, r2
 8006c86: ea4f 0245    	lsl.w	r2, r5, #0x1
 8006c8a: ea82 72e5    	eor.w	r2, r2, r5, asr #31
 8006c8e: f850 5c20    	ldr	r5, [r0, #-32]
 8006c92: 4411         	add	r1, r2
 8006c94: ea4f 0244    	lsl.w	r2, r4, #0x1
 8006c98: ea82 72e4    	eor.w	r2, r2, r4, asr #31
 8006c9c: f850 4c1c    	ldr	r4, [r0, #-28]
 8006ca0: 4411         	add	r1, r2
 8006ca2: ea4f 0245    	lsl.w	r2, r5, #0x1
 8006ca6: ea82 72e5    	eor.w	r2, r2, r5, asr #31
 8006caa: f850 5c18    	ldr	r5, [r0, #-24]
 8006cae: 4411         	add	r1, r2
 8006cb0: ea4f 0244    	lsl.w	r2, r4, #0x1
 8006cb4: ea82 72e4    	eor.w	r2, r2, r4, asr #31
 8006cb8: f850 4c14    	ldr	r4, [r0, #-20]
 8006cbc: 4411         	add	r1, r2
 8006cbe: ea4f 0245    	lsl.w	r2, r5, #0x1
 8006cc2: ea82 72e5    	eor.w	r2, r2, r5, asr #31
 8006cc6: f850 5c10    	ldr	r5, [r0, #-16]
 8006cca: 4411         	add	r1, r2
 8006ccc: ea4f 0244    	lsl.w	r2, r4, #0x1
 8006cd0: ea82 72e4    	eor.w	r2, r2, r4, asr #31
 8006cd4: 4411         	add	r1, r2
 8006cd6: ea4f 0245    	lsl.w	r2, r5, #0x1
 8006cda: ea82 72e5    	eor.w	r2, r2, r5, asr #31
 8006cde: 4411         	add	r1, r2
 8006ce0: ea4f 0243    	lsl.w	r2, r3, #0x1
 8006ce4: ea82 72e3    	eor.w	r2, r2, r3, asr #31
 8006ce8: 4411         	add	r1, r2
 8006cea: ea4f 0246    	lsl.w	r2, r6, #0x1
 8006cee: ea82 72e6    	eor.w	r2, r2, r6, asr #31
 8006cf2: 4411         	add	r1, r2
 8006cf4: ea4f 024a    	lsl.w	r2, r10, #0x1
 8006cf8: ea82 72ea    	eor.w	r2, r2, r10, asr #31
 8006cfc: 4411         	add	r1, r2
 8006cfe: ea4f 024e    	lsl.w	r2, lr, #0x1
 8006d02: ea82 72ee    	eor.w	r2, r2, lr, asr #31
 8006d06: eb02 0301    	add.w	r3, r2, r1
 8006d0a: f47f af18    	bne.w	0x8006b3e <raylar_crcl_encoder_service::benchmark::fold_sum::h16b920496cf2076e+0x7c0> @ imm = #-0x1d0
 8006d0e: 9302         	str	r3, [sp, #0x8]
 8006d10: a802         	add	r0, sp, #0x8
 8006d12: e9dd 1000    	ldrd	r1, r0, [sp]
 8006d16: 4048         	eors	r0, r1
 8006d18: 9902         	ldr	r1, [sp, #0x8]
 8006d1a: ea80 0008    	eor.w	r0, r0, r8
 8006d1e: ea80 000c    	eor.w	r0, r0, r12
 8006d22: 4048         	eors	r0, r1
 8006d24: b003         	add	sp, #0xc
 8006d26: e8bd 0f00    	pop.w	{r8, r9, r10, r11}
 8006d2a: bdf0         	pop	{r4, r5, r6, r7, pc}

08006d2c <raylar_crcl_encoder_service::benchmark::rounding::h32a8b1c6cb68e077>:
 8006d2c: b5f0         	push	{r4, r5, r6, r7, lr}
 8006d2e: af03         	add	r7, sp, #0xc
 8006d30: e92d 0b00    	push.w	{r8, r9, r11}
 8006d34: b082         	sub	sp, #0x8
 8006d36: 6801         	ldr	r1, [r0]
 8006d38: 9101         	str	r1, [sp, #0x4]
 8006d3a: a901         	add	r1, sp, #0x4
 8006d3c: f645 716c    	movw	r1, #0x5f6c
 8006d40: 9b01         	ldr	r3, [sp, #0x4]
 8006d42: 4401         	add	r1, r0
 8006d44: 1e5a         	subs	r2, r3, #0x1
 8006d46: f003 031f    	and	r3, r3, #0x1f
 8006d4a: f002 0c1f    	and	r12, r2, #0x1f
 8006d4e: 2201         	movs	r2, #0x1
 8006d50: fa02 f20c    	lsl.w	r2, r2, r12
 8006d54: f44f 6c80    	mov.w	r12, #0x400
 8006d58: f8d1 40d8    	ldr.w	r4, [r1, #0xd8]
 8006d5c: f1bc 0c10    	subs.w	r12, r12, #0x10
 8006d60: f8d1 50f0    	ldr.w	r5, [r1, #0xf0]
 8006d64: 4414         	add	r4, r2
 8006d66: f8d1 8108    	ldr.w	r8, [r1, #0x108]
 8006d6a: f8d1 60c0    	ldr.w	r6, [r1, #0xc0]
 8006d6e: fa44 f403    	asr.w	r4, r4, r3
 8006d72: f8d1 e198    	ldr.w	lr, [r1, #0x198]
 8006d76: f8c1 40d8    	str.w	r4, [r1, #0xd8]
 8006d7a: eb05 0402    	add.w	r4, r5, r2
 8006d7e: 4416         	add	r6, r2
 8006d80: f8d1 5120    	ldr.w	r5, [r1, #0x120]
 8006d84: fa44 f403    	asr.w	r4, r4, r3
 8006d88: f8d1 9180    	ldr.w	r9, [r1, #0x180]
 8006d8c: f8c1 40f0    	str.w	r4, [r1, #0xf0]
 8006d90: eb08 0402    	add.w	r4, r8, r2
 8006d94: fa46 f603    	asr.w	r6, r6, r3
 8006d98: fa44 f403    	asr.w	r4, r4, r3
 8006d9c: f8c1 60c0    	str.w	r6, [r1, #0xc0]
 8006da0: f8c1 4108    	str.w	r4, [r1, #0x108]
 8006da4: eb05 0402    	add.w	r4, r5, r2
 8006da8: f8d1 6138    	ldr.w	r6, [r1, #0x138]
 8006dac: fa44 f403    	asr.w	r4, r4, r3
 8006db0: f8d1 5150    	ldr.w	r5, [r1, #0x150]
 8006db4: f8c1 4120    	str.w	r4, [r1, #0x120]
 8006db8: eb06 0402    	add.w	r4, r6, r2
 8006dbc: f8d1 6168    	ldr.w	r6, [r1, #0x168]
 8006dc0: fa44 f403    	asr.w	r4, r4, r3
 8006dc4: f8c1 4138    	str.w	r4, [r1, #0x138]
 8006dc8: eb05 0402    	add.w	r4, r5, r2
 8006dcc: fa44 f403    	asr.w	r4, r4, r3
 8006dd0: f8c1 4150    	str.w	r4, [r1, #0x150]
 8006dd4: eb06 0402    	add.w	r4, r6, r2
 8006dd8: eb0e 0602    	add.w	r6, lr, r2
 8006ddc: fa44 f403    	asr.w	r4, r4, r3
 8006de0: fa46 f603    	asr.w	r6, r6, r3
 8006de4: f8c1 4168    	str.w	r4, [r1, #0x168]
 8006de8: f8c1 6198    	str.w	r6, [r1, #0x198]
 8006dec: eb09 0402    	add.w	r4, r9, r2
 8006df0: f8d1 61b0    	ldr.w	r6, [r1, #0x1b0]
 8006df4: fa44 f403    	asr.w	r4, r4, r3
 8006df8: 4416         	add	r6, r2
 8006dfa: f8c1 4180    	str.w	r4, [r1, #0x180]
 8006dfe: fa46 f603    	asr.w	r6, r6, r3
 8006e02: f8c1 61b0    	str.w	r6, [r1, #0x1b0]
 8006e06: f8d1 61c8    	ldr.w	r6, [r1, #0x1c8]
 8006e0a: 4416         	add	r6, r2
 8006e0c: fa46 f603    	asr.w	r6, r6, r3
 8006e10: f8c1 61c8    	str.w	r6, [r1, #0x1c8]
 8006e14: f8d1 61e0    	ldr.w	r6, [r1, #0x1e0]
 8006e18: 4416         	add	r6, r2
 8006e1a: fa46 f603    	asr.w	r6, r6, r3
 8006e1e: f8c1 61e0    	str.w	r6, [r1, #0x1e0]
 8006e22: f8d1 61f8    	ldr.w	r6, [r1, #0x1f8]
 8006e26: 4416         	add	r6, r2
 8006e28: fa46 f603    	asr.w	r6, r6, r3
 8006e2c: f8c1 61f8    	str.w	r6, [r1, #0x1f8]
 8006e30: f8d1 6210    	ldr.w	r6, [r1, #0x210]
 8006e34: 4416         	add	r6, r2
 8006e36: fa46 f603    	asr.w	r6, r6, r3
 8006e3a: f8c1 6210    	str.w	r6, [r1, #0x210]
 8006e3e: f8d1 6228    	ldr.w	r6, [r1, #0x228]
 8006e42: 4416         	add	r6, r2
 8006e44: fa46 f603    	asr.w	r6, r6, r3
 8006e48: f8c1 6228    	str.w	r6, [r1, #0x228]
 8006e4c: f501 71c0    	add.w	r1, r1, #0x180
 8006e50: d182         	bne	0x8006d58 <raylar_crcl_encoder_service::benchmark::rounding::h32a8b1c6cb68e077+0x2c> @ imm = #-0xfc
 8006e52: 6881         	ldr	r1, [r0, #0x8]
 8006e54: 9101         	str	r1, [sp, #0x4]
 8006e56: a901         	add	r1, sp, #0x4
 8006e58: f645 61b0    	movw	r1, #0x5eb0
 8006e5c: 9b01         	ldr	r3, [sp, #0x4]
 8006e5e: 4401         	add	r1, r0
 8006e60: 1e5a         	subs	r2, r3, #0x1
 8006e62: f003 031f    	and	r3, r3, #0x1f
 8006e66: f002 0c1f    	and	r12, r2, #0x1f
 8006e6a: 2201         	movs	r2, #0x1
 8006e6c: fa02 f20c    	lsl.w	r2, r2, r12
 8006e70: f44f 6c80    	mov.w	r12, #0x400
 8006e74: f8d1 4198    	ldr.w	r4, [r1, #0x198]
 8006e78: f1bc 0c10    	subs.w	r12, r12, #0x10
 8006e7c: f8d1 51b0    	ldr.w	r5, [r1, #0x1b0]
 8006e80: 4414         	add	r4, r2
 8006e82: f8d1 81c8    	ldr.w	r8, [r1, #0x1c8]
 8006e86: f8d1 e180    	ldr.w	lr, [r1, #0x180]
 8006e8a: fa44 f403    	asr.w	r4, r4, r3
 8006e8e: f8d1 9240    	ldr.w	r9, [r1, #0x240]
 8006e92: f8c1 4198    	str.w	r4, [r1, #0x198]
 8006e96: eb05 0402    	add.w	r4, r5, r2
 8006e9a: eb0e 0602    	add.w	r6, lr, r2
 8006e9e: f8d1 51e0    	ldr.w	r5, [r1, #0x1e0]
 8006ea2: fa44 f403    	asr.w	r4, r4, r3
 8006ea6: f8d1 e258    	ldr.w	lr, [r1, #0x258]
 8006eaa: f8c1 41b0    	str.w	r4, [r1, #0x1b0]
 8006eae: eb08 0402    	add.w	r4, r8, r2
 8006eb2: fa46 f603    	asr.w	r6, r6, r3
 8006eb6: fa44 f403    	asr.w	r4, r4, r3
 8006eba: f8c1 6180    	str.w	r6, [r1, #0x180]
 8006ebe: f8c1 41c8    	str.w	r4, [r1, #0x1c8]
 8006ec2: eb05 0402    	add.w	r4, r5, r2
 8006ec6: f8d1 61f8    	ldr.w	r6, [r1, #0x1f8]
 8006eca: fa44 f403    	asr.w	r4, r4, r3
 8006ece: f8d1 5210    	ldr.w	r5, [r1, #0x210]
 8006ed2: f8c1 41e0    	str.w	r4, [r1, #0x1e0]
 8006ed6: eb06 0402    	add.w	r4, r6, r2
 8006eda: f8d1 6228    	ldr.w	r6, [r1, #0x228]
 8006ede: fa44 f403    	asr.w	r4, r4, r3
 8006ee2: f8c1 41f8    	str.w	r4, [r1, #0x1f8]
 8006ee6: eb05 0402    	add.w	r4, r5, r2
 8006eea: fa44 f403    	asr.w	r4, r4, r3
 8006eee: f8c1 4210    	str.w	r4, [r1, #0x210]
 8006ef2: eb06 0402    	add.w	r4, r6, r2
 8006ef6: eb0e 0602    	add.w	r6, lr, r2
 8006efa: fa44 f403    	asr.w	r4, r4, r3
 8006efe: fa46 f603    	asr.w	r6, r6, r3
 8006f02: f8c1 4228    	str.w	r4, [r1, #0x228]
 8006f06: f8c1 6258    	str.w	r6, [r1, #0x258]
 8006f0a: eb09 0402    	add.w	r4, r9, r2
 8006f0e: f8d1 6270    	ldr.w	r6, [r1, #0x270]
 8006f12: fa44 f403    	asr.w	r4, r4, r3
 8006f16: 4416         	add	r6, r2
 8006f18: f8c1 4240    	str.w	r4, [r1, #0x240]
 8006f1c: fa46 f603    	asr.w	r6, r6, r3
 8006f20: f8c1 6270    	str.w	r6, [r1, #0x270]
 8006f24: f8d1 6288    	ldr.w	r6, [r1, #0x288]
 8006f28: 4416         	add	r6, r2
 8006f2a: fa46 f603    	asr.w	r6, r6, r3
 8006f2e: f8c1 6288    	str.w	r6, [r1, #0x288]
 8006f32: f8d1 62a0    	ldr.w	r6, [r1, #0x2a0]
 8006f36: 4416         	add	r6, r2
 8006f38: fa46 f603    	asr.w	r6, r6, r3
 8006f3c: f8c1 62a0    	str.w	r6, [r1, #0x2a0]
 8006f40: f8d1 62b8    	ldr.w	r6, [r1, #0x2b8]
 8006f44: 4416         	add	r6, r2
 8006f46: fa46 f603    	asr.w	r6, r6, r3
 8006f4a: f8c1 62b8    	str.w	r6, [r1, #0x2b8]
 8006f4e: f8d1 62d0    	ldr.w	r6, [r1, #0x2d0]
 8006f52: 4416         	add	r6, r2
 8006f54: fa46 f603    	asr.w	r6, r6, r3
 8006f58: f8c1 62d0    	str.w	r6, [r1, #0x2d0]
 8006f5c: f8d1 62e8    	ldr.w	r6, [r1, #0x2e8]
 8006f60: 4416         	add	r6, r2
 8006f62: fa46 f603    	asr.w	r6, r6, r3
 8006f66: f8c1 62e8    	str.w	r6, [r1, #0x2e8]
 8006f6a: f501 71c0    	add.w	r1, r1, #0x180
 8006f6e: d181         	bne	0x8006e74 <raylar_crcl_encoder_service::benchmark::rounding::h32a8b1c6cb68e077+0x148> @ imm = #-0xfe
 8006f70: 6901         	ldr	r1, [r0, #0x10]
 8006f72: 2601         	movs	r6, #0x1
 8006f74: 9101         	str	r1, [sp, #0x4]
 8006f76: a901         	add	r1, sp, #0x4
 8006f78: f645 7174    	movw	r1, #0x5f74
 8006f7c: f44f 6c80    	mov.w	r12, #0x400
 8006f80: 9b01         	ldr	r3, [sp, #0x4]
 8006f82: 4401         	add	r1, r0
 8006f84: 1e5a         	subs	r2, r3, #0x1
 8006f86: f003 031f    	and	r3, r3, #0x1f
 8006f8a: f002 021f    	and	r2, r2, #0x1f
 8006f8e: fa06 f202    	lsl.w	r2, r6, r2
 8006f92: f8d1 40d8    	ldr.w	r4, [r1, #0xd8]
 8006f96: f1bc 0c10    	subs.w	r12, r12, #0x10
 8006f9a: f8d1 50f0    	ldr.w	r5, [r1, #0xf0]
 8006f9e: 4414         	add	r4, r2
 8006fa0: f8d1 8108    	ldr.w	r8, [r1, #0x108]
 8006fa4: f8d1 60c0    	ldr.w	r6, [r1, #0xc0]
 8006fa8: fa44 f403    	asr.w	r4, r4, r3
 8006fac: f8d1 e198    	ldr.w	lr, [r1, #0x198]
 8006fb0: f8c1 40d8    	str.w	r4, [r1, #0xd8]
 8006fb4: eb05 0402    	add.w	r4, r5, r2
 8006fb8: 4416         	add	r6, r2
 8006fba: f8d1 5120    	ldr.w	r5, [r1, #0x120]
 8006fbe: fa44 f403    	asr.w	r4, r4, r3
 8006fc2: f8d1 9180    	ldr.w	r9, [r1, #0x180]
 8006fc6: f8c1 40f0    	str.w	r4, [r1, #0xf0]
 8006fca: eb08 0402    	add.w	r4, r8, r2
 8006fce: fa46 f603    	asr.w	r6, r6, r3
 8006fd2: fa44 f403    	asr.w	r4, r4, r3
 8006fd6: f8c1 60c0    	str.w	r6, [r1, #0xc0]
 8006fda: f8c1 4108    	str.w	r4, [r1, #0x108]
 8006fde: eb05 0402    	add.w	r4, r5, r2
 8006fe2: f8d1 6138    	ldr.w	r6, [r1, #0x138]
 8006fe6: fa44 f403    	asr.w	r4, r4, r3
 8006fea: f8d1 5150    	ldr.w	r5, [r1, #0x150]
 8006fee: f8c1 4120    	str.w	r4, [r1, #0x120]
 8006ff2: eb06 0402    	add.w	r4, r6, r2
 8006ff6: f8d1 6168    	ldr.w	r6, [r1, #0x168]
 8006ffa: fa44 f403    	asr.w	r4, r4, r3
 8006ffe: f8c1 4138    	str.w	r4, [r1, #0x138]
 8007002: eb05 0402    	add.w	r4, r5, r2
 8007006: fa44 f403    	asr.w	r4, r4, r3
 800700a: f8c1 4150    	str.w	r4, [r1, #0x150]
 800700e: eb06 0402    	add.w	r4, r6, r2
 8007012: eb0e 0602    	add.w	r6, lr, r2
 8007016: fa44 f403    	asr.w	r4, r4, r3
 800701a: fa46 f603    	asr.w	r6, r6, r3
 800701e: f8c1 4168    	str.w	r4, [r1, #0x168]
 8007022: f8c1 6198    	str.w	r6, [r1, #0x198]
 8007026: eb09 0402    	add.w	r4, r9, r2
 800702a: f8d1 61b0    	ldr.w	r6, [r1, #0x1b0]
 800702e: fa44 f403    	asr.w	r4, r4, r3
 8007032: 4416         	add	r6, r2
 8007034: f8c1 4180    	str.w	r4, [r1, #0x180]
 8007038: fa46 f603    	asr.w	r6, r6, r3
 800703c: f8c1 61b0    	str.w	r6, [r1, #0x1b0]
 8007040: f8d1 61c8    	ldr.w	r6, [r1, #0x1c8]
 8007044: 4416         	add	r6, r2
 8007046: fa46 f603    	asr.w	r6, r6, r3
 800704a: f8c1 61c8    	str.w	r6, [r1, #0x1c8]
 800704e: f8d1 61e0    	ldr.w	r6, [r1, #0x1e0]
 8007052: 4416         	add	r6, r2
 8007054: fa46 f603    	asr.w	r6, r6, r3
 8007058: f8c1 61e0    	str.w	r6, [r1, #0x1e0]
 800705c: f8d1 61f8    	ldr.w	r6, [r1, #0x1f8]
 8007060: 4416         	add	r6, r2
 8007062: fa46 f603    	asr.w	r6, r6, r3
 8007066: f8c1 61f8    	str.w	r6, [r1, #0x1f8]
 800706a: f8d1 6210    	ldr.w	r6, [r1, #0x210]
 800706e: 4416         	add	r6, r2
 8007070: fa46 f603    	asr.w	r6, r6, r3
 8007074: f8c1 6210    	str.w	r6, [r1, #0x210]
 8007078: f8d1 6228    	ldr.w	r6, [r1, #0x228]
 800707c: 4416         	add	r6, r2
 800707e: fa46 f603    	asr.w	r6, r6, r3
 8007082: f8c1 6228    	str.w	r6, [r1, #0x228]
 8007086: f501 71c0    	add.w	r1, r1, #0x180
 800708a: d182         	bne	0x8006f92 <raylar_crcl_encoder_service::benchmark::rounding::h32a8b1c6cb68e077+0x266> @ imm = #-0xfc
 800708c: 6981         	ldr	r1, [r0, #0x18]
 800708e: 2601         	movs	r6, #0x1
 8007090: 9101         	str	r1, [sp, #0x4]
 8007092: a901         	add	r1, sp, #0x4
 8007094: f645 61b8    	movw	r1, #0x5eb8
 8007098: f44f 6c80    	mov.w	r12, #0x400
 800709c: 9b01         	ldr	r3, [sp, #0x4]
 800709e: 4401         	add	r1, r0
 80070a0: 1e5a         	subs	r2, r3, #0x1
 80070a2: f003 031f    	and	r3, r3, #0x1f
 80070a6: f002 021f    	and	r2, r2, #0x1f
 80070aa: fa06 f202    	lsl.w	r2, r6, r2
 80070ae: f8d1 4198    	ldr.w	r4, [r1, #0x198]
 80070b2: f1bc 0c10    	subs.w	r12, r12, #0x10
 80070b6: f8d1 51b0    	ldr.w	r5, [r1, #0x1b0]
 80070ba: 4414         	add	r4, r2
 80070bc: f8d1 81c8    	ldr.w	r8, [r1, #0x1c8]
 80070c0: f8d1 6180    	ldr.w	r6, [r1, #0x180]
 80070c4: fa44 f403    	asr.w	r4, r4, r3
 80070c8: f8d1 e258    	ldr.w	lr, [r1, #0x258]
 80070cc: f8c1 4198    	str.w	r4, [r1, #0x198]
 80070d0: eb05 0402    	add.w	r4, r5, r2
 80070d4: 4416         	add	r6, r2
 80070d6: f8d1 51e0    	ldr.w	r5, [r1, #0x1e0]
 80070da: fa44 f403    	asr.w	r4, r4, r3
 80070de: f8d1 9240    	ldr.w	r9, [r1, #0x240]
 80070e2: f8c1 41b0    	str.w	r4, [r1, #0x1b0]
 80070e6: eb08 0402    	add.w	r4, r8, r2
 80070ea: fa46 f603    	asr.w	r6, r6, r3
 80070ee: fa44 f403    	asr.w	r4, r4, r3
 80070f2: f8c1 6180    	str.w	r6, [r1, #0x180]
 80070f6: f8c1 41c8    	str.w	r4, [r1, #0x1c8]
 80070fa: eb05 0402    	add.w	r4, r5, r2
 80070fe: f8d1 61f8    	ldr.w	r6, [r1, #0x1f8]
 8007102: fa44 f403    	asr.w	r4, r4, r3
 8007106: f8d1 5210    	ldr.w	r5, [r1, #0x210]
 800710a: f8c1 41e0    	str.w	r4, [r1, #0x1e0]
 800710e: eb06 0402    	add.w	r4, r6, r2
 8007112: f8d1 6228    	ldr.w	r6, [r1, #0x228]
 8007116: fa44 f403    	asr.w	r4, r4, r3
 800711a: f8c1 41f8    	str.w	r4, [r1, #0x1f8]
 800711e: eb05 0402    	add.w	r4, r5, r2
 8007122: fa44 f403    	asr.w	r4, r4, r3
 8007126: f8c1 4210    	str.w	r4, [r1, #0x210]
 800712a: eb06 0402    	add.w	r4, r6, r2
 800712e: eb0e 0602    	add.w	r6, lr, r2
 8007132: fa44 f403    	asr.w	r4, r4, r3
 8007136: fa46 f603    	asr.w	r6, r6, r3
 800713a: f8c1 4228    	str.w	r4, [r1, #0x228]
 800713e: f8c1 6258    	str.w	r6, [r1, #0x258]
 8007142: eb09 0402    	add.w	r4, r9, r2
 8007146: f8d1 6270    	ldr.w	r6, [r1, #0x270]
 800714a: fa44 f403    	asr.w	r4, r4, r3
 800714e: 4416         	add	r6, r2
 8007150: f8c1 4240    	str.w	r4, [r1, #0x240]
 8007154: fa46 f603    	asr.w	r6, r6, r3
 8007158: f8c1 6270    	str.w	r6, [r1, #0x270]
 800715c: f8d1 6288    	ldr.w	r6, [r1, #0x288]
 8007160: 4416         	add	r6, r2
 8007162: fa46 f603    	asr.w	r6, r6, r3
 8007166: f8c1 6288    	str.w	r6, [r1, #0x288]
 800716a: f8d1 62a0    	ldr.w	r6, [r1, #0x2a0]
 800716e: 4416         	add	r6, r2
 8007170: fa46 f603    	asr.w	r6, r6, r3
 8007174: f8c1 62a0    	str.w	r6, [r1, #0x2a0]
 8007178: f8d1 62b8    	ldr.w	r6, [r1, #0x2b8]
 800717c: 4416         	add	r6, r2
 800717e: fa46 f603    	asr.w	r6, r6, r3
 8007182: f8c1 62b8    	str.w	r6, [r1, #0x2b8]
 8007186: f8d1 62d0    	ldr.w	r6, [r1, #0x2d0]
 800718a: 4416         	add	r6, r2
 800718c: fa46 f603    	asr.w	r6, r6, r3
 8007190: f8c1 62d0    	str.w	r6, [r1, #0x2d0]
 8007194: f8d1 62e8    	ldr.w	r6, [r1, #0x2e8]
 8007198: 4416         	add	r6, r2
 800719a: fa46 f603    	asr.w	r6, r6, r3
 800719e: f8c1 62e8    	str.w	r6, [r1, #0x2e8]
 80071a2: f501 71c0    	add.w	r1, r1, #0x180
 80071a6: d182         	bne	0x80070ae <raylar_crcl_encoder_service::benchmark::rounding::h32a8b1c6cb68e077+0x382> @ imm = #-0xfc
 80071a8: 6a01         	ldr	r1, [r0, #0x20]
 80071aa: 2601         	movs	r6, #0x1
 80071ac: 9101         	str	r1, [sp, #0x4]
 80071ae: a901         	add	r1, sp, #0x4
 80071b0: f645 717c    	movw	r1, #0x5f7c
 80071b4: f44f 6c80    	mov.w	r12, #0x400
 80071b8: 9b01         	ldr	r3, [sp, #0x4]
 80071ba: 4401         	add	r1, r0
 80071bc: 1e5a         	subs	r2, r3, #0x1
 80071be: f003 031f    	and	r3, r3, #0x1f
 80071c2: f002 021f    	and	r2, r2, #0x1f
 80071c6: fa06 f202    	lsl.w	r2, r6, r2
 80071ca: f8d1 40d8    	ldr.w	r4, [r1, #0xd8]
 80071ce: f1bc 0c10    	subs.w	r12, r12, #0x10
 80071d2: f8d1 50f0    	ldr.w	r5, [r1, #0xf0]
 80071d6: 4414         	add	r4, r2
 80071d8: f8d1 8108    	ldr.w	r8, [r1, #0x108]
 80071dc: f8d1 60c0    	ldr.w	r6, [r1, #0xc0]
 80071e0: fa44 f403    	asr.w	r4, r4, r3
 80071e4: f8d1 e198    	ldr.w	lr, [r1, #0x198]
 80071e8: f8c1 40d8    	str.w	r4, [r1, #0xd8]
 80071ec: eb05 0402    	add.w	r4, r5, r2
 80071f0: 4416         	add	r6, r2
 80071f2: f8d1 5120    	ldr.w	r5, [r1, #0x120]
 80071f6: fa44 f403    	asr.w	r4, r4, r3
 80071fa: f8d1 9180    	ldr.w	r9, [r1, #0x180]
 80071fe: f8c1 40f0    	str.w	r4, [r1, #0xf0]
 8007202: eb08 0402    	add.w	r4, r8, r2
 8007206: fa46 f603    	asr.w	r6, r6, r3
 800720a: fa44 f403    	asr.w	r4, r4, r3
 800720e: f8c1 60c0    	str.w	r6, [r1, #0xc0]
 8007212: f8c1 4108    	str.w	r4, [r1, #0x108]
 8007216: eb05 0402    	add.w	r4, r5, r2
 800721a: f8d1 6138    	ldr.w	r6, [r1, #0x138]
 800721e: fa44 f403    	asr.w	r4, r4, r3
 8007222: f8d1 5150    	ldr.w	r5, [r1, #0x150]
 8007226: f8c1 4120    	str.w	r4, [r1, #0x120]
 800722a: eb06 0402    	add.w	r4, r6, r2
 800722e: f8d1 6168    	ldr.w	r6, [r1, #0x168]
 8007232: fa44 f403    	asr.w	r4, r4, r3
 8007236: f8c1 4138    	str.w	r4, [r1, #0x138]
 800723a: eb05 0402    	add.w	r4, r5, r2
 800723e: fa44 f403    	asr.w	r4, r4, r3
 8007242: f8c1 4150    	str.w	r4, [r1, #0x150]
 8007246: eb06 0402    	add.w	r4, r6, r2
 800724a: eb0e 0602    	add.w	r6, lr, r2
 800724e: fa44 f403    	asr.w	r4, r4, r3
 8007252: fa46 f603    	asr.w	r6, r6, r3
 8007256: f8c1 4168    	str.w	r4, [r1, #0x168]
 800725a: f8c1 6198    	str.w	r6, [r1, #0x198]
 800725e: eb09 0402    	add.w	r4, r9, r2
 8007262: f8d1 61b0    	ldr.w	r6, [r1, #0x1b0]
 8007266: fa44 f403    	asr.w	r4, r4, r3
 800726a: 4416         	add	r6, r2
 800726c: f8c1 4180    	str.w	r4, [r1, #0x180]
 8007270: fa46 f603    	asr.w	r6, r6, r3
 8007274: f8c1 61b0    	str.w	r6, [r1, #0x1b0]
 8007278: f8d1 61c8    	ldr.w	r6, [r1, #0x1c8]
 800727c: 4416         	add	r6, r2
 800727e: fa46 f603    	asr.w	r6, r6, r3
 8007282: f8c1 61c8    	str.w	r6, [r1, #0x1c8]
 8007286: f8d1 61e0    	ldr.w	r6, [r1, #0x1e0]
 800728a: 4416         	add	r6, r2
 800728c: fa46 f603    	asr.w	r6, r6, r3
 8007290: f8c1 61e0    	str.w	r6, [r1, #0x1e0]
 8007294: f8d1 61f8    	ldr.w	r6, [r1, #0x1f8]
 8007298: 4416         	add	r6, r2
 800729a: fa46 f603    	asr.w	r6, r6, r3
 800729e: f8c1 61f8    	str.w	r6, [r1, #0x1f8]
 80072a2: f8d1 6210    	ldr.w	r6, [r1, #0x210]
 80072a6: 4416         	add	r6, r2
 80072a8: fa46 f603    	asr.w	r6, r6, r3
 80072ac: f8c1 6210    	str.w	r6, [r1, #0x210]
 80072b0: f8d1 6228    	ldr.w	r6, [r1, #0x228]
 80072b4: 4416         	add	r6, r2
 80072b6: fa46 f603    	asr.w	r6, r6, r3
 80072ba: f8c1 6228    	str.w	r6, [r1, #0x228]
 80072be: f501 71c0    	add.w	r1, r1, #0x180
 80072c2: d182         	bne	0x80071ca <raylar_crcl_encoder_service::benchmark::rounding::h32a8b1c6cb68e077+0x49e> @ imm = #-0xfc
 80072c4: f246 0128    	movw	r1, #0x6028
 80072c8: 4401         	add	r1, r0
 80072ca: 9100         	str	r1, [sp]
 80072cc: 4669         	mov	r1, sp
 80072ce: f246 012c    	movw	r1, #0x602c
 80072d2: 5840         	ldr	r0, [r0, r1]
 80072d4: b002         	add	sp, #0x8
 80072d6: e8bd 0b00    	pop.w	{r8, r9, r11}
 80072da: bdf0         	pop	{r4, r5, r6, r7, pc}

080072dc <raylar_crcl_encoder_service::benchmark::sum_only::he5272faae351e30d>:
 80072dc: b5f0         	push	{r4, r5, r6, r7, lr}
 80072de: af03         	add	r7, sp, #0xc
 80072e0: e92d 0f00    	push.w	{r8, r9, r10, r11}
 80072e4: b08a         	sub	sp, #0x28
 80072e6: 9004         	str	r0, [sp, #0x10]
 80072e8: f500 30b8    	add.w	r0, r0, #0x17000
 80072ec: 3010         	adds	r0, #0x10
 80072ee: 2500         	movs	r5, #0x0
 80072f0: f240 39ff    	movw	r9, #0x3ff
 80072f4: f850 1f84    	ldr	r1, [r0, #132]!
 80072f8: f1b9 0921    	subs.w	r9, r9, #0x21
 80072fc: f1a0 0c78    	sub.w	r12, r0, #0x78
 8007300: 9108         	str	r1, [sp, #0x20]
 8007302: e950 4220    	ldrd	r4, r2, [r0, #-128]
 8007306: f1a0 0e60    	sub.w	lr, r0, #0x60
 800730a: e89c 1048    	ldm.w	r12, {r3, r6, r12}
 800730e: 442c         	add	r4, r5
 8007310: 4422         	add	r2, r4
 8007312: f850 1c04    	ldr	r1, [r0, #-4]
 8007316: 441a         	add	r2, r3
 8007318: 9107         	str	r1, [sp, #0x1c]
 800731a: f850 1c08    	ldr	r1, [r0, #-8]
 800731e: 4432         	add	r2, r6
 8007320: 9106         	str	r1, [sp, #0x18]
 8007322: f850 1c0c    	ldr	r1, [r0, #-12]
 8007326: e950 ab1b    	ldrd	r10, r11, [r0, #-108]
 800732a: 9105         	str	r1, [sp, #0x14]
 800732c: eb0c 0102    	add.w	r1, r12, r2
 8007330: 4451         	add	r1, r10
 8007332: f850 8c64    	ldr	r8, [r0, #-100]
 8007336: 4459         	add	r1, r11
 8007338: e89e 545c    	ldm.w	lr, {r2, r3, r4, r6, r10, r12, lr}
 800733c: 4441         	add	r1, r8
 800733e: 4411         	add	r1, r2
 8007340: f850 5c44    	ldr	r5, [r0, #-68]
 8007344: 4419         	add	r1, r3
 8007346: f1a0 0b2c    	sub.w	r11, r0, #0x2c
 800734a: 4421         	add	r1, r4
 800734c: 4431         	add	r1, r6
 800734e: f1a0 0640    	sub.w	r6, r0, #0x40
 8007352: 4451         	add	r1, r10
 8007354: e89b 0d00    	ldm.w	r11, {r8, r10, r11}
 8007358: 4461         	add	r1, r12
 800735a: 4471         	add	r1, lr
 800735c: ce4c         	ldm	r6, {r2, r3, r6}
 800735e: 4429         	add	r1, r5
 8007360: 4411         	add	r1, r2
 8007362: e950 4c0d    	ldrd	r4, r12, [r0, #-52]
 8007366: 4419         	add	r1, r3
 8007368: f850 2c20    	ldr	r2, [r0, #-32]
 800736c: 4431         	add	r1, r6
 800736e: f850 3c1c    	ldr	r3, [r0, #-28]
 8007372: 4421         	add	r1, r4
 8007374: f850 6c18    	ldr	r6, [r0, #-24]
 8007378: 4461         	add	r1, r12
 800737a: 4441         	add	r1, r8
 800737c: 4451         	add	r1, r10
 800737e: 4459         	add	r1, r11
 8007380: 4411         	add	r1, r2
 8007382: f850 2c14    	ldr	r2, [r0, #-20]
 8007386: 4419         	add	r1, r3
 8007388: f850 3c10    	ldr	r3, [r0, #-16]
 800738c: 4431         	add	r1, r6
 800738e: 4411         	add	r1, r2
 8007390: 9a05         	ldr	r2, [sp, #0x14]
 8007392: 4419         	add	r1, r3
 8007394: 4411         	add	r1, r2
 8007396: 9a06         	ldr	r2, [sp, #0x18]
 8007398: 4411         	add	r1, r2
 800739a: 9a07         	ldr	r2, [sp, #0x1c]
 800739c: 4411         	add	r1, r2
 800739e: 9a08         	ldr	r2, [sp, #0x20]
 80073a0: eb02 0501    	add.w	r5, r2, r1
 80073a4: d1a6         	bne	0x80072f4 <raylar_crcl_encoder_service::benchmark::sum_only::he5272faae351e30d+0x18> @ imm = #-0xb4
 80073a6: 9509         	str	r5, [sp, #0x24]
 80073a8: a809         	add	r0, sp, #0x24
 80073aa: 2500         	movs	r5, #0x0
 80073ac: f240 39ff    	movw	r9, #0x3ff
 80073b0: 9809         	ldr	r0, [sp, #0x24]
 80073b2: 9003         	str	r0, [sp, #0xc]
 80073b4: 9804         	ldr	r0, [sp, #0x10]
 80073b6: f500 30c0    	add.w	r0, r0, #0x18000
 80073ba: 300c         	adds	r0, #0xc
 80073bc: f850 1f84    	ldr	r1, [r0, #132]!
 80073c0: f1b9 0921    	subs.w	r9, r9, #0x21
 80073c4: f1a0 0c78    	sub.w	r12, r0, #0x78
 80073c8: 9108         	str	r1, [sp, #0x20]
 80073ca: e950 4220    	ldrd	r4, r2, [r0, #-128]
 80073ce: f1a0 0e60    	sub.w	lr, r0, #0x60
 80073d2: e89c 1048    	ldm.w	r12, {r3, r6, r12}
 80073d6: 442c         	add	r4, r5
 80073d8: 4422         	add	r2, r4
 80073da: f850 1c04    	ldr	r1, [r0, #-4]
 80073de: 441a         	add	r2, r3
 80073e0: 9107         	str	r1, [sp, #0x1c]
 80073e2: f850 1c08    	ldr	r1, [r0, #-8]
 80073e6: 4432         	add	r2, r6
 80073e8: 9106         	str	r1, [sp, #0x18]
 80073ea: f850 1c0c    	ldr	r1, [r0, #-12]
 80073ee: e950 ab1b    	ldrd	r10, r11, [r0, #-108]
 80073f2: 9105         	str	r1, [sp, #0x14]
 80073f4: eb0c 0102    	add.w	r1, r12, r2
 80073f8: 4451         	add	r1, r10
 80073fa: f850 8c64    	ldr	r8, [r0, #-100]
 80073fe: 4459         	add	r1, r11
 8007400: e89e 545c    	ldm.w	lr, {r2, r3, r4, r6, r10, r12, lr}
 8007404: 4441         	add	r1, r8
 8007406: 4411         	add	r1, r2
 8007408: f850 5c44    	ldr	r5, [r0, #-68]
 800740c: 4419         	add	r1, r3
 800740e: f1a0 0b2c    	sub.w	r11, r0, #0x2c
 8007412: 4421         	add	r1, r4
 8007414: 4431         	add	r1, r6
 8007416: f1a0 0640    	sub.w	r6, r0, #0x40
 800741a: 4451         	add	r1, r10
 800741c: e89b 0d00    	ldm.w	r11, {r8, r10, r11}
 8007420: 4461         	add	r1, r12
 8007422: 4471         	add	r1, lr
 8007424: ce4c         	ldm	r6, {r2, r3, r6}
 8007426: 4429         	add	r1, r5
 8007428: 4411         	add	r1, r2
 800742a: e950 4c0d    	ldrd	r4, r12, [r0, #-52]
 800742e: 4419         	add	r1, r3
 8007430: f850 2c20    	ldr	r2, [r0, #-32]
 8007434: 4431         	add	r1, r6
 8007436: f850 3c1c    	ldr	r3, [r0, #-28]
 800743a: 4421         	add	r1, r4
 800743c: f850 6c18    	ldr	r6, [r0, #-24]
 8007440: 4461         	add	r1, r12
 8007442: 4441         	add	r1, r8
 8007444: 4451         	add	r1, r10
 8007446: 4459         	add	r1, r11
 8007448: 4411         	add	r1, r2
 800744a: f850 2c14    	ldr	r2, [r0, #-20]
 800744e: 4419         	add	r1, r3
 8007450: f850 3c10    	ldr	r3, [r0, #-16]
 8007454: 4431         	add	r1, r6
 8007456: 4411         	add	r1, r2
 8007458: 9a05         	ldr	r2, [sp, #0x14]
 800745a: 4419         	add	r1, r3
 800745c: 4411         	add	r1, r2
 800745e: 9a06         	ldr	r2, [sp, #0x18]
 8007460: 4411         	add	r1, r2
 8007462: 9a07         	ldr	r2, [sp, #0x1c]
 8007464: 4411         	add	r1, r2
 8007466: 9a08         	ldr	r2, [sp, #0x20]
 8007468: eb02 0501    	add.w	r5, r2, r1
 800746c: d1a6         	bne	0x80073bc <raylar_crcl_encoder_service::benchmark::sum_only::he5272faae351e30d+0xe0> @ imm = #-0xb4
 800746e: 9509         	str	r5, [sp, #0x24]
 8007470: a809         	add	r0, sp, #0x24
 8007472: 2500         	movs	r5, #0x0
 8007474: f240 39ff    	movw	r9, #0x3ff
 8007478: 9809         	ldr	r0, [sp, #0x24]
 800747a: 9002         	str	r0, [sp, #0x8]
 800747c: 9804         	ldr	r0, [sp, #0x10]
 800747e: f500 30c8    	add.w	r0, r0, #0x19000
 8007482: 3008         	adds	r0, #0x8
 8007484: f850 1f84    	ldr	r1, [r0, #132]!
 8007488: f1b9 0921    	subs.w	r9, r9, #0x21
 800748c: f1a0 0c78    	sub.w	r12, r0, #0x78
 8007490: 9108         	str	r1, [sp, #0x20]
 8007492: e950 4220    	ldrd	r4, r2, [r0, #-128]
 8007496: f1a0 0e60    	sub.w	lr, r0, #0x60
 800749a: e89c 1048    	ldm.w	r12, {r3, r6, r12}
 800749e: 442c         	add	r4, r5
 80074a0: 4422         	add	r2, r4
 80074a2: f850 1c04    	ldr	r1, [r0, #-4]
 80074a6: 441a         	add	r2, r3
 80074a8: 9107         	str	r1, [sp, #0x1c]
 80074aa: f850 1c08    	ldr	r1, [r0, #-8]
 80074ae: 4432         	add	r2, r6
 80074b0: 9106         	str	r1, [sp, #0x18]
 80074b2: f850 1c0c    	ldr	r1, [r0, #-12]
 80074b6: e950 ab1b    	ldrd	r10, r11, [r0, #-108]
 80074ba: 9105         	str	r1, [sp, #0x14]
 80074bc: eb0c 0102    	add.w	r1, r12, r2
 80074c0: 4451         	add	r1, r10
 80074c2: f850 8c64    	ldr	r8, [r0, #-100]
 80074c6: 4459         	add	r1, r11
 80074c8: e89e 545c    	ldm.w	lr, {r2, r3, r4, r6, r10, r12, lr}
 80074cc: 4441         	add	r1, r8
 80074ce: 4411         	add	r1, r2
 80074d0: f850 5c44    	ldr	r5, [r0, #-68]
 80074d4: 4419         	add	r1, r3
 80074d6: f1a0 0b2c    	sub.w	r11, r0, #0x2c
 80074da: 4421         	add	r1, r4
 80074dc: 4431         	add	r1, r6
 80074de: f1a0 0640    	sub.w	r6, r0, #0x40
 80074e2: 4451         	add	r1, r10
 80074e4: e89b 0d00    	ldm.w	r11, {r8, r10, r11}
 80074e8: 4461         	add	r1, r12
 80074ea: 4471         	add	r1, lr
 80074ec: ce4c         	ldm	r6, {r2, r3, r6}
 80074ee: 4429         	add	r1, r5
 80074f0: 4411         	add	r1, r2
 80074f2: e950 4c0d    	ldrd	r4, r12, [r0, #-52]
 80074f6: 4419         	add	r1, r3
 80074f8: f850 2c20    	ldr	r2, [r0, #-32]
 80074fc: 4431         	add	r1, r6
 80074fe: f850 3c1c    	ldr	r3, [r0, #-28]
 8007502: 4421         	add	r1, r4
 8007504: f850 6c18    	ldr	r6, [r0, #-24]
 8007508: 4461         	add	r1, r12
 800750a: 4441         	add	r1, r8
 800750c: 4451         	add	r1, r10
 800750e: 4459         	add	r1, r11
 8007510: 4411         	add	r1, r2
 8007512: f850 2c14    	ldr	r2, [r0, #-20]
 8007516: 4419         	add	r1, r3
 8007518: f850 3c10    	ldr	r3, [r0, #-16]
 800751c: 4431         	add	r1, r6
 800751e: 4411         	add	r1, r2
 8007520: 9a05         	ldr	r2, [sp, #0x14]
 8007522: 4419         	add	r1, r3
 8007524: 4411         	add	r1, r2
 8007526: 9a06         	ldr	r2, [sp, #0x18]
 8007528: 4411         	add	r1, r2
 800752a: 9a07         	ldr	r2, [sp, #0x1c]
 800752c: 4411         	add	r1, r2
 800752e: 9a08         	ldr	r2, [sp, #0x20]
 8007530: eb02 0501    	add.w	r5, r2, r1
 8007534: d1a6         	bne	0x8007484 <raylar_crcl_encoder_service::benchmark::sum_only::he5272faae351e30d+0x1a8> @ imm = #-0xb4
 8007536: 9509         	str	r5, [sp, #0x24]
 8007538: a809         	add	r0, sp, #0x24
 800753a: f24f 0104    	movw	r1, #0xf004
 800753e: f24b 046c    	movw	r4, #0xb06c
 8007542: 9a04         	ldr	r2, [sp, #0x10]
 8007544: f24b 0574    	movw	r5, #0xb074
 8007548: f24b 0c70    	movw	r12, #0xb070
 800754c: 9809         	ldr	r0, [sp, #0x24]
 800754e: f04f 0b00    	mov.w	r11, #0x0
 8007552: f6cf 71ff    	movt	r1, #0xffff
 8007556: f2c0 0401    	movt	r4, #0x1
 800755a: f2c0 0501    	movt	r5, #0x1
 800755e: f2c0 0c01    	movt	r12, #0x1
 8007562: 9001         	str	r0, [sp, #0x4]
 8007564: f24b 0384    	movw	r3, #0xb084
 8007568: 1850         	adds	r0, r2, r1
 800756a: f2c0 0301    	movt	r3, #0x1
 800756e: 3184         	adds	r1, #0x84
 8007570: f850 e003    	ldr.w	lr, [r0, r3]
 8007574: f24b 0380    	movw	r3, #0xb080
 8007578: f2c0 0301    	movt	r3, #0x1
 800757c: f850 8003    	ldr.w	r8, [r0, r3]
 8007580: f24b 037c    	movw	r3, #0xb07c
 8007584: f2c0 0301    	movt	r3, #0x1
 8007588: f850 9003    	ldr.w	r9, [r0, r3]
 800758c: f24b 0378    	movw	r3, #0xb078
 8007590: f2c0 0301    	movt	r3, #0x1
 8007594: f850 a003    	ldr.w	r10, [r0, r3]
 8007598: f24b 0308    	movw	r3, #0xb008
 800759c: f2c0 0301    	movt	r3, #0x1
 80075a0: 58c6         	ldr	r6, [r0, r3]
 80075a2: f24b 0304    	movw	r3, #0xb004
 80075a6: f2c0 0301    	movt	r3, #0x1
 80075aa: 58c3         	ldr	r3, [r0, r3]
 80075ac: 445b         	add	r3, r11
 80075ae: 4433         	add	r3, r6
 80075b0: f24b 060c    	movw	r6, #0xb00c
 80075b4: f2c0 0601    	movt	r6, #0x1
 80075b8: 5986         	ldr	r6, [r0, r6]
 80075ba: 4433         	add	r3, r6
 80075bc: f24b 0610    	movw	r6, #0xb010
 80075c0: f2c0 0601    	movt	r6, #0x1
 80075c4: 5986         	ldr	r6, [r0, r6]
 80075c6: 4433         	add	r3, r6
 80075c8: f24b 0614    	movw	r6, #0xb014
 80075cc: f2c0 0601    	movt	r6, #0x1
 80075d0: 5986         	ldr	r6, [r0, r6]
 80075d2: 4433         	add	r3, r6
 80075d4: f24b 0618    	movw	r6, #0xb018
 80075d8: f2c0 0601    	movt	r6, #0x1
 80075dc: 5986         	ldr	r6, [r0, r6]
 80075de: 4433         	add	r3, r6
 80075e0: f24b 061c    	movw	r6, #0xb01c
 80075e4: f2c0 0601    	movt	r6, #0x1
 80075e8: 5986         	ldr	r6, [r0, r6]
 80075ea: 4433         	add	r3, r6
 80075ec: f24b 0620    	movw	r6, #0xb020
 80075f0: f2c0 0601    	movt	r6, #0x1
 80075f4: 5986         	ldr	r6, [r0, r6]
 80075f6: 4433         	add	r3, r6
 80075f8: f24b 0624    	movw	r6, #0xb024
 80075fc: f2c0 0601    	movt	r6, #0x1
 8007600: 5986         	ldr	r6, [r0, r6]
 8007602: 4433         	add	r3, r6
 8007604: f24b 0628    	movw	r6, #0xb028
 8007608: f2c0 0601    	movt	r6, #0x1
 800760c: 5986         	ldr	r6, [r0, r6]
 800760e: 4433         	add	r3, r6
 8007610: f24b 062c    	movw	r6, #0xb02c
 8007614: f2c0 0601    	movt	r6, #0x1
 8007618: 5986         	ldr	r6, [r0, r6]
 800761a: 4433         	add	r3, r6
 800761c: f24b 0630    	movw	r6, #0xb030
 8007620: f2c0 0601    	movt	r6, #0x1
 8007624: 5986         	ldr	r6, [r0, r6]
 8007626: 4433         	add	r3, r6
 8007628: f24b 0634    	movw	r6, #0xb034
 800762c: f2c0 0601    	movt	r6, #0x1
 8007630: 5986         	ldr	r6, [r0, r6]
 8007632: 4433         	add	r3, r6
 8007634: f24b 0638    	movw	r6, #0xb038
 8007638: f2c0 0601    	movt	r6, #0x1
 800763c: 5986         	ldr	r6, [r0, r6]
 800763e: 4433         	add	r3, r6
 8007640: f24b 063c    	movw	r6, #0xb03c
 8007644: f2c0 0601    	movt	r6, #0x1
 8007648: 5986         	ldr	r6, [r0, r6]
 800764a: 4433         	add	r3, r6
 800764c: f24b 0640    	movw	r6, #0xb040
 8007650: f2c0 0601    	movt	r6, #0x1
 8007654: 5986         	ldr	r6, [r0, r6]
 8007656: 4433         	add	r3, r6
 8007658: f24b 0644    	movw	r6, #0xb044
 800765c: f2c0 0601    	movt	r6, #0x1
 8007660: 5986         	ldr	r6, [r0, r6]
 8007662: 4433         	add	r3, r6
 8007664: f24b 0648    	movw	r6, #0xb048
 8007668: f2c0 0601    	movt	r6, #0x1
 800766c: 5986         	ldr	r6, [r0, r6]
 800766e: 4433         	add	r3, r6
 8007670: f24b 064c    	movw	r6, #0xb04c
 8007674: f2c0 0601    	movt	r6, #0x1
 8007678: 5986         	ldr	r6, [r0, r6]
 800767a: 4433         	add	r3, r6
 800767c: f24b 0650    	movw	r6, #0xb050
 8007680: f2c0 0601    	movt	r6, #0x1
 8007684: 5986         	ldr	r6, [r0, r6]
 8007686: 4433         	add	r3, r6
 8007688: f24b 0654    	movw	r6, #0xb054
 800768c: f2c0 0601    	movt	r6, #0x1
 8007690: 5986         	ldr	r6, [r0, r6]
 8007692: 4433         	add	r3, r6
 8007694: f24b 0658    	movw	r6, #0xb058
 8007698: f2c0 0601    	movt	r6, #0x1
 800769c: 5986         	ldr	r6, [r0, r6]
 800769e: 4433         	add	r3, r6
 80076a0: f24b 065c    	movw	r6, #0xb05c
 80076a4: f2c0 0601    	movt	r6, #0x1
 80076a8: 5986         	ldr	r6, [r0, r6]
 80076aa: 4433         	add	r3, r6
 80076ac: f24b 0660    	movw	r6, #0xb060
 80076b0: f2c0 0601    	movt	r6, #0x1
 80076b4: 5986         	ldr	r6, [r0, r6]
 80076b6: 4433         	add	r3, r6
 80076b8: f24b 0664    	movw	r6, #0xb064
 80076bc: f2c0 0601    	movt	r6, #0x1
 80076c0: 5986         	ldr	r6, [r0, r6]
 80076c2: 4433         	add	r3, r6
 80076c4: f24b 0668    	movw	r6, #0xb068
 80076c8: f2c0 0601    	movt	r6, #0x1
 80076cc: 5986         	ldr	r6, [r0, r6]
 80076ce: 4433         	add	r3, r6
 80076d0: 5906         	ldr	r6, [r0, r4]
 80076d2: 4433         	add	r3, r6
 80076d4: 5946         	ldr	r6, [r0, r5]
 80076d6: f850 000c    	ldr.w	r0, [r0, r12]
 80076da: 4418         	add	r0, r3
 80076dc: 4430         	add	r0, r6
 80076de: 4450         	add	r0, r10
 80076e0: 4448         	add	r0, r9
 80076e2: 4440         	add	r0, r8
 80076e4: eb0e 0b00    	add.w	r11, lr, r0
 80076e8: f47f af3c    	bne.w	0x8007564 <raylar_crcl_encoder_service::benchmark::sum_only::he5272faae351e30d+0x288> @ imm = #-0x188
 80076ec: f8cd b024    	str.w	r11, [sp, #0x24]
 80076f0: a809         	add	r0, sp, #0x24
 80076f2: f502 31d8    	add.w	r1, r2, #0x1b000
 80076f6: 2400         	movs	r4, #0x0
 80076f8: 9809         	ldr	r0, [sp, #0x24]
 80076fa: f240 3aff    	movw	r10, #0x3ff
 80076fe: 9000         	str	r0, [sp]
 8007700: f851 0f84    	ldr	r0, [r1, #132]!
 8007704: f1ba 0a21    	subs.w	r10, r10, #0x21
 8007708: f1a1 0680    	sub.w	r6, r1, #0x80
 800770c: 9008         	str	r0, [sp, #0x20]
 800770e: e951 5c1d    	ldrd	r5, r12, [r1, #-116]
 8007712: f1a1 0e60    	sub.w	lr, r1, #0x60
 8007716: ce49         	ldm	r6, {r0, r3, r6}
 8007718: 4420         	add	r0, r4
 800771a: f851 2c04    	ldr	r2, [r1, #-4]
 800771e: 4418         	add	r0, r3
 8007720: e951 b81b    	ldrd	r11, r8, [r1, #-108]
 8007724: 4430         	add	r0, r6
 8007726: 9207         	str	r2, [sp, #0x1c]
 8007728: 4428         	add	r0, r5
 800772a: f851 2c08    	ldr	r2, [r1, #-8]
 800772e: 4460         	add	r0, r12
 8007730: 9206         	str	r2, [sp, #0x18]
 8007732: 4458         	add	r0, r11
 8007734: f851 2c0c    	ldr	r2, [r1, #-12]
 8007738: f851 9c64    	ldr	r9, [r1, #-100]
 800773c: 4440         	add	r0, r8
 800773e: 9205         	str	r2, [sp, #0x14]
 8007740: e89e 586c    	ldm.w	lr, {r2, r3, r5, r6, r11, r12, lr}
 8007744: 4448         	add	r0, r9
 8007746: 4410         	add	r0, r2
 8007748: f851 4c44    	ldr	r4, [r1, #-68]
 800774c: 4418         	add	r0, r3
 800774e: 4428         	add	r0, r5
 8007750: 4430         	add	r0, r6
 8007752: f1a1 0640    	sub.w	r6, r1, #0x40
 8007756: 4458         	add	r0, r11
 8007758: f1a1 0b2c    	sub.w	r11, r1, #0x2c
 800775c: 4460         	add	r0, r12
 800775e: ce4c         	ldm	r6, {r2, r3, r6}
 8007760: 4470         	add	r0, lr
 8007762: 4420         	add	r0, r4
 8007764: e951 5c0d    	ldrd	r5, r12, [r1, #-52]
 8007768: 4410         	add	r0, r2
 800776a: e89b 0b00    	ldm.w	r11, {r8, r9, r11}
 800776e: 4418         	add	r0, r3
 8007770: 4430         	add	r0, r6
 8007772: f851 2c20    	ldr	r2, [r1, #-32]
 8007776: 4428         	add	r0, r5
 8007778: f851 3c1c    	ldr	r3, [r1, #-28]
 800777c: 4460         	add	r0, r12
 800777e: f851 6c18    	ldr	r6, [r1, #-24]
 8007782: 4440         	add	r0, r8
 8007784: 4448         	add	r0, r9
 8007786: 4458         	add	r0, r11
 8007788: 4410         	add	r0, r2
 800778a: f851 2c14    	ldr	r2, [r1, #-20]
 800778e: 4418         	add	r0, r3
 8007790: f851 3c10    	ldr	r3, [r1, #-16]
 8007794: 4430         	add	r0, r6
 8007796: 4410         	add	r0, r2
 8007798: 9a05         	ldr	r2, [sp, #0x14]
 800779a: 4418         	add	r0, r3
 800779c: 4410         	add	r0, r2
 800779e: 9a06         	ldr	r2, [sp, #0x18]
 80077a0: 4410         	add	r0, r2
 80077a2: 9a07         	ldr	r2, [sp, #0x1c]
 80077a4: 4410         	add	r0, r2
 80077a6: 9a08         	ldr	r2, [sp, #0x20]
 80077a8: eb02 0400    	add.w	r4, r2, r0
 80077ac: d1a8         	bne	0x8007700 <raylar_crcl_encoder_service::benchmark::sum_only::he5272faae351e30d+0x424> @ imm = #-0xb0
 80077ae: 9409         	str	r4, [sp, #0x24]
 80077b0: a809         	add	r0, sp, #0x24
 80077b2: e9dd 1002    	ldrd	r1, r0, [sp, #8]
 80077b6: 9a00         	ldr	r2, [sp]
 80077b8: 4048         	eors	r0, r1
 80077ba: 9901         	ldr	r1, [sp, #0x4]
 80077bc: 4048         	eors	r0, r1
 80077be: 9909         	ldr	r1, [sp, #0x24]
 80077c0: 4050         	eors	r0, r2
 80077c2: 4048         	eors	r0, r1
 80077c4: b00a         	add	sp, #0x28
 80077c6: e8bd 0f00    	pop.w	{r8, r9, r10, r11}
 80077ca: bdf0         	pop	{r4, r5, r6, r7, pc}

080077cc <raylar_crcl_encoder_service::benchmark::temporal::h98ba012ed5b5e4c9>:
 80077cc: b5f0         	push	{r4, r5, r6, r7, lr}
 80077ce: af03         	add	r7, sp, #0xc
 80077d0: f84d bd04    	str	r11, [sp, #-4]!
 80077d4: b081         	sub	sp, #0x4
 80077d6: f24c 012c    	movw	r1, #0xc02c
 80077da: f500 3290    	add.w	r2, r0, #0x12000
 80077de: 5844         	ldr	r4, [r0, r1]
 80077e0: eb00 0e01    	add.w	lr, r0, r1
 80077e4: f500 3190    	add.w	r1, r0, #0x12000
 80077e8: f102 0324    	add.w	r3, r2, #0x24
 80077ec: f101 0c28    	add.w	r12, r1, #0x28
 80077f0: f240 31ff    	movw	r1, #0x3ff
 80077f4: 4672         	mov	r2, lr
 80077f6: 6995         	ldr	r5, [r2, #0x18]
 80077f8: 390b         	subs	r1, #0xb
 80077fa: eba5 0404    	sub.w	r4, r5, r4
 80077fe: 605c         	str	r4, [r3, #0x4]
 8007800: 6b14         	ldr	r4, [r2, #0x30]
 8007802: eba4 0505    	sub.w	r5, r4, r5
 8007806: 609d         	str	r5, [r3, #0x8]
 8007808: 6c95         	ldr	r5, [r2, #0x48]
 800780a: eba5 0404    	sub.w	r4, r5, r4
 800780e: 60dc         	str	r4, [r3, #0xc]
 8007810: 6e14         	ldr	r4, [r2, #0x60]
 8007812: eba4 0505    	sub.w	r5, r4, r5
 8007816: 611d         	str	r5, [r3, #0x10]
 8007818: 6f95         	ldr	r5, [r2, #0x78]
 800781a: eba5 0404    	sub.w	r4, r5, r4
 800781e: 615c         	str	r4, [r3, #0x14]
 8007820: f8d2 4090    	ldr.w	r4, [r2, #0x90]
 8007824: eba4 0505    	sub.w	r5, r4, r5
 8007828: 619d         	str	r5, [r3, #0x18]
 800782a: f8d2 50a8    	ldr.w	r5, [r2, #0xa8]
 800782e: eba5 0404    	sub.w	r4, r5, r4
 8007832: 61dc         	str	r4, [r3, #0x1c]
 8007834: f8d2 40c0    	ldr.w	r4, [r2, #0xc0]
 8007838: eba4 0505    	sub.w	r5, r4, r5
 800783c: 621d         	str	r5, [r3, #0x20]
 800783e: f8d2 50d8    	ldr.w	r5, [r2, #0xd8]
 8007842: eba5 0404    	sub.w	r4, r5, r4
 8007846: 625c         	str	r4, [r3, #0x24]
 8007848: f8d2 60f0    	ldr.w	r6, [r2, #0xf0]
 800784c: eba6 0405    	sub.w	r4, r6, r5
 8007850: 629c         	str	r4, [r3, #0x28]
 8007852: f8d2 4108    	ldr.w	r4, [r2, #0x108]
 8007856: f502 7284    	add.w	r2, r2, #0x108
 800785a: eba4 0506    	sub.w	r5, r4, r6
 800785e: f843 5f2c    	str	r5, [r3, #44]!
 8007862: d1c8         	bne	0x80077f6 <raylar_crcl_encoder_service::benchmark::temporal::h98ba012ed5b5e4c9+0x2a> @ imm = #-0x70
 8007864: f500 3190    	add.w	r1, r0, #0x12000
 8007868: f240 32ff    	movw	r2, #0x3ff
 800786c: f501 5381    	add.w	r3, r1, #0x1020
 8007870: 4671         	mov	r1, lr
 8007872: f851 4f04    	ldr	r4, [r1, #4]!
 8007876: 698d         	ldr	r5, [r1, #0x18]
 8007878: 3a0b         	subs	r2, #0xb
 800787a: eba5 0404    	sub.w	r4, r5, r4
 800787e: 605c         	str	r4, [r3, #0x4]
 8007880: 6b0c         	ldr	r4, [r1, #0x30]
 8007882: eba4 0505    	sub.w	r5, r4, r5
 8007886: 609d         	str	r5, [r3, #0x8]
 8007888: 6c8d         	ldr	r5, [r1, #0x48]
 800788a: eba5 0404    	sub.w	r4, r5, r4
 800788e: 60dc         	str	r4, [r3, #0xc]
 8007890: 6e0c         	ldr	r4, [r1, #0x60]
 8007892: eba4 0505    	sub.w	r5, r4, r5
 8007896: 611d         	str	r5, [r3, #0x10]
 8007898: 6f8d         	ldr	r5, [r1, #0x78]
 800789a: eba5 0404    	sub.w	r4, r5, r4
 800789e: 615c         	str	r4, [r3, #0x14]
 80078a0: f8d1 4090    	ldr.w	r4, [r1, #0x90]
 80078a4: eba4 0505    	sub.w	r5, r4, r5
 80078a8: 619d         	str	r5, [r3, #0x18]
 80078aa: f8d1 50a8    	ldr.w	r5, [r1, #0xa8]
 80078ae: eba5 0404    	sub.w	r4, r5, r4
 80078b2: 61dc         	str	r4, [r3, #0x1c]
 80078b4: f8d1 40c0    	ldr.w	r4, [r1, #0xc0]
 80078b8: eba4 0505    	sub.w	r5, r4, r5
 80078bc: 621d         	str	r5, [r3, #0x20]
 80078be: f8d1 50d8    	ldr.w	r5, [r1, #0xd8]
 80078c2: eba5 0404    	sub.w	r4, r5, r4
 80078c6: 625c         	str	r4, [r3, #0x24]
 80078c8: f8d1 60f0    	ldr.w	r6, [r1, #0xf0]
 80078cc: eba6 0405    	sub.w	r4, r6, r5
 80078d0: 629c         	str	r4, [r3, #0x28]
 80078d2: f8d1 4108    	ldr.w	r4, [r1, #0x108]
 80078d6: f501 7184    	add.w	r1, r1, #0x108
 80078da: eba4 0506    	sub.w	r5, r4, r6
 80078de: f843 5f2c    	str	r5, [r3, #44]!
 80078e2: d1c8         	bne	0x8007876 <raylar_crcl_encoder_service::benchmark::temporal::h98ba012ed5b5e4c9+0xaa> @ imm = #-0x70
 80078e4: f500 31a0    	add.w	r1, r0, #0x14000
 80078e8: f240 32ff    	movw	r2, #0x3ff
 80078ec: f101 031c    	add.w	r3, r1, #0x1c
 80078f0: 4671         	mov	r1, lr
 80078f2: f851 4f08    	ldr	r4, [r1, #8]!
 80078f6: 698d         	ldr	r5, [r1, #0x18]
 80078f8: 3a0b         	subs	r2, #0xb
 80078fa: eba5 0404    	sub.w	r4, r5, r4
 80078fe: 605c         	str	r4, [r3, #0x4]
 8007900: 6b0c         	ldr	r4, [r1, #0x30]
 8007902: eba4 0505    	sub.w	r5, r4, r5
 8007906: 609d         	str	r5, [r3, #0x8]
 8007908: 6c8d         	ldr	r5, [r1, #0x48]
 800790a: eba5 0404    	sub.w	r4, r5, r4
 800790e: 60dc         	str	r4, [r3, #0xc]
 8007910: 6e0c         	ldr	r4, [r1, #0x60]
 8007912: eba4 0505    	sub.w	r5, r4, r5
 8007916: 611d         	str	r5, [r3, #0x10]
 8007918: 6f8d         	ldr	r5, [r1, #0x78]
 800791a: eba5 0404    	sub.w	r4, r5, r4
 800791e: 615c         	str	r4, [r3, #0x14]
 8007920: f8d1 4090    	ldr.w	r4, [r1, #0x90]
 8007924: eba4 0505    	sub.w	r5, r4, r5
 8007928: 619d         	str	r5, [r3, #0x18]
 800792a: f8d1 50a8    	ldr.w	r5, [r1, #0xa8]
 800792e: eba5 0404    	sub.w	r4, r5, r4
 8007932: 61dc         	str	r4, [r3, #0x1c]
 8007934: f8d1 40c0    	ldr.w	r4, [r1, #0xc0]
 8007938: eba4 0505    	sub.w	r5, r4, r5
 800793c: 621d         	str	r5, [r3, #0x20]
 800793e: f8d1 50d8    	ldr.w	r5, [r1, #0xd8]
 8007942: eba5 0404    	sub.w	r4, r5, r4
 8007946: 625c         	str	r4, [r3, #0x24]
 8007948: f8d1 60f0    	ldr.w	r6, [r1, #0xf0]
 800794c: eba6 0405    	sub.w	r4, r6, r5
 8007950: 629c         	str	r4, [r3, #0x28]
 8007952: f8d1 4108    	ldr.w	r4, [r1, #0x108]
 8007956: f501 7184    	add.w	r1, r1, #0x108
 800795a: eba4 0506    	sub.w	r5, r4, r6
 800795e: f843 5f2c    	str	r5, [r3, #44]!
 8007962: d1c8         	bne	0x80078f6 <raylar_crcl_encoder_service::benchmark::temporal::h98ba012ed5b5e4c9+0x12a> @ imm = #-0x70
 8007964: f500 31a8    	add.w	r1, r0, #0x15000
 8007968: f240 32ff    	movw	r2, #0x3ff
 800796c: f101 0318    	add.w	r3, r1, #0x18
 8007970: 4671         	mov	r1, lr
 8007972: f851 4f0c    	ldr	r4, [r1, #12]!
 8007976: 698d         	ldr	r5, [r1, #0x18]
 8007978: 3a0b         	subs	r2, #0xb
 800797a: eba5 0404    	sub.w	r4, r5, r4
 800797e: 605c         	str	r4, [r3, #0x4]
 8007980: 6b0c         	ldr	r4, [r1, #0x30]
 8007982: eba4 0505    	sub.w	r5, r4, r5
 8007986: 609d         	str	r5, [r3, #0x8]
 8007988: 6c8d         	ldr	r5, [r1, #0x48]
 800798a: eba5 0404    	sub.w	r4, r5, r4
 800798e: 60dc         	str	r4, [r3, #0xc]
 8007990: 6e0c         	ldr	r4, [r1, #0x60]
 8007992: eba4 0505    	sub.w	r5, r4, r5
 8007996: 611d         	str	r5, [r3, #0x10]
 8007998: 6f8d         	ldr	r5, [r1, #0x78]
 800799a: eba5 0404    	sub.w	r4, r5, r4
 800799e: 615c         	str	r4, [r3, #0x14]
 80079a0: f8d1 4090    	ldr.w	r4, [r1, #0x90]
 80079a4: eba4 0505    	sub.w	r5, r4, r5
 80079a8: 619d         	str	r5, [r3, #0x18]
 80079aa: f8d1 50a8    	ldr.w	r5, [r1, #0xa8]
 80079ae: eba5 0404    	sub.w	r4, r5, r4
 80079b2: 61dc         	str	r4, [r3, #0x1c]
 80079b4: f8d1 40c0    	ldr.w	r4, [r1, #0xc0]
 80079b8: eba4 0505    	sub.w	r5, r4, r5
 80079bc: 621d         	str	r5, [r3, #0x20]
 80079be: f8d1 50d8    	ldr.w	r5, [r1, #0xd8]
 80079c2: eba5 0404    	sub.w	r4, r5, r4
 80079c6: 625c         	str	r4, [r3, #0x24]
 80079c8: f8d1 60f0    	ldr.w	r6, [r1, #0xf0]
 80079cc: eba6 0405    	sub.w	r4, r6, r5
 80079d0: 629c         	str	r4, [r3, #0x28]
 80079d2: f8d1 4108    	ldr.w	r4, [r1, #0x108]
 80079d6: f501 7184    	add.w	r1, r1, #0x108
 80079da: eba4 0506    	sub.w	r5, r4, r6
 80079de: f843 5f2c    	str	r5, [r3, #44]!
 80079e2: d1c8         	bne	0x8007976 <raylar_crcl_encoder_service::benchmark::temporal::h98ba012ed5b5e4c9+0x1aa> @ imm = #-0x70
 80079e4: f500 30b0    	add.w	r0, r0, #0x16000
 80079e8: f85e 2f10    	ldr	r2, [lr, #16]!
 80079ec: 3014         	adds	r0, #0x14
 80079ee: f240 31ff    	movw	r1, #0x3ff
 80079f2: f8de 3018    	ldr.w	r3, [lr, #0x18]
 80079f6: 390b         	subs	r1, #0xb
 80079f8: eba3 0202    	sub.w	r2, r3, r2
 80079fc: 6042         	str	r2, [r0, #0x4]
 80079fe: f8de 2030    	ldr.w	r2, [lr, #0x30]
 8007a02: eba2 0303    	sub.w	r3, r2, r3
 8007a06: 6083         	str	r3, [r0, #0x8]
 8007a08: f8de 3048    	ldr.w	r3, [lr, #0x48]
 8007a0c: eba3 0202    	sub.w	r2, r3, r2
 8007a10: 60c2         	str	r2, [r0, #0xc]
 8007a12: f8de 2060    	ldr.w	r2, [lr, #0x60]
 8007a16: eba2 0303    	sub.w	r3, r2, r3
 8007a1a: 6103         	str	r3, [r0, #0x10]
 8007a1c: f8de 3078    	ldr.w	r3, [lr, #0x78]
 8007a20: eba3 0202    	sub.w	r2, r3, r2
 8007a24: 6142         	str	r2, [r0, #0x14]
 8007a26: f8de 2090    	ldr.w	r2, [lr, #0x90]
 8007a2a: eba2 0303    	sub.w	r3, r2, r3
 8007a2e: 6183         	str	r3, [r0, #0x18]
 8007a30: f8de 30a8    	ldr.w	r3, [lr, #0xa8]
 8007a34: eba3 0202    	sub.w	r2, r3, r2
 8007a38: 61c2         	str	r2, [r0, #0x1c]
 8007a3a: f8de 20c0    	ldr.w	r2, [lr, #0xc0]
 8007a3e: eba2 0303    	sub.w	r3, r2, r3
 8007a42: 6203         	str	r3, [r0, #0x20]
 8007a44: f8de 30d8    	ldr.w	r3, [lr, #0xd8]
 8007a48: eba3 0202    	sub.w	r2, r3, r2
 8007a4c: 6242         	str	r2, [r0, #0x24]
 8007a4e: f8de 40f0    	ldr.w	r4, [lr, #0xf0]
 8007a52: eba4 0203    	sub.w	r2, r4, r3
 8007a56: 6282         	str	r2, [r0, #0x28]
 8007a58: f8de 2108    	ldr.w	r2, [lr, #0x108]
 8007a5c: f50e 7e84    	add.w	lr, lr, #0x108
 8007a60: eba2 0304    	sub.w	r3, r2, r4
 8007a64: f840 3f2c    	str	r3, [r0, #44]!
 8007a68: d1c3         	bne	0x80079f2 <raylar_crcl_encoder_service::benchmark::temporal::h98ba012ed5b5e4c9+0x226> @ imm = #-0x7a
 8007a6a: f8cd c000    	str.w	r12, [sp]
 8007a6e: 4668         	mov	r0, sp
 8007a70: f8dc 0000    	ldr.w	r0, [r12]
 8007a74: b001         	add	sp, #0x4
 8007a76: f85d bb04    	ldr	r11, [sp], #4
 8007a7a: bdf0         	pop	{r4, r5, r6, r7, pc}

08007a7c <raylar_crcl_encoder_service::benchmark::pack_copy::ha831ca28ff45cfb7>:
 8007a7c: b5b0         	push	{r4, r5, r7, lr}
 8007a7e: af02         	add	r7, sp, #0x8
 8007a80: b082         	sub	sp, #0x8
 8007a82: 4604         	mov	r4, r0
 8007a84: f64f 40d8    	movw	r0, #0xfcd8
 8007a88: f2c0 0001    	movt	r0, #0x1
 8007a8c: 5821         	ldr	r1, [r4, r0]
 8007a8e: f1a1 0258    	sub.w	r2, r1, #0x58
 8007a92: f5b2 5ff0    	cmp.w	r2, #0x1e00
 8007a96: d819         	bhi	0x8007acc <raylar_crcl_encoder_service::benchmark::pack_copy::ha831ca28ff45cfb7+0x50> @ imm = #0x32
 8007a98: f504 31e8    	add.w	r1, r4, #0x1d000
 8007a9c: 1825         	adds	r5, r4, r0
 8007a9e: f504 30e0    	add.w	r0, r4, #0x1c000
 8007aa2: f501 616b    	add.w	r1, r1, #0xeb0
 8007aa6: 3058         	adds	r0, #0x58
 8007aa8: f005 f8cd    	bl	0x800cc46 <__aeabi_memcpy4> @ imm = #0x519a
 8007aac: 4620         	mov	r0, r4
 8007aae: f7fa ffea    	bl	0x8002a86 <raylar_crcl_encoder_service::benchmark::pack_header::haa3cc0f32481b1e3> @ imm = #-0x502c
 8007ab2: 6829         	ldr	r1, [r5]
 8007ab4: f641 6059    	movw	r0, #0x1e59
 8007ab8: 4281         	cmp	r1, r0
 8007aba: d210         	bhs	0x8007ade <raylar_crcl_encoder_service::benchmark::pack_copy::ha831ca28ff45cfb7+0x62> @ imm = #0x20
 8007abc: f504 30e0    	add.w	r0, r4, #0x1c000
 8007ac0: 9000         	str	r0, [sp]
 8007ac2: 9101         	str	r1, [sp, #0x4]
 8007ac4: 4668         	mov	r0, sp
 8007ac6: 6828         	ldr	r0, [r5]
 8007ac8: b002         	add	sp, #0x8
 8007aca: bdb0         	pop	{r4, r5, r7, pc}
 8007acc: f24e 03c8    	movw	r3, #0xe0c8
 8007ad0: 2058         	movs	r0, #0x58
 8007ad2: f6c0 0300    	movt	r3, #0x800
 8007ad6: f641 6258    	movw	r2, #0x1e58
 8007ada: f004 f9a8    	bl	0x800be2e <core::slice::index::slice_index_fail> @ imm = #0x4350
 8007ade: f24e 03b8    	movw	r3, #0xe0b8
 8007ae2: 2000         	movs	r0, #0x0
 8007ae4: f6c0 0300    	movt	r3, #0x800
 8007ae8: f641 6258    	movw	r2, #0x1e58
 8007aec: f004 f99f    	bl	0x800be2e <core::slice::index::slice_index_fail> @ imm = #0x433e