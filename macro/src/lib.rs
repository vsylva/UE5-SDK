use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{
    Attribute, Data, DeriveInput, Field, Fields, Lit, Meta, Token,
    parse::{Parse, ParseStream},
    parse_macro_input,
};

#[derive(Clone,)]
enum FieldOffset {
    Explicit(usize,),
    Implicit,
}

#[derive(Clone,)]
struct FieldInfo {
    name:   syn::Ident,
    ty:     syn::Type,
    vis:    syn::Visibility,
    offset: FieldOffset,
}

fn collect_fields(fields: &syn::punctuated::Punctuated<syn::Field, syn::token::Comma,>,) -> Vec<FieldInfo,> {
    let mut result = Vec::new();

    for field in fields.iter()
    {
        let name = field.ident.as_ref().unwrap().clone();
        let ty = field.ty.clone();
        let vis = field.vis.clone();

        let mut explicit_offset: Option<usize,> = None;
        let mut offset_count = 0usize;

        for attr in &field.attrs
        {
            if attr.path().is_ident("offset",)
            {
                offset_count += 1;
                if offset_count > 1
                {
                    panic!("Field '{}' has multiple #[offset(...)] attributes.", name);
                }
                if let Meta::List(meta_list,) = &attr.meta
                {
                    let s = meta_list.tokens.to_string();
                    explicit_offset = if s.starts_with("0x",)
                    {
                        usize::from_str_radix(&s[2..], 16,).ok()
                    }
                    else
                    {
                        s.parse::<usize>().ok()
                    };
                }
            }
        }

        let offset = match explicit_offset
        {
            Some(off,) => FieldOffset::Explicit(off,),
            None => FieldOffset::Implicit,
        };

        if let FieldOffset::Explicit(off,) = offset
        {
            if let Some(prev,) = result
                .iter()
                .rev()
                .find_map(|f: &FieldInfo| if let FieldOffset::Explicit(p,) = f.offset { Some(p,) } else { None },)
            {
                if off < prev
                {
                    panic!(
                        "Field '{}' has offset 0x{:X} which is less than previous explicit offset 0x{:X}. Offsets must be in order.",
                        name, off, prev
                    );
                }
            }
        }

        result.push(FieldInfo { name, ty, vis, offset, },);
    }

    result
}

