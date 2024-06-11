use strum_macros::{EnumCount, FromRepr};

#[derive(FromRepr, EnumCount, Clone, Copy, Debug, Default, PartialEq)]
#[repr(usize)]
pub enum Tag {
    #[default]
    Unknown = 0,
    Checksum = 1,
    PrecisionTimeStamp = 2,
    MissionID = 3,
    PlatformTailNumber = 4,
    PlatformHeadingAngle = 5,
    PlatformPitchAngle = 6,
    PlatformRollAngle = 7,
    PlatformTrueAirspeed = 8,
    PlatformIndicatedAirspeed = 9,
    PlatformDesignation = 10,
    ImageSourceSensor = 11,
    ImageCoordinateSystem = 12,
    SensorLatitude = 13,
    SensorLongitude = 14,
    SensorTrueAltitude = 15,
    SensorHorizontalFieldOfView = 16,
    SensorVerticalFieldOfView = 17,
    SensorRelativeAzimuthAngle = 18,
    SensorRelativeElevationAngle = 19,
    SensorRelativeRollAngle = 20,
    SlantRange = 21,
    TargetWidth = 22,
    FrameCenterLatitude = 23,
    FrameCenterLongitude = 24,
    FrameCenterElevation = 25,
}