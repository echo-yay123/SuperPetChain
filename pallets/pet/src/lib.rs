#![cfg_attr(not(feature = "std"), no_std)]
/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/reference/frame-pallets/>
pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;
	
	//use core::default::default;

    #[pallet::pallet]
    pub struct Pallet<T>(_);


    #[pallet::config]
    pub trait Config: frame_system::Config {
    	/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		/// The maximum length of a metadata string.
		#[pallet::constant]
		type StringLimit: Get<u32>;
    }


	type PetId = u32;

	#[derive(Encode, Decode, Clone, PartialEq, Eq, Default, RuntimeDebug, MaxEncodedLen, TypeInfo)]
    pub enum Species {
    #[default]
    Turtle,
    Snake,
    Rabbit,
    }


    #[derive(Encode, Decode, Clone, PartialEq, Eq, Default, RuntimeDebug, MaxEncodedLen, TypeInfo)]
	#[scale_info(skip_type_params(T))]
    pub struct PetInfo<T:Config> {
    	pub Name: BoundedVec<u8, T::StringLimit>,
   		pub Species: Species,
    		//pub Created_time: T::Moment,
   		pub HungryPoints: u32,
   		pub EnergyPoints: u32,
   		pub SkillPoints: u32,

    }  


	// The pallet's runtime storage items.
	// https://docs.substrate.io/main-docs/build/runtime-storage/
	#[pallet::storage]
	#[pallet::getter(fn something)]
	// Learn more about declaring storage items:
	// https://docs.substrate.io/main-docs/build/runtime-storage/#declaring-storage-items
	
    //pub type Pets<T: Config> = StorageMap<_, Blake2_128Concat, u32, PetInfo<T>,  PetState<T>>;
	pub type PetsInfo<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, (PetId, PetInfo<T>), >;
	
    // Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/main-docs/build/events-errors/
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event      
        //User do something with Pet , provide userId and perId
		PetMinted(T::AccountId, u32),
		PetTransfered(T::AccountId, T::AccountId, u32),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		IdNotExists,
		AlreadyHavePet,
        GetInformationFail,
		SalerDoNotHavePet,
		BuyerAlreadyHavePet,
		PetIdMismatch,
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// An example dispatchable that takes a singles value as a parameter, writes the value to
		/// storage and emits an event. This function must be dispatched by a signed extrinsic.
		#[pallet::call_index(0)]
		#[pallet::weight(0)]
		pub fn mint_pet(
			origin: OriginFor<T>,
			petname: BoundedVec<u8, T::StringLimit>,
			petspeies: Species,
			petid: u32,
		) -> DispatchResultWithPostInfo {
			
			let sender = ensure_signed(origin)?;
			
			//Check sender have a pet or not
			//If sender have a pet, return error message.
            ensure!(!PetsInfo::<T>::contains_key(&sender), Error::<T>::AlreadyHavePet);



			//let now = <pallet_timestamp::Pallet<T>>::get();
			let pet = PetInfo {
                Name: petname,
                Species: petspeies,
                //pub Created_time: T::Moment,
                HungryPoints: 0,
				EnergyPoints: 100,
				SkillPoints: 1,
			};

			PetsInfo::<T>::insert(sender.clone(),(petid, pet));
			
            //LastFeedTime::<T>::insert(id, &sender, now);

			Self::deposit_event(Event::PetMinted(sender, petid));
			Ok(().into())
		}
		
		#[pallet::call_index(1)]
		#[pallet::weight(0)]
		pub fn transfer_pet(
			origin: OriginFor<T>, 
			receiver: T::AccountId, 
			petid: u32,
		) -> DispatchResultWithPostInfo {
			
			let sender = ensure_signed(origin)?;
			
			//Make sure the saler have pet
			//And the buyer have no pet
			//Check the saler's petid, if the petid isn't the buyer want
			//The transaction can not be done.
            ensure!(PetsInfo::<T>::contains_key(&sender), Error::<T>::SalerDoNotHavePet);
			let (id,_) = PetsInfo::<T>::get(&sender).ok_or(Error::<T>::GetInformationFail)?;
			ensure!(!PetsInfo::<T>::contains_key(&receiver), Error::<T>::BuyerAlreadyHavePet);
			ensure!(petid == id, Error::<T>::PetIdMismatch);

			//Swap the pet	 
			PetsInfo::<T>::swap(sender.clone(),receiver.clone());

            // Emit an event that the pet was transfered.
            Self::deposit_event(Event::PetTransfered(sender,receiver,petid));
            // 交易发送成功			
			Ok(().into())	
		}
		
    }

