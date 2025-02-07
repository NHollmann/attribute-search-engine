use std::str::FromStr;

use attribute_search_engine::*;

#[derive(PartialEq, Eq, Hash)]
enum ServerOs {
    Debian,
    Alpine,
    Router,
    Win,
}

impl FromStr for ServerOs {
    type Err = ();

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "Debian" => Ok(Self::Debian),
            "Alpine" => Ok(Self::Alpine),
            "Router" => Ok(Self::Router),
            "Win" => Ok(Self::Win),
            _ => Err(()),
        }
    }
}

fn create_network_search_engine() -> SearchEngine<u8> {
    let mut index_name = SearchIndexHashMap::<_, String>::new();
    let mut index_os = SearchIndexHashMap::<_, ServerOs>::new();
    let mut index_ip4 = SearchIndexPrefixTree::<_>::new();
    let mut index_uptime = SearchIndexBTreeRange::<_, u64>::new();
    let mut index_user = SearchIndexHashMap::<_, String>::new();

    #[cfg_attr(rustfmt, rustfmt_skip)]
    let systems = vec![
        ( 0,"gateway",        ServerOs::Router, "192.168.0.1",  4323, vec!["root"]),
        ( 1,"firewall-01",    ServerOs::Debian, "192.168.0.11", 1133, vec!["root"]),
        ( 2,"firewall-02",    ServerOs::Debian, "192.168.0.12", 4567, vec!["root"]),
        ( 3,"router-dmz",     ServerOs::Router, "192.168.10.1", 2134, vec!["root"]),
        ( 4,"router-intern",  ServerOs::Router, "192.168.20.1", 4534, vec!["root"]),
        ( 5,"router-guests",  ServerOs::Router, "192.168.30.1", 1313, vec!["root"]),
        ( 6,"web-01",         ServerOs::Alpine, "192.168.10.2", 2345, vec!["root", "webmaster", "alex"]),
        ( 7,"web-02",         ServerOs::Alpine, "192.168.10.3", 8543, vec!["root", "webmaster", "alex", "peter"]),
        ( 8,"web-03",         ServerOs::Alpine, "192.168.10.4", 2355, vec!["root", "webmaster", "peter"]),
        ( 9,"mail",           ServerOs::Debian, "192.168.10.5", 2243, vec!["root", "postmaster", "alex", "peter", "hans"]),
        (10,"backups",        ServerOs::Debian, "192.168.20.3", 5322, vec!["root", "backup"]),
        (11,"fileshare",      ServerOs::Debian, "192.168.20.4", 9132, vec!["root", "alex", "peter", "hans"]),
        (12,"hotspot-users",  ServerOs::Router, "192.168.20.2", 1346, vec!["root"]),
        (13,"hotspot-guests", ServerOs::Router, "192.168.30.2", 4232, vec!["root"]),
        (14,"workstation-01", ServerOs::Win,    "192.168.20.5",  134, vec!["root", "alex"]),
        (15,"workstation-02", ServerOs::Win,    "192.168.20.6",   15, vec!["root", "peter"]),
        (16,"workstation-03", ServerOs::Win,    "192.168.20.7",  112, vec!["root", "hans"]),
    ];

    for (id, name, os, ip, uptime, users) in systems {
        index_name.insert(id, name.into());
        index_os.insert(id, os);
        index_ip4.insert(id, ip.into());
        index_uptime.insert(id, uptime);
        for user in users {
            index_user.insert(id, user.into());
        }
    }

    let mut engine = SearchEngine::new();
    engine.add_index("name", index_name);
    engine.add_index("os", index_os);
    engine.add_index("ip4", index_ip4);
    engine.add_index("uptime", index_uptime);
    engine.add_index("user", index_user);

    engine
}

mod query_string {
    use super::*;
    use std::collections::HashSet;

    macro_rules! query_test {
        ($name:ident $query:literal; $($res:expr),* $(,)?) => {
            #[test]
            fn $name() {
                let engine = create_network_search_engine();
                let (q, _) = engine.query_from_str($query).expect("valid query");
                assert_eq!(engine.search(&q), Ok(HashSet::from_iter(vec![$($res),*])));
            }
        };
    }

    // Empty query
    query_test! {empty "";}

    // Name queries
    query_test! {name_web_01 "+name:web-01"; 6}
    query_test! {name_web_01_02_03 "+name:web-01,web-02,web-03"; 6, 7, 8}

    // OS queries
    query_test! {os_router "+os:Router"; 0, 3, 4, 5, 12, 13}
    query_test! {os_debian_alpine "+os:Debian,Alpine"; 1, 2, 6, 7, 8, 9, 10, 11}

    // IP queries
    query_test! {ip4_single_match "+ip4:192.168.10.1"; 3}
    query_test! {ip4_multi_match "+ip4:192.168.0.1"; 0, 1, 2}
    query_test! {ip4_exact_match "+ip4:=192.168.0.1"; 0}
    query_test! {ip4_dmz "+ip4:192.168.10."; 3, 6, 7, 8, 9}
    query_test! {ip4_exact_dmz "+ip4:=192.168.10.";}

    // Uptime queries
    query_test! {uptime_eq_1133 "+uptime:1133"; 1}
    query_test! {uptime_eq_2135_15 "+uptime:=2134,=15"; 3, 15}
    query_test! {uptime_lt_1000 "+uptime:<1000"; 14, 15, 16}
    query_test! {uptime_gt_8000 "+uptime:>8000"; 7, 11}
    query_test! {uptime_3000_4000 "+uptime:4000-5000"; 0, 2, 4, 13}

    // User queries
    query_test! {user_root "+user:root"; 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16}
    query_test! {user_alex "+user:alex"; 6, 7, 9, 11, 14}
    query_test! {user_alex_peter "+user:alex,peter"; 6, 7, 8, 9, 11, 14, 15}
    query_test! {user_alex_peter_not_hans "+user:alex,peter -user:hans"; 6, 7, 8, 14, 15}

    // Complex queries
    query_test! {complex_dmz_not_alpine "+ip4:192.168.10. -os:Alpine"; 3, 9}
    query_test! {complex_alex_not_win "+user:alex -os:Win"; 6, 7, 9, 11}
    query_test! {complex_alex_intern_not_win "+user:alex +ip4:192.168.20. -os:Win"; 11}
}
