#include <lua.hpp>
#include <lauxlib.h>
#include <lualib.h>
 
extern "C" {
  int tst() {
    lua_State* L = lua_open();
    return LUA_VERSION_NUM;
  }
}
