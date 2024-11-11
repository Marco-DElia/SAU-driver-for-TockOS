 /// cortex-v8m/src/sau.rs
 
use kernel::utilities::registers::interfaces::{Readable, Writeable};
use kernel::utilities::registers::{register_bitfields, ReadOnly, ReadWrite};
use kernel::utilities::StaticRef;


use kernel::platform::sau;



pub struct SauRegisters {

    pub sau_ctrl : ReadWrite<u32, SauCtrl::Register>,
    pub sau_type : ReadOnly<u32, SauType::Register>,
    pub sau_rnr  : ReadWrite<u32, SauRnr::Register>,
    pub sau_rbar : ReadWrite<u32, SauRbar::Register>,
    pub sau_rlar : ReadWrite<u32, SauRlar::Register>,
    pub sau_sfsr : ReadOnly<u32, SauSfsr::Register>,
    pub sau_sfar : ReadWrite<u32, SauSfar::Register>,

}


register_bitfields![u32,

    SauCtrl [
        /// When SauCtrl.ENABLE = 0 this bit controls if the memory is marked as Non-secure
        /// or Secure. If SauCtrl.Enable = 1 this bis has no effect.
        ALLNS     OFFSET(1)   NUMBITS(1) [
            MemIsNonSecure = 1,
            MemIsSecure = 0,
        ],  

        /// Enable the SAU.
        ENABLE    OFFSET(0)   NUMBITS(1) [
            Enable = 1,
            Disable = 0
        ]   
    ],

    SauType [
        /// The number of implemented SAU regions.
        SREGION  OFFSET(0)   NUMBITS(8) []
    ],
    
    SauRnr [
        /// Selects the region currently accessed by SAU_RBAR and SAU_RLAR
        REGION OFFSET(0)   NUMBITS(8) []
    ],

    SauRbar [
        /// Upper 27 bits of the lower bound of the selected SAU region. 
        /// This value is zero extended to provide the base address.
        BASE    OFFSET(5)   NUMBITS(27) []
    ],   

    SauRlar [
        /// Upper 27 bits of the upper bound of the selected SAU region.
        /// This vaule is extended with the value 0x1F (to be sure that the
        /// address is a multiple of 32) to provide the limit address. 
        LADDR    OFFSET(5)   NUMBITS(27) [],

        /// Controls whether Non-secure state is permitted to execute an SG
        /// instruction from this region.
        NSC    OFFSET(1)   NUMBITS(1) [
            RegIsNonSecureCallable = 1,
            RegIsNotNonSecureCallable = 0
        ],

        /// SAU region enable.
        ENABLE    OFFSET(0)   NUMBITS(1) [
            Disable = 1,
            Enable = 0
        ]
    ],

    SauSfsr [
        /// Lazy state error flag. Sticky flag indicating that an error occurred during
        /// lazy state activation or deactivation.

        LSERR    OFFSET(7)    NUMBITS(1) [
            ErrorOccurred = 1,
            ErrorNotOccurred = 0
        ],

        /// This bit is set when the SFAR register contains a valid value.
        SFARVALID    OFFSET(6)    NUMBITS(1) [
            SFARValid = 1,
            SFARNotValid = 0
        ],

        /// 
        LSPERR    OFFSET(5)    NUMBITS(1) [
            ErrorOccurred = 1,
            ErrorNotOccurred = 0
        ],

        /// Invalid transition flag. Sticky flag indicating that an exception was
        /// raised due to a branch that was not flagged as being domain crossing causing
        /// a transition from Secure to Non-secure memory.
        INVTRAN    OFFSET(4)    NUMBITS(1) [
            ErrorOccurred = 1,
            ErrorNotOccurred = 0
        ],

        /// Attribution unit violation flag. Sticky flag indicating that an attempt
        /// was made to access parts of the address space that are marked as Secure
        ///with NS-Req for the transaction set to Non-secure.
        AUVIOL    OFFSET(3)    NUMBITS(1) [
            ErrorOccurred = 1,
            ErrorNotOccurred = 0
        ],

        /// Invalid exception return flag.
        INVER    OFFSET(2)    NUMBITS(1) [
            ErrorOccurred = 1,
            ErrorNotOccurred = 0
        ],

        /// Invalid integrity signature flag. This bit is set if the integrity signature
        /// in an exception stack frame is found to be invalid during the unstacking
        /// operation.
        INVIS    OFFSET(1)    NUMBITS(1) [
            ErrorOccurred = 1,
            ErrorNotOccurred = 0
        ],

        /// Invalid entry point. This bit is set if a function call from the Non-secure
        /// state or exception targets a non-SG instruction in the Secure state.
        /// This bit is also set if the target address is an SG instruction, but there
        /// is no matching SAU/IDAU region with the NSC flag set.
        INVEP    OFFSET(0)    NUMBITS(1) [
            ErrorOccurred = 1,
            ErrorNotOccurred = 0
        ],

    ],

    SauSfar [
        /// When the SFARVALID bit of the SFSR is set to 1, this field holds
        /// the address of an access that caused an SAU violation.
        ADDRESS    OFFSET(0)   NUMBITS(32) []
    ],

];

