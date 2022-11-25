use scrypto::prelude::*;
#[allow(unused_imports)]
use crate::data::*;

// Swap tokens on an external DEX
pub fn swap_fx(
    sum: Decimal, 
    fx: ResourceAddress, 
    dex: ComponentAddress, 
    abc_bckt: Bucket
) -> Bucket {
    let method = "buy_token_sell_exact_token".to_string(); 
    let args = args![sum,fx,abc_bckt];

    borrow_component!(dex).call::<Bucket>(&method, args)
} 

// Test external marketplace buy allowance for royalties computation.
pub fn out_currency(ext_mrkt: ComponentAddress, bdg_bckt_ref: Proof) -> ResourceAddress {
    let method = "out_currency".to_string(); 
    let args = args![bdg_bckt_ref];
                
    borrow_component!(ext_mrkt).call::<ResourceAddress>(&method, args)
}   

pub fn abc_stock(
    royalty: Bucket,
    abc_vault: ComponentAddress
) -> Decimal {
    let method = "abc_stock".to_string(); 
    let arg = args![royalty];
        
    borrow_component!(abc_vault).call::<Decimal>(&method, arg)
}

// Everlock $RDS token dead share in Dead Vault Component 
pub fn abc_everlock(dead_bckt: Bucket, dead_vault: ComponentAddress){
    let method = "abc_everlock".to_string(); 
    let arg = args![dead_bckt];
    borrow_component!(dead_vault).call::<Decimal>(&method, arg);
} 

pub fn buy_nft_ext(
    sale_nr: u128, 
    mrkt: ComponentAddress,
    bckt: Bucket, 
    caller_bdg_bckt_ref: Proof
) -> (Vec<Bucket>,Bucket) {
    let method = "buy_nft_ext".to_string(); 
    let args = args![sale_nr,mrkt,bckt,caller_bdg_bckt_ref];     
                
    borrow_component!(mrkt).call::<(Vec<Bucket>,Bucket)>(&method, args)
}

pub fn buy_ticket_ext(
    sale_nr: u128, 
    mrkt: ComponentAddress, 
    bckt: Bucket, 
    sum: u8,
    caller_bdg_bckt_ref: Proof
) -> (Bucket,Bucket) {
    let method = "buy_ticket_ext".to_string(); 
    let args = args![sale_nr,mrkt,bckt,sum,caller_bdg_bckt_ref];                
                 
    borrow_component!(mrkt).call::<(Bucket,Bucket)>(&method, args)
}

pub fn place_bid_ext(
    sale_nr: u128, 
    mrkt: ComponentAddress, 
    bckt: Bucket, 
    bidder_badge: Bucket,
    bid: Decimal,
    bid_bond: Decimal,
    caller_bdg_bckt_ref: Proof
) -> (Bucket,Bucket,Bucket) {
    let method = "place_bid_ext".to_string(); 
    let args = args![sale_nr,mrkt,bckt,bidder_badge,bid,bid_bond,caller_bdg_bckt_ref];                
            
    borrow_component!(mrkt).call::<(Bucket,Bucket,Bucket)>(&method, args)
}

pub fn buy_prop_ext(
    sale_nr: u128, 
    mrkt: ComponentAddress, 
    bckt: Bucket, 
    proposal: Decimal,
    deadline: u64,
    caller_bdg_bckt_ref: Proof
) -> (Bucket,Bucket) {
    let method = "buy_proposal_ext".to_string(); 
    let args = args![sale_nr,mrkt,bckt,proposal,deadline,caller_bdg_bckt_ref];                
            
    borrow_component!(mrkt).call::<(Bucket,Bucket)>(&method, args)
}

pub fn pay_winner_bid(mrkt: ComponentAddress, bckt: Bucket, bdg: Bucket) -> (Vec<Bucket>,Bucket) {
    let method = "pay_winner_bid".to_string(); 
    let args = args![bckt,bdg];        

    borrow_component!(mrkt).call::<(Vec<Bucket>,Bucket)>(&method, args)
}

pub fn reclaim_bid_bond(mrkt: ComponentAddress, bidder_badge: Bucket) -> Vec<Bucket> {
    let method = "reclaim_bid_bond".to_string(); 
    let args = args![bidder_badge];                
            
    borrow_component!(mrkt).call::<Vec<Bucket>>(&method, args)
}

pub fn reclaim_winner_ticket(mrkt: ComponentAddress, ticket_badge: Bucket) -> Vec<Bucket> {
    let method = "reclaim_winner_ticket".to_string(); 
    let args = args![ticket_badge];                

    borrow_component!(mrkt).call::<Vec<Bucket>>(&method, args)
}

pub fn reclaim_buy_proposal(mrkt: ComponentAddress, ex_badge: Bucket) -> Vec<Bucket> {
    let method = "reclaim_buy_proposal".to_string(); 
    let args = args![ex_badge];

    borrow_component!(mrkt).call::<Vec<Bucket>>(&method, args)
}

pub fn reset_foo_values(
    foo_fee: Decimal, 
    foo_royalty: Decimal,
    abc_address: ResourceAddress,
    foo_square_comp: ComponentAddress,
    foo_vault_comp: ComponentAddress,
    foo_badge_proof: Proof,
    ext_square: ComponentAddress
) -> bool {
    let method = "set_foo_values".to_string(); 
    let args = args![
        foo_fee,
        foo_royalty,
        abc_address,
        foo_square_comp,
        foo_vault_comp,
        foo_badge_proof
    ];                
        
    borrow_component!(ext_square).call::<bool>(&method, args)
}

