//! Group membership API.
use rdsys;
use rdsys::types::*;

use std::ffi::CStr;
use std::slice;
use std::fmt;

/// Group member information container.
pub struct GroupMemberInfo(RDKafkaGroupMemberInfo);

impl GroupMemberInfo {
    /// Return the id of the member.
    pub fn id(&self) -> &str {
        unsafe {
            CStr::from_ptr(self.0.member_id).to_str()
                .expect("Member id is not a valid UTF-8 string")
        }
    }

    /// Return the client id of the member.
    pub fn client_id(&self) -> &str {
        unsafe {
            CStr::from_ptr(self.0.client_id).to_str()
                .expect("Client id is not a valid UTF-8 string")
        }
    }

    /// Return the client host of the member.
    pub fn client_host(&self) -> &str {
        unsafe {
            CStr::from_ptr(self.0.client_host).to_str()
                .expect("Member host is not a valid UTF-8 string")
        }
    }
}

/// Group information container.
pub struct GroupInfo(RDKafkaGroupInfo);

impl GroupInfo {
    /// Return the name of the group.
    pub fn name(&self) -> &str {
        unsafe {
            CStr::from_ptr(self.0.group).to_str()
                .expect("Group name is not a valid UTF-8 string")
        }
    }

    /// Returns the members of the group.
    pub fn members(&self) -> &[GroupMemberInfo] {
        unsafe { slice::from_raw_parts(self.0.members as *const GroupMemberInfo, self.0.member_cnt as usize) }
    }

    /// Returns the state of the group.
    pub fn state(&self) -> &str {
        unsafe {
            CStr::from_ptr(self.0.state).to_str()
                .expect("State is not a valid UTF-8 string")
        }
    }

    /// Returns the protocol of the group.
    pub fn protocol(&self) -> &str {
        unsafe {
            CStr::from_ptr(self.0.protocol).to_str()
                .expect("Protocol name is not a valid UTF-8 string")
        }
    }

    /// Returns the protocol type of the group.
    pub fn protocol_type(&self) -> &str {
        unsafe {
            CStr::from_ptr(self.0.protocol_type).to_str()
                .expect("Protocol type is not a valid UTF-8 string")
        }
    }
}

impl fmt::Debug for GroupInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

/// List of groups. This structure wraps the  pointer returned by rdkafka-sys, and deallocates all
/// the native resources when dropped.
pub struct GroupList(*const RDKafkaGroupList);

impl GroupList {
    /// Creates a new group list given a pointer to the native rdkafka-sys group list structure.
    pub fn from_ptr(ptr: *const RDKafkaGroupList) -> GroupList {
        GroupList(ptr)
    }

    /// Returns all the groups in the list.
    pub fn groups(&self) -> &[GroupInfo] {
        unsafe { slice::from_raw_parts((*self.0).groups as *const GroupInfo, (*self.0).group_cnt as usize) }
    }
}

impl Drop for GroupList {
    fn drop(&mut self) {
        unsafe { rdsys::rd_kafka_group_list_destroy(self.0) };
    }
}
