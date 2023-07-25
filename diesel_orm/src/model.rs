use diesel::prelude::*;
use diesel::sql_types::*;

use crate::schema::{releases, changelogs, artifacts, affected_files};

#[derive(diesel_derive_enum::DbEnum, Debug, Copy, Clone, PartialEq, Eq)]
#[ExistingTypePath = "crate::schema::sql_types::ChannelType"]
pub enum ChannelType {
    Nightly,
    Beta,
    Stable,
}

#[derive(diesel_derive_enum::DbEnum, Debug, Copy, Clone, PartialEq, Eq)]
#[ExistingTypePath = "crate::schema::sql_types::SizeUnit"]
pub enum SizeUnit {
    Kb,
    Mb,
    Gb,
}

#[derive(diesel_derive_enum::DbEnum, Debug, Copy, Clone, PartialEq, Eq)]
#[ExistingTypePath = "crate::schema::sql_types::EditType"]
pub enum EditType {
    Add,
    Edit,
    Delete,
}


#[derive(Default, Queryable, Selectable, Identifiable, Associations, Deserialize, Serialize, Insertable, Debug, PartialEq)]
#[diesel(table_name = releases)]
pub struct Release {
    #[diesel(deserialize_as = "i32")]
    pub release_id: i32,
    pub channel: ChannelType,
    pub repo_fullname: String,
    pub diffs_url: String,
    pub released_at: Date,
}


#[derive(Default, Queryable, Selectable, Identifiable, Associations, Deserialize, Serialize, Insertable, Debug, PartialEq)]
#[diesel(belongs_to(Release))]
#[diesel(table_name = changelogs)]
pub struct Changelog {
    #[diesel(deserialize_as = "i32")]
    pub changelog_id: i32,
    pub commit_id: Bpchar,
    pub commited_at: Timestamp,
    pub commit_comment: String,
    pub commited_by: String,
    pub release_id: i32,
}

#[derive(Default, Queryable, Selectable, Identifiable, Associations, Deserialize, Serialize, Insertable, Debug, PartialEq)]
#[diesel(belongs_to(Release))]
#[diesel(table_name = artifacts)]
pub struct Artifact {
    #[diesel(deserialize_as = "i32")]
    pub artifact_id: i32,
    pub filename: String,
    pub filesize: Numeric,
    pub filesize_unit: SizeUnit,
    pub release_id: i32,
}

#[derive(Default, Queryable, Selectable, Identifiable, Associations, Deserialize, Serialize, Insertable, Debug, PartialEq)]
#[diesel(belongs_to(Release))]
#[diesel(table_name = affected_files)]
pub struct AffectedFiles {
    #[diesel(deserialize_as = "i32")]
    pub affected_file_id: i32,
    pub file_edit_type: EditType,
    pub file_path: String,
    pub release_id: i32,
}

#[derive(Insertable)]
#[diesel(table_name = releases)]
pub struct NewRelease<'a> {
    pub channel: &'a ChannelType,
    pub repo_fullname: &'a String,
    pub diffs_url: &'a String,
    pub released_at: &'a Date,
}

#[derive(Insertable)]
#[diesel(table_name = changelogs)]
pub struct NewChangelog<'a> {
    pub commit_id: &'a String,
    pub commited_at: &'a Timestamp,
    pub commit_comment, &'a String,
    pub commited_by: &'a String,
    pub release_id: &'a i32,
}

#[derive(Insertable)]
#[diesel(table_name = artifacts)]
pub struct NewArtifact<'a> {
    pub filename: &'a String,
    pub filesize: &'a Numeric,
    pub filesize_unit: &'a SizeUnit,
    pub release_id: &'a i32,
}

#[derive(Insertable)]
#[diesel(table_name = affected_files)]
pub struct NewAffectedFile<'a> {
    pub file_edit_type: &'a EditType,
    pub file_path: &'a String,
    pub release_id: &'a i32,
}

