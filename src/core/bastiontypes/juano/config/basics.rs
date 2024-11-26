/// # JuanoNodeType
/// 
/// ## JuanoNodeTypes
/// 
/// 
/// ### Basic
/// - Relay (0x00)
/// - Bridge (0x01)
/// - Database (can be open or private, uses connector)
/// - Gateway
/// - DecentralizedConsensusVirtualMachines (DCVM)
/// - StorageUnit
/// 
/// ### Advanced
/// - JuanoServe (can be attached to borneo)
/// - JuanoRepos
/// - JuanoLogicalUnit (Logic Unit)
/// - JuanoScript
/// - JuanoDatabase
pub struct JuanoNodeType(u16);

pub enum JuanoNodeTypes {
    Relay,
    Bridge,
    Database,
    Gateway,
    StrgUnit,
    
    DCVM,
}

pub struct JuanoNodeConfig<T>(T);

pub struct JuanoNodeBlockRepoPivot(String);

impl JuanoNodeType {
    pub fn new(nodetype: u16) -> Self {
        return Self(nodetype)
    }
    pub fn from(nodetype: u16) -> Self {
        return Self(nodetype)
    }
}