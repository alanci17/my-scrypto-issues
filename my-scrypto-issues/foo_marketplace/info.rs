use scrypto::prelude::*;
use crate::data::*;

pub fn amount(amount: Decimal){
    info!( " amount : {} ",amount);
}

pub fn collect_payment(accrued_tokens: Decimal, amount: Decimal){
    info!(" Accrued tokens {} ", accrued_tokens);   
    info!(" NFT accrued selling amount {} ", amount);
}

pub fn requested_amount(amount: Decimal){
    info!(" Requested amount: {} ", amount);
}

pub fn display_rest(rest: Decimal){
    info!(" Rest {} ", rest);
}

pub fn dex_output_amount(amount: Decimal){
    info!(" Foo Dex external currency output amount: {} ", amount);
}

pub fn net_gain(rest: Decimal){
    info!(" NFT sell net gain {} ",rest);
}    

pub fn net_fee(sum_one: Decimal, foo_royalty: Decimal){
    info!(" marketplace net fee {} ",sum_one);
    info!(" royalty {} ",foo_royalty);
}

pub fn royalty(foo_fee: Decimal){
    info!(" royalty {} ",foo_fee);
}

pub fn royalty_placed(amount: Decimal){
    info!(" $ABC royalty placed in RadiSquare vault {} ",amount);
}

pub fn instance_number(nmbr: u128){
    info!(" Instance number: {} ",nmbr);
}

pub fn bid_bond(bid_bond: Decimal){
    info!(" Bid bond: {} ",bid_bond);
}

pub fn higher_amount(amount: Decimal){
    info!(" Pls provide an amount higher then {}",amount);
}

pub fn unauthorized(){
    info!(" Auction is live or badge is unauthorized ");
}

pub fn time_unreached(ex_end: u64){
    info!(" Endtime unreached! {} ",ex_end);
}

pub fn unfound(flag: u8) {
    match flag {
        0 => info!(" Found no NFT in stock! "),
        1 => info!(" Could not find NFT in stock !"),
        2 => info!(" Unfound badge correspondence within map! "),
        3 => info!(" Unfounded correspondence "),
        7 => info!(" Unable to stock NFT "),
        _ => ()
    }
    if flag != 0 {
        std::process::abort()
    }
}

pub fn unfound_bckt(flag: u8) -> Bucket {
    match flag {
        4 => info!(" NFT not in stock! "),
        5 => info!(" Caller Badge not in stock! "),
        6 => info!(" Unavailable amount "),
        _ => ()
    }
    std::process::abort()
}

pub fn values(
    foo_fee: Decimal, 
    foo_royalty: Decimal, 
    abc_currency: ResourceAddress,
    foo_square: ComponentAddress,
    abc_vault: ComponentAddress
){
    info!(" FooSquare fee set to {}% ", foo_fee);
    info!(" Foo royalty fee set to {}% ", foo_royalty);
    info!(" ABC currency {} ", abc_currency);
    info!(" FooSquare component address set to {} ", foo_square);
    info!(" ABC Vault component address set to {} ", abc_vault);
}

pub fn dead_values(dead_vault: ComponentAddress, dead_share: Decimal) {
    info!(" Dead Vault Component Address set to {} ", dead_vault);
    info!(" Dead share fee set to {}% ", dead_share);
}

pub fn protocol_fee(fee: Decimal) {
    info!(" Protocol fee set to {}% ", fee);
}

pub fn deadlines(auction_dl: u64, last_bid_dl: u64, buy_proposal_dl: u64) {
    info!(" Auction deadline set to {} ", auction_dl);
    info!(" Auction last bid deadline set to {} ", last_bid_dl);
    info!(" Buy proposal deadline set to {} ", buy_proposal_dl);
}

pub fn buy_prop_badge_map(bpm: BuyPropTuple){
    info!("==========================");
    info!(
        "   Buyer Badge: 
        {}
            Proposal:        {} $ABC 
            End Time:        {} Epoch
            Higher proposal: {} $ABC 
            Caller Badge:       
        {} ",
        bpm.tuple.0,bpm.tuple.1,bpm.tuple.2,bpm.tuple.3,bpm.tuple.4
    );
    info!("==========================");
}

pub fn auction_badge_map(abm: AuctionTuple){
    info!("==========================");
    info!(
        "   Bidder Badge: 
        {}
            Bid:        {} $ABC 
            End Time:   {} Epoch
            Status:     {} 
            Bid Bond:   {} $ABC 
            Caller Badge:       
        {} ",
        abm.tuple.0,abm.tuple.1,abm.tuple.2,abm.tuple.3,abm.tuple.4,abm.tuple.5
    );
    info!("==========================");
}

pub fn raffle_badge_map(rbm: RaffleTuple){
    info!("==========================");
    info!(
        "   Ticket ID: {} 
            Ticket Badge: 
        {}
            Jackpot:        {} $ABC 
            End Time:       {} Epoch     
            Status:         {} 
            Caller Badge:       
        {} ",
        rbm.tuple.0,rbm.tuple.1,rbm.tuple.2,rbm.tuple.3,rbm.tuple.4,rbm.tuple.5
    );
    info!("==========================");
}

pub fn stock(
    amnt: Decimal, 
    addr: ResourceAddress, 
    e: String, 
    s: String, 
    nr: String, 
    key: NonFungibleId, 
    data: FooNFT, 
    price: Decimal
){
    info!(" Added {} NFT, {} ResAddress {} Ecosystem {} Series {} Number ",amnt,addr,e,s,nr);
    info!(" UUID: {} Traits: {} {} @{} $ABC ",key,data.data_1,data.data_2,price); 
}

