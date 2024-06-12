//! This issue is really screwing me: https://github.com/rust-lang/rfcs/issues/754.
//!
//! I would love to be able to store concrete KlvValue enum type in the Tag enum. So when the user
//! passes in the Tag they want from the KLV packet we can just determine how to parse the data but
//! this is not currently supported and at the time of writing this, appears that it never will be.
//!
//! I consulted this stack overflow post (made by an absolute genius) in order to reduce code
//! duplication.
//! https://stackoverflow.com/questions/36928569/how-can-i-create-enums-with-constant-values-in-rust



use crate::klv_value::KlvValueType;

/// Reference summary section of the [`KlvValue`] page for more in-depth reasoning but this macro
/// is shamelessly stolen from an absolute genius's
/// ([vallentin](https://stackoverflow.com/users/2470818/vallentin))
/// [post on stack overflow](https://stackoverflow.com/questions/36928569/how-can-i-create-enums-with-constant-values-in-rust).
macro_rules! def_tags {
    (
        $(#[$attr:meta])*
        $vis:vis $name:ident => $ret_typ:ty, $ret_id:ty {
            $($variant:ident => $typ:expr, $id:expr);+
            $(;)?
        }
    ) => {
        $(#[$attr])*
        $vis struct $name($ret_typ, $ret_id);

        // We allow non upper case globals here because we are using these constants as if they are
        // Enums.
        #[allow(non_upper_case_globals)]
        impl $name {
            $(
                pub const $variant: Self = Self($typ, $id);
            )+

            pub const VARIANTS: &'static [Self] = &[$(Self::$variant),+];
            pub const COUNT: usize = [$(Self::$variant),+].len();

            pub const fn tag_type(self) -> $ret_typ {
                self.0
            }

            pub const fn id(self) -> $ret_id {
                self.1
            }
        }

        impl Into<$ret_typ> for $name {
            fn into(self) -> $ret_typ {
                self.tag_type()
            }
        }

        impl Into<$ret_id> for $name {
            fn into(self) -> $ret_id {
                self.id()
            }
        }

        impl From<$ret_id> for $name {
            fn from(id: $ret_id) -> $name {
                match Self::VARIANTS.iter().find(|v| v.id() == id) {
                    Some(v) => *v,
                    None => Tag::Unknown,
                }
            }
        }
    };
}

def_tags! {
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub Tag => KlvValueType, usize {
        Unknown                                     => KlvValueType::Unknown,        0;
        Checksum                                    => KlvValueType::Uint16,        1;
        PrecisionTimeStamp                          => KlvValueType::Uint64,        2;
        MissionID                                   => KlvValueType::Utf8,          3;
        PlatformTailNumber                          => KlvValueType::Utf8,          4;
        PlatformHeadingAngle                        => KlvValueType::Uint16,        5;
        PlatformPitchAngle                          => KlvValueType::Int16,         6;
        PlatformRollAngle                           => KlvValueType::Int16,         7;
        PlatformTrueAirspeed                        => KlvValueType::Uint8,         8;
        PlatformIndicatedAirspeed                   => KlvValueType::Uint8,         9;
        PlatformDesignation                         => KlvValueType::Utf8,          10;
        ImageSourceSensor                           => KlvValueType::Utf8,          11;
        ImageCoordinateSystem                       => KlvValueType::Utf8,          12;
        SensorLatitude                              => KlvValueType::Int32,         13;
        SensorLongitude                             => KlvValueType::Int32,         14;
        SensorTrueAltitude                          => KlvValueType::Uint16,        15;
        SensorHorizontalFieldOfView                 => KlvValueType::Uint16,        16;
        SensorVerticalFieldOfView                   => KlvValueType::Uint16,        17;
        SensorRelativeAzimuthAngle                  => KlvValueType::Uint32,        18;
        SensorRelativeElevationAngle                => KlvValueType::Int32,         19;
        SensorRelativeRollAngle                     => KlvValueType::Uint32,        20;
        SlantRange                                  => KlvValueType::Uint32,        21;
        TargetWidth                                 => KlvValueType::Uint16,        22;
        FrameCenterLatitude                         => KlvValueType::Int32,         23;
        FrameCenterLongitude                        => KlvValueType::Int32,         24;
        FrameCenterElevation                        => KlvValueType::Uint16,        25;
        OffsetCornerLatitudePoint1                  => KlvValueType::Int16,         26;
        OffsetCornerLongitudePoint1                 => KlvValueType::Int16,         27;
        OffsetCornerLatitudePoint2                  => KlvValueType::Int16,         28;
        OffsetCornerLongitudePoint2                 => KlvValueType::Int16,         29;
        OffsetCornerLatitudePoint3                  => KlvValueType::Int16,         30;
        OffsetCornerLongitudePoint3                 => KlvValueType::Int16,         31;
        OffsetCornerLatitudePoint4                  => KlvValueType::Int16,         32;
        OffsetCornerLongitudePoint4                 => KlvValueType::Int16,         33;
        IcingDetected                               => KlvValueType::Uint8,         34;
        WindDirection                               => KlvValueType::Uint16,        35;
        WindSpeed                                   => KlvValueType::Uint8,         36;
        StaticPressure                              => KlvValueType::Uint16,        37;
        DensityAltitude                             => KlvValueType::Uint16,        38;
        OutsideAirTemperature                       => KlvValueType::Int8,          39;
        TargetLocationLatitude                      => KlvValueType::Int32,         40;
        TargetLocationLongitude                     => KlvValueType::Int32,         41;
        TargetLocationElevation                     => KlvValueType::Uint16,        42;
        TargetTrackGateWidth                        => KlvValueType::Uint8,         43;
        TargetTrackGateHeight                       => KlvValueType::Uint8,         44;
        TargetErrorEstimateCE90                     => KlvValueType::Uint16,        45;
        TargetErrorEstimateLe90                     => KlvValueType::Uint16,        46;
        GenericFlagData                             => KlvValueType::Uint8,         47;
        SecurityLocalSet                            => KlvValueType::Set,           48;
        DifferentialPressure                        => KlvValueType::Uint16,        49;
        PlatformAngleOfAttack                       => KlvValueType::Int16,         50;
        PlatformVerticalSpeed                       => KlvValueType::Int16,         51;
        PlatformSideslipAngle                       => KlvValueType::Int16,         52;
        AirfieldBarometricPressure                  => KlvValueType::Uint16,        53;
        AirfieldElevation                           => KlvValueType::Uint16,        54;
        RelativeHumidity                            => KlvValueType::Uint8,         55;
        PlatformGroundSpeed                         => KlvValueType::Uint8,         56;
        GroundRange                                 => KlvValueType::Uint32,        57;
        PlatformFuelRemaining                       => KlvValueType::Uint16,        58;
        PlatformCallSign                            => KlvValueType::Utf8,          59;
        WeaponLoad                                  => KlvValueType::Uint16,        60;
        WeaponFired                                 => KlvValueType::Uint8,         61;
        LaserPrfCode                                => KlvValueType::Uint16,        62;
        SensorFieldOfViewName                       => KlvValueType::Uint8,         63;
        PlatformMagneticHeading                     => KlvValueType::Uint16,        64;
        UasDatalinkLsVersionNumber                  => KlvValueType::Uint8,         65;
        Deprecated                                  => KlvValueType::Deprecated,    66;
        AlternatePlatformLatitude                   => KlvValueType::Int32,         67;
        AlternatePlatformLongitude                  => KlvValueType::Int32,         68;
        AlternatePlatformAltitude                   => KlvValueType::Uint16,        69;
        AlternatePlatformName                       => KlvValueType::Utf8,          70;
        AlternatePlatformHeading                    => KlvValueType::Uint16,        71;
        EventStartTime                              => KlvValueType::Uint64,        72;
        RvtLocalSet                                 => KlvValueType::Set,           73;
        VmtiLocalSet                                => KlvValueType::Set,           74;
        SensorEllipsoidHeight                       => KlvValueType::Uint16,        75;
        AlternatePlatformEllipsoidHeight            => KlvValueType::Uint16,        76;
        OperationalMode                             => KlvValueType::Uint8,         77;
        FrameCenterHeightAboveEllipsoid             => KlvValueType::Uint16,        78;
        SensorNorthVelocity                         => KlvValueType::Int16,         79;
        SensorEastVelocity                          => KlvValueType::Int16,         80;
        ImageHorizonPixelPack                       => KlvValueType::DLP,           81;
        CornerLatitudePoint1Full                    => KlvValueType::Int32,         82;
        CornerLongitudePoint1Full                   => KlvValueType::Int32,         83;
        CornerLatitudePoint2Full                    => KlvValueType::Int32,         84;
        CornerLongitudePoint2Full                   => KlvValueType::Int32,         85;
        CornerLatitudePoint3Full                    => KlvValueType::Int32,         86;
        CornerLongitudePoint3Full                   => KlvValueType::Int32,         87;
        CornerLatitudePoint4Full                    => KlvValueType::Int32,         88;
        CornerLongitudePoint4Full                   => KlvValueType::Int32,         89;
        PlatformPitchAngleFull                      => KlvValueType::Int32,         90;
        PlatformRollAngleFull                       => KlvValueType::Int32,         91;
        PlatformAngleOfAttackFull                   => KlvValueType::Int32,         92;
        PlatformSideslipAngleFull                   => KlvValueType::Int32,         93;
        MiisCoreIdentifier                          => KlvValueType::Byte,          94;
        SarMotionImageryLocalSet                    => KlvValueType::Set,           95;
        TargetWidthExtended                         => KlvValueType::IMAPB,         96;
        RangeImageLocalSet                          => KlvValueType::Set,           97;
        GeoRegistrationLocalSet                     => KlvValueType::Set,           98;
        CompositeImagingLocalSet                    => KlvValueType::Set,           99;
        SegmentLocalSet                             => KlvValueType::Set,           100;
        AmendLocalSet                               => KlvValueType::Set,           101;
        SdccFlp                                     => KlvValueType::FLP,           102;
        DensityAltitudeExtended                     => KlvValueType::IMAPB,         103;
        SensorEllipsoidHeightExtended               => KlvValueType::IMAPB,         104;
        AlternatePlatformEllipsoidHeightExtended    => KlvValueType::IMAPB,         105;
        StreamDesignator                            => KlvValueType::Utf8,          106;
        OperationalBase                             => KlvValueType::Utf8,          107;
        BroadcastSource                             => KlvValueType::Utf8,          108;
        RangeToRecoveryLocation                     => KlvValueType::IMAPB,         109;
        TimeAirborne                                => KlvValueType::Uint,          110;
        PropulsionUnitSpeed                         => KlvValueType::Uint,          111;
        PlatformCourseAngle                         => KlvValueType::IMAPB,         112;
        AltitudeAgl                                 => KlvValueType::IMAPB,         113;
        RadarAltimeter                              => KlvValueType::IMAPB,         114;
        ControlCommand                              => KlvValueType::DLP,           115;
        ControlCommandVerificationList              => KlvValueType::DLP,           116;
        SensorAzimuthRate                           => KlvValueType::IMAPB,         117;
        SensorElevationRate                         => KlvValueType::IMAPB,         118;
        SensorRollRate                              => KlvValueType::IMAPB,         119;
        OnboardMiStoragePercentFull                 => KlvValueType::IMAPB,         120;
        ActiveWaypointList                          => KlvValueType::DLP,           121;
        CountryCodes                                => KlvValueType::VLP,           122;
        NumberOfNavsatsInView                       => KlvValueType::Uint,          123;
        PositionMethodSource                        => KlvValueType::Uint,          124;
        PlatformStatus                              => KlvValueType::Uint,          125;
        SensorControlMode                           => KlvValueType::Uint,          126;
        SensorFrameRatePack                         => KlvValueType::DLP,           127;
        WavelengthsList                             => KlvValueType::VLP,           128;
        TargetId                                    => KlvValueType::Utf8,          129;
        AirbaseLocations                            => KlvValueType::VLP,           130;
        TakeoffTime                                 => KlvValueType::Uint,          131;
        TransmissionFrequency                       => KlvValueType::IMAPB,         132;
        OnboardMiStorageCapacity                    => KlvValueType::Uint,          133;
        ZoomPercentage                              => KlvValueType::IMAPB,         134;
        CommunicationsMethod                        => KlvValueType::Utf8,          135;
        LeapSeconds                                 => KlvValueType::Int,           136;
        CorrectionOffset                            => KlvValueType::Int,           137;
        PayloadList                                 => KlvValueType::VLP,           138;
        ActivePayloads                              => KlvValueType::Byte,          139;
        WeaponStores                                => KlvValueType::VLP,           140;
        WaypointList                                => KlvValueType::VLP,           141;
        ViewDomain                                  => KlvValueType::VLP,           142;
        MetadataSubstreamIdPack                     => KlvValueType::Byte,          143;
    }
}