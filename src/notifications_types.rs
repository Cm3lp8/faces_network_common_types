use bincode::{Decode, Encode};
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Decode, Encode, Debug)]
pub struct NotifAccrossNodes {
    notification_id: [u8; 16],
    creation_ts: i64,
    kind: NotifAcrossKind,
}

#[derive(Debug, Clone, Encode, Decode)]
pub enum NotifAcrossKind {
    NewInvitation {
        direction: String,
        invitation_uuid: [u8; 16],
        inviter_id: [u8; 16],
        inviter_name: String,
        invitee_name: String,
        invitee_id: [u8; 16],
        ts: i64,
        accepted: String,
    },
}
impl NotifAccrossNodes {
    pub fn new(notification_id: Uuid, creation_ts: DateTime<Utc>, kind: NotifAcrossKind) -> Self {
        Self {
            notification_id: notification_id.into_bytes(),
            creation_ts: creation_ts.timestamp(),
            kind,
        }
    }
    pub fn get_notification_id(&self) -> Uuid {
        Uuid::from_bytes(self.notification_id)
    }
    pub fn creation_ts(&self) -> Option<DateTime<Utc>> {
        DateTime::from_timestamp(self.creation_ts, 0)
    }
    pub fn kind(&self) -> &NotifAcrossKind {
        &self.kind
    }
}
impl NotifAcrossKind {
    pub fn new_invitation_notif(
        direction: &str,
        invitation_uuid: Uuid,
        inviter_id: Uuid,
        inviter_name: &str,
        invitee_name: &str,
        invitee_id: Uuid,
        timestamp: DateTime<Utc>,
        accepted: &str,
    ) -> Result<Self, String> {
        check_contract_on_direction(direction)?;
        check_contract_on_accepted(accepted)?;
        Ok(Self::NewInvitation {
            direction: direction.to_string(),
            invitation_uuid: invitation_uuid.into_bytes(),
            inviter_id: inviter_id.into_bytes(),
            inviter_name: inviter_name.to_string(),
            invitee_name: invitee_name.to_string(),
            invitee_id: invitee_id.into_bytes(),
            ts: timestamp.timestamp(),
            accepted: accepted.to_string(),
        })
    }

    pub fn get_direction(&self) -> &str {
        match self {
            Self::NewInvitation {
                direction,
                invitation_uuid: _,
                inviter_id: _,
                inviter_name: _,
                invitee_name: _,
                invitee_id: _,
                ts: _,
                accepted: _,
            } => direction.as_str(),
        }
    }
    pub fn get_inviter_id(&self) -> Uuid {
        match self {
            Self::NewInvitation {
                direction: _,
                invitation_uuid: _,
                inviter_id,
                inviter_name: _,
                invitee_name: _,
                invitee_id: _,
                ts: _,
                accepted: _,
            } => Uuid::from_bytes(*inviter_id),
        }
    }
    pub fn get_invitee_id(&self) -> Uuid {
        match self {
            Self::NewInvitation {
                direction: _,
                invitation_uuid: _,
                inviter_id: _,
                inviter_name: _,
                invitee_name: _,
                invitee_id,
                ts: _,
                accepted: _,
            } => Uuid::from_bytes(*invitee_id),
        }
    }
    pub fn get_invitation_id(&self) -> Uuid {
        match self {
            Self::NewInvitation {
                direction: _,
                invitation_uuid,
                inviter_id: _,
                inviter_name: _,
                invitee_name: _,
                invitee_id: _,
                ts: _,
                accepted: _,
            } => Uuid::from_bytes(*invitation_uuid),
        }
    }
    pub fn get_inviter_name(&self) -> &str {
        match self {
            Self::NewInvitation {
                direction: _,
                invitation_uuid: _,
                inviter_id: _,
                inviter_name,
                invitee_name: _,
                invitee_id: _,
                ts: _,
                accepted: _,
            } => inviter_name.as_str(),
        }
    }
    pub fn get_invitee_name(&self) -> &str {
        match self {
            Self::NewInvitation {
                direction: _,
                invitation_uuid: _,
                inviter_id: _,
                inviter_name: _,
                invitee_name,
                invitee_id: _,
                ts: _,
                accepted: _,
            } => invitee_name.as_str(),
        }
    }
    pub fn get_acceptation_status(&self) -> &str {
        match self {
            Self::NewInvitation {
                direction: _,
                invitation_uuid: _,
                inviter_id: _,
                inviter_name: _,
                invitee_name: _,
                invitee_id: _,
                ts: _,
                accepted,
            } => accepted.as_str(),
        }
    }
    pub fn get_timestamp(&self) -> Option<DateTime<Utc>> {
        match self {
            Self::NewInvitation {
                direction: _,
                invitation_uuid: _,
                inviter_id: _,
                inviter_name: _,
                invitee_name: _,
                invitee_id: _,
                ts,
                accepted: _,
            } => DateTime::from_timestamp(*ts, 0),
        }
    }
}

fn check_contract_on_accepted(accepted: &str) -> Result<(), String> {
    match accepted {
        "pending" => Ok(()),
        "accepted" => Ok(()),
        "refused" => Ok(()),
        _ => Err("check contract on accepted error : Wrong value".to_owned()),
    }
}
fn check_contract_on_direction(direction: &str) -> Result<(), String> {
    match direction {
        "incoming" => Ok(()),
        "outgoing" => Ok(()),
        _ => Err("check contract on direction error : Wrong value".to_owned()),
    }
}
