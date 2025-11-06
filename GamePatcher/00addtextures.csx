/*
int i = 0;
StringBuilder b = new StringBuilder();
foreach (UndertaleSprite s in Data.Sprites) {
	b.Append($"{i}: {s.Name.Content}\n");
	i++;
}
File.WriteAllText("sprites.txt", b.ToString());
*/

for (int i = 0; i < Data.EmbeddedTextures.Count; i++)
	if (File.Exists($"gfx/{i}.png"))
		Data.EmbeddedTextures[i].TextureData = new UndertaleEmbeddedTexture.TexData() { TextureBlob = File.ReadAllBytes($"gfx/{i}.png") };

UndertaleEmbeddedTexture embTex = new UndertaleEmbeddedTexture() {
	Name = Data.Strings.MakeString("Editor"),
	TextureData = new UndertaleEmbeddedTexture.TexData() {
		TextureBlob = File.ReadAllBytes("gfx/sheet.png")
	}
};

Data.EmbeddedTextures.Add(embTex);

UndertaleEmbeddedTexture embTex2 = new UndertaleEmbeddedTexture() {
	Name = Data.Strings.MakeString("EditorBG"),
	TextureData = new UndertaleEmbeddedTexture.TexData() {
		TextureBlob = File.ReadAllBytes("gfx/bg.png")
	}
};

Data.EmbeddedTextures.Add(embTex2);

UndertaleTexturePageItem AddTexture(ushort x, ushort y, ushort w, ushort h, UndertaleEmbeddedTexture t) {
	UndertaleTexturePageItem tpi;
	Data.TexturePageItems.Add(tpi = new UndertaleTexturePageItem() {
		SourceX = x,
		SourceY = y,
		SourceWidth = w,
		SourceHeight = h,
		TargetX = 0,
		TargetY = 0,
		TargetWidth = w,
		TargetHeight = h,
		BoundingWidth = w,
		BoundingHeight = h,
		TexturePage = t
	});
	return tpi;
}

Data.Sprites.ByName("spr_ice2").MarginTop = 1;
Data.Sprites.ByName("spr_ice2").Textures.Add(new UndertaleSprite.TextureEntry() {
	Texture = AddTexture(833, 896, 32, 32, embTex)
});

int spr_no = 0, bg_no = 0;

void AddSprite(ushort x, ushort y, ushort w, ushort h, string name = null, string copyMask = null) {
	UndertaleSprite spr;
	Data.Sprites.Add(spr = new UndertaleSprite() {
		Name = Data.Strings.MakeString("spr_e_" + (spr_no++).ToString()),
		Width = w,
		Height = h,
	});
	if (name != null)
		spr.Name = Data.Strings.MakeString("spr_e_" + name);
	if (copyMask != null) {
		var me = Data.Sprites.ByName(copyMask).CollisionMasks[0];
		spr.CollisionMasks.Add(new UndertaleSprite.MaskEntry() { Data = me.Data });
	}
	spr.Textures.Add(new UndertaleSprite.TextureEntry() {
		Texture = AddTexture(x, y, w, h, embTex)
	});
}

void AddSpriteC(ushort x, ushort y, ushort w, ushort h, string name = null) {
	UndertaleSprite spr;
	Data.Sprites.Add(spr = new UndertaleSprite() {
		Name = Data.Strings.MakeString("spr_e_" + (spr_no++).ToString()),
		Width = w,
		Height = h,
		OriginX = w >> 1,
		OriginY = h >> 1,
	});
	if (name != null)
		spr.Name = Data.Strings.MakeString("spr_e_" + name);
	spr.Textures.Add(new UndertaleSprite.TextureEntry() {
		Texture = AddTexture(x, y, w, h, embTex)
	});
}

void AddSpriteM(ushort x, ushort y, ushort w, ushort h, ushort ox, ushort oy, int ml, int mr, int mb, int mt, string name) {
	UndertaleSprite spr;
	Data.Sprites.Add(spr = new UndertaleSprite() {
		Name = Data.Strings.MakeString("spr_e_" + name),
		Width = w,
		Height = h,
		OriginX = ox,
		OriginY = oy,
		MarginLeft = ml,
		MarginRight = mr,
		MarginTop = mt,
		MarginBottom = mb,
	});
	spr.Textures.Add(new UndertaleSprite.TextureEntry() {
		Texture = AddTexture(x, y, w, h, embTex)
	});
	spr_no++;
}

