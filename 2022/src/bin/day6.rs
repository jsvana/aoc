use std::collections::{HashSet, VecDeque};

struct Window {
    window_size: usize,
    letters: VecDeque<char>,
}

impl Window {
    fn new(window_size: usize) -> Self {
        Self {
            window_size,
            letters: VecDeque::new(),
        }
    }

    fn add_letter(&mut self, letter: char) {
        self.letters.push_back(letter);

        if self.letters.len() > self.window_size {
            self.letters.pop_front();
        }
    }

    fn all_unique(&self) -> bool {
        if self.letters.len() < self.window_size {
            return false;
        }

        let chars: HashSet<char> = self.letters.iter().map(|c| *c).collect();
        chars.len() == self.window_size
    }
}

fn start_index(window_size: usize, input: &str) -> Option<usize> {
    let mut window = Window::new(window_size);

    for (i, letter) in input.chars().enumerate() {
        window.add_letter(letter);
        if window.all_unique() {
            return Some(i + 1);
        }
    }

    None
}

fn main() {
    println!("{}", start_index(4, "lrgrvgvttzmtmtgglmgmccpclppvdvtvvllvggvrggbwwlzlmzzbppnvpnvppcjjzhjhthnhjnhhhndhnnnsbnnhzzvhhplplzlrzzgpzpwzpwwsvsjvjfvvphpspwswrswscwscwsscffspsbbjjcjwjrwwtgwwgswswwzbzddqnnpqnpnqppwzwszsnsjjpddhvvcbbhhpzzlpzlzppfpvvmcmvvflfttrltrlldlglbgblltqtffrtrwrzwwzmzwmwwlzzhttwzzwnnmrrcdrdjrjqjvqvvjzzgccrllhmhzzfnfwwtzwzwpwhhdjhhmzzbbvggzdzccbzbbpcpqccjbcbppsttdjdnjnppjjnmmszmzgzddtctvctcvttgtbbzqqggnmmdllvdvmvzzhfffzvfvtfvtvwwcnwnvwwbccggjcjqcqcbcrrppdqppdzpzqppttjhjdjqjppzgzjjpllwrrbttrvvzzbhzzqppndppwqppnrpnnttfwttsrrgprggmtmhmzhzczwzmwzwrwqwrrrdqrrvssnlngnppfqqgbgjjcttbgtbtmtctmcmcmgmsgsffhghqhbbvtbbtltmltlnlpnngcnggbngbnnzgzccgcpgcpcjppnnzjzdjdggzjzljjhnncgcjcscfctcvttvqtqmqjjsqjqpqfqhqmmlvvmppfrfjjngnnfllrlhhppcjcbjcctgcgtcgcvgvffqfcfpcpdpffrbrvbvnnphpqpfqqtnttmtgtlgtgzttnvvpwvwvcwcfwcwmccwlclqlflpflplwpllndlltlqtlqqmqnqmnqnvqvrrtddqndnrdnnpzprrqnnggvqvhvpvptvvvzwzrwwscsqqmcmttbgtgpptzptzzvszvzdvvtsscbbrpptssltssztszttlvlqljlgljlhhwvhwvvqhvqhqrhqqcnqccnbcbppbffzqfqsfspsqsjjrhjjchcmhmnhmmzjmjmfjmmsbsvvgcggtdgghchrrpnnrttnthtdtmmhmdmppmgpgllrwlrwlwvvlmlglppzttsvsbsnbncnjnffddzcddbzzbzgbghhhtltwtggljjggsdswwpmmfhfsfvfrrgmrgrfggvzzbnbttwqqdcdppqcqpcpqpjqpjpbbgjbgjjfwfwpfpgpzgzmzgzdzzpwzwqqjqfqllgrgjjfvvqnvncntngnhgnhgnnzvvbsbmbqmqwmqwwhbwhhsccvhcclncnqccnvnzvvdgvgnvnttmbbhccwgwttlwtwqttqcqmcqcdcmmjpmmjsjhhprrnnqddjwdjjvvhvgvssthhnfhnnntfthhtggthhbrbrjbbjfbjjrgrsrjrqqqfwflfclflnnnnvggfqgqzzbbvttfcfvcvsswvssnzndndvnvqqznnrjnnsmmptmppncpchcctwtbbgbqqjqtqsqfsfvfvnvmvzzpgzppdzdvdqdjdnjnttvvjbbzrzqrqwrqrbqrqsqpspjssnqnpqqnjndjjzmmvbbrqrccrffhwhggbttpnpphwhhmrrndrnddzqzzfbfwbwnwtwjjwjmjsjcjgcjjfcftcffvpvwwbffgzgnnlfffnddtdbdlbbcjbjmmfpfzfbbwbdwwfmfpmmfjfffvzvdvvhrvrcvcscjjpfjjnfnzzrtrpphtppzrppwhhphthltlllttghgwwvlwlflhldlzzmbzzjppnwppvlplqqbtbwwccswccqzzjhjbbhbnhnshnsslmmlqqjfjrjjmvvhpjqhzqffhsdsbwpjvgpvmbfqltrmpnwfcptpfmtjcpbzfldbhcmzchshrlbjgggrfjcqhzqqvbzsczmbgqmzqmltlrtlbnsfvmlhbbcqbbltjpdrpznrglshvgdnqwlhthghvtbffddcjwgdzfswzbppjtdhstcqqmvzmjrvfjbhmrznwqczdjjclnhbmtdvvzwttwnrlfqwpglpcppdwdcvfqpqfnmbvzvmqlmnlgnrsqdjvtsftgnlrtzsrcqhltmhzhpmzqqfqrjwhqfnqdtnshwgfhcpjrlplnqczdlntnhsczrgfhflsfbmftsbptflqbpwblrfnfzvqtpblftmscpzgdhhsbdbjhqclnptwtmhbbfglmvwnbqgvqhmmswwjpfwqjbvznmcpdzcvbzjmfqnwstvvtdnlvnpznnblfqzjjrjgnsbtmmbjzsvmgwddtnzcvhvtdrmjgtcrjzznrssscrzcfbfpgpnpppsqcqpccnbdjnwrbvhrcwgqncjrzbdhzqpfhqbnvbfrzmlfbfvtpggrtdswnvlsvpjsmfchhpbbszbnqqfrmhpqzdjhmhmnnmplbtrpgphvvqdfbcfnrfrbfbtshlmlfltjnbmggqntvhdnlvtcvlhmlrlfzfrqmlwqzrdghvdvtsqvmpdjrjclmlmgjqwzzldnzvfmwmrrnfghsvpcwjdtlnrhpjczwpgfbhpnmcbpthsndfflbjhnlwdbbmlttfqcmswvppslptgzbvfgppvpnhjccrpgrpwtngmmccjghhcwddmnglschnpjwqtrtsvggnpzvsqshfvcnhptphtlmqmpznfzwvbnhwpsfwvpflsdjcjgfzjprbbfzgdbmrjgwrgfdphghrhnpvfncrdzcwtthmqtdwlhjsdthqpzhbjpgggndtrmwvcsqhzrzwbhtqsqthvqncprvnpsrlpvlvcjrcflhbdhrfthlfnqbzbmvlvhmbjnbbjhpjwlfflfhpfwcwnnsljthvzwprqjmgpldlzjnjtjfjrgnrpzpvzfcsrprbjhwnmccwhppjrlnndjdjzqwpcwnvqwgmnwbrjqqvbplvsncnmdfrbhrrhghfllhrghzmlnltgdsqlgbvnlchgcbqlpqptdwmsjpqrprlhqmstzjfnzgbgvlfshwpcrgzcqmmfwvhwlsdvplmdgrtfrjwpfvhnjqdbwsfcqhchstlzfpdljgvcqsfcnqccnpmvsqbmwjtzwhpglhbjwzmvgqwjhvwfhnlbtsgljzmlldcpjwdcfppmnmphdmhpmdqwwtjtrdhlrjlvzgpbcgvwcmtclgpqwhtpbdtdbdscfzbrzmgjlbppcnvphphfnvzdzzlvfsvsgbgqcnlqwmtcrpwzcvnmnvtmcdsstvqpqzdpvtdsbvtwhdvgzqmzvwlspgbwmlnsrqdqnjwrllncflqsrzdqtjqvpnpjlqfwqtlqfqwlltszcwtpmjtldjgvmvptpmzqhwmlvjgnntpvcslmhlhdbjtjjnvsbnzwtdclwbzrvlqzjljtbdjvwgbwcltvnbhfvtgqrbmzbbfvldhmdvfvtlqglnblfmmpjqmzlnfjltsqdrgmlhbhngrrmhnjndggsdcfmtssmmtmzvhzrmwjsqjcvbsgqgtvdmvqlvlrvglrtlshfdmfrmljjggwjbcsztsjmjftcbbjwrmgqvssrvtgzcgthtlgsjspfmdgwptjdrbswqlpfsbtjlnhllmjpbfhgpfcprpdnqqvqdmcbqhbcqtstvnjdzwzwvhhwmcvcfbdwczpwpdhvnstjnbblbprzsccmwrzgfhmrpvzfztvsrtncdhzhptpfqtnqwvqtwdpvcqztgjgrcbdnvqftphtfbtqdhrffdrdmwsbpvhshzvjbvsrljnzddmmfgcnfdssvzdbsfwmfjsdnslbrqsqfwfqbqszjwvgcjbhrfjcnlfhzvhcbbbpmhhvjdtgrqlcchqtvnhlrgtssllvgcdjrlzlzfbrrrvwvvcgfjdlpscsqljmmwmvwnvrgdmgcbvmwmgprbfrbgptlfjbhrmczwrzwbdhdvtgvldnzfgcngdfhbgqsfzlrbwbvdflrrsrcwthjzvgmdtndgtsjtswfbdqvcjtsdvrvqpmmdlghsdbzplgpfnstplpjdvttgzmnhssftqcqjvdvvdrmltbrpsjvqwbljrqrtqldzbwzznsdstvmdzbrvvtgrrphmbrzwnjbmqvfhljcdlbzqtcbjsfqdqcr").unwrap());
    println!("{}", start_index(14, "lrgrvgvttzmtmtgglmgmccpclppvdvtvvllvggvrggbwwlzlmzzbppnvpnvppcjjzhjhthnhjnhhhndhnnnsbnnhzzvhhplplzlrzzgpzpwzpwwsvsjvjfvvphpspwswrswscwscwsscffspsbbjjcjwjrwwtgwwgswswwzbzddqnnpqnpnqppwzwszsnsjjpddhvvcbbhhpzzlpzlzppfpvvmcmvvflfttrltrlldlglbgblltqtffrtrwrzwwzmzwmwwlzzhttwzzwnnmrrcdrdjrjqjvqvvjzzgccrllhmhzzfnfwwtzwzwpwhhdjhhmzzbbvggzdzccbzbbpcpqccjbcbppsttdjdnjnppjjnmmszmzgzddtctvctcvttgtbbzqqggnmmdllvdvmvzzhfffzvfvtfvtvwwcnwnvwwbccggjcjqcqcbcrrppdqppdzpzqppttjhjdjqjppzgzjjpllwrrbttrvvzzbhzzqppndppwqppnrpnnttfwttsrrgprggmtmhmzhzczwzmwzwrwqwrrrdqrrvssnlngnppfqqgbgjjcttbgtbtmtctmcmcmgmsgsffhghqhbbvtbbtltmltlnlpnngcnggbngbnnzgzccgcpgcpcjppnnzjzdjdggzjzljjhnncgcjcscfctcvttvqtqmqjjsqjqpqfqhqmmlvvmppfrfjjngnnfllrlhhppcjcbjcctgcgtcgcvgvffqfcfpcpdpffrbrvbvnnphpqpfqqtnttmtgtlgtgzttnvvpwvwvcwcfwcwmccwlclqlflpflplwpllndlltlqtlqqmqnqmnqnvqvrrtddqndnrdnnpzprrqnnggvqvhvpvptvvvzwzrwwscsqqmcmttbgtgpptzptzzvszvzdvvtsscbbrpptssltssztszttlvlqljlgljlhhwvhwvvqhvqhqrhqqcnqccnbcbppbffzqfqsfspsqsjjrhjjchcmhmnhmmzjmjmfjmmsbsvvgcggtdgghchrrpnnrttnthtdtmmhmdmppmgpgllrwlrwlwvvlmlglppzttsvsbsnbncnjnffddzcddbzzbzgbghhhtltwtggljjggsdswwpmmfhfsfvfrrgmrgrfggvzzbnbttwqqdcdppqcqpcpqpjqpjpbbgjbgjjfwfwpfpgpzgzmzgzdzzpwzwqqjqfqllgrgjjfvvqnvncntngnhgnhgnnzvvbsbmbqmqwmqwwhbwhhsccvhcclncnqccnvnzvvdgvgnvnttmbbhccwgwttlwtwqttqcqmcqcdcmmjpmmjsjhhprrnnqddjwdjjvvhvgvssthhnfhnnntfthhtggthhbrbrjbbjfbjjrgrsrjrqqqfwflfclflnnnnvggfqgqzzbbvttfcfvcvsswvssnzndndvnvqqznnrjnnsmmptmppncpchcctwtbbgbqqjqtqsqfsfvfvnvmvzzpgzppdzdvdqdjdnjnttvvjbbzrzqrqwrqrbqrqsqpspjssnqnpqqnjndjjzmmvbbrqrccrffhwhggbttpnpphwhhmrrndrnddzqzzfbfwbwnwtwjjwjmjsjcjgcjjfcftcffvpvwwbffgzgnnlfffnddtdbdlbbcjbjmmfpfzfbbwbdwwfmfpmmfjfffvzvdvvhrvrcvcscjjpfjjnfnzzrtrpphtppzrppwhhphthltlllttghgwwvlwlflhldlzzmbzzjppnwppvlplqqbtbwwccswccqzzjhjbbhbnhnshnsslmmlqqjfjrjjmvvhpjqhzqffhsdsbwpjvgpvmbfqltrmpnwfcptpfmtjcpbzfldbhcmzchshrlbjgggrfjcqhzqqvbzsczmbgqmzqmltlrtlbnsfvmlhbbcqbbltjpdrpznrglshvgdnqwlhthghvtbffddcjwgdzfswzbppjtdhstcqqmvzmjrvfjbhmrznwqczdjjclnhbmtdvvzwttwnrlfqwpglpcppdwdcvfqpqfnmbvzvmqlmnlgnrsqdjvtsftgnlrtzsrcqhltmhzhpmzqqfqrjwhqfnqdtnshwgfhcpjrlplnqczdlntnhsczrgfhflsfbmftsbptflqbpwblrfnfzvqtpblftmscpzgdhhsbdbjhqclnptwtmhbbfglmvwnbqgvqhmmswwjpfwqjbvznmcpdzcvbzjmfqnwstvvtdnlvnpznnblfqzjjrjgnsbtmmbjzsvmgwddtnzcvhvtdrmjgtcrjzznrssscrzcfbfpgpnpppsqcqpccnbdjnwrbvhrcwgqncjrzbdhzqpfhqbnvbfrzmlfbfvtpggrtdswnvlsvpjsmfchhpbbszbnqqfrmhpqzdjhmhmnnmplbtrpgphvvqdfbcfnrfrbfbtshlmlfltjnbmggqntvhdnlvtcvlhmlrlfzfrqmlwqzrdghvdvtsqvmpdjrjclmlmgjqwzzldnzvfmwmrrnfghsvpcwjdtlnrhpjczwpgfbhpnmcbpthsndfflbjhnlwdbbmlttfqcmswvppslptgzbvfgppvpnhjccrpgrpwtngmmccjghhcwddmnglschnpjwqtrtsvggnpzvsqshfvcnhptphtlmqmpznfzwvbnhwpsfwvpflsdjcjgfzjprbbfzgdbmrjgwrgfdphghrhnpvfncrdzcwtthmqtdwlhjsdthqpzhbjpgggndtrmwvcsqhzrzwbhtqsqthvqncprvnpsrlpvlvcjrcflhbdhrfthlfnqbzbmvlvhmbjnbbjhpjwlfflfhpfwcwnnsljthvzwprqjmgpldlzjnjtjfjrgnrpzpvzfcsrprbjhwnmccwhppjrlnndjdjzqwpcwnvqwgmnwbrjqqvbplvsncnmdfrbhrrhghfllhrghzmlnltgdsqlgbvnlchgcbqlpqptdwmsjpqrprlhqmstzjfnzgbgvlfshwpcrgzcqmmfwvhwlsdvplmdgrtfrjwpfvhnjqdbwsfcqhchstlzfpdljgvcqsfcnqccnpmvsqbmwjtzwhpglhbjwzmvgqwjhvwfhnlbtsgljzmlldcpjwdcfppmnmphdmhpmdqwwtjtrdhlrjlvzgpbcgvwcmtclgpqwhtpbdtdbdscfzbrzmgjlbppcnvphphfnvzdzzlvfsvsgbgqcnlqwmtcrpwzcvnmnvtmcdsstvqpqzdpvtdsbvtwhdvgzqmzvwlspgbwmlnsrqdqnjwrllncflqsrzdqtjqvpnpjlqfwqtlqfqwlltszcwtpmjtldjgvmvptpmzqhwmlvjgnntpvcslmhlhdbjtjjnvsbnzwtdclwbzrvlqzjljtbdjvwgbwcltvnbhfvtgqrbmzbbfvldhmdvfvtlqglnblfmmpjqmzlnfjltsqdrgmlhbhngrrmhnjndggsdcfmtssmmtmzvhzrmwjsqjcvbsgqgtvdmvqlvlrvglrtlshfdmfrmljjggwjbcsztsjmjftcbbjwrmgqvssrvtgzcgthtlgsjspfmdgwptjdrbswqlpfsbtjlnhllmjpbfhgpfcprpdnqqvqdmcbqhbcqtstvnjdzwzwvhhwmcvcfbdwczpwpdhvnstjnbblbprzsccmwrzgfhmrpvzfztvsrtncdhzhptpfqtnqwvqtwdpvcqztgjgrcbdnvqftphtfbtqdhrffdrdmwsbpvhshzvjbvsrljnzddmmfgcnfdssvzdbsfwmfjsdnslbrqsqfwfqbqszjwvgcjbhrfjcnlfhzvhcbbbpmhhvjdtgrqlcchqtvnhlrgtssllvgcdjrlzlzfbrrrvwvvcgfjdlpscsqljmmwmvwnvrgdmgcbvmwmgprbfrbgptlfjbhrmczwrzwbdhdvtgvldnzfgcngdfhbgqsfzlrbwbvdflrrsrcwthjzvgmdtndgtsjtswfbdqvcjtsdvrvqpmmdlghsdbzplgpfnstplpjdvttgzmnhssftqcqjvdvvdrmltbrpsjvqwbljrqrtqldzbwzznsdstvmdzbrvvtgrrphmbrzwnjbmqvfhljcdlbzqtcbjsfqdqcr").unwrap());
}