/// Possible error values returned by the SAU methods.
pub enum SauError {
    /// The region number parameter to set or get a region must be between 0 and
    /// region_numbers() - 1.
    RegionNumberTooBig,
    /// Bits 0 to 4 of the base address of a SAU region must be set to zero.
    WrongBaseAddress,
    /// Bits 0 to 4 of the limit address of a SAU region must be set to one.
    WrongLimitAddress,
}


////////////////////////////
//   _____        _    _  //
// /  ____|  /\  | |  | | //
// | (___   /  \ | |  | | //
// \___  \ / /\ \| |  | | //
//  ____) / ____ \ |__| | //
// |_____/_/    \_\____/  //
//                        //
////////////////////////////           

/// State related to the real physical SAU. 
///
/// There should only be one instantiation of this object as it represents
/// real hardware.

const SAU_BASE_ADDRESS: StaticRef<SauRegisters> =
    unsafe { StaticRef::new(0xE000EDD0 as *const SauRegisters) };

pub struct Sau <const NUM_REGIONS: usize>{

    registers: StaticRef<SauRegisters>,
    }

impl <const NUM_REGIONS: usize> Sau<NUM_REGIONS>{
    pub const unsafe fn new() -> Self {
        Self {
            registers: SAU_BASE_ADDRESS,
        }
    }

    pub fn region_numbers(&self) -> u8 {
       return self.registers.sau_type.read(SauType::SREGION) as u8;
    }

    pub fn enable(&mut self) {
        self.registers.sau_ctrl.write(SauCtrl::ENABLE::Enable)
    }

    pub fn set_region_intern(&mut self, region_number: u8, region: sau::SauRegion) -> Result<(), SauError> {
        
        if region_number >= self.region_numbers() {
            Err(SauError::RegionNumberTooBig)
        } else if region.base_address & 0x1F != 0 {
            Err(SauError::WrongBaseAddress)
        } else if region.limit_address & 0x1F != 0x1F {
            Err(SauError::WrongLimitAddress)
        } else {  
            self.registers.sau_rnr.set(region_number as u32);
            self.registers.sau_rbar.write(SauRbar::BASE.val(region.base_address >> 5));
            
            match region.attribute {
                sau::SauRegionAttribute::Secure => {
                    self.registers.sau_rlar.write(SauRlar::LADDR.val(region.limit_address >> 5) + SauRlar::NSC::RegIsNotNonSecureCallable + SauRlar::ENABLE::Enable);
                }
                sau::SauRegionAttribute::NonSecureCallable => {
                    self.registers.sau_rlar.write(SauRlar::LADDR.val(region.limit_address >> 5) + SauRlar::NSC::RegIsNonSecureCallable + SauRlar::ENABLE::Disable);
                }
                sau::SauRegionAttribute::NonSecure => {
                    self.registers.sau_rlar.write(SauRlar::LADDR.val(region.limit_address >> 5) + SauRlar::NSC::RegIsNotNonSecureCallable + SauRlar::ENABLE::Disable);
                }
            }
            Ok(())
        }
    }

    pub fn get_region(&mut self, region_number: u8) -> Result<sau::SauRegion, SauError> {

        if region_number >= self.region_numbers() {
            return Err(SauError::RegionNumberTooBig);
        } else {
            self.registers.sau_rnr.write(SauRnr::REGION.val(region_number.into()));
        }

        let base_address = self.registers.sau_rbar.read(SauRbar::BASE);
        let limit_address = self.registers.sau_rlar.read(SauRlar::LADDR);
	
	let bf = self.registers.sau_rlar.read(SauRlar::NSC) == 1;
	let bf2 = self.registers.sau_rlar.read(SauRlar::ENABLE) == 0;
        
        let attribute = match (bf, bf2) {
            (_, false) => sau::SauRegionAttribute::Secure,
            (true, true) => sau::SauRegionAttribute::NonSecureCallable,
            (false, true) => sau::SauRegionAttribute::NonSecure,
        };

        Ok(sau::SauRegion {
            base_address : base_address << 5,
            limit_address: (limit_address << 5) | 0x1F,
            attribute,
        })

    }

}


