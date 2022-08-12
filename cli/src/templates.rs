use anyhow::Result;
use std::{
    fs::File,
    io::{BufReader, BufWriter, Write},
    path::{Path, PathBuf},
};
use tui::text::Spans;

use serde::{Deserialize, Serialize};

use crate::highlighter::CodeHighligher;

pub trait ITemaplteRepository<'a> {
    fn get(&self, name: &str) -> Option<&'a Template>;
    fn list(&self) -> &'a Vec<Template>;
    fn search(&self, query: &str) -> Vec<&'a Template>;
    fn put(&mut self, template: Template<'a>) -> Result<()>;
}

pub struct JSONTemplateRepository<'a> {
    templates: Vec<Template<'a>>,
}

impl<'a> JSONTemplateRepository<'a> {
    pub fn load<P: AsRef<Path>>(path: &P) -> Result<Self> {
        let f = File::open(path)?;
        let rdr = BufReader::new(f);
        let templates: Vec<Template> = serde_json::from_reader(rdr)?;

        Ok(Self { templates })
    }

    pub fn serialize(&self) -> Result<String> {
        Ok(serde_json::to_string(&self.templates)?)
    }

    pub fn save<P: AsRef<Path>>(&self, path: &P) -> Result<()> {
        let s = self.serialize()?;

        let f = File::create(path)?;
        let mut writer = BufWriter::new(f);
        writer.write_all(s.as_bytes())?;        
        Ok(())
    }
}

impl<'a> Default for JSONTemplateRepository<'a> {
    fn default() -> Self {
        Self {
            templates: Vec::new(),
        }
    }
}

impl<'a> ITemaplteRepository<'a> for JSONTemplateRepository<'a> {
    fn put(&mut self, template: Template<'a>) -> Result<()> {
        self.templates.push(template);
        Ok(())
    }

    fn get(&self, name: &str) -> Option<&'a Template> {
        self.templates.iter().find(|template| template.name == name)
    }

    fn list(&self) -> &'a Vec<Template> {
        &self.templates
    }

    fn search(&self, query: &str) -> Vec<&'a Template> {
        self.templates
            .iter()
            .filter(|template| {
                template.name.contains(query)
                    || template
                        .description
                        .as_ref()
                        .map_or(false, |d| d.contains(query))
                    || template.template.contains(query)
            })
            .collect()
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Template<'a> {
    name: String,
    ext: Option<String>,
    description: Option<String>,
    template: String,
    #[serde(skip)]
    pub highlighted_template: Option<Vec<Spans<'a>>>,
}

impl<'a> Template<'a> {
    pub fn new(name: String, description: Option<String>, template: String) -> Self {
        let ext = PathBuf::from(&name)
            .extension()
            .map(|s| s.to_string_lossy().to_string());

        Self {
            name,
            ext,
            description,
            template,
            highlighted_template: None,
        }
    }

    pub fn highlight(&'a mut self) -> Result<()> {
        let highlighter = CodeHighligher::default();

        self.highlighted_template = if let Some(ext) = self.ext.as_ref() {
            Some(highlighter.highlight(ext, &self.template)?)
        } else {
            Some(highlighter.highlight("txt", &&self.template)?)
        };

        Ok(())
    }
}
