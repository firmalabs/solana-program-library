mod ERC {
    
    use solana_program::{pubkey::Pubkey};
    
    trait ERC20 {
	// Optional methods
	
	// fn name(&self) -> String;
	// fn symbol(&self) -> String;
	// fn decimals(&self) -> u8;
	
	fn total_supply(&self) -> usize;
	
	fn balance_of(address: &Pubkey) -> usize;
	
	fn transfer(to: &Pubkey, value: usize) -> Result<(), Option<&str>>;
	
	fn transfer_from(from: &Pubkey, to: &Pubkey, value: usize) -> Result<(), Option<&str>>;

	fn approve(spender: &Pubkey, value: usize) -> Result<(), Option<&str>>;

	fn allowance(owner: &Pubkey, spender: &Pubkey) -> usize;
    }
}

