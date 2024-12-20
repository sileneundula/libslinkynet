/// # SlinkyCommands
/// 
/// ## List of Commands
/// 
/// - help
/// 
/// ## File Directory
/// - lsfs [list file directory]
/// - cd [change file directory]
/// - newfs [new file]
/// 
/// ## Network
/// - lsn peers
/// 
/// ## Keyring (stored locally or cloud)
/// - key
/// - keyring
/// - get-key
/// - del-key
/// 

pub enum Commands {
    // About
    version,
    
    // File System (Sandboxed)
    lsfs,
    cd,
    newfs,
    sumfs,
    
    home, // home folder


    // Network
    init, // Initialize New Node
    connect, // Connect To Network

    lsn, // List Network
    lsp, // List Virtualized Ports (maybe)
    lsi, // List Identities
    
    lsprotocols, // List Protocols

    // Package
    spm, // Slinky Package Manager
    lsspm, // List Slinky Packages

    // Bridges
    bridgr, // Bridge Module

    // Security
    security, // security panel
    set_secure_level, // Sets Security Level

    // Slinky Network
    sli_get, // slinky-get | Get from slinky

    // Slab
    slab,
}