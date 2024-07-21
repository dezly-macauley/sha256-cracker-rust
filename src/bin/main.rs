use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process::exit;

use sha2::{Sha256, Digest};

fn main() {

    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Please enter one arguement");
        println!("E.g. Cargo run <sha256 hash>");
        exit(1);
    }

    let wanted_hash: &String = &args[1];

    let password_file: &str = "src/rockyou.txt";

    let mut attempts: i32 = 1;

    println!("Attempting to hack {}", wanted_hash);

    let password_list: File = File::open(password_file).unwrap();
    
    let reader = BufReader::new(password_list);

    for line in reader.lines() {

        let line: String = line.unwrap();
        let password = line.trim().to_owned().into_bytes();

        let password_hash: String = format!("{:x}", Sha256::digest(&password));

        println!("[{}] {} == {}", attempts, std::str::from_utf8(&password).unwrap(), password_hash);

        if & password_hash == wanted_hash {
            println!("Password hash found after {} attempts {} hashes to {}", attempts, std::str::from_utf8(&password).unwrap(), password_hash);
            exit(0);
        }
        attempts += 1;

    } 
    
    println!("Password hash not found.");
    

// ❯ cargo run -q --bin main 9e861941ad8bf5bcb649e5fde92d712528200a216018c2437371498e6ab7683d
// Attempting to hack 9e861941ad8bf5bcb649e5fde92d712528200a216018c2437371498e6ab7683d
// [1] 123456 == 8d969eef6ecad3c29a3a629280e686cf0c3f5d5a86aff3ca12020c923adc6c92
// [2] password == 5e884898da28047151d0e56f8dc6292773603d0d6aabbdd62a11ef721d1542d8
// [3] 123456789 == 15e2b0d3c33891ebb0f1ef609ec419420c20e320ce94c65fbc8c3312448eb225
// [4] 12345678 == ef797c8118f02dfb649607dd5d3f8c7623048c9c063d532cc95c5ed7a898a64f
// [5] 12345 == 5994471abb01112afcc18159f6cc74b4f511b99806da59b3caf5a9c173cacfc5
// [6] 111111 == bcb15f821479b4d5772bd0ca866c00ad5f926e3580720659cc80d39c9d09802a
// [7] 1234567 == 8bb0cf6eb9b17d0f7d22b456f121257dc1254e1f01665370476383ea776df414
// [8] sunshine == a941a4c4fd0c01cddef61b8be963bf4c1e2b0811c037ce3f1835fddf6ef6c223
// [9] qwerty == 65e84be33532fb784c48129675f9eff3a682b27168c0ea744b2cf58ee02337c5
// [10] iloveyou == e4ad93ca07acb8d908a3aa41e920ea4f4ef4f26e7f86cf8291c5db289780a5ae
// [11] princess == 04e77bf8f95cb3e1a36a59d1e93857c411930db646b46c218a0352e432023cf2
// [12] admin == 8c6976e5b5410415bde908bd4dee15dfb167a9c873fc4bb8a81f6f2ab448a918
// [13] welcome == 280d44ab1e9f79b5cce2dd4f58f5fe91f0fbacdac9f7447dffc318ceb79f2d02
// [14] 666666 == 94edf28c6d6da38fd35d7ad53e485307f89fbeaf120485c8d17a43f323deee71
// [15] abc123 == 6ca13d52ca70c883e0f0bb101e425a89e8624de51db2d2392593af6a84118090
// [16] football == 6382deaf1f5dc6e792b76db4a4a7bf2ba468884e000b25e7928e621e27fb23cb
// [17] 123123 == 96cae35ce8a9b0244178bf28e4966c2ce1b8385723a96a6b838858cdd6ca0a1e
// [18] monkey == 000c285457fc971f862a79b786476c78812c8897063c6fa9c045f579a3b2d63f
// [19] 654321 == 481f6cc0511143ccdd7e2d1b1b94faf0a700a8b49cd13922a70b5ae28acaa8c5
// [20] !@#$%^&* == abdac47091f60fe8bad1501c93b9fecb327bd89fd9d2a8d018e5d85a7771e4b8
// [21] charlie == b9dd960c1753459a78115d3cb845a57d924b6877e805b08bd01086ccdf34433c
// [22] aa123456 == 094dacfa4ae26448b7e7fdb6bf45b639ea9c9de2f942aa42310305f0657f9c61
// [23] donald == 4138cfbc5d36f31e8ae09ef4044bb88c0c9c6f289a6a1c27b335a99d1d8dc86f
// [24] password1 == 0b14d501a594442a01c6859541bcb3e8164d183d32937b851835442f69d5c94e
// [25] qwerty123 == daaad6e5604e8e17bd9f108d91e26afe6281dac8fda0091040a7a6d7bd9b43b5
// [26] letmein == 1c8bfe8f801d79745c4631d09fff36c82aa37fc4cce4fc946683d7b336b63032
// [27] zxcvbnm == 1df1854015e31ca286d015345eaff29a6c6073f70984a3a746823d4cac16b075
// [28] login == 428821350e9691491f616b754cd8315fb86d797ab35d843479e732ef90665324
// [29] starwars == 74fca0325b5fdb3a34badb40a2581cfbd5344187e8d3432952a5abc0929c1246
// [30] 121212 == 3ea87a56da3844b420ec2925ae922bc731ec16a4fc44dcbeafdad49b0e61d39c
// [31] bailey == 299d6631d639256a762b81ee007deb44cdd1cbc983e025038e113a0e709d3f7b
// [32] freedom == 13b1f7ec5beaefc781e43a3b344371cd49923a8a05edd71844b92f56f6a08d38
// [33] shadow == 0bb09d80600eec3eb9d7793a6f859bedde2a2d83899b70bd78e961ed674b32f4
// [34] passw0rd == 8f0e2f76e22b43e2855189877e7dc1e1e7d98c226c95db247cd1d547928334a9
// [35] master == fc613b4dfd6736a7bd268c8a0e74ed0d1c04a959f59dd74ef2874983fd443fc9
// [36] baseball == a01edad91c00abe7be5b72b5e36bf4ce3c6f26e8bce3340eba365642813ab8b6
// [37] buster == cbeaff314ef5ad032caa60ee2e8d8144ae52a8572c7d6f75631f3bd4080a7b16
// [38] Daniel == 7297db81c2f7916e25b9593f8c8785e1aa1487fa9f3961c50b7cc5f1a541bc82
// [39] Hannah == 79cb79153c08657ed66fe366e7e92fef8dcbc8be1a2e91f7cace938d61dc4b96
// [40] Thomas == 5dfcf9ef1fb1ecbce32fefe37ae99aff68832a7e2ac74f52daa5a1bcd8038118
// [41] summer == e83664255c6963e962bb20f9fcfaad1b570ddf5da69f5444ed37e5260f3ef689
// [42] George == 3d28271ec52e3d07fe14f5f16d01f2c09cbcac1949f9904b305136d0edbee12d
// [43] Harley == e3f1778453b7946b15eac18b00c44aeabf7c01638aafd50a93c942c48da645ab
// [44] 222222 == 4cc8f4d609b717356701c57a03e737e5ac8fe885da8c7163d3de47e01849c635
// [45] Jessica == fb338e53da57ec89c638f24e8f87697a70d0c0d8e4cc3a56f754211d2c6f0444
// [46] ginger == 08ddff4ebe39249a9208cd305b7d14091b1ebabef6adfa897cc34675fa0e0848
// [47] abcdef == bef57ec7f53a6d40beb640a780a639c83bc29ac8a9816f1fc6c5c6dcd93c4721
// [48] Jordan == e8bfe1ed693510570ced8b5ee70049cc4b985a77ec066ee345892f685d72cca6
// [49] 55555 == c507a68f3093e885765257ed3f176c757aaf62bb4cbc2ef94b2e7da3406d9676
// [50] Tigger == 49df5f6cae1d5191dfbf17a3696ef9818ee5bc9b11e5179afe21f0a23f9e385e
// [51] Joshua == cb14bf5073ebaf6d9d04b63164b7017b2011d3558fb2f80f9450c9f5de6f6de8
// [52] Pepper == d9538cf97df1f8d5643401d36fc8f15eaad2b490a20cd16ba94c18e8e75ac449
// [53] Robert == 2238dd61a1bf83816b40ad894518814b8edf7221d84d897ffd2c0466ace07c41
// [54] Matthew == edd2916124c93479ced1dd30f618d002478a35eeec25f633c33b9de974e201ad
// [55] 12341234 == 1718c24b10aeb8099e3fc44960ab6949ab76a267352459f203ea1036bec382c2
// [56] Andrew == c10873196eb1124ed74461c20a67094e395f2310f6305607b9694ee6b1ee8b43
// [57] lakers == be392b792809651cdec3485da4357568972ceb4cfd939c4bf5e5c3d011b04837
// [58] andrea == 5f3d6952c5c5e22077fabf461de80f1ce475752fe75afcf5ca46bac438405619
// [59] 1qaz2wsx == 059a00192592d5444bc0caad7203f98b506332e2cf7abb35d684ea9bf7c18f08
// [60] sophie == 5e0176c9d2070a5a2a22bf74b4abed303654690d58d64221ccbd022af827abc4
// [61] Ferrari == ff5aa449aa6ed54d533972ed385d2e3957e3e1b6ceacf55de80b520395d8dfe6
// [62] Cheese == 6e5a59d4fd56f798e609d28e3d6300b2d80bfbb76d27711c0c24006a129d4e17
// [63] Computer == 76ed42d22129dc354362704eb4b54208041b68736f976932aada43bc0035f7c0
// [64] jesus == a54e71f0e17f5aaf7946e66ab42cf3b1fd4e61d60581736c9f0eb1c3f794eb7c
// [65] Corvette == 65a88347a15db09ccabcd2d6c91f16351f93271624e7154c45b7b3cdeefad379
// [66] Mercedes == 3d0e65dfe82d2ab6ae3def37a3342a682f61a9be00bbbe052e4b0ae837ed8464
// [67] flower == c06b0cfe0cc5e900c57784484094331f095bf441995c3c31ea6c75691c786c35
// [68] Blahblah == cf3f07c18fa34fc82a23d82034ffb98cc28b1c6d58cd96f1158f88fd73bda98a
// [69] Maverick == 1aadcb76733000c9c85b52c63b732ef84b8038dab6fe81134ff9edbb021f5003
// [70] Hello == 185f8db32271fe25f561a6fc938b2e264306ec304eda518007d1764826381969
// [71] loveme == 78fe3f05768ff3a95c74ffafe366cc3474022d925ad5593af733bf8ac1ab0de6
// [72] nicole == b54a1af8b666f61c2dd5ae8f8a543133409fd28c3b78064c5db993bf2c8e77bc
// [73] hunter == e9a63a4eb15738ae85cd416221c8fcc4ccc0018fac91335b42eaa016c76e87f9
// [74] amanda == 52e8e47b38e854580afce4aade15dbd5ce0c0464da711afe71da123687d5a4cd
// [75] jennifer == 9ce8db922a8f4a7abd859adee70bd8b7a63321265487da54cf4bed6a69eb3e1b
// [76] banana == b493d48364afe44d11c0165cf470a4164d1e2609911ef998be868d46ade3de4e
// [77] chelsea == 1ecd41c03ef78bd6daeaa6bb008896607a8413bf8ba6266be80327554b370a9e
// [78] ranger == dbc4a04327176e6577b4da46df04564150053960eba5d89587dad1f76a818d80
// [79] trustno1 == 203b70b5ae883932161bbd0bded9357e763e63afce98b16230be33f0b94c2cc5
// [80] merlin == f6274d9892026fe47dd5f96f708ef8983dccc7bacf5ee4a90b2400805adaea0a
// [81] cookie == d7e83e28a04b537e64424546b14caf9b67bad2f28dabce68116e0d372319fa00
// [82] ashley == c64975ba3cf3f9cd58459710b0a42369f34b0759c9967fb5a47eea488e8bea79
// [83] bandit == 07299a3f8843d2cde6e11075c597b2ce9868221d2a401aba32abf05bd455af23
// [84] killer == ed45d626b07112a8a501d9672f3b92796a6754b8d8d9cb4c617fec9774889220
// [85] aaaaaa == ed02457b5c41d964dbd2f2a609d63fe1bb7528dbe55e1abf5b52c249cd735797
// [86] 1q2w3e == c0c4a69b17a7955ac230bfc8db4a123eaa956ccf3c0022e68b8d4e2f5b699d1f
// [87] zaq1zaq1 == 5850d367eed947f51b618b0b7576ba0eb8ae6810b5f3d2ec2b39865f0aa93a95
// [88] mustang == a92f6bdb75789bccc118adfcf704029aa58063c604bab4fcdd9cd126ef9b69af
// [89] test == 9f86d081884c7d659a2feaa0c55ad015a3bf4f1b2b0b822cd15d6c15b0f00a08
// [90] hockey == 308738b8195da46d65c96f4ee3909032e27c818d8a079bccb5a1ef62e8daaa45
// [91] dallas == fa2115f8d576a6ab722956697fc759c31d1cd6b93c8336bfebf73ed5cba2ff49
// [92] whatever == 85738f8f9a7f1b04b5329c590ebcb9e425925c6d0984089c43a022de4f19c281
// [93] admin123 == 240be518fabd2724ddb6f04eeb1da5967448d7e831c08c8fa822809f74c720a9
// [94] michael == 34550715062af006ac4fab288de67ecb44793c3a05c475227241535f6ef7a81b
// [95] liverpool == 9e861941ad8bf5bcb649e5fde92d712528200a216018c2437371498e6ab7683d
// Password hash found after 95 attempts liverpool hashes to 9e861941ad8bf5bcb649e5fde92d712528200a216018c2437371498e6
// ab7683d

sha256-cracker-rust on  main [!] is 󰏗 v0.1.0 via  v1.79.0 took 2s 


}
