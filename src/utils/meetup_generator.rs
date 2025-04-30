use crate::utils::loader::{load_things, load_words};
use crate::utils::string::Companyize;
use crate::utils::types::{Agenda, Meetup, Talk};
use anyhow::anyhow;
use rand::seq::SliceRandom;
use rand::Rng;
use regex::Regex;
use std::path::Path;
use time::macros::format_description;
use time::{Duration, OffsetDateTime};

impl Meetup {
    pub fn new(things_file: &Path, words_file: &Path) -> anyhow::Result<Self> {
        let things = match load_things(things_file) {
            Ok(things) => things,
            Err(e) => return Err(anyhow!("failed to load {}: {}", things_file.display(), e)),
        };
        let words = match load_words(words_file) {
            Ok(words) => words,
            Err(e) => return Err(anyhow!("failed to load {}: {}", words_file.display(), e)),
        };

        Ok(Self { things, words })
    }

    pub fn location(&self) -> String {
        "Shoreditch, probably".to_string()
    }

    pub fn date(&self) -> String {
        let today = OffsetDateTime::now_utc();
        let tomorrow = today + Duration::days(1);
        let format = format_description!("[day]/[month]/[year]");
        tomorrow.format(&format).expect("Cannot format time")
    }

    pub fn talker(&self) -> String {
        self.pair(&self.things.first_name, &self.things.last_name)
    }

    pub fn role(&self) -> String {
        self.pair(&self.things.job_role, &self.things.job_title)
    }

    pub fn refreshments(&self) -> String {
        self.pair(&self.things.food_style, &self.things.food)
    }

    pub fn company(&self) -> String {
        format!("{}.io", self.sample(&self.words).companyize())
    }

    pub fn agenda(&self, talks: usize) -> Agenda {
        let mut rng = rand::thread_rng();
        let templates = self.things.template.choose_multiple(&mut rng, talks);

        Agenda {
            talks: templates.map(|t| self.talk(t)).collect(),
            refreshment: self.refreshments(),
            location: self.location(),
            date: self.date(),
        }
    }

    pub fn single_talk(&self) -> Talk {
        self.talk(&self.sample(&self.things.template))
    }

    fn talk(&self, template: &str) -> Talk {
        Talk {
            title: self.fill_template(template),
            talker: self.talker(),
            role: self.role(),
            company: self.company(),
        }
    }

    fn something_ops(&self) -> String {
        let mut rng = rand::thread_rng();
        format!(
            "{}Ops",
            self.things
                .something_ops
                .choose_multiple(&mut rng, 4)
                .cloned()
                .collect::<Vec<String>>()
                .join(""),
        )
    }

    fn random_number(&self, match_string: &str) -> String {
        let ceiling = match_string
            .replace("%RAND", "")
            .replace("%", "")
            .parse::<usize>()
            .expect("cannot parse template number");

        rand::thread_rng().gen_range(1..ceiling).to_string()
    }

    fn fill_template(&self, template: &str) -> String {
        let rx = Regex::new(r"%[^%]+%").unwrap();

        rx.replace_all(template, |caps: &regex::Captures| match &caps[0] {
            "%adjective%" => self.sample(&self.things.adjective),
            "%company%" => self.sample(&self.things.company),
            "%degree%" => self.sample(&self.things.degree),
            "%driver%" => self.sample(&self.things.driver),
            "%extreme%" => self.sample(&self.things.extreme),
            "%FNOPS%" => self.something_ops(),
            "%job_title%" => self.sample(&self.things.job_title),
            "%language%" => self.sample(&self.things.language),
            "%panacea%" => self.sample(&self.things.panacea),
            "%quantifier%" => self.sample(&self.things.quantifier),
            "%service%" => self.sample(&self.things.service),
            "%skill_level%" => self.sample(&self.things.skill_level),
            "%tech%" => self.sample(&self.things.tech),
            "%time%" => self.sample(&self.things.time),
            "%verb%" => self.sample(&self.things.verb),
            "%WORD%" => self.sample(&self.words),
            s if s.starts_with("%RAND") => self.random_number(s),
            _ => "merp".to_string(),
        })
        .to_string()
    }

    fn pair(&self, list_1: &[String], list_2: &[String]) -> String {
        format!("{} {}", self.sample(list_1), self.sample(list_2))
    }

    fn sample(&self, list: &[String]) -> String {
        let mut rng = rand::thread_rng();
        list.choose(&mut rng)
            .unwrap_or_else(|| panic!("failed to get random sample from {:?}", list))
            .to_owned()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::utils::spec_helper::fixture;
    use regex::Regex;
    use std::path::PathBuf;

    #[test]
    fn test_meetup() {
        let meetup = Meetup::new(&fixture("test_things.toml"), &fixture("test_words.gz")).unwrap();

        assert_eq!("Shoreditch, probably".to_string(), meetup.location());

        let date_regex = Regex::new(r"^[0123]\d\/[012]\d\/20\d\d$").unwrap();
        assert!(date_regex.is_match(&meetup.date()));

        assert_eq!("John Smith".to_string(), meetup.talker());

        assert_eq!(
            "Full Stack Neckbeard without Portfolio".to_string(),
            meetup.role()
        );

        assert_eq!("artisanal avocado toast".to_string(), meetup.refreshments());

        assert_eq!("prognosticatr.io".to_string(), meetup.company());

        assert_eq!("ChatOps".to_string(), meetup.something_ops());

        assert_eq!(
            "Literal String".to_string(),
            meetup.fill_template("Literal String")
        );

        assert_eq!(
            "How we Tested our TestOps with Rust".to_string(),
            meetup.fill_template("How we %verb%ed our %service% with %tech%")
        );

        assert_eq!(
            "How to De-Risk Your Polyglot Persistence by Not Testing Rust",
            meetup
                .fill_template("How to %extreme% Your %quantifier% by %degree% %verb%ing %tech%",)
        );

        let num_rx = Regex::new(r"^EC2 at Scale: \d instances and Counting!$").unwrap();
        let num_result = meetup.fill_template("EC2 at Scale: %RAND9% instances and Counting!");
        assert!(num_rx.is_match(&num_result));

        assert_eq!(
            Talk {
                title: "How to De-Risk Your Polyglot Persistence by Not Testing Rust".to_string(),
                talker: "John Smith".to_string(),
                role: "Full Stack Neckbeard without Portfolio".to_string(),
                company: "prognosticatr.io".to_string(),
            },
            meetup.talk("How to %extreme% Your %quantifier% by %degree% %verb%ing %tech%")
        );
    }

    // Run through every template. We'll get a panic if any aren't fillable.
    #[test]
    fn test_all_templates() {
        let root = PathBuf::from(rocket::fs::relative!("."));
        let res_dir = root.join("resources");

        let meetup = Meetup::new(
            &res_dir.join("all_the_things.toml"),
            &res_dir.join("words.gz"),
        )
        .unwrap();

        for t in meetup.things.template.iter() {
            meetup.fill_template(t);
        }
    }
}
