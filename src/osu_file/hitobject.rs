use std::{error::Error, fmt::Display, str::FromStr};

use super::{Decimal, Integer, OsuFileParseError};

pub trait HitObject {
    fn x(&self) -> Integer;
    fn y(&self) -> Integer;
    fn set_x(&mut self, x: Integer);
    fn set_y(&mut self, y: Integer);

    fn time(&self) -> Integer;
    fn set_time(&mut self, time: Integer);

    fn obj_type(&self) -> &HitObjectType;

    fn newcombo(&self) -> bool;
    fn set_newcombo(&mut self, value: bool);

    fn hitsound(&self) -> &HitSound;
    fn set_hitsound(&mut self, hitsound: HitSound);

    fn hitsample(&self) -> &HitSample;
    fn hitsample_mut(&mut self) -> &mut HitSample;
}

pub fn parse_hitobject(hitobject: &str) -> Result<Box<dyn HitObject>, HitObjectParseError> {
    todo!()
}

#[derive(Debug)]
pub struct HitObjectParseError;

impl Display for HitObjectParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "There was a problem parsing a hitobject from a string")
    }
}

impl Error for HitObjectParseError {}

impl From<HitObjectParseError> for OsuFileParseError {
    fn from(err: HitObjectParseError) -> Self {
        Self::SectionParseError {
            source: Box::new(err),
        }
    }
}

pub enum HitObjectType {
    HitCircle,
    Slider,
    Spinner,
    OsuManiaHold,
}

pub enum HitSound {
    Normal,
    Whistle,
    Finish,
    Clap,
}

impl Default for HitSound {
    fn default() -> Self {
        Self::Normal
    }
}

#[derive(Default)]
pub struct HitSample {
    normal_set: SampleSet,
    addition: SampleSet,
    // TODO check if the file format accepts negative index
    index: Integer,
    volume: Volume,
    filename: String,
}

pub enum SampleSet {
    NoCustomSampleSet,
    NormalSet,
    SoftSet,
    DrumSet,
}

impl Default for SampleSet {
    fn default() -> Self {
        Self::NoCustomSampleSet
    }
}

#[derive(Default)]
pub struct Volume(Integer);

pub struct HitCircle {
    x: Integer,
    y: Integer,
    time: Integer,
    obj_type: HitObjectType,
    hitsound: HitSound,
    hitsample: HitSample,

    new_combo: bool,
}

impl Default for HitCircle {
    fn default() -> Self {
        Self {
            x: Default::default(),
            y: Default::default(),
            time: Default::default(),
            obj_type: HitObjectType::HitCircle,
            hitsound: Default::default(),
            hitsample: Default::default(),
            new_combo: Default::default(),
        }
    }
}

impl HitObject for HitCircle {
    fn x(&self) -> Integer {
        self.x
    }

    fn y(&self) -> Integer {
        self.y
    }

    fn set_x(&mut self, x: Integer) {
        self.x = x;
    }

    fn set_y(&mut self, y: Integer) {
        self.y = y;
    }

    fn time(&self) -> Integer {
        self.time
    }

    fn set_time(&mut self, time: Integer) {
        self.time = time;
    }

    fn obj_type(&self) -> &HitObjectType {
        &self.obj_type
    }

    fn newcombo(&self) -> bool {
        self.new_combo
    }

    fn set_newcombo(&mut self, value: bool) {
        self.new_combo = value;
    }

    fn hitsound(&self) -> &HitSound {
        &self.hitsound
    }

    fn set_hitsound(&mut self, hitsound: HitSound) {
        self.hitsound = hitsound;
    }

    fn hitsample(&self) -> &HitSample {
        &self.hitsample
    }

    fn hitsample_mut(&mut self) -> &mut HitSample {
        &mut self.hitsample
    }
}

impl HitCircle {
    pub fn new(
        x: Integer,
        y: Integer,
        time: Integer,
        hitsound: HitSound,
        hitsample: HitSample,
        new_combo: bool,
    ) -> Self {
        Self {
            x,
            y,
            time,
            obj_type: HitObjectType::HitCircle,
            hitsound,
            hitsample,
            new_combo,
        }
    }
}

pub struct Slider {
    x: Integer,
    y: Integer,
    time: Integer,
    obj_type: HitObjectType,
    hitsound: HitSound,
    hitsample: HitSample,

    new_combo: bool,

    curve_type: CurveType,
    curve_points: Vec<(Integer, Integer)>,
    slides: Integer,
    length: Decimal,
    // TODO
    edge_sounds: Vec<Integer>,
    // TODO
    edge_sets: Vec<String>,
}

pub enum CurveType {
    Bezier,
    Centripetal,
    Linear,
    PerfectCircle,
}

pub struct PipeVec<T> {
    vec: Vec<T>,
}

impl<T> FromStr for PipeVec<T> {
    type Err = PipeVecParseErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}

#[derive(Debug)]
pub struct PipeVecParseErr;

impl Display for PipeVecParseErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "There was a problem parsing a pipe-separated list of values"
        )
    }
}

impl Error for PipeVecParseErr {}
