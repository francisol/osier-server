
#ifndef EXTEND_LUA
#define EXTEND_LUA
#include "lprefix.h"


#include <stddef.h>

#include "lua.h"

#include "lualib.h"
#include "lauxlib.h"
#include "lfs/lfs.h"


// static const luaL_Reg extendlibs[] = {
//   {"lfs", luaopen_lfs},
//   {NULL, NULL}
// };

LUALIB_API void luaL_openExtendlibs(lua_State *L);
#endif