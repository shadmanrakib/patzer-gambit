.section __TEXT,__text,regular,pure_instructions
	.p2align	2
engine::moves::attacked::square_attacked::is_square_attacked:
Lfunc_begin55:
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/attacked/square_attacked.rs : 123
		pub fn is_square_attacked(
	.cfi_startproc
	stp x29, x30, [sp, #-16]!
	.cfi_def_cfa_offset 16
	mov x29, sp
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	.cfi_remember_state
	and w8, w0, #0xff
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/attacked/square_attacked.rs : 20
		let knight_move_mask = cache.knight_moves_masks[pos as usize];
	sxtb x0, w0
	cmp w8, #63
	b.hi LBB55_4
	ldr x10, [x2, #72]
	add x11, x3, x0, lsl #3
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/attacked/square_attacked.rs : 31
		cache.rook_potential_blockers_masks[pos as usize],
	ldr x8, [x11, #5632]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/precalculate/magic_bitboards.rs : 33
		let blockers_on_path = blockers & potential_blockers_mask;
	and x8, x8, x10
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/attacked/square_attacked.rs : 33
		cache.rook_magics[pos as usize],
	ldr x9, [x11, #6272]
		// /rustc/82e1608dfa6e0b5569232559e3d385fea5a93112/library/core/src/num/mod.rs : 1169
		uint_impl! {
	mul x8, x8, x9
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/attacked/square_attacked.rs : 34
		cache.rook_bit_counts[pos as usize],
	add x13, x3, x0
	mov w9, #6208
	ldrb w9, [x13, x9]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/precalculate/magic_bitboards.rs : 35
		let index = (hash >> (64 - bit_count)) as usize;
	neg w9, w9
	lsr x8, x8, x9
	mov w9, #24
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/attacked/square_attacked.rs : 36
		let rook_moves_mask = cache.rook_magic_attack_tables[pos as usize][rook_magic_index];
	smaddl x12, w0, w9, x3
	ldr x9, [x12, #16]
		// /rustc/82e1608dfa6e0b5569232559e3d385fea5a93112/library/core/src/slice/index.rs : 258
		&(*slice)[self]
	cmp x8, x9
	b.hs LBB55_5
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/attacked/square_attacked.rs : 52
		cache.bishop_potential_blockers_masks[pos as usize],
	ldr x9, [x11, #5120]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/precalculate/magic_bitboards.rs : 33
		let blockers_on_path = blockers & potential_blockers_mask;
	and x9, x9, x10
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/attacked/square_attacked.rs : 54
		cache.bishop_magics[pos as usize],
	ldr x10, [x11, #6784]
		// /rustc/82e1608dfa6e0b5569232559e3d385fea5a93112/library/core/src/num/mod.rs : 1169
		uint_impl! {
	mul x9, x9, x10
	mov w10, #6144
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/attacked/square_attacked.rs : 55
		cache.bishop_bit_counts[pos as usize],
	ldrb w10, [x13, x10]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/precalculate/magic_bitboards.rs : 35
		let index = (hash >> (64 - bit_count)) as usize;
	neg w10, w10
	lsr x9, x9, x10
	mov w10, #24
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/attacked/square_attacked.rs : 57
		let bishop_moves_mask = cache.bishop_magic_attack_tables[pos as usize][bishop_magic_index];
	smaddl x13, w0, w10, x3
	ldr x10, [x13, #1552]
		// /rustc/82e1608dfa6e0b5569232559e3d385fea5a93112/library/core/src/slice/index.rs : 258
		&(*slice)[self]
	cmp x9, x10
	b.hs LBB55_6
	ldr x10, [x11, #4096]
	mov w11, w1
	mov w14, #56
	umaddl x14, w1, w14, x2
	ldr x14, [x14, #96]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/attacked/square_attacked.rs : 57
		let bishop_moves_mask = cache.bishop_magic_attack_tables[pos as usize][bishop_magic_index];
	add x13, x13, #1536
	ldr x13, [x13]
	mov w15, #56
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 88
		return &self.boards[player as usize][piece as usize];
	umaddl x11, w11, w15, x2
		// /rustc/82e1608dfa6e0b5569232559e3d385fea5a93112/library/core/src/ops/bit.rs : 178
		bitand_impl! { bool usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 }
	ldr q0, [x11, #112]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/attacked/square_attacked.rs : 36
		let rook_moves_mask = cache.rook_magic_attack_tables[pos as usize][rook_magic_index];
	ldr x12, [x12]
		// /rustc/82e1608dfa6e0b5569232559e3d385fea5a93112/library/core/src/slice/index.rs : 258
		&(*slice)[self]
	add x8, x12, x8, lsl #3
	add x9, x13, x9, lsl #3
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 88
		return &self.boards[player as usize][piece as usize];
	add x12, x11, #104
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/player.rs : 15
		match self {
	eor w13, w1, #0x1
		// /rustc/82e1608dfa6e0b5569232559e3d385fea5a93112/library/core/src/ops/bit.rs : 178
		bitand_impl! { bool usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 }
	and x10, x14, x10
		// /rustc/82e1608dfa6e0b5569232559e3d385fea5a93112/library/core/src/num/mod.rs : 1169
		uint_impl! {
	fmov d1, x10
	cnt.8b v1, v1
	uaddlv.8b h1, v1
	fmov w10, s1
		// /rustc/82e1608dfa6e0b5569232559e3d385fea5a93112/library/core/src/ops/bit.rs : 178
		bitand_impl! { bool usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 }
	ld1r.2d { v1 }, [x12]
	zip2.2d v1, v0, v1
	ld1r.2d { v2 }, [x8]
	ld1r.2d { v3 }, [x9]
	and.16b v1, v1, v3
	and.16b v0, v0, v2
		// /rustc/82e1608dfa6e0b5569232559e3d385fea5a93112/library/core/src/num/mod.rs : 1169
		uint_impl! {
	cnt.16b v1, v1
	uaddlp.8h v1, v1
	uaddlp.4s v1, v1
	uaddlp.2d v1, v1
	cnt.16b v0, v0
	uaddlp.8h v0, v0
	uaddlp.4s v0, v0
	uaddlp.2d v0, v0
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/attacked/square_attacked.rs : 42
		attacked_count += attacking_rooks.count_ones() as i8;
	uzp1.4s v0, v0, v1
	xtn.4h v0, v0
		// /rustc/82e1608dfa6e0b5569232559e3d385fea5a93112/library/core/src/ops/bit.rs : 178
		bitand_impl! { bool usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 }
	ldr x8, [x11, #88]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/attacked/square_attacked.rs : 115
		let king_move_mask = cache.king_moves_masks[pos as usize];
	add x9, x3, x0, lsl #3
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/attacked/square_attacked.rs : 78
		let attacking_mask = cache.pawn_attack_moves_mask[attacker.opponent() as usize][pos as usize];
	add x12, x9, x13, lsl #9
	ldr x12, [x12, #3072]
		// /rustc/82e1608dfa6e0b5569232559e3d385fea5a93112/library/core/src/ops/bit.rs : 178
		bitand_impl! { bool usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 }
	and x8, x12, x8
		// /rustc/82e1608dfa6e0b5569232559e3d385fea5a93112/library/core/src/num/mod.rs : 1169
		uint_impl! {
	fmov d1, x8
	cnt.8b v1, v1
	uaddlv.8b h1, v1
	fmov w8, s1
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/attacked/square_attacked.rs : 115
		let king_move_mask = cache.king_moves_masks[pos as usize];
	ldr x9, [x9, #4608]
		// /rustc/82e1608dfa6e0b5569232559e3d385fea5a93112/library/core/src/ops/bit.rs : 178
		bitand_impl! { bool usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 }
	ldr x11, [x11, #128]
	and x9, x11, x9
		// /rustc/82e1608dfa6e0b5569232559e3d385fea5a93112/library/core/src/num/mod.rs : 1169
		uint_impl! {
	fmov d1, x9
	cnt.8b v1, v1
	uaddlv.8b h1, v1
	fmov w9, s1
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/attacked/square_attacked.rs : 117
		attacked_count += attacking_king.count_ones() as i8;
	addv.4h h0, v0
	fmov w11, s0
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/attacked/square_attacked.rs : 42
		attacked_count += attacking_rooks.count_ones() as i8;
	add w10, w11, w10
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/attacked/square_attacked.rs : 117
		attacked_count += attacking_king.count_ones() as i8;
	add w8, w8, w9
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/attacked/square_attacked.rs : 80
		attacked_count += attacking_pawns.count_ones() as i8;
	add w8, w10, w8
	sxtb w8, w8
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/attacked/square_attacked.rs : 129
		return times_square_attacked(pos, attacker, game, cache) > 0;
	cmp w8, #0
	cset w0, gt
	.cfi_def_cfa wsp, 16
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/attacked/square_attacked.rs : 213
		}
	ldp x29, x30, [sp], #16
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
LBB55_4:
	.cfi_restore_state
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/attacked/square_attacked.rs : 20
		let knight_move_mask = cache.knight_moves_masks[pos as usize];
Lloh109:
	adrp x2, l___unnamed_35@PAGE
Lloh110:
	add x2, x2, l___unnamed_35@PAGEOFF
	mov w1, #64
	bl core::panicking::panic_bounds_check
LBB55_5:
		// /rustc/82e1608dfa6e0b5569232559e3d385fea5a93112/library/core/src/slice/index.rs : 258
		&(*slice)[self]
Lloh111:
	adrp x2, l___unnamed_36@PAGE
Lloh112:
	add x2, x2, l___unnamed_36@PAGEOFF
	mov x0, x8
	mov x1, x9
	bl core::panicking::panic_bounds_check
LBB55_6:
Lloh113:
	adrp x2, l___unnamed_37@PAGE
Lloh114:
	add x2, x2, l___unnamed_37@PAGEOFF
	mov x0, x9
	mov x1, x10
	bl core::panicking::panic_bounds_check
	.loh AdrpAdd	Lloh109, Lloh110
	.loh AdrpAdd	Lloh111, Lloh112
	.loh AdrpAdd	Lloh113, Lloh114
Lfunc_end55:
	.cfi_endproc

	.p2align	2
engine::moves::perft::_perft_unmake:
Lfunc_begin56:
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/perft.rs : 126
		fn _perft_unmake(game: &mut GameState, cache: &PrecalculatedCache, depth: u16) -> u64 {
	.cfi_startproc
	stp x28, x27, [sp, #-96]!
	.cfi_def_cfa_offset 96
	stp x26, x25, [sp, #16]
	stp x24, x23, [sp, #32]
	stp x22, x21, [sp, #48]
	stp x20, x19, [sp, #64]
	stp x29, x30, [sp, #80]
	add x29, sp, #80
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	.cfi_offset w19, -24
	.cfi_offset w20, -32
	.cfi_offset w21, -40
	.cfi_offset w22, -48
	.cfi_offset w23, -56
	.cfi_offset w24, -64
	.cfi_offset w25, -72
	.cfi_offset w26, -80
	.cfi_offset w27, -88
	.cfi_offset w28, -96
	.cfi_remember_state
	sub sp, sp, #1, lsl #12
	sub sp, sp, #48
	tst w2, #0xffff
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/perft.rs : 129
		if depth == 0 {
	b.eq LBB56_7
	mov x22, x2
	mov x19, x1
	mov x20, x0
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/movelist.rs : 12
		MoveList {
	str xzr, [sp, #4104]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/perft.rs : 137
		game.side_to_move,
	ldrb w2, [x0, #220]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/perft.rs : 134
		super::pseudolegal::all::generate_pseudolegal_moves(
	add x0, sp, #8
	mov x1, x20
	mov x3, x19
	bl engine::moves::pseudolegal::all::generate_pseudolegal_moves
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/movelist.rs : 25
		self.end
	ldr x25, [sp, #4104]
		// /rustc/82e1608dfa6e0b5569232559e3d385fea5a93112/library/core/src/iter/range.rs : 729
		if self.start < self.end {
	cbz x25, LBB56_8
	mov x21, #0
	mov x26, #0
	sub w22, w22, #1
	add x23, sp, #8
	mov w27, #56
	b LBB56_4
LBB56_3:
	add x26, x26, #1
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/perft.rs : 150
		game.unmake_move(&move_item, unmake_metadata);
	sub x2, x29, #112
	mov x0, x20
	mov x1, x23
	bl engine::state::game::GameState::unmake_move
		// /rustc/82e1608dfa6e0b5569232559e3d385fea5a93112/library/core/src/cmp.rs : 1567
		ord_impl! { char usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 }
	add x23, x23, #16
		// /rustc/82e1608dfa6e0b5569232559e3d385fea5a93112/library/core/src/iter/range.rs : 729
		if self.start < self.end {
	cmp x25, x26
	b.eq LBB56_9
LBB56_4:
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/perft.rs : 141
		let move_item = &move_list.moves[index];
	cmp x26, #256
	b.eq LBB56_10
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/perft.rs : 142
		let player = game.side_to_move;
	ldrb w28, [x20, #220]
	cmp x28, #0
	cset w24, eq
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/perft.rs : 143
		let unmake_metadata = game.make_move(move_item);
	sub x0, x29, #112
	mov x1, x20
	mov x2, x23
	bl engine::state::game::GameState::make_move
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 88
		return &self.boards[player as usize][piece as usize];
	umaddl x8, w28, w27, x20
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/attacked/in_check.rs : 10
		let king = game
	ldr x8, [x8, #128]
		// /rustc/82e1608dfa6e0b5569232559e3d385fea5a93112/library/core/src/num/mod.rs : 1169
		uint_impl! {
	rbit x8, x8
	clz x0, x8
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/attacked/in_check.rs : 15
		return is_square_attacked(
	mov x1, x24
	mov x2, x20
	mov x3, x19
	bl engine::moves::attacked::square_attacked::is_square_attacked
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/perft.rs : 145
		if !is_in_check(player, game, cache) {
	tbnz w0, #0, LBB56_3
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/perft.rs : 146
		let move_nodes = _perft_unmake(game, cache, depth - 1);
	mov x0, x20
	mov x1, x19
	mov x2, x22
	bl engine::moves::perft::_perft_unmake
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/perft.rs : 147
		nodes += move_nodes;
	add x21, x0, x21
	b LBB56_3
LBB56_7:
	mov w21, #1
	b LBB56_9
	mov x21, #0
LBB56_9:
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/perft.rs : 154
		}
	mov x0, x21
	add sp, sp, #1, lsl #12
	add sp, sp, #48
	.cfi_def_cfa wsp, 96
	ldp x29, x30, [sp, #80]
	ldp x20, x19, [sp, #64]
	ldp x22, x21, [sp, #48]
	ldp x24, x23, [sp, #32]
	ldp x26, x25, [sp, #16]
	ldp x28, x27, [sp], #96
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	.cfi_restore w19
	.cfi_restore w20
	.cfi_restore w21
	.cfi_restore w22
	.cfi_restore w23
	.cfi_restore w24
	.cfi_restore w25
	.cfi_restore w26
	.cfi_restore w27
	.cfi_restore w28
	ret
LBB56_10:
	.cfi_restore_state
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/perft.rs : 141
		let move_item = &move_list.moves[index];
Lloh115:
	adrp x2, l___unnamed_38@PAGE
Lloh116:
	add x2, x2, l___unnamed_38@PAGEOFF
	mov w0, #256
	mov w1, #256
	bl core::panicking::panic_bounds_check
	.loh AdrpAdd	Lloh115, Lloh116
Lfunc_end56:
	.cfi_endproc

	.p2align	2
engine::state::game::GameState::make_move:
Lfunc_begin57:
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 127
		pub fn make_move(&mut self, move_item: &MoveItem) -> UnmakeMoveMetadata {
	.cfi_startproc
	stp x22, x21, [sp, #-48]!
	.cfi_def_cfa_offset 48
	stp x20, x19, [sp, #16]
	stp x29, x30, [sp, #32]
	add x29, sp, #32
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	.cfi_offset w19, -24
	.cfi_offset w20, -32
	.cfi_offset w21, -40
	.cfi_offset w22, -48
	.cfi_remember_state
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 137
		.remove_piece(self.side_to_move, move_item.from_pos);
	ldrsb x8, [x2, #4]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 113
		let removed = self.pos_to_piece[pos as usize];
	cmp x8, #64
	b.hs LBB57_55
	ldr s0, [x1, #216]
	ldr x11, [x1]
	ldr w12, [x1, #208]
	add x17, x1, #8
	ldrb w16, [x1, #220]
	ldrb w9, [x17, x8]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 114
		self.pos_to_piece[pos as usize] = Piece::Empty;
	strb wzr, [x17, x8]
	mov w10, #56
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 115
		self.boards[player as usize][removed as usize].unset(pos);
	umaddl x10, w16, w10, x17
	add x9, x10, x9, lsl #3
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 31
		*self &= INVERTED_SQUARE_MASKS[index as usize];
Lloh117:
	adrp x3, l___unnamed_25@PAGE
Lloh118:
	add x3, x3, l___unnamed_25@PAGEOFF
	ldr x10, [x3, x8, lsl #3]
	ldr x13, [x9, #72]
	and x13, x13, x10
	str x13, [x9, #72]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 116
		self.pos_to_player[player as usize].unset(pos);
	add x15, x17, x16, lsl #3
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 31
		*self &= INVERTED_SQUARE_MASKS[index as usize];
	ldr x9, [x15, #184]!
	and x9, x9, x10
	str x9, [x15]
	ldr x9, [x1, #72]
	and x9, x9, x10
	str x9, [x1, #72]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/pieces.rs : 14
		#[derive(PartialEq, Debug, Clone, Copy)]
	ldrb w13, [x2, #6]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 139
		if move_item.piece == Piece::Pawn {
	cmp w13, #1
	b.ne LBB57_5
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 140
		let final_piece = if move_item.promoting {
	ldrb w9, [x2, #9]
	ldrb w10, [x2, #7]
	cmp w9, #0
	csinc w4, w10, wzr, ne
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 147
		if move_item.enpassant {
	ldrb w9, [x2, #12]
	cbz w9, LBB57_11
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/square.rs : 40
		8 * self.rank + self.file
	and x10, x8, #0xfffffffffffffff8
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 149
		let to = Square::from(move_item.to_pos);
	ldrsb x9, [x2, #5]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/square.rs : 33
		file: pos % 8,
	ubfx w14, w9, #12, #3
	add w14, w9, w14
	and w14, w14, #0xf8
	sub w14, w9, w14
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/square.rs : 40
		8 * self.rank + self.file
	add x10, x10, w14, sxtb
	and w14, w10, #0xff
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 113
		let removed = self.pos_to_piece[pos as usize];
	cmp w14, #64
	b.hs LBB57_57
	and w14, w9, #0xff
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 137
		.remove_piece(self.side_to_move, move_item.from_pos);
	cmp w16, #0
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 113
		let removed = self.pos_to_piece[pos as usize];
	ldrb w5, [x17, x10]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 114
		self.pos_to_piece[pos as usize] = Piece::Empty;
	strb wzr, [x17, x10]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 137
		.remove_piece(self.side_to_move, move_item.from_pos);
	cset w6, eq
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 115
		self.boards[player as usize][removed as usize].unset(pos);
	add x7, x1, #8
	mov w19, #56
	umaddl x19, w6, w19, x7
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 31
		*self &= INVERTED_SQUARE_MASKS[index as usize];
	ldr x10, [x3, x10, lsl #3]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 115
		self.boards[player as usize][removed as usize].unset(pos);
	add x5, x19, x5, lsl #3
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 31
		*self &= INVERTED_SQUARE_MASKS[index as usize];
	ldr x19, [x5, #72]
	and x19, x19, x10
	str x19, [x5, #72]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 116
		self.pos_to_player[player as usize].unset(pos);
	add x5, x7, w6, uxtw #3
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 31
		*self &= INVERTED_SQUARE_MASKS[index as usize];
	ldr x6, [x5, #184]
	and x6, x6, x10
	str x6, [x5, #184]
	ldr x5, [x1, #72]
	and x10, x5, x10
	str x10, [x1, #72]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 113
		let removed = self.pos_to_piece[pos as usize];
	cmp w14, #64
	b.lo LBB57_13
	b LBB57_54
LBB57_5:
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 167
		.remove_piece(self.side_to_move.opponent(), move_item.to_pos);
	ldrsb x9, [x2, #5]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 113
		let removed = self.pos_to_piece[pos as usize];
	cmp x9, #64
	b.hs LBB57_54
	and w14, w9, #0xff
	and x5, x9, #0xff
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 137
		.remove_piece(self.side_to_move, move_item.from_pos);
	cmp w16, #0
	cset w4, eq
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 113
		let removed = self.pos_to_piece[pos as usize];
	ldrb w10, [x17, x9]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 114
		self.pos_to_piece[pos as usize] = Piece::Empty;
	strb wzr, [x17, x9]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 115
		self.boards[player as usize][removed as usize].unset(pos);
	add x6, x1, #80
	mov w7, #56
	umaddl x19, w4, w7, x6
	lsl x20, x10, #3
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 31
		*self &= INVERTED_SQUARE_MASKS[index as usize];
	ldr x3, [x3, x9, lsl #3]
	ldr x21, [x19, x20]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 115
		self.boards[player as usize][removed as usize].unset(pos);
	add x10, x1, w4, uxtw #3
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 31
		*self &= INVERTED_SQUARE_MASKS[index as usize];
	and x21, x21, x3
	str x21, [x19, x20]
	ldr x20, [x10, #192]!
	and x20, x20, x3
	ldr x21, [x1, #72]
	and x21, x21, x3
	str x21, [x1, #72]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 113
		let removed = self.pos_to_piece[pos as usize];
	ldrb w21, [x17, x9]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 114
		self.pos_to_piece[pos as usize] = Piece::Empty;
	strb wzr, [x17, x9]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 115
		self.boards[player as usize][removed as usize].unset(pos);
	lsl x21, x21, #3
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 31
		*self &= INVERTED_SQUARE_MASKS[index as usize];
	ldr x22, [x19, x21]
	and x22, x22, x3
	str x22, [x19, x21]
	str x20, [x10]
	ldr x19, [x1, #72]
	and x3, x19, x3
	str x3, [x1, #72]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 95
		self.boards[player as usize][piece as usize].set(pos);
	umaddl x3, w16, w7, x6
	lsl x6, x13, #3
	mov w7, #1
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 21
		*self |= 1 << index; // SQUARE_MASKS[index as usize];
	lsl x5, x7, x5
	ldr x7, [x3, x6]
	orr x7, x7, x5
	str x7, [x3, x6]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 96
		self.pos_to_piece[pos as usize] = piece;
	strb w13, [x17, x9]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 21
		*self |= 1 << index; // SQUARE_MASKS[index as usize];
	ldr x9, [x15]
	orr x17, x9, x5
	str x17, [x15]
	ldr x9, [x1, #72]
	orr x9, x9, x5
	str x9, [x1, #72]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 172
		if move_item.castling {
	ldrb w3, [x2, #13]
	cbz w3, LBB57_14
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 174
		match (self.side_to_move, move_item.to_pos) {
	cbz w16, LBB57_15
	cmp w14, #58
	b.eq LBB57_19
	cmp w14, #62
	b.ne LBB57_18
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 113
		let removed = self.pos_to_piece[pos as usize];
	ldrb w14, [x1, #71]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 114
		self.pos_to_piece[pos as usize] = Piece::Empty;
	strb wzr, [x1, #71]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 115
		self.boards[player as usize][removed as usize].unset(pos);
	add x3, x1, #80
	mov w5, #56
	umaddl x16, w16, w5, x3
	lsl x14, x14, #3
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 31
		*self &= INVERTED_SQUARE_MASKS[index as usize];
	ldr x6, [x16, x14]
	and x6, x6, #0x7fffffffffffffff
	str x6, [x16, x14]
	and x14, x17, #0x7fffffffffffffff
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 113
		let removed = self.pos_to_piece[pos as usize];
	ldrb w17, [x1, #69]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 115
		self.boards[player as usize][removed as usize].unset(pos);
	umaddl x3, w4, w5, x3
	lsl x17, x17, #3
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 31
		*self &= INVERTED_SQUARE_MASKS[index as usize];
	ldr x4, [x3, x17]
	and x4, x4, #0xdfffffffffffffff
	str x4, [x3, x17]
	str x14, [x15]
	ldr x14, [x10]
	and x14, x14, #0xdfffffffffffffff
	str x14, [x10]
	and x9, x9, #0x7fffffffffffffff
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 21
		*self |= 1 << index; // SQUARE_MASKS[index as usize];
	ldr x10, [x16, #32]
	orr x10, x10, #0x2000000000000000
	str x10, [x16, #32]
	mov w10, #4
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 96
		self.pos_to_piece[pos as usize] = piece;
	strb w10, [x1, #69]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 21
		*self |= 1 << index; // SQUARE_MASKS[index as usize];
	ldr x10, [x15]
	orr x10, x10, #0x2000000000000000
	str x10, [x15]
	orr x9, x9, #0x2000000000000000
	str x9, [x1, #72]
	mov w10, #1
	mov w14, #62
	b LBB57_21
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 160
		.remove_piece(self.side_to_move.opponent(), move_item.to_pos);
	ldrsb x9, [x2, #5]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 113
		let removed = self.pos_to_piece[pos as usize];
	cmp x9, #64
	b.hs LBB57_54
	and w14, w9, #0xff
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 137
		.remove_piece(self.side_to_move, move_item.from_pos);
	cmp w16, #0
	cset w10, eq
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 113
		let removed = self.pos_to_piece[pos as usize];
	ldrb w5, [x17, x9]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 114
		self.pos_to_piece[pos as usize] = Piece::Empty;
	strb wzr, [x17, x9]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 115
		self.boards[player as usize][removed as usize].unset(pos);
	add x6, x1, #8
	mov w7, #56
	umaddl x7, w10, w7, x6
	add x5, x7, x5, lsl #3
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 31
		*self &= INVERTED_SQUARE_MASKS[index as usize];
	ldr x7, [x3, x9, lsl #3]
	ldr x19, [x5, #72]
	and x19, x19, x7
	str x19, [x5, #72]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 116
		self.pos_to_player[player as usize].unset(pos);
	add x10, x6, w10, uxtw #3
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 31
		*self &= INVERTED_SQUARE_MASKS[index as usize];
	ldr x5, [x10, #184]
	and x5, x5, x7
	str x5, [x10, #184]
	ldr x10, [x1, #72]
	and x10, x10, x7
	str x10, [x1, #72]
LBB57_13:
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 137
		.remove_piece(self.side_to_move, move_item.from_pos);
	cmp w16, #0
	cset w10, eq
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 113
		let removed = self.pos_to_piece[pos as usize];
	ldrb w5, [x17, x9]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 114
		self.pos_to_piece[pos as usize] = Piece::Empty;
	strb wzr, [x17, x9]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 115
		self.boards[player as usize][removed as usize].unset(pos);
	add x6, x1, #80
	mov w7, #56
	umaddl x19, w10, w7, x6
	add x10, x1, w10, uxtw #3
	lsl x5, x5, #3
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 31
		*self &= INVERTED_SQUARE_MASKS[index as usize];
	ldr x3, [x3, x9, lsl #3]
	ldr x20, [x19, x5]
	and x20, x20, x3
	str x20, [x19, x5]
	ldr x5, [x10, #192]
	and x5, x5, x3
	str x5, [x10, #192]
	ldr x10, [x1, #72]
	and x10, x10, x3
	str x10, [x1, #72]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 95
		self.boards[player as usize][piece as usize].set(pos);
	and x10, x4, #0xff
	umaddl x16, w16, w7, x6
	lsl x10, x10, #3
	mov w3, #1
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 21
		*self |= 1 << index; // SQUARE_MASKS[index as usize];
	lsl x3, x3, x14
	ldr x5, [x16, x10]
	orr x5, x5, x3
	str x5, [x16, x10]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 96
		self.pos_to_piece[pos as usize] = piece;
	strb w4, [x17, x9]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 21
		*self |= 1 << index; // SQUARE_MASKS[index as usize];
	ldr x9, [x15]
	orr x9, x9, x3
	str x9, [x15]
	ldr x9, [x1, #72]
	orr x9, x9, x3
	str x9, [x1, #72]
	ldrb w10, [x2, #13]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 206
		if !move_item.capturing && !move_item.castling && move_item.piece != Piece::Pawn {
	ldr w9, [x1, #208]
	cmp w10, #0
	cset w10, ne
	b LBB57_22
	mov w10, #0
	b LBB57_21
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 174
		match (self.side_to_move, move_item.to_pos) {
	cmp w14, #2
	b.eq LBB57_20
	cmp w14, #6
	b.ne LBB57_18
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 113
		let removed = self.pos_to_piece[pos as usize];
	ldrb w14, [x1, #15]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 114
		self.pos_to_piece[pos as usize] = Piece::Empty;
	strb wzr, [x1, #15]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 115
		self.boards[player as usize][removed as usize].unset(pos);
	add x3, x1, #80
	mov w5, #56
	umaddl x16, w16, w5, x3
	lsl x14, x14, #3
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 31
		*self &= INVERTED_SQUARE_MASKS[index as usize];
	ldr x6, [x16, x14]
	and x6, x6, #0xffffffffffffff7f
	str x6, [x16, x14]
	and x14, x17, #0xffffffffffffff7f
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 113
		let removed = self.pos_to_piece[pos as usize];
	ldrb w17, [x1, #13]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 115
		self.boards[player as usize][removed as usize].unset(pos);
	umaddl x3, w4, w5, x3
	lsl x17, x17, #3
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 31
		*self &= INVERTED_SQUARE_MASKS[index as usize];
	ldr x4, [x3, x17]
	and x4, x4, #0xffffffffffffffdf
	str x4, [x3, x17]
	str x14, [x15]
	ldr x14, [x10]
	and x14, x14, #0xffffffffffffffdf
	str x14, [x10]
	and x9, x9, #0xffffffffffffff7f
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 21
		*self |= 1 << index; // SQUARE_MASKS[index as usize];
	ldr x10, [x16, #32]
	orr x10, x10, #0x20
	str x10, [x16, #32]
	mov w10, #4
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 96
		self.pos_to_piece[pos as usize] = piece;
	strb w10, [x1, #13]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 21
		*self |= 1 << index; // SQUARE_MASKS[index as usize];
	ldr x10, [x15]
	orr x10, x10, #0x20
	str x10, [x15]
	orr x9, x9, #0x20
	str x9, [x1, #72]
	mov w10, #1
	mov w14, #6
	b LBB57_21
LBB57_18:
	mov w10, #1
	b LBB57_21
LBB57_19:
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 113
		let removed = self.pos_to_piece[pos as usize];
	ldrb w14, [x1, #64]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 114
		self.pos_to_piece[pos as usize] = Piece::Empty;
	strb wzr, [x1, #64]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 115
		self.boards[player as usize][removed as usize].unset(pos);
	add x3, x1, #80
	mov w5, #56
	umaddl x16, w16, w5, x3
	lsl x14, x14, #3
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 31
		*self &= INVERTED_SQUARE_MASKS[index as usize];
	ldr x6, [x16, x14]
	and x6, x6, #0xfeffffffffffffff
	str x6, [x16, x14]
	and x14, x17, #0xfeffffffffffffff
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 113
		let removed = self.pos_to_piece[pos as usize];
	ldrb w17, [x1, #67]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 115
		self.boards[player as usize][removed as usize].unset(pos);
	umaddl x3, w4, w5, x3
	lsl x17, x17, #3
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 31
		*self &= INVERTED_SQUARE_MASKS[index as usize];
	ldr x4, [x3, x17]
	and x4, x4, #0xf7ffffffffffffff
	str x4, [x3, x17]
	str x14, [x15]
	ldr x14, [x10]
	and x14, x14, #0xf7ffffffffffffff
	str x14, [x10]
	and x9, x9, #0xfeffffffffffffff
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 21
		*self |= 1 << index; // SQUARE_MASKS[index as usize];
	ldr x10, [x16, #32]
	orr x10, x10, #0x800000000000000
	str x10, [x16, #32]
	mov w10, #4
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 96
		self.pos_to_piece[pos as usize] = piece;
	strb w10, [x1, #67]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 21
		*self |= 1 << index; // SQUARE_MASKS[index as usize];
	ldr x10, [x15]
	orr x10, x10, #0x800000000000000
	str x10, [x15]
	orr x9, x9, #0x800000000000000
	str x9, [x1, #72]
	mov w10, #1
	mov w14, #58
	b LBB57_21
LBB57_20:
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 113
		let removed = self.pos_to_piece[pos as usize];
	ldrb w14, [x1, #8]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 114
		self.pos_to_piece[pos as usize] = Piece::Empty;
	strb wzr, [x1, #8]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 115
		self.boards[player as usize][removed as usize].unset(pos);
	add x3, x1, #80
	mov w5, #56
	umaddl x16, w16, w5, x3
	lsl x14, x14, #3
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 31
		*self &= INVERTED_SQUARE_MASKS[index as usize];
	ldr x6, [x16, x14]
	and x6, x6, #0xfffffffffffffffe
	str x6, [x16, x14]
	and x14, x17, #0xfffffffffffffffe
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 113
		let removed = self.pos_to_piece[pos as usize];
	ldrb w17, [x1, #11]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 115
		self.boards[player as usize][removed as usize].unset(pos);
	umaddl x3, w4, w5, x3
	lsl x17, x17, #3
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 31
		*self &= INVERTED_SQUARE_MASKS[index as usize];
	ldr x4, [x3, x17]
	and x4, x4, #0xfffffffffffffff7
	str x4, [x3, x17]
	str x14, [x15]
	ldr x14, [x10]
	and x14, x14, #0xfffffffffffffff7
	str x14, [x10]
	and x9, x9, #0xfffffffffffffffe
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 21
		*self |= 1 << index; // SQUARE_MASKS[index as usize];
	ldr x10, [x16, #32]
	orr x10, x10, #0x8
	str x10, [x16, #32]
	mov w10, #4
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 96
		self.pos_to_piece[pos as usize] = piece;
	strb w10, [x1, #11]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 21
		*self |= 1 << index; // SQUARE_MASKS[index as usize];
	ldr x10, [x15]
	orr x10, x10, #0x8
	str x10, [x15]
	orr x9, x9, #0x8
	str x9, [x1, #72]
	mov w10, #1
	mov w14, #2
LBB57_21:
	mov x9, x12
LBB57_22:
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 206
		if !move_item.capturing && !move_item.castling && move_item.piece != Piece::Pawn {
	ldrb w15, [x2, #10]
	cmp w15, #0
	csinc w15, w10, wzr, eq
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 139
		if move_item.piece == Piece::Pawn {
	cmp w13, #1
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 206
		if !move_item.capturing && !move_item.castling && move_item.piece != Piece::Pawn {
	csinc w15, w15, wzr, ne
	cmp w15, #0
	csinc w9, wzr, w9, ne
	str w9, [x1, #208]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 213
		if move_item.double {
	ldrb w9, [x2, #11]
	cbz w9, LBB57_27
	and w9, w8, #0xf8
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/square.rs : 32
		rank: pos / 8,
	lsr w15, w14, #3
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 223
		if to_rank > from_rank {
	cmp w15, w9, lsr #3
	add w9, w15, #31
	csinc w9, w9, w15, hi
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/square.rs : 40
		8 * self.rank + self.file
	mov x15, x14
	bfi w15, w9, #3, #29
	and w16, w15, #0xff
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 229
		self.enpassant_square = SQUARE_MASKS[<Square as Into<i8>>::into(Square {
	sxtb x9, w15
	cmp w16, #64
	b.hs LBB57_56
Lloh119:
	adrp x15, l___unnamed_39@PAGE
Lloh120:
	add x15, x15, l___unnamed_39@PAGEOFF
	ldr x9, [x15, x9, lsl #3]
	ushll.8h v0, v0, #0
	str x9, [x1]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/player.rs : 1
		#[derive(Debug, PartialEq, Clone, Copy)]
	ldrb w9, [x1, #220]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 239
		if self.side_to_move == Player::Black {
	cbz w9, LBB57_28
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 240
		self.full_move_number += 1;
	ldr w15, [x1, #212]
	add w15, w15, #1
	str w15, [x1, #212]
	cmp w13, #6
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 245
		if move_item.castling || move_item.piece == Piece::King {
	csinc w10, w10, wzr, ne
	tbz w10, #0, LBB57_35
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 30
		self.black_queen_side = false;
	strh wzr, [x1, #218]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 255
		if move_item.piece == Piece::Rook {
	cmp w13, #4
	b.eq LBB57_36
	b LBB57_41
	mov x9, #0
	ushll.8h v0, v0, #0
	str x9, [x1]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/player.rs : 1
		#[derive(Debug, PartialEq, Clone, Copy)]
	ldrb w9, [x1, #220]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 239
		if self.side_to_move == Player::Black {
	cbnz w9, LBB57_25
	cmp w13, #6
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 245
		if move_item.castling || move_item.piece == Piece::King {
	csinc w10, w10, wzr, ne
	tbz w10, #0, LBB57_30
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 26
		self.white_queen_side = false;
	strh wzr, [x1, #216]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 255
		if move_item.piece == Piece::Rook {
	cmp w13, #4
	b.ne LBB57_41
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 256
		match (self.side_to_move, move_item.from_pos) {
	cbnz w9, LBB57_36
	cbz w8, LBB57_53
	and w8, w8, #0xff
	cmp w8, #7
	b.ne LBB57_41
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 261
		self.castle_permissions.white_king_side = false;
	strb wzr, [x1, #217]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/pieces.rs : 14
		#[derive(PartialEq, Debug, Clone, Copy)]
	ldrb w8, [x2, #8]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 272
		if move_item.captured_piece == Piece::Rook {
	cmp w8, #4
	b.eq LBB57_42
	b LBB57_52
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 255
		if move_item.piece == Piece::Rook {
	cmp w13, #4
	b.ne LBB57_39
LBB57_36:
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 256
		match (self.side_to_move, move_item.from_pos) {
	cmp w8, #56
	b.eq LBB57_40
	and w8, w8, #0xff
	cmp w8, #63
	b.ne LBB57_41
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 267
		self.castle_permissions.black_king_side = false;
	strb wzr, [x1, #219]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/pieces.rs : 14
		#[derive(PartialEq, Debug, Clone, Copy)]
	ldrb w8, [x2, #8]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 272
		if move_item.captured_piece == Piece::Rook {
	cmp w8, #4
	b.eq LBB57_42
	b LBB57_52
LBB57_39:
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/pieces.rs : 14
		#[derive(PartialEq, Debug, Clone, Copy)]
	ldrb w8, [x2, #8]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 272
		if move_item.captured_piece == Piece::Rook {
	cmp w8, #4
	b.eq LBB57_43
	b LBB57_52
LBB57_40:
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 264
		self.castle_permissions.black_queen_side = false;
	strb wzr, [x1, #218]
LBB57_41:
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/pieces.rs : 14
		#[derive(PartialEq, Debug, Clone, Copy)]
	ldrb w8, [x2, #8]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 272
		if move_item.captured_piece == Piece::Rook {
	cmp w8, #4
	b.ne LBB57_52
LBB57_42:
	cbz w9, LBB57_46
LBB57_43:
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 273
		match (self.side_to_move.opponent(), move_item.to_pos) {
	cbz w14, LBB57_49
	cmp w14, #7
	b.ne LBB57_51
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 278
		self.castle_permissions.white_king_side = false;
	strb wzr, [x1, #217]
	b LBB57_51
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 273
		match (self.side_to_move.opponent(), move_item.to_pos) {
	cmp w14, #56
	b.eq LBB57_50
	cmp w14, #63
	b.ne LBB57_51
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 284
		self.castle_permissions.black_king_side = false;
	strb wzr, [x1, #219]
	b LBB57_51
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 275
		self.castle_permissions.white_queen_side = false;
	strb wzr, [x1, #216]
	b LBB57_51
LBB57_50:
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 281
		self.castle_permissions.black_queen_side = false;
	strb wzr, [x1, #218]
LBB57_51:
	mov w8, #4
LBB57_52:
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 291
		self.side_to_move = self.side_to_move.opponent();
	eor w9, w9, #0x1
	strb w9, [x1, #220]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 293
		UnmakeMoveMetadata {
	xtn.8b v0, v0
	str s0, [x0, #8]
	str x11, [x0]
	str w12, [x0, #12]
	strb w8, [x0, #16]
	.cfi_def_cfa wsp, 48
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 299
		}
	ldp x29, x30, [sp, #32]
	ldp x20, x19, [sp, #16]
	ldp x22, x21, [sp], #48
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	.cfi_restore w19
	.cfi_restore w20
	.cfi_restore w21
	.cfi_restore w22
	ret
	.cfi_restore_state
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 258
		self.castle_permissions.white_queen_side = false;
	strb wzr, [x1, #216]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/pieces.rs : 14
		#[derive(PartialEq, Debug, Clone, Copy)]
	ldrb w8, [x2, #8]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 272
		if move_item.captured_piece == Piece::Rook {
	cmp w8, #4
	b.eq LBB57_42
	b LBB57_52
LBB57_54:
Lloh121:
	adrp x2, l___unnamed_40@PAGE
Lloh122:
	add x2, x2, l___unnamed_40@PAGEOFF
	mov x0, x9
	mov w1, #64
	bl core::panicking::panic_bounds_check
LBB57_55:
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 113
		let removed = self.pos_to_piece[pos as usize];
Lloh123:
	adrp x2, l___unnamed_40@PAGE
Lloh124:
	add x2, x2, l___unnamed_40@PAGEOFF
	mov x0, x8
	mov w1, #64
	bl core::panicking::panic_bounds_check
LBB57_56:
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 229
		self.enpassant_square = SQUARE_MASKS[<Square as Into<i8>>::into(Square {
Lloh125:
	adrp x2, l___unnamed_41@PAGE
Lloh126:
	add x2, x2, l___unnamed_41@PAGEOFF
	mov x0, x9
	mov w1, #64
	bl core::panicking::panic_bounds_check
LBB57_57:
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 113
		let removed = self.pos_to_piece[pos as usize];
Lloh127:
	adrp x2, l___unnamed_40@PAGE
Lloh128:
	add x2, x2, l___unnamed_40@PAGEOFF
	mov x0, x10
	mov w1, #64
	bl core::panicking::panic_bounds_check
	.loh AdrpAdd	Lloh117, Lloh118
	.loh AdrpAdd	Lloh119, Lloh120
	.loh AdrpAdd	Lloh121, Lloh122
	.loh AdrpAdd	Lloh123, Lloh124
	.loh AdrpAdd	Lloh125, Lloh126
	.loh AdrpAdd	Lloh127, Lloh128
.section __TEXT,__text,regular,pure_instructions
	.p2align	2
engine::moves::attacked::square_attacked::is_square_attacked:
Lfunc_begin55:
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/attacked/square_attacked.rs : 123
		pub fn is_square_attacked(
	.cfi_startproc
	stp x29, x30, [sp, #-16]!
	.cfi_def_cfa_offset 16
	mov x29, sp
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	.cfi_remember_state
	and w8, w0, #0xff
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/attacked/square_attacked.rs : 20
		let knight_move_mask = cache.knight_moves_masks[pos as usize];
	sxtb x0, w0
	cmp w8, #63
	b.hi LBB55_4
	ldr x10, [x2, #72]
	add x11, x3, x0, lsl #3
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/attacked/square_attacked.rs : 31
		cache.rook_potential_blockers_masks[pos as usize],
	ldr x8, [x11, #5632]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/precalculate/magic_bitboards.rs : 33
		let blockers_on_path = blockers & potential_blockers_mask;
	and x8, x8, x10
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/attacked/square_attacked.rs : 33
		cache.rook_magics[pos as usize],
	ldr x9, [x11, #6272]
		// /rustc/82e1608dfa6e0b5569232559e3d385fea5a93112/library/core/src/num/mod.rs : 1169
		uint_impl! {
	mul x8, x8, x9
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/attacked/square_attacked.rs : 34
		cache.rook_bit_counts[pos as usize],
	add x13, x3, x0
	mov w9, #6208
	ldrb w9, [x13, x9]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/precalculate/magic_bitboards.rs : 35
		let index = (hash >> (64 - bit_count)) as usize;
	neg w9, w9
	lsr x8, x8, x9
	mov w9, #24
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/attacked/square_attacked.rs : 36
		let rook_moves_mask = cache.rook_magic_attack_tables[pos as usize][rook_magic_index];
	smaddl x12, w0, w9, x3
	ldr x9, [x12, #16]
		// /rustc/82e1608dfa6e0b5569232559e3d385fea5a93112/library/core/src/slice/index.rs : 258
		&(*slice)[self]
	cmp x8, x9
	b.hs LBB55_5
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/attacked/square_attacked.rs : 52
		cache.bishop_potential_blockers_masks[pos as usize],
	ldr x9, [x11, #5120]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/precalculate/magic_bitboards.rs : 33
		let blockers_on_path = blockers & potential_blockers_mask;
	and x9, x9, x10
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/attacked/square_attacked.rs : 54
		cache.bishop_magics[pos as usize],
	ldr x10, [x11, #6784]
		// /rustc/82e1608dfa6e0b5569232559e3d385fea5a93112/library/core/src/num/mod.rs : 1169
		uint_impl! {
	mul x9, x9, x10
	mov w10, #6144
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/attacked/square_attacked.rs : 55
		cache.bishop_bit_counts[pos as usize],
	ldrb w10, [x13, x10]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/precalculate/magic_bitboards.rs : 35
		let index = (hash >> (64 - bit_count)) as usize;
	neg w10, w10
	lsr x9, x9, x10
	mov w10, #24
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/attacked/square_attacked.rs : 57
		let bishop_moves_mask = cache.bishop_magic_attack_tables[pos as usize][bishop_magic_index];
	smaddl x13, w0, w10, x3
	ldr x10, [x13, #1552]
		// /rustc/82e1608dfa6e0b5569232559e3d385fea5a93112/library/core/src/slice/index.rs : 258
		&(*slice)[self]
	cmp x9, x10
	b.hs LBB55_6
	ldr x10, [x11, #4096]
	mov w11, w1
	mov w14, #56
	umaddl x14, w1, w14, x2
	ldr x14, [x14, #96]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/attacked/square_attacked.rs : 57
		let bishop_moves_mask = cache.bishop_magic_attack_tables[pos as usize][bishop_magic_index];
	add x13, x13, #1536
	ldr x13, [x13]
	mov w15, #56
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 88
		return &self.boards[player as usize][piece as usize];
	umaddl x11, w11, w15, x2
		// /rustc/82e1608dfa6e0b5569232559e3d385fea5a93112/library/core/src/ops/bit.rs : 178
		bitand_impl! { bool usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 }
	ldr q0, [x11, #112]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/attacked/square_attacked.rs : 36
		let rook_moves_mask = cache.rook_magic_attack_tables[pos as usize][rook_magic_index];
	ldr x12, [x12]
		// /rustc/82e1608dfa6e0b5569232559e3d385fea5a93112/library/core/src/slice/index.rs : 258
		&(*slice)[self]
	add x8, x12, x8, lsl #3
	add x9, x13, x9, lsl #3
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 88
		return &self.boards[player as usize][piece as usize];
	add x12, x11, #104
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/player.rs : 15
		match self {
	eor w13, w1, #0x1
		// /rustc/82e1608dfa6e0b5569232559e3d385fea5a93112/library/core/src/ops/bit.rs : 178
		bitand_impl! { bool usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 }
	and x10, x14, x10
		// /rustc/82e1608dfa6e0b5569232559e3d385fea5a93112/library/core/src/num/mod.rs : 1169
		uint_impl! {
	fmov d1, x10
	cnt.8b v1, v1
	uaddlv.8b h1, v1
	fmov w10, s1
		// /rustc/82e1608dfa6e0b5569232559e3d385fea5a93112/library/core/src/ops/bit.rs : 178
		bitand_impl! { bool usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 }
	ld1r.2d { v1 }, [x12]
	zip2.2d v1, v0, v1
	ld1r.2d { v2 }, [x8]
	ld1r.2d { v3 }, [x9]
	and.16b v1, v1, v3
	and.16b v0, v0, v2
		// /rustc/82e1608dfa6e0b5569232559e3d385fea5a93112/library/core/src/num/mod.rs : 1169
		uint_impl! {
	cnt.16b v1, v1
	uaddlp.8h v1, v1
	uaddlp.4s v1, v1
	uaddlp.2d v1, v1
	cnt.16b v0, v0
	uaddlp.8h v0, v0
	uaddlp.4s v0, v0
	uaddlp.2d v0, v0
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/attacked/square_attacked.rs : 42
		attacked_count += attacking_rooks.count_ones() as i8;
	uzp1.4s v0, v0, v1
	xtn.4h v0, v0
		// /rustc/82e1608dfa6e0b5569232559e3d385fea5a93112/library/core/src/ops/bit.rs : 178
		bitand_impl! { bool usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 }
	ldr x8, [x11, #88]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/attacked/square_attacked.rs : 115
		let king_move_mask = cache.king_moves_masks[pos as usize];
	add x9, x3, x0, lsl #3
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/attacked/square_attacked.rs : 78
		let attacking_mask = cache.pawn_attack_moves_mask[attacker.opponent() as usize][pos as usize];
	add x12, x9, x13, lsl #9
	ldr x12, [x12, #3072]
		// /rustc/82e1608dfa6e0b5569232559e3d385fea5a93112/library/core/src/ops/bit.rs : 178
		bitand_impl! { bool usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 }
	and x8, x12, x8
		// /rustc/82e1608dfa6e0b5569232559e3d385fea5a93112/library/core/src/num/mod.rs : 1169
		uint_impl! {
	fmov d1, x8
	cnt.8b v1, v1
	uaddlv.8b h1, v1
	fmov w8, s1
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/attacked/square_attacked.rs : 115
		let king_move_mask = cache.king_moves_masks[pos as usize];
	ldr x9, [x9, #4608]
		// /rustc/82e1608dfa6e0b5569232559e3d385fea5a93112/library/core/src/ops/bit.rs : 178
		bitand_impl! { bool usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 }
	ldr x11, [x11, #128]
	and x9, x11, x9
		// /rustc/82e1608dfa6e0b5569232559e3d385fea5a93112/library/core/src/num/mod.rs : 1169
		uint_impl! {
	fmov d1, x9
	cnt.8b v1, v1
	uaddlv.8b h1, v1
	fmov w9, s1
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/attacked/square_attacked.rs : 117
		attacked_count += attacking_king.count_ones() as i8;
	addv.4h h0, v0
	fmov w11, s0
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/attacked/square_attacked.rs : 42
		attacked_count += attacking_rooks.count_ones() as i8;
	add w10, w11, w10
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/attacked/square_attacked.rs : 117
		attacked_count += attacking_king.count_ones() as i8;
	add w8, w8, w9
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/attacked/square_attacked.rs : 80
		attacked_count += attacking_pawns.count_ones() as i8;
	add w8, w10, w8
	sxtb w8, w8
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/attacked/square_attacked.rs : 129
		return times_square_attacked(pos, attacker, game, cache) > 0;
	cmp w8, #0
	cset w0, gt
	.cfi_def_cfa wsp, 16
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/attacked/square_attacked.rs : 213
		}
	ldp x29, x30, [sp], #16
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
LBB55_4:
	.cfi_restore_state
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/attacked/square_attacked.rs : 20
		let knight_move_mask = cache.knight_moves_masks[pos as usize];
Lloh109:
	adrp x2, l___unnamed_35@PAGE
Lloh110:
	add x2, x2, l___unnamed_35@PAGEOFF
	mov w1, #64
	bl core::panicking::panic_bounds_check
LBB55_5:
		// /rustc/82e1608dfa6e0b5569232559e3d385fea5a93112/library/core/src/slice/index.rs : 258
		&(*slice)[self]
Lloh111:
	adrp x2, l___unnamed_36@PAGE
Lloh112:
	add x2, x2, l___unnamed_36@PAGEOFF
	mov x0, x8
	mov x1, x9
	bl core::panicking::panic_bounds_check
LBB55_6:
Lloh113:
	adrp x2, l___unnamed_37@PAGE
Lloh114:
	add x2, x2, l___unnamed_37@PAGEOFF
	mov x0, x9
	mov x1, x10
	bl core::panicking::panic_bounds_check
	.loh AdrpAdd	Lloh109, Lloh110
	.loh AdrpAdd	Lloh111, Lloh112
	.loh AdrpAdd	Lloh113, Lloh114
Lfunc_end55:
	.cfi_endproc

	.p2align	2
engine::moves::perft::_perft_unmake:
Lfunc_begin56:
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/perft.rs : 126
		fn _perft_unmake(game: &mut GameState, cache: &PrecalculatedCache, depth: u16) -> u64 {
	.cfi_startproc
	stp x28, x27, [sp, #-96]!
	.cfi_def_cfa_offset 96
	stp x26, x25, [sp, #16]
	stp x24, x23, [sp, #32]
	stp x22, x21, [sp, #48]
	stp x20, x19, [sp, #64]
	stp x29, x30, [sp, #80]
	add x29, sp, #80
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	.cfi_offset w19, -24
	.cfi_offset w20, -32
	.cfi_offset w21, -40
	.cfi_offset w22, -48
	.cfi_offset w23, -56
	.cfi_offset w24, -64
	.cfi_offset w25, -72
	.cfi_offset w26, -80
	.cfi_offset w27, -88
	.cfi_offset w28, -96
	.cfi_remember_state
	sub sp, sp, #1, lsl #12
	sub sp, sp, #48
	tst w2, #0xffff
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/perft.rs : 129
		if depth == 0 {
	b.eq LBB56_7
	mov x22, x2
	mov x19, x1
	mov x20, x0
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/movelist.rs : 12
		MoveList {
	str xzr, [sp, #4104]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/perft.rs : 137
		game.side_to_move,
	ldrb w2, [x0, #220]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/perft.rs : 134
		super::pseudolegal::all::generate_pseudolegal_moves(
	add x0, sp, #8
	mov x1, x20
	mov x3, x19
	bl engine::moves::pseudolegal::all::generate_pseudolegal_moves
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/movelist.rs : 27
		self.end
	ldr x25, [sp, #4104]
		// /rustc/82e1608dfa6e0b5569232559e3d385fea5a93112/library/core/src/iter/range.rs : 729
		if self.start < self.end {
	cbz x25, LBB56_8
	mov x21, #0
	mov x26, #0
	sub w22, w22, #1
	add x23, sp, #8
	mov w27, #56
	b LBB56_4
LBB56_3:
	add x26, x26, #1
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/perft.rs : 150
		game.unmake_move(&move_item, unmake_metadata);
	sub x2, x29, #112
	mov x0, x20
	mov x1, x23
	bl engine::state::game::GameState::unmake_move
		// /rustc/82e1608dfa6e0b5569232559e3d385fea5a93112/library/core/src/cmp.rs : 1567
		ord_impl! { char usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 }
	add x23, x23, #16
		// /rustc/82e1608dfa6e0b5569232559e3d385fea5a93112/library/core/src/iter/range.rs : 729
		if self.start < self.end {
	cmp x25, x26
	b.eq LBB56_9
LBB56_4:
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/perft.rs : 141
		let move_item = &move_list.moves[index];
	cmp x26, #256
	b.eq LBB56_10
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/perft.rs : 142
		let player = game.side_to_move;
	ldrb w28, [x20, #220]
	cmp x28, #0
	cset w24, eq
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/perft.rs : 143
		let unmake_metadata = game.make_move(move_item);
	sub x0, x29, #112
	mov x1, x20
	mov x2, x23
	bl engine::state::game::GameState::make_move
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 88
		return &self.boards[player as usize][piece as usize];
	umaddl x8, w28, w27, x20
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/attacked/in_check.rs : 10
		let king = game
	ldr x8, [x8, #128]
		// /rustc/82e1608dfa6e0b5569232559e3d385fea5a93112/library/core/src/num/mod.rs : 1169
		uint_impl! {
	rbit x8, x8
	clz x0, x8
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/attacked/in_check.rs : 15
		return is_square_attacked(
	mov x1, x24
	mov x2, x20
	mov x3, x19
	bl engine::moves::attacked::square_attacked::is_square_attacked
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/perft.rs : 145
		if !is_in_check(player, game, cache) {
	tbnz w0, #0, LBB56_3
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/perft.rs : 146
		let move_nodes = _perft_unmake(game, cache, depth - 1);
	mov x0, x20
	mov x1, x19
	mov x2, x22
	bl engine::moves::perft::_perft_unmake
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/perft.rs : 147
		nodes += move_nodes;
	add x21, x0, x21
	b LBB56_3
LBB56_7:
	mov w21, #1
	b LBB56_9
	mov x21, #0
LBB56_9:
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/perft.rs : 154
		}
	mov x0, x21
	add sp, sp, #1, lsl #12
	add sp, sp, #48
	.cfi_def_cfa wsp, 96
	ldp x29, x30, [sp, #80]
	ldp x20, x19, [sp, #64]
	ldp x22, x21, [sp, #48]
	ldp x24, x23, [sp, #32]
	ldp x26, x25, [sp, #16]
	ldp x28, x27, [sp], #96
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	.cfi_restore w19
	.cfi_restore w20
	.cfi_restore w21
	.cfi_restore w22
	.cfi_restore w23
	.cfi_restore w24
	.cfi_restore w25
	.cfi_restore w26
	.cfi_restore w27
	.cfi_restore w28
	ret
LBB56_10:
	.cfi_restore_state
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/perft.rs : 141
		let move_item = &move_list.moves[index];
Lloh115:
	adrp x2, l___unnamed_38@PAGE
Lloh116:
	add x2, x2, l___unnamed_38@PAGEOFF
	mov w0, #256
	mov w1, #256
	bl core::panicking::panic_bounds_check
	.loh AdrpAdd	Lloh115, Lloh116
Lfunc_end56:
	.cfi_endproc

	.p2align	2
engine::state::game::GameState::make_move:
Lfunc_begin57:
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 127
		pub fn make_move(&mut self, move_item: &MoveItem) -> UnmakeMoveMetadata {
	.cfi_startproc
	stp x22, x21, [sp, #-48]!
	.cfi_def_cfa_offset 48
	stp x20, x19, [sp, #16]
	stp x29, x30, [sp, #32]
	add x29, sp, #32
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	.cfi_offset w19, -24
	.cfi_offset w20, -32
	.cfi_offset w21, -40
	.cfi_offset w22, -48
	.cfi_remember_state
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 137
		.remove_piece(self.side_to_move, move_item.from_pos);
	ldrsb x8, [x2, #4]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 113
		let removed = self.pos_to_piece[pos as usize];
	cmp x8, #64
	b.hs LBB57_55
	ldr s0, [x1, #216]
	ldr x11, [x1]
	ldr w12, [x1, #208]
	add x17, x1, #8
	ldrb w16, [x1, #220]
	ldrb w9, [x17, x8]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 114
		self.pos_to_piece[pos as usize] = Piece::Empty;
	strb wzr, [x17, x8]
	mov w10, #56
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 115
		self.boards[player as usize][removed as usize].unset(pos);
	umaddl x10, w16, w10, x17
	add x9, x10, x9, lsl #3
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 31
		*self &= INVERTED_SQUARE_MASKS[index as usize];
Lloh117:
	adrp x3, l___unnamed_25@PAGE
Lloh118:
	add x3, x3, l___unnamed_25@PAGEOFF
	ldr x10, [x3, x8, lsl #3]
	ldr x13, [x9, #72]
	and x13, x13, x10
	str x13, [x9, #72]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 116
		self.pos_to_player[player as usize].unset(pos);
	add x15, x17, x16, lsl #3
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 31
		*self &= INVERTED_SQUARE_MASKS[index as usize];
	ldr x9, [x15, #184]!
	and x9, x9, x10
	str x9, [x15]
	ldr x9, [x1, #72]
	and x9, x9, x10
	str x9, [x1, #72]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/pieces.rs : 14
		#[derive(PartialEq, Debug, Clone, Copy)]
	ldrb w13, [x2, #6]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 139
		if move_item.piece == Piece::Pawn {
	cmp w13, #1
	b.ne LBB57_5
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 140
		let final_piece = if move_item.promoting {
	ldrb w9, [x2, #9]
	ldrb w10, [x2, #7]
	cmp w9, #0
	csinc w4, w10, wzr, ne
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 147
		if move_item.enpassant {
	ldrb w9, [x2, #12]
	cbz w9, LBB57_11
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/square.rs : 40
		8 * self.rank + self.file
	and x10, x8, #0xfffffffffffffff8
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 149
		let to = Square::from(move_item.to_pos);
	ldrsb x9, [x2, #5]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/square.rs : 33
		file: pos % 8,
	ubfx w14, w9, #12, #3
	add w14, w9, w14
	and w14, w14, #0xf8
	sub w14, w9, w14
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/square.rs : 40
		8 * self.rank + self.file
	add x10, x10, w14, sxtb
	and w14, w10, #0xff
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 113
		let removed = self.pos_to_piece[pos as usize];
	cmp w14, #64
	b.hs LBB57_57
	and w14, w9, #0xff
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 137
		.remove_piece(self.side_to_move, move_item.from_pos);
	cmp w16, #0
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 113
		let removed = self.pos_to_piece[pos as usize];
	ldrb w5, [x17, x10]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 114
		self.pos_to_piece[pos as usize] = Piece::Empty;
	strb wzr, [x17, x10]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 137
		.remove_piece(self.side_to_move, move_item.from_pos);
	cset w6, eq
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 115
		self.boards[player as usize][removed as usize].unset(pos);
	add x7, x1, #8
	mov w19, #56
	umaddl x19, w6, w19, x7
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 31
		*self &= INVERTED_SQUARE_MASKS[index as usize];
	ldr x10, [x3, x10, lsl #3]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 115
		self.boards[player as usize][removed as usize].unset(pos);
	add x5, x19, x5, lsl #3
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 31
		*self &= INVERTED_SQUARE_MASKS[index as usize];
	ldr x19, [x5, #72]
	and x19, x19, x10
	str x19, [x5, #72]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 116
		self.pos_to_player[player as usize].unset(pos);
	add x5, x7, w6, uxtw #3
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 31
		*self &= INVERTED_SQUARE_MASKS[index as usize];
	ldr x6, [x5, #184]
	and x6, x6, x10
	str x6, [x5, #184]
	ldr x5, [x1, #72]
	and x10, x5, x10
	str x10, [x1, #72]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 113
		let removed = self.pos_to_piece[pos as usize];
	cmp w14, #64
	b.lo LBB57_13
	b LBB57_54
LBB57_5:
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 167
		.remove_piece(self.side_to_move.opponent(), move_item.to_pos);
	ldrsb x9, [x2, #5]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 113
		let removed = self.pos_to_piece[pos as usize];
	cmp x9, #64
	b.hs LBB57_54
	and w14, w9, #0xff
	and x5, x9, #0xff
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 137
		.remove_piece(self.side_to_move, move_item.from_pos);
	cmp w16, #0
	cset w4, eq
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 113
		let removed = self.pos_to_piece[pos as usize];
	ldrb w10, [x17, x9]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 114
		self.pos_to_piece[pos as usize] = Piece::Empty;
	strb wzr, [x17, x9]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 115
		self.boards[player as usize][removed as usize].unset(pos);
	add x6, x1, #80
	mov w7, #56
	umaddl x19, w4, w7, x6
	lsl x20, x10, #3
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 31
		*self &= INVERTED_SQUARE_MASKS[index as usize];
	ldr x3, [x3, x9, lsl #3]
	ldr x21, [x19, x20]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 115
		self.boards[player as usize][removed as usize].unset(pos);
	add x10, x1, w4, uxtw #3
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 31
		*self &= INVERTED_SQUARE_MASKS[index as usize];
	and x21, x21, x3
	str x21, [x19, x20]
	ldr x20, [x10, #192]!
	and x20, x20, x3
	ldr x21, [x1, #72]
	and x21, x21, x3
	str x21, [x1, #72]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 113
		let removed = self.pos_to_piece[pos as usize];
	ldrb w21, [x17, x9]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 114
		self.pos_to_piece[pos as usize] = Piece::Empty;
	strb wzr, [x17, x9]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 115
		self.boards[player as usize][removed as usize].unset(pos);
	lsl x21, x21, #3
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 31
		*self &= INVERTED_SQUARE_MASKS[index as usize];
	ldr x22, [x19, x21]
	and x22, x22, x3
	str x22, [x19, x21]
	str x20, [x10]
	ldr x19, [x1, #72]
	and x3, x19, x3
	str x3, [x1, #72]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 95
		self.boards[player as usize][piece as usize].set(pos);
	umaddl x3, w16, w7, x6
	lsl x6, x13, #3
	mov w7, #1
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 21
		*self |= 1 << index; // SQUARE_MASKS[index as usize];
	lsl x5, x7, x5
	ldr x7, [x3, x6]
	orr x7, x7, x5
	str x7, [x3, x6]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 96
		self.pos_to_piece[pos as usize] = piece;
	strb w13, [x17, x9]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 21
		*self |= 1 << index; // SQUARE_MASKS[index as usize];
	ldr x9, [x15]
	orr x17, x9, x5
	str x17, [x15]
	ldr x9, [x1, #72]
	orr x9, x9, x5
	str x9, [x1, #72]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 172
		if move_item.castling {
	ldrb w3, [x2, #13]
	cbz w3, LBB57_14
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 174
		match (self.side_to_move, move_item.to_pos) {
	cbz w16, LBB57_15
	cmp w14, #58
	b.eq LBB57_19
	cmp w14, #62
	b.ne LBB57_18
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 113
		let removed = self.pos_to_piece[pos as usize];
	ldrb w14, [x1, #71]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 114
		self.pos_to_piece[pos as usize] = Piece::Empty;
	strb wzr, [x1, #71]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 115
		self.boards[player as usize][removed as usize].unset(pos);
	add x3, x1, #80
	mov w5, #56
	umaddl x16, w16, w5, x3
	lsl x14, x14, #3
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 31
		*self &= INVERTED_SQUARE_MASKS[index as usize];
	ldr x6, [x16, x14]
	and x6, x6, #0x7fffffffffffffff
	str x6, [x16, x14]
	and x14, x17, #0x7fffffffffffffff
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 113
		let removed = self.pos_to_piece[pos as usize];
	ldrb w17, [x1, #69]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 115
		self.boards[player as usize][removed as usize].unset(pos);
	umaddl x3, w4, w5, x3
	lsl x17, x17, #3
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 31
		*self &= INVERTED_SQUARE_MASKS[index as usize];
	ldr x4, [x3, x17]
	and x4, x4, #0xdfffffffffffffff
	str x4, [x3, x17]
	str x14, [x15]
	ldr x14, [x10]
	and x14, x14, #0xdfffffffffffffff
	str x14, [x10]
	and x9, x9, #0x7fffffffffffffff
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 21
		*self |= 1 << index; // SQUARE_MASKS[index as usize];
	ldr x10, [x16, #32]
	orr x10, x10, #0x2000000000000000
	str x10, [x16, #32]
	mov w10, #4
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 96
		self.pos_to_piece[pos as usize] = piece;
	strb w10, [x1, #69]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 21
		*self |= 1 << index; // SQUARE_MASKS[index as usize];
	ldr x10, [x15]
	orr x10, x10, #0x2000000000000000
	str x10, [x15]
	orr x9, x9, #0x2000000000000000
	str x9, [x1, #72]
	mov w10, #1
	mov w14, #62
	b LBB57_21
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 160
		.remove_piece(self.side_to_move.opponent(), move_item.to_pos);
	ldrsb x9, [x2, #5]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 113
		let removed = self.pos_to_piece[pos as usize];
	cmp x9, #64
	b.hs LBB57_54
	and w14, w9, #0xff
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 137
		.remove_piece(self.side_to_move, move_item.from_pos);
	cmp w16, #0
	cset w10, eq
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 113
		let removed = self.pos_to_piece[pos as usize];
	ldrb w5, [x17, x9]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 114
		self.pos_to_piece[pos as usize] = Piece::Empty;
	strb wzr, [x17, x9]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 115
		self.boards[player as usize][removed as usize].unset(pos);
	add x6, x1, #8
	mov w7, #56
	umaddl x7, w10, w7, x6
	add x5, x7, x5, lsl #3
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 31
		*self &= INVERTED_SQUARE_MASKS[index as usize];
	ldr x7, [x3, x9, lsl #3]
	ldr x19, [x5, #72]
	and x19, x19, x7
	str x19, [x5, #72]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 116
		self.pos_to_player[player as usize].unset(pos);
	add x10, x6, w10, uxtw #3
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 31
		*self &= INVERTED_SQUARE_MASKS[index as usize];
	ldr x5, [x10, #184]
	and x5, x5, x7
	str x5, [x10, #184]
	ldr x10, [x1, #72]
	and x10, x10, x7
	str x10, [x1, #72]
LBB57_13:
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 137
		.remove_piece(self.side_to_move, move_item.from_pos);
	cmp w16, #0
	cset w10, eq
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 113
		let removed = self.pos_to_piece[pos as usize];
	ldrb w5, [x17, x9]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 114
		self.pos_to_piece[pos as usize] = Piece::Empty;
	strb wzr, [x17, x9]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 115
		self.boards[player as usize][removed as usize].unset(pos);
	add x6, x1, #80
	mov w7, #56
	umaddl x19, w10, w7, x6
	add x10, x1, w10, uxtw #3
	lsl x5, x5, #3
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 31
		*self &= INVERTED_SQUARE_MASKS[index as usize];
	ldr x3, [x3, x9, lsl #3]
	ldr x20, [x19, x5]
	and x20, x20, x3
	str x20, [x19, x5]
	ldr x5, [x10, #192]
	and x5, x5, x3
	str x5, [x10, #192]
	ldr x10, [x1, #72]
	and x10, x10, x3
	str x10, [x1, #72]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 95
		self.boards[player as usize][piece as usize].set(pos);
	and x10, x4, #0xff
	umaddl x16, w16, w7, x6
	lsl x10, x10, #3
	mov w3, #1
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 21
		*self |= 1 << index; // SQUARE_MASKS[index as usize];
	lsl x3, x3, x14
	ldr x5, [x16, x10]
	orr x5, x5, x3
	str x5, [x16, x10]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 96
		self.pos_to_piece[pos as usize] = piece;
	strb w4, [x17, x9]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 21
		*self |= 1 << index; // SQUARE_MASKS[index as usize];
	ldr x9, [x15]
	orr x9, x9, x3
	str x9, [x15]
	ldr x9, [x1, #72]
	orr x9, x9, x3
	str x9, [x1, #72]
	ldrb w10, [x2, #13]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 206
		if !move_item.capturing && !move_item.castling && move_item.piece != Piece::Pawn {
	ldr w9, [x1, #208]
	cmp w10, #0
	cset w10, ne
	b LBB57_22
	mov w10, #0
	b LBB57_21
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 174
		match (self.side_to_move, move_item.to_pos) {
	cmp w14, #2
	b.eq LBB57_20
	cmp w14, #6
	b.ne LBB57_18
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 113
		let removed = self.pos_to_piece[pos as usize];
	ldrb w14, [x1, #15]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 114
		self.pos_to_piece[pos as usize] = Piece::Empty;
	strb wzr, [x1, #15]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 115
		self.boards[player as usize][removed as usize].unset(pos);
	add x3, x1, #80
	mov w5, #56
	umaddl x16, w16, w5, x3
	lsl x14, x14, #3
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 31
		*self &= INVERTED_SQUARE_MASKS[index as usize];
	ldr x6, [x16, x14]
	and x6, x6, #0xffffffffffffff7f
	str x6, [x16, x14]
	and x14, x17, #0xffffffffffffff7f
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 113
		let removed = self.pos_to_piece[pos as usize];
	ldrb w17, [x1, #13]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 115
		self.boards[player as usize][removed as usize].unset(pos);
	umaddl x3, w4, w5, x3
	lsl x17, x17, #3
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 31
		*self &= INVERTED_SQUARE_MASKS[index as usize];
	ldr x4, [x3, x17]
	and x4, x4, #0xffffffffffffffdf
	str x4, [x3, x17]
	str x14, [x15]
	ldr x14, [x10]
	and x14, x14, #0xffffffffffffffdf
	str x14, [x10]
	and x9, x9, #0xffffffffffffff7f
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 21
		*self |= 1 << index; // SQUARE_MASKS[index as usize];
	ldr x10, [x16, #32]
	orr x10, x10, #0x20
	str x10, [x16, #32]
	mov w10, #4
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 96
		self.pos_to_piece[pos as usize] = piece;
	strb w10, [x1, #13]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 21
		*self |= 1 << index; // SQUARE_MASKS[index as usize];
	ldr x10, [x15]
	orr x10, x10, #0x20
	str x10, [x15]
	orr x9, x9, #0x20
	str x9, [x1, #72]
	mov w10, #1
	mov w14, #6
	b LBB57_21
LBB57_18:
	mov w10, #1
	b LBB57_21
LBB57_19:
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 113
		let removed = self.pos_to_piece[pos as usize];
	ldrb w14, [x1, #64]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 114
		self.pos_to_piece[pos as usize] = Piece::Empty;
	strb wzr, [x1, #64]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 115
		self.boards[player as usize][removed as usize].unset(pos);
	add x3, x1, #80
	mov w5, #56
	umaddl x16, w16, w5, x3
	lsl x14, x14, #3
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 31
		*self &= INVERTED_SQUARE_MASKS[index as usize];
	ldr x6, [x16, x14]
	and x6, x6, #0xfeffffffffffffff
	str x6, [x16, x14]
	and x14, x17, #0xfeffffffffffffff
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 113
		let removed = self.pos_to_piece[pos as usize];
	ldrb w17, [x1, #67]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 115
		self.boards[player as usize][removed as usize].unset(pos);
	umaddl x3, w4, w5, x3
	lsl x17, x17, #3
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 31
		*self &= INVERTED_SQUARE_MASKS[index as usize];
	ldr x4, [x3, x17]
	and x4, x4, #0xf7ffffffffffffff
	str x4, [x3, x17]
	str x14, [x15]
	ldr x14, [x10]
	and x14, x14, #0xf7ffffffffffffff
	str x14, [x10]
	and x9, x9, #0xfeffffffffffffff
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 21
		*self |= 1 << index; // SQUARE_MASKS[index as usize];
	ldr x10, [x16, #32]
	orr x10, x10, #0x800000000000000
	str x10, [x16, #32]
	mov w10, #4
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 96
		self.pos_to_piece[pos as usize] = piece;
	strb w10, [x1, #67]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 21
		*self |= 1 << index; // SQUARE_MASKS[index as usize];
	ldr x10, [x15]
	orr x10, x10, #0x800000000000000
	str x10, [x15]
	orr x9, x9, #0x800000000000000
	str x9, [x1, #72]
	mov w10, #1
	mov w14, #58
	b LBB57_21
LBB57_20:
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 113
		let removed = self.pos_to_piece[pos as usize];
	ldrb w14, [x1, #8]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 114
		self.pos_to_piece[pos as usize] = Piece::Empty;
	strb wzr, [x1, #8]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 115
		self.boards[player as usize][removed as usize].unset(pos);
	add x3, x1, #80
	mov w5, #56
	umaddl x16, w16, w5, x3
	lsl x14, x14, #3
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 31
		*self &= INVERTED_SQUARE_MASKS[index as usize];
	ldr x6, [x16, x14]
	and x6, x6, #0xfffffffffffffffe
	str x6, [x16, x14]
	and x14, x17, #0xfffffffffffffffe
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 113
		let removed = self.pos_to_piece[pos as usize];
	ldrb w17, [x1, #11]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 115
		self.boards[player as usize][removed as usize].unset(pos);
	umaddl x3, w4, w5, x3
	lsl x17, x17, #3
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 31
		*self &= INVERTED_SQUARE_MASKS[index as usize];
	ldr x4, [x3, x17]
	and x4, x4, #0xfffffffffffffff7
	str x4, [x3, x17]
	str x14, [x15]
	ldr x14, [x10]
	and x14, x14, #0xfffffffffffffff7
	str x14, [x10]
	and x9, x9, #0xfffffffffffffffe
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 21
		*self |= 1 << index; // SQUARE_MASKS[index as usize];
	ldr x10, [x16, #32]
	orr x10, x10, #0x8
	str x10, [x16, #32]
	mov w10, #4
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 96
		self.pos_to_piece[pos as usize] = piece;
	strb w10, [x1, #11]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 21
		*self |= 1 << index; // SQUARE_MASKS[index as usize];
	ldr x10, [x15]
	orr x10, x10, #0x8
	str x10, [x15]
	orr x9, x9, #0x8
	str x9, [x1, #72]
	mov w10, #1
	mov w14, #2
LBB57_21:
	mov x9, x12
LBB57_22:
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 206
		if !move_item.capturing && !move_item.castling && move_item.piece != Piece::Pawn {
	ldrb w15, [x2, #10]
	cmp w15, #0
	csinc w15, w10, wzr, eq
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 139
		if move_item.piece == Piece::Pawn {
	cmp w13, #1
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 206
		if !move_item.capturing && !move_item.castling && move_item.piece != Piece::Pawn {
	csinc w15, w15, wzr, ne
	cmp w15, #0
	csinc w9, wzr, w9, ne
	str w9, [x1, #208]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 213
		if move_item.double {
	ldrb w9, [x2, #11]
	cbz w9, LBB57_27
	and w9, w8, #0xf8
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/square.rs : 32
		rank: pos / 8,
	lsr w15, w14, #3
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 223
		if to_rank > from_rank {
	cmp w15, w9, lsr #3
	add w9, w15, #31
	csinc w9, w9, w15, hi
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/square.rs : 40
		8 * self.rank + self.file
	mov x15, x14
	bfi w15, w9, #3, #29
	and w16, w15, #0xff
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 229
		self.enpassant_square = SQUARE_MASKS[<Square as Into<i8>>::into(Square {
	sxtb x9, w15
	cmp w16, #64
	b.hs LBB57_56
Lloh119:
	adrp x15, l___unnamed_39@PAGE
Lloh120:
	add x15, x15, l___unnamed_39@PAGEOFF
	ldr x9, [x15, x9, lsl #3]
	ushll.8h v0, v0, #0
	str x9, [x1]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/player.rs : 1
		#[derive(Debug, PartialEq, Clone, Copy)]
	ldrb w9, [x1, #220]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 239
		if self.side_to_move == Player::Black {
	cbz w9, LBB57_28
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 240
		self.full_move_number += 1;
	ldr w15, [x1, #212]
	add w15, w15, #1
	str w15, [x1, #212]
	cmp w13, #6
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 245
		if move_item.castling || move_item.piece == Piece::King {
	csinc w10, w10, wzr, ne
	tbz w10, #0, LBB57_35
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 30
		self.black_queen_side = false;
	strh wzr, [x1, #218]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 255
		if move_item.piece == Piece::Rook {
	cmp w13, #4
	b.eq LBB57_36
	b LBB57_41
	mov x9, #0
	ushll.8h v0, v0, #0
	str x9, [x1]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/player.rs : 1
		#[derive(Debug, PartialEq, Clone, Copy)]
	ldrb w9, [x1, #220]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 239
		if self.side_to_move == Player::Black {
	cbnz w9, LBB57_25
	cmp w13, #6
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 245
		if move_item.castling || move_item.piece == Piece::King {
	csinc w10, w10, wzr, ne
	tbz w10, #0, LBB57_30
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 26
		self.white_queen_side = false;
	strh wzr, [x1, #216]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 255
		if move_item.piece == Piece::Rook {
	cmp w13, #4
	b.ne LBB57_41
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 256
		match (self.side_to_move, move_item.from_pos) {
	cbnz w9, LBB57_36
	cbz w8, LBB57_53
	and w8, w8, #0xff
	cmp w8, #7
	b.ne LBB57_41
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 261
		self.castle_permissions.white_king_side = false;
	strb wzr, [x1, #217]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/pieces.rs : 14
		#[derive(PartialEq, Debug, Clone, Copy)]
	ldrb w8, [x2, #8]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 272
		if move_item.captured_piece == Piece::Rook {
	cmp w8, #4
	b.eq LBB57_42
	b LBB57_52
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 255
		if move_item.piece == Piece::Rook {
	cmp w13, #4
	b.ne LBB57_39
LBB57_36:
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 256
		match (self.side_to_move, move_item.from_pos) {
	cmp w8, #56
	b.eq LBB57_40
	and w8, w8, #0xff
	cmp w8, #63
	b.ne LBB57_41
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 267
		self.castle_permissions.black_king_side = false;
	strb wzr, [x1, #219]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/pieces.rs : 14
		#[derive(PartialEq, Debug, Clone, Copy)]
	ldrb w8, [x2, #8]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 272
		if move_item.captured_piece == Piece::Rook {
	cmp w8, #4
	b.eq LBB57_42
	b LBB57_52
LBB57_39:
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/pieces.rs : 14
		#[derive(PartialEq, Debug, Clone, Copy)]
	ldrb w8, [x2, #8]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 272
		if move_item.captured_piece == Piece::Rook {
	cmp w8, #4
	b.eq LBB57_43
	b LBB57_52
LBB57_40:
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 264
		self.castle_permissions.black_queen_side = false;
	strb wzr, [x1, #218]
LBB57_41:
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/pieces.rs : 14
		#[derive(PartialEq, Debug, Clone, Copy)]
	ldrb w8, [x2, #8]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 272
		if move_item.captured_piece == Piece::Rook {
	cmp w8, #4
	b.ne LBB57_52
LBB57_42:
	cbz w9, LBB57_46
LBB57_43:
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 273
		match (self.side_to_move.opponent(), move_item.to_pos) {
	cbz w14, LBB57_49
	cmp w14, #7
	b.ne LBB57_51
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 278
		self.castle_permissions.white_king_side = false;
	strb wzr, [x1, #217]
	b LBB57_51
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 273
		match (self.side_to_move.opponent(), move_item.to_pos) {
	cmp w14, #56
	b.eq LBB57_50
	cmp w14, #63
	b.ne LBB57_51
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 284
		self.castle_permissions.black_king_side = false;
	strb wzr, [x1, #219]
	b LBB57_51
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 275
		self.castle_permissions.white_queen_side = false;
	strb wzr, [x1, #216]
	b LBB57_51
LBB57_50:
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 281
		self.castle_permissions.black_queen_side = false;
	strb wzr, [x1, #218]
LBB57_51:
	mov w8, #4
LBB57_52:
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 291
		self.side_to_move = self.side_to_move.opponent();
	eor w9, w9, #0x1
	strb w9, [x1, #220]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 293
		UnmakeMoveMetadata {
	xtn.8b v0, v0
	str s0, [x0, #8]
	str x11, [x0]
	str w12, [x0, #12]
	strb w8, [x0, #16]
	.cfi_def_cfa wsp, 48
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 299
		}
	ldp x29, x30, [sp, #32]
	ldp x20, x19, [sp, #16]
	ldp x22, x21, [sp], #48
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	.cfi_restore w19
	.cfi_restore w20
	.cfi_restore w21
	.cfi_restore w22
	ret
	.cfi_restore_state
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 258
		self.castle_permissions.white_queen_side = false;
	strb wzr, [x1, #216]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/pieces.rs : 14
		#[derive(PartialEq, Debug, Clone, Copy)]
	ldrb w8, [x2, #8]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 272
		if move_item.captured_piece == Piece::Rook {
	cmp w8, #4
	b.eq LBB57_42
	b LBB57_52
LBB57_54:
Lloh121:
	adrp x2, l___unnamed_40@PAGE
Lloh122:
	add x2, x2, l___unnamed_40@PAGEOFF
	mov x0, x9
	mov w1, #64
	bl core::panicking::panic_bounds_check
LBB57_55:
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 113
		let removed = self.pos_to_piece[pos as usize];
Lloh123:
	adrp x2, l___unnamed_40@PAGE
Lloh124:
	add x2, x2, l___unnamed_40@PAGEOFF
	mov x0, x8
	mov w1, #64
	bl core::panicking::panic_bounds_check
LBB57_56:
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 229
		self.enpassant_square = SQUARE_MASKS[<Square as Into<i8>>::into(Square {
Lloh125:
	adrp x2, l___unnamed_41@PAGE
Lloh126:
	add x2, x2, l___unnamed_41@PAGEOFF
	mov x0, x9
	mov w1, #64
	bl core::panicking::panic_bounds_check
LBB57_57:
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 113
		let removed = self.pos_to_piece[pos as usize];
Lloh127:
	adrp x2, l___unnamed_40@PAGE
Lloh128:
	add x2, x2, l___unnamed_40@PAGEOFF
	mov x0, x10
	mov w1, #64
	bl core::panicking::panic_bounds_check
	.loh AdrpAdd	Lloh117, Lloh118
	.loh AdrpAdd	Lloh119, Lloh120
	.loh AdrpAdd	Lloh121, Lloh122
	.loh AdrpAdd	Lloh123, Lloh124
	.loh AdrpAdd	Lloh125, Lloh126
	.loh AdrpAdd	Lloh127, Lloh128
.section __TEXT,__text,regular,pure_instructions
	.p2align	2
engine::moves::attacked::square_attacked::is_square_attacked:
Lfunc_begin55:
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/attacked/square_attacked.rs : 123
		pub fn is_square_attacked(
	.cfi_startproc
	stp x29, x30, [sp, #-16]!
	.cfi_def_cfa_offset 16
	mov x29, sp
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	.cfi_remember_state
	and w8, w0, #0xff
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/attacked/square_attacked.rs : 20
		let knight_move_mask = cache.knight_moves_masks[pos as usize];
	sxtb x0, w0
	cmp w8, #63
	b.hi LBB55_4
	ldr x10, [x2, #72]
	add x11, x3, x0, lsl #3
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/attacked/square_attacked.rs : 31
		cache.rook_potential_blockers_masks[pos as usize],
	ldr x8, [x11, #5632]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/precalculate/magic_bitboards.rs : 33
		let blockers_on_path = blockers & potential_blockers_mask;
	and x8, x8, x10
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/attacked/square_attacked.rs : 33
		cache.rook_magics[pos as usize],
	ldr x9, [x11, #6272]
		// /rustc/82e1608dfa6e0b5569232559e3d385fea5a93112/library/core/src/num/mod.rs : 1169
		uint_impl! {
	mul x8, x8, x9
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/attacked/square_attacked.rs : 34
		cache.rook_bit_counts[pos as usize],
	add x13, x3, x0
	mov w9, #6208
	ldrb w9, [x13, x9]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/precalculate/magic_bitboards.rs : 35
		let index = (hash >> (64 - bit_count)) as usize;
	neg w9, w9
	lsr x8, x8, x9
	mov w9, #24
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/attacked/square_attacked.rs : 36
		let rook_moves_mask = cache.rook_magic_attack_tables[pos as usize][rook_magic_index];
	smaddl x12, w0, w9, x3
	ldr x9, [x12, #16]
		// /rustc/82e1608dfa6e0b5569232559e3d385fea5a93112/library/core/src/slice/index.rs : 258
		&(*slice)[self]
	cmp x8, x9
	b.hs LBB55_5
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/attacked/square_attacked.rs : 52
		cache.bishop_potential_blockers_masks[pos as usize],
	ldr x9, [x11, #5120]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/precalculate/magic_bitboards.rs : 33
		let blockers_on_path = blockers & potential_blockers_mask;
	and x9, x9, x10
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/attacked/square_attacked.rs : 54
		cache.bishop_magics[pos as usize],
	ldr x10, [x11, #6784]
		// /rustc/82e1608dfa6e0b5569232559e3d385fea5a93112/library/core/src/num/mod.rs : 1169
		uint_impl! {
	mul x9, x9, x10
	mov w10, #6144
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/attacked/square_attacked.rs : 55
		cache.bishop_bit_counts[pos as usize],
	ldrb w10, [x13, x10]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/precalculate/magic_bitboards.rs : 35
		let index = (hash >> (64 - bit_count)) as usize;
	neg w10, w10
	lsr x9, x9, x10
	mov w10, #24
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/attacked/square_attacked.rs : 57
		let bishop_moves_mask = cache.bishop_magic_attack_tables[pos as usize][bishop_magic_index];
	smaddl x13, w0, w10, x3
	ldr x10, [x13, #1552]
		// /rustc/82e1608dfa6e0b5569232559e3d385fea5a93112/library/core/src/slice/index.rs : 258
		&(*slice)[self]
	cmp x9, x10
	b.hs LBB55_6
	ldr x10, [x11, #4096]
	mov w11, w1
	mov w14, #56
	umaddl x14, w1, w14, x2
	ldr x14, [x14, #96]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/attacked/square_attacked.rs : 57
		let bishop_moves_mask = cache.bishop_magic_attack_tables[pos as usize][bishop_magic_index];
	add x13, x13, #1536
	ldr x13, [x13]
	mov w15, #56
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 88
		return &self.boards[player as usize][piece as usize];
	umaddl x11, w11, w15, x2
		// /rustc/82e1608dfa6e0b5569232559e3d385fea5a93112/library/core/src/ops/bit.rs : 178
		bitand_impl! { bool usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 }
	ldr q0, [x11, #112]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/attacked/square_attacked.rs : 36
		let rook_moves_mask = cache.rook_magic_attack_tables[pos as usize][rook_magic_index];
	ldr x12, [x12]
		// /rustc/82e1608dfa6e0b5569232559e3d385fea5a93112/library/core/src/slice/index.rs : 258
		&(*slice)[self]
	add x8, x12, x8, lsl #3
	add x9, x13, x9, lsl #3
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 88
		return &self.boards[player as usize][piece as usize];
	add x12, x11, #104
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/player.rs : 15
		match self {
	eor w13, w1, #0x1
		// /rustc/82e1608dfa6e0b5569232559e3d385fea5a93112/library/core/src/ops/bit.rs : 178
		bitand_impl! { bool usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 }
	and x10, x14, x10
		// /rustc/82e1608dfa6e0b5569232559e3d385fea5a93112/library/core/src/num/mod.rs : 1169
		uint_impl! {
	fmov d1, x10
	cnt.8b v1, v1
	uaddlv.8b h1, v1
	fmov w10, s1
		// /rustc/82e1608dfa6e0b5569232559e3d385fea5a93112/library/core/src/ops/bit.rs : 178
		bitand_impl! { bool usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 }
	ld1r.2d { v1 }, [x12]
	zip2.2d v1, v0, v1
	ld1r.2d { v2 }, [x8]
	ld1r.2d { v3 }, [x9]
	and.16b v1, v1, v3
	and.16b v0, v0, v2
		// /rustc/82e1608dfa6e0b5569232559e3d385fea5a93112/library/core/src/num/mod.rs : 1169
		uint_impl! {
	cnt.16b v1, v1
	uaddlp.8h v1, v1
	uaddlp.4s v1, v1
	uaddlp.2d v1, v1
	cnt.16b v0, v0
	uaddlp.8h v0, v0
	uaddlp.4s v0, v0
	uaddlp.2d v0, v0
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/attacked/square_attacked.rs : 42
		attacked_count += attacking_rooks.count_ones() as i8;
	uzp1.4s v0, v0, v1
	xtn.4h v0, v0
		// /rustc/82e1608dfa6e0b5569232559e3d385fea5a93112/library/core/src/ops/bit.rs : 178
		bitand_impl! { bool usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 }
	ldr x8, [x11, #88]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/attacked/square_attacked.rs : 115
		let king_move_mask = cache.king_moves_masks[pos as usize];
	add x9, x3, x0, lsl #3
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/attacked/square_attacked.rs : 78
		let attacking_mask = cache.pawn_attack_moves_mask[attacker.opponent() as usize][pos as usize];
	add x12, x9, x13, lsl #9
	ldr x12, [x12, #3072]
		// /rustc/82e1608dfa6e0b5569232559e3d385fea5a93112/library/core/src/ops/bit.rs : 178
		bitand_impl! { bool usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 }
	and x8, x12, x8
		// /rustc/82e1608dfa6e0b5569232559e3d385fea5a93112/library/core/src/num/mod.rs : 1169
		uint_impl! {
	fmov d1, x8
	cnt.8b v1, v1
	uaddlv.8b h1, v1
	fmov w8, s1
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/attacked/square_attacked.rs : 115
		let king_move_mask = cache.king_moves_masks[pos as usize];
	ldr x9, [x9, #4608]
		// /rustc/82e1608dfa6e0b5569232559e3d385fea5a93112/library/core/src/ops/bit.rs : 178
		bitand_impl! { bool usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 }
	ldr x11, [x11, #128]
	and x9, x11, x9
		// /rustc/82e1608dfa6e0b5569232559e3d385fea5a93112/library/core/src/num/mod.rs : 1169
		uint_impl! {
	fmov d1, x9
	cnt.8b v1, v1
	uaddlv.8b h1, v1
	fmov w9, s1
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/attacked/square_attacked.rs : 117
		attacked_count += attacking_king.count_ones() as i8;
	addv.4h h0, v0
	fmov w11, s0
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/attacked/square_attacked.rs : 42
		attacked_count += attacking_rooks.count_ones() as i8;
	add w10, w11, w10
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/attacked/square_attacked.rs : 117
		attacked_count += attacking_king.count_ones() as i8;
	add w8, w8, w9
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/attacked/square_attacked.rs : 80
		attacked_count += attacking_pawns.count_ones() as i8;
	add w8, w10, w8
	sxtb w8, w8
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/attacked/square_attacked.rs : 129
		return times_square_attacked(pos, attacker, game, cache) > 0;
	cmp w8, #0
	cset w0, gt
	.cfi_def_cfa wsp, 16
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/attacked/square_attacked.rs : 213
		}
	ldp x29, x30, [sp], #16
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
LBB55_4:
	.cfi_restore_state
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/attacked/square_attacked.rs : 20
		let knight_move_mask = cache.knight_moves_masks[pos as usize];
Lloh109:
	adrp x2, l___unnamed_35@PAGE
Lloh110:
	add x2, x2, l___unnamed_35@PAGEOFF
	mov w1, #64
	bl core::panicking::panic_bounds_check
LBB55_5:
		// /rustc/82e1608dfa6e0b5569232559e3d385fea5a93112/library/core/src/slice/index.rs : 258
		&(*slice)[self]
Lloh111:
	adrp x2, l___unnamed_36@PAGE
Lloh112:
	add x2, x2, l___unnamed_36@PAGEOFF
	mov x0, x8
	mov x1, x9
	bl core::panicking::panic_bounds_check
LBB55_6:
Lloh113:
	adrp x2, l___unnamed_37@PAGE
Lloh114:
	add x2, x2, l___unnamed_37@PAGEOFF
	mov x0, x9
	mov x1, x10
	bl core::panicking::panic_bounds_check
	.loh AdrpAdd	Lloh109, Lloh110
	.loh AdrpAdd	Lloh111, Lloh112
	.loh AdrpAdd	Lloh113, Lloh114
Lfunc_end55:
	.cfi_endproc

	.p2align	2
engine::moves::perft::_perft_unmake:
Lfunc_begin56:
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/perft.rs : 126
		fn _perft_unmake(game: &mut GameState, cache: &PrecalculatedCache, depth: u16) -> u64 {
	.cfi_startproc
	stp x28, x27, [sp, #-96]!
	.cfi_def_cfa_offset 96
	stp x26, x25, [sp, #16]
	stp x24, x23, [sp, #32]
	stp x22, x21, [sp, #48]
	stp x20, x19, [sp, #64]
	stp x29, x30, [sp, #80]
	add x29, sp, #80
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	.cfi_offset w19, -24
	.cfi_offset w20, -32
	.cfi_offset w21, -40
	.cfi_offset w22, -48
	.cfi_offset w23, -56
	.cfi_offset w24, -64
	.cfi_offset w25, -72
	.cfi_offset w26, -80
	.cfi_offset w27, -88
	.cfi_offset w28, -96
	.cfi_remember_state
	sub sp, sp, #1, lsl #12
	sub sp, sp, #48
	tst w2, #0xffff
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/perft.rs : 129
		if depth == 0 {
	b.eq LBB56_7
	mov x22, x2
	mov x19, x1
	mov x20, x0
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/movelist.rs : 12
		MoveList {
	str xzr, [sp, #4104]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/perft.rs : 137
		game.side_to_move,
	ldrb w2, [x0, #220]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/perft.rs : 134
		super::pseudolegal::all::generate_pseudolegal_moves(
	add x0, sp, #8
	mov x1, x20
	mov x3, x19
	bl engine::moves::pseudolegal::all::generate_pseudolegal_moves
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/movelist.rs : 27
		self.end
	ldr x25, [sp, #4104]
		// /rustc/82e1608dfa6e0b5569232559e3d385fea5a93112/library/core/src/iter/range.rs : 729
		if self.start < self.end {
	cbz x25, LBB56_8
	mov x21, #0
	mov x26, #0
	sub w22, w22, #1
	add x23, sp, #8
	mov w27, #56
	b LBB56_4
LBB56_3:
	add x26, x26, #1
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/perft.rs : 150
		game.unmake_move(&move_item, unmake_metadata);
	sub x2, x29, #112
	mov x0, x20
	mov x1, x23
	bl engine::state::game::GameState::unmake_move
		// /rustc/82e1608dfa6e0b5569232559e3d385fea5a93112/library/core/src/cmp.rs : 1567
		ord_impl! { char usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 }
	add x23, x23, #16
		// /rustc/82e1608dfa6e0b5569232559e3d385fea5a93112/library/core/src/iter/range.rs : 729
		if self.start < self.end {
	cmp x25, x26
	b.eq LBB56_9
LBB56_4:
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/perft.rs : 141
		let move_item = &move_list.moves[index];
	cmp x26, #256
	b.eq LBB56_10
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/perft.rs : 142
		let player = game.side_to_move;
	ldrb w28, [x20, #220]
	cmp x28, #0
	cset w24, eq
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/perft.rs : 143
		let unmake_metadata = game.make_move(move_item);
	sub x0, x29, #112
	mov x1, x20
	mov x2, x23
	bl engine::state::game::GameState::make_move
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 88
		return &self.boards[player as usize][piece as usize];
	umaddl x8, w28, w27, x20
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/attacked/in_check.rs : 10
		let king = game
	ldr x8, [x8, #128]
		// /rustc/82e1608dfa6e0b5569232559e3d385fea5a93112/library/core/src/num/mod.rs : 1169
		uint_impl! {
	rbit x8, x8
	clz x0, x8
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/attacked/in_check.rs : 15
		return is_square_attacked(
	mov x1, x24
	mov x2, x20
	mov x3, x19
	bl engine::moves::attacked::square_attacked::is_square_attacked
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/perft.rs : 145
		if !is_in_check(player, game, cache) {
	tbnz w0, #0, LBB56_3
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/perft.rs : 146
		let move_nodes = _perft_unmake(game, cache, depth - 1);
	mov x0, x20
	mov x1, x19
	mov x2, x22
	bl engine::moves::perft::_perft_unmake
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/perft.rs : 147
		nodes += move_nodes;
	add x21, x0, x21
	b LBB56_3
LBB56_7:
	mov w21, #1
	b LBB56_9
	mov x21, #0
LBB56_9:
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/perft.rs : 154
		}
	mov x0, x21
	add sp, sp, #1, lsl #12
	add sp, sp, #48
	.cfi_def_cfa wsp, 96
	ldp x29, x30, [sp, #80]
	ldp x20, x19, [sp, #64]
	ldp x22, x21, [sp, #48]
	ldp x24, x23, [sp, #32]
	ldp x26, x25, [sp, #16]
	ldp x28, x27, [sp], #96
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	.cfi_restore w19
	.cfi_restore w20
	.cfi_restore w21
	.cfi_restore w22
	.cfi_restore w23
	.cfi_restore w24
	.cfi_restore w25
	.cfi_restore w26
	.cfi_restore w27
	.cfi_restore w28
	ret
LBB56_10:
	.cfi_restore_state
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/perft.rs : 141
		let move_item = &move_list.moves[index];
Lloh115:
	adrp x2, l___unnamed_38@PAGE
Lloh116:
	add x2, x2, l___unnamed_38@PAGEOFF
	mov w0, #256
	mov w1, #256
	bl core::panicking::panic_bounds_check
	.loh AdrpAdd	Lloh115, Lloh116
Lfunc_end56:
	.cfi_endproc

	.p2align	2
engine::state::game::GameState::make_move:
Lfunc_begin57:
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 127
		pub fn make_move(&mut self, move_item: &MoveItem) -> UnmakeMoveMetadata {
	.cfi_startproc
	stp x22, x21, [sp, #-48]!
	.cfi_def_cfa_offset 48
	stp x20, x19, [sp, #16]
	stp x29, x30, [sp, #32]
	add x29, sp, #32
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	.cfi_offset w19, -24
	.cfi_offset w20, -32
	.cfi_offset w21, -40
	.cfi_offset w22, -48
	.cfi_remember_state
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 137
		.remove_piece(self.side_to_move, move_item.from_pos);
	ldrsb x8, [x2, #4]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 113
		let removed = self.pos_to_piece[pos as usize];
	cmp x8, #64
	b.hs LBB57_58
	ldr s0, [x1, #216]
	ldr x11, [x1]
	ldr w12, [x1, #208]
	add x17, x1, #8
	ldrb w16, [x1, #220]
	ldrb w9, [x17, x8]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 114
		self.pos_to_piece[pos as usize] = Piece::Empty;
	strb wzr, [x17, x8]
	mov w10, #56
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 115
		self.boards[player as usize][removed as usize].unset(pos);
	umaddl x10, w16, w10, x17
	add x9, x10, x9, lsl #3
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 31
		*self &= INVERTED_SQUARE_MASKS[index as usize];
Lloh117:
	adrp x4, l___unnamed_25@PAGE
Lloh118:
	add x4, x4, l___unnamed_25@PAGEOFF
	ldr x10, [x4, x8, lsl #3]
	ldr x13, [x9, #72]
	and x13, x13, x10
	str x13, [x9, #72]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 116
		self.pos_to_player[player as usize].unset(pos);
	add x15, x17, x16, lsl #3
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 31
		*self &= INVERTED_SQUARE_MASKS[index as usize];
	ldr x9, [x15, #184]!
	and x9, x9, x10
	str x9, [x15]
	ldr x9, [x1, #72]
	and x9, x9, x10
	str x9, [x1, #72]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/pieces.rs : 14
		#[derive(PartialEq, Debug, Clone, Copy)]
	ldrb w14, [x2, #6]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 139
		if move_item.piece == Piece::Pawn {
	cmp w14, #1
	b.ne LBB57_5
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 140
		let final_piece = if move_item.promoting {
	ldrb w9, [x2, #9]
	ldrb w10, [x2, #7]
	cmp w9, #0
	csinc w3, w10, wzr, ne
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 147
		if move_item.enpassant {
	ldrb w9, [x2, #12]
	cbz w9, LBB57_11
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/square.rs : 40
		8 * self.rank + self.file
	and x10, x8, #0xfffffffffffffff8
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 149
		let to = Square::from(move_item.to_pos);
	ldrsb x9, [x2, #5]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/square.rs : 33
		file: pos % 8,
	ubfx w13, w9, #12, #3
	add w13, w9, w13
	and w13, w13, #0xf8
	sub w13, w9, w13
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/square.rs : 40
		8 * self.rank + self.file
	add x10, x10, w13, sxtb
	and w13, w10, #0xff
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 113
		let removed = self.pos_to_piece[pos as usize];
	cmp w13, #64
	b.hs LBB57_60
	and w13, w9, #0xff
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 137
		.remove_piece(self.side_to_move, move_item.from_pos);
	cmp w16, #0
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 113
		let removed = self.pos_to_piece[pos as usize];
	ldrb w5, [x17, x10]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 114
		self.pos_to_piece[pos as usize] = Piece::Empty;
	strb wzr, [x17, x10]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 137
		.remove_piece(self.side_to_move, move_item.from_pos);
	cset w6, eq
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 115
		self.boards[player as usize][removed as usize].unset(pos);
	add x7, x1, #8
	mov w19, #56
	umaddl x19, w6, w19, x7
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 31
		*self &= INVERTED_SQUARE_MASKS[index as usize];
	ldr x10, [x4, x10, lsl #3]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 115
		self.boards[player as usize][removed as usize].unset(pos);
	add x5, x19, x5, lsl #3
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 31
		*self &= INVERTED_SQUARE_MASKS[index as usize];
	ldr x19, [x5, #72]
	and x19, x19, x10
	str x19, [x5, #72]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 116
		self.pos_to_player[player as usize].unset(pos);
	add x5, x7, w6, uxtw #3
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 31
		*self &= INVERTED_SQUARE_MASKS[index as usize];
	ldr x6, [x5, #184]
	and x6, x6, x10
	str x6, [x5, #184]
	ldr x5, [x1, #72]
	and x10, x5, x10
	str x10, [x1, #72]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 113
		let removed = self.pos_to_piece[pos as usize];
	cmp w13, #64
	b.lo LBB57_13
	b LBB57_57
LBB57_5:
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 167
		.remove_piece(self.side_to_move.opponent(), move_item.to_pos);
	ldrsb x9, [x2, #5]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 113
		let removed = self.pos_to_piece[pos as usize];
	cmp x9, #64
	b.hs LBB57_57
	and w13, w9, #0xff
	and x5, x9, #0xff
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 137
		.remove_piece(self.side_to_move, move_item.from_pos);
	cmp w16, #0
	cset w3, eq
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 113
		let removed = self.pos_to_piece[pos as usize];
	ldrb w10, [x17, x9]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 114
		self.pos_to_piece[pos as usize] = Piece::Empty;
	strb wzr, [x17, x9]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 115
		self.boards[player as usize][removed as usize].unset(pos);
	add x6, x1, #80
	mov w7, #56
	umaddl x19, w3, w7, x6
	lsl x20, x10, #3
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 31
		*self &= INVERTED_SQUARE_MASKS[index as usize];
	ldr x4, [x4, x9, lsl #3]
	ldr x21, [x19, x20]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 115
		self.boards[player as usize][removed as usize].unset(pos);
	add x10, x1, w3, uxtw #3
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 31
		*self &= INVERTED_SQUARE_MASKS[index as usize];
	and x21, x21, x4
	str x21, [x19, x20]
	ldr x20, [x10, #192]!
	and x20, x20, x4
	ldr x21, [x1, #72]
	and x21, x21, x4
	str x21, [x1, #72]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 113
		let removed = self.pos_to_piece[pos as usize];
	ldrb w21, [x17, x9]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 114
		self.pos_to_piece[pos as usize] = Piece::Empty;
	strb wzr, [x17, x9]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 115
		self.boards[player as usize][removed as usize].unset(pos);
	lsl x21, x21, #3
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 31
		*self &= INVERTED_SQUARE_MASKS[index as usize];
	ldr x22, [x19, x21]
	and x22, x22, x4
	str x22, [x19, x21]
	str x20, [x10]
	ldr x19, [x1, #72]
	and x4, x19, x4
	str x4, [x1, #72]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 95
		self.boards[player as usize][piece as usize].set(pos);
	umaddl x4, w16, w7, x6
	lsl x6, x14, #3
	mov w7, #1
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 21
		*self |= 1 << index; // SQUARE_MASKS[index as usize];
	lsl x5, x7, x5
	ldr x7, [x4, x6]
	orr x7, x7, x5
	str x7, [x4, x6]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 96
		self.pos_to_piece[pos as usize] = piece;
	strb w14, [x17, x9]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 21
		*self |= 1 << index; // SQUARE_MASKS[index as usize];
	ldr x9, [x15]
	orr x17, x9, x5
	str x17, [x15]
	ldr x9, [x1, #72]
	orr x9, x9, x5
	str x9, [x1, #72]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 172
		if move_item.castling {
	ldrb w4, [x2, #13]
	cbz w4, LBB57_14
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 174
		match (self.side_to_move, move_item.to_pos) {
	cbz w16, LBB57_16
	cmp w13, #58
	b.eq LBB57_19
	cmp w13, #62
	b.ne LBB57_22
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 113
		let removed = self.pos_to_piece[pos as usize];
	ldrb w4, [x1, #71]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 115
		self.boards[player as usize][removed as usize].unset(pos);
	add x5, x1, #80
	mov w6, #56
	umaddl x16, w16, w6, x5
	lsl x4, x4, #3
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 31
		*self &= INVERTED_SQUARE_MASKS[index as usize];
	ldr x7, [x16, x4]
	and x7, x7, #0x7fffffffffffffff
	str x7, [x16, x4]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 114
		self.pos_to_piece[pos as usize] = Piece::Empty;
	strb wzr, [x1, #71]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 31
		*self &= INVERTED_SQUARE_MASKS[index as usize];
	and x17, x17, #0x7fffffffffffffff
	str x17, [x15]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 113
		let removed = self.pos_to_piece[pos as usize];
	ldrb w17, [x1, #69]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 115
		self.boards[player as usize][removed as usize].unset(pos);
	umaddl x3, w3, w6, x5
	lsl x17, x17, #3
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 31
		*self &= INVERTED_SQUARE_MASKS[index as usize];
	ldr x4, [x3, x17]
	and x4, x4, #0xdfffffffffffffff
	str x4, [x3, x17]
	ldr x17, [x10]
	and x17, x17, #0xdfffffffffffffff
	str x17, [x10]
	and x9, x9, #0x7fffffffffffffff
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 21
		*self |= 1 << index; // SQUARE_MASKS[index as usize];
	ldr x10, [x16, #32]
	orr x10, x10, #0x2000000000000000
	str x10, [x16, #32]
	mov w10, #4
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 96
		self.pos_to_piece[pos as usize] = piece;
	strb w10, [x1, #69]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 21
		*self |= 1 << index; // SQUARE_MASKS[index as usize];
	ldr x10, [x15]
	orr x10, x10, #0x2000000000000000
	str x10, [x15]
	orr x9, x9, #0x2000000000000000
	b LBB57_21
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 160
		.remove_piece(self.side_to_move.opponent(), move_item.to_pos);
	ldrsb x9, [x2, #5]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 113
		let removed = self.pos_to_piece[pos as usize];
	cmp x9, #64
	b.hs LBB57_57
	and w13, w9, #0xff
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 137
		.remove_piece(self.side_to_move, move_item.from_pos);
	cmp w16, #0
	cset w10, eq
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 113
		let removed = self.pos_to_piece[pos as usize];
	ldrb w5, [x17, x9]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 114
		self.pos_to_piece[pos as usize] = Piece::Empty;
	strb wzr, [x17, x9]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 115
		self.boards[player as usize][removed as usize].unset(pos);
	add x6, x1, #8
	mov w7, #56
	umaddl x7, w10, w7, x6
	add x5, x7, x5, lsl #3
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 31
		*self &= INVERTED_SQUARE_MASKS[index as usize];
	ldr x7, [x4, x9, lsl #3]
	ldr x19, [x5, #72]
	and x19, x19, x7
	str x19, [x5, #72]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 116
		self.pos_to_player[player as usize].unset(pos);
	add x10, x6, w10, uxtw #3
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 31
		*self &= INVERTED_SQUARE_MASKS[index as usize];
	ldr x5, [x10, #184]
	and x5, x5, x7
	str x5, [x10, #184]
	ldr x10, [x1, #72]
	and x10, x10, x7
	str x10, [x1, #72]
LBB57_13:
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 137
		.remove_piece(self.side_to_move, move_item.from_pos);
	cmp w16, #0
	cset w10, eq
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 113
		let removed = self.pos_to_piece[pos as usize];
	ldrb w5, [x17, x9]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 114
		self.pos_to_piece[pos as usize] = Piece::Empty;
	strb wzr, [x17, x9]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 115
		self.boards[player as usize][removed as usize].unset(pos);
	add x6, x1, #80
	mov w7, #56
	umaddl x19, w10, w7, x6
	add x10, x1, w10, uxtw #3
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 31
		*self &= INVERTED_SQUARE_MASKS[index as usize];
	ldr x4, [x4, x9, lsl #3]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 115
		self.boards[player as usize][removed as usize].unset(pos);
	lsl x5, x5, #3
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 31
		*self &= INVERTED_SQUARE_MASKS[index as usize];
	ldr x20, [x19, x5]
	and x20, x20, x4
	str x20, [x19, x5]
	ldr x5, [x10, #192]
	and x5, x5, x4
	str x5, [x10, #192]
	ldr x10, [x1, #72]
	and x10, x10, x4
	str x10, [x1, #72]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 95
		self.boards[player as usize][piece as usize].set(pos);
	and x10, x3, #0xff
	umaddl x16, w16, w7, x6
	lsl x10, x10, #3
	mov w4, #1
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 21
		*self |= 1 << index; // SQUARE_MASKS[index as usize];
	ldr x5, [x16, x10]
	lsl x4, x4, x13
	orr x5, x5, x4
	str x5, [x16, x10]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 96
		self.pos_to_piece[pos as usize] = piece;
	strb w3, [x17, x9]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 21
		*self |= 1 << index; // SQUARE_MASKS[index as usize];
	ldr x9, [x15]
	orr x9, x9, x4
	str x9, [x15]
	ldr x9, [x1, #72]
	orr x9, x9, x4
	str x9, [x1, #72]
	b LBB57_23
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 203
		if !move_item.capturing && !move_item.castling {
	ldrb w9, [x2, #10]
	cbnz w9, LBB57_22
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 204
		self.half_move_clock += 1;
	add w9, w12, #1
	str w9, [x1, #208]
	b LBB57_23
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 174
		match (self.side_to_move, move_item.to_pos) {
	cmp w13, #2
	b.eq LBB57_20
	cmp w13, #6
	b.ne LBB57_22
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 113
		let removed = self.pos_to_piece[pos as usize];
	ldrb w4, [x1, #15]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 115
		self.boards[player as usize][removed as usize].unset(pos);
	add x5, x1, #80
	mov w6, #56
	umaddl x16, w16, w6, x5
	lsl x4, x4, #3
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 31
		*self &= INVERTED_SQUARE_MASKS[index as usize];
	ldr x7, [x16, x4]
	and x7, x7, #0xffffffffffffff7f
	str x7, [x16, x4]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 114
		self.pos_to_piece[pos as usize] = Piece::Empty;
	strb wzr, [x1, #15]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 31
		*self &= INVERTED_SQUARE_MASKS[index as usize];
	and x17, x17, #0xffffffffffffff7f
	str x17, [x15]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 113
		let removed = self.pos_to_piece[pos as usize];
	ldrb w17, [x1, #13]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 115
		self.boards[player as usize][removed as usize].unset(pos);
	umaddl x3, w3, w6, x5
	lsl x17, x17, #3
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 31
		*self &= INVERTED_SQUARE_MASKS[index as usize];
	ldr x4, [x3, x17]
	and x4, x4, #0xffffffffffffffdf
	str x4, [x3, x17]
	ldr x17, [x10]
	and x17, x17, #0xffffffffffffffdf
	str x17, [x10]
	and x9, x9, #0xffffffffffffff7f
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 21
		*self |= 1 << index; // SQUARE_MASKS[index as usize];
	ldr x10, [x16, #32]
	orr x10, x10, #0x20
	str x10, [x16, #32]
	mov w10, #4
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 96
		self.pos_to_piece[pos as usize] = piece;
	strb w10, [x1, #13]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 21
		*self |= 1 << index; // SQUARE_MASKS[index as usize];
	ldr x10, [x15]
	orr x10, x10, #0x20
	str x10, [x15]
	orr x9, x9, #0x20
	b LBB57_21
LBB57_19:
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 113
		let removed = self.pos_to_piece[pos as usize];
	ldrb w4, [x1, #64]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 115
		self.boards[player as usize][removed as usize].unset(pos);
	add x5, x1, #80
	mov w6, #56
	umaddl x16, w16, w6, x5
	lsl x4, x4, #3
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 31
		*self &= INVERTED_SQUARE_MASKS[index as usize];
	ldr x7, [x16, x4]
	and x7, x7, #0xfeffffffffffffff
	str x7, [x16, x4]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 114
		self.pos_to_piece[pos as usize] = Piece::Empty;
	strb wzr, [x1, #64]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 31
		*self &= INVERTED_SQUARE_MASKS[index as usize];
	and x17, x17, #0xfeffffffffffffff
	str x17, [x15]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 113
		let removed = self.pos_to_piece[pos as usize];
	ldrb w17, [x1, #67]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 115
		self.boards[player as usize][removed as usize].unset(pos);
	umaddl x3, w3, w6, x5
	lsl x17, x17, #3
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 31
		*self &= INVERTED_SQUARE_MASKS[index as usize];
	ldr x4, [x3, x17]
	and x4, x4, #0xf7ffffffffffffff
	str x4, [x3, x17]
	ldr x17, [x10]
	and x17, x17, #0xf7ffffffffffffff
	str x17, [x10]
	and x9, x9, #0xfeffffffffffffff
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 21
		*self |= 1 << index; // SQUARE_MASKS[index as usize];
	ldr x10, [x16, #32]
	orr x10, x10, #0x800000000000000
	str x10, [x16, #32]
	mov w10, #4
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 96
		self.pos_to_piece[pos as usize] = piece;
	strb w10, [x1, #67]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 21
		*self |= 1 << index; // SQUARE_MASKS[index as usize];
	ldr x10, [x15]
	orr x10, x10, #0x800000000000000
	str x10, [x15]
	orr x9, x9, #0x800000000000000
	b LBB57_21
LBB57_20:
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 113
		let removed = self.pos_to_piece[pos as usize];
	ldrb w4, [x1, #8]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 115
		self.boards[player as usize][removed as usize].unset(pos);
	add x5, x1, #80
	mov w6, #56
	umaddl x16, w16, w6, x5
	lsl x4, x4, #3
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 31
		*self &= INVERTED_SQUARE_MASKS[index as usize];
	ldr x7, [x16, x4]
	and x7, x7, #0xfffffffffffffffe
	str x7, [x16, x4]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 114
		self.pos_to_piece[pos as usize] = Piece::Empty;
	strb wzr, [x1, #8]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 31
		*self &= INVERTED_SQUARE_MASKS[index as usize];
	and x17, x17, #0xfffffffffffffffe
	str x17, [x15]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 113
		let removed = self.pos_to_piece[pos as usize];
	ldrb w17, [x1, #11]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 115
		self.boards[player as usize][removed as usize].unset(pos);
	umaddl x3, w3, w6, x5
	lsl x17, x17, #3
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 31
		*self &= INVERTED_SQUARE_MASKS[index as usize];
	ldr x4, [x3, x17]
	and x4, x4, #0xfffffffffffffff7
	str x4, [x3, x17]
	ldr x17, [x10]
	and x17, x17, #0xfffffffffffffff7
	str x17, [x10]
	and x9, x9, #0xfffffffffffffffe
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 21
		*self |= 1 << index; // SQUARE_MASKS[index as usize];
	ldr x10, [x16, #32]
	orr x10, x10, #0x8
	str x10, [x16, #32]
	mov w10, #4
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 96
		self.pos_to_piece[pos as usize] = piece;
	strb w10, [x1, #11]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 21
		*self |= 1 << index; // SQUARE_MASKS[index as usize];
	ldr x10, [x15]
	orr x10, x10, #0x8
	str x10, [x15]
	orr x9, x9, #0x8
LBB57_21:
	str x9, [x1, #72]
LBB57_22:
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 206
		self.half_move_clock = 0;
	str wzr, [x1, #208]
LBB57_23:
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 213
		if move_item.double {
	ldrb w9, [x2, #11]
	cbz w9, LBB57_30
	and w9, w8, #0xf8
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/square.rs : 32
		rank: pos / 8,
	lsr w10, w13, #3
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 223
		if to_rank > from_rank {
	cmp w10, w9, lsr #3
	add w9, w10, #31
	csinc w9, w9, w10, hi
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/square.rs : 40
		8 * self.rank + self.file
	mov x10, x13
	bfi w10, w9, #3, #29
	and w15, w10, #0xff
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 229
		self.enpassant_square = SQUARE_MASKS[<Square as Into<i8>>::into(Square {
	sxtb x9, w10
	cmp w15, #64
	b.hs LBB57_59
Lloh119:
	adrp x10, l___unnamed_39@PAGE
Lloh120:
	add x10, x10, l___unnamed_39@PAGEOFF
	ldr x9, [x10, x9, lsl #3]
	ushll.8h v0, v0, #0
	str x9, [x1]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/player.rs : 1
		#[derive(Debug, PartialEq, Clone, Copy)]
	ldrb w9, [x1, #220]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 239
		if self.side_to_move == Player::Black {
	cbz w9, LBB57_31
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 240
		self.full_move_number += 1;
	ldr w10, [x1, #212]
	add w10, w10, #1
	str w10, [x1, #212]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 245
		if move_item.castling || move_item.piece == Piece::King {
	cmp w14, #6
	b.eq LBB57_39
	ldrb w10, [x2, #13]
	cbnz w10, LBB57_39
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 255
		if move_item.piece == Piece::Rook {
	cmp w14, #4
	b.eq LBB57_40
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/pieces.rs : 14
		#[derive(PartialEq, Debug, Clone, Copy)]
	ldrb w8, [x2, #8]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 272
		if move_item.captured_piece == Piece::Rook {
	cmp w8, #4
	b.eq LBB57_46
	b LBB57_55
	mov x9, #0
	ushll.8h v0, v0, #0
	str x9, [x1]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/player.rs : 1
		#[derive(Debug, PartialEq, Clone, Copy)]
	ldrb w9, [x1, #220]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 239
		if self.side_to_move == Player::Black {
	cbnz w9, LBB57_26
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 245
		if move_item.castling || move_item.piece == Piece::King {
	cmp w14, #6
	b.eq LBB57_33
	ldrb w10, [x2, #13]
	cbz w10, LBB57_34
LBB57_33:
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 26
		self.white_queen_side = false;
	strh wzr, [x1, #216]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 255
		if move_item.piece == Piece::Rook {
	cmp w14, #4
	b.ne LBB57_44
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 256
		match (self.side_to_move, move_item.from_pos) {
	cbnz w9, LBB57_40
	cbz w8, LBB57_56
	and w8, w8, #0xff
	cmp w8, #7
	b.ne LBB57_44
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 261
		self.castle_permissions.white_king_side = false;
	strb wzr, [x1, #217]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/pieces.rs : 14
		#[derive(PartialEq, Debug, Clone, Copy)]
	ldrb w8, [x2, #8]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 272
		if move_item.captured_piece == Piece::Rook {
	cmp w8, #4
	b.eq LBB57_45
	b LBB57_55
LBB57_39:
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 30
		self.black_queen_side = false;
	strh wzr, [x1, #218]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 255
		if move_item.piece == Piece::Rook {
	cmp w14, #4
	b.ne LBB57_44
LBB57_40:
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 256
		match (self.side_to_move, move_item.from_pos) {
	cmp w8, #56
	b.eq LBB57_43
	and w8, w8, #0xff
	cmp w8, #63
	b.ne LBB57_44
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 267
		self.castle_permissions.black_king_side = false;
	strb wzr, [x1, #219]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/pieces.rs : 14
		#[derive(PartialEq, Debug, Clone, Copy)]
	ldrb w8, [x2, #8]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 272
		if move_item.captured_piece == Piece::Rook {
	cmp w8, #4
	b.eq LBB57_45
	b LBB57_55
LBB57_43:
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 264
		self.castle_permissions.black_queen_side = false;
	strb wzr, [x1, #218]
LBB57_44:
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/pieces.rs : 14
		#[derive(PartialEq, Debug, Clone, Copy)]
	ldrb w8, [x2, #8]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 272
		if move_item.captured_piece == Piece::Rook {
	cmp w8, #4
	b.ne LBB57_55
LBB57_45:
	cbz w9, LBB57_49
LBB57_46:
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 273
		match (self.side_to_move.opponent(), move_item.to_pos) {
	cbz w13, LBB57_52
	cmp w13, #7
	b.ne LBB57_54
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 278
		self.castle_permissions.white_king_side = false;
	strb wzr, [x1, #217]
	b LBB57_54
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 273
		match (self.side_to_move.opponent(), move_item.to_pos) {
	cmp w13, #56
	b.eq LBB57_53
	cmp w13, #63
	b.ne LBB57_54
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 284
		self.castle_permissions.black_king_side = false;
	strb wzr, [x1, #219]
	b LBB57_54
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 275
		self.castle_permissions.white_queen_side = false;
	strb wzr, [x1, #216]
	b LBB57_54
LBB57_53:
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 281
		self.castle_permissions.black_queen_side = false;
	strb wzr, [x1, #218]
LBB57_54:
	mov w8, #4
LBB57_55:
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 291
		self.side_to_move = self.side_to_move.opponent();
	eor w9, w9, #0x1
	strb w9, [x1, #220]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 293
		UnmakeMoveMetadata {
	xtn.8b v0, v0
	str s0, [x0, #8]
	str x11, [x0]
	str w12, [x0, #12]
	strb w8, [x0, #16]
	.cfi_def_cfa wsp, 48
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 299
		}
	ldp x29, x30, [sp, #32]
	ldp x20, x19, [sp, #16]
	ldp x22, x21, [sp], #48
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	.cfi_restore w19
	.cfi_restore w20
	.cfi_restore w21
	.cfi_restore w22
	ret
	.cfi_restore_state
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 258
		self.castle_permissions.white_queen_side = false;
	strb wzr, [x1, #216]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/pieces.rs : 14
		#[derive(PartialEq, Debug, Clone, Copy)]
	ldrb w8, [x2, #8]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 272
		if move_item.captured_piece == Piece::Rook {
	cmp w8, #4
	b.eq LBB57_45
	b LBB57_55
LBB57_57:
Lloh121:
	adrp x2, l___unnamed_40@PAGE
Lloh122:
	add x2, x2, l___unnamed_40@PAGEOFF
	mov x0, x9
	mov w1, #64
	bl core::panicking::panic_bounds_check
LBB57_58:
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 113
		let removed = self.pos_to_piece[pos as usize];
Lloh123:
	adrp x2, l___unnamed_40@PAGE
Lloh124:
	add x2, x2, l___unnamed_40@PAGEOFF
	mov x0, x8
	mov w1, #64
	bl core::panicking::panic_bounds_check
LBB57_59:
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 229
		self.enpassant_square = SQUARE_MASKS[<Square as Into<i8>>::into(Square {
Lloh125:
	adrp x2, l___unnamed_41@PAGE
Lloh126:
	add x2, x2, l___unnamed_41@PAGEOFF
	mov x0, x9
	mov w1, #64
	bl core::panicking::panic_bounds_check
LBB57_60:
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 113
		let removed = self.pos_to_piece[pos as usize];
Lloh127:
	adrp x2, l___unnamed_40@PAGE
Lloh128:
	add x2, x2, l___unnamed_40@PAGEOFF
	mov x0, x10
	mov w1, #64
	bl core::panicking::panic_bounds_check
	.loh AdrpAdd	Lloh117, Lloh118
	.loh AdrpAdd	Lloh119, Lloh120
	.loh AdrpAdd	Lloh121, Lloh122
	.loh AdrpAdd	Lloh123, Lloh124
	.loh AdrpAdd	Lloh125, Lloh126
	.loh AdrpAdd	Lloh127, Lloh128
.section __TEXT,__text,regular,pure_instructions
	.p2align	2
engine::moves::attacked::square_attacked::is_square_attacked:
Lfunc_begin55:
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/attacked/square_attacked.rs : 123
		pub fn is_square_attacked(
	.cfi_startproc
	stp x29, x30, [sp, #-16]!
	.cfi_def_cfa_offset 16
	mov x29, sp
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	.cfi_remember_state
	and w8, w0, #0xff
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/attacked/square_attacked.rs : 20
		let knight_move_mask = cache.knight_moves_masks[pos as usize];
	sxtb x0, w0
	cmp w8, #63
	b.hi LBB55_4
	ldr x10, [x2, #72]
	add x11, x3, x0, lsl #3
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/attacked/square_attacked.rs : 31
		cache.rook_potential_blockers_masks[pos as usize],
	ldr x8, [x11, #5632]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/precalculate/magic_bitboards.rs : 33
		let blockers_on_path = blockers & potential_blockers_mask;
	and x8, x8, x10
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/attacked/square_attacked.rs : 33
		cache.rook_magics[pos as usize],
	ldr x9, [x11, #6272]
		// /rustc/82e1608dfa6e0b5569232559e3d385fea5a93112/library/core/src/num/mod.rs : 1169
		uint_impl! {
	mul x8, x8, x9
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/attacked/square_attacked.rs : 34
		cache.rook_bit_counts[pos as usize],
	add x13, x3, x0
	mov w9, #6208
	ldrb w9, [x13, x9]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/precalculate/magic_bitboards.rs : 35
		let index = (hash >> (64 - bit_count)) as usize;
	neg w9, w9
	lsr x8, x8, x9
	mov w9, #24
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/attacked/square_attacked.rs : 36
		let rook_moves_mask = cache.rook_magic_attack_tables[pos as usize][rook_magic_index];
	smaddl x12, w0, w9, x3
	ldr x9, [x12, #16]
		// /rustc/82e1608dfa6e0b5569232559e3d385fea5a93112/library/core/src/slice/index.rs : 258
		&(*slice)[self]
	cmp x8, x9
	b.hs LBB55_5
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/attacked/square_attacked.rs : 52
		cache.bishop_potential_blockers_masks[pos as usize],
	ldr x9, [x11, #5120]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/precalculate/magic_bitboards.rs : 33
		let blockers_on_path = blockers & potential_blockers_mask;
	and x9, x9, x10
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/attacked/square_attacked.rs : 54
		cache.bishop_magics[pos as usize],
	ldr x10, [x11, #6784]
		// /rustc/82e1608dfa6e0b5569232559e3d385fea5a93112/library/core/src/num/mod.rs : 1169
		uint_impl! {
	mul x9, x9, x10
	mov w10, #6144
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/attacked/square_attacked.rs : 55
		cache.bishop_bit_counts[pos as usize],
	ldrb w10, [x13, x10]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/precalculate/magic_bitboards.rs : 35
		let index = (hash >> (64 - bit_count)) as usize;
	neg w10, w10
	lsr x9, x9, x10
	mov w10, #24
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/attacked/square_attacked.rs : 57
		let bishop_moves_mask = cache.bishop_magic_attack_tables[pos as usize][bishop_magic_index];
	smaddl x13, w0, w10, x3
	ldr x10, [x13, #1552]
		// /rustc/82e1608dfa6e0b5569232559e3d385fea5a93112/library/core/src/slice/index.rs : 258
		&(*slice)[self]
	cmp x9, x10
	b.hs LBB55_6
	ldr x10, [x11, #4096]
	mov w11, w1
	mov w14, #56
	umaddl x14, w1, w14, x2
	ldr x14, [x14, #96]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/attacked/square_attacked.rs : 57
		let bishop_moves_mask = cache.bishop_magic_attack_tables[pos as usize][bishop_magic_index];
	add x13, x13, #1536
	ldr x13, [x13]
	mov w15, #56
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 88
		return &self.boards[player as usize][piece as usize];
	umaddl x11, w11, w15, x2
		// /rustc/82e1608dfa6e0b5569232559e3d385fea5a93112/library/core/src/ops/bit.rs : 178
		bitand_impl! { bool usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 }
	ldr q0, [x11, #112]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/attacked/square_attacked.rs : 36
		let rook_moves_mask = cache.rook_magic_attack_tables[pos as usize][rook_magic_index];
	ldr x12, [x12]
		// /rustc/82e1608dfa6e0b5569232559e3d385fea5a93112/library/core/src/slice/index.rs : 258
		&(*slice)[self]
	add x8, x12, x8, lsl #3
	add x9, x13, x9, lsl #3
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 88
		return &self.boards[player as usize][piece as usize];
	add x12, x11, #104
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/player.rs : 15
		match self {
	eor w13, w1, #0x1
		// /rustc/82e1608dfa6e0b5569232559e3d385fea5a93112/library/core/src/ops/bit.rs : 178
		bitand_impl! { bool usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 }
	and x10, x14, x10
		// /rustc/82e1608dfa6e0b5569232559e3d385fea5a93112/library/core/src/num/mod.rs : 1169
		uint_impl! {
	fmov d1, x10
	cnt.8b v1, v1
	uaddlv.8b h1, v1
	fmov w10, s1
		// /rustc/82e1608dfa6e0b5569232559e3d385fea5a93112/library/core/src/ops/bit.rs : 178
		bitand_impl! { bool usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 }
	ld1r.2d { v1 }, [x12]
	zip2.2d v1, v0, v1
	ld1r.2d { v2 }, [x8]
	ld1r.2d { v3 }, [x9]
	and.16b v1, v1, v3
	and.16b v0, v0, v2
		// /rustc/82e1608dfa6e0b5569232559e3d385fea5a93112/library/core/src/num/mod.rs : 1169
		uint_impl! {
	cnt.16b v1, v1
	uaddlp.8h v1, v1
	uaddlp.4s v1, v1
	uaddlp.2d v1, v1
	cnt.16b v0, v0
	uaddlp.8h v0, v0
	uaddlp.4s v0, v0
	uaddlp.2d v0, v0
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/attacked/square_attacked.rs : 42
		attacked_count += attacking_rooks.count_ones() as i8;
	uzp1.4s v0, v0, v1
	xtn.4h v0, v0
		// /rustc/82e1608dfa6e0b5569232559e3d385fea5a93112/library/core/src/ops/bit.rs : 178
		bitand_impl! { bool usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 }
	ldr x8, [x11, #88]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/attacked/square_attacked.rs : 115
		let king_move_mask = cache.king_moves_masks[pos as usize];
	add x9, x3, x0, lsl #3
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/attacked/square_attacked.rs : 78
		let attacking_mask = cache.pawn_attack_moves_mask[attacker.opponent() as usize][pos as usize];
	add x12, x9, x13, lsl #9
	ldr x12, [x12, #3072]
		// /rustc/82e1608dfa6e0b5569232559e3d385fea5a93112/library/core/src/ops/bit.rs : 178
		bitand_impl! { bool usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 }
	and x8, x12, x8
		// /rustc/82e1608dfa6e0b5569232559e3d385fea5a93112/library/core/src/num/mod.rs : 1169
		uint_impl! {
	fmov d1, x8
	cnt.8b v1, v1
	uaddlv.8b h1, v1
	fmov w8, s1
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/attacked/square_attacked.rs : 115
		let king_move_mask = cache.king_moves_masks[pos as usize];
	ldr x9, [x9, #4608]
		// /rustc/82e1608dfa6e0b5569232559e3d385fea5a93112/library/core/src/ops/bit.rs : 178
		bitand_impl! { bool usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 }
	ldr x11, [x11, #128]
	and x9, x11, x9
		// /rustc/82e1608dfa6e0b5569232559e3d385fea5a93112/library/core/src/num/mod.rs : 1169
		uint_impl! {
	fmov d1, x9
	cnt.8b v1, v1
	uaddlv.8b h1, v1
	fmov w9, s1
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/attacked/square_attacked.rs : 117
		attacked_count += attacking_king.count_ones() as i8;
	addv.4h h0, v0
	fmov w11, s0
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/attacked/square_attacked.rs : 42
		attacked_count += attacking_rooks.count_ones() as i8;
	add w10, w11, w10
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/attacked/square_attacked.rs : 117
		attacked_count += attacking_king.count_ones() as i8;
	add w8, w8, w9
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/attacked/square_attacked.rs : 80
		attacked_count += attacking_pawns.count_ones() as i8;
	add w8, w10, w8
	sxtb w8, w8
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/attacked/square_attacked.rs : 129
		return times_square_attacked(pos, attacker, game, cache) > 0;
	cmp w8, #0
	cset w0, gt
	.cfi_def_cfa wsp, 16
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/attacked/square_attacked.rs : 213
		}
	ldp x29, x30, [sp], #16
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
LBB55_4:
	.cfi_restore_state
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/attacked/square_attacked.rs : 20
		let knight_move_mask = cache.knight_moves_masks[pos as usize];
Lloh109:
	adrp x2, l___unnamed_35@PAGE
Lloh110:
	add x2, x2, l___unnamed_35@PAGEOFF
	mov w1, #64
	bl core::panicking::panic_bounds_check
LBB55_5:
		// /rustc/82e1608dfa6e0b5569232559e3d385fea5a93112/library/core/src/slice/index.rs : 258
		&(*slice)[self]
Lloh111:
	adrp x2, l___unnamed_36@PAGE
Lloh112:
	add x2, x2, l___unnamed_36@PAGEOFF
	mov x0, x8
	mov x1, x9
	bl core::panicking::panic_bounds_check
LBB55_6:
Lloh113:
	adrp x2, l___unnamed_37@PAGE
Lloh114:
	add x2, x2, l___unnamed_37@PAGEOFF
	mov x0, x9
	mov x1, x10
	bl core::panicking::panic_bounds_check
	.loh AdrpAdd	Lloh109, Lloh110
	.loh AdrpAdd	Lloh111, Lloh112
	.loh AdrpAdd	Lloh113, Lloh114
Lfunc_end55:
	.cfi_endproc

	.p2align	2
engine::moves::perft::_perft_unmake:
Lfunc_begin56:
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/perft.rs : 126
		fn _perft_unmake(game: &mut GameState, cache: &PrecalculatedCache, depth: u16) -> u64 {
	.cfi_startproc
	stp x28, x27, [sp, #-96]!
	.cfi_def_cfa_offset 96
	stp x26, x25, [sp, #16]
	stp x24, x23, [sp, #32]
	stp x22, x21, [sp, #48]
	stp x20, x19, [sp, #64]
	stp x29, x30, [sp, #80]
	add x29, sp, #80
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	.cfi_offset w19, -24
	.cfi_offset w20, -32
	.cfi_offset w21, -40
	.cfi_offset w22, -48
	.cfi_offset w23, -56
	.cfi_offset w24, -64
	.cfi_offset w25, -72
	.cfi_offset w26, -80
	.cfi_offset w27, -88
	.cfi_offset w28, -96
	.cfi_remember_state
	sub sp, sp, #1, lsl #12
	sub sp, sp, #48
	tst w2, #0xffff
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/perft.rs : 129
		if depth == 0 {
	b.eq LBB56_7
	mov x22, x2
	mov x19, x1
	mov x20, x0
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/movelist.rs : 12
		MoveList {
	str xzr, [sp, #4104]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/perft.rs : 137
		game.side_to_move,
	ldrb w2, [x0, #220]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/perft.rs : 134
		super::pseudolegal::all::generate_pseudolegal_moves(
	add x0, sp, #8
	mov x1, x20
	mov x3, x19
	bl engine::moves::pseudolegal::all::generate_pseudolegal_moves
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/movelist.rs : 27
		self.end
	ldr x25, [sp, #4104]
		// /rustc/82e1608dfa6e0b5569232559e3d385fea5a93112/library/core/src/iter/range.rs : 729
		if self.start < self.end {
	cbz x25, LBB56_8
	mov x21, #0
	mov x26, #0
	sub w22, w22, #1
	add x23, sp, #8
	mov w27, #56
	b LBB56_4
LBB56_3:
	add x26, x26, #1
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/perft.rs : 150
		game.unmake_move(&move_item, unmake_metadata);
	sub x2, x29, #112
	mov x0, x20
	mov x1, x23
	bl engine::state::game::GameState::unmake_move
		// /rustc/82e1608dfa6e0b5569232559e3d385fea5a93112/library/core/src/cmp.rs : 1567
		ord_impl! { char usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 }
	add x23, x23, #16
		// /rustc/82e1608dfa6e0b5569232559e3d385fea5a93112/library/core/src/iter/range.rs : 729
		if self.start < self.end {
	cmp x25, x26
	b.eq LBB56_9
LBB56_4:
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/perft.rs : 141
		let move_item = &move_list.moves[index];
	cmp x26, #256
	b.eq LBB56_10
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/perft.rs : 142
		let player = game.side_to_move;
	ldrb w28, [x20, #220]
	cmp x28, #0
	cset w24, eq
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/perft.rs : 143
		let unmake_metadata = game.make_move(move_item);
	sub x0, x29, #112
	mov x1, x20
	mov x2, x23
	bl engine::state::game::GameState::make_move
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 88
		return &self.boards[player as usize][piece as usize];
	umaddl x8, w28, w27, x20
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/attacked/in_check.rs : 10
		let king = game
	ldr x8, [x8, #128]
		// /rustc/82e1608dfa6e0b5569232559e3d385fea5a93112/library/core/src/num/mod.rs : 1169
		uint_impl! {
	rbit x8, x8
	clz x0, x8
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/attacked/in_check.rs : 15
		return is_square_attacked(
	mov x1, x24
	mov x2, x20
	mov x3, x19
	bl engine::moves::attacked::square_attacked::is_square_attacked
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/perft.rs : 145
		if !is_in_check(player, game, cache) {
	tbnz w0, #0, LBB56_3
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/perft.rs : 146
		let move_nodes = _perft_unmake(game, cache, depth - 1);
	mov x0, x20
	mov x1, x19
	mov x2, x22
	bl engine::moves::perft::_perft_unmake
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/perft.rs : 147
		nodes += move_nodes;
	add x21, x0, x21
	b LBB56_3
LBB56_7:
	mov w21, #1
	b LBB56_9
	mov x21, #0
LBB56_9:
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/perft.rs : 154
		}
	mov x0, x21
	add sp, sp, #1, lsl #12
	add sp, sp, #48
	.cfi_def_cfa wsp, 96
	ldp x29, x30, [sp, #80]
	ldp x20, x19, [sp, #64]
	ldp x22, x21, [sp, #48]
	ldp x24, x23, [sp, #32]
	ldp x26, x25, [sp, #16]
	ldp x28, x27, [sp], #96
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	.cfi_restore w19
	.cfi_restore w20
	.cfi_restore w21
	.cfi_restore w22
	.cfi_restore w23
	.cfi_restore w24
	.cfi_restore w25
	.cfi_restore w26
	.cfi_restore w27
	.cfi_restore w28
	ret
LBB56_10:
	.cfi_restore_state
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/perft.rs : 141
		let move_item = &move_list.moves[index];
Lloh115:
	adrp x2, l___unnamed_38@PAGE
Lloh116:
	add x2, x2, l___unnamed_38@PAGEOFF
	mov w0, #256
	mov w1, #256
	bl core::panicking::panic_bounds_check
	.loh AdrpAdd	Lloh115, Lloh116
Lfunc_end56:
	.cfi_endproc

	.p2align	2
engine::state::game::GameState::make_move:
Lfunc_begin57:
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 127
		pub fn make_move(&mut self, move_item: &MoveItem) -> UnmakeMoveMetadata {
	.cfi_startproc
	stp x22, x21, [sp, #-48]!
	.cfi_def_cfa_offset 48
	stp x20, x19, [sp, #16]
	stp x29, x30, [sp, #32]
	add x29, sp, #32
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	.cfi_offset w19, -24
	.cfi_offset w20, -32
	.cfi_offset w21, -40
	.cfi_offset w22, -48
	.cfi_remember_state
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 137
		.remove_piece(self.side_to_move, move_item.from_pos);
	ldrsb x8, [x2, #4]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 113
		let removed = self.pos_to_piece[pos as usize];
	cmp x8, #64
	b.hs LBB57_58
	ldr s0, [x1, #216]
	ldr x11, [x1]
	ldr w12, [x1, #208]
	add x17, x1, #8
	ldrb w16, [x1, #220]
	ldrb w9, [x17, x8]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 114
		self.pos_to_piece[pos as usize] = Piece::Empty;
	strb wzr, [x17, x8]
	mov w10, #56
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 115
		self.boards[player as usize][removed as usize].unset(pos);
	umaddl x10, w16, w10, x17
	add x9, x10, x9, lsl #3
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 31
		*self &= INVERTED_SQUARE_MASKS[index as usize];
Lloh117:
	adrp x4, l___unnamed_25@PAGE
Lloh118:
	add x4, x4, l___unnamed_25@PAGEOFF
	ldr x10, [x4, x8, lsl #3]
	ldr x13, [x9, #72]
	and x13, x13, x10
	str x13, [x9, #72]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 116
		self.pos_to_player[player as usize].unset(pos);
	add x15, x17, x16, lsl #3
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 31
		*self &= INVERTED_SQUARE_MASKS[index as usize];
	ldr x9, [x15, #184]!
	and x9, x9, x10
	str x9, [x15]
	ldr x9, [x1, #72]
	and x9, x9, x10
	str x9, [x1, #72]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/pieces.rs : 14
		#[derive(PartialEq, Debug, Clone, Copy)]
	ldrb w14, [x2, #6]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 139
		if move_item.piece == Piece::Pawn {
	cmp w14, #1
	b.ne LBB57_5
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 140
		let final_piece = if move_item.promoting {
	ldrb w9, [x2, #9]
	ldrb w10, [x2, #7]
	cmp w9, #0
	csinc w3, w10, wzr, ne
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 147
		if move_item.enpassant {
	ldrb w9, [x2, #12]
	cbz w9, LBB57_11
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/square.rs : 42
		8 * self.rank + self.file
	and x10, x8, #0xfffffffffffffff8
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 149
		let to = Square::from(move_item.to_pos);
	ldrsb x9, [x2, #5]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/square.rs : 34
		file: pos % 8,
	ubfx w13, w9, #12, #3
	add w13, w9, w13
	and w13, w13, #0xf8
	sub w13, w9, w13
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/square.rs : 42
		8 * self.rank + self.file
	add x10, x10, w13, sxtb
	and w13, w10, #0xff
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 113
		let removed = self.pos_to_piece[pos as usize];
	cmp w13, #64
	b.hs LBB57_60
	and w13, w9, #0xff
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 137
		.remove_piece(self.side_to_move, move_item.from_pos);
	cmp w16, #0
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 113
		let removed = self.pos_to_piece[pos as usize];
	ldrb w5, [x17, x10]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 114
		self.pos_to_piece[pos as usize] = Piece::Empty;
	strb wzr, [x17, x10]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 137
		.remove_piece(self.side_to_move, move_item.from_pos);
	cset w6, eq
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 115
		self.boards[player as usize][removed as usize].unset(pos);
	add x7, x1, #8
	mov w19, #56
	umaddl x19, w6, w19, x7
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 31
		*self &= INVERTED_SQUARE_MASKS[index as usize];
	ldr x10, [x4, x10, lsl #3]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 115
		self.boards[player as usize][removed as usize].unset(pos);
	add x5, x19, x5, lsl #3
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 31
		*self &= INVERTED_SQUARE_MASKS[index as usize];
	ldr x19, [x5, #72]
	and x19, x19, x10
	str x19, [x5, #72]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 116
		self.pos_to_player[player as usize].unset(pos);
	add x5, x7, w6, uxtw #3
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 31
		*self &= INVERTED_SQUARE_MASKS[index as usize];
	ldr x6, [x5, #184]
	and x6, x6, x10
	str x6, [x5, #184]
	ldr x5, [x1, #72]
	and x10, x5, x10
	str x10, [x1, #72]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 113
		let removed = self.pos_to_piece[pos as usize];
	cmp w13, #64
	b.lo LBB57_13
	b LBB57_57
LBB57_5:
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 167
		.remove_piece(self.side_to_move.opponent(), move_item.to_pos);
	ldrsb x9, [x2, #5]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 113
		let removed = self.pos_to_piece[pos as usize];
	cmp x9, #64
	b.hs LBB57_57
	and w13, w9, #0xff
	and x5, x9, #0xff
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 137
		.remove_piece(self.side_to_move, move_item.from_pos);
	cmp w16, #0
	cset w3, eq
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 113
		let removed = self.pos_to_piece[pos as usize];
	ldrb w10, [x17, x9]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 114
		self.pos_to_piece[pos as usize] = Piece::Empty;
	strb wzr, [x17, x9]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 115
		self.boards[player as usize][removed as usize].unset(pos);
	add x6, x1, #80
	mov w7, #56
	umaddl x19, w3, w7, x6
	lsl x20, x10, #3
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 31
		*self &= INVERTED_SQUARE_MASKS[index as usize];
	ldr x4, [x4, x9, lsl #3]
	ldr x21, [x19, x20]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 115
		self.boards[player as usize][removed as usize].unset(pos);
	add x10, x1, w3, uxtw #3
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 31
		*self &= INVERTED_SQUARE_MASKS[index as usize];
	and x21, x21, x4
	str x21, [x19, x20]
	ldr x20, [x10, #192]!
	and x20, x20, x4
	ldr x21, [x1, #72]
	and x21, x21, x4
	str x21, [x1, #72]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 113
		let removed = self.pos_to_piece[pos as usize];
	ldrb w21, [x17, x9]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 114
		self.pos_to_piece[pos as usize] = Piece::Empty;
	strb wzr, [x17, x9]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 115
		self.boards[player as usize][removed as usize].unset(pos);
	lsl x21, x21, #3
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 31
		*self &= INVERTED_SQUARE_MASKS[index as usize];
	ldr x22, [x19, x21]
	and x22, x22, x4
	str x22, [x19, x21]
	str x20, [x10]
	ldr x19, [x1, #72]
	and x4, x19, x4
	str x4, [x1, #72]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 95
		self.boards[player as usize][piece as usize].set(pos);
	umaddl x4, w16, w7, x6
	lsl x6, x14, #3
	mov w7, #1
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 21
		*self |= 1 << index; // SQUARE_MASKS[index as usize];
	lsl x5, x7, x5
	ldr x7, [x4, x6]
	orr x7, x7, x5
	str x7, [x4, x6]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 96
		self.pos_to_piece[pos as usize] = piece;
	strb w14, [x17, x9]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 21
		*self |= 1 << index; // SQUARE_MASKS[index as usize];
	ldr x9, [x15]
	orr x17, x9, x5
	str x17, [x15]
	ldr x9, [x1, #72]
	orr x9, x9, x5
	str x9, [x1, #72]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 172
		if move_item.castling {
	ldrb w4, [x2, #13]
	cbz w4, LBB57_14
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 174
		match (self.side_to_move, move_item.to_pos) {
	cbz w16, LBB57_16
	cmp w13, #58
	b.eq LBB57_19
	cmp w13, #62
	b.ne LBB57_22
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 113
		let removed = self.pos_to_piece[pos as usize];
	ldrb w4, [x1, #71]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 115
		self.boards[player as usize][removed as usize].unset(pos);
	add x5, x1, #80
	mov w6, #56
	umaddl x16, w16, w6, x5
	lsl x4, x4, #3
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 31
		*self &= INVERTED_SQUARE_MASKS[index as usize];
	ldr x7, [x16, x4]
	and x7, x7, #0x7fffffffffffffff
	str x7, [x16, x4]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 114
		self.pos_to_piece[pos as usize] = Piece::Empty;
	strb wzr, [x1, #71]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 31
		*self &= INVERTED_SQUARE_MASKS[index as usize];
	and x17, x17, #0x7fffffffffffffff
	str x17, [x15]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 113
		let removed = self.pos_to_piece[pos as usize];
	ldrb w17, [x1, #69]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 115
		self.boards[player as usize][removed as usize].unset(pos);
	umaddl x3, w3, w6, x5
	lsl x17, x17, #3
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 31
		*self &= INVERTED_SQUARE_MASKS[index as usize];
	ldr x4, [x3, x17]
	and x4, x4, #0xdfffffffffffffff
	str x4, [x3, x17]
	ldr x17, [x10]
	and x17, x17, #0xdfffffffffffffff
	str x17, [x10]
	and x9, x9, #0x7fffffffffffffff
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 21
		*self |= 1 << index; // SQUARE_MASKS[index as usize];
	ldr x10, [x16, #32]
	orr x10, x10, #0x2000000000000000
	str x10, [x16, #32]
	mov w10, #4
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 96
		self.pos_to_piece[pos as usize] = piece;
	strb w10, [x1, #69]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 21
		*self |= 1 << index; // SQUARE_MASKS[index as usize];
	ldr x10, [x15]
	orr x10, x10, #0x2000000000000000
	str x10, [x15]
	orr x9, x9, #0x2000000000000000
	b LBB57_21
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 160
		.remove_piece(self.side_to_move.opponent(), move_item.to_pos);
	ldrsb x9, [x2, #5]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 113
		let removed = self.pos_to_piece[pos as usize];
	cmp x9, #64
	b.hs LBB57_57
	and w13, w9, #0xff
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 137
		.remove_piece(self.side_to_move, move_item.from_pos);
	cmp w16, #0
	cset w10, eq
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 113
		let removed = self.pos_to_piece[pos as usize];
	ldrb w5, [x17, x9]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 114
		self.pos_to_piece[pos as usize] = Piece::Empty;
	strb wzr, [x17, x9]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 115
		self.boards[player as usize][removed as usize].unset(pos);
	add x6, x1, #8
	mov w7, #56
	umaddl x7, w10, w7, x6
	add x5, x7, x5, lsl #3
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 31
		*self &= INVERTED_SQUARE_MASKS[index as usize];
	ldr x7, [x4, x9, lsl #3]
	ldr x19, [x5, #72]
	and x19, x19, x7
	str x19, [x5, #72]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 116
		self.pos_to_player[player as usize].unset(pos);
	add x10, x6, w10, uxtw #3
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 31
		*self &= INVERTED_SQUARE_MASKS[index as usize];
	ldr x5, [x10, #184]
	and x5, x5, x7
	str x5, [x10, #184]
	ldr x10, [x1, #72]
	and x10, x10, x7
	str x10, [x1, #72]
LBB57_13:
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 137
		.remove_piece(self.side_to_move, move_item.from_pos);
	cmp w16, #0
	cset w10, eq
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 113
		let removed = self.pos_to_piece[pos as usize];
	ldrb w5, [x17, x9]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 114
		self.pos_to_piece[pos as usize] = Piece::Empty;
	strb wzr, [x17, x9]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 115
		self.boards[player as usize][removed as usize].unset(pos);
	add x6, x1, #80
	mov w7, #56
	umaddl x19, w10, w7, x6
	add x10, x1, w10, uxtw #3
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 31
		*self &= INVERTED_SQUARE_MASKS[index as usize];
	ldr x4, [x4, x9, lsl #3]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 115
		self.boards[player as usize][removed as usize].unset(pos);
	lsl x5, x5, #3
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 31
		*self &= INVERTED_SQUARE_MASKS[index as usize];
	ldr x20, [x19, x5]
	and x20, x20, x4
	str x20, [x19, x5]
	ldr x5, [x10, #192]
	and x5, x5, x4
	str x5, [x10, #192]
	ldr x10, [x1, #72]
	and x10, x10, x4
	str x10, [x1, #72]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 95
		self.boards[player as usize][piece as usize].set(pos);
	and x10, x3, #0xff
	umaddl x16, w16, w7, x6
	lsl x10, x10, #3
	mov w4, #1
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 21
		*self |= 1 << index; // SQUARE_MASKS[index as usize];
	ldr x5, [x16, x10]
	lsl x4, x4, x13
	orr x5, x5, x4
	str x5, [x16, x10]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 96
		self.pos_to_piece[pos as usize] = piece;
	strb w3, [x17, x9]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 21
		*self |= 1 << index; // SQUARE_MASKS[index as usize];
	ldr x9, [x15]
	orr x9, x9, x4
	str x9, [x15]
	ldr x9, [x1, #72]
	orr x9, x9, x4
	str x9, [x1, #72]
	b LBB57_23
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 203
		if !move_item.capturing && !move_item.castling {
	ldrb w9, [x2, #10]
	cbnz w9, LBB57_22
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 204
		self.half_move_clock += 1;
	add w9, w12, #1
	str w9, [x1, #208]
	b LBB57_23
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 174
		match (self.side_to_move, move_item.to_pos) {
	cmp w13, #2
	b.eq LBB57_20
	cmp w13, #6
	b.ne LBB57_22
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 113
		let removed = self.pos_to_piece[pos as usize];
	ldrb w4, [x1, #15]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 115
		self.boards[player as usize][removed as usize].unset(pos);
	add x5, x1, #80
	mov w6, #56
	umaddl x16, w16, w6, x5
	lsl x4, x4, #3
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 31
		*self &= INVERTED_SQUARE_MASKS[index as usize];
	ldr x7, [x16, x4]
	and x7, x7, #0xffffffffffffff7f
	str x7, [x16, x4]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 114
		self.pos_to_piece[pos as usize] = Piece::Empty;
	strb wzr, [x1, #15]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 31
		*self &= INVERTED_SQUARE_MASKS[index as usize];
	and x17, x17, #0xffffffffffffff7f
	str x17, [x15]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 113
		let removed = self.pos_to_piece[pos as usize];
	ldrb w17, [x1, #13]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 115
		self.boards[player as usize][removed as usize].unset(pos);
	umaddl x3, w3, w6, x5
	lsl x17, x17, #3
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 31
		*self &= INVERTED_SQUARE_MASKS[index as usize];
	ldr x4, [x3, x17]
	and x4, x4, #0xffffffffffffffdf
	str x4, [x3, x17]
	ldr x17, [x10]
	and x17, x17, #0xffffffffffffffdf
	str x17, [x10]
	and x9, x9, #0xffffffffffffff7f
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 21
		*self |= 1 << index; // SQUARE_MASKS[index as usize];
	ldr x10, [x16, #32]
	orr x10, x10, #0x20
	str x10, [x16, #32]
	mov w10, #4
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 96
		self.pos_to_piece[pos as usize] = piece;
	strb w10, [x1, #13]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 21
		*self |= 1 << index; // SQUARE_MASKS[index as usize];
	ldr x10, [x15]
	orr x10, x10, #0x20
	str x10, [x15]
	orr x9, x9, #0x20
	b LBB57_21
LBB57_19:
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 113
		let removed = self.pos_to_piece[pos as usize];
	ldrb w4, [x1, #64]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 115
		self.boards[player as usize][removed as usize].unset(pos);
	add x5, x1, #80
	mov w6, #56
	umaddl x16, w16, w6, x5
	lsl x4, x4, #3
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 31
		*self &= INVERTED_SQUARE_MASKS[index as usize];
	ldr x7, [x16, x4]
	and x7, x7, #0xfeffffffffffffff
	str x7, [x16, x4]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 114
		self.pos_to_piece[pos as usize] = Piece::Empty;
	strb wzr, [x1, #64]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 31
		*self &= INVERTED_SQUARE_MASKS[index as usize];
	and x17, x17, #0xfeffffffffffffff
	str x17, [x15]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 113
		let removed = self.pos_to_piece[pos as usize];
	ldrb w17, [x1, #67]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 115
		self.boards[player as usize][removed as usize].unset(pos);
	umaddl x3, w3, w6, x5
	lsl x17, x17, #3
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 31
		*self &= INVERTED_SQUARE_MASKS[index as usize];
	ldr x4, [x3, x17]
	and x4, x4, #0xf7ffffffffffffff
	str x4, [x3, x17]
	ldr x17, [x10]
	and x17, x17, #0xf7ffffffffffffff
	str x17, [x10]
	and x9, x9, #0xfeffffffffffffff
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 21
		*self |= 1 << index; // SQUARE_MASKS[index as usize];
	ldr x10, [x16, #32]
	orr x10, x10, #0x800000000000000
	str x10, [x16, #32]
	mov w10, #4
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 96
		self.pos_to_piece[pos as usize] = piece;
	strb w10, [x1, #67]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 21
		*self |= 1 << index; // SQUARE_MASKS[index as usize];
	ldr x10, [x15]
	orr x10, x10, #0x800000000000000
	str x10, [x15]
	orr x9, x9, #0x800000000000000
	b LBB57_21
LBB57_20:
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 113
		let removed = self.pos_to_piece[pos as usize];
	ldrb w4, [x1, #8]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 115
		self.boards[player as usize][removed as usize].unset(pos);
	add x5, x1, #80
	mov w6, #56
	umaddl x16, w16, w6, x5
	lsl x4, x4, #3
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 31
		*self &= INVERTED_SQUARE_MASKS[index as usize];
	ldr x7, [x16, x4]
	and x7, x7, #0xfffffffffffffffe
	str x7, [x16, x4]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 114
		self.pos_to_piece[pos as usize] = Piece::Empty;
	strb wzr, [x1, #8]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 31
		*self &= INVERTED_SQUARE_MASKS[index as usize];
	and x17, x17, #0xfffffffffffffffe
	str x17, [x15]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 113
		let removed = self.pos_to_piece[pos as usize];
	ldrb w17, [x1, #11]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 115
		self.boards[player as usize][removed as usize].unset(pos);
	umaddl x3, w3, w6, x5
	lsl x17, x17, #3
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 31
		*self &= INVERTED_SQUARE_MASKS[index as usize];
	ldr x4, [x3, x17]
	and x4, x4, #0xfffffffffffffff7
	str x4, [x3, x17]
	ldr x17, [x10]
	and x17, x17, #0xfffffffffffffff7
	str x17, [x10]
	and x9, x9, #0xfffffffffffffffe
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 21
		*self |= 1 << index; // SQUARE_MASKS[index as usize];
	ldr x10, [x16, #32]
	orr x10, x10, #0x8
	str x10, [x16, #32]
	mov w10, #4
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 96
		self.pos_to_piece[pos as usize] = piece;
	strb w10, [x1, #11]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 21
		*self |= 1 << index; // SQUARE_MASKS[index as usize];
	ldr x10, [x15]
	orr x10, x10, #0x8
	str x10, [x15]
	orr x9, x9, #0x8
LBB57_21:
	str x9, [x1, #72]
LBB57_22:
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 206
		self.half_move_clock = 0;
	str wzr, [x1, #208]
LBB57_23:
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 213
		if move_item.double {
	ldrb w9, [x2, #11]
	cbz w9, LBB57_30
	and w9, w8, #0xf8
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/square.rs : 33
		rank: pos / 8,
	lsr w10, w13, #3
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 223
		if to_rank > from_rank {
	cmp w10, w9, lsr #3
	add w9, w10, #31
	csinc w9, w9, w10, hi
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/square.rs : 42
		8 * self.rank + self.file
	mov x10, x13
	bfi w10, w9, #3, #29
	and w15, w10, #0xff
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 229
		self.enpassant_square = SQUARE_MASKS[<Square as Into<i8>>::into(Square {
	sxtb x9, w10
	cmp w15, #64
	b.hs LBB57_59
Lloh119:
	adrp x10, l___unnamed_39@PAGE
Lloh120:
	add x10, x10, l___unnamed_39@PAGEOFF
	ldr x9, [x10, x9, lsl #3]
	ushll.8h v0, v0, #0
	str x9, [x1]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/player.rs : 1
		#[derive(Debug, PartialEq, Clone, Copy)]
	ldrb w9, [x1, #220]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 239
		if self.side_to_move == Player::Black {
	cbz w9, LBB57_31
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 240
		self.full_move_number += 1;
	ldr w10, [x1, #212]
	add w10, w10, #1
	str w10, [x1, #212]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 245
		if move_item.castling || move_item.piece == Piece::King {
	cmp w14, #6
	b.eq LBB57_39
	ldrb w10, [x2, #13]
	cbnz w10, LBB57_39
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 255
		if move_item.piece == Piece::Rook {
	cmp w14, #4
	b.eq LBB57_40
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/pieces.rs : 14
		#[derive(PartialEq, Debug, Clone, Copy)]
	ldrb w8, [x2, #8]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 272
		if move_item.captured_piece == Piece::Rook {
	cmp w8, #4
	b.eq LBB57_46
	b LBB57_55
	mov x9, #0
	ushll.8h v0, v0, #0
	str x9, [x1]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/player.rs : 1
		#[derive(Debug, PartialEq, Clone, Copy)]
	ldrb w9, [x1, #220]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 239
		if self.side_to_move == Player::Black {
	cbnz w9, LBB57_26
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 245
		if move_item.castling || move_item.piece == Piece::King {
	cmp w14, #6
	b.eq LBB57_33
	ldrb w10, [x2, #13]
	cbz w10, LBB57_34
LBB57_33:
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 26
		self.white_queen_side = false;
	strh wzr, [x1, #216]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 255
		if move_item.piece == Piece::Rook {
	cmp w14, #4
	b.ne LBB57_44
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 256
		match (self.side_to_move, move_item.from_pos) {
	cbnz w9, LBB57_40
	cbz w8, LBB57_56
	and w8, w8, #0xff
	cmp w8, #7
	b.ne LBB57_44
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 261
		self.castle_permissions.white_king_side = false;
	strb wzr, [x1, #217]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/pieces.rs : 14
		#[derive(PartialEq, Debug, Clone, Copy)]
	ldrb w8, [x2, #8]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 272
		if move_item.captured_piece == Piece::Rook {
	cmp w8, #4
	b.eq LBB57_45
	b LBB57_55
LBB57_39:
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 30
		self.black_queen_side = false;
	strh wzr, [x1, #218]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 255
		if move_item.piece == Piece::Rook {
	cmp w14, #4
	b.ne LBB57_44
LBB57_40:
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 256
		match (self.side_to_move, move_item.from_pos) {
	cmp w8, #56
	b.eq LBB57_43
	and w8, w8, #0xff
	cmp w8, #63
	b.ne LBB57_44
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 267
		self.castle_permissions.black_king_side = false;
	strb wzr, [x1, #219]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/pieces.rs : 14
		#[derive(PartialEq, Debug, Clone, Copy)]
	ldrb w8, [x2, #8]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 272
		if move_item.captured_piece == Piece::Rook {
	cmp w8, #4
	b.eq LBB57_45
	b LBB57_55
LBB57_43:
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 264
		self.castle_permissions.black_queen_side = false;
	strb wzr, [x1, #218]
LBB57_44:
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/pieces.rs : 14
		#[derive(PartialEq, Debug, Clone, Copy)]
	ldrb w8, [x2, #8]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 272
		if move_item.captured_piece == Piece::Rook {
	cmp w8, #4
	b.ne LBB57_55
LBB57_45:
	cbz w9, LBB57_49
LBB57_46:
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 273
		match (self.side_to_move.opponent(), move_item.to_pos) {
	cbz w13, LBB57_52
	cmp w13, #7
	b.ne LBB57_54
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 278
		self.castle_permissions.white_king_side = false;
	strb wzr, [x1, #217]
	b LBB57_54
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 273
		match (self.side_to_move.opponent(), move_item.to_pos) {
	cmp w13, #56
	b.eq LBB57_53
	cmp w13, #63
	b.ne LBB57_54
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 284
		self.castle_permissions.black_king_side = false;
	strb wzr, [x1, #219]
	b LBB57_54
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 275
		self.castle_permissions.white_queen_side = false;
	strb wzr, [x1, #216]
	b LBB57_54
LBB57_53:
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 281
		self.castle_permissions.black_queen_side = false;
	strb wzr, [x1, #218]
LBB57_54:
	mov w8, #4
LBB57_55:
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 291
		self.side_to_move = self.side_to_move.opponent();
	eor w9, w9, #0x1
	strb w9, [x1, #220]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 293
		UnmakeMoveMetadata {
	xtn.8b v0, v0
	str s0, [x0, #8]
	str x11, [x0]
	str w12, [x0, #12]
	strb w8, [x0, #16]
	.cfi_def_cfa wsp, 48
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 299
		}
	ldp x29, x30, [sp, #32]
	ldp x20, x19, [sp, #16]
	ldp x22, x21, [sp], #48
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	.cfi_restore w19
	.cfi_restore w20
	.cfi_restore w21
	.cfi_restore w22
	ret
	.cfi_restore_state
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 258
		self.castle_permissions.white_queen_side = false;
	strb wzr, [x1, #216]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/pieces.rs : 14
		#[derive(PartialEq, Debug, Clone, Copy)]
	ldrb w8, [x2, #8]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 272
		if move_item.captured_piece == Piece::Rook {
	cmp w8, #4
	b.eq LBB57_45
	b LBB57_55
LBB57_57:
Lloh121:
	adrp x2, l___unnamed_40@PAGE
Lloh122:
	add x2, x2, l___unnamed_40@PAGEOFF
	mov x0, x9
	mov w1, #64
	bl core::panicking::panic_bounds_check
LBB57_58:
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 113
		let removed = self.pos_to_piece[pos as usize];
Lloh123:
	adrp x2, l___unnamed_40@PAGE
Lloh124:
	add x2, x2, l___unnamed_40@PAGEOFF
	mov x0, x8
	mov w1, #64
	bl core::panicking::panic_bounds_check
LBB57_59:
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 229
		self.enpassant_square = SQUARE_MASKS[<Square as Into<i8>>::into(Square {
Lloh125:
	adrp x2, l___unnamed_41@PAGE
Lloh126:
	add x2, x2, l___unnamed_41@PAGEOFF
	mov x0, x9
	mov w1, #64
	bl core::panicking::panic_bounds_check
LBB57_60:
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 113
		let removed = self.pos_to_piece[pos as usize];
Lloh127:
	adrp x2, l___unnamed_40@PAGE
Lloh128:
	add x2, x2, l___unnamed_40@PAGEOFF
	mov x0, x10
	mov w1, #64
	bl core::panicking::panic_bounds_check
	.loh AdrpAdd	Lloh117, Lloh118
	.loh AdrpAdd	Lloh119, Lloh120
	.loh AdrpAdd	Lloh121, Lloh122
	.loh AdrpAdd	Lloh123, Lloh124
	.loh AdrpAdd	Lloh125, Lloh126
	.loh AdrpAdd	Lloh127, Lloh128
.section __TEXT,__text,regular,pure_instructions
	.p2align	2
engine::moves::attacked::square_attacked::is_square_attacked:
Lfunc_begin55:
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/attacked/square_attacked.rs : 123
		pub fn is_square_attacked(
	.cfi_startproc
	stp x29, x30, [sp, #-16]!
	.cfi_def_cfa_offset 16
	mov x29, sp
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	.cfi_remember_state
	and w8, w0, #0xff
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/attacked/square_attacked.rs : 20
		let knight_move_mask = cache.knight_moves_masks[pos as usize];
	sxtb x0, w0
	cmp w8, #63
	b.hi LBB55_4
	ldr x10, [x2, #72]
	add x11, x3, x0, lsl #3
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/attacked/square_attacked.rs : 31
		cache.rook_potential_blockers_masks[pos as usize],
	ldr x8, [x11, #5632]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/precalculate/magic_bitboards.rs : 33
		let blockers_on_path = blockers & potential_blockers_mask;
	and x8, x8, x10
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/attacked/square_attacked.rs : 33
		cache.rook_magics[pos as usize],
	ldr x9, [x11, #6272]
		// /rustc/82e1608dfa6e0b5569232559e3d385fea5a93112/library/core/src/num/mod.rs : 1169
		uint_impl! {
	mul x8, x8, x9
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/attacked/square_attacked.rs : 34
		cache.rook_bit_counts[pos as usize],
	add x13, x3, x0
	mov w9, #6208
	ldrb w9, [x13, x9]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/precalculate/magic_bitboards.rs : 35
		let index = (hash >> (64 - bit_count)) as usize;
	neg w9, w9
	lsr x8, x8, x9
	mov w9, #24
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/attacked/square_attacked.rs : 36
		let rook_moves_mask = cache.rook_magic_attack_tables[pos as usize][rook_magic_index];
	smaddl x12, w0, w9, x3
	ldr x9, [x12, #16]
		// /rustc/82e1608dfa6e0b5569232559e3d385fea5a93112/library/core/src/slice/index.rs : 258
		&(*slice)[self]
	cmp x8, x9
	b.hs LBB55_5
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/attacked/square_attacked.rs : 52
		cache.bishop_potential_blockers_masks[pos as usize],
	ldr x9, [x11, #5120]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/precalculate/magic_bitboards.rs : 33
		let blockers_on_path = blockers & potential_blockers_mask;
	and x9, x9, x10
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/attacked/square_attacked.rs : 54
		cache.bishop_magics[pos as usize],
	ldr x10, [x11, #6784]
		// /rustc/82e1608dfa6e0b5569232559e3d385fea5a93112/library/core/src/num/mod.rs : 1169
		uint_impl! {
	mul x9, x9, x10
	mov w10, #6144
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/attacked/square_attacked.rs : 55
		cache.bishop_bit_counts[pos as usize],
	ldrb w10, [x13, x10]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/precalculate/magic_bitboards.rs : 35
		let index = (hash >> (64 - bit_count)) as usize;
	neg w10, w10
	lsr x9, x9, x10
	mov w10, #24
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/attacked/square_attacked.rs : 57
		let bishop_moves_mask = cache.bishop_magic_attack_tables[pos as usize][bishop_magic_index];
	smaddl x13, w0, w10, x3
	ldr x10, [x13, #1552]
		// /rustc/82e1608dfa6e0b5569232559e3d385fea5a93112/library/core/src/slice/index.rs : 258
		&(*slice)[self]
	cmp x9, x10
	b.hs LBB55_6
	ldr x10, [x11, #4096]
	mov w11, w1
	mov w14, #56
	umaddl x14, w1, w14, x2
	ldr x14, [x14, #96]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/attacked/square_attacked.rs : 57
		let bishop_moves_mask = cache.bishop_magic_attack_tables[pos as usize][bishop_magic_index];
	add x13, x13, #1536
	ldr x13, [x13]
	mov w15, #56
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 88
		return &self.boards[player as usize][piece as usize];
	umaddl x11, w11, w15, x2
		// /rustc/82e1608dfa6e0b5569232559e3d385fea5a93112/library/core/src/ops/bit.rs : 178
		bitand_impl! { bool usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 }
	ldr q0, [x11, #112]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/attacked/square_attacked.rs : 36
		let rook_moves_mask = cache.rook_magic_attack_tables[pos as usize][rook_magic_index];
	ldr x12, [x12]
		// /rustc/82e1608dfa6e0b5569232559e3d385fea5a93112/library/core/src/slice/index.rs : 258
		&(*slice)[self]
	add x8, x12, x8, lsl #3
	add x9, x13, x9, lsl #3
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 88
		return &self.boards[player as usize][piece as usize];
	add x12, x11, #104
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/player.rs : 15
		match self {
	eor w13, w1, #0x1
		// /rustc/82e1608dfa6e0b5569232559e3d385fea5a93112/library/core/src/ops/bit.rs : 178
		bitand_impl! { bool usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 }
	and x10, x14, x10
		// /rustc/82e1608dfa6e0b5569232559e3d385fea5a93112/library/core/src/num/mod.rs : 1169
		uint_impl! {
	fmov d1, x10
	cnt.8b v1, v1
	uaddlv.8b h1, v1
	fmov w10, s1
		// /rustc/82e1608dfa6e0b5569232559e3d385fea5a93112/library/core/src/ops/bit.rs : 178
		bitand_impl! { bool usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 }
	ld1r.2d { v1 }, [x12]
	zip2.2d v1, v0, v1
	ld1r.2d { v2 }, [x8]
	ld1r.2d { v3 }, [x9]
	and.16b v1, v1, v3
	and.16b v0, v0, v2
		// /rustc/82e1608dfa6e0b5569232559e3d385fea5a93112/library/core/src/num/mod.rs : 1169
		uint_impl! {
	cnt.16b v1, v1
	uaddlp.8h v1, v1
	uaddlp.4s v1, v1
	uaddlp.2d v1, v1
	cnt.16b v0, v0
	uaddlp.8h v0, v0
	uaddlp.4s v0, v0
	uaddlp.2d v0, v0
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/attacked/square_attacked.rs : 42
		attacked_count += attacking_rooks.count_ones() as i8;
	uzp1.4s v0, v0, v1
	xtn.4h v0, v0
		// /rustc/82e1608dfa6e0b5569232559e3d385fea5a93112/library/core/src/ops/bit.rs : 178
		bitand_impl! { bool usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 }
	ldr x8, [x11, #88]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/attacked/square_attacked.rs : 115
		let king_move_mask = cache.king_moves_masks[pos as usize];
	add x9, x3, x0, lsl #3
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/attacked/square_attacked.rs : 78
		let attacking_mask = cache.pawn_attack_moves_mask[attacker.opponent() as usize][pos as usize];
	add x12, x9, x13, lsl #9
	ldr x12, [x12, #3072]
		// /rustc/82e1608dfa6e0b5569232559e3d385fea5a93112/library/core/src/ops/bit.rs : 178
		bitand_impl! { bool usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 }
	and x8, x12, x8
		// /rustc/82e1608dfa6e0b5569232559e3d385fea5a93112/library/core/src/num/mod.rs : 1169
		uint_impl! {
	fmov d1, x8
	cnt.8b v1, v1
	uaddlv.8b h1, v1
	fmov w8, s1
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/attacked/square_attacked.rs : 115
		let king_move_mask = cache.king_moves_masks[pos as usize];
	ldr x9, [x9, #4608]
		// /rustc/82e1608dfa6e0b5569232559e3d385fea5a93112/library/core/src/ops/bit.rs : 178
		bitand_impl! { bool usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 }
	ldr x11, [x11, #128]
	and x9, x11, x9
		// /rustc/82e1608dfa6e0b5569232559e3d385fea5a93112/library/core/src/num/mod.rs : 1169
		uint_impl! {
	fmov d1, x9
	cnt.8b v1, v1
	uaddlv.8b h1, v1
	fmov w9, s1
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/attacked/square_attacked.rs : 117
		attacked_count += attacking_king.count_ones() as i8;
	addv.4h h0, v0
	fmov w11, s0
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/attacked/square_attacked.rs : 42
		attacked_count += attacking_rooks.count_ones() as i8;
	add w10, w11, w10
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/attacked/square_attacked.rs : 117
		attacked_count += attacking_king.count_ones() as i8;
	add w8, w8, w9
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/attacked/square_attacked.rs : 80
		attacked_count += attacking_pawns.count_ones() as i8;
	add w8, w10, w8
	sxtb w8, w8
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/attacked/square_attacked.rs : 129
		return times_square_attacked(pos, attacker, game, cache) > 0;
	cmp w8, #0
	cset w0, gt
	.cfi_def_cfa wsp, 16
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/attacked/square_attacked.rs : 213
		}
	ldp x29, x30, [sp], #16
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
LBB55_4:
	.cfi_restore_state
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/attacked/square_attacked.rs : 20
		let knight_move_mask = cache.knight_moves_masks[pos as usize];
Lloh109:
	adrp x2, l___unnamed_35@PAGE
Lloh110:
	add x2, x2, l___unnamed_35@PAGEOFF
	mov w1, #64
	bl core::panicking::panic_bounds_check
LBB55_5:
		// /rustc/82e1608dfa6e0b5569232559e3d385fea5a93112/library/core/src/slice/index.rs : 258
		&(*slice)[self]
Lloh111:
	adrp x2, l___unnamed_36@PAGE
Lloh112:
	add x2, x2, l___unnamed_36@PAGEOFF
	mov x0, x8
	mov x1, x9
	bl core::panicking::panic_bounds_check
LBB55_6:
Lloh113:
	adrp x2, l___unnamed_37@PAGE
Lloh114:
	add x2, x2, l___unnamed_37@PAGEOFF
	mov x0, x9
	mov x1, x10
	bl core::panicking::panic_bounds_check
	.loh AdrpAdd	Lloh109, Lloh110
	.loh AdrpAdd	Lloh111, Lloh112
	.loh AdrpAdd	Lloh113, Lloh114
Lfunc_end55:
	.cfi_endproc

	.p2align	2
engine::moves::perft::_perft_unmake:
Lfunc_begin56:
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/perft.rs : 126
		fn _perft_unmake(game: &mut GameState, cache: &PrecalculatedCache, depth: u16) -> u64 {
	.cfi_startproc
	stp x28, x27, [sp, #-96]!
	.cfi_def_cfa_offset 96
	stp x26, x25, [sp, #16]
	stp x24, x23, [sp, #32]
	stp x22, x21, [sp, #48]
	stp x20, x19, [sp, #64]
	stp x29, x30, [sp, #80]
	add x29, sp, #80
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	.cfi_offset w19, -24
	.cfi_offset w20, -32
	.cfi_offset w21, -40
	.cfi_offset w22, -48
	.cfi_offset w23, -56
	.cfi_offset w24, -64
	.cfi_offset w25, -72
	.cfi_offset w26, -80
	.cfi_offset w27, -88
	.cfi_offset w28, -96
	.cfi_remember_state
	sub sp, sp, #1, lsl #12
	sub sp, sp, #48
	tst w2, #0xffff
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/perft.rs : 129
		if depth == 0 {
	b.eq LBB56_7
	mov x22, x2
	mov x19, x1
	mov x20, x0
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/movelist.rs : 12
		MoveList {
	str xzr, [sp, #4104]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/perft.rs : 137
		game.side_to_move,
	ldrb w2, [x0, #220]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/perft.rs : 134
		super::pseudolegal::all::generate_pseudolegal_moves(
	add x0, sp, #8
	mov x1, x20
	mov x3, x19
	bl engine::moves::pseudolegal::all::generate_pseudolegal_moves
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/movelist.rs : 27
		self.end
	ldr x25, [sp, #4104]
		// /rustc/82e1608dfa6e0b5569232559e3d385fea5a93112/library/core/src/iter/range.rs : 729
		if self.start < self.end {
	cbz x25, LBB56_8
	mov x21, #0
	mov x26, #0
	sub w22, w22, #1
	add x23, sp, #8
	mov w27, #56
	b LBB56_4
LBB56_3:
	add x26, x26, #1
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/perft.rs : 150
		game.unmake_move(&move_item, unmake_metadata);
	sub x2, x29, #112
	mov x0, x20
	mov x1, x23
	bl engine::state::game::GameState::unmake_move
		// /rustc/82e1608dfa6e0b5569232559e3d385fea5a93112/library/core/src/cmp.rs : 1567
		ord_impl! { char usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 }
	add x23, x23, #16
		// /rustc/82e1608dfa6e0b5569232559e3d385fea5a93112/library/core/src/iter/range.rs : 729
		if self.start < self.end {
	cmp x25, x26
	b.eq LBB56_9
LBB56_4:
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/perft.rs : 141
		let move_item = &move_list.moves[index];
	cmp x26, #256
	b.eq LBB56_10
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/perft.rs : 142
		let player = game.side_to_move;
	ldrb w28, [x20, #220]
	cmp x28, #0
	cset w24, eq
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/perft.rs : 143
		let unmake_metadata = game.make_move(move_item);
	sub x0, x29, #112
	mov x1, x20
	mov x2, x23
	bl engine::state::game::GameState::make_move
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 88
		return &self.boards[player as usize][piece as usize];
	umaddl x8, w28, w27, x20
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/attacked/in_check.rs : 10
		let king = game
	ldr x8, [x8, #128]
		// /rustc/82e1608dfa6e0b5569232559e3d385fea5a93112/library/core/src/num/mod.rs : 1169
		uint_impl! {
	rbit x8, x8
	clz x0, x8
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/attacked/in_check.rs : 15
		return is_square_attacked(
	mov x1, x24
	mov x2, x20
	mov x3, x19
	bl engine::moves::attacked::square_attacked::is_square_attacked
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/perft.rs : 145
		if !is_in_check(player, game, cache) {
	tbnz w0, #0, LBB56_3
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/perft.rs : 146
		let move_nodes = _perft_unmake(game, cache, depth - 1);
	mov x0, x20
	mov x1, x19
	mov x2, x22
	bl engine::moves::perft::_perft_unmake
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/perft.rs : 147
		nodes += move_nodes;
	add x21, x0, x21
	b LBB56_3
LBB56_7:
	mov w21, #1
	b LBB56_9
	mov x21, #0
LBB56_9:
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/perft.rs : 154
		}
	mov x0, x21
	add sp, sp, #1, lsl #12
	add sp, sp, #48
	.cfi_def_cfa wsp, 96
	ldp x29, x30, [sp, #80]
	ldp x20, x19, [sp, #64]
	ldp x22, x21, [sp, #48]
	ldp x24, x23, [sp, #32]
	ldp x26, x25, [sp, #16]
	ldp x28, x27, [sp], #96
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	.cfi_restore w19
	.cfi_restore w20
	.cfi_restore w21
	.cfi_restore w22
	.cfi_restore w23
	.cfi_restore w24
	.cfi_restore w25
	.cfi_restore w26
	.cfi_restore w27
	.cfi_restore w28
	ret
LBB56_10:
	.cfi_restore_state
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/perft.rs : 141
		let move_item = &move_list.moves[index];
Lloh115:
	adrp x2, l___unnamed_38@PAGE
Lloh116:
	add x2, x2, l___unnamed_38@PAGEOFF
	mov w0, #256
	mov w1, #256
	bl core::panicking::panic_bounds_check
	.loh AdrpAdd	Lloh115, Lloh116
Lfunc_end56:
	.cfi_endproc

	.p2align	2
engine::state::game::GameState::make_move:
Lfunc_begin57:
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 127
		pub fn make_move(&mut self, move_item: &MoveItem) -> UnmakeMoveMetadata {
	.cfi_startproc
	stp x22, x21, [sp, #-48]!
	.cfi_def_cfa_offset 48
	stp x20, x19, [sp, #16]
	stp x29, x30, [sp, #32]
	add x29, sp, #32
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	.cfi_offset w19, -24
	.cfi_offset w20, -32
	.cfi_offset w21, -40
	.cfi_offset w22, -48
	.cfi_remember_state
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 137
		.remove_piece(self.side_to_move, move_item.from_pos);
	ldrsb x8, [x2, #4]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 113
		let removed = self.pos_to_piece[pos as usize];
	cmp x8, #64
	b.hs LBB57_58
	ldr s0, [x1, #216]
	ldr x11, [x1]
	ldr w12, [x1, #208]
	add x17, x1, #8
	ldrb w16, [x1, #220]
	ldrb w9, [x17, x8]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 114
		self.pos_to_piece[pos as usize] = Piece::Empty;
	strb wzr, [x17, x8]
	mov w10, #56
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 115
		self.boards[player as usize][removed as usize].unset(pos);
	umaddl x10, w16, w10, x17
	add x9, x10, x9, lsl #3
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 31
		*self &= INVERTED_SQUARE_MASKS[index as usize];
Lloh117:
	adrp x4, l___unnamed_25@PAGE
Lloh118:
	add x4, x4, l___unnamed_25@PAGEOFF
	ldr x10, [x4, x8, lsl #3]
	ldr x13, [x9, #72]
	and x13, x13, x10
	str x13, [x9, #72]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 116
		self.pos_to_player[player as usize].unset(pos);
	add x15, x17, x16, lsl #3
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 31
		*self &= INVERTED_SQUARE_MASKS[index as usize];
	ldr x9, [x15, #184]!
	and x9, x9, x10
	str x9, [x15]
	ldr x9, [x1, #72]
	and x9, x9, x10
	str x9, [x1, #72]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/pieces.rs : 14
		#[derive(PartialEq, Debug, Clone, Copy)]
	ldrb w14, [x2, #6]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 139
		if move_item.piece == Piece::Pawn {
	cmp w14, #1
	b.ne LBB57_5
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 140
		let final_piece = if move_item.promoting {
	ldrb w9, [x2, #9]
	ldrb w10, [x2, #7]
	cmp w9, #0
	csinc w3, w10, wzr, ne
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 147
		if move_item.enpassant {
	ldrb w9, [x2, #12]
	cbz w9, LBB57_11
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/square.rs : 57
		8 * self.rank + self.file
	and x10, x8, #0xfffffffffffffff8
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 149
		let to = Square::from(move_item.to_pos);
	ldrsb x9, [x2, #5]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/square.rs : 49
		file: pos % 8,
	ubfx w13, w9, #12, #3
	add w13, w9, w13
	and w13, w13, #0xf8
	sub w13, w9, w13
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/square.rs : 57
		8 * self.rank + self.file
	add x10, x10, w13, sxtb
	and w13, w10, #0xff
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 113
		let removed = self.pos_to_piece[pos as usize];
	cmp w13, #64
	b.hs LBB57_60
	and w13, w9, #0xff
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 137
		.remove_piece(self.side_to_move, move_item.from_pos);
	cmp w16, #0
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 113
		let removed = self.pos_to_piece[pos as usize];
	ldrb w5, [x17, x10]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 114
		self.pos_to_piece[pos as usize] = Piece::Empty;
	strb wzr, [x17, x10]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 137
		.remove_piece(self.side_to_move, move_item.from_pos);
	cset w6, eq
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 115
		self.boards[player as usize][removed as usize].unset(pos);
	add x7, x1, #8
	mov w19, #56
	umaddl x19, w6, w19, x7
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 31
		*self &= INVERTED_SQUARE_MASKS[index as usize];
	ldr x10, [x4, x10, lsl #3]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 115
		self.boards[player as usize][removed as usize].unset(pos);
	add x5, x19, x5, lsl #3
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 31
		*self &= INVERTED_SQUARE_MASKS[index as usize];
	ldr x19, [x5, #72]
	and x19, x19, x10
	str x19, [x5, #72]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 116
		self.pos_to_player[player as usize].unset(pos);
	add x5, x7, w6, uxtw #3
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 31
		*self &= INVERTED_SQUARE_MASKS[index as usize];
	ldr x6, [x5, #184]
	and x6, x6, x10
	str x6, [x5, #184]
	ldr x5, [x1, #72]
	and x10, x5, x10
	str x10, [x1, #72]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 113
		let removed = self.pos_to_piece[pos as usize];
	cmp w13, #64
	b.lo LBB57_13
	b LBB57_57
LBB57_5:
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 167
		.remove_piece(self.side_to_move.opponent(), move_item.to_pos);
	ldrsb x9, [x2, #5]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 113
		let removed = self.pos_to_piece[pos as usize];
	cmp x9, #64
	b.hs LBB57_57
	and w13, w9, #0xff
	and x5, x9, #0xff
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 137
		.remove_piece(self.side_to_move, move_item.from_pos);
	cmp w16, #0
	cset w3, eq
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 113
		let removed = self.pos_to_piece[pos as usize];
	ldrb w10, [x17, x9]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 114
		self.pos_to_piece[pos as usize] = Piece::Empty;
	strb wzr, [x17, x9]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 115
		self.boards[player as usize][removed as usize].unset(pos);
	add x6, x1, #80
	mov w7, #56
	umaddl x19, w3, w7, x6
	lsl x20, x10, #3
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 31
		*self &= INVERTED_SQUARE_MASKS[index as usize];
	ldr x4, [x4, x9, lsl #3]
	ldr x21, [x19, x20]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 115
		self.boards[player as usize][removed as usize].unset(pos);
	add x10, x1, w3, uxtw #3
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 31
		*self &= INVERTED_SQUARE_MASKS[index as usize];
	and x21, x21, x4
	str x21, [x19, x20]
	ldr x20, [x10, #192]!
	and x20, x20, x4
	ldr x21, [x1, #72]
	and x21, x21, x4
	str x21, [x1, #72]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 113
		let removed = self.pos_to_piece[pos as usize];
	ldrb w21, [x17, x9]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 114
		self.pos_to_piece[pos as usize] = Piece::Empty;
	strb wzr, [x17, x9]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 115
		self.boards[player as usize][removed as usize].unset(pos);
	lsl x21, x21, #3
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 31
		*self &= INVERTED_SQUARE_MASKS[index as usize];
	ldr x22, [x19, x21]
	and x22, x22, x4
	str x22, [x19, x21]
	str x20, [x10]
	ldr x19, [x1, #72]
	and x4, x19, x4
	str x4, [x1, #72]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 95
		self.boards[player as usize][piece as usize].set(pos);
	umaddl x4, w16, w7, x6
	lsl x6, x14, #3
	mov w7, #1
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 21
		*self |= 1 << index; // SQUARE_MASKS[index as usize];
	lsl x5, x7, x5
	ldr x7, [x4, x6]
	orr x7, x7, x5
	str x7, [x4, x6]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 96
		self.pos_to_piece[pos as usize] = piece;
	strb w14, [x17, x9]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 21
		*self |= 1 << index; // SQUARE_MASKS[index as usize];
	ldr x9, [x15]
	orr x17, x9, x5
	str x17, [x15]
	ldr x9, [x1, #72]
	orr x9, x9, x5
	str x9, [x1, #72]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 172
		if move_item.castling {
	ldrb w4, [x2, #13]
	cbz w4, LBB57_14
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 174
		match (self.side_to_move, move_item.to_pos) {
	cbz w16, LBB57_16
	cmp w13, #58
	b.eq LBB57_19
	cmp w13, #62
	b.ne LBB57_22
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 113
		let removed = self.pos_to_piece[pos as usize];
	ldrb w4, [x1, #71]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 115
		self.boards[player as usize][removed as usize].unset(pos);
	add x5, x1, #80
	mov w6, #56
	umaddl x16, w16, w6, x5
	lsl x4, x4, #3
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 31
		*self &= INVERTED_SQUARE_MASKS[index as usize];
	ldr x7, [x16, x4]
	and x7, x7, #0x7fffffffffffffff
	str x7, [x16, x4]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 114
		self.pos_to_piece[pos as usize] = Piece::Empty;
	strb wzr, [x1, #71]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 31
		*self &= INVERTED_SQUARE_MASKS[index as usize];
	and x17, x17, #0x7fffffffffffffff
	str x17, [x15]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 113
		let removed = self.pos_to_piece[pos as usize];
	ldrb w17, [x1, #69]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 115
		self.boards[player as usize][removed as usize].unset(pos);
	umaddl x3, w3, w6, x5
	lsl x17, x17, #3
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 31
		*self &= INVERTED_SQUARE_MASKS[index as usize];
	ldr x4, [x3, x17]
	and x4, x4, #0xdfffffffffffffff
	str x4, [x3, x17]
	ldr x17, [x10]
	and x17, x17, #0xdfffffffffffffff
	str x17, [x10]
	and x9, x9, #0x7fffffffffffffff
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 21
		*self |= 1 << index; // SQUARE_MASKS[index as usize];
	ldr x10, [x16, #32]
	orr x10, x10, #0x2000000000000000
	str x10, [x16, #32]
	mov w10, #4
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 96
		self.pos_to_piece[pos as usize] = piece;
	strb w10, [x1, #69]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 21
		*self |= 1 << index; // SQUARE_MASKS[index as usize];
	ldr x10, [x15]
	orr x10, x10, #0x2000000000000000
	str x10, [x15]
	orr x9, x9, #0x2000000000000000
	b LBB57_21
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 160
		.remove_piece(self.side_to_move.opponent(), move_item.to_pos);
	ldrsb x9, [x2, #5]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 113
		let removed = self.pos_to_piece[pos as usize];
	cmp x9, #64
	b.hs LBB57_57
	and w13, w9, #0xff
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 137
		.remove_piece(self.side_to_move, move_item.from_pos);
	cmp w16, #0
	cset w10, eq
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 113
		let removed = self.pos_to_piece[pos as usize];
	ldrb w5, [x17, x9]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 114
		self.pos_to_piece[pos as usize] = Piece::Empty;
	strb wzr, [x17, x9]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 115
		self.boards[player as usize][removed as usize].unset(pos);
	add x6, x1, #8
	mov w7, #56
	umaddl x7, w10, w7, x6
	add x5, x7, x5, lsl #3
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 31
		*self &= INVERTED_SQUARE_MASKS[index as usize];
	ldr x7, [x4, x9, lsl #3]
	ldr x19, [x5, #72]
	and x19, x19, x7
	str x19, [x5, #72]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 116
		self.pos_to_player[player as usize].unset(pos);
	add x10, x6, w10, uxtw #3
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 31
		*self &= INVERTED_SQUARE_MASKS[index as usize];
	ldr x5, [x10, #184]
	and x5, x5, x7
	str x5, [x10, #184]
	ldr x10, [x1, #72]
	and x10, x10, x7
	str x10, [x1, #72]
LBB57_13:
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 137
		.remove_piece(self.side_to_move, move_item.from_pos);
	cmp w16, #0
	cset w10, eq
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 113
		let removed = self.pos_to_piece[pos as usize];
	ldrb w5, [x17, x9]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 114
		self.pos_to_piece[pos as usize] = Piece::Empty;
	strb wzr, [x17, x9]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 115
		self.boards[player as usize][removed as usize].unset(pos);
	add x6, x1, #80
	mov w7, #56
	umaddl x19, w10, w7, x6
	add x10, x1, w10, uxtw #3
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 31
		*self &= INVERTED_SQUARE_MASKS[index as usize];
	ldr x4, [x4, x9, lsl #3]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 115
		self.boards[player as usize][removed as usize].unset(pos);
	lsl x5, x5, #3
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 31
		*self &= INVERTED_SQUARE_MASKS[index as usize];
	ldr x20, [x19, x5]
	and x20, x20, x4
	str x20, [x19, x5]
	ldr x5, [x10, #192]
	and x5, x5, x4
	str x5, [x10, #192]
	ldr x10, [x1, #72]
	and x10, x10, x4
	str x10, [x1, #72]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 95
		self.boards[player as usize][piece as usize].set(pos);
	and x10, x3, #0xff
	umaddl x16, w16, w7, x6
	lsl x10, x10, #3
	mov w4, #1
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 21
		*self |= 1 << index; // SQUARE_MASKS[index as usize];
	ldr x5, [x16, x10]
	lsl x4, x4, x13
	orr x5, x5, x4
	str x5, [x16, x10]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 96
		self.pos_to_piece[pos as usize] = piece;
	strb w3, [x17, x9]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 21
		*self |= 1 << index; // SQUARE_MASKS[index as usize];
	ldr x9, [x15]
	orr x9, x9, x4
	str x9, [x15]
	ldr x9, [x1, #72]
	orr x9, x9, x4
	str x9, [x1, #72]
	b LBB57_23
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 203
		if !move_item.capturing && !move_item.castling {
	ldrb w9, [x2, #10]
	cbnz w9, LBB57_22
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 204
		self.half_move_clock += 1;
	add w9, w12, #1
	str w9, [x1, #208]
	b LBB57_23
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 174
		match (self.side_to_move, move_item.to_pos) {
	cmp w13, #2
	b.eq LBB57_20
	cmp w13, #6
	b.ne LBB57_22
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 113
		let removed = self.pos_to_piece[pos as usize];
	ldrb w4, [x1, #15]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 115
		self.boards[player as usize][removed as usize].unset(pos);
	add x5, x1, #80
	mov w6, #56
	umaddl x16, w16, w6, x5
	lsl x4, x4, #3
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 31
		*self &= INVERTED_SQUARE_MASKS[index as usize];
	ldr x7, [x16, x4]
	and x7, x7, #0xffffffffffffff7f
	str x7, [x16, x4]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 114
		self.pos_to_piece[pos as usize] = Piece::Empty;
	strb wzr, [x1, #15]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 31
		*self &= INVERTED_SQUARE_MASKS[index as usize];
	and x17, x17, #0xffffffffffffff7f
	str x17, [x15]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 113
		let removed = self.pos_to_piece[pos as usize];
	ldrb w17, [x1, #13]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 115
		self.boards[player as usize][removed as usize].unset(pos);
	umaddl x3, w3, w6, x5
	lsl x17, x17, #3
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 31
		*self &= INVERTED_SQUARE_MASKS[index as usize];
	ldr x4, [x3, x17]
	and x4, x4, #0xffffffffffffffdf
	str x4, [x3, x17]
	ldr x17, [x10]
	and x17, x17, #0xffffffffffffffdf
	str x17, [x10]
	and x9, x9, #0xffffffffffffff7f
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 21
		*self |= 1 << index; // SQUARE_MASKS[index as usize];
	ldr x10, [x16, #32]
	orr x10, x10, #0x20
	str x10, [x16, #32]
	mov w10, #4
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 96
		self.pos_to_piece[pos as usize] = piece;
	strb w10, [x1, #13]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 21
		*self |= 1 << index; // SQUARE_MASKS[index as usize];
	ldr x10, [x15]
	orr x10, x10, #0x20
	str x10, [x15]
	orr x9, x9, #0x20
	b LBB57_21
LBB57_19:
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 113
		let removed = self.pos_to_piece[pos as usize];
	ldrb w4, [x1, #64]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 115
		self.boards[player as usize][removed as usize].unset(pos);
	add x5, x1, #80
	mov w6, #56
	umaddl x16, w16, w6, x5
	lsl x4, x4, #3
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 31
		*self &= INVERTED_SQUARE_MASKS[index as usize];
	ldr x7, [x16, x4]
	and x7, x7, #0xfeffffffffffffff
	str x7, [x16, x4]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 114
		self.pos_to_piece[pos as usize] = Piece::Empty;
	strb wzr, [x1, #64]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 31
		*self &= INVERTED_SQUARE_MASKS[index as usize];
	and x17, x17, #0xfeffffffffffffff
	str x17, [x15]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 113
		let removed = self.pos_to_piece[pos as usize];
	ldrb w17, [x1, #67]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 115
		self.boards[player as usize][removed as usize].unset(pos);
	umaddl x3, w3, w6, x5
	lsl x17, x17, #3
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 31
		*self &= INVERTED_SQUARE_MASKS[index as usize];
	ldr x4, [x3, x17]
	and x4, x4, #0xf7ffffffffffffff
	str x4, [x3, x17]
	ldr x17, [x10]
	and x17, x17, #0xf7ffffffffffffff
	str x17, [x10]
	and x9, x9, #0xfeffffffffffffff
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 21
		*self |= 1 << index; // SQUARE_MASKS[index as usize];
	ldr x10, [x16, #32]
	orr x10, x10, #0x800000000000000
	str x10, [x16, #32]
	mov w10, #4
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 96
		self.pos_to_piece[pos as usize] = piece;
	strb w10, [x1, #67]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 21
		*self |= 1 << index; // SQUARE_MASKS[index as usize];
	ldr x10, [x15]
	orr x10, x10, #0x800000000000000
	str x10, [x15]
	orr x9, x9, #0x800000000000000
	b LBB57_21
LBB57_20:
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 113
		let removed = self.pos_to_piece[pos as usize];
	ldrb w4, [x1, #8]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 115
		self.boards[player as usize][removed as usize].unset(pos);
	add x5, x1, #80
	mov w6, #56
	umaddl x16, w16, w6, x5
	lsl x4, x4, #3
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 31
		*self &= INVERTED_SQUARE_MASKS[index as usize];
	ldr x7, [x16, x4]
	and x7, x7, #0xfffffffffffffffe
	str x7, [x16, x4]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 114
		self.pos_to_piece[pos as usize] = Piece::Empty;
	strb wzr, [x1, #8]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 31
		*self &= INVERTED_SQUARE_MASKS[index as usize];
	and x17, x17, #0xfffffffffffffffe
	str x17, [x15]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 113
		let removed = self.pos_to_piece[pos as usize];
	ldrb w17, [x1, #11]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 115
		self.boards[player as usize][removed as usize].unset(pos);
	umaddl x3, w3, w6, x5
	lsl x17, x17, #3
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 31
		*self &= INVERTED_SQUARE_MASKS[index as usize];
	ldr x4, [x3, x17]
	and x4, x4, #0xfffffffffffffff7
	str x4, [x3, x17]
	ldr x17, [x10]
	and x17, x17, #0xfffffffffffffff7
	str x17, [x10]
	and x9, x9, #0xfffffffffffffffe
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 21
		*self |= 1 << index; // SQUARE_MASKS[index as usize];
	ldr x10, [x16, #32]
	orr x10, x10, #0x8
	str x10, [x16, #32]
	mov w10, #4
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 96
		self.pos_to_piece[pos as usize] = piece;
	strb w10, [x1, #11]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 21
		*self |= 1 << index; // SQUARE_MASKS[index as usize];
	ldr x10, [x15]
	orr x10, x10, #0x8
	str x10, [x15]
	orr x9, x9, #0x8
LBB57_21:
	str x9, [x1, #72]
LBB57_22:
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 206
		self.half_move_clock = 0;
	str wzr, [x1, #208]
LBB57_23:
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 213
		if move_item.double {
	ldrb w9, [x2, #11]
	cbz w9, LBB57_30
	and w9, w8, #0xf8
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/square.rs : 15
		let rank = pos / 8;
	lsr w10, w13, #3
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 217
		if to_rank > from_rank {
	cmp w10, w9, lsr #3
	add w9, w10, #31
	csinc w9, w9, w10, hi
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/square.rs : 20
		rank * 8 + file
	mov x10, x13
	bfi w10, w9, #3, #29
	and w15, w10, #0xff
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 223
		self.enpassant_square = SQUARE_MASKS[Square::index(enpassant_rank, file) as usize];
	sxtb x9, w10
	cmp w15, #64
	b.hs LBB57_59
Lloh119:
	adrp x10, l___unnamed_39@PAGE
Lloh120:
	add x10, x10, l___unnamed_39@PAGEOFF
	ldr x9, [x10, x9, lsl #3]
	ushll.8h v0, v0, #0
	str x9, [x1]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/player.rs : 1
		#[derive(Debug, PartialEq, Clone, Copy)]
	ldrb w9, [x1, #220]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 230
		if self.side_to_move == Player::Black {
	cbz w9, LBB57_31
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 231
		self.full_move_number += 1;
	ldr w10, [x1, #212]
	add w10, w10, #1
	str w10, [x1, #212]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 236
		if move_item.castling || move_item.piece == Piece::King {
	cmp w14, #6
	b.eq LBB57_39
	ldrb w10, [x2, #13]
	cbnz w10, LBB57_39
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 246
		if move_item.piece == Piece::Rook {
	cmp w14, #4
	b.eq LBB57_40
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/pieces.rs : 14
		#[derive(PartialEq, Debug, Clone, Copy)]
	ldrb w8, [x2, #8]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 263
		if move_item.captured_piece == Piece::Rook {
	cmp w8, #4
	b.eq LBB57_46
	b LBB57_55
	mov x9, #0
	ushll.8h v0, v0, #0
	str x9, [x1]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/player.rs : 1
		#[derive(Debug, PartialEq, Clone, Copy)]
	ldrb w9, [x1, #220]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 230
		if self.side_to_move == Player::Black {
	cbnz w9, LBB57_26
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 236
		if move_item.castling || move_item.piece == Piece::King {
	cmp w14, #6
	b.eq LBB57_33
	ldrb w10, [x2, #13]
	cbz w10, LBB57_34
LBB57_33:
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 26
		self.white_queen_side = false;
	strh wzr, [x1, #216]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 246
		if move_item.piece == Piece::Rook {
	cmp w14, #4
	b.ne LBB57_44
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 247
		match (self.side_to_move, move_item.from_pos) {
	cbnz w9, LBB57_40
	cbz w8, LBB57_56
	and w8, w8, #0xff
	cmp w8, #7
	b.ne LBB57_44
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 252
		self.castle_permissions.white_king_side = false;
	strb wzr, [x1, #217]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/pieces.rs : 14
		#[derive(PartialEq, Debug, Clone, Copy)]
	ldrb w8, [x2, #8]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 263
		if move_item.captured_piece == Piece::Rook {
	cmp w8, #4
	b.eq LBB57_45
	b LBB57_55
LBB57_39:
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 30
		self.black_queen_side = false;
	strh wzr, [x1, #218]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 246
		if move_item.piece == Piece::Rook {
	cmp w14, #4
	b.ne LBB57_44
LBB57_40:
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 247
		match (self.side_to_move, move_item.from_pos) {
	cmp w8, #56
	b.eq LBB57_43
	and w8, w8, #0xff
	cmp w8, #63
	b.ne LBB57_44
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 258
		self.castle_permissions.black_king_side = false;
	strb wzr, [x1, #219]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/pieces.rs : 14
		#[derive(PartialEq, Debug, Clone, Copy)]
	ldrb w8, [x2, #8]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 263
		if move_item.captured_piece == Piece::Rook {
	cmp w8, #4
	b.eq LBB57_45
	b LBB57_55
LBB57_43:
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 255
		self.castle_permissions.black_queen_side = false;
	strb wzr, [x1, #218]
LBB57_44:
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/pieces.rs : 14
		#[derive(PartialEq, Debug, Clone, Copy)]
	ldrb w8, [x2, #8]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 263
		if move_item.captured_piece == Piece::Rook {
	cmp w8, #4
	b.ne LBB57_55
LBB57_45:
	cbz w9, LBB57_49
LBB57_46:
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 264
		match (self.side_to_move.opponent(), move_item.to_pos) {
	cbz w13, LBB57_52
	cmp w13, #7
	b.ne LBB57_54
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 269
		self.castle_permissions.white_king_side = false;
	strb wzr, [x1, #217]
	b LBB57_54
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 264
		match (self.side_to_move.opponent(), move_item.to_pos) {
	cmp w13, #56
	b.eq LBB57_53
	cmp w13, #63
	b.ne LBB57_54
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 275
		self.castle_permissions.black_king_side = false;
	strb wzr, [x1, #219]
	b LBB57_54
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 266
		self.castle_permissions.white_queen_side = false;
	strb wzr, [x1, #216]
	b LBB57_54
LBB57_53:
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 272
		self.castle_permissions.black_queen_side = false;
	strb wzr, [x1, #218]
LBB57_54:
	mov w8, #4
LBB57_55:
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 282
		self.side_to_move = self.side_to_move.opponent();
	eor w9, w9, #0x1
	strb w9, [x1, #220]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 284
		UnmakeMoveMetadata {
	xtn.8b v0, v0
	str s0, [x0, #8]
	str x11, [x0]
	str w12, [x0, #12]
	strb w8, [x0, #16]
	.cfi_def_cfa wsp, 48
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 290
		}
	ldp x29, x30, [sp, #32]
	ldp x20, x19, [sp, #16]
	ldp x22, x21, [sp], #48
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	.cfi_restore w19
	.cfi_restore w20
	.cfi_restore w21
	.cfi_restore w22
	ret
	.cfi_restore_state
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 249
		self.castle_permissions.white_queen_side = false;
	strb wzr, [x1, #216]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/pieces.rs : 14
		#[derive(PartialEq, Debug, Clone, Copy)]
	ldrb w8, [x2, #8]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 263
		if move_item.captured_piece == Piece::Rook {
	cmp w8, #4
	b.eq LBB57_45
	b LBB57_55
LBB57_57:
Lloh121:
	adrp x2, l___unnamed_40@PAGE
Lloh122:
	add x2, x2, l___unnamed_40@PAGEOFF
	mov x0, x9
	mov w1, #64
	bl core::panicking::panic_bounds_check
LBB57_58:
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 113
		let removed = self.pos_to_piece[pos as usize];
Lloh123:
	adrp x2, l___unnamed_40@PAGE
Lloh124:
	add x2, x2, l___unnamed_40@PAGEOFF
	mov x0, x8
	mov w1, #64
	bl core::panicking::panic_bounds_check
LBB57_59:
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 223
		self.enpassant_square = SQUARE_MASKS[Square::index(enpassant_rank, file) as usize];
Lloh125:
	adrp x2, l___unnamed_41@PAGE
Lloh126:
	add x2, x2, l___unnamed_41@PAGEOFF
	mov x0, x9
	mov w1, #64
	bl core::panicking::panic_bounds_check
LBB57_60:
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 113
		let removed = self.pos_to_piece[pos as usize];
Lloh127:
	adrp x2, l___unnamed_40@PAGE
Lloh128:
	add x2, x2, l___unnamed_40@PAGEOFF
	mov x0, x10
	mov w1, #64
	bl core::panicking::panic_bounds_check
	.loh AdrpAdd	Lloh117, Lloh118
	.loh AdrpAdd	Lloh119, Lloh120
	.loh AdrpAdd	Lloh121, Lloh122
	.loh AdrpAdd	Lloh123, Lloh124
	.loh AdrpAdd	Lloh125, Lloh126
	.loh AdrpAdd	Lloh127, Lloh128
.section __TEXT,__text,regular,pure_instructions
	.p2align	2
engine::moves::attacked::square_attacked::is_square_attacked:
Lfunc_begin55:
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/attacked/square_attacked.rs : 123
		pub fn is_square_attacked(
	.cfi_startproc
	stp x29, x30, [sp, #-16]!
	.cfi_def_cfa_offset 16
	mov x29, sp
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	.cfi_remember_state
	and w8, w0, #0xff
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/attacked/square_attacked.rs : 20
		let knight_move_mask = cache.knight_moves_masks[pos as usize];
	sxtb x0, w0
	cmp w8, #63
	b.hi LBB55_4
	ldr x10, [x2, #72]
	add x11, x3, x0, lsl #3
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/attacked/square_attacked.rs : 31
		cache.rook_potential_blockers_masks[pos as usize],
	ldr x8, [x11, #5632]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/precalculate/magic_bitboards.rs : 33
		let blockers_on_path = blockers & potential_blockers_mask;
	and x8, x8, x10
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/attacked/square_attacked.rs : 33
		cache.rook_magics[pos as usize],
	ldr x9, [x11, #6272]
		// /rustc/82e1608dfa6e0b5569232559e3d385fea5a93112/library/core/src/num/mod.rs : 1169
		uint_impl! {
	mul x8, x8, x9
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/attacked/square_attacked.rs : 34
		cache.rook_bit_counts[pos as usize],
	add x13, x3, x0
	mov w9, #6208
	ldrb w9, [x13, x9]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/precalculate/magic_bitboards.rs : 35
		let index = (hash >> (64 - bit_count)) as usize;
	neg w9, w9
	lsr x8, x8, x9
	mov w9, #24
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/attacked/square_attacked.rs : 36
		let rook_moves_mask = cache.rook_magic_attack_tables[pos as usize][rook_magic_index];
	smaddl x12, w0, w9, x3
	ldr x9, [x12, #16]
		// /rustc/82e1608dfa6e0b5569232559e3d385fea5a93112/library/core/src/slice/index.rs : 258
		&(*slice)[self]
	cmp x8, x9
	b.hs LBB55_5
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/attacked/square_attacked.rs : 52
		cache.bishop_potential_blockers_masks[pos as usize],
	ldr x9, [x11, #5120]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/precalculate/magic_bitboards.rs : 33
		let blockers_on_path = blockers & potential_blockers_mask;
	and x9, x9, x10
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/attacked/square_attacked.rs : 54
		cache.bishop_magics[pos as usize],
	ldr x10, [x11, #6784]
		// /rustc/82e1608dfa6e0b5569232559e3d385fea5a93112/library/core/src/num/mod.rs : 1169
		uint_impl! {
	mul x9, x9, x10
	mov w10, #6144
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/attacked/square_attacked.rs : 55
		cache.bishop_bit_counts[pos as usize],
	ldrb w10, [x13, x10]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/precalculate/magic_bitboards.rs : 35
		let index = (hash >> (64 - bit_count)) as usize;
	neg w10, w10
	lsr x9, x9, x10
	mov w10, #24
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/attacked/square_attacked.rs : 57
		let bishop_moves_mask = cache.bishop_magic_attack_tables[pos as usize][bishop_magic_index];
	smaddl x13, w0, w10, x3
	ldr x10, [x13, #1552]
		// /rustc/82e1608dfa6e0b5569232559e3d385fea5a93112/library/core/src/slice/index.rs : 258
		&(*slice)[self]
	cmp x9, x10
	b.hs LBB55_6
	ldr x10, [x11, #4096]
	mov w11, w1
	mov w14, #56
	umaddl x14, w1, w14, x2
	ldr x14, [x14, #96]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/attacked/square_attacked.rs : 57
		let bishop_moves_mask = cache.bishop_magic_attack_tables[pos as usize][bishop_magic_index];
	add x13, x13, #1536
	ldr x13, [x13]
	mov w15, #56
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 88
		return &self.boards[player as usize][piece as usize];
	umaddl x11, w11, w15, x2
		// /rustc/82e1608dfa6e0b5569232559e3d385fea5a93112/library/core/src/ops/bit.rs : 178
		bitand_impl! { bool usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 }
	ldr q0, [x11, #112]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/attacked/square_attacked.rs : 36
		let rook_moves_mask = cache.rook_magic_attack_tables[pos as usize][rook_magic_index];
	ldr x12, [x12]
		// /rustc/82e1608dfa6e0b5569232559e3d385fea5a93112/library/core/src/slice/index.rs : 258
		&(*slice)[self]
	add x8, x12, x8, lsl #3
	add x9, x13, x9, lsl #3
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 88
		return &self.boards[player as usize][piece as usize];
	add x12, x11, #104
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/player.rs : 15
		match self {
	eor w13, w1, #0x1
		// /rustc/82e1608dfa6e0b5569232559e3d385fea5a93112/library/core/src/ops/bit.rs : 178
		bitand_impl! { bool usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 }
	and x10, x14, x10
		// /rustc/82e1608dfa6e0b5569232559e3d385fea5a93112/library/core/src/num/mod.rs : 1169
		uint_impl! {
	fmov d1, x10
	cnt.8b v1, v1
	uaddlv.8b h1, v1
	fmov w10, s1
		// /rustc/82e1608dfa6e0b5569232559e3d385fea5a93112/library/core/src/ops/bit.rs : 178
		bitand_impl! { bool usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 }
	ld1r.2d { v1 }, [x12]
	zip2.2d v1, v0, v1
	ld1r.2d { v2 }, [x8]
	ld1r.2d { v3 }, [x9]
	and.16b v1, v1, v3
	and.16b v0, v0, v2
		// /rustc/82e1608dfa6e0b5569232559e3d385fea5a93112/library/core/src/num/mod.rs : 1169
		uint_impl! {
	cnt.16b v1, v1
	uaddlp.8h v1, v1
	uaddlp.4s v1, v1
	uaddlp.2d v1, v1
	cnt.16b v0, v0
	uaddlp.8h v0, v0
	uaddlp.4s v0, v0
	uaddlp.2d v0, v0
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/attacked/square_attacked.rs : 42
		attacked_count += attacking_rooks.count_ones() as i8;
	uzp1.4s v0, v0, v1
	xtn.4h v0, v0
		// /rustc/82e1608dfa6e0b5569232559e3d385fea5a93112/library/core/src/ops/bit.rs : 178
		bitand_impl! { bool usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 }
	ldr x8, [x11, #88]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/attacked/square_attacked.rs : 115
		let king_move_mask = cache.king_moves_masks[pos as usize];
	add x9, x3, x0, lsl #3
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/attacked/square_attacked.rs : 78
		let attacking_mask = cache.pawn_attack_moves_mask[attacker.opponent() as usize][pos as usize];
	add x12, x9, x13, lsl #9
	ldr x12, [x12, #3072]
		// /rustc/82e1608dfa6e0b5569232559e3d385fea5a93112/library/core/src/ops/bit.rs : 178
		bitand_impl! { bool usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 }
	and x8, x12, x8
		// /rustc/82e1608dfa6e0b5569232559e3d385fea5a93112/library/core/src/num/mod.rs : 1169
		uint_impl! {
	fmov d1, x8
	cnt.8b v1, v1
	uaddlv.8b h1, v1
	fmov w8, s1
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/attacked/square_attacked.rs : 115
		let king_move_mask = cache.king_moves_masks[pos as usize];
	ldr x9, [x9, #4608]
		// /rustc/82e1608dfa6e0b5569232559e3d385fea5a93112/library/core/src/ops/bit.rs : 178
		bitand_impl! { bool usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 }
	ldr x11, [x11, #128]
	and x9, x11, x9
		// /rustc/82e1608dfa6e0b5569232559e3d385fea5a93112/library/core/src/num/mod.rs : 1169
		uint_impl! {
	fmov d1, x9
	cnt.8b v1, v1
	uaddlv.8b h1, v1
	fmov w9, s1
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/attacked/square_attacked.rs : 117
		attacked_count += attacking_king.count_ones() as i8;
	addv.4h h0, v0
	fmov w11, s0
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/attacked/square_attacked.rs : 42
		attacked_count += attacking_rooks.count_ones() as i8;
	add w10, w11, w10
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/attacked/square_attacked.rs : 117
		attacked_count += attacking_king.count_ones() as i8;
	add w8, w8, w9
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/attacked/square_attacked.rs : 80
		attacked_count += attacking_pawns.count_ones() as i8;
	add w8, w10, w8
	sxtb w8, w8
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/attacked/square_attacked.rs : 129
		return times_square_attacked(pos, attacker, game, cache) > 0;
	cmp w8, #0
	cset w0, gt
	.cfi_def_cfa wsp, 16
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/attacked/square_attacked.rs : 213
		}
	ldp x29, x30, [sp], #16
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
LBB55_4:
	.cfi_restore_state
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/attacked/square_attacked.rs : 20
		let knight_move_mask = cache.knight_moves_masks[pos as usize];
Lloh109:
	adrp x2, l___unnamed_35@PAGE
Lloh110:
	add x2, x2, l___unnamed_35@PAGEOFF
	mov w1, #64
	bl core::panicking::panic_bounds_check
LBB55_5:
		// /rustc/82e1608dfa6e0b5569232559e3d385fea5a93112/library/core/src/slice/index.rs : 258
		&(*slice)[self]
Lloh111:
	adrp x2, l___unnamed_36@PAGE
Lloh112:
	add x2, x2, l___unnamed_36@PAGEOFF
	mov x0, x8
	mov x1, x9
	bl core::panicking::panic_bounds_check
LBB55_6:
Lloh113:
	adrp x2, l___unnamed_37@PAGE
Lloh114:
	add x2, x2, l___unnamed_37@PAGEOFF
	mov x0, x9
	mov x1, x10
	bl core::panicking::panic_bounds_check
	.loh AdrpAdd	Lloh109, Lloh110
	.loh AdrpAdd	Lloh111, Lloh112
	.loh AdrpAdd	Lloh113, Lloh114
Lfunc_end55:
	.cfi_endproc

	.p2align	2
engine::moves::perft::_perft_unmake:
Lfunc_begin56:
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/perft.rs : 126
		fn _perft_unmake(game: &mut GameState, cache: &PrecalculatedCache, depth: u16) -> u64 {
	.cfi_startproc
	stp x28, x27, [sp, #-96]!
	.cfi_def_cfa_offset 96
	stp x26, x25, [sp, #16]
	stp x24, x23, [sp, #32]
	stp x22, x21, [sp, #48]
	stp x20, x19, [sp, #64]
	stp x29, x30, [sp, #80]
	add x29, sp, #80
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	.cfi_offset w19, -24
	.cfi_offset w20, -32
	.cfi_offset w21, -40
	.cfi_offset w22, -48
	.cfi_offset w23, -56
	.cfi_offset w24, -64
	.cfi_offset w25, -72
	.cfi_offset w26, -80
	.cfi_offset w27, -88
	.cfi_offset w28, -96
	.cfi_remember_state
	sub sp, sp, #1, lsl #12
	sub sp, sp, #48
	tst w2, #0xffff
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/perft.rs : 129
		if depth == 0 {
	b.eq LBB56_7
	mov x22, x2
	mov x19, x1
	mov x20, x0
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/movelist.rs : 12
		MoveList {
	str xzr, [sp, #4104]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/perft.rs : 137
		game.side_to_move,
	ldrb w2, [x0, #220]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/perft.rs : 134
		super::pseudolegal::all::generate_pseudolegal_moves(
	add x0, sp, #8
	mov x1, x20
	mov x3, x19
	bl engine::moves::pseudolegal::all::generate_pseudolegal_moves
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/movelist.rs : 27
		self.end
	ldr x25, [sp, #4104]
		// /rustc/82e1608dfa6e0b5569232559e3d385fea5a93112/library/core/src/iter/range.rs : 729
		if self.start < self.end {
	cbz x25, LBB56_8
	mov x21, #0
	mov x26, #0
	sub w22, w22, #1
	add x23, sp, #8
	mov w27, #56
	b LBB56_4
LBB56_3:
	add x26, x26, #1
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/perft.rs : 150
		game.unmake_move(&move_item, unmake_metadata);
	sub x2, x29, #112
	mov x0, x20
	mov x1, x23
	bl engine::state::game::GameState::unmake_move
		// /rustc/82e1608dfa6e0b5569232559e3d385fea5a93112/library/core/src/cmp.rs : 1567
		ord_impl! { char usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 }
	add x23, x23, #16
		// /rustc/82e1608dfa6e0b5569232559e3d385fea5a93112/library/core/src/iter/range.rs : 729
		if self.start < self.end {
	cmp x25, x26
	b.eq LBB56_9
LBB56_4:
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/perft.rs : 141
		let move_item = &move_list.moves[index];
	cmp x26, #256
	b.eq LBB56_10
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/perft.rs : 142
		let player = game.side_to_move;
	ldrb w28, [x20, #220]
	cmp x28, #0
	cset w24, eq
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/perft.rs : 143
		let unmake_metadata = game.make_move(move_item);
	sub x0, x29, #112
	mov x1, x20
	mov x2, x23
	bl engine::state::game::GameState::make_move
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 88
		return &self.boards[player as usize][piece as usize];
	umaddl x8, w28, w27, x20
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/attacked/in_check.rs : 10
		let king = game
	ldr x8, [x8, #128]
		// /rustc/82e1608dfa6e0b5569232559e3d385fea5a93112/library/core/src/num/mod.rs : 1169
		uint_impl! {
	rbit x8, x8
	clz x0, x8
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/attacked/in_check.rs : 15
		return is_square_attacked(
	mov x1, x24
	mov x2, x20
	mov x3, x19
	bl engine::moves::attacked::square_attacked::is_square_attacked
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/perft.rs : 145
		if !is_in_check(player, game, cache) {
	tbnz w0, #0, LBB56_3
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/perft.rs : 146
		let move_nodes = _perft_unmake(game, cache, depth - 1);
	mov x0, x20
	mov x1, x19
	mov x2, x22
	bl engine::moves::perft::_perft_unmake
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/perft.rs : 147
		nodes += move_nodes;
	add x21, x0, x21
	b LBB56_3
LBB56_7:
	mov w21, #1
	b LBB56_9
	mov x21, #0
LBB56_9:
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/perft.rs : 154
		}
	mov x0, x21
	add sp, sp, #1, lsl #12
	add sp, sp, #48
	.cfi_def_cfa wsp, 96
	ldp x29, x30, [sp, #80]
	ldp x20, x19, [sp, #64]
	ldp x22, x21, [sp, #48]
	ldp x24, x23, [sp, #32]
	ldp x26, x25, [sp, #16]
	ldp x28, x27, [sp], #96
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	.cfi_restore w19
	.cfi_restore w20
	.cfi_restore w21
	.cfi_restore w22
	.cfi_restore w23
	.cfi_restore w24
	.cfi_restore w25
	.cfi_restore w26
	.cfi_restore w27
	.cfi_restore w28
	ret
LBB56_10:
	.cfi_restore_state
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/perft.rs : 141
		let move_item = &move_list.moves[index];
Lloh115:
	adrp x2, l___unnamed_38@PAGE
Lloh116:
	add x2, x2, l___unnamed_38@PAGEOFF
	mov w0, #256
	mov w1, #256
	bl core::panicking::panic_bounds_check
	.loh AdrpAdd	Lloh115, Lloh116
Lfunc_end56:
	.cfi_endproc

	.p2align	2
engine::state::game::GameState::make_move:
Lfunc_begin57:
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 127
		pub fn make_move(&mut self, move_item: &MoveItem) -> UnmakeMoveMetadata {
	.cfi_startproc
	stp x22, x21, [sp, #-48]!
	.cfi_def_cfa_offset 48
	stp x20, x19, [sp, #16]
	stp x29, x30, [sp, #32]
	add x29, sp, #32
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	.cfi_offset w19, -24
	.cfi_offset w20, -32
	.cfi_offset w21, -40
	.cfi_offset w22, -48
	.cfi_remember_state
		// /rustc/82e1608dfa6e0b5569232559e3d385fea5a93112/library/core/src/clone.rs : 229
		impl_clone! {
	ldr s0, [x1, #216]
	ldr x10, [x1]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 130
		let prev_half_move_clock = self.half_move_clock;
	ldr w11, [x1, #208]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 134
		self.enpassant_square = 0;
	str xzr, [x1]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 139
		.remove_piece(self.side_to_move, move_item.from_pos);
	ldrsb x8, [x2, #4]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 113
		let removed = self.pos_to_piece[pos as usize];
	cmp x8, #64
	b.hs LBB57_57
	add x16, x1, #8
	ldrb w12, [x1, #220]
	ldrb w9, [x16, x8]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 114
		self.pos_to_piece[pos as usize] = Piece::Empty;
	strb wzr, [x16, x8]
	mov w13, #56
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 115
		self.boards[player as usize][removed as usize].unset(pos);
	umaddl x13, w12, w13, x16
	add x9, x13, x9, lsl #3
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 31
		*self &= INVERTED_SQUARE_MASKS[index as usize];
Lloh117:
	adrp x17, l___unnamed_25@PAGE
Lloh118:
	add x17, x17, l___unnamed_25@PAGEOFF
	ldr x13, [x17, x8, lsl #3]
	ldr x14, [x9, #72]
	and x14, x14, x13
	str x14, [x9, #72]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 116
		self.pos_to_player[player as usize].unset(pos);
	add x15, x16, x12, lsl #3
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 31
		*self &= INVERTED_SQUARE_MASKS[index as usize];
	ldr x9, [x15, #184]!
	and x9, x9, x13
	str x9, [x15]
	ldr x9, [x1, #72]
	and x9, x9, x13
	str x9, [x1, #72]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/pieces.rs : 14
		#[derive(PartialEq, Debug, Clone, Copy)]
	ldrb w14, [x2, #6]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 141
		if move_item.piece == Piece::Pawn {
	cmp w14, #1
	b.ne LBB57_4
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 142
		let final_piece = if move_item.promoting {
	ldrb w9, [x2, #9]
	ldrb w13, [x2, #7]
	cmp w9, #0
	csinc w3, w13, wzr, ne
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 149
		if move_item.enpassant {
	ldrb w9, [x2, #12]
	cbz w9, LBB57_10
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/square.rs : 57
		8 * self.rank + self.file
	and x9, x8, #0xfffffffffffffff8
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 151
		let to = Square::from(move_item.to_pos);
	ldrb w13, [x2, #5]
	sxtb w4, w13
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/square.rs : 49
		file: pos % 8,
	ubfx w4, w4, #12, #3
	add w4, w13, w4
	and w4, w4, #0xf8
	sub w4, w13, w4
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/square.rs : 57
		8 * self.rank + self.file
	add x9, x9, w4, sxtb
	and w4, w9, #0xff
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 113
		let removed = self.pos_to_piece[pos as usize];
	cmp w4, #64
	b.lo LBB57_12
	b LBB57_56
LBB57_4:
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 183
		.remove_piece(self.side_to_move.opponent(), move_item.to_pos);
	ldrsb x9, [x2, #5]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 113
		let removed = self.pos_to_piece[pos as usize];
	cmp x9, #64
	b.hs LBB57_56
	and w13, w9, #0xff
	and x4, x9, #0xff
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 139
		.remove_piece(self.side_to_move, move_item.from_pos);
	cmp w12, #0
	cset w3, eq
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 113
		let removed = self.pos_to_piece[pos as usize];
	ldrb w5, [x16, x9]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 114
		self.pos_to_piece[pos as usize] = Piece::Empty;
	strb wzr, [x16, x9]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 115
		self.boards[player as usize][removed as usize].unset(pos);
	add x6, x1, #80
	mov w7, #56
	umaddl x19, w3, w7, x6
	lsl x5, x5, #3
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 31
		*self &= INVERTED_SQUARE_MASKS[index as usize];
	ldr x20, [x17, x9, lsl #3]
	ldr x21, [x19, x5]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 115
		self.boards[player as usize][removed as usize].unset(pos);
	add x17, x1, w3, uxtw #3
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 31
		*self &= INVERTED_SQUARE_MASKS[index as usize];
	and x21, x21, x20
	str x21, [x19, x5]
	ldr x5, [x17, #192]!
	and x5, x5, x20
	ldr x21, [x1, #72]
	and x21, x21, x20
	str x21, [x1, #72]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 113
		let removed = self.pos_to_piece[pos as usize];
	ldrb w21, [x16, x9]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 114
		self.pos_to_piece[pos as usize] = Piece::Empty;
	strb wzr, [x16, x9]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 115
		self.boards[player as usize][removed as usize].unset(pos);
	lsl x21, x21, #3
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 31
		*self &= INVERTED_SQUARE_MASKS[index as usize];
	ldr x22, [x19, x21]
	and x22, x22, x20
	str x22, [x19, x21]
	str x5, [x17]
	ldr x5, [x1, #72]
	and x5, x5, x20
	str x5, [x1, #72]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 95
		self.boards[player as usize][piece as usize].set(pos);
	umaddl x5, w12, w7, x6
	lsl x6, x14, #3
	mov w7, #1
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 21
		*self |= 1 << index; // SQUARE_MASKS[index as usize];
	lsl x4, x7, x4
	ldr x7, [x5, x6]
	orr x7, x7, x4
	str x7, [x5, x6]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 96
		self.pos_to_piece[pos as usize] = piece;
	strb w14, [x16, x9]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 21
		*self |= 1 << index; // SQUARE_MASKS[index as usize];
	ldr x9, [x15]
	orr x16, x9, x4
	str x16, [x15]
	ldr x9, [x1, #72]
	orr x9, x9, x4
	str x9, [x1, #72]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 188
		if move_item.castling {
	ldrb w4, [x2, #13]
	cbz w4, LBB57_22
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 190
		match (self.side_to_move, move_item.to_pos) {
	cbz w12, LBB57_24
	cmp w13, #58
	b.eq LBB57_27
	cmp w13, #62
	b.ne LBB57_30
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 113
		let removed = self.pos_to_piece[pos as usize];
	ldrb w4, [x1, #71]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 115
		self.boards[player as usize][removed as usize].unset(pos);
	add x5, x1, #80
	mov w6, #56
	umaddl x7, w12, w6, x5
	lsl x4, x4, #3
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 31
		*self &= INVERTED_SQUARE_MASKS[index as usize];
	ldr x19, [x7, x4]
	and x19, x19, #0x7fffffffffffffff
	str x19, [x7, x4]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 114
		self.pos_to_piece[pos as usize] = Piece::Empty;
	strb wzr, [x1, #71]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 31
		*self &= INVERTED_SQUARE_MASKS[index as usize];
	and x16, x16, #0x7fffffffffffffff
	str x16, [x15]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 113
		let removed = self.pos_to_piece[pos as usize];
	ldrb w16, [x1, #69]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 115
		self.boards[player as usize][removed as usize].unset(pos);
	umaddl x3, w3, w6, x5
	lsl x16, x16, #3
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 31
		*self &= INVERTED_SQUARE_MASKS[index as usize];
	ldr x4, [x3, x16]
	and x4, x4, #0xdfffffffffffffff
	str x4, [x3, x16]
	ldr x16, [x17]
	and x16, x16, #0xdfffffffffffffff
	str x16, [x17]
	and x9, x9, #0x7fffffffffffffff
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 21
		*self |= 1 << index; // SQUARE_MASKS[index as usize];
	ldr x16, [x7, #32]
	orr x16, x16, #0x2000000000000000
	str x16, [x7, #32]
	mov w16, #4
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 96
		self.pos_to_piece[pos as usize] = piece;
	strb w16, [x1, #69]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 21
		*self |= 1 << index; // SQUARE_MASKS[index as usize];
	ldr x16, [x15]
	orr x16, x16, #0x2000000000000000
	str x16, [x15]
	orr x9, x9, #0x2000000000000000
	b LBB57_29
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 162
		.remove_piece(self.side_to_move.opponent(), move_item.to_pos);
	ldrsb x9, [x2, #5]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 113
		let removed = self.pos_to_piece[pos as usize];
	cmp x9, #64
	b.hs LBB57_56
	and w13, w9, #0xff
LBB57_12:
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 139
		.remove_piece(self.side_to_move, move_item.from_pos);
	cmp w12, #0
	cset w4, eq
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 113
		let removed = self.pos_to_piece[pos as usize];
	ldrb w5, [x16, x9]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 114
		self.pos_to_piece[pos as usize] = Piece::Empty;
	strb wzr, [x16, x9]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 115
		self.boards[player as usize][removed as usize].unset(pos);
	add x6, x1, #8
	mov w7, #56
	umaddl x7, w4, w7, x6
	add x5, x7, x5, lsl #3
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 31
		*self &= INVERTED_SQUARE_MASKS[index as usize];
	ldr x9, [x17, x9, lsl #3]
	ldr x7, [x5, #72]
	and x7, x7, x9
	str x7, [x5, #72]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 116
		self.pos_to_player[player as usize].unset(pos);
	add x5, x6, w4, uxtw #3
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 31
		*self &= INVERTED_SQUARE_MASKS[index as usize];
	ldr x4, [x5, #184]
	and x4, x4, x9
	str x4, [x5, #184]
	ldr x5, [x1, #72]
	and x9, x5, x9
	str x9, [x1, #72]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 166
		if move_item.double {
	ldrb w9, [x2, #11]
	cbz w9, LBB57_15
	and w9, w8, #0xf8
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/square.rs : 15
		let rank = pos / 8;
	sxtb w5, w13
	ubfx w5, w5, #12, #3
	add w5, w13, w5
	sbfx w6, w5, #3, #5
	and w5, w5, #0x1f8
	sub w5, w13, w5
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 170
		if to_rank > from_rank {
	cmp w6, w9, lsr #3
	add w9, w6, #31
	csinc w9, w9, w6, gt
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/square.rs : 20
		rank * 8 + file
	add w9, w5, w9, lsl #3
	and w5, w9, #0xff
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 176
		self.enpassant_square = SQUARE_MASKS[Square::index(enpassant_rank, file) as usize];
	sxtb x9, w9
	cmp w5, #63
	b.hi LBB57_58
Lloh119:
	adrp x5, l___unnamed_39@PAGE
Lloh120:
	add x5, x5, l___unnamed_39@PAGEOFF
	ldr x9, [x5, x9, lsl #3]
	str x9, [x1]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 113
		let removed = self.pos_to_piece[pos as usize];
	mov x9, x13
	sxtb x9, w13
	cmp w13, #64
	b.hs LBB57_56
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 139
		.remove_piece(self.side_to_move, move_item.from_pos);
	cmp w12, #0
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 113
		let removed = self.pos_to_piece[pos as usize];
	ldrb w5, [x16, x9]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 114
		self.pos_to_piece[pos as usize] = Piece::Empty;
	strb wzr, [x16, x9]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 139
		.remove_piece(self.side_to_move, move_item.from_pos);
	cset w6, eq
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 115
		self.boards[player as usize][removed as usize].unset(pos);
	add x7, x1, #80
	mov w19, #56
	umaddl x20, w6, w19, x7
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 31
		*self &= INVERTED_SQUARE_MASKS[index as usize];
	ldr x17, [x17, x9, lsl #3]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 115
		self.boards[player as usize][removed as usize].unset(pos);
	lsl x5, x5, #3
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 31
		*self &= INVERTED_SQUARE_MASKS[index as usize];
	ldr x21, [x20, x5]
	and x21, x21, x17
	str x21, [x20, x5]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 115
		self.boards[player as usize][removed as usize].unset(pos);
	add x5, x1, w6, uxtw #3
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 31
		*self &= INVERTED_SQUARE_MASKS[index as usize];
	and x4, x4, x17
	str x4, [x5, #192]
	ldr x4, [x1, #72]
	and x17, x4, x17
	str x17, [x1, #72]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 95
		self.boards[player as usize][piece as usize].set(pos);
	and x17, x3, #0xff
	umaddl x4, w12, w19, x7
	lsl x17, x17, #3
	mov w5, #1
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 21
		*self |= 1 << index; // SQUARE_MASKS[index as usize];
	ldr x6, [x4, x17]
	lsl x5, x5, x13
	orr x6, x6, x5
	str x6, [x4, x17]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 96
		self.pos_to_piece[pos as usize] = piece;
	strb w3, [x16, x9]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 21
		*self |= 1 << index; // SQUARE_MASKS[index as usize];
	ldr x9, [x15]
	orr x9, x9, x5
	str x9, [x15]
	ldr x9, [x1, #72]
	orr x9, x9, x5
	str x9, [x1, #72]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 229
		if self.side_to_move == Player::Black {
	cbz w12, LBB57_31
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 230
		self.full_move_number += 1;
	ldr w9, [x1, #212]
	add w9, w9, #1
	str w9, [x1, #212]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 235
		if move_item.castling || move_item.piece == Piece::King {
	cmp w14, #6
	b.eq LBB57_21
	ldrb w9, [x2, #13]
	cbnz w9, LBB57_21
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 245
		if move_item.piece == Piece::Rook {
	cmp w14, #4
	b.eq LBB57_36
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/pieces.rs : 14
		#[derive(PartialEq, Debug, Clone, Copy)]
	ldrb w8, [x2, #8]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 262
		if move_item.captured_piece == Piece::Rook {
	cmp w8, #4
	b.eq LBB57_42
	b LBB57_54
LBB57_21:
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 30
		self.black_queen_side = false;
	strh wzr, [x1, #218]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 245
		if move_item.piece == Piece::Rook {
	cmp w14, #4
	b.eq LBB57_36
	b LBB57_40
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 219
		if !move_item.capturing && !move_item.castling {
	ldrb w9, [x2, #10]
	cbnz w9, LBB57_30
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 220
		self.half_move_clock += 1;
	add w9, w11, #1
	str w9, [x1, #208]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 229
		if self.side_to_move == Player::Black {
	cbnz w12, LBB57_17
	b LBB57_31
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 190
		match (self.side_to_move, move_item.to_pos) {
	cmp w13, #2
	b.eq LBB57_28
	cmp w13, #6
	b.ne LBB57_30
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 113
		let removed = self.pos_to_piece[pos as usize];
	ldrb w4, [x1, #15]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 115
		self.boards[player as usize][removed as usize].unset(pos);
	add x5, x1, #80
	mov w6, #56
	umaddl x7, w12, w6, x5
	lsl x4, x4, #3
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 31
		*self &= INVERTED_SQUARE_MASKS[index as usize];
	ldr x19, [x7, x4]
	and x19, x19, #0xffffffffffffff7f
	str x19, [x7, x4]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 114
		self.pos_to_piece[pos as usize] = Piece::Empty;
	strb wzr, [x1, #15]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 31
		*self &= INVERTED_SQUARE_MASKS[index as usize];
	and x16, x16, #0xffffffffffffff7f
	str x16, [x15]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 113
		let removed = self.pos_to_piece[pos as usize];
	ldrb w16, [x1, #13]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 115
		self.boards[player as usize][removed as usize].unset(pos);
	umaddl x3, w3, w6, x5
	lsl x16, x16, #3
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 31
		*self &= INVERTED_SQUARE_MASKS[index as usize];
	ldr x4, [x3, x16]
	and x4, x4, #0xffffffffffffffdf
	str x4, [x3, x16]
	ldr x16, [x17]
	and x16, x16, #0xffffffffffffffdf
	str x16, [x17]
	and x9, x9, #0xffffffffffffff7f
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 21
		*self |= 1 << index; // SQUARE_MASKS[index as usize];
	ldr x16, [x7, #32]
	orr x16, x16, #0x20
	str x16, [x7, #32]
	mov w16, #4
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 96
		self.pos_to_piece[pos as usize] = piece;
	strb w16, [x1, #13]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 21
		*self |= 1 << index; // SQUARE_MASKS[index as usize];
	ldr x16, [x15]
	orr x16, x16, #0x20
	str x16, [x15]
	orr x9, x9, #0x20
	b LBB57_29
LBB57_27:
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 113
		let removed = self.pos_to_piece[pos as usize];
	ldrb w4, [x1, #64]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 115
		self.boards[player as usize][removed as usize].unset(pos);
	add x5, x1, #80
	mov w6, #56
	umaddl x7, w12, w6, x5
	lsl x4, x4, #3
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 31
		*self &= INVERTED_SQUARE_MASKS[index as usize];
	ldr x19, [x7, x4]
	and x19, x19, #0xfeffffffffffffff
	str x19, [x7, x4]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 114
		self.pos_to_piece[pos as usize] = Piece::Empty;
	strb wzr, [x1, #64]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 31
		*self &= INVERTED_SQUARE_MASKS[index as usize];
	and x16, x16, #0xfeffffffffffffff
	str x16, [x15]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 113
		let removed = self.pos_to_piece[pos as usize];
	ldrb w16, [x1, #67]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 115
		self.boards[player as usize][removed as usize].unset(pos);
	umaddl x3, w3, w6, x5
	lsl x16, x16, #3
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 31
		*self &= INVERTED_SQUARE_MASKS[index as usize];
	ldr x4, [x3, x16]
	and x4, x4, #0xf7ffffffffffffff
	str x4, [x3, x16]
	ldr x16, [x17]
	and x16, x16, #0xf7ffffffffffffff
	str x16, [x17]
	and x9, x9, #0xfeffffffffffffff
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 21
		*self |= 1 << index; // SQUARE_MASKS[index as usize];
	ldr x16, [x7, #32]
	orr x16, x16, #0x800000000000000
	str x16, [x7, #32]
	mov w16, #4
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 96
		self.pos_to_piece[pos as usize] = piece;
	strb w16, [x1, #67]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 21
		*self |= 1 << index; // SQUARE_MASKS[index as usize];
	ldr x16, [x15]
	orr x16, x16, #0x800000000000000
	str x16, [x15]
	orr x9, x9, #0x800000000000000
	b LBB57_29
LBB57_28:
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 113
		let removed = self.pos_to_piece[pos as usize];
	ldrb w4, [x1, #8]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 115
		self.boards[player as usize][removed as usize].unset(pos);
	add x5, x1, #80
	mov w6, #56
	umaddl x7, w12, w6, x5
	lsl x4, x4, #3
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 31
		*self &= INVERTED_SQUARE_MASKS[index as usize];
	ldr x19, [x7, x4]
	and x19, x19, #0xfffffffffffffffe
	str x19, [x7, x4]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 114
		self.pos_to_piece[pos as usize] = Piece::Empty;
	strb wzr, [x1, #8]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 31
		*self &= INVERTED_SQUARE_MASKS[index as usize];
	and x16, x16, #0xfffffffffffffffe
	str x16, [x15]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 113
		let removed = self.pos_to_piece[pos as usize];
	ldrb w16, [x1, #11]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 115
		self.boards[player as usize][removed as usize].unset(pos);
	umaddl x3, w3, w6, x5
	lsl x16, x16, #3
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 31
		*self &= INVERTED_SQUARE_MASKS[index as usize];
	ldr x4, [x3, x16]
	and x4, x4, #0xfffffffffffffff7
	str x4, [x3, x16]
	ldr x16, [x17]
	and x16, x16, #0xfffffffffffffff7
	str x16, [x17]
	and x9, x9, #0xfffffffffffffffe
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 21
		*self |= 1 << index; // SQUARE_MASKS[index as usize];
	ldr x16, [x7, #32]
	orr x16, x16, #0x8
	str x16, [x7, #32]
	mov w16, #4
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 96
		self.pos_to_piece[pos as usize] = piece;
	strb w16, [x1, #11]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 21
		*self |= 1 << index; // SQUARE_MASKS[index as usize];
	ldr x16, [x15]
	orr x16, x16, #0x8
	str x16, [x15]
	orr x9, x9, #0x8
LBB57_29:
	str x9, [x1, #72]
LBB57_30:
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 222
		self.half_move_clock = 0;
	str wzr, [x1, #208]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 229
		if self.side_to_move == Player::Black {
	cbnz w12, LBB57_17
LBB57_31:
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 235
		if move_item.castling || move_item.piece == Piece::King {
	cmp w14, #6
	b.eq LBB57_33
	ldrb w9, [x2, #13]
	cbz w9, LBB57_34
LBB57_33:
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 26
		self.white_queen_side = false;
	strh wzr, [x1, #216]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 245
		if move_item.piece == Piece::Rook {
	cmp w14, #4
	b.ne LBB57_40
	cbz w12, LBB57_49
LBB57_36:
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 246
		match (self.side_to_move, move_item.from_pos) {
	cmp w8, #56
	b.eq LBB57_39
	and w8, w8, #0xff
	cmp w8, #63
	b.ne LBB57_40
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 257
		self.castle_permissions.black_king_side = false;
	strb wzr, [x1, #219]
	b LBB57_40
LBB57_39:
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 254
		self.castle_permissions.black_queen_side = false;
	strb wzr, [x1, #218]
LBB57_40:
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/pieces.rs : 14
		#[derive(PartialEq, Debug, Clone, Copy)]
	ldrb w8, [x2, #8]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 262
		if move_item.captured_piece == Piece::Rook {
	cmp w8, #4
	b.ne LBB57_54
	cbz w12, LBB57_45
LBB57_42:
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 263
		match (self.side_to_move.opponent(), move_item.to_pos) {
	cbz w13, LBB57_48
	cmp w13, #7
	b.ne LBB57_53
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 268
		self.castle_permissions.white_king_side = false;
	strb wzr, [x1, #217]
	b LBB57_53
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 263
		match (self.side_to_move.opponent(), move_item.to_pos) {
	cmp w13, #56
	b.eq LBB57_52
	cmp w13, #63
	b.ne LBB57_53
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 274
		self.castle_permissions.black_king_side = false;
	strb wzr, [x1, #219]
	b LBB57_53
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 265
		self.castle_permissions.white_queen_side = false;
	strb wzr, [x1, #216]
	b LBB57_53
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 246
		match (self.side_to_move, move_item.from_pos) {
	cbz w8, LBB57_55
	and w8, w8, #0xff
	cmp w8, #7
	b.ne LBB57_40
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 251
		self.castle_permissions.white_king_side = false;
	strb wzr, [x1, #217]
	b LBB57_40
LBB57_52:
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 271
		self.castle_permissions.black_queen_side = false;
	strb wzr, [x1, #218]
LBB57_53:
	mov w8, #4
LBB57_54:
	ushll.8h v0, v0, #0
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 281
		self.side_to_move = self.side_to_move.opponent();
	eor w9, w12, #0x1
	strb w9, [x1, #220]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 283
		UnmakeMoveMetadata {
	xtn.8b v0, v0
	str s0, [x0, #8]
	str x10, [x0]
	str w11, [x0, #12]
	strb w8, [x0, #16]
	.cfi_def_cfa wsp, 48
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 289
		}
	ldp x29, x30, [sp, #32]
	ldp x20, x19, [sp, #16]
	ldp x22, x21, [sp], #48
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	.cfi_restore w19
	.cfi_restore w20
	.cfi_restore w21
	.cfi_restore w22
	ret
	.cfi_restore_state
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 248
		self.castle_permissions.white_queen_side = false;
	strb wzr, [x1, #216]
	b LBB57_40
LBB57_56:
Lloh121:
	adrp x2, l___unnamed_40@PAGE
Lloh122:
	add x2, x2, l___unnamed_40@PAGEOFF
	mov x0, x9
	mov w1, #64
	bl core::panicking::panic_bounds_check
LBB57_57:
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 113
		let removed = self.pos_to_piece[pos as usize];
Lloh123:
	adrp x2, l___unnamed_40@PAGE
Lloh124:
	add x2, x2, l___unnamed_40@PAGEOFF
	mov x0, x8
	mov w1, #64
	bl core::panicking::panic_bounds_check
LBB57_58:
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 176
		self.enpassant_square = SQUARE_MASKS[Square::index(enpassant_rank, file) as usize];
Lloh125:
	adrp x2, l___unnamed_41@PAGE
Lloh126:
	add x2, x2, l___unnamed_41@PAGEOFF
	mov x0, x9
	mov w1, #64
	bl core::panicking::panic_bounds_check
	.loh AdrpAdd	Lloh117, Lloh118
	.loh AdrpAdd	Lloh119, Lloh120
	.loh AdrpAdd	Lloh121, Lloh122
	.loh AdrpAdd	Lloh123, Lloh124
	.loh AdrpAdd	Lloh125, Lloh126
.section __TEXT,__text,regular,pure_instructions
	.p2align	2
engine::moves::attacked::square_attacked::is_square_attacked:
Lfunc_begin55:
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/attacked/square_attacked.rs : 123
		pub fn is_square_attacked(
	.cfi_startproc
	stp x29, x30, [sp, #-16]!
	.cfi_def_cfa_offset 16
	mov x29, sp
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	.cfi_remember_state
	and w8, w0, #0xff
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/attacked/square_attacked.rs : 20
		let knight_move_mask = cache.knight_moves_masks[pos as usize];
	sxtb x0, w0
	cmp w8, #63
	b.hi LBB55_4
	ldr x10, [x2, #72]
	add x11, x3, x0, lsl #3
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/attacked/square_attacked.rs : 31
		cache.rook_potential_blockers_masks[pos as usize],
	ldr x8, [x11, #5632]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/precalculate/magic_bitboards.rs : 33
		let blockers_on_path = blockers & potential_blockers_mask;
	and x8, x8, x10
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/attacked/square_attacked.rs : 33
		cache.rook_magics[pos as usize],
	ldr x9, [x11, #6272]
		// /rustc/82e1608dfa6e0b5569232559e3d385fea5a93112/library/core/src/num/mod.rs : 1169
		uint_impl! {
	mul x8, x8, x9
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/attacked/square_attacked.rs : 34
		cache.rook_bit_counts[pos as usize],
	add x13, x3, x0
	mov w9, #6208
	ldrb w9, [x13, x9]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/precalculate/magic_bitboards.rs : 35
		let index = (hash >> (64 - bit_count)) as usize;
	neg w9, w9
	lsr x8, x8, x9
	mov w9, #24
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/attacked/square_attacked.rs : 36
		let rook_moves_mask = cache.rook_magic_attack_tables[pos as usize][rook_magic_index];
	smaddl x12, w0, w9, x3
	ldr x9, [x12, #16]
		// /rustc/82e1608dfa6e0b5569232559e3d385fea5a93112/library/core/src/slice/index.rs : 258
		&(*slice)[self]
	cmp x8, x9
	b.hs LBB55_5
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/attacked/square_attacked.rs : 52
		cache.bishop_potential_blockers_masks[pos as usize],
	ldr x9, [x11, #5120]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/precalculate/magic_bitboards.rs : 33
		let blockers_on_path = blockers & potential_blockers_mask;
	and x9, x9, x10
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/attacked/square_attacked.rs : 54
		cache.bishop_magics[pos as usize],
	ldr x10, [x11, #6784]
		// /rustc/82e1608dfa6e0b5569232559e3d385fea5a93112/library/core/src/num/mod.rs : 1169
		uint_impl! {
	mul x9, x9, x10
	mov w10, #6144
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/attacked/square_attacked.rs : 55
		cache.bishop_bit_counts[pos as usize],
	ldrb w10, [x13, x10]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/precalculate/magic_bitboards.rs : 35
		let index = (hash >> (64 - bit_count)) as usize;
	neg w10, w10
	lsr x9, x9, x10
	mov w10, #24
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/attacked/square_attacked.rs : 57
		let bishop_moves_mask = cache.bishop_magic_attack_tables[pos as usize][bishop_magic_index];
	smaddl x13, w0, w10, x3
	ldr x10, [x13, #1552]
		// /rustc/82e1608dfa6e0b5569232559e3d385fea5a93112/library/core/src/slice/index.rs : 258
		&(*slice)[self]
	cmp x9, x10
	b.hs LBB55_6
	ldr x10, [x11, #4096]
	mov w11, w1
	mov w14, #56
	umaddl x14, w1, w14, x2
	ldr x14, [x14, #96]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/attacked/square_attacked.rs : 57
		let bishop_moves_mask = cache.bishop_magic_attack_tables[pos as usize][bishop_magic_index];
	add x13, x13, #1536
	ldr x13, [x13]
	mov w15, #56
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 88
		return &self.boards[player as usize][piece as usize];
	umaddl x11, w11, w15, x2
		// /rustc/82e1608dfa6e0b5569232559e3d385fea5a93112/library/core/src/ops/bit.rs : 178
		bitand_impl! { bool usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 }
	ldr q0, [x11, #112]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/attacked/square_attacked.rs : 36
		let rook_moves_mask = cache.rook_magic_attack_tables[pos as usize][rook_magic_index];
	ldr x12, [x12]
		// /rustc/82e1608dfa6e0b5569232559e3d385fea5a93112/library/core/src/slice/index.rs : 258
		&(*slice)[self]
	add x8, x12, x8, lsl #3
	add x9, x13, x9, lsl #3
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 88
		return &self.boards[player as usize][piece as usize];
	add x12, x11, #104
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/player.rs : 15
		match self {
	eor w13, w1, #0x1
		// /rustc/82e1608dfa6e0b5569232559e3d385fea5a93112/library/core/src/ops/bit.rs : 178
		bitand_impl! { bool usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 }
	and x10, x14, x10
		// /rustc/82e1608dfa6e0b5569232559e3d385fea5a93112/library/core/src/num/mod.rs : 1169
		uint_impl! {
	fmov d1, x10
	cnt.8b v1, v1
	uaddlv.8b h1, v1
	fmov w10, s1
		// /rustc/82e1608dfa6e0b5569232559e3d385fea5a93112/library/core/src/ops/bit.rs : 178
		bitand_impl! { bool usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 }
	ld1r.2d { v1 }, [x12]
	zip2.2d v1, v0, v1
	ld1r.2d { v2 }, [x8]
	ld1r.2d { v3 }, [x9]
	and.16b v1, v1, v3
	and.16b v0, v0, v2
		// /rustc/82e1608dfa6e0b5569232559e3d385fea5a93112/library/core/src/num/mod.rs : 1169
		uint_impl! {
	cnt.16b v1, v1
	uaddlp.8h v1, v1
	uaddlp.4s v1, v1
	uaddlp.2d v1, v1
	cnt.16b v0, v0
	uaddlp.8h v0, v0
	uaddlp.4s v0, v0
	uaddlp.2d v0, v0
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/attacked/square_attacked.rs : 42
		attacked_count += attacking_rooks.count_ones() as i8;
	uzp1.4s v0, v0, v1
	xtn.4h v0, v0
		// /rustc/82e1608dfa6e0b5569232559e3d385fea5a93112/library/core/src/ops/bit.rs : 178
		bitand_impl! { bool usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 }
	ldr x8, [x11, #88]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/attacked/square_attacked.rs : 115
		let king_move_mask = cache.king_moves_masks[pos as usize];
	add x9, x3, x0, lsl #3
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/attacked/square_attacked.rs : 78
		let attacking_mask = cache.pawn_attack_moves_mask[attacker.opponent() as usize][pos as usize];
	add x12, x9, x13, lsl #9
	ldr x12, [x12, #3072]
		// /rustc/82e1608dfa6e0b5569232559e3d385fea5a93112/library/core/src/ops/bit.rs : 178
		bitand_impl! { bool usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 }
	and x8, x12, x8
		// /rustc/82e1608dfa6e0b5569232559e3d385fea5a93112/library/core/src/num/mod.rs : 1169
		uint_impl! {
	fmov d1, x8
	cnt.8b v1, v1
	uaddlv.8b h1, v1
	fmov w8, s1
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/attacked/square_attacked.rs : 115
		let king_move_mask = cache.king_moves_masks[pos as usize];
	ldr x9, [x9, #4608]
		// /rustc/82e1608dfa6e0b5569232559e3d385fea5a93112/library/core/src/ops/bit.rs : 178
		bitand_impl! { bool usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 }
	ldr x11, [x11, #128]
	and x9, x11, x9
		// /rustc/82e1608dfa6e0b5569232559e3d385fea5a93112/library/core/src/num/mod.rs : 1169
		uint_impl! {
	fmov d1, x9
	cnt.8b v1, v1
	uaddlv.8b h1, v1
	fmov w9, s1
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/attacked/square_attacked.rs : 117
		attacked_count += attacking_king.count_ones() as i8;
	addv.4h h0, v0
	fmov w11, s0
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/attacked/square_attacked.rs : 42
		attacked_count += attacking_rooks.count_ones() as i8;
	add w10, w11, w10
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/attacked/square_attacked.rs : 117
		attacked_count += attacking_king.count_ones() as i8;
	add w8, w8, w9
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/attacked/square_attacked.rs : 80
		attacked_count += attacking_pawns.count_ones() as i8;
	add w8, w10, w8
	sxtb w8, w8
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/attacked/square_attacked.rs : 129
		return times_square_attacked(pos, attacker, game, cache) > 0;
	cmp w8, #0
	cset w0, gt
	.cfi_def_cfa wsp, 16
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/attacked/square_attacked.rs : 213
		}
	ldp x29, x30, [sp], #16
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
LBB55_4:
	.cfi_restore_state
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/attacked/square_attacked.rs : 20
		let knight_move_mask = cache.knight_moves_masks[pos as usize];
Lloh109:
	adrp x2, l___unnamed_35@PAGE
Lloh110:
	add x2, x2, l___unnamed_35@PAGEOFF
	mov w1, #64
	bl core::panicking::panic_bounds_check
LBB55_5:
		// /rustc/82e1608dfa6e0b5569232559e3d385fea5a93112/library/core/src/slice/index.rs : 258
		&(*slice)[self]
Lloh111:
	adrp x2, l___unnamed_36@PAGE
Lloh112:
	add x2, x2, l___unnamed_36@PAGEOFF
	mov x0, x8
	mov x1, x9
	bl core::panicking::panic_bounds_check
LBB55_6:
Lloh113:
	adrp x2, l___unnamed_37@PAGE
Lloh114:
	add x2, x2, l___unnamed_37@PAGEOFF
	mov x0, x9
	mov x1, x10
	bl core::panicking::panic_bounds_check
	.loh AdrpAdd	Lloh109, Lloh110
	.loh AdrpAdd	Lloh111, Lloh112
	.loh AdrpAdd	Lloh113, Lloh114
Lfunc_end55:
	.cfi_endproc

	.p2align	2
engine::moves::perft::_perft_unmake:
Lfunc_begin56:
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/perft.rs : 126
		fn _perft_unmake(game: &mut GameState, cache: &PrecalculatedCache, depth: u16) -> u64 {
	.cfi_startproc
	stp x28, x27, [sp, #-96]!
	.cfi_def_cfa_offset 96
	stp x26, x25, [sp, #16]
	stp x24, x23, [sp, #32]
	stp x22, x21, [sp, #48]
	stp x20, x19, [sp, #64]
	stp x29, x30, [sp, #80]
	add x29, sp, #80
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	.cfi_offset w19, -24
	.cfi_offset w20, -32
	.cfi_offset w21, -40
	.cfi_offset w22, -48
	.cfi_offset w23, -56
	.cfi_offset w24, -64
	.cfi_offset w25, -72
	.cfi_offset w26, -80
	.cfi_offset w27, -88
	.cfi_offset w28, -96
	.cfi_remember_state
	sub sp, sp, #1, lsl #12
	sub sp, sp, #48
	tst w2, #0xffff
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/perft.rs : 129
		if depth == 0 {
	b.eq LBB56_7
	mov x22, x2
	mov x19, x1
	mov x20, x0
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/movelist.rs : 12
		MoveList {
	str xzr, [sp, #4104]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/perft.rs : 137
		game.side_to_move,
	ldrb w2, [x0, #220]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/perft.rs : 134
		super::pseudolegal::all::generate_pseudolegal_moves(
	add x0, sp, #8
	mov x1, x20
	mov x3, x19
	bl engine::moves::pseudolegal::all::generate_pseudolegal_moves
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/movelist.rs : 27
		self.end
	ldr x25, [sp, #4104]
		// /rustc/82e1608dfa6e0b5569232559e3d385fea5a93112/library/core/src/iter/range.rs : 729
		if self.start < self.end {
	cbz x25, LBB56_8
	mov x21, #0
	mov x26, #0
	sub w22, w22, #1
	add x23, sp, #8
	mov w27, #56
	b LBB56_4
LBB56_3:
	add x26, x26, #1
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/perft.rs : 150
		game.unmake_move(&move_item, unmake_metadata);
	sub x2, x29, #112
	mov x0, x20
	mov x1, x23
	bl engine::state::game::GameState::unmake_move
		// /rustc/82e1608dfa6e0b5569232559e3d385fea5a93112/library/core/src/cmp.rs : 1567
		ord_impl! { char usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 }
	add x23, x23, #16
		// /rustc/82e1608dfa6e0b5569232559e3d385fea5a93112/library/core/src/iter/range.rs : 729
		if self.start < self.end {
	cmp x25, x26
	b.eq LBB56_9
LBB56_4:
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/perft.rs : 141
		let move_item = &move_list.moves[index];
	cmp x26, #256
	b.eq LBB56_10
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/perft.rs : 142
		let player = game.side_to_move;
	ldrb w28, [x20, #220]
	cmp x28, #0
	cset w24, eq
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/perft.rs : 143
		let unmake_metadata = game.make_move(move_item);
	sub x0, x29, #112
	mov x1, x20
	mov x2, x23
	bl engine::state::game::GameState::make_move
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 88
		return &self.boards[player as usize][piece as usize];
	umaddl x8, w28, w27, x20
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/attacked/in_check.rs : 10
		let king = game
	ldr x8, [x8, #128]
		// /rustc/82e1608dfa6e0b5569232559e3d385fea5a93112/library/core/src/num/mod.rs : 1169
		uint_impl! {
	rbit x8, x8
	clz x0, x8
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/attacked/in_check.rs : 15
		return is_square_attacked(
	mov x1, x24
	mov x2, x20
	mov x3, x19
	bl engine::moves::attacked::square_attacked::is_square_attacked
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/perft.rs : 145
		if !is_in_check(player, game, cache) {
	tbnz w0, #0, LBB56_3
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/perft.rs : 146
		let move_nodes = _perft_unmake(game, cache, depth - 1);
	mov x0, x20
	mov x1, x19
	mov x2, x22
	bl engine::moves::perft::_perft_unmake
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/perft.rs : 147
		nodes += move_nodes;
	add x21, x0, x21
	b LBB56_3
LBB56_7:
	mov w21, #1
	b LBB56_9
	mov x21, #0
LBB56_9:
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/perft.rs : 154
		}
	mov x0, x21
	add sp, sp, #1, lsl #12
	add sp, sp, #48
	.cfi_def_cfa wsp, 96
	ldp x29, x30, [sp, #80]
	ldp x20, x19, [sp, #64]
	ldp x22, x21, [sp, #48]
	ldp x24, x23, [sp, #32]
	ldp x26, x25, [sp, #16]
	ldp x28, x27, [sp], #96
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	.cfi_restore w19
	.cfi_restore w20
	.cfi_restore w21
	.cfi_restore w22
	.cfi_restore w23
	.cfi_restore w24
	.cfi_restore w25
	.cfi_restore w26
	.cfi_restore w27
	.cfi_restore w28
	ret
LBB56_10:
	.cfi_restore_state
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/perft.rs : 141
		let move_item = &move_list.moves[index];
Lloh115:
	adrp x2, l___unnamed_38@PAGE
Lloh116:
	add x2, x2, l___unnamed_38@PAGEOFF
	mov w0, #256
	mov w1, #256
	bl core::panicking::panic_bounds_check
	.loh AdrpAdd	Lloh115, Lloh116
Lfunc_end56:
	.cfi_endproc

	.p2align	2
engine::state::game::GameState::make_move:
Lfunc_begin57:
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 127
		pub fn make_move(&mut self, move_item: &MoveItem) -> UnmakeMoveMetadata {
	.cfi_startproc
	stp x22, x21, [sp, #-48]!
	.cfi_def_cfa_offset 48
	stp x20, x19, [sp, #16]
	stp x29, x30, [sp, #32]
	add x29, sp, #32
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	.cfi_offset w19, -24
	.cfi_offset w20, -32
	.cfi_offset w21, -40
	.cfi_offset w22, -48
	.cfi_remember_state
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 139
		.remove_piece(self.side_to_move, move_item.from_pos);
	ldrsb x8, [x2, #4]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 113
		let removed = self.pos_to_piece[pos as usize];
	cmp x8, #64
	b.hs LBB57_56
	ldr s0, [x1, #216]
	ldr x11, [x1]
	ldr w12, [x1, #208]
	ldrb w13, [x1, #220]
	add x17, x1, #8
	ldrb w9, [x17, x8]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 114
		self.pos_to_piece[pos as usize] = Piece::Empty;
	strb wzr, [x17, x8]
	mov w10, #56
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 115
		self.boards[player as usize][removed as usize].unset(pos);
	umaddl x10, w13, w10, x17
	add x9, x10, x9, lsl #3
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 31
		*self &= INVERTED_SQUARE_MASKS[index as usize];
Lloh117:
	adrp x4, l___unnamed_25@PAGE
Lloh118:
	add x4, x4, l___unnamed_25@PAGEOFF
	ldr x10, [x4, x8, lsl #3]
	ldr x14, [x9, #72]
	and x14, x14, x10
	str x14, [x9, #72]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 116
		self.pos_to_player[player as usize].unset(pos);
	add x16, x17, x13, lsl #3
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 31
		*self &= INVERTED_SQUARE_MASKS[index as usize];
	ldr x9, [x16, #184]!
	and x9, x9, x10
	str x9, [x16]
	ldr x9, [x1, #72]
	and x9, x9, x10
	str x9, [x1, #72]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/pieces.rs : 14
		#[derive(PartialEq, Debug, Clone, Copy)]
	ldrb w15, [x2, #6]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 141
		if move_item.piece == Piece::Pawn {
	cmp w15, #1
	b.ne LBB57_5
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 142
		let final_piece = if move_item.promoting {
	ldrb w9, [x2, #9]
	ldrb w10, [x2, #7]
	cmp w9, #0
	csinc w3, w10, wzr, ne
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 149
		if move_item.enpassant {
	ldrb w9, [x2, #12]
	cbz w9, LBB57_11
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/square.rs : 20
		rank * 8 + file
	and x10, x8, #0xfffffffffffffff8
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 151
		let file = Square::file(move_item.to_pos);
	ldrsb x9, [x2, #5]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/square.rs : 12
		pos % 8
	ubfx w14, w9, #12, #3
	add w14, w9, w14
	and w14, w14, #0xf8
	sub w14, w9, w14
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/square.rs : 20
		rank * 8 + file
	add x10, x10, w14, sxtb
	and w14, w10, #0xff
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 113
		let removed = self.pos_to_piece[pos as usize];
	cmp w14, #64
	b.hs LBB57_58
	and w14, w9, #0xff
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/player.rs : 15
		match self {
	cmp w13, #0
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 113
		let removed = self.pos_to_piece[pos as usize];
	ldrb w5, [x17, x10]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 114
		self.pos_to_piece[pos as usize] = Piece::Empty;
	strb wzr, [x17, x10]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/player.rs : 15
		match self {
	cset w6, eq
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 115
		self.boards[player as usize][removed as usize].unset(pos);
	add x7, x1, #8
	mov w19, #56
	umaddl x19, w6, w19, x7
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 31
		*self &= INVERTED_SQUARE_MASKS[index as usize];
	ldr x10, [x4, x10, lsl #3]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 115
		self.boards[player as usize][removed as usize].unset(pos);
	add x5, x19, x5, lsl #3
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 31
		*self &= INVERTED_SQUARE_MASKS[index as usize];
	ldr x19, [x5, #72]
	and x19, x19, x10
	str x19, [x5, #72]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 116
		self.pos_to_player[player as usize].unset(pos);
	add x5, x7, w6, uxtw #3
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 31
		*self &= INVERTED_SQUARE_MASKS[index as usize];
	ldr x6, [x5, #184]
	and x6, x6, x10
	str x6, [x5, #184]
	ldr x5, [x1, #72]
	and x10, x5, x10
	str x10, [x1, #72]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 113
		let removed = self.pos_to_piece[pos as usize];
	cmp w14, #64
	b.lo LBB57_13
	b LBB57_55
LBB57_5:
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 166
		.remove_piece(opponent, move_item.to_pos);
	ldrsb x9, [x2, #5]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 113
		let removed = self.pos_to_piece[pos as usize];
	cmp x9, #64
	b.hs LBB57_55
	and w14, w9, #0xff
	and x5, x9, #0xff
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/player.rs : 15
		match self {
	cmp w13, #0
	cset w3, eq
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 113
		let removed = self.pos_to_piece[pos as usize];
	ldrb w10, [x17, x9]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 114
		self.pos_to_piece[pos as usize] = Piece::Empty;
	strb wzr, [x17, x9]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 115
		self.boards[player as usize][removed as usize].unset(pos);
	add x6, x1, #80
	mov w7, #56
	umaddl x19, w3, w7, x6
	lsl x20, x10, #3
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 31
		*self &= INVERTED_SQUARE_MASKS[index as usize];
	ldr x4, [x4, x9, lsl #3]
	ldr x21, [x19, x20]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 115
		self.boards[player as usize][removed as usize].unset(pos);
	add x10, x1, w3, uxtw #3
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 31
		*self &= INVERTED_SQUARE_MASKS[index as usize];
	and x21, x21, x4
	str x21, [x19, x20]
	ldr x20, [x10, #192]!
	and x20, x20, x4
	ldr x21, [x1, #72]
	and x21, x21, x4
	str x21, [x1, #72]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 113
		let removed = self.pos_to_piece[pos as usize];
	ldrb w21, [x17, x9]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 114
		self.pos_to_piece[pos as usize] = Piece::Empty;
	strb wzr, [x17, x9]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 115
		self.boards[player as usize][removed as usize].unset(pos);
	lsl x21, x21, #3
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 31
		*self &= INVERTED_SQUARE_MASKS[index as usize];
	ldr x22, [x19, x21]
	and x22, x22, x4
	str x22, [x19, x21]
	str x20, [x10]
	ldr x19, [x1, #72]
	and x4, x19, x4
	str x4, [x1, #72]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 95
		self.boards[player as usize][piece as usize].set(pos);
	umaddl x4, w13, w7, x6
	lsl x6, x15, #3
	mov w7, #1
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 21
		*self |= 1 << index; // SQUARE_MASKS[index as usize];
	lsl x5, x7, x5
	ldr x7, [x4, x6]
	orr x7, x7, x5
	str x7, [x4, x6]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 96
		self.pos_to_piece[pos as usize] = piece;
	strb w15, [x17, x9]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 21
		*self |= 1 << index; // SQUARE_MASKS[index as usize];
	ldr x9, [x16]
	orr x17, x9, x5
	str x17, [x16]
	ldr x9, [x1, #72]
	orr x9, x9, x5
	str x9, [x1, #72]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 171
		if move_item.castling {
	ldrb w4, [x2, #13]
	cbz w4, LBB57_14
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 173
		match (self.side_to_move, move_item.to_pos) {
	cbz w13, LBB57_16
	cmp w14, #58
	b.eq LBB57_19
	cmp w14, #62
	b.ne LBB57_22
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 113
		let removed = self.pos_to_piece[pos as usize];
	ldrb w4, [x1, #71]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 115
		self.boards[player as usize][removed as usize].unset(pos);
	add x5, x1, #80
	mov w6, #56
	umaddl x7, w13, w6, x5
	lsl x4, x4, #3
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 31
		*self &= INVERTED_SQUARE_MASKS[index as usize];
	ldr x19, [x7, x4]
	and x19, x19, #0x7fffffffffffffff
	str x19, [x7, x4]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 114
		self.pos_to_piece[pos as usize] = Piece::Empty;
	strb wzr, [x1, #71]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 31
		*self &= INVERTED_SQUARE_MASKS[index as usize];
	and x17, x17, #0x7fffffffffffffff
	str x17, [x16]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 113
		let removed = self.pos_to_piece[pos as usize];
	ldrb w17, [x1, #69]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 115
		self.boards[player as usize][removed as usize].unset(pos);
	umaddl x3, w3, w6, x5
	lsl x17, x17, #3
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 31
		*self &= INVERTED_SQUARE_MASKS[index as usize];
	ldr x4, [x3, x17]
	and x4, x4, #0xdfffffffffffffff
	str x4, [x3, x17]
	ldr x17, [x10]
	and x17, x17, #0xdfffffffffffffff
	str x17, [x10]
	and x9, x9, #0x7fffffffffffffff
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 21
		*self |= 1 << index; // SQUARE_MASKS[index as usize];
	ldr x10, [x7, #32]
	orr x10, x10, #0x2000000000000000
	str x10, [x7, #32]
	mov w10, #4
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 96
		self.pos_to_piece[pos as usize] = piece;
	strb w10, [x1, #69]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 21
		*self |= 1 << index; // SQUARE_MASKS[index as usize];
	ldr x10, [x16]
	orr x10, x10, #0x2000000000000000
	str x10, [x16]
	orr x9, x9, #0x2000000000000000
	b LBB57_21
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 159
		.remove_piece(opponent, move_item.to_pos);
	ldrsb x9, [x2, #5]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 113
		let removed = self.pos_to_piece[pos as usize];
	cmp x9, #64
	b.hs LBB57_55
	and w14, w9, #0xff
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/player.rs : 15
		match self {
	cmp w13, #0
	cset w10, eq
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 113
		let removed = self.pos_to_piece[pos as usize];
	ldrb w5, [x17, x9]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 114
		self.pos_to_piece[pos as usize] = Piece::Empty;
	strb wzr, [x17, x9]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 115
		self.boards[player as usize][removed as usize].unset(pos);
	add x6, x1, #8
	mov w7, #56
	umaddl x7, w10, w7, x6
	add x5, x7, x5, lsl #3
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 31
		*self &= INVERTED_SQUARE_MASKS[index as usize];
	ldr x7, [x4, x9, lsl #3]
	ldr x19, [x5, #72]
	and x19, x19, x7
	str x19, [x5, #72]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 116
		self.pos_to_player[player as usize].unset(pos);
	add x10, x6, w10, uxtw #3
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 31
		*self &= INVERTED_SQUARE_MASKS[index as usize];
	ldr x5, [x10, #184]
	and x5, x5, x7
	str x5, [x10, #184]
	ldr x10, [x1, #72]
	and x10, x10, x7
	str x10, [x1, #72]
LBB57_13:
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/player.rs : 15
		match self {
	cmp w13, #0
	cset w10, eq
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 113
		let removed = self.pos_to_piece[pos as usize];
	ldrb w5, [x17, x9]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 114
		self.pos_to_piece[pos as usize] = Piece::Empty;
	strb wzr, [x17, x9]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 115
		self.boards[player as usize][removed as usize].unset(pos);
	add x6, x1, #80
	mov w7, #56
	umaddl x19, w10, w7, x6
	add x10, x1, w10, uxtw #3
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 31
		*self &= INVERTED_SQUARE_MASKS[index as usize];
	ldr x4, [x4, x9, lsl #3]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 115
		self.boards[player as usize][removed as usize].unset(pos);
	lsl x5, x5, #3
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 31
		*self &= INVERTED_SQUARE_MASKS[index as usize];
	ldr x20, [x19, x5]
	and x20, x20, x4
	str x20, [x19, x5]
	ldr x5, [x10, #192]
	and x5, x5, x4
	str x5, [x10, #192]
	ldr x10, [x1, #72]
	and x10, x10, x4
	str x10, [x1, #72]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 95
		self.boards[player as usize][piece as usize].set(pos);
	and x10, x3, #0xff
	umaddl x4, w13, w7, x6
	lsl x10, x10, #3
	mov w5, #1
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 21
		*self |= 1 << index; // SQUARE_MASKS[index as usize];
	ldr x6, [x4, x10]
	lsl x5, x5, x14
	orr x6, x6, x5
	str x6, [x4, x10]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 96
		self.pos_to_piece[pos as usize] = piece;
	strb w3, [x17, x9]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 21
		*self |= 1 << index; // SQUARE_MASKS[index as usize];
	ldr x9, [x16]
	orr x9, x9, x5
	str x9, [x16]
	ldr x9, [x1, #72]
	orr x9, x9, x5
	str x9, [x1, #72]
	b LBB57_23
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 202
		if !move_item.capturing && !move_item.castling {
	ldrb w9, [x2, #10]
	cbnz w9, LBB57_22
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 203
		self.half_move_clock += 1;
	add w9, w12, #1
	str w9, [x1, #208]
	b LBB57_23
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 173
		match (self.side_to_move, move_item.to_pos) {
	cmp w14, #2
	b.eq LBB57_20
	cmp w14, #6
	b.ne LBB57_22
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 113
		let removed = self.pos_to_piece[pos as usize];
	ldrb w4, [x1, #15]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 115
		self.boards[player as usize][removed as usize].unset(pos);
	add x5, x1, #80
	mov w6, #56
	umaddl x7, w13, w6, x5
	lsl x4, x4, #3
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 31
		*self &= INVERTED_SQUARE_MASKS[index as usize];
	ldr x19, [x7, x4]
	and x19, x19, #0xffffffffffffff7f
	str x19, [x7, x4]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 114
		self.pos_to_piece[pos as usize] = Piece::Empty;
	strb wzr, [x1, #15]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 31
		*self &= INVERTED_SQUARE_MASKS[index as usize];
	and x17, x17, #0xffffffffffffff7f
	str x17, [x16]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 113
		let removed = self.pos_to_piece[pos as usize];
	ldrb w17, [x1, #13]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 115
		self.boards[player as usize][removed as usize].unset(pos);
	umaddl x3, w3, w6, x5
	lsl x17, x17, #3
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 31
		*self &= INVERTED_SQUARE_MASKS[index as usize];
	ldr x4, [x3, x17]
	and x4, x4, #0xffffffffffffffdf
	str x4, [x3, x17]
	ldr x17, [x10]
	and x17, x17, #0xffffffffffffffdf
	str x17, [x10]
	and x9, x9, #0xffffffffffffff7f
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 21
		*self |= 1 << index; // SQUARE_MASKS[index as usize];
	ldr x10, [x7, #32]
	orr x10, x10, #0x20
	str x10, [x7, #32]
	mov w10, #4
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 96
		self.pos_to_piece[pos as usize] = piece;
	strb w10, [x1, #13]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 21
		*self |= 1 << index; // SQUARE_MASKS[index as usize];
	ldr x10, [x16]
	orr x10, x10, #0x20
	str x10, [x16]
	orr x9, x9, #0x20
	b LBB57_21
LBB57_19:
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 113
		let removed = self.pos_to_piece[pos as usize];
	ldrb w4, [x1, #64]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 115
		self.boards[player as usize][removed as usize].unset(pos);
	add x5, x1, #80
	mov w6, #56
	umaddl x7, w13, w6, x5
	lsl x4, x4, #3
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 31
		*self &= INVERTED_SQUARE_MASKS[index as usize];
	ldr x19, [x7, x4]
	and x19, x19, #0xfeffffffffffffff
	str x19, [x7, x4]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 114
		self.pos_to_piece[pos as usize] = Piece::Empty;
	strb wzr, [x1, #64]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 31
		*self &= INVERTED_SQUARE_MASKS[index as usize];
	and x17, x17, #0xfeffffffffffffff
	str x17, [x16]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 113
		let removed = self.pos_to_piece[pos as usize];
	ldrb w17, [x1, #67]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 115
		self.boards[player as usize][removed as usize].unset(pos);
	umaddl x3, w3, w6, x5
	lsl x17, x17, #3
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 31
		*self &= INVERTED_SQUARE_MASKS[index as usize];
	ldr x4, [x3, x17]
	and x4, x4, #0xf7ffffffffffffff
	str x4, [x3, x17]
	ldr x17, [x10]
	and x17, x17, #0xf7ffffffffffffff
	str x17, [x10]
	and x9, x9, #0xfeffffffffffffff
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 21
		*self |= 1 << index; // SQUARE_MASKS[index as usize];
	ldr x10, [x7, #32]
	orr x10, x10, #0x800000000000000
	str x10, [x7, #32]
	mov w10, #4
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 96
		self.pos_to_piece[pos as usize] = piece;
	strb w10, [x1, #67]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 21
		*self |= 1 << index; // SQUARE_MASKS[index as usize];
	ldr x10, [x16]
	orr x10, x10, #0x800000000000000
	str x10, [x16]
	orr x9, x9, #0x800000000000000
	b LBB57_21
LBB57_20:
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 113
		let removed = self.pos_to_piece[pos as usize];
	ldrb w4, [x1, #8]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 115
		self.boards[player as usize][removed as usize].unset(pos);
	add x5, x1, #80
	mov w6, #56
	umaddl x7, w13, w6, x5
	lsl x4, x4, #3
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 31
		*self &= INVERTED_SQUARE_MASKS[index as usize];
	ldr x19, [x7, x4]
	and x19, x19, #0xfffffffffffffffe
	str x19, [x7, x4]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 114
		self.pos_to_piece[pos as usize] = Piece::Empty;
	strb wzr, [x1, #8]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 31
		*self &= INVERTED_SQUARE_MASKS[index as usize];
	and x17, x17, #0xfffffffffffffffe
	str x17, [x16]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 113
		let removed = self.pos_to_piece[pos as usize];
	ldrb w17, [x1, #11]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 115
		self.boards[player as usize][removed as usize].unset(pos);
	umaddl x3, w3, w6, x5
	lsl x17, x17, #3
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 31
		*self &= INVERTED_SQUARE_MASKS[index as usize];
	ldr x4, [x3, x17]
	and x4, x4, #0xfffffffffffffff7
	str x4, [x3, x17]
	ldr x17, [x10]
	and x17, x17, #0xfffffffffffffff7
	str x17, [x10]
	and x9, x9, #0xfffffffffffffffe
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 21
		*self |= 1 << index; // SQUARE_MASKS[index as usize];
	ldr x10, [x7, #32]
	orr x10, x10, #0x8
	str x10, [x7, #32]
	mov w10, #4
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 96
		self.pos_to_piece[pos as usize] = piece;
	strb w10, [x1, #11]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 21
		*self |= 1 << index; // SQUARE_MASKS[index as usize];
	ldr x10, [x16]
	orr x10, x10, #0x8
	str x10, [x16]
	orr x9, x9, #0x8
LBB57_21:
	str x9, [x1, #72]
LBB57_22:
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 205
		self.half_move_clock = 0;
	str wzr, [x1, #208]
LBB57_23:
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 212
		if move_item.double {
	ldrb w9, [x2, #11]
	cbz w9, LBB57_30
	and w9, w8, #0xf8
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/square.rs : 15
		let rank = pos / 8;
	lsr w10, w14, #3
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 216
		if to_rank > from_rank {
	cmp w10, w9, lsr #3
	add w9, w10, #31
	csinc w9, w9, w10, hi
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/square.rs : 20
		rank * 8 + file
	mov x10, x14
	bfi w10, w9, #3, #29
	and w16, w10, #0xff
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 222
		self.enpassant_square = SQUARE_MASKS[Square::index(enpassant_rank, file) as usize];
	sxtb x9, w10
	cmp w16, #64
	b.hs LBB57_57
Lloh119:
	adrp x10, l___unnamed_39@PAGE
Lloh120:
	add x10, x10, l___unnamed_39@PAGEOFF
	ldr x9, [x10, x9, lsl #3]
	ushll.8h v0, v0, #0
	str x9, [x1]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/player.rs : 1
		#[derive(Debug, PartialEq, Clone, Copy)]
	ldrb w9, [x1, #220]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 228
		if self.side_to_move == Player::Black {
	cbz w9, LBB57_31
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 229
		self.full_move_number += 1;
	ldr w9, [x1, #212]
	add w9, w9, #1
	str w9, [x1, #212]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 234
		if move_item.castling || move_item.piece == Piece::King {
	cmp w15, #6
	b.eq LBB57_28
	ldrb w9, [x2, #13]
	cbz w9, LBB57_29
LBB57_28:
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 30
		self.black_queen_side = false;
	strh wzr, [x1, #218]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 244
		if move_item.piece == Piece::Rook {
	cmp w15, #4
	b.eq LBB57_36
	b LBB57_40
	mov x9, #0
	ushll.8h v0, v0, #0
	str x9, [x1]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/player.rs : 1
		#[derive(Debug, PartialEq, Clone, Copy)]
	ldrb w9, [x1, #220]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 228
		if self.side_to_move == Player::Black {
	cbnz w9, LBB57_26
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 234
		if move_item.castling || move_item.piece == Piece::King {
	cmp w15, #6
	b.eq LBB57_33
	ldrb w10, [x2, #13]
	cbz w10, LBB57_34
LBB57_33:
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 26
		self.white_queen_side = false;
	strh wzr, [x1, #216]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 244
		if move_item.piece == Piece::Rook {
	cmp w15, #4
	b.ne LBB57_40
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 245
		match (self.side_to_move, move_item.from_pos) {
	cbz w9, LBB57_49
LBB57_36:
	cmp w8, #56
	b.eq LBB57_39
	and w8, w8, #0xff
	cmp w8, #63
	b.ne LBB57_40
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 256
		self.castle_permissions.black_king_side = false;
	strb wzr, [x1, #219]
	b LBB57_40
LBB57_39:
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 253
		self.castle_permissions.black_queen_side = false;
	strb wzr, [x1, #218]
LBB57_40:
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/pieces.rs : 14
		#[derive(PartialEq, Debug, Clone, Copy)]
	ldrb w8, [x2, #8]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 261
		if move_item.captured_piece == Piece::Rook {
	cmp w8, #4
	b.ne LBB57_53
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 262
		match (opponent, move_item.to_pos) {
	cbz w13, LBB57_45
	cbz w14, LBB57_48
	cmp w14, #7
	b.ne LBB57_53
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 267
		self.castle_permissions.white_king_side = false;
	strb wzr, [x1, #217]
	b LBB57_53
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 262
		match (opponent, move_item.to_pos) {
	cmp w14, #56
	b.eq LBB57_52
	cmp w14, #63
	b.ne LBB57_53
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 273
		self.castle_permissions.black_king_side = false;
	strb wzr, [x1, #219]
	b LBB57_53
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 264
		self.castle_permissions.white_queen_side = false;
	strb wzr, [x1, #216]
	b LBB57_53
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 245
		match (self.side_to_move, move_item.from_pos) {
	cbz w8, LBB57_54
	and w8, w8, #0xff
	cmp w8, #7
	b.ne LBB57_40
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 250
		self.castle_permissions.white_king_side = false;
	strb wzr, [x1, #217]
	b LBB57_40
LBB57_52:
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 270
		self.castle_permissions.black_queen_side = false;
	strb wzr, [x1, #218]
LBB57_53:
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 280
		self.side_to_move = opponent;
	eor w9, w13, #0x1
	strb w9, [x1, #220]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 282
		UnmakeMoveMetadata {
	xtn.8b v0, v0
	str s0, [x0, #8]
	str x11, [x0]
	str w12, [x0, #12]
	strb w8, [x0, #16]
	.cfi_def_cfa wsp, 48
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 288
		}
	ldp x29, x30, [sp, #32]
	ldp x20, x19, [sp, #16]
	ldp x22, x21, [sp], #48
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	.cfi_restore w19
	.cfi_restore w20
	.cfi_restore w21
	.cfi_restore w22
	ret
	.cfi_restore_state
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 247
		self.castle_permissions.white_queen_side = false;
	strb wzr, [x1, #216]
	b LBB57_40
LBB57_55:
Lloh121:
	adrp x2, l___unnamed_40@PAGE
Lloh122:
	add x2, x2, l___unnamed_40@PAGEOFF
	mov x0, x9
	mov w1, #64
	bl core::panicking::panic_bounds_check
LBB57_56:
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 113
		let removed = self.pos_to_piece[pos as usize];
Lloh123:
	adrp x2, l___unnamed_40@PAGE
Lloh124:
	add x2, x2, l___unnamed_40@PAGEOFF
	mov x0, x8
	mov w1, #64
	bl core::panicking::panic_bounds_check
LBB57_57:
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 222
		self.enpassant_square = SQUARE_MASKS[Square::index(enpassant_rank, file) as usize];
Lloh125:
	adrp x2, l___unnamed_41@PAGE
Lloh126:
	add x2, x2, l___unnamed_41@PAGEOFF
	mov x0, x9
	mov w1, #64
	bl core::panicking::panic_bounds_check
LBB57_58:
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 113
		let removed = self.pos_to_piece[pos as usize];
Lloh127:
	adrp x2, l___unnamed_40@PAGE
Lloh128:
	add x2, x2, l___unnamed_40@PAGEOFF
	mov x0, x10
	mov w1, #64
	bl core::panicking::panic_bounds_check
	.loh AdrpAdd	Lloh117, Lloh118
	.loh AdrpAdd	Lloh119, Lloh120
	.loh AdrpAdd	Lloh121, Lloh122
	.loh AdrpAdd	Lloh123, Lloh124
	.loh AdrpAdd	Lloh125, Lloh126
	.loh AdrpAdd	Lloh127, Lloh128
.section __TEXT,__text,regular,pure_instructions
	.p2align	2
engine::moves::attacked::square_attacked::is_square_attacked:
Lfunc_begin55:
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/attacked/square_attacked.rs : 123
		pub fn is_square_attacked(
	.cfi_startproc
	stp x29, x30, [sp, #-16]!
	.cfi_def_cfa_offset 16
	mov x29, sp
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	.cfi_remember_state
	and w8, w0, #0xff
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/attacked/square_attacked.rs : 20
		let knight_move_mask = cache.knight_moves_masks[pos as usize];
	sxtb x0, w0
	cmp w8, #63
	b.hi LBB55_4
	ldr x10, [x2, #72]
	add x11, x3, x0, lsl #3
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/attacked/square_attacked.rs : 31
		cache.rook_potential_blockers_masks[pos as usize],
	ldr x8, [x11, #5632]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/precalculate/magic_bitboards.rs : 33
		let blockers_on_path = blockers & potential_blockers_mask;
	and x8, x8, x10
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/attacked/square_attacked.rs : 33
		cache.rook_magics[pos as usize],
	ldr x9, [x11, #6272]
		// /rustc/82e1608dfa6e0b5569232559e3d385fea5a93112/library/core/src/num/mod.rs : 1169
		uint_impl! {
	mul x8, x8, x9
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/attacked/square_attacked.rs : 34
		cache.rook_bit_counts[pos as usize],
	add x13, x3, x0
	mov w9, #6208
	ldrb w9, [x13, x9]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/precalculate/magic_bitboards.rs : 35
		let index = (hash >> (64 - bit_count)) as usize;
	neg w9, w9
	lsr x8, x8, x9
	mov w9, #24
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/attacked/square_attacked.rs : 36
		let rook_moves_mask = cache.rook_magic_attack_tables[pos as usize][rook_magic_index];
	smaddl x12, w0, w9, x3
	ldr x9, [x12, #16]
		// /rustc/82e1608dfa6e0b5569232559e3d385fea5a93112/library/core/src/slice/index.rs : 258
		&(*slice)[self]
	cmp x8, x9
	b.hs LBB55_5
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/attacked/square_attacked.rs : 52
		cache.bishop_potential_blockers_masks[pos as usize],
	ldr x9, [x11, #5120]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/precalculate/magic_bitboards.rs : 33
		let blockers_on_path = blockers & potential_blockers_mask;
	and x9, x9, x10
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/attacked/square_attacked.rs : 54
		cache.bishop_magics[pos as usize],
	ldr x10, [x11, #6784]
		// /rustc/82e1608dfa6e0b5569232559e3d385fea5a93112/library/core/src/num/mod.rs : 1169
		uint_impl! {
	mul x9, x9, x10
	mov w10, #6144
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/attacked/square_attacked.rs : 55
		cache.bishop_bit_counts[pos as usize],
	ldrb w10, [x13, x10]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/precalculate/magic_bitboards.rs : 35
		let index = (hash >> (64 - bit_count)) as usize;
	neg w10, w10
	lsr x9, x9, x10
	mov w10, #24
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/attacked/square_attacked.rs : 57
		let bishop_moves_mask = cache.bishop_magic_attack_tables[pos as usize][bishop_magic_index];
	smaddl x13, w0, w10, x3
	ldr x10, [x13, #1552]
		// /rustc/82e1608dfa6e0b5569232559e3d385fea5a93112/library/core/src/slice/index.rs : 258
		&(*slice)[self]
	cmp x9, x10
	b.hs LBB55_6
	ldr x10, [x11, #4096]
	mov w11, w1
	mov w14, #56
	umaddl x14, w1, w14, x2
	ldr x14, [x14, #96]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/attacked/square_attacked.rs : 57
		let bishop_moves_mask = cache.bishop_magic_attack_tables[pos as usize][bishop_magic_index];
	add x13, x13, #1536
	ldr x13, [x13]
	mov w15, #56
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 88
		return &self.boards[player as usize][piece as usize];
	umaddl x11, w11, w15, x2
		// /rustc/82e1608dfa6e0b5569232559e3d385fea5a93112/library/core/src/ops/bit.rs : 178
		bitand_impl! { bool usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 }
	ldr q0, [x11, #112]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/attacked/square_attacked.rs : 36
		let rook_moves_mask = cache.rook_magic_attack_tables[pos as usize][rook_magic_index];
	ldr x12, [x12]
		// /rustc/82e1608dfa6e0b5569232559e3d385fea5a93112/library/core/src/slice/index.rs : 258
		&(*slice)[self]
	add x8, x12, x8, lsl #3
	add x9, x13, x9, lsl #3
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 88
		return &self.boards[player as usize][piece as usize];
	add x12, x11, #104
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/player.rs : 15
		match self {
	eor w13, w1, #0x1
		// /rustc/82e1608dfa6e0b5569232559e3d385fea5a93112/library/core/src/ops/bit.rs : 178
		bitand_impl! { bool usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 }
	and x10, x14, x10
		// /rustc/82e1608dfa6e0b5569232559e3d385fea5a93112/library/core/src/num/mod.rs : 1169
		uint_impl! {
	fmov d1, x10
	cnt.8b v1, v1
	uaddlv.8b h1, v1
	fmov w10, s1
		// /rustc/82e1608dfa6e0b5569232559e3d385fea5a93112/library/core/src/ops/bit.rs : 178
		bitand_impl! { bool usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 }
	ld1r.2d { v1 }, [x12]
	zip2.2d v1, v0, v1
	ld1r.2d { v2 }, [x8]
	ld1r.2d { v3 }, [x9]
	and.16b v1, v1, v3
	and.16b v0, v0, v2
		// /rustc/82e1608dfa6e0b5569232559e3d385fea5a93112/library/core/src/num/mod.rs : 1169
		uint_impl! {
	cnt.16b v1, v1
	uaddlp.8h v1, v1
	uaddlp.4s v1, v1
	uaddlp.2d v1, v1
	cnt.16b v0, v0
	uaddlp.8h v0, v0
	uaddlp.4s v0, v0
	uaddlp.2d v0, v0
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/attacked/square_attacked.rs : 42
		attacked_count += attacking_rooks.count_ones() as i8;
	uzp1.4s v0, v0, v1
	xtn.4h v0, v0
		// /rustc/82e1608dfa6e0b5569232559e3d385fea5a93112/library/core/src/ops/bit.rs : 178
		bitand_impl! { bool usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 }
	ldr x8, [x11, #88]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/attacked/square_attacked.rs : 115
		let king_move_mask = cache.king_moves_masks[pos as usize];
	add x9, x3, x0, lsl #3
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/attacked/square_attacked.rs : 78
		let attacking_mask = cache.pawn_attack_moves_mask[attacker.opponent() as usize][pos as usize];
	add x12, x9, x13, lsl #9
	ldr x12, [x12, #3072]
		// /rustc/82e1608dfa6e0b5569232559e3d385fea5a93112/library/core/src/ops/bit.rs : 178
		bitand_impl! { bool usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 }
	and x8, x12, x8
		// /rustc/82e1608dfa6e0b5569232559e3d385fea5a93112/library/core/src/num/mod.rs : 1169
		uint_impl! {
	fmov d1, x8
	cnt.8b v1, v1
	uaddlv.8b h1, v1
	fmov w8, s1
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/attacked/square_attacked.rs : 115
		let king_move_mask = cache.king_moves_masks[pos as usize];
	ldr x9, [x9, #4608]
		// /rustc/82e1608dfa6e0b5569232559e3d385fea5a93112/library/core/src/ops/bit.rs : 178
		bitand_impl! { bool usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 }
	ldr x11, [x11, #128]
	and x9, x11, x9
		// /rustc/82e1608dfa6e0b5569232559e3d385fea5a93112/library/core/src/num/mod.rs : 1169
		uint_impl! {
	fmov d1, x9
	cnt.8b v1, v1
	uaddlv.8b h1, v1
	fmov w9, s1
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/attacked/square_attacked.rs : 117
		attacked_count += attacking_king.count_ones() as i8;
	addv.4h h0, v0
	fmov w11, s0
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/attacked/square_attacked.rs : 42
		attacked_count += attacking_rooks.count_ones() as i8;
	add w10, w11, w10
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/attacked/square_attacked.rs : 117
		attacked_count += attacking_king.count_ones() as i8;
	add w8, w8, w9
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/attacked/square_attacked.rs : 80
		attacked_count += attacking_pawns.count_ones() as i8;
	add w8, w10, w8
	sxtb w8, w8
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/attacked/square_attacked.rs : 129
		return times_square_attacked(pos, attacker, game, cache) > 0;
	cmp w8, #0
	cset w0, gt
	.cfi_def_cfa wsp, 16
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/attacked/square_attacked.rs : 213
		}
	ldp x29, x30, [sp], #16
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
LBB55_4:
	.cfi_restore_state
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/attacked/square_attacked.rs : 20
		let knight_move_mask = cache.knight_moves_masks[pos as usize];
Lloh109:
	adrp x2, l___unnamed_35@PAGE
Lloh110:
	add x2, x2, l___unnamed_35@PAGEOFF
	mov w1, #64
	bl core::panicking::panic_bounds_check
LBB55_5:
		// /rustc/82e1608dfa6e0b5569232559e3d385fea5a93112/library/core/src/slice/index.rs : 258
		&(*slice)[self]
Lloh111:
	adrp x2, l___unnamed_36@PAGE
Lloh112:
	add x2, x2, l___unnamed_36@PAGEOFF
	mov x0, x8
	mov x1, x9
	bl core::panicking::panic_bounds_check
LBB55_6:
Lloh113:
	adrp x2, l___unnamed_37@PAGE
Lloh114:
	add x2, x2, l___unnamed_37@PAGEOFF
	mov x0, x9
	mov x1, x10
	bl core::panicking::panic_bounds_check
	.loh AdrpAdd	Lloh109, Lloh110
	.loh AdrpAdd	Lloh111, Lloh112
	.loh AdrpAdd	Lloh113, Lloh114
Lfunc_end55:
	.cfi_endproc

	.p2align	2
engine::moves::perft::_perft_unmake:
Lfunc_begin56:
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/perft.rs : 126
		fn _perft_unmake(game: &mut GameState, cache: &PrecalculatedCache, depth: u16) -> u64 {
	.cfi_startproc
	stp x28, x27, [sp, #-96]!
	.cfi_def_cfa_offset 96
	stp x26, x25, [sp, #16]
	stp x24, x23, [sp, #32]
	stp x22, x21, [sp, #48]
	stp x20, x19, [sp, #64]
	stp x29, x30, [sp, #80]
	add x29, sp, #80
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	.cfi_offset w19, -24
	.cfi_offset w20, -32
	.cfi_offset w21, -40
	.cfi_offset w22, -48
	.cfi_offset w23, -56
	.cfi_offset w24, -64
	.cfi_offset w25, -72
	.cfi_offset w26, -80
	.cfi_offset w27, -88
	.cfi_offset w28, -96
	.cfi_remember_state
	sub sp, sp, #1, lsl #12
	sub sp, sp, #48
	tst w2, #0xffff
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/perft.rs : 129
		if depth == 0 {
	b.eq LBB56_7
	mov x22, x2
	mov x19, x1
	mov x20, x0
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/movelist.rs : 12
		MoveList {
	str xzr, [sp, #4104]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/perft.rs : 137
		game.side_to_move,
	ldrb w2, [x0, #220]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/perft.rs : 134
		super::pseudolegal::all::generate_pseudolegal_moves(
	add x0, sp, #8
	mov x1, x20
	mov x3, x19
	bl engine::moves::pseudolegal::all::generate_pseudolegal_moves
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/movelist.rs : 27
		self.end
	ldr x25, [sp, #4104]
		// /rustc/82e1608dfa6e0b5569232559e3d385fea5a93112/library/core/src/iter/range.rs : 729
		if self.start < self.end {
	cbz x25, LBB56_8
	mov x21, #0
	mov x26, #0
	sub w22, w22, #1
	add x23, sp, #8
	mov w27, #56
	b LBB56_4
LBB56_3:
	add x26, x26, #1
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/perft.rs : 150
		game.unmake_move(&move_item, unmake_metadata);
	sub x2, x29, #112
	mov x0, x20
	mov x1, x23
	bl engine::state::game::GameState::unmake_move
		// /rustc/82e1608dfa6e0b5569232559e3d385fea5a93112/library/core/src/cmp.rs : 1567
		ord_impl! { char usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 }
	add x23, x23, #16
		// /rustc/82e1608dfa6e0b5569232559e3d385fea5a93112/library/core/src/iter/range.rs : 729
		if self.start < self.end {
	cmp x25, x26
	b.eq LBB56_9
LBB56_4:
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/perft.rs : 141
		let move_item = &move_list.moves[index];
	cmp x26, #256
	b.eq LBB56_10
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/perft.rs : 142
		let player = game.side_to_move;
	ldrb w28, [x20, #220]
	cmp x28, #0
	cset w24, eq
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/perft.rs : 143
		let unmake_metadata = game.make_move(move_item);
	sub x0, x29, #112
	mov x1, x20
	mov x2, x23
	bl engine::state::game::GameState::make_move
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 88
		return &self.boards[player as usize][piece as usize];
	umaddl x8, w28, w27, x20
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/attacked/in_check.rs : 10
		let king = game
	ldr x8, [x8, #128]
		// /rustc/82e1608dfa6e0b5569232559e3d385fea5a93112/library/core/src/num/mod.rs : 1169
		uint_impl! {
	rbit x8, x8
	clz x0, x8
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/attacked/in_check.rs : 15
		return is_square_attacked(
	mov x1, x24
	mov x2, x20
	mov x3, x19
	bl engine::moves::attacked::square_attacked::is_square_attacked
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/perft.rs : 145
		if !is_in_check(player, game, cache) {
	tbnz w0, #0, LBB56_3
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/perft.rs : 146
		let move_nodes = _perft_unmake(game, cache, depth - 1);
	mov x0, x20
	mov x1, x19
	mov x2, x22
	bl engine::moves::perft::_perft_unmake
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/perft.rs : 147
		nodes += move_nodes;
	add x21, x0, x21
	b LBB56_3
LBB56_7:
	mov w21, #1
	b LBB56_9
	mov x21, #0
LBB56_9:
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/perft.rs : 154
		}
	mov x0, x21
	add sp, sp, #1, lsl #12
	add sp, sp, #48
	.cfi_def_cfa wsp, 96
	ldp x29, x30, [sp, #80]
	ldp x20, x19, [sp, #64]
	ldp x22, x21, [sp, #48]
	ldp x24, x23, [sp, #32]
	ldp x26, x25, [sp, #16]
	ldp x28, x27, [sp], #96
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	.cfi_restore w19
	.cfi_restore w20
	.cfi_restore w21
	.cfi_restore w22
	.cfi_restore w23
	.cfi_restore w24
	.cfi_restore w25
	.cfi_restore w26
	.cfi_restore w27
	.cfi_restore w28
	ret
LBB56_10:
	.cfi_restore_state
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/moves/perft.rs : 141
		let move_item = &move_list.moves[index];
Lloh115:
	adrp x2, l___unnamed_38@PAGE
Lloh116:
	add x2, x2, l___unnamed_38@PAGEOFF
	mov w0, #256
	mov w1, #256
	bl core::panicking::panic_bounds_check
	.loh AdrpAdd	Lloh115, Lloh116
Lfunc_end56:
	.cfi_endproc

	.p2align	2
engine::state::game::GameState::make_move:
Lfunc_begin57:
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 127
		pub fn make_move(&mut self, move_item: &MoveItem) -> UnmakeMoveMetadata {
	.cfi_startproc
	stp x22, x21, [sp, #-48]!
	.cfi_def_cfa_offset 48
	stp x20, x19, [sp, #16]
	stp x29, x30, [sp, #32]
	add x29, sp, #32
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	.cfi_offset w19, -24
	.cfi_offset w20, -32
	.cfi_offset w21, -40
	.cfi_offset w22, -48
	.cfi_remember_state
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 139
		.remove_piece(self.side_to_move, move_item.from_pos);
	ldrsb x8, [x2, #4]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 113
		let removed = self.pos_to_piece[pos as usize];
	cmp x8, #64
	b.hs LBB57_56
	ldr s0, [x1, #216]
	ldr x11, [x1]
	ldr w12, [x1, #208]
	ldrb w13, [x1, #220]
	add x17, x1, #8
	ldrb w9, [x17, x8]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 114
		self.pos_to_piece[pos as usize] = Piece::Empty;
	strb wzr, [x17, x8]
	mov w10, #56
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 115
		self.boards[player as usize][removed as usize].unset(pos);
	umaddl x10, w13, w10, x17
	add x9, x10, x9, lsl #3
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 31
		*self &= INVERTED_SQUARE_MASKS[index as usize];
Lloh117:
	adrp x4, l___unnamed_25@PAGE
Lloh118:
	add x4, x4, l___unnamed_25@PAGEOFF
	ldr x10, [x4, x8, lsl #3]
	ldr x14, [x9, #72]
	and x14, x14, x10
	str x14, [x9, #72]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 116
		self.pos_to_player[player as usize].unset(pos);
	add x16, x17, x13, lsl #3
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 31
		*self &= INVERTED_SQUARE_MASKS[index as usize];
	ldr x9, [x16, #184]!
	and x9, x9, x10
	str x9, [x16]
	ldr x9, [x1, #72]
	and x9, x9, x10
	str x9, [x1, #72]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/pieces.rs : 14
		#[derive(PartialEq, Debug, Clone, Copy)]
	ldrb w15, [x2, #6]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 141
		if move_item.piece == Piece::Pawn {
	cmp w15, #1
	b.ne LBB57_5
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 142
		let final_piece = if move_item.promoting {
	ldrb w9, [x2, #9]
	ldrb w10, [x2, #7]
	cmp w9, #0
	csinc w3, w10, wzr, ne
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 149
		if move_item.enpassant {
	ldrb w9, [x2, #12]
	cbz w9, LBB57_11
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/square.rs : 20
		rank * 8 + file
	and x10, x8, #0xfffffffffffffff8
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 151
		let file = Square::file(move_item.to_pos);
	ldrsb x9, [x2, #5]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/square.rs : 12
		pos % 8
	ubfx w14, w9, #12, #3
	add w14, w9, w14
	and w14, w14, #0xf8
	sub w14, w9, w14
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/square.rs : 20
		rank * 8 + file
	add x10, x10, w14, sxtb
	and w14, w10, #0xff
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 113
		let removed = self.pos_to_piece[pos as usize];
	cmp w14, #64
	b.hs LBB57_58
	and w14, w9, #0xff
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/player.rs : 15
		match self {
	cmp w13, #0
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 113
		let removed = self.pos_to_piece[pos as usize];
	ldrb w5, [x17, x10]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 114
		self.pos_to_piece[pos as usize] = Piece::Empty;
	strb wzr, [x17, x10]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/player.rs : 15
		match self {
	cset w6, eq
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 115
		self.boards[player as usize][removed as usize].unset(pos);
	add x7, x1, #8
	mov w19, #56
	umaddl x19, w6, w19, x7
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 31
		*self &= INVERTED_SQUARE_MASKS[index as usize];
	ldr x10, [x4, x10, lsl #3]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 115
		self.boards[player as usize][removed as usize].unset(pos);
	add x5, x19, x5, lsl #3
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 31
		*self &= INVERTED_SQUARE_MASKS[index as usize];
	ldr x19, [x5, #72]
	and x19, x19, x10
	str x19, [x5, #72]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 116
		self.pos_to_player[player as usize].unset(pos);
	add x5, x7, w6, uxtw #3
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 31
		*self &= INVERTED_SQUARE_MASKS[index as usize];
	ldr x6, [x5, #184]
	and x6, x6, x10
	str x6, [x5, #184]
	ldr x5, [x1, #72]
	and x10, x5, x10
	str x10, [x1, #72]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 113
		let removed = self.pos_to_piece[pos as usize];
	cmp w14, #64
	b.lo LBB57_13
	b LBB57_55
LBB57_5:
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 166
		.remove_piece(opponent, move_item.to_pos);
	ldrsb x9, [x2, #5]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 113
		let removed = self.pos_to_piece[pos as usize];
	cmp x9, #64
	b.hs LBB57_55
	and w14, w9, #0xff
	and x5, x9, #0xff
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/player.rs : 15
		match self {
	cmp w13, #0
	cset w3, eq
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 113
		let removed = self.pos_to_piece[pos as usize];
	ldrb w10, [x17, x9]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 114
		self.pos_to_piece[pos as usize] = Piece::Empty;
	strb wzr, [x17, x9]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 115
		self.boards[player as usize][removed as usize].unset(pos);
	add x6, x1, #80
	mov w7, #56
	umaddl x19, w3, w7, x6
	lsl x20, x10, #3
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 31
		*self &= INVERTED_SQUARE_MASKS[index as usize];
	ldr x4, [x4, x9, lsl #3]
	ldr x21, [x19, x20]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 115
		self.boards[player as usize][removed as usize].unset(pos);
	add x10, x1, w3, uxtw #3
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 31
		*self &= INVERTED_SQUARE_MASKS[index as usize];
	and x21, x21, x4
	str x21, [x19, x20]
	ldr x20, [x10, #192]!
	and x20, x20, x4
	ldr x21, [x1, #72]
	and x21, x21, x4
	str x21, [x1, #72]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 113
		let removed = self.pos_to_piece[pos as usize];
	ldrb w21, [x17, x9]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 114
		self.pos_to_piece[pos as usize] = Piece::Empty;
	strb wzr, [x17, x9]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 115
		self.boards[player as usize][removed as usize].unset(pos);
	lsl x21, x21, #3
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 31
		*self &= INVERTED_SQUARE_MASKS[index as usize];
	ldr x22, [x19, x21]
	and x22, x22, x4
	str x22, [x19, x21]
	str x20, [x10]
	ldr x19, [x1, #72]
	and x4, x19, x4
	str x4, [x1, #72]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 95
		self.boards[player as usize][piece as usize].set(pos);
	umaddl x4, w13, w7, x6
	lsl x6, x15, #3
	mov w7, #1
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 21
		*self |= 1 << index; // SQUARE_MASKS[index as usize];
	lsl x5, x7, x5
	ldr x7, [x4, x6]
	orr x7, x7, x5
	str x7, [x4, x6]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 96
		self.pos_to_piece[pos as usize] = piece;
	strb w15, [x17, x9]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 21
		*self |= 1 << index; // SQUARE_MASKS[index as usize];
	ldr x9, [x16]
	orr x17, x9, x5
	str x17, [x16]
	ldr x9, [x1, #72]
	orr x9, x9, x5
	str x9, [x1, #72]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 171
		if move_item.castling {
	ldrb w4, [x2, #13]
	cbz w4, LBB57_14
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 173
		match (self.side_to_move, move_item.to_pos) {
	cbz w13, LBB57_16
	cmp w14, #58
	b.eq LBB57_19
	cmp w14, #62
	b.ne LBB57_22
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 113
		let removed = self.pos_to_piece[pos as usize];
	ldrb w4, [x1, #71]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 115
		self.boards[player as usize][removed as usize].unset(pos);
	add x5, x1, #80
	mov w6, #56
	umaddl x7, w13, w6, x5
	lsl x4, x4, #3
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 31
		*self &= INVERTED_SQUARE_MASKS[index as usize];
	ldr x19, [x7, x4]
	and x19, x19, #0x7fffffffffffffff
	str x19, [x7, x4]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 114
		self.pos_to_piece[pos as usize] = Piece::Empty;
	strb wzr, [x1, #71]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 31
		*self &= INVERTED_SQUARE_MASKS[index as usize];
	and x17, x17, #0x7fffffffffffffff
	str x17, [x16]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 113
		let removed = self.pos_to_piece[pos as usize];
	ldrb w17, [x1, #69]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 115
		self.boards[player as usize][removed as usize].unset(pos);
	umaddl x3, w3, w6, x5
	lsl x17, x17, #3
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 31
		*self &= INVERTED_SQUARE_MASKS[index as usize];
	ldr x4, [x3, x17]
	and x4, x4, #0xdfffffffffffffff
	str x4, [x3, x17]
	ldr x17, [x10]
	and x17, x17, #0xdfffffffffffffff
	str x17, [x10]
	and x9, x9, #0x7fffffffffffffff
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 21
		*self |= 1 << index; // SQUARE_MASKS[index as usize];
	ldr x10, [x7, #32]
	orr x10, x10, #0x2000000000000000
	str x10, [x7, #32]
	mov w10, #4
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 96
		self.pos_to_piece[pos as usize] = piece;
	strb w10, [x1, #69]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 21
		*self |= 1 << index; // SQUARE_MASKS[index as usize];
	ldr x10, [x16]
	orr x10, x10, #0x2000000000000000
	str x10, [x16]
	orr x9, x9, #0x2000000000000000
	b LBB57_21
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 159
		.remove_piece(opponent, move_item.to_pos);
	ldrsb x9, [x2, #5]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 113
		let removed = self.pos_to_piece[pos as usize];
	cmp x9, #64
	b.hs LBB57_55
	and w14, w9, #0xff
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/player.rs : 15
		match self {
	cmp w13, #0
	cset w10, eq
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 113
		let removed = self.pos_to_piece[pos as usize];
	ldrb w5, [x17, x9]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 114
		self.pos_to_piece[pos as usize] = Piece::Empty;
	strb wzr, [x17, x9]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 115
		self.boards[player as usize][removed as usize].unset(pos);
	add x6, x1, #8
	mov w7, #56
	umaddl x7, w10, w7, x6
	add x5, x7, x5, lsl #3
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 31
		*self &= INVERTED_SQUARE_MASKS[index as usize];
	ldr x7, [x4, x9, lsl #3]
	ldr x19, [x5, #72]
	and x19, x19, x7
	str x19, [x5, #72]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 116
		self.pos_to_player[player as usize].unset(pos);
	add x10, x6, w10, uxtw #3
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 31
		*self &= INVERTED_SQUARE_MASKS[index as usize];
	ldr x5, [x10, #184]
	and x5, x5, x7
	str x5, [x10, #184]
	ldr x10, [x1, #72]
	and x10, x10, x7
	str x10, [x1, #72]
LBB57_13:
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/player.rs : 15
		match self {
	cmp w13, #0
	cset w10, eq
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 113
		let removed = self.pos_to_piece[pos as usize];
	ldrb w5, [x17, x9]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 114
		self.pos_to_piece[pos as usize] = Piece::Empty;
	strb wzr, [x17, x9]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 115
		self.boards[player as usize][removed as usize].unset(pos);
	add x6, x1, #80
	mov w7, #56
	umaddl x19, w10, w7, x6
	add x10, x1, w10, uxtw #3
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 31
		*self &= INVERTED_SQUARE_MASKS[index as usize];
	ldr x4, [x4, x9, lsl #3]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 115
		self.boards[player as usize][removed as usize].unset(pos);
	lsl x5, x5, #3
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 31
		*self &= INVERTED_SQUARE_MASKS[index as usize];
	ldr x20, [x19, x5]
	and x20, x20, x4
	str x20, [x19, x5]
	ldr x5, [x10, #192]
	and x5, x5, x4
	str x5, [x10, #192]
	ldr x10, [x1, #72]
	and x10, x10, x4
	str x10, [x1, #72]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 95
		self.boards[player as usize][piece as usize].set(pos);
	and x10, x3, #0xff
	umaddl x4, w13, w7, x6
	lsl x10, x10, #3
	mov w5, #1
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 21
		*self |= 1 << index; // SQUARE_MASKS[index as usize];
	ldr x6, [x4, x10]
	lsl x5, x5, x14
	orr x6, x6, x5
	str x6, [x4, x10]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 96
		self.pos_to_piece[pos as usize] = piece;
	strb w3, [x17, x9]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 21
		*self |= 1 << index; // SQUARE_MASKS[index as usize];
	ldr x9, [x16]
	orr x9, x9, x5
	str x9, [x16]
	ldr x9, [x1, #72]
	orr x9, x9, x5
	str x9, [x1, #72]
	b LBB57_23
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 202
		if !move_item.capturing && !move_item.castling {
	ldrb w9, [x2, #10]
	cbnz w9, LBB57_22
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 203
		self.half_move_clock += 1;
	add w9, w12, #1
	str w9, [x1, #208]
	b LBB57_23
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 173
		match (self.side_to_move, move_item.to_pos) {
	cmp w14, #2
	b.eq LBB57_20
	cmp w14, #6
	b.ne LBB57_22
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 113
		let removed = self.pos_to_piece[pos as usize];
	ldrb w4, [x1, #15]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 115
		self.boards[player as usize][removed as usize].unset(pos);
	add x5, x1, #80
	mov w6, #56
	umaddl x7, w13, w6, x5
	lsl x4, x4, #3
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 31
		*self &= INVERTED_SQUARE_MASKS[index as usize];
	ldr x19, [x7, x4]
	and x19, x19, #0xffffffffffffff7f
	str x19, [x7, x4]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 114
		self.pos_to_piece[pos as usize] = Piece::Empty;
	strb wzr, [x1, #15]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 31
		*self &= INVERTED_SQUARE_MASKS[index as usize];
	and x17, x17, #0xffffffffffffff7f
	str x17, [x16]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 113
		let removed = self.pos_to_piece[pos as usize];
	ldrb w17, [x1, #13]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 115
		self.boards[player as usize][removed as usize].unset(pos);
	umaddl x3, w3, w6, x5
	lsl x17, x17, #3
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 31
		*self &= INVERTED_SQUARE_MASKS[index as usize];
	ldr x4, [x3, x17]
	and x4, x4, #0xffffffffffffffdf
	str x4, [x3, x17]
	ldr x17, [x10]
	and x17, x17, #0xffffffffffffffdf
	str x17, [x10]
	and x9, x9, #0xffffffffffffff7f
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 21
		*self |= 1 << index; // SQUARE_MASKS[index as usize];
	ldr x10, [x7, #32]
	orr x10, x10, #0x20
	str x10, [x7, #32]
	mov w10, #4
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 96
		self.pos_to_piece[pos as usize] = piece;
	strb w10, [x1, #13]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 21
		*self |= 1 << index; // SQUARE_MASKS[index as usize];
	ldr x10, [x16]
	orr x10, x10, #0x20
	str x10, [x16]
	orr x9, x9, #0x20
	b LBB57_21
LBB57_19:
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 113
		let removed = self.pos_to_piece[pos as usize];
	ldrb w4, [x1, #64]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 115
		self.boards[player as usize][removed as usize].unset(pos);
	add x5, x1, #80
	mov w6, #56
	umaddl x7, w13, w6, x5
	lsl x4, x4, #3
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 31
		*self &= INVERTED_SQUARE_MASKS[index as usize];
	ldr x19, [x7, x4]
	and x19, x19, #0xfeffffffffffffff
	str x19, [x7, x4]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 114
		self.pos_to_piece[pos as usize] = Piece::Empty;
	strb wzr, [x1, #64]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 31
		*self &= INVERTED_SQUARE_MASKS[index as usize];
	and x17, x17, #0xfeffffffffffffff
	str x17, [x16]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 113
		let removed = self.pos_to_piece[pos as usize];
	ldrb w17, [x1, #67]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 115
		self.boards[player as usize][removed as usize].unset(pos);
	umaddl x3, w3, w6, x5
	lsl x17, x17, #3
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 31
		*self &= INVERTED_SQUARE_MASKS[index as usize];
	ldr x4, [x3, x17]
	and x4, x4, #0xf7ffffffffffffff
	str x4, [x3, x17]
	ldr x17, [x10]
	and x17, x17, #0xf7ffffffffffffff
	str x17, [x10]
	and x9, x9, #0xfeffffffffffffff
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 21
		*self |= 1 << index; // SQUARE_MASKS[index as usize];
	ldr x10, [x7, #32]
	orr x10, x10, #0x800000000000000
	str x10, [x7, #32]
	mov w10, #4
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 96
		self.pos_to_piece[pos as usize] = piece;
	strb w10, [x1, #67]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 21
		*self |= 1 << index; // SQUARE_MASKS[index as usize];
	ldr x10, [x16]
	orr x10, x10, #0x800000000000000
	str x10, [x16]
	orr x9, x9, #0x800000000000000
	b LBB57_21
LBB57_20:
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 113
		let removed = self.pos_to_piece[pos as usize];
	ldrb w4, [x1, #8]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 115
		self.boards[player as usize][removed as usize].unset(pos);
	add x5, x1, #80
	mov w6, #56
	umaddl x7, w13, w6, x5
	lsl x4, x4, #3
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 31
		*self &= INVERTED_SQUARE_MASKS[index as usize];
	ldr x19, [x7, x4]
	and x19, x19, #0xfffffffffffffffe
	str x19, [x7, x4]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 114
		self.pos_to_piece[pos as usize] = Piece::Empty;
	strb wzr, [x1, #8]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 31
		*self &= INVERTED_SQUARE_MASKS[index as usize];
	and x17, x17, #0xfffffffffffffffe
	str x17, [x16]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 113
		let removed = self.pos_to_piece[pos as usize];
	ldrb w17, [x1, #11]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 115
		self.boards[player as usize][removed as usize].unset(pos);
	umaddl x3, w3, w6, x5
	lsl x17, x17, #3
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 31
		*self &= INVERTED_SQUARE_MASKS[index as usize];
	ldr x4, [x3, x17]
	and x4, x4, #0xfffffffffffffff7
	str x4, [x3, x17]
	ldr x17, [x10]
	and x17, x17, #0xfffffffffffffff7
	str x17, [x10]
	and x9, x9, #0xfffffffffffffffe
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 21
		*self |= 1 << index; // SQUARE_MASKS[index as usize];
	ldr x10, [x7, #32]
	orr x10, x10, #0x8
	str x10, [x7, #32]
	mov w10, #4
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 96
		self.pos_to_piece[pos as usize] = piece;
	strb w10, [x1, #11]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 21
		*self |= 1 << index; // SQUARE_MASKS[index as usize];
	ldr x10, [x16]
	orr x10, x10, #0x8
	str x10, [x16]
	orr x9, x9, #0x8
LBB57_21:
	str x9, [x1, #72]
LBB57_22:
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 205
		self.half_move_clock = 0;
	str wzr, [x1, #208]
LBB57_23:
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 212
		if move_item.double {
	ldrb w9, [x2, #11]
	cbz w9, LBB57_30
	and w9, w8, #0xf8
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/square.rs : 15
		let rank = pos / 8;
	lsr w10, w14, #3
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 216
		if to_rank > from_rank {
	cmp w10, w9, lsr #3
	add w9, w10, #31
	csinc w9, w9, w10, hi
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/square.rs : 20
		rank * 8 + file
	mov x10, x14
	bfi w10, w9, #3, #29
	and w16, w10, #0xff
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 222
		self.enpassant_square = SQUARE_MASKS[Square::index(enpassant_rank, file) as usize];
	sxtb x9, w10
	cmp w16, #64
	b.hs LBB57_57
Lloh119:
	adrp x10, l___unnamed_39@PAGE
Lloh120:
	add x10, x10, l___unnamed_39@PAGEOFF
	ldr x9, [x10, x9, lsl #3]
	ushll.8h v0, v0, #0
	str x9, [x1]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/player.rs : 1
		#[derive(Debug, PartialEq, Clone, Copy)]
	ldrb w9, [x1, #220]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 228
		if self.side_to_move == Player::Black {
	cbz w9, LBB57_31
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 229
		self.full_move_number += 1;
	ldr w9, [x1, #212]
	add w9, w9, #1
	str w9, [x1, #212]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 234
		if move_item.castling || move_item.piece == Piece::King {
	cmp w15, #6
	b.eq LBB57_28
	ldrb w9, [x2, #13]
	cbz w9, LBB57_29
LBB57_28:
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 30
		self.black_queen_side = false;
	strh wzr, [x1, #218]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 244
		if move_item.piece == Piece::Rook {
	cmp w15, #4
	b.eq LBB57_36
	b LBB57_40
	mov x9, #0
	ushll.8h v0, v0, #0
	str x9, [x1]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/player.rs : 1
		#[derive(Debug, PartialEq, Clone, Copy)]
	ldrb w9, [x1, #220]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 228
		if self.side_to_move == Player::Black {
	cbnz w9, LBB57_26
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 234
		if move_item.castling || move_item.piece == Piece::King {
	cmp w15, #6
	b.eq LBB57_33
	ldrb w10, [x2, #13]
	cbz w10, LBB57_34
LBB57_33:
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 26
		self.white_queen_side = false;
	strh wzr, [x1, #216]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 244
		if move_item.piece == Piece::Rook {
	cmp w15, #4
	b.ne LBB57_40
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 245
		match (self.side_to_move, move_item.from_pos) {
	cbz w9, LBB57_49
LBB57_36:
	cmp w8, #56
	b.eq LBB57_39
	and w8, w8, #0xff
	cmp w8, #63
	b.ne LBB57_40
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 256
		self.castle_permissions.black_king_side = false;
	strb wzr, [x1, #219]
	b LBB57_40
LBB57_39:
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 253
		self.castle_permissions.black_queen_side = false;
	strb wzr, [x1, #218]
LBB57_40:
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/pieces.rs : 14
		#[derive(PartialEq, Debug, Clone, Copy)]
	ldrb w8, [x2, #8]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 261
		if move_item.captured_piece == Piece::Rook {
	cmp w8, #4
	b.ne LBB57_53
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 262
		match (opponent, move_item.to_pos) {
	cbz w13, LBB57_45
	cbz w14, LBB57_48
	cmp w14, #7
	b.ne LBB57_53
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 267
		self.castle_permissions.white_king_side = false;
	strb wzr, [x1, #217]
	b LBB57_53
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 262
		match (opponent, move_item.to_pos) {
	cmp w14, #56
	b.eq LBB57_52
	cmp w14, #63
	b.ne LBB57_53
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 273
		self.castle_permissions.black_king_side = false;
	strb wzr, [x1, #219]
	b LBB57_53
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 264
		self.castle_permissions.white_queen_side = false;
	strb wzr, [x1, #216]
	b LBB57_53
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 245
		match (self.side_to_move, move_item.from_pos) {
	cbz w8, LBB57_54
	and w8, w8, #0xff
	cmp w8, #7
	b.ne LBB57_40
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 250
		self.castle_permissions.white_king_side = false;
	strb wzr, [x1, #217]
	b LBB57_40
LBB57_52:
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 270
		self.castle_permissions.black_queen_side = false;
	strb wzr, [x1, #218]
LBB57_53:
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 280
		self.side_to_move = opponent;
	eor w9, w13, #0x1
	strb w9, [x1, #220]
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 282
		UnmakeMoveMetadata {
	xtn.8b v0, v0
	str s0, [x0, #8]
	str x11, [x0]
	str w12, [x0, #12]
	strb w8, [x0, #16]
	.cfi_def_cfa wsp, 48
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 288
		}
	ldp x29, x30, [sp, #32]
	ldp x20, x19, [sp, #16]
	ldp x22, x21, [sp], #48
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	.cfi_restore w19
	.cfi_restore w20
	.cfi_restore w21
	.cfi_restore w22
	ret
	.cfi_restore_state
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 247
		self.castle_permissions.white_queen_side = false;
	strb wzr, [x1, #216]
	b LBB57_40
LBB57_55:
Lloh121:
	adrp x2, l___unnamed_40@PAGE
Lloh122:
	add x2, x2, l___unnamed_40@PAGEOFF
	mov x0, x9
	mov w1, #64
	bl core::panicking::panic_bounds_check
LBB57_56:
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 113
		let removed = self.pos_to_piece[pos as usize];
Lloh123:
	adrp x2, l___unnamed_40@PAGE
Lloh124:
	add x2, x2, l___unnamed_40@PAGEOFF
	mov x0, x8
	mov w1, #64
	bl core::panicking::panic_bounds_check
LBB57_57:
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/game.rs : 222
		self.enpassant_square = SQUARE_MASKS[Square::index(enpassant_rank, file) as usize];
Lloh125:
	adrp x2, l___unnamed_41@PAGE
Lloh126:
	add x2, x2, l___unnamed_41@PAGEOFF
	mov x0, x9
	mov w1, #64
	bl core::panicking::panic_bounds_check
LBB57_58:
		// /Users/shadmanrakib/Desktop/patzer-gambit/engine/src/state/boards.rs : 113
		let removed = self.pos_to_piece[pos as usize];
Lloh127:
	adrp x2, l___unnamed_40@PAGE
Lloh128:
	add x2, x2, l___unnamed_40@PAGEOFF
	mov x0, x10
	mov w1, #64
	bl core::panicking::panic_bounds_check
	.loh AdrpAdd	Lloh117, Lloh118
	.loh AdrpAdd	Lloh119, Lloh120
	.loh AdrpAdd	Lloh121, Lloh122
	.loh AdrpAdd	Lloh123, Lloh124
	.loh AdrpAdd	Lloh125, Lloh126
	.loh AdrpAdd	Lloh127, Lloh128
