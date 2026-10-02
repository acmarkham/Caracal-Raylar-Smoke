use embassy_time::{Duration, Instant};

use crate::{LinkError, NodeId};

use super::{ChannelProfile, ProfileBand};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LinkTarget {
    Broadcast,
    Gateway,
    Peer(NodeId),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LinkPurpose {
    BootstrapBroadcast,
    FastData,
    ReliableData,
    LowEnergyData,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BandMask(u8);

impl BandMask {
    pub const SUB_GHZ: Self = Self(1 << 0);
    pub const GHZ_2_4: Self = Self(1 << 1);
    pub const ALL: Self = Self(Self::SUB_GHZ.0 | Self::GHZ_2_4.0);

    pub const fn contains(self, band: ProfileBand) -> bool {
        let bit = match band {
            ProfileBand::SubGhz => Self::SUB_GHZ.0,
            ProfileBand::Ghz2_4 => Self::GHZ_2_4.0,
        };
        self.0 & bit != 0
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LinkConstraints {
    pub allowed_bands: BandMask,
    pub maximum_airtime: Option<Duration>,
}

impl Default for LinkConstraints {
    fn default() -> Self {
        Self {
            allowed_bands: BandMask::ALL,
            maximum_airtime: None,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LinkRequest {
    pub target: LinkTarget,
    pub purpose: LinkPurpose,
    pub constraints: LinkConstraints,
    pub system_slot: Option<Instant>,
}

pub trait LinkEstimator {
    fn select_profile(&self, request: &LinkRequest) -> Result<ChannelProfile, LinkError>;
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StaticLinkEstimator {
    broadcast: ChannelProfile,
    default_data: ChannelProfile,
}

impl StaticLinkEstimator {
    pub const fn new(broadcast: ChannelProfile, default_data: ChannelProfile) -> Self {
        Self {
            broadcast,
            default_data,
        }
    }

    pub const fn broadcast_profile(&self) -> &ChannelProfile {
        &self.broadcast
    }

    pub fn phase_one_eu868() -> Self {
        let profile = ChannelProfile::phase_one_eu868_bootstrap();
        Self::new(profile.clone(), profile)
    }
}

impl LinkEstimator for StaticLinkEstimator {
    fn select_profile(&self, request: &LinkRequest) -> Result<ChannelProfile, LinkError> {
        let selected = if request.purpose == LinkPurpose::BootstrapBroadcast
            || request.target == LinkTarget::Broadcast
        {
            &self.broadcast
        } else {
            &self.default_data
        };
        if !request.constraints.allowed_bands.contains(selected.band()) {
            return Err(LinkError::NoAcceptableProfile);
        }
        // Phase I has no calibrated airtime model. A zero maximum is known to
        // be impossible; non-zero bounds are retained for a later estimator.
        if request
            .constraints
            .maximum_airtime
            .is_some_and(|duration| duration.as_ticks() == 0)
        {
            return Err(LinkError::NoAcceptableProfile);
        }
        Ok(selected.clone())
    }
}
