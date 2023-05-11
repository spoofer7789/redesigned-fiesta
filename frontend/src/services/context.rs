use yew::prelude::*;
use std::rc::Rc;
use crate::services::auth;
use yewdux::prelude::*;

#[derive(Store, Default, PartialEq, Clone)]

pub struct UserContext {
    pub jwt_token: Option<String>,
    pub is_logged_in: bool,
}

type Context = Option<Rc<UserContext>>;

pub fn get_user_context() -> Context {
    Some(Rc::new(UserContext {
        jwt_token: auth::get_jwt_token(),
        is_logged_in: auth::get_jwt_token().is_some(),
    }))
}

pub fn update_user_context(context: &mut Context, jwt_token: Option<String>, is_logged_in: bool) {
    *context = Some(Rc::new(UserContext {
        jwt_token,
        is_logged_in,
    }));
}