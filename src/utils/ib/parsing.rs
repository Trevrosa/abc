use chrono::{Datelike, Utc};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Warn {
    #[error("defaulted to sl")]
    SlDefaulted,
    #[error("defaulted to current year")]
    YearDefaulted,
    #[error("defaulted to sl & current year")]
    Both,
}

impl Warn {
    fn new(sl: bool, year: bool) -> Option<Self> {
        if sl && year {
            Some(Self::Both)
        } else if sl {
            Some(Self::SlDefaulted)
        } else if year {
            Some(Self::YearDefaulted)
        } else {
            None
        }
    }
}

#[derive(Debug)]
pub struct Query {
    subject: Subject,
    kind: QueryKind,
    year: u16,
}

impl Query {
    #[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
    pub fn parse(s: &str) -> (Option<Self>, Option<Warn>) {
        let (subject, sl_defaulted) = Subject::parse(s, false);
        let kind = QueryKind::parse(s);
        let (mut year, year_defaulted) = s
            .split(' ')
            .find_map(|w| w.parse::<u16>().ok())
            .map_or_else(|| (Utc::now().year() as u16, true), |y| (y, false));
        if matches!(kind, QueryKind::PastPaper) {
            year -= 1;
        }

        let query = subject.map(|subject| Self {
            subject,
            kind,
            year,
        });

        (query, Warn::new(sl_defaulted, year_defaulted))
    }
}

#[derive(Debug, Default)]
pub enum QueryKind {
    PastPaper,
    DataBooklet,
    SubjectGuide,
    SubjectReport,
    #[default]
    All,
}

impl QueryKind {
    fn parse(s: &str) -> Self {
        if s.contains("past") {
            Self::PastPaper
        } else if s.contains("booklet") || s.contains("formula") {
            Self::DataBooklet
        } else if s.contains("guide") {
            Self::SubjectGuide
        } else if s.contains("report") {
            Self::SubjectReport
        } else {
            Self::default()
        }
    }
}

#[allow(clippy::upper_case_acronyms)]
#[rustfmt::skip]
#[derive(Debug)]
pub enum Subject {
    English(Level),
    Chinese(Level),
    /// `ab` as in ab initio
    Spanish { level: Level, ab: bool },
    Chemistry(Level),
    Biology(Level),
    Physics(Level),
    Design(Level),
    ESS(Level),
    SEHS(Level),
    /// `aa` as in analysis and approaches 
    Mathematics { level: Level, aa: bool },
    VisualArts(Level),
    Theater(Level),
    Music(Level),
    ExtendedEssay(Box<Subject>),
    TheoryOfKnowledge(TOKType),
}

#[derive(Debug)]
pub enum TOKType {
    Essay,
    Exhibition,
}

#[derive(Debug, Default)]
pub enum Level {
    HL,
    #[default]
    SL,
}

impl Level {
    // returns Err variant if fallbacks to default
    fn parse(s: &str) -> Result<Self, Self> {
        if s.contains("hl") {
            Ok(Self::HL)
        } else if s.contains("sl") {
            Ok(Self::SL)
        } else {
            Err(Self::default())
        }
    }
}

impl Subject {
    /// input the full query.
    ///
    /// set `ee` to true when needing to detect the subject of the EE without infinitely recursing
    fn parse(s: &str, ee: bool) -> (Option<Self>, bool) {
        use self::Subject::{
            Biology, Chemistry, Chinese, Design, ESS, English, ExtendedEssay, Mathematics, Music,
            Physics, SEHS, Spanish, Theater, TheoryOfKnowledge, VisualArts,
        };

        let s = s.to_lowercase();
        let (level, defaulted) = match Level::parse(&s) {
            Ok(lvl) => (lvl, false),
            Err(lvl) => (lvl, true),
        };

        let subject = if !ee
            && (s.contains("ee") || s.contains("extended essay"))
            && let (Some(subject), _) = Self::parse(&s, true)
        {
            Some(ExtendedEssay(Box::new(subject)))
        } else if s.contains("english") {
            Some(English(level))
        } else if s.contains("chinese") {
            Some(Chinese(level))
        } else if s.contains("spanish") {
            // TODO: only ab initio
            Some(Spanish { level, ab: true })
        } else if s.contains("chem") {
            Some(Chemistry(level))
        } else if s.contains("bio") {
            Some(Biology(level))
        } else if s.contains("phy") {
            Some(Physics(level))
        } else if s.contains("design") | s.contains("dt") {
            Some(Design(level))
        } else if s.contains("sehs") || s.contains("sports") {
            Some(SEHS(level))
        } else if s.contains("ess") {
            Some(ESS(level))
        } else if s.contains("math") || s.contains("maths") {
            Some(Mathematics {
                level,
                aa: s.contains("aa"),
            })
        } else if s.contains("art") || s.contains("visual art") {
            Some(VisualArts(level))
        } else if s.contains("theater") || s.contains("drama") {
            Some(Theater(level))
        } else if s.contains("music") {
            Some(Music(level))
        } else if s.contains("tok") || s.contains("theory of knowledge") {
            Some(TheoryOfKnowledge(if s.contains("exhibition") {
                TOKType::Exhibition
            } else {
                TOKType::Essay
            }))
        } else {
            None
        };

        (subject, defaulted)
    }
}
