mod courses;
mod login;
mod template;
pub use courses::{courses, scope_content};
pub use login::{login, login_is_valid, login_status, logout, Login};
pub use template::{file_exists, get_template, save_response};

pub mod submit;
pub use submit::{nth_last_submission_info, submission_info, submission_list, submit};

mod scope;
pub use scope::select_scope;

use crate::{Resources, Storage, RP};
use anyhow::{anyhow, Result};

pub fn ping(_res: &mut Resources<impl RP>) -> bool {
    true
}

fn require_login(res: &Resources<impl RP>) -> Result<&str> {
    res.storage
        .get()
        .get_token()
        .ok_or_else(|| anyhow!("Not currently logged in"))
}
