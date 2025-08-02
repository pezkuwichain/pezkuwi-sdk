use crate::{
    mock::{
        ExtBuilder, Test, Welati, RuntimeOrigin, RuntimeEvent,
        run_to_block, last_event, add_parliament_member
    },
    types::*,
    Error,
    Event as WelatiEvent,
};
use frame_support::{
    assert_noop, assert_ok,
    BoundedVec,
};
use sp_runtime::traits::BadOrigin;

// ===== SEÇİM SİSTEMİ TESTLERİ =====

#[test]
fn initiate_election_works() {
    ExtBuilder::default().build().execute_with(|| {
        assert_ok!(Welati::initiate_election(
            RuntimeOrigin::root(),
            ElectionType::Presidential,
            None,
            None,
        ));

        let expected_event = RuntimeEvent::Welati(WelatiEvent::ElectionStarted {
            election_id: 0,
            election_type: ElectionType::Presidential,
            start_block: 1,
            end_block: 1 + 86_400 + 259_200 + 432_000,
        });
        assert_eq!(last_event(), expected_event);

        assert!(Welati::active_elections(0).is_some());
        assert_eq!(Welati::next_election_id(), 1);
    });
}

#[test]
fn initiate_election_fails_for_non_root() {
    ExtBuilder::default().build().execute_with(|| {
        assert_noop!(
            Welati::initiate_election(
                RuntimeOrigin::signed(1),
                ElectionType::Presidential,
                None,
                None,
            ),
            BadOrigin
        );
    });
}

#[test]
fn register_candidate_works_for_parliamentary() {
    ExtBuilder::default().build().execute_with(|| {
        assert_ok!(Welati::initiate_election(
            RuntimeOrigin::root(),
            ElectionType::Parliamentary,
            None,
            None,
        ));

        let parliamentary_endorsers: Vec<u64> = (2..=51).collect();
        
        assert_ok!(Welati::register_candidate(
            RuntimeOrigin::signed(1),
            0,
            None,
            parliamentary_endorsers,
        ));

        assert_eq!(
            last_event(),
            RuntimeEvent::Welati(WelatiEvent::CandidateRegistered {
                election_id: 0,
                candidate: 1,
                deposit_paid: 10_000,
            })
        );

        assert!(Welati::election_candidates(0, 1).is_some());
    });
}

#[test]
fn register_candidate_fails_insufficient_endorsements() {
    ExtBuilder::default().build().execute_with(|| {
        assert_ok!(Welati::initiate_election(
            RuntimeOrigin::root(),
            ElectionType::Presidential,
            None,
            None,
        ));

        let endorsers = vec![2, 3, 4];
        
        assert_noop!(
            Welati::register_candidate(
                RuntimeOrigin::signed(1),
                0,
                None,
                endorsers,
            ),
            Error::<Test>::InsufficientEndorsements
        );
    });
}

#[test]
fn register_candidate_fails_after_deadline() {
    ExtBuilder::default().build().execute_with(|| {
        assert_ok!(Welati::initiate_election(
            RuntimeOrigin::root(),
            ElectionType::Parliamentary,
            None,
            None,
        ));

        run_to_block(86_400 + 100);

        let endorsers: Vec<u64> = (2..=51).collect();
        
        assert_noop!(
            Welati::register_candidate(
                RuntimeOrigin::signed(1),
                0,
                None,
                endorsers,
            ),
            Error::<Test>::CandidacyPeriodExpired
        );
    });
}

#[test]
fn register_candidate_fails_already_candidate() {
    ExtBuilder::default().build().execute_with(|| {
        assert_ok!(Welati::initiate_election(
            RuntimeOrigin::root(),
            ElectionType::Parliamentary,
            None,
            None,
        ));

        let endorsers: Vec<u64> = (2..=51).collect();
        
        assert_ok!(Welati::register_candidate(
            RuntimeOrigin::signed(1),
            0,
            None,
            endorsers.clone(),
        ));

        assert_noop!(
            Welati::register_candidate(
                RuntimeOrigin::signed(1),
                0,
                None,
                endorsers,
            ),
            Error::<Test>::AlreadyCandidate
        );
    });
}

