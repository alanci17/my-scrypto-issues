use scrypto::prelude::*;
use crate::data::*;
#[allow(unused_imports)]
use crate::info::*;

blueprint! {
    struct List {
        // NFT sell instance data Hashmap:
        //   Key                    = (userBadge Address, instance number),
        //   Value.0                = (NFT resource address, NFT id, NFT data), 
        //   Value.1                = (Selling status flag, profit amount), 
        //   Value.2 (Normal mode)  = (price, buy offer amount, deadline, unused data(Decimal,u64,u8,u128)).
        //   Value.2 (Auction mode) = (reserve price, highest bid, deadline, bid bond, last minute bid deadline,
        //                           unused data(u8,u128)).
        //   Value.2 (Raffle mode)  = (reserve price, ticket price, deadline, unused data(Decimal), last minute bid deadline, 
        //                           tickets amount, winner ticket id).       
        list_map: HashMap<
            (ResourceAddress,u128),
            (
                Vec<(ResourceAddress,NonFungibleId,FooNFT)>,
                (u8,Decimal),
                (Decimal,Decimal,u64,Decimal,u64,u8,u128)
            )
        >,
    }

    #[allow(dead_code)]
    impl List {
        pub fn new() -> ListComponent {
            Self {
                list_map: HashMap::new()
            }
            .instantiate()
        }

        // Insert new Foo NFT selling instance data within Foo NFT selling instance data Hashmap
        // whenever a new selling instance is started
        pub fn map_insert(&mut self, tab: Tab) {
            let tup_one = tab.tuple.0;
            let tup_two = tab.tuple.1;
            self.list_map.insert(tup_one,tup_two);
        }

        // Retrieve instance number from related map invoked by seller's methods.
        // Chech made within Foo NFT selling instance data Hashmap.
        pub fn check_status(&mut self, nmbr: u128, flag: u8) {
            for (key,value) in self.list_map.iter() {
                if key.1 == nmbr {
                    match flag {
                        2 => if value.1.0 == 0 {
                            break;
                        }
                        _ => ()
                    }
                } 
            }
        }

        // Switch Badge addresses data between buyer/seller whenever a buy proposal has been accepted
        // or a raffle jackpot has been collected within Foo NFT selling instance data Hashmap.
        pub fn switch_badge(&mut self, bdg: ResourceAddress, flag: u8, nmbr: u128, profit: Decimal){
            let tab = Tab::new();
            let mut tup_two = tab.tuple.1;
            let mut founded = false;
            for (key,value) in self.list_map.iter_mut() {
                if key.1 == nmbr {
                    value.1.1 = profit;
                    match flag {
                        1 => value.1.0 = 11,
                        _ => value.1.0 = 2
                    }
                    tup_two = value.clone();
                    founded = true;
                    break;
                }
            }
            assert!(founded," Correspondence unfounded ");
            let tup_one = (bdg,nmbr);
            self.list_map.insert(tup_one,tup_two);
        } 

        // Check a badge existence within Foo NFT selling instance data Hashmap.
        pub fn check_badge(&mut self, bdg_addr: ResourceAddress) -> bool {                   
            for (key,_value) in self.list_map.iter() {
                if key.0 == bdg_addr {
                    return true;
                }    
            }     
            false
        } 

        // Retrieve raffle NFT winner within Foo NFT selling instance data Hashmap.
        pub fn raffle_winner(&mut self, nmbr: u128, val: u128) {
            for (key,value) in self.list_map.iter_mut() { 
                if key.1 == nmbr {
                    value.2.6 = val;
                }
            }
        }

        // Update NFT selling instance status within Foo NFT selling instance data Hashmap.
        // Method needs to be internally call by any other method able to modify selling instance 
        // conditions to reflect NFT selling instance actual status in terms of time and triggering 
        // his state transition at occurence.
        pub fn update(&mut self, nr: u128, bdg: ResourceAddress, dl: u64) -> (Vec<Tab>,Vec<u128>,Vec<u128>,u8) { 
            let mut v: Vec<Tab> = Vec::new();
            let mut nmbr_vec: Vec<u128> = Vec::new();   
            let mut switch_vec: Vec<u128> = Vec::new(); 
            let mut wave = false;       
            let mut tckt = 0;                      
            for (key,value) in self.list_map.iter_mut() { 
                if key.1 == nr && bdg == ResourceAddress::from(RADIX_TOKEN) || nr == 0 && bdg == key.0 {  
                    wave = true;
                    let now = Runtime::current_epoch();
                    let tckt_prc_dec = Decimal::from(value.2.5);
                    if now > value.2.2 {
                        match value.1.0 {
                            0 => {                                                      // NFT on Sell
                                    value.2.1 = dec!("0");
                                    value.2.2 = 0;
                                }
                            3 => {                                                     // NFT on Auction
                                    if value.2.1 >= value.2.0 {
                                        value.1.0 = 4;
                                    } else {
                                        value.1.0 = 7;
                                    }
                                    switch_vec.push(key.1); 
                                    if now > value.2.2+dl && value.2.1 >= value.2.0 {   
                                        value.1.0 = 5;
                                    }
                                }
                            4 => if now > value.2.2+dl && value.2.1 >= value.2.0 {
                                    value.1.0 = 5;
                                } 
                            8 => if value.2.1*tckt_prc_dec >= value.2.0 {               // NFT on Raffle
                                    value.1.0 = 9;
                                    tckt = value.2.5;
                                    nmbr_vec.push(key.1);
                                } else {
                                    value.1.0 = 10;
                                    switch_vec.push(key.1);
                                }
                            _ => ()
                        }
                    }
                    let tup_one = (key.0,key.1);
                    let tup_two = (
                        value.0.clone(),
                        (value.1.0,value.1.1),
                        (value.2.0,value.2.1,value.2.2,value.2.3,value.2.4,value.2.5,value.2.6)
                    );
                    let tab = Tab { tuple:(tup_one,tup_two)};  
                    v.push(tab);      
                    info!(" Panic? @ [crate::list::update] ");                            
                    match nr { 
                        0 => (),
                        _ => break
                    } 
                } 
            } 
            assert!(wave," Correspondence unfounded! ");
            (v,nmbr_vec,switch_vec,tckt) 
        }

        // Update NFT status within user Badge HashMap
        pub fn buy_nft(&mut self, sale_nr: u128, rest: Decimal) {
            let mut wave = false;
            for (key,value) in self.list_map.iter_mut() {
                if key.1 == sale_nr {    
                    value.1.0 = 1;
                    value.1.1 = rest;
                    wave = true;
                    break;
                }
            }
            assert!(wave," NFT correspondence unfounded! ");
        } 

        // Update NFT status within user Badge HashMap
        pub fn buy_ticket(&mut self, nmbr: u128, sum: u8, new_end: bool) -> u8 {
            let mut total_tckt = 0;
            let mut wave: bool = false;
            for (key,value) in self.list_map.iter_mut() {
                if key.1 == nmbr && Runtime::current_epoch() <= value.2.2 {
                    value.2.5 += sum;
                    total_tckt = value.2.5;
                    wave = true;
                    if new_end { 
                        value.2.2 += 1;
                    } 
                    break;                        
                }
            } 
            assert!(wave," Unable to Update NFT status ");

            total_tckt
        }

        // Update NFT selling instance status within Foo NFT selling instance data Hashmap. 
        // Foo NFT selling instance in normal mode
        pub fn buy_prop(&mut self, nmbr: u128, prop: Decimal, endtime: u64) -> (u64,u8) {
            let mut founded = false;
            let end = Runtime::current_epoch()+endtime;         
            let mut flag = 0;                                  
            for (key,value) in self.list_map.iter_mut() {
                if key.1 == nmbr {
                    if prop > value.2.1 {
                        value.2.1 = prop; 
                        value.2.2 = end;
                        flag = 1;
                    } else {
                        higher_amount(value.2.1);
                        break;
                    }  
                founded = true;
                break;
                }
            } 
            assert!(founded," Unable to update values ");

            (end,flag)
        }

        // Update NFT selling instance status within Foo NFT selling instance data Hashmap. 
        // Foo NFT selling instance in auction mode
        pub fn place_bid(&mut self, nmbr: u128, bid: Decimal, new_end: bool) -> bool {
            let mut wave: bool = false;
            for (key,value) in self.list_map.iter_mut() {
                if key.1 == nmbr && Runtime::current_epoch() <= value.2.2 {
                    if bid > value.2.1 {
                        value.2.1 = bid;
                            if new_end {
                                value.2.2 += 1;
                            } 
                        wave = true;
                    } else {
                        higher_amount(value.2.1);
                        break;
                    }  
                    break;
                }
            } 
            assert!(wave," Unable to Update NFT status ");

            wave
        }

        // Update NFT selling instance status within Foo NFT selling instance data Hashmap. 
        // Foo NFT selling instance retired, "unstock_nft" method called by NFT seller.
        pub fn unstock(&mut self, nr: u128) {
            for (key,value) in self.list_map.iter_mut() {
                if key.1 == nr {
                    value.1.0 = 12;
                    break; 
                }      
            }
        }

        // Update NFT selling instance status within Foo NFT selling instance data Hashmap. 
        // Foo NFT selling instance in auction mode
        pub fn pay_win_bid(&mut self, nmbr: u128, end: u64) -> Decimal {
            let mut rest = dec!("0");
            let mut wave = false;
            let now = Runtime::current_epoch();
            for (key,value) in self.list_map.iter_mut() {
                if key.1 == nmbr {
                    assert!(value.2.1 >= value.2.0," Reserve price unmatched ");
                    if now >= value.2.2+1 && now <= value.2.2+end && value.1.1 == dec!("0") {
                        rest = value.2.1-value.2.3;
                        value.1.0 = 6;
                        value.1.1 = value.2.1;
                        wave = true;
                        break;
                    }
                }    
            }
            assert!(wave," Check NFT data ");

            rest
        }
    }
}