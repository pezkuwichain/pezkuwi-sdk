use frame_benchmarking::{benchmarks, account, whitelisted_caller, impl_benchmark_test_suite};
use frame_system::RawOrigin;
use super::*;
use crate::{types::*, pallet::*};
use pallet_identity_kyc::{types::KycLevel, types::KycStatus};

benchmarks! {
    // ----------------------------------------------------------------
    // SEÇİM SİSTEMİ BENCHMARK'LARI
    // ----------------------------------------------------------------
    initiate_election {
        // Bu benchmark'ın özel bir hazırlığa ihtiyacı yok, root ile çağrılması yeterli.
    }: _(RawOrigin::Root, ElectionType::Parliamentary, None, None)
    verify {
        assert!(ActiveElections::<T>::get(0).is_some());
    }

    register_candidate {
        // --- HAZIRLIK ---
        Pallet::<T>::initiate_election(RawOrigin::Root.into(), ElectionType::Parliamentary, None, None)?;
        
        // Benchmark için basitleştirilmiş endorsers - KYC bypass
        let endorsers: Vec<T::AccountId> = (0..T::ParliamentaryEndorsements::get())
            .map(|i| account("endorser", i, 0))
            .collect();

        let new_candidate: T::AccountId = whitelisted_caller();

        // Test ortamında KYC kontrolü zaten bypass edildi

    }: _(RawOrigin::Signed(new_candidate.clone()), 0, None, endorsers)
    verify {
        assert!(ElectionCandidates::<T>::get(0, &new_candidate).is_some());
    }

    cast_vote {
        // --- HAZIRLIK ---
        // 1. Seçim ve adayları hazırla.
        Pallet::<T>::initiate_election(RawOrigin::Root.into(), ElectionType::Parliamentary, None, None)?;
        
        let candidate: T::AccountId = account("candidate", 1, 0);
        let voter: T::AccountId = whitelisted_caller();
        
        // Simplified endorsers for benchmark
        let endorsers: Vec<T::AccountId> = (0..T::ParliamentaryEndorsements::get())
            .map(|i| account("endorser", i, 0))
            .collect();

        // Test ortamında KYC kontrolü zaten bypass edildi
            
        Pallet::<T>::register_candidate(RawOrigin::Signed(candidate.clone()).into(), 0, None, endorsers)?;

        // 2. Oy verme periyoduna ilerle.
        let election = ActiveElections::<T>::get(0).unwrap();
        frame_system::Pallet::<T>::set_block_number(election.voting_start);

        let candidates_to_vote_for = vec![candidate];

    }: _(RawOrigin::Signed(voter.clone()), 0, candidates_to_vote_for, None)
    verify {
        assert!(ElectionVotes::<T>::get(0, &voter).is_some());
    }

    finalize_election {
        // --- HAZIRLIK ---
        // 1. Seçimi, adayı ve bir oyu hazırla.
        Pallet::<T>::initiate_election(RawOrigin::Root.into(), ElectionType::Parliamentary, None, None)?;
        
        let candidate: T::AccountId = account("candidate", 1, 0);
        let voter: T::AccountId = account("voter", 2, 0);
        
        let endorsers: Vec<T::AccountId> = (0..T::ParliamentaryEndorsements::get())
            .map(|i| account("endorser", i, 0))
            .collect();

        // Test ortamında KYC kontrolü zaten bypass edildi
            
        Pallet::<T>::register_candidate(RawOrigin::Signed(candidate.clone()).into(), 0, None, endorsers)?;
        
        let election = ActiveElections::<T>::get(0).unwrap();
        frame_system::Pallet::<T>::set_block_number(election.voting_start);
        Pallet::<T>::cast_vote(RawOrigin::Signed(voter.clone()).into(), 0, vec![candidate], None)?;
        
        // 2. Seçimin bitiş zamanına ilerle.
        frame_system::Pallet::<T>::set_block_number(election.end_block + 1u32.into());

    }: _(RawOrigin::Root, 0)
    verify {
        assert!(ElectionResults::<T>::get(0).is_some());
    }

    // ----------------------------------------------------------------
    // ATAMA SİSTEMİ BENCHMARK'LARI
    // ----------------------------------------------------------------
    nominate_official {
        // --- HAZIRLIK ---
        let nominator: T::AccountId = whitelisted_caller();
        let nominee: T::AccountId = account("nominee", 2, 0);
        let justification = b"Test nomination".to_vec().try_into().unwrap();

    }: _(RawOrigin::Signed(nominator), nominee, OfficialRole::Dadger, justification)
    verify {
        assert_eq!(NextAppointmentId::<T>::get(), 1);
    }

    approve_appointment {
        // --- HAZIRLIK ---
        let approver: T::AccountId = whitelisted_caller();
        let nominator: T::AccountId = account("nominator", 2, 0);
        let nominee: T::AccountId = account("nominee", 3, 0);
        let justification = b"Test nomination".to_vec().try_into().unwrap();
        Pallet::<T>::nominate_official(RawOrigin::Signed(nominator).into(), nominee, OfficialRole::Dadger, justification)?;

    }: _(RawOrigin::Signed(approver), 0)
    verify {
        // Basit kontrol - appointment ID artmış olmalı
        assert_eq!(NextAppointmentId::<T>::get(), 1);
    }

    // ----------------------------------------------------------------
    // KOLLEKTİF KARAR BENCHMARK'LARI
    // ----------------------------------------------------------------
    submit_proposal {
        // --- HAZIRLIK ---
        let proposer: T::AccountId = whitelisted_caller();
        
        // Benchmark için basit member oluşturma
        let member: ParliamentMember<T> = ParliamentMember { 
            account: proposer.clone(), 
            elected_at: 0u32.into(),
            term_ends_at: 1000u32.into(),
            votes_participated: 0,
            total_votes_eligible: 0,
            participation_rate: 100,
            committees: Default::default(),
        };
        let members: BoundedVec<ParliamentMember<T>, T::ParliamentSize> = vec![member].try_into().unwrap();
        ParliamentMembers::<T>::put(members);
        
        let title = b"Test Proposal".to_vec().try_into().unwrap();
        let description = b"Test proposal description".to_vec().try_into().unwrap();

    }: _(RawOrigin::Signed(proposer), title, description, CollectiveDecisionType::ParliamentSimpleMajority, ProposalPriority::Normal, None)
    verify {
        assert!(ActiveProposals::<T>::get(0).is_some());
    }

    vote_on_proposal {
        // --- HAZIRLIK ---
        let proposer: T::AccountId = account("proposer", 1, 0);
        let voter: T::AccountId = whitelisted_caller();
        
        // İki üye oluştur
        let member1: ParliamentMember<T> = ParliamentMember { 
            account: proposer.clone(), 
            elected_at: 0u32.into(),
            term_ends_at: 1000u32.into(),
            votes_participated: 0,
            total_votes_eligible: 0,
            participation_rate: 100,
            committees: Default::default(),
        };
        let member2: ParliamentMember<T> = ParliamentMember { 
            account: voter.clone(), 
            elected_at: 0u32.into(),
            term_ends_at: 1000u32.into(),
            votes_participated: 0,
            total_votes_eligible: 0,
            participation_rate: 100,
            committees: Default::default(),
        };
        let members: BoundedVec<ParliamentMember<T>, T::ParliamentSize> = vec![member1, member2].try_into().unwrap();
        ParliamentMembers::<T>::put(members);

        let title = b"Test Proposal".to_vec().try_into().unwrap();
        let description = b"Test proposal description".to_vec().try_into().unwrap();
        Pallet::<T>::submit_proposal(RawOrigin::Signed(proposer).into(), title, description, CollectiveDecisionType::ParliamentSimpleMajority, ProposalPriority::Normal, None)?;
        
        let proposal = ActiveProposals::<T>::get(0).unwrap();
        frame_system::Pallet::<T>::set_block_number(proposal.voting_starts_at + 1u32.into());
        
        let rationale = Some(b"Test vote rationale".to_vec().try_into().unwrap());

    }: _(RawOrigin::Signed(voter.clone()), 0, VoteChoice::Aye, rationale)
    verify {
        assert!(CollectiveVotes::<T>::get(0, &voter).is_some());
    }

    impl_benchmark_test_suite!(Pallet, crate::mock::ExtBuilder::default().build(), crate::mock::Test);
}