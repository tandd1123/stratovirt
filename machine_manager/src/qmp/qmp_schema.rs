// Copyright (c) 2020 Huawei Technologies Co.,Ltd. All rights reserved.
//
// StratoVirt is licensed under Mulan PSL v2.
// You can use this software according to the terms and conditions of the Mulan
// PSL v2.
// You may obtain a copy of Mulan PSL v2 at:
//         http://license.coscl.org.cn/MulanPSL2
// THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY
// KIND, EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO
// NON-INFRINGEMENT, MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
// See the Mulan PSL v2 for more details.

extern crate serde;
extern crate serde_json;

use serde::{Deserialize, Serialize};
pub use serde_json::Value as Any;

use crate::qmp::{Command, Empty, Event, TimeStamp};

/// A error enum for qmp
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum QmpErrorClass {
    #[serde(rename = "GenericError")]
    GenericError(String),
    #[serde(rename = "CommandNotFound")]
    CommandNotFound(String),
    #[serde(rename = "DeviceNotActive")]
    DeviceNotActive(String),
    #[serde(rename = "DeviceNotFound")]
    DeviceNotFound(String),
    #[serde(rename = "KVMMissingCap")]
    KVMMissingCap(String),
}

impl QmpErrorClass {
    pub fn to_content(&self) -> String {
        match self {
            QmpErrorClass::GenericError(s) => s.to_string(),
            QmpErrorClass::CommandNotFound(s) => s.to_string(),
            QmpErrorClass::DeviceNotActive(s) => s.to_string(),
            QmpErrorClass::DeviceNotFound(s) => s.to_string(),
            QmpErrorClass::KVMMissingCap(s) => s.to_string(),
        }
    }
}

/// A enum to store all command struct
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "execute")]
pub enum QmpCommand {
    #[serde(rename = "qmp_capabilities")]
    qmp_capabilities {
        #[serde(default)]
        arguments: qmp_capabilities,
    },
    quit {
        #[serde(default)]
        arguments: quit,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        id: Option<u32>,
    },
    stop {
        #[serde(default)]
        arguments: stop,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        id: Option<u32>,
    },
    cont {
        #[serde(default)]
        arguments: cont,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        id: Option<u32>,
    },
    device_add {
        arguments: device_add,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        id: Option<u32>,
    },
    device_del {
        arguments: device_del,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        id: Option<u32>,
    },
    netdev_add {
        arguments: netdev_add,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        id: Option<u32>,
    },
    netdev_del {
        arguments: netdev_del,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        id: Option<u32>,
    },
    #[serde(rename = "query-hotpluggable-cpus")]
    query_hotpluggable_cpus {
        #[serde(default)]
        arguments: query_hotpluggable_cpus,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        id: Option<u32>,
    },
    #[serde(rename = "query-cpus")]
    query_cpus {
        #[serde(default)]
        arguments: query_cpus,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        id: Option<u32>,
    },
    #[serde(rename = "query-status")]
    query_status {
        #[serde(default)]
        arguments: query_status,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        id: Option<u32>,
    },
    getfd {
        arguments: getfd,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        id: Option<u32>,
    },
    #[serde(rename = "blockdev-add")]
    blockdev_add {
        arguments: blockdev_add,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        id: Option<u32>,
    },
    #[serde(rename = "blockdev-del")]
    blockdev_del {
        arguments: blockdev_del,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        id: Option<u32>,
    },
}

/// qmp_capabilities
///
/// Enable QMP capabilities.
///
/// # Examples
///
/// ```text
/// -> { "execute": "qmp_capabilities" }
/// <- { "return": {} }
/// ```
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct qmp_capabilities {}

impl Command for qmp_capabilities {
    const NAME: &'static str = "qmp_capabilities";
    type Res = Empty;

    fn back(self) -> Empty {
        Default::default()
    }
}

