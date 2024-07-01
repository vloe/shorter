use phf::{phf_map, Map};

// degenerate but works for now
pub(crate) static TLDS: Map<&'static str, &'static str> = phf_map! {
    "aaa" => "aaa",
    "aarp" => "aarp",
    "abb" => "abb",
    "abbott" => "abbott",
    "abbvie" => "abbvie",
    "abc" => "abc",
    "able" => "able",
    "abogado" => "abogado",
    "abudhabi" => "abudhabi",
    "ac" => "ac",
    "academy" => "academy",
    "accenture" => "accenture",
    "accountant" => "accountant",
    "accountants" => "accountants",
    "aco" => "aco",
    "actor" => "actor",
    "ad" => "ad",
    "ads" => "ads",
    "adult" => "adult",
    "ae" => "ae",
    "aeg" => "aeg",
    "aero" => "aero",
    "aetna" => "aetna",
    "af" => "af",
    "afl" => "afl",
    "africa" => "africa",
    "ag" => "ag",
    "agakhan" => "agakhan",
    "agency" => "agency",
    "ai" => "ai",
    "aig" => "aig",
    "airbus" => "airbus",
    "airforce" => "airforce",
    "airtel" => "airtel",
    "akdn" => "akdn",
    "al" => "al",
    "alibaba" => "alibaba",
    "alipay" => "alipay",
    "allfinanz" => "allfinanz",
    "allstate" => "allstate",
    "ally" => "ally",
    "alsace" => "alsace",
    "alstom" => "alstom",
    "am" => "am",
    "amazon" => "amazon",
    "americanexpress" => "americanexpress",
    "americanfamily" => "americanfamily",
    "amex" => "amex",
    "amfam" => "amfam",
    "amica" => "amica",
    "amsterdam" => "amsterdam",
    "analytics" => "analytics",
    "android" => "android",
    "anquan" => "anquan",
    "anz" => "anz",
    "ao" => "ao",
    "aol" => "aol",
    "apartments" => "apartments",
    "app" => "app",
    "apple" => "apple",
    "aq" => "aq",
    "aquarelle" => "aquarelle",
    "ar" => "ar",
    "arab" => "arab",
    "aramco" => "aramco",
    "archi" => "archi",
    "army" => "army",
    "arpa" => "arpa",
    "art" => "art",
    "arte" => "arte",
    "as" => "as",
    "asda" => "asda",
    "asia" => "asia",
    "associates" => "associates",
    "at" => "at",
    "athleta" => "athleta",
    "attorney" => "attorney",
    "au" => "au",
    "auction" => "auction",
    "audi" => "audi",
    "audible" => "audible",
    "audio" => "audio",
    "auspost" => "auspost",
    "author" => "author",
    "auto" => "auto",
    "autos" => "autos",
    "aw" => "aw",
    "aws" => "aws",
    "ax" => "ax",
    "axa" => "axa",
    "az" => "az",
    "azure" => "azure",
    "ba" => "ba",
    "baby" => "baby",
    "baidu" => "baidu",
    "banamex" => "banamex",
    "band" => "band",
    "bank" => "bank",
    "bar" => "bar",
    "barcelona" => "barcelona",
    "barclaycard" => "barclaycard",
    "barclays" => "barclays",
    "barefoot" => "barefoot",
    "bargains" => "bargains",
    "baseball" => "baseball",
    "basketball" => "basketball",
    "bauhaus" => "bauhaus",
    "bayern" => "bayern",
    "bb" => "bb",
    "bbc" => "bbc",
    "bbt" => "bbt",
    "bbva" => "bbva",
    "bcg" => "bcg",
    "bcn" => "bcn",
    "bd" => "bd",
    "be" => "be",
    "beats" => "beats",
    "beauty" => "beauty",
    "beer" => "beer",
    "bentley" => "bentley",
    "berlin" => "berlin",
    "best" => "best",
    "bestbuy" => "bestbuy",
    "bet" => "bet",
    "bf" => "bf",
    "bg" => "bg",
    "bh" => "bh",
    "bharti" => "bharti",
    "bi" => "bi",
    "bible" => "bible",
    "bid" => "bid",
    "bike" => "bike",
    "bing" => "bing",
    "bingo" => "bingo",
    "bio" => "bio",
    "biz" => "biz",
    "bj" => "bj",
    "black" => "black",
    "blackfriday" => "blackfriday",
    "blockbuster" => "blockbuster",
    "blog" => "blog",
    "bloomberg" => "bloomberg",
    "blue" => "blue",
    "bm" => "bm",
    "bms" => "bms",
    "bmw" => "bmw",
    "bn" => "bn",
    "bnpparibas" => "bnpparibas",
    "bo" => "bo",
    "boats" => "boats",
    "boehringer" => "boehringer",
    "bofa" => "bofa",
    "bom" => "bom",
    "bond" => "bond",
    "boo" => "boo",
    "book" => "book",
    "booking" => "booking",
    "bosch" => "bosch",
    "bostik" => "bostik",
    "boston" => "boston",
    "bot" => "bot",
    "boutique" => "boutique",
    "box" => "box",
    "br" => "br",
    "bradesco" => "bradesco",
    "bridgestone" => "bridgestone",
    "broadway" => "broadway",
    "broker" => "broker",
    "brother" => "brother",
    "brussels" => "brussels",
    "bs" => "bs",
    "bt" => "bt",
    "build" => "build",
    "builders" => "builders",
    "business" => "business",
    "buy" => "buy",
    "buzz" => "buzz",
    "bv" => "bv",
    "bw" => "bw",
    "by" => "by",
    "bz" => "bz",
    "bzh" => "bzh",
    "ca" => "ca",
    "cab" => "cab",
    "cafe" => "cafe",
    "cal" => "cal",
    "call" => "call",
    "calvinklein" => "calvinklein",
    "cam" => "cam",
    "camera" => "camera",
    "camp" => "camp",
    "canon" => "canon",
    "capetown" => "capetown",
    "capital" => "capital",
    "capitalone" => "capitalone",
    "car" => "car",
    "caravan" => "caravan",
    "cards" => "cards",
    "care" => "care",
    "career" => "career",
    "careers" => "careers",
    "cars" => "cars",
    "casa" => "casa",
    "case" => "case",
    "cash" => "cash",
    "casino" => "casino",
    "cat" => "cat",
    "catering" => "catering",
    "catholic" => "catholic",
    "cba" => "cba",
    "cbn" => "cbn",
    "cbre" => "cbre",
    "cc" => "cc",
    "cd" => "cd",
    "center" => "center",
    "ceo" => "ceo",
    "cern" => "cern",
    "cf" => "cf",
    "cfa" => "cfa",
    "cfd" => "cfd",
    "cg" => "cg",
    "ch" => "ch",
    "chanel" => "chanel",
    "channel" => "channel",
    "charity" => "charity",
    "chase" => "chase",
    "chat" => "chat",
    "cheap" => "cheap",
    "chintai" => "chintai",
    "christmas" => "christmas",
    "chrome" => "chrome",
    "church" => "church",
    "ci" => "ci",
    "cipriani" => "cipriani",
    "circle" => "circle",
    "cisco" => "cisco",
    "citadel" => "citadel",
    "citi" => "citi",
    "citic" => "citic",
    "city" => "city",
    "ck" => "ck",
    "cl" => "cl",
    "claims" => "claims",
    "cleaning" => "cleaning",
    "click" => "click",
    "clinic" => "clinic",
    "clinique" => "clinique",
    "clothing" => "clothing",
    "cloud" => "cloud",
    "club" => "club",
    "clubmed" => "clubmed",
    "cm" => "cm",
    "cn" => "cn",
    "co" => "co",
    "coach" => "coach",
    "codes" => "codes",
    "coffee" => "coffee",
    "college" => "college",
    "cologne" => "cologne",
    "com" => "com",
    "commbank" => "commbank",
    "community" => "community",
    "company" => "company",
    "compare" => "compare",
    "computer" => "computer",
    "comsec" => "comsec",
    "condos" => "condos",
    "construction" => "construction",
    "consulting" => "consulting",
    "contact" => "contact",
    "contractors" => "contractors",
    "cooking" => "cooking",
    "cool" => "cool",
    "coop" => "coop",
    "corsica" => "corsica",
    "country" => "country",
    "coupon" => "coupon",
    "coupons" => "coupons",
    "courses" => "courses",
    "cpa" => "cpa",
    "cr" => "cr",
    "credit" => "credit",
    "creditcard" => "creditcard",
    "creditunion" => "creditunion",
    "cricket" => "cricket",
    "crown" => "crown",
    "crs" => "crs",
    "cruise" => "cruise",
    "cruises" => "cruises",
    "cu" => "cu",
    "cuisinella" => "cuisinella",
    "cv" => "cv",
    "cw" => "cw",
    "cx" => "cx",
    "cy" => "cy",
    "cymru" => "cymru",
    "cyou" => "cyou",
    "cz" => "cz",
    "dabur" => "dabur",
    "dad" => "dad",
    "dance" => "dance",
    "data" => "data",
    "date" => "date",
    "dating" => "dating",
    "datsun" => "datsun",
    "day" => "day",
    "dclk" => "dclk",
    "dds" => "dds",
    "de" => "de",
    "deal" => "deal",
    "dealer" => "dealer",
    "deals" => "deals",
    "degree" => "degree",
    "delivery" => "delivery",
    "dell" => "dell",
    "deloitte" => "deloitte",
    "delta" => "delta",
    "democrat" => "democrat",
    "dental" => "dental",
    "dentist" => "dentist",
    "desi" => "desi",
    "design" => "design",
    "dev" => "dev",
    "dhl" => "dhl",
    "diamonds" => "diamonds",
    "diet" => "diet",
    "digital" => "digital",
    "direct" => "direct",
    "directory" => "directory",
    "discount" => "discount",
    "discover" => "discover",
    "dish" => "dish",
    "diy" => "diy",
    "dj" => "dj",
    "dk" => "dk",
    "dm" => "dm",
    "dnp" => "dnp",
    "do" => "do",
    "docs" => "docs",
    "doctor" => "doctor",
    "dog" => "dog",
    "domains" => "domains",
    "dot" => "dot",
    "download" => "download",
    "drive" => "drive",
    "dtv" => "dtv",
    "dubai" => "dubai",
    "dunlop" => "dunlop",
    "dupont" => "dupont",
    "durban" => "durban",
    "dvag" => "dvag",
    "dvr" => "dvr",
    "dz" => "dz",
    "earth" => "earth",
    "eat" => "eat",
    "ec" => "ec",
    "eco" => "eco",
    "edeka" => "edeka",
    "edu" => "edu",
    "education" => "education",
    "ee" => "ee",
    "eg" => "eg",
    "email" => "email",
    "emerck" => "emerck",
    "energy" => "energy",
    "engineer" => "engineer",
    "engineering" => "engineering",
    "enterprises" => "enterprises",
    "epson" => "epson",
    "equipment" => "equipment",
    "er" => "er",
    "ericsson" => "ericsson",
    "erni" => "erni",
    "es" => "es",
    "esq" => "esq",
    "estate" => "estate",
    "et" => "et",
    "eu" => "eu",
    "eurovision" => "eurovision",
    "eus" => "eus",
    "events" => "events",
    "exchange" => "exchange",
    "expert" => "expert",
    "exposed" => "exposed",
    "express" => "express",
    "extraspace" => "extraspace",
    "fage" => "fage",
    "fail" => "fail",
    "fairwinds" => "fairwinds",
    "faith" => "faith",
    "family" => "family",
    "fan" => "fan",
    "fans" => "fans",
    "farm" => "farm",
    "farmers" => "farmers",
    "fashion" => "fashion",
    "fast" => "fast",
    "fedex" => "fedex",
    "feedback" => "feedback",
    "ferrari" => "ferrari",
    "ferrero" => "ferrero",
    "fi" => "fi",
    "fidelity" => "fidelity",
    "fido" => "fido",
    "film" => "film",
    "final" => "final",
    "finance" => "finance",
    "financial" => "financial",
    "fire" => "fire",
    "firestone" => "firestone",
    "firmdale" => "firmdale",
    "fish" => "fish",
    "fishing" => "fishing",
    "fit" => "fit",
    "fitness" => "fitness",
    "fj" => "fj",
    "fk" => "fk",
    "flickr" => "flickr",
    "flights" => "flights",
    "flir" => "flir",
    "florist" => "florist",
    "flowers" => "flowers",
    "fly" => "fly",
    "fm" => "fm",
    "fo" => "fo",
    "foo" => "foo",
    "food" => "food",
    "football" => "football",
    "ford" => "ford",
    "forex" => "forex",
    "forsale" => "forsale",
    "forum" => "forum",
    "foundation" => "foundation",
    "fox" => "fox",
    "fr" => "fr",
    "free" => "free",
    "fresenius" => "fresenius",
    "frl" => "frl",
    "frogans" => "frogans",
    "frontier" => "frontier",
    "ftr" => "ftr",
    "fujitsu" => "fujitsu",
    "fun" => "fun",
    "fund" => "fund",
    "furniture" => "furniture",
    "futbol" => "futbol",
    "fyi" => "fyi",
    "ga" => "ga",
    "gal" => "gal",
    "gallery" => "gallery",
    "gallo" => "gallo",
    "gallup" => "gallup",
    "game" => "game",
    "games" => "games",
    "gap" => "gap",
    "garden" => "garden",
    "gay" => "gay",
    "gb" => "gb",
    "gbiz" => "gbiz",
    "gd" => "gd",
    "gdn" => "gdn",
    "ge" => "ge",
    "gea" => "gea",
    "gent" => "gent",
    "genting" => "genting",
    "george" => "george",
    "gf" => "gf",
    "gg" => "gg",
    "ggee" => "ggee",
    "gh" => "gh",
    "gi" => "gi",
    "gift" => "gift",
    "gifts" => "gifts",
    "gives" => "gives",
    "giving" => "giving",
    "gl" => "gl",
    "glass" => "glass",
    "gle" => "gle",
    "global" => "global",
    "globo" => "globo",
    "gm" => "gm",
    "gmail" => "gmail",
    "gmbh" => "gmbh",
    "gmo" => "gmo",
    "gmx" => "gmx",
    "gn" => "gn",
    "godaddy" => "godaddy",
    "gold" => "gold",
    "goldpoint" => "goldpoint",
    "golf" => "golf",
    "goo" => "goo",
    "goodyear" => "goodyear",
    "goog" => "goog",
    "google" => "google",
    "gop" => "gop",
    "got" => "got",
    "gov" => "gov",
    "gp" => "gp",
    "gq" => "gq",
    "gr" => "gr",
    "grainger" => "grainger",
    "graphics" => "graphics",
    "gratis" => "gratis",
    "green" => "green",
    "gripe" => "gripe",
    "grocery" => "grocery",
    "group" => "group",
    "gs" => "gs",
    "gt" => "gt",
    "gu" => "gu",
    "gucci" => "gucci",
    "guge" => "guge",
    "guide" => "guide",
    "guitars" => "guitars",
    "guru" => "guru",
    "gw" => "gw",
    "gy" => "gy",
    "hair" => "hair",
    "hamburg" => "hamburg",
    "hangout" => "hangout",
    "haus" => "haus",
    "hbo" => "hbo",
    "hdfc" => "hdfc",
    "hdfcbank" => "hdfcbank",
    "health" => "health",
    "healthcare" => "healthcare",
    "help" => "help",
    "helsinki" => "helsinki",
    "here" => "here",
    "hermes" => "hermes",
    "hiphop" => "hiphop",
    "hisamitsu" => "hisamitsu",
    "hitachi" => "hitachi",
    "hiv" => "hiv",
    "hk" => "hk",
    "hkt" => "hkt",
    "hm" => "hm",
    "hn" => "hn",
    "hockey" => "hockey",
    "holdings" => "holdings",
    "holiday" => "holiday",
    "homedepot" => "homedepot",
    "homegoods" => "homegoods",
    "homes" => "homes",
    "homesense" => "homesense",
    "honda" => "honda",
    "horse" => "horse",
    "hospital" => "hospital",
    "host" => "host",
    "hosting" => "hosting",
    "hot" => "hot",
    "hotels" => "hotels",
    "hotmail" => "hotmail",
    "house" => "house",
    "how" => "how",
    "hr" => "hr",
    "hsbc" => "hsbc",
    "ht" => "ht",
    "hu" => "hu",
    "hughes" => "hughes",
    "hyatt" => "hyatt",
    "hyundai" => "hyundai",
    "ibm" => "ibm",
    "icbc" => "icbc",
    "ice" => "ice",
    "icu" => "icu",
    "id" => "id",
    "ie" => "ie",
    "ieee" => "ieee",
    "ifm" => "ifm",
    "ikano" => "ikano",
    "il" => "il",
    "im" => "im",
    "imamat" => "imamat",
    "imdb" => "imdb",
    "immo" => "immo",
    "immobilien" => "immobilien",
    "in" => "in",
    "inc" => "inc",
    "industries" => "industries",
    "infiniti" => "infiniti",
    "info" => "info",
    "ing" => "ing",
    "ink" => "ink",
    "institute" => "institute",
    "insurance" => "insurance",
    "insure" => "insure",
    "int" => "int",
    "international" => "international",
    "intuit" => "intuit",
    "investments" => "investments",
    "io" => "io",
    "ipiranga" => "ipiranga",
    "iq" => "iq",
    "ir" => "ir",
    "irish" => "irish",
    "is" => "is",
    "ismaili" => "ismaili",
    "ist" => "ist",
    "istanbul" => "istanbul",
    "it" => "it",
    "itau" => "itau",
    "itv" => "itv",
    "jaguar" => "jaguar",
    "java" => "java",
    "jcb" => "jcb",
    "je" => "je",
    "jeep" => "jeep",
    "jetzt" => "jetzt",
    "jewelry" => "jewelry",
    "jio" => "jio",
    "jll" => "jll",
    "jm" => "jm",
    "jmp" => "jmp",
    "jnj" => "jnj",
    "jo" => "jo",
    "jobs" => "jobs",
    "joburg" => "joburg",
    "jot" => "jot",
    "joy" => "joy",
    "jp" => "jp",
    "jpmorgan" => "jpmorgan",
    "jprs" => "jprs",
    "juegos" => "juegos",
    "juniper" => "juniper",
    "kaufen" => "kaufen",
    "kddi" => "kddi",
    "ke" => "ke",
    "kerryhotels" => "kerryhotels",
    "kerrylogistics" => "kerrylogistics",
    "kerryproperties" => "kerryproperties",
    "kfh" => "kfh",
    "kg" => "kg",
    "kh" => "kh",
    "ki" => "ki",
    "kia" => "kia",
    "kids" => "kids",
    "kim" => "kim",
    "kindle" => "kindle",
    "kitchen" => "kitchen",
    "kiwi" => "kiwi",
    "km" => "km",
    "kn" => "kn",
    "koeln" => "koeln",
    "komatsu" => "komatsu",
    "kosher" => "kosher",
    "kp" => "kp",
    "kpmg" => "kpmg",
    "kpn" => "kpn",
    "kr" => "kr",
    "krd" => "krd",
    "kred" => "kred",
    "kuokgroup" => "kuokgroup",
    "kw" => "kw",
    "ky" => "ky",
    "kyoto" => "kyoto",
    "kz" => "kz",
    "la" => "la",
    "lacaixa" => "lacaixa",
    "lamborghini" => "lamborghini",
    "lamer" => "lamer",
    "lancaster" => "lancaster",
    "land" => "land",
    "landrover" => "landrover",
    "lanxess" => "lanxess",
    "lasalle" => "lasalle",
    "lat" => "lat",
    "latino" => "latino",
    "latrobe" => "latrobe",
    "law" => "law",
    "lawyer" => "lawyer",
    "lb" => "lb",
    "lc" => "lc",
    "lds" => "lds",
    "lease" => "lease",
    "leclerc" => "leclerc",
    "lefrak" => "lefrak",
    "legal" => "legal",
    "lego" => "lego",
    "lexus" => "lexus",
    "lgbt" => "lgbt",
    "li" => "li",
    "lidl" => "lidl",
    "life" => "life",
    "lifeinsurance" => "lifeinsurance",
    "lifestyle" => "lifestyle",
    "lighting" => "lighting",
    "like" => "like",
    "lilly" => "lilly",
    "limited" => "limited",
    "limo" => "limo",
    "lincoln" => "lincoln",
    "link" => "link",
    "lipsy" => "lipsy",
    "live" => "live",
    "living" => "living",
    "lk" => "lk",
    "llc" => "llc",
    "llp" => "llp",
    "loan" => "loan",
    "loans" => "loans",
    "locker" => "locker",
    "locus" => "locus",
    "lol" => "lol",
    "london" => "london",
    "lotte" => "lotte",
    "lotto" => "lotto",
    "love" => "love",
    "lpl" => "lpl",
    "lplfinancial" => "lplfinancial",
    "lr" => "lr",
    "ls" => "ls",
    "lt" => "lt",
    "ltd" => "ltd",
    "ltda" => "ltda",
    "lu" => "lu",
    "lundbeck" => "lundbeck",
    "luxe" => "luxe",
    "luxury" => "luxury",
    "lv" => "lv",
    "ly" => "ly",
    "ma" => "ma",
    "madrid" => "madrid",
    "maif" => "maif",
    "maison" => "maison",
    "makeup" => "makeup",
    "man" => "man",
    "management" => "management",
    "mango" => "mango",
    "map" => "map",
    "market" => "market",
    "marketing" => "marketing",
    "markets" => "markets",
    "marriott" => "marriott",
    "marshalls" => "marshalls",
    "mattel" => "mattel",
    "mba" => "mba",
    "mc" => "mc",
    "mckinsey" => "mckinsey",
    "md" => "md",
    "me" => "me",
    "med" => "med",
    "media" => "media",
    "meet" => "meet",
    "melbourne" => "melbourne",
    "meme" => "meme",
    "memorial" => "memorial",
    "men" => "men",
    "menu" => "menu",
    "merckmsd" => "merckmsd",
    "mg" => "mg",
    "mh" => "mh",
    "miami" => "miami",
    "microsoft" => "microsoft",
    "mil" => "mil",
    "mini" => "mini",
    "mint" => "mint",
    "mit" => "mit",
    "mitsubishi" => "mitsubishi",
    "mk" => "mk",
    "ml" => "ml",
    "mlb" => "mlb",
    "mls" => "mls",
    "mm" => "mm",
    "mma" => "mma",
    "mn" => "mn",
    "mo" => "mo",
    "mobi" => "mobi",
    "mobile" => "mobile",
    "moda" => "moda",
    "moe" => "moe",
    "moi" => "moi",
    "mom" => "mom",
    "monash" => "monash",
    "money" => "money",
    "monster" => "monster",
    "mormon" => "mormon",
    "mortgage" => "mortgage",
    "moscow" => "moscow",
    "moto" => "moto",
    "motorcycles" => "motorcycles",
    "mov" => "mov",
    "movie" => "movie",
    "mp" => "mp",
    "mq" => "mq",
    "mr" => "mr",
    "ms" => "ms",
    "msd" => "msd",
    "mt" => "mt",
    "mtn" => "mtn",
    "mtr" => "mtr",
    "mu" => "mu",
    "museum" => "museum",
    "music" => "music",
    "mv" => "mv",
    "mw" => "mw",
    "mx" => "mx",
    "my" => "my",
    "mz" => "mz",
    "na" => "na",
    "nab" => "nab",
    "nagoya" => "nagoya",
    "name" => "name",
    "navy" => "navy",
    "nba" => "nba",
    "nc" => "nc",
    "ne" => "ne",
    "nec" => "nec",
    "net" => "net",
    "netbank" => "netbank",
    "netflix" => "netflix",
    "network" => "network",
    "neustar" => "neustar",
    "new" => "new",
    "news" => "news",
    "next" => "next",
    "nextdirect" => "nextdirect",
    "nexus" => "nexus",
    "nf" => "nf",
    "nfl" => "nfl",
    "ng" => "ng",
    "ngo" => "ngo",
    "nhk" => "nhk",
    "ni" => "ni",
    "nico" => "nico",
    "nike" => "nike",
    "nikon" => "nikon",
    "ninja" => "ninja",
    "nissan" => "nissan",
    "nissay" => "nissay",
    "nl" => "nl",
    "no" => "no",
    "nokia" => "nokia",
    "norton" => "norton",
    "now" => "now",
    "nowruz" => "nowruz",
    "nowtv" => "nowtv",
    "np" => "np",
    "nr" => "nr",
    "nra" => "nra",
    "nrw" => "nrw",
    "ntt" => "ntt",
    "nu" => "nu",
    "nyc" => "nyc",
    "nz" => "nz",
    "obi" => "obi",
    "observer" => "observer",
    "office" => "office",
    "okinawa" => "okinawa",
    "olayan" => "olayan",
    "olayangroup" => "olayangroup",
    "ollo" => "ollo",
    "om" => "om",
    "omega" => "omega",
    "one" => "one",
    "ong" => "ong",
    "onl" => "onl",
    "online" => "online",
    "ooo" => "ooo",
    "open" => "open",
    "oracle" => "oracle",
    "orange" => "orange",
    "org" => "org",
    "organic" => "organic",
    "origins" => "origins",
    "osaka" => "osaka",
    "otsuka" => "otsuka",
    "ott" => "ott",
    "ovh" => "ovh",
    "pa" => "pa",
    "page" => "page",
    "panasonic" => "panasonic",
    "paris" => "paris",
    "pars" => "pars",
    "partners" => "partners",
    "parts" => "parts",
    "party" => "party",
    "pay" => "pay",
    "pccw" => "pccw",
    "pe" => "pe",
    "pet" => "pet",
    "pf" => "pf",
    "pfizer" => "pfizer",
    "pg" => "pg",
    "ph" => "ph",
    "pharmacy" => "pharmacy",
    "phd" => "phd",
    "philips" => "philips",
    "phone" => "phone",
    "photo" => "photo",
    "photography" => "photography",
    "photos" => "photos",
    "physio" => "physio",
    "pics" => "pics",
    "pictet" => "pictet",
    "pictures" => "pictures",
    "pid" => "pid",
    "pin" => "pin",
    "ping" => "ping",
    "pink" => "pink",
    "pioneer" => "pioneer",
    "pizza" => "pizza",
    "pk" => "pk",
    "pl" => "pl",
    "place" => "place",
    "play" => "play",
    "playstation" => "playstation",
    "plumbing" => "plumbing",
    "plus" => "plus",
    "pm" => "pm",
    "pn" => "pn",
    "pnc" => "pnc",
    "pohl" => "pohl",
    "poker" => "poker",
    "politie" => "politie",
    "porn" => "porn",
    "post" => "post",
    "pr" => "pr",
    "pramerica" => "pramerica",
    "praxi" => "praxi",
    "press" => "press",
    "prime" => "prime",
    "pro" => "pro",
    "prod" => "prod",
    "productions" => "productions",
    "prof" => "prof",
    "progressive" => "progressive",
    "promo" => "promo",
    "properties" => "properties",
    "property" => "property",
    "protection" => "protection",
    "pru" => "pru",
    "prudential" => "prudential",
    "ps" => "ps",
    "pt" => "pt",
    "pub" => "pub",
    "pw" => "pw",
    "pwc" => "pwc",
    "py" => "py",
    "qa" => "qa",
    "qpon" => "qpon",
    "quebec" => "quebec",
    "quest" => "quest",
    "racing" => "racing",
    "radio" => "radio",
    "re" => "re",
    "read" => "read",
    "realestate" => "realestate",
    "realtor" => "realtor",
    "realty" => "realty",
    "recipes" => "recipes",
    "red" => "red",
    "redstone" => "redstone",
    "redumbrella" => "redumbrella",
    "rehab" => "rehab",
    "reise" => "reise",
    "reisen" => "reisen",
    "reit" => "reit",
    "reliance" => "reliance",
    "ren" => "ren",
    "rent" => "rent",
    "rentals" => "rentals",
    "repair" => "repair",
    "report" => "report",
    "republican" => "republican",
    "rest" => "rest",
    "restaurant" => "restaurant",
    "review" => "review",
    "reviews" => "reviews",
    "rexroth" => "rexroth",
    "rich" => "rich",
    "richardli" => "richardli",
    "ricoh" => "ricoh",
    "ril" => "ril",
    "rio" => "rio",
    "rip" => "rip",
    "ro" => "ro",
    "rocks" => "rocks",
    "rodeo" => "rodeo",
    "rogers" => "rogers",
    "room" => "room",
    "rs" => "rs",
    "rsvp" => "rsvp",
    "ru" => "ru",
    "rugby" => "rugby",
    "ruhr" => "ruhr",
    "run" => "run",
    "rw" => "rw",
    "rwe" => "rwe",
    "ryukyu" => "ryukyu",
    "sa" => "sa",
    "saarland" => "saarland",
    "safe" => "safe",
    "safety" => "safety",
    "sakura" => "sakura",
    "sale" => "sale",
    "salon" => "salon",
    "samsclub" => "samsclub",
    "samsung" => "samsung",
    "sandvik" => "sandvik",
    "sandvikcoromant" => "sandvikcoromant",
    "sanofi" => "sanofi",
    "sap" => "sap",
    "sarl" => "sarl",
    "sas" => "sas",
    "save" => "save",
    "saxo" => "saxo",
    "sb" => "sb",
    "sbi" => "sbi",
    "sbs" => "sbs",
    "sc" => "sc",
    "scb" => "scb",
    "schaeffler" => "schaeffler",
    "schmidt" => "schmidt",
    "scholarships" => "scholarships",
    "school" => "school",
    "schule" => "schule",
    "schwarz" => "schwarz",
    "science" => "science",
    "scot" => "scot",
    "sd" => "sd",
    "se" => "se",
    "search" => "search",
    "seat" => "seat",
    "secure" => "secure",
    "security" => "security",
    "seek" => "seek",
    "select" => "select",
    "sener" => "sener",
    "services" => "services",
    "seven" => "seven",
    "sew" => "sew",
    "sex" => "sex",
    "sexy" => "sexy",
    "sfr" => "sfr",
    "sg" => "sg",
    "sh" => "sh",
    "shangrila" => "shangrila",
    "sharp" => "sharp",
    "shaw" => "shaw",
    "shell" => "shell",
    "shia" => "shia",
    "shiksha" => "shiksha",
    "shoes" => "shoes",
    "shop" => "shop",
    "shopping" => "shopping",
    "shouji" => "shouji",
    "show" => "show",
    "si" => "si",
    "silk" => "silk",
    "sina" => "sina",
    "singles" => "singles",
    "site" => "site",
    "sj" => "sj",
    "sk" => "sk",
    "ski" => "ski",
    "skin" => "skin",
    "sky" => "sky",
    "skype" => "skype",
    "sl" => "sl",
    "sling" => "sling",
    "sm" => "sm",
    "smart" => "smart",
    "smile" => "smile",
    "sn" => "sn",
    "sncf" => "sncf",
    "so" => "so",
    "soccer" => "soccer",
    "social" => "social",
    "softbank" => "softbank",
    "software" => "software",
    "sohu" => "sohu",
    "solar" => "solar",
    "solutions" => "solutions",
    "song" => "song",
    "sony" => "sony",
    "soy" => "soy",
    "spa" => "spa",
    "space" => "space",
    "sport" => "sport",
    "spot" => "spot",
    "sr" => "sr",
    "srl" => "srl",
    "ss" => "ss",
    "st" => "st",
    "stada" => "stada",
    "staples" => "staples",
    "star" => "star",
    "statebank" => "statebank",
    "statefarm" => "statefarm",
    "stc" => "stc",
    "stcgroup" => "stcgroup",
    "stockholm" => "stockholm",
    "storage" => "storage",
    "store" => "store",
    "stream" => "stream",
    "studio" => "studio",
    "study" => "study",
    "style" => "style",
    "su" => "su",
    "sucks" => "sucks",
    "supplies" => "supplies",
    "supply" => "supply",
    "support" => "support",
    "surf" => "surf",
    "surgery" => "surgery",
    "suzuki" => "suzuki",
    "sv" => "sv",
    "swatch" => "swatch",
    "swiss" => "swiss",
    "sx" => "sx",
    "sy" => "sy",
    "sydney" => "sydney",
    "systems" => "systems",
    "sz" => "sz",
    "tab" => "tab",
    "taipei" => "taipei",
    "talk" => "talk",
    "taobao" => "taobao",
    "target" => "target",
    "tatamotors" => "tatamotors",
    "tatar" => "tatar",
    "tattoo" => "tattoo",
    "tax" => "tax",
    "taxi" => "taxi",
    "tc" => "tc",
    "tci" => "tci",
    "td" => "td",
    "tdk" => "tdk",
    "team" => "team",
    "tech" => "tech",
    "technology" => "technology",
    "tel" => "tel",
    "temasek" => "temasek",
    "tennis" => "tennis",
    "teva" => "teva",
    "tf" => "tf",
    "tg" => "tg",
    "th" => "th",
    "thd" => "thd",
    "theater" => "theater",
    "theatre" => "theatre",
    "tiaa" => "tiaa",
    "tickets" => "tickets",
    "tienda" => "tienda",
    "tips" => "tips",
    "tires" => "tires",
    "tirol" => "tirol",
    "tj" => "tj",
    "tjmaxx" => "tjmaxx",
    "tjx" => "tjx",
    "tk" => "tk",
    "tkmaxx" => "tkmaxx",
    "tl" => "tl",
    "tm" => "tm",
    "tmall" => "tmall",
    "tn" => "tn",
    "to" => "to",
    "today" => "today",
    "tokyo" => "tokyo",
    "tools" => "tools",
    "top" => "top",
    "toray" => "toray",
    "toshiba" => "toshiba",
    "total" => "total",
    "tours" => "tours",
    "town" => "town",
    "toyota" => "toyota",
    "toys" => "toys",
    "tr" => "tr",
    "trade" => "trade",
    "trading" => "trading",
    "training" => "training",
    "travel" => "travel",
    "travelers" => "travelers",
    "travelersinsurance" => "travelersinsurance",
    "trust" => "trust",
    "trv" => "trv",
    "tt" => "tt",
    "tube" => "tube",
    "tui" => "tui",
    "tunes" => "tunes",
    "tushu" => "tushu",
    "tv" => "tv",
    "tvs" => "tvs",
    "tw" => "tw",
    "tz" => "tz",
    "ua" => "ua",
    "ubank" => "ubank",
    "ubs" => "ubs",
    "ug" => "ug",
    "uk" => "uk",
    "unicom" => "unicom",
    "university" => "university",
    "uno" => "uno",
    "uol" => "uol",
    "ups" => "ups",
    "us" => "us",
    "uy" => "uy",
    "uz" => "uz",
    "va" => "va",
    "vacations" => "vacations",
    "vana" => "vana",
    "vanguard" => "vanguard",
    "vc" => "vc",
    "ve" => "ve",
    "vegas" => "vegas",
    "ventures" => "ventures",
    "verisign" => "verisign",
    "versicherung" => "versicherung",
    "vet" => "vet",
    "vg" => "vg",
    "vi" => "vi",
    "viajes" => "viajes",
    "video" => "video",
    "vig" => "vig",
    "viking" => "viking",
    "villas" => "villas",
    "vin" => "vin",
    "vip" => "vip",
    "virgin" => "virgin",
    "visa" => "visa",
    "vision" => "vision",
    "viva" => "viva",
    "vivo" => "vivo",
    "vlaanderen" => "vlaanderen",
    "vn" => "vn",
    "vodka" => "vodka",
    "volvo" => "volvo",
    "vote" => "vote",
    "voting" => "voting",
    "voto" => "voto",
    "voyage" => "voyage",
    "vu" => "vu",
    "wales" => "wales",
    "walmart" => "walmart",
    "walter" => "walter",
    "wang" => "wang",
    "wanggou" => "wanggou",
    "watch" => "watch",
    "watches" => "watches",
    "weather" => "weather",
    "weatherchannel" => "weatherchannel",
    "webcam" => "webcam",
    "weber" => "weber",
    "website" => "website",
    "wed" => "wed",
    "wedding" => "wedding",
    "weibo" => "weibo",
    "weir" => "weir",
    "wf" => "wf",
    "whoswho" => "whoswho",
    "wien" => "wien",
    "wiki" => "wiki",
    "williamhill" => "williamhill",
    "win" => "win",
    "windows" => "windows",
    "wine" => "wine",
    "winners" => "winners",
    "wme" => "wme",
    "wolterskluwer" => "wolterskluwer",
    "woodside" => "woodside",
    "work" => "work",
    "works" => "works",
    "world" => "world",
    "wow" => "wow",
    "ws" => "ws",
    "wtc" => "wtc",
    "wtf" => "wtf",
    "xbox" => "xbox",
    "xerox" => "xerox",
    "xihuan" => "xihuan",
    "xin" => "xin",
    "xxx" => "xxx",
    "xyz" => "xyz",
    "yachts" => "yachts",
    "yahoo" => "yahoo",
    "yamaxun" => "yamaxun",
    "yandex" => "yandex",
    "ye" => "ye",
    "yodobashi" => "yodobashi",
    "yoga" => "yoga",
    "yokohama" => "yokohama",
    "you" => "you",
    "youtube" => "youtube",
    "yt" => "yt",
    "yun" => "yun",
    "za" => "za",
    "zappos" => "zappos",
    "zara" => "zara",
    "zero" => "zero",
    "zip" => "zip",
    "zm" => "zm",
    "zone" => "zone",
    "zuerich" => "zuerich",
    "zw" => "zw"
};
