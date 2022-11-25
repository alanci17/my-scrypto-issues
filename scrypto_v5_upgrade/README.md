-------------------------------------------------------------------------------------------
# Foo Marketplace issues test after upgrade to Scrypto v0.5.0/v0.6.0  
-------------------------------------------------------------------------------------------    

Original NFT Marketplace package was succesfully tested with Scrypto v0.4.0.
Once upgraded to Scrypto v0.6.0, most NFT buying methods, once called, returned a failed transaction.

Breakpoint seems to be on method ```crate::list::update```, where is enquired as well as updated NFT sale instance status.
However at the same time ```Foosquare::unstock_nft``` method that invokes same method above works. 

Once incorporated ```crate::list``` hashmap & methods directly into ```crate::foo_square``` and excluded ```crate::list``` from codeflow, 
algo runs smoothly (```"foo_square.rs"``` file).

Maybe issue is addressable to new owned component implementation within Scrypto v0.5.0 upgrade, or maybe I missed something on the way.

This is the Hashmap carrying NFT sale instance info & status that's updated by aforementioned method:

```
list_map: HashMap<
	(ResourceAddress,u128),
        (
        	Vec<(ResourceAddress,NonFungibleId,FooNFT)>,
                (u8,Decimal),
                (Decimal,Decimal,u64,Decimal,u64,u8,u128)		
        )
>
```
-------------------------------------------------------------------------------------------
# Index  
-------------------------------------------------------------------------------------------	

