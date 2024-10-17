use registry::*;
use registry::Hive;

/*
HIVE
    ClassesRoot,
    CurrentConfig,
    CurrentUser,
    CurrentUserLocalSettings,
    LocalMachine,
    PerformanceData,
    Users,
*/

pub struct ShativiraRegistryAPI;

pub enum RegistryActions {
    Read,
    Delete,
    Edit,
}

// Computer\HKEY_USERS\.DEFAULT\Control Panel\Desktop\MuiCached

impl ShativiraRegistryAPI {
    pub fn new<T: AsRef<str>>(hive: registry::Hive, path: T, action: registry::Security) {
        let registrypath = path.as_ref();

        let regpath_split = registrypath.split("\\").into_iter();

        hive::


    }
}