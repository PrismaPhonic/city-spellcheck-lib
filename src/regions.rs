use std::fmt;

#[derive(Debug, Copy, Clone)]
pub enum Region {
    Province(CAProvince),
    Territory(CATerritory),
    State(USState),
    None,
}

#[derive(Debug, Copy, Clone)]
pub enum CAProvince {
    ON,
    QC,
    NS,
    NB,
    MB,
    BC,
    PE,
    SK,
    AB,
    NL,
}

#[derive(Debug, Copy, Clone)]
pub enum CATerritory {
    NT,
    NU,
    YT,
}

#[derive(Debug, Copy, Clone)]
pub enum USState {
    AL,
    AK,
    AZ,
    AR,
    CA,
    CO,
    CT,
    DE,
    FL,
    GA,
    HI,
    ID,
    IL,
    IN,
    IA,
    KS,
    KY,
    LA,
    ME,
    MD,
    MA,
    MI,
    MN,
    MS,
    MO,
    MT,
    NE,
    NV,
    NH,
    NJ,
    NM,
    NY,
    NC,
    ND,
    OH,
    OK,
    OR,
    PA,
    RI,
    SC,
    SD,
    TN,
    TX,
    UT,
    VT,
    VA,
    WA,
    WV,
    WI,
    WY,
}

impl fmt::Display for Region {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Region::State(USState::AL) => write!(f, "AL"),
            Region::State(USState::AK) => write!(f, "AK"),
            Region::State(USState::AZ) => write!(f, "AZ"),
            Region::State(USState::AR) => write!(f, "AR"),
            Region::State(USState::CA) => write!(f, "CA"),
            Region::State(USState::CO) => write!(f, "CO"),
            Region::State(USState::CT) => write!(f, "CT"),
            Region::State(USState::DE) => write!(f, "DE"),
            Region::State(USState::FL) => write!(f, "FL"),
            Region::State(USState::GA) => write!(f, "GA"),
            Region::State(USState::HI) => write!(f, "HI"),
            Region::State(USState::ID) => write!(f, "ID"),
            Region::State(USState::IL) => write!(f, "IL"),
            Region::State(USState::IN) => write!(f, "IN"),
            Region::State(USState::IA) => write!(f, "IA"),
            Region::State(USState::KS) => write!(f, "KS"),
            Region::State(USState::KY) => write!(f, "KY"),
            Region::State(USState::LA) => write!(f, "LA"),
            Region::State(USState::ME) => write!(f, "ME"),
            Region::State(USState::MD) => write!(f, "MD"),
            Region::State(USState::MA) => write!(f, "MA"),
            Region::State(USState::MI) => write!(f, "MI"),
            Region::State(USState::MN) => write!(f, "MN"),
            Region::State(USState::MS) => write!(f, "MS"),
            Region::State(USState::MO) => write!(f, "MO"),
            Region::State(USState::MT) => write!(f, "MT"),
            Region::State(USState::NE) => write!(f, "NE"),
            Region::State(USState::NV) => write!(f, "NV"),
            Region::State(USState::NH) => write!(f, "NH"),
            Region::State(USState::NJ) => write!(f, "NJ"),
            Region::State(USState::NM) => write!(f, "NM"),
            Region::State(USState::NY) => write!(f, "NY"),
            Region::State(USState::NC) => write!(f, "NC"),
            Region::State(USState::ND) => write!(f, "ND"),
            Region::State(USState::OH) => write!(f, "OH"),
            Region::State(USState::OK) => write!(f, "OK"),
            Region::State(USState::OR) => write!(f, "OR"),
            Region::State(USState::PA) => write!(f, "PA"),
            Region::State(USState::RI) => write!(f, "RI"),
            Region::State(USState::SC) => write!(f, "SC"),
            Region::State(USState::SD) => write!(f, "SD"),
            Region::State(USState::TN) => write!(f, "TN"),
            Region::State(USState::TX) => write!(f, "TX"),
            Region::State(USState::UT) => write!(f, "UT"),
            Region::State(USState::VT) => write!(f, "VT"),
            Region::State(USState::VA) => write!(f, "VA"),
            Region::State(USState::WA) => write!(f, "WA"),
            Region::State(USState::WV) => write!(f, "WV"),
            Region::State(USState::WI) => write!(f, "WI"),
            Region::State(USState::WY) => write!(f, "WY"),
            Region::Province(CAProvince::AB) => write!(f, "AB"),
            Region::Province(CAProvince::BC) => write!(f, "BC"),
            Region::Province(CAProvince::MB) => write!(f, "MB"),
            Region::Province(CAProvince::NB) => write!(f, "NB"),
            Region::Province(CAProvince::NL) => write!(f, "NL"),
            Region::Province(CAProvince::NS) => write!(f, "NS"),
            Region::Province(CAProvince::ON) => write!(f, "ON"),
            Region::Province(CAProvince::PE) => write!(f, "PE"),
            Region::Province(CAProvince::QC) => write!(f, "QC"),
            Region::Province(CAProvince::SK) => write!(f, "SK"),
            Region::Territory(CATerritory::NT) => write!(f, "NT"),
            Region::Territory(CATerritory::NU) => write!(f, "NU"),
            Region::Territory(CATerritory::YT) => write!(f, "YT"),
            Region::None => write!(f, ""),
        }
    }
}
