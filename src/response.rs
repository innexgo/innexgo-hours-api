use super::request::{AdminshipKind, CourseMembershipKind, EncounterKind, SubscriptionKind};
use either::Either;
use serde::{Deserialize, Serialize};
use strum::AsRefStr;

#[derive(Clone, Debug, Serialize, Deserialize, AsRefStr)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum InnexgoHoursError {
  Ok,
  NotFound,

  DecodeError,
  MethodNotAllowed,

  NoCapability,
  ApiKeyUnauthorized,
  PasswordIncorrect,
  PasswordInsecure,
  PasswordCannotCreateForOthers,
  UserNonexistent,
  ApiKeyNonexistent,
  UserExistent,
  UserNameEmpty,
  UserEmailEmpty,
  UserEmailInvalidated,
  UserKindInvalid,

  SubscriptionNonexistent,
  SubscriptionExpired,
  SubscriptionUnauthorized,
  SubscriptionLimited,

  SchoolNonexistent,
  SchoolArchived,

  SchoolKeyNonexistent,
  SchoolKeyExpired,
  SchoolKeyUsed,

  SchoolDurationNonexistent,
  SchoolDurationDayInvalid,
  SchoolDurationMinuteInvalid,
  SchoolDurationDataNonexistent,

  AdminshipCannotLeaveEmpty,

  SessionRequestNonexistent,
  SessionRequestResponseExistent,
  SessionRequestResponseCannotCancelStudent,

  SessionNotRelevant,
  SessionNonexistent,

  CommittmentExistent,
  CommittmentNonexistent,
  CommittmentCannotCreateForOthersStudent,
  CommittmentCannotCreateHiddenStudent,
  CommittmentCannotCreateUncancellableStudent,

  CourseNonexistent,
  CourseArchived,

  CourseKeyNonexistent,
  CourseKeyExpired,
  CourseKeyUsed,

  CourseMembershipNonexistent,
  CourseMembershipCannotLeaveEmpty,

  LocationNonexistent,

  NegativeDuration,
  CannotAlterPast,

  VerificationChallengeNonexistent,
  VerificationChallengeTimedOut,
  PasswordResetNonexistent,
  PasswordExistent,
  PasswordResetTimedOut,
  EmailRatelimit,
  EmailBlacklisted,
  Unknown,
  InternalServerError,
  AuthInternalServerError,
  AuthBadRequest,
  AuthNetworkError,
  AuthOther,
  Network,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct School {
  pub school_id: i64,
  pub creator_user_id: i64,
  pub creation_time: i64,
  pub whole: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SchoolData {
  pub school_data_id: i64,
  pub creator_user_id: i64,
  pub creation_time: i64,
  pub school: School,
  pub name: String,
  pub description: String,
  pub active: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Subscription {
  pub subscription_id: i64,
  pub creation_time: i64,
  pub creator_user_id: i64,
  pub subscription_kind: SubscriptionKind,
  pub max_uses: i64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SchoolDuration {
  pub school_duration_id: i64,
  pub creation_time: i64,
  pub creator_user_id: i64,
  pub school: School,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SchoolDurationData {
  pub school_duration_data_id: i64,
  pub creation_time: i64,
  pub creator_user_id: i64,
  pub school_duration: SchoolDuration,
  pub day: i64,
  pub minute_start: i64,
  pub minute_end: i64,
  pub active: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SchoolKey {
  pub school_key_key: String,
  pub creation_time: i64,
  pub creator_user_id: i64,
  pub school: School,
  pub start_time: i64,
  pub end_time: i64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SchoolKeyData {
  pub school_key_data_id: i64,
  pub creation_time: i64,
  pub creator_user_id: i64,
  pub school_key: SchoolKey,
  pub active: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Adminship {
  pub adminship_id: i64,
  pub creation_time: i64,
  pub creator_user_id: i64,
  pub user_id: i64,
  pub school: School,
  pub adminship_kind: AdminshipKind,
  pub school_key: Option<SchoolKey>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Location {
  pub location_id: i64,
  pub creation_time: i64,
  pub creator_user_id: i64,
  pub school: School,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LocationData {
  pub location_data_id: i64,
  pub creation_time: i64,
  pub creator_user_id: i64,
  pub location: Location,
  pub name: String,
  pub address: String,
  pub phone: String,
  pub active: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Course {
  pub course_id: i64,
  pub creator_user_id: i64,
  pub creation_time: i64,
  pub school: School,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CourseData {
  pub course_data_id: i64,
  pub creation_time: i64,
  pub creator_user_id: i64,
  pub course: Course,
  pub name: String,
  pub description: String,
  pub homeroom: bool,
  pub active: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CourseKey {
  pub course_key_key: String,
  pub creation_time: i64,
  pub creator_user_id: i64,
  pub course: Course,
  pub max_uses: i64,
  pub course_membership_kind: CourseMembershipKind,
  pub start_time: i64,
  pub end_time: i64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CourseKeyData {
  pub course_key_data_id: i64,
  pub creation_time: i64,
  pub creator_user_id: i64,
  pub course_key: CourseKey,
  pub active: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CourseMembership {
  pub course_membership_id: i64,
  pub creation_time: i64,
  pub creator_user_id: i64,
  pub user_id: i64,
  pub course: Course,
  pub course_membership_kind: CourseMembershipKind,
  pub course_key: Option<CourseKey>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Session {
  pub session_id: i64,
  pub creation_time: i64,
  pub creator_user_id: i64,
  pub course: Course,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SessionData {
  pub session_data_id: i64,
  pub creation_time: i64,
  pub creator_user_id: i64,
  pub session: Session,
  pub name: String,
  pub start_time: i64,
  pub end_time: i64,
  pub active: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SessionRequest {
  pub session_request_id: i64,
  pub creation_time: i64,
  pub creator_user_id: i64,
  pub course: Course,
  pub message: String,
  pub start_time: i64,
  pub end_time: i64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SessionRequestResponse {
  pub session_request: SessionRequest,
  pub creation_time: i64,
  pub creator_user_id: i64,
  pub message: String,
  pub commitment: Option<Committment>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Committment {
  pub commitment_id: i64,
  pub creation_time: i64,
  pub creator_user_id: i64,
  pub attendee_user_id: i64,
  pub session: Session,
  pub active: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Encounter {
  pub encounter_id: i64,
  pub creation_time: i64,
  pub creator_user_id: i64,
  pub location: Location,
  pub user_id: i64,
  pub encounter_kind: EncounterKind,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Stay {
  pub stay_id: i64,
  pub creation_time: i64,
  pub creator_user_id: i64,
  pub attendee_user_id: i64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StayData {
  pub stay_data_id: i64,
  pub creation_time: i64,
  pub creator_user_id: i64,
  pub stay: Stay,
  #[serde(with = "either::serde_untagged")]
  pub fst: Either<Encounter, i64>,
  #[serde(with = "either::serde_untagged")]
  pub snd: Either<Encounter, i64>,
  pub active: bool,
}
