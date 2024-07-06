// module org.apache.zookeeper.data
pub mod data {
    use jute::jute_message;

    jute_message!(Id {
        scheme: String,
        id: String,
    });

    jute_message!(ACL { perms: i32, id: Id });

    jute_message!(Stat {
        czxid: i64,
        mzxid: i64,
        ctime: i64,
        mtime: i64,
        version: i32,
        cversion: i32,
        aversion: i32,
        ephemeral_owner: i64,
        data_length: i32,
        num_children: i32,
        pzxid: i64,
    });

    jute_message!(StatPersisted {
        czxid: i64,           // created zxid
        mzxid: i64,           // last modified zxid
        ctime: i64,           // created
        mtime: i64,           // last modified
        version: i32,         // version
        cversion: i32,        // child version
        aversion: i32,        // acl version
        ephemeral_owner: i64, // owner id if ephemeral, 0 otw
        pzxid: i64,           // last modified children
    });

    jute_message!(ClientInfo {
        auth_scheme: String, // Authentication scheme
        user: String,        // username or any other id(for example ip)
    });
}

// module org.apache.zookeeper.proto
pub mod proto {
    use super::data::{ClientInfo, Stat, ACL};
    use jute::{jute_message, Buffer};

    jute_message!(ConnectRequest {
        protocol_version: i32,
        last_zxid_seen: i64,
        time_out: i32,
        session_id: i64,
        passwd: Buffer,
        read_only: bool,
    });

    jute_message!(ConnectResponse {
        protocol_version: i32,
        timeout: i32,
        session_id: i64,
        passwd: Buffer,
        read_only: bool,
    });

    jute_message!(SetWatches {
        relative_zxid: i64,
        data_watches: Vec<String>,
        exist_watches: Vec<String>,
        child_watches: Vec<String>,
    });

    jute_message!(SetWatches2 {
        relative_zxid: i64,
        data_watches: Vec<String>,
        exist_watches: Vec<String>,
        child_watches: Vec<String>,
        persistent_watches: Vec<String>,
        persistent_recursive_watches: Vec<String>,
    });

    jute_message!(RequestHeader {
        xid: i32,
        r#type: i32,
    });

    jute_message!(MultiHeader {
        r#type: i32,
        done: bool,
        err: i32,
    });

    jute_message!(AuthPacket {
        r#type: i32,
        scheme: String,
        auth: Buffer,
    });

    jute_message!(ReplyHeader {
        xid: i32,
        zxid: i64,
        err: i32,
    });

    jute_message!(GetDataRequest {
        path: String,
        watch: bool,
    });

    jute_message!(SetDataRequest {
        path: String,
        data: Buffer,
        version: i32,
    });

    jute_message!(ReconfigRequest {
        joining_servers: String,
        leaving_servers: String,
        new_members: String,
        cur_config_id: i64,
    });

    jute_message!(SetDataResponse { stat: Stat });

    jute_message!(GetSASLRequest { token: Buffer });

    jute_message!(SetSASLRequest { token: Buffer });

    jute_message!(SetSASLResponse { token: Buffer });

    jute_message!(CreateRequest {
        path: String,
        data: Buffer,
        acl: Vec<ACL>,
        flags: i32,
    });

    jute_message!(CreateTTLRequest {
        path: String,
        data: Buffer,
        acl: Vec<ACL>,
        flags: i32,
        ttl: i64,
    });

    jute_message!(DeleteRequest {
        path: String,
        version: i32,
    });

    jute_message!(GetChildrenRequest {
        path: String,
        watch: bool,
    });

    jute_message!(GetAllChildrenNumberRequest { path: String });

    jute_message!(GetChildren2Request {
        path: String,
        watch: bool,
    });

    jute_message!(CheckVersionRequest {
        path: String,
        version: i32,
    });

    jute_message!(GetMaxChildrenRequest { path: String });

    jute_message!(GetMaxChildrenResponse { max: i32 });

    jute_message!(SetMaxChildrenRequest {
        path: String,
        max: i32,
    });

    jute_message!(SyncRequest { path: String });

    jute_message!(SyncResponse { path: String });

    jute_message!(GetACLRequest { path: String });

    jute_message!(SetACLRequest {
        path: String,
        acl: Vec<ACL>,
        version: i32,
    });

