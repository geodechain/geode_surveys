/*
ABOUT THIS CONTRACT...
This contract offers a way for users to offer surveys that pay users
to repond. You can choose from a 1, 3 or 5 question survey to offer.  
*/

#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod surveys {

    use ink::prelude::vec::Vec;
    use ink::storage::Mapping;
    use ink::env::hash::{Sha2x256, HashOutput};


    // PRELIMINARY DATA STRUCTURES >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>

    #[derive(Clone, Debug, PartialEq, Eq)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(feature = "std",derive(ink::storage::traits::StorageLayout,))]
    pub struct Survey5 {
        owner: AccountId,
        survey_id: Hash,
        survey_name: Vec<u8>,
        length: u8,
        public: bool,
        open: bool,
        survey_balance: Balance,
        payout: Balance,
        max_responses: u128,
        total_responses: u128,
        q1: Vec <u8>,
        q2: Vec <u8>,
        q3: Vec <u8>,
        q4: Vec <u8>,
        q5: Vec <u8>,
    }
    
    impl Default for Survey5 {
        fn default() -> Survey5 {
            Survey5 {
                owner: AccountId::from([0x0; 32]),
                survey_id: Hash::default(),
                survey_name: <Vec<u8>>::default(),
                length: u8::default(),
                public: bool::default(),
                open: bool::default(),
                survey_balance: Balance::default(),
                payout: Balance::default(),
                max_responses: u128::default(),
                total_responses: u128::default(),
                q1: <Vec<u8>>::default(),
                q2: <Vec<u8>>::default(),
                q3: <Vec<u8>>::default(),
                q4: <Vec<u8>>::default(),
                q5: <Vec<u8>>::default(),
            }
        }
    }

    #[derive(Clone, Debug, PartialEq, Eq)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(feature = "std",derive(ink::storage::traits::StorageLayout,))]
    pub struct Survey3 {
        owner: AccountId,
        survey_id: Hash,
        survey_name: Vec<u8>,
        length: u8,
        public: bool,
        open: bool,
        survey_balance: Balance,
        payout: Balance,
        max_responses: u128,
        total_responses: u128,
        q1: Vec <u8>,
        q2: Vec <u8>,
        q3: Vec <u8>,
    }
    
    impl Default for Survey3 {
        fn default() -> Survey3 {
            Survey3 {
                owner: AccountId::from([0x0; 32]),
                survey_id: Hash::default(),
                survey_name: <Vec<u8>>::default(),
                length: u8::default(),
                public: bool::default(),
                open: bool::default(),
                survey_balance: Balance::default(),
                payout: Balance::default(),
                max_responses: u128::default(),
                total_responses: u128::default(),
                q1: <Vec<u8>>::default(),
                q2: <Vec<u8>>::default(),
                q3: <Vec<u8>>::default(),
            }
        }
    }


    #[derive(Clone, Debug, PartialEq, Eq)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(feature = "std",derive(ink::storage::traits::StorageLayout,))]
    pub struct Survey1 {
        owner: AccountId,
        survey_id: Hash,
        survey_name: Vec<u8>,
        length: u8,
        public: bool,
        open: bool,
        survey_balance: Balance,
        payout: Balance,
        max_responses: u128,
        total_responses: u128,
        q1: Vec <u8>,
    }
    
    impl Default for Survey1 {
        fn default() -> Survey1 {
            Survey1 {
                owner: AccountId::from([0x0; 32]),
                survey_id: Hash::default(),
                survey_name: <Vec<u8>>::default(),
                length: u8::default(),
                public: bool::default(),
                open: bool::default(),
                survey_balance: Balance::default(),
                payout: Balance::default(),
                max_responses: u128::default(),
                total_responses: u128::default(),
                q1: <Vec<u8>>::default(),
            }
        }
    }


    #[derive(Clone, Debug, PartialEq, Eq)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(feature = "std",derive(ink::storage::traits::StorageLayout,))]
    pub struct Response5 {
        survey_id: Hash,
        response_id: Hash,
        timestamp: u64,
        respondent: AccountId,
        ip_address: Vec<u8>,
        a1: Vec<u8>,
        a2: Vec<u8>,
        a3: Vec<u8>,
        a4: Vec<u8>,
        a5: Vec<u8>,
    }
    
    impl Default for Response5 {
        fn default() -> Response5 {
            Response5 {
                survey_id: Hash::default(),
                response_id: Hash::default(),
                timestamp: u64::default(),
                respondent: AccountId::from([0x0; 32]),
                ip_address: <Vec<u8>>::default(),
                a1: <Vec<u8>>::default(),
                a2: <Vec<u8>>::default(),
                a3: <Vec<u8>>::default(),
                a4: <Vec<u8>>::default(),
                a5: <Vec<u8>>::default(),
            }
        }
    }


    #[derive(Clone, Debug, PartialEq, Eq)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(feature = "std",derive(ink::storage::traits::StorageLayout,))]
    pub struct Response3 {
        survey_id: Hash,
        response_id: Hash,
        timestamp: u64,
        respondent: AccountId,
        ip_address: Vec<u8>,
        a1: Vec<u8>,
        a2: Vec<u8>,
        a3: Vec<u8>,
    }
    
    impl Default for Response3 {
        fn default() -> Response3 {
            Response3 {
                survey_id: Hash::default(),
                response_id: Hash::default(),
                timestamp: u64::default(),
                respondent: AccountId::from([0x0; 32]),
                ip_address: <Vec<u8>>::default(),
                a1: <Vec<u8>>::default(),
                a2: <Vec<u8>>::default(),
                a3: <Vec<u8>>::default(),
            }
        }
    }


    #[derive(Clone, Debug, PartialEq, Eq)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(feature = "std",derive(ink::storage::traits::StorageLayout,))]
    pub struct Response1 {
        survey_id: Hash,
        response_id: Hash,
        timestamp: u64,
        respondent: AccountId,
        ip_address: Vec<u8>,
        a1: Vec<u8>,
    }
    
    impl Default for Response1 {
        fn default() -> Response1 {
            Response1 {
                survey_id: Hash::default(),
                response_id: Hash::default(),
                timestamp: u64::default(),
                respondent: AccountId::from([0x0; 32]),
                ip_address: <Vec<u8>>::default(),
                a1: <Vec<u8>>::default(),
            }
        }
    }

    #[derive(Clone, Debug, PartialEq, Eq)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(feature = "std",derive(ink::storage::traits::StorageLayout,))]
    pub struct Available {
        surveys1: Vec<Survey1>,
        surveys3: Vec<Survey3>,
        surveys5: Vec<Survey5>,
    }
    
    impl Default for Available {
        fn default() -> Available {
            Available {
                surveys1: <Vec<Survey1>>::default(),
                surveys3: <Vec<Survey3>>::default(),
                surveys5: <Vec<Survey5>>::default(),
            }
        }
    }




    // EVENT DEFINITIONS >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>
    // event that someone has made a new survey
    #[ink(event)]
    pub struct NewSurvey {
        #[ink(topic)]
        owner: AccountId,
        #[ink(topic)]
        survey_id: Hash,
        #[ink(topic)]
        survey_name: Vec<u8>,
        length: u8,
        payout: Balance,
        max_responses: u128, 
    }

    // event that someone has filled out a survey
    #[ink(event)]
    pub struct NewResponse {
        #[ink(topic)]
        survey_id: Hash,
        #[ink(topic)]
        response_id: Hash,
        timestamp: u64,
        #[ink(topic)]
        respondent: AccountId,
        payout: Balance,
    }

    #[ink(event)]
    pub struct SurveyClosed {
        #[ink(topic)]
        owner: AccountId,
        #[ink(topic)]
        survey_id: Hash,
        #[ink(topic)]
        survey_name: Vec<u8>,
    }


    // ERROR DEFINITIONS >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>

    // Errors that can occur upon calling this contract
    #[derive(Debug, PartialEq, Eq)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    pub enum Error {
        // User made a new survey but did not send enough coin or survey is out of coin to pay
        NotEnoughBalance,
        // User tried to repond but has already taken this survey
        DuplicateResponse,
        // Survey questions or answers were too big (too many characters)
        DataTooLarge,
        // Tried to close a survey that is not theirs
        AccessDenied,
        // When a payout fails to send
        PayoutFailed,
        // Tried to close a survey using a length (1, 3 or 5) that does not match the survey
        WrongSurveyLength,
        // (new errors are added at the end so the existing error numbers do not change)
        // The contract is paused by the root user
        ContractPaused,
        // The contract has been permanently shut down by the root user
        ContractShutDown,
        // Only the root user can call this message
        NotRoot,
    }

    // ACTUAL CONTRACT STORAGE >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>
    #[ink(storage)]
    pub struct ContractStorage {
        map_1q_id_to_survey: Mapping<Hash, Survey1>,
        map_3q_id_to_survey: Mapping<Hash, Survey3>,
        map_5q_id_to_survey: Mapping<Hash, Survey5>,
        map_1q_id_to_response: Mapping<Hash, Response1>,
        map_3q_id_to_response: Mapping<Hash, Response3>,
        map_5q_id_to_response: Mapping<Hash, Response5>,
        map_surveyid_responseid: Mapping<(Hash,u128), Hash>,
        map_survey_index: Mapping<Hash, u128>,
        map_account_responses: Mapping<(AccountId,Hash), u8>,
        map_account_surveys: Mapping<AccountId, Vec<Hash>>,
        top_surveys1: Vec<Hash>,
        top_surveys3: Vec<Hash>,
        top_surveys5: Vec<Hash>,
        map_survey_ip_count: Mapping<(Hash,Vec<u8>), u8>,
        // (survey owner, ip address) -> 1 if that owner has banned that ip from their surveys
        map_owner_banned_ip: Mapping<(AccountId,Vec<u8>), u8>,
        // the root user (set to the account that instantiates the contract)
        root: AccountId,
        // when true, no one can send messages that change storage (root can still unpause)
        paused: bool,
        // when true, the contract is permanently shut down (cannot be undone)
        shut_down: bool,
    }

    // CONTRACT LOGIC >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>

    impl ContractStorage {
        
        // CONSTRUCTORS >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>
        // Constructors are implicitly payable when the contract is instantiated.

        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                map_1q_id_to_survey: Mapping::default(),
                map_3q_id_to_survey: Mapping::default(),
                map_5q_id_to_survey: Mapping::default(),
                map_1q_id_to_response: Mapping::default(),
                map_3q_id_to_response: Mapping::default(),
                map_5q_id_to_response: Mapping::default(),
                map_surveyid_responseid: Mapping::default(),
                map_survey_index: Mapping::default(),
                map_account_responses: Mapping::default(),
                map_account_surveys: Mapping::default(),
                top_surveys1: <Vec<Hash>>::default(),
                top_surveys3: <Vec<Hash>>::default(),
                top_surveys5: <Vec<Hash>>::default(),
                map_survey_ip_count: Mapping::default(),
                map_owner_banned_ip: Mapping::default(),
                root: Self::env().caller(),
                paused: false,
                shut_down: false,
            }
        }

        // HELPERS (internal only, not callable from outside the contract) >>>>>>>>>>>>>>>>

        // a survey is "live" if it is open, can still pay a respondent,
        // and has not reached its max responses
        fn is_live(open: bool, survey_balance: Balance, payout: Balance,
            total_responses: u128, max_responses: u128) -> bool {
            open && payout > 0 && survey_balance >= payout && total_responses < max_responses
        }

        // remove a survey id from a top surveys list (does nothing if it is not there)
        fn remove_from_list(list: &mut Vec<Hash>, survey_id: &Hash) {
            list.retain(|id| id != survey_id);
        }

        // stop any storage-changing message while the contract is paused or shut down
        fn ensure_active(&self) -> Result<(), Error> {
            if self.shut_down {
                return Err(Error::ContractShutDown);
            }
            if self.paused {
                return Err(Error::ContractPaused);
            }
            Ok(())
        }

        // has this survey's owner banned this ip address from their surveys?
        fn is_banned(&self, owner: AccountId, ip_address: &[u8]) -> bool {
            self.map_owner_banned_ip.contains((owner, ip_address.to_vec()))
        }

        // MESSAGES THAT ALTER CONTRACT STORAGE >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>

        // 0 🟢 Make A New Survey with 5 Questions
        #[ink(message, payable)]
        pub fn make_a_new_survey_5(&mut self, 
            survey_name: Vec<u8>,
            public: bool,
            payout: Balance,
            max_responses: u128,
            question_1: Vec <u8>,
            question_2: Vec <u8>,
            question_3: Vec <u8>,
            question_4: Vec <u8>,
            question_5: Vec <u8>,
        ) -> Result<(), Error> {

            let caller = Self::env().caller();

            // get caller's current list of surveys
            // no new surveys while the contract is paused or shut down
            self.ensure_active()?;

            let mut mysurveys = self.map_account_surveys.get(caller).unwrap_or_default();

            // get the payment from the caller
            // the 'payable' tag on this message allows the user to send any amount
            // so we determine here if they send enough
            let staked: Balance = self.env().transferred_value();
            let minstake: Balance = payout.saturating_mul(max_responses);

            // if the data is too big or the payment too small, send an error
            if survey_name.len() > 600 || question_1.len() > 300 || question_2.len() > 300 
            || question_3.len() > 300 || question_4.len() > 300 || question_5.len() > 300 
            || payout == 0 || max_responses == 0 
            || staked < minstake || mysurveys.len() > 399 {
                return Err(Error::DataTooLarge);
            }
            else { 

                // make the survey id hash 
                // set up the data that will go into the survey_id
                let time = self.env().block_timestamp();
                // create the survey_id by hashing the above data
                let encodable = (caller, time, survey_name.clone()); // Implements `scale::Encode`
                let mut new_survey_id_u8 = <Sha2x256 as HashOutput>::Type::default(); // 256-bit buffer
                ink::env::hash_encoded::<Sha2x256, _>(&encodable, &mut new_survey_id_u8);
                let new_survey_id: Hash = Hash::from(new_survey_id_u8);

                // create the survey
                let new_survey = Survey5 {
                    owner: caller,
                    survey_id: new_survey_id,
                    survey_name: survey_name.clone(),
                    length: 5,
                    public: public,
                    open: true,
                    survey_balance: staked,
                    payout: payout,
                    max_responses: max_responses,
                    total_responses: 0,
                    q1: question_1,
                    q2: question_2,
                    q3: question_3,
                    q4: question_4,
                    q5: question_5,
                };
                
                // UPDATE STORAGE 
                
                // update map_5q_id_to_survey
                if self.map_5q_id_to_survey.try_insert(&new_survey_id, &new_survey).is_err() {
                    return Err(Error::DataTooLarge);
                }

                // update map_survey_index
                let init_index: u128 = 0;
                if self.map_survey_index.try_insert(&new_survey_id, &init_index).is_err() {
                    return Err(Error::DataTooLarge);
                }

                // update map_account_surveys
                // add this survey to the caller's list
                mysurveys.push(new_survey_id);
                if self.map_account_surveys.try_insert(&caller, &mysurveys).is_err() {
                    return Err(Error::DataTooLarge);
                }

                // update list of 100 top paying surveys
                // if the list is full, first drop any surveys that are used up
                // (out of balance, max responses reached, or closed) to make room
                if self.top_surveys5.len() >= 100 {
                    let map = &self.map_5q_id_to_survey;
                    self.top_surveys5.retain(|id| {
                        let s = map.get(id).unwrap_or_default();
                        Self::is_live(s.open, s.survey_balance, s.payout, s.total_responses, s.max_responses)
                    });
                }
                // get the list of top paying surveys
                let leaderboard = &self.top_surveys5;
                // if there are fewer than 100 surveys in the list, just add this one
                // otherwise, compete by comparing each payout to this payout
                if leaderboard.len() < 100 {
                    // add this survey directly
                    self.top_surveys5.push(new_survey_id);
                }
                else {
                    // if the leaderboard is full, find the survey with the lowest payout
                    let lowest_hash = leaderboard[0];
                    let mut lowest_payout: Balance = self.map_5q_id_to_survey.get(lowest_hash).unwrap_or_default().payout;
                    let mut low_index: usize = 0;
                    for (i, id) in leaderboard.iter().enumerate() {
                        // get the payout and index
                        let bid: Balance = self.map_5q_id_to_survey.get(id).unwrap_or_default().payout;
                        if bid < lowest_payout { 
                            lowest_payout = bid;
                            low_index = i;
                        }
                    }
                    // compare the lowest payout with this new survey payout
                    if payout > lowest_payout {
                        // if the new survey payout is higher...
                        // kick out the low bidder 
                        self.top_surveys5.remove(low_index);
                        // and add the new survey to the leaderboard
                        self.top_surveys5.push(new_survey_id);
                    }
                    // if the new survey payout is lower, do nothing
                }

                // EMIT AN EVENT IF THE SURVEY IS PUBLIC
                if public == true {
                    self.env().emit_event(NewSurvey {
                        owner: caller,
                        survey_id: new_survey_id,
                        survey_name: survey_name,
                        length: 5,
                        payout: payout,
                        max_responses: max_responses,  
                    });
                }
            }
            Ok(())
        }

        // 1 🟢 Make A New Survey with 3 Questions
        #[ink(message, payable)]
        pub fn make_a_new_survey_3(&mut self, 
            survey_name: Vec<u8>,
            public: bool,
            payout: Balance,
            max_responses: u128,
            question_1: Vec <u8>,
            question_2: Vec <u8>,
            question_3: Vec <u8>,
        ) -> Result<(), Error> {

            let caller = Self::env().caller();

            // get caller's current list of surveys
            // no new surveys while the contract is paused or shut down
            self.ensure_active()?;

            let mut mysurveys = self.map_account_surveys.get(caller).unwrap_or_default();

            // get the payment from the caller
            // the 'payable' tag on this message allows the user to send any amount
            // so we determine here if they send enough
            let staked: Balance = self.env().transferred_value();
            let minstake: Balance = payout.saturating_mul(max_responses);

            // if the data is too big or the payment too small, send an error
            if survey_name.len() > 600 || question_1.len() > 300 || question_2.len() > 300 
            || payout == 0 || max_responses == 0 
            || question_3.len() > 300 || staked < minstake || mysurveys.len() > 399 {
                return Err(Error::DataTooLarge);
            }
            else { 

                // make the survey id hash 
                // set up the data that will go into the survey_id
                let time = self.env().block_timestamp();
                // create the survey_id by hashing the above data
                let encodable = (caller, time, survey_name.clone()); // Implements `scale::Encode`
                let mut new_survey_id_u8 = <Sha2x256 as HashOutput>::Type::default(); // 256-bit buffer
                ink::env::hash_encoded::<Sha2x256, _>(&encodable, &mut new_survey_id_u8);
                let new_survey_id: Hash = Hash::from(new_survey_id_u8);

                // create the survey
                let new_survey = Survey3 {
                    owner: caller,
                    survey_id: new_survey_id,
                    survey_name: survey_name.clone(),
                    length: 3,
                    public: public,
                    open: true,
                    survey_balance: staked,
                    payout: payout,
                    max_responses: max_responses,
                    total_responses: 0,
                    q1: question_1,
                    q2: question_2,
                    q3: question_3,
                };
                
                // UPDATE STORAGE 
                
                // update map_3q_id_to_survey
                if self.map_3q_id_to_survey.try_insert(&new_survey_id, &new_survey).is_err() {
                    return Err(Error::DataTooLarge);
                }

                // update map_survey_index
                let init_index: u128 = 0;
                if self.map_survey_index.try_insert(&new_survey_id, &init_index).is_err() {
                    return Err(Error::DataTooLarge);
                }

                // update map_account_surveys
                // add this survey to the caller's list
                mysurveys.push(new_survey_id);
                if self.map_account_surveys.try_insert(&caller, &mysurveys).is_err() {
                    return Err(Error::DataTooLarge);
                }

                // update list of 100 top paying surveys
                // if the list is full, first drop any surveys that are used up
                // (out of balance, max responses reached, or closed) to make room
                if self.top_surveys3.len() >= 100 {
                    let map = &self.map_3q_id_to_survey;
                    self.top_surveys3.retain(|id| {
                        let s = map.get(id).unwrap_or_default();
                        Self::is_live(s.open, s.survey_balance, s.payout, s.total_responses, s.max_responses)
                    });
                }
                // get the list of top paying surveys
                let leaderboard = &self.top_surveys3;
                // if there are fewer than 100 surveys in the list, just add this one
                // otherwise, compete by comparing each payout to this payout
                if leaderboard.len() < 100 {
                    // add this survey directly
                    self.top_surveys3.push(new_survey_id);
                }
                else {
                    // if the leaderboard is full, find the survey with the lowest payout
                    let lowest_hash = leaderboard[0];
                    let mut lowest_payout: Balance = self.map_3q_id_to_survey.get(lowest_hash).unwrap_or_default().payout;
                    let mut low_index: usize = 0;
                    for (i, id) in leaderboard.iter().enumerate() {
                        // get the payout and index
                        let bid: Balance = self.map_3q_id_to_survey.get(id).unwrap_or_default().payout;
                        if bid < lowest_payout { 
                            lowest_payout = bid;
                            low_index = i;
                        }
                    }
                    // compare the lowest payout with this new survey payout
                    if payout > lowest_payout {
                        // if the new survey payout is higher...
                        // kick out the low bidder 
                        self.top_surveys3.remove(low_index);
                        // and add the new survey to the leaderboard
                        self.top_surveys3.push(new_survey_id);
                    }
                    // if the new survey payout is lower, do nothing
                }

                // EMIT AN EVENT IF THE SURVEY IS PUBLIC
                if public == true {
                    self.env().emit_event(NewSurvey {
                        owner: caller,
                        survey_id: new_survey_id,
                        survey_name: survey_name,
                        length: 3,
                        payout: payout,
                        max_responses: max_responses,  
                    });
                }
            }
            Ok(())
        }

        // 2 🟢 Make A New Survey with 1 Question
        #[ink(message, payable)]
        pub fn make_a_new_survey_1(&mut self, 
            survey_name: Vec<u8>,
            public: bool,
            payout: Balance,
            max_responses: u128,
            question_1: Vec <u8>,
        ) -> Result<(), Error> {

            let caller = Self::env().caller();

            // get caller's current list of surveys
            // no new surveys while the contract is paused or shut down
            self.ensure_active()?;

            let mut mysurveys = self.map_account_surveys.get(caller).unwrap_or_default();

            // get the payment from the caller
            // the 'payable' tag on this message allows the user to send any amount
            // so we determine here if they send enough
            let staked: Balance = self.env().transferred_value();
            let minstake: Balance = payout.saturating_mul(max_responses);

            // if the data is too big or the payment too small, send an error
            if survey_name.len() > 600 || question_1.len() > 300 
            || payout == 0 || max_responses == 0 
            || staked < minstake || mysurveys.len() > 399 {
                return Err(Error::DataTooLarge);
            }
            else { 

                // make the survey id hash 
                // set up the data that will go into the survey_id
                let time = self.env().block_timestamp();
                // create the survey_id by hashing the above data
                let encodable = (caller, time, survey_name.clone()); // Implements `scale::Encode`
                let mut new_survey_id_u8 = <Sha2x256 as HashOutput>::Type::default(); // 256-bit buffer
                ink::env::hash_encoded::<Sha2x256, _>(&encodable, &mut new_survey_id_u8);
                let new_survey_id: Hash = Hash::from(new_survey_id_u8);

                // create the survey
                let new_survey = Survey1 {
                    owner: caller,
                    survey_id: new_survey_id,
                    survey_name: survey_name.clone(),
                    length: 1,
                    public: public,
                    open: true,
                    survey_balance: staked,
                    payout: payout,
                    max_responses: max_responses,
                    total_responses: 0,
                    q1: question_1,
                };
                
                // UPDATE STORAGE 
                
                // update map_1q_id_to_survey
                if self.map_1q_id_to_survey.try_insert(&new_survey_id, &new_survey).is_err() {
                    return Err(Error::DataTooLarge);
                }

                // update map_survey_index
                let init_index: u128 = 0;
                if self.map_survey_index.try_insert(&new_survey_id, &init_index).is_err() {
                    return Err(Error::DataTooLarge);
                }

                // update map_account_surveys
                // add this survey to the caller's list
                mysurveys.push(new_survey_id);
                if self.map_account_surveys.try_insert(&caller, &mysurveys).is_err() {
                    return Err(Error::DataTooLarge);
                }

                // update list of 100 top paying surveys
                // if the list is full, first drop any surveys that are used up
                // (out of balance, max responses reached, or closed) to make room
                if self.top_surveys1.len() >= 100 {
                    let map = &self.map_1q_id_to_survey;
                    self.top_surveys1.retain(|id| {
                        let s = map.get(id).unwrap_or_default();
                        Self::is_live(s.open, s.survey_balance, s.payout, s.total_responses, s.max_responses)
                    });
                }
                // get the list of top paying surveys
                let leaderboard = &self.top_surveys1;
                // if there are fewer than 100 surveys in the list, just add this one
                // otherwise, compete by comparing each payout to this payout
                if leaderboard.len() < 100 {
                    // add this survey directly
                    self.top_surveys1.push(new_survey_id);
                }
                else {
                    // if the leaderboard is full, find the survey with the lowest payout
                    let lowest_hash = leaderboard[0];
                    let mut lowest_payout: Balance = self.map_1q_id_to_survey.get(lowest_hash).unwrap_or_default().payout;
                    let mut low_index: usize = 0;
                    for (i, id) in leaderboard.iter().enumerate() {
                        // get the payout and index
                        let bid: Balance = self.map_1q_id_to_survey.get(id).unwrap_or_default().payout;
                        if bid < lowest_payout { 
                            lowest_payout = bid;
                            low_index = i;
                        }
                    }
                    // compare the lowest payout with this new survey payout
                    if payout > lowest_payout {
                        // if the new survey payout is higher...
                        // kick out the low bidder 
                        self.top_surveys1.remove(low_index);
                        // and add the new survey to the leaderboard
                        self.top_surveys1.push(new_survey_id);
                    }
                    // if the new survey payout is lower, do nothing
                }

                // EMIT AN EVENT IF THE SURVEY IS PUBLIC
                if public == true {
                    self.env().emit_event(NewSurvey {
                        owner: caller,
                        survey_id: new_survey_id,
                        survey_name: survey_name,
                        length: 1,
                        payout: payout,
                        max_responses: max_responses,  
                    });
                }
            }
            Ok(())
        }

        // 3 🟢 Close My Survey 
        #[ink(message)]
        pub fn close_my_survey(&mut self, 
            survey_id: Hash,
            length: u8,
        ) -> Result<(), Error> {

            // no closing while paused. After a shutdown, closing IS allowed: it is how
            // owners get their leftover coin back once the contract is shut down.
            if self.paused && !self.shut_down {
                return Err(Error::ContractPaused);
            }

            let caller = Self::env().caller();

            // get caller's current list of surveys
            let mysurveys = self.map_account_surveys.get(caller).unwrap_or_default();

            // is the survey in this owner's list? if not, error
            if !mysurveys.contains(&survey_id) {
                return Err(Error::AccessDenied);
            }

            // close the survey...
            // get the survey details, mark it closed, zero its balance, and put it back in storage.
            // If the survey does not exist under this length (1, 3 or 5), return an error
            // instead of writing a blank survey into the wrong map.
            let (paythis, survey_name) = match length {
                1 => {
                    let mut survey = self.map_1q_id_to_survey.get(survey_id)
                        .ok_or(Error::WrongSurveyLength)?;
                    let paythis: Balance = survey.survey_balance;
                    survey.open = false;
                    survey.survey_balance = 0;
                    if self.map_1q_id_to_survey.try_insert(&survey_id, &survey).is_err() {
                        return Err(Error::DataTooLarge);
                    }
                    // remove this survey from the list of 100 top paying surveys (if it is there)
                    Self::remove_from_list(&mut self.top_surveys1, &survey_id);
                    (paythis, survey.survey_name)
                }
                3 => {
                    let mut survey = self.map_3q_id_to_survey.get(survey_id)
                        .ok_or(Error::WrongSurveyLength)?;
                    let paythis: Balance = survey.survey_balance;
                    survey.open = false;
                    survey.survey_balance = 0;
                    if self.map_3q_id_to_survey.try_insert(&survey_id, &survey).is_err() {
                        return Err(Error::DataTooLarge);
                    }
                    Self::remove_from_list(&mut self.top_surveys3, &survey_id);
                    (paythis, survey.survey_name)
                }
                5 => {
                    let mut survey = self.map_5q_id_to_survey.get(survey_id)
                        .ok_or(Error::WrongSurveyLength)?;
                    let paythis: Balance = survey.survey_balance;
                    survey.open = false;
                    survey.survey_balance = 0;
                    if self.map_5q_id_to_survey.try_insert(&survey_id, &survey).is_err() {
                        return Err(Error::DataTooLarge);
                    }
                    Self::remove_from_list(&mut self.top_surveys5, &survey_id);
                    (paythis, survey.survey_name)
                }
                _ => return Err(Error::WrongSurveyLength),
            };

            // pay the remainder back to the owner.
            // If the transfer fails we return an error, which reverts the whole call
            // (so the balance is never zeroed without the owner being refunded).
            // Closing an already-closed survey refunds nothing, because its balance is now 0.
            if paythis > 0 {
                if self.env().transfer(caller, paythis).is_err() {
                    return Err(Error::PayoutFailed);
                }
            }

            // EMIT AN EVENT
            self.env().emit_event(SurveyClosed {
                owner: caller,
                survey_id: survey_id,
                survey_name: survey_name, 
            });

            Ok(())
        }

        // 4 🟢 Take A Survey of 5 Questions
        #[ink(message)]
        pub fn take_a_survey_5(&mut self, 
            survey_id: Hash,
            ip_address: Vec<u8>,
            answer_1: Vec<u8>,
            answer_2: Vec<u8>,
            answer_3: Vec<u8>,
            answer_4: Vec<u8>,
            answer_5: Vec<u8>,
        ) -> Result<(), Error> {

            let caller = Self::env().caller();
            let time = self.env().block_timestamp();

            // no responses while the contract is paused or shut down
            self.ensure_active()?;

            // did the caller already take this survey?
            let repeat = self.map_account_responses.get((caller, survey_id)).unwrap_or_default();

            // how many responses have we had from this ip address?
            let mut ip_count = self.map_survey_ip_count.get((survey_id, ip_address.clone())).unwrap_or_default();

            // get the survey details
            let mut survey = self.map_5q_id_to_survey.get(survey_id).unwrap_or_default();

            // if the data is too big or the user has done this survey before,
            // or there is not enough balance on this survey or the survey is full,
            // or the survey is closed, send an error
            if answer_1.len() > 300 || answer_2.len() > 300 || answer_3.len() > 300 
            || answer_4.len() > 300 || answer_5.len() > 300 
            || repeat==1 || survey.survey_balance < survey.payout
            || survey.total_responses >= survey.max_responses || survey.open==false || ip_count > 3
            || self.is_banned(survey.owner, &ip_address) {
                return Err(Error::AccessDenied);
            }
            else { 

                // create the response_id by hashing the above data
                let encodable = (caller, survey_id); // Implements `scale::Encode`
                let mut new_response_id_u8 = <Sha2x256 as HashOutput>::Type::default(); // 256-bit buffer
                ink::env::hash_encoded::<Sha2x256, _>(&encodable, &mut new_response_id_u8);
                let new_response_id: Hash = Hash::from(new_response_id_u8);

                // create the response
                let new_response = Response5 {
                    survey_id: survey_id,
                    response_id: new_response_id,
                    timestamp: time,
                    respondent: caller,
                    ip_address: ip_address.clone(),
                    a1: answer_1,
                    a2: answer_2,
                    a3: answer_3,
                    a4: answer_4,
                    a5: answer_5,
                };
                
                // UPDATE STORAGE 

                // update the ip count for this survey
                ip_count = ip_count.saturating_add(1);
                if self.map_survey_ip_count.try_insert(&(survey_id,ip_address), &ip_count).is_err() {
                    return Err(Error::DataTooLarge);
                }

                // update survey details to increment response count
                survey.total_responses = survey.total_responses.saturating_add(1);
                survey.survey_balance = survey.survey_balance.saturating_sub(survey.payout);
                if self.map_5q_id_to_survey.try_insert(&survey_id, &survey).is_err() {
                    return Err(Error::DataTooLarge);
                }

                // if this response used up the survey (max responses reached or not enough
                // balance left for another payout), remove it from the top surveys list
                if !Self::is_live(survey.open, survey.survey_balance, survey.payout,
                    survey.total_responses, survey.max_responses) {
                    Self::remove_from_list(&mut self.top_surveys5, &survey_id);
                }
                
                // update map_5q_id_to_response
                if self.map_5q_id_to_response.try_insert(&new_response_id, &new_response).is_err() {
                    return Err(Error::DataTooLarge);
                }

                // update map_surveyid_responseid
                if self.map_surveyid_responseid.try_insert(&(survey_id, survey.total_responses), &new_response_id).is_err() {
                    return Err(Error::DataTooLarge);
                }

                // update map_survey_index
                if self.map_survey_index.try_insert(&survey_id, &survey.total_responses).is_err() {
                    return Err(Error::DataTooLarge);
                }

                // update map_account_responses
                let done: u8 = 1;
                if self.map_account_responses.try_insert(&(caller, survey_id), &done).is_err() {
                    return Err(Error::DataTooLarge);
                }

                // PAY THE RESPONDENT
                let paythis = survey.payout;
                if self.env().balance() > paythis && paythis > 0 {
                    // pay the owner the amount paythis
                    if self.env().transfer(caller, paythis).is_err() {
                        return Err(Error::PayoutFailed);
                    }
                }
                // if the balance is zero, Error (ZeroBalance)
                else {
                    return Err(Error::NotEnoughBalance);
                }

                // EMIT AN EVENT IF THE SURVEY IS PUBLIC
                if survey.public == true {
                    self.env().emit_event(NewResponse {
                        survey_id: survey_id,
                        response_id: new_response_id,
                        timestamp: time,
                        respondent: caller,
                        payout: paythis,  
                    });
                }
            }
            Ok(())
        }

        // 5 🟢 Take A Survey of 3 Questions
        #[ink(message)]
        pub fn take_a_survey_3(&mut self, 
            survey_id: Hash,
            ip_address: Vec<u8>,
            answer_1: Vec<u8>,
            answer_2: Vec<u8>,
            answer_3: Vec<u8>,
        ) -> Result<(), Error> {

            let caller = Self::env().caller();
            let time = self.env().block_timestamp();

            // no responses while the contract is paused or shut down
            self.ensure_active()?;

            // did the caller already take this survey?
            let repeat = self.map_account_responses.get((caller, survey_id)).unwrap_or_default();

            // how many responses have we had from this ip address?
            let mut ip_count = self.map_survey_ip_count.get((survey_id, ip_address.clone())).unwrap_or_default();

            // get the survey details
            let mut survey = self.map_3q_id_to_survey.get(survey_id).unwrap_or_default();

            // if the data is too big or the user has done this survey before,
            // or there is not enough balance on this survey or the survey is full,
            // or the survey is closed, send an error
            if answer_1.len() > 300 || answer_2.len() > 300 || answer_3.len() > 300  
            || repeat==1 || survey.survey_balance < survey.payout
            || survey.total_responses >= survey.max_responses || survey.open==false || ip_count > 3 
            || self.is_banned(survey.owner, &ip_address) {
                return Err(Error::AccessDenied);
            }
            else { 

                // create the response_id by hashing the above data
                let encodable = (caller, survey_id); // Implements `scale::Encode`
                let mut new_response_id_u8 = <Sha2x256 as HashOutput>::Type::default(); // 256-bit buffer
                ink::env::hash_encoded::<Sha2x256, _>(&encodable, &mut new_response_id_u8);
                let new_response_id: Hash = Hash::from(new_response_id_u8);

                // create the response
                let new_response = Response3 {
                    survey_id: survey_id,
                    response_id: new_response_id,
                    timestamp: time,
                    respondent: caller,
                    ip_address: ip_address.clone(),
                    a1: answer_1,
                    a2: answer_2,
                    a3: answer_3,
                };
                
                // UPDATE STORAGE 

                // update the ip count for this survey
                ip_count = ip_count.saturating_add(1);
                if self.map_survey_ip_count.try_insert(&(survey_id,ip_address), &ip_count).is_err() {
                    return Err(Error::DataTooLarge);
                }

                // update survey details to increment response count
                survey.total_responses = survey.total_responses.saturating_add(1);
                survey.survey_balance = survey.survey_balance.saturating_sub(survey.payout);
                if self.map_3q_id_to_survey.try_insert(&survey_id, &survey).is_err() {
                    return Err(Error::DataTooLarge);
                }

                // if this response used up the survey (max responses reached or not enough
                // balance left for another payout), remove it from the top surveys list
                if !Self::is_live(survey.open, survey.survey_balance, survey.payout,
                    survey.total_responses, survey.max_responses) {
                    Self::remove_from_list(&mut self.top_surveys3, &survey_id);
                }
                
                // update map_3q_id_to_response
                if self.map_3q_id_to_response.try_insert(&new_response_id, &new_response).is_err() {
                    return Err(Error::DataTooLarge);
                }

                // update map_surveyid_responseid
                if self.map_surveyid_responseid.try_insert(&(survey_id, survey.total_responses), &new_response_id).is_err() {
                    return Err(Error::DataTooLarge);
                }

                // update map_survey_index
                if self.map_survey_index.try_insert(&survey_id, &survey.total_responses).is_err() {
                    return Err(Error::DataTooLarge);
                }

                // update map_account_responses
                let done: u8 = 1;
                if self.map_account_responses.try_insert(&(caller, survey_id), &done).is_err() {
                    return Err(Error::DataTooLarge);
                }

                // PAY THE RESPONDENT
                let paythis = survey.payout;
                if self.env().balance() > paythis && paythis > 0 {
                    // pay the owner the amount paythis
                    if self.env().transfer(caller, paythis).is_err() {
                        return Err(Error::PayoutFailed);
                    }
                }
                // if the balance is zero, Error (ZeroBalance)
                else {
                    return Err(Error::NotEnoughBalance);
                }

                // EMIT AN EVENT IF THE SURVEY IS PUBLIC
                if survey.public == true {
                    self.env().emit_event(NewResponse {
                        survey_id: survey_id,
                        response_id: new_response_id,
                        timestamp: time,
                        respondent: caller, 
                        payout: paythis, 
                    });
                }
            }
            Ok(())
        }


        // 6 🟢 Take A Survey of 1 Question
        #[ink(message)]
        pub fn take_a_survey_1(&mut self, 
            survey_id: Hash,
            ip_address: Vec<u8>,
            answer_1: Vec<u8>,
        ) -> Result<(), Error> {

            let caller = Self::env().caller();
            let time = self.env().block_timestamp();

            // no responses while the contract is paused or shut down
            self.ensure_active()?;

            // did the caller already take this survey?
            let repeat = self.map_account_responses.get((caller, survey_id)).unwrap_or_default();

            // how many responses have we had from this ip address?
            let mut ip_count = self.map_survey_ip_count.get((survey_id, ip_address.clone())).unwrap_or_default();

            // get the survey details
            let mut survey = self.map_1q_id_to_survey.get(survey_id).unwrap_or_default();

            // if the data is too big or the user has done this survey before,
            // or there is not enough balance on this survey or the survey is full,
            // or the survey is closed, send an error
            if answer_1.len() > 300 
            || repeat==1 || survey.survey_balance < survey.payout
            || survey.total_responses >= survey.max_responses || survey.open==false || ip_count > 3 
            || self.is_banned(survey.owner, &ip_address) {
                return Err(Error::AccessDenied);
            }
            else { 

                // create the response_id by hashing the above data
                let encodable = (caller, survey_id); // Implements `scale::Encode`
                let mut new_response_id_u8 = <Sha2x256 as HashOutput>::Type::default(); // 256-bit buffer
                ink::env::hash_encoded::<Sha2x256, _>(&encodable, &mut new_response_id_u8);
                let new_response_id: Hash = Hash::from(new_response_id_u8);

                // create the response
                let new_response = Response1 {
                    survey_id: survey_id,
                    response_id: new_response_id,
                    timestamp: time,
                    respondent: caller,
                    ip_address: ip_address.clone(),
                    a1: answer_1,
                };
                
                // UPDATE STORAGE 

                // update the ip count for this survey
                ip_count = ip_count.saturating_add(1);
                if self.map_survey_ip_count.try_insert(&(survey_id,ip_address), &ip_count).is_err() {
                    return Err(Error::DataTooLarge);
                }

                // update survey details to increment response count
                survey.total_responses = survey.total_responses.saturating_add(1);
                survey.survey_balance = survey.survey_balance.saturating_sub(survey.payout);
                if self.map_1q_id_to_survey.try_insert(&survey_id, &survey).is_err() {
                    return Err(Error::DataTooLarge);
                }

                // if this response used up the survey (max responses reached or not enough
                // balance left for another payout), remove it from the top surveys list
                if !Self::is_live(survey.open, survey.survey_balance, survey.payout,
                    survey.total_responses, survey.max_responses) {
                    Self::remove_from_list(&mut self.top_surveys1, &survey_id);
                }
                
                // update map_1q_id_to_response
                if self.map_1q_id_to_response.try_insert(&new_response_id, &new_response).is_err() {
                    return Err(Error::DataTooLarge);
                }

                // update map_surveyid_responseid
                if self.map_surveyid_responseid.try_insert(&(survey_id, survey.total_responses), &new_response_id).is_err() {
                    return Err(Error::DataTooLarge);
                }

                // update map_survey_index
                if self.map_survey_index.try_insert(&survey_id, &survey.total_responses).is_err() {
                    return Err(Error::DataTooLarge);
                }

                // update map_account_responses
                let done: u8 = 1;
                if self.map_account_responses.try_insert(&(caller, survey_id), &done).is_err() {
                    return Err(Error::DataTooLarge);
                }

                // PAY THE RESPONDENT
                let paythis = survey.payout;
                if self.env().balance() > paythis && paythis > 0 {
                    // pay the owner the amount paythis
                    if self.env().transfer(caller, paythis).is_err() {
                        return Err(Error::PayoutFailed);
                    }
                }
                // if the balance is zero, Error (ZeroBalance)
                else {
                    return Err(Error::NotEnoughBalance);
                }

                // EMIT AN EVENT IF THE SURVEY IS PUBLIC
                if survey.public == true {
                    self.env().emit_event(NewResponse {
                        survey_id: survey_id,
                        response_id: new_response_id,
                        timestamp: time,
                        respondent: caller,
                        payout: paythis,  
                    });
                }
            }
            Ok(())
        }


        // MESSAGE FUNCTIONS THAT RETRIEVE DATA FROM STORAGE  >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>

        // 7 🟢 Get_Responses_5 
        #[ink(message)]
        pub fn get_survey_responses_5(&self, 
            survey_id: Hash, 
            from_response_number: u128, 
            to_response_number:u128
        ) -> Vec<Response5> {
            let mut results = Vec::new();
            // make sure the caller is the owner of this survey and to is larger than from
            let caller = Self::env().caller();
            let survey_details = self.map_5q_id_to_survey.get(survey_id).unwrap_or_default();
            if caller == survey_details.owner && to_response_number > from_response_number {
                // for each index between from and to...
                for i in from_response_number..to_response_number {
                    // get the response at that index
                    let response_hash = self.map_surveyid_responseid.get((survey_id, i)).unwrap_or_default();
                    let response = self.map_5q_id_to_response.get(response_hash).unwrap_or_default();
                    // add the response to the results vector
                    results.push(response);
                }
            }

            results
        }
        
        // 8 🟢 Get_Responses_3 
        #[ink(message)]
        pub fn get_survey_responses_3(&self, 
            survey_id: Hash, 
            from_response_number: u128, 
            to_response_number:u128
        ) -> Vec<Response3> {
            let mut results = Vec::new();
            // make sure the caller is the owner of this survey and to is larger than from
            let caller = Self::env().caller();
            let survey_details = self.map_3q_id_to_survey.get(survey_id).unwrap_or_default();
            if caller == survey_details.owner && to_response_number > from_response_number {
                // for each index between from and to...
                for i in from_response_number..to_response_number {
                    // get the response at that index
                    let response_hash = self.map_surveyid_responseid.get((survey_id, i)).unwrap_or_default();
                    let response = self.map_3q_id_to_response.get(response_hash).unwrap_or_default();
                    // add the response to the results vector
                    results.push(response);
                }
            }

            results
        }
        
        // 9 🟢 Get_Responses_1 
        #[ink(message)]
        pub fn get_survey_responses_1(&self, 
            survey_id: Hash, 
            from_response_number: u128, 
            to_response_number:u128
        ) -> Vec<Response1> {
            let mut results = Vec::new();
            // make sure the caller is the owner of this survey and to is larger than from
            let caller = Self::env().caller();
            let survey_details = self.map_1q_id_to_survey.get(survey_id).unwrap_or_default();
            if caller == survey_details.owner && to_response_number > from_response_number {
                // for each index between from and to...
                for i in from_response_number..to_response_number {
                    // get the response at that index
                    let response_hash = self.map_surveyid_responseid.get((survey_id, i)).unwrap_or_default();
                    let response = self.map_1q_id_to_response.get(response_hash).unwrap_or_default();
                    // add the response to the results vector
                    results.push(response);
                }
            }

            results
        }
        
        // 10 🟢 Available_Surveys
        #[ink(message)]
        pub fn get_available_surveys(&self) -> Available {

            let mut results1 = Vec::new();
            let mut results3 = Vec::new();
            let mut results5 = Vec::new();

            // after a shutdown nothing is available
            if self.shut_down {
                return Available { surveys1: results1, surveys3: results3, surveys5: results5 };
            }

            // iterate on each vector to get the survey details
            for i in self.top_surveys1.iter() {
                let thissurvey = self.map_1q_id_to_survey.get(i).unwrap_or_default();
                // if there is sufficient balance on this survey and it has not hit max responses, 
                // and it is not private, include it 
                if thissurvey.survey_balance >= thissurvey.payout 
                && thissurvey.payout > 0 && thissurvey.public == true
                && thissurvey.total_responses < thissurvey.max_responses {
                    results1.push(thissurvey);
                }
            }
            for i in self.top_surveys3.iter() {
                let thissurvey = self.map_3q_id_to_survey.get(i).unwrap_or_default();
                // if there is sufficient balance on this survey and it has not hit max responses, include it
                if thissurvey.survey_balance >= thissurvey.payout 
                && thissurvey.payout > 0 && thissurvey.public == true
                && thissurvey.total_responses < thissurvey.max_responses {
                    results3.push(thissurvey);
                }
            }
            for i in self.top_surveys5.iter() {
                let thissurvey = self.map_5q_id_to_survey.get(i).unwrap_or_default();
                // if there is sufficient balance on this survey and it has not hit max responses, include it
                if thissurvey.survey_balance >= thissurvey.payout 
                && thissurvey.payout > 0 && thissurvey.public == true
                && thissurvey.total_responses < thissurvey.max_responses {
                    results5.push(thissurvey);
                }
            }

            let results = Available {
                surveys1: results1,
                surveys3: results3,
                surveys5: results5,
            };

            results
        }
        
        // 11 🟢 My_Surveys
        #[ink(message)]
        pub fn get_my_surveys(&self) -> Available {
            let caller = Self::env().caller();

            let mut results1 = Vec::new();
            let mut results3 = Vec::new();
            let mut results5 = Vec::new();

            // get the caller's survey list
            let mysurveys = self.map_account_surveys.get(caller).unwrap_or_default();
            // iterate to get the survey details
            for i in mysurveys.iter() {
                if self.map_1q_id_to_survey.contains(i) {
                    // get the details and add them to results
                    let thissurvey = self.map_1q_id_to_survey.get(i).unwrap_or_default();
                    results1.push(thissurvey);
                }
                if self.map_3q_id_to_survey.contains(i) {
                    // get the details and add them to results
                    let thissurvey = self.map_3q_id_to_survey.get(i).unwrap_or_default();
                    results3.push(thissurvey);
                }
                if self.map_5q_id_to_survey.contains(i) {
                    // get the details and add them to results
                    let thissurvey = self.map_5q_id_to_survey.get(i).unwrap_or_default();
                    results5.push(thissurvey);
                }
            }

            let results = Available {
                surveys1: results1,
                surveys3: results3,
                surveys5: results5,
            };

            results
        }

        // 12 🟢 Ban an ip address from all of the caller's own surveys
        // (the ip can still take surveys owned by other people)
        #[ink(message)]
        pub fn ban_this_ip_address(&mut self,
            survey_id: Hash,
            ip_address: Vec<u8>,
        ) -> Result<(), Error> {

            self.ensure_active()?;

            let caller = Self::env().caller();

            // is the caller the survey owner? and did the offending ip address really take the survey?
            let mysurveys = self.map_account_surveys.get(caller).unwrap_or_default();
            let ip_response = self.map_survey_ip_count.get((survey_id,ip_address.clone())).unwrap_or_default();
            if mysurveys.contains(&survey_id) && ip_response > 0 {
                // ban the ip address from this caller's surveys only
                let yes: u8 = 1;
                if self.map_owner_banned_ip.try_insert(&(caller, ip_address), &yes).is_err() {
                    return Err(Error::DataTooLarge);
                }
            }

            Ok(())
        }

        // 13 🟢 Get A Specific Survey's Questions
        #[ink(message)]
        pub fn get_survey_by_id(&self,
            survey_id: Hash
        ) -> Available {

            let mut results1 = Vec::new();
            let mut results3 = Vec::new();
            let mut results5 = Vec::new();

            // check if it is in the 1Q mapping
            if self.map_1q_id_to_survey.contains(survey_id) {
                let thissurvey = self.map_1q_id_to_survey.get(survey_id).unwrap_or_default();
                // always include it, even if closed or used up, so there is a permanent record
                // (callers should check open / survey_balance / total_responses before offering it)
                results1.push(thissurvey);
            }
            // check if it is in the 3Q mapping
            if self.map_3q_id_to_survey.contains(survey_id) {
                let thissurvey = self.map_3q_id_to_survey.get(survey_id).unwrap_or_default();
                // always include it, even if closed or used up, so there is a permanent record
                // (callers should check open / survey_balance / total_responses before offering it)
                results3.push(thissurvey);
            }
            // check if it is in the 5Q mapping
            if self.map_5q_id_to_survey.contains(survey_id) {
                let thissurvey = self.map_5q_id_to_survey.get(survey_id).unwrap_or_default();
                // always include it, even if closed or used up, so there is a permanent record
                // (callers should check open / survey_balance / total_responses before offering it)
                results5.push(thissurvey);
            }

            let results = Available {
                surveys1: results1,
                surveys3: results3,
                surveys5: results5,
            };

            results
        }

        // 14 🟢 ROOT ONLY: Pause or unpause the contract
        // pause = true stops every message that changes storage (new surveys, responses,
        // closing surveys, banning). Read-only messages keep working.
        // pause = false resumes normal operation. Cannot be used after a shutdown.
        #[ink(message)]
        pub fn pause_contract(&mut self,
            pause: bool,
        ) -> Result<(), Error> {
            if Self::env().caller() != self.root {
                return Err(Error::NotRoot);
            }
            if self.shut_down {
                return Err(Error::ContractShutDown);
            }
            self.paused = pause;
            Ok(())
        }

        // 15 🟢 ROOT ONLY: Permanently shut down the contract
        // Stops every message for good except the read-only messages and close_my_survey,
        // so owners can still close their surveys and get their leftover coin back.
        // Cannot be undone (pause_contract no longer works after this).
        #[ink(message)]
        pub fn shutdown_contract(&mut self) -> Result<(), Error> {
            if Self::env().caller() != self.root {
                return Err(Error::NotRoot);
            }
            self.shut_down = true;
            Ok(())
        }

        // END OF MESSAGE FUNCTIONS

    }
    // END OF CONTRACT LOGIC

    // UNIT TESTS >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>
    #[cfg(test)]
    mod tests {
        use super::*;
        use ink::env::{test, DefaultEnvironment};

        type E = DefaultEnvironment;

        fn accounts() -> test::DefaultAccounts<E> { test::default_accounts::<E>() }

        fn setup() -> ContractStorage {
            // alice instantiates the contract, so alice is root
            test::set_caller::<E>(accounts().alice);
            let c = ContractStorage::new();
            let me = test::callee::<E>();
            test::set_account_balance::<E>(me, 1_000_000_000);
            c
        }

        // create a public 1Q survey as `who`, staking exactly payout * max
        fn make1(c: &mut ContractStorage, who: AccountId, name: &str, payout: Balance, max: u128) -> Hash {
            test::set_caller::<E>(who);
            test::set_value_transferred::<E>(payout * max);
            test::advance_block::<E>(); // new timestamp -> unique survey id
            c.make_a_new_survey_1(name.as_bytes().to_vec(), true, payout, max, b"q?".to_vec()).unwrap();
            test::set_value_transferred::<E>(0);
            *c.map_account_surveys.get(who).unwrap().last().unwrap()
        }

        fn take1(c: &mut ContractStorage, who: AccountId, id: Hash) -> Result<(), Error> {
            test::set_caller::<E>(who);
            c.take_a_survey_1(id, b"1.1.1.1".to_vec(), b"yes".to_vec())
        }

        // (1) a survey that hits max responses leaves the available list without being closed
        #[ink::test]
        fn used_up_survey_is_removed_from_available() {
            let a = accounts();
            let mut c = setup();
            let id = make1(&mut c, a.alice, "s", 10, 1);
            assert_eq!(c.get_available_surveys().surveys1.len(), 1);
            take1(&mut c, a.bob, id).unwrap();
            assert!(!c.top_surveys1.contains(&id));
            assert_eq!(c.get_available_surveys().surveys1.len(), 0);
        }

        // (1) a full list of used-up surveys no longer blocks a new survey with the same payout
        #[ink::test]
        fn full_list_of_dead_surveys_does_not_block_new_survey() {
            let a = accounts();
            let mut c = setup();
            for i in 0..100 {
                let id = make1(&mut c, a.alice, &format!("s{}", i), 10, 1);
                // simulate an old dead survey still sitting on the list (as on the live contract)
                let mut s = c.map_1q_id_to_survey.get(id).unwrap();
                s.total_responses = 1;
                s.survey_balance = 0;
                c.map_1q_id_to_survey.insert(id, &s);
            }
            assert_eq!(c.top_surveys1.len(), 100);
            let new_id = make1(&mut c, a.charlie, "new", 10, 1);
            assert_eq!(c.top_surveys1, vec![new_id]);
            let avail = c.get_available_surveys().surveys1;
            assert_eq!(avail.len(), 1);
            assert_eq!(avail[0].survey_id, new_id);
        }

        // (2) closing zeroes the balance; closing again refunds nothing
        #[ink::test]
        fn close_zeroes_balance_and_cannot_refund_twice() {
            let a = accounts();
            let mut c = setup();
            let id = make1(&mut c, a.alice, "s", 10, 5); // stakes 50
            test::set_caller::<E>(a.alice);
            let before = test::get_account_balance::<E>(a.alice).unwrap();
            c.close_my_survey(id, 1).unwrap();
            assert_eq!(test::get_account_balance::<E>(a.alice).unwrap(), before + 50);
            let s = c.map_1q_id_to_survey.get(id).unwrap();
            assert_eq!(s.survey_balance, 0);
            assert!(!s.open);
            // second close: no extra refund
            c.close_my_survey(id, 1).unwrap();
            assert_eq!(test::get_account_balance::<E>(a.alice).unwrap(), before + 50);
        }

        // (3) closing with the wrong length errors and writes nothing
        #[ink::test]
        fn close_with_wrong_length_errors() {
            let a = accounts();
            let mut c = setup();
            let id = make1(&mut c, a.alice, "s", 10, 5);
            test::set_caller::<E>(a.alice);
            assert_eq!(c.close_my_survey(id, 5), Err(Error::WrongSurveyLength));
            assert_eq!(c.close_my_survey(id, 3), Err(Error::WrongSurveyLength));
            assert_eq!(c.close_my_survey(id, 2), Err(Error::WrongSurveyLength));
            assert!(!c.map_5q_id_to_survey.contains(id));
            assert!(!c.map_3q_id_to_survey.contains(id));
            assert!(c.map_1q_id_to_survey.get(id).unwrap().open);
            // someone else's survey is still AccessDenied
            test::set_caller::<E>(a.bob);
            assert_eq!(c.close_my_survey(id, 1), Err(Error::AccessDenied));
        }

        // (4) get_survey_by_id still returns closed and used-up surveys
        #[ink::test]
        fn get_survey_by_id_returns_closed_and_used_up() {
            let a = accounts();
            let mut c = setup();
            let closed = make1(&mut c, a.alice, "closed", 10, 5);
            test::set_caller::<E>(a.alice);
            c.close_my_survey(closed, 1).unwrap();
            let r = c.get_survey_by_id(closed).surveys1;
            assert_eq!(r.len(), 1);
            assert!(!r[0].open);

            let used = make1(&mut c, a.alice, "used", 10, 1);
            take1(&mut c, a.bob, used).unwrap();
            let r = c.get_survey_by_id(used).surveys1;
            assert_eq!(r.len(), 1);
            assert_eq!(r[0].total_responses, 1);
        }

        fn take1_ip(c: &mut ContractStorage, who: AccountId, id: Hash, ip: &str) -> Result<(), Error> {
            test::set_caller::<E>(who);
            c.take_a_survey_1(id, ip.as_bytes().to_vec(), b"yes".to_vec())
        }

        fn make3(c: &mut ContractStorage, who: AccountId, payout: Balance, max: u128) -> Hash {
            test::set_caller::<E>(who);
            test::set_value_transferred::<E>(payout * max);
            test::advance_block::<E>();
            c.make_a_new_survey_3(b"s3".to_vec(), true, payout, max,
                b"a".to_vec(), b"b".to_vec(), b"c".to_vec()).unwrap();
            test::set_value_transferred::<E>(0);
            *c.map_account_surveys.get(who).unwrap().last().unwrap()
        }

        fn make5(c: &mut ContractStorage, who: AccountId, payout: Balance, max: u128) -> Hash {
            test::set_caller::<E>(who);
            test::set_value_transferred::<E>(payout * max);
            test::advance_block::<E>();
            c.make_a_new_survey_5(b"s5".to_vec(), true, payout, max,
                b"a".to_vec(), b"b".to_vec(), b"c".to_vec(), b"d".to_vec(), b"e".to_vec()).unwrap();
            test::set_value_transferred::<E>(0);
            *c.map_account_surveys.get(who).unwrap().last().unwrap()
        }

        fn bal(who: AccountId) -> Balance { test::get_account_balance::<E>(who).unwrap() }

        // (5) a ban only covers the banning owner's surveys (all of them), not anyone else's
        #[ink::test]
        fn ip_ban_is_scoped_to_the_banning_owner() {
            let a = accounts();
            let mut c = setup();
            let alice1 = make1(&mut c, a.alice, "a1", 10, 5);
            let alice2 = make1(&mut c, a.alice, "a2", 10, 5);
            let charlie1 = make1(&mut c, a.charlie, "c1", 10, 5);
            take1_ip(&mut c, a.bob, alice1, "9.9.9.9").unwrap();

            // only the owner of the survey the ip actually took can ban it
            test::set_caller::<E>(a.charlie);
            c.ban_this_ip_address(alice1, b"9.9.9.9".to_vec()).unwrap();
            assert!(!c.is_banned(a.alice, &b"9.9.9.9".to_vec()));
            assert!(!c.is_banned(a.charlie, &b"9.9.9.9".to_vec()));

            test::set_caller::<E>(a.alice);
            c.ban_this_ip_address(alice1, b"9.9.9.9".to_vec()).unwrap();

            // blocked from alice's other survey (any account using that ip)...
            assert_eq!(take1_ip(&mut c, a.django, alice2, "9.9.9.9"), Err(Error::AccessDenied));
            // ...but still free to take charlie's survey
            take1_ip(&mut c, a.django, charlie1, "9.9.9.9").unwrap();
            // other ips are unaffected on alice's surveys
            take1_ip(&mut c, a.eve, alice2, "8.8.8.8").unwrap();
        }

        // (6) only root can pause; pause blocks every storage-changing message; reads still work
        #[ink::test]
        fn pause_blocks_writes_and_only_root_can_pause() {
            let a = accounts();
            let mut c = setup();
            let id = make1(&mut c, a.bob, "s", 10, 5);

            test::set_caller::<E>(a.bob);
            assert_eq!(c.pause_contract(true), Err(Error::NotRoot));

            test::set_caller::<E>(a.alice);
            c.pause_contract(true).unwrap();

            test::set_caller::<E>(a.bob);
            test::set_value_transferred::<E>(50);
            assert_eq!(c.make_a_new_survey_1(b"x".to_vec(), true, 10, 5, b"q".to_vec()), Err(Error::ContractPaused));
            assert_eq!(c.make_a_new_survey_3(b"x".to_vec(), true, 10, 5, b"q".to_vec(), b"q".to_vec(), b"q".to_vec()), Err(Error::ContractPaused));
            assert_eq!(c.make_a_new_survey_5(b"x".to_vec(), true, 10, 5, b"q".to_vec(), b"q".to_vec(), b"q".to_vec(), b"q".to_vec(), b"q".to_vec()), Err(Error::ContractPaused));
            test::set_value_transferred::<E>(0);
            assert_eq!(take1(&mut c, a.charlie, id), Err(Error::ContractPaused));
            assert_eq!(c.take_a_survey_3(id, b"1".to_vec(), b"a".to_vec(), b"a".to_vec(), b"a".to_vec()), Err(Error::ContractPaused));
            assert_eq!(c.take_a_survey_5(id, b"1".to_vec(), b"a".to_vec(), b"a".to_vec(), b"a".to_vec(), b"a".to_vec(), b"a".to_vec()), Err(Error::ContractPaused));
            test::set_caller::<E>(a.bob);
            assert_eq!(c.close_my_survey(id, 1), Err(Error::ContractPaused));
            assert_eq!(c.ban_this_ip_address(id, b"1".to_vec()), Err(Error::ContractPaused));

            // reads still work
            assert_eq!(c.get_available_surveys().surveys1.len(), 1);
            assert_eq!(c.get_my_surveys().surveys1.len(), 1);

            // unpause and everything works again
            test::set_caller::<E>(a.alice);
            c.pause_contract(false).unwrap();
            take1(&mut c, a.charlie, id).unwrap();
        }

        // (7) shutdown: root only, permanent, leaves only reads and close_my_survey working
        #[ink::test]
        fn shutdown_stops_all_but_reads_and_close() {
            let a = accounts();
            let mut c = setup();
            let s1 = make1(&mut c, a.bob, "b1", 10, 5);   // 50 staked
            let s3 = make3(&mut c, a.charlie, 7, 4);      // 28 staked
            let s5 = make5(&mut c, a.bob, 3, 10);         // 30 staked
            take1(&mut c, a.django, s1).unwrap();         // s1 now has 40 left

            test::set_caller::<E>(a.bob);
            assert_eq!(c.shutdown_contract(), Err(Error::NotRoot));

            // shutdown works even while paused
            test::set_caller::<E>(a.alice);
            c.pause_contract(true).unwrap();
            c.shutdown_contract().unwrap();
            assert_eq!(c.pause_contract(false), Err(Error::ContractShutDown));
            assert_eq!(c.pause_contract(true), Err(Error::ContractShutDown));

            // no new surveys, responses or bans
            test::set_caller::<E>(a.eve);
            test::set_value_transferred::<E>(50);
            assert_eq!(c.make_a_new_survey_1(b"x".to_vec(), true, 10, 5, b"q".to_vec()), Err(Error::ContractShutDown));
            assert_eq!(c.make_a_new_survey_3(b"x".to_vec(), true, 10, 5, b"q".to_vec(), b"q".to_vec(), b"q".to_vec()), Err(Error::ContractShutDown));
            assert_eq!(c.make_a_new_survey_5(b"x".to_vec(), true, 10, 5, b"q".to_vec(), b"q".to_vec(), b"q".to_vec(), b"q".to_vec(), b"q".to_vec()), Err(Error::ContractShutDown));
            test::set_value_transferred::<E>(0);
            assert_eq!(take1(&mut c, a.eve, s1), Err(Error::ContractShutDown));
            assert_eq!(c.take_a_survey_3(s3, b"1".to_vec(), b"a".to_vec(), b"a".to_vec(), b"a".to_vec()), Err(Error::ContractShutDown));
            assert_eq!(c.take_a_survey_5(s5, b"1".to_vec(), b"a".to_vec(), b"a".to_vec(), b"a".to_vec(), b"a".to_vec(), b"a".to_vec()), Err(Error::ContractShutDown));
            test::set_caller::<E>(a.bob);
            assert_eq!(c.ban_this_ip_address(s1, b"1.1.1.1".to_vec()), Err(Error::ContractShutDown));

            // reads still work; nothing is listed as available
            let av = c.get_available_surveys();
            assert!(av.surveys1.is_empty() && av.surveys3.is_empty() && av.surveys5.is_empty());
            assert_eq!(c.get_my_surveys().surveys1.len(), 1);
            assert_eq!(c.get_survey_by_id(s3).surveys3.len(), 1);
            assert_eq!(c.get_survey_responses_1(s1, 0, 1).len(), 1);

            // owners close their surveys and get their leftover coin back
            let bob0 = bal(a.bob);
            let charlie0 = bal(a.charlie);
            c.close_my_survey(s1, 1).unwrap();
            c.close_my_survey(s5, 5).unwrap();
            assert_eq!(bal(a.bob), bob0 + 40 + 30);
            test::set_caller::<E>(a.charlie);
            c.close_my_survey(s3, 3).unwrap();
            assert_eq!(bal(a.charlie), charlie0 + 28);
            // and only once
            c.close_my_survey(s3, 3).unwrap();
            assert_eq!(bal(a.charlie), charlie0 + 28);
            // still only the owner can close
            test::set_caller::<E>(a.eve);
            assert_eq!(c.close_my_survey(s1, 1), Err(Error::AccessDenied));
        }
    }

}
