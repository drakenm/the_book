mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

mod back_of_house {
    mod chef {
        fn taste_test() {}

        fn serve_order() {}
    }

    mod cook {
        fn prepare_order() {}

        fn cook_order() {}
    }

    mod busser {
        fn clean_table() {}

        fn set_table() {}

        fn clean_dishes() {}

        fn curb_trash() {}
    }
}