pub fn nft_mint(foo_uuid: NonFungibleId, foo_addr: ResourceAddress){
    info!(" Foo_NFT_id {} ",foo_uuid);
    info!(" Foo_NFT_res_addr {} ",foo_addr);
    info!(" ======================================================================== ");
}

pub fn meta_mint(meta_uuid: NonFungibleId, meta_addr: ResourceAddress){
    info!(" meta_NFT_id {} ",meta_uuid);
    info!(" meta_NFT_res_addr {} ",meta_addr);
    info!(" ======================================================================== ");
}

pub fn index(index: usize, nmbr: u128){
    info!(" Index: {}",index);
    info!(" Instance number: {}",nmbr);
}

pub fn winner(id: u128, bdg: ResourceAddress){
    info!(" Winner ID: {}", id);
    info!(" Winner Badge: {}", bdg);
}

pub fn picked(addr: ResourceAddress, key: NonFungibleId){
    info!(" ============================================== ");
    info!(" NFT collected ");
    info!(" resource address: {} ",addr);
    info!(" key: {} ",key);
    info!(" ============================================== ");
}

pub fn position(tab: Tab, auction_dl: u64) {        
    let key = tab.tuple.0;
    let value = tab.tuple.1;
    let v = value.0.get(0).unwrap();

    info!(" NFT: {} ",&v.0);
    info!(" NFT key: {} ",&v.1);
    info!("--------------------------");
    match value.1.0 {    
        0 => info!(
            " NFT on Sell
            Instance number : {}
            Price: {} $ABC 
            Buy proposal: {} $ABC 
            Deadline: {}",
            key.1,value.2.0,value.2.1,value.2.2
        ),       
        1 => info!(
            " NFT Sold.
            Instance number : {} 
            Accrued profit: {} $ABC ",
            key.1,value.1.1
        ),
        2 => info!(
            " Buy proposal Accepted. 
            Instance number : {}
            Payed amount: {} $ABC ",
            key.1,value.2.1
        ),
        12 => info!(" NFT withdrawn from sale "),
        3 => info!(
            " NFT on Auction.
            Instance number : {} 
            Reserve price: {} $ABC 
            Highest bid: {} $ABC
            Deadline: {}
            Bid bond: {}
            Last minute bid war deadline: {} ",
            key.1,value.2.0,value.2.1,value.2.2,value.2.3,value.2.4
        ), 
        4 => info!(
            " Auction ended.
            Instance number : {} 
            Reserve price: {} $ABC
            Winning bid: {} $ABC
            Payment deadline: {} 
            Bid bond: {}",
            key.1,value.2.0,value.2.1,value.2.2+auction_dl,value.2.3
        ),
        5 => info!(
            " Auction ended. Payment deadline outdated.
            Instance number : {} 
            Reserve price: {} $ABC
            Winning bid: {} $ABC
            Bid bond penalty: {} $ABC
            To claim penalty start a new auction or unstock item ",
            key.1,value.2.0,value.2.1,value.2.3
        ),
        6 => info!(
            " Auction honored & payment withdrawable.
            Instance number : {} 
            Reserve price: {} $ABC
            Accrued amount: {} $ABC ",
            key.1,value.2.0,value.2.1
        ),
        7 => info!(
            " Auction ended. Reserve price unmatched.
            Instance number : {}
            Reserve price: {} $ABC
            Higher bid: {} $ABC
            deadline: {} 
            Start a new auction or a new raffle or unstock item ",
            key.1,value.2.0,value.2.1,value.2.2
        ),
        8 => info!(
            " NFT on Raffle.
            Instance number : {} 
            Reserve price: {} $ABC
            Jackpot: {} 
            Ticket price: {} $ABC 
            Deadline: {}
            Tickets sold: {}
            Last minute tickets fomo deadline: {} ",
            key.1,value.2.0,value.2.1*value.2.5,value.2.1,value.2.2,value.2.5,value.2.4
        ), 
        9 => info!(
            " Raffle ended. 
            Instance number : {}
            Reserve price: {} $ABC 
            Jackpot: {}
            Ticket price: {} $ABC 
            Deadline: {}
            Tickets sold: {}
            Winner ticket: {} ",
            key.1,value.2.0,value.2.1*value.2.5,value.2.1,value.2.2,value.2.5,value.2.4
        ),
        10 => info!(
            " Raffle ended. Reserve price unmatched.
            Instance number : {}
            Reserve price: {} $ABC 
            Jackpot: {} 
            Deadline: {}
            Start a new raffle or a new auction or unstock item ",
            key.1,value.2.0,value.2.1*value.2.5,value.2.2
        ),  
        11 => info!(
            " Raffle succesfully concluded.
            Instance number : {} 
            Jackpot: {} $ABC redeemed
            Deadline: {}
            Winning Ticket: {}
            Please claim won Nft ",
            key.1,value.2.1*value.2.5,value.2.2,value.2.6
        ),
        _ => std::process::abort()                
    }    
    info!("--------------------------");                
    info!(" {} ",&v.2.data_1);
    info!(" {} ",&v.2.data_2);
    info!(" {} ",&v.2.data_3);
    info!(" {} ",&v.2.data_4);
    info!(" Green boost: {} ",v.2.value_1);
    info!(" Energy:      {} ",v.2.value_2);
    info!(" Stamina:     {} ",v.2.value_3);
    info!("==========================");
    info!("==========================");        
}




