use crate::state::LuaValue;
use std::cell::RefCell;
use std::rc::Rc;

#[allow(non_camel_case_types)]
pub trait luaState {
    fn get_top(&self) -> isize;

    fn push(&mut self, value: LuaValue);

    fn call(&mut self, nargs: isize, nresults: isize);

    fn is_integer(&self, index: isize) -> bool;
    fn is_function(&self, index: isize) -> bool;
}

#[allow(non_camel_case_types)]
pub type lua_State = Rc<RefCell<dyn luaState>>;

#[allow(non_snake_case)]
pub fn luaL_newstate() -> lua_State {
    let l = crate::state::LuaState::init();
    Rc::new(RefCell::new(l))
}

#[allow(non_snake_case)]
pub fn luaL_loadfile(l: lua_State, filename: &str) {
    let proto = crate::vm::read_chunk(filename);
    let closure = LuaValue::new_lua_closure(proto);
    l.borrow_mut().push(closure);
}

pub fn lua_call(l: lua_State, nargs: isize, nresults: isize) {
    l.borrow_mut().call(nargs, nresults)
}
