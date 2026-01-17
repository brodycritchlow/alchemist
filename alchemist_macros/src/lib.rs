use proc_macro::TokenStream;
use quote::quote;
use syn::{
    FnArg, Ident, ItemFn, LitInt, Pat, Token, parenthesized, parse::Parse, parse::ParseStream,
    parse_macro_input,
};

struct TypeExpr {
    name: Ident,
    inner: Option<Vec<TypeExpr>>,
}

impl Parse for TypeExpr {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name: Ident = input.parse()?;

        let inner = if input.peek(syn::token::Paren) {
            let content;
            parenthesized!(content in input);

            let mut types = vec![content.parse::<TypeExpr>()?];
            while content.peek(Token![|]) {
                content.parse::<Token![|]>()?;
                types.push(content.parse()?);
            }
            Some(types)
        } else {
            None
        };

        Ok(TypeExpr { name, inner })
    }
}

struct AlchemistArgs {
    types: Vec<TypeExpr>,
    iterations: usize,
}

impl Parse for AlchemistArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut types = Vec::new();
        let mut iterations = 100;

        while !input.is_empty() {
            if input.peek(Ident) {
                let ident: Ident = input.fork().parse()?;
                if ident == "iterations" {
                    input.parse::<Ident>()?;
                    input.parse::<Token![=]>()?;
                    let lit: LitInt = input.parse()?;
                    iterations = lit.base10_parse()?;

                    if input.peek(Token![,]) {
                        input.parse::<Token![,]>()?;
                    }
                    continue;
                }
            }

            types.push(input.parse()?);

            if input.peek(Token![,]) {
                input.parse::<Token![,]>()?;
            }
        }

        Ok(AlchemistArgs { types, iterations })
    }
}

fn generate_type_expr(expr: &TypeExpr) -> proc_macro2::TokenStream {
    let name = expr.name.to_string();

    match name.as_str() {
        "int" => quote! { <i32 as alchemist::Value>::generate() },
        "uint" => quote! { <u32 as alchemist::Value>::generate() },
        "float" => quote! { <f64 as alchemist::Value>::generate() },
        "bool" => quote! { <bool as alchemist::Value>::generate() },
        "char" => quote! { <char as alchemist::Value>::generate() },
        "str" => quote! { <String as alchemist::Value>::generate() },

        "list" => {
            let inner = expr.inner.as_ref().expect("list requires inner type");
            let inner_gen = generate_union(inner);
            quote! {
                {
                    let len = (rand::random::<u8>() % 10) as usize;
                    (0..len).map(|_| #inner_gen).collect::<Vec<_>>()
                }
            }
        }

        "option" => {
            let inner = expr.inner.as_ref().expect("option requires inner type");
            let inner_gen = generate_union(inner);
            quote! {
                if rand::random::<bool>() { Some(#inner_gen) } else { None }
            }
        }

        "tuple" => {
            let inner = expr.inner.as_ref().expect("tuple requires inner types");
            let gens: Vec<_> = inner.iter().map(generate_type_expr).collect();
            quote! { (#(#gens),*) }
        }

        _ => panic!("Unknown type: {}", name),
    }
}

fn generate_union(types: &[TypeExpr]) -> proc_macro2::TokenStream {
    if types.len() == 1 {
        return generate_type_expr(&types[0]);
    }

    let len = types.len();
    let branches = types.iter().enumerate().map(|(i, ty)| {
        let value_gen = generate_type_expr(ty);
        quote! { #i => Box::new(#value_gen) as Box<dyn std::any::Any> }
    });

    quote! {
        match (rand::random::<u8>() as usize) % #len {
            #(#branches,)*
            _ => unreachable!()
        }
    }
}

#[proc_macro_attribute]
pub fn alchemist(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(attr as AlchemistArgs);
    let input = parse_macro_input!(item as ItemFn);

    let fn_name = &input.sig.ident;
    let fn_block = &input.block;

    let param_names: Vec<_> = input
        .sig
        .inputs
        .iter()
        .filter_map(|arg| {
            if let FnArg::Typed(pat_type) = arg {
                if let Pat::Ident(pat_ident) = &*pat_type.pat {
                    return Some(&pat_ident.ident);
                }
            }
            None
        })
        .collect();

    let generators: Vec<_> = args
        .types
        .iter()
        .zip(param_names.iter())
        .map(|(ty, name)| {
            let value_gen = generate_type_expr(ty);
            quote! { let #name = #value_gen; }
        })
        .collect();

    let format_parts: Vec<_> = param_names
        .iter()
        .map(|name| format!("  {} = {{:?}}", name))
        .collect();
    let format_str = format_parts.join("\n");

    let iterations = args.iterations;

    quote! {
        #[test]
        fn #fn_name() {
            for _iteration in 0..#iterations {
                #(#generators)*

                let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    #fn_block
                }));

                if let Err(err) = result {
                    eprintln!("\n\x1b[31mTest failed on iteration {} of {}\x1b[0m", _iteration + 1, #iterations);
                    eprintln!("\x1b[33mGenerated values:\x1b[0m");
                    eprintln!(#format_str, #(&#param_names),*);
                    eprintln!();
                    std::panic::resume_unwind(err);
                }
            }
        }
    }.into()
}
