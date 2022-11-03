use box_drawing_table::{ansi_term::Style, Align, Border, Cell, Column, Row, Table};
use clap::Parser;

/// A simple utility to look up ASCII code
#[derive(Debug, Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(value_parser)]
    /// single character (e.g. "A"), ASCII number (e.g. "65", "0o101", "0x41"), or caret notation (e.g. "^@")
    query: Vec<String>,

    #[clap(long, default_value_t = false)]
    /// Disable pretty-printing
    simple: bool,

    #[clap(short, long, default_value_t = false)]
    /// List all signals
    list: bool,

    #[clap(short, long, default_value_t = false)]
    /// Look up ASCII digits
    digit: bool,
}

fn main() {
    let args = Args::parse();

    let mut rows = Vec::new();

    let queries: Vec<String> = if args.list {
        ('\x00'..='\x7f')
            .into_iter()
            .map(|ch| ch.to_string())
            .collect()
    } else {
        args.query
    };

    for q in queries {
        let ch;
        if q.starts_with("0x") || q.starts_with("0b") || q.starts_with("0o") {
            ch = number_to_char(&q);
        } else if q.len() == 2 && q.starts_with('^') {
            ch = caret_notation(&q);
        } else if !q.is_empty() && q.is_ascii() {
            let first_ch = q.chars().next().unwrap();
            if q.len() >= 2 || (!args.digit && first_ch.is_ascii_digit()) {
                ch = number_to_char(&q);
            } else {
                ch = Some(first_ch);
            }
        } else {
            ch = None;
        }

        let display;
        let bin;
        let oct;
        let hex;
        let dec;
        if let Some(ch) = ch {
            display = display_repr(ch);
            bin = format!("0b{:07b}", ch as u8);
            oct = format!("0o{:03o}", ch as u8);
            hex = format!("0x{:02X}", ch as u8);
            dec = format!("{}", ch as u8);
        } else {
            display = q.as_str();
            bin = "-".to_owned();
            oct = "-".to_owned();
            hex = "-".to_owned();
            dec = "-".to_owned();
        }

        if args.simple {
            println!("{} {} {} {} {}", display, hex, dec, oct, bin);
        } else {
            rows.push(Row::flexible_height(vec![
                Cell {
                    value: display.to_owned(),
                    align: Align::Left,
                    style: Style::default().bold(),
                },
                Cell::right(hex),
                Cell::right(dec),
                Cell::right(oct),
                Cell::right(bin),
            ]));
        }
    }

    if !args.simple && !rows.is_empty() {
        create_table(rows);
    }
}

fn create_table(rows: Vec<Row>) {
    let mut table = Table::new(vec![
        Border::Double.into(),
        Column::flexible_width(),
        Border::Single.into(),
        Column::flexible_width(),
        Border::Single.into(),
        Column::flexible_width(),
        Border::Single.into(),
        Column::flexible_width(),
        Border::Single.into(),
        Column::flexible_width(),
        Border::Double.into(),
    ]);

    table.append_row(Border::Double.into());
    table.append_row(Row::flexible_height(vec![
        Cell::left("char"),
        Cell::left("hex"),
        Cell::left("dec"),
        Cell::left("oct"),
        Cell::left("bin"),
    ]));
    table.append_row(Border::Single.into());

    for r in rows {
        table.append_row(r);
    }

    table.append_row(Border::Double.into());
    print!("{}", table);
}

fn number_to_char(number: &str) -> Option<char> {
    let parsed = if let Some(hex) = number.strip_prefix("0x") {
        u8::from_str_radix(hex, 16)
    } else if let Some(oct) = number.strip_prefix("0o") {
        u8::from_str_radix(oct, 8)
    } else if let Some(bin) = number.strip_prefix("0b") {
        u8::from_str_radix(bin, 2)
    } else {
        number.parse::<u8>()
    };

    match parsed {
        Ok(val @ 0..=0x7F) => Some(val as char),
        _ => None,
    }
}

fn caret_notation(text: &str) -> Option<char> {
    match text.strip_prefix('^').unwrap().chars().next().unwrap() {
        ch @ ('@'..='_' | '?') => Some(((ch as u8) ^ 0x40) as char),
        _ => None,
    }
}

