use std::{
    error::Error,
    fmt::{Debug, Display},
    num::ParseIntError,
    str::FromStr,
};

use super::{
    section_error::{InvalidKey, MissingValue},
    Decimal, Integer, DELIMITER,
};

/// A struct representing the general section of the .osu file
#[derive(PartialEq, Debug)]
pub struct General {
    /// Location of the audio file relative to the current folder
    pub audio_filename: String,
    /// Milliseconds of silence before the audio starts playing
    pub audio_lead_in: Integer,
    /// Deprecated
    pub audio_hash: String,
    /// Time in milliseconds when the audio preview should start
    /// - Defaults to `-1`
    pub preview_time: Integer,
    /// Speed of the countdown before the first hit object
    /// - Defaults to `Normal`
    pub countdown: CountdownSpeed,
    /// Sample set that will be used if timing points do not override it
    /// - Defaults to `Normal`
    pub sample_set: SampleSet,
    /// Multiplier for the threshold in time where hit objects placed close together stack
    /// - Defaults to `0.7`
    pub stack_leniency: Decimal,
    /// Game mode
    /// - Defaults to `osu`
    pub mode: GameMode,
    /// Whether or not breaks have a letterboxing effect
    /// - Defaults to `false`
    pub letterbox_in_breaks: bool,
    /// Deprecated
    /// - Defaults to `true`
    pub story_fire_in_front: bool,
    /// Whether or not the storyboard can use the user's skin images
    /// - Defaults to `false`
    pub use_skin_sprites: bool,
    /// Deprecated
    /// - Defaults to `false`
    pub always_show_playfield: bool,
    /// Draw order of hit circle overlays compared to hit numbers
    /// - Defaults to `NoChange`
    pub overlay_position: OverlayPosition,
    /// Preferred skin to use during gameplay
    pub skin_preference: String,
    /// Whether or not a warning about flashing colours should be shown at the beginning of the map
    /// - Defaults to `false`
    pub epilepsy_warning: bool,
    /// Time in beats that the countdown starts before the first hit object
    /// - Defaults to `false`
    pub countdown_offset: Integer,
    /// Whether or not the "N+1" style key layout is used for osu!mania
    /// - Defaults to `false`
    pub special_style: bool,
    /// Whether or not the storyboard allows widescreen viewing
    /// - Defaults to `false`
    pub widescreen_storyboard: bool,
    /// Whether or not sound samples will change rate when playing with speed-changing mods
    /// - Defaults to `false`
    pub samples_match_playback_rate: bool,
}

impl Default for General {
    fn default() -> Self {
        Self {
            audio_filename: Default::default(),
            audio_lead_in: Default::default(),
            audio_hash: Default::default(),
            preview_time: -1,
            countdown: Default::default(),
            sample_set: Default::default(),
            stack_leniency: 0.7,
            mode: Default::default(),
            letterbox_in_breaks: Default::default(),
            story_fire_in_front: true,
            use_skin_sprites: Default::default(),
            always_show_playfield: Default::default(),
            overlay_position: Default::default(),
            skin_preference: Default::default(),
            epilepsy_warning: Default::default(),
            countdown_offset: Default::default(),
            special_style: Default::default(),
            widescreen_storyboard: Default::default(),
            samples_match_playback_rate: Default::default(),
        }
    }
}

impl FromStr for General {
    type Err = GeneralParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut general = Self::default();

        let s = s.trim();