void XSprite(string sprite, int start) {
	var src = Data.Sprites.ByName(sprite);
	for (int i = start; i < src.Textures.Count; i++) {
		var spr = new UndertaleSprite() {
			Name = Data.Strings.MakeString("spr_e_" + (spr_no++).ToString()),
			Width = src.Width,
			Height = src.Height,
		};
		spr.Textures.Add(new UndertaleSprite.TextureEntry() {
			Texture = src.Textures[i].Texture
		});
		Data.Sprites.Add(spr);
	}
}

void AddBG(ushort x, ushort y, ushort w, ushort h, ushort xy = 0) {
	var tex = AddTexture(x, y, w, h, embTex2);
	if (xy != 0) {
		tex.BoundingHeight = xy;
	}
	Data.Backgrounds.Add(new UndertaleBackground() {
		Name = Data.Strings.MakeString("bg_e_" + (bg_no++).ToString()),
		Texture = tex
	});
}

void Bamboo(int i) {
	UndertaleTexturePageItem bg = Data.Backgrounds[i].Texture;
	UndertaleTexturePageItem bg2 = AddTexture(bg.SourceX, (ushort)(bg.SourceY + 192), bg.SourceWidth, 24, bg.TexturePage);
	bg2.TargetX = bg.TargetX;
	bg2.BoundingWidth = bg.BoundingWidth;
	Data.Backgrounds.Add(new UndertaleBackground() {
		Name = Data.Strings.MakeString("background_e_bamboo_" + i.ToString()),
		Texture = bg2
	});
}

Bamboo(46);
Bamboo(47);
Bamboo(48);
Bamboo(57);
Bamboo(58);
Bamboo(59);

var fnt = Data.Sprites.ByName("spr_font_number").Textures;
fnt.Add(new UndertaleSprite.TextureEntry() { Texture = AddTexture(961, 933, 16, 20, embTex) });

Console.WriteLine($"SPRITE BASE: {Data.Sprites.Count}");
Console.WriteLine($"BG BASE: {Data.Backgrounds.Count}");