#[test]
fn cast_vote_works() {
    ExtBuilder::default().build().execute_with(|| {
        // 1. Seçimi başlat
        assert_ok!(Welati::initiate_election(
            RuntimeOrigin::root(),
            ElectionType::Parliamentary,
            None,
            None,
        ));

        // 2. Bir aday kaydet (hesap 1)
        let endorsers: Vec<u64> = (3..=52).collect(); // 50 destekçi
        assert_ok!(Welati::register_candidate(
            RuntimeOrigin::signed(1), // Aday
            0,                        // Seçim ID'si
            None,                     // Bölge ID'si
            endorsers,
        ));

        // 3. Oy verme periyoduna ilerle
        run_to_block(86_400 + 259_200 + 1);

        // 4. Oy kullan (hesap 2, aday 1'e oy veriyor)
        let candidates_to_vote_for = vec![1]; 
        assert_ok!(Welati::cast_vote(
            RuntimeOrigin::signed(2),       // Seçmen
            0,                              // Seçim ID'si
            candidates_to_vote_for.clone(), // Oy verilen aday(lar)
            None,                           // Bölge ID'si
        ));

        // 5. Event'i ve depolama durumunu doğrula
        assert_eq!(
            last_event(),
            RuntimeEvent::Welati(WelatiEvent::VoteCast {
                election_id: 0,
                voter: 2,
                candidates: candidates_to_vote_for,
                district_id: None,
            })
        );
        assert!(Welati::election_votes(0, 2).is_some());
    });
}

#[test]
fn cast_vote_fails_already_voted() {
    ExtBuilder::default().build().execute_with(|| {
        assert_ok!(Welati::initiate_election(
            RuntimeOrigin::root(),
            ElectionType::Parliamentary,
            None,
            None,
        ));

        let endorsers: Vec<u64> = (3..=52).collect();
        assert_ok!(Welati::register_candidate(
            RuntimeOrigin::signed(1),
            0,
            None,
            endorsers,
        ));

        run_to_block(86_400 + 259_200 + 1);

        let candidates = vec![1];
        
        assert_ok!(Welati::cast_vote(
            RuntimeOrigin::signed(2),
            0,
            candidates.clone(),
            None,
        ));

        assert_noop!(
            Welati::cast_vote(
                RuntimeOrigin::signed(2),
                0,
                candidates,
                None,
            ),
            Error::<Test>::AlreadyVoted
        );
    });
}

#[test]
fn cast_vote_fails_wrong_period() {
    ExtBuilder::default().build().execute_with(|| {
        assert_ok!(Welati::initiate_election(
            RuntimeOrigin::root(),
            ElectionType::Parliamentary,
            None,
            None,
        ));

        let candidates = vec![1];
        
        assert_noop!(
            Welati::cast_vote(
                RuntimeOrigin::signed(2),
                0,
                candidates,
                None,
            ),
            Error::<Test>::VotingPeriodNotStarted
        );
    });
}

#[test]
fn finalize_election_works() {
    ExtBuilder::default().build().execute_with(|| {
        assert_ok!(Welati::initiate_election(
            RuntimeOrigin::root(),
            ElectionType::Parliamentary,
            None,
            None,
        ));

        // Seçimin bitiş tarihinden sonrasına geç
        // candidacy (86_400) + campaign (259_200) + voting (432_000) + 1
        run_to_block(86_400 + 259_200 + 432_000 + 10); // Ekstra güvenlik için +10

        assert_ok!(Welati::finalize_election(
            RuntimeOrigin::root(),
            0,
        ));

        if let Some(election) = Welati::active_elections(0) {
            assert_eq!(election.status, ElectionStatus::Completed);
        }
    });
}

// ===== ATAMA SİSTEMİ TESTLERİ =====

#[test]
fn nominate_official_works() {
    ExtBuilder::default().build().execute_with(|| {
        let justification = b"Qualified candidate".to_vec().try_into().unwrap();
        
        assert_ok!(Welati::nominate_official(
            RuntimeOrigin::signed(1),
            2,
            OfficialRole::Dadger,
            justification,
        ));

        assert_eq!(Welati::next_appointment_id(), 1);
    });
}

#[test]
fn approve_appointment_works() {
    ExtBuilder::default().build().execute_with(|| {
        let justification = b"Qualified candidate".to_vec().try_into().unwrap();
        
        assert_ok!(Welati::nominate_official(
            RuntimeOrigin::signed(1),
            2,
            OfficialRole::Dadger,
            justification,
        ));

        assert_ok!(Welati::approve_appointment(
            RuntimeOrigin::signed(1),
            0,
        ));
    });
}

// ===== KOLLEKTİF KARAR TESTLERİ =====

#[test]
fn submit_proposal_works() {
    ExtBuilder::default().build().execute_with(|| {
        let title = b"Test Proposal".to_vec().try_into().unwrap();
        let description = b"Test proposal description".to_vec().try_into().unwrap();

        // CRITICAL FIX: Helper fonksiyonu kullan
        add_parliament_member(1);

        assert_ok!(Welati::submit_proposal(
            RuntimeOrigin::signed(1),
            title,
            description,
            CollectiveDecisionType::ParliamentSimpleMajority,
            ProposalPriority::Normal,
            None,
        ));

        assert_eq!(Welati::next_proposal_id(), 1);
        assert!(Welati::active_proposals(0).is_some());
    });
}

