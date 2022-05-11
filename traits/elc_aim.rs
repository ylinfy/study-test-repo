#[brush::wrapper]
pub type ElcAimRef = dyn ElcAim;

#[brush::trait_definition]
pub trait ElcAim {
    #[ink(message)]
    fn get_elcaim(&self) -> u128;

    #[ink(message)]
    fn update_elcaim(&mut self, _cycle: u128) -> u128; 

    #[ink(message)]
    fn set_k(&mut self, _newk: u128);
}