#include <iostream>
#include <algorithm>
#include <fstream>
#include <ctime>
#include <deque>
#include <map>
#include <vector>
#include <string.h>
#include <string>
#include <WinSock2.h>
#include <WS2tcpip.h>
#include <Windows.h>
#include <WinBase.h>
#include <stdint.h>
#include "sdk/public/steam/steam_api.h"

#pragma comment(lib,"ws2_32.lib")

#define EXT extern "C" __declspec(dllexport)
#define SOCK_DEAD {if(is_debug)sock=INVALID_SOCKET;else quit_requested=true;return;}

bool handle_internally(std::string);

bool quit_requested = false;
bool is_debug;

char env_buffer[32768];
std::string get_env(const char* varname, std::string default_value) {
	int result = GetEnvironmentVariableA(varname, env_buffer, 32768);
	if (result)
		return std::string(env_buffer);
	else
		return default_value;
}

std::ofstream* logFile = nullptr;

EXT void xlog(const char* msg) {
	std::cout << msg << std::endl;

	if (!logFile) {
		std::string filepath = get_env("BANKI_LOG_FILE", "banki.log");
		logFile = new std::ofstream(filepath.c_str());
	}

	if (logFile->good()) {
		logFile->write(msg, strlen(msg));
		logFile->write("\n", 1);
		logFile->flush();
	}
}

WSADATA wsadata;
bool wsadata_set = false;
SOCKET sock = INVALID_SOCKET;
u_long one = 1;
std::string portkey;
void ensure_connected() {
	if (sock != INVALID_SOCKET) return;

	xlog("Connecting to daemon");
	
	if (!wsadata_set) {
		xlog("Initiating WSA");
		int error = WSAStartup(MAKEWORD(2, 2), &wsadata);
		if (error) {
			std::string error_msg{ "WSA error " };
			error_msg += std::to_string(error);
			xlog(error_msg.c_str());
			quit_requested = true;
		}
		wsadata_set = true;
	}

	sock = socket(AF_INET, SOCK_STREAM, 0);

	struct sockaddr_in server;
	ZeroMemory(&server, sizeof(server));
	inet_pton(AF_INET, "127.0.0.1", &server.sin_addr.s_addr);
	server.sin_family = AF_INET;
	server.sin_port = htons(58008);

	const char* key = "0000000000000000";

	xlog("Reading portkey");
	if (portkey != "") {
		std::ifstream pkf{ portkey };
		if (pkf.good()) {
			char* pkb = new char[18];
			pkf.read(pkb, 18);
			memcpy(&server.sin_port, pkb, 2);
			key = pkb + 2;
			pkf.close();
			DeleteFileA(portkey.c_str());
		}
		else {
			xlog("Failed to read portkey");
			quit_requested = true;
			return;
		}
	}

	xlog("Connecting to socket");
	if (connect(sock, (SOCKADDR*)&server, sizeof(server))) {
		if (!is_debug) {
			int error = WSAGetLastError();
			std::string error_msg{ "Failed to connect to daemon: socket encountered error " };
			error_msg += std::to_string(error);
			xlog(error_msg.c_str());
			quit_requested = true;
		}
		return;
	}

	xlog("Sending key");
	int len = send(sock, key, 16, 0);
	if (len != 16) {
		if (len == SOCKET_ERROR) {
			int error = WSAGetLastError();
			std::string error_msg{ "Socket encountered error " };
			error_msg += std::to_string(error);
			xlog(error_msg.c_str());
		}

		else {
			std::string error_msg{ "Socket only sent " };
			error_msg += std::to_string(len);
			error_msg += " bytes";
			xlog(error_msg.c_str());
		}

		SOCK_DEAD;
	}

	xlog("ioctl");
	ioctlsocket(sock, FIONBIO, &one);
}

std::deque<std::string> command_queue, command_queue_2;
std::vector<std::string> on_room_load;

