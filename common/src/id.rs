const fn encode_nibble(nibble: u32) -> u8 {
	b"37CEFHKLMNPQRWXY"[nibble as usize]
}

const fn decode_nibble(byte: u8) -> u32 {
	const B_3: u8 = '3' as u8;
	const B_7: u8 = '7' as u8;
	const B_C: u8 = 'C' as u8;
	const B_E: u8 = 'E' as u8;
	const B_F: u8 = 'F' as u8;
	const B_H: u8 = 'H' as u8;
	const B_K: u8 = 'K' as u8;
	const B_L: u8 = 'L' as u8;
	const B_M: u8 = 'M' as u8;
	const B_N: u8 = 'N' as u8;
	const B_P: u8 = 'P' as u8;
	const B_Q: u8 = 'Q' as u8;
	const B_R: u8 = 'R' as u8;
	const B_W: u8 = 'W' as u8;
	const B_X: u8 = 'X' as u8;
	const B_Y: u8 = 'Y' as u8;

	match byte {
		B_3 => 0,
		B_7 => 1,
		B_C => 2,
		B_E => 3,
		B_F => 4,
		B_H => 5,
		B_K => 6,
		B_L => 7,
		B_M => 8,
		B_N => 9,
		B_P => 10,
		B_Q => 11,
		B_R => 12,
		B_W => 13,
		B_X => 14,
		B_Y => 15,
		_ => 16
	}
}

pub fn id_to_code(id: u32) -> String {
	let bytes = vec![
		encode_nibble((id & 0xf00000) >> 20),
		encode_nibble((id & 0xf0000) >> 16),
		encode_nibble((id & 0xf000) >> 12),
		'-' as u8,
		encode_nibble((id & 0xf00) >> 8),
		encode_nibble((id & 0xf0) >> 4),
		encode_nibble(id & 0xf),
	];

	unsafe { String::from_utf8_unchecked(bytes) }
}

pub fn code_to_id(code: &str) -> u32 {
	let mut code = code.as_bytes();
	let short_code;
	if code.len() == 7 && code[3] == '-' as u8 {
		short_code = [code[0], code[1], code[2], code[4], code[5], code[6]];
		code = &short_code;
	}

	if code.len() != 6 {
		return u32::MAX;
	}

	let mut id = 0;

	for byte in code {
		let nibble = decode_nibble(*byte);
		if nibble > 15 {
			return u32::MAX;
		}
		id = (id << 4) | nibble;
	}

	id
}