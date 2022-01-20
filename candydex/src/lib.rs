use sbor::*;
use scrypto::prelude::*;

blueprint! {
    struct CandyStore {
        collected_xrd: Vault,       
        candy_vaults: HashMap<Address, Vault>,
        candy_ratio: HashMap<Address, Decimal>,
        prices: HashMap<Address, Decimal>,
        name: HashMap<Address, String>,
        symbol: HashMap<Address,String>,
        meta: HashMap<Address, MetaToken>,
        minter_badge: Vault,
        owner_badge: ResourceDef,
        xrd_fee: Decimal,
        xrd_claimed: Decimal,
        fee: Decimal
    }

    impl CandyStore {
        pub fn new(fee: Decimal) -> (Component,Bucket) {
            let minter_badge : Bucket = ResourceBuilder::new_fungible(DIVISIBILITY_NONE) 
                .metadata("name", " MinterBadge ")
                .initial_supply_fungible(1);
            let badge_bucket = ResourceBuilder::new_fungible(DIVISIBILITY_NONE)
                .metadata("name", " OwnerBadge ")
                .initial_supply_fungible(1);

            let component = Self { 
                collected_xrd: Vault::new(RADIX_TOKEN),
                candy_vaults: HashMap::new(),
                candy_ratio: HashMap::new(),
                prices: HashMap::new(),
                name: HashMap::new(),
                symbol: HashMap::new(),
                meta: HashMap::new(),
                minter_badge: Vault::with_bucket(minter_badge),
                owner_badge: badge_bucket.resource_def(),
                xrd_fee: 0.into(),
                xrd_claimed: 0.into(),
                fee
            }
            .instantiate();
            (component,badge_bucket)
        }   

            fn add_meta_candy(&mut self, candy_name: String, candy_symbol: String, candy_address: Address) {
                assert!(!self.meta.contains_key(&candy_address)," Candy already exist ");

                let meta_res_def = ResourceBuilder::new_fungible(DIVISIBILITY_MAXIMUM)
                    .metadata("name", format!(" m{}", candy_name.clone()))
                    .metadata("symbol", format!(" m{}", candy_symbol.clone()))
                    .flags(MINTABLE | BURNABLE)
                    .badge(self.minter_badge.resource_def(), MAY_MINT | MAY_BURN)
                    .no_initial_supply();
                self.meta.insert(candy_address.clone(),MetaToken::new(candy_name, candy_symbol, candy_address, meta_res_def));
            }

            fn meta_mint(&mut self, candy_amnt: Decimal, address: Address, ratio: Decimal) -> Bucket {
                let m_candy = self.meta.get(&address).unwrap();
                let meta_candy = self.minter_badge.authorize(|auth| m_candy.meta_res_def.mint(candy_amnt/ratio, auth));
                meta_candy
            }

            fn candyprice(&mut self, candy_out_addr: Address) -> Decimal {
                let price: Decimal;  
                let _price = match self.prices.get(&candy_out_addr) {
                                Some(_price) => price = *_price,
                                None         => { info!("Candy not in stock !");
                                                  std::process::abort()
                                                }
                };
                price
            }     

            fn candyput_pri(&mut self, prc_in: Decimal, prc_out: Decimal, addr_in: Address, candy: Bucket) -> (Decimal,Decimal) {
                let mut candy_amnt = candy.amount();
                let nmbr = candy.amount()*prc_in/prc_out;
                if prc_in == 0i32.into() { candy_amnt = CandyStore::adjust_fee(self, candy_amnt); }   
                let amount = CandyStore::candy_sum(self, candy_amnt, candy.resource_address(), addr_in, 1i32.into());
                let v = self.candy_vaults.entry(candy.resource_address()).or_insert(Vault::new(candy.resource_address()));
                v.put(candy);
                (nmbr,amount)
            } 

            fn candyput_sec(&mut self, amnt: Decimal, addr: Address, prc_in: Decimal, prc_out: Decimal, candy: Bucket) -> Bucket {
                let amount = candy.amount();
                let amnt_: Decimal;
                if prc_in == 0i32.into() {amnt_ = CandyStore::candy_sum(self, amnt, addr, candy.resource_address(), 1i32.into());}                     
                else {amnt_ = amnt*prc_in/prc_out;}
                assert!( amnt_ <= amount, "Not enough input amount");
                let v = self.candy_vaults.entry(candy.resource_address()).or_insert(Vault::new(candy.resource_address()));
                v.put(candy);
                let candy_output: Bucket = v.take(amount-amnt_);
                candy_output
            } 

            fn candytake(&mut self, candy_out_nbr: Decimal, candy_out_addr: Address) -> Bucket {
                let one: Decimal = 1i32.into();
                let candy_bucket: Bucket = match self.candy_vaults.get(&candy_out_addr) {
                    Some(vault) => vault.take(candy_out_nbr-(candy_out_nbr*self.fee/100)),
                    None => { info!("Candy not in stock !");
                              std::process::abort()
                            }
                };
                let total_candy = self.candy_vaults.get(&candy_out_addr).unwrap().amount();
                let new_ratio = one+(candy_out_nbr/total_candy)*(self.fee/100);
                self.candy_ratio.insert(candy_out_addr,new_ratio);      
                candy_bucket
            }

            fn price_mod(&mut self, amount: Decimal, address: Address, price: Decimal, flag: Decimal) -> Decimal {
                let total_candy = self.candy_vaults.get(&address).unwrap().amount();
                let candy_amnt = amount;
                let new_price: Decimal;
                if flag == 1i32.into() { new_price = total_candy*price/(total_candy-candy_amnt); 
                }else{ new_price = total_candy*price/(total_candy+candy_amnt);                   
                }    
                new_price
            }  

            fn candy_sum(&mut self, amnt_pri: Decimal, addr_pri: Address, addr_sec:Address, flag: Decimal)-> Decimal {     
                let mut amount: Decimal = amnt_pri;   
                let price_in: Decimal = CandyStore::candyprice(self, addr_pri);   
                let price_out : Decimal = CandyStore::candyprice(self, addr_sec); 
                let tot_amnt_out = self.candy_vaults.get(&addr_sec).unwrap().amount();
                let candy_out_amnt = amnt_pri*price_in/price_out;    
                if flag == 2i32.into() {amount = amnt_pri/price_in;}                    
                if flag == 1i32.into() {         
                  let price_new = price_out*tot_amnt_out/(tot_amnt_out+candy_out_amnt);
                  amount = price_new*candy_out_amnt/price_out;
                } else { 
                  let price_new: Decimal = CandyStore::price_mod(self, amount, addr_pri, price_in, 1i32.into());
                  self.prices.insert(addr_pri, price_new);
                  amount = amnt_pri/price_new;
                }  
                amount
            } 
                                                                                                                        
            fn adjust_fee(&mut self, amount_in: Decimal ) -> Decimal { 
                let hundred: Decimal = 100i32.into();
                let amount_out = amount_in*hundred/(hundred-self.fee);
                amount_out
            }
            // get_candy_sell_amount => buy_exact_xrd_sell_candy (bexsc)
        pub fn get_candy_sell_amount_bexsc(&mut self, candy_addr: Address, xrd_amnt: Decimal) -> Decimal {   //buy_xrd  
            let xrd_amount = CandyStore::adjust_fee(self, xrd_amnt);
            let price = CandyStore::candyprice(self, candy_addr);
            let new_price: Decimal = CandyStore::price_mod(self, xrd_amount/price, candy_addr, price, 0i32.into());
            let amount = xrd_amount/new_price;
            amount
        }
            // get_xrd_buy_amount => buy_xrd_sell_exact_candy (bxsec)
        pub fn  get_xrd_buy_amount_bxsec(&mut self, candy_addr: Address, candy_amnt: Decimal) -> Decimal {   //buy_xrd  
            let price = CandyStore::candyprice(self, candy_addr);
            let new_price: Decimal = CandyStore::price_mod(self, candy_amnt, candy_addr, price, 0i32.into());
            let amount = candy_amnt*new_price;
            let amount_final = amount-amount*self.fee/100;
            amount_final
        }
            // get_candy_buy_amount => buy_candy_sell_exact_xrd (bcsex)
        pub fn get_candy_buy_amount_bcsex(&mut self, candy_addr: Address, xrd_amnt: Decimal) -> Decimal {    //buy_candy 
            let price = CandyStore::candyprice(self, candy_addr);
            let new_price: Decimal = CandyStore::price_mod(self, xrd_amnt/price, candy_addr, price, 1i32.into());
            let amount = xrd_amnt/new_price;
            let amount_final = amount-amount*self.fee/100;
            amount_final
        }
            // get_xrd_sell_amount => buy_exact_candy_sell_xrd (becsx)
        pub fn get_xrd_sell_amount_becsx(&mut self, candy_addr: Address, candy_amnt: Decimal) -> Decimal {   //buy_candy 
            let candy_amount = CandyStore::adjust_fee(self, candy_amnt);
            let price = CandyStore::candyprice(self, candy_addr);
            let new_price: Decimal = CandyStore::price_mod(self,  candy_amount, candy_addr, price, 1i32.into());
            let amount = candy_amount*new_price;
            amount
        }
            //get_candy_sell_amount => buy_exact_candy_sell_candy (becsc)
        pub fn get_candy_sell_amount_becsc(&mut self, amnt_in: Decimal, addr_in: Address, addr_out: Address) -> Decimal {     
            let amount_in = CandyStore::adjust_fee(self, amnt_in);
            let amount_out = CandyStore::candy_sum(self, amount_in, addr_in, addr_out, 1i32.into());
            amount_out
        }
            //get_candy_sell_amount => buy_candy_sell_exact_candy (bcsec)                             
        pub fn get_candy_buy_amount_bcsec(&mut self, addr_in: Address, amnt_out: Decimal, addr_out: Address) -> Decimal {  
            let amount_out = CandyStore::adjust_fee(self, amnt_out);   
            let amount = CandyStore::candy_sum(self, amount_out, addr_out, addr_in, 1i32.into());
            let amount_final = amount-amount*self.fee/100;
            amount_final
        }
        
        #[auth(owner_badge)]
        pub fn set_fee(&mut self, prtcl_fee: Decimal) {
            assert!(prtcl_fee >= 0.into() && prtcl_fee <= 1.into()," Let's pass a fee in thousandths !");
            self.fee = prtcl_fee;
            info!(" Protocol fee set to {}% ", self.fee);
        }

        #[auth(owner_badge)]
        pub fn claim_xrd_fee(&mut self) -> Bucket {
            info!(" Fee value {} XRD ", self.xrd_fee);
            let xrd_amount = self.collected_xrd.amount();
            let xrd_output: Bucket = self.collected_xrd.take(self.xrd_fee);
            self.xrd_claimed += xrd_amount-self.collected_xrd.amount();
            info!(" Protocol fee claimed {} XRD ", self.xrd_claimed);
            xrd_output
        }

        pub fn stock_candy(&mut self, candy: Bucket, new_price: Decimal, candy_name: String, candy_symbol: String) -> Bucket {
            let candy_addr = candy.resource_address();
            let candy_amnt = candy.amount();
    
            assert!( candy_addr != self.collected_xrd.resource_address(), "cannot stock XRD as candy");
            assert!(new_price > 0.into(), "new price must be a positive value");
            
            match self.prices.get(&candy_addr) {
                  Some(&_price) => { info!(" Candy already in Vault. Please use restock_candy function");
                                    std::process::abort()
                                  }
                  _ => info!(" Added {} {} candy, {} symbol @{}XRD price ", candy_amnt, candy_name, candy_symbol ,new_price)
            }

            let v = self.candy_vaults.entry(candy_addr).or_insert(Vault::new(candy_addr));
            v.put(candy);
             
            self.prices.insert(candy_addr, new_price);
            self.name.insert(candy_addr, candy_name.clone());
            self.symbol.insert(candy_addr, candy_symbol.clone());

            match self.candy_ratio.get(&candy_addr) {
                  Some(&ratio) => info!(" Now ratio is: {}", ratio),
                  _ => { info!(" New inserted ratio equals 1 .");
                         self.candy_ratio.insert(candy_addr,1i32.into());
                       },
            }
            let ratio = *self.candy_ratio.get(&candy_addr).unwrap();
            CandyStore::add_meta_candy(self, candy_name, candy_symbol, candy_addr);
            let meta_candy: Bucket = CandyStore::meta_mint(self, candy_amnt, candy_addr, ratio);
         
            meta_candy
        }

        pub fn restock_candy(&mut self, candy: Bucket) -> Bucket {
            let candy_addr = candy.resource_address();
            assert!( candy_addr != self.collected_xrd.resource_address(), "cannot stock XRD as candy");
            let amnt = candy.amount();
            let candy_name = &*self.name.get(&candy_addr).unwrap();
            let name = candy_name.to_string();
            let candy_symbol = &*self.symbol.get(&candy_addr).unwrap();
            let symbol = candy_symbol.to_string();
            let price = *self.prices.get(&candy_addr).unwrap();
            let ratio = *self.candy_ratio.get(&candy_addr).unwrap();

            match self.prices.get(&candy_addr) {
             Some(&_price) => info!(" Added {} {} candy, {} symbol @{}XRD price, ratio {} ", amnt, name, symbol , price, ratio), 
                         _ => { info!(" Found no candy in Vault. Please use stock_candy function");
                                std::process::abort()
                              }                
            }

            let v = self.candy_vaults.entry(candy_addr).or_insert(Vault::new(candy_addr));
            v.put(candy);
            let meta_candy: Bucket = CandyStore::meta_mint(self, amnt, candy_addr, ratio);
            meta_candy
        }

        pub fn unstock_candy(&mut self, candy_addr: Address, meta_candy: Bucket ) -> (Bucket,Bucket) {
            let meta_candy_amnt: Decimal = meta_candy.amount(); 
            let ratio: Decimal;
            let candy_out_nbr: Decimal;
            let candy_bucket: Bucket;
            let xrd_out: Bucket;
            ratio = *self.candy_ratio.get(&candy_addr).unwrap();
            candy_out_nbr = meta_candy_amnt*ratio; 
            let total_candy = self.candy_vaults.get(&candy_addr).unwrap().amount();
            if candy_out_nbr <= total_candy {
                    candy_bucket = match self.candy_vaults.get(&candy_addr) {
                        Some(vault) => vault.take(candy_out_nbr),
                        None => {
                            info!("Candy not in stock !");
                            std::process::abort()
                        }
                    };
                    let zero: Decimal = 0i32.into();
                    xrd_out = self.collected_xrd.take(zero); 
            }else{  let delta_candy = candy_out_nbr-total_candy;   
                    candy_bucket = match self.candy_vaults.get(&candy_addr) {
                        Some(vault) => vault.take(total_candy),
                        None => {
                            info!("Candy not in stock !");
                            std::process::abort()
                        }
                    }; 
                    let price_in: Decimal = CandyStore::candyprice(self, candy_addr);
                    let xrd_amnt = delta_candy*price_in;
                    assert!( xrd_amnt <= self.collected_xrd.amount(), " Not enough XRD in Vault ");
                    xrd_out = self.collected_xrd.take(xrd_amnt);
            }
            CandyStore::burn(self, meta_candy);

            (candy_bucket,xrd_out)
        } 

            fn burn(&mut self, meta_candy: Bucket) {
                self.minter_badge.authorize(|auth| meta_candy.burn_with_auth(auth));
            }

        pub fn get_price(&self, candy_addr: Address) {
                // Make sure the candy_addr is not XRD
            assert!( candy_addr != self.collected_xrd.resource_address(), "XRD is priceless");
                // Display name, symbol, price if present, display error otherwise
            match self.name.get(&candy_addr) {
             Some(_name) => {
              match self.symbol.get(&candy_addr) {
               Some(_symbol) => { 
                match self.candy_ratio.get(&candy_addr) {
                 Some(_candy_ratio) => {
                  match self.prices.get(&candy_addr) {
                   Some(_price) => info!(" Address:{} name:{}({}) price:{} XRD, ratio:{}", candy_addr, _name, _symbol, _price, _candy_ratio),
                   None => info!("Could not find candy in stock !")};},
                 None => info!("Could not find candy in stock !")};},   
               None => info!("Could not find candy in stock !")};},
             None => info!("Could not find candy in stock !")};            
        }

        pub fn get_reserve(&self, candy_addr: Address) {
            match self.name.get(&candy_addr) {
             Some(_name) => {   let total_candy = self.candy_vaults.get(&candy_addr).unwrap().amount();
                                info!(" {} candy reserve amount is {} ", _name, total_candy);
                            },
             None => {info!(" Could not find candy in stock !");       
                      std::process::abort()
                     } 
            }            
        }

        pub fn menu(&self){
            for (addr_name,str_name) in self.name.iter() {
             for (addr_sym,str_sym) in self.symbol.iter() {
              for (addr_pri,decimal) in self.prices.iter() {
               if addr_name == addr_sym && addr_name == addr_pri {
                info!(" At address {} we've got {} ({}) candies at {} XRD each ", addr_name, str_name, str_sym, decimal);
               }
              }
             }
            }
        } 
     
        pub fn buy_candy_sell_exact_xrd(&mut self, min_in: Decimal, addr_in: Address, xrd_out: Bucket) -> Bucket { //ok
            let xrd_amnt = xrd_out.amount();
            self.collected_xrd.put(xrd_out);
            let amount_in = CandyStore::candy_sum(self, xrd_amnt, addr_in, addr_in, 2i32.into());
            assert!( amount_in >= min_in, "Not enough candies output amount");
            let candy_bucket: Bucket = CandyStore::candytake(self, amount_in, addr_in);
            candy_bucket
        }

        pub fn buy_candy_sell_exact_candy(&mut self, min_in: Decimal, addr_in: Address, candy_out: Bucket) -> Bucket { 
            let addr_out = candy_out.resource_address();
            assert!(addr_in != addr_out," Same candy's address detect! ");
            let (_nmbr,amount_in) = CandyStore::candyput_pri(self, 0i32.into(), 1i32.into(), addr_in, candy_out);         
            assert!( amount_in >= min_in, "Not enough candies output amount");
            let candy_bucket: Bucket = CandyStore::candytake(self, amount_in, addr_in);
            candy_bucket
        }

        pub fn buy_xrd_sell_exact_candy(&mut self, xrd_min: Decimal, candy_out: Bucket) -> Bucket {  
            let addr: Address = candy_out.resource_address();
            let price_out: Decimal = CandyStore::candyprice(self, candy_out.resource_address());
            assert!( candy_out.amount()*price_out <= self.collected_xrd.amount(), "Not enough XRD in Vault");
            let new_price: Decimal = CandyStore::price_mod(self, candy_out.amount(), addr, price_out, 0i32.into());  
            self.prices.insert(addr, new_price);
            let (nmbr,_amount_in) = CandyStore::candyput_pri(self, new_price*new_price, new_price, addr, candy_out);
            assert!( nmbr >= xrd_min , "Not enough xrd output amount");
            let candy_bucket = self.collected_xrd.take(*&(nmbr-nmbr*self.fee/100));
            self.xrd_fee = self.xrd_fee+nmbr*self.fee/100;
            candy_bucket
        }   
               
        pub fn buy_exact_candy_sell_xrd(&mut self, nbr_in: Decimal, addr_in: Address, xrd_out: Bucket) -> (Bucket,Bucket) { //here
            let amnt_in = CandyStore::adjust_fee(self, nbr_in);  
            let mut xrd_amnt = CandyStore::candy_sum(self, amnt_in, addr_in, addr_in, 0i32.into());         
            xrd_amnt = amnt_in*amnt_in/xrd_amnt; 
            assert!( xrd_amnt <=  xrd_out.amount(), " Not enough XRD input");
            self.collected_xrd.put(xrd_out.take(xrd_amnt));          
            let candy_bucket: Bucket = CandyStore::candytake(self, amnt_in, addr_in);
            (candy_bucket,xrd_out)
        }     
 
        pub fn buy_exact_candy_sell_candy(&mut self, amnt_in: Decimal, addr_in: Address, candy_out: Bucket) -> (Bucket,Bucket) {
            let addr_out = candy_out.resource_address();
            assert!(addr_in != addr_out," Same candy's address detect! ");
            let amount_in = CandyStore::adjust_fee(self, amnt_in);
            let candy_output: Bucket = CandyStore::candyput_sec(self, amount_in, addr_in, 0i32.into(), 1i32.into(), candy_out);
            let candy_bucket: Bucket = CandyStore::candytake(self, amount_in, addr_in);
            (candy_output,candy_bucket)
        }  

        pub fn buy_exact_xrd_sell_candy(&mut self, xrd_in: Decimal, candy_out: Bucket) -> (Bucket,Bucket) {   //this no        
            let addr = candy_out.resource_address();
            let xrd_input = CandyStore::adjust_fee(self, xrd_in); 
            let price_out: Decimal = CandyStore::candyprice(self, candy_out.resource_address());
            assert!( xrd_in <= self.collected_xrd.amount(), "Not enough XRD in Vault");     
            let new_price: Decimal = CandyStore::price_mod(self, xrd_input/price_out, addr, price_out, 0i32.into()); 
            self.prices.insert(addr, new_price);       
            let candy_output: Bucket = CandyStore::candyput_sec(self, xrd_input, addr, 1i32.into(), new_price, candy_out);             
            let candy_bucket = self.collected_xrd.take(*&(xrd_input-xrd_input*self.fee/100));
            self.xrd_fee = self.xrd_fee+xrd_input*self.fee/100;
            (candy_output,candy_bucket)
        }  
    }
} 
           
#[derive(Debug, Clone, TypeId, Encode, Decode, Describe, PartialEq, Eq)]
pub struct MetaToken {
    candy_name: String,
    candy_symbol: String,
    candy_address: Address,
    meta_res_def: ResourceDef
}

impl MetaToken {
    pub fn new(candy_name: String, candy_symbol: String, candy_address: Address, meta_res_def: ResourceDef ) -> Self {
        Self {
            candy_name,
            candy_symbol,
            candy_address,
            meta_res_def
        }
    }
}                  


