use std::collections::HashSet;

fn is_unique(candidate: &[char], window_size: usize) -> bool {
    candidate
        .iter()
        .fold(HashSet::new(), |mut acc, &n| {
            acc.insert(n);
            acc
        })
        .len()
        == window_size
}

fn day06(signal: &str, window_size: usize) -> usize {
    signal
        .chars()
        .collect::<Vec<char>>()
        .windows(window_size)
        .take_while(|&w| !is_unique(w, window_size))
        .collect::<Vec<_>>()
        .len()
        + window_size
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_delimiter_position_a() {
        let result = day06("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 4);
        assert_eq!(result, 7);
    }

    #[test]
    fn find_delimiter_position_b() {
        let result = day06("bvwbjplbgvbhsrlpgdmjqwftvncz", 4);
        assert_eq!(result, 5);
    }

    #[test]
    fn find_delimiter_position_c() {
        let result = day06("nppdvjthqldpwncqszvftbrmjlhg", 4);
        assert_eq!(result, 6);
    }

    #[test]
    fn find_delimiter_position_d() {
        let result = day06("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 4);
        assert_eq!(result, 10);
    }

    #[test]
    fn find_delimiter_position_e() {
        let result = day06("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 4);
        assert_eq!(result, 11);
    }

    #[test]
    fn find_delimiter_position_part_a() {
        let result = day06("pjbjvjtjljplppjssvtvwtwptptztltbtrrjgrjrzrqrjrbrhbrhrlllbpbdbbzqqgsqshqssjjbsjbsbmbhhmchhrqrcqqbwbqwwqrrznnsbswwwdjwdwmmsvszzlbbgddbgdgfgttzjzrjzrjzzvrvqqgpqggtbgtgvvrhvhtvtjjbpjjfjhjbhbddbjjjmzmtmgmpgmmmljlnljjmpmbpmbmrrlhhlppdgdfgfbbqlbbtffjjgvvnpvpbpttqmmnhhgfgrrwhrrbnbznzccmbmvmmzszsvsbvbccsrslsjsbbtdtwtvttvpvzzvbzzwczwcczhzhwhjhjghgppgpgttdwdhwhphnppqmpqqhthcchdhmhnnbcnbcbbggbfblljttwsswspsggpjjzszcscsmcmdmnngzzhrzrbrhbhzbzvzgvgffnlnljlrrhchhsvhssmpmccncdnccgdgbglgtlggnllsvvpfvpfpbffsgglddjrrzzphhptprtppwrwffzllrbrmrpmpdmpdmpplpspcphcphhgmmqnmmvnmmdvmvsmmqjmmlmlbmlbmlltlptphpnncscbscbcggwhhgjgqjqmjmdjmdjdrrvgvvzlldgdnnvttmmpffdjjvvchcwwbhwbwzzlmlccrttcntccpcgccpgcpggdrrbtrthhlrrbqrrpspdsdldbldblbzbpbgggtngtgqqtwwdjjmmcrmcrcvclcddhllpzpdzzmccrtccfvffccfhccpscctbbbzqzvvllgwlgwgvwwjswjswsvvwhhvjvsjvjmmjhjrhrmrvrnrccmnmzzmdzzbtbvbqvvgzgcgvvvvlltvllbfbqqrppwhpwpffzddzdzwdzzrggmhmfhmhbbzjzsjzzhhjdhhdnndsnssnfffbmffwhwrhhmmbnnbrnrbrrtqtztnzzzzblbhbdhbbfmfqfmmsgszzvfzfmfwfnwfwggwngnqnwqnwqwvwqvwvdvrvjjfnjnmnfmmwzzltztjjqnnnmlmzlmmrcmclcqqhrhdhccdfdvfvccvtcvvdmmtmccwjjcbcrrjmrjrdrffgwwvbvlvsspwpsspzsschssmqsqmqmlqmmqgqfqcqjcjtthjttlddfvvwwjvvtpvvfsvffqnffznzqzszgzmmjttwztwzzhqqccqsqmqnmqqjhqhzhwhvhdddsndsdfftvffwlwnnmmdpmmnhhrqrrclcdlcddhcdcppgrprnpnptnpphgpbqfngdgzvgndwcgrwcsfmhzsvddhzbgjmvvdjjzswvgnpmvgdpwsgbgjzjpsrfdzdzjzzrpplbhsmgddqzjbdzdzltqqwqjzqvwfmcdppbdbprrwzhmnrqclzrnmdjnfbwmvdrwtpwvgscrqgpndqnzbjsbljcbthbpgdjdcdwfhpvjnbsfjdlrjldvvmtfdslrhlfwmvclqrljrqmmjgqfwmfgwdjzzptgcthvtgdswsqjrqvnzmtqldjjcqnfhtvbwhjqlvpptfwjrdpcwvzddgcjzvqbhtsnnnjqqmqlbgvqmvjhvvpbzcbdmhgmcjbfcccsvlzjztvjzrrlhtgwccdcgcptqlmdhmdhvqzfntbjqtsmvqgwsltqntgszllntrljfgfsghtbbcqrdgwqphmbqtzmjqccrgvqpqpchzjstdmmtvntwjqsbcqjgnhzlllcfbpgtgrhwwhqqdlgrlsbzbmchvjnsgpdnmqvtgwqjpgflqgfngjfcfwqzmvvgzmmhbgfnbzvzclwclqdcccgbrrzpwdtprgsvhbgsnbntgrvnzhrnzfzdmnlbnrbqvmjbwpgvjlhbcvsrlqmcsnlrvtfdwtvcbmlndgbctsnmtctjszlpddqmzbtphhhfznwbdfsgppmdmczmhmmrzpllfqqbgvlsrscpfgznhdhgrnnnvrchgvzlqbgvcfghjvlvrvpclfcshbmvglcfrjbzrbcjmjjrfgqthwfrqbgtjldmbnfwllspmwrvstvrltvrlvrtjvprgtgzjlrgclvjhqpfcwcdbdtzwdsdfrtsvtvgjmsszdfqlmhqqlzswjfndswlmhcrhglphvpnfjpbmggbwlmzjchpnrllbjpmgmzjjrqpqgsbrszqhdljcpnclvrvbntgtcdcmhtdhgslhpvdjpvrszfrjhsbvcvtfwvvgczprnpbhmnnlmctbtqdjspgvhvnhwvspwgnjvzllwlnjhfjwsslppmjbfbdnthcpzbcmnnbvhctgwgdvhvlrbltmdnlfcsncqgrmjprshdvvtvcccgzhszcjgczhmhtvmccjpchqshhdzjjhbfpzqdjszdhdvlmgctmwcjprwlsqbcqhlcrfdgnqzfdfvqslmqlppbsvbmjmfbrtdmpmtqvwvppcfzddjzhhzlrrnnhbrlhmzlqwftprfvctnfhfhfzrnrvggfqmqwcwszhtbfjncprgwcqbjlvtnrprlwwghswvprjmsbmqvwnnfggprndvshfvvwtrqjpwghgbppftgzhqjslfzhngwfsjnmjzdsjqgpmglwnjlcgmczgvndszrszcpnzqpbzjmgrfsbjlghwrbqsqdhlnhzsvsgbqhbcdffjlgrbdrrjvclzqpftlhdvvcrvlgvlpnjcqdcbdjtlwnldjhhhzrwsqlhlsztwrznfsszptlrhjmqwmnfwjtjwmmmtvwzhpmjgzgsscbddgvvhpcnhvnggzhbzvvjlmdftpbcsvtsttrvgghptmmcdclbdvmnsdntthfbdznbclwccnlzcvdwzrqgddjszvbdqcjppzrtpnrhfcvvwpjqczgqwzzzvzmlnlzqszvtllftthgwgftjzsndpzzcnqpcvmsdvvfrjdsvfclsqqhsjrrctfvdrlhfmhprjggdcmqrrbqtwnrllhhztvjgmzqszbvqfwsgllvhsvfrjffvdscwjzqlzlwdpgthddpgzjfdbdqpsnntwpslvsdpqfnsgcllszcjwvtqhwhpfrlfdgwrfmgfpjmvnstrmtfcvgwlqdfqvntltqtrmjjtwcthvwntqgvncssplnmvlnstlcphvlcmvjnstwldtntchcbmzmlzhgjfbrdlgzvqpgcndmfdnmcnwhmpdnpqstfddddcrpgrpfwfbzjqtnzwwqpzrqpmrjpfznrndfgwhtlvrcrphqfjzjbttwhgnsngqwvnsbvcqtjlmhvmnmnnmjcmlpnpgmrqsbmgljvsfqvrlljqzmzqqbgpvcrwdjmgsglssjswmnvtshhfqjhqmfmvcjwfpwsppgtrqsbhhcdljnjphnjszqpvdplbwzpwmmpwfhmhngtllzqvpmgdctmfqwwqjszssmjhwnrjdtmmvpdnwlqtcbpfcmwtbjmmsmmdpqgzdhsblgjmjbpzgqvqhnggtwmhztbbhlflllgwblncjjsngdgvsfdmsbsvlpnjjzqqbzhsqclmjnnmmwlpvtgwqmcgmrqdwdddlgbvhntbztbjnqhdlggnzwsdtdzprgddhtcttjrcpszgchtfwqjsdlnbntfwqpzpfsqrqjhthmcfszwtwcqwbvfzdnrrpmzjdrhsgmhfbsldvcrjdwvpqpszzlvbptljgvccqsdhhnztjpghbvhfptgplqdvldjzfthpspwvgljwnnndwrqzbrstnqbvrrcghssnrpvtrhmvcmbngwndzfswmgjwnnzqdcjhpthcgvthsnwqzrnzrvdjmctchhsbnrtvctzqfpcjhzmhnfjlqftbjztfbcppgmwvrzzrvlcpnpwwpvtcpdplrcfpgfqjtlfjtphhpcltwqcbqbznbtjrtdrpgtvzmgsclhpptrssqqbctdrftqzmwjmrmjtgmjmsnbnspjvcqpqnmgzgjrmfhghvsfsdqnbdjsbcpczsdswdcvhfzlgpzbtmztcnbpcvjnlcdmmlbtwzsfqtfnlrwjtwmgslcgptgbdsfwdhppvfwbbgdfdqtrbncbznmqtchzsdzlhlhjnnbpdvnnfjrdfbdqmvcb", 4);
        assert_eq!(result, 1275);
    }

    #[test]
    fn find_delimiter_position_part_b() {
        let result = day06("pjbjvjtjljplppjssvtvwtwptptztltbtrrjgrjrzrqrjrbrhbrhrlllbpbdbbzqqgsqshqssjjbsjbsbmbhhmchhrqrcqqbwbqwwqrrznnsbswwwdjwdwmmsvszzlbbgddbgdgfgttzjzrjzrjzzvrvqqgpqggtbgtgvvrhvhtvtjjbpjjfjhjbhbddbjjjmzmtmgmpgmmmljlnljjmpmbpmbmrrlhhlppdgdfgfbbqlbbtffjjgvvnpvpbpttqmmnhhgfgrrwhrrbnbznzccmbmvmmzszsvsbvbccsrslsjsbbtdtwtvttvpvzzvbzzwczwcczhzhwhjhjghgppgpgttdwdhwhphnppqmpqqhthcchdhmhnnbcnbcbbggbfblljttwsswspsggpjjzszcscsmcmdmnngzzhrzrbrhbhzbzvzgvgffnlnljlrrhchhsvhssmpmccncdnccgdgbglgtlggnllsvvpfvpfpbffsgglddjrrzzphhptprtppwrwffzllrbrmrpmpdmpdmpplpspcphcphhgmmqnmmvnmmdvmvsmmqjmmlmlbmlbmlltlptphpnncscbscbcggwhhgjgqjqmjmdjmdjdrrvgvvzlldgdnnvttmmpffdjjvvchcwwbhwbwzzlmlccrttcntccpcgccpgcpggdrrbtrthhlrrbqrrpspdsdldbldblbzbpbgggtngtgqqtwwdjjmmcrmcrcvclcddhllpzpdzzmccrtccfvffccfhccpscctbbbzqzvvllgwlgwgvwwjswjswsvvwhhvjvsjvjmmjhjrhrmrvrnrccmnmzzmdzzbtbvbqvvgzgcgvvvvlltvllbfbqqrppwhpwpffzddzdzwdzzrggmhmfhmhbbzjzsjzzhhjdhhdnndsnssnfffbmffwhwrhhmmbnnbrnrbrrtqtztnzzzzblbhbdhbbfmfqfmmsgszzvfzfmfwfnwfwggwngnqnwqnwqwvwqvwvdvrvjjfnjnmnfmmwzzltztjjqnnnmlmzlmmrcmclcqqhrhdhccdfdvfvccvtcvvdmmtmccwjjcbcrrjmrjrdrffgwwvbvlvsspwpsspzsschssmqsqmqmlqmmqgqfqcqjcjtthjttlddfvvwwjvvtpvvfsvffqnffznzqzszgzmmjttwztwzzhqqccqsqmqnmqqjhqhzhwhvhdddsndsdfftvffwlwnnmmdpmmnhhrqrrclcdlcddhcdcppgrprnpnptnpphgpbqfngdgzvgndwcgrwcsfmhzsvddhzbgjmvvdjjzswvgnpmvgdpwsgbgjzjpsrfdzdzjzzrpplbhsmgddqzjbdzdzltqqwqjzqvwfmcdppbdbprrwzhmnrqclzrnmdjnfbwmvdrwtpwvgscrqgpndqnzbjsbljcbthbpgdjdcdwfhpvjnbsfjdlrjldvvmtfdslrhlfwmvclqrljrqmmjgqfwmfgwdjzzptgcthvtgdswsqjrqvnzmtqldjjcqnfhtvbwhjqlvpptfwjrdpcwvzddgcjzvqbhtsnnnjqqmqlbgvqmvjhvvpbzcbdmhgmcjbfcccsvlzjztvjzrrlhtgwccdcgcptqlmdhmdhvqzfntbjqtsmvqgwsltqntgszllntrljfgfsghtbbcqrdgwqphmbqtzmjqccrgvqpqpchzjstdmmtvntwjqsbcqjgnhzlllcfbpgtgrhwwhqqdlgrlsbzbmchvjnsgpdnmqvtgwqjpgflqgfngjfcfwqzmvvgzmmhbgfnbzvzclwclqdcccgbrrzpwdtprgsvhbgsnbntgrvnzhrnzfzdmnlbnrbqvmjbwpgvjlhbcvsrlqmcsnlrvtfdwtvcbmlndgbctsnmtctjszlpddqmzbtphhhfznwbdfsgppmdmczmhmmrzpllfqqbgvlsrscpfgznhdhgrnnnvrchgvzlqbgvcfghjvlvrvpclfcshbmvglcfrjbzrbcjmjjrfgqthwfrqbgtjldmbnfwllspmwrvstvrltvrlvrtjvprgtgzjlrgclvjhqpfcwcdbdtzwdsdfrtsvtvgjmsszdfqlmhqqlzswjfndswlmhcrhglphvpnfjpbmggbwlmzjchpnrllbjpmgmzjjrqpqgsbrszqhdljcpnclvrvbntgtcdcmhtdhgslhpvdjpvrszfrjhsbvcvtfwvvgczprnpbhmnnlmctbtqdjspgvhvnhwvspwgnjvzllwlnjhfjwsslppmjbfbdnthcpzbcmnnbvhctgwgdvhvlrbltmdnlfcsncqgrmjprshdvvtvcccgzhszcjgczhmhtvmccjpchqshhdzjjhbfpzqdjszdhdvlmgctmwcjprwlsqbcqhlcrfdgnqzfdfvqslmqlppbsvbmjmfbrtdmpmtqvwvppcfzddjzhhzlrrnnhbrlhmzlqwftprfvctnfhfhfzrnrvggfqmqwcwszhtbfjncprgwcqbjlvtnrprlwwghswvprjmsbmqvwnnfggprndvshfvvwtrqjpwghgbppftgzhqjslfzhngwfsjnmjzdsjqgpmglwnjlcgmczgvndszrszcpnzqpbzjmgrfsbjlghwrbqsqdhlnhzsvsgbqhbcdffjlgrbdrrjvclzqpftlhdvvcrvlgvlpnjcqdcbdjtlwnldjhhhzrwsqlhlsztwrznfsszptlrhjmqwmnfwjtjwmmmtvwzhpmjgzgsscbddgvvhpcnhvnggzhbzvvjlmdftpbcsvtsttrvgghptmmcdclbdvmnsdntthfbdznbclwccnlzcvdwzrqgddjszvbdqcjppzrtpnrhfcvvwpjqczgqwzzzvzmlnlzqszvtllftthgwgftjzsndpzzcnqpcvmsdvvfrjdsvfclsqqhsjrrctfvdrlhfmhprjggdcmqrrbqtwnrllhhztvjgmzqszbvqfwsgllvhsvfrjffvdscwjzqlzlwdpgthddpgzjfdbdqpsnntwpslvsdpqfnsgcllszcjwvtqhwhpfrlfdgwrfmgfpjmvnstrmtfcvgwlqdfqvntltqtrmjjtwcthvwntqgvncssplnmvlnstlcphvlcmvjnstwldtntchcbmzmlzhgjfbrdlgzvqpgcndmfdnmcnwhmpdnpqstfddddcrpgrpfwfbzjqtnzwwqpzrqpmrjpfznrndfgwhtlvrcrphqfjzjbttwhgnsngqwvnsbvcqtjlmhvmnmnnmjcmlpnpgmrqsbmgljvsfqvrlljqzmzqqbgpvcrwdjmgsglssjswmnvtshhfqjhqmfmvcjwfpwsppgtrqsbhhcdljnjphnjszqpvdplbwzpwmmpwfhmhngtllzqvpmgdctmfqwwqjszssmjhwnrjdtmmvpdnwlqtcbpfcmwtbjmmsmmdpqgzdhsblgjmjbpzgqvqhnggtwmhztbbhlflllgwblncjjsngdgvsfdmsbsvlpnjjzqqbzhsqclmjnnmmwlpvtgwqmcgmrqdwdddlgbvhntbztbjnqhdlggnzwsdtdzprgddhtcttjrcpszgchtfwqjsdlnbntfwqpzpfsqrqjhthmcfszwtwcqwbvfzdnrrpmzjdrhsgmhfbsldvcrjdwvpqpszzlvbptljgvccqsdhhnztjpghbvhfptgplqdvldjzfthpspwvgljwnnndwrqzbrstnqbvrrcghssnrpvtrhmvcmbngwndzfswmgjwnnzqdcjhpthcgvthsnwqzrnzrvdjmctchhsbnrtvctzqfpcjhzmhnfjlqftbjztfbcppgmwvrzzrvlcpnpwwpvtcpdplrcfpgfqjtlfjtphhpcltwqcbqbznbtjrtdrpgtvzmgsclhpptrssqqbctdrftqzmwjmrmjtgmjmsnbnspjvcqpqnmgzgjrmfhghvsfsdqnbdjsbcpczsdswdcvhfzlgpzbtmztcnbpcvjnlcdmmlbtwzsfqtfnlrwjtwmgslcgptgbdsfwdhppvfwbbgdfdqtrbncbznmqtchzsdzlhlhjnnbpdvnnfjrdfbdqmvcb", 14);
        assert_eq!(result, 3605);
    }
}