char sock_data[65536];
unsigned int sock_data_end = 0;
void read_queue() {
	ensure_connected();
	
	if (sock == INVALID_SOCKET) return;

	int len = recv(sock, sock_data + sock_data_end, 65536 - sock_data_end, 0);

	if (len == SOCKET_ERROR) {
		int error = WSAGetLastError();

		if (error == WSAEWOULDBLOCK) return; else {
			std::string error_msg{ "Socket encountered error " };
			error_msg += std::to_string(error);
			xlog(error_msg.c_str());
			SOCK_DEAD;
		}
	}

	if (!len) {
		xlog("Socket closed");
		SOCK_DEAD;
	}

	char* this_command_start = sock_data;
	int this_command_size = sock_data_end;
	for (unsigned int i = sock_data_end; i < sock_data_end + len; i++) {
		if (sock_data[i]) {
			this_command_size++;
		} else {
			command_queue_2.emplace_back(this_command_start, this_command_size);
			if (command_queue_2.back() == "_") {
				command_queue_2.pop_back();
				while (!command_queue_2.empty()) {
					command_queue.push_back(command_queue_2.front());
					command_queue_2.pop_front();
				}
			}
			this_command_start = sock_data + i + 1;
			this_command_size = 0;
		}
	}

	if (this_command_size && this_command_start > sock_data) {
		memmove(sock_data, this_command_start, this_command_size);
	}

	sock_data_end = this_command_size;
}

EXT const char* steam_name() {
	return SteamFriends()->GetPersonaName();
}

std::string last_return;
EXT const char* query() {
	read_queue();
	if (quit_requested) return "q";
	
	do {
		if (command_queue.empty()) return "";
		last_return = command_queue.front();
		command_queue.pop_front();
	} while (handle_internally(last_return));
	return last_return.c_str();
}

struct StackItem {
	int id;
	double height;
};

struct StackBuilderItem {
	StackItem stack_item;
	int down;
	int up;
};

std::map<uint64_t, double> tile_map;
std::vector<std::vector<StackItem>> stacks;
std::map<int, StackBuilderItem> stack_builder;

EXT void send_event(double event_d) {
	unsigned char event_i = (unsigned char)event_d;

	ensure_connected();

	if (!event_i) {
		for (std::string command : on_room_load) {
			command_queue.push_back(command);
		}

		tile_map.clear();
		stacks.clear();
		stack_builder.clear();
	}

	if (sock == INVALID_SOCKET)
		return;

at_send:
	
	if (send(sock, (char*)&event_i, 1, 0) == SOCKET_ERROR) {
		int error = WSAGetLastError();

		if (error == WSAEWOULDBLOCK) {
			YieldProcessor();
			goto at_send;
		} else {
			std::string error_msg{ "Socket encountered error " };
			error_msg += std::to_string(error);
			xlog(error_msg.c_str());
			SOCK_DEAD;
		}
	}
}

EXT void send_real(double real) {
	ensure_connected();

	if (sock == INVALID_SOCKET)
		return;

at_send:

	if (send(sock, (char*)&real, sizeof(double), 0) == SOCKET_ERROR) {
		int error = WSAGetLastError();

		if (error == WSAEWOULDBLOCK) {
			YieldProcessor();
			goto at_send;
		}
		else {
			std::string error_msg{ "Socket encountered error " };
			error_msg += std::to_string(error);
			xlog(error_msg.c_str());
			SOCK_DEAD;
		}
	}
}

EXT void send_string(const char* str) {

	int remaining = strlen(str);
	send_real(remaining);

	while (remaining) {
		int result = send(sock, str, remaining, 0);

		if (result == SOCKET_ERROR) {
			int error = WSAGetLastError();

			if (error == WSAEWOULDBLOCK) {
				YieldProcessor();
				continue;
			}
			else {
				std::string error_msg{ "Socket encountered error " };
				error_msg += std::to_string(error);
				xlog(error_msg.c_str());
				SOCK_DEAD;
				return;
			}
		}

		remaining -= result;
		str += result;
	}
}

std::vector<double> id_map;

EXT void set_instance_id(double ctrl_id, double gm_id) {
	unsigned int i_ctrl_id = (unsigned int)ctrl_id;
	if (id_map.size() <= i_ctrl_id)
		id_map.resize(i_ctrl_id + 1);
	id_map[i_ctrl_id] = gm_id;
}

EXT double get_instance_id(double ctrl_id) {
	return id_map[(int)ctrl_id];
}

EXT void set_tile_id(double x, double y, double id) {
	uint64_t xi = ((int)x) & 65535;
	uint64_t yi = ((int64_t)y) & 4294967295;
	tile_map[xi | (yi << 16)] = id;
}

EXT double get_tile_id(double x, double y) {
	uint64_t xi = ((int)x) & 65535;
	uint64_t yi = ((int64_t)y) & 4294967295;
	std::map<uint64_t, double>::iterator i = tile_map.find(xi | (yi << 16));
	if (i == tile_map.end())
		return -1;
	return i->second;
}

EXT void s_register(double self, double down, double up, double height) {
	StackBuilderItem sbi;
	sbi.up = (int)up;
	sbi.down = (int)down;
	sbi.stack_item.height = height;
	sbi.stack_item.id = (int)self;
	stack_builder[sbi.stack_item.id] = sbi;
}