#[test]
fn vote_on_proposal_works() {
    ExtBuilder::default().build().execute_with(|| {
        let title = b"Test Proposal".to_vec().try_into().unwrap();
        let description = b"Test proposal description".to_vec().try_into().unwrap();

        // CRITICAL FIX: Helper fonksiyonları kullan
        add_parliament_member(1);
        add_parliament_member(2);

        assert_ok!(Welati::submit_proposal(
            RuntimeOrigin::signed(1),
            title,
            description,
            CollectiveDecisionType::ParliamentSimpleMajority,
            ProposalPriority::Normal,
            None,
        ));

        let proposal = Welati::active_proposals(0).unwrap();
        run_to_block(proposal.voting_starts_at + 1);

        let rationale = Some(b"Good proposal".to_vec().try_into().unwrap());

        assert_ok!(Welati::vote_on_proposal(
            RuntimeOrigin::signed(2),
            0,
            VoteChoice::Aye,
            rationale,
        ));

        assert!(Welati::collective_votes(0, 2).is_some());
    });
}

// ===== HELPER FUNCTION TESTLERİ =====

#[test]
fn get_required_trust_score_works() {
    ExtBuilder::default().build().execute_with(|| {
        assert_eq!(
            Welati::get_required_trust_score(&ElectionType::Presidential),
            600
        );
        
        assert_eq!(
            Welati::get_required_trust_score(&ElectionType::Parliamentary),
            300
        );
        
        assert_eq!(
            Welati::get_required_trust_score(&ElectionType::ConstitutionalCourt),
            750
        );
    });
}

#[test]
fn get_required_endorsements_works() {
    ExtBuilder::default().build().execute_with(|| {
        assert_eq!(
            Welati::get_required_endorsements(&ElectionType::Presidential),
            100
        );
        
        assert_eq!(
            Welati::get_required_endorsements(&ElectionType::Parliamentary),
            50
        );
        
        assert_eq!(
            Welati::get_required_endorsements(&ElectionType::SpeakerElection),
            0
        );
    });
}

#[test]
fn get_minimum_turnout_works() {
    ExtBuilder::default().build().execute_with(|| {
        assert_eq!(
            Welati::get_minimum_turnout(&ElectionType::Presidential),
            50
        );
        
        assert_eq!(
            Welati::get_minimum_turnout(&ElectionType::Parliamentary),
            40
        );
        
        assert_eq!(
            Welati::get_minimum_turnout(&ElectionType::SpeakerElection),
            30
        );
    });
}

#[test]
fn calculate_vote_weight_works() {
    ExtBuilder::default().build().execute_with(|| {
        assert_eq!(
            Welati::calculate_vote_weight(&1, &ElectionType::Presidential),
            1
        );
        
        assert_eq!(
            Welati::calculate_vote_weight(&1, &ElectionType::Parliamentary),
            1
        );
        
        let weight = Welati::calculate_vote_weight(&1, &ElectionType::SpeakerElection);
        assert!(weight >= 1 && weight <= 10);
    });
}

// ===== ERROR CASE TESTLERİ =====

#[test]
fn election_not_found_error_works() {
    ExtBuilder::default().build().execute_with(|| {
        assert_noop!(
            Welati::register_candidate(
                RuntimeOrigin::signed(1),
                999,
                None,
                vec![2, 3],
            ),
            Error::<Test>::ElectionNotFound
        );
    });
}

#[test]
fn proposal_not_found_error_works() {
    ExtBuilder::default().build().execute_with(|| {
        assert_noop!(
            Welati::vote_on_proposal(
                RuntimeOrigin::signed(1),
                999,
                VoteChoice::Aye,
                None,
            ),
            Error::<Test>::ProposalNotFound
        );
    });
}

// ===== INTEGRATION TESTLERİ =====