fn build_struct_body(
    struct_name: &syn::Ident,
    field_infos: &[FieldInfo],
    base_field_tokens: Option<TokenStream2,>,
    base_size_expr: TokenStream2,
) -> (Vec<TokenStream2,>, Vec<TokenStream2,>, TokenStream2,) {
    let mut generated_fields: Vec<TokenStream2,> = Vec::new();
    let mut offset_checks: Vec<TokenStream2,> = Vec::new();

    if let Some(base,) = base_field_tokens
    {
        generated_fields.push(base,);
    }

    let mut prev_end_expr: TokenStream2 = base_size_expr;
    let mut pad_index: usize = 0;

    for (i, field,) in field_infos.iter().enumerate()
    {
        let fname = &field.name;
        let fty = &field.ty;
        let fvis = &field.vis;
        let fname_str = fname.to_string();

        match &field.offset
        {
            FieldOffset::Explicit(target,) =>
            {
                let pad_name = syn::Ident::new(&format!("__pad__{}", pad_index), fname.span(),);
                pad_index += 1;

                generated_fields.push(quote! {
                    #pad_name: [u8; #target - (#prev_end_expr)]
                },);

                offset_checks.push(quote! {
                    const _: () = {
                        const ACTUAL: usize = ::core::mem::offset_of!(#struct_name, #fname);
                        const _: () = assert!(
                            ACTUAL == #target,
                            concat!("Offset mismatch for '", #fname_str, "'")
                        );
                    };
                },);

                prev_end_expr = quote! {
                    #target + ::core::mem::size_of::<#fty>()
                };
            }
            FieldOffset::Implicit =>
            {
                if i > 0
                {
                    let prev = &field_infos[i - 1];
                    let prev_name = &prev.name;
                    let prev_ty = &prev.ty;
                    offset_checks.push(quote! {
                        const _: () = {
                            const ACTUAL: usize = ::core::mem::offset_of!(#struct_name, #fname);
                            const PREV: usize = ::core::mem::offset_of!(#struct_name, #prev_name);
                            const _: () = assert!(
                                ACTUAL == PREV + ::core::mem::size_of::<#prev_ty>(),
                                concat!("Implicit field '", #fname_str, "' is not immediately after previous field")
                            );
                        };
                    },);
                }

                prev_end_expr = quote! {
                    (#prev_end_expr) + ::core::mem::size_of::<#fty>()
                };
            }
        }

        generated_fields.push(quote! {
            #fvis #fname: #fty
        },);
    }

    let field_names: Vec<_,> = field_infos.iter().map(|f| &f.name,).collect();
    let test_body = quote! {
        print!("\r");
        #({
            let offset = ::core::mem::offset_of!(super::#struct_name, #field_names);
            println!(
                "\x1b[90m{}:\x1b[0m {}: \x1b[1;32m0x{:04X}\x1b[0m",
                stringify!(#struct_name),
                stringify!(#field_names),
                offset
            );
        })*
        println!();
    };

    (generated_fields, offset_checks, test_body,)
}

struct InheritArgs {
    base_type:      syn::Path,
    is_valid_field: Option<syn::Ident,>,
}

impl Parse for InheritArgs {
    fn parse(input: ParseStream,) -> syn::Result<Self,> {
        let base_type: syn::Path = input.parse()?;

        let is_valid_field = if input.peek(Token![,],)
        {
            let _: Token![,] = input.parse()?;
            let ident: syn::Ident = input.parse()?;
            Some(ident,)
        }
        else
        {
            None
        };

        Ok(InheritArgs { base_type, is_valid_field, },)
    }
}

#[proc_macro_attribute]
pub fn inherit(attr: TokenStream, item: TokenStream,) -> TokenStream {
    let args = parse_macro_input!(attr as InheritArgs);
    let input = parse_macro_input!(item as DeriveInput);

    let name = &input.ident;
    let vis = &input.vis;
    let attrs = &input.attrs;
    let base_type = &args.base_type;

    let fields = match &input.data
    {
        Data::Struct(data,) => match &data.fields
        {
            Fields::Named(f,) => &f.named,
            _ => panic!("inherit only supports structs with named fields"),
        },
        _ => panic!("inherit only supports structs"),
    };

    let field_infos = collect_fields(fields,);

    let base_field_tokens = quote! { __inherit__: #base_type };
    let base_size_expr = quote! { ::core::mem::size_of::<#base_type>() };

    let (generated_fields, offset_checks, test_body,) =
        build_struct_body(name, &field_infos, Some(base_field_tokens,), base_size_expr,);

    let test_mod =
        syn::Ident::new(&format!("__test_output_offsets_{}__", name.to_string().to_lowercase()), name.span(),);

    let is_valid_impl = match &args.is_valid_field
    {
        Some(flag_field,) => quote! {
            impl #name {
                #[inline]
                pub unsafe fn is_valid(self: *const Self) -> bool {
                    if self.is_null() {
                        return false;
                    }

                    (*self).__inherit__.#flag_field
                        & (0x0000_8000 | 0x0001_0000 | 0x4000_0000)
                        == 0

                }
            }
        },
        None => quote! {},
    };

    let expanded = quote! {
        #[repr(C)]
        #(#attrs)*
        #vis struct #name {
            #(#generated_fields),*
        }

        #(#offset_checks)*

        #[cfg(test)]
        mod #test_mod {
            #[test]
            fn offsets() {
                #test_body
            }
        }

        impl ::core::ops::Deref for #name {
            type Target = #base_type;
            fn deref(&self) -> &Self::Target {
                &self.__inherit__
            }
        }

        impl ::core::ops::DerefMut for #name {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.__inherit__
            }
        }

        impl #name {
            #[inline]
            pub unsafe fn vtable(self: *mut #name) -> *mut usize {
                *(self as *mut *mut usize)
            }

            #[inline]
            pub unsafe fn vtable_entry(self: *mut #name, index: usize) -> *mut usize {
                self.vtable().add(index,)
            }
        }

        #is_valid_impl
    };

    TokenStream::from(expanded,)
}

#[proc_macro_attribute]
pub fn disinherit(_attr: TokenStream, item: TokenStream,) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);

    let name = &input.ident;
    let vis = &input.vis;
    let attrs = &input.attrs;

    let fields = match &input.data
    {
        Data::Struct(data,) => match &data.fields
        {
            Fields::Named(f,) => &f.named,
            _ => panic!("disinherit only supports structs with named fields"),
        },
        _ => panic!("disinherit only supports structs"),
    };

    let field_infos = collect_fields(fields,);

    let (generated_fields, offset_checks, test_body,) = build_struct_body(name, &field_infos, None, quote! { 0usize },);

    let test_mod =
        syn::Ident::new(&format!("__test_output_offsets_{}__", name.to_string().to_lowercase()), name.span(),);

    let expanded = quote! {
        #[repr(C)]
        #(#attrs)*
        #vis struct #name {
            #(#generated_fields),*
        }

        #(#offset_checks)*

        #[cfg(test)]
        mod #test_mod {
            #[test]
            fn offsets() {
                #test_body
            }
        }
    };

    TokenStream::from(expanded,)
}

fn get_bits_attr(attrs: &[Attribute],) -> Option<(u8, Option<syn::Ident,>,),> {
    for attr in attrs
    {
        if attr.path().is_ident("bits",)
        {
            let result = attr.parse_args_with(|input: syn::parse::ParseStream| {
                let lit: Lit = input.parse()?;
                let n = if let Lit::Int(li,) = lit
                {
                    li.base10_parse::<u8>().map_err(|e| input.error(e.to_string(),),)?
                }
                else
                {
                    return Err(input.error("expected integer literal",),);
                };
                let name: Option<syn::Ident,> = if input.peek(Token![,],)
                {
                    let _: Token![,] = input.parse()?;
                    Some(input.parse()?,)
                }
                else
                {
                    None
                };
                Ok((n, name,),)
            },);
            return result.ok();
        }
    }
    None
}

