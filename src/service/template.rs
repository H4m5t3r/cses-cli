use crate::command;
use crate::entities::{TemplateParameters, TemplateResponse};
use crate::{CsesApi, Filesystem, Resources, Storage, RP};
use anyhow::{Context, Result};

pub fn create_template_parameters(
    res: &mut Resources<impl RP>,
    parameters: command::Template,
) -> Result<TemplateParameters> {
    let course_id = super::select_course(res, parameters.course_id)?;
    Ok(TemplateParameters {
        course: course_id,
        file: parameters.file_name,
        task: parameters.task_id,
        language: parameters.language,
    })
}

pub fn get_template(
    res: &mut Resources<impl RP>,
    parameters: &TemplateParameters,
) -> Result<TemplateResponse> {
    (|| -> Result<_> {
        let token = res.storage.get().get_token();
        Ok(res.api.get_template(
            token,
            &parameters.course,
            parameters.task,
            parameters.language.as_deref(),
            parameters.file.as_deref(),
        )?)
    })()
    .context("Failed querying code template from the server")
}

pub fn file_exists(res: &Resources<impl RP>, file_name: &str) -> bool {
    res.filesystem.file_exists(file_name)
}

pub fn save_response(res: &mut Resources<impl RP>, response: &TemplateResponse) -> Result<()> {
    res.filesystem.write_file(
        &res.filesystem.decode_base64(&response.template_source)?,
        &response.filename,
    )
}