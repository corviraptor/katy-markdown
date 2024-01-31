use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
#[logos()]
enum Token {
    #[regex(".+", priority = 0)]
    Text,

    #[regex(r"[\[].+[\]][\(].+[\)]")]
    Link,

    #[regex("[<].+[>]")]
    Url,

    #[regex("\\.")]
    Escape,

    #[regex("#+[ ].+")]
    Header,

    #[regex("=+|-+")]
    HeaderAbove,

    #[regex("[*].+[*]", priority = 1)]
    Italic,

    #[regex("[*]{2}.+[*]{2}", priority = 2)]
    Bold,

    #[regex("[*]{3}.+[*]{3}", priority = 3)]
    BoldItalic,

    #[regex(">+[ ].+")]
    BlockQuote,

    #[regex(">>+[ ].+")]
    NestedBlockQuote,

    #[regex("[0-9]+[.][ ].+")]
    OrderedListEntry,

    #[regex("[-][ ].+")]
    UnorderedListEntry,

    #[regex("[ ]{4}")]
    Indentation,

    #[regex("['].+[']")]
    Code,

    #[regex("[']{2}.+[']{2}")]
    NoCode,
}

#[cfg(test)]
mod tests {
    use crate::lexer::Token;
    use logos::Logos;

    fn check(input: &str, expect: Token) {
        let mut lexer = Token::lexer(input);

        assert_eq!(lexer.next(), Some(Ok(expect)));
        assert_eq!(lexer.slice(), input);
    }

    fn bad_check(input: &str, exclude: Token) {
        let mut lexer = Token::lexer(input);

        assert_ne!(lexer.next(), Some(Ok(exclude)));
        assert_eq!(lexer.slice(), input);
    }

    #[test]
    fn text() {
        check("lorem ipsum", Token::Text);
    }

    #[test]
    fn not_text() {
        bad_check("# header, not text", Token::Text);
    }

    #[test]
    fn link() {
        check("[adgnshj](sdfsdg)", Token::Link);
    }

    #[test]
    fn url() {
        check("<some//link.com>", Token::Url);
    }

    #[test]
    fn escape() {
        check("\\>", Token::Escape);
    }

    #[test]
    fn header1() {
        check("# header", Token::Header);
    }

    #[test]
    fn header_above_equals() {
        check("=====", Token::HeaderAbove);
    }

    #[test]
    fn header_above_minus() {
        check("-----", Token::HeaderAbove);
    }

    #[test]
    fn italic() {
        check("*italicized text*", Token::Italic);
    }

    #[test]
    fn bold() {
        check("**bold text**", Token::Bold);
        bad_check("**bold text**", Token::Italic);
    }

    #[test]
    fn bold_italic() {
        check("***bold italic text***", Token::BoldItalic);
        bad_check("***bold italic text***", Token::Italic);
        bad_check("***bold italic text***", Token::Bold);
    }

    #[test]
    fn block_quote() {
        check("> block quote", Token::BlockQuote);
    }

    #[test]
    fn nested_block_quote() {
        check(">> nested block quote", Token::NestedBlockQuote);
        bad_check(">> nested block quote", Token::BlockQuote);
    }

    #[test]
    fn ordered_list_entry() {
        check("24675. list entry", Token::OrderedListEntry);
    }

    #[test]
    fn unordered_list_entry() {
        check("- unordered list entry", Token::UnorderedListEntry);
    }

    #[test]
    fn indentation() {
        check("    ", Token::Indentation);
        bad_check(" ", Token::Indentation);
    }

    #[test]
    fn code() {
        check("'code'", Token::Code);
    }

    #[test]
    fn no_code() {
        check("''not code''", Token::NoCode);
        bad_check("''not code''", Token::Code);
    }
}