/// quit
///
/// This command will cause the StratoVirt process to exit gracefully. While every
/// attempt is made to send the QMP response before terminating, this is not
/// guaranteed.  When using this interface, a premature EOF would not be
/// unexpected.
///
/// # Examples
///
/// ```text
/// -> { "execute": "quit" }
/// <- { "return": {}}
/// ```
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct quit {}

impl Command for quit {
    const NAME: &'static str = "quit";
    type Res = Empty;

    fn back(self) -> Empty {
        Default::default()
    }
}

/// stop
///
/// Stop all guest VCPU execution
///
/// # Examples
///
/// ```text
/// -> { "execute": "stop" }
/// <- { "return": {} }
/// ```
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct stop {}

impl Command for stop {
    const NAME: &'static str = "stop";
    type Res = Empty;

    fn back(self) -> Empty {
        Default::default()
    }
}

/// cont
///
/// Resume guest VCPU execution.
///
/// # Examples
///
/// ```text
/// -> { "execute": "cont" }
/// <- { "return": {} }
/// ```
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct cont {}

impl Command for cont {
    const NAME: &'static str = "cont";
    type Res = Empty;

    fn back(self) -> Empty {
        Default::default()
    }
}

/// device_add
///
/// # Arguments
///
/// * `id` - the device's ID, must be unique.
/// * `driver` - the name of the new device's driver.
/// * `addr` - the address device insert into.
///
/// Additional arguments depend on the type.
///
/// # Examples
///
/// ```text
/// -> { "execute": "device_add",
///      "arguments": { "id": "net-0", "driver": "virtio-net-mmio", "addr": "0x0"}}
/// <- { "return": {} }
/// ```
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct device_add {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "driver")]
    pub driver: String,
    #[serde(rename = "addr")]
    pub addr: Option<String>,
    #[serde(rename = "lun")]
    pub lun: Option<usize>,
}

impl Command for device_add {
    const NAME: &'static str = "device_add";
    type Res = Empty;

