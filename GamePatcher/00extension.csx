
UndertaleExtension ext;
Data.Extensions.Add(ext = new UndertaleExtension() {
	Name = Data.Strings.MakeString("banki_edit"),
	ClassName = Data.Strings.MakeString(""),
	FolderName = Data.Strings.MakeString(".")
});

UndertaleExtensionFile file;
ext.Files.Add(file = new UndertaleExtensionFile() {
	Filename = Data.Strings.MakeString("bankibuilder.dll"),
	Kind = UndertaleExtensionKind.Dll
});

byte none = 0, real = 1, str = 2;
uint next_func_id = 100;

void AddFunc(string name, byte ret, params byte[] args) {
	UndertaleExtensionFunction func;
	file.Functions.Add(func = new UndertaleExtensionFunction() {
		Name = Data.Strings.MakeString("e_" + name),
		ExtName = Data.Strings.MakeString(name),
		ID = next_func_id++,
		Kind = 11,
		RetType = ret == str ? UndertaleExtensionVarType.String : UndertaleExtensionVarType.Double
	});

	foreach (byte arg in args) {
		func.Arguments.Add(new UndertaleExtensionFunctionArg(arg == str ? UndertaleExtensionVarType.String : UndertaleExtensionVarType.Double));
	}
}

AddFunc("onload", none, str);
AddFunc("send_event", none, real);
AddFunc("query", str);
AddFunc("xlog", none, str);
AddFunc("real", real, real);
AddFunc("set_instance_id", none, real, real);
AddFunc("get_instance_id", real, real);
AddFunc("send_real", none, real);
AddFunc("send_string", none, str);
AddFunc("set_tile_id", none, real, real, real);
AddFunc("get_tile_id", real, real, real);
AddFunc("s_register", none, real, real, real, real);
AddFunc("s_remove", none, real);
AddFunc("s_base", real, real);
AddFunc("s_height", real, real);
AddFunc("paste_list_clear", none);
AddFunc("paste_list_add", none, real);
AddFunc("paste_list_contains", real, real);
AddFunc("steam_name", str);
AddFunc("tas_start", none, str);
AddFunc("tas_active", real);
AddFunc("tas_get", real);
AddFunc("tas_add", none, real, real, real, real, real, real, real, real);
AddFunc("tas_end", none, str);
AddFunc("tas_try_read", none, str);

var newProductID = new byte[] { 0xBA, 0x5E, 0xBA, 0x11, 0xBA, 0xDD, 0x06, 0x60, 0xBE, 0xEF, 0xED, 0xBA, 0x0B, 0xAB, 0xBA, 0xBE };
Data.FORM.EXTN.productIdData.Add(newProductID);
