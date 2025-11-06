
// Adds the "ilovebeer" cheat code

MakeScript("string_ends_w", @"

var str = argument0
var substr = argument1
if (string_length(str) < string_length(substr))
    return 0;
var i = ((string_length(str) - string_length(substr)) + 1)
for (var j = 1; j <= string_length(substr); j += 1)
{
    if (string_ord_at(str, i) != string_ord_at(substr, j))
        return 0;
    i += 1
}
return 1;

", 2, 5);

Data.GameObjects.ByName("obj_doremy").EventHandlerFor(EventType.Create, Data.Strings, Data.Code, Data.CodeLocals).AppendGML("\ntext_typed=\"\"\n", Data);

Data.GameObjects.ByName("obj_doremy").EventHandlerFor(EventType.Step, Data.Strings, Data.Code, Data.CodeLocals).AppendGML(@"

var key = keyboard_lastchar
if (!(string_ends_w(text_typed, key)))
{
    text_typed += key
    if string_ends_w(string_lower(text_typed), ""iloveber"")
    {
        ini_open(global.savedata)
        for (var i = 1; i < 11; i++)
        {
            wn = ""T""
            if (i > 1)
                wn = string(i)
            ini_write_real(((""stageTime_"" + wn) + ""01_9""), ""clearCount"", 1)
            ini_write_real(((""stageTime_"" + wn) + ""02_9""), ""clearCount"", 1)
            ini_write_real(((""stageTime_"" + wn) + ""03_9""), ""clearCount"", 1)
            ini_write_real(((""stageTime_"" + wn) + ""04_9""), ""clearCount"", 1)
            ini_write_real(((""stageTime_"" + wn) + ""05_9""), ""clearCount"", 1)
            ini_write_real(((""stageTime_"" + wn) + ""EX_9""), ""clearCount"", 1)
            ini_write_real(((""Stage"" + string(i)) + ""EndFlag""), ""Flag"", 1)
        }
        ini_close()
        instance_destroy(obj_stageTimeLoadEn.id)
        instance_create_depth(0, 0, 0, obj_stageTimeLoadEn)
        obj_doorSecret.clearFlag = 1
    }
	if string_ends_w(string_lower(text_typed), ""fjdedit"")
		room_goto(rm_editor)
    if (string_length(text_typed) > 16)
        text_typed = string_delete(text_typed, 1, 4)
}

", Data);