void build_stacks() {
	if (stack_builder.size() != 0) {
		for (const auto& pair : stack_builder) {
			auto& sbi = pair.second;
			if (sbi.down <= 0 && sbi.up > 0) {
				std::vector<StackItem> stack;
				int key = sbi.stack_item.id;
				while (key > 0) {
					stack.push_back(stack_builder[key].stack_item);
					key = stack_builder[key].up;
				}
				stacks.push_back(stack);
			}
		}

		stack_builder.clear();
	}
}

EXT void s_remove(double self) {
	int id = (int)self;
	for (auto& stack : stacks) {
		for (size_t i = 0; i < stack.size(); i++) {
			if (stack[i].id == id) {
				if (i == 0) {
					stack.erase(stack.begin());
					return;
				}

				if (i < stack.size() - 2) {
					std::vector<StackItem> new_stack;
					for (size_t j = i + 1; j < stack.size(); j++)
						new_stack.push_back(stack[j]);
					stack.resize(i);
					stacks.push_back(new_stack);
					return;
				}

				stack.resize(i);
				return;
			}
		}
	}
}

EXT double s_base(double self) {
	build_stacks();
	int id = (int)self;
	int base;
	for (const auto& stack : stacks) {
		if (stack.size() == 0)
			continue;
		base = stack[0].id;
		for (const auto& si : stack)
			if (si.id == id)
				return base;
	}
	return self;
}

EXT double s_height(double self) {
	int id = (int)self;
	double height;
	for (const auto& stack : stacks) {
		height = 0;
		for (const auto& si : stack) {
			if (si.id == id)
				return height;
			height += si.height;
		}
	}
	return 0;
}

std::vector<double> paste_list;
EXT void paste_list_clear() {
	paste_list.clear();
}

EXT void paste_list_add(double i) {
	paste_list.push_back(i);
}

EXT double paste_list_contains(double i) {
	return (double)(std::find(paste_list.begin(), paste_list.end(), i) != paste_list.end());
}

double reals[16];

EXT double real(double i) {
	return reals[(int)i];
}

#define TAS_NONE 0
#define TAS_READ 1
#define TAS_WRITE 2

int tas_state = TAS_NONE;
std::string tas_fn;
std::vector<uint8_t> tas_data;
uint8_t tas_last, tas_count;
size_t tas_index;

EXT void tas_try_read(const char* fn) {
	tas_fn = fn;
	tas_state = TAS_READ;
}

EXT void tas_start(const char *path) {
	tas_index = 0;
	tas_count = 0;
	if (tas_state == TAS_WRITE) {
		tas_last = -1;
		tas_data.clear();
	}
	if (tas_state == TAS_READ && tas_fn.size()) {
		tas_data.clear();
		std::ifstream file(std::string(path) + tas_fn);
		tas_fn = "";
		if (file.good()) {
			file >> std::noskipws;
			std::copy(std::istream_iterator<uint8_t>(file), std::istream_iterator<uint8_t>(), std::back_inserter(tas_data));
			file.close();
		}
		else {
			tas_state = TAS_NONE;
		}
	}
}

EXT double tas_active() {
	return tas_state;
}

EXT void tas_add(double b7, double b6, double b5, double b4, double b3, double b2, double b1, double b0) {
	if (tas_state == TAS_WRITE) {
		uint8_t state =
			(b0 > 0.5) |
			(b1 > 0.5) << 1 |
			(b2 > 0.5) << 2 |
			(b3 > 0.5) << 3 |
			(b4 > 0.5) << 4 |
			(b5 > 0.5) << 5 |
			(b6 > 0.5) << 6 |
			(b7 > 0.5) << 7;

		if (state & 15) {
			tas_data.push_back(state);
		}
		else if (state != tas_last || tas_data.back() == 255) {
			tas_data.push_back(state);
			tas_data.push_back(0);
		}
		else {
			tas_data.back()++;
		}

		tas_last = state;
	}
}

EXT void tas_end(const char* path) {
	if (tas_state == TAS_WRITE && !tas_data.empty()) {
		std::ofstream file(std::string(path) + tas_fn);
		if (file.good()) {
			file.write((char*)&tas_data[0], std::streamsize(tas_data.size()));
			file.close();
		}
	}
}

