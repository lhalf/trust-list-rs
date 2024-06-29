use std::io::Write;

use anyhow::{Context, Error};

use crate::crates_io::Crate;

pub struct OutputFile {
    path: String,
}

impl OutputFile {
    pub fn at_path(path: &str) -> Self {
        Self {
            path: path.to_string(),
        }
    }

    pub fn exists(&self) -> bool {
        std::path::Path::new(&self.path).exists()
    }

    pub fn write_md_table(&self, crates: Vec<Crate>) -> Result<(), Error> {
        let mut file = self.recreate()?;

        file.write_all(Crate::table_heading().as_bytes())
            .expect("unable to write");

        file.write_all(Crate::table_gap().as_bytes())
            .expect("unable to write");

        for _crate in crates {
            file.write_all(_crate.table_entry().as_bytes())
                .expect("unable to write");
        }

        Ok(())
    }

    fn recreate(&self) -> Result<std::fs::File, Error> {
        std::fs::File::create(&self.path).context("could not create file")
    }

    pub fn crates_from_md_table(&self) -> Result<Vec<String>, Error> {
        if !self.exists() {
            return Err(anyhow::anyhow!("file does not exist"));
        }

        let contents = std::fs::read_to_string(&self.path).context("failed to open file")?;

        Ok(contents
            .split("\n")
            .skip(2)
            .map(|line| line.split("|").skip(1).take(1).collect::<Vec<&str>>())
            .flatten()
            .map(|crate_name| crate_name.trim().to_string())
            .collect::<Vec<String>>())
    }
}

impl Crate {
    fn table_heading() -> String {
        // how to do?
        "|crate|downloads|created|last_updated|link|\n".to_string()
    }

    fn table_gap() -> String {
        // also how to do?
        "|-|-|-|-|-|\n".to_string()
    }

    fn table_entry(&self) -> String {
        format!(
            "|{}|{}|{}|{}|{}|\n",
            self.name,
            self.downloads,
            self.created_at.format("%d/%m/%Y"),
            self.updated_at.format("%d/%m/%Y"),
            self.repository
        )
    }
}
