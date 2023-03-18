//! Constants for the Ethereum Altair Light Client.
pub(crate) const FINALIZED_ROOT_INDEX: usize =
  get_generalized_index(BeaconState::FinalizedCheckpoint("root".to_owned()));
pub(crate) const CURRENT_SYNC_COMMITTEE_INDEX: usize =
  get_generalized_index(BeaconState::CurrentSyncCommittee);
pub(crate) const CURRENT_SYNC_COMMITTEE_INDEX_FLOOR_LOG2: usize =
  CURRENT_SYNC_COMMITTEE_INDEX.checked_ilog2().unwrap() as usize;

pub(crate) const NEXT_SYNC_COMMITTEE_INDEX: usize =
  get_generalized_index(BeaconState::NextSyncCommittee);

pub(crate) const MIN_SYNC_COMMITTEE_PARTICIPANTS: usize = 1;
pub(crate) const UPDATE_TIMEOUT: usize = SLOTS_PER_EPOCH * EPOCHS_PER_SYNC_COMMITTEE_PERIOD;

enum BeaconState {
  FinalizedCheckpoint(String),
  NextSyncCommittee,
  CurrentSyncCommittee,
}

const fn get_generalized_index(state: BeaconState) -> usize {
  match state {
    BeaconState::FinalizedCheckpoint(_root) => 105,
    BeaconState::NextSyncCommittee => 54,
    BeaconState::CurrentSyncCommittee => 55,
  }
}
