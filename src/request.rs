// Types of arguments for auth handlers
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use strum::AsRefStr;

#[derive(Clone, Debug, Serialize, Deserialize, AsRefStr)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SubscriptionKind {
  Valid,
  Cancel,
}

impl TryFrom<u8> for SubscriptionKind {
  type Error = u8;
  fn try_from(val: u8) -> Result<SubscriptionKind, u8> {
    match val {
      x if x == SubscriptionKind::Valid as u8 => Ok(SubscriptionKind::Valid),
      x if x == SubscriptionKind::Cancel as u8 => Ok(SubscriptionKind::Cancel),
      x => Err(x),
    }
  }
}

#[derive(Clone, Debug, Serialize, Deserialize, AsRefStr)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AdminshipKind {
  Admin,
  Cancel,
}

impl TryFrom<u8> for AdminshipKind {
  type Error = u8;
  fn try_from(val: u8) -> Result<AdminshipKind, u8> {
    match val {
      x if x == AdminshipKind::Admin as u8 => Ok(AdminshipKind::Admin),
      x if x == AdminshipKind::Cancel as u8 => Ok(AdminshipKind::Cancel),
      x => Err(x),
    }
  }
}

#[derive(Clone, Debug, Serialize, Deserialize, AsRefStr)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CourseMembershipKind {
  Student,
  Instructor,
  Cancel,
}

impl TryFrom<u8> for CourseMembershipKind {
  type Error = u8;
  fn try_from(val: u8) -> Result<CourseMembershipKind, u8> {
    match val {
      x if x == CourseMembershipKind::Student as u8 => Ok(CourseMembershipKind::Student),
      x if x == CourseMembershipKind::Instructor as u8 => Ok(CourseMembershipKind::Instructor),
      x if x == CourseMembershipKind::Cancel as u8 => Ok(CourseMembershipKind::Cancel),
      x => Err(x),
    }
  }
}

#[derive(Clone, Debug, Serialize, Deserialize, AsRefStr)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum EncounterKind {
  Manual,
  Hardware,
}

impl TryFrom<u8> for EncounterKind {
  type Error = u8;
  fn try_from(val: u8) -> Result<EncounterKind, u8> {
    match val {
      x if x == EncounterKind::Manual as u8 => Ok(EncounterKind::Manual),
      x if x == EncounterKind::Hardware as u8 => Ok(EncounterKind::Hardware),
      x => Err(x),
    }
  }
}

#[derive(Clone, Debug, Serialize, Deserialize, AsRefStr)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum IrregularityKind {
  Absent,
  Tardy,
  LeaveNoreturn,
  LeaveReturn
}

