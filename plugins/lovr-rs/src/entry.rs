// entry point

use mlua::prelude::*;
use crate::game::Game;
use crate::lovr;
use crate::lovr::LovrCallbacks;

#[mlua::lua_module]
fn lovr_rs(lua: &Lua) -> LuaResult<LuaValue> {
    lua.globals()
        .get::<LuaTable>("lovr")?
        .set("run", lua.create_function(lovr_run)?)?;

    Ok(LuaNil)
}

fn lovr_run(lua: &Lua, _: ()) -> LuaResult<LuaFunction> {
    let game = Box::new(Game);
    let mut game_loop = game.run()?;
    lua.create_function_mut(move |_lua, _: ()| -> LuaResult<LuaValue> {
        match game_loop()? {
            lovr::RunCommand::Quit(code) => Ok(LuaValue::Number(code.into())),
            lovr::RunCommand::Continue => Ok(LuaNil),
        }
    })
}

