#[derive(Clone)]
pub struct UserContext {
    pub jwt_token: Option<String>,
    pub is_logged_in: bool,
}

pub enum Action {
    Login(String),
    Logout,
}

impl Reducer for UserContext {
    type Action = Action;
    type Output = Self;

    fn reduce(self, action: Self::Action) -> Self {
        match action {
            Action::Login(token) => UserContext {
                jwt_token: Some(token),
                is_logged_in: true,
            },
            Action::Logout => UserContext {
                jwt_token: None,
                is_logged_in: false,
            },
        }
    }
}