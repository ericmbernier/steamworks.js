use napi_derive::napi;

#[napi]
pub mod friends {
    use napi_derive::napi;
    use steamworks::FriendFlags;

    /// A single Steam friend's public info.
    #[napi(object)]
    pub struct Friend {
        pub steam_id64: String,
        pub name: String,
        pub state: u32,
    }

    /// List the local user's immediate friends (persona name + id + state).
    /// Returns an empty vec when Steam has no friend data available.
    #[napi]
    pub fn get_friends() -> Vec<Friend> {
        let client = crate::client::get_client();
        client
            .friends()
            .get_friends(FriendFlags::IMMEDIATE)
            .iter()
            .map(|friend| Friend {
                steam_id64: friend.id().raw().to_string(),
                name: friend.name(),
                state: friend.state() as u32,
            })
            .collect()
    }
}
