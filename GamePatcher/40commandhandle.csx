MakeScript("e_handle_cmd", @"

//e_xlog(""Running command: "" + argument0);

var cmdi = string_ord_at(argument0, 1);
var a, b, c;

if (cmdi == 40)
{
	e_set_instance_id(real(string_lettersdigits(argument0)), instance_create_depth(e_real(0), e_real(1), e_real(2), e_real(3)));
	return false;
}

if (cmdi == 41)
{
	instance_destroy(e_get_instance_id(real(string_lettersdigits(argument0))));
	return false;
}

if (cmdi == 43)
{
	with (e_get_instance_id(real(argument0)))
	{
		x = e_real(0);
		y = e_real(1);
	}
	return false;
}

if (cmdi == 103)
{
	a = e_get_tile_id(e_real(0), e_real(1));
	if (a >= 0)
		tile_delete(a);
	if (e_real(2) >= 0)
	{
		b = e_real(1) * 32;
		if (b > 1048576)
			b -= 2097152;
		e_set_tile_id(e_real(0), e_real(1), tile_add(e_real(2), e_real(3), e_real(4), e_real(5), e_real(6), e_real(0) * 32, b, e_real(7)));
	}
	else
		e_set_tile_id(e_real(0), e_real(1), -1);
	return false;
}

if (cmdi == 104)
{
	tile_add(e_real(2), e_real(3), e_real(4), e_real(5), e_real(6), e_real(0) * 32, e_real(1) * 32, e_real(7))
	return false;
}

if (cmdi == 62)
{
	room_goto(real(string_lettersdigits(argument0)));
	return true;
}

if (cmdi == 89)
{
	return true;
}

if (cmdi == 98)
{
	a = 0;
	b = true;
	while (a < 8)
	{
		if b
		{
			if (e_real(a) >= 0)
			{
				background_index[a] = e_real(a) << 0
				background_visible[a] = e_real(a) > 0
				background_htiled[a] = true
				background_vtiled[a] = e_real(a) == (e_real(a) << 0)
				background_hspeed[a] = 0
				background_vspeed[a] = 0
				background_blend[a] = 0xffffff
				background_alpha[a] = 1
			}
			else
			{
				b = false;
				background_visible[a] = false;
			}
		}
		else
		{
			background_visible[a] = false;
		}
		a += 1;
	}
}

if (cmdi == 118)
{
	view_xview[0] = e_real(0);
	view_yview[0] = e_real(1);
	return false;
}

if (cmdi == 72)
{
	with (instance_create_depth(e_real(0), e_real(1), 5, obj_head)) {
		spring = e_real(2);
		hspeed = 0;
		vspeed = 0;
		if (spring == 3)
			global.sukimaBall1 = id;
		else if (spring == 4)
			global.sukimaBall2 = id;
	}
	return false;
}

if (cmdi == 115)
{
	with (e_get_instance_id(e_real(0)))
	{
		sprite_index = e_real(1);
	}
	return false;
}

if (cmdi == 82)
{
	with (e_get_instance_id(e_real(0)))
	{
		switch (string_ord_at(argument0, 2))
		{
			case 48:
				r0 = e_real(1);
			break;
			case 49:
				r1 = e_real(1);
			break;
			case 50:
				r2 = e_real(1);
			break;
			case 51:
				r3 = e_real(1);
			break;
			case 52:
				r4 = e_real(1);
			break;
			case 53:
				r5 = e_real(1);
			break;
		}
	}
	return false;
}

if (cmdi == 83)
{
	with (e_get_instance_id(e_real(0)))
	{
		switch (string_ord_at(argument0, 2))
		{
			case 48:
				s0 = string_copy(argument0, 3, string_length(argument0) - 2);
			break;
			case 49:
				s1 = string_copy(argument0, 3, string_length(argument0) - 2);
			break;
		}
	}
	return false;
}

if (cmdi == 79)
{
	with (e_get_instance_id(e_real(0)))
	{
		switch (string_ord_at(argument0, 2))
		{
			case 48:
				o0 = e_get_instance_id(e_real(1));
			break;
			case 49:
				o1 = e_get_instance_id(e_real(1));
			break;
			case 50:
				o2 = e_get_instance_id(e_real(1));
			break;
			case 51:
				o3 = e_get_instance_id(e_real(1));
			break;
			case 52:
				o4 = e_get_instance_id(e_real(1));
			break;
			case 53:
				o5 = e_get_instance_id(e_real(1));
			break;
			case 54:
				o6 = e_get_instance_id(e_real(1));
			break;
			case 55:
				o7 = e_get_instance_id(e_real(1));
			break;
		}
	}
	return false;
}

if (cmdi == 84)
{
	with (e_get_instance_id(e_real(0)))
	{
		e_send_event(253);
		switch (string_ord_at(argument0, 2))
		{
			case 48:
				e_send_string(s0);
			break;
			case 49:
				e_send_string(s1);
			break;
			case 50:
				e_send_string(s2);
			break;
			case 51:
				e_send_string(s3);
			break;
		}
	}
	return false;
}

if (cmdi == 85)
{
	with (e_get_instance_id(e_real(0)))
	{
		e_send_event(254);
		switch (string_ord_at(argument0, 2))
		{
			case 48:
				e_send_real(r0);
			break;
			case 49:
				e_send_real(r1);
			break;
			case 50:
				e_send_real(r2);
			break;
			case 51:
				e_send_real(r3);
			break;
		}
	}
	return false;
}

if (cmdi == 88)
{
	with (e_get_instance_id(e_real(0)))
	{
		image_xscale = e_real(1);
		image_yscale = e_real(2);
	}
	return false;
}

if (cmdi == 68)
{
	with (e_get_instance_id(e_real(0)))
	{
		image_angle = e_real(1);
	}
	return false;
}

if (cmdi == 71)
{
	switch (string_ord_at(argument0, 2))
	{
		case 81:
			global.quickRetry = e_real(0);
		break;
	}
	return false;
}

if (cmdi == 64)
{
	global.character = string_lettersdigits(argument0);
	if instance_exists(obj_gameMgr) {
		if global.nineHeadMode {
    		obj_gameMgr.bankiHead = max(9, e_real(0));
    		obj_gameMgr.energy = max(9, e_real(0));
    		obj_gameMgr.ice = max(9, e_real(0));
    		obj_gameMgr.cost = max(99, e_real(0));
		} else {
    		obj_gameMgr.bankiHead = e_real(0);
	    	obj_gameMgr.energy = e_real(0);
    		obj_gameMgr.ice = e_real(0);
    		obj_gameMgr.cost = e_real(0);
		}
		obj_gameMgr.darkRefundTo = obj_gameMgr.energy - 1;
		obj_gameMgr.seijaCan = e_real(1);
		global.playerCamera = 1;
		instance_create(0, 0, obj_playStartMgr);
	}
	return false;
}

if (cmdi == 65)
{
	with (e_get_instance_id(e_real(0)))
	{
		image_alpha = e_real(1);
	}
	return false;
}

if (cmdi == 67)
{
	with (e_get_instance_id(e_real(0)))
	{
		image_blend = e_real(1);
	}
	return false;
}

if (cmdi == 70)
{
	with (e_get_instance_id(e_real(0)))
	{
		image_index = e_real(1);
	}
	return false;
}

if (cmdi == 80)
{
	audio_play_sound(e_real(0), 10, false);
	return false;
}

if (cmdi == 77)
{
	a = audio_is_playing(e_real(0))
	if (!a)
		audio_stop_sound(global.bgm)
	audio_stop_sound(bgm_stage6_sign)
	scr_bgmVol()
	global.bgm = e_real(0)
	if (global.bgm && !a)
		global.bgm = audio_play_sound(global.bgm, 100, true)
	return false;
}

if (cmdi == 42) {
	with (e_get_instance_id(e_real(0))) {
		dir = string_lettersdigits(argument0);
	}
	return false;
}

if (cmdi == 126) {
	a = real(string_lettersdigits(argument0));
	view_wview[0] = a;
	view_hview[0] = a * 9 / 16;
	return false;
}

if (cmdi == 57) {
	global.nineHeadMode = string_ord_at(argument0, 2) > 80;
	return false;
}

if (cmdi == 116) {
	global.bestTime_min = e_real(0)
	global.bestTime_sec = e_real(1)
	global.custom_pb = e_real(2)
	global.custom_wr = e_real(3)
	global.custom_vote = e_real(4)
}

if (cmdi == 100)
{
	e_send_event(129);
	e_send_string(game_save_id);
	return true;
}

if (cmdi == 113)
{
	game_end();
	return true;
}

return false;

", 1, 6);

MakeScript("e_handle_all_queue", @"

var cmd;

while true
{
	cmd = e_query();

	if (cmd == """")
	{
		break;
	}

	if e_handle_cmd(cmd)
	{
		break;
	}
}

", 0, 3);
