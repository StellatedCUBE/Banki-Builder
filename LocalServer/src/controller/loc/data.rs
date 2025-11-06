use crate::controller::menu::main_menu::BROWSE_TABS;

use super::MaybeLocalized;

pub const LOC_DELETE: MaybeLocalized = MaybeLocalized::Localized(
	"Delete",
	"削除",
	"删除"
);

pub const LOC_SWAP_ENDS: MaybeLocalized = MaybeLocalized::Localized(
	"Swap",
	"スワップ",
	"交换"
);

pub const LOC_INSERT_BEFORE: MaybeLocalized = MaybeLocalized::Localized(
	"Insert <",
	"",
	""
);

pub const LOC_INSERT_AFTER: MaybeLocalized = MaybeLocalized::Localized(
	"Insert >",
	"",
	""
);

pub const LOC_LOOP_PATH: MaybeLocalized = MaybeLocalized::Localized(
	"Loop",
	"",
	""
);

pub const LOC_UNLOOP_PATH: MaybeLocalized = MaybeLocalized::Localized(
	"Unloop",
	"",
	""
);

pub const LOC_REVERSE: MaybeLocalized = MaybeLocalized::Localized(
	"Reverse",
	"",
	""
);

pub const LOC_SELF: MaybeLocalized = MaybeLocalized::Localized(
	"You",
	"あなた",
	"你"
);

pub const LOC_UNKNOWN_USER: MaybeLocalized = MaybeLocalized::Localized(
	"Unknown",
	"無名",
	"某人"
);

pub const LOC_NOT_CONNECTED: MaybeLocalized = MaybeLocalized::Localized(
	"Unable to connect to server\n\nOnline features are unavailable",
	"サーバーに接続できません",
	"不能连接至服务器"
);

pub const LOC_OUT_OF_DATE: MaybeLocalized = MaybeLocalized::Localized(
	"An new version of Banki Builder is available\n\nRun the installer again to update",
	"",
	""
);

pub const LOC_CONNECTING: MaybeLocalized = MaybeLocalized::Localized(
	"Connecting to server...",
	"接続中",
	""
);

pub const LOC_ENTER_NAME: MaybeLocalized = MaybeLocalized::Localized(
	"Please enter your name",
	"名前を入力",
	"输入姓名"
);

pub const LOC_BAD_NAME: MaybeLocalized = MaybeLocalized::Localized(
	"Invalid name",
	"無効な名前です",
	"名称无效"
);

pub const LOC_NEED_STEAM_AUTH: MaybeLocalized = MaybeLocalized::Localized(
	"Log in with Steam to continue",
	"",
	""
);

pub const LOC_OPEN_BROWSER: MaybeLocalized = MaybeLocalized::Localized(
	"OPEN BROWSER",
	"",
	""
);

pub const LOC_NAME_LEVEL: MaybeLocalized = MaybeLocalized::Localized(
	"Enter a name for your level:",
	"",
	""
);

pub const LOC_PLAY_9_HEAD: MaybeLocalized = MaybeLocalized::Localized(
	"Play (9th Head Mode)",
	"",
	""
);

pub const LOC_EXPORT: MaybeLocalized = MaybeLocalized::Localized(
	"Export as file",
	"",
	""
);

pub const LOC_PUBLISH: MaybeLocalized = MaybeLocalized::Localized(
	"Publish",
	"",
	""
);

pub const LOC_UNPUBLISH: MaybeLocalized = MaybeLocalized::Localized(
	"Unpublish",
	"",
	""
);

pub const LOC_SURE: MaybeLocalized = MaybeLocalized::Localized(
	"Are you sure?",
	"",
	""
);

pub const LOC_SURE_Y: MaybeLocalized = MaybeLocalized::Localized(
	"YES",
	"",
	""
);

pub const LOC_SURE_N: MaybeLocalized = MaybeLocalized::Localized(
	"NO",
	"",
	""
);

pub const LOC_PUBLISH_CLEAR_REQUIRED: MaybeLocalized = MaybeLocalized::Localized(
	"To publish a level, you must first beat it.\nIf you use any speedrun techniques here, your level will\nautomatically have the \"Advanced Techniques\" tag.",
	"",
	""
);

pub const LOC_PUBLISH_CLEAR_REQUIRED_PP: MaybeLocalized = MaybeLocalized::Localized(
	"You must collect the puzzle piece.",
	"",
	""
);

pub const LOC_PUBLISH_ATTEMPT: MaybeLocalized = MaybeLocalized::Localized(
	"ATTEMPT",
	"",
	""
);

pub const LOC_PUBLISH_COMMIT: MaybeLocalized = MaybeLocalized::Localized(
	"SUBMIT",
	"",
	""
);

pub const LOC_TAG_HEADER: MaybeLocalized = MaybeLocalized::Localized(
	"TAGS",
	"タグ",
	"标签"
);

pub const LOC_TAG_MANDATORY: MaybeLocalized = MaybeLocalized::Localized(
	"Mandatory Tags\n(You must select these if they apply to your level)",
	"",
	""
);

pub const LOC_TAG_OTHER: MaybeLocalized = MaybeLocalized::Localized(
	"Additional Tags",
	"",
	""
);

pub const LOC_PUBLISH_UPLOADING: MaybeLocalized = MaybeLocalized::Localized(
	"Uploading...",
	"",
	""
);

pub const LOC_PUBLISH_FAILED: MaybeLocalized = MaybeLocalized::Localized(
	"There was an error when publishing your level.",
	"",
	""
);

pub const LOC_PUBLISH_BACK: MaybeLocalized = MaybeLocalized::Localized(
	"BACK",
	"",
	""
);

pub const LOC_PUBLISH_SUCCESS_HEADER: MaybeLocalized = MaybeLocalized::Localized(
	"SUCCESS",
	"",
	""
);

pub const LOC_PUBLISH_SUCCESS: MaybeLocalized = MaybeLocalized::Localized(
	"%\nhas been published!\n\n\nLevel code:",
	"%",
	"%"
);

pub const LOC_BROWSE_TAB: [MaybeLocalized; BROWSE_TABS] = [
	MaybeLocalized::Localized(
		"New",
		"",
		""
	),
	MaybeLocalized::Localized(
		"Top",
		"",
		""
	),
	MaybeLocalized::Localized(
		"Enter ID",
		"",
		""
	),
];

pub const LOC_ENTER_ID_HEADER: MaybeLocalized = MaybeLocalized::Localized(
	"Enter ID:",
	"",
	""
);

pub const LOC_ENTER_ID_SUBMIT: MaybeLocalized = MaybeLocalized::Localized(
	"Search",
	"",
	""
);

pub const LOC_NO_RESULTS: MaybeLocalized = MaybeLocalized::Localized(
	"No levels found",
	"",
	""
);

pub const LOC_NETWORK_ERR: MaybeLocalized = MaybeLocalized::Localized(
	"Network Error",
	"",
	""
);

pub const LOC_SRT_WARNING: MaybeLocalized = MaybeLocalized::Localized(
	"SPEEDRUN TECHNIQUES DETECTED!",
	"",
	""
);