> [Part_1](#part_1) . Instantiate Dex Component, mint some resources 
>
> [Part_2](#part_2) . Instantiate Marketplace Component & Set current Epoch
>
> [Part_3](#part_3) . Mint NFT resource
>
> [Part_4](#part_4) . Stock NFT on Foo Marketplace on Normal sell mode
>
> [Part_5](#part_5) . Try to buy a NFT
>
> [Part_6](#part_6) . Try to place a buy proposal on a listed NFT 
>
> [Part_7](#part_7) . Unstock NFT
>
> [Part_8](#part_8) . Stock NFT on Foo Marketplace on Auction sell mode
>
> [Part_9](#part_9) . Try to place an auction bid on a listed NFT on Auction sell mode
>
> [Part_10](#part_10) . Set current Epoch & Unstock NFT 
>
> [Part_11](#part_11) . Stock NFT on Foo Marketplace on Raffle sell mode
>
> [Part_12](#part_12) . Try to buy some tickets on a listed NFT on Raffle sell mode 
>
> [Part_13](#part_13) . Set current Epoch & Unstock NFT
>
> [Part_14](#part_14) . Notes


#
### Part_1 
# Instantiate Dex Component, mint some coin
-------------------------------------------------------------------------------------------
 	
Premise: publish and instantiate a specific Dex Component (file ```lib(dex).rs```) ain't mandatory as I'm not gonna use it within any further invoked method,
anyway to instantiate the Marketplace Component is required a ComponentAddress, as a Dex is used within some methods.
A currency is also required so I'm gonna mint some resources. 
   
	
>```dex_instantiate.sh```
```
|
Transaction Status: COMMITTED SUCCESS
Transaction Fee: 0.056366 XRD burned, 0.0028183 XRD tipped to validators
Cost Units: 100000000 limit, 563660 consumed, 0.0000001 XRD per cost unit
Logs: 0
Instructions:
├─ CallMethod { component_address: account_sim1q0a7ecesc0aepgnz3v66zf52ssdgfn0nvtaqgtual2rq7mtrxn, method_name: "lock_fee", args: Struct(Decimal("10")) }
├─ CallFunction { package_address: package_sim1qydgklu9wh0kse9qky44uuw9n23q8j2ddw3zv7up5sjqm05cg5, blueprint_name: "CandyDex", method_name: "new", args: Struct(Decimal("1")) }
└─ CallMethod { component_address: account_sim1q0a7ecesc0aepgnz3v66zf52ssdgfn0nvtaqgtual2rq7mtrxn, method_name: "deposit_batch", args: Struct(Expression("ENTIRE_WORKTOP")) }
Instruction Outputs:
├─ ()
├─ Tuple(ComponentAddress("component_sim1qt3ysdg6sequcswl8mvyufctk8jqcu3mhja5r7uhe8xqc2dfj7"), Bucket(1028u32))
└─ ()
New Entities: 3
└─ Component: component_sim1qt3ysdg6sequcswl8mvyufctk8jqcu3mhja5r7uhe8xqc2dfj7
├─ Resource: resource_sim1qr7muqu8t24mgx5l5a0t94lprztcxhyzj2ssy93w5qtqfgyytr
└─ Resource: resource_sim1qzanvhsgl6wzed5vfk5x34k3aqaj84uwgk595epy4dysurgfe0
```

>```resim new-token-fixed --name "ABC" 100000 --symbol "ABC"``` 
```
|
└─ Resource: resource_sim1qq4zuwzl327fm6ddgxrac8lexu2ypwh3nkeqqdsceq8qzyem0x
```

[Back Up](#index)
#
### Part_2 
# Instantiate Marketplace Component & Set current Epoch
-------------------------------------------------------------------------------------------

Please publish the Marketplace package using ```"foo_square(issues).rs"``` file. 

>```Foosquare::new``` function requires ABC token resource address as well as a Dex component address as arguments.

>```foo_instantiate.sh```
```
|
Transaction Status: COMMITTED SUCCESS
Transaction Fee: 0.1470475 XRD burned, 0.007352375 XRD tipped to validators
Cost Units: 100000000 limit, 1470475 consumed, 0.0000001 XRD per cost unit
Logs: 0
Instructions:
├─ CallMethod { component_address: account_sim1q0a7ecesc0aepgnz3v66zf52ssdgfn0nvtaqgtual2rq7mtrxn, method_name: "lock_fee", args: Struct(Decimal("10")) }
├─ CallFunction { package_address: package_sim1qylst4639qrrrtrnsdl8krm26pua46nec4p2xv003a8q7skg7r, blueprint_name: "FooSquare", method_name: "new", args: Struct(Decimal("3"), ResourceAddress("resource_sim1qq4zuwzl327fm6ddgxrac8lexu2ypwh3nkeqqdsceq8qzyem0x"), ComponentAddress("component_sim1qt3ysdg6sequcswl8mvyufctk8jqcu3mhja5r7uhe8xqc2dfj7")) }
└─ CallMethod { component_address: account_sim1q0a7ecesc0aepgnz3v66zf52ssdgfn0nvtaqgtual2rq7mtrxn, method_name: "deposit_batch", args: Struct(Expression("ENTIRE_WORKTOP")) }
Instruction Outputs:
├─ ()
├─ Tuple(ComponentAddress("component_sim1qgu6yqxh53cyjf3v07cmq5aa0980p200xp87y3tc396sxd5hx6"), Bucket(1028u32), Bucket(1030u32))
└─ ()
New Entities: 9
├─ Component: component_sim1qgu6yqxh53cyjf3v07cmq5aa0980p200xp87y3tc396sxd5hx6	FooSquare Component address
├─ Component: component_sim1q2l0txaem9hehmf0hzqg6zpxflfp52nutvlcwfamymdq2l5rtt
├─ Component: component_sim1qf52f8k5w9fh2mp6gymmgxdaap0zzw5q4p2unfpgsp2qmeupye
└─ Component: component_sim1qfua65fds2wlah2693te8f9zlpxfrgtlykdrfx3jzllsz4u2cy
├─ Resource: resource_sim1qz34guhnus230hed76h875889ydxnz82g2qr0rnl5rps4kuaz3
├─ Resource: resource_sim1qzh2y74shv68av697tg8lk2fa23x5vztndnsfsgjmnwqzdz8pk
├─ Resource: resource_sim1qrxd4xfsqsr6da7g24c7vyg68rpgmfxk98vrc6yv0svqdj3nec
├─ Resource: resource_sim1qq44xn5m6nmv0yavnajr2yc60r47z8q0ltnf4t3jr9xqa78nt6
└─ Resource: resource_sim1qzmlqhpyrf0gzmue2d2nrwz342fkfsy7m55c2eyxs79q30pc7g
```

>```resim set-current-epoch 1```


[Back Up](#index)
#
### Part_3 
# Mint NFT resource
-------------------------------------------------------------------------------------------


>```mint_nft.sh```
```
|
Transaction Status: COMMITTED SUCCESS
Transaction Fee: 0.0720466 XRD burned, 0.00360233 XRD tipped to validators
Cost Units: 100000000 limit, 720466 consumed, 0.0000001 XRD per cost unit
Logs: 3
├─ [INFO ]  Foo_NFT_id 300710000000eba21559b6dfd5b7f617c1129461d0de
├─ [INFO ]  Foo_NFT_res_addr resource_sim1qzmlqhpyrf0gzmue2d2nrwz342fkfsy7m55c2eyxs79q30pc7g
└─ [INFO ]  ========================================================================
Instructions:
├─ CallMethod { component_address: account_sim1q0a7ecesc0aepgnz3v66zf52ssdgfn0nvtaqgtual2rq7mtrxn, method_name: "lock_fee", args: Struct(Decimal("10")) }
├─ CallMethod { component_address: component_sim1qgu6yqxh53cyjf3v07cmq5aa0980p200xp87y3tc396sxd5hx6, method_name: "mint_nft", args: Struct() }
└─ CallMethod { component_address: account_sim1q0a7ecesc0aepgnz3v66zf52ssdgfn0nvtaqgtual2rq7mtrxn, method_name: "deposit_batch", args: Struct(Expression("ENTIRE_WORKTOP")) }
Instruction Outputs:
├─ ()
├─ Bucket(1027u32)
└─ ()
New Entities: 0
```

[Back Up](#index)
#
### Part_4 
# Stock NFT on Foo Marketplace on Normal sell mode
-------------------------------------------------------------------------------------------

>```stock_nft_normal.sh```
```
|
Transaction Status: COMMITTED SUCCESS
Transaction Fee: 0.3732654 XRD burned, 0.01866327 XRD tipped to validators
Cost Units: 100000000 limit, 3732654 consumed, 0.0000001 XRD per cost unit
Logs: 6
├─ [INFO ]  Instance number: 1
├─ [INFO ]  Added 1 NFT, resource_sim1qzmlqhpyrf0gzmue2d2nrwz342fkfsy7m55c2eyxs79q30pc7g ResAddress Foo Ecosystem Alpha Series 1 Number
├─ [INFO ]  UUID: 300710000000eba21559b6dfd5b7f617c1129461d0de Traits:  1st: pink 2nd: pink   3rd: blue 4th: orange  @20 $ABC
├─ [INFO ]  meta_NFT_id 300710000000f88cde715ae3cf47de9c734b16c4606d
├─ [INFO ]  meta_NFT_res_addr resource_sim1qqw3hjt4k5lppwp5sgmsm87xm3937fc3dmhhp0mk77gs3zz69v
└─ [INFO ]  ========================================================================
Instructions:
├─ CallMethod { component_address: account_sim1q0a7ecesc0aepgnz3v66zf52ssdgfn0nvtaqgtual2rq7mtrxn, method_name: "lock_fee", args: Struct(Decimal("10")) }
├─ CallMethod { component_address: account_sim1q0a7ecesc0aepgnz3v66zf52ssdgfn0nvtaqgtual2rq7mtrxn, method_name: "withdraw_by_ids", args: Struct(Set<NonFungibleId>(NonFungibleId("300710000000eba21559b6dfd5b7f617c1129461d0de")), ResourceAddress("resource_sim1qzmlqhpyrf0gzmue2d2nrwz342fkfsy7m55c2eyxs79q30pc7g")) }
├─ TakeFromWorktopByIds { ids: {300710000000eba21559b6dfd5b7f617c1129461d0de}, resource_address: resource_sim1qzmlqhpyrf0gzmue2d2nrwz342fkfsy7m55c2eyxs79q30pc7g }
├─ CallMethod { component_address: account_sim1q0a7ecesc0aepgnz3v66zf52ssdgfn0nvtaqgtual2rq7mtrxn, method_name: "withdraw_by_amount", args: Struct(Decimal("1"), ResourceAddress("resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqu57yag")) }
├─ TakeFromWorktopByAmount { amount: 1, resource_address: resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqu57yag }
├─ CallMethod { component_address: component_sim1qgu6yqxh53cyjf3v07cmq5aa0980p200xp87y3tc396sxd5hx6, method_name: "stock_nft", args: Struct(Vec<Bucket>(Bucket(512u32)), Bucket(513u32), Decimal("20"), false, false, Decimal("20"), 4000u64, Decimal("4"), Decimal("3")) }
└─ CallMethod { component_address: account_sim1q0a7ecesc0aepgnz3v66zf52ssdgfn0nvtaqgtual2rq7mtrxn, method_name: "deposit_batch", args: Struct(Expression("ENTIRE_WORKTOP")) }
Instruction Outputs:
├─ ()
├─ Bucket(1025u32)
├─ Bucket(512u32)
├─ Bucket(1027u32)
├─ Bucket(513u32)
├─ Tuple(Vec<Bucket>(Bucket(1033u32)), Bucket(1028u32), Bucket(1038u32))
└─ ()
New Entities: 2
├─ Resource: resource_sim1qqw3hjt4k5lppwp5sgmsm87xm3937fc3dmhhp0mk77gs3zz69v	Meta NFT resource address
└─ Resource: resource_sim1qqqxrkq3pc9pfefn6h7sqzmujmkptqv69f8srudk95gqj8pj65	Seller Badge resource address
```

[Back Up](#index)
#
### Part_5 
# Try to buy a NFT
-------------------------------------------------------------------------------------------


>```buy_nft.sh```
```
|
Transaction Status: COMMITTED FAILURE: KernelError(WasmError(WasmError("Trap(Trap { kind: Unreachable })")))
Transaction Fee: 0.222987 XRD burned, 0.01114935 XRD tipped to validators
Cost Units: 100000000 limit, 2229870 consumed, 0.0000001 XRD per cost unit
Logs: 1
└─ [INFO ]  Panic? @ [crate::list::update]
Instructions:
├─ CallMethod { component_address: account_sim1q0a7ecesc0aepgnz3v66zf52ssdgfn0nvtaqgtual2rq7mtrxn, method_name: "lock_fee", args: Struct(Decimal("10")) }
├─ CallMethod { component_address: account_sim1q0a7ecesc0aepgnz3v66zf52ssdgfn0nvtaqgtual2rq7mtrxn, method_name: "withdraw_by_amount", args: Struct(Decimal("20"), ResourceAddress("resource_sim1qq4zuwzl327fm6ddgxrac8lexu2ypwh3nkeqqdsceq8qzyem0x")) }
├─ TakeFromWorktopByAmount { amount: 20, resource_address: resource_sim1qq4zuwzl327fm6ddgxrac8lexu2ypwh3nkeqqdsceq8qzyem0x }
├─ CallMethod { component_address: account_sim1q0a7ecesc0aepgnz3v66zf52ssdgfn0nvtaqgtual2rq7mtrxn, method_name: "create_proof_by_amount", args: Struct(Decimal("1"), ResourceAddress("resource_sim1qq4zuwzl327fm6ddgxrac8lexu2ypwh3nkeqqdsceq8qzyem0x")) }
├─ PopFromAuthZone
├─ CallMethod { component_address: component_sim1qgu6yqxh53cyjf3v07cmq5aa0980p200xp87y3tc396sxd5hx6, method_name: "buy_nft", args: Struct(1u128, Bucket(512u32), Proof(513u32)) }
├─ DropAllProofs
└─ CallMethod { component_address: account_sim1q0a7ecesc0aepgnz3v66zf52ssdgfn0nvtaqgtual2rq7mtrxn, method_name: "deposit_batch", args: Struct(Expression("ENTIRE_WORKTOP")) }
New Entities: 0
```

[Back Up](#index)
#
### Part_6 
# Try to place a buy proposal on a listed NFT
-------------------------------------------------------------------------------------------

>```buy_proposal.sh```
```
|
Transaction Status: COMMITTED FAILURE: KernelError(WasmError(WasmError("Trap(Trap { kind: Unreachable })")))
Transaction Fee: 0.2240818 XRD burned, 0.01120409 XRD tipped to validators
Cost Units: 100000000 limit, 2240818 consumed, 0.0000001 XRD per cost unit
Logs: 1
└─ [INFO ]  Panic? @ [crate::list::update]
Instructions:
├─ CallMethod { component_address: account_sim1q0a7ecesc0aepgnz3v66zf52ssdgfn0nvtaqgtual2rq7mtrxn, method_name: "lock_fee", args: Struct(Decimal("10")) }
├─ CallMethod { component_address: account_sim1q0a7ecesc0aepgnz3v66zf52ssdgfn0nvtaqgtual2rq7mtrxn, method_name: "withdraw_by_amount", args: Struct(Decimal("19"), ResourceAddress("resource_sim1qq4zuwzl327fm6ddgxrac8lexu2ypwh3nkeqqdsceq8qzyem0x")) }
├─ TakeFromWorktopByAmount { amount: 19, resource_address: resource_sim1qq4zuwzl327fm6ddgxrac8lexu2ypwh3nkeqqdsceq8qzyem0x }
├─ CallMethod { component_address: account_sim1q0a7ecesc0aepgnz3v66zf52ssdgfn0nvtaqgtual2rq7mtrxn, method_name: "create_proof_by_amount", args: Struct(Decimal("1"), ResourceAddress("resource_sim1qq4zuwzl327fm6ddgxrac8lexu2ypwh3nkeqqdsceq8qzyem0x")) }
├─ PopFromAuthZone
├─ CallMethod { component_address: component_sim1qgu6yqxh53cyjf3v07cmq5aa0980p200xp87y3tc396sxd5hx6, method_name: "buy_proposal", args: Struct(1u128, Bucket(512u32), Decimal("19"), 4000u64, Proof(513u32)) }
├─ DropAllProofs
└─ CallMethod { component_address: account_sim1q0a7ecesc0aepgnz3v66zf52ssdgfn0nvtaqgtual2rq7mtrxn, method_name: "deposit_batch", args: Struct(Expression("ENTIRE_WORKTOP")) }
New Entities: 0
```

[Back Up](#index)
#
### Part_7 
# Unstock NFT
-------------------------------------------------------------------------------------------

>```Foosquare::unstock_nft``` method requires metaNFT previously minted within ```Foosquare::stock_nft``` method as argument.
	
>```unstock_nft.sh```
```
|
Transaction Status: COMMITTED SUCCESS
Transaction Fee: 0.4357842 XRD burned, 0.02178921 XRD tipped to validators
Cost Units: 100000000 limit, 4357842 consumed, 0.0000001 XRD per cost unit
Logs: 6
├─ [INFO ]  Panic? @ [crate::list::update]
├─ [INFO ]  ==============================================
├─ [INFO ]  NFT collected
├─ [INFO ]  resource address: resource_sim1qzmlqhpyrf0gzmue2d2nrwz342fkfsy7m55c2eyxs79q30pc7g
├─ [INFO ]  key: 300710000000eba21559b6dfd5b7f617c1129461d0de
└─ [INFO ]  ==============================================
Instructions:
├─ CallMethod { component_address: account_sim1q0a7ecesc0aepgnz3v66zf52ssdgfn0nvtaqgtual2rq7mtrxn, method_name: "lock_fee", args: Struct(Decimal("10")) }
├─ CallMethod { component_address: account_sim1q0a7ecesc0aepgnz3v66zf52ssdgfn0nvtaqgtual2rq7mtrxn, method_name: "withdraw_by_ids", args: Struct(Set<NonFungibleId>(NonFungibleId("300710000000f88cde715ae3cf47de9c734b16c4606d")), ResourceAddress("resource_sim1qqw3hjt4k5lppwp5sgmsm87xm3937fc3dmhhp0mk77gs3zz69v")) }
├─ TakeFromWorktopByIds { ids: {300710000000f88cde715ae3cf47de9c734b16c4606d}, resource_address: resource_sim1qqw3hjt4k5lppwp5sgmsm87xm3937fc3dmhhp0mk77gs3zz69v }
├─ CallMethod { component_address: component_sim1qgu6yqxh53cyjf3v07cmq5aa0980p200xp87y3tc396sxd5hx6, method_name: "unstock_nft", args: Struct(Vec<Bucket>(Bucket(512u32))) }
└─ CallMethod { component_address: account_sim1q0a7ecesc0aepgnz3v66zf52ssdgfn0nvtaqgtual2rq7mtrxn, method_name: "deposit_batch", args: Struct(Expression("ENTIRE_WORKTOP")) }
Instruction Outputs:
├─ ()
├─ Bucket(1025u32)
├─ Bucket(512u32)
├─ Tuple(Vec<Bucket>(Bucket(1029u32)), Bucket(1030u32))
└─ ()
New Entities: 0
```

[Back Up](#index)
#
### Part_8 
# Stock NFT on Foo Marketplace on Auction sell mode
-------------------------------------------------------------------------------------------

>```stock_nft_auction.sh```
```
|
Transaction Status: COMMITTED SUCCESS
Transaction Fee: 0.3732654 XRD burned, 0.01866327 XRD tipped to validators
Cost Units: 100000000 limit, 3732654 consumed, 0.0000001 XRD per cost unit
Logs: 6
├─ [INFO ]  Instance number: 2
├─ [INFO ]  Added 1 NFT, resource_sim1qzmlqhpyrf0gzmue2d2nrwz342fkfsy7m55c2eyxs79q30pc7g ResAddress Foo Ecosystem Alpha Series 1 Number
├─ [INFO ]  UUID: 300710000000eba21559b6dfd5b7f617c1129461d0de Traits:  1st: pink 2nd: pink   3rd: blue 4th: orange  @20 $ABC
├─ [INFO ]  meta_NFT_id 300710000000f11f730c9a02505dde791663dff378d8
├─ [INFO ]  meta_NFT_res_addr resource_sim1qqw3hjt4k5lppwp5sgmsm87xm3937fc3dmhhp0mk77gs3zz69v
└─ [INFO ]  ========================================================================
Instructions:
├─ CallMethod { component_address: account_sim1q0a7ecesc0aepgnz3v66zf52ssdgfn0nvtaqgtual2rq7mtrxn, method_name: "lock_fee", args: Struct(Decimal("10")) }
├─ CallMethod { component_address: account_sim1q0a7ecesc0aepgnz3v66zf52ssdgfn0nvtaqgtual2rq7mtrxn, method_name: "withdraw_by_ids", args: Struct(Set<NonFungibleId>(NonFungibleId("300710000000eba21559b6dfd5b7f617c1129461d0de")), ResourceAddress("resource_sim1qzmlqhpyrf0gzmue2d2nrwz342fkfsy7m55c2eyxs79q30pc7g")) }
├─ TakeFromWorktopByIds { ids: {300710000000eba21559b6dfd5b7f617c1129461d0de}, resource_address: resource_sim1qzmlqhpyrf0gzmue2d2nrwz342fkfsy7m55c2eyxs79q30pc7g }
├─ CallMethod { component_address: account_sim1q0a7ecesc0aepgnz3v66zf52ssdgfn0nvtaqgtual2rq7mtrxn, method_name: "withdraw_by_amount", args: Struct(Decimal("1"), ResourceAddress("resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqu57yag")) }
├─ TakeFromWorktopByAmount { amount: 1, resource_address: resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqu57yag }
├─ CallMethod { component_address: component_sim1qgu6yqxh53cyjf3v07cmq5aa0980p200xp87y3tc396sxd5hx6, method_name: "stock_nft", args: Struct(Vec<Bucket>(Bucket(512u32)), Bucket(513u32), Decimal("20"), false, false, Decimal("20"), 4000u64, Decimal("4"), Decimal("3")) }
└─ CallMethod { component_address: account_sim1q0a7ecesc0aepgnz3v66zf52ssdgfn0nvtaqgtual2rq7mtrxn, method_name: "deposit_batch", args: Struct(Expression("ENTIRE_WORKTOP")) }
Instruction Outputs:
├─ ()
├─ Bucket(1025u32)
├─ Bucket(512u32)
├─ Bucket(1027u32)
├─ Bucket(513u32)
├─ Tuple(Vec<Bucket>(Bucket(1033u32)), Bucket(1028u32), Bucket(1038u32))
└─ ()
New Entities: 1
└─ Resource: resource_sim1qpu7xg34qug3v25nz2zvj8qcpehzxt69lnfwlq2v0zpqg8y37p	Seller Badge resource address
```

[Back Up](#index)
#
### Part_9 
# Try to place an auction bid on a listed NFT on Auction sell mode
-------------------------------------------------------------------------------------------

>```place_bid.sh```
```
|
Transaction Status: COMMITTED FAILURE: KernelError(WasmError(WasmError("Trap(Trap { kind: Unreachable })")))
Transaction Fee: 0.2287537 XRD burned, 0.011437685 XRD tipped to validators
Cost Units: 100000000 limit, 2287537 consumed, 0.0000001 XRD per cost unit
Logs: 1
└─ [INFO ]  Panic? @ [crate::list::update]
Instructions:
├─ CallMethod { component_address: account_sim1q0a7ecesc0aepgnz3v66zf52ssdgfn0nvtaqgtual2rq7mtrxn, method_name: "lock_fee", args: Struct(Decimal("10")) }
├─ CallMethod { component_address: account_sim1q0a7ecesc0aepgnz3v66zf52ssdgfn0nvtaqgtual2rq7mtrxn, method_name: "withdraw_by_amount", args: Struct(Decimal("5"), ResourceAddress("resource_sim1qq4zuwzl327fm6ddgxrac8lexu2ypwh3nkeqqdsceq8qzyem0x")) }
├─ TakeFromWorktopByAmount { amount: 4, resource_address: resource_sim1qq4zuwzl327fm6ddgxrac8lexu2ypwh3nkeqqdsceq8qzyem0x }
├─ TakeFromWorktopByAmount { amount: 1, resource_address: resource_sim1qq4zuwzl327fm6ddgxrac8lexu2ypwh3nkeqqdsceq8qzyem0x }
├─ CallMethod { component_address: account_sim1q0a7ecesc0aepgnz3v66zf52ssdgfn0nvtaqgtual2rq7mtrxn, method_name: "create_proof_by_amount", args: Struct(Decimal("1"), ResourceAddress("resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqu57yag")) }
├─ PopFromAuthZone
├─ CallMethod { component_address: component_sim1qgu6yqxh53cyjf3v07cmq5aa0980p200xp87y3tc396sxd5hx6, method_name: "place_bid", args: Struct(2u128, Bucket(512u32), Bucket(513u32), Decimal("21"), Decimal("4"), Proof(514u32)) }
├─ DropAllProofs
└─ CallMethod { component_address: account_sim1q0a7ecesc0aepgnz3v66zf52ssdgfn0nvtaqgtual2rq7mtrxn, method_name: "deposit_batch", args: Struct(Expression("ENTIRE_WORKTOP")) }
New Entities: 0
```

[Back Up](#index)
#
### Part_10 
# Set current Epoch & Unstock NFT 
-------------------------------------------------------------------------------------------

Set current Epoch to terminate Auction instance

>```resim set-current-epoch 20000```

>```Foosquare::unstock_nft``` method requires metaNFT previously minted within ```Foosquare::stock_nft``` method as argument.

>```unstock_nft.sh```
```
|
Transaction Status: COMMITTED SUCCESS
Transaction Fee: 0.3829001 XRD burned, 0.019145005 XRD tipped to validators
Cost Units: 100000000 limit, 3829001 consumed, 0.0000001 XRD per cost unit
Logs: 6
├─ [INFO ]  Panic? @ [crate::list::update]
├─ [INFO ]  ==============================================
├─ [INFO ]  NFT collected
├─ [INFO ]  resource address: resource_sim1qzmlqhpyrf0gzmue2d2nrwz342fkfsy7m55c2eyxs79q30pc7g
├─ [INFO ]  key: 300710000000eba21559b6dfd5b7f617c1129461d0de
└─ [INFO ]  ==============================================
Instructions:
├─ CallMethod { component_address: account_sim1q0a7ecesc0aepgnz3v66zf52ssdgfn0nvtaqgtual2rq7mtrxn, method_name: "lock_fee", args: Struct(Decimal("10")) }
├─ CallMethod { component_address: account_sim1q0a7ecesc0aepgnz3v66zf52ssdgfn0nvtaqgtual2rq7mtrxn, method_name: "withdraw_by_ids", args: Struct(Set<NonFungibleId>(NonFungibleId("300710000000f11f730c9a02505dde791663dff378d8")), ResourceAddress("resource_sim1qqw3hjt4k5lppwp5sgmsm87xm3937fc3dmhhp0mk77gs3zz69v")) }
├─ TakeFromWorktopByIds { ids: {300710000000f11f730c9a02505dde791663dff378d8}, resource_address: resource_sim1qqw3hjt4k5lppwp5sgmsm87xm3937fc3dmhhp0mk77gs3zz69v }
├─ CallMethod { component_address: component_sim1qgu6yqxh53cyjf3v07cmq5aa0980p200xp87y3tc396sxd5hx6, method_name: "unstock_nft", args: Struct(Vec<Bucket>(Bucket(512u32))) }
└─ CallMethod { component_address: account_sim1q0a7ecesc0aepgnz3v66zf52ssdgfn0nvtaqgtual2rq7mtrxn, method_name: "deposit_batch", args: Struct(Expression("ENTIRE_WORKTOP")) }
Instruction Outputs:
├─ ()
├─ Bucket(1025u32)
├─ Bucket(512u32)
├─ Tuple(Vec<Bucket>(Bucket(1029u32)), Bucket(1030u32))
└─ ()
New Entities: 0
```

[Back Up](#index)
#
### Part_11
# Stock NFT on Foo Marketplace on Raffle sell mode 
-------------------------------------------------------------------------------------------


>```stock_nft_raffle.sh```
```
|
Transaction Status: COMMITTED SUCCESS
Transaction Fee: 0.4280108 XRD burned, 0.02140054 XRD tipped to validators
Cost Units: 100000000 limit, 4280108 consumed, 0.0000001 XRD per cost unit
Logs: 6
├─ [INFO ]  Instance number: 3
├─ [INFO ]  Added 1 NFT, resource_sim1qzmlqhpyrf0gzmue2d2nrwz342fkfsy7m55c2eyxs79q30pc7g ResAddress Foo Ecosystem Alpha Series 1 Number
├─ [INFO ]  UUID: 300710000000eba21559b6dfd5b7f617c1129461d0de Traits:  1st: pink 2nd: pink   3rd: blue 4th: orange  @0 $ABC
├─ [INFO ]  meta_NFT_id 300710000000e467e6a06f3818abe357593d4dd3ab4f
├─ [INFO ]  meta_NFT_res_addr resource_sim1qqw3hjt4k5lppwp5sgmsm87xm3937fc3dmhhp0mk77gs3zz69v
└─ [INFO ]  ========================================================================
Instructions:
├─ CallMethod { component_address: account_sim1q0a7ecesc0aepgnz3v66zf52ssdgfn0nvtaqgtual2rq7mtrxn, method_name: "lock_fee", args: Struct(Decimal("10")) }
├─ CallMethod { component_address: account_sim1q0a7ecesc0aepgnz3v66zf52ssdgfn0nvtaqgtual2rq7mtrxn, method_name: "withdraw_by_ids", args: Struct(Set<NonFungibleId>(NonFungibleId("300710000000eba21559b6dfd5b7f617c1129461d0de")), ResourceAddress("resource_sim1qzmlqhpyrf0gzmue2d2nrwz342fkfsy7m55c2eyxs79q30pc7g")) }
├─ TakeFromWorktopByIds { ids: {300710000000eba21559b6dfd5b7f617c1129461d0de}, resource_address: resource_sim1qzmlqhpyrf0gzmue2d2nrwz342fkfsy7m55c2eyxs79q30pc7g }
├─ CallMethod { component_address: account_sim1q0a7ecesc0aepgnz3v66zf52ssdgfn0nvtaqgtual2rq7mtrxn, method_name: "withdraw_by_amount", args: Struct(Decimal("1"), ResourceAddress("resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqu57yag")) }
├─ TakeFromWorktopByAmount { amount: 1, resource_address: resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqu57yag }
├─ CallMethod { component_address: component_sim1qgu6yqxh53cyjf3v07cmq5aa0980p200xp87y3tc396sxd5hx6, method_name: "stock_nft", args: Struct(Vec<Bucket>(Bucket(512u32)), Bucket(513u32), Decimal("0"), false, true, Decimal("100"), 4000u64, Decimal("0"), Decimal("1")) }
└─ CallMethod { component_address: account_sim1q0a7ecesc0aepgnz3v66zf52ssdgfn0nvtaqgtual2rq7mtrxn, method_name: "deposit_batch", args: Struct(Expression("ENTIRE_WORKTOP")) }
Instruction Outputs:
├─ ()
├─ Bucket(1025u32)
├─ Bucket(512u32)
├─ Bucket(1027u32)
├─ Bucket(513u32)
├─ Tuple(Vec<Bucket>(Bucket(1031u32)), Bucket(1028u32), Bucket(1036u32))
└─ ()
New Entities: 1
└─ Resource: resource_sim1qpu26ftf0cwtg2acl88uga5f05elyzr4yua0vv6xj2mq5we4s7	Seller Badge resource address
```

[Back Up](#index)
#
### Part_12
# Try to buy some tickets on a listed NFT on Raffle sell mode
-------------------------------------------------------------------------------------------

>```buy_ticket.sh```
```
|
Transaction Status: COMMITTED FAILURE: KernelError(WasmError(WasmError("Trap(Trap { kind: Unreachable })")))
Transaction Fee: 0.2489243 XRD burned, 0.012446215 XRD tipped to validators
Cost Units: 100000000 limit, 2489243 consumed, 0.0000001 XRD per cost unit
Logs: 1
└─ [INFO ]  Panic? @ [crate::list::update]
Instructions:
├─ CallMethod { component_address: account_sim1q0a7ecesc0aepgnz3v66zf52ssdgfn0nvtaqgtual2rq7mtrxn, method_name: "lock_fee", args: Struct(Decimal("10")) }
├─ CallMethod { component_address: account_sim1q0a7ecesc0aepgnz3v66zf52ssdgfn0nvtaqgtual2rq7mtrxn, method_name: "withdraw_by_amount", args: Struct(Decimal("25"), ResourceAddress("resource_sim1qq4zuwzl327fm6ddgxrac8lexu2ypwh3nkeqqdsceq8qzyem0x")) }
├─ TakeFromWorktopByAmount { amount: 25, resource_address: resource_sim1qq4zuwzl327fm6ddgxrac8lexu2ypwh3nkeqqdsceq8qzyem0x }
├─ CallMethod { component_address: account_sim1q0a7ecesc0aepgnz3v66zf52ssdgfn0nvtaqgtual2rq7mtrxn, method_name: "create_proof_by_amount", args: Struct(Decimal("1"), ResourceAddress("resource_sim1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzqu57yag")) }
├─ PopFromAuthZone
├─ CallMethod { component_address: component_sim1qgu6yqxh53cyjf3v07cmq5aa0980p200xp87y3tc396sxd5hx6, method_name: "buy_ticket", args: Struct(3u128, Bucket(512u32), 25u8, Proof(513u32)) }
├─ DropAllProofs
└─ CallMethod { component_address: account_sim1q0a7ecesc0aepgnz3v66zf52ssdgfn0nvtaqgtual2rq7mtrxn, method_name: "deposit_batch", args: Struct(Expression("ENTIRE_WORKTOP")) }
New Entities: 0
```

[Back Up](#index)
#
### Part_13
# Set current Epoch & Unstock NFT
-------------------------------------------------------------------------------------------

Set current Epoch to terminate Raffle instance

>```resim set-current-epoch 30000```

>```Foosquare::unstock_nft``` method requires metaNFT previously minted within ```Foosquare::stock_nft``` method as argument.

>```unstock_nft.sh```
```
|
Transaction Status: COMMITTED SUCCESS
Transaction Fee: 0.4252666 XRD burned, 0.02126333 XRD tipped to validators
Cost Units: 100000000 limit, 4252666 consumed, 0.0000001 XRD per cost unit
Logs: 6
├─ [INFO ]  Panic? @ [crate::list::update]
├─ [INFO ]  ==============================================
├─ [INFO ]  NFT collected
├─ [INFO ]  resource address: resource_sim1qzmlqhpyrf0gzmue2d2nrwz342fkfsy7m55c2eyxs79q30pc7g
├─ [INFO ]  key: 300710000000eba21559b6dfd5b7f617c1129461d0de
└─ [INFO ]  ==============================================
Instructions:
├─ CallMethod { component_address: account_sim1q0a7ecesc0aepgnz3v66zf52ssdgfn0nvtaqgtual2rq7mtrxn, method_name: "lock_fee", args: Struct(Decimal("10")) }
├─ CallMethod { component_address: account_sim1q0a7ecesc0aepgnz3v66zf52ssdgfn0nvtaqgtual2rq7mtrxn, method_name: "withdraw_by_ids", args: Struct(Set<NonFungibleId>(NonFungibleId("300710000000e467e6a06f3818abe357593d4dd3ab4f")), ResourceAddress("resource_sim1qqw3hjt4k5lppwp5sgmsm87xm3937fc3dmhhp0mk77gs3zz69v")) }
├─ TakeFromWorktopByIds { ids: {300710000000e467e6a06f3818abe357593d4dd3ab4f}, resource_address: resource_sim1qqw3hjt4k5lppwp5sgmsm87xm3937fc3dmhhp0mk77gs3zz69v }
├─ CallMethod { component_address: component_sim1qgu6yqxh53cyjf3v07cmq5aa0980p200xp87y3tc396sxd5hx6, method_name: "unstock_nft", args: Struct(Vec<Bucket>(Bucket(512u32))) }
└─ CallMethod { component_address: account_sim1q0a7ecesc0aepgnz3v66zf52ssdgfn0nvtaqgtual2rq7mtrxn, method_name: "deposit_batch", args: Struct(Expression("ENTIRE_WORKTOP")) }
Instruction Outputs:
├─ ()
├─ Bucket(1025u32)
├─ Bucket(512u32)
├─ Tuple(Vec<Bucket>(Bucket(1029u32)), Bucket(1030u32))
└─ ()
New Entities: 0
```

[Back Up](#index)
#
### Part_14
# Notes 
-------------------------------------------------------------------------------------------

Original Foosquare Blueprint ( first one I tried to update to Scrypto v0.6) had a sligtly different ```list_map``` HashMap, where every selling method 
managed his own data tuple:

```
list_map: HashMap<
	(ResourceAddress,u128),
        (
        	Vec<(ResourceAddress,NonFungibleId,RadishNFT)>,
                (u8,Decimal),
                (Decimal,Decimal,u64),					// Normal 
                (Decimal,Decimal,u64,Decimal,u64),			// Auction
                (Decimal,Decimal,u64,u8,u64,u128)			// Raffle
        )
>
```

instead of :


```
list_map: HashMap<
	(ResourceAddress,u128),
        (
        	Vec<(ResourceAddress,NonFungibleId,FooNFT)>,
                (u8,Decimal),
                (Decimal,Decimal,u64,Decimal,u64,u8,u128)		// Normal & Auction & Raffle
        )
>
```
With this original configuration, in NFT normal sale mode, ```FooSquare::buy_nft``` method worked, in reverse ```FooSquare::buy_nft_ext``` method doesn't, 
so when I tried to buy an NFT from an external Component throught component call it triggered a failed transaction.
NFT Auction sale mode and NFT Raffle sale mode never worked.


[Back Up](#index)