#[test]
fn complete_election_cycle_works() {
    ExtBuilder::default().build().execute_with(|| {
        // 1. Seçim başlat
        assert_ok!(Welati::initiate_election(
            RuntimeOrigin::root(),
            ElectionType::Parliamentary,
            None,
            None,
        ));

        // 2. Adaylar kaydolsun
        let endorsers1: Vec<u64> = (10..=59).collect();
        let endorsers2: Vec<u64> = (60..=109).collect();
        
        assert_ok!(Welati::register_candidate(
            RuntimeOrigin::signed(1),
            0,
            None,
            endorsers1,
        ));
        
        assert_ok!(Welati::register_candidate(
            RuntimeOrigin::signed(2),
            0,
            None,
            endorsers2,
        ));

        // 3. Voting period'a geç
        run_to_block(86_400 + 259_200 + 1);

        // 4. Oylar kullanılsın
        assert_ok!(Welati::cast_vote(
            RuntimeOrigin::signed(3),
            0,
            vec![1],
            None,
        ));
        
        assert_ok!(Welati::cast_vote(
            RuntimeOrigin::signed(4),
            0,
            vec![2],
            None,
        ));

        // 5. Seçimi sonlandır
        run_to_block(86_400 + 259_200 + 432_000 + 2);
        
        assert_ok!(Welati::finalize_election(
            RuntimeOrigin::root(),
            0,
        ));

        assert!(Welati::election_results(0).is_some());
    });
}

#[test]
fn complete_appointment_cycle_works() {
    ExtBuilder::default().build().execute_with(|| {
        let justification = b"Experienced lawyer".to_vec().try_into().unwrap();
        
        assert_ok!(Welati::nominate_official(
            RuntimeOrigin::signed(1),
            5,
            OfficialRole::Dadger,
            justification,
        ));

        assert_ok!(Welati::approve_appointment(
            RuntimeOrigin::signed(1),
            0,
        ));

        if let Some(process) = Welati::appointment_processes(0) {
            assert_eq!(process.status, AppointmentStatus::Approved);
        }
    });
}

#[test]
fn complete_proposal_cycle_works() {
    ExtBuilder::default().build().execute_with(|| {
        let title = b"Budget Amendment".to_vec().try_into().unwrap();
        let description = b"Increase education budget by 10%".to_vec().try_into().unwrap();

        // CRITICAL FIX: Helper fonksiyonları kullan
        add_parliament_member(1);
        add_parliament_member(2);
        add_parliament_member(3);

        assert_ok!(Welati::submit_proposal(
            RuntimeOrigin::signed(1),
            title,
            description,
            CollectiveDecisionType::ParliamentSimpleMajority,
            ProposalPriority::High,
            None,
        ));

        let proposal = Welati::active_proposals(0).unwrap();
        run_to_block(proposal.voting_starts_at + 1);

        assert_ok!(Welati::vote_on_proposal(
            RuntimeOrigin::signed(2),
            0,
            VoteChoice::Aye,
            None,
        ));

        assert_ok!(Welati::vote_on_proposal(
            RuntimeOrigin::signed(3),
            0,
            VoteChoice::Aye,
            None,
        ));

        if let Some(proposal) = Welati::active_proposals(0) {
            assert_eq!(proposal.aye_votes, 2);
        }
    });
}

// ===== RUNOFF ELECTION TESTLERİ =====

#[test]
fn initiate_runoff_election_works() {
    ExtBuilder::default().build().execute_with(|| {
        let runoff_candidates: BoundedVec<u64, _> = vec![1, 2].try_into().unwrap();
        
        assert_ok!(Welati::initiate_election(
            RuntimeOrigin::root(),
            ElectionType::Presidential,
            None,
            Some(runoff_candidates),
        ));

        assert!(Welati::active_elections(0).is_some());
        assert!(Welati::election_candidates(0, 1).is_some());
        assert!(Welati::election_candidates(0, 2).is_some());
        
        if let Some(election) = Welati::active_elections(0) {
            assert_eq!(election.status, ElectionStatus::CampaignPeriod);
        }
    });
}

#[test]
fn runoff_election_fails_with_wrong_candidate_count() {
    ExtBuilder::default().build().execute_with(|| {
        let invalid_candidates: Result<BoundedVec<u64, _>, _> = vec![1, 2, 3].try_into();
        
        if let Ok(candidates) = invalid_candidates {
            assert_noop!(
                Welati::initiate_election(
                    RuntimeOrigin::root(),
                    ElectionType::Presidential,
                    None,
                    Some(candidates),
                ),
                Error::<Test>::InvalidInitialCandidates
            );
        }
    });
}

#[test]
fn runoff_election_fails_for_non_presidential() {
    ExtBuilder::default().build().execute_with(|| {
        let runoff_candidates: BoundedVec<u64, _> = vec![1, 2].try_into().unwrap();
        
        assert_noop!(
            Welati::initiate_election(
                RuntimeOrigin::root(),
                ElectionType::Parliamentary,
                None,
                Some(runoff_candidates),
            ),
            Error::<Test>::InvalidElectionType
        );
    });
}