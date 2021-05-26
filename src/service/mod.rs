use crate::{CsesApi, Resources, Storage, RP};
use anyhow::{anyhow, Result};
pub struct Login {
    pub username: String,
    pub password: String,
}

pub fn ping(_res: &mut Resources<impl RP>) -> bool {
    true
}

pub fn login(res: &mut Resources<impl RP>, login: &Login) -> Result<()> {
    let token = res.api.login(login)?;
    res.storage.set_token(Some(token));
    res.storage.save()
}

pub fn logout(res: &mut Resources<impl RP>) -> Result<()> {
    if let Some(token) = res.storage.get_token() {
        res.api.logout(token)?;
        res.storage.set_token(None);
        res.storage.save()?;
        Ok(())
    } else {
        Err(anyhow!("not currently logged in"))
    }
}