impl TryFrom<u8> for IrregularityKind {
  type Error = u8;
  fn try_from(val: u8) -> Result<IrregularityKind, u8> {
    match val {
      x if x == IrregularityKind::Absent as u8 => Ok(IrregularityKind::Absent),
      x if x == IrregularityKind::Tardy as u8 => Ok(IrregularityKind::Tardy),
      x if x == IrregularityKind::LeaveNoreturn as u8 => Ok(IrregularityKind::LeaveNoreturn),
      x if x == IrregularityKind::LeaveReturn as u8 => Ok(IrregularityKind::LeaveReturn),
      x => Err(x),
    }
  }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubscriptionNewProps {
  pub subscription_kind: SubscriptionKind,
  pub api_key: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SchoolNewProps {
  pub name: String,
  pub description: String,
  pub whole: bool,
  pub api_key: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SchoolDataNewProps {
  pub school_id: i64,
  pub name: String,
  pub description: String,
  pub active: bool,
  pub api_key: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SchoolDurationNewProps {
  pub school_id: i64,
  pub day: i64,
  pub minute_start: i64,
  pub minute_end: i64,
  pub api_key: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SchoolDurationDataNewProps {
  pub school_duration_id: i64,
  pub day: i64,
  pub minute_start: i64,
  pub minute_end: i64,
  pub active: bool,
  pub api_key: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SchoolKeyNewProps {
  pub school_id: i64,
  pub start_time: i64,
  pub end_time: i64,
  pub api_key: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SchoolKeyDataNewProps {
  pub school_key_key: String,
  pub active: bool,
  pub api_key: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdminshipNewCancelProps {
  pub user_id: i64,
  pub school_id: i64,
  pub api_key: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdminshipNewKeyProps {
  pub school_key_key: String,
  pub api_key: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LocationNewProps {
  pub school_id: i64,
  pub name: String,
  pub address: String,
  pub phone: String,
  pub api_key: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LocationDataNewProps {
  pub location_id: i64,
  pub name: String,
  pub address: String,
  pub phone: String,
  pub active: bool,
  pub api_key: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CourseNewProps {
  pub school_id: i64,
  pub location_id: i64,
  pub name: String,
  pub description: String,
  pub homeroom: bool,
  pub api_key: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CourseDataNewProps {
  pub course_id: i64,
  pub location_id: i64,
  pub name: String,
  pub description: String,
  pub homeroom: bool,
  pub active: bool,
  pub api_key: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CourseKeyNewProps {
  pub course_id: i64,
  pub course_membership_kind: CourseMembershipKind,
  pub max_uses: i64,
  pub start_time: i64,
  pub end_time: i64,
  pub api_key: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CourseKeyDataNewProps {
  pub course_key_key: String,
  pub active: bool,
  pub api_key: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CourseMembershipNewCancelProps {
  pub user_id: i64,
  pub course_id: i64,
  pub api_key: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CourseMembershipNewKeyProps {
  pub course_key_key: String,
  pub api_key: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SessionNewProps {
  pub name: String,
  pub course_id: i64,
  pub start_time: i64,
  pub end_time: i64,
  // produces a commitment with all of these
  // more can always be added later
  pub attendee_user_ids: Vec<i64>,
  pub api_key: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SessionDataNewProps {
  pub session_id: i64,
  pub name: String,
  pub active: bool,
  pub start_time: i64,
  pub end_time: i64,
  pub api_key: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SessionRequestNewProps {
  pub course_id: i64,
  pub message: String,
  pub start_time: i64,
  pub end_time: i64,
  pub api_key: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SessionRequestResponseNewProps {
  pub session_request_id: i64,
  pub message: String,
  pub session_id: Option<i64>,
  pub api_key: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommitmentNewProps {
  pub attendee_user_id: i64,
  pub session_id: i64,
  pub api_key: String,
  pub active: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EncounterNewProps {
  pub attendee_user_id: i64,
  pub location_id: i64,
  pub api_key: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EncounterNewAttendsProps {
  pub attendee_user_id: i64,
  pub location_id: i64,
  pub api_key: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StayNewProps {
  pub attendee_user_id: i64,
  pub location_id: i64,
  pub fst_time: Option<i64>,
  pub fst_encounter_id: Option<i64>,
  pub snd_time: Option<i64>,
  pub snd_encounter_id: Option<i64>,
  pub api_key: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StayDataNewProps {
  pub stay_id: i64,
  pub fst_time: Option<i64>,
  pub fst_encounter_id: Option<i64>,
  pub snd_time: Option<i64>,
  pub snd_encounter_id: Option<i64>,
  pub active: bool,
  pub api_key: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubscriptionViewProps {
  pub subscription_id: Option<Vec<i64>>,
  pub min_creation_time: Option<i64>,
  pub max_creation_time: Option<i64>,
  pub creator_user_id: Option<Vec<i64>>,
  pub subscription_kind: Option<Vec<SubscriptionKind>>,
  pub only_recent: bool,
  pub api_key: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SchoolViewProps {
  pub school_id: Option<Vec<i64>>,
  pub min_creation_time: Option<i64>,
  pub max_creation_time: Option<i64>,
  pub creator_user_id: Option<Vec<i64>>,
  pub whole: Option<bool>,
  pub api_key: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SchoolDataViewProps {
  pub school_data_id: Option<Vec<i64>>,
  pub min_creation_time: Option<i64>,
  pub max_creation_time: Option<i64>,
  pub creator_user_id: Option<Vec<i64>>,
  pub school_id: Option<Vec<i64>>,
  pub name: Option<Vec<String>>,
  pub partial_name: Option<String>,
  pub description: Option<Vec<String>>,
  pub partial_description: Option<String>,
  pub active: Option<bool>,
  pub only_recent: bool,
  pub api_key: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SchoolDurationViewProps {
  pub school_duration_id: Option<Vec<i64>>,
  pub min_creation_time: Option<i64>,
  pub max_creation_time: Option<i64>,
  pub creator_user_id: Option<Vec<i64>>,
  pub school_id: Option<Vec<i64>>,
  pub api_key: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SchoolDurationDataViewProps {
  pub school_duration_data_id: Option<Vec<i64>>,
  pub min_creation_time: Option<i64>,
  pub max_creation_time: Option<i64>,
  pub creator_user_id: Option<Vec<i64>>,
  pub school_duration_id: Option<Vec<i64>>,
  pub day: Option<Vec<i64>>,
  pub min_minute_start: Option<i64>,
  pub max_minute_start: Option<i64>,
  pub min_minute_end: Option<i64>,
  pub max_minute_end: Option<i64>,
  pub active: Option<bool>,
  pub only_recent: bool,
  pub school_id: Option<Vec<i64>>,
  pub api_key: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SchoolKeyViewProps {
  pub school_key_key: Option<Vec<String>>,
  pub min_creation_time: Option<i64>,
  pub max_creation_time: Option<i64>,
  pub creator_user_id: Option<Vec<i64>>,
  pub school_id: Option<Vec<i64>>,
  pub min_start_time: Option<i64>,
  pub max_start_time: Option<i64>,
  pub min_end_time: Option<i64>,
  pub max_end_time: Option<i64>,
  pub api_key: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SchoolKeyDataViewProps {
  pub school_key_data_id: Option<Vec<i64>>,
  pub min_creation_time: Option<i64>,
  pub max_creation_time: Option<i64>,
  pub creator_user_id: Option<Vec<i64>>,
  pub school_key_key: Option<Vec<String>>,
  pub active: Option<bool>,
  pub school_id: Option<Vec<i64>>,
  pub min_start_time: Option<i64>,
  pub max_start_time: Option<i64>,
  pub min_end_time: Option<i64>,
  pub max_end_time: Option<i64>,
  pub only_recent: bool,
  pub api_key: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdminshipViewProps {
  pub adminship_id: Option<Vec<i64>>,
  pub min_creation_time: Option<i64>,
  pub max_creation_time: Option<i64>,
  pub creator_user_id: Option<Vec<i64>>,
  pub user_id: Option<Vec<i64>>,
  pub school_id: Option<Vec<i64>>,
  pub adminship_kind: Option<Vec<AdminshipKind>>,
  pub adminship_has_source: Option<bool>,
  pub school_key_key: Option<Vec<String>>,
  pub only_recent: bool,
  pub api_key: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LocationViewProps {
  pub location_id: Option<Vec<i64>>,
  pub min_creation_time: Option<i64>,
  pub max_creation_time: Option<i64>,
  pub creator_user_id: Option<Vec<i64>>,
  pub school_id: Option<Vec<i64>>,
  pub api_key: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LocationDataViewProps {
  pub location_data_id: Option<Vec<i64>>,
  pub min_creation_time: Option<i64>,
  pub max_creation_time: Option<i64>,
  pub creator_user_id: Option<Vec<i64>>,
  pub location_id: Option<Vec<i64>>,
  pub name: Option<Vec<String>>,
  pub partial_name: Option<String>,
  pub address: Option<Vec<String>>,
  pub partial_address: Option<String>,
  pub phone: Option<Vec<String>>,
  pub active: Option<bool>,
  pub only_recent: bool,
  pub school_id: Option<Vec<i64>>,
  pub api_key: String,
}


#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CourseViewProps {
  pub course_id: Option<Vec<i64>>,
  pub min_creation_time: Option<i64>,
  pub max_creation_time: Option<i64>,
  pub creator_user_id: Option<Vec<i64>>,
  pub school_id: Option<Vec<i64>>,
  pub api_key: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CourseDataViewProps {
  pub course_data_id: Option<Vec<i64>>,
  pub min_creation_time: Option<i64>,
  pub max_creation_time: Option<i64>,
  pub creator_user_id: Option<Vec<i64>>,
  pub course_id: Option<Vec<i64>>,
  pub location_id: Option<Vec<i64>>,
  pub name: Option<Vec<String>>,
  pub partial_name: Option<String>,
  pub description: Option<Vec<String>>,
  pub partial_description: Option<String>,
  pub homeroom: Option<bool>,
  pub active: Option<bool>,
  pub only_recent: bool,
  pub school_id: Option<Vec<i64>>,
  pub api_key: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CourseKeyViewProps {
  pub course_key_key: Option<Vec<String>>,
  pub min_creation_time: Option<i64>,
  pub max_creation_time: Option<i64>,
  pub creator_user_id: Option<Vec<i64>>,
  pub course_id: Option<Vec<i64>>,
  pub max_uses: Option<Vec<i64>>,
  pub course_membership_kind: Option<Vec<CourseMembershipKind>>,
  pub min_start_time: Option<i64>,
  pub max_start_time: Option<i64>,
  pub min_end_time: Option<i64>,
  pub max_end_time: Option<i64>,
  pub api_key: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CourseKeyDataViewProps {
  pub course_key_data_id: Option<Vec<i64>>,
  pub min_creation_time: Option<i64>,
  pub max_creation_time: Option<i64>,
  pub creator_user_id: Option<Vec<i64>>,
  pub course_key_key: Option<Vec<String>>,
  pub active: Option<bool>,
  pub course_id: Option<Vec<i64>>,
  pub max_uses: Option<Vec<i64>>,
  pub course_membership_kind: Option<Vec<CourseMembershipKind>>,
  pub min_start_time: Option<i64>,
  pub max_start_time: Option<i64>,
  pub min_end_time: Option<i64>,
  pub max_end_time: Option<i64>,
  pub only_recent: bool,
  pub api_key: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CourseMembershipViewProps {
  pub course_membership_id: Option<Vec<i64>>,
  pub min_creation_time: Option<i64>,
  pub max_creation_time: Option<i64>,
  pub creator_user_id: Option<Vec<i64>>,
  pub user_id: Option<Vec<i64>>,
  pub course_id: Option<Vec<i64>>,
  pub course_membership_kind: Option<Vec<CourseMembershipKind>>,
  pub course_membership_from_key: Option<bool>,
  pub course_key_key: Option<Vec<String>>,
  pub only_recent: bool,
  pub api_key: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SessionViewProps {
  pub session_id: Option<Vec<i64>>,
  pub min_creation_time: Option<i64>,
  pub max_creation_time: Option<i64>,
  pub creator_user_id: Option<Vec<i64>>,
  pub course_id: Option<Vec<i64>>,
  pub api_key: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SessionDataViewProps {
  pub session_data_id: Option<Vec<i64>>,
  pub min_creation_time: Option<i64>,
  pub max_creation_time: Option<i64>,
  pub creator_user_id: Option<Vec<i64>>,
  pub session_id: Option<Vec<i64>>,
  pub name: Option<Vec<String>>,
  pub partial_name: Option<String>,
  pub min_start_time: Option<i64>,
  pub max_start_time: Option<i64>,
  pub min_end_time: Option<i64>,
  pub max_end_time: Option<i64>,
  pub active: Option<bool>,
  pub course_id: Option<Vec<i64>>,
  pub only_recent: bool,
  pub api_key: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SessionRequestViewProps {
  pub session_request_id: Option<Vec<i64>>,
  pub min_creation_time: Option<i64>,
  pub max_creation_time: Option<i64>,
  pub creator_user_id: Option<Vec<i64>>,
  pub course_id: Option<Vec<i64>>,
  pub message: Option<Vec<String>>,
  pub partial_message: Option<String>,
  pub min_start_time: Option<i64>,
  pub max_start_time: Option<i64>,
  pub min_end_time: Option<i64>,
  pub max_end_time: Option<i64>,
  pub responded: Option<bool>,
  pub api_key: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SessionRequestResponseViewProps {
  pub session_request_id: Option<Vec<i64>>,
  pub min_creation_time: Option<i64>,
  pub max_creation_time: Option<i64>,
  pub creator_user_id: Option<Vec<i64>>,
  pub message: Option<Vec<String>>,
  pub partial_message: Option<String>,
  pub accepted: Option<bool>,
  pub commitment_id: Option<Vec<i64>>,
  pub attendee_user_id: Option<Vec<i64>>,
  pub course_id: Option<Vec<i64>>,
  pub min_start_time: Option<i64>,
  pub max_start_time: Option<i64>,
  pub min_end_time: Option<i64>,
  pub max_end_time: Option<i64>,
  pub session_id: Option<Vec<i64>>,
  pub api_key: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommitmentViewProps {
  pub commitment_id: Option<Vec<i64>>,
  pub min_creation_time: Option<i64>,
  pub max_creation_time: Option<i64>,
  pub creator_user_id: Option<Vec<i64>>,
  pub attendee_user_id: Option<Vec<i64>>,
  pub session_id: Option<Vec<i64>>,
  pub course_id: Option<Vec<i64>>,
  pub min_start_time: Option<i64>,
  pub max_start_time: Option<i64>,
  pub min_end_time: Option<i64>,
  pub max_end_time: Option<i64>,
  pub active: Option<bool>,
  pub from_request_response: Option<bool>,
  pub only_recent: bool,
  pub api_key: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EncounterViewProps {
  pub encounter_id: Option<Vec<i64>>,
  pub min_creation_time: Option<i64>,
  pub max_creation_time: Option<i64>,
  pub creator_user_id: Option<Vec<i64>>,
  pub attendee_user_id: Option<Vec<i64>>,
  pub location_id: Option<Vec<i64>>,
  pub api_key: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StayViewProps {
  pub stay_id: Option<Vec<i64>>,
  pub min_creation_time: Option<i64>,
  pub max_creation_time: Option<i64>,
  pub creator_user_id: Option<Vec<i64>>,
  pub attendee_user_id: Option<Vec<i64>>,
  pub location_id: Option<Vec<i64>>,
  pub api_key: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StayDataViewProps {
  pub stay_data_id: Option<Vec<i64>>,
  pub min_creation_time: Option<i64>,
  pub max_creation_time: Option<i64>,
  pub creator_user_id: Option<Vec<i64>>,
  pub stay_id: Option<Vec<i64>>,
  pub fst_encounter_id: Option<Vec<i64>>,
  pub snd_encounter_id: Option<Vec<i64>>,
  pub min_start_time: Option<i64>,
  pub max_start_time: Option<i64>,
  pub min_end_time: Option<i64>,
  pub max_end_time: Option<i64>,
  pub active: bool,
  pub only_recent: bool,
  pub attendee_user_id: Option<Vec<i64>>,
  pub location_id: Option<Vec<i64>>,
  pub api_key: String,
}

