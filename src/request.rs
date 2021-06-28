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
pub enum CommittmentResponseKind {
  Present,
  Tardy,
  Absent,
  Cancelled,
}

impl TryFrom<u8> for CommittmentResponseKind {
  type Error = u8;
  fn try_from(val: u8) -> Result<CommittmentResponseKind, u8> {
    match val {
      x if x == CommittmentResponseKind::Present as u8 => Ok(CommittmentResponseKind::Present),
      x if x == CommittmentResponseKind::Tardy as u8 => Ok(CommittmentResponseKind::Tardy),
      x if x == CommittmentResponseKind::Absent as u8 => Ok(CommittmentResponseKind::Absent),
      x if x == CommittmentResponseKind::Cancelled as u8 => Ok(CommittmentResponseKind::Cancelled),
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

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubscriptionNewProps {
  pub subscription_kind: SubscriptionKind,
  pub api_key: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CourseNewProps{
  pub school_id: i64,
  pub name: String,
  pub description: String,
  pub api_key: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CourseDataNewProps{
  pub course_id: i64,
  pub name: String,
  pub description: String,
  pub active: bool,
  pub api_key: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CourseKeyNewValidProps{
  pub course_id: i64,
  pub course_membership_kind: CourseMembershipKind,
  pub max_uses: i64,
  pub api_key: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CourseKeyNewCancelProps{
  pub course_key: String,
  pub api_key: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CourseMembershipNewCancelProps{
  pub user_id: i64,
  pub course_id: i64,
  pub api_key: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CourseMembershipNewKeyProps{
  pub course_key: String,
  pub api_key: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SchoolNewProps{
  pub name: String,
  pub description: String,
  pub whole: bool,
  pub api_key: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SchoolDataNewProps{
  pub school_id: i64,
  pub name: String,
  pub description: String,
  pub active: bool,
  pub api_key: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdminshipRequestNewProps{
  pub school_id: i64,
  pub message: String,
  pub api_key: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdminshipRequestResponseNewProps{
  pub adminship_request_id: i64,
  pub message: String,
  pub accept: bool,
  pub api_key: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdminshipNewCancelProps{
  pub user_id: i64,
  pub school_id: i64,
  pub api_key: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdminshipNewValidProps{
  pub adminship_request_response_id: i64,
  pub api_key: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SessionNewProps{
  pub name: String,
  pub course_id: i64,
  pub hidden: bool,
  pub api_key: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SessionDataNewProps{
  pub session_id: i64,
  pub name: String,
  pub hidden: bool,
  pub active: bool,
  pub api_key: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SessionRequestNewProps{
  pub course_id: i64,
  pub message: String,
  pub api_key: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SessionRequestResponseNewRejectProps{
  pub session_request_id: i64,
  pub message: String,
  pub api_key: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SessionRequestResponseNewAcceptProps{
  pub session_request_id: i64,
  pub message: String,
  pub committment_id: i64,
  pub api_key: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommittmentNewProps{
  pub attendee_user_id: i64,
  pub session_id: i64,
  pub cancellable: bool,
  pub api_key: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommittmentResponseNewProps{
  pub committment_id: i64,
  pub committment_response_kind: CommittmentResponseKind,
  pub api_key: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubscriptionViewProps{
  pub subscription_id: Option<Vec<i64>>,          
  pub min_creation_time: Option<i64>,             
  pub max_creation_time: Option<i64>,             
  pub creator_user_id: Option<i64>,               
  pub subscription_kind: Option<SubscriptionKind>,
  pub max_uses: Option<i64>,                      
  pub only_recent: bool,                          
  pub offset: Option<i64>,
  pub count: Option<i64>,
  pub api_key: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SchoolViewProps{
  pub school_id: Option<Vec<i64>>,         //
  pub min_creation_time: Option<i64>, //
  pub max_creation_time: Option<i64>, //
  pub creator_user_id: Option<i64>,   //
  pub whole: Option<bool>,            //
  pub offset: Option<i64>,
  pub count: Option<i64>,
  pub api_key: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SchoolDataViewProps{
  pub school_data_id: Option<Vec<i64>>,    
  pub min_creation_time: Option<i64>,      
  pub max_creation_time: Option<i64>,      
  pub creator_user_id: Option<i64>,        
  pub school_id: Option<i64>,              
  pub name: Option<String>,                
  pub partial_school_name: Option<String>,
  pub description: Option<String>,         
  pub partial_description: Option<String>, 
  pub active: Option<bool>,                
  pub only_recent: bool,                   
  pub offset: Option<i64>,
  pub count: Option<i64>,
  pub api_key: String,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]

pub struct CourseViewProps{
  pub course_id: Option<Vec<i64>>,         //
  pub min_creation_time: Option<i64>, //
  pub max_creation_time: Option<i64>, //
  pub creator_user_id: Option<i64>,   //
  pub school_id: Option<i64>,         //
  pub offset: Option<i64>,
  pub count: Option<i64>,
  pub api_key: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CourseDataViewProps{
  pub course_data_id: Option<Vec<i64>>,            //
  pub min_creation_time: Option<i64>,         //
  pub max_creation_time: Option<i64>,         //
  pub creator_user_id: Option<i64>,           //
  pub course_id: Option<i64>,                 //
  pub name: Option<String>,                   //
  pub partial_name: Option<String>,           //
  pub description: Option<String>,            //
  pub partial_description: Option<String>,    //
  pub active: Option<bool>,                   //
  pub only_recent: bool,                      //
  pub school_id: Option<i64>,                 //
  pub offset: Option<i64>,
  pub count: Option<i64>,
  pub api_key: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CourseKeyViewProps{
  pub course_key_id: Option<Vec<i64>>,
  pub min_creation_time: Option<i64>,
  pub max_creation_time: Option<i64>,
  pub creator_user_id: Option<i64>,
  pub course_id: Option<i64>,
  pub course_key_valid: Option<bool>,
  pub course_membership_kind: Option<CourseMembershipKind>,
  pub min_duration: Option<i64>,
  pub max_duration: Option<i64>,
  pub max_uses: Option<i64>,
  pub only_recent: bool,
  pub offset: Option<i64>,
  pub count: Option<i64>,
  pub api_key: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CourseMembershipViewProps{
  pub course_membership_id: Option<Vec<i64>>,
  pub min_creation_time: Option<i64>,
  pub max_creation_time: Option<i64>,
  pub creator_user_id: Option<i64>,
  pub user_id: Option<i64>,
  pub course_id: Option<i64>,
  pub course_membership_kind: Option<CourseMembershipKind>,
  pub course_membership_has_source: Option<bool>,
  pub course_key_id: Option<i64>,
  pub user_name: Option<String>,
  pub partial_user_name: Option<String>,
  pub only_recent: bool,
  pub offset: Option<i64>,
  pub count: Option<i64>,
  pub api_key: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdminshipRequestViewProps{
  pub adminship_request_id: Option<Vec<i64>>,
  pub min_creation_time: Option<i64>,
  pub max_creation_time: Option<i64>,
  pub creator_user_id: Option<i64>,
  pub school_id: Option<i64>,
  pub message: Option<String>,
  pub responded: Option<bool>,
  pub offset: Option<i64>,
  pub count: Option<i64>,
  pub api_key: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdminshipRequestResponseViewProps{
  pub adminship_request_id: Option<Vec<i64>>,
  pub min_creation_time: Option<i64>,
  pub max_creation_time: Option<i64>,
  pub creator_user_id: Option<i64>,
  pub message: Option<String>,
  pub accepted: Option<bool>,
  pub responded: Option<bool>,
  pub requester_user_id: Option<i64>,
  pub school_id: Option<i64>,
  pub offset: Option<i64>,
  pub count: Option<i64>,
  pub api_key: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdminshipViewProps{
  pub adminship_id: Option<Vec<i64>>,
  pub min_creation_time: Option<i64>,
  pub max_creation_time: Option<i64>,
  pub creator_user_id: Option<i64>,
  pub user_id: Option<i64>,
  pub school_id: Option<i64>,
  pub adminship_kind: Option<AdminshipKind>,
  pub adminship_has_source: Option<bool>,
  pub adminship_request_response_id: Option<i64>,
  pub user_name: Option<String>,
  pub partial_user_name: Option<String>,
  pub only_recent: bool,
  pub offset: Option<i64>,
  pub count: Option<i64>,
  pub api_key: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SessionViewProps{
  pub session_id: Option<Vec<i64>>,
  pub min_creation_time: Option<i64>,
  pub max_creation_time: Option<i64>,
  pub creator_user_id: Option<i64>,
  pub course_id: Option<i64>,
  pub offset: Option<i64>,
  pub count: Option<i64>,
  pub api_key: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SessionDataViewProps{
  pub session_data_id: Option<Vec<i64>>,
  pub min_creation_time: Option<i64>,
  pub max_creation_time: Option<i64>,
  pub creator_user_id: Option<i64>,
  pub session_id: Option<i64>,
  pub name: Option<String>,
  pub partial_name: Option<String>,
  pub min_start_time: Option<i64>,
  pub max_start_time: Option<i64>,
  pub min_duration: Option<i64>,
  pub max_duration: Option<i64>,
  pub hidden: Option<bool>,
  pub active: Option<bool>,
  pub only_recent: bool,
  pub course_id: Option<i64>,
  pub offset: Option<i64>,
  pub count: Option<i64>,
  pub api_key: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SessionRequestViewProps{
  pub session_request_id: Option<Vec<i64>>,
  pub min_creation_time: Option<i64>,
  pub max_creation_time: Option<i64>,
  pub creator_user_id: Option<i64>,
  pub attendee_user_id: Option<i64>,
  pub course_id: Option<i64>,
  pub message: Option<String>,
  pub min_start_time: Option<i64>,
  pub max_start_time: Option<i64>,
  pub min_duration: Option<i64>,
  pub max_duration: Option<i64>,
  pub responded: Option<bool>,
  pub offset: Option<i64>,
  pub count: Option<i64>,
  pub api_key: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SessionRequestResponseViewProps{
  pub session_request_id: Option<Vec<i64>>,
  pub min_creation_time: Option<i64>,
  pub max_creation_time: Option<i64>,
  pub creator_user_id: Option<i64>,
  pub message: Option<String>,
  pub accepted: Option<bool>,
  pub committment_id: Option<i64>,
  pub attendee_user_id: Option<i64>,
  pub course_id: Option<i64>,
  pub min_start_time: Option<i64>,
  pub max_start_time: Option<i64>,
  pub min_duration: Option<i64>,
  pub max_duration: Option<i64>,
  pub responded: Option<bool>,
  pub session_id: Option<i64>,
  pub offset: Option<i64>,
  pub count: Option<i64>,
  pub api_key: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommittmentViewProps{
  pub committment_id: Option<Vec<i64>>,
  pub min_creation_time: Option<i64>,
  pub max_creation_time: Option<i64>,
  pub creator_user_id: Option<i64>,
  pub attendee_user_id: Option<i64>,
  pub session_id: Option<i64>,
  pub cancellable: Option<bool>,
  pub course_id: Option<i64>,
  pub min_start_time: Option<i64>,
  pub max_start_time: Option<i64>,
  pub min_duration: Option<i64>,
  pub max_duration: Option<i64>,
  pub responded: Option<bool>,
  pub from_request_response: Option<bool>,
  pub offset: Option<i64>,
  pub count: Option<i64>,
  pub api_key: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommittmentResponseViewProps{
  pub committment_id: Option<Vec<i64>>,
  pub min_creation_time: Option<i64>,
  pub max_creation_time: Option<i64>,
  pub creator_user_id: Option<i64>,
  pub committment_response_kind: Option<CommittmentResponseKind>,
  pub attendee_user_id: Option<i64>,
  pub course_id: Option<i64>,
  pub min_start_time: Option<i64>,
  pub max_start_time: Option<i64>,
  pub min_duration: Option<i64>,
  pub max_duration: Option<i64>,
  pub session_id: Option<i64>,
  pub offset: Option<i64>,
  pub count: Option<i64>,
  pub api_key: String,
}
