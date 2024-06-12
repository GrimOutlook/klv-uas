//! This issue is really screwing me: https://github.com/rust-lang/rfcs/issues/754.
//!
//! I would love to be able to store concrete KlvValue enum type in the Tag enum. So when the user
//! passes in the Tag they want from the KLV packet we can just determine how to parse the data but
//! this is not currently supported and at the time of writing this, appears that it never will be.
//!
//! I consulted this stack overflow post (made by an absolute genius) in order to reduce code
//! duplication.
//! https://stackoverflow.com/questions/36928569/how-can-i-create-enums-with-constant-values-in-rust

use strum_macros::{EnumCount, EnumDiscriminants, FromRepr};
use crate::klv_value::{KlvValue, KlvValueType};

/// Reference summary section of the [`KlvValue`] page for more in-depth reasoning but this macro
/// is shamelessly stolen from an absolute genius's
/// ([vallentin](https://stackoverflow.com/users/2470818/vallentin))
/// [post on stack overflow](https://stackoverflow.com/questions/36928569/how-can-i-create-enums-with-constant-values-in-rust).
macro_rules! def_tags {
    (
        $(#[$attr:meta])*
        $vis:vis $name:ident => $ret_type:ty {
            $($variant:ident => ($typ:expr, $id:expr));+
            $(,)?
        }
    ) => {
        $(#[$attr])*
        $vis struct $name($ret_type);

        impl $name {
            $(
                pub const $variant: Self = Self($typ, $id);
            )+

            pub const VARIANTS: &'static [Self] = &[$(Self::$variant),+];

            pub const fn r#type(self) -> $ret_type {
                self.0
            }

            pub const fn id(self) -> $ret_type {
                self.1
            }
        }
    };
}

def_tags! {
    #[derive(Clone, Copy, Debug, FromRepr, PartialEq)]
    pub Tag => KlvValueType {
        Checksum => (KlvValueType::Uint16, 1);
        PrecisionTimeStamp => (KlvValueType::Uint64, 2);
        MissionID => (KlvValueType::Utf8, 3);
        // PlatformTailNumber = 4,
        // PlatformHeadingAngle = 5,
        // PlatformPitchAngle = 6,
        // PlatformRollAngle = 7,
        // PlatformTrueAirspeed = 8,
        // PlatformIndicatedAirspeed = 9,
        // PlatformDesignation = 10,
        // ImageSourceSensor = 11,
        // ImageCoordinateSystem = 12,
        // SensorLatitude = 13,
        // SensorLongitude = 14,
        // SensorTrueAltitude = 15,
        // SensorHorizontalFieldOfView = 16,
        // SensorVerticalFieldOfView = 17,
        // SensorRelativeAzimuthAngle = 18,
        // SensorRelativeElevationAngle = 19,
        // SensorRelativeRollAngle = 20,
        // SlantRange = 21,
        // TargetWidth = 22,
        // FrameCenterLatitude = 23,
        // FrameCenterLongitude = 24,
        // FrameCenterElevation = 25,
        // OffsetCornerLatitudePoint1 = 26,
        // OffsetCornerLongitudePoint1 = 27,
        // OffsetCornerLatitudePoint2 = 28,
        // OffsetCornerLongitudePoint2 = 29,
        // OffsetCornerLatitudePoint3 = 30,
        // OffsetCornerLongitudePoint3 = 31,
        // OffsetCornerLatitudePoint4 = 32,
        // OffsetCornerLongitudePoint4 = 33,
        // IcingDetected = 34,
        // WindDirection = 35,
        // WindSpeed = 36,
        // StaticPressure = 37,
        // DensityAltitude = 38,
        // OutsideAirTemperature = 39,
        // TargetLocationLatitude = 40,
        // TargetLocationLongitude = 41,
        // TargetLocationElevation = 42,
        // TargetTrackGateWidth = 43,
        // TargetTrackGateHeight = 44,
        // TargetErrorEstimateCE90 = 45,
        // TargetErrorEstimateLe90 = 46,
        // GenericFlagData = 47,
        // SecurityLocalSet = 48,
        // DifferentialPressure = 49,
        // PlatformAngleOfAttack = 50,
        // PlatformVerticalSpeed = 51,
        // PlatformSideslipAngle = 52,
        // AirfieldBarometricPressure = 53,
        // AirfieldElevation = 54,
        // RelativeHumidity = 55,
        // PlatformGroundSpeed = 56,
        // GroundRange = 57,
        // PlatformFuelRemaining = 58,
        // PlatformCallSign = 59,
        // WeaponLoad = 60,
        // WeaponFired = 61,
        // LaserPrfCode = 62,
        // SensorFieldOfViewName = 63,
        // PlatformMagneticHeading = 64,
        // UasDatalinkLsVersionNumber = 65,
        // Deprecated = 66,
        // AlternatePlatformLatitude = 67,
        // AlternatePlatformLongitude = 68,
        // AlternatePlatformAltitude = 69,
        // AlternatePlatformName = 70,
        // AlternatePlatformHeading = 71,
        // EventStartTime = 72,
        // RvtLocalSet = 73,
        // VmtiLocalSet = 74,
        // SensorEllipsoidHeight = 75,
        // AlternatePlatformEllipsoidHeight = 76,
        // OperationalMode = 77,
        // FrameCenterHeightAboveEllipsoid = 78,
        // SensorNorthVelocity = 79,
        // SensorEastVelocity = 80,
        // ImageHorizonPixelPack = 81,
        // CornerLatitudePoint1Full = 82,
        // CornerLongitudePoint1Full = 83,
        // CornerLatitudePoint2Full = 84,
        // CornerLongitudePoint2Full = 85,
        // CornerLatitudePoint3Full = 86,
        // CornerLongitudePoint3Full = 87,
        // CornerLatitudePoint4Full = 88,
        // CornerLongitudePoint4Full = 89,
        // PlatformPitchAngleFull = 90,
        // PlatformRollAngleFull = 91,
        // PlatformAngleOfAttackFull = 92,
        // PlatformSideslipAngleFull = 93,
        // MiisCoreIdentifier = 94,
        // SarMotionImageryLocalSet = 95,
        // TargetWidthExtended = 96,
        // RangeImageLocalSet = 97,
        // GeoRegistrationLocalSet = 98,
        // CompositeImagingLocalSet = 99,
        // SegmentLocalSet = 100,
        // AmendLocalSet = 101,
        // SdccFlp = 102,
        // DensityAltitudeExtended = 103,
        // SensorEllipsoidHeightExtended = 104,
        // AlternatePlatformEllipsoidHeightExtended = 105,
        // StreamDesignator = 106,
        // OperationalBase = 107,
        // BroadcastSource = 108,
        // RangeToRecoveryLocation = 109,
        // TimeAirborne = 110,
        // PropulsionUnitSpeed = 111,
        // PlatformCourseAngle = 112,
        // AltitudeAgl = 113,
        // RadarAltimeter = 114,
        // ControlCommand = 115,
        // ControlCommandVerificationList = 116,
        // SensorAzimuthRate = 117,
        // SensorElevationRate = 118,
        // SensorRollRate = 119,
        // OnboardMiStoragePercentFull = 120,
        // ActiveWaypointList = 121,
        // CountryCodes = 122,
        // NumberOfNavsatsInView = 123,
        // PositionMethodSource = 124,
        // PlatformStatus = 125,
        // SensorControlMode = 126,
        // SensorFrameRatePack = 127,
        // WavelengthsList = 128,
        // TargetId = 129,
        // AirbaseLocations = 130,
        // TakeoffTime = 131,
        // TransmissionFrequency = 132,
        // OnboardMiStorageCapacity = 133,
        // ZoomPercentage = 134,
        // CommunicationsMethod = 135,
        // LeapSeconds = 136,
        // CorrectionOffset = 137,
        // PayloadList = 138,
        // ActivePayloads = 139,
        // WeaponStores = 140,
        // WaypointList = 141,
        // ViewDomain = 142,
        // MetadataSubstreamIdPack = 143,
    }
}