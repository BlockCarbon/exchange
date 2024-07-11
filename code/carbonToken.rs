module BasicCarbonOffsets::BCO {
    use 0x1::Signer;
    use 0x1::Account;
    use 0x1::Coin;
    use 0x1::Event;
    use 0x1::Vector;
    
    struct MintEvent has copy, drop, store {
        amount: u64,
        to: address,
    }

    struct BurnEvent has copy, drop, store {
        amount: u64,
        from: address,
    }

    struct BCO has key {
        balance: Coin.T<BCO>,
        admin: address,
    }

    public fun initialize(account: &signer, initial_supply: u64) {
        let admin = Signer.address_of(account);
        let coin = Coin.mint<BCO>(initial_supply);
        let bco = BCO {
            balance: coin,
            admin: admin,
        };
        move_to(account, bco);
    }

    public fun mint(account: &signer, to: address, amount: u64) {
        let bco = borrow_global_mut<BCO>(Signer.address_of(account));
        assert!(Signer.address_of(account) == bco.admin, 1);
        let coin = Coin.mint<BCO>(amount);
        Coin.deposit(to, coin);
        Event::emit_event<MintEvent>(&bco.mint_events, MintEvent { amount, to });
    }

    public fun burn(account: &signer, amount: u64) {
        let bco = borrow_global_mut<BCO>(Signer.address_of(account));
        let from = Signer.address_of(account);
        let coin = Coin.withdraw<BCO>(&bco.balance, amount);
        Coin.burn(coin);
        Event::emit_event<BurnEvent>(&bco.burn_events, BurnEvent { amount, from });
    }
}
