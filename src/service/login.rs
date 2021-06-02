use crate::{CsesApi, Resources, Storage, RP};
use anyhow::Result;
use miniserde::{Deserialize, Serialize};

use super::require_login;

#[derive(Serialize, Deserialize)]
pub struct Login {
    pub username: String,
    pub password: String,
}

pub fn login(res: &mut Resources<impl RP>, login: &Login) -> Result<()> {
    let token = res.api.login(login)?;
    res.storage.get_mut().set_token(token);
    res.storage.save()?;
    Ok(())
}

pub fn logout(res: &mut Resources<impl RP>) -> Result<()> {
    let token = require_login(res)?;
    res.api.logout(token)?;
    res.storage.delete()?;
    Ok(())
}

/// Checks if a session is active, disregarding whether it is still valid
pub fn login_exists(res: &Resources<impl RP>) -> bool {
    res.storage.get().get_token().is_some()
}
