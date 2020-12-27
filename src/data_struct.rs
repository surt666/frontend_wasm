use strum_macros::{Display, EnumIter, EnumString}; 

#[derive(Debug, Clone, EnumIter, Display, EnumString)]
pub enum Region{
    #[strum(serialize = "EuCentral1", serialize = "eu-central-1")]
    EuCentral1,
    #[strum(serialize = "UsEast1", serialize = "us-east-1")]
    UsEast1,
} 

impl  Default for Region {
    fn default() -> Self { Region::EuCentral1}
}

#[derive(Debug, Clone, EnumIter, Display, EnumString)]
pub enum Criticality{
    Critical,
    NonCritical,
} 

impl  Default for Criticality {
    fn default() -> Self { Criticality::NonCritical}
}

#[derive(Debug, Clone, EnumIter, Display, EnumString)]
pub enum Confidentiality{
    StrictlyConfidential,
    Confidential,
    Internal,
    Public,
} 

impl  Default for Confidentiality {
    fn default() -> Self { Confidentiality::Public}
}

pub type AccountNumber = String;

#[derive(Debug, Default, Clone)]
pub struct Dataset {
    pub name: String,
    pub managed: bool,
    pub financial: bool,
    pub esh: bool,
    pub gxp: bool,
    pub pii: bool,
    pub confidentiality: Confidentiality,
    pub criticality: Criticality,
    pub owner: String,
    pub company: Option<String>,
    pub domain: Option<String>,
    pub cost_center: String,
    pub region: Region,
    pub client_account: AccountNumber,
    pub sns_topic: String,
}