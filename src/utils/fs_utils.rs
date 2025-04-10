use handlebars::Handlebars;
use log::{debug, error, info};
use std::path::Path;
use std::{fs, io::Error};

use crate::config::connection::set_environment_variable;

pub fn read_hbs_template(file_name: &str) -> Result<String, Error> {
    info!("Reading from file {}", &file_name);
    let template_path = set_environment_variable("TEMPLATE_PATH", "./static/templates/");
    let full_path = format!("{}{}.hbs", template_path, &file_name);
    debug!("{:?}", &full_path);

    let file_contents = fs::read_to_string(&full_path);

    match file_contents {
        Ok(contents) => Ok(contents),
        Err(e) => {
            error!("Error reading template:: {}", e.to_string());
            Err(e)
        }
    }
}

pub fn register_templates(template_dir: &Path, handlebars: &mut Handlebars<'_>) {
    for entry in std::fs::read_dir(template_dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.is_dir() {
            register_templates(&path, handlebars);
        } else {
            let name = path.file_stem().unwrap().to_str().unwrap().to_owned();
            handlebars.register_template_file(&name, &path).unwrap();
        }
    }
}
