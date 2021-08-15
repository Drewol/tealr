use std::marker::PhantomData;

use mlua::{FromLuaMulti, Lua, MetaMethod, Result, ToLuaMulti, UserData, UserDataMethods};

use super::{MaybeSend, TealDataMethods};
use crate::TealMultiValue;

///Used to turn [UserDataMethods](mlua::UserDataMethods) into [TealDataMethods](crate::mlu::TealDataMethods).
///
///This allows you to easily implement [UserData](mlua::UserData) by wrapping the [UserDataMethods](mlua::UserDataMethods) in this struct
///and then passing it to the TealData implementation
///
pub struct UserDataWrapper<'a, 'lua, Container, T>
where
    Container: UserDataMethods<'lua, T>,
    T: UserData,
{
    cont: &'a mut Container,
    _t: std::marker::PhantomData<(&'a (), T)>,
    _x: &'lua std::marker::PhantomData<()>,
    //_t: &'lua PhantomData<T>,
}
impl<'a, 'lua, Container, T> UserDataWrapper<'a, 'lua, Container, T>
where
    Container: UserDataMethods<'lua, T> + 'a,
    T: UserData,
{
    ///wraps it.
    ///```
    ///# use mlua::{Lua, Result, UserData, UserDataMethods};
    ///# use tealr::{mlu::{TealData, TealDataMethods,UserDataWrapper}, TypeWalker,  TypeName,};
    /// struct Example {}
    /// impl TealData for Example {}
    /// impl UserData for Example {
    ///     fn add_methods<'lua, T: UserDataMethods<'lua, Self>>(methods: &mut T) {
    ///         let mut x = UserDataWrapper::from_user_data_methods(methods);
    ///         <Self as TealData>::add_methods(&mut x);
    ///     }
    ///}
    ///
    ///```
    pub fn from_user_data_methods(cont: &'a mut Container) -> Self {
        Self {
            cont,
            _t: PhantomData,
            _x: &PhantomData,
        }
    }
}

impl<'a, 'lua, Container, T> TealDataMethods<'lua, T> for UserDataWrapper<'a, 'lua, Container, T>
where
    T: UserData,
    Container: UserDataMethods<'lua, T>,
{
    #[inline(always)]
    fn add_method<S, A, R, M>(&mut self, name: &S, method: M)
    where
        S: ?Sized + AsRef<[u8]>,
        A: FromLuaMulti<'lua> + TealMultiValue,
        R: ToLuaMulti<'lua> + TealMultiValue,
        M: 'static + MaybeSend + Fn(&'lua Lua, &T, A) -> Result<R>,
    {
        self.cont.add_method(name, method)
    }
    #[inline(always)]
    fn add_method_mut<S, A, R, M>(&mut self, name: &S, method: M)
    where
        S: ?Sized + AsRef<[u8]>,
        A: FromLuaMulti<'lua>,
        R: ToLuaMulti<'lua>,
        M: 'static + MaybeSend + FnMut(&'lua Lua, &mut T, A) -> Result<R>,
    {
        self.cont.add_method_mut(name, method)
    }
    #[inline(always)]
    fn add_function<S, A, R, F>(&mut self, name: &S, function: F)
    where
        S: ?Sized + AsRef<[u8]>,
        A: FromLuaMulti<'lua>,
        R: ToLuaMulti<'lua>,
        F: 'static + MaybeSend + Fn(&'lua Lua, A) -> Result<R>,
    {
        self.cont.add_function(name, function)
    }
    #[inline(always)]
    fn add_function_mut<S, A, R, F>(&mut self, name: &S, function: F)
    where
        S: ?Sized + AsRef<[u8]>,
        A: FromLuaMulti<'lua>,
        R: ToLuaMulti<'lua>,
        F: 'static + MaybeSend + FnMut(&'lua Lua, A) -> Result<R>,
    {
        self.cont.add_function_mut(name, function)
    }
    #[inline(always)]
    fn add_meta_method<A, R, M>(&mut self, meta: MetaMethod, method: M)
    where
        A: FromLuaMulti<'lua>,
        R: ToLuaMulti<'lua>,
        M: 'static + MaybeSend + Fn(&'lua Lua, &T, A) -> Result<R>,
    {
        self.cont.add_meta_method(meta, method)
    }
    #[inline(always)]
    fn add_meta_method_mut<A, R, M>(&mut self, meta: MetaMethod, method: M)
    where
        A: FromLuaMulti<'lua>,
        R: ToLuaMulti<'lua>,
        M: 'static + MaybeSend + FnMut(&'lua Lua, &mut T, A) -> Result<R>,
    {
        self.cont.add_meta_method_mut(meta, method)
    }
    #[inline(always)]
    fn add_meta_function<A, R, F>(&mut self, meta: MetaMethod, function: F)
    where
        A: FromLuaMulti<'lua>,
        R: ToLuaMulti<'lua>,
        F: 'static + MaybeSend + Fn(&'lua Lua, A) -> Result<R>,
    {
        self.cont.add_meta_function(meta, function)
    }
    #[inline(always)]
    fn add_meta_function_mut<A, R, F>(&mut self, meta: MetaMethod, function: F)
    where
        A: FromLuaMulti<'lua>,
        R: ToLuaMulti<'lua>,
        F: 'static + MaybeSend + FnMut(&'lua Lua, A) -> Result<R>,
    {
        self.cont.add_meta_function_mut(meta, function)
    }

    #[cfg(feature = "mlua_async")]
    #[inline(always)]
    fn add_async_method<S: ?Sized, A, R, M, MR>(&mut self, name: &S, method: M)
    where
        T: Clone,
        S: AsRef<[u8]>,
        A: FromLuaMulti<'lua> + TealMultiValue,
        R: ToLuaMulti<'lua> + TealMultiValue,
        M: 'static + MaybeSend + Fn(&'lua Lua, T, A) -> MR,
        MR: 'lua + std::future::Future<Output = Result<R>>,
    {
        self.cont.add_async_method(name, method)
    }

    #[cfg(feature = "mlua_async")]
    #[inline(always)]
    fn add_async_function<S: ?Sized, A, R, F, FR>(&mut self, name: &S, function: F)
    where
        S: AsRef<[u8]>,
        A: FromLuaMulti<'lua> + TealMultiValue,
        R: ToLuaMulti<'lua> + TealMultiValue,
        F: 'static + MaybeSend + Fn(&'lua Lua, A) -> FR,
        FR: 'lua + std::future::Future<Output = Result<R>>,
    {
        self.cont.add_async_function(name, function)
    }
}