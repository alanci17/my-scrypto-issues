use scrypto::prelude::*;
use crate::data::*;
use crate::info::*;

blueprint! {
    struct FooFarm {  
		// A vault that holds the mint badge
        foo_nft_minter_badge: Vault,
        // Resource definition of Radish NFT series
        foo_nft_res_def: ResourceAddress       
    }

    #[allow(dead_code)]
    impl FooFarm {
        pub fn new() -> FooFarmComponent {
        	// Create a Protocol Minter Badge resource
            let foo_nft_minter_badge = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_NONE)
                .metadata("Name", "Radish NFT Minter Badge")
                .initial_supply(1);
            // Create an NFT resource with mutable supply    
            let foo_nft_res_def = ResourceBuilder::new_non_fungible()
                .metadata("Ecosystem", "Foo")
                .metadata("Series", "Alpha")
                .metadata("Number", "1".to_string())
                .mintable(rule!(require(foo_nft_minter_badge.resource_address())), LOCKED)
                .burnable(rule!(require(foo_nft_minter_badge.resource_address())), LOCKED)
                .updateable_non_fungible_data(rule!(require(foo_nft_minter_badge.resource_address())), LOCKED)
                .no_initial_supply();
            Self {
                foo_nft_minter_badge: Vault::with_bucket(foo_nft_minter_badge),
                foo_nft_res_def
            }
            .instantiate()
        }

        // Mint one Foo NFT  
        pub fn nft_mint(&mut self) -> Bucket {   
        	let nft_key = NonFungibleId::random();          
            let out_url = "https://gistcdn.githack.com/alanci17/4010e8a0db866b7abb36ef89cca9b701/raw/5b305f012ca90cb3324a6ed7ccf484600ceb31f8/9991.svg".to_string();
            let tab = "\" \n \"".to_string();
            let svg_1 = "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"400\" height=\"400\"><circle cx=\"200\" cy=\"400\" r=\"200\" stroke=\"black\" stroke-width=\"5\" fill=\"pink\" />".to_string();
            let svg_2 = "<circle cx=\"0\" cy=\"0\" r=\"150\" stroke=\"black\" stroke-width=\"5\" fill=\"pink\" />".to_string();
            let svg_3 = "<circle cx=\"350\" cy=\"350\" r=\"175\" stroke=\"black\" stroke-width=\"5\" fill=\"blue\" />".to_string();
            let svg_4 = "<circle cx=\"350\" cy=\"100\" r=\"175\" stroke=\"black\" stroke-width=\"5\" fill=\"orange\" />".to_string();
            let svg_5 = "<circle cx=\"300\" cy=\"300\" r=\"150\" stroke=\"black\" stroke-width=\"5\" fill=\"green\" />".to_string();
            let svg_6 = "<circle cx=\"250\" cy=\"250\" r=\"125\" stroke=\"black\" stroke-width=\"5\" fill=\"red\" />".to_string();
            let svg_7 = "<circle cx=\"200\" cy=\"200\" r=\"100\" stroke=\"black\" stroke-width=\"5\" fill=\"palevioletred\" />".to_string();
            let svg_8 = "<circle cx=\"150\" cy=\"150\" r=\"75\" stroke=\"black\" stroke-width=\"5\" fill=\"turquoise\" /></svg>".to_string();
            let svg = tab + &svg_1 + &svg_2 + &svg_3 + &svg_4 + &svg_5 + &svg_6 + &svg_7 + &svg_8;
            let str_1 = " 1st: pink 2nd: pink ".to_string();
            let str_2 = " 3rd: blue 4th: orange ".to_string();
            let str_3 = " 5th: green 6th: red ".to_string();
            let str_4 = " 7th: palevioletred 8th: turquoise ".to_string();

            let new_nft = FooNFT {
                uri: out_url.to_owned() + &svg,
                data_1: str_1,
                data_2: str_2,
                data_3: str_3,
                data_4: str_4,
                value_1: 1,
                value_2: 1,
                value_3: 1
            };

            nft_mint(nft_key.clone(),self.foo_nft_res_def);
        
            self.foo_nft_minter_badge.authorize(|| { 
                borrow_resource_manager!(self.foo_nft_res_def).mint_non_fungible(&nft_key,new_nft)
            }) 
        }
    }
}    