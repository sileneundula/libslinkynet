/// # JuanoNodeType
/// 
/// ## JuanoNodeTypes
/// 
/// 
/// ### Basic
/// - Relay
/// - Bridge
/// - Database (can be open or private, uses connector)
/// - Gateway
/// - DecentralizedConsensusVirtualMachines (DCVM)
/// 
/// ### Advanced
/// - JuanoServe (can be attached to borneo)
/// - JuanoRepos
/// - JuanoLogicalUnit (Logic Unit)
/// - JuanoScript
pub struct JuanoNodeType(u16);

pub struct JuanoNodeContainer<T>(T);

pub struct JuanoNodeBlockRepoPivot(String);

impl JuanoNodeType {
    pub fn new(nodetype: u16) -> Self {
        return Self(nodetype)
    }
    pub fn from(nodetype: u16) -> Self {
        return Self(nodetype)
    }
}