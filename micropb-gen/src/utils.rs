use std::sync::LazyLock;

use proc_macro2::TokenStream;
use regex::Regex;
use syn::Lifetime;

pub(crate) fn unescape_c_escape_string(s: &str) -> Vec<u8> {
    let src = s.as_bytes();
    let len = src.len();
    let mut dst = Vec::new();

    let mut p = 0;

    while p < len {
        if src[p] != b'\\' {
            dst.push(src[p]);
            p += 1;
        } else {
            p += 1;
            if p == len {
                panic!("invalid c-escaped default binary value ({s}): ends with '\'",)
            }
            match src[p] {
                b'a' => {
                    dst.push(0x07);
                    p += 1;
                }
                b'b' => {
                    dst.push(0x08);
                    p += 1;
                }
                b'f' => {
                    dst.push(0x0C);
                    p += 1;
                }
                b'n' => {
                    dst.push(0x0A);
                    p += 1;
                }
                b'r' => {
                    dst.push(0x0D);
                    p += 1;
                }
                b't' => {
                    dst.push(0x09);
                    p += 1;
                }
                b'v' => {
                    dst.push(0x0B);
                    p += 1;
                }
                b'\\' => {
                    dst.push(0x5C);
                    p += 1;
                }
                b'?' => {
                    dst.push(0x3F);
                    p += 1;
                }
                b'\'' => {
                    dst.push(0x27);
                    p += 1;
                }
                b'"' => {
                    dst.push(0x22);
                    p += 1;
                }
                b'0'..=b'7' => {
                    let mut octal = 0;
                    for _ in 0..3 {
                        if p < len && src[p] >= b'0' && src[p] <= b'7' {
                            octal = octal * 8 + (src[p] - b'0');
                            p += 1;
                        } else {
                            break;
                        }
                    }
                    dst.push(octal);
                }
                b'x' | b'X' => {
                    if p + 3 > len {
                        panic!("invalid c-escaped default binary value ({s}): incomplete hex value",)
                    }
                    match u8::from_str_radix(&s[p + 1..p + 3], 16) {
                        Ok(b) => dst.push(b),
                        _ => panic!(
                            "invalid c-escaped default binary value ({}): invalid hex value",
                            &s[p..p + 2]
                        ),
                    }
                    p += 3;
                }
                _ => panic!("invalid c-escaped default binary value ({s}): invalid escape",),
            }
        }
    }
    dst
}

pub(crate) fn path_suffix(path: &str) -> &str {
    path.rsplit_once('.')
        .map(|(_, suffix)| suffix)
        .unwrap_or(path)
}

/// Ignore static and _ lifetimes
fn usable_lifetime(lt: &Lifetime) -> bool {
    lt.ident != "static" && lt.ident != "_"
}

/// Find the first lifetime embedded in a type
pub(crate) fn find_lifetime_from_type(ty: &syn::Type) -> Option<&Lifetime> {
    match ty {
        syn::Type::Array(tarr) => find_lifetime_from_type(&tarr.elem),
        syn::Type::Group(t) => find_lifetime_from_type(&t.elem),
        syn::Type::Paren(t) => find_lifetime_from_type(&t.elem),
        syn::Type::Reference(tref) => tref
            .lifetime
            .as_ref()
            .filter(|lt| usable_lifetime(lt))
            .or_else(|| find_lifetime_from_type(&tref.elem)),
        syn::Type::Slice(tslice) => find_lifetime_from_type(&tslice.elem),
        syn::Type::Tuple(tuple) => tuple.elems.iter().find_map(find_lifetime_from_type),
        syn::Type::Path(tpath) => find_lifetime_from_path(&tpath.path),
        _ => None,
    }
}

/// Find the first lifetime embedded in a type path
pub(crate) fn find_lifetime_from_path(tpath: &syn::Path) -> Option<&Lifetime> {
    if let syn::PathArguments::AngleBracketed(args) =
        &tpath.segments.last().expect("empty type path").arguments
    {
        args.args.iter().find_map(|arg| match arg {
            syn::GenericArgument::Lifetime(lt) => usable_lifetime(lt).then_some(lt),
            syn::GenericArgument::Type(ty) => find_lifetime_from_type(ty),
            _ => None,
        })
    } else {
        None
    }
}

pub(crate) fn find_lifetime_from_str(s: &str) -> Option<Lifetime> {
    static RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"'[_a-zA-Z][_a-zA-Z0-9]*").unwrap());
    // The regex should match a valid lifetime, so the parse should always succeed
    RE.find_iter(s)
        .map(|m| syn::parse_str(m.as_str()).unwrap())
        .find(usable_lifetime)
}

pub(crate) trait TryIntoTokens<E> {
    fn try_into_tokens(self) -> Result<TokenStream, E>;
}
impl<I, E> TryIntoTokens<E> for I
where
    I: IntoIterator<Item = Result<TokenStream, E>>,
{
    fn try_into_tokens(self) -> Result<TokenStream, E> {
        let mut tokens = TokenStream::new();
        for res in self.into_iter() {
            tokens.extend(res?);
        }
        Ok(tokens)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn suffix() {
        assert_eq!(path_suffix("test"), "test");
        assert_eq!(path_suffix("a.b.c"), "c");
    }

    #[test]
    fn unescape_c_string() {
        assert_eq!(
            &b"hello world"[..],
            &unescape_c_escape_string("hello world")[..]
        );

        assert_eq!(&b"\0"[..], &unescape_c_escape_string(r#"\0"#)[..]);

        assert_eq!(
            &[0o012, 0o156],
            &unescape_c_escape_string(r#"\012\156"#)[..]
        );
        assert_eq!(&[0x01, 0x02], &unescape_c_escape_string(r#"\x01\x02"#)[..]);

        assert_eq!(
            &b"\0\x01\x07\x08\x0C\n\r\t\x0B\\\'\"\xFE"[..],
            &unescape_c_escape_string(r#"\0\001\a\b\f\n\r\t\v\\\'\"\xfe"#)[..]
        );
    }

    #[test]
    fn find_lifetime() {
        fn test_lifetime(typestr: &str, expect_some: bool) {
            assert_eq!(find_lifetime_from_str(typestr).is_some(), expect_some);
            let ty: syn::Type = syn::parse_str(typestr).unwrap();
            assert_eq!(find_lifetime_from_type(&ty).is_some(), expect_some);
        }

        test_lifetime("Vec", false);
        test_lifetime("Vec<u8>", false);

        test_lifetime("std::Vec<'a>", true);
        test_lifetime("&'a [u8]", true);
        test_lifetime("[&'a u8; 10]", true);
        test_lifetime("([&'a u8; 10])", true);
        test_lifetime("std::Option<std::Vec<'a>>", true);
        test_lifetime("([&'a u8])", true);
        test_lifetime("(u32, u8, &'a bool)", true);

        test_lifetime("std::Vec<'static>", false);
        test_lifetime("&'static [u8]", false);
        test_lifetime("[&'static u8; 10]", false);
        test_lifetime("([&'static u8; 10])", false);
        test_lifetime("std::Option<std::Vec<'static>>", false);
        test_lifetime("([&'static u8])", false);
        test_lifetime("(u32, u8, &'static bool)", false);

        test_lifetime("&'static std::Option<std::Vec<'a>>", true);
        test_lifetime("&'static std::Option<std::Vec<'static, Ref<'a>>>", true);
        test_lifetime("&'static [&'a u8]", true);
        test_lifetime("(&'static u32, &'a u64)", true);
    }
}
