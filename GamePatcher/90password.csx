
string before = "else if (global.password == \"KAGERO\")";

ReplaceTextInGML("gml_Script_scr_password", before, @"

else if (global.password == ""GAMESELECT"")
{
	room_goto(rm_titleSelect)
}

else if (global.password == ""FLANGAME"")
{
	instance_create(x, y, obj_secret_fadeE)
    obj_secretTextMgr.textInput = 0
    global.secretCreate = 1339
    global.secretStage = rm_flanGame
    audio_play_sound(se_goal_get, 30, false)
    global.passwordSucces = 1
}

else if (global.password == ""RASOBI"")
{
	instance_create(x, y, obj_secret_fadeE)
    obj_secretTextMgr.textInput = 0
    global.secretCreate = 1339
    global.secretStage = rm_RASOBI
	global.character = ""rumia""
    audio_play_sound(se_goal_get, 30, false)
    global.passwordSucces = 1
}

else if (global.password == ""RM003"")
{
	instance_create(x, y, obj_secret_fadeE)
    obj_secretTextMgr.textInput = 0
    global.secretCreate = 1339
    global.secretStage = rm_003
    audio_play_sound(se_goal_get, 30, false)
    global.passwordSucces = 1
}

else if (global.password == ""LVLEDITOR"")
{
	instance_create(x, y, obj_secret_fadeE)
    obj_secretTextMgr.textInput = 0
    global.secretCreate = 1339
    global.secretStage = rm_editor
    audio_play_sound(se_goal_get, 30, false)
    global.passwordSucces = 1
}

" + before); 
