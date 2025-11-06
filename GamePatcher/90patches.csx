var dctx = new GlobalDecompileContext(Data, false);

ReplaceTextInGML("gml_Object_obj_jerry_Step_0", "== 400", @"== 400 && instance_exists(obj_minigame1Mgr)");
ReplaceTextInGML("gml_Object_obj_jerry_Collision_77", "obj_minigame1Mgr.j", "if(instance_exists(obj_minigame1Mgr))\nobj_minigame1Mgr.j");
ReplaceTextInGML("gml_Object_obj_jerry_Collision_78", "obj_minigame1Mgr.j", "if(instance_exists(obj_minigame1Mgr))\nobj_minigame1Mgr.j");
ReplaceTextInGML("gml_Object_obj_camera_Step_0", "cameraTimer < 4", "cameraTimer < 4 && instance_number(obj_player) == 1");
ReplaceTextInGML("gml_Object_obj_swtichBlockHole_Collision_120", "switchFlag", "obj_swtichBlockHole.switchFlag");
ReplaceTextInGML("gml_Object_obj_swtichBlockBlueHole_Collision_124", "switchFlag", "obj_swtichBlockBlueHole.switchFlag");
ReplaceTextInGML("gml_Object_obj_switchBlockGreenHole_Collision_128", "switchFlag", "obj_switchBlockGreenHole.switchFlag");
ReplaceTextInGML("gml_Object_obj_switchBlockYellowHole_Collision_132", "switchFlag", "obj_switchBlockYellowHole.switchFlag");
ReplaceTextInGML("gml_Object_obj_mochiBlock2_Step_0", "timer >= 10", "timer == 10");
ReplaceTextInGML("gml_Object_obj_trackPositionMgr_Step_0", "global.bgm", "track");
ReplaceTextInGML("gml_Object_obj_trackPositionMgr_Step_0", "obj_timeWall.", "if instance_exists(obj_timeWall)\nobj_timeWall.");
ReplaceTextInGML("gml_Object_obj_trackPositionMgr_Step_0", "obj_timeWallOff.", "if instance_exists(obj_timeWallOff)\nobj_timeWallOff.");
ReplaceTextInGML("gml_Object_obj_timeWallCount_Step_0", "obj_timeWall.", "if instance_exists(obj_timeWall)\nobj_timeWall.");
ReplaceTextInGML("gml_Object_obj_timeWallCount_Step_0", "obj_timeWallOff.", "if instance_exists(obj_timeWallOff)\nobj_timeWallOff.");
ReplaceTextInGML("gml_Object_obj_trackPositionMgr_Create_0", "}\npos", "track = global.bgm\n}\npos");
ReplaceTextInGML("gml_Object_obj_darkSwitchOn_Create_0", "instance_create(x, (y - 32), obj_switchBlockWhiteEffect)", "fx = instance_create(x, y, obj_switchBlockWhiteEffect)");
ReplaceTextInGML("gml_Object_obj_headThrowUp_Step_0", "- 128", "- up * 128");
ReplaceTextInGML("gml_Object_obj_mochiBlock1_Step_0", "vspeed -=", "vs -=");
ReplaceTextInGML("gml_Object_obj_mochiBlock1_Step_0", "gravity", "g");
ReplaceTextInGML("gml_Object_obj_bullet1_Other_0", "instance", "if(!instance_exists(obj_e_precipitator))\ninstance");
ReplaceTextInGML("gml_Object_obj_player_Collision_138", "vspeed > 0", "vspeed > other.vs");
ReplaceTextInGML("gml_Object_obj_player_Collision_138", "gravity = 0", "gravity = 0\nif (other.vs < 0)\nvspeed = -8.9");
ReplaceTextInGML("gml_Object_obj_iceWall1_Step_0", "collision_line(x, y, (x + 31), (y - 1), obj_player, true, true)", "max(collision_line(x, y, (x + 31), (y - 1), obj_player, true, true), collision_line(x, y, (x + 31), (y - 1), obj_e_playerLike, true, true))");
ReplaceTextInGML("gml_Object_obj_bounceBlock_Step_0", "obj_player.", "_id.");
ReplaceTextInGML("gml_Object_obj_bounceBlock_Step_0", "collision_line(x, y, (x + 31), (y - 1), obj_player, true, true)", "max(collision_line(x, y, (x + 31), (y - 1), obj_player, true, true), collision_line(x, y, (x + 31), (y - 1), obj_e_playerLike, true, true))");
ReplaceTextInGML("gml_Object_obj_reverseSwitch_Collision_65", "-13", "-13\ne_flip()");
ReplaceTextInGML("gml_Object_obj_player_Collision_75", "sekibanki\"", "sekibanki\" || instance_exists(obj_e_precipitator)");
ReplaceTextInGML("gml_Object_obj_cirnoGoalMission_Draw_64", "draw_sprite_ext(spr_cirnoMenuClearMission, 0", "if !instance_exists(obj_e_precipitator)\ndraw_sprite_ext(spr_cirnoMenuClearMission, 0");
ReplaceTextInGML("gml_Script_scr_pauseMenu", "case 2:", "case 2:\nif obj_pauseMgr.customLevel\ninstance_create(0, 0, obj_entranceBackMgr)\nelse");
ReplaceTextInGML("gml_Object_obj_pauseMgr_Step_0", "audio_pause_sound", "blinkySign=audio_is_playing(bgm_stage6_sign)\naudio_pause_sound(bgm_stage6_sign)\naudio_pause_sound");
ReplaceTextInGML("gml_Object_obj_pauseMgr_Step_0", "audio_resume_sound", "if(blinkySign)\naudio_resume_sound(bgm_stage6_sign)\naudio_resume_sound");
ReplaceTextInGML("gml_Object_obj_gameMgr_Step_0", "global.seconds += (1 / room_speed)", "{\nglobal.seconds += (1 / room_speed)\nframeTime += 1\n}");
ReplaceTextInGML("gml_Object_obj_gameMgr_Step_0", "global.count_up == 1", "(global.count_up && !obj_goal.bStageClear)");
ReplaceTextInGML("gml_Object_obj_gameMgr_Step_0", "audio_stop_sound", "audio_stop_sound(bgm_stage6_sign)\naudio_stop_sound");
ReplaceTextInGML("gml_Object_obj_cannonBlock_Step_0", "Timer == 30", "Timer == 30 && global.character == \"sekibanki\"");
ReplaceTextInGML("gml_Object_obj_cannonBlockD_Step_0", "Timer == 30", "Timer == 30 && global.character == \"sekibanki\"");
ReplaceTextInGML("gml_Object_obj_cannonBlockL_Step_0", "Timer == 30", "Timer == 30 && global.character == \"sekibanki\"");
ReplaceTextInGML("gml_Object_obj_cannonBlockR_Step_0", "Timer == 30", "Timer == 30 && global.character == \"sekibanki\"");
ReplaceTextInGML("gml_Object_obj_cannonBlockUD_Step_0", "Timer == 30", "Timer == 30 && global.character == \"sekibanki\"");
ReplaceTextInGML("gml_Object_obj_cannonBlockLR_Step_0", "Timer == 30", "Timer == 30 && global.character == \"sekibanki\"");
ReplaceTextInGML("gml_Object_obj_cannonBlockUD_Step_0", " flag = 0", "flag = 0\nif (y == c_white)\nexit");
ReplaceTextInGML("gml_Object_obj_cannonBlockLR_Step_0", " flag = 0", "flag = 0\nif (y == c_white)\nexit");
ReplaceTextInGML("gml_Object_obj_goalMgr_Step_0", "instance_create(x, y, obj_seijaBackMenuMgr)", "{\nif instance_exists(obj_e_precipitator)\ninstance_create(0,0,obj_entranceBackMgr)\nelse\ninstance_create(x,y,obj_seijaBackMenuMgr)\n}");
ReplaceTextInGML("gml_Object_obj_seijaOther_Step_0", ".y", ".pdy + 1");
ReplaceTextInGML("gml_Object_obj_player_Create_0", "global.character ==", "room < rm_otoge && global.character ==");
ReplaceTextInGML("gml_Object_obj_iceWall1_Step_0", "iceCollision == 1", "iceCollision && y != c_white");
ReplaceTextInGML("gml_Object_obj_player_Collision_75", "if (global.character", "e_read_input()\nif (global.character");
ReplaceTextInGML("gml_Object_obj_player_Collision_75", "keyboard_check_pressed(ord(\"C\")) || gamepad_button_check_pressed(global.gamePad, global.gamePad_c)", "obj_gameMgr.i_c");
ReplaceTextInGML("gml_Object_obj_player_Collision_75", "keyboard_check_pressed(vk_up) || gamepad_button_check_pressed(global.gamePad, gp_padu)", "(obj_gameMgr.i_u && !obj_gameMgr.i_w_u)");
ReplaceTextInGML("gml_Object_obj_player_Collision_75", "headHold = 1", "headHold = 1\npickedUp = other.id\nif (obj_gameMgr.frameTime - other.lastBounce < 2 && instance_exists(obj_e_watch_srt))\nobj_e_watch_srt.flag = true");
ReplaceTextInGML("gml_Object_obj_clearTime_Draw_64", "global.rank == 5", "global.rank == 5 || obj_goal.newRecord == 2");
ReplaceTextInGML("gml_Room_rm_boot_Create", "!?:_{}@<-", "!?:_{}@<-/l");
ReplaceTextInGML("gml_Object_obj_goal_Collision_65", "with", "if obj_gameMgr.frameTime == 0\nexit\nwith");
ReplaceTextInGML("gml_Object_obj_head_Step_0", "vspeed = 0.5", "if place_free(x, y + 1)\nvspeed = 0.5");
ReplaceTextInGML("gml_Object_obj_goalMgr_Create_0", "ini_open", "if (room == rm_editor)\nexit\nini_open");

