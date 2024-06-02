/// Actions related to KnownHostsStore
pub enum KnownHostsAction {
    /// This action will issue an update to the KnownHostsStore
    LoadKnownHosts,
}

/// Actions related to AppStateStore
pub enum AppStateAction {
    /// This action will make the AppStateStore set the current screen to Exiting
    Exit,
    /// This action will make the application end
    StopApp
}

pub enum Action {
    KnownHostsAction(KnownHostsAction),
    AppStateAction(AppStateAction)
}