fn is_bool_type(ty: &syn::Type,) -> bool {
    if let syn::Type::Path(tp,) = ty
    {
        if let Some(seg,) = tp.path.segments.last()
        {
            return seg.ident == "bool";
        }
    }
    false
}

fn strip_bits_attr(attrs: &[Attribute],) -> Vec<Attribute,> {
    attrs.iter().filter(|a| !a.path().is_ident("bits",),).cloned().collect()
}

struct BitField {
    name:       syn::Ident,
    vis:        syn::Visibility,
    bit_offset: u8,
}

struct BitGroup {
    storage_name: syn::Ident,
    leader_attrs: Vec<Attribute,>,
    fields:       Vec<BitField,>,
}

enum Slot {
    Normal(Field,),
    Group(BitGroup,),
}

#[proc_macro_attribute]
pub fn bitfields(_attr: TokenStream, item: TokenStream,) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);

    let struct_name = &input.ident;
    let vis = &input.vis;
    let generics = &input.generics;

    let named_fields = match &input.data
    {
        Data::Struct(ds,) => match &ds.fields
        {
            Fields::Named(nf,) => &nf.named,
            _ =>
            {
                return syn::Error::new_spanned(&input.ident, "bitfields: only named-field structs",)
                    .to_compile_error()
                    .into();
            }
        },
        _ =>
        {
            return syn::Error::new_spanned(&input.ident, "bitfields: only structs",).to_compile_error().into();
        }
    };

    let mut slots: Vec<Slot,> = Vec::new();
    let mut iter = named_fields.iter().peekable();

    while let Some(field,) = iter.next()
    {
        let fname = field.ident.as_ref().unwrap();

        if let Some((total_bits, Some(storage_name,),),) = get_bits_attr(&field.attrs,)
        {
            if !is_bool_type(&field.ty,)
            {
                return syn::Error::new_spanned(fname, "bitfields: #[bits(N,NAME)] must be on a bool field",)
                    .to_compile_error()
                    .into();
            }

            let leader_attrs = strip_bits_attr(&field.attrs,);

            let mut bits = vec![BitField { name: fname.clone(), vis: field.vis.clone(), bit_offset: 0, }];

            for i in 1..total_bits
            {
                match iter.peek()
                {
                    Some(next,) if is_bool_type(&next.ty,) && get_bits_attr(&next.attrs,).is_none() =>
                    {
                        let nf = iter.next().unwrap();
                        bits.push(BitField {
                            name:       nf.ident.as_ref().unwrap().clone(),
                            vis:        nf.vis.clone(),
                            bit_offset: i,
                        },);
                    }
                    _ =>
                    {
                        return syn::Error::new_spanned(
                            fname,
                            format!(
                                "bitfields: #[bits({},{})] expects {} more bool field(s)",
                                total_bits,
                                storage_name,
                                total_bits - 1
                            ),
                        )
                        .to_compile_error()
                        .into();
                    }
                }
            }

            slots.push(Slot::Group(BitGroup { storage_name, leader_attrs, fields: bits, },),);
        }
        else
        {
            let mut f = field.clone();
            f.attrs = strip_bits_attr(&f.attrs,);
            slots.push(Slot::Normal(f,),);
        }
    }

    let new_fields: Vec<_,> = slots
        .iter()
        .map(|slot| match slot
        {
            Slot::Normal(f,) => quote! { #f },
            Slot::Group(g,) =>
            {
                let sname = &g.storage_name;
                let attrs = &g.leader_attrs;
                quote! { #(#attrs)* #sname: u8 }
            }
        },)
        .collect();

    let methods: Vec<_,> = slots
        .iter()
        .flat_map(|slot| {
            let mut out = Vec::new();
            let Slot::Group(g,) = slot
            else
            {
                return out;
            };

            let sname = &g.storage_name;

            for bf in &g.fields
            {
                let fname = &bf.name;
                if fname.to_string().starts_with('_',)
                {
                    continue;
                }
                let bvis = &bf.vis;
                let offset = bf.bit_offset;
                let getter = syn::Ident::new(&format!("bit_get_{}", fname), fname.span(),);
                let setter = syn::Ident::new(&format!("bit_set_{}", fname), fname.span(),);

                out.push(quote! {
                    #[inline]
                    #bvis const fn #getter(&self) -> bool {
                        (self.#sname >> #offset) & 1 != 0
                    }
                    #[inline]
                    #bvis fn #setter(&mut self, val: bool) {
                        if val {
                            self.#sname |= 1 << #offset;
                        } else {
                            self.#sname &= !(1u8 << #offset);
                        }
                    }
                },);
            }
            out
        },)
        .collect();

    let struct_attrs: Vec<_,> = input.attrs.iter().filter(|a| !a.path().is_ident("bitfields",),).collect();
    let (impl_generics, ty_generics, where_clause,) = generics.split_for_impl();

    quote! {
        #(#struct_attrs)*
        #vis struct #struct_name #generics {
            #(#new_fields),*
        }

        impl #impl_generics #struct_name #ty_generics #where_clause {
            #(#methods)*
        }
    }
    .into()
}
