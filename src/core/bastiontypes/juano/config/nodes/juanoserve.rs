pub struct JuanoServe;

pub struct JuanoServeAddress(String);

pub struct JuanoServeConfig {
    serve_type: JuanoServeType,
    auth: JuanoServeAuthTypes,
    
    immutable: bool,
}

pub enum JuanoServeType {
    Markdown,
    Static,
}

pub enum JuanoServeAuthTypes {
    None,
    Certificate,
    Password,
    BorneoLogin,
}