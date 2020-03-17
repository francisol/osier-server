
#include "lua_extend.h"
#include "path/path.h"
static const luaL_Reg extendlibs[] = {
  {"lfs", luaopen_lfs},
  {"lpath", luaopen_path},
  {NULL, NULL}
};


LUALIB_API void luaL_openExtendlibs (lua_State *L) {
  const luaL_Reg *lib;
  /* "require" functions from 'loadedlibs' and set results to global table */
  for (lib = extendlibs; lib->func; lib++) {
    luaL_requiref(L, lib->name, lib->func, 1);
    lua_pop(L, 1);  /* remove lib */
  }
}