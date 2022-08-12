use anyhow::Result;
use crossterm::style::Color;
use std::{
    fs::File,
    io::{BufReader, BufWriter, Write},
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};

use crate::highlighter::CodeHighligher;

pub trait ITemaplteRepository {
    fn get(&self, name: &str) -> Option<&Template>;
    fn list(&self) -> &Vec<Template>;
    fn search(&self, query: &str) -> Vec<&Template>;
    fn put(&mut self, template: Template) -> Result<()>;
}

pub struct JSONTemplateRepository {
    templates: Vec<Template>,
}

impl JSONTemplateRepository {
    pub fn load<P: AsRef<Path>>(path: &P) -> Result<Self> {
        if !path.as_ref().exists() {
            return Ok(Self {
                templates: Vec::new(),
            });
        };

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

impl Default for JSONTemplateRepository {
    fn default() -> Self {
        Self {
            templates: Vec::new(),
        }
    }
}

impl ITemaplteRepository for JSONTemplateRepository {
    fn put(&mut self, template: Template) -> Result<()> {
        if let Some(_template) = self.templates.iter_mut().find(|t| t.name == template.name) {
            *_template = template;
        } else {
            self.templates.push(template);
        }

        Ok(())
    }

    fn get(&self, name: &str) -> Option<&Template> {
        self.templates.iter().find(|template| template.name == name)
    }

    fn list(&self) -> &Vec<Template> {
        &self.templates
    }

    fn search(&self, query: &str) -> Vec<&Template> {
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
pub struct Template {
    name: String,
    ext: Option<String>,
    description: Option<String>,
    template: String,
    #[serde(skip)]
    pub highlighted_template: Option<Vec<(String, Color)>>,
}

impl Template {
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

    pub fn highlight(&mut self) -> Result<()> {
        let highlighter = CodeHighligher::default();
        self.highlighted_template = if let Some(ext) = self.ext.as_ref() {
            Some(highlighter.highlight(ext, &self.template)?)
        } else {
            Some(highlighter.highlight("txt", &self.template)?)
        };

        Ok(())
    }
}