/////////////////////////////////////////////////////
//   _____  _______      _______  _    _  _____    //
// /  ____||__   __| /\ |__   __|| |  | |/  ____|  //
// | (____    | |   /  \   | |   | |  | || (____   //
// \___   \   | |  / /\ \  | |   | |  | |\___   \  //
//  ____) |   | | / ____ \ | |   | |__| | ____) |  //
// |______/   |_|/_/    \_\|_|   \______/|______/  //
//                                                 //                        
/////////////////////////////////////////////////////

/// SauStatus is the software abstraction of the operational
/// status of the Sau.
/// The cortex-m SAU has eight regions, This struct caches the results
/// of region configuration. Its possible to create a new status, and
/// to modify it. This struct has been created to assign easly a status
/// to the physical SAU. This allows for state manipulation at a different
///time than the physical configuration assignment.

pub struct SauStatus<const NUM_REGIONS: usize> {

    regions: [sau::SauRegion; NUM_REGIONS],
    used: [bool; NUM_REGIONS],
}

impl<const NUM_REGIONS: usize> SauStatus<NUM_REGIONS> {

    pub fn new() -> Self {
        let regions = [
            sau::SauRegion {
                base_address: 0,
                limit_address: 0,
                attribute: sau::SauRegionAttribute::NonSecure,
            };
            NUM_REGIONS
        ];

        let used = [false; NUM_REGIONS];

        SauStatus { regions, used }
    }   
    
}



//////////////////////////////////////////////////////////
//    _  __                 _   _____         _ _       //
//   | |/ /___ _ _ _ _  ___| | |_   _| _ __ _(_) |_     //
//   | ' </ -_) '_| ' \/ -_) |   | || '_/ _` | |  _|    //
//   |_|\_\___|_| |_||_\___|_|   |_||_| \__,_|_|\__|    //
//                                                      //
//////////////////////////////////////////////////////////

/// SAU module must implement the SAU trait defined inside the Kernel.
/// The Kernel must provide a generic interface that can be specialized for a specific SAU.
/// The Kernel only implements (in platform/sau.rs) some local structure and then a large trait.
/// For more details about the trait functions semantics, check in kernel/src/plaftorm/sau.rs

impl<const NUM_REGIONS: usize> sau::SAU for Sau<NUM_REGIONS> {

    type SauStatus = SauStatus<NUM_REGIONS>;

    fn enable_sau(&self) {
        self.registers.sau_ctrl.write(SauCtrl::ENABLE::Enable)
    }
    
    fn disable_sau(&self) {
        self.registers.sau_ctrl.write(SauCtrl::ENABLE::Disable)
    }
    
    fn number_total_regions(&self)-> usize {
        return self.registers.sau_type.read(SauType::SREGION) as usize;
    }
    
    
    fn new_status(&self) -> Self::SauStatus {
        let status = Self::SauStatus::new();
        return status;
    }
    
    fn reset_status(&self, status: &mut Self::SauStatus) {
        
        for i in 0..NUM_REGIONS {
            status.used[i] = false;
        }
    }
    
    
    fn region_is_used(&self, status: &Self::SauStatus, region_number: usize) -> Option<bool> {
        if region_number < NUM_REGIONS {
            Some(status.used[region_number])
        } else {
            None
        }
    }

    fn set_region(&self, status: &mut Self::SauStatus, base_address: u32, limit_address: u32, attribute: sau::SauRegionAttribute, region_number: usize) -> Option<usize> {
        if region_number < NUM_REGIONS {
           status.regions[region_number] = sau::SauRegion {
                base_address: base_address,
                limit_address: limit_address,
                attribute: attribute,
            };
            status.used[region_number] = true;
            Some(region_number)
        } else {
            None
        }
    }

    fn reset_region(&self, status: &mut Self::SauStatus, region_number: usize) -> Option<usize> {
        if region_number < NUM_REGIONS {
            status.used[region_number] = false;
            status.regions[region_number] = sau::SauRegion {
                base_address: 0,
                limit_address: 0,
                attribute: sau::SauRegionAttribute::NonSecure,
            };
            Some(region_number)
        } else {
            None
        }
    }
    
    
    
    fn load_status(&mut self, status: &Self::SauStatus) -> Self::SauStatus {
        
        let mut retstatus = Self::SauStatus::new();
        
        for i in 0..NUM_REGIONS {
           
           if status.used[i] == true {
               let _ = self.set_region_intern(i as u8, status.regions[i]);
               retstatus.regions[i] = status.regions[i];
           }            
        }
        return retstatus;
    }
}




















