
Console.WriteLine($"OBJECT BASE: {Data.GameObjects.Count}");

UndertaleGameObject obj, editor_obj;
Data.GameObjects.Add(obj = editor_obj = new UndertaleGameObject() {
	Name = Data.Strings.MakeString("obj_editor"),
	Visible = false
});

obj.EventHandlerFor(EventType.Create, Data.Strings, Data.Code, Data.CodeLocals).ReplaceGML(@"

e_send_event(0)
e_handle_all_queue()
m_x = -1
m_y = -1
vx = -1
vy = -1
wf = true
ife = -1

for (var i = 0; i < 8; i += 1)
{
	keys[i] = 0
	if (i < 4)
		md[i] = 2
}

", Data);

obj.EventHandlerFor(EventType.Step, EventSubtypeStep.EndStep, Data.Strings, Data.Code, Data.CodeLocals).ReplaceGML(@"

//var vm = 0
//var mm;

//if (vx != (view_xview[0]) || vy != (view_yview[0]))
//{
//	vm = (vx - view_xview[0]) * (vx - view_xview[0]) + (vy - view_yview[0]) * (vy - view_yview[0])
//	vx = view_xview[0]
//	vy = view_yview[0]
//}

if (m_x != mouse_x || m_y != mouse_y)
{
	e_send_event(1)
	e_send_real(mouse_x)
	e_send_real(mouse_y)
//	mm = (m_x - mouse_x) * (m_x - mouse_x) + (m_y - mouse_y) * (m_y - mouse_y)
	m_x = mouse_x
	m_y = mouse_y

	//e_xlog(string(vm) + "" "" + string(mm))

//	if mm > vm + 1
//		e_send_event(4)
}

var d;
var i;
for (i = 1; i < 4; i += 1)
{
	d = device_mouse_check_button(0, i);
	if (md[i] != d)
	{
		md[i] = d
		e_send_event(2 + d)
		e_send_event(i)
	}
}

if (global.gamePad >= 0)
{
	for (d = 32769; d < 32785; d += 1)
	{
		if gamepad_button_check(global.gamePad, d)
		{
			e_send_event(5)
			e_send_event(d - 32768)
		}
		if gamepad_button_check_released(global.gamePad, d)
		{
			e_send_event(6)
			e_send_event(d - 32768)
		}
	}
}

for (d = 0; d < 8; d += 1)
{
	if (keys[d] && keyboard_check_released(keys[d]))
	{
		e_send_event(8)
		e_send_real(keys[d])
		keys[d] = 0
	}
}

if (keyboard_lastkey != vk_nokey && !keyboard_check_released(keyboard_lastkey))
{
	for (d = 0; d < 8; d += 1)
	{
		if (keys[d] == keyboard_lastkey)
			break

		if (d == 7)
		{
			for (i = 0; i < 8; i += 1)
			{
				if (keys[i] == 0)
				{
					keys[i] = keyboard_lastkey
					e_send_event(7)
					e_send_real(keyboard_lastkey)
					break
				}
			}
		}
	}
	keyboard_lastkey = vk_nokey
}

if (ife >= 0)
{
	e_send_event(131)
	e_send_event(ife)
	ife = -1
}

if (wf != window_has_focus())
{
	wf = window_has_focus()
	if wf
		e_send_event(4)
	else
		e_send_event(9)
}

e_handle_all_queue()

", Data);

// === CURSOR ===

Data.GameObjects.Add(obj = new UndertaleGameObject() {
	Name = Data.Strings.MakeString("obj_e_cursor"),
	Visible = true
});

obj.EventHandlerFor(EventType.Create, Data.Strings, Data.Code, Data.CodeLocals).ReplaceGML(@"

r0 = 0
r1 = 0
image_speed = 0

", Data);

obj.EventHandlerFor(EventType.Draw, Data.Strings, Data.Code, Data.CodeLocals).ReplaceGML(@"

x = mouse_x + r0
y = mouse_y + r1
if (sprite_index >= 0)
	draw_self()

", Data);

// === TEXTBOX ===

Data.GameObjects.Add(obj = new UndertaleGameObject() {
	Name = Data.Strings.MakeString("obj_e_textbox"),
	Visible = true
});

obj.EventHandlerFor(EventType.Create, Data.Strings, Data.Code, Data.CodeLocals).ReplaceGML(@"

s0 = """"
post_carat = """"

focused = false
r0 = false
r1 = 1024
r2 = 480
r3 = 0
r4 = 0
r5 = 360
s = 1

font = global.font_ch

", Data);

obj.EventHandlerFor(EventType.Destroy, Data.Strings, Data.Code, Data.CodeLocals).ReplaceGML("if (focused)\nobj_editor.ife = r3", Data);

obj.EventHandlerFor(EventType.Step, Data.Strings, Data.Code, Data.CodeLocals).ReplaceGML(@"

if r4 == 1 {
	r4 = 0
	s0 = e_steam_name()
}

if r4 == 2 {
	r4 = 0
	font = global.font
}

if (r0 && !focused)
{
	focused = true
	keyboard_string = s0
	post_carat = """"
	keyboard_check_pressed(vk_left)
	keyboard_check_pressed(vk_right)
	keyboard_check_pressed(vk_escape)
	keyboard_check_pressed(vk_enter)
	keyboard_check_pressed(vk_delete)
	keyboard_check_pressed(vk_home)
	keyboard_check_pressed(vk_end)

	if (r3 != 2)
		r3 = 0
	
	e_send_event(130)
}

if (!r0 && focused)
{
	focused = false
	obj_editor.ife = r3
}

draw_set_font(font)
if (string_width(s0) > r5)
	s = 0.5
else
	s = 1

if focused
{
	if font == global.font && s0 != keyboard_string + post_carat {
		var i = 1;
		var nks = """";
		while i <= string_length(keyboard_string) {
			var nc = string_upper(string_copy(keyboard_string, i, 1))
			if (string_pos(nc, ""1234567890-QWERTYUIOPASDFGHJKLZXCVBNM"") != 0)
				nks += nc
			i += 1
		}
		keyboard_string = nks
	}

	s0 = keyboard_string + post_carat
	while (string_length(s0) > r1 || string_width(s0) > r2)
	{
		keyboard_string = string_copy(keyboard_string, 1, string_length(keyboard_string) - 1)
		s0 = keyboard_string + post_carat
	}

	if (r3 == 2)
	{
		r3 = 0
		if (mouse_x <= x)
		{
			keyboard_string = """"
			post_carat = s0
		}
		else if (mouse_x >= x + s * string_width(s0))
		{
			keyboard_string = s0
			post_carat = """"
		}
		else
		{
			var i = 0
			var dist = r2

			while (i < string_length(s0))
			{
				var char_x = x + s * string_width(string_copy(s0, 1, i))
				var new_dist = abs(mouse_x - char_x)
				if (new_dist < dist)
					dist = new_dist
				else
					break
				i += 1
			}

			if (i > 0)
				i -= 1

			keyboard_string = string_copy(s0, 1, i)
			post_carat = string_copy(s0, i + 1, string_length(s0) - 1)
		}
	}

	if (keyboard_check_pressed(vk_left) && string_length(keyboard_string) > 0)
	{
		post_carat = string_copy(keyboard_string, string_length(keyboard_string), 1) + post_carat
		keyboard_string = string_copy(keyboard_string, 1, string_length(keyboard_string) - 1)
	}

	if (keyboard_check_pressed(vk_right) && string_length(post_carat) > 0)
	{
		keyboard_string += string_copy(post_carat, 1, 1)
		post_carat = string_copy(post_carat, 2, string_length(post_carat) - 1)
	}

	if keyboard_check_pressed(vk_home)
	{
		keyboard_string = """"
		post_carat = s0
	}

	if keyboard_check_pressed(vk_end)
	{
		keyboard_string = s0
		post_carat = """"
	}

	if keyboard_check_pressed(vk_enter)
	{
		r3 = 3
	}

	if (r3 == 3 || keyboard_check_pressed(vk_escape))
	{
		focused = false
		r0 = false
		if (r3 == 0)
			r3 = 4
		obj_editor.ife = r3
	}

	if (keyboard_check_pressed(vk_delete) && string_length(post_carat) > 0)
	{
		post_carat = string_copy(post_carat, 2, string_length(post_carat) - 1)
	}

	s0 = keyboard_string + post_carat
}

", Data);

obj.EventHandlerFor(EventType.Draw, Data.Strings, Data.Code, Data.CodeLocals).ReplaceGML(@"

draw_set_font(font)
draw_set_halign(fa_left)
draw_set_valign(fa_middle)
if (font == global.font)
	draw_set_colour(c_white)
else
	draw_set_colour(c_black)
draw_text_transformed(x, y, s0, s, s, 0)

if focused
{
	var cx = x + s * string_width(keyboard_string)
	if (font == global.font) {
		draw_set_colour(c_black)
		draw_line_width(cx, y - s * 12, cx, y + s * 8, 2)
	} else
		draw_line_width(cx, y - s * 14, cx, y + s * 10, 1)
}

draw_set_valign(0)
", Data);

// === EDITOR (ACTUAL) ===

Data.GameObjects.Add(obj = new UndertaleGameObject() {
	Name = Data.Strings.MakeString("obj_e_editor_actual"),
	Visible = true
});

obj.EventHandlerFor(EventType.Create, Data.Strings, Data.Code, Data.CodeLocals).ReplaceGML(@"

r0 = false

pin = false
px = 0
py = 0

lsvx = -0.5
lsvy = -0.5

", Data);

obj.EventHandlerFor(EventType.Step, Data.Strings, Data.Code, Data.CodeLocals).ReplaceGML(@"

if r0
{
	if pin
	{
		view_xview[0] -= mouse_x - px
		view_yview[0] -= mouse_y - py
	}
	else
	{
		pin = true
		px = mouse_x
		py = mouse_y
	}
}
else
{
	pin = false
}

if (view_xview[0] != lsvx || view_yview[0] != lsvy)
{
	lsvx = view_xview[0]
	lsvy = view_yview[0]
	e_send_event(252)
	e_send_real(lsvx)
	e_send_real(lsvy)
}

", Data);

obj.EventHandlerFor(EventType.Draw, Data.Strings, Data.Code, Data.CodeLocals).ReplaceGML(@"

var w = view_wview[0] / 480
var p 
var e = view_xview[0] + view_wview[0]
var a = view_yview[0] - 1
var b = a + view_hview[0] + 2

draw_set_colour(c_black)
draw_set_alpha(0.4)

if (w < 2)
	p = (view_xview[0] & ~31) - 1
else
	p = 480 * floor(view_xview[0] / 480) - 480

while (p < e)
{
	draw_line_width(p, a, p, b, w)
	if (w < 2)
		p += 32
	else
		p += 480
}

if (w < 2)
	p = (a & ~31) - 1
else
	p = 270 * floor(view_yview[0] / 270) - 270

e = b
a = view_xview[0] - 1
b = a + view_wview[0] + 2

while (p < e)
{
	draw_line_width(a, p, b, p, w)
	if (w < 2)
		p += 32
	else
		p += 270
}

draw_set_alpha(1);

", Data);

// === NO ANIMATION ===

Data.GameObjects.Add(obj = new UndertaleGameObject() {
	Name = Data.Strings.MakeString("obj_e_no_anim"),
	Visible = true
});

obj.EventHandlerFor(EventType.Create, Data.Strings, Data.Code, Data.CodeLocals).ReplaceGML("image_speed=0", Data);

// === STAGE START MANAGER ===

Data.GameObjects.Add(obj = new UndertaleGameObject() {
	Name = Data.Strings.MakeString("obj_e_stageStartMgr"),
	Visible = true
});

obj.EventHandlerFor(EventType.Create, Data.Strings, Data.Code, Data.CodeLocals).ReplaceGML(@"

if (global.quickRetry == 0)
{
	instance_create(960, 0, obj_stageStartBar1)
	instance_create(960, 0, obj_stageStartBar2)
	//instance_create(0, 0, obj_act1)
}
stageStartTimer = 0
lsvx = 0.5
lsvy = 0
s0 = """"
s1 = """"
y0 = 960
y1 = 960
ts = 0

", Data);

obj.EventHandlerFor(EventType.Step, Data.Strings, Data.Code, Data.CodeLocals).ReplaceGML(@"

if instance_exists(obj_gameMgr) {
	stageStartTimer += 1

	if (stageStartTimer == 120) {
		obj_gameMgr.playinput = 1
		global.count_up = 1
	}
}

with obj_entranceBackMgr {
	if (resetTimer == 1)
		e_send_event(251)
	else if (resetTimer == 118)
		room_restart()
}

with obj_rumiaBackMenuMgr {
	if (resetTimer == 1)
		e_send_event(251)
	else if (resetTimer == 118)
		room_restart()
}

if (view_xview[0] != lsvx || view_yview[0] != lsvy)
{
	lsvx = view_xview[0]
	lsvy = view_yview[0]
	e_send_event(252)
	e_send_real(lsvx)
	e_send_real(lsvy)
}

y0 += (398 - y0) * 0.1
y1 += (300 - y1) * 0.1
if (stageStartTimer > 150)
	y0 += (820 - y0) * 0.07
if (stageStartTimer > 165)
	y1 += (860 - y1) * 0.07
	

", Data);

obj.EventHandlerFor(EventType.Step, EventSubtypeStep.EndStep, Data.Strings, Data.Code, Data.CodeLocals).ReplaceGML(@"

if instance_exists(obj_camera)
{
	view_xview[0] = median(obj_viewDummyL.x + 33, obj_camera.x - 240, obj_viewDummyR.x - 480)
	view_yview[0] = median(obj_viewDummyU.y + 33, obj_camera.y - 135, obj_viewDummyD.y - 270)
}

", Data);

obj.EventHandlerFor(EventType.Draw, EventSubtypeDraw.DrawGUI, Data.Strings, Data.Code, Data.CodeLocals).ReplaceGML(@"
if stageStartTimer <= 240 && instance_exists(obj_stageStartBar1) {
	draw_set_font(font_message)
	if ts == 0 {
		if (string_width(s1) > 192)
			ts = 1
		else
			ts = 2
	}
	draw_set_colour(c_white)
	draw_set_halign(fa_right)
	draw_set_valign(fa_middle)
	draw_text_transformed(940, y0 + 28, s0, 1, 1, 0)
	draw_text_transformed(940, y1 + 80, s1, ts, ts, 0)
	draw_set_halign(0)
	draw_set_valign(0)
}
", Data);

// === CAMERA FOLLOW ===

Data.GameObjects.Add(obj = new UndertaleGameObject() {
	Name = Data.Strings.MakeString("obj_e_cam_follow"),
	Visible = true
});

obj.EventHandlerFor(EventType.Create, Data.Strings, Data.Code, Data.CodeLocals).ReplaceGML(@"

r0 = 0
r1 = 0
r2 = -1
image_speed = 0

", Data);

obj.EventHandlerFor(EventType.Draw, Data.Strings, Data.Code, Data.CodeLocals).ReplaceGML(@"

var p = r2
var i = r0
var j = r1
var a = 0
while p >= 0
{
	a += 1
	if a > 100 {
		e_xlog(""too deep"")
		break
	}
	with e_get_instance_id(p)
	{
		i += r0
		j += r1
		p = r2
	}
}

x = i + view_xview[0]
y = j + view_yview[0]

if (sprite_index >= 0)
	draw_self()

", Data);

// === TEXT ===

Data.GameObjects.Add(obj = new UndertaleGameObject() {
	Name = Data.Strings.MakeString("obj_e_text"),
	Visible = true
});

obj.EventHandlerFor(EventType.Create, Data.Strings, Data.Code, Data.CodeLocals).ReplaceGML("s0=\"\"\nr0=global.font\nr1=0\nr2=0\nr3=99999\nimage_xscale=0.5\nimage_yscale=0.5\n//s=0.5", Data);

obj.EventHandlerFor(EventType.Draw, Data.Strings, Data.Code, Data.CodeLocals).ReplaceGML(@"

if (r0 < 0)
	r0 = global.font_ch
draw_set_font(r0)

//if (s == 0.5 && r3 < 99999 && r3 < s * string_width(s0))
//	s = 0.25

draw_set_colour(image_blend)
draw_set_halign(r1)
draw_set_valign(r2)
draw_text_transformed(x, y, s0, image_xscale, image_yscale, 0)
draw_set_halign(0)
draw_set_valign(0)

", Data);

// === SELECT BOX ===

Data.GameObjects.Add(obj = new UndertaleGameObject() {
	Name = Data.Strings.MakeString("obj_e_select_box"),
	Visible = true
});

obj.EventHandlerFor(EventType.Create, Data.Strings, Data.Code, Data.CodeLocals).ReplaceGML("r0=0\nr1=0\nr2=0\nr3=0", Data);

obj.EventHandlerFor(EventType.Draw, Data.Strings, Data.Code, Data.CodeLocals).ReplaceGML(@"

var w = view_wview[0] / 480

draw_set_colour(c_yellow)

draw_line_width(r0, r1, r2, r1, w)
draw_line_width(r0, r1, r0, r3, w)
draw_line_width(r2, r1, r2, r3, w)
draw_line_width(r0, r3, r2, r3, w)

", Data);

// === RECTANGLE ===

Data.GameObjects.Add(obj = new UndertaleGameObject() {
	Name = Data.Strings.MakeString("obj_e_rect"),
	Visible = true
});

obj.EventHandlerFor(EventType.Create, Data.Strings, Data.Code, Data.CodeLocals).ReplaceGML("r0=0\nr1=0\nr2=0\nr3=0", Data);

obj.EventHandlerFor(EventType.Draw, Data.Strings, Data.Code, Data.CodeLocals).ReplaceGML(@"

draw_set_colour(r0)
draw_set_alpha(image_alpha)
draw_rectangle(x + r3 * view_xview[0], y + r3 * view_yview[0], x + r1 + r3 * view_xview[0], y + r2 + r3 * view_yview[0], false)
draw_set_alpha(1)

", Data);

// === DETECTOR ===

Data.GameObjects.Add(obj = new UndertaleGameObject() {
	Name = Data.Strings.MakeString("obj_e_detector"),
	Visible = true,
	Sprite = Data.Sprites.ByName("spr_bankiBlock")
});

obj.EventHandlerFor(EventType.Create, Data.Strings, Data.Code, Data.CodeLocals).ReplaceGML("image_speed=0", Data);

Data.GameObjects.Add(obj = new UndertaleGameObject() {
	Name = Data.Strings.MakeString("obj_e_detectorblock_solid"),
	Visible = true,
	Sprite = Data.Sprites.ByName("spr_bankiBlock2"),
	ParentId = Data.GameObjects.ByName("obj_wall3"),
	Depth = 19,
	Solid = true
});

obj.EventHandlerFor(EventType.Create, Data.Strings, Data.Code, Data.CodeLocals).ReplaceGML("image_speed=0\nflip=0", Data);

obj.EventHandlerFor(EventType.Step, EventSubtypeStep.BeginStep, Data.Strings, Data.Code, Data.CodeLocals).ReplaceGML(@"
if flip {
	instance_create(x, y, obj_e_detectorblock_solid + 1)
	instance_destroy()
}
", Data);

Data.GameObjects.Add(obj = new UndertaleGameObject() {
	Name = Data.Strings.MakeString("obj_e_detectorblock_unsolid"),
	Visible = true,
	Sprite = Data.Sprites.ByName("spr_bankiBlockWhite"),
	Depth = 19
});

obj.EventHandlerFor(EventType.Create, Data.Strings, Data.Code, Data.CodeLocals).ReplaceGML("image_speed=0\nflip=0", Data);

obj.EventHandlerFor(EventType.Step, EventSubtypeStep.BeginStep, Data.Strings, Data.Code, Data.CodeLocals).ReplaceGML(@"
if flip {
	instance_create(x, y, obj_e_detectorblock_solid)
	instance_destroy()
}
", Data);

// === PRECIPITATOR ===

Data.GameObjects.Add(obj = new UndertaleGameObject() {
	Name = Data.Strings.MakeString("obj_e_precipitator"),
	Visible = false
});

obj.EventHandlerFor(EventType.Create, Data.Strings, Data.Code, Data.CodeLocals).ReplaceGML("r0=0\nr1=0\nr2=0\nr3=0\nwarpTimer=0\nwarpHash=0\ndetect=0\nrandomize()\nf=true" + @"
	if !(obj_gameMgr.seijaCan % 2) {
		if ((obj_gameMgr.seijaCan & 4) > 0)
			global.seijaItem = 2
		else if ((obj_gameMgr.seijaCan & 8) > 0)
			global.seijaItem = 3
		else if ((obj_gameMgr.seijaCan & 2) > 0)
			global.seijaItem = 1
	}
", Data);

obj.EventHandlerFor(EventType.Step, EventSubtypeStep.BeginStep, Data.Strings, Data.Code, Data.CodeLocals).ReplaceGML(@"

if f {
	f = false
	with instance_create(r4, min(x, y) - 32, obj_cannonBlockEnd)
		image_xscale = (other.r5 - other.r4) / 32 + 2
	with instance_create(r4, max(x, y), obj_cannonBlockEnd)
		image_xscale = (other.r5 - other.r4) / 32 + 2
	with instance_create(r4, min(x, y) - 32, obj_cannonBlockEnd)
		image_yscale = (max(x, y) - min(x, y)) / 32 + 2
	with instance_create(r5, min(x, y) - 32, obj_cannonBlockEnd)
		image_yscale = (max(x, y) - min(x, y)) / 32 + 2
}

var pDetect = detect
if instance_exists(obj_e_detector) {
	detect = false
	with (obj_e_detector) {
		if (
			collision_rectangle((x + 1), (y + 1), (x + 31), (y + 31), obj_head, 1, 1) != noone ||
			collision_rectangle((x + 1), (y + 1), (x + 31), (y + 31), obj_holdHead, 1, 1) != noone ||
			collision_rectangle((x + 1), (y + 1), (x + 31), (y + 31), obj_switchBlock, 1, 1) != noone ||
			collision_rectangle((x + 1), (y + 1), (x + 31), (y + 31), obj_switchBlockBlue, 1, 1) != noone ||
			collision_rectangle((x + 1), (y + 1), (x + 31), (y + 31), obj_switchBlockGreen, 1, 1) != noone ||
			collision_rectangle((x + 1), (y + 1), (x + 31), (y + 31), obj_switchBlockYellow, 1, 1) != noone ||
			collision_rectangle((x + 1), (y + 1), (x + 31), (y + 31), obj_turara, 1, 1) != noone ||
			collision_rectangle((x + 1), (y + 1), (x + 31), (y + 31), 1539, 1, 1) != noone ||
			collision_rectangle((x + 1), (y + 1), (x + 31), (y + 31), obj_darkBomb, 1, 1) != noone ||
			collision_rectangle((x + 1), (y + 1), (x + 31), (y + 31), obj_magicBomb, 1, 1) != noone ||
			collision_rectangle((x + 1), (y + 1), (x + 31), (y + 31), obj_chandelier, 1, 1) != noone
		) {
			other.detect = true
			break
		}
	}
	obj_e_detector.image_index = detect
	if detect != pDetect {
		if instance_exists(obj_e_detectorblock_solid)
			obj_e_detectorblock_solid.flip = true
		if instance_exists(obj_e_detectorblock_unsolid)
			obj_e_detectorblock_unsolid.flip = true
	}
}

with (obj_player)
	if (y > max(other.x, other.y))
		e_kill_player(true)

", Data);

obj.EventHandlerFor(EventType.Step, EventSubtypeStep.EndStep, Data.Strings, Data.Code, Data.CodeLocals).ReplaceGML(@"

warpTimer -= 1

if (random(1) < r0)
	instance_create_depth(random(1440) - 480 + view_xview[0], y, -5000, obj_e_precipitator + 1)

", Data);

// === PRECIPITATION ===

Data.GameObjects.Add(obj = new UndertaleGameObject() {
	Name = Data.Strings.MakeString("obj_e_precipitation"),
	Visible = true
});

obj.EventHandlerFor(EventType.Create, Data.Strings, Data.Code, Data.CodeLocals).ReplaceGML(@"

image_blend = obj_e_precipitator.image_blend
sprite_index = obj_e_precipitator.r1
if (sprite_index == spr_star)
	image_speed = 0.3
else
	image_speed = 0.2

", Data);

obj.EventHandlerFor(EventType.Step, EventSubtypeStep.EndStep, Data.Strings, Data.Code, Data.CodeLocals).ReplaceGML(@"

x += obj_e_precipitator.r2
y += obj_e_precipitator.r3

if (y > obj_e_precipitator.x && obj_e_precipitator.r3 > 0)
	instance_destroy()
if (y < obj_e_precipitator.x && obj_e_precipitator.r3 < 0)
	instance_destroy()

", Data);

// === BG ===

Data.GameObjects.Add(obj = new UndertaleGameObject() {
	Name = Data.Strings.MakeString("obj_e_bg"),
	Visible = false
});

obj.EventHandlerFor(EventType.Create, Data.Strings, Data.Code, Data.CodeLocals).ReplaceGML("r0=-1\nff=true", Data);

obj.EventHandlerFor(EventType.Step, EventSubtypeStep.EndStep, Data.Strings, Data.Code, Data.CodeLocals).ReplaceGML(@"

if (ff && r0 >= 0) {
	ff = false
	switch (r0) {
	case 0: // Dream Fields
		background_colour = 0xffa64c
	break

	case 1: // Bamboo
		background_colour = 0x424332
		if (!instance_exists(obj_e_precipitator) || max(obj_e_precipitator.x, obj_e_precipitator.y) > min(obj_e_precipitator.x, obj_e_precipitator.y) + 480) {
			background_index[3] = background_e_bamboo_59
			background_index[4] = background_e_bamboo_58
			background_index[5] = background_e_bamboo_57
		}
	break

	case 2: // Azure Winter
	case 16: // Cirno
	case 19: // RASOBI
		background_colour = 0xffd5a6
	break

	case 4: // Ultramarine Rain
		background_colour = 0x6b462e
	break

	case 5: // Outside World
		background_colour = 0x3e1a0d
	break

	case 7: // Shining Needle Castle
		background_colour = 0x8f5753
	break

	case 10: // Jerry Attack
		background_colour = 0xaf8dff
	break
	
	case 13: // Reach Moon
		background_colour = 0x4e271e
		if (!instance_exists(obj_e_precipitator) || max(obj_e_precipitator.x, obj_e_precipitator.y) > min(obj_e_precipitator.x, obj_e_precipitator.y) + 480) {
			background_index[3] = background_e_bamboo_48
			background_index[4] = background_e_bamboo_47
			background_index[5] = background_e_bamboo_46
		}
	break
	}
}

switch (r0) {
case 0: // Dream Fields
	e_bg_move(0, 0, -999999, 300, 480)
	e_bg_move(1, 1/ 1.7, -999999, 300, 480)
	e_bg_move(2, 1/ 2, -999999, 300, 480)
	e_bg_move(3, 1/ 2.5, -999999, 300, 480)
	e_bg_move(4, 1/ 3.5, -999999, 300, 480)
	e_bg_move(5, 1/ 2, -999999, 999999, 480)
break

case 1: // Bamboo
case 13: // Reach Moon
	e_bg_move(-1, 1/ 1.3, -999999, 290, 480)
	e_bg_move(2, 1/ 1.6, -999999, 290, 480)
	e_bg_move(3, 1/ 2, -999999, 290, 480)
	e_bg_move(4, 1/ 2.5, -999999, 290, 480)
	e_bg_move(5, 1/ 3.5, -999999, 290, 480)
	e_bg_move(6, 1/ 2, -999999, 290, 480)
break

case 2: // Azure Winter
	e_bg_move(-1, 1/ 1.15, -999999, 300, 480)
	e_bg_move(2, 1/ 1.25, -999999, 300, 480)
	e_bg_move(3, 1/ 1.4, -999999, 300, 480)
	e_bg_move(4, 1/ 1.7, -999999, 300, 480)
	e_bg_move(5, 1/ 2, -999999, 300, 480)
	e_bg_move(6, 1/ 2.5, -999999, 300, 480)
	e_bg_move(7, 1/ 3.5, -999999, 300, 480)
break

case 3: // Forest of Magic
	e_bg_move(0, 1/ 1.1, -999999, 230, 480)
	e_bg_move(1, 1/ 1.15, -999999, 230, 480)
	e_bg_move(2, 1/ 1.25, -999999, 230, 480)
	e_bg_move(3, 1/ 1.4, -999999, 230, 480)
	e_bg_move(4, 1/ 1.7, -999999, 230, 480)
	e_bg_move(5, 1/ 2, -999999, 230, 480)
	e_bg_move(6, 1/ 2.5, -999999, 230, 480)
	e_bg_move(7, 1/ 3.5, -999999, 230, 480)
	background_y[7] = min(view_yview[0] - 1, background_y[7] - 191)
break

case 4: // Ultramarine Rain
	e_bg_move(-1, 1/ 1.1, 0, 999999, -1)
	e_bg_move(2, 1/ 1.2, 0, 999999, -1)
	e_bg_move(3, 1/ 1.3, 0, 999999, -1)
break

case 5: // Outside World
	e_bg_move(-1, 1/ 1.3, -999999, 380, 640)
	e_bg_move(2, 1/ 1.7, -999999, 380, 640)
	e_bg_move(3, 1/ 2, -999999, 380, 640)
	e_bg_move(4, 1/ 2.5, -999999, 380, 640)
break

case 7: // Shining Needle Castle
	e_bg_move(-1, 1/ 1.1, -999999, 1000, 1184)
	e_bg_move(2, 1/ 1.15, -999999, 1000, 1184)
	e_bg_move(3, 1/ 1.2, -999999, 1000, 1184)
	e_bg_move(4, 1/ 2, -999999, 999999, 1184)
break

case 10: // Jerry Attack
	e_bg_move(-1, 1/ 1.3, -999999, 450, 720)
	e_bg_move(2, 1/ 1.7, -999999, 450, 720)
	e_bg_move(3, 1/ 2, -999999, 450, 720)
	e_bg_move(4, 1/ 2.5, -999999, 450, 720)
break

case 11: // Faraway Labyrinth
case 14: // Mind Break
	if (view_yview[0] < 0)
		background_colour = 0x974
	else
		background_colour = 0x40412
	e_bg_move(-1, 1/ 1.3, -999999, 999999, 0)
	e_bg_move(2, 1/ 1.5, -999999, 999999, 0)
break

case 12: // Dancing Stars
	if (view_yview[0] < 0)
		background_colour = 0xf0d0d
	else
		background_colour = 0x431a19
	e_bg_move(-1, 1/ 1.3, -999999, 999999, 0)
break

case 15: // Fireflies
	if (view_yview[0] < 0)
		background_colour = 0x4e271e
	else
		background_colour = 0x985839
	e_bg_move(-1, 1/ 1.3, -999999, 999999, 0)
break

case 16: // Cirno
case 19: // RASOBI
	e_bg_move(-1, 1/ 1.3, -999999, 500, 540)
break

case 17: // Rumia
	if (view_yview[0] < 0)
		background_colour = 0x2f2f36
	else
		background_colour = 0x958988
	e_bg_move(-1, 1/ 1.3, -999999, 999999, 0)
break

case 20: // Purple
	if (view_yview[0] < 0)
		background_colour = 0x290e16
	else
		background_colour = 0xb42871
	e_bg_move(-1, 1/ 1.3, -999999, 999999, 0)
break
}

", Data);

Data.GameObjects.ByName("obj_bgS").EventHandlerFor(EventType.Create, Data.Strings, Data.Code, Data.CodeLocals).ReplaceGML("background_color = 0xf3cdac", Data);

// === SEIJA CAMERA ===

Data.GameObjects.Add(obj = new UndertaleGameObject() {
	Name = Data.Strings.MakeString("obj_e_seijaCamera"),
	Visible = true,
	Sprite = Data.Sprites.ByName("spr_seijaCamera"),
});

obj.EventHandlerFor(EventType.Create, Data.Strings, Data.Code, Data.CodeLocals).ReplaceGML("a=0", Data);

obj.EventHandlerFor(EventType.Step, EventSubtypeStep.EndStep, Data.Strings, Data.Code, Data.CodeLocals).ReplaceGML(@"
if !instance_exists(obj_player) {
	instance_destroy()
	exit
}

a += 0.02
if (a > 1)
	a -= 2

x = floor(obj_player.x / 32 + 2 - 5 * obj_player.dir) * 32
y = floor(obj_player.y / 32 - 1) * 32
visible = (global.seijaItem == 1 && (obj_gameMgr.seijaCan & 2) > 0)
", Data);

obj.EventHandlerFor(EventType.Draw, Data.Strings, Data.Code, Data.CodeLocals).ReplaceGML("draw_sprite_ext(spr_seijaCamera, global.seijaCamera, x, y, 1, 1, 0, c_white, abs(a))", Data);

// === MOVING PLATFORM ===

Data.GameObjects.Add(obj = new UndertaleGameObject() {
	Name = Data.Strings.MakeString("obj_e_floorMove"),
	Visible = true,
	ParentId = Data.GameObjects.ByName("obj_floor"),
	Sprite = Data.Sprites.ByName("spr_floorMove")
});

obj.EventHandlerFor(EventType.Create, Data.Strings, Data.Code, Data.CodeLocals).ReplaceGML(@"
	o0=0
	o1=0
	o2=0
	o3=0
	p_ind = 0
	xs=true
", Data);

obj.EventHandlerFor(EventType.Step, EventSubtypeStep.Step, Data.Strings, Data.Code, Data.CodeLocals).ReplaceGML(@"

var a;
var b;
var c;

if (hspeed == 0 && vspeed == 0) {
	p_ind = (p_ind + 1) % (string_length(s0) / 14)
	r0 = real(string_copy(s0, p_ind * 14 + 1, 7)) - 524288
	r1 = real(string_copy(s0, p_ind * 14 + 8, 7)) - 524288

	a = r0 - x;
	b = r1 - y;
	c = sqrt(a * a + b * b) / r2;
	hspeed = a / c;
	vspeed = b / c;
	if xs {
		xs = false
		if o2 {
			x2 = o2.x - x
			y2 = o2.y - y
		}
		if o3 {
			x3 = o3.x - x
			y3 = o3.y - y
		}
	}
}

", Data);

obj.EventHandlerFor(EventType.Step, EventSubtypeStep.EndStep, Data.Strings, Data.Code, Data.CodeLocals).ReplaceGML(@"

if (hspeed * (r0 - x) + vspeed * (r1 - y) <= 0) {
	hspeed = 0;
	vspeed = 0;
	x = r0;
	y = r1;
}

var py = y - 32;
if (py % 32 == 0)
	py += 0.0625

if (o0 && instance_exists(o0)) {
	o0.x = x;
	o0.y = py;
}

if (o1 && instance_exists(o1)) {
	o1.x = x;
	o1.y = py;
}

py = bbox_bottom
if (py % 32 == 0)
	py -= 0.0625

if (o2 && instance_exists(o2)) {
	o2.x = x + x2;
	o2.y = py + y2;
}

if (o3 && instance_exists(o3)) {
	o3.x = x + x3;
	o3.y = py + y3;
}

", Data);

Data.GameObjects.Add(obj = new UndertaleGameObject() {
	Name = Data.Strings.MakeString("obj_e_floorMove_e"),
	Visible = true
});

obj.EventHandlerFor(EventType.Create, Data.Strings, Data.Code, Data.CodeLocals).ReplaceGML("r2 = false\nr3 = true", Data);

obj.EventHandlerFor(EventType.Draw, Data.Strings, Data.Code, Data.CodeLocals).ReplaceGML(@"

draw_set_colour(c_white)
draw_set_alpha(0.5)
draw_circle(x, y, 8, false)

if r3 {
	var d = sqrt((x - r0) * (x - r0) + (y - r1) * (y - r1));
	var c = floor((d - 24) / 8);
	var cd = (d - 24) / c;
	var a = darctan2(r0 - x, r1 - y);
	var t;

	while c {
		c -= 1
		t = (12 + cd * (c + 0.5)) / d
		if sprite_index < 0 {
			draw_circle(lerp(x, r0, t), lerp(y, r1, t), 2, false)
		} else {
			draw_sprite_ext(sprite_index, 0, lerp(x, r0, t), lerp(y, r1, t), 1, 1, a, c_white, 0.5)
		}
	}
}

if (!r2)
	draw_sprite(spr_floorMove, 0, x - 16, y - 16)
draw_set_alpha(1)
if (r2)
	draw_sprite(spr_floorMove, 0, x - 16, y - 16)

", Data);

// === WARP ===

Data.GameObjects.Add(obj = new UndertaleGameObject() {
	Name = Data.Strings.MakeString("obj_e_warp"),
	Visible = true,
});

obj.EventHandlerFor(EventType.Draw, Data.Strings, Data.Code, Data.CodeLocals).ReplaceGML(@"

draw_set_colour(c_white)
draw_set_alpha(0.5)
draw_line_width(x, y, r0, r1, 2)
draw_set_alpha(1)

", Data);

// === CRAWLING ONMYOUDAMA ===

Data.GameObjects.Add(obj = new UndertaleGameObject() {
	Name = Data.Strings.MakeString("obj_e_onmyoudamaCrawl"),
	Visible = true,
	Sprite = Data.Sprites.ByName("spr_onmyoudama1")
});

obj.EventHandlerFor(EventType.Create, Data.Strings, Data.Code, Data.CodeLocals).ReplaceGML("rot=0\nrot2=0\nr0=0\nmochi=noone\nmm=false\nflipper=noone", Data);

obj.EventHandlerFor(EventType.Step, EventSubtypeStep.Step, Data.Strings, Data.Code, Data.CodeLocals).ReplaceGML(@"
	rot -= 8
	rot2 += 5
	if (rot == -360)
		rot = 0
	if (rot2 == 360)
		rot2 = 0
	
	if mm || (flipper != noone && flipper.flip)
		exit
	
	if gravity > 0 {
		if place_meeting(x, y + floor(vspeed) + 1, obj_seijaHirarinuno) {
			y = obj_seijaHirarinuno.y - (bbox_bottom - y)
			gravity = 0
			vspeed = 0
			r1 = 0
			mochi = noone
			flipper = noone
		} else if (!place_free(x, y + 1)) {
			y -= vspeed
			move_contact_solid(270, vspeed + 2)
			gravity = 0
			vspeed = 0
			r1 = 0
			mochi = noone
			flipper = noone
		}

		else if (vspeed > 8)
			vspeed = 8
	}

	else {
		var dx = 0;
		var dy = 0;
		var gx = 0;
		var gy = 0;

		switch r1 {
			case 0:
				dx = 1
				gy = 1
			break;

			case 1:
				dy = 1
				gx = -1
			break;

			case 2:
				dx = -1
				gy = -1
			break;

			case 3:
				dy = -1
				gx = 1
			break;
		}

		if r0 {
			dx = -dx
			dy = -dy
		}

		if (place_free(x + gx, y + gy) && !place_meeting(x + gx, y + gy, obj_seijaHirarinuno)) {
			if (place_free(x + gx - dx, y + gy - dy) && !place_meeting(x + gx - dx, y + gy - dy, obj_seijaHirarinuno))
				gravity = 0.3
			else {
				x += gx
				y += gy
				if (r0)
					r1 = (r1 + 3) % 4
				else
					r1 = (r1 + 1) % 4
			}
		}

		else if (place_free(x + dx, y + dy) && !place_meeting(x + dx, y + dy, obj_seijaHirarinuno)) {
			mochi = collision_rectangle(bbox_left + gx, bbox_top + gy, bbox_right + gx, bbox_bottom + gy, obj_mochiBlock1, true, true)
			flipper = collision_rectangle(bbox_left + gx, bbox_top + gy, bbox_right + gx, bbox_bottom + gy, obj_e_onmyoudamaCrawl + 13, true, true)
			x += dx
			y += dy
		}

		else if (r0)
			r1 = (r1 + 1) % 4
		else
			r1 = (r1 + 3) % 4

		if mochi != noone && mochi.vs != 0 {
			mx = x - mochi.x
			my = y - mochi.y
			mm = true
		}
	}
", Data);

obj.EventHandlerFor(EventType.Step, EventSubtypeStep.EndStep, Data.Strings, Data.Code, Data.CodeLocals).ReplaceGML(@"
	if mm && instance_exists(mochi) {
		x = mochi.x + mx
		y = mochi.y + my
	}
		
	else if mm {
		mochi = collision_rectangle(bbox_left, bbox_top, bbox_right, bbox_bottom, obj_mochiBlock2, true, true)
		if (mochi == noone)
			exit

		var mcx = mochi.x + 48
		var mcy = mochi.y + 48
		var scx = mx + mcx
		var scy = my + mcy
		var ss = bbox_right - bbox_left
		var px = lerp(mcx, scx, 32 / (32 + ss))
		var py = lerp(mcy, scy, 32 / (32 + ss))
		var pdx = px - scx + 16
		var pdy = py - scy + 16
		px = (px - mcx) * 3.1 + mcx
		py = (py - mcy) * 3.1 + mcy
		x = px - pdx
		y = py - pdy - 2

		if !place_free(x, y) {
			instance_create(x, y, obj_headEffect)
			audio_play_sound(se_hold, 10, false)
			instance_destroy()
			exit
		}

		move_contact_solid(270 - r1 * 90, 4)

		mm = false
		mochi = noone
	}
", Data);

obj.EventHandlerFor(EventType.Draw, Data.Strings, Data.Code, Data.CodeLocals).ReplaceGML(@"
	if (r0)
		draw_sprite_ext(spr_onmyoudama1, 0, x + 32, y, -1, 1, 0, c_white, 1)
	else
		draw_self()
	draw_sprite_ext(spr_onmyoudama1_1, 0, (x + 16), (y + 16), 1, 1, rot, c_white, 1)
	draw_sprite_ext(spr_onmyoudama0, 0, (x + 16), (y + 16), 1, 1, rot2, c_white, 1)
", Data);

obj.EventHandlerFor(EventType.Collision, 65, Data.Strings, Data.Code, Data.CodeLocals).ReplaceGML("if (bbox_top < other.bbox_bottom - 2)\ne_kill_player(true)", Data);
obj.EventHandlerFor(EventType.Collision, 101, Data.Strings, Data.Code, Data.CodeLocals).ReplaceGML("effect_create_below(3, (x + 16), (y + 16), 2, 16777215)\ninstance_destroy()", Data);

// === FAIRY ===

Data.GameObjects.Add(obj = new UndertaleGameObject() {
	Name = Data.Strings.MakeString("obj_e_fairyDead"),
	Visible = true,
	Depth = -2
});

obj.EventHandlerFor(EventType.Create, Data.Strings, Data.Code, Data.CodeLocals).ReplaceGML("vspeed=-4.7\ngravity=0.5\nimage_speed=0", Data);

obj.EventHandlerFor(EventType.Step, EventSubtypeStep.Step, Data.Strings, Data.Code, Data.CodeLocals).ReplaceGML(@"
	image_angle += hspeed * 4.5
	image_index = 2
	if (y > max(obj_e_precipitator.x, obj_e_precipitator.y))
		instance_destroy()
", Data);

Data.GameObjects.Add(obj = new UndertaleGameObject() {
	Name = Data.Strings.MakeString("obj_e_fairy"),
	Visible = true,
	Sprite = Data.Sprites.ByName("spr_fairy1D")
});

obj.EventHandlerFor(EventType.Create, Data.Strings, Data.Code, Data.CodeLocals).ReplaceGML(@"
	x += 16
	y += 16
	hs = 1
	tt = 8
	at = 0
	search = 0
	alert = noone
	image_speed = 0
	lh = noone
	rs = true
	o0 = noone
	o1 = noone
	dir = 1
", Data);

obj.EventHandlerFor(EventType.Destroy, Data.Strings, Data.Code, Data.CodeLocals).ReplaceGML(@"
	with (instance_create(x, y, obj_e_fairyDead)) {
		sprite_index = other.sprite_index
		hspeed = 2 * sign(other.hs)
		image_xscale = other.image_xscale
	}
	e_s_remove(id)
", Data);

obj.EventHandlerFor(EventType.Step, EventSubtypeStep.BeginStep, Data.Strings, Data.Code, Data.CodeLocals).ReplaceGML(@"
	if rs {
		rs = false
		e_s_register(id, o0, o1, 27)
	}
", Data);

obj.EventHandlerFor(EventType.Step, EventSubtypeStep.Step, Data.Strings, Data.Code, Data.CodeLocals).ReplaceGML(@"
var flr

if image_index < 2 {
	at++
	image_index = at % 31 > 15
}

if e_s_base(id) == id {
	hspeed = hs

	if place_free(x, y + 1) {
		flr = max(
			collision_rectangle(bbox_left, bbox_bottom - 2, bbox_right, bbox_bottom + 2, obj_floor, true, true),
			collision_rectangle(bbox_left, bbox_bottom - 2, bbox_right, bbox_bottom + 4, obj_seijaHirarinuno, true, true)
		)
		if flr == noone {
			gravity = 0.3
			search = 0
			if instance_exists(alert)
				instance_destroy(alert)
		} else if vspeed >= flr.vspeed {
			y = flr.y - 15
			if (gravity != 0)
				instance_create(x - 16, y - 16, obj_landingEffect)
			gravity = 0
			vspeed = flr.vspeed
		}
	} else if gravity != 0 && vspeed >= -1 {
		gravity = 0
		y -= vspeed
		move_contact_solid(270, vspeed + 1)
		vspeed = 0
		instance_create(x - 16, y - 16, obj_landingEffect)
	}
	
	if (!place_free(x, y) && !place_free(x + hs, y)) {
		hspeed = 0
		vspeed = 0
		gravity = 0
		lh = noone
		exit
	}

	if vspeed > 5 {
		vspeed = 5
	}

	else if (vspeed < 0 && !place_free(x, y + vspeed)) {
		move_contact_solid(90, -vspeed - 1)
		vspeed = 0
	}

	if object_index > obj_e_fairy && image_index == 2 {
		instance_destroy(collision_rectangle(bbox_left + hs, bbox_top, bbox_right + hs, bbox_bottom, obj_block, true, true))
		instance_destroy(collision_rectangle(bbox_left + hs, bbox_top, bbox_right + hs, bbox_bottom, obj_iceWall1, true, true))
		instance_destroy(collision_rectangle(bbox_left + hs, bbox_top, bbox_right + hs, bbox_bottom, obj_bounceBlock, true, true))
		instance_destroy(collision_rectangle(bbox_left + hs, bbox_top, bbox_right + hs, bbox_bottom, obj_block22, true, true))
		with (collision_rectangle(bbox_left + hs, bbox_top, bbox_right + hs, bbox_bottom, obj_mochiBlock1, true, true))
			mochiFlag = true
		with (collision_rectangle(bbox_left + hs, bbox_top, bbox_right + hs, bbox_bottom, obj_iceSwitch, true, true)) {
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
			instance_create(other.x-24, other.y-24, obj_iceMakeEffect)
		}
	}

	if search {
		hspeed = 0
		if search == 20 {
			search = 0
			image_index = 2
			hs *= 6
			if (hs > 0)
				instance_create(x - 16, y - 16, obj_darkBlockEffectL)
			else
				instance_create(x - 16, y - 16, obj_darkBlockEffect)
		}

		else
			search += 1
	}

	else if (!place_free(x + hs, y) || collision_rectangle(x, bbox_top + 1, x + 16 * image_xscale + hs, bbox_bottom - 1, obj_seijaHirarinuno, true, true) || (
		abs(hs) == 1 &&
		collision_rectangle(x + 17 * image_xscale, bbox_top + 1, x + 18 * image_xscale, bbox_bottom - 1, obj_e_fairy, true, true) != noone
	) || (
		gravity == 0 &&
		object_index == obj_e_fairy &&
		place_free(32 * floor((x + 16 * image_xscale) / 32) + 16, y + 8) &&
		place_free(32 * floor((x + 32 * image_xscale) / 32) + 16, y + 8) &&
		collision_rectangle(x + 16 * image_xscale, bbox_bottom - 2, x + 40 * image_xscale, bbox_bottom + 2, obj_floor, true, true) == noone &&
		collision_rectangle(x + 16 * image_xscale, bbox_bottom - 2, x + 40 * image_xscale, bbox_bottom + 2, obj_seijaHirarinuno, true, true) == noone
	)) {
		image_index = 0
		at = 0
		if (tt < 4)
			hspeed = 0
		else {
			if (hs > 0)
				hs = -1
			else
				hs = 1

			image_xscale = hs
			x += 2 * hs
			tt = 0
		}
	}

	else if gravity == 0 && abs(hs) == 1 && (
		(collision_rectangle(x, y - 8, x + 112 * image_xscale, y + 8, obj_player, true, true) && !instance_exists(obj_seijaHirarinuno)) ||
		collision_rectangle(x, y - 8, x + 112 * image_xscale, y + 8, 1544, true, true)
	) {
		search = 1
		hspeed = 0
		alert = instance_create_depth(x - 29, y - 29, -256, obj_hukidashi_e)
	}

	if abs(hs) > 1 {
		flr = collision_rectangle(x + 17 * image_xscale, bbox_top, x + 18 * image_xscale, bbox_bottom, obj_e_fairy, true, true)
		if (flr != noone && flr.vspeed > -2) {
			flr.vspeed = -4.5
			if flr.image_index == 2 {
				flr.hs = sign(flr.hs)
				hs = sign(hs)
				vspeed = -4.5
			}
		}
	}

	if (hspeed != hs)
		hspeed = 0
	
	if collision_rectangle(bbox_left, bbox_top + 1, bbox_right, bbox_bottom - 1, obj_desert, true, true) != noone {
		hspeed /= 2
		vspeed = min(vspeed, 0.5)
	}

	flr = collision_rectangle(bbox_left + 1, bbox_bottom - 2, bbox_right - 1, bbox_bottom + 2, obj_e_floorMove, true, true)
	if (flr != noone && flr.bbox_top > bbox_bottom - 3) {
		hspeed += flr.hspeed
		vspeed = flr.vspeed
		if (!place_free(x + hspeed, y + min(0, vspeed)))
			hspeed = 0
		if !place_free(x, y + sign(vspeed)) {
			if (vspeed < 0)
				instance_destroy()
			else {
				vspeed = 0
				move_contact_solid(270, vspeed + 1)
			}
		}
	}

	else {
		if (collision_rectangle(bbox_left + 1, bbox_bottom - 2, bbox_right - 1, bbox_bottom + 2, obj_Lfloor, true, true) != noone && vspeed >= 0 && place_free(x + hspeed - 1, y))
			hspeed -= 1
		if (collision_rectangle(bbox_left + 1, bbox_bottom - 2, bbox_right - 1, bbox_bottom + 2, obj_Rfloor, true, true) != noone && vspeed >= 0 && place_free(x + hspeed + 1, y))
			hspeed += 1
	}

	tt++
}

if (instance_exists(lh) && (lh.bbox_left > bbox_right || lh.bbox_right < bbox_left || lh.bbox_bottom < bbox_top))
	lh = noone

dir = image_xscale
", Data);

obj.EventHandlerFor(EventType.Step, EventSubtypeStep.EndStep, Data.Strings, Data.Code, Data.CodeLocals).ReplaceGML(@"
	var b = e_s_base(id)
	if b != id {
		hspeed = 0
		vspeed = 0
		if (at == 1)
			at = random(15)
		image_xscale = b.dir
		hs = image_xscale
		x = (b.bbox_left + b.bbox_right) / 2
		y = b.bbox_bottom - e_s_height(id) - 15
		if (b.vspeed < 0 && !instance_exists(o1) && !place_free(x, y) && place_free(x, y + 2) && collision_rectangle(b.bbox_left + 1, b.bbox_bottom - 2, b.bbox_right - 1, b.bbox_bottom + 4, obj_e_floorMove, true, true) != noone) {
			instance_destroy()
			audio_play_sound(se_hold, 10, false)
		}
	}
", Data);

obj.EventHandlerFor(EventType.Collision, 65, Data.Strings, Data.Code, Data.CodeLocals).ReplaceGML("if lh != other.id && (!instance_exists(obj_seijaHirarinuno) || bbox_bottom - vspeed > obj_seijaHirarinuno.bbox_top) {\nlh = other.id\ne_bonk(x, other)\n}", Data);
obj.EventHandlerFor(EventType.Collision, 1544, Data.Strings, Data.Code, Data.CodeLocals).ReplaceGML("if lh != other.id && o0 != other.id && o1 != other.id {\nlh = other.id\ne_bonk(x, other)\n}", Data);
obj.EventHandlerFor(EventType.Collision, 77, Data.Strings, Data.Code, Data.CodeLocals).ReplaceGML("hs = 0.5 - other.dir\ninstance_destroy()", Data);
obj.EventHandlerFor(EventType.Collision, 78, Data.Strings, Data.Code, Data.CodeLocals).ReplaceGML("instance_destroy()", Data);
obj.EventHandlerFor(EventType.Collision, 101, Data.Strings, Data.Code, Data.CodeLocals).ReplaceGML("instance_destroy()", Data);
obj.EventHandlerFor(EventType.Collision, 85, Data.Strings, Data.Code, Data.CodeLocals).ReplaceGML("hs = 0.5 - other.dir\ninstance_destroy()", Data);
obj.EventHandlerFor(EventType.Collision, 86, Data.Strings, Data.Code, Data.CodeLocals).ReplaceGML("hs = -1\ninstance_destroy()", Data);
obj.EventHandlerFor(EventType.Collision, 87, Data.Strings, Data.Code, Data.CodeLocals).ReplaceGML("hs = 1\ninstance_destroy()", Data);
obj.EventHandlerFor(EventType.Collision, 88, Data.Strings, Data.Code, Data.CodeLocals).ReplaceGML("instance_destroy()", Data);
obj.EventHandlerFor(EventType.Collision, 89, Data.Strings, Data.Code, Data.CodeLocals).ReplaceGML("instance_destroy()", Data);
obj.EventHandlerFor(EventType.Collision, 90, Data.Strings, Data.Code, Data.CodeLocals).ReplaceGML("instance_destroy()", Data);
obj.EventHandlerFor(EventType.Collision, 1467, Data.Strings, Data.Code, Data.CodeLocals).ReplaceGML("instance_destroy()", Data);
obj.EventHandlerFor(EventType.Collision, 1469, Data.Strings, Data.Code, Data.CodeLocals).ReplaceGML("instance_destroy()", Data);

Data.GameObjects.Add(obj = new UndertaleGameObject() {
	Name = Data.Strings.MakeString("obj_e_fairy2"),
	Visible = true,
	Sprite = Data.Sprites.ByName("spr_fairy3D"),
	ParentId = obj
});

obj.EventHandlerFor(EventType.Collision, 65, Data.Strings, Data.Code, Data.CodeLocals).ReplaceGML("if (!instance_exists(obj_seijaHirarinuno) || bbox_bottom - vspeed > obj_seijaHirarinuno.bbox_top) e_kill_player(true)", Data);

// === MOVING CHANDELIER ===

Data.GameObjects.Add(obj = new UndertaleGameObject() {
	Name = Data.Strings.MakeString("obj_e_movingFloorChandelier"),
	Visible = true,
	ParentId = Data.GameObjects.ByName("obj_e_floorMove"),
	Sprite = Data.Sprites.ByName("spr_chandelier"),
	Depth = -1
});

obj.EventHandlerFor(EventType.Step, EventSubtypeStep.EndStep, Data.Strings, Data.Code, Data.CodeLocals).ReplaceGML(@"

x = mf.x + mfx;
y = mf.y + mfy;
hspeed = mf.hspeed;
vspeed = mf.vspeed;

var py = y - 32;
if (py % 32 == 0)
	py += 0.0625

if (o0 && instance_exists(o0)) {
	o0.x = x + x0
	o0.y = py
}
if (o1 && instance_exists(o1)) {
	o1.x = x + x1
	o1.y = py
}
if (o2 && instance_exists(o2)) {
	o2.x = x + x2
	o2.y = py
}
if (o3 && instance_exists(o3)) {
	o3.x = x + x3
	o3.y = py
}

", Data);

obj.EventHandlerFor(EventType.Step, EventSubtypeStep.Step, Data.Strings, Data.Code, Data.CodeLocals);

// Conveyor line

Data.GameObjects.Add(obj = new UndertaleGameObject() {
	Name = Data.Strings.MakeString("obj_e_conveyorChandelierLine"),
	Visible = true,
	ParentId = Data.GameObjects.ByName("obj_chandelierLine"),
	Sprite = Data.Sprites.ByName("spr_chandelierLine")
});

obj.EventHandlerFor(EventType.Step, EventSubtypeStep.Step, Data.Strings, Data.Code, Data.CodeLocals).ReplaceGML(@"
	if instance_exists(o0)
		x = o0.x - 8
", Data);

// Conveyor version

Data.GameObjects.Add(obj = new UndertaleGameObject() {
	Name = Data.Strings.MakeString("obj_e_conveyorChandelier"),
	Visible = true,
	ParentId = Data.GameObjects.ByName("obj_e_floorMove"),
	Sprite = Data.Sprites.ByName("spr_chandelier"),
	Depth = -1
});

obj.EventHandlerFor(EventType.Create, Data.Strings, Data.Code, Data.CodeLocals).ReplaceGML("fall=false\nbuild=true\no0=0\no1=0\no2=0\no3=0\nrx=x\nx0=-8\nx1=-8\nx2=24\nx3=24", Data);

string fallQuery = @"
	collision_rectangle((x + 22), y, (x + 26), (y - r0), obj_headThrow, 1, 1) ||
	collision_rectangle((x + 23), y, (x + 24), (y - r0), obj_headThrowUp, 1, 1) ||
	collision_rectangle((x + 22), y, (x + 26), (y - r0), obj_iceBullet, 1, 1) ||
	collision_rectangle((x + 22), y, (x + 26), (y - r0), obj_iceBullet2, 1, 1) ||
	collision_rectangle((x + 22), y, (x + 26), (y - r0), obj_iceBullet3, 1, 1) ||
	collision_rectangle((x + 22), y, (x + 26), (y - r0), obj_iceBullet4, 1, 1) ||
	collision_rectangle((x + 22), y, (x + 26), (y - r0), obj_iceBullet5, 1, 1) ||
	collision_rectangle((x + 22), y, (x + 26), (y - r0), obj_iceBulletUp, 1, 1) ||
	collision_rectangle((x + 18), y, (x + 30), (y - r0), obj_darkBombHit, 1, 1) ||
	collision_rectangle((x + 18), y, (x + 30), (y - r0), obj_magicBombHit, 1, 1) ||
	collision_rectangle(x, y, x + 48, y + 1, obj_hammerEffect, 1, 1)
";

Data.Code.ByName("gml_Object_obj_chandelier_Step_0").ReplaceGML(@"
	if (!fall && (
		" + fallQuery + @"
	))
		fall = true
	if !fall {
		var fairy = collision_rectangle((x + 22), y, (x + 26), (y - r0), obj_e_fairy2, 1, 1)
		if (fairy != noone && abs(fairy.hs) > 1 && sign(x + 24 - fairy.x - fairy.hs) == fairy.image_xscale)
			fall = true
	}
	else
		Timer++
	if (Timer == 1) {
		gravity = 0.3
		audio_play_sound(se_chandelier1, 30, false)
		instance_create(xPoint, yPoint, obj_himoEffect)
		instance_create(xPoint, yPoint, obj_headEffect)
	}
", Data);

obj.EventHandlerFor(EventType.Step, EventSubtypeStep.Step, Data.Strings, Data.Code, Data.CodeLocals).ReplaceGML(@"
	var r1 = 0;
	var lf;

	if build {
		build = false
		var i = 0
		while i < r0 {
			i += 32
			with (instance_create(x - 8, y - i, obj_e_conveyorChandelierLine))
				o0 = other.id
		}
		i = instance_create(x - 8, y, obj_e_conveyorChandelierLine)
		i.image_index = 1
		i.o0 = id
	}

	hspeed = 0

	lf = collision_rectangle(x + 22, y - r0 - 4, x + 25, y - r0 - 2, obj_Lfloor, true, true)

	if lf == noone {
		if (collision_rectangle(x + 22, y - r0 - 4, x + 25, y - r0 - 2, obj_Rfloor, true, true) == noone) {
			fall = true
			with obj_e_conveyorChandelierLine
				if (o0 == other.id)
					instance_destroy()
		}
		else {
			r1 = -(collision_rectangle(x + 20, y - r0 - 4, x + 21, y - r0 - 2, obj_Rfloor, true, true) != noone)
		}
	}
	
	else {
		r1 = (collision_rectangle(x + 26, y - r0 - 4, x + 27, y - r0 - 2, obj_Lfloor, true, true) != noone)
	}

	if (
		fall ||
		" + fallQuery + @"
	) {
		fall = instance_create(x, y, obj_chandelier)
		with (fall) {
			fall = true
			o0 = other.o0
			o1 = other.o1
			o2 = other.o2
			o3 = other.o3
			x0 = other.x0;
			x1 = other.x1;
			x2 = other.x2;
			x3 = other.x3;
		}
		with (obj_e_movingFloorChandelier) {
			if (mf == other.id)
				mf = other.fall
		}
		instance_destroy()
	}

	else if (
		collision_rectangle(bbox_left + r1, bbox_top + 1, bbox_right + r1, bbox_bottom - 1, obj_chandelier, 1, 1) == noone &&
		collision_rectangle(bbox_left + r1, bbox_top + 1, bbox_right + r1, bbox_bottom - 1, obj_e_conveyorChandelier, 1, 1) == noone
	) {
		rx += r1
		if !place_free(rx, y)
			rx = x
		hspeed = rx - x
	}

	if !fall {
		var fairy = collision_rectangle((x + 22), y, (x + 26), (y - r0), obj_e_fairy2, 1, 1)
		if (fairy != noone && abs(fairy.hs) > 1 && sign(x + 24 - fairy.x - fairy.hs) == fairy.image_xscale)
			fall = true
	}
", Data);

obj.EventHandlerFor(EventType.Step, EventSubtypeStep.EndStep, Data.Strings, Data.Code, Data.CodeLocals).ReplaceGML(@"
	x = rx
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

obj.EventHandlerFor(EventType.Collision, 152, Data.Strings, Data.Code, Data.CodeLocals).ReplaceGML("fall=true", Data);

// === PLAYERLIKE ===

Data.GameObjects.Add(obj = new UndertaleGameObject() {
	Name = Data.Strings.MakeString("obj_e_playerLike"),
	Sprite = Data.Sprites.ByName("spr_bankiidle"),
});

obj.EventHandlerFor(EventType.Create, Data.Strings, Data.Code, Data.CodeLocals).ReplaceGML(@"
	walk = 0
	landedOnce = false
	bonk = 0
	bonkh = 0
	pushL = false
	pushR = false
	image_speed = 0
	dir = 1
	frame = random(120)
	o0 = noone
	o1 = noone
	rs = true
	warpHash = 0.5
	warpTimer = 0
", Data);

obj.EventHandlerFor(EventType.Destroy, Data.Strings, Data.Code, Data.CodeLocals).ReplaceGML("e_s_remove(id)", Data);

obj.EventHandlerFor(EventType.Step, EventSubtypeStep.BeginStep, Data.Strings, Data.Code, Data.CodeLocals).ReplaceGML(@"
	if rs {
		rs = false
		e_s_register(id, o0, o1, 28 + object_index - obj_e_playerLike)
	}
", Data);

obj.EventHandlerFor(EventType.Step, EventSubtypeStep.Step, Data.Strings, Data.Code, Data.CodeLocals).ReplaceGML(@"
if id == e_s_base(id) {
	if (landedOnce && bonk == 0) {
		if ((walk < 0 && pushL) || (walk > 0 && pushR))
			hspeed = sign(walk)
		else
			hspeed = walk
	}

	else
		hspeed = 0
	
	if (walk != 0)
		dir = sign(walk)
	
	if (vspeed > 6)
		vspeed = 6
	
	if bonk > 0 {
		bonk += 1
		if (gravity != 0 && bonk > 23)
			bonk = 23
		else if (bonk > 25)
			bonk = 0
		hspeed = bonkh
	}

	if (vspeed < 0)
		landedOnce = true
	
	if (place_free(x, y + 1) && collision_rectangle(bbox_left, bbox_bottom - 14, bbox_right, bbox_bottom + 1, obj_floor, true, true) == noone)
		gravity = 0.3
	else if vspeed > 0 {
		gravity = 0
		vspeed = 0
		landedOnce = true
		if (!place_free(x, y) && place_free(x, y + 1))
			y -= 1
	}
	
	var floorMove = collision_rectangle(bbox_left, bbox_bottom - 2, bbox_right, bbox_bottom + 3, obj_e_floorMove, 1, 1);
	if (floorMove != noone && (vspeed >= 0 || vspeed >= floorMove.vspeed) && floorMove.bbox_top > bbox_bottom - 3) {
		vspeed = floorMove.vspeed / 256;
		if (vspeed > 0) {
			y = min(y, floorMove.y - (bbox_bottom - y)) - 1;
			move_contact_solid(270, 4);
			y = floor(y);
			gravity = 0;
			landedOnce = true
		} else if floorMove.vspeed < 0 {
			if !place_free(x, y - 1) {
				instance_destroy()
				instance_create(x, y, obj_headEffect);
				audio_play_sound(se_hold, 10, false);
				exit
			}
			y = floorMove.y - 32
			landedOnce = true
		}
		if (collision_rectangle(bbox_left, bbox_bottom - 2, bbox_right, ceil(bbox_bottom) + 1, obj_e_floorMove, 1, 1)) {
			hspeed += floorMove.hspeed;
		}
	}
	
	if (collision_rectangle((x + 6), (y + 30), (x + 25), (y + 33), obj_Lfloor, 1, 1) != noone)
		hspeed -= 1
	if (collision_rectangle((x + 6), (y + 30), (x + 25), (y + 33), obj_Rfloor, 1, 1) != noone)
		hspeed += 1
	
	if collision_rectangle((x + 7), (y + 3), (x + 24), (y + 31), obj_desert, true, true) != noone {
		hspeed /= 3
		vspeed = 0.5
		if (abs(bonkh) < 0.5)
			bonk = 0
		else
			bonkh = (abs(bonkh) - 0.5) * sign(bonkh)
	}
	
	if (hspeed != 0 && !place_free(x + hspeed, y)) {
		if (sign(hspeed) == sign(walk))
			walk *= -1
		hspeed = 0
		bonkh = 0
		move_contact_solid(90 * (1 - sign(hspeed)), abs(hspeed) + 1)
	}

	warpTimer -= 1
}
	
pushL = false
pushR = false
frame += 1
", Data);

obj.EventHandlerFor(EventType.Step, EventSubtypeStep.EndStep, Data.Strings, Data.Code, Data.CodeLocals).ReplaceGML(@"
	var b = e_s_base(id)
	if b != id {
		landedOnce = true
		hspeed = 0
		vspeed = 0
		dir = b.dir
		walk = abs(walk) * sign(dir)
		x = (b.bbox_left + b.bbox_right) / 2 - 15
		y = b.bbox_bottom - e_s_height(id) - 32
		if (b.vspeed < 0 && !instance_exists(o1) && !place_free(x, y) && place_free(x, y + 2) && collision_rectangle(b.bbox_left + 1, b.bbox_bottom - 2, b.bbox_right - 1, b.bbox_bottom + 4, obj_e_floorMove, true, true) != noone) {
			audio_play_sound(se_hold, 10, false)
			instance_create(x, y, obj_headEffect);
			instance_destroy()
		}
	}
", Data);

obj.EventHandlerFor(EventType.Draw, Data.Strings, Data.Code, Data.CodeLocals).ReplaceGML(@"
	var px
	var sx
	var si
	
	if dir < 0 {
		px = x + 32
		sx = -1
	} else {
		px = x
		sx = 1
	}

	if (bonk > 0)
		si = 8
	else if (gravity == 0 || id != e_s_base(id))
		si = frame % 120 > 110
	else if (vspeed < 0)
		si = 4
	else
		si = 5
	
	draw_sprite_ext(spr_banki_doremy, si, px, y, sx, 1, 0, c_white, 1)
", Data);

obj.EventHandlerFor(EventType.Collision, 138, Data.Strings, Data.Code, Data.CodeLocals).ReplaceGML(@"
	if vspeed < 0 {
		other.mochiFlag = true
		move_contact_solid(90, -vspeed)
		vspeed = 0
	} else if vspeed > other.vs {
		move_contact_solid(270, vspeed)
		instance_create(x, y, obj_landingEffect)
		landedOnce = true
		if (other.vs >= 0) {
			vspeed = 0
			gravity = 0
		} else {
			vspeed = -8.9
			gravity = 0.3
		}
	}
", Data);

obj.EventHandlerFor(EventType.Collision, (uint)0, Data.Strings, Data.Code, Data.CodeLocals).ReplaceGML(@"
	if other.solid {
		if vspeed < 0 {
			move_contact_solid(90, -vspeed)
		} else if vspeed > 0 {
			move_contact_solid(270, vspeed)
			gravity = 0
			instance_create(x, y, obj_landingEffect)
			landedOnce = true
		}
		vspeed = 0
	}
", Data);

obj.EventHandlerFor(EventType.Collision, 39, Data.Strings, Data.Code, Data.CodeLocals).ReplaceGML(@"
	if other.solid {
		if vspeed < 0 {
			move_contact_solid(90, -vspeed)
		} else if vspeed > 0 {
			move_contact_solid(270, vspeed)
			gravity = 0
			instance_create(x, y, obj_landingEffect)
			landedOnce = true
		}
		vspeed = 0
	}
", Data);

obj.EventHandlerFor(EventType.Collision, 40, Data.Strings, Data.Code, Data.CodeLocals).ReplaceGML(@"
	if other.solid {
		if vspeed < 0 {
			move_contact_solid(90, -vspeed)
		} else if vspeed > 0 {
			move_contact_solid(270, vspeed)
			gravity = 0
			instance_create(x, y, obj_landingEffect)
			landedOnce = true
		}
		vspeed = 0
	}
", Data);

obj.EventHandlerFor(EventType.Collision, 10, Data.Strings, Data.Code, Data.CodeLocals).ReplaceGML(@"
	if vspeed < 0 {
		move_contact_solid(90, -vspeed)
	} else if vspeed > 0 {
		move_contact_solid(270, vspeed)
		gravity = 0
		instance_create(x, y, obj_landingEffect)
		landedOnce = true
	}
	vspeed = 0
", Data);

obj.EventHandlerFor(EventType.Collision, 11, Data.Strings, Data.Code, Data.CodeLocals).ReplaceGML(@"
	if vspeed < 0 {
		move_contact_solid(90, -vspeed)
	} else if vspeed > 0 {
		move_contact_solid(270, vspeed)
		gravity = 0
		instance_create(x, y, obj_landingEffect)
		landedOnce = true
	}
	vspeed = 0
", Data);

obj.EventHandlerFor(EventType.Collision, 1469, Data.Strings, Data.Code, Data.CodeLocals).ReplaceGML("e_bonk(obj_player.x + 16, id)", Data);

Data.GameObjects.Add(obj = new UndertaleGameObject() {
	Name = Data.Strings.MakeString("obj_e_flan"),
	Sprite = Data.Sprites.ByName("spr_bankiidle"),
	ParentId = obj
});

obj.EventHandlerFor(EventType.Create, Data.Strings, Data.Code, Data.CodeLocals).ReplaceGML("event_inherited()\nwalk = 5", Data);

obj.EventHandlerFor(EventType.Draw, Data.Strings, Data.Code, Data.CodeLocals).ReplaceGML(@"
	var ii = gravity == 0 && frame % 14 < 7 && bonk == 0 && landedOnce && e_s_base(id) == id;
	var si;
	if (bonk > 0)
		si = spr_e_flanBonk
	else
		si = spr_playerF
	if (dir > 0)
		draw_sprite(si, ii, x, y)
	else
		draw_sprite_ext(si, ii, x + 32, y, -1, 1, 0, c_white, 1)
", Data);

// === BONK ===

Data.GameObjects.Add(obj = new UndertaleGameObject() {
	Name = Data.Strings.MakeString("obj_e_playerBonkTimer")
});

obj.EventHandlerFor(EventType.Create, Data.Strings, Data.Code, Data.CodeLocals).ReplaceGML("t=0\nd=0", Data);

obj.EventHandlerFor(EventType.Step, EventSubtypeStep.Step, Data.Strings, Data.Code, Data.CodeLocals).ReplaceGML(@"
	if !instance_exists(obj_player)
		exit
	t++
	if (obj_player.vspeed != 0 && t > 24 && t < 30)
		t = 24
	if (t > 25) {
		if (global.viewmode == 0)
			obj_gameMgr.playinput = true
		if instance_exists(obj_bankiOther)
			obj_bankiOther.visible = false
		if instance_exists(obj_cirnoOther)
			obj_cirnoOther.visible = false
		obj_player.visible = true
		instance_destroy()
	} else {
		with (obj_player) {
			if (collision_rectangle(bbox_left, bbox_top, bbox_right, bbox_bottom, obj_desert, true, true) != noone) {
				if (abs(other.d) < 0.5)
					other.t = 50
				else
					other.d = (abs(other.d) - 0.5) * sign(other.d)
			}
			if place_free(x + other.d + sign(other.d), y)
				obj_player.hspeed = other.d
			else
				other.d = 0
		}
	}
", Data);

// === BLACK SWITCH ===

var dctx = new GlobalDecompileContext(Data, false);

Data.GameObjects.Add(obj = new UndertaleGameObject() {
	Name = Data.Strings.MakeString("obj_e_blackSwitch"),
	Sprite = Data.Sprites.ByName("spr_redSwitch")
});

obj.EventHandlerFor(EventType.Create, Data.Strings, Data.Code, Data.CodeLocals).ReplaceGML("tAnim=0\nsubImage=0\nimage_speed=0", Data);

obj.EventHandlerFor(EventType.Draw, Data.Strings, Data.Code, Data.CodeLocals).ReplaceGML("draw_sprite_ext(spr_reverseSwitch, subImage, x + 32, y + 32, 1, 1, 180, c_white, 1)", Data);

obj.EventHandlerFor(EventType.Step, EventSubtypeStep.Step, Data.Strings, Data.Code, Data.CodeLocals).ReplaceGML(Decompiler.Decompile(Data.Code.ByName("gml_Object_obj_reverseSwitch_Step_0"), dctx), Data);

obj.EventHandlerFor(EventType.Collision, 65, Data.Strings, Data.Code, Data.CodeLocals).ReplaceGML(@"
	if (other.vspeed < 0 || other.bbox_bottom - y > 17)
		exit
	tAnim = -13
	other.vspeed = -6
	other.gravity = 0.3
	instance_create(x, (y - 22), obj_jumpEffectH)
	audio_play_sound(se_switch, 10, false)
	audio_play_sound(se_jump, 10, false)
	e_flip()
", Data);

// === GREY SWITCH R ===

Data.GameObjects.Add(obj = new UndertaleGameObject() {
	Name = Data.Strings.MakeString("obj_e_graySwitchR"),
	Sprite = Data.Sprites.ByName("spr_graySwitch")
});

obj.EventHandlerFor(EventType.Create, Data.Strings, Data.Code, Data.CodeLocals).ReplaceGML("tAnim=0\nsubImage=0\nimage_speed=0", Data);

obj.EventHandlerFor(EventType.Draw, Data.Strings, Data.Code, Data.CodeLocals).ReplaceGML("draw_sprite_ext(spr_graySwitch, subImage, x + 32, y + 32, 1, 1, 180, c_white, 1)", Data);

obj.EventHandlerFor(EventType.Step, EventSubtypeStep.Step, Data.Strings, Data.Code, Data.CodeLocals).ReplaceGML(Decompiler.Decompile(Data.Code.ByName("gml_Object_obj_graySwitch_Step_0"), dctx), Data);

obj.EventHandlerFor(EventType.Collision, 65, Data.Strings, Data.Code, Data.CodeLocals).ReplaceGML(@"
	if (other.vspeed < 0 && collision_line((x + 5), (y + 18), (x + 26), (y + 19), other.id, true, true) != noone) {
		tAnim = -13
		other.vspeed = 3
		other.gravity = 0.3
		instance_create(x, (y + 22), obj_jumpEffectHR)
		audio_play_sound(se_switch, 10, false)
		audio_play_sound(se_jump, 10, false)
		if instance_exists(obj_Lfloor)
			obj_Lfloor.switchFlag = 1
		if instance_exists(obj_Rfloor)
			obj_Rfloor.switchFlag = 1
	}
", Data);

// === WHITE SWITCH R ===

Data.GameObjects.Add(obj = new UndertaleGameObject() {
	Name = Data.Strings.MakeString("obj_e_whiteSwitchR"),
	Sprite = Data.Sprites.ByName("spr_whiteSwitch")
});

obj.EventHandlerFor(EventType.Create, Data.Strings, Data.Code, Data.CodeLocals).ReplaceGML("tAnim=0\nsubImage=0\nimage_speed=0", Data);

obj.EventHandlerFor(EventType.Draw, Data.Strings, Data.Code, Data.CodeLocals).ReplaceGML("draw_sprite_ext(spr_whiteSwitch, subImage, x + 32, y + 32, 1, 1, 180, c_white, 1)", Data);

obj.EventHandlerFor(EventType.Step, EventSubtypeStep.Step, Data.Strings, Data.Code, Data.CodeLocals).ReplaceGML(Decompiler.Decompile(Data.Code.ByName("gml_Object_obj_whiteSwich_Step_0"), dctx), Data);

obj.EventHandlerFor(EventType.Collision, 65, Data.Strings, Data.Code, Data.CodeLocals).ReplaceGML(@"
	if (other.vspeed < 0 && collision_line((x + 5), (y + 18), (x + 26), (y + 19), other.id, true, true) != noone) {
		tAnim = -13
		other.vspeed = 3
		other.gravity = 0.3
		instance_create(x, (y + 22), obj_jumpEffectHR)
		audio_play_sound(se_switch, 10, false)
		audio_play_sound(se_jump, 10, false)
		if !obj_whiteSwich.flag {
			if (instance_exists(obj_cannonBlockUD) == 1)
				obj_cannonBlockUD.flag = 1
			if (instance_exists(obj_cannonBlockLR) == 1)
				obj_cannonBlockLR.flag = 1
		}
	}
", Data);

// === FLIPPER ===

Data.GameObjects.Add(obj = new UndertaleGameObject() {
	Name = Data.Strings.MakeString("obj_e_flipper"),
	Solid = true,
	ParentId = Data.GameObjects[0]
});

var createCode = "";
var passiveCode = "";
var ejectCode = "";
var startCode = "";
var turnCode = "";
var rebuildCode = "";
var colourCases = "";

foreach (string colour in "red blue green yellow".Split(' ')) {
	colourCases += $@"
		case spr_{colour}Switch:
			if (o?.image_angle == 180)
				ni = instance_create(o?.x - 32, o?.y - 32, obj_{colour}SwitchR)
			else
				ni = instance_create(o?.x, o?.y, obj_{colour}Switch)
		break
	";
}

for (int i = 0; i < 8; i++) {
	createCode += $"o{i}=noone\n";

	passiveCode += @"
		if instance_exists(o?) {
			if (o?.y < y)
				o?.y = y - 36
			else if (o?.y < y + 16)
				o?.y = y + 4
			else
				o?.y = y + 36
			o?.x = round(o?.x)
		}
	".Replace('?', (char)(i + 48));

	startCode += @"
		if instance_exists(o?) {
			ni = instance_create_depth(o?.x, o?.y, o?.depth, obj_e_no_anim)
			ni.sprite_index = o?.sprite_index
			ni.image_angle = o?.image_angle

			switch (o?.object_index) {
				case obj_e_blackSwitch:
					ni.sprite_index = spr_reverseSwitch
				case obj_redSwitchR:
				case obj_blueSwitchR:
				case obj_greenSwitchR:
				case obj_yellowSwitchR:
				case obj_e_graySwitchR:
				case obj_e_whiteSwitchR:
					ni.x += 32
					ni.y += 32
					ni.image_angle = 180
				break

				case obj_dawkSwtich:
					with (obj_darkSwitchOn)
						if (o0 == other.o?)
							{ o0 = ni; break; }
				break

				case obj_e_no_anim:
				case obj_swtichBlockHole:
				case obj_swtichBlockBlueHole:
				case obj_switchBlockGreenHole:
				case obj_switchBlockYellowHole:
					nj = ni
					ni = o?
					o? = nj
				break
			}

			instance_destroy(o?)
			o? = ni
		}
	".Replace('?', (char)(i + 48));
	
	turnCode += @"
		if instance_exists(o?) {
			o?.x -= x 
			o?.y -= y
			ni = 0.9876883405951378 * o?.x + 0.15643446504023087 * o?.y
			nj = 0.9876883405951378 * o?.y - 0.15643446504023087 * o?.x
			o?.x = ni + x
			o?.y = nj + y
			o?.image_angle += 9
		}
	".Replace('?', (char)(i + 48));
	
	rebuildCode += (@"
		if instance_exists(o?) {
			ni = false
			switch (o?.sprite_index) {
				" + colourCases + @"
				case spr_toge:
					ni = instance_create(o?.x - 32, o?.y - 32, obj_toge2)
				break

				case spr_toge2:
					ni = instance_create(o?.x - 32, o?.y - 32, obj_toge)
				break

				case spr_spring:
					ni = instance_create(o?.x - 32, o?.y - 32, obj_spring2)
				break

				case spr_spring2:
					ni = instance_create(o?.x - 32, o?.y - 32, obj_spring)
				break

				case spr_reverseSwitch:
					if (o?.image_angle == 180)
						ni = instance_create(o?.x - 32, o?.y - 32, obj_e_blackSwitch)
					else
						ni = instance_create(o?.x, o?.y, obj_reverseSwitch)
				break
		
				case spr_graySwitch:
					if (o?.image_angle == 180)
						ni = instance_create(o?.x - 32, o?.y - 32, obj_e_graySwitchR)
					else
						ni = instance_create(o?.x, o?.y, obj_graySwitch)
				break
		
				case spr_whiteSwitch:
					if (o?.image_angle == 180)
						ni = instance_create(o?.x - 32, o?.y - 32, obj_e_whiteSwitchR)
					else
						ni = instance_create(o?.x, o?.y, obj_whiteSwich)
				break

				case spr_darkSwitch:
					ni = instance_create(o?.x, o?.y, obj_dawkSwtich)
					ni.image_angle = o?.image_angle
					with (obj_darkSwitchOn)
						if (o0 == other.o?)
							{ o0 = ni; break; }
				break
			}

			if ni {
				instance_destroy(o?)
				o? = ni
			}

			else
				o?.image_angle %= 360
		}
	").Replace('?', (char)(i + 48));
}

foreach (var o in "obj_player obj_head obj_switchBlock obj_switchBlockBlue obj_switchBlockGreen obj_switchBlockYellow obj_e_fairy obj_e_playerLike".Split(' ')) {
	ejectCode += "with (" + o + @") {
		if collision_rectangle(bbox_left, bbox_top, bbox_right, bbox_bottom, other.id, true, true) != noone {
			if bbox_top + bbox_bottom > 2 * other.y {
				if place_free(x, y - bbox_top + other.bbox_bottom + 1)
					y = y - bbox_top + other.bbox_bottom + 1
				else if place_free(x, y - bbox_bottom + other.bbox_top - 1)
					y = y - bbox_bottom + other.bbox_top - 1
			} else {
				if place_free(x, y - bbox_bottom + other.bbox_top - 1)
					y = y - bbox_bottom + other.bbox_top - 1
				else if place_free(x, y - bbox_top + other.bbox_bottom + 1)
					y = y - bbox_top + other.bbox_bottom + 1
			}
		}
	}
	";
}

obj.EventHandlerFor(EventType.Create, Data.Strings, Data.Code, Data.CodeLocals).ReplaceGML("us=true\nflip=false\nflipping=false\n" + createCode, Data);

obj.EventHandlerFor(EventType.Step, EventSubtypeStep.Step, Data.Strings, Data.Code, Data.CodeLocals).ReplaceGML(@"
	var ni
	var nj

	if flipping {
		image_angle += 9
		" + turnCode + @"
		
		with (obj_e_onmyoudamaCrawl) {
			if flipper == other.id {
				x += 16 - other.x
				y += 16 - other.y
				ni = 0.9876883405951378 * x + 0.15643446504023087 * y
				nj = 0.9876883405951378 * y - 0.15643446504023087 * x
				x = ni + other.x - 16
				y = nj + other.y - 16
			}
		}

		if image_angle % 180 == 0 {
			flipping = false
			flip = false
			us = true
			solid = true
			if (r0 == 1)
				r0 = 2
			else if (r0 == 2)
				r0 = 1
			with (obj_e_onmyoudamaCrawl) {
				if flipper == other.id {
					r1 = (r1 + 2) % 4
					x = round(x)
					y = round(y)
					if !place_free(x, y) {
						instance_create(x, y, obj_headEffect)
						audio_play_sound(se_hold, 10, false)
						instance_destroy()
					}
				}
			}
			" + rebuildCode + @"
		}
	}

	else if flip {
		flipping = true
		solid = false
		" + startCode + @"
	}

	else if (r0 % 2 && collision_rectangle(bbox_left, bbox_top - 7, bbox_right, bbox_top, obj_player, true, true) != noone) || (r0 > 1 && collision_rectangle(bbox_left, bbox_bottom, bbox_right, bbox_bottom + 7, obj_player, true, true) != noone)
		e_kill_player(false)

	if us {
		" + passiveCode + @"
		switch r0 {
			case 0:
				sprite_index = spr_e_flipperNone
			break

			case 1:
				sprite_index = spr_reverseFloor
				image_angle = 0
			break

			case 2:
				sprite_index = spr_reverseFloor
				image_angle = 180
			break

			case 3:
				sprite_index = spr_e_flipperBoth
			break
		}
	}
", Data);

obj.EventHandlerFor(EventType.Step, EventSubtypeStep.EndStep, Data.Strings, Data.Code, Data.CodeLocals).ReplaceGML($@"
	if us {{
		us = false
		{ejectCode}
	}}
", Data);

// === ICE BULLET CAMERA TRACKER ===

Data.GameObjects.Add(obj = new UndertaleGameObject() {
	Visible = false,
	Name = Data.Strings.MakeString("obj_e_iceBulletCamera"),
});

obj.EventHandlerFor(EventType.Step, EventSubtypeStep.EndStep, Data.Strings, Data.Code, Data.CodeLocals).ReplaceGML(@"
	if !instance_exists(obj_cameraTarget) {
		instance_destroy()
		exit
	}

	var f = false
	if instance_exists(o0) {
		f = collision_rectangle(o0.bbox_left, o0.bbox_top, o0.bbox_right, o0.bbox_bottom, obj_cannonBlockEnd, true, true) != noone
		obj_cameraTarget.x = o0.x
		obj_cameraTarget.y = o0.y
	} else if instance_exists(obj_iceBullet2) {
		o0 = obj_iceBullet2.id
	} else if instance_exists(obj_iceBullet3) {
		o0 = obj_iceBullet3.id
	} else if instance_exists(obj_iceBullet4) {
		o0 = obj_iceBullet4.id
	} else if instance_exists(obj_iceBullet5) {
		o0 = obj_iceBullet5.id
	} else {
		f = true
	}

	if f {
		instance_destroy()
		with (obj_cannonBlock)
			if (cannonFlag)
				exit
		with (obj_cannonBlockD)
			if (cannonFlag)
				exit
		with (obj_cannonBlockL)
			if (cannonFlag)
				exit
		with (obj_cannonBlockR)
			if (cannonFlag)
				exit
		with (obj_cannonBlockUD)
			if (cannonFlag)
				exit
		with (obj_cannonBlockLR)
			if (cannonFlag)
				exit
		global.playerCamera = true
		if (global.viewmode == 0)
			obj_gameMgr.playinput = true
		if instance_exists(obj_whiteSwich)
			obj_whiteSwich.flag = false
		instance_destroy(obj_cameraTarget)
	}
", Data);

// === SEIJA GRANT ITEM ===

Data.GameObjects.Add(obj = new UndertaleGameObject() {
	Name = Data.Strings.MakeString("obj_e_seijaGrantItem"),
	Visible = true,
});

obj.EventHandlerFor(EventType.Create, Data.Strings, Data.Code, Data.CodeLocals).ReplaceGML("t=0", Data);

obj.EventHandlerFor(EventType.Draw, Data.Strings, Data.Code, Data.CodeLocals).ReplaceGML("draw_sprite(spr_seijaItem, r0, x + 16, y + 16 + round(sin(t)))", Data);

obj.EventHandlerFor(EventType.Step, EventSubtypeStep.Step, Data.Strings, Data.Code, Data.CodeLocals).ReplaceGML(@"
	t += 0.08
	if collision_rectangle(x + 8, y + 8, x + 24, y + 24, obj_player, true, true) != noone {
		obj_gameMgr.seijaCan |= 1 << r0
		if (obj_gameMgr.seijaCan == 1 << r0)
			global.seijaItem = r0
		action_sound(33, 0)
		instance_destroy()
	}
", Data);

// === SEIJA PHOTO ===

Data.GameObjects.Add(obj = new UndertaleGameObject() {
	Name = Data.Strings.MakeString("obj_e_scItem"),
	Visible = true
});

obj.EventHandlerFor(EventType.Step, EventSubtypeStep.EndStep, Data.Strings, Data.Code, Data.CodeLocals).ReplaceGML("if (i != noone && !instance_exists(i))\ninstance_destroy()", Data);

obj.EventHandlerFor(EventType.Draw, Data.Strings, Data.Code, Data.CodeLocals).ReplaceGML(@"
	if (instance_exists(obj_e_seijaCamera) && obj_e_seijaCamera.visible)
		draw_sprite_ext(s, f, x + obj_e_seijaCamera.x + r * 32, y + obj_e_seijaCamera.y + r * 32, 1, 1, r * 180, c_white, 0.5)
", Data);

// === NO PHOTOGRAPHY SIGN ===

Data.GameObjects.Add(obj = new UndertaleGameObject() {
	Name = Data.Strings.MakeString("obj_e_noPhotography"),
	Visible = true,
	Sprite = Data.Sprites.ByName("spr_e_noPhotography"),
});

// === TAG BUTTON ===

Data.GameObjects.Add(obj = new UndertaleGameObject() {
	Name = Data.Strings.MakeString("obj_e_tag"),
	Visible = true
});

obj.EventHandlerFor(EventType.Create, Data.Strings, Data.Code, Data.CodeLocals).ReplaceGML("i=0\nr0=false\ns0=\"\"", Data);

obj.EventHandlerFor(EventType.Step, EventSubtypeStep.Step, Data.Strings, Data.Code, Data.CodeLocals).ReplaceGML(@"
	if (r0 && i < 15)
		i += 3
	else if (!r0 && i > 0)
		i -= 3
", Data);

obj.EventHandlerFor(EventType.Draw, Data.Strings, Data.Code, Data.CodeLocals).ReplaceGML(@"
	draw_set_colour(0xff0000 + 0x1111 * (15 - i))
	draw_rectangle(x, y, x + 120, y + 16, false)
	draw_set_font(font_message2)
	draw_set_color(0x111111 * i)
	draw_set_halign(fa_center)
	draw_set_valign(fa_top)
	draw_text_transformed(x + 60, y + 2, s0, 0.5, 0.5, 0)
	draw_set_halign(0)
", Data);

// === THROBBER ===

Data.GameObjects.Add(obj = new UndertaleGameObject() {
	Name = Data.Strings.MakeString("obj_e_throbber"),
	Visible = true,
	Sprite = Data.Sprites.ByName("spr_onmyoudama1_1")
});

obj.EventHandlerFor(EventType.Step, EventSubtypeStep.EndStep, Data.Strings, Data.Code, Data.CodeLocals).ReplaceGML("image_angle -= 5", Data);

// === WATCH SPEEDRUN TECHNIQUES ===

Data.GameObjects.Add(obj = new UndertaleGameObject() {
	Name = Data.Strings.MakeString("obj_e_watch_srt")
});

obj.EventHandlerFor(EventType.Create, Data.Strings, Data.Code, Data.CodeLocals).ReplaceGML("flag = false\ns0 = \"\"", Data);

obj.EventHandlerFor(EventType.Draw, EventSubtypeDraw.DrawGUI, Data.Strings, Data.Code, Data.CodeLocals).ReplaceGML(@"
	if flag {
		draw_set_halign(fa_center)
		draw_set_font(global.font)
		draw_set_color(c_red)
		draw_text(480, 8, s0)
		draw_set_halign(0)
		if obj_goal.bStageClear {
			e_send_event(247)
			instance_destroy()
		}
	}
", Data);

// === WALL ===

Data.GameObjects.Add(obj = new UndertaleGameObject() {
	Name = Data.Strings.MakeString("obj_e_wall"),
	Sprite = Data.Sprites.ByName("spr_wall"),
	ParentId = Data.GameObjects[0],
	Solid = true,
	Visible = false,
});

// === ROOM ===

UndertaleRoom room;
Data.Rooms.Add(room = new UndertaleRoom() {
	Name = Data.Strings.MakeString("rm_editor"),
	Speed = 60,
	BackgroundColor = 0x241212,
});

room.GameObjects.Add(new UndertaleRoom.GameObject() {
	ObjectDefinition = editor_obj
});

room.Views[0] = new UndertaleRoom.View() {
	Enabled = true,
	ViewWidth = 480,
	ViewHeight = 270,
	PortWidth = 960,
	PortHeight = 540,
	BorderX = 240,
	BorderY = 135
};
