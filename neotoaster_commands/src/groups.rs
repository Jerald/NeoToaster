macro_rules! groups {
    ( $(mod $m:ident;) + ) => {
        $(mod $m;)+
        
        mod __groups {
            const GROUPS_LEN: usize = 0 $( + { let _ = stringify!($m); 1 } )+ ;
            pub static GROUP_LIST: [&::serenity::framework::standard::CommandGroup; GROUPS_LEN] = [
                $({ use super::$m::*; &::serenity_group_name::group_name!($m) }),+
            ];
        }

        pub use __groups::GROUP_LIST;
    }
}

groups! {
    mod general;
    mod yolol;
}