use scrypto::prelude::*;

use crate::data::*;
#[allow(unused_imports)]
use crate::list::*;
use crate::calls::*;
use crate::foo_farm::*;
use crate::info::*;
use crate::maps::*;

blueprint! {
    struct FooSquare {        
        // abc vault.
        collected_abc: Vault,         
        // NFT Hashmap of vaults.                                   
        nft_vaults: HashMap<ResourceAddress,Vault>,   

        abc_vaults: HashMap<u128,Vault>,

        badge_vaults: HashMap<ResourceAddress,Vault>,

        // nft Hashmap with nft data, nft key, nft price. Used to return NFT availability within protocol.
        nft_map: HashMap<ResourceAddress,Vec<(u128,NonFungibleId,Decimal,bool)>>,

        // NFT Hashmap with address, total accrued selling amount, NFT & metaNFT keys, NFT accrued selling amount.
        meta_map: HashMap<ResourceAddress,(Decimal,Vec<(NonFungibleId,NonFungibleId,Decimal,u128)>)>,         
        // metanft Hashmap with NFT Address, metaNFT ResourceDef & Data. 
        meta: HashMap<ResourceAddress,ResourceAddress>,  

        ext_mrkt_map: Vec<(ComponentAddress,ResourceAddress,Decimal,ResourceAddress,ResourceAddress)>,
        
        // Badge to mint and burn metaCandies.                      
        minter_badge: Vault,         
        // Owner badge to determine protocol fee and collect accrued abc fee.                                     
        owner_badge: ResourceAddress, 

        foo_badge: ResourceAddress, 

        abc_fee: Decimal, 

        abc_claimed: Decimal,

        currency: ResourceAddress,

        foo_comp_addr: ComponentAddress,
        // Protocol fee variable.
        fee: Decimal,

        maps: MapsComponent,

        list: ListComponent,

        abc: Abc,

        foo_farm: FooFarmComponent,

        instance_number: u128,

        list_map: HashMap<
            (ResourceAddress,u128),
            (
                Vec<(ResourceAddress,NonFungibleId,FooNFT)>,
                (u8,Decimal),
                (Decimal,Decimal,u64,Decimal,u64,u8,u128)
            )
        >
    }

    impl FooSquare {
        pub fn new(
            fee: Decimal, 
            currency: ResourceAddress,
            dex: ComponentAddress
        ) -> (ComponentAddress,Bucket,Bucket) {
            let minter_badge = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_NONE)
                .metadata("name", " MinterBadge ")
                .initial_supply(Decimal::one());

           let owner_badge = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_NONE)
                .metadata("name", " OwnerBadge ")
                .initial_supply(Decimal::one());

            let foo_badge = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_NONE)
                .metadata("name", " FooBadge ")
                .initial_supply(Decimal::one()); 

            let access_rules = AccessRules::new()   
                .default(rule!(allow_all));
           
            let mut abc_square: FooSquareComponent = Self {
                collected_abc: Vault::new(currency.clone()),
                nft_vaults: HashMap::new(),
                abc_vaults: HashMap::new(),
                badge_vaults: HashMap::new(),
                nft_map: HashMap::new(),
                meta_map: HashMap::new(),
                meta: HashMap::new(),
                ext_mrkt_map: Vec::new(),
                minter_badge: Vault::with_bucket(minter_badge),
                owner_badge: owner_badge.resource_address(),
                foo_badge: foo_badge.resource_address(),
                abc_fee:  Decimal::zero(), 
                abc_claimed: Decimal::zero(),
                currency,
                foo_comp_addr: dex,
                fee, 
                maps: MapsComponent::new(),
                list: ListComponent::new(),
                abc: Abc::new(dec!("0"),dec!("0"),dex,currency),
                foo_farm: FooFarmComponent::new(),
                instance_number: 1,
                list_map: HashMap::new()
            }
            .instantiate();
            abc_square.add_access_check(access_rules);
            
            (abc_square.globalize(),owner_badge,foo_badge)
        }

            // Mint one Foo NFT.
        pub fn mint_nft(&mut self) -> Bucket {
            
            self.foo_farm.nft_mint() 
        }

            // Stock NFT function callable by an end user wishing to list a NFT to protocol.
        pub fn stock_nft(
            &mut self, 
            nft_bckt_sum: Vec<Bucket>,
            badge: Bucket,
            price: Decimal,
            auction: bool,
            raffle: bool,
            reserve_prc: Decimal,
            time: u64,
            bid_bond: Decimal,
            start_prc: Decimal
        ) -> (Vec<Bucket>,Bucket,Bucket) {
            let bdg_addr = badge.resource_address();            
            let nmbr = self.instance_number; 
            self.instance_number += 1;  
            instance_number(nmbr);                 

            let mut meta_bckt: Vec<Bucket> = Vec::new();
            let badge_evo: Bucket;
            let mut tab = Tab::new();
            let mut vec_data = tab.tuple.1.0.clone();
            let mut check_auction_map = false;
            let mut check_raffle_map = false;
            let mut j = 0;
            for nft in nft_bckt_sum.into_iter() {
                let addr = nft.resource_address();
                let key = nft.non_fungible::<FooNFT>().id();
                let amnt = nft.amount();
                if j == 0 {
                    vec_data.clear();
                }

                assert!(!auction && !raffle || auction && !raffle || !auction && raffle," Check data ");
                if !auction && !raffle {
                    assert!(price > Decimal::zero()," New price must be a positive value ");
                } else {
                    assert!(price == Decimal::zero()," In selected mode NFT price must be set to zero ");
                    assert!(time <= self.abc.auction_dl," Check auction duration ");
                    if auction && !raffle {
                        assert!(bid_bond <= reserve_prc/Decimal::from("5")," Max bid bond allowed is 20% reserve price ");
                    } else {
                        assert!(start_prc <= reserve_prc/Decimal::from("100")," A minimum of 100 tickets are required within a raffle instance ");
                    }
                }
                let eco = borrow_resource_manager!(nft.resource_address()).metadata()["Ecosystem"].clone();
                let series = borrow_resource_manager!(nft.resource_address()).metadata()["Series"].clone();
                let nr = borrow_resource_manager!(nft.resource_address()).metadata()["Number"].clone();
            
                // Retrieve NFT data
                let data: FooNFT = nft.non_fungible().data();
                vec_data.push((addr,key.clone(),data.clone()));

                let mut v: Vec<(u128,NonFungibleId,Decimal,bool)> = Vec::new();
    
                if self.nft_map.contains_key(&addr) {
                    match self.nft_map.get_mut(&addr) {
                        Some(v) => {
                            v.push((nmbr,key.clone(),price,true));
                            let vault = self.nft_vaults.get_mut(&addr).unwrap();
                            vault.put(nft);
                            check_auction_map = true;
                            check_raffle_map = true;
                        }
                        _ => unfound(7)
                    }
                } else {
                    let vault = self.nft_vaults.entry(addr).or_insert(Vault::new(addr));
                    vault.put(nft);

                    v.push((nmbr,key.clone(),price,true));
                    self.nft_map.insert(addr,v);

                    let mut v_key: Vec<(NonFungibleId,NonFungibleId,Decimal,u128)> = Vec::new();
                    v_key.push((key.clone(),key.clone(),dec!("0"),nmbr));
             
                    self.meta_map.entry(addr) 
                                 .and_modify(|z| z.1.append(&mut v_key))                     
                                 .or_insert((Decimal::zero(),v_key));          

                    self.add_meta_nft(eco.clone(), series.clone(), nr.clone(), addr);
                }
                stock(amnt, addr, eco, series, nr, key.clone(), data.clone(), price);
                
                // Mint metaNFT and insert relative hashmaps values.
                let meta_nft = self.meta_mint(addr, data, key, nmbr);

                meta_bckt.push(meta_nft);
                j += 1;
            }
            self.abc_vaults.insert(nmbr,Vault::new(self.currency));

            // Check and update same NFT past auction or past raffle data within relative maps 
            if check_auction_map && auction {   
                self.update_auction_map(nmbr);
            } else if check_raffle_map && raffle { 
                self.update_raffle_map(nmbr); 
            }

            let end = Runtime::current_epoch()+time;
            let dl =end+self.abc.last_bid_dl;
            let mut tup_two = tab.tuple.1;
                
            //if self.list.check_badge(bdg_addr) {  
            if self.check_badge_list(bdg_addr) {                 
                let tup_one = (bdg_addr, nmbr);
                tup_two.0 = vec_data.clone(); 
                if !auction && !raffle {  
                    tup_two.2 = (price,dec!("0"),0,dec!("0"),0,0,0);  
                } else if !raffle {
                    tup_two.1.0 = 3;  
                    tup_two.2 = (reserve_prc,start_prc,end,bid_bond,dl,0,0);     
                } else {  
                    tup_two.1.0 = 8;
                    tup_two.2 = (reserve_prc,start_prc,end,dec!("0"),dl,0,0);
                }  
                tab = Tab { tuple:(tup_one,tup_two)};
                //self.list.map_insert(tab);
                self.map_insert_list(tab);
                badge_evo = Bucket::new(RADIX_TOKEN);
            } else {   
                badge_evo = self.add_badge(
                    vec_data.clone(), 
                    price,
                    auction,
                    raffle,
                    reserve_prc,
                    start_prc,
                    end,
                    bid_bond,
                    dl,
                    nmbr
                );
            }

            (meta_bckt,badge,badge_evo)
        }

            // Unstock NFT function callable by an end user wishing to withdraw owned NFT from protocol.
        pub fn unstock_nft(&mut self, meta_nft_bckt_sum: Vec<Bucket>) -> (Vec<Bucket>,Bucket) {
            let tup = self.check_meta(meta_nft_bckt_sum,true,0,0,dec!("0"));
            let chk = self.status_check(tup.m.3);
            assert!(chk.t.0 == 0 || chk.t.0 == 5 || chk.t.0 == 7 || chk.t.0 == 10," Unauthorized operation ");           
            //self.list.unstock(tup.m.3);
            self.unstock_list(tup.m.3);

            // Erase relative hashmaps values from map & collect NFT 
            let mut nft_bckt: Vec<Bucket> = Vec::new();
            for (nft_address,vec_key) in tup.m.0.into_iter() {
                for key in vec_key {
                    picked(nft_address,key.clone());
                    nft_bckt.push(self.nft_take(nft_address,key,false)); 
                }
            }    

            (nft_bckt,self.bid_bond(chk.t.0,chk.t.4,chk.t.1,chk.t.2,tup.m.3))
        }

            // Method callable by NFT provider to collect payment received by NFT selling
            // providing a relative user Badge as reference.
        pub fn collect_payment(&mut self, meta_nft_bckt_sum: Vec<Bucket>) -> Bucket {
            let tup = self.check_meta(meta_nft_bckt_sum,false,0,0,dec!("0"));
            let chk = self.status_check(tup.m.3);        
            assert!(chk.t.0 == 1," Unauthorized operation "); 
            let accrued_token_bckt = self.abc_vaults.get(&tup.m.3).unwrap();
            collect_payment(accrued_token_bckt.amount(),chk.t.9);

            self.abc_collect(tup.m.3, chk.t.9)
        }

        pub fn collect_buy_prop_payment(&mut self, meta_vec: Vec<Bucket>, sum: Decimal) -> Bucket { 
            let tup = self.check_meta(meta_vec,false,0,0,dec!("0"));               
            let chk = self.status_check(tup.m.3);
            assert!(chk.t.0 == 0," Unauthorized operation ");        

            let (caller_bdg_addr,new_badge,max_value) = self.maps.collect_buy_proposal(tup.m.3,sum);

            self.switch_badge(new_badge,0,tup.m.3,dec!("0"));
            self.switch_status(tup.m.3,0);

            // Collect $ABC from Vault and hold back Protocol fees
            let abc_bckt = self.abc_collect(tup.m.3, max_value);
            let (_rest,output_bckt) = self.take_fee(tup.m.3,max_value,abc_bckt,caller_bdg_addr,0);

            output_bckt
        }

        pub fn collect_auction_payment(&mut self, meta_nft_bckt_sum: Vec<Bucket>) -> Bucket {                                        
            let tup = self.check_meta(meta_nft_bckt_sum,false,0,0,dec!("0"));
            let chk = self.status_check(tup.m.3);
            assert!(chk.t.0 == 6," Unauthorized operation ");    

            let caller_badge_addr = self.maps.collect_auction_payment(tup.m.3);   

            // Collect $ABC from Vault and hold back Protocol fees
            let abc_bckt = FooSquare::abc_collect(self, tup.m.3, chk.t.2);
            let (_rest,output_bckt) = 
                self.take_fee(tup.m.3,chk.t.2,abc_bckt,caller_badge_addr,0);

            output_bckt
        }

        pub fn collect_raffle_jackpot(&mut self, meta_nft_bckt_sum: Vec<Bucket>) -> Bucket {
            let tup = self.check_meta(meta_nft_bckt_sum,false,0,0,dec!("0"));
            let chk = self.status_check(tup.m.3);
            assert!(chk.t.0 == 9," Unauthorized operation ");    
            amount(chk.t.7);

            let (caller_badge_addr,new_badge,flag) = self.maps.collect_jackpot(tup.m.3);            
            
            if flag == 3 {  
                 self.erase_map_entry(tup.m.3, chk.t.3, 0);
            } else if flag == 1 {

                // Switch Badge addresses data within user Badge HashMap & update NFT selling status
                self.switch_badge(new_badge,1,chk.t.8,dec!("0"));
            } else if flag == 2 { 
                self.erase_map_entry(tup.m.3, 0, 4);
            }

            // Collect $ABC from Vault and hold back Protocol fees
            let abc_bckt = self.abc_collect(tup.m.3, chk.t.7);
            let (_rest,output_bckt) = 
                self.take_fee(tup.m.3,chk.t.7,abc_bckt,caller_badge_addr,0);
            
            output_bckt
        }

        pub fn buy_nft_ext(    
            &mut self,       
            sale_nr: u128,
            mrkt_addr: ComponentAddress,
            abc_bckt: Bucket,
            bdg_ref: Proof
        ) -> (Vec<Bucket>,Bucket) {
            if !self.nft_match(sale_nr,false).n.0 {
                let (extmrkt,caller_badge,dex) = self.check_buy(mrkt_addr);
                let bckt = self.bckt_fx(dec!("0"),extmrkt.tuple.3,dex,abc_bckt);
                dex_output_amount(bckt.amount().clone());
                let proof = caller_badge.create_proof();
                let (a,b) = buy_nft_ext(sale_nr,extmrkt.tuple.0,bckt,proof);
                self.caller_bdg_put(caller_badge);

                (a,b)
            } else {

                self.buy_nft(sale_nr,abc_bckt,bdg_ref)
            }
        } 

            // Obtain an exact nft amount in exchange of a maximum abc amount. 
            // Function swap abc for exact nft.
        pub fn buy_nft(&mut self, sale_nr: u128, mut abc_bckt: Bucket, bdg_ref: Proof) -> (Vec<Bucket>,Bucket) { 
            let matched = self.nft_match(sale_nr,false);
            assert_eq!(matched.n.0,true);  
            let chk = self.check_status(sale_nr,2);
            assert!(chk.t.0 == 0," NFT not on sell ");
            let bdg_ref: ValidatedProof = bdg_ref.unsafe_skip_proof_validation();
            
            let mut bdg_addr = ResourceAddress::from(RADIX_TOKEN);  
            if abc_bckt.resource_address() != self.currency {
                abc_bckt = self.swap_fx(matched.n.2,self.currency,self.abc.dex,abc_bckt);
                bdg_addr = bdg_ref.resource_address();
            } 
            requested_amount(matched.n.2);      
            assert!( matched.n.2 <= abc_bckt.amount(), " Not enough abc input ");
            
            let (rest,abc_bckt) = self.take_fee(sale_nr,matched.n.2,abc_bckt,bdg_addr,1);
            display_rest(rest);
                
            // Update NFT status within user Badge HashMap
            //self.list.buy_nft(sale_nr,rest);
            self.buy_nft_list(sale_nr,rest);
                
            let mut output_vec_bckt: Vec<Bucket> = Vec::new(); 
            let addr_key_map = self.check_meta_id(sale_nr);
            for (nft_address,vec_key) in addr_key_map.into_iter() {
                for key in vec_key {
                    picked(nft_address,key.clone());
                    output_vec_bckt.push(self.nft_take(nft_address, key, true));
                }
            } 

            (output_vec_bckt,abc_bckt) 
        }

        pub fn buy_proposal_ext(
            &mut self, 
            sale_nr: u128,
            mrkt_addr: ComponentAddress,
            abc_bckt: Bucket,
            prop: Decimal,
            endtime: u64,
            bdg_ref: Proof
        ) -> (Bucket,Bucket) {
            if !self.nft_match(sale_nr,false).n.0 {
                let (extmrkt,caller_badge,dex) = self.check_buy(mrkt_addr);
                let bckt = self.bckt_fx(prop,extmrkt.tuple.3,dex,abc_bckt);
                dex_output_amount(bckt.amount().clone());
                let proof = caller_badge.create_proof();
                let (a,b) = buy_prop_ext(sale_nr,extmrkt.tuple.0,bckt,prop,endtime,proof);
                self.caller_bdg_put(caller_badge);

                (a,b)
            } else {

                self.buy_proposal(sale_nr, abc_bckt, prop, endtime, bdg_ref)
            }          
        } 

        pub fn buy_proposal(
            &mut self, 
            sale_nr: u128,
            mut abc_bckt: Bucket,
            prop: Decimal,
            endtime: u64,
            bdg_ref: Proof
        ) -> (Bucket,Bucket) {
            assert_eq!(self.nft_match(sale_nr,false).n.0,true); 
            assert!(endtime <= self.abc.buy_prop_dl, " Please provide a valid deadline! "); 
            let chk = self.check_status(sale_nr,2);                                         
            assert!(chk.t.0 == 0," NFT not on sell anymore ");
            let bdg_ref: ValidatedProof = bdg_ref.unsafe_skip_proof_validation();
            assert!(bdg_ref.amount() == dec!("1")," Badge proof check failed ");

            // Check if provided currency is requested one otherwise swap it
            if abc_bckt.resource_address() != self.currency {
                abc_bckt = self.swap_fx(prop,self.currency,self.abc.dex,abc_bckt);
            } 

            // Update NFT status within user Badge HashMap
            //let (end,flag) = self.list.buy_prop(chk.t.8,prop,endtime);
            let (end,flag) = self.buy_prop_list(chk.t.8,prop,endtime);

            // Update current higher proposal in related map resetting previous proposal values.
            // Return related badge resource address if call is made by an external marketplace.
            // Return a Badge if proposal is acceptable otherwise return an empty bucket.
            let out_bckt: Bucket;
            let mut bdg_addr = ResourceAddress::from(RADIX_TOKEN);
            if flag == 1 {  
                self.maps.buy_proposal_ext(chk.t.8);
                let extmrkt = self.ext_mrkt_data(self.abc.dex,bdg_ref.resource_address());
                if extmrkt.tuple.4 {    
                    bdg_addr = extmrkt.tuple.1; 
                } 
                out_bckt = self.add_buyer_badge(prop,dec!("0"),end,0,dec!("0"),bdg_addr,flag,chk.t.8);

                // Put amount of NFT Buy Proposal into related vault.
                self.abc_put(sale_nr, abc_bckt.take(prop));
            } else {
                out_bckt = Bucket::new(RADIX_TOKEN);
            }
            
            (out_bckt,abc_bckt)      
        } 

        pub fn reclaim_buy_proposal(&mut self, ex_badge: Bucket) -> Vec<Bucket> { 
            let (ex_badge,nmbr,mrkt_addr) = FooSquare::buy_bdg_data(ex_badge); 
            if self.badge_in(nmbr,ex_badge.resource_address(),0) { 
                let ex_badge_addr = ex_badge.resource_address();
                let mut output_vec_bckt: Vec<Bucket> = Vec::new();
                let (ex_flag,ex_amnt,ex_end) = self.maps.reclaim_prop(nmbr,ex_badge_addr);
                match ex_flag { 
                    0 => {  
                        self.bckt_burn(ex_badge);
                        output_vec_bckt.push(self.abc_collect(nmbr, ex_amnt));
                    }
                    1 => {
                        if ex_end < Runtime::current_epoch() {
                            self.bckt_burn(ex_badge);
                            output_vec_bckt.push(self.abc_collect(nmbr, ex_amnt));
                        } else {
                            time_unreached(ex_end);
                            std::process::abort()
                        }
                    }
                    2 => {  
                        // Retrieve instance number, check correspondence & take NFT from relative maps 
                        let (ex_badge,nmbr,_nft_addr) = FooSquare::buy_bdg_data(ex_badge);
                        let addr_key_map = self.check_meta_id(nmbr);
                        for (nft_address,vec_key) in addr_key_map.into_iter() {
                            for key in vec_key {
                                picked(nft_address,key.clone());
                                output_vec_bckt.push(self.nft_take(nft_address,key,true)); 
                            }
                        } 

                        // Burn provided Badge
                        self.bckt_burn(ex_badge);
                    }
                    _ => unfound(2)
                }

                // Remove related NFT buy_proposals from map once verified condition.
                self.maps.remove_prop(nmbr,ex_badge_addr);

                output_vec_bckt
            } else { 
                let (extmrkt,dex) = self.check_rec(mrkt_addr); 
                let vec_bckt = reclaim_buy_proposal(extmrkt.tuple.0,ex_badge); 
                
                self.swap(extmrkt,vec_bckt,dex) 
            }          
        }

        pub fn buy_ticket_ext(
            &mut self, 
            sale_nr: u128,
            mrkt_addr: ComponentAddress,
            abc_bckt: Bucket,
            sum: u8,
            bdg_ref: Proof
        ) -> (Bucket,Bucket) {
            if !self.nft_match(sale_nr,false).n.0 {
                let (extmrkt,caller_badge,dex) = self.check_buy(mrkt_addr);
                let bckt = self.bckt_fx(dec!("0"),extmrkt.tuple.3,dex,abc_bckt);
                dex_output_amount(bckt.amount().clone()); 
                let proof = caller_badge.create_proof();
                let (a,b) = buy_ticket_ext(sale_nr,extmrkt.tuple.0,bckt,sum,proof);
                self.caller_bdg_put(caller_badge);

                (a,b)
            } else {

                self.buy_ticket(sale_nr, abc_bckt, sum, bdg_ref)
            }          
        } 

        pub fn buy_ticket(
            &mut self, 
            sale_nr: u128,
            mut abc_bckt: Bucket,
            sum: u8,
            bdg_ref: Proof
        ) -> (Bucket,Bucket) { 
            assert_eq!(self.nft_match(sale_nr,false).n.0,true);
            let mut chk = self.check_status(sale_nr,1);
            assert!(chk.t.0 == 8," NFT not on Raffle ");
            let bdg_ref: ValidatedProof = bdg_ref.unsafe_skip_proof_validation();
            assert!(bdg_ref.amount() == dec!("1")," Badge proof check failed ");

            // Check if provided currency is requested one otherwise swap it
            if abc_bckt.resource_address() != self.currency {
                abc_bckt = self.swap_fx(chk.t.2*sum,self.currency,self.abc.dex,abc_bckt);
            }   
            let amnt = abc_bckt.amount();  
            assert!(amnt/sum >= chk.t.2," Check $ABC amount provided "); 

            // Increase Raffle deadline by an Epoch if tickets are purchased within last valid Epoch as
            // long as auction deadline limit ain't outdated
            let mut new_end: bool = false;
            if Runtime::current_epoch() == chk.t.3 && Runtime::current_epoch() < chk.t.5 {
                if sum >= chk.t.6.wrapping_div(20) { 
                    chk.t.3 += 1;
                    new_end = true;
                } 
            } 

            // Update current raffle data in related map and check if ticket badge is present.
            // Return related badge resource address if call is made by an external marketplace.
            // Return a Badge if tickets order is acceptable otherwise return an empty bucket.
            let sum_dec = Decimal::from(sum);
            //let ttl_dec = Decimal::from(self.list.buy_ticket(chk.t.8,sum,new_end));
            let ttl_dec = Decimal::from(self.buy_ticket_list(chk.t.8,sum,new_end));
            self.maps.buy_ticket_ext(sale_nr,ttl_dec,chk.t.3,new_end);  
            let output_bckt: Bucket;
            let mut bdg_addr = ResourceAddress::from(RADIX_TOKEN);
            let extmrkt = self.ext_mrkt_data(self.abc.dex,bdg_ref.resource_address());
            if extmrkt.tuple.4 {    
                bdg_addr = extmrkt.tuple.1; 
            } 
            output_bckt = self.add_buyer_badge(sum_dec,ttl_dec,chk.t.3,2,chk.t.4,bdg_addr,0,chk.t.8);

            // Put amount of NFT Raffles Tickets purchase into related vault.
            FooSquare::abc_put(self, sale_nr, abc_bckt.take(sum_dec*chk.t.2));
                
            (output_bckt,abc_bckt)
        } 

        pub fn reclaim_winner_ticket(&mut self, ticket_badge: Bucket) -> Vec<Bucket> { 
            let (ticket_badge,nmbr,mrkt_addr) = FooSquare::raffle_bdg_data(ticket_badge);
            if self.badge_in(nmbr,ticket_badge.resource_address(),2) {
                let chk = self.status_check(nmbr);
                assert!(chk.t.0 >= 9," Unauthorized operation ");
                let mut output_vec_bckt: Vec<Bucket> = Vec::new();

                let (wave,sum,tup) = self.maps.reclaim_ticket(nmbr,ticket_badge.resource_address(),chk.t.2);
                match wave {
                    0 => {
                        if chk.t.2*Decimal::from(chk.t.6) < chk.t.1 {
                            self.bckt_burn(ticket_badge);
                            output_vec_bckt.push(self.abc_collect(nmbr, sum));
                        } else {
                            self.bckt_burn(ticket_badge);
                            output_vec_bckt.push(Bucket::new(RADIX_TOKEN));
                        }
                    }
                    _ => {
                        if tup.0 != 0 {
                            let mut v: Vec<(u128,ResourceAddress,Decimal,u64,u8,ResourceAddress)> = Vec::new();
                            v.push(tup);
                            self.maps.raffle_map_insert(nmbr,v);
                        }
                        let addr_key_map = self.check_meta_id(nmbr);
                        for (nft_address,vec_key) in addr_key_map.into_iter() {
                            for key in vec_key {
                                picked(nft_address,key.clone());
                                output_vec_bckt.push(self.nft_take(nft_address,key,true)); 
                            }
                        } 

                        // Burn provided Badge
                        self.bckt_burn(ticket_badge);
                    }
                }

                output_vec_bckt
            } else {
                let (extmrkt,dex) = self.check_rec(mrkt_addr);
                let mut vec_bckt = reclaim_winner_ticket(extmrkt.tuple.0,ticket_badge);
                if vec_bckt.get(0).unwrap().resource_address() != ResourceAddress::from(RADIX_TOKEN) {
                    vec_bckt = self.swap(extmrkt,vec_bckt,dex);
                }

                vec_bckt
            }          
        }

        pub fn place_bid_ext(
            &mut self, 
            sale_nr: u128,
            mrkt_addr: ComponentAddress,
            abc_bckt: Bucket,
            bidder_badge: Bucket,
            bid: Decimal,
            bid_bond: Decimal,
            bdg_ref: Proof
        ) -> (Bucket,Bucket,Bucket) {
            if !self.nft_match(sale_nr,false).n.0 {
                let (extmrkt,caller_badge,dex) = self.check_buy(mrkt_addr);
                let bckt = self.bckt_fx(bid_bond,extmrkt.tuple.3,dex,abc_bckt);
                dex_output_amount(bckt.amount().clone());
                let proof = caller_badge.create_proof();
                let (a,b,c) = place_bid_ext(sale_nr,extmrkt.tuple.0,bckt,bidder_badge,bid,bid_bond,proof);
                self.caller_bdg_put(caller_badge);

                (a,b,c)
            } else {

                self.place_bid(sale_nr,abc_bckt,bidder_badge,bid,bid_bond,bdg_ref)
            }          
        }   
 
        pub fn place_bid(
            &mut self, 
            sale_nr: u128,
            mut abc_bckt: Bucket,
            bidder_badge: Bucket,
            bid: Decimal,
            bid_bond: Decimal,
            bdg_ref: Proof
        ) -> (Bucket,Bucket,Bucket) {
            assert_eq!(self.nft_match(sale_nr,false).n.0,true); 
        
            // Check if NFT is listed in auction mode & provided resources and data are valid
            let mut chk = self.check_status(sale_nr,0);
            assert!(chk.t.0 == 3," NFT not in Auction ");
            assert!(abc_bckt.amount() >= chk.t.4," Check bid bond amount ");  
            assert!(bid > chk.t.2," An higher bid has been placed yet ");
            let bdg_ref: ValidatedProof = bdg_ref.unsafe_skip_proof_validation();
            assert!(bdg_ref.amount() == dec!("1")," Badge proof check failed ");

            // Check if provided currency is requested one otherwise swap it
            if abc_bckt.resource_address() != self.currency {
                abc_bckt = self.swap_fx(bid_bond,self.currency,self.abc.dex,abc_bckt);
            }
            
            // Increase Auction deadline by an Epoch if bid is placed within last valid Epoch as
            // long as auction deadline limit ain't outdated
            let mut new_end: bool = false;
            if Runtime::current_epoch() == chk.t.3 && Runtime::current_epoch() < chk.t.5 {
                chk.t.3 += 1;
                new_end = true; 
            } 
    
            // Update NFT status within user Badge HashMap
            //let wave = self.list.place_bid(chk.t.8,bid,new_end);
            let wave = self.place_bid_list(chk.t.8,bid,new_end);

            let mut s = 0;
            if bid >= chk.t.1 {
                s = 1;
            }
                  
            // Update current winning bid in related map and check if bidder badge is present 
            // Return related badge resource address if call is made by an external marketplace.
            // Return a Badge if bid is acceptable otherwise return an empty bucket. 
            let output_bckt: Bucket;
            let mut bdg_addr = ResourceAddress::from(RADIX_TOKEN);
            if self.maps.place_bid(chk.t.8,bidder_badge.resource_address(),s,bid,new_end,wave) {  
                let extmrkt = self.ext_mrkt_data(self.abc.dex,bdg_ref.resource_address()); 
                if extmrkt.tuple.4 {    
                    bdg_addr = extmrkt.tuple.1; 
                } 
                output_bckt = 
                    self.add_buyer_badge(bid,dec!("0"),chk.t.3,1,chk.t.4,bdg_addr,s,chk.t.8);

            // Put amount of NFT Placed Bid into related vault.
                self.abc_put(sale_nr, abc_bckt.take(chk.t.4));
            } else {
                output_bckt = Bucket::new(RADIX_TOKEN);
            }
            
            (output_bckt,abc_bckt,bidder_badge)  
        }

        pub fn reclaim_bid_bond(
            &mut self, 
            bidder_badge: Bucket
        ) -> Vec<Bucket> {
            let (bidder_badge,nmbr,mrkt_addr) = FooSquare::buy_bdg_data(bidder_badge);
            if self.badge_in(nmbr,bidder_badge.resource_address(),1) {
                let mut output_vec_bckt: Vec<Bucket> = Vec::new();
                let (bid_bond,answer,winner_flag,burn_badge_flag) = 
                    self.maps.reclaim_bond(nmbr,bidder_badge.resource_address(),self.abc.auction_dl);
                if answer { 
                    self.bckt_burn(bidder_badge);
                    output_vec_bckt.push(self.abc_collect(nmbr, bid_bond));
                } else if !winner_flag || !burn_badge_flag {
                    if !winner_flag {
                        unauthorized();
                    }
                    output_vec_bckt.push(bidder_badge);
                } else {
                    self.bckt_burn(bidder_badge);
                    output_vec_bckt.push(Bucket::new(RADIX_TOKEN));
                }

                output_vec_bckt
            } else {
                let (extmrkt,dex) = self.check_rec(mrkt_addr);         
                let vec_bckt = reclaim_bid_bond(extmrkt.tuple.0,bidder_badge);

                self.swap(extmrkt,vec_bckt,dex)
            }          
        }

        pub fn pay_winner_bid(&mut self, mut abc_bckt: Bucket, bidder_badge: Bucket) -> (Vec<Bucket>,Bucket) {
            let (bidder_badge,nmbr,mrkt_addr) = FooSquare::buy_bdg_data(bidder_badge);
            if self.nft_match(nmbr,true).n.0 { 
                let chk = self.check_status(nmbr,1);
                assert!(chk.t.0 == 4," NFT not in Auction payment mode ");

                //let rest = self.list.pay_win_bid(nmbr,self.abc.auction_dl);
                let rest = self.pay_win_bid_list(nmbr,self.abc.auction_dl);

                // Check if provided currency is requested one otherwise swap it
                if abc_bckt.resource_address() != self.currency {
                    abc_bckt = self.swap_fx(rest,self.currency,self.abc.dex,abc_bckt);
                }

                // Verify if provided badge is the winner one
                self.maps.pay_winner_bid(nmbr,bidder_badge.resource_address());

                let mut output_vec_bckt: Vec<Bucket> = Vec::new();
                let addr_key_map = self.check_meta_id(nmbr);
                for (nft_address,vec_key) in addr_key_map.into_iter() {
                    for key in vec_key {
                        picked(nft_address,key.clone());
                        output_vec_bckt.push(self.nft_take(nft_address,key,true)); 
                    }
                }             
                self.bckt_burn(bidder_badge);
                self.abc_put(nmbr, abc_bckt.take(rest));

                (output_vec_bckt,abc_bckt)
            } else {
                let (extmrkt,dex) = self.check_rec(mrkt_addr);
                let bckt = self.bckt_fx(dec!("0"),extmrkt.tuple.3,dex,abc_bckt);
                dex_output_amount(bckt.amount().clone());
                pay_winner_bid(extmrkt.tuple.0,bckt,bidder_badge)
            }          
        }

            // Retrieve nft provider position providing a relative userBadge as reference.
        pub fn ask_position(&mut self, badge: Proof) -> Vec<Tab> {
            let badge: ValidatedProof = badge.unsafe_skip_proof_validation();
            let output_vector = self.update_state(0, badge.resource_address());
            for tab in output_vector.clone() {
                position(tab.clone(), self.abc.auction_dl);
            }
 
            output_vector
        }

            // Retrieve nft selling status providing a relative instance number as reference.
        pub fn ask_instance(&mut self, sale_nr: u128) -> Vec<Tab> { 
            let mut output_vector = self.update_state(sale_nr, ResourceAddress::from(RADIX_TOKEN));  
            for mut tab in output_vector.clone() {
                    if tab.tuple.1.1.0 == 3 || tab.tuple.1.1.0 == 8 {
                        tab.tuple.1.2.0 = dec!(0);
                    } 
                    position(tab.clone(), self.abc.auction_dl);
                    output_vector.push(tab);
            }
 
            output_vector
        }

            // Add an external marketplace address & fee related to an NFT resource address 
            // to list on sell there too. Mint a Caller Badge to send to that marketplace and relate 
            // it to other data.         
        pub fn add_ext_mrkt(
            &mut self, 
            ext_square: ComponentAddress, 
            ext_fee: Decimal,
            ext_fx: ResourceAddress
        ) -> Bucket {
            let (mut founded,mut bdg_addr) = (false,ResourceAddress::from(RADIX_TOKEN));
            let mut badge = bdg_addr;
            for val in self.ext_mrkt_map.iter() {    
                if ext_square == val.0 {
                    (bdg_addr,badge,founded) = (val.1,val.4,true);
                    break;
                }    
            }
            if ext_square != self.abc.square {
                if founded && bdg_addr != ResourceAddress::from(RADIX_TOKEN) {
                    self.ext_mrkt_map.retain(|x| x.0 != ext_square);
                    self.ext_mrkt_map.push((ext_square,bdg_addr,ext_fee,ext_fx,badge));
                    Bucket::new(RADIX_TOKEN)
                } else {
                    let caller_bdg_res_def = ResourceBuilder::new_fungible()
                        .divisibility(DIVISIBILITY_NONE)
                        .metadata("name","CallerBadge")
                        .mintable(rule!(require(self.minter_badge.resource_address())), LOCKED)
                        .burnable(rule!(require(self.minter_badge.resource_address())), LOCKED)
                        .no_initial_supply(); 

                    if founded && bdg_addr == ResourceAddress::from(RADIX_TOKEN) {
                        self.ext_mrkt_map.retain(|x| x.0 != ext_square);  
                    }

                    self.ext_mrkt_map.push((ext_square,caller_bdg_res_def,ext_fee,ext_fx,badge));
                    self.minter_badge
                        .authorize(|| { borrow_resource_manager!(caller_bdg_res_def).mint(dec!("1")) })
                }
            } else {
                if founded {
                    self.ext_mrkt_map.retain(|x| x.0 != self.abc.square);
                }
            
                self.ext_mrkt_map.push((self.abc.square,self.foo_badge,self.abc.fee,self.abc.currency,badge));
                Bucket::new(RADIX_TOKEN)
            }
        }  

            // Remove external marketplace allowance related a specified NFT resource address. 
        pub fn remove_ext_mrkt(&mut self, ext_square: ComponentAddress) {
            let mut abc_badge = ResourceAddress::from(RADIX_TOKEN);
            let zero = abc_badge;
            let (mut i,mut founded) = (0,false);
            for val in self.ext_mrkt_map.iter_mut() {
                if val.0 == ext_square {
                    (abc_badge,founded) = (val.4,true);
                    break;
                }    
                i += 1;
            }
            if founded {
                self.ext_mrkt_map.remove(i);
                if self.foo_comp_addr == self.abc.square {
                    self.ext_mrkt_map.push((ext_square,zero,dec!("0"),zero,abc_badge));
                }
            }
        }

            // Stock External Marketplace badge in relative Vaults Hashmap: FooBadge for 
            // FooSquare or CallerBadge for others Markeplace Components.
        pub fn stock_badge(&mut self, ext_square: ComponentAddress, caller_badge: Bucket){
            let mut founded = false;
            for val in self.ext_mrkt_map.iter_mut() {
                if val.0 == ext_square { 
                    val.4 = caller_badge.resource_address();
                    founded = true;
                    break;
                }    
            }
            if !founded { 
                let zero = ResourceAddress::from(RADIX_TOKEN);
                self.ext_mrkt_map.push((ext_square,zero,dec!("0"),zero,caller_badge.resource_address()));
            }
            let vault = self.badge_vaults.entry(caller_badge.resource_address())
                .or_insert(Vault::new(caller_badge.resource_address()));
            vault.put(caller_badge);
        }

            // Retrieve external marketplace currency to perform reverse swap
        pub fn out_currency(&mut self, bdg_ref: Proof) -> ResourceAddress { 
            let bdg_ref: ValidatedProof = bdg_ref.unsafe_skip_proof_validation();               
            let extmrkt = self.ext_mrkt_data(self.abc.dex,bdg_ref.resource_address());
            assert!(extmrkt.tuple.4," NFT correspondence unfounded ");
            
            extmrkt.tuple.3
        } 

            // Set Foo settings in case of external Marketplace Component implementation & insert
            // FooSquare in related External Marketplace Map 
        pub fn set_foo_values(                                                                  
            &mut self, 
            foo_fee: Decimal, 
            foo_royalty: Decimal,
            abc_address: ResourceAddress,
            foo_square_address: ComponentAddress,
            abc_vault: ComponentAddress,
            foo_bdg: Proof
        ) -> bool {
            foo_bdg.validate_proof(ProofValidationMode::ValidateContainsAmount(self.foo_badge,dec!("1")))
                .expect(" [foo_square::set_foo_values]: Unauthorized ");
            assert!(foo_royalty >= dec!("0") && foo_royalty <= dec!("1")," delta fee 0:1 ");            
            
            self.abc.badge = self.foo_badge;
            self.abc.fee = foo_fee;
            self.abc.royalty = foo_royalty;
            self.abc.currency = abc_address;
            self.abc.square = foo_square_address;
            self.abc.vault = abc_vault;
            values(foo_fee,foo_royalty,abc_address,foo_square_address,abc_vault);
        
            true
        } 

        pub fn reset_foo_values(
            &mut self, 
            fee: Decimal, 
            royalty: Decimal,
            abc: ResourceAddress,
            foo_mrkt: ComponentAddress,
            abc_vault: ComponentAddress,
            foo_badge_addr: ResourceAddress,
            ext_mrkt: ComponentAddress
        ) {
            let badge = self.caller_bdg_take(foo_badge_addr);
            assert!(
                reset_foo_values(fee,royalty,abc,foo_mrkt,abc_vault,badge.create_proof(),ext_mrkt),
                " Unable to reset values "
            );
            self.caller_bdg_put(badge);
        }

            // Set Foo Dead Vault Component address & dead share.
        pub fn set_dead_values(&mut self, dead_vault: ComponentAddress, dead_share: Decimal) {      
            assert!(dead_share <= Decimal::from(100)," Max allowed value is 100 ");
            (self.abc.dead_vault,self.abc.dead_share) = (dead_vault,dead_share);
            dead_values(self.abc.dead_vault,self.abc.dead_share);
        }

            // Set protocol fee function whom only protocol owner can succesfully call.
        pub fn set_fee(&mut self, prtcl_fee: Decimal) {
            assert!(prtcl_fee >= dec!("0") && prtcl_fee <= dec!("10")," delta fee 0:10 ");
            self.fee = prtcl_fee;
            protocol_fee(self.fee);
        }

        pub fn set_deadlines(&mut self, auction: u64, last_bid_deadline: u64, buy_proposal: u64) {
            assert!(last_bid_deadline <= auction/10," Please lower last bid deadline ");
            self.abc.auction_dl = auction;    
            self.abc.last_bid_dl = last_bid_deadline;
            self.abc.buy_prop_dl = buy_proposal;
            deadlines(self.abc.auction_dl,self.abc.last_bid_dl,self.abc.buy_prop_dl);
        }

        pub fn set_comp_addr(&mut self){
            self.foo_comp_addr = Runtime::actor().as_component().0;
        } 

            // Retrieve external marketplace values 
            fn ext_mrkt_data(&mut self, cmp: ComponentAddress, bdg: ResourceAddress) -> ExtMrkt {   
                let mut extmrkt = ExtMrkt::new(self.abc.dex);
                for val in self.ext_mrkt_map.iter() {
                    if val.1 == bdg && cmp == self.abc.dex {
                        extmrkt.tuple.0 = val.0;
                        extmrkt.tuple.1 = val.1;
                        extmrkt.tuple.2 = val.2;
                        extmrkt.tuple.3 = val.3;
                        extmrkt.tuple.4 = true;
                        extmrkt.tuple.5 = val.4;
                        break;
                    } else if val.0 == cmp && bdg == ResourceAddress::from(RADIX_TOKEN) {
                        extmrkt.tuple.0 = val.0;    
                        extmrkt.tuple.3 = val.3;    
                        extmrkt.tuple.5 = val.4;
                        break;
                    }    
                }

                extmrkt
            }

            // Take Caller Badge from Vault relate to an external Marketplace resource address.
            fn caller_bdg_take(&mut self, ext_badge: ResourceAddress) -> Bucket {
                match self.badge_vaults.get_mut(&ext_badge) {
                    Some(vault) => vault.take(Decimal::one()),
                    None => {
                        info!(" Caller Badge not in stock! ");
                        std::process::abort()
                    }
                }
            } 

            // Test external marketplace buy allowance for royalties computation.
            fn out_fx(&self, ext_mrkt: ComponentAddress, bdg_bckt_ref: Proof) -> ResourceAddress {
                out_currency(ext_mrkt,bdg_bckt_ref)
            } 

            fn bckt_fx(&mut self, min: Decimal, ad: ResourceAddress, d: ComponentAddress, b: Bucket) -> Bucket {
                if ad != b.resource_address().clone() {
                    self.swap_fx(min, ad, d, b)
                } else {
                    b
                }
            }

            fn swap(&mut self, em: ExtMrkt, mut vb: Vec<Bucket>, dex: ComponentAddress) -> Vec<Bucket> {
                if vb.get(0).unwrap().resource_address() == em.tuple.3 {    
                    let bckt = self.swap_fx(dec!("0"), self.currency, dex, vb.pop().unwrap());            
                    vb.push(bckt);
                }

                vb
            }

            // Swap tokens on an external DEX
            fn swap_fx(&self, sum: Decimal, fx: ResourceAddress, dex: ComponentAddress, abc: Bucket) -> Bucket { 
                swap_fx(sum,fx,dex,abc)
            }

            // Put Caller Badge in Vault relate to an external Marketplace resource address.
            fn caller_bdg_put(&mut self, caller_badge: Bucket){
                let v = self.badge_vaults.get_mut(&caller_badge.resource_address()).unwrap();
                v.put(caller_badge);
            }

            fn check_buy(&mut self, mrkt_addr: ComponentAddress) -> (ExtMrkt,Bucket,ComponentAddress) {
                let extmrkt = self.ext_mrkt_data(mrkt_addr,ResourceAddress::from(RADIX_TOKEN));
                let caller_badge = self.caller_bdg_take(extmrkt.tuple.5);
                let bdg_bckt_ref = caller_badge.create_proof();
                let output_currency = self.out_fx(extmrkt.tuple.0,bdg_bckt_ref);
                assert!(self.currency == output_currency," External marketplace unauthorized ");

                (extmrkt,caller_badge,self.abc.dex.clone())
            }

            fn check_rec(&mut self, mrkt_addr: ComponentAddress) -> (ExtMrkt,ComponentAddress) {          
                let extmrkt = self.ext_mrkt_data(mrkt_addr,ResourceAddress::from(RADIX_TOKEN));
                let caller_badge = self.caller_bdg_take(extmrkt.tuple.5);
                self.caller_bdg_put(caller_badge);

                (extmrkt,self.abc.dex.clone())
            }

            fn check_meta( 
                &mut self,
                meta_nft_bckt_sum: Vec<Bucket>,
                flag: bool,
                new_nmbr: u128,
                price_flag: u8,
                new_price: Decimal  
            ) -> CheckMeta {
                let mut vmk: Vec<NonFungibleId> = Vec::new();
                let mut addr_metakey_map: HashMap<ResourceAddress,Vec<NonFungibleId>> = HashMap::new();
                let (mut esc,mut key_tuple_bool,mut ix,mut jx) = (false,false,0,0);
                for (addr,meta_addr) in self.meta.iter() {
                    for _bckt in meta_nft_bckt_sum.iter() {
                        if meta_addr == &meta_nft_bckt_sum.get(jx).unwrap().resource_address() {
                            let meta_key = meta_nft_bckt_sum.get(jx).unwrap().non_fungible::<FooNFT>().id();
                            match addr_metakey_map.get_mut(&addr) {    
                                Some(v) => { 
                                    v.push(meta_key);
                                }
                                _ => { 
                                    vmk.clear();
                                    vmk.push(meta_key);
                                    addr_metakey_map.insert(*addr,vmk.clone());
                                }
                            }                    
                            ix += 1;
                        }
                        jx += 1;
                        if ix == meta_nft_bckt_sum.len() {
                            esc = true;
                            break;
                        }
                        if jx == meta_nft_bckt_sum.len() {
                           jx = 0; 
                        }

                    }
                    if esc {
                        key_tuple_bool = true;
                        break;
                    }
                }  
                assert!(key_tuple_bool," Key correspondence unfounded ");

                for meta_nft_burn in meta_nft_bckt_sum {
                    self.bckt_burn(meta_nft_burn);
                }  

                let (mut amount,mut number,mut old_nmbr,mut switch) = (dec!(0),0,0,false);
                let mut vec_new: Vec<(NonFungibleId,NonFungibleId,Decimal,u128)> = Vec::new();
                let mut chk = CheckTuple::new();
                let (mut esc,mut key_tuple_bool,mut i,mut iy) = (false,false,0,0);
                let mut vec_nmbr: Vec<u128> = Vec::new();
                for (nft_addr,meta_nft_key_vec) in addr_metakey_map.iter_mut() {
                    let (_sum,v_key) = self.meta_map.get_mut(&nft_addr).unwrap();
                   
                    for _meta_nft_key in meta_nft_key_vec.clone() {  
                        let mut j = 0;
                        for mut tuple in v_key.clone() {
                            if tuple.1 == meta_nft_key_vec[i] { 
                                if number != 0 && new_nmbr == 0 {
                                    assert!(tuple.3 == number," Mixed instances detected ");
                                } 
                                vec_nmbr.push(tuple.3);
                                if new_nmbr != 0 {
                                    old_nmbr = tuple.3;
                                    tuple.3 = new_nmbr;
                                }
                                number = tuple.3;
                                if !flag && tuple.2 == Decimal::zero() { 
                                    tuple.2 = Decimal::one();  
                                    let tup = tuple.clone();
                                    vec_new.push(tup);          
                                    switch = true;         
                                }
                                v_key.remove(j);
                                meta_nft_key_vec[i] = tuple.0.clone();  
                                i += 1;
                                iy += 1;                                                 
                            } else {
                                j += 1;
                            }
                            if i == meta_nft_key_vec.len() {
                                if iy == ix {
                                    (amount,key_tuple_bool,esc) = (tuple.2,true,true);
                                    break;
                                } 
                                i = 0;
                            }
                        }
                        if switch {
                            for tuple in &vec_new {
                                v_key.push(tuple.clone());
                            }
                            vec_new.clear();
                        }
                        if esc {    
                            break;
                        }
                    }           

                    if esc {
                        if new_nmbr != 0 {
                            assert!(vec_nmbr.iter().all(|x| *x == old_nmbr)," Mixed instances detected ");
                            chk = self.status_check(old_nmbr);
                        }
                        break;
                    }
                } 
                assert!(key_tuple_bool," Key correspondence unfounded ");
                
                let mut nft_vec: Vec<(ResourceAddress,NonFungibleId)> = Vec::new();
                for (nft_addr,meta_nft_key_vec) in addr_metakey_map.clone().into_iter() { 
                    for keys in meta_nft_key_vec {
                        nft_vec.push((nft_addr,keys.clone()));
                    }
                }

                if new_nmbr != 0 || new_price != Decimal::zero() {
                    for (nft_addr,key) in nft_vec.clone() {
                        match self.nft_map.get_mut(&nft_addr) {
                            Some(v) => {
                               for mut data in v { 
                                    if data.1 == key.clone() {
                                        if price_flag != 0 {
                                            data.2 = new_price;                 // modify_nft_price method
                                        } else {
                                            (data.0,data.3) = (number,true);    // restart_auction restart_raffle methods
                                        }
                                    }                               
                                }                        
                            }  
                            None => { 
                                info!(" Unfounded correspondence ");
                                std::process::abort()
                            }
                        }
                    }
                }
                
                let output_tuple = (addr_metakey_map,nft_vec,amount,number,old_nmbr,chk.t.0);
                CheckMeta { m: output_tuple }
            }

            fn check_meta_id(
                &mut self,
                number: u128,
            ) -> HashMap<ResourceAddress,Vec<NonFungibleId>> {
                let (mut key_tuple_bool,mut switch) = (false,false);
                let mut vk: Vec<NonFungibleId> = Vec::new();
                let mut addr_key_map: HashMap<ResourceAddress,Vec<NonFungibleId>> = HashMap::new();
                let mut vec_new: Vec<(NonFungibleId,NonFungibleId,Decimal,u128)> = Vec::new();
                
                for (addr,(_a,v_key)) in self.meta_map.iter_mut() { 
                    let mut i = 0;                    
                    for mut tuple in v_key.clone() {
                        if number == tuple.3 {
                            match addr_key_map.get_mut(&addr) {
                                Some(v) => v.push(tuple.0.clone()),
                                _ => { 
                                    vk.clear();
                                    vk.push(tuple.0.clone());
                                    addr_key_map.insert(*addr,vk.clone());
                                }
                            } 
                            if tuple.2 != dec!("1") {
                               (tuple.2,switch) = (dec!("1"),true);
                               vec_new.push(tuple);       
                            }
                            v_key.remove(i);
                            key_tuple_bool = true;
                        } else {
                            vec_new.push(tuple);
                            i += 1;
                        }
                    }
                    if switch {
                        v_key.clear();
                        v_key.append(&mut vec_new);
                    }
                    if !key_tuple_bool {
                        vec_new.clear();
                    }
                }
                assert!(key_tuple_bool," Key correspondence unfounded ");
                    
                addr_key_map
            } 

            // Retrieve instance number from related map invoked by sellers methods. 
            fn check_status(&mut self, nmbr: u128, flag: u8) -> CheckTuple { 
                //self.list.check_status(nmbr,flag); 
                self.check_status_list(nmbr,flag);                         
                self.status_check(nmbr)                                     
            }  

            // Check Auction or Raffle mode by related NFT and export related data. 
            fn status_check(&mut self, nmbr: u128) -> CheckTuple { 
                let mut tup = CheckTuple::new();   
                let output_vector = self.update_state(nmbr, ResourceAddress::from(RADIX_TOKEN));           
                for tab in output_vector.clone() { 
                    let (_key,val) = tab.tuple;
                    let amnt = val.2.1*val.2.5;
                    match val.1.0 {
                        3 ... 7 => tup.t = (val.1.0,val.2.0,val.2.1,val.2.2,val.2.3,0,0,amnt,nmbr,val.1.1),
                        8 ... 12 => tup.t = (val.1.0,val.2.0,val.2.1,val.2.2,dec!("0"),val.2.4,val.2.5,amnt,nmbr,val.1.1),
                        _ => tup.t = (val.1.0,dec!("0"),dec!("0"),0,dec!("0"),0,0,amnt,nmbr,val.1.1)
                    }
                }

                CheckTuple { t: tup.t }
            }

            fn update_state(&mut self, nr: u128, bdg: ResourceAddress) -> Vec<Tab> { 
                let mut output_vec: Vec<Tab> = Vec::new();
                //let (v,nmbr_vec,switch_vec,tckt) = self.list.update(nr,bdg,self.abc.auction_dl);
                let (v,nmbr_vec,switch_vec,tckt) = self.update_list(nr,bdg,self.abc.auction_dl);

                if switch_vec.len() > 0 { 
                    for sale_number in switch_vec { 
                        self.switch_status(sale_number,0); 
                    }
                }

                if nmbr_vec.len() > 0 { 
                    for mut tab in v {
                        for sale_number in nmbr_vec.clone() {
                            if tab.tuple.0.1 == sale_number {
                                tab.tuple.1.2.6 = self.raffle_winner(sale_number,tckt);
                                self.switch_status(sale_number,0); 
                            }
                        }
                        output_vec.push(tab);
                    }

                    output_vec
                } else { 
                    
                    v
                }
            }

            fn buy_bdg_data(nft: Bucket) -> (Bucket,u128,ComponentAddress) {
                let nft_data: Mode = nft.non_fungible().data();
                (nft,nft_data.instance_nmbr,nft_data.mrkt_addr)
            }
            
            fn raffle_bdg_data(nft: Bucket) -> (Bucket,u128,ComponentAddress) {
                let nft_data: TicketID = nft.non_fungible().data();
                (nft,nft_data.instance_nmbr,nft_data.mrkt_addr)
            }

            // Switch Badge addresses data between buyer/seller whenever a buy proposal has been accepted
            // or a raffle jackpot has been collected 
            fn switch_badge(&mut self, bdg: ResourceAddress, flag: u8, nmbr: u128, profit: Decimal){
                //self.list.switch_badge(bdg,flag,nmbr,profit);
                self.switch_badge_list(bdg,flag,nmbr,profit);
            }

            fn take_fee(
                &mut self,
                sale_nr: u128,
                abc_amnt: Decimal, 
                mut abc_bckt: Bucket,
                bdg_addr: ResourceAddress,
                flag: u8
            ) -> (Decimal,Bucket) {
                let h = dec!("100");
                let rest: Decimal;
                if self.currency != self.abc.currency {
                    let foo_bckt: Bucket;
                    rest = abc_amnt-abc_amnt*self.fee/h;
                    net_gain(rest);

                    // NFT sold on External Marketplace by another External Marketplace
                    if bdg_addr != self.foo_badge && bdg_addr != ResourceAddress::from(RADIX_TOKEN) {
                        let extmrkt = self.ext_mrkt_data(self.abc.dex,bdg_addr);
                        assert!(extmrkt.tuple.4," NFT correspondence unfounded 2 ");
                        
                        self.collected_abc.put(abc_bckt.take(abc_amnt*(self.fee-extmrkt.tuple.2-self.abc.royalty)/h));
                        let ext_fee_bckt = abc_bckt.take(abc_amnt*extmrkt.tuple.2/h);
                        foo_bckt = abc_bckt.take(abc_amnt*self.abc.royalty/h);                        
                        let royalty = self.swap_fx(dec!("0"),extmrkt.tuple.3,self.abc.dex,ext_fee_bckt);
                        let amnt = abc_stock(royalty,extmrkt.tuple.0);
                        info!(" Fee placed in external Marketplace Vault {} ",amnt);

                    // NFT sold by External Marketplace
                    } else if bdg_addr == ResourceAddress::from(RADIX_TOKEN) {                       
                        self.collected_abc.put(abc_bckt.take(abc_amnt*(self.fee-self.abc.royalty)/h));
                        foo_bckt = abc_bckt.take(abc_amnt*self.abc.royalty/h);

                        let sum_one = abc_amnt*(self.fee-self.abc.royalty)/h;
                        let foo_royalty = abc_amnt*self.abc.royalty/h;
                        net_fee(sum_one,foo_royalty);
                        
                    // NFT sold on External Marketplace by FooSquare
                    } else {                                                                
                        assert!(bdg_addr == self.foo_badge," FooSquare Selling Tx ");                   
                        self.collected_abc.put(abc_bckt.take(abc_amnt*(self.fee-self.abc.fee)/h));
                        foo_bckt = abc_bckt.take(abc_amnt*self.abc.fee/h);  

                        let foo_fee = abc_amnt*self.abc.fee/h;
                        royalty(foo_fee);
                    }
                    let royalty = self.swap_fx(dec!("0"),self.abc.currency,self.abc.dex,foo_bckt);
                    let amount = abc_stock(royalty,self.abc.vault);
                    royalty_placed(amount);

                // NFT sold on FooSquare
                } else {
                    let mut foo_fee = abc_amnt*self.abc.fee/h;
                    if self.abc.dead_share > dec!("0") {    
                        let dead_fee = (abc_amnt*self.abc.fee/h)*self.abc.dead_share/h;
                        foo_fee -= dead_fee;
                        self.abc_everlock(abc_bckt.take(dead_fee));
                    }

                    self.collected_abc.put(abc_bckt.take(foo_fee));
                    rest = abc_amnt-abc_amnt*self.abc.fee/h;
                    net_gain(rest);
                }

                // Put proceeds from NFT sale into seller related vault.
                match flag {
                    1 => self.abc_put(sale_nr, abc_bckt.take(rest)),
                    _ => ()
                }

                (rest,abc_bckt)
            } 

            // Everlock $ABC token dead share in Dead Vault Component 
            fn abc_everlock(&mut self, dead_bckt: Bucket){
                abc_everlock(dead_bckt,self.abc.dead_vault);
            } 

            // Collect bid bond in case of auction payment deadline ovetaken 
            fn bid_bond(&mut self, s: u8, bb: Decimal, rp: Decimal, m: Decimal, n: u128) -> Bucket {
                let mut collected_bid_bond: bool = false;
                if s == 5 && bb > dec!("0") {
                    bid_bond(bb);
                    collected_bid_bond = true;
                }   
                if collected_bid_bond && m >= rp {   
                    self.abc_collect(n, bb)
                } else {
                    Bucket::new(RADIX_TOKEN)
                }
            } 

            fn abc_put(&mut self, sale_nr: u128, bckt: Bucket){
                match self.abc_vaults.get_mut(&sale_nr) {
                    Some(vault) => vault.put(bckt),
                    None => std::process::abort()
                }
            } 

            fn abc_collect(&mut self, sale_nr: u128, amnt: Decimal) -> Bucket {
                let output_abc: Bucket;
                match self.abc_vaults.get_mut(&sale_nr) {
                    Some(vault) => output_abc = vault.take(amnt),
                    None => output_abc = unfound_bckt(6)
                }
            
                output_abc
            }

            // Update past auction bids in related map
            fn update_auction_map(&mut self, nmbr: u128){
                self.maps.update_auction_map(nmbr);
            } 

            // Update past raffle bids in related map
            fn update_raffle_map(&mut self, nmbr: u128){
                self.maps.update_raffle_map(nmbr); 
            } 

            // Change NFT sale status switching related flag. 
            fn switch_status(&mut self, nmbr: u128, flag: usize){
                let mut answer = false;
                for (_key,v) in self.nft_map.iter_mut() { 
                    for val in v { 
                        if val.0 == nmbr { 
                            match flag {
                                1 => val.3 = true, 
                                _ => val.3 = false
                            }
                            answer = true;
                        }
                    }
                }
                assert!(answer," Sale instance unfounded ! ");
            } 

            fn raffle_winner(&mut self, nmbr: u128, tickets: u8) -> u128 { 
                let tckts = usize::from(tickets); 
                let hash = Runtime::transaction_hash().to_string();
                let seed = &hash[0..5];
                let result = usize::from_str_radix(&seed, 16).unwrap();
                let index_value = result % tckts;
                index(index_value,nmbr);

                let w = self.maps.raffle_winner(nmbr);
                let val = w.get(index_value).unwrap(); 
                winner(val.0,val.1);
                self.maps.update_raffle_winner(val.0,val.1);   
                //self.list.raffle_winner(nmbr,val.0);
                self.raffle_winner_list(nmbr,val.0);

                val.0
            } 

            // Verify requested NFT is present within FooSquare listing or not. 
            fn nft_match(&mut self, nmbr: u128, flag: bool) -> NftMatch {         
                let (mut answer,mut price) = (false,dec!("0"));
                let mut nft_vec: Vec<(ResourceAddress,NonFungibleId)> = Vec::new();
                for (key,v) in self.nft_map.iter_mut() { 
                    for val in v.clone() { 
                        if val.0 == nmbr && val.3 || val.0 == nmbr && flag { 
                            nft_vec.push((*key,val.1));
                            (answer,price) = (true,val.2);
                        }
                    }
                }
                if !answer { 
                    nft_vec.push((ResourceAddress::from(RADIX_TOKEN),NonFungibleId::from_u64(0u64)));
                }
                let output_tuple = (answer,nft_vec,price);
                
                NftMatch { n: output_tuple}
            } 

            fn bckt_burn(&mut self, bckt: Bucket) {
                self.minter_badge.authorize(|| {bckt.burn()});
            }

            // Take buyed NFT from nft vault and erase data in relative hashmap.
            fn nft_take(&mut self, addr: ResourceAddress, key: NonFungibleId, flag: bool) -> Bucket {
                self.erase_from_map(addr, key.clone());

                // Update NFT sell profit amount
                if flag {
                    let (a,v) = self.meta_map.get_mut(&addr).unwrap();
                    for value in v {
                        if value.0 == key.clone() {
                            *a += value.2;
                        }
                    }
                }

                match self.nft_vaults.get_mut(&addr) {
                    Some(vault) => vault.take_non_fungible(&key),
                    None => {
                        info!(" NFT not in stock! ");
                        std::process::abort()
                    }
                }
            }

            // erase NFT data from map
            fn erase_from_map(&mut self, nft_addr: ResourceAddress, nft_key: NonFungibleId){
                let v = self.nft_map.get_mut(&nft_addr).unwrap();
                let mut i = 0;
                for data in v.clone() { 
                    if i < v.clone().len() && data.1 == nft_key {
                        v.remove(i);
                    }    
                    i += 1;                     
                }  
            }  

            // Ckeck buyer badge correspondence within related maps.
            fn badge_in(&mut self, nmbr: u128, bdg: ResourceAddress, j: u8) -> bool {  
                
                self.maps.badge_in(nmbr,bdg,j)
            }

            fn erase_map_entry(&mut self, nmbr: u128, a: u64, b: u8) {

                self.maps.erase_map_entry(nmbr,a,b)
            } 

            // Mint a Badge following a NFT stock event.
            fn add_badge(
                &mut self,  
                vec_data: Vec<(ResourceAddress,NonFungibleId,FooNFT)>,
                price: Decimal, 
                auction: bool,
                raffle: bool,
                reserve_prc: Decimal,
                start_prc: Decimal,
                end: u64,
                bid_bond: Decimal,
                dl: u64,             
                nmbr: u128
            ) -> Bucket {
                let user_badge: ResourceAddress = ResourceBuilder::new_fungible()
                    .divisibility(DIVISIBILITY_NONE)
                    .metadata("name", " FooSquare User Badge ")
                    .mintable(rule!(require(self.minter_badge.resource_address())), LOCKED)
                    .burnable(rule!(require(self.minter_badge.resource_address())), LOCKED)
                    .no_initial_supply();

                let mut tab = Tab::new();
                let mut tup_two = tab.tuple.1;
                tup_two.0 = vec_data.clone();
                let tup_one = (user_badge, nmbr);

                if !auction && !raffle {  
                    tup_two.2.0 = price;
                } else if !raffle {
                    tup_two.1.0 = 3;
                    tup_two.2 = (reserve_prc,start_prc,end,bid_bond,dl,0,0);
                } else {
                    tup_two.1.0 = 8;   
                    tup_two.2 = (reserve_prc,start_prc,end,dec!("0"),dl,0,0);
                } 

                tab = Tab { tuple:(tup_one,tup_two)};
                //self.list.map_insert(tab);
                self.map_insert_list(tab);
                self.minter_badge
                    .authorize(|| { borrow_resource_manager!(user_badge).mint(Decimal::one()) })
            }

            // Mint a Badge and populate a hashmap following a Buy Proposal or an Auction Bid event.
            fn add_buyer_badge(
                &mut self, 
                amnt: Decimal,
                sum: Decimal, 
                end: u64,
                mode: u8,       
                bid_bond: Decimal,
                badge_addr: ResourceAddress,    
                status: u8,
                sale_nr: u128
            ) -> Bucket {   
                let buy_bdg: ResourceAddress = ResourceBuilder::new_non_fungible()
                    .metadata("name","BuyerBadge")
                    .metadata("instance", format!("{}", sale_nr))
                    .metadata("marketplace", format!("{}", self.foo_comp_addr))
                    .mintable(rule!(require(self.minter_badge.resource_address())), LOCKED)
                    .burnable(rule!(require(self.minter_badge.resource_address())), LOCKED)
                    .no_initial_supply(); 

                let key = NonFungibleId::from_bytes(sale_nr.to_be_bytes().to_vec());
                let data = Mode { 
                    instance_nmbr: sale_nr,
                    mrkt_addr: self.foo_comp_addr, 
                    listing_mode: mode
                };
                match mode {
                    0 => {
                        let mut v: Vec<(ResourceAddress,Decimal,u64,u8,ResourceAddress)> = Vec::new();
                        v.push((buy_bdg, amnt, end, 1, badge_addr));
                        self.maps.insert_buy_prop_map(sale_nr,v);
        
                        self.minter_badge.authorize(|| { 
                            borrow_resource_manager!(buy_bdg).mint_non_fungible(&key, data)
                        })     
                    }
                    1 => {
                        let mut v: Vec<(ResourceAddress,Decimal,u64,u8,Decimal,ResourceAddress)> = Vec::new();  
                        v.push((buy_bdg, amnt, end, status, bid_bond, badge_addr));
                        self.maps.insert_auction_map(sale_nr,v);          

                        self.minter_badge.authorize(|| { 
                            borrow_resource_manager!(buy_bdg).mint_non_fungible(&key, data)
                        })
                    }
                    _ => {
                        let mut vec_id = self.gen_ticket_id(amnt);
                        let id = TicketID {
                            instance_nmbr: sale_nr,
                            mrkt_addr: self.foo_comp_addr, 
                            key: key.clone(),
                            v: vec_id.clone()
                        };
                        let mut v: Vec<(u128,ResourceAddress,Decimal,u64,u8,ResourceAddress)> = Vec::new();
                        for tckt_id in vec_id.iter_mut() {
                            v.push((*tckt_id,buy_bdg, sum, end, 0, badge_addr));
                        }
                        self.maps.insert_raffle_map(sale_nr,v);
                        
                        self.minter_badge.authorize(|| { 
                            borrow_resource_manager!(buy_bdg).mint_non_fungible(&key, id)
                        })   
                    }
                }
            }

            fn gen_ticket_id(&self, amnt: Decimal) -> Vec<u128> {
                let mut i = dec!("0");
                let mut vec_id = Vec::new();
                loop {
                    let tckt_id = u128::from(Runtime::generate_uuid());
                    vec_id.push(tckt_id);
                    i += 1;
                    if i == amnt {
                        break;
                    }
                }

                vec_id
            }   

            fn add_meta_nft(
                &mut self, 
                eco: String,
                series: String, 
                number: String, 
                address: ResourceAddress
            ){
                assert!(!self.meta.contains_key(&address)," meta nft already exist ");

                let meta_res_def: ResourceAddress = ResourceBuilder::new_non_fungible()
                    .metadata("Ecosystem", format!(" m-{}",eco))
                    .metadata("Series", format!(" m-{}",series))
                    .metadata("Number", format!(" m-{}",number))
                    .mintable(rule!(require(self.minter_badge.resource_address())), LOCKED)
                    .burnable(rule!(require(self.minter_badge.resource_address())), LOCKED)
                    .no_initial_supply(); 

                self.meta.insert(address.clone(),meta_res_def);
            }

            // Mint a metanft amount relative to an amount of nft provided to protocol 
            fn meta_mint(
                &mut self,  
                nft_address: ResourceAddress, 
                new_nft: FooNFT,
                nft_key: NonFungibleId,
                nmbr: u128
            ) -> Bucket {             
                let meta_res_def = self.meta.get(&nft_address).unwrap().clone();               
                let meta_nft_key = NonFungibleId::random();
                meta_mint(meta_nft_key.clone(),meta_res_def);
                match self.meta_map.get_mut(&nft_address) { 
                    Some((_a,v_key)) => {
                        let mut i: usize = 0;
                        for keys in v_key.clone() { 
                            if nft_key == keys.0 && nft_key == keys.1 && nmbr == keys.3 { 
                                v_key.remove(i);
                                break;
                            }     
                            i += 1;
                        } 
                        v_key.push((nft_key.clone(),meta_nft_key.clone(),Decimal::zero(),nmbr));  
                    }    
                    None => std::process::abort()                  
                };

                self.meta.insert(nft_address.clone(),meta_res_def.clone());
        
                self.minter_badge.authorize(|| { 
                    borrow_resource_manager!(meta_res_def).mint_non_fungible(&meta_nft_key,new_nft)
                }) 
            }

        // List ( imported methods ) 

        fn map_insert_list(&mut self, tab: Tab) {
            let tup_one = tab.tuple.0;
            let tup_two = tab.tuple.1;
            self.list_map.insert(tup_one,tup_two);
        }

        fn check_status_list(&mut self, nmbr: u128, flag: u8) {
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

        fn switch_badge_list(&mut self, bdg: ResourceAddress, flag: u8, nmbr: u128, profit: Decimal){
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

        fn check_badge_list(&mut self, bdg_addr: ResourceAddress) -> bool {                   
            for (key,_value) in self.list_map.iter() {
                if key.0 == bdg_addr {
                    return true;
                }    
            }     
            false
        } 

        fn raffle_winner_list(&mut self, nmbr: u128, val: u128) {
            for (key,value) in self.list_map.iter_mut() { 
                if key.1 == nmbr {
                    value.2.6 = val;
                }
            }
        }

        fn update_list(&mut self, nr: u128, bdg: ResourceAddress, dl: u64) -> (Vec<Tab>,Vec<u128>,Vec<u128>,u8) { 
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
        fn buy_nft_list(&mut self, sale_nr: u128, rest: Decimal) {
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
        fn buy_ticket_list(&mut self, nmbr: u128, sum: u8, new_end: bool) -> u8 {
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

        fn buy_prop_list(&mut self, nmbr: u128, prop: Decimal, endtime: u64) -> (u64,u8) {
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

        fn place_bid_list(&mut self, nmbr: u128, bid: Decimal, new_end: bool) -> bool {
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

        fn unstock_list(&mut self, nr: u128) {
            for (key,value) in self.list_map.iter_mut() {
                if key.1 == nr {
                    value.1.0 = 12;
                    break; 
                }      
            }
        }

        fn pay_win_bid_list(&mut self, nmbr: u128, end: u64) -> Decimal {
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

