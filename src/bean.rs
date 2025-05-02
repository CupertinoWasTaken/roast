use std::collections::HashMap;

use semver::VersionReq;
use serde::{Deserialize, Serialize};
use url::Url;

use crate::triple::SerdeTriple;

#[derive(Serialize, Deserialize)]
pub enum PackageKind {
    Binary,
    Library,
}

type BoxedStr = Box<str>;

#[derive(Serialize, Deserialize)]
pub struct PackageData {
    pub name: BoxedStr,
    pub version: VersionReq,
    pub kind: PackageKind,
    pub description: Option<BoxedStr>,
    pub license: Option<BoxedStr>,
    pub maintainers: Vec<BoxedStr>,
    pub homepage: Option<Url>,
}

#[derive(Serialize, Deserialize)]
pub struct PackageSource {
    url: Url,
    #[serde(with = "hex")]
    hash: [u8; blake3::OUT_LEN],
}

#[derive(Serialize, Deserialize)]
pub struct BeanFile {
    pub package: PackageData,
    pub dependences: HashMap<BoxedStr, VersionReq>,
    pub sources: HashMap<SerdeTriple, PackageSource>
}
