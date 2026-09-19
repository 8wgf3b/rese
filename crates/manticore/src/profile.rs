use std::{collections::HashSet, fs::read_to_string, path::Path};

use serde::{Deserialize, Deserializer};
use thiserror::Error;

use crate::{
    bullets::{Bullet, BulletId, EmptyBulletError},
    date::YearMonth,
    validate::Validate,
};

#[derive(Debug, Deserialize)]
pub struct Contact {
    pub name: String,
    pub email: String,
    pub phone: String,
    pub location: String,
    pub linkedin: String,
    pub github: String,
}

#[derive(Debug, Deserialize)]
pub struct Education {
    pub institution: String,
    pub degree: String,
    pub location: String,
    pub start: YearMonth,
    #[serde(default, deserialize_with = "deserialize_optional_yearmonth")]
    pub end: Option<YearMonth>,
}

#[derive(Debug, Deserialize)]
pub struct Experience {
    pub org: String,
    pub title: String,
    pub location: String,
    pub start: YearMonth,
    #[serde(default, deserialize_with = "deserialize_optional_yearmonth")]
    pub end: Option<YearMonth>,
    pub bullets: Vec<Bullet>,
    pub stack: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct Project {
    pub name: String,
    pub start: YearMonth,
    #[serde(default, deserialize_with = "deserialize_optional_yearmonth")]
    pub end: Option<YearMonth>,
    pub links: Vec<String>,
    pub bullets: Vec<Bullet>,
    pub stack: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct Skill {
    pub name: String,
    pub tags: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct Certification {
    pub name: String,
    pub link: String,
    pub issued: YearMonth,
    #[serde(default, deserialize_with = "deserialize_optional_yearmonth")]
    pub expires: Option<YearMonth>,
}

#[derive(Debug, Deserialize)]
pub struct Profile {
    pub contact: Contact,
    pub education: Vec<Education>,
    pub skills: Vec<Skill>,
    pub experience: Vec<Experience>,
    pub projects: Vec<Project>,
    pub certifications: Vec<Certification>,
}

impl Profile {
    fn duplicate_bulletids(&self) -> impl IntoIterator<Item = BulletId> {
        let mut set = HashSet::new();
        let mut res = vec![];
        for b in self
            .experience
            .iter()
            .flat_map(|e| e.bullets.iter().map(|b| &b.id))
        {
            if !set.insert(b.clone()) {
                res.push(b.clone())
            }
        }
        for b in self
            .projects
            .iter()
            .flat_map(|e| e.bullets.iter().map(|b| &b.id))
        {
            if !set.insert(b.clone()) {
                res.push(b.clone())
            }
        }
        res
    }

    pub fn load(path: impl AsRef<Path>) -> Result<Profile, CoreError> {
        let contents = read_to_string(path)?;
        let p: Profile = serde_yaml::from_str(&contents)?;
        let ec = p.validate();
        if ec.is_empty() {
            Ok(p)
        } else {
            Err(CoreError::Validation(ec))
        }
    }
}

#[derive(Error, Debug)]
pub enum CoreError {
    #[error("failed to read Profile file: {0}")]
    IO(#[from] std::io::Error),
    #[error("failed to parse yaml file: {0}")]
    YAML(#[from] serde_yaml::Error),
    #[error("profile validation failed: {} errors", .0.len())]
    Validation(Vec<ProfileError>),
}

#[derive(Error, Debug)]
pub enum ProfileError {
    #[error("Duplicate bullet id: {0}")]
    DuplicateBulletIds(BulletId),
    #[error("'{index}' Company: {source}")]
    Experience {
        index: String,
        #[source]
        source: ExperienceError,
    },
    #[error("'{index}' Project: {source}")]
    Project {
        index: String,
        #[source]
        source: ProjectError,
    },
    #[error("Skill[{index}]: {source}")]
    Skill {
        index: usize,
        #[source]
        source: SkillError,
    },
    #[error("'{index}' Certificate: {source}")]
    Certification {
        index: String,
        #[source]
        source: CertificationError,
    },
    #[error("'{index}' Education: {source}")]
    Education {
        index: String,
        #[source]
        source: EducationError,
    },
}

impl Validate for Profile {
    type Error = ProfileError;

    fn validate(&self) -> Vec<Self::Error> {
        let mut res: Vec<ProfileError> = vec![];
        res.extend(
            self.duplicate_bulletids()
                .into_iter()
                .map(ProfileError::DuplicateBulletIds),
        );
        res.extend(self.experience.iter().flat_map(|e| {
            e.validate().into_iter().map(|ee| ProfileError::Experience {
                index: e.org.to_string(),
                source: ee,
            })
        }));
        res.extend(self.projects.iter().flat_map(|e| {
            e.validate().into_iter().map(|ee| ProfileError::Project {
                index: e.name.to_string(),
                source: ee,
            })
        }));
        res.extend(self.skills.iter().enumerate().flat_map(|(i, e)| {
            e.validate().into_iter().map(move |ee| ProfileError::Skill {
                index: i,
                source: ee,
            })
        }));
        res.extend(self.education.iter().flat_map(|e| {
            e.validate().into_iter().map(|ee| ProfileError::Education {
                index: e.institution.to_string(),
                source: ee,
            })
        }));
        res.extend(self.certifications.iter().flat_map(|e| {
            e.validate()
                .into_iter()
                .map(|ee| ProfileError::Certification {
                    index: e.name.to_string(),
                    source: ee,
                })
        }));
        res
    }
}

#[derive(Error, Debug)]
pub enum ExperienceError {
    #[error("End date '{end}' is earlier than start date '{start}'")]
    DateError { start: YearMonth, end: YearMonth },
    #[error("bullet[{index}]: {source}")]
    BulletError {
        index: usize,
        #[source]
        source: EmptyBulletError,
    },
}

impl Validate for Experience {
    type Error = ExperienceError;

    fn validate(&self) -> Vec<Self::Error> {
        let mut res = vec![];
        if let Some(e) = self.end
            && e < self.start
        {
            res.push(Self::Error::DateError {
                start: self.start,
                end: e,
            })
        }
        res.extend(self.bullets.iter().enumerate().filter_map(|(i, b)| {
            b.validate().first().map(|s| Self::Error::BulletError {
                index: i,
                source: s.clone(),
            })
        }));
        res
    }
}

#[derive(Error, Debug)]
pub enum ProjectError {
    #[error("End date '{end}' is earlier than start date '{start}'")]
    DateError { start: YearMonth, end: YearMonth },
    #[error("bullet[{index}]: {source}")]
    BulletError {
        index: usize,
        #[source]
        source: EmptyBulletError,
    },
}

impl Validate for Project {
    type Error = ProjectError;

    fn validate(&self) -> Vec<Self::Error> {
        let mut res = vec![];
        if let Some(e) = self.end
            && e < self.start
        {
            res.push(Self::Error::DateError {
                start: self.start,
                end: e,
            })
        }
        res.extend(self.bullets.iter().enumerate().filter_map(|(i, b)| {
            b.validate().first().map(|s| Self::Error::BulletError {
                index: i,
                source: s.clone(),
            })
        }));
        res
    }
}

#[derive(Error, Debug)]
pub enum EducationError {
    #[error("End date '{end}' is earlier than start date '{start}'")]
    DateError { start: YearMonth, end: YearMonth },
}

impl Validate for Education {
    type Error = EducationError;

    fn validate(&self) -> Vec<Self::Error> {
        let mut res = vec![];
        if let Some(e) = self.end
            && e < self.start
        {
            res.push(Self::Error::DateError {
                start: self.start,
                end: e,
            })
        }
        res
    }
}

#[derive(Error, Debug)]
#[error("Invalid url {0}")]
pub struct CertificationError(String);

impl Validate for Certification {
    type Error = CertificationError;

    fn validate(&self) -> Vec<Self::Error> {
        let mut res = vec![];
        if !self.link.starts_with("https://") {
            res.push(CertificationError(self.link.clone()))
        }
        res
    }
}

#[derive(Error, Debug)]
#[error("Empty skill")]
pub struct SkillError;

impl Validate for Skill {
    type Error = SkillError;

    fn validate(&self) -> Vec<Self::Error> {
        let mut res = vec![];
        if self.name.trim().is_empty() {
            res.push(SkillError)
        }
        res
    }
}

fn deserialize_optional_yearmonth<'de, D: Deserializer<'de>>(
    des: D,
) -> Result<Option<YearMonth>, D::Error> {
    let opt: Option<String> = Option::deserialize(des)?;
    opt.map(|x| x.parse::<YearMonth>().map_err(serde::de::Error::custom))
        .transpose()
}