ReplaceTextInGML("gml_Object_obj_seijaHirarinuno_Step_0", "y = obj_player.y", @"
	if (obj_player.vspeed == 0 && instance_exists(obj_e_precipitator)) {
		y = obj_player.pdy
	} else
		y = obj_player.y
");

ReplaceTextInGML("gml_Object_obj_pauseMgr_Step_0", "instance_deactivate_all(true)", @"
	customLevel = instance_exists(obj_e_precipitator)
	instance_deactivate_all(true)
	instance_activate_object(obj_editor)
	instance_activate_object(obj_e_stageStartMgr)
");

ReplaceTextInGML("gml_Object_obj_pauseBG_Draw_64", "if (global.nineHeadMode == 0)", @"
	if instance_exists(obj_e_stageStartMgr) {
		draw_set_font(font_message)
		draw_set_halign(fa_right)
		draw_set_valign(fa_middle)
		if (string_width(obj_e_stageStartMgr.s1) > 192)
			draw_text(logo1 + 412, 176 + 80, obj_e_stageStartMgr.s1)
		else
			draw_text_transformed(logo1 + 412, 176 + 80, obj_e_stageStartMgr.s1, 2, 2, 0)
		draw_text(logo2 + 502, 306, obj_e_stageStartMgr.s0)
		draw_set_halign(0)
		draw_set_valign(0)
	}
	if instance_exists(obj_pauseMenuMgr)
		f = obj_pauseMenuMgr.fontMenu
	draw_set_font(f)
	if (global.nineHeadMode == 0)
");

ReplaceTextInGML("gml_Script_scr_cirnoGoalMenu", "switch", @"
	if instance_exists(obj_e_precipitator) {
		if mpos == 0 {
			instance_create(0, 0, obj_entranceBackMgr)
			audio_play_sound(se_decide, 30, false)
		} else if mpos == 1 {
			instance_create(0, 0, obj_retryMgr)
			audio_play_sound(se_decide, 30, false)
		} else if mpos == 2 && global.custom_vote >= 0 {
			e_send_event(248)
			global.custom_vote = 1 - global.custom_vote
			if (global.custom_vote == 1)
				menu[2] = ""l LIKED!""
			else
				menu[2] = ""LIKE!""
		}
	} else
		switch"
);

ReplaceTextInGML("gml_Object_obj_head_Step_0", "_id = collision_line((x + 7), (y + 9), (x + 24), (y + 23), obj_desert, true, true)", @"
	if (instance_exists(obj_e_precipitator))
		_id = collision_rectangle((x + 7), (y + 9), (x + 24), (y + 23), obj_desert, true, true)
	else
		_id = collision_line((x + 7), (y + 9), (x + 24), (y + 23), obj_desert, true, true)
");

ReplaceTextInGML("gml_Object_obj_turara_Step_0", "if ", @"
	if kp {
		e_kill_player(false)
		instance_destroy()
		exit
	}
	if kpl {
		e_bonk(x + 16, kpl, gravity > 0)
		instance_destroy()
		exit
	}
	if (o0 == 0)
		o0 = collision_rectangle(x + 1, y + 1, x + 31, y + 2, obj_e_floorMove, 1, 1)
	if (instance_exists(obj_e_precipitator) && place_free(x, y - 2) && o0 == noone) || collision_line((x + 7), (y + 1), (x + 25), (y + 256), obj_e_playerLike, true, true) != noone ||"
);

ReplaceTextInGML("gml_Object_obj_seijaUI_Draw_64", "draw_set_font(global.font)", @"
	if ((obj_gameMgr.seijaCan & (1 << global.seijaItem)) > 0)
		draw_set_font(global.font)
	else {
		draw_set_colour(0)
		draw_set_alpha(0.8)
		draw_rectangle(0, 404, 960, 506, false)
		draw_set_colour(c_white)
		draw_set_alpha(1)
		draw_sprite_ext(spr_seijaUI, 0, px, py, scX, scY, rot, col, a)
		exit
	}
");

ReplaceTextInGML("gml_Object_obj_trackPositionMgr_Create_0", "if ((", @"
if x == 3 {
	bpm = 140
	global.bgm = audio_play_sound(bgm_stage6_2, 10, true)
	track = global.bgm
}

else if x == 2 {
	global.bgm = audio_play_sound(bgm_stage6, 10, true)
	track = global.bgm
}

else if x == 1 {
	audio_stop_sound(bgm_stage6_sign)
	track = audio_play_sound(bgm_stage6_sign, 100, true)
}

else if ((");

ReplaceTextInGML("gml_Object_obj_mochiBlock1_Step_0", "instance_create(pointX, pointY, obj_mochiBlock2)", @"
	if (o0 && instance_exists(o0))
		o0.visible = false;
	if (o1 && instance_exists(o1))
		o1.visible = false;
	if (o2 && instance_exists(o2))
		o2.visible = false;
	if (o3 && instance_exists(o3))
		o3.visible = false;
	with (instance_create(pointX, pointY, obj_mochiBlock2)) {
		o0 = other.o0;
		o1 = other.o1;
		o2 = other.o2;
		o3 = other.o3;
	}
");

ReplaceTextInGML("gml_Object_obj_mochiBlock2_Step_0", "visible = true", @"{
	visible = true;
	if (o0 && instance_exists(o0)) {
		o0.visible = true;
		if (o0.y < y + 32)
			o0.y = y - 32;
		else
			o0.y = y + 96 + o0.image_angle * 32 / 180;
	}
	if (o1 && instance_exists(o1)) {
		o1.visible = true;
		if (o1.y < y + 32)
			o1.y = y - 32;
		else
			o1.y = y + 96 + o1.image_angle * 32 / 180;
	}
	if (o2 && instance_exists(o2)) {
		o2.visible = true;
		if (o2.y < y + 32)
			o2.y = y - 32;
		else
			o2.y = y + 96 + o2.image_angle * 32 / 180;
	}
	if (o3 && instance_exists(o3)) {
		o3.visible = true;
		if (o3.y < y + 32)
			o3.y = y - 32;
		else
			o3.y = y + 96 + o3.image_angle * 32 / 180;
	}
}");

Data.Code.ByName("gml_Object_obj_mochiBlock1_Create_0").AppendGML("o0=0\no1=0\no2=0\no3=0\ng=0\nvs=0", Data);
Data.Code.ByName("gml_Object_obj_headThrowUp_Create_0").AppendGML("up=1", Data);
Data.Code.ByName("gml_Object_obj_chandelier_Create_0").AppendGML("r0=128\nr1=0\no0=0\no1=0\no2=0\no3=0\nx0=-8\nx1=-8\nx2=24\nx3=24", Data);
Data.Code.ByName("gml_Object_obj_item_headPlus_Create_0").AppendGML("r0=1\no0=0", Data);
Data.Code.ByName("gml_Object_obj_onmyoudama3_Create_0").AppendGML("r0=1\nr1=1", Data);
Data.Code.ByName("gml_Object_obj_obmyoudama4_Create_0").AppendGML("r0=0\nr1=0\nr2=1\nr3=1", Data);
Data.Code.ByName("gml_Object_obj_clearTime_Draw_64").AppendGML("if (obj_goal.newRecord == 2)\ndraw_text(x, (y + 72), \"WORLD RECORD!\")", Data);
Data.Code.ByName("gml_Object_obj_clearTime2_Draw_64").AppendGML("if (obj_goal.newRecord == 2)\ndraw_text(x, (y + 100), \"WORLD RECORD!\")", Data);
Data.Code.ByName("gml_Object_obj_bullet1_Create_0").AppendGML("if (!place_free(x, y))\nimage_alpha=0", Data);
Data.Code.ByName("gml_Object_obj_playStartMgr_Create_0").AppendGML("canGo=false", Data);

Data.Code.ByName("gml_Object_obj_gameMgr_Create_0").AppendGML(@"
	i_canRead = true
	i_u = false
	i_d = false
	darkRefundTo = 2
	seijaCan = 15
	frameTime = 0
	if (room != rm_editor)
		e_tas_try_read(""tas/"" + room_get_name(room) + "".tas"")
	e_tas_start(game_save_id)
", Data);

Data.Code.ByName("gml_Object_obj_chandelier_Step_0").AppendGML(@"
	if r1 {
		if r1 == 2 {
			r0 = y - min(obj_e_precipitator.x, obj_e_precipitator.y) + 64
		}
		r1 = 0
		var i = 0
		while i < r0 {
			i += 32
			instance_create(x - 8, y - i, obj_chandelierLine)
		}
		i = instance_create(x - 8, y, obj_chandelierLine)
		i.image_index = 1
	}
", Data);

Data.Code.ByName("gml_Object_obj_iceWall1_Destroy_0").AppendGML(@"
	if sprite_index == spr_ice2 {
		with (instance_create_depth(x + 9, y + 10, -999, obj_blank)) {
			gravity = 0.3
			hspeed = -3
			vspeed = -5
			sprite_index = spr_e_frog
		}
	}
", Data);

Data.Code.ByName("gml_Object_obj_cirnoGoalMenuMgr_Create_0").AppendGML(@"
	if instance_exists(obj_e_precipitator) {
		if instance_exists(obj_e_cam_follow)
			menu[0] = ""BACK EDITOR!""
		else if instance_exists(obj_e_watch_srt)
			menu[0] = ""SUBMIT!""
		else
			menu[0] = ""BACK MENU!""
		menu[1] = ""RETRY!""
		switch (global.custom_vote) {
			case 0:
				menu[2] = ""LIKE!""
			break

			case 1:
				menu[2] = ""l LIKED!""
			break

			default:
				menu[2] = """"
		}

	}
", Data);

Data.Code.ByName("gml_Object_obj_pauseMenuMgr_Create_0").AppendGML(@"
	if obj_pauseMgr.customLevel {
		//if instance_exists(obj_e_cam_follow)
		//	menu[2] = ""BACK TO EDITOR""
		//else
			menu[2] = ""BACK TO MENU""
	}
", Data);

Data.Code.ByName("gml_Object_obj_magicSquareB_Collision_65").ReplaceGML(@"
	if instance_exists(obj_magicSquarePointB) {
		obj_player.x = obj_magicSquarePointB.x
		obj_player.y = obj_magicSquarePointB.y
		audio_play_sound(se_warp, 30, false)
	}

	else if (!instance_exists(obj_seijaHirarinuno) && obj_e_precipitator.warpTimer < 0 || obj_e_precipitator.warpHash != x % 256 + y / 32 % 32) {
		obj_player.x = r0
		obj_player.y = r1
		obj_e_precipitator.warpHash = r0 % 256 + r1 / 32 % 32
		audio_play_sound(se_warp, 30, false)
	}

	obj_e_precipitator.warpTimer = 1;
", Data);

Data.Code.ByName("gml_Object_obj_onmyoudama3_Step_0").ReplaceGML(@"
	rot -= 8
	rot2 += 5
	if (rot == -360)
		rot = 0
	if (rot2 == 360)
		rot2 = 0
	bulletTimer1++
	if (bulletTimer1 == 6)
	{
		if r1
		with (instance_create((x + 16), (y + 16), obj_bullet1))
			vspeed = 3
		if r0
		with (instance_create((x + 16), (y + 16), obj_bullet1))
			vspeed = -3
		bulletTimer1 = 0
	}
", Data);

Data.Code.ByName("gml_Object_obj_obmyoudama4_Step_0").ReplaceGML(@"
	rot -= 8
	rot2 += 5
	if (rot == -360)
		rot = 0
	if (rot2 == 360)
		rot2 = 0
	bulletTimer1++
	if (bulletTimer1 == 6)
	{
		if r1
		with (instance_create((x + 16), (y + 16), obj_bullet1))
			vspeed = 3
		if r0
		with (instance_create((x + 16), (y + 16), obj_bullet1))
			vspeed = -3
		if r3
		with (instance_create((x + 16), (y + 16), obj_bullet1))
		{
			hspeed = 3
			image_index = 1
		}
		if r2
		with (instance_create((x + 16), (y + 16), obj_bullet1))
		{
			hspeed = -3
			image_index = 1
		}
		bulletTimer1 = 0
	}
", Data);

Data.GameObjects.ByName("obj_bullet1").EventHandlerFor(EventType.Step, EventSubtypeStep.EndStep, Data.Strings, Data.Code, Data.CodeLocals).AppendGML(@"
	if (instance_exists(obj_e_precipitator) && (x < obj_e_precipitator.r4 || x > obj_e_precipitator.r5 || y < min(obj_e_precipitator.x, obj_e_precipitator.y) - 512 || y > max(obj_e_precipitator.x, obj_e_precipitator.y)))
		instance_destroy()
", Data);

Data.GameObjects.ByName("obj_turara").EventHandlerFor(EventType.Collision, 64, Data.Strings, Data.Code, Data.CodeLocals).ReplaceGML("instance_destroy(other)", Data);
Data.GameObjects.ByName("obj_turara").EventHandlerFor(EventType.Collision, 180, Data.Strings, Data.Code, Data.CodeLocals).ReplaceGML("vspeed=min(vspeed,0.5)", Data);
Data.GameObjects.ByName("obj_mochiBlock1").EventHandlerFor(EventType.Step, EventSubtypeStep.EndStep, Data.Strings, Data.Code, Data.CodeLocals).AppendGML(@"
	y += vs;
	if (o0 && instance_exists(o0))
		o0.y += vs;
	if (o1 && instance_exists(o1))
		o1.y += vs;
	if (o2 && instance_exists(o2))
		o2.y += vs;
	if (o3 && instance_exists(o3))
		o3.y += vs;
	vs += g;
", Data);

void Shard(string obj) {
	ReplaceTextInGML(obj + "_Other_0", "action", "if(!instance_exists(obj_e_precipitator))\naction");
	Data.Code.ByName(obj + "_Step_0").AppendGML("if(instance_exists(obj_e_precipitator) && y > max(obj_e_precipitator.x, obj_e_precipitator.y))\ninstance_destroy()", Data);
}

void Shard4(string pfx) {
	for (int i = 1; i < 5; i++)
		Shard(pfx + i.ToString());
}

Shard("gml_Object_obj_jerryShard");
Shard("gml_Object_obj_bubbleShard");
Shard4("gml_Object_obj_iceShard");
Shard4("gml_Object_obj_blockShard");
Shard4("gml_Object_obj_block2shard");

void CannonHead(string cn) {
	ReplaceTextInGML(cn, "instance_destroy()", @"{
		if (instance_exists(obj_e_precipitator)) {
			if (instance_exists(obj_cameraTarget)) {
				global.playerCamera = 1;
				instance_destroy(obj_cameraTarget);
				if (global.viewmode == 0)
					obj_gameMgr.playinput = 1;
				if (instance_exists(obj_whiteSwich))
					obj_whiteSwich.flag = 0;
			}
		}
		instance_destroy();
	}");
}

CannonHead("gml_Object_obj_headThrowDownCannon_Step_0");
CannonHead("gml_Object_obj_headThrowUpCannon_Step_0");
CannonHead("gml_Object_obj_headThrowLCannon_Step_0");
CannonHead("gml_Object_obj_headThrowCannon_Step_0");

var flipperId = (uint)Data.GameObjects.IndexOf(Data.GameObjects.ByName("obj_e_flipper"));

Data.GameObjects.ByName("obj_player").EventHandlerFor(EventType.Collision, flipperId, Data.Strings, Data.Code, Data.CodeLocals).ReplaceGML(@"
	if (other.solid) {
		if (vspeed < 0) {
			move_contact_solid(90, abs(vspeed))
			vspeed = 0
		}
		else if (vspeed > 0) {
			move_contact_solid(270, abs(vspeed))
			floorCheckCount = -1
			vspeed = 0
			gravity = 0
			instance_create(x, y, obj_landingEffect)
			floorCheck = 0
		}
	}
", Data);

Data.GameObjects.ByName("obj_head").EventHandlerFor(EventType.Collision, flipperId, Data.Strings, Data.Code, Data.CodeLocals).ReplaceGML(@"
	if (other.solid) {
		if (vspeed < 0) {
			move_contact_solid(90, abs(vspeed))
			vspeed = 0
		}
		else if (vspeed > 0) {
			move_contact_solid(270, abs(vspeed))
			vspeed = 0
			gravity = 0
			hspeed = 0
		}
	}
", Data);

Data.GameObjects.ByName("obj_turara").EventHandlerFor(EventType.Collision, (uint)0, Data.Strings, Data.Code, Data.CodeLocals).ReplaceGML("if other.solid\ninstance_destroy()", Data);
Data.GameObjects.ByName("obj_turara").EventHandlerFor(EventType.Collision, 39, Data.Strings, Data.Code, Data.CodeLocals).ReplaceGML("if other.solid\ninstance_destroy()", Data);
Data.GameObjects.ByName("obj_turara").EventHandlerFor(EventType.Collision, 40, Data.Strings, Data.Code, Data.CodeLocals).ReplaceGML("if other.solid\ninstance_destroy()", Data);
Data.GameObjects.ByName("obj_turara").EventHandlerFor(EventType.Collision, 10, Data.Strings, Data.Code, Data.CodeLocals).ReplaceGML("instance_destroy()", Data);
Data.GameObjects.ByName("obj_turara").EventHandlerFor(EventType.Collision, 11, Data.Strings, Data.Code, Data.CodeLocals).ReplaceGML("instance_destroy()", Data);

void MPStick(string objName) {
	var obj = Data.GameObjects.ByName(objName);

	var code = @"
		var floorMove = collision_rectangle(bbox_left, bbox_bottom - 2, bbox_right, bbox_bottom + 3, obj_e_floorMove, 1, 1);
		if (floorMove != noone && (vspeed >= 0 || vspeed >= floorMove.vspeed) && floorMove.bbox_top > bbox_bottom - 3) {
			vspeed = 0;
			if (floorMove.vspeed > 0) {
				y = min(y, floorMove.y - (bbox_bottom - y)) - 1;
				move_contact_solid(270, 4);
				y = floor(y);
				gravity = 0;
			} else if (floorMove.vspeed < 0 && !place_free(x, y - 1)) {
				action_kill_object();
				instance_create(x, y, obj_headEffect);
				audio_play_sound(se_hold, 10, false);
			}
			if (collision_rectangle(bbox_left, bbox_bottom - 2, bbox_right, ceil(bbox_bottom) + 1, obj_e_floorMove, 1, 1))
				hspeed = floorMove.hspeed;
			else
				hspeed = 0;
			movingFloorX = hspeed != 0;
		}
		else if (movingFloorX) {
			hspeed = 0;
			movingFloorX = false;
			if (!place_free(x, y) && place_free(x, y - 3)) {
				while (!place_free(x, y)) {
					y -= 1;
				}
			}
		}
	";

	obj.EventHandlerFor(EventType.Step, EventSubtypeStep.Step, Data.Strings, Data.Code, Data.CodeLocals).AppendGML(code, Data);
	obj.EventHandlerFor(EventType.Create, Data.Strings, Data.Code, Data.CodeLocals).AppendGML("movingFloorX = false", Data);
}

MPStick("obj_head");

void GemHole(string objName) {
	var eh = Data.GameObjects.ByName(objName).Events[4][0].Actions[0].CodeId;
	var lines = Decompiler.Decompile(eh, dctx).Split('\n');
	var code = "if (image_angle != 0)\nexit\n";

	foreach (var line in lines) {
		if (line.Contains("instance_create")) {
			code += $"with ({line}) {{\no0 = other.id;\n}}\n";
			int.TryParse(new String(line.Where(Char.IsDigit).ToArray()), out int offset);
			objName = line.Substring(line.IndexOf("obj_"));
			objName = objName.Substring(0, objName.IndexOf(')'));
			Data.GameObjects.ByName(objName).EventHandlerFor(EventType.Step, EventSubtypeStep.EndStep, Data.Strings, Data.Code, Data.CodeLocals).AppendGML($"x=o0.x\ny=o0.y\nvisible=o0.visible\nimage_angle=o0.image_angle", Data);
			Data.GameObjects.ByName(objName).Sprite.OriginY = offset;
		}

		else {
			code += line;
			code += "\n";
		}
	}

	eh.ReplaceGML(code, Data);
}

GemHole("obj_swtichBlockHole");
GemHole("obj_swtichBlockBlueHole");
GemHole("obj_switchBlockGreenHole");
GemHole("obj_switchBlockYellowHole");

string mochiBounce = Decompiler.Decompile(Data.Code.ByName("gml_Object_obj_mochiBlock1_Collision_75"), dctx);

void MochiBounce(string obj) {
	var id = (uint)Data.GameObjects.IndexOf(Data.GameObjects.ByName(obj));
	Data.GameObjects.ByName("obj_mochiBlock1").EventHandlerFor(EventType.Collision, id, Data.Strings, Data.Code, Data.CodeLocals).ReplaceGML(mochiBounce.Replace("obj_head", obj), Data);
}

MochiBounce("obj_e_fairy");

void BombBounce(string obj) {
	Data.GameObjects.ByName(obj).EventHandlerFor(EventType.Collision, 1467, Data.Strings, Data.Code, Data.CodeLocals).ReplaceGML("if other.Timer == 0 {\nvspeed = -6.5\ngravity = 0.3\n}", Data);
}

BombBounce("obj_head");
BombBounce("obj_e_playerLike");

void TerminalVelocity(string obj) {
	Data.GameObjects.ByName(obj).EventHandlerFor(EventType.Step, EventSubtypeStep.Step, Data.Strings, Data.Code, Data.CodeLocals).AppendGML("if (vspeed > 16)\nvspeed = 16", Data);
}

void AddSolidCheck(string obj) {
	var code = Data.Code.ByName($"gml_Object_{obj}_Collision_0");
	code.ReplaceGML($"if other.solid {{\n{Decompiler.Decompile(code, dctx)}\n}}", Data);
}

AddSolidCheck("obj_head");

var plid = (uint)Data.GameObjects.IndexOf(Data.GameObjects.ByName("obj_e_playerLike"));

void Gem(string objName) {
	MochiBounce(objName);
	BombBounce(objName);
	MPStick(objName);
	TerminalVelocity(objName);
	AddSolidCheck(objName);
	var co = Data.Code.ByName($"gml_Object_{objName}_Step_0");
	var lines = Decompiler.Decompile(co, dctx).Split('\n').Select(l => l.Trim());
	var code = "";

	var vars = new HashSet<string>();

	foreach (var line in lines) {
		if (line.Contains("obj_gameMgr.")) {
			var vn = line.Substring(line.IndexOf("obj_gameMgr.") + 12);
			vn = vn.Substring(0, vn.IndexOf(' '));
			vars.Add(vn);

			code += line.Replace("obj_gameMgr.", "");
		}

		else {
			code += line;
		}
		
		code += "\n";
	}

	foreach (var vn in vars) {
		code += $"if {vn}\nobj_gameMgr.{vn} = 1\n";
	}

	co.ReplaceGML(code, Data);

	Data.GameObjects.ByName(objName).EventHandlerFor(EventType.Collision, plid, Data.Strings, Data.Code, Data.CodeLocals).ReplaceGML(@"
		if x < other.x && place_free(x - 1, y) {
			x -= 1
			other.pushL = true
		} else if x > other.x && place_free(x + 1, y) {
			x += 1
			other.pushR = true
		}
	", Data);

	Data.GameObjects.ByName(objName).EventHandlerFor(EventType.Collision, flipperId, Data.Strings, Data.Code, Data.CodeLocals).ReplaceGML(
		Decompiler.Decompile(Data.GameObjects.ByName(objName).EventHandlerFor(EventType.Collision, 44, Data.Strings, Data.Code, Data.CodeLocals), dctx).Replace("obj_wallBlueOff", "other"), Data);
	
	var id = Data.GameObjects.IndexOf(Data.GameObjects.ByName(objName));
	ReplaceTextInGML($"gml_Object_obj_floor_Collision_{id}", "-6", "-6 - instance_exists(obj_e_precipitator)");
}

Gem("obj_switchBlock");
Gem("obj_switchBlockBlue");
Gem("obj_switchBlockGreen");
Gem("obj_switchBlockYellow");

void Button(string objName) {
	var co = Data.Code.ByName($"gml_Object_{objName}_Collision_65");
	var lines = Decompiler.Decompile(co, dctx).Split('\n');
	var code = "";

	foreach (var line in lines) {
		if (!line.Contains("obj_jumpEffectH") && !line.Contains("obj_player") && !line.Contains("se_jump")) {
			code += line;
			code += "\n";
		}
	}

	code += "instance_destroy(other)";

	Data.GameObjects.ByName(objName).EventHandlerFor(EventType.Collision, 152, Data.Strings, Data.Code, Data.CodeLocals).ReplaceGML(code, Data);
}

Button("obj_redSwitch");
Button("obj_blueSwitch");
Button("obj_greenSwitch");
Button("obj_yellowSwitch");
Button("obj_graySwitch");
Button("obj_whiteSwich");
Button("obj_e_blackSwitch");

var darkSwitchCode = @"
	if sprite_index > 0 {
		with (instance_create(x, y, obj_darkSwitchOn)) {
			o0 = other.id
		}
		sprite_index = -1
	}
";

Data.GameObjects.ByName("obj_dawkSwtich").EventHandlerFor(EventType.Collision, 77, Data.Strings, Data.Code, Data.CodeLocals).ReplaceGML(darkSwitchCode, Data);
Data.GameObjects.ByName("obj_dawkSwtich").EventHandlerFor(EventType.Collision, 78, Data.Strings, Data.Code, Data.CodeLocals).ReplaceGML(darkSwitchCode, Data);
Data.GameObjects.ByName("obj_dawkSwtich").EventHandlerFor(EventType.Collision, 101, Data.Strings, Data.Code, Data.CodeLocals).ReplaceGML(darkSwitchCode, Data);
Data.GameObjects.ByName("obj_dawkSwtich").EventHandlerFor(EventType.Collision, 1467, Data.Strings, Data.Code, Data.CodeLocals).ReplaceGML(darkSwitchCode, Data);

Data.GameObjects.ByName("obj_darkSwitchOn").EventHandlerFor(EventType.Step, EventSubtypeStep.EndStep, Data.Strings, Data.Code, Data.CodeLocals).AppendGML(@"
	x=o0.x
	y=o0.y
	visible=o0.visible
	image_angle=o0.image_angle
	fx.x=x
	fx.y=y
	fx.visible=visible
	fx.image_angle=image_angle
", Data);

var flipIceSwitch = @"
if (image_index == 0)
	obj_iceSwitch.image_index = 1
else
	obj_iceSwitch.image_index = 0
if (obj_wallIce.blockStatus == 1)
	obj_wallIce.blockStatus = 0
else
	obj_wallIce.blockStatus = 1
if (obj_wallIceOff.blockStatus == 1)
	obj_wallIceOff.blockStatus = 0
else
	obj_wallIceOff.blockStatus = 1
audio_play_sound(se_iceSwitch, 30, false)
";

Data.GameObjects.ByName("obj_iceSwitch").EventHandlerFor(EventType.Collision, 1469, Data.Strings, Data.Code, Data.CodeLocals).ReplaceGML($"if other.a > 0.95 {{{flipIceSwitch}}}", Data);

flipIceSwitch += "instance_create(other.x-8, other.y-8, obj_iceMakeEffect)\n";

Data.GameObjects.ByName("obj_iceSwitch").EventHandlerFor(EventType.Collision, 152, Data.Strings, Data.Code, Data.CodeLocals).ReplaceGML(flipIceSwitch + "instance_destroy(other)", Data);
Data.GameObjects.ByName("obj_iceSwitch").EventHandlerFor(EventType.Collision, 77, Data.Strings, Data.Code, Data.CodeLocals).ReplaceGML(flipIceSwitch + @"
	if other.object_index == 77 && (x - other.x) * other.hspeed > 0 {
		with (other) {
			dir = !dir
			ThrowPoint = x - (ThrowPoint - x)
			hspeed *= -1
		}
	}
	else
		other.tAnim = 119
", Data);
Data.GameObjects.ByName("obj_iceSwitch").EventHandlerFor(EventType.Collision, 78, Data.Strings, Data.Code, Data.CodeLocals).ReplaceGML(flipIceSwitch + @"
	if other.object_index == 78 && (y - other.y) * other.up < 0 {
		with (other) {
			ThrowPoint = y - (ThrowPoint - y)
			up *= -1
		}
	}
	else
		other.tAnim = 119
", Data);

var chandelierLandCode = Decompiler.Decompile(Data.Code.ByName("gml_Object_obj_chandelier_Collision_0"), dctx);
var mfConvert = @"
	if (other.object_index == obj_e_floorMove || other.object_index == obj_e_movingFloorChandelier || other.object_index == obj_e_conveyorChandelier || other.object_index = obj_chandelier) {
		if (other.object_index == obj_e_movingFloorChandelier) {
			mf = other.mf;
			mfx = other.mfx + x - other.x;
			mfy = other.mfy - 32;
		} else {
			mf = other.id;
			mfx = x - other.x;
			mfy = -32;
		}
		with (instance_create(x, y, obj_e_movingFloorChandelier)) {
			o0 = other.o0;
			o1 = other.o1;
			o2 = other.o2;
			o3 = other.o3;
			x0 = other.x0;
			x1 = other.x1;
			x2 = other.x2;
			x3 = other.x3;
			mf = other.mf;
			mfx = other.mfx;
			mfy = other.mfy;
		}
		with (obj_e_movingFloorChandelier) {
			if mf == other.id {
				mf = other.mf;
				mfx += other.mfx;
				mfy += other.mfy;
			}
		}
		instance_destroy();
	}
";

Data.GameObjects.ByName("obj_chandelier").EventHandlerFor(EventType.Collision, (uint)0, Data.Strings, Data.Code, Data.CodeLocals).ReplaceGML($"if (fall && other.solid && other.y > y + 8) {{\n{chandelierLandCode}\n}}", Data);
Data.GameObjects.ByName("obj_chandelier").EventHandlerFor(EventType.Collision, 39, Data.Strings, Data.Code, Data.CodeLocals).ReplaceGML($"if (fall && other.solid && other.y > y + 8) {{\n{chandelierLandCode}\n}}", Data);
Data.GameObjects.ByName("obj_chandelier").EventHandlerFor(EventType.Collision, 40, Data.Strings, Data.Code, Data.CodeLocals).ReplaceGML($"if (fall && other.solid && other.y > y + 8) {{\n{chandelierLandCode}\n}}", Data);
Data.GameObjects.ByName("obj_chandelier").EventHandlerFor(EventType.Collision, 10, Data.Strings, Data.Code, Data.CodeLocals).ReplaceGML($"if (fall && other.y > y + 8) {{\n{chandelierLandCode}\n}}", Data);
Data.GameObjects.ByName("obj_chandelier").EventHandlerFor(EventType.Collision, 11, Data.Strings, Data.Code, Data.CodeLocals).ReplaceGML($"if (fall && other.y > y + 8) {{\n{chandelierLandCode}\n}}", Data);
Data.GameObjects.ByName("obj_chandelier").EventHandlerFor(EventType.Collision, 1463, Data.Strings, Data.Code, Data.CodeLocals).ReplaceGML($"if (fall && other.y > y + 8) {{\n{chandelierLandCode}\n}}", Data);
Data.GameObjects.ByName("obj_chandelier").EventHandlerFor(EventType.Collision, 611, Data.Strings, Data.Code, Data.CodeLocals).ReplaceGML($"if (fall && other.y > y + 8 && gravity != 0) {{\n{chandelierLandCode}\n{mfConvert}\n}}", Data);
Data.GameObjects.ByName("obj_chandelier").EventHandlerFor(EventType.Collision, 152, Data.Strings, Data.Code, Data.CodeLocals).ReplaceGML("fall=true", Data);

Data.GameObjects.ByName("obj_chandelier").EventHandlerFor(EventType.Step, EventSubtypeStep.EndStep, Data.Strings, Data.Code, Data.CodeLocals).ReplaceGML(@"
	if (o0 && instance_exists(o0)) {
		o0.x = x + x0
		o0.y = y - 32
	}
	if (o1 && instance_exists(o1)) {
		o1.x = x + x1
		o1.y = y - 32
	}
	
	if (o2 && instance_exists(o2)) {
		o2.x = x + x2
		o2.y = y - 32
	}
	if (o3 && instance_exists(o3)) {
		o3.x = x + x3
		o3.y = y - 32
	}
", Data);

TerminalVelocity("obj_chandelier");

foreach (var evt in Data.GameObjects.ByName("obj_item_headPlus").Events[4]) {
	ReplaceTextInGML(evt.Actions[0].CodeId.Name.Content, "obj_gameMgr.bankiHead++", @"
		obj_gameMgr.bankiHead += r0
		obj_gameMgr.ice += r0
		obj_gameMgr.energy += r0
		obj_gameMgr.cost += r0
		obj_gameMgr.darkRefundTo += r0
		if (o0)
			instance_destroy(o0)
	");
}

Data.GameObjects.ByName("obj_item_headPlus").EventHandlerFor(EventType.Draw, Data.Strings, Data.Code, Data.CodeLocals).ReplaceGML(@"
	var bs = spr_e_cirnoHead;
	var sy = y - 1;
	var c = 128;
	if global.character == ""sekibanki"" {
		draw_self()
		exit
	}

	else if (global.character == ""rumia"")
		bs = spr_e_rumiaHead
	else if (global.character == ""seija"")
		bs = spr_e_seijaHead
	
	if floor(image_index) == 0 {
		sy = y
		c = 0
	}
	else if floor(image_index) == 2 {
		sy = y - 2
		c = 4210943
	}

	draw_sprite(bs, 0, x, sy)
	draw_sprite_ext(bs + 3, 0, x, sy, 1, 1, 0, c, 1);
", Data);

void BlockBullets(uint id) {
	Data.GameObjects.ByName("obj_bullet1").EventHandlerFor(EventType.Collision, id, Data.Strings, Data.Code, Data.CodeLocals).ReplaceGML("if (instance_exists(obj_e_precipitator) && other.solid)\ninstance_destroy()", Data);
}

BlockBullets(0);
BlockBullets(10);
BlockBullets(11);

void BlockBulletsDown(uint id) {
	Data.GameObjects.ByName("obj_bullet1").EventHandlerFor(EventType.Collision, id, Data.Strings, Data.Code, Data.CodeLocals).ReplaceGML("if vspeed > 0\ninstance_destroy()", Data);
}

BlockBulletsDown(176);
BlockBulletsDown(1517 + 17);

Data.GameObjects.ByName("obj_bullet1").EventHandlerFor(EventType.Collision, 1463, Data.Strings, Data.Code, Data.CodeLocals).ReplaceGML("instance_destroy()", Data);

void OnIceBullet(string objName, string code = null) {
	var obj = Data.GameObjects.ByName(objName);
	code ??= Decompiler.Decompile(obj.EventHandlerFor(EventType.Collision, 77, Data.Strings, Data.Code, Data.CodeLocals), dctx);
	obj.EventHandlerFor(EventType.Collision, 85, Data.Strings, Data.Code, Data.CodeLocals).ReplaceGML(code, Data);
	obj.EventHandlerFor(EventType.Collision, 86, Data.Strings, Data.Code, Data.CodeLocals).ReplaceGML(code, Data);
	obj.EventHandlerFor(EventType.Collision, 87, Data.Strings, Data.Code, Data.CodeLocals).ReplaceGML(code, Data);
	obj.EventHandlerFor(EventType.Collision, 88, Data.Strings, Data.Code, Data.CodeLocals).ReplaceGML(code, Data);
	obj.EventHandlerFor(EventType.Collision, 89, Data.Strings, Data.Code, Data.CodeLocals).ReplaceGML(code, Data);
	obj.EventHandlerFor(EventType.Collision, 90, Data.Strings, Data.Code, Data.CodeLocals).ReplaceGML(code, Data);
}

void Cannon(int id, string direction) {
	Data.GameObjects[id].EventHandlerFor(EventType.Create, Data.Strings, Data.Code, Data.CodeLocals).AppendGML("bf=0\nbft=0", Data);
	Data.GameObjects[id].EventHandlerFor(EventType.Step, EventSubtypeStep.EndStep, Data.Strings, Data.Code, Data.CodeLocals).AppendGML(@$"
		bft++
		if bft == 6 {{
			bft = 0
			if bf {{
				bf = false
				if ({direction} == ""UP"")
					with (instance_create(x + 16, y - 6, obj_bullet1))
						vspeed = -3
				else if ({direction} == ""DOWN"")
					with (instance_create(x + 16, y + 38, obj_bullet1))
						vspeed = 3
				else if ({direction} == ""LEFT"") {{
					with (instance_create(x - 6, y + 16, obj_bullet1)) {{
						hspeed = -3
						image_index = 1
					}}
				}}
				else if ({direction} == ""RIGHT"") {{
					with (instance_create(x + 38, y + 16, obj_bullet1)) {{
						hspeed = 3
						image_index = 1
					}}
				}}
			}}
		}}

		if cannonFlag
			global.iceResidual = true

		if Timer == 30 {{
			Timer = 0
			cannonFlag = false
			//audio_play_sound(se_iceShot, 10, false)

			if ({direction} == ""UP"") {{
				with instance_create(x, y - 26, obj_e_iceBulletCamera) {{
					o0 = instance_create(x, y, obj_iceBullet5)
					instance_create(x, y, obj_headEffect)
				}}
			}}

			else if ({direction} == ""DOWN"") {{
				with instance_create(x, y + 25, obj_e_iceBulletCamera) {{
					o0 = instance_create(x, y, obj_iceBullet4)
					instance_create(x, y, obj_headEffect)
				}}
			}}

			else if ({direction} == ""LEFT"") {{
				with instance_create(x - 29, y, obj_e_iceBulletCamera) {{
					o0 = instance_create(x, y, obj_iceBullet2)
					instance_create(x, y, obj_headEffect)
				}}
			}}

			else if ({direction} == ""RIGHT"") {{
				with instance_create(x + 29, y, obj_e_iceBulletCamera) {{
					o0 = instance_create(x, y, obj_iceBullet3)
					instance_create(x, y, obj_headEffect)
				}}
			}}

			if (global.viewmode == 0)
				obj_gameMgr.playinput = false
		}}
	", Data);
	Data.GameObjects.ByName("obj_bullet1").EventHandlerFor(EventType.Collision, (uint)id, Data.Strings, Data.Code, Data.CodeLocals).ReplaceGML("other.bf=true\ninstance_destroy()", Data);
	OnIceBullet(Data.GameObjects[id].Name.Content);
}

Cannon(169, "\"UP\"");
Cannon(170, "\"DOWN\"");
Cannon(171, "\"LEFT\"");
Cannon(172, "\"RIGHT\"");
Cannon(173, "dir");
Cannon(174, "dir");

Data.GameObjects.ByName("obj_turara").EventHandlerFor(EventType.Create, Data.Strings, Data.Code, Data.CodeLocals).AppendGML("o0=0\nkp=false\nkpl=false", Data);
Data.GameObjects.ByName("obj_turara").EventHandlerFor(EventType.Step, EventSubtypeStep.EndStep, Data.Strings, Data.Code, Data.CodeLocals).AppendGML(@"
	if gravity == 0 && instance_exists(o0) {
		if place_free(o0.x, o0.bbox_bottom) || place_free(x, y - 2) {
			x = o0.x
			y = o0.bbox_bottom
		} else
			o0 = noone
	}
", Data);
Data.GameObjects.ByName("obj_turara").EventHandlerFor(EventType.Collision, 611, Data.Strings, Data.Code, Data.CodeLocals).ReplaceGML("if (gravity != 0 && bbox_bottom - vspeed < other.bbox_bottom)\ninstance_destroy()", Data);
Data.GameObjects.ByName("obj_turara").EventHandlerFor(EventType.Collision, 65, Data.Strings, Data.Code, Data.CodeLocals).ReplaceGML(@"
	if instance_exists(obj_e_precipitator)
		kp = true
	else {
		e_kill_player(true)
		instance_destroy()
	}
", Data);

void BellBounceI(string bell, string bounce, bool pl) {
	var id = (uint)Data.GameObjects.IndexOf(Data.GameObjects.ByName(bounce));
	var lines = Decompiler.Decompile(Data.Code.ByName($"gml_Object_{bell}_Collision_65"), dctx).Split('\n');
	var code = "";
	var bdl = false;
	string _id = null;
	
	foreach (var line in lines) {
		if (line.Contains("var _id") || (!pl && line.Contains("se_jump")))
			continue;
		if (line.Contains("obj_bankiDownLine"))
			bdl = true;
		if (line.Contains("_id = "))
			_id = line.Substring(line.IndexOf("_id =") + 6).Replace("obj_player", bounce).Replace("obj_bankiDownLine", bounce);
		else
			code += line.Replace("_id", _id).Replace("obj_player", "other") + '\n';
	}

	if (!pl)
		code = code.Replace("other.vspeed > 0", "other.vspeed > 0 && other.bbox_bottom < bbox_bottom");
	else if (bdl)
		code = code.Replace("other.vspeed > 0", "other.vspeed > 0 && other.bbox_bottom - 10 < bbox_bottom");

	Data.GameObjects.ByName(bell).EventHandlerFor(EventType.Collision, id, Data.Strings, Data.Code, Data.CodeLocals).ReplaceGML(code, Data);
}

void BellBounce(string obj, bool pl) {
	BellBounceI("obj_bell", obj, pl);
	for (int i = 1; i < 8; i++)
		BellBounceI($"obj_piano{i}", obj, pl);
}

BellBounce("obj_e_fairy", false);
BellBounce("obj_e_playerLike", true);

Data.GameObjects.ByName("obj_head").EventHandlerFor(EventType.Collision, plid, Data.Strings, Data.Code, Data.CodeLocals).ReplaceGML(@"
	if (other.vspeed < 0 || other.bbox_bottom - y > 17)
		exit
	if spring == 0 {
		other.vspeed = -7.5
		other.gravity = 0.3
		if (global.bankiSprite == 16)
			subImage = 12
		else
			subImage = 1
		audio_play_sound(se_jump, 10, false)
		instance_create(x, (y - 23), obj_jumpEffectH)
	} else if spring < 3 {
		other.vspeed = -9
		other.gravity = 0.3
		tAnim = -13
		audio_play_sound(se_switch, 10, false)
		audio_play_sound(se_jump, 10, false)
		instance_create(x, (y - 22), obj_jumpEffectH)
	}
", Data);

Data.GameObjects.ByName("obj_spring").EventHandlerFor(EventType.Collision, plid, Data.Strings, Data.Code, Data.CodeLocals).ReplaceGML(@"
	if (other.vspeed < 0 || other.bbox_bottom - y > 17)
		exit
	other.vspeed = -12
	other.gravity = 0.3
	tAnim = -13
	audio_play_sound(se_switch, 10, false)
	audio_play_sound(se_jump, 10, false)
	instance_create(x, (y - 22), obj_jumpEffectH)
", Data);

Data.GameObjects.ByName("obj_spring2").EventHandlerFor(EventType.Collision, plid, Data.Strings, Data.Code, Data.CodeLocals).ReplaceGML(@"
	if (other.vspeed < 0 && collision_line((x + 5), (y + 18), (x + 26), (y + 19), other.id, true, true)) {
		tAnim = -13
		other.vspeed = 12
		other.gravity = 0.3
		audio_play_sound(se_switch, 10, false)
		audio_play_sound(se_jump, 10, false)
	}
", Data);

Data.GameObjects.ByName("obj_floor").EventHandlerFor(EventType.Collision, plid, Data.Strings, Data.Code, Data.CodeLocals).ReplaceGML(@"
	if (other.vspeed < 0 || other.bbox_bottom - y > 14)
		exit
	with other {
		y = other.y - (bbox_bottom - y) + 1
		vspeed = 0
		gravity = 0
		landedOnce = true
	}
", Data);

void Switch(string obj, bool lands) {
	var code = Decompiler.Decompile(Data.Code.ByName($"gml_Object_{obj}_Collision_65"), dctx).Replace("obj_player.", "other.").Replace("obj_player", "other.id");
	if (lands)
		code += "\nother.landedOnce = true";
	Data.GameObjects.ByName(obj).EventHandlerFor(EventType.Collision, plid, Data.Strings, Data.Code, Data.CodeLocals).ReplaceGML(code, Data);
}

Switch("obj_redSwitch", true);
Switch("obj_redSwitchR", false);
Switch("obj_blueSwitch", true);
Switch("obj_blueSwitchR", false);
Switch("obj_greenSwitch", true);
Switch("obj_greenSwitchR", false);
Switch("obj_yellowSwitch", true);
Switch("obj_yellowSwitchR", false);
Switch("obj_graySwitch", true);
Switch("obj_whiteSwich", true);
Switch("obj_reverseSwitch", false);

Switch("obj_e_blackSwitch", true);
Switch("obj_e_graySwitchR", false);
Switch("obj_e_whiteSwitchR", false);

Data.GameObjects.ByName("obj_turara").EventHandlerFor(EventType.Collision, plid, Data.Strings, Data.Code, Data.CodeLocals).ReplaceGML(@"
	if (kpl)
		e_bonk(x + 16, other, gravity > 0)
	else
		kpl = other.id
", Data);

Data.GameObjects.ByName("obj_onmyoudama0").EventHandlerFor(EventType.Collision, plid, Data.Strings, Data.Code, Data.CodeLocals).ReplaceGML("e_bonk(x + 16, other)", Data);
Data.GameObjects.ByName("obj_onmyoudama3").EventHandlerFor(EventType.Collision, plid, Data.Strings, Data.Code, Data.CodeLocals).ReplaceGML("e_bonk(x + 16, other)", Data);
Data.GameObjects.ByName("obj_obmyoudama4").EventHandlerFor(EventType.Collision, plid, Data.Strings, Data.Code, Data.CodeLocals).ReplaceGML("e_bonk(x + 16, other)", Data);
Data.GameObjects.ByName("obj_e_onmyoudamaCrawl").EventHandlerFor(EventType.Collision, plid, Data.Strings, Data.Code, Data.CodeLocals).ReplaceGML("e_bonk(x + 16, other)", Data);
Data.GameObjects.ByName("obj_bullet1").EventHandlerFor(EventType.Collision, plid, Data.Strings, Data.Code, Data.CodeLocals).ReplaceGML("e_bonk(x, other, vspeed > 0)\ninstance_destroy()", Data);

Data.GameObjects.ByName("obj_magicSquareB").EventHandlerFor(EventType.Collision, plid, Data.Strings, Data.Code, Data.CodeLocals).ReplaceGML(@"
	if (other.warpTimer < 0 || other.warpHash != x % 256 + y / 32 % 32) && other.id == e_s_base(other.id) {
		other.x = r0
		other.y = r1
		other.warpHash = r0 % 256 + r1 / 32 % 32
		audio_play_sound(se_warp, 30, false)
	}

	other.warpTimer = 1;
", Data);

Data.Sprites.ByName("spr_switchBlockWhiteOn").OriginY = 32;

Data.GameObjects.ByName("obj_block22").EventHandlerFor(EventType.Collision, 152, Data.Strings, Data.Code, Data.CodeLocals).ReplaceGML("instance_destroy(other)\ninstance_destroy()", Data);

var rumiaSprite = Data.Sprites.ByName("spr_rumiaidle");
var rumiaBottom = new UndertaleSprite() {
	Name = Data.Strings.MakeString("spr_e_rumiaL"),
	Width = rumiaSprite.Width,
	Height = rumiaSprite.Height,
	MarginLeft = rumiaSprite.MarginLeft,
	MarginRight = rumiaSprite.MarginRight,
	MarginTop = rumiaSprite.MarginTop,
	MarginBottom = rumiaSprite.MarginBottom,
};

foreach (var te in rumiaSprite.Textures) {
	var tex = te.Texture;
	var tpi = new UndertaleTexturePageItem() {
		SourceX = tex.SourceX,
		SourceY = (ushort)(tex.SourceY + 22),
		SourceWidth = tex.SourceWidth,
		SourceHeight = (ushort)(tex.SourceHeight - 22),
		TargetX = tex.TargetX,
		TargetY = (ushort)(tex.TargetY + 22),
		TargetWidth = tex.TargetWidth,
		TargetHeight = (ushort)(tex.TargetHeight - 22),
		BoundingWidth = tex.BoundingWidth,
		BoundingHeight = tex.BoundingHeight,
		TexturePage = tex.TexturePage
	};
	Data.TexturePageItems.Add(tpi);
	rumiaBottom.Textures.Add(new UndertaleSprite.TextureEntry() { Texture = tpi });
}

Data.Sprites.Add(rumiaBottom);

Data.Code.ByName("gml_Object_obj_player_Step_0").ReplaceGML(File.ReadAllText("player.step.gml"), Data);
Data.Code.ByName("gml_Object_obj_player_Draw_0").ReplaceGML(File.ReadAllText("player.draw.gml"), Data);

Data.GameObjects.ByName("obj_gameMgr").EventHandlerFor(EventType.Step, EventSubtypeStep.EndStep, Data.Strings, Data.Code, Data.CodeLocals).ReplaceGML("i_canRead=true", Data);

void KeyBlock(string obj, string colour) {
	var code = "";

	for (int i = 85; i <= 90; i++) {
		code += @$"
			if collision_rectangle(x, y, x + 32, y + 32, {i}, true, true) != noone && obj_gameMgr.{colour}_key > 0 {{
				obj_gameMgr.{colour}_key -= 1
				audio_play_sound(se_unlock, 10, false)
				instance_destroy()
			}}
		";
	}

	Data.Code.ByName($"gml_Object_{obj}_Step_0").AppendGML(code, Data);
}

OnIceBullet("obj_iceWall1");
OnIceBullet("obj_bounceBlock");
OnIceBullet("obj_block22");
KeyBlock("obj_keyblock1", "yellow");
KeyBlock("obj_keyblock2", "red");
KeyBlock("obj_keyblock3", "green");
KeyBlock("obj_keyblock4", "blue");
OnIceBullet("obj_dawkSwtich");

Data.Code.ByName("gml_Object_obj_debugBlock_Step_0").ReplaceGML(@"
if instance_exists(obj_player)
{
	if (obj_player.dir == 1)
		x = floor(obj_player.x / 32) * 32 - 32
	else if (obj_player.dir == 0)
		x = floor(obj_player.x / 32) * 32 + 64
	y = floor(obj_player.y / 32) * 32
}
if (a == 0)
	alphaFlag = 0
else if (a == 1)
	alphaFlag = 1
if (alphaFlag == 0)
	a += 0.05
else if (alphaFlag == 1)
	a -= 0.05
", Data);

Data.Code.ByName("gml_Object_obj_onmyoudama0_Collision_65").ReplaceGML("e_kill_player(true)", Data);
Data.Code.ByName("gml_Object_obj_onmyoudama3_Collision_65").ReplaceGML("e_kill_player(true)", Data);
Data.Code.ByName("gml_Object_obj_obmyoudama4_Collision_65").ReplaceGML("e_kill_player(true)", Data);
Data.Code.ByName("gml_Object_obj_bullet1_Collision_65").ReplaceGML("e_kill_player(!instance_exists(obj_e_precipitator))", Data);

Data.GameObjects.ByName("obj_block22").EventHandlerFor(EventType.Collision, 101, Data.Strings, Data.Code, Data.CodeLocals).ReplaceGML("instance_destroy()", Data);
Data.GameObjects.ByName("obj_iceWall1").EventHandlerFor(EventType.Collision, 101, Data.Strings, Data.Code, Data.CodeLocals).ReplaceGML("instance_destroy()", Data);
Data.GameObjects.ByName("obj_bounceBlock").EventHandlerFor(EventType.Collision, 101, Data.Strings, Data.Code, Data.CodeLocals).ReplaceGML("instance_destroy()", Data);
Data.GameObjects.ByName("obj_iceWall1").EventHandlerFor(EventType.Collision, 1467, Data.Strings, Data.Code, Data.CodeLocals).ReplaceGML("instance_destroy()", Data);
Data.GameObjects.ByName("obj_bounceBlock").EventHandlerFor(EventType.Collision, 1467, Data.Strings, Data.Code, Data.CodeLocals).ReplaceGML("instance_destroy()", Data);
Data.GameObjects.ByName("obj_block22").EventHandlerFor(EventType.Collision, 1469, Data.Strings, Data.Code, Data.CodeLocals).ReplaceGML("instance_destroy()", Data);
Data.GameObjects.ByName("obj_iceWall1").EventHandlerFor(EventType.Collision, 1469, Data.Strings, Data.Code, Data.CodeLocals).ReplaceGML("instance_destroy()", Data);
Data.GameObjects.ByName("obj_bounceBlock").EventHandlerFor(EventType.Collision, 1469, Data.Strings, Data.Code, Data.CodeLocals).ReplaceGML("instance_destroy()", Data);

Data.GameObjects.ByName("obj_block2").EventHandlerFor(EventType.Collision, 1467, Data.Strings, Data.Code, Data.CodeLocals).ReplaceGML("destroyTimer = true", Data);
Data.GameObjects.ByName("obj_block22").EventHandlerFor(EventType.Collision, 1467, Data.Strings, Data.Code, Data.CodeLocals).ReplaceGML("destroyTimer = true", Data);

Data.Code.ByName("gml_Object_obj_seijaUI_Create_0").ReplaceGML(@"
	a = 0
	x = 480
	y = 270
	decide = 0
	Timer = 0
	if instance_exists(obj_gameMgr) {
		if (obj_gameMgr.seijaCan & 1) instance_create(x, y, obj_seijaUILeft)
		if ((obj_gameMgr.seijaCan & 2) > 0) instance_create(x, y, obj_seijaUIRight)
		if ((obj_gameMgr.seijaCan & 4) > 0) instance_create(x, y, obj_seijaUIUp)
		if ((obj_gameMgr.seijaCan & 8) > 0) instance_create(x, y, obj_seijaUIDown)
		obj_gameMgr.playinput = 0
	}
", Data);

Data.Code.ByName("gml_Object_obj_goal_Collision_65").AppendGML(@"
	if instance_exists(obj_e_precipitator) {
		newRecord = 0
		global.minutes = floor(obj_gameMgr.frameTime / 3600)
		global.seconds = obj_gameMgr.frameTime % 3600 / 60
		if !global.nineHeadMode {
			if (obj_gameMgr.frameTime < global.custom_wr)
				newRecord = 2
			else if (obj_gameMgr.frameTime < global.custom_pb)
				newRecord = 1
		}
	}
	e_tas_end(game_save_id)
	if (obj_gameMgr.peace_get)
		e_send_event(249)
	else
		e_send_event(250)
	e_send_real(obj_gameMgr.frameTime)
", Data);

Data.Code.ByName("gml_Object_obj_playStartMgr_Step_0").ReplaceGML(@"
	if !instance_exists(obj_player) {
		instance_destroy()
		exit
	}
	if (obj_player.vspeed < 0)
		canGo = true
	if (obj_gameMgr.i_l || obj_gameMgr.i_r) && (obj_player.floorCheck == 0 || canGo) {
		obj_gameMgr.playinput = true
		global.count_up = true
		instance_destroy()
	}
	Timer += 1
	if (Timer == 120)
		instance_destroy()
", Data);

foreach (var code in Data.Code)
	if (code.Name.Content.Contains("Save_Destroy_0"))
		ReplaceTextInGML(code.Name.Content, "steam_upload_score", "if (e_tas_active() == 0) steam_upload_score");

Data.Code.ByName("gml_Object_obj_head_Collision_65").AppendGML("if spring < 3 {\nlastBounce = obj_gameMgr.frameTime\nif other.pickedUp == id && instance_exists(obj_e_watch_srt)\nobj_e_watch_srt.flag = true\n}", Data);
Data.Code.ByName("gml_Object_obj_head_Create_0").AppendGML("lastBounce = -4\nstuckTime = 4 * instance_exists(obj_e_watch_srt)\nx_p = x\ny_p = y", Data);
Data.Code.ByName("gml_Object_obj_player_Create_0").AppendGML("pickedUp = noone", Data);
Data.Code.ByName("gml_Object_obj_head_Step_0").AppendGML(@"
	if stuckTime > 0 {
		if (x == x_p && y == y_p && !place_free(x, y) && floor((y + 16) / 32) != floor(bbox_top / 32) && place_free(x, 32 * floor((y + 16) / 32) - (bbox_top - y))) {
			stuckTime += 1
			if (stuckTime > 8)
				obj_e_watch_srt.flag = true
		}
		else
			stuckTime -= 1
		x_p = x
		y_p = y
	}
", Data);

Data.GameObjects.ByName("obj_bullet1").EventHandlerFor(EventType.Step, EventSubtypeStep.EndStep, Data.Strings, Data.Code, Data.CodeLocals).AppendGML("image_alpha=1", Data);