/* 
		/// The owner can feed its pet.
		#[pallet::call_index(1)]
		#[pallet::weight(0)]
		pub fn feed_pet(origin: OriginFor<T>, id: u32) -> DispatchResultWithPostInfo {
			let sender = ensure_signed(origin)?;
			let pet = Pets::<T>::get(id).ok_or(Error::<T>::PetNotExists)?;
			ensure!(pet.owner == sender, Error::<T>::NotPetOwner);

			LastFeedTime::<T>::insert(id, &sender, <pallet_timestamp::Pallet<T>>::get());

			Self::deposit_event(Event::PetFeeded(sender, id));
			Ok(().into())
		}

		/// The pet has been hungry for last 3 days
		/// others can save the cat by feeding it.
		#[pallet::call_index(2)]
		#[pallet::weight(0)]
		pub fn save_pet(origin: OriginFor<T>, id: u32) -> DispatchResultWithPostInfo {
			let sender = ensure_signed(origin)?;
			let pet = Pets::<T>::get(id).ok_or(Error::<T>::PetNotExists)?;
			ensure!(pet.owner != sender, Error::<T>::OwnerShouldFeedPet);

			let now = <pallet_timestamp::Pallet<T>>::get();
			let can_save = LastFeedTime::<T>::iter_prefix(id).into_iter().all(|(account, last_feed_time)| {
				(now > last_feed_time + (259_200_000 as u32).into()) // 3 days since last feed
					|| sender == account
			});

			if !can_save {
				return Err(Error::<T>::NotAvailableToSave.into())
			}

			LastFeedTime::<T>::insert(id, &sender, now);

			Self::deposit_event(Event::PetSaved(sender, id));
			Ok(().into())
		}

		/// The previous owner stops feeding the pet for a long time
		/// the saver has active feeding the pet in last 3 days
		/// the save can own the cat without agree from its previous owner
		#[pallet::call_index(3)]
		#[pallet::weight(0)]
		pub fn save_pet_by_force(origin: OriginFor<T>, id: u32) -> DispatchResultWithPostInfo {
			let sender = ensure_signed(origin)?;
			let mut pet = Pets::<T>::get(id).ok_or(Error::<T>::PetNotExists)?;
			let previous_pet_owner = pet.owner.clone();
			ensure!(previous_pet_owner != sender, Error::<T>::OwnerShouldFeedPet);

			let now = <pallet_timestamp::Pallet<T>>::get();
			let can_force_save = LastFeedTime::<T>::iter_prefix(id).into_iter().all(|(account, last_feed_time)| {
				(previous_pet_owner == account && now > last_feed_time + (518_400_000 as u32).into()) // 6 days since last feed
					|| (sender == account && now < last_feed_time + (86_400_000 as u32).into()) // 1 day since last feed
			});

			if !can_force_save {
				return Err(Error::<T>::NotAvailableToForceSave.into())
			}

			pet.owner = sender.clone();
			Pets::<T>::insert(id, pet);

			let res = LastFeedTime::<T>::clear_prefix(id, u32::MAX, None);
			ensure!(res.maybe_cursor.is_none(), Error::<T>::LastFeedNotCleared);
			LastFeedTime::<T>::insert(id, &sender, now);

			Self::deposit_event(Event::PetForceSaved(sender, id));
			Ok(().into())
		}		
		
		*/
		
		
	
}

