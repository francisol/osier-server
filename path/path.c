#include "path.h"
#include<string.h>
#include "lauxlib.h"
int path_join(lua_State *L)
{
    const char *parent = luaL_checkstring(L, 1);
    const char *child = luaL_checkstring(L, 2);
    char buf[2048];
    int len = strlen(parent);
    int index = 0;
    while (parent[len - 1] == '/')
    {
        --len;
    }
    int start = 0;
    while (child[start]!='/')
    {
        start++;
    }
    if (start==0)
    {
        lua_pushstring(L, child);
        return 1;
    }
    if (child[start-1]!='.')
    {
        start = 0;
    }else
    {
        start++;
    }
    strncat(buf, parent, len);
    strncat(buf, "/", 1);
    strcat(buf, child+start);
    while (buf[index]!='/')
    {
        ++index;
    }

    lua_pushstring(L, buf+index);
    return 1;
}

int path_relative(lua_State *L)
{
    const char *parent = luaL_checkstring(L, 1);
    const char *child = luaL_checkstring(L, 2);
    while (parent && child && *parent == *child)
    {
        ++parent;
        ++child;
    }
    while (child && *child == '/')
    {
        ++child;
    }
    lua_pushstring(L, child);
    return 1;
}

static const struct luaL_Reg pathlib[] = {
    {"join", path_join},
    {"relative", path_relative},
    {NULL, NULL},
};
int luaopen_path(lua_State *L)
{
    luaL_newlib(L, pathlib);
    return 1;
}