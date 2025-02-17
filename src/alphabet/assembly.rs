//! Assembly wordlist ripped from: <https://gitlab.com/Aketzu/ruttu/-/blob/master/lib/codegen.rb>

use super::{Alphabet, Error, Result};

#[rustfmt::skip]
static ALPHABET_ASSEMBLY: &[&str] = &["nilviäinen","ikea","jakkara","kehveli","hitti","hermo","rytky","puntti","insinööri","banaani","kebab","höpsö","herra","hanska","teline","kainalo","kana","kenkä","tiltti","slaagi","barbi","kärpänen","ritari","piirakka","snobi","hylje","dippi","sissi","komppi","diiva","bensa","betoni","rakas","karkki","päivä","paavo","öky","kirurgi","itiö","mamma","ständi","hilavitkutin","vati","skutsi","hillo","rengas","marja","lukaali","naula","nakki","hessu","lepsu","sima","harakka","toka","koukku","paita","osasto","pirtelö","silmä","pizza","elämä","hieno","kasuaali","suklaa","häävi","kantapää","haukka","hissi","öljy","nokka","tasku","käärö","lusikka","naatti","osviitta","kihara","remppa","satula","örkki","peukalo","teini","pyörä","mikkihiiri","kreppi","velho","toukka","pannu","makro","mikro","pakki","sätky","hökkeli","nysse","puskuri","mättö","raffi","laastari","pelti","lenssu","hipsteri","ryppy","klonkku","sämpylä","porkkana","pora","hammas","valo","pöljä","parvi","larppi","puuhamaa","seppo","loraus","kiva","hassu","sipuli","höyry","kruiseri","mankeli","hätä","töppönen","nugetti","jyrsijä","poliisi","kynttilä","muikku","kuusi","kirkas","laukku","kärki","vihreä","sitkeä","viileä","raskas","paukku","yskä","nasta","selkeä","pieni","avoin","tahmea","sileä","hevonen","kissa","mäkäräinen","jäätävä","villi","ryyni","tabletti","torvi","pekoni","suora","lammas","keitto","panda","eläke","seepra","sale","skene","bonus","basso","akvaario","hoplop","platina","kevyt","kuula","perjantai","hikka","norppa","deitti","kelmi","saparo","paperi","aatami","hepuli","lepakko","sukka","kolmio","pleksi","jokeri","teekkari","kaappi","sinkki","mäti","tiivitaavi","siirappi","bounty","valkoinen","limusiini","möhköfantti","pimeä","pöllö","traktori","käntty","raakile","keisari","aito","bingo","keppi","nolo","hyppy","peili","laatta","korkea","karva","pörrö","planeetta","hiiri","liukas","harmaa","vitsi","nälkä","puhdas","kaista","jupiter","heebo","kultakala","freesi","örinä","mummo","femma","jätti","taksi","mato","sauhu","pähkinä","rekisteri","saippua","leidi","alien","suomu","ruuvi","tatti","teippi","kirppis","höylä","kymppi","käppyrä","prätkä","palatsi","asennus","päärynä","varasto","hytti","pulju","korkki","vankila","monsteri","sametti"];

pub struct Assembly {}

impl Alphabet for Assembly {
    fn count(&self) -> usize {
        ALPHABET_ASSEMBLY.len()
    }

    fn item(&self, n: usize) -> Result<String> {
        ALPHABET_ASSEMBLY
            .get(n)
            .map(std::string::ToString::to_string)
            .ok_or(Error::NonExistentCharacter(n))
    }
}
