
// argument0 = bg
// argument1 = parallax
// argument2 = y_min
// argument3 = y_max
// argument4 = snap_to_bottom
MakeScript("e_bg_move", @"

var ay = view_yview[0];

if (argument4 > 0 && instance_exists(obj_e_precipitator))
	ay -= max(obj_e_precipitator.x, obj_e_precipitator.y) - argument4;
else if (argument4 < 0 && instance_exists(obj_e_precipitator))
	ay -= min(obj_e_precipitator.x, obj_e_precipitator.y);

if (argument0 < 0) {
	var i = 1 - argument0;
	var bx = view_xview[0] * argument1;
	var by = view_yview[0] + (argument1 - 1) * clamp(ay, argument2, argument3);
	while (i > 0) {
		i -= 1;
		background_x[i] = bx;
		background_y[i] = by;
	}
} else {
	background_x[argument0] = view_xview[0] * argument1;
	background_y[argument0] = view_yview[0] + (argument1 - 1) * clamp(ay, argument2, argument3);
}

", 5, 9);

// argument0 = bypass_seija_protect
MakeScript("e_kill_player", @"

if (!obj_goal.bStageClear && (argument0 || !instance_exists(obj_seijaHirarinuno))) {
	with (obj_player) {
		instance_create((x + 16), (y + 16), obj_deadMgr);
		effect_create_below(3, (x + 16), (y + 16), 2, c_white);
		instance_destroy();
	}
	if (audio_is_playing(bgm_stage6) || audio_is_playing(bgm_stage6_2)) {
		audio_stop_sound(global.bgm);
	}
	if instance_exists(obj_e_precipitator)
		instance_destroy(obj_seijaHirarinuno);
}

", 1, 1);

// argument0 = x
// argument1 = bonkee
// argument2 = down
MakeScript("e_bonk", @"
var d = 0
if (argument1.bbox_left + argument1.bbox_right < argument0 * 2 - 3)
	d = -4
else if (argument1.bbox_left + argument1.bbox_right > argument0 * 2 + 3)
	d = 4

if argument2
	argument1.vspeed = max(4.5, argument1.vspeed)
else
	argument1.vspeed = min(-4.5, argument1.vspeed)

if argument1.object_index == obj_player {
	if global.character == ""sekibanki"" {
		if !instance_exists(obj_bankiOther)
			instance_create(argument1.x, argument1.y, obj_bankiOther)
		obj_bankiOther.visible = true
		obj_bankiOther.subImage = 8
		argument1.visible = false
	}
	else if global.character == ""cirno"" {
		if !instance_exists(obj_cirnoOther)
			instance_create(argument1.x, argument1.y, obj_cirnoOther)
		obj_cirnoOther.visible = true
		obj_cirnoOther.subImage = 1
		argument1.visible = false
	}
	obj_gameMgr.playinput = false
	if instance_exists(obj_e_playerBonkTimer)
		obj_e_playerBonkTimer.t = 0
	else
		instance_create(0, 0, obj_e_playerBonkTimer)
	if (d != 0)
		obj_e_playerBonkTimer.d = d
}

else if !instance_exists(argument1.o0) {
	argument1.bonk = 1
	if d != 0 {
		argument1.bonkh = d
		argument1.walk = sign(d) * abs(argument1.walk)
	}
}

", 2, 3);

var flipCode = @"
	if instance_exists(obj_e_flipper) {
		obj_e_flipper.flip = true
";

foreach (var obj in "obj_player obj_head obj_switchBlock obj_switchBlockBlue obj_switchBlockGreen obj_switchBlockYellow obj_e_fairy obj_e_playerLike".Split(' ')) {
	flipCode += "with (" + obj + @") {
		if collision_rectangle(bbox_left, bbox_bottom - 2, bbox_right, bbox_bottom + 2, obj_e_flipper, true, true) != noone && !place_free(x, y + 2) {
			gravity = 0.3
			vspeed = -7.5
		}
	}
	";
}

MakeScript("e_flip", flipCode + '}', 0, 0);

MakeScript("e_read_input", @"
var td;
with obj_gameMgr {
	if i_canRead {
		i_canRead = false;
		i_w_u = i_u;
		i_w_d = i_d;
		td = e_tas_get();
		if td < 0 {
			i_l = keyboard_check(vk_left) || gamepad_button_check(global.gamePad, gp_padl) || gamepad_axis_value(global.gamePad, gp_axislh) < (-global.stick);
			i_r = keyboard_check(vk_right) || gamepad_button_check(global.gamePad, gp_padr) || gamepad_axis_value(global.gamePad, gp_axislh) > global.stick;
			i_u = keyboard_check(vk_up) || gamepad_button_check(global.gamePad, gp_padu) || gamepad_axis_value(global.gamePad, gp_axislv) < (-global.stick);
			i_d = keyboard_check(vk_down) || gamepad_button_check(global.gamePad, gp_padd);
			i_j = keyboard_check_pressed(vk_space) || gamepad_button_check_pressed(global.gamePad, global.gamePad_jump);
			i_z = keyboard_check_pressed(ord(""Z"")) || gamepad_button_check_pressed(global.gamePad, global.gamePad_z);
			i_x = keyboard_check_pressed(ord(""X"")) || gamepad_button_check_pressed(global.gamePad, global.gamePad_x);
			i_c = keyboard_check_pressed(ord(""C"")) || gamepad_button_check_pressed(global.gamePad, global.gamePad_c);
			if global.viewmode
				e_tas_add(false, false, false, false, false, false, false, false);
			else
				e_tas_add(i_l, i_r, i_u, i_d, i_j, i_z, i_x, i_c);
		} else {
			i_l = td & 128;
			i_r = td & 64;
			i_u = td & 32;
			i_d = td & 16;
			i_j = td & 8;
			i_z = td & 4;
			i_x = td & 2;
			i_c = td & 1;
		}
	}
}
", 0, 1);

// argument0 = object
MakeScript("e_photo", @"
if (argument0.bbox_left >= obj_e_seijaCamera.bbox_left && argument0.bbox_right <= obj_e_seijaCamera.bbox_right && argument0.bbox_top >= obj_e_seijaCamera.bbox_top && argument0.bbox_bottom <= obj_e_seijaCamera.bbox_bottom) {
	with (instance_create_depth(argument0.x - obj_e_seijaCamera.x, argument0.y - obj_e_seijaCamera.y, argument0.depth - 3000, obj_e_scItem)) {
		s = argument0.sprite_index
		switch argument0.object_index {
			case obj_cannonBlockUD:
			case obj_cannonBlockLR:
				f = argument0.subImage
			break

			default:
				f = floor(argument0.image_index)
			break
		}
		i = argument0
		con = argument0.solid
		o = noone
		r = false
	}
	argument0.solid = false
	argument0.y = c_white
}
", 1, 1);

// argument0 = object
MakeScript("e_photo_type", @"
if (argument0.bbox_left >= obj_e_seijaCamera.bbox_left && argument0.bbox_right <= obj_e_seijaCamera.bbox_right && argument0.bbox_top >= obj_e_seijaCamera.bbox_top && argument0.bbox_bottom <= obj_e_seijaCamera.bbox_bottom) {
	with (instance_create_depth(argument0.x - obj_e_seijaCamera.x, argument0.y - obj_e_seijaCamera.y, argument0.depth - 3000, obj_e_scItem)) {
		s = argument0.sprite_index
		o = argument0.object_index
		i = noone
		if (argument0.image_speed == 0)
			f = argument0.image_index
		else
			f = 0
		switch o {
			case obj_e_blackSwitch:
				s = spr_reverseSwitch
			case obj_redSwitchR:
			case obj_blueSwitchR:
			case obj_greenSwitchR:
			case obj_yellowSwitchR:
			case obj_e_graySwitchR:
			case obj_e_whiteSwitchR:
				r = true
			break

			default:
				r = false
			break
		}
	}
	instance_destroy(argument0, false)
}
", 1, 1);

MakeScript("e_sc_cut", @"
instance_create(0, 0, obj_seijaCameraShutter)
audio_play_sound(se_camera, 10, false)
global.seijaCamera = true
obj_player.darkBlockTimer = 100
obj_player.darkBlockFlag = 1
obj_gameMgr.playinput = 0
obj_player.FreezePoint = obj_player.x
if instance_exists(obj_seijaOther)
{
	obj_seijaOther.visible = true
	obj_seijaOther.subImage = 1
	obj_player.visible = false
}
else
{
	with (instance_create(obj_player.x, obj_player.y, obj_seijaOther))
		subImage = 1
	obj_seijaOther.visible = true
	obj_player.visible = false
}

with (obj_floor3)
	e_photo(id)
with (obj_block)
	e_photo(id)
with (obj_toge)
	if (y % 32 == 0)
		e_photo_type(id)
with (obj_toge2)
	if (y % 32 == 0)
		e_photo_type(id)
with (obj_spring)
	if (y % 32 == 0)
		e_photo_type(id)
with (obj_spring2)
	if (y % 32 == 0)
		e_photo_type(id)
with (obj_iceWall1)
	e_photo(id)
with (obj_bounceBlock)
	e_photo(id)
with (obj_redSwitch)
	if (y % 32 == 0)
		e_photo_type(id)
with (obj_blueSwitch)
	if (y % 32 == 0)
		e_photo_type(id)
with (obj_greenSwitch)
	if (y % 32 == 0)
		e_photo_type(id)
with (obj_yellowSwitch)
	if (y % 32 == 0)
		e_photo_type(id)
with (obj_redSwitchR)
	if (y % 32 == 0)
		e_photo_type(id)
with (obj_blueSwitchR)
	if (y % 32 == 0)
		e_photo_type(id)
with (obj_greenSwitchR)
	if (y % 32 == 0)
		e_photo_type(id)
with (obj_yellowSwitchR)
	if (y % 32 == 0)
		e_photo_type(id)
with (obj_graySwitch)
	if (y % 32 == 0)
		e_photo_type(id)
with (obj_whiteSwich)
	if (y % 32 == 0)
		e_photo_type(id)
with (obj_reverseSwitch)
	if (y % 32 == 0)
		e_photo_type(id)
with (obj_e_blackSwitch)
	if (y % 32 == 0)
		e_photo_type(id)
with (obj_e_graySwitchR)
	if (y % 32 == 0)
		e_photo_type(id)
with (obj_e_whiteSwitchR)
	if (y % 32 == 0)
		e_photo_type(id)
with (obj_wallRed)
	if (solid)
		e_photo_type(id)
with (obj_wallBlue)
	if (solid)
		e_photo_type(id)
with (obj_wallGreen)
	if (solid)
		e_photo_type(id)
with (obj_wallYellow)
	if (solid)
		e_photo_type(id)
with (obj_wallIce)
	if (solid)
		e_photo_type(id)
with (obj_wallWhite)
	if (solid)
		e_photo_type(id)
with (obj_wallRedOff)
	if (solid)
		e_photo_type(id)
with (obj_wallBlueOff)
	if (solid)
		e_photo_type(id)
with (obj_wallGreenOff)
	if (solid)
		e_photo_type(id)
with (obj_wallYellowOff)
	if (solid)
		e_photo_type(id)
with (obj_wallIceOff)
	if (solid)
		e_photo_type(id)
with (obj_wallWhiteOff)
	if (solid)
		e_photo_type(id)
with (obj_e_detectorblock_solid)
	e_photo_type(id)
with (obj_wall4)
	e_photo(id)
with (obj_item_key1)
	e_photo(id)
with (obj_item_key2)
	e_photo(id)
with (obj_item_key3)
	e_photo(id)
with (obj_item_key4)
	e_photo(id)
with (obj_keyblock1)
	e_photo(id)
with (obj_keyblock2)
	e_photo(id)
with (obj_keyblock3)
	e_photo(id)
with (obj_keyblock4)
	e_photo(id)
with (obj_redSwtichBlock)
	e_photo(id)
with (obj_blueSwtichBlock)
	e_photo(id)
with (obj_greenSwitchBlock)
	e_photo(id)
with (obj_yellowSwitchBlock)
	e_photo(id)
with (obj_iceSwitch)
	e_photo(id)
with (obj_Lfloor)
	e_photo_type(id)
with (obj_Rfloor)
	e_photo_type(id)
with (obj_block2)
	e_photo_type(id)
with (obj_block22)
	e_photo_type(id)
with (obj_cannonBlock)
	e_photo_type(id)
with (obj_cannonBlockD)
	e_photo_type(id)
with (obj_cannonBlockL)
	e_photo_type(id)
with (obj_cannonBlockR)
	e_photo_type(id)
with (obj_cannonBlockUD)
	e_photo(id)
with (obj_cannonBlockLR)
	e_photo(id)
/*
with (obj_timeWall)
	if (solid)
		e_photo_type(id)
with (obj_timeWallOff)
	if (solid)
		e_photo_type(id)
*/
with (obj_mochiBlock1)
	if (vs == 0)
		e_photo_type(id)

", 0, 0);

var pasteConnectCode = "";

foreach (var obj in new string[] {
	"obj_toge",
	"obj_spring",
	"obj_redSwitch",
	"obj_blueSwitch",
	"obj_greenSwitch",
	"obj_yellowSwitch",
	"obj_graySwitch",
	"obj_whiteSwich",
	"obj_e_blackSwitch",
	"obj_swtichBlockHole",
	"obj_swtichBlockBlueHole",
	"obj_switchBlockGreenHole",
	"obj_switchBlockYellowHole",
	"obj_dawkSwtich"
}) {
	pasteConnectCode += $"with ({obj}) " + @"{
		if (y % 32 == 0?) {
			con = collision_rectangle(x + 15, y + 33, x + 17, y + 35, obj_mochiBlock1, true, true)
			if (con != noone && con.vs == 0 && (e_paste_list_contains(id) || e_paste_list_contains(con)) && bbox_left >= con.bbox_left && bbox_right <= con.bbox_right) {
				if (con.o0 <= 0 || !instance_exists(con.o0)) con.o0 = id;
				else if (con.o1 <= 0 || !instance_exists(con.o1)) con.o1 = id;
				else if (con.o2 <= 0 || !instance_exists(con.o2)) con.o2 = id;
				else if (con.o3 <= 0 || !instance_exists(con.o3)) con.o3 = id;
			}
			else if e_paste_list_contains(id) {
				con = collision_rectangle(x + 15, y + 33, x + 17, y + 35, obj_chandelier, true, true)
				if (con != noone && con.gravity == 0) {
					if (con.o0 <= 0 || !instance_exists(con.o0)) { con.o0 = id; con.x0 = x - con.x; }
					else if (con.o1 <= 0 || !instance_exists(con.o1)) { con.o1 = id; con.x1 = x - con.x; }
					else if (con.o2 <= 0 || !instance_exists(con.o2)) { con.o2 = id; con.x2 = x - con.x; }
					else if (con.o3 <= 0 || !instance_exists(con.o3)) { con.o3 = id; con.x3 = x - con.x; }
				}
				else {
					con = collision_rectangle(x + 15, y + 33, x + 17, y + 35, obj_e_conveyorChandelier, true, true)
					if (con != noone && con.gravity == 0) {
						if (con.o0 <= 0 || !instance_exists(con.o0)) { con.o0 = id; con.x0 = x - con.x; }
						else if (con.o1 <= 0 || !instance_exists(con.o1)) { con.o1 = id; con.x1 = x - con.x; }
						else if (con.o2 <= 0 || !instance_exists(con.o2)) { con.o2 = id; con.x2 = x - con.x; }
						else if (con.o3 <= 0 || !instance_exists(con.o3)) { con.o3 = id; con.x3 = x - con.x; }
					}
				}
			}
		}
	".Replace("?", obj == "obj_dawkSwtich" ? "&& image_angle == 0" : "");

	if (obj == "obj_dawkSwtich")
		pasteConnectCode += @"
		else if (x % 32 == 0 && y % 32 == 0 && image_angle == 180) {
			con = collision_rectangle(x - 17, y - 35, x - 15, y - 32, obj_mochiBlock1, true, true)
			if (con != noone && con.vs == 0 && (e_paste_list_contains(id) || e_paste_list_contains(con))) {
				if (con.o0 <= 0 || !instance_exists(con.o0)) con.o0 = id;
				else if (con.o1 <= 0 || !instance_exists(con.o1)) con.o1 = id;
				else if (con.o2 <= 0 || !instance_exists(con.o2)) con.o2 = id;
				else if (con.o3 <= 0 || !instance_exists(con.o3)) con.o3 = id;
			}
		}
		";

	pasteConnectCode += "}\n";
}

foreach (var obj in new string[] {
	"obj_toge2",
	"obj_spring2",
	"obj_redSwitchR",
	"obj_blueSwitchR",
	"obj_greenSwitchR",
	"obj_yellowSwitchR",
	"obj_e_graySwitchR",
	"obj_e_whiteSwitchR",
	"obj_reverseSwitch"
}) {
	pasteConnectCode += $"with ({obj}) " + @"{
		if (x % 32 == 0 && y % 32 == 0) {
			con = collision_rectangle(x + 15, y - 3, x + 17, y, obj_mochiBlock1, true, true)
			if (con != noone && con.vs == 0 && (e_paste_list_contains(id) || e_paste_list_contains(con))) {
				if (con.o0 <= 0 || !instance_exists(con.o0)) con.o0 = id;
				else if (con.o1 <= 0 || !instance_exists(con.o1)) con.o1 = id;
				else if (con.o2 <= 0 || !instance_exists(con.o2)) con.o2 = id;
				else if (con.o3 <= 0 || !instance_exists(con.o3)) con.o3 = id;
			}
		}
	}
	";
}

var pasteCheckCode = "";

foreach (var grp in new string[][] {
	new string[] {
		"obj_redSwitchR",
		"obj_blueSwitchR",
		"obj_greenSwitchR",
		"obj_yellowSwitchR",
		"obj_e_graySwitchR",
		"obj_e_whiteSwitchR",
		"obj_reverseSwitch"
	},
	new string[] {
		"obj_redSwitch",
		"obj_blueSwitch",
		"obj_greenSwitch",
		"obj_yellowSwitch",
		"obj_graySwitch",
		"obj_whiteSwich",
		"obj_e_blackSwitch"
	},
	new string[] {
		"obj_item_key1",
		"obj_item_key2",
		"obj_item_key3",
		"obj_item_key4"
	},
}) {
	foreach (var item in grp)
		pasteCheckCode += $"case {item}:\n";
	foreach (var item in grp)
		pasteCheckCode += $"if (collision_rectangle(bbox_left, bbox_top, bbox_right, bbox_bottom, {item}, true, true) != noone)\ninstance_destroy(id, false)\n";
	pasteCheckCode += "break\n";
}

MakeScript("e_sc_paste", @"
global.seijaCamera = false
audio_play_sound(se_camera2, 10, false)

with obj_e_scItem {
	if instance_exists(i) {
		i.x = x + obj_e_seijaCamera.x
		i.y = y + obj_e_seijaCamera.y
		i.xprevious = i.x
		i.yprevious = i.y
	} else if (o != noone) {
		con = false
		switch o {
			case obj_wallRed:
			case obj_wallBlue:
			case obj_wallGreen:
			case obj_wallYellow:
			case obj_wallIce:
			case obj_wallWhite:
				con = true
				if (!o.blockStatus)
					o += 1
			break
			
			case obj_wallRedOff:
			case obj_wallBlueOff:
			case obj_wallGreenOff:
			case obj_wallYellowOff:
			case obj_wallIceOff:
			case obj_wallWhiteOff:
				con = true
				if (!o.blockStatus)
					o -= 1
			break

			case obj_timeWall:
			case obj_timeWallOff:
				con = true
				if (!o.timeWall)
					o ^= 1
			break
		}
		with (instance_create(x + obj_e_seijaCamera.x, y + obj_e_seijaCamera.y, o)) {
			blockStatus = true
			timeWall = true
			e_paste_list_add(id)
			other.i = id
			other.con = other.con || solid
			solid = false
		}
	}

	if instance_exists(i) {
		with i {
			if other.con {
				if !place_free(x, y)
					instance_destroy(id, false)
			} else {
				switch object_index {
					" + pasteCheckCode + @"
					default:
						if (collision_rectangle(bbox_left, bbox_top, bbox_right, bbox_bottom, object_index, true, true) != noone)
							instance_destroy(id, false)
					break
				}
			}
		}
	}
}

with obj_e_scItem {
	if (con && instance_exists(i))
		i.solid = true
}

instance_destroy(obj_e_scItem)

" + pasteConnectCode + "e_paste_list_clear()", 0, 1);
