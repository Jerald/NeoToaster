macro_rules! groups {
    ( $(mod $mod:ident $(as $alias:ident)? ;) + ) => {
        $(mod $mod;)+
        
        mod __groups {
            const GROUPS_LEN: usize = 0 $( + { let _ = stringify!($mod); 1 } )+ ;
            pub static GROUP_LIST: [&::serenity::framework::standard::CommandGroup; GROUPS_LEN] = [
                $({
                    #[allow(unused_imports)]
                    use super::$mod::*;
                    groups!(@name $mod $(, $alias)?)
                }),+
            ];
        }

        pub use __groups::GROUP_LIST;
    };

    (@name $mod:ident) => {
        &::serenity_group_name::group_name!($mod)
    };

    (@name $mod:ident, $alias:ident) => {
        &::serenity_group_name::group_name!($alias)
    };
}

groups! {
    mod general;
    mod yolol;
    mod yolol_wars as yololwars;
}