        for line in s.lines() {
            match line.split_once(DELIMITER) {
                Some((key, mut value)) => {
                    value = value.trim();

                    let parse_result = match key.trim() {
                        "AudioFilename" => {
                            general.audio_filename = value.to_owned();
                            Ok(())
                        }
                        "AudioLeadIn" => general.audio_lead_in = value.parse().unwrap_err(),
                        "AudioHash" => general.audio_hash = value.to_owned(),
                        "PreviewTime" => general.preview_time = parse_error_return(value, line)?,
                        "Countdown" => {
                            general.countdown =
                                match parse_error_return::<Integer>(value, line)?.try_into() {
                                    Ok(value) => value,
                                    Err(err) => {
                                        return Err(GeneralKeyParseError {
                                            source: Box::new(err),
                                            line: line.to_owned(),
                                        })
                                    }
                                }
                        }
                        "SampleSet" => {
                            general.sample_set = match SampleSet::from_str(value) {
                                Ok(value) => value,
                                Err(err) => {
                                    return Err(GeneralKeyParseError {
                                        source: Box::new(err),
                                        line: line.to_owned(),
                                    })
                                }
                            }
                        }
                        "StackLeniency" => {
                            general.stack_leniency = parse_error_return(value, line)?
                        }
                        "Mode" => {
                            general.mode =
                                match parse_error_return::<Integer>(value, line)?.try_into() {
                                    Ok(value) => value,
                                    Err(err) => {
                                        return Err(GeneralKeyParseError {
                                            source: Box::new(err),
                                            line: line.to_owned(),
                                        })
                                    }
                                }
                        }
                        "LetterboxInBreaks" => {
                            general.letterbox_in_breaks = parse_zero_one_bool(value, line)?
                        }
                        "StoryFireInFront" => {
                            general.story_fire_in_front = parse_zero_one_bool(value, line)?
                        }
                        "UseSkinSprites" => {
                            general.use_skin_sprites = parse_zero_one_bool(value, line)?
                        }
                        "AlwaysShowPlayfield" => {
                            general.always_show_playfield = parse_zero_one_bool(value, line)?
                        }
                        "OverlayPosition" => {
                            general.overlay_position = match OverlayPosition::from_str(value) {
                                Ok(value) => value,
                                Err(err) => {
                                    return Err(GeneralKeyParseError {
                                        source: Box::new(err),
                                        line: line.to_owned(),
                                    })
                                }
                            }
                        }
                        "SkinPreference" => general.skin_preference = value.to_owned(),
                        "EpilepsyWarning" => {
                            general.epilepsy_warning = parse_zero_one_bool(value, line)?
                        }
                        "CountdownOffset" => {
                            general.countdown_offset = parse_error_return(value, line)?
                        }
                        "SpecialStyle" => general.special_style = parse_zero_one_bool(value, line)?,
                        "WidescreenStoryboard" => {
                            general.widescreen_storyboard = parse_zero_one_bool(value, line)?
                        }
                        "SamplesMatchPlaybackRate" => {
                            general.samples_match_playback_rate = parse_zero_one_bool(value, line)?
                        }
                        _ => {
                            return Err(GeneralKeyParseError {
                                source: Box::new(InvalidKey(key.to_owned())),
                                line: line.to_owned(),
                            })
                        }
                    };
                }
                None => {
                    return Err(GeneralKeyParseError {
                        source: Box::new(MissingValue(line.to_owned())),
                        line: line.to_owned(),
                    })
                }
            }
        }

        Ok(general)
    }
}

fn parse_error_return<T>(value: &str, line: &str) -> Result<T, GeneralKeyParseError>
where
    T: FromStr,
    <T as FromStr>::Err: Error + 'static,
{
    match value.parse::<T>() {
        Ok(value) => Ok(value),
        Err(err) => Err(GeneralKeyParseError {
            source: Box::new(err),
            line: line.to_owned(),
        }),
    }
}

fn parse_zero_one_bool(value: &str, line: &str) -> Result<bool, GeneralKeyParseError> {
    let value = parse_error_return(value, line)?;

    match value {
        0 => Ok(false),
        1 => Ok(true),
        _ => {
            return Err(GeneralKeyParseError {
                source: Box::new(ParseBoolError),
                line: line.to_owned(),
            })
        }
    }
}

/// Error for when having a problem parsing 0 or 1 as a boolean
#[derive(Debug)]
struct ParseBoolError;

impl Display for ParseBoolError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error parsing integer as `true` or `false`")
    }
}

impl Error for ParseBoolError {}

impl Display for General {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut key_value = Vec::new();

        key_value.push(format!("AudioFilename: {}", self.audio_filename));
        key_value.push(format!("AudioLeadIn: {}", self.audio_lead_in));
        key_value.push(format!("AudioHash: {}", self.audio_hash));
        key_value.push(format!("PreviewTime: {}", self.preview_time));
        key_value.push(format!("Countdown: {}", self.countdown));
        key_value.push(format!("SampleSet: {}", self.sample_set));
        key_value.push(format!("StackLeniency: {}", self.stack_leniency));
        key_value.push(format!("Mode: {}", self.mode));
        key_value.push(format!(
            "LetterboxInBreaks: {}",
            self.letterbox_in_breaks as Integer
        ));
        key_value.push(format!(
            "StoryFireInFront: {}",
            self.story_fire_in_front as Integer
        ));
        key_value.push(format!(
            "UseSkinSprites: {}",
            self.use_skin_sprites as Integer
        ));
        key_value.push(format!(
            "AlwaysShowPlayfield: {}",
            self.always_show_playfield as Integer
        ));
        key_value.push(format!("OverlayPosition: {}", self.overlay_position));
        key_value.push(format!("SkinPreference: {}", self.skin_preference));
        key_value.push(format!(
            "EpilepsyWarning: {}",
            self.epilepsy_warning as Integer
        ));
        key_value.push(format!("CountdownOffset: {}", self.countdown_offset));
        key_value.push(format!("SpecialStyle: {}", self.special_style as Integer));
        key_value.push(format!(
            "WidescreenStoryboard: {}",
            self.widescreen_storyboard as Integer
        ));
        key_value.push(format!(
            "SamplesMatchPlaybackRate: {}",
            self.samples_match_playback_rate as Integer
        ));

        write!(f, "{}", key_value.join("\r\n"))
    }
}

#[derive(Debug)]
pub struct GeneralParseError {
    source: GeneralKeyParseError,
}

impl Display for GeneralParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "There was a problem parsing the `General` section")
    }
}

impl Error for GeneralParseError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.source)
    }
}

