use std::collections::HashMap;

/// Devicetree Node Status
pub enum Status {
    Okay,
    Disabled,
    Reserved,
    Fail,
    FailSSS,
}

/// DeviceTree compatible is a String alias
///
/// The compatible string may commonly be formatted as manufacturer,model
/// and so an alias with type specific helpers is available for inspecting
/// strings of this format.
pub type Compatible = String;

/// DeviceTree address cells is a u32 alias
pub type AddressCells = u32;

/// Devicetree size cells is a u32 alias
pub type SizeCells = u32;

/// A generic property of a node
pub enum Property {
    Phandle(u32),
    U32(u32),
    U64(u64),
    String(String),
    StringList(Vec<String>),
}

/// DeviceTree Node with common attributes as members
pub struct Node {
    pub name: String,
    pub unit_address: u64,
    pub label: String,
    pub status: Status,
    pub compatible: Vec<Compatible>,
    pub model: String,
    pub address_cells: AddressCells,
    pub size_cells: SizeCells,
    pub properties: HashMap<String, Property>,
}

/// Version directive data
pub enum Version {
    V1,
}

/// Memory reserve directive data
pub struct MemReserve {
    pub address: u64,
    pub length: u64,
}

/// Node selector
pub enum NodeSelector {
    Path(String),
    Label(String),
    Name(String),
}

/// Property selector
pub enum PropertySelector {
    Name(String),
}

/// DTS directives
pub enum Directive  {
    DtsVersion(Version),
    MemReserve(MemReserve),
    Include(String),
    DeleteNode(NodeSelector),
    DeleteProperty(PropertySelector),
}

/// DeviceTree represented as an IR
pub struct DeviceTree {
    pub directives: Vec<Directive>,
    pub nodes: Vec<Node>,
    pub relations: Vec<(u32, Vec<u32>)>
}

pub fn parse_dts(dts: &str) -> Result<DeviceTree, std::io::Error> {
    Ok(DeviceTree {
        directives: Vec::new(),
        nodes: Vec::new(),
        relations: Vec::new(),
    })
}
