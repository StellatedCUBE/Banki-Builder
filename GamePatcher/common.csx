using System.Text;
using System.Linq;

Data.ToolInfo.AppDataProfiles = @"z:\tmp\umt_appdata\";

void MakeScript(string name, string gml, int args, int locals) {
	UndertaleString ns = Data.Strings.MakeString(name);

	UndertaleCode code = new UndertaleCode() {
		ArgumentsCount = (ushort)args,
		LocalsCount = (uint)locals,
		Name = ns
	};

	Data.Code.Add(code);

	code.ReplaceGML(gml, Data);

	UndertaleScript script = Data.Scripts.ByName(name);

	if (script != null) {
		script.Code = code;
		return;
	}
	
	script = new UndertaleScript() {
		Name = ns,
		Code = code
	};

	Data.Scripts.Add(script);
}