impl From<GeneralKeyParseError> for GeneralParseError {
    fn from(err: GeneralKeyParseError) -> Self {
        GeneralParseError { source: err }
    }
}

#[derive(Debug)]
/// Error for when parsing a key: value line
pub struct GeneralKeyParseError {
    source: Box<dyn Error>,
}

impl From<ParseIntError> for GeneralKeyParseError {
    fn from(err: ParseIntError) -> Self {
        GeneralKeyParseError {
            source: Box::new(err),
        }
    }
}

impl Display for GeneralKeyParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error parsing a key: value line in General")
    }
}

impl Error for GeneralKeyParseError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(self.source.as_ref())
    }
}

/// Speed of the countdown before the first hit
#[derive(PartialEq, Eq, Debug)]
pub enum CountdownSpeed {
    NoCountdown,
    Normal,
    Half,
    Double,
}

impl Display for CountdownSpeed {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            CountdownSpeed::NoCountdown => 0,
            CountdownSpeed::Normal => 1,
            CountdownSpeed::Half => 2,
            CountdownSpeed::Double => 3,
        };

        write!(f, "{value}")
    }
}

impl TryFrom<i32> for CountdownSpeed {
    type Error = CountdownSpeedParseError;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(CountdownSpeed::NoCountdown),
            1 => Ok(CountdownSpeed::Normal),
            2 => Ok(CountdownSpeed::Half),
            3 => Ok(CountdownSpeed::Double),
            _ => Err(CountdownSpeedParseError),
        }
    }
}

/// Error used when there's an error parsing the string as enum
#[derive(Debug)]
pub struct CountdownSpeedParseError;

impl Display for CountdownSpeedParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error trying to parse `value` as CountdownSpeed")
    }
}

impl Error for CountdownSpeedParseError {}

impl Default for CountdownSpeed {
    fn default() -> Self {
        Self::Normal
    }
}

/// Sample set that will be used if timing points do not override it
#[derive(PartialEq, Eq, Debug)]
pub enum SampleSet {
    Normal,
    Soft,
    Drum,
}

impl Display for SampleSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            SampleSet::Normal => "Normal",
            SampleSet::Soft => "Soft",
            SampleSet::Drum => "Drum",
        };

        write!(f, "{value}")
    }
}

impl Default for SampleSet {
    fn default() -> Self {
        SampleSet::Normal
    }
}

impl FromStr for SampleSet {
    type Err = SampleSetParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Normal" => Ok(SampleSet::Normal),
            "Soft" => Ok(SampleSet::Soft),
            "Drum" => Ok(SampleSet::Drum),
            _ => Err(SampleSetParseError),
        }
    }
}

/// Error used when there's an error parsing the string as enum
#[derive(Debug)]
pub struct SampleSetParseError;

impl Display for SampleSetParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error trying to parse `value` as SampleSet")
    }
}

impl Error for SampleSetParseError {}

/// Game mode of the .osu file
#[derive(PartialEq, Eq, Debug)]
pub enum GameMode {
    Osu,
    Taiko,
    Catch,
    Mania,
}

impl Display for GameMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            GameMode::Osu => 0,
            GameMode::Taiko => 1,
            GameMode::Catch => 2,
            GameMode::Mania => 3,
        };

        write!(f, "{value}")
    }
}

impl Default for GameMode {
    fn default() -> Self {
        Self::Osu
    }
}

impl TryFrom<i32> for GameMode {
    type Error = GameModeParseError;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(GameMode::Osu),
            1 => Ok(GameMode::Taiko),
            2 => Ok(GameMode::Catch),
            3 => Ok(GameMode::Mania),
            _ => Err(GameModeParseError),
        }
    }
}

/// Error used when there's an error parsing the string as enum
#[derive(Debug)]
pub struct GameModeParseError;

impl Display for GameModeParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error trying to parse `value` as GameMode")
    }
}

impl Error for GameModeParseError {}

/// Draw order of hit circle overlays compared to hit numbers
#[derive(PartialEq, Eq, Debug)]
pub enum OverlayPosition {
    /// Use skin setting
    NoChange,
    /// Draw overlays under numbers
    Below,
    /// Draw overlays on top of numbers
    Above,
}

impl Display for OverlayPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            OverlayPosition::NoChange => "NoChange",
            OverlayPosition::Below => "Below",
            OverlayPosition::Above => "Above",
        };

        write!(f, "{value}")
    }
}

impl Default for OverlayPosition {
    fn default() -> Self {
        Self::NoChange
    }
}

impl FromStr for OverlayPosition {
    type Err = OverlayPositionParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "NoChange" => Ok(OverlayPosition::NoChange),
            "Below" => Ok(OverlayPosition::Below),
            "Above" => Ok(OverlayPosition::Above),
            _ => Err(OverlayPositionParseError),
        }
    }
}

/// Error used when there's an error parsing the string as enum
#[derive(Debug)]
pub struct OverlayPositionParseError;

impl Display for OverlayPositionParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error trying to parse `value` as OverlayPosition")
    }
}

impl Error for OverlayPositionParseError {}
