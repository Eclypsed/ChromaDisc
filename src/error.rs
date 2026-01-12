//! Errors to identify sgio failures straight from the MMC-6 spec

use mmc_errors::MMCError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MMCError {
    #[error(transparent)]
    UnitAttentionCondition(#[from] UnitAttentionCondition),
    #[error(transparent)]
    CDBOrParameterValidationError(#[from] CDBOrParameterValidationError),
    #[error(transparent)]
    ReadinessError(#[from] ReadinessError),
    #[error(transparent)]
    ProtocolError(#[from] ProtocolError),
    #[error(transparent)]
    GeneralMediaAccessError(#[from] GeneralMediaAccessError),
    #[error(transparent)]
    ReadingError(#[from] ReadingError),
    #[error(transparent)]
    WritingError(#[from] WritingError),
    #[error(transparent)]
    HardwareFailure(#[from] HardwareFailure),
    #[error(transparent)]
    NonATAPIEnvironmentError(#[from] NonATAPIEnvironmentError),
}

impl MMCError {
    pub fn from_codes(sk: u8, asc: u8, ascq: u8) -> Option<Self> {
        UnitAttentionCondition::from_codes(sk, asc, ascq)
            .map(Self::UnitAttentionCondition)
            .or_else(|| {
                CDBOrParameterValidationError::from_codes(sk, asc, ascq)
                    .map(Self::CDBOrParameterValidationError)
            })
            .or_else(|| ReadinessError::from_codes(sk, asc, ascq).map(Self::ReadinessError))
            .or_else(|| ProtocolError::from_codes(sk, asc, ascq).map(Self::ProtocolError))
            .or_else(|| {
                GeneralMediaAccessError::from_codes(sk, asc, ascq)
                    .map(Self::GeneralMediaAccessError)
            })
            .or_else(|| ReadingError::from_codes(sk, asc, ascq).map(Self::ReadingError))
            .or_else(|| WritingError::from_codes(sk, asc, ascq).map(Self::WritingError))
            .or_else(|| HardwareFailure::from_codes(sk, asc, ascq).map(Self::HardwareFailure))
            .or_else(|| {
                NonATAPIEnvironmentError::from_codes(sk, asc, ascq)
                    .map(Self::NonATAPIEnvironmentError)
            })
    }
}

#[derive(Error, MMCError, Debug)]
pub enum UnitAttentionCondition {
    #[error("NOT READY TO READY CHANGE, MEDIUM MAY HAVE CHANGED")]
    #[mmc_error(sk = 0x6, asc = 0x28, ascq = 0x00)]
    NotReadyToReadyChangeMediumMayHaveChanged,
    #[error("IMPORT OR EXPORT ELEMENT ACCESSED")]
    #[mmc_error(sk = 0x6, asc = 0x28, ascq = 0x01)]
    ImportOrExportElementAccessed,
    #[error("FORMAT-LAYER MAY HAVE CHANGED")]
    #[mmc_error(sk = 0x6, asc = 0x28, ascq = 0x02)]
    FormatLayerMayHaveChanged,
    #[error("POWER ON, RESET, OR BUS DEVICE RESET OCCURRED")]
    #[mmc_error(sk = 0x6, asc = 0x29, ascq = 0x00)]
    PowerOnResetOrBusDeviceResetOccured,
    #[error("POWER ON OCCURRED")]
    #[mmc_error(sk = 0x6, asc = 0x29, ascq = 0x01)]
    PowerOnOccured,
    #[error("BUS RESET OCCURRED")]
    #[mmc_error(sk = 0x6, asc = 0x29, ascq = 0x02)]
    BusResetOccured,
    #[error("BUS DEVICE RESET FUNCTION OCCURRED")]
    #[mmc_error(sk = 0x6, asc = 0x29, ascq = 0x03)]
    BusDeviceResetFunctionOccured,
    #[error("DEVICE INTERNAL RESET")]
    #[mmc_error(sk = 0x6, asc = 0x29, ascq = 0x04)]
    DeviceInternalReset,
    #[error("PARAMETERS CHANGED")]
    #[mmc_error(sk = 0x6, asc = 0x2A, ascq = 0x00)]
    ParametersChanged,
    #[error("MODE PARAMETERS CHANGED")]
    #[mmc_error(sk = 0x6, asc = 0x2A, ascq = 0x01)]
    ModeParametersChanged,
    #[error("LOG PARAMETERS CHANGED")]
    #[mmc_error(sk = 0x6, asc = 0x2A, ascq = 0x02)]
    LogParametersChanged,
    #[error("INSUFFICIENT TIME FOR OPERATION")]
    #[mmc_error(sk = 0x6, asc = 0x2E, ascq = 0x00)]
    InsufficientTimeForOperation,
    #[error("MEDIUM DESTINATION ELEMENT FULL")]
    #[mmc_error(sk = 0x6, asc = 0x3B, ascq = 0x0D)]
    MediumDestinationElementFull,
    #[error("MEDIUM SOURCE ELEMENT EMPTY")]
    #[mmc_error(sk = 0x6, asc = 0x3B, ascq = 0x0E)]
    MediumSourceElementEmpty,
    #[error("END OF MEDIUM REACHED")]
    #[mmc_error(sk = 0x6, asc = 0x3B, ascq = 0x0F)]
    EndOfMediumReached,
    #[error("MEDIUM MAGAZINE NOT ACCESSIBLE")]
    #[mmc_error(sk = 0x6, asc = 0x3B, ascq = 0x11)]
    MediumMagazineNotAccessible,
    #[error("MEDIUM MAGAZINE REMOVED")]
    #[mmc_error(sk = 0x6, asc = 0x3B, ascq = 0x12)]
    MediumMagazineRemoved,
    #[error("MEDIUM MAGAZINE INSERTED")]
    #[mmc_error(sk = 0x6, asc = 0x3B, ascq = 0x13)]
    MediumMagazineInserted,
    #[error("MEDIUM MAGAZINE LOCKED")]
    #[mmc_error(sk = 0x6, asc = 0x3B, ascq = 0x14)]
    MediumMagazineLocked,
    #[error("MEDIUM MAGAZINE UNLOCKED")]
    #[mmc_error(sk = 0x6, asc = 0x3B, ascq = 0x15)]
    MediumMagazineUnlocked,
    #[error("TARGET OPERATING CONDITIONS HAVE CHANGED")]
    #[mmc_error(sk = 0x6, asc = 0x3F, ascq = 0x00)]
    TargetOperatingConditionsHaveChanged,
    #[error("MICROCODE HAS BEEN CHANGED")]
    #[mmc_error(sk = 0x6, asc = 0x3F, ascq = 0x01)]
    MicrocodeHasBeenChanged,
    #[error("CHANGED OPERATING DEFINITION")]
    #[mmc_error(sk = 0x6, asc = 0x3F, ascq = 0x02)]
    ChangedOperatingDefinition,
    #[error("INQUIRY DATA HAS CHANGED")]
    #[mmc_error(sk = 0x6, asc = 0x3F, ascq = 0x03)]
    InquiryDataHasChanged,
    #[error("OPERATOR REQUEST OR STATE CHANGE INPUT")]
    #[mmc_error(sk = 0x6, asc = 0x5A, ascq = 0x00)]
    OperatorRequestOrStateChangeInput,
    #[error("OPERATOR MEDIUM REMOVAL REQUEST")]
    #[mmc_error(sk = 0x6, asc = 0x5A, ascq = 0x01)]
    OperatorMediumRemovalRequest,
    #[error("OPERATOR SELECTED WRITE PROTECT")]
    #[mmc_error(sk = 0x6, asc = 0x5A, ascq = 0x02)]
    OperatorSelectedWriteProtect,
    #[error("OPERATOR SELECTED WRITE PERMIT")]
    #[mmc_error(sk = 0x6, asc = 0x5A, ascq = 0x03)]
    OperatorSelectedWritePermit,
    #[error("LOG EXCEPTION")]
    #[mmc_error(sk = 0x6, asc = 0x5B, ascq = 0x00)]
    LogException,
    #[error("THRESHOLD CONDITION MET")]
    #[mmc_error(sk = 0x6, asc = 0x5B, ascq = 0x01)]
    ThresholdConditionMet,
    #[error("LOG COUNTER AT MAXIMUM")]
    #[mmc_error(sk = 0x6, asc = 0x5B, ascq = 0x02)]
    LogCounterAtMaximum,
    #[error("LOG LIST CODES EXHAUSTED")]
    #[mmc_error(sk = 0x6, asc = 0x5B, ascq = 0x03)]
    LogListCodesExhausted,
    #[error("LOW POWER CONDITION ON")]
    #[mmc_error(sk = 0x6, asc = 0x5E, ascq = 0x00)]
    LowPowerConditionOn,
    #[error("IDLE CONDITION ACTIVATED BY TIMER")]
    #[mmc_error(sk = 0x6, asc = 0x5E, ascq = 0x01)]
    IdleConditionActivatedByTimer,
    #[error("STANDBY CONDITION ACTIVATED BY TIMER")]
    #[mmc_error(sk = 0x6, asc = 0x5E, ascq = 0x02)]
    StandbyConditionActivatedByTimer,
    #[error("IDLE CONDITION ACTIVATED BY COMMAND")]
    #[mmc_error(sk = 0x6, asc = 0x5E, ascq = 0x03)]
    IdleConditionActivatedByCommand,
    #[error("STANDBY CONDITION ACTIVATED BY COMMAND")]
    #[mmc_error(sk = 0x6, asc = 0x5E, ascq = 0x04)]
    StandbyConditionActivatedByCommand,
}

#[derive(Error, MMCError, Debug)]
pub enum CDBOrParameterValidationError {
    #[error("PARAMETER LIST LENGTH ERROR")]
    #[mmc_error(sk = 0x5, asc = 0x1A, ascq = 0x00)]
    ParameterListLengthError,
    #[error("INVALID COMMAND OPERATION CODE")]
    #[mmc_error(sk = 0x5, asc = 0x20, ascq = 0x00)]
    InvalidCommandOperationCode,
    #[error("LOGICAL BLOCK ADDRESS OUT OF RANGE")]
    #[mmc_error(sk = 0x5, asc = 0x21, ascq = 0x00)]
    LogicalBlockAddressOutOfRange,
    #[error("INVALID ELEMENT ADDRESS")]
    #[mmc_error(sk = 0x5, asc = 0x21, ascq = 0x01)]
    InvalidElementAddress,
    #[error("INVALID ADDRESS FOR WRITE")]
    #[mmc_error(sk = 0x5, asc = 0x21, ascq = 0x02)]
    InvalidAddressForWrite,
    #[error("INVALID WRITE CROSSING LAYER JUMP")]
    #[mmc_error(sk = 0x5, asc = 0x21, ascq = 0x03)]
    InvalidWriteCrossingLayerJump,
    #[error("INVALID FUNCTION")]
    #[mmc_error(sk = 0x5, asc = 0x22, ascq = 0x00)]
    InvalidFunction,
    #[error("INVALID FIELD IN CDB")]
    #[mmc_error(sk = 0x5, asc = 0x24, ascq = 0x00)]
    InvalidFieldInCdb,
    #[error("INVALID FIELD IN PARAMETER LIST")]
    #[mmc_error(sk = 0x5, asc = 0x26, ascq = 0x00)]
    InvalidFieldInParameterList,
    #[error("PARAMETER NOT SUPPORTED")]
    #[mmc_error(sk = 0x5, asc = 0x26, ascq = 0x01)]
    ParameterNotSupported,
    #[error("PARAMETER VALUE INVALID")]
    #[mmc_error(sk = 0x5, asc = 0x26, ascq = 0x02)]
    ParameterValueInvalid,
    #[error("THRESHOLD PARAMETERS NOT SUPPORTED")]
    #[mmc_error(sk = 0x5, asc = 0x26, ascq = 0x03)]
    ThresholdParametersNotSupporter,
}

#[derive(Error, MMCError, Debug)]
pub enum ReadinessError {
    #[error("LOGICAL UNIT NOT READY, CAUSE NOT REPORTABLE")]
    #[mmc_error(sk = 0x2, asc = 0x04, ascq = 0x00)]
    LogicalUnitNotReadyCauseNotReportable,
    #[error("LOGICAL UNIT IS IN PROCESS OF BECOMING READY")]
    #[mmc_error(sk = 0x2, asc = 0x04, ascq = 0x01)]
    LogicalUnitIsInProcessOfBecomingReady,
    #[error("LOGICAL UNIT NOT READY, INITIALIZING CMD. REQUIRED")]
    #[mmc_error(sk = 0x2, asc = 0x04, ascq = 0x02)]
    LogicalUnitNotReadyInitializingCmdRequired,
    #[error("LOGICAL UNIT NOT READY, MANUAL INTERVENTION REQUIRED")]
    #[mmc_error(sk = 0x2, asc = 0x04, ascq = 0x03)]
    LogicalUnitNotReadyManualInterventionRequired,
    #[error("LOGICAL UNIT NOT READY, FORMAT IN PROGRESS")]
    #[mmc_error(sk = 0x2, asc = 0x04, ascq = 0x04)]
    LogicalUnitNotReadyFormatInProgress,
    #[error("LOGICAL UNIT NOT READY, OPERATION IN PROGRESS")]
    #[mmc_error(sk = 0x2, asc = 0x04, ascq = 0x07)]
    LogicalUnitNotReadyOperationInProgress,
    #[error("LOGICAL UNIT NOT READY, LONG WRITE IN PROGRESS")]
    #[mmc_error(sk = 0x2, asc = 0x04, ascq = 0x08)]
    LogicalUnitNotReadyLongWriteInProgress,
    #[error("WRITE ERROR RECOVERY NEEDED")]
    #[mmc_error(sk = 0x2, asc = 0x0C, ascq = 0x07)]
    WriteErrorRecoveryNeeded,
    #[error("DEFECTS IN ERROR WINDOW")]
    #[mmc_error(sk = 0x2, asc = 0x0C, ascq = 0x0F)]
    DefectsInErrorWindow,
    #[error("INCOMPATIBLE MEDIUM INSTALLED")]
    #[mmc_error(sk = 0x2, asc = 0x30, ascq = 0x00)]
    IncompatibleMediumInstalled,
    #[error("CANNOT READ MEDIUM – UNKNOWN FORMAT")]
    #[mmc_error(sk = 0x2, asc = 0x30, ascq = 0x01)]
    CannotReadMediumUnknownFormat,
    #[error("CANNOT READ MEDIUM – INCOMPATIBLE FORMAT")]
    #[mmc_error(sk = 0x2 | 0x5, asc = 0x30, ascq = 0x02)]
    CannotReadMediumIncompatibleFormat,
    #[error("CLEANING CARTRIDGE INSTALLED")]
    #[mmc_error(sk = 0x2, asc = 0x30, ascq = 0x03)]
    CleaningCartridgeInstalled,
    #[error("CANNOT WRITE MEDIUM – UNKNOWN FORMAT")]
    #[mmc_error(sk = 0x2 | 0x5, asc = 0x30, ascq = 0x04)]
    CannotWriteMediumUnknownFormat,
    #[error("CANNOT WRITE MEDIUM – INCOMPATIBLE FORMAT")]
    #[mmc_error(sk = 0x2 | 0x5, asc = 0x30, ascq = 0x05)]
    CannotWriteMediumIncompatibleFormat,
    #[error("CANNOT FORMAT MEDIUM – INCOMPATIBLE MEDIUM")]
    #[mmc_error(sk = 0x2 | 0x5, asc = 0x30, ascq = 0x06)]
    CannotFormatMediumIncompatibleMedium,
    #[error("CLEANING FAILURE")]
    #[mmc_error(sk = 0x2, asc = 0x30, ascq = 0x07)]
    CleaningFailure,
    #[error("CANNOT WRITE MEDIUM – UNSUPPORTED MEDIUM VERSION")]
    #[mmc_error(sk = 0x2 | 0x5, asc = 0x30, ascq = 0x11)]
    CannotWriteMediumUnsupportedMediumVersion,
    #[error("MEDIUM NOT PRESENT")]
    #[mmc_error(sk = 0x2, asc = 0x3A, ascq = 0x00)]
    MediumNotPresent,
    #[error("MEDIUM NOT PRESENT – TRAY CLOSED")]
    #[mmc_error(sk = 0x2, asc = 0x3A, ascq = 0x01)]
    MediumNotPresentTrayClosed,
    #[error("MEDIUM NOT PRESENT – TRAY OPEN")]
    #[mmc_error(sk = 0x2, asc = 0x3A, ascq = 0x02)]
    MediumNotPresentTrayOpen,
    #[error("LOGICAL UNIT HAS NOT SELF-CONFIGURED YET")]
    #[mmc_error(sk = 0x2, asc = 0x3E, ascq = 0x00)]
    LogicalUnitHasNotSelfConfiguredYet,
}

#[derive(Error, MMCError, Debug)]
pub enum ProtocolError {
    #[error("COMMAND SEQUENCE ERROR")]
    #[mmc_error(sk = 0x5, asc = 0x2C, ascq = 0x00)]
    CommandSequenceError,
    #[error("CURRENT PROGRAM AREA IS NOT EMPTY")]
    #[mmc_error(sk = 0x5, asc = 0x2C, ascq = 0x03)]
    CurrentProgramAreaIsNotEmpty,
    #[error("CURRENT PROGRAM AREA IS EMPTY")]
    #[mmc_error(sk = 0x5, asc = 0x2C, ascq = 0x04)]
    CurrentProgramAreaIsEmpty,
    #[error("CANNOT WRITE — APPLICATION CODE MISMATCH")]
    #[mmc_error(sk = 0x5, asc = 0x30, ascq = 0x08)]
    CannotWriteApplicationCodeMismatch,
    #[error("CURRENT SESSION NOT FIXATED FOR APPEND")]
    #[mmc_error(sk = 0x5, asc = 0x30, ascq = 0x09)]
    CurrentSessionNotFixatedForAppend,
    #[error("MEDIUM NOT FORMATTED")]
    #[mmc_error(sk = 0x5, asc = 0x30, ascq = 0x10)]
    MediumNotFormatted,
    #[error("SAVING PARAMETERS NOT SUPPORTED")]
    #[mmc_error(sk = 0x5, asc = 0x39, ascq = 0x00)]
    SavingParametersNotSupported,
    #[error("INVALID BITS IN IDENTIFY MESSAGE")]
    #[mmc_error(sk = 0x5, asc = 0x3D, ascq = 0x00)]
    InvalidBitsInIdentifyMessage,
    #[error("MESSAGE ERROR")]
    #[mmc_error(sk = 0x5, asc = 0x43, ascq = 0x00)]
    MessageError,
    #[error("MEDIUM REMOVAL PREVENTED")]
    #[mmc_error(sk = 0x5, asc = 0x53, ascq = 0x02)]
    MediumRemovalPrevented,
    #[error("ILLEGAL MODE FOR THIS TRACK")]
    #[mmc_error(sk = 0x5, asc = 0x64, ascq = 0x00)]
    IllegalModeForThisTrack,
    #[error("INVALID PACKET SIZE")]
    #[mmc_error(sk = 0x5, asc = 0x64, ascq = 0x01)]
    InvalidPacketSize,
    #[error("COPY PROTECTION KEY EXCHANGE FAILURE – AUTHENTICATION FAILURE")]
    #[mmc_error(sk = 0x5, asc = 0x6F, ascq = 0x00)]
    CopyProtectionKeyExchangeFailureAuthenticationFailure,
    #[error("COPY PROTECTION KEY EXCHANGE FAILURE – KEY NOT PRESENT")]
    #[mmc_error(sk = 0x5, asc = 0x6F, ascq = 0x01)]
    CopyProtectionKeyExchangeFailureKeyNotPresent,
    #[error("COPY PROTECTION KEY EXCHANGE FAILURE –KEY NOT ESTABLISHED")]
    #[mmc_error(sk = 0x5, asc = 0x6F, ascq = 0x02)]
    CopyProtectionKeyExchangeFailureKeyNotEstablished,
    #[error("READ OF SCRAMBLED SECTOR WITHOUT AUTHENTICATION")]
    #[mmc_error(sk = 0x5, asc = 0x6F, ascq = 0x03)]
    ReadOfScrambledSectorWithoutAuthentication,
    #[error("MEDIA REGION CODE IS MISMATCHED TO LOGICAL UNIT REGION")]
    #[mmc_error(sk = 0x5, asc = 0x6F, ascq = 0x04)]
    MediaRegionCodeIsMismatchedToLogicalUnitRegion,
    #[error("LOGICAL UNIT REGION MUST BE PERMANENT/REGION RESET COUNT ERROR")]
    #[mmc_error(sk = 0x5, asc = 0x6F, ascq = 0x05)]
    LogicalUnitRegionMustBePermanentRegionResetCountError,
    #[error("INSUFFICIENT BLOCK COUNT FOR BINDING NONCE RECORDING")]
    #[mmc_error(sk = 0x5, asc = 0x6F, ascq = 0x06)]
    InsufficientBlockCountForBindingNonceRecording,
    #[error("CONFLICT IN BINDING NONCE RECORDING")]
    #[mmc_error(sk = 0x5, asc = 0x6F, ascq = 0x07)]
    ConflictInBindingNonceRecording,
    #[error("EMPTY OR PARTIALLY WRITTEN RESERVED TRACK")]
    #[mmc_error(sk = 0x5, asc = 0x72, ascq = 0x04)]
    EmptyOrPartiallyWrittenReservedTrack,
    #[error("NO MORE TRACK RESERVATIONS ALLOWED")]
    #[mmc_error(sk = 0x5, asc = 0x72, ascq = 0x05)]
    NoMoreTrackReservationsAllowed,
}

#[derive(Error, MMCError, Debug)]
pub enum GeneralMediaAccessError {
    #[error("NO REFERENCE POSITION FOUND")]
    #[mmc_error(sk = 0x3, asc = 0x06, ascq = 0x00)]
    NoReferencePositionFound,
    #[error("TRACK FOLLOWING ERROR")]
    #[mmc_error(sk = 0x4, asc = 0x09, ascq = 0x00)]
    TrackFollowingError,
    #[error("TRACKING SERVO FAILURE")]
    #[mmc_error(sk = 0x4, asc = 0x09, ascq = 0x01)]
    TrackingServoFailure,
    #[error("FOCUS SERVO FAILURE")]
    #[mmc_error(sk = 0x4, asc = 0x09, ascq = 0x02)]
    FocusServoFailure,
    #[error("SPINDLE SERVO FAILURE")]
    #[mmc_error(sk = 0x4, asc = 0x09, ascq = 0x03)]
    SpindleServoFailure,
    #[error("RANDOM POSITIONING ERROR")]
    #[mmc_error(sk = 0x3, asc = 0x15, ascq = 0x00)]
    RandomPositioningError,
    #[error("MECHANICAL POSITIONING ERROR")]
    #[mmc_error(sk = 0x3, asc = 0x15, ascq = 0x01)]
    MechanicalPositioningError,
    #[error("INCOMPATIBLE MEDIUM INSTALLED")]
    #[mmc_error(sk = 0x5, asc = 0x30, ascq = 0x00)]
    IncompatibleMediumInstalled,
    #[error("CANNOT READ MEDIUM – UNKNOWN FORMAT")]
    #[mmc_error(sk = 0x5, asc = 0x30, ascq = 0x01)]
    CannotReadMediumUnknownFormat,
    #[error("CANNOT READ MEDIUM – INCOMPATIBLE FORMAT")]
    #[mmc_error(sk = 0x5, asc = 0x30, ascq = 0x02)]
    CannotReadMediumIncompatibleFormat,
    #[error("CLEANING CARTRIDGE INSTALLED")]
    #[mmc_error(sk = 0x5, asc = 0x30, ascq = 0x03)]
    CleaningCartridgeInstalled,
    #[error("CANNOT WRITE MEDIUM – UNKNOWN FORMAT")]
    #[mmc_error(sk = 0x5, asc = 0x30, ascq = 0x04)]
    CannotWriteMediumUnknownFormat,
    #[error("CANNOT WRITE MEDIUM – INCOMPATIBLE FORMAT")]
    #[mmc_error(sk = 0x5, asc = 0x30, ascq = 0x05)]
    CannotWriteMediumIncompatibleFormat,
    #[error("CANNOT FORMAT MEDIUM – INCOMPATIBLE MEDIUM")]
    #[mmc_error(sk = 0x5, asc = 0x30, ascq = 0x06)]
    CannotFormatMediumIncompatibleMedium,
    #[error("CLEANING FAILURE")]
    #[mmc_error(sk = 0x5, asc = 0x30, ascq = 0x07)]
    CleaningFailure,
    #[error("CANNOT WRITE – APPLICATION CODE MISMATCH")]
    #[mmc_error(sk = 0x5, asc = 0x30, ascq = 0x08)]
    CannotWriteApplicationCodeMismatch,
    #[error("CURRENT SESSION NOT FIXATED FOR APPEND")]
    #[mmc_error(sk = 0x5, asc = 0x30, ascq = 0x09)]
    CurrentSessionNotFixatedForAppend,
    #[error("MEDIUM NOT FORMATTED")]
    #[mmc_error(sk = 0x5, asc = 0x30, ascq = 0x10)]
    MediumNotFormatted,
    #[error("MEDIUM FORMAT CORRUPTED")]
    #[mmc_error(sk = 0x3, asc = 0x31, ascq = 0x00)]
    MediumFormatCorrupted,
    #[error("FORMAT COMMAND FAILED")]
    #[mmc_error(sk = 0x3, asc = 0x31, ascq = 0x01)]
    FormatCommandFailed,
    #[error("ZONED FORMATTING FAILED DUE TO SPARE LINKING")]
    #[mmc_error(sk = 0x3, asc = 0x31, ascq = 0x02)]
    ZonedFormattingFailedDueToSpareLinking,
    #[error("UNABLE TO RECOVER TABLE-OF-CONTENTS")]
    #[mmc_error(sk = 0x3, asc = 0x57, ascq = 0x00)]
    UnableToRecoverTableOfContents,
    #[error("CD CONTROL ERROR")]
    #[mmc_error(sk = 0x3, asc = 0x73, ascq = 0x00)]
    CdControlError,
}

#[derive(Error, MMCError, Debug)]
pub enum ReadingError {
    #[error("UNRECOVERED READ ERROR")]
    #[mmc_error(sk = 0x3, asc = 0x11, ascq = 0x00)]
    UnrecoveredReadError,
    #[error("READ RETRIES EXHAUSTED")]
    #[mmc_error(sk = 0x3, asc = 0x11, ascq = 0x01)]
    ReadRetriesExhausted,
    #[error("ERROR TOO LONG TO CORRECT")]
    #[mmc_error(sk = 0x3, asc = 0x11, ascq = 0x02)]
    ErrorTooLongToCorrect,
    #[error("L-EC UNCORRECTABLE ERROR")]
    #[mmc_error(sk = 0x3, asc = 0x11, ascq = 0x05)]
    LECUncorrectableError,
    #[error("CIRC UNRECOVERED ERROR")]
    #[mmc_error(sk = 0x3, asc = 0x11, ascq = 0x06)]
    CircUnrecoveredError,
    #[error("ERROR READING UPC/EAN NUMBER")]
    #[mmc_error(sk = 0x3, asc = 0x11, ascq = 0x0F)]
    ErrorReadingUpcEanNumber,
    #[error("ERROR READING ISRC NUMBER")]
    #[mmc_error(sk = 0x3, asc = 0x11, ascq = 0x10)]
    ErrorReadingIsrcNumber,
    #[error("READ ERROR – LOSS OF STREAMING")]
    #[mmc_error(sk = 0xB, asc = 0x11, ascq = 0x11)]
    ReadErrorLossOfStreaming,
    #[error("RANDOM POSITIONING ERROR")]
    #[mmc_error(sk = 0x3, asc = 0x15, ascq = 0x00)]
    RandomPositioningError,
    #[error("MECHANICAL POSITIONING ERROR")]
    #[mmc_error(sk = 0x3, asc = 0x15, ascq = 0x01)]
    MechanicalPositioningError,
    #[error("POSITIONING ERROR DETECTED BY READ OF MEDIUM")]
    #[mmc_error(sk = 0x3, asc = 0x15, ascq = 0x02)]
    PositioningErrorDetectedByReadOfMedium,
    #[error("RECOVERED DATA WITH NO ERROR CORRECTION APPLIED")]
    #[mmc_error(sk = 0x1, asc = 0x17, ascq = 0x00)]
    RecoveredDataWithNoErrorCorrectionApplied,
    #[error("RECOVERED DATA WITH RETRIES")]
    #[mmc_error(sk = 0x1, asc = 0x17, ascq = 0x01)]
    RecoveredDataWithRetries,
    #[error("RECOVERED DATA WITH POSITIVE HEAD OFFSET")]
    #[mmc_error(sk = 0x1, asc = 0x17, ascq = 0x02)]
    RecoveredDataWithPositiveHeadOffset,
    #[error("RECOVERED DATA WITH NEGATIVE HEAD OFFSET")]
    #[mmc_error(sk = 0x1, asc = 0x17, ascq = 0x03)]
    RecoveredDataWithNegativeHeadOffset,
    #[error("RECOVERED DATA WITH RETRIES AND/OR CIRC APPLIED")]
    #[mmc_error(sk = 0x1, asc = 0x17, ascq = 0x04)]
    RecoveredDataWithRetriesAndOrcIrcApplied,
    #[error("RECOVERED DATA USING PREVIOUS SECTOR ID")]
    #[mmc_error(sk = 0x1, asc = 0x17, ascq = 0x05)]
    RecoveredDataUsingPreviousSectorId,
    #[error("RECOVERED DATA WITHOUT ECC – RECOMMEND REASSIGNMENT")]
    #[mmc_error(sk = 0x1, asc = 0x17, ascq = 0x07)]
    RecoveredDataWithoutEccRecommendReassignment,
    #[error("RECOVERED DATA WITHOUT ECC – RECOMMEND REWRITE")]
    #[mmc_error(sk = 0x1, asc = 0x17, ascq = 0x08)]
    RecoveredDataWithoutEccRecommendRewrite,
    #[error("RECOVERED DATA WITHOUT ECC – DATA REWRITTEN")]
    #[mmc_error(sk = 0x1, asc = 0x17, ascq = 0x09)]
    RecoveredDataWithoutEccDataRewritten,
    #[error("RECOVERED DATA WITH ERROR CORRECTION APPLIED")]
    #[mmc_error(sk = 0x1, asc = 0x18, ascq = 0x00)]
    RecoveredDataWithErrorCorrectionApplied,
    #[error("RECOVERED DATA WITH ERROR CORR. & RETRIES APPLIED")]
    #[mmc_error(sk = 0x1, asc = 0x18, ascq = 0x01)]
    RecoveredDataWithErrorCorrRetriesApplied,
    #[error("RECOVERED DATA – DATA AUTO-REALLOCATED")]
    #[mmc_error(sk = 0x1, asc = 0x18, ascq = 0x02)]
    RecoveredDataDataAutoReallocated,
    #[error("RECOVERED DATA WITH CIRC")]
    #[mmc_error(sk = 0x1, asc = 0x18, ascq = 0x03)]
    RecoveredDataWithCirc,
    #[error("RECOVERED DATA WITH L-EC")]
    #[mmc_error(sk = 0x1, asc = 0x18, ascq = 0x04)]
    RecoveredDataWithLEC,
    #[error("RECOVERED DATA – RECOMMEND REASSIGNMENT")]
    #[mmc_error(sk = 0x1, asc = 0x18, ascq = 0x05)]
    RecoveredDataRecommendReassignment,
    #[error("RECOVERED DATA – RECOMMEND REWRITE")]
    #[mmc_error(sk = 0x1, asc = 0x18, ascq = 0x06)]
    RecoveredDataRecommendRewrite,
    #[error("RECOVERED DATA WITH LINKING")]
    #[mmc_error(sk = 0x1, asc = 0x18, ascq = 0x08)]
    RecoveredDataWithLinking,
    #[error("INCOMPATIBLE MEDIUM INSTALLED")]
    #[mmc_error(sk = 0x2 | 0x5, asc = 0x30, ascq = 0x00)]
    IncompatibleMediumInstalled,
    #[error("CANNOT READ MEDIUM – UNKNOWN FORMAT")]
    #[mmc_error(sk = 0x2 | 0x5, asc = 0x30, ascq = 0x01)]
    CannotReadMediumUnknownFormat,
    #[error("CANNOT READ MEDIUM – INCOMPATIBLE FORMAT")]
    #[mmc_error(sk = 0x2 | 0x5, asc = 0x30, ascq = 0x02)]
    CannotReadMediumIncompatibleFormat,
    #[error("CLEANING CARTRIDGE INSTALLED")]
    #[mmc_error(sk = 0x2 | 0x5, asc = 0x30, ascq = 0x03)]
    CleaningCartridgeInstalled,
    #[error("BLANK CHECK")]
    #[mmc_error(sk = 0x8, asc = _, ascq = _)]
    BlankCheck,
}

#[derive(Error, MMCError, Debug)]
pub enum WritingError {
    #[error("WRITE ERROR")]
    #[mmc_error(sk = 0x3, asc = 0x0C, ascq = 0x00)]
    WriteError,
    #[error("WRITE ERROR – RECOVERY NEEDED")]
    #[mmc_error(sk = 0x3, asc = 0x0C, ascq = 0x07)]
    WriteErrorRecoveryNeeded,
    #[error("WRITE ERROR – RECOVERY FAILED")]
    #[mmc_error(sk = 0x3, asc = 0x0C, ascq = 0x08)]
    WriteErrorRecoveryFailed,
    #[error("WRITE ERROR – LOSS OF STREAMING")]
    #[mmc_error(sk = 0x3, asc = 0x0C, ascq = 0x09)]
    WriteErrorLossOfStreaming,
    #[error("WRITE ERROR – PADDING BLOCKS ADDED")]
    #[mmc_error(sk = 0x3, asc = 0x0C, ascq = 0x0A)]
    WriteErrorPaddingBlocksAdded,
    #[error("WRITE PROTECTED")]
    #[mmc_error(sk = 0x7, asc = 0x27, ascq = 0x00)]
    WriteProtected,
    #[error("HARDWARE WRITE PROTECTED")]
    #[mmc_error(sk = 0x7, asc = 0x27, ascq = 0x01)]
    HardwareWriteProtected,
    #[error("LOGICAL UNIT SOFTWARE WRITE PROTECTED")]
    #[mmc_error(sk = 0x7, asc = 0x27, ascq = 0x02)]
    LogicalUnitSoftwareWriteProtected,
    #[error("ASSOCIATED WRITE PROTECT")]
    #[mmc_error(sk = 0x7, asc = 0x27, ascq = 0x03)]
    AssociatedWriteProtect,
    #[error("PERSISTENT WRITE PROTECT")]
    #[mmc_error(sk = 0x7, asc = 0x27, ascq = 0x04)]
    PersistentWriteProtect,
    #[error("PERMANENT WRITE PROTECT")]
    #[mmc_error(sk = 0x7, asc = 0x27, ascq = 0x05)]
    PermanentWriteProtect,
    #[error("CONDITIONAL WRITE PROTECT")]
    #[mmc_error(sk = 0x7, asc = 0x27, ascq = 0x06)]
    ConditionalWriteProtect,
    #[error("CANNOT WRITE MEDIUM – UNKNOWN FORMAT")]
    #[mmc_error(sk = 0x2, asc = 0x30, ascq = 0x04)]
    CannotWriteMediumUnknownFormat,
    #[error("CANNOT WRITE MEDIUM – INCOMPATIBLE FORMAT")]
    #[mmc_error(sk = 0x2, asc = 0x30, ascq = 0x05)]
    CannotWriteMediumIncompatibleFormat,
    #[error("CANNOT FORMAT MEDIUM – INCOMPATIBLE MEDIUM")]
    #[mmc_error(sk = 0x2, asc = 0x30, ascq = 0x06)]
    CannotFormatMediumIncompatibleMedium,
    #[error("CLEANING FAILURE")]
    #[mmc_error(sk = 0x2, asc = 0x30, ascq = 0x07)]
    CleaningFailure,
    #[error("CANNOT WRITE – APPLICATION CODE MISMATCH")]
    #[mmc_error(sk = 0x5, asc = 0x30, ascq = 0x08)]
    CannotWriteApplicationCodeMismatch,
    #[error("CURRENT SESSION NOT FIXATED FOR APPEND")]
    #[mmc_error(sk = 0x5, asc = 0x30, ascq = 0x09)]
    CurrentSessionNotFixatedForAppend,
    #[error("MEDIUM NOT FORMATTED")]
    #[mmc_error(sk = 0x5, asc = 0x30, ascq = 0x10)]
    MediumNotFormatted,
    #[error("CANNOT WRITE MEDIUM – UNSUPPORTED MEDIUM VERSION")]
    #[mmc_error(sk = 0x2, asc = 0x30, ascq = 0x11)]
    CannotWriteMediumUnsupportedMediumVersion,
    #[error("MEDIUM FORMAT CORRUPTED")]
    #[mmc_error(sk = 0x3, asc = 0x31, ascq = 0x00)]
    MediumFormatCorrupted,
    #[error("FORMAT COMMAND FAILED")]
    #[mmc_error(sk = 0x3, asc = 0x31, ascq = 0x01)]
    FormatCommandFailed,
    #[error("ZONED FORMATTING FAILED DUE TO SPARE LINKING")]
    #[mmc_error(sk = 0x3, asc = 0x31, ascq = 0x02)]
    ZonedFormattingFailedDueToSpareLinking,
    #[error("NO DEFECT SPARE LOCATION AVAILABLE")]
    #[mmc_error(sk = 0x3, asc = 0x32, ascq = 0x00)]
    NoDefectSpareLocationAvailable,
    #[error("ERASE FAILURE")]
    #[mmc_error(sk = 0x3, asc = 0x51, ascq = 0x00)]
    EraseFailure,
    #[error("ERASE FAILURE – INCOMPLETE ERASE OPERATION DETECTED")]
    #[mmc_error(sk = 0x3, asc = 0x51, ascq = 0x01)]
    EraseFailureIncompleteEraseOperationDetected,
    #[error("FAILURE PREDICTION THRESHOLD EXCEEDED")]
    #[mmc_error(sk = 0x1 | 0x3, asc = 0x5D, ascq = 0x00)]
    FailurePredictionThresholdExceeded,
    #[error("MEDIA FAILURE PREDICTION THRESHOLD EXCEEDED")]
    #[mmc_error(sk = 0x1 | 0x3, asc = 0x5D, ascq = 0x01)]
    MediaFailurePredictionThresholdExceeded,
    #[error("LOGICAL UNIT FAILURE PREDICTION THRESHOLD EXCEEDED")]
    #[mmc_error(sk = 0x1 | 0x3, asc = 0x5D, ascq = 0x02)]
    LogicalUnitFailurePredictionThresholdExceeded,
    #[error("FAILURE PREDICTION THRESHOLD EXCEEDED – Predicted Spare Area Exhaustion")]
    #[mmc_error(sk = 0x1 | 0x3, asc = 0x5D, ascq = 0x03)]
    FailurePredictionThresholdExceededPredictedSpareAreaExhaustion,
    #[error("FAILURE PREDICTION THRESHOLD EXCEEDED (FALSE)")]
    #[mmc_error(sk = 0x1, asc = 0x5D, ascq = 0xFF)]
    FailurePredictionThresholdExceededFalse,
    #[error("SESSION FIXATION ERROR")]
    #[mmc_error(sk = 0x3, asc = 0x72, ascq = 0x00)]
    SessionFixationError,
    #[error("SESSION FIXATION ERROR WRITING LEAD-IN")]
    #[mmc_error(sk = 0x3, asc = 0x72, ascq = 0x01)]
    SessionFixationErrorWritingLeadIn,
    #[error("SESSION FIXATION ERROR WRITING LEAD-OUT")]
    #[mmc_error(sk = 0x3, asc = 0x72, ascq = 0x02)]
    SessionFixationErrorWritingLeadOut,
    #[error("SESSION FIXATION ERROR – INCOMPLETE TRACK IN SESSION")]
    #[mmc_error(sk = 0x5, asc = 0x72, ascq = 0x03)]
    SessionFixationErrorIncompleteTrackInSession,
    #[error("EMPTY OR PARTIALLY WRITTEN RESERVED TRACK")]
    #[mmc_error(sk = 0x5, asc = 0x72, ascq = 0x04)]
    EmptyOrPartiallyWrittenReservedTrack,
    #[error("NO MORE TRACK RESERVATIONS ALLOWED")]
    #[mmc_error(sk = 0x5, asc = 0x72, ascq = 0x05)]
    NoMoreTrackReservationsAllowed,
    #[error("RMZ EXTENSION IS NOT ALLOWED")]
    #[mmc_error(sk = 0x5, asc = 0x72, ascq = 0x06)]
    RmzExtensionIsNotAllowed,
    #[error("NO MORE TEST ZONE EXTENSIONS ARE ALLOWED")]
    #[mmc_error(sk = 0x5, asc = 0x72, ascq = 0x07)]
    NoMoreTestZoneExtensionsAreAllowed,
    #[error("POWER CALIBRATION AREA ALMOST FULL")]
    #[mmc_error(sk = 0x1, asc = 0x73, ascq = 0x01)]
    PowerCalibrationAreaAlmostFull,
    #[error("POWER CALIBRATION AREA IS FULL")]
    #[mmc_error(sk = 0x3, asc = 0x73, ascq = 0x02)]
    PowerCalibrationAreaIsFull,
    #[error("POWER CALIBRATION AREA ERROR")]
    #[mmc_error(sk = 0x3, asc = 0x73, ascq = 0x03)]
    PowerCalibrationAreaError,
    #[error("PROGRAM MEMORY AREA UPDATE FAILURE")]
    #[mmc_error(sk = 0x3, asc = 0x73, ascq = 0x04)]
    ProgramMemoryAreaUpdateFailure,
    #[error("PROGRAM MEMORY AREA IS FULL")]
    #[mmc_error(sk = 0x3, asc = 0x73, ascq = 0x05)]
    ProgramMemoryAreaIsFull,
    #[error("RMA/PMA IS ALMOST FULL")]
    #[mmc_error(sk = 0x1, asc = 0x73, ascq = 0x06)]
    RmaPmaIsAlmostFull,
    #[error("CURRENT POWER CALIBRATION AREA IS ALMOST FULL")]
    #[mmc_error(sk = 0x3, asc = 0x73, ascq = 0x10)]
    CurrentPowerCalibrationAreaIsAlmostFull,
    #[error("CURRENT POWER CALIBRATION AREA IS FULL")]
    #[mmc_error(sk = 0x3, asc = 0x73, ascq = 0x11)]
    CurrentPowerCalibrationAreaIsFull,
    #[error("RDZ IS FULL")]
    #[mmc_error(sk = 0x5, asc = 0x73, ascq = 0x17)]
    RdzIsFull,
    #[error("BLANK CHECK")]
    #[mmc_error(sk = 0x8, asc = _, ascq = _)]
    BlankCheck,
}

#[derive(Error, MMCError, Debug)]
pub enum HardwareFailure {
    #[error("CLEANING REQUESTED")]
    #[mmc_error(sk = 0x4, asc = 0x00, ascq = 0x17)]
    CleaningRequested,
    #[error("LOGICAL UNIT DOES NOT RESPOND TO SELECTION")]
    #[mmc_error(sk = 0x4, asc = 0x05, ascq = 0x00)]
    LogicalUnitDoesNotRespondToSelection,
    #[error("LOGICAL UNIT COMMUNICATION FAILURE")]
    #[mmc_error(sk = 0x4, asc = 0x08, ascq = 0x00)]
    LogicalUnitCommunicationFailure,
    #[error("LOGICAL UNIT COMMUNICATION TIMEOUT")]
    #[mmc_error(sk = 0x4, asc = 0x08, ascq = 0x01)]
    LogicalUnitCommunicationTimeout,
    #[error("LOGICAL UNIT COMMUNICATION PARITY ERROR")]
    #[mmc_error(sk = 0x4, asc = 0x08, ascq = 0x02)]
    LogicalUnitCommunicationParityError,
    #[error("LOGICAL UNIT COMMUNICATION CRC ERROR (ULTRA-DMA/32)")]
    #[mmc_error(sk = 0x4, asc = 0x08, ascq = 0x03)]
    LogicalUnitCommunicationCrcErrorUltraDma32,
    #[error("TRACK FOLLOWING ERROR")]
    #[mmc_error(sk = 0x4, asc = 0x09, ascq = 0x00)]
    TrackFollowingError,
    #[error("TRACKING SERVO FAILURE")]
    #[mmc_error(sk = 0x4, asc = 0x09, ascq = 0x01)]
    TrackingServoFailure,
    #[error("FOCUS SERVO FAILURE")]
    #[mmc_error(sk = 0x4, asc = 0x09, ascq = 0x02)]
    FocusServoFailure,
    #[error("SPINDLE SERVO FAILURE")]
    #[mmc_error(sk = 0x4, asc = 0x09, ascq = 0x03)]
    SpindleServoFailure,
    #[error("HEAD SELECT FAULT")]
    #[mmc_error(sk = 0x4, asc = 0x09, ascq = 0x04)]
    HeadSelectFault,
    #[error("RANDOM POSITIONING ERROR")]
    #[mmc_error(sk = 0x4, asc = 0x15, ascq = 0x00)]
    RandomPositioningError,
    #[error("MECHANICAL POSITIONING ERROR")]
    #[mmc_error(sk = 0x4, asc = 0x15, ascq = 0x01)]
    MechanicalPositioningError,
    #[error("SYNCHRONOUS DATA TRANSFER ERROR")]
    #[mmc_error(sk = 0x4, asc = 0x1B, ascq = 0x00)]
    SynchronousDataTransferError,
    #[error("MECHANICAL POSITIONING OR CHANGER ERROR")]
    #[mmc_error(sk = 0x4, asc = 0x3B, ascq = 0x16)]
    MechanicalPositioningOrChangerError,
    #[error("LOGICAL UNIT FAILURE")]
    #[mmc_error(sk = 0x4, asc = 0x3E, ascq = 0x01)]
    LogicalUnitFailure,
    #[error("TIMEOUT ON LOGICAL UNIT")]
    #[mmc_error(sk = 0x4, asc = 0x3E, ascq = 0x02)]
    TimeoutOnLogicalUnit,
    #[error("DIAGNOSTIC FAILURE ON COMPONENT NN (80H-FFH)")]
    #[mmc_error(sk = 0x4, asc = 0x40, ascq = 0x80..=0xFF)]
    DiagnosticFailureOnComponentNN,
    #[error("INTERNAL TARGET FAILURE")]
    #[mmc_error(sk = 0x4, asc = 0x44, ascq = 0x00)]
    InternalTargetFailure,
    #[error("UNSUCCESSFUL SOFT RESET")]
    #[mmc_error(sk = 0x4, asc = 0x46, ascq = 0x00)]
    UnsuccessfulSoftReset,
    #[error("SCSI PARITY ERROR")]
    #[mmc_error(sk = 0x4, asc = 0x47, ascq = 0x00)]
    ScsiParityError,
    #[error("COMMAND PHASE ERROR")]
    #[mmc_error(sk = 0x4, asc = 0x4A, ascq = 0x00)]
    CommandPhaseError,
    #[error("DATA PHASE ERROR")]
    #[mmc_error(sk = 0x4, asc = 0x4B, ascq = 0x00)]
    DataPhaseError,
    #[error("LOGICAL UNIT FAILED SELF-CONFIGURATION")]
    #[mmc_error(sk = 0x4, asc = 0x4C, ascq = 0x00)]
    LogicalUnitFailedSelfConfiguration,
    #[error("MEDIA LOAD OR EJECT FAILED")]
    #[mmc_error(sk = 0x4, asc = 0x53, ascq = 0x00)]
    MediaLoadOrEjectFailed,
    #[error("FAILURE PREDICTION THRESHOLD EXCEEDED – Predicted Media failure")]
    #[mmc_error(sk = 0x1, asc = 0x5D, ascq = 0x01)]
    FailurePredictionThresholdExceededPredictedMediaFailure,
    #[error("LOGICAL UNIT FAILURE PREDICTION THRESHOLD EXCEEDED")]
    #[mmc_error(sk = 0x1, asc = 0x5D, ascq = 0x02)]
    LogicalUnitFailurePredictionThresholdExceeded,
    #[error("FAILURE PREDICTION THRESHOLD EXCEEDED – Predicted Spare Area Exhaustion")]
    #[mmc_error(sk = 0x1, asc = 0x5D, ascq = 0x03)]
    FailurePredictionThresholdExceededPredictedSpareAreaExhaustion,
    #[error("FAILURE PREDICTION THRESHOLD EXCEEDED (FALSE)")]
    #[mmc_error(sk = 0x1, asc = 0x5D, ascq = 0xFF)]
    FailurePredictionThresholdExceededFalse,
    #[error("VOLTAGE FAULT")]
    #[mmc_error(sk = 0x4, asc = 0x65, ascq = 0x00)]
    VoltageFault,
}

#[derive(Error, MMCError, Debug)]
pub enum NonATAPIEnvironmentError {
    #[error("I/O PROCESS TERMINATED")]
    #[mmc_error(sk = 0xB, asc = 0x00, ascq = 0x06)]
    IOProcessTerminated,
    #[error("MULTIPLE PERIPHERAL DEVICES SELECTED")]
    #[mmc_error(sk = 0x5, asc = 0x07, ascq = 0x00)]
    MultiplePeripheralDevicesSelected,
    #[error("LOGICAL UNIT COMMUNICATION CRC ERROR (ULTRA-DMA/32)")]
    #[mmc_error(sk = 0x4, asc = 0x08, ascq = 0x03)]
    LogicalUnitCommunicationCrcErrorUltraDma32,
    #[error("HEAD SELECT FAULT")]
    #[mmc_error(sk = 0x4, asc = 0x09, ascq = 0x04)]
    HeadSelectFault,
    #[error("WARNING")]
    #[mmc_error(sk = 0x1, asc = 0x0B, ascq = 0x00)]
    Warning,
    #[error("WARNING – SPECIFIED TEMPERATURE EXCEEDED")]
    #[mmc_error(sk = 0x1, asc = 0x0B, ascq = 0x01)]
    WarningSpecifiedTemperatureExceeded,
    #[error("WARNING – ENCLOSURE DEGRADED")]
    #[mmc_error(sk = 0x1, asc = 0x0B, ascq = 0x02)]
    WarningEnclosureDegraded,
    #[error("SYNCHRONOUS DATA TRANSFER ERROR")]
    #[mmc_error(sk = 0x4, asc = 0x1B, ascq = 0x00)]
    SynchronousDataTransferError,
    #[error("LOGICAL UNIT NOT SUPPORTED")]
    #[mmc_error(sk = 0x5, asc = 0x25, ascq = 0x00)]
    LogicalUnitNotSupported,
    #[error("RESERVATIONS PREEMPTED")]
    #[mmc_error(sk = 0x6, asc = 0x2A, ascq = 0x03)]
    ReservationsPreempted,
    #[error("COPY CANNOT EXECUTE SINCE INITIATOR CANNOT DISCONNECT")]
    #[mmc_error(sk = 0x5, asc = 0x2B, ascq = 0x00)]
    CopyCannotExecuteSinceInitiatorCannotDisconnect,
    #[error("COMMANDS CLEARED BY ANOTHER INITIATOR")]
    #[mmc_error(sk = 0x6, asc = 0x2F, ascq = 0x00)]
    CommandsClearedByAnotherInitiator,
    #[error("ENCLOSURE FAILURE")]
    #[mmc_error(sk = _, asc = 0x34, ascq = 0x00)]
    EnclosureFailure,
    #[error("ENCLOSURE SERVICES FAILURE")]
    #[mmc_error(sk = _, asc = 0x35, ascq = 0x00)]
    EnclosureServicesFailure,
    #[error("UNSUPPORTED ENCLOSURE FUNCTION")]
    #[mmc_error(sk = _, asc = 0x35, ascq = 0x01)]
    UnsupportedEnclosureFunction,
    #[error("ENCLOSURE SERVICES UNAVAILABLE")]
    #[mmc_error(sk = _, asc = 0x35, ascq = 0x02)]
    EnclosureServicesUnavailable,
    #[error("ENCLOSURE SERVICES TRANSFER FAILURE")]
    #[mmc_error(sk = _, asc = 0x35, ascq = 0x03)]
    EnclosureServicesTransferFailure,
    #[error("ENCLOSURE SERVICES TRANSFER REFUSED")]
    #[mmc_error(sk = _, asc = 0x35, ascq = 0x04)]
    EnclosureServicesTransferRefused,
    #[error("INVALID BITS IN IDENTIFY MESSAGE")]
    #[mmc_error(sk = 0x5, asc = 0x3D, ascq = 0x00)]
    InvalidBitsInIdentifyMessage,
    #[error("MESSAGE ERROR")]
    #[mmc_error(sk = 0x5, asc = 0x43, ascq = 0x00)]
    MessageError,
    #[error("SELECT OR RESELECT FAILURE")]
    #[mmc_error(sk = 0xB, asc = 0x45, ascq = 0x00)]
    SelectOrReselectFailure,
    #[error("SCSI PARITY ERROR")]
    #[mmc_error(sk = 0x4, asc = 0x47, ascq = 0x00)]
    ScsiParityError,
    #[error("INITIATOR DETECTED ERROR MESSAGE RECEIVED")]
    #[mmc_error(sk = 0xB, asc = 0x48, ascq = 0x00)]
    InitiatorDetectedErrorMessageReceived,
    #[error("INVALID MESSAGE ERROR")]
    #[mmc_error(sk = 0xB, asc = 0x49, ascq = 0x00)]
    InvalidMessageError,
    #[error("COMMAND PHASE ERROR")]
    #[mmc_error(sk = 0x4, asc = 0x4A, ascq = 0x00)]
    CommandPhaseError,
    #[error("DATA PHASE ERROR")]
    #[mmc_error(sk = 0x4, asc = 0x4B, ascq = 0x00)]
    DataPhaseError,
    #[error("TAGGED OVERLAPPED COMMANDS (NN = QUEUE TAG)")]
    #[mmc_error(sk = 0xB, asc = 0x4D, ascq = _)]
    TaggedOverlappedCommandsNN,
}