EXT double tas_get() {
	if (tas_state == TAS_READ && tas_index < tas_data.size()) {
		uint8_t data = tas_data[tas_index];
		if (data & 15 || tas_index + 1 == tas_data.size()) {
			tas_index++;
		}
		else if (tas_count == tas_data[tas_index + 1]) {
			tas_index += 2;
			tas_count = 0;
		}
		else {
			tas_count++;
		}
		return data;
	}

	return -1;
}

bool onload_run = false;
EXT void onload(const char *gsi) {
	if (onload_run)
		return;

	onload_run = true;

	//is_debug = true;// get_env("BANKI_DEBUG", "")[0] == '1';

	xlog("BankiEdit 0.0.2");

	if (is_debug)
		xlog("Debug mode enabled");

	/*
	std::ifstream hostname_linux{ "z:\\etc\\hostname" };
	std::string hostname;

	if (hostname_linux.good()) {
		xlog("!! Running on Proton !!");
		char buffer[256];
		memset(buffer, 0, 256);
		hostname_linux.read(buffer, 255);
		hostname = std::string(buffer);
	}

	else {
		char buffer[MAX_COMPUTERNAME_LENGTH + 1];
		GetComputerNameA(buffer, (LPDWORD)MAX_COMPUTERNAME_LENGTH + 1);
		hostname = std::string(buffer);
	}

	while (hostname.find_first_of('\n') != std::string::npos)
		hostname.pop_back();

	hostname = "Running on " + hostname;
	xlog(hostname.c_str());
	*/

	std::ifstream passwd{ "z:\\etc\\passwd" };

	if (passwd.good()) {
		xlog("Running via Proton");
		passwd.close();
	}

	time_t now;
	struct tm nt;
	time(&now);
	gmtime_s(&nt, &now);
	char buf[sizeof "2011-10-08T07:07:09Z"];
	strftime(buf, sizeof buf, "%FT%TZ", &nt);
	std::string timemsg{ "Time: " };
	timemsg += buf;
	xlog(timemsg.c_str());
	xlog("----------------------------------------\n");

	std::string args{ "\"" };
	args += gsi;
	args += "\" ";
	args += std::to_string(SteamUser()->GetSteamID().ConvertToUint64());
	args += " ";
	args += std::to_string(GetCurrentProcessId());

	SECURITY_ATTRIBUTES sa;
	sa.nLength = sizeof(sa);
	sa.lpSecurityDescriptor = NULL;
	sa.bInheritHandle = TRUE;

	HANDLE h = CreateFileW(TEXT("daemon.log"),
		FILE_WRITE_DATA,
		FILE_SHARE_WRITE | FILE_SHARE_READ,
		&sa,
		CREATE_ALWAYS,
		FILE_ATTRIBUTE_NORMAL,
		NULL);

	STARTUPINFOA si;
	PROCESS_INFORMATION pi;
	ZeroMemory(&si, sizeof(si));
	si.cb = sizeof(si);
	si.dwFlags = STARTF_USESTDHANDLES;
	si.hStdInput = INVALID_HANDLE_VALUE;
	si.hStdError = h;
	si.hStdOutput = h;
	ZeroMemory(&pi, sizeof(pi));

	if (CreateProcessA("bankidaemon.exe", (LPSTR)args.c_str(), NULL, NULL, TRUE, 0, NULL, NULL, &si, &pi)) {
		portkey = gsi;
		portkey += "portkey";
		xlog("Starting daemon");
	}
	else {
		xlog("Unable to start daemon");
	}
}

bool handle_internally(std::string command) {
	size_t pos = 0;
	switch (command[0]) {
	case 'I':
		on_room_load.clear();
	case 'i':
		while (pos != std::string::npos) {
			size_t new_pos = command.find_first_of('\x1e', pos + 1);
			on_room_load.push_back(command.substr(pos + 1, new_pos - pos - 1));
			pos = new_pos;
		}
		return true;
	case 'l':
		xlog(command.c_str() + 1);
		return true;
	case 'f':
		for (unsigned int i = 0; i < command.length() / 8; i++) {
			float real;
			char* real_buffer = (char*)&real;

			for (int j = 0; j < 4; j++) {
				real_buffer[j] = ((command[i * 8 + j * 2 + 1] & 15) << 4)
				               | (command[i * 8 + j * 2 + 2] & 15);
			}

			reals[i] = real;
		}
		return true;
	case '[':
		if (tas_state = command.size() == 1 ? TAS_NONE : TAS_READ)
			tas_fn = command.substr(1);
		return true;
	case ']':
		if (tas_state = command.size() == 1 ? TAS_NONE : TAS_WRITE)
			tas_fn = command.substr(1);
		return true;

	default:
		return false;
	}
}