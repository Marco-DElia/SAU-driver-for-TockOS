/// tock/kernel/src/platform/sau.rs
 /// Possible attribute of a SAU region.
#[derive(Copy, Clone)]
 pub enum SauRegionAttribute {
     /// SAU region is Secure
    Secure,
    /// SAU region is Non-Secure Callable
    NonSecureCallable,
    /// SAU region is Non-Secure
    NonSecure,
 }
 
 /// Description of a SAU region.
 #[derive(Copy, Clone)]
 pub struct SauRegion {
    /// First address of the region, its 5 least significant bits must be set to zero.
    pub base_address: u32,
    /// Last address of the region, its 5 least significant bits must be set to one.
    pub limit_address: u32,
    /// Attribute of the region.
    pub attribute: SauRegionAttribute,
 }
 impl SauRegion {
    pub const fn new(base_address: u32, limit_address: u32, attribute:
    SauRegionAttribute)-> SauRegion {
    Self {
    base_address: base_address,
    limit_address: limit_address,
    attribute: attribute,
    }
    }
 }

 pub trait SAU {
    type SauStatus;
    fn enable_sau(&self);
    fn disable_sau(&self);
    fn number_total_regions(&self)-> usize;
    /// None of these five functions write to the hardware
    fn new_status(&self)-> Self::SauStatus;
    fn reset_status(&self, status: &mut Self::SauStatus);
    fn region_is_used(&self, status: &Self::SauStatus, region_number: usize)-> Option<bool>;
    fn set_region(&self, status: &mut Self::SauStatus, base_address: u32,
    limit_address: u32, attribute: SauRegionAttribute, region_number: usize)->
    Option<usize>;
    fn reset_region(&self, status: &mut Self::SauStatus, region_number: usize)-> Option<usize>;
    /// Commits the status to the hardware
    fn load_status(&mut self, status: &Self::SauStatus)-> Self::SauStatus;
 }