AddSprite(480, 752, 211, 62);   // SEIJA_GAME
AddSprite(480, 585, 141, 77);   // BUTTON_SELECT_BIG
AddSprite(480, 270, 11, 15);    // CURSOR
AddSprite(492, 270, 34, 34);    // BUTTON_SELECT_CIRCLE
AddSprite(621, 585, 67, 63);    // BUTTON_SELECT_SMALL
AddSprite(526, 270, 480, 40);   // EDITOR_TOP_BAR
AddSprite(724, 585, 16, 16);    // TOOL_SMALL
AddSprite(1648, 585, 400, 270); // LEVEL_SETTINGS_MENU_BG
AddSprite(822, 613, 28, 6);     // TILE_SPLIT_HORIZONTAL
AddSprite(688, 585, 35, 34);    // TOOL_SMALL_ICONS
AddSprite(723, 585, 36, 36);    // TOOL
AddSprite(791, 585, 31, 31);    // POOF
AddSprite(688, 618, 14, 15);    // DELETE
AddSprite(929, 864, 32, 32);    // BG_TILE_CIRNO_FULL
AddSprite(480, 285, 12, 10);    // EDIT_BUTTON
AddSprite(966, 798, 9, 9);      // SEIJA_NO
AddSprite(705, 621, 240, 180);  // CREATE_BG
AddSprite(480, 707, 225, 45);   // LEVEL_SELECT
AddSprite(737, 801, 32, 32);    // BG_TILE_1
AddSprite(705, 801, 32, 32);    // BG_TILE_2
AddSprite(801, 801, 32, 32);    // BG_TILE_3
AddSprite(833, 801, 32, 32);    // BG_TILE_4
AddSprite(865, 801, 32, 32);    // BG_TILE_5
AddSprite(897, 801, 32, 32);    // BG_TILE_6
AddSprite(801, 833, 32, 32);    // BG_TILE_7
AddSprite(833, 833, 32, 32);    // BG_TILE_8
AddSprite(897, 833, 32, 32);    // BG_TILE_9
AddSprite(929, 832, 32, 32);    // BG_TILE_10
AddSprite(759, 585, 32, 32);    // GROUND_BLOCK_1
AddSprite(851, 585, 32, 32);    // GROUND_BLOCK_2
AddSprite(769, 801, 32, 32);    // GROUND_BLOCK_3
AddSprite(882, 585, 32, 32);    // GROUND_BLOCK_4
AddSprite(769, 832, 32, 32);    // GROUND_BLOCK_5
AddSprite(737, 833, 32, 32);    // GROUND_BLOCK_6
AddSprite(706, 833, 32, 32);    // GROUND_BLOCK_7
AddSprite(865, 833, 32, 32);    // GROUND_BLOCK_8
AddSprite(913, 585, 32, 32);    // GROUND_BLOCK_9
AddSprite(929, 800, 32, 32);    // GROUND_BLOCK_10
AddSprite(705, 865, 32, 32);    // GROUND_BLOCK_JERRY
AddSprite(769, 864, 32, 32);    // BG_TILE_STARS
AddSprite(737, 865, 32, 32);    // GROUND_BLOCK_STARS
AddSprite(801, 865, 32, 32);    // BG_TILE_MINDBREAK
AddSprite(865, 864, 32, 32);    // GROUND_BLOCK_MINDBREAK
AddSprite(833, 865, 32, 32);    // GROUND_BLOCK_FIREFLIES
AddSprite(945, 585, 32, 32);    // GROUND_BLOCK_CIRNO
AddSprite(945, 647, 32, 32);    // GROUND_BLOCK_RUMIA
AddSprite(945, 679, 32, 32);    // BG_TILE_RUMIA
AddSprite(897, 865, 32, 32);    // BG_TILE_FIREFLIES
AddSprite(945, 711, 32, 32);    // GROUND_BLOCK_PURPLE
AddSprite(945, 743, 32, 32);    // BG_TILE_PURPLE
AddSprite(945, 616, 32, 32);    // BG_TILE_CIRNO
AddSprite(945, 673, 32, 6);     // RUMIA_DANGLE
AddSprite(1024, 0, 1005, 900);  // MAIN_MENU_JP
AddSprite(0, 0, 1005, 900);     // MAIN_MENU_EN
AddSprite(0, 1024, 1005, 900);  // MAIN_MENU_ZH
AddSprite(162, 900, 153, 54);   // CIRNO_GAME
AddSprite(162, 954, 228, 22);   // RUMIA_GAME
AddSprite(162, 976, 32, 32);    // ALT_GROUND_1
AddSprite(193, 976, 32, 32);    // ALT_GROUND_2
AddSprite(347, 900, 32, 32);    // ALT_GROUND_3
AddSprite(976, 585, 32, 32);    // ALT_GROUND_4
AddSprite(224, 976, 32, 32);    // ALT_GROUND_5
AddSprite(255, 976, 32, 32);    // ALT_GROUND_6
AddSprite(410, 900, 32, 32);    // ALT_GROUND_7
AddSprite(317, 976, 32, 32);    // ALT_GROUND_8
AddSprite(378, 900, 32, 32);    // ALT_GROUND_PURPLE
AddSprite(348, 976, 32, 32);    // ALT_GROUND_10
AddSprite(315, 900, 32, 32);    // ALT_GROUND_JERRY
AddSprite(379, 976, 32, 32);    // ALT_GROUND_RED
AddSprite(1638, 855, 410, 170); // TOOL_DROPDOWN_BG
AddSprite(822, 585, 8, 14);     // TOOL_DROPDOWN_BUTTON
AddSprite(822, 599, 8, 14);     // TOOL_DROPDOWN_BUTTON_RETRACT
AddSprite(830, 585, 6, 28);     // TILE_SPLIT_VERTICAL
AddSprite(945, 775, 30, 19);    // TILE_SPLIT_TOOL
AddSprite(315, 932, 25, 18);    // SWITCH_RED_R
AddSprite(340, 932, 25, 18);    // SWITCH_BLUE_R
AddSprite(365, 932, 25, 18);    // SWITCH_GREEN_R
AddSprite(390, 932, 25, 18);    // SWITCH_YELLOW_R
AddSprite(442, 900, 32, 32);    // CANNON_RED_DOWN
AddSprite(442, 931, 32, 32);    // CANNON_RED_LEFT
AddSprite(442, 962, 32, 32);    // CANNON_RED_RIGHT
AddSprite(688, 633, 15, 15);    // SWAP
AddSprite(929, 896, 32, 32);    // CHANDELIER_TOOL
AddSprite(414, 931, 7, 7);      // BUTTON_PLUS
AddSprite(845, 593, 6, 8);      // BUTTON_UP_OFF
AddSprite(845, 601, 6, 8);      // BUTTON_UP_ON
AddSprite(836, 585, 6, 8);      // BUTTON_DOWN_OFF
AddSprite(842, 585, 6, 8);      // BUTTON_DOWN_ON
AddSprite(836, 593, 8, 6);      // BUTTON_LEFT_OFF
AddSprite(836, 599, 8, 6);      // BUTTON_LEFT_ON
AddSprite(427, 932, 8, 6);      // BUTTON_RIGHT_OFF
AddSprite(836, 605, 8, 6);      // BUTTON_RIGHT_ON
AddSprite(420, 931, 7, 7);      // BUTTON_MINUS
AddSprite(415, 938, 16, 15,       "frog");
AddSprite(390, 950, 15, 15);    // CLOCKWISE
AddSprite(649, 648, 14, 14);    // BUTTON_FACE_BOTTOM
AddSprite(663, 648, 14, 14);    // BUTTON_FACE_LEFT
AddSprite(677, 648, 14, 14);    // BUTTON_FACE_TOP
AddSprite(691, 648, 14, 14);    // BUTTON_FACE_RIGHT
AddSprite(897, 897, 32, 28);    // CRAWL_CLOCKWISE
AddSprite(867, 896, 30, 28);    // CRAWL_ANTICLOCKWISE
AddSprite(705, 897, 32, 32);    // BLOCK_RED
AddSprite(737, 897, 32, 32);    // BLOCK_BLUE
AddSprite(769, 896, 32, 32);    // BLOCK_PURPLE
AddSprite(801, 897, 32, 32);    // BLOCK_GREEN
AddSprite(286, 976, 32, 32);    // ALT_GROUND_WHITE
AddSprite(975, 775, 32, 32);    // ALT_GROUND_BROWN
AddSpriteC(924, 928, 37, 35);   // SYM_CIRCLE
AddSpriteC(961, 807, 36, 33);   // SYM_ARROW_AA
AddSpriteC(894, 925, 30, 28);   // SYM_ARROW_DIAG
AddSprite(834, 928, 31, 29);    // TOOL_SYMBOL
AddSprite(405, 953, 16, 16);    // SYM_ICON_CIRCLE
AddSprite(411, 969, 15, 16);    // SYM_ICON_ARROW_U
AddSprite(421, 953, 16, 16);    // SYM_ICON_ARROW_UR
AddSprite(977, 760, 16, 15);    // SYM_ICON_ARROW_R
AddSprite(426, 985, 16, 15);    // SYM_ICON_ARROW_DR
AddSprite(426, 969, 15, 16);    // SYM_ICON_ARROW_D
AddSprite(411, 985, 16, 15);    // SYM_ICON_ARROW_DL
AddSprite(977, 745, 16, 15);    // SYM_ICON_ARROW_L
AddSprite(977, 729, 16, 16);    // SYM_ICON_ARROW_UL
AddSprite(961, 840, 30, 32,       "flanBonk");
AddSpriteM(1952, 277, 96, 8, 48, 4, 0, 95, 6, 1, "flipperNone");
AddSpriteM(1952, 270, 96, 22, 48, 11, 0, 95, 14, 8, "flipperBoth");
AddSprite(769, 928, 32, 20);    // TOOL_FLIPPER_NONE
AddSprite(801, 929, 32, 20);    // TOOL_FLIPPER_UP
AddSprite(769, 928, 32, 27);    // TOOL_FLIPPER_DOWN
AddSprite(801, 929, 32, 27);    // TOOL_FLIPPER_BOTH
AddSprite(865, 924, 25, 32);    // BLACK_SWITCH
AddSprite(442, 994, 25, 18);    // SWITCH_GREY_R
AddSprite(977, 681, 25, 18);    // SWITCH_WHITE_R
AddSprite(977, 698, 23, 32);    // DARKSWITCH_R
AddSprite(2030, 292, 18, 18);   // TOOL_ZOOMED
AddSprite(709, 586, 14, 11);    // TOOL_PAN
AddSprite(689, 586, 14, 14);    // TOOL_MOVE
AddSprite(689, 605, 13, 13);    // TOOL_SELECT
AddSprite(635, 648, 14, 14);    // ZOOM_IN
AddSprite(977, 617, 29, 24,       "cirnoHead");
AddSprite(977, 641, 27, 26,       "rumiaHead");
AddSprite(961, 872, 27, 24,       "seijaHead");
AddSprite(0, 900, 29, 24,         "cirnoOutline");
AddSprite(29, 900, 27, 26,        "rumiaOutline");
AddSprite(56, 900, 27, 24,        "seijaOutline");
AddSprite(962, 897, 24, 12,       "cirnoHold");
AddSprite(962, 910, 23, 22,       "rumiaHold");
AddSprite(106, 900, 24, 16);    // SEIJA_BUTTON_LEFT
AddSprite(83, 900, 24, 16);     // SEIJA_BUTTON_RIGHT
AddSprite(130, 900, 16, 24);    // SEIJA_BUTTON_UP
AddSprite(146, 900, 16, 24);    // SEIJA_BUTTON_DOWN
XSprite("spr_seijaItem", 0);    // SEIJA_HIRARINUNO
////////////////////////////    // SEIJA_CAMERA
////////////////////////////    // SEIJA_BOMB
////////////////////////////    // SEIJA_HAMMER
AddSprite(0, 925, 24, 28,         "seijaBonk");
AddSpriteC(739, 929, 30, 31,      "noPhotography");
AddSprite(147, 924, 15, 16);    // SYM_ICON_NO_PHOTOGRAPHY
AddSprite(480, 662, 225, 45);   // LEVEL_BANKI
AddSprite(480, 814, 225, 45);   // LEVEL_CIRNO
AddSprite(480, 859, 225, 45);   // LEVEL_RUMIA
AddSprite(480, 904, 225, 45);   // LEVEL_SEIJA
AddSprite(705, 929, 28, 30);    // TOOL_DELETE
AddSprite(480, 1920, 224, 128); // LEVEL_EXTRA
AddSprite(539, 949, 166, 75);   // CONFIRM_MENU
AddSprite(474, 900, 6, 10);     // R_ARROW
AddSprite(480, 949, 60, 60);    // PUBLISH
AddSprite(704, 1864, 225, 184); // BROWSE_BG
AddSprite(411, 1000, 9, 9);     // ICON_HAX
AddSprite(420, 1000, 9, 9);     // ICON_SPEEDRUN
AddSprite(429, 1000, 9, 9);     // ICON_TROLL
AddSprite(82, 916, 30, 30);     // GREY_CIRCLE
AddSprite(621, 648, 14, 14);    // PLUS1
AddSprite(621, 648, 14, 14);    // PLUS2
AddSprite(132, 924, 15, 16);    // LOOP
AddSpriteC(820, 618, 5, 3);     // SMALL_ARROW
AddSprite(696, 752, 10, 16);    // SYM_ICON_VCORD
AddSprite(112, 916, 16, 10);    // SYM_ICON_HCORD
AddSpriteC(702, 752, 4, 32);    // CORD

AddBG(0, 0, 640, 480);       // STAR_WHITE
AddBG(640, 0, 640, 480);     // STAR_COMMON
AddBG(1280, 0, 640, 480);    // STAR_RED
AddBG(0, 480, 640, 480);     // STAR_PURPLE
AddBG(0, 960, 128, 384);     // TILES
AddBG(641, 481, 640, 320);   // FOM TOP