    jute_message!(SetACLResponse { stat: Stat });

    jute_message!(AddWatchRequest {
        path: String,
        mode: i32,
    });

    jute_message!(WatcherEvent {
        r#type: i32, // event type
        state: i32,  // state of the Keeper client runtime
        path: String,
    });

    jute_message!(ErrorResponse { err: i32 });

    jute_message!(CreateResponse { path: String });

    jute_message!(Create2Response {
        path: String,
        stat: Stat,
    });

    jute_message!(ExistsRequest {
        path: String,
        watch: bool,
    });

    jute_message!(ExistsResponse { stat: Stat });

    jute_message!(GetDataResponse {
        data: Buffer,
        stat: Stat,
    });

    jute_message!(GetChildrenResponse {
         children: Vec<String>,
    });

    jute_message!(GetAllChildrenNumberResponse { total_number: i32 });

    jute_message!(GetChildren2Response {
         children: Vec<String>,
        stat: Stat,
    });

    jute_message!(GetACLResponse {
        acl: Vec<ACL>,
        stat: Stat,
    });

    jute_message!(CheckWatchesRequest {
        path: String,
        r#type: i32,
    });

    jute_message!(RemoveWatchesRequest {
        path: String,
        r#type: i32,
    });

    jute_message!(GetEphemeralsRequest {
        prefix_path: String
    });

    jute_message!(GetEphemeralsResponse {
         ephemerals: Vec<String>,
    });

    jute_message!(WhoAmIResponse {
         client_info: Vec<ClientInfo>,
    });
}

// module org.apache.zookeeper.server.quorum
pub mod quorum {
    use super::data::Id;
    use jute::{jute_message, Buffer};

    jute_message!(LearnerInfo {
        server_id: i64,
        protocol_version: i32,
        config_version: i64,
    });

    jute_message!(QuorumPacket {
        r#type:i32, // Request, Ack, Commit, Ping
        zxid: i64,
        data: Buffer, // Only significant when type is request
        authinfo: Vec<Id>,
    });

    jute_message!(QuorumAuthPacket {
        magic: i64,
        status: i32,
        token: Buffer,
    });
}

// module org.apache.zookeeper.server.persistence
pub mod persistence {
    use jute::jute_message;

    jute_message!(FileHeader {
        magic: i32,
        version: i32,
        dbid: i64,
    });
}

// module org.apache.zookeeper.txn
pub mod txn {
    use jute::{jute_message, Buffer};

    use super::data::ACL;

    jute_message!(TxnDigest {
        version: i32,
        tree_digest: i64,
    });

    jute_message!(TxnHeader {
        client_id: i64,
        cxid: i32,
        zxid: i64,
        time: i64,
        r#type: i32,
    });

    jute_message!(CreateTxnV0 {
        path: String,
        data: Buffer,
         acl: Vec<ACL>,
        ephemeral: bool,
    });

    jute_message!(CreateTxn {
        path: String,
        data: Buffer,
        acl: Vec<ACL>,
        ephemeral: bool,
        parent_cversion: i32,
    });

    jute_message!(CreateTTLTxn {
        path: String,
        data: Buffer,
        acl: Vec<ACL>,
        parent_cversion: i32,
        ttl: i64,
    });

    jute_message!(CreateContainerTxn {
        path: String,
        data: Buffer,
        acl: Vec<ACL>,
        parent_cversion: i32,
    });

    jute_message!(DeleteTxn { path: String });

    jute_message!(SetDataTxn {
        path: String,
        data: Buffer,
        version: i32,
    });

    jute_message!(CheckVersionTxn {
        path: String,
        version: i32,
    });

    jute_message!(SetACLTxn {
        path: String,
         acl: Vec<ACL>,
        version: i32,
    });

    jute_message!(SetMaxChildrenTxn {
        path: String,
        max: i32,
    });

    jute_message!(CreateSessionTxn { timeout: i32 });

    jute_message!(CloseSessionTxn {
        paths2delete: Vec<String>,
    });

    jute_message!(ErrorTxn { err: i32 });

    jute_message!(Txn {
        r#type: i32,
        data: Buffer,
    });

    jute_message!(MultiTxn {
        txns: Vec<Txn>,
    });
}
