use super::request::{
  AdminshipKind, CommittmentResponseKind, CourseMembershipKind, SubscriptionKind,
};
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

  AdminshipCannotLeaveEmpty,

  SessionRequestNonexistent,
  SessionRequestResponseExistent,
  SessionRequestResponseCannotCancelStudent,

  SessionCannotCreateForOthersStudent,

  SessionNonexistent,

  CommittmentExistent,
  CommittmentNonexistent,
  CommittmentCannotCreateForOthersStudent,
  CommittmentCannotCreateHiddenStudent,
  CommittmentCannotCreateUncancellableStudent,

  CommittmentResponseKindInvalid,
  CommittmentResponseExistent,
  CommittmentResponseUncancellable,
  CommittmentResponseCannotCreateForOthersStudent,

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
pub struct SchoolKey {
  pub school_key_key: String,
  pub creation_time: i64,
  pub creator_user_id: i64,
  pub school: School,
  pub max_uses: i64,
  pub start_time: i64,
  pub end_time: i64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SchoolKeyData  {
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
  pub name: String,
  pub description: String,
  pub valid: bool,
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
pub struct CourseKeyData  {
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
  pub attendee_user_id: i64,
  pub course: Course,
  pub message: String,
  pub start_time: i64,
  pub duration: i64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SessionRequestResponse {
  pub session_request: SessionRequest,
  pub creation_time: i64,
  pub creator_user_id: i64,
  pub message: String,
  pub committment: Option<Committment>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Committment {
  pub committment_id: i64,
  pub creation_time: i64,
  pub creator_user_id: i64,
  pub attendee_user_id: i64,
  pub cancellable: bool,
  pub session: Session,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommittmentResponse {
  pub committment: Committment,
  pub creation_time: i64,
  pub creator_user_id: i64,
  pub kind: CommittmentResponseKind,
}
