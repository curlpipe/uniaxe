// Contains a large amount of unicode character conversion tables
use std::collections::HashMap;

pub const TEMPLATE: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";

#[must_use]
pub fn set_helper(from: &str, to: &str) -> HashMap<char, char> {
    // Provide a neat way of converting entire alphabets
    let mut result = HashMap::new();
    for (f, t) in from.chars().zip(to.chars()) {
        result.insert(f, t);
    }
    result
}

#[must_use]
pub fn generate_table() -> HashMap<char, char> {
    // Generate a lookup table
    let mut result = HashMap::new();
    result.extend(set_helper(
        "\u{1d51e}\u{1d51f}\u{1d520}\u{1d521}\u{1d522}\u{1d523}\u{1d524}\u{1d525}\u{1d526}\u{1d527}\u{1d528}\u{1d529}\u{1d52a}\u{1d52b}\u{1d52c}\u{1d52d}\u{1d52e}\u{1d52f}\u{1d530}\u{1d531}\u{1d532}\u{1d533}\u{1d534}\u{1d535}\u{1d536}\u{1d537}\u{1d504}\u{1d505}\u{212d}\u{1d507}\u{1d508}\u{1d509}\u{1d50a}\u{210c}\u{2111}\u{1d50d}\u{1d50e}\u{1d50f}\u{1d510}\u{1d511}\u{1d512}\u{1d513}\u{1d514}\u{211c}\u{1d516}\u{1d517}\u{1d518}\u{1d519}\u{1d51a}\u{1d51b}\u{1d51c}\u{2128}0123456789",
        TEMPLATE,
    ));
    result.extend(set_helper(
        "\u{1d586}\u{1d587}\u{1d588}\u{1d589}\u{1d58a}\u{1d58b}\u{1d58c}\u{1d58d}\u{1d58e}\u{1d58f}\u{1d590}\u{1d591}\u{1d592}\u{1d593}\u{1d594}\u{1d595}\u{1d596}\u{1d597}\u{1d598}\u{1d599}\u{1d59a}\u{1d59b}\u{1d59c}\u{1d59d}\u{1d59e}\u{1d59f}\u{1d56c}\u{1d56d}\u{1d56e}\u{1d56f}\u{1d570}\u{1d571}\u{1d572}\u{1d573}\u{1d574}\u{1d575}\u{1d576}\u{1d577}\u{1d578}\u{1d579}\u{1d57a}\u{1d57b}\u{1d57c}\u{1d57d}\u{1d57e}\u{1d57f}\u{1d580}\u{1d581}\u{1d582}\u{1d583}\u{1d584}\u{1d585}0123456789",
        TEMPLATE,
    ));
    result.extend(set_helper(
        "\u{1d4ea}\u{1d4eb}\u{1d4ec}\u{1d4ed}\u{1d4ee}\u{1d4ef}\u{1d4f0}\u{1d4f1}\u{1d4f2}\u{1d4f3}\u{1d4f4}\u{1d4f5}\u{1d4f6}\u{1d4f7}\u{1d4f8}\u{1d4f9}\u{1d4fa}\u{1d4fb}\u{1d4fc}\u{1d4fd}\u{1d4fe}\u{1d4ff}\u{1d500}\u{1d501}\u{1d502}\u{1d503}\u{1d4d0}\u{1d4d1}\u{1d4d2}\u{1d4d3}\u{1d4d4}\u{1d4d5}\u{1d4d6}\u{1d4d7}\u{1d4d8}\u{1d4d9}\u{1d4da}\u{1d4db}\u{1d4dc}\u{1d4dd}\u{1d4de}\u{1d4df}\u{1d4e0}\u{1d4e1}\u{1d4e2}\u{1d4e3}\u{1d4e4}\u{1d4e5}\u{1d4e6}\u{1d4e7}\u{1d4e8}\u{1d4e9}0123456789",
        TEMPLATE,
    ));
    result.extend(set_helper(
        "\u{1d4b6}\u{1d4b7}\u{1d4b8}\u{1d4b9}\u{1d452}\u{1d4bb}\u{1d454}\u{1d4bd}\u{1d4be}\u{1d4bf}\u{1d4c0}\u{1d4c1}\u{1d4c2}\u{1d4c3}\u{1d45c}\u{1d4c5}\u{1d4c6}\u{1d4c7}\u{1d4c8}\u{1d4c9}\u{1d4ca}\u{1d4cb}\u{1d4cc}\u{1d4cd}\u{1d4ce}\u{1d4cf}\u{1d49c}\u{1d435}\u{1d49e}\u{1d49f}\u{1d438}\u{1d439}\u{1d4a2}\u{1d43b}\u{1d43c}\u{1d4a5}\u{1d4a6}\u{1d43f}\u{1d440}\u{1d4a9}\u{1d4aa}\u{1d4ab}\u{1d4ac}\u{1d445}\u{1d4ae}\u{1d4af}\u{1d4b0}\u{1d4b1}\u{1d4b2}\u{1d4b3}\u{1d4b4}\u{1d4b5}\u{1d7e2}\u{1d7e3}\u{1d7e4}\u{1d7e5}\u{1d7e6}\u{1d7e7}\u{1d7e8}\u{1d7e9}\u{1d7ea}\u{1d7eb}",
        TEMPLATE,
    ));
    result.extend(set_helper(
        "\u{1d552}\u{1d553}\u{1d554}\u{1d555}\u{1d556}\u{1d557}\u{1d558}\u{1d559}\u{1d55a}\u{1d55b}\u{1d55c}\u{1d55d}\u{1d55e}\u{1d55f}\u{1d560}\u{1d561}\u{1d562}\u{1d563}\u{1d564}\u{1d565}\u{1d566}\u{1d567}\u{1d568}\u{1d569}\u{1d56a}\u{1d56b}\u{1d538}\u{1d539}\u{2102}\u{1d53b}\u{1d53c}\u{1d53d}\u{1d53e}\u{210d}\u{1d540}\u{1d541}\u{1d542}\u{1d543}\u{1d544}\u{2115}\u{1d546}\u{2119}\u{211a}\u{211d}\u{1d54a}\u{1d54b}\u{1d54c}\u{1d54d}\u{1d54e}\u{1d54f}\u{1d550}\u{2124}\u{1d7d8}\u{1d7d9}\u{1d7da}\u{1d7db}\u{1d7dc}\u{1d7dd}\u{1d7de}\u{1d7df}\u{1d7e0}\u{1d7e1}",
        TEMPLATE,
    ));
    result.extend(set_helper("\u{ff41}\u{ff42}\u{ff43}\u{ff44}\u{ff45}\u{ff46}\u{ff47}\u{ff48}\u{ff49}\u{ff4a}\u{ff4b}\u{ff4c}\u{ff4d}\u{ff4e}\u{ff4f}\u{ff50}\u{ff51}\u{ff52}\u{ff53}\u{ff54}\u{ff55}\u{ff56}\u{ff57}\u{ff58}\u{ff59}\u{ff5a}\u{ff21}\u{ff22}\u{ff23}\u{ff24}\u{ff25}\u{ff26}\u{ff27}\u{ff28}\u{ff29}\u{ff2a}\u{ff2b}\u{ff2c}\u{ff2d}\u{ff2e}\u{ff2f}\u{ff30}\u{ff31}\u{ff32}\u{ff33}\u{ff34}\u{ff35}\u{ff36}\u{ff37}\u{ff38}\u{ff39}\u{ff3a}\u{ff10}\u{ff11}\u{ff12}\u{ff13}\u{ff14}\u{ff15}\u{ff16}\u{ff17}\u{ff18}\u{ff19}", TEMPLATE));
    result.extend(set_helper(
        "\u{1d4b6}\u{1d4b7}\u{1d4b8}\u{1d4b9}\u{1d452}\u{1d4bb}\u{1d454}\u{1d4bd}\u{1d4be}\u{1d4bf}\u{1d4c0}\u{1d4c1}\u{1d4c2}\u{1d4c3}\u{2741}\u{1d4c5}\u{1d4c6}\u{1d4c7}\u{1d4c8}\u{1d4c9}\u{1d4ca}\u{1d4cb}\u{1d4cc}\u{1d4cd}\u{1d4ce}\u{1d4cf}\u{1d49c}\u{1d435}\u{1d49e}\u{1d49f}\u{1d438}\u{1d439}\u{1d4a2}\u{1d43b}\u{1d43c}\u{1d4a5}\u{1d4a6}\u{1d43f}\u{1d440}\u{1d4a9}\u{1f338}\u{1d4ab}\u{1d4ac}\u{1d445}\u{1d4ae}\u{1d4af}\u{1d4b0}\u{1d4b1}\u{1d4b2}\u{1d4b3}\u{1d4b4}\u{1d4b5}\u{1f36a}\u{1d7e3}\u{1d7e4}\u{1d7e5}\u{1d7e6}\u{1d7e7}\u{1d7e8}\u{1d7e9}\u{1d7ea}\u{1d7eb}",
        TEMPLATE,
    ));
    result.extend(set_helper(
        "\u{1d00}\u{299}\u{1d04}\u{1d05}\u{1d07}\u{a730}\u{262}\u{29c}\u{26a}\u{1d0a}\u{1d0b}\u{29f}\u{1d0d}\u{274}\u{1d0f}\u{1d18}Q\u{280}\u{a731}\u{1d1b}\u{1d1c}\u{1d20}\u{1d21}x\u{28f}\u{1d22}\u{1d00}\u{299}\u{1d04}\u{1d05}\u{1d07}\u{a730}\u{262}\u{29c}\u{26a}\u{1d0a}\u{1d0b}\u{29f}\u{1d0d}\u{274}\u{1d0f}\u{1d18}Q\u{280}\u{a731}\u{1d1b}\u{1d1c}\u{1d20}\u{1d21}x\u{28f}\u{1d22}0123456789",
        TEMPLATE,
    ));
    result.extend(set_helper(
        "\u{250}q\u{254}p\u{1dd}\u{25f}\u{253}\u{265}\u{131}\u{27e}\u{29e}l\u{26f}uodb\u{279}s\u{287}n\u{28c}\u{28d}x\u{28e}z\u{2200}\u{1660}\u{186}\u{15e1}\u{18e}\u{2132}\u{2141}HI\u{17f}\u{22ca}\u{2e5}WNO\u{500}\u{38c}\u{1d1a}S\u{22a5}\u{2229}\u{39b}MX\u{2144}Z0\u{21c2}\u{1105}\u{190}\u{3123}\u{78e}9\u{3125}86",
        TEMPLATE,
    ));
    result.extend(set_helper(
        "\u{1f130}\u{1f131}\u{1f132}\u{1f133}\u{1f134}\u{1f135}\u{1f136}\u{1f137}\u{1f138}\u{1f139}\u{1f13a}\u{1f13b}\u{1f13c}\u{1f13d}\u{1f13e}\u{1f13f}\u{1f140}\u{1f141}\u{1f142}\u{1f143}\u{1f144}\u{1f145}\u{1f146}\u{1f147}\u{1f148}\u{1f149}\u{1f130}\u{1f131}\u{1f132}\u{1f133}\u{1f134}\u{1f135}\u{1f136}\u{1f137}\u{1f138}\u{1f139}\u{1f13a}\u{1f13b}\u{1f13c}\u{1f13d}\u{1f13e}\u{1f13f}\u{1f140}\u{1f141}\u{1f142}\u{1f143}\u{1f144}\u{1f145}\u{1f146}\u{1f147}\u{1f148}\u{1f149}0123456789",
        TEMPLATE,
    ));
    result.extend(set_helper(
        "\u{e0}\u{334}\u{353}\u{32e}\u{32a}\u{319}\u{32c}\u{354}\u{300}\u{31a}b\u{336}\u{327}\u{31d}\u{312}c\u{336}\u{353}\u{33d}\u{35b}\u{357}\u{309}\u{34c}\u{308}\u{301}\u{315}\u{31a}\u{35d}d\u{335}\u{318}\u{32f}\u{303}\u{307}\u{313}\u{301}\u{310}\u{308}\u{301}\u{301}\u{306}\u{315}\u{315}e\u{334}\u{321}\u{31e}\u{324}\u{320}\u{354}\u{339}\u{348}\u{34b}\u{34b}\u{33f}\u{33e}\u{35d}\u{345}f\u{335}\u{31e}\u{318}\u{318}\u{330}\u{317}\u{346}\u{345}g\u{335}\u{32c}\u{310}\u{34b}\u{315}\u{1e25}\u{336}\u{318}\u{32e}\u{32a}\u{349}\u{33b}\u{33b}\u{33b}\u{350}\u{ec}\u{337}\u{31d}\u{33e}\u{312}\u{33d}j\u{338}\u{327}\u{32e}\u{33c}\u{354}\u{33a}\u{329}\u{330}\u{305}\u{308}\u{315}k\u{338}\u{316}\u{354}\u{316}\u{329}\u{320}\u{308}\u{301}\u{302}\u{357}\u{30d}l\u{334}\u{319}\u{31f}\u{33f}m\u{337}\u{31c}\u{32a}\u{316}\u{324}\u{349}\u{30b}\u{30b}\u{30f}\u{342}\u{346}\u{312}\u{352}\u{30f}\u{35c}\u{146}\u{334}\u{349}\u{32b}\u{33b}\u{32b}\u{330}\u{33c}\u{310}\u{301}\u{350}\u{30a}\u{360}\u{35d}o\u{337}\u{34d}\u{349}\u{34d}\u{33f}\u{360}p\u{336}\u{31d}\u{31e}\u{347}\u{329}\u{306}\u{30b}\u{314}\u{30f}\u{314}q\u{336}\u{31d}\u{32f}\u{349}\u{333}\u{34d}\u{317}\u{330}\u{34a}\u{211}\u{336}\u{35a}\u{331}\u{332}\u{32e}\u{31d}\u{34c}s\u{334}\u{322}\u{31b}\u{317}\u{326}\u{333}\u{347}\u{355}\u{348}\u{31c}\u{312}\u{33d}\u{163}\u{335}\u{31b}\u{31f}\u{325}\u{332}\u{33a}\u{31c}\u{353}\u{34a}\u{313}\u{30a}\u{314}\u{30a}\u{308}\u{1b0}\u{337}\u{33c}\u{313}\u{31a}\u{358}v\u{338}\u{31b}\u{34e}\u{332}\u{314}\u{308}\u{301}\u{34b}\u{342}\u{30a}\u{357}\u{307}\u{35c}w\u{334}\u{35a}\u{32c}\u{347}\u{349}\u{347}\u{306}\u{352}\u{301}\u{304}x\u{334}\u{31b}\u{326}\u{325}\u{342}\u{300}\u{34c}y\u{336}\u{327}\u{32b}\u{32a}\u{359}\u{356}\u{333}\u{326}\u{30e}\u{301}\u{311}\u{357}\u{315}z\u{334}\u{328}\u{349}\u{35a}\u{32e}\u{317}\u{324}\u{33b}\u{330}\u{347}\u{350}\u{301}\u{34b}A\u{338}\u{320}\u{316}\u{324}\u{346}B\u{337}\u{35a}\u{346}\u{300}\u{c7}\u{338}\u{327}\u{326}\u{325}\u{33a}\u{354}\u{323}\u{32a}\u{315}\u{1e0c}\u{338}\u{320}\u{320}\u{329}\u{330}\u{312}\u{116}\u{337}\u{34e}\u{30e}\u{357}\u{350}\u{357}\u{313}\u{34a}\u{315}F\u{338}\u{321}\u{329}\u{330}\u{33c}\u{354}\u{304}\u{30e}\u{31a}\u{360}G\u{336}\u{328}\u{348}\u{316}\u{32e}\u{316}\u{356}\u{34e}\u{31c}\u{35b}\u{303}\u{309}\u{34a}\u{301}\u{309}\u{308}\u{301}\u{35c}\u{360}H\u{338}\u{316}\u{316}\u{324}\u{348}\u{32b}\u{347}\u{35a}\u{347}\u{356}\u{31c}\u{35b}\u{34c}\u{30e}\u{346}\u{312}\u{33d}\u{315}\u{360}\u{360}\u{1e2e}\u{336}\u{321}\u{321}\u{329}\u{324}\u{331}\u{34c}\u{351}\u{33e}\u{30c}\u{34b}\u{30d}\u{302}\u{315}J\u{337}\u{339}\u{359}\u{32e}\u{308}\u{301}\u{351}\u{357}\u{30a}K\u{338}\u{328}\u{34e}\u{348}\u{331}\u{332}\u{32a}\u{326}\u{33a}\u{30a}\u{30c}\u{312}\u{313}\u{1e3c}\u{336}\u{328}\u{332}\u{356}\u{31c}\u{333}\u{32c}\u{357}\u{357}\u{306}\u{33f}M\u{334}\u{32d}\u{33e}\u{312}\u{358}N\u{335}\u{31e}\u{356}\u{32d}\u{31d}\u{311}\u{351}\u{313}\u{302}\u{309}\u{350}\u{358}\u{345}\u{1ece}\u{334}\u{31f}\u{356}\u{32c}\u{354}\u{34e}\u{331}\u{330}\u{31c}\u{309}\u{342}\u{351}\u{301}\u{33e}\u{33d}\u{315}\u{31a}\u{31a}P\u{335}\u{32f}\u{333}\u{32c}\u{355}\u{319}\u{333}\u{32a}\u{332}\u{319}\u{306}\u{30a}\u{34c}\u{33d}\u{346}\u{30e}\u{360}\u{35d}\u{360}Q\u{336}\u{320}\u{32e}\u{32f}\u{318}\u{326}\u{34c}\u{34a}\u{307}\u{301}\u{308}\u{301}\u{305}\u{315}\u{360}R\u{334}\u{347}\u{32a}\u{356}\u{317}\u{30b}\u{30b}S\u{337}\u{348}\u{33c}\u{316}\u{332}\u{313}\u{308}\u{301}\u{304}\u{30f}\u{315}\u{360}\u{162}\u{336}\u{31f}\u{35a}\u{35a}\u{32e}\u{354}\u{33f}\u{30b}\u{33e}\u{307}\u{33d}\u{301}\u{308}\u{301}\u{35c}\u{360}\u{35d}\u{35d}\u{d9}\u{337}\u{34d}\u{32e}\u{314}\u{35c}\u{35d}V\u{335}\u{327}\u{34e}\u{33c}\u{34d}\u{359}\u{318}\u{33c}\u{32e}\u{355}\u{317}\u{357}\u{312}\u{311}W\u{336}\u{359}\u{320}\u{342}\u{310}\u{308}\u{301}\u{310}\u{308}\u{301}\u{33f}\u{357}\u{31a}\u{358}\u{360}X\u{338}\u{355}\u{31f}\u{331}\u{333}\u{324}\u{323}\u{32d}\u{339}\u{331}\u{31e}\u{301}\u{300}\u{34b}\u{308}\u{301}\u{342}\u{307}\u{351}\u{30e}\u{31a}\u{1e8e}\u{334}\u{322}\u{354}\u{355}\u{353}\u{31d}\u{349}\u{32e}\u{30a}\u{313}\u{315}\u{35d}Z\u{336}\u{34e}\u{33a}\u{332}\u{32e}\u{357}\u{30a}\u{307}\u{345}0\u{338}\u{31b}\u{32f}\u{325}\u{33b}\u{319}\u{31f}\u{349}\u{34e}\u{313}\u{357}\u{350}\u{309}\u{315}1\u{337}\u{329}\u{30e}\u{313}\u{300}\u{360}2\u{337}\u{31f}\u{347}\u{32d}\u{33b}\u{300}3\u{338}\u{330}\u{30d}4\u{338}\u{34d}\u{323}\u{349}\u{339}\u{318}\u{30e}\u{310}\u{304}\u{307}\u{34a}\u{31a}5\u{336}\u{325}\u{331}\u{32a}\u{307}\u{309}\u{301}\u{34c}\u{304}\u{33d}\u{35d}6\u{338}\u{31b}\u{330}\u{31c}\u{320}\u{324}\u{331}\u{301}\u{33d}7\u{334}\u{32f}\u{333}\u{329}\u{355}\u{349}\u{300}\u{35b}\u{30a}\u{350}\u{357}\u{307}\u{313}\u{35d}\u{35d}8\u{334}\u{330}\u{331}\u{32f}\u{326}\u{311}\u{301}\u{313}\u{301}\u{301}\u{301}\u{300}\u{313}\u{35b}\u{301}9\u{335}\u{330}\u{31f}\u{300}\u{300}\u{308}\u{301}\u{308}\u{301}\u{351}\u{303}\u{358}",
        TEMPLATE,
    ));
    result.extend(set_helper(
        "\u{1f170}\u{1f171}\u{1f172}\u{1f173}\u{1f174}\u{1f175}\u{1f176}\u{1f177}\u{1f178}\u{1f179}\u{1f17a}\u{1f17b}\u{1f17c}\u{1f17d}\u{1f17e}\u{1f17f}\u{1f180}\u{1f181}\u{1f182}\u{1f183}\u{1f184}\u{1f185}\u{1f186}\u{1f187}\u{1f188}\u{1f189}\u{1f170}\u{1f171}\u{1f172}\u{1f173}\u{1f174}\u{1f175}\u{1f176}\u{1f177}\u{1f178}\u{1f179}\u{1f17a}\u{1f17b}\u{1f17c}\u{1f17d}\u{1f17e}\u{1f17f}\u{1f180}\u{1f181}\u{1f182}\u{1f183}\u{1f184}\u{1f185}\u{1f186}\u{1f187}\u{1f188}\u{1f189}0123456789",
        TEMPLATE,
    ));
    result.extend(set_helper(
        "\u{2090}bcd\u{2091}fg\u{2095}\u{1d62}\u{2c7c}\u{2096}\u{2097}\u{2098}\u{2099}\u{2092}\u{209a}q\u{1d63}\u{209b}\u{209c}\u{1d64}\u{1d65}w\u{2093}yz\u{2090}BCD\u{2091}FG\u{2095}\u{1d62}\u{2c7c}\u{2096}\u{2097}\u{2098}\u{2099}\u{2092}\u{209a}Q\u{1d63}\u{209b}\u{209c}\u{1d64}\u{1d65}W\u{2093}YZ\u{2080}\u{2081}\u{2082}\u{2083}\u{2084}\u{2085}\u{2086}\u{2087}\u{2088}\u{2089}",
        TEMPLATE,
    ));
    result.extend(set_helper(
        "\u{1d43}\u{1d47}\u{1d9c}\u{1d48}\u{1d49}\u{1da0}\u{1d4d}\u{2b0}\u{2071}\u{2b2}\u{1d4f}\u{2e1}\u{1d50}\u{207f}\u{1d52}\u{1d56}q\u{2b3}\u{2e2}\u{1d57}\u{1d58}\u{1d5b}\u{2b7}\u{2e3}\u{2b8}\u{1dbb}\u{1d2c}\u{1d2e}\u{1d9c}\u{1d30}\u{1d31}\u{1da0}\u{1d33}\u{1d34}\u{1d35}\u{1d36}\u{1d37}\u{1d38}\u{1d39}\u{1d3a}\u{1d3c}\u{1d3e}Q\u{1d3f}\u{2e2}\u{1d40}\u{1d41}\u{2c7d}\u{1d42}\u{2e3}\u{2b8}\u{1dbb}\u{2070}\u{b9}\u{b2}\u{b3}\u{2074}\u{2075}\u{2076}\u{2077}\u{2078}\u{2079}",
        TEMPLATE,
    ));
    result.extend(set_helper(
        "\u{24d0}\u{24d1}\u{24d2}\u{24d3}\u{24d4}\u{24d5}\u{24d6}\u{24d7}\u{24d8}\u{24d9}\u{24da}\u{24db}\u{24dc}\u{24dd}\u{24de}\u{24df}\u{24e0}\u{24e1}\u{24e2}\u{24e3}\u{24e4}\u{24e5}\u{24e6}\u{24e7}\u{24e8}\u{24e9}\u{24b6}\u{24b7}\u{24b8}\u{24b9}\u{24ba}\u{24bb}\u{24bc}\u{24bd}\u{24be}\u{24bf}\u{24c0}\u{24c1}\u{24c2}\u{24c3}\u{24c4}\u{24c5}\u{24c6}\u{24c7}\u{24c8}\u{24c9}\u{24ca}\u{24cb}\u{24cc}\u{24cd}\u{24ce}\u{24cf}\u{24ea}\u{2460}\u{2461}\u{2462}\u{2463}\u{2464}\u{2465}\u{2466}\u{2467}\u{2468}",
        TEMPLATE,
    ));
    result.extend(set_helper(
        "\u{e04}\u{e52}\u{3c2}\u{e54}\u{454}\u{166}\u{feee}\u{452}\u{e40}\u{5df}\u{43a}\u{26d}\u{e53}\u{e20}\u{e4f}\u{5e7}\u{1ee3}\u{433}\u{e23}\u{547}\u{e22}\u{5e9}\u{e2c}\u{5d0}\u{5e5}\u{579}\u{e04}\u{e52}\u{3c2}\u{e54}\u{454}\u{166}\u{feee}\u{452}\u{e40}\u{5df}\u{43a}\u{26d}\u{e53}\u{e20}\u{e4f}\u{5e7}\u{1ee3}\u{433}\u{e23}\u{547}\u{e22}\u{5e9}\u{e2c}\u{5d0}\u{5e5}\u{579}0123456789",
        TEMPLATE,
    ));
    result.extend(set_helper(
        "\u{3b1}\u{10a6}\u{188}\u{503}\u{4bd}\u{3dd}\u{260}\u{50b}\u{3b9}\u{29d}\u{199}\u{285}\u{271}\u{273}\u{3c3}\u{3c1}\u{3d9}\u{27e}\u{282}\u{19a}\u{3c5}\u{28b}\u{26f}x\u{10e7}\u{225}ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789",
        TEMPLATE,
    ));
    result.extend(set_helper(
        "\u{1df}\u{26e}\u{188}\u{256}\u{25b}\u{284}\u{262}\u{266}\u{268}\u{29d}\u{4c4}\u{29f}\u{28d}\u{57c}\u{585}\u{584}\u{566}\u{280}\u{586}\u{236}\u{28a}\u{28b}\u{561}\u{4fc}\u{28f}\u{290}\u{1df}\u{26e}\u{188}\u{256}\u{25b}\u{284}\u{262}\u{266}\u{268}\u{29d}\u{4c4}\u{29f}\u{28d}\u{57c}\u{585}\u{584}\u{566}\u{280}\u{586}\u{236}\u{28a}\u{28b}\u{561}\u{4fc}\u{28f}\u{290}0123456789",
        TEMPLATE,
    ));
    result.extend(set_helper(
        "\u{13d7}\u{13f0}\u{1348}\u{13b4}\u{13cb}\u{13a6}\u{13b6}\u{13c2}\u{13a5}\u{13e0}\u{13e6}\u{13dd}\u{13b7}\u{13c1}\u{13a7}\u{13ae}\u{13a4}\u{13d2}\u{13d5}\u{13d6}\u{13ec}\u{13c9}\u{13c7}\u{1300}\u{13a9}\u{135a}\u{13d7}\u{13f0}\u{1348}\u{13b4}\u{13cb}\u{13a6}\u{13b6}\u{13c2}\u{13a5}\u{13e0}\u{13e6}\u{13dd}\u{13b7}\u{13c1}\u{13a7}\u{13ae}\u{13a4}\u{13d2}\u{13d5}\u{13d6}\u{13ec}\u{13c9}\u{13c7}\u{1300}\u{13a9}\u{135a}0123456789",
        TEMPLATE,
    ));
    result.extend(set_helper(
        "\u{105}\u{10ea}\u{188}\u{256}\u{25b}\u{284}\u{260}\u{267}\u{131}\u{29d}\u{199}\u{196}\u{271}\u{14b}\u{1a1}\u{2118}\u{566}\u{f5e}\u{282}\u{26c}\u{173}\u{6f7}\u{1ff3}\u{4b3}\u{10e7}\u{291}\u{105}\u{10ea}\u{188}\u{256}\u{25b}\u{284}\u{260}\u{267}\u{131}\u{29d}\u{199}\u{196}\u{271}\u{14b}\u{1a1}\u{2118}\u{566}\u{f5e}\u{282}\u{26c}\u{173}\u{6f7}\u{1ff3}\u{4b3}\u{10e7}\u{291}0123456789",
        TEMPLATE,
    ));
    result.extend(set_helper(
        "\u{e04}\u{e56}\u{a2}\u{ed3}\u{113}f\u{e87}hi\u{e27}kl\u{e53}\u{e96}\u{ed0}p\u{e51}r\u{15e}t\u{e19}\u{e07}\u{e9f}x\u{e2f}\u{e8a}\u{e04}\u{e56}\u{a2}\u{ed3}\u{113}f\u{e87}hi\u{e27}kl\u{e53}\u{e96}\u{ed0}p\u{e51}r\u{15e}t\u{e19}\u{e07}\u{e9f}x\u{e2f}\u{e8a}0123456789",
        TEMPLATE,
    ));
    result.extend(set_helper(
        "\u{1d41a}\u{1d41b}\u{1d41c}\u{1d41d}\u{1d41e}\u{1d41f}\u{1d420}\u{1d421}\u{1d422}\u{1d423}\u{1d424}\u{1d425}\u{1d426}\u{1d427}\u{1d428}\u{1d429}\u{1d42a}\u{1d42b}\u{1d42c}\u{1d42d}\u{1d42e}\u{1d42f}\u{1d430}\u{1d431}\u{1d432}\u{1d433}\u{1d400}\u{1d401}\u{1d402}\u{1d403}\u{1d404}\u{1d405}\u{1d406}\u{1d407}\u{1d408}\u{1d409}\u{1d40a}\u{1d40b}\u{1d40c}\u{1d40d}\u{1d40e}\u{1d40f}\u{1d410}\u{1d411}\u{1d412}\u{1d413}\u{1d414}\u{1d415}\u{1d416}\u{1d417}\u{1d418}\u{1d419}\u{1d7ce}\u{1d7cf}\u{1d7d0}\u{1d7d1}\u{1d7d2}\u{1d7d3}\u{1d7d4}\u{1d7d5}\u{1d7d6}\u{1d7d7}",
        TEMPLATE,
    ));
    result.extend(set_helper(
        "\u{1d622}\u{1d623}\u{1d624}\u{1d625}\u{1d626}\u{1d627}\u{1d628}\u{1d629}\u{1d62a}\u{1d62b}\u{1d62c}\u{1d62d}\u{1d62e}\u{1d62f}\u{1d630}\u{1d631}\u{1d632}\u{1d633}\u{1d634}\u{1d635}\u{1d636}\u{1d637}\u{1d638}\u{1d639}\u{1d63a}\u{1d63b}\u{1d608}\u{1d609}\u{1d60a}\u{1d60b}\u{1d60c}\u{1d60d}\u{1d60e}\u{1d60f}\u{1d610}\u{1d611}\u{1d612}\u{1d613}\u{1d614}\u{1d615}\u{1d616}\u{1d617}\u{1d618}\u{1d619}\u{1d61a}\u{1d61b}\u{1d61c}\u{1d61d}\u{1d61e}\u{1d61f}\u{1d620}\u{1d621}0123456789",
        TEMPLATE,
    ));
    result.extend(set_helper(
        "\u{1d656}\u{1d657}\u{1d658}\u{1d659}\u{1d65a}\u{1d65b}\u{1d65c}\u{1d65d}\u{1d65e}\u{1d65f}\u{1d660}\u{1d661}\u{1d662}\u{1d663}\u{1d664}\u{1d665}\u{1d666}\u{1d667}\u{1d668}\u{1d669}\u{1d66a}\u{1d66b}\u{1d66c}\u{1d66d}\u{1d66e}\u{1d66f}\u{1d63c}\u{1d63d}\u{1d63e}\u{1d63f}\u{1d640}\u{1d641}\u{1d642}\u{1d643}\u{1d644}\u{1d645}\u{1d646}\u{1d647}\u{1d648}\u{1d649}\u{1d64a}\u{1d64b}\u{1d64c}\u{1d64d}\u{1d64e}\u{1d64f}\u{1d650}\u{1d651}\u{1d652}\u{1d653}\u{1d654}\u{1d655}0123456789",
        TEMPLATE,
    ));
    result.extend(set_helper(
        "\u{1d68a}\u{1d68b}\u{1d68c}\u{1d68d}\u{1d68e}\u{1d68f}\u{1d690}\u{1d691}\u{1d692}\u{1d693}\u{1d694}\u{1d695}\u{1d696}\u{1d697}\u{1d698}\u{1d699}\u{1d69a}\u{1d69b}\u{1d69c}\u{1d69d}\u{1d69e}\u{1d69f}\u{1d6a0}\u{1d6a1}\u{1d6a2}\u{1d6a3}\u{1d670}\u{1d671}\u{1d672}\u{1d673}\u{1d674}\u{1d675}\u{1d676}\u{1d677}\u{1d678}\u{1d679}\u{1d67a}\u{1d67b}\u{1d67c}\u{1d67d}\u{1d67e}\u{1d67f}\u{1d680}\u{1d681}\u{1d682}\u{1d683}\u{1d684}\u{1d685}\u{1d686}\u{1d687}\u{1d688}\u{1d689}\u{1d7f6}\u{1d7f7}\u{1d7f8}\u{1d7f9}\u{1d7fa}\u{1d7fb}\u{1d7fc}\u{1d7fd}\u{1d7fe}\u{1d7ff}",
        TEMPLATE,
    ));
    result.extend(set_helper(
        "\u{39b}B\u{1103}D\u{3a3}FG\u{389}IJK\u{1102}M\u{41f}\u{4e8}PQ\u{42f}\u{1a7}\u{1ac}\u{426}V\u{429}XYZ\u{39b}B\u{1103}D\u{3a3}FG\u{389}IJK\u{1102}M\u{41f}\u{4e8}PQ\u{42f}\u{1a7}\u{1ac}\u{426}V\u{429}XYZ0123456789",
        TEMPLATE,
    ));
    result.extend(set_helper(
        "\u{3b1}\u{432}\u{a2}\u{2202}\u{454}\u{192}g\u{43d}\u{3b9}\u{5e0}\u{43a}\u{2113}\u{43c}\u{3b7}\u{3c3}\u{3c1}q\u{44f}\u{455}\u{442}\u{3c5}\u{3bd}\u{3c9}\u{3c7}\u{443}z\u{3b1}\u{432}\u{a2}\u{2202}\u{454}\u{192}g\u{43d}\u{3b9}\u{5e0}\u{43a}\u{2113}\u{43c}\u{3b7}\u{3c3}\u{3c1}q\u{44f}\u{455}\u{442}\u{3c5}\u{3bd}\u{3c9}\u{3c7}\u{443}z0123456789",
        TEMPLATE,
    ));
    result.extend(set_helper(
        "\u{e5}\u{df}\u{a2}\u{d0}\u{ea}\u{a3}gh\u{ef}jklm\u{f1}\u{f0}\u{fe}qr\u{a7}\u{2020}\u{b5}vwx\u{a5}z\u{c4}\u{df}\u{c7}\u{d0}\u{c8}\u{a3}GH\u{cc}JKLM\u{f1}\u{d6}\u{fe}QR\u{a7}\u{2020}\u{da}VW\u{d7}\u{a5}Z0123456789",
        TEMPLATE,
    ));
    result.extend(set_helper(
        "\u{20b3}\u{e3f}\u{20b5}\u{110}\u{246}\u{20a3}\u{20b2}\u{2c67}\u{142}J\u{20ad}\u{2c60}\u{20a5}\u{20a6}\u{d8}\u{20b1}Q\u{2c64}\u{20b4}\u{20ae}\u{244}V\u{20a9}\u{4fe}\u{24e}\u{2c6b}\u{20b3}\u{e3f}\u{20b5}\u{110}\u{246}\u{20a3}\u{20b2}\u{2c67}\u{142}J\u{20ad}\u{2c60}\u{20a5}\u{20a6}\u{d8}\u{20b1}Q\u{2c64}\u{20b4}\u{20ae}\u{244}V\u{20a9}\u{4fe}\u{24e}\u{2c6b}0123456789",
        TEMPLATE,
    ));
    result.extend(set_helper("\u{5342}\u{4e43}\u{531a}\u{15ea}\u{4e47}\u{5343}\u{13b6}\u{5344}\u{4e28}\u{ff8c}\u{49c}\u{3125}\u{722a}\u{51e0}\u{3116}\u{5369}\u{24a}\u{5c3a}\u{4e02}\u{3112}\u{3129}\u{142f}\u{5c71}\u{4e42}\u{311a}\u{4e59}\u{5342}\u{4e43}\u{531a}\u{15ea}\u{4e47}\u{5343}\u{13b6}\u{5344}\u{4e28}\u{ff8c}\u{49c}\u{3125}\u{722a}\u{51e0}\u{3116}\u{5369}\u{24a}\u{5c3a}\u{4e02}\u{3112}\u{3129}\u{142f}\u{5c71}\u{4e42}\u{311a}\u{4e59}0123456789", TEMPLATE));
    result.extend(set_helper("\u{ff91}\u{4e43}\u{1103}\u{308a}\u{4e47}\u{ff77}\u{30e0}\u{3093}\u{ff89}\u{ff8c}\u{30ba}\u{ff9a}\u{ffb6}\u{5200}\u{306e}\u{ff71}\u{3090}\u{5c3a}\u{4e02}\u{ff72}\u{3072}\u{221a}W\u{ff92}\u{ff98}\u{4e59}\u{ff91}\u{4e43}\u{1103}\u{308a}\u{4e47}\u{ff77}\u{30e0}\u{3093}\u{ff89}\u{ff8c}\u{30ba}\u{ff9a}\u{ffb6}\u{5200}\u{306e}\u{ff71}\u{3090}\u{5c3a}\u{4e02}\u{ff72}\u{3072}\u{221a}W\u{ff92}\u{ff98}\u{4e59}0123456789", TEMPLATE));
    result.extend(set_helper(
        "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789",
        TEMPLATE,
    ));
    result.extend(set_helper("\u{ff41}\u{ff42}\u{ff43}\u{ff44}\u{ff45}\u{ff46}\u{ff47}\u{ff48}\u{ff49}\u{ff4a}\u{ff4b}\u{ff4c}\u{ff4d}\u{ff4e}\u{ff4f}\u{ff50}\u{ff51}\u{ff52}\u{ff53}\u{ff54}\u{ff55}\u{ff56}\u{ff57}\u{ff58}\u{ff59}\u{ff5a}\u{ff21}\u{ff22}\u{ff23}\u{ff24}\u{ff25}\u{ff26}\u{ff27}\u{ff28}\u{ff29}\u{ff2a}\u{ff2b}\u{ff2c}\u{ff2d}\u{ff2e}\u{ff2f}\u{ff30}\u{ff31}\u{ff32}\u{ff33}\u{ff34}\u{ff35}\u{ff36}\u{ff37}\u{ff38}\u{ff39}\u{ff3a}\u{ff10}\u{ff11}\u{ff12}\u{ff13}\u{ff14}\u{ff15}\u{ff16}\u{ff17}\u{ff18}\u{ff19}", TEMPLATE));
    result.extend(set_helper("\u{ff41}\u{ff42}\u{ff43}\u{ff44}\u{ff45}\u{ff46}\u{ff47}\u{ff48}\u{ff49}\u{ff4a}\u{ff4b}\u{ff4c}\u{ff4d}\u{ff4e}\u{ff4f}\u{ff50}\u{ff51}\u{ff52}\u{ff53}\u{ff54}\u{ff55}\u{ff56}\u{ff57}\u{ff58}\u{ff59}\u{ff5a}\u{39b}\u{ff22}\u{ff23}\u{ff24}\u{39e}\u{ff26}\u{ff27}\u{ff28}\u{ff29}\u{ff2a}\u{ff2b}\u{ff2c}\u{ff2d}\u{ff2e}\u{2662}\u{ff30}\u{ff31}\u{ff32}\u{ff33}\u{ff34}\u{ff35}\u{ff36}\u{ff37}\u{ff38}\u{ff39}\u{ff3a}\u{ff10}\u{ff11}\u{ff12}\u{ff13}\u{ff14}\u{ff15}\u{ff16}\u{ff17}\u{ff18}\u{ff19}", TEMPLATE));
    result.extend(set_helper("\u{ff41}\u{ff42}\u{ff43}\u{ff44}\u{ff45}\u{ff46}\u{ff47}\u{ff48}\u{ff49}\u{ff4a}\u{ff4b}\u{ff4c}\u{ff4d}\u{ff4e}\u{ff4f}\u{ff50}\u{ff51}\u{ff52}\u{ff53}\u{ff54}\u{ff55}\u{ff56}\u{ff57}\u{ff58}\u{ff59}\u{ff5a}\u{ff21}\u{ff22}\u{ff23}\u{ff24}\u{ff25}\u{ff26}\u{ff27}\u{ff28}\u{ff29}\u{ff2a}\u{ff2b}\u{ff2c}\u{ff2d}\u{ff2e}\u{ff2f}\u{ff30}\u{ff31}\u{ff32}\u{ff33}\u{ff34}\u{ff35}\u{ff36}\u{ff37}\u{ff38}\u{ff39}\u{ff3a}\u{ff10}\u{ff11}\u{ff12}\u{ff13}\u{ff14}\u{ff15}\u{ff16}\u{ff17}\u{ff18}\u{ff19}", TEMPLATE));
    result.extend(set_helper(
        "a\u{489}b\u{489}c\u{489}d\u{489}e\u{489}f\u{489}g\u{489}h\u{489}i\u{489}j\u{489}k\u{489}l\u{489}m\u{489}n\u{489}o\u{489}p\u{489}q\u{489}r\u{489}s\u{489}t\u{489}u\u{489}v\u{489}w\u{489}x\u{489}y\u{489}z\u{489}A\u{489}B\u{489}C\u{489}D\u{489}E\u{489}F\u{489}G\u{489}H\u{489}I\u{489}J\u{489}K\u{489}L\u{489}M\u{489}N\u{489}O\u{489}P\u{489}Q\u{489}R\u{489}S\u{489}T\u{489}U\u{489}V\u{489}W\u{489}X\u{489}Y\u{489}Z\u{489}0\u{489}1\u{489}2\u{489}3\u{489}4\u{489}5\u{489}6\u{489}7\u{489}8\u{489}9\u{489}",
        TEMPLATE,
    ));
    result.extend(set_helper(
        "a\u{336}b\u{336}c\u{336}d\u{336}e\u{336}f\u{336}g\u{336}h\u{336}i\u{336}j\u{336}k\u{336}l\u{336}m\u{336}n\u{336}o\u{336}p\u{336}q\u{336}r\u{336}s\u{336}t\u{336}u\u{336}v\u{336}w\u{336}x\u{336}y\u{336}z\u{336}A\u{336}B\u{336}C\u{336}D\u{336}E\u{336}F\u{336}G\u{336}H\u{336}I\u{336}J\u{336}K\u{336}L\u{336}M\u{336}N\u{336}O\u{336}P\u{336}Q\u{336}R\u{336}S\u{336}T\u{336}U\u{336}V\u{336}W\u{336}X\u{336}Y\u{336}Z\u{336}0\u{336}1\u{336}2\u{336}3\u{336}4\u{336}5\u{336}6\u{336}7\u{336}8\u{336}9\u{336}",
        TEMPLATE,
    ));
    result.extend(set_helper(
        "a\u{334}b\u{334}c\u{334}d\u{334}e\u{334}f\u{334}g\u{334}h\u{334}i\u{334}j\u{334}k\u{334}l\u{334}m\u{334}n\u{334}o\u{334}p\u{334}q\u{334}r\u{334}s\u{334}t\u{334}u\u{334}v\u{334}w\u{334}x\u{334}y\u{334}z\u{334}A\u{334}B\u{334}C\u{334}D\u{334}E\u{334}F\u{334}G\u{334}H\u{334}I\u{334}J\u{334}K\u{334}L\u{334}M\u{334}N\u{334}O\u{334}P\u{334}Q\u{334}R\u{334}S\u{334}T\u{334}U\u{334}V\u{334}W\u{334}X\u{334}Y\u{334}Z\u{334}0\u{334}1\u{334}2\u{334}3\u{334}4\u{334}5\u{334}6\u{334}7\u{334}8\u{334}9\u{334}",
        TEMPLATE,
    ));
    result.extend(set_helper(
        "a\u{337}b\u{337}c\u{337}d\u{337}e\u{337}f\u{337}g\u{337}h\u{337}i\u{337}j\u{337}k\u{337}l\u{337}m\u{337}n\u{337}o\u{337}p\u{337}q\u{337}r\u{337}s\u{337}t\u{337}u\u{337}v\u{337}w\u{337}x\u{337}y\u{337}z\u{337}A\u{337}B\u{337}C\u{337}D\u{337}E\u{337}F\u{337}G\u{337}H\u{337}I\u{337}J\u{337}K\u{337}L\u{337}M\u{337}N\u{337}O\u{337}P\u{337}Q\u{337}R\u{337}S\u{337}T\u{337}U\u{337}V\u{337}W\u{337}X\u{337}Y\u{337}Z\u{337}0\u{337}1\u{337}2\u{337}3\u{337}4\u{337}5\u{337}6\u{337}7\u{337}8\u{337}9\u{337}",
        TEMPLATE,
    ));
    result.extend(set_helper(
        "a\u{332}b\u{332}c\u{332}d\u{332}e\u{332}f\u{332}g\u{332}h\u{332}i\u{332}j\u{332}k\u{332}l\u{332}m\u{332}n\u{332}o\u{332}p\u{332}q\u{332}r\u{332}s\u{332}t\u{332}u\u{332}v\u{332}w\u{332}x\u{332}y\u{332}z\u{332}A\u{332}B\u{332}C\u{332}D\u{332}E\u{332}F\u{332}G\u{332}H\u{332}I\u{332}J\u{332}K\u{332}L\u{332}M\u{332}N\u{332}O\u{332}P\u{332}Q\u{332}R\u{332}S\u{332}T\u{332}U\u{332}V\u{332}W\u{332}X\u{332}Y\u{332}Z\u{332}0\u{332}1\u{332}2\u{332}3\u{332}4\u{332}5\u{332}6\u{332}7\u{332}8\u{332}9\u{332}",
        TEMPLATE,
    ));
    result.extend(set_helper(
        "a\u{333}b\u{333}c\u{333}d\u{333}e\u{333}f\u{333}g\u{333}h\u{333}i\u{333}j\u{333}k\u{333}l\u{333}m\u{333}n\u{333}o\u{333}p\u{333}q\u{333}r\u{333}s\u{333}t\u{333}u\u{333}v\u{333}w\u{333}x\u{333}y\u{333}z\u{333}A\u{333}B\u{333}C\u{333}D\u{333}E\u{333}F\u{333}G\u{333}H\u{333}I\u{333}J\u{333}K\u{333}L\u{333}M\u{333}N\u{333}O\u{333}P\u{333}Q\u{333}R\u{333}S\u{333}T\u{333}U\u{333}V\u{333}W\u{333}X\u{333}Y\u{333}Z\u{333}0\u{333}1\u{333}2\u{333}3\u{333}4\u{333}5\u{333}6\u{333}7\u{333}8\u{333}9\u{333}",
        TEMPLATE,
    ));
    result.extend(set_helper(
        "a\u{33e}b\u{33e}c\u{33e}d\u{33e}e\u{33e}f\u{33e}g\u{33e}h\u{33e}i\u{33e}j\u{33e}k\u{33e}l\u{33e}m\u{33e}n\u{33e}o\u{33e}p\u{33e}q\u{33e}r\u{33e}s\u{33e}t\u{33e}u\u{33e}v\u{33e}w\u{33e}x\u{33e}y\u{33e}z\u{33e}A\u{33e}B\u{33e}C\u{33e}D\u{33e}E\u{33e}F\u{33e}G\u{33e}H\u{33e}I\u{33e}J\u{33e}K\u{33e}L\u{33e}M\u{33e}N\u{33e}O\u{33e}P\u{33e}Q\u{33e}R\u{33e}S\u{33e}T\u{33e}U\u{33e}V\u{33e}W\u{33e}X\u{33e}Y\u{33e}Z\u{33e}0\u{33e}1\u{33e}2\u{33e}3\u{33e}4\u{33e}5\u{33e}6\u{33e}7\u{33e}8\u{33e}9\u{33e}",
        TEMPLATE,
    ));
    result.extend(set_helper(
        "a\u{34e}b\u{34e}c\u{34e}d\u{34e}e\u{34e}f\u{34e}g\u{34e}h\u{34e}i\u{34e}j\u{34e}k\u{34e}l\u{34e}m\u{34e}n\u{34e}o\u{34e}p\u{34e}q\u{34e}r\u{34e}s\u{34e}t\u{34e}u\u{34e}v\u{34e}w\u{34e}x\u{34e}y\u{34e}z\u{34e}A\u{34e}B\u{34e}C\u{34e}D\u{34e}E\u{34e}F\u{34e}G\u{34e}H\u{34e}I\u{34e}J\u{34e}K\u{34e}L\u{34e}M\u{34e}N\u{34e}O\u{34e}P\u{34e}Q\u{34e}R\u{34e}S\u{34e}T\u{34e}U\u{34e}V\u{34e}W\u{34e}X\u{34e}Y\u{34e}Z\u{34e}0\u{34e}1\u{34e}2\u{34e}3\u{34e}4\u{34e}5\u{34e}6\u{34e}7\u{34e}8\u{34e}9\u{34e}",
        TEMPLATE,
    ));
    result.extend(set_helper(
        "a\u{353}\u{33d}b\u{353}\u{33d}c\u{353}\u{33d}d\u{353}\u{33d}e\u{353}\u{33d}f\u{353}\u{33d}g\u{353}\u{33d}h\u{353}\u{33d}i\u{353}\u{33d}j\u{353}\u{33d}k\u{353}\u{33d}l\u{353}\u{33d}m\u{353}\u{33d}n\u{353}\u{33d}o\u{353}\u{33d}p\u{353}\u{33d}q\u{353}\u{33d}r\u{353}\u{33d}s\u{353}\u{33d}t\u{353}\u{33d}u\u{353}\u{33d}v\u{353}\u{33d}w\u{353}\u{33d}x\u{353}\u{33d}y\u{353}\u{33d}z\u{353}\u{33d}A\u{353}\u{33d}B\u{353}\u{33d}C\u{353}\u{33d}D\u{353}\u{33d}E\u{353}\u{33d}F\u{353}\u{33d}G\u{353}\u{33d}H\u{353}\u{33d}I\u{353}\u{33d}J\u{353}\u{33d}K\u{353}\u{33d}L\u{353}\u{33d}M\u{353}\u{33d}N\u{353}\u{33d}O\u{353}\u{33d}P\u{353}\u{33d}Q\u{353}\u{33d}R\u{353}\u{33d}S\u{353}\u{33d}T\u{353}\u{33d}U\u{353}\u{33d}V\u{353}\u{33d}W\u{353}\u{33d}X\u{353}\u{33d}Y\u{353}\u{33d}Z\u{353}\u{33d}0\u{353}\u{33d}1\u{353}\u{33d}2\u{353}\u{33d}3\u{353}\u{33d}4\u{353}\u{33d}5\u{353}\u{33d}6\u{353}\u{33d}7\u{353}\u{33d}8\u{353}\u{33d}9\u{353}\u{33d}",
        TEMPLATE,
    ));
    result.extend(set_helper(
        "\u{1d5ba}\u{1d5bb}\u{1d5bc}\u{1d5bd}\u{1d5be}\u{1d5bf}\u{1d5c0}\u{1d5c1}\u{1d5c2}\u{1d5c3}\u{1d5c4}\u{1d5c5}\u{1d5c6}\u{1d5c7}\u{1d5c8}\u{1d5c9}\u{1d5ca}\u{1d5cb}\u{1d5cc}\u{1d5cd}\u{1d5ce}\u{1d5cf}\u{1d5d0}\u{1d5d1}\u{1d5d2}\u{1d5d3}\u{1d5a0}\u{1d5a1}\u{1d5a2}\u{1d5a3}\u{1d5a4}\u{1d5a5}\u{1d5a6}\u{1d5a7}\u{1d5a8}\u{1d5a9}\u{1d5aa}\u{1d5ab}\u{1d5ac}\u{1d5ad}\u{1d5ae}\u{1d5af}\u{1d5b0}\u{1d5b1}\u{1d5b2}\u{1d5b3}\u{1d5b4}\u{1d5b5}\u{1d5b6}\u{1d5b7}\u{1d5b8}\u{1d5b9}0123456789",
        TEMPLATE,
    ));
    result.extend(set_helper(
        "\u{1d5ee}\u{1d5ef}\u{1d5f0}\u{1d5f1}\u{1d5f2}\u{1d5f3}\u{1d5f4}\u{1d5f5}\u{1d5f6}\u{1d5f7}\u{1d5f8}\u{1d5f9}\u{1d5fa}\u{1d5fb}\u{1d5fc}\u{1d5fd}\u{1d5fe}\u{1d5ff}\u{1d600}\u{1d601}\u{1d602}\u{1d603}\u{1d604}\u{1d605}\u{1d606}\u{1d607}\u{1d5d4}\u{1d5d5}\u{1d5d6}\u{1d5d7}\u{1d5d8}\u{1d5d9}\u{1d5da}\u{1d5db}\u{1d5dc}\u{1d5dd}\u{1d5de}\u{1d5df}\u{1d5e0}\u{1d5e1}\u{1d5e2}\u{1d5e3}\u{1d5e4}\u{1d5e5}\u{1d5e6}\u{1d5e7}\u{1d5e8}\u{1d5e9}\u{1d5ea}\u{1d5eb}\u{1d5ec}\u{1d5ed}\u{1d7ec}\u{1d7ed}\u{1d7ee}\u{1d7ef}\u{1d7f0}\u{1d7f1}\u{1d7f2}\u{1d7f3}\u{1d7f4}\u{1d7f5}",
        TEMPLATE,
    ));
    result.extend(set_helper(
        "\u{1d482}\u{1d483}\u{1d484}\u{1d485}\u{1d486}\u{1d487}\u{1d488}\u{1d489}\u{1d48a}\u{1d48b}\u{1d48c}\u{1d48d}\u{1d48e}\u{1d48f}\u{1d490}\u{1d491}\u{1d492}\u{1d493}\u{1d494}\u{1d495}\u{1d496}\u{1d497}\u{1d498}\u{1d499}\u{1d49a}\u{1d49b}\u{1d468}\u{1d469}\u{1d46a}\u{1d46b}\u{1d46c}\u{1d46d}\u{1d46e}\u{1d46f}\u{1d470}\u{1d471}\u{1d472}\u{1d473}\u{1d474}\u{1d475}\u{1d476}\u{1d477}\u{1d478}\u{1d479}\u{1d47a}\u{1d47b}\u{1d47c}\u{1d47d}\u{1d47e}\u{1d47f}\u{1d480}\u{1d481}0123456789",
        TEMPLATE,
    ));
    /*
    result.extend(set_helper("", TEMPLATE));
    result.extend(set_helper("", TEMPLATE));
    result.extend(set_helper("", TEMPLATE));
    */
    result
}