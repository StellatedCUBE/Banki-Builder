var dctx = new GlobalDecompileContext(Data, false);

void Patch(string name) {
	var code = Data.Code.ByName(name);
	var gml = Decompiler.Decompile(code, dctx);
	var ngml = "";

	while (gml.Contains("font_add_sprite")) {
		int i = gml.IndexOf("font_add_sprite");
		ngml += gml.Substring(0, i);
		ngml += "(global.dynfont_";
		while (gml[i] != '(')
			i++;
		while (gml[++i] != ',')
			ngml += gml[i];
		while (gml[i] != ')')
			i++;
		gml = gml.Substring(i);
	}

	code.ReplaceGML(ngml + gml, Data);
}

Patch("gml_Object_obj_cirnoStageSelectMenuMgr_Create_0");
Patch("gml_Object_obj_menu_Create_0");
Patch("gml_Object_obj_cirnoGoalMenuMgr_Create_0");
Patch("gml_Object_obj_titleNewVersion_Create_0");
Patch("gml_Object_obj_savaDataSelectMgr_Create_0");
Patch("gml_Object_obj_seijaMenuTimeUI_Create_0");
Patch("gml_Object_obj_characterMenuMgr_Create_0");
Patch("gml_Object_obj_cirnoTitleMenuMgr_Create_0");
Patch("gml_Object_obj_pauseBG_Create_0");
Patch("gml_Object_obj_titleMenuMgr_Create_0");
Patch("gml_Object_obj_recordMgr_Create_0");
Patch("gml_Object_obj_headAddGUI_Create_0");
Patch("gml_Object_obj_pauseMenuMgr_Create_0");
Patch("gml_Object_obj_kaisouMenuMgr_Create_0");
Patch("gml_Object_obj_rumiaStageSelectMenuMgr_Create_0");
Patch("gml_Object_obj_optionMenuMgr_Create_0");
Patch("gml_Object_obj_seijaTitleMenuMgr_Create_0");
Patch("gml_Object_obj_gameMgr_Create_0");
Patch("gml_Object_obj_langMenuMgr_Create_0");
Patch("gml_Object_obj_mapMenuMgr_Create_0");
Patch("gml_Object_obj_rumiaTitleMenuMgr_Create_0");
Patch("gml_Object_obj_siyuuMenuMgr_Create_0");
Patch("gml_Object_obj_achievementUIMgr_Create_0");
Patch("gml_Object_obj_recordSelectMgr_Create_0");
Patch("gml_Object_obj_osyareMenuMgr_Create_0");
Patch("gml_Object_obj_staffrollBG_Create_0");
Patch("gml_Object_obj_minigameRetryMenu_Create_0");
Patch("gml_Object_obj_entrancePauseMenuMgr_Create_0");
Patch("gml_Object_obj_minigame1Mgr_Create_0");
Patch("gml_Object_obj_secretTextMgr_Create_0");
Patch("gml_Object_obj_seijaStageSelectMenuMgr_Create_0");
Patch("gml_Object_obj_retryMenuMgr_Create_0");
Patch("gml_Object_obj_cirnoMenuTimeUI_Create_0");
Patch("gml_Object_obj_introSkipMenuMgr_Create_0");
Patch("gml_Object_obj_recollectionMenuMgr_Create_0");

Data.Code.ByName("gml_Room_rm_boot_Create").AppendGML(@"
	global.dynfont_spr_font_number = global.font
	global.dynfont_spr_font_small = global.font_small
	global.dynfont_spr_font_small2 = font_add_sprite_ext(spr_font_small2, ""0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ.()[]#$&'!?:_{}@<-"", 0, 0)
	global.dynfont_spr_achievementIcon = font_add_sprite_ext(spr_achievementIcon, ""0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ()[]"", 0, 0)
	global.dynfont_spr_font_number2 = font_add_sprite_ext(spr_font_number2, ""0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ.()[]#$&'!?:_{}@<-/|"", 0, 0)
	e_onload(game_save_id)
", Data);