    fn back(self) -> Empty {
        Default::default()
    }
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct FileOptions {
    pub driver: String,
    pub filename: String,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct CacheOptions {
    #[serde(rename = "no-flush")]
    pub no_flush: Option<bool>,
    pub direct: Option<bool>,
}

/// blockdev_add
///
/// # Arguments
///
/// * `node_name` - the device's ID, must be unique.
/// * `file` - the backend file information.
/// * `cache` - if use direct io.
/// * `read_only` - if readonly.
///
/// Additional arguments depend on the type.
///
/// # Examples
///
/// ```text
/// -> { "execute": "blockdev_add",
///      "arguments":  {"node-name": "drive-0",
///                     "file": {"driver": "file", "filename": "/path/to/block"},
///                     "cache": {"direct": true}, "read-only": false }}
/// <- { "return": {} }
/// ```
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct blockdev_add {
    #[serde(rename = "node-name")]
    pub node_name: String,
    pub file: FileOptions,
    pub cache: Option<CacheOptions>,
    #[serde(rename = "read-only")]
    pub read_only: Option<bool>,
}

impl Command for blockdev_add {
    const NAME: &'static str = "blockdev-add";
    type Res = Empty;

    fn back(self) -> Empty {
        Default::default()
    }
}

/// netdev_add
///
/// # Arguments
///
/// * `id` - the device's ID, must be unique.
/// * `ifname` - the backend tap dev name.
/// * `fds` - the file fd opened by upper level.
///
/// Additional arguments depend on the type.
///
/// # Examples
///
/// ```text
/// -> { "execute": "netdev_add",
///      "arguments":  {"id": "net-0", "ifname": "tap0", "fds": 123 }}
/// <- { "return": {} }
/// ```
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct netdev_add {
    pub id: String,
    #[serde(rename = "ifname")]
    pub if_name: Option<String>,
    pub fds: Option<String>,
}

impl Command for netdev_add {
    const NAME: &'static str = "netdev_add";
    type Res = Empty;

    fn back(self) -> Empty {
        Default::default()
    }
}

/// device_del
///
/// Remove a device from a guest
///
/// # Arguments
///
/// * `id` - the device's ID or QOM path.
///
/// # Errors
///
/// If `id` is not a valid device, DeviceNotFound.
///
/// # Notes
///
/// When this command completes, the device may not be removed from the
/// guest. Hot removal is an operation that requires guest cooperation.
/// This command merely requests that the guest begin the hot removal
/// process. Completion of the device removal process is signaled with a
/// DEVICE_DELETED event. Guest reset will automatically complete removal
/// for all devices.
///
/// # Examples
///
/// ```text
/// -> { "execute": "device_del",
///      "arguments": { "id": "net-0" } }
/// <- { "return": {} }
/// ```
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct device_del {
    pub id: String,
}

impl Command for device_del {
    const NAME: &'static str = "device_del";
    type Res = Empty;

    fn back(self) -> Empty {
        Default::default()
    }
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct blockdev_del {
    #[serde(rename = "node-name")]
    pub node_name: String,
}

impl Command for blockdev_del {
    const NAME: &'static str = "blockdev-del";
    type Res = Empty;

    fn back(self) -> Empty {
        Default::default()
    }
}

/// netdev_del
///
/// Remove a network backend.
///
/// # Arguments
///
/// * `id` - The name of the network backend to remove.
///
/// # Errors
///
/// If `id` is not a valid network backend, DeviceNotFound
///
/// # Examples
///
/// ```text
/// -> { "execute": "netdev_del", "arguments": { "id": "net-0" } }
/// <- { "return": {} }
/// ```
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct netdev_del {
    pub id: String,
}

impl Command for netdev_del {
    const NAME: &'static str = "netdev_del";
    type Res = Empty;

    fn back(self) -> Empty {
        Default::default()
    }
}

/// query-hotpluggable-cpus:
///
/// # Returns
///
/// A list of Hotpluggable CPU objects.
///
/// # Examples
///
/// For pc machine type started with -smp 1,maxcpus=2:
/// ```text
/// -> { "execute": "query-hotpluggable-cpus" }
/// <- {"return": [
///      {
///         "type": "qemu64-x86_64-cpu", "vcpus-count": 1,
///         "props": {"core-id": 0, "socket-id": 1, "thread-id": 0}
///      },
///      {
///         "qom-path": "/machine/unattached/device[0]",
///         "type": "qemu64-x86_64-cpu", "vcpus-count": 1,
///         "props": {"core-id": 0, "socket-id": 0, "thread-id": 0}
///      }
///    ]}
/// ```
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct query_hotpluggable_cpus {}

impl Command for query_hotpluggable_cpus {
    const NAME: &'static str = "query-hotpluggable-cpus";
    type Res = Vec<HotpluggableCPU>;

    fn back(self) -> Vec<HotpluggableCPU> {
        Default::default()
    }
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct HotpluggableCPU {
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(rename = "vcpus-count")]
    pub vcpus_count: isize,
    #[serde(rename = "props")]
    pub props: CpuInstanceProperties,
    #[serde(rename = "qom-path", default, skip_serializing_if = "Option::is_none")]
    pub qom_path: Option<String>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct CpuInstanceProperties {
    #[serde(rename = "node-id", default, skip_serializing_if = "Option::is_none")]
    pub node_id: Option<isize>,
    #[serde(rename = "socket-id", default, skip_serializing_if = "Option::is_none")]
    pub socket_id: Option<isize>,
    #[serde(rename = "thread-id", default, skip_serializing_if = "Option::is_none")]
    pub thread_id: Option<isize>,
    #[serde(rename = "core-id", default, skip_serializing_if = "Option::is_none")]
    pub core_id: Option<isize>,
}

/// query-cpus:
///
/// This command causes vCPU threads to exit to userspace, which causes
/// a small interruption to guest CPU execution. This will have a negative
/// impact on realtime guests and other latency sensitive guest workloads.
/// It is recommended to use @query-cpus-fast instead of this command to
/// avoid the vCPU interruption.
///
/// # Returns
///
/// A list of information about each virtual CPU.
///
/// # Examples
///
/// ```text
/// -> { "execute": "query-cpus" }
/// <- { "return": [
///          {
///             "CPU":0,
///             "current":true,
///             "halted":false,
///             "qom_path":"/machine/unattached/device[0]",
///             "arch":"x86",
///             "thread_id":3134
///          },
///          {
///             "CPU":1,
///             "current":false,
///             "halted":true,
///             "qom_path":"/machine/unattached/device[2]",
///             "arch":"x86",
///             "thread_id":3135
///          }
///       ]
///    }
/// ```
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct query_cpus {}

impl Command for query_cpus {
    const NAME: &'static str = "query-cpus";
    type Res = Vec<CpuInfo>;

    fn back(self) -> Vec<CpuInfo> {
        Default::default()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "arch")]
pub enum CpuInfo {
    #[serde(rename = "x86")]
    x86 {
        #[serde(rename = "current")]
        current: bool,
        #[serde(rename = "qom_path")]
        qom_path: String,
        #[serde(rename = "halted")]
        halted: bool,
        #[serde(rename = "props", default, skip_serializing_if = "Option::is_none")]
        props: Option<CpuInstanceProperties>,
        #[serde(rename = "CPU")]
        CPU: isize,
        #[serde(rename = "thread_id")]
        thread_id: isize,
        #[serde(flatten)]
        #[serde(rename = "x86")]
        x86: CpuInfoX86,
    },
    #[serde(rename = "arm")]
    Arm {
        #[serde(rename = "current")]
        current: bool,
        #[serde(rename = "qom_path")]
        qom_path: String,
        #[serde(rename = "halted")]
        halted: bool,
        #[serde(rename = "props", default, skip_serializing_if = "Option::is_none")]
        props: Option<CpuInstanceProperties>,
        #[serde(rename = "CPU")]
        CPU: isize,
        #[serde(rename = "thread_id")]
        thread_id: isize,
        #[serde(flatten)]
        #[serde(rename = "Arm")]
        arm: CpuInfoArm,
    },
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct CpuInfoX86 {}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct CpuInfoArm {}

/// query-status
///
/// Query the run status of all VCPUs.
///
/// # Returns
///
/// `StatusInfo` reflecting all VCPUs.
///
/// # Examples
///
/// ```text
/// -> { "execute": "query-status" }
/// <- { "return": { "running": true,
///                  "singlestep": false,
///                  "status": "running" } }
/// ```
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct query_status {}

impl Command for query_status {
    const NAME: &'static str = "query-status";
    type Res = StatusInfo;

    fn back(self) -> StatusInfo {
        Default::default()
    }
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct StatusInfo {
    #[serde(rename = "singlestep")]
    pub singlestep: bool,
    #[serde(rename = "running")]
    pub running: bool,
    #[serde(rename = "status")]
    pub status: RunState,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RunState {
    #[serde(rename = "debug")]
    debug,
    #[serde(rename = "inmigrate")]
    inmigrate,
    #[serde(rename = "internal-error")]
    internal_error,
    #[serde(rename = "io-error")]
    io_error,
    #[serde(rename = "paused")]
    paused,
    #[serde(rename = "postmigrate")]
    postmigrate,
    #[serde(rename = "prelaunch")]
    prelaunch,
    #[serde(rename = "finish-migrate")]
    finish_migrate,
    #[serde(rename = "restore-vm")]
    restore_vm,
    #[serde(rename = "running")]
    running,
    #[serde(rename = "save-vm")]
    save_vm,
    #[serde(rename = "shutdown")]
    shutdown,
    #[serde(rename = "suspended")]
    suspended,
    #[serde(rename = "watchdog")]
    watchdog,
    #[serde(rename = "guest-panicked")]
    guest_panicked,
    #[serde(rename = "colo")]
    colo,
    #[serde(rename = "preconfig")]
    preconfig,
}

impl Default for RunState {
    fn default() -> Self {
        RunState::debug
    }
}

/// getfd
///
/// Receive a file descriptor via SCM rights and assign it a name
///
/// # Arguments
///
/// * `fdname` - File descriptor name.
///
/// # Examples
///
/// ```text
/// -> { "execute": "getfd", "arguments": { "fdname": "fd1" } }
/// <- { "return": {} }
/// ```
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct getfd {
    #[serde(rename = "fdname")]
    pub fd_name: String,
}

impl Command for getfd {
    const NAME: &'static str = "getfd";

    type Res = Empty;

    fn back(self) -> Empty {
        Default::default()
    }
}

/// SHUTDOWN
///
/// Emitted when the virtual machine has shut down, indicating that StratoVirt is
/// about to exit.
///
/// # Notes
///
/// If the command-line option "-no-shutdown" has been specified, StratoVirt
/// will not exit, and a STOP event will eventually follow the SHUTDOWN event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SHUTDOWN {
    /// If true, the shutdown was triggered by a guest request (such as
    /// a guest-initiated ACPI shutdown request or other hardware-specific
    /// action) rather than a host request (such as sending StratoVirt a SIGINT).
    #[serde(rename = "guest")]
    pub guest: bool,
    pub reason: String,
}

impl Event for SHUTDOWN {
    const NAME: &'static str = "SHUTDOWN";
}

/// RESET
///
/// Emitted when the virtual machine is reset
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RESET {
    /// If true, the reset was triggered by a guest request (such as
    /// a guest-initiated ACPI reboot request or other hardware-specific action
    /// ) rather than a host request (such as the QMP command system_reset).
    #[serde(rename = "guest")]
    pub guest: bool,
}

impl Event for RESET {
    const NAME: &'static str = "RESET";
}

/// STOP
///
/// Emitted when the virtual machine is stopped
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct STOP {}

impl Event for STOP {
    const NAME: &'static str = "STOP";
}

/// RESUME
///
/// Emitted when the virtual machine resumes execution
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RESUME {}

impl Event for RESUME {
    const NAME: &'static str = "RESUME";
}

/// DEVICE_DELETED
///
/// Emitted whenever the device removal completion is acknowledged by the guest.
/// At this point, it's safe to reuse the specified device ID. Device removal can
/// be initiated by the guest or by HMP/QMP commands.
///
/// # Examples
///
/// ```text
/// <- { "event": "DEVICE_DELETED",
///      "data": { "device": "virtio-net-mmio-0",
///                "path": "/machine/peripheral/virtio-net-mmio-0" },
///      "timestamp": { "seconds": 1265044230, "microseconds": 450486 } }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DEVICE_DELETED {
    /// Device name.
    #[serde(rename = "device", default, skip_serializing_if = "Option::is_none")]
    pub device: Option<String>,
    /// Device path.
    #[serde(rename = "path")]
    pub path: String,
}

impl Event for DEVICE_DELETED {
    const NAME: &'static str = "DEVICE_DELETED";
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "event")]
pub enum QmpEvent {
    #[serde(rename = "SHUTDOWN")]
    SHUTDOWN {
        data: SHUTDOWN,
        timestamp: TimeStamp,
    },
    #[serde(rename = "RESET")]
    RESET { data: RESET, timestamp: TimeStamp },
    #[serde(rename = "STOP")]
    STOP {
        #[serde(default)]
        data: STOP,
        timestamp: TimeStamp,
    },
    #[serde(rename = "RESUME")]
    RESUME {
        #[serde(default)]
        data: RESUME,
        timestamp: TimeStamp,
    },
    #[serde(rename = "DEVICE_DELETED")]
    DEVICE_DELETED {
        data: DEVICE_DELETED,
        timestamp: TimeStamp,
    },
}
