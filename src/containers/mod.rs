//! Containers from https://github.com/ethereum/consensus-specs/blob/dev/specs/altair/light-client/sync-protocol.md

use crate::constants::CURRENT_SYNC_COMMITTEE_INDEX_FLOOR_LOG2;

pub type Bytes32 = [u8; 32];
// todos:
pub type SyncCommittee = usize;
pub type BeaconBlockHeader = usize;
pub type SyncAggregate = usize;
pub type Slot = usize;

// todo: implement Container, other types from where?
#[derive(Debug, Clone)]
pub struct LightClientHeader {
  beacon: BeaconBlockHeader,
}

// derive container
pub struct LightClientBootstrap {
  /// Header matching the requested beacon block root
  header: LightClientHeader,
  /// Sync committee corresponding to the header state root
  current_sync_committee: SyncCommittee,
  current_sync_committee_branch: Vec<Bytes32>,
}

// derive Container
pub struct LightClientUpdate {
  next_sync_committee:        SyncCommittee,
  next_sync_committee_branch: Vec<Bytes32>,
  attested_header:            LightClientHeader,
  finalized_header:           LightClientHeader,
  finality_branch:            Vec<Bytes32>,
  sync_aggregate:             SyncAggregate,
  signature_slot:             Slot,
}

// container
pub struct LightClientFinalityUpdate {
  attested_header:  LightClientHeader,
  finalized_header: LightClientHeader,
  finality_branch:  Vec<Bytes32>,
  sync_aggregate:   SyncAggregate,
  signature_slot:   Slot,
}

// container
pub struct LightClientOptimisticUpdate {
  attested_header: LightClientHeader,
  sync_aggregate:  SyncAggregate,
  signature_slot:  Slot,
}

// dataclass, object
pub struct LightClientStore {
  finalized_header:                 LightClientHeader,
  current_sync_committee:           SyncCommittee,
  next_sync_committee:              SyncCommittee,
  best_valid_update:                Option<LightClientUpdate>,
  optimistic_header:                LightClientHeader,
  previous_max_active_participants: u64,
  current_max_active_participants:  u64,
}

impl LightClientHeader {
  pub fn is_valid_light_client_header(&self) -> bool {
    true
  }
}

impl LightClientUpdate {
  pub fn is_sync_committee_update(&self) -> bool {
    todo!()
    // self.next_sync_committee_branch != Bytes32::default() 
  }
}