fn display_repr(ascii_char: char) -> &'static str {
    match ascii_char {
        '\x00' => r#"NUL '\0' (null character)"#,
        '\x01' => r#"SOH (start of heading)"#,
        '\x02' => r#"STX (start of text)"#,
        '\x03' => r#"ETX (end of text)"#,
        '\x04' => r#"EOT (end of transmission)"#,
        '\x05' => r#"ENQ (enquiry)"#,
        '\x06' => r#"ACK (acknowledge)"#,
        '\x07' => r#"BEL '\a' (bell)"#,
        '\x08' => r#"BS  '\b' (backspace)"#,
        '\x09' => r#"HT  '\t' (horizontal tab)"#,
        '\x0A' => r#"LF  '\n' (new line)"#,
        '\x0B' => r#"VT  '\v' (vertical tab)"#,
        '\x0C' => r#"FF  '\f' (form feed)"#,
        '\x0D' => r#"CR  '\r' (carriage ret)"#,
        '\x0E' => r#"SO  (shift out)"#,
        '\x0F' => r#"SI  (shift in)"#,
        '\x10' => r#"DLE (data link escape)"#,
        '\x11' => r#"DC1 (device control 1)"#,
        '\x12' => r#"DC2 (device control 2)"#,
        '\x13' => r#"DC3 (device control 3)"#,
        '\x14' => r#"DC4 (device control 4)"#,
        '\x15' => r#"NAK (negative ack.)"#,
        '\x16' => r#"SYN (synchronous idle)"#,
        '\x17' => r#"ETB (end of trans. blk)"#,
        '\x18' => r#"CAN (cancel)"#,
        '\x19' => r#"EM  (end of medium)"#,
        '\x1A' => r#"SUB (substitute)"#,
        '\x1B' => r#"ESC (escape)"#,
        '\x1C' => r#"FS  (file separator)"#,
        '\x1D' => r#"GS  (group separator)"#,
        '\x1E' => r#"RS  (record separator)"#,
        '\x1F' => r#"US  (unit separator)"#,
        '\x20' => r#"SPACE"#,
        '\x21' => r#"!"#,
        '\x22' => r#"""#,
        '\x23' => r#"#"#,
        '\x24' => r#"$"#,
        '\x25' => r#"%"#,
        '\x26' => r#"&"#,
        '\x27' => r#"'"#,
        '\x28' => r#"("#,
        '\x29' => r#")"#,
        '\x2A' => r#"*"#,
        '\x2B' => r#"+"#,
        '\x2C' => r#"#"#,
        '\x2D' => r#"-"#,
        '\x2E' => r#"."#,
        '\x2F' => r#"/"#,
        '\x30' => r#"0"#,
        '\x31' => r#"1"#,
        '\x32' => r#"2"#,
        '\x33' => r#"3"#,
        '\x34' => r#"4"#,
        '\x35' => r#"5"#,
        '\x36' => r#"6"#,
        '\x37' => r#"7"#,
        '\x38' => r#"8"#,
        '\x39' => r#"9"#,
        '\x3A' => r#":"#,
        '\x3B' => r#";"#,
        '\x3C' => r#"<"#,
        '\x3D' => r#"="#,
        '\x3E' => r#">"#,
        '\x3F' => r#"?"#,
        '\x40' => r#"@"#,
        '\x41' => r#"A"#,
        '\x42' => r#"B"#,
        '\x43' => r#"C"#,
        '\x44' => r#"D"#,
        '\x45' => r#"E"#,
        '\x46' => r#"F"#,
        '\x47' => r#"G"#,
        '\x48' => r#"H"#,
        '\x49' => r#"I"#,
        '\x4A' => r#"J"#,
        '\x4B' => r#"K"#,
        '\x4C' => r#"L"#,
        '\x4D' => r#"M"#,
        '\x4E' => r#"N"#,
        '\x4F' => r#"O"#,
        '\x50' => r#"P"#,
        '\x51' => r#"Q"#,
        '\x52' => r#"R"#,
        '\x53' => r#"S"#,
        '\x54' => r#"T"#,
        '\x55' => r#"U"#,
        '\x56' => r#"V"#,
        '\x57' => r#"W"#,
        '\x58' => r#"X"#,
        '\x59' => r#"Y"#,
        '\x5A' => r#"Z"#,
        '\x5B' => r#"["#,
        '\x5C' => r#"\  '\\'"#,
        '\x5D' => r#"]"#,
        '\x5E' => r#"^"#,
        '\x5F' => r#"_"#,
        '\x60' => r#"`"#,
        '\x61' => r#"a"#,
        '\x62' => r#"b"#,
        '\x63' => r#"c"#,
        '\x64' => r#"d"#,
        '\x65' => r#"e"#,
        '\x66' => r#"f"#,
        '\x67' => r#"g"#,
        '\x68' => r#"h"#,
        '\x69' => r#"i"#,
        '\x6A' => r#"j"#,
        '\x6B' => r#"k"#,
        '\x6C' => r#"l"#,
        '\x6D' => r#"m"#,
        '\x6E' => r#"n"#,
        '\x6F' => r#"o"#,
        '\x70' => r#"p"#,
        '\x71' => r#"q"#,
        '\x72' => r#"r"#,
        '\x73' => r#"s"#,
        '\x74' => r#"t"#,
        '\x75' => r#"u"#,
        '\x76' => r#"v"#,
        '\x77' => r#"w"#,
        '\x78' => r#"x"#,
        '\x79' => r#"y"#,
        '\x7A' => r#"z"#,
        '\x7B' => r#"{"#,
        '\x7C' => r#"|"#,
        '\x7D' => r#"}"#,
        '\x7E' => r#"~"#,
        '\x7F' => r#"DEL"#,
        _ => unreachable!(),
    }
}
