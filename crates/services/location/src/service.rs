use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::watch::{DynReceiver, Receiver, Watch};
use raylar_drivers::gps::GpsFix;

use crate::{LocationConfig, LocationFilter, LocationState};

pub const DEFAULT_LOCATION_WATCHERS: usize = 4;
pub const DEFAULT_LOCATION_HISTORY: usize = 9;

pub type LocationMutex = CriticalSectionRawMutex;
pub type LocationStateReceiver<'a, const WATCHERS: usize> =
    Receiver<'a, LocationMutex, LocationState, WATCHERS>;

pub struct LocationResources<const WATCHERS: usize = DEFAULT_LOCATION_WATCHERS> {
    state: Watch<LocationMutex, LocationState, WATCHERS>,
}

impl<const WATCHERS: usize> LocationResources<WATCHERS> {
    pub const fn new() -> Self {
        Self {
            state: Watch::new_with(LocationState::invalid()),
        }
    }

    pub fn state_receiver(&self) -> Option<LocationStateReceiver<'_, WATCHERS>> {
        self.state.receiver()
    }

    pub fn state(&self) -> LocationState {
        self.state.try_get().unwrap_or_else(LocationState::invalid)
    }
}

impl<const WATCHERS: usize> Default for LocationResources<WATCHERS> {
    fn default() -> Self {
        Self::new()
    }
}

pub struct LocationService<
    const WATCHERS: usize = DEFAULT_LOCATION_WATCHERS,
    const HISTORY: usize = DEFAULT_LOCATION_HISTORY,
> {
    resources: &'static LocationResources<WATCHERS>,
    gps_fixes: DynReceiver<'static, GpsFix>,
    filter: LocationFilter<HISTORY>,
}

impl<const WATCHERS: usize, const HISTORY: usize> LocationService<WATCHERS, HISTORY> {
    pub const fn new(
        resources: &'static LocationResources<WATCHERS>,
        gps_fixes: DynReceiver<'static, GpsFix>,
        config: LocationConfig,
    ) -> Self {
        Self {
            resources,
            gps_fixes,
            filter: LocationFilter::new(config),
        }
    }

    pub fn state(&self) -> LocationState {
        self.resources.state()
    }

    pub fn watch(&self) -> Option<LocationStateReceiver<'_, WATCHERS>> {
        self.resources.state_receiver()
    }

    pub async fn run(mut self) -> ! {
        let state = self.resources.state.sender();
        state.send(self.filter.ingest(self.gps_fixes.get().await));

        loop {
            let fix = self.gps_fixes.changed().await;
            state.send(self.filter.ingest(fix));

            while let Some(fix) = self.gps_fixes.try_changed() {
                state.send(self.filter.ingest(fix));
            }
        }
    }
}
